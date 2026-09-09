use super::*;
use crate::editor::generation2d::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_show_modes() {
    let mut app = app().await;
    assert!(render_body(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE).await.contains("procedural2d-play-catalogue.mode.preview"));
}

#[semio_framework_async_macros::async_test]
async fn generation2d_labels_resolve_native_english_by_default() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("\"Sources\""));
    assert!(json.contains("\"Components\""));
    assert!(json.contains("\"Sinks\""));
    assert!(json.contains("\"Show mode\""));
    assert!(!json.contains("Quellen"));
}
