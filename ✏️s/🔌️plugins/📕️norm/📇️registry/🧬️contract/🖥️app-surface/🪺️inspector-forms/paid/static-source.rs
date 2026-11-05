//! 🏗️ Exact AOT declarations and seed rows become native immutable sources through one original decoder.
use semio_framework_replication_mutation::{CompiledMutationInputDefinitions,CompiledMutationInputError};
use semio_framework::{ActionArgDef,ActionArgShapeSeed,JsonNativeInputCursor};
use semio_framework_pack_json::JsonReadLimits;
use semio_framework_value::{NativeDecodeControl,OriginalRetirementControl,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::shared_issuance::SealedShared;
use semio_framework_value_native_owned::{NativeReceive,OriginalReceivingPrimary};
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource;
type Cache<T>=SealedShared<Vec<T>>;
type Source<T>=RetainedCloneSource<Vec<T>,Cache<T>>;
type Admission<T>=RetainedCloneSourceAdmission<Vec<T>,Cache<T>>;
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
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{self.source.close_step(grant)}
 pub fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()}
}
impl<T:NativeReceive<dyn NativeValueSource>+Sync+'static> semio_framework_value::retirement::RetireOwned for NormInspectorPreparedStaticSource<T>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<T:NativeReceive<dyn NativeValueSource>+Sync+'static> semio_framework_value::retirement::RetirementCursor for NormInspectorPreparedStaticSource<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.next_close_demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{let turn=Self::close_step(self,grant);match turn.result{Ok(step)=>semio_framework_value::retirement::RetirementStep::Progress(step.progress(),turn.work),Err(error)=>semio_framework_value::retirement::RetirementStep::Failure(error,turn.work)}}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
/// 🧵️ Parser, native receiver, cache and original source remain separately closeable until transfer and retirement.
pub struct NormInspectorStaticSourcePreparation<T:NativeReceive<dyn NativeValueSource>+Sync+'static>{
 compiled:&'static CompiledMutationInputDefinitions,raw:&'static str,limits:JsonReadLimits,
 ledger:u64,recipient:Option<u64>,phase:Phase,closing:bool,
 cursor:ManuallyDrop<Option<JsonNativeInputCursor<'static,Cache<T>>>>,
 cache:ManuallyDrop<Option<Cache<T>>>,admission:ManuallyDrop<Option<Admission<T>>>,
 source:ManuallyDrop<Option<Source<T>>>,retirement:RetainedCloneClose,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"norm.inspector.static-source")}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn add(mut demand:RetirementDemand,bytes:usize)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(bytes).ok_or_else(invalid)?;Ok(demand)}
