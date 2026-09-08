
use super::*;

#[semio_framework_async_macros::async_test]
async fn shooting_labels_resolve_native_english_by_default() {
    assert_eq!(shooting_play_labels(&ShootingConfig::default()).shots.as_str(), "Shots");
}

#[semio_framework_async_macros::async_test]
async fn shooting_labels_resolve_german_from_the_config_locale() {
    assert_eq!(shooting_play_labels(&ShootingConfig { locale: "de-DE".into(), ..ShootingConfig::default() }).shots.as_str(), "Aufnahmen");
}
