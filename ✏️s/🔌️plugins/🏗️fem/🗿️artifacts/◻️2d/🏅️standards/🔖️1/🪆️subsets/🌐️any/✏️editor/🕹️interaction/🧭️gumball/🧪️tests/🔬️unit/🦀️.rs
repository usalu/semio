use super::*;
use crate::editor::fem2d::transient::FemGumballTransient;
use crate::editor::fem2d::interaction::canvas_gesture::FEM2D_UTILITY_TRANSFORM;
use store::ArtifactDsl;

type Fem2dSnapshot = crate::Fem2dSnapshot;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("demo document parses")
}

#[test]
fn gumball_meta_layer_emitted_when_transform_utility_and_selection() {
    let doc = demo();
    let meta = fem2d_gumball_meta_layer(&doc, &["n1".into()], FEM2D_UTILITY_TRANSFORM, Some("w")).expect("meta");
    assert_eq!(meta.get("role").and_then(|value| value.as_str()), Some("meta"));
    assert!(meta.get("gumball").is_some());
    assert_eq!(meta.pointer("/gumball/config/moveAxes").and_then(|value| value.as_bool()), Some(true));
}

#[test]
fn gumball_meta_absent_without_selection() {
    let doc = demo();
    assert!(fem2d_gumball_meta_layer(&doc, &[], FEM2D_UTILITY_TRANSFORM, None).is_none());
}

/// 🎯️ A translate tick is the relative `move-selection` leaf over the selection's literal geometry, pivoted on its
/// centroid, and moves exactly that geometry by the offset.
#[test]
fn a_tick_is_the_relative_leaf_over_the_selected_geometry() {
    let doc = demo();
    let n1 = doc.nodes.iter().find(|node| node.id == "n1").expect("n1").clone();
    let tick = fem2d_gumball_tick(&doc, &["n1".into()], Fem2dGumballMotion::Translate { dx: 1.0, dy: 2.0 }).expect("a node selection moves geometry");
    assert_eq!((tick.node_ids.clone(), tick.region_ids.clone(), tick.pivot_x, tick.pivot_y, tick.dx, tick.dy), (vec!["n1".to_string()], Vec::new(), n1.x, n1.y, 1.0, 2.0));
    let mut moved = doc.clone();
    crate::standards::v1::subsets::any::schema::mutations::apply_fem2d_mutation(&mut moved, &Fem2dMutation::MoveSelection(tick)).expect("the tick applies");
    let after = moved.nodes.iter().find(|node| node.id == "n1").expect("n1");
    assert_eq!((after.x, after.y), (n1.x + 1.0, n1.y + 2.0));
    assert!(fem2d_gumball_tick(&doc, &[], Fem2dGumballMotion::Rotate { angle: 0.5 }).is_none(), "an empty selection moves nothing");
}

/// ➕️ Ticks of one gesture compose into ONE net leaf: offsets add, angles add, factors multiply; an identity tick
/// changes nothing and a different kind of motion does not compose.
#[test]
fn ticks_compose_into_one_net_leaf() {
    let doc = demo();
    let tick = |motion| fem2d_gumball_tick(&doc, &["n1".into()], motion).expect("tick");
    let moved = fem2d_gumball_then(&tick(Fem2dGumballMotion::Translate { dx: 0.5, dy: 0.0 }), &tick(Fem2dGumballMotion::Translate { dx: 0.25, dy: 1.0 })).expect("translations add");
    assert_eq!((moved.dx, moved.dy), (0.75, 1.0));
    let turned = fem2d_gumball_then(&tick(Fem2dGumballMotion::Rotate { angle: 0.5 }), &tick(Fem2dGumballMotion::Rotate { angle: 0.25 })).expect("rotations add");
    assert_eq!(turned.angle, 0.75);
    let scaled = fem2d_gumball_then(&tick(Fem2dGumballMotion::Scale { sx: 2.0, sy: 1.0 }), &tick(Fem2dGumballMotion::Scale { sx: 1.5, sy: 0.5 })).expect("factors multiply");
    assert_eq!((scaled.sx, scaled.sy), (3.0, 0.5));
    assert_eq!(fem2d_gumball_then(&moved, &tick(Fem2dGumballMotion::Translate { dx: 0.0, dy: 0.0 })), Some(moved.clone()), "an identity tick changes nothing");
    assert!(fem2d_gumball_then(&moved, &tick(Fem2dGumballMotion::Rotate { angle: 0.5 })).is_none(), "a rotation never folds into a drag");
}

