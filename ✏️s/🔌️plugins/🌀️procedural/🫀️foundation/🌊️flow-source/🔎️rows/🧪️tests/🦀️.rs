use super::retained_scratch_scan::ScratchRowScan;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn key(row:&String)->&str{row.as_str()}
#[test]
fn scratch_identity_scan_matches_first_original_row_without_allocations(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for case in cases.as_array().unwrap(){
  let rows:Vec<String>=serde_json::from_value(case["rows"].clone()).unwrap();let target=case["key"].as_str().unwrap();let mut scan=ScratchRowScan::new();let mut turns=0;
  for _ in 0..10000{
   let d=scan.next_demands(&rows,target,key).unwrap();let g=grant(d);
   for denied in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g},RetainedCloneGrant{maximum_depth:0,..g}]{
    if denied.maximum_copy_bytes==g.maximum_copy_bytes&&denied.maximum_items==g.maximum_items&&denied.maximum_depth==g.maximum_depth{continue}
    let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||scan.advance(&rows,target,key,denied));assert_eq!(turn.work,0);assert!(!turn.result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(scan.next_demands(&rows,target,key).unwrap(),d)
   }
   let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||scan.advance(&rows,target,key,g));turns+=1;assert_eq!(turn.work,1);assert!(d.copy_bytes<=128);assert_eq!(scan.step_progress().copied_bytes,d.copy_bytes);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(scan.step_progress().fits(g));if turn.result.unwrap(){break}
  }assert!(scan.ready());assert_eq!(serde_json::to_value(scan.result()).unwrap(),case["expected"]);assert_eq!(turns,case["expectedTurns"].as_u64().unwrap());
 }
}
