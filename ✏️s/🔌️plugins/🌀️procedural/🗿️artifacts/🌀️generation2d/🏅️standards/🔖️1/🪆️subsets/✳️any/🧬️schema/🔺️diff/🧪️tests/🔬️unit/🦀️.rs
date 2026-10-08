use super::*;
use crate::standards::v1::subsets::any::schema::empty_generation2d_snapshot;
use crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshotRead;

fn camera_diff(oracle: &serde_json::Value, key: &str) -> (Generation2dDiff, CameraJson) {
    let camera: CameraJson = semio_framework_pack_json::from_json_str(&oracle[key].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    (Generation2dDiff { camera: Some(camera.clone()), ..Default::default() }, camera)
}

/// ➕️ Absorb keeps the incoming value of a scalar field, keeps the other rows, and the absorbed diff applies like its parts.
#[test]
fn diff_absorb_prefers_incoming_camera_and_preserves_other_rows() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧲️absorb/🔣️.json")).unwrap();
    let base = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    let (mut sum, _) = camera_diff(&oracle, "firstCamera");
    MutationDiff::absorb(&mut sum, Generation2dDiff { selected_generation: Some(Generation2dSelectionChange { id: None }), ..Default::default() });
    let (incoming_diff, incoming) = camera_diff(&oracle, "incomingCamera");
    MutationDiff::absorb(&mut sum, incoming_diff);
    let sum = Generation2dDiffRead::new(sum);
    assert_eq!(sum.camera.as_ref(), Some(&incoming));
    assert_eq!(sum.selected_generation, Some(Generation2dSelectionChange { id: None }));
    let next = Generation2dSnapshotRead::new(protocol::apply_diff(&*sum, &base).expect("absorbed diff applies"));
    assert_eq!(next.host_snapshot.camera, incoming);
    let camera: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&next.host_snapshot.camera)).unwrap();
    assert_eq!(camera, oracle["incomingCamera"]);
}

/// 🩹 Added rows insert at their index, patched rows replace in place, and the inverse restores the base.
#[test]
fn diff_apply_updates_widget_rows_and_inverts() {
    let snapshot = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    let existing_id = widget_id(&snapshot.host_snapshot.widgets[1]).to_string();
    let diff = Generation2dDiffRead::new(Generation2dDiff {
        widgets: Some(Generation2dWidgetsDelta {
            inserted: vec![Generation2dWidgetInsertion { index: 1, row: Widget::InputNote { id: "fresh".into(), text: "fresh".into() } }],
            modified: vec![Generation2dWidgetModification { id: existing_id.clone(), patch: Generation2dWidgetPatch::Replace { widget: Widget::InputNote { id: existing_id.clone(), text: "replaced".into() } } }],
            ..Default::default()
        }),
        ..Default::default()
    });
    let next = Generation2dSnapshotRead::new(protocol::apply_diff(&*diff, &snapshot).expect("valid mutation diff"));
    assert_eq!(next.host_snapshot.widgets.len(), snapshot.host_snapshot.widgets.len() + 1);
    assert_eq!(widget_id(&next.host_snapshot.widgets[1]), "fresh");
    let replaced = next.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == existing_id.as_str()).expect("replaced");
    assert_eq!(replaced, &Widget::InputNote { id: existing_id, text: "replaced".into() });
    let inverse = Generation2dDiffRead::new(DiffAlgebra::inverse(&*diff, &*snapshot));
    let restored = Generation2dSnapshotRead::new(protocol::apply_diff(&*inverse, &next).expect("inverse applies"));
    assert_eq!(*restored, *snapshot);
}

