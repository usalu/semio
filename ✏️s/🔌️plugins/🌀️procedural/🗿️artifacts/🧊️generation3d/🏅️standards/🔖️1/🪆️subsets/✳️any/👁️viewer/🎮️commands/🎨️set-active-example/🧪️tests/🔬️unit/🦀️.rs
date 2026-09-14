use super::*;
use crate::standards::v1::subsets::any::schema::{
    PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
    PROCEDURAL_EXAMPLE_SPHERE_TORUS,
};
use crate::viewer::generation3d::modes::view::windows::preview;
use crate::viewer::generation3d::unit_tests::context;
use crate::viewer::generation3d::unit_tests::context::{app, dispatch_with_view, run_actions, view_shell_view};
use crate::viewer::generation3d::Generation3dViewCommand;
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::ArtifactViewer;

/// 📜️ The language-agnostic picker table both surfaces answer — the viewer's own half is `viewer`.
const EXAMPLE_SWITCH_FIXTURE_JSON: &str = include_str!("../../../../../🧫️fixtures/🎨️example-switch.json");

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
///
/// 🕳️ `None` is "nothing picked", `Some("")` the picker's own `No example` row — two different
/// states, and the whole point of the option on the config leaf.
fn viewed_node_ids(example_id: Option<&str>) -> Vec<String> {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = semio_framework_plugin::ArtifactView::new(&snapshot, &history);
    let config = Generation3dViewConfig { active_example_id: example_id.map(str::to_string), ..Generation3dViewConfig::default() };
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
    let topology = <crate::viewer::generation3d::Generation3dViewer as ArtifactViewer>::interaction_topology(&doc, &cfg);
    let ids = topology.domains.get("graph").expect("graph domain").ordered.iter().filter(|node| node.granularity == "node" && node.parent.is_none()).map(|node| node.id.clone()).collect();
    snapshot.retire_cold();
    ids
}

/// 🕳️ The picker's `No example` row shows NO example — on the read-only surface exactly as on the
/// sibling one. It used to resolve to the document this session opened, which in the playground IS a
/// bundled example (the hexagonal mushroom column), so the row that promises no example painted its
/// three meshes (`meshesLen 3641`, measured on the served viewer 2026-09-13). Never picking anything
/// is a DIFFERENT state and still shows the opened document
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_no_example_row_clears_the_viewed_document_and_never_picking_does_not() {
    let _serial = context::lock();
    let fixture = dsl::json::parse(EXAMPLE_SWITCH_FIXTURE_JSON).expect("the example-switch fixture parses");
    let viewer = fixture.get("viewer").cloned().expect("the fixture declares the viewer's picker states");
    assert_eq!(viewer.get("windowKind").and_then(dsl::json::Value::as_str), Some(preview::WINDOW_KIND_ID));
    let states = viewer.get("states").cloned().expect("states");
    let states = states.as_array().expect("states is a table");
    assert_eq!(states.len(), 3, "the picker has three states, not two");
    let opened = viewed_node_ids(None);
    for state in states {
        let id = state.get("id").and_then(dsl::json::Value::as_str).expect("state id");
        let picked = state.get("activeExampleId").and_then(dsl::json::Value::as_str);
        let nodes = viewed_node_ids(picked);
        println!("[STATS] viewer picker state {id} activeExampleId={picked:?} nodes={nodes:?}");
        match state.get("viewed").and_then(dsl::json::Value::as_str).expect("viewed") {
            "opened" => assert_eq!(nodes, opened, "{id}: never picking anything keeps showing the opened document"),
            "empty" => assert!(nodes.is_empty(), "{id}: the `No example` row must clear the viewed document, not load one"),
            example_id => {
                assert_eq!(nodes, viewed_node_ids(Some(example_id)), "{id}: a named example is looked at as itself");
                assert!(!nodes.is_empty(), "{id}: a named example reports its own graph");
            }
        }
    }
    assert_ne!(viewed_node_ids(Some("")), viewed_node_ids(Some(PROCEDURAL_EXAMPLE_HEX_COLUMN)), "`No example` must not resolve to the default document");
}

