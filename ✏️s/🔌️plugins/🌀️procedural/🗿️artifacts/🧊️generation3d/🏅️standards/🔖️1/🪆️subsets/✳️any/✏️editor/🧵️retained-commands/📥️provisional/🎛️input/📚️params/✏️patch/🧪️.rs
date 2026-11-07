//! 🧪️ Original parameter upserts equal the independent native cold dictionary and preserve payload owners.
use semio_os_flow::neural::ColdRetire as _;
use semio_framework_value::FromValue as _;
use super::*;
use super::super::retained_scratch_testing::ScratchCaller;
use semio_framework_artifact_flow_flow::neural::Atom;
use semio_framework_value::retirement::RetireOwned;
fn policy()->serde_json::Value{serde_json::from_str(include_str!("../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap()}
fn stamp(value:&Value)->(u8,u64,usize){match value{Value::Dictionary(value)=>(0,value.original_root_identity().unwrap_or(0)as u64,value.len()),Value::Atom(Atom::String(value))=>(1,value.as_ptr()as u64,value.len()),Value::Atom(Atom::Decimal(value))=>(2,value.to_bits(),0),Value::Atom(Atom::Integer(value))=>(3,*value as u64,0),Value::Atom(Atom::Boolean(value))=>(4,u64::from(*value),0),Value::Atom(Atom::Null)=>(5,0,0)}}
fn close(work:&mut NativeParamPatchWork,caller:&mut ScratchCaller){work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|same|{let d=work.next_close_demands(same.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(same,d,|g|work.close_step(g))?;Ok(())}));}panic!("original native patch did not physically close")}
#[test]
fn original_native_param_patch_matches_cold_upsert_with_one_fuel_and_full_prefix_close(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy=policy();let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
 for row in corpus["cases"].as_array().unwrap(){for items in [1,512]{for cut in [None,Some(0),Some(1),Some(2),Some(7)]{
  let base_wire=row["base"].to_string();let value_wire=row["value"].to_string();
  let((mut params,key,value),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(semio_framework_pack_json::from_json_str::<Dictionary>(&base_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(),row["key"].as_str().unwrap().to_string(),semio_framework_pack_json::from_json_str::<Value>(&value_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()));
  assert!(!seed.overflowed);let identity=stamp(&value);let mut caller=ScratchCaller::new(&policy,items,seed.requested_bytes.checked_sub(seed.released_bytes).unwrap());let mut work=caller.turn(|cx|Ok(NativeParamPatchWork::new(value,cx)));let mut count=0;
  loop{
   if cut==Some(count){caller.cancel();break}
   let d=work.next_demands(&params,&key,stock.maximum_copy_bytes).unwrap();let g=RetainedCloneGrant{maximum_items:items,..stock};
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..g}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g})].into_iter().flatten(){
    let before=(work.phase,params.original_root_identity());let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||work.advance(&mut params,&key,denied));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(turn.work,0);assert!(!turn.result.unwrap());assert_eq!(work.step_progress(),Default::default());assert!(before==(work.phase,params.original_root_identity()));
   }
   let ready=caller.turn(|cx|work.step(&mut params,&key,cx));count+=1;if ready{break}assert!(count<32768);
  }
  let matched=if cut.is_none(){
   let original=semio_framework_pack_json::from_json_str::<Dictionary>(&base_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let original_value=semio_framework_pack_json::from_json_str::<Value>(&value_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let expected=original.insert(row["key"].as_str().unwrap(),original_value);
   let actual_wire=semio_framework_pack_json::to_json_string(&params);let expected_wire=semio_framework_pack_json::to_json_string(&expected);let same=params==expected&&actual_wire==expected_wire&&serde_json::from_str::<serde_json::Value>(&actual_wire).unwrap()==row["expected"]&&params.get(&key).is_some_and(|value|stamp(value)==identity)&&work.applied();expected.retire_cold();same
  }else{true};
  close(&mut work,&mut caller);caller.close(params);caller.close(key);let calls=caller.finish();assert!(matched,"{}",row["name"]);assert!(calls<=200);
 }}}
}
#[test]
fn original_native_param_patch_rejects_a_foreign_real_caller_and_equal_replacement_source(){
 let policy=policy();let((mut params,key,value),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(Dictionary::new(),String::from("value"),Value::Atom(Atom::Boolean(true))));let mut caller=ScratchCaller::new(&policy,1,seed.requested_bytes-seed.released_bytes);let mut work=caller.turn(|cx|Ok(NativeParamPatchWork::new(value,cx)));let mut foreign=ScratchCaller::new(&policy,1,0);
 let refused=foreign.turn(|cx|{let before=(cx.fuel_remaining(),cx.retained_progress());let refused=work.step(&mut params,&key,cx).is_err();Ok(refused&&before==(cx.fuel_remaining(),cx.retained_progress()))});foreign.finish();caller.turn(|cx|work.step(&mut params,&key,cx));let other=String::from("value");let source_refused=work.next_demands(&params,&other,4096).is_err();other.retire_decoded();close(&mut work,&mut caller);caller.close(params);caller.close(key);caller.finish();assert!(refused&&source_refused);
}
#[test]
fn original_native_param_patch_preserves_exact_nonfinite_and_signed_zero_payload_words(){
 let policy=policy();for word in [0x8000000000000000,0x7ff0000000000000,0x7ff8000000000007,0xfff8000000000009]{
  let mut params=Dictionary::new();let value=Value::Atom(Atom::Decimal(f64::from_bits(word)));let mut caller=ScratchCaller::new(&policy,1,0);let mut work=caller.turn(|cx|Ok(NativeParamPatchWork::new(value,cx)));
  for _ in 0..32768{if caller.turn(|cx|work.step(&mut params,"value",cx)){break}}let same=matches!(params.get("value"),Some(Value::Atom(Atom::Decimal(value)))if value.to_bits()==word);
  close(&mut work,&mut caller);caller.close(params);caller.finish();assert!(same);
 }
}
