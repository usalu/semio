use super::*;
use crate::standards::v1::subsets::any::schema::{
    PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
    PROCEDURAL_EXAMPLE_SPHERE_TORUS,
};
use crate::viewer::generation3d::modes::view::windows::preview;
use crate::viewer::generation3d::testkit;
use crate::viewer::generation3d::testkit::{app, armed_window_ids, dispatch_with_view, view_shell_view};
use crate::viewer::generation3d::Generation3dViewCommand;
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::ArtifactViewer;

/// 📚️ Every example the dialect publishes, in the order the manifest registers them.
const EVERY_BUNDLED_EXAMPLE: &[&str] = &[
    PROCEDURAL_EXAMPLE_HEX_COLUMN,
    PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
    PROCEDURAL_EXAMPLE_SPHERE_TORUS,
    PROCEDURAL_EXAMPLE_BOX_FILLET,
    PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
    PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
    PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
    PROCEDURAL_EXAMPLE_BOX_SHELL,
];

/// 🕸️ The `graph` node ids the read-only surface reports for one config — the cheapest honest probe
/// of WHICH document the surface is looking at, since `interaction_topology` runs the very same
/// `Generation3dViewedDocument::resolve` the preview render and the `flowEvalTick` chain do.
fn viewed_node_ids(example_id: &str) -> Vec<String> {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = semio_framework_plugin::ArtifactView::new(&snapshot, &history);
    let config = Generation3dViewConfig { active_example_id: example_id.into(), ..Generation3dViewConfig::default() };
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
    let topology = <crate::viewer::generation3d::Generation3dViewer as ArtifactViewer>::interaction_topology(&doc, &cfg);
    let ids = topology.domains.get("graph").expect("graph domain").ordered.iter().filter(|node| node.granularity == "node" && node.parent.is_none()).map(|node| node.id.clone()).collect();
    snapshot.retire_cold();
    ids
}

/// 🎨️ All eight bundled examples really switch what the read-only surface is looking at. The
/// document is a constant here — it is the CONFIG that moves — which is exactly the difference from
/// the sibling surface's own switch (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn every_bundled_example_switches_the_viewed_document() {
    let _serial = testkit::lock();
    let opened = viewed_node_ids("");
    assert!(!opened.is_empty(), "the opened document must report its own graph nodes");
    let mut seen: Vec<(&str, Vec<String>)> = Vec::new();
    for example_id in EVERY_BUNDLED_EXAMPLE {
        let nodes = viewed_node_ids(example_id);
        assert!(!nodes.is_empty(), "{example_id} must report the example's own graph nodes");
        seen.push((example_id, nodes));
    }
    println!("[STATS] viewer viewed-document switch examples={} opened={opened:?}", seen.len());
    for (example_id, nodes) in &seen {
        println!("[STATS] viewer example {example_id} nodes={nodes:?}");
    }
    // 🧾️ Seven of the eight are structurally distinct graphs; `hexagonal-mushroom-column` IS the
    // default document, so it is the one whose node set legitimately equals the opened one.
    for (example_id, nodes) in &seen {
        if *example_id == PROCEDURAL_EXAMPLE_HEX_COLUMN {
            assert_eq!(nodes, &opened, "the hex column example is the default document");
        } else {
            assert_ne!(nodes, &opened, "{example_id} must not be looked at as the opened document");
        }
    }
}

