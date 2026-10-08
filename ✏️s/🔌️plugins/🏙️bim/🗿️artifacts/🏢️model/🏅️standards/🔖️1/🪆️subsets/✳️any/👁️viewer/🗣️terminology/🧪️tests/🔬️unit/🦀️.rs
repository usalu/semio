use super::*;
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

fn labels(locale: Locale, terminology: Terminology) -> &'static BimViewerLabels {
    bim_viewer_labels(&ViewModel::new(locale, terminology))
}

#[test]
fn every_label_is_written_in_both_languages_and_both_terminologies() {
    for (english, german) in [(&BimViewerLabels::NATIVE_EN, &BimViewerLabels::NATIVE_DE), (&BimViewerLabels::REUSE_EN, &BimViewerLabels::REUSE_DE)] {
        let mut german_rows = Vec::new();
        german.for_each_label(|name, text| german_rows.push((name, text.as_str())));
        let mut count = 0;
        english.for_each_label(|name, text| {
            count += 1;
            assert!(!text.as_str().is_empty(), "{name} has no English text");
            assert!(german_rows.iter().any(|(other, translated)| *other == name && !translated.is_empty()), "{name} has no German text");
        });
        assert_eq!(count, BimViewerLabels::FIELD_NAMES.len());
    }
}

#[test]
fn labels_resolve_by_language_and_terminology() {
    assert_eq!(labels(Locale::En, Terminology::Native).storeys.as_str(), "Storeys");
    assert_eq!(labels(Locale::De, Terminology::Native).storeys.as_str(), "Geschosse");
    assert_eq!(labels(Locale::En, Terminology::Reuse).storeys.as_str(), "Levels");
    assert_eq!(labels(Locale::De, Terminology::Reuse).storeys.as_str(), "Ebenen");
}
