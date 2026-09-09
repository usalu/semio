use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_world3d_shape_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.surface_kind, SurfaceKind::World3d);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::standards::v1::subsets::any::schema::inferences::forest_play_scene();
    let _node = render(&document);
}
