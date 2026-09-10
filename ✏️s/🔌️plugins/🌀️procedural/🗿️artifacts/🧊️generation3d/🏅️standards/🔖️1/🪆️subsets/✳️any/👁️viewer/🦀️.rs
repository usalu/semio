//! 👁️ Generation3d viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Generation3dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Generation3dViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT import anything from the sibling `✏️editor` module
//! (`policyViewerPurityBreaches`).
//!
//! 🕹️ Read-only does not mean inert. A user opening a generation3d artifact read-only gets: a real
//! `World3dScene` of the evaluated geometry, live hover and selection over the framework's own
//! `graph` interaction domain, a preview camera they can orbit/pan/zoom, four shading modes, three
//! LOD steps and a full sun rig — every one of them a `Migrated` interactive job with an exact
//! app-owned bounded reducer, publishing on the CONFIG lane only. That publication contract is the
//! runtime half of the read-only guarantee whose compile-time half is `ViewEmit`.

use crate::viewer::generation3d::commands::{set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::viewer::generation3d::modes::view;
use crate::viewer::generation3d::modes::view::windows::preview;
use crate::viewer::generation3d::presence::{Generation3dViewPresence, Generation3dViewPresenceMutation};
use crate::viewer::generation3d::transient::{Generation3dViewTransient, Generation3dViewTransientMutation, SetPreviewEval};
use crate::{Generation3dMutation, Generation3dSnapshot, GENERATION3D_DIALECT, GENERATION_3D_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_os_flow::{FlowEvalSession, FlowHost};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    app::InteractionView, ActionDefinition, ActionDescriptor, ActionKind, AppOperationContext, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane,
    ArtifactView, ConfigView, Dialect, DomainTopology, Emit, EphemeralEmit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, InteractiveJobClassification, Label,
    LocalizedLabel, MergeMode, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, ViewEmit, Viewer, ViewerApp, WindowMeasure,
};
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const GENERATION3D_VIEW_APP_ID: &str = "procedural3d-view";

/// 🎯️ An `ActionDescriptor` addressed at this viewer — the single factory every chrome measure
/// builds its `on_change` with.
pub fn generation3d_view_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: GENERATION3D_VIEW_APP_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }
}
//#endregion 🔖️Constants

