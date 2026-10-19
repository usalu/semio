//! 💼️ Mounted state of the FEM 3D session: the per-app-instance state, its transitions and its drop retirement. See [`super`].

use super::*;

//#region 💼️MountedSession
pub(super) struct MountedState {
    pub(super) identity: Identity,
    pub(super) snapshot: Option<store::os_store::snapshot_read::SnapshotRead<Fem3dSnapshot>>,
    pub(super) snapshot_return: Option<store::os_store::snapshot_read::SnapshotReadReturn>,
    pub(super) cancel: semio_framework_job::CancelToken,
    pub(super) preview_sequence: u64,
    pub(super) credit: Fem3dPageCredit,
    pub(super) backing: Fem3dBackingCredit,
    pub(super) solver: Option<Fem3dSolverView>,
    pub(super) numerical: Option<Box<Fem3dNumericalChild>>,
    pub(super) numerical_done: bool,
    pub(super) candidate: Option<Box<Fem3dPageVisualJob>>,
    pub(super) current: Option<Fem3dPageVisualLease>,
    pub(super) displaced: Option<Fem3dPageVisualLease>,
    pub(super) close_lane: u8,
    pub(super) done: bool,
    pub(super) recovery: Rc<MountedRecoverySlot>,
}

impl MountedState {
    pub(super) fn new(identity: Identity, snapshot: store::os_store::snapshot_read::SnapshotRead<Fem3dSnapshot>, current: Option<Fem3dPageVisualLease>, credit: Fem3dPageCredit, recovery: Rc<MountedRecoverySlot>) -> Self {
        let solver = Fem3dSolverView::new(identity.freshness(0), snapshot.nodes.len());
        Self {
            identity,
            snapshot: Some(snapshot),
            snapshot_return: None,
            cancel: semio_framework_job::root_cancel_token(),
            preview_sequence: 0,
            credit,
            backing: Fem3dBackingCredit::new(),
            solver: Some(solver),
            numerical: Some(Box::new(Fem3dNumericalChild::new())),
            numerical_done: false,
            candidate: None,
            current,
            displaced: None,
            close_lane: 0,
            done: false,
            recovery,
        }
    }

    fn fail(&mut self, detail: Vec<u8>) -> JobStep {
        JobStep::Failed(if detail.capacity() <= FAULT_BYTES { detail } else { b"fem3d.visual-fault-capacity".to_vec() })
    }

