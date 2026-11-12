//! 📝️ Sorted original component indices become exact compact JSON text without serialization mirrors.
use super::{GumballSelectionWork,SelectionKind,invalid};
use crate::source_birth::{Birth,begun,closed,construction,fixed,local_progress,resumed};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneStep,RetirementDemand,RetirementPollExit};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn digits(mut value:u32)->usize{let mut count=1;while value>=10{value/=10;count+=1}count}
pub struct GumballSelectionTextWork{output:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,birth:Birth,original:OriginalSourceBinding,source:Option<usize>,index:usize,bytes:usize,phase:u8,closing:bool}
impl GumballSelectionTextWork{
 pub fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub fn new_original(original:OriginalSourceBinding)->Self{Self{output:ManuallyDrop::new(None),retirement:Default::default(),birth:Default::default(),original,source:None,index:0,bytes:2,phase:0,closing:false}}
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
  self.validate(selection)?;if !self.original.matches(cx)||!selection.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.ready()&&!self.birth.settling(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  let d=self.next_demands(selection,cx.retained_grant().maximum_copy_bytes)?;let grant=cx.retained_grant();
  let mut birth=std::mem::take(&mut self.birth);let settled=birth.run(cx,construction(d),||{
   if !retained_clone_grant_admits(grant,d){return Err(invalid())}
   self.source=Some(selection as*const _ as usize);let progress=local_progress(d);
   (||{
    match self.phase{
     0=>{if let Some(value)=selection.component(self.index){self.bytes=self.bytes.checked_add(digits(value)+usize::from(self.index!=0)).ok_or_else(invalid)?;self.index+=1}else{self.index=0;self.phase=1}},
     1=>{*self.output=Some(String::with_capacity(self.bytes));self.phase=2},
     2=>{self.output.as_mut().ok_or_else(invalid)?.push('[');self.phase=3},
     3=>{let output=self.output.as_mut().ok_or_else(invalid)?;if let Some(mut value)=selection.component(self.index){if self.index!=0{output.push(',')}let mut buffer=[0u8;10];let mut cursor=10;loop{cursor-=1;buffer[cursor]=b'0'+(value%10)as u8;value/=10;if value==0{break}}output.push_str(std::str::from_utf8(&buffer[cursor..]).map_err(|_|invalid())?);self.index+=1}else{output.push(']');self.phase=4}},
     _=>return Err(invalid()),
    }Ok(progress)
   })().map_err(|error:semio_framework_value::ValueError|error.with_retained_progress(progress))
  });self.birth=birth;Ok(settled?&&self.ready())
 }
 pub fn take_into(&mut self,selection:&GumballSelectionWork,slot:&mut Option<String>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(selection)?;if !self.original.matches(cx)||!selection.original.matches(cx)||!self.birth.settling()&&(!self.ready()||slot.is_some()){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;
  let d=self.next_demands(selection,cx.retained_grant().maximum_copy_bytes)?;
  let mut birth=std::mem::take(&mut self.birth);let settled=birth.into_slot(cx,slot,construction(RetirementDemand{capacity_bytes:0,release_bytes:0,depth:1,..d}),||self.output.take().ok_or_else(invalid).map(|output|{self.phase=5;(output,local_progress(d))}));self.birth=birth;settled
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,semio_framework_value::ValueError>{
  if !self.birth.terminal_is_empty(){return self.birth.close_demand(copy)}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.output.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,copy)}Ok(if self.source.is_some(){fixed(size_of::<Option<usize>>())}else{Default::default()})
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}if !self.closing{return Err(invalid().into())}if !self.birth.close_step(cx)?{return Ok(false)}
  let body=cx.retained_grant().maximum_copy_bytes;let d=self.next_close_demands(body)?;closed(cx,d,|effect|self.close_original(body,d,effect))?;Ok(self.terminal_is_empty())
 }
 fn close_original(&mut self,body:usize,d:RetirementDemand,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.retirement.is_empty(){return resumed(&mut self.retirement,body,effect)}if self.output.is_some(){return begun(&mut self.retirement,&mut self.output,effect)}
  if self.source.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.source=None;Ok(RetainedCloneStep::Complete(local_progress(d)))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.retirement.is_empty()&&self.source.is_none()&&self.birth.terminal_is_empty()}
}
impl Drop for GumballSelectionTextWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"gumball selection text retains its component source or original allocation");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.output)}}}}
