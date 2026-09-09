use super::*;
use crate::editor::vcs::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn vcs_labels_resolve_native_english_by_default() {
    let mut instance = app().await;
    let json = render_body(&mut instance, VCS_PLAY_BODY_INSPECTION).await;
    assert!(json.contains("Title"));
    assert!(json.contains("Status"));
    assert!(json.contains("Notes"));
    assert!(json.contains("Tags"));
}
