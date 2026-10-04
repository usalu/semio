//! 🔌️ Unmounted fallible shared transport cell; native compiler and allocator laws remain pending.
use crate::{PackRetryDisposition,PackTransportCategory};
use semio_framework_value::ValueRefusalKind;
use std::error::Error;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize,Ordering};
#[path="👥️context/🦀️.rs"]
mod shared_context;
pub use shared_context::{PackTransportContext,PackTransportPolicy,PackTransportPhase,PackTransportProgress,TransportContextRefusal,TransportContextRefusalCause,SharedTransportReservation};

/// 🚦️ Observer boundaries finish before a provider can create an external failure.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum TransportCaptureStage{Before,After}

/// 🧾️ Private admission fields prevent observers from changing the reservation snapshot.
pub struct TransportAdmission{maximum_bytes:usize,committed_bytes:usize}
impl TransportAdmission{
 pub const fn new(maximum_bytes:usize,committed_bytes:usize)->Self{Self{maximum_bytes,committed_bytes}}
 pub const fn committed_bytes(&self)->usize{self.committed_bytes}
}

/// 🛑️ Refusal has zero retained bytes and a separate transient physical-reserve witness.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct TransportCaptureRefusal{pub kind:ValueRefusalKind,pub reason:&'static str,pub allocated_bytes:usize,pub physical_bytes:usize}
impl std::fmt::Display for TransportCaptureRefusal{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.reason)}}
impl Error for TransportCaptureRefusal{}

