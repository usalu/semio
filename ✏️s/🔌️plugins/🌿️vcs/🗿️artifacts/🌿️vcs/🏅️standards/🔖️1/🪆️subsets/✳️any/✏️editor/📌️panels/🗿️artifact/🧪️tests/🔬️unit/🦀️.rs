use super::*;
use crate::editor::vcs::unit_tests::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_checkpoints() {
    let mut instance = app().await;
    let json = render_body(&mut instance, VCS_PLAY_BODY_DOCUMENT).await;
    assert!(json.contains("vcs-play-document.checkpoint"));
}

#[semio_framework_async_macros::async_test]
async fn vcs_labels_resolve_native_english_by_default() {
    let mut instance = app().await;
    let json = render_body(&mut instance, VCS_PLAY_BODY_DOCUMENT).await;
    assert!(json.contains("Alternatives"));
    assert!(json.contains("checkpoints"));
}
