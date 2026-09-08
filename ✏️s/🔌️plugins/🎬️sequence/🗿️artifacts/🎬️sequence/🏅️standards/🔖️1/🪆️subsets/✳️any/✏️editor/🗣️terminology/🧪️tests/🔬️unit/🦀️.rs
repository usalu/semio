
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(sequence_play_labels(&SequenceConfig::default()).steps.as_str(), "Steps");
    assert_eq!(sequence_play_labels(&SequenceConfig { locale: "de-DE".into(), ..SequenceConfig::default() }).steps.as_str(), "Schritte");
}
