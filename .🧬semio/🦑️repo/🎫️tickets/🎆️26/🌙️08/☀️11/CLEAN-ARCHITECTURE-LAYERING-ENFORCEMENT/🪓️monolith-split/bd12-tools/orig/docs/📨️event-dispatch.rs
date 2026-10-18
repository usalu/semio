//! 📨️ Event dispatch phases of the reactor turn: the selection of the one event a turn serves, its routing, and the original receiving advance that follows it. See [`super`].

use super::*;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

/// 🎟️ Retains a completed donation and selects the one event this turn may serve under its retained grant, if any.
pub(super) fn select_turn_event<PA: PluginApp>(runtime: &PluginRuntime<PA>, events: &mut Option<Event>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<Event>, semio_framework_job::OriginalTurnFailure> {
    if matches!(events.as_ref(),Some(Event::Completed { .. })) {
        original_receiving::retain_completed(runtime,events,cx)?;
    }
    let selected_event = if matches!(events.as_ref(),Some(Event::Completed { .. })) {
        None
    } else if events.is_some() {
        let grant=cx.retained_grant();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0 { None } else {
            cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;
            cx.consume_fuel(1);
            events.take()
        }
    } else { None };
    Ok(selected_event)
}

/// 📨️ Routes the selected event to the owner it names; dirty surfaces and document effects accumulate in the caller's owners.
pub(super) async fn dispatch_turn_event<PA: PluginApp>(runtime: &PluginRuntime<PA>, selected_event: Option<Event>, dirty: &mut DirtyPollOwners, document_backbone_effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    for event in selected_event {
        match event {
            Event::InstanceOpen { request, app_id, actor, assets, quotas, .. } => {
                let instance = request.instance_id;
                let fresh = runtime.guest_lifetimes.borrow_mut().admit(request).map_err(reactor_close_fault)?;
                if fresh {
                    semio_framework_plugin_reactor_core::admit_package_assets(assets);
                    if let Err(error) = INSTANCE_METADATA.with(|metadata| metadata.borrow_mut().insert(instance, app_id.0.clone(), quotas)) {
                        runtime.guest_lifetimes.borrow_mut().remove_uncreated(instance).map_err(reactor_close_fault)?;
                        return Err(error.into());
                    }
                    semio_framework_plugin_runtime_instances::inst_opening::plugin_open_actor_instance(runtime,request,app_id.0,actor,identity,cx).await?;
                }
            }
            Event::InstanceClose(_) | Event::InstanceLifecycleAck(_) => {}
            Event::CommandIngressPage { .. } => {
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-page-event-bypass"), "command page must use poll_kernel's dedicated owner argument").into());
            }
            Event::ColdDocumentPairPage(_) => {
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.cold-pair-page-event-bypass"), "cold document pair page must use poll_kernel's dedicated owner argument").into());
            }
            Event::UiIntent { instance, intent } => ui_intent_event(runtime, instance, intent, dirty)?,
            Event::SurfaceVisible { surface, body_key, view_state } => {
                if let Some(instance) = parse_surface_instance(&surface) {
                    native_close_key(runtime, instance)?;
                    semio_framework_plugin_runtime_ops::ops_surface_render::plugin_mount_surface(runtime, instance, surface.clone(), body_key, &view_state).await?;
                    let surface =
                        ui_contract::SurfaceId::try_from(surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                    dirty.try_surface(instance, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                }
            }
            Event::SurfaceHidden { surface } => {
                if let Some(instance) = parse_surface_instance(&surface) {
                    if native_close_key(runtime, instance).is_ok() {
                        semio_framework_plugin_runtime_ops::ops_surface_render::plugin_hide_surface(runtime, instance, &surface).await?;
                    }
                }
            }
            Event::SurfaceResized { .. } => {}
            Event::PatchAck { receipt, surface, revision } => {
                if !live_patch_receipt(runtime, receipt) {
                    continue;
                }
                with_pending_patches(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| PATCHES.with(|patches| patches.mark_published_ack(ack))))
                    .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.patch-ack-authority"), reason))?;
            }
            Event::PatchRejected { receipt, surface, revision, .. } => {
                if !live_patch_receipt(runtime, receipt) {
                    continue;
                }
                with_pending_patches(|pending| pending.borrow_mut().apply_issued_rejection(receipt, &surface, revision, |generation| PATCHES.with(|patches| patches.mark_rejected(&surface, generation))));
            }
            Event::Completed { .. } => unreachable!("Completed source donation uses the original borrowed receiver"),
            Event::HttpChunk { req, bytes, done } => {
                if append_extension_response_page(req, &bytes) {
                    continue;
                }
                REGISTRY.with(|registry| {
                    let cap = registry.instance_of(req).and_then(|instance| INSTANCE_METADATA.with(|metadata| metadata.borrow().get(instance).and_then(|entry| entry.quota.message_bytes))).unwrap_or(64 * 1024 * 1024) as usize;
                    registry.append_chunk(req, &bytes, done, cap);
                });
            }
            Event::JobProgress { job, .. } => {
                if let Some(binding) = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow().accepted(job)) {
                    dirty_background_surfaces(runtime, binding.instance, &mut *dirty).await?;
                }
            }
            Event::JobCompleted { job, result } => job_completed_event(runtime, job, result, dirty, document_backbone_effects, identity, cx).await?,
            Event::Message { source: MessageEndpoint::Shell { instance }, payload } => {
                if let Some(token) = semio_framework_plugin_job::TypedOperationResultPage::renderer_ack_token(&payload) {
                    if instance.0.parse::<u32>().ok() == Some(token.receiver) {
                        let _ = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(runtime, token).await?;
                    }
                } else if let Some(effects) = semio_framework_plugin_runtime_ops::ops_document::plugin_handle_document_backbone_binding(runtime, &instance.0, &payload).await? {
                    document_backbone_effects.extend(effects);
                }
            }
            Event::Message { source: MessageEndpoint::Backbone { uri }, payload } => {
                let output = semio_framework_plugin_runtime_ops::ops_document::plugin_receive_document_backbone(runtime, &uri, &payload, identity, cx).await?;
                if output.document_changed {
                    dirty_document_surfaces(runtime, output.instance_id, &mut *dirty).await?;
                } else {
                    dirty_background_surfaces(runtime, output.instance_id, &mut *dirty).await?;
                }
                for frame in output.frames {
                    route_app_frame(output.instance_id, &frame, &mut *document_backbone_effects);
                }
                document_backbone_effects.extend(output.effects);
            }
            Event::Message { .. } => {}
            Event::Timer { id } => {
                ARMED_TIMERS.with(|timers| {
                    timers.borrow_mut().remove(id);
                });
                TASK_EXECUTOR.with(|executor| executor.wake(id));
            }
            Event::Wake => {}
            Event::Request { req, from, capability, payload } => {
                let owner=RetainedExtensionInvocation::new(req,from,capability,payload);
                RETAINED_EXTENSION_INVOCATIONS.with(|owners|{let mut owners=owners.borrow_mut();let slot=owners.iter_mut().find(|slot|slot.is_none()).ok_or_else(||reactor_close_fault("original extension request slots are full"))?;*slot=Some(owner);Ok::<_,semio_framework::Fault>(())})?;
            }
            Event::SuspendRequest => semio_framework_plugin_extension::extension_runtime_lift::extension_deactivate().await,
            Event::Activate { .. } | Event::CapabilityChanged { .. } | Event::QuotaChanged { .. } => {}
        }
    }
    Ok(())
}

/// 🎯️ Decodes one UI intent, drops it when its tree is stale and batches it per instance for the dispatch pass.
fn ui_intent_event<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance: semio_framework::kernel::PluginInstanceId, intent: Vec<u8>, dirty: &mut DirtyPollOwners) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    let numeric_instance = instance.0.parse::<u32>().map_err(|_| reactor_close_fault("invalid intent instance"))?;
    native_close_key(runtime, numeric_instance)?;
    if let Ok(intent_value) = store::pack_rt::decode_wire_value(&intent) {
        if let Ok(intent) = serde_json::from_value::<ui_contract::UiIntent>(intent_value.into()) {
            if parse_surface_instance(intent.surface.as_ref()) != Some(numeric_instance) {
                return Err(reactor_close_fault("intent surface names another lifetime").into());
            }
            let current_revision = PATCHES.with(|patches| patches.revision(&intent.surface.0));
            if !is_stale_intent(intent.revision, current_revision, DEFAULT_REVISION_TOLERANCE) {
                dirty
                    .try_intent(numeric_instance, intent)
                    .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-intent-capacity"), "fixed dirty intent authority is saturated"))?;
            }
        }
    }
    Ok(())
}

