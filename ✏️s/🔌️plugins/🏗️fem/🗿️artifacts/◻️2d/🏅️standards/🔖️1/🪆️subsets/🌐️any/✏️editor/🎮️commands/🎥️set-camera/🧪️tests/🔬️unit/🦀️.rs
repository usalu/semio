
use super::*;
use crate::editor::fem2d::Fem2dCommand;
use crate::editor::fem2d::testkit::{dispatch, fem2d_app};

#[semio_framework_async_macros::async_test]
async fn set_camera_action_writes_config_not_artifact_mutations() {
    let mut app = fem2d_app();
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, Fem2dCommand::SetCamera(SetCamera { x: 1.0, y: 2.0, zoom: 1.5 })).await;
    assert!(result.mutations.is_empty(), "setCamera must not emit a document VCS operation");
    assert_eq!(app.snapshot().expect("snapshot"), before, "the document must be unchanged by a config-only command");
}
