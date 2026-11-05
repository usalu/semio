//! 📋️ Prevalidated procedural row edits move original payloads through one funded scratch backing.
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,original_vector::OriginalVectorCursor,controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum ScratchRowsKind{Insert,Replace,Remove}
pub struct ScratchRowsWork<T:RetireOwned>{kind:ScratchRowsKind,at:usize,index:usize,length:usize,field:Option<usize>,payload:ManuallyDrop<Option<T>>,pending:ManuallyDrop<Option<T>>,original:Option<OriginalVectorCursor<T>>,output:ManuallyDrop<Vec<T>>,displaced:Option<ControlledRetirement<T>>,output_close:Option<ControlledRetirement<Vec<T>>>,phase:u8,closing:bool,progress:RetainedCloneProgress,work:u64}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn metadata()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
impl<T:RetireOwned> ScratchRowsWork<T>{
 pub fn new(kind:ScratchRowsKind,at:usize,payload:Option<T>)->Result<Self,(ValueError,Option<T>)>{if !T::controlled_retirement_supported(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"scratch row payload requires controlled native retirement"),payload))}Ok(Self{kind,at,index:0,length:0,field:None,payload:ManuallyDrop::new(payload),pending:ManuallyDrop::new(None),original:None,output:ManuallyDrop::new(Vec::new()),displaced:None,output_close:None,phase:0,closing:false,progress:Default::default(),work:0})}
 pub fn advancing_retirement(&self)->bool{!self.closing&&matches!(self.phase,5|6)}
 pub fn ready(&self)->bool{!self.closing&&self.phase==8}
 fn validate(&self,rows:&Vec<T>)->Result<(),ValueError>{
  if self.closing{return Err(invalid("scratch row edit is closing"))}if self.kind==ScratchRowsKind::Remove&&self.payload.is_some(){return Err(invalid("scratch row removal has an insertion payload"))}
  if self.phase==0{if self.kind!=ScratchRowsKind::Insert&&self.at>=rows.len(){return Err(invalid("scratch row target was not prevalidated"))}if self.kind!=ScratchRowsKind::Remove&&self.payload.is_none(){return Err(invalid("scratch row insertion payload is absent"))}}
  else if self.field!=Some(rows as *const Vec<T> as usize)||self.phase<8&&(!rows.is_empty()||rows.capacity().saturating_mul(size_of::<T>())!=0){return Err(invalid("scratch row field changed during original custody"))}Ok(())
 }
 pub fn next_demands(&self,rows:&Vec<T>,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(rows)?;match self.phase{0=>Ok(OriginalVectorCursor::<T>::constructor_demand()),1=>Ok(RetirementDemand{capacity_bytes:self.length.checked_mul(size_of::<T>()).ok_or_else(||invalid("scratch row backing extent overflow"))?,..metadata()}),2 if self.kind==ScratchRowsKind::Insert&&self.index==self.at&&self.payload.is_some()=>Ok(RetirementDemand{copy_bytes:size_of::<T>(),..metadata()}),4 if self.kind!=ScratchRowsKind::Remove||self.index!=self.at=>Ok(RetirementDemand{copy_bytes:size_of::<T>(),..metadata()}),3=>Ok(self.original.as_ref().ok_or_else(||invalid("scratch row original is absent"))?.next_take_demand()),5=>self.displaced.as_ref().ok_or_else(||invalid("scratch displaced owner is absent"))?.selected_demand(copy),6=>self.original.as_ref().ok_or_else(||invalid("scratch row original is absent"))?.next_demand(copy),_=>Ok(metadata())}
 }
 pub fn advance(&mut self,rows:&mut Vec<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{self.progress=Default::default();self.work=0;let result=self.advance_original(rows,grant).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}}
 fn advance_original(&mut self,rows:&mut Vec<T>,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  let d=self.next_demands(rows,grant.maximum_copy_bytes)?;if self.ready(){return Ok(true)}if !permits(grant,d){return Ok(false)}
  self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   0=>{self.field=Some(rows as *const Vec<T> as usize);let old=rows.len();if self.kind==ScratchRowsKind::Insert{self.at=self.at.min(old)}self.length=match self.kind{ScratchRowsKind::Insert=>old.checked_add(1).ok_or_else(||invalid("scratch row count overflow"))?,ScratchRowsKind::Replace=>old,ScratchRowsKind::Remove=>old-1};let turn=OriginalVectorCursor::admit(std::mem::take(rows),grant);self.work=turn.work;self.progress=Default::default();match turn.result{Ok((original,p))=>{self.original=Some(original);self.progress=p;self.phase=1},Err((error,original))=>{*rows=original;self.field=None;return Err(error)}}},
   1=>{self.output.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"scratch row backing birth refused"))?;self.progress.retained_capacity_bytes=self.output.capacity().checked_mul(size_of::<T>()).ok_or_else(||invalid("scratch row actual backing overflow"))?;self.phase=2},
   2=>{if self.kind==ScratchRowsKind::Insert&&self.index==self.at&&self.payload.is_some(){self.output.push(self.payload.take().unwrap())}else if self.original.as_ref().unwrap().remaining().is_empty(){self.original.as_mut().unwrap().begin_close();self.phase=6}else{self.phase=3}},
   3=>{let turn=self.original.as_mut().unwrap().take_front(grant);self.work=turn.work;self.progress=Default::default();if let Some((value,p))=turn.result?{*self.pending=Some(value);self.progress=p;self.phase=4}},
   4=>{let old=self.pending.take().ok_or_else(||invalid("scratch row handoff omitted its original"))?;if self.index==self.at&&self.kind!=ScratchRowsKind::Insert{self.displaced=Some(ControlledRetirement::new(old).unwrap_or_else(|_|unreachable!("scratch rows have native retirement")));if self.kind==ScratchRowsKind::Replace{self.output.push(self.payload.take().ok_or_else(||invalid("scratch replacement payload is absent"))?)}self.phase=5}else{self.output.push(old);self.phase=2}self.index+=1},
   5=>{let owner=self.displaced.as_mut().unwrap();let result=owner.step(grant);self.work=owner.last_turn_work();self.progress=owner.step_progress();result?;if owner.terminal_is_empty(){self.displaced=None;self.phase=2}},
   6=>{let original=self.original.as_mut().unwrap();let turn=original.step(grant);self.work=turn.work;self.progress=turn.result?.progress();if original.terminal_is_empty(){self.original=None;self.phase=7}},
   7=>{if self.output.len()!=self.length{return Err(invalid("scratch row output count differs from its original plan"))}*rows=std::mem::take(&mut self.output);self.phase=8;return Ok(true)},
   _=>return Err(invalid("scratch row phase is invalid")),
  }Ok(false)
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(original)=self.original.as_mut(){original.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(displaced)=self.displaced.as_ref(){return displaced.selected_demand(copy)}if self.pending.is_some()||self.payload.is_some(){return Ok(metadata())}if let Some(original)=self.original.as_ref(){return original.next_demand(copy)}if let Some(output)=self.output_close.as_ref(){return output.selected_demand(copy)}if !self.output.is_empty()||self.output.capacity().saturating_mul(size_of::<T>())>0{return Ok(metadata())}Ok(Default::default())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}}
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid("scratch row close was not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !permits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(owner)=self.displaced.as_mut(){let result=owner.step(grant);self.work=owner.last_turn_work();self.progress=owner.step_progress();result?;if owner.terminal_is_empty(){self.displaced=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if self.pending.is_some()||self.payload.is_some(){let value=self.pending.take().or_else(||self.payload.take()).unwrap();self.displaced=Some(ControlledRetirement::new(value).unwrap_or_else(|_|unreachable!("scratch payload has native retirement")));self.work=1;self.progress.copied_items=1;return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(original)=self.original.as_mut(){let turn=original.step(grant);self.work=turn.work;self.progress=turn.result?.progress();if original.terminal_is_empty(){self.original=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(owner)=self.output_close.as_mut(){let result=owner.step(grant);self.work=owner.last_turn_work();self.progress=owner.step_progress();result?;if owner.terminal_is_empty(){self.output_close=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  let output=std::mem::take(&mut self.output);self.output_close=Some(ControlledRetirement::new(output).unwrap_or_else(|_|unreachable!("scratch vector has native retirement")));self.work=1;self.progress.copied_items=1;Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.original.is_none()&&self.displaced.is_none()&&self.output_close.is_none()&&self.pending.is_none()&&self.payload.is_none()&&self.output.is_empty()&&self.output.capacity().saturating_mul(size_of::<T>())==0}
}
impl<T:RetireOwned> Drop for ScratchRowsWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"scratch row edit retains original rows or output backing");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.payload);ManuallyDrop::drop(&mut self.output)}}}}
