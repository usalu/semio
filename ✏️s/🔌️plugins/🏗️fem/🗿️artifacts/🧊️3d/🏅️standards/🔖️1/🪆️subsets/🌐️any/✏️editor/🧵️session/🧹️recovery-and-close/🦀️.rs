//! 🧹️ Recovery and close of the FEM 3D session: the abandoned instance recovery, retirement, maintenance and the close steps. See [`super`].

use super::*;

//#region 💼️MountedSession
pub(super) fn recover_abandoned_one(registry: &mut Registry, app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Option<PluginLifecycleStep> {
    let shell = registry.recovery_cursor;
    registry.recovery_cursor = (registry.recovery_cursor + 1) % SHELL_CAPACITY;
    let recovery = registry.recoveries[shell].clone();
    let (identity, publication) = recovery.publication.get()?;
    if identity.app_instance_id != app_instance_id || recovery.reserved.get() != Some(identity) {
        return None;
    }
    if publication == MountedRecoveryPublication::Retained {
        let retained = registry.shells[shell].try_borrow().ok().is_some_and(|owner| owner.as_ref().is_some_and(|state| state.identity == identity && state.done));
        if !retained {
            return Some(PluginLifecycleStep::Blocked { reason: "FEM3D retained job handoff identity mismatch" });
        }
        recovery.clear_publication(identity);
        return Some(session_pending(1, 0));
    }
    let mut state = recovery.take_owner(identity);
    if state.is_none() {
        let Ok(mut shell_owner) = registry.shells[shell].try_borrow_mut() else { return Some(session_pending(0, 0)) };
        if shell_owner.as_ref().is_some_and(|state| state.identity == identity) {
            let discovered = shell_owner.take();
            drop(shell_owner);
            if let Some(discovered) = discovered {
                if let Err(discovered) = recovery.publish_owner(identity, discovered) {
                    std::mem::forget(discovered);
                    return Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state recovery slot collision" });
                }
            }
            state = recovery.take_owner(identity);
        }
    }
    let Some(mut state) = state else { return Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state owner is not discoverable" }) };
    if state.identity != identity {
        std::mem::forget(state);
        return Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state generation mismatch" });
    }
    state.cancel.cancel_now();
    let step = state.close_step(cx, grant);
    if !matches!(&step, PluginLifecycleStep::Complete(_)) {
        if let Err(state) = recovery.restore_owner(identity, state) {
            std::mem::forget(state);
            return Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state restore collision" });
        }
        return Some(step);
    }
    if !state.terminal_is_empty() {
        if let Err(state) = recovery.restore_owner(identity, state) {
            std::mem::forget(state);
        }
        return Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state false terminal" });
    }
    drop(state);
    let active = identity.app_instance_id as usize % ACTIVE_CAPACITY;
    if registry.current[active].is_some_and(|current| current.identity == identity && current.shell as usize == shell) {
        registry.current[active] = None;
    }
    if registry.retiring[shell] && registry.retiring_app[shell] == identity.app_instance_id {
        registry.retiring[shell] = false;
        registry.retiring_app[shell] = 0;
        registry.retiring_count[active] = registry.retiring_count[active].saturating_sub(1);
    }
    recovery.clear_publication(identity);
    if !recovery.release(identity) {
        return Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned recovery authority false terminal" });
    }
    registry.release_credit(shell as u16);
    registry.release(shell as u16);
    Some(session_pending(1, 0))
}

fn retire_one(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let shell = registry.maintenance_cursor;
        registry.maintenance_cursor = (registry.maintenance_cursor + 1) % SHELL_CAPACITY;
        if !registry.retiring[shell] || registry.retiring_app[shell] != app_instance_id {
            return session_pending(0, 0);
        }
        let step = {
            let mut owner = registry.shells[shell].borrow_mut();
            let Some(state) = owner.as_mut() else {
                if registry.recoveries[shell].publication.get().is_some_and(|current| current.1 == MountedRecoveryPublication::Recover) {
                    return session_pending(0, 0);
                }
                return PluginLifecycleStep::Blocked { reason: "FEM3D retiring shell empty" };
            };
            state.close_step(cx, grant)
        };
        if matches!(step, PluginLifecycleStep::Complete(_)) {
            let terminal_identity = registry.shells[shell].borrow().as_ref().filter(|state| state.terminal_is_empty()).map(|state| state.identity);
            let Some(identity) = terminal_identity else {
                return PluginLifecycleStep::Blocked { reason: "FEM3D retiring state false terminal" };
            };
            *registry.shells[shell].borrow_mut() = None;
            registry.retiring[shell] = false;
            registry.retiring_app[shell] = 0;
            let slot = app_instance_id as usize % ACTIVE_CAPACITY;
            registry.retiring_count[slot] = registry.retiring_count[slot].saturating_sub(1);
            registry.recoveries[shell].clear_publication(identity);
            assert!(registry.recoveries[shell].release(identity), "FEM3D normal retirement recovery authority mismatch");
            registry.release_credit(shell as u16);
            registry.release(shell as u16);
            return session_pending(1, 0);
        }
        step
    })
}

