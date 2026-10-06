/// 🌊️ One dispatch of a streamed `translateSelection` against the gesture its window holds.
fn drive(held: Option<&GestureState<Puzzle2dMutation>>, phase: GesturePhase, request: Option<SelectToolRequest>, revision: &str) -> GestureDrive<GestureState<Puzzle2dMutation>, Puzzle2dMutation> {
    drive_chart_gesture::<select_tool::SelectTool>(held, "translateSelection", phase, request, "seed-1", revision).expect("the select tool drives")
}

#[test]
fn a_streamed_gesture_spans_dispatches_and_commits_one_transaction() {
    let base = board();
    let opened = drive(None, GesturePhase::Stream, Some(request(&base, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), -400.0, 0.0)])), "rev-1");
    assert!(opened.committed.is_none() && !opened.continued, "the first tick opens a gesture and publishes nothing");
    let first = opened.next.flatten().expect("an open gesture is the window's next gesture");
    assert_eq!((first.states.as_slice(), first.verb.as_str(), first.base_revision.as_str(), &first.context), (&["root".to_string(), "streaming".to_string()][..], "translateSelection", "rev-1", &semio_framework_value::DslValue::Bool(true)));
    assert_eq!(first.entries, vec![(PUZZLE2D_SELECT_TOOL_LEAF_KEY.to_string(), Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["right"]), dx: -400.0, dy: 0.0 }))], "a stream holds ONE net leaf");
    let ticked = drive(Some(&first), GesturePhase::Stream, Some(request(&base, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), -548.0, 0.0)])), "rev-1");
    assert!(ticked.committed.is_none() && ticked.continued, "a second tick continues the held gesture");
    let second = ticked.next.flatten().expect("the gesture is still open");
    assert_eq!(second.transaction, first.transaction, "every tick of one gesture joins ONE transaction");
    assert_eq!(second.entries, vec![(PUZZLE2D_SELECT_TOOL_LEAF_KEY.to_string(), Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["right"]), dx: -948.0, dy: 0.0 }))], "the ticks add up into the one leaf");
    let finished = drive(Some(&second), GesturePhase::Commit, Some(request(&base, Vec::new())), "rev-1");
    let (transaction, mutations) = finished.committed.expect("the commit publishes the gesture");
    assert_eq!(transaction, first.transaction, "the commit publishes the ref minted at the first tick");
    assert_eq!(transaction.tool, "s.puzzle.puzzle2d@1/*#editor#translateSelection");
    assert_eq!(mutations, puzzle2d_selection_yields(&base, &[Puzzle2dSelectionRecord::drag(ids(&["right"]), -948.0, 0.0)], 12.0).into_iter().map(|(_, mutation)| mutation).collect::<Vec<_>>(), "the net leaf plus the connection its drop lands");
    assert_eq!(finished.next, Some(None), "a committed gesture leaves the window's slot empty");
}

#[test]
fn a_host_abort_or_a_moved_base_mid_gesture_leaves_zero_trace() {
    let base = board();
    let open = drive(None, GesturePhase::Stream, Some(request(&base, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), -400.0, 0.0)])), "rev-1").next.flatten().expect("open");
    for reason in [ToolAbortReason::Blur, ToolAbortReason::CaptureLost, ToolAbortReason::BaseMoved, ToolAbortReason::Frozen, ToolAbortReason::Retired] {
        let aborted = drive(Some(&open), GesturePhase::Abort(reason), None, "rev-1");
        assert!(aborted.committed.is_none() && aborted.next == Some(None), "{reason:?} drops the open gesture and publishes nothing");
    }
    let moved = drive(Some(&open), GesturePhase::Commit, Some(request(&base, Vec::new())), "rev-2");
    assert!(moved.committed.is_none() && moved.next == Some(None), "a commit on a moved base commits nothing and drops the gesture");
    let resting = drive(None, GesturePhase::Abort(ToolAbortReason::Blur), None, "rev-1");
    assert!(resting.committed.is_none() && resting.next.is_none(), "aborting a resting tool is a no-op");
}

#[test]
fn a_tampered_or_resting_gesture_never_resumes() {
    let base = board();
    let open = drive(None, GesturePhase::Stream, Some(request(&base, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), -400.0, 0.0)])), "rev-1").next.flatten().expect("open");
    let resting = GestureState { states: vec!["root".into(), "idle".into()], ..open.clone() };
    assert!(matches!(<ChartGesture<select_tool::SelectTool> as GestureTool>::resume(&resting), Err(ToolRefusal::Unclosed)), "a resting chart with an open transaction would let the next gesture join it");
    let unknown = GestureState { states: vec!["root".into(), "dragging".into()], ..open.clone() };
    assert!(<ChartGesture<select_tool::SelectTool> as GestureTool>::resume(&unknown).is_err(), "a configuration the chart does not know is refused");
    let dropped = drive(Some(&unknown), GesturePhase::Commit, Some(request(&base, Vec::new())), "rev-1");
    assert!(dropped.committed.is_none() && dropped.next == Some(None), "a gesture the tool cannot restore is dropped with zero trace");
}

/// 🎚️ LAW (F21): a committed leaf states its inputs at the precision their schema declares, whatever a host measured —
/// an `f32` drag of 60 recorded as 59.99996 is the row "by (60, 0)", and a motion that rounds to the identity is no
/// edit at all.
#[test]
fn a_committed_leaf_states_its_inputs_at_their_declared_precision() {
    let base = board();
    let (_, dragged) = commit("seed-1", &base, 0.0, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), 59.99996, -0.004)]).expect("the drag commits");
    assert_eq!(dragged, vec![Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["right"]), dx: 60.0, dy: 0.0 })]);
    let scale = Puzzle2dSelectionRecord { targets: ids(&["right"]), motion: Puzzle2dSelectionMotion::Scale { pivot_x: 10.004999, pivot_y: -0.001, factor: 1.4999999 }, proximity: Vec::new(), connect: false };
    let (_, scaled) = commit("seed-1", &base, 0.0, vec![scale]).expect("the scaling commits");
    assert_eq!(Puzzle2dSelectionRecord::from_leaf(&scaled[0], false).map(|record| record.motion), Some(Puzzle2dSelectionMotion::Scale { pivot_x: 10.0, pivot_y: 0.0, factor: 1.5 }));
    assert_eq!(commit("seed-1", &base, 0.0, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), 0.004, -0.004)]), None, "a drag that rounds to nothing leaves zero trace");
    let exact = Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["right"]), dx: 12.25, dy: -7.5 });
    assert_eq!(puzzle2d_declared_precision(exact.clone()), exact, "an input already at its precision is untouched");
}
