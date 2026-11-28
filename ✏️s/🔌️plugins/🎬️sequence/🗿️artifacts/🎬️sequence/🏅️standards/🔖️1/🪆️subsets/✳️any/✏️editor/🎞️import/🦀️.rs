//! 🎞️ Original reserved media stays in its request while typed parameters and private publications receive funded closure.
use semio_plugin_job_receiving::original_pack_receiving::OriginalPackReceiving;
use semio_plugin_job_receiving::original_steps::{receipt,retired};
use neural_engine::Dictionary;
use semio_plugin::{DslValue,Emit,MediaPayload,NoConfigMutation,ArtifactReservedToolInput,ArtifactReservedToolJobRequest,EditorApp};
use semio_plugin::app::ArtifactReservedWork;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand,InteractiveJobStep,InteractiveJobCloseStep,JobOutcomeBorrow,admit_original_construction_settled,OriginalConstructionAdmission,OriginalConstructionFailure};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll,RetirementPollExit,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::RetainedCloneClose};
use crate::mutations::SequenceMutation;
use crate::{SequenceStep,SequenceWorkingScene,StepParams};
use crate::editor::sequence::{edit_rules,app::SequencePlayApp,host::max_serial_in_snapshot,editor_vocabulary::{sequence_child_leaves_emit,sequence_host_snapshot_from_children}};
use std::mem::size_of;
pub const SEQUENCE_IMPORT_TOOL_ID:&str="import-media";
const SEQUENCE_IMPORT_PORT:&str="steps:in";
type Request=ArtifactReservedToolJobRequest<EditorApp<SequencePlayApp>>;
type Receiving=OriginalPackReceiving<Dictionary>;
type SequenceEmit=Emit<SequenceMutation,NoConfigMutation>;
type Rejection=semio_plugin::app::ArtifactToolCompletionRejection<EditorApp<SequencePlayApp>>;
type Turn<'a>=Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>;

