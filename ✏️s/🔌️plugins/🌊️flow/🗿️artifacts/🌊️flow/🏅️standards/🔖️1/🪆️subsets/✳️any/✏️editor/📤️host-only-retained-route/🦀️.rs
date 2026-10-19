//! 📤️ Host only retained route: the host only tool ids, the host effect payload, job and job factory and the scalar command and host wire views. See [`super`].

use super::*;

//#region 🧵️HostOnlyRetainedRoutes
pub const FLOW_HOST_ONLY_TOOL_IDS: &[&str] = &["evaluate", "contextMenuAt", "openSpotlight", "replaceImage", "flowEvalTick", "flowEvalResolve"];

const FLOW_HOST_ONLY_RAW_BYTES: usize = 16_384;

pub(super) struct FlowHostEffectPayload {
    pub(super) command: FlowCommand,
    pub(super) snapshot: Arc<FlowSnapshot>,
    pub(super) config: Arc<FlowMainWindowConfig>,
    pub(super) history: Arc<semio_framework_plugin::HistoryView>,
    pub(super) children: Arc<semio_framework_plugin::ChildContentView>,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) completion: semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<FlowPlayApp>>,
}

pub fn flow_scalar_command_view(command: &FlowCommand) -> Result<store::os_pack::record::scalar_witness::ScalarRecordView<'_>, &'static str> {
    use store::os_pack::record::scalar_witness::{ScalarRecordField as Field, ScalarRecordView};
    let ordinal = FlowCommand::TOOL_JOB_IDS.iter().position(|id| *id == command.command_id()).ok_or("Flow scalar command ordinal missing")? as u64;
    let fields = match command {
        FlowCommand::Evaluate(_) | FlowCommand::OpenSpotlight(_) => [None, None, None],
        FlowCommand::ContextMenuAt(command) => [Some(Field::Text(&command.id)), None, None],
        FlowCommand::ReplaceImage(command) => [Some(Field::Text(&command.id)), None, None],
        FlowCommand::FlowEvalTick(command) => [Some(Field::Text(&command.window_id)), Some(Field::Text(&command.window_kind_id)), None],
        FlowCommand::FlowEvalResolve(command) => [Some(Field::Text(&command.window_id)), Some(Field::U64(command.node_hash)), command.output_json.as_deref().map(Field::Text)],
        _ => return Err("Flow command does not have an admitted scalar record witness"),
    };
    Ok(ScalarRecordView { ordinal, fields })
}

fn flow_host_wire_view(payload: &FlowHostEffectPayload) -> Result<store::os_pack::record::scalar_witness::ScalarRecordView<'_>, &'static str> {
    flow_scalar_command_view(&payload.command)
}

struct FlowHostEffectJob {
    pub(super) payload: Option<Arc<FlowHostEffectPayload>>,
    payload_close: Option<semio_framework_value::retirement::arc_custody::SharedControlledRetirement<FlowHostEffectPayload>>,
    decoder_close: Option<semio_framework_value::retirement::arc_custody::SharedControlledRetirement<FlowHostEffectPayload>>,
    completion_refusal: Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_plugin::ArtifactToolCompletionRejection<semio_framework_plugin::EditorApp<FlowPlayApp>>>>,
    pub(super) input: Option<semio_framework_action_bus::RetainedToolWireInput>,
    decoder: Option<store::os_pack::record::scalar_witness::ScalarRecordWireWitness<FlowHostEffectPayload>>,
    pub(super) page: usize,
    byte: usize,
    validated: bool,
    pub(super) completed: bool,
    pub(super) closing: bool,
}

impl FlowHostEffectJob {
    pub(super) fn fault() -> semio_framework_value::ValueError {
        semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Flow host effect source refused its route")
    }
    pub(super) fn close_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(owner)=self.completion_refusal.as_ref(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.max(1)})}
        if let Some(input)=self.input.as_ref(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:input.next_close_copy_byte_demand()?,capacity_bytes:input.next_close_capacity_byte_demand(copy)?,release_bytes:input.next_close_release_byte_demand()?,depth:input.next_close_depth_demand()?.max(1)})}
        if self.decoder.is_some(){return Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
        if let Some(owner)=self.decoder_close.as_ref().or(self.payload_close.as_ref()){return Ok(semio_framework_value::RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.max(1)})}
        Ok(semio_framework_value::RetirementDemand{depth:usize::from(self.payload.is_some()),..Default::default()})
    }
}

