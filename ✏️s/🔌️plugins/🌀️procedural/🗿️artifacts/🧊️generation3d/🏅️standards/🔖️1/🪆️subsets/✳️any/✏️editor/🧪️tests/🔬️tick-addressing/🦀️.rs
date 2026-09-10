use super::*;
use crate::editor::generation3d::testkit::{self, app_with_registry};
use crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE;
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::{ActionMeta, PluginApp, ViewModel, ViewWindowInstance};

const TICK_ADDRESSING_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/🪟️tick-addressing.json");

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
    attached: Vec<AttachedWindow>,
    armed_window_ids: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct DispatchCase {
    id: String,
    attached: Vec<AttachedWindow>,
    current_window_id: String,
    payload_window_id: String,
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

fn armed_window_ids(effects: &[Effect]) -> Vec<String> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "flowEvalTick" => Some(args.as_ref().and_then(|args| args.get("windowId")).and_then(dsl::DslValue::as_str).unwrap_or_default().to_string()),
            _ => None,
        })
        .collect()
}

/// ⚖️ LAW: what `pending_effects` may put on the wire for a given attached-window roster. Every armed
/// tick names a concrete preview window, and a roster with no preview window arms NOTHING — which is
/// what keeps a not-yet-mounted surface from spinning a chain that can never land
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn every_armed_tick_names_a_preview_window_that_is_actually_attached() {
    let _serial = test_support::lock();
    let fixture = fixture();
    let mut app = app_with_registry().await;
    testkit::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_RECT_EXTRUDE.into() })).await;
    for case in &fixture.arming {
        let view = roster(&fixture, &case.attached);
        let armed = armed_window_ids(&app.pending_effects(Some(&view)).await);
        assert_eq!(armed, case.armed_window_ids, "arming case {}", case.id);
        eprintln!("[DEBUG] tick arming {}: attached={:?} armed={armed:?}", case.id, view.window_instances.iter().map(|window| window.id.as_str()).collect::<Vec<_>>());
    }
    let empty = armed_window_ids(&app.pending_effects(None).await);
    assert!(empty.is_empty(), "no roster at all must arm nothing, got {empty:?}");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: which of those addresses the REAL retained route admits, driven through
/// `PluginApp::handle_action` → wire decode → `ArtifactRetainedCommandPhase::Preflight` → work. The
/// served app's shell is focused on the flow window throughout, exactly as it is on a boot.
#[semio_framework_async_macros::async_test]
async fn only_a_preview_addressed_tick_passes_the_retained_preflight() {
    let _serial = test_support::lock();
    let fixture = fixture();
    for case in &fixture.dispatch {
        let mut app = app_with_registry().await;
        let view = roster(&fixture, &case.attached).for_window_instance(&case.current_window_id).expect("the current window is attached");
        let args = flow_eval_tick::window_args(&case.payload_window_id);
        let action_meta = ActionMeta { view_state: Some(view), ..semio_framework_plugin::testkit::meta("local") };
        let outcome = match testkit::dispatch_effect_command(&mut app, "flowEvalTick", Some(&args), &action_meta).await {
            Ok(()) => match semio_framework_plugin::testkit::settle_registered_typed_operation(&mut *app, action_meta.instance_id).await {
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
        semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
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
    let _serial = test_support::lock();
    let mut app = app_with_registry().await;
    let (flow_view, preview_view) = testkit::shell_views("procedural-main", "procedural-preview");
    let action_meta = ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::testkit::meta("local") };
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_RECT_EXTRUDE }).into()), &action_meta).await.expect("setActiveExample dispatches from the flow window");
    let receipt = testkit::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample faulted: {:?}", receipt.lanes);

    let ticks = testkit::drain_armed_flow_eval_ticks(&mut app, &flow_view).await;
    assert!(ticks > 0, "setActiveExample must arm at least one addressed flowEvalTick");

    let graph = testkit::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await;
    let scene = semio_framework_plugin::testkit::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(&graph).expect("node-graph scene decodes off the rendered flow surface");
    let status_json = scene.status_json.clone().expect("the flow window publishes a per-widget evaluation status");
    for widget_id in ["width", "height", "distance", "rect", "vector", "extrude", "volume"] {
        assert_eq!(super::tests::node_eval_status(&status_json, widget_id), "ok", "{widget_id} never finished evaluating: {status_json}");
    }

    let preview = testkit::render_with_view(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await;
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::testkit::decode_fixture_scene_with_lanes(&preview).expect("projected preview body must decode as an assembled world-3d scene");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
    let mesh_count = meshes.as_array().map_or(0, Vec::len);
    assert!(mesh_count >= 1, "the extruded volume must reach the preview as at least one mesh, got {mesh_count}: {}", world.meshes_json);
    eprintln!("[DEBUG] self-dispatched tick chain finished: ticks={ticks} status={status_json} meshes={mesh_count}");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}