/// 🧳️ The wrapper retains all source lanes; this worker owns only admitted parsing and publication candidates.
#[derive(Default)]
pub struct SequenceImportJob{
 parser:Option<semio_framework_pack_json::JsonGrammarCursor<DslValue>>,
 json_error:Option<semio_framework_pack_json::JsonError>,
 value:Option<DslValue>,
 receiving:Option<Box<Receiving>>,
 construction:OriginalConstructionAdmission,
 construction_failure:Option<OriginalConstructionFailure>,
 params:Option<Dictionary>,
 emit:Option<SequenceEmit>,
 rejected:Option<Rejection>,
 child_fault:Option<semio_plugin::Fault>,
 retirement:RetainedCloneClose,
 checkpoint_publication:semio_framework_job_publication::RetainedJobPublication,
 checkpoint_delivered:bool,
 fault_publication:semio_framework_job_publication::RetainedJobPublication,
 fault_detail:Option<Vec<u8>>,
 phase:u8,
 completed:bool,
 closing:bool,
}
fn invalid()->ValueError{ValueError::invariant("Sequence original reserved import lost its required source")}
fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn event(cx:&mut StepContext<'_>,construction:&mut OriginalConstructionAdmission,d:RetirementDemand,run:impl FnOnce()->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
 let mut done=None;
 admit_original_construction_settled(cx,construction,&mut done,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},1,|_,_|RetainedWorkOutcome{result:run().map(|()|((),progress(d))),work:1})
}
fn original_receive(cx:&mut StepContext<'_>,run:impl FnOnce(&mut semio_framework_value_native_owned::NativeDecodeControl<'_>,semio_framework_value::RetainedCloneGrant)->Result<RetainedWorkOutcome<RetainedCloneStep,ValueError>,ValueError>)->Result<(),OriginalTurnFailure>{
 let issued=cx.retained_grant();let mut scope=None;let mut stepped=None;
 if !cx.with_native_receiving(&mut scope,|control,grant|{stepped=Some(run(control,grant)?);Ok(())})?{return Ok(())}
 let scope=scope.ok_or_else(invalid)?;
 if scope.source_refusal.is_some()||scope.call.is_err(){return Err(ValueError::invariant("Sequence original native receiving scope refused").with_retained_progress(scope.retained_progress()).into())}
 let turn=stepped.ok_or_else(invalid)?;
 let effects=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
 let receipt=cx.consume_receiving_retained(issued,effects);cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));
 match turn.result{Ok(_)=>Ok(receipt?),Err(error)=>Err(OriginalTurnFailure{receipt_refusal:receipt.err(),..OriginalTurnFailure::from(error)})}
}
impl SequenceImportJob{
 fn fault<'a>(&'a mut self,cx:&mut StepContext<'_>,detail:&'static str)->Turn<'a>{
  if self.fault_detail.is_none(){
   let d=RetirementDemand{capacity_bytes:detail.len(),copy_bytes:detail.len()+size_of::<Option<Vec<u8>>>(),depth:1,..Default::default()};
   if !event(cx,&mut self.construction,d,||{self.fault_detail=Some(detail.as_bytes().to_vec());Ok(())})?{return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
  }
  self.fault_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault,self.fault_detail.as_deref().unwrap_or_default(),cx)
 }
 fn receive_parameters(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if cx.should_yield(){return Ok(false)}
  match self.phase{
   2=>{
    let wraps=!matches!(self.value.as_ref().ok_or_else(invalid)?,DslValue::Object(_));
    let d=RetirementDemand{copy_bytes:size_of::<(Option<DslValue>,u8)>()+if wraps{5+size_of::<(String,DslValue)>()}else{0},capacity_bytes:if wraps{5+size_of::<(String,DslValue)>()}else{0},depth:1,..Default::default()};
    event(cx,&mut self.construction,d,||{let original=self.value.take().ok_or_else(invalid)?;self.value=Some(if wraps{let mut entries=Vec::with_capacity(1);entries.push(("value".to_owned(),original));DslValue::Object(entries)}else{original});self.phase=3;Ok(())})?;
   },
   3=>{if Receiving::admit_value(&mut self.value,cx,&mut self.construction,&mut self.receiving)?{self.phase=4}},
   4=>{if self.receiving.as_mut().ok_or_else(invalid)?.advance(cx)?{self.phase=5}},
   5=>{if self.receiving.as_mut().ok_or_else(invalid)?.take(cx,&mut self.params)?{self.phase=6}},
   6=>{if self.close_receiving(cx)?{event(cx,&mut self.construction,fixed::<u8>(),||{self.phase=7;Ok(())})?;}},
   7=>return Ok(true),
   _=>return Err(invalid().into()),
  }
  Ok(self.phase==7)
 }
 fn receiving_release()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<Receiving>>>(),..Receiving::release_demands()}}
 fn close_receiving(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Some(receiver)=self.receiving.as_mut()else{return Ok(true)};
  if !receiver.terminal_is_empty(){cx.with_original_retirement(|cx|receiver.close_step(cx))?;return Ok(false)}
  let d=Self::receiving_release();
  retired(cx,d,|_|{self.receiving.take();Ok(RetainedCloneStep::Progress(receipt(d)))})?;Ok(false)
 }
 fn decode(&mut self,request:&mut Request,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if cx.should_yield(){return Ok(false)}
  match self.phase{
   0=>{
    let Some(ArtifactReservedToolInput::Media{port,media})=request.input.as_mut()else{return Err(invalid().into())};
    if port!=SEQUENCE_IMPORT_PORT{return Err(invalid().into())}
    match &mut media.payload{
     MediaPayload::Intrinsic{value,..}=>{event(cx,&mut self.construction,fixed::<(DslValue,Option<DslValue>,u8)>(),||{self.value=Some(std::mem::replace(value,DslValue::Null));self.phase=2;Ok(())})?;},
     MediaPayload::Structured{..}=>{event(cx,&mut self.construction,fixed::<(Option<semio_framework_pack_json::JsonGrammarCursor<DslValue>>,u8)>(),||{self.parser=Some(semio_framework_pack_json::JsonGrammarCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject));self.phase=1;Ok(())})?;},
     MediaPayload::Binary{..}=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Sequence steps:in requires intrinsic or structured parameters").into()),
    }
   },
   1=>{
    if self.value.is_some(){event(cx,&mut self.construction,fixed::<u8>(),||{self.phase=2;Ok(())})?;return Ok(false)}
    let Some(ArtifactReservedToolInput::Media{media,..})=request.input.as_ref()else{return Err(invalid().into())};
    let MediaPayload::Structured{json,..}=&media.payload else{return Err(invalid().into())};
    let parser=self.parser.as_mut().ok_or_else(invalid)?;
    let units=cx.fuel_remaining().max(1);
    original_receive(cx,|control,grant|{
     let capture=size_of::<Option<DslValue>>()+size_of::<Option<semio_framework_pack_json::JsonError>>();
     let demand=parser.normal_step_demands(json.as_str()).map_err(|error|ValueError::literal(error.kind(),"Sequence JSON demand refused"))?;
     let mut demand=demand;demand.copy_bytes=demand.copy_bytes.saturating_add(capture);
     if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
     let child=semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-capture,..grant};
     let Some(result)=control.with_effect(child,units,|control,effect|parser.step(json,1,control,effect))? else{return Ok(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})};
     let work=parser.last_turn_work();let mut p=parser.normal_step_progress();p.copied_bytes+=capture;
     let result=match result{
      Ok(Some(value))=>{self.value=Some(value);Ok(RetainedCloneStep::Progress(p))},
      Ok(None)=>Ok(RetainedCloneStep::Progress(p)),
      Err(error)=>{let kind=error.kind();self.json_error=Some(error);Err(ValueError::literal(kind,"Sequence parameter JSON grammar refused").with_retained_progress(p))},
     };
     Ok(RetainedWorkOutcome{result,work:work.into()})
    })?;
    if self.value.is_some(){event(cx,&mut self.construction,fixed::<u8>(),||{self.phase=2;Ok(())})?;}
   },
   2..=6=>{self.receive_parameters(cx)?;},
   7=>{
    let live=sequence_host_snapshot_from_children(request.snapshot.as_ref(),&request.children).map_err(|_|invalid())?;
    let serial=max_serial_in_snapshot(&live).max(100).checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Sequence import step identity space exhausted"))?;
    let id=format!("step-{serial}");
    let x=live.steps.iter().map(|step|step.x).fold(0.0_f64,f64::max)+if live.steps.is_empty(){0.0}else{280.0};
    let mut edit=edit_rules::SceneEdit::new(SequenceWorkingScene{steps:live.steps.clone(),edges:live.edges.clone()});
    let params=StepParams(self.params.take().ok_or_else(invalid)?);
    edit.insert_step(SequenceStep{id,kind:"computation.import".into(),params,x,y:0.0,slot:None,collapsed:false});
    self.emit=Some(sequence_child_leaves_emit(request.snapshot.as_ref(),edit.leaves));self.phase=8;cx.consume_fuel(1);
   },
   8=>return Ok(true),
   _=>return Err(invalid().into()),
  }
  Ok(self.phase==8)
 }
 fn step_value<'a>(&'a mut self,request:&mut Request,cx:&mut StepContext<'_>)->Turn<'a>{
  if cx.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)}
  if self.fault_detail.is_some(){return self.fault(cx,"")}
  if self.rejected.is_some(){return self.fault(cx,"Sequence import completion remains rejected")}
  if self.child_fault.is_some(){return self.fault(cx,"Sequence child preparation remains refused")}
  if self.phase<8{cx.set_stage("sequence-import-receive");if !self.decode(request,cx)?{return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}}
  if !self.checkpoint_delivered{let outcome=self.checkpoint_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Checkpoint{applied_progress:1},&[1],cx)?;if outcome.is_some(){self.checkpoint_delivered=true;}return Ok(outcome)}
  cx.set_stage("sequence-import-publish");
  if let Some(emit)=self.emit.as_mut(){
   let step=emit.prepare_child_original(cx).map_err(|failure|OriginalTurnFailure::from(failure.debit_original(cx)))?;
   cx.consume_retained(step.progress())?;
   match step{
    semio_plugin::app::ChildEmitPreparationStep::Ready(_)=>{},
    semio_plugin::app::ChildEmitPreparationStep::Pending(_)=>return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into),
    semio_plugin::app::ChildEmitPreparationStep::Refused(fault,_)=>{self.child_fault=Some(fault);return self.fault(cx,"Sequence child preparation refused")},
   }
  }
  if !self.completed{
   if !request.completion.has_mounted_consumer(){return self.fault(cx,"Sequence import completion consumer is absent")}
   let emit=self.emit.take().ok_or_else(invalid)?;
   if let Err(rejected)=request.completion.complete(Ok(emit),semio_plugin::EphemeralEmit::default()){self.rejected=Some(rejected);return self.fault(cx,"Sequence import completion refused")}
   self.completed=true;
  }
  JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)
 }
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.construction.terminal_is_empty(){if self.construction.failure_untaken(){let d=self.construction.take_failure_demand();return Ok(RetirementDemand{copy_bytes:d.copy_bytes,depth:d.depth,..Default::default()})}return Ok(self.construction.close_demand())}
  if !self.fault_publication.terminal_is_empty(){return self.fault_publication.retirement_demands()}
  if !self.checkpoint_publication.terminal_is_empty(){return self.checkpoint_publication.retirement_demands()}
  if let Some(receiver)=self.receiving.as_ref(){return if receiver.terminal_is_empty(){Ok(Self::receiving_release())}else{receiver.close_demands()}}
  macro_rules! demand{($field:ident,$ty:ty)=>{if self.$field.is_some()||!self.retirement.is_empty(){return self.retirement.next_owner_demands::<$ty>(self.$field.is_some(),&None,body)}};}
  demand!(construction_failure,OriginalConstructionFailure);demand!(value,DslValue);demand!(parser,semio_framework_pack_json::JsonGrammarCursor<DslValue>);demand!(json_error,semio_framework_pack_json::JsonError);demand!(params,Dictionary);demand!(emit,SequenceEmit);demand!(rejected,Rejection);demand!(child_fault,semio_plugin::Fault);demand!(fault_detail,Vec<u8>);
  Ok(Default::default())
 }
 fn close_turn(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  if !self.fault_publication.terminal_is_empty(){let turn=self.fault_publication.close_step(effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}
  if !self.checkpoint_publication.terminal_is_empty(){let turn=self.checkpoint_publication.close_step(effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}
  if !self.retirement.is_empty(){return self.retirement.step_granted(body,effect)}
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   macro_rules! retire{($field:ident)=>{if self.$field.is_some(){return Ok(self.retirement.begin_granted(&mut self.$field,effect).result?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}};}
   retire!(construction_failure);retire!(value);retire!(parser);retire!(json_error);retire!(params);retire!(emit);retire!(rejected);retire!(child_fault);retire!(fault_detail);
   Ok(RetainedCloneStep::Complete(Default::default()))
  })();
  RetirementPollExit::settle(effect,result)
 }
}
impl ArtifactReservedWork<EditorApp<SequencePlayApp>> for SequenceImportJob{
 fn step<'a>(&'a mut self,request:&'a mut Request,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{InteractiveJobStep::from_callback(self.step_value(request,cx))}
 fn borrow_outcome<'a>(&'a self,d:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,ValueError>{
  match d.kind(){semio_framework_job::JobOutcomeKind::Yield=>d.yielded(),semio_framework_job::JobOutcomeKind::Cancelled=>d.cancelled(),semio_framework_job::JobOutcomeKind::Complete=>d.complete(None,None),semio_framework_job::JobOutcomeKind::CheckpointReady{..}=>self.checkpoint_publication.borrow_outcome(d),semio_framework_job::JobOutcomeKind::Fault=>self.fault_publication.borrow_outcome(d),semio_framework_job::JobOutcomeKind::PreviewReady=>Err(invalid())}
 }
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.begin_close();
  if !self.construction.terminal_is_empty(){
   if self.construction.failure_untaken(){
    let d=self.construction.take_failure_demand();let moved=self.construction.take_failure_into_step(&mut self.construction_failure,cx)?;
    return Ok(InteractiveJobCloseStep::Pending{progress:if moved{RetainedCloneProgress{copied_items:d.items,copied_bytes:d.copy_bytes,..Default::default()}}else{Default::default()}})
   }
   return Ok(InteractiveJobCloseStep::Pending{progress:self.construction.retire_step(cx)?.progress()})
  }
  if self.receiving.is_some(){self.close_receiving(cx)?;return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
  let body=cx.retained_grant().maximum_copy_bytes;let d=self.demands(body)?;let step=semio_framework_job::close_original_effect_step(cx,d,|effect|self.close_turn(body,effect))?;
  Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.parser.is_none()&&self.json_error.is_none()&&self.value.is_none()&&self.receiving.is_none()&&self.params.is_none()&&self.emit.is_none()&&self.rejected.is_none()&&self.child_fault.is_none()&&self.retirement.is_empty()&&self.construction.terminal_is_empty()&&self.construction_failure.is_none()&&self.fault_detail.is_none()&&self.checkpoint_publication.terminal_is_empty()&&self.fault_publication.terminal_is_empty()}
}

