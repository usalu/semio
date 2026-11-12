//! 🏗️ Exact AOT declarations and seed rows become native immutable sources through one original decoder.
use semio_framework_replication_mutation::{CompiledMutationInputDefinitions,CompiledMutationInputError};
use semio_framework::{ActionArgDef,ActionArgShapeSeed,JsonNativeInputCursor};
use semio_framework_pack_json::JsonReadLimits;
use semio_framework_value::{NativeDecodeControl,OriginalRetirementControl,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{NativeAllocationIdentity,RetainedCloneClose,RetainedCloneSource,StagedSourceAdmission,retained_clone_grant_admits};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value_native_owned::NativeReceive;
use semio_framework_value_native_owned::native_decoding::NativeReceivingFailure;
use semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource;
use super::funding::{pay,run};
use std::mem::{ManuallyDrop,size_of};
type Cache<T>=Vec<T>;
type Source<T>=RetainedCloneSource<Cache<T>>;
type Admission<T>=StagedSourceAdmission<Cache<T>,Cache<T>>;
type Cursor<T>=JsonNativeInputCursor<'static,Cache<T>>;
#[derive(Clone,Copy)]
enum Phase{Birth,Read,Take,SourceBirth,Source,Ready}
/// 📚️ The exact compiled record stays attached after both original source admissions.
pub struct NormInspectorPreparedStaticSource<T:NativeReceive<dyn NativeValueSource>+Sync+'static>{
 source:Source<T>,compiled:&'static CompiledMutationInputDefinitions,
}
impl<T:NativeReceive<dyn NativeValueSource>+Sync+'static> NormInspectorPreparedStaticSource<T>{
 pub fn borrow(&self)->semio_framework_value::retained_clone::RetainedCloneRef<'_,Vec<T>>{self.source.borrow()}
 pub fn compiled(&self)->&'static CompiledMutationInputDefinitions{self.compiled}
 pub fn same_compiled_source<U:NativeReceive<dyn NativeValueSource>+Sync+'static>(&self,other:&NormInspectorPreparedStaticSource<U>)->bool{
  let(a,b)=(self.compiled,other.compiled);
  a.provenance==b.provenance&&a.semantic_kind==b.semantic_kind&&a.schema_sha256==b.schema_sha256&&a.document_sha256==b.document_sha256&&a.framework_document_sha256==b.framework_document_sha256
 }
 pub fn next_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.source.next_close_demand(body)}
 pub fn next_close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.source.next_close_work_demand(body)}
 pub fn close_poll(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.source.close_step(body,effect)}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{super::funding::step(self.close_poll(body,effect))}
 pub fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()}
}
impl<T:NativeReceive<dyn NativeValueSource>+Sync+'static> RetireOwned for NormInspectorPreparedStaticSource<T>{
 type RetirementCursor=Self;
 fn retirement_seed(self)->Self{self}
 fn controlled_retirement_supported()->bool{true}
}
impl<T:NativeReceive<dyn NativeValueSource>+Sync+'static> RetirementCursor for NormInspectorPreparedStaticSource<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.next_close_demand(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.next_close_work_demand(body)}
 semio_framework_value::native_retirement_reply!();
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 derived_cursor_quotes!();
}
/// 🧵️ Parser, native receiver, cache and original source remain separately closeable until transfer and retirement.
pub struct NormInspectorStaticSourcePreparation<T:NativeReceive<dyn NativeValueSource>+Sync+'static>{
 compiled:&'static CompiledMutationInputDefinitions,raw:&'static str,limits:JsonReadLimits,
 ledger:NativeAllocationIdentity,recipient:Option<u64>,phase:Phase,closing:bool,
 cursor:ManuallyDrop<Option<Cursor<T>>>,
 cache:ManuallyDrop<Option<Cache<T>>>,admission:ManuallyDrop<Option<Admission<T>>>,
 source:ManuallyDrop<Option<Source<T>>>,retirement:RetainedCloneClose,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"norm.inspector.static-source")}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn add(mut demand:RetirementDemand,bytes:usize)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(bytes).ok_or_else(invalid)?;Ok(demand)}
