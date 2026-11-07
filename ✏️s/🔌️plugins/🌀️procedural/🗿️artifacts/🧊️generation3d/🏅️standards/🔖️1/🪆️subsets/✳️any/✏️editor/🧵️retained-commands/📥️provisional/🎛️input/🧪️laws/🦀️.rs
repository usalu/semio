//! 🧪️ Provisional typed inputs equal the genuine central operation under one-fuel original callers.
use super::Generation3dMutation;
use super::retained_native_provisional_fold::{NativeProvisionalFold,NativeProvisionalFolds};
use super::retained_typed_input::{P3ProvisionalFold,ProvisionalTypedInputWork};
use super::retained_scratch_testing::ScratchCaller;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_job::{OriginalSourceBinding,close_original_retained_step};
use semio_framework_value::retirement::RetireOwned;
fn close(work:&mut ProvisionalTypedInputWork,caller:&mut ScratchCaller){
 work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|same|{let d=work.next_close_demands(same.retained_grant().maximum_copy_bytes)?;close_original_retained_step(same,d,|g|work.close_step(g))?;Ok(())}));}panic!("typed landing did not close original owners")
}
#[test]
fn provisional_typed_input_lands_like_the_original_central_operation_with_one_fuel(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for case in cases.as_array().unwrap(){for items in [1,512]{for cut in [None,Some(0),Some(1),Some(2),Some(7)]{
  let before_wire=case["before"].to_string();let mutation_wire=case["mutation"].to_string();let after_wire=case["after"].to_string();
  let before_oracle:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&before_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let original=super::host_scratch_test_original(&before_oracle,&case["mutation"]);before_oracle.retire_cold();
  let expected:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&after_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let expected_wire=semio_framework_pack_json::to_json_string(&original);assert_eq!(expected_wire,semio_framework_pack_json::to_json_string(&expected),"{}",case["name"]);expected.retire_cold();original.retire_cold();
  let((mut source,mutation),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
   let source:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&before_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
   let mutation:super::Generation3dMutation=semio_framework_pack_json::from_json_str(&mutation_wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();(source,mutation)
  });assert!(!seed.overflowed);
  let mut caller=ScratchCaller::new(&policy,items,seed.requested_bytes.checked_sub(seed.released_bytes).unwrap());
  let mut work=caller.turn(|cx|{let Ok(NativeProvisionalFold::Additional(P3ProvisionalFold::Input(work)))=<super::retained_provisional_dispatch::ProvisionalFolds as NativeProvisionalFolds<Generation3dMutation>>::into_fold_original(mutation,OriginalSourceBinding::borrow(cx))else{panic!("typed landing fixture dispatch")};Ok(work)});
  let mut count=0;loop{if cut==Some(count){caller.cancel();let canceled=caller.turn(|cx|{let canceled=cx.is_cancelled();let ready=work.step(&mut source,cx)?;Ok(canceled&&!ready)});assert!(canceled);break}let ready=caller.turn(|cx|work.step(&mut source,cx));count+=1;if ready{break}assert!(count<32768)}
  let matched=if cut.is_none(){work.applied()==case["applied"].as_bool().unwrap()&&semio_framework_pack_json::to_json_string(&source)==expected_wire}else{true};
  close(&mut work,&mut caller);caller.close(source);caller.finish();assert!(matched,"{}",case["name"]);
 }}}
}