pub fn maintenance_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
    let (maximum_items, maximum_bytes) = (grant.maximum_items, grant.maximum_release_bytes);
    if maximum_items == 0 {
        return session_pending(0, 0);
    }
    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, cx, grant)) {
        return step;
    }
    if let Some((items, bytes)) = world3d_snapshot_recovery_close_step(maximum_bytes) {
        return session_pending(items, bytes);
    }
    if let Some((items, bytes)) = close_recovered_fem3d_backing(maximum_bytes) {
        return session_pending(items, bytes);
    }
    retire_one(app_instance_id, cx, grant)
}

/// 📏️ Quotes the next close or maintenance turn: one owner release bounded by the largest session owner.
pub fn close_demands(app_instance_id: u32) -> RetirementDemand {
    if terminal_is_empty(app_instance_id) {
        RetirementDemand::default()
    } else {
        RetirementDemand { copy_bytes: SESSION_CLOSE_COPY_BYTES, release_bytes: SESSION_CLOSE_RELEASE_BYTES, depth: SESSION_CLOSE_DEPTH, ..Default::default() }
    }
}

pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
    let (maximum_items, maximum_bytes) = (grant.maximum_items, grant.maximum_release_bytes);
    if maximum_items == 0 {
        return session_pending(0, 0);
    }
    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, cx, grant)) {
        return step;
    }
    if let Some((items, bytes)) = world3d_snapshot_recovery_close_step(maximum_bytes) {
        return session_pending(items, bytes);
    }
    if let Some((items, bytes)) = close_recovered_fem3d_backing(maximum_bytes) {
        return session_pending(items, bytes);
    }
    if terminal_is_empty(app_instance_id) {
        return PluginLifecycleStep::Complete(RetainedCloneProgress::default());
    }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let slot = app_instance_id as usize % ACTIVE_CAPACITY;
        if registry.pending[slot].as_ref().is_some_and(|pending| pending.render.app_instance_id == app_instance_id) {
            registry.pending[slot] = None;
            return session_pending(1, 0);
        }
        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
            if !registry.retain_retiring(app_instance_id, current.shell) {
                return PluginLifecycleStep::Blocked { reason: "FEM3D close retirement capacity" };
            }
            if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
                if let Some(state) = owner.as_ref() {
                    state.cancel.cancel_now();
                }
            }
            registry.current[slot] = None;
            return session_pending(1, 0);
        }
        drop(registry);
        retire_one(app_instance_id, cx, grant)
    })
}

pub fn terminal_is_empty(app_instance_id: u32) -> bool {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let slot = app_instance_id as usize % ACTIVE_CAPACITY;
        registry.pending[slot].as_ref().is_none_or(|pending| pending.render.app_instance_id != app_instance_id)
            && registry.current[slot].is_none_or(|current| current.app_instance_id != app_instance_id)
            && (registry.retiring_owner[slot] != app_instance_id || registry.retiring_count[slot] == 0)
            && registry.recoveries.iter().all(|recovery| !recovery.contains(app_instance_id))
    })
}
//#endregion 💼️MountedSession
