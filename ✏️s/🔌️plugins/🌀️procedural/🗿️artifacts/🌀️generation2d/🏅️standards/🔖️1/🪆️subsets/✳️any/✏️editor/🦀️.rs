//! 🎲️ Generation2d editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command
//! enum and the manifest stitch (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1).
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's own inferences. This file is a routing table: `handle`
//! → `Generation2dCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls
//! one passthrough per node.

use crate::editor::generation2d::commands::{
    add_generation, add_widget, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, connect_media_ports, enter_generate, flow_eval_resolve, flow_eval_tick, generation, move_media_node, node_graph_edit, node_graph_viewport, remove_generation,
    remove_widget, rename_generation, reorganize, select_generation, set_contributions, set_eval_outputs, set_show_mode, update_generation_values,
};
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::editor::generation2d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use crate::editor::generation2d::modes::generate::windows::{form, generations, preview as generate_preview};
use crate::editor::generation2d::modes::{edit, generate};
use crate::editor::generation2d::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::generation2d::terminology::{generation2d_labels, Generation2dLabels};
use crate::editor::generation2d::transient::{Generation2dTransient, Generation2dTransientMutation, SetGenerationPreview};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::{artifact_kind, Generation2dSnapshot, GENERATION2D_DIALECT, GENERATION_2D_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_os_flow::{FlowEvalSession, FlowHost};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::{
    app::InteractionView, ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane,
    ArtifactReservedJob, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion,
    ArtifactView, CommandDefinition, ConfigView, Dialect, DomainTopology, DraftView, Editor, EditorApp, Effect, Emit, EphemeralEmit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef,
    InteractionTopology, Label, LocalizedLabel, MediaClass, MediaForm, MediaType, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};
use store::EngineHandles;

//#region 🔖️Constants
/// 🏷️ Plain string tag (NOT a trait const — `ArtifactEditor::DIALECT`+`ROLE` derive the real surface
/// id now, contract §2.1) reused wherever a window/panel needs a stable controller/action-factory id.
pub const GENERATION2D_PLAY_APP_ID: &str = "procedural2d-play";

fn close_flow_session(session: &mut FlowEvalSession) {
    session.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(usize::MAX, usize::MAX);
    }
}

fn categorized_action(id: &str, label: LocalizedLabel, kind: ActionKind, category: &str) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, label, kind).with_category(category)
}

/// 🧵️ Classifies the internal flow continuation as backed by its bounded first-step factory.
fn migrated_command(mut definition: CommandDefinition) -> CommandDefinition {
    definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    definition
}

/// 🧠️ The ONE `FlowEvalSession` this app instance retains across turns. Every entry point used to
/// build a throwaway `FlowEvalSession::new()`, so `flowEvalResolve`'s `seed_node_cache` would have
/// landed on a cache destroyed before the next tick could read it. Mirrors
/// `Generation3dInstanceOperationOwner` and the framework's reference `FlowInstanceOperationOwner`.
struct Generation2dInstanceOperationOwner {
    eval_session: Option<FlowEvalSession>,
    closing: bool,
}

