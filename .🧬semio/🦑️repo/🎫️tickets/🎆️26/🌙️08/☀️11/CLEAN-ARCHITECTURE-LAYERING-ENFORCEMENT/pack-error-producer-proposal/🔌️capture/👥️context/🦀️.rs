//! 👥️ Unmounted fallible shared admission context; native compiler and concurrent laws remain pending.
use super::{OwnedTransportError,ReservedCell,TransportAdmission,TransportCaptureRefusal,TransportCell};
use crate::{PackError,PackRetryDisposition,PackTransportCategory};
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::error::Error;
use std::ptr::NonNull;
use std::sync::{Mutex,atomic::{AtomicBool,AtomicUsize,Ordering}};

/// 🚦️ Every fallible checkpoint has an actual source-authored phase and byte witness.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PackTransportPhase{ContextBefore,ContextAfter,ContextReady,BeforeReserve,AfterReserve,ReservationCommitted,ReservationReleased,SourcePublished,ContextClosed,BeforeOperation,AfterOperation}
/// 📊️ Owned policies receive finite requested/physical bytes and cumulative admitted charge.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct PackTransportProgress{pub phase:PackTransportPhase,pub category:Option<PackTransportCategory>,pub bytes:usize,pub committed_bytes:usize}
/// 🎛️ The caller owns cancellation/progress authority and original typed refusal messages.
pub trait PackTransportPolicy:Send+Sync{fn checkpoint(&self,progress:PackTransportProgress)->Result<(),ValueError>;fn progress(&self,progress:PackTransportProgress);}
/// 🛑️ Construction returns the original admission and policy before either can be erased.
pub enum TransportContextRefusalCause{Admission(TransportCaptureRefusal),Value(ValueError)}
pub struct TransportContextRefusal<P>{pub cause:TransportContextRefusalCause,pub admission:TransportAdmission,pub policy:P}

struct ContextCell<P>{references:AtomicUsize,admission:Mutex<TransportAdmission>,canceled:AtomicBool,policy:P}
#[derive(Clone,Copy)]
struct ContextVTable{
 admission:unsafe fn(NonNull<()>)->*const Mutex<TransportAdmission>,
 canceled:unsafe fn(NonNull<()>)->*const AtomicBool,
 policy:unsafe fn(NonNull<()>)->*const(dyn PackTransportPolicy+'static),
 retain:unsafe fn(NonNull<()>),release:unsafe fn(NonNull<()>,usize),
}
/// 🪪️ One fallibly admitted context cell is shared by caller, source, sink and published causes.
pub struct PackTransportContext{pointer:NonNull<()>,capacity:usize,allocated_bytes:usize,vtable:ContextVTable}
unsafe impl Send for PackTransportContext{}
unsafe impl Sync for PackTransportContext{}
impl Clone for PackTransportContext{fn clone(&self)->Self{unsafe{(self.vtable.retain)(self.pointer)};Self{pointer:self.pointer,capacity:self.capacity,allocated_bytes:self.allocated_bytes,vtable:self.vtable}}}
impl Drop for PackTransportContext{fn drop(&mut self){unsafe{(self.vtable.release)(self.pointer,self.capacity)}}}
impl PackTransportContext{
 pub fn try_new<P:PackTransportPolicy+'static>(admission:TransportAdmission,policy:P)->Result<Self,TransportContextRefusal<P>>{create_with(admission,policy,|storage|storage.try_reserve_exact(1).is_ok())}
 pub const fn allocated_bytes(&self)->usize{self.allocated_bytes}
 pub fn committed_bytes(&self)->usize{self.admission().lock().unwrap_or_else(std::sync::PoisonError::into_inner).committed_bytes}
 pub fn cancel(&self){self.canceled().store(true,Ordering::Release);}
 pub fn checkpoint(&self,phase:PackTransportPhase,category:PackTransportCategory,bytes:usize)->Result<(),PackError>{
  if self.canceled().load(Ordering::Acquire){return Err(PackError::TransportAdmission{category,refusal:refusal(ValueRefusalKind::Canceled,"shared transport context canceled",if phase==PackTransportPhase::AfterReserve{bytes}else{0})});}
  self.policy().checkpoint(PackTransportProgress{phase,category:Some(category),bytes,committed_bytes:self.committed_bytes()}).map_err(PackError::ValueRefusal)?;
  if self.canceled().load(Ordering::Acquire){return Err(PackError::TransportAdmission{category,refusal:refusal(ValueRefusalKind::Canceled,"shared transport context canceled",if phase==PackTransportPhase::AfterReserve{bytes}else{0})});}
  Ok(())
 }
 pub fn reserve<E:Error+Send+Sync+'static>(&self,category:PackTransportCategory,retry:PackRetryDisposition)->Result<SharedTransportReservation<E>,PackError>{
  reserve_error_with(self,category,retry,|storage|storage.try_reserve_exact(1).is_ok())
 }
 pub fn operation<E:Error+Send+Sync+'static,T,F:FnOnce()->Result<(T,usize),E>>(&self,category:PackTransportCategory,retry:PackRetryDisposition,requested_bytes:usize,operation:F)->Result<T,PackError>{
  let reserve=self.reserve::<E>(category,retry)?;
  self.checkpoint(PackTransportPhase::BeforeOperation,category,requested_bytes)?;
  match operation(){
   Ok((value,completed_bytes))=>{drop(reserve);self.checkpoint(PackTransportPhase::AfterOperation,category,completed_bytes)?;Ok(value)},
   Err(error)=>Err(PackError::TransportFailure(reserve.publish(error))),
  }
 }
 fn admission(&self)->&Mutex<TransportAdmission>{unsafe{&*(self.vtable.admission)(self.pointer)}}
 fn canceled(&self)->&AtomicBool{unsafe{&*(self.vtable.canceled)(self.pointer)}}
 fn policy(&self)->&(dyn PackTransportPolicy+'static){unsafe{&*(self.vtable.policy)(self.pointer)}}
 fn refund(&self,bytes:usize){let mut admission=self.admission().lock().unwrap_or_else(std::sync::PoisonError::into_inner);admission.committed_bytes=admission.committed_bytes.checked_sub(bytes).expect("live shared reservation charge is present");}
 fn report(&self,phase:PackTransportPhase,category:Option<PackTransportCategory>,bytes:usize){self.policy().progress(PackTransportProgress{phase,category,bytes,committed_bytes:self.committed_bytes()});}
}

