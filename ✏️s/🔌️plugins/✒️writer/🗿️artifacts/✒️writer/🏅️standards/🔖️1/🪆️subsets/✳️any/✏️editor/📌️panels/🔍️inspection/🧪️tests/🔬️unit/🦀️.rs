use super::*;
use crate::editor::writer::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(WRITER_PLAY_BODY_INSPECTION));
}

#[semio_framework_async_macros::async_test]
async fn writer_labels_resolve_native_by_default() {
    let mut app = new_app().await;
    let inspection = render_body(&mut app, WRITER_PLAY_BODY_INSPECTION).await;
    assert!(inspection.contains("\"Document\""));
    assert!(inspection.contains("\"Camera\""));
}
