use super::*;

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_dsl_round_trips_bundled_reuse_example() {
    let document = parse_dsl(REUSE_TERRAIN_EXAMPLE_TEXT).expect("parse reuse-terrain example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_dsl_round_trips_arbitrary_exaggeration() {
    store::os_store::test_support::assert_dsl_round_trip(&GisTerrainSnapshot { exaggeration: 2.75, imported_features_json: String::new(), ..Default::default() });
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_dsl_round_trips_imported_features_json() {
    store::os_store::test_support::assert_dsl_round_trip(&GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: r#"{"positions":[{"id":"p1","lon":1.0,"lat":2.0}],"routes":[],"regions":[]}"#.into(), ..Default::default() });
}

#[semio_framework_async_macros::async_test]
async fn print_dsl_reparses_to_the_same_document() {
    let document = parse_dsl(REUSE_TERRAIN_EXAMPLE_TEXT).expect("parse reuse-terrain example");
    assert_eq!(parse_dsl(&print_dsl(&document)).expect("reparse"), document);
}
