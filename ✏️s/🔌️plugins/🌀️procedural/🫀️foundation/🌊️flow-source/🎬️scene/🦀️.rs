//! 🎬️ Procedural evaluation retains one genuine read while native scratch fields and original provisional leaves settle.
use super::retained_scratch_driver::{begun,funded_source_turn,funded_turn,pay,replied,turn_work};
use super::retained_flow_source::ProceduralFlowSourceWork;
use super::retained_projected_clone::ProjectedFieldCloneWork;
use super::retained_provisional_replay::ProvisionalReplayWork;
use super::retained_native_provisional_fold::NativeProvisionalFolds;
use super::GenerationPlayState;
use semio_plugin::{ArtifactApp,app::ArtifactOwnedContextHandle};
use semio_os_kernel::os_store::snapshot_read::SnapshotRead;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_pack_json::native_decoding::owned::NativeReceive;
use semio_framework_job::{OriginalSourceBinding,StepContext,OriginalTurnFailure};
use semio_framework_value::{DslValue,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedNativeFieldProjection,RetainedOwnedProjection,RetainedOwnedProjectionAdmission,RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Borrow,Host,ProjectGeneration,TakeProjection,CloneGeneration,TakeGeneration,CloseGeneration,Replay,Ready}
struct Fields<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>>where A::Mutation:NativeReceive<DslValue>{
 source:ProceduralFlowSourceWork<A::Snapshot>,clone:Option<ProjectedFieldCloneWork<GenerationPlayState>>,generation:Option<GenerationPlayState>,replay:ProvisionalReplayWork<A,S>,admission:RetainedOwnedProjectionAdmission<GenerationPlayState>,projected:Option<RetainedOwnedProjection<GenerationPlayState>>
}
pub struct ProvisionalSceneWork<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>>where A::Mutation:NativeReceive<DslValue>{
 fields:ManuallyDrop<Fields<A,S>>,project:RetainedNativeFieldProjection<A::Snapshot,GenerationPlayState>,original:OriginalSourceBinding,context:bool,phase:Phase,closing:bool,retirement:RetainedCloneClose,progress:RetainedCloneProgress
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scene changed its genuine captured source or preparation phase")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
impl<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>> ProvisionalSceneWork<A,S>where A::Mutation:NativeReceive<DslValue>{
 /// 🪪️ Binds one genuine read to its Flow and generation fields.
 ///
 /// # Safety
 /// `host` and `generation` must each return only their field of the input and be bounded, callback-free, nonallocating and nonunwinding.
 pub unsafe fn new(read:SnapshotRead<A::Snapshot>,host:for<'a>fn(&'a A::Snapshot)->&'a FlowHostSnapshot,generation:for<'a>fn(&'a A::Snapshot)->&'a GenerationPlayState,revision:[u8;32],business_generation:u64,cx:&StepContext<'_>)->Self{
  let original=OriginalSourceBinding::borrow(cx);unsafe{Self::with_replay_original(read,host,generation,revision,business_generation,ProvisionalReplayWork::new_original(original),false,original)}
 }
 /// 🪪️ Binds one genuine read and an existing replay under an existing original binding.
 ///
 /// # Safety
 /// See [`Self::new`].
 pub unsafe fn with_replay_original(read:SnapshotRead<A::Snapshot>,host:for<'a>fn(&'a A::Snapshot)->&'a FlowHostSnapshot,generation:for<'a>fn(&'a A::Snapshot)->&'a GenerationPlayState,revision:[u8;32],business_generation:u64,replay:ProvisionalReplayWork<A,S>,context:bool,original:OriginalSourceBinding)->Self{
  Self{fields:ManuallyDrop::new(Fields{source:unsafe{ProceduralFlowSourceWork::new_original(read,host,revision,business_generation,true,original)},clone:None,generation:None,replay,admission:Default::default(),projected:None}),project:unsafe{RetainedNativeFieldProjection::new(generation,2)},original,context,phase:Phase::Borrow,closing:false,retirement:Default::default(),progress:Default::default()}
 }
 fn authenticate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.closing||!self.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;Ok(())}
 pub fn receive_context(&mut self,context:&ArtifactOwnedContextHandle<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.authenticate(cx)?;if self.phase!=Phase::Borrow{return Err(invalid().into())}let ready=self.fields.replay.receive_context(context,cx)?;if ready{self.context=true}Ok(ready)}
 pub fn ready(&self)->bool{!self.closing&&self.phase==Phase::Ready}
 pub fn source_owner(&self)->&ProceduralFlowSourceWork<A::Snapshot>{&self.fields.source}
 pub fn source_owner_mut(&mut self)->&mut ProceduralFlowSourceWork<A::Snapshot>{&mut self.fields.source}
 pub fn source(&self)->Result<Option<&FlowHostSnapshot>,ValueError>{if self.ready(){self.fields.source.source()}else{Ok(None)}}
 pub fn generation(&self)->Option<&GenerationPlayState>{if self.ready(){self.fields.generation.as_ref()}else{None}}
 pub fn host_and_generation_mut(&mut self)->Result<Option<(&mut FlowHostSnapshot,&GenerationPlayState)>,ValueError>{
  if !self.ready(){return Ok(None)}let fields=&mut*self.fields;
  Ok(Some((fields.source.source_mut()?.ok_or_else(invalid)?,fields.generation.as_ref().ok_or_else(invalid)?)))
 }
 pub fn provisional_generation(&self)->Option<u64>{self.fields.replay.generation()}
 pub fn next_demands(&self,copy:usize,work:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid())}
  match self.phase{
   Phase::Borrow=>self.fields.source.original_source_demands(work),
   Phase::Host=>self.fields.source.next_demands(copy,work),
   Phase::ProjectGeneration=>Ok(RetainedOwnedProjection::<GenerationPlayState>::constructor_demand()),
   Phase::TakeProjection=>{let mut d=self.fields.admission.take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<ProjectedFieldCloneWork<GenerationPlayState>>()).ok_or_else(invalid)?;Ok(d)},
   Phase::CloneGeneration|Phase::TakeGeneration=>self.fields.clone.as_ref().ok_or_else(invalid)?.next_demands(copy,work),
   Phase::CloseGeneration=>match self.fields.clone.as_ref(){Some(work)if !work.terminal_is_empty()=>work.next_close_demands(copy),_=>Ok(fixed(0))},
   Phase::Replay=>self.fields.replay.next_demands(self.fields.source.source()?.ok_or_else(invalid)?,self.fields.generation.as_ref().ok_or_else(invalid)?,copy,work),
   Phase::Ready=>Ok(Default::default()),
  }
 }
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  match self.phase{
   Phase::Borrow=>{if self.fields.source.prepare_original_source(cx)?{self.phase=Phase::Host}},
   Phase::Host=>{if self.fields.source.step(cx)?{self.phase=Phase::ProjectGeneration}},
   Phase::ProjectGeneration=>{
    let d=self.next_demands(cx.retained_grant().maximum_copy_bytes,turn_work(cx))?;
    let fields=&mut*self.fields;let(source,admission)=(&fields.source,&mut fields.admission);let witness=&self.project;
    let step=funded_source_turn(cx,|effect|{
     if !retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
     let result=match replied(effect,|effect,reply|source.project_original(witness,admission,effect,reply)){Ok(Some(result))=>result,Ok(None)=>Err(invalid()),Err(error)=>Err(error)};
     RetainedWorkOutcome{result,work:effect.performed_work()}
    })?;
    if step.is_some_and(|step|step.progress()!=Default::default()){self.phase=Phase::TakeProjection}
   },
   Phase::TakeProjection=>{
    let d=self.next_demands(cx.retained_grant().maximum_copy_bytes,turn_work(cx))?;let original=self.original;
    let fields=&mut*self.fields;let(admission,projected,clone)=(&mut fields.admission,&mut fields.projected,&mut fields.clone);
    funded_source_turn(cx,|effect|{
     if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units().map_or(true,|remaining|remaining<1){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
     let result=match replied(effect,|effect,reply|admission.take(projected,effect,reply)){Ok(Some(result))=>result.map(|step|{if let Some(projection)=projected.take(){*clone=Some(ProjectedFieldCloneWork::new_original(projection,original))}RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:step.progress().copied_bytes+size_of::<ProjectedFieldCloneWork<GenerationPlayState>>(),..step.progress()})}),Ok(None)=>Err(invalid()),Err(error)=>Err(error)};
     RetainedWorkOutcome{result,work:effect.performed_work()}
    })?;
    if self.fields.clone.is_some(){self.phase=Phase::CloneGeneration}
   },
   Phase::CloneGeneration=>{if self.fields.clone.as_mut().ok_or_else(invalid)?.step(cx)?{self.phase=Phase::TakeGeneration}},
   Phase::TakeGeneration=>{
    let fields=&mut*self.fields;if fields.clone.as_mut().ok_or_else(invalid)?.take_into(&mut fields.generation,cx)?{fields.clone.as_mut().unwrap().begin_close();self.phase=Phase::CloseGeneration}
   },
   Phase::CloseGeneration=>{
    if self.fields.clone.as_ref().ok_or_else(invalid)?.terminal_is_empty(){
     let fields=&mut*self.fields;let phase=&mut self.phase;let context=self.context;
     funded_turn(cx,|effect|{let result=match pay(effect,1){Ok(true)=>{fields.clone=None;*phase=if context{Phase::Replay}else{Phase::Ready};Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))},Ok(false)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)};RetainedWorkOutcome{result,work:effect.performed_work()}})?;
    }else{
     let work=self.fields.clone.as_mut().unwrap();let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
     cx.with_original_retirement(|cx|semio_framework_job::close_original_effect_step(cx,d,|effect|{let body=effect.grant().maximum_copy_bytes;work.close_step(body,effect)}))?;
    }
   },
   Phase::Replay=>{
    let fields=&mut*self.fields;
    if fields.replay.step(fields.source.source_mut()?.ok_or_else(invalid)?,fields.generation.as_mut().ok_or_else(invalid)?,cx)?{self.phase=Phase::Ready}
   },
   Phase::Ready=>{},
  }Ok(self.ready())
 }
 pub fn begin_close(&mut self){self.closing=true;self.fields.replay.begin_close();if let Some(work)=self.fields.clone.as_mut(){work.begin_close()}self.fields.source.begin_close()}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.fields.replay.terminal_is_empty(){return self.fields.replay.next_close_demands(copy)}
  if let Some(work)=self.fields.clone.as_ref(){return if work.terminal_is_empty(){Ok(fixed(0))}else{work.next_close_demands(copy)}}
  if let Some(projection)=self.fields.projected.as_ref(){return projection.next_close_demand(copy)}
  if !self.fields.admission.terminal_is_empty(){return self.fields.admission.close_demand(copy)}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.generation.is_some(){return self.retirement.next_owner_demands::<GenerationPlayState>(true,&None,copy)}
  self.fields.source.next_close_demands(copy)
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let result=self.close_original(body,effect);RetirementPollExit::settle(effect,result)
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(invalid().into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(effect.grant(),self.next_close_demands(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let step=if !self.fields.replay.terminal_is_empty(){RetirementPollExit::resume(self.fields.replay.close_step(body,effect))?}
  else if let Some(work)=self.fields.clone.as_mut(){if work.terminal_is_empty(){self.fields.clone=None;RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{RetirementPollExit::resume(work.close_step(body,effect))?}}
  else if let Some(projection)=self.fields.projected.as_mut(){let step=RetirementPollExit::resume(projection.close_step(body,effect))?;if projection.terminal_is_empty(){self.fields.projected=None}step}
  else if !self.fields.admission.terminal_is_empty(){RetirementPollExit::resume(self.fields.admission.close_step(body,effect))?}
  else if !self.retirement.is_empty(){RetirementPollExit::resume(self.retirement.step_granted(body,effect))?}
  else if self.fields.generation.is_some(){begun(&mut self.retirement,&mut self.fields.generation,effect)?}
  else{RetirementPollExit::resume(self.fields.source.close_step(body,effect))?};
  self.progress=step.progress();Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.replay.terminal_is_empty()&&self.fields.clone.is_none()&&self.fields.projected.is_none()&&self.fields.admission.terminal_is_empty()&&self.fields.generation.is_none()&&self.retirement.is_empty()&&self.fields.source.terminal_is_empty()}
}
impl<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>> Drop for ProvisionalSceneWork<A,S>where A::Mutation:NativeReceive<DslValue>{
 fn drop(&mut self){assert!(self.terminal_is_empty(),"procedural scene retains its original read, context, native scratch or clone owners");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}
}
