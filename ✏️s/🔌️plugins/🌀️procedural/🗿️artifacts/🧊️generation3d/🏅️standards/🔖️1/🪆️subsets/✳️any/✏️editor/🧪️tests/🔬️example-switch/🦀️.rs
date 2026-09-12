use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app_with_registry};
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
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(graph).expect("node-graph scene decodes off the rendered flow surface");
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
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow_view, _preview_view) = context::shell_views(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let before = published_node_ids(&context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    assert_eq!(before, authored_node_ids(PROCEDURAL_EXAMPLE_HEX_COLUMN), "the boot flow window publishes the hex-column graph");
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_SHELL }).into()), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("setActiveExample dispatches");
    let receipt = context::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample published a fault lane");
    eprintln!("[DEBUG] example switch receipt: lanes={:?} ui_scope={:?} completions={} effects={}", receipt.lanes, receipt.ui_scope, receipt.completions, receipt.effects.len());
    let after = published_node_ids(&context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    eprintln!("[DEBUG] flow window node ids after the switch: {after:?}");
    assert_eq!(after, authored_node_ids(PROCEDURAL_EXAMPLE_BOX_SHELL), "the flow window kept the previous example's graph");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: the whole picker roster, in one session, in order — every hop publishes ITS OWN graph and
/// retires the previous fixture completely (no leaked node from the example before it).
#[semio_framework_async_macros::async_test]
async fn every_example_switch_publishes_its_own_graph_without_leaking_the_previous_one() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow_view, _preview_view) = context::shell_views(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let mut previous = PROCEDURAL_EXAMPLE_HEX_COLUMN;
    for example_id in EXAMPLE_SWITCH_ORDER {
        app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": example_id }).into()), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("setActiveExample dispatches");
        let receipt = context::settle(&mut app).await;
        assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "{example_id} published a fault lane");
        let published = published_node_ids(&context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
        let authored = authored_node_ids(example_id);
        assert_eq!(published, authored, "{example_id}: the flow window did not republish this example's graph");
        for leaked in authored_node_ids(previous).difference(&authored) {
            assert!(!published.contains(leaked), "{example_id}: the previous example's node {leaked} survived the switch");
        }
        eprintln!("[DEBUG] example switch {previous} -> {example_id}: published {} nodes ui_scope={:?}", published.len(), receipt.ui_scope);
        previous = example_id;
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: the SERVED dispatch — the picker's action arrives with the shell's own live view attached and
/// the FLOW window current, never with the bare local meta a unit test defaults to. Both windows must
/// end up on the new example: the flow window republishes its graph, and the preview window's armed
/// evaluation chain restarts against the new fixture instead of holding the previous example's result
/// (`📓️runtime-verification-2026-09-09.md` boot #11).
#[semio_framework_async_macros::async_test]
async fn a_shell_dispatched_example_switch_republishes_both_windows_and_rearms_the_eval_chain() {
    use crate::editor::generation3d::modes::edit::windows::preview::transient::Generation3dPreviewWindowTransientOwner;
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow_view, preview_view) = context::shell_views(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    context::drain_armed_flow_eval_ticks(&mut app, &flow_view).await;
    let evaluated = app
        .window_transient_snapshot(&preview_view)
        .expect("preview window transient snapshot")
        .and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.clone()));
    eprintln!("[DEBUG] boot preview eval text bytes={:?}", evaluated.as_deref().map(str::len));
    let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("local") };
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_SHELL }).into()), &action_meta).await.expect("the shell dispatches setActiveExample under the flow window");
    let receipt = context::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "a shell-dispatched setActiveExample published a fault lane");
    let published = published_node_ids(&context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    assert_eq!(published, authored_node_ids(PROCEDURAL_EXAMPLE_BOX_SHELL), "the shell-dispatched switch did not republish the flow window's graph");
    // 🔒️ The switch arms from its OWN emit, and the retained session's per-window latch is what
    // keeps the following refresh poll from stacking a SECOND chain on top of it — one pending tick
    // per (instance, preview window), whoever asked for it.
    let polled = app.pending_effects(Some(&flow_view)).await;
    eprintln!("[DEBUG] shell switch: lanes={:?} ui_scope={:?} own effects={} polled after switch={}", receipt.lanes, receipt.ui_scope, receipt.effects.len(), polled.len());
    assert!(
        receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")),
        "the switched document must leave the preview window's evaluation chain armed, got {:?}",
        receipt.effects
    );
    assert!(
        !polled.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")),
        "the refresh poll must add no second chain to a window that already owes a tick, got {polled:?}"
    );
    context::drain_armed_flow_eval_ticks_from(&mut app, &flow_view, &receipt.effects).await;
    let after = app
        .window_transient_snapshot(&preview_view)
        .expect("preview window transient snapshot")
        .and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.clone()));
    eprintln!("[DEBUG] switched preview eval text bytes={:?}", after.as_deref().map(str::len));
    assert!(after.is_some(), "the preview window published no evaluation for the switched example");
    assert_ne!(after, evaluated, "the preview window kept the PREVIOUS example's evaluation after the switch");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: the switch arms its OWN evaluation restart. Before this law the gesture published
