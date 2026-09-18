//! 🗄️ The store lane: a real `ArtifactStore` over `Wfc3dSnapshot`/`Wfc3dMutation` opens on a bundled
//! example, applies an edit, round-trips through the document text and pack carriers, and is CLOSED
//! through its own bounded owners before it is dropped — the discipline every mounted artifact store
//! owes ("close every store before drop").

use crate::mutations::{change_seed, Wfc3dMutation};
use crate::Wfc3dSnapshot;

/// 🏪️ Opens the fixture store. A bare store carries no owner catalog and refuses its first edit, so
/// this installs the exact bounded owners production installs.
async fn open_store(document: Wfc3dSnapshot) -> store::ArtifactStore<Wfc3dSnapshot, Wfc3dMutation> {
    let envelope = store::create_document_envelope("s.wfc.wfc3d/v1", "wfc3d", document, None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid wfc3d artifact store fixture");
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<Wfc3dSnapshot, Wfc3dMutation>());
    store
}

async fn close_store(mut store: store::ArtifactStore<Wfc3dSnapshot, Wfc3dMutation>) {
    let mut steps = 0;
    while !store.close_owned_terminal_is_empty() {
        store.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("the wfc3d document store closes through its exact bounded owners");
        steps += 1;
        assert!(steps < 100_000, "the close ladder must terminate");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_store_opens_on_a_bundled_example_applies_an_edit_and_closes() {
    let document = crate::examples::two_room_corridor::snapshot();
    let mut store = open_store(document.clone()).await;
    assert_eq!(store.snapshot().expect("initial projection"), document);

    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![change_seed(99)], description: None }).await.expect("the seed edit applies");
    assert_eq!(store.snapshot().expect("edited projection").seed, 99);

    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    close_store(store).await;
}

/// 🧊️ The largest bundled example carries differently-sized slot boxes and two relations — the
/// document shape the preview and the solver both have to survive.
#[semio_framework_async_macros::async_test]
async fn the_tower_example_round_trips_through_a_live_store() {
    let document = crate::examples::tower_stack::snapshot();
    let store = open_store(document.clone()).await;
    assert_eq!(store.snapshot().expect("initial projection"), document);
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    close_store(store).await;
}
