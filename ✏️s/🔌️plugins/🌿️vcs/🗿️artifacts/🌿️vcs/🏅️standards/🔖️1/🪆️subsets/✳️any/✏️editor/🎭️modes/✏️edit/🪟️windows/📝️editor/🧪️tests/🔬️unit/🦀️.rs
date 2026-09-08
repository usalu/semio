
use super::*;
use crate::editor::vcs::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_editor_scene() {
    let mut instance = app().await;
    let json = render_body(&mut instance, VCS_PLAY_BODY_EDITOR).await;
    assert!(!json.contains("text-editor"), "editor must no longer be a raw JSON editor: {json}");
    for action in ["incrementCounter", "commitCheckpoint", "undo", "redo", "createAlternative"] {
        assert!(json.contains(action), "missing editor button for {action}: {json}");
    }
    assert!(json.contains(" · Counter "), "missing title/counter summary: {json}");
}

#[semio_framework_async_macros::async_test]
async fn vcs_labels_resolve_native_english_by_default() {
    let mut instance = app().await;
    let json = render_body(&mut instance, VCS_PLAY_BODY_EDITOR).await;
    assert!(json.contains("Actions"));
    assert!(json.contains("Commit"));
    assert!(json.contains("Branch"));
    assert!(json.contains("Undo"));
    assert!(json.contains("Redo"));
    assert!(json.contains("Counter"));
}
