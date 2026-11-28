use super::*;
use semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballTransient;
use crate::editor::fem2d::interaction::canvas_gesture::FEM2D_UTILITY_TRANSFORM;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{Fem2dSnapshot, Fem2dSnapshotRetiring, Fem2dSnapshotValue};

fn demo() -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().expect("demo document parses"))
}

#[test]
fn gumball_meta_layer_emitted_when_transform_utility_and_selection() {
    let doc = demo();
    let meta = fem2d_gumball_meta_layer(&doc, &["scope:1:4:node2:n1".into()], FEM2D_UTILITY_TRANSFORM, &Fem2dGumballConfig::default()).expect("meta");
    assert_eq!(meta.get("role").and_then(|value| value.as_str()), Some("meta"));
    assert!(meta.get("gumball").is_some());
    assert_eq!(meta.pointer("/gumball/config/moveAxes").and_then(|value| value.as_bool()), Some(true));
}

#[test]
fn gumball_meta_config_follows_the_window_flags() {
    let doc = demo();
    let mut config = Fem2dGumballConfig::default();
    config.set_flag("rotate", Some(false)).expect("rotate is a flag");
    config.set_flag("scaleAxes", None).expect("scaleAxes is a flag");
    let meta = fem2d_gumball_meta_layer(&doc, &["scope:1:4:node2:n1".into()], FEM2D_UTILITY_TRANSFORM, &config).expect("meta");
    let flags: Vec<_> = ["moveAxes", "rotate", "scaleAxes", "scaleUniform"].into_iter().map(|key| meta.pointer(&format!("/gumball/config/{key}")).and_then(|value| value.as_bool())).collect();
    assert_eq!(flags, [Some(true), Some(false), Some(false), Some(true)]);
    assert!(config.set_flag("sparkle", None).is_err());
}

#[test]
fn gumball_meta_absent_without_selection() {
    let doc = demo();
    assert!(fem2d_gumball_meta_layer(&doc, &[], FEM2D_UTILITY_TRANSFORM, &Fem2dGumballConfig::default()).is_none());
}

/// 🎯️ A translate tick is the relative `move-selection` leaf over the selection's literal geometry, pivoted on its
/// centroid, and moves exactly that geometry by the offset.
#[test]
fn a_tick_is_the_relative_leaf_over_the_selected_geometry() {
    let doc = demo();
    let n1 = doc.nodes.iter().find(|node| node.id.as_str() == "n1").expect("n1").clone();
    let tick = fem2d_gumball_tick(&doc, &["scope:1:4:node2:n1".into()], Fem2dGumballMotion::Translate { dx: 1.0, dy: 2.0 }).expect("a node selection moves geometry");
    assert_eq!((tick.node_ids.clone(), tick.region_ids.clone(), tick.pivot_x, tick.pivot_y, tick.dx, tick.dy), (vec!["n1".to_string()], Vec::new(), n1.x, n1.y, 1.0, 2.0));
    let mut moved = doc.to_reference();
    semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation(&mut moved, &Fem2dMutation::MoveSelection(tick)).expect("the tick applies");
    let after = moved.nodes.iter().find(|node| node.id == "n1").expect("n1");
    assert_eq!((after.x, after.y), (n1.x + 1.0, n1.y + 2.0));
    assert!(fem2d_gumball_tick(&doc, &[], Fem2dGumballMotion::Rotate { angle: 0.5 }).is_none(), "an empty selection moves nothing");
}

/// 🛠️ Drives the tool the way the retained route does, on window `window` of `transient`.
fn drive(transient: &FemGumballTransient, window: &str, phase: GesturePhase, tick: Option<MoveSelection>, base: &str) -> semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballDrive<Fem2dMutation> {
    semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::fem_gumball_drive::<Fem2dGumballTool>(transient, window, "translateSelection", phase, tick, "seed", base).expect("the gumball tool accepts the dispatch")
}

/// 🛠️ The net leaf a gesture commits when the host sends `ticks` (`None` drops that tick as stale, so the guest never sees it).
fn net_of(verb: &str, ticks: Vec<(GesturePhase, Option<Option<MoveSelection>>)>) -> Option<MoveSelection> {
    let mut transient = FemGumballTransient::default();
    for (phase, tick) in ticks {
        let Some(tick) = tick else { continue };
        let step = semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::fem_gumball_drive::<Fem2dGumballTool>(&transient, "w", verb, phase, tick, "seed", "base").expect("the gumball tool accepts the dispatch");
        if let Some((_, mutations)) = step.committed {
            let [Fem2dMutation::MoveSelection(net)] = mutations.as_slice() else { panic!("one net leaf: {mutations:?}") };
            return Some(net.clone());
        }
        transient = step.transient.unwrap_or_default();
    }
    None
}

