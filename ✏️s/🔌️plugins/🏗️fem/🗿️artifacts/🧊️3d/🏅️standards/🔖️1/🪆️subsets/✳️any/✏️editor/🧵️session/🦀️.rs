//! 🧵️ Mounted FEM3D visual publication on the shared bounded-job reactor.

use crate::analyses::{AssemblyCsrBuild, AssemblyJob, AssemblyJobConstruction, MountedAnalysisModel, MountedAnalysisSupport};
use crate::artifacts::fem3d::{element_id, load_id, Fem3dSnapshot, FemElement, FemLoad};
use crate::elements3d::Tet4;
use crate::mesh::{MeshJob, MeshOpts, MountedPlanarDomain};
use crate::model::{Bar3, Dof, Element, Elements, Frame3, Node};
use crate::sparse::{Csr, LdltJob, ModalInputConstruction, MountedScalarSlots, PcgJob, PcgJobConstruction, SubspaceIterationJob};
use semio_framework::kernel::{Effect, JobPlacement};
use semio_framework_job::{Generation, InteractiveJob, OperationId, RetainedJobPayload, RevisionId, StepBudget, StepContext, StepOutcome};
use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, JobBudget, JobStep};
use semio_framework_plugin::{AppRenderOperationContext, ArtifactView, PluginCloseStep};
use semio_framework_ui_scene::{
    world3d_snapshot_abort_write, world3d_snapshot_abort_write_step, world3d_snapshot_admit_page, world3d_snapshot_begin, world3d_snapshot_begin_close, world3d_snapshot_close_step, world3d_snapshot_recover_lease, world3d_snapshot_recover_page,
    world3d_snapshot_recover_write, world3d_snapshot_recovery_close_step, world3d_snapshot_seal, world3d_snapshot_terminal_is_empty, world3d_snapshot_with_page, world3d_snapshot_write_terminal_is_empty, World3dSnapshotDescriptor,
    World3dSnapshotFault, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPage, World3dSnapshotPageKind, World3dSnapshotWriteToken, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

//#region 🔖️Contract
pub const FEM3D_MOUNTED_VISUAL_JOB_KIND: &str = "semio.fem3d.mounted-live-visual";
const ACTIVE_CAPACITY: usize = 16;
const SHELL_CAPACITY: usize = 32;
const MAXIMUM_REGIONS: usize = 32;
const MAXIMUM_NODES: usize = 128;
const MAXIMUM_ELEMENTS: usize = 128;
const MAXIMUM_SUPPORTS: usize = 64;
const MAXIMUM_LOADS: usize = 64;
const MAXIMUM_FIELDS: usize = 128;
const FAULT_BYTES: usize = WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY;
const JOB_TAG: u64 = 0xf3d0_0000_0000_0000;
const JOB_COUNTER_MAXIMUM: u64 = 0x000f_ffff_ffff_ffff;
const INPUT_BYTES: usize = 63;
const FEM3D_SOLVER_FIELDS_PER_PAGE: usize = 16;
const FEM3D_SOLVER_PAGE_COUNT: usize = MAXIMUM_FIELDS / FEM3D_SOLVER_FIELDS_PER_PAGE;
const FEM3D_SOLVER_SCALAR_PAGE_BYTES: usize = std::mem::size_of::<Fem3dSolverScalar>() * FEM3D_SOLVER_FIELDS_PER_PAGE;
const FEM3D_SOLVER_INITIALIZED_PAGE_BYTES: usize = std::mem::size_of::<bool>() * FEM3D_SOLVER_FIELDS_PER_PAGE;
const FEM3D_REGION_ORDER_BYTES: usize = std::mem::size_of::<Option<usize>>() * MAXIMUM_REGIONS;
const FEM3D_ELEMENT_ORDER_BYTES: usize = std::mem::size_of::<Option<usize>>() * MAXIMUM_ELEMENTS;
const FEM3D_PROCESS_BACKING_ITEMS: usize = FEM3D_SOLVER_PAGE_COUNT * 2 + 2;
const FEM3D_PROCESS_BACKING_BYTES: usize = FEM3D_SOLVER_PAGE_COUNT * (FEM3D_SOLVER_SCALAR_PAGE_BYTES + FEM3D_SOLVER_INITIALIZED_PAGE_BYTES) + FEM3D_REGION_ORDER_BYTES + FEM3D_ELEMENT_ORDER_BYTES;

#[derive(Clone, Copy)]
struct Fem3dBackingCredit {
    admitted_items: usize,
    admitted_bytes: usize,
    live_items: usize,
    live_bytes: usize,
}

impl Fem3dBackingCredit {
    fn new() -> Self {
        Self { admitted_items: FEM3D_PROCESS_BACKING_ITEMS, admitted_bytes: FEM3D_PROCESS_BACKING_BYTES, live_items: 0, live_bytes: 0 }
    }

    fn claim(&mut self, bytes: usize) -> bool {
        let Some(items) = self.live_items.checked_add(1) else { return false };
        let Some(live_bytes) = self.live_bytes.checked_add(bytes) else { return false };
        if items > self.admitted_items || live_bytes > self.admitted_bytes {
            return false;
        }
        self.live_items = items;
        self.live_bytes = live_bytes;
        true
    }

    fn release(&mut self, items: usize, bytes: usize) -> bool {
        let Some(live_items) = self.live_items.checked_sub(items) else { return false };
        let Some(live_bytes) = self.live_bytes.checked_sub(bytes) else { return false };
        self.live_items = live_items;
        self.live_bytes = live_bytes;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.live_items == 0 && self.live_bytes == 0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Fem3dVisualState {
    #[default]
    Unmeshed,
    CoarseMesh,
    RefinedMesh,
    Assembling,
    SolvingUnconverged,
    SolvingConverged,
    ValidatedFinal,
    FaultedCancelled,
}

impl Fem3dVisualState {
    fn id(self) -> &'static str {
        match self {
            Self::Unmeshed => "unmeshed",
            Self::CoarseMesh => "coarse-mesh",
            Self::RefinedMesh => "refined-mesh",
            Self::Assembling => "assembling",
            Self::SolvingUnconverged => "solving-unconverged",
            Self::SolvingConverged => "solving-converged",
            Self::ValidatedFinal => "validated-final",
            Self::FaultedCancelled => "faulted-cancelled-last-valid",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fem3dVisualField {
    pub node_id: String,
    pub displacement: [f64; 3],
    pub residual: [f64; 3],
    pub reaction: [f64; 3],
    pub contour: f64,
    pub mode_shape: [f64; 3],
    pub eigen_estimate: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fem3dVisualFreshness {
    pub app_instance_id: u32,
    pub model_revision: u64,
    pub document_generation: u64,
    pub operation: u64,
    pub numerical_preview_sequence: u64,
    pub surface_generation: u64,
    pub renderer_scene_generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fem3dVisualJobStage {
    ReserveSnapshot,
    ReadProgressScalar,
    OrderRegionKey,
    BuildRegion,
    OrderElementKey,
    BuildMeshElement,
    BuildAssemblyMark,
    BuildLoadGlyph,
    BuildSupportGlyph,
    BuildDisplacementEntry,
    BuildResidualEntry,
    BuildReactionEntry,
    BuildContourEntry,
    BuildModeEntry,
    BuildLabelEntry,
    SealPages,
    ValidateFreshness,
    PublishLease,
    RetireDisplacedLease,
    Complete,
}

//#region 📦️PageVisualJob
const FEM3D_VISUAL_PAGES: usize = 21;
const FEM3D_VISUAL_LABEL_EN: &str = "FEM progress; validated final; cancel retry discard";
const FEM3D_VISUAL_LABEL_DE: &str = "FEM-Fortschritt; endgültig validiert; abbrechen wiederholen verwerfen";

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
    freshness: Fem3dVisualFreshness,
    state: Fem3dVisualState,
    residual_norm: f64,
    tolerance: f64,
    completed: usize,
    total: usize,
    scalars: [Option<Box<[std::mem::MaybeUninit<Fem3dSolverScalar>; FEM3D_SOLVER_FIELDS_PER_PAGE]>>; FEM3D_SOLVER_PAGE_COUNT],
    initialized: [Option<Box<[bool; FEM3D_SOLVER_FIELDS_PER_PAGE]>>; FEM3D_SOLVER_PAGE_COUNT],
    initialized_count: usize,
    len: usize,
    close_lane: usize,
}

impl Fem3dSolverView {
    fn new(freshness: Fem3dVisualFreshness, len: usize) -> Self {
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

    fn scalar(&self, index: usize) -> Option<Fem3dSolverScalar> {
        let page = index / FEM3D_SOLVER_FIELDS_PER_PAGE;
        let slot = index % FEM3D_SOLVER_FIELDS_PER_PAGE;
        if index >= self.len || !self.initialized.get(page).and_then(Option::as_deref).is_some_and(|initialized| initialized[slot]) {
            return None;
        }
        self.scalars.get(page).and_then(Option::as_deref).map(|scalars| unsafe { scalars[slot].assume_init() })
    }

    fn admit_page(&mut self, page: usize, initialized: bool, backing: &mut Fem3dBackingCredit) -> bool {
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

    fn set_len(&mut self, freshness: Fem3dVisualFreshness, len: usize) -> bool {
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

    fn ready(&self) -> bool {
        self.state == Fem3dVisualState::ValidatedFinal && self.initialized_count == self.len && self.total == self.len
    }

    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
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
enum Fem3dNumericalStage {
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

struct FixedSlots<T, const N: usize> {
    slots: [Option<T>; N],
    admitted: usize,
    len: usize,
}

impl<T, const N: usize> FixedSlots<T, N> {
    fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), admitted: 0, len: 0 }
    }

    fn admit_one(&mut self, target: usize) -> Result<bool, ()> {
        if target > N {
            return Err(());
        }
        if self.admitted < target {
            self.admitted += 1;
            return Ok(false);
        }
        Ok(true)
    }

    fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == self.admitted {
            return Err(value);
        }
        self.slots[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    fn get(&self, index: usize) -> Option<&T> {
        (index < self.len).then(|| self.slots[index].as_ref()).flatten()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.len).then(|| self.slots[index].as_mut()).flatten()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn pop(&mut self) -> Option<T> {
        self.len.checked_sub(1).and_then(|index| {
            self.len = index;
            self.slots[index].take()
        })
    }

    fn close_admission_one(&mut self) -> bool {
        if self.admitted != 0 {
            self.admitted -= 1;
            return false;
        }
        true
    }
}

struct Fem3dMeshedSolid {
    solid_index: usize,
    node_ids: FixedSlots<String, MAXIMUM_FIELDS>,
    top_offset: usize,
    points: FixedSlots<[f64; 2], MAXIMUM_FIELDS>,
    tris: FixedSlots<[u32; 3], MAXIMUM_ELEMENTS>,
}

struct Fem3dNumericalChild {
    stage: Fem3dNumericalStage,
    model: Option<MountedAnalysisModel>,
    analysis_node_ids: FixedSlots<String, MAXIMUM_FIELDS>,
    meshed_solids: FixedSlots<Fem3dMeshedSolid, MAXIMUM_REGIONS>,
    node_cursor: usize,
    element_cursor: usize,
    support_cursor: usize,
    dof_cursor: usize,
    lookup_cursor: usize,
    material_cursor: usize,
    section_cursor: usize,
    resolved_material: usize,
    resolved_section: usize,
    element_materials: [usize; MAXIMUM_ELEMENTS],
    element_sections: [usize; MAXIMUM_ELEMENTS],
    pending_element_nodes: [usize; 2],
    pending_support: Option<MountedAnalysisSupport>,
    pending_element: Option<Elements>,
    mass_update_cursor: usize,
    pending_translational_mass: f64,
    pending_rotational_mass: f64,
    assembly_build: Option<AssemblyJobConstruction>,
    assembly: Option<AssemblyJob<'static>>,
    csr_build: Option<AssemblyCsrBuild>,
    pcg_build: Option<PcgJobConstruction>,
    rejected_pcg_matrix: Option<Csr>,
    rejected_pcg_rhs: Option<MountedScalarSlots>,
    pcg: Option<PcgJob>,
    modal_build: Option<ModalInputConstruction>,
    ldlt: Option<LdltJob>,
    subspace: Option<SubspaceIterationJob>,
    modal_mass: Option<crate::sparse::Csr>,
    scalar_axis: usize,
    scalar: Fem3dSolverScalar,
    equations: [[Option<usize>; 6]; MAXIMUM_FIELDS],
    full_equations: [[Option<usize>; 6]; MAXIMUM_FIELDS],
    full_rhs: [f64; MAXIMUM_FIELDS * 6],
    free_order: usize,
    rhs: MountedScalarSlots,
    load_cursor: usize,
    load_node_cursor: usize,
    load_solid_cursor: usize,
    load_triangle_cursor: usize,
    load_vertex_cursor: usize,
    load_element_cursor: usize,
    load_node_indices: [usize; 2],
    load_positions: [[f64; 3]; 2],
    reaction_entry: usize,
    reaction_accumulator: f64,
    solid_cursor: usize,
    hole_cursor: usize,
    point_cursor: usize,
    solid_domain: Option<MountedPlanarDomain>,
    mesh: Option<MeshJob>,
    solid_points: FixedSlots<[f64; 2], MAXIMUM_FIELDS>,
    solid_tris: FixedSlots<[u32; 3], MAXIMUM_ELEMENTS>,
    solid_node_ids: FixedSlots<String, MAXIMUM_FIELDS>,
    solid_node_analysis_indices: FixedSlots<usize, MAXIMUM_FIELDS>,
    volume_node_cursor: usize,
    pending_node_id: Option<String>,
    pending_node_index: usize,
    pending_node_needs_analysis: bool,
    tet_cursor: usize,
    tet_phase: usize,
    pending_tet: Option<Elements>,
    pending_tet_indices: [usize; 4],
    pending_tet_mass: f64,
    solid_material_cursor: usize,
    resolved_solid_material: usize,
    solid_materials: [usize; MAXIMUM_REGIONS],
    close_lane: u8,
    close_started_jobs: u8,
    operation: Option<semio_framework_job::Operation>,
    solver_page_cursor: usize,
    solver_page_lane: bool,
    modal_lumped_mass: [f64; MAXIMUM_FIELDS * 6],
    modal_free_mass: MountedScalarSlots,
    fault_payload: Option<RetainedJobPayload>,
}

impl Fem3dNumericalChild {
    fn new() -> Self {
        Self {
            stage: Fem3dNumericalStage::ReserveSolverPages,
            model: Some(MountedAnalysisModel::new()),
            analysis_node_ids: FixedSlots::new(),
            meshed_solids: FixedSlots::new(),
            node_cursor: 0,
            element_cursor: 0,
            support_cursor: 0,
            dof_cursor: 0,
            lookup_cursor: 0,
            material_cursor: 0,
            section_cursor: 0,
            resolved_material: 0,
            resolved_section: 0,
            element_materials: [0; MAXIMUM_ELEMENTS],
            element_sections: [0; MAXIMUM_ELEMENTS],
            pending_element_nodes: [0; 2],
            pending_support: None,
            pending_element: None,
            mass_update_cursor: 0,
            pending_translational_mass: 0.0,
            pending_rotational_mass: 0.0,
            assembly_build: None,
            assembly: None,
            csr_build: None,
            pcg_build: None,
            rejected_pcg_matrix: None,
            rejected_pcg_rhs: None,
            pcg: None,
            modal_build: None,
            ldlt: None,
            subspace: None,
            modal_mass: None,
            scalar_axis: 0,
            scalar: Fem3dSolverScalar::default(),
            equations: [[None; 6]; MAXIMUM_FIELDS],
            full_equations: [[None; 6]; MAXIMUM_FIELDS],
            full_rhs: [0.0; MAXIMUM_FIELDS * 6],
            free_order: 0,
            rhs: MountedScalarSlots::new(),
            load_cursor: 0,
            load_node_cursor: 0,
            load_solid_cursor: 0,
            load_triangle_cursor: 0,
            load_vertex_cursor: 0,
            load_element_cursor: 0,
            load_node_indices: [0; 2],
            load_positions: [[0.0; 3]; 2],
            reaction_entry: 0,
            reaction_accumulator: 0.0,
            solid_cursor: 0,
            hole_cursor: 0,
            point_cursor: 0,
            solid_domain: None,
            mesh: None,
            solid_points: FixedSlots::new(),
            solid_tris: FixedSlots::new(),
            solid_node_ids: FixedSlots::new(),
            solid_node_analysis_indices: FixedSlots::new(),
            volume_node_cursor: 0,
            pending_node_id: None,
            pending_node_index: 0,
            pending_node_needs_analysis: false,
            tet_cursor: 0,
            tet_phase: 0,
            pending_tet: None,
            pending_tet_indices: [0; 4],
            pending_tet_mass: 0.0,
            solid_material_cursor: 0,
            resolved_solid_material: 0,
            solid_materials: [0; MAXIMUM_REGIONS],
            close_lane: 0,
            close_started_jobs: 0,
            operation: None,
            solver_page_cursor: 0,
            solver_page_lane: false,
            modal_lumped_mass: [0.0; MAXIMUM_FIELDS * 6],
            modal_free_mass: MountedScalarSlots::new(),
            fault_payload: None,
        }
    }

    fn solid_point(&self, doc: &Fem3dSnapshot, index: usize) -> Result<[f64; 3], Vec<u8>> {
        let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
        let footprint = self.solid_points.len();
        if footprint == 0 {
            return Err(b"fem3d.numerical-solid-empty-mesh".to_vec());
        }
        let layers = solid.layers.max(1);
        let layer = index / footprint;
        let point = *self.solid_points.get(index % footprint).ok_or_else(|| b"fem3d.numerical-solid-point".to_vec())?;
        Ok([point[0], point[1], solid.base_z + solid.height * layer as f64 / layers as f64])
    }

    fn solid_tet_indices(&self) -> Result<[usize; 4], Vec<u8>> {
        let source = self.solid_tris.get(self.tet_cursor).ok_or_else(|| b"fem3d.numerical-solid-triangle".to_vec())?;
        Ok(Self::tet_indices(self.solid_points.len(), *source, self.point_cursor, self.tet_phase))
    }

    fn tet_indices(footprint: usize, source: [u32; 3], layer: usize, phase: usize) -> [usize; 4] {
        let mut bottom = [source[0] as usize, source[1] as usize, source[2] as usize];
        if bottom[1] < bottom[0] && bottom[1] <= bottom[2] {
            bottom = [bottom[1], bottom[2], bottom[0]];
        } else if bottom[2] < bottom[0] && bottom[2] < bottom[1] {
            bottom = [bottom[2], bottom[0], bottom[1]];
        }
        let [n0, n1, n2] = bottom.map(|index| layer * footprint + index);
        let [n3, n4, n5] = bottom.map(|index| (layer + 1) * footprint + index);
        match (phase, n1 <= n2) {
            (0, true) => [n0, n1, n2, n5],
            (1, true) => [n0, n1, n5, n4],
            (0, false) => [n0, n1, n2, n4],
            (1, false) => [n0, n2, n5, n4],
            _ => [n0, n3, n4, n5],
        }
    }

    fn apply_rhs_value(&mut self, node: usize, axis: usize, value: f64) -> Result<(), Vec<u8>> {
        if !value.is_finite() {
            return Err(b"fem3d.numerical-load-value".to_vec());
        }
        if let Some(equation) = self.full_equations[node][axis] {
            self.full_rhs[equation] += value;
        }
        if let Some(equation) = self.equations[node][axis] {
            self.rhs.add_at(equation, value).map_err(|_| b"fem3d.numerical-rhs-index".to_vec())?;
        }
        Ok(())
    }

    fn retain_fault(&mut self, payload: RetainedJobPayload) -> Vec<u8> {
        self.fault_payload = Some(payload);
        b"fem3d.numerical-child-fault".to_vec()
    }

    fn step_model(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Vec<u8>> {
        match self.stage {
            Fem3dNumericalStage::ReserveNodes => {
                if self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.admit_node_one(doc.nodes.len()).map_err(|_| b"fem3d.numerical-node-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::ReserveNodeIds;
                }
            }
            Fem3dNumericalStage::ReserveNodeIds => {
                if self.analysis_node_ids.admit_one(doc.nodes.len()).map_err(|_| b"fem3d.numerical-node-id-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::ReserveElements;
                }
            }
            Fem3dNumericalStage::ReserveElements => {
                if self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.admit_element_one(doc.elements.len()).map_err(|_| b"fem3d.numerical-element-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::ReserveSupports;
                }
            }
            Fem3dNumericalStage::ReserveSupports => {
                if self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.admit_support_one(doc.supports.len()).map_err(|_| b"fem3d.numerical-support-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::ReserveMeshedSolids;
                }
            }
            Fem3dNumericalStage::ReserveMeshedSolids => {
                if self.meshed_solids.admit_one(doc.solids.len()).map_err(|_| b"fem3d.numerical-solid-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::Nodes;
                }
            }
            Fem3dNumericalStage::Nodes => {
                if let Some(node) = doc.nodes.get(self.node_cursor) {
                    self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.push_node(Node { id: node.id.clone(), pos: [node.x, node.y, node.z] }).map_err(|_| b"fem3d.numerical-node-slot".to_vec())?;
                    self.stage = Fem3dNumericalStage::NodeId;
                } else {
                    self.stage = Fem3dNumericalStage::ElementMaterial;
                }
            }
            Fem3dNumericalStage::NodeId => {
                let node = doc.nodes.get(self.node_cursor).ok_or_else(|| b"fem3d.numerical-node".to_vec())?;
                self.analysis_node_ids.push(node.id.clone()).map_err(|_| b"fem3d.numerical-node-id-slot".to_vec())?;
                self.node_cursor += 1;
                self.stage = Fem3dNumericalStage::Nodes;
            }
            Fem3dNumericalStage::ElementMaterial => {
                let Some(element) = doc.elements.get(self.element_cursor) else {
                    self.stage = Fem3dNumericalStage::SolidDomainOuterReserve;
                    return Ok(false);
                };
                let material_id = match element {
                    FemElement::Bar { material_id, .. } | FemElement::Frame { material_id, .. } => material_id,
                };
                let Some(material) = doc.materials.get(self.material_cursor) else { return Err(b"fem3d.numerical-material".to_vec()) };
                if &material.id == material_id {
                    self.resolved_material = self.material_cursor;
                    self.material_cursor = 0;
                    self.stage = Fem3dNumericalStage::ElementSection;
                } else {
                    self.material_cursor += 1;
                }
            }
            Fem3dNumericalStage::ElementSection => {
                let Some(element) = doc.elements.get(self.element_cursor) else { return Err(b"fem3d.numerical-element".to_vec()) };
                let section_id = match element {
                    FemElement::Bar { section_id, .. } | FemElement::Frame { section_id, .. } => section_id,
                };
                let Some(section) = doc.sections.get(self.section_cursor) else { return Err(b"fem3d.numerical-section".to_vec()) };
                if &section.id == section_id {
                    self.resolved_section = self.section_cursor;
                    self.section_cursor = 0;
                    self.stage = Fem3dNumericalStage::ElementStart;
                } else {
                    self.section_cursor += 1;
                }
            }
            Fem3dNumericalStage::ElementStart | Fem3dNumericalStage::ElementEnd => {
                let Some(element) = doc.elements.get(self.element_cursor) else { return Err(b"fem3d.numerical-element".to_vec()) };
                let (start, end) = match element {
                    FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start, end),
                };
                let target = if self.stage == Fem3dNumericalStage::ElementStart { start } else { end };
                let Some(node) = doc.nodes.get(self.lookup_cursor) else { return Err(b"fem3d.numerical-element-node".to_vec()) };
                if &node.id == target {
                    self.pending_element_nodes[usize::from(self.stage == Fem3dNumericalStage::ElementEnd)] = self.lookup_cursor;
                    self.lookup_cursor = 0;
                    self.stage = if self.stage == Fem3dNumericalStage::ElementStart { Fem3dNumericalStage::ElementEnd } else { Fem3dNumericalStage::ElementCommit };
                } else {
                    self.lookup_cursor += 1;
                }
            }
            Fem3dNumericalStage::ElementCommit => {
                let element = doc.elements.get(self.element_cursor).ok_or_else(|| b"fem3d.numerical-element".to_vec())?;
                let material = doc.materials.get(self.resolved_material).ok_or_else(|| b"fem3d.numerical-material".to_vec())?;
                let section = doc.sections.get(self.resolved_section).ok_or_else(|| b"fem3d.numerical-section".to_vec())?;
                let built: Elements = match element {
                    FemElement::Bar { id, start, end, .. } => Bar3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, a: section.area, density: material.rho }.into(),
                    FemElement::Frame { id, start, end, roll, .. } => {
                        Frame3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, g: material.g, a: section.area, iy: section.iy, iz: section.iz, j: section.j, roll: *roll, density: material.rho }.into()
                    }
                };
                self.element_materials[self.element_cursor] = self.resolved_material;
                self.element_sections[self.element_cursor] = self.resolved_section;
                let start = doc.nodes.get(self.pending_element_nodes[0]).ok_or_else(|| b"fem3d.numerical-element-node".to_vec())?;
                let end = doc.nodes.get(self.pending_element_nodes[1]).ok_or_else(|| b"fem3d.numerical-element-node".to_vec())?;
                let dx = end.x - start.x;
                let dy = end.y - start.y;
                let dz = end.z - start.z;
                let length = (dx * dx + dy * dy + dz * dz).sqrt();
                self.pending_translational_mass = material.rho * section.area * length * 0.5;
                self.pending_rotational_mass = if matches!(element, FemElement::Frame { .. }) { material.rho * (section.area * length.powi(3) / 105.0 + section.j * length / 3.0) } else { 0.0 };
                self.pending_element = Some(built);
                self.mass_update_cursor = 0;
                self.stage = Fem3dNumericalStage::ElementMass;
            }
            Fem3dNumericalStage::ElementMass => {
                let limit = if self.pending_rotational_mass > 0.0 { 12 } else { 6 };
                if self.mass_update_cursor < limit {
                    let node = self.pending_element_nodes[(self.mass_update_cursor / 3) % 2];
                    let axis = self.mass_update_cursor % 3 + usize::from(self.mass_update_cursor >= 6) * 3;
                    let value = if self.mass_update_cursor < 6 { self.pending_translational_mass } else { self.pending_rotational_mass };
                    self.modal_lumped_mass[node * 6 + axis] += value;
                    self.mass_update_cursor += 1;
                } else {
                    self.stage = Fem3dNumericalStage::ElementInsert;
                }
            }
            Fem3dNumericalStage::ElementInsert => {
                let built = self.pending_element.take().ok_or_else(|| b"fem3d.numerical-element-owner".to_vec())?;
                if let Err(built) = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.push_element(built) {
                    self.pending_element = Some(built);
                    return Err(b"fem3d.numerical-element-slot".to_vec());
                }
                self.element_cursor += 1;
                self.stage = Fem3dNumericalStage::ElementMaterial;
            }
            Fem3dNumericalStage::SolidDomainOuterReserve => {
                let Some(solid) = doc.solids.get(self.solid_cursor) else {
                    self.stage = Fem3dNumericalStage::SupportDofReserve;
                    return Ok(false);
                };
                if solid.layers == 0 || solid.height <= 0.0 || solid.mesh_size <= 0.0 || solid.outline.len() < 3 {
                    return Err(b"fem3d.numerical-solid-domain".to_vec());
                }
                if self.solid_domain.is_none() {
                    self.solid_domain = Some(MountedPlanarDomain::new());
                }
                if self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?.admit_outer_one(solid.outline.len()).map_err(|_| b"fem3d.numerical-solid-outline-admission".to_vec())? {
                    self.point_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidDomainOuterPoint;
                }
            }
            Fem3dNumericalStage::SolidDomainOuterPoint => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                if let Some(point) = solid.outline.get(self.point_cursor) {
                    self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?.push_outer(*point).map_err(|_| b"fem3d.numerical-solid-outline-slot".to_vec())?;
                    self.point_cursor += 1;
                } else {
                    self.stage = Fem3dNumericalStage::SolidDomainHolesReserve;
                }
            }
            Fem3dNumericalStage::SolidDomainHolesReserve => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                if self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?.admit_hole_one(solid.holes.len()).map_err(|_| b"fem3d.numerical-solid-holes-admission".to_vec())? {
                    self.hole_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidDomainHoleReserve;
                }
            }
            Fem3dNumericalStage::SolidDomainHoleReserve => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let Some(hole) = solid.holes.get(self.hole_cursor) else {
                    self.stage = Fem3dNumericalStage::SolidMeshBegin;
                    return Ok(false);
                };
                let domain = self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?;
                if self.point_cursor == 0 {
                    domain.begin_hole().map_err(|_| b"fem3d.numerical-solid-hole-slot".to_vec())?;
                    self.point_cursor = 1;
                    return Ok(false);
                }
                if domain.admit_hole_point_one(self.hole_cursor, hole.len()).map_err(|_| b"fem3d.numerical-solid-hole-admission".to_vec())? {
                    self.point_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidDomainHolePoint;
                }
            }
            Fem3dNumericalStage::SolidDomainHolePoint => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let hole = solid.holes.get(self.hole_cursor).ok_or_else(|| b"fem3d.numerical-solid-hole".to_vec())?;
                if let Some(point) = hole.get(self.point_cursor) {
                    self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-hole-owner".to_vec())?.push_hole_point(self.hole_cursor, *point).map_err(|_| b"fem3d.numerical-solid-hole-point-slot".to_vec())?;
                    self.point_cursor += 1;
                } else {
                    self.hole_cursor += 1;
                    self.point_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidDomainHoleReserve;
                }
            }
            Fem3dNumericalStage::SolidMaterial => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let Some(material) = doc.materials.get(self.solid_material_cursor) else { return Err(b"fem3d.numerical-solid-material".to_vec()) };
                if material.id == solid.material_id {
                    self.resolved_solid_material = self.solid_material_cursor;
                    self.solid_material_cursor = 0;
                    self.point_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidOwnersReserve;
                } else {
                    self.solid_material_cursor += 1;
                }
            }
            Fem3dNumericalStage::SolidOwnersReserve => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let node_count = self.solid_points.len().checked_mul(solid.layers + 1).ok_or_else(|| b"fem3d.numerical-solid-node-overflow".to_vec())?;
                let tet_count = self.solid_tris.len().checked_mul(solid.layers).and_then(|count| count.checked_mul(3)).ok_or_else(|| b"fem3d.numerical-solid-element-overflow".to_vec())?;
                let model = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?;
                if model.nodes_len().saturating_add(node_count) > MAXIMUM_FIELDS || model.elements_len().saturating_add(tet_count) > MAXIMUM_ELEMENTS {
                    return Err(b"fem3d.numerical-solid-capacity".to_vec());
                }
                match self.point_cursor {
                    0 if self.solid_node_ids.admit_one(node_count).map_err(|_| b"fem3d.numerical-solid-node-id-admission".to_vec())? => self.point_cursor += 1,
                    1 if self.solid_node_analysis_indices.admit_one(node_count).map_err(|_| b"fem3d.numerical-solid-node-index-admission".to_vec())? => self.point_cursor += 1,
                    2 if model.admit_node_one(model.nodes_len() + node_count).map_err(|_| b"fem3d.numerical-solid-node-admission".to_vec())? => self.point_cursor += 1,
                    3 if self.analysis_node_ids.admit_one(self.analysis_node_ids.len() + node_count).map_err(|_| b"fem3d.numerical-solid-analysis-id-admission".to_vec())? => self.point_cursor += 1,
                    4 if model.admit_element_one(model.elements_len() + tet_count).map_err(|_| b"fem3d.numerical-solid-tet-admission".to_vec())? => self.point_cursor += 1,
                    _ => {
                        if self.point_cursor < 5 {
                            return Ok(false);
                        }
                        self.volume_node_cursor = 0;
                        self.lookup_cursor = 0;
                        self.point_cursor = 0;
                        self.stage = Fem3dNumericalStage::SolidNodeLookup;
                        return Ok(false);
                    }
                }
            }
            Fem3dNumericalStage::SolidNodeLookup => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let total = self.solid_points.len().checked_mul(solid.layers + 1).ok_or_else(|| b"fem3d.numerical-solid-node-overflow".to_vec())?;
                if self.volume_node_cursor == total {
                    self.point_cursor = 0;
                    self.tet_cursor = 0;
                    self.tet_phase = 0;
                    self.stage = Fem3dNumericalStage::SolidTet;
                } else if let Some(node) = self.model.as_ref().and_then(|model| model.node(self.lookup_cursor)) {
                    let point = self.solid_point(doc, self.volume_node_cursor)?;
                    if (node.pos[0] - point[0]).abs() < 1e-9 && (node.pos[1] - point[1]).abs() < 1e-9 && (node.pos[2] - point[2]).abs() < 1e-9 {
                        self.pending_node_id = Some(node.id.clone());
                        self.pending_node_index = self.lookup_cursor;
                        self.pending_node_needs_analysis = false;
                        self.lookup_cursor = 0;
                        self.stage = Fem3dNumericalStage::SolidNodeId;
                    } else {
                        self.lookup_cursor += 1;
                    }
                } else {
                    self.lookup_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidNodeCreate;
                }
            }
            Fem3dNumericalStage::SolidNodeCreate => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let point = self.solid_point(doc, self.volume_node_cursor)?;
                let id = format!("{}_m{}", solid.id, self.volume_node_cursor);
                if id.len() > INPUT_BYTES {
                    return Err(b"fem3d.numerical-solid-node-id".to_vec());
                }
                let model = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?;
                self.pending_node_index = model.nodes_len();
                model.push_node(Node { id: id.clone(), pos: point }).map_err(|_| b"fem3d.numerical-solid-node-slot".to_vec())?;
                self.pending_node_id = Some(id);
                self.pending_node_needs_analysis = true;
                self.stage = Fem3dNumericalStage::SolidNodeId;
            }
            Fem3dNumericalStage::SolidNodeId => {
                let id = self.pending_node_id.as_ref().ok_or_else(|| b"fem3d.numerical-solid-node-owner".to_vec())?;
                match self.point_cursor {
                    0 => {
                        if self.pending_node_needs_analysis {
                            self.analysis_node_ids.push(id.clone()).map_err(|_| b"fem3d.numerical-solid-analysis-id-slot".to_vec())?;
                        }
                        self.point_cursor = 1;
                    }
                    1 => {
                        self.solid_node_ids.push(id.clone()).map_err(|_| b"fem3d.numerical-solid-node-id-slot".to_vec())?;
                        self.point_cursor = 2;
                    }
                    _ => {
                        self.solid_node_analysis_indices.push(self.pending_node_index).map_err(|_| b"fem3d.numerical-solid-node-index-slot".to_vec())?;
                        self.pending_node_id = None;
                        self.point_cursor = 0;
                        self.volume_node_cursor += 1;
                        self.lookup_cursor = 0;
                        self.stage = Fem3dNumericalStage::SolidNodeLookup;
                    }
                }
            }
            Fem3dNumericalStage::SolidTet => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                if self.point_cursor == solid.layers {
                    self.stage = Fem3dNumericalStage::SolidIndicesRetire;
                } else if self.tet_cursor == self.solid_tris.len() {
                    self.point_cursor += 1;
                    self.tet_cursor = 0;
                    self.tet_phase = 0;
                } else {
                    let material = doc.materials.get(self.resolved_solid_material).ok_or_else(|| b"fem3d.numerical-solid-material".to_vec())?;
                    let indices = self.solid_tet_indices()?;
                    let nodes = [
                        self.solid_node_ids.get(indices[0]).cloned().ok_or_else(|| b"fem3d.numerical-solid-node-id".to_vec())?,
                        self.solid_node_ids.get(indices[1]).cloned().ok_or_else(|| b"fem3d.numerical-solid-node-id".to_vec())?,
                        self.solid_node_ids.get(indices[2]).cloned().ok_or_else(|| b"fem3d.numerical-solid-node-id".to_vec())?,
                        self.solid_node_ids.get(indices[3]).cloned().ok_or_else(|| b"fem3d.numerical-solid-node-id".to_vec())?,
                    ];
                    let positions = [self.solid_point(doc, indices[0])?, self.solid_point(doc, indices[1])?, self.solid_point(doc, indices[2])?, self.solid_point(doc, indices[3])?];
                    let a = [positions[1][0] - positions[0][0], positions[1][1] - positions[0][1], positions[1][2] - positions[0][2]];
                    let b = [positions[2][0] - positions[0][0], positions[2][1] - positions[0][1], positions[2][2] - positions[0][2]];
                    let c = [positions[3][0] - positions[0][0], positions[3][1] - positions[0][1], positions[3][2] - positions[0][2]];
                    let volume = (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])).abs() / 6.0;
                    self.pending_tet_mass = material.rho * volume / 4.0;
                    self.pending_tet_indices = indices;
                    self.pending_tet = Some(Tet4 { id: format!("{}_c{}_{}", solid.id, self.point_cursor * self.solid_tris.len() + self.tet_cursor, self.tet_phase), nodes, e: material.e, nu: material.nu, density: material.rho }.into());
                    self.mass_update_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidTetMass;
                }
            }
            Fem3dNumericalStage::SolidTetMass => {
                if self.mass_update_cursor < 12 {
                    let index = self.pending_tet_indices[self.mass_update_cursor / 3];
                    let analysis = *self.solid_node_analysis_indices.get(index).ok_or_else(|| b"fem3d.numerical-solid-node-index".to_vec())?;
                    self.modal_lumped_mass[analysis * 6 + self.mass_update_cursor % 3] += self.pending_tet_mass;
                    self.mass_update_cursor += 1;
                } else {
                    self.stage = Fem3dNumericalStage::SolidTetCommit;
                }
            }
            Fem3dNumericalStage::SolidTetCommit => {
                let tet = self.pending_tet.take().ok_or_else(|| b"fem3d.numerical-solid-tet-owner".to_vec())?;
                if let Err(tet) = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.push_element(tet) {
                    self.pending_tet = Some(tet);
                    return Err(b"fem3d.numerical-solid-tet-slot".to_vec());
                }
                self.tet_phase += 1;
                if self.tet_phase == 3 {
                    self.tet_phase = 0;
                    self.tet_cursor += 1;
                }
                self.stage = Fem3dNumericalStage::SolidTet;
            }
            Fem3dNumericalStage::SolidIndicesRetire => {
                if self.solid_node_analysis_indices.pop().is_some() {
                    return Ok(false);
                }
                if !self.solid_node_analysis_indices.close_admission_one() {
                    return Ok(false);
                }
                self.stage = Fem3dNumericalStage::SolidCommit;
            }
            Fem3dNumericalStage::SolidCommit => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let retained = Fem3dMeshedSolid {
                    solid_index: self.solid_cursor,
                    node_ids: std::mem::replace(&mut self.solid_node_ids, FixedSlots::new()),
                    top_offset: solid.layers * self.solid_points.len(),
                    points: std::mem::replace(&mut self.solid_points, FixedSlots::new()),
                    tris: std::mem::replace(&mut self.solid_tris, FixedSlots::new()),
                };
                if let Err(retained) = self.meshed_solids.push(retained) {
                    self.solid_node_ids = retained.node_ids;
                    self.solid_points = retained.points;
                    self.solid_tris = retained.tris;
                    return Err(b"fem3d.numerical-meshed-solid-slot".to_vec());
                }
                self.solid_materials[self.solid_cursor] = self.resolved_solid_material;
                self.solid_cursor += 1;
                self.stage = Fem3dNumericalStage::SolidDomainOuterReserve;
            }
            Fem3dNumericalStage::SupportDofReserve => {
                let Some(support) = doc.supports.get(self.support_cursor) else {
                    self.stage = Fem3dNumericalStage::PublishFieldCount;
                    return Ok(false);
                };
                if support.fixed.len() > 6 {
                    return Err(b"fem3d.numerical-support-dof-capacity".to_vec());
                }
                self.pending_support = Some(MountedAnalysisSupport::new(support.node_id.clone()));
                self.stage = Fem3dNumericalStage::SupportDof;
            }
            Fem3dNumericalStage::SupportDof => {
                let support = doc.supports.get(self.support_cursor).ok_or_else(|| b"fem3d.numerical-support".to_vec())?;
                if let Some(dof) = support.fixed.get(self.dof_cursor) {
                    self.pending_support.as_mut().ok_or_else(|| b"fem3d.numerical-support-owner".to_vec())?.push_fixed(Dof::from(*dof)).map_err(|_| b"fem3d.numerical-support-dof-slot".to_vec())?;
                    self.dof_cursor += 1;
                } else {
                    self.stage = Fem3dNumericalStage::SupportCommit;
                }
            }
            Fem3dNumericalStage::SupportCommit => {
                let support = self.pending_support.take().ok_or_else(|| b"fem3d.numerical-support-owner".to_vec())?;
                if let Err(support) = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.push_support(support) {
                    self.pending_support = Some(support);
                    return Err(b"fem3d.numerical-support-slot".to_vec());
                }
                self.support_cursor += 1;
                self.dof_cursor = 0;
                self.stage = Fem3dNumericalStage::SupportDofReserve;
            }
            _ => return Ok(true),
        }
        Ok(false)
    }

    fn step(&mut self, doc: &Fem3dSnapshot, solver: &mut Fem3dSolverView, backing: &mut Fem3dBackingCredit, freshness: Fem3dVisualFreshness, operation: semio_framework_job::Operation, context: &mut StepContext<'_>) -> Result<bool, Vec<u8>> {
        self.operation = Some(operation);
        let delegated = matches!(self.stage, Fem3dNumericalStage::SolidMesh | Fem3dNumericalStage::Assembly | Fem3dNumericalStage::Pcg | Fem3dNumericalStage::Ldlt | Fem3dNumericalStage::Subspace);
        if !delegated {
            if context.is_cancelled() {
                return Err(b"fem3d.numerical-cancelled".to_vec());
            }
            if context.should_yield() {
                return Ok(false);
            }
            context.consume_fuel(1);
            if context.is_cancelled() {
                return Err(b"fem3d.numerical-cancelled".to_vec());
            }
        }
        if self.stage == Fem3dNumericalStage::ReserveSolverPages {
            if self.solver_page_cursor == FEM3D_SOLVER_PAGE_COUNT {
                self.stage = Fem3dNumericalStage::ReserveNodes;
            } else {
                if !solver.admit_page(self.solver_page_cursor, self.solver_page_lane, backing) {
                    return Err(b"fem3d.numerical-solver-page-admission".to_vec());
                }
                if self.solver_page_lane {
                    self.solver_page_cursor += 1;
                }
                self.solver_page_lane = !self.solver_page_lane;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshBegin {
            let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
            let layers = solid.layers.max(1);
            let model = self.model.as_ref().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?;
            let maximum_points = MAXIMUM_FIELDS.saturating_sub(model.nodes_len()) / (layers + 1);
            let maximum_triangles = MAXIMUM_ELEMENTS.saturating_sub(model.elements_len()) / (layers * 3);
            if maximum_points < 3 || maximum_triangles == 0 {
                return Err(b"fem3d.numerical-solid-admission".to_vec());
            }
            let domain = self.solid_domain.take().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?;
            self.mesh = Some(MeshJob::new_mounted_bounded(domain, MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 }, operation, maximum_points, maximum_triangles));
            self.stage = Fem3dNumericalStage::SolidMesh;
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMesh {
            return match self.mesh.as_mut().ok_or_else(|| b"fem3d.numerical-solid-mesh-child".to_vec())?.step(context) {
                StepOutcome::Complete(_) => {
                    self.point_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidMeshReservePoints;
                    Ok(false)
                }
                StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => Ok(false),
                StepOutcome::Cancelled => Err(b"fem3d.numerical-cancelled".to_vec()),
                StepOutcome::Fault(fault) => Err(self.retain_fault(fault.detail)),
            };
        }
        if self.stage == Fem3dNumericalStage::SolidMeshReservePoints {
            let (points, _) = self.mesh.as_ref().and_then(MeshJob::completed_counts).ok_or_else(|| b"fem3d.numerical-solid-mesh-false-terminal".to_vec())?;
            if self.solid_points.admit_one(points).map_err(|_| b"fem3d.numerical-solid-mesh-point-admission".to_vec())? {
                self.point_cursor = 0;
                self.stage = Fem3dNumericalStage::SolidMeshCopyPoint;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshCopyPoint {
            let (points, _) = self.mesh.as_ref().and_then(MeshJob::completed_counts).ok_or_else(|| b"fem3d.numerical-solid-mesh-false-terminal".to_vec())?;
            if self.point_cursor < points {
                let point = self.mesh.as_ref().and_then(|mesh| mesh.completed_point(self.point_cursor)).ok_or_else(|| b"fem3d.numerical-solid-mesh-point".to_vec())?;
                self.solid_points.push(point).map_err(|_| b"fem3d.numerical-solid-mesh-point-slot".to_vec())?;
                self.point_cursor += 1;
            } else {
                self.stage = Fem3dNumericalStage::SolidMeshReserveTriangles;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshReserveTriangles {
            let (_, triangles) = self.mesh.as_ref().and_then(MeshJob::completed_counts).ok_or_else(|| b"fem3d.numerical-solid-mesh-false-terminal".to_vec())?;
            if self.solid_tris.admit_one(triangles).map_err(|_| b"fem3d.numerical-solid-mesh-triangle-admission".to_vec())? {
                self.point_cursor = 0;
                self.stage = Fem3dNumericalStage::SolidMeshCopyTriangle;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshCopyTriangle {
            let (_, triangles) = self.mesh.as_ref().and_then(MeshJob::completed_counts).ok_or_else(|| b"fem3d.numerical-solid-mesh-false-terminal".to_vec())?;
            if self.point_cursor < triangles {
                let triangle = self.mesh.as_ref().and_then(|mesh| mesh.completed_triangle(self.point_cursor)).ok_or_else(|| b"fem3d.numerical-solid-mesh-triangle".to_vec())?;
                self.solid_tris.push(triangle).map_err(|_| b"fem3d.numerical-solid-mesh-triangle-slot".to_vec())?;
                self.point_cursor += 1;
            } else {
                self.stage = Fem3dNumericalStage::SolidMeshRetire;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshRetire {
            let (terminal, _, _) = self.mesh.as_mut().ok_or_else(|| b"fem3d.numerical-solid-mesh-child".to_vec())?.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
            if terminal {
                self.mesh = None;
                self.stage = Fem3dNumericalStage::SolidMaterial;
            }
            return Ok(false);
        }
        if matches!(
            self.stage,
            Fem3dNumericalStage::ReserveNodes
                | Fem3dNumericalStage::ReserveNodeIds
                | Fem3dNumericalStage::ReserveElements
                | Fem3dNumericalStage::ReserveSupports
                | Fem3dNumericalStage::ReserveMeshedSolids
                | Fem3dNumericalStage::Nodes
                | Fem3dNumericalStage::NodeId
                | Fem3dNumericalStage::ElementMaterial
                | Fem3dNumericalStage::ElementSection
                | Fem3dNumericalStage::ElementStart
                | Fem3dNumericalStage::ElementEnd
                | Fem3dNumericalStage::ElementCommit
                | Fem3dNumericalStage::ElementMass
                | Fem3dNumericalStage::ElementInsert
                | Fem3dNumericalStage::SolidDomainOuterReserve
                | Fem3dNumericalStage::SolidDomainOuterPoint
                | Fem3dNumericalStage::SolidDomainHolesReserve
                | Fem3dNumericalStage::SolidDomainHoleReserve
                | Fem3dNumericalStage::SolidDomainHolePoint
                | Fem3dNumericalStage::SolidMaterial
                | Fem3dNumericalStage::SolidOwnersReserve
                | Fem3dNumericalStage::SolidNodeLookup
                | Fem3dNumericalStage::SolidNodeCreate
                | Fem3dNumericalStage::SolidNodeId
                | Fem3dNumericalStage::SolidTet
                | Fem3dNumericalStage::SolidTetMass
                | Fem3dNumericalStage::SolidTetCommit
                | Fem3dNumericalStage::SolidIndicesRetire
                | Fem3dNumericalStage::SolidCommit
                | Fem3dNumericalStage::SupportDofReserve
                | Fem3dNumericalStage::SupportDof
                | Fem3dNumericalStage::SupportCommit
        ) {
            self.step_model(doc)?;
            return Ok(false);
        }
        match self.stage {
            Fem3dNumericalStage::PublishFieldCount => {
                if !solver.set_len(freshness, self.analysis_node_ids.len()) {
                    return Err(b"fem3d.numerical-field-count".to_vec());
                }
                self.stage = Fem3dNumericalStage::MountAssembly;
            }
            Fem3dNumericalStage::MountAssembly => {
                let model = Arc::new(self.model.take().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?);
                self.assembly_build = Some(AssemblyJobConstruction::new_mounted(model, operation, 1));
                self.stage = Fem3dNumericalStage::PrepareAssembly;
            }
            Fem3dNumericalStage::PrepareAssembly => match self.assembly_build.as_mut().ok_or_else(|| b"fem3d.numerical-assembly-build".to_vec())?.step_one() {
                Ok(false) => {}
                Ok(true) => {
                    self.assembly = self.assembly_build.as_mut().and_then(AssemblyJobConstruction::take_complete);
                    if self.assembly.is_none() {
                        return Err(b"fem3d.numerical-assembly-false-terminal".to_vec());
                    }
                    self.stage = Fem3dNumericalStage::Assembly;
                }
                Err(error) => return Err(error.to_string().into_bytes()),
            },
            Fem3dNumericalStage::Assembly => match self.assembly.as_mut().ok_or_else(|| b"fem3d.numerical-assembly".to_vec())?.step(context) {
                StepOutcome::Complete(_) => {
                    self.node_cursor = 0;
                    self.dof_cursor = 0;
                    self.free_order = self.assembly.as_ref().map_or(0, AssemblyJob::visual_free_order);
                    self.stage = Fem3dNumericalStage::ReserveModalMass;
                }
                StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => return Ok(false),
                StepOutcome::Cancelled => return Err(b"fem3d.numerical-cancelled".to_vec()),
                StepOutcome::Fault(fault) => return Err(self.retain_fault(fault.detail)),
            },
            Fem3dNumericalStage::ReserveModalMass => {
                if self.modal_free_mass.admit_one(self.free_order).map_err(|_| b"fem3d.numerical-modal-mass-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::InitializeModalMass;
                }
            }
            Fem3dNumericalStage::InitializeModalMass => {
                if self.modal_free_mass.len() < self.free_order {
                    self.modal_free_mass.push(0.0).map_err(|_| b"fem3d.numerical-modal-mass-slot".to_vec())?;
                } else {
                    self.node_cursor = 0;
                    self.dof_cursor = 0;
                    self.stage = Fem3dNumericalStage::MapEquation;
                }
            }
            Fem3dNumericalStage::MapEquation => {
                if self.node_cursor == self.analysis_node_ids.len() {
                    self.stage = Fem3dNumericalStage::ReserveRhs;
                } else if self.dof_cursor == 6 {
                    self.node_cursor += 1;
                    self.dof_cursor = 0;
                } else {
                    let dof = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz][self.dof_cursor];
                    if let Some((full, compact)) = self.assembly.as_ref().and_then(|assembly| self.analysis_node_ids.get(self.node_cursor).and_then(|node_id| assembly.visual_equation_indices(node_id, dof))) {
                        self.full_equations[self.node_cursor][self.dof_cursor] = Some(full);
                        self.equations[self.node_cursor][self.dof_cursor] = compact;
                        if let Some(compact) = compact {
                            self.modal_free_mass.add_at(compact, self.modal_lumped_mass[self.node_cursor * 6 + self.dof_cursor]).map_err(|_| b"fem3d.numerical-modal-mass-index".to_vec())?;
                        }
                    }
                    self.dof_cursor += 1;
                }
            }
            Fem3dNumericalStage::ReserveRhs => {
                if self.rhs.admit_one(self.free_order).map_err(|_| b"fem3d.numerical-rhs-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::InitializeRhs;
                }
            }
            Fem3dNumericalStage::InitializeRhs => {
                if self.rhs.len() < self.free_order {
                    self.rhs.push(0.0).map_err(|_| b"fem3d.numerical-rhs-slot".to_vec())?;
                } else {
                    self.node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ApplyLoad;
                }
            }
            Fem3dNumericalStage::ApplyLoad => {
                let Some(case) = doc.load_cases.first() else { return Err(b"fem3d.numerical-load-case".to_vec()) };
                let Some(load) = case.loads.get(self.load_cursor) else {
                    if case.self_weight {
                        self.load_element_cursor = 0;
                        self.load_triangle_cursor = 0;
                        self.load_node_cursor = 0;
                        self.stage = Fem3dNumericalStage::ResolveSelfWeightMemberNode;
                        return Ok(false);
                    }
                    let assembly = self.assembly.take().ok_or_else(|| b"fem3d.numerical-assembly-owner".to_vec())?;
                    self.csr_build = Some(AssemblyCsrBuild::new_free(assembly).map_err(|_| b"fem3d.numerical-assembly-terminal".to_vec())?);
                    self.stage = Fem3dNumericalStage::BuildCsr;
                    return Ok(false);
                };
                self.load_node_cursor = 0;
                self.stage = match load {
                    FemLoad::Nodal { .. } => Fem3dNumericalStage::ApplyNodalLoad,
                    FemLoad::MemberUdl { .. } => {
                        self.load_element_cursor = 0;
                        Fem3dNumericalStage::ResolveMemberLoad
                    }
                    FemLoad::Area { .. } => {
                        self.load_solid_cursor = 0;
                        Fem3dNumericalStage::ResolveAreaLoad
                    }
                };
            }
            Fem3dNumericalStage::ApplyNodalLoad => {
                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::Nodal { node_id, dof, value, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err(b"fem3d.numerical-load-kind".to_vec());
                };
                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-load-node".to_vec()) };
                if candidate == node_id {
                    let axis = Dof::from(*dof).index();
                    self.apply_rhs_value(self.load_node_cursor, axis, *value)?;
                    self.load_cursor += 1;
                    self.load_node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ApplyLoad;
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::ResolveMemberLoad => {
                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::MemberUdl { element_id, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err(b"fem3d.numerical-load-kind".to_vec());
                };
                let Some(element) = doc.elements.get(self.load_element_cursor) else { return Err(b"fem3d.numerical-member-load-element".to_vec()) };
                if crate::artifacts::fem3d::element_id(element) == element_id {
                    self.load_triangle_cursor = 0;
                    self.load_node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ResolveMemberNode;
                } else {
                    self.load_element_cursor += 1;
                }
            }
            Fem3dNumericalStage::ResolveMemberNode => {
                let element = doc.elements.get(self.load_element_cursor).ok_or_else(|| b"fem3d.numerical-member-load-element".to_vec())?;
                let (start, end) = match element {
                    FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start, end),
                };
                let target = if self.load_triangle_cursor == 0 { start } else { end };
                let Some(node) = doc.nodes.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-member-load-node".to_vec()) };
                if &node.id == target {
                    self.load_node_indices[self.load_triangle_cursor] = self.load_node_cursor;
                    self.load_positions[self.load_triangle_cursor] = [node.x, node.y, node.z];
                    self.load_triangle_cursor += 1;
                    self.load_node_cursor = 0;
                    if self.load_triangle_cursor == 2 {
                        self.load_vertex_cursor = 0;
                        self.stage = Fem3dNumericalStage::ApplyMemberScalar;
                    }
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::ApplyMemberScalar => {
                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::MemberUdl { wx, wy, wz, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err(b"fem3d.numerical-load-kind".to_vec());
                };
                if self.load_vertex_cursor == 12 {
                    self.load_cursor += 1;
                    self.load_element_cursor = 0;
                    self.stage = Fem3dNumericalStage::ApplyLoad;
                } else {
                    let endpoint = self.load_vertex_cursor / 6;
                    let axis = self.load_vertex_cursor % 6;
                    let d = [self.load_positions[1][0] - self.load_positions[0][0], self.load_positions[1][1] - self.load_positions[0][1], self.load_positions[1][2] - self.load_positions[0][2]];
                    let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                    if length <= f64::EPSILON {
                        return Err(b"fem3d.numerical-member-load-length".to_vec());
                    }
                    let force = [*wx * length * 0.5, *wy * length * 0.5, *wz * length * 0.5];
                    let moment = [(d[1] * *wz - d[2] * *wy) * length / 12.0, (d[2] * *wx - d[0] * *wz) * length / 12.0, (d[0] * *wy - d[1] * *wx) * length / 12.0];
                    let value = if axis < 3 {
                        force[axis]
                    } else if endpoint == 0 {
                        moment[axis - 3]
                    } else {
                        -moment[axis - 3]
                    };
                    self.apply_rhs_value(self.load_node_indices[endpoint], axis, value)?;
                    self.load_vertex_cursor += 1;
                }
            }
            Fem3dNumericalStage::ResolveAreaLoad => {
                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::Area { solid_id, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err(b"fem3d.numerical-load-kind".to_vec());
                };
                let Some(solid) = self.meshed_solids.get(self.load_solid_cursor) else { return Err(b"fem3d.numerical-area-load-solid".to_vec()) };
                if doc.solids.get(solid.solid_index).is_some_and(|candidate| &candidate.id == solid_id) {
                    self.load_triangle_cursor = 0;
                    self.load_vertex_cursor = 0;
                    self.load_node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ApplyAreaNode;
                } else {
                    self.load_solid_cursor += 1;
                }
            }
            Fem3dNumericalStage::ApplyAreaNode => {
                let case = doc.load_cases.first().ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::Area { pressure, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err(b"fem3d.numerical-load-kind".to_vec());
                };
                let solid = self.meshed_solids.get(self.load_solid_cursor).ok_or_else(|| b"fem3d.numerical-area-load-solid".to_vec())?;
                let Some(triangle) = solid.tris.get(self.load_triangle_cursor) else {
                    self.load_cursor += 1;
                    self.load_solid_cursor = 0;
                    self.stage = Fem3dNumericalStage::ApplyLoad;
                    return Ok(false);
                };
                let p0 = *solid.points.get(triangle[0] as usize).ok_or_else(|| b"fem3d.numerical-area-load-point".to_vec())?;
                let p1 = *solid.points.get(triangle[1] as usize).ok_or_else(|| b"fem3d.numerical-area-load-point".to_vec())?;
                let p2 = *solid.points.get(triangle[2] as usize).ok_or_else(|| b"fem3d.numerical-area-load-point".to_vec())?;
                let area = (0.5 * ((p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]))).abs();
                let point = triangle[self.load_vertex_cursor] as usize;
                let node_id = solid.node_ids.get(solid.top_offset + point).ok_or_else(|| b"fem3d.numerical-area-load-node".to_vec())?;
                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-area-load-node".to_vec()) };
                if candidate == node_id {
                    self.apply_rhs_value(self.load_node_cursor, Dof::Tz.index(), -*pressure * area / 3.0)?;
                    self.load_node_cursor = 0;
                    self.load_vertex_cursor += 1;
                    if self.load_vertex_cursor == 3 {
                        self.load_vertex_cursor = 0;
                        self.load_triangle_cursor += 1;
                    }
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::ResolveSelfWeightMemberNode => {
                let Some(element) = doc.elements.get(self.load_element_cursor) else {
                    self.load_solid_cursor = 0;
                    self.load_triangle_cursor = 0;
                    self.load_vertex_cursor = 0;
                    self.load_node_cursor = 0;
                    self.point_cursor = 0;
                    self.tet_phase = 0;
                    self.stage = Fem3dNumericalStage::ApplySelfWeightSolid;
                    return Ok(false);
                };
                let (start, end) = match element {
                    FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start, end),
                };
                let target = if self.load_triangle_cursor == 0 { start } else { end };
                let Some(node) = doc.nodes.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-self-weight-node".to_vec()) };
                if &node.id == target {
                    self.load_node_indices[self.load_triangle_cursor] = self.load_node_cursor;
                    self.load_positions[self.load_triangle_cursor] = [node.x, node.y, node.z];
                    self.load_triangle_cursor += 1;
                    self.load_node_cursor = 0;
                    if self.load_triangle_cursor == 2 {
                        self.load_vertex_cursor = 0;
                        self.stage = Fem3dNumericalStage::ApplySelfWeightMember;
                    }
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::ApplySelfWeightMember => {
                if self.load_vertex_cursor == 2 {
                    self.load_element_cursor += 1;
                    self.load_triangle_cursor = 0;
                    self.load_node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ResolveSelfWeightMemberNode;
                } else {
                    let material = doc.materials.get(self.element_materials[self.load_element_cursor]).ok_or_else(|| b"fem3d.numerical-self-weight-material".to_vec())?;
                    let section = doc.sections.get(self.element_sections[self.load_element_cursor]).ok_or_else(|| b"fem3d.numerical-self-weight-section".to_vec())?;
                    let d = [self.load_positions[1][0] - self.load_positions[0][0], self.load_positions[1][1] - self.load_positions[0][1], self.load_positions[1][2] - self.load_positions[0][2]];
                    let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                    self.apply_rhs_value(self.load_node_indices[self.load_vertex_cursor], Dof::Tz.index(), -material.rho * section.area * length * 9.80665 * 0.5)?;
                    self.load_vertex_cursor += 1;
                }
            }
            Fem3dNumericalStage::ApplySelfWeightSolid => {
                let Some(meshed) = self.meshed_solids.get(self.load_solid_cursor) else {
                    let assembly = self.assembly.take().ok_or_else(|| b"fem3d.numerical-assembly-owner".to_vec())?;
                    self.csr_build = Some(AssemblyCsrBuild::new_free(assembly).map_err(|_| b"fem3d.numerical-assembly-terminal".to_vec())?);
                    self.stage = Fem3dNumericalStage::BuildCsr;
                    return Ok(false);
                };
                let solid = doc.solids.get(meshed.solid_index).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                if self.point_cursor == solid.layers {
                    self.load_solid_cursor += 1;
                    self.load_triangle_cursor = 0;
                    self.load_vertex_cursor = 0;
                    self.load_node_cursor = 0;
                    self.point_cursor = 0;
                    self.tet_phase = 0;
                    return Ok(false);
                }
                if self.load_triangle_cursor == meshed.tris.len() {
                    self.point_cursor += 1;
                    self.load_triangle_cursor = 0;
                    self.load_vertex_cursor = 0;
                    self.tet_phase = 0;
                    return Ok(false);
                }
                if self.tet_phase == 3 {
                    self.load_triangle_cursor += 1;
                    self.load_vertex_cursor = 0;
                    self.tet_phase = 0;
                    return Ok(false);
                }
                if self.load_vertex_cursor == 4 {
                    self.tet_phase += 1;
                    self.load_vertex_cursor = 0;
                    self.load_node_cursor = 0;
                    return Ok(false);
                }
                let triangle = *meshed.tris.get(self.load_triangle_cursor).ok_or_else(|| b"fem3d.numerical-self-weight-solid-triangle".to_vec())?;
                let indices = Self::tet_indices(meshed.points.len(), triangle, self.point_cursor, self.tet_phase);
                let node_id = meshed.node_ids.get(indices[self.load_vertex_cursor]).ok_or_else(|| b"fem3d.numerical-self-weight-solid-node".to_vec())?;
                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err(b"fem3d.numerical-self-weight-solid-node".to_vec()) };
                if candidate == node_id {
                    let position = |index: usize| -> Result<[f64; 3], Vec<u8>> {
                        let layer = index / meshed.points.len();
                        let point = *meshed.points.get(index % meshed.points.len()).ok_or_else(|| b"fem3d.numerical-self-weight-solid-point".to_vec())?;
                        Ok([point[0], point[1], solid.base_z + solid.height * layer as f64 / solid.layers as f64])
                    };
                    let p0 = position(indices[0])?;
                    let p1 = position(indices[1])?;
                    let p2 = position(indices[2])?;
                    let p3 = position(indices[3])?;
                    let a = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
                    let b = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
                    let c = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];
                    let volume = (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])).abs() / 6.0;
                    let material = doc.materials.get(self.solid_materials[meshed.solid_index]).ok_or_else(|| b"fem3d.numerical-self-weight-material".to_vec())?;
                    self.apply_rhs_value(self.load_node_cursor, Dof::Tz.index(), -material.rho * volume * 9.80665 / 4.0)?;
                    self.load_vertex_cursor += 1;
                    self.load_node_cursor = 0;
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::BuildCsr => match self.csr_build.as_mut().ok_or_else(|| b"fem3d.numerical-csr".to_vec())?.step_one() {
                Ok(false) => {}
                Ok(true) => {
                    self.stage = Fem3dNumericalStage::BeginPcg;
                }
                Err(detail) => return Err(detail.to_vec()),
            },
            Fem3dNumericalStage::BeginPcg => {
                let matrix = self.csr_build.as_mut().and_then(AssemblyCsrBuild::take_complete).ok_or_else(|| b"fem3d.numerical-csr-terminal".to_vec())?;
                match PcgJobConstruction::new_with_mounted_rhs(operation, matrix, std::mem::take(&mut self.rhs)) {
                    Ok(construction) => self.pcg_build = Some(construction),
                    Err((matrix, rhs)) => {
                        self.rejected_pcg_matrix = Some(matrix);
                        self.rejected_pcg_rhs = Some(rhs);
                        return Err(b"fem3d.numerical-pcg-rhs".to_vec());
                    }
                }
                self.stage = Fem3dNumericalStage::PreparePcg;
            }
            Fem3dNumericalStage::PreparePcg => match self.pcg_build.as_mut().ok_or_else(|| b"fem3d.numerical-pcg-build".to_vec())?.step_one() {
                Ok(false) => {}
                Ok(true) => {
                    self.pcg = self.pcg_build.as_mut().and_then(PcgJobConstruction::take_complete);
                    if self.pcg.is_none() {
                        return Err(b"fem3d.numerical-pcg-false-terminal".to_vec());
                    }
                    self.stage = Fem3dNumericalStage::Pcg;
                }
                Err(detail) => return Err(detail.to_vec()),
            },
            Fem3dNumericalStage::Pcg => match self.pcg.as_mut().ok_or_else(|| b"fem3d.numerical-pcg".to_vec())?.step(context) {
                StepOutcome::Complete(_) => {
                    self.stage = Fem3dNumericalStage::ReadNodeScalar;
                    self.node_cursor = 0;
                    self.scalar_axis = 0;
                }
                StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => return Ok(false),
                StepOutcome::Cancelled => return Err(b"fem3d.numerical-cancelled".to_vec()),
                StepOutcome::Fault(fault) => return Err(self.retain_fault(fault.detail)),
            },
            Fem3dNumericalStage::ReadNodeScalar => {
                if self.node_cursor == self.analysis_node_ids.len() {
                    self.stage = Fem3dNumericalStage::BeginModal;
                } else {
                    if let Some(equation) = self.equations[self.node_cursor][self.scalar_axis] {
                        let value = self.pcg.as_ref().and_then(|pcg| pcg.visual_scalar(equation)).ok_or_else(|| b"fem3d.numerical-node-scalar".to_vec())?;
                        self.scalar.displacement[self.scalar_axis] = value.displacement;
                        self.scalar.residual[self.scalar_axis] = value.residual;
                        self.scalar.contour = self.scalar.contour.max(value.contour);
                    }
                    self.reaction_entry = 0;
                    self.reaction_accumulator = 0.0;
                    self.stage = Fem3dNumericalStage::RecoverReaction;
                }
            }
            Fem3dNumericalStage::RecoverReaction => {
                let Some(row) = self.full_equations[self.node_cursor][self.scalar_axis] else {
                    self.scalar_axis += 1;
                    self.stage = if self.scalar_axis == 3 { Fem3dNumericalStage::PublishNodeScalar } else { Fem3dNumericalStage::ReadNodeScalar };
                    return Ok(false);
                };
                if let Some((entry_row, entry_col, value)) = self.csr_build.as_ref().and_then(|build| build.visual_full_entry(self.reaction_entry)) {
                    if entry_row == row {
                        let displacement = self.csr_build.as_ref().and_then(|build| build.visual_compact_index(entry_col)).and_then(|compact| self.pcg.as_ref().and_then(|pcg| pcg.visual_scalar(compact))).map_or(0.0, |scalar| scalar.displacement);
                        self.reaction_accumulator += value * displacement;
                    }
                    self.reaction_entry += 1;
                } else {
                    self.scalar.reaction[self.scalar_axis] = self.reaction_accumulator - self.full_rhs[row];
                    self.scalar_axis += 1;
                    self.stage = if self.scalar_axis == 3 { Fem3dNumericalStage::PublishNodeScalar } else { Fem3dNumericalStage::ReadNodeScalar };
                }
            }
            Fem3dNumericalStage::PublishNodeScalar => {
                solver.publish_scalar(freshness, self.node_cursor, self.scalar).map_err(|_| b"fem3d.numerical-solver-publication".to_vec())?;
                self.node_cursor += 1;
                self.scalar_axis = 0;
                self.scalar = Fem3dSolverScalar::default();
                self.stage = Fem3dNumericalStage::ReadNodeScalar;
            }
            Fem3dNumericalStage::BeginModal => {
                let matrix = self.pcg.as_mut().and_then(PcgJob::take_completed_matrix).ok_or_else(|| b"fem3d.numerical-modal-matrix".to_vec())?;
                self.modal_build = Some(ModalInputConstruction::new_mounted(matrix, std::mem::take(&mut self.modal_free_mass)));
                self.stage = Fem3dNumericalStage::PrepareModal;
            }
            Fem3dNumericalStage::PrepareModal => match self.modal_build.as_mut().ok_or_else(|| b"fem3d.numerical-modal-build".to_vec())?.step_one() {
                Ok(false) => {}
                Ok(true) => {
                    self.stage = Fem3dNumericalStage::BeginLdlt;
                }
                Err(detail) => return Err(detail.to_vec()),
            },
            Fem3dNumericalStage::BeginLdlt => {
                let (stiffness, mass) = self.modal_build.as_mut().and_then(ModalInputConstruction::take_complete).ok_or_else(|| b"fem3d.numerical-modal-terminal".to_vec())?;
                if stiffness.n == 0 || stiffness.n > 40 {
                    return Err(b"fem3d.numerical-modal-order".to_vec());
                }
                self.modal_mass = Some(mass);
                self.ldlt = Some(LdltJob::new(operation, stiffness, 1));
                self.stage = Fem3dNumericalStage::Ldlt;
            }
            Fem3dNumericalStage::Ldlt => match self.ldlt.as_mut().ok_or_else(|| b"fem3d.numerical-ldlt".to_vec())?.step(context) {
                StepOutcome::Complete(_) => {
                    self.stage = Fem3dNumericalStage::BeginSubspace;
                }
                StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => return Ok(false),
                StepOutcome::Cancelled => return Err(b"fem3d.numerical-cancelled".to_vec()),
                StepOutcome::Fault(fault) => return Err(self.retain_fault(fault.detail)),
            },
            Fem3dNumericalStage::BeginSubspace => {
                let factor = self.ldlt.as_mut().and_then(LdltJob::take_factor).ok_or_else(|| b"fem3d.numerical-ldlt-factor".to_vec())?;
                let mass = self.modal_mass.take().ok_or_else(|| b"fem3d.numerical-modal-mass".to_vec())?;
                let order = mass.n;
                self.subspace = Some(SubspaceIterationJob::new(operation, factor, mass, order, 1, 30));
                self.stage = Fem3dNumericalStage::Subspace;
            }
            Fem3dNumericalStage::Subspace => match self.subspace.as_mut().ok_or_else(|| b"fem3d.numerical-subspace".to_vec())?.step(context) {
                StepOutcome::Complete(_) => {
                    self.node_cursor = 0;
                    self.scalar_axis = 0;
                    self.stage = Fem3dNumericalStage::ReadModeScalar;
                }
                StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => return Ok(false),
                StepOutcome::Cancelled => return Err(b"fem3d.numerical-cancelled".to_vec()),
                StepOutcome::Fault(fault) => return Err(self.retain_fault(fault.detail)),
            },
            Fem3dNumericalStage::ReadModeScalar => {
                if self.node_cursor == self.analysis_node_ids.len() {
                    self.stage = Fem3dNumericalStage::PublishProgress;
                } else {
                    if let Some(equation) = self.equations[self.node_cursor][self.scalar_axis] {
                        let (component, eigenvalue) = self.subspace.as_ref().and_then(|subspace| subspace.visual_mode_scalar(0, equation)).ok_or_else(|| b"fem3d.numerical-mode-scalar".to_vec())?;
                        self.scalar.mode_shape[self.scalar_axis] = component;
                        self.scalar.eigen_estimate = eigenvalue;
                    }
                    self.scalar_axis += 1;
                    if self.scalar_axis == 3 {
                        self.stage = Fem3dNumericalStage::PublishModeScalar;
                    }
                }
            }
            Fem3dNumericalStage::PublishModeScalar => {
                let mut scalar = solver.scalar(self.node_cursor).ok_or_else(|| b"fem3d.numerical-static-scalar".to_vec())?;
                scalar.mode_shape = self.scalar.mode_shape;
                scalar.eigen_estimate = self.scalar.eigen_estimate;
                solver.publish_scalar(freshness, self.node_cursor, scalar).map_err(|_| b"fem3d.numerical-mode-publication".to_vec())?;
                self.node_cursor += 1;
                self.scalar_axis = 0;
                self.scalar = Fem3dSolverScalar::default();
                self.stage = Fem3dNumericalStage::ReadModeScalar;
            }
            Fem3dNumericalStage::PublishProgress => {
                let (_, _, residual, tolerance, converged) = self.pcg.as_ref().ok_or_else(|| b"fem3d.numerical-pcg".to_vec())?.visual_progress();
                let (_, _, mode_residual, modes_converged) = self.subspace.as_ref().ok_or_else(|| b"fem3d.numerical-subspace".to_vec())?.visual_progress();
                let state = if converged && modes_converged { Fem3dVisualState::ValidatedFinal } else { Fem3dVisualState::SolvingUnconverged };
                if !solver.publish_progress(freshness, state, residual.max(mode_residual), tolerance, self.analysis_node_ids.len(), self.analysis_node_ids.len()) {
                    return Err(b"fem3d.numerical-progress-publication".to_vec());
                }
                self.stage = Fem3dNumericalStage::Complete;
            }
            Fem3dNumericalStage::Complete => return Ok(true),
            _ => {}
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    fn close_interactive(step: semio_framework_job::InteractiveJobCloseStep) -> (bool, usize, usize) {
        match step {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => (false, released_items, released_bytes),
            semio_framework_job::InteractiveJobCloseStep::Blocked => (false, 0, 0),
            semio_framework_job::InteractiveJobCloseStep::Complete => (true, 0, 0),
        }
    }

    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(payload) = self.fault_payload.as_mut() {
            return match payload.close_step(1, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => (false, released_items, released_bytes),
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.fault_payload = None;
                    (false, 1, 0)
                }
            };
        }
        if let Some(matrix) = self.rejected_pcg_matrix.as_mut() {
            let step = matrix.close_step(maximum_bytes);
            if step.0 {
                self.rejected_pcg_matrix = None;
            }
            return (false, step.1, step.2);
        }
        if let Some(rhs) = self.rejected_pcg_rhs.as_mut() {
            if rhs.close_step() {
                self.rejected_pcg_rhs = None;
            }
            return (false, 1, 0);
        }
        let step = match self.close_lane {
            0 => {
                let Some(job) = self.mesh.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.mesh = None;
                }
                step
            }
            1 => {
                let Some(job) = self.assembly_build.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.assembly_build = None;
                }
                step
            }
            2 => {
                let Some(job) = self.assembly.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.assembly = None;
                }
                step
            }
            3 => {
                let Some(job) = self.csr_build.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.csr_build = None;
                }
                step
            }
            4 => {
                let Some(job) = self.pcg_build.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.pcg_build = None;
                }
                step
            }
            5 => {
                let Some(job) = self.pcg.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.pcg = None;
                }
                step
            }
            6 => {
                let Some(job) = self.modal_build.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.modal_build = None;
                }
                step
            }
            7 => {
                let Some(job) = self.ldlt.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                if self.close_started_jobs & 1 == 0 {
                    job.begin_close();
                    self.close_started_jobs |= 1;
                    return (false, 1, 0);
                }
                let step = Self::close_interactive(job.close_step(1, maximum_bytes));
                if step.0 {
                    self.ldlt = None;
                }
                step
            }
            8 => {
                let Some(job) = self.subspace.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                if self.close_started_jobs & 2 == 0 {
                    job.begin_close();
                    self.close_started_jobs |= 2;
                    return (false, 1, 0);
                }
                let step = Self::close_interactive(job.close_step(1, maximum_bytes));
                if step.0 {
                    self.subspace = None;
                }
                step
            }
            9 => {
                let Some(mass) = self.modal_mass.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                let step = mass.close_step(maximum_bytes);
                if step.0 {
                    self.modal_mass = None;
                }
                step
            }
            10 => {
                let Some(domain) = self.solid_domain.as_mut() else {
                    self.close_lane += 1;
                    return (false, 0, 0);
                };
                if domain.close_step() {
                    self.solid_domain = None;
                    (true, 1, 0)
                } else {
                    (false, 1, 0)
                }
            }
            11 => {
                if let Some(solid) = self.meshed_solids.len().checked_sub(1).and_then(|index| self.meshed_solids.get_mut(index)) {
                    if let Some(value) = solid.node_ids.len().checked_sub(1).and_then(|index| solid.node_ids.get_mut(index)) {
                        let bytes = value.capacity();
                        if bytes != 0 {
                            if bytes > maximum_bytes {
                                return (false, 0, 0);
                            }
                            *value = String::new();
                            return (false, 1, bytes);
                        }
                        solid.node_ids.pop();
                        return (false, 1, 0);
                    }
                    if !solid.node_ids.close_admission_one() {
                        return (false, 1, 0);
                    }
                    if solid.tris.pop().is_some() || !solid.tris.close_admission_one() {
                        return (false, 1, 0);
                    }
                    if solid.points.pop().is_some() || !solid.points.close_admission_one() {
                        return (false, 1, 0);
                    }
                    self.meshed_solids.pop();
                    return (false, 1, 0);
                }
                (self.meshed_solids.close_admission_one(), 1, 0)
            }
            12 => {
                if let Some(value) = self.pending_node_id.as_mut() {
                    let bytes = value.capacity();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        *value = String::new();
                        return (false, 1, bytes);
                    }
                    self.pending_node_id = None;
                    return (false, 1, 0);
                }
                (true, 0, 0)
            }
            13 => {
                if let Some(value) = self.solid_node_ids.len().checked_sub(1).and_then(|index| self.solid_node_ids.get_mut(index)) {
                    let bytes = value.capacity();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        *value = String::new();
                        return (false, 1, bytes);
                    }
                    self.solid_node_ids.pop();
                    return (false, 1, 0);
                }
                if !self.solid_node_ids.close_admission_one() {
                    return (false, 1, 0);
                }
                if self.solid_node_analysis_indices.pop().is_some() || !self.solid_node_analysis_indices.close_admission_one() {
                    return (false, 1, 0);
                }
                (true, 0, 0)
            }
            14 => {
                let Some(support) = self.pending_support.as_mut() else { return (true, 0, 0) };
                let step = support.close_step(maximum_bytes);
                if step.0 {
                    self.pending_support = None;
                }
                step
            }
            15 => {
                if let Some(value) = self.analysis_node_ids.len().checked_sub(1).and_then(|index| self.analysis_node_ids.get_mut(index)) {
                    let bytes = value.capacity();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        *value = String::new();
                        return (false, 1, bytes);
                    }
                    self.analysis_node_ids.pop();
                    return (false, 1, 0);
                }
                (self.analysis_node_ids.close_admission_one(), 1, 0)
            }
            16 => {
                if self.solid_tris.pop().is_some() || !self.solid_tris.close_admission_one() {
                    return (false, 1, 0);
                }
                if self.solid_points.pop().is_some() || !self.solid_points.close_admission_one() {
                    return (false, 1, 0);
                }
                (true, 0, 0)
            }
            17 => (self.rhs.close_step(), 1, 0),
            18 => (self.modal_free_mass.close_step(), 1, 0),
            19 => {
                let Some(element) = self.pending_element.as_mut() else { return (true, 0, 0) };
                if let Some(bytes) = element.mounted_next_string_bytes() {
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    let released = element.close_mounted_string_step().map_or(0, |bytes| bytes);
                    return (false, 1, released);
                }
                self.pending_element = None;
                (true, 1, 0)
            }
            20 => {
                let Some(element) = self.pending_tet.as_mut() else { return (true, 0, 0) };
                if let Some(bytes) = element.mounted_next_string_bytes() {
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    let released = element.close_mounted_string_step().map_or(0, |bytes| bytes);
                    return (false, 1, released);
                }
                self.pending_tet = None;
                (true, 1, 0)
            }
            21 => {
                let Some(model) = self.model.as_mut() else { return (true, 0, 0) };
                let step = model.close_step(maximum_bytes);
                if step.0 {
                    self.model = None;
                }
                step
            }
            _ => return (true, 0, 0),
        };
        if step.0 {
            self.close_lane += 1;
            return (false, step.1, step.2);
        }
        return step;
    }
}

struct FixedOrder<const N: usize> {
    slots: Box<[Option<usize>; N]>,
    len: usize,
}

enum Fem3dRecoveredBacking {
    SolverScalar(Box<[std::mem::MaybeUninit<Fem3dSolverScalar>; FEM3D_SOLVER_FIELDS_PER_PAGE]>),
    SolverInitialized(Box<[bool; FEM3D_SOLVER_FIELDS_PER_PAGE]>),
    RegionOrder(Box<[Option<usize>; MAXIMUM_REGIONS]>),
    ElementOrder(Box<[Option<usize>; MAXIMUM_ELEMENTS]>),
}

const FEM3D_RECOVERED_BACKING_CAPACITY: usize = SHELL_CAPACITY * FEM3D_PROCESS_BACKING_ITEMS;

struct Fem3dBackingRecovery {
    owners: [Option<Fem3dRecoveredBacking>; FEM3D_RECOVERED_BACKING_CAPACITY],
}

impl Fem3dBackingRecovery {
    const fn new() -> Self {
        Self { owners: [const { None }; FEM3D_RECOVERED_BACKING_CAPACITY] }
    }
}

thread_local! {
    static FEM3D_BACKING_RECOVERY: RefCell<Fem3dBackingRecovery> = const { RefCell::new(Fem3dBackingRecovery::new()) };
}

fn recover_fem3d_backing(owner: Fem3dRecoveredBacking) -> bool {
    FEM3D_BACKING_RECOVERY.with(|recovery| {
        let Ok(mut recovery) = recovery.try_borrow_mut() else { return false };
        let Some(slot) = recovery.owners.iter_mut().find(|slot| slot.is_none()) else { return false };
        *slot = Some(owner);
        true
    })
}

fn close_recovered_fem3d_backing(maximum_bytes: usize) -> Option<(usize, usize)> {
    FEM3D_BACKING_RECOVERY.with(|recovery| {
        let mut recovery = recovery.borrow_mut();
        let owner = recovery.owners.iter_mut().find(|slot| slot.is_some())?;
        let bytes = match owner.as_ref()? {
            Fem3dRecoveredBacking::SolverScalar(_) => FEM3D_SOLVER_SCALAR_PAGE_BYTES,
            Fem3dRecoveredBacking::SolverInitialized(_) => FEM3D_SOLVER_INITIALIZED_PAGE_BYTES,
            Fem3dRecoveredBacking::RegionOrder(_) => FEM3D_REGION_ORDER_BYTES,
            Fem3dRecoveredBacking::ElementOrder(_) => FEM3D_ELEMENT_ORDER_BYTES,
        };
        if maximum_bytes < bytes {
            return Some((0, 0));
        }
        let owner = owner.take()?;
        match owner {
            Fem3dRecoveredBacking::SolverScalar(owner) => drop(owner),
            Fem3dRecoveredBacking::SolverInitialized(owner) => drop(owner),
            Fem3dRecoveredBacking::RegionOrder(owner) => drop(owner),
            Fem3dRecoveredBacking::ElementOrder(owner) => drop(owner),
        }
        Some((1, bytes))
    })
}

impl<const N: usize> FixedOrder<N> {
    fn new(backing: &mut Fem3dBackingCredit) -> Result<Self, ()> {
        let bytes = std::mem::size_of::<Option<usize>>() * N;
        if !backing.claim(bytes) {
            return Err(());
        }
        Ok(Self { slots: Box::new([None; N]), len: 0 })
    }

    fn push(&mut self, value: usize) -> Result<(), ()> {
        let Some(slot) = self.slots.get_mut(self.len) else { return Err(()) };
        *slot = Some(value);
        self.len += 1;
        Ok(())
    }

    fn get(&self, index: usize) -> Option<usize> {
        self.slots.get(index).copied().flatten()
    }

    fn swap(&mut self, left: usize, right: usize) {
        self.slots.swap(left, right);
    }
}

#[derive(Clone, Copy)]
struct Fem3dPageCredit {
    item_count: u32,
    byte_count: u32,
    draw_count: u32,
    draw_bytes: u32,
}

impl Fem3dPageCredit {
    fn descriptor(self, freshness: Fem3dVisualFreshness) -> World3dSnapshotDescriptor {
        World3dSnapshotDescriptor {
            revision: freshness.model_revision,
            generation: freshness.renderer_scene_generation,
            page_count: FEM3D_VISUAL_PAGES as u16,
            item_count: self.item_count,
            byte_count: self.byte_count,
            draw_count: 1,
            draw_instance_count: self.draw_count,
            draw_byte_count: self.draw_bytes,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Fem3dPageVisualLease {
    freshness: Fem3dVisualFreshness,
    snapshot: World3dSnapshotLease,
    close_started: bool,
}

impl Fem3dPageVisualLease {
    pub fn snapshot(&self) -> World3dSnapshotLease {
        self.snapshot
    }

    fn matches(&self, freshness: Fem3dVisualFreshness) -> bool {
        self.freshness == freshness
    }

    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if maximum_bytes < WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY {
            return (false, 0, 0);
        }
        if !self.close_started {
            if world3d_snapshot_begin_close(self.snapshot).is_err() {
                return (false, 0, 0);
            }
            self.close_started = true;
            return (false, 1, 0);
        }
        match world3d_snapshot_close_step(self.snapshot) {
            Ok(true) => (true, 1, 0),
            Ok(false) => (false, 1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY),
            Err(_) if world3d_snapshot_terminal_is_empty(self.snapshot) => (true, 1, 0),
            Err(_) => (false, 0, 0),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_started && world3d_snapshot_terminal_is_empty(self.snapshot)
    }
}

impl Drop for Fem3dPageVisualLease {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            let _ = world3d_snapshot_recover_lease(self.snapshot);
            self.close_started = true;
        }
    }
}

pub struct Fem3dPageVisualJob {
    freshness: Fem3dVisualFreshness,
    credit: Fem3dPageCredit,
    stage: Fem3dVisualJobStage,
    token: Option<World3dSnapshotWriteToken>,
    pages: [Option<World3dSnapshotPage>; FEM3D_VISUAL_PAGES],
    region_order: Option<FixedOrder<MAXIMUM_REGIONS>>,
    element_order: Option<FixedOrder<MAXIMUM_ELEMENTS>>,
    reserve_lane: usize,
    scalar_cursor: u8,
    order_input: usize,
    order_slot: usize,
    cursor: usize,
    lookup_cursor: usize,
    point_cursor: usize,
    item_phase: u8,
    load_case_cursor: usize,
    load_cursor: usize,
    solid_sum: [f64; 2],
    endpoint_a: [f64; 3],
    endpoint_b: [f64; 3],
    seal_cursor: usize,
    admit_cursor: usize,
    validated: bool,
    complete: Option<Fem3dPageVisualLease>,
    close_lane: u8,
    abort_started: bool,
}

impl Fem3dPageVisualJob {
    fn backing_usage(&self) -> (usize, usize) {
        let region = usize::from(self.region_order.is_some());
        let element = usize::from(self.element_order.is_some());
        (region + element, region * FEM3D_REGION_ORDER_BYTES + element * FEM3D_ELEMENT_ORDER_BYTES)
    }

    fn page_kind(index: usize) -> World3dSnapshotPageKind {
        match index {
            0 => World3dSnapshotPageKind::Mesh,
            1..=9 => World3dSnapshotPageKind::Instance,
            _ => World3dSnapshotPageKind::Status,
        }
    }

    fn new(freshness: Fem3dVisualFreshness, credit: Fem3dPageCredit) -> Self {
        Self {
            freshness,
            credit,
            stage: Fem3dVisualJobStage::ReserveSnapshot,
            token: None,
            pages: std::array::from_fn(|_| None),
            region_order: None,
            element_order: None,
            reserve_lane: 0,
            scalar_cursor: 0,
            order_input: 0,
            order_slot: 0,
            cursor: 0,
            lookup_cursor: 0,
            point_cursor: 0,
            item_phase: 0,
            load_case_cursor: 0,
            load_cursor: 0,
            solid_sum: [0.0; 2],
            endpoint_a: [0.0; 3],
            endpoint_b: [0.0; 3],
            seal_cursor: 0,
            admit_cursor: 0,
            validated: false,
            complete: None,
            close_lane: 0,
            abort_started: false,
        }
    }

    pub fn stage(&self) -> Fem3dVisualJobStage {
        self.stage
    }

    fn advance(&mut self, stage: Fem3dVisualJobStage) {
        self.stage = stage;
        self.cursor = 0;
        self.lookup_cursor = 0;
        self.point_cursor = 0;
        self.item_phase = 0;
    }

    fn order_region_one(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Vec<u8>> {
        let Some(order) = self.region_order.as_mut() else { return Err(b"fem3d.visual-region-order-owner".to_vec()) };
        if self.order_slot != 0 {
            let left = order.get(self.order_slot - 1).ok_or_else(|| b"fem3d.visual-region-order-left".to_vec())?;
            let right = order.get(self.order_slot).ok_or_else(|| b"fem3d.visual-region-order-right".to_vec())?;
            if doc.solids[left].id > doc.solids[right].id {
                order.swap(self.order_slot - 1, self.order_slot);
                self.order_slot -= 1;
            } else {
                self.order_slot = 0;
                self.order_input += 1;
            }
            return Ok(false);
        }
        if self.order_input == doc.solids.len() {
            return Ok(true);
        }
        order.push(self.order_input).map_err(|()| b"fem3d.visual-region-order-capacity".to_vec())?;
        self.order_slot = order.len - 1;
        if self.order_slot == 0 {
            self.order_input += 1;
        }
        Ok(false)
    }

    fn order_element_one(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Vec<u8>> {
        let Some(order) = self.element_order.as_mut() else { return Err(b"fem3d.visual-element-order-owner".to_vec()) };
        if self.order_slot != 0 {
            let left = order.get(self.order_slot - 1).ok_or_else(|| b"fem3d.visual-element-order-left".to_vec())?;
            let right = order.get(self.order_slot).ok_or_else(|| b"fem3d.visual-element-order-right".to_vec())?;
            if element_id(&doc.elements[left]) > element_id(&doc.elements[right]) {
                order.swap(self.order_slot - 1, self.order_slot);
                self.order_slot -= 1;
            } else {
                self.order_slot = 0;
                self.order_input += 1;
            }
            return Ok(false);
        }
        if self.order_input == doc.elements.len() {
            return Ok(true);
        }
        order.push(self.order_input).map_err(|()| b"fem3d.visual-element-order-capacity".to_vec())?;
        self.order_slot = order.len - 1;
        if self.order_slot == 0 {
            self.order_input += 1;
        }
        Ok(false)
    }

    fn element_endpoints(element: &FemElement) -> (&str, &str) {
        match element {
            FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start, end),
        }
    }

    fn push_item(&mut self, page: usize, strings: [Option<&str>; 4], numbers: [f64; 16], number_len: u8, indexes: [u32; 8], index_len: u8, flags: u16) -> Result<(), Vec<u8>> {
        let owner = self.pages.get_mut(page).and_then(Option::as_mut).ok_or_else(|| b"fem3d.visual-page-owner".to_vec())?;
        let byte_count = strings.iter().try_fold(0usize, |total, value| total.checked_add(value.map_or(0, |text| text.len()))).ok_or_else(|| b"fem3d.visual-page-byte-overflow".to_vec())?;
        if owner.item_count() == WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY || owner.byte_count().checked_add(byte_count).is_none_or(|count| count > WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY) {
            return Err(b"fem3d.visual-page-credit".to_vec());
        }
        let first = match strings[0] {
            Some(value) => Some(owner.push_string(value).map_err(|_| b"fem3d.visual-page-string".to_vec())?),
            None => None,
        };
        let second = match strings[1] {
            Some(value) => Some(owner.push_string(value).map_err(|_| b"fem3d.visual-page-string".to_vec())?),
            None => None,
        };
        let third = match strings[2] {
            Some(value) => Some(owner.push_string(value).map_err(|_| b"fem3d.visual-page-string".to_vec())?),
            None => None,
        };
        let fourth = match strings[3] {
            Some(value) => Some(owner.push_string(value).map_err(|_| b"fem3d.visual-page-string".to_vec())?),
            None => None,
        };
        owner.push_item(World3dSnapshotItem { strings: [first, second, third, fourth], numbers, indexes, number_len, index_len, flags }).map_err(|_| b"fem3d.visual-page-item".to_vec())
    }

    fn instance_numbers(position: [f64; 3], scale: [f64; 3]) -> [f64; 16] {
        let mut numbers = [0.0; 16];
        numbers[..3].copy_from_slice(&position);
        numbers[3..7].copy_from_slice(&[0.0, 0.0, 0.0, 1.0]);
        numbers[7..10].copy_from_slice(&scale);
        numbers[10..14].copy_from_slice(&[0.35, 0.75, 0.95, 1.0]);
        numbers
    }

    fn field_numbers(field: Fem3dSolverScalar, position: [f64; 3], vector: [f64; 3], scalar: f64) -> [f64; 16] {
        let mut numbers = [0.0; 16];
        numbers[..3].copy_from_slice(&vector);
        numbers[3] = scalar;
        numbers[4] = field.residual[0];
        numbers[5] = field.residual[1];
        numbers[6] = field.residual[2];
        numbers[7] = field.reaction[0];
        numbers[8] = field.reaction[1];
        numbers[9] = field.reaction[2];
        numbers[10] = field.eigen_estimate;
        numbers[11..14].copy_from_slice(&position);
        numbers
    }

    fn progress_numbers(solver: &Fem3dSolverView) -> [f64; 16] {
        let mut numbers = [0.0; 16];
        numbers[0] = solver.residual_norm;
        numbers[1] = solver.tolerance;
        numbers[2] = solver.completed as f64;
        numbers
    }

    fn field_page(stage: Fem3dVisualJobStage, index: usize) -> usize {
        let base = match stage {
            Fem3dVisualJobStage::BuildDisplacementEntry => 10,
            Fem3dVisualJobStage::BuildResidualEntry => 12,
            Fem3dVisualJobStage::BuildReactionEntry => 14,
            Fem3dVisualJobStage::BuildContourEntry => 16,
            _ => 18,
        };
        base + index / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY
    }

    fn step_one(&mut self, doc: &Fem3dSnapshot, solver: &Fem3dSolverView, backing: &mut Fem3dBackingCredit, freshness: Fem3dVisualFreshness) -> Result<bool, Vec<u8>> {
        match self.stage {
            Fem3dVisualJobStage::ReserveSnapshot => {
                if doc.solids.len() > MAXIMUM_REGIONS || doc.nodes.len() > MAXIMUM_NODES || doc.elements.len() > MAXIMUM_ELEMENTS || doc.supports.len() > MAXIMUM_SUPPORTS || doc.nodes.len() > MAXIMUM_FIELDS {
                    return Err(b"fem3d.visual-maximum-plus-one".to_vec());
                }
                if self.reserve_lane == 0 {
                    self.token = Some(world3d_snapshot_begin(self.credit.descriptor(self.freshness)).map_err(|_| b"fem3d.visual-page-preflight".to_vec())?);
                } else if self.reserve_lane == 1 {
                    self.region_order = Some(FixedOrder::new(backing).map_err(|_| b"fem3d.visual-region-order-admission".to_vec())?);
                } else if self.reserve_lane == 2 {
                    self.element_order = Some(FixedOrder::new(backing).map_err(|_| b"fem3d.visual-element-order-admission".to_vec())?);
                } else if let Some(page) = self.reserve_lane.checked_sub(3).filter(|page| *page < FEM3D_VISUAL_PAGES) {
                    self.pages[page] = Some(World3dSnapshotPage::new(Self::page_kind(page)));
                } else {
                    self.stage = Fem3dVisualJobStage::ReadProgressScalar;
                    return Ok(false);
                }
                self.reserve_lane += 1;
            }
            Fem3dVisualJobStage::ReadProgressScalar => {
                if solver.freshness != self.freshness {
                    return Err(b"fem3d.visual-solver-view-stale".to_vec());
                }
                match self.scalar_cursor {
                    0 => {
                        let _ = solver.state;
                    }
                    1 => {
                        let _ = solver.residual_norm;
                    }
                    2 => {
                        let _ = solver.tolerance;
                    }
                    3 => {
                        let _ = solver.completed;
                    }
                    4 => {
                        let _ = solver.total;
                    }
                    _ => {
                        self.order_input = 0;
                        self.order_slot = 0;
                        self.stage = Fem3dVisualJobStage::OrderRegionKey;
                        return Ok(false);
                    }
                }
                self.scalar_cursor += 1;
            }
            Fem3dVisualJobStage::OrderRegionKey => {
                if self.order_region_one(doc)? {
                    self.advance(Fem3dVisualJobStage::BuildRegion);
                }
            }
            Fem3dVisualJobStage::BuildRegion => {
                let Some(index) = self.region_order.as_ref().and_then(|order| order.get(self.cursor)) else {
                    self.order_input = 0;
                    self.order_slot = 0;
                    self.advance(Fem3dVisualJobStage::OrderElementKey);
                    return Ok(false);
                };
                let solid = &doc.solids[index];
                if let Some(point) = solid.outline.get(self.point_cursor) {
                    self.solid_sum[0] += point[0];
                    self.solid_sum[1] += point[1];
                    self.point_cursor += 1;
                    return Ok(false);
                }
                let divisor = self.point_cursor.max(1) as f64;
                let kind = if solid.layers == 1 { 1 } else { 2 };
                self.push_item(
                    1,
                    [Some(&solid.id), Some(if kind == 1 { "tetrahedron" } else { "hexahedron" }), None, None],
                    Self::instance_numbers([self.solid_sum[0] / divisor, self.solid_sum[1] / divisor, solid.base_z + solid.height * 0.5], [1.0, 1.0, solid.height]),
                    14,
                    [kind, 0, 0, 0, 0, 0, 0, 0],
                    1,
                    1,
                )?;
                self.cursor += 1;
                self.point_cursor = 0;
                self.solid_sum = [0.0; 2];
            }
            Fem3dVisualJobStage::OrderElementKey => {
                if self.order_element_one(doc)? {
                    self.advance(Fem3dVisualJobStage::BuildMeshElement);
                }
            }
            Fem3dVisualJobStage::BuildMeshElement => {
                if self.item_phase == 0 {
                    if let Some(node) = doc.nodes.get(self.cursor) {
                        self.push_item(2 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(&node.id), None, None, None], Self::instance_numbers([node.x, node.y, node.z], [0.05; 3]), 14, [0; 8], 0, 2)?;
                        self.cursor += 1;
                    } else {
                        self.cursor = 0;
                        self.item_phase = 1;
                    }
                    return Ok(false);
                }
                let Some(index) = self.element_order.as_ref().and_then(|order| order.get(self.cursor)) else {
                    self.advance(Fem3dVisualJobStage::BuildAssemblyMark);
                    return Ok(false);
                };
                let element = &doc.elements[index];
                let (start, end) = Self::element_endpoints(element);
                if self.item_phase == 1 {
                    let Some(node) = doc.nodes.get(self.lookup_cursor) else { return Err(b"fem3d.visual-element-start".to_vec()) };
                    if node.id == start {
                        self.endpoint_a = [node.x, node.y, node.z];
                        self.lookup_cursor = 0;
                        self.item_phase = 2;
                    } else {
                        self.lookup_cursor += 1;
                    }
                    return Ok(false);
                }
                if self.item_phase == 2 {
                    let Some(node) = doc.nodes.get(self.lookup_cursor) else { return Err(b"fem3d.visual-element-end".to_vec()) };
                    if node.id == end {
                        self.endpoint_b = [node.x, node.y, node.z];
                        self.lookup_cursor = 0;
                        self.item_phase = 3;
                    } else {
                        self.lookup_cursor += 1;
                    }
                    return Ok(false);
                }
                let midpoint = [(self.endpoint_a[0] + self.endpoint_b[0]) * 0.5, (self.endpoint_a[1] + self.endpoint_b[1]) * 0.5, (self.endpoint_a[2] + self.endpoint_b[2]) * 0.5];
                let dx = self.endpoint_b[0] - self.endpoint_a[0];
                let dy = self.endpoint_b[1] - self.endpoint_a[1];
                let dz = self.endpoint_b[2] - self.endpoint_a[2];
                let length = (dx * dx + dy * dy + dz * dz).sqrt();
                self.push_item(4 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(element_id(element)), None, None, None], Self::instance_numbers(midpoint, [0.05, 0.05, length]), 14, [0; 8], 0, 3)?;
                self.cursor += 1;
                self.item_phase = 1;
            }
            Fem3dVisualJobStage::BuildAssemblyMark => {
                let Some(index) = self.element_order.as_ref().and_then(|order| order.get(self.cursor)) else {
                    self.advance(Fem3dVisualJobStage::BuildLoadGlyph);
                    return Ok(false);
                };
                self.push_item(6 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(element_id(&doc.elements[index])), None, None, None], Self::instance_numbers([0.0; 3], [0.03; 3]), 14, [0; 8], 0, 4)?;
                self.cursor += 1;
            }
            Fem3dVisualJobStage::BuildLoadGlyph => {
                let Some(case) = doc.load_cases.get(self.load_case_cursor) else {
                    self.advance(Fem3dVisualJobStage::BuildSupportGlyph);
                    return Ok(false);
                };
                let Some(load) = case.loads.get(self.load_cursor) else {
                    self.load_case_cursor += 1;
                    self.load_cursor = 0;
                    return Ok(false);
                };
                self.push_item(8, [Some(load_id(load)), None, None, None], Self::instance_numbers([0.0, 0.0, 1.0], [0.03, 0.03, 0.3]), 14, [0; 8], 0, 5)?;
                self.load_cursor += 1;
            }
            Fem3dVisualJobStage::BuildSupportGlyph => {
                let Some(support) = doc.supports.get(self.cursor) else {
                    self.advance(Fem3dVisualJobStage::BuildDisplacementEntry);
                    return Ok(false);
                };
                self.push_item(9, [Some(&support.id), None, None, None], Self::instance_numbers([0.0; 3], [0.1; 3]), 14, [0; 8], 0, 6)?;
                self.cursor += 1;
            }
            Fem3dVisualJobStage::BuildDisplacementEntry | Fem3dVisualJobStage::BuildResidualEntry | Fem3dVisualJobStage::BuildReactionEntry | Fem3dVisualJobStage::BuildContourEntry | Fem3dVisualJobStage::BuildModeEntry => {
                let Some(node) = doc.nodes.get(self.cursor) else {
                    let next = match self.stage {
                        Fem3dVisualJobStage::BuildDisplacementEntry => Fem3dVisualJobStage::BuildResidualEntry,
                        Fem3dVisualJobStage::BuildResidualEntry => Fem3dVisualJobStage::BuildReactionEntry,
                        Fem3dVisualJobStage::BuildReactionEntry => Fem3dVisualJobStage::BuildContourEntry,
                        Fem3dVisualJobStage::BuildContourEntry => Fem3dVisualJobStage::BuildModeEntry,
                        _ => Fem3dVisualJobStage::BuildLabelEntry,
                    };
                    self.advance(next);
                    return Ok(false);
                };
                let field = solver.scalar(self.cursor).ok_or_else(|| b"fem3d.visual-solver-field".to_vec())?;
                let (vector, scalar, flag) = match self.stage {
                    Fem3dVisualJobStage::BuildDisplacementEntry => (field.displacement, field.contour, 10),
                    Fem3dVisualJobStage::BuildResidualEntry => (field.residual, solver.residual_norm, 11),
                    Fem3dVisualJobStage::BuildReactionEntry => (field.reaction, field.contour, 12),
                    Fem3dVisualJobStage::BuildContourEntry => ([0.0; 3], field.contour, 13),
                    _ => (field.mode_shape, field.eigen_estimate, 14),
                };
                self.push_item(Self::field_page(self.stage, self.cursor), [Some(&node.id), None, None, None], Self::field_numbers(field, [node.x, node.y, node.z], vector, scalar), 14, [0; 8], 0, flag)?;
                self.cursor += 1;
            }
            Fem3dVisualJobStage::BuildLabelEntry => {
                if self.cursor == 0 {
                    self.push_item(20, [Some("en"), Some(FEM3D_VISUAL_LABEL_EN), None, None], Self::progress_numbers(solver), 3, [solver.total as u32, solver.state as u32, 0, 0, 0, 0, 0, 0], 2, 20)?;
                    self.cursor = 1;
                } else if self.cursor == 1 {
                    self.push_item(20, [Some("de"), Some(FEM3D_VISUAL_LABEL_DE), None, None], Self::progress_numbers(solver), 3, [solver.total as u32, solver.state as u32, 0, 0, 0, 0, 0, 0], 2, 20)?;
                    self.cursor = 2;
                } else if self.cursor == 2 {
                    self.push_item(0, [Some("box"), None, None, None], [0.0; 16], 0, [self.credit.draw_count, self.credit.draw_bytes, 0, 0, 0, 0, 0, 0], 2, 30)?;
                    self.cursor = 3;
                } else {
                    self.advance(Fem3dVisualJobStage::SealPages);
                }
            }
            Fem3dVisualJobStage::SealPages => {
                if self.seal_cursor < FEM3D_VISUAL_PAGES {
                    self.pages[self.seal_cursor].as_mut().ok_or_else(|| b"fem3d.visual-seal-page".to_vec())?.seal().map_err(|_| b"fem3d.visual-seal-page".to_vec())?;
                    self.seal_cursor += 1;
                } else if self.admit_cursor < FEM3D_VISUAL_PAGES {
                    let page = self.pages[self.admit_cursor].take().ok_or_else(|| b"fem3d.visual-admit-page".to_vec())?;
                    let token = self.token.ok_or_else(|| b"fem3d.visual-page-token".to_vec())?;
                    if let Err(rejected) = world3d_snapshot_admit_page(token, page) {
                        self.pages[self.admit_cursor] = Some(rejected.page);
                        return Err(b"fem3d.visual-page-admission".to_vec());
                    }
                    self.admit_cursor += 1;
                } else {
                    self.stage = Fem3dVisualJobStage::ValidateFreshness;
                }
            }
            Fem3dVisualJobStage::ValidateFreshness => {
                if freshness != self.freshness || solver.freshness != self.freshness {
                    return Err(b"fem3d.visual-stale-before-publication".to_vec());
                }
                self.validated = true;
                self.stage = Fem3dVisualJobStage::PublishLease;
            }
            Fem3dVisualJobStage::PublishLease => {
                if !self.validated {
                    return Err(b"fem3d.visual-publication-without-freshness".to_vec());
                }
                let token = self.token.take().ok_or_else(|| b"fem3d.visual-page-token".to_vec())?;
                let snapshot = world3d_snapshot_seal(token).map_err(|_| b"fem3d.visual-page-seal".to_vec())?;
                self.complete = Some(Fem3dPageVisualLease { freshness, snapshot, close_started: false });
                self.stage = Fem3dVisualJobStage::RetireDisplacedLease;
            }
            Fem3dVisualJobStage::RetireDisplacedLease => {
                if self.region_order.take().is_some() {
                    if !backing.release(1, FEM3D_REGION_ORDER_BYTES) {
                        return Err(b"fem3d.visual-region-order-credit".to_vec());
                    }
                } else if self.element_order.take().is_some() {
                    if !backing.release(1, FEM3D_ELEMENT_ORDER_BYTES) {
                        return Err(b"fem3d.visual-element-order-credit".to_vec());
                    }
                } else {
                    self.stage = Fem3dVisualJobStage::Complete;
                }
            }
            Fem3dVisualJobStage::Complete => return Ok(true),
        }
        Ok(self.stage == Fem3dVisualJobStage::Complete)
    }

    fn take_complete(&mut self) -> Option<Fem3dPageVisualLease> {
        if self.stage == Fem3dVisualJobStage::Complete {
            self.complete.take()
        } else {
            None
        }
    }

    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(lease) = self.complete.as_mut() {
            let result = lease.close_step(maximum_bytes);
            if result.0 {
                self.complete = None;
                return (false, 1, 0);
            }
            return result;
        }
        if let Some(page) = self.pages.iter_mut().find(|page| page.is_some()) {
            if maximum_bytes < WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY {
                return (false, 0, 0);
            }
            *page = None;
            return (false, 1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
        }
        if let Some(token) = self.token {
            if !self.abort_started {
                if world3d_snapshot_abort_write(token).is_err() {
                    return (false, 0, 0);
                }
                self.abort_started = true;
                return (false, 1, 0);
            }
            match world3d_snapshot_abort_write_step(token) {
                Ok(true) => {
                    self.token = None;
                    return (false, 1, 0);
                }
                Ok(false) => return (false, 1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY),
                Err(_) if world3d_snapshot_write_terminal_is_empty(token) => {
                    self.token = None;
                    return (false, 1, 0);
                }
                Err(_) => return (false, 0, 0),
            }
        }
        if self.region_order.take().is_some() {
            return (false, 1, std::mem::size_of::<Option<usize>>() * MAXIMUM_REGIONS);
        }
        if self.element_order.take().is_some() {
            return (false, 1, std::mem::size_of::<Option<usize>>() * MAXIMUM_ELEMENTS);
        }
        (true, 0, 0)
    }

    fn terminal_is_empty(&self) -> bool {
        self.complete.is_none() && self.token.is_none() && self.pages.iter().all(Option::is_none) && self.region_order.is_none() && self.element_order.is_none()
    }
}

