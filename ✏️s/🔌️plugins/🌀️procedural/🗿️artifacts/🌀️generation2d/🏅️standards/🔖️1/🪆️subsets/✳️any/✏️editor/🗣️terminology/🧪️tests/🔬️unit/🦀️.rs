use super::*;

#[test]
fn labels_resolve_native_english_and_german_from_the_shared_view_state() {
    assert_eq!(generation2d_labels(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).sources.as_str(), "Sources");
    assert_eq!(generation2d_labels(&semio_plugin::ViewModel { locale: semio_ui_locale::Locale::De, ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::De, semio_ui_locale::Terminology::Native) }).sources.as_str(), "Quellen");
}
