
use super::SetLocale;
use crate::editor::writer::testkit::{dispatch, new_app, render};
use crate::editor::writer::{WriterCommand, WRITER_PLAY_BODY_INSPECTION};

#[semio_framework_async_macros::async_test]
async fn writer_labels_resolve_native_english_and_german() {
    let mut app = new_app().await;
    let english = render(&mut app, WRITER_PLAY_BODY_INSPECTION).await;
    assert!(english.contains("\"Document\"") && english.contains("\"Camera\""), "english labels: {english}");
    dispatch(&mut app, WriterCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
    let german = render(&mut app, WRITER_PLAY_BODY_INSPECTION).await;
    assert!(german.contains("Dokument") && german.contains("Kamera"), "german labels: {german}");
}