impl Drop for Fem3dPageVisualJob {
    fn drop(&mut self) {
        if let Some(lease) = self.complete.take() {
            drop(lease);
        }
        for page in self.pages.iter_mut().filter_map(Option::take) {
            if let Err(page) = world3d_snapshot_recover_page(page) {
                std::mem::forget(page);
                panic!("FEM3D visual page recovery capacity");
            }
        }
        if let Some(token) = self.token.take() {
            let _ = world3d_snapshot_recover_write(token);
        }
        if let Some(order) = self.region_order.take() {
            assert!(recover_fem3d_backing(Fem3dRecoveredBacking::RegionOrder(order.slots)), "FEM3D region order recovery capacity");
        }
        if let Some(order) = self.element_order.take() {
            assert!(recover_fem3d_backing(Fem3dRecoveredBacking::ElementOrder(order.slots)), "FEM3D element order recovery capacity");
        }
        assert!(self.terminal_is_empty(), "FEM3D visual candidate recovery handback was not shallow and exact");
    }
}
//#endregion 📦️PageVisualJob

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
    snapshot: Option<store::SnapshotRead<Fem3dSnapshot>>,
    snapshot_return: Option<store::SnapshotReadReturn>,
    cancel: semio_framework_job::CancelToken,
    preview_sequence: u64,
    credit: Fem3dPageCredit,
    backing: Fem3dBackingCredit,
    solver: Option<Fem3dSolverView>,
    numerical: Option<Fem3dNumericalChild>,
    numerical_done: bool,
    candidate: Option<Fem3dPageVisualJob>,
    current: Option<Fem3dPageVisualLease>,
    displaced: Option<Fem3dPageVisualLease>,
    close_lane: u8,
    done: bool,
    recovery: Rc<MountedRecoverySlot>,
}

