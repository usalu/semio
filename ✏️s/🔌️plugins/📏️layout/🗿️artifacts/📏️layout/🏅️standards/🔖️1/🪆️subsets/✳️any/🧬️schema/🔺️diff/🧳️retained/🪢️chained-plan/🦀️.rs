//! 🪢 Repeated nested-list patches retain original row identities across every admitted prefix.
use super::keyed_plan::{KeyedPlan,KeyedSource,PlanAdvance};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::mem::size_of;
/// 🧭 A stable row identity names live post or one original authored insertion.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Address{pub step:Option<usize>,pub row:usize}
/// 🩹 Each patch retains the identity version it actually addressed before later replacement.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Modification{pub target:Address,pub step:usize,pub index:usize}
/// 👁 All payload and keys remain borrowed from original immutable owners.
pub trait ChainedSource{
 fn held_len(&self)->usize;fn held_key(&self,index:usize)->&str;fn steps_len(&self)->usize;
 fn removed_len(&self,step:usize)->usize;fn removed(&self,step:usize,index:usize)->(&str,usize);
 fn inserted_len(&self,step:usize)->usize;fn inserted(&self,step:usize,index:usize)->(&str,usize);
 fn moved_len(&self,step:usize)->usize;fn moved(&self,step:usize,index:usize)->(&str,usize,usize);
 fn modified_len(&self,step:usize)->usize;fn modified_key(&self,step:usize,index:usize)->&str;
}
struct View<'a,S>{rows:&'a [Address],source:&'a S,step:usize}
impl<S:ChainedSource> KeyedSource for View<'_,S>{
 fn held_len(&self)->usize{self.rows.len()}fn held_key(&self,i:usize)->&str{let at=self.rows[i];match at.step{None=>self.source.held_key(at.row),Some(step)=>self.source.inserted(step,at.row).0}}
 fn removed_len(&self)->usize{self.source.removed_len(self.step)}fn removed(&self,i:usize)->(&str,usize){self.source.removed(self.step,i)}
 fn inserted_len(&self)->usize{self.source.inserted_len(self.step)}fn inserted(&self,i:usize)->(&str,usize){self.source.inserted(self.step,i)}
 fn moved_len(&self)->usize{self.source.moved_len(self.step)}fn moved(&self,i:usize)->(&str,usize,usize){self.source.moved(self.step,i)}
 fn modified_len(&self)->usize{self.source.modified_len(self.step)}fn modified_key(&self,i:usize)->&str{self.source.modified_key(self.step,i)}
}
/// 🧳 Two admitted numeric buffers flatten each prefix without retaining copied row payloads.
pub struct ChainedPlan{phase:u8,step:usize,index:usize,count:usize,maximum:usize,changes:usize,rows:Vec<Address>,next:Vec<Address>,edits:Vec<Modification>,current:KeyedPlan,closing:bool,error:Option<&'static str>}
impl Default for ChainedPlan{fn default()->Self{Self::new()}}
impl ChainedPlan{
 pub fn new()->Self{Self{phase:0,step:0,index:0,count:0,maximum:0,changes:0,rows:Vec::new(),next:Vec::new(),edits:Vec::new(),current:KeyedPlan::new(),closing:false,error:None}}
 fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,message)}
 fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:128,depth:1,..Default::default()}}
 fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=128;d.depth+=1;d}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-128,maximum_depth:g.maximum_depth-1,..g}}
 fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=128;p.copied_items=p.copied_items.max(1);p}
 fn refused(error:ValueError)->ValueError{let p=Self::receipt(error.retained_progress());error.with_retained_progress(p)}
 fn capacity(&self)->usize{(self.rows.capacity()+self.next.capacity())*size_of::<Address>()+self.edits.capacity()*size_of::<Modification>()}
 pub fn demand(&self,s:&impl ChainedSource)->Result<RetirementDemand,ValueError>{if self.phase==2{let capacity=self.maximum.checked_mul(size_of::<Address>()).and_then(|n|n.checked_mul(2)).and_then(|n|self.changes.checked_mul(size_of::<Modification>()).and_then(|m|n.checked_add(m))).ok_or_else(||Self::fault("chained numeric metadata extent overflow"))?;return Ok(RetirementDemand{capacity_bytes:capacity,..Self::fixed()})}if self.phase==4&&self.step<s.steps_len(){return self.current.demand(&View{rows:&self.rows,source:s,step:self.step}).map(Self::nested)}if self.phase==8{return Ok(Self::nested(self.current.close_demand()))}Ok(Self::fixed())}
 pub fn advance(&mut self,s:&impl ChainedSource,g:RetainedCloneGrant)->Result<PlanAdvance,ValueError>{let mut result=PlanAdvance{progress:Default::default(),done:self.phase==11,error:self.error};if self.closing||self.phase==11||self.error.is_some(){return Ok(result)}let d=self.demand(s)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(result)}result.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{
  0=>{self.count=s.held_len();self.maximum=self.count;self.phase=1},
  1=>if self.step==s.steps_len(){self.phase=2;self.step=0}else{match self.count.checked_sub(s.removed_len(self.step)).and_then(|n|n.checked_add(s.inserted_len(self.step))){None=>self.error=Some("mutation.apply.missing-target"),Some(count)=>{self.count=count;self.maximum=self.maximum.max(count);self.changes=self.changes.checked_add(s.modified_len(self.step)).ok_or_else(||Self::fault("chained patch metadata count overflow").with_retained_progress(result.progress))?;self.step+=1}}},
  2=>{let reserved=self.rows.try_reserve_exact(self.maximum).and_then(|_|self.next.try_reserve_exact(self.maximum)).and_then(|_|self.edits.try_reserve_exact(self.changes));result.progress.retained_capacity_bytes=self.capacity();if reserved.is_err()||self.capacity()>g.maximum_capacity_bytes{return Err(Self::fault("chained numeric metadata reservation failed").with_retained_progress(result.progress))}self.phase=3},
  3=>if self.index<s.held_len(){self.rows.push(Address{step:None,row:self.index});self.index+=1}else{self.phase=4;self.index=0},
  4=>if self.step==s.steps_len(){self.phase=11;result.done=true}else{let step=self.current.advance(&View{rows:&self.rows,source:s,step:self.step},Self::child(g)).map_err(Self::refused)?;result.progress=Self::receipt(step.progress);if let Some(error)=step.error{self.error=Some(error)}else if step.done{self.phase=5;self.index=0}},
  5=>{let origins=self.current.origins().expect("admitted chained prefix");if self.index==origins.len(){self.phase=6;self.index=0}else{let at=origins[self.index].expect("occupied admitted prefix");self.next.push(if at.inserted{Address{step:Some(self.step),row:at.index}}else{self.rows[at.index]});self.index+=1}},
  6=>{let targets=self.current.modification_targets().expect("admitted chained patches");if self.index==targets.len(){self.phase=7;self.index=0}else{self.edits.push(Modification{target:self.next[targets[self.index]],step:self.step,index:self.index});self.index+=1}},
  7=>{self.current.begin_close();self.phase=8},
  8=>{result.progress=Self::receipt(self.current.close_step(Self::child(g)).map_err(Self::refused)?);if self.current.terminal_is_empty(){self.phase=9}},
  9=>if self.rows.pop().is_none(){std::mem::swap(&mut self.rows,&mut self.next);self.current=KeyedPlan::new();self.step+=1;self.phase=4},_=>return Err(Self::fault("chained numeric phase is invalid"))
 }result.error=self.error;Ok(result)}
 pub fn origins(&self)->Option<&[Address]>{(self.phase==11).then_some(self.rows.as_slice())}
 pub fn modifications(&self)->Option<&[Modification]>{(self.phase==11).then_some(self.edits.as_slice())}
 pub fn begin_close(&mut self){self.closing=true;self.current.begin_close()}
 pub fn close_demand(&self)->RetirementDemand{if !self.current.terminal_is_empty(){return Self::nested(self.current.close_demand())}RetirementDemand{release_bytes:if self.rows.is_empty()&&self.next.is_empty()&&self.edits.is_empty(){self.capacity()}else{0},..Self::fixed()}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if !self.closing{return Err(Self::fault("chained original close not begun"))}let d=self.close_demand();if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(Default::default())}if !self.current.terminal_is_empty(){return self.current.close_step(Self::child(g)).map(Self::receipt).map_err(Self::refused)}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()};if self.rows.pop().is_some(){return Ok(p)}if self.next.pop().is_some(){return Ok(p)}if self.edits.pop().is_some(){return Ok(p)}p.released_bytes=self.capacity();self.rows=Vec::new();self.next=Vec::new();self.edits=Vec::new();self.phase=12;Ok(p)}
 pub fn terminal_is_empty(&self)->bool{self.rows.is_empty()&&self.next.is_empty()&&self.edits.is_empty()&&self.capacity()==0&&self.current.terminal_is_empty()}
}
impl Drop for ChainedPlan{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"chained numeric original ownership needs funded close")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
