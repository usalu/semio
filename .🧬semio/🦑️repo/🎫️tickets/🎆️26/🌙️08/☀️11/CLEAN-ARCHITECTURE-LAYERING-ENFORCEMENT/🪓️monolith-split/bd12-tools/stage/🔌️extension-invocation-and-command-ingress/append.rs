
/// 🔌️ Advances the retained extension invocation and the extension retirement under the mounted close policy; reports whether extension retirement still has work.
pub(super) async fn drive_extension_turn<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, effects: &mut Vec<Effect>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    if let Some(effect)=advance_retained_extension_invocation(cx).await?{effects.push(effect);}
    let original_extension_grant = runtime.mounted_owner_policy().close;
    let remaining=cx.retained_grant();
    let extension_grant = semio_framework_value::RetainedCloneGrant { maximum_items: original_extension_grant.maximum_items.min(remaining.maximum_items),maximum_copy_bytes:original_extension_grant.maximum_copy_bytes.min(remaining.maximum_copy_bytes),maximum_capacity_bytes:original_extension_grant.maximum_capacity_bytes.min(remaining.maximum_capacity_bytes),maximum_release_bytes:original_extension_grant.maximum_release_bytes.min(remaining.maximum_release_bytes),maximum_depth:original_extension_grant.maximum_depth.min(remaining.maximum_depth) };
    let (extension_retirement_work, _) = cx.with_retained_loan(extension_grant,|same|semio_framework_plugin_extension::extension_runtime_lift::extension_retirement_turn(same))?;
    Ok(extension_retirement_work)
}
