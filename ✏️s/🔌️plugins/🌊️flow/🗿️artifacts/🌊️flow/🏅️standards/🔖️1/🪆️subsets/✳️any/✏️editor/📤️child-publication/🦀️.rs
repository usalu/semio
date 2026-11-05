//! 📨️ The parent command and genuine child read authorize one exact original native leaf batch.
use semio_plugin::{ChildEmitPreparation,ChildContentView,AppOperationContext,app::{MemberKeyRef,child_read::{ChildReadCursor,ChildReadStamp,ChildReadSourceCustody,OriginalChildMemberLookup},child_read::command::{OriginalChildEditorCommandCursor,OriginalChildEditorCommandAuthority}}};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetireOwned,retained_clone::{RetainedCloneRef,retained_clone_grant_admits},retirement::controlled::ControlledRetirement};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_s_artifact_stdio_semio_flow::{schema::{snapshot::SemioFlowSnapshot,mutations::SemioFlowMutation},editor::child_editor::{SEMIO_FLOW_CHILD_READ,SEMIO_FLOW_CHILD_APPLY_MUTATIONS,semio_flow_mutation_batch_demand,admit_semio_flow_mutation_batch}};
use std::mem::size_of;
#[path="🧷️reference/🦀️.rs"]mod reference;
use reference::FLOW_CONTENT_REFERENCE_LENDER;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Flow child publication source changed")}
fn add(a:RetirementDemand,b:RetirementDemand)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:a.copy_bytes.checked_add(b.copy_bytes).ok_or_else(invalid)?,capacity_bytes:a.capacity_bytes.checked_add(b.capacity_bytes).ok_or_else(invalid)?,release_bytes:a.release_bytes.checked_add(b.release_bytes).ok_or_else(invalid)?,depth:a.depth.max(b.depth)})}
#[derive(semio_framework_value::RetireOwned)]
struct Owners{lookup:Option<OriginalChildMemberLookup>,read_cursor:Option<ChildReadCursor>,child:ChildReadSourceCustody<SemioFlowSnapshot>,qualifier:Option<OriginalChildEditorCommandCursor>,authority:Option<OriginalChildEditorCommandAuthority>,preparation:Option<ChildEmitPreparation>}
/// 📤️ The surrounding job retains its original parent and command sources until this custody is closed.
type References=((u64,usize,usize,usize),(u64,usize,usize,usize));
pub(super) struct FlowChildPublicationWork{references:Option<References>,owners:ControlledRetirement<Owners>,stamp:Option<ChildReadStamp>,phase:u8,closing:bool,origin:Option<(u64,u64)>}
impl FlowChildPublicationWork{
 pub(super) fn new()->Self{Self{references:None,owners:ControlledRetirement::retain(Owners{lookup:None,read_cursor:None,child:ChildReadSourceCustody::new(),qualifier:None,authority:None,preparation:None}),stamp:None,phase:0,closing:false,origin:None}}
 pub(super) fn read_step<C:Sync>(&mut self,parent:RetainedCloneRef<'_,crate::FlowSnapshot>,command:RetainedCloneRef<'_,C>,children:&ChildContentView,operation:&AppOperationContext,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let mut leaves=None;for _ in 0..512{if self.owners.original().ok_or_else(invalid)?.child.ready(){return Ok(true)}let before=(cx.fuel_remaining(),cx.retained_progress());if self.once(parent,command,children,operation,&mut leaves,cx)?.is_some(){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled()||before==(cx.fuel_remaining(),cx.retained_progress()){break}}Ok(self.owners.original().ok_or_else(invalid)?.child.ready())
 }
 pub(super) fn child_source(&self)->Result<RetainedCloneRef<'_,SemioFlowSnapshot>,ValueError>{self.owners.original().ok_or_else(invalid)?.child.source()}
 pub(super) fn step<C:Sync>(&mut self,parent:RetainedCloneRef<'_,crate::FlowSnapshot>,command:RetainedCloneRef<'_,C>,children:&ChildContentView,operation:&AppOperationContext,leaves:&mut Option<Vec<SemioFlowMutation>>,cx:&mut StepContext<'_>)->Result<Option<ChildEmitPreparation>,OriginalTurnFailure>{
  for _ in 0..512{let before=(cx.fuel_remaining(),cx.retained_progress());if let Some(output)=self.once(parent,command,children,operation,leaves,cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.fuel_remaining(),cx.retained_progress()){break}}Ok(None)
 }
 fn once<C:Sync>(&mut self,parent:RetainedCloneRef<'_,crate::FlowSnapshot>,command:RetainedCloneRef<'_,C>,children:&ChildContentView,operation:&AppOperationContext,leaves:&mut Option<Vec<SemioFlowMutation>>,cx:&mut StepContext<'_>)->Result<Option<ChildEmitPreparation>,OriginalTurnFailure>{
  let origin=(cx.operation().0,cx.generation().0);let references=(parent.reference_identity(),command.reference_identity());if self.references.is_some_and(|r|r!=references)||self.closing||self.origin.is_some_and(|o|o!=origin)||origin!=(operation.operation_id,operation.generation){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  let owners=self.owners.original_mut().ok_or_else(invalid)?;if let Some(lookup)=owners.lookup.as_ref(){lookup.next_demand::<SemioFlowSnapshot,_>(parent,children,MemberKeyRef::root("content",&parent.get().content.child_id),operation)?;}
  if self.origin.is_none(){let d=RetirementDemand{copy_bytes:size_of::<Option<(u64,u64)>>()+size_of::<Option<References>>(),depth:1,..Default::default()};if !cx.retained_work_can_enter(d){return Ok(None)}self.origin=Some(origin);self.references=Some(references);cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;return Ok(None)}
  if owners.preparation.is_some(){let d=RetirementDemand{copy_bytes:2*size_of::<ChildEmitPreparation>(),depth:1,..Default::default()};if !cx.retained_work_can_enter(d){return Ok(None)}cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;self.phase=8;return Ok(owners.preparation.take())}
  if self.phase==8{return Err(invalid().into())}
  if owners.lookup.is_none(){let d=OriginalChildMemberLookup::birth_demand();if !cx.retained_work_can_enter(d){return Ok(None)}let(value,p)=OriginalChildMemberLookup::admit(cx.retained_grant())?;owners.lookup=Some(value);cx.consume_fuel(1);cx.consume_retained(p)?;return Ok(None)}
  if owners.read_cursor.is_none(){let d=ChildReadCursor::birth_demand();if !cx.retained_work_can_enter(d){return Ok(None)}let(value,p)=ChildReadCursor::admit(cx.retained_grant())?;owners.read_cursor=Some(value);cx.consume_fuel(1);cx.consume_retained(p)?;return Ok(None)}
  if owners.child.ready()&&owners.qualifier.is_none(){let d=OriginalChildEditorCommandCursor::birth_demand();if !cx.retained_work_can_enter(d){return Ok(None)}let(value,p)=OriginalChildEditorCommandCursor::admit(cx.retained_grant())?;owners.qualifier=Some(value);cx.consume_fuel(1);cx.consume_retained(p)?;return Ok(None)}
  if owners.authority.is_some(){let d=semio_flow_mutation_batch_demand("content".len(),parent.get().content.child_id.len())?;if !cx.retained_work_can_enter(d){return Ok(None)}let result=admit_semio_flow_mutation_batch("content",&parent.get().content.child_id,leaves,&mut owners.authority,cx.retained_grant());cx.consume_fuel(1);match result{Ok(Some((preparation,p)))=>{owners.preparation=Some(preparation);cx.consume_retained(p)?},Ok(None)=>{},Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))}return Ok(None)}
  let key=MemberKeyRef::root("content",&parent.get().content.child_id);
  if !owners.lookup.as_ref().unwrap().ready(){let lookup=owners.lookup.as_mut().unwrap();let d=lookup.demand::<SemioFlowSnapshot>();if !cx.retained_work_can_enter(d){return Ok(None)}let result=SEMIO_FLOW_CHILD_READ.read_original(parent,children,key,operation,lookup,cx.retained_grant());cx.consume_fuel(1);match result{Ok(advance)=>cx.consume_retained(advance.progress)?,Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))}return Ok(None)}
  let raw=semio_plugin::app::child_read::erase_child_admission_source(parent,0x464c4f57);
  let quote=owners.lookup.as_ref().unwrap().quote_ready_receiving::<SemioFlowSnapshot,_>(parent,children,key,operation,|read|{
   if self.stamp.is_none(){owners.read_cursor.as_ref().unwrap().next_demand(raw,&FLOW_CONTENT_REFERENCE_LENDER,0,read,&SEMIO_FLOW_CHILD_READ.identity)}else if !owners.child.ready(){owners.child.next_admit_demand()}else if self.phase==6{owners.qualifier.as_ref().unwrap().take_authority_demand()}else{owners.qualifier.as_ref().unwrap().next_demand()}
  })?;if !cx.retained_work_can_enter(quote){return Ok(None)}
  let metadata=owners.lookup.as_ref().unwrap().borrow_ready_demand::<SemioFlowSnapshot>();let turn=owners.lookup.as_ref().unwrap().borrow_ready::<SemioFlowSnapshot,_>(parent,children,key,operation,cx.retained_grant());let borrow_work=turn.work;let borrowed=match turn.result{Ok(read)=>read,Err(error)=>{cx.consume_fuel(borrow_work);return Err(OriginalTurnFailure::debit_original(error,cx))}};let read=borrowed.read.ok_or_else(invalid)?;let g=RetainedCloneGrant{maximum_copy_bytes:cx.retained_grant().maximum_copy_bytes-metadata.copy_bytes,..cx.retained_grant()};
  let(result,work)=if self.stamp.is_none(){
   let cursor=owners.read_cursor.as_mut().unwrap();let result=SEMIO_FLOW_CHILD_READ.qualify(raw,&FLOW_CONTENT_REFERENCE_LENDER,0,&read,cursor,g);let work=cursor.last_turn_work();(result.map(|advance|{self.stamp=advance.stamp;advance.progress}),work)
  }else if !owners.child.ready(){
   let result=SEMIO_FLOW_CHILD_READ.admit_original_source(&read,self.stamp.as_ref().unwrap(),&mut owners.child,g);let work=owners.child.last_turn_work();(result.map(|step|step.progress()),work)
  }else if self.phase==6{
   let cursor=owners.qualifier.as_mut().unwrap();let turn=cursor.take_authority(&owners.child,&read,g);let work=turn.work;(turn.result.map(|advance|{if let Some(authority)=advance.authority{owners.authority=Some(authority)}advance.progress}),work)
  }else{
   let cursor=owners.qualifier.as_mut().unwrap();let result=SEMIO_FLOW_CHILD_APPLY_MUTATIONS.qualify_original(parent,command,self.stamp.as_ref().unwrap(),&read,cursor,g);let work=cursor.last_turn_work();(result.map(|advance|{if advance.stamp.is_some(){self.phase=6}advance.progress}),work)
  };
  cx.consume_fuel(borrow_work.max(work));let merge=|p:RetainedCloneProgress|RetainedCloneProgress{copied_items:borrowed.progress.copied_items.max(p.copied_items),copied_bytes:borrowed.progress.copied_bytes+p.copied_bytes,retained_capacity_bytes:borrowed.progress.retained_capacity_bytes+p.retained_capacity_bytes,released_bytes:borrowed.progress.released_bytes+p.released_bytes};
  match result{Ok(p)=>cx.consume_retained(merge(p))?,Err(error)=>{let p=merge(error.retained_progress());return Err(OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))}}Ok(None)

 }
 pub(super) fn begin_close(&mut self){self.closing=true}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if !self.owners.terminal_is_empty(){self.owners.next_demand(copy)}else{Ok(RetirementDemand{copy_bytes:size_of::<Option<ChildReadStamp>>()+size_of::<Option<(u64,u64)>>()+size_of::<Option<References>>(),depth:1,..Default::default()})}}
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if !self.owners.terminal_is_empty(){let result=self.owners.step(g);return RetainedWorkOutcome{result,work:self.owners.last_turn_work()}}let d=RetirementDemand{copy_bytes:size_of::<Option<ChildReadStamp>>()+size_of::<Option<(u64,u64)>>()+size_of::<Option<References>>(),depth:1,..Default::default()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.stamp=None;self.origin=None;self.references=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}}
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.owners.terminal_is_empty()&&self.stamp.is_none()&&self.origin.is_none()&&self.references.is_none()}
}
impl Drop for FlowChildPublicationWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original Flow child publication requires terminal original source and native leaf custody")}}

#[path="🔤️param-json/🦀️.rs"]
mod param_json;
#[path="🧩️node/🦀️.rs"]
mod node;

#[path="🍃️leaves/🦀️.rs"]
mod leaves;
pub(crate) use leaves::{FlowLeafWork,FlowLeafMode};
