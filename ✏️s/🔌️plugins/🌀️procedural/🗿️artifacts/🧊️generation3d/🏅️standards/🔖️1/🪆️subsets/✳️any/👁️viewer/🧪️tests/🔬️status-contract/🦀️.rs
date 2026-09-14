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
    chain_census: ChainCensus,
}

/// ⛓️ The chain ledger's own fixture section: the hop-by-hop census a preview window prices its
/// progress off, and the two rules that make that price monotone.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChainCensus {
    monotone: bool,
    sequence: Vec<ChainCensusStep>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChainCensusStep {
    step: String,
    working: bool,
    nodes_done: u32,
    nodes_total: u32,
    in_flight: u32,
    wave: Vec<String>,
    units_done: u32,
    units_total: u32,
    ratio: f64,
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
    framework_reserved_verb: bool,
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
                // 🛑 The gesture is the framework's `toolRunAbort` on the run; the kernel RELEASE below is
                // how that abort's close reaches the two extension registries the guest cannot see
                // (`preview_eval::release_invocations_for`). The session latch is what turns the published
                // phase into `cancelled`.
                session.cancel_preview_evaluation(VIEW_PREVIEW_WINDOW_ID);
                let payload = preview_eval::FlowEvalRelease { window_id: VIEW_PREVIEW_WINDOW_ID.to_string(), window_kind_id: preview::WINDOW_KIND_ID.to_string() };
                let invocations = preview_eval::release_invocations_for(&payload, address(state.addressable));
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

/// ⏯️ The run a fixture state implies: the abort affordance is the RUN's, so a state the fixture calls
/// `cancellable` is a state with a live `previewEval` run and every other state has none. Mirrors
/// `🧵️preview-eval/🧪️tests/🔬️unit`'s own `run_view`.
fn run_view(cancellable: bool) -> Option<semio_framework_plugin::ToolRunView> {
    cancellable.then(|| {
        semio_framework_plugin::ToolRunView::new(
            preview_eval::PREVIEW_EVAL_TOOL_ID,
            semio_framework_tool_run::ToolRunIdentity { id: semio_framework_tool_run::ToolRunId { app_instance_id: 1, run: 1 }, generation: 1, base_revision: [0; 32] },
            semio_framework_tool_run::ToolRunState::Running,
        )
    })
}

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
    assert_eq!(viewer.cancel_action, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID);
    assert_eq!(viewer.cancel_action, fixture.cancel_action);
    for state in &contract.states {
        let mut session = FlowEvalSession::new();
        replay(state, &mut session);
        let published = preview_eval::preview_progress_status_json_for(Some(&session), run_view(state.status.cancellable).as_ref(), address(state.addressable));
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

/// ⚖️ LAW: the progress ratio a preview window publishes is MONOTONE within one evaluation, and it
/// actually MOVES — the hop-by-hop census in `🧫️fixtures/🛑️preview-cancel.json` (`chainCensus`),
/// replayed through the ONE owning ledger both renderers read (`World3dComputeStatusV1`).
///
/// 🩸️ The budgeted-eval ledger aggregates its LIVE rows only, so a finishing node shrank both halves
/// of its fraction and the published pill counted DOWN — measured as `Computing 7/7 (100%)` →
/// `4/6 (67%)` → `3/6 (50%)` → `2/6 (33%)` across one evaluation
/// (`📓️wgpu-progress-visibility-2026-09-14.md` §4.3). A bar that runs backwards is the top
/// perceived-stall defect in the whole surface: on the longest example the user watches the
/// percentage fall and concludes the app is looping.
///
/// 🌊️ The sequence is the COALESCED chain's: four contributed nodes across three dependency levels
/// park in three waves, not four hops (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_published_progress_ratio_is_monotone_and_advances_across_one_evaluation() {
    let fixture = preview_cancel_fixture();
    let census = &fixture.chain_census;
    assert!(census.monotone, "the fixture declares the law it is replaying");
    let mut ratios = Vec::new();
    let mut nodes_done = Vec::new();
    for step in &census.sequence {
        let status = semio_framework_os_flow::PreviewChainStatus { nodes_done: step.nodes_done, nodes_total: step.nodes_total, in_flight: step.in_flight, working: step.working };
        assert_eq!(status.units(), (step.units_done, step.units_total), "{}: the chain ledger owes the fixture's units", step.step);
        assert!((status.ratio() - step.ratio).abs() < 1e-12, "{}: ratio {} is not the fixture's {}", step.step, status.ratio(), step.ratio);
        assert_eq!(step.in_flight as usize, step.wave.len(), "{}: every parked node of a wave is one answer in flight", step.step);
        if step.working {
            assert!(status.ratio() < 1.0, "{}: a working chain may never publish 1.0 — \"done\" is the one answer live work may not give", step.step);
        }
        ratios.push(status.ratio());
        nodes_done.push(step.nodes_done);
    }
    for pair in ratios.windows(2) {
        assert!(pair[1] >= pair[0] - 1e-12, "the published ratio went BACKWARDS: {ratios:?}");
    }
    for pair in nodes_done.windows(2) {
        assert!(pair[1] >= pair[0], "the node census shrank: {nodes_done:?}");
    }
    assert!(ratios.last().copied().unwrap_or_default() > ratios.first().copied().unwrap_or_default(), "a monotone ratio that never moves is a bar that reads \"nothing yet\" and then \"done\": {ratios:?}");
    let waves: Vec<usize> = census.sequence.iter().filter(|step| !step.wave.is_empty()).map(|step| step.wave.len()).collect();
    assert_eq!(waves, vec![2, 1, 1], "four contributed nodes across three dependency levels cost THREE waves, not four hops");
    eprintln!("[DEBUG] chain census ratios={ratios:?} nodesDone={nodes_done:?} waves={waves:?}");
}

/// ⚖️ LAW: a preview window rendered WITHOUT a retained session still publishes the whole contract —
/// idle, zero progress, nothing to cancel. Publishing nothing is never an option.
#[test]
fn a_sessionless_viewer_preview_still_publishes_the_contract() {
    let _serial = context::lock();
    let fixture = preview_cancel_fixture();
    let status: serde_json::Value = serde_json::from_str(&preview_eval::preview_progress_status_json_for(None, None, address(true))).expect("status json");
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

/// ⚖️ LAW: the verb the status names is the FRAMEWORK-RESERVED run abort, and the viewer earns it by
/// declaring the `previewEval` tool run — `build_definition` injects the seven `toolRun*` actions onto
/// every window kind exactly when an app declares a run. A plugin cancel command of its own would be a
/// second contract for one gesture, which is why this surface declares none (ticket
/// 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS moved cancellation onto the run).
#[test]
fn the_viewer_publishes_the_framework_run_abort_it_earns_by_declaring_the_run() {
    let fixture = preview_cancel_fixture();
    let viewer = fixture.status_contract.surfaces.iter().find(|row| row.surface == VIEWER_SURFACE_ID).expect("viewer surface row");
    assert!(viewer.framework_reserved_verb, "the fixture states the published verb is framework-reserved");
    assert_eq!(viewer.cancel_action, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID);
    let definition = create_generation3d_viewer();
    assert!(definition.tools.iter().any(|tool| tool.id == preview_eval::PREVIEW_EVAL_TOOL_ID && tool.run.is_some()), "the viewer declares the preview-evaluation RUN, which is what injects the abort");
    assert!(definition.modes.iter().any(|mode| mode.tools.iter().any(|tool_ref| tool_ref.as_str() == preview_eval::PREVIEW_EVAL_TOOL_ID)), "a declared tool no mode references is refused outright by the builder");
    assert!(!definition.commands.iter().any(|command| command.id == viewer.cancel_action), "the abort is injected, never a plugin command");
    // 🧯️ The kernel RELEASE the abort's close hands the extensions is the plugin's own retained verb,
    // and it is the one that still needs a route and a host-only lane.
    assert!(GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.contains(&"flowEvalRelease"), "the release hop needs a retained route");
    let release = definition.commands.iter().find(|command| command.id == "flowEvalRelease").expect("the viewer declares the release hop");
    assert_eq!(release.kind, ActionKind::View, "a release retires ephemeral runtime work, never the document");
    assert_eq!(release.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "an unclassified command is rejected with interactive-job.not-ui-safe");
    let contract = <Generation3dViewFlowEvalJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .find(|contract| contract.tool_id == "flowEvalRelease")
        .expect("the release hop needs an exact publication contract");
    assert_eq!(contract.lanes, &[ArtifactToolPublicationLane::HostOnly], "a release publishes no store lane at all");
}

/// ⚖️ LAW: the kernel release an abort's close hands the extensions really dispatches through the live
/// interactive-job pipeline on the read-only surface and really leaves the document untouched — the
/// viewer half of `every_viewer_action_dispatches_live_and_never_mutates_the_document`, kept here
/// because the verb is a hidden runtime command rather than a window chrome action.
#[semio_framework_async_macros::async_test]
async fn a_viewer_kernel_release_dispatches_live_and_never_mutates_the_document() {
    let _serial = context::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    let shell_view = context::view_shell_view(VIEW_PREVIEW_WINDOW_ID);
    let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(shell_view), ..semio_framework_plugin::artifact_app_laws::meta("local") };
    let args = preview_eval::window_args(VIEW_PREVIEW_WINDOW_ID, preview::WINDOW_KIND_ID);
    context::dispatch_effect_command(&mut app, "flowEvalRelease", Some(&args), &action_meta).await.expect("the shell dispatches the declared release verb");
    let receipt = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut *app, action_meta.instance_id).await.expect("the cancel settles through the retained ladder");
    assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "the viewer kernel release faulted in the retained job ladder");
    assert_eq!(context::snapshot(&app), before, "a kernel release must not mutate the document");
    eprintln!("[DEBUG] viewer kernel release settled lanes={:?} effects={}", receipt.lanes, receipt.effects.len());
    let _ = app.pending_effects(None).await;
}

