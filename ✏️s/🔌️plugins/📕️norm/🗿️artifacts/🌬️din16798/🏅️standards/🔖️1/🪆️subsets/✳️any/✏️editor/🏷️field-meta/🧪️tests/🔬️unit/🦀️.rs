use super::*;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
use semio_s_artifact_norm_din16798_scene as scene;

fn documents() -> Vec<Din16798Snapshot> {
    vec![Din16798Snapshot::compliant_office(), Din16798Snapshot::residential_method3(), Din16798Snapshot::noncompliant_office()]
}

fn assert_described(path: &str, unit: &str) {
    let meta = din16798_field_meta(path).unwrap_or_else(|| panic!("{path} has no field meta"));
    assert!(!meta.label.en.is_empty() && !meta.label.de.is_empty(), "{path} has no English and German label");
    assert_ne!(meta.label.en, meta.label.de, "{path} label is not translated");
    if unit != "1" && unit != "token" {
        assert_eq!(meta.unit(), Some(unit), "{path} unit");
    }
}

#[test]
fn every_handle_write_path_has_bilingual_meta_with_the_unit_of_the_binding() {
    for document in documents() {
        for handle in scene::handles(&document, &scene::entities_of(&document)) {
            for write in &handle.writes {
                assert_described(&write.path, &write.unit);
            }
        }
    }
}

#[test]
fn every_diagram_marker_binding_shares_the_unit_of_the_inspector() {
    for document in documents() {
        let report = semio_s_artifact_norm_din16798_inferences::evaluate(&document);
        for diagram in scene::diagrams(&document, &report, None) {
            for marker in &diagram.markers {
                if let Some(binding) = &marker.binding {
                    assert_described(&binding.path, &binding.unit);
                }
            }
        }
    }
}

#[test]
fn the_category_and_method_choices_speak_both_languages() {
    for path in ["zones[id=zone-office].comfortCategory", "zones[id=zone-office].ventMethod", "zones[id=zone-office].pollutionClass", "zones[id=zone-office].comfortModel"] {
        let meta = din16798_field_meta(path).unwrap_or_else(|| panic!("{path}"));
        let choices = meta.choices().unwrap_or_else(|| panic!("{path} offers choices"));
        assert!(choices.iter().all(|choice| !choice.label_en.is_empty() && !choice.label_de.is_empty()));
    }
    let tokens: Vec<&str> = din16798_field_meta("zones[id=zone-office].comfortCategory").and_then(|m| m.choices().map(|c| c.iter().map(|x| x.value).collect())).expect("category choices");
    assert_eq!(tokens, ["I", "II", "III", "IV"]);
}
