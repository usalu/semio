use super::*;
use crate::editor::shooting::unit_tests::context::{dispatch, shooting_app};
use crate::editor::shooting::ShootingCommand;

/// 💾️ LAW: the submitted label names the saved camera in ONE document edit; a blank submit falls back to `Camera N`.
#[semio_framework_async_macros::async_test]
async fn save_camera_stores_the_submitted_label() {
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::SaveCamera(save_camera::SaveCamera { label: "  Hero ".into() })).await;
    assert!(result.edited_document(), "saving a camera is a real document edit");
    let saved = app.snapshot().expect("snapshot").saved_cameras;
    assert_eq!(saved.last().map(|camera| camera.label.as_str()), Some("Hero"));
    dispatch(&mut app, ShootingCommand::SaveCamera(save_camera::SaveCamera { label: String::new() })).await;
    let saved = app.snapshot().expect("snapshot").saved_cameras;
    assert_eq!(saved.last().map(|camera| camera.label.clone()), Some(format!("Camera {}", saved.len())));
}

#[semio_framework_async_macros::async_test]
async fn set_camera_never_touches_the_document() {
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::SetCamera(set_camera::SetCamera { camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() } })).await;
    assert!(!result.edited_document(), "the free/live camera is config-only");
}