impl Generation2dInstanceOperationOwner {
    fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), closing: false }
    }

    fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("generation2d-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("generation2d-eval-session-owner-missing"))
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for Generation2dInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    /// 🧹️ A LIVE session owns nothing retirable — `FlowEvalSession::close_step` answers `Blocked`
    /// until `begin_close`, and reporting that from the live maintenance ladder spent the runtime's
    /// zero-progress credit every idle turn (26/09/09/PROCEDURAL-3D-END-TO-END, close-ladder lane).
    fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        if !self.closing {
            return Ok(semio_framework_plugin::PluginCloseStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(session) = self.eval_session.as_mut() else { return Ok(semio_framework_plugin::PluginCloseStep::Complete) };
        let step = session.close_step(maximum_items, maximum_bytes);
        if session.terminal_is_empty() {
            self.eval_session = None;
        }
        Ok(match step {
            semio_framework_job::InteractiveJobCloseStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "Generation2d evaluation session awaits its exact close grant" },
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

//#endregion 🔖️Constants

//#region 🔖️ArtifactIo
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `create_generation2d_app`'s
/// `.artifact_kind(...)` document schema/media type verbatim, plus two workflow ports: `params:in`
/// (generic Data×Value parametric input) and `drawing:out` (TwoD×Vector, tagged with draw's already-
/// registered `2d.drawing` kind id).
pub async fn generation2d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        "generation.2d",
        MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "2d.generation".into(), name: "2D Generation".into(), dimension: "2d".into(), component_kind: "generation2d".into() },
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
            id: "drawing:out".into(),
            label: "Drawing".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            kind_id: Some("2d.drawing".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        },
    ])
    .await
}
//#endregion 🔖️ArtifactIo

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Generation2dPlayApp::Command` — the SOLE dispatch surface for generation2d's own behavior.
    /// Each row states BOTH the manifest action id (`command_id()`) and the `dsl` wire keyword
    /// order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum Generation2dCommand for Generation2dSnapshot, Generation2dMutation, Generation2dConfig, Generation2dConfigMutation, ctx = FlowEvalSession {
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "moveMediaNode" as "move-media-node" => move_media_node::MoveMediaNode,
        "addWidget" as "add-widget" => add_widget::AddWidget,
        "removeWidget" as "remove-widget" => remove_widget::RemoveWidget,
        "connectMediaPorts" as "connect-media-ports" => connect_media_ports::ConnectMediaPorts,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "addGeneration" as "add-generation" => add_generation::AddGeneration,
        "removeGeneration" as "remove-generation" => remove_generation::RemoveGeneration,
        "renameGeneration" as "rename-generation" => rename_generation::RenameGeneration,
        "updateGenerationValues" as "update-generation-values" => update_generation_values::UpdateGenerationValues,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setShowMode" as "set-show-mode" => set_show_mode::SetShowMode,
        "generate" as "generate" => enter_generate::Generate,
        "setEvalOutputs" as "set-eval-outputs" => set_eval_outputs::SetEvalOutputs,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasWheel" as "canvas-wheel" => canvas_wheel::CanvasWheel,
        "selectGeneration" as "select-generation" => select_generation::SelectGeneration,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "setContributions" as "set-contributions" => set_contributions::SetContributions}
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const GENERATION2D_BOUNDED_TOOL_IDS: &[&str] = &[
    "nodeGraphViewport",
    "setShowMode",
    "generate",
    "canvasPointerDown",
    "canvasPointerMove",
    "canvasPointerUp",
    "canvasWheel",
    "addGeneration",
    "removeGeneration",
    "renameGeneration",
    "updateGenerationValues",
    "selectGeneration",
    "flowEvalTick",
    "flowEvalResolve",
    "nodeGraphEdit",
    "moveMediaNode",
    "addWidget",
    "removeWidget",
    "connectMediaPorts",
    "reorganize",
    "setEvalOutputs",
];
const GENERATION2D_PREVIEW_TOOL_IDS: &[&str] = &["addGeneration", "removeGeneration", "renameGeneration", "updateGenerationValues", "selectGeneration"];
const GENERATION2D_RETAINED_PAYLOAD_SCHEMA: &str = "generation.2d.tool-command.v1";
const GENERATION2D_RETAINED_RAW_BYTES: usize = 8_192;
/// 🧮️ The ONE declared quantity of every generation2d retained route: at most 32 point-invertible
/// durable items per gesture. The preflight ceiling below, every `extent` a work answers, both
/// durable lanes' one-item store footprint and the bounded proof's work units are all read off it —
/// never spelled again. The preview ladder walks one item per fixture widget plus its own two
/// framing steps, which is why the declaration is 32 items and not one
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const GENERATION2D_RETAINED_CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(32);
const GENERATION2D_RETAINED_WORK_ITEMS: usize = GENERATION2D_RETAINED_CAPACITY.work_items();
/// 🧾️ Decoded JSON items one retained command's args may carry — a WIRE bound, deliberately NOT the
/// work capacity.
const GENERATION2D_RETAINED_DECODED_ITEMS: usize = 64;

/// 🧾️ The ONE execution contract all 21 routes publish — the factory's `execution_contract()` and
/// every row of `bounded_first_step_tool_proofs!` alike.
fn generation2d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION2D_RETAINED_RAW_BYTES, GENERATION2D_RETAINED_DECODED_ITEMS, GENERATION2D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

fn generation2d_bounded_extent(_command: &Generation2dCommand, _snapshot: &Generation2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    GENERATION2D_RETAINED_CAPACITY.rows_for_items(1)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn generation2d_retained_reduce(
    command: &Generation2dCommand,
    snapshot: &Generation2dSnapshot,
    config: &Generation2dConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation2dPlayApp>>>,
    operation: &AppOperationContext,
    session: &mut FlowEvalSession,
) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation, NoDraftMutation>, Fault> {
    if !GENERATION2D_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("generation2d-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    match command {
        Generation2dCommand::NodeGraphEdit(payload) => node_graph_edit::apply_selected(payload, &doc, &interaction.selection.get("graph").map(|selection| selection.ids.clone()).unwrap_or_default()),
        _ => command.dispatch(&doc, &cfg, session),
    }
}

/// 🧵️ The retained-session twin of `BoundedArtifactCommandWork` — see
/// `Generation2dInstanceOperationOwner` for why a per-dispatch session cannot carry a continuation.
struct Generation2dSessionCommandWork {
    tool_id: &'static str,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    consumed: bool,
}

impl Generation2dSessionCommandWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, consumed: false }
    }
}

impl ArtifactCommandWork<EditorApp<Generation2dPlayApp>> for Generation2dSessionCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Generation2dCommand,
        snapshot: &Generation2dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation2dPlayApp>>>,
    ) -> Option<usize> {
        generation2d_bounded_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation2dPlayApp>>, Fault> {
        if self.consumed {
            return Err(Fault::from("generation2d-session-command-work-repeated"));
        }
        self.consumed = true;
        let emit = self.instance_owner.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| generation2d_retained_reduce(input.command, input.snapshot, input.config, input.history, input.interaction, input.hover, input.context, input.operation, session))?
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
}

struct Generation2dPreviewCommandWork {
    tool_id: &'static str,
    initialized: bool,
    emit: Option<Emit<Generation2dMutation, Generation2dConfigMutation, NoDraftMutation>>,
    host: Option<FlowHost>,
    /// 🧹️ The host's explicit retirement ladder — a bare `Option<FlowHost>::take()`-and-drop panics
    /// on the cloned fixture's `OrderedMap<WidgetLayout>` root, so it is drained under the grant.
    host_retirement: Option<semio_framework_os_flow::FlowHostRetirement>,
    session: Option<FlowEvalSession>,
    closing: bool,
}

impl Generation2dPreviewCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, initialized: false, emit: None, host: None, host_retirement: None, session: None, closing: false }
    }
}

impl ArtifactCommandWork<EditorApp<Generation2dPlayApp>> for Generation2dPreviewCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        _command: &Generation2dCommand,
        snapshot: &Generation2dSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation2dPlayApp>>>,
    ) -> Option<usize> {
        GENERATION2D_RETAINED_CAPACITY.rows_for_items(snapshot.fixture.widgets.len().checked_add(2)?)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation2dPlayApp>>, Fault> {
        if !self.initialized {
            let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
            let cfg = ConfigView { snapshot: input.config, window: None };
            let result = generation::prepare_command(input.command, &doc, &cfg).ok_or_else(|| Fault::from("generation2d-preview-command-route-rejected"))?;
            self.initialized = true;
            if !result.publishes_preview {
                return Ok(ArtifactCommandWorkStep::Complete(result.emit));
            }
            let Some(values) = result.preview_values else {
                return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: result.emit, ephemeral: EphemeralEmit { transient: vec![SetGenerationPreview { preview_text: None }.into()], ..Default::default() } });
            };
            let host = crate::standards::v1::subsets::any::schema::generation_preview_host(&input.snapshot.fixture, &values);
            let mut session = FlowEvalSession::new();
            session.sync(&host);
            self.emit = Some(result.emit);
            self.host = Some(host);
            self.session = Some(session);
            return Ok(ArtifactCommandWorkStep::Progress { stage: "generation2d-preview-evaluation", preview: br#"{"en":"Evaluating preview","de":"Vorschau wird ausgewertet"}"# });
        }
        let host = self.host.as_mut().ok_or_else(|| Fault::from("generation2d-preview-host-owner-missing"))?;
        let session = self.session.as_mut().ok_or_else(|| Fault::from("generation2d-preview-session-owner-missing"))?;
        if session.tick(host) {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "generation2d-preview-evaluation", preview: br#"{"en":"Evaluating preview","de":"Vorschau wird ausgewertet"}"# });
        }
        let preview_text = session.eval_json().to_string();
        let emit = self.emit.take().ok_or_else(|| Fault::from("generation2d-preview-emit-owner-missing"))?;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { transient: vec![SetGenerationPreview { preview_text: Some(preview_text) }.into()], ..Default::default() } })
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(session) = self.session.as_mut() {
            session.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        if let Some(session) = self.session.as_mut() {
            let step = session.close_step(maximum_items, maximum_bytes);
            if matches!(step, InteractiveJobCloseStep::Complete) && session.terminal_is_empty() {
                self.session.take();
                if let Some(host) = self.host.take() {
                    self.host_retirement = Some(semio_framework_os_flow::FlowHostRetirement::new(host));
                }
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            return step;
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
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.session.is_none() && self.host.is_none() && self.host_retirement.is_none()
    }
}

struct Generation2dBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation2dBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION2D_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation2dBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation2dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation2d_bounded_contract()
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
        if input.declared_bytes() > GENERATION2D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation2d retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation2dBoundedCommandJobFactory {
    type Owner = EditorApp<Generation2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION2D_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_2D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setShowMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "generate", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasWheel", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "removeGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "renameGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "updateGenerationValues", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "selectGeneration", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "flowEvalResolve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "moveMediaNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addWidget", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeWidget", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "connectMediaPorts", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setEvalOutputs", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

//#endregion 🧵️RetainedCommands

//#region 🧩️ContributionsRoute
/// 🧩️ The host→guest contributions route — the exact twin of generation3d's
/// (`…/🧊️generation3d/…/✏️editor/🦀️.rs`), and for the same reason: the shell's whole
/// `flow.extension` closure is what makes this app's operators resolvable, and it is far too large
/// for the 8 KiB gesture quota the other 21 routes share.
const GENERATION2D_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];
const GENERATION2D_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "generation.2d.contributions-command.v1";
/// 📐️ The REAL wire ceiling of one contributions page: the framework's own public-invocation string
/// bound — which no tool contract can widen, because `validate_public_json_envelope` runs first —
/// at its worst-case escaped width, plus the addressed envelope.
const GENERATION2D_CONTRIBUTIONS_ENVELOPE_BYTES: usize = 4_096;
const GENERATION2D_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::PUBLIC_INVOCATION_STRING_BYTES * semio_framework::PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR + GENERATION2D_CONTRIBUTIONS_ENVELOPE_BYTES;

fn generation2d_contributions_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION2D_CONTRIBUTIONS_RAW_BYTES, GENERATION2D_RETAINED_DECODED_ITEMS, GENERATION2D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page against the app instance's retained session.
struct Generation2dContributionsWork {
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    consumed: bool,
}

impl ArtifactCommandWork<EditorApp<Generation2dPlayApp>> for Generation2dContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
    }

    fn extent(
        &self,
        command: &Generation2dCommand,
        snapshot: &Generation2dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation2dPlayApp>>>,
    ) -> Option<usize> {
        generation2d_bounded_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation2dPlayApp>>, Fault> {
        if self.consumed {
            return Err(Fault::from("generation2d-contributions-work-repeated"));
        }
        self.consumed = true;
        let Generation2dCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("generation2d-contributions-route-rejected"));
        };
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: None };
        let emit = self.instance_owner.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| owner.with_session(|session| set_contributions::handle(payload, &doc, &cfg, session))?)?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
}

struct Generation2dContributionsJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation2dContributionsJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION2D_CONTRIBUTIONS_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation2dContributionsJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation2dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION2D_CONTRIBUTIONS_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation2d_contributions_contract()
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
        if input.declared_bytes() > GENERATION2D_CONTRIBUTIONS_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation2d contributions command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation2dContributionsJobFactory {
    type Owner = EditorApp<Generation2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION2D_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_2D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

struct Generation2dBoundedCommandJobFactoryProofs;

impl Generation2dBoundedCommandJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation2d@1/*#editor",
        document_schema: "generation.2d",
        factory: "Generation2dBoundedCommandJobFactory",
        factory_type: Generation2dBoundedCommandJobFactory,
        contract: generation2d_bounded_contract(),
        tools: [
            "nodeGraphViewport",
            "setShowMode",
            "generate",
            "canvasPointerDown",
            "canvasPointerMove",
            "canvasPointerUp",
            "canvasWheel",
            "addGeneration",
            "removeGeneration",
            "renameGeneration",
            "updateGenerationValues",
            "selectGeneration",
            "flowEvalTick",
            "flowEvalResolve",
            "nodeGraphEdit",
            "moveMediaNode",
            "addWidget",
            "removeWidget",
            "connectMediaPorts",
            "reorganize",
            "setEvalOutputs",
        ]
    }
}

struct Generation2dContributionsJobFactoryProofs;

impl Generation2dContributionsJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation2d@1/*#editor",
        document_schema: "generation.2d",
        factory: "Generation2dContributionsJobFactory",
        factory_type: Generation2dContributionsJobFactory,
        contract: generation2d_contributions_contract(),
        tools: ["setContributions"]
    }
}
//#endregion 🧩️ContributionsRoute

//#region 📬️ArtifactStorePreparation
/// 🧬️ Builds one `protocol::Edit<M>` for either lane's `advance()` — the two lanes differ only in `M`
/// and their id prefix, so this one generic helper replaces two copies of the same literal.
fn generation2d_next_edit<M>(prefix: &str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
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

const GENERATION2D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 262_144;

fn generation2d_artifact_mutation_retained_bytes(mutation: &Generation2dMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation2d-artifact-mutation-encode-failed".to_string())
}

/// 🧺️ The ONE fold-contract footprint BOTH durable lanes declare, straight off the route capacity:
/// one forward row plus the one row `Mutation::inverse` yields. `ArtifactStore::fold_batch_item`
/// sizes the staged inverse vector as `footprint.work_items - admitted_items`
/// (`🧰️framework/…/🏪️store/🦀️.rs`), so a `work_items: 1` footprint leaves ZERO inverse capacity and
/// refuses every undoable mutation. Every `Generation2dMutation` inverse is a `Vec` of length 0 or 1
/// (`🧬️mutations/*/↩️inverse/🦀️.rs`), so this is exact, not a margin.
fn generation2d_one_item_footprint(retained_bytes: usize) -> store::ArtifactStoreOneItemFootprint {
    GENERATION2D_RETAINED_CAPACITY.one_item_footprint(retained_bytes)
}

fn admit_generation2d_artifact_mutation(mutation: &Generation2dMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation2d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION2D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("generation2d-artifact-mutation-envelope".into());
    }
    Ok(generation2d_one_item_footprint(retained_bytes))
}

/// 🧬️ Raises the mutation's delta, applies it and CLOSES the delta — a `Generation2dDiff` owns the
/// projections it displaces, so the intermediate delta is retired rather than dropped.
fn prepare_generation2d_artifact(base: &Generation2dSnapshot, mutation: Generation2dMutation) -> Result<(Generation2dSnapshot, Vec<Generation2dMutation>, Generation2dMutation), String> {
    admit_generation2d_artifact_mutation(&mutation)?;
    let inverse = protocol::Mutation::inverse(&mutation, base);
    let diff = protocol::Mutation::diff(&mutation, base).into_parts().0;
    let applied = protocol::MutationDiff::apply(&diff, base);
    diff.retire_cold();
    let post = applied.map_err(|_| "generation2d-artifact-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

struct Generation2dArtifactStorePreparationFactory;

struct Generation2dArtifactStorePreparation {
    base: Option<store::SnapshotRead<Generation2dSnapshot>>,
    mutation: Option<Generation2dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation2dSnapshot, Generation2dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Generation2dSnapshot, Generation2dMutation> for Generation2dArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &Generation2dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("generation2d-artifact-lane-or-description-envelope".into());
        }
        admit_generation2d_artifact_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation2dSnapshot, Generation2dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation2dSnapshot, Generation2dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation2dSnapshot, Generation2dMutation>> {
        let retained_bytes = generation2d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION2D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION2D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation2dArtifactStorePreparation {
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

impl store::ArtifactStoreOneItemPreparation<Generation2dSnapshot, Generation2dMutation> for Generation2dArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation2d-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "generation2d-artifact-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_generation2d_artifact(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "generation2d-artifact-authority-missing".to_string())?;
        let edit = generation2d_next_edit("generation2d-artifact-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation2dSnapshot, Generation2dMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation2dSnapshot, Generation2dMutation>> {
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
        if self.prepared.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if let Some(mutation) = self.mutation.take() {
            crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_retire_mutation_cold(mutation);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("generation2d-artifact-base-retirement-rejected".into());
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
const GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES: usize = 128;
const GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES: usize = 4_096;

//#region 🎟️Admission
fn generation2d_config_text_bytes(config: &Generation2dConfig) -> usize {
    [config.show_mode.len(), config.selected_generation_id.as_ref().map_or(0, String::len)].into_iter().fold(0usize, usize::saturating_add)
}

fn generation2d_config_publication_bytes(mutation: &Generation2dConfigMutation) -> Result<usize, String> {
    let bytes = match mutation {
        Generation2dConfigMutation::SetCamera { .. } => 0,
        Generation2dConfigMutation::SetShowMode { value } => value.len(),
        Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id } => selected_generation_id.as_ref().map_or(0, String::len),
        _ => return Err("generation2d-config-unsupported-mutation".into()),
    };
    if bytes > GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES {
        return Err("generation2d-config-text-envelope".into());
    }
    Ok(GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES)
}

struct Generation2dConfigPreparationFactory;

impl store::ArtifactStoreOneItemPreparationFactory<Generation2dConfig, Generation2dConfigMutation> for Generation2dConfigPreparationFactory {
    fn preflight(&self, mutation: &Generation2dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > 64) {
            return Err("generation2d-config-lane-or-description-envelope".into());
        }
        Ok(generation2d_one_item_footprint(generation2d_config_publication_bytes(mutation)?))
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation2dConfig, Generation2dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation2dConfig, Generation2dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation2dConfig, Generation2dConfigMutation>> {
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > 64
            || self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
            || generation2d_config_text_bytes(request.base.get()) > GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation2dConfigPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}
//#endregion 🎟️Admission

//#region 🧵️Preparation
struct Generation2dConfigPreparation {
    base: Option<store::SnapshotRead<Generation2dConfig>>,
    mutation: Option<Generation2dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation2dConfig, Generation2dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparation<Generation2dConfig, Generation2dConfigMutation> for Generation2dConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || grant.maximum_bytes < GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.checkpoint.cursor != 0 {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation2d-config-base-owner-missing".to_string())?;
        let mutation = self.mutation.as_ref().ok_or_else(|| "generation2d-config-mutation-owner-missing".to_string())?;
        let mut next = base.get().clone();
        let inverse = match mutation {
            Generation2dConfigMutation::SetCamera { camera } => {
                next.camera = camera.clone();
                Generation2dConfigMutation::SetCamera { camera: base.get().camera.clone() }
            }
            Generation2dConfigMutation::SetShowMode { value } => {
                next.show_mode = value.clone();
                Generation2dConfigMutation::SetShowMode { value: base.get().show_mode.clone() }
            }
            Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id } => {
                next.selected_generation_id.clone_from(selected_generation_id);
                Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: base.get().selected_generation_id.clone() }
            }
            _ => return Err("generation2d-config-unsupported-mutation".into()),
        };
        if generation2d_config_text_bytes(&next) > GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES {
            return Err("generation2d-config-post-text-envelope".into());
        }
        let authority = self.authority.as_ref().ok_or_else(|| "generation2d-config-authority-missing".to_string())?;
        let id = format!("generation2d-config-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(),
            actor: Some(authority.actor().to_string()),
            forwards: vec![mutation.clone()],
            inverse: vec![inverse],
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
            description: self.description.clone(),
            coalesce_key: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(next))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation2dConfig, Generation2dConfigMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation2dConfig, Generation2dConfigMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 || grant.maximum_bytes < GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("generation2d-config-base-retirement-rejected".into());
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
//#endregion 🧵️Preparation
//#region 🧪️PreparationLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️generation2d-config-preparation-laws/🦀️.rs"]
mod generation2d_config_preparation_laws;
//#endregion 🧪️PreparationLaws
//#endregion 📬️ConfigStorePreparation

//#region 🎞️ReservedImport
const GENERATION2D_IMPORT_TOOL_ID: &str = "import-media";
const GENERATION2D_IMPORT_PORT: &str = "params:in";

/// 🎞️ The ONE concrete resumable importer this app owns. The framework registers the reserved
/// `import-media` factory for every app (`register_framework_reserved_factories`) but never a
/// concrete job, so `VcsArtifactApp::import_media` fails closed with
/// `interactive-job.missing-reserved-builder` until the app hands one back here. Two bounded steps:
/// decode the `params:in` object into `replace-widget` rows, then publish them through the
/// completion authority (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
struct Generation2dImportJob {
    port: String,
    media_json: Option<String>,
    snapshot: Option<std::sync::Arc<Generation2dSnapshot>>,
    mutations: Vec<Generation2dMutation>,
    decoded: bool,
    completed: bool,
    closing: bool,
    completion: Option<ArtifactToolCompletion<EditorApp<Generation2dPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Generation2dPlayApp>>>,
}

fn generation2d_job_payload(cx: &mut StepContext<'_>, stream: JobPayloadStream, bytes: &[u8]) -> RetainedJobPayload {
    match cx.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            RetainedJobPayload::empty(stream)
        }
    }
}

fn generation2d_job_fault(cx: &mut StepContext<'_>, detail: &str) -> StepOutcome {
    let bytes = detail.as_bytes();
    let bounded = &bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)];
    StepOutcome::Fault(JobFault { detail: generation2d_job_payload(cx, JobPayloadStream::Fault, bounded) })
}

impl Generation2dImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Generation2dPlayApp>>, port: String, media: semio_framework_plugin::Media) -> Self {
        let media_json = match media.payload {
            semio_framework_plugin::MediaPayload::Structured { json, .. } => Some(json),
            semio_framework_plugin::MediaPayload::Binary { .. } => None,
        };
        Self { port, media_json, snapshot: Some(request.snapshot), mutations: Vec::new(), decoded: false, completed: false, closing: false, completion: Some(request.completion), pending_completion_rejection: None }
    }

    fn decode(&mut self, cx: &mut StepContext<'_>) -> Option<StepOutcome> {
        if self.port != GENERATION2D_IMPORT_PORT {
            return Some(generation2d_job_fault(cx, "generation2d import only implements params:in"));
        }
        let Some(media_json) = self.media_json.as_ref() else {
            return Some(generation2d_job_fault(cx, "generation2d params:in requires a structured payload"));
        };
        let Ok(parsed) = dsl::json::parse(media_json) else {
            return Some(generation2d_job_fault(cx, "generation2d params:in payload is not valid json"));
        };
        let Some(object) = parsed.as_object() else {
            return Some(generation2d_job_fault(cx, "generation2d params:in payload must be a JSON object"));
        };
        let Some(snapshot) = self.snapshot.as_ref() else {
            return Some(generation2d_job_fault(cx, "generation2d import lost its snapshot authority"));
        };
        let mut rows = Vec::new();
        for (widget_id_key, value) in object.iter() {
            let Some(number) = value.as_f64() else { continue };
            let Some(semio_framework_artifact_flow_flow::Widget::InputSlider { id, label, min, max, step, .. }) = snapshot.fixture.widgets.iter().find(|widget| crate::widget_id(widget) == widget_id_key) else { continue };
            rows.push(crate::standards::v1::subsets::any::schema::mutations::text::replace_widget(semio_framework_artifact_flow_flow::Widget::InputSlider {
                id: id.clone(),
                label: label.clone(),
                value: number,
                min: *min,
                max: *max,
                step: *step,
            }));
        }
        self.mutations = rows;
        self.decoded = true;
        None
    }
}

impl InteractiveJob for Generation2dImportJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return generation2d_job_fault(cx, "generation2d import completion remains rejected");
        }
        if !self.decoded {
            cx.set_stage("generation2d-import-decode");
            if let Some(outcome) = self.decode(cx) {
                return outcome;
            }
            cx.consume_fuel(1);
            return StepOutcome::CheckpointReady(Checkpoint { state: generation2d_job_payload(cx, JobPayloadStream::CheckpointState, &[1]), applied_progress: 1 });
        }
        cx.set_stage("generation2d-import-publish");
        if !self.completed {
            let mutations = std::mem::take(&mut self.mutations);
            let Some(completion) = self.completion.as_ref() else {
                return generation2d_job_fault(cx, "generation2d import lost its completion authority");
            };
            if !completion.has_mounted_consumer() {
                return generation2d_job_fault(cx, "generation2d import completion consumer is absent");
            }
            if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                return generation2d_job_fault(cx, &message);
            }
            self.completed = true;
        }
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        match ArtifactReservedJob::close_step(self, maximum_items, maximum_bytes) {
            Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
            Ok(semio_framework_plugin::PluginCloseStep::AwaitingInput { .. } | semio_framework_plugin::PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) if ArtifactReservedJob::terminal_is_empty(self) => semio_framework_job::InteractiveJobCloseStep::Complete,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) => semio_framework_job::InteractiveJobCloseStep::Blocked,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactReservedJob::terminal_is_empty(self)
    }
}

impl ArtifactReservedJob for Generation2dImportJob {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if maximum_items == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, _maximum_bytes) {
                    return Ok(step);
                }
            }
            self.pending_completion_rejection = None;
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(mutation) = self.mutations.pop() {
            crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_retire_mutation_cold(mutation);
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.mutations.capacity() > 0 {
            self.mutations = Vec::new();
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.media_json.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.port.is_empty() || self.port.capacity() > 0 {
            self.port = String::new();
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.snapshot.as_ref().is_some_and(|snapshot| std::sync::Arc::strong_count(snapshot) == 1) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "generation2d import snapshot has no mounted retained authority" });
        }
        if self.snapshot.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.completion.as_ref().is_some_and(|completion| !completion.has_mounted_consumer()) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "generation2d import completion has no mounted consumer authority" });
        }
        if self.completion.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_empty()
            && self.port.capacity() == 0
            && self.media_json.is_none()
            && self.snapshot.is_none()
            && self.mutations.is_empty()
            && self.mutations.capacity() == 0
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
    }
}
//#endregion 🎞️ReservedImport

//#region 🔖️Generation2dPlayApp
/// 🧪️ Unit struct apart from `eval_session`: every former runtime field lives in [`Generation2dConfig`],
/// written through [`Generation2dConfigMutation`]s. The eval session is the one piece of state that is
/// neither document nor view — it is threaded into every command handler as the `app_commands!`
/// dispatch context.
#[derive(Default)]
pub struct Generation2dPlayApp;

impl ArtifactEditor for Generation2dPlayApp {
    /// 🛍️ Publishes the whole registered flow operator catalogue once per app instance on the reserved
    /// `framework.section.catalogue` retained surface — never on the node-graph scene, whose fixed
    /// `UI_FIXED_BYTES` admission it exceeds threefold with the real `brep`/`math` sets installed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
    fn app_catalogue_json() -> String {
        semio_framework_os_flow::flow_app_catalogue_json()
    }

    type Snapshot = Generation2dSnapshot;
    type Mutation = Generation2dMutation;
    type Config = Generation2dConfig;
    type ConfigMutation = Generation2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::generation2d::presence::Generation2dPresence;
    type PresenceMutation = crate::editor::generation2d::presence::Generation2dPresenceMutation;
    type Transient = Generation2dTransient;
    type TransientMutation = Generation2dTransientMutation;

    type Command = Generation2dCommand;

    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_envelope_decode_owner_bundle())
    }

    /// 🧠️ One retained `FlowEvalSession` per app instance — see [`Generation2dInstanceOperationOwner`].
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Generation2dInstanceOperationOwner::new())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_document_store_owners())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_document_store_initialization_job(envelope, operation, generation))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Generation2d atomic publication authority is absent or stale"))
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
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default()).expect("default Generation2d presence is the exact empty terminal")))
    }

    const DIALECT: Dialect = GENERATION2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_2D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Generation2dArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Generation2dConfigPreparationFactory))
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    /// 🧾️ BOTH factories' proofs, in registration order — `validate_tool_job_rows` matches this
    /// catalogue against the registered factories exactly.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Generation2dBoundedCommandJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(Generation2dContributionsJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Generation2dBoundedCommandJobFactory::new(&controller))?;
        registry.register(Generation2dContributionsJobFactory::new(&controller))
    }

    /// 🎞️ `import-media` is the only reserved route generation2d owns. The framework registers the
    /// reserved factory but never a concrete importer, so every inbound `params:in` delivery is
    /// routed exclusively through this builder (`dispatch_import_media` →
    /// `build_artifact_reserved_media_job`); `copy`/`cut`/`paste` stay on the framework's own
    /// reserved factories.
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != GENERATION2D_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("generation2d import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("generation2d import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(Generation2dImportJob::new(request, port, media))))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GENERATION2D_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str()) && !GENERATION2D_CONTRIBUTIONS_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("generation2d-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let contributions = GENERATION2D_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id);
        let work: Box<dyn ArtifactCommandWork<EditorApp<Generation2dPlayApp>>> = if contributions {
            Box::new(Generation2dContributionsWork { instance_owner: request.instance_operation_owner, consumed: false })
        } else if GENERATION2D_PREVIEW_TOOL_IDS.contains(&tool_id) {
            Box::new(Generation2dPreviewCommandWork::new(tool_id))
        } else {
            Box::new(Generation2dSessionCommandWork::new(tool_id, request.instance_operation_owner))
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
            Generation2dCommand::command_id,
            if contributions { GENERATION2D_CONTRIBUTIONS_RAW_BYTES } else { GENERATION2D_RETAINED_RAW_BYTES },
            GENERATION2D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::generation2d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Generation2dSnapshot {
        crate::standards::v1::subsets::any::schema::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(semio_framework::io::resolve_ready(generation2d_io()))
    }

    fn command_id(command: &Generation2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Generation2dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(dsl::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        let u64_arg = |keys: &[&str]| -> Option<u64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|number| number as u64)))) };
        match action {
            "nodeGraphEdit" => Ok(Generation2dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: str_arg(&["operationsJson", "operations_json"]).or_else(|| args.get("operations").map(dsl::json::to_json_string)).unwrap_or_else(|| "[]".into()),
            })),
            "moveMediaNode" => Ok(Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: f64_arg(&["x"]).unwrap_or(0.0), y: f64_arg(&["y"]).unwrap_or(0.0) })),
            "addWidget" => Ok(Generation2dCommand::AddWidget(add_widget::AddWidget { kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()), neuron_kind: str_arg(&["neuronKind", "neuron_kind"]), x: f64_arg(&["x"]), y: f64_arg(&["y"]) })),
            "removeWidget" => Ok(Generation2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "connectMediaPorts" => Ok(Generation2dCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts {
                source_node_id: str_arg(&["sourceNodeId", "source_node_id"]).unwrap_or_default(),
                source_port_id: str_arg(&["sourcePortId", "source_port_id"]).unwrap_or_default(),
                target_node_id: str_arg(&["targetNodeId", "target_node_id"]).unwrap_or_default(),
                target_port_id: str_arg(&["targetPortId", "target_port_id"]).unwrap_or_default(),
            })),
            "reorganize" => Ok(Generation2dCommand::Reorganize(reorganize::Reorganize {})),
            "addGeneration" => Ok(Generation2dCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(Generation2dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(Generation2dCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name"]).unwrap_or_default() })),
            "updateGenerationValues" => {
                let value = args.get("value").map_or(dsl::DslValue::Null, |entry| entry.clone());
                Ok(Generation2dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value,
                }))
            }
            "nodeGraphViewport" => {
                let viewport_json = str_arg(&["viewportJson", "viewport_json"])
                    .or_else(|| args.get("camera").map(|value| if value.as_str().is_some() { value.as_str().unwrap_or("{}").to_string() } else { dsl::json::to_json_string(value) }))
                    .unwrap_or_else(|| "{}".into());
                Ok(Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json }))
            }
            "setShowMode" => Ok(Generation2dCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode"]).unwrap_or_default() })),
            "generate" => Ok(Generation2dCommand::Generate(enter_generate::Generate {})),
            "setEvalOutputs" => Ok(Generation2dCommand::SetEvalOutputs(set_eval_outputs::SetEvalOutputs { outputs_json: str_arg(&["outputsJson", "outputs_json", "evalJson"]).unwrap_or_else(|| "{}".into()) })),
            "canvasPointerDown" => Ok(Generation2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {})),
            "canvasPointerMove" => Ok(Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {})),
            "canvasPointerUp" => Ok(Generation2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {})),
            "canvasWheel" => Ok(Generation2dCommand::CanvasWheel(canvas_wheel::CanvasWheel {})),
            "selectGeneration" => Ok(Generation2dCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]) })),
            "flowEvalTick" => Ok(Generation2dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {})),
            "flowEvalResolve" => Ok(Generation2dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve {
                node_hash: u64_arg(&["nodeHash", "node_hash"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
            })),
            "setContributions" => Ok(Generation2dCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json"]).unwrap_or_default(),
                page: u64_arg(&["page"]).unwrap_or_default(),
                page_count: u64_arg(&["pageCount", "page_count"]).unwrap_or(1),
            })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    /// 🕹️ `nodeGraphEdit` reads the `graph` interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg, ctx)`
    /// signature is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &Generation2dCommand,
        doc: &ArtifactView<'_, Generation2dSnapshot>,
        cfg: &ConfigView<'_, Generation2dConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation, Self::DraftMutation>, Fault> {
        let mut session = FlowEvalSession::new();
        let result = match command {
            Generation2dCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, interaction, &mut session),
            _ => command.dispatch(doc, cfg, &mut session),
        };
        close_flow_session(&mut session);
        result
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every top-level widget is a "node" (root unless
    /// nested in a `Widget::Cluster`'s own `tree.neurons`, where each nested `Neuron` becomes a "node"
    /// parented to its owning cluster's widget id — the DAG-parent-links transitive-hover source: hovering
    /// a Cluster's own tree item transitively covers every widget nested inside it). Synapses become
    /// "edge" targets, parented to nothing (edges are leaves, not containers).
    fn interaction_topology(doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>) -> InteractionTopology {
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
        for widget in &fixture.widgets {
            let id = crate::widget_id(widget).to_string();
            ordered.push(TopologyNode { id: id.clone(), granularity: "node".into(), parent: None });
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

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
    /// covers every mutation path (edits, undo/redo, remote operations) in one place instead of each
    /// action re-checking.
    ///
    /// 🪟️ Unlike generation3d's, this tick is a `HostOnly` route: it publishes into no window
    /// transient, declares no `retained_window_transient_target`, and therefore carries no window id
    /// — but it is still pointless before the first `Event::SurfaceVisible`, when `view` is `None`
    /// and nothing is mounted to render what it computes, so the chain waits instead of spinning
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn pending_effects(doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        if view.is_none_or(|view| view.window_instances.is_empty()) {
            return Vec::new();
        }
        let mut session = FlowEvalSession::new();
        let pending = crate::standards::v1::subsets::any::schema::with_host_session(&doc.snapshot.fixture, &mut session, |host, session| session.sync(host));
        let effects = if pending { vec![flow_eval_tick::rearm(101)] } else { Vec::new() };
        close_flow_session(&mut session);
        effects
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let mut session = FlowEvalSession::new();
        let rendered = generation2d_render_body(body_key, doc, cfg, view_state, &session);
        close_flow_session(&mut session);
        rendered
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Generation2dSnapshot>,
        cfg: &ConfigView<'_, Generation2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        if body_key == generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW {
            let labels = generation2d_labels(view_state);
            let node = generate_preview::render(cfg.snapshot, transient.snapshot.generation_preview_text.as_deref(), labels)?;
            return Ok(semio_framework_plugin::built_to_component_tree(node));
        }
        owner
            .with_mut::<Generation2dInstanceOperationOwner, _>(|owner| owner.with_session(|session| generation2d_render_body(body_key, doc, cfg, view_state, session)))
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("generation2d.eval-session-owner", error.message))?
    }
    /// 🗂️ Grouped disclosure: `addWidget`/`reorganize`/`generate` stay top-level; the display-mode
    /// toggle, generation authoring, and generation selection each fold into their own taxonomy group;
    /// the delete-selection item stays a direct destructive item last.
    ///
    /// 🕹️ `context_menu` carries no `InteractionView` either (same gap as `render` — see ticket
    /// 26/08/14's w3b-summary.md), so the selection-dependent delete row below always takes the
    /// "nothing selected" branch rather than reading a stale/wrong selection.
    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &ArtifactView<'_, Generation2dSnapshot>,
        _cfg: &ConfigView<'_, Generation2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        {
            let labels = semio_framework_plugin::resolve_labels::<Generation2dLabels>(view_state);
            let is_de = view_state.locale == semio_framework_plugin::Locale::De;
            let selected: Vec<String> = Vec::new();
            let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);
            let mut menu = Menu::of(registry).action("addWidget").action("reorganize").action("generate");
            menu = menu.group("mode", |m| m.action("setShowMode"));
            menu = menu.group("create", |m| m.action("addGeneration"));
            menu = menu.group("methods", |m| m.action("selectGeneration"));
            if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
                menu = menu.item(spec);
            }
            menu.build()
        }
    }

    /// 🎞️ Declares `export_media`'s default document schema — pack-encodes `doc.snapshot`, wrapped
    /// `Structured{schema: Self::DOCUMENT_SCHEMA, json: base64}` — plus `"drawing:out"`.
    fn export_media(port: &str, doc: &ArtifactView<'_, Generation2dSnapshot>) -> Result<semio_framework_plugin::Media, semio_framework_plugin::MediaError> {
        match port {
            "drawing:out" => {
                let eval_json = crate::standards::v1::subsets::any::schema::evaluate_generation_preview(&doc.snapshot.fixture, &semio_framework_artifact_playbook_playbook::PlaybookValues::new());
                let layers_json = crate::standards::v1::subsets::any::schema::generation_preview_layers(&eval_json);
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "2d.drawing".into(), json: layers_json } })
            }
            "document:out" => {
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media {
                    media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
                    payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            _ => Err(semio_framework_plugin::MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"params:in"`: a generic Data×Value JSON object `{widgetId: number}` — patches matching
    /// `InputSlider` widgets' `value` field, leaving unmatched keys/widget kinds untouched.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &ArtifactView<'_, Generation2dSnapshot>) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        if port != "params:in" {
            return Err(semio_framework_plugin::MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(semio_framework_plugin::MediaError::Payload(port.to_string(), "params:in expects a Structured JSON object payload".into()));
        };
        let parsed = dsl::json::parse(json).map_err(|error| semio_framework_plugin::MediaError::Payload(port.to_string(), error.to_string()))?;
        let Some(object) = parsed.as_object() else {
            return Err(semio_framework_plugin::MediaError::Payload(port.to_string(), "params:in payload must be a JSON object".into()));
        };
        let mut operations = Vec::new();
        for (widget_id_key, value) in object.iter() {
            let Some(number) = value.as_f64() else { continue };
            let Some(widget) = doc.snapshot.fixture.widgets.iter().find(|widget| crate::widget_id(widget) == widget_id_key) else { continue };
            if let semio_framework_artifact_flow_flow::Widget::InputSlider { id, label, min, max, step, .. } = widget {
                operations.push(crate::standards::v1::subsets::any::schema::mutations::text::replace_widget(semio_framework_artifact_flow_flow::Widget::InputSlider {
                    id: id.clone(),
                    label: label.clone(),
                    value: number,
                    min: *min,
                    max: *max,
                    step: *step,
                }));
            }
        }
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️Generation2dPlayApp

//#region 🔖️Manifest
pub fn create_generation2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(GENERATION2D_DIALECT)
        .document(["semio", "procedural", "2d"])
        .artifact_kind(artifact_kind())
        .icon_id("generation2d")
        .mode_def(edit::definition())
        .mode_def(generate::definition())
        .mode_layout(generate::GENERATION2D_PLAY_MODE_GENERATE, generate::GENERATION2D_PLAY_LAYOUT_GENERATE)
        .default_mode_id(edit::GENERATION2D_PLAY_MODE_EDIT)
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
        // 🗂️ Referenced by `Generation2dPlayApp::context_menu` — categorized for grouped-context-menu disclosure.
        .action_with(categorized_action("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Mutation, "selection"))
        .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
        .action_with(categorized_action("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Mutation, "create"))
        .mutation("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"))
        .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Ports verbinden"))
        .action_with(categorized_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "transform"))
        .action_with(categorized_action("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Mutation, "create"))
        .mutation("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"))
        .mutation("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"))
        .mutation("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"))
        // 👁️ Ephemeral view actions — camera, the show-mode display toggle, and evaluation scratch
        // (emit no operations). Selection/hover are the framework's `graph` interaction domain now
        // (`.interaction(...)` below) — the six framework verbs auto-inject.
        .action_with(ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"), ActionKind::View, "camera"))
        .action_with(categorized_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"), ActionKind::View, "mode"))
        .action_with(categorized_action("generate", LocalizedLabel::native("Generate", "Generieren"), ActionKind::View, "actions"))
        .view_action("setEvalOutputs", LocalizedLabel::native("Set Eval Outputs", "Auswertungsausgaben festlegen"))
        .action_with(ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Canvas-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
        .action_with(ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Canvas-Zeiger bewegt"), ActionKind::View, "mouse-pointer"))
        .action_with(ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Canvas-Zeiger losgelassen"), ActionKind::View, "mouse-pointer"))
        .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Canvas-Mausrad"))
        .action_with(categorized_action("selectGeneration", LocalizedLabel::native("Select Generation", "Generation auswählen"), ActionKind::View, "methods"))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), ActionKind::View) })
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
        .action_interactive_job("moveMediaNode", InteractiveJobClassification::Migrated)
        .action_interactive_job("addWidget", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeWidget", InteractiveJobClassification::Migrated)
        .action_interactive_job("connectMediaPorts", InteractiveJobClassification::Migrated)
        .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
        .action_interactive_job("addGeneration", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeGeneration", InteractiveJobClassification::Migrated)
        .action_interactive_job("renameGeneration", InteractiveJobClassification::Migrated)
        .action_interactive_job("updateGenerationValues", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
        .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
        .action_interactive_job("generate", InteractiveJobClassification::Migrated)
        .action_interactive_job("setEvalOutputs", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasWheel", InteractiveJobClassification::Migrated)
        .action_interactive_job("selectGeneration", InteractiveJobClassification::Migrated)
        .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung aufnehmen"), "runtime", ActionKind::View) }))
        .action_interactive_job("flowEvalTick", InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalResolve", InteractiveJobClassification::Migrated)
        .command(migrated_command(CommandDefinition {
            in_palette: false,
            ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([
                ActionArgDef::text("json", LocalizedLabel::native("Contributions Page", "Beiträge-Seite")),
                ActionArgDef::text("page", LocalizedLabel::native("Page", "Seite")),
                ActionArgDef::text("pageCount", LocalizedLabel::native("Page Count", "Seitenanzahl")),
            ])
        }))
        .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
        // 📝️ Staged argument form for the palette-visible add-widget action (default materialized host-side).
        .action_args("addWidget", vec![
            ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                ActionArgOption::new("neuron", LocalizedLabel::native("Component", "Komponente")),
                ActionArgOption::new("outputPreview", LocalizedLabel::native("Preview", "Vorschau")),
                ActionArgOption::new("outputExport", LocalizedLabel::native("Export", "Export")),
            ]).default_value(&"inputSlider"),
        ])
        // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
        // one domain over the flow-graph widget DAG, node/edge/handle granularities,
        // `HierarchyProvider::Topology` (see `Generation2dPlayApp::interaction_topology` above) —
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
        .window_kind_interactions(flow_window::GENERATION2D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
        .window_kind_interactions(edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW, vec![InteractionRef::new("graph")])
        .window_kind_interactions(generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
        // 📇️ Window-scoped action ownership — the generation2d twin of generation3d's own block
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END), asserted by
        // `every_emitted_action_is_declared_on_its_window_kind`. The node-graph host's two verbs on the
        // flow window, the canvas host's four pointer verbs on the two Canvas2d scene windows, the
        // generation tree's four row verbs on the Generations window, the form's one change verb on the
        // Form window. Everything else — `setShowMode`, `generate`, `addWidget`, `reorganize`,
        // `setEvalOutputs`, the framework's history ids — is app-scoped, stays unowned, and is therefore
        // copied onto every window by `build_definition`.
        .window_kind_action_refs(flow_window::GENERATION2D_PLAY_WINDOW_MAIN, vec!["nodeGraphEdit".into(), "nodeGraphViewport".into()])
        .window_kind_action_refs(edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW, vec!["canvasPointerDown".into(), "canvasPointerMove".into(), "canvasPointerUp".into(), "canvasWheel".into()])
        .window_kind_action_refs(generations::GENERATION2D_PLAY_WINDOW_GENERATIONS, vec!["addGeneration".into(), "selectGeneration".into(), "renameGeneration".into(), "removeGeneration".into()])
        .window_kind_action_refs(form::GENERATION2D_PLAY_WINDOW_GENERATE_FORM, vec!["updateGenerationValues".into()])
        .window_kind_action_refs(generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW, vec!["canvasPointerDown".into(), "canvasPointerMove".into(), "canvasPointerUp".into(), "canvasWheel".into()])
        .keybinding("mod+z", "undo")
        .keybinding("mod+shift+z", "redo")
        .config(Generation2dPlayApp::config_spec())
        .io(semio_framework::io::resolve_ready(generation2d_io()))
        // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
        // `AppDefinition`, not the old `App { definition, examples }` — there is no `.example(...)`/
        // `.workflow(...)` on this builder, so the old `"default"` app-level example registration and
        // the no-op `.workflow("generation2d", …)` call are dropped here (not silently — reported in
        // this packet's migration notes). The subset's own `📚️examples` facet is the modern,
        // role-agnostic replacement surface for this.
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

/// 🧱️ Every window body of the generation2d editor, rendered against ONE already-resolved evaluation
/// session — shared by the marks-free `render` (scratch session) and `render_with_request_context`
/// (the instance's retained session) so there is exactly one body-key match in the app.
fn generation2d_render_body(
    body_key: &str,
    doc: &ArtifactView<'_, Generation2dSnapshot>,
    cfg: &ConfigView<'_, Generation2dConfig>,
    view_state: &semio_framework_plugin::ViewModel,
    session: &FlowEvalSession,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let labels = generation2d_labels(view_state);
    let rendered = match body_key {
        flow_window::GENERATION2D_PLAY_BODY_MAIN => flow_window::render(document, config, session),
        edit_preview::GENERATION2D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, session),
        generations::GENERATION2D_PLAY_BODY_GENERATIONS => generations::render(&document.generation, view_state.locale, view_state.terminology),
        form::GENERATION2D_PLAY_BODY_GENERATE_FORM => form::render(document, &document.generation, labels),
        generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(config, None, labels),
        document_panel::GENERATION2D_PLAY_BODY_DOCUMENT => document_panel::render(document, config, labels),
        catalogue_panel::GENERATION2D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
        inspection_panel::GENERATION2D_PLAY_BODY_INSPECTION => inspection_panel::render(document, config, labels),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
    };
    Ok(semio_framework_plugin::built_to_component_tree(rendered?))
}
