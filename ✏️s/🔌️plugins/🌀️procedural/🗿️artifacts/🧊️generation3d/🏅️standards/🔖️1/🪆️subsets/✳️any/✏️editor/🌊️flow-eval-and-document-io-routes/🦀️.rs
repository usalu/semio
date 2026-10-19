//! 🌊️ Flow eval and document IO routes: the flow eval job factory and the document IO contract, work, job factory and retirement. See [`super`].

use super::*;

//#region 🔖️Generation3dPlayApp
//#region 🧵️RetainedCommands
//#region ⏱️FlowEvalRoute
/// ⏱️ The preview chain's OWN route: its own tool ids, its own factory and its own execution
/// contract, because what it carries is nothing like a gesture. `flowTessellateResolve` folds one
/// `tessellate` envelope — up to one whole [`MESH_PACK_CHUNK_BASE64_CHARS`] mesh-body chunk — and
/// under the 8 KiB gesture quota that envelope had to be cut into 4 KiB slices, one per
/// `flowEvalTick` round trip. Five of the eight bundled examples therefore never painted: at
/// seconds per round trip a ten-chunk body is minutes of streaming with `phase: idle` on the
/// surface the whole time (`📓️preview-mesh-delivery-2026-09-12.md`).
pub fn generation3d_flow_eval_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_FLOW_EVAL_RAW_BYTES, GENERATION3D_RETAINED_DECODED_ITEMS, GENERATION3D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

pub struct Generation3dFlowEvalJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation3dFlowEvalJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_FLOW_EVAL_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dFlowEvalJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_FLOW_EVAL_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_flow_eval_contract()
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
        if input.declared_bytes() > GENERATION3D_FLOW_EVAL_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d flow-eval command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dFlowEvalJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_FLOW_EVAL_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowEvalResolve", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowTessellateResolve", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowEvalRelease", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "flowTessellateCancelResolve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

pub(super) struct Generation3dFlowEvalJobFactoryProofs;

impl Generation3dFlowEvalJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        artifact_schema: "generation.3d",
        factory: "Generation3dFlowEvalJobFactory",
        factory_type: Generation3dFlowEvalJobFactory,
        contract: generation3d_flow_eval_contract(),
        tools: ["flowEvalTick", "flowEvalResolve", "flowTessellateResolve", "flowEvalRelease", "flowTessellateCancelResolve"]
    }
}
//#endregion ⏱️FlowEvalRoute

//#region 📄️DocumentIoRoute
pub const GENERATION3D_DOCUMENT_SOURCE_ALLOCATION_BYTES: usize = 1_048_576;

pub const GENERATION3D_DOCUMENT_OWNERSHIP_SOURCE_MULTIPLES: usize = 32;

pub const GENERATION3D_DOCUMENT_SCAFFOLD_BYTES: usize = 16_384;

const GENERATION3D_DOCUMENT_DOWNLOAD_BYTES: usize = 33_554_432;

/// 📄️ The user's import/export route. Its three tools are the whole UI surface of this artifact's
/// nine `🚪️io` leaves, which ticket 26/09/09/PROCEDURAL-3D-END-TO-END found round-trip tested and
/// reachable by nothing at all (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1).
pub fn generation3d_document_io_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_DOCUMENT_IO_RAW_BYTES, GENERATION3D_RETAINED_DECODED_ITEMS, GENERATION3D_RETAINED_WORK_ITEMS as u64, GENERATION3D_DOCUMENT_DOWNLOAD_BYTES, 7_500)
}

/// 📄️ Runs one import/export hop against the app instance's RETAINED owner — the export reads its
/// retained preview, and a completed import re-arms every attached preview through the same
/// per-window latch an example switch uses.
enum Generation3dDocumentRetirement {
    Projection(semio_framework_dsl_record::native_encoding::RetainedFieldProjection),
    Projected(semio_framework_dsl_record::FieldValue),
    Writer(semio_framework_dsl_record::RetainedRecordWriter),
    Text(store::semio_format::RetainedTextEnvelope),
    Envelope(semio_s_artifact_procedural_3d_io::document_io::Generation3dDocumentEnvelope),
    Export(semio_s_artifact_procedural_3d_io::Generation3dDocumentExport),
}

impl semio_framework_value::retirement::RetireOwned for Generation3dDocumentRetirement {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{match self{Self::Projection(value)=>value.retirement(),Self::Projected(value)=>value.retirement(),Self::Writer(value)=>value.retirement(),Self::Text(value)=>value.retirement(),Self::Envelope(value)=>value.retirement(),Self::Export(value)=>value.retirement()}}
    fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Projection(value)=>value.retirement_birth_bytes(),Self::Projected(value)=>value.retirement_birth_bytes(),Self::Writer(value)=>value.retirement_birth_bytes(),Self::Text(value)=>value.retirement_birth_bytes(),Self::Envelope(value)=>value.retirement_birth_bytes(),Self::Export(value)=>value.retirement_birth_bytes()}}
    fn controlled_retirement_supported()->bool{true}
}