impl MountedState {
    fn new(identity: Identity, snapshot: store::SnapshotRead<Fem3dSnapshot>, current: Option<Fem3dPageVisualLease>, credit: Fem3dPageCredit, recovery: Rc<MountedRecoverySlot>) -> Self {
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
            numerical: Some(Fem3dNumericalChild::new()),
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

    fn step(&mut self, budget: JobBudget) -> JobStep {
        if self.cancel.is_cancelled_now() {
            return self.fail(b"fem3d.visual-cancelled".to_vec());
        }
        if budget.fuel == 0 || budget.deadline_ms == 0 {
            return JobStep::Running(None);
        }
        let Some(now) = semio_framework_job::default_now_us() else { return JobStep::Running(None) };
        let deadline = now.saturating_add(u64::from(budget.deadline_ms).min(8));
        let mut cx = StepContext::new(self.identity.operation, self.identity.generation, StepBudget::new(budget.fuel, deadline), self.cancel.clone(), semio_framework_job::default_now_us, &mut self.preview_sequence);
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
            let step = self.numerical.as_mut().map(|numerical| numerical.step(snapshot, solver, &mut self.backing, freshness, operation, &mut cx));
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
            self.candidate = Some(Fem3dPageVisualJob::new(self.identity.freshness(0), self.credit));
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
                let Some(lease) = self.candidate.as_mut().and_then(Fem3dPageVisualJob::take_complete) else { return self.fail(b"fem3d.visual-complete-owner".to_vec()) };
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

    fn close_step(&mut self, maximum_bytes: usize) -> PluginCloseStep {
        match self.close_lane {
            0 => {
                if let Some(candidate) = self.candidate.as_mut() {
                    let before = candidate.backing_usage();
                    let (terminal, items, bytes) = candidate.close_step(maximum_bytes);
                    let after = candidate.backing_usage();
                    if !self.backing.release(before.0 - after.0, before.1 - after.1) {
                        return PluginCloseStep::Blocked { reason: "FEM3D visual candidate process credit mismatch" };
                    }
                    if !terminal {
                        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
                    }
                    if !candidate.terminal_is_empty() {
                        return PluginCloseStep::Blocked { reason: "FEM3D visual candidate false terminal" };
                    }
                    self.candidate = None;
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                self.close_lane = 1;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            1 => {
                if let Some(displaced) = self.displaced.as_mut() {
                    let (terminal, items, bytes) = displaced.close_step(maximum_bytes);
                    if !terminal {
                        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
                    }
                    if !displaced.terminal_is_empty() {
                        return PluginCloseStep::Blocked { reason: "FEM3D displaced lease false terminal" };
                    }
                    self.displaced = None;
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                self.close_lane = 2;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            2 => {
                if let Some(current) = self.current.as_mut() {
                    let (terminal, items, bytes) = current.close_step(maximum_bytes);
                    if !terminal {
                        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
                    }
                    if !current.terminal_is_empty() {
                        return PluginCloseStep::Blocked { reason: "FEM3D current lease false terminal" };
                    }
                    self.current = None;
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                self.close_lane = 3;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            3 => {
                if let Some(numerical) = self.numerical.as_mut() {
                    let (terminal, items, bytes) = numerical.close_step(maximum_bytes);
                    if !terminal {
                        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
                    }
                    self.numerical = None;
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                self.close_lane = 4;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            4 => {
                if let Some(solver) = self.solver.as_mut() {
                    let (terminal, items, bytes) = solver.close_step(maximum_bytes);
                    if items != 0 && !self.backing.release(items, bytes) {
                        return PluginCloseStep::Blocked { reason: "FEM3D solver process credit mismatch" };
                    }
                    if !terminal {
                        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
                    }
                    self.solver = None;
                    return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
                }
                self.close_lane = 5;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            5 => {
                if let Some(snapshot) = self.snapshot.take() {
                    let Some(witness) = snapshot.return_to_registry_witness() else {
                        return PluginCloseStep::Blocked { reason: "FEM3D snapshot already returned" };
                    };
                    self.snapshot_return = Some(witness);
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<store::SnapshotRead<Fem3dSnapshot>>() };
                }
                if self.snapshot_return.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
                    return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                if self.snapshot_return.take().is_some() {
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<store::SnapshotReadReturn>() };
                }
                self.close_lane = 6;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            6 => {
                if !self.backing.terminal_is_empty() {
                    return PluginCloseStep::Blocked { reason: "FEM3D process backing credit remains live" };
                }
                self.close_lane = 7;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            _ => PluginCloseStep::Complete,
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
}

impl BoundedJob for MountedJob {
    fn step(&mut self, budget: JobBudget) -> JobStep {
        let Ok(mut shell) = self.shell.try_borrow_mut() else { return JobStep::Running(None) };
        let Some(state) = shell.as_mut().filter(|state| state.identity == self.identity) else { return JobStep::Failed(b"fem3d.visual-stale-shell".to_vec()) };
        let step = state.step(budget);
        if matches!(&step, JobStep::Done(_)) {
            self.completed = true;
        }
        step
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
        true
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

fn factory(job: u64, input: &[u8]) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    let (shell, identity) = decode_input(job, input).ok_or_else(|| b"fem3d.visual-input".to_vec())?;
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let owner = registry.shells.get(shell as usize).ok_or_else(|| b"fem3d.visual-shell".to_vec())?.clone();
        let recovery = registry.recoveries.get(shell as usize).ok_or_else(|| b"fem3d.visual-recovery".to_vec())?.clone();
        if !owner.try_borrow().is_ok_and(|state| state.as_ref().is_some_and(|state| state.identity == identity)) {
            return Err(b"fem3d.visual-stale-factory".to_vec());
        }
        if recovery.reserved.get() != Some(identity) {
            return Err(b"fem3d.visual-stale-recovery".to_vec());
        }
        Ok(Box::new(MountedJob { shell: owner, recovery, identity, completed: false }) as Box<dyn BoundedJob>)
    })
}

pub fn initialize() {
    MOUNTED.with(|registry| {
        let _ = registry.borrow().free_len;
    });
    semio_framework_plugin::reactor::jobs::register_bounded_job_kind(FEM3D_MOUNTED_VISUAL_JOB_KIND, factory as BoundedJobFactory);
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
            return false;
        }
        let Some(pending) = registry.pending[slot].as_mut() else { return false };
        match pending.preflight.step_one(snapshot) {
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
    build(owner.as_ref().and_then(|state| state.current.as_ref()))
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

fn recover_abandoned_one(registry: &mut Registry, app_instance_id: u32, maximum_bytes: usize) -> Option<PluginCloseStep> {
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
            return Some(PluginCloseStep::Blocked { reason: "FEM3D retained job handoff identity mismatch" });
        }
        recovery.clear_publication(identity);
        return Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
    }
    let mut state = recovery.take_owner(identity);
    if state.is_none() {
        let Ok(mut shell_owner) = registry.shells[shell].try_borrow_mut() else { return Some(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }) };
        if shell_owner.as_ref().is_some_and(|state| state.identity == identity) {
            let discovered = shell_owner.take();
            drop(shell_owner);
            if let Some(discovered) = discovered {
                if let Err(discovered) = recovery.publish_owner(identity, discovered) {
                    std::mem::forget(discovered);
                    return Some(PluginCloseStep::Blocked { reason: "FEM3D abandoned state recovery slot collision" });
                }
            }
            state = recovery.take_owner(identity);
        }
    }
    let Some(mut state) = state else { return Some(PluginCloseStep::Blocked { reason: "FEM3D abandoned state owner is not discoverable" }) };
    if state.identity != identity {
        std::mem::forget(state);
        return Some(PluginCloseStep::Blocked { reason: "FEM3D abandoned state generation mismatch" });
    }
    state.cancel.cancel_now();
    let step = state.close_step(maximum_bytes);
    if !matches!(&step, PluginCloseStep::Complete) {
        if let Err(state) = recovery.restore_owner(identity, state) {
            std::mem::forget(state);
            return Some(PluginCloseStep::Blocked { reason: "FEM3D abandoned state restore collision" });
        }
        return Some(step);
    }
    if !state.terminal_is_empty() {
        if let Err(state) = recovery.restore_owner(identity, state) {
            std::mem::forget(state);
        }
        return Some(PluginCloseStep::Blocked { reason: "FEM3D abandoned state false terminal" });
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
        return Some(PluginCloseStep::Blocked { reason: "FEM3D abandoned recovery authority false terminal" });
    }
    registry.release_credit(shell as u16);
    registry.release(shell as u16);
    Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
}

fn retire_one(app_instance_id: u32, maximum_bytes: usize) -> PluginCloseStep {
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let shell = registry.maintenance_cursor;
        registry.maintenance_cursor = (registry.maintenance_cursor + 1) % SHELL_CAPACITY;
        if !registry.retiring[shell] || registry.retiring_app[shell] != app_instance_id {
            return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let step = {
            let mut owner = registry.shells[shell].borrow_mut();
            let Some(state) = owner.as_mut() else {
                if registry.recoveries[shell].publication.get().is_some_and(|current| current.1 == MountedRecoveryPublication::Recover) {
                    return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                return PluginCloseStep::Blocked { reason: "FEM3D retiring shell empty" };
            };
            state.close_step(maximum_bytes)
        };
        if matches!(step, PluginCloseStep::Complete) {
            let terminal_identity = registry.shells[shell].borrow().as_ref().filter(|state| state.terminal_is_empty()).map(|state| state.identity);
            let Some(identity) = terminal_identity else {
                return PluginCloseStep::Blocked { reason: "FEM3D retiring state false terminal" };
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
            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        step
    })
}

pub fn maintenance_step(app_instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
    if maximum_items == 0 {
        return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
    }
    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, maximum_bytes)) {
        return step;
    }
    if let Some((items, bytes)) = world3d_snapshot_recovery_close_step(maximum_bytes) {
        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
    }
    if let Some((items, bytes)) = close_recovered_fem3d_backing(maximum_bytes) {
        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
    }
    retire_one(app_instance_id, maximum_bytes)
}

pub fn close_step(app_instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
    if maximum_items == 0 {
        return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
    }
    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, maximum_bytes)) {
        return step;
    }
    if let Some((items, bytes)) = world3d_snapshot_recovery_close_step(maximum_bytes) {
        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
    }
    if let Some((items, bytes)) = close_recovered_fem3d_backing(maximum_bytes) {
        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
    }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let slot = app_instance_id as usize % ACTIVE_CAPACITY;
        registry.pending[slot] = None;
        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
            if !registry.retain_retiring(app_instance_id, current.shell) {
                return PluginCloseStep::Blocked { reason: "FEM3D close retirement capacity" };
            }
            if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
                if let Some(state) = owner.as_ref() {
                    state.cancel.cancel_now();
                }
            }
            registry.current[slot] = None;
            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        drop(registry);
        retire_one(app_instance_id, maximum_bytes)
    })
}

pub fn terminal_is_empty(app_instance_id: u32) -> bool {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let slot = app_instance_id as usize % ACTIVE_CAPACITY;
        registry.pending[slot].is_none()
            && registry.current[slot].is_none_or(|current| current.app_instance_id != app_instance_id)
            && (registry.retiring_owner[slot] != app_instance_id || registry.retiring_count[slot] == 0)
            && registry.recoveries.iter().all(|recovery| !recovery.contains(app_instance_id))
    })
}
//#endregion 💼️MountedSession

//#region 🧪️Laws
#[cfg(test)]
mod tests {
    use super::*;

    fn freshness(generation: u64) -> Fem3dVisualFreshness {
        Fem3dVisualFreshness { app_instance_id: 7, model_revision: 11, document_generation: generation, operation: 13, numerical_preview_sequence: 17, surface_generation: generation, renderer_scene_generation: generation }
    }

    fn credit(doc: &Fem3dSnapshot) -> Fem3dPageCredit {
        let mut preflight = SnapshotPreflight::new();
        let mut turns = 0;
        while !preflight.step_one(doc).expect("bounded preflight") && turns < 1_024 {
            turns += 1;
        }
        preflight.credit().expect("exact credit")
    }

    fn solver(doc: &Fem3dSnapshot, scalar: Fem3dSolverScalar) -> Fem3dSolverView {
        let mut solver = Fem3dSolverView::new(freshness(19), doc.nodes.len());
        let mut backing = Fem3dBackingCredit::new();
        for page in 0..FEM3D_SOLVER_PAGE_COUNT {
            assert!(solver.admit_page(page, false, &mut backing));
            assert!(solver.admit_page(page, true, &mut backing));
        }
        for index in 0..doc.nodes.len() {
            solver.publish_scalar(freshness(19), index, scalar).expect("generation-qualified scalar");
        }
        assert!(solver.publish_progress(freshness(19), Fem3dVisualState::ValidatedFinal, 0.125, 1e-8, doc.nodes.len(), doc.nodes.len()));
        solver
    }

    fn build(doc: &Fem3dSnapshot, solver: &Fem3dSolverView) -> Fem3dPageVisualLease {
        let mut job = Fem3dPageVisualJob::new(freshness(19), credit(doc));
        let mut backing = Fem3dBackingCredit::new();
        let mut turns = 0;
        while !job.step_one(doc, solver, &mut backing, freshness(19)).expect("one production opportunity") && turns < 4_096 {
            turns += 1;
        }
        assert!(turns < 4_096);
        job.take_complete().expect("sealed page lease")
    }

    fn close(lease: &mut Fem3dPageVisualLease) -> usize {
        let mut released_pages = 0;
        let mut turns = 0;
        loop {
            let (terminal, _, bytes) = lease.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
            released_pages += usize::from(bytes == WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
            turns += 1;
            if terminal {
                break;
            }
            assert!(turns < 128);
        }
        released_pages
    }

    #[test]
    fn fem3d_visual_maximum_plus_one_rejects_before_owner_transfer() {
        let mut doc = Fem3dSnapshot::default();
        doc.nodes.resize_with(MAXIMUM_NODES + 1, || crate::artifacts::fem3d::FemNode { id: "n".into(), x: 0.0, y: 0.0, z: 0.0 });
        let producer = doc.nodes.as_ptr();
        let credit = Fem3dPageCredit { item_count: 3, byte_count: 7, draw_count: 0, draw_bytes: 3 };
        let mut job = Fem3dPageVisualJob::new(freshness(19), credit);
        let solver = Fem3dSolverView::new(freshness(19), 0);
        let mut backing = Fem3dBackingCredit::new();
        assert!(job.step_one(&doc, &solver, &mut backing, freshness(19)).is_err());
        assert_eq!(producer, doc.nodes.as_ptr());
        assert_eq!(job.stage(), Fem3dVisualJobStage::ReserveSnapshot);
    }

    #[test]
    fn fem3d_snapshot_preflight_page_maximum_plus_one_returns_exact_producer() {
        let mut doc = Fem3dSnapshot::default();
        doc.nodes.push(crate::artifacts::fem3d::FemNode { id: "n".repeat(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY + 1), x: 0.0, y: 0.0, z: 0.0 });
        let producer = doc.nodes.as_ptr();
        let mut preflight = SnapshotPreflight::new();
        let before = preflight;
        assert_eq!(preflight.step_one(&doc), Err(()));
        assert_eq!(producer, doc.nodes.as_ptr());
        assert_eq!(preflight.item_count, before.item_count);
        assert_eq!(preflight.byte_count, before.byte_count);
    }

    #[test]
    fn fem3d_solver_nonzero_generation_corresponds_to_every_published_field_page() {
        let mut doc = Fem3dSnapshot::default();
        doc.nodes.push(crate::artifacts::fem3d::FemNode { id: "n1".into(), x: 1.0, y: 2.0, z: 3.0 });
        let scalar = Fem3dSolverScalar { displacement: [1.0, 2.0, 3.0], residual: [4.0, 5.0, 6.0], reaction: [7.0, 8.0, 9.0], contour: 13.0, mode_shape: [10.0, 11.0, 12.0], eigen_estimate: 17.0 };
        let solver = solver(&doc, scalar);
        assert_eq!(solver.scalar(0), Some(scalar));
        let mut lease = build(&doc, &solver);
        let displacement = world3d_snapshot_with_page(lease.snapshot(), 10, |page| page.item(0).expect("displacement").numbers).expect("displacement page");
        let residual = world3d_snapshot_with_page(lease.snapshot(), 12, |page| page.item(0).expect("residual").numbers).expect("residual page");
        let reaction = world3d_snapshot_with_page(lease.snapshot(), 14, |page| page.item(0).expect("reaction").numbers).expect("reaction page");
        let contour = world3d_snapshot_with_page(lease.snapshot(), 16, |page| page.item(0).expect("contour").numbers).expect("contour page");
        let mode = world3d_snapshot_with_page(lease.snapshot(), 18, |page| page.item(0).expect("mode").numbers).expect("mode page");
        assert_eq!(&displacement[..3], &scalar.displacement);
        assert_eq!(&residual[..3], &scalar.residual);
        assert_eq!(&reaction[..3], &scalar.reaction);
        assert_eq!(contour[3], scalar.contour);
        assert_eq!(&mode[..3], &scalar.mode_shape);
        assert_eq!(mode[3], scalar.eigen_estimate);
        assert_eq!(close(&mut lease), FEM3D_VISUAL_PAGES);
    }

    #[test]
    fn fem3d_visual_stale_cancel_fault_deadline_and_interrupted_device_close_preserve_last_valid() {
        let doc = Fem3dSnapshot::default();
        let solver = solver(&doc, Fem3dSolverScalar::default());
        let mut current = build(&doc, &solver);
        let current_snapshot = current.snapshot();
        let mut stale = Fem3dPageVisualJob::new(freshness(19), credit(&doc));
        let mut backing = Fem3dBackingCredit::new();
        let mut turns = 0;
        while stale.stage() != Fem3dVisualJobStage::ValidateFreshness && turns < 1_024 {
            stale.step_one(&doc, &solver, &mut backing, freshness(19)).expect("bounded step");
            turns += 1;
        }
        assert!(stale.step_one(&doc, &solver, &mut backing, freshness(23)).is_err());
        assert!(stale.take_complete().is_none());
        assert_eq!(current.snapshot(), current_snapshot);
        assert_eq!(stale.close_step(0), (false, 0, 0));
        while !stale.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).0 && turns < 2_048 {
            turns += 1;
        }
        assert!(stale.terminal_is_empty());
        assert_eq!(close(&mut current), FEM3D_VISUAL_PAGES);
    }

    #[test]
    fn fem3d_visual_replay_accessibility_fixed_page_close_and_each_step_are_bounded() {
        let doc = Fem3dSnapshot::default();
        let solver = solver(&doc, Fem3dSolverScalar::default());
        let mut first = build(&doc, &solver);
        let mut second = build(&doc, &solver);
        let labels = |lease: &Fem3dPageVisualLease| {
            world3d_snapshot_with_page(lease.snapshot(), 20, |page| {
                let en = page.item(0).and_then(|item| item.strings[1]).and_then(|span| page.string(span)).map(str::to_owned);
                let de = page.item(1).and_then(|item| item.strings[1]).and_then(|span| page.string(span)).map(str::to_owned);
                (en, de)
            })
            .expect("label page")
        };
        let first_labels = labels(&first);
        let second_labels = labels(&second);
        assert_eq!(first_labels, second_labels);
        assert_eq!(first_labels.0.as_deref(), Some(FEM3D_VISUAL_LABEL_EN));
        assert_eq!(first_labels.1.as_deref(), Some(FEM3D_VISUAL_LABEL_DE));
        assert_eq!(close(&mut first), FEM3D_VISUAL_PAGES);
        assert_eq!(close(&mut second), FEM3D_VISUAL_PAGES);

        let mut job = Fem3dPageVisualJob::new(freshness(19), credit(&doc));
        let mut backing = Fem3dBackingCredit::new();
        let started = std::time::Instant::now();
        let _ = job.step_one(&doc, &solver, &mut backing, freshness(19));
        assert!(started.elapsed().as_micros() < 8_000);
        while !job.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).0 {}
        assert!(job.terminal_is_empty());
    }

    #[test]
    fn fem3d_numerical_fixed_owner_maximum_plus_one_refuses_unchanged_and_closes_one_slot() {
        let started = std::time::Instant::now();
        let mut ids = FixedSlots::<String, 1>::new();
        assert_eq!(ids.admit_one(2), Err(()));
        assert_eq!(ids.admitted, 0);
        assert_eq!(ids.admit_one(1), Ok(false));
        assert_eq!(ids.admit_one(1), Ok(true));
        assert_eq!(ids.push("kept".into()), Ok(()));
        let rejected = "returned".to_owned();
        let rejected_pointer = rejected.as_ptr();
        let returned = ids.push(rejected).expect_err("full fixed owner returns producer");
        assert_eq!(returned.as_ptr(), rejected_pointer);
        assert_eq!(ids.len(), 1);
        assert_eq!(ids.pop().as_deref(), Some("kept"));
        assert!(!ids.close_admission_one());
        assert!(ids.close_admission_one());

        let mut model = MountedAnalysisModel::new();
        assert_eq!(model.admit_node_one(MAXIMUM_FIELDS + 1), Err(()));
        assert_eq!(model.nodes_len(), 0);
        let node = Node { id: "returned-node".into(), x: 0.0, y: 0.0, z: 0.0 };
        let node_pointer = node.id.as_ptr();
        let returned = model.push_node(node).expect_err("unadmitted node returns producer");
        assert_eq!(returned.id.as_ptr(), node_pointer);

        let mut domain = MountedPlanarDomain::new();
        assert_eq!(domain.admit_outer_one(MAXIMUM_FIELDS + 1), Err(()));
        assert_eq!(domain.push_outer([1.0, 2.0]), Err([1.0, 2.0]));

        let mut scalars = MountedScalarSlots::new();
        assert_eq!(scalars.admit_one(MAXIMUM_FIELDS * 6 + 1), Err(()));
        assert_eq!(scalars.len(), 0);
        assert_eq!(scalars.push(17.0), Err(17.0));
        assert!(started.elapsed().as_micros() < 8_000);
    }

    #[test]
    fn fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized() {
        use crate::artifacts::fem3d::{FemDof, FemLoadCase, FemMaterial, FemNode, FemSolid, FemSupport};

        let doc = Fem3dSnapshot {
            nodes: vec![FemNode { id: "n0".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n1".into(), x: 1.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 1.0, y: 1.0, z: 0.0 }, FemNode { id: "n3".into(), x: 0.0, y: 1.0, z: 0.0 }],
            materials: vec![FemMaterial { id: "m".into(), name: "M".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
            solids: vec![FemSolid { id: "s".into(), name: "S".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.25, layers: 1, mesh_size: 2.0, material_id: "m".into() }],
            supports: (0..4).map(|index| FemSupport { id: format!("f{index}"), node_id: format!("n{index}"), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] }).collect(),
            load_cases: vec![FemLoadCase { id: "g".into(), name: "G".into(), loads: vec![], self_weight: true }],
            ..Default::default()
        };
        let operation = semio_framework_job::Operation::new(OperationId(13), RevisionId(11), Generation(19), 23);
        let mut child = Fem3dNumericalChild::new();
        let mut fields = Fem3dSolverView::new(freshness(19), doc.nodes.len());
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview = 0;
        let mut terminal = false;
        for _ in 0..200_000 {
            let deadline = semio_framework_job::default_now_us().unwrap().checked_add(8_000).unwrap();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, deadline), cancel.clone(), semio_framework_job::default_now_us, &mut preview);
            let started = std::time::Instant::now();
            terminal = child.step(&doc, &mut fields, freshness(19), operation, &mut context).expect("production numerical child");
            assert_eq!(context.fuel_remaining(), 0);
            assert!(started.elapsed().as_micros() < 8_000);
            if terminal {
                break;
            }
        }
        assert!(terminal);
        assert!(fields.ready());
        assert!((0..fields.len).filter_map(|index| fields.scalar(index)).any(|scalar| scalar.displacement != [0.0; 3] || scalar.reaction != [0.0; 3]));
        assert!((0..fields.len).filter_map(|index| fields.scalar(index)).any(|scalar| scalar.eigen_estimate > 0.0 && scalar.mode_shape != [0.0; 3]));
        assert_eq!(child.close_step(0), (false, 0, 0));
        for _ in 0..200_000 {
            if child.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).0 {
                return;
            }
        }
        panic!("numerical child close did not reach exact terminal");
    }

    #[test]
    fn fem3d_production_field_correspondence_rejects_sparse_and_zero_aliases() {
        let scalar = Fem3dSolverScalar { displacement: [1.0, -2.0, 3.0], residual: [4.0, -5.0, 6.0], reaction: [7.0, -8.0, 9.0], contour: 3.0, mode_shape: [0.25, -0.5, 0.75], eigen_estimate: 17.0 };
        let mut fields = Fem3dSolverView::new(freshness(19), 2);
        let mut backing = Fem3dBackingCredit::new();
        for page in 0..FEM3D_SOLVER_PAGE_COUNT {
            assert!(fields.admit_page(page, false, &mut backing));
            assert!(fields.admit_page(page, true, &mut backing));
        }
        assert!(fields.publish_scalar(freshness(19), 1, scalar).is_ok());
        assert!(!fields.publish_progress(freshness(19), Fem3dVisualState::ValidatedFinal, 0.0, 1e-8, 2, 2));
        assert!(!fields.ready());
        assert!(fields.publish_scalar(freshness(19), 0, scalar).is_ok());
        assert!(fields.publish_progress(freshness(19), Fem3dVisualState::ValidatedFinal, 0.0, 1e-8, 2, 2));
        assert!(fields.ready());
        assert_ne!(fields.scalar(0).expect("field").reaction, fields.scalar(0).expect("field").residual);
        assert_ne!(fields.scalar(0).expect("field").mode_shape, fields.scalar(0).expect("field").displacement);
        assert!(fields.publish_scalar(freshness(23), 0, scalar).is_err());
        assert!(!fields.set_len(freshness(19), MAXIMUM_FIELDS + 1));
    }

    #[test]
    fn fem3d_process_permits_precede_solver_order_allocation_and_drop_handoff_closes_one_backing() {
        let mut backing = Fem3dBackingCredit::new();
        let region = FixedOrder::<MAXIMUM_REGIONS>::new(&mut backing).unwrap();
        let region_pointer = region.slots.as_ptr();
        let element = FixedOrder::<MAXIMUM_ELEMENTS>::new(&mut backing).unwrap();
        let element_pointer = element.slots.as_ptr();
        let before_refusal = (backing.live_items, backing.live_bytes);
        assert!(!backing.claim(FEM3D_PROCESS_BACKING_BYTES));
        assert_eq!((backing.live_items, backing.live_bytes), before_refusal);
        assert_eq!(region_pointer, region.slots.as_ptr());
        assert_eq!(element_pointer, element.slots.as_ptr());
        drop(region);
        assert!(backing.release(1, FEM3D_REGION_ORDER_BYTES));
        drop(element);
        assert!(backing.release(1, FEM3D_ELEMENT_ORDER_BYTES));
        assert!(backing.terminal_is_empty());

        let mut solver = Fem3dSolverView::new(freshness(31), 1);
        assert!(solver.admit_page(0, false, &mut backing));
        assert!(solver.admit_page(0, true, &mut backing));
        let scalar_pointer = solver.scalars[0].as_deref().unwrap().as_ptr();
        let initialized_pointer = solver.initialized[0].as_deref().unwrap().as_ptr();
        drop(solver);
        assert_eq!(close_recovered_fem3d_backing(0), Some((0, 0)));
        assert_eq!(close_recovered_fem3d_backing(FEM3D_SOLVER_SCALAR_PAGE_BYTES.max(FEM3D_SOLVER_INITIALIZED_PAGE_BYTES)), Some((1, FEM3D_SOLVER_SCALAR_PAGE_BYTES)));
        assert_eq!(close_recovered_fem3d_backing(FEM3D_SOLVER_INITIALIZED_PAGE_BYTES), Some((1, FEM3D_SOLVER_INITIALIZED_PAGE_BYTES)));
        assert!(!scalar_pointer.is_null());
        assert!(!initialized_pointer.is_null());

        let doc = Fem3dSnapshot::default();
        let solver = Fem3dSolverView::new(freshness(19), 0);
        let mut candidate = Fem3dPageVisualJob::new(freshness(19), credit(&doc));
        assert!(!candidate.step_one(&doc, &solver, &mut backing, freshness(19)).unwrap());
        assert!(!candidate.step_one(&doc, &solver, &mut backing, freshness(19)).unwrap());
        assert!(!candidate.step_one(&doc, &solver, &mut backing, freshness(19)).unwrap());
        drop(candidate);
        assert_eq!(close_recovered_fem3d_backing(FEM3D_REGION_ORDER_BYTES.max(FEM3D_ELEMENT_ORDER_BYTES)), Some((1, FEM3D_REGION_ORDER_BYTES)));
        assert_eq!(close_recovered_fem3d_backing(FEM3D_ELEMENT_ORDER_BYTES), Some((1, FEM3D_ELEMENT_ORDER_BYTES)));
        assert_eq!(world3d_snapshot_recovery_close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY), Some((1, 0)));
    }

    fn recovery_identity(app_instance_id: u32, generation: u64) -> Identity {
        Identity { app_instance_id, base_revision: RevisionId(11), generation: Generation(generation), canonical_base_revision: [generation as u8; 32], operation: OperationId(JOB_TAG | generation), job: JOB_TAG | generation }
    }

    fn recoverable_state(identity: Identity, recovery: Rc<MountedRecoverySlot>) -> MountedState {
        MountedState {
            identity,
            snapshot: None,
            snapshot_return: None,
            cancel: semio_framework_job::root_cancel_token(),
            preview_sequence: 0,
            credit: Fem3dPageCredit { item_count: 0, byte_count: 0, draw_count: 0, draw_bytes: 0 },
            backing: Fem3dBackingCredit::new(),
            solver: Some(Fem3dSolverView::new(identity.freshness(0), 1)),
            numerical: None,
            numerical_done: false,
            candidate: None,
            current: None,
            displaced: None,
            close_lane: 0,
            done: false,
            recovery,
        }
    }

    fn reserve_recovery_state(registry: &mut Registry, identity: Identity) -> (u16, Rc<MountedRecoverySlot>) {
        let shell = registry.allocate().unwrap();
        assert!(registry.reserve_credit(shell));
        let recovery = registry.recoveries[shell as usize].clone();
        assert!(recovery.reserve(identity));
        registry.current[identity.app_instance_id as usize % ACTIVE_CAPACITY] = Some(Current { app_instance_id: identity.app_instance_id, shell, identity });
        (shell, recovery)
    }

    fn drain_recovery_state(registry: &mut Registry, identity: Identity, shell: u16) {
        for _ in 0..128 {
            registry.recovery_cursor = shell as usize;
            let step = recover_abandoned_one(registry, identity.app_instance_id, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).unwrap();
            match step {
                PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
                }
                PluginCloseStep::Complete => {}
                PluginCloseStep::Blocked { reason } => panic!("{reason}"),
            }
            if !registry.recoveries[shell as usize].contains(identity.app_instance_id) {
                assert!(registry.shells[shell as usize].borrow().is_none());
                assert_eq!(registry.credit_items[shell as usize], 0);
                assert_eq!(registry.credit_bytes[shell as usize], 0);
                return;
            }
        }
        panic!("mounted recovery did not reach terminal zero");
    }

