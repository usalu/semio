use semio_framework_test_fixtures::fixture_text;

#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = fixture_text!("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn bundled_example_infers_a_surface_without_a_synthetic_child_reference() {
    let text = super::source().document();
    let snapshot = <crate::GisTerrainSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).expect("demo fixture parses");
    assert!(snapshot.mesh.is_none());
    let grid = snapshot.elevation_source.as_ref().expect("demo owns elevation source").grid();
    assert!(crate::schema::imported_map::is_bundled_terrain_grid(&grid));
    let overlays = snapshot.imported_map.as_ref().expect("demo owns overlays");
    assert_eq!((overlays.positions.len(), overlays.routes.len(), overlays.regions.len()), (2, 1, 1));
    let relief = crate::gis_terrain_mesh_from_snapshot(&snapshot).expect("Liège terrain surface");
    let positions = &relief.meshes[0].primitives[0].positions;
    assert!(positions.iter().any(|position| position.z != positions[0].z));
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = fixture_text!("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::GisTerrainSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::GisTerrainInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::GisTerrainInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::GisTerrainInference::infer(&crate::GisTerrainSnapshot::default()).expect("valid materialized inference fixture"), crate::standards::v1::subsets::any::schema::inferences::GisTerrainInference::default(),);
}
//#endregion 🧪️InferenceLaws
