use super::*;
use crate::mutations::change_net_floor_area_m2;
use crate::Din18599Snapshot;

fn sample_mutation() -> Din18599Mutation {
    Din18599Mutation::ChangeNetFloorAreaM2(change_net_floor_area_m2::ChangeNetFloorAreaM2 {
        new_net_floor_area_m2: 120.0,
    })
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
    let envelope = store::create_document_envelope("norm.din18599/v1", "din18599", Din18599Snapshot::default(), None);
    let mut store = store::os_store::test_support::plain_test_store(envelope).await;
    store
        .dispatch(store::ArtifactCommand::Apply {
            mutations: vec![sample_mutation()],
            description: None,
        })
        .await
        .expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    store::os_store::test_support::close_plain_test_store(&mut store);
}
