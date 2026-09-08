
//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 —
//! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
//! now exist for real (landed by lane 0-F, see `📓️w0-f-report.md`), so this uses them directly
//! rather than writing local stand-ins.
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn vcs_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::vcs::VcsViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn vcs_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::vcs::VcsPlayApp, crate::viewer::vcs::VcsViewer>().await;
}
