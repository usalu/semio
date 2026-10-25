//! 🌐️ Flow eval chain and bounded job factory: the flow eval window work, the inline continuation, the flow resolve work and the bounded command job factory. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
//#region ⏱️FlowEvalChain
/// ⏱️ The viewer's window-scoped `flowEvalTick` route — the read-only twin of the sibling surface's
/// `Generation3dFlowEvalWindowWork`, running the identical shared chain.
///
/// 🪟️ The tick's window is the one its PAYLOAD names, never the one the shell happened to be
/// focused on when it redispatched the self-armed effect. `retained_window_transient_target`
/// captured that exact window's transient authority (validating it against the trusted ViewModel
/// roster first), so the two only have to agree here.
pub(super) struct Generation3dViewFlowEvalWindowWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) tessellate: preview_eval::PreviewTessellateReceive,
    evaluation:preview_eval::PreviewEvalReceive,
    pub(super) complete: bool,
    pub(super) closing: bool,
}

impl Generation3dViewFlowEvalWindowWork {
    pub(super) fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, tessellate: preview_eval::PreviewTessellateReceive::new(),evaluation:preview_eval::PreviewEvalReceive::new(), complete: false, closing: false }
    }
}

/// 🔁️ The wave a just-folded extension answer unblocked, run INLINE inside that answer's own guest
/// turn — the read-only twin of the editor's `flow_eval_tick::continue_inline`, over the document
/// this surface is LOOKING at.
///
/// 🅿️ A refused continuation publishes nothing and touches no latch: the window still owes the hop
/// it owed, and the `previewEval` run job dispatches it on its next step.
fn generation3d_view_continue_inline(
    window_id: &str,
    window_kind_id: &str,
    host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    tolerance: f64,
    session: &mut FlowEvalSession,
    retained_eval: Option<&str>,
    turn_started_us: Option<u64>,
    retained_grant: semio_framework_value::RetainedCloneGrant,
) -> Result<preview_eval::FlowEvalTickOutcome, semio_framework_value::ValueError> {
    if !session.inline_continuation_admitted(window_id, turn_started_us, semio_framework_job::default_now_us()) {
        return Ok(preview_eval::FlowEvalTickOutcome { extension_invocations: Vec::new(), publication: semio_framework_os_flow::FlowEvalPublication::Retained, census_moved: false });
    }
    session.arm_window_tick(window_id, retained_grant)?;
    preview_eval::evaluate_tick(window_id, window_kind_id, host_snapshot, tolerance, session, retained_eval, turn_started_us, retained_grant)
}

/// 🐢️ The viewer's binding of [`preview_eval::chain_ui_scope`] — one preview body, no graph body
/// (a read-only surface paints no node-graph computing chrome) and the framework tool-run panel that
/// carries the abort affordance.
fn generation3d_view_chain_ui_scope(census_moved: bool) -> semio_framework::kernel::UiDirtyScope {
    preview_eval::chain_ui_scope(preview::BODY_KEY, None, &[semio_framework_plugin::FRAMEWORK_TOOL_RUN_BODY_KEY], census_moved)
}

/// 🪟️ The preview window one chain route addresses, or `None` for a route that addresses none.
pub(super) fn generation3d_view_flow_eval_window_address(command: &Generation3dViewCommand) -> Option<(&str, &str)> {
    match command {
        Generation3dViewCommand::FlowEvalTick(payload) => Some((payload.window_id.as_str(), payload.window_kind_id.as_str())),
        Generation3dViewCommand::FlowEvalResolve(payload) => Some((payload.window_id.as_str(), payload.window_kind_id.as_str())),
        Generation3dViewCommand::FlowTessellateResolve(payload) => Some((payload.window_id.as_str(), payload.window_kind_id.as_str())),
        _ => None,
    }
}

