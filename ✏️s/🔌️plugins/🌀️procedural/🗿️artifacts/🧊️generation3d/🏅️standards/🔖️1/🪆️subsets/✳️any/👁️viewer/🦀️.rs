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

use crate::preview_eval;
use crate::viewer::generation3d::commands::{cancel_preview_eval, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::viewer::generation3d::modes::view;
use crate::viewer::generation3d::modes::view::windows::preview;
use crate::viewer::generation3d::presence::{Generation3dViewPresence, Generation3dViewPresenceMutation};
use crate::viewer::generation3d::transient::{Generation3dViewTransient, Generation3dViewTransientMutation};
use crate::{Generation3dMutation, Generation3dSnapshot, GENERATION3D_DIALECT, GENERATION_3D_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_os_flow::FlowEvalSession;
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

/// 🪟️ The window kinds this surface's `flowEvalTick` chain may be addressed to — this viewer has
/// exactly one, and it is the ONLY window holding a retained evaluation publication. Handed to the
/// shared [`preview_eval::attached_preview_windows`] / [`preview_eval::preview_kind`], which is
/// where the editor's two-preview roster is expressed the same way.
const GENERATION3D_VIEW_PREVIEW_KINDS: &[&str] = &[preview::WINDOW_KIND_ID];

/// 🪟️ Every attached viewer preview window, off the shell's trusted `ViewModel` roster.
fn generation3d_view_preview_windows(view: Option<&semio_framework_plugin::ViewModel>) -> Vec<(&str, &'static str)> {
    preview_eval::attached_preview_windows(view, GENERATION3D_VIEW_PREVIEW_KINDS)
}
/// 📚️ What this read-only surface is LOOKING at: the document the session opened, or — once the
/// navbar picker named one — the bundled example's own projection.
///
/// 🔒️ This is how a viewer "loads" a document without owning write authority. The editor's
/// `setActiveExample` replaces the artifact's fixture; this surface cannot, so the picked id lives
/// on its CONFIG (`Generation3dViewConfig::active_example_id`) and every read path — the preview
/// render, the `flowEvalTick` chain and the interaction topology — resolves it here, exactly once,
/// through the surface-neutral `🧬️schema::example_snapshot`.
///
/// 🧹️ `Example` owns a real `Generation3dSnapshot` whose neural `Dictionary` roots abort the process
/// on a bare drop, so every caller MUST finish with [`Self::retire`] (ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END).
enum Generation3dViewedDocument<'a> {
    Opened(&'a Generation3dSnapshot),
    Example(Generation3dSnapshot),
}

impl<'a> Generation3dViewedDocument<'a> {
    fn resolve(snapshot: &'a Generation3dSnapshot, config: &Generation3dViewConfig) -> Self {
        match crate::standards::v1::subsets::any::schema::example_snapshot(&config.active_example_id) {
            Some(example) => Self::Example(example),
            None => Self::Opened(snapshot),
        }
    }

    fn snapshot(&self) -> &Generation3dSnapshot {
        match self {
            Self::Opened(snapshot) => snapshot,
            Self::Example(example) => example,
        }
    }

    fn retire(self) {
        if let Self::Example(example) = self {
            example.retire_cold();
        }
    }
}
//#endregion 🔖️Constants

//#region 🔖️InstanceOperationOwner
/// 🧠️ The ONE `FlowEvalSession` this viewer instance retains across turns — its neural cache, its
/// `eval_json`, its live preview meshes and, critically, its `pending_tessellate_by_hash` table.
///
/// 🐛️ Before this owner the viewer built a throwaway `FlowEvalSession::new()` per command and
/// ticked it synchronously to completion (`👁️viewer/🦀️.rs:179-207` as of 2026-09-12): an in-flight
/// tessellate handle noted while emitting the request was gone before any answer could arrive, and
/// no answer could arrive at all because the sync loop emitted no `ExtensionInvocation`. The
/// brep/math operators are CONTRIBUTED at runtime and never linked into the guest, so that loop
/// could only ever fault — the read-only surface could never paint a brep-bearing example
/// (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 1). Mirrors the sibling surface's own
/// retained owner exactly.
struct Generation3dViewInstanceOperationOwner {
    eval_session: Option<FlowEvalSession>,
    closing: bool,
}

impl Generation3dViewInstanceOperationOwner {
    fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), closing: false }
    }

    fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("generation3d-view-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("generation3d-view-eval-session-owner-missing"))
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for Generation3dViewInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    /// 🧹️ A LIVE session owns nothing retirable — `FlowEvalSession::close_step` answers `Blocked`
    /// until `begin_close`, and reporting that from the live maintenance ladder spends the
    /// runtime's zero-progress credit every idle turn.
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
            semio_framework_job::InteractiveJobCloseStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "Generation3d viewer evaluation session awaits its exact close grant" },
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