/// 👁️ The gesture really dispatches through the interactive-job pipeline, publishes on the CONFIG
/// lane only, leaves the document byte-identical, and re-arms the attached preview's chain from its
/// own emit rather than waiting for a host `refresh-ui`.
///
/// 🧵️ One FRESH surface per example, deliberately: `preview_eval::rearm_attached_previews` arms
/// through the retained session's per-window latch, so a second switch on a session whose first tick
/// is still outstanding correctly arms NOTHING — that is the peer lane's own contract, pinned
/// separately by `consecutive_switches_arm_once_until_the_chain_answers` below.
#[semio_framework_async_macros::async_test]
async fn every_bundled_example_dispatches_live_rearms_the_preview_and_never_mutates_the_document() {
    let _serial = testkit::lock();
    let shell_view = view_shell_view("view-preview");
    for example_id in EVERY_BUNDLED_EXAMPLE.iter().chain(std::iter::once(&"")) {
        let mut app = app().await;
        let before = testkit::snapshot(&app);
        let command = Generation3dViewCommand::SetActiveExample(SetActiveExample { example_id: (*example_id).into() });
        assert_eq!(command.command_id(), "setActiveExample");
        let receipt = dispatch_with_view(&mut app, command, shell_view.clone()).await.expect("the viewer example switch settles");
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample({example_id}) faulted: {:?}", receipt.lanes);
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Artifact), "a viewer example switch must never publish on the artifact lane");
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Draft), "a viewer example switch must never publish on the draft lane");
        assert!(receipt.lanes.contains(&TypedOperationResultLane::Config), "the picked example must land on the viewer's own config lane: {:?}", receipt.lanes);
        let armed = armed_window_ids(&receipt.effects);
        println!("[STATS] viewer setActiveExample {example_id:?} lanes={:?} armed={armed:?}", receipt.lanes);
        assert_eq!(armed, vec!["view-preview".to_string()], "the switch owes the attached preview its own re-armed tick ({example_id})");
        assert_eq!(testkit::snapshot(&app), before, "viewer setActiveExample({example_id}) must not mutate the document");
    }
}

/// 🧵️ The re-arm is LATCHED per window: a switch arms the attached preview once, and a second switch
/// dispatched before that tick is answered adds no duplicate — otherwise every pick through the
/// navbar would stack another chain on a surface that already owes one.
#[semio_framework_async_macros::async_test]
async fn consecutive_switches_arm_once_until_the_chain_answers() {
    let _serial = testkit::lock();
    let mut app = app().await;
    let shell_view = view_shell_view("view-preview");
    let first = dispatch_with_view(&mut app, Generation3dViewCommand::SetActiveExample(SetActiveExample { example_id: PROCEDURAL_EXAMPLE_BOX_SHELL.into() }), shell_view.clone()).await.expect("first switch");
    let second = dispatch_with_view(&mut app, Generation3dViewCommand::SetActiveExample(SetActiveExample { example_id: PROCEDURAL_EXAMPLE_RECTANGLE_WIRE.into() }), shell_view).await.expect("second switch");
    println!("[STATS] viewer switch latch first={:?} second={:?}", armed_window_ids(&first.effects), armed_window_ids(&second.effects));
    assert_eq!(armed_window_ids(&first.effects), vec!["view-preview".to_string()]);
    assert!(armed_window_ids(&second.effects).is_empty(), "an outstanding tick is not armed twice");
    assert!(second.lanes.contains(&TypedOperationResultLane::Config), "the second switch still lands its own config edit");
}

/// 🚫️ An id the dialect never published is refused, not silently applied — the picker can only ever
/// offer declared ids, so anything else is a wire defect worth naming.
#[semio_framework_async_macros::async_test]
async fn an_unpublished_example_id_is_refused() {
    let _serial = testkit::lock();
    let mut app = app().await;
    let before = testkit::snapshot(&app);
    let shell_view = view_shell_view("view-preview");
    let receipt = dispatch_with_view(&mut app, Generation3dViewCommand::SetActiveExample(SetActiveExample { example_id: "not-a-real-example".into() }), shell_view).await;
    let refused = match receipt {
        Err(fault) => fault.message,
        Ok(receipt) => {
            assert!(receipt.lanes.contains(&TypedOperationResultLane::Fault), "an unpublished example id must fault: {:?}", receipt.lanes);
            "fault lane".to_string()
        }
    };
    println!("[STATS] viewer setActiveExample refusal={refused}");
    assert_eq!(testkit::snapshot(&app), before, "a refused example switch leaves the document untouched");
}

/// 🪟️ The window the switch re-arms is the one the SHELL says is attached; with no roster at all
/// there is nothing to publish into and the switch arms nothing rather than spinning a chain that
/// cannot land.
#[semio_framework_async_macros::async_test]
async fn an_unattached_surface_arms_no_tick() {
    let _serial = testkit::lock();
    let mut app = app().await;
    let receipt = dispatch_with_view(&mut app, Generation3dViewCommand::SetActiveExample(SetActiveExample { example_id: PROCEDURAL_EXAMPLE_BOX_SHELL.into() }), semio_framework_plugin::ViewModel::default())
        .await
        .expect("the switch settles without an attached preview");
    assert!(armed_window_ids(&receipt.effects).is_empty(), "no attached preview window means no armed tick");
    assert_eq!(preview::WINDOW_KIND_ID, "procedural-view-preview");
}