fn unit(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
fn settle<V>(poll:RetainedWorkPoll<V,NativeReceivingFailure>)->RetainedWorkOutcome<V,NativeReceivingFailure>{
 match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"norm.inspector.static-source child cannot stay pending on a synchronous recipient").into()),work}}
}
impl<T:NativeReceive<dyn NativeValueSource>+Sync+'static> NormInspectorStaticSourcePreparation<T>{
 pub fn constructor_demand()->RetirementDemand{header::<Self>()}
 fn admit(compiled:&'static CompiledMutationInputDefinitions,raw:Result<&'static str,CompiledMutationInputError>,limits:JsonReadLimits,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let demand=Self::constructor_demand();if !retained_clone_grant_admits(grant,demand){return Ok(None)}
  let raw=raw.map_err(|error|ValueError::literal(ValueRefusalKind::UnsupportedOwner,error.detail))?;
  Ok(Some((Self{compiled,raw,limits,ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),phase:Phase::Birth,closing:false,cursor:ManuallyDrop::new(None),cache:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),retirement:Default::default()},unit(demand))))
 }
 fn same(&self,control:&NativeDecodeControl<'_>)->Result<(),ValueError>{if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){return Err(invalid())}Ok(())}
 pub fn ready(&self)->bool{!self.closing&&matches!(self.phase,Phase::Ready)}
 pub fn demands(&self,body:usize,maximum_work:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body)}
  let demand=match self.phase{
   Phase::Birth=>Cursor::<T>::constructor_demand(),
   Phase::Read=>self.cursor.as_ref().ok_or_else(invalid)?.demands(body,maximum_work)?,
   Phase::Take=>add(self.cursor.as_ref().ok_or_else(invalid)?.demands(body,maximum_work)?,size_of::<Option<Cache<T>>>())?,
   Phase::SourceBirth=>header::<Option<Admission<T>>>(),
   Phase::Source=>{let admission=self.admission.as_ref().ok_or_else(invalid)?;if admission.terminal_is_empty(){header::<Option<Admission<T>>>()}else{admission.next_owned_demand()?}},
   Phase::Ready=>header::<Option<NormInspectorPreparedStaticSource<T>>>(),
  };add(demand,size_of::<Phase>())
 }
 pub fn advance_work_demand(&self,maximum_work:u64)->Result<u64,ValueError>{
  if self.closing{return Err(invalid())}
  match self.phase{
   Phase::Birth|Phase::SourceBirth=>Ok(1),
   Phase::Read=>Ok(maximum_work),
   Phase::Take=>self.cursor.as_ref().ok_or_else(invalid)?.take_work_demand(),
   Phase::Source=>{let admission=self.admission.as_ref().ok_or_else(invalid)?;if admission.terminal_is_empty(){Ok(1)}else{admission.next_owned_work_demand()}},
   Phase::Ready=>Err(invalid()),
  }
 }
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,NativeReceivingFailure>{
  let mut work=0u128;let result=self.turn(control,body,maximum_work,grant,&mut work);RetainedWorkOutcome{result,work}
 }
 fn turn(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work:u64,grant:RetainedCloneGrant,work:&mut u128)->Result<RetainedCloneStep,NativeReceivingFailure>{
  self.same(control)?;if self.closing||self.ready(){return Err(invalid().into())}
  let demand=self.demands(body,maximum_work)?;let denied=||Ok(RetainedCloneStep::Progress(Default::default()));
  if maximum_work==0||!retained_clone_grant_admits(grant,demand){return denied()}
  let units=self.advance_work_demand(maximum_work)?;control.checkpoint()?;
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Phase>(),..grant};
  let inline=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes-size_of::<Phase>(),..Default::default()};
  let mut progress=match self.phase{
   Phase::Birth=>{
    let(raw,limits)=(self.raw,self.limits);
    let Some(admitted)=run(control,demand,units,work,|control,effect|{pay(effect,1)?;Cursor::<T>::admit(raw,limits,control,child)})?else{return denied()};
    let Some((cursor,progress))=admitted else{return denied()};*self.cursor=Some(cursor);self.phase=Phase::Read;progress
   },
   Phase::Read=>{
    let witness=control.source_witness()?;let cursor=self.cursor.as_mut().ok_or_else(invalid)?;
    let turn=settle(cursor.advance(control,witness,body,maximum_work,child));*work+=turn.work;
    let progress=turn.result?.progress();if cursor.ready(){self.phase=Phase::Take}progress
   },
   Phase::Take=>{
    let cursor=self.cursor.as_mut().ok_or_else(invalid)?;let mut taken=None;
    let Some(step)=run(control,demand,units,work,|control,effect|cursor.take(control,&mut taken,effect).result)?else{return denied()};
    let mut progress=step.progress();let Some(cache)=taken else{return Ok(RetainedCloneStep::Progress(progress))};
    *self.cache=Some(cache);progress.copied_bytes+=size_of::<Option<Cache<T>>>();self.phase=Phase::SourceBirth;progress
   },
   Phase::SourceBirth=>{
    let cache=&mut self.cache;let admission=&mut self.admission;
    if run(control,demand,units,work,|_,effect|{pay(effect,1)?;let value=cache.take().ok_or_else(invalid)?;**admission=Some(StagedSourceAdmission::owned(value));Ok(())})?.is_none(){return denied()}
    self.phase=Phase::Source;inline
   },
   Phase::Source=>{
    let admission=self.admission.as_mut().ok_or_else(invalid)?;
    if admission.terminal_is_empty(){
     if run(control,demand,units,work,|_,effect|pay(effect,1))?.is_none(){return denied()}
     self.admission.take();if self.source.is_none(){return Err(invalid().into())}self.phase=Phase::Ready;inline
    }else{
     let source=&mut self.source;
     let Some(step)=run(control,demand,units,work,|_,effect|admission.advance_owned(source,effect).result)?else{return denied()};step.progress()
    }
   },
   Phase::Ready=>return Err(invalid().into()),
  };progress.copied_bytes+=size_of::<Phase>();Ok(RetainedCloneStep::Progress(progress))
 }
 pub fn take(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(NormInspectorPreparedStaticSource<T>,RetainedCloneProgress)>>{
  let mut work=0u128;let result=(||{
   self.same(control)?;if !self.ready(){return Err(invalid())}
   let demand=self.demands(grant.maximum_copy_bytes,1)?;if !retained_clone_grant_admits(grant,demand){return Ok(None)}
   control.checkpoint()?;
   if run(control,demand,1,&mut work,|_,effect|pay(effect,1))?.is_none(){return Ok(None)}
   let source=self.source.take().ok_or_else(invalid)?;self.begin_close();
   Ok(Some((NormInspectorPreparedStaticSource{source,compiled:self.compiled},unit(demand))))
  })();RetainedWorkOutcome{result,work}
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(cursor)=self.cursor.as_mut(){cursor.begin_close()}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(self.close_quote(body)?.0)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{Ok(self.close_quote(body)?.1)}
 fn close_quote(&self,body:usize)->Result<(RetirementDemand,u64),ValueError>{
  if !self.closing{return Err(invalid())}
  macro_rules! owned{($ty:ty,$present:expr)=>{return Ok((self.retirement.next_owner_demands::<$ty>($present,&None,body)?,self.retirement.next_owner_work::<$ty>($present,&None,body)?))};}
  if !self.retirement.is_empty(){owned!(Admission<T>,false)}
  if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok((header::<Option<Cursor<T>>>(),1))}else{Ok((cursor.close_demands(body)?,cursor.close_work_demand(body)?))}}
  if let Some(admission)=self.admission.as_ref(){if admission.terminal_is_empty(){return Ok((header::<Option<Admission<T>>>(),1))}owned!(Admission<T>,true)}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok((header::<Option<Source<T>>>(),1))}else{Ok((source.next_close_demand(body)?,source.next_close_work_demand(body)?))}}
  if self.cache.is_some(){owned!(Cache<T>,true)}
  Ok((Default::default(),0))
 }
 pub fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  use RetirementPollExit as X;
  let grant=effect.grant();let body=grant.maximum_copy_bytes;
  let result=(||->Result<RetainedCloneStep,X>{
   if !self.closing||self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){return Err(invalid().into())}
   let demand=self.close_demands(body)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   let released=||RetainedCloneStep::Progress(unit(demand));
   if !self.retirement.is_empty(){return X::resume(self.retirement.step_granted(body,effect))}
   if let Some(cursor)=self.cursor.as_mut(){if cursor.terminal_is_empty(){self.cursor.take();X::pay(effect,1)?;return Ok(released())}return X::resume(cursor.close_step(control,effect))}
   if self.admission.as_ref().is_some_and(|admission|admission.terminal_is_empty()){self.admission.take();X::pay(effect,1)?;return Ok(released())}
   if self.admission.is_some(){let turn=self.retirement.begin_granted(&mut self.admission,effect);return Ok(turn.result?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}
   if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();X::pay(effect,1)?;return Ok(released())}return X::resume(source.close_step(body,effect))}
   let turn=self.retirement.begin_granted(&mut self.cache,effect);Ok(turn.result?.unwrap_or(RetainedCloneStep::Progress(Default::default())))
  })();X::settle(effect,result)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.is_none()&&self.cache.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.retirement.is_empty()}
}
impl NormInspectorStaticSourcePreparation<ActionArgDef>{
 pub fn definitions(compiled:&'static CompiledMutationInputDefinitions,limits:JsonReadLimits,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{Self::admit(compiled,compiled.definitions_json,limits,control,grant)}
}
impl NormInspectorStaticSourcePreparation<ActionArgShapeSeed>{
 pub fn seeds(compiled:&'static CompiledMutationInputDefinitions,limits:JsonReadLimits,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{Self::admit(compiled,compiled.seed_definitions_json,limits,control,grant)}
}
impl<T:NativeReceive<dyn NativeValueSource>+Sync+'static> Drop for NormInspectorStaticSourcePreparation<T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"norm static input source requires funded closure")}}

#[cfg(test)]
#[path="static-source-tests.rs"]
mod tests;
