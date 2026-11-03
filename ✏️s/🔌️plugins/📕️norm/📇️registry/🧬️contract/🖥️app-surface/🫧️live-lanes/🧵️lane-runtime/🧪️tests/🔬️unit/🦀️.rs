use super::*;
use crate::direct_manipulation::{AbortReason, DragEvent, DragState, HandleDragTool, Modifiers};
use crate::{get_value_at_path, GhostSource};
use semio_s_plugin_norm_structural_scene::{Affine, EntityKind, EntityRef, FieldBinding, GripStyle, Handle, HandleKind, Interval, LocalizedCopy, SnapRule, Vec3};

fn handle() -> Handle {
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

fn document() -> DslValue {
    DslValue::object([("members".to_string(), DslValue::Array(vec![DslValue::object([("id".to_string(), DslValue::String("b1".into())), ("length".to_string(), DslValue::float(4.0))])]))])
}

fn length_of(document: &DslValue) -> f64 {
    get_value_at_path(document, "members[id=b1].length").unwrap().as_f64().unwrap()
}

fn presence(user: &str, hover: &str) -> NormPresence {
    let mut p = NormPresence::new(user);
    p.hover = Some(EntityRef::new(EntityKind::Member, hover, format!("members[id={hover}]")));
    p
}

#[test]
fn the_coalescer_keeps_the_latest_value_per_key() {
    let mut c = Coalescer::default();
    for i in 0..100 {
        c.push("a", i);
    }
    c.push("b", 1);
    assert_eq!((c.len(), c.dropped()), (2, 99));
    assert_eq!(c.drain(), vec![("a", 99), ("b", 1)]);
    assert!(c.is_empty());
}

#[test]
fn presence_is_limited_to_30_hz_and_only_sends_changes() {
    let mut p = PresencePublisher::default();
    p.offer(presence("u", "b1"));
    assert!(p.poll(0).is_some());
    p.offer(presence("u", "b2"));
    assert!(p.poll(20).is_none());
    assert_eq!(p.poll(34).unwrap().hover.unwrap().id, "b2");
    assert!(p.poll(200).is_none());
    let sent = (0..1000u64).filter(|t| { p.offer(presence("u", &format!("e{t}"))); p.poll(1000 + t).is_some() }).count();
    assert!(sent <= 30, "{sent} sends in one second");
}

#[test]
fn a_short_connection_loss_drops_ticks_never_blocks_and_recovers_at_once() {
    let mut p = PresencePublisher::default();
    p.offer(presence("u", "b1"));
    assert!(p.poll(0).is_some());
    p.set_link(Link::Lost { since_ms: 100 });
    for t in 0..20u64 {
        p.offer(presence("u", &format!("e{t}")));
        assert!(p.poll(100 + t * 34).is_none());
    }
    assert!(!p.offline_too_long(5_000) && p.offline_too_long(100 + MAX_LINK_LOSS_MS + 1));
    assert!(p.suppressed() >= 20);
    p.set_link(Link::Online);
    assert_eq!(p.poll(900).unwrap().hover.unwrap().id, "e19");
}

#[test]
fn evaluation_is_debounced_per_tier_and_commit_is_immediate() {
    let mut s = EvalScheduler::default();
    s.on_tick(0);
    assert!(s.poll(59).is_empty());
    assert_eq!(s.poll(60), [EvalTier::T1]);
    assert_eq!(s.poll(60), [EvalTier::T1]);
    assert!(s.poll(249).is_empty());
    assert_eq!(s.poll(250), [EvalTier::T2]);
    assert!(s.poll(1000).is_empty());
    s.on_commit();
    assert_eq!(s.poll(1000), [EvalTier::T1, EvalTier::T2]);
}

#[test]
fn the_preview_document_overlays_drag_then_ghost_and_leaves_the_original() {
    let doc = document();
    let drag = DragPreview { handle: "h".into(), writes: vec![FieldWrite::number("members[id=b1].length", 5.0)], started_seq: 1, origin: 0.0, s: 1.0, entry: String::new() };
    let ghost = GhostPreview { writes: vec![FieldWrite::number("members[id=b1].length", 6.0)], source: GhostSource::Hover };
    let transient = NormTransient::default().with_drag(Some(drag)).with_ghost(Some(ghost));
    assert_eq!(length_of(&overlay_document(&doc, &transient).unwrap()), 6.0);
    assert_eq!(length_of(&doc), 4.0);
    assert_eq!(overlay_document(&doc, &NormTransient::default()).unwrap(), doc);
    let bad = NormTransient::default().with_drag(Some(DragPreview { handle: "h".into(), writes: vec![FieldWrite::number("members[id=zz].length", 1.0)], started_seq: 1, origin: 0.0, s: 1.0, entry: String::new() }));
    assert!(overlay_document(&doc, &bad).is_err());
}

#[test]
fn the_transient_is_a_pure_fold_of_lane_events() {
    let events = vec![LaneEvent::EvalStarted { units: vec!["u1".into(), "u2".into()] }, LaneEvent::EvalUnitDone { unit: "u1".into() }];
    let t = events.into_iter().fold(NormTransient::default(), NormTransient::apply);
    assert_eq!((t.eval.done, t.eval.total, t.eval.running.clone()), (1, 2, vec!["u2".to_string()]));
    assert!((t.eval.fraction() - 0.5).abs() < 1e-12);
    let cancelled = t.apply(LaneEvent::EvalCancelled);
    assert!(cancelled.eval.cancelled && cancelled.eval.is_idle());
    assert!(cancelled.apply(LaneEvent::EvalFinished).is_clear());
}

#[test]
fn a_drag_previews_per_frame_commits_once_and_an_abort_leaves_no_trace() {
    let handles = [handle()];
    let tool = HandleDragTool { handles: &handles, seq: 3 };
    let mut rt = LaneRuntime::new("ueli");
    let mut state = DragState::Idle;
    let mut committed = Vec::new();
    let feed = |rt: &mut LaneRuntime, state: &mut DragState, committed: &mut Vec<Vec<FieldWrite>>, event: DragEvent, now: u64| {
        let (next, effect) = tool.step(std::mem::replace(state, DragState::Idle), event);
        *state = next;
        if let Some(writes) = rt.accept(effect, now) {
            committed.push(writes);
        }
    };
    feed(&mut rt, &mut state, &mut committed, DragEvent::Hover { handle: Some("b1/length/end".into()), s: 4.0 }, 0);
    feed(&mut rt, &mut state, &mut committed, DragEvent::Press { handle: "b1/length/end".into(), s: 4.0 }, 0);
    for i in 1..=50 {
        feed(&mut rt, &mut state, &mut committed, DragEvent::Move { s: 4.0 + f64::from(i) * 0.01, modifiers: Modifiers::NONE }, u64::from(i as u32));
    }
    assert!(rt.transient.is_clear() && rt.dropped_ticks() == 49);
    assert!(rt.frame() && !rt.frame());
    assert_eq!(rt.transient.drag.as_ref().unwrap().started_seq, 3);
    assert!(rt.presence.poll(1_000).unwrap().drag.is_some());
    feed(&mut rt, &mut state, &mut committed, DragEvent::Abort(AbortReason::Escape), 60);
    assert!(rt.transient.is_clear() && committed.is_empty());
    assert!(rt.presence.poll(2_000).unwrap().drag.is_none());
    feed(&mut rt, &mut state, &mut committed, DragEvent::Press { handle: "b1/length/end".into(), s: 4.0 }, 70);
    feed(&mut rt, &mut state, &mut committed, DragEvent::Move { s: 6.0, modifiers: Modifiers::NONE }, 71);
    assert!(rt.frame());
    feed(&mut rt, &mut state, &mut committed, DragEvent::Release, 80);
    assert_eq!(committed, vec![vec![FieldWrite::number("members[id=b1].length", 6.0)]]);
    assert!(rt.transient.is_clear());
    assert_eq!(rt.scheduler.poll(80), [EvalTier::T1, EvalTier::T2]);
}

#[test]
fn dragging_continues_locally_while_the_link_is_lost() {
    let handles = [handle()];
    let tool = HandleDragTool { handles: &handles, seq: 1 };
    let mut rt = LaneRuntime::new("ueli");
    rt.presence.set_link(Link::Lost { since_ms: 0 });
    let (state, effect) = tool.step(DragState::Armed { handle: "b1/length/end".into(), origin: 4.0 }, DragEvent::Move { s: 5.0, modifiers: Modifiers::NONE });
    rt.accept(effect, 1);
    assert!(rt.frame() && rt.transient.drag.is_some() && rt.presence.poll(2).is_none());
    let (_, effect) = tool.step(state, DragEvent::Release);
    assert!(rt.accept(effect, 3).is_some());
}

#[test]
fn viewport_actions_fold_into_the_view_lane_and_round_trip() {
    use semio_framework_value::{FromValue, ToValue};
    let fold = |events: Vec<LaneEvent>| events.into_iter().fold(NormTransient::default(), NormTransient::apply);
    let act = |action, text, number| view_event(action, text, number).expect("event");
    let lane = fold(vec![act(VIEW_ACTION_FIELD, Some("stress"), None), act(VIEW_ACTION_TRACK, Some("fire"), None), act(VIEW_ACTION_SEEK, None, Some(1800.0)), act(VIEW_ACTION_TOGGLE, None, None), act(VIEW_ACTION_SPEED, None, Some(2.0))]).view;
    assert_eq!((lane.active_field.as_deref(), lane.track.as_deref(), lane.value, lane.playing, lane.speed), (Some("stress"), Some("fire"), Some(1800.0), true, 2.0));
    let reselected = fold(vec![act(VIEW_ACTION_TRACK, Some("fire"), None), act(VIEW_ACTION_SEEK, None, Some(60.0)), act(VIEW_ACTION_TOGGLE, None, None), act(VIEW_ACTION_TRACK, Some("age"), None)]).view;
    assert_eq!((reselected.value, reselected.playing), (None, false));
    assert_eq!(fold(vec![act(VIEW_ACTION_FIELD, Some("x"), None), act(VIEW_ACTION_FIELD, Some(""), None)]).view.active_field, None);
    assert!(view_event(VIEW_ACTION_SEEK, None, None).is_none() && view_event(VIEW_ACTION_SPEED, None, Some(0.0)).is_none() && view_event("viewportFrame", None, None).is_none());
    let framed = fold(vec![act(VIEW_ACTION_FRAME, None, Some(1.0))]).view;
    assert!(framed.frame && !fold(vec![act(VIEW_ACTION_FRAME, None, Some(1.0)), act(VIEW_ACTION_FRAME, None, Some(1.0))]).view.frame, "a double tap frames the selection and the next one gives the whole scene back");
    let opened = fold(vec![act(VIEW_ACTION_MENU, None, Some(1.0))]).view;
    assert!(opened.menu && !fold(vec![act(VIEW_ACTION_MENU, None, Some(1.0)), act(VIEW_ACTION_MENU, None, Some(0.0))]).view.menu, "the Edit menu opens and closes by number");
    assert!(menu_closes(true, None) && menu_closes(true, Some(&LaneEvent::ViewCheck(Some(1)))) && menu_closes(true, Some(&LaneEvent::DragCleared)), "any other command (copy, delete, focus a check, a nudge) ends an open menu");
    assert!(!menu_closes(true, Some(&LaneEvent::ViewMenu(true))) && !menu_closes(true, Some(&LaneEvent::ViewMenu(false))) && !menu_closes(false, None), "the menu's own open and close are left alone, and a closed menu has nothing to end");
    let transient = fold(vec![act(VIEW_ACTION_TRACK, Some("fire"), None)]);
    assert_eq!(NormTransient::from_value(transient.to_value()).expect("decode"), transient);
    assert!(NormTransient::default().apply(act(VIEW_ACTION_TOGGLE, None, None)).is_clear());
}
