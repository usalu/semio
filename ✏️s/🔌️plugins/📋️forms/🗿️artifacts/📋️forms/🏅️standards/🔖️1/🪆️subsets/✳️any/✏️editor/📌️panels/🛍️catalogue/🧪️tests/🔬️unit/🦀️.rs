use super::*;
use crate::editor::forms::unit_tests::context::{forms_app, render as render_body};
use crate::editor::forms::FORMS_PLAY_BODY_CATALOGUE as BODY_CATALOGUE;

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_question_kinds() {
    let mut app = forms_app().await;
    let json = render_body(&mut app, BODY_CATALOGUE).await;
    assert!(json.contains("forms-play-catalogue.text"));
    assert!(json.contains("forms-play-catalogue.add-step"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_items_export_the_question_kind_drag_mime() {
    let mut app = forms_app().await;
    let json = render_body(&mut app, BODY_CATALOGUE).await;
    assert!(json.contains(FORMS_QUESTION_DRAG_MIME));
    assert!(json.contains(r#""draggable":true"#) || json.contains(r#""draggable": true"#));
}
