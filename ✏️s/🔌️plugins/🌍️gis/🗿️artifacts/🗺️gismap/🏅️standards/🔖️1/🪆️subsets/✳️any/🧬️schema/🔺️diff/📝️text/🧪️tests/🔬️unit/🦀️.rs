use super::*;

#[semio_framework_async_macros::async_test]
async fn repeated_feature_patches_match_the_neutral_serde_oracle() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("patch composition fixture");
    for case in cases.as_array().expect("fixture cases") {
        let id = case["id"].as_str().expect("case identity");
        let base = GisMapSnapshot { regions: vec![MapFeature { id: id.into(), data: dsl::DslValue::from(&case["initial"]) }], ..Default::default() };
        let mut diff = GisMapDiff::default();
        let mut oracle = case["initial"].clone();
        for patch in case["patches"].as_array().expect("patch sequence") {
            if let Some(data) = patch.get("data") {
                oracle = data.clone();
            }
            diff.absorb(GisMapDiff {
                regions: Some(GisMapFeaturesDelta { patched: vec![GisMapFeaturePatchEntry { id: id.into(), patch: crate::MapFeaturePatch { data: patch.get("data").map(dsl::DslValue::from) } }], ..Default::default() }),
                ..Default::default()
            });
        }
        assert_eq!(oracle, case["expected"], "{id}: independent replacement oracle");
        let snapshot = diff.apply(&base).expect("composed patch applies");
        assert_eq!(snapshot.regions[0].data, dsl::DslValue::from(&oracle), "{id}: composed payload");
        assert_eq!(diff.regions.expect("region delta").patched.len(), 1, "{id}: one patch per feature");
    }
}

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