//#region 🔖️Command
semio_framework_plugin::view_commands! {
    /// 👁️ `Generation3dViewer::Command` — the SOLE dispatch surface for the read-only surface's own
    /// behavior, covering EVERY declared action. Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break. `dispatch` returns `ViewEmit`, so no row of this
    /// table can ever carry an artifact or draft mutation.
    pub enum Generation3dViewCommand for Generation3dSnapshot, Generation3dViewConfig, Generation3dViewConfigMutation {
        "setShowMode" as "show-mode" => set_show_mode::SetShowMode,
        "setLodMode" as "lod-mode" => set_lod_mode::SetLodMode,
        "setCamera" as "camera" => set_camera::SetCamera,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setContributions" as "set-contributions" => set_contributions::SetContributions}
}

impl Default for Generation3dViewCommand {
    fn default() -> Self {
        Generation3dViewCommand::SetShowMode(set_show_mode::SetShowMode { value: crate::viewer::generation3d::config::default_view_show_mode() })
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedCommands
/// 🧾️ Every viewer tool id, in `Generation3dViewCommand` declaration order — a bijection with the
/// command enum's rows and with `Generation3dViewBoundedCommandJobFactory::PUBLICATION_CONTRACTS`.
const GENERATION3D_VIEW_TOOL_IDS: &[&str] = &["setShowMode", "setLodMode", "setCamera", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity"];
const GENERATION3D_VIEW_PAYLOAD_SCHEMA: &str = "generation.3d.view-command.v1";
/// 🎒️ Real bound for one viewer command's wire payload: the largest is `setCamera`'s two coordinate
/// triples plus a fov, well under a kilobyte — 8 KiB stays a real ceiling, not a rubber stamp.
const GENERATION3D_VIEW_RAW_BYTES: usize = 8_192;
/// 🎒️ One config edit plus the ephemeral presence/transient completion — the evaluation itself is
/// chunked across `step()` calls, not across work items.
const GENERATION3D_VIEW_WORK_ITEMS: usize = 8;
/// 🎛️ Admission envelope for ONE encoded viewer config mutation on the config publication lane —
/// the largest is `setCamera`'s two coordinate triples plus a fov, so 8 KiB is the same real
/// ceiling `GENERATION3D_VIEW_RAW_BYTES` places on the wire payload it is decoded from.
const GENERATION3D_VIEW_CONFIG_STORE_MAXIMUM_BYTES: usize = GENERATION3D_VIEW_RAW_BYTES;

fn generation3d_view_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_RAW_BYTES, 32, 32, 16_384, 7_500)
}

#[expect(clippy::unnecessary_wraps, reason = "The retained work contract takes an optional extent callback.")]
fn generation3d_view_bounded_extent(_command: &Generation3dViewCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 👁️ The one reducer every viewer tool runs. It converts the typed command's `ViewEmit` into the
/// runtime `Emit` the retained-command machinery publishes — built solely from `ViewEmit`'s three
/// fields, so the document and draft lanes stay empty BY CONSTRUCTION here exactly as they do in
/// `ViewerApp::handle`, never by a runtime check a later edit could forget.
#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn generation3d_view_retained_reduce(
    command: &Generation3dViewCommand,
    snapshot: &Generation3dSnapshot,
    config: &Generation3dViewConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<Generation3dViewer>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Generation3dMutation, Generation3dViewConfigMutation, NoDraftMutation>, Fault> {
    if !GENERATION3D_VIEW_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("generation3d-view-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    let view_emit = command.dispatch(&doc, &cfg)?;
    Ok(Emit { config_mutations: view_emit.config_mutations, effects: view_emit.effects, ui_scope: view_emit.ui_dirty, ..Default::default() })
}

/// 🧮️ Folds this command's emitted config operations onto the live config, so the ephemeral
/// presence broadcast reports the state the co-viewer is about to see rather than the one it left.
fn generation3d_view_next_config(base: &Generation3dViewConfig, mutations: &[Generation3dViewConfigMutation]) -> Generation3dViewConfig {
    let mut next = base.clone();
    for mutation in mutations {
        next = protocol::Mutation::diff(mutation, &next).into_parts().0;
    }
    next
}

/// 👁️ One viewer command's real work: settle the config operation, then drive the flow evaluation
/// to completion in bounded `Progress` steps (cancellable through `begin_close`/`close_step`), and
/// complete with the ephemeral halves — SHARED presence (camera + show mode, so a co-viewer can
/// follow) and LOCAL-ONLY transient (the evaluated geometry the preview window repaints from).
struct Generation3dViewCommandWork {
    tool_id: &'static str,
    emit: Option<Emit<Generation3dMutation, Generation3dViewConfigMutation, NoDraftMutation>>,
    presence: Vec<Generation3dViewPresenceMutation>,
    host: Option<FlowHost>,
    /// 🧹️ The host's explicit retirement ladder — a bare `Option<FlowHost>::take()`-and-drop panics
    /// on the cloned fixture's `OrderedMap<WidgetLayout>` root, so it is drained under the grant.
    host_retirement: Option<semio_framework_os_flow::FlowHostRetirement>,
    session: Option<FlowEvalSession>,
    started: bool,
    complete: bool,
    closing: bool,
}

impl Generation3dViewCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, emit: None, presence: Vec::new(), host: None, host_retirement: None, session: None, started: false, complete: false, closing: false }
    }

    /// 📤️ Completes the command, publishing the evaluated geometry ONLY when this tick's eval JSON
    /// differs from what the transient already retains — a republication of identical bytes costs a
    /// fresh allocation now and a byte-proportional retirement later, for no repaint.
    fn finish(&mut self, publication: &semio_framework_os_flow::FlowEvalPublication) -> Result<ArtifactCommandWorkStep<ViewerApp<Generation3dViewer>>, Fault> {
        self.complete = true;
        let emit = self.emit.take().ok_or_else(|| Fault::from("generation3d-view-emit-owner-absent"))?;
        let presence = std::mem::take(&mut self.presence);
        let transient = match publication {
            semio_framework_os_flow::FlowEvalPublication::Retained => Vec::new(),
            semio_framework_os_flow::FlowEvalPublication::Changed(eval_text) => vec![Generation3dViewTransientMutation::from(SetPreviewEval { eval_text: eval_text.clone() })],
        };
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence, transient, window_transient: Vec::new() } })
    }
}

