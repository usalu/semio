use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app_with_registry};
use crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE;
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::{ActionMeta, PluginApp, ViewModel, ViewWindowInstance};

/// ⚖️ The ONE language-agnostic addressing fixture BOTH surfaces answer — subset-level, not
/// editor-level, since the viewer runs the identical chain and its own laws read the same rows
/// (`👁️viewer/🧪️tests/🔬️eval-chain`, `contract.ts`).
const TICK_ADDRESSING_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🪟️tick-addressing.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TickAddressingFixture {
    format: String,
    version: u8,
    window_kinds: std::collections::BTreeMap<String, String>,
    arming: Vec<ArmingCase>,
    dispatch: Vec<DispatchCase>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttachedWindow {
    id: String,
    kind: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArmingCase {
    id: String,
    surface: String,
    attached: Vec<AttachedWindow>,
    armed_window_ids: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct DispatchCase {
    id: String,
    surface: String,
    attached: Vec<AttachedWindow>,
    current_window_id: String,
    payload_window_id: String,
    payload_window_kind: String,
    admitted: bool,
}

fn fixture() -> TickAddressingFixture {
    let fixture: TickAddressingFixture = serde_json::from_str(TICK_ADDRESSING_FIXTURE_JSON).expect("tick addressing fixture");
    assert_eq!(fixture.format, "semio.generation3d.tick-addressing");
    assert_eq!(fixture.version, 1);
    fixture
}

fn roster(fixture: &TickAddressingFixture, attached: &[AttachedWindow]) -> ViewModel {
    let window_instances = attached
        .iter()
        .map(|window| ViewWindowInstance { id: window.id.clone(), window_kind_id: fixture.window_kinds.get(&window.kind).unwrap_or_else(|| panic!("fixture window kind {}", window.kind)).clone() })
        .collect();
    ViewModel { window_instances, ..Default::default() }
}

/// 🪟️ The distinct windows a driven run's hops addressed, sorted.
fn hop_windows(receipt: &context::PreviewRunReceipt) -> Vec<String> {
    receipt.hop_windows.iter().cloned().collect::<std::collections::BTreeSet<_>>().into_iter().collect()
}

/// ⚖️ LAW: which windows the `previewEval` run started off a given attached-window roster evaluates.
/// Every hop names a concrete preview window, and a roster with no preview window starts NO run — which
/// is what keeps a not-yet-mounted surface from spinning work that can never land
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn every_run_hop_names_a_preview_window_that_is_actually_attached() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = fixture();
    for case in fixture.arming.iter().filter(|case| case.surface == "editor") {
        let mut app = app_with_registry().await;
        context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_RECT_EXTRUDE.into() })).await;
        let view = roster(&fixture, &case.attached);
        let owed = app.pending_effects(Some(&view)).await;
        let started = context::run_actions(&owed);
        assert_eq!(started.is_empty(), case.armed_window_ids.is_empty(), "arming case {}: a run starts exactly when a preview window is attached, got {started:?}", case.id);
        let receipt = context::drive_preview_run(&mut app, &view, &owed).await;
        let mut expected = case.armed_window_ids.clone();
        expected.sort();
        eprintln!("[DEBUG] run hops {}: attached={:?} hops={:?}", case.id, view.window_instances.iter().map(|window| window.id.as_str()).collect::<Vec<_>>(), receipt.hop_windows);
        assert_eq!(hop_windows(&receipt), expected, "arming case {}", case.id);
        assert!(context::owed_run_actions(&mut app, &semio_framework_plugin::ViewModel::default()).await.is_empty(), "no roster at all starts nothing");
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    }
}

