//! 🧪️ Atomic typed edit staging and semantic admission laws.

use super::*;
use semio_framework_value::DslValue;

#[path = "🧫️model/🦀️.rs"]
pub(crate) mod model;

#[test]
fn sibling_writes_replay_against_the_preceding_typed_parent() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🪆️nested-edit/🧫️cases/🔣️.json")).expect("language-neutral cases");
    let bundle = &fixture["bundle"];
    let source = model::Document::from_value(DslValue::from(&bundle["document"])).expect("typed source");
    let writes: Vec<_> = bundle["writes"].as_array().expect("writes").iter().map(|write| FieldWrite { path: write["path"].as_str().expect("path").to_owned(), value: DslValue::from(&write["value"]) }).collect();
    let operations = resolve_set_fields(&source, &writes, model::resolve).expect("typed bundle");
    let expected = model::Document::from_value(DslValue::from(&bundle["expected"])).expect("typed expected output");
    assert_eq!(model::replay(&source, &operations), expected);
    assert_eq!(source, model::document());
    let mut oracle = bundle["document"].clone();
    for write in bundle["writes"].as_array().expect("writes") {
        *oracle.pointer_mut(write["pointer"].as_str().expect("JSON Pointer")).expect("existing target") = write["value"].clone();
    }
    assert_eq!(semio_framework_value::ToValue::to_value(&model::replay(&source, &operations)), DslValue::from(&oracle));
}

#[test]
fn typed_bundle_refuses_errors_and_fatals_but_preserves_warning_operations() {
    let source = model::document();
    let writes = [FieldWrite::number("configuration.a", 3.0), FieldWrite::number("configuration.b", 4.0)];
    for fatal in [false, true] {
        model::reset_retired();
        let mut calls = 0;
        let result = resolve_set_fields(&source, &writes, |document, edit| {
            calls += 1;
            if calls == 2 { return Ok(vec![if fatal { model::Operation::Fatal } else { model::Operation::Error }]) }
            model::resolve(document, edit)
        });
        assert!(result.is_err(), "semantic refusal must not emit a partial bundle");
        assert_eq!(model::retired(), 2, "every unissued operation retires through its domain owner");
        assert_eq!(source, model::document());
    }
    let operations = resolve_set_fields(&source, &writes, |document, edit| {
        model::resolve(document, edit).map(|operations| operations.into_iter().map(|operation| match operation { model::Operation::Replace(value) => model::Operation::Warning(value), other => other }).collect())
    }).expect("warnings remain admissible");
    assert_eq!(model::replay(&source, &operations).configuration, model::Configuration { a: 3.0, b: 4.0, count: 1 });
    assert!(operations.iter().all(|operation| matches!(operation, model::Operation::Warning(_))));
}

#[test]
fn public_typed_admission_keeps_exact_values_and_refuses_a_coercible_raw_draft() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🪆️nested-edit/🧫️cases/🔣️.json")).expect("typed admission cases");
    for case in fixture["typedAdmission"].as_array().expect("typed admission cases") {
        model::reset_retired();
        let source = model::document();
        let raw = DslValue::from(&case["value"]);
        let edits = [NormEdit::SetField { path: "configuration.a".into(), value: DslValue::float(3.0) }, NormEdit::SetField { path: "configuration.count".into(), value: raw.clone() }];
        let mut observed = None;
        let result = resolve_norm_edits(&source, &edits, |document, edit| {
            if let NormEdit::SetField { path, value } = edit {
                if path == "configuration.count" {
                    assert_eq!(document.configuration.a, 3.0);
                    observed = Some(value.clone());
                }
            }
            model::resolve(document, edit)
        });
        assert_eq!(observed, Some(raw), "{}", case["name"]);
        assert_eq!(result.is_ok(), case["accept"].as_bool().expect("accept"), "{}", case["name"]);
        assert_eq!(model::retired(), usize::from(!case["accept"].as_bool().expect("accept")));
        if let Ok(operations) = result {
            let output = model::replay(&source, &operations);
            let mut oracle = serde_json::json!({"configuration":{"a":1.0,"b":2.0,"count":1}});
            *oracle.pointer_mut("/configuration/a").expect("a") = serde_json::json!(3.0);
            *oracle.pointer_mut("/configuration/count").expect("count") = case["count"].clone();
            assert_eq!(semio_framework_value::ToValue::to_value(&output), DslValue::from(&oracle));
        }
        assert_eq!(source, model::document());
    }
}

#[test]
fn ui_numeric_adaptation_precedes_exact_typed_admission() {
    let source = model::document();
    let writes = [FieldWrite::number("configuration.a", 3.0), FieldWrite::number("configuration.count", 5.6)];
    let operations = resolve_set_fields(&source, &writes, model::resolve).expect("UI numeric adaptation");
    assert_eq!(model::replay(&source, &operations).configuration, model::Configuration { a: 3.0, b: 2.0, count: 6 });
    assert_eq!(source, model::document());
}
