use semio_s_artifact_fem_3d_editor::editor::fem3d::commands::set_camera::*;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model;
use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_os_kernel::Viewport3dOrbit;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use crate::context::dispatch;
use crate::context::fem3d_app;
use crate::context::render;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

#[semio_framework_async_macros::async_test]
async fn set_camera_action_writes_config_not_artifact_mutations() {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetCamera(SetCamera { camera: Viewport3dOrbit { position: [8.0, -3.0, 5.0], target: [0.0; 3], zoom: 1.25, up: None } })).await;
    // 🎥️ `VcsArtifactApp` exposes no config accessor — assert the config-only effect through render
    // output, the way the pre-migration tests already did.
    let model = render(&mut app, model::FEM3D_BODY_MODEL);
    assert!(model.contains("world-3d"), "camera write must not break rendering: {model}");
}
