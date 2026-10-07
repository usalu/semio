use crate::standards::v1::subsets::any::io::binary::snapshot::*;
use crate::sample_scene_fixture::sample_scene;

#[semio_framework_async_macros::async_test]
async fn cad_scene_round_trips_through_pack() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&sample_scene());
    let bytes = encode(&sample_scene());
    assert_eq!(decode(&bytes).expect("decode"), sample_scene());
}

#[semio_framework_async_macros::async_test]
async fn cad_pack_schema_identity_preserves_literal_children_and_rejects_foreign_domains() {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let mut scene = sample_scene();
    store::os_store::test_support::assert_pack_schema_identity(&scene);
    scene.drawings.push(store::ArtifactChild::new("drawing-1".into(), semio_framework_artifact_reference::ArtifactRef::parse_uri("other-id!s.stdio.semio@v1/drawing").expect("uri")));
    assert_eq!(decode(&encode(&scene)).unwrap(),scene);
    scene.drawings.last_mut().unwrap().target.dialect.subset="model".into();
    assert!(decode(&encode(&scene)).is_err(), "a drawing child from the model domain must not decode");
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `CadMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing dsl/pack round-trip laws (same pattern as `mathematical_pack`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    use crate::mutations::create_shape_model::CreateShapeModel;
    use crate::op::CadMutation;
    use crate::{empty_cad_snapshot, sample_scene_fixture::sample_model_child, CAD_DOCUMENT_SCHEMA};
    use crate::host::owned::new_cad_store;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand};

    let mut store = new_cad_store(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad-demo", empty_cad_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    let sample = sample_model_child("command-envelope-1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.clone() })], transaction: None }).await.expect("apply");
    let edit: &Edit<CadMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<CadSnapshot, CadMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
