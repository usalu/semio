
use super::*;
use crate::editor::forms::FORMS_PLAY_BODY_DOCUMENT as BODY_DOCUMENT;
use crate::editor::forms::testkit::{forms_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_tree_declares_drop_action() {
    let mut app = forms_app().await;
    let json = render_body(&mut app, BODY_DOCUMENT).await;
    assert!(json.contains(r#""dropAction""#));
    assert!(json.contains("dropQuestionKind"));
}

#[semio_framework_async_macros::async_test]
async fn document_lists_steps() {
    let mut app = forms_app().await;
    let json = render_body(&mut app, BODY_DOCUMENT).await;
    assert!(json.contains("forms-play-document.steps"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(FORMS_PLAY_BODY_DOCUMENT));
}
