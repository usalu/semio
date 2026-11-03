//! 🧪️ `tests` — moved out of `🏷️field-meta/🦀️.rs` into its canonical test implementation.

use super::*;
#[test]
fn every_editable_leaf_has_meta_with_both_labels() {
    for (path, meta) in TABLE {
        assert!(!meta.label.en.is_empty() && !meta.label.de.is_empty(), "{path}");
        let resolved = en1995_field_meta(&path.replace("[]", "[0]")).unwrap_or_else(|| panic!("{path} must resolve through an index"));
        assert_eq!(resolved.label.en, meta.label.en);
    }
    assert!(en1995_field_meta("members[3].actions[1].qLineNPerM").is_some_and(|m| m.unit() == Some("N/m")));
    assert!(en1995_field_meta("connections[0].actions[0].fKN").is_some_and(|m| m.unit() == Some("N")));
    assert!(en1995_field_meta("members[0].strengthClass").is_some_and(|m| m.choices().is_some_and(|c| c.iter().any(|x| x.value == "GL24h" && x.label_de.contains("Brettschichtholz")))));
    assert!(en1995_field_meta("members[0].role").is_some_and(|m| m.choices().is_some_and(|c| c.len() == 4)));
}

#[test]
fn strength_choices_are_exactly_the_tabulated_classes() {
    let values: Vec<&str> = STRENGTH.iter().map(|c| c.value).collect();
    assert_eq!(values, semio_s_artifact_norm_en1995_schema::checks::timber::strength_class_options());
    for value in values {
        assert!(semio_s_artifact_norm_en1995_schema::checks::timber::properties_for_class(value).is_some(), "{value} must be tabulated");
    }
}

#[test]
fn catalogue_case_inputs_resolve_to_their_typed_vocabulary_row() {
    let b = en1995_field_meta("cases[id=case-1].numbers[id=b].value").expect("b");
    assert_eq!((b.unit(), b.label.de), (Some("mm"), "Breite b"));
    let f_u = en1995_field_meta("cases[id=case-1].numbers[id=f_u].value").expect("f_u");
    assert_eq!(f_u.unit(), Some("N/mm²"));
    let kind = en1995_field_meta("cases[id=case-1].texts[id=kind].value").expect("kind");
    assert!(kind.choices().is_some_and(|c| c.iter().any(|x| x.value == "screw" && x.label_de == "Schraube")));
    assert!(en1995_field_meta("cases[id=case-1].numbers[id=not_in_the_vocabulary].value").is_some_and(|m| m.label.en == "Number value"));
}
