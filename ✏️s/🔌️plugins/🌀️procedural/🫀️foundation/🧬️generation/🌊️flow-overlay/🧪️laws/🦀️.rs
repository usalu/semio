//! 🧪️ Generation overlays equal the original native/JSON behavior and retain every canceled owner.
use super::retained_flow_overlay::GenerationFlowOverlayWork;
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_flow_flow::OrderedMap;
use semio_framework_value::DslValue;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observed<T>(f:impl FnOnce()->T)->(T,usize,usize){let(v,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);assert!(!h.overflowed);(v,h.requested_bytes,h.released_bytes)}
fn receipt(p:RetainedCloneProgress,g:RetainedCloneGrant,born:usize,released:usize){assert!(p.fits(g));assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes))}
fn snapshot(widgets:&serde_json::Value)->FlowHostSnapshot{let wire=serde_json::json!({"schema":"flow.host_snapshot","camera":{"x":0,"y":0,"zoom":1},"widgets":widgets,"synapses":[],"layout":{}});semio_framework_pack_json::from_json_str(&wire.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
#[test]
fn native_generation_overlays_equal_original_json_patch_with_exact_receipts(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for row in corpus.as_array().unwrap(){for cancel in [None,Some(0),Some(3),Some(8),Some(40)]{
  let mut source=snapshot(&row["widgets"]);let expected=snapshot(&row["expected"]);let values:OrderedMap<DslValue>=semio_framework_pack_json::from_json_str(&row["values"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let original_values=semio_framework_pack_json::parse(&row["values"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let original=semio_os_flow::forms_bridge::apply_generation_values_to_host_snapshot(&semio_framework_pack_json::to_json_string(&source),original_values.as_object().unwrap());
  let original:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&original,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(original,expected);
  let mut work=GenerationFlowOverlayWork::new();let mut completed=false;
  for index in 0..100000{
   if cancel==Some(index){break}
   let d=work.next_demands(&source,&values,4).unwrap();let g=grant(d);
   for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue}let(turn,born,released)=observed(||work.advance(&mut source,&values,denied));assert!(!turn.result.unwrap());assert_eq!(turn.work,0);assert_eq!((born,released),(0,0));}
   let(turn,born,released)=observed(||work.advance(&mut source,&values,g));let ready=turn.result.unwrap();receipt(work.step_progress(),g,born,released);if ready{completed=true;break}
  }
  if cancel.is_none(){assert!(completed);assert_eq!(source,expected)}
  work.begin_close();for _ in 0..100000{if work.terminal_is_empty(){break}let g=grant(work.next_close_demands(4).unwrap());let(turn,born,released)=observed(||work.close_step(g));receipt(turn.result.unwrap().progress(),g,born,released)}assert!(work.terminal_is_empty());
  source.retire_cold();expected.retire_cold();original.retire_cold();let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::new(values).unwrap_or_else(|_|panic!("original values declare controlled retirement"));
  for _ in 0..100000{if close.terminal_is_empty(){break}let g=grant(close.selected_demand(4).unwrap());let(step,born,released)=observed(||close.step(g).unwrap());receipt(step.progress(),g,born,released)}assert!(close.terminal_is_empty());
 }}
}
