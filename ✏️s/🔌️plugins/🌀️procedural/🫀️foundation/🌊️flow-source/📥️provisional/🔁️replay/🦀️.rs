//! 🔁️ Original provisional leaves replay through native receiving and retained domain producers.
use super::retained_provisional_source::{ProvisionalSourceWork,ProvisionalFieldWork};
use super::retained_native_provisional_fold::{NativeProvisionalFold,NativeProvisionalFolds};
use super::retained_scratch_driver::{begun,funded_source_turn,funded_turn,local_progress,pay,replied};
use super::GenerationPlayState;
use semio_plugin::{ArtifactApp,app::ArtifactOwnedContextHandle};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_pack_json::native_decoding::owned::NativeReceive;
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalTurnPrimary,close_original_effect_step};
use semio_framework_value::{DslValue,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedOwnedProjection,RetainedOwnedProjectionAdmission,RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Source,Project,TakeProject,Receive,Take,CloseReceiver,CloseFailure,CreateFold,Apply,CloseFold,Next,Ready}
struct Fields<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>>where A::Mutation:NativeReceive<DslValue>{
 source:ProvisionalSourceWork<A>,admission:RetainedOwnedProjectionAdmission<DslValue>,receiver:Option<ProvisionalFieldWork<A::Mutation>>,mutation:Option<A::Mutation>,fold:Option<NativeProvisionalFold<S::Additional>>,failure:Option<ControlledRetirement<OriginalTurnFailure>>
}
pub struct ProvisionalReplayWork<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>>where A::Mutation:NativeReceive<DslValue>{
 fields:ManuallyDrop<Fields<A,S>>,retirement:RetainedCloneClose,original:OriginalSourceBinding,index:usize,phase:Phase,closing:bool,progress:RetainedCloneProgress
}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn ignorable(error:&OriginalTurnFailure)->bool{
 if error.receipt_refusal.is_some()||error.context_refusal.is_some()||error.work_refusal.is_some(){return false}
 matches!(&error.primary,OriginalTurnPrimary::Value(primary) if primary.kind==ValueRefusalKind::InvalidValue)
}
impl<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>> ProvisionalReplayWork<A,S>where A::Mutation:NativeReceive<DslValue>{
 pub fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub fn new_original(original:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{source:ProvisionalSourceWork::new_original(original),admission:Default::default(),receiver:None,mutation:None,fold:None,failure:None}),retirement:Default::default(),original,index:0,phase:Phase::Source,closing:false,progress:Default::default()}}
 pub fn receive_context(&mut self,context:&ArtifactOwnedContextHandle<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;self.fields.source.receive_context(context,cx)
 }
 fn authenticate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.original.matches(cx)&&!self.closing{cx.original_source_parent()?.validate_borrowed_context(cx)?;Ok(())}else{Err(invalid("provisional replay requires its initiating original source").into())}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==Phase::Ready}
 pub fn generation(&self)->Option<u64>{self.fields.source.generation()}
 pub fn next_demands(&self,host:&FlowHostSnapshot,state:&GenerationPlayState,copy:usize,work:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid("provisional replay is closing"))}
  match self.phase{
   Phase::Source=>self.fields.source.next_demands(),
   Phase::Project=>Ok(self.fields.source.projection_demands(self.index,&self.fields.admission)?.0),
   Phase::TakeProject=>{let mut d=self.fields.admission.take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<ProvisionalFieldWork<A::Mutation>>()).ok_or_else(||invalid("provisional receiver header overflow"))?;Ok(d)},
   Phase::Receive=>self.fields.receiver.as_ref().ok_or_else(||invalid("provisional receiver is absent"))?.next_demands(copy,work),
   Phase::Take=>self.fields.receiver.as_ref().ok_or_else(||invalid("provisional receiver is absent"))?.take_demands(),
   Phase::CloseReceiver=>self.fields.receiver.as_ref().ok_or_else(||invalid("provisional receiver is absent"))?.next_close_demands(copy),
   Phase::CloseFailure=>self.fields.failure.as_ref().ok_or_else(||invalid("provisional failure owner is absent"))?.selected_demand(copy),
   Phase::CreateFold=>Ok(fixed(2*size_of::<Option<A::Mutation>>()+2*size_of::<Option<NativeProvisionalFold<S::Additional>>>())),
   Phase::Apply=>self.fields.fold.as_ref().ok_or_else(||invalid("provisional fold is absent"))?.next_demands(host,state,copy),
   Phase::CloseFold=>self.fields.fold.as_ref().ok_or_else(||invalid("provisional fold is absent"))?.next_close_demands(copy),
   Phase::Next=>Ok(fixed(0)),Phase::Ready=>Ok(Default::default()),
  }
 }
 pub fn step(&mut self,host:&mut FlowHostSnapshot,state:&mut GenerationPlayState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  match self.phase{
   Phase::Source=>{if self.fields.source.step(cx)?{self.phase=if self.fields.source.len()==0{Phase::Ready}else{Phase::Project}}},
   Phase::Project=>{
    let index=self.index;let fields=&mut*self.fields;let(source,admission)=(&fields.source,&mut fields.admission);
    let step=funded_source_turn(cx,|effect|{
     let result=match replied(effect,|effect,reply|source.project_into(index,admission,effect,reply)){Ok(Some(result))=>result,Ok(None)=>Err(invalid("provisional leaf projection suspended")),Err(error)=>Err(error)};
     RetainedWorkOutcome{result,work:effect.performed_work()}
    })?;
    if step.is_some_and(|step|step.progress()!=Default::default()){self.phase=Phase::TakeProject}
   },
   Phase::TakeProject=>{
    let d=self.next_demands(host,state,cx.retained_grant().maximum_copy_bytes,0)?;let original=self.original;
    let fields=&mut*self.fields;let(admission,receiver)=(&mut fields.admission,&mut fields.receiver);
    funded_source_turn(cx,|effect|{
     if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units().map_or(true,|remaining|remaining<1){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
     let mut projected:Option<RetainedOwnedProjection<DslValue>>=None;
     let result=match replied(effect,|effect,reply|admission.take(&mut projected,effect,reply)){Ok(Some(result))=>result.map(|step|{if let Some(projection)=projected.take(){*receiver=Some(ProvisionalFieldWork::new_original(projection,original))}RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:step.progress().copied_bytes+size_of::<ProvisionalFieldWork<A::Mutation>>(),..step.progress()})}),Ok(None)=>Err(invalid("provisional leaf handoff suspended")),Err(error)=>Err(error)};
     RetainedWorkOutcome{result,work:effect.performed_work()}
    })?;
    if self.fields.receiver.is_some(){self.phase=Phase::Receive}
   },
   Phase::Receive=>{
    match self.fields.receiver.as_mut().ok_or_else(||invalid("provisional receiver is absent"))?.step(cx){
     Ok(true)=>self.phase=Phase::Take,Ok(false)=>{},
     Err(error)=>{if !ignorable(&error){return Err(error)}self.fields.failure=Some(ControlledRetirement::new(error).unwrap_or_else(|_|unreachable!("original failures support controlled retirement")));self.fields.receiver.as_mut().unwrap().begin_close();self.phase=Phase::CloseReceiver}
    }
   },
   Phase::Take=>{
    let fields=&mut *self.fields;let(receiver,mutation)=(&mut fields.receiver,&mut fields.mutation);
    if receiver.as_mut().ok_or_else(||invalid("provisional receiver is absent"))?.take_into(mutation,cx)?{receiver.as_mut().unwrap().begin_close();self.phase=Phase::CloseReceiver}
   },
   Phase::CloseReceiver=>{
    let receiver=self.fields.receiver.as_mut().ok_or_else(||invalid("provisional receiver is absent"))?;
    let d=receiver.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
    cx.with_original_retirement(|cx|{close_original_effect_step(cx,d,|effect|{let body=effect.grant().maximum_copy_bytes;receiver.close_step(body,effect)})?;Ok(())})?;
    if receiver.terminal_is_empty(){self.fields.receiver=None;self.phase=if self.fields.failure.is_some(){Phase::CloseFailure}else{Phase::CreateFold}}
   },
   Phase::CloseFailure=>{
    let failure=self.fields.failure.as_mut().ok_or_else(||invalid("provisional failure owner is absent"))?;
    let d=failure.selected_demand(cx.retained_grant().maximum_copy_bytes)?;
    cx.with_original_retirement(|cx|{close_original_effect_step(cx,d,|effect|{let body=effect.grant().maximum_copy_bytes;failure.step(body,effect)})?;Ok(())})?;
    if failure.terminal_is_empty(){self.fields.failure=None;self.phase=Phase::Next}
   },
   Phase::CreateFold=>{
    let d=self.next_demands(host,state,cx.retained_grant().maximum_copy_bytes,0)?;let original=self.original;
    let fields=&mut *self.fields;let(mutation,fold)=(&mut fields.mutation,&mut fields.fold);
    funded_turn(cx,|effect|{
     let result=match pay(effect,1){
      Ok(true)=>match mutation.take(){
       Some(input)=>match S::into_fold_original(input,original){Ok(created)=>{*fold=Some(created);Ok(RetainedCloneStep::Complete(local_progress(d)))},Err(input)=>{*mutation=Some(input);Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original provisional domain producer is incomplete").with_retained_progress(local_progress(d)))}},
       None=>Err(invalid("original provisional mutation is absent")),
      },
      Ok(false)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error),
     };
     RetainedWorkOutcome{result,work:effect.performed_work()}
    })?;
    if self.fields.fold.is_some(){self.phase=Phase::Apply}
   },
   Phase::Apply=>{if self.fields.fold.as_mut().ok_or_else(||invalid("provisional fold is absent"))?.step(host,state,cx)?{self.fields.fold.as_mut().unwrap().begin_close();self.phase=Phase::CloseFold}},
   Phase::CloseFold=>{
    let fold=self.fields.fold.as_mut().ok_or_else(||invalid("provisional fold is absent"))?;let d=fold.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
    cx.with_original_retirement(|cx|{close_original_effect_step(cx,d,|effect|{let body=effect.grant().maximum_copy_bytes;fold.close_step(body,effect)})?;Ok(())})?;
    if fold.terminal_is_empty(){self.fields.fold=None;self.phase=Phase::Next}
   },
   Phase::Next=>{
    let index=&mut self.index;let phase=&mut self.phase;let length=self.fields.source.len();
    funded_turn(cx,|effect|{let result=match pay(effect,1){Ok(true)=>{*index+=1;*phase=if *index==length{Phase::Ready}else{Phase::Project};Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))},Ok(false)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)};RetainedWorkOutcome{result,work:effect.performed_work()}})?;
   },
   Phase::Ready=>{},
  }Ok(self.ready())
 }
 pub fn begin_close(&mut self){self.closing=true;self.fields.source.begin_close();if let Some(receiver)=self.fields.receiver.as_mut(){receiver.begin_close()}if let Some(fold)=self.fields.fold.as_mut(){fold.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(receiver)=self.fields.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(fixed(0))}else{receiver.next_close_demands(copy)}}
  if let Some(fold)=self.fields.fold.as_ref(){return if fold.terminal_is_empty(){Ok(fixed(0))}else{fold.next_close_demands(copy)}}
  if let Some(failure)=self.fields.failure.as_ref(){return if failure.terminal_is_empty(){Ok(fixed(0))}else{failure.selected_demand(copy)}}
  if !self.fields.admission.terminal_is_empty(){return self.fields.admission.close_demand(copy)}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.mutation.is_some(){return self.retirement.next_owner_demands::<A::Mutation>(true,&None,copy)}
  self.fields.source.next_close_demands(copy)
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let result=self.close_original(body,effect);RetirementPollExit::settle(effect,result)
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(invalid("provisional replay close was not begun").into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(effect.grant(),self.next_close_demands(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let step=if let Some(receiver)=self.fields.receiver.as_mut(){if receiver.terminal_is_empty(){self.fields.receiver=None;RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{RetirementPollExit::resume(receiver.close_step(body,effect))?}}
  else if let Some(fold)=self.fields.fold.as_mut(){if fold.terminal_is_empty(){self.fields.fold=None;RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{RetirementPollExit::resume(fold.close_step(body,effect))?}}
  else if let Some(failure)=self.fields.failure.as_mut(){if failure.terminal_is_empty(){self.fields.failure=None;RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{RetirementPollExit::resume(failure.step(body,effect))?}}
  else if !self.fields.admission.terminal_is_empty(){RetirementPollExit::resume(self.fields.admission.close_step(body,effect))?}
  else if !self.retirement.is_empty(){RetirementPollExit::resume(self.retirement.step_granted(body,effect))?}
  else if self.fields.mutation.is_some(){begun(&mut self.retirement,&mut self.fields.mutation,effect)?}
  else{RetirementPollExit::resume(self.fields.source.close_step(body,effect))?};
  self.progress=step.progress();Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.receiver.is_none()&&self.fields.mutation.is_none()&&self.fields.fold.is_none()&&self.fields.failure.is_none()&&self.fields.admission.terminal_is_empty()&&self.retirement.is_empty()&&self.fields.source.terminal_is_empty()}
}
impl<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>> Drop for ProvisionalReplayWork<A,S>where A::Mutation:NativeReceive<DslValue>{
 fn drop(&mut self){assert!(self.terminal_is_empty(),"provisional replay retains original context, leaf, receiver or domain fold");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}
}
