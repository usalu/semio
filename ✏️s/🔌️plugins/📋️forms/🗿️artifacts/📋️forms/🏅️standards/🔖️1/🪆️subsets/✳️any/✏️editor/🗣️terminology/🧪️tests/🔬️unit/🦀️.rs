use crate::editor::forms::terminology::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(forms_play_labels(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).kind_boolean.as_str(), "Boolean");
    assert_eq!(forms_play_labels(&semio_plugin::ViewModel { locale: semio_ui_locale::Locale::De, ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::De, semio_ui_locale::Terminology::Native) }).kind_boolean.as_str(), "Boolescher Wert");
}
