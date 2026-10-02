//! 🧭️ Laws of the gumball tool's statechart (design §5): a one-shot is ONE committed transaction of ONE relative leaf, a
//! stream keeps ONE open transaction whose single entry is the NET motion of every tick, the release commits it, a cancel
//! leaves zero trace, an identity motion commits nothing; the motion algebra composes offsets, rotations and factors.

use super::*;
use crate::standards::v1::subsets::any::schema::mutations::drag_transforms::DragTransforms;

fn clock(ms: u64) -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: ms, logical: 0 }
}

fn runner(seed: &str) -> ToolMachineRunner<gumball_tool::GumballTool, GumballToolHost> {
    ToolMachineRunner::start("s.procedural.generation3d@1/*#editor#translateSelection", protocol::ActorId(seed.into()), GumballToolContext::default(), GumballToolHost).expect("the gumball tool starts at rest")
}

fn translate(targets: &[&str], offset: [f64; 3]) -> GumballRecord {
    GumballRecord { targets: targets.iter().map(|id| id.to_string()).collect(), motion: GumballMotion::Translate(offset) }
}

fn open_leaf(runner: &ToolMachineRunner<gumball_tool::GumballTool, GumballToolHost>) -> Option<Generation3dMutation> {
    runner.transaction().and_then(|transaction| transaction.entries().iter().find(|(key, _)| key == GENERATION3D_GUMBALL_LEAF_KEY).map(|(_, leaf)| leaf.clone()))
}

/// ⚖️ LAW: a one-shot is ONE committed transaction of the relative leaf, and two one-shots are two transactions.
#[test]
fn a_one_shot_is_one_transaction_of_one_relative_leaf() {
    let mut first = runner("seed-1");
    let ToolStep::Committed(reference, leaves) = first.send(gumball_tool::Event::Once(translate(&["a__gumball_translate"], [1.0, 2.0, 3.0])), clock(1)).expect("the one-shot commits") else { panic!("a one-shot commits") };
    assert!(reference.id.starts_with("tx-") && reference.tool.ends_with("#translateSelection"), "{reference:?}");
    assert_eq!(leaves, vec![Generation3dMutation::DragTransforms(DragTransforms { targets: vec!["a__gumball_translate".into()], dx: 1.0, dy: 2.0, dz: 3.0 })]);
    assert!(first.at_rest());
    let mut second = runner("seed-2");
    let ToolStep::Committed(other, _) = second.send(gumball_tool::Event::Once(translate(&["a__gumball_translate"], [1.0, 0.0, 0.0])), clock(2)).expect("commits") else { panic!("commits") };
    assert_ne!(reference.id, other.id, "two gestures are two transactions");
}

/// ⚖️ LAW: a stream holds ONE open transaction whose single entry is the net offset of every tick; the release folds the
/// last tick in and commits it under the ref minted at the first tick.
#[test]
fn a_stream_commits_the_net_motion_as_one_transaction() {
    let mut tool = runner("seed");
    assert_eq!(tool.send(gumball_tool::Event::Stream(translate(&["t"], [1.0, 0.0, 0.0])), clock(1)).expect("opens"), ToolStep::Open);
    let minted = tool.transaction().expect("open").reference().clone();
    assert_eq!(tool.send(gumball_tool::Event::Stream(translate(&["t"], [0.5, 2.0, 0.0])), clock(2)).expect("ticks"), ToolStep::Open);
    assert_eq!(open_leaf(&tool), Some(Generation3dMutation::DragTransforms(DragTransforms { targets: vec!["t".into()], dx: 1.5, dy: 2.0, dz: 0.0 })), "ONE net entry, never one per tick");
    assert_eq!(tool.transaction().expect("open").entries().len(), 1);
    let ToolStep::Committed(reference, leaves) = tool.send(gumball_tool::Event::Finish(translate(&["t"], [0.5, 0.0, -1.0])), clock(3)).expect("releases") else { panic!("the release commits") };
    assert_eq!(reference, minted, "the ref is the one minted at the first tick");
    assert_eq!(leaves, vec![Generation3dMutation::DragTransforms(DragTransforms { targets: vec!["t".into()], dx: 2.0, dy: 2.0, dz: -1.0 })]);
    assert!(tool.at_rest());
}

