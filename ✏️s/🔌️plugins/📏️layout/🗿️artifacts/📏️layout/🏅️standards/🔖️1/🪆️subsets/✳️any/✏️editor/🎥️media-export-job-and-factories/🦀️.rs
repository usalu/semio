//! 🎥️ Media export job and factories of the layout play app: the media export job with its interactive job and reserved job implementations, the media export job factory and the owned tool job factory implementations of both export factories. See [`super`].

use super::*;

pub struct LayoutMediaExportJob {
 pub(super) inner:Option<LayoutExportJob>,pub(super) completion:Option<ArtifactMediaExportCompletion>,pub(super) active:Option<Box<dyn store::ErasedSnapshotRetirement>>,
 metadata:std::mem::ManuallyDrop<Option<([Vec<u8>;2],Option<(String,String)>)>>,metadata_retirement:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<([Vec<u8>;2],Option<(String,String)>)>>>,
 pending_result:std::mem::ManuallyDrop<Option<Result<ArtifactMediaExportResult,semio_framework::MediaError>>>,result_published:bool,
 pub(super) slot:LayoutPublicationSlot,pub(super) completed:bool,
}

impl LayoutMediaExportJob{
 pub fn admit_original(request:&mut Option<semio_framework_plugin::ArtifactMediaExportJobRequest<EditorApp<LayoutPlayApp>>>,body:&mut Option<Box<dyn ArtifactReservedJob+Send>>,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
  let _=cx.original_source_parent()?;if cx.should_yield()||cx.is_cancelled(){return Ok(false)}let grant=cx.retained_grant();
  if body.is_none(){let input=request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"layout producer lost original request"))?;let page=input.snapshot.pages.first().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"layout media requires one actual page"))?;
   if !bounded_id(&page.id)||!bounded_id(&input.parent_document_id){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"layout media requires its exact bounded original authority").into())}let bytes=size_of::<Self>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes||grant.maximum_capacity_bytes<bytes{return Ok(false)}
   let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(std::alloc::Layout::new::<Self>())}.cast::<Self>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"layout media original body allocation refused"))?;
   let owner=Self{inner:None,completion:None,active:None,metadata:std::mem::ManuallyDrop::new(Some((std::array::from_fn(|_|Vec::new()),None))),metadata_retirement:std::mem::ManuallyDrop::new(None),pending_result:std::mem::ManuallyDrop::new(None),result_published:false,slot:LayoutPublicationSlot::default(),completed:false};*body=Some(unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())});cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
  }
  let owner=(&mut**body.as_mut().unwrap()as &mut dyn std::any::Any).downcast_mut::<Self>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"layout media requires its exact concrete body"))?;
  if owner.inner.is_some()&&request.is_none(){return Ok(true)}if owner.inner.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout original request cannot overwrite its prepared body").into())}
  let input=request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"layout media lost original source before admission"))?;let page=input.snapshot.pages.first().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"layout captured page changed"))?;let fields=&mut owner.metadata.as_mut().unwrap().0;
  for index in 0..2{let bytes=if index==0{page.id.len()}else{64};let output=&mut fields[index];if output.capacity()<bytes{if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(false)}output.try_reserve_exact(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"layout original authority allocation refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
   if output.len()<bytes{let count=(bytes-output.len()).min(grant.maximum_copy_bytes);if grant.maximum_items==0||grant.maximum_depth==0||count==0{return Ok(false)}let start=output.len();if index==0{output.extend_from_slice(&page.id.as_bytes()[start..start+count])}else{const HEX:&[u8;16]=b"0123456789abcdef";for offset in start..start+count{let value=input.canonical_base_revision[offset/2];output.push(HEX[usize::from(if offset%2==0{value>>4}else{value&15})])}}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  }
  let bytes=size_of::<LayoutExportJob>()+size_of::<semio_framework_plugin::ArtifactMediaExportJobRequest<EditorApp<LayoutPlayApp>>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
  let input=request.take().unwrap();let fields=&mut owner.metadata.as_mut().unwrap().0;let page_id=String::from_utf8(std::mem::take(&mut fields[0])).expect("layout retained page bytes preserve original UTF-8");let revision=String::from_utf8(std::mem::take(&mut fields[1])).expect("layout original revision is hexadecimal");
  let export=LayoutExportRequest{kind:LayoutExportKind::Svg,page_id:Some(page_id),snapshot:input.snapshot,preflight_json:None,parent_document_id:input.parent_document_id,canonical_base_revision_hex:revision};
  owner.inner=Some(LayoutExportJob::from_admitted(input.operation,input.source_identity,export,input.output_chunks).with_snapshot_close_lease(input.snapshot_close).with_media_output_credit(input.output_credit));owner.completion=Some(input.completion);owner.metadata.as_mut().unwrap().1=Some((input.tool_id,input.port));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
 }
 pub(super) fn close_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
  let one=semio_framework_value::RetirementDemand{depth:1,..Default::default()};
  if let Some(inner)=self.inner.as_ref(){return if inner.terminal_is_empty(){Ok(one)}else{Ok(semio_framework_value::RetirementDemand{copy_bytes:inner.next_close_copy_byte_demand()?,capacity_bytes:inner.next_close_capacity_byte_demand(body)?,release_bytes:inner.next_close_release_byte_demand()?,depth:inner.next_close_depth_demand()?.max(1)})}}
  if let Some(active)=self.active.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body)}
  if self.pending_result.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_result)}
  if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion)}
  if !self.slot.terminal_is_empty(){return self.slot.retirement_demands()}
  if let Some(owner)=self.metadata_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(one)}else{owner.selected_demand(body)}}
  Ok(if self.metadata.is_some(){one}else{Default::default()})
 }
 fn close_turn(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
  let item=semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()};
  if self.inner.as_ref().is_some_and(InteractiveJob::terminal_is_empty){self.inner.take();return Ok(semio_framework_value::RetainedCloneStep::Progress(item))}
  if self.active.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active,grant)}
  if self.pending_result.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_result,&mut self.active,grant)}
  if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.active,grant)}
  if !self.slot.terminal_is_empty(){return self.slot.close_step(grant)}
  if let Some(owner)=self.metadata_retirement.as_mut(){if owner.terminal_is_empty(){self.metadata_retirement.take();return Ok(semio_framework_value::RetainedCloneStep::Progress(item))}return owner.step(grant)}
  if let Some(metadata)=self.metadata.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(metadata){Ok(owner)=>*self.metadata_retirement=Some(owner),Err((error,metadata))=>{*self.metadata=Some(metadata);return Err(error)}}return Ok(semio_framework_value::RetainedCloneStep::Progress(item))}
  Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default()))
 }
}

