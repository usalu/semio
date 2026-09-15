use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world3d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, PROCESS3D_VIEW_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
}

#[semio_framework_async_macros::async_test]
async fn render_world_scene_contains_processed_mesh() {
    let fixture = crate::empty_process3d_snapshot();
    let node = render(&fixture).expect("bounded workpiece");
    let scene: semio_framework_plugin::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("assemble world3d scene");
    assert!(scene.meshes_json.contains("processed") || scene.instances_json.contains("processed"), "expected the processed mesh id in the scene: {}", scene.meshes_json);
}