/// 🎯️ LAW (T1 accuracy): the host sends every stream tick and the commit CUMULATIVE from the press, so dropping any
/// subset of the stream ticks as stale (a refused tick is never retried) still commits exactly the full gesture's net
/// leaf — translate, turn and scale alike; with every stream tick dropped the commit alone carries the whole gesture.
#[test]
fn a_stream_tick_dropped_as_stale_costs_the_committed_net_nothing() {
    let doc = demo();
    let selection = ["n1", "p8"].map(|id| crate::editor::fem2d::interaction::fem2d_entity_target_id(FEM2D_GRANULARITY_NODE, id).expect("node identity"));
    let motions: [(&str, Vec<Fem2dGumballMotion>); 3] = [
        ("translateSelection", [0.25, 0.5, 0.875, 1.0].map(|dx| Fem2dGumballMotion::Translate { dx, dy: -dx / 2.0 }).to_vec()),
        ("rotateSelection", [0.1, 0.4, 0.7, 0.75].map(|angle| Fem2dGumballMotion::Rotate { angle }).to_vec()),
        ("scaleSelection", [1.25, 1.5, 2.0, 1.75].map(|factor| Fem2dGumballMotion::Scale { sx: factor, sy: 1.0 / factor }).to_vec()),
    ];
    for (verb, steps) in motions {
        let ticks = |dropped: &[usize]| -> Vec<(GesturePhase, Option<Option<MoveSelection>>)> {
            steps.iter().enumerate().map(|(index, motion)| (if index + 1 == steps.len() { GesturePhase::Commit } else { GesturePhase::Stream }, (!dropped.contains(&index)).then(|| fem2d_gumball_tick(&doc, &selection, *motion)))).collect()
        };
        let full = net_of(verb, ticks(&[])).unwrap_or_else(|| panic!("{verb}: the full gesture commits"));
        let last = fem2d_gumball_tick(&doc, &selection, *steps.last().expect("steps")).expect("tick");
        assert_eq!(full, last, "{verb}: the net is the cumulative total of the last tick");
        for dropped in [&[1][..], &[0][..], &[2][..], &[0, 1][..], &[1, 2][..], &[0, 1, 2][..]] {
            assert_eq!(net_of(verb, ticks(dropped)), Some(full.clone()), "{verb}: dropping stream ticks {dropped:?} loses nothing");
        }
    }
}

/// 💾️ LAW: a one-shot dispatch is ONE committed transaction holding the one leaf, and leaves no transient behind.
#[test]
fn a_one_shot_tick_commits_one_transaction() {
    let doc = demo();
    let tick = fem2d_gumball_tick(&doc, &["scope:1:4:node2:n1".into()], Fem2dGumballMotion::Translate { dx: 1.0, dy: 0.0 });
    let done = drive(&FemGumballTransient::default(), "w", GesturePhase::Once, tick.clone(), "base");
    let (reference, mutations) = done.committed.expect("committed");
    assert!(reference.id.starts_with("tx-") && reference.tool == format!("{FEM2D_EDITOR_APP_ID}#translateSelection"));
    assert_eq!(mutations, vec![Fem2dMutation::MoveSelection(tick.expect("tick"))]);
    assert!(done.transient.is_none(), "a one-shot persists nothing");
}

