use super::*;
use crate::editor::fem3d::interaction::retiring::Retiring;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemAxis;
use semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::fem_gumball_drive;
use semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballDrive;
use semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballTransient;

fn demo() -> Retiring<Fem3dSnapshot> {
    Retiring::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("demo snapshot"))
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn targets(values: &[(&str, &str)]) -> Vec<String> {
    values.iter().map(|(kind, id)| super::super::fem3d_entity_target(kind, id).unwrap()).collect()
}

fn moved(doc: &Fem3dSnapshot, tick: MoveSelection) -> Retiring<Fem3dSnapshot> {
    let mut value = doc.to_reference();
    semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation(&mut value, &Fem3dMutation::MoveSelection(tick)).expect("the tick applies");
    Retiring::new(Fem3dSnapshot::from_reference(value).expect("the moved snapshot"))
}

fn node(doc: &Fem3dSnapshot, id: &str) -> [f64; 3] {
    let node = doc.nodes.iter().find(|node| node.id.as_str() == id).expect("node");
    [node.x, node.y, node.z]
}

#[test]
fn targets_resolve_members_supports_and_loads_to_their_geometry() {
    let doc = demo();
    let resolved = fem3d_transform_targets(&doc, &[super::super::fem3d_entity_target("element", "fb1_0").unwrap(), super::super::fem3d_entity_target("support", "s_20").unwrap(), super::super::fem3d_load_target("dead", "l1").unwrap(), super::super::fem3d_entity_target("material", "steel").unwrap()]);
    assert_eq!(resolved.node_ids, ids(&["n00_l1", "n20_g", "n20_l1"]), "nodes come once each, in document order");
    assert_eq!(resolved.solid_ids, ids(&["sol1"]));
    assert!(fem3d_transform_targets(&doc, &targets(&[("material", "steel")])).is_empty());
}

/// 🧮️ Repeated opaque identities project once, retaining document order and a centroid independent of selection order.
#[test]
fn wide_selection_resolves_each_geometry_once_in_document_order() {
    let vector: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️projection/🔣️.json")).unwrap();
    let mut value = Fem3dSnapshotValue::default();
    for index in 0..vector["nodeCount"].as_u64().unwrap() {
        value.nodes.push(semio_s_fem_node::volumetric::FemNodeValue { id: format!("node:{index}"), x: index as f64, y: 0.0, z: 0.0 });
    }
    let doc = Retiring::new(Fem3dSnapshot::from_reference(value).expect("wide snapshot"));
    let rows: Vec<_> = doc.nodes.iter().collect();
    let selection: Vec<String> = rows.into_iter().rev().flat_map(|node| {
        let identity = super::super::fem3d_entity_target("node", node.id.as_str()).unwrap();
        [identity.clone(), identity]
    }).collect();
    let resolved = fem3d_transform_targets(&doc, &selection);
    assert_eq!(resolved.node_ids, doc.nodes.iter().map(|node| node.id.as_str().to_owned()).collect::<Vec<_>>());
    assert_eq!(resolved.node_ids.len() as u64, vector["nodeCount"].as_u64().unwrap());
    assert_eq!(fem3d_transform_pivot(&doc, &resolved).unwrap(), [vector["pivot"][0].as_f64().unwrap(), 0.0, 0.0]);
    let interaction = Fem3dInteractionSnapshot::selecting(selection);
    let record: serde_json::Value = serde_json::from_str(&fem3d_selection_json(&doc, &interaction, true, &Fem3dGumballConfig::default())).unwrap();
    assert_eq!(record["gumballTarget"], vector["pivot"]);
}

