use super::*;
use crate::editor::vcs::unit_tests::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_history_scene() {
    let mut instance = app().await;
    let json = render_body(&mut instance, VCS_PLAY_BODY_HISTORY).await;
    assert!(json.contains("graph-timeline"), "missing graph-timeline surface kind: {json}");
    assert!(json.contains("lane"), "missing lane field in history columns: {json}");
    assert!(!json.contains("\"table\""), "history must not fall back to a generic table: {json}");
}
