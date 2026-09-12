use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn generate_preview_hints_without_evaluated_output() {
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION2D_PLAY_BODY_GENERATE_PREVIEW).await;
    close(app);
    assert!(rendered.contains("evaluate a generation"), "{rendered}");
}
