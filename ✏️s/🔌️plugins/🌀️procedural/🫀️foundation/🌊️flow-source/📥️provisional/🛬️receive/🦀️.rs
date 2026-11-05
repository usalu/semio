//! 🛬️ Provisional native receiving consumes genuine field projections under the original decoder.
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalTurnPrimary,OriginalConstructionDemand,admit_original_construction};
use semio_framework_value::{DslValue,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedOwnedProjection,retained_clone_grant_admits};
use semio_framework_pack_json::native_decoding::owned::{NativeReceive,NativeReceivingOriginal};
use std::mem::ManuallyDrop;
pub struct ProvisionalFieldWork<T:NativeReceive<DslValue>>{receiver:ManuallyDrop<Option<NativeReceivingOriginal<DslValue,T>>>,original_caller:semio_framework_job::OriginalSourceBinding,operation:(u64,u64),closing:bool}
impl<T:NativeReceive<DslValue>> ProvisionalFieldWork<T>{
 pub fn new(source:RetainedOwnedProjection<DslValue>,cx:&StepContext<'_>)->Self{Self::new_original(source,semio_framework_job::OriginalSourceBinding::borrow(cx))}
 pub fn new_original(source:RetainedOwnedProjection<DslValue>,original_caller:semio_framework_job::OriginalSourceBinding)->Self{Self{receiver:ManuallyDrop::new(Some(NativeReceivingOriginal::from_projection(source))),original_caller,operation:(original_caller.operation().0,original_caller.generation().0),closing:false}}
 fn authenticate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.operation==(cx.operation().0,cx.generation().0)&&self.original_caller.matches(cx)&&!self.closing{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiving requires its initiating caller").into())}}
 pub fn ready(&self)->bool{!self.closing&&self.receiver.as_ref().is_some_and(NativeReceivingOriginal::output_ready)}
 pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.receiver.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiver is absent"))?.demands(copy)}
 pub fn take_demands(&self)->Result<RetirementDemand,ValueError>{self.receiver.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiver is absent"))?.take_demand()}
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;
  for _ in 0..512{
   if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
   let grant=cx.retained_grant();let d=self.next_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(false)}
   let mut progress=RetainedCloneProgress::default();let mut work=0;
   let result:Result<(),OriginalTurnFailure>=cx.with_native_receiving(|control|{
    let turn=self.receiver.as_mut().unwrap().advance(control,grant.maximum_copy_bytes,grant);work=turn.work;
    progress=match turn.result{Ok(step)=>step.progress(),Err(error)=>{progress=error.progress;return Err(OriginalTurnFailure{effects:progress,primary:OriginalTurnPrimary::Receiving(error),receipt_refusal:None,context_refusal:None})}};Ok(())
   });
   let receipt=cx.receiving_issued_grant().map(|grant|cx.consume_receiving_retained(grant,progress)).transpose();cx.consume_fuel(work);
   match(result,receipt){(Err(mut error),Err(receipt))=>{error.context_refusal=Some(receipt);return Err(error)},(Err(error),_)=>return Err(error),(_,Err(error))=>return Err(error.into()),_=>{}}
   if progress==Default::default(){return Ok(self.ready())}
  }Ok(self.ready())
 }
 pub fn take_into(&mut self,output:&mut Option<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;let receiver=self.receiver.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiver is absent"))?;let demand=receiver.take_demand()?;
  admit_original_construction(cx,output,OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|grant|{
   let turn=receiver.take(grant);RetainedWorkOutcome{result:turn.result.and_then(|output|output.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiver omitted its funded output"))),work:turn.work}
  })
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{match self.receiver.as_ref(){Some(receiver)if !receiver.terminal_is_empty()=>receiver.close_demands(copy),Some(_)=>Ok(RetirementDemand{depth:1,..Default::default()}),None=>Ok(Default::default())}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiver close was not begun")),work:0}}
  let Some(receiver)=self.receiver.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  if !receiver.terminal_is_empty(){return receiver.close_step(grant)}
  if grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  self.receiver.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.receiver.is_none()}
}
impl<T:NativeReceive<DslValue>> Drop for ProvisionalFieldWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"provisional native receiver retains its original field or output");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.receiver)}}}}
