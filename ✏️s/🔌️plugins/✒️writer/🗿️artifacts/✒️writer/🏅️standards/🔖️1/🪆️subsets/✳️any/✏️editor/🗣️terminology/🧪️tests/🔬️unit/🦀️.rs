use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(writer_play_labels(&WriterConfig::default()).document.as_str(), "Document");
    assert_eq!(writer_play_labels(&WriterConfig { locale: "de-DE".into(), ..WriterConfig::default() }).document.as_str(), "Dokument");
}
