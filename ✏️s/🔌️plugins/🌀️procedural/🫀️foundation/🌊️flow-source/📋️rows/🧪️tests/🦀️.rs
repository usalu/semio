//! 🧪️ Scratch row edits preserve sequence order and retain every denied or canceled native owner.
use super::retained_scratch_rows::{ScratchRowsKind,ScratchRowsWork};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observed<T>(f:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);assert!(!heap.overflowed);(value,heap.requested_bytes,heap.released_bytes)}
fn account(p:RetainedCloneProgress,g:RetainedCloneGrant,born:usize,released:usize){assert!(p.fits(g));assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes))}
fn denied(d:RetirementDemand,g:RetainedCloneGrant)->impl Iterator<Item=RetainedCloneGrant>{[(true,RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0,RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0,RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().filter_map(|(active,g)|active.then_some(g))}
#[test]
fn scratch_rows_preserve_owned_order_with_exact_receipts_and_cancellation(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for case in cases.as_array().unwrap(){for cancel in [None,Some(0),Some(1),Some(3),Some(9),Some(40)]{
  let mut rows:Vec<String>=serde_json::from_value(case["rows"].clone()).unwrap();
  let payload:Option<String>=serde_json::from_value(case["value"].clone()).unwrap();
  let kind=match case["kind"].as_str().unwrap(){"insert"=>ScratchRowsKind::Insert,"replace"=>ScratchRowsKind::Replace,"remove"=>ScratchRowsKind::Remove,_=>unreachable!()};
  let(work,born,released)=observed(||ScratchRowsWork::new(kind,case["index"].as_u64().unwrap()as usize,payload));assert_eq!((born,released),(0,0));let mut work=work.unwrap_or_else(|_|panic!("native String supports scratch retirement"));let mut ready=false;
  for turn in 0..10000{
   if cancel==Some(turn){break}
   let d=work.next_demands(&rows,4096).unwrap();let g=grant(d);
   for denied in denied(d,g){let(step,born,released)=observed(||work.advance(&mut rows,denied));assert_eq!(step.work,0);assert!(!step.result.unwrap());assert_eq!((born,released),(0,0));assert_eq!(work.next_demands(&rows,4096).unwrap(),d)}
   let(step,born,released)=observed(||work.advance(&mut rows,g));account(work.step_progress(),g,born,released);if step.result.unwrap(){ready=true;break}
  }
  if cancel.is_none(){assert!(ready);assert_eq!(serde_json::to_value(&rows).unwrap(),case["expected"])}
  work.begin_close();
  for _ in 0..10000{if work.terminal_is_empty(){break}let d=work.next_close_demands(4096).unwrap();let g=grant(d);
   for denied in denied(d,g){let(step,born,released)=observed(||work.close_step(denied));assert_eq!(step.work,0);assert_eq!(step.result.unwrap().progress(),Default::default());assert_eq!((born,released),(0,0));assert_eq!(work.next_close_demands(4096).unwrap(),d)}
   let(step,born,released)=observed(||work.close_step(g));account(step.result.unwrap().progress(),g,born,released)
  }assert!(work.terminal_is_empty());
  let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::new(rows).unwrap_or_else(|_|panic!("scratch strings have native vector retirement"));
  for _ in 0..10000{if close.terminal_is_empty(){break}let g=grant(close.selected_demand(4096).unwrap());let(step,born,released)=observed(||close.step(g));account(step.unwrap().progress(),g,born,released)}assert!(close.terminal_is_empty());
 }}
}
