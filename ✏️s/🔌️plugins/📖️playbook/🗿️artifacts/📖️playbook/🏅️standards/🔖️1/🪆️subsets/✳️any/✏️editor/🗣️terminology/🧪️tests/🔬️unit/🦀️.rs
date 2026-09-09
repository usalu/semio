use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(playbook_play_labels(&semio_framework_plugin::ViewModel::default()).kind_arg.as_str(), "Kind");
    assert_eq!(playbook_play_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() }).kind_arg.as_str(), "Art");
}