impl ArtifactCommandWork<ViewerApp<Generation3dViewer>> for Generation3dViewFlowEvalWindowWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Generation3dViewCommand,
        _snapshot: &Generation3dSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<Generation3dViewer>>>,
    ) -> Option<usize> {
        let (window_id, window_kind_id) = generation3d_view_flow_eval_window_address(command)?;
        let context = context?;
        let window = context.window_transient.as_ref()?;
        (!window_id.is_empty()
            && window.window_id() == window_id
            && window_kind_id == window.window_kind_id()
            && preview_eval::preview_kind(window.window_kind_id(), GENERATION3D_VIEW_PREVIEW_KINDS).is_some()
            && window.get::<preview::transient::Generation3dViewPreviewWindowTransientOwner>().is_some())
        .then_some(1)
    }

    fn receive_original_command(&mut self,command:&mut Generation3dViewCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if let Generation3dViewCommand::FlowTessellateResolve(payload)=command{self.tessellate.configure_scope([Some(preview::BODY_KEY),None],[Some(semio_framework_plugin::FRAMEWORK_TOOL_RUN_BODY_KEY),None]);return self.tessellate.receive(payload,cx)}if let Generation3dViewCommand::FlowEvalResolve(payload)=command{self.evaluation.configure_scope([Some(preview::BODY_KEY),None],[Some(semio_framework_plugin::FRAMEWORK_TOOL_RUN_BODY_KEY),None]);return self.evaluation.receive(payload,cx)}Ok(true)
    }

    fn work_demands(&self,input:&ArtifactCommandInputs<'_,ViewerApp<Generation3dViewer>>,maximum_copy_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Generation3dViewCommand::FlowTessellateResolve(payload)=input.command{return self.instance_owner.inspect_original::<Generation3dViewInstanceOperationOwner,_>(|owner|owner.eval_session.as_ref().map_or_else(||Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original preview session is closing")),|session|self.tessellate.demands(payload,session,maximum_copy_bytes)))?.unwrap_or(Ok(Default::default()))}
        if let Generation3dViewCommand::FlowEvalResolve(payload)=input.command{return self.instance_owner.inspect_original::<Generation3dViewInstanceOperationOwner,_>(|owner|owner.eval_session.as_ref().map_or_else(||Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original preview session is closing")),|session|self.evaluation.demands(payload,session,maximum_copy_bytes)))?.unwrap_or(Ok(Default::default()))}
        command_work_step_demands::<ViewerApp<Generation3dViewer>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, ViewerApp<Generation3dViewer>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<ViewerApp<Generation3dViewer>>, semio_framework_job::OriginalTurnFailure> {
        if let Generation3dViewCommand::FlowTessellateResolve(payload)=input.command{
            if self.complete||self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original preview command work is terminal").into())}
            if self.tessellate.is_folded(){if let Some(ui_scope)=self.tessellate.publish_scope(cx)?{self.complete=true;return Ok(ArtifactCommandWorkStep::Complete(Emit{ui_scope,..Default::default()}))}}else{self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner,_>(|owner|owner.with_session_waking(|session|self.tessellate.step(payload,session,cx)))??;}
            return Ok(ArtifactCommandWorkStep::Progress{stage:"previewReceive",preview:br#"{"en":"Receiving preview geometry","de":"Vorschaugeometrie wird empfangen"}"#})
        }
        if let Generation3dViewCommand::FlowEvalResolve(payload)=input.command{
            if self.complete||self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original preview command work is terminal").into())}
            if self.evaluation.is_folded(){if let Some(ui_scope)=self.evaluation.publish_scope(cx)?{self.complete=true;return Ok(ArtifactCommandWorkStep::Complete(Emit{ui_scope,..Default::default()}))}}else{self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner,_>(|owner|owner.with_session_waking(|session|self.evaluation.step(payload,session,cx)))??;}
            return Ok(ArtifactCommandWorkStep::Progress{stage:"previewReceive",preview:br#"{"en":"Receiving evaluation output","de":"Auswertungsergebnis wird empfangen"}"#})
        }
        let original: Result<_, Fault> = (|| {
        if self.complete || self.closing {
            return Err(Fault::from("generation3d-view-flow-eval-window-work-terminal"));
        }
        let turn_started_us = semio_framework_job::default_now_us();
        let (payload_window_id, payload_window_kind_id) = generation3d_view_flow_eval_window_address(input.command).ok_or_else(|| Fault::from("generation3d-view-flow-eval-window-command-mismatch"))?;
        let context = input.context.ok_or_else(|| Fault::from("generation3d-view-flow-eval-window-context-required"))?;
        let window = context.window_transient.as_ref().ok_or_else(|| Fault::from("generation3d-view-flow-eval-window-transient-required"))?;
        if window.window_id() != payload_window_id || payload_window_kind_id != window.window_kind_id() || preview_eval::preview_kind(window.window_kind_id(), GENERATION3D_VIEW_PREVIEW_KINDS).is_none() {
            return Err(Fault::from("generation3d-view-flow-eval-window-owner-mismatch"));
        }
        let retained_eval = window.get::<preview::transient::Generation3dViewPreviewWindowTransientOwner>().map(|state| state.preview_eval_text.as_deref()).ok_or_else(|| Fault::from("generation3d-view-flow-eval-window-owner-required"))?;
        let tolerance = input.config.tolerance();
        let viewed = Generation3dViewedDocument::resolve(input.snapshot, input.config);
        let retained_grant=cx.retained_grant();
        let outcome = self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| {
            owner.with_session_waking(|session| {
                let host_snapshot = &viewed.snapshot().host_snapshot;
                match input.command {
                    Generation3dViewCommand::FlowEvalTick(_) => preview_eval::evaluate_tick(window.window_id(), window.window_kind_id(), host_snapshot, tolerance, session, retained_eval, None, retained_grant).map_err(|error| Fault::from(error.to_string())),
                    Generation3dViewCommand::FlowEvalResolve(_) => unreachable!(),
                    Generation3dViewCommand::FlowTessellateResolve(_) => unreachable!(),
                    _ => Err(Fault::from("generation3d-view-flow-eval-window-command-mismatch")),
                }
            })
        });
        viewed.retire();
        let outcome = outcome??;
        self.complete = true;
        let window_transient = match outcome.publication {
            semio_framework_os_flow::FlowEvalPublication::Retained => Vec::new(),
            semio_framework_os_flow::FlowEvalPublication::Changed(eval_text) => vec![preview::transient::addressed(window, eval_text)?],
        };
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit: Emit { extension_invocations: outcome.extension_invocations, ui_scope: generation3d_view_chain_ui_scope(outcome.census_moved), ..Default::default() },
            ephemeral: EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient },
        })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self){self.closing=true;self.tessellate.begin_close();self.evaluation.begin_close();}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{self.begin_close();if !self.tessellate.terminal_is_empty(){self.tessellate.close_step(cx)}else{self.evaluation.close_step(cx)}}
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if !self.tessellate.terminal_is_empty(){self.tessellate.close_demands(0)?.copy_bytes}else{self.evaluation.close_demands(0)?.copy_bytes})}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(if !self.tessellate.terminal_is_empty(){self.tessellate.close_demands(copy)?.capacity_bytes}else{self.evaluation.close_demands(copy)?.capacity_bytes})}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if !self.tessellate.terminal_is_empty(){self.tessellate.close_demands(0)?.release_bytes}else{self.evaluation.close_demands(0)?.release_bytes})}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if !self.tessellate.terminal_is_empty(){self.tessellate.close_demands(0)?.depth}else{self.evaluation.close_demands(0)?.depth})}
    fn terminal_is_empty(&self)->bool{self.closing&&self.tessellate.terminal_is_empty()&&self.evaluation.terminal_is_empty()}
}

