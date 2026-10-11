use super::*;

#[semio_framework_async_macros::async_test]
async fn shooting_store_type_alias_constructs_from_an_empty_envelope() {
    let mut store = ShootingStore::new(store::create_document_envelope(crate::SHOOTING_DOCUMENT_SCHEMA, "shooting", crate::empty_shooting_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(store::funded_bounded_artifact_store_owners::<ShootingSnapshot, ShootingMutation>().expect("funded document owners")).unwrap_or_else(|(error, _owners)| panic!("{}", error.into_message()));
    assert!(store.snapshot().expect("snapshot").assets.is_empty());
    while !store.close_owned_terminal_is_empty() {
        let demand = store.close_owned_demands(4_096).expect("the document store quotes its close");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(4_096), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        store.close_owned_step(grant).expect("shooting document store closes through its exact bounded owners");
    }
}
