//! 📥️ Original live gestures retain immutable canonical roots and actual ranked entry owners.
use super::{GumballGestures,GumballGesture,GumballMotion,GENERATION3D_GUMBALL_LEAF_KEY};
use semio_framework_artifact_flow_flow::OrderedMap;
use semio_os_flow::SharedOwner;
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step,receive_original_retained_work};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedAlias,RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
struct Fields{map:Option<OrderedMap<GumballGesture>>,key:Option<SharedOwner<String>>,gesture:Option<SharedOwner<GumballGesture>>}
/// 👁️ Every field borrows the actual immutable gesture entry retained by its original read.
pub struct GumballPreviewRecord<'a>{pub window:&'a str,pub ids:&'a [String],pub motion:GumballMotion}
pub struct OriginalGumballRead{fields:ManuallyDrop<Fields>,original:OriginalSourceBinding,source:Option<(usize,Option<usize>,u64)>,retirement:RetainedCloneClose,ordinal:usize,entry:usize,phase:u8,motion:Option<GumballMotion>,closing:bool,progress:RetainedCloneProgress,work:u64}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original live gesture source or caller changed")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
impl OriginalGumballRead{
 pub fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub fn new_original(original:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{map:None,key:None,gesture:None}),original,source:None,retirement:Default::default(),ordinal:0,entry:0,phase:0,motion:None,closing:false,progress:Default::default(),work:0}}
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
 pub fn advance(&mut self,source:&GumballGestures,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;let result=self.advance_original(source,grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn advance_original(&mut self,source:&GumballGestures,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  let d=self.next_demands(source,grant.maximum_copy_bytes)?;if self.ready(){return Ok(true)}if !retained_clone_grant_admits(grant,d){return Ok(false)}
  match self.phase{
   0=>{
    let turn=source.live().admit_alias(grant);self.work=turn.work;let(map,mut p)=turn.result?;p.copied_bytes+=size_of::<(usize,Option<usize>,u64)>();self.fields.map=Some(map);self.progress=p;self.source=Some(Self::identity(source));self.phase=1
   },
   1=>{
    self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
    if let Some((key,gesture))=self.fields.map.as_ref().ok_or_else(invalid)?.entry_shared_at_rank(self.ordinal){self.fields.key=Some(key);self.fields.gesture=Some(gesture);self.entry=0;self.phase=2}else{self.phase=5}
   },
   2=>{
    self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
    if let Some((key,_))=self.fields.gesture.as_ref().ok_or_else(invalid)?.entries.get(self.entry){
     let equal=key.len()==GENERATION3D_GUMBALL_LEAF_KEY.len()&&key.as_bytes().iter().zip(GENERATION3D_GUMBALL_LEAF_KEY.as_bytes()).fold(true,|equal,(a,b)|equal&(*a==*b));
     if equal{self.phase=6}else{self.entry+=1}
    }else{self.phase=4}
   },
   4=>{
    if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);self.work=turn.work;self.progress=turn.result?.progress()}
    else if self.fields.key.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.key,grant);self.work=turn.work;self.progress=turn.result?.map_or(Default::default(),|step|step.progress())}
    else if self.fields.gesture.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.gesture,grant);self.work=turn.work;self.progress=turn.result?.map_or(Default::default(),|step|step.progress())}
    else{self.work=1;self.progress.copied_items=1;self.ordinal+=1;self.motion=None;self.phase=1}
   },
   6=>{self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};self.motion=GumballMotion::of_leaf(&self.fields.gesture.as_ref().ok_or_else(invalid)?.entries.get(self.entry).ok_or_else(invalid)?.1);self.phase=if self.motion.is_some(){3}else{4}},
   _=>return Err(invalid()),
  }Ok(self.ready())
 }
 pub fn step(&mut self,source:&GumballGestures,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;self.validate(source)?;
  for _ in 0..512{
   if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
   if self.phase==4{
    let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;
    let step=cx.with_original_retirement(|cx|close_original_retained_step(cx,d,|g|{let turn=self.advance(source,g);RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(self.progress)),work:turn.work}}))?;
    if step.progress()==Default::default(){return Ok(self.ready())}
   }else{
    let turn=self.advance(source,cx.retained_grant());let step=receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(self.progress)),work:turn.work})?;
    if self.ready()||step.progress()==Default::default(){return Ok(self.ready())}
   }
  }Ok(self.ready())
 }
 pub fn next_record(&mut self,source:&GumballGestures,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;self.validate(source)?;if self.phase!=3{return Err(invalid().into())}
  let d=fixed(size_of::<u8>());let mut accepted=None;
  admit_original_construction(cx,&mut accepted,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|{self.phase=4;RetainedWorkOutcome{result:Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})
 }
 fn record_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.key.is_some(){return self.retirement.next_owner_demands::<SharedOwner<String>>(true,&None,copy)}
  if self.fields.gesture.is_some(){return self.retirement.next_owner_demands::<SharedOwner<GumballGesture>>(true,&None,copy)}
  Ok(fixed(0))
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty()||self.fields.key.is_some()||self.fields.gesture.is_some(){return self.record_close_demands(copy)}
  if self.fields.map.is_some(){return self.retirement.next_owner_demands::<OrderedMap<GumballGesture>>(true,&None,copy)}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let turn=if !self.retirement.is_empty(){self.retirement.step_granted(grant)}else if self.fields.key.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.key,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}}else if self.fields.gesture.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.gesture,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}}else{let turn=self.retirement.begin_granted(&mut self.fields.map,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}};
  self.work=turn.work;self.progress=turn.result?.progress();Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.map.is_none()&&self.fields.key.is_none()&&self.fields.gesture.is_none()&&self.retirement.is_empty()}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
}
impl Drop for OriginalGumballRead{fn drop(&mut self){assert!(self.terminal_is_empty(),"original gesture read retains its actual root, entry or source retirement");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}

#[cfg(test)]
#[path="../../../../../../../../../../🫀️foundation/🌊️flow-source/🧪️caller/🦀️.rs"]
mod scratch_testing;
#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod tests;
