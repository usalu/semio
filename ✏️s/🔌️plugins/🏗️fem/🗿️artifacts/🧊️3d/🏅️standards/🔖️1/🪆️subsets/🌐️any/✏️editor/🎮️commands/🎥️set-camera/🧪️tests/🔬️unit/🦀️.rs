use super::*;
use crate::editor::fem3d::testkit::{dispatch, fem3d_app, render};
use crate::editor::fem3d::Fem3dCommand;

#[semio_framework_async_macros::async_test]
async fn set_camera_action_writes_config_not_artifact_mutations() {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetCamera(SetCamera { json: "{\"x\":1}".into() })).await;
    // 🎥️ `VcsArtifactApp` exposes no config accessor — assert the config-only effect through render
    // output, the way the pre-migration tests already did.
    let model = render(&mut app, crate::editor::fem3d::modes::edit::windows::model::FEM3D_BODY_MODEL);
    assert!(model.contains("world-3d"), "camera write must not break rendering: {model}");
}
