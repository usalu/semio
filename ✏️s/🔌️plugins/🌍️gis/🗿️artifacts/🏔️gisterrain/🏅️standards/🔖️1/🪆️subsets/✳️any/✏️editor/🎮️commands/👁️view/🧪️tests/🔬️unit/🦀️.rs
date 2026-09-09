use super::*;
use crate::editor::gis3d::testkit::{app, close, dispatch};
use crate::editor::gis3d::Gis3dCommand;
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn camera_is_config_state_and_emits_no_operations() {
    let mut app = app().await;
    let camera = dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: json!({ "position": [1.0, 1.0, 1.0] }).to_string() })).await;
    assert_eq!(camera.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::Config).count(), 1, "camera publishes exactly one config operation");
    assert!(!camera.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "camera never publishes document state");
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_camera_reaches_the_rendered_scene() {
    let mut app = app().await;
    dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: json!({ "position": [123.0, 1.0, 1.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string() })).await;
    assert!(crate::editor::gis3d::testkit::render(&mut app, crate::editor::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE).await.contains("123"));
    close(&mut app);
}
