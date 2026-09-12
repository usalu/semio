use super::*;
use crate::editor::wires::unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_canvas_scene() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, WIRES_PLAY_BODY_COMPOSITE).await.contains("canvas-2d"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_canvas_2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, WIRES_PLAY_BODY_COMPOSITE);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
}
