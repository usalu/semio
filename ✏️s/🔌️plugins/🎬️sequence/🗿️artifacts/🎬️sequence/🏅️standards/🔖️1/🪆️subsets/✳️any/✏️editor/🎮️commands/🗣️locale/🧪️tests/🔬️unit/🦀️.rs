
use crate::editor::sequence::SequenceCommand;
use crate::editor::sequence::testkit::{dispatch, new_app, render};

use super::set_locale::SetLocale;

#[semio_framework_async_macros::async_test]
async fn sequence_labels_render_native_english_by_default() {
    let mut app = new_app().await;
    let document_json = render(&mut app, crate::editor::sequence::panels::document::SEQUENCE_PLAY_BODY_DOCUMENT).await;
    assert!(document_json.contains("\"Steps\""));
    assert!(document_json.contains("\"Flow edges\""));
}

#[semio_framework_async_macros::async_test]
async fn sequence_labels_render_german_locale() {
    let mut app = new_app().await;
    dispatch(&mut app, SequenceCommand::SetLocale(SetLocale { value: "de".into() })).await;
    let document_json = render(&mut app, crate::editor::sequence::panels::document::SEQUENCE_PLAY_BODY_DOCUMENT).await;
    assert!(document_json.contains("Schritte"));
    assert!(document_json.contains("Ablaufkanten"));
    assert!(!document_json.contains("\"Steps\""));
}
