//! 🧮️ Original evaluation retains its qualified source until session publication and full child closure.
use super::super::{FlowCommand,FlowSnapshot,FlowApp,FlowInstanceOperationOwner};
use crate::editor::flow::host::{FlowComposedSourceWork,FlowComposedScene};
use super::super::child_publication::FlowChildPublicationWork;
use semio_plugin::{ArtifactInstanceOperationOwnerHandle,EditorApp,app::ArtifactOwnedToolJobContext,AppOperationContext};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,InteractiveJobCloseStep as Close};
use crate::retained::{close_with_effect,poll_outcome,begin_close_owner};
use semio_framework_value::{RetireOwned,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
type Infos=<flow::registry::FlowOperatorInfoMap as std::ops::Deref>::Target;
type Reference=(u64,usize,usize,usize);
#[derive(Clone,Copy,PartialEq,Eq)]
struct Binding{source:OriginalSourceBinding,parent:Reference,command:Reference,infos:usize,context:usize}
#[derive(RetireOwned)]
struct Fields{scene:Option<FlowComposedScene>}
pub use flow::FlowEvaluationPrepared;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Flow evaluation changed its source, caller or publication custody")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn progress(bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}
pub struct FlowEvaluationCommandWork{source:FlowComposedSourceWork,child:FlowChildPublicationWork,pipeline:flow::FlowEvaluationPipeline,fields:ManuallyDrop<Option<Fields>>,retirement:RetainedCloneClose,binding:Option<Binding>,phase:u8,closing:bool}
impl FlowEvaluationCommandWork{
 pub fn new(evaluation:flow::FlowEvaluationWork)->Self{Self{source:FlowComposedSourceWork::new(),child:FlowChildPublicationWork::new(),pipeline:flow::FlowEvaluationPipeline::new(evaluation),fields:ManuallyDrop::new(Some(Fields{scene:None})),retirement:Default::default(),binding:None,phase:0,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{if cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(fixed(bytes)){return Ok(false)}cx.consume_fuel(1);cx.consume_retained(progress(bytes))?;Ok(true)}
 fn paid_close(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{if cx.retirement_should_yield()||!cx.retained_work_can_enter(fixed(bytes)){return Ok(false)}cx.consume_fuel(1);cx.consume_retained(progress(bytes))?;Ok(true)}
 pub fn step<A:FlowApp>(&mut self,parent:RetainedCloneRef<'_,FlowSnapshot>,command:RetainedCloneRef<'_,FlowCommand>,context:&ArtifactOwnedToolJobContext<EditorApp<A>>,operation:&AppOperationContext,owner:&ArtifactInstanceOperationOwnerHandle,infos:&Infos,cx:&mut StepContext<'_>)->Result<Option<FlowEvaluationPrepared>,OriginalTurnFailure>{for _ in 0..512{let before=(cx.fuel_remaining(),cx.retained_progress());if let Some(output)=self.once(parent,command,context,operation,owner,infos,cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.fuel_remaining(),cx.retained_progress()){break}}Ok(None)}
 fn once<A:FlowApp>(&mut self,parent:RetainedCloneRef<'_,FlowSnapshot>,command:RetainedCloneRef<'_,FlowCommand>,context:&ArtifactOwnedToolJobContext<EditorApp<A>>,operation:&AppOperationContext,owner:&ArtifactInstanceOperationOwnerHandle,infos:&Infos,cx:&mut StepContext<'_>)->Result<Option<FlowEvaluationPrepared>,OriginalTurnFailure>{
  let binding=Binding{source:OriginalSourceBinding::borrow(cx),parent:parent.reference_identity(),command:command.reference_identity(),infos:infos as*const _ as usize,context:context as*const _ as usize};if self.closing||self.binding.is_some_and(|prior|prior!=binding)||!context.provisional().is_empty(){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}if self.binding.is_none(){if Self::paid(cx,size_of::<Option<Binding>>())?{self.binding=Some(binding)}return Ok(None)}
  match self.phase{
   0=>if self.child.read_step(parent,command,&context.children,operation,cx)?{self.phase=1},
   1=>if self.source.step(parent,Some(self.child.child_source()?),cx)?{self.phase=2},
   2=>{let d=self.source.scene_alias_demand()?;if !cx.retained_work_can_enter(d){return Ok(None)}let turn=self.source.retain_scene(cx.retained_grant());cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));match turn.result{Ok(Some((scene,p)))=>{self.fields.as_mut().ok_or_else(invalid)?.scene=Some(scene);self.phase=3;cx.consume_retained(p)?},Ok(None)=>{},Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))}},
   3=>{let binding=self.source.original_binding().ok_or_else(invalid)?;let fields=self.fields.as_mut().ok_or_else(invalid)?;if owner.with_original_mut::<FlowInstanceOperationOwner,_>(|instance|instance.publish_scene_original(context,binding,&mut fields.scene,cx))?.unwrap_or(false){self.phase=4}},
   4=>{let source=self.source.read_source(parent.get())?;let pipeline=&mut self.pipeline;if let Some(output)=owner.with_original_mut::<FlowInstanceOperationOwner,_>(|instance|instance.with_original_session(|session|pipeline.step(&source,infos,session,cx)))?.flatten(){self.phase=5;return Ok(Some(output))}},
   _=>return Err(invalid().into())
  }Ok(None)
 }
 pub fn source<'a>(&'a self,parent:&'a FlowSnapshot)->Result<crate::editor::flow::host::FlowComposedRead<'a>,ValueError>{self.source.read_source(parent)}
 pub fn restore_output(&mut self,output:FlowEvaluationPrepared){assert_eq!(self.phase,5);self.pipeline.restore_output(output);self.phase=4}
 pub fn begin_close(&mut self){self.closing=true;self.source.begin_close();self.child.begin_close();self.pipeline.begin_close()}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if !self.pipeline.terminal_is_empty(){return self.pipeline.close_demands(copy)}if !self.source.terminal_is_empty(){return self.source.next_close_demands(copy)}if !self.child.terminal_is_empty(){return self.child.close_demands(copy)}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.fields.is_some(){return self.retirement.next_owner_demands::<Fields>(true,&None,copy)}Ok(fixed(size_of::<Option<Binding>>()))}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<Close,OriginalTurnFailure>{
  if !self.closing||self.binding.is_some_and(|binding|!binding.source.matches(cx)){return Err(invalid().into())}

  if !self.pipeline.terminal_is_empty(){return self.pipeline.close_step(cx).map(|step|match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}
  if !self.source.terminal_is_empty(){return self.source.close_step(cx).map(|step|match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}
  let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_with_effect(cx,d,|g,effect|{if !self.child.terminal_is_empty(){return self.child.close_step(g,effect)}if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}if self.fields.is_some(){return begin_close_owner(&mut self.retirement,&mut self.fields,effect)}if self.binding.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(progress(d.copy_bytes))),work:1}})?;Ok(if self.terminal_is_empty(){Close::Complete{progress:step.progress()}}else{Close::Pending{progress:step.progress()}})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.pipeline.terminal_is_empty()&&self.source.terminal_is_empty()&&self.child.terminal_is_empty()&&self.fields.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl Drop for FlowEvaluationCommandWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original Flow evaluation must close its whole source, result and publication family");unsafe{ManuallyDrop::drop(&mut self.fields)}}}

#[path="🔎️origin.rs"]
mod origin;
pub use origin::{FlowOriginLookupWork,FlowOriginLookupOutcome};

#[path="📤️job.rs"]
mod job;
pub use job::FlowEvaluationJobWork;

