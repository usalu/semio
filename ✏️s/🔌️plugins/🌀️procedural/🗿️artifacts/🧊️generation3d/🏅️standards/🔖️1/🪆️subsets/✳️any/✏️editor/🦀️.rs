//! 🧱️ Generation3d editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.

use crate::editor::generation3d::commands::{
    add_generation, add_widget, cancel_preview_eval, delete_selection, flow_eval_resolve, flow_eval_tick, flow_tessellate_resolve, move_media_node, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection, update_generation_values,
};
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use crate::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use crate::editor::generation3d::modes::{edit, generate};
use crate::editor::generation3d::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::generation3d::terminology::generation3d_labels;
use crate::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation, SetGenerationPreview};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
use semio_framework_os_flow::{FlowEvalSession, FlowHost};
// 🚧️ SDK note (ticket 26/08/16 contract §2.1/§2.4): `ArtifactEditor`/`Editor`/`Dialect` are curated at
// `semio_framework_plugin`'s crate root as of W0-F/W2-FIX — imported bare here, no `app::` prefix
// needed (unlike the earlier cad pilot, written before that gap closed). `app::InteractionView` is a
// separate, still-uncurated gap (unrelated to this ticket) — kept qualified.
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    app::InteractionView, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract,
    ArtifactToolPublicationLane, ArtifactView, CommandDefinition, ConfigView, Dialect, DomainTopology, DraftView, Editor, EditorApp, Effect, Emit, EphemeralEmit, ExampleSource, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider,
    HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, InteractiveJobClassification, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaType, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode,
    SelectionSpec, TopologyNode, UtilityDefinition, WindowMeasure,
};
use serde_json::Value;
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_APP_ID: &str = "procedural3d-play";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎚️options/*`) builds its `on_change`/item actions with.
pub fn generation3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: GENERATION_3D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }
}

fn categorized_action(id: &str, label: LocalizedLabel, kind: ActionKind, category: &str) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, label, kind).with_category(category)
}

/// 🧵️ Classifies the internal flow continuation as backed by its bounded first-step factory.
fn migrated_command(mut definition: CommandDefinition) -> CommandDefinition {
    definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    definition
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Generation3dPlayApp::Command` — the SOLE dispatch surface for generation3d's own behavior,
    /// covering EVERY declared action. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Generation3dCommand for Generation3dSnapshot, Generation3dMutation, Generation3dConfig, Generation3dConfigMutation, ctx = FlowEvalSession {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "nodeGraphEdit" as "graph-edit" => node_graph_edit::NodeGraphEdit,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "removeWidget" as "remove-widget" => remove_widget::RemoveWidget,
        "moveMediaNode" as "move-node" => move_media_node::MoveMediaNode,
        "addWidget" as "add-widget" => add_widget::AddWidget,
        "patchFlowWidgets" as "patch-flow-widgets" => patch_flow_widgets::PatchFlowWidgets,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "addGeneration" as "add-generation" => add_generation::AddGeneration,
        "removeGeneration" as "remove-generation" => remove_generation::RemoveGeneration,
        "renameGeneration" as "rename-generation" => rename_generation::RenameGeneration,
        "updateGenerationValues" as "update-generation-values" => update_generation_values::UpdateGenerationValues,
        "nodeGraphViewport" as "viewport" => node_graph_viewport::NodeGraphViewport,
        "setLodMode" as "lod-mode" => set_lod_mode::SetLodMode,
        "setShowMode" as "show-mode" => set_show_mode::SetShowMode,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setCamera" as "camera" => set_camera::SetCamera,
        "selectGeneration" as "select-generation" => select_generation::SelectGeneration,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "flowTessellateResolve" as "flow-tessellate-resolve" => flow_tessellate_resolve::FlowTessellateResolve,
        "cancelPreviewEval" as "cancel-preview-eval" => cancel_preview_eval::CancelPreviewEval}
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️InstanceOperationOwner
/// 🧠️ The ONE `FlowEvalSession` this app instance retains across turns — its neural cache, its
/// `eval_json`, its live preview meshes and, critically, its `pending_tessellate_by_hash` table.
/// Every entry point used to build a throwaway `FlowEvalSession::new()`, so an in-flight tessellate
/// handle noted while emitting the request was gone before its answer could arrive and
/// `resolve_preview_tessellate` always returned `false` — the 3d preview could never paint. (The
/// throwaway also violated `FlowEvalSession`'s own `Drop` contract, which rejects a live drop.)
/// Mirrors `FlowInstanceOperationOwner` in the flow artifact — the framework's reference owner.
struct Generation3dInstanceOperationOwner {
    eval_session: Option<FlowEvalSession>,
    closing: bool,
}

impl Generation3dInstanceOperationOwner {
    fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), closing: false }
    }

    fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("generation3d-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("generation3d-eval-session-owner-missing"))
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for Generation3dInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        let Some(session) = self.eval_session.as_mut() else { return Ok(semio_framework_plugin::PluginCloseStep::Complete) };
        let step = session.close_step(maximum_items, maximum_bytes);
        if session.terminal_is_empty() {
            self.eval_session = None;
        }
        Ok(match step {
            semio_framework_job::InteractiveJobCloseStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "Generation3d evaluation session awaits its exact close grant" },
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes },
            semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_plugin::PluginCloseStep::Complete,
        })
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if let Some(session) = self.eval_session.as_mut() {
            session.begin_close();
        }
        self.maintenance_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.eval_session.is_none()
    }
}

/// 🧹️ The ownerless fallback: a scratch session retired through the explicit `begin_close` +
/// granted `close_step` loop `FlowEvalSession`'s `Drop` contract demands. Reachable only from the
/// marks-free `render` and the non-retained `handle`, neither of which is handed the app-instance
/// operation owner by the framework — every live path uses the RETAINED session instead, so nothing
/// that must survive a turn is ever computed here.
fn with_scratch_session<R>(body: impl FnOnce(&mut FlowEvalSession) -> R) -> R {
    let mut session = FlowEvalSession::new();
    let result = body(&mut session);
    session.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(usize::MAX, usize::MAX);
    }
    result
}
//#endregion 🔖️InstanceOperationOwner

//#region 🔖️Generation3dPlayApp
/// 🧪️ Unit struct: the retained evaluation session lives in [`Generation3dInstanceOperationOwner`]
/// and every other former runtime field lives in [`Generation3dConfig`], written through
/// [`Generation3dConfigMutation`]s.
#[derive(Default)]
pub struct Generation3dPlayApp;

/// 🎥️ Parses the flow-graph camera out of `command_from_action`'s JSON args — either a nested
/// `{camera: {...}}` object or flat `x`/`y`/`zoom` keys.
fn parse_flow_camera_json(args: &dsl::DslValue) -> semio_framework_artifact_flow_flow::CameraJson {
    if let Some(camera) = args.get("camera") {
        // 🌉️ `semio_framework_artifact_flow_flow::CameraJson` derives `ToValue`/`FromValue` alongside its `Serialize`/
        // `Deserialize` (see `🌊️flow/🗿️artifact/🦀️.rs`), so this decodes straight off the
        // first-party bridge — no `serde_json` involved.
        if let Ok(parsed) = dsl::from_dsl_value::<semio_framework_artifact_flow_flow::CameraJson>(camera.clone()) {
            return parsed;
        }
    }
    semio_framework_artifact_flow_flow::CameraJson {
        x: args.get("x").and_then(dsl::DslValue::as_f64).unwrap_or(0.0),
        y: args.get("y").and_then(dsl::DslValue::as_f64).unwrap_or(0.0),
        zoom: args.get("zoom").and_then(dsl::DslValue::as_f64).unwrap_or(1.0),
    }
}

/// 🎥️ Parses the 3D preview camera out of `command_from_action`'s JSON args; falls back to the default
/// camera on any malformed/missing `camera` object.
fn parse_preview_camera_json(args: &dsl::DslValue) -> crate::editor::generation3d::config::Generation3dPreviewCamera {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = <crate::editor::generation3d::config::Generation3dPreviewCamera as protocol::FromValue>::from_value(camera.clone()) {
            return parsed;
        }
    }
    crate::editor::generation3d::config::Generation3dPreviewCamera::default()
}

/// 🕸️ Every node's visible port ids (`{nodeId}@{portId}`), read from the SAME
/// `fixture_to_workflow` projection the node-graph window paints — so an interaction target and a
/// graph pick can never drift apart.
fn generation3d_port_ids_by_node(fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> std::collections::BTreeMap<String, Vec<String>> {
    let (graph_nodes, _) = crate::standards::v1::subsets::any::schema::with_host(fixture, |host| crate::standards::v1::subsets::any::schema::fixture_to_workflow(&host.dag.fixture));
    graph_nodes.into_iter().map(|node| (node.id, node.inputs.into_iter().chain(node.outputs).map(|port| port.id).collect())).collect()
}

/// 🧱️ Every window body of the generation3d editor, rendered against one already-resolved set of
/// `graph` marks. Shared by `render` (marks-free) and `render_with_request_context` (live marks) so
/// there is exactly one body-key match in the app.
fn generation3d_render_body(
    body_key: &str,
    document: &Generation3dSnapshot,
    config: &Generation3dConfig,
    preview_eval_text: Option<&str>,
    generation_preview_text: Option<&str>,
    view_state: &semio_framework_plugin::ViewModel,
    marks: &PreviewInteractionMarks,
    session: &FlowEvalSession,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = generation3d_labels(view_state);
    let active_utility = view_state.active_utility_id.as_deref().unwrap_or("move");
    let node = match body_key {
        flow_window::GENERATION_3D_PLAY_BODY_MAIN => flow_window::render(document, config, session, marks),
        edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, preview_eval_text, session, active_utility, marks),
        generations::GENERATION_3D_PLAY_BODY_GENERATIONS => generations::render(&document.generation, view_state.locale, view_state.terminology),
        form::GENERATION_3D_PLAY_BODY_GENERATE_FORM => form::render(&document.fixture, &document.generation, labels),
        generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(&document.fixture, &document.generation, generation_preview_text, config, labels, active_utility, marks),
        document_panel::GENERATION_3D_PLAY_BODY_DOCUMENT => document_panel::render(&document.fixture, labels),
        catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
        inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION => inspection_panel::render(&document.fixture, &marks.graph_selection_ids(), labels),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}

//#region 🧵️RetainedCommands
/// 🧾️ Every gen3d tool id, in `Generation3dCommand` declaration order — a bijection with
/// `Generation3dCommand`'s 29 rows (asserted by `retained_route_dispositions_are_exact_and_exhaustive`
/// below) and with `Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS`.
const GENERATION3D_RETAINED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "nodeGraphEdit",
    "deleteSelection",
    "removeWidget",
    "moveMediaNode",
    "addWidget",
    "patchFlowWidgets",
    "reorganize",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "addGeneration",
    "removeGeneration",
    "renameGeneration",
    "updateGenerationValues",
    "nodeGraphViewport",
    "setLodMode",
    "setShowMode",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setCamera",
    "selectGeneration",
    "flowEvalTick",
    "flowEvalResolve",
    "flowTessellateResolve",
    "cancelPreviewEval",
];
const GENERATION3D_RETAINED_PAYLOAD_SCHEMA: &str = "generation.3d.tool-command.v1";
const GENERATION3D_RETAINED_RAW_BYTES: usize = 8_192;
const GENERATION3D_PREVIEW_TOOL_IDS: &[&str] = &["setActiveExample", "addGeneration", "removeGeneration", "renameGeneration", "updateGenerationValues", "selectGeneration"];
const GENERATION3D_RETAINED_WORK_ITEMS: usize = 32;
/// 🧺️ The ONE fold-contract footprint BOTH durable lanes of all 29 retained routes declare —
/// artifact and config alike, never a second literal. `store::ArtifactStore::fold_batch_item`
/// rejects a candidate whose `forwards.len() + inverse.len()` exceeds the gesture-wide footprint
/// this preflight declared, and every `Generation3dMutation`/`Generation3dConfigMutation` inverse is
/// at most one row (proved by `fold_contract::…_declares_the_exact_fold_envelope_…`), so the exact
/// declaration is one forward row plus one inverse row. The `work_items: 1` this used to build
/// fail-closed EVERY retained command with `batched item candidate failed its exact fixed fold
/// contract` — the boot-time `setActiveExample` included
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn generation3d_one_item_footprint(retained_bytes: usize) -> store::ArtifactStoreOneItemFootprint {
    store::ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes)
}
/// 🎒️ Real bound for one Artifact-lane edit: the 8 built-in example DSLs top out around 1.6 KB of text
/// (`📚️examples/*/🖼️assets/*/🗣️.dsl.semio`), and `setActiveExample`'s full-fixture replacement is the
/// single largest Artifact mutation any of the 27 tools ever emits — 64 KiB stays a real ceiling, not a
/// rubber stamp, for every example plus ordinary interactive graph edits.
const GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;
/// 🎒️ Real bound for one Config-lane edit: a full snapshot containing the flow/preview cameras,
/// selected generation, and sun JSON remains bounded independently of computed preview output.
const GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 262_144;

fn generation3d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_RETAINED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

#[expect(clippy::unnecessary_wraps, reason = "BoundedArtifactCommandWork requires an optional extent callback")]
fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

struct Generation3dPreviewCommandWork {
    tool_id: &'static str,
    emit: Option<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>>,
    host: Option<FlowHost>,
    /// 🧹️ The host's explicit retirement ladder, armed by `close_step` when the evaluation host is
    /// handed over. A bare `Option<FlowHost>::take()`-and-drop panics on the fixture's
    /// `OrderedMap<WidgetLayout>` root, so the host is drained under the caller's grant instead.
    host_retirement: Option<semio_framework_os_flow::FlowHostRetirement>,
    session: Option<FlowEvalSession>,
    started: bool,
    complete: bool,
    closing: bool,
}

impl Generation3dPreviewCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, emit: None, host: None, host_retirement: None, session: None, started: false, complete: false, closing: false }
    }

    fn complete(&mut self, preview_text: Option<String>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        self.complete = true;
        let emit = self.emit.take().ok_or_else(|| Fault::from("generation3d-preview-emit-owner-absent"))?;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![Generation3dTransientMutation::from(SetGenerationPreview { preview_text })], window_transient: Vec::new() } })
    }
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dPreviewCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Generation3dCommand,
        snapshot: &Generation3dSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        if matches!(command, Generation3dCommand::SetActiveExample(_)) {
            return Some(1);
        }
        if !matches!(command, Generation3dCommand::AddGeneration(_) | Generation3dCommand::RemoveGeneration(_) | Generation3dCommand::RenameGeneration(_) | Generation3dCommand::UpdateGenerationValues(_) | Generation3dCommand::SelectGeneration(_)) {
            return None;
        }
        snapshot.fixture.widgets.len().checked_add(2).filter(|extent| *extent <= GENERATION3D_RETAINED_WORK_ITEMS)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        if self.closing || self.complete {
            return Err(Fault::from("generation3d-preview-work-is-terminal"));
        }
        if !self.started {
            self.started = true;
            if let Generation3dCommand::SetActiveExample(payload) = input.command {
                let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
                let cfg = ConfigView { snapshot: input.config, window: None };
                self.emit = Some(set_active_example::emit(payload, &doc, &cfg)?);
                return self.complete(None);
            }
            let result = crate::editor::generation3d::commands::generation::generation_command_result_for(input.command, input.snapshot, input.config).ok_or_else(|| Fault::from("generation3d-preview-command-unowned"))?;
            self.emit = Some(result.emit);
            let Some(fixture) = result.preview_fixture else {
                return self.complete(None);
            };
            let mut host = FlowHost::from_fixture(fixture);
            host.set_neuron_kind_infos_json(&semio_framework_os_flow::flow_neuron_kind_infos_json());
            let mut session = FlowEvalSession::new();
            session.sync(&host);
            self.host = Some(host);
            self.session = Some(session);
            return Ok(ArtifactCommandWorkStep::Progress { stage: "generation3d-preview-evaluation", preview: b"{\"en\":\"Evaluating generation preview\",\"de\":\"Generierungsvorschau wird ausgewertet\"}" });
        }
        let host = self.host.as_mut().ok_or_else(|| Fault::from("generation3d-preview-host-owner-absent"))?;
        let session = self.session.as_mut().ok_or_else(|| Fault::from("generation3d-preview-session-owner-absent"))?;
        if session.tick(host) {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "generation3d-preview-evaluation", preview: b"{\"en\":\"Evaluating generation preview\",\"de\":\"Generierungsvorschau wird ausgewertet\"}" });
        }
        let preview_text = Some(session.eval_json().to_string());
        self.complete(preview_text)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(session) = self.session.as_mut() {
            session.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Blocked;
        }
        if let Some(session) = self.session.as_mut() {
            match session.close_step(maximum_items, maximum_bytes) {
                InteractiveJobCloseStep::Complete => {
                    if !session.terminal_is_empty() {
                        return InteractiveJobCloseStep::Blocked;
                    }
                    self.session.take();
                    return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                step => return step,
            }
        }
        if let Some(host) = self.host.take() {
            self.host_retirement = Some(semio_framework_os_flow::FlowHostRetirement::new(host));
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(retirement) = self.host_retirement.as_mut() {
            return match retirement.close_page(maximum_items, maximum_bytes) {
                Ok(false) => InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                Ok(true) => {
                    self.host_retirement = None;
                    InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                Err(_) => InteractiveJobCloseStep::Blocked,
            };
        }
        if self.emit.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.emit.is_none() && self.host.is_none() && self.host_retirement.is_none() && self.session.is_none()
    }
}

struct Generation3dFlowEvalWindowWork {
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    complete: bool,
    closing: bool,
}

impl Generation3dFlowEvalWindowWork {
    fn new(instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self { Self { instance_owner, complete: false, closing: false } }
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dFlowEvalWindowWork {
    fn tool_id(&self) -> &'static str { "flowEvalTick" }

    fn extent(
        &self,
        command: &Generation3dCommand,
        _snapshot: &Generation3dSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        let context = context?;
        let view = context.view_state.as_ref()?;
        let window = context.window_transient.as_ref()?;
        (matches!(command, Generation3dCommand::FlowEvalTick(_))
            && view.window_id.as_deref() == Some(window.window_id())
            && window.window_kind_id() == edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW
            && window.get::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>().is_some())
        .then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        if self.complete || self.closing { return Err(Fault::from("generation3d-flow-eval-window-work-terminal")); }
        let context = input.context.ok_or_else(|| Fault::from("generation3d-flow-eval-window-context-required"))?;
        let view = context.view_state.as_ref().ok_or_else(|| Fault::from("generation3d-flow-eval-window-view-required"))?;
        let window = context.window_transient.as_ref().ok_or_else(|| Fault::from("generation3d-flow-eval-window-transient-required"))?;
        if view.window_id.as_deref() != Some(window.window_id()) || window.window_kind_id() != edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW {
            return Err(Fault::from("generation3d-flow-eval-window-owner-mismatch"));
        }
        window.get::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>().ok_or_else(|| Fault::from("generation3d-flow-eval-window-owner-required"))?;
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: context.window_config.as_ref() };
        let (emit, eval_text) = self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| owner.with_session(|session| flow_eval_tick::evaluate(&doc, &cfg, session))?)?;
        self.complete = true;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit,
            ephemeral: EphemeralEmit {
                presence: Vec::new(),
                transient: Vec::new(),
                window_transient: vec![edit_preview::transient::addressed(window, eval_text)?],
            },
        })
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if self.closing { semio_framework_job::InteractiveJobCloseStep::Complete } else { semio_framework_job::InteractiveJobCloseStep::Blocked }
    }

    fn terminal_is_empty(&self) -> bool { self.closing }
}

/// 🕹️ `nodeGraphEdit`/`deleteSelection`/`{translate,rotate,scale}Selection` read real `graph` selection
/// directly off `protocol::InteractionState` (the raw, crate-public half of what `app::InteractionView`
/// wraps) — plugin code cannot construct an `InteractionView` itself (`state`/`hover`/`peers` are
/// `pub(crate)` to `semio_framework_plugin`), so this is the only way a retained-command-job reducer can
/// preserve the same real-selection behavior `Generation3dPlayApp::handle` gives those five commands above.
#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn generation3d_retained_reduce(
    command: &Generation3dCommand,
    snapshot: &Generation3dSnapshot,
    config: &Generation3dConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    operation: &AppOperationContext,
    session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>, Fault> {
    if !GENERATION3D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("generation3d-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    let selected = || interaction.selection.get("graph").map(|selection| selection.ids.clone()).unwrap_or_default();
    match command {
        Generation3dCommand::NodeGraphEdit(payload) => Ok(node_graph_edit::apply_selected(payload, &doc, &selected())),
        Generation3dCommand::DeleteSelection(_payload) => Ok(delete_selection::apply_selected(&doc, &selected())),
        Generation3dCommand::TranslateSelection(payload) => Ok(translate_selection::apply_selected(payload, &doc, &selected())),
        Generation3dCommand::RotateSelection(payload) => Ok(rotate_selection::apply_selected(payload, &doc, &selected())),
        Generation3dCommand::ScaleSelection(payload) => Ok(scale_selection::apply_selected(payload, &doc, &selected())),
        _ => command.dispatch(&doc, &cfg, session),
    }
}

/// 🧵️ The retained-session twin of `BoundedArtifactCommandWork`: identical one-shot reduce, except
/// the reducer runs against the app instance's RETAINED [`Generation3dInstanceOperationOwner`]
/// session instead of a session born and destroyed inside the same dispatch. That is what makes
/// `flowEvalResolve`'s `seed_node_cache` and `flowTessellateResolve`'s `resolve_preview_tessellate`
/// land on the same cache the next render reads.
struct Generation3dSessionCommandWork {
    tool_id: &'static str,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    consumed: bool,
}

impl Generation3dSessionCommandWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, consumed: false }
    }
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dSessionCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Generation3dCommand,
        snapshot: &Generation3dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        generation3d_bounded_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        if self.consumed {
            return Err(Fault::from("generation3d-session-command-work-repeated"));
        }
        self.consumed = true;
        let emit = self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| generation3d_retained_reduce(input.command, input.snapshot, input.config, input.history, input.interaction, input.hover, input.context, input.operation, session))?
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
}

struct Generation3dBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GENERATION3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dBoundedCommandJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeWidget", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "moveMediaNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addWidget", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchFlowWidgets", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "removeGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "renameGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "updateGenerationValues", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setShowMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "selectGeneration", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        // ⏱️ The tick's ONLY store lane is the addressed preview window's OWN transient
        // (`Generation3dFlowEvalWindowWork::step`'s `CompleteWithEphemeral`); its self-redispatch
        // effects and extension invocations are not store lanes at all and need no declaration.
        // `Config` was doubly wrong — it grants a lane the tick never writes and withholds the one it
        // does, so every dispatched tick was refused with `typed-operation emitted a store lane absent
        // from its exact factory publication contract`. Pairing `HostOnly` with it was equally wrong:
        // `HostOnly` MEANS "no store lane" and `ArtifactToolFactoryRegistry::register` rejects it
        // alongside any other lane with `interactive-job.publication-contract`
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowEvalResolve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "flowTessellateResolve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "cancelPreviewEval", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️ArtifactStorePreparation
/// 🧬️ Builds one `protocol::Edit<M>` for either lane's `advance()` — the two lanes differ only in `M`
/// and their id prefix, so this one generic helper replaces two copies of the same ~20-line literal.
fn generation3d_next_edit<M>(prefix: &str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

fn generation3d_artifact_mutation_retained_bytes(mutation: &Generation3dMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation3d-artifact-mutation-encode-failed".to_string())
}

fn admit_generation3d_artifact_mutation(mutation: &Generation3dMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("generation3d-artifact-mutation-envelope".into());
    }
    Ok(generation3d_one_item_footprint(retained_bytes))
}

/// 🧬️ Raises the mutation's delta, applies it and CLOSES the delta — a `Generation3dDiff` owns the
/// projections it displaces (a `FlowFixture` whose `layout` is an `OrderedMap` root that rejects a bare
/// drop), so the intermediate delta is retired rather than dropped: leaving it to drop glue aborted the
/// whole store-publication turn the moment a layout entry existed
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Mirrors the `🌀️generation2d` twin exactly.
fn prepare_generation3d_artifact(base: &Generation3dSnapshot, mutation: Generation3dMutation) -> Result<(Generation3dSnapshot, Vec<Generation3dMutation>, Generation3dMutation), String> {
    admit_generation3d_artifact_mutation(&mutation)?;
    let inverse = protocol::Mutation::inverse(&mutation, base);
    let diff = protocol::Mutation::diff(&mutation, base).into_parts().0;
    let applied = protocol::MutationDiff::apply(&diff, base);
    diff.retire_cold();
    let post = applied.map_err(|_| "generation3d-artifact-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

struct Generation3dArtifactStorePreparationFactory;

struct Generation3dArtifactStorePreparation {
    base: Option<store::SnapshotRead<Generation3dSnapshot>>,
    mutation: Option<Generation3dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &Generation3dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("generation3d-artifact-lane-or-description-envelope".into());
        }
        admit_generation3d_artifact_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation>> {
        let retained_bytes = generation3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation3dArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation3d-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "generation3d-artifact-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_generation3d_artifact(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "generation3d-artifact-authority-missing".to_string())?;
        let edit = generation3d_next_edit("generation3d-artifact-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("generation3d-artifact-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ArtifactStorePreparation

//#region 📬️ConfigStorePreparation
fn generation3d_config_mutation_retained_bytes(mutation: &Generation3dConfigMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation3d-config-mutation-encode-failed".to_string())
}

fn admit_generation3d_config_mutation(mutation: &Generation3dConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation3d_config_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("generation3d-config-mutation-envelope".into());
    }
    Ok(generation3d_one_item_footprint(retained_bytes))
}

fn prepare_generation3d_config(base: &Generation3dConfig, mutation: Generation3dConfigMutation) -> Result<(Generation3dConfig, Vec<Generation3dConfigMutation>, Generation3dConfigMutation), String> {
    admit_generation3d_config_mutation(&mutation)?;
    let inverse = protocol::Mutation::inverse(&mutation, base);
    let diff = protocol::Mutation::diff(&mutation, base).into_parts().0;
    let post = protocol::MutationDiff::apply(&diff, base).map_err(|_| "generation3d-config-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

struct Generation3dConfigPreparationFactory;

struct Generation3dConfigPreparation {
    base: Option<store::SnapshotRead<Generation3dConfig>>,
    mutation: Option<Generation3dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Generation3dConfig, Generation3dConfigMutation> for Generation3dConfigPreparationFactory {
    fn preflight(&self, mutation: &Generation3dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("generation3d-config-lane-or-description-envelope".into());
        }
        admit_generation3d_config_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation3dConfig, Generation3dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation3dConfig, Generation3dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation3dConfig, Generation3dConfigMutation>> {
        let retained_bytes = generation3d_config_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation3dConfigPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Generation3dConfig, Generation3dConfigMutation> for Generation3dConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation3d-config-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "generation3d-config-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_generation3d_config(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "generation3d-config-authority-missing".to_string())?;
        let edit = generation3d_next_edit("generation3d-config-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("generation3d-config-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ConfigStorePreparation

impl ArtifactEditor for Generation3dPlayApp {
    /// 🛍️ Publishes the whole registered flow operator catalogue once per app instance on the reserved
    /// `framework.section.catalogue` retained surface — never on the node-graph scene, whose fixed
    /// `UI_FIXED_BYTES` admission it exceeds threefold with the real `brep`/`math` sets installed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
    fn app_catalogue_json() -> String {
        semio_framework_os_flow::flow_app_catalogue_json()
    }

    type Snapshot = Generation3dSnapshot;
    type Mutation = Generation3dMutation;
    type Config = Generation3dConfig;
    type ConfigMutation = Generation3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::generation3d::presence::Generation3dPresence;
    type PresenceMutation = crate::editor::generation3d::presence::Generation3dPresenceMutation;
    type Transient = Generation3dTransient;
    type TransientMutation = Generation3dTransientMutation;

    type Command = Generation3dCommand;

    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_envelope_decode_owner_bundle())
    }

    /// 🧠️ One retained `FlowEvalSession` per app instance — see [`Generation3dInstanceOperationOwner`].
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Generation3dInstanceOperationOwner::new())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_document_store_owners())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_document_store_initialization_job(envelope, operation, generation))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Generation3d atomic publication authority is absent or stale"))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default()).expect("default Generation3d presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>()
    }

    const DIALECT: Dialect = crate::GENERATION3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Generation3dArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Generation3dConfigPreparationFactory))
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Generation3dBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GENERATION3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("generation3d-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Generation3dPlayApp>>> =
            if tool_id == "flowEvalTick" {
                Box::new(Generation3dFlowEvalWindowWork::new(request.instance_operation_owner))
            } else if GENERATION3D_PREVIEW_TOOL_IDS.contains(&tool_id) {
                Box::new(Generation3dPreviewCommandWork::new(tool_id))
            } else {
                Box::new(Generation3dSessionCommandWork::new(tool_id, request.instance_operation_owner))
            };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            Generation3dCommand::command_id,
            GENERATION3D_RETAINED_RAW_BYTES,
            GENERATION3D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        document_schema: "generation.3d",
        factory: "Generation3dBoundedCommandJobFactory",
        factory_type: Generation3dBoundedCommandJobFactory,
        tools: {
            "setActiveExample" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "nodeGraphEdit" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "deleteSelection" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "removeWidget" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "moveMediaNode" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "addWidget" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "patchFlowWidgets" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "reorganize" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "translateSelection" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "rotateSelection" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "scaleSelection" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "addGeneration" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "removeGeneration" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "renameGeneration" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "updateGenerationValues" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "nodeGraphViewport" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setLodMode" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setShowMode" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "toggleSun" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setSunAzimuth" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setSunElevation" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setSunIntensity" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setCamera" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "selectGeneration" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "flowEvalTick" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "flowEvalResolve" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "flowTessellateResolve" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "cancelPreviewEval" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        }
    }

    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::generation3d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Generation3dSnapshot {
        crate::standards::v1::subsets::any::schema::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(semio_framework::io::resolve_ready(generation3d_io()))
    }

    /// 🎞️ `geometry:out` plus the inherited `document:out` default, replicated inline (overriding
    /// `export_media` shadows the trait's provided body for every port on this app).
    fn export_media(port: &str, doc: &ArtifactView<'_, Generation3dSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "geometry:out" => {
                let mesh = export_mesh_from_document(doc.snapshot);
                Ok(semio_framework_plugin::Media {
                    media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
                    payload: semio_framework_plugin::MediaPayload::Structured { schema: "3d.mesh".into(), json: dsl::json::to_json_string(&mesh) },
                })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media { media_type, payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"params:in"` — patches matching `InputSlider` widgets from a `{widgetId: number}` JSON
    /// object; unmatched keys/non-slider widgets are silently ignored.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &ArtifactView<'_, Generation3dSnapshot>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "params:in" => {
                let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "params:in importer only accepts a Structured JSON object payload".into()));
                };
                let parsed = dsl::json::parse(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let object = parsed.as_object().cloned().ok_or_else(|| MediaError::Payload(port.to_string(), "params:in payload must be a JSON object".into()))?;
                let fixture = &doc.snapshot.fixture;
                let mut operations = Vec::new();
                for (target_id, value) in object.iter() {
                    let Some(number) = value.as_f64() else { continue };
                    let Some((_index, widget)) = fixture.widgets.iter().enumerate().find(|(_, widget)| crate::widget_id(widget) == target_id) else { continue };
                    if let semio_framework_artifact_flow_flow::Widget::InputSlider { id, label, min, max, step, .. } = widget {
                        operations.push(Generation3dMutation::UpdateWidget(crate::standards::v1::subsets::any::schema::mutations::update_widget::UpdateWidget {
                            widget: semio_framework_artifact_flow_flow::Widget::InputSlider { id: id.clone(), label: label.clone(), value: number, min: *min, max: *max, step: *step },
                        }));
                    }
                }
                Ok(Emit::mutations(operations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Generation3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Generation3dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(dsl::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        let u64_arg = |keys: &[&str]| -> Option<u64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|number| number as u64)))) };
        match action {
            "setActiveExample" => Ok(Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "nodeGraphEdit" => Ok(Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: str_arg(&["operationsJson", "operations_json"]).or_else(|| args.get("operations").map(dsl::json::to_json_string)).unwrap_or_else(|| "[]".into()),
            })),
            "deleteSelection" => Ok(Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "removeWidget" => Ok(Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "moveMediaNode" => Ok(Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: f64_arg(&["x"]).unwrap_or(0.0), y: f64_arg(&["y"]).unwrap_or(0.0) })),
            "addWidget" => Ok(Generation3dCommand::AddWidget(add_widget::AddWidget { kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()), x: f64_arg(&["x"]), y: f64_arg(&["y"]) })),
            "patchFlowWidgets" => Ok(Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets {
                widget_ids: {
                    let mut ids = string_list("widgetIds");
                    if ids.is_empty() {
                        ids = string_list("widget_ids");
                    }
                    ids
                },
                field: str_arg(&["field"]).unwrap_or_default(),
                value: f64_arg(&["value"]),
            })),
            "reorganize" => Ok(Generation3dCommand::Reorganize(reorganize::Reorganize {})),
            "translateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids, dx: f64_arg(&["dx"]).unwrap_or(0.0), dy: f64_arg(&["dy"]).unwrap_or(0.0), dz: f64_arg(&["dz"]).unwrap_or(0.0) }))
            }
            "rotateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::RotateSelection(rotate_selection::RotateSelection {
                    node_ids,
                    ax: f64_arg(&["ax"]).unwrap_or(0.0),
                    ay: f64_arg(&["ay"]).unwrap_or(0.0),
                    az: f64_arg(&["az"]).unwrap_or(0.0),
                    angle: f64_arg(&["angle"]).unwrap_or(0.0),
                }))
            }
            "scaleSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids, sx: f64_arg(&["sx"]).unwrap_or(1.0), sy: f64_arg(&["sy"]).unwrap_or(1.0), sz: f64_arg(&["sz"]).unwrap_or(1.0) }))
            }
            "addGeneration" => Ok(Generation3dCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name"]).unwrap_or_default() })),
            "updateGenerationValues" => {
                let value = args.get("value").cloned().unwrap_or(dsl::DslValue::Null);
                Ok(Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value,
                }))
            }
            "nodeGraphViewport" => Ok(Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: parse_flow_camera_json(&args) })),
            "setLodMode" => Ok(Generation3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "setShowMode" => Ok(Generation3dCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode", "show_mode"]).unwrap_or_default() })),
            "toggleSun" => Ok(Generation3dCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setSunAzimuth" => Ok(Generation3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunElevation" => Ok(Generation3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunIntensity" => Ok(Generation3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setCamera" => Ok(Generation3dCommand::SetCamera(set_camera::SetCamera { camera: parse_preview_camera_json(&args) })),
            "selectGeneration" => Ok(Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "flowEvalTick" => Ok(Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {})),
            "flowEvalResolve" => Ok(Generation3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve {
                node_hash: u64_arg(&["nodeHash", "node_hash"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
            })),
            "flowTessellateResolve" => Ok(Generation3dCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve {
                node_hash: u64_arg(&["nodeHash", "node_hash"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
            })),
            "cancelPreviewEval" => Ok(Generation3dCommand::CancelPreviewEval(cancel_preview_eval::CancelPreviewEval {})),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit`/`{translate,rotate,scale}Selection` read the `graph`
    /// interaction domain directly (bypassing the `app_commands!`-generated `dispatch`, whose
    /// per-row `$module::handle(payload, doc, cfg, ctx)` signature is framework-fixed and has no
    /// `interaction` slot) — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &Generation3dCommand,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, Self::DraftMutation>, Fault> {
        with_scratch_session(|session| match command {
            Generation3dCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction, session),
            Generation3dCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, interaction, session),
            Generation3dCommand::TranslateSelection(payload) => translate_selection::apply(payload, doc, cfg, interaction, session),
            Generation3dCommand::RotateSelection(payload) => rotate_selection::apply(payload, doc, cfg, interaction, session),
            Generation3dCommand::ScaleSelection(payload) => scale_selection::apply(payload, doc, cfg, interaction, session),
            _ => command.dispatch(doc, cfg, session),
        })
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every widget's visible ports become `handle`
    /// targets (`{nodeId}@{portId}`, byte-identical to the node-graph's own pick ids) parented to
    /// their widget, which is what makes `HoverSpec { transitive: true }` light up every channel's
    /// preview geometry from a single node hover, and resolve a preview-instance hover back to its
    /// node. Every top-level widget is a "node" (root unless
    /// nested in a `Widget::Cluster`'s own `tree.neurons`, where each nested `Neuron` becomes a "node"
    /// parented to its owning cluster's widget id — the DAG-parent-links transitive-hover source: hovering
    /// a Cluster's own tree item transitively covers every widget nested inside it). Synapses become
    /// "edge" targets, parented to nothing (edges are leaves, not containers).
    fn interaction_topology(doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>) -> InteractionTopology {
        fn walk_neuron(neuron: &semio_framework_artifact_flow_flow::neural::Neuron, parent: String, ordered: &mut Vec<TopologyNode>) {
            ordered.push(TopologyNode { id: neuron.id.clone(), granularity: "node".into(), parent: Some(parent) });
            if let Some(tree) = &neuron.tree {
                for child in &tree.neurons {
                    walk_neuron(child, neuron.id.clone(), ordered);
                }
            }
        }
        let fixture = &doc.snapshot.fixture;
        let mut ordered = Vec::new();
        let ports_by_node = generation3d_port_ids_by_node(fixture);
        for widget in &fixture.widgets {
            let id = crate::widget_id(widget).to_string();
            ordered.push(TopologyNode { id: id.clone(), granularity: "node".into(), parent: None });
            for port in ports_by_node.get(&id).into_iter().flatten() {
                ordered.push(TopologyNode { id: port.clone(), granularity: "handle".into(), parent: Some(id.clone()) });
            }
            if let semio_framework_artifact_flow_flow::Widget::Cluster { tree, .. } = widget {
                for child in &tree.neurons {
                    walk_neuron(child, id.clone(), &mut ordered);
                }
            }
        }
        for synapse in &fixture.synapses {
            ordered.push(TopologyNode { id: synapse.id.clone(), granularity: "edge".into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert("graph".to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes.
    fn pending_effects(doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>) -> Vec<Effect> {
        with_scratch_session(|session| {
            let pending = crate::standards::v1::subsets::any::schema::with_host_session(&doc.snapshot.fixture, session, |host, session| session.sync(host));
            if pending {
                vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(104), action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
            } else {
                Vec::new()
            }
        })
    }

    /// 🕹️ The marks-free entry point the framework still offers (no owner, no transient, no
    /// interaction) — every live window goes through `render_with_request_context` instead.
    fn render(body_key: &str, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        with_scratch_session(|session| generation3d_render_body(body_key, doc.snapshot, cfg.snapshot, None, None, view_state, &PreviewInteractionMarks::default(), session))
    }

    /// 🕹️ Resolves the live `graph` hover/selection once per render and threads it into every
    /// window body — the node graph paints the hovered node/port, the world previews paint the
    /// hovered/selected instances, and the inspection panel finally sees a real selection.
    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Generation3dTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let preview_eval_text = transient.window::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.as_deref());
        let marks = PreviewInteractionMarks::from_interaction(interaction);
        owner
            .with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
                owner.with_session(|session| generation3d_render_body(body_key, doc.snapshot, cfg.snapshot, preview_eval_text, transient.snapshot.generation_preview_text.as_deref(), view_state, &marks, session))
            })
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("generation3d.eval-session-owner", error.message))?
    }

    fn window_measures(_doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let measures = edit_preview::preview_window_measures(config, generation3d_action);
        HashMap::from([
            (flow_window::GENERATION_3D_PLAY_WINDOW_MAIN.to_string(), flow_window::window_measures(&config.lod_mode, generation3d_action)),
            (edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
            (generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
        ])
    }

    /// 🗂️ The marks-free entry point the framework still offers — every live right-click goes
    /// through `context_menu_with_request_context` instead, so this delegate builds the same menu
    /// against an empty selection.
    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &PreviewInteractionMarks::default(), registry)
    }

    /// 🕹️ `context_menu`'s interaction-aware twin — the one the runtime actually calls
    /// (`VcsArtifactApp::context_menu`). `ContextMenuRequest.surface.selection` only ever carries what
    /// the clicked surface itself painted, so a world-preview instance selected from the node graph
    /// (or vice versa) never reaches a menu row through it; `interaction` is the authoritative
    /// framework-owned `graph` selection, the same one `render_with_request_context` paints from.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &PreviewInteractionMarks::from_interaction(interaction), registry)
    }
}

impl Generation3dPlayApp {
    /// 🗂️ The ONE context-menu implementation — both `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here.
    ///
    /// Grouped disclosure: `reorganize` stays top-level, the transform trio appears only for a real
    /// selection, creation/removal/generation methods fold into taxonomy groups, and
    /// `delete-selection` stays a direct destructive item last.
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        _cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        marks: &PreviewInteractionMarks,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
        let labels = generation3d_labels(view_state);
        let is_de = view_state.locale == semio_framework_plugin::Locale::De;
        let (selected_nodes, selected_edges) = marks.graph_selection_domains(&doc.snapshot.fixture);
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected_nodes, &selected_edges);
        let has_selection = !nodes.is_empty() || !edges.is_empty();
        let mut menu = Menu::of(registry).action("reorganize");
        if has_selection {
            menu = menu.action("translateSelection").action("rotateSelection").action("scaleSelection");
        }
        menu = menu.group("create", |m| m.action("addWidget").action("addGeneration"));
        if has_selection {
            menu = menu.group("targets", |m| m.action("removeWidget").action("removeGeneration"));
        }
        menu = menu.group("methods", |m| m.action("renameGeneration").action("updateGenerationValues").action("patchFlowWidgets"));
        if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
            menu = menu.item(spec);
        }
        menu.build()
    }
}

//#endregion 🔖️Generation3dPlayApp

//#region 🔖️Manifest
pub fn create_generation3d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::GENERATION3D_DIALECT).document(["semio", "procedural", "3d"])
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) }))
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung aufnehmen"), "runtime", ActionKind::View) }))
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowTessellateResolve", LocalizedLabel::native("Resolve Preview Tessellation", "Vorschau-Tessellierung aufnehmen"), "runtime", ActionKind::View) }))
            .command(migrated_command(CommandDefinition::bounded_catalog("cancelPreviewEval", LocalizedLabel::native("Cancel Preview Computation", "Vorschauberechnung abbrechen"), "runtime", ActionKind::View)))
            .artifact_kind(artifact_kind())
            .icon_id("workflow")
            .mode_def(edit::definition())
            .mode_def(generate::definition())
            .default_mode_id(edit::GENERATION_3D_PLAY_MODE_EDIT)
            .mode_layout(generate::GENERATION_3D_PLAY_MODE_GENERATE, generate::GENERATION_3D_PLAY_LAYOUT_GENERATE)
            .window_kind_def(flow_window::definition())
            .window_kind_def(edit_preview::definition())
            .window_kind_def(generations::definition())
            .window_kind_def(form::definition())
            .window_kind_def(generate_preview::definition())
            .default_layout(edit::layout())
            .named_layout(generate::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Document-mutating operations — dispatched as VCS operations with a true inverse.
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"))
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .action_with(categorized_action("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"), ActionKind::Mutation, "targets"))
            .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(categorized_action("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Mutation, "create"))
            .action_with(categorized_action("patchFlowWidgets", LocalizedLabel::native("Patch Flow Widgets", "Flow-Elemente aktualisieren"), ActionKind::Mutation, "methods"))
            .action_with(categorized_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Mutation, "create"))
            .action_with(categorized_action("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"), ActionKind::Mutation, "targets"))
            .action_with(categorized_action("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"), ActionKind::Mutation, "methods"))
            .action_with(categorized_action("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"), ActionKind::Mutation, "methods"))
            // 👁️ Ephemeral view actions — world picking, graph camera, sun/LOD/show-mode display toggles, preview camera.
            // Selection/hover are the framework's `graph` interaction domain now (`.interaction(...)`
            // below) — the six framework verbs (`interactionSelect`/`interactionHover`/`clearSelection`/
            // `selectAll`/`setSelectionMode`/`setInteractionGranularity`) auto-inject.
            .action_with(ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
            .view_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .view_action("selectGeneration", LocalizedLabel::native("Set Generation", "Generation auswählen"))
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeWidget", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveMediaNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addWidget", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchFlowWidgets", InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("updateGenerationValues", InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("flowEvalTick", InteractiveJobClassification::Migrated)
            .action_interactive_job("flowEvalResolve", InteractiveJobClassification::Migrated)
            .action_interactive_job("flowTessellateResolve", InteractiveJobClassification::Migrated)
            .action_interactive_job("cancelPreviewEval", InteractiveJobClassification::Migrated)
            .action_args("addWidget", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("neuron", LocalizedLabel::native("Neuron", "Neuron")),
                    ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                    ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                    ActionArgOption::new("outputPreview", LocalizedLabel::native("Preview", "Vorschau")),
                ]).default_value(&"inputSlider"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")),
                    ActionArgOption::new(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")),
                ]).required(),
            ])
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
            .window_kind_utilities(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
            // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
            // one domain over the flow-graph widget DAG, node/edge/handle granularities,
            // `HierarchyProvider::Topology` (see `Generation3dPlayApp::interaction_topology` below) —
            // transitive hover is the headline feature: hovering a Cluster group node highlights every
            // widget nested in its tree.
            .interaction(InteractionDefinition {
                id: "graph".into(),
                label: LocalizedLabel::native("Graph", "Graph"),
                granularities: vec![
                    GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                    GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
                    GranularityDefinition { id: "handle".into(), label: LocalizedLabel::native("Handle", "Griff"), icon_id: "move".into() },
                ],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
            .window_kind_interactions(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec![InteractionRef::new("graph")])
            .window_kind_interactions(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Generation3dPlayApp::config_spec())
            .io(semio_framework::io::resolve_ready(generation3d_io()))
            .build_definition()
}

/// 📚️ The eight bundled `📚️examples/🎬️<slug>` fixtures, in `schema::is_generation3d_example_id`'s
/// order — plugged into `.editor_with_examples::<Generation3dPlayApp>(create_generation3d_app(), …)`
/// at the plugin root so the react shell's example dropdown (`NavbarExampleSelect/🟦️.tsx`, fed by
/// `activePluginManifest.examples`) stops being hidden for `generation3d`.
pub fn examples() -> Vec<ExampleSource> {
    vec![
        crate::examples::art_generation3d_hexagonal_mushroom_column::source(),
        crate::examples::art_generation3d_rectangle_extrude_volume::source(),
        crate::examples::art_generation3d_sphere_cut_with_torus::source(),
        crate::examples::art_generation3d_box_fillet_preview::source(),
        crate::examples::art_generation3d_sphere_box_fuse::source(),
        crate::examples::art_generation3d_face_sweep_extrude::source(),
        crate::examples::art_generation3d_rectangle_wire_preview::source(),
        crate::examples::art_generation3d_box_shell_preview::source(),
    ]
}
//#endregion 🔖️Manifest

//#region 🔖️ArtifactIo
/// 🔌️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// this app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_generation3d_app` declares via `.artifact_kind(...)`; `params:in`/`geometry:out` are the
/// workflow-specific ports beyond the implicit document in/out ports.
pub async fn generation3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        "generation.3d",
        MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "3d.generation".into(), name: "3D Generation".into(), dimension: "3d".into(), component_kind: "generation3d".into() },
    )
    .await
    .with_ports(vec![
        semio_framework_plugin::MediaPortSpec {
            id: "params:in".into(),
            label: "Parameters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: None,
            required: false,
            multiplicity: semio_framework::PortMultiplicity::One,
        },
        semio_framework_plugin::MediaPortSpec {
            id: "geometry:out".into(),
            label: "Geometry".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            kind_id: Some("3d.mesh".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        },
    ])
    .await
}
//#endregion 🔖️ArtifactIo

//#region 🔖️PreviewPipeline
/// 🎥️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// every function here references [`Generation3dConfig`] (directly, or is reachable only from a
/// function that does), which made this app behavior, not artifact-schema-pure document compute; the
/// snapshot-pure fixture/gumball helpers stayed in `crate::standards::v1::subsets::any::schema` instead.
/// ⏱️ Face/edge units one `tessellate` round trip may spend. Deliberately small: the reactor faults
/// an instance whose single `maintenance_step` runs past 8 ms, so the preview buys many cheap round
/// trips instead of one uncancellable synchronous solve.
pub const PREVIEW_TESSELLATE_STEP_BUDGET: u32 = 24;

pub fn preview_tolerance(lod_mode: &str) -> f64 {
    match lod_mode {
        "coarse" => 0.15,
        "fine" => 0.02,
        _ => 0.05,
    }
}

pub fn preview_camera_json(cfg: &Generation3dConfig) -> String {
    semio_framework_ui::wgpu::world3d_camera_json(cfg.preview_camera.position, cfg.preview_camera.target, cfg.preview_camera.fov)
}

//#region 🔖️PreviewInteraction
/// 🕹️ The framework interaction domain every generation3d window is bound to (see the
/// `window_kind_interactions` calls in the manifest stitch): the node graph, the edit preview and
/// the generate preview all read and write the same `graph` hover/selection.
pub const GENERATION_3D_INTERACTION_DOMAIN: &str = "graph";

/// 🐁️ The channel a pointer hovers on. `InteractionState.hover` holds exactly one live channel per
/// domain, so reading any other channel reads empty rather than stale.
pub const GENERATION_3D_INTERACTION_CHANNEL: &str = "pointer";

/// 🎯️ The granularity a plain world-3d instance pick/hover reports. A preview instance id is
/// channel-qualified (`{widgetId}@{channel}#{index}`), which is the node graph's own `handle`
/// (port) target shape — so a world hit and a graph port hit land on the same granularity.
pub const GENERATION_3D_INTERACTION_GRANULARITY: &str = "handle";

/// 🕹️ One render's resolved `graph`-domain marks.
///
/// A preview instance id is `{widgetId}@{channel}#{index}`, so an id counts as marked when the
/// domain names the instance itself, its channel (`{widgetId}@{channel}` — byte-identical to the
/// port id the node graph's own picks already use) or its widget (`{widgetId}`). That three-level
/// match is exactly what makes hover bidirectional: hovering a node in the graph lights up every
/// one of its channels' preview geometry, and hovering one preview instance in the world lights up
/// its node — and its port — back in the graph.
///
/// 🎯️ The world window reports the PORT, not the instance: an instance carries
/// `interactionId = {widgetId}@{channel}` (see `preview_payload`) because `validate_state` prunes
/// any hover/selection id absent from `interaction_topology`, and the per-index instance count is
/// evaluation-derived so it cannot be declared there.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PreviewInteractionMarks {
    pub hovered: std::collections::BTreeSet<String>,
    pub selected: std::collections::BTreeSet<String>,
}

impl PreviewInteractionMarks {
    /// 🕹️ Reads the framework-owned domain: hover off the ephemeral pointer channel, selection off
    /// the persisted interaction store. The app stores neither itself.
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self { hovered: interaction.hover(GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_CHANNEL).ids.iter().cloned().collect(), selected: interaction.selection(GENERATION_3D_INTERACTION_DOMAIN).ids.iter().cloned().collect() }
    }

    fn marked(set: &std::collections::BTreeSet<String>, widget_id: &str, channel: &str, index: usize) -> bool {
        set.contains(widget_id) || set.contains(&format!("{widget_id}@{channel}")) || set.contains(&format!("{widget_id}@{channel}#{index}"))
    }

    pub fn hovers(&self, widget_id: &str, channel: &str, index: usize) -> bool {
        Self::marked(&self.hovered, widget_id, channel, index)
    }

    pub fn selects(&self, widget_id: &str, channel: &str, index: usize) -> bool {
        Self::marked(&self.selected, widget_id, channel, index)
    }

    /// 🕸️ The widget id behind any interaction id — `{w}`, `{w}@{c}` and `{w}@{c}#{i}` all resolve
    /// to `{w}`.
    pub fn widget_of(id: &str) -> &str {
        let base = id.split('#').next().unwrap_or(id);
        base.split('@').next().unwrap_or(base)
    }

    /// 🕸️ The channel (port) id behind an interaction id, `None` for a bare widget id.
    pub fn port_of(id: &str) -> Option<&str> {
        let base = id.split('#').next().unwrap_or(id);
        base.split_once('@').map(|(_, port)| port)
    }

    /// 🕸️ `(nodeId, portId)` the node graph paints as hovered — a preview instance hovered in the
    /// world resolves to its node AND its channel, so the graph highlights the exact output port
    /// whose geometry the pointer is over.
    pub fn hovered_graph_target(&self) -> Option<(String, Option<String>)> {
        let id = self.hovered.iter().next()?;
        Some((Self::widget_of(id).to_string(), Self::port_of(id).map(str::to_string)))
    }

    /// 🕸️ Every id the node graph should highlight: each hovered id plus the widget it belongs to,
    /// deduplicated and ordered.
    pub fn graph_highlight_ids(&self) -> Vec<String> {
        let mut ids: std::collections::BTreeSet<String> = self.hovered.clone();
        for id in &self.hovered {
            ids.insert(Self::widget_of(id).to_string());
        }
        ids.into_iter().collect()
    }

    /// 🕸️ The `graph` selection split into the node and edge domains a context menu addresses. A
    /// synapse id is an `edge`; everything else projects onto its owning widget id (a preview
    /// instance `{w}@{c}#{i}` selected in the world therefore targets its node exactly like a click
    /// in the graph does).
    pub fn graph_selection_domains(&self, fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> (Vec<String>, Vec<String>) {
        let synapses: std::collections::BTreeSet<&str> = fixture.synapses.iter().map(|synapse| synapse.id.as_str()).collect();
        let mut nodes = std::collections::BTreeSet::new();
        let mut edges = std::collections::BTreeSet::new();
        for id in &self.selected {
            if synapses.contains(id.as_str()) {
                edges.insert(id.clone());
            } else {
                nodes.insert(Self::widget_of(id).to_string());
            }
        }
        (nodes.into_iter().collect(), edges.into_iter().collect())
    }

    /// 🕸️ The `graph` selection projected onto widget ids — what `NodeGraphScene::selection` paints.
    pub fn graph_selection_ids(&self) -> Vec<String> {
        self.selected.iter().map(|id| Self::widget_of(id).to_string()).collect::<std::collections::BTreeSet<String>>().into_iter().collect()
    }
}
//#endregion 🔖️PreviewInteraction

/// 🧭️ World-3d selection payload with the host-owned gumball utility spliced in, so the transform
/// handles follow the window-projected `ViewModel.active_utility_id`, and with the live
/// `graph` marks `render_with_request_context` resolved — the gumball now shows for a real
/// selection instead of always reporting empty. `"rectangle"` (the pre-migration default
/// `selection_method`) is hardcoded: the framework tracks no persistent "last marquee method"
/// outside a live gesture.
pub fn preview_selection_json(cfg: &Generation3dConfig, active_utility: &str, payload: &PreviewPayload) -> String {
    let mut value = dsl::json::parse(&semio_framework_plugin::world3d_selection_json("rectangle", &payload.selected_ids, payload.hovered_id.as_deref())).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
    let show_mode = if cfg.show_mode.is_empty() { "shaded" } else { cfg.show_mode.as_str() };
    let (show_edges, selection_mode) = match show_mode {
        "wireframe" => (true, "mesh"),
        "points" => (false, "mesh"),
        "shaded+edges" => (true, "mesh"),
        _ => (false, "mesh"),
    };
    if let Some(object) = value.as_object_mut() {
        object.insert("transformMode", dsl::json::Value::String(active_utility.to_string()));
        object.insert("gumballActive", dsl::json::Value::Bool(!payload.selected_ids.is_empty() && !active_utility.is_empty()));
        object.insert("showEdges", dsl::json::Value::Bool(show_edges));
        object.insert("selectionMode", dsl::json::Value::String(selection_mode.to_string()));
        object.insert("granularity", dsl::json::Value::String(selection_mode.to_string()));
    }
    dsl::json::to_string(&value)
}

fn merge_status_json(computing: Option<String>, preview_status: Option<String>) -> Option<String> {
    match (computing, preview_status) {
        (Some(c), Some(p)) => {
            let mut computing_object = dsl::json::parse(&c).ok().and_then(|value| value.as_object().cloned()).unwrap_or_else(|| {
                let mut fallback = dsl::json::Object::new();
                fallback.insert("computing", dsl::json::Value::Bool(true));
                fallback
            });
            let preview_object = dsl::json::parse(&p).ok().and_then(|value| value.as_object().cloned()).unwrap_or_default();
            for (key, value) in preview_object.iter() {
                computing_object.insert(key, value.clone());
            }
            Some(dsl::json::to_string(&dsl::json::Value::Object(computing_object)))
        }
        (Some(c), None) => Some(c),
        (None, Some(p)) => Some(p),
        (None, None) => None,
    }
}

/// 👁️ Merges the session's live "still computing" flag, the resumable tessellation's progress and a
/// fresh `preview_status_json` result into the one status object the preview window publishes.
pub fn preview_scene_status_json(session: &FlowEvalSession, preview_status: Option<String>) -> Option<String> {
    let computing = session.pending().then(|| r#"{"computing":true}"#.to_string());
    merge_status_json(merge_status_json(computing, Some(preview_progress_status_json(session))), preview_status)
}

/// 📈 The schema-first tessellation progress object: `phase` (wire tag) and its `phaseLabel`
/// English/German pair, the monotone `progress` counters and ratio, `cancellable` (drives the
/// `cancelPreviewEval` affordance) and any typed validate-gate `diagnostics`.
pub fn preview_progress_status_json(session: &FlowEvalSession) -> String {
    let status = session.preview_tessellate_status();
    let (english, german) = status.phase.labels();
    let mut label = dsl::json::Object::new();
    label.insert("en", dsl::json::Value::String(english.to_string()));
    label.insert("de", dsl::json::Value::String(german.to_string()));
    let mut progress = dsl::json::Object::new();
    progress.insert("unitsDone", dsl::json::Value::from(u64::from(status.units_done)));
    progress.insert("unitsTotal", dsl::json::Value::from(u64::from(status.units_total)));
    progress.insert("facesDone", dsl::json::Value::from(u64::from(status.faces_done)));
    progress.insert("facesTotal", dsl::json::Value::from(u64::from(status.faces_total)));
    progress.insert("inFlight", dsl::json::Value::from(u64::from(status.in_flight)));
    progress.insert("ratio", dsl::json::Value::from(status.ratio()));
    let mut object = dsl::json::Object::new();
    object.insert("phase", dsl::json::Value::String(status.phase.tag().to_string()));
    object.insert("phaseLabel", dsl::json::Value::Object(label));
    object.insert("progress", dsl::json::Value::Object(progress));
    object.insert("cancellable", dsl::json::Value::Bool(status.is_cancellable()));
    object.insert("cancelAction", dsl::json::Value::String("cancelPreviewEval".to_string()));
    if status.diagnostics > 0 {
        object.insert("diagnosticCount", dsl::json::Value::from(u64::from(status.diagnostics)));
        let entries: Vec<dsl::json::Value> = session
            .preview_diagnostic_entries()
            .into_iter()
            .map(|(handle, issues)| {
                let mut entry = dsl::json::Object::new();
                entry.insert("handle", dsl::json::Value::String(handle.to_string()));
                entry.insert("issues", dsl::json::parse(issues).unwrap_or(dsl::json::Value::Array(Vec::new())));
                dsl::json::Value::Object(entry)
            })
            .collect();
        object.insert("diagnostics", dsl::json::Value::Array(entries));
    }
    dsl::json::to_string(&dsl::json::Value::Object(object))
}

pub fn is_brep_geometry_handle(handle: &str) -> bool {
    if handle.is_empty() {
        return false;
    }
    if handle.starts_with("solid-")
        || handle.starts_with("shell-")
        || handle.starts_with("face-")
        || handle.starts_with("wire-")
        || handle.starts_with("edge-")
        || handle.starts_with("vertex-")
        || handle.starts_with("compound-")
        || handle.starts_with("curve-")
        || handle.starts_with("surface-")
    {
        return true;
    }
    // Blake3 hex digests minted by `BrepKernel::mint` (no kind prefix).
    handle.len() == 64 && handle.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

/// 🔌️ Point/vector geometry synthesized without a kernel round-trip, for a math-style output
/// channel that carries `x`/`y`/`z` coordinates instead of a brep handle.
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewInlineGeometry {
    Point { x: f64, y: f64, z: f64 },
    Vector { x: f64, y: f64, z: f64 },
}

/// 🔌️ One previewable value found on one output channel of one widget — the channel-aware
/// replacement for the old unordered handle flattening, so preview instance ids can be
/// channel-qualified (`{widgetId}@{channel}#{index}`, see `preview_payload`).
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewChannelItem {
    pub channel: String,
    pub index: usize,
    pub handle: String,
    pub inline: Option<PreviewInlineGeometry>,
}

/// 🔎️ A `$schema: "list"` dictionary's entries in index order (`"0"`, `"1"`, …) — the wire form
/// `semio_framework_artifact_flow_flow::neural::Dictionary` lists actually take (an object with numeric-string keys, not a JSON
/// array), so ordering has to be recovered by parsing the keys rather than trusting map iteration.
fn preview_channel_list_entries(map: &dsl::json::Object) -> Vec<&dsl::json::Value> {
    let mut entries: Vec<(usize, &dsl::json::Value)> = map.iter().filter_map(|(key, value)| key.parse::<usize>().ok().map(|index| (index, value))).collect();
    entries.sort_by_key(|(index, _)| *index);
    entries.into_iter().map(|(_, value)| value).collect()
}

/// 🔎️ Depth-first walk of one channel's evaluated value, emitting one [`PreviewChannelItem`] per
/// geometry-bearing leaf in encounter order. Arrays and `$schema: "list"` dictionaries recurse;
/// a handle passing `is_brep_geometry_handle` or an `x`/`y`/`z` point/vector is a leaf; everything
/// else (numbers, strings, booleans, plain dictionaries) is pure data and yields nothing.
fn collect_preview_channel_items(channel: &str, value: &dsl::json::Value, index: &mut usize, items: &mut Vec<PreviewChannelItem>) {
    match value {
        dsl::json::Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(dsl::json::Value::as_str) {
                if is_brep_geometry_handle(handle) {
                    items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: handle.into(), inline: None });
                    *index += 1;
                    return;
                }
            }
            if map.get("$schema").and_then(dsl::json::Value::as_str) == Some("list") {
                for entry in preview_channel_list_entries(map) {
                    collect_preview_channel_items(channel, entry, index, items);
                }
                return;
            }
            let coords = ["x", "y", "z"].into_iter().map(|key| map.get(key).and_then(dsl::json::Value::as_f64)).collect::<Option<Vec<_>>>();
            if let Some(coords) = coords {
                let (x, y, z) = (coords[0], coords[1], coords[2]);
                let inline = if map.get("$schema").and_then(dsl::json::Value::as_str) == Some("vector") { PreviewInlineGeometry::Vector { x, y, z } } else { PreviewInlineGeometry::Point { x, y, z } };
                items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: String::new(), inline: Some(inline) });
                *index += 1;
            }
        }
        dsl::json::Value::Array(list) => {
            for entry in list {
                collect_preview_channel_items(channel, entry, index, items);
            }
        }
        _ => {}
    }
}

/// 🔌️ Channel-by-channel enumeration of one widget's preview-bearing values: sorted `"out"`
/// channel keys (falling back to `"in"` only when the widget has no `"out"` at all), each walked
/// depth-first into its geometry-bearing leaves. Replaced the old flat,
/// unordered handle collection — every call site that needs handles/points/vectors for preview routes
/// through this one function now.
pub fn preview_channel_items_for_widget(eval: &dsl::json::Value, widget_id: &str) -> Vec<PreviewChannelItem> {
    let Some(widget_eval) = eval.get(widget_id) else {
        return Vec::new();
    };
    let Some(channels) = widget_eval.get("out").or_else(|| widget_eval.get("in")) else {
        return Vec::new();
    };
    let Some(map) = channels.as_object() else {
        return Vec::new();
    };
    let mut keys: Vec<&str> = map.iter().map(|(key, _)| key).collect();
    keys.sort();
    let mut items = Vec::new();
    for key in keys {
        let mut index = 0usize;
        if let Some(value) = map.get(key) {
            collect_preview_channel_items(key, value, &mut index, &mut items);
        }
    }
    items
}

/// 👁️ Whether a widget contributes preview geometry at all. A `Neuron` carries its own author-set
/// `preview` toggle; an `OutputPreview` is a preview by construction; a `Cluster` has no toggle of
/// its own (`semio_framework_artifact_flow_flow::neural::Neuron` — its inner neurons — carries none either), so its contract
/// output channels always preview, which is the only way a grouped sub-graph's geometry reaches
/// the 3D world at all.
pub fn widget_previews(widget: &semio_framework_artifact_flow_flow::Widget) -> bool {
    matches!(widget, semio_framework_artifact_flow_flow::Widget::Neuron { preview: true, .. } | semio_framework_artifact_flow_flow::Widget::OutputPreview { .. } | semio_framework_artifact_flow_flow::Widget::Cluster { .. })
}

fn mesh_has_preview_geometry(data: &semio_framework_plugin::MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
}

/// 🔌️ Half-extent (world units) of the axis cross drawn for a `PreviewInlineGeometry::Point`.
const PREVIEW_POINT_MARKER_HALF_EXTENT: f64 = 0.05;

/// 🔌️ A small axis cross at `(x, y, z)` — the point-channel preview marker, built without a
/// kernel round-trip. Carries both `positions` (so `"points"` show mode still has something to
/// draw once `apply_show_mode_mesh` strips `edge_positions`) and the cross itself as edges.
fn point_marker_mesh(x: f64, y: f64, z: f64) -> semio_framework_plugin::MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    let e = PREVIEW_POINT_MARKER_HALF_EXTENT as f32;
    semio_framework_plugin::MeshData { positions: vec![x, y, z], edge_positions: vec![x - e, y, z, x + e, y, z, x, y - e, z, x, y + e, z, x, y, z - e, x, y, z + e], ..Default::default() }
}

/// 🔌️ A line segment from the world origin to `(x, y, z)` — the vector-channel preview marker,
/// built without a kernel round-trip.
fn vector_marker_mesh(x: f64, y: f64, z: f64) -> semio_framework_plugin::MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    semio_framework_plugin::MeshData { positions: vec![0.0, 0.0, 0.0, x, y, z], edge_positions: vec![0.0, 0.0, 0.0, x, y, z], ..Default::default() }
}

fn apply_show_mode_mesh(mut data: semio_framework_plugin::MeshData, show_mode: &str) -> semio_framework_plugin::MeshData {
    let show_mode = match show_mode {
        "solid" | "shaded" | "shaded+edges" | "wireframe" | "points" => show_mode,
        _ => "shaded",
    };
    match show_mode {
        "wireframe" => {
            data.positions.clear();
            data.normals.clear();
            data.indices.clear();
            data.face_ids.clear();
            data
        }
        "points" => {
            data.indices.clear();
            data.normals.clear();
            data.edge_positions.clear();
            data
        }
        _ => data,
    }
}

pub fn preview_status_json(eval_json: &str, fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> Option<String> {
    let eval = dsl::json::parse(eval_json).ok()?;
    if eval.get("error").and_then(dsl::json::Value::as_str).is_some() {
        let mut error_object = dsl::json::Object::new();
        error_object.insert("error", eval.get("error").cloned().unwrap_or(dsl::json::Value::Null));
        return Some(dsl::json::to_string(&dsl::json::Value::Object(error_object)));
    }
    let mut errors = dsl::json::Object::new();
    for widget in &fixture.widgets {
        let id = crate::widget_id(widget).to_string();
        let Some(entry) = eval.get(&id) else { continue };
        if let Some(error) = entry.get("error").and_then(dsl::json::Value::as_str) {
            errors.insert(id, dsl::json::Value::String(error.to_string()));
        }
    }
    if errors.is_empty() {
        None
    } else {
        let mut wrapper = dsl::json::Object::new();
        wrapper.insert("widgetErrors", dsl::json::Value::Object(errors));
        Some(dsl::json::to_string(&dsl::json::Value::Object(wrapper)))
    }
}

/// 🧵️ Pure per-render mesh lookup: the session's resolved `pack` mesh body decodes into typed
/// arrays (no JSON number-array parse), and only a session-free caller falls back to tessellating.
fn mesh_data_for_preview_handle(handle: &str, tolerance: f64, session: Option<&FlowEvalSession>) -> Option<semio_framework_plugin::MeshData> {
    if let Some(session) = session {
        if let Some(pack) = session.preview_mesh_pack(handle) {
            if let Some(data) = decode_preview_mesh_pack(pack) {
                if mesh_has_preview_geometry(&data) {
                    return Some(data);
                }
            }
        }
        if session.preview_diagnostics(handle).is_some() {
            return None;
        }
    }
    let data = semio_framework_os_flow::tessellate_geometry(handle, tolerance).ok()?;
    mesh_has_preview_geometry(&data).then_some(data)
}

/// 🎒️ Decodes a base64 `pack` mesh body the extension shipped back — the single decode seam every
/// preview reader goes through.
fn decode_preview_mesh_pack(base64_body: &str) -> Option<semio_framework_plugin::MeshData> {
    let bytes = semio_framework_os_flow::brep_geometry::decode_base64(base64_body).ok()?;
    semio_framework_os_flow::brep_geometry::decode_mesh_pack(&bytes).ok()
}

/// 🧊 Geometry handles on preview widgets that still need an extension tessellate.
pub fn pending_preview_tessellate_handles(eval_json: &str, fixture: &semio_framework_artifact_flow_flow::FlowFixture, session: &FlowEvalSession) -> Vec<String> {
    if eval_json.is_empty() {
        return Vec::new();
    }
    let eval = dsl::json::parse(eval_json).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
    let mut handles = Vec::new();
    for widget in &fixture.widgets {
        let preview = widget_previews(widget);
        if !preview {
            continue;
        }
        let id = crate::widget_id(widget).to_string();
        for handle in preview_channel_items_for_widget(&eval, &id).into_iter().filter_map(|item| (!item.handle.is_empty()).then_some(item.handle)) {
            if session.preview_diagnostics(&handle).is_some() {
                continue;
            }
            let ready = session.preview_mesh_pack(&handle).and_then(|pack| {
                let data = decode_preview_mesh_pack(pack)?;
                mesh_has_preview_geometry(&data).then_some(())
            });
            if ready.is_none() {
                handles.push(handle);
            }
        }
    }
    handles
}

/// 📨 Extension invocations that tessellate preview handles inside the owning brep extension kernel.
/// Each one names `flowTessellateResolve` as its response action, so the SDK's request registry mints
/// the `req`, parks the continuation and dispatches the mesh JSON straight back into this app — the
/// hand-minted `RequestId` this used to write owned no registry slot, so every tessellation result
/// was discarded and the 3d preview could never paint.
pub fn preview_tessellate_invocations(session: &mut FlowEvalSession, eval_json: &str, fixture: &semio_framework_artifact_flow_flow::FlowFixture, cfg: &Generation3dConfig) -> Vec<semio_framework_plugin::ExtensionInvocation> {
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let tolerance_bits = tolerance.to_bits();
    let mut live = std::collections::HashSet::new();
    let eval = dsl::json::parse(eval_json).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
    for widget in &fixture.widgets {
        let id = crate::widget_id(widget).to_string();
        for item in preview_channel_items_for_widget(&eval, &id) {
            if !item.handle.is_empty() {
                live.insert(item.handle);
            }
        }
    }
    session.retain_preview_meshes(&live);
    let mut invocations = Vec::new();
    for handle in pending_preview_tessellate_handles(eval_json, fixture, session) {
        let node_hash = semio_framework_os_flow::preview_tessellate_node_hash(&handle, tolerance_bits);
        if session.note_pending_tessellate(node_hash, handle.clone()) {
            let mut request_object = dsl::json::Object::new();
            request_object.insert("handle", dsl::json::Value::String(handle));
            request_object.insert("tolerance", dsl::json::Value::from(tolerance));
            request_object.insert("nodeHash", dsl::json::Value::from(node_hash));
            request_object.insert("budget", dsl::json::Value::from(u64::from(PREVIEW_TESSELLATE_STEP_BUDGET)));
            request_object.insert("chunk", dsl::json::Value::from(u64::from(session.next_tessellate_chunk(node_hash))));
            invocations.push(semio_framework_plugin::ExtensionInvocation::new("brep", "tessellate", dsl::json::to_string(&dsl::json::Value::Object(request_object)), "flowTessellateResolve"));
        }
    }
    invocations
}

/// 👁️ One preview render's world-3d payload: the handle-deduplicated mesh table, the per-channel
/// instance table, and the instance ids the live `graph` marks resolved to — so the scene's
/// `selection_json` paints exactly the same hover/selection the instances themselves carry.
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewPayload {
    pub meshes_json: String,
    pub instances_json: String,
    pub selected_ids: Vec<String>,
    pub hovered_id: Option<String>,
}

/// 👁️ An empty payload is the empty JSON ARRAY, not the empty string — every consumer feeds these
/// straight into a `World3dScene` and compares them against `"[]"`.
impl Default for PreviewPayload {
    fn default() -> Self {
        Self { meshes_json: "[]".into(), instances_json: "[]".into(), selected_ids: Vec::new(), hovered_id: None }
    }
}

/// 🧊️ The interaction-free, session-free entry point: the mesh-export bridge and the schema tests
/// evaluate geometry without any live window, so they carry no marks and no tessellation cache.
pub fn preview_payload_from_eval(eval_json: &str, fixture: &semio_framework_artifact_flow_flow::FlowFixture, cfg: &Generation3dConfig) -> (String, String) {
    let payload = preview_payload(eval_json, fixture, cfg, None, &PreviewInteractionMarks::default());
    (payload.meshes_json, payload.instances_json)
}

/// 👁️ One preview instance per geometry-bearing value per OUTPUT CHANNEL — the whole point of the
/// channel-qualified ids: a widget with several outputs previews every one of them, not just the
/// first handle its evaluation happened to expose.
/// 🧮️ `[f64; 3]` -> a `pack::json` array, for the position/scale fields below.
fn vec3_json(v: [f64; 3]) -> dsl::json::Value {
    dsl::json::Value::Array(v.into_iter().map(dsl::json::Value::from).collect())
}

pub fn preview_payload(eval_json: &str, fixture: &semio_framework_artifact_flow_flow::FlowFixture, cfg: &Generation3dConfig, session: Option<&FlowEvalSession>, marks: &PreviewInteractionMarks) -> PreviewPayload {
    if eval_json.is_empty() {
        return PreviewPayload::default();
    }
    if let Ok(parsed) = dsl::json::parse(eval_json) {
        if parsed.get("error").and_then(dsl::json::Value::as_str).is_some() {
            return PreviewPayload::default();
        }
    }
    let eval = dsl::json::parse(eval_json).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let show_mode = if cfg.show_mode.is_empty() { "solid" } else { cfg.show_mode.as_str() };
    let mut meshes: Vec<dsl::json::Value> = Vec::new();
    let mut instances: Vec<dsl::json::Value> = Vec::new();
    // 🔁️ Dedup key is the brep HANDLE, not the widget/channel that emitted it: two channels (even
    // on different widgets) that resolve to the same handle share one tessellated mesh entry and
    // still each get their own instance — see the mesh-id lookup below.
    let mut mesh_id_by_handle: HashMap<String, String> = HashMap::new();
    let mut selected_ids: Vec<String> = Vec::new();
    let mut hovered_id: Option<String> = None;
    for widget in &fixture.widgets {
        let id = crate::widget_id(widget).to_string();
        let preview = widget_previews(widget);
        if !preview {
            continue;
        }
        let items = preview_channel_items_for_widget(&eval, &id);
        if items.is_empty() {
            continue;
        }
        for item in &items {
            let PreviewChannelItem { channel, index, handle, inline } = item;
            let instance_id = format!("{id}@{channel}#{index}");
            let own_mesh_id = format!("eval-{id}@{channel}#{index}");
            let mesh_id = if handle.is_empty() { own_mesh_id } else { mesh_id_by_handle.get(handle).cloned().unwrap_or(own_mesh_id) };
            if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                let data = match inline {
                    Some(PreviewInlineGeometry::Point { x, y, z }) => Some(point_marker_mesh(*x, *y, *z)),
                    Some(PreviewInlineGeometry::Vector { x, y, z }) => Some(vector_marker_mesh(*x, *y, *z)),
                    None => mesh_data_for_preview_handle(handle, tolerance, session),
                };
                if let Some(data) = data {
                    let data = apply_show_mode_mesh(data, show_mode);
                    if mesh_has_preview_geometry(&data) {
                        let mut mesh_object = dsl::json::Object::new();
                        mesh_object.insert("id", dsl::json::Value::String(mesh_id.clone()));
                        mesh_object.insert("data", dsl::json::Value::from(data));
                        meshes.push(dsl::json::Value::Object(mesh_object));
                        if !handle.is_empty() {
                            mesh_id_by_handle.insert(handle.clone(), mesh_id.clone());
                        }
                    }
                }
            }
            if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                let selected = marks.selects(&id, channel, *index);
                let hovered = marks.hovers(&id, channel, *index);
                if selected {
                    selected_ids.push(instance_id.clone());
                }
                if hovered && hovered_id.is_none() {
                    hovered_id = Some(instance_id.clone());
                }
                let mut instance_object = dsl::json::Object::new();
                instance_object.insert("id", dsl::json::Value::String(instance_id));
                instance_object.insert("meshId", dsl::json::Value::String(mesh_id));
                instance_object.insert("position", vec3_json([0.0, 0.0, 0.0]));
                instance_object.insert("rotation", dsl::json::Value::Array(vec![dsl::json::Value::from(0.0), dsl::json::Value::from(0.0), dsl::json::Value::from(0.0), dsl::json::Value::from(1.0)]));
                instance_object.insert("scale", vec3_json([1.0, 1.0, 1.0]));
                instance_object.insert("label", dsl::json::Value::String(format!("{id}@{channel}")));
                instance_object.insert("interactionId", dsl::json::Value::String(format!("{id}@{channel}")));
                instance_object.insert("selected", dsl::json::Value::Bool(selected));
                instance_object.insert("hovered", dsl::json::Value::Bool(hovered));
                instances.push(dsl::json::Value::Object(instance_object));
            }
        }
    }
    PreviewPayload { meshes_json: dsl::json::to_string(&dsl::json::Value::Array(meshes)), instances_json: dsl::json::to_string(&dsl::json::Value::Array(instances)), selected_ids, hovered_id }
}
//#endregion 🔖️PreviewPipeline

//#region 🔖️MeshBridge
/// 🧊️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// the DWG-import mesh bridge: `export_mesh_from_document` builds a default [`Generation3dConfig`] to
/// run the same preview pipeline the app's own render path uses, which is why this cluster is app
/// behavior rather than artifact-schema-pure compute.
pub fn merge_preview_meshes(meshes: &[semio_framework_plugin::MeshData]) -> semio_framework_plugin::MeshData {
    let mut merged = semio_framework_plugin::MeshData::default();
    for mesh in meshes {
        let vertex_offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(&mesh.positions);
        merged.normals.extend(&mesh.normals);
        merged.colors.extend(&mesh.colors);
        merged.indices.extend(mesh.indices.iter().map(|index| index + vertex_offset));
        merged.edge_positions.extend(&mesh.edge_positions);
        if !mesh.edge_ids.is_empty() {
            let edge_base = merged.edge_ids.len() as u32;
            merged.edge_ids.extend(mesh.edge_ids.iter().map(|id| id + edge_base));
        }
    }
    merged
}

pub fn export_mesh_from_document(projection: &Generation3dSnapshot) -> semio_framework_plugin::MeshData {
    let config = Generation3dConfig::default();
    let eval_json = crate::standards::v1::subsets::any::schema::with_host(&projection.fixture, |host| host.evaluate().unwrap_or_default());
    let (meshes_json, _) = preview_payload_from_eval(&eval_json, &projection.fixture, &config);
    // 🌉️ `MeshData` has its own first-party `FromValue` (see `mesh_data_for_preview_handle`'s
    // note) — decode the per-mesh `data` field straight through the `pack::json`/`DslValue` bridge.
    let meshes: Vec<semio_framework_plugin::MeshData> = dsl::json::parse(&meshes_json)
        .ok()
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|entry| entry.get("data").cloned())
        .filter_map(|data| dsl::FromValue::from_value(dsl::json::to_dsl_value(&data)).ok())
        .collect();
    merge_preview_meshes(&meshes)
}

pub fn generation3d_mesh_from_document(doc: &dsl::DslValue) -> Result<semio_framework_plugin::MeshData, String> {
    let projection = <Generation3dSnapshot as protocol::FromValue>::from_value(doc.clone()).map_err(|err| err.to_string())?;
    let mesh = export_mesh_from_document(&projection);
    projection.retire_cold();
    Ok(mesh)
}

pub fn generation3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<protocol::json::Value, String> {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let value = protocol::json::from_dsl_value(&protocol::ToValue::to_value(&snapshot));
    snapshot.retire_cold();
    Ok(value)
}

//#endregion 🔖️MeshBridge

//#region 🧪️TestSupport
/// 🧵️ `tessellate_geometry` (flow core brep geometry session) (and the flow-eval neuron kernel cache it sits behind)
/// is a process-wide cache shared by every test in this ONE merged crate — before the crate
/// consolidation, the artifact/app constitutional crates each ran in their own `cargo test` process, so
/// a `TEST_SERIAL` local to one of them never had to coordinate with the other's. Now that every
/// taxonomy node's tests share one test binary, ANY test that evaluates a flow fixture and/or tessellates
/// BRep geometry (directly here, or indirectly via the app's preview-window `render()`) must acquire
/// THIS single crate-wide lock — see `crate::editor::generation3d::modes::edit::windows::preview`'s test
/// for the app-side half of this. Rehomed from the deleted `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[cfg(test)]
#[path = "🧪️tests/🔬️test-support/🦀️.rs"]
pub(crate) mod test_support;
//#endregion 🧪️TestSupport

//#region 🧪️Testkit
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FoldContract
/// 🧺️ The store's batched fold envelope, as this app's two durable lanes declare it — its own module
/// because the law is about the PUBLICATION contract, not about any one command
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(test)]
#[path = "🧪️tests/🔬️fold-contract/🦀️.rs"]
mod fold_contract;
//#endregion 🧪️FoldContract
