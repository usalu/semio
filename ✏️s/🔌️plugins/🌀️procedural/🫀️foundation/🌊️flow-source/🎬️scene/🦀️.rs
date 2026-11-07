//! 🎬️ Procedural evaluation retains one genuine read while native scratch fields and original provisional leaves settle.
use super::retained_scratch_driver::funded_construction;
use super::retained_flow_source::ProceduralFlowSourceWork;
use super::retained_projected_clone::ProjectedFieldCloneWork;
use super::retained_provisional_replay::ProvisionalReplayWork;
use super::retained_native_provisional_fold::NativeProvisionalFolds;
use super::GenerationPlayState;
use semio_plugin::{ArtifactApp,app::ArtifactOwnedContextHandle};
use semio_os_kernel::os_store::snapshot_read::SnapshotRead;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_pack_json::native_decoding::owned::NativeReceive;
use semio_framework_job::{OriginalSourceBinding,StepContext,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedOwnedProjection,RetainedCloneClose};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Borrow,Host,ProjectGeneration,CloneGeneration,TakeGeneration,CloseGeneration,Replay,Ready}
struct Fields<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>>where A::Mutation:NativeReceive<DslValue>{
 source:ProceduralFlowSourceWork<A::Snapshot>,clone:Option<ProjectedFieldCloneWork<GenerationPlayState>>,generation:Option<GenerationPlayState>,replay:ProvisionalReplayWork<A,S>
}
pub struct ProvisionalSceneWork<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>>where A::Mutation:NativeReceive<DslValue>{
 fields:ManuallyDrop<Fields<A,S>>,project:for<'a>fn(&'a A::Snapshot)->&'a GenerationPlayState,original:OriginalSourceBinding,context:bool,phase:Phase,closing:bool,retirement:RetainedCloneClose,progress:RetainedCloneProgress,work:u64
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scene changed its genuine captured source or preparation phase")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
impl<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>> ProvisionalSceneWork<A,S>where A::Mutation:NativeReceive<DslValue>{
 pub fn new(read:SnapshotRead<A::Snapshot>,host:for<'a>fn(&'a A::Snapshot)->&'a FlowHostSnapshot,generation:for<'a>fn(&'a A::Snapshot)->&'a GenerationPlayState,revision:[u8;32],business_generation:u64,cx:&StepContext<'_>)->Self{
  let original=OriginalSourceBinding::borrow(cx);Self::with_replay_original(read,host,generation,revision,business_generation,ProvisionalReplayWork::new_original(original),false,original)
 }
 pub fn with_replay_original(read:SnapshotRead<A::Snapshot>,host:for<'a>fn(&'a A::Snapshot)->&'a FlowHostSnapshot,generation:for<'a>fn(&'a A::Snapshot)->&'a GenerationPlayState,revision:[u8;32],business_generation:u64,replay:ProvisionalReplayWork<A,S>,context:bool,original:OriginalSourceBinding)->Self{
  Self{fields:ManuallyDrop::new(Fields{source:ProceduralFlowSourceWork::new_original(read,host,revision,business_generation,true,original),clone:None,generation:None,replay}),project:generation,original,context,phase:Phase::Borrow,closing:false,retirement:Default::default(),progress:Default::default(),work:0}
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
 pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid())}
  match self.phase{
   Phase::Borrow=>self.fields.source.original_source_demands(),
   Phase::Host=>self.fields.source.next_demands(copy),
   Phase::ProjectGeneration=>{let mut d=RetainedOwnedProjection::<GenerationPlayState>::constructor_demand();d.copy_bytes=d.copy_bytes.checked_add(size_of::<ProjectedFieldCloneWork<GenerationPlayState>>()).ok_or_else(invalid)?;Ok(d)},
   Phase::CloneGeneration|Phase::TakeGeneration=>self.fields.clone.as_ref().ok_or_else(invalid)?.next_demands(copy),
   Phase::CloseGeneration=>match self.fields.clone.as_ref(){Some(work)if !work.terminal_is_empty()=>work.next_close_demands(copy),_=>Ok(fixed(0))},
   Phase::Replay=>self.fields.replay.next_demands(self.fields.source.source()?.ok_or_else(invalid)?,self.fields.generation.as_ref().ok_or_else(invalid)?,copy),
   Phase::Ready=>Ok(Default::default()),
  }
 }
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  match self.phase{
   Phase::Borrow=>{if self.fields.source.prepare_original_source(cx)?{self.phase=Phase::Host}},
   Phase::Host=>{if self.fields.source.step(cx)?{self.phase=Phase::ProjectGeneration}},
   Phase::ProjectGeneration=>{
    let d=self.next_demands(cx.retained_grant().maximum_copy_bytes)?;let original=self.original;let project=self.project;
    let fields=&mut*self.fields;let(source,clone)=(&fields.source,&mut fields.clone);
    if funded_construction(cx,clone,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|g|{
     let turn=source.project_original(2,project,g);
     let result=turn.result.map(|(projection,mut p)|{p.copied_bytes+=size_of::<ProjectedFieldCloneWork<GenerationPlayState>>();(ProjectedFieldCloneWork::new_original(projection,original),p)});
     RetainedWorkOutcome{result,work:turn.work}
    })?{self.phase=Phase::CloneGeneration}
   },
   Phase::CloneGeneration=>{if self.fields.clone.as_mut().ok_or_else(invalid)?.step(cx)?{self.phase=Phase::TakeGeneration}},
   Phase::TakeGeneration=>{
    let fields=&mut*self.fields;if fields.clone.as_mut().ok_or_else(invalid)?.take_into(&mut fields.generation,cx)?{fields.clone.as_mut().unwrap().begin_close();self.phase=Phase::CloseGeneration}
   },
   Phase::CloseGeneration=>{
    if self.fields.clone.as_ref().ok_or_else(invalid)?.terminal_is_empty(){
     let mut accepted=None;let fields=&mut*self.fields;let phase=&mut self.phase;let context=self.context;
     admit_original_construction(cx,&mut accepted,OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:0,release_bytes:0,depth:1},|_|{fields.clone=None;*phase=if context{Phase::Replay}else{Phase::Ready};RetainedWorkOutcome{result:Ok(((),RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})?;
    }else{
     let work=self.fields.clone.as_mut().unwrap();let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
     cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|g|work.close_step(g))?;Ok(())})?;
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
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.generation.is_some(){return self.retirement.next_owner_demands::<GenerationPlayState>(true,&None,copy)}
  self.fields.source.next_close_demands(copy)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.fields.replay.terminal_is_empty(){let turn=self.fields.replay.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress()}
  else if let Some(work)=self.fields.clone.as_mut(){if work.terminal_is_empty(){self.fields.clone=None;self.work=1;self.progress.copied_items=1}else{let turn=work.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress()}}
  else if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);self.work=turn.work;self.progress=turn.result?.progress()}
  else if self.fields.generation.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.generation,grant);self.work=turn.work;self.progress=turn.result?.map_or(Default::default(),|step|step.progress())}
  else{let turn=self.fields.source.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress()}
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.replay.terminal_is_empty()&&self.fields.clone.is_none()&&self.fields.generation.is_none()&&self.retirement.is_empty()&&self.fields.source.terminal_is_empty()}
}
impl<A:ArtifactApp,S:NativeProvisionalFolds<A::Mutation>> Drop for ProvisionalSceneWork<A,S>where A::Mutation:NativeReceive<DslValue>{
 fn drop(&mut self){assert!(self.terminal_is_empty(),"procedural scene retains its original read, context, native scratch or clone owners");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}
}
