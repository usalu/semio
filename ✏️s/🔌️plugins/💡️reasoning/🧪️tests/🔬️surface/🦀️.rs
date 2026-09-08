
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn wires_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::wires::WiresViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn wires_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::wires::ReasoningWiresPlayApp, crate::viewer::wires::WiresViewer>().await;
}
