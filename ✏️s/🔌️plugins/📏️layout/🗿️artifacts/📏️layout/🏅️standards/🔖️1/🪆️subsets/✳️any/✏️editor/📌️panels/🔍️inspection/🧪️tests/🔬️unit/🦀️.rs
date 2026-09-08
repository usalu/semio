
use super::*;
use crate::editor::layout::testkit::{layout_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_inspector_always_summarises_the_document() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_INSPECTION).await;
    assert!(json.contains(LAYOUT_DOCUMENT_SCHEMA));
    assert!(json.contains("page-1"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_INSPECTION));
}