pub struct Generation3dDocumentIoWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
    pub(super) closing: bool,
    source_identity: Option<usize>,
    source_demand: Option<usize>,
    expand_admission: bool,
    pub(super) projection: Option<semio_framework_dsl_record::native_encoding::RetainedFieldProjection>,
    projected: Option<semio_framework_dsl_record::FieldValue>,
    writer: Option<semio_framework_dsl_record::RetainedRecordWriter>,
    pub(super) text: Option<store::semio_format::RetainedTextEnvelope>,
    pub(super) envelope: Option<semio_s_artifact_procedural_3d_io::document_io::Generation3dDocumentEnvelope>,
    candidate: Option<semio_s_artifact_procedural_3d_io::Generation3dDocumentExport>,
    pub(super) retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<Generation3dDocumentRetirement>>,
    encoding: Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,
    pub(super) phase: u8,
}

impl Generation3dDocumentIoWork {
    pub(super) fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, consumed: false, closing:false, source_identity:None, source_demand:None, expand_admission:false, projection:None, projected:None, writer:None, text:None, envelope:None, candidate:None, retirement:None, encoding:None, phase:0 }
    }

    fn park_retirement(&mut self,value:Generation3dDocumentRetirement)->Result<(),semio_framework_value::ValueError>{
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(value){
            Ok(owner)=>{self.retirement=Some(owner);Ok(())},
            Err((error,value))=>{match value{Generation3dDocumentRetirement::Projection(value)=>self.projection=Some(value),Generation3dDocumentRetirement::Projected(value)=>self.projected=Some(value),Generation3dDocumentRetirement::Writer(value)=>self.writer=Some(value),Generation3dDocumentRetirement::Text(value)=>self.text=Some(value),Generation3dDocumentRetirement::Envelope(value)=>self.envelope=Some(value),Generation3dDocumentRetirement::Export(value)=>self.candidate=Some(value)}Err(error)},
        }
    }
    fn owns_retirement_source(&self)->bool{self.projection.is_some()||self.projected.is_some()||self.writer.is_some()||self.text.is_some()||self.envelope.is_some()||self.candidate.is_some()}
    pub(super) fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::retained_clone::RetainedCloneGrant,semio_framework_value::ValueError>{
        use semio_framework_value::retained_clone::RetainedCloneGrant;
        if let Some(owner)=self.retirement.as_ref(){let d=owner.selected_demand(copy)?;return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth})}
        if !self.owns_retirement_source(){return Ok(RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0})}
        let frontier=semio_framework_value_persistent::list::PagedList::<Box<dyn semio_framework_value::retirement::RetirementCursor>,{usize::MAX}>::default();
        Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:frontier.next_capacity_allocation_bytes(1).map_err(semio_framework_value::ValueError::from)?.unwrap_or(0),maximum_release_bytes:0,maximum_depth:1})
    }

    fn text_step(&mut self,input:&ArtifactCommandInputs<'_,EditorApp<Generation3dPlayApp>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>,Fault>{
        use semio_framework_value::NativeEncodeControl;
        let identity=input.snapshot as*const Generation3dSnapshot as usize;
        if self.source_identity.is_some_and(|source|source!=identity){return Err(Fault::from("generation3d-document-source-changed"))}
        if cx.is_cancelled(){return Err(Fault::from("generation3d-document-canceled"))}
        let stage=match self.phase{0|1 if self.source_demand.is_none()=>"document-measure",0|1=>"document-project",2=>"document-physical",3=>"document-envelope",4=>"document-base64",_=>"document-retirement"};
        cx.set_stage(stage);
        while !cx.should_yield(){
            let mut accepted=|_|!cx.is_cancelled();
            let mut control=match self.encoding.take(){Some(receipt)=>NativeEncodeControl::resume(receipt,&mut accepted),None=>{self.expand_admission=self.phase==0;Ok(NativeEncodeControl::new(GENERATION3D_DOCUMENT_SCAFFOLD_BYTES,&mut accepted))}}.map_err(|error|Fault::from(error.to_string()))?;
            if self.retirement.is_some(){
                let result=(||->Result<(),semio_framework_value::ValueError>{
                    control.checkpoint()?;
                    let mut grant=self.retirement_demands(8)?;grant.maximum_copy_bytes=8;
                    control.charge(grant.maximum_capacity_bytes)?;
                    let step=self.retirement.as_mut().unwrap().step(grant)?;
                    if matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){self.retirement.take();}
                    Ok(())
                })();
                self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);
                result.map_err(|error|Fault::from(error.to_string()))?;cx.consume_fuel(1);continue;
            }
            if self.phase==1&&self.source_demand.is_none(){
                let measured=self.projection.as_mut().unwrap().measure_step(input.snapshot,1,GENERATION3D_DOCUMENT_SOURCE_ALLOCATION_BYTES,&mut control);
                match measured{
                    Ok(Some(bytes))=>{
                        self.source_demand=Some(bytes);
                        if self.expand_admission{control=match control.admit_capacity(bytes,GENERATION3D_DOCUMENT_OWNERSHIP_SOURCE_MULTIPLES,GENERATION3D_DOCUMENT_SCAFFOLD_BYTES){Ok(control)=>control,Err((control,error))=>{self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);return Err(Fault::from(error.to_string()))}};self.expand_admission=false;}
                    },
                    Ok(None)=>{},
                    Err(error)=>{self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);return Err(Fault::from(error.to_string()))},
                }
                self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);cx.consume_fuel(1);continue;
            }
            let result=(||->Result<(),semio_framework_value::ValueError>{
                match self.phase{
                    0=>{self.source_identity=Some(identity);self.projection=Some(semio_framework_dsl_record::native_encoding::RetainedFieldProjection::new(input.snapshot));self.phase=1;},
                    1=>if let Some(value)=self.projection.as_mut().unwrap().step(input.snapshot,1,&mut control)?{
                        self.projected=Some(value);
                        let spec=semio_s_artifact_procedural_3d_io_text::snapshot::generation3d_document_spec_controlled(&mut control)?;
                        if !matches!(self.projected,Some(semio_framework_dsl_record::FieldValue::Record(_))){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"generation3d projection root is not a record"))}
                        let Some(semio_framework_dsl_record::FieldValue::Record(record))=self.projected.take()else{unreachable!()};
                        self.writer=Some(semio_framework_dsl_record::RetainedRecordWriter::new(record,spec,semio_framework_dsl_record::JoinMode::Document,GENERATION3D_DOCUMENT_DOWNLOAD_BYTES));
                        let retired=self.projection.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Projection(retired))?;self.phase=2;
                    },
                    2=>if let Some(body)=self.writer.as_mut().unwrap().step(1,&mut control)?{
                        self.text=Some(store::semio_format::RetainedTextEnvelope::new("procedural.generation3d".into(),store::semio_format::Component::Dsl,1,body,cx.retained_grant()));
                        let retired=self.writer.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Writer(retired))?;self.phase=3;
                    },
                    3=>if let Some(text)=self.text.as_mut().unwrap().step(1,8,&mut control)?{
                        let row=semio_s_artifact_procedural_3d_io::document_io::EXPORT_FORMATS.iter().find(|row|row.id=="txt").unwrap();
                        self.envelope=Some(semio_s_artifact_procedural_3d_io::document_io::Generation3dDocumentEnvelope::new(row,text.into_bytes()));
                        let retired=self.text.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Text(retired))?;self.phase=4;
                    },
                    4=>if let Some(export)=self.envelope.as_mut().unwrap().step(1,&mut control).map_err(|error|semio_framework_value::ValueError::new(error.kind,error.message))?{
                        self.candidate=Some(export);let retired=self.envelope.take().unwrap();self.park_retirement(Generation3dDocumentRetirement::Envelope(retired))?;self.phase=5;
                    },
                    _=>{},
                }
                Ok(())
            })();
            self.encoding=Some(control.pause().map_err(|error|Fault::from(error.to_string()))?);
            result.map_err(|error|Fault::from(error.to_string()))?;
            cx.consume_fuel(1);
            if self.phase==5&&self.retirement.is_none(){
                let export=self.candidate.take().ok_or_else(||Fault::from("generation3d-document-output-missing"))?;
                self.consumed=true;
                return Ok(ArtifactCommandWorkStep::Complete(Emit::effect(Effect::DownloadMediaExport{filename:export.filename,mime_type:export.mime_type,data:export.data,encoding:export.encoding})));
            }
        }
        Ok(ArtifactCommandWorkStep::Progress{stage,preview:b"{}"})
    }
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dDocumentIoWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 🧮️ An import replaces the whole graph: it removes every widget the document holds and plants
    /// the three-widget import fixture, so its extent is the document's own width plus that fixture —
    /// exactly the accounting `Generation3dPreviewCommandWork` does for an example switch. A request
    /// and an export author no durable row at all and declare the one-item extent.
    fn extent(
        &self,
        command: &Generation3dCommand,
        snapshot: &Generation3dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        if matches!(command, Generation3dCommand::ImportDocument(_)) {
            return GENERATION3D_RETAINED_CAPACITY.rows_for_items(snapshot.host_snapshot.widgets.len().checked_add(3)?);
        }
        generation3d_bounded_extent(command, snapshot, interaction)
    }

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<EditorApp<Generation3dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed || self.closing { return Err(Fault::from("generation3d-document-io-work-repeated")); }
        if matches!(input.command,Generation3dCommand::ExportDocument(payload) if payload.format=="txt"){return self.text_step(input,cx)}
        self.consumed = true;
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: None };
        let emit = match input.command {
            Generation3dCommand::ImportDocumentRequest(payload) => import_document_request::emit(payload, &doc.snapshot.host_snapshot)?,
            Generation3dCommand::ExportDocument(payload) => {
                let meshes = if payload.format == "txt" { None } else {
                    self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| owner.with_session(|session| export_document::retained_meshes(&doc, &cfg, session, payload.widget_id.as_deref())))??
                };
                export_document::emit(payload, &doc, meshes.as_deref())?
            }
            Generation3dCommand::ImportDocument(payload) => {
                let windows = generation3d_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
                let servable = flow_eval_tick::may_rearm(&input.snapshot.host_snapshot);
                self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
                    let mut emit = import_document::emit(payload, &doc, &cfg)?;
                    if !emit.artifact_mutations.is_empty() {
                        owner.owe_attached_previews_carrying(&windows, servable, &mut emit, input.operation.retained)?;
                    }
                    Ok(emit)
                })?
            }
            _ => return Err(Fault::from("generation3d-document-io-route-rejected")),
        };
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self){self.closing=true;}

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
        if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand={let d=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_value::RetirementDemand{copy_bytes:d.maximum_copy_bytes,capacity_bytes:d.maximum_capacity_bytes,release_bytes:d.maximum_release_bytes,depth:d.maximum_depth}};
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{(
            ||->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        if !self.closing{return Ok(InteractiveJobCloseStep::Blocked)}
        if grant.maximum_items==0{return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()})}
        if self.retirement.is_none(){
            let source=if let Some(value)=self.projection.take(){Some(Generation3dDocumentRetirement::Projection(value))}
            else if let Some(value)=self.projected.take(){Some(Generation3dDocumentRetirement::Projected(value))}
            else if let Some(value)=self.writer.take(){Some(Generation3dDocumentRetirement::Writer(value))}
            else if let Some(value)=self.text.take(){Some(Generation3dDocumentRetirement::Text(value))}
            else if let Some(value)=self.envelope.take(){Some(Generation3dDocumentRetirement::Envelope(value))}
            else{self.candidate.take().map(Generation3dDocumentRetirement::Export)};
            if let Some(value)=source{if let Err(error)=self.park_retirement(value){return Err(error)}}
            else{self.encoding.take();return Ok(InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()})}
        }
        match self.retirement.as_mut().unwrap().step(grant){
            Ok(RetainedCloneStep::Complete(progress))=>{self.retirement.take();Ok(InteractiveJobCloseStep::Pending{progress})},
            Ok(RetainedCloneStep::Progress(progress))=>Ok(InteractiveJobCloseStep::Pending{progress}),
            Err(error)=>Err(error),
        }
    })().map(|step|semio_framework_value::RetainedCloneStep::Progress(step.progress()))})?;
        Ok(if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(0).map(|grant|grant.maximum_copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(copy).map(|grant|grant.maximum_capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(0).map(|grant|grant.maximum_release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.retirement_demands(0).map(|grant|grant.maximum_depth)}

    fn terminal_is_empty(&self)->bool{self.closing&&self.projection.is_none()&&self.projected.is_none()&&self.writer.is_none()&&self.text.is_none()&&self.envelope.is_none()&&self.candidate.is_none()&&self.retirement.is_none()&&self.encoding.is_none()}
}

pub struct Generation3dDocumentIoJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation3dDocumentIoJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_DOCUMENT_IO_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dDocumentIoJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_DOCUMENT_IO_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_document_io_contract()
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
        if input.declared_bytes() > GENERATION3D_DOCUMENT_IO_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d document-io command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dDocumentIoJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_DOCUMENT_IO_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 📄️ `importDocumentRequest` and `exportDocument` author no store row at all — one asks the
    /// shell for a file, the other hands it a download — so both are `HostOnly`. `importDocument`
    /// replaces the graph on the Artifact lane and carries the fresh fixture's camera on the Config
    /// lane, exactly as `setActiveExample` does.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "importDocumentRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importDocument", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "exportDocument", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

pub(super) struct Generation3dDocumentIoJobFactoryProofs;

impl Generation3dDocumentIoJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        artifact_schema: "generation.3d",
        factory: "Generation3dDocumentIoJobFactory",
        factory_type: Generation3dDocumentIoJobFactory,
        contract: generation3d_document_io_contract(),
        tools: ["importDocumentRequest", "importDocument", "exportDocument"]
    }
}
//#endregion 📄️DocumentIoRoute
//#endregion 🧵️RetainedCommands
//#endregion 🔖️Generation3dPlayApp
