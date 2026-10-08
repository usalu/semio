use super::*;
use crate::FemAxis;
use semio_s_artifact_fem_2d::editor::fem2d::transient::{fem_gumball_drive, FemGumballDrive, FemGumballTransient};

fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot()
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn moved(doc: &Fem3dSnapshot, tick: MoveSelection) -> Fem3dSnapshot {
    let mut snapshot = doc.clone();
    crate::central_apply::apply_fem3d_mutation(&mut snapshot, &Fem3dMutation::MoveSelection(tick)).expect("the tick applies");
    snapshot
}

fn node(doc: &Fem3dSnapshot, id: &str) -> [f64; 3] {
    let node = doc.nodes.iter().find(|node| node.id == id).expect("node");
    [node.x, node.y, node.z]
}

#[test]
fn targets_resolve_members_supports_and_loads_to_their_geometry() {
    let doc = demo();
    let targets = fem3d_transform_targets(&doc, &ids(&["fb1_0", "s_20", "l1", "steel"]));
    assert_eq!(targets.node_ids, ids(&["n00_l1", "n20_g", "n20_l1"]), "nodes come once each, in document order");
    assert_eq!(targets.solid_ids, ids(&["sol1"]));
    assert!(fem3d_transform_targets(&doc, &ids(&["steel"])).is_empty());
}

/// 🎯️ A tick is the relative `move-selection` leaf over the selection's literal geometry, pivoted on its centroid,
/// and moves exactly that geometry: a drag by the offset, a quarter turn about the pivot, a scaling about it.
#[test]
fn a_tick_is_the_relative_leaf_over_the_selected_geometry() {
    let doc = demo();
    let drag = fem3d_gumball_tick(&doc, &ids(&["n20_l1", "sol1"]), Fem3dGumballMotion::Translate { dx: 1.0, dy: 2.0, dz: 3.0 }).expect("a node and a solid move geometry");
    assert_eq!((drag.node_ids.clone(), drag.solid_ids.clone(), [drag.dx, drag.dy, drag.dz]), (ids(&["n20_l1"]), ids(&["sol1"]), [1.0, 2.0, 3.0]));
    let dragged = moved(&doc, drag);
    assert_eq!(node(&dragged, "n20_l1"), [9.0, 2.0, 5.8]);
    let slab = dragged.solids.iter().find(|solid| solid.id == "sol1").expect("sol1");
    assert_eq!(slab.outline[0], [11.0, 2.0]);
    assert!((slab.base_z - doc.solids[0].base_z - 3.0).abs() < 1e-12 && slab.height == doc.solids[0].height, "the slab rides the drag: {slab:?}");
    let quarter = std::f64::consts::FRAC_PI_2;
    let turn = fem3d_gumball_tick(&doc, &ids(&["n20_g", "n00_g"]), Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: quarter }).expect("turn");
    assert_eq!([turn.pivot_x, turn.pivot_y, turn.pivot_z], [4.0, 0.0, 0.0], "the pivot is the selection's centroid");
    let turned = node(&moved(&doc, turn), "n20_g");
    assert!((turned[0] - 4.0).abs() < 1e-9 && (turned[1] - 4.0).abs() < 1e-9, "n20_g swings a quarter turn about (4, 0, 0): {turned:?}");
    let stretch = fem3d_gumball_tick(&doc, &ids(&["n00_g", "n20_g"]), Fem3dGumballMotion::Scale { sx: 2.0, sy: 1.0, sz: 1.0 }).expect("stretch");
    assert_eq!(node(&moved(&doc, stretch), "n20_g"), [12.0, 0.0, 0.0]);
    let mut sideways = demo();
    sideways.solids[0].axis = FemAxis::Y;
    let deeper = fem3d_gumball_tick(&sideways, &ids(&["sol1"]), Fem3dGumballMotion::Scale { sx: 1.0, sy: 2.0, sz: 1.0 }).expect("deeper");
    let extruded = moved(&sideways, deeper);
    assert!((extruded.solids[0].height - 2.0 * sideways.solids[0].height).abs() < 1e-9, "a Y extrusion doubles along Y: {}", extruded.solids[0].height);
    assert!(fem3d_gumball_tick(&doc, &ids(&["steel"]), Fem3dGumballMotion::Translate { dx: 1.0, dy: 0.0, dz: 0.0 }).is_none(), "a material moves no geometry");
}