/// 🧬️ Completes a spawned job: resolves its render binding, hands a reserved output to its instance and resolves the parked request.
async fn job_completed_event<PA: PluginApp>(runtime: &PluginRuntime<PA>, job: u64, result: semio_framework::kernel::RequestOutcome, dirty: &mut DirtyPollOwners, document_backbone_effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    let instance = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow().owned(job).map(|binding| binding.instance));
    if let Some(binding) = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().complete(job)) {
        dirty_background_surfaces(runtime, binding.instance, &mut *dirty).await?;
    }
    let outcome = semio_framework_plugin_actor_host::outcome_to_result(result);
    if let Some(instance) = instance {
        if native_close_key(runtime, instance).is_ok() {
            let reserved_output = match &outcome {
                Ok(bytes) => Ok(bytes.clone()),
                Err(fault) => Err(fault.clone()),
            };
            let output = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_complete_reserved_spawned_job(runtime, instance, job, reserved_output, identity,cx).await?;
            route_exchange_output(instance, output, &mut *document_backbone_effects,cx)?;
        }
    }
    REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(job), outcome));
    Ok(())
}

/// 📥️ Advances the original receiving owners (intents retained, responses, intents) with the turn's context.
pub(super) async fn advance_original_receiving<PA: PluginApp>(runtime: &PluginRuntime<PA>, dirty: &mut DirtyPollOwners, effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    if !original_receiving::retain_intents(&mut dirty.intents,cx)? {
        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original intent receiving slot retains an earlier batch").into());
    }
    original_receiving::advance_responses(runtime,identity,&mut *effects,cx).await?;
    original_receiving::advance_intents(runtime,identity,&mut *effects,cx).await?;
    Ok(())
}