/// 📦️ No ledger mutex remains held during IO; unused capacity refunds only its own charge.
pub struct SharedTransportReservation<E:Error+Send+Sync+'static>{cell:Option<ReservedCell<E>>,claim:AdmissionClaim}
impl<E:Error+Send+Sync+'static> SharedTransportReservation<E>{
 pub fn allocated_bytes(&self)->usize{self.cell.as_ref().unwrap().allocated_bytes}
 pub fn publish(mut self,source:E)->OwnedTransportError{let cell=self.cell.take().unwrap();let category=cell.category;let bytes=cell.allocated_bytes;let source=cell.publish(source,Some(self.claim.context.clone()));self.claim.bytes=0;self.claim.context.report(PackTransportPhase::SourcePublished,Some(category),bytes);source}
}
impl<E:Error+Send+Sync+'static> Drop for SharedTransportReservation<E>{fn drop(&mut self){if let Some(cell)=self.cell.take(){drop(cell);let bytes=self.claim.refund();if !std::thread::panicking(){self.claim.context.report(PackTransportPhase::ReservationReleased,None,bytes);}}}}

struct AdmissionClaim{context:PackTransportContext,bytes:usize}
impl AdmissionClaim{fn refund(&mut self)->usize{let bytes=std::mem::replace(&mut self.bytes,0);if bytes!=0{self.context.refund(bytes);}bytes}}
impl Drop for AdmissionClaim{fn drop(&mut self){self.refund();}}

fn refusal(kind:ValueRefusalKind,reason:&'static str,physical_bytes:usize)->TransportCaptureRefusal{TransportCaptureRefusal{kind,reason,allocated_bytes:0,physical_bytes}}
fn progress(phase:PackTransportPhase,bytes:usize,committed_bytes:usize)->PackTransportProgress{PackTransportProgress{phase,category:None,bytes,committed_bytes}}

fn create_with<P:PackTransportPolicy+'static,A:FnOnce(&mut Vec<ContextCell<P>>)->bool>(mut admission:TransportAdmission,policy:P,allocate:A)->Result<PackTransportContext,TransportContextRefusal<P>>{
 let requested=std::mem::size_of::<ContextCell<P>>();let before=admission.committed_bytes;
 let fail=|cause,admission,policy|TransportContextRefusal{cause,admission,policy};
 let Some(remaining)=admission.maximum_bytes.checked_sub(before) else{return Err(fail(TransportContextRefusalCause::Admission(refusal(ValueRefusalKind::OwnershipLimit,"context admission baseline exceeds maximum",0)),admission,policy));};
 if requested>isize::MAX as usize||requested>remaining{return Err(fail(TransportContextRefusalCause::Admission(refusal(ValueRefusalKind::OwnershipLimit,"context cell exceeds finite physical credit",0)),admission,policy));}
 if let Err(error)=policy.checkpoint(progress(PackTransportPhase::ContextBefore,requested,before)){return Err(fail(TransportContextRefusalCause::Value(error),admission,policy));}
 let mut storage=Vec::new();
 if !allocate(&mut storage){return Err(fail(TransportContextRefusalCause::Admission(refusal(ValueRefusalKind::AllocationFailed,"context cell reserve failed",0)),admission,policy));}
 let Some(physical)=storage.capacity().checked_mul(requested) else{return Err(fail(TransportContextRefusalCause::Admission(refusal(ValueRefusalKind::OwnershipLimit,"context capacity overflow",0)),admission,policy));};
 if storage.capacity()<1||!storage.is_empty(){return Err(fail(TransportContextRefusalCause::Admission(refusal(ValueRefusalKind::InvariantViolated,"context allocator violated empty-cell reservation",physical)),admission,policy));}
 if physical>remaining{return Err(fail(TransportContextRefusalCause::Admission(refusal(ValueRefusalKind::OwnershipLimit,"context actual capacity exceeds finite physical credit",physical)),admission,policy));}
 if let Err(error)=policy.checkpoint(progress(PackTransportPhase::ContextAfter,physical,before)){return Err(fail(TransportContextRefusalCause::Value(error),admission,policy));}
 admission.committed_bytes=before+physical;
 storage.push(ContextCell{references:AtomicUsize::new(1),admission:Mutex::new(admission),canceled:AtomicBool::new(false),policy});
 let capacity=storage.capacity();let pointer=NonNull::new(storage.as_mut_ptr()).unwrap().cast();std::mem::forget(storage);
 let context=PackTransportContext{pointer,capacity,allocated_bytes:physical,vtable:ContextVTable{admission:admission_ptr::<P>,canceled:canceled_ptr::<P>,policy:policy_ptr::<P>,retain:retain::<P>,release:release::<P>}};
 context.report(PackTransportPhase::ContextReady,None,physical);Ok(context)
}

