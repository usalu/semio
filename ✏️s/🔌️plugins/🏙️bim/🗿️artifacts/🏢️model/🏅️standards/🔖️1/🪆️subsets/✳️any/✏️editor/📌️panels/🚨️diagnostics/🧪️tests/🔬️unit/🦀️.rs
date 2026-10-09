use super::*;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode;
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

const DEFECTS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/⚠️diagnostics/💥️defects/📸️snapshot/🔣️.json");
const CLEAN: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/⚠️diagnostics/🏡️clean/📸️snapshot/🔣️.json");

fn model(text: &str) -> (ModelSnapshot, ModelInference) {
    let snapshot: ModelSnapshot = from_json_str(text, JsonMemberPolicy::Reject).expect("fixture decodes");
    let inference = ModelInference::infer(&snapshot).expect("infers");
    (snapshot, inference)
}

fn text_with(snapshot: &ModelSnapshot, inference: &ModelInference, view: &ViewModel) -> String {
    let node = render(snapshot, inference, crate::editor::bim::terminology::bim_labels(view), &TreeWindows::for_body(view, BODY_KEY)).expect("the panel renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

fn text(snapshot: &ModelSnapshot, inference: &ModelInference, locale: Locale) -> String {
    text_with(snapshot, inference, &ViewModel::new(locale, Terminology::Native))
}

fn demo_with(found: Vec<Diagnostic>) -> (ModelSnapshot, ModelInference) {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    (snapshot, ModelInference { diagnostics: found, ..ModelInference::default() })
}

#[semio_framework_async_macros::async_test]
async fn the_groups_partition_the_findings_by_severity_storey_and_kind() {
    let (snapshot, inference) = model(DEFECTS);
    let found = &inference.diagnostics;
    let mut seen: Vec<usize> = SEVERITIES.iter().flat_map(|severity| groups(&snapshot, found, *severity)).flat_map(|group| group.kinds.into_iter().flat_map(|kind| kind.findings)).collect();
    seen.sort_unstable();
    assert_eq!(seen, (0..found.len()).collect::<Vec<_>>(), "every finding is in exactly one group");
    for severity in SEVERITIES {
        for group in groups(&snapshot, found, severity) {
            assert!(!group.kinds.is_empty() && group.total() > 0);
            for kind in &group.kinds {
                assert!(kind.findings.iter().all(|index| found[*index].severity == severity && found[*index].storey == group.storey && found[*index].code.category() == kind.category));
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn the_group_table_lists_the_severities_in_order_with_a_count_per_kind() {
    let (snapshot, inference) = model(DEFECTS);
    let table: serde_json::Value = serde_json::from_str(&groups_json(&snapshot, &inference.diagnostics)).expect("JSON");
    assert_eq!(table.as_object().expect("an object").keys().cloned().collect::<std::collections::BTreeSet<_>>(), ["error", "note", "warning"].map(String::from).into());
    let total: f64 = table.as_object().expect("an object").values().flat_map(|rows| rows.as_array().expect("rows").iter()).flat_map(|row| row["kinds"].as_object().expect("kinds").values()).map(|count| count.as_f64().expect("count")).sum();
    assert_eq!(total as usize, inference.diagnostics.len(), "every finding is counted once");
}

#[semio_framework_async_macros::async_test]
async fn storeys_follow_their_level_with_the_unknown_after_and_the_whole_model_last() {
    let mut found = vec![Diagnostic::new(DiagnosticCode::RefWallType, &["w-south"]).on("st-first"), Diagnostic::new(DiagnosticCode::RefWallType, &["w-east"]).on("st-ground"), Diagnostic::new(DiagnosticCode::RefWallType, &["w-north"]).on("st-gone"), Diagnostic::new(DiagnosticCode::RefWallType, &["w-west"])];
    let (snapshot, _) = demo_with(Vec::new());
    let order = |found: &[Diagnostic]| groups(&snapshot, found, Severity::Error).into_iter().map(|group| group.storey).collect::<Vec<_>>();
    let expected = vec![Some("st-ground".to_string()), Some("st-first".to_string()), Some("st-gone".to_string()), None];
    assert_eq!(order(&found), expected);
    found.reverse();
    assert_eq!(order(&found), expected, "the order does not depend on the order of the findings");
}

#[semio_framework_async_macros::async_test]
async fn kinds_follow_the_order_of_the_codes() {
    let found = vec![Diagnostic::new(DiagnosticCode::RefWallType, &["w-south"]).on("st-ground"), Diagnostic::new(DiagnosticCode::ClashWallWall, &["w-south", "w-east"]).on("st-ground"), Diagnostic::new(DiagnosticCode::ClashColumnColumn, &["c-1", "c-2"]).on("st-ground")];
    let (snapshot, _) = demo_with(Vec::new());
    let errors = groups(&snapshot, &found, Severity::Error);
    let kinds: Vec<&str> = errors.iter().flat_map(|group| group.kinds.iter().map(|kind| kind.category)).collect();
    assert_eq!(kinds, vec!["clash", "reference"]);
    assert_eq!(groups(&snapshot, &found, Severity::Warning)[0].kinds[0].findings, vec![1]);
}

#[semio_framework_async_macros::async_test]
async fn the_defect_model_lists_its_findings_by_severity_with_counts_in_english_and_german() {
    let (snapshot, inference) = model(DEFECTS);
    let english = text(&snapshot, &inference, Locale::En);
    let counts = |severity: Severity| inference.diagnostics.iter().filter(|found| found.severity == severity).count();
    assert!(english.contains(&format!("Errors ({})", counts(Severity::Error))) && english.contains(&format!("Warnings ({})", counts(Severity::Warning))), "{english}");
    for expected in ["Clashes", "References", "Wall w-missing-type uses the missing wall type"] {
        assert!(english.contains(expected), "the panel shows '{expected}': {english}");
    }
    let german = text(&snapshot, &inference, Locale::De);
    assert!(german.contains(&format!("Fehler ({})", counts(Severity::Error))) && german.contains("Kollisionen") && german.contains("Verweise"), "{german}");
    assert!(german.contains("Wand w-missing-type verwendet den fehlenden Wandtyp"), "{german}");
    assert!(!german.contains("Errors (") && !german.contains("Clashes"));
}

#[semio_framework_async_macros::async_test]
async fn a_finding_row_says_its_severity_in_words_and_activates_select_findings() {
    let (snapshot, inference) = model(DEFECTS);
    let rendered = text(&snapshot, &inference, Locale::En);
    assert!(rendered.contains("Error ·") && rendered.contains("selectFindings"), "{rendered}");
    assert!(rendered.contains("alert-circle") && rendered.contains(&format!("{ROOT}.finding.")), "{rendered}");
}

#[semio_framework_async_macros::async_test]
async fn a_clean_model_says_so_in_both_languages() {
    let (snapshot, inference) = model(CLEAN);
    assert!(text(&snapshot, &inference, Locale::En).contains("No problems found"));
    assert!(text(&snapshot, &inference, Locale::De).contains("Keine Probleme gefunden"));
}

#[semio_framework_async_macros::async_test]
async fn the_notes_group_is_closed_until_the_author_opens_it() {
    let (snapshot, inference) = demo_with(vec![Diagnostic::new(DiagnosticCode::TagEmpty, &["tag-1"]).on("st-ground"), Diagnostic::new(DiagnosticCode::RefWallType, &["w-south"]).on("st-ground")]);
    let closed = text(&snapshot, &inference, Locale::En);
    assert!(closed.contains("Notes (1)") && closed.contains("Errors (1)") && closed.contains("uses the missing wall type"), "{closed}");
    assert!(!closed.contains("prints nothing"), "a closed group builds no rows: {closed}");
    let mut view = ViewModel::new(Locale::En, Terminology::Native);
    view.tree_windows = vec![semio_framework_plugin::TreeWindowRequest { body_key: BODY_KEY.into(), node_key: format!("{ROOT}.note"), open: Some(true), offset: 0, rows: 20 }];
    let opened = text_with(&snapshot, &inference, &view);
    assert!(opened.contains("prints nothing"), "{opened}");
}

#[semio_framework_async_macros::async_test]
async fn a_long_list_of_one_kind_builds_no_rows_until_the_author_opens_it() {
    let found = (0..600).map(|index| Diagnostic::new(DiagnosticCode::RefWallType, &[format!("w-{index:03}").as_str()]).on("st-ground").lacking("wt-gone")).collect();
    let (snapshot, inference) = demo_with(found);
    let rendered = text(&snapshot, &inference, Locale::En);
    let built = rendered.matches(&format!("{ROOT}.finding.")).count();
    assert!(rendered.contains("Errors (600)") && built <= 60, "{built} rows were built for 600 findings");
}

#[semio_framework_async_macros::async_test]
async fn every_category_of_a_code_has_a_label_in_both_languages() {
    for category in categories() {
        for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
            assert_ne!(category_label(labels, category), category, "the category '{category}' needs a diag_cat label");
        }
    }
    assert_eq!(category_label(&BimLabels::NATIVE_EN, "clash"), "Clashes");
    assert_eq!(category_label(&BimLabels::NATIVE_DE, "clash"), "Kollisionen");
}

#[semio_framework_async_macros::async_test]
async fn long_messages_are_clipped_to_the_fixed_text_capacity() {
    assert_eq!(clip("short"), "short");
    let clipped = clip(&"ä".repeat(1000));
    assert_eq!(clipped.chars().count(), CLIP);
    assert!(clipped.len() < 512 && clipped.ends_with('…'));
}
