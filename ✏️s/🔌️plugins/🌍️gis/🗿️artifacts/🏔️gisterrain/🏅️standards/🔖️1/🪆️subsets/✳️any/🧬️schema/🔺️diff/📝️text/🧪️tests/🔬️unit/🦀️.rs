use super::*;

#[semio_framework_async_macros::async_test]
async fn field_diffs_absorb_last_writer_wins_and_apply_onto_the_snapshot() {
    let base = GisTerrainSnapshot { exaggeration: 1.0, imported_map: None, ..Default::default() };
    let mut diff = GisTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
    diff.absorb(GisTerrainDiff { exaggeration: Some(3.0), imported_map: Some(crate::schema::diff::ImportedMapChange{value:None}), ..Default::default() });
    let next = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(next.exaggeration, 3.0);
    assert_eq!(next.imported_map, None);
}

#[semio_framework_async_macros::async_test]
async fn a_whole_artifact_diff_wins_over_every_field_diff() {
    let base = GisTerrainSnapshot { exaggeration: 1.0, imported_map: None, ..Default::default() };
    let replacement_exaggeration = 9.0;
    let replacement_imported_map = Some(crate::schema::ImportedMap::default());
    let replacement = GisTerrainSnapshot {
        exaggeration: replacement_exaggeration,
        imported_map: replacement_imported_map.clone(),
        mesh: Some(crate::gis_terrain_mesh_child_handle(&crate::gis_terrain_mesh_content_key(replacement_exaggeration, replacement_imported_map.as_ref()))),
    };
    let mut diff = GisTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
    diff.absorb(diff_set_snapshot(&replacement));
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), replacement);
}
