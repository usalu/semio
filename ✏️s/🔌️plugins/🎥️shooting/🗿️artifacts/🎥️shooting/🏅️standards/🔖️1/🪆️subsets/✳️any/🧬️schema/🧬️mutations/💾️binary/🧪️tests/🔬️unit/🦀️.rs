
use super::*;
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

    let mut store = store::ArtifactStore::<ShootingSnapshot, ShootingMutation>::new(store::create_document_envelope(crate::SHOOTING_DOCUMENT_SCHEMA, "shooting", crate::empty_shooting_snapshot(), None)).await.expect("valid artifact store fixture");
    let asset = crate::ShootingAsset { id: "a1".into(), name: "Asset".into(), url: "/mesh/a1.glb".into(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None };
    let create = crate::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset { asset, index: Some(0) };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![ShootingMutation::CreateAsset(create)], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
