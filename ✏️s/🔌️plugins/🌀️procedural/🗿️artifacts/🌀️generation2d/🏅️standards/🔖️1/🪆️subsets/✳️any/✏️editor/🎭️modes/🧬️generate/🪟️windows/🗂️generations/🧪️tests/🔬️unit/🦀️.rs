
use super::*;
use crate::editor::generation2d::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn generate_mode_renders_surfaces() {
    let mut app = app().await;
    assert!(render_body(&mut app, GENERATION2D_PLAY_BODY_GENERATIONS).await.contains("addGeneration"));
}
