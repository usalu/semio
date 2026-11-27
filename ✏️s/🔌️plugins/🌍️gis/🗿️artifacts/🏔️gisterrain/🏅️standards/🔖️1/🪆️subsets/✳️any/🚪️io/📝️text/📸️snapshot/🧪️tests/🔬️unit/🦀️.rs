use crate::standards::v1::subsets::any::io::text::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_dsl_round_trips_bundled_reuse_example() {
    let document = default_terrain_document();
    semio_os_store_test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_dsl_round_trips_arbitrary_exaggeration() {
    semio_os_store_test_support::assert_dsl_round_trip(&GisTerrainSnapshot { exaggeration: 2.75, imported_map: None, ..Default::default() });
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_dsl_round_trips_imported_map() {
    semio_os_store_test_support::assert_dsl_round_trip(&GisTerrainSnapshot { exaggeration: 1.0, imported_map: Some(crate::schema::imported_map::ImportedMap::from_json(r#"{"positions":[{"id":"p1","lon":1.0,"lat":2.0}],"routes":[],"regions":[]}"#).unwrap()), ..Default::default() });
}

#[semio_framework_async_macros::async_test]
async fn print_dsl_reparses_to_the_same_document() {
    let document = default_terrain_document();
    assert_eq!(parse_dsl(&print_dsl(&document)).expect("reparse"), document);
}

#[test]
fn authored_polar_overlay_geometry_uses_wgs84_while_remote_tiles_keep_their_limit() {
    let mut document = default_terrain_document();
    document.elevation_source.as_mut().unwrap().bounds = [8.5, 88.0, 9.5, 90.0];
    document.imported_map = Some(crate::schema::imported_map::ImportedMap::from_json(r#"{
        "positions":[{"id":"pole","lon":9,"lat":90},{"id":"outside","lon":9,"lat":90.00000000000001},{"id":"low","lon":9,"lat":45}],
        "routes":[{"id":"polar-route","points":[[8.5,89],[9.5,89]]}],
        "regions":[{"id":"polar-region","ring":[[8.5,89],[9.5,89],[9,90]]}]
    }"#).unwrap());
    let authored = fixture_descriptor(&document);
    assert_eq!(authored.project_origin.lat, 89.0);
    assert_eq!(authored.overlays.positions.iter().map(|position| position.id.as_str()).collect::<Vec<_>>(), ["pole", "low"]);
    assert_eq!(authored.overlays.routes[0].coordinates, [[8.5, 89.0], [9.5, 89.0]]);
    assert_eq!(authored.overlays.regions[0].rings[0][2], [9.0, 90.0]);

    document.elevation_source = None;
    let tiled = fixture_descriptor(&document);
    assert_eq!(tiled.project_origin.lat, 45.0);
    assert_eq!(tiled.overlays.positions.len(), 1);
    assert_eq!(tiled.overlays.positions[0].id, "low");
    assert!(tiled.overlays.routes.is_empty() && tiled.overlays.regions.is_empty());
}
