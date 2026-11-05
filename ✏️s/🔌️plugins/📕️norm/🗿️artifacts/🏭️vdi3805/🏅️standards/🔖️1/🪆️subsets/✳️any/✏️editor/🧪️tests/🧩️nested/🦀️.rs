//! 🌡️ VDI descendant edits retain typed radiator configuration and recompute its real exponent check.

use semio_framework_value::DslValue;
use semio_s_artifact_norm_vdi3805_model::conforming_blatt_3_dataset;
use semio_s_artifact_norm_vdi3805_mutations::{resolve_edit, Vdi3805Mutation};
use semio_s_plugin_norm_app_surface::NormEdit;
use semio_s_plugin_norm_compliance::CheckStatus;

#[test]
fn a_nested_heat_exponent_edit_recomputes_the_real_radiator_check_without_mutating_the_original() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("business fixture");
    let number = |key: &str| law[key].as_f64().expect("fixture number");
    let document = conforming_blatt_3_dataset();
    let original = serde_json::to_value(&document).expect("independent original snapshot");
    let product = &document.catalog.products[0];
    assert_eq!(product.id, law["productId"].as_str().expect("product id"));
    assert_eq!(product.identity.article_number, product.id);
    let mut expected_configuration = serde_json::to_value(&product.configuration).expect("independent original configuration");
    assert_eq!(expected_configuration["attributes"]["kind"], "radiator");
    assert_eq!(expected_configuration["attributes"]["heatExponentN"], law["valueBefore"]);
    let before = semio_s_artifact_norm_vdi3805_inferences::evaluate(&document);
    let check_before = before.checks.iter().find(|check| check.id == law["checkId"].as_str().expect("check id")).expect("original exponent check");
    assert_eq!(check_before.computed.value, number("valueBefore"));
    assert!((check_before.utilization - number("numeratorBefore") / number("denominator")).abs() < 1e-12);
    assert_eq!(check_before.status, CheckStatus::Pass);
    let operations = resolve_edit(&document, &NormEdit::SetField {
        path: law["path"].as_str().expect("path").into(),
        value: DslValue::float(number("value")),
    }).expect("registered typed parent edit");
    let [Vdi3805Mutation::ChangeProductConfiguration(payload)] = operations.as_slice() else { panic!("one typed product-configuration mutation") };
    assert_eq!(payload.id, product.id);
    *expected_configuration.pointer_mut("/attributes/heatExponentN").expect("independent exponent pointer") = law["value"].clone();
    assert_eq!(serde_json::to_value(&payload.new_configuration).expect("typed configuration payload"), expected_configuration);
    assert_eq!(serde_json::to_value(&document).expect("original after resolution"), original);
    let (changed, messages) = crate::protocol::apply_mutation(&document, &operations[0]).expect("native typed mutation");
    assert!(messages.is_empty(), "typed mutation must be admitted without diagnostics: {messages:?}");
    assert_eq!(serde_json::to_value(&document).expect("original after application"), original);
    assert_eq!(changed.catalog.products[0].identity, product.identity);
    assert_eq!(serde_json::to_value(&changed.catalog.products[0].configuration).expect("applied typed configuration"), expected_configuration);
    let after = semio_s_artifact_norm_vdi3805_inferences::evaluate(&changed);
    let check_after = after.checks.iter().find(|check| check.id == law["checkId"].as_str().expect("check id")).expect("changed exponent check");
    assert_eq!(check_after.computed.value, number("value"));
    assert!((check_after.utilization - number("numeratorAfter") / number("denominator")).abs() < 1e-12);
    assert_eq!(check_after.status, CheckStatus::Pass);
}

#[test]
fn a_rendered_product_row_removes_its_owned_identity_without_mutating_the_original() {
    let law: serde_json::Value = serde_json::from_str(include_str!("remove.json")).expect("row removal fixture");
    let document = conforming_blatt_3_dataset();
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
