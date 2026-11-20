//! 📇️ Original borrowed identifiers resolve through a funded scalar collision chain without copied strings.
use semio_framework_value_persistent::list::{PagedListReleaseFailure,PagedListReleaseOutcome,PagedListReservationFailure,PagedListReservationOutcome};
use semio_framework_value::{RetainedAllocationEffect,ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneProgress,numeric_scratch::NumericIndex};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct MetadataKey{pub kind:u8,pub a:usize,pub b:usize}
pub trait MetadataSource{fn identity(&self)->usize;fn text(&self,key:MetadataKey)->Result<&str,&'static str>;}
#[derive(Clone,Copy)]
struct Entry{key:MetadataKey,domain:u8,scope:usize,value:usize,next:Option<usize>}
#[derive(Clone,Copy)]
struct Task{key:MetadataKey,domain:u8,scope:usize,insert:Option<usize>,hash:u32,at:usize,phase:u8,candidate:Option<usize>,compared:usize}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum MetadataLookup{Pending,Ready(Option<usize>)}
pub struct MetadataIndex{buckets:NumericIndex<u32,usize>,entries:NumericIndex<usize,Entry>,task:Option<Task>,source:Option<usize>,cancelled:bool,last:RetainedCloneProgress}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn mix(hash:u32,byte:u8)->u32{(hash^u32::from(byte)).wrapping_mul(16777619)}
fn seed(domain:u8,scope:usize)->u32{let mut hash=mix(2166136261,domain);for byte in (scope as u64).to_le_bytes(){hash=mix(hash,byte)}hash}
impl MetadataIndex{
 pub fn new()->Self{Self{buckets:NumericIndex::new(),entries:NumericIndex::new(),task:None,source:None,cancelled:false,last:Default::default()}}
 pub fn active(&self)->bool{self.task.is_some()}
 pub fn last_receipt(&self)->RetainedCloneProgress{self.last}
 fn insertion_bytes()->usize{std::mem::size_of::<(usize,Entry)>()+std::mem::size_of::<(u32,usize)>()}
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{let mut demand=RetirementDemand{depth:1,..Default::default()};let Some(task)=self.task else{return Ok(demand)};if task.phase==0||task.candidate.is_some()||task.insert.is_none(){return Ok(demand)}demand=self.entries.next_reserve_demand()?;if demand.depth==0&&self.buckets.get(&task.hash).is_none(){demand=self.buckets.next_reserve_demand()?;}if demand.depth==0{demand.depth=1;demand.copy_bytes=Self::insertion_bytes();}Ok(demand)}
 pub fn query<S:MetadataSource+?Sized>(&mut self,source:&S,key:MetadataKey,domain:u8,scope:usize,insert:Option<usize>,effect:&mut RetainedAllocationEffect<'_>)->Result<MetadataLookup,ValueError>{
  let grant=effect.grant();
  self.last=Default::default();if self.cancelled{return Err(ValueError::literal(ValueRefusalKind::Canceled,"layout metadata index is cancelled"));}let demand=self.next_demand()?;if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(MetadataLookup::Pending);}if self.source.is_some_and(|identity|identity!=source.identity()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout metadata source changed"));}self.source=Some(source.identity());
  let mut task=self.task.take().unwrap_or(Task{key,domain,scope,insert,hash:seed(domain,scope),at:0,phase:0,candidate:None,compared:0});if task.key!=key||task.domain!=domain||task.scope!=scope||task.insert!=insert{self.task=Some(task);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout metadata query changed"));}self.last.copied_items=1;let value=source.text(task.key).map_err(invalid)?;
  if task.phase==0{if let Some(point)=value[task.at..].chars().next(){let mut bytes=[0;4];for byte in point.encode_utf8(&mut bytes).as_bytes(){task.hash=mix(task.hash,*byte)}task.at+=point.len_utf8();}else{task.candidate=self.buckets.get(&task.hash).copied();task.phase=1;}self.task=Some(task);return Ok(MetadataLookup::Pending);}
  if let Some(at)=task.candidate{let entry=*self.entries.get(&at).ok_or_else(||invalid("layout metadata entry is missing"))?;let candidate=source.text(entry.key).map_err(invalid)?;if entry.domain!=domain||entry.scope!=scope||candidate.len()!=value.len(){task.candidate=entry.next;task.compared=0;}else if let Some(point)=value[task.compared..].chars().next(){if candidate[task.compared..].chars().next()!=Some(point){task.candidate=entry.next;task.compared=0;}else{task.compared+=point.len_utf8();}}else{if insert.is_some(){return Err(invalid("layout metadata identifier is duplicated"));}return Ok(MetadataLookup::Ready(Some(entry.value)));}self.task=Some(task);return Ok(MetadataLookup::Pending);}
  let Some(value)=insert else{return Ok(MetadataLookup::Ready(None))};
  let entry=self.entries.next_reserve_demand()?.depth;let bucket=if self.buckets.get(&task.hash).is_none(){self.buckets.next_reserve_demand()?.depth}else{0};if entry>0||bucket>0{let result=if entry>0{self.entries.reserve_one_funded(effect)}else{self.buckets.reserve_one_funded(effect)};self.task=Some(task);self.last=Default::default();post_reservation(result.result,&mut self.last)?;return Ok(MetadataLookup::Pending);}
  if grant.maximum_copy_bytes<Self::insertion_bytes(){self.task=Some(task);return Ok(MetadataLookup::Pending);}let at=self.entries.stored_keys();self.entries.insert_reserved(at,Entry{key:task.key,domain,scope,value,next:self.buckets.get(&task.hash).copied()}).map_err(|_|invalid("layout metadata entry reservation failed"))?;self.buckets.insert_reserved(task.hash,at).map_err(|_|invalid("layout metadata bucket reservation failed"))?;self.last.copied_bytes=Self::insertion_bytes();Ok(MetadataLookup::Ready(Some(value)))
 }
 pub fn cancel(&mut self){self.cancelled=true;self.task=None;self.source=None;}
 pub fn next_close_demand(&self)->Result<RetirementDemand,ValueError>{
  Ok(if !self.entries.terminal_is_empty(){self.entries.next_close_demand()?}else{self.buckets.next_close_demand()?})
 }
 pub fn close_one(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
  self.cancel();self.last=Default::default();
  let copy=self.next_close_demand()?.copy_bytes;
  let result=if !self.entries.terminal_is_empty(){self.entries.close_copy_step(effect)}else{self.buckets.close_copy_step(effect)};
  self.last=post_release(result.result,copy)?;
  Ok(self.last)
 }
 pub fn terminal_is_empty(&self)->bool{self.cancelled&&self.entries.terminal_is_empty()&&self.buckets.terminal_is_empty()}
}
impl Default for MetadataIndex{fn default()->Self{Self::new()}}
semio_framework_value::artifact_retire_leaf!(MetadataKey,Entry,Task);
semio_framework_value::artifact_retire_struct!(MetadataIndex{buckets,entries,task,source,cancelled,last});


pub fn post_reservation(result:Result<PagedListReservationOutcome<RetainedCloneProgress>,PagedListReservationFailure>,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError>{
    match result{
        Ok(PagedListReservationOutcome::Denied)=>Ok(false),
        Ok(PagedListReservationOutcome::Ready(progress))=>{
            *receipt=receipt.checked_add(progress)?;
            Ok(true)
        },
        Err(PagedListReservationFailure::Work(error))=>Err(error),
        Err(PagedListReservationFailure::Allocation{error,copied_bytes})=>{
            let progress=RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0||copied_bytes!=0),copied_bytes,retained_capacity_bytes:error.allocated_bytes,..Default::default()};
            *receipt=receipt.checked_add(progress)?;
            Err(ValueError::from(error.refusal()).with_retained_progress(*receipt))
        },
    }
}

pub(super) fn post_release(result:Result<PagedListReleaseOutcome,PagedListReleaseFailure>,copy:usize)->Result<RetainedCloneProgress,ValueError>{
    match result{
        Ok(PagedListReleaseOutcome::Ready(progress))=>Ok(RetainedCloneProgress{
            copied_items:usize::from(progress.progressed),
            copied_bytes:if progress.progressed{copy}else{0},
            released_bytes:progress.released_allocation_bytes,
            ..Default::default()
        }),
        Ok(PagedListReleaseOutcome::Denied)=>Ok(Default::default()),
        Err(PagedListReleaseFailure::Work(error))=>Err(error),
        Err(PagedListReleaseFailure::Release(error))=>Err(error.into()),
    }
}

/// ⏱️ Records one bounded CPU step when a nested owner did not already report work for it.
pub fn record_native_step(effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError>{
    if effect.performed_work()==0 && !effect.performed(1)?{
        return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Layout native step exceeded its Work grant"));
    }
    Ok(())
}
