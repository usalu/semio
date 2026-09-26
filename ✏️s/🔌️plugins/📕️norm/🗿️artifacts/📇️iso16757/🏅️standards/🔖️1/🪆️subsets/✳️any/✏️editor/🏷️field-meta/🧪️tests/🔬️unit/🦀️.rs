//! 🧪️ `tests` — moved out of `🏷️field-meta/🦀️.rs` into its canonical test implementation.
use super::*;

#[test]
fn top_level_paths_are_localized_en_and_de() {
    let catalogue = iso16757_field_meta("catalogue").expect("catalogue");
    assert_eq!(catalogue.label_en, "Catalogue");
    assert_eq!(catalogue.label_de, "Katalog");
}

#[test]
fn list_wildcards_match_id_selectors() {
    let meta = iso16757_field_meta("catalogue.products[id=product-cv].variants[id=variant-50].geometryId").expect("wildcard");
    assert_eq!(meta.label_en, "Geometry id");
    let bounds = iso16757_field_meta("geometry.objects.geom-valve-50.spaces[id=installation].bounds.min").expect("bounds");
    assert_eq!(bounds.unit, Some("m"));
    assert_eq!(bounds.label_de, "Minimum");
}

#[test]
fn choices_are_localized_not_raw_codes() {
    let exchange = iso16757_field_meta("exchangeProcess").expect("exchange");
    let choices = exchange.choices.expect("choices");
    assert!(choices.iter().any(|c| c.value == "determineProduct" && c.label_de == "Produkt bestimmen"));
    let op = iso16757_field_meta("selection.constraints[0].operator").expect("operator");
    let op_choices = op.choices.expect("op choices");
    assert!(op_choices.iter().any(|c| c.value == "equal" && c.label_de == "Gleich" && c.label_en == "Equal"));
}

#[test]
fn edition_profile_choices_are_localized() {
    let meta = iso16757_field_meta("catalogue.metadata.editionProfile").expect("edition");
    let choices = meta.choices.expect("choices");
    assert!(choices.iter().any(|c| c.value == "fullPublished" && c.label_en == "Full published edition" && c.label_de == "Vollständig veröffentlichte Ausgabe"));
    assert!(choices.iter().any(|c| c.value == "part5_2025" && c.label_de == "Teil 5 (2025)"));
}

#[test]
fn part_number_rule_source_has_explicit_meta() {
    let meta = iso16757_field_meta("partNumberRule.source").expect("source");
    assert_eq!(meta.label_en, "Part-number rule source");
    assert_eq!(meta.label_de, "Teilenummernregel-Quelltext");
}
