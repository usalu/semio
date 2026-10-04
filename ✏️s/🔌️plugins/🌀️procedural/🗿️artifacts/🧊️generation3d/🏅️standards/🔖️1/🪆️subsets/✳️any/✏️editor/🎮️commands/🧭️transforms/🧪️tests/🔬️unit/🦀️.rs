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

/// 🚂️ LAW (audit F5): the gumball rides the shared streamed-gesture runner (`drive_gesture`): a stream persists between
/// dispatches as ONE open transaction of the net leaf and the selection it opened on, a resumed gesture keeps that selection,
/// the release resumes it and commits the net motion under the ref minted at the first tick, and a base moved under it drops
/// it with zero trace; `continues` reads exactly the runner's continuation rule.
#[test]
fn the_gumball_rides_the_shared_gesture_runner() {
    let tick = |ids: &str, offset: [f64; 3]| GumballTick { ids: vec![ids.to_string()], record: translate(&["t"], offset) };
    let first = drive_gesture::<Generation3dGumballTool>(None, "translateSelection", GesturePhase::Stream, Some(tick("a", [1.0, 0.0, 0.0])), "seed", "base");
    assert!(first.committed.is_none());
    let open = first.next.flatten().expect("the stream opens a persisted gesture");
    assert_eq!(open.ids, vec!["a".to_string()]);
    assert!(open.continues("translateSelection", GesturePhase::Commit, "base") && open.continues("translateSelection", GesturePhase::Stream, "base"));
    assert!(!open.continues("translateSelection", GesturePhase::Once, "base") && !open.continues("rotateSelection", GesturePhase::Stream, "base") && !open.continues("translateSelection", GesturePhase::Stream, "moved"));
    let open = drive_gesture::<Generation3dGumballTool>(Some(&open), "translateSelection", GesturePhase::Stream, Some(tick("b", [0.5, 2.0, 0.0])), "seed", "base").next.flatten().expect("the tick keeps the gesture open");
    assert_eq!(open.ids, vec!["a".to_string()], "a resumed gesture keeps the selection it opened on");
    assert_eq!(open.leaf(), Some(&Generation3dMutation::DragTransforms(DragTransforms { targets: vec!["t".into()], dx: 1.5, dy: 2.0, dz: 0.0 })), "ONE net entry");
    let release = drive_gesture::<Generation3dGumballTool>(Some(&open), "translateSelection", GesturePhase::Commit, Some(tick("a", [0.5, 0.0, -1.0])), "seed", "base");
    let (reference, leaves) = release.committed.expect("the release commits");
    assert_eq!(reference, open.transaction, "the ref is the one minted at the first tick");
    assert_eq!(leaves, vec![Generation3dMutation::DragTransforms(DragTransforms { targets: vec!["t".into()], dx: 2.0, dy: 2.0, dz: -1.0 })]);
    assert_eq!(release.next, Some(None), "the committed gesture is cleared");
    let moved = drive_gesture::<Generation3dGumballTool>(Some(&open), "translateSelection", GesturePhase::Commit, Some(tick("a", [0.5, 0.0, 0.0])), "seed", "moved");
    assert!(moved.committed.is_none(), "a base moved under the gesture commits nothing");
    assert_eq!(moved.next, Some(None), "and drops the gesture with zero trace");
}

/// ⚖️ LAW (design §19.1): every gumball tool declares exactly the kind of the relative leaf its motion yields as its intent,
/// so a first grab's history row (the operator splice, then the leaf) is labelled by that leaf; every mesh edit tool declares
/// the `create-widget` of the operator it inserts (audit P4); other tools declare none.
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
    for verb in ["editMeshSelection", "knifeMeshSelection", "deleteSelection"] {
        assert_eq!(<Editor as ArtifactEditor>::tool_intent_kinds(&format!("{}#{verb}", crate::editor::generation3d::GENERATION3D_EDITOR_APP_ID)), &["create-widget"], "a mesh edit's row reads the operator it inserts: {verb}");
    }
    assert!(<Editor as ArtifactEditor>::tool_intent_kinds(&format!("{}#nodeGraphEdit", crate::editor::generation3d::GENERATION3D_EDITOR_APP_ID)).is_empty());
}

