//! 🪢 Repeated positional patches resolve against every previous admitted numeric prefix.
use super::{positional_plan::{PositionalPlan,PositionalSource},chained_plan::Address,keyed_plan::PlanAdvance};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::mem::size_of;
/// 👁 Original guide or style-run sources lend only extents and row indices.
pub trait PositionalChainSource{fn held_len(&self)->usize;fn steps_len(&self)->usize;fn target_len(&self,step:usize)->Option<usize>;fn rows_len(&self,step:usize)->usize;fn row_index(&self,step:usize,index:usize)->usize;}
struct View<'a,S>{held:usize,source:&'a S,step:usize}
impl<S:PositionalChainSource> PositionalSource for View<'_,S>{fn held_len(&self)->usize{self.held}fn target_len(&self)->Option<usize>{self.source.target_len(self.step)}fn rows_len(&self)->usize{self.source.rows_len(self.step)}fn row_index(&self,i:usize)->usize{self.source.row_index(self.step,i)}}
/// 🧳 Prefix replacement and truncation retain numeric provenance without candidate payloads.
pub struct PositionalChain{phase:u8,step:usize,index:usize,count:usize,maximum:usize,rows:Vec<Address>,next:Vec<Address>,current:PositionalPlan,closing:bool,error:Option<&'static str>}
impl Default for PositionalChain{fn default()->Self{Self::new()}}
impl PositionalChain{
 pub fn new()->Self{Self{phase:0,step:0,index:0,count:0,maximum:0,rows:Vec::new(),next:Vec::new(),current:PositionalPlan::new(),closing:false,error:None}}
 fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,message)}
 fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:128,depth:1,..Default::default()}}
 fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=128;d.depth+=1;d}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-128,maximum_depth:g.maximum_depth-1,..g}}
 fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=128;p.copied_items=p.copied_items.max(1);p}
 fn refused(error:ValueError)->ValueError{let p=Self::receipt(error.retained_progress());error.with_retained_progress(p)}
 fn capacity(&self)->usize{(self.rows.capacity()+self.next.capacity())*size_of::<Address>()}
 pub fn demand(&self,s:&impl PositionalChainSource)->Result<RetirementDemand,ValueError>{if self.phase==2{return Ok(RetirementDemand{capacity_bytes:self.maximum.checked_mul(size_of::<Address>()).and_then(|n|n.checked_mul(2)).ok_or_else(||Self::fault("positional prefix metadata extent overflow"))?,..Self::fixed()})}if self.phase==4&&self.step<s.steps_len(){return self.current.demand(&View{held:self.rows.len(),source:s,step:self.step}).map(Self::nested)}if self.phase==7{return Ok(Self::nested(self.current.close_demand()))}Ok(Self::fixed())}
 pub fn advance(&mut self,s:&impl PositionalChainSource,g:RetainedCloneGrant)->Result<PlanAdvance,ValueError>{let mut result=PlanAdvance{progress:Default::default(),done:self.phase==9,error:self.error};if self.closing||self.phase==9||self.error.is_some(){return Ok(result)}let d=self.demand(s)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(result)}result.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{
  0=>{self.count=s.held_len();self.maximum=self.count;self.phase=1},
  1=>if self.step==s.steps_len(){self.phase=2;self.step=0}else{self.count=s.target_len(self.step).unwrap_or(self.count);self.maximum=self.maximum.max(self.count);self.step+=1},
  2=>{let reserved=self.rows.try_reserve_exact(self.maximum).and_then(|_|self.next.try_reserve_exact(self.maximum));result.progress.retained_capacity_bytes=self.capacity();if reserved.is_err()||self.capacity()>g.maximum_capacity_bytes{return Err(Self::fault("positional prefix metadata reservation failed").with_retained_progress(result.progress))}self.phase=3},
  3=>if self.index<s.held_len(){self.rows.push(Address{step:None,row:self.index});self.index+=1}else{self.phase=4;self.index=0},
  4=>if self.step==s.steps_len(){self.phase=9;result.done=true}else{let step=self.current.advance(&View{held:self.rows.len(),source:s,step:self.step},Self::child(g)).map_err(Self::refused)?;result.progress=Self::receipt(step.progress);if let Some(error)=step.error{self.error=Some(error)}else if step.done{self.phase=5;self.index=0}},
  5=>{let origins=self.current.origins().expect("admitted positional prefix");if self.index==origins.len(){self.phase=6;self.index=0}else{let at=origins[self.index].expect("occupied positional source");self.next.push(if at.inserted{Address{step:Some(self.step),row:at.index}}else{self.rows[at.index]});self.index+=1}},
  6=>{self.current.begin_close();self.phase=7},7=>{result.progress=Self::receipt(self.current.close_step(Self::child(g)).map_err(Self::refused)?);if self.current.terminal_is_empty(){self.phase=8}},
  8=>if self.rows.pop().is_none(){std::mem::swap(&mut self.rows,&mut self.next);self.current=PositionalPlan::new();self.step+=1;self.phase=4},_=>return Err(Self::fault("positional prefix phase is invalid"))
 }result.error=self.error;Ok(result)}
 pub fn origins(&self)->Option<&[Address]>{(self.phase==9).then_some(self.rows.as_slice())}
 pub fn begin_close(&mut self){self.closing=true;self.current.begin_close()}
 pub fn close_demand(&self)->RetirementDemand{if !self.current.terminal_is_empty(){return Self::nested(self.current.close_demand())}RetirementDemand{release_bytes:if self.rows.is_empty()&&self.next.is_empty(){self.capacity()}else{0},..Self::fixed()}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if !self.closing{return Err(Self::fault("positional prefix original close not begun"))}let d=self.close_demand();if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(Default::default())}if !self.current.terminal_is_empty(){return self.current.close_step(Self::child(g)).map(Self::receipt).map_err(Self::refused)}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()};if self.rows.pop().is_some(){return Ok(p)}if self.next.pop().is_some(){return Ok(p)}p.released_bytes=self.capacity();self.rows=Vec::new();self.next=Vec::new();self.phase=10;Ok(p)}
 pub fn terminal_is_empty(&self)->bool{self.rows.is_empty()&&self.next.is_empty()&&self.capacity()==0&&self.current.terminal_is_empty()}
}
impl Drop for PositionalChain{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"positional prefix original ownership needs funded close")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
