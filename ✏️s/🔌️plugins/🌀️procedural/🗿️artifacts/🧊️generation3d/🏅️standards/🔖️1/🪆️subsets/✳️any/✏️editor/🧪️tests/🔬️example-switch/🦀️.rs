use super::*;
use crate::editor::generation3d::testkit::{self, app_with_registry};
use crate::standards::v1::subsets::any::schema::{
    example_snapshot, snapshot::Generation3dSnapshotRead, PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
    PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_SPHERE_TORUS,
};
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::PluginApp;

const EXAMPLE_SWITCH_ORDER: [&str; 8] = [
    PROCEDURAL_EXAMPLE_HEX_COLUMN,
    PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
    PROCEDURAL_EXAMPLE_SPHERE_TORUS,
    PROCEDURAL_EXAMPLE_BOX_FILLET,
    PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
    PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
    PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
    PROCEDURAL_EXAMPLE_BOX_SHELL,
];

/// 🪪️ The node ids the FLOW WINDOW actually publishes — read off the rendered surface, never off the
/// document, because the live defect is a surface that keeps the previous fixture.
fn published_node_ids(graph: &str) -> std::collections::BTreeSet<String> {
    let scene = semio_framework_plugin::testkit::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(graph).expect("node-graph scene decodes off the rendered flow surface");
    scene.nodes.iter().map(|node| node.id.clone()).collect()
}

fn authored_node_ids(example_id: &str) -> std::collections::BTreeSet<String> {
    let read = Generation3dSnapshotRead::new(example_snapshot(example_id).expect("bundled example snapshot"));
    read.fixture.widgets.iter().map(crate::widget_id).map(str::to_string).collect()
}

