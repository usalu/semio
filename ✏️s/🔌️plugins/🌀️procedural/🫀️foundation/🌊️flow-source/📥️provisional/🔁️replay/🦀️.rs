//! 🔁️ Original provisional leaves replay through native receiving and retained domain producers.
use super::retained_provisional_source::{ProvisionalSourceWork,ProvisionalFieldWork};
use super::retained_native_provisional_fold::{NativeProvisionalFold,NativeProvisionalMutation};
use super::GenerationPlayState;
use semio_plugin::{ArtifactApp,app::ArtifactOwnedContextHandle};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_pack_json::native_decoding::owned::NativeReceive;
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalTurnPrimary,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedOwnedProjection,RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Source,Project,Receive,Take,CloseReceiver,CloseFailure,CreateFold,Apply,CloseFold,Next,Ready}
struct Fields<A:ArtifactApp>where A::Mutation:NativeReceive<DslValue>+NativeProvisionalMutation{
 source:ProvisionalSourceWork<A>,receiver:Option<ProvisionalFieldWork<A::Mutation>>,mutation:Option<A::Mutation>,fold:Option<NativeProvisionalFold>,failure:Option<ControlledRetirement<OriginalTurnFailure>>
}
pub(crate) struct ProvisionalReplayWork<A:ArtifactApp>where A::Mutation:NativeReceive<DslValue>+NativeProvisionalMutation{
 fields:ManuallyDrop<Fields<A>>,retirement:RetainedCloneClose,original:OriginalSourceBinding,index:usize,phase:Phase,closing:bool,progress:RetainedCloneProgress,work:u64
}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn ignorable(error:&OriginalTurnFailure)->bool{
 if error.receipt_refusal.is_some()||error.context_refusal.is_some(){return false}
 match &error.primary{OriginalTurnPrimary::Receiving(original)=>original.quote_refusal.is_none()&&original.allocation_refusal.is_none()&&original.primary.as_ref().is_some_and(|primary|primary.kind==ValueRefusalKind::InvalidValue),_=>false}
}
impl<A:ArtifactApp> ProvisionalReplayWork<A>where A::Mutation:NativeReceive<DslValue>+NativeProvisionalMutation{
 pub(crate) fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub(crate) fn new_original(original:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{source:ProvisionalSourceWork::new_original(original),receiver:None,mutation:None,fold:None,failure:None}),retirement:Default::default(),original,index:0,phase:Phase::Source,closing:false,progress:Default::default(),work:0}}
 pub(crate) fn receive_context(&mut self,context:&ArtifactOwnedContextHandle<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;self.fields.source.receive_context(context,cx)
 }
 fn authenticate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.original.matches(cx)&&!self.closing{cx.original_source_parent()?.validate_borrowed_context(cx)?;Ok(())}else{Err(invalid("provisional replay requires its initiating original source").into())}}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.phase==Phase::Ready}
 pub(crate) fn generation(&self)->Option<u64>{self.fields.source.generation()}
 pub(crate) fn next_demands(&self,host:&FlowHostSnapshot,state:&GenerationPlayState,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid("provisional replay is closing"))}
  match self.phase{
   Phase::Source=>self.fields.source.next_demands(),
   Phase::Project=>Ok(RetainedOwnedProjection::<DslValue>::constructor_demand()),
   Phase::Receive=>self.fields.receiver.as_ref().ok_or_else(||invalid("provisional receiver is absent"))?.next_demands(copy),
   Phase::Take=>self.fields.receiver.as_ref().ok_or_else(||invalid("provisional receiver is absent"))?.take_demands(),
   Phase::CloseReceiver=>self.fields.receiver.as_ref().ok_or_else(||invalid("provisional receiver is absent"))?.next_close_demands(copy),
   Phase::CloseFailure=>self.fields.failure.as_ref().ok_or_else(||invalid("provisional failure owner is absent"))?.selected_demand(copy),
   Phase::CreateFold=>Ok(fixed(2*size_of::<Option<A::Mutation>>()+2*size_of::<Option<NativeProvisionalFold>>())),
   Phase::Apply=>self.fields.fold.as_ref().ok_or_else(||invalid("provisional fold is absent"))?.next_demands(host,state,copy),
   Phase::CloseFold=>self.fields.fold.as_ref().ok_or_else(||invalid("provisional fold is absent"))?.next_close_demands(copy),
   Phase::Next=>Ok(fixed(0)),Phase::Ready=>Ok(Default::default()),
  }
 }
 pub(crate) fn step(&mut self,host:&mut FlowHostSnapshot,state:&mut GenerationPlayState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  match self.phase{
   Phase::Source=>{if self.fields.source.step(cx)?{self.phase=if self.fields.source.len()==0{Phase::Ready}else{Phase::Project}}},
   Phase::Project=>{
    let d=RetainedOwnedProjection::<DslValue>::constructor_demand();let grant=cx.retained_grant();let original=self.original;let index=self.index;
    let fields=&mut *self.fields;let source=&fields.source;let receiver=&mut fields.receiver;
    if admit_original_construction(cx,receiver,construction(d),|_|{let turn=source.project(index,grant);RetainedWorkOutcome{result:turn.result.map(|(projection,p)|(ProvisionalFieldWork::new_original(projection,original),p)),work:turn.work}})?{self.phase=Phase::Receive}
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
    cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|g|receiver.close_step(g))?;Ok(())})?;
    if receiver.terminal_is_empty(){self.fields.receiver=None;self.phase=if self.fields.failure.is_some(){Phase::CloseFailure}else{Phase::CreateFold}}
   },
   Phase::CloseFailure=>{
    let failure=self.fields.failure.as_mut().ok_or_else(||invalid("provisional failure owner is absent"))?;
    let d=failure.selected_demand(cx.retained_grant().maximum_copy_bytes)?;
    cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|g|{let result=failure.step(g);RetainedWorkOutcome{result,work:failure.last_turn_work()}})?;Ok(())})?;
    if failure.terminal_is_empty(){self.fields.failure=None;self.phase=Phase::Next}
   },
   Phase::CreateFold=>{
    let d=self.next_demands(host,state,cx.retained_grant().maximum_copy_bytes)?;let original=self.original;
    let fields=&mut *self.fields;let(mutation,fold)=(&mut fields.mutation,&mut fields.fold);
    if admit_original_construction(cx,fold,construction(d),|_|{
     let input=match mutation.take(){Some(input)=>input,None=>return RetainedWorkOutcome{result:Err(invalid("original provisional mutation is absent")),work:0}};
     let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
     match input.into_fold_original(original){
      Ok(fold)=>RetainedWorkOutcome{result:Ok((fold,p)),work:1},
      Err(input)=>{*mutation=Some(input);RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original provisional domain producer is incomplete").with_retained_progress(p)),work:1}}
     }
    })?{self.phase=Phase::Apply}
   },
   Phase::Apply=>{if self.fields.fold.as_mut().ok_or_else(||invalid("provisional fold is absent"))?.step(host,state,cx)?{self.fields.fold.as_mut().unwrap().begin_close();self.phase=Phase::CloseFold}},
   Phase::CloseFold=>{
    let fold=self.fields.fold.as_mut().ok_or_else(||invalid("provisional fold is absent"))?;let d=fold.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
    cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|g|fold.close_step(g))?;Ok(())})?;
    if fold.terminal_is_empty(){self.fields.fold=None;self.phase=Phase::Next}
   },
   Phase::Next=>{
    let mut accepted=None;let d=fixed(0);let index=&mut self.index;let phase=&mut self.phase;let length=self.fields.source.len();
    admit_original_construction(cx,&mut accepted,construction(d),|_|{*index+=1;*phase=if *index==length{Phase::Ready}else{Phase::Project};RetainedWorkOutcome{result:Ok(((),RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})?;
   },
   Phase::Ready=>{},
  }Ok(self.ready())
 }
 pub(crate) fn begin_close(&mut self){self.closing=true;self.fields.source.begin_close();if let Some(receiver)=self.fields.receiver.as_mut(){receiver.begin_close()}if let Some(fold)=self.fields.fold.as_mut(){fold.begin_close()}}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(receiver)=self.fields.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(fixed(0))}else{receiver.next_close_demands(copy)}}
  if let Some(fold)=self.fields.fold.as_ref(){return if fold.terminal_is_empty(){Ok(fixed(0))}else{fold.next_close_demands(copy)}}
  if let Some(failure)=self.fields.failure.as_ref(){return if failure.terminal_is_empty(){Ok(fixed(0))}else{failure.selected_demand(copy)}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.mutation.is_some(){return self.retirement.next_owner_demands::<A::Mutation>(true,&None,copy)}
  self.fields.source.next_close_demands(copy)
 }
 pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid("provisional replay close was not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(receiver)=self.fields.receiver.as_mut(){if receiver.terminal_is_empty(){self.fields.receiver=None;self.work=1;self.progress.copied_items=1}else{let turn=receiver.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress()}}
  else if let Some(fold)=self.fields.fold.as_mut(){if fold.terminal_is_empty(){self.fields.fold=None;self.work=1;self.progress.copied_items=1}else{let turn=fold.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress()}}
  else if let Some(failure)=self.fields.failure.as_mut(){if failure.terminal_is_empty(){self.fields.failure=None;self.work=1;self.progress.copied_items=1}else{let result=failure.step(grant);self.work=failure.last_turn_work();self.progress=failure.step_progress();result?}}
  else if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);self.work=turn.work;self.progress=turn.result?.progress()}
  else if self.fields.mutation.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.mutation,grant);self.work=turn.work;self.progress=turn.result?.map_or(Default::default(),|step|step.progress())}
  else{let turn=self.fields.source.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress()}
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.fields.receiver.is_none()&&self.fields.mutation.is_none()&&self.fields.fold.is_none()&&self.fields.failure.is_none()&&self.retirement.is_empty()&&self.fields.source.terminal_is_empty()}
}
impl<A:ArtifactApp> Drop for ProvisionalReplayWork<A>where A::Mutation:NativeReceive<DslValue>+NativeProvisionalMutation{
 fn drop(&mut self){assert!(self.terminal_is_empty(),"provisional replay retains original context, leaf, receiver or domain fold");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}
}
