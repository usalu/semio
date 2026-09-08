
use super::*;

#[semio_framework_async_macros::async_test]
async fn field_diffs_absorb_last_writer_wins_and_apply_onto_the_snapshot() {
    let base = GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: String::new(), ..Default::default() };
    let mut diff = GisTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
    diff.absorb(GisTerrainDiff { exaggeration: Some(3.0), imported_features_json: Some("null".into()), ..Default::default() });
    let next = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(next.exaggeration, 3.0);
    assert_eq!(next.imported_features_json, "null");
}

#[semio_framework_async_macros::async_test]
async fn a_whole_artifact_diff_wins_over_every_field_diff() {
    let base = GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: String::new(), ..Default::default() };
    let replacement_exaggeration = 9.0;
    let replacement_imported_features_json = "{}".to_string();
    let replacement = GisTerrainSnapshot {
        exaggeration: replacement_exaggeration,
        imported_features_json: replacement_imported_features_json.clone(),
        mesh: Some(crate::gis_terrain_mesh_child_handle(&crate::gis_terrain_mesh_content_key(replacement_exaggeration, &replacement_imported_features_json))),
    };
    let mut diff = GisTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
    diff.absorb(diff_set_snapshot(&replacement));
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), replacement);
}
