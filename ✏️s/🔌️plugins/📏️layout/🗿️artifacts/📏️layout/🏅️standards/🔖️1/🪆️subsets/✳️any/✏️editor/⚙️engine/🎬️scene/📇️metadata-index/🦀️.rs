//! 📇️ Original borrowed identifiers resolve through a funded scalar collision chain without copied strings.
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,numeric_scratch::NumericIndex};
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
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{let mut demand=RetirementDemand{depth:1,..Default::default()};let Some(task)=self.task else{return Ok(demand)};if task.phase==0||task.candidate.is_some()||task.insert.is_none(){return Ok(demand)}demand=self.entries.next_reserve_demand()?;if demand.capacity_bytes==0&&self.buckets.get(&task.hash).is_none(){demand=self.buckets.next_reserve_demand()?;}demand.depth=demand.depth.max(1);if demand.capacity_bytes==0{demand.copy_bytes=Self::insertion_bytes();}Ok(demand)}
 pub fn query<S:MetadataSource+?Sized>(&mut self,source:&S,key:MetadataKey,domain:u8,scope:usize,insert:Option<usize>,grant:RetainedCloneGrant)->Result<MetadataLookup,ValueError>{
  self.last=Default::default();if self.cancelled{return Err(ValueError::literal(ValueRefusalKind::Canceled,"layout metadata index is cancelled"));}let demand=self.next_demand()?;if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(MetadataLookup::Pending);}if self.source.is_some_and(|identity|identity!=source.identity()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout metadata source changed"));}self.source=Some(source.identity());
  let mut task=self.task.take().unwrap_or(Task{key,domain,scope,insert,hash:seed(domain,scope),at:0,phase:0,candidate:None,compared:0});if task.key!=key||task.domain!=domain||task.scope!=scope||task.insert!=insert{self.task=Some(task);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout metadata query changed"));}self.last.copied_items=1;let value=source.text(task.key).map_err(invalid)?;
  if task.phase==0{if let Some(point)=value[task.at..].chars().next(){let mut bytes=[0;4];for byte in point.encode_utf8(&mut bytes).as_bytes(){task.hash=mix(task.hash,*byte)}task.at+=point.len_utf8();}else{task.candidate=self.buckets.get(&task.hash).copied();task.phase=1;}self.task=Some(task);return Ok(MetadataLookup::Pending);}
  if let Some(at)=task.candidate{let entry=*self.entries.get(&at).ok_or_else(||invalid("layout metadata entry is missing"))?;let candidate=source.text(entry.key).map_err(invalid)?;if entry.domain!=domain||entry.scope!=scope||candidate.len()!=value.len(){task.candidate=entry.next;task.compared=0;}else if let Some(point)=value[task.compared..].chars().next(){if candidate[task.compared..].chars().next()!=Some(point){task.candidate=entry.next;task.compared=0;}else{task.compared+=point.len_utf8();}}else{if insert.is_some(){return Err(invalid("layout metadata identifier is duplicated"));}return Ok(MetadataLookup::Ready(Some(entry.value)));}self.task=Some(task);return Ok(MetadataLookup::Pending);}
  let Some(value)=insert else{return Ok(MetadataLookup::Ready(None))};
  let entry=self.entries.next_allocation_bytes()?;let bucket=if self.buckets.get(&task.hash).is_none(){self.buckets.next_allocation_bytes()?}else{0};if entry>0||bucket>0{let result=if entry>0{self.entries.reserve_one_funded(grant)}else{self.buckets.reserve_one_funded(grant)};self.task=Some(task);match result{Ok(progress)=>{self.last.retained_capacity_bytes=progress.retained_capacity_bytes;self.last.copied_bytes=progress.copied_bytes;},Err(error)=>{self.last.retained_capacity_bytes=error.allocated_bytes;return Err(ValueError::from(error.refusal()));}}return Ok(MetadataLookup::Pending);}
  if grant.maximum_copy_bytes<Self::insertion_bytes(){self.task=Some(task);return Ok(MetadataLookup::Pending);}let at=self.entries.stored_keys();self.entries.insert_reserved(at,Entry{key:task.key,domain,scope,value,next:self.buckets.get(&task.hash).copied()}).map_err(|_|invalid("layout metadata entry reservation failed"))?;self.buckets.insert_reserved(task.hash,at).map_err(|_|invalid("layout metadata bucket reservation failed"))?;self.last.copied_bytes=Self::insertion_bytes();Ok(MetadataLookup::Ready(Some(value)))
 }
 pub fn cancel(&mut self){self.cancelled=true;self.task=None;self.source=None;}
 pub fn next_close_copy_bytes(&self)->usize{if !self.entries.terminal_is_empty(){self.entries.next_close_copy_byte_demand()}else{self.buckets.next_close_copy_byte_demand()}}
 pub fn next_close_release_bytes(&self)->Result<usize,ValueError>{Ok(if !self.entries.terminal_is_empty(){self.entries.next_close_release_byte_demand()?}else{self.buckets.next_close_release_byte_demand()?})}
 pub fn close_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.cancel();let copy=self.next_close_copy_bytes();let progress=if !self.entries.terminal_is_empty(){self.entries.close_copy_step_funded(grant.maximum_items,grant.maximum_copy_bytes,grant.maximum_release_bytes)}else{self.buckets.close_copy_step_funded(grant.maximum_items,grant.maximum_copy_bytes,grant.maximum_release_bytes)}?;self.last=RetainedCloneProgress{copied_items:usize::from(progress.progressed),copied_bytes:if progress.progressed{copy}else{0},retained_capacity_bytes:0,released_bytes:progress.released_allocation_bytes};Ok(self.last)}
 pub fn terminal_is_empty(&self)->bool{self.cancelled&&self.entries.terminal_is_empty()&&self.buckets.terminal_is_empty()}
}
impl Default for MetadataIndex{fn default()->Self{Self::new()}}
semio_framework_value::artifact_retire_leaf!(MetadataKey,Entry,Task);
semio_framework_value::artifact_retire_struct!(MetadataIndex{buckets,entries,task,source,cancelled,last});
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
