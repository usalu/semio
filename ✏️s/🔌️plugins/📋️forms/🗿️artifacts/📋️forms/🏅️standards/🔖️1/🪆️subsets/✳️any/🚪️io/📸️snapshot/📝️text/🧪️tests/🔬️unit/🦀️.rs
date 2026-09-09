use super::*;
use crate::{forms_children_from_steps, forms_steps, FormStep, FORMS_DOCUMENT_SCHEMA};
use store::os_store::test_support::assert_dsl_round_trip;

#[semio_framework_async_macros::async_test]
async fn snapshot_dsl_round_trips_with_composed_children() {
    let steps = vec![FormStep { id: "s1".into(), title: "Step".into(), description: None, blocks: Vec::new() }];
    let (structure, results) = forms_children_from_steps(&steps);
    let snapshot = FormsSnapshot { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: Some("T".into()), structure, results };
    let printed = store::ArtifactDsl::print_dsl(&snapshot);
    let parsed = <FormsSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("parses");
    assert_eq!(parsed, snapshot);
}

/// 🩹️ Each test builds `spec` via [`parse_playbook_example_dsl`] (real content, cache-warm in
/// THIS call) rather than [`parse_dsl`] on the raw example text directly — `FormsSnapshot`'s
/// own persisted codec is independently proven correct by this facet's own
/// `snapshot_dsl_round_trips_with_composed_children`; what these three prove is that the
/// example fixtures parse as real playbook content AND that `assert_dsl_round_trip` (which
/// exercises `FormsSnapshot::print_dsl`/`parse_dsl` on the resulting cache-warm snapshot) holds
/// for them too.
#[semio_framework_async_macros::async_test]
async fn building_component_fixture_dsl_round_trips() {
    let spec = parse_playbook_example_dsl(BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋️building-component.forms parses");
    assert_eq!(spec.id, "building-component");
    assert_eq!(forms_steps(&spec).len(), 2);
    assert_dsl_round_trip(&spec);
}

#[semio_framework_async_macros::async_test]
async fn default_fixture_dsl_round_trips() {
    let spec = parse_playbook_example_dsl(DEFAULT_EXAMPLE_TEXT).expect("📋️default.forms parses");
    assert_eq!(spec.id, "default");
    assert_eq!(forms_steps(&spec).len(), 1);
    assert_dsl_round_trip(&spec);
}

#[semio_framework_async_macros::async_test]
async fn onboarding_fixture_dsl_round_trips() {
    let spec = parse_playbook_example_dsl(ONBOARDING_EXAMPLE_TEXT).expect("📋️onboarding.forms parses");
    assert_eq!(spec.id, "onboarding");
    assert_eq!(forms_steps(&spec).len(), 3);
    assert_dsl_round_trip(&spec);
}
