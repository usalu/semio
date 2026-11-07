//! 🧪️ Retried dispatch retains the same original request allocation under the actual one-fuel caller.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
fn observe<T>(f:impl FnOnce()->T)->(T,usize,usize){let(v,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);(v,h.requested_bytes,h.released_bytes)}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,g:RetainedCloneGrant,births:&mut usize,releases:&mut usize,f:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let mut output=None;let(_,born,free)=observe(||caller.turn(g,|cx|cx.with_original_work_loan(1,1500,|cx|{output=Some(f(cx)?);Ok(())})).unwrap());let p=caller.receipt();assert!(p.fits(g));assert_eq!((born,free),(p.retained_capacity_bytes,p.released_bytes));*births+=born;*releases+=free;output.unwrap()}
fn retire(caller:&mut OriginalJobFixtureCaller,g:RetainedCloneGrant,births:&mut usize,releases:&mut usize,f:impl FnOnce(&mut StepContext<'_>)->Result<(),OriginalTurnFailure>){turn(caller,g,births,releases,|cx|cx.with_original_retirement(f))}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[test]
fn original_flow_retry_lookup_preserves_genuine_origin_one_fuel_cancel_and_entire_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️origin.json")).unwrap();let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096};
 for row in fixture["cases"].as_array().unwrap(){for session_first in [false,true]{for stop in [None,Some(0),Some(1),Some(7)]{
  let(mut session,born,free)=observe(||{let mut session=flow::FlowEvalSession::new();for source in row["origins"].as_array().unwrap(){session.retain_invocation_origin_cold(flow::FlowInvocationOriginLease::from_cold(flow::FlowInvocationOrigin{window_id:source["window"].as_str().unwrap().into(),window_kind_id:"main".into(),neuron_id:source["neuron"].as_str().unwrap().into(),extension_id:"extension".into(),operator_id:"operator".into(),node_hash:source["hash"].as_u64().unwrap(),input_json:"{}".into(),dependency_json:"[]".into(),operator_version:"registry:0;geometry:1;policy:1".into(),outer_node_hash:None,preview_source:None}));}if row["progress"].as_bool().unwrap(){session.resolve_preview_eval_cold(row["hash"].as_u64().unwrap(),r#"{"done":false,"phase":"evaluate","unitsDone":1,"unitsTotal":2}"#).unwrap();}session});let original=born-free;let word=row["window"].as_str().unwrap();let hash=row["hash"].as_u64().unwrap();let oracle=session.invocation_origin_cold(word,hash);assert_eq!(oracle.map(|o|o.neuron_id.as_str()),row["expected"].as_str());let pointer=oracle.map(|o|o as*const _ as usize);let mut lookup=FlowOriginLookupWork::new(hash);let mut caller=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),stock,||Some(1000),1500,32768);let(mut births,mut releases)=(0,0);let mut output=None;
  for call in 0..200{if stop==Some(call){caller.cancel().cancel_now();let(_,born,free)=observe(||caller.turn(stock,|cx|cx.with_original_work_loan(1,1500,|cx|{assert!(lookup.step(&session,word,cx)?.is_none());Ok(())})).unwrap());assert_eq!((born,free),(0,0));assert_eq!(caller.receipt(),Default::default());break}assert!(turn(&mut caller,RetainedCloneGrant{maximum_items:0,..stock},&mut births,&mut releases,|cx|lookup.step(&session,word,cx)).is_none());output=turn(&mut caller,stock,&mut births,&mut releases,|cx|lookup.step(&session,word,cx));if output.is_some(){break}}
  if stop.is_none(){let result=output.as_ref().expect("original lookup exceeded its actual 200 opportunities");assert_eq!(result.source.as_ref().map(|source|&**source as*const _ as usize),pointer);assert_eq!(result.resume,pointer.is_some()&&row["progress"].as_bool().unwrap());}
  if stop.is_some(){caller.cancel().cancel_now();let(_,born,free)=observe(||caller.turn(stock,|cx|cx.with_original_work_loan(1,1500,|cx|{assert!(lookup.step(&session,word,cx)?.is_none());Ok(())})).unwrap());assert_eq!((born,free),(0,0));assert_eq!(caller.receipt(),Default::default());}
  lookup.begin_close();let mut output=semio_framework_value::retirement::controlled::ControlledRetirement::new(output).unwrap_or_else(|_|unreachable!());session.begin_close();
  for family in if session_first{[0,1,2]}else{[1,2,0]}{for _ in 0..100000{match family{
   0=>{if session.terminal_is_empty(){break}let d=RetirementDemand{copy_bytes:session.next_close_copy_byte_demand().unwrap(),capacity_bytes:session.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:session.next_close_release_byte_demand().unwrap(),depth:session.next_close_depth_demand().unwrap()};retire(&mut caller,grant(d),&mut births,&mut releases,|cx|{close_original_retained_step(cx,d,|g|session.retire_source_step(g))?;Ok(())});},
   1=>{if lookup.terminal_is_empty(){break}let d=lookup.close_demands(4096).unwrap();retire(&mut caller,grant(d),&mut births,&mut releases,|cx|lookup.close_step(cx).map(|_|()));},
   _=>{if output.terminal_is_empty(){break}let d=output.selected_demand(4096).unwrap();retire(&mut caller,grant(d),&mut births,&mut releases,|cx|{close_original_retained_step(cx,d,|g|{let result=output.step(g);RetainedWorkOutcome{result,work:output.last_turn_work()}})?;Ok(())});}
  }}}
  assert!(session.terminal_is_empty()&&lookup.terminal_is_empty()&&output.terminal_is_empty());assert_eq!(original+births,releases);caller.retire(stock);
 }}}
}


fn close_pending_owned<T:semio_framework_value::RetireOwned>(value:T,caller:&mut OriginalJobFixtureCaller,stock:RetainedCloneGrant,births:&mut usize,releases:&mut usize){
 let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_|unreachable!());
 for _ in 0..100000{if close.terminal_is_empty(){return}let d=close.selected_demand(stock.maximum_copy_bytes).unwrap();retire(caller,grant(d),births,releases,|cx|{close_original_retained_step(cx,d,|g|{let result=close.step(g);RetainedWorkOutcome{result,work:close.last_turn_work()}})?;Ok(())});}panic!("original pending output did not close")
}

#[test]
fn original_shared_pending_dispatch_preserves_order_origin_wire_and_entire_caller_stock(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/📨️pending/🧪️cases.json")).unwrap();
 let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096};
 let warm=flow::FlowEvalSession::new();warm.retire_cold();
 for row in rows.as_array().unwrap(){for stop in [None,Some(0),Some(1),Some(7),Some(20)]{
  let((mut session,source,pending),born,free)=observe(||{
   let mut session=flow::FlowEvalSession::new();
   for source in row["origins"].as_array().unwrap(){session.retain_invocation_origin_cold(flow::FlowInvocationOriginLease::from_cold(flow::FlowInvocationOrigin{window_id:source["window"].as_str().unwrap().into(),window_kind_id:"main".into(),neuron_id:source["neuron"].as_str().unwrap().into(),extension_id:"extension".into(),operator_id:"operator".into(),node_hash:source["hash"].as_u64().unwrap(),input_json:"{}".into(),dependency_json:"[]".into(),operator_version:"original-version".into(),outer_node_hash:None,preview_source:None}));}
   for hash in row["progress"].as_array().unwrap(){session.resolve_preview_eval_cold(hash.as_u64().unwrap(),r#"{"done":false,"phase":"evaluate","unitsDone":1,"unitsTotal":2}"#).unwrap();}
   let pending:std::collections::VecDeque<_>=row["pending"].as_array().unwrap().iter().map(|value|flow::neural::PendingExtensionEval{neuron_id:value["neuron"].as_str().unwrap().into(),extension_id:"extension".into(),operator_id:"operator".into(),input_json:"{}".into(),node_hash:value["hash"].as_u64().unwrap()}).collect();
   let source=semio_framework_artifact_flow_flow::FlowHostSnapshot::default();(session,source,pending)
  });
  let original=born-free;let mut caller=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),stock,||Some(1000),1500,32768);let(mut births,mut releases)=(0,0);
  let mut work=turn(&mut caller,stock,&mut births,&mut releases,|cx|{let bytes=size_of::<flow::FlowPendingDispatchWork>()+2*size_of::<std::collections::VecDeque<flow::neural::PendingExtensionEval>>();cx.consume_fuel(1);cx.consume_retained(progress(bytes))?;Ok(flow::FlowPendingDispatchWork::new(pending,flow::flow_extension_registry_generation(),cx))});
  let word=row["window"].as_str().unwrap();let kind="main";let mut seen=0;let mut complete=false;
  let mut foreign=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),stock,||Some(1000),1500,32768);let(mut foreign_births,mut foreign_releases)=(0,0);
  let refusal=turn(&mut foreign,stock,&mut foreign_births,&mut foreign_releases,|cx|Ok(match work.step(&source,&mut session,word,kind,cx){Err(refusal)=>refusal,Ok(_)=>panic!("same labels must not replace the physical pending caller")}));
  close_pending_owned(refusal,&mut foreign,stock,&mut foreign_births,&mut foreign_releases);assert_eq!(foreign_births,foreign_releases);foreign.retire(stock);
  for call in 0..200{
   if stop==Some(call){caller.cancel().cancel_now();assert!(turn(&mut caller,stock,&mut births,&mut releases,|cx|Ok(cx.is_cancelled()&&work.step(&source,&mut session,word,kind,cx)?.is_none())));assert_eq!(caller.receipt(),Default::default());break}
   let denied=turn(&mut caller,RetainedCloneGrant{maximum_items:0,..stock},&mut births,&mut releases,|cx|work.step(&source,&mut session,word,kind,cx));assert!(denied.is_none());assert_eq!(caller.receipt(),Default::default());
   let d=work.next_demands(&source,&session,word,kind,stock.maximum_copy_bytes).unwrap();
   let output=turn(&mut caller,grant(d),&mut births,&mut releases,|cx|work.step(&source,&mut session,word,kind,cx));
   match output{
    Some(flow::FlowPendingDispatchStep::Invocation(output))=>{
     let body:serde_json::Value=serde_json::from_str(&output.request_json).unwrap();assert_eq!(body["neuronId"],row["expected"][seen]["neuron"]);assert_eq!(body["resume"],row["expected"][seen]["resume"]);assert_eq!(body["extensionId"],"extension");assert_eq!(output.extension_id,"extension");assert_eq!(output.capability,"evaluate");assert_eq!(output.response_action,"flowEvalResolve");assert!(body.get("retained").is_none());if body["resume"]==true{assert_eq!(body["inputJson"],"");assert_eq!(body["dependencyJson"],"");}
     seen+=1;close_pending_owned(output,&mut caller,stock,&mut births,&mut releases);
    },
    Some(flow::FlowPendingDispatchStep::Complete)=>{complete=true;break},
    None=>{},
   }
  }
  if stop.is_none(){assert!(complete,"shared pending batch exceeded original 200 opportunities: {}",row["id"]);assert_eq!(seen,row["expected"].as_array().unwrap().len());}
  work.begin_close();for _ in 0..100000{if work.terminal_is_empty(){break}let d=work.close_demands(stock.maximum_copy_bytes).unwrap();retire(&mut caller,grant(d),&mut births,&mut releases,|cx|work.close_step(cx).map(|_|()));}assert!(work.terminal_is_empty());
  close_pending_owned(source,&mut caller,stock,&mut births,&mut releases);
  session.begin_close();for _ in 0..100000{if session.terminal_is_empty(){break}let d=RetirementDemand{copy_bytes:session.next_close_copy_byte_demand().unwrap(),capacity_bytes:session.next_close_capacity_byte_demand(stock.maximum_copy_bytes).unwrap(),release_bytes:session.next_close_release_byte_demand().unwrap(),depth:session.next_close_depth_demand().unwrap()};retire(&mut caller,grant(d),&mut births,&mut releases,|cx|{close_original_retained_step(cx,d,|g|session.retire_source_step(g))?;Ok(())});}
  assert!(session.terminal_is_empty());assert_eq!(original+births,releases);caller.retire(stock);
 }}
}

#[test]
fn original_text_ordering_quote_and_physical_caller_match_the_same_engine(){
 let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096};let(mut births,mut releases)=(0,0);
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),stock,||Some(1000),1500,32768);let mut work=flow::FlowTextOrderingWork::new();let(left,right)=("original.源","original.源");
 let d=work.next_demands(left,right,stock.maximum_copy_bytes).unwrap();assert!(turn(&mut caller,grant(d),&mut births,&mut releases,|cx|work.step(left,right,cx)).is_none());
 let mut foreign=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),stock,||Some(1000),1500,32768);let(mut fb,mut fr)=(0,0);
 let failure=turn(&mut foreign,stock,&mut fb,&mut fr,|cx|Ok(work.step(left,right,cx).expect_err("text comparison must retain its initiating physical caller")));close_pending_owned(failure,&mut foreign,stock,&mut fb,&mut fr);assert_eq!(fb,fr);foreign.retire(stock);
 let mut result=None;for _ in 0..200{let d=work.next_demands(left,right,stock.maximum_copy_bytes).unwrap();result=turn(&mut caller,grant(d),&mut births,&mut releases,|cx|work.step(left,right,cx));if result.is_some(){break}}
 assert_eq!(result,Some(std::cmp::Ordering::Equal));work.begin_close();for _ in 0..200{if work.terminal_is_empty(){break}let d=work.close_demands();retire(&mut caller,grant(d),&mut births,&mut releases,|cx|{close_original_retained_step(cx,d,|g|work.close_step(g))?;Ok(())});}assert!(work.terminal_is_empty());assert_eq!(births,releases);caller.retire(stock);
}
