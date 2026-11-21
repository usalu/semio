use super::*;

#[semio_framework_async_macros::async_test]
async fn the_terrain_snapshot_defaults_to_an_unpopulated_terrain() {
    let document = GisTerrainSnapshot::default();
    assert_eq!(document.exaggeration, 0.0);
    assert!(document.imported_map.is_none());
    assert!(document.elevation_source.is_none());
    assert!(gis_terrain_mesh_from_snapshot(&document).is_err());
}

#[semio_framework_async_macros::async_test]
async fn terrain_mesh_exaggeration_changes_only_vertex_heights() {
    let document = crate::standards::v1::subsets::any::io::text::snapshot::default_terrain_document();
    assert!(document.mesh.is_none());
    let mesh = gis_terrain_mesh_from_snapshot(&document).expect("bundled Liège relief");
    assert_eq!(mesh.meshes.len(), 1);
    let primitive = &mesh.meshes[0].primitives[0];
    let doubled = gis_terrain_mesh_from_snapshot(&GisTerrainSnapshot { exaggeration: 3.0, ..document })
        .expect("exaggerated Liège relief");
    let doubled = &doubled.meshes[0].primitives[0];
    assert_eq!(primitive.indices, doubled.indices);
    for (position, raised) in primitive.positions.iter().zip(&doubled.positions) {
        assert_eq!([position.x, position.y, position.z * 2.0], [raised.x, raised.y, raised.z]);
    }
}
