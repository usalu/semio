//! 🧮️ Solver view and numerical vocabulary: scalar fields, the solver view, numerical stages, fixed slots, child ends and the session close budget.

use super::*;

//#region 📦️PageVisualJob
pub const FEM3D_VISUAL_PAGES: usize = 21;
pub const FEM3D_VISUAL_LABEL_EN: &str = "FEM progress; validated final; cancel retry discard";
pub const FEM3D_VISUAL_LABEL_DE: &str = "FEM-Fortschritt; endgültig validiert; abbrechen wiederholen verwerfen";

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fem3dSolverScalar {
    pub displacement: [f64; 3],
    pub residual: [f64; 3],
    pub reaction: [f64; 3],
    pub contour: f64,
    pub mode_shape: [f64; 3],
    pub eigen_estimate: f64,
}

pub struct Fem3dSolverView {
    pub(super) freshness: Fem3dVisualFreshness,
    pub state: Fem3dVisualState,
    pub residual_norm: f64,
    pub tolerance: f64,
    pub completed: usize,
    pub total: usize,
    pub scalars: [Option<Box<[std::mem::MaybeUninit<Fem3dSolverScalar>; FEM3D_SOLVER_FIELDS_PER_PAGE]>>; FEM3D_SOLVER_PAGE_COUNT],
    pub initialized: [Option<Box<[bool; FEM3D_SOLVER_FIELDS_PER_PAGE]>>; FEM3D_SOLVER_PAGE_COUNT],
    initialized_count: usize,
    pub len: usize,
    pub close_lane: usize,
}

impl Fem3dSolverView {
    pub fn new(freshness: Fem3dVisualFreshness, len: usize) -> Self {
        Self {
            freshness,
            state: Fem3dVisualState::Unmeshed,
            residual_norm: f64::INFINITY,
            tolerance: 1e-8,
            completed: 0,
            total: len,
            scalars: std::array::from_fn(|_| None),
            initialized: std::array::from_fn(|_| None),
            initialized_count: 0,
            len,
            close_lane: 0,
        }
    }

    pub fn scalar(&self, index: usize) -> Option<Fem3dSolverScalar> {
        let page = index / FEM3D_SOLVER_FIELDS_PER_PAGE;
        let slot = index % FEM3D_SOLVER_FIELDS_PER_PAGE;
        if index >= self.len || !self.initialized.get(page).and_then(Option::as_deref).is_some_and(|initialized| initialized[slot]) {
            return None;
        }
        self.scalars.get(page).and_then(Option::as_deref).map(|scalars| unsafe { scalars[slot].assume_init() })
    }

    pub fn admit_page(&mut self, page: usize, initialized: bool, backing: &mut Fem3dBackingCredit) -> bool {
        if page >= FEM3D_SOLVER_PAGE_COUNT {
            return false;
        }
        if initialized {
            if self.initialized[page].is_some() || !backing.claim(FEM3D_SOLVER_INITIALIZED_PAGE_BYTES) {
                return false;
            }
            self.initialized[page] = Some(Box::new([false; FEM3D_SOLVER_FIELDS_PER_PAGE]));
        } else {
            if self.scalars[page].is_some() || !backing.claim(FEM3D_SOLVER_SCALAR_PAGE_BYTES) {
                return false;
            }
            self.scalars[page] = Some(Box::new([std::mem::MaybeUninit::uninit(); FEM3D_SOLVER_FIELDS_PER_PAGE]));
        }
        true
    }

    pub fn set_len(&mut self, freshness: Fem3dVisualFreshness, len: usize) -> bool {
        if freshness != self.freshness || self.initialized_count != 0 || len == 0 || len > MAXIMUM_FIELDS {
            return false;
        }
        self.len = len;
        self.total = len;
        true
    }

    pub fn publish_scalar(&mut self, freshness: Fem3dVisualFreshness, index: usize, scalar: Fem3dSolverScalar) -> Result<(), Fem3dSolverScalar> {
        if freshness != self.freshness || index >= self.len {
            return Err(scalar);
        }
        let page = index / FEM3D_SOLVER_FIELDS_PER_PAGE;
        let slot = index % FEM3D_SOLVER_FIELDS_PER_PAGE;
        let (Some(scalars), Some(initialized)) = (self.scalars.get_mut(page).and_then(Option::as_deref_mut), self.initialized.get_mut(page).and_then(Option::as_deref_mut)) else {
            return Err(scalar);
        };
        if !scalar.displacement.iter().chain(&scalar.residual).chain(&scalar.reaction).chain(&scalar.mode_shape).all(|value| value.is_finite()) || !scalar.contour.is_finite() || !scalar.eigen_estimate.is_finite() {
            return Err(scalar);
        }
        scalars[slot].write(scalar);
        if !initialized[slot] {
            self.initialized_count += 1;
        }
        initialized[slot] = true;
        self.completed = self.completed.max(index + 1);
        self.state = Fem3dVisualState::SolvingUnconverged;
        Ok(())
    }

