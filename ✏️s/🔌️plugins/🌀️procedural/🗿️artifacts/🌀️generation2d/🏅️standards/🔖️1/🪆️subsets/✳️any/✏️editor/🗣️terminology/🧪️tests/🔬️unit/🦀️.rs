
use super::*;

#[test]
fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(generation2d_labels(&semio_framework_plugin::ViewModel::default()).sources.as_str(), "Sources");
    assert_eq!(generation2d_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() }).sources.as_str(), "Quellen");
}
