use super::*;
use semio_framework_value::{FromValue, ToValue};
use semio_s_plugin_norm_structural_scene::{canonical_json, Affine, EntityKind, EntityRef, FieldBinding, GripStyle, Handle, HandleKind, Interval, LocalizedCopy, SnapRule, Vec3};

fn handle() -> Handle {
    Handle {
        id: "member-b1/length/end".into(),
        entity: EntityRef::new(EntityKind::Member, "b1", "members[id=b1]"),
        kind: HandleKind::ScalarAlong { axis: Vec3::new(1.0, 0.0, 0.0) },
        anchor: Vec3::new(4.0, 0.0, 0.0),
        grip: GripStyle::Arrow,
        writes: vec![FieldBinding::new("members[id=b1].length", Affine::IDENTITY, "m")],
        snap: SnapRule { grid: None, increment: Some(0.05), targets: vec![], catalogue: vec![], tolerance: 0.0 },
        limits: Some(Interval::new(0.5, 12.0)),
        accessible_name: LocalizedCopy::new("Length", "Länge"),
    }
}

#[test]
fn a_default_transient_is_clear_and_an_open_drag_is_not() {
    let idle = NormTransient::default();
    assert!(idle.is_clear() && idle.eval.is_idle() && idle.eval.fraction() == 1.0);
    let dragging = idle.with_drag(Some(DragPreview::at(&handle(), 4.25, 7)));
    assert!(!dragging.is_clear());
    assert_eq!(dragging.clone().with_drag(None), NormTransient::default());
}

#[test]
fn a_drag_preview_resolves_bound_values_through_the_handle() {
    let drag = DragPreview::at(&handle(), 99.0, 3);
    assert_eq!(drag.started_seq, 3);
    assert_eq!(drag.writes, vec![FieldWrite::number("members[id=b1].length", 12.0)]);
}

#[test]
fn the_overlay_lists_drag_writes_before_ghost_writes() {
    let ghost = GhostPreview { writes: vec![FieldWrite::number("a", 1.0)], source: GhostSource::Remedy { check_id: "c1".into(), index: 0 } };
    let state = NormTransient::default().with_drag(Some(DragPreview::at(&handle(), 4.0, 1))).with_ghost(Some(ghost));
    let paths: Vec<_> = state.overlay().map(|w| w.path.as_str()).collect();
    assert_eq!(paths, ["members[id=b1].length", "a"]);
}

#[test]
fn progress_fraction_is_clamped() {
    let progress = EvalProgress { running: vec!["u1".into()], done: 5, total: 4, cancelled: false };
    assert_eq!(progress.fraction(), 1.0);
    assert_eq!(EvalProgress { done: 1, total: 4, ..EvalProgress::default() }.fraction(), 0.25);
}

#[test]
fn lanes_cover_the_persisted_shared_matrix_exactly_once() {
    let combos: Vec<_> = Lane::ALL.iter().map(|l| (l.persisted(), l.shared())).collect();
    assert_eq!(combos, [(true, true), (true, false), (false, true), (false, false)]);
}

#[test]
fn payloads_round_trip_through_the_value_codec() {
    let mut presence = NormPresence::new("ueli");
    presence.selection = vec![EntityRef::new(EntityKind::Member, "b1", "members[id=b1]")];
    presence.camera = Some(CameraPose { eye: Vec3::new(5.0, 5.0, 5.0), target: Vec3::ZERO, up: Vec3::new(0.0, 0.0, 1.0), fov_y: 0.8 });
    presence.drag = Some(DragPreview::at(&handle(), 4.0, 2));
    assert_eq!(NormPresence::from_value(presence.to_value()).expect("decode"), presence);
    let transient = NormTransient::default().with_drag(presence.drag.clone());
    assert_eq!(NormTransient::from_value(transient.to_value()).expect("decode"), transient);
}

#[test]
fn the_wire_form_is_camel_case() {
    let json = canonical_json(&NormTransient::default().with_ghost(Some(GhostPreview { writes: vec![], source: GhostSource::Hover })));
    assert!(json.contains("\"ghost\":{") && json.contains("\"source\":{\"kind\":\"hover\"}") && json.contains("\"eval\":{"));
}

#[test]
fn view_writes_select_the_combination_on_the_view_lane_and_never_the_document() {
    let writes = vec![FieldWrite::number("view.activeCombination", 3.0), FieldWrite::number("members[id=b1].length", 4.0)];
    assert!(is_view_write(&writes[0]) && !is_view_write(&writes[1]));
    assert_eq!(view_write_events(&writes), vec![LaneEvent::ViewItem(Some(3))]);
    assert_eq!(view_write_events(&[FieldWrite::number("view.activeCombination", -1.0)]), vec![LaneEvent::ViewItem(None)]);
    assert!(view_write_events(&[FieldWrite::number("view.unknown", 1.0)]).is_empty());
    let group = view_write_events(&[FieldWrite::number("view.activeTrafficGroup", 2.0), FieldWrite::number("view.activeCombination", 1.0)]);
    assert_eq!(group, vec![LaneEvent::ViewGroup(Some(2)), LaneEvent::ViewItem(Some(1))]);
    let lane = group.into_iter().fold(NormTransient::default(), NormTransient::apply).view;
    assert_eq!((lane.group, lane.item), (Some(2), Some(1)), "the group is its own ephemeral local view state, independent of the combination");
    assert_eq!(NormTransient::default().apply(LaneEvent::ViewGroup(Some(1))).apply(LaneEvent::ViewGroup(None)).view.group, None);
    assert_eq!(view_write_events(&[FieldWrite::number("view.activeTrafficGroup", -1.0)]), vec![LaneEvent::ViewGroup(None)]);
    let view = NormTransient::default().apply(LaneEvent::ViewItem(Some(3))).view;
    assert_eq!(view.item, Some(3));
    assert_eq!(NormTransient::default().apply(LaneEvent::ViewItem(Some(3))).apply(LaneEvent::ViewItem(None)).view.item, None);
}

#[test]
fn the_stepping_actions_become_step_intents_with_a_direction_and_the_check_event_folds() {
    assert_eq!(view_event(VIEW_ACTION_STEP_CHECK, None, Some(1.0)), Some(LaneEvent::ViewStep { step: ViewStep::Check, forward: true }));
    assert_eq!(view_event(VIEW_ACTION_STEP_GOVERNING, None, Some(-1.0)), Some(LaneEvent::ViewStep { step: ViewStep::Governing, forward: false }));
    assert_eq!(view_event(VIEW_ACTION_STEP_COMBINATION, None, Some(1.0)), Some(LaneEvent::ViewStep { step: ViewStep::Combination, forward: true }));
    assert_eq!(view_event(VIEW_ACTION_STEP_CHECK, None, None), None);
    let transient = NormTransient::default().apply(LaneEvent::ViewCheck(Some(2)));
    assert_eq!(transient.view.check, Some(2));
    assert_eq!(transient.clone().apply(LaneEvent::ViewStep { step: ViewStep::Check, forward: true }), transient);
    assert_eq!(transient.apply(LaneEvent::ViewCheck(None)).view.check, None);
}
