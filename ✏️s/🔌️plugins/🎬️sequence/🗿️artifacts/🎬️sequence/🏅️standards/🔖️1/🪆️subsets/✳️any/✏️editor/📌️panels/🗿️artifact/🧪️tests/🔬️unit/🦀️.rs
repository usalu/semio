
use super::*;
use crate::editor::sequence::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_steps() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_DOCUMENT).await.contains("sequence-play-document.steps"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(SEQUENCE_PLAY_BODY_DOCUMENT));
}