/// ⚖️ LAW: picking an example REPUBLISHES the flow window's graph. Boot #11 of
/// `📓️runtime-verification-2026-09-09.md` saw the picker label change to "Box Shell Preview" while the
/// flow window kept the hexagonal-mushroom-column node ids and no fault appeared anywhere.
#[semio_framework_async_macros::async_test]
async fn set_active_example_republishes_the_flow_window_graph() {
    let _serial = test_support::lock();
    let mut app = app_with_registry().await;
    let (flow_view, _preview_view) = testkit::shell_views(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let before = published_node_ids(&testkit::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    assert_eq!(before, authored_node_ids(PROCEDURAL_EXAMPLE_HEX_COLUMN), "the boot flow window publishes the hex-column graph");
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_SHELL }).into()), &semio_framework_plugin::testkit::meta("local")).await.expect("setActiveExample dispatches");
    let receipt = testkit::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample published a fault lane");
    eprintln!("[DEBUG] example switch receipt: lanes={:?} ui_scope={:?} completions={} effects={}", receipt.lanes, receipt.ui_scope, receipt.completions, receipt.effects.len());
    let after = published_node_ids(&testkit::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    eprintln!("[DEBUG] flow window node ids after the switch: {after:?}");
    assert_eq!(after, authored_node_ids(PROCEDURAL_EXAMPLE_BOX_SHELL), "the flow window kept the previous example's graph");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: the whole picker roster, in one session, in order — every hop publishes ITS OWN graph and
/// retires the previous fixture completely (no leaked node from the example before it).
#[semio_framework_async_macros::async_test]
async fn every_example_switch_publishes_its_own_graph_without_leaking_the_previous_one() {
    let _serial = test_support::lock();
    let mut app = app_with_registry().await;
    let (flow_view, _preview_view) = testkit::shell_views(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let mut previous = PROCEDURAL_EXAMPLE_HEX_COLUMN;
    for example_id in EXAMPLE_SWITCH_ORDER {
        app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": example_id }).into()), &semio_framework_plugin::testkit::meta("local")).await.expect("setActiveExample dispatches");
        let receipt = testkit::settle(&mut app).await;
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "{example_id} published a fault lane");
        let published = published_node_ids(&testkit::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
        let authored = authored_node_ids(example_id);
        assert_eq!(published, authored, "{example_id}: the flow window did not republish this example's graph");
        for leaked in authored_node_ids(previous).difference(&authored) {
            assert!(!published.contains(leaked), "{example_id}: the previous example's node {leaked} survived the switch");
        }
        eprintln!("[DEBUG] example switch {previous} -> {example_id}: published {} nodes ui_scope={:?}", published.len(), receipt.ui_scope);
        previous = example_id;
    }
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: the SERVED dispatch — the picker's action arrives with the shell's own live view attached and
/// the FLOW window current, never with the bare local meta a unit test defaults to. Both windows must
/// end up on the new example: the flow window republishes its graph, and the preview window's armed
/// evaluation chain restarts against the new fixture instead of holding the previous example's result
/// (`📓️runtime-verification-2026-09-09.md` boot #11).
#[semio_framework_async_macros::async_test]
async fn a_shell_dispatched_example_switch_republishes_both_windows_and_rearms_the_eval_chain() {
    use crate::editor::generation3d::modes::edit::windows::preview::transient::Generation3dPreviewWindowTransientOwner;
    let _serial = test_support::lock();
    let mut app = app_with_registry().await;
    let (flow_view, preview_view) = testkit::shell_views(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    testkit::drain_armed_flow_eval_ticks(&mut app, &flow_view).await;
    let evaluated = app
        .window_transient_snapshot(&preview_view)
        .expect("preview window transient snapshot")
        .and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.clone()));
    eprintln!("[DEBUG] boot preview eval text bytes={:?}", evaluated.as_deref().map(str::len));
    let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::testkit::meta("local") };
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_SHELL }).into()), &action_meta).await.expect("the shell dispatches setActiveExample under the flow window");
    let receipt = testkit::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "a shell-dispatched setActiveExample published a fault lane");
    let published = published_node_ids(&testkit::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    assert_eq!(published, authored_node_ids(PROCEDURAL_EXAMPLE_BOX_SHELL), "the shell-dispatched switch did not republish the flow window's graph");
    let armed = app.pending_effects(Some(&flow_view)).await;
    eprintln!("[DEBUG] shell switch: lanes={:?} ui_scope={:?} own effects={} armed after switch={}", receipt.lanes, receipt.ui_scope, receipt.effects.len(), armed.len());
    assert!(
        armed.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")),
        "the switched document must leave the preview window's evaluation chain armed, got {armed:?}"
    );
    testkit::drain_armed_flow_eval_ticks(&mut app, &flow_view).await;
    let after = app
        .window_transient_snapshot(&preview_view)
        .expect("preview window transient snapshot")
        .and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.clone()));
    eprintln!("[DEBUG] switched preview eval text bytes={:?}", after.as_deref().map(str::len));
    assert!(after.is_some(), "the preview window published no evaluation for the switched example");
    assert_ne!(after, evaluated, "the preview window kept the PREVIOUS example's evaluation after the switch");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: the switch arms its OWN evaluation restart. Before this law the gesture published
/// `effects=0` and the preview's recovery depended entirely on the host reaching
/// `Generation3dPlayApp::pending_effects` through a later `refresh-ui` — a dependency that fails
/// silently, with no fault anywhere, the moment that refresh is narrowed or lost.
#[semio_framework_async_macros::async_test]
async fn set_active_example_arms_every_attached_preview_windows_evaluation_chain() {
    let _serial = test_support::lock();
    let mut app = app_with_registry().await;
    let (flow_view, _preview_view) = testkit::shell_views(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::testkit::meta("local") };
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_SHELL }).into()), &action_meta).await.expect("setActiveExample dispatches");
    let receipt = testkit::settle(&mut app).await;
    let armed: Vec<&str> = receipt
        .effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "flowEvalTick" => args.as_ref().and_then(|args| args.get("windowId")).and_then(|value| value.as_str()),
            _ => None,
        })
        .collect();
    eprintln!("[DEBUG] setActiveExample armed ticks for {armed:?} out of {} published effects", receipt.effects.len());
    assert_eq!(armed, vec![edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW], "the switch must arm the attached preview window's own evaluation chain");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}
