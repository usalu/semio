//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5's
//! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
//! new_viewer}` landed (W0-F gap closure) — used directly here, no local stand-ins.

#[semio_framework_async_macros::async_test]
async fn writer_viewer_never_mutates() {
    semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::writer::WriterViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn writer_editor_and_viewer_share_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::writer::WriterPlayApp, crate::viewer::writer::WriterViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn standard_mounts_exactly_one_subset() {
    assert_eq!(crate::artifacts::writer::standards::v1::standard::<crate::plugin::WriterApps>().subsets.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn subset_dialect_is_the_canonical_writer_dialect() {
    assert_eq!(crate::artifacts::writer::standards::v1::subsets::any::subset::<crate::plugin::WriterApps>().dialect, crate::artifacts::writer::WRITER_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn subset_declares_ten_io_entries() {
    assert_eq!(crate::artifacts::writer::standards::v1::subsets::any::subset::<crate::plugin::WriterApps>().io.entries.len(), 10);
}
