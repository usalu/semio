//! 🧵️ Retained command route of the generation2d editor: the bounded, preview and flow-eval tool ids, the three command works and the bounded job factory. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
pub const GENERATION2D_BOUNDED_TOOL_IDS: &[&str] = &[
    "nodeGraphViewport",
    "setShowMode",
    "setActiveExample",
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

pub(super) const GENERATION2D_PREVIEW_TOOL_IDS: &[&str] = &["addGeneration", "removeGeneration", "renameGeneration", "updateGenerationValues", "selectGeneration"];

/// ⏱️ The `previewEval` run's hop routes, answered by [`Generation2dFlowEvalWork`].
pub(super) const GENERATION2D_FLOW_EVAL_TOOL_IDS: &[&str] = &["flowEvalTick", "flowEvalResolve"];

const GENERATION2D_RETAINED_PAYLOAD_SCHEMA: &str = "generation.2d.tool-command.v1";

pub const GENERATION2D_RETAINED_RAW_BYTES: usize = 8_192;

/// 🧮️ The ONE declared quantity of every generation2d retained route: at most 32 point-invertible
/// durable items per gesture. The preflight ceiling below, every `extent` a work answers, both
/// durable lanes' one-item store footprint and the bounded proof's work units are all read off it —
/// never spelled again. The preview ladder walks one item per fixture widget plus its own two
/// framing steps, which is why the declaration is 32 items and not one
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const GENERATION2D_RETAINED_CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(32);

pub(super) const GENERATION2D_RETAINED_WORK_ITEMS: usize = GENERATION2D_RETAINED_CAPACITY.work_items();

/// 🧾️ Decoded JSON items one retained command's args may carry — a WIRE bound, deliberately NOT the
/// work capacity.
pub(super) const GENERATION2D_RETAINED_DECODED_ITEMS: usize = 64;

/// 🧾️ The ONE execution contract all 21 routes publish — the factory's `execution_contract()` and
/// every row of `bounded_first_step_tool_proofs!` alike.
pub fn generation2d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION2D_RETAINED_RAW_BYTES, GENERATION2D_RETAINED_DECODED_ITEMS, GENERATION2D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

pub(super) fn generation2d_bounded_extent(_command: &Generation2dCommand, _snapshot: &Generation2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    GENERATION2D_RETAINED_CAPACITY.rows_for_items(1)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn generation2d_retained_reduce(
    command: &Generation2dCommand,
    snapshot: &Generation2dSnapshot,
    config: &Generation2dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation2dPlayApp>>>,
    operation: &AppOperationContext,
    session: &mut FlowEvalSession,
) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation, NoDraftMutation>, Fault> {
    if !GENERATION2D_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("generation2d-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    match command {
        Generation2dCommand::NodeGraphViewport(payload) => {
            let context = context.ok_or_else(|| Fault::from("generation2d-main-window-context-required"))?;
            let view = context.view_state.as_ref().ok_or_else(|| Fault::from("generation2d-main-window-view-required"))?;
            let base = flow_window::config::from_snapshot(context.window_config.as_ref());
            let mut next = base.clone();
            next.viewport = payload.viewport;
            Ok(Emit { window_config_mutations: flow_window::config::addressed(view, &base, next)?, ..Default::default() })
        }
        _ => command.dispatch(&doc, &cfg, session),
    }
}

/// 🧵️ The retained-session twin of `BoundedArtifactCommandWork` — see
/// `Generation2dInstanceOperationOwner` for why a per-dispatch session cannot carry a continuation.
pub(super) struct Generation2dSessionCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
}

impl Generation2dSessionCommandWork {
    pub(super) fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, consumed: false }
    }
}

