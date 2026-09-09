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
    let node = serde_json::to_string(&render(&fixture).expect("bounded workpiece")).expect("render json");
    assert!(node.contains("processed"), "expected the processed mesh id in scene json: {node}");
}