/// 🪄️ that address an extension ACTOR rather than a preview window, and so publish no store lane at all
/// (`HostOnly`). The window-addressed folds live on [`Generation3dViewFlowEvalWindowWork`] instead,
/// because they continue the chain and owe their window a publication.
pub(super) struct Generation3dViewFlowResolveWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
}

impl ArtifactCommandWork<ViewerApp<Generation3dViewer>> for Generation3dViewFlowResolveWork {
    fn receive_original_command(&mut self,command:&mut Generation3dViewCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if let Generation3dViewCommand::FlowTessellateCancelResolve(payload)=command{let g=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||g.maximum_items==0||g.maximum_depth==0{return Ok(false)}if let Some(fault)=payload.fault.take(){cx.consume_fuel(1);let original=semio_framework_job::OriginalTurnFailure::from(fault);let receipt_refusal=cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}).err();return Err(semio_framework_job::OriginalTurnFailure{receipt_refusal,..original})}if !payload.input_valid{cx.consume_fuel(1);return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original cancel reply routing input is malformed").with_retained_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}),cx))}}Ok(true)
    }
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
        if self.consumed {
            return Err(Fault::from("generation3d-view-flow-resolve-work-repeated"));
        }
        self.consumed = true;
        let extension_invocations = self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| {
            owner.with_session_waking(|session| match input.command {
                Generation3dViewCommand::FlowEvalRelease(payload) => Ok(preview_eval::release_invocations(payload)),
                Generation3dViewCommand::FlowTessellateCancelResolve(payload) => {
                    Ok(preview_eval::resolve_tessellate_cancel(payload, session))
                }
                _ => Err(Fault::from("generation3d-view-flow-resolve-route-rejected")),
            })?
        })?;
        Ok(ArtifactCommandWorkStep::Complete(Emit { extension_invocations, ..Default::default() }))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}
//#endregion ⏱️FlowEvalChain

pub(super) struct Generation3dViewBoundedCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation3dViewBoundedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
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

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: GENERATION3D_VIEW_RAW_BYTES, checkpoint_resume: false);

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
