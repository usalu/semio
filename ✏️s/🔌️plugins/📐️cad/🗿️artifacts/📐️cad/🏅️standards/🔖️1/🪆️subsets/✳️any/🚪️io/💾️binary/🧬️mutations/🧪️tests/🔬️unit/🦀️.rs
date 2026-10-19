extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
use semio_s_artifact_cad_cad_host::new_cad_store;
use semio_s_artifact_cad_cad_mutations::standards::v1::subsets::any::io::binary::mutations::*;
use semio_s_artifact_cad_cad_mutations::CadMutation;
use protocol::OpBinary;
use semio_s_artifact_cad_cad_mut_slots::create_shape_model::CreateShapeModel;
use semio_s_artifact_cad_cad_model::{snapshot::empty_cad_snapshot, snapshot::CAD_DOCUMENT_SCHEMA};
use semio_s_artifact_cad_cad_testing::sample_model_child;
use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactCommand};

#[semio_framework_async_macros::async_test]
async fn encode_decode_op_round_trips_a_representative_operation() {
    let sample = sample_model_child("op-round-trip-1");
    let operation = CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.clone() });
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn cad_projection_defaults() {
    let store = new_cad_store(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("store");
    assert_eq!(store.snapshot().expect("projection").id, "cad");
}

#[semio_framework_async_macros::async_test]
async fn create_shape_model_round_trips_through_store() {
    let mut store = new_cad_store(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("store");
    let sample = sample_model_child("store-round-trip-1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.clone() })], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    let scene = store.snapshot().expect("projection");
    assert_eq!(scene.shape_model.expect("shape_model set").child_id, sample.child_id);
}