/// ⚖️ LAW: which of those addresses the REAL retained route admits, driven through
/// `PluginApp::handle_action` → wire decode → `ArtifactRetainedCommandPhase::Preflight` → work. The
/// served app's shell is focused on the flow window throughout, exactly as it is on a boot.
#[semio_framework_async_macros::async_test]
async fn only_a_preview_addressed_tick_passes_the_retained_preflight() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = fixture();
    for case in fixture.dispatch.iter().filter(|case| case.surface == "editor") {
        let mut app = app_with_registry().await;
        let view = roster(&fixture, &case.attached).for_window_instance(&case.current_window_id).expect("the current window is attached");
        let kind = fixture.window_kinds.get(&case.payload_window_kind).cloned().unwrap_or_default();
        let args = flow_eval_tick::window_args(&case.payload_window_id, &kind);
        let action_meta = ActionMeta { view_state: Some(view), ..semio_framework_plugin::artifact_app_laws::meta("local") };
        let outcome = match context::dispatch_effect_command(&mut app, "flowEvalTick", Some(&args), &action_meta).await {
            Ok(()) => match semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut *app, action_meta.instance_id).await {
                Ok(receipt) => receipt.lanes.contains(&TypedOperationResultLane::Fault).then(|| format!("retained publication faulted: {:?}", receipt.lanes)),
                Err(fault) => Some(format!("{fault:?}")),
            },
            Err(fault) => Some(format!("{fault:?}")),
        };
        eprintln!("[DEBUG] tick dispatch {}: payloadWindowId={:?} refusal={outcome:?}", case.id, case.payload_window_id);
        assert_eq!(outcome.is_none(), case.admitted, "dispatch case {}: {outcome:?}", case.id);
        if let Some(detail) = outcome {
            assert!(!detail.contains("exceeds semantic work capacity"), "dispatch case {}: an addressing refusal must not be reported as a work-capacity fault: {detail}", case.id);
        }
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    }
}

/// ⚖️ LAW: the served boot's own chain, with NOTHING hand-addressed. `setActiveExample` is dispatched
/// under the shell's current window (the flow window), and every following tick is the `action`+`args`
/// of an `Effect::DispatchAction` the app itself armed, replayed through `handle_action` the way
/// `ShellHost` feeds `requestedEffects` back — so the tick's address, its wire decode and the retained
/// route's preflight are all real.
///
/// 🐛️ Regression guard for the served-app stall (`📓️work-capacity-2026-09-10.md` §7): the tick chain
/// carried no window address, the preview-window gate refused every dispatch, and the graph stayed on
/// `extrusion-axis: computing` with `data-meshes-json="[]"` forever
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn set_active_example_drives_the_self_dispatched_tick_chain_to_a_rendered_mesh() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow_view, preview_view) = context::shell_views("procedural-main", "procedural-preview");
    let action_meta = ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("local") };
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_RECT_EXTRUDE }).into()), &action_meta).await.expect("setActiveExample dispatches from the flow window");
    let receipt = context::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample faulted: {:?}", receipt.lanes);

    // 🔒️ The switch carries the run start on its OWN emit, and the shell feeds those `requestedEffects`
    // back — exactly what is replayed here, together with every hop the run hands the host.
    assert_eq!(context::run_actions(&receipt.effects), vec![semio_framework_plugin::TOOL_RUN_START_ACTION_ID.to_string()], "setActiveExample carries exactly one run start");
    let ticks = context::drain_armed_flow_eval_ticks_from(&mut app, &flow_view, &receipt.effects).await;
    assert!(ticks > 0, "the run setActiveExample started must dispatch at least one addressed flowEvalTick");

    let graph = context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await;
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(&graph).expect("node-graph scene decodes off the rendered flow surface");
    let status_json = scene.status_json.clone().expect("the flow window publishes a per-widget evaluation status");
    for widget_id in ["width", "height", "distance", "rect", "vector", "extrude", "volume"] {
        assert_eq!(super::unit_tests::node_eval_status(&status_json, widget_id), "ok", "{widget_id} never finished evaluating: {status_json}");
    }

    let preview = context::render_with_view(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await;
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&preview).expect("projected preview body must decode as an assembled world-3d scene");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
    let mesh_count = meshes.as_array().map_or(0, Vec::len);
    assert!(mesh_count >= 1, "the extruded volume must reach the preview as at least one mesh, got {mesh_count}: {}", world.meshes_json);
    eprintln!("[DEBUG] self-dispatched tick chain finished: ticks={ticks} status={status_json} meshes={mesh_count}");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: generate-mode preview evaluation is the SAME `previewEval` run as edit preview, never a dead
