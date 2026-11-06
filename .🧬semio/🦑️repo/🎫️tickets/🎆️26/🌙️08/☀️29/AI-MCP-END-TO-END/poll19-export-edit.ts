import{readFileSync,writeFileSync,renameSync}from"node:fs";
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs";let source=readFileSync(file,"utf8");
const start=source.indexOf("pub fn poll_owned<"),end=source.indexOf("\n/// ♻️ Retires the captured input",start);if(start<0||end<0||!source.slice(start,end).includes("return_json"))throw Error("expected obsolete Poll body");
const replacement=`pub fn poll_owned<PA:semio_plugin_reactor_app::PluginApp+'static>(runtime:&semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,budget:kernel::OriginalJobBudget)->u64{
 let input=budget.input;ORIGINAL_CLOCK_BINDING.with(|slot|slot.set(input.work.clock_binding));
 let result=with_entry(|entry|entry.dispatch_poll_header_step(input,budget.step_budget(),original_job_clock,|cx|{
  poll_owned_step(runtime,budget,cx)?;
  let (captured,encoded)=ORIGINAL_OWNED_OUTPUT.with(|slot|{let slot=slot.try_borrow().map_err(|_|busy())?;let captured=slot.get().is_some();let encoded=match slot.encoded(){Some(bytes)=>Some((u32::try_from(bytes.as_ptr()as usize).map_err(|_|busy())?,u32::try_from(bytes.len()).map_err(|_|busy())?)),None=>None};Ok::<_,ValueError>((captured,encoded))})?;
  let input_pending=ORIGINAL_OWNED_INPUT.with(|slot|slot.try_borrow().map(|slot|slot.as_ref().is_some_and(|owner|owner.poll_event_pending())).map_err(|_|busy()))?;
  let(pointer,length)=encoded.unwrap_or((0,0));Ok(OriginalActorFixedKind::TurnOutput{ready:encoded.is_some(),captured,input_pending,pointer,length})
 }));
 match result{Ok(Ok(Some(word)))=>word,Ok(Ok(None))=>semio_plugin_abi::operation_authority::owned_result::encode_refusal(ValueRefusalKind::WorkLimit),Ok(Err(error))|Err(error)=>semio_plugin_abi::operation_authority::owned_result::encode_refusal(error.kind)}
}
`;
source=source.slice(0,start)+replacement+source.slice(end);
const stepStart=source.indexOf("fn poll_owned_step<"),stepEnd=source.indexOf("pub fn poll_owned<",stepStart);let step=source.slice(stepStart,stepEnd);const before=" super::with_core_wire_reserve(cx,152,|cx|{\n";if(step.split(before).length!==2||!step.endsWith(" })\n}\n"))throw Error("expected one outer body reserve");step=step.replace(before,"");step=step.slice(0,-7)+"}\n";source=source.slice(0,stepStart)+step+source.slice(stepEnd);
const temp=import.meta.dirname+"/🗑️generated/poll19-export.rs";writeFileSync(temp,source);renameSync(temp,file);
