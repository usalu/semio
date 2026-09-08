
use super::*;
use crate::document_dsl as dsl;
use crate::{FORMS_DOCUMENT_SCHEMA, FormStep, forms_children_from_steps};

#[semio_framework_async_macros::async_test]
async fn snapshot_pack_round_trips_with_composed_children() {
    let steps = vec![FormStep { id: "s1".into(), title: "Step".into(), description: None, blocks: Vec::new() }];
    let (structure, results) = forms_children_from_steps(&steps);
    let snapshot = FormsSnapshot { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: None, structure, results };
    let encoded = store::ArtifactPack::encode_pack(&snapshot);
    let decoded = <FormsSnapshot as store::ArtifactPack>::decode_pack(&encoded).expect("decodes");
    assert_eq!(decoded, snapshot);
}

#[semio_framework_async_macros::async_test]
async fn building_component_fixture_pack_agrees_with_dsl() {
    let spec = dsl::parse_playbook_example_dsl(dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋️building-component.forms parses");
    store::os_store::test_support::assert_dsl_pack_equivalence(&spec);
    let bytes = encode(&spec);
    assert_eq!(decode(&bytes).expect("decode"), spec);
}

#[semio_framework_async_macros::async_test]
async fn default_fixture_pack_agrees_with_dsl() {
    let spec = dsl::parse_playbook_example_dsl(dsl::DEFAULT_EXAMPLE_TEXT).expect("📋️default.forms parses");
    store::os_store::test_support::assert_dsl_pack_equivalence(&spec);
    let bytes = encode(&spec);
    assert_eq!(decode(&bytes).expect("decode"), spec);
}

#[semio_framework_async_macros::async_test]
async fn onboarding_fixture_pack_agrees_with_dsl() {
    let spec = dsl::parse_playbook_example_dsl(dsl::ONBOARDING_EXAMPLE_TEXT).expect("📋️onboarding.forms parses");
    store::os_store::test_support::assert_dsl_pack_equivalence(&spec);
    let bytes = encode(&spec);
    assert_eq!(decode(&bytes).expect("decode"), spec);
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `FormMutation`'s `CreateStep` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing dsl/pack round-trip laws (same pattern as `mathematical`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::{FORMS_DOCUMENT_SCHEMA, FormStep, op::FormMutation};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, ArtifactStore, create_document_envelope};

    let document = crate::forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, vec![FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() }]);
    let mut store: ArtifactStore<FormsSnapshot, FormMutation> = ArtifactStore::new(create_document_envelope(FORMS_DOCUMENT_SCHEMA, "forms-demo", document, None)).await.expect("valid artifact store fixture");
    let step = FormStep { id: "step-2".into(), title: "Review".into(), description: None, blocks: Vec::new() };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![FormMutation::CreateStep(crate::mutations::create_step::mutation::CreateStep { step, index: None })], description: None }).await.expect("apply");
    let edit: &Edit<FormMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<FormsSnapshot, FormMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
