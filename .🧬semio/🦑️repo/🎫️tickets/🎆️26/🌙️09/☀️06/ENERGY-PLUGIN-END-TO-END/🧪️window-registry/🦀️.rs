//! 🧪️ Actual production registry source validates atomic insertion and original cancellation against a portable ordered map fixture.
#[path="/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗂️registry/🦀️.rs"]
mod registry;
#[cfg(test)]
mod tests{
 use super::registry::*;
 use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
 #[global_allocator]
 static ALLOCATOR:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
 fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
 fn observe<T>(work:impl FnOnce()->(T,RetainedCloneProgress))->T{let((value,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));value}
 fn insert(registry:&mut WindowRegistry<String,u64>,name:&str,value:u64){let(mut key,mut row)=(Some(name.to_owned()),Some(value));for _ in 0..128{let d=registry.insert_original_demands().unwrap();let done=observe(||{let(done,p)=registry.insert_original(&mut key,&mut row,grant(d)).unwrap();(done,p)});if done{return}}panic!("original insertion failed to complete")}
 fn close(registry:&mut WindowRegistry<String,u64>){while !registry.is_empty(){let d=registry.pop_demand().unwrap();let popped=registry.pop_original(grant(d)).unwrap().unwrap();drop(popped)}while !registry.terminal_is_empty(){let d=registry.backing_demand().unwrap();observe(||{let step=registry.close_backing_step(grant(d)).unwrap();((),step.progress())});}}
 #[test]
 fn original_window_insert_is_atomic_and_every_partial_hole_cancels(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for cutoff in 0..12{let mut registry=WindowRegistry::new();for row in fixture["initial"].as_array().unwrap(){insert(&mut registry,row[0].as_str().unwrap(),row[1].as_u64().unwrap())}let(mut key,mut value)=(Some(fixture["insert"][0].as_str().unwrap().to_owned()),Some(fixture["insert"][1].as_u64().unwrap()));let address=key.as_ref().unwrap().as_ptr();let mut published=false;
 for _ in 0..cutoff{let d=registry.insert_original_demands().unwrap();let denied=RetainedCloneGrant{maximum_items:0,..grant(d)};observe(||{let(done,p)=registry.insert_original(&mut key,&mut value,denied).unwrap();assert!(!done);((),p)});let done=observe(||registry.insert_original(&mut key,&mut value,grant(d)).unwrap());if done{published=true;break}assert_eq!(key.as_ref().unwrap().as_ptr(),address);assert!(registry.get("b").is_none());let actual=registry.iter().map(|(key,value)|serde_json::json!([key,value])).collect::<Vec<_>>();assert_eq!(serde_json::Value::Array(actual),fixture["initial"]);}
 if !published{for _ in 0..128{let d=registry.cancel_insert_original_demands().unwrap();let done=observe(||registry.cancel_insert_original(&mut key,&mut value,grant(d)).unwrap());if done{break}}assert_eq!(key.as_ref().unwrap().as_ptr(),address);assert_eq!(value,Some(2));assert_eq!(registry.len(),3);}else{assert_eq!(registry.get("b"),Some(&2));assert!(key.is_none()&&value.is_none());}
 close(&mut registry);
 }
 }
}
