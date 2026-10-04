use super::*;
use crate::{forms_children_from_steps, forms_steps, FormStep, FORMS_DOCUMENT_SCHEMA};
use store::os_store::test_support::assert_dsl_round_trip;

#[semio_framework_async_macros::async_test]
async fn snapshot_dsl_round_trips_with_composed_children() {
    let steps = vec![FormStep { id: "s1".into(), title: "Step".into(), description: None, blocks: Vec::new() }];
    let (structure, results) = forms_children_from_steps(&steps);
    let snapshot = FormsSnapshot { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: Some("T".into()), definition: Default::default(), responses: Vec::new(), structure, results };
    let printed = store::ArtifactDsl::print_dsl(&snapshot);
    let parsed = <FormsSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("parses");
    assert_eq!(parsed, snapshot);
}

/// 📚️ Templates use exactly the same import path as saved form documents.
#[semio_framework_async_macros::async_test]
async fn building_component_fixture_dsl_round_trips() {
    let spec = parse_dsl(BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋️building-component.forms parses");
    assert_eq!(spec.id, "building-component");
    assert_eq!(forms_steps(&spec).len(), 2);
    assert_dsl_round_trip(&spec);
}

#[semio_framework_async_macros::async_test]
async fn default_fixture_dsl_round_trips() {
    let spec = parse_dsl(DEFAULT_EXAMPLE_TEXT).expect("📋️default.forms parses");
    assert_eq!(spec.id, "default");
    assert_eq!(forms_steps(&spec).len(), 1);
    assert_dsl_round_trip(&spec);
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../../../🖼️assets/📇️contact/🔣️.json")).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&spec)).unwrap();
    assert_eq!(actual, expected);
}

#[semio_framework_async_macros::async_test]
async fn onboarding_fixture_dsl_round_trips() {
    let spec = parse_dsl(ONBOARDING_EXAMPLE_TEXT).expect("📋️onboarding.forms parses");
    assert_eq!(spec.id, "onboarding");
    assert_eq!(forms_steps(&spec).len(), 3);
    assert_dsl_round_trip(&spec);
}
