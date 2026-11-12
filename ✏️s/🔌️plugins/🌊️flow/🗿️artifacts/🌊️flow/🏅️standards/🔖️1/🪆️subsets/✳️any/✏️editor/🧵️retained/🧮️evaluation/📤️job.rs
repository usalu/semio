//! 📤️ Evaluate and Tick retain real sources through typed invocation and session handoff.
use super::*;
use super::super::FlowOriginalIntake;
use super::super::super::FlowMutation;
use semio_plugin::{Emit,NoConfigMutation,ExtensionInvocation,retained_command::{ArtifactCommandWork,ArtifactCommandInputs,ArtifactCommandWorkStep},ArtifactInstanceOperationOwnerAliasRetirement};
use flow::invocation_publication::{FlowInvocationDispatch};
use std::collections::VecDeque;
type Output=Emit<FlowMutation,NoConfigMutation>;
#[derive(RetireOwned)]
#[value(send_only)]
struct Outputs{emit:Option<Output>,pending:VecDeque<flow::neural::PendingExtensionEval>,invocation:Option<FlowInvocationDispatch>}
fn session<R>(owner:&ArtifactInstanceOperationOwnerHandle,body:impl FnOnce(&mut flow::FlowEvalSession)->Result<R,OriginalTurnFailure>)->Result<Option<R>,OriginalTurnFailure>{owner.with_original_mut::<FlowInstanceOperationOwner,_>(|instance|instance.with_original_session(body))}
fn pending<A:FlowApp>()->ArtifactCommandWorkStep<EditorApp<A>>{ArtifactCommandWorkStep::Pending{stage:"flow-evaluation-original",preview:br#"{"en":"Evaluating graph","de":"Graph wird ausgewertet"}"#}}
fn window(command:&FlowCommand)->Result<(&str,&str),ValueError>{match command{FlowCommand::Evaluate(_)=>Ok((crate::editor::flow::editor_vocabulary::FLOW_PLAY_WINDOW_MAIN,crate::editor::flow::editor_vocabulary::FLOW_PLAY_WINDOW_MAIN)),FlowCommand::FlowEvalTick(command)=>Ok((&command.window_id,&command.window_kind_id)),_=>Err(invalid())}}
pub struct FlowEvaluationJobWork{tool:&'static str,owner:Option<ArtifactInstanceOperationOwnerHandle>,owner_close:Option<ArtifactInstanceOperationOwnerAliasRetirement>,intake:FlowOriginalIntake,birth:Option<flow::FlowEvaluationWork>,pipeline:Option<FlowEvaluationCommandWork>,key_work:flow::FlowEvalWindowKeyWork,key:Option<flow::FlowEvalWindowKey>,pending_dispatch:Option<flow::FlowPendingDispatchWork>,fields:ManuallyDrop<Option<Outputs>>,retirement:RetainedCloneClose,phase:u8,count:usize,unfinished:bool,completed:bool,closing:bool}
impl FlowEvaluationJobWork{
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.pending_dispatch.is_none()&&self.pipeline.is_none()&&self.birth.is_none()&&self.key_work.terminal_is_empty()&&self.retirement.is_empty()&&self.fields.is_none()&&self.intake.terminal_is_empty()&&self.owner.is_none()&&self.owner_close.is_none()}
 pub fn new(tool:&'static str,owner:ArtifactInstanceOperationOwnerHandle)->Self{Self{tool,owner:Some(owner),owner_close:None,intake:FlowOriginalIntake::new(tool),birth:None,pipeline:None,key_work:flow::FlowEvalWindowKeyWork::new(),key:None,pending_dispatch:None,fields:ManuallyDrop::new(Some(Outputs{emit:Some(Output::default()),pending:VecDeque::new(),invocation:None})),retirement:Default::default(),phase:0,count:0,unfinished:false,completed:false,closing:false}}
 fn owner(&self)->Result<&ArtifactInstanceOperationOwnerHandle,ValueError>{self.owner.as_ref().ok_or_else(invalid)}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{FlowEvaluationCommandWork::paid(cx,bytes)}
 fn paid_close(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{FlowEvaluationCommandWork::paid_close(cx,bytes)}
 fn once<A:FlowApp>(&mut self,input:&ArtifactCommandInputs<'_,EditorApp<A>>,cx:&mut StepContext<'_>)->Result<Option<Output>,OriginalTurnFailure>{
  if self.completed||self.closing||!self.intake.source_matches(cx){return Err(invalid().into())}if cx.is_cancelled()||cx.should_yield(){return Ok(None)}let context=input.context.ok_or_else(invalid)?;
  match self.phase{
   0=>if self.intake.receive_parent(input,cx)?{self.phase=1},
   1=>{let owner=self.owner.as_ref().ok_or_else(invalid)?;let intake=&self.intake;let birth=&mut self.birth;if birth.is_none(){session(owner,|same|intake.receive_evaluation_work(same,birth,0,cx))?;return Ok(None)}if Self::paid(cx,size_of::<flow::FlowEvaluationWork>()+size_of::<FlowEvaluationCommandWork>())?{self.pipeline=Some(FlowEvaluationCommandWork::new(self.birth.take().ok_or_else(invalid)?));self.phase=2}},
   2=>{let(parent,command,infos)=self.intake.references()?;let pipeline=self.pipeline.as_mut().ok_or_else(invalid)?;if let Some(output)=pipeline.step(parent,command,context,input.operation,self.owner.as_ref().ok_or_else(invalid)?,infos,cx)?{self.count=output.pending.len();self.unfinished=output.unfinished;self.fields.as_mut().ok_or_else(invalid)?.pending=VecDeque::from(output.pending);self.phase=3}},
   3=>{let(_,command,_)=self.intake.references()?;if let Some(key)=self.key_work.step(window(command.get())?.0,cx)?{self.key=Some(key);self.phase=4}},
   4=>{let key=self.key.ok_or_else(invalid)?;if session(self.owner()?,|same|{let d=RetirementDemand{copy_bytes:same.next_window_tick_key_copy_byte_demand(key)?,capacity_bytes:same.next_window_tick_key_capacity_byte_demand(key,cx.retained_grant().maximum_copy_bytes)?,release_bytes:same.next_window_tick_key_release_byte_demand(key)?,depth:same.next_window_tick_key_depth_demand(key)?.max(1)};if !cx.retained_work_can_enter(d){return Ok(false)}let turn=same.prepare_window_tick_key_original(key,cx.retained_grant());let ready=turn.result.as_ref().is_ok_and(|step|matches!(step,RetainedCloneStep::Complete(_)));semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(ready)})?.unwrap_or(false){self.phase=5}},
   5=>{let fields=self.fields.as_mut().ok_or_else(invalid)?;let emit=fields.emit.as_mut().ok_or_else(invalid)?;if self.count>emit.extension_invocations.capacity(){let bytes=self.count.checked_mul(size_of::<ExtensionInvocation>()).ok_or_else(invalid)?;let d=RetirementDemand{capacity_bytes:bytes,depth:1,..Default::default()};if !cx.retained_work_can_enter(d){return Ok(None)}assert!(emit.extension_invocations.is_empty()&&emit.extension_invocations.capacity()==0);emit.extension_invocations.try_reserve_exact(self.count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original extension invocation output reservation refused"))?;let p=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:emit.extension_invocations.capacity()*size_of::<ExtensionInvocation>(),..Default::default()};cx.consume_fuel(1);cx.consume_retained(p)?;}else if Self::paid(cx,1)?{self.phase=6}},
   6=>{
    if self.pending_dispatch.is_none(){
     if Self::paid(cx,size_of::<flow::FlowPendingDispatchWork>()+2*size_of::<VecDeque<flow::neural::PendingExtensionEval>>())?{
      let pending=std::mem::take(&mut self.fields.as_mut().ok_or_else(invalid)?.pending);
      self.pending_dispatch=Some(flow::FlowPendingDispatchWork::new(pending,self.intake.registry_generation()?,cx));
     }return Ok(None)
    }
    let(parent,command,_)=self.intake.references()?;let source=self.pipeline.as_ref().ok_or_else(invalid)?.source(parent.get())?;let(window,kind)=window(command.get())?;
    let pending=self.pending_dispatch.as_mut().ok_or_else(invalid)?;let owner=self.owner.as_ref().ok_or_else(invalid)?;
    if let Some(Some(output))=session(owner,|same|pending.step(&source,same,window,kind,cx))?{
     match output{flow::FlowPendingDispatchStep::Invocation(invocation)=>{self.fields.as_mut().ok_or_else(invalid)?.invocation=Some(invocation);self.phase=62},flow::FlowPendingDispatchStep::Complete=>self.phase=16}
    }
   },
   62=>if Self::paid(cx,2*size_of::<ExtensionInvocation>())?{
    let fields=self.fields.as_mut().ok_or_else(invalid)?;let FlowInvocationDispatch{extension_id,capability,request_json,response_action}=fields.invocation.take().ok_or_else(invalid)?;
    fields.emit.as_mut().ok_or_else(invalid)?.extension_invocations.push(ExtensionInvocation{extension_id,capability,request_json,response_action});self.phase=6;
   },
   16=>{let owner=self.owner.as_ref().ok_or_else(invalid)?;let key=self.key.ok_or_else(invalid)?;let count=self.count;let unfinished=self.unfinished;if session(owner,|same|{let own=2*size_of::<Output>();let mut d=flow::FlowEvalSession::window_evaluation_publication_demands();d.copy_bytes=d.copy_bytes.checked_add(own).ok_or_else(invalid)?;if !cx.retained_work_can_enter(d){return Ok(false)}let g=RetainedCloneGrant{maximum_copy_bytes:cx.retained_grant().maximum_copy_bytes-own,..cx.retained_grant()};let turn=same.publish_window_evaluation_key_original(key,count,unfinished,g);let ready=turn.result.as_ref().is_ok_and(|s|matches!(s,RetainedCloneStep::Complete(_)));let result=turn.result.and_then(|step|{let mut p=step.progress();if p.copied_items!=0{p.copied_bytes=p.copied_bytes.checked_add(own).ok_or_else(invalid)?}Ok(match step{RetainedCloneStep::Progress(_)=>RetainedCloneStep::Progress(p),RetainedCloneStep::Complete(_)=>RetainedCloneStep::Complete(p)})});semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;Ok(ready)})?.unwrap_or(false){let mut emit=self.fields.as_mut().ok_or_else(invalid)?.emit.take().ok_or_else(invalid)?;emit.ui_scope=semio_framework::kernel::UiDirtyScope::Full;self.completed=true;self.phase=17;return Ok(Some(emit))}},
   _=>return Err(invalid().into())
  }Ok(None)
 }
 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(pending)=self.pending_dispatch.as_ref(){return if pending.terminal_is_empty(){Ok(fixed(size_of::<flow::FlowPendingDispatchWork>()))}else{pending.close_demands(copy)}}if let Some(pipeline)=self.pipeline.as_ref(){return if pipeline.terminal_is_empty(){Ok(fixed(size_of::<FlowEvaluationCommandWork>()))}else{pipeline.close_demands(copy)}}if let Some(birth)=self.birth.as_ref(){return if birth.terminal_is_empty(){Ok(fixed(size_of::<flow::FlowEvaluationWork>()))}else{birth.next_close_demands(copy)}}if !self.key_work.terminal_is_empty(){return Ok(self.key_work.close_demands())}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.fields.is_some(){return self.retirement.next_owner_demands::<Outputs>(true,&None,copy)}if !self.intake.terminal_is_empty(){return self.intake.close_demands(copy)}if let Some(close)=self.owner_close.as_ref(){return close.demands(copy)}Ok(if self.owner.is_some(){ArtifactInstanceOperationOwnerAliasRetirement::constructor_demands()}else{Default::default()})
 }
}
impl<A:FlowApp> ArtifactCommandWork<EditorApp<A>> for FlowEvaluationJobWork{
 fn tool_id(&self)->&'static str{self.tool}
 fn receive_original_command(&mut self,command:&mut FlowCommand,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.intake.receive_command(command,cx)}
 fn receive_original_context(&mut self,_context:&semio_plugin::app::ArtifactOwnedContextHandle<EditorApp<A>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.intake.receive_catalogue(self.owner.as_ref().ok_or_else(invalid)?,cx)}
 fn extent(&self,command:&FlowCommand,_snapshot:&FlowSnapshot,_interaction:&protocol::InteractionState,context:Option<&ArtifactOwnedToolJobContext<EditorApp<A>>>)->Option<usize>{(!self.closing&&!self.completed&&command.command_id()==self.tool&&matches!(self.tool,"evaluate"|"flowEvalTick")&&context.is_some_and(|context|context.provisional().is_empty())).then_some(1)}
 fn step(&mut self,input:&ArtifactCommandInputs<'_,EditorApp<A>>,cx:&mut StepContext<'_>)->Result<ArtifactCommandWorkStep<EditorApp<A>>,OriginalTurnFailure>{for _ in 0..512{let before=(cx.fuel_remaining(),cx.retained_progress());if let Some(output)=self.once(input,cx)?{return Ok(ArtifactCommandWorkStep::Complete(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.fuel_remaining(),cx.retained_progress()){break}}Ok(pending())}
 fn begin_close(&mut self){self.closing=true;self.intake.begin_close();self.key_work.begin_close();if let Some(work)=self.pending_dispatch.as_mut(){work.begin_close()}if let Some(work)=self.pipeline.as_mut(){work.begin_close()}if let Some(work)=self.birth.as_mut(){work.begin_close()}}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<Close,OriginalTurnFailure>{
  if !self.closing||!self.intake.source_matches(cx){return Err(invalid().into())}
  if let Some(pending)=self.pending_dispatch.as_mut(){
   if !pending.terminal_is_empty(){return pending.close_step(cx).map(|step|match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}
   if !Self::paid_close(cx,size_of::<flow::FlowPendingDispatchWork>())?{return Ok(Close::Blocked)}
   self.pending_dispatch=None;return Ok(Close::Pending{progress:progress(size_of::<flow::FlowPendingDispatchWork>())})
  }
  if let Some(pipeline)=self.pipeline.as_mut(){if !pipeline.terminal_is_empty(){return pipeline.close_step(cx).map(|step|match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}if !Self::paid_close(cx,size_of::<FlowEvaluationCommandWork>())?{return Ok(Close::Blocked)}self.pipeline=None;return Ok(Close::Pending{progress:progress(size_of::<FlowEvaluationCommandWork>())})}
  if let Some(birth)=self.birth.as_mut(){if birth.terminal_is_empty(){if !Self::paid_close(cx,size_of::<flow::FlowEvaluationWork>())?{return Ok(Close::Blocked)}self.birth=None;return Ok(Close::Pending{progress:progress(size_of::<flow::FlowEvaluationWork>())})}let d=birth.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_with_effect(cx,d,|g,effect|poll_outcome(birth.close_step(g.maximum_copy_bytes,effect)))?;return Ok(Close::Pending{progress:step.progress()})}
  if !self.key_work.terminal_is_empty(){return self.key_work.close_step(cx).map(|step|match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}
  let d=self.demands(cx.retained_grant().maximum_copy_bytes)?;
  if !self.retirement.is_empty()||self.fields.is_some(){let step=close_with_effect(cx,d,|g,effect|{if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}begin_close_owner(&mut self.retirement,&mut self.fields,effect)})?;return Ok(Close::Pending{progress:step.progress()})}
  if !self.intake.terminal_is_empty(){let step=close_with_effect(cx,d,|g,effect|self.intake.close_step(g,effect))?;return Ok(Close::Pending{progress:step.progress()})}
  if self.owner.is_some(){if !cx.retained_work_can_enter(d)||cx.fuel_remaining()==0{return Ok(Close::Blocked)}self.owner_close=Some(ArtifactInstanceOperationOwnerAliasRetirement::new(self.owner.take().ok_or_else(invalid)?));cx.consume_fuel(1);cx.consume_retained(progress(d.copy_bytes))?;return Ok(Close::Pending{progress:progress(d.copy_bytes)})}
  if let Some(close)=self.owner_close.as_mut(){let g=cx.retained_grant();let step=close.step(cx,g)?;if close.terminal_is_empty(){self.owner_close=None}return Ok(if self.terminal_is_empty(){Close::Complete{progress:step.progress()}}else{Close::Pending{progress:step.progress()}})}Ok(Close::Complete{progress:Default::default()})
 }
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_frame_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
 fn terminal_is_empty(&self)->bool{FlowEvaluationJobWork::terminal_is_empty(self)}
}
impl Drop for FlowEvaluationJobWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original Flow evaluation job must close every original source and output");unsafe{ManuallyDrop::drop(&mut self.fields)}}}