/// ⚖️ LAW (audit S4): every gumball refusal is its own NAMED fault code under `generation3d.gumball.` — the code a shell
/// localizes — and the fault it becomes carries that code, never the generic `app.message`; a component set that changed
/// mid-gesture is refused by name.
#[test]
fn every_gumball_refusal_is_a_named_fault_code() {
    let refusals = [
        GumballRefusal::UnknownOperation,
        GumballRefusal::NoShapeSource,
        GumballRefusal::KindUnavailable("k".into()),
        GumballRefusal::NoShapeOutput,
        GumballRefusal::ListOutput,
        GumballRefusal::IdentifierOccupied,
        GumballRefusal::TransformUnavailable("t".into()),
        GumballRefusal::MeshMissing,
        GumballRefusal::NotIndexedMesh,
        GumballRefusal::SelectionChanged,
        GumballRefusal::ComponentSelection("c".into()),
        GumballRefusal::HostEdit("h".into()),
    ];
    let codes: std::collections::BTreeSet<&str> = refusals.iter().map(GumballRefusal::code).collect();
    assert_eq!(codes.len(), refusals.len(), "one code per refusal");
    assert!(codes.iter().all(|code| code.starts_with("generation3d.gumball.")), "{codes:?}");
    for refusal in refusals {
        assert_eq!(Fault::from(refusal.clone()).code.0, refusal.code());
    }
    let snapshot = FlowHostSnapshot::default();
    let changed = validate_component_gesture(&snapshot, &["shape@meshOut#0.face.0".into()], &["shape@meshOut#0.face.1".into()]).expect_err("another component set");
    assert_eq!(changed.code.0, "generation3d.gumball.selection-changed");
    snapshot.retire_cold();
}

/// ⚖️ LAW (design §20.12): the editor declares a localized notice for EVERY gumball refusal code — the table passes the
/// framework's validation (code grammar, unique codes, every locale × terminology cell, one placeholder set) — and a
/// refusal that names a kind fills `{kind}` from the fault's param, never from its English developer detail.
#[test]
fn every_gumball_refusal_code_has_a_localized_notice() {
    use semio_framework_plugin::ArtifactEditor;
    use semio_framework_ui_locale::{Locale, Terminology};
    type Editor = crate::editor::generation3d::Generation3dPlayApp;
    let notices = semio_framework::fault_notice_definitions(<Editor as ArtifactEditor>::fault_notices());
    assert_eq!(semio_framework::validate_fault_notices(&notices), Vec::new(), "the gumball notice table is a valid table");
    let refusals = [
        GumballRefusal::UnknownOperation,
        GumballRefusal::NoShapeSource,
        GumballRefusal::KindUnavailable("brep.mesh.box".into()),
        GumballRefusal::NoShapeOutput,
        GumballRefusal::ListOutput,
        GumballRefusal::IdentifierOccupied,
        GumballRefusal::TransformUnavailable("brep.xform.rotate".into()),
        GumballRefusal::MeshMissing,
        GumballRefusal::NotIndexedMesh,
        GumballRefusal::SelectionChanged,
        GumballRefusal::ComponentSelection("an English developer detail".into()),
        GumballRefusal::HostEdit("an English developer detail".into()),
    ];
    assert_eq!(notices.len(), refusals.len(), "one notice per refusal code");
    for refusal in refusals {
        let fault = Fault::from(refusal.clone());
        for (locale, terminology) in [(Locale::En, Terminology::Native), (Locale::De, Terminology::Native), (Locale::En, Terminology::Reuse), (Locale::De, Terminology::Reuse)] {
            let text = semio_framework::fault_notice_text(&notices, refusal.code(), fault.params.as_deref(), terminology, locale).unwrap_or_else(|| panic!("{} has a complete notice in {locale:?}", refusal.code()));
            assert!(!text.contains('{') && !text.contains("developer detail"), "{text}");
        }
    }
    let kind = semio_framework::fault_notice_text(&notices, "generation3d.gumball.kind-unavailable", Fault::from(GumballRefusal::KindUnavailable("brep.mesh.box".into())).params.as_deref(), Terminology::Native, Locale::De);
    assert_eq!(kind.as_deref(), Some("Die Knotenart brep.mesh.box ist hier nicht verfügbar."));
}
