//! 💼️ Job bridge and live mount: the bounded job bridge with the input codec and the live mount entries that initialize, prepare, reconcile, retire and close the session. See [`super`].

use super::*;

//#region 💼️JobBridge
struct MountedBoundedJob {
    pub(super) shell: Rc<RefCell<Option<MountedState>>>,
    pub(super) identity: MountedIdentity,
    input: Option<Vec<u8>>,
    restored: Option<Vec<u8>>,
}

impl BoundedJob for MountedBoundedJob {
    /// ⏱️ Drives exact session units back to back until the host's step ceiling elapses, a unit
    /// publishes output, or the session leaves `Running` — the host owes the job one round trip per
    /// ceiling, never one per unit.
    fn step(&mut self, budget: JobBudget, _original: &mut IoRunControl<'_, '_>, _snapshot: &mut SqliteSnapshotControl<'_>, cx: &mut StepContext<'_>) -> Result<JobStep, ValueError> {
        let Ok(mut shell) = self.shell.try_borrow_mut() else { return Ok(JobStep::Running(None)) };
        let Some(state) = shell.as_mut() else { return Ok(JobStep::Failed(b"fem2d.session-owner-missing".to_vec())) };
        if state.identity != self.identity {
            return Ok(JobStep::Failed(b"fem2d.session-aba".to_vec()));
        }
        let Some(started) = semio_framework_job::default_now_us() else { return Ok(state.step(budget, cx)) };
        let ceiling_ms = u64::from(budget.deadline_ms).min(SESSION_STEP_CEILING_MS);
        let deadline = started.saturating_add(ceiling_ms.saturating_mul(1_000));
        let mut fuel = budget.fuel;
        let mut step = JobStep::Running(None);
        for _ in 0..SESSION_UNITS_PER_STEP {
            let now = semio_framework_job::default_now_us().unwrap_or(deadline);
            if fuel == 0 || now >= deadline || cx.should_yield() {
                break;
            }
            let remaining_ms = u32::try_from(deadline.saturating_sub(now).div_ceil(1_000)).unwrap_or(u32::MAX).max(1);
            step = state.step(JobBudget { fuel, deadline_ms: remaining_ms }, cx);
            fuel = fuel.saturating_sub(1);
            if !matches!(step, JobStep::Running(None)) {
                break;
            }
        }
        Ok(step)
    }

    /// 🧹️ The heavy session owner is retired by the registry's own close ladder; this handle releases its two original input buffers and itself.
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, ValueError> {
        let grant = cx.retained_grant();
        let demand = self.retirement_demands(0)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(false);
        }
        if let Some(input) = self.input.take() {
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, released_bytes: input.capacity(), ..Default::default() })?;
            return Ok(false);
        }
        if let Some(restored) = self.restored.take() {
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, released_bytes: restored.capacity(), ..Default::default() })?;
            return Ok(false);
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        Ok(true)
    }

    fn retirement_demands(&self, _copy: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { release_bytes: self.input.as_ref().map_or_else(|| self.restored.as_ref().map_or(0, Vec::capacity), Vec::capacity), depth: 1, ..Default::default() })
    }

    fn cancel(&mut self) {
        if let Ok(shell) = self.shell.try_borrow() {
            if let Some(state) = shell.as_ref() {
                state.cancel.cancel_now();
            }
        }
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        let shell = self.shell.try_borrow().ok()?;
        let state = shell.as_ref()?;
        let mut bytes = Vec::with_capacity(24);
        bytes.extend_from_slice(&state.identity.operation.0.to_le_bytes());
        bytes.extend_from_slice(&state.identity.base_revision.0.to_le_bytes());
        bytes.extend_from_slice(&state.identity.generation.0.to_le_bytes());
        Some(bytes)
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        self.input.is_none() && self.restored.is_none()
    }
}

pub(super) fn encode_input(shell: u16, identity: MountedIdentity) -> Vec<u8> {
    let mut input = Vec::with_capacity(INPUT_BYTES);
    input.push(1);
    input.extend_from_slice(&shell.to_le_bytes());
    input.extend_from_slice(&identity.app_instance_id.to_le_bytes());
    input.extend_from_slice(&identity.base_revision.0.to_le_bytes());
    input.extend_from_slice(&identity.generation.0.to_le_bytes());
    input.extend_from_slice(&identity.canonical_base_revision);
    input.extend_from_slice(&identity.operation.0.to_le_bytes());
    input
}