/// ➕️ Ticks of one gesture compose into ONE net leaf: offsets add, angles about one axis add, factors multiply; an
/// identity tick changes nothing, and another axis or another kind of motion does not compose.
#[test]
fn ticks_compose_into_one_net_leaf() {
    let doc = demo();
    let tick = |motion| fem3d_gumball_tick(&doc, &ids(&["n20_l1", "n00_l1"]), motion).expect("tick");
    let drag = fem3d_gumball_then(&tick(Fem3dGumballMotion::Translate { dx: 0.5, dy: 0.0, dz: 1.0 }), &tick(Fem3dGumballMotion::Translate { dx: 0.25, dy: 1.0, dz: 0.0 })).expect("translations add");
    assert_eq!([drag.dx, drag.dy, drag.dz], [0.75, 1.0, 1.0]);
    let about_z = |angle| tick(Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 2.0], angle });
    let turn = fem3d_gumball_then(&about_z(0.5), &about_z(0.25)).expect("rotations about one axis add");
    assert_eq!(turn.angle, 0.75);
    assert!(fem3d_gumball_then(&turn, &tick(Fem3dGumballMotion::Rotate { axis: [1.0, 0.0, 0.0], angle: 0.25 })).is_none(), "another axis does not compose");
    let stretch = fem3d_gumball_then(&tick(Fem3dGumballMotion::Scale { sx: 2.0, sy: 1.0, sz: 1.0 }), &tick(Fem3dGumballMotion::Scale { sx: 1.5, sy: 0.5, sz: 2.0 })).expect("factors multiply");
    assert_eq!([stretch.sx, stretch.sy, stretch.sz], [3.0, 0.5, 2.0]);
    assert_eq!(fem3d_gumball_then(&drag, &tick(Fem3dGumballMotion::Translate { dx: 0.0, dy: 0.0, dz: 0.0 })), Some(drag.clone()), "an identity tick changes nothing");
    assert!(fem3d_gumball_then(&drag, &about_z(0.5)).is_none(), "a rotation never folds into a drag");
}

/// 🛠️ Drives the tool the way the retained route does, on window `window` of `transient`.
fn drive(transient: &FemGumballTransient, window: &str, verb: &str, phase: GesturePhase, tick: Option<MoveSelection>, base: &str) -> FemGumballDrive<Fem3dMutation> {
    fem_gumball_drive::<Fem3dGumballTool>(transient, window, verb, phase, tick, "seed", base).expect("the gumball tool accepts the dispatch")
}

/// 💾️ LAW: a one-shot dispatch is ONE committed transaction holding the one leaf and leaves no transient behind; a
/// tick that moves nothing or that the leaf refuses (a rotation about the zero axis) commits nothing.
#[test]
fn a_one_shot_tick_commits_one_transaction() {
    let doc = demo();
    let tick = fem3d_gumball_tick(&doc, &ids(&["n20_l1"]), Fem3dGumballMotion::Translate { dx: 1.0, dy: 0.0, dz: 0.0 });
    let done = drive(&FemGumballTransient::default(), "w", "translateSelection", GesturePhase::Once, tick.clone(), "base");
    let (reference, mutations) = done.committed.expect("committed");
    assert!(reference.id.starts_with("tx-") && reference.tool == format!("{FEM3D_EDITOR_APP_ID}#translateSelection"), "{reference:?}");
    assert_eq!(mutations, vec![Fem3dMutation::MoveSelection(tick.expect("tick"))]);
    assert!(done.transient.is_none(), "a one-shot persists nothing");
    let idle = fem3d_gumball_tick(&doc, &ids(&["n20_l1"]), Fem3dGumballMotion::Translate { dx: 0.0, dy: 0.0, dz: 0.0 });
    assert!(drive(&FemGumballTransient::default(), "w", "translateSelection", GesturePhase::Once, idle, "base").committed.is_none(), "an identity tick commits nothing");
    let axisless = fem3d_gumball_tick(&doc, &ids(&["n20_l1"]), Fem3dGumballMotion::Rotate { axis: [0.0; 3], angle: 0.5 });
    assert!(drive(&FemGumballTransient::default(), "w", "rotateSelection", GesturePhase::Once, axisless, "base").committed.is_none(), "a rotation about the zero axis commits nothing");
}

