//! 🧪️ Owned graph continuations retain actual source custody through relocation, denial and paid close.
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot,validation::{graph::LayoutGraphRead,retained::owned::{OwnedLayoutGraphRead,OwnedLayoutGraphValidation}}};
use super::{FixtureSource,original_turn};
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkPoll,RetirementDemand,ValueError,retained_clone::{RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};
use std::sync::{Arc,atomic::{AtomicBool,AtomicUsize,Ordering}};
#[derive(Default)]
struct Events{sequence:AtomicUsize,begin:AtomicUsize,terminal:AtomicUsize,dropped:AtomicUsize,closing:AtomicBool,address:AtomicUsize,source_release:AtomicUsize,identity_checks:AtomicUsize,reads_after_close:AtomicUsize}
struct Reader{source:&'static LayoutSnapshot,binding:Option<RetainedCloneBinding>,events:Arc<Events>}
fn fixture(base:&serde_json::Value,duplicate:bool)->FixtureSource<LayoutSnapshot>{
 let mut document:LayoutSnapshot=semio_framework_pack_json::from_json_str(&base.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 if duplicate{document.stories[1].id=document.stories[0].id.clone()}
 FixtureSource::from_owner(document)
}
impl Reader{
 fn new(source:RetainedCloneRef<'_,LayoutSnapshot>)->(Self,Arc<Events>){
  let mut binding=None;
  original_turn(generous(),|p|*p,|effect|source.bind(&mut binding,effect).result.map(|p|p.unwrap_or_default())).unwrap();
  assert!(binding.is_some());
  let address=std::ptr::from_ref(source.get());
  let source=unsafe{&*address};
  let events=Arc::new(Events::default());
  events.address.store(address as usize,Ordering::Relaxed);
  (Self{source,binding,events:events.clone()},events)
 }
 fn identity(&self){self.events.identity_checks.fetch_add(1,Ordering::Relaxed);assert_eq!(std::ptr::from_ref(self.source) as usize,self.events.address.load(Ordering::Relaxed))}
}
impl OwnedLayoutGraphRead for Reader{
 fn graph(&self)->&dyn LayoutGraphRead{if self.events.closing.load(Ordering::Relaxed){self.events.reads_after_close.fetch_add(1,Ordering::Relaxed);}assert!(!self.events.closing.load(Ordering::Relaxed));self.identity();self.source}
 fn begin_close(&mut self){self.identity();assert!(!self.events.closing.swap(true,Ordering::Relaxed));self.events.begin.store(self.events.sequence.fetch_add(1,Ordering::Relaxed)+1,Ordering::Relaxed)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let RetainedWorkPoll::Complete(turn)=RetainedCloneBinding::close_one(&mut self.binding,body,effect) else{panic!("fixture original source must outlive its reader alias")};
  let p=turn.result.map_err(|error|error.into_value_error())?;
  self.events.source_release.fetch_add(p.progress().released_bytes,Ordering::Relaxed);
  if self.terminal_is_empty(){self.events.terminal.store(self.events.sequence.fetch_add(1,Ordering::Relaxed)+1,Ordering::Relaxed)}
  Ok(p)
 }
 fn terminal_is_empty(&self)->bool{self.binding.is_none()}
}
impl Drop for Reader{fn drop(&mut self){assert!(std::thread::panicking()||self.binding.is_none());self.events.dropped.store(self.events.sequence.fetch_add(1,Ordering::Relaxed)+1,Ordering::Relaxed)}}
fn generous()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:16*1024*1024,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:256}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn start(owner:&mut OwnedLayoutGraphValidation,reader:&mut Option<Reader>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
 original_turn(g,|p|*p,|effect| owner.start(reader.take().unwrap(),effect).map_err(|(error,held)|{*reader=Some(held);error}))
}
fn event(ordinal:&mut usize,maximum:usize){assert!(*ordinal<maximum,"bounded actual ownership observations");*ordinal+=1;}
#[test]
fn owned_graph_reader_moves_across_denied_live_and_cancelled_continuations(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let maximum=fixture["maxObservedTurns"].as_u64().unwrap()as usize;
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/✅️validation/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let mut ordinal=0;let mut birth_denied=0;let mut source=fixture(&corpus["base"],row["duplicateStory"].as_bool().unwrap());let(reader,events)=Reader::new(source.borrow());let mut reader=Some(reader);let mut owner=OwnedLayoutGraphValidation::new();let birth=OwnedLayoutGraphValidation::reader_birth_demand::<Reader>();
  for axis in fixture["birthDenials"].as_array().unwrap(){let mut denied=grant(birth);match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"copy"=>denied.maximum_copy_bytes-=1,"capacity"=>denied.maximum_capacity_bytes-=1,"depth"=>denied.maximum_depth-=1,_=>unreachable!()}assert!(start(&mut owner,&mut reader,denied).is_err());assert!(reader.is_some());birth_denied+=1;assert_eq!(events.sequence.load(Ordering::Relaxed),0);event(&mut ordinal,maximum)}
  let g=grant(birth);let p=start(&mut owner,&mut reader,g).unwrap();assert!(reader.is_none());let mut capacity=p.retained_capacity_bytes;event(&mut ordinal,maximum);
  let mut other=fixture(&corpus["base"],false);let(refused,_)=Reader::new(other.borrow());let mut refused=Some(refused);
  assert!(start(&mut owner,&mut refused,g).is_err());
  let mut refused=refused.unwrap();event(&mut ordinal,maximum);refused.begin_close();
  while !refused.terminal_is_empty(){let body=generous().maximum_copy_bytes;let d=refused.close_demand(body).unwrap();original_turn(grant(d),|p|p.progress(),|effect|refused.close_step(body,effect)).unwrap();}
  drop(refused);while !other.terminal_is_empty(){other.close_once().unwrap();}
  let stop=row["stop"].as_str().unwrap();let mut places=[Some(owner),None];let mut current=1;places[1]=places[0].take();let mut relocations=1;let mut completed=false;let mut valid=None;let mut program_births=0;let mut turn_count=0;
  if stop!="birth"{loop{turn_count+=1;assert!(turn_count<maximum);let owner=places[current].as_mut().unwrap();let d=owner.demand(generous().maximum_copy_bytes).unwrap();let mut denied=grant(d);denied.maximum_items=0;let result=original_turn(denied,|p|p.progress,|effect|owner.advance(generous().maximum_copy_bytes,effect)).unwrap();assert_eq!(result.progress,Default::default());event(&mut ordinal,maximum);let g=grant(d);let result=original_turn(g,|p|p.progress,|effect|owner.advance(generous().maximum_copy_bytes,effect)).unwrap();capacity+=result.progress.retained_capacity_bytes;let born=d.depth==1&&d.capacity_bytes>0;if born{program_births+=1}if result.done{completed=true;valid=Some(result.error.is_none())}event(&mut ordinal,maximum);let should_stop=completed||stop=="index"&&turn_count==1||stop=="program-birth"&&born||stop=="program-pending"&&program_births>0&&!born;let value=places[current].take();current=1-current;assert!(places[current].is_none());places[current]=value;relocations+=1;if should_stop{break}}}
  assert_eq!(completed,row["completed"].as_bool().unwrap());assert_eq!(valid,row["valid"].as_bool());if stop.starts_with("program-"){assert_eq!(program_births,1);assert!(!completed)}
  let owner=places[current].as_mut().unwrap();owner.begin_close();let mut released=0;let mut denied_close=0;
  while !owner.terminal_is_empty(){let d=owner.close_demand(generous().maximum_copy_bytes).unwrap();let mut denied=grant(d);denied.maximum_items=0;let p=original_turn(denied,|p|p.progress(),|effect|owner.close_step(generous().maximum_copy_bytes,effect)).unwrap().progress();assert_eq!(p,Default::default());denied_close+=1;event(&mut ordinal,maximum);let g=grant(d);let p=original_turn(g,|p|p.progress(),|effect|owner.close_step(generous().maximum_copy_bytes,effect)).unwrap().progress();released+=p.released_bytes;event(&mut ordinal,maximum)}
  let source_released=events.source_release.load(Ordering::Relaxed);assert_eq!(released-source_released,capacity);let order=[events.begin.load(Ordering::Relaxed),events.terminal.load(Ordering::Relaxed),events.dropped.load(Ordering::Relaxed)];assert_eq!(order,[1,2,3]);assert!(denied_close>0);
  assert!(events.identity_checks.load(Ordering::Relaxed)>0);assert_eq!(events.reads_after_close.load(Ordering::Relaxed),0);while !source.terminal_is_empty(){source.close_once().unwrap();}
  
 }
}
