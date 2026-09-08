
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(playbook_play_labels(&PlaybookConfig::default()).kind_arg.as_str(), "Kind");
    assert_eq!(playbook_play_labels(&PlaybookConfig { locale: "de-DE".into(), ..PlaybookConfig::default() }).kind_arg.as_str(), "Art");
}