#[cfg(test)]
mod original_parameter_laws{
 use super::*;
 use semio_plugin::artifact_app_laws::OriginalWorkFixture;
 #[test]
 fn parameters_keep_original_custody_through_zero_work_and_denied_header_turns(){
  let rows:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/import-parameters.json")).unwrap();
  for row in rows["cases"].as_array().unwrap(){
   let value: DslValue=semio_framework_pack_json::from_json_str(row["wire"].as_str().unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
   let mut work=SequenceImportJob{value:Some(value),phase:2,..Default::default()};
   let original=work.value.as_ref().unwrap() as *const DslValue;
   let mut caller=OriginalWorkFixture::new();
   let grant=OriginalWorkFixture::stock();
   caller.work_turn(grant,0,|cx|{assert!(!work.receive_parameters(cx)?);Ok(())}).unwrap();
   assert_eq!(work.phase,2);assert_eq!(work.value.as_ref().unwrap() as *const DslValue,original);
   caller.turn(semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:0,..grant},|cx|{assert!(!work.receive_parameters(cx)?);Ok(())}).unwrap();
   assert_eq!(work.phase,2);assert_eq!(work.value.as_ref().unwrap() as *const DslValue,original);
   for _ in 0..10000{if work.phase==7{break}caller.turn(grant,|cx|{work.receive_parameters(cx)?;Ok(())}).unwrap();}
   assert_eq!(work.phase,7,"{}",row["id"]);
   let actual:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(work.params.as_ref().unwrap())).unwrap();
   assert_eq!(actual,row["expected"],"{}",row["id"]);
   work.begin_close();
   for _ in 0..10000{if work.terminal_is_empty(){break}caller.turn(grant,|cx|{work.close_step(cx)?;Ok(())}).unwrap();}
   assert!(work.terminal_is_empty(),"{}",row["id"]);caller.finish();
  }
 }
}
