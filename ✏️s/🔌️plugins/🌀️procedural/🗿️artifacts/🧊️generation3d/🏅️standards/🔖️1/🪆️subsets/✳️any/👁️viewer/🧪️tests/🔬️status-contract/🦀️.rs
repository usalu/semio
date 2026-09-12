//! 📈️ The read-only surface's half of the ONE preview-window status contract.
//!
//! 🐛️ Regression guard for the viewer's silent status: the preview window rendered meshes while its
//! `World3dScene.status_json` stayed `None`, so `data-status-json` carried no `phase`, no `progress`
//! and no `cancellable` at all — the user could watch nothing and stop nothing, and the shell had no
//! `cancelAction` to declare (`🗑️generated/journey-7/results.json`, rows `view:*`, ticket
//! 26/09/09/PROCEDURAL-3D-END-TO-END). The projection now lives once, in `🧵️preview-eval`, and this
//! module drives the VIEWER's binding of it against the same language-agnostic fixture the editor's
//! cancellation lane drives, whose third-party twin is
//! `✏️editor/🎮️commands/🛑️cancel-preview-eval/🧪️tests/🔬️unit/contract.ts`.

use super::*;
use crate::viewer::generation3d::unit_tests::context::{self, app};
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::PluginApp;

const PREVIEW_CANCEL_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🛑️preview-cancel.json");
const VIEWER_SURFACE_ID: &str = "viewer";
const GEOMETRY_EXTENSION_PLUGIN_ID: &str = "flow-extension-brep";

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreviewCancelFixture {
    format: String,
    version: u8,
    cancel_action: String,
    phase_labels: std::collections::BTreeMap<String, PhaseLabel>,
    status_contract: StatusContract,
}

