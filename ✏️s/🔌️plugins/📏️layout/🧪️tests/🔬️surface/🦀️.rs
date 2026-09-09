//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 canonical helpers
//! (`semio_framework_plugin::testkit::{assert_viewer_never_mutates,
//! assert_editor_and_viewer_share_dialect, new_viewer}`) — closed by lane 0-F (`📓️w0-f-report.md`
//! Gap 2), used directly here rather than local stand-ins.
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn layout_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::layout::LayoutViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn layout_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::layout::LayoutPlayApp, crate::viewer::layout::LayoutViewer>().await;
}
