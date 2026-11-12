//! 🧪️ Original identifier allocation and every prefix close use the same physical caller.
use super::*;
use super::super::retained_scratch_testing::ScratchCaller;
fn policy()->serde_json::Value{serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap()}
fn operation(text:&str)->GumballOperation{match text{"translate"=>GumballOperation::Translate,"rotate"=>GumballOperation::Rotate,"scale"=>GumballOperation::Scale,_=>panic!("invalid identity case")}}
fn close(work:&mut GumballNodeIdentityWork,caller:&mut ScratchCaller){
 work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|same|{work.close_step(same)?;Ok(())}));}panic!("gumball identifier did not close")
}
#[test]
fn original_node_identifiers_match_cold_utf8_and_component_suffixes_with_one_fuel(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy=policy();
 for case in corpus["cases"].as_array().unwrap(){for items in [1,512]{for cut in [None,Some(0),Some(1),Some(2),Some(3),Some(7)]{
  let(source,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||case["source"].as_str().unwrap().to_string());let mut caller=ScratchCaller::new(&policy,items,seed.requested_bytes-seed.released_bytes);
  let identity=GumballNodeIdentity{operation:operation(case["operation"].as_str().unwrap()),component:case["component"].as_bool().unwrap(),ordinal:case["ordinal"].as_str().unwrap().parse().unwrap()};let mut work=caller.turn(|cx|Ok(GumballNodeIdentityWork::new(identity,cx)));let mut output=None;let mut count=0;
  loop{if cut==Some(count){caller.cancel();break}let ready=caller.turn(|cx|work.step(&source,cx));count+=1;if ready{break}assert!(count<32768)}
  let matched=if cut.is_none(){caller.turn(|cx|work.take_into(&source,&mut output,cx));let cold=if identity.component{if identity.ordinal==0{format!("{source}__{}Components",case["operation"].as_str().unwrap())}else{format!("{source}__{}Components_{}",case["operation"].as_str().unwrap(),identity.ordinal)}}else{crate::gumball_widget_id(&source,case["operation"].as_str().unwrap())};output.as_deref()==Some(cold.as_str())&&output.as_deref()==case["expected"].as_str()}else{true};
  close(&mut work,&mut caller);if let Some(output)=output.take(){caller.close(output)}caller.close(source);caller.finish();assert!(matched,"{}",case["name"]);
 }}}
}
#[test]
fn original_node_identifiers_refuse_invalid_ordinals_foreign_callers_and_substitute_sources(){
 let policy=policy();let(source,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||"🏠".to_string());let mut caller=ScratchCaller::new(&policy,1,seed.requested_bytes-seed.released_bytes);
 for identity in [GumballNodeIdentity{operation:GumballOperation::Translate,component:true,ordinal:1},GumballNodeIdentity{operation:GumballOperation::Translate,component:false,ordinal:2}]{
  let mut work=caller.turn(|cx|Ok(GumballNodeIdentityWork::new(identity,cx)));let refused=caller.turn(|cx|{let before=(cx.fuel_remaining(),cx.retained_progress());let refused=work.step(&source,cx).is_err();Ok(refused&&before==(cx.fuel_remaining(),cx.retained_progress()))});close(&mut work,&mut caller);assert!(refused);
 }
 let identity=GumballNodeIdentity{operation:GumballOperation::Rotate,component:false,ordinal:0};let mut work=caller.turn(|cx|Ok(GumballNodeIdentityWork::new(identity,cx)));let mut other=ScratchCaller::new(&policy,1,0);let refused=other.turn(|cx|{let before=(cx.fuel_remaining(),cx.retained_progress());let refused=work.step(&source,cx).is_err();Ok(refused&&before==(cx.fuel_remaining(),cx.retained_progress()))});other.finish();
 caller.turn(|cx|work.step(&source,cx));let substitute=source.clone();let changed=work.next_demands(&substitute,512).is_err();drop(substitute);close(&mut work,&mut caller);caller.close(source);caller.finish();assert!(refused&&changed);
}
