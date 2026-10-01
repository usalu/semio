//! 🛠️ Laws of the transform tool itself, driven dispatch by dispatch exactly as the retained step threads it: a
//! one-shot is ONE transaction, a streamed gesture accumulates ONE net leaf in its open transaction until its commit,
//! every host abort leaves zero trace, and the persisted state round-trips through the window transient's value form.

use super::*;

fn document() -> LayoutSnapshot {
    crate::standards::v1::subsets::any::schema::default_document()
}

fn drag(dx: f64, dy: f64) -> LayoutFrameRecord {
    LayoutFrameRecord { page_id: "page-1".into(), targets: vec!["frame-1".into()], motion: LayoutFrameMotion::Drag { dx, dy } }
}

fn step(phase: LayoutTransformPhase, record: Option<LayoutFrameRecord>, open: Option<&LayoutTransformToolState>, revision: &str) -> LayoutTransformDispatch {
    layout_transform_dispatch("translateSelection", phase, record, &document(), open, "seed-1", revision, Some("locked"))
}

fn opened(dispatch: LayoutTransformDispatch) -> LayoutTransformToolState {
    *dispatch.tool.expect("the transient changed").expect("the gesture is open")
}

#[test]
fn transform_utility_id_and_options() {
    assert_eq!(UTILITY_ID, "transform");
    assert_eq!(definition().id, UTILITY_ID);
    let options = options();
    assert!(options.move_axes && options.rotate && options.scale_axes && options.scale_uniform);
}

/// ➕️ Ticks compose into one net transform: offsets and angles add, factors multiply — only about one pivot and over
/// the same targets.
#[test]
fn ticks_compose_into_one_net_transform() {
    assert_eq!(drag(1.0, 2.0).then(&drag(3.0, -1.0)), Some(drag(4.0, 1.0)));
    let turn = |angle: f64, pivot: f64| LayoutFrameRecord { page_id: "page-1".into(), targets: vec!["frame-1".into()], motion: LayoutFrameMotion::Rotate { pivot_x: pivot, pivot_y: pivot, angle } };
    assert_eq!(turn(0.25, 5.0).then(&turn(0.5, 5.0)).map(|record| record.motion), Some(LayoutFrameMotion::Rotate { pivot_x: 5.0, pivot_y: 5.0, angle: 0.75 }));
    assert_eq!(turn(0.25, 5.0).then(&turn(0.5, 6.0)), None, "another pivot is another gesture");
    let scaling = |sx: f64| LayoutFrameRecord { page_id: "page-1".into(), targets: vec!["frame-1".into()], motion: LayoutFrameMotion::Scale { pivot_x: 0.0, pivot_y: 0.0, sx, sy: 1.0 } };
    assert_eq!(scaling(2.0).then(&scaling(1.5)).map(|record| record.motion), Some(LayoutFrameMotion::Scale { pivot_x: 0.0, pivot_y: 0.0, sx: 3.0, sy: 1.0 }));
    assert_eq!(drag(1.0, 0.0).then(&scaling(2.0)), None, "another kind is another gesture");
}

/// ✋️ A one-shot is ONE committed transaction: the parametric leaf, the tool `<appId>#<verb>`, a `tx-` id minted from
/// the admission's seed, no coalesce key, and no window state.
#[test]
fn a_one_shot_is_one_transaction() {
    let dispatch = step(LayoutTransformPhase::Once, Some(drag(5.0, -2.0)), None, "rev-1");
    assert!(dispatch.tool.is_none(), "a one-shot leaves the window transient alone");
    let transaction = dispatch.emit.transaction.clone().expect("the commit carries its transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.layout.layout@1/*#editor#translateSelection", "{transaction:?}");
    assert_eq!(dispatch.emit.artifact_mutations, vec![drag(5.0, -2.0).mutation()]);
    assert_eq!(dispatch.emit.coalesce_key, None);
}

/// 🌊️ A streamed gesture: every tick lands in ONE open transaction the window persists — no edit — and the commit
/// publishes the net leaf under the ref minted at the first tick, clearing the window state.
#[test]
fn a_stream_publishes_nothing_until_its_commit_publishes_one_transaction() {
    let first = step(LayoutTransformPhase::Stream, Some(drag(10.0, 0.0)), None, "rev-1");
    assert!(first.emit.artifact_mutations.is_empty() && first.emit.transaction.is_none(), "a tick is no edit");
    let open = opened(first);
    let second = opened(step(LayoutTransformPhase::Stream, Some(drag(15.0, 5.0)), Some(&open), "rev-1"));
    assert_eq!(second.transaction, open.transaction, "every tick rides the one open transaction");
    assert_eq!(second.entries.len(), 1, "the open transaction holds ONE net leaf");
    let commit = step(LayoutTransformPhase::Commit, Some(drag(0.0, 0.0)), Some(&second), "rev-1");
    assert_eq!(commit.emit.artifact_mutations, vec![drag(25.0, 5.0).mutation()], "the commit publishes the net leaf");
    assert_eq!(commit.emit.transaction, Some(open.transaction), "under the ref minted when the gesture opened");
    assert_eq!(commit.tool, Some(None), "the commit clears the persisted gesture");
}

