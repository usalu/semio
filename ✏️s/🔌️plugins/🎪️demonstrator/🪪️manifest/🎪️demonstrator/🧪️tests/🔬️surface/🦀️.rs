//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5's
//! `semio_framework_plugin::artifact_app_laws::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
//! (W0-F gap 2) are used directly here — no local stand-ins, exercised against demonstrator's own
//! `🎪️playground` editor/viewer pair (the six foreign plugins' own surfaces registered above
//! belong to their own owning plugins' surface tests, not this one).
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn playground_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::playground::PlaygroundViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn playground_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::playground::PlaygroundEditor, crate::viewer::playground::PlaygroundViewer>().await;
}
