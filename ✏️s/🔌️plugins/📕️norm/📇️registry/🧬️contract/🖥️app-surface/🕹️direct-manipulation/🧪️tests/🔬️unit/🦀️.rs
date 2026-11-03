use super::*;
use crate::{DragPreview, FieldWrite};
use semio_s_plugin_norm_structural_scene::{Affine, EntityKind, EntityRef, FieldBinding, ChoiceToken, GripStyle, Handle, HandleKind, Interval, LocalizedCopy, Quantise, SnapRule, Vec3};

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

fn bolts_handle() -> Handle {
    let mut handle = length_handle();
    handle.id = "b1/bolts/count".into();
    handle.writes = vec![FieldBinding { quantise: Some(Quantise::Int), ..FieldBinding::new("members[id=b1].bolts", Affine { base: 0.0, k: 2.0 }, "-") }, FieldBinding::new("members[id=b1].pitch", Affine { base: 0.1, k: 0.5 }, "m")];
    handle
}

fn run(tool: &HandleDragTool, events: Vec<DragEvent>) -> (DragState, Vec<DragEffect>) {
    events.into_iter().fold((DragState::Idle, Vec::new()), |(state, mut effects), event| {
        let (next, effect) = tool.step(state, event);
        effects.push(effect);
        (next, effects)
    })
}

fn preview_value(effect: &DragEffect) -> f64 {
    match effect {
        DragEffect::Preview(DragPreview { writes, .. }) => writes[0].value.as_f64().expect("number"),
        other => panic!("expected preview, got {other:?}"),
    }
}

fn press(s: f64) -> Vec<DragEvent> {
    vec![DragEvent::Hover { handle: Some("b1/length/end".into()), s }, DragEvent::Press { handle: "b1/length/end".into(), s }]
}

#[test]
fn a_pointer_drag_previews_snapped_values_and_the_release_commits_one_bundle() {
    let handles = [length_handle()];
    let tool = HandleDragTool { handles: &handles, seq: 7 };
    let mut events = press(4.0);
    events.extend([DragEvent::Move { s: 4.27, modifiers: Modifiers::NONE }, DragEvent::Release]);
    let (state, effects) = run(&tool, events);
    assert!((preview_value(&effects[2]) - 4.25).abs() < 1e-12);
    assert!(matches!(&effects[2], DragEffect::Preview(p) if p.started_seq == 7 && p.handle == "b1/length/end"));
    assert_eq!(effects[3], DragEffect::Commit(vec![FieldWrite::number("members[id=b1].length", 4.25)]));
    assert!(matches!(state, DragState::Hover { s, .. } if (s - 4.27).abs() < 1e-12));
}

#[test]
fn a_release_at_the_origin_or_a_click_commits_nothing() {
    let handles = [length_handle()];
    let tool = HandleDragTool { handles: &handles, seq: 1 };
    let mut click = press(4.0);
    click.push(DragEvent::Release);
    assert_eq!(run(&tool, click).1.last(), Some(&DragEffect::None));
    let mut there_and_back = press(4.0);
    there_and_back.extend([DragEvent::Move { s: 6.0, modifiers: Modifiers::NONE }, DragEvent::Move { s: 4.0, modifiers: Modifiers::NONE }, DragEvent::Release]);
    assert_eq!(run(&tool, there_and_back).1.last(), Some(&DragEffect::Cleared));
}

#[test]
fn escape_and_lost_capture_abort_without_trace() {
    let handles = [length_handle()];
    let tool = HandleDragTool { handles: &handles, seq: 1 };
    let mut escape = press(4.0);
    escape.extend([DragEvent::Move { s: 6.0, modifiers: Modifiers::NONE }, DragEvent::Abort(AbortReason::Escape)]);
    let (state, effects) = run(&tool, escape);
    assert_eq!(effects.last(), Some(&DragEffect::Cleared));
    assert_eq!(state, DragState::Hover { handle: "b1/length/end".into(), s: 4.0 });
    let mut lost = press(4.0);
    lost.extend([DragEvent::Move { s: 6.0, modifiers: Modifiers::NONE }, DragEvent::Abort(AbortReason::CaptureLost)]);
    assert_eq!(run(&tool, lost).0, DragState::Idle);
    assert!(!effects.iter().any(|e| matches!(e, DragEffect::Commit(_))));
}