/// `effects=0` and the preview's recovery depended entirely on the host reaching
/// `Generation3dPlayApp::pending_effects` through a later `refresh-ui` — a dependency that fails
/// silently, with no fault anywhere, the moment that refresh is narrowed or lost.
#[semio_framework_async_macros::async_test]
async fn set_active_example_arms_every_attached_preview_windows_evaluation_chain() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow_view, _preview_view) = context::shell_views(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("local") };
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_SHELL }).into()), &action_meta).await.expect("setActiveExample dispatches");
    let receipt = context::settle(&mut app).await;
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
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

//#region 🎨️FixtureLaw
/// ⚖️ The ONE language-agnostic RUNTIME fixture the picker answers. `📚️example-picker.json`
/// (`🛂️manifest`) says which examples a surface may OFFER; this says what picking one must DO —
/// republish exactly that example's authored widget ids on the flow window, leak none of the
/// previous example's, re-arm every attached preview window's chain from the gesture's own emit, and
/// leave no preview holding the previous example's evaluation.
const EXAMPLE_SWITCH_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🎨️example-switch.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExampleSwitchFixture {
    format: String,
    version: u8,
    window_kinds: std::collections::BTreeMap<String, String>,
    examples: std::collections::BTreeMap<String, Vec<String>>,
    rows: Vec<ExampleSwitchRow>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExampleSwitchAttachedWindow {
    id: String,
    kind: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExampleSwitchRow {
    id: String,
    attached: Vec<ExampleSwitchAttachedWindow>,
    picks: Vec<String>,
    published: Vec<String>,
    #[serde(default)]
    retired: Vec<String>,
    armed: Vec<String>,
    #[serde(default)]
    preview_evaluation_changed: bool,
}

fn example_switch_fixture() -> ExampleSwitchFixture {
    let fixture: ExampleSwitchFixture = serde_json::from_str(EXAMPLE_SWITCH_FIXTURE_JSON).expect("example switch fixture");
    assert_eq!(fixture.format, "semio.generation3d.example-switch");
    assert_eq!(fixture.version, 1);
    fixture
}

/// 🪟️ The shell roster one row declares, plus the flow window the picker always dispatches under.
fn example_switch_views(fixture: &ExampleSwitchFixture, attached: &[ExampleSwitchAttachedWindow]) -> (semio_framework_plugin::ViewModel, semio_framework_plugin::ViewModel) {
    let flow_kind = fixture.window_kinds.get("flow").expect("fixture flow window kind").clone();
    let mut roster = vec![semio_framework_plugin::ViewWindowInstance { id: flow_window::GENERATION_3D_PLAY_WINDOW_MAIN.into(), window_kind_id: flow_kind }];
    for window in attached {
        roster.push(semio_framework_plugin::ViewWindowInstance {
            id: window.id.clone(),
            window_kind_id: fixture.window_kinds.get(&window.kind).unwrap_or_else(|| panic!("fixture window kind {}", window.kind)).clone(),
        });
    }
    let view = semio_framework_plugin::ViewModel { window_instances: roster, ..Default::default() };
    let flow = view.for_window_instance(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN).expect("flow window instance");
    let first_preview = attached.first().map(|window| view.for_window_instance(&window.id).expect("preview window instance")).unwrap_or_else(|| flow.clone());
    (flow, first_preview)
}

fn armed_window_ids(effects: &[Effect]) -> Vec<String> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "flowEvalTick" => args.as_ref().and_then(|args| args.get("windowId")).and_then(|value| value.as_str()).map(str::to_string),
            _ => None,
        })
        .collect()
}

