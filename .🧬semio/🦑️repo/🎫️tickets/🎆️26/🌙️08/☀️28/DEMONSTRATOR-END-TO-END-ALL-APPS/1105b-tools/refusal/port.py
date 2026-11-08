import sys
F="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/⚠️refusal/🦀️.rs"
L=open(F,encoding="utf-8").read().split("\n")
def idx(prefix,start=0):
    for i in range(start,len(L)):
        if L[i].startswith(prefix): return i
    sys.exit("missing "+prefix)
a=idx("fn advance(&mut self,input:crate::retained_clone::RetainedCloneRef<'_,I>,control")
b=idx(" }",a)
assert b-a==14,(a,b)
SEL='''fn select_advance(&self,input:crate::retained_clone::RetainedCloneRef<'_,I>,_body:usize,maximum_work_units:u64)->Result<semio_framework_value_native_receiver_protocol::NativeAdvanceSelection,ValueError>{
  use crate::owned::tree::invalid;use semio_framework_value_native_receiver_protocol::NativeAdvanceSelection as Selection;
  if self.closing||self.phase>=6{return Err(invalid())}let bound=input.check_binding(&self.binding)?;
  let work=match self.phase{0=>if bound{1}else{input.binding_work_demand(&self.binding)?},1=>if self.fields.ready(){self.fields.take_work_demand()?}else{return Ok(Selection::Nested)},2..=5=>return Ok(Selection::Nested),_=>return Err(invalid())};
  if work>maximum_work_units{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retirement receiver action exceeds its original work ceiling"))}Ok(Selection::Local{work})
 }
fn advance_nested(&mut self,input:crate::retained_clone::RetainedCloneRef<'_,I>,control:&mut crate::NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:crate::RetainedCloneGrant)->crate::RetainedWorkPoll<crate::RetainedCloneStep,crate::OriginalReceivingPrimary>{
  use crate::RetainedWorkPoll::{Complete,Pending};use crate::owned::tree::{fixed,receipt,child_grant,invalid};use crate::retained_clone::retained_clone_grant_admits;use semio_framework_value_retirement::OriginalRetirementControl;
  let fail=|error:ValueError|->crate::RetainedWorkPoll<crate::RetainedCloneStep,crate::OriginalReceivingPrimary>{Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if self.closing||self.phase>=6{return fail(invalid())}
  let d=match self.demands(input,body,maximum_work_units){Ok(d)=>d,Err(error)=>return fail(error)};
  if !retained_clone_grant_admits(g,d){return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})}
  if let Err(error)=control.checkpoint(){return fail(error)}
  if let Some((ledger,recipient))=self.identity{if control.ledger_identity()!=ledger||control.retirement_recipient_identity()!=recipient{return fail(ValueError::invariant("retirement receiver original recipient changed"))}}
  match self.phase{
   1=>{if self.fields.ready(){return fail(invalid())}self.fields.advance(input,control,body,maximum_work_units,child_grant(g,0)).map(|step|crate::RetainedCloneStep::Progress(step.progress()))}
   3=>{let saved=*self.birth.get_or_insert(crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:crate::OriginalFailureStorage::birth_bytes(),maximum_release_bytes:0,maximum_depth:1});if saved.restricted_by(g)!=saved{return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})}
    match control.prepare_failure_storage(saved){
     Pending{work}=>Pending{work},
     Complete(turn)=>{let settled=!control.failure_storage_birth_pending();if settled{self.birth=None;if turn.result.is_ok()&&control.has_retirement_failure_storage(){self.phase=4}}Complete(crate::RetainedWorkOutcome{result:turn.result.map(|step|crate::RetainedCloneStep::Progress(step.progress())).map_err(Into::into),work:turn.work})}
    }
   }
   2|4|5=>{
    let result=(||->Result<crate::RetainedCloneStep,crate::OriginalReceivingPrimary>{match self.phase{
     2=>{let needs=self.pending.as_ref().ok_or_else(||ValueError::invariant("retirement node fields absent"))?.child.is_some();if needs{let recipient=control.retirement_recipient_identity().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retirement child requires its original funding recipient"))?;self.identity=Some((control.ledger_identity(),Some(recipient)));self.phase=if control.has_retirement_failure_storage(){4}else{3}}else{self.phase=5}Ok(crate::RetainedCloneStep::Progress(receipt(fixed(0))))},
     4=>{let pool_grant=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_depth:1,..Default::default()};let storage=control.with_retirement::<_,ValueError>(pool_grant,|loan|{if !loan.failure_storage_birth().terminal_is_empty(){return Err(ValueError::invariant("retirement failure birth remains unsettled"))}Ok(loan.failure_storage_pool().take())})?;let Some(storage)=storage else{return Err(ValueError::invariant("original prepaid retirement node changed").into())};*self.storage=Some(storage);self.phase=5;Ok(crate::RetainedCloneStep::Progress(receipt(d)))},
     5=>{if self.pending.as_ref().is_some_and(|fields|fields.child.is_some())&&self.storage.is_none(){return Err(ValueError::invariant("prepaid retirement node absent").into())}let fields=self.pending.take().ok_or_else(||ValueError::invariant("retirement fields absent"))?;let NativeRetirementNodeFields{primary,work_refusal,receipt_refusal,allocation_refusal,progress_override,child}=fields;let value=match child{None=>crate::OriginalRetirementFailure::from_owned_parts(primary,work_refusal,receipt_refusal,allocation_refusal,None,progress_override),Some(child)=>{let storage=self.storage.take().ok_or_else(||ValueError::invariant("validated prepaid retirement node absent"))?;let value=storage.attach_owned_parts(child,primary,work_refusal,receipt_refusal,allocation_refusal);match progress_override{Some(progress)=>value.with_retained_progress(progress),None=>value}}};*self.output=Some(value);self.phase=6;Ok(crate::RetainedCloneStep::Complete(receipt(d)))},
     _=>Err(invalid().into())}})();
    let work=u128::from(result.is_ok());Complete(crate::RetainedWorkOutcome{result,work})
   }
   _=>fail(invalid())
  }
 }
fn advance_local<'reply>(&mut self,input:crate::retained_clone::RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_receiver_protocol::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<semio_framework_value_native_receiver_protocol::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,ValueError>{
  use crate::owned::tree::{fixed,receipt,invalid};use crate::retained_clone::retained_clone_grant_admits;
  let g=effect.grant();let maximum_work_units=effect.maximum_work_units();
  let result:Result<Option<crate::RetainedCloneStep>,crate::OriginalReceivingPrimary>=(||{
   let d=self.demands(input,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(Some(crate::RetainedCloneStep::Progress(Default::default())))}
   match self.phase{
    0=>{if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(crate::RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retirement receiver action exceeded its original work").into())}self.phase=1;Ok(Some(crate::RetainedCloneStep::Progress(receipt(fixed(0)))))},
    1=>match self.fields.take(&mut *self.pending,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.pending.is_none(){return Ok(Some(crate::RetainedCloneStep::Progress(Default::default())))}self.phase=2;Ok(Some(crate::RetainedCloneStep::Progress(step.progress())))}},
    _=>Err(invalid().into())
   }
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }'''.split("\n")
L[a:b+1]=SEL
open(F,"w",encoding="utf-8").write("\n".join(L))
