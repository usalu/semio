import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),f="🧰️framework/🔨️modules/🧵️job/🛬️receiving/🧪️tests/🦀️.rs",s=readFileSync(f,"utf8");let n=s;function region(a:string,b:string,z:string){const i=n.indexOf(a),j=n.indexOf(b,i+a.length);if(i<0||j<0)throw Error("region "+a);n=n.slice(0,i)+z+n.slice(j)}
region("fn receive_step(","fn close_decoder(",`fn source_received(outcome:Option<OriginalNativeReceivingOutcome>){let outcome=outcome.expect("funded original Source outcome is retained");assert!(outcome.call.is_ok(),"original decoder call refused");assert!(outcome.source_refusal.is_none(),"independent Source posting refused")}
fn receive_step(cx:&mut StepContext<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>)->Result<RetainedCloneStep,OriginalTurnFailure>{
 let grant=cx.retained_grant();let mut outcome=None;let mut child=None;
 if !cx.with_native_receiving(&mut outcome,|control|{child=Some(operation(control,grant));Ok(())})?{assert!(outcome.is_none()&&child.is_none());return Ok(RetainedCloneStep::Progress(Default::default()))}
 source_received(outcome);child.expect("original child result was retained before Source posting").result.map_err(|primary|native_failure(primary,None))
}
`);
region("fn copy_native_text(","#[test]\nfn original_context_charges",`fn copy_native_text(caller:&mut OriginalJobFixtureCaller,owner:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,text:&str)->String{
 use semio_framework_value_native_owned::native_decoding::NativeReceivingEffect;
 let mut admission=RetainedCloneSource::<String>::admission_owned(text.to_owned(),());let mut source=None;let mut effect=NativeReceivingEffect::default();
 for _ in 0..10000{
  if source.is_some()&&!effect.has_pending_recovery(){break}
  caller.turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,_|{
   let turn=if effect.has_pending_recovery(){control.finish_receiving(&mut effect)}else{let demand=admission.next_demand().unwrap();let ready=admission.phase()=="ready";let work=if ready{1}else{admission.advance_work_demand().unwrap()};control.receive_allocation(demand,&mut effect,demand.capacity_bytes!=0,work,|g,actual|{if ready{admission.take_output(&mut source,g)}else{let mut none=None;admission.advance(g,actual.unwrap_or(&mut none))}})};
   RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}
  })?;Ok(())}).unwrap();
 }
 assert!(source.is_some());assert!(admission.terminal_is_empty());assert!(!effect.has_pending_recovery());
 let mut receiver=<String as NativeReceive<String>>::receiver();
 for _ in 0..10000{if receiver.output_ready(){break}caller.turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,g|{let maximum=match control.available_work_units(){Ok(n)=>n,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};receiver.advance(source.as_ref().unwrap().borrow(),control,g.maximum_copy_bytes,maximum,g)})?;Ok(())}).unwrap();}
 assert!(receiver.output_ready());let mut output=None;
 caller.turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,_|{let d=receiver.take_demand().unwrap();let q=receiver.take_work_demand().unwrap();let turn=control.receive_allocation(d,&mut effect,false,q,|g,_|{let turn=receiver.take(g);RetainedWorkOutcome{result:turn.result.map(|value|if let Some((value,p))=value{output=Some(value);RetainedCloneStep::Progress(p)}else{RetainedCloneStep::Progress(Default::default())}),work:turn.work}});RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())}).unwrap();assert!(output.is_some());
 caller.turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,_|{let d=RetirementDemand{depth:1,..Default::default()};let turn=control.receive_allocation(d,&mut effect,false,1,|g,_|{if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}RetainedWorkOutcome{result:receiver.begin_close().map(|()|RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}});RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())}).unwrap();
 let mut close_effect=NativeReceivingEffect::<semio_framework_value::OriginalRetirementFailure>::default();
 for _ in 0..10000{if receiver.terminal_is_empty()&&!close_effect.has_pending_recovery(){break}caller.turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,g|{let turn=if close_effect.has_pending_recovery(){control.finish_retirement_receiving(&mut close_effect)}else{let d=receiver.close_demands(g.maximum_copy_bytes).unwrap();let q=receiver.close_work_demand(g.maximum_copy_bytes).unwrap();control.receive_retirement(d,&mut close_effect,d.capacity_bytes!=0,q,|g,actual|{let mut none=None;receiver.close_step(g,actual.unwrap_or(&mut none))})};RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())}).unwrap();}
 assert!(receiver.terminal_is_empty());
 for _ in 0..10000{if source.as_ref().unwrap().terminal_is_empty()&&!close_effect.has_pending_recovery(){break}caller.turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,g|{let turn=if close_effect.has_pending_recovery(){control.finish_retirement_receiving(&mut close_effect)}else{let d=source.as_ref().unwrap().next_close_demand(g.maximum_copy_bytes).unwrap();let q=source.as_ref().unwrap().next_close_work_demand(g.maximum_copy_bytes).unwrap();control.receive_retirement(d,&mut close_effect,d.capacity_bytes!=0,q,|g,actual|{let mut none=None;source.as_mut().unwrap().close_step(g,actual.unwrap_or(&mut none))})};RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())}).unwrap();}
 assert!(source.as_ref().unwrap().terminal_is_empty());assert!(!close_effect.has_pending_recovery());drop(source);output.take().unwrap()
}
`);
n=n.replaceAll("maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:","maximum_items:4,maximum_copy_bytes:4096,maximum_capacity_bytes:");
const q=join(t,"🗑️generated/source-helper-normal.rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent Source laws");renameSync(q,f);console.log("Source native text helper now uses full outcome, pre-effect frames and actual independent work");
