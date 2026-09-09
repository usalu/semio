use super::*;
use crate::mutations::change_v_ed_kn;
use crate::En1997Snapshot;

fn sample_mutation() -> En1997Mutation {
    En1997Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 620.0 })
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
    let envelope = store::create_document_envelope("norm.en1997/v1", "en1997", En1997Snapshot::default(), None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![sample_mutation()], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
