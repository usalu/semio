
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(vcs_play_labels(&VcsDemoConfig::default()).commit.as_str(), "Commit");
    assert_eq!(vcs_play_labels(&VcsDemoConfig { locale: "de-DE".into(), ..VcsDemoConfig::default() }).undo.as_str(), "Rückgängig");
}
