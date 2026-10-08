use super::*;
use semio_framework_ui_locale::{Locale, Terminology};

fn rows(labels: &BimLabels) -> Vec<(&'static str, String)> {
    let mut rows = Vec::new();
    labels.for_each_label(|name, text| rows.push((name, text.as_str().to_string())));
    rows
}

fn placeholders(text: &str) -> Vec<String> {
    text.split('{').skip(1).filter_map(|rest| rest.split('}').next()).map(str::to_string).collect()
}

#[semio_framework_async_macros::async_test]
async fn every_label_has_english_and_german_text() {
    let (en, de) = (rows(&BimLabels::NATIVE_EN), rows(&BimLabels::NATIVE_DE));
    assert_eq!(en.len(), BimLabels::FIELD_NAMES.len());
    assert_eq!(de.len(), en.len());
    for ((name, english), (_, german)) in en.iter().zip(&de) {
        assert!(!english.trim().is_empty() && !german.trim().is_empty(), "{name} is empty in a locale");
    }
}

#[semio_framework_async_macros::async_test]
async fn placeholders_agree_between_the_locales() {
    for ((name, english), (_, german)) in rows(&BimLabels::NATIVE_EN).iter().zip(&rows(&BimLabels::NATIVE_DE)) {
        assert_eq!(placeholders(english), placeholders(german), "{name} placeholders differ between English and German");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_locale_comes_from_the_view_model_alone() {
    let english = semio_framework_plugin::ViewModel::new(Locale::En, Terminology::Native);
    let german = semio_framework_plugin::ViewModel::new(Locale::De, Terminology::Native);
    assert_eq!(bim_labels(&english).kind_storey.as_str(), "Storey");
    assert_eq!(bim_labels(&german).kind_storey.as_str(), "Geschoss");
    assert_eq!(bim_labels(&german).locale(), Locale::De);
}

#[semio_framework_async_macros::async_test]
async fn placeholders_are_filled_by_name() {
    assert_eq!(BimLabels::named(BimLabels::NATIVE_DE.action_add_named, "Wand"), "Wand hinzufügen");
    assert_eq!(BimLabels::counted(BimLabels::NATIVE_EN.status_elements, 3), "3 elements");
}
