//! 🧪️ Owned graph continuations retain actual source custody through relocation, denial and paid close.
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot,validation::{graph::LayoutGraphRead,retained::owned::{OwnedLayoutGraphRead,OwnedLayoutGraphValidation}}};
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep}};
use std::sync::{Arc,atomic::{AtomicBool,AtomicUsize,Ordering}};
#[derive(Default)]
struct Events{sequence:AtomicUsize,begin:AtomicUsize,terminal:AtomicUsize,dropped:AtomicUsize,closing:AtomicBool,address:AtomicUsize,source_release:AtomicUsize,identity_checks:AtomicUsize,reads_after_close:AtomicUsize}
struct Reader{source:RetainedCloneSource<LayoutSnapshot>,events:Arc<Events>}
impl Reader{
 fn new(base:&serde_json::Value,duplicate:bool)->(Self,Arc<Events>){let mut document:LayoutSnapshot=semio_framework_pack_json::from_json_str(&base.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();if duplicate{document.stories[1].id=document.stories[0].id.clone()}let(source,_)=RetainedCloneSource::admit_owned(document,(),generous()).unwrap_or_else(|_|panic!("genuine original reader admission"));let events=Arc::new(Events::default());events.address.store(source.view()as*const _ as usize,Ordering::Relaxed);(Self{source,events:events.clone()},events)}
 fn identity(&self){self.events.identity_checks.fetch_add(1,Ordering::Relaxed);assert_eq!(self.source.view()as*const _ as usize,self.events.address.load(Ordering::Relaxed))}
}
impl OwnedLayoutGraphRead for Reader{
 fn graph(&self)->&dyn LayoutGraphRead{if self.events.closing.load(Ordering::Relaxed){self.events.reads_after_close.fetch_add(1,Ordering::Relaxed);}assert!(!self.events.closing.load(Ordering::Relaxed));self.identity();self.source.view()}
 fn begin_close(&mut self){self.identity();assert!(!self.events.closing.swap(true,Ordering::Relaxed));self.events.begin.store(self.events.sequence.fetch_add(1,Ordering::Relaxed)+1,Ordering::Relaxed)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.source.next_close_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let p=self.source.close_step(g)?;self.events.source_release.fetch_add(p.progress().released_bytes,Ordering::Relaxed);if self.source.terminal_is_empty(){self.events.terminal.store(self.events.sequence.fetch_add(1,Ordering::Relaxed)+1,Ordering::Relaxed)}Ok(p)}
 fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()}
}
impl Drop for Reader{fn drop(&mut self){assert!(std::thread::panicking()||self.source.terminal_is_empty());self.events.dropped.store(self.events.sequence.fetch_add(1,Ordering::Relaxed)+1,Ordering::Relaxed)}}
fn generous()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:16*1024*1024,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:256}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn receipt(g:RetainedCloneGrant,p:RetainedCloneProgress,heap:(usize,usize)){assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes))}
fn event(ordinal:&mut usize,maximum:usize){assert!(*ordinal<maximum,"bounded actual ownership observations");*ordinal+=1;}
#[test]
fn owned_graph_reader_moves_across_denied_live_and_cancelled_continuations(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let maximum=fixture["maxObservedTurns"].as_u64().unwrap()as usize;
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/✅️validation/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let mut ordinal=0;let mut birth_denied=0;let(reader,events)=Reader::new(&corpus["base"],row["duplicateStory"].as_bool().unwrap());let mut reader=reader;let mut owner=OwnedLayoutGraphValidation::new();let birth=OwnedLayoutGraphValidation::reader_birth_demand::<Reader>();
  for axis in fixture["birthDenials"].as_array().unwrap(){let mut denied=grant(birth);match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"copy"=>denied.maximum_copy_bytes-=1,"capacity"=>denied.maximum_capacity_bytes-=1,"depth"=>denied.maximum_depth-=1,_=>unreachable!()}let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.start(reader,denied));assert_eq!(heap,(0,0));reader=result.err().unwrap().1;birth_denied+=1;assert_eq!(events.sequence.load(Ordering::Relaxed),0);event(&mut ordinal,maximum)}
  let g=grant(birth);let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.start(reader,g));let p=result.unwrap_or_else(|_|panic!("paid actual reader birth"));receipt(g,p,heap);let mut capacity=p.retained_capacity_bytes;event(&mut ordinal,maximum);
  let(mut refused,_)=Reader::new(&corpus["base"],false);let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.start(refused,g));assert_eq!(heap,(0,0));refused=result.err().unwrap().1;event(&mut ordinal,maximum);refused.begin_close();while !refused.terminal_is_empty(){let d=refused.close_demand(generous().maximum_copy_bytes).unwrap();refused.close_step(grant(d)).unwrap()}drop(refused);
  let stop=row["stop"].as_str().unwrap();let mut places=[Some(owner),None];let mut current=1;places[1]=places[0].take();let mut relocations=1;let mut completed=false;let mut valid=None;let mut program_births=0;let mut turn_count=0;
  if stop!="birth"{loop{turn_count+=1;assert!(turn_count<maximum);let owner=places[current].as_mut().unwrap();let d=owner.demand(generous().maximum_copy_bytes).unwrap();let mut denied=grant(d);denied.maximum_items=0;let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.advance(denied.maximum_copy_bytes,denied));let result=result.unwrap();receipt(denied,result.progress,heap);assert_eq!(result.progress,Default::default());event(&mut ordinal,maximum);let g=grant(d);let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.advance(g.maximum_copy_bytes,g));let result=result.unwrap();receipt(g,result.progress,heap);capacity+=result.progress.retained_capacity_bytes;let born=d.depth==1&&d.capacity_bytes>0;if born{program_births+=1}if result.done{completed=true;valid=Some(result.error.is_none())}event(&mut ordinal,maximum);let should_stop=completed||stop=="index"&&turn_count==1||stop=="program-birth"&&born||stop=="program-pending"&&program_births>0&&!born;let value=places[current].take();current=1-current;assert!(places[current].is_none());places[current]=value;relocations+=1;if should_stop{break}}}
  assert_eq!(completed,row["completed"].as_bool().unwrap());assert_eq!(valid,row["valid"].as_bool());if stop.starts_with("program-"){assert_eq!(program_births,1);assert!(!completed)}
  let owner=places[current].as_mut().unwrap();owner.begin_close();let mut released=0;let mut denied_close=0;
  while !owner.terminal_is_empty(){let d=owner.close_demand(generous().maximum_copy_bytes).unwrap();let mut denied=grant(d);denied.maximum_items=0;let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.close_step(denied));let p=result.unwrap().progress();receipt(denied,p,heap);assert_eq!(p,Default::default());denied_close+=1;event(&mut ordinal,maximum);let g=grant(d);let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.close_step(g));let p=result.unwrap().progress();receipt(g,p,heap);released+=p.released_bytes;event(&mut ordinal,maximum)}
  let source_released=events.source_release.load(Ordering::Relaxed);assert_eq!(released-source_released,capacity);let order=[events.begin.load(Ordering::Relaxed),events.terminal.load(Ordering::Relaxed),events.dropped.load(Ordering::Relaxed)];assert_eq!(order,[1,2,3]);assert!(denied_close>0);
  let identity_checks=events.identity_checks.load(Ordering::Relaxed);let reads_after_close=events.reads_after_close.load(Ordering::Relaxed);
  
 }
}
