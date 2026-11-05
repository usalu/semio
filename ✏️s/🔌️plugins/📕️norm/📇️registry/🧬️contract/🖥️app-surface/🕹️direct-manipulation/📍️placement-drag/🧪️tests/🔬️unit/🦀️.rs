use super::*;
use crate::{get_value_at_path, insert_value_at_path, set_value_at_path};
use semio_s_plugin_norm_structural_scene::{Affine, EntityKind, EntityRef, FieldBinding, GripStyle, HandleKind, Interval, LocalizedCopy, SnapRule};

fn object(entries: Vec<(&str, DslValue)>) -> DslValue {
    DslValue::object(entries.into_iter().map(|(k, v)| (k.to_string(), v)))
}

fn document(rows: Vec<DslValue>) -> DslValue {
    object(vec![("members", DslValue::Array(vec![object(vec![("id", DslValue::String("b1".into())), ("length", DslValue::float(4.0))])])), ("placements", DslValue::Array(rows))])
}

fn row(id: &str, x: f64, y: f64, z: f64, yaw: f64) -> DslValue {
    placement_row_value(&Placement { entity_id: id.into(), origin: Vec3::new(x, y, z), yaw })
}

fn grip(tag: &str, leaf: &str, base: f64) -> Handle {
    Handle {
        id: format!("b1:{tag}"),
        entity: EntityRef::new(EntityKind::Member, "b1", "members[id=b1]"),
        kind: HandleKind::Translate { axis: Vec3::X },
        anchor: Vec3::new(2.0, 3.0, 0.5),
        grip: GripStyle::Ring,
        writes: vec![FieldBinding::new(format!("placements[id=b1].{leaf}"), Affine { base, k: 1.0 }, "m")],
        snap: SnapRule { grid: Some(0.05), increment: None, targets: vec![], catalogue: vec![], tolerance: 0.0 },
        limits: Some(Interval::new(-1.0e3, 1.0e3)),
        accessible_name: LocalizedCopy::new("Move", "Verschieben"),
    }
}

fn grips() -> Vec<Handle> {
    vec![grip("move-x", "x", 2.0), grip("move-y", "y", 3.0)]
}

fn edits_of(tree: &DslValue, writes: &[FieldWrite]) -> Vec<NormEdit> {
    edits_with_yaw(tree, writes, None)
}

fn edits_with_yaw(tree: &DslValue, writes: &[FieldWrite], yaw: Option<f64>) -> Vec<NormEdit> {
    let handles = grips();
    plan_placed_fields(tree, writes, &|id| standing_pose(&handles, id, &|_| yaw)).expect("plans")
}

fn replay(tree: &mut DslValue, edits: &[NormEdit]) {
    for edit in edits {
        match edit {
            NormEdit::SetField { path, value } => set_value_at_path(tree, path, value.clone()).expect("set"),
            NormEdit::InsertItem { path, index, value } => insert_value_at_path(tree, path, *index, value.clone()).expect("insert"),
            NormEdit::RemoveItem { .. } | NormEdit::MoveItem { .. } => unreachable!("a drag never removes or reorders"),
        }
    }
}

#[test]
fn the_first_drag_of_an_entity_without_a_row_is_one_insert_with_the_standing_pose_and_reloads_to_it() {
    let mut tree = document(vec![]);
    let edits = edits_of(&tree, &[FieldWrite::number("placements[id=b1].x", 2.5)]);
    assert_eq!(edits, vec![NormEdit::InsertItem { path: "placements".into(), index: 0, value: Some(row("b1", 2.5, 3.0, 0.5, 0.0)) }]);
    replay(&mut tree, &edits);
    assert_eq!(placements_of(&tree), vec![Placement { entity_id: "b1".into(), origin: Vec3::new(2.5, 3.0, 0.5), yaw: 0.0 }]);
}

