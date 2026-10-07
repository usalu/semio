/// 🔬️ Separate allocator events prevent net shrink from hiding retained or replaced backing.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct HeapAllocationObservation{pub requested_bytes:usize,pub released_bytes:usize,pub largest_release_bytes:usize,pub overflowed:bool}
impl HeapAllocationObservation{
    fn add(mut self,requested:usize,released:usize)->Self{match self.requested_bytes.checked_add(requested){Some(value)=>self.requested_bytes=value,None=>self.overflowed=true}match self.released_bytes.checked_add(released){Some(value)=>self.released_bytes=value,None=>self.overflowed=true}self.largest_release_bytes=self.largest_release_bytes.max(released);self}
    fn merge(self,other:Self)->Self{let mut value=self.add(other.requested_bytes,other.released_bytes);value.largest_release_bytes=self.largest_release_bytes.max(other.largest_release_bytes);value.overflowed|=other.overflowed;value}
}
thread_local!{static THREAD_ALLOCATION_EVENTS:std::cell::Cell<Option<HeapAllocationObservation>>=const{std::cell::Cell::new(None)};}
fn record_allocation_event(requested:usize,released:usize){let _=THREAD_ALLOCATION_EVENTS.try_with(|slot|{if let Some(value)=slot.get(){slot.set(Some(value.add(requested,released)));}});}
/// 🧪️ Captures successful system allocation events on the caller thread without allocating a recorder.
pub fn observe_heap_allocations_on_this_thread<T>(operation:impl FnOnce()->T)->(T,HeapAllocationObservation){
    struct Restore(Option<HeapAllocationObservation>);
    impl Drop for Restore{fn drop(&mut self){let _=THREAD_ALLOCATION_EVENTS.try_with(|slot|{let current=slot.replace(None).unwrap_or_default();slot.set(self.0.map(|prior|prior.merge(current)));});}}
    let restore=Restore(THREAD_ALLOCATION_EVENTS.with(|slot|slot.replace(Some(HeapAllocationObservation::default()))));let result=operation();let observation=THREAD_ALLOCATION_EVENTS.with(|slot|slot.get().expect("allocator observation scope remains installed"));drop(restore);(result,observation)
}
