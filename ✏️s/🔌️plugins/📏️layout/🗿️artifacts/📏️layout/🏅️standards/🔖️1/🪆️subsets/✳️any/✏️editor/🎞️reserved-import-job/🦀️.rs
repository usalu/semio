//! 🎞️ Layout import work borrows its original reserved media and authority until paid terminal close.
use super::*;
use semio_framework_plugin::tool_contract::ArtifactReservedWork;

//#region 🎞️ReservedImport
pub(super) const LAYOUT_IMPORT_TOOL_ID:&str="import-media";
const LAYOUT_IMPORT_PORT:&str="fields:in";
pub(super) struct LayoutImportJob{
 pub(super) mutations:Option<Vec<LayoutMutation>>,
 pub(super) active:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
 checkpoint_publication:semio_framework_job_publication::RetainedJobPublication,
 checkpoint_delivered:bool,
 pub(super) fault_publication:semio_framework_job_publication::RetainedJobPublication,
 fault_detail:Option<Vec<u8>>,
 pub(super) decoded:bool,
 pub(super) completed:bool,
 closing:bool,
 pending_completion_rejection:Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<LayoutPlayApp>>>,
 last_close_work:u64,
}
type LayoutJobTurn<'a>=Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_value::ValueError>;
impl Default for LayoutImportJob{
 fn default()->Self{Self{mutations:None,active:None,checkpoint_publication:semio_framework_job_publication::RetainedJobPublication::new(),checkpoint_delivered:false,fault_publication:semio_framework_job_publication::RetainedJobPublication::new(),fault_detail:None,decoded:false,completed:false,closing:false,pending_completion_rejection:None,last_close_work:0}}
}
impl LayoutImportJob{
 pub(super) fn fault<'a>(&'a mut self,cx:&mut StepContext<'_>,detail:&str)->LayoutJobTurn<'a>{
  if self.fault_detail.is_none(){let bytes=detail.as_bytes();self.fault_detail=Some(bytes[..bytes.len().min(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)].to_vec());}
  self.fault_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault,self.fault_detail.as_deref().unwrap_or_default(),cx)
 }
 fn decode(&mut self,request:&ArtifactReservedToolJobRequest<EditorApp<LayoutPlayApp>>)->Result<(),&'static str>{
  let Some(ArtifactReservedToolInput::Media{port,media})=request.input.as_ref()else{return Err("layout import lost its original media owner")};
  if port!=LAYOUT_IMPORT_PORT{return Err("layout import only implements fields:in")}
  let doc=ArtifactView::new(request.snapshot.as_ref(),request.history.as_ref());
  match LayoutPlayApp::import_media(LAYOUT_IMPORT_PORT,media,&doc){Ok(emit)=>{self.mutations=Some(emit.artifact_mutations);self.decoded=true;Ok(())},Err(_)=>Err("layout import media decode refused")}
 }
 fn demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
  if !self.fault_publication.terminal_is_empty(){return self.fault_publication.retirement_demands()}
  if !self.checkpoint_publication.terminal_is_empty(){return self.checkpoint_publication.retirement_demands()}
  if let Some(active)=self.active.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body)}
  if self.pending_completion_rejection.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_completion_rejection)}
  if self.mutations.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutations)}
  if self.fault_detail.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.fault_detail)}
  Ok(Default::default())
 }
 fn close_value(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
  use semio_framework_value::{RetainedCloneStep,RetainedWorkOutcome};
  self.last_close_work=0;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.demands(grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let turn:RetainedWorkOutcome<RetainedCloneStep>=if !self.fault_publication.terminal_is_empty(){self.fault_publication.close_step(grant)}
  else if !self.checkpoint_publication.terminal_is_empty(){self.checkpoint_publication.close_step(grant)}
  else if self.active.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active,grant)}
  else if self.pending_completion_rejection.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_completion_rejection,&mut self.active,grant)}
  else if self.mutations.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutations,&mut self.active,grant)}
  else if self.fault_detail.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.fault_detail,&mut self.active,grant)}
  else{return Ok(RetainedCloneStep::Complete(Default::default()))};
  self.last_close_work=turn.work;turn.result.map(|step|if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
 /// 🪜️ Reads media and completion directly from the same retained source request.
 fn step_value<'a>(&'a mut self,request:&ArtifactReservedToolJobRequest<EditorApp<LayoutPlayApp>>,cx:&mut StepContext<'_>)->LayoutJobTurn<'a>{
  if cx.is_cancelled(){return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx)}
  if self.fault_detail.is_some(){return self.fault(cx,"")}
  if self.pending_completion_rejection.is_some(){return self.fault(cx,"layout import completion remains rejected")}
  if !self.decoded{cx.set_stage("layout-import-decode");if let Err(detail)=self.decode(request){return self.fault(cx,detail)}cx.consume_fuel(1);}
  if !self.checkpoint_delivered{let outcome=self.checkpoint_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Checkpoint{applied_progress:1},&[1],cx)?;if outcome.is_some(){self.checkpoint_delivered=true;}return Ok(outcome)}
  cx.set_stage("layout-import-publish");
  if !self.completed{let completion=&request.completion;if !completion.has_mounted_consumer(){return self.fault(cx,"layout import completion consumer is absent")}
   let mutations=self.mutations.take().unwrap_or_default();
   if let Err(rejected)=completion.complete(Ok(Emit{artifact_mutations:mutations,ui_scope:semio_framework::kernel::UiDirtyScope::Full,..Default::default()}),semio_framework_plugin::EphemeralEmit::default()){let message=rejected.fault.message.clone();self.pending_completion_rejection=Some(rejected);return self.fault(cx,&message)}
   self.completed=true;
  }
  semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None)
 }
}
impl ArtifactReservedWork<EditorApp<LayoutPlayApp>> for LayoutImportJob{
 fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<LayoutPlayApp>>,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(request,cx).map_err(semio_framework_job::OriginalTurnFailure::from))}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,semio_framework_value::ValueError>{
  match descriptor.kind(){semio_framework_job::JobOutcomeKind::Yield=>descriptor.yielded(),semio_framework_job::JobOutcomeKind::Cancelled=>descriptor.cancelled(),semio_framework_job::JobOutcomeKind::Complete=>descriptor.complete(None,None),semio_framework_job::JobOutcomeKind::CheckpointReady{..}=>self.checkpoint_publication.borrow_outcome(descriptor),semio_framework_job::JobOutcomeKind::Fault=>self.fault_publication.borrow_outcome(descriptor),semio_framework_job::JobOutcomeKind::PreviewReady=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout import publishes no preview"))}
 }
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(body)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
  self.begin_close();semio_framework_job::close_original_granted_step(cx,|grant|{let result=self.close_value(grant).map(|step|if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{semio_framework_job::InteractiveJobCloseStep::Pending{progress:step.progress()}});semio_framework_value::RetainedWorkOutcome{result,work:self.last_close_work}})
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.mutations.is_none()&&self.active.is_none()&&self.pending_completion_rejection.is_none()&&self.fault_detail.is_none()&&self.fault_publication.terminal_is_empty()&&self.checkpoint_publication.terminal_is_empty()}
}
//#endregion 🎞️ReservedImport