fn reserve_error_with<E:Error+Send+Sync+'static,A:FnOnce(&mut Vec<TransportCell<E>>)->bool>(context:&PackTransportContext,category:PackTransportCategory,retry:PackRetryDisposition,allocate:A)->Result<SharedTransportReservation<E>,PackError>{
 let requested=std::mem::size_of::<TransportCell<E>>();
 let fail=|kind,reason,physical_bytes|PackError::TransportAdmission{category,refusal:refusal(kind,reason,physical_bytes)};
 {
  let admission=context.admission().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
  if requested>isize::MAX as usize||requested>admission.maximum_bytes.saturating_sub(admission.committed_bytes){return Err(fail(ValueRefusalKind::OwnershipLimit,"shared source cell exceeds finite physical credit",0));}
 }
 context.checkpoint(PackTransportPhase::BeforeReserve,category,requested)?;
 let mut claim=AdmissionClaim{context:context.clone(),bytes:0};
 {
  let mut admission=context.admission().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
  if requested>admission.maximum_bytes.saturating_sub(admission.committed_bytes){return Err(fail(ValueRefusalKind::OwnershipLimit,"shared source pending cell exceeds remaining credit",0));}
  admission.committed_bytes+=requested;claim.bytes=requested;
 }
 let mut storage=Vec::new();if !allocate(&mut storage){return Err(fail(ValueRefusalKind::AllocationFailed,"shared source cell reserve failed",0));}
 let physical=storage.capacity().checked_mul(requested).ok_or_else(||fail(ValueRefusalKind::OwnershipLimit,"shared source capacity overflow",0))?;
 if storage.capacity()<1||!storage.is_empty(){return Err(fail(ValueRefusalKind::InvariantViolated,"shared source allocator violated empty-cell reservation",physical));}
 {
  let mut admission=context.admission().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
  let extra=physical-requested;
  if extra>admission.maximum_bytes.saturating_sub(admission.committed_bytes){return Err(fail(ValueRefusalKind::OwnershipLimit,"shared source actual capacity exceeds remaining credit",physical));}
  admission.committed_bytes+=extra;claim.bytes=physical;
 }
 let reservation=SharedTransportReservation{cell:Some(ReservedCell{storage,allocated_bytes:physical,category,retry}),claim};
 context.checkpoint(PackTransportPhase::AfterReserve,category,physical)?;
 context.report(PackTransportPhase::ReservationCommitted,Some(category),physical);
 Ok(reservation)
}

unsafe fn admission_ptr<P>(pointer:NonNull<()>)->*const Mutex<TransportAdmission>{unsafe{&pointer.cast::<ContextCell<P>>().as_ref().admission}}
unsafe fn canceled_ptr<P>(pointer:NonNull<()>)->*const AtomicBool{unsafe{&pointer.cast::<ContextCell<P>>().as_ref().canceled}}
unsafe fn policy_ptr<P:PackTransportPolicy+'static>(pointer:NonNull<()>)->*const(dyn PackTransportPolicy+'static){unsafe{&pointer.cast::<ContextCell<P>>().as_ref().policy}}
unsafe fn retain<P>(pointer:NonNull<()>){let references=unsafe{&pointer.cast::<ContextCell<P>>().as_ref().references};if references.fetch_add(1,Ordering::Relaxed)>=isize::MAX as usize{std::process::abort();}}
unsafe fn release<P:PackTransportPolicy>(pointer:NonNull<()>,capacity:usize){
 let references=unsafe{&pointer.cast::<ContextCell<P>>().as_ref().references};
 if references.fetch_sub(1,Ordering::Release)==1{
  std::sync::atomic::fence(Ordering::Acquire);
  let storage=unsafe{Vec::from_raw_parts(pointer.cast::<ContextCell<P>>().as_ptr(),1,capacity)};
  let cell=&storage[0];
  let committed=cell.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).committed_bytes;
  if !std::thread::panicking(){cell.policy.progress(PackTransportProgress{phase:PackTransportPhase::ContextClosed,category:None,bytes:capacity*std::mem::size_of::<ContextCell<P>>(),committed_bytes:committed});}
  drop(storage);
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