//#endregion 🔖️InstanceOperationOwner

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
        "setContributions" as "set-contributions" => set_contributions::SetContributions,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "flowTessellateResolve" as "flow-tessellate-resolve" => flow_tessellate_resolve::FlowTessellateResolve,
        "cancelPreviewEval" as "cancel-preview-eval" => cancel_preview_eval::CancelPreviewEval,
        "flowTessellateCancelResolve" as "flow-tessellate-cancel-resolve" => flow_tessellate_cancel_resolve::FlowTessellateCancelResolve,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample}
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
/// 🎨️ The navbar example picker's own tool id — an APP-scoped gesture, dispatched from the shell
/// chrome rather than from a window, exactly as it is on the sibling surface. Kept out of
/// [`GENERATION3D_VIEW_TOOL_IDS`] for that reason: that list is the statement of what THIS viewer's
/// one window dispatches, and `build_definition` copies an unowned action onto every window kind
/// anyway, which is what gets it past `ShellHost`'s `declaredAction` gate
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const GENERATION3D_VIEW_EXAMPLE_TOOL_IDS: &[&str] = &["setActiveExample"];
/// ⏱️ The runtime chain's own tool ids — a HOST route, never a user action: nobody clicks a tick.
/// Kept out of [`GENERATION3D_VIEW_TOOL_IDS`] for exactly that reason, so the window-kind action
/// law stays a statement about what a user can dispatch from this window.
const GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS: &[&str] = &["flowEvalTick", "flowEvalResolve", "flowTessellateResolve", "cancelPreviewEval", "flowTessellateCancelResolve"];
const GENERATION3D_VIEW_PAYLOAD_SCHEMA: &str = "generation.3d.view-command.v1";
const GENERATION3D_VIEW_FLOW_EVAL_PAYLOAD_SCHEMA: &str = "generation.3d.view-flow-eval-command.v1";
const GENERATION3D_VIEW_EXAMPLE_PAYLOAD_SCHEMA: &str = "generation.3d.view-example-command.v1";
/// 🎒️ One example id on the wire — the same 8 KiB ceiling every other viewer gesture lives under.
const GENERATION3D_VIEW_EXAMPLE_RAW_BYTES: usize = GENERATION3D_VIEW_RAW_BYTES;
/// 🎒️ Real bound for one chain hop's wire payload, and the same one the sibling editor surface's
/// proven chain declares (`GENERATION3D_FLOW_EVAL_RAW_BYTES`): a `flowTessellateResolve` carries a
/// whole `tessellate` envelope, i.e. one `brep_geometry::MESH_PACK_CHUNK_BASE64_CHARS` mesh-body
/// chunk plus its accounting header, and it is pinned to
/// `brep_geometry::tessellate_envelope_maximum_bytes()` by
/// `view_tessellate_envelope_fits_the_declared_wire_bound`.
///
/// ⚖️ It is NOT the 8 KiB gesture quota. An extension answer reaches its resolve as a parked
/// continuation rather than a public invocation body, but the body still crosses THIS route's
/// declared bound, and at 8 KiB the transfer unit collapsed to 4 KiB — one `flowEvalTick` round
/// trip per 4 KiB of mesh, i.e. ten of them for `sphere-cut-with-torus`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️preview-mesh-delivery-2026-09-12.md`).
const GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES: usize = 65_536;
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
    if !GENERATION3D_VIEW_TOOL_IDS.contains(&command.command_id()) && !GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.contains(&command.command_id()) {
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

/// 👁️ One viewer command's real work: settle the config operation and complete with the ephemeral
/// SHARED presence (camera + show mode, so a co-viewer can follow) — then RE-ARM the addressed
/// `flowEvalTick` chain for every attached preview window.
///
/// 🐛️ It used to build a fresh `FlowEvalSession::new()` per command and `tick` it synchronously to
/// completion here, with zero `ExtensionInvocation` sites in the whole file. The brep/math
/// operators are contributed by the host at runtime and never linked into the guest, so that loop
/// could only ever fault and no brep-bearing example could tessellate in the viewer at all. The
/// evaluation is now the shared addressed chain's job (`🧵️preview-eval`), exactly as it is on the
/// sibling surface (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 1).
struct Generation3dViewCommandWork {
    tool_id: &'static str,
    /// 🔒️ The app instance's retained evaluation session, reached ONLY to arm the preview chain
    /// through its per-window latch — see [`semio_framework_os_flow::FlowEvalSession::arm_window_tick`].
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    emit: Option<Emit<Generation3dMutation, Generation3dViewConfigMutation, NoDraftMutation>>,
    presence: Vec<Generation3dViewPresenceMutation>,
    complete: bool,
    closing: bool,
}

impl Generation3dViewCommandWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, emit: None, presence: Vec::new(), complete: false, closing: false }
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
        self.complete = true;
        let mut emit = generation3d_view_retained_reduce(input.command, input.snapshot, input.config, input.history, input.interaction, input.hover, input.context, input.operation)?;
        let next = generation3d_view_next_config(input.config, &emit.config_mutations);
        self.presence = vec![
            Generation3dViewPresenceMutation::from(crate::viewer::generation3d::presence::SetPreviewCamera { camera: next.preview_camera.clone() }),
            Generation3dViewPresenceMutation::from(crate::viewer::generation3d::presence::SetShowMode { value: next.show_mode.clone() }),
        ];
        // 🪟️ A view command that changes what the MESHES would be (the LOD deflection, the shading
        // filter) owes the chain a restart, addressed at the windows the SHELL says are attached —
        // never at a window this route invents. A pure camera/sun move changes no mesh, but the
        // chain is idempotent: an already-settled session re-publishes nothing.
        let windows = generation3d_view_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
        emit.effects.extend(self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| owner.with_session(|session| preview_eval::rearm_attached_previews(session, &windows)))?);
        let presence = std::mem::take(&mut self.presence);
        self.emit = Some(emit);
        let emit = self.emit.take().ok_or_else(|| Fault::from("generation3d-view-emit-owner-absent"))?;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence, transient: Vec::new(), window_transient: Vec::new() } })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Blocked;
        }
        if self.emit.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.emit.is_none()
    }
}