    pub fn publish_progress(&mut self, freshness: Fem3dVisualFreshness, state: Fem3dVisualState, residual_norm: f64, tolerance: f64, completed: usize, total: usize) -> bool {
        if freshness != self.freshness || completed > total || total > MAXIMUM_FIELDS || matches!(state, Fem3dVisualState::SolvingConverged | Fem3dVisualState::ValidatedFinal) && (self.initialized_count != self.len || total != self.len) {
            return false;
        }
        self.state = state;
        self.residual_norm = residual_norm;
        self.tolerance = tolerance;
        self.completed = completed;
        self.total = total;
        true
    }

    pub fn ready(&self) -> bool {
        self.state == Fem3dVisualState::ValidatedFinal && self.initialized_count == self.len && self.total == self.len
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if self.close_lane >= FEM3D_SOLVER_PAGE_COUNT * 2 {
            self.len = 0;
            return (true, 0, 0);
        }
        let scalar_page = self.close_lane < FEM3D_SOLVER_PAGE_COUNT;
        let page = self.close_lane % FEM3D_SOLVER_PAGE_COUNT;
        let bytes = if scalar_page { FEM3D_SOLVER_SCALAR_PAGE_BYTES } else { FEM3D_SOLVER_INITIALIZED_PAGE_BYTES };
        if bytes > maximum_bytes {
            return (false, 0, 0);
        }
        let released = if scalar_page { self.scalars[page].take().is_some() } else { self.initialized[page].take().is_some() };
        self.close_lane += 1;
        (false, usize::from(released), if released { bytes } else { 0 })
    }
}