#[derive(serde::Deserialize)]
struct PhaseLabel {
    en: String,
    de: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatusContract {
    object_keys: Vec<String>,
    progress_keys: Vec<String>,
    debug_keys: Vec<String>,
    evaluate_fault_code: String,
    address_miss_code: String,
    surfaces: Vec<SurfaceRow>,
    states: Vec<StatusState>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SurfaceRow {
    surface: String,
    window_kind_id: String,
    cancel_action: String,
    declares_cancel_command: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatusState {
    id: String,
    addressable: bool,
    sequence: Vec<StatusEvent>,
    status: ExpectedStatus,
    fault: Option<ExpectedFault>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatusEvent {
    event: String,
    #[serde(default)]
    handle: String,
    #[serde(default)]
    phase: String,
    #[serde(default)]
    units_done: u32,
    #[serde(default)]
    units_total: u32,
    #[serde(default)]
    faces_done: u32,
    #[serde(default)]
    faces_total: u32,
    #[serde(default)]
    fault_code: String,
    #[serde(default)]
    fault_message: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedStatus {
    phase: String,
    cancellable: bool,
    units_done: u32,
    units_total: u32,
    faces_done: u32,
    faces_total: u32,
    in_flight: u32,
    ratio: ExpectedRatio,
}

#[derive(serde::Deserialize)]
struct ExpectedRatio {
    done: u32,
    total: u32,
}

impl ExpectedRatio {
    /// 📈️ The fixture's own `ratioLaw`, replayed: a zero total is 1 while nothing is in flight.
    fn value(&self, in_flight: u32) -> f64 {
        if self.total == 0 {
            return if in_flight == 0 { 1.0 } else { 0.0 };
        }
        f64::from(self.done) / f64::from(self.total)
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedFault {
    code: String,
    extension_id: String,
    #[serde(default)]
    capability: String,
}

fn preview_cancel_fixture() -> PreviewCancelFixture {
    let fixture: PreviewCancelFixture = serde_json::from_str(PREVIEW_CANCEL_FIXTURE_JSON).expect("preview cancel fixture");
    assert_eq!(fixture.format, "semio.generation3d.preview-cancel");
    assert_eq!(fixture.version, 1);
    fixture
}

fn node_hash(handle: &str) -> u64 {
    semio_framework_os_flow::preview_tessellate_node_hash(handle, 0.05_f64.to_bits())
}

/// 🪪️ The address the fixture's `addressable` flag stands for — supplied rather than looked up, so
/// the unaddressable branch is provable without uninstalling the process-global contribution table
/// (which poisons the flow catalogue's cache lock for every later test in this binary).
fn address(addressable: bool) -> Result<String, semio_framework_os_flow::FlowExtensionAddressMiss> {
    if addressable {
        Ok(GEOMETRY_EXTENSION_PLUGIN_ID.to_string())
    } else {
        Err(semio_framework_os_flow::FlowExtensionAddressMiss { extension_id: preview_eval::GENERATION_3D_GEOMETRY_EXTENSION_ID.to_string(), contributed: Vec::new() })
    }
}

fn replay(state: &StatusState, session: &mut FlowEvalSession) {
    for event in &state.sequence {
        match event.event.as_str() {
            "admit" => assert!(session.note_pending_tessellate(node_hash(&event.handle), event.handle.clone()), "{}: {} must be admitted", state.id, event.handle),
            "step" => {
                let body = serde_json::json!({ "phase": event.phase, "unitsDone": event.units_done, "unitsTotal": event.units_total, "facesDone": event.faces_done, "facesTotal": event.faces_total });
                session.resolve_preview_tessellate(node_hash(&event.handle), &body.to_string());
            }
            "evaluateFault" => {
                let resolve = preview_eval::FlowEvalResolve {
                    window_id: VIEW_PREVIEW_WINDOW_ID.to_string(),
                    window_kind_id: preview::WINDOW_KIND_ID.to_string(),
                    node_hash: 1,
                    output_json: String::new(),
                    extension_id: GEOMETRY_EXTENSION_PLUGIN_ID.to_string(),
                    ok: false,
                    fault_code: event.fault_code.clone(),
                    fault_message: event.fault_message.clone(),
                };
                preview_eval::resolve_eval(&resolve, session);
            }
            "cancel" => {
                let payload = preview_eval::CancelPreviewEval { window_id: VIEW_PREVIEW_WINDOW_ID.to_string(), window_kind_id: preview::WINDOW_KIND_ID.to_string() };
                let invocations = preview_eval::cancel_preview_eval_for(&payload, session, address(state.addressable));
                // 🚪️ HOW MANY doors the gesture knocks on is the cancellation lane's own law
                // (`rows[].invocations`); this law only states the half the STATUS depends on — an
                // addressable kernel is told, an unaddressable one has no actor to tell.
                assert_eq!(!invocations.is_empty(), state.addressable, "{}: an addressable kernel is told, an unaddressable one is not", state.id);
            }
            other => panic!("{}: unknown status-contract event {other}", state.id),
        }
    }
}

const VIEW_PREVIEW_WINDOW_ID: &str = "window:procedural-view-preview";

fn assert_contract_shape(status: &serde_json::Value, contract: &StatusContract, labels: &std::collections::BTreeMap<String, PhaseLabel>, cancel_action: &str, where_: &str) {
    for key in &contract.object_keys {
        if key == "debug" {
            continue;
        }
        assert!(status.get(key).is_some(), "{where_}: the published status must carry {key}: {status}");
    }
    for key in &contract.progress_keys {
        assert!(status["progress"].get(key).is_some(), "{where_}: progress must carry {key}: {status}");
    }
    assert_eq!(status["cancelAction"].as_str(), Some(cancel_action), "{where_}: cancelAction");
    let phase = status["phase"].as_str().unwrap_or_default();
    let label = labels.get(phase).unwrap_or_else(|| panic!("{where_}: the fixture declares no label for phase {phase}"));
    assert_eq!(status["phaseLabel"]["en"].as_str(), Some(label.en.as_str()), "{where_}: English label");
    assert_eq!(status["phaseLabel"]["de"].as_str(), Some(label.de.as_str()), "{where_}: German label — a surface carries both, with no default language");
}

/// ⚖️ LAW: the viewer preview publishes the full status contract in every state the fixture names —
/// idle, computing, faulted (both kinds) and cancelled — through the SAME surface-neutral projection
/// the two editor preview windows publish.
#[test]
fn the_viewer_preview_status_obeys_the_shared_contract_in_every_state() {
    let _serial = context::lock();
    let fixture = preview_cancel_fixture();
    let contract = &fixture.status_contract;
    let viewer = contract.surfaces.iter().find(|row| row.surface == VIEWER_SURFACE_ID).expect("the fixture declares the viewer surface");
    assert_eq!(viewer.window_kind_id, preview::WINDOW_KIND_ID, "the fixture's viewer window kind must be the one this surface declares");
    assert_eq!(viewer.cancel_action, preview_eval::PREVIEW_CANCEL_ACTION_ID);
    assert_eq!(viewer.cancel_action, fixture.cancel_action);
    for state in &contract.states {
        let mut session = FlowEvalSession::new();
        replay(state, &mut session);
        let published = preview_eval::preview_progress_status_json_for(Some(&session), address(state.addressable));
        crate::flow_operators::retire_flow_eval_session(session);
        let status: serde_json::Value = serde_json::from_str(&published).expect("the viewer preview status is one JSON object");
        assert_contract_shape(&status, contract, &fixture.phase_labels, &viewer.cancel_action, &state.id);
        assert_eq!(status["phase"].as_str(), Some(state.status.phase.as_str()), "{}: phase — {status}", state.id);
        assert_eq!(status["cancellable"].as_bool(), Some(state.status.cancellable), "{}: cancellable — {status}", state.id);
        assert_eq!(status["progress"]["unitsDone"].as_u64(), Some(u64::from(state.status.units_done)), "{}: unitsDone", state.id);
        assert_eq!(status["progress"]["unitsTotal"].as_u64(), Some(u64::from(state.status.units_total)), "{}: unitsTotal", state.id);
        assert_eq!(status["progress"]["facesDone"].as_u64(), Some(u64::from(state.status.faces_done)), "{}: facesDone", state.id);
        assert_eq!(status["progress"]["facesTotal"].as_u64(), Some(u64::from(state.status.faces_total)), "{}: facesTotal", state.id);
        assert_eq!(status["progress"]["inFlight"].as_u64(), Some(u64::from(state.status.in_flight)), "{}: inFlight", state.id);
        let expected_ratio = state.status.ratio.value(state.status.in_flight);
        let observed_ratio = status["progress"]["ratio"].as_f64().expect("ratio is a number");
        assert!((observed_ratio - expected_ratio).abs() < 1e-12, "{}: ratio {observed_ratio} is not the fixture's {expected_ratio}", state.id);
        match &state.fault {
            Some(expected) => {
                assert_eq!(status["fault"]["code"].as_str(), Some(expected.code.as_str()), "{}: fault code — {status}", state.id);
                assert_eq!(status["fault"]["extensionId"].as_str(), Some(expected.extension_id.as_str()), "{}: fault extensionId", state.id);
                if !expected.capability.is_empty() {
                    assert_eq!(status["fault"]["capability"].as_str(), Some(expected.capability.as_str()), "{}: fault capability", state.id);
                }
                assert!(status["fault"]["message"]["en"].as_str().is_some_and(|text| !text.is_empty()), "{}: a fault carries an English message", state.id);
                assert!(status["fault"]["message"]["de"].as_str().is_some_and(|text| !text.is_empty()), "{}: a fault carries a German message", state.id);
            }
            None => assert!(status.get("fault").is_none(), "{}: a healthy status publishes no fault — {status}", state.id),
        }
        eprintln!("[DEBUG] viewer status state={} published={published}", state.id);
    }
    assert_eq!(contract.evaluate_fault_code, semio_framework_os_flow::ExtensionEvaluateFault::CODE);
    assert_eq!(contract.address_miss_code, semio_framework_os_flow::FlowExtensionAddressMiss::CODE);
    assert!(!contract.debug_keys.is_empty());
}

/// ⚖️ LAW: a preview window rendered WITHOUT a retained session still publishes the whole contract —
/// idle, zero progress, nothing to cancel. Publishing nothing is never an option.
#[test]
fn a_sessionless_viewer_preview_still_publishes_the_contract() {
    let _serial = context::lock();
    let fixture = preview_cancel_fixture();
    let status: serde_json::Value = serde_json::from_str(&preview_eval::preview_progress_status_json_for(None, address(true))).expect("status json");
    assert_contract_shape(&status, &fixture.status_contract, &fixture.phase_labels, &fixture.cancel_action, "sessionless");
    assert_eq!(status["phase"].as_str(), Some("idle"));
    assert_eq!(status["cancellable"].as_bool(), Some(false));
    assert_eq!(status["progress"]["unitsTotal"].as_u64(), Some(0));
}

/// ⚖️ LAW: the RENDERED viewer preview scene actually carries the contract on
/// `World3dScene.status_json` — the end-to-end half, through the real window body the shell reads
/// `data-status-json` off. This is the assertion the browser gap was measured against.
#[semio_framework_async_macros::async_test]
async fn the_rendered_viewer_preview_scene_carries_the_status_contract() {
    let _serial = context::lock();
    let fixture = preview_cancel_fixture();
    let mut app = app().await;
    let shell_view = context::view_shell_view(VIEW_PREVIEW_WINDOW_ID);
    let projection = context::render_with_view(&mut app, preview::BODY_KEY, &shell_view).await;
    drop(app);
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&projection).expect("the viewer preview body decodes as a world-3d scene");
    let published = world.status_json.clone().expect("the viewer preview window must publish a status contract, exactly as both editor preview windows do");
    let status: serde_json::Value = serde_json::from_str(&published).expect("the published status is one JSON object");
    assert_contract_shape(&status, &fixture.status_contract, &fixture.phase_labels, &fixture.cancel_action, "rendered");
    for key in &fixture.status_contract.debug_keys {
        assert!(status["debug"].get(key).is_some(), "the rendered status must carry debug.{key}: {published}");
    }
    eprintln!("[DEBUG] rendered viewer preview status={published}");
}

/// ⚖️ LAW: the cancel verb the status names is a verb this surface actually DECLARES, is a `View`
/// action, is `Migrated`, and never publishes on a document or draft lane. The shell learns the verb
/// from the published status alone, so an undeclared one is dropped by `ShellHost`'s `declaredAction`
/// gate before `plugin.handleAction` and the button does nothing.
#[test]
fn the_viewer_declares_the_cancel_verb_its_status_names() {
    let fixture = preview_cancel_fixture();
    let viewer = fixture.status_contract.surfaces.iter().find(|row| row.surface == VIEWER_SURFACE_ID).expect("viewer surface row");
    assert!(viewer.declares_cancel_command, "the fixture states the viewer declares its cancel command");
    let definition = create_generation3d_viewer();
    let command = definition.commands.iter().find(|command| command.id == viewer.cancel_action).unwrap_or_else(|| panic!("the viewer must declare {}", viewer.cancel_action));
    assert_eq!(command.kind, ActionKind::View, "a cancel retires ephemeral runtime work, never the document");
    assert_eq!(command.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "an unclassified command is rejected with interactive-job.not-ui-safe");
    assert!(GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.contains(&viewer.cancel_action.as_str()), "the cancel verb needs a retained route");
    let contract = <Generation3dViewFlowEvalJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .find(|contract| contract.tool_id == viewer.cancel_action)
        .expect("the cancel verb needs an exact publication contract");
    assert_eq!(contract.lanes, &[ArtifactToolPublicationLane::HostOnly], "a cancel publishes no store lane at all");
}

/// ⚖️ LAW: the cancel really dispatches through the live interactive-job pipeline on the read-only
/// surface and really leaves the document untouched — the viewer half of
/// `every_viewer_action_dispatches_live_and_never_mutates_the_document`, kept here because the verb
/// is a hidden runtime command rather than a window chrome action.
#[semio_framework_async_macros::async_test]
async fn a_viewer_cancel_dispatches_live_and_never_mutates_the_document() {
    let _serial = context::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    let shell_view = context::view_shell_view(VIEW_PREVIEW_WINDOW_ID);
    let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(shell_view), ..semio_framework_plugin::artifact_app_laws::meta("local") };
    let args = preview_eval::window_args(VIEW_PREVIEW_WINDOW_ID, preview::WINDOW_KIND_ID);
    context::dispatch_effect_command(&mut app, preview_eval::PREVIEW_CANCEL_ACTION_ID, Some(&args), &action_meta).await.expect("the shell dispatches the declared cancel verb");
    let receipt = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut *app, action_meta.instance_id).await.expect("the cancel settles through the retained ladder");
    assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "the viewer cancel faulted in the retained job ladder");
    assert_eq!(context::snapshot(&app), before, "a cancel must not mutate the document");
    eprintln!("[DEBUG] viewer cancel settled lanes={:?} effects={}", receipt.lanes, receipt.effects.len());
    let _ = app.pending_effects(None).await;
}