impl ArtifactCommandWork<ViewerApp<Generation3dViewer>> for Generation3dViewCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Generation3dViewCommand,
        snapshot: &Generation3dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<Generation3dViewer>>>,
    ) -> Option<usize> {
        generation3d_view_bounded_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, ViewerApp<Generation3dViewer>>) -> Result<ArtifactCommandWorkStep<ViewerApp<Generation3dViewer>>, Fault> {
        if self.closing || self.complete {
            return Err(Fault::from("generation3d-view-work-is-terminal"));
        }
        if !self.started {
            self.started = true;
            let emit = generation3d_view_retained_reduce(input.command, input.snapshot, input.config, input.history, input.interaction, input.hover, input.context, input.operation)?;
            let next = generation3d_view_next_config(input.config, &emit.config_mutations);
            self.presence = vec![
                Generation3dViewPresenceMutation::from(crate::viewer::generation3d::presence::SetPreviewCamera { camera: next.preview_camera.clone() }),
                Generation3dViewPresenceMutation::from(crate::viewer::generation3d::presence::SetShowMode { value: next.show_mode.clone() }),
            ];
            self.emit = Some(emit);
            let mut host = FlowHost::from_fixture(input.snapshot.fixture.clone());
            host.set_neuron_kind_info_map(semio_framework_os_flow::flow_neuron_kind_info_map());
            let mut session = FlowEvalSession::new();
            session.sync(&host);
            self.host = Some(host);
            self.session = Some(session);
            return Ok(ArtifactCommandWorkStep::Progress { stage: "generation3d-view-preview-evaluation", preview: b"{\"en\":\"Evaluating preview geometry\",\"de\":\"Vorschaugeometrie wird ausgewertet\"}" });
        }
        let host = self.host.as_mut().ok_or_else(|| Fault::from("generation3d-view-host-owner-absent"))?;
        let session = self.session.as_mut().ok_or_else(|| Fault::from("generation3d-view-session-owner-absent"))?;
        if session.tick(host) {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "generation3d-view-preview-evaluation", preview: b"{\"en\":\"Evaluating preview geometry\",\"de\":\"Vorschaugeometrie wird ausgewertet\"}" });
        }
        let retained_eval = input.context.and_then(|context| context.transient.preview_eval_text.as_deref());
        let publication = session.eval_publication_for(retained_eval);
        self.finish(&publication)
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

struct Generation3dViewBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dViewBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_VIEW_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Generation3dViewBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<Generation3dViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<Generation3dViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_VIEW_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_view_bounded_contract()
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
        if input.declared_bytes() > GENERATION3D_VIEW_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d viewer command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Generation3dViewBoundedCommandJobFactory {
    type Owner = ViewerApp<Generation3dViewer>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_VIEW_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 🔒️ CONFIG-ONLY, for every single tool. This table is the runtime half of the read-only
    /// guarantee: naming `ArtifactToolPublicationLane::Artifact` here would be rejected against a
    /// viewer's own emit, and no row does.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setShowMode", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 🧩️ContributionsRoute
/// 🧩️ The host→guest contributions route on the READ-ONLY surface. The viewer's preview window
/// evaluates its own geometry (`with_host(fixture, |host| host.evaluate())`), so it needs exactly
/// the same contributed operators the editor does; the registry it installs into is process-wide,
/// but a session that only ever opens this app is never handed the editor's own route
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];
const GENERATION3D_VIEW_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "generation.3d.view-contributions-command.v1";
/// 📐️ The REAL wire ceiling of one contributions page: the framework's own public-invocation string
/// bound — which no tool contract can widen, because `validate_public_json_envelope` runs before the
/// addressed tool's contract — at its worst-case escaped width, plus the addressed envelope.
const GENERATION3D_VIEW_CONTRIBUTIONS_ENVELOPE_BYTES: usize = 4_096;
const GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::PUBLIC_INVOCATION_STRING_BYTES * semio_framework::PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR + GENERATION3D_VIEW_CONTRIBUTIONS_ENVELOPE_BYTES;