/// 🧮️ The transient bytes one work step may materialize: the work's own frame plus the one emit it can stage.
pub(super) fn command_work_step_demands<A: semio_framework_plugin::ArtifactApp>(work_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    Ok(semio_framework_value::RetirementDemand { copy_bytes: work_bytes.saturating_add(size_of::<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>()), depth: 1, ..Default::default() })
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

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<EditorApp<Generation2dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation2dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("generation2d-session-command-work-repeated"));
        }
        self.consumed = true;
        let windows = generation2d_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
        let servable = semio_s_artifact_procedural_2d_preview_eval::may_rearm(&input.snapshot.host_snapshot);
        let emit = self.instance_owner.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| {
            let mut emit = owner.with_session(|session| generation2d_retained_reduce(input.command, input.snapshot, input.config, input.history, input.interaction, input.hover, input.context, input.operation, session))??;
            owner.owe_attached_previews_for_mutations(&windows, servable, &mut emit, input.operation.retained)?;
            Ok(emit)
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

/// ⏱️ The `previewEval` run's two hops against the instance's RETAINED sessions: `flowEvalTick`
/// evaluates the target its window kind names and records the digest it evaluated, `flowEvalResolve`
/// folds one extension answer into that target's session. Both wake the run job. The generate
/// preview's evaluation is published into the app transient its window renders.
pub(super) struct Generation2dFlowEvalWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
    pub(super) receiver:Option<semio_framework_os_flow::FlowEvaluationReplyReceiver>,
    pub(super) closing:bool,
}

impl ArtifactCommandWork<EditorApp<Generation2dPlayApp>> for Generation2dFlowEvalWork {
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
        let kind = match command {
            Generation2dCommand::FlowEvalTick(payload) => payload.window_kind_id.as_str(),
            Generation2dCommand::FlowEvalResolve(payload) => payload.window_kind_id.as_str(),
            _ => return None,
        };
        generation2d_preview_target(kind).and_then(|_| generation2d_bounded_extent(command, snapshot, interaction))
    }

    fn receive_original_command(&mut self,command:&mut Generation2dCommand,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let Generation2dCommand::FlowEvalResolve(payload)=command else{return Ok(true)};if self.receiver.is_some(){return Ok(true)}let g=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||g.maximum_items==0||g.maximum_depth==0{return Ok(false)}
        if let Some(fault)=payload.fault.take(){cx.consume_fuel(1);let original=semio_framework_job::OriginalTurnFailure::from(fault);let receipt_refusal=cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}).err();return Err(semio_framework_job::OriginalTurnFailure{receipt_refusal,..original})}
        if !payload.input_valid{cx.consume_fuel(1);return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original evaluation reply routing input is malformed").with_retained_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}),cx))}
        let output=std::mem::replace(&mut payload.output,semio_framework_value::DslValue::Null);match semio_framework_os_flow::FlowEvaluationReplyReceiver::new(output,cx){Ok(receiver)=>{self.receiver=Some(receiver);Ok(true)},Err((error,source))=>{match source{semio_framework_os_flow::FlowEvaluationReplyIntakeSource::Value(output)=>payload.output=output,semio_framework_os_flow::FlowEvaluationReplyIntakeSource::Receiver(receiver)=>self.receiver=Some(receiver)}Err(error)}}
    }
    fn work_demands(&self,input:&ArtifactCommandInputs<'_,EditorApp<Generation2dPlayApp>>,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Generation2dCommand::FlowEvalResolve(payload)=input.command{let receiver=self.receiver.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original evaluation reply was not donated"))?;let target=generation2d_preview_target(&payload.window_kind_id).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original evaluation window kind is unknown"))?;
            return self.instance_owner.inspect_original::<Generation2dInstanceOperationOwner,_>(|owner|match target{PreviewEvalTarget::Document=>owner.eval_session.as_ref(),PreviewEvalTarget::Generation=>owner.generation_session.as_ref()}.map_or_else(||Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original evaluation session is closing")),|session|receiver.next_demands(session,body,&payload.window_id)))?.unwrap_or(Ok(Default::default()))
        }command_work_step_demands::<EditorApp<Generation2dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation2dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        if let Generation2dCommand::FlowEvalResolve(payload)=input.command{
            if self.consumed||self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original evaluation command work is terminal").into())}let target=generation2d_preview_target(&payload.window_kind_id).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original evaluation window kind is unknown"))?;let receiver=self.receiver.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original evaluation reply was not donated"))?;let address=semio_framework_os_flow::FlowEvaluationReplyAddress{node_hash:payload.node_hash,window_id:&payload.window_id,window_kind_id:&payload.window_kind_id};
            let received=self.instance_owner.with_mut::<Generation2dInstanceOperationOwner,_>(|owner|{let(mut sessions,link)=owner.parts()?;let result=receiver.step(sessions.get_mut(target),&address,cx);link.wake();Ok(result)})??;
            if received{self.receiver=None;self.consumed=true;return Ok(ArtifactCommandWorkStep::Complete(Emit::default()))}return Ok(ArtifactCommandWorkStep::Progress{stage:"evaluationReceive",preview:br#"{"en":"Receiving evaluation output","de":"Auswertungsergebnis wird empfangen"}"#})
        }
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("generation2d-flow-eval-work-repeated"));
        }
        self.consumed = true;
        let overlay = generation2d_provisional_snapshot(input.snapshot, input.context.map_or(&[][..], |context| context.provisional()));
        let committed = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: None };
        let provisional_generation = input.context.map_or(0, |context| context.provisional_generation());
        let hop = {
            let doc = ArtifactView::with_operation(overlay.as_ref().unwrap_or(input.snapshot), input.history, input.operation.clone());
            self.instance_owner.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| generation2d_flow_eval_hop(owner, input.command, &doc, &committed, &cfg, provisional_generation))
        };
        if let Some(overlay) = overlay {
            overlay.retire_cold();
        }
        let (emit, transient) = hop?;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { transient, ..Default::default() } })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
    fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close()}}
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{self.begin_close();if let Some(receiver)=self.receiver.as_mut(){let step=receiver.close_step(cx)?;if receiver.terminal_is_empty(){self.receiver=None;}return Ok(if self.receiver.is_none(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})}Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.receiver.as_ref().map(|receiver|receiver.next_close_demands(0)).transpose()?.unwrap_or_default().copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.receiver.as_ref().map(|receiver|receiver.next_close_demands(copy)).transpose()?.unwrap_or_default().capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.receiver.as_ref().map(|receiver|receiver.next_close_demands(0)).transpose()?.unwrap_or_default().release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.receiver.as_ref().map(|receiver|receiver.next_close_demands(0)).transpose()?.unwrap_or_default().depth)}
    fn terminal_is_empty(&self)->bool{self.closing&&self.receiver.is_none()}
}

