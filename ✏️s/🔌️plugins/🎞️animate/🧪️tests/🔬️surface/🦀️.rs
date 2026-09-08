
//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — SDK gap now closed
//! (`📓️w0-f-report.md`): `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
//! assert_editor_and_viewer_share_dialect}` are real, exercised directly here.
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn animate_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::animate::AnimatePresentationViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn animate_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::animate::AnimatePresentationPlayApp, crate::viewer::animate::AnimatePresentationViewer>().await;
}
