//! 🔢️ Original committed preview-link bytes stream through their genuine Pack writer and one funded hash byte.
use super::retained_flow_source::ProceduralFlowSourceWork;
use semio_s_artifact_procedural_retained_source::retained_scratch_driver::{funded_source_turn,funded_turn,pay,replied,turn_work};
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetirementPollExit,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value::retained_clone::{RetainedNativeFieldProjection,RetainedOwnedProjection,RetainedOwnedProjectionAdmission,retained_clone_grant_admits};
use semio_framework_value::retirement::RetireOwned;
use std::mem::ManuallyDrop;

type TreeProject<T> = for<'a>fn(&'a T)->&'a dyn ArtifactCanonicalJsonTree;
type TreeWitness<T> = RetainedNativeFieldProjection<T,dyn ArtifactCanonicalJsonTree>;
const OFFSET:u64=0xcbf2_9ce4_8422_2325;
const PRIME:u64=0x0000_0100_0000_01b3;
const EPOCH:u64=0x9e37_79b9_7f4a_7c15;
const FIXTURE_DISCRIMINATOR:usize=0x44494700;
fn mix(hash:u64,byte:u8)->u64{(hash^u64::from(byte)).wrapping_mul(PRIME)}
fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn admitted(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn local(copy_bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()}}