pub(super) fn decode_input(job: u64, input: &[u8]) -> Option<(u16, MountedIdentity)> {
    if input.len() != INPUT_BYTES || input[0] != 1 {
        return None;
    }
    let shell = u16::from_le_bytes(input[1..3].try_into().ok()?);
    let app_instance_id = u32::from_le_bytes(input[3..7].try_into().ok()?);
    let base_revision = semio_framework_job::RevisionId(u64::from_le_bytes(input[7..15].try_into().ok()?));
    let generation = semio_framework_job::Generation(u64::from_le_bytes(input[15..23].try_into().ok()?));
    let canonical_base_revision = input[23..55].try_into().ok()?;
    let operation = OperationId(u64::from_le_bytes(input[55..63].try_into().ok()?));
    if operation.0 != job || job & !FEM2D_JOB_COUNTER_MAXIMUM != FEM2D_JOB_TAG {
        return None;
    }
    Some((shell, MountedIdentity { app_instance_id, base_revision, generation, canonical_base_revision, operation, job }))
}

/// 📐️ The original request transfers its two handles into the boxed job under the framework's own original-job admission quote.
fn mounted_job_demands(_job: u64, input: &Option<Vec<u8>>, restored: &Option<Vec<u8>>, _cx: &StepContext<'_>) -> Result<RetainedCloneGrant, ValueError> {
    semio_framework_plugin::reactor::jobs::original_job_admission_demands::<MountedBoundedJob>(input, restored)
}

fn mounted_job_admit(job: u64, input: &mut Option<Vec<u8>>, restored: &mut Option<Vec<u8>>, cx: &mut StepContext<'_>) -> Result<Option<Box<dyn BoundedJob>>, ValueError> {
    let bytes = input.as_deref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "fem2d.session-input"))?;
    let (shell, identity) = decode_input(job, bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "fem2d.session-input"))?;
    let shell_owner = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let shell_owner = registry.shells.get(shell as usize).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "fem2d.session-shell"))?.clone();
        let matches = shell_owner.try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity == identity));
        if !matches {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "fem2d.session-stale-factory"));
        }
        Ok(shell_owner)
    })?;
    semio_framework_plugin::reactor::jobs::admit_original_job(input, restored, cx, move |input, restored| MountedBoundedJob { shell: shell_owner, identity, input: Some(input), restored })
}

/// 🏭️ Preallocates the fixed arena at plugin installation and registers the explicit job factory.
pub fn initialize() {
    MOUNTED.with(|registry| {
        let _ = registry.borrow().free_len;
    });
    semio_framework_plugin::reactor::jobs::register_bounded_job_kind(FEM2D_MOUNTED_JOB_KIND, BoundedJobFactory { admit: mounted_job_admit, demands: mounted_job_demands });
}
//#endregion 💼️JobBridge

//#region 🎛️LiveMount
pub(super) fn current_identity(app_instance_id: u32) -> Option<MountedIdentity> {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        registry.current[slot].filter(|current| current.app_instance_id == app_instance_id).map(|current| current.identity)
    })
}

