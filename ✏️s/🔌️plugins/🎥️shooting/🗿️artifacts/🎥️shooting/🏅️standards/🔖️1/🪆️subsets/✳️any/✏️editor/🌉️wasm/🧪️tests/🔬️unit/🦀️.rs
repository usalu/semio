use super::*;

#[semio_framework_async_macros::async_test]
async fn shooting_store_type_alias_constructs_from_an_empty_envelope() {
    let mut store = ShootingStore::new(store::create_document_envelope(crate::SHOOTING_DOCUMENT_SCHEMA, "shooting", crate::empty_shooting_snapshot(), None)).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<ShootingSnapshot, ShootingMutation>());
    assert!(store.snapshot().expect("snapshot").assets.is_empty());
    while !store.close_owned_terminal_is_empty() {
        store.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("shooting document store closes through its exact bounded owners");
    }
}