#[test]
fn limits_clamp_the_preview_and_unknown_handles_are_ignored() {
    let handles = [length_handle()];
    let tool = HandleDragTool { handles: &handles, seq: 1 };
    let mut events = press(4.0);
    events.push(DragEvent::Move { s: 99.0, modifiers: Modifiers::NONE });
    assert_eq!(preview_value(run(&tool, events).1.last().unwrap()), 12.0);
    let (state, _) = run(&tool, vec![DragEvent::Press { handle: "nope".into(), s: 1.0 }]);
    assert_eq!(state, DragState::Idle);
}

#[test]
fn multi_field_handles_write_every_binding_with_quantisation() {
    let handle = bolts_handle();
    let writes = writes_for(&handle, 2.4);
    let values: Vec<(&str, f64)> = writes.iter().map(|w| (w.path.as_str(), w.value.as_f64().unwrap())).collect();
    assert_eq!(values[0], ("members[id=b1].bolts", 5.0));
    assert_eq!(values[1].0, "members[id=b1].pitch");
    assert!((values[1].1 - 1.3).abs() < 1e-12);
}

#[test]
fn the_keyboard_covers_every_gesture() {
    let handles = [length_handle()];
    let tool = HandleDragTool { handles: &handles, seq: 1 };
    let hover = DragEvent::Hover { handle: Some("b1/length/end".into()), s: 4.0 };
    let nudge_up = |m| match key_intent(&Key::ArrowRight, m) { KeyIntent::Drag(e) => e, other => panic!("{other:?}") };
    let (s1, e1) = tool.step(DragState::Idle, hover);
    let (s2, e2) = tool.step(s1, nudge_up(Modifiers::NONE));
    assert!((preview_value(&e2) - 4.05).abs() < 1e-12 && e1 == DragEffect::None);
    let (s3, e3) = tool.step(s2, nudge_up(Modifiers::SHIFT));
    assert!((preview_value(&e3) - 4.55).abs() < 1e-12);
    let (s4, e4) = tool.step(s3, nudge_up(Modifiers::ALT));
    assert!((preview_value(&e4) - 4.555).abs() < 1e-12);
    let (s5, e5) = tool.step(s4, match key_intent(&Key::Enter, Modifiers::NONE) { KeyIntent::Drag(e) => e, other => panic!("{other:?}") });
    assert!(matches!(e5, DragEffect::Commit(_)) && matches!(s5, DragState::Hover { .. }));
    let (s6, _) = tool.step(s5, nudge_up(Modifiers::NONE));
    let (s7, e7) = tool.step(s6, match key_intent(&Key::Escape, Modifiers::NONE) { KeyIntent::Drag(e) => e, other => panic!("{other:?}") });
    assert_eq!(e7, DragEffect::Cleared);
    assert!(matches!(s7, DragState::Hover { .. }));
    assert_eq!(key_intent(&Key::Tab, Modifiers::SHIFT), KeyIntent::Cycle(-1));
    assert_eq!(key_intent(&Key::Char('x'), Modifiers::NONE), KeyIntent::Ignored);
}

#[test]
fn typed_entry_overrides_the_drag_exactly_and_respects_limits() {
    let handles = [length_handle()];
    let tool = HandleDragTool { handles: &handles, seq: 1 };
    let mut events = press(4.0);
    events.push(DragEvent::Move { s: 5.0, modifiers: Modifiers::NONE });
    events.extend("4,237".chars().map(DragEvent::Entry));
    let (state, effects) = run(&tool, events);
    assert!((preview_value(effects.last().unwrap()) - 4.237).abs() < 1e-12);
    let (_, effects) = run(&tool, vec![DragEvent::Hover { handle: Some("b1/length/end".into()), s: 4.0 }, DragEvent::Entry('9'), DragEvent::Entry('9')]);
    assert_eq!(preview_value(effects.last().unwrap()), 12.0);
    let (_, backspaced) = tool.step(state, DragEvent::EntryBackspace);
    assert!((preview_value(&backspaced) - 4.23).abs() < 1e-12);
    assert_eq!(parse_entry("1,5"), Some(1.5));
    assert_eq!(parse_entry("-"), None);
}