fn generation3d_view_contributions_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES, 32, 32, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page. Publishes nothing at all — not even the config lane every
/// other viewer tool writes.
struct Generation3dViewContributionsWork {
    consumed: bool,
}

impl ArtifactCommandWork<ViewerApp<Generation3dViewer>> for Generation3dViewContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
    }

    fn extent(
        &self,
        command: &Generation3dViewCommand,
        snapshot: &Generation3dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<Generation3dViewer>>>,
    ) -> Option<usize> {
        generation3d_view_bounded_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, ViewerApp<Generation3dViewer>>) -> Result<ArtifactCommandWorkStep<ViewerApp<Generation3dViewer>>, Fault> {
        if self.consumed {
            return Err(Fault::from("generation3d-view-contributions-work-repeated"));
        }
        self.consumed = true;
        let Generation3dViewCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("generation3d-view-contributions-route-rejected"));
        };
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: None };
        set_contributions::handle(payload, &doc, &cfg)?;
        Ok(ArtifactCommandWorkStep::Complete(Emit::default()))
    }
}

struct Generation3dViewContributionsJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dViewContributionsJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Generation3dViewContributionsJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<Generation3dViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<Generation3dViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_VIEW_CONTRIBUTIONS_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_view_contributions_contract()
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
        if input.declared_bytes() > GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d viewer contributions command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Generation3dViewContributionsJobFactory {
    type Owner = ViewerApp<Generation3dViewer>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

struct Generation3dViewBoundedCommandJobFactoryProofs;

impl Generation3dViewBoundedCommandJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Generation3dViewer>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#viewer",
        document_schema: "generation.3d",
        factory: "Generation3dViewBoundedCommandJobFactory",
        factory_type: Generation3dViewBoundedCommandJobFactory,
        contract: generation3d_view_bounded_contract(),
        tools: ["setShowMode", "setLodMode", "setCamera", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity"]
    }
}

struct Generation3dViewContributionsJobFactoryProofs;

impl Generation3dViewContributionsJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Generation3dViewer>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#viewer",
        document_schema: "generation.3d",
        factory: "Generation3dViewContributionsJobFactory",
        factory_type: Generation3dViewContributionsJobFactory,
        contract: generation3d_view_contributions_contract(),
        tools: ["setContributions"]
    }
}
//#endregion 🧩️ContributionsRoute


