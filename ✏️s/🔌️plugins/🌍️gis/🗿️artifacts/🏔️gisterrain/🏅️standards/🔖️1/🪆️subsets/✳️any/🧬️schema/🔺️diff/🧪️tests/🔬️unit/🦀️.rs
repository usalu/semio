use super::*;

#[semio_framework_async_macros::async_test]
async fn field_diffs_absorb_last_writer_wins_and_apply_onto_the_snapshot() {
    let base = GisTerrainSnapshot { exaggeration: 1.0, imported_map: None, ..Default::default() };
    let mut diff = GisTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
    diff.absorb(GisTerrainDiff { exaggeration: Some(3.0), imported_map: Some(crate::schema::diff::ImportedMapChange{value:None}), ..Default::default() });
    let next = protocol::apply_diff(&diff, &base).expect("valid mutation diff");
    assert_eq!(next.exaggeration, 3.0);
    assert_eq!(next.imported_map, None);
}

