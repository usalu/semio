//! 📝️ Sorted original component indices become exact compact JSON text without serialization mirrors.
use super::{GumballSelectionWork,SelectionKind,invalid,fixed};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext,OriginalConstructionDemand,admit_original_construction};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn digits(mut value:u32)->usize{let mut count=1;while value>=10{value/=10;count+=1}count}
pub struct GumballSelectionTextWork{output:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,original:OriginalSourceBinding,source:Option<usize>,index:usize,bytes:usize,phase:u8,closing:bool}
impl GumballSelectionTextWork{
 pub fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub fn new_original(original:OriginalSourceBinding)->Self{Self{output:ManuallyDrop::new(None),retirement:Default::default(),original,source:None,index:0,bytes:2,phase:0,closing:false}}
 fn validate(&self,selection:&GumballSelectionWork)->Result<(),semio_framework_value::ValueError>{if self.closing||selection.result()!=Some(Ok(SelectionKind::Components))||self.source.is_some_and(|source|source!=selection as*const _ as usize){return Err(invalid())}Ok(())}
 pub fn ready(&self)->bool{!self.closing&&self.phase==4}
 pub fn next_demands(&self,selection:&GumballSelectionWork,_copy:usize)->Result<RetirementDemand,semio_framework_value::ValueError>{
  self.validate(selection)?;Ok(match self.phase{
   0=>fixed(4*size_of::<usize>()+size_of::<u32>()),
   1=>RetirementDemand{copy_bytes:size_of::<String>(),capacity_bytes:self.bytes,depth:1,..Default::default()},
   2=>fixed(2*size_of::<usize>()+1),
   3=>fixed(2*size_of::<usize>()+selection.component(self.index).map_or(1,|value|digits(value)+usize::from(self.index!=0))),
   4=>fixed(2*size_of::<Option<String>>()),
   _=>Default::default(),
  })
 }
 pub fn step(&mut self,selection:&GumballSelectionWork,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(selection)?;if !self.original.matches(cx)||!selection.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  let d=self.next_demands(selection,cx.retained_grant().maximum_copy_bytes)?;let mut accepted=None;
  let grant=cx.retained_grant();
  admit_original_construction(cx,&mut accepted,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|_|{let g=grant;
   if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
   self.source=Some(selection as*const _ as usize);let result=(||{
    match self.phase{
     0=>{if let Some(value)=selection.component(self.index){self.bytes=self.bytes.checked_add(digits(value)+usize::from(self.index!=0)).ok_or_else(invalid)?;self.index+=1}else{self.index=0;self.phase=1}},
     1=>{*self.output=Some(String::with_capacity(self.bytes));self.phase=2},
     2=>{self.output.as_mut().ok_or_else(invalid)?.push('[');self.phase=3},
     3=>{let output=self.output.as_mut().ok_or_else(invalid)?;if let Some(mut value)=selection.component(self.index){if self.index!=0{output.push(',')}let mut buffer=[0u8;10];let mut cursor=10;loop{cursor-=1;buffer[cursor]=b'0'+(value%10)as u8;value/=10;if value==0{break}}output.push_str(std::str::from_utf8(&buffer[cursor..]).map_err(|_|invalid())?);self.index+=1}else{output.push(']');self.phase=4}},
     _=>return Err(invalid()),
    }Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))
   })().map_err(|error:semio_framework_value::ValueError|error.with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}));RetainedWorkOutcome{result,work:1}
  })?;Ok(self.ready())
 }
 pub fn take_into(&mut self,selection:&GumballSelectionWork,slot:&mut Option<String>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(selection)?;if !self.original.matches(cx)||!selection.original.matches(cx)||!self.ready()||slot.is_some(){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;
  let d=self.next_demands(selection,cx.retained_grant().maximum_copy_bytes)?;
  admit_original_construction(cx,slot,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|RetainedWorkOutcome{result:self.output.take().ok_or_else(invalid).map(|output|{self.phase=5;(output,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}),work:1})
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,semio_framework_value::ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.output.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,copy)}Ok(if self.source.is_some(){fixed(size_of::<Option<usize>>())}else{Default::default()})}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if !self.retirement.is_empty(){return self.retirement.step_granted(g)}if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,g);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}
  if self.source.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let d=match self.next_close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.source=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.retirement.is_empty()&&self.source.is_none()}
}
impl Drop for GumballSelectionTextWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"gumball selection text retains its component source or original allocation");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.output)}}}}