/// 🛠️ Drives the tool the way the retained route does, on window `window` of `transient`.
fn drive(transient: &FemGumballTransient, window: &str, phase: GesturePhase, tick: Option<MoveSelection>, base: &str) -> crate::editor::fem2d::transient::FemGumballDrive<Fem2dMutation> {
    crate::editor::fem2d::transient::fem_gumball_drive::<Fem2dGumballTool>(transient, window, "translateSelection", phase, tick, "seed", base)
}

/// 💾️ LAW: a one-shot dispatch is ONE committed transaction holding the one leaf, and leaves no transient behind.
#[test]
fn a_one_shot_tick_commits_one_transaction() {
    let doc = demo();
    let tick = fem2d_gumball_tick(&doc, &["n1".into()], Fem2dGumballMotion::Translate { dx: 1.0, dy: 0.0 });
    let done = drive(&FemGumballTransient::default(), "w", GesturePhase::Once, tick.clone(), "base");
    let (reference, mutations) = done.committed.expect("committed");
    assert!(reference.id.starts_with("tx-") && reference.tool == format!("{FEM2D_EDITOR_APP_ID}#translateSelection"));
    assert_eq!(mutations, vec![Fem2dMutation::MoveSelection(tick.expect("tick"))]);
    assert!(done.transient.is_none(), "a one-shot persists nothing");
}

/// 🌊️ LAW: streamed ticks accumulate in ONE open transaction persisted for their window, and the commit publishes
/// the NET leaf once under the ref minted at the first tick.
#[test]
fn streamed_ticks_commit_the_net_leaf_once() {
    let doc = demo();
    let tick = |dx| fem2d_gumball_tick(&doc, &["n1".into()], Fem2dGumballMotion::Translate { dx, dy: 0.0 });
    let first = drive(&FemGumballTransient::default(), "w", GesturePhase::Stream, tick(0.5), "base");
    assert!(first.committed.is_none(), "a stream tick commits nothing");
    let open = first.transient.expect("the first tick opens the window's gesture");
    let minted = open.gestures["w"].transaction.clone();
    let second = drive(&open, "w", GesturePhase::Stream, tick(0.25), "base");
    let open = second.transient.expect("the second tick advances it");
    assert_eq!(open.gestures["w"].transaction, minted, "every tick rides the same transaction");
    let preview = open.preview::<Fem2dSnapshot, Fem2dMutation>(&doc).expect("an open gesture previews");
    let start = doc.nodes.iter().find(|node| node.id == "n1").expect("n1").x;
    assert_eq!(preview.nodes.iter().find(|node| node.id == "n1").expect("n1").x, start + 0.75, "the preview is the net transform");
    let done = drive(&open, "w", GesturePhase::Commit, tick(0.0), "base");
    let (reference, mutations) = done.committed.expect("the commit publishes");
    assert_eq!(reference, minted);
    let [Fem2dMutation::MoveSelection(net)] = mutations.as_slice() else { panic!("one net leaf: {mutations:?}") };
    assert_eq!(net.dx, 0.75);
    assert!(done.transient.expect("the commit clears the gesture").gestures.is_empty());
}

