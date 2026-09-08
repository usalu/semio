
use super::*;

#[test]
fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(generation2d_labels(&Generation2dConfig::default()).sources.as_str(), "Sources");
    assert_eq!(generation2d_labels(&Generation2dConfig { locale: "de-DE".into(), ..Generation2dConfig::default() }).sources.as_str(), "Quellen");
}