fn preview_eval_text(app: &mut crate::editor::generation3d::unit_tests::context::Generation3dApp, preview_view: &semio_framework_plugin::ViewModel) -> Option<String> {
    use crate::editor::generation3d::modes::edit::windows::preview::transient::Generation3dPreviewWindowTransientOwner;
    app.window_transient_snapshot(preview_view).ok().flatten().and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.clone()))
}

/// ⚖️ LAW: every row of `🎨️example-switch.json`, replayed on a live editor instance under the shell's
/// own flow-window view — the graph the flow window publishes, the widgets it must have retired, the
/// preview chains the gesture itself armed, and whether the preview's retained evaluation moved.
#[semio_framework_async_macros::async_test]
async fn every_example_switch_row_of_the_fixture_holds() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = example_switch_fixture();
    for row in &fixture.rows {
        let mut app = app_with_registry().await;
        let (flow_view, preview_view) = example_switch_views(&fixture, &row.attached);
        let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("local") };
        context::drain_armed_flow_eval_ticks(&mut app, &flow_view).await;
        let before_eval = preview_eval_text(&mut app, &preview_view);
        let mut armed = Vec::new();
        for pick in &row.picks {
            app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": pick }).into()), &action_meta).await.expect("setActiveExample dispatches");
            let receipt = context::settle(&mut app).await;
            assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "{}: picking {pick:?} published a fault lane", row.id);
            armed = armed_window_ids(&receipt.effects);
            context::drain_armed_flow_eval_ticks_from(&mut app, &flow_view, &receipt.effects).await;
        }
        let published = published_node_ids(&context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
        let expected: std::collections::BTreeSet<String> = row.published.iter().cloned().collect();
        eprintln!("[DEBUG] example-switch row {} picks={:?} published={published:?} armed={armed:?}", row.id, row.picks);
        assert_eq!(published, expected, "{}: the flow window did not publish exactly this row's graph", row.id);
        for leaked in &row.retired {
            assert!(!published.contains(leaked), "{}: the previous example's widget {leaked} survived the switch", row.id);
        }
        // 🔁️ WHICH windows, not in which order: the retained ladder drains `Emit::effects` with
        // `pop()`, so the published order is the roster's reverse and carries no meaning.
        let mut armed_sorted = armed.clone();
        armed_sorted.sort();
        let mut expected_armed = row.armed.clone();
        expected_armed.sort();
        assert_eq!(armed_sorted, expected_armed, "{}: the switch armed the wrong preview chains", row.id);
        if !row.picks.is_empty() {
            let after_eval = preview_eval_text(&mut app, &preview_view);
            if row.preview_evaluation_changed {
                assert_ne!(after_eval, before_eval, "{}: the preview window kept the PREVIOUS example's evaluation", row.id);
            } else {
                assert_eq!(after_eval, before_eval, "{}: a no-op pick moved the preview's retained evaluation", row.id);
            }
        }
        // 🪪️ Every authored id of the picked example, and nothing else — the fixture's own table is
        // the oracle the TypeScript twin reads too, so a drifting DSL asset fails here first.
        if let Some(authored) = row.picks.last().and_then(|pick| fixture.examples.get(pick)) {
            let authored: std::collections::BTreeSet<String> = authored.iter().cloned().collect();
            assert_eq!(expected, authored, "{}: the row's expectation disagrees with the fixture's own authored table", row.id);
        }
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    }
}
//#endregion 🎨️FixtureLaw