/// 🎯️ A tick is the relative `move-selection` leaf over the selection's literal geometry, pivoted on its centroid,
/// and moves exactly that geometry: a drag by the offset, a quarter turn about the pivot, a scaling about it.
#[test]
fn a_tick_is_the_relative_leaf_over_the_selected_geometry() {
    let doc = demo();
    let drag = fem3d_gumball_tick(&doc, &targets(&[("node", "n20_l1"), ("solid", "sol1")]), Fem3dGumballMotion::Translate { dx: 1.0, dy: 2.0, dz: 3.0 }).expect("a node and a solid move geometry");
    assert_eq!((drag.node_ids.clone(), drag.solid_ids.clone(), [drag.dx, drag.dy, drag.dz]), (ids(&["n20_l1"]), ids(&["sol1"]), [1.0, 2.0, 3.0]));
    let dragged = moved(&doc, drag);
    assert_eq!(node(&dragged, "n20_l1"), [9.0, 2.0, 5.8]);
    let slab = dragged.solids.iter().find(|solid| solid.id == "sol1").expect("sol1");
    assert_eq!(slab.outline[0], [11.0, 2.0]);
    assert!((slab.base_z - doc.solids[0].base_z - 3.0).abs() < 1e-12 && slab.height == doc.solids[0].height, "the slab rides the drag: {slab:?}");
    let quarter = std::f64::consts::FRAC_PI_2;
    let turn = fem3d_gumball_tick(&doc, &targets(&[("node", "n20_g"), ("node", "n00_g")]), Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: quarter }).expect("turn");
    assert_eq!([turn.pivot_x, turn.pivot_y, turn.pivot_z], [4.0, 0.0, 0.0], "the pivot is the selection's centroid");
    let turned = node(&moved(&doc, turn), "n20_g");
    assert!((turned[0] - 4.0).abs() < 1e-9 && (turned[1] - 4.0).abs() < 1e-9, "n20_g swings a quarter turn about (4, 0, 0): {turned:?}");
    let stretch = fem3d_gumball_tick(&doc, &targets(&[("node", "n00_g"), ("node", "n20_g")]), Fem3dGumballMotion::Scale { sx: 2.0, sy: 1.0, sz: 1.0 }).expect("stretch");
    assert_eq!(node(&moved(&doc, stretch), "n20_g"), [12.0, 0.0, 0.0]);
    let mut sideways = demo();
    sideways.solids[0].axis = FemAxis::Y;
    let deeper = fem3d_gumball_tick(&sideways, &targets(&[("solid", "sol1")]), Fem3dGumballMotion::Scale { sx: 1.0, sy: 2.0, sz: 1.0 }).expect("deeper");
    let extruded = moved(&sideways, deeper);
    assert!((extruded.solids[0].height - 2.0 * sideways.solids[0].height).abs() < 1e-9, "a Y extrusion doubles along Y: {}", extruded.solids[0].height);
    assert!(fem3d_gumball_tick(&doc, &targets(&[("material", "steel")]), Fem3dGumballMotion::Translate { dx: 1.0, dy: 0.0, dz: 0.0 }).is_none(), "a material moves no geometry");
}

/// 🛠️ Drives the tool the way the retained route does, on window `window` of `transient`.
fn drive(transient: &FemGumballTransient, window: &str, verb: &str, phase: GesturePhase, tick: Option<MoveSelection>, base: &str) -> FemGumballDrive<Fem3dMutation> {
    fem_gumball_drive::<Fem3dGumballTool>(transient, window, verb, phase, tick, "seed", base).expect("the gumball tool accepts the dispatch")
}

/// 🛠️ The net leaf a gesture commits when the host sends `ticks` (`None` drops that tick as stale, so the guest never sees it).
fn net_of(verb: &str, ticks: Vec<(GesturePhase, Option<Option<MoveSelection>>)>) -> Option<MoveSelection> {
    let mut transient = FemGumballTransient::default();
    for (phase, tick) in ticks {
        let Some(tick) = tick else { continue };
        let step = drive(&transient, "w", verb, phase, tick, "base");
        if let Some((_, mutations)) = step.committed {
            let [Fem3dMutation::MoveSelection(net)] = mutations.as_slice() else { panic!("one net leaf: {mutations:?}") };
            return Some(net.clone());
        }
        transient = step.transient.unwrap_or_default();
    }
    None
}