/// ⚖️ LAW: the viewer's preview window offers the verb its status names. The window kind used to
/// withhold it — so `World3dHost`'s cancel button was dropped by `ShellHost`'s `declaredAction` gate
/// and a long read-only evaluation had no cancel affordance at all
/// (`📓️audit-user-journey-gaps-2026-09-13.md` §1.3 gap #9, §5 row "Flow eval (viewer)"). It now comes
/// from the framework's own injection pass over the declared run, which is the point of the move.
#[test]
fn the_viewer_preview_window_offers_the_cancel_verb_the_fixture_names() {
    let fixture: serde_json::Value = serde_json::from_str(PREVIEW_CANCEL_FIXTURE_JSON).expect("preview-cancel fixture");
    let row = fixture["statusContract"]["surfaces"].as_array().expect("surfaces").iter().find(|row| row["surface"] == "viewer").expect("the viewer surface row");
    let window_kind_id = row["windowKindId"].as_str().expect("window kind id");
    let verb = row["cancelAction"].as_str().expect("cancel action");
    assert_eq!(row["offersCancelOnWindow"].as_bool(), Some(true), "the fixture must claim the viewer window offers it");
    let definition = crate::viewer::generation3d::create_generation3d_viewer();
    let window = definition.window_kinds.iter().find(|window| window.id == window_kind_id).unwrap_or_else(|| panic!("{window_kind_id} is a declared window kind"));
    assert!(window.actions.iter().any(|action| action.id == verb), "{window_kind_id} must offer {verb}: {:?}", window.actions.iter().map(|action| action.id.as_str()).collect::<Vec<_>>());
}
