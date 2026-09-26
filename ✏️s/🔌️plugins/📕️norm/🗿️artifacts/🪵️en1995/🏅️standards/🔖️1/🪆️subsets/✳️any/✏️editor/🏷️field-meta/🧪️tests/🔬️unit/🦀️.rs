//! 🧪️ `tests` — moved out of `🏷️field-meta/🦀️.rs` into its canonical test implementation.
use super::*;

#[test]
fn every_editable_leaf_has_meta_with_both_labels() {
    for (path, meta) in TABLE {
        assert!(!meta.label_en.is_empty() && !meta.label_de.is_empty(), "{path}");
        let resolved = en1995_field_meta(&path.replace("[]", "[0]")).unwrap_or_else(|| panic!("{path} must resolve through an index"));
        assert_eq!(resolved.label_en, meta.label_en);
    }
    assert!(en1995_field_meta("members[3].actions[1].qLineNPerM").is_some_and(|m| m.unit == Some("N/m")));
    assert!(en1995_field_meta("connections[0].actions[0].fKN").is_some_and(|m| m.unit == Some("N")));
    assert!(en1995_field_meta("members[0].strengthClass").is_some_and(|m| m.choices.is_some_and(|c| c.iter().any(|x| x.value == "GL24h" && x.label_de.contains("Brettschichtholz")))));
    assert!(en1995_field_meta("members[0].role").is_some_and(|m| m.choices.is_some_and(|c| c.len() == 4)));
}

#[test]
fn strength_choices_are_exactly_the_tabulated_classes() {
    let values: Vec<&str> = STRENGTH.iter().map(|c| c.value).collect();
    assert_eq!(values, crate::artifact_schema::strength_class_options());
    for value in values {
        assert!(crate::artifact_schema::properties_for_class(value).is_some(), "{value} must be tabulated");
    }
}
