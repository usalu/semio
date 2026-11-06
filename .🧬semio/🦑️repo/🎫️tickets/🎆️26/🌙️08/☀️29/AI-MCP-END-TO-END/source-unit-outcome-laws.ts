import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),b="🧰️framework/🔨️modules/🧵️job/🛬️receiving/",f=b+"🧪️tests/🦀️.rs",s=readFileSync(f,"utf8");let n=s;function region(a:string,b:string,z:string){const i=n.indexOf(a),j=n.indexOf(b,i+a.length);if(i<0||j<0)throw Error("region "+a);n=n.slice(0,i)+z+n.slice(j)}
n=n.replace("let grant=cx.retained_grant();let mut outcome=None;let mut child=None;","let mut outcome=None;let mut child=None;").replace("cx.with_native_receiving(&mut outcome,|control|{child=Some(operation(control,grant));","cx.with_native_receiving(&mut outcome,|control,body_grant|{child=Some(operation(control,body_grant));").replaceAll("cx.with_native_retirement_receiving(&mut outcome,|control|","cx.with_native_retirement_receiving(&mut outcome,|control,body_grant|").replaceAll("retire_original_text(&mut raw,&mut close,&mut frame,control,grant)","retire_original_text(&mut raw,&mut close,&mut frame,control,body_grant)").replace("cx.with_native_retirement_receiving(&mut denied,|_|","cx.with_native_retirement_receiving(&mut denied,|_,_|");
region("#[test]\nfn original_receiving_ready_scope","#[test]\nfn native_receiving_uses",`#[test]
fn original_receiving_ready_scope_settles_a_ready_step_and_refuses_a_suspended_one(){
 let stock=bank();let grant=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(76),Generation(5),stock,logical_now_us,u64::MAX,10000);let mut owner=OriginalJobDecodeOwner::empty();
 caller.turn_with_decoder(grant,&mut owner,|cx|{
  let mut outcome=None;let mut value=None;assert!(cx.with_native_receiving_ready(&mut outcome,async|_,_|{value=Some(7u8);Ok(())})?);assert_eq!(value,Some(7));source_received(outcome);
  let mut suspended=None;assert!(cx.with_native_receiving_ready(&mut suspended,async|_,_|std::future::pending::<Result<(),ValueError>>().await)?);
  let refused=suspended.expect("suspension refusal stays in its preborn Source outcome");assert!(refused.source_refusal.is_none());let call=refused.call.err().expect("a suspended native step must refuse");let primary=call.context.as_ref().or(call.operation.as_ref()).expect("original suspension cause is retained");assert_eq!(primary.kind,ValueRefusalKind::InvariantViolated);assert!(call.output.is_none());Ok(())
 }).unwrap();close_decoder(&mut caller,&mut owner,grant);caller.retire(stock);
}

`);

region("#[test]\nfn native_receiving_uses","#[test]\nfn original_context_accepts",`#[test]
fn native_receiving_uses_the_actual_current_cpu_loan_without_a_second_debit(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/case.schema.json")).unwrap();for(k,v)in row.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v)}
 let stock=bank();let mut caller=OriginalJobFixtureCaller::admit(OperationId(7680),Generation(3),stock,||Some(0),10000,10000);let mut owner=OriginalJobDecodeOwner::empty();let bytes:Vec<u8>=serde_json::from_value(row["probe"].clone()).unwrap();
 caller.turn_with_decoder(stock,&mut owner,|cx|cx.with_original_work_loan(row["quantum"].as_u64().unwrap(),10000,|cx|{
  let before=cx.original_work_units_spent();let mut outcome=None;let mut child=None;
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_native_receiving(&mut outcome,|control,_|{
   let start=control.actual_work();assert_eq!(control.available_work_units()?,row["childQuantum"].as_u64().unwrap());assert!(control.prepare_work(1)?);let valid=std::str::from_utf8(&bytes).is_ok();control.commit_work(1)?;assert_eq!(control.prepare_work(1)?,row["postScopeAdditionalWork"].as_bool().unwrap());child=Some((valid,control.actual_work()-start));Ok(())
  }));assert!(result?);source_received(outcome);assert!(!heap.overflowed);assert_eq!([heap.requested_bytes,heap.released_bytes],[0,0]);let(valid,work)=child.expect("native probe completed inside the actual Source loan");assert_eq!(valid,row["valid"].as_bool().unwrap());assert_eq!(work,row["actualWork"].as_u64().unwrap());assert_eq!(cx.original_work_units_spent()-before,u128::from(row["sourceWork"].as_u64().unwrap()));Ok(())
 })).unwrap();close_decoder(&mut caller,&mut owner,stock);caller.retire(stock);
}

`);
region("#[test]\nfn original_context_accepts","#[test]\nfn cancelled_source_recovers",`#[test]
fn original_context_accepts_metadata_of_a_wide_loan_without_inventing_allocation(){
 use semio_framework_value_native_owned::native_decoding::NativeReceivingEffect;
 let stock=RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:64<<20,maximum_capacity_bytes:64<<20,maximum_release_bytes:64<<20,maximum_depth:128};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(72),Generation(3),stock,logical_now_us,u64::MAX,1000000);let mut owner=OriginalJobDecodeOwner::empty();let mut frame=NativeReceivingEffect::default();
 caller.turn_with_decoder(stock,&mut owner,|cx|{let issued=cx.retained_grant();assert!(issued.maximum_capacity_bytes>=26000);let before=cx.retained_progress();let step=receive_step(cx,|control,_|{let d=RetirementDemand{copy_bytes:1,depth:1,..Default::default()};let turn=control.receive_allocation(d,&mut frame,false,1,|g,_|{if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let mut byte=[0];byte.copy_from_slice(&[1]);assert_eq!(byte,[1]);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})),work:1}});RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;
  assert_eq!(step.progress().retained_capacity_bytes,0);assert_eq!(delta(cx.retained_progress(),before).retained_capacity_bytes,0);assert_eq!(step.progress().copied_bytes,1);Ok(())
 }).unwrap();assert!(!frame.has_pending_recovery());close_decoder(&mut caller,&mut owner,stock);caller.retire(stock);
}

`);
const q=join(t,"🗑️generated/source-unit-outcome-laws.rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent Source laws");renameSync(q,f);
const fixture=b+"🎟️work/case.json",schema=b+"🎟️work/case.schema.json",a=readFileSync(fixture,"utf8"),z=readFileSync(schema,"utf8"),law=JSON.parse(a),definition=JSON.parse(z);law.quantum=3;law.childQuantum=1;law.sourcePlacementWork=2;law.sourceWork=3;for(const[k,v]of Object.entries(law)){if(!definition.required.includes(k))definition.required.push(k);definition.properties[k]={const:v}}
for(const[path,old,next]of[[fixture,a,JSON.stringify(law,null,2)+"\n"],[schema,z,JSON.stringify(definition,null,2)+"\n"]]){const tmp=join(t,"🗑️generated/source-unit-"+Date.now()+".json");writeFileSync(tmp,next);if(readFileSync(path,"utf8")!==old)throw Error("concurrent workfixture");renameSync(tmp,path)}
console.log("Ready/work/metadata Source laws use full outcomes, exact body grant, automatic single posting");