//#region ⏱️FlowEvalChain
/// ⏱️ The viewer's window-scoped `flowEvalTick` route — the read-only twin of the sibling surface's
/// `Generation3dFlowEvalWindowWork`, running the identical shared chain.
///
/// 🪟️ The tick's window is the one its PAYLOAD names, never the one the shell happened to be
/// focused on when it redispatched the self-armed effect. `retained_window_transient_target`
/// captured that exact window's transient authority (validating it against the trusted ViewModel
/// roster first), so the two only have to agree here.
struct Generation3dViewFlowEvalWindowWork {
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    complete: bool,
    closing: bool,
}

impl Generation3dViewFlowEvalWindowWork {
    fn new(instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { instance_owner, complete: false, closing: false }
    }
}

impl ArtifactCommandWork<ViewerApp<Generation3dViewer>> for Generation3dViewFlowEvalWindowWork {
    fn tool_id(&self) -> &'static str {
        "flowEvalTick"
    }

    fn extent(
        &self,
        command: &Generation3dViewCommand,
        _snapshot: &Generation3dSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<Generation3dViewer>>>,
    ) -> Option<usize> {
        let Generation3dViewCommand::FlowEvalTick(payload) = command else { return None };
        let context = context?;
        let window = context.window_transient.as_ref()?;
        (!payload.window_id.is_empty()
            && window.window_id() == payload.window_id
            && payload.window_kind_id == window.window_kind_id()
            && preview_eval::preview_kind(window.window_kind_id(), GENERATION3D_VIEW_PREVIEW_KINDS).is_some()
            && window.get::<preview::transient::Generation3dViewPreviewWindowTransientOwner>().is_some())
        .then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, ViewerApp<Generation3dViewer>>) -> Result<ArtifactCommandWorkStep<ViewerApp<Generation3dViewer>>, Fault> {
        if self.complete || self.closing {
            return Err(Fault::from("generation3d-view-flow-eval-window-work-terminal"));
        }
        let Generation3dViewCommand::FlowEvalTick(payload) = input.command else { return Err(Fault::from("generation3d-view-flow-eval-window-command-mismatch")) };
        let context = input.context.ok_or_else(|| Fault::from("generation3d-view-flow-eval-window-context-required"))?;
        let window = context.window_transient.as_ref().ok_or_else(|| Fault::from("generation3d-view-flow-eval-window-transient-required"))?;
        if window.window_id() != payload.window_id || payload.window_kind_id != window.window_kind_id() || preview_eval::preview_kind(window.window_kind_id(), GENERATION3D_VIEW_PREVIEW_KINDS).is_none() {
            return Err(Fault::from("generation3d-view-flow-eval-window-owner-mismatch"));
        }
        let retained_eval = window.get::<preview::transient::Generation3dViewPreviewWindowTransientOwner>().map(|state| state.preview_eval_text.as_deref()).ok_or_else(|| Fault::from("generation3d-view-flow-eval-window-owner-required"))?;
        let tolerance = input.config.tolerance();
        // 📚️ The tick evaluates what this surface is LOOKING at, which is the picked example once the
        // navbar named one — the read-only counterpart of the editor switch replacing its fixture.
        let viewed = Generation3dViewedDocument::resolve(input.snapshot, input.config);
        let outcome = self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| owner.with_session(|session| preview_eval::evaluate_tick(window.window_id(), window.window_kind_id(), &viewed.snapshot().fixture, tolerance, session, retained_eval)));
        viewed.retire();
        let outcome = outcome?;
        self.complete = true;
        let window_transient = match outcome.publication {
            semio_framework_os_flow::FlowEvalPublication::Retained => Vec::new(),
            semio_framework_os_flow::FlowEvalPublication::Changed(eval_text) => vec![preview::transient::addressed(window, eval_text)?],
        };
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit: Emit { effects: outcome.effects, extension_invocations: outcome.extension_invocations, ..Default::default() },
            ephemeral: EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient },
        })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if self.closing {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Blocked
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

/// ✅️ The viewer's `flowEvalResolve` / `flowTessellateResolve` route: folds one extension answer
/// into the app instance's RETAINED session — which is what makes `seed_node_cache` and
/// `resolve_preview_tessellate` land on the same cache the next tick and the next render read —
/// and re-arms the addressed chain. Publishes no store lane at all (`HostOnly`).
struct Generation3dViewFlowResolveWork {
    tool_id: &'static str,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    consumed: bool,
}

impl ArtifactCommandWork<ViewerApp<Generation3dViewer>> for Generation3dViewFlowResolveWork {
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
        if self.consumed {
            return Err(Fault::from("generation3d-view-flow-resolve-work-repeated"));
        }
        self.consumed = true;
        let mut invocations = Vec::new();
        let effects = self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| match input.command {
                Generation3dViewCommand::FlowEvalResolve(payload) => Ok(preview_eval::resolve_eval(payload, session)),
                Generation3dViewCommand::FlowTessellateResolve(payload) => Ok(preview_eval::resolve_tessellate(payload, session)),
                // 🛑️ The gesture's own route. It emits an EXTENSION INVOCATION rather than an
                // effect — the kernel jobs live in the geometry extension's own instance — so it is
                // the one row here whose emit is not purely `effects`.
                Generation3dViewCommand::CancelPreviewEval(payload) => {
                    invocations = preview_eval::cancel_preview_eval(payload, session);
                    Ok(Vec::new())
                }
                Generation3dViewCommand::FlowTessellateCancelResolve(payload) => {
                    preview_eval::resolve_tessellate_cancel(payload, session);
                    Ok(Vec::new())
                }
                _ => Err(Fault::from("generation3d-view-flow-resolve-route-rejected")),
            })?
        })?;
        Ok(ArtifactCommandWorkStep::Complete(Emit { effects, extension_invocations: invocations, ..Default::default() }))
    }
}
//#endregion ⏱️FlowEvalChain


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
/// evaluates its geometry through the SAME addressed `flowEvalTick` chain the sibling surface runs
/// (`🧵️preview-eval`), whose `evaluate`/`tessellate` hops are answered by the contributed brep and
/// math extensions — so it needs exactly the same contributed operators. The registry it installs
/// into is process-wide, but a session that only ever opens this app is never handed the sibling
/// surface's own route, which is why this one exists
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];
const GENERATION3D_VIEW_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "generation.3d.view-contributions-command.v1";
/// 📐️ The REAL wire ceiling of one contributions page: the framework's own public-invocation string
/// bound — which no tool contract can widen, because `validate_public_json_envelope` runs before the
/// addressed tool's contract — at its worst-case escaped width, plus the addressed envelope.
const GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::PUBLIC_INVOCATION_BODY_BYTES;