    pub(super) fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep {
        if self.cancel.is_cancelled_now() {
            return self.fail(b"fem3d.visual-cancelled".to_vec());
        }
        if budget.fuel == 0 || budget.deadline_ms == 0 {
            return JobStep::Running(None);
        }
        if cx.should_yield() {
            return JobStep::Running(None);
        }
        if let Some(displaced) = self.displaced.as_mut() {
            cx.consume_fuel(1);
            let (terminal, _, _) = displaced.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
            if terminal {
                self.displaced = None;
            }
            return JobStep::Running(None);
        }
        if self.done {
            return JobStep::Done(self.identity.generation.0.to_le_bytes().to_vec());
        }
        if !self.numerical_done {
            let freshness = self.identity.freshness(0);
            let operation = semio_framework_job::Operation::new(self.identity.operation, self.identity.base_revision, self.identity.generation, self.identity.job);
            let Some(snapshot) = self.snapshot.as_ref() else { return self.fail(b"fem3d.numerical-snapshot-owner".to_vec()) };
            let Some(solver) = self.solver.as_mut() else { return self.fail(b"fem3d.numerical-solver-owner".to_vec()) };
            let step = self.numerical.as_mut().map(|numerical| numerical.step(snapshot, solver, &mut self.backing, freshness, operation, cx));
            return match step {
                Some(Ok(true)) => {
                    self.numerical_done = true;
                    JobStep::Running(None)
                }
                Some(Ok(false)) => JobStep::Running(None),
                Some(Err(detail)) => self.fail(detail),
                None => self.fail(b"fem3d.numerical-owner".to_vec()),
            };
        }
        if self.candidate.is_none() {
            cx.consume_fuel(1);
            let fields_ready = self.solver.as_ref().is_some_and(Fem3dSolverView::ready);
            if !fields_ready {
                return JobStep::Running(None);
            }
            self.candidate = Some(Box::new(Fem3dPageVisualJob::new(self.identity.freshness(0), self.credit)));
            return JobStep::Running(None);
        }
        let freshness = self.identity.freshness(0);
        let Some(snapshot) = self.snapshot.as_ref() else { return self.fail(b"fem3d.visual-snapshot-owner".to_vec()) };
        let Some(solver) = self.solver.as_ref() else { return self.fail(b"fem3d.visual-solver-owner".to_vec()) };
        cx.consume_fuel(1);
        let step = self.candidate.as_mut().map(|candidate| candidate.step_one(snapshot, solver, &mut self.backing, freshness));
        match step {
            Some(Ok(false)) => JobStep::Running(None),
            Some(Ok(true)) => {
                let Some(lease) = self.candidate.as_deref_mut().and_then(Fem3dPageVisualJob::take_complete) else { return self.fail(b"fem3d.visual-complete-owner".to_vec()) };
                let live = current_identity(self.identity.app_instance_id) == Some(self.identity)
                    && snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision)
                    && !self.cancel.is_cancelled_now()
                    && lease.matches(freshness);
                self.candidate = None;
                if !live {
                    self.displaced = Some(lease);
                    return JobStep::Running(None);
                }
                self.displaced = self.current.replace(lease);
                self.done = true;
                JobStep::Running(None)
            }
            Some(Err(detail)) => self.fail(detail),
            None => self.fail(b"fem3d.visual-candidate-owner".to_vec()),
        }
    }

    pub(super) fn close_step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
        let maximum_bytes = grant.maximum_release_bytes;
        match self.close_lane {
            0 => {
                if let Some(candidate) = self.candidate.as_mut() {
                    let before = candidate.backing_usage();
                    let (terminal, items, bytes) = candidate.close_step(maximum_bytes);
                    let after = candidate.backing_usage();
                    if !self.backing.release(before.0 - after.0, before.1 - after.1) {
                        return PluginLifecycleStep::Blocked { reason: "FEM3D visual candidate process credit mismatch" };
                    }
                    if !terminal {
                        return session_pending(items, bytes);
                    }
                    if !candidate.terminal_is_empty() {
                        return PluginLifecycleStep::Blocked { reason: "FEM3D visual candidate false terminal" };
                    }
                    self.candidate = None;
                    return session_pending(1, 0);
                }
                self.close_lane = 1;
                session_pending(1, 0)
            }
            1 => {
                if let Some(displaced) = self.displaced.as_mut() {
                    let (terminal, items, bytes) = displaced.close_step(maximum_bytes);
                    if !terminal {
                        return session_pending(items, bytes);
                    }
                    if !displaced.terminal_is_empty() {
                        return PluginLifecycleStep::Blocked { reason: "FEM3D displaced lease false terminal" };
                    }
                    self.displaced = None;
                    return session_pending(1, 0);
                }
                self.close_lane = 2;
                session_pending(1, 0)
            }
            2 => {
                if let Some(current) = self.current.as_mut() {
                    let (terminal, items, bytes) = current.close_step(maximum_bytes);
                    if !terminal {
                        return session_pending(items, bytes);
                    }
                    if !current.terminal_is_empty() {
                        return PluginLifecycleStep::Blocked { reason: "FEM3D current lease false terminal" };
                    }
                    self.current = None;
                    return session_pending(1, 0);
                }
                self.close_lane = 3;
                session_pending(1, 0)
            }
            3 => {
                if let Some(numerical) = self.numerical.as_mut() {
                    let (terminal, items, bytes) = numerical.close_step(cx, grant);
                    if !terminal {
                        return session_pending(items, bytes);
                    }
                    self.numerical = None;
                    return session_pending(1, 0);
                }
                self.close_lane = 4;
                session_pending(1, 0)
            }
            4 => {
                if let Some(solver) = self.solver.as_mut() {
                    let (terminal, items, bytes) = solver.close_step(maximum_bytes);
                    if items != 0 && !self.backing.release(items, bytes) {
                        return PluginLifecycleStep::Blocked { reason: "FEM3D solver process credit mismatch" };
                    }
                    if !terminal {
                        return session_pending(items, bytes);
                    }
                    self.solver = None;
                    return session_pending(items, bytes);
                }
                self.close_lane = 5;
                session_pending(1, 0)
            }
            5 => {
                if let Some(snapshot) = self.snapshot.take() {
                    let Some(witness) = snapshot.return_to_registry_witness() else {
                        return PluginLifecycleStep::Blocked { reason: "FEM3D snapshot already returned" };
                    };
                    self.snapshot_return = Some(witness);
                    return session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotRead<Fem3dSnapshot>>());
                }
                if self.snapshot_return.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
                    return session_pending(0, 0);
                }
                if self.snapshot_return.take().is_some() {
                    return session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotReadReturn>());
                }
                self.close_lane = 6;
                session_pending(1, 0)
            }
            6 => {
                if !self.backing.terminal_is_empty() {
                    return PluginLifecycleStep::Blocked { reason: "FEM3D process backing credit remains live" };
                }
                self.close_lane = 7;
                session_pending(1, 0)
            }
            _ => PluginLifecycleStep::Complete(RetainedCloneProgress::default()),
        }
    }

    fn terminal_placeholder(&self) -> Self {
        Self {
            identity: self.identity,
            snapshot: None,
            snapshot_return: None,
            cancel: self.cancel.clone(),
            preview_sequence: self.preview_sequence,
            credit: self.credit,
            backing: Fem3dBackingCredit::new(),
            solver: None,
            numerical: None,
            numerical_done: true,
            candidate: None,
            current: None,
            displaced: None,
            close_lane: 7,
            done: true,
            recovery: self.recovery.clone(),
        }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.close_lane >= 7
            && self.snapshot.is_none()
            && self.snapshot_return.is_none()
            && self.solver.is_none()
            && self.numerical.is_none()
            && self.candidate.is_none()
            && self.current.is_none()
            && self.displaced.is_none()
            && self.backing.terminal_is_empty()
    }
}

impl Drop for MountedState {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        self.cancel.cancel_now();
        let identity = self.identity;
        let recovery = self.recovery.clone();
        let placeholder = self.terminal_placeholder();
        let owner = std::mem::replace(self, placeholder);
        if let Err(owner) = recovery.publish_owner(identity, owner) {
            std::mem::forget(owner);
            panic!("FEM3D mounted state recovery reservation mismatch");
        }
    }
}
//#endregion 💼️MountedSession
