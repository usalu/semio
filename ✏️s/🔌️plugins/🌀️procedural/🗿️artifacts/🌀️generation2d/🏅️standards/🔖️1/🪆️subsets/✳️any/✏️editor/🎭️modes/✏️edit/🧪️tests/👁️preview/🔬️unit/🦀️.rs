use super::*;
use crate::editor::generation2d::testkit::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_preview_canvas_scene() {
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION2D_PLAY_BODY_PREVIEW).await;
    close(app);
    assert!(rendered.contains("canvas-2d"), "{rendered}");
}
