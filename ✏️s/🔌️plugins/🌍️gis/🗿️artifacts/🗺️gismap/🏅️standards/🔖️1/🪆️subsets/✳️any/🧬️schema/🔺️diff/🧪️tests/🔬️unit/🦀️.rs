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
                regions: Some(GisMapFeaturesDelta::modification(id, crate::MapFeaturePatch { data: patch.get("data").map(semio_framework_value::DslValue::from), ..Default::default() })),
                ..Default::default()
            });
        }
        assert_eq!(oracle, case["expected"], "{id}: independent replacement oracle");
        let snapshot = protocol::apply_diff(&diff, &base).expect("composed patch applies");
        assert_eq!(snapshot.regions[0].data, semio_framework_value::DslValue::from(&oracle), "{id}: composed payload");
        assert_eq!(diff.regions.expect("region delta").modified.len(), 1, "{id}: one patch per feature");
    }
}

fn feature(id: &str) -> MapFeature {
    MapFeature { id: id.into(), data: semio_framework_value::DslValue::String(id.into()) }
}

#[semio_framework_async_macros::async_test]
async fn collection_diffs_absorb_and_apply_add_remove_patch() {
    let base = GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() };
    let mut diff = GisMapDiff { positions: Some(GisMapFeaturesDelta::removal(&base.positions, 0)), ..Default::default() };
    diff.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta::insertion(0, feature("p2"))), ..Default::default() });
    let next = protocol::apply_diff(&diff, &base).expect("valid mutation diff");
    assert_eq!(next.positions.len(), 1);
    assert_eq!(next.positions[0].id, "p2");
}

fn patch_to(id: &str, data: &str) -> GisMapFeaturesDelta {
    GisMapFeaturesDelta::modification(id, crate::MapFeaturePatch { data: Some(semio_framework_value::DslValue::String(data.into())), ..Default::default() })
}

/// ➕️ insert∘remove of one feature cancels, insert∘patch stays an insertion plus its own modification, remove∘insert keeps the replacement.
#[semio_framework_async_macros::async_test]
async fn absorb_coalesces_same_key_rows() {
    let created = GisMapDiff { positions: Some(GisMapFeaturesDelta { inserted: vec![GisMapFeatureInsertion { index: 0, row: feature("p2") }, GisMapFeatureInsertion { index: 1, row: feature("p3") }], ..Default::default() }), ..Default::default() };
    let mut sum = created.clone();
    sum.absorb(GisMapDiff { positions: Some(patch_to("p2", "renamed")), ..Default::default() });
    assert_eq!(sum.positions.as_ref().expect("positions").inserted[0].row, feature("p2"), "the framework keeps a patch of an inserted row as its own modified entry");
    assert_eq!(sum.positions.as_ref().expect("positions").modified, vec![GisMapFeatureModification { id: "p2".into(), patch: crate::MapFeaturePatch { data: Some(semio_framework_value::DslValue::String("renamed".into())), ..Default::default() } }]);
    assert_eq!(protocol::apply_diff(&sum, &GisMapSnapshot::default()).expect("composed diff applies").positions[0].data, semio_framework_value::DslValue::String("renamed".into()), "inserted then patched applies in order");
    sum.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta { removed: vec![GisMapFeatureRemoval { id: "p2".into(), index: 0 }], ..Default::default() }), ..Default::default() });
    assert_eq!(sum.positions.as_ref().expect("positions").inserted.iter().map(|insertion| insertion.row.clone()).collect::<Vec<_>>(), vec![feature("p3")]);
    assert_eq!(sum.positions.as_ref().expect("positions").inserted[0].index, 0, "the survivor slides to the slot the cancelled row freed");
    let mut cancelled = created;
    cancelled.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta { removed: vec![GisMapFeatureRemoval { id: "p2".into(), index: 0 }, GisMapFeatureRemoval { id: "p3".into(), index: 1 }], ..Default::default() }), ..Default::default() });
    assert!(cancelled.positions.is_none(), "insert∘remove leaves nothing");
    let base = GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() };
    let mut replaced = GisMapDiff { positions: Some(GisMapFeaturesDelta::removal(&base.positions, 0)), ..Default::default() };
    replaced.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta::insertion(0, MapFeature { id: "p1".into(), data: semio_framework_value::DslValue::String("again".into()) })), ..Default::default() });
    assert_eq!(protocol::apply_diff(&replaced, &base).expect("replacement applies").positions[0].data, semio_framework_value::DslValue::String("again".into()));
}

