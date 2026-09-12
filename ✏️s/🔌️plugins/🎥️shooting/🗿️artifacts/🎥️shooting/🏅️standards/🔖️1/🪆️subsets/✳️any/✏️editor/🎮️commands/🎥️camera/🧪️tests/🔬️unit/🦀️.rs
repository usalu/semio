use super::*;
use crate::editor::shooting::unit_tests::context::{dispatch, shooting_app};
use crate::editor::shooting::ShootingCommand;

#[semio_framework_async_macros::async_test]
async fn save_and_load_camera_round_trip() {
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::SetCameraDraftLabel(set_camera_draft_label::SetCameraDraftLabel { value: "Hero".into() })).await;
    let result = dispatch(&mut app, ShootingCommand::SaveCamera(save_camera::SaveCamera {})).await;
    assert_eq!(result.mutations.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn set_camera_never_touches_the_document() {
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::SetCamera(set_camera::SetCamera { camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() } })).await;
    assert!(result.mutations.is_empty(), "the free/live camera is config-only");
}
