const fs=require('fs'),f='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs',s=fs.readFileSync(f,'utf8'),x='pub fn poll_owned<';const body=`fn poll_owned_step<PA:semio_plugin_reactor_app::PluginApp+'static>(runtime:&semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,budget:kernel::OriginalJobBudget,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
 super::with_core_wire_reserve(cx,152,|cx|{
  ORIGINAL_OWNED_OUTPUT.with(|output|{
   let mut output=output.try_borrow_mut().map_err(|_|busy())?;
   if output.get().is_some(){if output.capture_step::<OriginalTurnFailure>(cx,|_|Err(busy().into()))?{output.encode_step(cx)?;}return Ok(())}
   ORIGINAL_OWNED_INPUT.with(|slot|{
    let mut slot=slot.try_borrow_mut().map_err(|_|busy())?;let owner=slot.as_mut().ok_or_else(busy)?;
    if !owner.prepare_poll_event(cx)?{return Ok(())}
    owner.with_poll_step(cx,|poll,event,cx|{
     let authority=kernel::OriginalJobBudget{retained_policy:cx.retained_grant(),fuel:cx.fuel_remaining(),deadline_us:cx.deadline_us(),..budget};let kernel_budget=poll.limits.narrow(authority);
     output.capture_step(cx,|cx|{
      let mut result=None;semio_plugin_abi::operation_authority::with_actor_operation_authority(|identity|{result=Some(semio_plugin_runtime_ops::ops_typed_operations::drive_self_waking_ready(semio_plugin_reactor_turn::poll_kernel(runtime,event,None,poll.cold_pair_page.as_ref(),kernel_budget,identity,cx)));Ok(())})?;
      result.ok_or_else(||OriginalTurnFailure::from(busy()))?
     })?;Ok(())
    })?;Ok(())
   })
  })
 })
}
`;if(s.split(x).length!==2)throw Error('anchor');let p=f+'.poll-step';fs.writeFileSync(p,s.replace(x,body+x),{flag:'wx'});if(fs.readFileSync(f,'utf8')!==s){fs.unlinkSync(p);throw Error('concurrent change')}fs.renameSync(p,f);