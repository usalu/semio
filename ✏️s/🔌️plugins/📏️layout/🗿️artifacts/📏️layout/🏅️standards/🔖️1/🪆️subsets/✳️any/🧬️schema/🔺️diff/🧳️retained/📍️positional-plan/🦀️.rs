//! 📍️ Original positional rows remain borrowed until typed ownership commit.
use super::keyed_plan::{Origin,PlanAdvance};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::mem::size_of;
/// 👁️ List length and row coordinates carry no copied domain payload.
pub trait PositionalSource{fn held_len(&self)->usize;fn target_len(&self)->Option<usize>;fn rows_len(&self)->usize;fn row_index(&self,index:usize)->usize;}
/// 🧳 One numeric buffer admits sparse replacement, extension and truncation.
pub struct PositionalPlan{phase:u8,index:usize,count:usize,slots:Vec<Option<Origin>>,closing:bool,error:Option<&'static str>}
impl Default for PositionalPlan{fn default()->Self{Self::new()}}
impl PositionalPlan{
 pub fn new()->Self{Self{phase:0,index:0,count:0,slots:Vec::new(),closing:false,error:None}}
 pub fn demand(&self,s:&impl PositionalSource)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:128,capacity_bytes:if self.phase==0{s.target_len().unwrap_or(s.held_len()).checked_mul(size_of::<Option<Origin>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"positional metadata extent overflow"))?}else{0},depth:1,..Default::default()})}
 pub fn advance(&mut self,s:&impl PositionalSource,g:RetainedCloneGrant)->Result<PlanAdvance,ValueError>{let mut result=PlanAdvance{progress:Default::default(),done:self.phase==4,error:self.error};if self.closing||self.phase==4||self.error.is_some(){return Ok(result)}let d=self.demand(s)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(result)}result.progress=RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()};match self.phase{
  0=>{self.count=s.target_len().unwrap_or(s.held_len());let reserved=self.slots.try_reserve_exact(self.count);result.progress.retained_capacity_bytes=self.slots.capacity()*size_of::<Option<Origin>>();if reserved.is_err()||result.progress.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"positional metadata reservation failed").with_retained_progress(result.progress))}self.phase=1},
  1=>if self.slots.len()<self.count{let at=self.slots.len();self.slots.push((at<s.held_len()).then_some(Origin{inserted:false,index:at}))}else{self.phase=2;self.index=0},
  2=>if self.index==s.rows_len(){self.phase=3;self.index=0}else{let at=s.row_index(self.index);if at>=self.count{self.error=Some("mutation.apply.invalid-order")}else if self.slots[at].is_some_and(|at|at.inserted){self.error=Some("mutation.apply.duplicate-target")}else{self.slots[at]=Some(Origin{inserted:true,index:self.index});self.index+=1}},
  3=>if self.index==self.count{self.phase=4;result.done=true}else if self.slots[self.index].is_none(){self.error=Some("mutation.apply.missing-target")}else{self.index+=1},_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"unknown positional preflight phase"))
 }result.error=self.error;Ok(result)}
 pub fn origins(&self)->Option<&[Option<Origin>]>{(self.phase==4).then_some(self.slots.as_slice())}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:128,release_bytes:if self.slots.is_empty(){self.slots.capacity()*size_of::<Option<Origin>>()}else{0},depth:1,..Default::default()}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"positional close not started"))}let d=self.close_demand();if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(Default::default())}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()};if self.slots.pop().is_some(){return Ok(p)}p.released_bytes=self.slots.capacity()*size_of::<Option<Origin>>();self.slots=Vec::new();self.phase=5;Ok(p)}
 pub fn terminal_is_empty(&self)->bool{self.slots.is_empty()&&self.slots.capacity()==0}
}
impl Drop for PositionalPlan{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"positional metadata needs funded close")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
