//! 📇️ Procedural instances finance their genuine shared catalogue through the existing lifecycle loan.
use semio_os_flow::registry::FlowOperatorCatalogueOwner;
use semio_plugin::{PluginLifecycleStep,PluginLifecycleFailure};
use semio_framework_job::StepContext;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
fn delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{
 let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original catalogue lifecycle progress decreased");
 Ok(RetainedCloneProgress{copied_items:after.copied_items.checked_sub(before.copied_items).ok_or_else(invalid)?,copied_bytes:after.copied_bytes.checked_sub(before.copied_bytes).ok_or_else(invalid)?,retained_capacity_bytes:after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes).ok_or_else(invalid)?,released_bytes:after.released_bytes.checked_sub(before.released_bytes).ok_or_else(invalid)?})
}
pub(crate) fn step(owner:&mut FlowOperatorCatalogueOwner,closing:bool,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
 if closing&&owner.terminal_is_empty()||!closing&&owner.ready(){return Ok(PluginLifecycleStep::Complete(Default::default()))}
 let loan=semio_plugin::app::original_lifecycle_grant(cx,grant);
 cx.with_retained_loan(loan,|cx|{
  let before=cx.retained_progress();
  if closing{owner.begin_close();let d=owner.close_demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,d,|g|owner.close_step(g)))?;}
  else{owner.step(cx)?;}
  let progress=delta(before,cx.retained_progress())?;
  Ok(if closing&&owner.terminal_is_empty()||!closing&&owner.ready(){PluginLifecycleStep::Complete(progress)}else{PluginLifecycleStep::Progress(progress)})
 }).map_err(PluginLifecycleFailure::from)
}