impl InteractiveJob for LayoutMediaExportJob{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->LayoutExportTurn<'a>{
  if !self.slot.settle(cx)?{return Ok(None)}if self.slot.publication.is_some(){return self.slot.advance(cx)}
  if self.pending_result.is_some()||self.result_published{self.completed=self.completion.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout media lost its original completion receiver"))?.complete_original(&mut self.pending_result,&mut self.result_published,cx)?;return Ok(None)}
  let inner=self.inner.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout media cannot drive an unadmitted body"))?;
  if inner.commit_ready()&&!self.completed{
   if let Some(credit)=inner.media_output_credit.as_ref(){credit.credit(LAYOUT_MEDIA_EXPORT_SCHEMA.len())?}
   let chunks=inner.output_chunks.take().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout media lost supplied output roots"))?;
   let media=ArtifactMediaExportResult::structured(MediaType{class:MediaClass::TwoD,form:MediaForm::Vector},LAYOUT_MEDIA_EXPORT_SCHEMA,"application/json",chunks)?;
   *self.pending_result=Some(Ok(media));return Ok(None);
  }inner.step(cx)
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,semio_framework_value::ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_ if self.slot.delivered=>self.slot.borrow_outcome(descriptor),_=>self.inner.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout outcome lost its original body"))?.borrow_outcome(descriptor)}}
 fn begin_close(&mut self){if let Some(inner)=self.inner.as_mut(){inner.begin_close()}}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
  self.begin_close();if let Some(inner)=self.inner.as_mut(){if !inner.terminal_is_empty(){return inner.close_step(cx).map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()})}}
  let demand=self.close_demand(cx.retained_grant().maximum_copy_bytes)?;let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_turn(grant))?;Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
 }
 fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand(0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.inner.is_none()&&self.completion.is_none()&&self.active.is_none()&&self.slot.terminal_is_empty()&&self.metadata.is_none()&&self.metadata_retirement.is_none()&&self.pending_result.is_none()}
}

