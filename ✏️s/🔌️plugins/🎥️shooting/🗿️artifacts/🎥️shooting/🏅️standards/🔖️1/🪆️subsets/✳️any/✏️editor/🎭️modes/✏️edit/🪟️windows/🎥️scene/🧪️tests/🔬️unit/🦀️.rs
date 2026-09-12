use super::*;
use crate::editor::shooting::unit_tests::context::{scene_window_measures, shooting_app};

#[semio_framework_async_macros::async_test]
async fn renders_world_model_scene() {
    let mut app = shooting_app().await;
    let scene = crate::editor::shooting::unit_tests::context::world_scene(&mut app).await;
    let environment: Value = parse(scene.environment_json.as_deref().unwrap()).unwrap();
    assert_eq!(environment["sun"]["azimuth"], json!(45.0));
    assert_eq!(environment["material"]["roughness"], json!(1.0));
    let frame: Value = parse(scene.frame_json.as_deref().unwrap()).unwrap();
    assert_eq!(frame["width"], json!(256));
    assert_eq!(frame["shape"], json!("rectangle"));
    let fit: Value = parse(scene.fit_json.as_deref().unwrap()).unwrap();
    assert_eq!(fit["enabled"], json!(true));
    let camera: Value = parse(&scene.camera_json).unwrap();
    assert_eq!(camera["zoom"], json!(1.0));
    assert_eq!(camera["projection"], json!("perspective"));
}

#[semio_framework_async_macros::async_test]
async fn model_scene_uses_asset_mesh_urls() {
    let mut app = shooting_app().await;
    let scene = crate::editor::shooting::unit_tests::context::world_scene(&mut app).await;
    assert!(scene.meshes_json.contains("mesh:🧊️base"));
    assert!(scene.meshes_json.contains("/mesh/🧊️base.glb"));
}

#[semio_framework_async_macros::async_test]
async fn window_measures_surface_eight_scene_measures() {
    let mut app = shooting_app().await;
    let measures = scene_window_measures(&mut app).await;
    assert_eq!(measures.len(), 8);
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world_3d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, SHOOTING_PLAY_BODY_SCENE);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
