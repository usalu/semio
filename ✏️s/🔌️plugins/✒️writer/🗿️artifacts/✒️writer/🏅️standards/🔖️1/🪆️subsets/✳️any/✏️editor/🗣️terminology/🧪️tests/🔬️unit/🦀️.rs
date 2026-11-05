use crate::editor::writer::terminology::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(writer_play_labels(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).artifact.as_str(), "Artifact");
    assert_eq!(writer_play_labels(&semio_plugin::ViewModel { locale: semio_ui_locale::Locale::De, ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::De, semio_ui_locale::Terminology::Native) }).artifact.as_str(), "Artefakt");
}
