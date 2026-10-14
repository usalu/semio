use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_camera::*;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_os_kernel::Viewport2d;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::context::{dispatch, fem2d_app};
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;

#[semio_framework_async_macros::async_test]
async fn set_camera_action_writes_config_not_artifact_mutations() {
    let mut app = fem2d_app();
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, Fem2dCommand::SetCamera(SetCamera { x: 1.0, y: 2.0, zoom: 1.5 })).await;
    assert!(result.mutations.is_empty(), "setCamera must not emit a document VCS operation");
    assert_eq!(app.snapshot().expect("snapshot"), before, "the document must be unchanged by a config-only command");
}
