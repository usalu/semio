use super::codec::{items_of, member, number_of, read_value, text_of};
use super::*;
use crate::standards::v1::subsets::any::io::text::inferences::diagnostics::table_json;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode;
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const DEFECTS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/⚠️diagnostics/💥️defects/📸️snapshot/🔣️.json");
const CLEAN: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/⚠️diagnostics/🏡️clean/📸️snapshot/🔣️.json");

fn inferred(text: &str) -> ModelInference {
    let snapshot: ModelSnapshot = from_json_str(text, JsonMemberPolicy::Reject).expect("fixture decodes");
    ModelInference::infer(&snapshot).expect("infers")
}

#[test]
fn the_document_has_the_counts_a_finding_per_finding_and_the_index_by_element_and_storey() {
    let inference = inferred(DEFECTS);
    let root = read_value(&diagnostics_json(&inference)).expect("the document is JSON");
    let total = member(&root, "counts").and_then(|counts| member(counts, "total")).and_then(number_of).expect("the total");
    assert_eq!(total as usize, inference.diagnostics.len());
    let findings = items_of(member(&root, "findings").expect("findings"));
    assert_eq!(findings.len(), inference.diagnostics.len());
    for (json, found) in findings.iter().zip(&inference.diagnostics) {
        assert_eq!(member(json, "code").and_then(text_of), Some(found.code.slug()));
        assert_eq!(member(json, "severity").and_then(text_of), Some(severity_token(found.severity).as_str()));
        assert_eq!(member(json, "category").and_then(text_of), Some(found.code.category()));
        assert_eq!(items_of(member(json, "elements").expect("elements")).iter().filter_map(text_of).collect::<Vec<_>>(), found.elements);
        let message = member(json, "message").expect("message");
        assert_eq!(member(message, "en").and_then(text_of), found.text("en").as_deref());
        assert_eq!(member(message, "de").and_then(text_of), found.text("de").as_deref());
        assert_ne!(member(message, "en").and_then(text_of), member(message, "de").and_then(text_of), "the languages differ");
    }
    let elements = member(&root, "elements").expect("elements");
    assert_eq!(codec::members_of(elements).len(), inference.diagnostic_index.elements.len());
    let storeys = member(&root, "storeys").expect("storeys");
    assert_eq!(codec::members_of(storeys).len(), inference.diagnostic_index.storeys.len());
}

#[test]
fn a_finding_without_a_storey_has_a_null_storey_and_the_numbers_read_back_exactly() {
    let mut found = Diagnostic::new(DiagnosticCode::ClashWallWall, &["w-a", "w-b"]).with("overlap_area", 0.1 + 0.2).with("overlap_volume", f64::NAN);
    found.storey = None;
    let json = finding_value(&found);
    assert_eq!(member(&json, "storey"), Some(&JsonValue::Null));
    let values = member(&json, "values").expect("values");
    assert_eq!(member(values, "overlap_area").and_then(number_of), Some(0.1 + 0.2), "the shortest text that reads back as the same f64");
    assert_eq!(member(values, "overlap_volume"), Some(&JsonValue::Null), "a number that is not finite is null");
}

#[test]
fn a_clean_model_exports_an_empty_document_and_the_text_is_deterministic() {
    let inference = inferred(CLEAN);
    let text = diagnostics_json(&inference);
    assert_eq!(text, diagnostics_json(&inferred(CLEAN)));
    assert!(text.ends_with("}\n"));
    let root = read_value(&text).expect("JSON");
    assert!(items_of(member(&root, "findings").expect("findings")).is_empty());
    assert_eq!(member(&root, "counts").and_then(|counts| member(counts, "total")).and_then(number_of), Some(0.0));
}

#[test]
fn the_adjudicated_findings_read_back_from_the_export_equal_the_table_of_the_inference() {
    let inference = inferred(DEFECTS);
    assert_eq!(adjudicated_json(&diagnostics_json(&inference)), Ok(table_json(&inference.diagnostics)));
    assert!(adjudicated_json("not json").is_err());
    assert!(adjudicated_json("{}").is_err());
}

#[test]
fn the_serializer_targets_the_stdio_json_dialect() {
    assert_eq!(JSON_DIALECT.artifact_kind, "s.stdio.json");
    assert_eq!(<ModelIntoJson as Serializer<ModelSnapshot>>::INTO, JSON_DIALECT);
}

/// 📁️ The directory of the committed exports of the findings of the defect house: the files the python `csv` and `json` modules read back in the export case.
const EXPORT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/💡️inferences/⚠️diagnostics/💥️defects/📤️export");

#[test]
fn the_committed_json_file_is_the_current_export_of_the_defect_house() {
    let text = diagnostics_json(&inferred(DEFECTS));
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(EXPORT_DIR).expect("the fixture directory");
        std::fs::write(format!("{EXPORT_DIR}/⚠️diagnostics.json"), text.as_bytes()).expect("the file is written");
    }
    let committed = std::fs::read(format!("{EXPORT_DIR}/⚠️diagnostics.json")).unwrap_or_else(|error| panic!("{error}. Run the test with BIM_BLESS=1 to write the file."));
    assert_eq!(String::from_utf8(committed).expect("UTF-8"), text, "the committed export drifted: rewrite it with BIM_BLESS=1");
}