#[test]
fn ctrl_frees_the_drag_from_snapping_but_not_from_limits() {
    let handle = length_handle();
    assert_eq!(snap(&handle, 4.27, Modifiers::CTRL), Snapped { s: 4.27, source: SnapSource::Free });
    assert_eq!(snap(&handle, 99.0, Modifiers::CTRL), Snapped { s: 12.0, source: SnapSource::Limit });
    assert_eq!(snap(&handle, 4.27, Modifiers::NONE).source, SnapSource::Increment);
}

#[test]
fn targets_and_catalogue_values_win_within_tolerance_and_explain_themselves() {
    let mut handle = length_handle();
    handle.snap = SnapRule { grid: Some(0.5), increment: None, targets: vec![3.0], catalogue: vec![3.2], tolerance: 0.15 };
    assert_eq!(snap(&handle, 3.05, Modifiers::NONE), Snapped { s: 3.0, source: SnapSource::Target });
    assert_eq!(snap(&handle, 3.18, Modifiers::NONE), Snapped { s: 3.2, source: SnapSource::Catalogue });
    assert_eq!(snap(&handle, 4.6, Modifiers::NONE), Snapped { s: 4.5, source: SnapSource::Grid });
}

#[test]
fn snapping_without_ctrl_equals_the_schema_resolution() {
    let mut handle = length_handle();
    handle.snap.targets = vec![2.0, 7.5];
    handle.snap.tolerance = 0.12;
    for raw in [-3.0, 0.4, 1.93, 2.07, 4.123, 7.41, 11.99, 40.0] {
        assert_eq!(snap(&handle, raw, Modifiers::NONE).s, handle.resolve_s(raw), "raw {raw}");
    }
}

#[test]
fn nudge_steps_scale_with_modifiers_and_stop_at_the_limits() {
    let handle = length_handle();
    assert_eq!(nudge_step(&handle, Modifiers::SHIFT), 0.5);
    assert!((nudge_step(&handle, Modifiers::ALT) - 0.005).abs() < 1e-15);
    assert_eq!(nudge_step(&handle, Modifiers { shift: true, alt: true, ctrl: false }), 0.05);
    assert_eq!(nudge(&handle, 11.99, 1.0, Modifiers::SHIFT), Snapped { s: 12.0, source: SnapSource::Limit });
    let mut bare = length_handle();
    bare.snap = SnapRule::NONE;
    assert!((nudge_step(&bare, Modifiers::NONE) - 0.115).abs() < 1e-12);
    bare.limits = None;
    assert_eq!(nudge_step(&bare, Modifiers::NONE), 1.0);
}

#[test]
fn tab_cycles_the_handles_in_both_directions() {
    let ids: Vec<String> = ["a", "b", "c"].map(String::from).to_vec();
    assert_eq!(cycle_focus(&ids, None, 1).as_deref(), Some("a"));
    assert_eq!(cycle_focus(&ids, None, -1).as_deref(), Some("c"));
    assert_eq!(cycle_focus(&ids, Some("c"), 1).as_deref(), Some("a"));
    assert_eq!(cycle_focus(&ids, Some("a"), -1).as_deref(), Some("c"));
    assert_eq!(cycle_focus(&[], None, 1), None);
    assert_eq!(focus_announcement("Length", "4.25", "m"), "Length: 4.25 m");
}

#[test]
fn host_phases_map_to_statechart_events() {
    assert_eq!(DragPhase::parse("stream", None), Some(DragPhase::Stream));
    assert_eq!(DragPhase::parse("abort", Some("captureLost")), Some(DragPhase::Abort(AbortReason::CaptureLost)));
    assert_eq!(DragPhase::parse("abort", None), Some(DragPhase::Abort(AbortReason::Escape)));
    assert_eq!(DragPhase::parse("nope", None), None);
    assert_eq!(DragPhase::Commit.event(1.0, Modifiers::NONE), DragEvent::Release);
}

#[test]
fn pick_targets_round_trip_and_grips_win() {
    for raw in ["b1", "b1#face:3", "b1#edge:0", "b1#vertex:2", "b1#row:11"] {
        assert_eq!(PickTarget::parse(raw).expect(raw).format(), raw);
    }
    assert_eq!(PickTarget::parse("b1#face:x"), None);
    assert_eq!(PickTarget::parse(""), None);
    assert_eq!(resolve_pick(Some("b1/length/end"), Some("b1")), PickOutcome::Handle("b1/length/end".into()));
    assert_eq!(resolve_pick(None, Some("b1#face:3")), PickOutcome::Entity(PickTarget { entity_id: "b1".into(), part: PickPart::Face(3) }));
    assert_eq!(resolve_pick(None, None), PickOutcome::Nothing);
}

