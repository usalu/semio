//! 🧪️ Genuine one-fuel callers compare typed construction with the defining native cold fold.
use semio_os_flow::neural::ColdRetire as _;
use semio_framework_value::FromValue as _;
use super::*;
use super::super::retained_scratch_testing::ScratchCaller;
use semio_framework_value::retirement::RetireOwned;
fn policy()->serde_json::Value{serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap()}
fn close(work:&mut TypedLiteralWork,caller:&mut ScratchCaller){
 work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|same|{let d=work.next_close_demands(same.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(same,d,|g|work.close_step(g))?;Ok(())}));}panic!("typed literal did not reach physical terminal")
}
#[test]
fn original_typed_literals_match_all_eleven_native_kinds_with_one_fuel_and_prefix_close(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy=policy();
 for case in corpus["cases"].as_array().unwrap(){for items in [1,512]{for cut in [None,Some(0),Some(1),Some(2),Some(7)]{
  let wire=case["input"].to_string();let(input,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_pack_json::from_json_str::<WidgetInputValue>(&wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
  assert!(!seed.overflowed);let mut caller=ScratchCaller::new(&policy,items,seed.requested_bytes.checked_sub(seed.released_bytes).unwrap());let mut work=caller.turn(|cx|Ok(TypedLiteralWork::new(cx)));let mut count=0;let mut output=None;
  loop{if cut==Some(count){caller.cancel();break}let ready=caller.turn(|cx|work.step(&input,cx));count+=1;if ready{break}assert!(count<32768)}
  let matched=if cut.is_none(){
   let expected=input.neural_value();let expected_wire=semio_framework_pack_json::to_json_string(&expected);
   caller.turn(|cx|work.take_into(&mut output,cx));let actual=output.as_ref().unwrap();let actual_wire=semio_framework_pack_json::to_json_string(actual);let native_equal=actual==&expected;
   let corpus_equal=serde_json::from_str::<serde_json::Value>(&actual_wire).unwrap()==case["expected"];expected.retire_cold();native_equal&&actual_wire==expected_wire&&corpus_equal
  }else{true};
  close(&mut work,&mut caller);if let Some(value)=output.take(){caller.close(value)}caller.close(input);let calls=caller.finish();assert!(matched,"{}",case["name"]);assert!(calls<=200);
 }}}
}
#[test]
fn typed_literal_denials_preserve_original_source_and_physical_caller(){
 let policy=policy();let(input,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||WidgetInputValue::Text("original 💠 text".into()));let mut caller=ScratchCaller::new(&policy,1,seed.requested_bytes-seed.released_bytes);let mut work=caller.turn(|cx|Ok(TypedLiteralWork::new(cx)));
 let g:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();let d=work.next_demands(&input,g.maximum_copy_bytes).unwrap();
 for denied in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},RetainedCloneGrant{maximum_depth:0,..g}]{
  let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||work.advance(&input,denied));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(turn.work,0);assert!(!turn.result.unwrap());assert_eq!(work.step_progress(),Default::default());
 }
 let mut other=ScratchCaller::new(&policy,1,0);let foreign_refused=other.turn(|cx|{let before=(cx.fuel_remaining(),cx.retained_progress());let refused=work.step(&input,cx).is_err();Ok(refused&&before==(cx.fuel_remaining(),cx.retained_progress()))});other.finish();
 caller.turn(|cx|work.step(&input,cx));let same=WidgetInputValue::Text("original 💠 text".into());let refused=work.next_demands(&same,g.maximum_copy_bytes).is_err();same.retire_decoded();close(&mut work,&mut caller);caller.close(input);caller.finish();assert!(foreign_refused&&refused);
}
