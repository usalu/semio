//! 🧳️ Original-funded borrowed validation owns one fixed continuation and one sealed source alias.
use super::graph::LayoutGraphRead;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use std::{future::Future,pin::Pin,task::{Context,Poll,Waker},mem::size_of_val};
#[path="📋️program.rs"]mod program;
#[path="📇️index.rs"]mod index;
type Validation=Result<(),&'static str>;
type Program=Pin<Box<dyn Future<Output=Validation>+Send>>;
/// 🧾️ Every cooperative turn reports its original-funded work and semantic outcome.
pub struct ValidationAdvance{pub progress:RetainedCloneProgress,pub done:bool,pub error:Option<&'static str>}
/// 🛡️ The source alias outlives the immutable program loan and is closed only after its continuation.
pub struct LayoutValidationCursor{binding:Option<RetainedCloneBinding>,program:Option<Program>,capacity:usize,index:index::Index,bound:bool,started:bool,done:bool,closing:bool,error:Option<&'static str>}
impl Default for LayoutValidationCursor{fn default()->Self{Self::new()}}
impl LayoutValidationCursor{
 const OVERHEAD:usize=256;
 fn nested(d:RetirementDemand)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:d.copy_bytes.checked_add(Self::OVERHEAD).ok_or_else(||Self::fault("Layout validation nested copy extent overflow"))?,depth:d.depth.checked_add(1).ok_or_else(||Self::fault("Layout validation nested depth overflow"))?,..d})}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(Self::OVERHEAD),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
 fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(Self::OVERHEAD);p}
 fn failure(error:ValueError)->ValueError{let p=Self::receipt(error.retained_progress());error.with_retained_progress(p)}
 pub fn new()->Self{Self{binding:None,program:None,capacity:0,index:Default::default(),bound:false,started:false,done:false,closing:false,error:None}}
 fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
 fn program_bytes<G:LayoutGraphRead+'static>(&self,source:RetainedCloneRef<'_,G>)->usize{size_of_val(&program::check_document(source.get(),self.index.slots()))}
 pub fn demand<G:LayoutGraphRead+'static>(&self,source:RetainedCloneRef<'_,G>,_body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(Self::fault("Layout validation is closing"))}let binding=source.binding_demand(&self.binding)?;if !self.bound{return Self::nested(binding.unwrap_or_default())}if self.done{return Ok(Default::default())}if !self.index.done(){return Self::nested(self.index.demand()?)}if !self.started{return Ok(RetirementDemand{copy_bytes:self.program_bytes(source),capacity_bytes:self.program_bytes(source),depth:1,..Default::default()})}Ok(RetirementDemand{copy_bytes:self.capacity.checked_mul(2).and_then(|n|n.checked_add(1024)).ok_or_else(||Self::fault("Layout validation work extent overflow"))?,depth:1,..Default::default()})
 }
 pub fn next_demand<G:LayoutGraphRead+'static>(&self,source:RetainedCloneRef<'_,G>,body:usize)->Result<RetirementDemand,ValueError>{self.demand(source,body)}
 /// 🪢️ The sealed binding and fixed index backing remain live until the funded continuation drop preceding every index slot, backing and alias retirement.
 pub fn advance<G:LayoutGraphRead+'static>(&mut self,source:RetainedCloneRef<'_,G>,body:usize,g:RetainedCloneGrant)->Result<ValidationAdvance,ValueError>{
  let mut result=ValidationAdvance{progress:Default::default(),done:self.done,error:self.error};let d=self.demand(source,body)?;if self.done||!retained_clone_grant_admits(g,d){return Ok(result)}
  if !self.bound{let progress=source.bind(&mut self.binding,Self::child(g)).map_err(Self::failure)?.unwrap_or_default();if self.binding.is_some(){self.bound=true;}result.progress=Self::receipt(progress);return Ok(result)}
  if !self.index.done(){result.progress=Self::receipt(self.index.advance(source.get(),Self::child(g)).map_err(Self::failure)?);if let Some(error)=self.index.error(){self.done=true;self.error=Some(error);result.done=true;result.error=Some(error);}return Ok(result)}
  result.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if !self.started{let address=source.get()as*const G;let loan: &'static G=unsafe{&*address};self.capacity=self.program_bytes(source);let index: index::Slots<'static>=unsafe{std::mem::transmute(self.index.slots())};self.program=Some(Box::pin(program::check_document(loan,index)));self.started=true;result.progress.retained_capacity_bytes=self.capacity;return Ok(result)}
  let mut context=Context::from_waker(Waker::noop());if let Poll::Ready(outcome)=self.program.as_mut().ok_or_else(||Self::fault("Layout validation continuation is absent"))?.as_mut().poll(&mut context){self.done=true;self.error=outcome.err();result.done=true;result.error=self.error;}Ok(result)
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.program.is_some(){Ok(RetirementDemand{copy_bytes:self.capacity.checked_mul(2).ok_or_else(||Self::fault("Layout validation close extent overflow"))?,release_bytes:self.capacity,depth:1,..Default::default()})}else if !self.index.terminal_is_empty(){Self::nested(self.index.close_demand())}else{Self::nested(RetainedCloneBinding::close_demand(&self.binding,body.saturating_sub(Self::OVERHEAD))?)}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(Self::fault("Layout validation close has not begun"))}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.program.is_some(){self.program=None;let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:self.capacity,..Default::default()};self.capacity=0;self.started=false;return Ok(RetainedCloneStep::Progress(p))}if !self.index.terminal_is_empty(){return self.index.close_step(Self::child(g)).map(|p|RetainedCloneStep::Progress(Self::receipt(p))).map_err(Self::failure)}let step=RetainedCloneBinding::close_one(&mut self.binding,Self::child(g)).map_err(Self::failure)?;if self.binding.is_none(){self.bound=false;self.done=false;self.error=None;}let p=Self::receipt(step.progress());Ok(if self.binding.is_none(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.program.is_none()&&self.capacity==0&&self.index.terminal_is_empty()}
}
impl Drop for LayoutValidationCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Layout validation requires funded continuation and original alias closure")}}
#[cfg(test)]#[path="🧪️tests.rs"]mod tests;

#[path="📦️owned/🦀️.rs"]pub mod owned;

#[cfg(test)]#[path="🪆️references/🧪️tests/🦀️.rs"]mod drawing_reference_tests;
