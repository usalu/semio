use super::*;

#[test]
fn labels_resolve_native_english_and_german_from_the_shared_view_state() {
    assert_eq!(generation3d_labels(&semio_framework_plugin::ViewModel::default()).widgets.as_str(), "Widgets");
    assert_eq!(generation3d_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() }).widgets.as_str(), "Elemente");
}

#[test]
fn unknown_catalog_kind_falls_back_to_the_id_itself() {
    let labels = generation3d_labels(&semio_framework_plugin::ViewModel::default());
    assert_eq!(generation3d_catalog_label("bogusKind", labels), "bogusKind");
}

/// 🗣️ The shared roster both halves of the i18n law answer.
fn terminology_fixture() -> serde_json::Value {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗣️terminology.json");
    serde_json::from_str(&std::fs::read_to_string(path).expect("the terminology fixture is readable")).expect("the terminology fixture is json")
}

/// 🗣️ Every user-visible generation3d label carries all four locale/terminology spellings, and each
/// one says exactly what the shared fixture declares.
///
/// `app_labels!` already makes a MISSING locale a compile error, so the gap this closes is the
/// locale that is PRESENT but wrong: English pasted into `native_de`, an empty string, or a field
/// added to the struct and never added to the roster the TypeScript twin checks. Walking
/// `FIELD_NAMES` rather than naming 36 fields by hand is what keeps the law true for label 37.
#[test]
fn every_user_visible_label_is_declared_in_every_locale_exactly_as_the_shared_fixture_says() {
    let fixture = terminology_fixture();
    let rows = fixture["labels"].as_object().expect("the fixture declares a label roster");
    let declared: std::collections::BTreeSet<&str> = Generation3dLabels::FIELD_NAMES.iter().copied().collect();
    let rostered: std::collections::BTreeSet<&str> = rows.keys().map(String::as_str).collect();
    assert_eq!(declared, rostered, "every declared label is rostered and every rostered label is declared");

    for (locale_key, set) in [
        ("nativeEn", &Generation3dLabels::NATIVE_EN),
        ("nativeDe", &Generation3dLabels::NATIVE_DE),
        ("reuseEn", &Generation3dLabels::REUSE_EN),
        ("reuseDe", &Generation3dLabels::REUSE_DE),
    ] {
        set.for_each_label(|field, text| {
            let expected = rows[field][locale_key].as_str().unwrap_or_else(|| panic!("{field}: the fixture declares {locale_key}"));
            assert_eq!(text.as_str(), expected, "{field}: {locale_key}");
            assert!(!text.as_str().is_empty(), "{field}: {locale_key} is never empty");
        });
    }
}

/// 🇩🇪️ No label ships English in its German slot. A handful of rows are the same word in both
/// languages by design (a proper noun, an international term); every other row must differ, which is
/// what makes "someone pasted the English across" a failing test rather than a reading exercise.
#[test]
fn german_is_a_translation_rather_than_a_copy_of_the_english() {
    let fixture = terminology_fixture();
    let identical: std::collections::BTreeSet<String> =
        fixture["identicalByDesign"].as_array().expect("declared").iter().map(|value| value.as_str().expect("string").to_string()).collect();
    let mut translated = 0_usize;
    let (english, german) = (&Generation3dLabels::NATIVE_EN, &Generation3dLabels::NATIVE_DE);
    let mut pairs: Vec<(&'static str, String)> = Vec::new();
    english.for_each_label(|field, text| pairs.push((field, text.as_str().to_string())));
    let mut index = 0_usize;
    german.for_each_label(|field, text| {
        let (english_field, english_text) = &pairs[index];
        index += 1;
        assert_eq!(*english_field, field, "both walks visit the same fields in the same order");
        if identical.contains(field) {
            assert_eq!(english_text, text.as_str(), "{field}: declared identical by design, so it must actually be identical");
        } else {
            assert_ne!(english_text.as_str(), text.as_str(), "{field}: the German slot still holds the English text");
            translated += 1;
        }
    });
    assert_eq!(translated, Generation3dLabels::FIELD_NAMES.len() - identical.len(), "every non-identical field was checked");
}
