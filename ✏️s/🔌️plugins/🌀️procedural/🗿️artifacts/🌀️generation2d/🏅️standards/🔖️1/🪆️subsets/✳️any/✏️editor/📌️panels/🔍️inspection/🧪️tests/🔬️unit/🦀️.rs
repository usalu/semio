use super::*;
use crate::editor::generation2d::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn generation2d_labels_translate_catalogue_and_inspector_in_german() {
    let mut app = app().await;
    let inspector_json = render_body(&mut app, GENERATION2D_PLAY_BODY_INSPECTION).await;
    assert!(inspector_json.contains("Elemente:"));
}
