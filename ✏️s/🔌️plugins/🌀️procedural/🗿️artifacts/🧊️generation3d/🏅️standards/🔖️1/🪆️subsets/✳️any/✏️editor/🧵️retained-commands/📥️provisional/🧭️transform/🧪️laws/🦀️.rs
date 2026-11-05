//! 🧪️ Relative transform scratch folds equal original central operations under the unchanged one-fuel caller.
use super::retained_provisional_transform::ProvisionalTransformWork;
use super::retained_native_provisional_fold::{NativeProvisionalFold,NativeProvisionalMutation};
use super::retained_typed_input::P3ProvisionalFold;
use super::retained_scratch_testing::ScratchCaller;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_job::{OriginalSourceBinding,close_original_retained_step};
fn materialize(base:&serde_json::Value,case:&serde_json::Value)->serde_json::Value{
 let mut before=base.clone();
 if let Some(params)=case["before"]["params"].as_object(){for widget in before["widgets"].as_array_mut().unwrap(){if let Some(params)=params.get(widget["id"].as_str().unwrap()){widget["params"]=params.clone()}}}
 if let Some(append)=case["before"]["append"].as_array(){before["widgets"].as_array_mut().unwrap().extend(append.iter().cloned())}
 if case["before"].get("synapses").is_some(){before["synapses"]=case["before"]["synapses"].clone()}
 before
}
fn expected(before:&serde_json::Value,case:&serde_json::Value)->serde_json::Value{
 let mut after=before.clone();
 for widget in after["widgets"].as_array_mut().unwrap(){if let Some(params)=case["expectedParams"].get(widget["id"].as_str().unwrap()).and_then(serde_json::Value::as_object){for(key,value)in params{widget["params"][key]=value.clone()}}}
 after
}
fn close(work:&mut ProvisionalTransformWork,caller:&mut ScratchCaller){
 work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|same|{let d=work.next_close_demands(same.retained_grant().maximum_copy_bytes)?;close_original_retained_step(same,d,|g|work.close_step(g))?;Ok(())}));}panic!("transform did not close its original owners")
}
#[test]
fn provisional_transforms_equal_original_central_operations_with_one_fuel_and_physical_cleanup(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){for items in [1,512]{for cut in [None,Some(0),Some(1),Some(2),Some(7)]{
  let before=materialize(&fixture["base"],case);let before_wire=before.to_string();let mutation_wire=case["mutation"].to_string();let after_wire=expected(&before,case).to_string();
  let oracle_source:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&before_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let oracle=super::host_scratch_test_original(&oracle_source,&case["mutation"]);oracle_source.retire_cold();
  let expected:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&after_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let expected_wire=semio_framework_pack_json::to_json_string(&oracle);assert_eq!(expected_wire,semio_framework_pack_json::to_json_string(&expected),"{}",case["name"]);expected.retire_cold();oracle.retire_cold();
  let((mut source,mutation),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
   let source:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&before_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
   let mutation:super::Generation3dMutation=semio_framework_pack_json::from_json_str(&mutation_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();(source,mutation)
  });assert!(!seed.overflowed);
  let mut caller=ScratchCaller::new(&policy,items,seed.requested_bytes.checked_sub(seed.released_bytes).unwrap());
  let mut work=caller.turn(|cx|{let Ok(NativeProvisionalFold::Additional(P3ProvisionalFold::Transform(work)))=mutation.into_fold_original(OriginalSourceBinding::borrow(cx))else{panic!("transform fixture dispatch")};Ok(work)});
  let mut count=0;loop{if cut==Some(count){caller.cancel();let canceled=caller.turn(|cx|{let canceled=cx.is_cancelled();let ready=work.step(&mut source,cx)?;Ok(canceled&&!ready)});assert!(canceled);break}let ready=caller.turn(|cx|work.step(&mut source,cx));count+=1;if ready{break}assert!(count<32768)}
  let matched=if cut.is_none(){work.applied()==case["applied"].as_bool().unwrap()&&semio_framework_pack_json::to_json_string(&source)==expected_wire}else{true};
  close(&mut work,&mut caller);caller.close(source);caller.finish();assert!(matched,"{}",case["name"]);
 }}}
}
