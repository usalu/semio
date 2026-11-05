//! 🧪️ Canonical gesture roots retain original values across writes with original one-fuel receipts.
use super::*;
use super::scratch_testing::ScratchCaller;
use semio_framework_job::close_original_retained_step;
use semio_framework_value::RetainedWorkOutcome;
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn gesture(ids:Vec<String>)->GumballGesture{
 let leaf=GumballMotion::Translate([1.0,0.0,0.0]).leaf(ids.clone());
 GumballGesture{states:Vec::new(),verb:"translateSelection".into(),authoring_seed:"native-source".into(),base_revision:"native-base".into(),transaction:crate::protocol::TransactionRef{id:"native-tx".into(),tool:"native-tool".into()},entries:vec![(GENERATION3D_GUMBALL_LEAF_KEY.into(),leaf)],ids}
}
fn fixture(rows:&serde_json::Value)->GumballGestures{
 let mut state=GumballGestures::default();for(window,ids)in rows.as_object().unwrap(){let ids:Vec<String>=serde_json::from_value(ids.clone()).unwrap();state.replace_gesture(window.clone(),gesture(ids));}state
}
fn rows(map:&OrderedMap<GumballGesture>)->serde_json::Value{serde_json::Value::Array(map.iter().map(|(key,gesture)|serde_json::json!([key,gesture.ids])).collect())}
fn close_read(work:&mut OriginalGumballRead,caller:&mut ScratchCaller){
 work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|cx|{let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|work.close_step(g))?;Ok(())}));}panic!("actual gesture read did not retire")
}
fn close_state(state:&mut GumballGestures,caller:&mut ScratchCaller){
 for _ in 0..32768{if state.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|cx|{let d=state.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|state.close_original(g))?;Ok(())}));}panic!("actual gesture map did not retire")
}
#[test]
fn original_live_gesture_source_matches_neutral_and_sql_snapshots_with_exact_original_cleanup(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for case in cases.as_array().unwrap(){for items in [1,512]{for cut in [None,Some(0),Some(1),Some(2),Some(3),Some(7)]{
  let(mut state,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture(&case["initial"]));assert!(!heap.overflowed);
  let identities:std::collections::BTreeMap<String,(usize,usize)>=state.live().iter().map(|(window,gesture)|(window.clone(),(gesture.ids.as_ptr()as usize,gesture.entries.as_ptr()as usize))).collect();
  let mut caller=ScratchCaller::new(&policy,items,heap.requested_bytes.checked_sub(heap.released_bytes).unwrap());let mut read=caller.turn(|cx|Ok(OriginalGumballRead::new(cx)));let mut calls=0;let mut actual=Vec::new();
  loop{
   if cut==Some(calls){caller.cancel();break}
   let ready=caller.turn(|cx|read.step(&state,cx));calls+=1;
   if ready{
    if read.finished(){break}
    let pointer=caller.turn(|cx|{let record=read.record(&state,cx)?.unwrap();Ok((record.window.as_ptr()as usize,record.ids.as_ptr()as usize))});
    let fields=&*read.fields;let window=fields.key.as_ref().unwrap();let value=fields.gesture.as_ref().unwrap();
    assert_eq!(pointer.0,window.as_ptr()as usize);assert_eq!(pointer.1,identities[window.as_str()].0);assert_eq!(value.entries.as_ptr()as usize,identities[window.as_str()].1);
    actual.push(serde_json::json!([window.as_str(),value.ids]));
    caller.turn(|cx|read.next_record(&state,cx));
   }
   assert!(calls<32768)
  }
  if cut.is_none(){
   assert_eq!(serde_json::Value::Array(actual),case["expected"]);
   let(_,writer_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
    for write in case["writes"].as_array().unwrap(){let window=write["window"].as_str().unwrap();if write["ids"].is_null(){state.abort(window,semio_framework_tool_machine::ToolAbortReason::Tool);}else{let ids=serde_json::from_value(write["ids"].clone()).unwrap();state.replace_gesture(window.into(),gesture(ids));state.revision=state.revision.wrapping_add(1);}}
   });assert!(!writer_heap.overflowed);
   assert_eq!(rows(state.live()),case["live"]);assert_eq!(rows(read.fields.map.as_ref().unwrap()),case["expected"]);assert_eq!(read.matches(&state),case["matches"].as_bool().unwrap());
   if !read.matches(&state){let failure=caller.turn(|cx|Ok(read.step(&state,cx).err().expect("changed canonical source must refuse")));caller.close(failure)}
   let mut writer=ScratchCaller::new(&policy,items,writer_heap.requested_bytes.checked_sub(writer_heap.released_bytes).unwrap());close_state(&mut state,&mut writer);writer.finish();
  }else{close_state(&mut state,&mut caller);}
  close_read(&mut read,&mut caller);caller.finish();
 }}}
}
#[test]
fn original_live_gesture_read_refuses_foreign_callers_and_zero_grants_without_source_effects(){
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let wire=serde_json::json!({"window":["source"]});
 let(mut state,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fixture(&wire));assert!(!heap.overflowed);
 let mut caller=ScratchCaller::new(&policy,1,heap.requested_bytes-heap.released_bytes);let mut read=caller.turn(|cx|Ok(OriginalGumballRead::new(cx)));
 let mut foreign=ScratchCaller::new(&policy,1,0);let failure=foreign.turn(|cx|Ok(read.step(&state,cx).err().expect("same-label foreign caller")));foreign.close(failure);foreign.finish();
 for _ in 0..32768{
  if read.ready(){break}
  caller.turn(|cx|{let d=read.next_demands(&state,cx.retained_grant().maximum_copy_bytes)?;let grant=RetainedCloneGrant{maximum_items:0,..cx.retained_grant()};let before=state.live().original_root_identity();let turn=read.advance(&state,grant);assert_eq!(turn.work,0);assert!(!turn.result?);assert_eq!(read.step_progress(),Default::default());assert_eq!(read.next_demands(&state,cx.retained_grant().maximum_copy_bytes)?,d);assert_eq!(state.live().original_root_identity(),before);Ok(())});
  caller.turn(|cx|read.step(&state,cx));
 }
 close_state(&mut state,&mut caller);close_read(&mut read,&mut caller);caller.finish();
}
#[test]
fn first_original_transform_entry_is_the_only_live_motion_source(){
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 let(mut state,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
  let mut state=GumballGestures::default();let mut value=gesture(vec!["shape".into()]);
  value.entries.insert(0,("other".into(),GumballMotion::Scale([2.0;3]).leaf(vec!["different".into()])));
  value.entries.push((GENERATION3D_GUMBALL_LEAF_KEY.into(),GumballMotion::Translate([99.0;3]).leaf(vec!["later".into()])));
  state.replace_gesture("window".into(),value);state
 });assert!(!heap.overflowed);
 let mut caller=ScratchCaller::new(&policy,1,heap.requested_bytes-heap.released_bytes);let mut read=caller.turn(|cx|Ok(OriginalGumballRead::new(cx)));
 while !read.ready(){caller.turn(|cx|read.step(&state,cx));}
 assert!(caller.turn(|cx|Ok(read.record(&state,cx)?.unwrap().motion==GumballMotion::Translate([1.0,0.0,0.0]))));
 close_state(&mut state,&mut caller);close_read(&mut read,&mut caller);caller.finish();
}
