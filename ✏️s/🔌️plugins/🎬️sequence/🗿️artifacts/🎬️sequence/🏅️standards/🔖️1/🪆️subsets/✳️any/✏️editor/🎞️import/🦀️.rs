//! 🎞️ Original reserved media stays in its request while typed parameters and private publications receive funded closure.
use neural_engine::Dictionary;
use semio_plugin::{DslValue,Emit,MediaPayload,NoConfigMutation,ArtifactReservedToolInput,ArtifactReservedToolJobRequest,EditorApp};
use semio_plugin::tool_contract::ArtifactReservedWork;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand,InteractiveJobStep,InteractiveJobCloseStep,JobOutcomeBorrow,admit_original_construction,receive_original_retained_work,close_original_retained_step};
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneClose}};
use semio_framework_value_native_owned::{OriginalReceivingPrimary,owned::NativeReceivingOriginal};
use crate::mutations::SequenceMutation;
use crate::{SequenceStep,SequenceWorkingScene,StepParams};
use crate::editor::sequence::{edit_rules,app::SequencePlayApp,host::max_serial_in_snapshot,editor_vocabulary::{sequence_child_leaves_emit,sequence_host_snapshot_from_children}};
use std::mem::size_of;
pub const SEQUENCE_IMPORT_TOOL_ID:&str="import-media";
const SEQUENCE_IMPORT_PORT:&str="steps:in";
type Request=ArtifactReservedToolJobRequest<EditorApp<SequencePlayApp>>;
type Receiver=NativeReceivingOriginal<DslValue,Dictionary>;
type Admission=RetainedCloneSourceAdmission<DslValue,DslValue,()>;
type SequenceEmit=Emit<SequenceMutation,NoConfigMutation>;
type Rejection=semio_plugin::app::ArtifactToolCompletionRejection<EditorApp<SequencePlayApp>>;
type Turn<'a>=Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>;