/// 🪞️ The document a derived view evaluates while a continuous tool transaction is open (design §13, F-7): the committed
/// snapshot with every open press's and typing run's provisional leaves (the framework's, as of admission) folded in —
/// `None` while nothing is open, so the committed snapshot is read as is. The overlay is retired cold, never dropped.
fn generation2d_provisional_snapshot(committed: &Generation2dSnapshot, provisional: &[semio_framework_value::DslValue]) -> Option<Generation2dSnapshot> {
    if provisional.is_empty() {
        return None;
    }
    let mut overlay = committed.clone();
    for value in provisional {
        if let Ok(leaf) = <Generation2dMutation as semio_framework_value::FromValue>::from_value(value.clone()) {
            generation2d_fold_provisional(&mut overlay, leaf);
        }
    }
    Some(overlay)
}

/// 🫥️ Folds one provisional leaf into a preview overlay and retires it. A leaf the overlay refuses paints nothing — a
/// preview shows only what applies — and its refusal is never lost: the commit that lands the same leaf on the committed
/// document reports it as the history row's outcome.
fn generation2d_fold_provisional(overlay: &mut Generation2dSnapshot, leaf: Generation2dMutation) {
    let _refused_paints_nothing = semio_s_artifact_procedural_2d_mutations::central_apply::apply_generation2d_mutation(overlay, &leaf);
    leaf.retire_cold();
}