/// 🧯️ Every host abort of an open gesture — and a document that moved under it — publishes no edit and clears it; a
/// commit, an abort or a stray tail with no open gesture leaves zero trace.
#[test]
fn every_host_abort_leaves_zero_trace() {
    let open = opened(step(LayoutTransformPhase::Stream, Some(drag(10.0, 0.0)), None, "rev-1"));
    for reason in ToolAbortReason::ALL {
        let aborted = step(LayoutTransformPhase::Abort(reason), None, Some(&open), "rev-1");
        assert!(aborted.emit.artifact_mutations.is_empty() && aborted.emit.transaction.is_none(), "{reason:?}: no edit");
        assert_eq!(aborted.tool, Some(None), "{reason:?}: the gesture is cleared");
    }
    let moved = step(LayoutTransformPhase::Commit, Some(drag(1.0, 0.0)), Some(&open), "rev-2");
    assert!(moved.emit.artifact_mutations.is_empty(), "a gesture on a moved base commits nothing");
    assert_eq!(moved.tool, Some(None), "baseMoved clears the gesture");
    for phase in [LayoutTransformPhase::Commit, LayoutTransformPhase::Abort(ToolAbortReason::Blur)] {
        let stray = step(phase, Some(drag(3.0, 0.0)), None, "rev-1");
        assert!(stray.emit.artifact_mutations.is_empty() && stray.tool.is_none(), "{phase:?} at rest leaves zero trace");
    }
}

/// 🔀️ A one-shot or another verb interrupting an open gesture aborts it (`captureLost`): only the interrupting
/// transform counts, under a fresh ref minted from its own admission's seed.
#[test]
fn an_interruption_aborts_the_open_gesture() {
    let open = opened(step(LayoutTransformPhase::Stream, Some(drag(10.0, 0.0)), None, "rev-1"));
    let once = layout_transform_dispatch("translateSelection", LayoutTransformPhase::Once, Some(drag(1.0, 0.0)), &document(), Some(&open), "seed-2", "rev-1", None);
    assert_eq!(once.emit.artifact_mutations, vec![drag(1.0, 0.0).mutation()], "the interrupted gesture contributed nothing");
    assert_ne!(once.emit.transaction.as_ref().map(|transaction| &transaction.id), Some(&open.transaction.id));
    assert_eq!(once.tool, Some(None));
    let turn = LayoutFrameRecord { page_id: "page-1".into(), targets: vec!["frame-1".into()], motion: LayoutFrameMotion::Rotate { pivot_x: 0.0, pivot_y: 0.0, angle: 0.5 } };
    let rotated = layout_transform_dispatch("rotateSelection", LayoutTransformPhase::Stream, Some(turn), &document(), Some(&open), "seed-3", "rev-1", None);
    let reopened = opened(rotated);
    assert_eq!((reopened.verb.as_str(), reopened.entries.len()), ("rotateSelection", 1), "another verb's tick opens its own gesture");
    assert_ne!(reopened.transaction.id, open.transaction.id);
}

/// 🔒️ A request whose every target is locked raises the one localized notice and leaves zero trace.
#[test]
fn a_fully_locked_request_raises_one_notice() {
    let mut base = document();
    if let crate::Frame::Rect { locked, .. } = base.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").expect("demo rect") {
        *locked = Some(true);
    }
    let dispatch = layout_transform_dispatch("translateSelection", LayoutTransformPhase::Once, Some(drag(5.0, 0.0)), &base, None, "seed-1", "rev-1", Some("locked"));
    assert!(dispatch.emit.artifact_mutations.is_empty());
    assert!(matches!(dispatch.emit.effects.as_slice(), [semio_framework::kernel::Effect::Notify { message }] if message == "locked"), "one localized notice");
}

/// 💾️ The persisted gesture survives the window transient's value form and resumes as the same open gesture; the
/// window previews its provisional leaf, never the document.
#[test]
fn the_persisted_gesture_round_trips_and_previews() {
    let open = opened(step(LayoutTransformPhase::Stream, Some(drag(10.0, 4.0)), None, "rev-1"));
    let restored: LayoutTransformToolState = dsl::FromValue::from_value(dsl::ToValue::to_value(&open)).expect("the tool state decodes from its value form");
    assert_eq!(restored, open);
    let tool = LayoutTransformTool::resume(&restored).expect("the gesture resumes");
    assert!(!tool.at_rest(), "a resumed gesture is still streaming");
    let base = document();
    let preview = layout_transform_tool_preview(&base, &restored);
    let origin = |document: &LayoutSnapshot| document.pages[0].frames.iter().find(|frame| frame.id() == "frame-1").map(|frame| (frame.bounds().x, frame.bounds().y)).expect("demo rect");
    let (x, y) = origin(&base);
    assert_eq!(origin(&preview), (x + 10.0, y + 4.0), "the preview applies the provisional leaf");
    assert_eq!(origin(&document()), (x, y), "the document never moves while the gesture is open");
}
