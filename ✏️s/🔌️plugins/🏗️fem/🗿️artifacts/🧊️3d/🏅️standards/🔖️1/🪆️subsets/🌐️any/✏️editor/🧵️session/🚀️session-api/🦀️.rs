//! 🚀️ Session API of the FEM 3D session: the initialization, snapshot read, reconcile and the live visual and solver publication. See [`super`].

use super::*;

//#region 💼️MountedSession
pub fn initialize() {
    MOUNTED.with(|registry| {
        let _ = registry.borrow().free_len;
    });
    semio_framework_plugin::reactor::jobs::register_bounded_job_kind(FEM3D_MOUNTED_VISUAL_JOB_KIND, BoundedJobFactory { admit: job_admit, demands: job_demands });
}

pub(super) fn current_identity(app_instance_id: u32) -> Option<Identity> {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        registry.current[app_instance_id as usize % ACTIVE_CAPACITY].filter(|current| current.app_instance_id == app_instance_id).map(|current| current.identity)
    })
}

pub fn prepare_snapshot_read(render: AppRenderOperationContext, snapshot: &Fem3dSnapshot) -> bool {
    if render.app_instance_id == 0 || snapshot.solids.len() > MAXIMUM_REGIONS || snapshot.nodes.len() > MAXIMUM_NODES || snapshot.elements.len() > MAXIMUM_ELEMENTS || snapshot.supports.len() > MAXIMUM_SUPPORTS {
        return false;
    }
    let slot = render.app_instance_id as usize % ACTIVE_CAPACITY;
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        if registry.current[slot].is_some_and(|current| current.app_instance_id != render.app_instance_id) {
            return false;
        }
        if registry.current[slot].is_some_and(|current| current.identity.base_revision == render.base_revision && current.identity.generation == render.generation) {
            return false;
        }
        let matches = |pending: PendingSnapshot| {
            pending.render.app_instance_id == render.app_instance_id && pending.render.base_revision == render.base_revision && pending.render.generation == render.generation && pending.render.canonical_base_revision == render.canonical_base_revision
        };
        if !registry.pending[slot].is_some_and(matches) {
            registry.pending[slot] = Some(PendingSnapshot { render, preflight: SnapshotPreflight::new() });
        }
        let Some(pending) = registry.pending[slot].as_mut() else { return false };
        let mut outcome = Ok(false);
        for _ in 0..PREFLIGHT_UNITS_PER_OPPORTUNITY {
            outcome = pending.preflight.step_one(snapshot);
            if outcome != Ok(false) {
                break;
            }
        }
        match outcome {
            Ok(complete) => complete,
            Err(()) => {
                registry.pending[slot] = None;
                false
            }
        }
    })
}

