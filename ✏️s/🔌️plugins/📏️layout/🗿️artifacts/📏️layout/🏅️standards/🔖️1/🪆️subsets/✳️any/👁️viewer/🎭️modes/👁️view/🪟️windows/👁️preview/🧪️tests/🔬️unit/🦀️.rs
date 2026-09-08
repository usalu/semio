
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_canvas_2d_preview_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert!(matches!(def.surface_kind, SurfaceKind::Canvas2d));
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::standards::v1::subsets::any::schema::default_document();
    let node = render(&document).expect("layout preview surface");
    assert!(matches!(node.component, semio_framework_plugin::plugin_app_close_prelude::Component::Surface(_)));
}

#[semio_framework_async_macros::async_test]
async fn viewer_canvas_layers_renders_the_page_background() {
    let document = crate::standards::v1::subsets::any::schema::default_document();
    let json = viewer_canvas_layers(&document);
    assert!(json.contains("layout.page-bg"));
}
