
use super::*;
use crate::editor::forms::testkit::{dispatch, forms_app, render};
use crate::editor::forms::{FORMS_PLAY_BODY_BLUEPRINT, FormsCommand};

#[semio_framework_async_macros::async_test]
async fn forms_labels_resolve_native_english_and_german() {
    let mut app = forms_app().await;
    let english = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT).await;
    assert!(english.contains("Boolean"), "english labels: {english}");
    dispatch(&mut app, FormsCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
    let german = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT).await;
    assert!(german.contains("Boolescher Wert"), "german labels: {german}");
}
