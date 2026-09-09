use super::*;
use crate::op::VcsDemoMutation;
use crate::VCS_DOCUMENT_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn vcs_demo_projection_dsl_pack_equivalence() {
    let projection = crate::standards::v1::subsets::any::schema::empty_vcs_snapshot();
    store::os_store::test_support::assert_dsl_pack_equivalence(&projection);
    let bytes = encode(&projection);
    assert_eq!(decode(&bytes).expect("decode"), projection);
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `VcsDemoMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip law (same pattern as `mathematical`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

    let mut store: ArtifactStore<VcsSnapshot, VcsDemoMutation> =
        ArtifactStore::new(create_document_envelope(VCS_DOCUMENT_SCHEMA, "vcs-demo", crate::standards::v1::subsets::any::schema::empty_vcs_snapshot(), None)).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::mutations::rename_vcs("Renamed".into())], description: None }).await.expect("apply");
    let edit: &Edit<VcsDemoMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<VcsSnapshot, VcsDemoMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