pub(super) fn layout_build_export_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<LayoutPlayApp>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    use semio_framework_plugin::__value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
    use semio_s_artifact_layout_layout_engine_export::{LayoutExportKind, LayoutExportRequest};
    let original = request.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original layout export request is absent"))?;
    let (kind, page_id) = match &*original.command {
        LayoutCommand::ExportPng(payload) => (LayoutExportKind::Png, payload.page_id.clone().or_else(|| Some(blueprint::config::from_snapshot(original.window_config.as_ref()).active_page_id))),
        LayoutCommand::ExportSvg(payload) => (LayoutExportKind::Svg, payload.page_id.clone().or_else(|| Some(blueprint::config::from_snapshot(original.window_config.as_ref()).active_page_id))),
        LayoutCommand::ExportPdf(payload) => (LayoutExportKind::Pdf, payload.page_id.clone()),
        LayoutCommand::ExportStep(payload) => (LayoutExportKind::Step, payload.page_id.clone().or_else(|| Some(blueprint::config::from_snapshot(original.window_config.as_ref()).active_page_id))),
        LayoutCommand::ExportIdml(_) => (LayoutExportKind::Idml, None),
        LayoutCommand::ExportPackage(_) => (LayoutExportKind::Package, None),
        _ => return Ok(false),
    };
    if original.tool_id != kind.tool_id() {
        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "layout export command does not match its exact registered tool").into());
    }
    if original.checkpoint.is_some() || original.input.as_ref().is_some_and(|input| input.declared_bytes() > MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES) {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "layout export retained ingress rejects a checkpoint or an oversized command").into());
    }
    let demand = semio_framework::action_bus::ToolJobAdmissionDemand { items: 1, copy_bytes: 0, capacity_bytes: std::mem::size_of::<LayoutExportToolJob>() + std::mem::size_of::<LayoutExportToolPayload>(), release_bytes: 0, depth: 1 };
    semio_framework::action_bus::admit_original_tool_construction(cx, output, demand, |_| {
        let mut taken = request.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original layout export request is absent"))?;
        let canonical_base_revision_hex = taken.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join("");
        drop(taken.output_chunks);
        let payload = LayoutExportToolPayload {
            request: LayoutExportRequest { kind, page_id, snapshot: taken.snapshot, preflight_json: None, parent_document_id: taken.parent_document_id, canonical_base_revision_hex },
            output_chunks: semio_framework_plugin::app::ArtifactOutputChunks::new(semio_s_artifact_layout_layout_engine_export::MAX_LAYOUT_EXPORT_OUTPUT_BYTES),
            completion: Some(taken.completion),
        };
        let job = match taken.input.take() {
            Some(input) => LayoutExportToolJob::from_wire_payload(taken.operation, payload, input).map_err(|error| ValueError::new(ValueRefusalKind::AllocationFailed, error.detail))?,
            None => LayoutExportToolJob::from_payload(taken.operation, payload),
        };
        Ok((semio_framework_plugin::ErasedToolJob::from_owned(Box::new(job)), RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: std::mem::size_of::<LayoutExportToolJob>(), ..Default::default() }))
    })
}
