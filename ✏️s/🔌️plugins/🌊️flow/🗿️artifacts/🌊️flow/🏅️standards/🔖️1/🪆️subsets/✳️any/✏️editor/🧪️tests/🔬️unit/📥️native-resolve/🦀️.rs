//! 📥️ The actual action bridge donates native output and window buffers without parsing or cloning them.
use super::*;
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,retained_clone::RetainedCloneClose};
fn allocation(value:&DslValue)->Option<(usize,usize,usize)>{match value{DslValue::Object(rows)=>Some((rows.as_ptr()as usize,rows.len(),rows.capacity())),DslValue::Array(rows)=>Some((rows.as_ptr()as usize,rows.len(),rows.capacity())),DslValue::String(text)=>Some((text.as_ptr()as usize,text.len(),text.capacity())),_=>None}}
fn observe<T>(run:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);assert!(!heap.overflowed);(value,heap.requested_bytes,heap.released_bytes)}
#[test]
fn native_resolve_action_bridge_matches_the_neutral_contract_and_preserves_original_buffers(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let(mut args,born,freed)=observe(||Some(semio_framework_pack_json::from_json_str::<DslValue>(row["argsJson"].as_str().unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()));let original=born-freed;
  let before=semio_framework_pack_json::to_json_string(args.as_ref().unwrap());let window=args.as_ref().unwrap().get("windowId").and_then(DslValue::as_str).map(|text|text.as_ptr()as usize);let output=args.as_ref().unwrap().get("output").and_then(allocation);
  let mut fault=None;let(result,born,freed)=observe(||<FlowPlayApp as semio_plugin::ArtifactEditor>::command_from_action("flowEvalResolve",&mut args,&mut fault));let(mut births,mut releases)=(born,freed);
  let(mut command,mut failure)=(None,None);let matched=match result{Ok(value)=>{command=Some(value);row["accepted"]==true},Err(error)=>{failure=Some(error);row["accepted"]==false}};
  let mut same=true;
  if let Some(FlowCommand::FlowEvalResolve(value))=command.as_ref(){
   same&=value.window_id==row["windowId"].as_str().unwrap()&&value.node_hash.to_string()==row["nodeHashWord"].as_str().unwrap();
   same&=Some(value.window_id.as_ptr()as usize)==window&&value.output.as_ref().and_then(allocation)==output;
   same&=(born,freed)==(0,0);
   same&=match value.output.as_ref(){Some(value)=>row.get("output").is_some_and(|expected|serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(value)).unwrap()==*expected),None=>row.get("output").is_none()};
   same&=args.as_ref().unwrap().get("windowId").and_then(DslValue::as_str)==Some("")&&args.as_ref().unwrap().get("output").is_none_or(|value|matches!(value,DslValue::Null));
  }else{same&=failure.is_some()&&semio_framework_pack_json::to_json_string(args.as_ref().unwrap())==before}
  type Owned=(Option<DslValue>,Option<FlowCommand>,Option<Fault>);let mut owned:Option<Owned>=Some((args,command,failure));let mut close=RetainedCloneClose::default();let mut turns=0;
  for _ in 0..32768{if owned.is_none()&&close.is_empty(){break}let d=close.next_owner_demands::<Owned>(owned.is_some(),&None,4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(turn,born,freed)=observe(||if close.is_empty(){let turn=close.begin_granted(&mut owned,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}else{close.step_granted(grant)});let p=turn.result.unwrap().progress();assert!(turn.work<=1&&p.fits(grant));assert_eq!((born,freed),(p.retained_capacity_bytes,p.released_bytes));births+=born;releases+=freed;turns+=1;}
  assert!(owned.is_none()&&close.is_empty());assert_eq!(original+births,releases);assert!(turns<=200);assert!(matched&&same,"{}",row["name"]);
 }
}
