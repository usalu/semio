//! 🧲️ Retained view commands: the retained tool ids and capacities, the bounded view contract, the retained reduction and the view command work. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
/// 🧾️ Every viewer tool id, in `Generation3dViewCommand` declaration order — a bijection with the
/// command enum's rows and with `Generation3dViewBoundedCommandJobFactory::PUBLICATION_CONTRACTS`.
pub const GENERATION3D_VIEW_TOOL_IDS: &[&str] = &["setShowMode", "setLodMode", "setCamera", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity"];

/// 🎨️ The navbar example picker's own tool id — an APP-scoped gesture, dispatched from the shell
/// chrome rather than from a window, exactly as it is on the sibling surface. Kept out of
/// [`GENERATION3D_VIEW_TOOL_IDS`] for that reason: that list is the statement of what THIS viewer's
/// one window dispatches, and `build_definition` copies an unowned action onto every window kind
/// anyway, which is what gets it past `ShellHost`'s `declaredAction` gate
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) const GENERATION3D_VIEW_EXAMPLE_TOOL_IDS: &[&str] = &["setActiveExample"];

/// ⏱️ The `previewEval` run's hop tool ids — a HOST route, never a user action: nobody clicks a tick.
/// Kept out of [`GENERATION3D_VIEW_TOOL_IDS`] for exactly that reason, so the window-kind action
/// law stays a statement about what a user can dispatch from this window.
pub(super) const GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS: &[&str] = &["flowEvalTick", "flowEvalResolve", "flowTessellateResolve", "flowEvalRelease", "flowTessellateCancelResolve"];

/// 🪟️ The chain routes ADDRESSED AT A PREVIEW WINDOW, sharing one publication layer
/// ([`Generation3dViewFlowEvalWindowWork`]): the dispatched hop and the two folds that continue the
/// chain inline. A release addresses an extension actor, not a window, and stays off it.
pub(super) const GENERATION3D_VIEW_FLOW_EVAL_WINDOW_TOOL_IDS: &[&str] = &["flowEvalTick", "flowEvalResolve", "flowTessellateResolve"];

/// 📤️ The reader's OWN io verb, on its own route. Export is the ONE direction a viewer may have —
/// it reads the document and hands the shell a file — so its publication lane is `HostOnly` and it
/// stays out of [`GENERATION3D_VIEW_TOOL_IDS`], whose every row writes the Config lane and
/// broadcasts presence. Import replaces a document and therefore lives only on the editor
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, io-surface lane).
pub(super) const GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS: &[&str] = &["exportDocument"];

pub(super) const GENERATION3D_VIEW_DOCUMENT_IO_PAYLOAD_SCHEMA: &str = "generation.3d.view-document-io-command.v1";

/// 🎒️ One format id and its addressed envelope — nothing this route carries INBOUND is larger than
/// a gesture, so it shares the gesture quota rather than declaring a second literal. The bytes flow
/// the other way, in a `DownloadMediaExport` effect, which no wire bound governs.
pub(super) const GENERATION3D_VIEW_DOCUMENT_IO_RAW_BYTES: usize = GENERATION3D_VIEW_RAW_BYTES;

pub(super) const GENERATION3D_VIEW_PAYLOAD_SCHEMA: &str = "generation.3d.view-command.v1";

pub(super) const GENERATION3D_VIEW_FLOW_EVAL_PAYLOAD_SCHEMA: &str = "generation.3d.view-flow-eval-command.v1";

pub(super) const GENERATION3D_VIEW_EXAMPLE_PAYLOAD_SCHEMA: &str = "generation.3d.view-example-command.v1";

/// 🎒️ One example id on the wire — the same 8 KiB ceiling every other viewer gesture lives under.
pub(super) const GENERATION3D_VIEW_EXAMPLE_RAW_BYTES: usize = GENERATION3D_VIEW_RAW_BYTES;

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
pub(super) const GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES: usize = 65_536;