/// 🧾️ The active job retains the actual source and immutable selected-id owner through this cursor's exact closure.
pub struct PreviewLinkDigestWork<T:RetireOwned+Sync>{
 fixture:TreeWitness<T>,generation:Option<TreeWitness<T>>,epoch:u64,hash:u64,part:u8,phase:u8,
 admission:RetainedOwnedProjectionAdmission<dyn ArtifactCanonicalJsonTree>,projected:bool,
 owner:ManuallyDrop<Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>>,writer:Option<ArtifactCanonicalJsonTreeCursor>,
 bytes:[u8;64],length:usize,offset:usize,selected:Option<(usize,usize)>,selection_offset:usize,
 output:Option<u64>,prepared:bool,closing:bool,progress:RetainedCloneProgress,
}
impl<T:RetireOwned+Sync> PreviewLinkDigestWork<T>{
 /// # Safety
 /// Both accessors must return only their own initialized field of the retained source and be bounded, callback-free, nonallocating and nonunwinding.
 pub unsafe fn new(fixture:TreeProject<T>,generation:Option<TreeProject<T>>,provisional_generation:u64)->Self{Self{fixture:unsafe{TreeWitness::new(fixture,FIXTURE_DISCRIMINATOR)},generation:generation.map(|project|unsafe{TreeWitness::new(project,FIXTURE_DISCRIMINATOR+1)}),epoch:provisional_generation,hash:OFFSET,part:0,phase:0,admission:Default::default(),projected:false,owner:ManuallyDrop::new(None),writer:None,bytes:[0;64],length:0,offset:0,selected:None,selection_offset:0,output:None,prepared:false,closing:false,progress:Default::default()}}
 fn validate(&self,source:&ProceduralFlowSourceWork<T>,selected_id:&str,work:u64)->Result<(),ValueError>{
  source.original_source_demands(work)?;
  if self.generation.is_some()&&self.selected.is_some_and(|identity|identity!=(selected_id.as_ptr()as usize,selected_id.len())){return Err(fault("preview digest selected source changed"))}Ok(())
 }
 pub fn next_demands(&self,source:&ProceduralFlowSourceWork<T>,selected_id:&str,copy:usize,work:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}self.validate(source,selected_id,work)?;if !self.prepared{return source.original_source_demands(work)}
  match self.phase{0=>if self.projected{self.admission.take_demand()}else{Ok(RetainedOwnedProjection::<dyn ArtifactCanonicalJsonTree>::constructor_demand())},1=>Ok(ArtifactCanonicalJsonTreeCursor::constructor_demand()),2=>self.writer.as_ref().ok_or_else(||fault("preview digest writer is absent"))?.next_demand(),3|4|5=>Ok(scalar(1)),6=>Ok(scalar(8)),7=>Ok(Default::default()),_=>Err(fault("preview digest phase is invalid"))}
 }
 pub fn step(&mut self,source:&mut ProceduralFlowSourceWork<T>,selected_id:&str,cx:&mut StepContext<'_>)->Result<Option<u64>,OriginalTurnFailure>{
  self.progress=Default::default();
  if self.closing{return Err(fault("preview digest is closing").into())}
  self.validate(source,selected_id,turn_work(cx)).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;
  if !self.prepared{if !source.prepare_original_source(cx)?{return Ok(None)}self.progress=source.step_progress();self.prepared=true;}
  for _ in 0..512{
   if cx.is_cancelled()||cx.should_yield(){return Ok(None)}
   if self.phase==7{return Ok(self.output.take())}
   let g=cx.retained_grant();let d=self.next_demands(source,selected_id,g.maximum_copy_bytes,turn_work(cx)).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;
   if !admitted(d,g){return Ok(None)}
   let sourced=self.phase==0;
   let run=|effect:&mut RetainedAllocationEffect<'_>|{let result=self.advance_original(source,selected_id,effect);RetainedWorkOutcome{result,work:effect.performed_work()}};
   let step=if sourced{funded_source_turn(cx,run)?}else{funded_turn(cx,run)?};
   let Some(step)=step else{return Ok(None)};let p=step.progress();
   self.progress=self.progress.checked_add(p).map_err(|error|OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))?;
   if p==Default::default(){return Ok(None)}
  }Ok(None)
 }
 fn advance_original(&mut self,source:&ProceduralFlowSourceWork<T>,selected_id:&str,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let g=effect.grant();let idle=RetainedCloneStep::Progress(Default::default());
  let progress=match self.phase{
   0=>if !self.projected{
    let witness=if self.part==0{&self.fixture}else{self.generation.as_ref().ok_or_else(||fault("preview digest generation projector is absent"))?};
    let admission=&mut self.admission;
    let step=replied(effect,|effect,reply|source.project_original(witness,admission,effect,reply))?.ok_or_else(||fault("preview digest projection suspended"))??;
    self.projected=step.progress()!=Default::default();step.progress()
   }else{
    if effect.remaining_work_units()?<1{return Ok(idle)}
    let admission=&mut self.admission;let owner=&mut *self.owner;
    let step=replied(effect,|effect,reply|admission.take(owner,effect,reply))?.ok_or_else(||fault("preview digest projection take suspended"))??;
    if self.owner.is_some(){
     if self.generation.is_some()&&self.selected.is_none(){self.selected=Some((selected_id.as_ptr()as usize,selected_id.len()))}
     self.projected=false;self.phase=1;
    }step.progress()
   },
   1=>{
    if !pay(effect,1)?{return Ok(idle)}
    let owner=self.owner.take().ok_or_else(||fault("preview digest original tree is absent"))?;
    match ArtifactCanonicalJsonTreeCursor::admit(owner,g){Ok((writer,p))=>{self.writer=Some(writer);self.phase=2;p},Err((error,owner))=>{*self.owner=Some(owner);return Err(error)}}
   },
   2=>{
    let writer=self.writer.as_mut().ok_or_else(||fault("preview digest native writer is absent"))?;
    let step=writer.advance(&mut self.bytes,effect)?;
    self.length=step.written_bytes;self.offset=0;if self.length>0{self.phase=3}else if writer.is_complete(){self.writer=None;self.phase=4}
    step.ownership.progress()
   },
   3=>{if !pay(effect,1)?{return Ok(idle)}self.hash=mix(self.hash,self.bytes[self.offset]);self.offset+=1;if self.offset==self.length{self.phase=2}local(1)},
   4=>{if !pay(effect,1)?{return Ok(idle)}self.hash=mix(self.hash,0x1f);self.phase=if self.part==0&&self.generation.is_some(){self.part=1;0}else if self.part==1{5}else{6};local(1)},
   5=>{if !pay(effect,1)?{return Ok(idle)}let byte=selected_id.as_bytes().get(self.selection_offset).copied().unwrap_or(0x1f);self.hash=mix(self.hash,byte);if self.selection_offset==selected_id.len(){self.phase=6}else{self.selection_offset+=1}local(1)},
   6=>{if !pay(effect,1)?{return Ok(idle)}self.output=Some(self.hash^self.epoch.wrapping_mul(EPOCH));self.phase=7;local(8)},
   _=>return Err(fault("preview digest event is terminal")),
  };
  Ok(RetainedCloneStep::Progress(progress))
 }
 pub fn restore_output(&mut self,output:u64)->Result<(),u64>{if self.phase!=7||self.output.is_some(){Err(output)}else{self.output=Some(output);Ok(())}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(writer)=self.writer.as_mut(){writer.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(writer)=self.writer.as_ref(){return writer.next_demand()}if !self.admission.terminal_is_empty(){return self.admission.close_demand(copy)}if let Some(owner)=self.owner.as_ref(){return owner.next_close_demand(copy)}if self.output.is_some(){return Ok(scalar(8))}Ok(Default::default())}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let result=self.close_original(body,effect);RetirementPollExit::settle(effect,result)
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(fault("preview digest close was not begun").into())}
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let g=effect.grant();let d=self.next_close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(writer)=self.writer.as_mut(){self.progress=writer.close_step(effect)?.progress();if writer.terminal_is_empty(){self.writer=None}}
  else if !self.admission.terminal_is_empty(){self.progress=RetirementPollExit::resume(self.admission.close_step(body,effect))?.progress()}
  else if let Some(owner)=self.owner.as_mut(){self.progress=RetirementPollExit::resume(owner.close_step(body,effect))?.progress();if owner.terminal_is_empty(){self.owner.take();}}
  else if self.output.is_some(){RetirementPollExit::pay(effect,1)?;self.output=None;self.progress=local(8)}
  Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(self.progress)}else{RetainedCloneStep::Progress(self.progress)})
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn terminal_is_empty(&self)->bool{(self.closing||self.phase==7)&&self.owner.is_none()&&self.writer.is_none()&&self.output.is_none()&&self.admission.terminal_is_empty()}
}
impl<T:RetireOwned+Sync> Drop for PreviewLinkDigestWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"preview digest retains original tree ownership or output");unsafe{ManuallyDrop::drop(&mut self.owner)}}}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