/// 🔍️ Advances exactly one schema owner/scalar census before the shared application may
/// request an opaque snapshot lease. The fixed shell and exact process credit are reserved only
/// after the census reaches its complete witness.
pub fn prepare_snapshot_read(render: AppRenderOperationContext, snapshot: &Fem2dSnapshot) -> bool {
    if render.app_instance_id == 0 {
        return false;
    }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let current_slot = render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        let previous = match registry.current[current_slot] {
            Some(current) if current.app_instance_id != render.app_instance_id => return false,
            Some(current) if current.identity.base_revision == render.base_revision && current.identity.generation == render.generation => return false,
            Some(current) => Some(current),
            None => None,
        };
        if let Some(pending) = registry.pending[current_slot] {
            if pending.app_instance_id != render.app_instance_id {
                return false;
            }
            if pending.identity.base_revision == render.base_revision && pending.identity.generation == render.generation && pending.identity.canonical_base_revision == render.canonical_base_revision {
                return true;
            }
            registry.pending[current_slot] = None;
            registry.release_credit(pending.shell);
            registry.release(pending.shell);
        }
        if previous.is_some() && !registry.retiring.iter().any(Option::is_none) {
            return false;
        }
        let matches_render = |pending: PendingSnapshotAdmission| {
            pending.app_instance_id == render.app_instance_id && pending.render.base_revision == render.base_revision && pending.render.generation == render.generation && pending.render.canonical_base_revision == render.canonical_base_revision
        };
        if let Some(fault) = registry.preflight_fault[current_slot] {
            if fault.app_instance_id == render.app_instance_id && fault.render.base_revision == render.base_revision && fault.render.generation == render.generation && fault.render.canonical_base_revision == render.canonical_base_revision {
                return false;
            }
            registry.preflight_fault[current_slot] = None;
        }
        if !registry.preflight[current_slot].is_some_and(matches_render) {
            registry.preflight[current_slot] = Some(PendingSnapshotAdmission { app_instance_id: render.app_instance_id, render, cursor: SnapshotAdmissionCursor::new() });
        }
        let completed = {
            let preflight = registry.preflight[current_slot].as_mut().expect("matching FEM preflight retained");
            let mut outcome = Ok(false);
            for _ in 0..SESSION_PREFLIGHT_UNITS_PER_OPPORTUNITY {
                outcome = preflight.cursor.step_one(snapshot);
                if outcome != Ok(false) {
                    break;
                }
            }
            match outcome {
                Ok(completed) => completed,
                Err(detail) => {
                    registry.preflight[current_slot] = None;
                    registry.preflight_fault[current_slot] = Some(PendingSnapshotFault { app_instance_id: render.app_instance_id, render, detail: std::str::from_utf8(detail).unwrap_or("fem2d.session-preflight-fault"), emitted: false });
                    return false;
                }
            }
        };
        if !completed {
            return false;
        }
        let preflight = registry.preflight[current_slot].take().expect("completed FEM preflight retained");
        let (process_items, process_bytes) = match preflight.cursor.process_credit() {
            Ok(credit) => credit,
            Err(detail) => {
                registry.preflight_fault[current_slot] = Some(PendingSnapshotFault { app_instance_id: render.app_instance_id, render, detail: std::str::from_utf8(detail).unwrap_or("fem2d.session-process-credit-fault"), emitted: false });
                return false;
            }
        };
        let Some(shell) = registry.allocate() else { return false };
        if !registry.reserve_credit(shell, process_items, process_bytes) {
            registry.release(shell);
            return false;
        }
        let Some(counter) = registry.next_job.checked_add(1).filter(|counter| *counter <= FEM2D_JOB_COUNTER_MAXIMUM) else {
            registry.release_credit(shell);
            registry.release(shell);
            return false;
        };
        registry.next_job = counter;
        let job = FEM2D_JOB_TAG | counter;
        let identity = MountedIdentity { app_instance_id: render.app_instance_id, base_revision: render.base_revision, generation: render.generation, canonical_base_revision: render.canonical_base_revision, operation: OperationId(job), job };
        registry.pending[current_slot] = Some(PendingAdmission { app_instance_id: render.app_instance_id, shell, identity, admitted_items: process_items });
        true
    })
}

/// 🔁️ Reconciles one live store revision and returns only the exact host job effects needed.
pub fn reconcile(doc: &ArtifactView<'_, Fem2dSnapshot>) -> Vec<Effect> {
    let Some(render) = doc.render_operation() else { return Vec::new() };
    if render.app_instance_id == 0 {
        return Vec::new();
    }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let current_slot = render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        if let Some(fault) = registry.preflight_fault[current_slot].as_mut().filter(|fault| {
            fault.app_instance_id == render.app_instance_id && fault.render.base_revision == render.base_revision && fault.render.generation == render.generation && fault.render.canonical_base_revision == render.canonical_base_revision
        }) {
            if !fault.emitted {
                fault.emitted = true;
                return vec![Effect::Notify { message: fault.detail.to_string() }];
            }
            return Vec::new();
        }
        let previous = if let Some(current) = registry.current[current_slot] {
            if current.app_instance_id != render.app_instance_id {
                return Vec::new();
            }
            if current.identity.base_revision == render.base_revision && current.identity.generation == render.generation {
                return Vec::new();
            }
            if !registry.retiring.iter().any(Option::is_none) {
                return Vec::new();
            }
            Some(current)
        } else {
            None
        };
        let Some(pending) = registry.pending[current_slot].filter(|pending| {
            pending.app_instance_id == render.app_instance_id && pending.identity.base_revision == render.base_revision && pending.identity.generation == render.generation && pending.identity.canonical_base_revision == render.canonical_base_revision
        }) else {
            return Vec::new();
        };
        let shell = pending.shell;
        let snapshot = match doc.take_snapshot_read() {
            Ok(snapshot) => snapshot,
            Err(_) => return Vec::new(),
        };
        registry.pending[current_slot] = None;
        if let Some(previous) = previous {
            assert!(registry.retain_retirement(previous.shell), "retirement slot was preflighted under the registry owner");
            if let Ok(shell) = registry.shells[previous.shell as usize].try_borrow() {
                if let Some(state) = shell.as_ref() {
                    state.cancel.cancel_now();
                }
            }
            registry.current[current_slot] = None;
        }
        let identity = pending.identity;
        let job = identity.job;
        *registry.shells[shell as usize].borrow_mut() = Some(MountedState::new(identity, snapshot, pending.admitted_items));
        registry.current[current_slot] = Some(CurrentSession { app_instance_id: render.app_instance_id, shell, identity });
        let mut effects = Vec::with_capacity(2);
        if let Some(previous) = previous {
            effects.push(Effect::CancelJob { job: previous.identity.job });
        }
        effects.push(Effect::SpawnJob { job, kind: FEM2D_MOUNTED_JOB_KIND.to_string(), input: encode_input(shell, identity), placement: JobPlacement::Isolated });
        effects
    })
}

