
//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 promises
//! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
//! new_viewer}` — landed for real as of this packet (unlike the cad pilot, which had to write local
//! stand-ins), so these call the canonical framework versions directly.
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn shooting_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::shooting::ShootingViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn shooting_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::shooting::ShootingPlayApp, crate::viewer::shooting::ShootingViewer>().await;
}
