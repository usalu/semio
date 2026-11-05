use crate::editor::vcs::terminology::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(vcs_play_labels(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).commit.as_str(), "Commit");
    assert_eq!(vcs_play_labels(&semio_plugin::ViewModel { locale: semio_ui_locale::Locale::De, ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::De, semio_ui_locale::Terminology::Native) }).undo.as_str(), "Rückgängig");
}
