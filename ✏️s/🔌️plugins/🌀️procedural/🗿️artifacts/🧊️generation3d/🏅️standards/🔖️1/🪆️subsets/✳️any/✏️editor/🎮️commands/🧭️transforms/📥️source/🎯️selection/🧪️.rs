//! 🧪️ Genuine one-fuel callers compare original selection grouping with the defining cold selection API.
use super::*;
use super::super::retained_scratch_testing::ScratchCaller;
use semio_s_artifact_procedural_3d_editor_selection::component_group;
fn policy()->serde_json::Value{serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap()}
fn close(work:&mut GumballSelectionWork,caller:&mut ScratchCaller){
 work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|same|{work.close_step(same)?;Ok(())}));}panic!("gumball selection did not reach physical terminal")
}
#[test]
fn gumball_selections_preserve_basic_component_and_analytic_grouping(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy=policy();
 for case in corpus["cases"].as_array().unwrap(){for items in [1,512]{for cut in [None,Some(0),Some(1),Some(7),Some(37)]{
  let(ids,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||case["ids"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().to_string()).collect::<Vec<_>>());assert!(!seed.overflowed);
  let mut caller=ScratchCaller::new(&policy,items,seed.requested_bytes-seed.released_bytes);let mut work=caller.turn(|cx|Ok(GumballSelectionWork::new(cx)));let mut count=0;
  loop{if cut==Some(count){caller.cancel();break}let ready=caller.turn(|cx|work.step(&ids,cx));count+=1;if ready{break}assert!(count<32768)}
  let matched=if cut.is_none(){
   let actual=work.result().unwrap();let mode=case["kind"].as_str().unwrap();
   match actual{
    Ok(SelectionKind::Basic)=>mode=="basic"&&!ids.iter().any(|id|ComponentTarget::parse(id).is_some()),
    Ok(SelectionKind::Components)=>{
     let(original,components)=component_group(&ids).unwrap();let borrowed=caller.turn(|cx|Ok(work.target(&ids,cx)?.is_some_and(|target|target==original)));let neutral=case["components"].as_array().unwrap();
     borrowed&&components.len()==work.component_count()&&neutral.len()==work.component_count()&&(0..components.len()).all(|index|work.component(index)==Some(components[index])&&neutral[index].as_u64()==Some(u64::from(components[index])))
    },
    Err(refusal)=>mode=="refused"&&case["refusal"].as_str()==Some(refusal.key())&&component_group(&ids).err().as_deref()==Some(refusal.detail()),
   }
  }else{true};
  close(&mut work,&mut caller);caller.close(ids);caller.finish();assert!(matched,"{}",case["name"]);
 }}}
}
#[test]
fn gumball_selection_denials_and_foreign_original_caller_leave_the_cursor_unspent(){
 let policy=policy();let(ids,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||vec!["a@mesh#0.face.2".to_string()]);let mut caller=ScratchCaller::new(&policy,1,seed.requested_bytes-seed.released_bytes);let mut work=caller.turn(|cx|Ok(GumballSelectionWork::new(cx)));
 let grant:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();let d=work.next_demands(&ids,grant.maximum_copy_bytes).unwrap();
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
  let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||work.advance(&ids,denied));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(turn.work,0);assert!(!turn.result.unwrap());assert_eq!(work.progress,Default::default());
 }
 let mut other=ScratchCaller::new(&policy,1,0);let foreign=other.turn(|cx|{let before=(cx.fuel_remaining(),cx.retained_progress());let refused=work.step(&ids,cx).is_err();Ok(refused&&before==(cx.fuel_remaining(),cx.retained_progress()))});other.finish();
 caller.turn(|cx|work.step(&ids,cx));let same=vec!["a@mesh#0.face.2".to_string()];let different=work.next_demands(&same,grant.maximum_copy_bytes).is_err();drop(same);close(&mut work,&mut caller);caller.close(ids);caller.finish();assert!(foreign&&different);
}

fn close_text(work:&mut GumballSelectionTextWork,caller:&mut ScratchCaller){
 work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|same|{work.close_step(same)?;Ok(())}));}panic!("gumball selection text did not reach physical terminal")
}
#[test]
fn original_sorted_selection_text_matches_compact_json_and_closes_every_prefix(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy=policy();
 for case in corpus["cases"].as_array().unwrap().iter().filter(|case|case["kind"]=="components"){for cut in [None,Some(0),Some(1),Some(2),Some(5)]{
  let(ids,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||case["ids"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().to_string()).collect::<Vec<_>>());let mut caller=ScratchCaller::new(&policy,512,seed.requested_bytes-seed.released_bytes);let mut selection=caller.turn(|cx|Ok(GumballSelectionWork::new(cx)));
  for _ in 0..32768{if caller.turn(|cx|selection.step(&ids,cx)){break}}
  let mut text=caller.turn(|cx|Ok(GumballSelectionTextWork::new(cx)));let mut count=0;let mut output=None;
  loop{if cut==Some(count){caller.cancel();break}let ready=caller.turn(|cx|text.step(&selection,cx));count+=1;if ready{break}assert!(count<32768)}
  let matched=if cut.is_none(){caller.turn(|cx|text.take_into(&selection,&mut output,cx));output.as_deref()==case["selectionText"].as_str()}else{true};
  close_text(&mut text,&mut caller);close(&mut selection,&mut caller);if let Some(output)=output.take(){caller.close(output)}caller.close(ids);caller.finish();assert!(matched,"{}",case["name"]);
 }}
}