fn generation3d_view_contributions_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES, 32, 32, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page. Publishes nothing at all — not even the config lane every
/// other viewer tool writes.
struct Generation3dViewContributionsWork {
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
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
        // 🪟️ The install's re-arm is addressed to the windows the SHELL says are attached — the
        // same trusted `ViewModel` roster `pending_effects` arms the first chain off.
        let windows = generation3d_view_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
        let effects = self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| owner.with_session(|session| set_contributions::apply(payload, &doc, &cfg, session, &windows)))??;
        Ok(ArtifactCommandWorkStep::Complete(Emit { effects, ..Default::default() }))
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

//#region 🎨️ExampleRoute
fn generation3d_view_example_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_EXAMPLE_RAW_BYTES, 32, 32, 16_384, 7_500)
}

/// 🎨️ The navbar example picker's own factory. It runs the very same
/// [`Generation3dViewCommandWork`] every window gesture does — so the switch re-arms every attached
/// preview's chain from its own emit, never from a host `refresh-ui` — but it is registered
/// separately because it is an APP-scoped verb, not one this viewer's single window dispatches.
struct Generation3dViewExampleJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dViewExampleJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Generation3dViewExampleJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<Generation3dViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<Generation3dViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_VIEW_EXAMPLE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_view_example_contract()
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
        if input.declared_bytes() > GENERATION3D_VIEW_EXAMPLE_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d viewer example command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Generation3dViewExampleJobFactory {
    type Owner = ViewerApp<Generation3dViewer>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_VIEW_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 🔒️ Still read-only, and this is the row that proves it: loading an example on a viewer is a
    /// CONFIG edit plus an ephemeral presence broadcast. The sibling surface's own
    /// `setActiveExample` names `ArtifactToolPublicationLane::Artifact` here; this one cannot, and
    /// `no_viewer_tool_publishes_on_the_artifact_lane` holds it to that.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] =
        &[ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] }];
}

