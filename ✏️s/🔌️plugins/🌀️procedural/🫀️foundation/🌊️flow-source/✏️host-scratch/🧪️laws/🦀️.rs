//! 🧪️ Procedural host edits equal actual cold domain operations under one original finite caller.
use super::retained_host_scratch::HostScratchDomain;
use super::retained_native_provisional_fold::{NativeProvisionalFold,NativeProvisionalFolds};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
pub(super) fn clock()->Option<u64>{Some(1000)}
fn delta(a:RetainedCloneProgress,b:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:b.copied_items-a.copied_items,copied_bytes:b.copied_bytes-a.copied_bytes,retained_capacity_bytes:b.retained_capacity_bytes-a.retained_capacity_bytes,released_bytes:b.released_bytes-a.released_bytes}}
pub(super) fn observed<R>(cx:&mut semio_framework_job::StepContext<'_>,run:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<R,semio_framework_job::OriginalTurnFailure>{let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||run(cx));let p=delta(before,cx.retained_progress());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result}
pub(super) fn close<T:RetireOwned>(value:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,stock:RetainedCloneGrant,calls:&mut usize){
 let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("native host field supports controlled retirement"));
 while !owner.terminal_is_empty(){*calls+=1;assert!(*calls<=200);caller.turn_with_decoder(stock,decoder,|cx|observed(cx,|cx|cx.with_original_retirement(|cx|{
  for _ in 0..512{if owner.terminal_is_empty()||cx.should_yield(){break}let d=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|{let result=owner.step(g);RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;if step.progress()==Default::default(){break}}Ok(())
 }))).unwrap()}
}
fn mutation_wire(domain:HostScratchDomain,edit:&serde_json::Value)->serde_json::Value{
 let(name,payload)=edit.as_object().unwrap().iter().next().unwrap();let mut payload=payload.clone();
 let variant=match name.as_str(){"InsertWidget"=>"CreateWidget","ReplaceWidget"=>if domain==HostScratchDomain::Generation2d{"ReplaceWidget"}else{"UpdateWidget"},"RemoveWidget"=>"DeleteWidget","InsertSynapse"=>"ConnectSynapse","ReplaceSynapse"=>if domain==HostScratchDomain::Generation2d{"ReplaceSynapse"}else{"UpdateSynapse"},"RemoveSynapse"=>"DisconnectSynapse","SetLayout"=>"MoveWidget","RemoveLayout"=>if domain==HostScratchDomain::Generation2d{"ClearWidgetLayout"}else{"DeleteWidgetPosition"},"SetCamera"=>"UpdateCamera","SetSchema"=>"ChangeSchema","SetSlider"=>"ChangeSliderValue","MoveNodes"=>"MoveNodes",_=>unreachable!()};
 if domain==HostScratchDomain::Generation2d{let mut object=serde_json::Map::new();object.insert(variant.into(),payload);serde_json::Value::Object(object)}else{if name=="SetSchema"{let schema=payload.as_object_mut().unwrap().remove("schema").unwrap();payload["newSchema"]=schema}let mut tag=variant[..1].to_lowercase();tag.push_str(&variant[1..]);payload["mutation"]=tag.into();payload}
}
#[test]
fn procedural_host_scratch_matches_original_domain_with_receipts_and_prefix_close(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
 let domain=super::host_scratch_test_domain();let name=if domain==HostScratchDomain::Generation2d{"2d"}else{"3d"};
 for case in corpus["cases"].as_array().unwrap().iter().filter(|case|case["domains"].as_array().unwrap().iter().any(|item|item==name)){
  for cut in [None,Some(0),Some(1)]{
   let mut source:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&corpus["base"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
   let mut expected_json=corpus["base"].clone();for(key,value)in case["expected"].as_object().unwrap(){expected_json[key]=value.clone()}
   let expected:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&expected_json.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
   let original=super::host_scratch_test_original(&source,&mutation_wire(domain,&case["edit"]));let expected_wire=semio_framework_pack_json::to_json_string(&expected);assert_eq!(semio_framework_pack_json::to_json_string(&original),expected_wire,"{}",case["name"]);
   let mutation:super::HostScratchFixtureMutation=semio_framework_pack_json::from_json_str(&mutation_wire(domain,&case["edit"]).to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(policy["operation"].as_u64().unwrap()),Generation(policy["generation"].as_u64().unwrap()),stock,clock,policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());let mut decoder=OriginalJobDecodeOwner::empty();let mut calls=0;
   let mut work=caller.turn_with_decoder(stock,&mut decoder,|cx|{let Ok(NativeProvisionalFold::Host(work))=<super::ProvisionalSceneTestFolds as NativeProvisionalFolds<super::HostScratchFixtureMutation>>::into_fold_original(mutation,semio_framework_job::OriginalSourceBinding::borrow(cx))else{panic!("host fixture must dispatch to its actual retained host producer")};Ok(work)}).unwrap();
   loop{if cut==Some(calls){break}calls+=1;assert!(calls<=200);let ready=caller.turn_with_decoder(stock,&mut decoder,|cx|observed(cx,|cx|work.step(&mut source,cx))).unwrap();if ready{break}}
   if cut.is_none(){assert!(work.ready());assert_eq!(work.applied(),case["applied"].as_bool().unwrap());assert_eq!(semio_framework_pack_json::to_json_string(&source),expected_wire,"{}",case["name"])}
   work.begin_close();while !work.terminal_is_empty(){calls+=1;assert!(calls<=200);caller.turn_with_decoder(stock,&mut decoder,|cx|observed(cx,|cx|cx.with_original_retirement(|cx|{for _ in 0..512{if work.terminal_is_empty()||cx.should_yield(){break}let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|work.close_step(g))?;if step.progress()==Default::default(){break}}Ok(())}))).unwrap()}
   close(source,&mut caller,&mut decoder,stock,&mut calls);
   while !decoder.terminal_is_empty(){calls+=1;assert!(calls<=200);caller.turn(stock,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})).unwrap()}
   assert_eq!(caller.stock(),stock);assert!(caller.cumulative_receipt().fits(stock));assert!(policy["fuel"].as_u64().unwrap()-caller.fuel_remaining()<32000);caller.retire(stock);expected.retire_cold();original.retire_cold();
  }
 }
}