/// 🧯️ LAW: a host abort, a base that moved under the gesture and an interrupting one-shot each drop the open gesture
/// with zero trace; a sibling window's gesture is untouched.
#[test]
fn host_aborts_leave_zero_trace_and_keep_sibling_gestures() {
    let doc = demo();
    let tick = || fem2d_gumball_tick(&doc, &["n1".into()], Fem2dGumballMotion::Translate { dx: 0.5, dy: 0.0 });
    let left = drive(&FemGumballTransient::default(), "left", GesturePhase::Stream, tick(), "base").transient.expect("left opens");
    let both = drive(&left, "right", GesturePhase::Stream, tick(), "base").transient.expect("right opens");
    assert_eq!(both.gestures.len(), 2, "two windows hold two gestures");
    let aborted = drive(&both, "left", GesturePhase::Abort(ToolAbortReason::Blur), None, "base");
    assert!(aborted.committed.is_none());
    let rest = aborted.transient.expect("the abort clears the window's gesture");
    assert_eq!(rest.gestures.keys().collect::<Vec<_>>(), vec!["right"], "only the owner's gesture is dropped");
    let moved = drive(&rest, "right", GesturePhase::Commit, tick(), "moved");
    assert!(moved.committed.is_none(), "a commit on a moved base is dropped with its gesture");
    assert!(moved.transient.expect("baseMoved clears it").gestures.is_empty());
    let open = drive(&FemGumballTransient::default(), "w", GesturePhase::Stream, tick(), "base").transient.expect("opens");
    let interrupted = drive(&open, "w", GesturePhase::Once, tick(), "base");
    let (_, mutations) = interrupted.committed.expect("the one-shot commits its own transaction");
    let [Fem2dMutation::MoveSelection(only)] = mutations.as_slice() else { panic!("one leaf") };
    assert_eq!(only.dx, 0.5, "the interrupted stream contributes nothing");
    assert!(interrupted.transient.expect("captureLost clears the stream").gestures.is_empty());
}

/// ⏪️ LAW: a gumball move edited in history replays its downstream: the preview base is the state right before the
/// move, and the Report replay re-applies the downstream relative scaling and turn onto the edited move — exactly the
/// fresh fold of the edited log; overwrite commits it.
#[semio_framework_async_macros::async_test]
async fn a_gumball_move_edited_in_history_replays_its_downstream() {
    use protocol::OpBinary;
    let base = demo();
    let tick = |motion| Fem2dMutation::MoveSelection(fem2d_gumball_tick(&base, &["n1".to_string()], motion).expect("the selection moves geometry"));
    let log = [tick(Fem2dGumballMotion::Translate { dx: 1.0, dy: 0.0 }), tick(Fem2dGumballMotion::Scale { sx: 2.0, sy: 1.0 }), tick(Fem2dGumballMotion::Rotate { angle: 0.5 })];
    let mut store = store::ArtifactStore::<Fem2dSnapshot, Fem2dMutation>::new(store::create_document_envelope::<Fem2dSnapshot, Fem2dMutation>(crate::FEM_2D_SCHEMA, "gumball-time-travel", base.clone(), None)).await.expect("the store opens");
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<Fem2dSnapshot, Fem2dMutation>());
    for mutation in &log {
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], description: None, transaction: None }).await.expect("the edit applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let edited = tick(Fem2dGumballMotion::Translate { dx: -0.5, dy: 3.0 });
    let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: crate::FEM_2D_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    assert_eq!(store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref(), &base, "the preview base is the state right before the edited move");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited move");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "a re-offset move never blocks finalizing");
    let fold = |mutations: &[Fem2dMutation]| {
        let mut snapshot = base.clone();
        for mutation in mutations {
            crate::standards::v1::subsets::any::schema::mutations::apply_fem2d_mutation(&mut snapshot, mutation).expect("the log folds");
        }
        snapshot
    };
    let fresh = fold(&[edited, log[1].clone(), log[2].clone()]);
    assert_ne!(fresh, fold(&log), "the edit changes the outcome");
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
    store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
    assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
    let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<Fem2dSnapshot, Fem2dMutation>();
    for _ in 0..4_096 {
        if disposer.terminal_is_empty(&store) {
            break;
        }
        disposer.close_step(&mut store, 1, 1 << 20).expect("the store retires");
    }
    assert!(disposer.terminal_is_empty(&store), "the standalone store retires to its terminal-empty shell");
}
