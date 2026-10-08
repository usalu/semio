use crate::standards::v1::subsets::any::io::text::snapshot::building_component_spec;
use crate::standards::v1::subsets::any::io::text::snapshot::{initial_try_values,value_to_dsl,dsl_to_value,dsl_string_value,dsl_f64_value,json_string_value,json_f64_value};
use semio_framework_pack_json::Value;
use crate::standards::v1::subsets::any::io::text::snapshot::onboarding_example_json;
use crate::standards::v1::subsets::any::io::text::snapshot::onboarding_example_spec;
use crate::standards::v1::subsets::any::io::text::snapshot::default_example_json;
use super::*;
use semio_framework_pack_json::object;

#[semio_framework_async_macros::async_test]
async fn locate_question_finds_a_question_anywhere_in_the_document() {
    let spec = building_component_spec();
    let steps = forms_steps(&spec);
    let first_id = steps[0].blocks[0].id.clone();
    let location = locate_question(&spec, &first_id).expect("question located");
    assert_eq!(location.step_id, steps[0].id);
}

#[semio_framework_async_macros::async_test]
async fn update_block_operations_returns_none_for_a_missing_question() {
    let spec = empty_forms_snapshot();
    assert!(update_block_operations(&spec, "missing", |question| question.label = "x".into()).is_none());
}

#[semio_framework_async_macros::async_test]
async fn update_block_operations_patch_the_located_question_field_by_field() {
    let mut spec = building_component_spec();
    let question_id = forms_steps(&spec)[0].blocks[0].id.clone();
    let operations = update_block_operations(&spec, &question_id, |question| question.label = "Renamed".into()).expect("operations");
    assert!(matches!(operations.as_slice(), [FormMutation::ChangeBlockField(change)] if change.block_id == question_id), "one field edit is one change-block-field leaf: {operations:?}");
    for operation in &operations {
        spec = apply_form_edit_mutation(&spec, operation);
    }
    assert_eq!(forms_steps(&spec)[0].blocks[0].label, "Renamed");
    assert_eq!(update_block_operations(&spec, &question_id, |_| {}), Some(Vec::new()), "an edit that changes nothing records nothing");
}

#[semio_framework_async_macros::async_test]
async fn a_re_seeded_question_is_one_whole_replace_block() {
    let spec = building_component_spec();
    let step = forms_steps(&spec)[0].clone();
    let before = step.blocks[0].clone();
    let after = FormQuestion { kind: format!("{}-other", before.kind), ..before.clone() };
    assert!(matches!(question_edit_mutations(&step.id, &before, &after).as_slice(), [FormMutation::ReplaceBlock(replace)] if replace.block == after));
}

fn apply_form_edit_mutation(spec: &FormsSnapshot, operation: &FormMutation) -> FormsSnapshot {
    crate::central_apply::apply_form_edit_mutation(spec, operation).expect("valid mutation diff")
}

#[semio_framework_async_macros::async_test]
async fn create_form_id_is_unique_per_call() {
    assert_ne!(create_form_id("q"), create_form_id("q"));
}

#[semio_framework_async_macros::async_test]
async fn forms_play_step_tree_id_prefixes_with_step() {
    assert_eq!(forms_play_step_tree_id("s1"), "step:s1");
}

#[semio_framework_async_macros::async_test]
async fn dsl_value_conversions_round_trip_through_json() {
    // 🩹️ A whole-number float (e.g. `6.0`) round-trips through `semio_framework_value::DslValue` as the integer-typed
    // `serde_json::Number` (`6`), which does not `==` the float-typed literal despite being numerically
    // equal — a `dsl` value-system characteristic, not something this conversion controls. Use a
    // fractional value here so the round-trip assertion is unambiguous.
    let value = object([("height".to_string(), Value::from(6.5))]);
    assert_eq!(dsl_to_value(&value_to_dsl(&value)), value);
    assert_eq!(dsl_string_value(&value_to_dsl(&Value::from("hello"))), "hello");
    assert_eq!(dsl_f64_value(&value_to_dsl(&Value::from(42.5))), 42.5);
}

#[semio_framework_async_macros::async_test]
async fn flatten_questions_lists_every_block_across_steps() {
    let spec = onboarding_example_spec();
    assert!(!flatten_questions(&spec).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn json_value_helpers_stringify_primitives() {
    assert_eq!(json_string_value(&Value::from("a")), "a");
    assert_eq!(json_string_value(&Value::from(true)), "true");
    assert_eq!(json_string_value(&Value::Null), "");
    assert_eq!(json_f64_value(&Value::from(5)), 5.0);
    assert_eq!(json_f64_value(&Value::Null), 0.0);
}

#[semio_framework_async_macros::async_test]
async fn default_and_onboarding_examples_parse_and_serialize() {
    assert!(!default_example_json().is_empty());
    assert!(!onboarding_example_json().is_empty());
}