struct Generation3dViewExampleJobFactoryProofs;

impl Generation3dViewExampleJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Generation3dViewer>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#viewer",
        document_schema: "generation.3d",
        factory: "Generation3dViewExampleJobFactory",
        factory_type: Generation3dViewExampleJobFactory,
        contract: generation3d_view_example_contract(),
        tools: ["setActiveExample"]
    }
}
//#endregion 🎨️ExampleRoute

//#region ⏱️FlowEvalRoute
fn generation3d_view_flow_eval_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES, 32, 32, 16_384, 7_500)
}

/// ⏱️ The three runtime chain routes, on their OWN factory: their publication lanes are nothing
/// like a view command's. `flowEvalTick`'s only store lane is the ADDRESSED preview window's own
/// transient (never config, never presence); the two resolves write no store lane at all.
struct Generation3dViewFlowEvalJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dViewFlowEvalJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Generation3dViewFlowEvalJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<Generation3dViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<Generation3dViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_VIEW_FLOW_EVAL_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_view_flow_eval_contract()
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
        if input.declared_bytes() > GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d viewer flow-eval command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Generation3dViewFlowEvalJobFactory {
    type Owner = ViewerApp<Generation3dViewer>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 🔒️ Still read-only: `WindowTransient` is an EPHEMERAL lane. Naming
    /// `ArtifactToolPublicationLane::Artifact` here would be rejected against a viewer's own emit,
    /// and no row does.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowEvalResolve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "flowTessellateResolve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "cancelPreviewEval", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "flowTessellateCancelResolve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

struct Generation3dViewFlowEvalJobFactoryProofs;

impl Generation3dViewFlowEvalJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Generation3dViewer>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#viewer",
        document_schema: "generation.3d",
        factory: "Generation3dViewFlowEvalJobFactory",
        factory_type: Generation3dViewFlowEvalJobFactory,
        contract: generation3d_view_flow_eval_contract(),
        tools: ["flowEvalTick", "flowEvalResolve", "flowTessellateResolve", "cancelPreviewEval", "flowTessellateCancelResolve"]
    }
}
//#endregion ⏱️FlowEvalRoute


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

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_document_store_owners())
    }

    /// 🗃️ The viewer holds a real document store (read-only, but owned), so it owes the same bounded
    /// disposer the editor does — without it `PluginApp::close_step` fails closed with
    /// `interactive-job.close-owned-disposer-missing` and NO viewer fixture can ever reach its
    /// terminal-empty witness (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
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
        registry.register(Generation3dViewContributionsJobFactory::new(&controller))?;
        registry.register(Generation3dViewExampleJobFactory::new(&controller))?;
        registry.register(Generation3dViewFlowEvalJobFactory::new(&controller))
    }

    /// 🧠️ One retained `FlowEvalSession` per viewer instance — see
    /// [`Generation3dViewInstanceOperationOwner`]. Without it every chain hop would fold its answer
    /// into a session that died with the command that built it.
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Generation3dViewInstanceOperationOwner::new())
    }

    /// 🪟️ The viewer preview window's retained evaluation publication.
    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<preview::transient::Generation3dViewPreviewWindowTransientOwner>()
    }

    /// 🎯️ `flowEvalTick` publishes the evaluation into ONE preview window's retained transient, and
    /// the whole chain is self-dispatched: an `Effect::DispatchAction` reaches the shell with no
    /// window of its own and is redispatched under whichever window is current. Naming the target
    /// off the PAYLOAD makes the runtime validate it against the trusted ViewModel roster and
    /// capture that exact window's mutation authority (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        match command {
            Generation3dViewCommand::FlowEvalTick(payload) if !payload.window_id.is_empty() => preview_eval::preview_kind(&payload.window_kind_id, GENERATION3D_VIEW_PREVIEW_KINDS).map(|kind| (payload.window_id.as_str(), kind)),
            _ => None,
        }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the fixture has pending (uncomputed) nodes AND the
    /// preview window is actually attached to hold the evaluation it will publish. Before the first
    /// `Event::SurfaceVisible` the host has no roster at all (`view` is `None`) and there is nothing
    /// to publish into: this arms NOTHING and waits, rather than spinning a chain that cannot land.
    ///
    /// 🚧️ It also asks [`preview_eval::may_rearm`] first: while the graph's operator kinds are
    /// uncontributed, a tick can only recompute the same `flow.extension-not-contributed` miss and
    /// settle into another refresh. `setContributions` re-arms every attached preview once the
    /// registry moves.
    ///
    /// 🔒️ "Does this window owe a tick" is asked of the app instance's RETAINED session through
    /// `FlowEvalSession::arm_owed_window_tick`, never of a throwaway one — a scratch session's latch
    /// is always clear, so a poll that used one armed a second chain on top of the running one at
    /// every host refresh (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn pending_effects(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view: Option<&semio_framework_plugin::ViewModel>) -> Vec<semio_framework_plugin::Effect> {
        let windows = generation3d_view_preview_windows(view);
        if windows.is_empty() || !preview_eval::may_rearm(&doc.snapshot.fixture) {
            return Vec::new();
        }
        owner
            .with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| {
                owner.with_session(|session| {
                    session.retain_window_tick_latches(&windows.iter().map(|(window_id, _)| *window_id).collect::<Vec<_>>());
                    windows.iter().filter(|(window_id, _)| session.arm_owed_window_tick(window_id)).map(|(window_id, window_kind_id)| preview_eval::rearm(window_id, window_kind_id, 104)).collect::<Vec<_>>()
                })
            })
            .unwrap_or_default()
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GENERATION3D_VIEW_TOOL_IDS.contains(&request.tool_id.as_str())
            && !GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.contains(&request.tool_id.as_str())
            && !GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str())
            && !GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.contains(&request.tool_id.as_str())
        {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("generation3d-view-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let contributions = GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id);
        let example = GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.contains(&tool_id);
        let flow_eval = GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.contains(&tool_id);
        let work: Box<dyn ArtifactCommandWork<ViewerApp<Generation3dViewer>>> = if contributions {
            Box::new(Generation3dViewContributionsWork { instance_owner: request.instance_operation_owner, consumed: false })
        } else if tool_id == "flowEvalTick" {
            Box::new(Generation3dViewFlowEvalWindowWork::new(request.instance_operation_owner))
        } else if flow_eval {
            Box::new(Generation3dViewFlowResolveWork { tool_id, instance_owner: request.instance_operation_owner, consumed: false })
        } else {
            Box::new(Generation3dViewCommandWork::new(tool_id, request.instance_operation_owner))
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
            Generation3dViewCommand::command_id,
            if contributions {
                GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES
            } else if flow_eval {
                GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES
            } else if example {
                GENERATION3D_VIEW_EXAMPLE_RAW_BYTES
            } else {
                GENERATION3D_VIEW_RAW_BYTES
            },
            GENERATION3D_VIEW_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🧾️ ALL THREE factories' proofs, in registration order. The bounded rows read their contract
    /// off `generation3d_view_bounded_contract()` instead of respelling `8_192, 32, 32, …` seven
    /// times — one declared quantity, one place.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Generation3dViewBoundedCommandJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(Generation3dViewContributionsJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(Generation3dViewExampleJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(Generation3dViewFlowEvalJobFactoryProofs::bounded_first_step_tool_proofs());
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
            // 🎨️ `buildActiveExampleAction` (`🛠️ShellHelpers/🟦️.tsx`) sends `{exampleId}`; an empty
            // selection means "the document this session opened", never "no argument".
            "setActiveExample" => Ok(Generation3dViewCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "setSunAzimuth" => Ok(Generation3dViewCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunElevation" => Ok(Generation3dViewCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunIntensity" => Ok(Generation3dViewCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setContributions" => Ok(Generation3dViewCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json"]).unwrap_or_default(),
                page: u64_arg(&["page"]).unwrap_or_default(),
                page_count: u64_arg(&["pageCount", "page_count"]).unwrap_or(1),
            })),
            // ⏱️ The runtime chain. `windowId`/`windowKindId` are the address the tick, both
            // resolves and every re-arm carry; `nodeHash`/`outputJson` are echoed back onto a
            // response action by `reactor::extension_response_args`.
            "flowEvalTick" => Ok(Generation3dViewCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
            })),
            "flowEvalResolve" => Ok(Generation3dViewCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
                node_hash: u64_arg(&["nodeHash", "node_hash"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
                extension_id: str_arg(&["extensionId", "extension_id"]).unwrap_or_default(),
                ok: args.get("ok").and_then(dsl::DslValue::as_bool).unwrap_or(false),
                fault_code: str_arg(&["faultCode", "fault_code"]).unwrap_or_default(),
                fault_message: str_arg(&["faultMessage", "fault_message"]).unwrap_or_default(),
            })),
            "flowTessellateResolve" => Ok(Generation3dViewCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
                node_hash: u64_arg(&["nodeHash", "node_hash"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
            })),
            // 🛑️ The user's own stop gesture. The shell learns the verb from THIS surface's
            // published status contract (`cancelAction`), and `World3dHost` sends it with the
            // window address the status was published under.
            "cancelPreviewEval" => Ok(Generation3dViewCommand::CancelPreviewEval(cancel_preview_eval::CancelPreviewEval {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_else(|| preview::WINDOW_KIND_ID.to_string()),
            })),
            "flowTessellateCancelResolve" => Ok(Generation3dViewCommand::FlowTessellateCancelResolve(flow_tessellate_cancel_resolve::FlowTessellateCancelResolve {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
                ok: args.get("ok").and_then(dsl::DslValue::as_bool).unwrap_or(false),
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
    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> InteractionTopology {
        fn walk_neuron(neuron: &semio_framework_artifact_flow_flow::neural::Neuron, parent: String, ordered: &mut Vec<TopologyNode>) {
            ordered.push(TopologyNode { id: neuron.id.clone(), granularity: "node".into(), parent: Some(parent) });
            if let Some(tree) = &neuron.tree {
                for child in &tree.neurons {
                    walk_neuron(child, neuron.id.clone(), ordered);
                }
            }
        }
        let viewed = Generation3dViewedDocument::resolve(doc.snapshot, cfg.snapshot);
        let fixture = &viewed.snapshot().fixture;
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
        viewed.retire();
        InteractionTopology { domains }
    }

    /// 🕹️ The marks-free entry point the framework still offers (no owner, no transient, no
    /// interaction) — every live window goes through `render_with_request_context` instead.
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let viewed = Generation3dViewedDocument::resolve(doc.snapshot, cfg.snapshot);
        let tree = generation3d_view_render_body(body_key, viewed.snapshot(), cfg.snapshot, None, None, &preview::Generation3dViewMarks::default());
        viewed.retire();
        tree
    }

    /// 🕹️ Resolves the live `graph` hover/selection once per render and threads it into the preview
    /// body, alongside the evaluation the ADDRESSED chain published into this preview window's own
    /// transient and the retained session holding the meshes `flowTessellateResolve` folded in.
    ///
    /// 🪟️ The window-scoped publication is read first and the app-level lane is the fallback: the
    /// chain writes the window's own transient (that is what `retained_window_transient_target`
    /// captures authority for), and a surface rendered before any tick landed still has to paint
    /// its empty world rather than evaluate itself.
    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let preview_eval_text = transient
            .window::<preview::transient::Generation3dViewPreviewWindowTransientOwner>()
            .and_then(|state| state.preview_eval_text.as_deref())
            .or(transient.snapshot.preview_eval_text.as_deref());
        let marks = preview::Generation3dViewMarks::from_interaction(interaction);
        let viewed = Generation3dViewedDocument::resolve(doc.snapshot, cfg.snapshot);
        let tree = owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| owner.with_session(|session| generation3d_view_render_body(body_key, viewed.snapshot(), cfg.snapshot, preview_eval_text, Some(session), &marks)));
        viewed.retire();
        tree.map_err(|error| semio_framework_plugin::PluginAssemblyError::new("generation3d-view.eval-session-owner", error.message))?
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
    session: Option<&FlowEvalSession>,
    marks: &preview::Generation3dViewMarks,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let node = match body_key {
        preview::BODY_KEY => preview::render(document, config, eval_json, session, marks),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
/// ⏱️ One hidden runtime command of the evaluation chain. Never in the palette, never a `View`
/// action a window declares: a user does not dispatch a tick, the app does.
fn generation3d_view_runtime_command(id: &str, label: LocalizedLabel, args: &[&str]) -> semio_framework_plugin::CommandDefinition {
    let mut definition = semio_framework_plugin::CommandDefinition {
        in_palette: false,
        ..semio_framework_plugin::CommandDefinition::bounded_catalog(id, label, "runtime", ActionKind::View).with_args(args.iter().map(|arg| semio_framework_plugin::ActionArgDef::text(*arg, LocalizedLabel::native(*arg, *arg))).collect::<Vec<_>>())
    };
    definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    definition
}

/// 🎨️ The bundled examples this dialect offers, as the picker's declared argument options — the
/// SAME eight ids `editor::generation3d::examples()` registers onto the manifest, named through the
/// surface-neutral `🧬️schema` constants so this read-only surface never reaches into `::editor::`
/// (`policyViewerPurityBreaches`).
fn generation3d_view_example_options() -> Vec<semio_framework_plugin::ActionArgOption> {
    use crate::standards::v1::subsets::any::schema as generation3d_schema;
    use semio_framework_plugin::ActionArgOption;
    vec![
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")),
    ]
}

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
        // 🎨️ The navbar example picker's verb. `ActionKind::View`, not `Mutation` like the sibling
        // surface's: loading an example into a READ-ONLY surface writes this surface's config, never
        // the document — and `ShellHost` refuses a `mutation`-kind action on a viewer session outright
        // (`🏛️ShellHost/🟦️.tsx`'s read-only gate), so a `Mutation` row here would show the picker and
        // then silently swallow every pick.
        .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::View, "panel-left"))
        .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
        .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
        .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
        .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
        .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
        .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
        .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        .action_args("setActiveExample", vec![semio_framework_plugin::ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), generation3d_view_example_options()).required()])
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
        // ⏱️ The runtime evaluation chain — three hidden host COMMANDS, never user actions: the app
        // self-dispatches them through `Effect::DispatchAction`, and `reactor::extension_response_args`
        // addresses the two resolves. A command the manifest does not declare is dropped by
        // `ShellHost`'s own `declaredAction` gate before `plugin.handleAction` is ever called, so an
        // undeclared tick would stall the whole chain silently
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        .command(generation3d_view_runtime_command("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), &["windowId", "windowKindId"]))
        .command(generation3d_view_runtime_command("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung auflösen"), &["windowId", "windowKindId", "nodeHash", "outputJson"]))
        .command(generation3d_view_runtime_command("flowTessellateResolve", LocalizedLabel::native("Resolve Tessellation", "Tessellierung auflösen"), &["windowId", "windowKindId", "nodeHash", "outputJson"]))
        // 🛑️ The one chain verb a USER dispatches, so it is in the palette. The shell never learns
        // it from code: it learns it from this surface's own published status contract
        // (`cancelAction`), and `World3dHost`'s cancel button sends it with the `windowId` the
        // status was published under — the kind id is this viewer's only preview kind, so
        // `command_from_action` fills it in.
        .command({
            let mut definition = semio_framework_plugin::CommandDefinition::bounded_catalog("cancelPreviewEval", LocalizedLabel::native("Cancel Preview Computation", "Vorschauberechnung abbrechen"), "runtime", ActionKind::View)
                .with_args(["windowId", "windowKindId"].iter().map(|arg| semio_framework_plugin::ActionArgDef::text(*arg, LocalizedLabel::native(*arg, *arg))).collect::<Vec<_>>());
            definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
            definition
        })
        .command(generation3d_view_runtime_command("flowTessellateCancelResolve", LocalizedLabel::native("Resolve Preview Cancellation", "Vorschauabbruch aufnehmen"), &["windowId", "windowKindId", "outputJson", "ok"]))
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

//#region 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests


//#region 🧪️EvalChain
/// ⏱️ The read-only surface's half of the ONE addressed evaluation law — its own module because it
/// answers the SAME language-agnostic fixture (`🧫️fixtures/🪟️tick-addressing.json`) the sibling
/// surface answers, plus the served-order recovery law that proves a painted viewer preview.
#[cfg(test)]
#[path = "🧪️tests/🔬️eval-chain/🦀️.rs"]
mod eval_chain_tests;
//#endregion 🧪️EvalChain

//#region 🧪️StatusContract
/// 📈️ The read-only surface's half of the ONE preview-window status contract — its own module
/// because it answers the SAME language-agnostic fixture (`🧫️fixtures/🛑️preview-cancel.json`) the
/// editor's cancellation lane answers, whose third-party twin lives beside the editor's own cancel
/// command.
#[cfg(test)]
#[path = "🧪️tests/🔬️status-contract/🦀️.rs"]
mod status_contract_tests;
//#endregion 🧪️StatusContract
