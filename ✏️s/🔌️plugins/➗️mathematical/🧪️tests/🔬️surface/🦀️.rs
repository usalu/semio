//! 🧪️ The editor/viewer pair's own cross-surface guarantees (contract §2.5), using the landed
//! framework testkit directly: `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
//! assert_editor_and_viewer_share_dialect, new_viewer}`.
use crate::editor::equation::EquationPlayApp;
use crate::viewer::equation::EquationViewer;

#[semio_framework_async_macros::async_test]
async fn equation_viewer_never_mutates() {
    semio_framework_plugin::testkit::assert_viewer_never_mutates::<EquationViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn equation_editor_and_viewer_share_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<EquationPlayApp, EquationViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn equation_viewer_instantiates_through_new_viewer() {
    let _app = semio_framework_plugin::testkit::new_viewer::<EquationViewer>().await;
}