impl ArtifactReservedJob for LayoutMediaExportJob {}

pub struct LayoutMediaExportJobFactory {
    pub(super) keys: [ToolFactoryKey; 1],
}

impl LayoutMediaExportJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: [ToolFactoryKey::new(controller_id, LAYOUT_MEDIA_EXPORT_TOOL_ID)] }
    }
}

impl ToolJobFactory for LayoutMediaExportJobFactory {
    type Payload = ArtifactReservedToolJob;
    type Job = ArtifactReservedToolJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(MAX_LAYOUT_EXPORT_RAW_BYTES, MAX_LAYOUT_EXPORT_DECODED_ITEMS, 1, MAX_LAYOUT_EXPORT_OUTPUT_BYTES, 2_000, 64, 1)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(payload)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for LayoutMediaExportJobFactory {
    type Owner = EditorApp<LayoutPlayApp>;
    const TOOL_IDS: &'static [&'static str] = &[LAYOUT_MEDIA_EXPORT_TOOL_ID];
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: LAYOUT_MEDIA_EXPORT_TOOL_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

impl LayoutExportJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: LAYOUT_EXPORT_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for LayoutExportJobFactory {
    type Payload = LayoutExportToolPayload;
    type Job = LayoutExportToolJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LAYOUT_EXPORT_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES, MAX_LAYOUT_EXPORT_DECODED_ITEMS, 1, MAX_LAYOUT_EXPORT_OUTPUT_BYTES, 2_000, 64, 1)
    }

    fn create_job(&mut self, operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        let kind = payload.request.kind;
        let name = output_name(&payload.request);
        let inner = LayoutExportJob::from_admitted(operation,(operation.operation,operation.generation),payload.request,payload.output_chunks);
        Ok(LayoutExportToolJob {
            inner: Some(inner),
            rejected_download:None,
            pending_download: None,
            slot: LayoutPublicationSlot::default(),
            pending_operation: None,
            pending_request: None,
            pending_output_chunks: None,
            completion: payload.completion,
            kind,
            name,
            raw_input: None,
            raw_bytes: Vec::new(),
            raw_page_cursor: 0,
            raw_validated: true,
            completed: false,
        })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: Operation,
        payload: Self::Payload,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        if checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("layout export retained ingress does not accept an unvalidated checkpoint owner"), input, checkpoint));
        }
        let declared_bytes = input.declared_bytes();
        if declared_bytes > MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES {
            return Err((ToolJobFactoryError::new("layout export retained ingress exceeds its command byte envelope"), input, None));
        }
        let kind = payload.request.kind;
        let mut job = LayoutExportToolJob {
            inner: None,
            rejected_download:None,
            pending_download: None,
            slot: LayoutPublicationSlot::default(),
            pending_operation: Some(operation),
            pending_request: Some(payload.request),
            pending_output_chunks: Some(payload.output_chunks),
            completion: payload.completion,
            kind,
            name: String::new(),
            raw_input: None,
            raw_bytes: Vec::new(),
            raw_page_cursor: 0,
            raw_validated: false,
            completed: false,
        };
        if job.raw_bytes.try_reserve_exact(declared_bytes).is_err() {
            return Err((ToolJobFactoryError::new("layout export retained decoder capacity was not admitted"), input, None));
        }
        job.raw_input = Some(input);
        Ok(job)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for LayoutExportJobFactory {
    type Owner = EditorApp<LayoutPlayApp>;
    const TOOL_IDS: &'static [&'static str] = LAYOUT_EXPORT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "exportPng", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportSvg", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportPdf", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportPackage", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}