//#region 🔖️InteractionTopology
/// 🕸️ Every node's visible port ids (`{nodeId}@{portId}`) — read-only twin of the sibling surface's
/// own projection, so a world pick in this viewer names the exact same declared target ids.
fn generation3d_view_port_ids_by_node(fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> std::collections::BTreeMap<String, Vec<String>> {
    let (graph_nodes, _) = crate::standards::v1::subsets::any::schema::with_host(fixture, |host| crate::standards::v1::subsets::any::schema::fixture_to_workflow(&host.dag.fixture));
    graph_nodes.into_iter().map(|node| (node.id, node.inputs.into_iter().chain(node.outputs).map(|port| port.id).collect())).collect()
}
//#endregion 🔖️InteractionTopology

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Generation3dViewer;

impl semio_framework_plugin::ArtifactViewer for Generation3dViewer {
    type Snapshot = Generation3dSnapshot;
    type Mutation = Generation3dMutation;
    type Config = Generation3dViewConfig;
    type ConfigMutation = Generation3dViewConfigMutation;
    type Presence = Generation3dViewPresence;
    type PresenceMutation = Generation3dViewPresenceMutation;
    type Transient = Generation3dViewTransient;
    type TransientMutation = Generation3dViewTransientMutation;
    type Command = Generation3dViewCommand;

    const DIALECT: Dialect = GENERATION3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_document_store_owners())
    }

    /// 🗃️ The viewer holds a real document store (read-only, but owned), so it owes the same bounded
    /// disposer the editor does — without it `PluginApp::close_step` fails closed with
    /// `interactive-job.close-owned-disposer-missing` and NO viewer fixture can ever reach its
    /// terminal-empty witness (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(
            semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default()).expect("default Generation3d viewer presence is the exact empty terminal"),
        ))
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    /// 🎛️ The viewer's `setCamera`/`setShowMode`/`setLodMode`/sun tools all publish onto the CONFIG
    /// lane; without this authority every one of them fails closed with
    /// `interactive-job.publication-authority-missing` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("generation3d-view-config-retained", GENERATION3D_VIEW_CONFIG_STORE_MAXIMUM_BYTES))
    }

    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Presence, Self::PresenceMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Generation3dViewBoundedCommandJobFactory::new(&controller))?;
        registry.register(Generation3dViewContributionsJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GENERATION3D_VIEW_TOOL_IDS.contains(&request.tool_id.as_str()) && !GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("generation3d-view-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let contributions = GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id);
        let work: Box<dyn ArtifactCommandWork<ViewerApp<Generation3dViewer>>> =
            if contributions { Box::new(Generation3dViewContributionsWork { consumed: false }) } else { Box::new(Generation3dViewCommandWork::new(tool_id)) };
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
            Generation3dViewCommand::command_id,
            if contributions { GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES } else { GENERATION3D_VIEW_RAW_BYTES },
            GENERATION3D_VIEW_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🧾️ BOTH factories' proofs, in registration order. The bounded rows now read their contract
    /// off `generation3d_view_bounded_contract()` instead of respelling `8_192, 32, 32, …` seven
    /// times — one declared quantity, one place.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Generation3dViewBoundedCommandJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(Generation3dViewContributionsJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    /// 🎯️ Maps the host's declared command id + JSON args onto this viewer's typed command — the
    /// bridge `PluginApp::handle_command` takes for EVERY structurally addressed invocation
    /// (`plugin_handle_command` → `A::command_from_action`). Without it the shell's own
    /// `setContributions` push, and every chrome measure's `on_change`, fail closed with
    /// `app.command.unsupported` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or_else(dsl::DslValue::null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(dsl::DslValue::as_f64)) };
        let u64_arg = |keys: &[&str]| -> Option<u64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|number| number as u64)))) };
        match action {
            "setShowMode" => Ok(Generation3dViewCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode", "show_mode"]).unwrap_or_default() })),
            "setLodMode" => Ok(Generation3dViewCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "setCamera" => Ok(Generation3dViewCommand::SetCamera(set_camera::SetCamera {
                camera: args
                    .get("camera")
                    .and_then(|camera| <crate::viewer::generation3d::config::Generation3dViewCamera as dsl::FromValue>::from_value(camera.clone()).ok())
                    .unwrap_or_default(),
            })),
            "toggleSun" => Ok(Generation3dViewCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setSunAzimuth" => Ok(Generation3dViewCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunElevation" => Ok(Generation3dViewCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunIntensity" => Ok(Generation3dViewCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setContributions" => Ok(Generation3dViewCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json"]).unwrap_or_default(),
                page: u64_arg(&["page"]).unwrap_or_default(),
                page_count: u64_arg(&["pageCount", "page_count"]).unwrap_or(1),
            })),
            other => Err(Fault::from(format!("action '{other}' is not declared by the generation3d viewer"))),
        }
    }

    /// 🏷️ The declared tool id of the command, NOT the trait's generic `"typed-command"` default:
    /// `qualified_tool_proof` looks the bounded first-step proof up by this verb, so leaving the
    /// default in place makes EVERY viewer action fail closed with `interactive-job.missing-factory`.
    fn command_id(command: &Generation3dViewCommand) -> &'static str {
        command.command_id()
    }

    fn initial_snapshot() -> Generation3dSnapshot {
        crate::standards::v1::subsets::any::schema::default_snapshot()
    }

    /// 👁️ Structurally read-only: `dispatch` returns `ViewEmit`, so this signature cannot widen into
    /// a document write no matter what a payload module does.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕸️ The same `graph` topology the sibling surface declares — every top-level widget is a
    /// `node`, each of its visible ports a `handle` parented to it, each synapse an `edge`, and each
    /// `Cluster`'s nested neurons `node`s parented to their cluster (the transitive-hover source).
    /// Without this a world pick would be pruned by `validate_state` and nothing would ever light up.
    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> InteractionTopology {
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
        let ports_by_node = generation3d_view_port_ids_by_node(fixture);
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

    /// 🕹️ The marks-free entry point the framework still offers — every live window goes through
    /// `render_with_request_context` instead.
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        generation3d_view_render_body(body_key, doc.snapshot, cfg.snapshot, None, &preview::Generation3dViewMarks::default())
    }

    /// 🕹️ Resolves the live `graph` hover/selection once per render and threads it into the preview
    /// body, alongside the ephemeral evaluated geometry the last command computed.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        generation3d_view_render_body(body_key, doc.snapshot, cfg.snapshot, transient.snapshot.preview_eval_text.as_deref(), &preview::Generation3dViewMarks::from_interaction(interaction))
    }

    /// 🎚️ The Preview window's chrome: show mode, LOD and the sun group, all bound to this viewer's
    /// own actions so the read-only surface's window controls actually do something.
    fn window_measures(_doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(preview::WINDOW_KIND_ID.to_string(), preview::preview_window_measures(cfg.snapshot, generation3d_view_action))])
    }
}