/// 🎯️ LAW (T1 accuracy): the host sends every stream tick and the commit CUMULATIVE from the grab (`gumballCumulative`), so
/// dropping any subset of the stream ticks as stale (a refused tick is never retried) still commits exactly the full
/// gesture's net leaf — translate, turn (also when the reported axis flips) and scale alike; with every stream tick
/// dropped the commit alone carries the whole gesture.
#[test]
fn a_stream_tick_dropped_as_stale_costs_the_committed_net_nothing() {
    let doc = demo();
    let selection = targets(&[("node", "n20_l1"), ("node", "n00_l1")]);
    let motions: [(&str, Vec<Fem3dGumballMotion>); 3] = [
        ("translateSelection", [0.25, 0.5, 0.875, 1.0].map(|dx| Fem3dGumballMotion::Translate { dx, dy: -dx / 2.0, dz: dx / 4.0 }).to_vec()),
        ("rotateSelection", vec![Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: 0.1 }, Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: 0.4 }, Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, -1.0], angle: 0.3 }, Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: 0.75 }]),
        ("scaleSelection", [1.25, 1.5, 2.0, 1.75].map(|factor| Fem3dGumballMotion::Scale { sx: factor, sy: 1.0 / factor, sz: 1.0 }).to_vec()),
    ];
    for (verb, steps) in motions {
        let ticks = |dropped: &[usize]| -> Vec<(GesturePhase, Option<Option<MoveSelection>>)> {
            steps.iter().enumerate().map(|(index, motion)| (if index + 1 == steps.len() { GesturePhase::Commit } else { GesturePhase::Stream }, (!dropped.contains(&index)).then(|| fem3d_gumball_tick(&doc, &selection, *motion)))).collect()
        };
        let full = net_of(verb, ticks(&[])).unwrap_or_else(|| panic!("{verb}: the full gesture commits"));
        let last = fem3d_gumball_tick(&doc, &selection, *steps.last().expect("steps")).expect("tick");
        assert_eq!(full, last, "{verb}: the net is the cumulative total of the last tick");
        for dropped in [&[1][..], &[0][..], &[2][..], &[0, 1][..], &[1, 2][..], &[0, 1, 2][..]] {
            assert_eq!(net_of(verb, ticks(dropped)), Some(full.clone()), "{verb}: dropping stream ticks {dropped:?} loses nothing");
        }
    }
}

/// 💾️ LAW: a one-shot dispatch is ONE committed transaction holding the one leaf and leaves no transient behind; a
/// tick that moves nothing or that the leaf refuses (a rotation about the zero axis) commits nothing.
#[test]
fn a_one_shot_tick_commits_one_transaction() {
    let doc = demo();
    let tick = fem3d_gumball_tick(&doc, &targets(&[("node", "n20_l1")]), Fem3dGumballMotion::Translate { dx: 1.0, dy: 0.0, dz: 0.0 });
    let done = drive(&FemGumballTransient::default(), "w", "translateSelection", GesturePhase::Once, tick.clone(), "base");
    let (reference, mutations) = done.committed.expect("committed");
    assert!(reference.id.starts_with("tx-") && reference.tool == format!("{FEM3D_EDITOR_APP_ID}#translateSelection"), "{reference:?}");
    assert_eq!(mutations, vec![Fem3dMutation::MoveSelection(tick.expect("tick"))]);
    assert!(done.transient.is_none(), "a one-shot persists nothing");
    let idle = fem3d_gumball_tick(&doc, &targets(&[("node", "n20_l1")]), Fem3dGumballMotion::Translate { dx: 0.0, dy: 0.0, dz: 0.0 });
    assert!(drive(&FemGumballTransient::default(), "w", "translateSelection", GesturePhase::Once, idle, "base").committed.is_none(), "an identity tick commits nothing");
    let axisless = fem3d_gumball_tick(&doc, &targets(&[("node", "n20_l1")]), Fem3dGumballMotion::Rotate { axis: [0.0; 3], angle: 0.5 });
    assert!(drive(&FemGumballTransient::default(), "w", "rotateSelection", GesturePhase::Once, axisless, "base").committed.is_none(), "a rotation about the zero axis commits nothing");
}