/// 🎒️ Real bound for one viewer command's wire payload: the largest is `setCamera`'s two coordinate
/// triples plus a fov, well under a kilobyte — 8 KiB stays a real ceiling, not a rubber stamp.
pub(super) const GENERATION3D_VIEW_RAW_BYTES: usize = 8_192;

/// 🎒️ One config edit plus the ephemeral presence/transient completion — the evaluation itself is
/// chunked across `step()` calls, not across work items.
pub(super) const GENERATION3D_VIEW_WORK_ITEMS: usize = 8;

pub(super) fn generation3d_view_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_RAW_BYTES, 32, 32, 16_384, 7_500)
}

#[expect(clippy::unnecessary_wraps, reason = "The retained work contract takes an optional extent callback.")]
pub(super) fn generation3d_view_bounded_extent(_command: &Generation3dViewCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
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
        let diff = protocol::ColdMutation::diff(mutation, &next).into_parts().0;
        let applied = protocol::apply_diff_cold(&diff, &next);
        if let Ok(post) = applied {
            next = post;
        }
    }
    next
}

/// 👁️ One viewer command's real work: settle the config operation and complete with the ephemeral
/// SHARED presence (camera + show mode, so a co-viewer can follow) — then owe every attached preview
/// window an evaluation for the `previewEval` run.
///
/// 🐛️ It used to build a fresh `FlowEvalSession::new()` per command and `tick` it synchronously to
/// completion here, with zero `ExtensionInvocation` sites in the whole file. The brep/math
/// operators are contributed by the host at runtime and never linked into the guest, so that loop
/// could only ever fault and no brep-bearing example could tessellate in the viewer at all. The
/// evaluation is now the shared addressed chain's job (`🧵️preview-eval`), exactly as it is on the
/// sibling surface (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 1).
pub(super) struct Generation3dViewCommandWork {
    pub(super) tool_id: &'static str,
    /// 🔒️ The app instance's retained evaluation session, reached ONLY to owe the attached previews an
    /// evaluation on their per-window latches.
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) emit: Option<Emit<Generation3dMutation, Generation3dViewConfigMutation, NoDraftMutation>>,
    pub(super) presence: Vec<Generation3dViewPresenceMutation>,
    emit_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<Emit<Generation3dMutation, Generation3dViewConfigMutation, NoDraftMutation>>>,
    presence_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<Vec<Generation3dViewPresenceMutation>>>,
    pub(super) complete: bool,
    pub(super) closing: bool,
}

impl Generation3dViewCommandWork {
    pub(super) fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, emit: None, presence: Vec::new(), emit_retirement: None, presence_retirement: None, complete: false, closing: false }
    }

    pub(super) fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::RetirementDemand;
        if self.emit.is_some() || self.presence.capacity() != 0 {
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(retirement) = self.emit_retirement.as_ref() {
            return Ok(RetirementDemand { copy_bytes: retirement.next_copy_byte_demand()?, capacity_bytes: retirement.next_capacity_byte_demand(copy)?, release_bytes: retirement.next_release_byte_demand()?, depth: retirement.next_depth_demand()? });
        }
        if let Some(retirement) = self.presence_retirement.as_ref() {
            return Ok(RetirementDemand { copy_bytes: retirement.next_copy_byte_demand()?, capacity_bytes: retirement.next_capacity_byte_demand(copy)?, release_bytes: retirement.next_release_byte_demand()?, depth: retirement.next_depth_demand()? });
        }
        Ok(Default::default())
    }
}

