//! 📋️ Native scratch fields copy through the original retained projection and existing clone owner.
use super::retained_scratch_driver::{OriginalScratchWork,drive};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneOwner,RetainedOwnedProjection,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};
pub(crate) struct ProjectedFieldCloneWork<T:RetainedClone>{projection:Option<RetainedOwnedProjection<T>>,cursor:ManuallyDrop<RetainedCloneOwner<T>>,output:ManuallyDrop<Option<T>>,close:Option<ControlledRetirement<T>>,original:semio_framework_job::OriginalSourceBinding,origin:(u64,u64),phase:u8,closing:bool,progress:RetainedCloneProgress,work:u64}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"projected scratch clone changed receiving phase or original caller")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
impl<T:RetainedClone> ProjectedFieldCloneWork<T>{
 pub(crate) fn new(projection:RetainedOwnedProjection<T>,cx:&StepContext<'_>)->Self{Self::new_original(projection,semio_framework_job::OriginalSourceBinding::borrow(cx))}
 pub(crate) fn new_original(projection:RetainedOwnedProjection<T>,original:semio_framework_job::OriginalSourceBinding)->Self{Self{projection:Some(projection),cursor:ManuallyDrop::new(Default::default()),output:ManuallyDrop::new(None),close:None,original,origin:(original.operation().0,original.generation().0),phase:0,closing:false,progress:Default::default(),work:0}}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.phase==4}
 pub(crate) fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid())}match self.phase{
   0=>self.cursor.advance_demands(self.projection.as_ref().ok_or_else(invalid)?.borrow()?,copy),
   1=>self.cursor.take_demand(),
   2=>self.cursor.selected_close_demand(copy),
   3=>self.projection.as_ref().ok_or_else(invalid)?.next_close_demand(copy),
   4=>Ok(fixed(2*size_of::<Option<T>>())),
   _=>Err(invalid()),
  }
 }
 pub(crate) fn advance(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;let result=self.advance_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn advance_original(&mut self,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  if self.ready(){return Ok(true)}if !retained_clone_grant_admits(grant,self.next_demands(grant.maximum_copy_bytes)?){return Ok(false)}
  match self.phase{
   0=>{let turn=self.cursor.advance(self.projection.as_ref().unwrap().borrow()?,grant.maximum_copy_bytes,grant);self.work=turn.work;let step=turn.result?;self.progress=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){self.phase=1}},
   1=>{let turn=self.cursor.take(grant);self.work=turn.work;if let Some((value,p))=turn.result?{*self.output=Some(value);self.progress=p;self.phase=2}},
   2=>{let turn=self.cursor.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress();if self.cursor.terminal_is_empty(){self.phase=3}},
   3=>{let projection=self.projection.as_mut().unwrap();let turn=projection.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress();if projection.terminal_is_empty(){self.projection=None;self.phase=4}},
   _=>return Err(invalid()),
  }Ok(self.ready())
 }
 pub(crate) fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{drive(self,&mut (),cx)}
 pub(crate) fn take_into(&mut self,output:&mut Option<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.origin!=(cx.operation().0,cx.generation().0)||!self.original.matches(cx)||!self.ready()||output.is_some(){return Err(invalid().into())}
  let d=self.next_demands(cx.retained_grant().maximum_copy_bytes)?;let g=cx.retained_grant();
  admit_original_construction(cx,output,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|_|{
   if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
   let result=self.output.take().ok_or_else(invalid).map(|value|{self.phase=5;(value,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})});RetainedWorkOutcome{result,work:1}
  })
 }
 pub(crate) fn begin_close(&mut self){self.closing=true}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.cursor.terminal_is_empty(){return self.cursor.selected_close_demand(copy)}
  if let Some(projection)=self.projection.as_ref(){return projection.next_close_demand(copy)}
  if let Some(close)=self.close.as_ref(){return close.selected_demand(copy)}
  Ok(if self.output.is_some(){fixed(size_of::<Option<T>>())}else{Default::default()})
 }
 pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.cursor.terminal_is_empty(){let turn=self.cursor.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress()}
  else if let Some(projection)=self.projection.as_mut(){let turn=projection.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress();if projection.terminal_is_empty(){self.projection=None}}
  else if let Some(close)=self.close.as_mut(){let turn=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();turn?;if close.terminal_is_empty(){self.close=None}}
  else if let Some(output)=self.output.take(){self.close=Some(ControlledRetirement::new(output).unwrap_or_else(|_|unreachable!("native cloned field declares controlled retirement")));self.work=1;self.progress.copied_items=1}
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.projection.is_none()&&self.cursor.terminal_is_empty()&&self.output.is_none()&&self.close.is_none()}
}
impl<T:RetainedClone> OriginalScratchWork<()> for ProjectedFieldCloneWork<T>{
 fn origin(&self)->(u64,u64){self.origin}
 fn source_binding(&self)->semio_framework_job::OriginalSourceBinding{self.original}
 fn ready(&self)->bool{ProjectedFieldCloneWork::ready(self)}
 fn advancing_retirement(&self)->bool{matches!(self.phase,2|3)}
 fn next_demands(&self,_source:&(),copy:usize)->Result<RetirementDemand,ValueError>{ProjectedFieldCloneWork::next_demands(self,copy)}
 fn advance(&mut self,_source:&mut (),grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{ProjectedFieldCloneWork::advance(self,grant)}
 fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
impl<T:RetainedClone> Drop for ProjectedFieldCloneWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"projected native field retains its source, clone cursor or output");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.output)}}}}