/// 🧳️ The wrapper retains all source lanes; this worker owns only admitted parsing and publication candidates.
#[derive(Default)]
pub struct SequenceImportJob{
 parser:Option<semio_framework_pack_json::JsonGrammarCursor<DslValue>>,
 json_error:Option<semio_framework_pack_json::JsonError>,
 value:Option<DslValue>,
 admission:Option<Admission>,
 source:Option<RetainedCloneSource<DslValue>>,
 receiver:Option<Receiver>,
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
fn event(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce()->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
 let mut done=None;
 admit_original_construction(cx,&mut done,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|_|RetainedWorkOutcome{result:run().map(|()|((),progress(d))),work:1})
}
fn original_receive(cx:&mut StepContext<'_>,run:impl FnOnce(&mut semio_framework_value_native_owned::NativeDecodeControl<'_>,semio_framework_value::RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>)->Result<(),OriginalTurnFailure>{
 let issued=cx.retained_grant();let mut work=0;
 let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|control|{let turn=run(control,issued);work=turn.work;turn.result});
 let effects=match &result{Ok(step)=>step.progress(),Err(primary)=>primary.retained_progress()};
 let receipt=cx.consume_receiving_retained(issued,effects);cx.consume_fuel(work);
 match result{Ok(_)=>receipt.map_err(Into::into),Err(primary)=>Err(OriginalTurnFailure{effects,primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None,work_refusal:None})}
}
impl SequenceImportJob{
 fn fault<'a>(&'a mut self,cx:&mut StepContext<'_>,detail:&'static str)->Turn<'a>{
  if self.fault_detail.is_none(){
   let d=RetirementDemand{capacity_bytes:detail.len(),copy_bytes:detail.len()+size_of::<Option<Vec<u8>>>(),depth:1,..Default::default()};
   if !event(cx,d,||{self.fault_detail=Some(detail.as_bytes().to_vec());Ok(())})?{return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
  }
  self.fault_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault,self.fault_detail.as_deref().unwrap_or_default(),cx)
 }
 fn receive_parameters(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if cx.should_yield(){return Ok(false)}
  match self.phase{
   2=>{
    let wraps=!matches!(self.value.as_ref().ok_or_else(invalid)?,DslValue::Object(_));
    let d=RetirementDemand{copy_bytes:size_of::<(Option<DslValue>,u8)>()+if wraps{5+size_of::<(String,DslValue)>()}else{0},capacity_bytes:if wraps{5+size_of::<(String,DslValue)>()}else{0},depth:1,..Default::default()};
    event(cx,d,||{let original=self.value.take().ok_or_else(invalid)?;self.value=Some(if wraps{let mut entries=Vec::with_capacity(1);entries.push(("value".to_owned(),original));DslValue::Object(entries)}else{original});self.phase=3;Ok(())})?;
   },
   3=>{
    if self.source.is_some(){event(cx,fixed::<(Option<Admission>,u8)>(),||{if !self.admission.as_ref().is_some_and(|a|a.terminal_is_empty()){return Err(invalid())}self.admission=None;self.phase=4;Ok(())})?;return Ok(false)}
    if self.admission.is_none(){event(cx,fixed::<(Option<DslValue>,Option<Admission>)>(),||{self.admission=Some(RetainedCloneSource::<DslValue>::admission_owned(self.value.take().ok_or_else(invalid)?,()));Ok(())})?;return Ok(false)}
    let admission=self.admission.as_mut().ok_or_else(invalid)?;
    let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,cx.retained_grant())}else{admission.advance(cx.retained_grant())};
    receive_original_retained_work(cx,turn)?;
    if self.source.is_some(){event(cx,fixed::<(Option<Admission>,u8)>(),||{if !self.admission.as_ref().is_some_and(|a|a.terminal_is_empty()){return Err(invalid())}self.admission=None;self.phase=4;Ok(())})?;}
   },
   4=>{event(cx,fixed::<(Option<Receiver>,Option<RetainedCloneSource<DslValue>>,u8)>(),||{self.receiver=Some(Receiver::new(self.source.take().ok_or_else(invalid)?));self.phase=5;Ok(())})?;},
   5=>{
    let receiver=self.receiver.as_mut().ok_or_else(invalid)?;
    if !receiver.output_ready(){original_receive(cx,|control,grant|receiver.advance(control,grant.maximum_copy_bytes,grant))?;}
    if receiver.output_ready(){event(cx,fixed::<u8>(),||{self.phase=6;Ok(())})?;}
   },
   6=>{
    if self.params.is_some(){event(cx,fixed::<u8>(),||{self.phase=7;Ok(())})?;return Ok(false)}
    let turn=self.receiver.as_mut().ok_or_else(invalid)?.take(cx.retained_grant());
    let result=turn.result.map(|ready|match ready{Some((value,p))=>{self.params=Some(value);RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())});
    receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;
    if self.params.is_some(){event(cx,fixed::<u8>(),||{self.phase=7;Ok(())})?;}
   },
   7=>return Ok(true),
   _=>return Err(invalid().into()),
  }
  Ok(self.phase==7)
 }
 fn decode(&mut self,request:&mut Request,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if cx.should_yield(){return Ok(false)}
  match self.phase{
   0=>{
    let Some(ArtifactReservedToolInput::Media{port,media})=request.input.as_mut()else{return Err(invalid().into())};
    if port!=SEQUENCE_IMPORT_PORT{return Err(invalid().into())}
    match &mut media.payload{
     MediaPayload::Intrinsic{value,..}=>{event(cx,fixed::<(DslValue,Option<DslValue>,u8)>(),||{self.value=Some(std::mem::replace(value,DslValue::Null));self.phase=2;Ok(())})?;},
     MediaPayload::Structured{..}=>{event(cx,fixed::<(Option<semio_framework_pack_json::JsonGrammarCursor<DslValue>>,u8)>(),||{self.parser=Some(semio_framework_pack_json::JsonGrammarCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject));self.phase=1;Ok(())})?;},
     MediaPayload::Binary{..}=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Sequence steps:in requires intrinsic or structured parameters").into()),
    }
   },
   1=>{
    if self.value.is_some(){event(cx,fixed::<u8>(),||{self.phase=2;Ok(())})?;return Ok(false)}
    let Some(ArtifactReservedToolInput::Media{media,..})=request.input.as_ref()else{return Err(invalid().into())};
    let MediaPayload::Structured{json,..}=&media.payload else{return Err(invalid().into())};
    let parser=self.parser.as_mut().ok_or_else(invalid)?;
    original_receive(cx,|control,grant|{
     let capture=size_of::<Option<DslValue>>()+size_of::<Option<semio_framework_pack_json::JsonError>>();
     let demand=parser.normal_step_demands(json.as_str()).map_err(|error|ValueError::literal(error.kind(),"Sequence JSON demand refused"));
     let demand=match demand{Ok(mut d)=>{d.copy_bytes=d.copy_bytes.saturating_add(capture);d},Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
     if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
     let child=semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-capture,..grant};
     let result=parser.step(json,1,control,child);let work=parser.last_turn_work();let mut p=parser.normal_step_progress();p.copied_bytes+=capture;
     let result=match result{
      Ok(Some(value))=>{self.value=Some(value);Ok(RetainedCloneStep::Progress(p))},
      Ok(None)=>Ok(RetainedCloneStep::Progress(p)),
      Err(error)=>{let kind=error.kind();self.json_error=Some(error);Err(ValueError::literal(kind,"Sequence parameter JSON grammar refused").with_retained_progress(p).into())},
     };RetainedWorkOutcome{result,work}
    })?;
    if self.value.is_some(){event(cx,fixed::<u8>(),||{self.phase=2;Ok(())})?;}
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
   let grant=cx.retained_grant();
   let turn=emit.prepare_child_original(grant);cx.consume_fuel(turn.work);
   match turn.result{
    Ok(semio_plugin::app::ChildEmitPreparationStep::Ready(p))=>{cx.consume_retained(p)?;},
    Ok(semio_plugin::app::ChildEmitPreparationStep::Pending(p))=>{cx.consume_retained(p)?;return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)},
    Ok(semio_plugin::app::ChildEmitPreparationStep::Refused(fault,p))=>{self.child_fault=Some(fault);cx.consume_retained(p)?;return self.fault(cx,"Sequence child preparation refused")},
    Err(fault)=>{self.child_fault=Some(fault);return self.fault(cx,"Sequence child preparation failed")},
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
  if !self.fault_publication.terminal_is_empty(){return self.fault_publication.retirement_demands()}
  if !self.checkpoint_publication.terminal_is_empty(){return self.checkpoint_publication.retirement_demands()}
  if let Some(admission)=self.admission.as_ref(){return if admission.terminal_is_empty(){Ok(fixed::<Option<Admission>>())}else if admission.phase()=="ready"{Ok(fixed::<Option<RetainedCloneSource<DslValue>>>())}else{admission.next_return_demand()}}
  if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(fixed::<Option<Receiver>>())}else{receiver.close_demands(body)}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed::<Option<RetainedCloneSource<DslValue>>>())}else{source.next_close_demand(body)}}
  macro_rules! demand{($field:ident,$ty:ty)=>{if self.$field.is_some()||!self.retirement.is_empty(){return self.retirement.next_owner_demands::<$ty>(self.$field.is_some(),&None,body)}};}
  demand!(value,DslValue);demand!(parser,semio_framework_pack_json::JsonGrammarCursor<DslValue>);demand!(json_error,semio_framework_pack_json::JsonError);demand!(params,Dictionary);demand!(emit,SequenceEmit);demand!(rejected,Rejection);demand!(child_fault,semio_plugin::Fault);demand!(fault_detail,Vec<u8>);
  Ok(Default::default())
 }
 fn close_turn(&mut self,grant:semio_framework_value::RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.fault_publication.terminal_is_empty(){return self.fault_publication.close_step(grant)}
  if !self.checkpoint_publication.terminal_is_empty(){return self.checkpoint_publication.close_step(grant)}
  if let Some(admission)=self.admission.as_mut(){
   if admission.terminal_is_empty(){self.admission=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress(fixed::<Option<Admission>>()))),work:1}}
   if admission.phase()=="ready"{return admission.take_output(&mut self.source,grant)}
   let mut returned=None;let turn=admission.return_inputs(&mut returned,grant);if let Some((value,()))=returned{self.value=Some(value)}return turn
  }
  if let Some(receiver)=self.receiver.as_mut(){
   receiver.begin_close();if !receiver.terminal_is_empty(){return receiver.close_step(grant)}
   self.receiver=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress(fixed::<Option<Receiver>>()))),work:1}
  }
  if let Some(source)=self.source.as_mut(){
   if !source.terminal_is_empty(){return source.close_step(grant)}
   self.source=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress(fixed::<Option<RetainedCloneSource<DslValue>>>()))),work:1}
  }
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}
  macro_rules! retire{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,grant);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}};}
  retire!(value);retire!(parser);retire!(json_error);retire!(params);retire!(emit);retire!(rejected);retire!(child_fault);retire!(fault_detail);
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
}
impl ArtifactReservedWork<EditorApp<SequencePlayApp>> for SequenceImportJob{
 fn step<'a>(&'a mut self,request:&'a mut Request,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{InteractiveJobStep::from_callback(self.step_value(request,cx))}
 fn borrow_outcome<'a>(&'a self,d:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,ValueError>{
  match d.kind(){semio_framework_job::JobOutcomeKind::Yield=>d.yielded(),semio_framework_job::JobOutcomeKind::Cancelled=>d.cancelled(),semio_framework_job::JobOutcomeKind::Complete=>d.complete(None,None),semio_framework_job::JobOutcomeKind::CheckpointReady{..}=>self.checkpoint_publication.borrow_outcome(d),semio_framework_job::JobOutcomeKind::Fault=>self.fault_publication.borrow_outcome(d),semio_framework_job::JobOutcomeKind::PreviewReady=>Err(invalid())}
 }
 fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close()}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.begin_close();let d=self.demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|grant|self.close_turn(grant))?;
  Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.parser.is_none()&&self.json_error.is_none()&&self.value.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.receiver.is_none()&&self.params.is_none()&&self.emit.is_none()&&self.rejected.is_none()&&self.child_fault.is_none()&&self.retirement.is_empty()&&self.fault_detail.is_none()&&self.checkpoint_publication.terminal_is_empty()&&self.fault_publication.terminal_is_empty()}
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
