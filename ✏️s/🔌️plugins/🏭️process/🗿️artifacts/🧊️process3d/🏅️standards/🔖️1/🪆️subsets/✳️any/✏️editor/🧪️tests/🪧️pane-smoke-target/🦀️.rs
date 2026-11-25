//! 🪧️ Bearbeiten pane smoke target: the shared app harness plus the pane-smoke law alone, so the pane is provable without the full unit tree.

#![recursion_limit = "512"]
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
use semio_s_artifact_process_process3d_harness as context;
use semio_s_artifact_process_process3d_commands::step::add_step;
use semio_s_artifact_process_process3d_commands::document::set_active_example;
use semio_s_artifact_process_process3d_kit::PROCESS3D_EXAMPLE_CONCRETE_FOREST;
use semio_s_artifact_process_process3d_editor::*;
use semio_s_artifact_process_process3d_modes::windows::workpiece::{self, PROCESS_3D_PLAY_BODY_MAIN};
use semio_plugin::{artifact_app_laws, EditorApp, PluginApp};
use crate::context::unseeded_app_with_registry as app;

#[path = "../🔬️unit/🪧️pane-smoke/🦀️.rs"]
mod pane_smoke;
#[path = "🪜️composition/🦀️.rs"]
mod composition;

/// 📥️ The host delivers both roster updates through the editor's addressed command registry.
#[semio_framework_async_macros::async_test]
async fn host_delivered_set_contributions_dispatches_through_the_command_registry() {
    use semio_plugin::app::TypedOperationResultLane;

    let law: serde_json::Value = serde_json::from_str(include_str!("config-publication.json")).expect("config publication law");
    let contributions = law["contributions"].to_string();
    let mut app = crate::context::app_with_registry();
    assert_ne!(app.rendered_config().contributions_json, contributions);
    let receipt = crate::context::install_contributions(&mut app, contributions.clone());
    assert_eq!(app.rendered_config().contributions_json, contributions);
    let config_pages = receipt.lanes.iter().filter(|lane| **lane == TypedOperationResultLane::Config).count();
    assert_eq!(law["configPages"].as_u64(), Some(config_pages as u64));
    assert_eq!(receipt.lanes.last(), Some(&TypedOperationResultLane::Terminal));
    assert_eq!(law["terminalCompletions"].as_u64(), Some(receipt.completions as u64));
    assert!(!app.has_pending_typed_operations());
    assert!(app.maintenance_terminal_is_empty());
    app.snapshot().expect("the app keeps a readable snapshot after the delivery");
    app.close_fixture();
    assert!(app.close_terminal_is_empty());
}
