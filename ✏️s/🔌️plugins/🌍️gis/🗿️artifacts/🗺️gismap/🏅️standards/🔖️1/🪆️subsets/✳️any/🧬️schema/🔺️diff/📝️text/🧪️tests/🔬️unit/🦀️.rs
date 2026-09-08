
use super::*;

fn feature(id: &str) -> MapFeature {
    MapFeature { id: id.into(), data: dsl::DslValue::String(id.into()) }
}

#[semio_framework_async_macros::async_test]
async fn a_whole_artifact_diff_wins_over_every_collection_diff() {
    let base = GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() };
    let replacement = crate::gis_map_snapshot_with_derived_children(GisMapSnapshot { routes: vec![feature("r1")], ..Default::default() });
    let mut diff = GisMapDiff { positions: Some(GisMapFeaturesDelta { removed: vec!["p1".into()], ..Default::default() }), ..Default::default() };
    diff.absorb(diff_set_snapshot(&replacement));
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), replacement);
}

#[semio_framework_async_macros::async_test]
async fn collection_diffs_absorb_and_apply_add_remove_patch() {
    let base = GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() };
    let mut diff = GisMapDiff { positions: Some(GisMapFeaturesDelta { removed: vec!["p1".into()], ..Default::default() }), ..Default::default() };
    diff.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta { added: vec![feature("p2")], ..Default::default() }), ..Default::default() });
    let next = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(next.positions.len(), 1);
    assert_eq!(next.positions[0].id, "p2");
}
