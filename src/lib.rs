use agent_stream_kit::ASKit;
use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime, RunEvent,
};

mod commands;
mod error;
mod models;
mod observer;

pub use error::{Error, Result};
pub use models::*;

use observer::BoardObserver;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the askit APIs.
pub trait ASKitExt<R: Runtime> {
  fn askit(&self) -> &ASKit;
}

impl<R: Runtime, T: Manager<R>> crate::ASKitExt<R> for T {
  fn askit(&self) -> &ASKit {
    self.state::<ASKit>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("askit")
    .invoke_handler(tauri::generate_handler![
      commands::get_agent_definition,
      commands::get_agent_definitions,
      commands::get_agent_flows,
      commands::new_agent_flow,
      commands::rename_agent_flow,
      commands::unique_flow_name,
      commands::add_agent_flow,
      commands::remove_agent_flow,
      commands::insert_agent_flow,
      commands::copy_sub_flow,
      commands::start_agent_flow,
      commands::stop_agent_flow,
      commands::new_agent_flow_node,
      commands::add_agent_flow_node,
      commands::remove_agent_flow_node,
      commands::add_agent_flow_edge,
      commands::remove_agent_flow_edge,
      commands::start_agent,
      commands::stop_agent,
      commands::write_board,
      commands::set_agent_config,
      commands::get_global_config,
      commands::get_global_configs,
      commands::set_global_config,
      commands::set_global_configs,
      commands::get_agent_default_config,
    ])
    .setup(|app, _api| {
      let askit = ASKit::init()?;
      askit.subscribe(Box::new(BoardObserver {
          app: app.clone(),
      }));
      app.manage(askit);
      Ok(())
    })
    .on_event(|app, event| {
      match event {
        RunEvent::Ready => {
          tauri::async_runtime::block_on(async move {
            let askit = app.state::<ASKit>();
            askit.ready().await.unwrap();
          });
        }
        RunEvent::Exit => {
          let askit = app.state::<ASKit>();
          askit.quit();
        }
        _ => {}
      }
    })
    .build()
}
