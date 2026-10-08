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

/// ↩️ The negative diff restores the base fields and `between` is the state delta (the owned mesh handle is preserved).
#[semio_framework_async_macros::async_test]
async fn inverse_and_between_restore_the_base() {
    let base = GisTerrainSnapshot { exaggeration: 1.0, imported_map: None, ..Default::default() };
    let diff = GisTerrainDiff { exaggeration: Some(4.0), imported_map: Some(crate::schema::diff::ImportedMapChange { value: Some(crate::schema::ImportedMap::default()) }) };
    let after = protocol::apply_diff(&diff, &base).expect("diff applies");
    assert_eq!(protocol::apply_diff(&protocol::DiffAlgebra::inverse(&diff, &base), &after).expect("negative diff applies"), base);
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<GisTerrainSnapshot, GisTerrainDiff>(&base, &after).await;
}