/// 🔢️ The digest a preview target is evaluated and owed under: the committed document's target digest folded with the
/// continuous-tool overlay generation, so every tick, release and zero-trace abort of an open press owes ONE fresh
/// evaluation and an evaluated overlay owes nothing more.
pub(crate) fn generation2d_preview_digest(target: PreviewEvalTarget, committed: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, provisional_generation: u64) -> u64 {
    flow_eval_tick::target_digest(target, committed, cfg) ^ provisional_generation.wrapping_mul(0x9e37_79b9_7f4a_7c15)
}

/// ⏱️ One `previewEval` hop against `owner`: the tick or the answer fold `command` names, over the
/// session of the target its window kind evaluates. `doc` is what the hop evaluates (the committed document with any open
/// tool transaction's provisional leaves folded in), `committed` and `provisional_generation` what it is recorded under.
/// Answers the hop's emit and the app transient the generate preview's changed evaluation owes.
pub fn generation2d_flow_eval_hop(
    owner: &mut Generation2dInstanceOperationOwner,
    command: &Generation2dCommand,
    doc: &ArtifactView<'_, Generation2dSnapshot>,
    committed: &ArtifactView<'_, Generation2dSnapshot>,
    cfg: &ConfigView<'_, Generation2dConfig>,
    provisional_generation: u64,
) -> Result<(Emit<Generation2dMutation, Generation2dConfigMutation, NoDraftMutation>, Vec<Generation2dTransientMutation>), Fault> {
    let (mut sessions, link) = owner.parts()?;
    let result = match command {
        Generation2dCommand::FlowEvalTick(payload) => {
            let target = generation2d_preview_target(&payload.window_kind_id).ok_or_else(|| Fault::from("generation2d-flow-eval-window-kind-unknown"))?;
            let (emit, publication) = flow_eval_tick::evaluate(&payload.window_id, &payload.window_kind_id, target, doc, cfg, sessions.get_mut(target))?;
            link.note_evaluated(target, generation2d_preview_digest(target, committed, cfg, provisional_generation));
            let transient = match (target, publication) {
                (PreviewEvalTarget::Generation, semio_framework_os_flow::FlowEvalPublication::Changed(preview_text)) => vec![SetGenerationPreview { preview_text }.into()],
                _ => Vec::new(),
            };
            Ok((emit, transient))
        }
        Generation2dCommand::FlowEvalResolve(_) => unreachable!(),
        _ => Err(Fault::from("generation2d-flow-eval-route-rejected")),
    };
    link.wake();
    result
}

/// 🧬️ The generation commands against the instance's retained owner: the one-shot reduce, and every
/// attached preview owed a fresh evaluation carried on the gesture's own emit — the generate preview
/// is evaluated by the `previewEval` run, never by a tick loop inside this command.
pub(super) struct Generation2dGenerationCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
}

impl ArtifactCommandWork<EditorApp<Generation2dPlayApp>> for Generation2dGenerationCommandWork {
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
        GENERATION2D_RETAINED_CAPACITY.rows_for_items(snapshot.host_snapshot.widgets.len().checked_add(2)?)
    }

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<EditorApp<Generation2dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation2dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("generation2d-generation-command-work-repeated"));
        }
        self.consumed = true;
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: None };
        let result = prepare_command(input.command, &doc, &cfg).ok_or_else(|| Fault::from("generation2d-generation-command-route-rejected"))?;
        let mut emit = result.emit;
        if result.publishes_preview {
            let windows = generation2d_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
            let servable = semio_s_artifact_procedural_2d_preview_eval::may_rearm(&input.snapshot.host_snapshot);
            self.instance_owner.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| owner.owe_attached_previews_carrying(&windows, servable, &mut emit, input.operation.retained))?;
        }
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub struct Generation2dBoundedCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation2dBoundedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
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
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
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
        ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setShowMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "generate", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasWheel", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "removeGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "renameGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "updateGenerationValues", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "selectGeneration", lanes: &[ArtifactToolPublicationLane::Config] },
        // ⏱️ The tick's one store lane is the app transient the generate preview renders; its extension
        // invocations are no store lane at all.
        ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[ArtifactToolPublicationLane::Transient] },
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
