//! 🧮️ ISO descendant edits retain typed script semantics and recompute the real part number.

use semio_framework_value::DslValue;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_artifact_norm_iso16757_mutations::{resolve_edit, Iso16757Mutation};
use semio_s_plugin_norm_app_surface::NormEdit;
use semio_s_plugin_norm_compliance::CheckStatus;

#[test]
fn a_nested_script_source_edit_recomputes_the_real_part_number_without_mutating_the_original() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("business fixture");
    let number = |key: &str| law[key].as_f64().expect("fixture number");
    let document = Iso16757Snapshot::reference_fixture();
    let original = serde_json::to_value(&document).expect("independent original snapshot");
    let original_rule = serde_json::to_value(&document.part_number_rule).expect("independent original rule");
    assert_eq!(original_rule["kind"], "script");
    assert_eq!(original_rule["functionId"], law["functionId"]);
    assert_eq!(original_rule["source"], law["sourceBefore"]);
    assert_eq!(number("computedBefore"), number("dn") * number("multiplier") + number("offsetBefore"));
    assert_eq!(number("computedAfter"), number("dn") * number("multiplier") + number("offsetAfter"));
    let before = semio_s_artifact_norm_iso16757_inferences::evaluate(&document);
    let check_before = before.checks.iter().find(|check| check.id == law["checkId"].as_str().expect("check id")).expect("original part-number check");
    assert_eq!(check_before.computed.value, number("computedBefore"));
    assert_eq!(check_before.status, CheckStatus::Pass);
    let operations = resolve_edit(&document, &NormEdit::SetField {
        path: law["path"].as_str().expect("path").into(),
        value: DslValue::String(law["value"].as_str().expect("script source").into()),
    }).expect("registered typed parent edit");
    let [Iso16757Mutation::ReplacePartNumberRule(payload)] = operations.as_slice() else { panic!("one typed part-number-rule mutation") };
    let mut expected_rule = original_rule;
    *expected_rule.pointer_mut("/source").expect("independent source pointer") = law["value"].clone();
    assert_eq!(serde_json::to_value(&payload.new_rule).expect("typed rule payload"), expected_rule);
    assert_eq!(serde_json::to_value(&document).expect("original after resolution"), original);
    let (changed, messages) = crate::protocol::apply_mutation(&document, &operations[0]).expect("native typed mutation");
    assert!(messages.is_empty(), "typed mutation must be admitted without diagnostics: {messages:?}");
    assert_eq!(serde_json::to_value(&document).expect("original after application"), original);
    assert_eq!(serde_json::to_value(&changed.part_number_rule).expect("applied typed rule"), expected_rule);
    let after = semio_s_artifact_norm_iso16757_inferences::evaluate(&changed);
    let check_after = after.checks.iter().find(|check| check.id == law["checkId"].as_str().expect("check id")).expect("changed part-number check");
    assert_eq!(check_after.computed.value, number("computedAfter"));
    assert_eq!(check_after.status, CheckStatus::Pass);
}

#[test]
fn a_rendered_product_row_removes_its_owned_identity_without_mutating_the_original() {
    let law: serde_json::Value = serde_json::from_str(include_str!("remove.json")).expect("row removal fixture");
    let document = Iso16757Snapshot::reference_fixture();
    let original = serde_json::to_value(&document).expect("independent original");
    let collection = law["collection"].as_str().expect("collection");
    let pointer = format!("/{}", collection.replace('.', "/"));
    let products = original.pointer(&pointer).and_then(serde_json::Value::as_array).expect("actual products");
    let id = products[0]["id"].as_str().expect("actual rendered row identity");
    let operations = resolve_edit(&document, &NormEdit::RemoveItem {
        path: format!("{collection}[id={id}]"),
        index: law["index"].as_u64().expect("index") as usize,
    }).expect("rendered row has a typed removal route");
    assert_eq!(operations.len(), 1);
    let (changed, messages) = crate::protocol::apply_mutation(&document, &operations[0]).expect("typed row removal");
    assert!(messages.is_empty(), "typed removal diagnostics: {messages:?}");
    let mut expected = original.clone();
    expected.pointer_mut(&pointer).and_then(serde_json::Value::as_array_mut).expect("independent collection").remove(0);
    assert_eq!(serde_json::to_value(&changed).expect("changed document"), expected);
    assert_eq!(serde_json::to_value(&document).expect("unchanged original"), original);
}
