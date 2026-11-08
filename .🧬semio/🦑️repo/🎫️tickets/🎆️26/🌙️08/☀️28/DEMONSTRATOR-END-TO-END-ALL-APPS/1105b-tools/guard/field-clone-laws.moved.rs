//! 🧪️ Native field cloning keeps the captured issuer and exact heap receipts across refusal and cancellation.
use crate as snapshot_read;
use crate::{SnapshotRead,SnapshotReadFieldClone,SnapshotReadRegistryHandle};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value::ErasedSnapshotRetirement;
use std::sync::Arc;
type Root=(Vec<String>,u64);
fn field(root:&Root)->&Vec<String>{&root.0}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(call:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(call);assert!(!heap.overflowed);(value,heap.requested_bytes,heap.released_bytes)}
fn receipt(p:RetainedCloneProgress,g:RetainedCloneGrant,born:usize,released:usize){assert!(p.fits(g));assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));}
#[test]
fn native_field_clone_retains_original_store_issuer_and_receipts(){
 let corpus:serde_json::Value=serde_json::from_str::<serde_json::Value>(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap()["fieldClone"].clone();
 for law in corpus.as_array().unwrap(){
  let values:Vec<String>=serde_json::from_value(law["field"].clone()).unwrap();let expected=values.clone();let root=Arc::new((values,73));let pointer=Arc::as_ptr(&root);
  let registry=SnapshotReadRegistryHandle::new();let generation=law["generation"].as_u64().unwrap();let revision=[law["revisionByte"].as_u64().unwrap()as u8;32];assert!(registry.publish_authority(generation,revision));
  let lease=registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_|panic!("genuine Store issuer must issue original field read"));let read=SnapshotRead::new(root,lease);let identity=read.lease_identity();
  let mut work=SnapshotReadFieldClone::new(read,field,1,revision,generation);let mut output=None;

  if law["cancelAfter"].is_null(){
   let mut ready=false;for _ in 0..100{let g=grant(work.next_source_demands().unwrap());let(turn,born,released)=observe(||work.prepare_source(g));ready=turn.result.unwrap();receipt(work.step_progress(),g,born,released);if ready{break}}assert!(ready);assert!(!work.complete());
   let g=grant(semio_framework_value::retained_clone::RetainedOwnedProjection::<Vec<String>>::constructor_demand());let(turn,born,released)=observe(||work.project_original(2,field,g));let(mut projection,p)=turn.result.unwrap();receipt(p,g,born,released);assert_eq!(projection.borrow().unwrap().get()as*const _,field(work.original().unwrap().get())as*const _);
   for _ in 0..10000{if projection.terminal_is_empty(){break}let g=grant(projection.next_close_demand(4).unwrap());let(turn,born,released)=observe(||projection.close_step(g));receipt(turn.result.unwrap().progress(),g,born,released)}assert!(projection.terminal_is_empty());
  }

  for index in 0..10000{
   assert_eq!(work.original().unwrap().lease_identity(),identity);assert_eq!(work.original().unwrap().get()as*const _,pointer);
   if law["cancelAfter"].as_u64()==Some(index){break}
   let d=work.next_demands(4).unwrap();let g=grant(d);
   for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue}if semio_framework_value::retained_clone::retained_clone_grant_admits(denied,work.next_demands(denied.maximum_copy_bytes).unwrap()){continue}let(turn,born,released)=observe(||work.advance(denied));assert!(turn.result.unwrap().is_none());assert_eq!(turn.work,0);assert_eq!((born,released),(0,0));}
   let(turn,born,released)=observe(||work.advance(g));let value=turn.result.unwrap();receipt(work.step_progress(),g,born,released);
   if let Some(value)=value{assert_eq!(value,expected);output=Some(value);break}
  }
  if law["cancelAfter"].is_null(){assert!(output.is_some());}
  if let Some(output)=output{work.restore_output(output)}
  assert!(registry.publish_authority(generation+1,[0;32]));assert!(work.validate_current().is_err());work.begin_close();
  let mut closed=(RetainedCloneProgress::default(),0usize,0usize);for _ in 0..10000{if work.terminal_is_empty(){break}let g=grant(work.next_close_demands(4).unwrap());let(turn,born,released)=observe(||work.close_step(g));let step=turn.result.unwrap();receipt(step.progress(),g,born,released);closed=(closed.0.checked_add(step.progress()).unwrap(),closed.1+born,closed.2+released);}assert_eq!((closed.0.retained_capacity_bytes,closed.0.released_bytes),(closed.1,closed.2),"dropping terminal-empty owners leaves the exact close receipt total unchanged");
  assert!(work.terminal_is_empty());drop(work);
  let mut active=None;
  for _ in 0..10000{
   if active.is_some(){finish_box(&mut active);}
   if !registry.has_returned(){break}
   let g=close_grant();
   let(turn,born,released)=observe(||registry.try_admit_one_returned::<Root,_>(g,|root,grant|semio_framework_value::retirement::arc_custody::admit_shared_retirement(root,grant,false)));
   let(owner,p)=turn.result.unwrap_or_else(|_|panic!("original returned field read must retire"));receipt(p,g,born,released);active=owner;
  }
  finish_box(&mut active);assert!(!registry.has_returned());finish_registry(registry);
 }
}

fn close_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:4096}}
fn finish_box(owner:&mut Option<Box<dyn ErasedSnapshotRetirement>>){for _ in 0..10000{if owner.is_none(){return}let d=snapshot_read::artifact_retirement_box_demands(owner.as_ref().unwrap(),4096).unwrap();let g=grant(d);let(turn,born,released)=observe(||snapshot_read::artifact_retirement_box_close_step(owner,g));receipt(turn.result.unwrap().progress(),g,born,released)}panic!("original field test box did not close")}
fn finish_registry(registry:SnapshotReadRegistryHandle){let mut owner=Some(snapshot_read::SnapshotReadRegistryAliasRetirement::new(registry));for _ in 0..10000{if owner.is_none(){return}let d=snapshot_read::snapshot_registry_alias_demands(&owner).unwrap();let g=grant(d);let(step,born,released)=observe(||snapshot_read::snapshot_registry_alias_close_step(&mut owner,g).unwrap());receipt(step.progress(),g,born,released)}panic!("original field test issuer did not close")}