impl FlowHostEffectJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_value::ValueError> {
        if context.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(context);
        }
        if context.should_yield() || context.fuel_remaining() == 0 {
            return Ok(None);
        }
        if !self.validated {
            let grant=context.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes==0{return Ok(None)}
            let Some(input) = self.input.as_ref() else { return Err(Self::fault()) };
            let page = input.page(self.page);
            if page.is_some_and(|page| self.byte == page.len()) {
                self.page += 1;
                self.byte = 0;
                context.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;
                return Ok(None);
            }
            let Some(decoder) = self.decoder.as_mut() else { return Err(Self::fault()) };
            let compared_bytes=match decoder.advance(page.and_then(|page| page.get(self.byte)).copied()) {
                Ok(store::os_pack::record::scalar_witness::ScalarRecordWireStep::Consumed { compared_bytes }) => {self.byte += 1;compared_bytes},
                Ok(store::os_pack::record::scalar_witness::ScalarRecordWireStep::Progress { compared_bytes }) => compared_bytes,
                Ok(store::os_pack::record::scalar_witness::ScalarRecordWireStep::Complete) => {self.validated = true;0},
                Err(_) => return Err(Self::fault()),
            };
            context.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:compared_bytes,..Default::default()})?;
            return Ok(None);
        }
        if !self.completed {
            let Some(payload) = self.payload.as_ref() else { return Err(Self::fault()) };
            let view = ArtifactView::with_children(payload.snapshot.as_ref(), &payload.history, (*payload.children).clone());
            let emit = payload.instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| {
                owner.with_session(|session| match &payload.command {
                    FlowCommand::Evaluate(_) => crate::flow_composed_snapshot(&payload.snapshot, &payload.children).map(|composed| evaluate::evaluate_result(&composed, &payload.config, session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN)),
                    FlowCommand::FlowEvalTick(command) => crate::flow_composed_snapshot(&payload.snapshot, &payload.children).map(|composed| flow_eval_tick::tick_result(&composed, &payload.config, session, &command.window_id, &command.window_kind_id)),
                    FlowCommand::FlowEvalResolve(command) => flow_eval_resolve::handle(command, &view, &ConfigView { snapshot: &NoConfig {}, window: None }, session),
                    FlowCommand::ContextMenuAt(_) | FlowCommand::OpenSpotlight(_) | FlowCommand::ReplaceImage(_) => Ok(Emit::default()),
                    _ => Err(Fault::from("flow-host-effect-route-mismatch")),
                })?
            });
            if let Err(original)=payload.completion.complete(emit, semio_framework_plugin::EphemeralEmit::default()) {
                self.completion_refusal=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original completion refusal defines full source retirement")));
                return Err(Self::fault());
            }
            self.completed = true;
            context.consume_fuel(1);
        }
        semio_framework_job::JobOutcomeBorrow::admit_complete(context,None,None)
    }
}

impl semio_framework_job::InteractiveJob for FlowHostEffectJob {
    fn step<'a>(&'a mut self,context:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(context).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,semio_framework_value::ValueError>{
        match descriptor.kind(){semio_framework_job::JobOutcomeKind::Yield=>descriptor.yielded(),semio_framework_job::JobOutcomeKind::Cancelled=>descriptor.cancelled(),semio_framework_job::JobOutcomeKind::Complete if self.completed=>descriptor.complete(None,None),_=>Err(Self::fault())}
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.input.as_mut() {
            input.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            use semio_framework_job::InteractiveJobCloseStep as Close;use semio_framework_value::{RetainedCloneProgress as Progress,RetainedCloneStep};
            if !self.closing||grant.maximum_items==0{return Ok(Close::Blocked)}let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return Err(error)};
            if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Close::Blocked)}
            if let Some(owner)=self.completion_refusal.as_mut(){if owner.terminal_is_empty(){self.completion_refusal=None;return Ok(Close::Pending{progress:Progress{copied_items:1,..Default::default()}})}return Ok(match owner.step(grant){Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>Close::Pending{progress},Err(error)=>return Err(error)})}
            if let Some(input)=self.input.as_mut(){let step=input.close_step(grant)?;if input.terminal_is_empty(){self.input=None;}return Ok(match step{Close::Complete{progress}=>Close::Pending{progress:Progress{copied_items:progress.copied_items.max(1),..progress}},step=>step})}
            if let Some(decoder)=self.decoder.as_mut(){decoder.begin_close();let root=decoder.take_root().expect("original decoder owns its original payload Arc");assert!(self.payload.as_ref().is_some_and(|payload|Arc::ptr_eq(payload,&root)));assert!(decoder.terminal_is_empty());self.decoder=None;self.decoder_close=Some(semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(root));return Ok(Close::Pending{progress:Progress{copied_items:1,..Default::default()}})}
            let owner=if self.decoder_close.is_some(){&mut self.decoder_close}else{&mut self.payload_close};
            if let Some(cursor)=owner.as_mut(){if cursor.terminal_is_empty(){*owner=None;return Ok(Close::Pending{progress:Progress{copied_items:1,..Default::default()}})}return Ok(match cursor.step(grant){Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>Close::Pending{progress},Err(error)=>return Err(error)})}
            if let Some(payload)=self.payload.take(){self.payload_close=Some(semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(payload));return Ok(Close::Pending{progress:Progress{copied_items:1,..Default::default()}})}
            Ok(Close::Complete{progress:Default::default()})
        })
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(copy)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.depth)}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.payload.is_none() && self.payload_close.is_none() && self.decoder_close.is_none() && self.completion_refusal.is_none() && self.input.is_none() && self.decoder.is_none()
    }
}

pub struct FlowHostEffectJobFactory {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowHostEffectJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_HOST_ONLY_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FlowHostEffectJobFactory {
    type Payload = FlowHostEffectPayload;
    type Job = FlowHostEffectJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(FLOW_HOST_ONLY_RAW_BYTES, 256, 1, 16_384, 7_500, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(FlowHostEffectJob { payload: Some(Arc::new(payload)), payload_close:None, decoder_close:None, completion_refusal:None, input: None, decoder: None, page: 0, byte: 0, validated: true, completed: false, closing: false })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() || input.declared_bytes() > FLOW_HOST_ONLY_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("Flow host-only job rejects checkpoint or oversized wire owner"), input, checkpoint));
        }
        let mut job = match self.create_job(operation, payload) {
            Ok(job) => job,
            Err(error) => return Err((error, input, None)),
        };
        job.input = Some(input);
        job.decoder = Some(store::os_pack::record::scalar_witness::ScalarRecordWireWitness::new(job.payload.as_ref().unwrap().clone(), flow_host_wire_view));
        job.validated = false;
        Ok(job)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowHostEffectJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_HOST_ONLY_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "evaluate", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "contextMenuAt", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openSpotlight", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "replaceImage", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "flowEvalResolve", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    ];
}
//#endregion 🧵️HostOnlyRetainedRoutes
