
use super::*;
use crate::mutations::create_shape_model::CreateShapeModel;
use crate::{CAD_DOCUMENT_SCHEMA, empty_cad_snapshot, testkit::sample_model_child};
use store::{ArtifactCommand, create_document_envelope};

#[semio_framework_async_macros::async_test]
async fn encode_decode_op_round_trips_a_representative_operation() {
    let sample = sample_model_child("op-round-trip-1");
    let operation = CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() });
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn cad_projection_defaults() {
    let store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None)).await.expect("store");
    assert_eq!(store.snapshot().expect("projection").id, "cad");
}

#[semio_framework_async_macros::async_test]
async fn create_shape_model_round_trips_through_store() {
    let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None)).await.expect("store");
    let sample = sample_model_child("store-round-trip-1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() })], description: None }).await.expect("apply");
    let scene = store.snapshot().expect("projection");
    assert_eq!(scene.shape_model.expect("shape_model set").child_id, sample.child_id);
}
