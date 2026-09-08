
use super::*;
use crate::editor::forms::FORMS_PLAY_BODY_INSPECTION as BODY_INSPECTION;
use crate::editor::forms::testkit::{forms_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_inspector_always_shows_the_document_summary() {
    let mut app = forms_app().await;
    let json = render_body(&mut app, BODY_INSPECTION).await;
    assert!(json.contains("forms-play-inspector.summary"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body() {
    let definition = definition();
    assert_eq!(definition.body_key.as_deref(), Some(FORMS_PLAY_BODY_INSPECTION));
    assert!(matches!(definition.group, PanelGroup::Details));
}