/// ⚖️ LAW: a cancelled stream and a host abort leave zero trace; a release whose net motion is the identity commits empty.
#[test]
fn a_cancel_or_an_identity_release_leaves_zero_trace() {
    let mut cancelled = runner("seed");
    cancelled.send(gumball_tool::Event::Stream(translate(&["t"], [1.0, 0.0, 0.0])), clock(1)).expect("opens");
    assert!(matches!(cancelled.send(gumball_tool::Event::Cancel, clock(2)).expect("cancels"), ToolStep::Aborted(_, _)));
    assert!(cancelled.at_rest() && cancelled.transaction().is_none());
    let mut aborted = runner("seed");
    aborted.send(gumball_tool::Event::Stream(translate(&["t"], [1.0, 0.0, 0.0])), clock(1)).expect("opens");
    assert!(matches!(aborted.abort(ToolAbortReason::CaptureLost), ToolStep::Aborted(_, ToolAbortReason::CaptureLost)));
    assert!(aborted.at_rest() && aborted.transaction().is_none());
    let mut identity = runner("seed");
    identity.send(gumball_tool::Event::Stream(translate(&["t"], [1.0, 0.0, 0.0])), clock(1)).expect("opens");
    assert!(matches!(identity.send(gumball_tool::Event::Finish(translate(&["t"], [-1.0, 0.0, 0.0])), clock(2)).expect("releases"), ToolStep::Empty(_)), "a drag back to the start is no edit");
    let mut still = runner("seed");
    assert_eq!(still.send(gumball_tool::Event::Once(translate(&["t"], [0.0; 3])), clock(1)).expect("a no-motion one-shot is guarded"), ToolStep::Idle);
}

/// ⚖️ LAW: the motion algebra — offsets add, factors multiply, rotations compose (`AxisAngle::then`), families never mix,
/// and only an admissible non-identity motion moves; a relative leaf round-trips its motion.
#[test]
fn the_motion_algebra_composes_within_one_family() {
    assert_eq!(GumballMotion::Translate([1.0, 2.0, 3.0]).then(&GumballMotion::Translate([0.5, -2.0, 0.0])), Some(GumballMotion::Translate([1.5, 0.0, 3.0])));
    assert_eq!(GumballMotion::Scale([2.0, 1.0, 1.0]).then(&GumballMotion::Scale([1.5, 3.0, 1.0])), Some(GumballMotion::Scale([3.0, 3.0, 1.0])));
    let quarter = AxisAngle { axis: [0.0, 0.0, 1.0], angle: std::f64::consts::FRAC_PI_2 };
    let Some(GumballMotion::Rotate(half)) = GumballMotion::Rotate(quarter).then(&GumballMotion::Rotate(quarter)) else { panic!("two quarter turns compose") };
    assert!((half.angle - std::f64::consts::PI).abs() < 1e-12, "{half:?}");
    assert_eq!(GumballMotion::Translate([1.0; 3]).then(&GumballMotion::Scale([2.0; 3])), None, "families never mix");
    assert!(!GumballMotion::Translate([0.0; 3]).moves() && !GumballMotion::Scale([1.0; 3]).moves() && !GumballMotion::Scale([0.0, 1.0, 1.0]).moves());
    assert!(!GumballMotion::Rotate(AxisAngle { axis: [0.0; 3], angle: 1.0 }).moves() && !GumballMotion::Rotate(AxisAngle { axis: [0.0, 0.0, 1.0], angle: 0.0 }).moves());
    for motion in [GumballMotion::Translate([1.0, 2.0, 3.0]), GumballMotion::Rotate(quarter), GumballMotion::Scale([2.0, 3.0, 4.0])] {
        assert_eq!(GumballMotion::of_leaf(&motion.leaf(vec!["t".into()])), Some(motion), "{motion:?} round-trips its leaf");
        assert_eq!(motion.operation(), match motion { GumballMotion::Translate(_) => "translate", GumballMotion::Rotate(_) => "rotate", GumballMotion::Scale(_) => "scale" });
    }
}

/// ⚖️ LAW: the `World3dHost` live protocol's phases: absent = one-shot, `stream`, `commit`, `abort` with its reason (absent
/// = `tool`); an unknown phase or abort reason is refused.
#[test]
fn the_host_phases_parse_exactly() {
    assert_eq!(GumballPhase::parse(None, None), Some(GumballPhase::Once));
    assert_eq!(GumballPhase::parse(Some("stream"), None), Some(GumballPhase::Stream));
    assert_eq!(GumballPhase::parse(Some("commit"), None), Some(GumballPhase::Commit));
    assert_eq!(GumballPhase::parse(Some("abort"), Some("blur")), Some(GumballPhase::Abort(ToolAbortReason::Blur)));
    assert_eq!(GumballPhase::parse(Some("abort"), None), Some(GumballPhase::Abort(ToolAbortReason::Tool)));
    assert_eq!(GumballPhase::parse(Some("abort"), Some("sideways")), None);
    assert_eq!(GumballPhase::parse(Some("drag"), None), None);
}