/// synchronous `FlowEvalSession::tick` inside `Generation3dPreviewCommandWork`: `addGeneration` carries the
/// run start on its own emit, and the run's hops address the generate preview window.
#[semio_framework_async_macros::async_test]
async fn generate_preview_eval_is_the_run_addressed_at_the_generate_preview() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (generations_view, _preview_view) = context::generate_shell_views("generation3d-generations", "generation3d-generate-form", "generation3d-generate-preview");
    let action_meta = ActionMeta { view_state: Some(generations_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("local") };
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_RECT_EXTRUDE }).into()), &action_meta).await.expect("setActiveExample");
    let receipt = context::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample faulted: {:?}", receipt.lanes);
    let settled = context::drive_preview_run(&mut app, &generations_view, &receipt.effects).await;
    assert_eq!(settled.state.as_deref(), Some("finalized"), "the switch's run settles: {settled:?}");
    app.handle_action("addGeneration", None, &action_meta).await.expect("addGeneration");
    let receipt = context::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "addGeneration faulted: {:?}", receipt.lanes);
    assert_eq!(context::run_actions(&receipt.effects), vec![semio_framework_plugin::TOOL_RUN_START_ACTION_ID.to_string()], "addGeneration carries exactly one run start on its own emit");
    let run = context::drive_preview_run(&mut app, &generations_view, &receipt.effects).await;
    eprintln!("[DEBUG] generate preview run: {run:?}");
    assert!(run.hop_windows.iter().any(|window| window == "generation3d-generate-preview"), "the run must evaluate the generate preview window: {run:?}");
    assert!(run.answered > 0, "generate preview eval must cross the extension boundary, not settle as a dead sync tick: {run:?}");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: generate mode's ENTRY state and the `addGeneration` affordance that leaves it, end to end.
///
/// Two halves, both measured live on 6018 as broken (`🗑️generated/journey-3/results.json`: entering
/// generate mode gave `hosts=[]` and `windows=[]` for 189 s, ticket 26/09/09/PROCEDURAL-3D-END-TO-END):
/// 1. **Entry** — with no generation the preview window is ALREADY a world-3d status host publishing a
///    tessellation `phase` plus the authored hint, so the shell has a surface to show progress on.
/// 2. **Leaving it** — `addGeneration`, the action the generations window's own "Add Generation" tree
///    item binds, selects a generation whose patched fixture drives the SAME addressed tick chain to
///    real tessellated geometry: `meshes > 0` in the generate preview, not just a re-armed tick.
#[semio_framework_async_macros::async_test]
async fn add_generation_from_the_generations_window_drives_the_generate_preview_to_a_rendered_mesh() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (generations_view, preview_view) = context::generate_shell_views("generation3d-generations", "generation3d-generate-form", "generation3d-generate-preview");
    let action_meta = ActionMeta { view_state: Some(generations_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("local") };

    let entry = context::render_with_view(&mut app, generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW, &preview_view).await;
    let entry_scene: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&entry).expect("the generate-mode entry preview is a world-3d status host");
    let entry_status = entry_scene.status_json.clone().expect("the generate preview publishes a status host before anything is generated");
    let entry_json: serde_json::Value = serde_json::from_str(&entry_status).expect("entry status json");
    // 🪪️ With the geometry extension actually contributed the entry phase is IDLE — nothing is
    // evaluating and nothing is faulted, which is what the shell's world host needs to show a quiet
    // window rather than a spinner or a fault badge.
    assert_eq!(entry_json.get("phase").and_then(serde_json::Value::as_str), Some("idle"), "entry status must be idle under a contributed geometry extension: {entry_status}");
    assert!(entry_json.get("hint").and_then(serde_json::Value::as_str).unwrap_or_default().contains("evaluate a generation"), "entry status must carry the hint: {entry_status}");
    assert_eq!(entry_scene.instances_json, "[]", "entry state draws no instances: {}", entry_scene.instances_json);

    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_RECT_EXTRUDE }).into()), &action_meta).await.expect("setActiveExample");
    let receipt = context::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample faulted: {:?}", receipt.lanes);
    context::drain_armed_flow_eval_ticks_from(&mut app, &generations_view, &receipt.effects).await;

    app.handle_action("addGeneration", None, &action_meta).await.expect("addGeneration dispatches from the generations window");
    let added = context::settle(&mut app).await;
    assert!(!added.lanes.contains(&TypedOperationResultLane::Fault), "addGeneration faulted: {:?}", added.lanes);
    let ticks = context::drain_armed_flow_eval_ticks_from(&mut app, &generations_view, &added.effects).await;
    assert!(ticks > 0, "addGeneration must arm at least one addressed flowEvalTick");

    let preview = context::render_with_view(&mut app, generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW, &preview_view).await;
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&preview).expect("the generate preview stays a world-3d surface once a generation exists");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("generate preview meshes json");
    let mesh_count = meshes.as_array().map_or(0, Vec::len);
    let status = world.status_json.clone().unwrap_or_default();
    eprintln!("[DEBUG] addGeneration generate-preview chain: ticks={ticks} meshes={mesh_count} status={status}");
    assert!(mesh_count >= 1, "the generation the UI added must reach the generate preview as at least one mesh, got {mesh_count}: {}", world.meshes_json);
    assert!(!status.contains("\"hint\""), "a preview with evaluated geometry must drop the entry hint: {status}");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