/// 🧱️ Every window body of the generation3d viewer, rendered against one already-resolved set of
/// `graph` marks — one body-key match for both render entry points.
fn generation3d_view_render_body(
    body_key: &str,
    document: &Generation3dSnapshot,
    config: &Generation3dViewConfig,
    eval_json: Option<&str>,
    marks: &preview::Generation3dViewMarks,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let node = match body_key {
        preview::BODY_KEY => preview::render(document, config, eval_json, marks),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_generation3d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(GENERATION3D_DIALECT)
        .document(["semio", "procedural", "3d"])
        .icon_id("workflow")
        .mode_def(view::definition())
        .default_mode_id(view::GENERATION3D_VIEW_MODE_VIEW)
        .window_kind_def(preview::definition())
        .default_layout(view::layout())
        // 👁️ Ephemeral view actions — the only kind a read-only surface may declare. Every one is
        // config-only; none reaches the document.
        .action_with(ActionDefinition::new("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"), ActionKind::View, "eye"))
        .action_with(ActionDefinition::new("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
        .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
        .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
        .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
        .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
        .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
        .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
        .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
        .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
        .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
        .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
        .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
        .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
        // 🧩️ Not a view action: a hidden host COMMAND, because the host pushes it and no user ever
        // invokes it. It publishes nothing at all, not even the config lane the seven above write.
        .command({
            let mut definition = semio_framework_plugin::CommandDefinition {
                in_palette: false,
                ..semio_framework_plugin::CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([
                    semio_framework_plugin::ActionArgDef::text("json", LocalizedLabel::native("Contributions Page", "Beiträge-Seite")),
                    semio_framework_plugin::ActionArgDef::text("page", LocalizedLabel::native("Page", "Seite")),
                    semio_framework_plugin::ActionArgDef::text("pageCount", LocalizedLabel::native("Page Count", "Seitenanzahl")),
                ])
            };
            definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
            definition
        })
        // 🕹️ First-class hover/selection over the same flow-graph widget DAG the sibling surface
        // declares — read-only, but a viewer still hovers, selects and inspects. Selection stays
        // `broadcast: true` so a co-viewer sees what this one is looking at.
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
        .window_kind_interactions(preview::WINDOW_KIND_ID, vec![InteractionRef::new("graph")])
        // 📇️ Window-scoped action ownership (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). This viewer has
        // exactly one window and exactly seven view actions, and that window's own chrome dispatches all
        // seven — `setShowMode`/`setLodMode` and the sun group from `preview_window_measures`, `setCamera`
        // from the world host's viewport gesture. Declaring them explicitly is what keeps
        // `WindowKindDefinition.actions` a statement about this window rather than a copy of the app list
        // (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5334-5338`). Asserted by
        // `every_emitted_action_is_declared_on_the_preview_window_kind`.
        .window_kind_action_refs(preview::WINDOW_KIND_ID, vec![
            "setShowMode".into(),
            "setLodMode".into(),
            "setCamera".into(),
            "toggleSun".into(),
            "setSunAzimuth".into(),
            "setSunElevation".into(),
            "setSunIntensity".into(),
        ])
        .build_definition()
}
//#endregion 🔖️Manifest

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
