//! ⚙️ Mounted job of the FEM 3D session: the bounded job, its input codec and the admission helpers. See [`super`].

use super::*;

//#region 💼️MountedSession
pub(super) struct MountedJob {
    pub(super) shell: Rc<RefCell<Option<MountedState>>>,
    pub(super) recovery: Rc<MountedRecoverySlot>,
    pub(super) identity: Identity,
    pub(super) completed: bool,
    pub(super) input: Option<Vec<u8>>,
    pub(super) restored: Option<Vec<u8>>,
}

impl BoundedJob for MountedJob {
    /// ⏱️ Drives exact session units back to back until the host's step ceiling elapses, a unit
    /// publishes output, or the session leaves `Running`.
    fn step(&mut self, _original: &mut IoRunControl<'_, '_>, _snapshot: &mut SqliteSnapshotControl<'_>, cx: &mut StepContext<'_>) -> Result<JobStep, semio_framework_job::OriginalTurnFailure> {
        let Ok(mut shell) = self.shell.try_borrow_mut() else { return Ok(JobStep::Running(None)); };
        let Some(state) = shell.as_mut().filter(|state| state.identity == self.identity) else { return Ok(JobStep::Failed(b"fem3d.visual-stale-shell".to_vec())); };
        let started = semio_framework_job::default_now_us();
        let deadline = started.map(|started| started.saturating_add(STEP_CEILING_MS.saturating_mul(1_000)).min(cx.deadline_us()));
        let mut step = JobStep::Running(None);
        for _ in 0..UNITS_PER_STEP {
            if cx.should_yield() { break; }
            if let Some(deadline) = deadline { if semio_framework_job::default_now_us().unwrap_or(deadline) >= deadline { break; } }
            step = state.step(cx)?;
            if deadline.is_none() || !matches!(step, JobStep::Running(None)) { break; }
        }
        if matches!(&step, JobStep::Done(_)) {
            record_terminal(self.identity, semio_framework_value::RequestTerminalKind::Validated);
            self.completed = true;
        }
        Ok(step)
    }

    /// 🧹️ The heavy session owner is retired by the registry's own close ladder; this handle releases its two original input buffers and itself.
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
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
        record_terminal(self.identity, semio_framework_value::RequestTerminalKind::Cancelled);
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

impl Drop for MountedJob {
    fn drop(&mut self) {
        if self.completed || self.shell.try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity == self.identity && state.done)) {
            record_terminal(self.identity, semio_framework_value::RequestTerminalKind::Validated);
            let _ = self.recovery.publish(self.identity, MountedRecoveryPublication::Retained);
            return;
        }
        if !self.recovery.publish(self.identity, MountedRecoveryPublication::Recover) {
            return;
        }
        self.cancel();
        let Ok(mut shell) = self.shell.try_borrow_mut() else { return };
        let Some(state) = shell.as_ref().filter(|state| state.identity == self.identity) else { return };
        state.cancel.cancel_now();
        let Some(state) = shell.take() else { return };
        if let Err(state) = self.recovery.publish_owner(self.identity, state) {
            *shell = Some(state);
        }
    }
}

pub(super) fn encode_input(shell: u16, identity: Identity) -> Vec<u8> {
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

fn decode_input(job: u64, input: &[u8]) -> Option<(u16, Identity)> {
    if input.len() != INPUT_BYTES || input[0] != 1 {
        return None;
    }
    let shell = u16::from_le_bytes(input[1..3].try_into().ok()?);
    let app_instance_id = u32::from_le_bytes(input[3..7].try_into().ok()?);
    let base_revision = RevisionId(u64::from_le_bytes(input[7..15].try_into().ok()?));
    let generation = Generation(u64::from_le_bytes(input[15..23].try_into().ok()?));
    let canonical_base_revision = input[23..55].try_into().ok()?;
    let operation = OperationId(u64::from_le_bytes(input[55..63].try_into().ok()?));
    if operation.0 != job || job & !JOB_COUNTER_MAXIMUM != JOB_TAG {
        return None;
    }
    Some((shell, Identity { app_instance_id, base_revision, generation, canonical_base_revision, operation, job }))
}

/// 📐️ The original request transfers its two handles into the boxed job under the framework's own original-job admission quote.
pub(super) fn job_demands(_job: u64, input: &Option<Vec<u8>>, restored: &Option<Vec<u8>>, _cx: &StepContext<'_>) -> Result<RetainedCloneGrant, ValueError> {
    semio_framework_plugin::reactor::jobs::original_job_admission_demands::<MountedJob>(input, restored)
}

pub(super) fn job_admit(job: u64, input: &mut Option<Vec<u8>>, restored: &mut Option<Vec<u8>>, cx: &mut StepContext<'_>) -> Result<Option<Box<dyn BoundedJob>>, ValueError> {
    let bytes = input.as_deref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "fem3d.visual-input"))?;
    let (shell, identity) = decode_input(job, bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "fem3d.visual-input"))?;
    let (owner, recovery) = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let owner = registry.shells.get(shell as usize).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "fem3d.visual-shell"))?.clone();
        let recovery = registry.recoveries.get(shell as usize).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "fem3d.visual-recovery"))?.clone();
        if !owner.try_borrow().is_ok_and(|state| state.as_ref().is_some_and(|state| state.identity == identity)) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "fem3d.visual-stale-factory"));
        }
        if recovery.reserved.get() != Some(identity) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "fem3d.visual-stale-recovery"));
        }
        Ok((owner, recovery))
    })?;
    semio_framework_plugin::reactor::jobs::admit_original_job(input, restored, cx, move |input, restored| MountedJob { shell: owner, recovery, identity, completed: false, input: Some(input), restored })
}
//#endregion 💼️MountedSession
