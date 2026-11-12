//! 📋️ Native scratch fields copy through the original retained projection and existing clone owner.
use super::retained_scratch_driver::{OriginalScratchWork,drive,funded_turn,local_progress,pay,replied,settled_step};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneOwner,RetainedOwnedProjection,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};
pub struct ProjectedFieldCloneWork<T:RetainedClone>{projection:Option<RetainedOwnedProjection<T>>,cursor:ManuallyDrop<RetainedCloneOwner<T>>,output:ManuallyDrop<Option<T>>,close:Option<ControlledRetirement<T>>,original:semio_framework_job::OriginalSourceBinding,origin:(u64,u64),phase:u8,closing:bool,recover:bool,progress:RetainedCloneProgress}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"projected scratch clone changed receiving phase or original caller")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
impl<T:RetainedClone> ProjectedFieldCloneWork<T>{
 pub fn new(projection:RetainedOwnedProjection<T>,cx:&StepContext<'_>)->Self{Self::new_original(projection,semio_framework_job::OriginalSourceBinding::borrow(cx))}
 pub fn new_original(projection:RetainedOwnedProjection<T>,original:semio_framework_job::OriginalSourceBinding)->Self{Self{projection:Some(projection),cursor:ManuallyDrop::new(Default::default()),output:ManuallyDrop::new(None),close:None,original,origin:(original.operation().0,original.generation().0),phase:0,closing:false,recover:false,progress:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==4}
 pub fn next_demands(&self,copy:usize,work:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid())}match self.phase{
   0=>self.cursor.advance_demands(self.projection.as_ref().ok_or_else(invalid)?.borrow()?,copy,work),
   1=>self.cursor.take_demand(),
   2=>self.cursor.selected_close_demand(copy),
   3=>self.projection.as_ref().ok_or_else(invalid)?.next_close_demand(copy),
   4=>Ok(fixed(2*size_of::<Option<T>>())),
   _=>Err(invalid()),
  }
 }
 pub fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();let result=self.advance_original(effect).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 fn advance_original(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let grant=effect.grant();let body=grant.maximum_copy_bytes;let maximum=effect.maximum_work_units();
  if self.ready(){return Ok(true)}if !retained_clone_grant_admits(grant,self.next_demands(body,maximum)?){return Ok(false)}
  match self.phase{
   0=>{
    let source=self.projection.as_ref().unwrap().borrow()?;
    if !self.recover&&effect.remaining_work_units()?<self.cursor.advance_work_demand(source,body,maximum)?{return Ok(false)}
    let cursor=&mut*self.cursor;
    let reply=if self.recover{replied(effect,|effect,reply|cursor.recover_advance_into(source,body,maximum,effect,reply))}else{replied(effect,|effect,reply|cursor.advance_into(source,body,maximum,effect,reply))}?;
    match reply{Some(result)=>{self.recover=false;let step=result.map_err(OriginalRetirementFailure::into_value_error)?;self.progress=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){self.phase=1}},None=>self.recover=true}
   },
   1=>{
    if effect.remaining_work_units()?<1{return Ok(false)}
    let cursor=&mut*self.cursor;let output=&mut*self.output;
    let step=replied(effect,|effect,reply|cursor.take(output,effect,reply))?.ok_or_else(invalid)??;self.progress=step.progress();if self.output.is_some(){self.phase=2}
   },
   2=>{
    if effect.remaining_work_units()?<self.cursor.selected_close_work_demand(body)?{return Ok(false)}
    let cursor=&mut*self.cursor;
    let reply=if self.recover{replied(effect,|effect,reply|cursor.recover_close_into(body,effect,reply))}else{replied(effect,|effect,reply|cursor.close_into(body,effect,reply))}?;
    match reply{Some(result)=>{self.recover=false;self.progress=result.map_err(OriginalRetirementFailure::into_value_error)?.progress();if self.cursor.terminal_is_empty(){self.phase=3}},None=>self.recover=true}
   },
   3=>{let projection=self.projection.as_mut().unwrap();self.progress=settled_step(projection.close_step(body,effect))?.progress();if projection.terminal_is_empty(){self.projection=None;self.phase=4}},
   _=>return Err(invalid()),
  }Ok(self.ready())
 }
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{drive(self,&mut (),cx)}
 pub fn take_into(&mut self,output:&mut Option<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.origin!=(cx.operation().0,cx.generation().0)||!self.original.matches(cx)||!self.ready()||output.is_some(){return Err(invalid().into())}
  let demand=self.next_demands(cx.retained_grant().maximum_copy_bytes,0)?;
  let step=funded_turn(cx,|effect|{
   let result=if !retained_clone_grant_admits(effect.grant(),demand){Ok(RetainedCloneStep::Progress(Default::default()))}else{match pay(effect,1){Ok(true)=>self.output.take().ok_or_else(invalid).map(|value|{*output=Some(value);self.phase=5;RetainedCloneStep::Complete(local_progress(demand))}),Ok(false)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)}};
   RetainedWorkOutcome{result,work:effect.performed_work()}
  })?;
  Ok(step.is_some()&&output.is_some())
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.cursor.terminal_is_empty(){return self.cursor.selected_close_demand(copy)}
  if let Some(projection)=self.projection.as_ref(){return projection.next_close_demand(copy)}
  if let Some(close)=self.close.as_ref(){return close.selected_demand(copy)}
  Ok(if self.output.is_some(){fixed(size_of::<Option<T>>())}else{Default::default()})
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
  self.progress=Default::default();let result=self.close_original(body,effect);semio_framework_value::RetirementPollExit::settle(effect,result)
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,semio_framework_value::RetirementPollExit>{
  use semio_framework_value::RetirementPollExit;
  if !self.closing{return Err(invalid().into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(effect.grant(),self.next_close_demands(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.cursor.terminal_is_empty(){
   if effect.remaining_work_units()?<self.cursor.selected_close_work_demand(body)?{return Ok(RetainedCloneStep::Progress(Default::default()))}
   let cursor=&mut*self.cursor;
   let reply=if self.recover{replied(effect,|effect,reply|cursor.recover_close_into(body,effect,reply))}else{replied(effect,|effect,reply|cursor.close_into(body,effect,reply))}?;
   match reply{Some(result)=>{self.recover=false;self.progress=result.map_err(RetirementPollExit::from)?.progress()},None=>{self.recover=true;return Err(RetirementPollExit::Pending(effect.performed_work()))}}
  }
  else if let Some(projection)=self.projection.as_mut(){self.progress=RetirementPollExit::resume(projection.close_step(body,effect))?.progress();if projection.terminal_is_empty(){self.projection=None}}
  else if let Some(close)=self.close.as_mut(){self.progress=RetirementPollExit::resume(close.step(body,effect))?.progress();if close.terminal_is_empty(){self.close=None}}
  else if let Some(output)=self.output.take(){self.close=Some(ControlledRetirement::new(output).unwrap_or_else(|_|unreachable!("native cloned field declares controlled retirement")));self.progress.copied_items=1}
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.projection.is_none()&&self.cursor.terminal_is_empty()&&self.output.is_none()&&self.close.is_none()}
}
impl<T:RetainedClone> OriginalScratchWork<()> for ProjectedFieldCloneWork<T>{
 fn origin(&self)->(u64,u64){self.origin}
 fn source_binding(&self)->semio_framework_job::OriginalSourceBinding{self.original}
 fn ready(&self)->bool{ProjectedFieldCloneWork::ready(self)}
 fn advancing_retirement(&self)->bool{matches!(self.phase,2|3)}
 fn next_demands(&self,_source:&(),copy:usize,work:u64)->Result<RetirementDemand,ValueError>{ProjectedFieldCloneWork::next_demands(self,copy,work)}
 fn advance(&mut self,_source:&mut (),effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{ProjectedFieldCloneWork::advance(self,effect)}
 fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
impl<T:RetainedClone> Drop for ProjectedFieldCloneWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"projected native field retains its source, clone cursor or output");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.output)}}}}
