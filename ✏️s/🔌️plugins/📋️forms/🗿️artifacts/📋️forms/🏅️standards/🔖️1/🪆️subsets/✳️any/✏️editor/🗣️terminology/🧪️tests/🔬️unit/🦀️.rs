
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(forms_play_labels(&FormsConfig::default()).kind_boolean.as_str(), "Boolean");
    assert_eq!(forms_play_labels(&FormsConfig { locale: "de-DE".into(), ..FormsConfig::default() }).kind_boolean.as_str(), "Boolescher Wert");
}
