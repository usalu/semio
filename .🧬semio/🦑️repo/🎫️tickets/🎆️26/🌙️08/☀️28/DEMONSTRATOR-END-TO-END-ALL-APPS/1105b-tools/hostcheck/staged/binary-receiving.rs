//! 🔢️ Persisted binary operations become typed native projections through the vocabulary's own receiving cursor, never through a cold decode; the installer names this encoding explicitly.
use super::{ArtifactOriginalOperationReceiving,OriginalOperationReceivingAdmission};
use protocol::{OpBinaryReceiving,OpBinaryReceivingCursor};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,FactoryAuthority,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneSourceTake,RetainedCloneStep,RetainedOwnedProjection,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary};
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
use std::{mem::{ManuallyDrop,size_of},sync::Arc};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase{Admit,Receive,Take,CloseCursor,Authority,TypedSource,TypedProjection,Ready}

/// 🔢️ Retains the original binary wire, the receiving cursor and the typed owners until funded closure.
pub struct OriginalBinaryReceiving<M:OpBinaryReceiving+Sync>{
 ledger:u64,recipient:Option<u64>,wire:ManuallyDrop<Option<RetainedOwnedProjection<Vec<u8>>>>,
 cursor:ManuallyDrop<Option<M::Receiving>>,typed:ManuallyDrop<Option<M>>,authority:ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,
 typed_admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<M,FactoryOwnedSourceAuthority<M>>>>,typed_source:ManuallyDrop<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>,
 output:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,factory_close:Option<FactoryAuthority>,
 retirement:RetainedCloneClose,phase:Phase,closing:bool,faulted:bool,taking_owned:bool,
}
fn refused(failure:OriginalReceivingPrimary)->ValueError{let progress=failure.progress;failure.primary.or(failure.quote_refusal).or(failure.allocation_refusal).unwrap_or_else(||invalid("original binary native construction refused")).with_retained_progress(progress)}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn close_header<T>()->RetirementDemand{let _=std::marker::PhantomData::<T>;RetirementDemand{depth:1,..Default::default()}}
impl<M:OpBinaryReceiving+Sync> OriginalBinaryReceiving<M>{
 pub fn constructor_demand()->RetirementDemand{header::<Self>()}
 pub fn admit(wire:RetainedOwnedProjection<Vec<u8>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,control:&NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,RetainedOwnedProjection<Vec<u8>>)>{
  let d=Self::constructor_demand();if !retained_clone_grant_admits(g,d){return Err((invalid("original binary receiving shell was not funded"),wire));}
  Ok((Self{ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity(),wire:ManuallyDrop::new(Some(wire)),cursor:ManuallyDrop::new(None),typed:ManuallyDrop::new(None),authority:ManuallyDrop::new(None),typed_admission:ManuallyDrop::new(None),typed_source:ManuallyDrop::new(None),output:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(Arc::clone(factory))),factory_close:None,retirement:Default::default(),phase:Phase::Admit,closing:false,faulted:false,taking_owned:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 fn source(&self)->Result<&Vec<u8>,ValueError>{Ok(self.wire.as_ref().ok_or_else(||invalid("original binary wire is absent"))?.borrow()?.get())}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body);}if self.faulted{return Err(invalid("faulted original binary receiving requires cleanup"));}
  Ok(match self.phase{
   Phase::Admit=>RetirementDemand{depth:1,..Default::default()},
   Phase::Receive=>self.cursor.as_ref().ok_or_else(||invalid("original binary cursor is absent"))?.demands(self.source()?,body)?,
   Phase::Take=>RetirementDemand{depth:1,..Default::default()},
   Phase::CloseCursor=>{let cursor=self.cursor.as_ref().ok_or_else(||invalid("original binary cursor is absent"))?;if cursor.terminal_is_empty(){close_header::<Option<M::Receiving>>()}else{cursor.close_demands(body)?}},
   Phase::Authority=>header::<FactoryOwnedSourceAuthority<M>>(),
   Phase::TypedSource=>{let admission=self.typed_admission.as_ref().ok_or_else(||invalid("original binary typed source admission is absent"))?;if admission.phase()=="ready"{header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>()}else{admission.next_demand()?}},
   Phase::TypedProjection=>RetainedOwnedProjection::<M>::constructor_demand(),
   Phase::Ready=>header::<Option<RetainedOwnedProjection<M>>>(),
  })
 }
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=self.advance_turn(control,body,g,&mut work);RetainedWorkOutcome{result,work}}
 fn advance_turn(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant,work:&mut u64)->Result<RetainedCloneStep,ValueError>{
  if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original binary receiving changed its decoder ledger"));}
  let d=self.demands(body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.closing{return control.with_retirement(g,|retirement|{let turn=self.close_step(retirement,g);*work=turn.work;turn.result});}
  control.checkpoint()?;*work=1;if self.phase!=Phase::Receive{control.admit_turn_capacity(g.maximum_capacity_bytes)?;if d.capacity_bytes!=0&&self.phase!=Phase::TypedSource{control.charge(d.capacity_bytes)?;}}
  let child=RetainedCloneGrant{maximum_items:1,..g};let unit=||RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  let mut p=match self.phase{
   Phase::Admit=>{let wire=self.wire.as_ref().ok_or_else(||invalid("original binary wire is absent"))?.borrow()?;let bytes=wire.get();let turn=M::admit_receiving(bytes,(0,bytes.len()),child);*work=turn.work;match turn.result?{Some((cursor,p))=>{*self.cursor=Some(cursor);self.phase=Phase::Receive;p},None=>return Ok(RetainedCloneStep::Progress(Default::default()))}},
   Phase::Receive=>{let wire=self.wire.as_ref().ok_or_else(||invalid("original binary wire is absent"))?.borrow()?;let cursor=self.cursor.as_mut().ok_or_else(||invalid("original binary cursor is absent"))?;let turn=cursor.step(wire.get(),control,body,g);*work=turn.work;let p=turn.result.map_err(refused)?;if cursor.is_ready(){self.phase=Phase::Take;}p},
   Phase::Take=>{let cursor=self.cursor.as_mut().ok_or_else(||invalid("original binary cursor is absent"))?;let turn=cursor.take_into(&mut self.typed,child);*work=turn.work;let p=turn.result?;if self.typed.is_some(){self.phase=Phase::CloseCursor;}p},
   Phase::CloseCursor=>{let cursor=self.cursor.as_mut().ok_or_else(||invalid("original binary cursor is absent"))?;if cursor.terminal_is_empty(){if let Some(cursor)=self.cursor.take(){std::mem::forget(cursor);}self.phase=Phase::Authority;unit()}else{let turn=cursor.close_step(child);*work=turn.work;turn.result?.progress()}},
   Phase::Authority=>{let authority=FactoryOwnedSourceAuthority::new(self.typed.take().ok_or_else(||invalid("original binary typed value is absent"))?,Arc::clone(self.factory.as_ref().ok_or_else(||invalid("original binary factory is absent"))?));*self.typed_admission=Some(RetainedCloneSourceAdmission::new(authority,(),|original:&FactoryOwnedSourceAuthority<M>|original.get()));self.phase=Phase::TypedSource;unit()},
   Phase::TypedSource=>{let admission=self.typed_admission.as_mut().ok_or_else(||invalid("original binary typed admission is absent"))?;if admission.phase()=="ready"{let turn=admission.take_output(&mut self.typed_source,child);*work=turn.work;let step=turn.result?;if admission.terminal_is_empty(){self.typed_admission.take();self.phase=Phase::TypedProjection;}step.progress()}else{let turn=admission.advance(child);*work=turn.work;let p=turn.result?.progress();if p.retained_capacity_bytes!=0{control.charge(p.retained_capacity_bytes)?;}p}},
   Phase::TypedProjection=>{let turn=self.typed_source.as_ref().ok_or_else(||invalid("original binary typed source is absent"))?.retain_original(child);*work=turn.work;let(output,p)=turn.result?;*self.output=Some(output);self.phase=Phase::Ready;p},
   Phase::Ready=>return Ok(RetainedCloneStep::Complete(Default::default())),
  };
  p.copied_items=p.copied_items.max(1);semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original binary receiving")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn ready(&self)->bool{!self.taking_owned&&!self.closing&&!self.faulted&&self.phase==Phase::Ready&&self.output.is_some()}
 pub fn owned_handoff_pending(&self)->bool{self.taking_owned&&!self.closing&&!self.faulted}
 pub fn take(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>>{let result=self.take_ready(g);let work=u64::from(matches!(result,Ok(Some(_))));RetainedWorkOutcome{result,work}}
 fn take_ready(&mut self,g:RetainedCloneGrant)->Result<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Ok(None);}let d=header::<Option<RetainedOwnedProjection<M>>>();if !retained_clone_grant_admits(g,d){return Ok(None);}let output=self.output.take().ok_or_else(||invalid("original binary receiving output already transferred"))?;self.begin_close();Ok(Some((output,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn take_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.faulted||self.phase!=Phase::Ready||(!self.taking_owned&&self.output.is_none()){return Err(invalid("original binary owned handoff requires its completed output"))}
  if !self.taking_owned{return Ok(header::<bool>())}
  if let Some(output)=self.output.as_ref(){return if output.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{output.close_demand(body)}}
  if let Some(source)=self.typed_source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>())}else{source.next_take_demand(body)}}
  if self.authority.is_some(){return Ok(RetirementDemand{copy_bytes:FactoryOwnedSourceAuthority::<M>::transfer_copy_byte_demand()+size_of::<Option<M>>(),depth:1,..Default::default()})}
  Err(invalid("original binary owned handoff lost its unique authority"))
 }
 pub fn take_owned_into(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=self.take_owned_turn(output,control,body,g,&mut work);RetainedWorkOutcome{result,work}}
 fn take_owned_turn(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant,work:&mut u64)->Result<RetainedCloneStep,ValueError>{
  if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original binary owned handoff changed its decoder ledger"))}if output.is_some(){return Err(invalid("original binary owned handoff destination is occupied"))}
  let d=self.take_owned_demands(body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;*work=1;control.admit_turn_capacity(g.maximum_capacity_bytes)?;if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?}
  let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  if !self.taking_owned{self.taking_owned=true;return Ok(unit())}
  if let Some(projection)=self.output.as_mut(){if projection.terminal_is_empty(){self.output.take();return Ok(unit())}let turn=projection.close_step(g);*work=turn.work;return turn.result}
  if let Some(source)=self.typed_source.as_mut(){if source.terminal_is_empty(){self.typed_source.take();return Ok(unit())}let turn=source.take_authority(g);*work=turn.work;return match turn.result?{RetainedCloneSourceTake::Pending(p)=>Ok(RetainedCloneStep::Progress(p)),RetainedCloneSourceTake::Ready(authority,p)=>{*self.authority=Some(authority);Ok(RetainedCloneStep::Progress(p))}}}
  let authority=self.authority.take().ok_or_else(||invalid("original binary owned handoff requires its unique authority"))?;match authority.take_original(g){Ok((value,factory,mut p))=>{*output=Some(value);self.factory_close=Some(FactoryAuthority::new(factory));self.closing=true;p.copied_bytes+=size_of::<Option<M>>();Ok(RetainedCloneStep::Complete(p))},Err((error,authority))=>{*self.authority=Some(authority);Err(error)}}
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<M>(false,&None,body);}
  if let Some(admission)=self.typed_admission.as_ref(){return admission.next_return_demand();}
  if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok(close_header::<Option<M::Receiving>>())}else{cursor.close_demands(body)};}
  if let Some(output)=self.output.as_ref(){return if output.terminal_is_empty(){Ok(close_header::<Option<RetainedOwnedProjection<M>>>())}else{output.close_demand(body)};}
  if let Some(source)=self.typed_source.as_ref(){return if source.terminal_is_empty(){Ok(close_header::<Option<RetainedCloneSource<M,FactoryOwnedSourceAuthority<M>>>>())}else{source.next_close_demand(body)};}
  if let Some(wire)=self.wire.as_ref(){return if wire.terminal_is_empty(){Ok(close_header::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{wire.close_demand(body)};}
  if self.typed.is_some(){return self.retirement.next_owner_demands::<M>(true,&None,body);}
  if self.authority.is_some(){return self.retirement.next_owner_demands::<FactoryOwnedSourceAuthority<M>>(true,&None,body);}
  if let Some(factory)=self.factory_close.as_ref(){return if factory.terminal_is_empty(){Ok(close_header::<FactoryAuthority>())}else{factory.demands(body)};}if self.factory.is_some(){return Ok(close_header::<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>());}Ok(Default::default())
 }
 pub fn close_step(&mut self,control:&mut dyn semio_framework_value::OriginalRetirementControl,g:RetainedCloneGrant)->RetainedWorkOutcome{
  let mut work=0;
  let result=(||{
   if control.ledger_identity()!=self.ledger||control.retirement_recipient_identity()!=self.recipient{return Err(invalid("original binary cleanup changed its decoder ledger"));}
   if !self.closing{return Err(invalid("original binary cleanup was not started"));}
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
   let d=self.close_demands(g.maximum_copy_bytes)?;
   if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
   if d.capacity_bytes!=0{control.charge(d.capacity_bytes)?;}
   let unit=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});
   if !self.retirement.is_empty(){let turn=self.retirement.step_granted(g);work=turn.work;return turn.result;}
   if let Some(admission)=self.typed_admission.as_mut(){if admission.output_published(){let turn=admission.take_output(&mut self.typed_source,g);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()));}let mut returned=None;let turn=admission.return_inputs(&mut returned,g);if let Some((authority,()))=returned{*self.authority=Some(authority);}if admission.terminal_is_empty(){self.typed_admission.take();}work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()));}
   if let Some(cursor)=self.cursor.as_mut(){if cursor.terminal_is_empty(){if let Some(cursor)=self.cursor.take(){std::mem::forget(cursor);}work=1;return Ok(unit());}let turn=cursor.close_step(g);work=turn.work;return turn.result;}
   macro_rules! close{($field:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field.take();work=1;return Ok(unit());}let turn=owner.close_step(g);work=turn.work;return turn.result;}};}
   close!(output);close!(typed_source);close!(wire);
   macro_rules! owned{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,g);work=turn.work;if let Some(step)=turn.result?{return Ok(step);}}};}
   owned!(typed);owned!(authority);
   if self.factory_close.is_none(){if let Some(factory)=self.factory.take(){self.factory_close=Some(FactoryAuthority::new(factory));work=1;return Ok(unit());}}
   if let Some(factory)=self.factory_close.as_mut(){
    if factory.terminal_is_empty(){self.factory_close=None;work=1;return Ok(unit());}
    let turn=factory.step(g);work=turn.work;return turn.result;
   }
   Ok(RetainedCloneStep::Complete(Default::default()))
  })();
  RetainedWorkOutcome{result,work}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.wire.is_none()&&self.cursor.is_none()&&self.typed.is_none()&&self.authority.is_none()&&self.typed_admission.is_none()&&self.typed_source.is_none()&&self.output.is_none()&&self.factory.is_none()&&self.factory_close.is_none()&&self.retirement.is_empty()}
}
impl<M:OpBinaryReceiving+Sync> Drop for OriginalBinaryReceiving<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original binary receiving requires funded closure of all source and typed owners");}}
impl<M:OpBinaryReceiving+Sync+Send+'static> ArtifactOriginalOperationReceiving<M> for OriginalBinaryReceiving<M> where M::Receiving:Send{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome{self.advance(control,body,grant)}
 fn ready(&self)->bool{self.ready()}
 fn owned_handoff_pending(&self)->bool{self.owned_handoff_pending()}
 fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>>{self.take(grant)}
 fn take_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.take_owned_demands(body)}
 fn take_owned_into(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome{self.take_owned_into(output,control,body,grant)}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
 fn close_step(&mut self,control:&mut dyn semio_framework_value::OriginalRetirementControl,grant:RetainedCloneGrant)->RetainedWorkOutcome{self.close_step(control,grant)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
 fn allocation_bytes(&self)->usize{size_of::<Self>()}
}
fn demands<M:OpBinaryReceiving+Sync>()->RetirementDemand{let d=OriginalBinaryReceiving::<M>::constructor_demand();RetirementDemand{copy_bytes:d.copy_bytes+size_of::<Box<dyn ArtifactOriginalOperationReceiving<M>>>(),capacity_bytes:size_of::<OriginalBinaryReceiving<M>>(),..d}}
fn admit<M:OpBinaryReceiving+Sync+Send+'static>(wire:&mut Option<RetainedOwnedProjection<Vec<u8>>>,output:&mut Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> where M::Receiving:Send{
 if output.is_some()||wire.is_none(){return Err(invalid("original binary receiver requires its retained wire and empty output"))}let d=demands::<M>();if !retained_clone_grant_admits(grant,d){return Ok(Default::default())}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;
 match OriginalBinaryReceiving::admit(wire.take().ok_or_else(||invalid("original binary receiver lost its wire"))?,factory,control,grant){Ok((receiver,_))=>{*output=Some(Box::new(receiver));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})},Err((error,original))=>{*wire=Some(original);Err(error)}}
}
impl<M:OpBinaryReceiving+Sync+Send+'static> OriginalOperationReceivingAdmission<M> where M::Receiving:Send{
 /// 🔢️ The admission of a vocabulary whose persisted operations are binary-encoded.
 pub fn binary(batch_retirement:crate::lifted_member_owned_batch::BatchRetirementIssuer<M>)->Self{Self{batch_retirement,demands:demands::<M>,admit:admit::<M>}}
}