/// 🎨️ All eight bundled examples really switch what the read-only surface is looking at. The
/// document is a constant here — it is the CONFIG that moves — which is exactly the difference from
/// the sibling surface's own switch (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn every_bundled_example_switches_the_viewed_document() {
    let _serial = context::lock();
    let opened = viewed_node_ids(None);
    assert!(!opened.is_empty(), "the opened document must report its own graph nodes");
    let mut seen: Vec<(&str, Vec<String>)> = Vec::new();
    for example_id in EVERY_BUNDLED_EXAMPLE {
        let nodes = viewed_node_ids(Some(example_id));
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
/// lane only, leaves the document byte-identical, and carries the attached preview's `previewEval` run
/// start on its own emit rather than waiting for a host `refresh-ui`.
///
/// 🧵️ One FRESH surface per example, deliberately: the run start is requested through the link's
/// one-request latch, so a second switch while the first start stands correctly carries NOTHING —
/// pinned separately by `consecutive_switches_request_one_start_until_the_run_answers` below.
#[semio_framework_async_macros::async_test]
async fn every_bundled_example_dispatches_live_rearms_the_preview_and_never_mutates_the_document() {
    let _serial = context::lock();
    let shell_view = view_shell_view("view-preview");
    for example_id in EVERY_BUNDLED_EXAMPLE.iter().chain(std::iter::once(&"")) {
        let mut app = app().await;
        let before = context::snapshot(&app);
        let command = Generation3dViewCommand::SetActiveExample(SetActiveExample { example_id: (*example_id).into() });
        assert_eq!(command.command_id(), "setActiveExample");
        let receipt = dispatch_with_view(&mut app, command, shell_view.clone()).await.expect("the viewer example switch settles");
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample({example_id}) faulted: {:?}", receipt.lanes);
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Artifact), "a viewer example switch must never publish on the artifact lane");
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Draft), "a viewer example switch must never publish on the draft lane");
        assert!(receipt.lanes.contains(&TypedOperationResultLane::Config), "the picked example must land on the viewer's own config lane: {:?}", receipt.lanes);
        let carried = run_actions(&receipt.effects);
        println!("[STATS] viewer setActiveExample {example_id:?} lanes={:?} carried={carried:?}", receipt.lanes);
        assert_eq!(carried, vec![semio_framework_plugin::TOOL_RUN_START_ACTION_ID.to_string()], "the switch carries the attached preview its own run start ({example_id})");
        assert_eq!(context::snapshot(&app), before, "viewer setActiveExample({example_id}) must not mutate the document");
    }
}

/// 🧵️ One gesture costs at most ONE run start: each switch carries its own, and the framework admits
/// exactly one run for both — the second start meets the live run (`toolRun.busy`) and the run it
/// joins evaluates the example picked last.
#[semio_framework_async_macros::async_test]
async fn consecutive_switches_carry_one_start_each_and_settle_one_run() {
    let _serial = context::lock();
    let mut app = app().await;
    let shell_view = view_shell_view("view-preview");
    let first = dispatch_with_view(&mut app, Generation3dViewCommand::SetActiveExample(SetActiveExample { example_id: PROCEDURAL_EXAMPLE_BOX_SHELL.into() }), shell_view.clone()).await.expect("first switch");
    let second = dispatch_with_view(&mut app, Generation3dViewCommand::SetActiveExample(SetActiveExample { example_id: PROCEDURAL_EXAMPLE_RECTANGLE_WIRE.into() }), shell_view.clone()).await.expect("second switch");
    println!("[STATS] viewer switch latch first={:?} second={:?}", run_actions(&first.effects), run_actions(&second.effects));
    assert_eq!(run_actions(&first.effects), vec![semio_framework_plugin::TOOL_RUN_START_ACTION_ID.to_string()]);
    assert_eq!(run_actions(&second.effects), vec![semio_framework_plugin::TOOL_RUN_START_ACTION_ID.to_string()], "the second gesture carries at most its own one start");
    let effects: Vec<_> = first.effects.iter().chain(&second.effects).cloned().collect();
    let run = crate::viewer::generation3d::unit_tests::context::drive_preview_run(&mut app, &shell_view, &effects).await;
    assert_eq!(run.state.as_deref(), Some("finalized"), "both starts settle into one finalized run: {run:?}");
    assert!(second.lanes.contains(&TypedOperationResultLane::Config), "the second switch still lands its own config edit");
}

/// 🚫️ An id the dialect never published is refused, not silently applied — the picker can only ever
/// offer declared ids, so anything else is a wire defect worth naming.
#[semio_framework_async_macros::async_test]
async fn an_unpublished_example_id_is_refused() {
    let _serial = context::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
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
    assert_eq!(context::snapshot(&app), before, "a refused example switch leaves the document untouched");
}

/// 🪟️ The windows the switch owes are the ones the SHELL says are attached; with no roster at all there
/// is nothing to publish into and the switch starts nothing rather than spinning a run that cannot land.
#[semio_framework_async_macros::async_test]
async fn an_unattached_surface_starts_no_run() {
    let _serial = context::lock();
    let mut app = app().await;
    let receipt = dispatch_with_view(&mut app, Generation3dViewCommand::SetActiveExample(SetActiveExample { example_id: PROCEDURAL_EXAMPLE_BOX_SHELL.into() }), semio_framework_plugin::ViewModel::default())
        .await
        .expect("the switch settles without an attached preview");
    assert!(run_actions(&receipt.effects).is_empty(), "no attached preview window means no run start");
    assert_eq!(preview::WINDOW_KIND_ID, "procedural-view-preview");
}
