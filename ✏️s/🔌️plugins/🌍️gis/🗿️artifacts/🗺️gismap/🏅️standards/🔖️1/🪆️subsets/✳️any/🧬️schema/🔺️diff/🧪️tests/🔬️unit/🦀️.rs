use super::*;

#[semio_framework_async_macros::async_test]
async fn repeated_feature_patches_match_the_neutral_serde_oracle() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../🚪️io/📝️text/🔺️diff/🧫️fixtures/🔬️unit/🔣️.json")).expect("patch composition fixture");
    for case in cases.as_array().expect("fixture cases") {
        let id = case["id"].as_str().expect("case identity");
        let base = GisMapSnapshot { regions: vec![MapFeature { id: id.into(), data: semio_framework_value::DslValue::from(&case["initial"]) }], ..Default::default() };
        let mut diff = GisMapDiff::default();
        let mut oracle = case["initial"].clone();
        for patch in case["patches"].as_array().expect("patch sequence") {
            if let Some(data) = patch.get("data") {
                oracle = data.clone();
            }
            diff.absorb(GisMapDiff {
                regions: Some(GisMapFeaturesDelta { patched: vec![GisMapFeaturePatchEntry { id: id.into(), patch: crate::MapFeaturePatch { data: patch.get("data").map(semio_framework_value::DslValue::from) } }], ..Default::default() }),
                ..Default::default()
            });
        }
        assert_eq!(oracle, case["expected"], "{id}: independent replacement oracle");
        let snapshot = protocol::apply_diff(&diff, &base).expect("composed patch applies");
        assert_eq!(snapshot.regions[0].data, semio_framework_value::DslValue::from(&oracle), "{id}: composed payload");
        assert_eq!(diff.regions.expect("region delta").patched.len(), 1, "{id}: one patch per feature");
    }
}

fn feature(id: &str) -> MapFeature {
    MapFeature { id: id.into(), data: semio_framework_value::DslValue::String(id.into()) }
}

#[semio_framework_async_macros::async_test]
async fn collection_diffs_absorb_and_apply_add_remove_patch() {
    let base = GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() };
    let mut diff = GisMapDiff { positions: Some(GisMapFeaturesDelta { removed: vec!["p1".into()], ..Default::default() }), ..Default::default() };
    diff.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta { added: vec![feature("p2")], ..Default::default() }), ..Default::default() });
    let next = protocol::apply_diff(&diff, &base).expect("valid mutation diff");
    assert_eq!(next.positions.len(), 1);
    assert_eq!(next.positions[0].id, "p2");
}

/// ➕️ create∘delete of one feature cancels, patch∘create folds into the added row, delete∘create keeps the replacement.
#[semio_framework_async_macros::async_test]
async fn absorb_coalesces_same_key_rows() {
    let created = GisMapDiff { positions: Some(GisMapFeaturesDelta { added: vec![feature("p2"), feature("p3")], ..Default::default() }), ..Default::default() };
    let mut sum = created.clone();
    sum.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta { patched: vec![GisMapFeaturePatchEntry { id: "p2".into(), patch: crate::MapFeaturePatch { data: Some(semio_framework_value::DslValue::String("renamed".into())) } }], ..Default::default() }), ..Default::default() });
    assert_eq!(sum.positions.as_ref().expect("positions").added[0].data, semio_framework_value::DslValue::String("renamed".into()));
    sum.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta { removed: vec!["p2".into()], ..Default::default() }), ..Default::default() });
    assert_eq!(sum.positions.as_ref().expect("positions").added, vec![feature("p3")]);
    let mut cancelled = created;
    cancelled.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta { removed: vec!["p2".into(), "p3".into()], ..Default::default() }), ..Default::default() });
    assert!(cancelled.positions.is_none(), "create∘delete leaves nothing");
    let base = GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() };
    let mut replaced = GisMapDiff { positions: Some(GisMapFeaturesDelta { removed: vec!["p1".into()], ..Default::default() }), ..Default::default() };
    replaced.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta { added: vec![MapFeature { id: "p1".into(), data: semio_framework_value::DslValue::String("again".into()) }], ..Default::default() }), ..Default::default() });
    assert_eq!(protocol::apply_diff(&replaced, &base).expect("replacement applies").positions[0].data, semio_framework_value::DslValue::String("again".into()));
}

/// ↩️ The negative diff restores a removed feature at its base position and `between` is the state delta.
#[semio_framework_async_macros::async_test]
async fn inverse_and_between_restore_the_base() {
    let base = GisMapSnapshot { positions: vec![feature("p1"), feature("p2"), feature("p3")], ..Default::default() };
    let removed = GisMapDiff { positions: Some(GisMapFeaturesDelta { removed: vec!["p2".into()], ..Default::default() }), ..Default::default() };
    let after = protocol::apply_diff(&removed, &base).expect("delete applies");
    assert_eq!(protocol::apply_diff(&protocol::DiffAlgebra::inverse(&removed, &base), &after).expect("negative diff applies"), base);
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<GisMapSnapshot, GisMapDiff>(&base, &after).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<GisMapSnapshot, GisMapDiff>(&after, &base).await;
}

/// ⚖️ Ordered-collection law on a MIDDLE row: deleting, moving or inserting in the middle inverts at the original index.
#[semio_framework_async_macros::async_test]
async fn deleting_or_moving_a_middle_feature_inverts_at_its_original_index() {
    use crate::mutations::{create_position::CreatePosition, delete_position::DeletePosition, reorder_positions::ReorderPositions, GisMapMutation};
    let base = GisMapSnapshot { positions: vec![feature("p1"), feature("p2"), feature("p3")], ..Default::default() };
    for mutation in [
        GisMapMutation::DeletePosition(DeletePosition { id: "p2".into() }),
        GisMapMutation::ReorderPositions(ReorderPositions { id: "p2".into(), to_index: 0 }),
        GisMapMutation::CreatePosition(CreatePosition { index: 1, item: feature("p9") }),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
