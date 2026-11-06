import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),b="🧰️framework/🔨️modules/🧵️job/🛬️receiving/",f=b+"🧪️tests/🦀️.rs",s=readFileSync(f,"utf8");let n=s;function region(a:string,b:string,z:string){const i=n.indexOf(a),j=n.indexOf(b,i+a.length);if(i<0||j<0)throw Error("region "+a);n=n.slice(0,i)+z+n.slice(j)}
region("fn retire_original_text(","#[test]\nfn original_receiving_ready_scope",`fn retire_original_text(raw:&mut Option<String>,close:&mut RetainedCloneClose,frame:&mut semio_framework_value_native_owned::native_decoding::NativeReceivingEffect<semio_framework_value::OriginalRetirementFailure>,control:&mut NativeRetirementDecodeControl<'_, '_>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
 let turn=if frame.has_pending_recovery(){control.finish_retirement_receiving(frame)}else{
  let d=match close.next_owner_demands::<String>(raw.is_some(),&None,grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};let q=match close.next_owner_work::<String>(raw.is_some(),&None,grant.maximum_copy_bytes){Ok(q)=>q,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  let available=match control.available_work_units(){Ok(n)=>n,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if q>available{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  control.receive_retirement(d,frame,d.capacity_bytes!=0,q,|g,actual|{let mut none=None;let marker=actual.unwrap_or(&mut none);if !close.is_empty(){close.step_granted(g,marker)}else{let turn=close.begin_granted(raw,g,marker);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}})
 };
 RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}
}
#[test]
fn original_context_retirement_after_cancel_keeps_ledger_deadline_fuel_and_exact_heap(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../../🛬️receiving/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["retirementCases"].as_array().unwrap(){
  let stock=bank();let grant=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(75),Generation(4),stock,logical_now_us,u64::MAX,10000);let mut owner=OriginalJobDecodeOwner::empty();let mut raw=Some(copy_native_text(&mut caller,&mut owner,grant,"π\u0000🌿"));let identity=owner.receiving_identity();assert_eq!(serde_json::to_value(raw.as_ref().unwrap()).unwrap(),serde_json::json!("π\u0000🌿"));
  if case["cancelled"].as_bool().unwrap(){caller.cancel().cancel_now()}
  let mut close=RetainedCloneClose::default();let mut frame=semio_framework_value_native_owned::native_decoding::NativeReceivingEffect::default();let visited=std::cell::Cell::new(false);let admitted=owner.owned_bytes();let mut initial=Default::default();
  caller.turn_with_decoder(grant,&mut owner,|cx|{
   let deadline=if case["deadlineOpen"].as_bool().unwrap(){cx.deadline_us()}else{0};let before=cx.retained_progress();
   let result:Result<(),OriginalTurnFailure>=cx.with_original_work_loan(case["fuel"].as_u64().unwrap(),deadline,|cx|cx.with_original_retirement(|cx|{
    let mut outcome=None;let mut child=None;
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_native_retirement_receiving(&mut outcome,|control|{visited.set(true);assert_eq!(Some((control.ledger_identity(),control.retirement_recipient_identity().unwrap())),identity);child=Some(retire_original_text(&mut raw,&mut close,&mut frame,control,grant));Ok(())}));
    assert!(!heap.overflowed);match result{Ok(true)=>{source_received(outcome);let turn=child.expect("actual retirement result is retained");let p=turn.result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));},Ok(false)=>{assert!(outcome.is_none()&&child.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));},Err(failure)=>{assert!(!case["visited"].as_bool().unwrap());assert_eq!(failure.retained_progress(),Default::default());assert!(outcome.is_none()&&child.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}}Ok(())
   }));
   initial=delta(cx.retained_progress(),before);if let Err(failure)=result{assert!(!case["visited"].as_bool().unwrap());assert_eq!(failure.retained_progress(),Default::default())}Ok(())
  }).unwrap();assert_eq!(visited.get(),case["visited"].as_bool().unwrap());assert_eq!(owner.owned_bytes()-admitted,initial.retained_capacity_bytes);
  for _ in 0..10000{
   if raw.is_none()&&close.is_empty()&&!frame.has_pending_recovery(){break}
   let admitted=owner.owned_bytes();let mut actual=Default::default();caller.turn_with_decoder(grant,&mut owner,|cx|cx.with_original_retirement(|cx|{
    let before=cx.retained_progress();let mut outcome=None;let mut child=None;
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_native_retirement_receiving(&mut outcome,|control|{assert_eq!(Some((control.ledger_identity(),control.retirement_recipient_identity().unwrap())),identity);child=Some(retire_original_text(&mut raw,&mut close,&mut frame,control,grant));Ok(())}));assert!(result?);source_received(outcome);let p=child.expect("funded original retirement ran").result.unwrap().progress();actual=delta(cx.retained_progress(),before);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(actual.fits(grant));Ok(())
   })).unwrap();assert_eq!(owner.owned_bytes()-admitted,actual.retained_capacity_bytes);
  }
  assert!(raw.is_none()&&close.is_empty()&&!frame.has_pending_recovery());assert_eq!(owner.receiving_identity(),identity);close_decoder(&mut caller,&mut owner,grant);caller.retire(stock);
 }
}
`);
const q=join(t,"🗑️generated/source-retirement-law.rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent Source laws");renameSync(q,f);
const fixture=b+"🧪️tests/🧫️fixtures/🔣️.json",old=readFileSync(fixture,"utf8"),law=JSON.parse(old);for(const row of law.retirementCases)if(row.visited)row.fuel=6;writeFileSync(join(t,"🗑️generated/source-retirement-corpus.json"),JSON.stringify(law,null,2)+"\n");if(readFileSync(fixture,"utf8")!==old)throw Error("concurrent corpus");renameSync(join(t,"🗑️generated/source-retirement-corpus.json"),fixture);
console.log("Cancellation cleanup law uses exact Source retirement gate and real allocator evidence");