/// 🎚️ One streamed (or released) gumball tick of `extrude` in window `preview-1`, as the live `World3dHost` sends it.
fn streamed(phase: GumballPhase, offset: [f64; 3]) -> GumballDispatch<'static> {
    GumballDispatch { verb: "translateSelection", window: "preview-1", ids: vec!["extrude".into()], motion: GumballMotion::Translate(offset), phase, authoring_seed: "seed", base_revision: [0; 32] }
}

/// ⚖️ LAW (live consumer): while the first grab of a shape streams, the previews paint exactly what its release commits —
/// the overlay splices the transform operator in place of the shape with the NET offset, the marks follow the selection
/// onto that operator, the committed snapshot stays untouched — and a host abort leaves nothing to paint.
#[test]
fn an_open_gesture_previews_exactly_what_its_release_commits() {
    use crate::standards::v1::subsets::any::schema::{example_snapshot, mutations::apply_generation3d_mutation, PROCEDURAL_EXAMPLE_HEX_COLUMN};
    let committed = example_snapshot(PROCEDURAL_EXAMPLE_HEX_COLUMN).expect("the hexagonal column example");
    let operator = "extrude__gumball_translate";
    let marks = super::super::PreviewInteractionMarks { selected: ["extrude".to_string()].into(), ..Default::default() };
    let mut gestures = GumballGestures::default();
    for offset in [[1.0, 0.0, 0.0], [0.5, 2.0, 0.0]] {
        assert!(gestures.dispatch(streamed(GumballPhase::Stream, offset), &committed.host_snapshot).expect("a tick streams").artifact_mutations.is_empty(), "a tick is provisional");
    }
    let (overlay, following) = super::super::generation3d_gumball_preview(&committed, &marks, &gestures).expect("an open gesture paints");
    let previews = |snapshot: &crate::Generation3dSnapshot, id: &str| snapshot.host_snapshot.widgets.iter().find(|widget| crate::widget_id(widget) == id).map(|widget| matches!(widget, Widget::Neuron { preview: true, .. }));
    assert_eq!((previews(&overlay, operator), previews(&overlay, "extrude")), (Some(true), Some(false)), "the operator is painted in place of the shape");
    assert_eq!(previews(&committed, operator), None, "the committed snapshot never sees the open gesture");
    assert_eq!(following.selected, [operator.to_string()].into(), "the marks follow the selection onto the operator");
    let released = gestures.dispatch(streamed(GumballPhase::Commit, [0.0; 3]), &committed.host_snapshot).expect("the release commits");
    assert!(released.transaction.is_some(), "the release is ONE tool transaction");
    let mut landed = committed.clone();
    for row in released.artifact_mutations {
        apply_generation3d_mutation(&mut landed, &row).expect("the committed rows apply");
        row.retire_cold();
    }
    assert_eq!(landed, overlay, "the preview painted exactly what the release committed");
    assert!(super::super::generation3d_gumball_preview(&committed, &marks, &gestures).is_none(), "a released gesture paints nothing more");
    gestures.dispatch(streamed(GumballPhase::Stream, [1.0, 0.0, 0.0]), &committed.host_snapshot).expect("a second gesture opens");
    assert!(gestures.abort("preview-1", ToolAbortReason::Blur));
    assert!(super::super::generation3d_gumball_preview(&committed, &marks, &gestures).is_none(), "an aborted gesture leaves nothing to paint");
    for snapshot in [landed, overlay, committed] {
        snapshot.retire_cold();
    }
}

/// ⚖️ LAW (design §19.1): every gumball tool declares exactly the kind of the relative leaf its motion yields as its intent,
/// so a first grab's history row (the operator splice, then the leaf) is labelled by that leaf; other tools declare none.
#[test]
fn every_gumball_tool_declares_the_leaf_it_yields_as_its_intent() {
    use semio_framework_plugin::ArtifactEditor;
    type Editor = crate::editor::generation3d::Generation3dPlayApp;
    for (verb, motion) in [("translateSelection", GumballMotion::Translate([1.0, 0.0, 0.0])), ("rotateSelection", GumballMotion::Rotate(AxisAngle { axis: [0.0, 0.0, 1.0], angle: 0.5 })), ("scaleSelection", GumballMotion::Scale([2.0, 1.0, 1.0]))] {
        let tool = format!("{}#{verb}", crate::editor::generation3d::GENERATION3D_EDITOR_APP_ID);
        let leaf = motion.leaf(vec!["t".into()]);
        let kind = <Generation3dMutation as protocol::SemanticMutation<crate::Generation3dSnapshot>>::semantics(&leaf).kind;
        assert_eq!(<Editor as ArtifactEditor>::tool_intent_kinds(&tool), &[kind], "{tool}");
    }
    assert!(<Editor as ArtifactEditor>::tool_intent_kinds(&format!("{}#nodeGraphEdit", crate::editor::generation3d::GENERATION3D_EDITOR_APP_ID)).is_empty());
}