impl Drop for Fem3dSolverView {
    fn drop(&mut self) {
        for owner in self.scalars.iter_mut().filter_map(Option::take) {
            assert!(recover_fem3d_backing(Fem3dRecoveredBacking::SolverScalar(owner)), "FEM3D solver scalar recovery capacity");
        }
        for owner in self.initialized.iter_mut().filter_map(Option::take) {
            assert!(recover_fem3d_backing(Fem3dRecoveredBacking::SolverInitialized(owner)), "FEM3D solver initialized recovery capacity");
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fem3dNumericalStage {
    ReserveSolverPages,
    ReserveNodes,
    ReserveNodeIds,
    ReserveElements,
    ReserveSupports,
    ReserveMeshedSolids,
    Nodes,
    NodeId,
    ElementMaterial,
    ElementSection,
    ElementStart,
    ElementEnd,
    ElementCommit,
    ElementMass,
    ElementInsert,
    SolidDomainOuterReserve,
    SolidDomainOuterPoint,
    SolidDomainHolesReserve,
    SolidDomainHoleReserve,
    SolidDomainHolePoint,
    SolidMeshBegin,
    SolidMesh,
    SolidMeshReservePoints,
    SolidMeshCopyPoint,
    SolidMeshReserveTriangles,
    SolidMeshCopyTriangle,
    SolidMeshRetire,
    SolidMaterial,
    SolidOwnersReserve,
    SolidNodeLookup,
    SolidNodeCreate,
    SolidNodeId,
    SolidTet,
    SolidTetMass,
    SolidTetCommit,
    SolidIndicesRetire,
    SolidCommit,
    SupportDofReserve,
    SupportDof,
    SupportCommit,
    PublishFieldCount,
    MountAssembly,
    PrepareAssembly,
    Assembly,
    ReserveModalMass,
    InitializeModalMass,
    MapEquation,
    ReserveRhs,
    InitializeRhs,
    ResolveSourceCase,
    ApplyLoad,
    ApplyNodalLoad,
    ResolveMemberLoad,
    ResolveMemberNode,
    ApplyMemberScalar,
    ResolveAreaLoad,
    ApplyAreaNode,
    ResolveSelfWeightMemberNode,
    ApplySelfWeightMember,
    ApplySelfWeightSolid,
    BuildCsr,
    BeginPcg,
    PreparePcg,
    Pcg,
    ReadNodeScalar,
    RecoverReaction,
    PublishNodeScalar,
    RecoverStressElement,
    PublishStressScalar,
    BeginModal,
    PrepareModal,
    BeginLdlt,
    Ldlt,
    BeginSubspace,
    Subspace,
    ReadModeScalar,
    PublishModeScalar,
    PublishProgress,
    Complete,
}

pub struct FixedSlots<T, const N: usize> {
    pub slots: [Option<T>; N],
    pub admitted: usize,
    pub len: usize,
}

impl<T, const N: usize> FixedSlots<T, N> {
    pub fn new() -> Self {
        Self { slots: [const { None }; N], admitted: 0, len: 0 }
    }

    pub fn admit_one(&mut self, target: usize) -> Result<bool, ()> {
        if target > N {
            return Err(());
        }
        if self.admitted < target {
            self.admitted += 1;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == self.admitted {
            return Err(value);
        }
        self.slots[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        (index < self.len).then(|| self.slots[index].as_ref()).flatten()
    }

    pub(super) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.len).then(|| self.slots[index].as_mut()).flatten()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn pop(&mut self) -> Option<T> {
        self.len.checked_sub(1).and_then(|index| {
            self.len = index;
            self.slots[index].take()
        })
    }

    pub fn close_admission_one(&mut self) -> bool {
        if self.admitted != 0 {
            self.admitted -= 1;
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.admitted == 0
    }
}

/// 🧭️ How one child job step ended, decided before the lent outcome borrow is released.
#[derive(Clone, Copy)]
pub enum ChildEnd {
    Yield,
    Lent,
    Complete,
    Fault,
    Cancelled,
}

pub enum Fem3dNumericalFailure {
    Diagnostic(Vec<u8>),
    Original(semio_framework_job::OriginalTurnFailure),
}
impl From<Vec<u8>> for Fem3dNumericalFailure { fn from(value: Vec<u8>) -> Self { Self::Diagnostic(value) } }
impl From<semio_framework_job::OriginalTurnFailure> for Fem3dNumericalFailure { fn from(value: semio_framework_job::OriginalTurnFailure) -> Self { Self::Original(value) } }

pub(super) fn child_end(result: Result<Option<JobOutcomeBorrow<'_>>, semio_framework_job::OriginalTurnFailure>) -> Result<ChildEnd, semio_framework_job::OriginalTurnFailure> {
    Ok(match result? {
        None | Some(JobOutcomeBorrow::Yield { .. }) => ChildEnd::Yield,
        Some(JobOutcomeBorrow::PreviewReady { .. } | JobOutcomeBorrow::CheckpointReady { .. }) => ChildEnd::Lent,
        Some(JobOutcomeBorrow::Complete { .. }) => ChildEnd::Complete,
        Some(JobOutcomeBorrow::Fault { .. }) => ChildEnd::Fault,
        Some(JobOutcomeBorrow::Cancelled { .. }) => ChildEnd::Cancelled,
    })
}

/// 🎟️ One close unit's receipt: `items` released owners and `bytes` of released backing.
pub fn session_pending(items: usize, bytes: usize) -> PluginLifecycleStep {
    PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() })
}

const fn largest(sizes: &[usize]) -> usize {
    let mut index = 0;
    let mut largest = 0;
    while index < sizes.len() {
        if sizes[index] > largest {
            largest = sizes[index];
        }
        index += 1;
    }
    largest
}

/// 📏️ The largest logical element one close unit retires; every quote of the next close turn is bounded by it.
pub const SESSION_CLOSE_COPY_BYTES: usize = 4_096;

/// 🪜️ The deepest retained frontier one close unit reaches: session owner, job, writer and payload.
pub const SESSION_CLOSE_DEPTH: usize = 3;

/// 📏️ The largest single owner release one close unit performs; every quote of the next close turn is bounded by it.
pub const SESSION_CLOSE_RELEASE_BYTES: usize = largest(&[
    WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY,
    MOUNTED_ANALYSIS_BACKING_BYTES,
    INPUT_BYTES,
    size_of::<Fem3dNumericalChild>(),
    size_of::<MeshJob>(),
    size_of::<AssemblyJob<'static>>(),
    size_of::<PcgJob>(),
    size_of::<LdltJob>(),
    size_of::<SubspaceIterationJob>(),
    size_of::<Fem3dPageVisualJob>(),
    size_of::<Fem3dPageVisualLease>(),
    size_of::<store::os_store::snapshot_read::SnapshotRead<Fem3dSnapshot>>(),
    size_of::<store::os_store::snapshot_read::SnapshotReadReturn>(),
]);

//#endregion 📦️PageVisualJob
