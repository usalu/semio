use super::*;
use crate::editor::layout::testkit::{layout_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_preview_canvas_scene() {
    let mut app = layout_app().await;
    assert!(render_body(&mut app, LAYOUT_PLAY_BODY_PREVIEW).await.contains("canvas-2d"));
}

#[semio_framework_async_macros::async_test]
async fn preview_scene_has_white_background_and_no_guides() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_PREVIEW).await;
    assert!(json.contains("layout.page-bg"));
    assert!(!json.contains("layout.guide."));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_canvas_2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, LAYOUT_PLAY_BODY_PREVIEW);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    assert!(definition.options.measures.is_empty());
}
