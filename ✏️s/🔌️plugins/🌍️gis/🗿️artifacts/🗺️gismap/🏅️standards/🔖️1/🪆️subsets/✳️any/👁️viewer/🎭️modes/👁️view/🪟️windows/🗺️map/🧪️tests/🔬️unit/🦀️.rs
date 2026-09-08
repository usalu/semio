
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_tiled_map_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.surface_kind, SurfaceKind::TiledMap);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::schema::default_document();
    let _node = render(&document);
}
