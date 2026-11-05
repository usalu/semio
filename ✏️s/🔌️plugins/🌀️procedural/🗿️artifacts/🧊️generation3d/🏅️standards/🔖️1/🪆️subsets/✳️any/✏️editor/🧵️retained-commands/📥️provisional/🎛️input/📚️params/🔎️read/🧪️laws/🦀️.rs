//! 🧪️ Native parameter reads preserve original fallbacks under one genuine one-fuel caller.
use super::retained_native_param_read::{NativeParamReadWork,ParamReadKind};
use super::retained_scratch_testing::ScratchCaller;
use semio_framework_artifact_flow_flow::neural::{Dictionary,Value,Atom};
use semio_framework_job::{OriginalSourceBinding,close_original_retained_step};
fn close(work:&mut NativeParamReadWork,caller:&mut ScratchCaller){work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|same|{let d=work.next_close_demands(same.retained_grant().maximum_copy_bytes)?;close_original_retained_step(same,d,|g|work.close_step(g))?;Ok(())}));}panic!("native parameter reader retains source cursors")}
#[test]
fn native_parameter_reads_match_neutral_fallbacks_with_original_one_fuel_and_prefix_close(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for case in cases.as_array().unwrap(){for items in [1,512]{for cut in [None,Some(0),Some(1),Some(7)]{
  let wire=case["params"].to_string();let(source,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_pack_json::from_json_str::<Dictionary>(&wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());assert!(!seed.overflowed);
  let fallback:[f64;3]=serde_json::from_value(case["fallback"].clone()).unwrap();let expected:[f64;3]=serde_json::from_value(case["expected"].clone()).unwrap();
  let key=match case["key"].as_str().unwrap(){"offset"=>"offset","axis"=>"axis","factor"=>"factor","angle"=>"angle",_=>unreachable!()};
  let kind=if case["kind"]=="vector"{ParamReadKind::Vector{key,fallback}}else{ParamReadKind::Number{key,fallback:fallback[0]}};
  let mut caller=ScratchCaller::new(&policy,items,seed.requested_bytes.checked_sub(seed.released_bytes).unwrap());let mut work=caller.turn(|cx|Ok(NativeParamReadWork::new_original(kind,OriginalSourceBinding::borrow(cx))));let mut count=0;
  loop{if cut==Some(count){caller.cancel();break}let ready=caller.turn(|cx|work.step(&source,cx));count+=1;if ready{break}assert!(count<32768)}
  let matched=if cut.is_none(){work.values()==Some(&expected)}else{true};close(&mut work,&mut caller);caller.close(source);caller.finish();assert!(matched,"{}",case["name"]);
 }}}
}
#[test]
fn native_parameter_reads_retain_nonfinite_and_signed_zero_carriers(){
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for bits in [0x8000000000000000u64,0x7ff8000000000042]{
  let(source,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Dictionary::new().insert("angle",Value::Dictionary(Dictionary::new().insert("value",Value::Atom(Atom::Decimal(f64::from_bits(bits)))))));assert!(!seed.overflowed);
  let mut caller=ScratchCaller::new(&policy,1,seed.requested_bytes-seed.released_bytes);let mut work=caller.turn(|cx|Ok(NativeParamReadWork::new_original(ParamReadKind::Number{key:"angle",fallback:1.0},OriginalSourceBinding::borrow(cx))));
  for _ in 0..32768{if caller.turn(|cx|work.step(&source,cx)){break}}let actual=work.values().unwrap()[0].to_bits();close(&mut work,&mut caller);caller.close(source);caller.finish();assert_eq!(actual,bits);
 }
}