/// 🌊️ LAW: streamed ticks accumulate in ONE open transaction persisted for their window, previewed as the net
/// transform, and the commit publishes the NET leaf once under the ref minted at the first tick.
#[test]
fn streamed_ticks_commit_the_net_leaf_once() {
    let doc = demo();
    let tick = |angle| fem3d_gumball_tick(&doc, &ids(&["n20_g", "n00_g"]), Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle });
    let first = drive(&FemGumballTransient::default(), "w", "rotateSelection", GesturePhase::Stream, tick(0.5), "base");
    assert!(first.committed.is_none(), "a stream tick commits nothing");
    let open = first.transient.expect("the first tick opens the window's gesture");
    let minted = open.gestures["w"].transaction.clone();
    let second = drive(&open, "w", "rotateSelection", GesturePhase::Stream, tick(std::f64::consts::FRAC_PI_2 - 0.5), "base");
    let open = second.transient.expect("the second tick advances it");
    assert_eq!(open.gestures["w"].transaction, minted, "every tick rides the same transaction");
    let preview = open.preview::<Fem3dSnapshot, Fem3dMutation>(&doc).expect("an open gesture previews");
    let swung = node(&preview, "n20_g");
    assert!((swung[0] - 4.0).abs() < 1e-9 && (swung[1] - 4.0).abs() < 1e-9, "the preview is the net quarter turn: {swung:?}");
    let done = drive(&open, "w", "rotateSelection", GesturePhase::Commit, tick(0.0), "base");
    let (reference, mutations) = done.committed.expect("the commit publishes");
    assert_eq!(reference, minted);
    let [Fem3dMutation::MoveSelection(net)] = mutations.as_slice() else { panic!("one net leaf: {mutations:?}") };
    assert!((net.angle - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    assert!(done.transient.expect("the commit clears the gesture").gestures.is_empty());
}

/// 🧯️ LAW: a host abort, a base that moved under the gesture and a switch to another verb each drop the open gesture
/// with zero trace; a sibling window's gesture is untouched.
#[test]
fn host_aborts_leave_zero_trace_and_keep_sibling_gestures() {
    let doc = demo();
    let tick = || fem3d_gumball_tick(&doc, &ids(&["n20_l1"]), Fem3dGumballMotion::Translate { dx: 0.5, dy: 0.0, dz: 0.0 });
    let left = drive(&FemGumballTransient::default(), "left", "translateSelection", GesturePhase::Stream, tick(), "base").transient.expect("left opens");
    let both = drive(&left, "right", "translateSelection", GesturePhase::Stream, tick(), "base").transient.expect("right opens");
    assert_eq!(both.gestures.len(), 2, "two windows hold two gestures");
    let aborted = drive(&both, "left", "translateSelection", GesturePhase::Abort(ToolAbortReason::CaptureLost), None, "base");
    assert!(aborted.committed.is_none());
    let rest = aborted.transient.expect("the abort clears the window's gesture");
    assert_eq!(rest.gestures.keys().collect::<Vec<_>>(), vec!["right"], "only the owner's gesture is dropped");
    let moved_base = drive(&rest, "right", "translateSelection", GesturePhase::Commit, tick(), "moved");
    assert!(moved_base.committed.is_none(), "a commit on a moved base is dropped with its gesture");
    assert!(moved_base.transient.expect("baseMoved clears it").gestures.is_empty());
    let open = drive(&FemGumballTransient::default(), "w", "translateSelection", GesturePhase::Stream, tick(), "base").transient.expect("opens");
    let scale = fem3d_gumball_tick(&doc, &ids(&["n20_l1", "n00_l1"]), Fem3dGumballMotion::Scale { sx: 2.0, sy: 1.0, sz: 1.0 });
    let switched = drive(&open, "w", "scaleSelection", GesturePhase::Stream, scale, "base").transient.expect("the new verb opens its own gesture");
    assert_eq!(switched.gestures["w"].verb, "scaleSelection", "the drag was dropped captureLost, the scaling opened fresh");
    assert_ne!(switched.gestures["w"].transaction, open.gestures["w"].transaction);
}

#[test]
fn the_selection_record_arms_the_gumball_only_with_the_transform_utility() {
    let doc = demo();
    let config = Fem3dGumballConfig::default();
    let selected = Fem3dInteractionSnapshot::selecting(["n20_l1"]);
    assert!(fem3d_gumball_active(&doc, &selected, true, &config));
    assert!(!fem3d_gumball_active(&doc, &selected, false, &config));
    assert!(!fem3d_gumball_active(&doc, &Fem3dInteractionSnapshot::selecting(["steel"]), true, &config));
    let json = fem3d_selection_json(&doc, &selected, true, &config);
    assert!(json.contains("\"gumballLiveDispatch\":true"), "{json}");
    assert!(json.contains("\"gumballTarget\":[8"), "{json}");
    let off = Fem3dGumballConfig { move_axes: false, move_planes: false, rotate: false, scale_axes: false, scale_uniform: false };
    assert!(!fem3d_gumball_active(&doc, &selected, true, &off));
}

/// ⏪️ LAW: a gumball move edited in history replays its downstream: the preview base is the state right before the
/// move, and the Report replay re-applies the downstream relative scaling and turn onto the edited move — exactly the
/// fresh fold of the edited log; overwrite commits it.
#[semio_framework_async_macros::async_test]
async fn a_gumball_move_edited_in_history_replays_its_downstream() {
    use protocol::OpBinary;
    let base = demo();
    let tick = |motion| Fem3dMutation::MoveSelection(fem3d_gumball_tick(&base, &ids(&["n20_l1", "sol1"]), motion).expect("the selection moves geometry"));
    let log = [tick(Fem3dGumballMotion::Translate { dx: 1.0, dy: 0.0, dz: 0.0 }), tick(Fem3dGumballMotion::Scale { sx: 2.0, sy: 1.0, sz: 1.0 }), tick(Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: 0.5 })];
    let mut store = store::ArtifactStore::<Fem3dSnapshot, Fem3dMutation>::new(store::create_document_envelope::<Fem3dSnapshot, Fem3dMutation>(crate::FEM_3D_SCHEMA, "gumball-time-travel", base.clone(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("the store opens");
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<Fem3dSnapshot, Fem3dMutation>());
    for mutation in &log {
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], transaction: None }).await.expect("the edit applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let edited = tick(Fem3dGumballMotion::Translate { dx: -0.5, dy: 3.0, dz: 0.25 });
    let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: crate::FEM_3D_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    assert_eq!(store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref(), &base, "the preview base is the state right before the edited move");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited move");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "a re-offset move never blocks finalizing");
    let fold = |mutations: &[Fem3dMutation]| {
        let mut snapshot = base.clone();
        for mutation in mutations {
            crate::central_apply::apply_fem3d_mutation(&mut snapshot, mutation).expect("the log folds");
        }
        snapshot
    };
    let fresh = fold(&[edited, log[1].clone(), log[2].clone()]);
    assert_ne!(fresh, fold(&log), "the edit changes the outcome");
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
    store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
    assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
    let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<Fem3dSnapshot, Fem3dMutation>();
    for _ in 0..4_096 {
        if disposer.terminal_is_empty(&store) {
            break;
        }
        disposer.close_step(&mut store, 1, 1 << 20).expect("the store retires");
    }
    assert!(disposer.terminal_is_empty(&store), "the standalone store retires to its terminal-empty shell");
}