pub fn reconcile(doc: &ArtifactView<'_, Fem3dSnapshot>) -> Vec<Effect> {
    let Some(render) = doc.render_operation() else { return Vec::new() };
    let slot = render.app_instance_id as usize % ACTIVE_CAPACITY;
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(pending) = registry.pending[slot].filter(|pending| {
            pending.render.app_instance_id == render.app_instance_id
                && pending.render.base_revision == render.base_revision
                && pending.render.generation == render.generation
                && pending.render.canonical_base_revision == render.canonical_base_revision
                && pending.preflight.stage == SnapshotPreflightStage::Complete
        }) else {
            return Vec::new();
        };
        let Ok(credit) = pending.preflight.credit() else { return Vec::new() };
        let previous = registry.current[slot];
        if let Some(previous) = previous {
            if registry.retiring[previous.shell as usize] || (registry.retiring_count[slot] != 0 && registry.retiring_owner[slot] != render.app_instance_id) {
                return Vec::new();
            }
        }
        let Some(shell) = registry.allocate() else { return Vec::new() };
        if !registry.reserve_credit(shell) {
            registry.release(shell);
            return Vec::new();
        }
        let Some(counter) = registry.next_job.checked_add(1).filter(|counter| *counter <= JOB_COUNTER_MAXIMUM) else {
            registry.release_credit(shell);
            registry.release(shell);
            return Vec::new();
        };
        let snapshot = match doc.take_snapshot_read() {
            Ok(snapshot) => snapshot,
            Err(_) => {
                registry.release_credit(shell);
                registry.release(shell);
                return Vec::new();
            }
        };
        registry.next_job = counter;
        registry.pending[slot] = None;
        let job = JOB_TAG | counter;
        let identity = Identity { app_instance_id: render.app_instance_id, base_revision: render.base_revision, generation: render.generation, canonical_base_revision: render.canonical_base_revision, operation: OperationId(job), job };
        let recovery = registry.recoveries[shell as usize].clone();
        if !recovery.reserve(identity) {
            registry.release_credit(shell);
            registry.release(shell);
            return Vec::new();
        }
        if let Some(previous) = previous {
            if !registry.retain_retiring(render.app_instance_id, previous.shell) {
                assert!(recovery.release(identity));
                registry.release_credit(shell);
                registry.release(shell);
                return Vec::new();
            }
        }
        let former = previous.and_then(|previous| registry.shells[previous.shell as usize].try_borrow_mut().ok()?.as_mut()?.current.take());
        if let Some(previous) = previous {
            if let Ok(owner) = registry.shells[previous.shell as usize].try_borrow() {
                if let Some(state) = owner.as_ref() {
                    state.cancel.cancel_now();
                }
            }
        }
        *registry.shells[shell as usize].borrow_mut() = Some(MountedState::new(identity, snapshot, former, credit, recovery));
        registry.current[slot] = Some(Current { app_instance_id: render.app_instance_id, shell, identity });
        let mut effects = Vec::with_capacity(2);
        if let Some(previous) = previous {
            effects.push(Effect::CancelJob { job: previous.identity.job });
        }
        effects.push(Effect::SpawnJob { job, kind: FEM3D_MOUNTED_VISUAL_JOB_KIND.to_string(), input: encode_input(shell, identity), placement: JobPlacement::Isolated });
        eprintln!(
            "[TRACE] fem3d live_visual reconcile spawn: instance={} revision={} generation={} shell={shell} job={job:#x} meshItems={} drawInstances={} cancelled={}",
            render.app_instance_id,
            render.base_revision.0,
            render.generation.0,
            credit.item_count,
            credit.draw_count,
            effects.len() - 1
        );
        effects
    })
}

pub fn with_live_visual<R>(render: Option<AppRenderOperationContext>, build: impl FnOnce(Option<&Fem3dPageVisualLease>) -> R) -> R {
    let Some(render) = render else { return build(None) };
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let current = registry.current[render.app_instance_id as usize % ACTIVE_CAPACITY]?;
        if current.app_instance_id != render.app_instance_id || current.identity.base_revision != render.base_revision || current.identity.generation != render.generation {
            return None;
        }
        Some(registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return build(None) };
    let Ok(owner) = shell.try_borrow() else { return build(None) };
    build(owner.as_ref().and_then(|state| state.current.as_ref().filter(|lease| lease.matches(state.identity.freshness(state.preview_sequence)))))
}

pub fn publish_solver_scalar(render: AppRenderOperationContext, index: usize, scalar: Fem3dSolverScalar) -> Result<(), Fem3dSolverScalar> {
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let current = registry.current[render.app_instance_id as usize % ACTIVE_CAPACITY]?;
        (current.app_instance_id == render.app_instance_id && current.identity.base_revision == render.base_revision && current.identity.generation == render.generation).then(|| registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return Err(scalar) };
    let Ok(mut owner) = shell.try_borrow_mut() else { return Err(scalar) };
    let Some(state) = owner.as_mut() else { return Err(scalar) };
    let freshness = state.identity.freshness(state.preview_sequence);
    state.solver.as_mut().ok_or(scalar)?.publish_scalar(freshness, index, scalar)
}

pub fn publish_solver_progress(render: AppRenderOperationContext, state_value: Fem3dVisualState, residual_norm: f64, tolerance: f64, completed: usize, total: usize) -> bool {
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let current = registry.current[render.app_instance_id as usize % ACTIVE_CAPACITY]?;
        (current.app_instance_id == render.app_instance_id && current.identity.base_revision == render.base_revision && current.identity.generation == render.generation).then(|| registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return false };
    let Ok(mut owner) = shell.try_borrow_mut() else { return false };
    let Some(state) = owner.as_mut() else { return false };
    let freshness = state.identity.freshness(state.preview_sequence);
    state.solver.as_mut().is_some_and(|solver| solver.publish_progress(freshness, state_value, residual_norm, tolerance, completed, total))
}
//#endregion 💼️MountedSession
