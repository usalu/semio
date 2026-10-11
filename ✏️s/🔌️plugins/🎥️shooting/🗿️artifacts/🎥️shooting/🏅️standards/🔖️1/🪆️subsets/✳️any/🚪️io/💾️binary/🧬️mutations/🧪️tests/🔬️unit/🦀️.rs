use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::ShootingSnapshot;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = ShootingMutation::SetActiveShot(crate::standards::v1::subsets::any::schema::mutations::set_active_shot::SetActiveShot { shot_id: Some("s1".into()) });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn shooting_document_text_round_trips_store_with_applied_operation() {
    use store::ArtifactCommand;

    let mut store = store::ArtifactStore::<ShootingSnapshot, ShootingMutation>::new(store::create_document_envelope(crate::SHOOTING_DOCUMENT_SCHEMA, "shooting", crate::empty_shooting_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(store::funded_bounded_artifact_store_owners::<ShootingSnapshot, ShootingMutation>().expect("funded document owners")).unwrap_or_else(|(error, _owners)| panic!("{}", error.into_message()));
    let asset = crate::ShootingAsset { id: "a1".into(), name: "Asset".into(), url: "/mesh/a1.glb".into(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None };
    let create = crate::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset { asset, index: Some(0) };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![ShootingMutation::CreateAsset(create)], transaction: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    while !store.close_owned_terminal_is_empty() {
        let demand = store.close_owned_demands(4_096).expect("the document store quotes its close");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(4_096), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        store.close_owned_step(grant).expect("shooting document store closes through its exact bounded owners");
    }
}
