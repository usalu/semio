//! 🎟️ A parent retains genuine returned flat allocations until their complete physical release grant.
use crate::{ValueError,ValueRefusalKind};
use std::{alloc::Layout,mem::ManuallyDrop,ptr::NonNull};

struct ReturnedAllocation{pointer:NonNull<u8>,layout:Layout}
trait AllocationRelease{unsafe fn release(pointer:NonNull<u8>,layout:Layout);}
struct GlobalAllocationRelease;
impl AllocationRelease for GlobalAllocationRelease{unsafe fn release(pointer:NonNull<u8>,layout:Layout){unsafe{std::alloc::dealloc(pointer.as_ptr(),layout)}}}
/// 🧷️ Tokens contain one unique global allocation and no live typed elements; its allocator permits release on any thread.
unsafe impl Send for ReturnedAllocation{}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum AllocationReturnStep{Pending{released_items:usize,released_bytes:usize},Complete}

/// 🏠️ Finite inline parent responsibility holds returned allocations without allocating another queue.
pub struct ParentAllocationReturn<const N:usize>{slots:[Option<ReturnedAllocation>;N],maximum_allocation_bytes:usize,maximum_total_bytes:usize,retained_bytes:usize}
impl<const N:usize> ParentAllocationReturn<N>{
    /// 🛂️ Installs the actual parent's per-allocation and cumulative return authority without allocating.
    pub fn try_new(maximum_allocation_bytes:usize,maximum_total_bytes:usize)->Result<Self,ValueError>{
        if N==0||maximum_allocation_bytes==0||maximum_total_bytes<maximum_allocation_bytes{return Err(refusal(ValueRefusalKind::OwnershipLimit,"parent allocation return authority is invalid"));}
        Ok(Self{slots:std::array::from_fn(|_|None),maximum_allocation_bytes,maximum_total_bytes,retained_bytes:0})
    }
    /// 📊️ Reports complete allocations still physically owned by this parent.
    pub fn retained_bytes(&self)->usize{self.retained_bytes}
    /// 👓️ Tests physical terminal-empty independently from logical child handback.
    pub fn terminal_is_empty(&self)->bool{self.slots.iter().all(Option::is_none)}
    /// 🎟️ Receives only an empty typed Vec's genuine global allocation; refusal leaves its owner unchanged.
    pub fn return_empty_vec<T>(&mut self,owner:&mut Vec<T>,maximum_items:usize)->Result<bool,ValueError>{
        if maximum_items==0{return Ok(false);}if !owner.is_empty(){return Err(refusal(ValueRefusalKind::InvariantViolated,"parent can only receive vector backing after every typed element is returned"));}
        let layout=Layout::array::<T>(owner.capacity()).map_err(|_|refusal(ValueRefusalKind::OwnershipLimit,"returned vector allocation layout is invalid"))?;
        if layout.size()==0{*owner=Vec::new();return Ok(true);}let Some(slot)=self.admit(layout)?else{return Ok(false)};
        let mut allocation=ManuallyDrop::new(std::mem::take(owner));let pointer=NonNull::new(allocation.as_mut_ptr().cast()).expect("allocated vector backing is nonnull");
        self.slots[slot]=Some(ReturnedAllocation{pointer,layout});self.retained_bytes+=layout.size();Ok(true)
    }
    /// 🧵️ Returns a uniquely owned String allocation after retiring its byte-free logical text cell.
    pub fn return_text(&mut self,owner:&mut String,maximum_items:usize)->Result<bool,ValueError>{
        if maximum_items==0{return Ok(false);}let layout=Layout::array::<u8>(owner.capacity()).map_err(|_|refusal(ValueRefusalKind::OwnershipLimit,"returned text allocation layout is invalid"))?;
        if layout.size()==0{owner.clear();return Ok(true);}let Some(slot)=self.admit(layout)?else{return Ok(false)};
        let mut allocation=ManuallyDrop::new(std::mem::take(owner).into_bytes());let pointer=NonNull::new(allocation.as_mut_ptr()).expect("allocated text backing is nonnull");
        self.slots[slot]=Some(ReturnedAllocation{pointer,layout});self.retained_bytes+=layout.size();Ok(true)
    }
    fn admit(&self,layout:Layout)->Result<Option<usize>,ValueError>{
        if layout.size()>self.maximum_allocation_bytes||self.retained_bytes.checked_add(layout.size()).is_none_or(|bytes|bytes>self.maximum_total_bytes){return Err(refusal(ValueRefusalKind::OwnershipLimit,"returned allocation exceeds actual parent responsibility"));}
        Ok(self.slots.iter().position(Option::is_none))
    }
    /// 📏️ Exposes the next whole physical allocation without spending the caller's grant.
    pub fn next_close_byte_demand(&self)->usize{self.slots.iter().find_map(|slot|slot.as_ref().map(|allocation|allocation.layout.size())).unwrap_or(0)}
    /// ♻️ Deallocates at most one genuine backing allocation under its complete Layout byte grant.
    pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->AllocationReturnStep{
        self.close_with::<GlobalAllocationRelease>(maximum_items,maximum_bytes)
    }
    fn close_with<A:AllocationRelease>(&mut self,maximum_items:usize,maximum_bytes:usize)->AllocationReturnStep{
        if maximum_items==0||maximum_bytes==0{return AllocationReturnStep::Pending{released_items:0,released_bytes:0};}
        let Some(index)=self.slots.iter().position(Option::is_some)else{return AllocationReturnStep::Complete};
        let bytes=self.slots[index].as_ref().unwrap().layout.size();if bytes>maximum_bytes{return AllocationReturnStep::Pending{released_items:0,released_bytes:0};}
        let allocation=self.slots[index].take().unwrap();unsafe{A::release(allocation.pointer,allocation.layout)};self.retained_bytes-=bytes;
        AllocationReturnStep::Pending{released_items:1,released_bytes:bytes}
    }
}
impl<const N:usize> Drop for ParentAllocationReturn<N>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"parent allocation return retains actual physical allocations");}}
fn refusal(kind:ValueRefusalKind,message:&'static str)->ValueError{ValueError::new(kind,message)}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