#[test]
fn handles_follow_the_selection_in_stable_order_and_sub_parts_scope_them() {
    let mut face = length_handle();
    face.id = "b1/thickness/face:3".into();
    let mut other = length_handle();
    other.id = "b2/length/end".into();
    other.entity = EntityRef::new(EntityKind::Member, "b2", "members[id=b2]");
    let all = [length_handle(), face.clone(), other.clone()];
    let selection = [other.entity.clone(), all[0].entity.clone(), other.entity.clone()];
    let ids: Vec<&str> = handles_for(&all, &selection).iter().map(|h| h.id.as_str()).collect();
    assert_eq!(ids, ["b2/length/end", "b1/length/end", "b1/thickness/face:3"]);
    assert!(PickTarget::parse("b1#face:3").unwrap().owns(&face));
    assert!(!PickTarget::parse("b1#face:4").unwrap().owns(&face));
    assert!(PickTarget::whole("b1").owns(&face));
}

#[test]
fn snapping_agrees_with_the_numpy_oracle() {
    let cases = [(0.0, 0.05, 0.0), (4.27, 0.05, 0.0), (4.123, 0.05, 0.0), (11.99, 0.05, 0.0), (99.0, 0.05, 0.0), (-3.0, 0.05, 0.0), (1.93, 0.05, 0.12), (7.41, 0.05, 0.12), (6.38, 0.25, 0.0), (2.07, 0.25, 0.12)];
    let targets = [2.0, 7.5];
    let json = format!(
        "[{}]",
        cases.iter().map(|(raw, inc, tol)| format!("{{\"raw\":{raw},\"min\":0.5,\"max\":12.0,\"targets\":{:?},\"tolerance\":{tol},\"increment\":{inc}}}", if *tol > 0.0 { targets.to_vec() } else { vec![] })).collect::<Vec<_>>().join(",")
    );
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🕹️direct-manipulation/🔮️oracles/🐍️.py");
    let out = semio_s_plugin_norm_compliance::python_oracle::run(script, &[], Some(json.as_bytes()));
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let expected: Vec<f64> = String::from_utf8(out.stdout).unwrap().lines().map(|l| l.parse().unwrap()).collect();
    for ((raw, inc, tol), want) in cases.iter().zip(expected) {
        let mut handle = length_handle();
        handle.snap = SnapRule { grid: None, increment: Some(*inc), targets: if *tol > 0.0 { targets.to_vec() } else { vec![] }, catalogue: vec![], tolerance: *tol };
        assert!((snap(&handle, *raw, Modifiers::NONE).s - want).abs() < 1e-12, "raw {raw}");
    }
}

fn shape_handle() -> Handle {
    let tokens = ["None", "Flat", "Tee", "Angle"].iter().map(|t| semio_s_plugin_norm_structural_scene::ChoiceToken { token: (*t).into(), label: LocalizedCopy::new(format!("{t} en"), format!("{t} de")) }).collect();
    let mut handle = length_handle();
    handle.id = "p1/stiffener-shape".into();
    handle.kind = HandleKind::ChoiceStepper;
    handle.writes = vec![FieldBinding::token("platedPanels[id=p1].stiffenerShape", tokens)];
    handle.snap = SnapRule { increment: Some(1.0), ..SnapRule::NONE };
    handle.limits = Some(Interval::new(0.0, 3.0));
    handle
}

#[test]
fn a_choice_stepper_cycles_by_keyboard_and_commits_the_enum_token() {
    let handles = [shape_handle()];
    let tool = HandleDragTool { handles: &handles, seq: 1 };
    let key = |k: Key| match key_intent(&k, Modifiers::NONE) { KeyIntent::Drag(e) => e, other => panic!("{other:?}") };
    let (s1, _) = tool.step(DragState::Idle, DragEvent::Hover { handle: Some("p1/stiffener-shape".into()), s: 3.0 });
    let (s2, e2) = tool.step(s1, key(Key::ArrowRight));
    let DragEffect::Preview(preview) = e2 else { panic!("preview expected") };
    assert_eq!((preview.s, &preview.writes[0].value), (0.0, &semio_framework_value::DslValue::String("None".into())));
    let (s3, _) = tool.step(s2, key(Key::ArrowRight));
    let (s4, e4) = tool.step(s3, key(Key::Enter));
    let DragEffect::Commit(writes) = e4 else { panic!("commit expected") };
    assert_eq!(writes, vec![FieldWrite { path: "platedPanels[id=p1].stiffenerShape".into(), value: semio_framework_value::DslValue::String("Flat".into()) }]);
    assert!(matches!(s4, DragState::Hover { .. }));
}

