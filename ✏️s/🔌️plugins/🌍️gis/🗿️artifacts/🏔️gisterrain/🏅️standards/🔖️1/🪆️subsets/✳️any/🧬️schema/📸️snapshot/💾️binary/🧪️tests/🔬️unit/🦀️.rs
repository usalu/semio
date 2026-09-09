use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::text as dsl;

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_pack_agrees_with_dsl_for_bundled_reuse_example() {
    let document = dsl::parse_dsl(dsl::REUSE_TERRAIN_EXAMPLE_TEXT).expect("parse reuse-terrain example");
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    assert_eq!(decode(&encode(&document)).expect("decode"), document);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_pack_agrees_with_dsl_for_arbitrary_exaggeration() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&GisTerrainSnapshot { exaggeration: 2.75, imported_features_json: String::new(), ..Default::default() });
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_pack_agrees_with_dsl_for_imported_features_json() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: r#"{"positions":[{"id":"p1","lon":1.0,"lat":2.0}],"routes":[],"regions":[]}"#.into(), ..Default::default() });
}
