//! 🎨️ Dirty surface render and reconcile phases of the reactor turn: renders every dirty surface under its reconcile reservation and drives the reconcile opportunities of the turn. See [`super`].

use super::*;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;

/// 🎨️ Renders each dirty surface of the turn under a reserved reconcile grant, defers a surface that was refused its reservation and records presence updates.
pub(super) async fn render_dirty_surfaces<PA: PluginApp>(runtime: &PluginRuntime<PA>, dirty: DirtyPollOwners, now_ms: u64, effects: &mut Vec<Effect>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    for (instance, surface) in dirty.surfaces {
        if native_close_key(runtime, instance).is_err() {
            continue;
        }
        let surface_key = surface.as_ref().to_owned();
        let _ = semio_framework_plugin_window_kits_rich::tree_kit::take_arena_unbuilt_rows();
        let mounted = match native_close_key(runtime, instance) {
            Ok(key) => PATCHES.with(|patches| patches.reserve_mounted(surface, key)),
            Err(_) => Err(surface),
        };
        match mounted {
            Ok(mut grant) => match semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(runtime, instance, &surface_key,&mut |source,cx|grant.commit_source(source,cx),cx).await {
                Ok(Some(presence)) => {
                    let unbuilt = semio_framework_plugin_window_kits_rich::tree_kit::take_arena_unbuilt_rows();
                    ARENA_STARVED.with(|starved| starved.borrow_mut().note(instance, surface_key.as_str(), unbuilt));
                    for update in presence {
                        PRESENCE.with(|hub| {
                            let mut hub = hub.borrow_mut();
                            hub.record_own(update.surface.clone(), update.node_key.clone(), update.own, update.ttl_ms);
                            for peer in update.peers {
                                hub.record_peer(update.surface.clone(), update.node_key.clone(), peer, update.ttl_ms, now_ms);
                            }
                        });
                    }
                }
                Ok(None)=>{
                    grant.cancel_and_defer();
                }
                Err(failure)=>{
                    grant.cancel();
                    return Err(failure);
                }
            },
            Err(surface) => {
                if let Some(reason) = PATCHES.with(|patches| patches.take_unreported_reserve_refusal()).filter(|reason| reason.starts_with("registry-")) {
                    effects.push(shell_fault_effect(
                        instance,
                        &semio_framework::Fault::new(
                            semio_framework::FaultOrigin::Os,
                            semio_framework::FaultCode::new("ui.surface-reconcile-unadmitted"),
                            format!("dirty surface {surface_key} was refused its reconcile reservation by {reason}, so its render is deferred until another surface returns credit"),
                        ),
                    ));
                }
                PATCHES.with(|patches| patches.defer(surface)).map_err(|surface| {
                    semio_framework::Fault::new(
                        semio_framework::FaultOrigin::Os,
                        semio_framework::FaultCode::new("ui.dirty-surface-deferred-capacity"),
                        format!("deferred render ring is full, so dirty surface {} would never re-render", surface.as_ref()),
                    )
                })?;
            }
        }
    }
    Ok(())
}

/// 🩹️ Drives the reconcile opportunities of the turn, reports whether reconcile arms another turn and answers a render fault with a shell fault effect.
pub(super) fn drive_turn_reconcile(effects: &mut Vec<Effect>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let reconcile_work = PATCHES
        .with(|patches| -> Result<bool,semio_framework_job::OriginalTurnFailure> {
            drive_reconcile_within(patches, reconcile_step_opportunities(cx.fuel_remaining()), cx)?;
            Ok(reconcile_arms_turn(patches))
        })
        ?;
    if let Some((instance, message)) = PATCHES.with(patches::PatchTracker::take_render_fault) {
        effects.push(shell_fault_effect(instance, &semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new(SURFACE_RENDER_FAULT_CODE), message)));
    }
    Ok(reconcile_work)
}
