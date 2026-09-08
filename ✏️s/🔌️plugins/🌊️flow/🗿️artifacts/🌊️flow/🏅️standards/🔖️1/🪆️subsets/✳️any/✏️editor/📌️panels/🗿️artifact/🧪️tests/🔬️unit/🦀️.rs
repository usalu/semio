
use super::*;
use crate::editor::flow::testkit::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_widgets() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_DOCUMENT).await.contains("flow-play-document.widgets"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(FLOW_PLAY_BODY_DOCUMENT));
}
