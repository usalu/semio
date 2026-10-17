//! 🧪️ A fresh original static gate admits one exclusive native borrow without physical lock backing.
use super::*;
#[global_allocator]
static ORIGINAL_GATE_HEAP: crate::HeapWitness = crate::HeapWitness;

#[test]
fn original_static_telemetry_recording_cold_backings_preserve_zero_native_birth(){
 let law:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../📈️percentile/🔐️gate/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for owner in law["backingOwners"].as_array().unwrap(){
  let owner=owner.as_str().unwrap();
  let ((),heap)=crate::observe_heap_allocations_on_this_thread(||match owner{
   "percentile"=>{let verdict=crate::Watchdog::start_at("original.static-native-percentile",crate::OperationId(919),crate::Generation(3),crate::InteractiveStage::InteractiveStep,Some(0)).finish_at(Some(1));assert_eq!(verdict.elapsed_us(),Some(1));assert!(!verdict.is_fault());},
   "trace"=>{let event=crate::record_trace_event_at(crate::OperationId(919),crate::Generation(3),crate::TraceStage::Committed,Some(1)).unwrap();assert_eq!(event.operation,crate::OperationId(919));assert_eq!(event.generation,crate::Generation(3));},
   "violation"=>{let verdict=crate::Watchdog::start_at("original.static-native-violation",crate::OperationId(919),crate::Generation(3),crate::InteractiveStage::InteractiveStep,Some(0)).finish_at(Some(9001));assert_eq!(verdict.elapsed_us(),Some(9001));assert!(verdict.violation().is_some());},
   _=>panic!("unknown original static telemetry owner")
  });
  assert_eq!((heap.requested_bytes,heap.released_bytes),(law["heapBirthBytes"].as_u64().unwrap()as usize,0),"{owner}");
 }
}
#[test]
fn original_static_telemetry_gate_preserves_exclusive_custody_and_zero_native_birth(){
 let original=StaticTryGate::new(17u64);let mut guard=original.try_lock().unwrap();assert_eq!(*guard,17);assert!(original.try_lock().is_none());*guard=19;drop(guard);let guard=original.try_lock().unwrap();assert_eq!(*guard,19);assert!(original.try_lock().is_none());drop(guard);assert!(original.try_lock().is_some());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