/// 🌊️ LAW: cumulative streamed ticks replace ONE open leaf of ONE transaction persisted for their window, and the commit
/// publishes the NET leaf once under the ref minted at the first tick.
#[test]
fn streamed_ticks_commit_the_net_leaf_once() {
    let doc = demo();
    let tick = |dx| fem2d_gumball_tick(&doc, &["scope:1:4:node2:n1".into()], Fem2dGumballMotion::Translate { dx, dy: 0.0 });
    let first = drive(&FemGumballTransient::default(), "w", GesturePhase::Stream, tick(0.5), "base");
    assert!(first.committed.is_none(), "a stream tick commits nothing");
    let open = first.transient.expect("the first tick opens the window's gesture");
    let minted = open.gesture("w").expect("window w holds a gesture").transaction.clone();
    let second = drive(&open, "w", GesturePhase::Stream, tick(0.75), "base");
    let open = second.transient.expect("the second tick advances it");
    assert_eq!(open.gesture("w").expect("window w holds a gesture").transaction, minted, "every tick rides the same transaction");
    let preview = open.preview::<Fem2dSnapshot, Fem2dSnapshotValue, Fem2dMutation>(&doc, Fem2dSnapshot::to_reference, |value| Fem2dSnapshot::from_reference(value).ok()).map(Fem2dSnapshotRetiring::new).expect("an open gesture previews");
    let start = doc.nodes.iter().find(|node| node.id.as_str() == "n1").expect("n1").x;
    assert_eq!(preview.nodes.iter().find(|node| node.id.as_str() == "n1").expect("n1").x, start + 0.75, "the preview is the net transform");
    let done = drive(&open, "w", GesturePhase::Commit, tick(0.75), "base");
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
    let tick = || fem2d_gumball_tick(&doc, &["scope:1:4:node2:n1".into()], Fem2dGumballMotion::Translate { dx: 0.5, dy: 0.0 });
    let left = drive(&FemGumballTransient::default(), "left", GesturePhase::Stream, tick(), "base").transient.expect("left opens");
    let both = drive(&left, "right", GesturePhase::Stream, tick(), "base").transient.expect("right opens");
    assert_eq!(both.gestures.len(), 2, "two windows hold two gestures");
    let aborted = drive(&both, "left", GesturePhase::Abort(ToolAbortReason::Blur), None, "base");
    assert!(aborted.committed.is_none());
    let rest = aborted.transient.expect("the abort clears the window's gesture");
    assert_eq!(rest.windows().collect::<Vec<_>>(), vec!["right"], "only the owner's gesture is dropped");
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
    let tick = |motion| Fem2dMutation::MoveSelection(fem2d_gumball_tick(&base, &["n1", "p8"].map(|id| crate::editor::fem2d::interaction::fem2d_entity_target_id(FEM2D_GRANULARITY_NODE, id).expect("node identity")), motion).expect("the selection moves geometry"));
    let log = [tick(Fem2dGumballMotion::Translate { dx: 1.0, dy: 0.0 }), tick(Fem2dGumballMotion::Scale { sx: 2.0, sy: 1.0 }), tick(Fem2dGumballMotion::Rotate { angle: 0.5 })];
    let mut store = store::ArtifactStore::<Fem2dSnapshot, Fem2dMutation>::new::<::semio_os_kernel::os_store::ArtifactSnapshotPublication>(store::os_store::document_materialize::create_document_envelope::<Fem2dSnapshot, Fem2dMutation>(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA, "gumball-time-travel", (*base).clone(), None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("the store opens");
    store.install_document_store_owners_exact(semio_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Fem2dSnapshot, Fem2dMutation>().expect("funded bounded document owners")).unwrap_or_else(|(error, _)| panic!("bounded document owners install refused: {error}"));
    for mutation in &log {
        store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation.clone()], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("the edit applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let edited = tick(Fem2dGumballMotion::Translate { dx: -0.5, dy: 3.0 });
    let drafts: protocol::HistoryInputDrafts = [(ids[0].clone(), protocol::InputReplacement::Input { schema: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    assert_eq!(store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref(), &*base, "the preview base is the state right before the edited move");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited move");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::os_store::replay::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "a re-offset move never blocks finalizing");
    let fold = |mutations: &[Fem2dMutation]| {
        let mut snapshot = base.to_reference();
        for mutation in mutations {
            semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation(&mut snapshot, mutation).expect("the log folds");
        }
        Fem2dSnapshotRetiring::new(Fem2dSnapshot::from_reference(snapshot).expect("the folded log seals"))
    };
    let fresh = fold(&[edited, log[1].clone(), log[2].clone()]);
    assert_ne!(*fresh, *fold(&log), "the edit changes the outcome");
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &*fresh, "the replay equals the fresh fold of the edited log");
    store.commit_finished_replay(result, store::HistoryFinalization::Overwrite, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("overwrite commits");
    assert_eq!(store.snapshot_ref(), &*fresh, "the overwritten history folds to the edited state");
    let mut disposer = semio_plugin::bounded_document_store_disposer::<Fem2dSnapshot, Fem2dMutation>();
    for _ in 0..4_096 {
        if disposer.terminal_is_empty(&store) {
            break;
        }
        let demand = disposer.retirement_demands(&store, 1 << 20).expect("the store quotes its next retirement turn");
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(1 << 20), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        disposer.close_step_cold(&mut store, grant).expect("the store retires");
    }
    assert!(disposer.terminal_is_empty(&store), "the standalone store retires to its terminal-empty shell");
}

/// 🔄️ LAW: a large rotate or uniform scale of a multi-node selection keeps EVERY entity (counts, ids, unselected geometry bit for bit) and maps each selected node EXACTLY by the rotation or scale about the selection centroid, which itself stays put. A single node is its own centroid, so rotating or uniformly scaling it moves nothing.
#[test]
fn a_large_rotate_or_uniform_scale_keeps_every_entity_and_maps_positions_exactly() {
    let doc = demo();
    let ids = ["n1", "p8", "ridge", "rc0"];
    let selection: Vec<String> = ids.iter().map(|id| crate::editor::fem2d::interaction::fem2d_entity_target_id(FEM2D_GRANULARITY_NODE, id).expect("node identity")).collect();
    let before = doc.to_reference();
    let pivot = |reference: &Fem2dSnapshotValue| {
        let points: Vec<(f64, f64)> = ids.iter().map(|id| reference.nodes.iter().find(|node| node.id == *id).map(|node| (node.x, node.y)).expect("demo node")).collect();
        (points.iter().map(|p| p.0).sum::<f64>() / points.len() as f64, points.iter().map(|p| p.1).sum::<f64>() / points.len() as f64)
    };
    let (cx, cy) = pivot(&before);
    let motions = [("rotate 60", Fem2dGumballMotion::Rotate { angle: 1.0 }), ("rotate 166", Fem2dGumballMotion::Rotate { angle: 2.9 }), ("rotate -115", Fem2dGumballMotion::Rotate { angle: -2.0 }), ("rotate 401", Fem2dGumballMotion::Rotate { angle: 7.0 }), ("uniform x3", Fem2dGumballMotion::Scale { sx: 3.0, sy: 3.0 }), ("uniform x0.2", Fem2dGumballMotion::Scale { sx: 0.2, sy: 0.2 })];
    for (name, motion) in motions {
        let tick = fem2d_gumball_tick(&doc, &selection, motion).unwrap_or_else(|| panic!("{name}: a node selection moves geometry"));
        let mut after = doc.to_reference();
        semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation(&mut after, &Fem2dMutation::MoveSelection(tick)).unwrap_or_else(|fault| panic!("{name}: the tick applies: {fault:?}"));
        assert_eq!((after.nodes.len(), after.elements.len(), after.regions.len(), after.supports.len(), after.load_cases.len(), after.combinations.len(), after.materials.len(), after.sections.len()), (before.nodes.len(), before.elements.len(), before.regions.len(), before.supports.len(), before.load_cases.len(), before.combinations.len(), before.materials.len(), before.sections.len()), "{name}: no entity is lost or added");
        assert_eq!(after.nodes.iter().map(|node| node.id.clone()).collect::<Vec<_>>(), before.nodes.iter().map(|node| node.id.clone()).collect::<Vec<_>>(), "{name}: node ids and order");
        assert_eq!(after.elements, before.elements, "{name}: elements keep their node references");
        assert_eq!(after.supports, before.supports, "{name}: supports are untouched");
        assert_eq!(after.regions, before.regions, "{name}: an unselected region is untouched");
        let (angle, factor) = match motion {
            Fem2dGumballMotion::Rotate { angle } => (angle, 1.0),
            Fem2dGumballMotion::Scale { sx, .. } => (0.0, sx),
            Fem2dGumballMotion::Translate { .. } => unreachable!("a rotate or scale"),
        };
        let (sin, cos) = angle.sin_cos();
        for (old, new) in before.nodes.iter().zip(after.nodes.iter()) {
            if ids.contains(&old.id.as_str()) {
                let (u, v) = ((old.x - cx) * factor, (old.y - cy) * factor);
                let expected = (cx + u * cos - v * sin, cy + u * sin + v * cos);
                assert!((new.x - expected.0).abs() < 1e-9 && (new.y - expected.1).abs() < 1e-9, "{name}: {} maps to {expected:?}, got {:?}", old.id, (new.x, new.y));
            } else {
                assert_eq!((new.x, new.y), (old.x, old.y), "{name}: unselected {} stays bit for bit", old.id);
            }
        }
        let (nx, ny) = pivot(&after);
        assert!((nx - cx).abs() < 1e-9 && (ny - cy).abs() < 1e-9, "{name}: the centroid stays put");
    }
    let lone = [crate::editor::fem2d::interaction::fem2d_entity_target_id(FEM2D_GRANULARITY_NODE, "ridge").expect("node identity")];
    for motion in [Fem2dGumballMotion::Rotate { angle: 2.9 }, Fem2dGumballMotion::Scale { sx: 3.0, sy: 3.0 }] {
        let tick = fem2d_gumball_tick(&doc, &lone, motion).expect("a node selection resolves a tick");
        assert!(tick.is_identity(), "a lone node moves nothing, so its tick is the identity");
        let mut after = doc.to_reference();
        semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation(&mut after, &Fem2dMutation::MoveSelection(tick)).expect("the tick applies");
        let (old, new) = (before.nodes.iter().find(|node| node.id == "ridge").expect("ridge"), after.nodes.iter().find(|node| node.id == "ridge").expect("ridge"));
        assert!((new.x - old.x).abs() < 1e-12 && (new.y - old.y).abs() < 1e-12, "a lone node is its own pivot");
    }
}

/// 🫥️ LAW: a gesture that moves nothing leaves the history length and the document unchanged: rotating or uniformly scaling a lone node about its own centroid, or a zero-length drag, resolves to the identity tick, so a one-shot commits no transaction and a streamed gesture (preview ticks included) retracts its leaf and commits nothing; a gesture that does move still commits exactly one.
#[test]
fn an_identity_gesture_leaves_the_history_length_and_the_document_unchanged() {
    let doc = demo();
    let lone = [crate::editor::fem2d::interaction::fem2d_entity_target_id(FEM2D_GRANULARITY_NODE, "ridge").expect("node identity")];
    let pair = ["n1", "p8"].map(|id| crate::editor::fem2d::interaction::fem2d_entity_target_id(FEM2D_GRANULARITY_NODE, id).expect("node identity"));
    let nothing = [(&lone[..], Fem2dGumballMotion::Rotate { angle: 2.9 }), (&lone[..], Fem2dGumballMotion::Scale { sx: 3.0, sy: 3.0 }), (&lone[..], Fem2dGumballMotion::Translate { dx: 0.0, dy: 0.0 }), (&pair[..], Fem2dGumballMotion::Rotate { angle: 0.0 }), (&pair[..], Fem2dGumballMotion::Scale { sx: 1.0, sy: 1.0 })];
    for (selection, motion) in nothing {
        let tick = fem2d_gumball_tick(&doc, selection, motion).expect("the selection resolves a tick");
        assert!(tick.is_identity(), "{motion:?} moves nothing, so its tick is the identity");
        assert!(drive(&FemGumballTransient::default(), "w", GesturePhase::Once, Some(tick.clone()), "base").committed.is_none(), "{motion:?}: a one-shot commits no command");
        let mut transient = FemGumballTransient::default();
        for (phase, at) in [(GesturePhase::Stream, 0), (GesturePhase::Stream, 1), (GesturePhase::Commit, 2)] {
            let step = drive(&transient, "w", phase, Some(tick.clone()), "base");
            assert!(step.committed.is_none(), "{motion:?}: tick {at} commits no command");
            transient = step.transient.unwrap_or_default();
        }
        assert!(transient.gestures.is_empty(), "{motion:?}: nothing is left open");
    }
    let mut streamed = FemGumballTransient::default();
    for angle in [0.4, 0.8] {
        let step = drive(&streamed, "w", GesturePhase::Stream, fem2d_gumball_tick(&doc, &pair, Fem2dGumballMotion::Rotate { angle }), "base");
        streamed = step.transient.expect("the preview stays open");
    }
    let back = fem2d_gumball_tick(&doc, &lone, Fem2dGumballMotion::Rotate { angle: 0.0 });
    let done = drive(&streamed, "w", GesturePhase::Commit, back, "base");
    assert!(done.committed.is_none(), "a gesture dragged back onto its press commits no command");
    let real = drive(&FemGumballTransient::default(), "w", GesturePhase::Once, fem2d_gumball_tick(&doc, &pair, Fem2dGumballMotion::Rotate { angle: 0.8 }), "base");
    assert_eq!(real.committed.expect("a rotation that moves commits").1.len(), 1);
}