impl<T:NativeReceive<dyn NativeValueSource>+Sync+'static> NormInspectorStaticSourcePreparation<T>{
 pub fn constructor_demand()->RetirementDemand{header::<Self>()}
 fn admit(compiled:&'static CompiledMutationInputDefinitions,raw:Result<&'static str,CompiledMutationInputError>,limits:JsonReadLimits,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let demand=Self::constructor_demand();if !retained_clone_grant_admits(grant,demand){return Ok(None)}
  let raw=raw.map_err(|error|ValueError::literal(ValueRefusalKind::UnsupportedOwner,error.detail))?;
  Ok(Some((Self{compiled,raw,limits,ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),phase:Phase::Birth,closing:false,cursor:ManuallyDrop::new(None),cache:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),retirement:Default::default()},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
 }
 fn same(&self,control:&NativeDecodeControl<'_>)->Result<(),ValueError>{if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){return Err(invalid())}Ok(())}
 pub fn ready(&self)->bool{!self.closing&&matches!(self.phase,Phase::Ready)}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body)}
  let demand=match self.phase{
   Phase::Birth=>JsonNativeInputCursor::<Cache<T>>::constructor_demand(),
   Phase::Read=>self.cursor.as_ref().ok_or_else(invalid)?.demands(body)?,
   Phase::Take=>add(self.cursor.as_ref().ok_or_else(invalid)?.demands(body)?,size_of::<Option<Cache<T>>>())?,
   Phase::SourceBirth=>header::<Option<Admission<T>>>(),
   Phase::Source=>{let admission=self.admission.as_ref().ok_or_else(invalid)?;if admission.phase()=="ready"{header::<Option<Source<T>>>()}else{admission.next_demand()?}},
   Phase::Ready=>header::<Option<NormInspectorPreparedStaticSource<T>>>(),
  };add(demand,size_of::<Phase>())
 }
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
  let mut work=0;let result=(||{
   self.same(control)?;if self.closing||self.ready(){return Err(invalid().into())}
   let demand=self.demands(body)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
   control.checkpoint()?;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Phase>(),..grant};
   let mut progress=match self.phase{
    Phase::Birth=>{let Some((cursor,progress))=JsonNativeInputCursor::admit(self.raw,self.limits,control,child)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*self.cursor=Some(cursor);self.phase=Phase::Read;work=1;progress},
    Phase::Read=>{let cursor=self.cursor.as_mut().ok_or_else(invalid)?;let turn=cursor.advance(control,body,child);work=turn.work;let progress=turn.result?.progress();if cursor.ready(){self.phase=Phase::Take}progress},
    Phase::Take=>{let turn=self.cursor.as_mut().ok_or_else(invalid)?.take(control,RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-size_of::<Option<Cache<T>>>(),..child});work=turn.work;let Some((cache,mut progress))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*self.cache=Some(cache);progress.copied_bytes+=size_of::<Option<Cache<T>>>();self.phase=Phase::SourceBirth;progress},
    Phase::SourceBirth=>{*self.admission=Some(RetainedCloneSourceAdmission::new(self.cache.take().ok_or_else(invalid)?,(),|cache|cache.get()));self.phase=Phase::Source;work=1;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<Admission<T>>>(),..Default::default()}},
    Phase::Source=>{let admission=self.admission.as_mut().ok_or_else(invalid)?;let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,child)}else{admission.advance(child)};let turn=control.receive_retained_work(child,turn);work=turn.work;let progress=turn.result?.progress();if admission.terminal_is_empty(){self.admission.take();self.phase=Phase::Ready}progress},
    Phase::Ready=>return Err(invalid().into()),
   };progress.copied_bytes+=size_of::<Phase>();Ok(RetainedCloneStep::Progress(progress))
  })();RetainedWorkOutcome{result,work}
 }
 pub fn take(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(NormInspectorPreparedStaticSource<T>,RetainedCloneProgress)>>{
  let mut work=0;let result=(||{
   self.same(control)?;if !self.ready(){return Err(invalid())}
   let demand=self.demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(None)}
   control.checkpoint()?;let source=self.source.take().ok_or_else(invalid)?;self.begin_close();work=1;
   Ok(Some((NormInspectorPreparedStaticSource{source,compiled:self.compiled},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
  })();RetainedWorkOutcome{result,work}
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(cursor)=self.cursor.as_mut(){cursor.begin_close()}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid())}
  if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok(header::<Option<JsonNativeInputCursor<'static,Cache<T>>>>())}else{cursor.close_demands(body)}}
  if let Some(admission)=self.admission.as_ref(){return if admission.terminal_is_empty(){Ok(header::<Option<Admission<T>>>())}else if admission.phase()=="ready"{Ok(header::<Option<Source<T>>>())}else{admission.next_return_demand()}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<Source<T>>>())}else{source.next_close_demand(body)}}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Cache<T>>(false,&None,body)}
  if self.cache.is_some(){return self.retirement.next_owner_demands::<Cache<T>>(true,&None,body)}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,control:&mut dyn OriginalRetirementControl,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let mut work=0;let result=(||{
   if self.ledger!=control.ledger_identity()||self.recipient!=control.retirement_recipient_identity(){return Err(invalid())}
   let demand=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   if let Some(cursor)=self.cursor.as_mut(){if cursor.terminal_is_empty(){self.cursor.take();work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}let turn=cursor.close_step(control,grant);work=turn.work;return turn.result}
   if demand.capacity_bytes!=0{control.charge(demand.capacity_bytes)?}
   if let Some(admission)=self.admission.as_mut(){
    if admission.terminal_is_empty(){self.admission.take();work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
    let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,grant)}else{let mut returned=None;let turn=admission.return_inputs(&mut returned,grant);if let Some((cache,()))=returned{*self.cache=Some(cache)}turn};work=turn.work;return turn.result
   }
   if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}let turn=source.close_step(grant);work=turn.work;return turn.result}
   let turn=if self.retirement.is_empty(){let turn=self.retirement.begin_granted(&mut self.cache,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}else{self.retirement.step_granted(grant)};work=turn.work;turn.result
  })();RetainedWorkOutcome{result,work}
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