/// 🌊️ LAW: cumulative streamed ticks replace ONE open leaf of ONE transaction persisted for their window, previewed as the net
/// transform, and the commit publishes the NET leaf once under the ref minted at the first tick.
#[test]
fn streamed_ticks_commit_the_net_leaf_once() {
    let doc = demo();
    let tick = |angle| fem3d_gumball_tick(&doc, &targets(&[("node", "n20_g"), ("node", "n00_g")]), Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle });
    let first = drive(&FemGumballTransient::default(), "w", "rotateSelection", GesturePhase::Stream, tick(0.5), "base");
    assert!(first.committed.is_none(), "a stream tick commits nothing");
    let open = first.transient.expect("the first tick opens the window's gesture");
    let minted = open.gesture("w").expect("window w holds a gesture").transaction.clone();
    let second = drive(&open, "w", "rotateSelection", GesturePhase::Stream, tick(std::f64::consts::FRAC_PI_2), "base");
    let open = second.transient.expect("the second tick advances it");
    assert_eq!(open.gesture("w").expect("window w holds a gesture").transaction, minted, "every tick rides the same transaction");
    let preview = open.preview::<Fem3dSnapshot, Fem3dSnapshotValue, Fem3dMutation>(&doc, Fem3dSnapshot::to_reference, |value| Fem3dSnapshot::from_reference(value).ok()).map(Retiring::new).expect("an open gesture previews");
    let swung = node(&preview, "n20_g");
    assert!((swung[0] - 4.0).abs() < 1e-9 && (swung[1] - 4.0).abs() < 1e-9, "the preview is the net quarter turn: {swung:?}");
    let done = drive(&open, "w", "rotateSelection", GesturePhase::Commit, tick(std::f64::consts::FRAC_PI_2), "base");
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
    let tick = || fem3d_gumball_tick(&doc, &targets(&[("node", "n20_l1")]), Fem3dGumballMotion::Translate { dx: 0.5, dy: 0.0, dz: 0.0 });
    let left = drive(&FemGumballTransient::default(), "left", "translateSelection", GesturePhase::Stream, tick(), "base").transient.expect("left opens");
    let both = drive(&left, "right", "translateSelection", GesturePhase::Stream, tick(), "base").transient.expect("right opens");
    assert_eq!(both.gestures.len(), 2, "two windows hold two gestures");
    let aborted = drive(&both, "left", "translateSelection", GesturePhase::Abort(ToolAbortReason::CaptureLost), None, "base");
    assert!(aborted.committed.is_none());
    let rest = aborted.transient.expect("the abort clears the window's gesture");
    assert_eq!(rest.windows().collect::<Vec<_>>(), vec!["right"], "only the owner's gesture is dropped");
    let moved_base = drive(&rest, "right", "translateSelection", GesturePhase::Commit, tick(), "moved");
    assert!(moved_base.committed.is_none(), "a commit on a moved base is dropped with its gesture");
    assert!(moved_base.transient.expect("baseMoved clears it").gestures.is_empty());
    let open = drive(&FemGumballTransient::default(), "w", "translateSelection", GesturePhase::Stream, tick(), "base").transient.expect("opens");
    let scale = fem3d_gumball_tick(&doc, &targets(&[("node", "n20_l1"), ("node", "n00_l1")]), Fem3dGumballMotion::Scale { sx: 2.0, sy: 1.0, sz: 1.0 });
    let switched = drive(&open, "w", "scaleSelection", GesturePhase::Stream, scale, "base").transient.expect("the new verb opens its own gesture");
    assert_eq!(switched.gesture("w").expect("window w holds a gesture").verb, "scaleSelection", "the drag was dropped captureLost, the scaling opened fresh");
    assert_ne!(switched.gesture("w").expect("window w holds a gesture").transaction, open.gesture("w").expect("window w holds a gesture").transaction);
}

