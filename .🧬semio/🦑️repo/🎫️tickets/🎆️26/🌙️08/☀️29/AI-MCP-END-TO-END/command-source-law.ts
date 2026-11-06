import fs from"node:fs";const p="🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🧪️tests/🔬️unit/🦀️.rs";const old=fs.readFileSync(p,"utf8");const law=`
#[test]
fn original_page_set_admission_and_capture_post_only_actual_heap_effects(){
 use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
 let born=CommandPageSet::source_admission_demand(2).unwrap();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:born.copy_bytes,maximum_capacity_bytes:born.capacity_bytes,maximum_release_bytes:0,maximum_depth:1};
 let mut pages=None;
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:born.copy_bytes-1,..grant},RetainedCloneGrant{maximum_capacity_bytes:born.capacity_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
  let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||CommandPageSet::admit_source_step(&mut pages,2,denied));
  assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert!(pages.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
 let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||CommandPageSet::admit_source_step(&mut pages,2,grant));let progress=turn.result.unwrap().progress();
 assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert!(progress.fits(grant));assert!(pages.is_some());
 let cursor=CommandPageCursor{owner:7,generation:11,command_index:0,command_count:1,instance:13,seq:17,kind:19,page_index:0,page_count:1,item_count:0,metadata:0};
 let mut input=Some((cursor,FixedCommandPage::try_copy_from(&[19]).unwrap()));let pointer=input.as_ref().unwrap().1.as_slice().as_ptr();
 let d=CommandPageSet::source_capture_demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
  let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pages.as_mut().unwrap().capture_source_step(&mut input,denied));
  assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert!(input.is_some());assert!(pages.as_ref().unwrap().is_empty());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
 let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pages.as_mut().unwrap().capture_source_step(&mut input,grant));assert!(turn.result.unwrap().progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(input.is_none());assert_eq!(pages.as_ref().unwrap().pages.back().unwrap().as_slice().as_ptr(),pointer);
}
`;if(fs.readFileSync(p,"utf8")!==old)throw Error("stale");fs.writeFileSync(import.meta.dir+"/🗑️generated/command-source-law.rs",old+law);fs.renameSync(import.meta.dir+"/🗑️generated/command-source-law.rs",p);console.log("Authored real command page admission/capture heap law.");