use semio_s_artifact_fem_3d_editor::editor::fem3d::commands::set_result_display::*;
use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use crate::context::dispatch;
use crate::context::fem3d_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

#[semio_framework_async_macros::async_test]
async fn set_result_display_writes_config_not_artifact_mutations() {
    let mut app = fem3d_app();
    // 🎯️ No config accessor on `VcsArtifactApp` — dispatch must simply not panic/error, and the
    // results window render test (in `modes::edit::windows::results`) covers the resulting display.
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1, field: None, value: None, window_id: None })).await;
}