#[test]
fn the_selection_record_arms_the_gumball_only_with_the_transform_utility() {
    let doc = demo();
    let config = Fem3dGumballConfig::default();
    let selected = Fem3dInteractionSnapshot::selecting(targets(&[("node", "n20_l1")]));
    assert!(fem3d_gumball_active(&doc, &selected, true, &config));
    assert!(!fem3d_gumball_active(&doc, &selected, false, &config));
    assert!(!fem3d_gumball_active(&doc, &Fem3dInteractionSnapshot::selecting(targets(&[("material", "steel")])), true, &config));
    let json = fem3d_selection_json(&doc, &selected, true, &config);
    assert!(json.contains("\"gumballLiveDispatch\":true"), "{json}");
    assert!(json.contains("\"gumballCumulative\":true"), "{json}");
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
    let tick = |motion| Fem3dMutation::MoveSelection(fem3d_gumball_tick(&base, &targets(&[("node", "n20_l1"), ("solid", "sol1")]), motion).expect("the selection moves geometry"));
    let log = [tick(Fem3dGumballMotion::Translate { dx: 1.0, dy: 0.0, dz: 0.0 }), tick(Fem3dGumballMotion::Scale { sx: 2.0, sy: 1.0, sz: 1.0 }), tick(Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: 0.5 })];
    let mut store = store::ArtifactStore::<Fem3dSnapshot, Fem3dMutation>::new::<::semio_os_kernel::os_store::ArtifactSnapshotPublication>(store::os_store::document_materialize::create_document_envelope::<Fem3dSnapshot, Fem3dMutation>(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA, "gumball-time-travel", base.clone(), None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("the store opens");
    store.install_document_store_owners_exact(semio_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Fem3dSnapshot, Fem3dMutation>().expect("funded bounded document owners")).unwrap_or_else(|(error, _)| panic!("bounded document owners install refused: {error}"));
    for mutation in &log {
        store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation.clone()], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("the edit applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let edited = tick(Fem3dGumballMotion::Translate { dx: -0.5, dy: 3.0, dz: 0.25 });
    let drafts: protocol::HistoryInputDrafts = [(ids[0].clone(), protocol::InputReplacement::Input { schema: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    assert_eq!(store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref(), &*base, "the preview base is the state right before the edited move");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited move");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::os_store::replay::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "a re-offset move never blocks finalizing");
    let fold = |mutations: &[Fem3dMutation]| {
        let mut value = base.to_reference();
        for mutation in mutations {
            semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation(&mut value, mutation).expect("the log folds");
        }
        Retiring::new(Fem3dSnapshot::from_reference(value).expect("the folded snapshot"))
    };
    let fresh = fold(&[edited, log[1].clone(), log[2].clone()]);
    assert_ne!(*fresh, *fold(&log), "the edit changes the outcome");
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &*fresh, "the replay equals the fresh fold of the edited log");
    store.commit_finished_replay(result, store::HistoryFinalization::Overwrite, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("overwrite commits");
    assert_eq!(store.snapshot_ref(), &*fresh, "the overwritten history folds to the edited state");
    let mut disposer = semio_plugin::bounded_document_store_disposer::<Fem3dSnapshot, Fem3dMutation>();
    for _ in 0..1 << 16 {
        if disposer.terminal_is_empty(&store) {
            break;
        }
        let demand = disposer.retirement_demands(&store, 1 << 20).expect("the store quotes its next retirement turn");
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(1 << 20), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        disposer.close_step_cold(&mut store, grant).expect("the store retires");
    }
    assert!(disposer.terminal_is_empty(&store), "the standalone store retires to its terminal-empty shell");
}

/// 🌀️ LAW (F2): the ~60 px drag of the Z ring on the slab `sol1` — a stream of cumulative ticks whose reported axis carries
/// sensor noise and flips sign when the pointer reverses — replaces ONE net leaf that turns the slab into a valid, planar,
/// same-area, same-height solid about its own centroid.
#[test]
fn a_sixty_pixel_ring_drag_turns_the_slab_into_a_valid_solid() {
    let doc = demo();
    let target = targets(&[("solid", "sol1")]);
    let area = |solid: &semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSolid| solid.outline.iter().zip(solid.outline.iter().cycle().skip(1)).map(|(a, b)| a[0] * b[1] - b[0] * a[1]).sum::<f64>() / 2.0;
    let base = doc.solids.iter().find(|solid| solid.id == "sol1").expect("sol1").clone();
    let totals: [f64; 12] = [0.12, 0.24, 0.35, 0.45, 0.55, 0.5, 0.46, 0.58, 0.71, 0.83, 0.93, 1.02];
    let mut transient = FemGumballTransient::default();
    let mut net: Option<MoveSelection> = None;
    for (index, total) in totals.iter().enumerate() {
        let noise = 1e-7 * ((index as f64 * 2.3).sin());
        let axis = [noise, noise * 0.5, 1.0];
        let tick = fem3d_gumball_tick(&doc, &target, Fem3dGumballMotion::Rotate { axis, angle: *total }).expect("tick");
        assert!(fem3d_gumball_moves(&tick), "step {index} moves");
        let step = drive(&transient, "w", "rotateSelection", if index + 1 == totals.len() { GesturePhase::Commit } else { GesturePhase::Stream }, Some(tick), "base");
        if let Some((_, mutations)) = step.committed {
            let [Fem3dMutation::MoveSelection(leaf)] = mutations.as_slice() else { panic!("one net leaf: {mutations:?}") };
            net = Some(leaf.clone());
        }
        transient = step.transient.unwrap_or_default();
    }
    let net = net.expect("net leaf");
    assert!((net.angle - 1.02).abs() < 1e-12, "the net angle is the last cumulative total: {}", net.angle);
    let turned = moved(&doc, net);
    let solid = turned.solids.iter().find(|solid| solid.id == "sol1").expect("the slab is still there");
    assert_ne!(solid.outline, base.outline, "the slab turned");
    assert!(solid.outline.iter().flatten().all(|value| value.is_finite()));
    assert!((solid.height - base.height).abs() < 1e-6 && (solid.base_z - base.base_z).abs() < 1e-6, "{solid:?}");
    assert!((area(solid) - area(&base)).abs() < 1e-6, "a turn keeps the footprint area");
    assert!(solid.height > 0.0 && solid.outline.len() == base.outline.len(), "the turned slab keeps its extrusion");
}

/// 📏️ LAW (F2): a scale tick that is zero, negative or not finite is dropped by the tool and never reaches the leaf, so
/// a handle dragged through its origin cannot destroy the solid; a positive tick scales it about its centroid.
#[test]
fn degenerate_scale_ticks_never_reach_the_slab() {
    let doc = demo();
    let target = targets(&[("solid", "sol1")]);
    for factors in [[0.0, 1.0, 1.0], [-1.5, 1.0, 1.0], [1.0, f64::NAN, 1.0], [1.0, 1.0, f64::INFINITY]] {
        let tick = fem3d_gumball_tick(&doc, &target, Fem3dGumballMotion::Scale { sx: factors[0], sy: factors[1], sz: factors[2] });
        assert!(drive(&FemGumballTransient::default(), "w", "scaleSelection", GesturePhase::Stream, tick, "base").transient.is_none_or(|open| open.gestures.is_empty()), "{factors:?} opens no gesture");
    }
    let tick = fem3d_gumball_tick(&doc, &target, Fem3dGumballMotion::Scale { sx: 1.5, sy: 1.0, sz: 1.0 }).expect("tick");
    let scaled = moved(&doc, tick);
    let solid = scaled.solids.iter().find(|solid| solid.id == "sol1").expect("sol1");
    assert!(solid.height > 0.0 && solid.outline.iter().flatten().all(|value| value.is_finite()), "{solid:?}");
}

/// 🔄️ LAW: a large rotate or uniform scale of a multi-node selection keeps EVERY entity (counts, unselected geometry bit for bit) and maps each selected node EXACTLY by the Rodrigues rotation or the scale about the selection centroid, which itself stays put; a lone node is its own pivot and moves nothing.
#[test]
fn a_large_rotate_or_uniform_scale_keeps_every_entity_and_maps_positions_exactly() {
    let doc = demo();
    let picked = ["n00_g", "n20_g", "n20_l1"];
    let selection = targets(&picked.map(|id| ("node", id)));
    let before = doc.to_reference();
    let at = |snapshot: &Fem3dSnapshotValue, id: &str| { let node = snapshot.nodes.iter().find(|node| node.id == id).expect("demo node"); [node.x, node.y, node.z] };
    let centroid = |snapshot: &Fem3dSnapshotValue| { let points = picked.map(|id| at(snapshot, id)); [0, 1, 2].map(|axis| points.iter().map(|point| point[axis]).sum::<f64>() / points.len() as f64) };
    let pivot = centroid(&before);
    let rodrigues = |vector: [f64; 3], axis: [f64; 3], angle: f64| {
        let length = axis.iter().map(|value| value * value).sum::<f64>().sqrt();
        let k = axis.map(|value| value / length);
        let (sin, cos) = angle.sin_cos();
        let cross = [k[1] * vector[2] - k[2] * vector[1], k[2] * vector[0] - k[0] * vector[2], k[0] * vector[1] - k[1] * vector[0]];
        let dot = k[0] * vector[0] + k[1] * vector[1] + k[2] * vector[2];
        [0, 1, 2].map(|i| vector[i] * cos + cross[i] * sin + k[i] * dot * (1.0 - cos))
    };
    let motions = [([0.0, 0.0, 1.0], 1.0), ([0.0, 0.0, 1.0], 2.9), ([1.0, 0.0, 0.0], -2.0), ([1.0, 1.0, 1.0], 7.0), ([0.0, 1.0, 0.0], 3.1)];
    for (axis, angle) in motions {
        let tick = fem3d_gumball_tick(&doc, &selection, Fem3dGumballMotion::Rotate { axis, angle }).expect("a node selection moves geometry");
        let after = moved(&doc, tick).to_reference();
        assert_eq!((after.nodes.len(), after.elements.len(), after.solids.len(), after.supports.len(), after.load_cases.len()), (before.nodes.len(), before.elements.len(), before.solids.len(), before.supports.len(), before.load_cases.len()), "rotate {angle} about {axis:?}: no entity is lost or added");
        for (old, new) in before.nodes.iter().zip(after.nodes.iter()) {
            assert_eq!(old.id, new.id, "node order");
            if picked.contains(&old.id.as_str()) {
                let turned = rodrigues([old.x - pivot[0], old.y - pivot[1], old.z - pivot[2]], axis, angle);
                let expected = [pivot[0] + turned[0], pivot[1] + turned[1], pivot[2] + turned[2]];
                assert!([new.x, new.y, new.z].iter().zip(expected).all(|(got, want)| (got - want).abs() < 1e-9), "{} maps to {expected:?}, got {:?}", old.id, [new.x, new.y, new.z]);
            } else {
                assert_eq!([new.x, new.y, new.z], [old.x, old.y, old.z], "unselected {} stays bit for bit", old.id);
            }
        }
        assert!(centroid(&after).iter().zip(pivot).all(|(got, want)| (got - want).abs() < 1e-9), "the centroid stays put");
    }
    let tick = fem3d_gumball_tick(&doc, &selection, Fem3dGumballMotion::Scale { sx: 3.0, sy: 3.0, sz: 3.0 }).expect("uniform scale");
    let after = moved(&doc, tick).to_reference();
    assert_eq!(after.nodes.len(), before.nodes.len());
    for id in picked {
        let (old, new) = (at(&before, id), at(&after, id));
        assert!([0, 1, 2].iter().all(|&i| (new[i] - (pivot[i] + 3.0 * (old[i] - pivot[i]))).abs() < 1e-9), "{id} scales about the centroid");
    }
    assert!(centroid(&after).iter().zip(pivot).all(|(got, want)| (got - want).abs() < 1e-9), "the scale keeps the centroid");
    let lone = targets(&[("node", "n20_l1")]);
    for motion in [Fem3dGumballMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: 2.9 }, Fem3dGumballMotion::Scale { sx: 3.0, sy: 3.0, sz: 3.0 }] {
        let tick = fem3d_gumball_tick(&doc, &lone, motion).expect("a node selection resolves a tick");
        assert!(tick.is_identity(), "a lone node moves nothing, so its tick is the identity");
        let after = moved(&doc, tick).to_reference();
        assert!(at(&after, "n20_l1").iter().zip(at(&before, "n20_l1")).all(|(got, want)| (got - want).abs() < 1e-12), "a lone node is its own pivot");
    }
}

/// 🫥️ LAW: a gesture that moves nothing leaves the history length and the document unchanged: rotating or uniformly scaling a lone node about its own centroid, or a zero-length drag, resolves to the identity tick, so a one-shot commits no transaction and a streamed gesture (preview ticks included) retracts its leaf and commits nothing; a gesture that does move still commits exactly one.
#[test]
fn an_identity_gesture_leaves_the_history_length_and_the_document_unchanged() {
    let doc = demo();
    let lone = targets(&[("node", "n20_l1")]);
    let pair = targets(&[("node", "n20_g"), ("node", "n00_g")]);
    let z = [0.0, 0.0, 1.0];
    let nothing = [(&lone, Fem3dGumballMotion::Rotate { axis: z, angle: 2.9 }), (&lone, Fem3dGumballMotion::Scale { sx: 3.0, sy: 3.0, sz: 3.0 }), (&lone, Fem3dGumballMotion::Translate { dx: 0.0, dy: 0.0, dz: 0.0 }), (&pair, Fem3dGumballMotion::Rotate { axis: z, angle: 0.0 }), (&pair, Fem3dGumballMotion::Scale { sx: 1.0, sy: 1.0, sz: 1.0 })];
    for (selection, motion) in nothing {
        let tick = fem3d_gumball_tick(&doc, selection, motion).expect("the selection resolves a tick");
        assert!(tick.is_identity(), "{motion:?} moves nothing, so its tick is the identity");
        assert!(drive(&FemGumballTransient::default(), "w", "rotateSelection", GesturePhase::Once, Some(tick.clone()), "base").committed.is_none(), "{motion:?}: a one-shot commits no command");
        let mut transient = FemGumballTransient::default();
        for (phase, at) in [(GesturePhase::Stream, 0), (GesturePhase::Stream, 1), (GesturePhase::Commit, 2)] {
            let step = drive(&transient, "w", "rotateSelection", phase, Some(tick.clone()), "base");
            assert!(step.committed.is_none(), "{motion:?}: tick {at} commits no command");
            transient = step.transient.unwrap_or_default();
        }
        assert!(transient.gestures.is_empty(), "{motion:?}: nothing is left open");
    }
    let mut streamed = FemGumballTransient::default();
    for angle in [0.4, 0.8] {
        let step = drive(&streamed, "w", "rotateSelection", GesturePhase::Stream, fem3d_gumball_tick(&doc, &pair, Fem3dGumballMotion::Rotate { axis: z, angle }), "base");
        streamed = step.transient.expect("the preview stays open");
    }
    let back = fem3d_gumball_tick(&doc, &lone, Fem3dGumballMotion::Rotate { axis: z, angle: 0.0 });
    let done = drive(&streamed, "w", "rotateSelection", GesturePhase::Commit, back, "base");
    assert!(done.committed.is_none(), "a gesture dragged back onto its press commits no command");
    let real = drive(&FemGumballTransient::default(), "w", "rotateSelection", GesturePhase::Once, fem3d_gumball_tick(&doc, &pair, Fem3dGumballMotion::Rotate { axis: z, angle: 0.8 }), "base");
    assert_eq!(real.committed.expect("a rotation that moves commits").1.len(), 1);
}
