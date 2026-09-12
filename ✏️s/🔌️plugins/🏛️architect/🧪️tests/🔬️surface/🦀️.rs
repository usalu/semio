//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 canonical helpers
//! (`semio_framework_plugin::artifact_app_laws::{assert_viewer_never_mutates,
//! assert_editor_and_viewer_share_dialect, new_viewer}`) — closed by lane 0-F (`📓️w0-f-report.md`
//! Gap 2), used directly here rather than local stand-ins.
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn architect_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::architect::ArchitectViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn architect_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::architect::ArchitectPlayApp, crate::viewer::architect::ArchitectViewer>().await;
}