/// ⚖️ A row added then removed is no row at all; a row removed then added again is a replace applied in sequence order.
#[test]
fn absorb_cancels_add_then_remove_and_turns_remove_then_add_into_replace() {
    let base = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    let note = Widget::InputNote { id: "note".into(), text: "n".into() };
    let tail = base.host_snapshot.widgets.len();
    let added = Generation2dDiff { widgets: Some(Generation2dWidgetsDelta { inserted: vec![Generation2dWidgetInsertion { index: tail, row: note.clone() }], ..Default::default() }), ..Default::default() };
    let removed = Generation2dDiff { widgets: Some(Generation2dWidgetsDelta { removed: vec![Generation2dWidgetRemoval { id: "note".into(), index: tail }], ..Default::default() }), ..Default::default() };
    let mut cancelled = added.clone();
    MutationDiff::absorb(&mut cancelled, removed);
    assert_eq!(cancelled.widgets, None);
    let existing = widget_id(&base.host_snapshot.widgets[0]).to_string();
    let drop_existing = Generation2dDiff { widgets: Some(Generation2dWidgetsDelta { removed: vec![Generation2dWidgetRemoval { id: existing.clone(), index: 0 }], ..Default::default() }), ..Default::default() };
    let bring_back = Generation2dDiff { widgets: Some(Generation2dWidgetsDelta { inserted: vec![Generation2dWidgetInsertion { index: 0, row: Widget::InputNote { id: existing.clone(), text: "again".into() } }], ..Default::default() }), ..Default::default() };
    let mut replaced = drop_existing.clone();
    MutationDiff::absorb(&mut replaced, bring_back.clone());
    let sequential = Generation2dSnapshotRead::new(protocol::apply_diff(&bring_back, &Generation2dSnapshotRead::new(protocol::apply_diff(&drop_existing, &base).expect("drop applies"))).expect("bring back applies"));
    let absorbed = Generation2dSnapshotRead::new(protocol::apply_diff(&replaced, &base).expect("absorbed applies"));
    assert_eq!(*absorbed, *sequential);
    Generation2dDiff::retire_cold(replaced);
    Generation2dDiff::retire_cold(bring_back);
    Generation2dDiff::retire_cold(added);
}

/// 🔐️ LAW: the generic replay seams reach this artifact through the `MutationDiff` CONTRACT, never
/// through the inherent helper, so both cold-retirement hooks must be overridden — an inhabited
/// `fixture` owns an `OrderedMap<WidgetLayout>` root whose bare drop aborts the process, which is
/// exactly what `os_vcs::apply_mutation` did on every undone/redone 2d operation.
#[test]
fn the_mutation_diff_contract_retires_an_inhabited_layout_delta_and_its_scratch_projection() {
    let base = empty_generation2d_snapshot();
    let diff = Generation2dDiff { layout: Some(Generation2dLayoutDelta { added: vec![Generation2dLayoutRow { id: "laid-out".into(), layout: semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 } }], ..Default::default() }), ..Default::default() };
    let scratch = protocol::apply_diff(&diff, &base).expect("inhabited layout delta applies");
    assert!(scratch.host_snapshot.layout.contains_key("laid-out"));
    <Generation2dDiff as MutationDiff<Generation2dSnapshot>>::retire_projection(scratch);
    <Generation2dDiff as MutationDiff<Generation2dSnapshot>>::retire_cold(diff);
    base.retire_cold();
}

/// ⚖️ A positional widget delta moves a middle row, sums with an insertion, and inverts row by row; applied through the central applier.
#[test]
fn positional_widget_delta_moves_sums_and_inverts_at_middle_rows() {
    let base = Generation2dSnapshotRead::new(empty_generation2d_snapshot());
    let length = base.host_snapshot.widgets.len();
    assert!(length >= 2, "the empty snapshot seeds at least two widgets");
    let note = || Widget::InputNote { id: "fresh".into(), text: "fresh".into() };
    let insert = Generation2dDiffRead::new(Generation2dDiff { widgets: Some(Generation2dWidgetsDelta::insertion(1, note())), ..Default::default() });
    let inserted = Generation2dSnapshotRead::new(protocol::apply_diff(&*insert, &base).expect("valid insertion"));
    let relocate = Generation2dDiffRead::new(Generation2dDiff { widgets: Some(Generation2dWidgetsDelta::relocation(&inserted.host_snapshot.widgets, length, 0)), ..Default::default() });
    let moved = Generation2dSnapshotRead::new(protocol::apply_diff(&*relocate, &inserted).expect("valid relocation"));
    assert_eq!(widget_id(&moved.host_snapshot.widgets[0]), widget_id(&inserted.host_snapshot.widgets[length]));
    let mut sum = Generation2dDiff { widgets: Some(Generation2dWidgetsDelta::insertion(1, note())), ..Default::default() };
    MutationDiff::absorb(&mut sum, Generation2dDiff { widgets: Some(Generation2dWidgetsDelta::relocation(&inserted.host_snapshot.widgets, length, 0)), ..Default::default() });
    let sum = Generation2dDiffRead::new(sum);
    let summed = Generation2dSnapshotRead::new(protocol::apply_diff(&*sum, &base).expect("valid sum"));
    assert_eq!(summed.host_snapshot.widgets, moved.host_snapshot.widgets);
    let inverse = Generation2dDiffRead::new(DiffAlgebra::inverse(&*sum, &base));
    let restored = Generation2dSnapshotRead::new(protocol::apply_diff(&*inverse, &moved).expect("valid inverse"));
    assert_eq!(restored.host_snapshot.widgets, base.host_snapshot.widgets);
}
