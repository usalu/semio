//! 🎞️ Layout import work borrows its original reserved media and authority until paid terminal close.
use super::*;
use semio_plugin::plugin_app_close_prelude::ArtifactReservedWork;
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneStep,RetirementPollExit,retained_clone::RetainedCloneClose};

//#region 🎞️ReservedImport
pub(super) const LAYOUT_IMPORT_TOOL_ID:&str="import-media";
const LAYOUT_IMPORT_PORT:&str="fields:in";
pub(super) struct LayoutImportJob{
 pub(super) mutations:Option<Vec<LayoutMutation>>,
 active:RetainedCloneClose,
 checkpoint_publication:semio_framework_job_publication::RetainedJobPublication,
 checkpoint_delivered:bool,
 pub(super) fault_publication:semio_framework_job_publication::RetainedJobPublication,
 fault_detail:Option<Vec<u8>>,
 pub(super) decoded:bool,
 pub(super) completed:bool,
 closing:bool,
 pending_completion_rejection:Option<semio_plugin::app::ArtifactToolCompletionRejection<EditorApp<LayoutPlayApp>>>,
}
type LayoutJobTurn<'a>=Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>;
impl Default for LayoutImportJob{
 fn default()->Self{Self{mutations:None,active:Default::default(),checkpoint_publication:semio_framework_job_publication::RetainedJobPublication::new(),checkpoint_delivered:false,fault_publication:semio_framework_job_publication::RetainedJobPublication::new(),fault_detail:None,decoded:false,completed:false,closing:false,pending_completion_rejection:None}}
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
  if !self.active.is_empty()||self.pending_completion_rejection.is_some(){return self.active.next_owner_demands::<semio_plugin::app::ArtifactToolCompletionRejection<EditorApp<LayoutPlayApp>>>(self.pending_completion_rejection.is_some(),&None,body)}
  if self.mutations.is_some(){return self.active.next_owner_demands::<Vec<LayoutMutation>>(true,&None,body)}
  if self.fault_detail.is_some(){return self.active.next_owner_demands::<Vec<u8>>(true,&None,body)}
  Ok(Default::default())
 }
 fn close_value(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.fault_publication.terminal_is_empty(){return Ok(self.fault_publication.close_step(effect).result?)}
  if !self.checkpoint_publication.terminal_is_empty(){return Ok(self.checkpoint_publication.close_step(effect).result?)}
  if !self.active.is_empty(){return RetirementPollExit::resume(self.active.step_granted(body,effect))}
  let turn=if self.pending_completion_rejection.is_some(){self.active.begin_granted(&mut self.pending_completion_rejection,effect)}
  else if self.mutations.is_some(){self.active.begin_granted(&mut self.mutations,effect)}
  else if self.fault_detail.is_some(){self.active.begin_granted(&mut self.fault_detail,effect)}
  else{return Ok(RetainedCloneStep::Complete(Default::default()))};
  Ok(turn.result?.unwrap_or(RetainedCloneStep::Progress(Default::default())))
 }
 /// 🪜️ Reads media and completion directly from the same retained source request.
 fn step_value<'a>(&'a mut self,request:&ArtifactReservedToolJobRequest<EditorApp<LayoutPlayApp>>,cx:&mut StepContext<'_>)->LayoutJobTurn<'a>{
  if cx.is_cancelled(){return Ok(semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx)?)}
  if self.fault_detail.is_some(){return self.fault(cx,"")}
  if self.pending_completion_rejection.is_some(){return self.fault(cx,"layout import completion remains rejected")}
  if !self.decoded{cx.set_stage("layout-import-decode");if let Err(detail)=self.decode(request){return self.fault(cx,detail)}cx.consume_fuel(1);}
  if !self.checkpoint_delivered{let outcome=self.checkpoint_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Checkpoint{applied_progress:1},&[1],cx)?;if outcome.is_some(){self.checkpoint_delivered=true;}return Ok(outcome)}
  cx.set_stage("layout-import-publish");
  if !self.completed{let completion=&request.completion;if !completion.has_mounted_consumer(){return self.fault(cx,"layout import completion consumer is absent")}
   let mutations=self.mutations.take().unwrap_or_default();
   if let Err(rejected)=completion.complete(Ok(Emit{artifact_mutations:mutations,ui_scope:semio_framework::kernel::UiDirtyScope::Full,..Default::default()}),semio_plugin::EphemeralEmit::default()){let message=rejected.fault.message.clone();self.pending_completion_rejection=Some(rejected);return self.fault(cx,&message)}
   self.completed=true;
  }
  Ok(semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None)?)
 }
}
impl ArtifactReservedWork<EditorApp<LayoutPlayApp>> for LayoutImportJob{
 fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<LayoutPlayApp>>,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(request,cx))}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,semio_framework_value::ValueError>{
  match descriptor.kind(){semio_framework_job::JobOutcomeKind::Yield=>descriptor.yielded(),semio_framework_job::JobOutcomeKind::Cancelled=>descriptor.cancelled(),semio_framework_job::JobOutcomeKind::Complete=>descriptor.complete(None,None),semio_framework_job::JobOutcomeKind::CheckpointReady{..}=>self.checkpoint_publication.borrow_outcome(descriptor),semio_framework_job::JobOutcomeKind::Fault=>self.fault_publication.borrow_outcome(descriptor),semio_framework_job::JobOutcomeKind::PreviewReady=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout import publishes no preview"))}
 }
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(body)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
  self.begin_close();
  let body=cx.retained_grant().maximum_copy_bytes;
  let demand=self.demands(body)?;
  let step=semio_framework_job::close_original_effect_step(cx,demand,|effect|{
   let result=self.close_value(body,effect);
   RetirementPollExit::settle(effect,result)
  })?;
  Ok(if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{semio_framework_job::InteractiveJobCloseStep::Pending{progress:step.progress()}})
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.mutations.is_none()&&self.active.is_empty()&&self.pending_completion_rejection.is_none()&&self.fault_detail.is_none()&&self.fault_publication.terminal_is_empty()&&self.checkpoint_publication.terminal_is_empty()}
}
//#endregion 🎞️ReservedImport
