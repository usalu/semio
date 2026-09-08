
use super::*;
use crate::schema::mutations::change_exaggeration::ChangeExaggeration;
use crate::schema::mutations::change_imported_features::ChangeImportedFeatures;
use crate::{GIS_3D_TERRAIN_SCHEMA, GisTerrainSnapshot};

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
    store::os_store::test_support::assert_op_line_round_trip(&GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: r#"{"positions":[]}"#.into() }));
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_text_round_trips_through_store() {
    let initial = GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: String::new(), ..Default::default() };
    let envelope = store::create_document_envelope(GIS_3D_TERRAIN_SCHEMA, "gis3d-demo", initial, None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.0 })], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
