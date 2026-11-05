//! 🔢️ Original committed preview-link bytes stream through their genuine Pack writer and one funded hash byte.
use super::retained_flow_source::ProceduralFlowSourceWork;
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value::retained_clone::RetainedOwnedProjection;
use semio_framework_value::retirement::RetireOwned;
use std::mem::ManuallyDrop;

type TreeProject<T> = for<'a>fn(&'a T)->&'a dyn ArtifactCanonicalJsonTree;
const OFFSET:u64=0xcbf2_9ce4_8422_2325;
const PRIME:u64=0x0000_0100_0000_01b3;
const EPOCH:u64=0x9e37_79b9_7f4a_7c15;
fn mix(hash:u64,byte:u8)->u64{(hash^u64::from(byte)).wrapping_mul(PRIME)}
fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn admitted(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}

/// 🧾️ The active job retains the actual source and immutable selected-id owner through this cursor's exact closure.
pub struct PreviewLinkDigestWork<T:RetireOwned+Sync>{
 fixture:TreeProject<T>,generation:Option<TreeProject<T>>,epoch:u64,hash:u64,part:u8,phase:u8,
 owner:ManuallyDrop<Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>>,writer:Option<ArtifactCanonicalJsonTreeCursor>,
 bytes:[u8;64],length:usize,offset:usize,selected:Option<(usize,usize)>,selection_offset:usize,
 output:Option<u64>,prepared:bool,closing:bool,progress:RetainedCloneProgress,work:u64,last_event_progress:RetainedCloneProgress,
}
impl<T:RetireOwned+Sync> PreviewLinkDigestWork<T>{
 pub fn new(fixture:TreeProject<T>,generation:Option<TreeProject<T>>,provisional_generation:u64)->Self{Self{fixture,generation,epoch:provisional_generation,hash:OFFSET,part:0,phase:0,owner:ManuallyDrop::new(None),writer:None,bytes:[0;64],length:0,offset:0,selected:None,selection_offset:0,output:None,prepared:false,closing:false,progress:Default::default(),work:0,last_event_progress:Default::default()}}
 fn validate(&self,source:&ProceduralFlowSourceWork<T>,selected_id:&str)->Result<(),ValueError>{
  source.original_source_demands()?;
  if self.generation.is_some()&&self.selected.is_some_and(|identity|identity!=(selected_id.as_ptr()as usize,selected_id.len())){return Err(fault("preview digest selected source changed"))}Ok(())
 }
 pub fn next_demands(&self,source:&ProceduralFlowSourceWork<T>,selected_id:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}self.validate(source,selected_id)?;if !self.prepared{return source.original_source_demands()}
  match self.phase{0=>Ok(RetainedOwnedProjection::<dyn ArtifactCanonicalJsonTree>::constructor_demand()),1=>Ok(ArtifactCanonicalJsonTreeCursor::constructor_demand()),2=>self.writer.as_ref().ok_or_else(||fault("preview digest writer is absent"))?.next_demand(),3|4|5=>Ok(scalar(1)),6=>Ok(scalar(8)),7=>Ok(Default::default()),_=>Err(fault("preview digest phase is invalid"))}
 }
 pub fn step(&mut self,source:&mut ProceduralFlowSourceWork<T>,selected_id:&str,cx:&mut StepContext<'_>)->Result<Option<u64>,OriginalTurnFailure>{
  self.progress=Default::default();self.work=0;
  if self.closing{return Err(fault("preview digest is closing").into())}
  self.validate(source,selected_id).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;
  if !self.prepared{let ready=source.prepare_original_source(cx)?;self.progress=source.step_progress();self.work=source.last_turn_work();if !ready{return Ok(None)}self.prepared=true;}
  for _ in 0..512{
   if cx.is_cancelled()||cx.should_yield(){return Ok(None)}
   if self.phase==7{return Ok(self.output.take())}
   let g=cx.retained_grant();let d=self.next_demands(source,selected_id,g.maximum_copy_bytes).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;
   if !admitted(d,g){return Ok(None)}
   let turn=self.advance_original(source,selected_id,g);let p=self.last_event_progress; 
   let work=turn.work.max(u64::from(turn.result.is_err()));self.work+=work;
   self.progress=self.progress.checked_add(p).map_err(|error|OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))?;
   cx.consume_fuel(work);
   match turn.result{Ok(())=>cx.consume_retained(p)?,Err(error)=>return Err(OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))}
   if p==Default::default()&&work==0{return Ok(None)}
  }Ok(None)
 }
 fn advance_original(&mut self,source:&ProceduralFlowSourceWork<T>,selected_id:&str,g:RetainedCloneGrant)->RetainedWorkOutcome<()>{
  self.last_event_progress=Default::default();let mut work=1;
  let result=(||{
   match self.phase{
    0=>{
     let project=if self.part==0{self.fixture}else{self.generation.ok_or_else(||fault("preview digest generation projector is absent"))?};
     let turn=source.project_original(0x44494700+usize::from(self.part),project,g);work=turn.work;
     let(owner,p)=turn.result.map_err(|error|{self.last_event_progress=error.retained_progress();error})?;*self.owner=Some(owner);self.last_event_progress=p;
     if self.generation.is_some()&&self.selected.is_none(){self.selected=Some((selected_id.as_ptr()as usize,selected_id.len()))}
     self.phase=1;
    },
    1=>{let owner=self.owner.take().ok_or_else(||fault("preview digest original tree is absent"))?;match ArtifactCanonicalJsonTreeCursor::admit(owner,g){Ok((writer,p))=>{self.writer=Some(writer);self.last_event_progress=p;self.phase=2},Err((error,owner))=>{*self.owner=Some(owner);return Err(error)}}},
    2=>{let writer=self.writer.as_mut().ok_or_else(||fault("preview digest native writer is absent"))?;let turn=writer.advance(&mut self.bytes,g);work=writer.last_turn_work();let step=turn.map_err(|error|{self.last_event_progress=error.retained_progress();error})?;self.last_event_progress=step.ownership.progress();self.length=step.written_bytes;self.offset=0;if self.length>0{self.phase=3}else if writer.is_complete(){self.writer=None;self.phase=4}},
    3=>{self.hash=mix(self.hash,self.bytes[self.offset]);self.offset+=1;self.last_event_progress=RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()};if self.offset==self.length{self.phase=2}},
    4=>{self.hash=mix(self.hash,0x1f);self.last_event_progress=RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()};self.phase=if self.part==0&&self.generation.is_some(){self.part=1;0}else if self.part==1{5}else{6}},
    5=>{let byte=selected_id.as_bytes().get(self.selection_offset).copied().unwrap_or(0x1f);self.hash=mix(self.hash,byte);self.last_event_progress=RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()};if self.selection_offset==selected_id.len(){self.phase=6}else{self.selection_offset+=1}},
    6=>{self.output=Some(self.hash^self.epoch.wrapping_mul(EPOCH));self.last_event_progress=RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()};self.phase=7},
    _=>{work=0;return Err(fault("preview digest event is terminal"))},
   }Ok(())
  })();RetainedWorkOutcome{result,work}
 }
 pub fn restore_output(&mut self,output:u64)->Result<(),u64>{if self.phase!=7||self.output.is_some(){Err(output)}else{self.output=Some(output);Ok(())}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(writer)=self.writer.as_mut(){writer.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(writer)=self.writer.as_ref(){return writer.next_demand()}if let Some(owner)=self.owner.as_ref(){return owner.next_close_demand(copy)}if self.output.is_some(){return Ok(scalar(8))}Ok(Default::default())}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;
  let result=(||{
   if !self.closing{return Err(fault("preview digest close was not begun"))}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   let d=self.next_close_demands(g.maximum_copy_bytes)?;if !admitted(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if let Some(writer)=self.writer.as_mut(){let turn=writer.close_step(g);self.work=writer.last_turn_work();let step=turn.map_err(|error|{self.progress=error.retained_progress();error})?;self.progress=step.progress();if writer.terminal_is_empty(){self.writer=None}}
   else if let Some(owner)=self.owner.as_mut(){let turn=owner.close_step(g);self.work=turn.work;self.progress=turn.result.map_err(|error|{self.progress=error.retained_progress();error})?.progress();if owner.terminal_is_empty(){self.owner.take();}}
   else if self.output.take().is_some(){self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()}}
   Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(self.progress)}else{RetainedCloneStep::Progress(self.progress)})
  })().map_err(|error:ValueError|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn terminal_is_empty(&self)->bool{(self.closing||self.phase==7)&&self.owner.is_none()&&self.writer.is_none()&&self.output.is_none()}
}
impl<T:RetireOwned+Sync> Drop for PreviewLinkDigestWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"preview digest retains original tree ownership or output");unsafe{ManuallyDrop::drop(&mut self.owner)}}}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