#[test]
fn a_drag_of_an_entity_with_a_row_sets_only_the_changed_leaves() {
    let mut tree = document(vec![row("b1", 2.5, 3.0, 0.5, 0.0)]);
    let edits = edits_of(&tree, &[FieldWrite::number("placements[id=b1].x", 2.5), FieldWrite::number("placements[id=b1].y", 4.0)]);
    assert_eq!(edits, vec![NormEdit::SetField { path: "placements[id=b1].y".into(), value: DslValue::float(4.0) }]);
    replay(&mut tree, &edits);
    assert_eq!(placements_of(&tree)[0].origin, Vec3::new(2.5, 4.0, 0.5));
}

#[test]
fn a_release_onto_the_present_pose_writes_nothing() {
    let tree = document(vec![row("b1", 2.5, 3.0, 0.5, 0.0)]);
    assert!(edits_of(&tree, &[FieldWrite::number("placements[id=b1].x", 2.5)]).is_empty());
}

#[test]
fn a_bundle_of_placement_and_field_writes_resolves_in_one_batch() {
    let tree = document(vec![]);
    let edits = edits_of(&tree, &[FieldWrite::number("members[id=b1].length", 5.0), FieldWrite::number("placements[id=b1].y", 3.5)]);
    assert!(matches!(edits.as_slice(), [NormEdit::InsertItem { .. }, NormEdit::SetField { path, .. }] if path == "members[id=b1].length"), "{edits:?}");
}

#[test]
fn the_preview_of_a_missing_row_is_complete_and_the_overlay_inserts_the_row_the_release_commits() {
    let tree = document(vec![]);
    let writes = complete_preview(&placements_of(&tree), &|id| standing_pose(&grips(), id, &|_| None), vec![FieldWrite::number("placements[id=b1].x", 2.5)]);
    assert_eq!(writes.len(), 4);
    let mut preview = tree.clone();
    overlay_writes(&mut preview, &writes).expect("overlay");
    let mut committed = tree.clone();
    replay(&mut committed, &edits_of(&tree, &[FieldWrite::number("placements[id=b1].x", 2.5)]));
    assert_eq!(placements_of(&preview), placements_of(&committed));
    assert!(get_value_at_path(&tree, "placements[id=b1].x").is_err(), "the committed document is untouched");
}

#[test]
fn a_partial_preview_of_a_missing_row_is_refused() {
    let mut tree = document(vec![]);
    assert!(overlay_writes(&mut tree, &[FieldWrite::number("placements[id=b1].x", 2.5)]).unwrap_err().contains("lacks"));
}

#[test]
fn the_first_drag_of_an_entity_without_a_row_keeps_the_yaw_its_derived_projection_stands_in() {
    let mut tree = document(vec![]);
    let edits = edits_with_yaw(&tree, &[FieldWrite::number("placements[id=b1].x", 2.5)], Some(0.75));
    assert_eq!(edits, vec![NormEdit::InsertItem { path: "placements".into(), index: 0, value: Some(row("b1", 2.5, 3.0, 0.5, 0.75)) }]);
    replay(&mut tree, &edits);
    assert_eq!(placements_of(&tree)[0].yaw, 0.75);
    let preview = complete_preview(&placements_of(&document(vec![])), &|id| standing_pose(&grips(), id, &|_| Some(0.75)), vec![FieldWrite::number("placements[id=b1].x", 2.5)]);
    assert!(preview.iter().any(|write| write.path == "placements[id=b1].yaw" && write.value.as_f64() == Some(0.75)), "the preview carries the derived yaw: {preview:?}");
}

#[test]
fn a_yaw_grip_overrides_the_derived_yaw_and_a_row_overrides_both() {
    let mut handles = grips();
    handles.push(grip("turn", "yaw", 1.25));
    assert_eq!(standing_pose(&handles, "b1", &|_| Some(0.75)).map(|pose| pose.yaw), Some(1.25));
    assert_eq!(standing_pose(&grips(), "b1", &|_| None).map(|pose| pose.yaw), Some(0.0));
    assert_eq!(standing_pose(&grips(), "other", &|_| Some(0.75)), None);
    let tree = document(vec![row("b1", 2.0, 3.0, 0.5, 0.25)]);
    assert!(edits_with_yaw(&tree, &[FieldWrite::number("placements[id=b1].x", 2.0)], Some(0.75)).is_empty(), "the persisted yaw wins over the derived one");
}
