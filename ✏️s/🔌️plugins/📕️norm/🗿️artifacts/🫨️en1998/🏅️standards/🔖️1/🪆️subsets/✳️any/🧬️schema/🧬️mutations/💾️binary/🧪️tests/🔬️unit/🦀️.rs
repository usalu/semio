use super::*;
use crate::mutations::change_seismic_zone;
use crate::En1998Snapshot;

fn sample_mutation() -> En1998Mutation {
    En1998Mutation::ChangeSeismicZone(change_seismic_zone::ChangeSeismicZone { new_seismic_zone: 3 })
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
    let envelope = store::create_document_envelope("norm.en1998/v1", "en1998", En1998Snapshot::default(), None);
    // 🏪️ A bare `ArtifactStore::new` installs NO document-store owners: the first `Apply` is refused
    // with `edit history insertion requires its exact mutation retirement factory`, and a store that
    // survived would then trip the terminal-empty shallow-shell witness in `Drop`.
    let mut store = store::os_store::test_support::plain_test_store(envelope).await;
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![sample_mutation()], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    store::os_store::test_support::close_plain_test_store(&mut store);
}
