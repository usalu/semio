//! 🪪️ Deterministic transform identifiers copy original UTF8 in bounded funded windows.
use crate::source_birth::{Birth,begun,closed,construction,fixed,local_progress,resumed};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneStep,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GumballOperation{Translate,Rotate,Scale}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct GumballNodeIdentity{pub operation:GumballOperation,pub component:bool,pub ordinal:u64}
impl GumballNodeIdentity{
 pub fn suffix(self)->&'static str{match(self.operation,self.component){(GumballOperation::Translate,false)=>"__gumball_translate",(GumballOperation::Rotate,false)=>"__gumball_rotate",(GumballOperation::Scale,false)=>"__gumball_scale",(GumballOperation::Translate,true)=>"__translateComponents",(GumballOperation::Rotate,true)=>"__rotateComponents",(GumballOperation::Scale,true)=>"__scaleComponents"}}
 pub fn neuron_kind(self,mesh:bool)->&'static str{match(self.operation,self.component,mesh){(GumballOperation::Translate,true,_)=>"brep.mesh.translateComponents",(GumballOperation::Rotate,true,_)=>"brep.mesh.rotateComponents",(GumballOperation::Scale,true,_)=>"brep.mesh.scaleComponents",(GumballOperation::Translate,false,true)=>"brep.mesh.translate",(GumballOperation::Rotate,false,true)=>"brep.mesh.rotate",(GumballOperation::Scale,false,true)=>"brep.mesh.scale",(GumballOperation::Translate,false,false)=>"brep.xform.translate",(GumballOperation::Rotate,false,false)=>"brep.xform.rotate",(GumballOperation::Scale,false,false)=>"brep.xform.scale"}}
}
pub struct GumballNodeIdentityWork{output:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,birth:Birth,original:OriginalSourceBinding,identity:GumballNodeIdentity,source:Option<(usize,usize)>,ordinal:[u8;20],start:usize,byte:usize,phase:u8,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"gumball node identity changed its original source, caller or ordinal")}
fn digits(mut value:u64)->usize{let mut length=1;while value>=10{value/=10;length+=1}length}
fn end(source:&str,byte:usize)->usize{let mut end=(byte+source.len().saturating_sub(byte).min(64)).min(source.len());while !source.is_char_boundary(end){end-=1}end}
impl GumballNodeIdentityWork{
 pub fn new(identity:GumballNodeIdentity,cx:&StepContext<'_>)->Self{Self::new_original(identity,OriginalSourceBinding::borrow(cx))}
 pub fn new_original(identity:GumballNodeIdentity,original:OriginalSourceBinding)->Self{Self{output:ManuallyDrop::new(None),retirement:Default::default(),birth:Default::default(),original,identity,source:None,ordinal:[0;20],start:20,byte:0,phase:0,closing:false}}
 fn validate(&self,source:&str)->Result<(),ValueError>{if self.closing||self.identity.ordinal==1||!self.identity.component&&self.identity.ordinal!=0||self.source.is_some_and(|bound|bound!=(source.as_ptr()as usize,source.len())){return Err(invalid())}Ok(())}
 pub fn ready(&self)->bool{!self.closing&&self.phase==4}
 pub fn next_demands(&self,source:&str,_copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;Ok(match self.phase{
   0=>RetirementDemand{copy_bytes:4*size_of::<usize>()+size_of::<String>()+20,capacity_bytes:source.len().checked_add(self.identity.suffix().len()).and_then(|length|length.checked_add(if self.identity.ordinal==0{0}else{1+digits(self.identity.ordinal)})).ok_or_else(invalid)?,depth:1,..Default::default()},
   1=>fixed(2*size_of::<usize>()+end(source,self.byte)-self.byte),
   2=>fixed(size_of::<usize>()+self.identity.suffix().len()),
   3=>fixed(size_of::<usize>()+usize::from(self.identity.ordinal!=0)*(1+20-self.start)),
   4=>fixed(2*size_of::<Option<String>>()),
   _=>Default::default(),
  })
 }
 pub fn step(&mut self,source:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(source)?;if !self.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.ready()&&!self.birth.settling(){return Ok(true)}if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
  let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;let grant=cx.retained_grant();
  let mut birth=std::mem::take(&mut self.birth);let settled=birth.run(cx,construction(d),||{
   if !retained_clone_grant_admits(grant,d){return Err(invalid())}
   let progress=local_progress(d);
   (||{
    match self.phase{
     0=>{self.source=Some((source.as_ptr()as usize,source.len()));if self.identity.ordinal!=0{let mut value=self.identity.ordinal;loop{self.start-=1;self.ordinal[self.start]=b'0'+(value%10)as u8;value/=10;if value==0{break}}}*self.output=Some(String::with_capacity(d.capacity_bytes));self.phase=1},
     1=>{let next=end(source,self.byte);self.output.as_mut().ok_or_else(invalid)?.push_str(&source[self.byte..next]);self.byte=next;if next==source.len(){self.phase=2}},
     2=>{self.output.as_mut().ok_or_else(invalid)?.push_str(self.identity.suffix());self.phase=3},
     3=>{if self.identity.ordinal!=0{let output=self.output.as_mut().ok_or_else(invalid)?;output.push('_');output.push_str(std::str::from_utf8(&self.ordinal[self.start..]).map_err(|_|invalid())?)}self.phase=4},
     _=>return Err(invalid()),
    }Ok(progress)
   })().map_err(|error:ValueError|error.with_retained_progress(progress))
  });self.birth=birth;Ok(settled?&&self.ready())
 }
 pub fn take_into(&mut self,source:&str,slot:&mut Option<String>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(source)?;if !self.original.matches(cx)||!self.birth.settling()&&(!self.ready()||slot.is_some()){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;
  let mut birth=std::mem::take(&mut self.birth);let settled=birth.into_slot(cx,slot,construction(RetirementDemand{capacity_bytes:0,release_bytes:0,depth:1,..d}),||self.output.take().ok_or_else(invalid).map(|output|{self.phase=5;(output,local_progress(d))}));self.birth=birth;settled
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.birth.terminal_is_empty(){return self.birth.close_demand(copy)}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.output.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,copy)}Ok(if self.source.is_some(){fixed(size_of::<Option<(usize,usize)>>())}else{Default::default()})
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
impl Drop for GumballNodeIdentityWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"gumball node identity retains its original UTF8 allocation or source binding");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.output)}}}}
#[cfg(test)]
#[path="🧪️.rs"]
mod tests;

impl From<&crate::GumballMotion> for GumballOperation{fn from(motion:&crate::GumballMotion)->Self{match motion{crate::GumballMotion::Translate(_)=>Self::Translate,crate::GumballMotion::Rotate(_)=>Self::Rotate,crate::GumballMotion::Scale(_)=>Self::Scale}}}
