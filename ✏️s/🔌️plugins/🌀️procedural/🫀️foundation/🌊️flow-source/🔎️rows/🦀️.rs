//! 🔎️ Scratch identity lookup compares at most 64 original row-key byte pairs per individually funded event.
use super::retained_scratch_driver::pay;
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand,RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind};
#[derive(Clone,Copy,PartialEq,Eq)]
struct Binding{field:usize,rows:usize,length:usize,key:usize,key_length:usize,projection:usize}
pub struct ScratchRowScan{binding:Option<Binding>,index:usize,byte:usize,result:Option<usize>,done:bool,progress:RetainedCloneProgress}
impl ScratchRowScan{
 pub fn new()->Self{Self{binding:None,index:0,byte:0,result:None,done:false,progress:Default::default()}}
 fn binding<T>(rows:&Vec<T>,key:&str,project:fn(&T)->&str)->Binding{Binding{field:rows as *const Vec<T>as usize,rows:rows.as_ptr()as usize,length:rows.len(),key:key.as_ptr()as usize,key_length:key.len(),projection:project as *const () as usize}}
 fn validate<T>(&self,rows:&Vec<T>,key:&str,project:fn(&T)->&str)->Result<(),ValueError>{if self.binding.is_some_and(|binding|binding!=Self::binding(rows,key,project)){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"scratch row lookup changed its original keys"))}else{Ok(())}}
 pub fn ready(&self)->bool{self.done}
 pub fn result(&self)->Option<usize>{self.result}
 pub fn next_demands<T>(&self,rows:&Vec<T>,key:&str,project:fn(&T)->&str)->Result<RetirementDemand,ValueError>{
  self.validate(rows,key,project)?;let copy_bytes=if !self.done{rows.get(self.index).map(project).map_or(0,|candidate|if candidate.len()==key.len(){key.len().saturating_sub(self.byte).min(64)*2}else{0})}else{0};Ok(RetirementDemand{copy_bytes,depth:usize::from(!self.done),..Default::default()})
 }
 pub fn advance<T>(&mut self,rows:&Vec<T>,key:&str,project:fn(&T)->&str,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();let result=(||{
   let demand=self.next_demands(rows,key,project)?;if self.done{return Ok(true)}let grant=effect.grant();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(false)}if !pay(effect,1)?{return Ok(false)}
   self.binding=Some(Self::binding(rows,key,project));self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
   let Some(candidate)=rows.get(self.index).map(project)else{self.done=true;return Ok(true)};
   if candidate.len()!=key.len(){self.index+=1;self.byte=0;return Ok(false)}
   if self.byte<key.len(){let end=self.byte+demand.copy_bytes/2;let different=candidate.as_bytes()[self.byte..end].iter().zip(&key.as_bytes()[self.byte..end]).fold(false,|different,(a,b)|different|(*a!=*b));if different{self.index+=1;self.byte=0;return Ok(false)}self.byte=end}
   if self.byte==key.len(){self.result=Some(self.index);self.done=true;return Ok(true)}Ok(false)
  })();RetainedWorkOutcome{result:result.map_err(|error:ValueError|error.with_retained_progress(self.progress)),work:effect.performed_work()}
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
