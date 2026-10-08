use crate::host::owned::new_cad_store;
use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::mutations::create_shape_model::CreateShapeModel;
use crate::{empty_cad_snapshot, sample_scene_fixture::sample_model_child, CAD_DOCUMENT_SCHEMA};
use store::{create_document_envelope, ArtifactCommand};

#[semio_framework_async_macros::async_test]
async fn encode_decode_op_round_trips_a_representative_operation() {
    let sample = sample_model_child("op-round-trip-1");
    let operation = CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.clone() });
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn cad_projection_defaults() {
    let store = new_cad_store(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("store");
    assert_eq!(store.snapshot().expect("projection").id, "cad");
}

#[semio_framework_async_macros::async_test]
async fn create_shape_model_round_trips_through_store() {
    let mut store = new_cad_store(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("store");
    let sample = sample_model_child("store-round-trip-1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.clone() })], transaction: None }).await.expect("apply");
    let scene = store.snapshot().expect("projection");
    assert_eq!(scene.shape_model.expect("shape_model set").child_id, sample.child_id);
}
