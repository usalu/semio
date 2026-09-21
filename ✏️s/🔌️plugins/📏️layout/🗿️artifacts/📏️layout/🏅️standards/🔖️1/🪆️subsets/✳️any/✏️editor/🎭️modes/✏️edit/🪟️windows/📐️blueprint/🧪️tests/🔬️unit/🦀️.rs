use super::*;
use crate::editor::layout::unit_tests::context::{layout_app, render as render_body, scene as body_scene};

#[semio_framework_async_macros::async_test]
async fn renders_blueprint_canvas_scene() {
    let mut app = layout_app().await;
    assert!(render_body(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).await.contains("canvas-2d"));
}

#[semio_framework_async_macros::async_test]
async fn blueprint_scene_has_page_background_and_guides() {
    // 🧷️ `layers_json` is read off the DECODED scene: the built surface carries its scene as a binary
    // pack, so the projected tree's text holds none of these ids. Inside that field the layer list is
    // still ordinary JSON, asserted on unquoted substrings rather than an exact `"key":"value"` shape.
    let mut app = layout_app().await;
    let json = body_scene(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).await.layers_json;
    assert!(json.contains("layout.page-bg"));
    assert!(json.contains("0.97"));
    assert!(json.contains("layout.guide.margin"));
    assert!(json.contains("layout.guide.column"));
    assert!(json.contains("segments"));
    assert!(json.contains("fill") && json.contains("color"));
    assert!(!json.contains("linkId"));
}

#[semio_framework_async_macros::async_test]
async fn inherited_frame_gets_dashed_stroke_in_blueprint() {
    let mut app = layout_app().await;
    let json = body_scene(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).await.layers_json;
    assert!(json.contains("dash") && json.contains("4.0") && json.contains("3.0"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_canvas_2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, LAYOUT_PLAY_BODY_BLUEPRINT);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    assert!(definition.options.measures.is_empty());
}
