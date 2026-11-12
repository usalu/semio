//! 📥️ Original live gestures retain immutable canonical roots and actual ranked entry owners.
use super::{GumballGestures,GumballGesture,GumballMotion,GENERATION3D_GUMBALL_LEAF_KEY};
use semio_framework_artifact_flow_flow::OrderedMap;
use semio_os_flow::SharedOwner;
use crate::source_birth::{Birth,begun,closed,construction,effected,fixed,local_progress,replied,resumed};
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedAlias,RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
struct Fields{map:Option<OrderedMap<GumballGesture>>,key:Option<SharedOwner<String>>,gesture:Option<SharedOwner<GumballGesture>>}
/// 👁️ Every field borrows the actual immutable gesture entry retained by its original read.
pub struct GumballPreviewRecord<'a>{pub window:&'a str,pub ids:&'a [String],pub motion:GumballMotion}
pub struct OriginalGumballRead{fields:ManuallyDrop<Fields>,original:OriginalSourceBinding,source:Option<(usize,Option<usize>,u64)>,retirement:RetainedCloneClose,birth:Birth,ordinal:usize,entry:usize,phase:u8,motion:Option<GumballMotion>,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original live gesture source or caller changed")}
impl OriginalGumballRead{
 pub fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub fn new_original(original:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{map:None,key:None,gesture:None}),original,source:None,retirement:Default::default(),birth:Default::default(),ordinal:0,entry:0,phase:0,motion:None,closing:false}}
 fn identity(source:&GumballGestures)->(usize,Option<usize>,u64){(source as*const _ as usize,source.open.as_ref().and_then(OrderedMap::original_root_identity),source.revision)}
 pub fn matches(&self,source:&GumballGestures)->bool{!source.closing&&self.source.is_none_or(|bound|bound==Self::identity(source))}
 fn validate(&self,source:&GumballGestures)->Result<(),ValueError>{if self.closing{return Err(invalid())}if !self.matches(source){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original live gesture authority was invalidated"))}Ok(())}
 fn authenticate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;Ok(())}
 pub fn ready(&self)->bool{!self.closing&&matches!(self.phase,3|5)}
 pub fn finished(&self)->bool{!self.closing&&self.phase==5}
 pub fn revision(&self)->Option<u64>{self.source.map(|(_,_,revision)|revision)}
 pub fn record<'a>(&'a self,source:&GumballGestures,cx:&StepContext<'_>)->Result<Option<GumballPreviewRecord<'a>>,OriginalTurnFailure>{
  self.authenticate(cx)?;self.validate(source)?;
  if !self.ready(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original gesture entry is still pending").into())}if self.finished(){return Ok(None)}
  let fields=&*self.fields;let gesture=fields.gesture.as_ref().ok_or_else(invalid)?;
  Ok(Some(GumballPreviewRecord{window:fields.key.as_ref().ok_or_else(invalid)?.as_str(),ids:&gesture.ids,motion:self.motion.ok_or_else(invalid)?}))
 }
 pub fn next_demands(&self,source:&GumballGestures,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;
  match self.phase{
   0=>{let mut d=source.live().alias_demand();d.copy_bytes=d.copy_bytes.checked_add(size_of::<(usize,Option<usize>,u64)>()).ok_or_else(invalid)?;Ok(d)},
   1=>Ok(fixed(if self.ordinal<self.fields.map.as_ref().ok_or_else(invalid)?.len(){2*(size_of::<SharedOwner<String>>()+size_of::<SharedOwner<GumballGesture>>())}else{0})),
   2=>{let fields=&*self.fields;let key=fields.gesture.as_ref().ok_or_else(invalid)?.entries.get(self.entry).map(|(key,_)|key);Ok(fixed(key.map_or(0,|key|if key.len()==GENERATION3D_GUMBALL_LEAF_KEY.len(){2*key.len()}else{0})))},
   3|5=>Ok(Default::default()),
   4=>self.record_close_demands(copy),
   6=>Ok(fixed(size_of::<GumballMotion>())),
   _=>Err(invalid()),
  }
 }
 fn alias(&mut self,source:&GumballGestures,d:RetirementDemand,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let mut alias=None;
  let funded=effected(cx,d,|effect|{
   let step=replied(effect,|effect,reply|source.live().admit_alias_into(&mut alias,effect,reply))?.map_or(Ok(RetainedCloneStep::Progress(Default::default())),|settled|settled.map_err(|failure|failure.into_value_error()))?;
   let mut progress=step.progress();if alias.is_some(){progress.copied_bytes+=size_of::<(usize,Option<usize>,u64)>()}Ok(if alias.is_some(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
  })?;
  if !funded{return Ok(false)}let Some(map)=alias else{return Ok(false)};
  self.fields.map=Some(map);self.source=Some(Self::identity(source));self.phase=1;Ok(true)
 }
 fn retire_record(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let body=cx.retained_grant().maximum_copy_bytes;let d=self.record_close_demands(body)?;
  let step=cx.with_original_retirement(|cx|closed(cx,d,|effect|self.retire_original(body,d,effect)))?;Ok(step.progress()!=Default::default())
 }
 fn retire_original(&mut self,body:usize,d:RetirementDemand,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.retirement.is_empty(){return resumed(&mut self.retirement,body,effect)}
  if self.fields.key.is_some(){return begun(&mut self.retirement,&mut self.fields.key,effect)}
  if self.fields.gesture.is_some(){return begun(&mut self.retirement,&mut self.fields.gesture,effect)}
  if !retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.ordinal+=1;self.motion=None;self.phase=1;Ok(RetainedCloneStep::Progress(local_progress(d)))
 }
 fn local(&mut self,source:&GumballGestures,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;let grant=cx.retained_grant();
  let mut birth=std::mem::take(&mut self.birth);let settled=birth.run(cx,construction(d),||{if !retained_clone_grant_admits(grant,d){return Err(invalid())}self.advance_local(d)});self.birth=birth;settled
 }
 fn advance_local(&mut self,d:RetirementDemand)->Result<RetainedCloneProgress,ValueError>{
  match self.phase{
   1=>{
    if let Some((key,gesture))=self.fields.map.as_ref().ok_or_else(invalid)?.entry_shared_at_rank(self.ordinal){self.fields.key=Some(key);self.fields.gesture=Some(gesture);self.entry=0;self.phase=2}else{self.phase=5}
   },
   2=>{
    if let Some((key,_))=self.fields.gesture.as_ref().ok_or_else(invalid)?.entries.get(self.entry){
     let equal=key.len()==GENERATION3D_GUMBALL_LEAF_KEY.len()&&key.as_bytes().iter().zip(GENERATION3D_GUMBALL_LEAF_KEY.as_bytes()).fold(true,|equal,(a,b)|equal&(*a==*b));
     if equal{self.phase=6}else{self.entry+=1}
    }else{self.phase=4}
   },
   6=>{self.motion=GumballMotion::of_leaf(&self.fields.gesture.as_ref().ok_or_else(invalid)?.entries.get(self.entry).ok_or_else(invalid)?.1);self.phase=if self.motion.is_some(){3}else{4}},
   _=>return Err(invalid()),
  }Ok(local_progress(d))
 }
 pub fn step(&mut self,source:&GumballGestures,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;self.validate(source)?;
  for _ in 0..512{
   if self.ready()&&!self.birth.settling(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
   let fuel=cx.fuel_remaining();
   let advanced=if self.birth.settling(){self.local(source,cx)?}else{match self.phase{
    0=>{let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;self.alias(source,d,cx)?},
    4=>self.retire_record(cx)?,
    _=>self.local(source,cx)?,
   }};
   if !advanced&&cx.fuel_remaining()==fuel{return Ok(self.ready()&&!self.birth.settling())}
  }Ok(self.ready()&&!self.birth.settling())
 }
 pub fn next_record(&mut self,source:&GumballGestures,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;self.validate(source)?;if self.phase!=3&&!self.birth.settling(){return Err(invalid().into())}
  let d=fixed(size_of::<u8>());let grant=cx.retained_grant();
  let mut birth=std::mem::take(&mut self.birth);let settled=birth.run(cx,construction(d),||{if !retained_clone_grant_admits(grant,d){return Err(invalid())}self.phase=4;Ok(local_progress(d))});self.birth=birth;settled
 }
 fn record_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.key.is_some(){return self.retirement.next_owner_demands::<SharedOwner<String>>(true,&None,copy)}
  if self.fields.gesture.is_some(){return self.retirement.next_owner_demands::<SharedOwner<GumballGesture>>(true,&None,copy)}
  Ok(fixed(0))
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.birth.terminal_is_empty(){return self.birth.close_demand(copy)}
  if !self.retirement.is_empty()||self.fields.key.is_some()||self.fields.gesture.is_some(){return self.record_close_demands(copy)}
  if self.fields.map.is_some(){return self.retirement.next_owner_demands::<OrderedMap<GumballGesture>>(true,&None,copy)}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}if !self.closing{return Err(invalid().into())}if !self.birth.close_step(cx)?{return Ok(false)}
  let body=cx.retained_grant().maximum_copy_bytes;let d=self.next_close_demands(body)?;closed(cx,d,|effect|self.close_original(body,effect))?;Ok(self.terminal_is_empty())
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.retirement.is_empty(){return resumed(&mut self.retirement,body,effect)}
  if self.fields.key.is_some(){return begun(&mut self.retirement,&mut self.fields.key,effect)}
  if self.fields.gesture.is_some(){return begun(&mut self.retirement,&mut self.fields.gesture,effect)}
  if self.fields.map.is_some(){return begun(&mut self.retirement,&mut self.fields.map,effect)}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.map.is_none()&&self.fields.key.is_none()&&self.fields.gesture.is_none()&&self.retirement.is_empty()&&self.birth.terminal_is_empty()}
}
impl Drop for OriginalGumballRead{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original gesture read retains its actual root, entry or source retirement");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}

#[cfg(test)]
#[path="../../../../../../../../../../🫀️foundation/🌊️flow-source/🧪️caller/🦀️.rs"]
mod scratch_testing;
#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod tests;