#[test]
fn a_nudge_moves_the_bound_field_by_exactly_its_schema_step_and_modifiers_scale_it() {
    let mut handle = length_handle();
    handle.writes[0].quantise = Some(Quantise::Round { step: 0.01 });
    handle.writes[0].map = Affine { base: 0.0, k: 0.5 };
    assert!((schema_step(&handle).expect("round step") - 0.02).abs() < 1e-12, "a round step of 0.01 on a slope of 0.5 is 0.02 handle units");
    let first = nudge(&handle, 4.0, 1.0, Modifiers::NONE).s;
    assert!((handle.writes[0].value_at(first) - handle.writes[0].value_at(4.0) - 0.01).abs() < 1e-12, "one arrow press moves the field by one schema step");
    assert!((nudge_step(&handle, Modifiers::SHIFT) - 0.2).abs() < 1e-12 && (nudge_step(&handle, Modifiers::ALT) - 0.002).abs() < 1e-12);
    handle.writes[0].quantise = Some(Quantise::Int);
    handle.writes[0].map = Affine::IDENTITY;
    assert_eq!(schema_step(&handle), Some(1.0));
    handle.writes[0].quantise = Some(Quantise::Choice { values: vec![0.5, 2.0, 1.0, 4.0] });
    assert_eq!(schema_step(&handle), Some(0.5), "the smallest gap of a choice list");
    handle.writes[0].quantise = Some(Quantise::Decades { per_decade: 4.0 });
    assert_eq!(schema_step(&handle), Some(0.25));
    handle.writes[0].quantise = None;
    assert_eq!(schema_step(&handle), None);
    assert_eq!(nudge_step(&handle, Modifiers::NONE), 0.05, "without a schema step the snap increment applies");
}

#[test]
fn enter_on_a_focused_numeric_grip_opens_the_inline_field_and_a_second_enter_commits_one_bundle() {
    let handles = [length_handle()];
    let tool = HandleDragTool { handles: &handles, seq: 3 };
    let enter = || match key_intent(&Key::Enter, Modifiers::NONE) { KeyIntent::Drag(e) => e, other => panic!("{other:?}") };
    let hover = DragState::Hover { handle: "b1/length/end".into(), s: 4.25 };
    let (opened, effect) = tool.step(hover.clone(), enter());
    let DragEffect::Preview(preview) = effect else { panic!("the field opens as a preview") };
    assert_eq!((preview.entry.as_str(), preview.s), ("4.25", 4.25), "pre-filled with the shown value");
    let (typed, _) = tool.step(opened.clone(), DragEvent::EntryBackspace);
    let (typed, _) = tool.step(typed, DragEvent::EntryBackspace);
    let (typed, _) = tool.step(typed, DragEvent::Entry('7'));
    let (settled, committed) = tool.step(typed, enter());
    assert_eq!(committed, DragEffect::Commit(vec![FieldWrite::number("members[id=b1].length", 4.7)]));
    assert!(matches!(settled, DragState::Hover { .. }));
    assert_eq!(tool.step(opened.clone(), enter()).1, DragEffect::Cleared, "an unchanged value commits nothing, so no undo step");
    let (cancelled, cleared) = tool.step(opened, DragEvent::Abort(AbortReason::Escape));
    assert_eq!((cancelled, cleared), (hover, DragEffect::Cleared));
}

#[test]
fn enter_on_a_token_grip_opens_no_field() {
    let mut handle = length_handle();
    handle.writes = vec![FieldBinding::token("members[id=b1].kind", vec![ChoiceToken { token: "a".into(), label: LocalizedCopy::new("A", "A") }, ChoiceToken { token: "b".into(), label: LocalizedCopy::new("B", "B") }])];
    let handles = [handle];
    let tool = HandleDragTool { handles: &handles, seq: 0 };
    let hover = DragState::Hover { handle: "b1/length/end".into(), s: 0.0 };
    assert_eq!(tool.step(hover.clone(), DragEvent::Commit), (hover, DragEffect::None));
}
