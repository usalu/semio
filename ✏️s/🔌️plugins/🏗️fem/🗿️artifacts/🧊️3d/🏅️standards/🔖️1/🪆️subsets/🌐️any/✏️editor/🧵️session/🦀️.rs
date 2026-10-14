//! 🧵️ Mounted FEM3D visual publication on the shared bounded-job reactor.

use semio_s_fem_analyses_model::{MountedAnalysisModel, MOUNTED_ANALYSIS_BACKING_BYTES};
use semio_s_fem_mesh::MountedPlanarDomain;
use semio_s_fem_model::{Dof, Node};
use semio_s_fem_static::Elements;
use semio_s_fem_sparse::MountedScalarSlots;
use semio_s_fem_sparse_pcg::PcgJob;
use semio_s_fem_sparse_subspace::SubspaceIterationJob;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework::kernel::{Effect, JobPlacement};
use semio_framework_job::{Generation, OperationId, RevisionId, StepContext};
use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, IoRunControl, JobBudget, JobStep, SqliteSnapshotControl};
use semio_framework_plugin::{AppRenderOperationContext, ArtifactView, PluginLifecycleStep};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
#[cfg(test)]
use semio_framework_ui_scene::world3d_snapshot_with_page;
use semio_framework_ui_scene::{world3d_snapshot_recovery_close_step, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Mutex;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_MOUNTED_VISUAL_JOB_KIND;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::ACTIVE_CAPACITY;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::SHELL_CAPACITY;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_REGIONS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_NODES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_ELEMENTS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_SUPPORTS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_LOADS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_FIELDS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::STEP_CEILING_MS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FAULT_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::JOB_TAG;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::JOB_COUNTER_MAXIMUM;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::INPUT_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_SOLVER_SCALAR_PAGE_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_SOLVER_INITIALIZED_PAGE_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_REGION_ORDER_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_ELEMENT_ORDER_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_PROCESS_BACKING_ITEMS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_PROCESS_BACKING_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dBackingCredit;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dVisualState;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dVisualFreshness;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dVisualJobStage;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_VISUAL_PAGES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_VISUAL_LABEL_EN;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_VISUAL_LABEL_DE;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dSolverScalar;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dSolverView;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dNumericalStage;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FixedSlots;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::ChildEnd;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::session_pending;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::SESSION_CLOSE_COPY_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::SESSION_CLOSE_DEPTH;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::SESSION_CLOSE_RELEASE_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dNumericalChild;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FixedOrder;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::close_recovered_fem3d_backing;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dPageCredit;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dPageVisualLease;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dPageVisualJob;


//#region 💼️MountedSession
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Identity {
    app_instance_id: u32,
    base_revision: RevisionId,
    generation: Generation,
    canonical_base_revision: [u8; 32],
    operation: OperationId,
    job: u64,
}

impl Identity {
    fn freshness(self, preview_sequence: u64) -> Fem3dVisualFreshness {
        Fem3dVisualFreshness {
            app_instance_id: self.app_instance_id,
            model_revision: self.base_revision.0,
            document_generation: self.generation.0,
            operation: self.operation.0,
            numerical_preview_sequence: preview_sequence,
            surface_generation: self.generation.0,
            renderer_scene_generation: self.generation.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MountedRecoveryPublication {
    Retained,
    Recover,
}

struct MountedRecoverySlot {
    reserved: Cell<Option<Identity>>,
    publication: Cell<Option<(Identity, MountedRecoveryPublication)>>,
    owner: Mutex<Option<MountedState>>,
}

impl MountedRecoverySlot {
    fn new() -> Self {
        Self { reserved: Cell::new(None), publication: Cell::new(None), owner: Mutex::new(None) }
    }

    fn reserve(&self, identity: Identity) -> bool {
        let owner = self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if self.reserved.get().is_some() || self.publication.get().is_some() || owner.is_some() {
            return false;
        }
        self.reserved.set(Some(identity));
        true
    }

    fn publish(&self, identity: Identity, publication: MountedRecoveryPublication) -> bool {
        if self.reserved.get() != Some(identity) {
            return false;
        }
        match self.publication.get() {
            None => self.publication.set(Some((identity, publication))),
            Some(current) if current == (identity, publication) => {}
            Some(_) => return false,
        }
        true
    }

    fn publish_owner(&self, identity: Identity, state: MountedState) -> Result<(), MountedState> {
        if !self.publish(identity, MountedRecoveryPublication::Recover) {
            return Err(state);
        }
        let mut owner = self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if owner.is_some() {
            return Err(state);
        }
        *owner = Some(state);
        Ok(())
    }

    fn take_owner(&self, identity: Identity) -> Option<MountedState> {
        (self.reserved.get() == Some(identity)).then(|| self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take()).flatten()
    }

    fn restore_owner(&self, identity: Identity, state: MountedState) -> Result<(), MountedState> {
        if self.reserved.get() != Some(identity) {
            return Err(state);
        }
        let mut owner = self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if owner.is_some() {
            return Err(state);
        }
        *owner = Some(state);
        Ok(())
    }

    fn clear_publication(&self, identity: Identity) -> bool {
        if self.publication.get().is_some_and(|current| current.0 == identity) {
            self.publication.set(None);
            return true;
        }
        false
    }

    fn release(&self, identity: Identity) -> bool {
        let owner = self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if self.reserved.get() != Some(identity) || self.publication.get().is_some() || owner.is_some() {
            return false;
        }
        self.reserved.set(None);
        true
    }

    fn contains(&self, app_instance_id: u32) -> bool {
        self.reserved.get().is_some_and(|identity| identity.app_instance_id == app_instance_id)
    }
}

struct MountedState {
    identity: Identity,
    snapshot: Option<store::os_store::snapshot_read::SnapshotRead<Fem3dSnapshot>>,
    snapshot_return: Option<store::os_store::snapshot_read::SnapshotReadReturn>,
    cancel: semio_framework_job::CancelToken,
    preview_sequence: u64,
    credit: Fem3dPageCredit,
    backing: Fem3dBackingCredit,
    solver: Option<Fem3dSolverView>,
    numerical: Option<Box<Fem3dNumericalChild>>,
    numerical_done: bool,
    candidate: Option<Box<Fem3dPageVisualJob>>,
    current: Option<Fem3dPageVisualLease>,
    displaced: Option<Fem3dPageVisualLease>,
    close_lane: u8,
    done: bool,
    recovery: Rc<MountedRecoverySlot>,
}

impl MountedState {
    fn new(identity: Identity, snapshot: store::os_store::snapshot_read::SnapshotRead<Fem3dSnapshot>, current: Option<Fem3dPageVisualLease>, credit: Fem3dPageCredit, recovery: Rc<MountedRecoverySlot>) -> Self {
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

    fn step(&mut self, budget: JobBudget, cx: &mut StepContext<'_>) -> JobStep {
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

    fn close_step(&mut self, grant: RetainedCloneGrant) -> PluginLifecycleStep {
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
                    let (terminal, items, bytes) = numerical.close_step(grant);
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

    fn terminal_is_empty(&self) -> bool {
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

#[derive(Clone, Copy)]
struct Current {
    app_instance_id: u32,
    shell: u16,
    identity: Identity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SnapshotPreflightStage {
    Nodes,
    Elements,
    Regions,
    Supports,
    LoadCases,
    Loads,
    Complete,
}

#[derive(Clone, Copy)]
struct SnapshotPreflight {
    stage: SnapshotPreflightStage,
    outer: usize,
    inner: usize,
    load_count: usize,
    page_items: [u16; FEM3D_VISUAL_PAGES],
    page_bytes: [u32; FEM3D_VISUAL_PAGES],
    item_count: u32,
    byte_count: u32,
    draw_count: u32,
    draw_bytes: u32,
}

impl SnapshotPreflight {
    fn new() -> Self {
        let mut preflight = Self { stage: SnapshotPreflightStage::Nodes, outer: 0, inner: 0, load_count: 0, page_items: [0; FEM3D_VISUAL_PAGES], page_bytes: [0; FEM3D_VISUAL_PAGES], item_count: 0, byte_count: 0, draw_count: 0, draw_bytes: 0 };
        let _ = preflight.charge(0, 1, 3, 0, 3);
        let _ = preflight.charge(20, 1, 2 + FEM3D_VISUAL_LABEL_EN.len(), 0, 0);
        let _ = preflight.charge(20, 1, 2 + FEM3D_VISUAL_LABEL_DE.len(), 0, 0);
        preflight
    }

    fn charge(&mut self, page: usize, items: u16, bytes: usize, draws: u32, draw_bytes: usize) -> Result<(), ()> {
        let page_items = self.page_items[page].checked_add(items).ok_or(())?;
        let page_bytes = self.page_bytes[page].checked_add(u32::try_from(bytes).map_err(|_| ())?).ok_or(())?;
        let item_count = self.item_count.checked_add(u32::from(items)).ok_or(())?;
        let byte_count = self.byte_count.checked_add(u32::try_from(bytes).map_err(|_| ())?).ok_or(())?;
        let draw_count = self.draw_count.checked_add(draws).ok_or(())?;
        let draw_bytes = self.draw_bytes.checked_add(u32::try_from(draw_bytes).map_err(|_| ())?).ok_or(())?;
        if usize::from(page_items) > WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY
            || page_bytes as usize > WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY
            || item_count as usize > FEM3D_VISUAL_PAGES * WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY
            || byte_count as usize > FEM3D_VISUAL_PAGES * WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY
        {
            return Err(());
        }
        self.page_items[page] = page_items;
        self.page_bytes[page] = page_bytes;
        self.item_count = item_count;
        self.byte_count = byte_count;
        self.draw_count = draw_count;
        self.draw_bytes = draw_bytes;
        Ok(())
    }

    fn credit(self) -> Result<Fem3dPageCredit, ()> {
        (self.stage == SnapshotPreflightStage::Complete).then_some(Fem3dPageCredit { item_count: self.item_count, byte_count: self.byte_count, draw_count: self.draw_count, draw_bytes: self.draw_bytes }).ok_or(())
    }

    fn next_stage(&mut self, stage: SnapshotPreflightStage) {
        self.stage = stage;
        self.outer = 0;
        self.inner = 0;
    }

    fn step_one(&mut self, snapshot: &Fem3dSnapshot) -> Result<bool, ()> {
        match self.stage {
            SnapshotPreflightStage::Nodes => {
                let Some(node) = snapshot.nodes.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Elements);
                    return Ok(false);
                };
                let page = 2 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY;
                self.charge(page, 1, node.id.len(), 1, node.id.len())?;
                self.charge(10 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.charge(12 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.charge(14 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.charge(16 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.charge(18 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.outer += 1;
            }
            SnapshotPreflightStage::Elements => {
                let Some(element) = snapshot.elements.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Regions);
                    return Ok(false);
                };
                let id = element_id(element);
                self.charge(4 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, id.len(), 1, id.len())?;
                self.charge(6 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, id.len(), 1, id.len())?;
                self.outer += 1;
            }
            SnapshotPreflightStage::Regions => {
                let Some(solid) = snapshot.solids.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Supports);
                    return Ok(false);
                };
                let kind_bytes = if solid.layers == 1 { "tetrahedron".len() } else { "hexahedron".len() };
                self.charge(1, 1, solid.id.len().checked_add(kind_bytes).ok_or(())?, 1, solid.id.len())?;
                self.outer += 1;
            }
            SnapshotPreflightStage::Supports => {
                let Some(support) = snapshot.supports.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::LoadCases);
                    return Ok(false);
                };
                self.charge(9, 1, support.id.len(), 1, support.id.len())?;
                self.outer += 1;
            }
            SnapshotPreflightStage::LoadCases => {
                let Some(case) = snapshot.load_cases.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Loads);
                    return Ok(false);
                };
                let next = self.load_count.checked_add(case.loads.len()).ok_or(())?;
                if next > MAXIMUM_LOADS {
                    return Err(());
                }
                self.load_count = next;
                self.outer += 1;
            }
            SnapshotPreflightStage::Loads => {
                let Some(case) = snapshot.load_cases.get(self.outer) else {
                    self.stage = SnapshotPreflightStage::Complete;
                    return Ok(false);
                };
                let Some(load) = case.loads.get(self.inner) else {
                    self.outer += 1;
                    self.inner = 0;
                    return Ok(false);
                };
                let id = load_id(load);
                self.charge(8, 1, id.len(), 1, id.len())?;
                self.inner += 1;
            }
            SnapshotPreflightStage::Complete => return Ok(true),
        }
        Ok(false)
    }
}

#[derive(Clone, Copy)]
struct PendingSnapshot {
    render: AppRenderOperationContext,
    preflight: SnapshotPreflight,
}

struct Registry {
    shells: [Rc<RefCell<Option<MountedState>>>; SHELL_CAPACITY],
    recoveries: [Rc<MountedRecoverySlot>; SHELL_CAPACITY],
    current: [Option<Current>; ACTIVE_CAPACITY],
    pending: [Option<PendingSnapshot>; ACTIVE_CAPACITY],
    retiring: [bool; SHELL_CAPACITY],
    retiring_app: [u32; SHELL_CAPACITY],
    retiring_count: [usize; ACTIVE_CAPACITY],
    retiring_owner: [u32; ACTIVE_CAPACITY],
    free: [u16; SHELL_CAPACITY],
    free_len: usize,
    maintenance_cursor: usize,
    recovery_cursor: usize,
    next_job: u64,
    credit_items: [usize; SHELL_CAPACITY],
    credit_bytes: [usize; SHELL_CAPACITY],
    reserved_items: usize,
    reserved_bytes: usize,
}

impl Registry {
    fn new() -> Self {
        Self {
            shells: std::array::from_fn(|_| Rc::new(RefCell::new(None))),
            recoveries: std::array::from_fn(|_| Rc::new(MountedRecoverySlot::new())),
            current: std::array::from_fn(|_| None),
            pending: std::array::from_fn(|_| None),
            retiring: [false; SHELL_CAPACITY],
            retiring_app: [0; SHELL_CAPACITY],
            retiring_count: [0; ACTIVE_CAPACITY],
            retiring_owner: [0; ACTIVE_CAPACITY],
            free: std::array::from_fn(|index| (SHELL_CAPACITY - 1 - index) as u16),
            free_len: SHELL_CAPACITY,
            maintenance_cursor: 0,
            recovery_cursor: 0,
            next_job: 0,
            credit_items: [0; SHELL_CAPACITY],
            credit_bytes: [0; SHELL_CAPACITY],
            reserved_items: 0,
            reserved_bytes: 0,
        }
    }

    fn allocate(&mut self) -> Option<u16> {
        let next = self.free_len.checked_sub(1)?;
        self.free_len = next;
        Some(self.free[next])
    }

    fn release(&mut self, shell: u16) {
        assert_eq!(self.credit_items[shell as usize], 0, "FEM3D shell released before its process item credit");
        assert_eq!(self.credit_bytes[shell as usize], 0, "FEM3D shell released before its process byte credit");
        assert!(self.recoveries[shell as usize].reserved.get().is_none(), "FEM3D shell released before its recovery authority");
        if self.free_len < SHELL_CAPACITY {
            self.free[self.free_len] = shell;
            self.free_len += 1;
        }
    }

    fn reserve_credit(&mut self, shell: u16) -> bool {
        let slot = shell as usize;
        if self.credit_items[slot] != 0 || self.credit_bytes[slot] != 0 {
            return false;
        }
        let Some(items) = self.reserved_items.checked_add(FEM3D_PROCESS_BACKING_ITEMS).filter(|items| *items <= SHELL_CAPACITY * FEM3D_PROCESS_BACKING_ITEMS) else { return false };
        let Some(bytes) = self.reserved_bytes.checked_add(FEM3D_PROCESS_BACKING_BYTES).filter(|bytes| *bytes <= SHELL_CAPACITY * FEM3D_PROCESS_BACKING_BYTES) else { return false };
        self.credit_items[slot] = FEM3D_PROCESS_BACKING_ITEMS;
        self.credit_bytes[slot] = FEM3D_PROCESS_BACKING_BYTES;
        self.reserved_items = items;
        self.reserved_bytes = bytes;
        true
    }

    fn release_credit(&mut self, shell: u16) {
        let items = std::mem::take(&mut self.credit_items[shell as usize]);
        let bytes = std::mem::take(&mut self.credit_bytes[shell as usize]);
        assert_eq!((items, bytes), (FEM3D_PROCESS_BACKING_ITEMS, FEM3D_PROCESS_BACKING_BYTES), "FEM3D shell returned a mismatched process credit");
        self.reserved_items -= items;
        self.reserved_bytes -= bytes;
    }

    fn retain_retiring(&mut self, app_instance_id: u32, shell: u16) -> bool {
        let slot = app_instance_id as usize % ACTIVE_CAPACITY;
        if self.retiring[shell as usize] || (self.retiring_count[slot] != 0 && self.retiring_owner[slot] != app_instance_id) {
            return false;
        }
        self.retiring[shell as usize] = true;
        self.retiring_app[shell as usize] = app_instance_id;
        self.retiring_owner[slot] = app_instance_id;
        self.retiring_count[slot] += 1;
        true
    }
}

thread_local! {
    static MOUNTED: RefCell<Registry> = RefCell::new(Registry::new());
}

struct MountedJob {
    shell: Rc<RefCell<Option<MountedState>>>,
    recovery: Rc<MountedRecoverySlot>,
    identity: Identity,
    completed: bool,
    input: Option<Vec<u8>>,
    restored: Option<Vec<u8>>,
}

impl BoundedJob for MountedJob {
    /// ⏱️ Drives exact session units back to back until the host's step ceiling elapses, a unit
    /// publishes output, or the session leaves `Running`.
    fn step(&mut self, budget: JobBudget, _original: &mut IoRunControl<'_, '_>, _snapshot: &mut SqliteSnapshotControl<'_>, cx: &mut StepContext<'_>) -> Result<JobStep, ValueError> {
        let Ok(mut shell) = self.shell.try_borrow_mut() else { return Ok(JobStep::Running(None)) };
        let Some(state) = shell.as_mut().filter(|state| state.identity == self.identity) else { return Ok(JobStep::Failed(b"fem3d.visual-stale-shell".to_vec())) };
        let started = semio_framework_job::default_now_us();
        let ceiling_ms = u64::from(budget.deadline_ms).min(STEP_CEILING_MS);
        let deadline = started.map(|started| started.saturating_add(ceiling_ms.saturating_mul(1_000)));
        let mut fuel = budget.fuel;
        let mut step = JobStep::Running(None);
        for _ in 0..UNITS_PER_STEP {
            let Some(deadline) = deadline else {
                step = state.step(budget, cx);
                break;
            };
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
        if matches!(&step, JobStep::Done(_)) {
            self.completed = true;
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

impl Drop for MountedJob {
    fn drop(&mut self) {
        if self.completed {
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

fn encode_input(shell: u16, identity: Identity) -> Vec<u8> {
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
fn job_demands(_job: u64, input: &Option<Vec<u8>>, restored: &Option<Vec<u8>>, _cx: &StepContext<'_>) -> Result<RetainedCloneGrant, ValueError> {
    semio_framework_plugin::reactor::jobs::original_job_admission_demands::<MountedJob>(input, restored)
}

fn job_admit(job: u64, input: &mut Option<Vec<u8>>, restored: &mut Option<Vec<u8>>, cx: &mut StepContext<'_>) -> Result<Option<Box<dyn BoundedJob>>, ValueError> {
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

pub fn initialize() {
    MOUNTED.with(|registry| {
        let _ = registry.borrow().free_len;
    });
    semio_framework_plugin::reactor::jobs::register_bounded_job_kind(FEM3D_MOUNTED_VISUAL_JOB_KIND, BoundedJobFactory { admit: job_admit, demands: job_demands });
}

fn current_identity(app_instance_id: u32) -> Option<Identity> {
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

fn recover_abandoned_one(registry: &mut Registry, app_instance_id: u32, grant: RetainedCloneGrant) -> Option<PluginLifecycleStep> {
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
    let step = state.close_step(grant);
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

fn retire_one(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
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
            state.close_step(grant)
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

pub fn maintenance_step(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
    let (maximum_items, maximum_bytes) = (grant.maximum_items, grant.maximum_release_bytes);
    if maximum_items == 0 {
        return session_pending(0, 0);
    }
    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, grant)) {
        return step;
    }
    if let Some((items, bytes)) = world3d_snapshot_recovery_close_step(maximum_bytes) {
        return session_pending(items, bytes);
    }
    if let Some((items, bytes)) = close_recovered_fem3d_backing(maximum_bytes) {
        return session_pending(items, bytes);
    }
    retire_one(app_instance_id, grant)
}

/// 📏️ Quotes the next close or maintenance turn: one owner release bounded by the largest session owner.
pub fn close_demands(app_instance_id: u32) -> RetirementDemand {
    if terminal_is_empty(app_instance_id) {
        RetirementDemand::default()
    } else {
        RetirementDemand { copy_bytes: SESSION_CLOSE_COPY_BYTES, release_bytes: SESSION_CLOSE_RELEASE_BYTES, depth: SESSION_CLOSE_DEPTH, ..Default::default() }
    }
}

pub fn close_step(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
    let (maximum_items, maximum_bytes) = (grant.maximum_items, grant.maximum_release_bytes);
    if maximum_items == 0 {
        return session_pending(0, 0);
    }
    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, grant)) {
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
        retire_one(app_instance_id, grant)
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

//#region 🧪️Laws
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Laws