/// 👁️ Borrows the immutable generation-exact mounted visual lease only while the renderer adopts it.
pub fn with_live_visual<R>(render: Option<AppRenderOperationContext>, build: impl FnOnce(Option<&Fem2dMountedVisualLease>) -> R) -> R {
    let Some(render) = render else { return build(None) };
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let current = registry.current[render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY]?;
        if current.app_instance_id != render.app_instance_id || current.identity.base_revision != render.base_revision || current.identity.generation != render.generation {
            return None;
        }
        Some(registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return build(None) };
    let Ok(owner) = shell.try_borrow() else { return build(None) };
    build(owner.as_ref().and_then(|state| state.visual_current.as_ref()).filter(|lease| lease.matches(render.app_instance_id, render.base_revision.0, render.generation.0)))
}

fn retire_one(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(index) = registry.retiring.iter().position(|shell| shell.is_some_and(|shell| registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity.app_instance_id == app_instance_id))))
        else {
            return PluginLifecycleStep::Complete(RetainedCloneProgress::default());
        };
        let shell = registry.retiring[index].expect("matched retirement shell");
        let step = {
            let mut owner = match registry.shells[shell as usize].try_borrow_mut() {
                Ok(owner) => owner,
                Err(_) => return PluginLifecycleStep::Blocked { reason: "mounted FEM job owner is checked out by its worker turn" },
            };
            let Some(state) = owner.as_mut() else { return PluginLifecycleStep::Complete(RetainedCloneProgress::default()) };
            state.close_step(cx, grant)
        };
        if matches!(step, PluginLifecycleStep::Complete(_)) {
            let terminal = registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(MountedState::terminal_is_empty));
            if !terminal {
                return PluginLifecycleStep::Blocked { reason: "mounted FEM job reported a false terminal shell" };
            }
            *registry.shells[shell as usize].borrow_mut() = None;
            registry.retiring[index] = None;
            registry.release_credit(shell);
            registry.release(shell);
            return session_pending(1, 0);
        }
        step
    })
}

pub fn maintenance_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
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
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        if registry.preflight_fault[slot].is_some_and(|fault| fault.app_instance_id == app_instance_id) {
            registry.preflight_fault[slot] = None;
            return session_pending(1, size_of::<PendingSnapshotFault>());
        }
        if registry.preflight[slot].is_some_and(|preflight| preflight.app_instance_id == app_instance_id) {
            registry.preflight[slot] = None;
            return session_pending(1, size_of::<PendingSnapshotAdmission>());
        }
        if let Some(pending) = registry.pending[slot].filter(|pending| pending.app_instance_id == app_instance_id) {
            registry.pending[slot] = None;
            registry.release_credit(pending.shell);
            registry.release(pending.shell);
            return session_pending(1, INPUT_BYTES);
        }
        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
            if !registry.retain_retirement(current.shell) {
                return PluginLifecycleStep::Blocked { reason: "mounted FEM retirement arena is saturated" };
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
        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        registry.preflight_fault[slot].is_none_or(|fault| fault.app_instance_id != app_instance_id)
            && registry.preflight[slot].is_none_or(|preflight| preflight.app_instance_id != app_instance_id)
            && registry.pending[slot].is_none_or(|pending| pending.app_instance_id != app_instance_id)
            && registry.current[slot].is_none_or(|current| current.app_instance_id != app_instance_id)
            && registry.retiring.iter().all(|shell| shell.is_none_or(|shell| registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_none_or(|state| state.identity.app_instance_id != app_instance_id))))
    })
}
//#endregion 🎛️LiveMount