/// ↩️ The negative diff restores a removed feature at its base position.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_base() {
    let base = GisMapSnapshot { positions: vec![feature("p1"), feature("p2"), feature("p3")], ..Default::default() };
    let removed = GisMapDiff { positions: Some(GisMapFeaturesDelta::removal(&base.positions, 1)), ..Default::default() };
    let after = protocol::apply_diff(&removed, &base).expect("delete applies");
    assert_eq!(protocol::apply_diff(&protocol::DiffAlgebra::inverse(&removed, &base), &after).expect("negative diff applies"), base);
}

/// ↕️ A move is one positional row; it inverts to the opposite row and composes with a following removal at the base index.
#[semio_framework_async_macros::async_test]
async fn moves_are_positional_rows_that_invert_and_compose() {
    let base = GisMapSnapshot { positions: vec![feature("p1"), feature("p2"), feature("p3")], ..Default::default() };
    let moved = GisMapDiff { positions: Some(GisMapFeaturesDelta::relocation(&base.positions, 0, 2)), ..Default::default() };
    let after = protocol::apply_diff(&moved, &base).expect("move applies");
    assert_eq!(after.positions.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), ["p2", "p3", "p1"]);
    assert_eq!(protocol::apply_diff(&protocol::DiffAlgebra::inverse(&moved, &base), &after).expect("negative diff applies"), base);
    let mut sum = moved.clone();
    sum.absorb(GisMapDiff { positions: Some(GisMapFeaturesDelta::removal(&after.positions, 2)), ..Default::default() });
    let removed = protocol::apply_diff(&sum, &base).expect("composed diff applies");
    assert_eq!(removed.positions.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), ["p2", "p3"], "move∘remove removes at the base index");
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

fn object_feature(id: &str, entries: &[(&str, &str)]) -> MapFeature {
    MapFeature { id: id.into(), data: semio_framework_value::DslValue::Object(entries.iter().map(|(key, value)| ((*key).to_string(), semio_framework_value::DslValue::String((*value).to_string()))).collect()) }
}

fn property_diff(id: &str, edits: Vec<crate::schema::feature::MapFeaturePropertyEdit>) -> GisMapDiff {
    GisMapDiff { positions: Some(GisMapFeaturesDelta::modification(id, crate::MapFeaturePatch { properties: edits, ..Default::default() })), ..Default::default() }
}

fn set_edit(key: &str, value: &str, before: Option<&str>) -> crate::schema::feature::MapFeaturePropertyEdit {
    crate::schema::feature::MapFeaturePropertyEdit { key: key.into(), before: before.map(str::to_owned), set: Some(crate::schema::feature::MapFeaturePropertyValue { value: semio_framework_value::DslValue::String(value.into()) }) }
}

fn remove_edit(key: &str) -> crate::schema::feature::MapFeaturePropertyEdit {
    crate::schema::feature::MapFeaturePropertyEdit { key: key.into(), before: None, set: None }
}

/// ✏️ Property edits apply in order, compose by concatenation, and their inverse restores values AND key order.
#[semio_framework_async_macros::async_test]
async fn property_patches_apply_absorb_and_invert_with_key_order() {
    let base = GisMapSnapshot { positions: vec![object_feature("p1", &[("a", "1"), ("b", "2"), ("c", "3")])], ..Default::default() };
    let removal = property_diff("p1", vec![remove_edit("b")]);
    let removed = protocol::apply_diff(&removal, &base).expect("removal applies");
    assert_eq!(removed.positions[0].data, object_feature("p1", &[("a", "1"), ("c", "3")]).data);
    let negative = protocol::DiffAlgebra::inverse(&removal, &base);
    let restored = protocol::apply_diff(&negative, &removed).expect("inverse applies");
    assert_eq!(restored, base, "removing the middle key and undoing it restores its position");
    let mut sum = removal.clone();
    sum.absorb(property_diff("p1", vec![set_edit("z", "9", Some("c")), set_edit("a", "one", None)]));
    let summed = protocol::apply_diff(&sum, &base).expect("composed edits apply");
    assert_eq!(summed.positions[0].data, object_feature("p1", &[("a", "one"), ("z", "9"), ("c", "3")]).data);
    let undone = protocol::apply_diff(&protocol::DiffAlgebra::inverse(&sum, &base), &summed).expect("composed inverse applies");
    assert_eq!(undone, base, "the composed inverse restores values and key order");
    let replacement = GisMapDiff { positions: Some(GisMapFeaturesDelta::modification("p1", crate::MapFeaturePatch { data: Some(semio_framework_value::DslValue::String("whole".into())), ..Default::default() })), ..Default::default() };
    let mut superseded = removal;
    superseded.absorb(replacement);
    assert!(superseded.positions.as_ref().expect("positions").modified[0].patch.properties.is_empty(), "a later whole replacement supersedes earlier property edits");
}