struct TransportCell<E>{references:AtomicUsize,source:E,context:Option<PackTransportContext>}
#[derive(Clone,Copy)]
struct TransportVTable{
 source:unsafe fn(NonNull<()>)->*const(dyn Error+Send+Sync+'static),
 retain:unsafe fn(NonNull<()>),
 release:unsafe fn(NonNull<()>,usize),
}

/// 🪪️ Concrete source identity survives Clone through one already admitted cell.
pub struct OwnedTransportError{pointer:NonNull<()>,capacity:usize,allocated_bytes:usize,category:PackTransportCategory,retry:PackRetryDisposition,vtable:TransportVTable}
unsafe impl Send for OwnedTransportError{}
unsafe impl Sync for OwnedTransportError{}
impl OwnedTransportError{
 pub fn source_error(&self)->&(dyn Error+Send+Sync+'static){unsafe{&*(self.vtable.source)(self.pointer)}}
 pub const fn category(&self)->PackTransportCategory{self.category}
 pub const fn retry(&self)->PackRetryDisposition{self.retry}
 pub const fn allocated_bytes(&self)->usize{self.allocated_bytes}
}
impl Clone for OwnedTransportError{
 fn clone(&self)->Self{unsafe{(self.vtable.retain)(self.pointer)};Self{pointer:self.pointer,capacity:self.capacity,allocated_bytes:self.allocated_bytes,category:self.category,retry:self.retry,vtable:self.vtable}}
}
impl Drop for OwnedTransportError{fn drop(&mut self){unsafe{(self.vtable.release)(self.pointer,self.capacity)}}}
impl PartialEq for OwnedTransportError{fn eq(&self,other:&Self)->bool{self.pointer==other.pointer&&self.category==other.category&&self.retry==other.retry}}
impl Eq for OwnedTransportError{}
impl std::fmt::Display for OwnedTransportError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{std::fmt::Display::fmt(self.source_error(),f)}}
impl std::fmt::Debug for OwnedTransportError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OwnedTransportError").field("category",&self.category).field("retry",&self.retry).field("allocated_bytes",&self.allocated_bytes).field("source",&self.source_error()).finish()}}
impl Error for OwnedTransportError{fn source(&self)->Option<&(dyn Error+'static)>{Some(self.source_error())}}

unsafe fn source_ref<E:Error+Send+Sync+'static>(pointer:NonNull<()>)->*const(dyn Error+Send+Sync+'static){unsafe{&pointer.cast::<TransportCell<E>>().as_ref().source}}
unsafe fn retain<E>(pointer:NonNull<()>){
 let references=unsafe{&pointer.cast::<TransportCell<E>>().as_ref().references};
 if references.fetch_add(1,Ordering::Relaxed)>=isize::MAX as usize{std::process::abort();}
}
unsafe fn release<E>(pointer:NonNull<()>,capacity:usize){
 let references=unsafe{&pointer.cast::<TransportCell<E>>().as_ref().references};
 if references.fetch_sub(1,Ordering::Release)==1{std::sync::atomic::fence(Ordering::Acquire);unsafe{drop(Vec::from_raw_parts(pointer.cast::<TransportCell<E>>().as_ptr(),1,capacity));}}
}

/// 📦️ Dropping an unused reserve releases physical capacity and restores caller admission.
pub struct TransportReservation<'a,E:Error+Send+Sync+'static>{storage:Vec<TransportCell<E>>,admission:&'a mut TransportAdmission,before:usize,allocated_bytes:usize,category:PackTransportCategory,retry:PackRetryDisposition,published:bool,marker:PhantomData<E>}
struct ReservedCell<E>{storage:Vec<TransportCell<E>>,allocated_bytes:usize,category:PackTransportCategory,retry:PackRetryDisposition}
impl<E:Error+Send+Sync+'static> ReservedCell<E>{
 fn publish(mut self,source:E,context:Option<PackTransportContext>)->OwnedTransportError{
  self.storage.push(TransportCell{references:AtomicUsize::new(1),source,context});
  let capacity=self.storage.capacity();let pointer=NonNull::new(self.storage.as_mut_ptr()).unwrap().cast();
  std::mem::forget(self.storage);
  OwnedTransportError{pointer,capacity,allocated_bytes:self.allocated_bytes,category:self.category,retry:self.retry,vtable:TransportVTable{source:source_ref::<E>,retain:retain::<E>,release:release::<E>}}
 }
}
impl<E:Error+Send+Sync+'static> TransportReservation<'_,E>{
 pub const fn allocated_bytes(&self)->usize{self.allocated_bytes}
 pub fn publish(mut self,source:E)->OwnedTransportError{
  let cell=ReservedCell{storage:std::mem::take(&mut self.storage),allocated_bytes:self.allocated_bytes,category:self.category,retry:self.retry};
  self.published=true;cell.publish(source,None)
 }
}
impl<E:Error+Send+Sync+'static> Drop for TransportReservation<'_,E>{fn drop(&mut self){if !self.published{self.admission.committed_bytes=self.before;}}}

/// 🧱️ Reserves the actual concrete cell before invoking any provider IO operation.
pub fn reserve_transport<E:Error+Send+Sync+'static,F:FnMut(TransportCaptureStage,usize)->bool>(admission:&mut TransportAdmission,category:PackTransportCategory,retry:PackRetryDisposition,observe:F)->Result<TransportReservation<'_,E>,TransportCaptureRefusal>{
 reserve_with(admission,category,retry,observe,|storage|storage.try_reserve_exact(1).is_ok())
}

fn reserve_with<E:Error+Send+Sync+'static,F:FnMut(TransportCaptureStage,usize)->bool,A:FnOnce(&mut Vec<TransportCell<E>>)->bool>(admission:&mut TransportAdmission,category:PackTransportCategory,retry:PackRetryDisposition,mut observe:F,allocate:A)->Result<TransportReservation<'_,E>,TransportCaptureRefusal>{
 let before=admission.committed_bytes;let requested=std::mem::size_of::<TransportCell<E>>();
 let refuse=|kind,reason,physical_bytes|TransportCaptureRefusal{kind,reason,allocated_bytes:0,physical_bytes};
 let remaining=admission.maximum_bytes.checked_sub(before).ok_or_else(||refuse(ValueRefusalKind::OwnershipLimit,"transport admission baseline exceeds maximum",0))?;
 if requested>isize::MAX as usize||requested>remaining{return Err(refuse(ValueRefusalKind::OwnershipLimit,"transport cell exceeds owned physical ceiling",0));}
 if !observe(TransportCaptureStage::Before,requested){return Err(refuse(ValueRefusalKind::Canceled,"transport reservation canceled before allocation",0));}
 let mut storage=Vec::new();
 if !allocate(&mut storage){return Err(refuse(ValueRefusalKind::AllocationFailed,"transport cell reserve failed",0));}
 let physical=storage.capacity().checked_mul(requested).ok_or_else(||refuse(ValueRefusalKind::OwnershipLimit,"transport physical capacity overflow",0))?;
 if storage.capacity()<1||!storage.is_empty(){return Err(refuse(ValueRefusalKind::InvariantViolated,"transport allocator violated empty-cell reservation",physical));}
 if physical>remaining{return Err(refuse(ValueRefusalKind::OwnershipLimit,"transport actual capacity exceeds owned physical ceiling",physical));}
 if !observe(TransportCaptureStage::After,physical){return Err(refuse(ValueRefusalKind::Canceled,"transport reservation canceled after allocation",physical));}
 admission.committed_bytes=before+physical;
 Ok(TransportReservation{storage,admission,before,allocated_bytes:physical,category,retry,published:false,marker:PhantomData})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
