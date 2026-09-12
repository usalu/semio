use super::*;
use crate::editor::generation3d::unit_tests::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn generate_mode_renders_surfaces() {
    let mut app = app().await;
    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATIONS).await;
    assert!(body.contains("addGeneration"), "the generate-mode generations window must offer the add action: {body}");
}
