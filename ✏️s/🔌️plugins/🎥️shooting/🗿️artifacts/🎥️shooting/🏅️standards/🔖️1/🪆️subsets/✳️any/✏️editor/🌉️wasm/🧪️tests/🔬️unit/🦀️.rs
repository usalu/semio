
use super::*;

#[semio_framework_async_macros::async_test]
async fn shooting_store_type_alias_constructs_from_an_empty_envelope() {
    let store = ShootingStore::new(store::create_document_envelope(crate::SHOOTING_DOCUMENT_SCHEMA, "shooting", crate::empty_shooting_snapshot(), None)).await.expect("valid artifact store fixture");
    assert!(store.snapshot().expect("snapshot").assets.is_empty());
}
