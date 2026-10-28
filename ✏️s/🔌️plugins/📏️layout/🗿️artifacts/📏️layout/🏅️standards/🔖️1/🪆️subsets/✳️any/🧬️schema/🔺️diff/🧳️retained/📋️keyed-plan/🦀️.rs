//! 📋️ Borrowed keyed-list admission retains numeric addresses, never row payloads.
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::mem::size_of;
/// 👁️ Keys and coordinates borrow their original live-post and diff owners.
pub trait KeyedSource {
 fn held_len(&self)->usize;fn held_key(&self,index:usize)->&str;
 fn removed_len(&self)->usize;fn removed(&self,index:usize)->(&str,usize);
 fn inserted_len(&self)->usize;fn inserted(&self,index:usize)->(&str,usize);
 fn moved_len(&self)->usize;fn moved(&self,index:usize)->(&str,usize,usize);
 fn modified_len(&self)->usize;fn modified_key(&self,index:usize)->&str;
}
/// 🧭 One final slot names an original live row or an authored insertion.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Origin{pub inserted:bool,pub index:usize}
/// 🧾 Each numeric preflight turn reports its performed ownership work.
pub struct PlanAdvance{pub progress:RetainedCloneProgress,pub done:bool,pub error:Option<&'static str>}
/// 🧳 Numeric preflight preserves every original source allocation for the typed commit.
pub struct KeyedPlan{phase:u8,index:usize,other:usize,offset:usize,count:usize,flags:Vec<u8>,slots:Vec<Option<Origin>>,targets:Vec<usize>,closing:bool,error:Option<&'static str>}
impl Default for KeyedPlan{fn default()->Self{Self::new()}}
impl KeyedPlan{
 pub fn new()->Self{Self{phase:0,index:0,other:0,offset:0,count:0,flags:Vec::new(),slots:Vec::new(),targets:Vec::new(),closing:false,error:None}}
 fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,message)}
 fn target_len(s:&impl KeyedSource)->Result<usize,ValueError>{s.held_len().checked_sub(s.removed_len()).and_then(|n|n.checked_add(s.inserted_len())).ok_or_else(||Self::fault("keyed list final extent is invalid"))}
 fn capacity_for(s:&impl KeyedSource)->Result<usize,ValueError>{let count=Self::target_len(s)?;count.checked_mul(size_of::<Option<Origin>>()).and_then(|n|n.checked_add(s.held_len())).and_then(|n|s.modified_len().checked_mul(size_of::<usize>()).and_then(|m|n.checked_add(m))).ok_or_else(||Self::fault("keyed list metadata extent overflow"))}
 fn capacity(&self)->usize{self.flags.capacity()+self.slots.capacity()*size_of::<Option<Origin>>()+self.targets.capacity()*size_of::<usize>()}
 pub fn demand(&self,s:&impl KeyedSource)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:640,capacity_bytes:if self.phase==0{Self::capacity_for(s)?}else{0},depth:1,..Default::default()})}
 fn compare(&mut self,a:&str,b:&str)->Option<bool>{if a.len()!=b.len(){self.offset=0;return Some(false)}let end=a.len().min(self.offset.saturating_add(256));if a.as_bytes()[self.offset..end]!=b.as_bytes()[self.offset..end]{self.offset=0;return Some(false)}self.offset=end;if end==a.len(){self.offset=0;Some(true)}else{None}}
 pub fn advance(&mut self,s:&impl KeyedSource,g:RetainedCloneGrant)->Result<PlanAdvance,ValueError>{
  let mut result=PlanAdvance{progress:Default::default(),done:self.phase==9,error:self.error};if self.closing||self.phase==9||self.error.is_some(){return Ok(result)}let d=self.demand(s)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(result)}result.progress=RetainedCloneProgress{copied_items:1,copied_bytes:640,..Default::default()};
  match self.phase{
   0=>{self.count=Self::target_len(s)?;let reserved=self.flags.try_reserve_exact(s.held_len()).and_then(|_|self.slots.try_reserve_exact(self.count)).and_then(|_|self.targets.try_reserve_exact(s.modified_len()));result.progress.retained_capacity_bytes=self.capacity();if reserved.is_err()||self.capacity()>g.maximum_capacity_bytes{return Err(Self::fault("keyed list metadata reservation failed").with_retained_progress(result.progress))}self.phase=1},
   1=>if self.flags.len()<s.held_len(){self.flags.push(0)}else{self.phase=2},
   2=>if self.slots.len()<self.count{self.slots.push(None)}else{self.phase=3;self.index=0},
   3=>if self.index==s.removed_len(){self.phase=4;self.index=0}else{let(id,at)=s.removed(self.index);if at>=s.held_len(){self.error=Some("mutation.apply.missing-target")}else{match self.compare(id,s.held_key(at)){None=>{},Some(false)=>self.error=Some("mutation.apply.missing-target"),Some(true)=>if self.flags[at]!=0{self.error=Some("mutation.apply.duplicate-target")}else{self.flags[at]=1;self.index+=1}}}},
   4=>if self.index==s.inserted_len(){self.phase=5;self.index=0}else{let(_,at)=s.inserted(self.index);if at>=self.count{self.error=Some("mutation.apply.invalid-add-index")}else if self.slots[at].is_some(){self.error=Some("mutation.apply.duplicate-target")}else{self.slots[at]=Some(Origin{inserted:true,index:self.index});self.index+=1}},
   5=>if self.index==s.moved_len(){self.phase=6;self.index=0;self.other=0}else{let(id,from,to)=s.moved(self.index);if from>=s.held_len(){self.error=Some("mutation.apply.missing-target")}else{match self.compare(id,s.held_key(from)){None=>{},Some(false)=>self.error=Some("mutation.apply.missing-target"),Some(true)=>if self.flags[from]!=0{self.error=Some("mutation.apply.duplicate-target")}else if to>=self.count{self.error=Some("mutation.apply.invalid-move-index")}else if self.slots[to].is_some(){self.error=Some("mutation.apply.duplicate-target")}else{self.flags[from]=2;self.slots[to]=Some(Origin{inserted:false,index:from});self.index+=1}}}},
   6=>if self.other==self.count{self.phase=7;self.index=0;self.other=1}else if self.slots[self.other].is_some(){self.other+=1}else if self.index>=s.held_len(){self.error=Some("mutation.apply.missing-target")}else if self.flags[self.index]!=0{self.index+=1}else{self.slots[self.other]=Some(Origin{inserted:false,index:self.index});self.index+=1;self.other+=1},
   7=>if self.index>=self.count{self.phase=8;self.index=0;self.other=0}else if self.other>=self.count{self.index+=1;self.other=self.index+1}else{let(a,b)=(self.slots[self.index].expect("first admitted key"),self.slots[self.other].expect("second admitted key"));let key=|at:Origin|if at.inserted{s.inserted(at.index).0}else{s.held_key(at.index)};match self.compare(key(a),key(b)){None=>{},Some(true)=>self.error=Some("mutation.apply.duplicate-target"),Some(false)=>self.other+=1}},
   8=>if self.index==s.modified_len(){self.phase=9;result.done=true}else if self.other>=self.count{self.error=Some("mutation.apply.missing-target")}else{let at=self.slots[self.other].expect("modified target key");let key=if at.inserted{s.inserted(at.index).0}else{s.held_key(at.index)};match self.compare(s.modified_key(self.index),key){None=>{},Some(true)=>{self.targets.push(self.other);self.index+=1;self.other=0},Some(false)=>self.other+=1}},
   _=>return Err(Self::fault("unknown keyed preflight phase"))
  }result.error=self.error;Ok(result)
 }
 pub fn origins(&self)->Option<&[Option<Origin>]>{(self.phase==9).then_some(self.slots.as_slice())}
 pub fn modification_targets(&self)->Option<&[usize]>{(self.phase==9).then_some(self.targets.as_slice())}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:128,release_bytes:if self.flags.is_empty()&&self.slots.is_empty()&&self.targets.is_empty(){self.capacity()}else{0},depth:1,..Default::default()}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if !self.closing{return Err(Self::fault("keyed plan close not started"))}let d=self.close_demand();if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(Default::default())}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()};if self.flags.pop().is_some(){return Ok(p)}if self.slots.pop().is_some(){return Ok(p)}if self.targets.pop().is_some(){return Ok(p)}p.released_bytes=self.capacity();self.flags=Vec::new();self.slots=Vec::new();self.targets=Vec::new();self.phase=10;Ok(p)}
 pub fn terminal_is_empty(&self)->bool{self.capacity()==0&&self.flags.is_empty()&&self.slots.is_empty()&&self.targets.is_empty()}
}
impl Drop for KeyedPlan{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"keyed preflight metadata needs funded close")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
