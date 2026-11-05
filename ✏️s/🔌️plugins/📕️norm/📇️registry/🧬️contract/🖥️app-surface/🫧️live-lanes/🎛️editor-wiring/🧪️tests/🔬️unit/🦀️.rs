use super::*;
use crate::direct_manipulation::{AbortReason, DragPhase};
use crate::{GhostPreview, GhostSource, NormPresence};
use semio_s_plugin_norm_structural_scene::{Affine, EntityKind, EntityRef, FieldBinding, GripStyle, HandleKind, Interval, LocalizedCopy, SnapRule, Vec3};

fn length_handle() -> Handle {
    Handle {
        id: "b1/length/end".into(),
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

fn gesture(step: GestureStep) -> Gesture {
    Gesture { entity: Some(EntityRef::new(EntityKind::Member, "b1", "members[id=b1]")), handle: "b1/length/end".into(), step }
}

fn phase(phase: DragPhase, s: f64) -> Gesture {
    gesture(GestureStep::Phase { phase, s, origin: 4.0, modifiers: Modifiers::NONE, entry: String::new() })
}

fn key(key: Key, s: f64) -> Gesture {
    gesture(GestureStep::Key { key, s, origin: 4.0, modifiers: Modifiers::NONE, entry: String::new() })
}

fn fold(transient: NormTransient, outcome: &GestureOutcome) -> NormTransient {
    outcome.events.iter().cloned().fold(transient, NormTransient::apply)
}

#[test]
fn a_drag_previews_in_the_transient_and_commits_exactly_one_write_bundle() {
    let handles = [length_handle()];
    let streamed = gesture_step(&handles, &NormTransient::default(), &phase(DragPhase::Stream, 4.27), 3);
    let transient = fold(NormTransient::default(), &streamed);
    assert_eq!(transient.drag.as_ref().map(|d| d.writes.clone()), Some(vec![FieldWrite::number("members[id=b1].length", 4.25)]));
    assert_eq!(streamed.commit, None);
    let released = gesture_step(&handles, &transient, &phase(DragPhase::Commit, 4.27), 3);
    assert_eq!(released.commit, Some(vec![FieldWrite::number("members[id=b1].length", 4.25)]));
    assert!(fold(transient, &released).drag.is_none());
    assert_eq!(commit_writes(&handles, &phase(DragPhase::Commit, 4.27)), Some(vec![FieldWrite::number("members[id=b1].length", 4.25)]));
}

#[test]
fn escape_and_lost_capture_leave_no_trace_and_no_commit() {
    let handles = [length_handle()];
    let transient = fold(NormTransient::default(), &gesture_step(&handles, &NormTransient::default(), &phase(DragPhase::Stream, 6.0), 1));
    assert!(transient.drag.is_some());
    for abort in [phase(DragPhase::Abort(AbortReason::Escape), 6.0), phase(DragPhase::Abort(AbortReason::CaptureLost), 6.0), key(Key::Escape, 6.0)] {
        let outcome = gesture_step(&handles, &transient, &abort, 1);
        assert_eq!(outcome.commit, None);
        assert!(fold(transient.clone(), &outcome).drag.is_none());
        assert_eq!(commit_writes(&handles, &abort), None);
    }
}

#[test]
fn a_keyboard_nudge_commits_what_the_equivalent_drag_commits() {
    let handles = [length_handle()];
    let nudged = gesture_step(&handles, &NormTransient::default(), &key(Key::ArrowRight, 4.0), 1);
    let transient = fold(NormTransient::default(), &nudged);
    let target = transient.drag.as_ref().map(|d| d.s).expect("the nudge previews");
    let by_key = commit_writes(&handles, &gesture(GestureStep::Key { key: Key::Enter, s: target, origin: 4.0, modifiers: Modifiers::NONE, entry: String::new() }));
    let by_pointer = commit_writes(&handles, &phase(DragPhase::Commit, target));
    assert_eq!(by_key, by_pointer);
    assert_eq!(by_key, Some(vec![FieldWrite::number("members[id=b1].length", 4.05)]));
}

#[test]
fn typed_entry_commits_the_exact_value() {
    let handles = [length_handle()];
    let typed = gesture(GestureStep::Key { key: Key::Enter, s: 4.0, origin: 4.0, modifiers: Modifiers::NONE, entry: "7,3".into() });
    assert_eq!(commit_writes(&handles, &typed), Some(vec![FieldWrite::number("members[id=b1].length", 7.3)]));
}

#[test]
fn a_release_at_the_origin_commits_nothing() {
    assert_eq!(commit_writes(&[length_handle()], &phase(DragPhase::Commit, 4.0)), None);
}

#[test]
fn gestures_parse_from_the_wire_json_and_refuse_ambiguous_ones() {
    let json = r#"{"entity":{"kind":"member","id":"b1"},"handle":"b1/length/end","s":4.27,"origin":4.0,"phase":"stream","modifiers":{"shift":true}}"#;
    let parsed = Gesture::parse(json).expect("parses");
    assert_eq!(parsed.handle, "b1/length/end");
    assert!(matches!(parsed.step, GestureStep::Phase { phase: DragPhase::Stream, modifiers, .. } if modifiers.shift && !modifiers.alt));
    let both = r#"{"entity":{"kind":"member","id":"b1"},"handle":"h","s":1,"phase":"stream","key":"Enter"}"#;
    assert!(Gesture::parse(both).is_err());
    assert!(Gesture::parse(r#"{"handle":"h","s":1,"phase":"stream"}"#).expect("handle-only gesture").entity.is_none());
    assert!(Gesture::parse(r#"{"s":1,"phase":"stream"}"#).is_err());
}

#[test]
fn the_live_invocation_reads_every_row_and_a_plain_bundle_has_no_gesture() {
    let plain = LiveInvocation::set_fields(r#"[{"path":"members[id=b1].length","value":4.25}]"#, None, None).expect("plain");
    assert!(plain.gesture.is_none());
    assert!(matches!(plain.command, LiveCommand::SetFields { ref writes } if writes.len() == 1));
    assert!(matches!(LiveInvocation::preview_remedy("c1", 2, 0, None).expect("remedy").command, LiveCommand::PreviewRemedy { index: 2, .. }));
    let insert = LiveInvocation::preview_remedy("", 0, 0, Some(r#"{"spec":"member","at":[1,2,3]}"#)).expect("insert ghost");
    assert!(matches!(insert.command, LiveCommand::PreviewInsert { at: Some(at), .. } if at == [1.0, 2.0, 3.0]));
    assert_eq!(LiveInvocation::clear_preview().command, LiveCommand::ClearPreview);
    assert_eq!(LiveInvocation::cancel_evaluation().command, LiveCommand::CancelEvaluation);
    assert!(LiveInvocation::evaluate_units(Some(r#"["u1"]"#), "commit").is_ok());
    assert!(LiveInvocation::evaluate_units(Some("1"), "commit").is_err());
}

#[test]
fn the_lane_stores_round_trip_and_unreadable_text_is_the_empty_lane() {
    let mut transient = NormTransient::default();
    transient = transient.apply(LaneEvent::GhostPreviewed(GhostPreview { writes: vec![FieldWrite::number("a", 1.0)], source: GhostSource::Hover }));
    assert_eq!(crate::transient_of_state(&crate::transient_state(&transient)), transient);
    assert_eq!(crate::transient_of_state("not json"), NormTransient::default());
    let presence = NormPresence::new("ana");
    assert_eq!(crate::presence_of_state(&crate::presence_state(&presence)), presence);
}

mod marker_cell {
    use super::*;
    use semio_s_plugin_norm_structural_scene::{Axis, AxisScale, DataTable, Diagram2d, Marker, MarkerShape, QuantityKind, Quantise, TickRule, Tone, Vec2};

    fn diagram() -> Diagram2d {
        let axis = |unit: &str, domain: Option<Interval>| Axis { label: LocalizedCopy::new("Position", "Lage"), quantity: QuantityKind::Length, unit: unit.into(), scale: AxisScale::Linear, domain, ticks: TickRule::Auto };
        let mut binding = FieldBinding::new("placements[id=p1].x", Affine { base: 0.0, k: 1.0 }, "m");
        binding.quantise = Some(Quantise::Round { step: 0.01 });
        Diagram2d {
            id: "plan".into(),
            title: LocalizedCopy::new("Plan", "Grundriss"),
            description: LocalizedCopy::new("Plan", "Grundriss"),
            axes: vec![axis("m", Some(Interval::new(0.0, 10.0))), axis("mm", None)],
            series: Vec::new(),
            markers: vec![Marker { id: "p1".into(), at: Vec2::new(2.5, 14.0), shape: MarkerShape::Diamond, tone: Tone::Warning, label: LocalizedCopy::new("Placement P1", "Platzierung P1"), utilisation: Some(0.93), bound_check: Some("c1".into()), binding: Some(binding) }],
            annotations: Vec::new(),
            focus: None,
            legend: false,
            table_twin: DataTable { columns: Vec::new() },
        }
    }

    fn committed(value: &str) -> Option<Gesture> {
        let text = format!("{{\"handle\":\"marker:plan:p1\",\"key\":\"Enter\",\"s\":2.5,\"origin\":2.5,\"value\":\"{value}\"}}");
        let args = semio_framework_pack_json::from_json_str::<DslValue>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
        Gesture::parse(&crate::gesture_json_arg(Some(&args))?).ok()
    }

    #[test]
    fn the_text_a_cell_commits_becomes_the_typed_entry_of_an_enter_gesture() {
        let gesture = committed("350 cm").expect("a gesture");
        assert_eq!(gesture.handle, "marker:plan:p1");
        assert!(matches!(&gesture.step, GestureStep::Key { key: Key::Enter, entry, s, origin, .. } if entry == "350 cm" && *s == 2.5 && *origin == 2.5));
    }

    #[test]
    fn a_typed_cell_value_is_the_one_commit_of_a_drag_released_at_that_value() {
        let diagrams = [diagram()];
        let writes = marker_row_writes_in(&diagrams, &committed("350 cm").unwrap()).expect("addresses a marker").expect("a commit");
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0].path, "placements[id=p1].x");
        assert!((writes[0].value.as_f64().unwrap() - 3.5).abs() < 1e-12, "the unit-aware parse of the table row");
        assert_eq!(marker_row_writes_in(&diagrams, &committed("2,5").unwrap()), Some(None), "the shown value commits nothing");
        assert_eq!(marker_row_writes_in(&diagrams, &committed("3 kN").unwrap()), Some(None), "a unit of another dimension is refused");
        assert!((marker_row_writes_in(&diagrams, &committed("99").unwrap()).unwrap().unwrap()[0].value.as_f64().unwrap() - 10.0).abs() < 1e-12, "clamped to the limits of the axis");
    }

    #[test]
    fn only_a_committing_enter_on_a_known_marker_is_a_marker_commit() {
        let diagrams = [diagram()];
        assert_eq!(marker_row_writes_in(&diagrams, &gesture(GestureStep::Key { key: Key::Enter, s: 1.0, origin: 1.0, modifiers: Modifiers::NONE, entry: "1".into() })), None, "a 3D grip is no marker");
        let mut unknown = committed("3 m").unwrap();
        unknown.handle = "marker:plan:gone".into();
        assert_eq!(marker_row_writes_in(&diagrams, &unknown), Some(None));
        let mut escape = committed("3 m").unwrap();
        escape.step = GestureStep::Key { key: Key::Escape, s: 2.5, origin: 2.5, modifiers: Modifiers::NONE, entry: "3 m".into() };
        assert_eq!(marker_row_writes_in(&diagrams, &escape), Some(None), "Escape never commits");
    }
}