/// 🧮️ The transient bytes one work step may materialize: the work's own frame plus the one emit it can stage.
pub(super) fn command_work_step_demands<A: semio_framework_plugin::ArtifactApp>(work_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    Ok(semio_framework_value::RetirementDemand { copy_bytes: work_bytes.saturating_add(size_of::<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>()), depth: 1, ..Default::default() })
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

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, ViewerApp<Generation3dViewer>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<ViewerApp<Generation3dViewer>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, ViewerApp<Generation3dViewer>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<ViewerApp<Generation3dViewer>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.closing || self.complete {
            return Err(Fault::from("generation3d-view-work-is-terminal"));
        }
        self.complete = true;
        let mut emit = generation3d_view_retained_reduce(input.command, input.snapshot, input.config, input.history, input.interaction, input.hover, input.context, input.operation)?;
        let next = generation3d_view_next_config(input.config, &emit.config_mutations);
        self.presence = vec![
            Generation3dViewPresenceMutation::from(semio_s_artifact_procedural_3d_viewer_presence::SetPreviewCamera { camera: next.preview_camera.clone() }),
            Generation3dViewPresenceMutation::from(semio_s_artifact_procedural_3d_viewer_presence::SetShowMode { value: next.show_mode.clone() }),
        ];
        if next.active_example_id != input.config.active_example_id || next.tolerance().to_bits() != input.config.tolerance().to_bits() {
            let windows = generation3d_view_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
            let viewed = Generation3dViewedDocument::resolve(input.snapshot, &next);
            let servable = preview_eval::may_rearm(&viewed.snapshot().host_snapshot);
            viewed.retire();
            self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| owner.owe_attached_previews_carrying(&windows, servable, &mut emit, input.operation.retained))?;
        }
        let presence = std::mem::take(&mut self.presence);
        self.emit = Some(emit);
        let emit = self.emit.take().ok_or_else(|| Fault::from("generation3d-view-emit-owner-absent"))?;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence, transient: Vec::new(), window_transient: Vec::new() } })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
        if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{(
            ||->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let refused = |error: semio_framework_value::ValueError| Err(error);
        let idle = RetainedCloneProgress::default();
        let one = RetainedCloneProgress { copied_items: 1, ..idle };
        if !self.closing {
            return Ok(InteractiveJobCloseStep::Blocked);
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(InteractiveJobCloseStep::Pending { progress: idle });
        }
        if let Some(emit) = self.emit.take() {
            return match semio_framework_value::retirement::controlled::ControlledRetirement::new(emit) {
                Ok(owner) => {
                    self.emit_retirement = Some(owner);
                    Ok(InteractiveJobCloseStep::Pending { progress: one })
                }
                Err((error, emit)) => {
                    self.emit = Some(emit);
                    refused(error)
                }
            };
        }
        if let Some(retirement) = self.emit_retirement.as_mut() {
            let step = retirement.step(grant);
            if retirement.terminal_is_empty() {
                self.emit_retirement = None;
            }
            return match step {
                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => Ok(InteractiveJobCloseStep::Pending { progress }),
                Err(error) => refused(error),
            };
        }
        if !self.presence.is_empty() || self.presence.capacity() != 0 {
            return match semio_framework_value::retirement::controlled::ControlledRetirement::new(std::mem::take(&mut self.presence)) {
                Ok(owner) => {
                    self.presence_retirement = Some(owner);
                    Ok(InteractiveJobCloseStep::Pending { progress: one })
                }
                Err((error, presence)) => {
                    self.presence = presence;
                    refused(error)
                }
            };
        }
        if let Some(retirement) = self.presence_retirement.as_mut() {
            let step = retirement.step(grant);
            if retirement.terminal_is_empty() {
                self.presence_retirement = None;
            }
            return match step {
                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => Ok(InteractiveJobCloseStep::Pending { progress }),
                Err(error) => refused(error),
            };
        }
        Ok(InteractiveJobCloseStep::Complete { progress: idle })
    })().map(|step|semio_framework_value::RetainedCloneStep::Progress(step.progress()))})?;
        Ok(if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }


    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.emit.is_none() && self.emit_retirement.is_none() && self.presence.is_empty() && self.presence.capacity() == 0 && self.presence_retirement.is_none()
    }
}
//#endregion 🧵️RetainedCommands
