use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::schema::mutations::change_exaggeration::ChangeExaggeration;
use crate::schema::mutations::change_imported_features::ChangeImportedFeatures;
use crate::{GisTerrainSnapshot, GIS_3D_TERRAIN_SCHEMA};

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.0 });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_change_exaggeration_op_line_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 3.0 }));
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_change_imported_features_op_line_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: Some(crate::schema::imported_map::ImportedMap::from_json(r#"{"positions":[]}"#).unwrap()) }));
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_text_round_trips_through_store() {
    let initial = GisTerrainSnapshot { exaggeration: 1.0, imported_map: None, ..Default::default() };
    let envelope = store::os_store::document_materialize::create_document_envelope(GIS_3D_TERRAIN_SCHEMA, "gis3d-demo", initial, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(semio_framework_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners::<GisTerrainSnapshot, GisTerrainMutation>().expect("funded bounded document owners")).unwrap_or_else(|(error, _)| panic!("bounded document owners install refused: {error}"));
    store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.0 })], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    use semio_framework_plugin::ArtifactOwnedDisposer;
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<GisTerrainSnapshot, GisTerrainMutation>::new();
    let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    for _ in 0..100_000 {
        let demand = disposer.retirement_demands(&store, page).expect("the terrain document store quotes its next close turn");
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes.max(page), maximum_depth: demand.depth.max(1) };
        match disposer.close_step(&mut store, grant).expect("terrain document store close step") {
            semio_framework_plugin::PluginLifecycleStep::Progress(progress) => {
                assert!(progress.copied_items <= 1);
                assert!(progress.released_bytes <= grant.maximum_release_bytes);
            }
            semio_framework_plugin::PluginLifecycleStep::AwaitingInput { reason } | semio_framework_plugin::PluginLifecycleStep::Blocked { reason } => panic!("fresh terrain document store close unexpectedly blocked: {reason}"),
            semio_framework_plugin::PluginLifecycleStep::Complete(_) => break,
        }
    }
    assert!(disposer.terminal_is_empty(&store));
    drop(disposer);
    drop(store);
}
