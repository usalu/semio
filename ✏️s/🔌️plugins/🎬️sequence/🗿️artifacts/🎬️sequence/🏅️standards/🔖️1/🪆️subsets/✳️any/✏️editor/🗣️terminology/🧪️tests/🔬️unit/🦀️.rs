use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(sequence_play_labels(&semio_framework_plugin::ViewModel::default()).steps.as_str(), "Steps");
    assert_eq!(sequence_play_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() }).steps.as_str(), "Schritte");
}
