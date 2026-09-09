use super::*;

#[semio_framework_async_macros::async_test]
async fn the_terrain_snapshot_defaults_to_a_flat_unimported_terrain() {
    let document = GisTerrainSnapshot::default();
    assert_eq!(document.exaggeration, 0.0);
    assert!(document.imported_features_json.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_terrain_snapshot_composes_a_content_addressed_mesh_child() {
    let document = GisTerrainSnapshot::default();
    assert!(document.mesh.is_some(), "the terrain always composes a mesh child, even when flat/unimported");
    let mesh = gis_terrain_mesh_from_snapshot(&document);
    assert_eq!(mesh.meshes.len(), 1);
    assert_eq!(mesh.meshes[0].primitives[0].positions.len(), 4, "a flat placeholder quad");
}
