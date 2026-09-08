
use super::*;
use crate::editor::generation3d::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn inspector_shows_no_selection_by_default() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_INSPECTION).await.contains("Schema:"));
}
