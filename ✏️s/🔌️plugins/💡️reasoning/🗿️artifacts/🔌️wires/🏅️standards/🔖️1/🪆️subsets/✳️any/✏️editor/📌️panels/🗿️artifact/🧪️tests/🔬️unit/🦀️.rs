use super::*;
use crate::editor::wires::testkit::{metabolism_app, render as render_body};
use crate::editor::wires::WIRES_PLAY_BODY_DOCUMENT as APP_BODY_DOCUMENT;

#[semio_framework_async_macros::async_test]
async fn document_has_identities_section() {
    let mut app = metabolism_app().await;
    let json = render_body(&mut app, APP_BODY_DOCUMENT).await;
    assert!(json.contains("wires-play-document.identities"));
    assert!(json.contains("Metabolism"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_DOCUMENT));
}
