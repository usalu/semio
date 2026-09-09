use super::*;
use crate::mutations::change_m_ed_knm;
use crate::En1996Snapshot;

fn sample_mutation() -> En1996Mutation {
    En1996Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 12.5 })
}

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let mutation = sample_mutation();
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    let bytes = encode_op(&mutation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), mutation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_through_store() {
    let envelope = store::create_document_envelope("norm.en1996/v1", "en1996", En1996Snapshot::default(), None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![sample_mutation()], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
