//! 🔎️ Borrowed current contribution discovery keeps domain selection at its caller.
use crate::conformance::providers::{ProviderAdmission,ProviderInput,ProviderOperationControl,ProviderRefusal,ProviderTarget};
pub struct ConformanceContributionInput<'a> {pub eligible:bool,pub name:&'a str,pub manifest:&'a str,pub declaration:Option<&'a ProviderInput<'a>>}
#[derive(Default)]
pub struct ConformanceDiscovery {completed:usize,current:usize,admitted:usize,terminal:bool}
struct ForwardControl<'a,C> {control:&'a mut C,completed:&'a mut usize}
impl<C:ProviderOperationControl> ProviderOperationControl for ForwardControl<'_,C> {
 fn checkpoint(&mut self,_completed:usize)->Result<(),ProviderRefusal>{self.control.checkpoint(*self.completed)?;*self.completed=self.completed.checked_add(1).ok_or(ProviderRefusal::Budget)?;Ok(())}
}
impl ConformanceDiscovery {
 pub fn completed(&self)->usize{self.completed}
 pub fn admitted(&self)->usize{self.admitted}
 pub fn current(&self)->usize{self.current}
 fn step<C:ProviderOperationControl>(&mut self,control:&mut C)->Result<(),ProviderRefusal>{control.checkpoint(self.completed)?;self.completed=self.completed.checked_add(1).ok_or(ProviderRefusal::Budget)?;Ok(())}
 fn same<C:ProviderOperationControl>(&mut self,left:&str,right:&str,control:&mut C)->Result<bool,ProviderRefusal>{self.step(control)?;if left.len()!=right.len(){return Ok(false);}for(left,right)in left.bytes().zip(right.bytes()){self.step(control)?;if left!=right{return Ok(false);}}Ok(true)}
 pub fn discover<'a,C:ProviderOperationControl,R:FnMut(&'a ConformanceContributionInput<'a>,ProviderTarget<'a>)>(&mut self,inputs:&'a[ConformanceContributionInput<'a>],control:&mut C,receive:&mut R)->Result<(),ProviderRefusal>{
  if self.terminal{return Err(ProviderRefusal::InvalidDeclaration);}self.terminal=true;
  for(index,input)in inputs.iter().enumerate(){
   self.current=index;self.step(control)?;if !input.eligible{continue;}
   if input.name.is_empty()||input.manifest.is_empty(){return Err(ProviderRefusal::InvalidDeclaration);}
   for byte in input.name.bytes(){self.step(control)?;if !byte.is_ascii_alphanumeric()&&byte!=b'_'&&byte!=b'-'{return Err(ProviderRefusal::InvalidDeclaration);}}
   for _ in input.manifest.bytes(){self.step(control)?;}
   for previous in &inputs[..index]{self.step(control)?;if previous.eligible&&(self.same(previous.name,input.name,control)?||self.same(previous.manifest,input.manifest,control)?){return Err(ProviderRefusal::InvalidDeclaration);}}
   let mut admission=ProviderAdmission::default();
   let mut forward=ForwardControl{control,completed:&mut self.completed};
   let mut deliver=|target|{receive(input,target);self.admitted+=1;};
   admission.admit(input.declaration,&mut forward,&mut deliver)?;
  }
  self.current=inputs.len();self.step(control)?;Ok(())
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