    #[test]
    fn fem3d_queued_running_and_state_drop_publish_exact_identity_and_drain_one_owner() {
        let mut registry = Registry::new();

        let queued_identity = recovery_identity(41, 1);
        let (queued_shell, queued_recovery) = reserve_recovery_state(&mut registry, queued_identity);
        *registry.shells[queued_shell as usize].borrow_mut() = Some(recoverable_state(queued_identity, queued_recovery.clone()));
        let queued_job = MountedJob { shell: registry.shells[queued_shell as usize].clone(), recovery: queued_recovery.clone(), identity: queued_identity, completed: false };
        let queued_borrow = registry.shells[queued_shell as usize].borrow_mut();
        drop(queued_job);
        assert_eq!(queued_recovery.publication.get(), Some((queued_identity, MountedRecoveryPublication::Recover)));
        assert_eq!(queued_borrow.as_ref().map(|state| state.identity), Some(queued_identity));
        drop(queued_borrow);
        registry.recovery_cursor = queued_shell as usize;
        let _ = recover_abandoned_one(&mut registry, queued_identity.app_instance_id, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY);
        assert_eq!(queued_recovery.owner.lock().unwrap().as_ref().map(|state| state.identity), Some(queued_identity));
        drain_recovery_state(&mut registry, queued_identity, queued_shell);

        let running_identity = recovery_identity(42, 2);
        let (running_shell, running_recovery) = reserve_recovery_state(&mut registry, running_identity);
        let mut running_state = recoverable_state(running_identity, running_recovery.clone());
        assert!(running_state.solver.as_mut().unwrap().admit_page(0, false, &mut running_state.backing));
        *registry.shells[running_shell as usize].borrow_mut() = Some(running_state);
        let running_job = MountedJob { shell: registry.shells[running_shell as usize].clone(), recovery: running_recovery.clone(), identity: running_identity, completed: false };
        drop(running_job);
        assert!(registry.shells[running_shell as usize].borrow().is_none());
        assert_eq!(running_recovery.owner.lock().unwrap().as_ref().map(|state| state.identity), Some(running_identity));
        drain_recovery_state(&mut registry, running_identity, running_shell);

        let state_identity = recovery_identity(43, 3);
        let (state_shell, state_recovery) = reserve_recovery_state(&mut registry, state_identity);
        drop(recoverable_state(state_identity, state_recovery.clone()));
        assert_eq!(state_recovery.publication.get(), Some((state_identity, MountedRecoveryPublication::Recover)));
        assert_eq!(state_recovery.owner.lock().unwrap().as_ref().map(|state| state.identity), Some(state_identity));
        drain_recovery_state(&mut registry, state_identity, state_shell);
    }
}
//#endregion 🧪️Laws
