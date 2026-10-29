//! 🧾️ Visual job contract: capacities, step ceiling, backing credit, visual state, field, freshness and job stage.

use super::*;

//#region 🔖️Contract
pub const FEM3D_MOUNTED_VISUAL_JOB_KIND: &str = "semio.fem3d.mounted-live-visual";
pub const ACTIVE_CAPACITY: usize = 16;
pub const SHELL_CAPACITY: usize = 32;
pub const MAXIMUM_REGIONS: usize = 32;
pub const MAXIMUM_NODES: usize = 128;
pub const MAXIMUM_ELEMENTS: usize = 128;
pub const MAXIMUM_SUPPORTS: usize = 64;
pub const MAXIMUM_LOADS: usize = 64;
pub const MAXIMUM_FIELDS: usize = 128;
/// 🧮️ Preflight census units one snapshot-read opportunity may charge before yielding to the next
/// refresh — each unit is one constant-cost owner or item, so a demo-sized census admits within the
/// opportunity that opened it rather than one host refresh per unit.
const PREFLIGHT_UNITS_PER_OPPORTUNITY: usize = 4_096;
/// ⏱️ Wall-clock ceiling of one host job step (the interactive 8 ms contract) and the most session
/// units the job bridge drives inside it — one host round trip per ceiling, never one per unit.
pub const STEP_CEILING_MS: u64 = 8;
const UNITS_PER_STEP: usize = 65_536;
pub const FAULT_BYTES: usize = WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY;
pub const JOB_TAG: u64 = 0xf3d0_0000_0000_0000;
pub const JOB_COUNTER_MAXIMUM: u64 = 0x000f_ffff_ffff_ffff;
pub const INPUT_BYTES: usize = 63;
pub(super) const FEM3D_SOLVER_FIELDS_PER_PAGE: usize = 16;
pub(super) const FEM3D_SOLVER_PAGE_COUNT: usize = MAXIMUM_FIELDS / FEM3D_SOLVER_FIELDS_PER_PAGE;
pub const FEM3D_SOLVER_SCALAR_PAGE_BYTES: usize = size_of::<Fem3dSolverScalar>() * FEM3D_SOLVER_FIELDS_PER_PAGE;
pub const FEM3D_SOLVER_INITIALIZED_PAGE_BYTES: usize = size_of::<bool>() * FEM3D_SOLVER_FIELDS_PER_PAGE;
pub const FEM3D_REGION_ORDER_BYTES: usize = size_of::<Option<usize>>() * MAXIMUM_REGIONS;
pub const FEM3D_ELEMENT_ORDER_BYTES: usize = size_of::<Option<usize>>() * MAXIMUM_ELEMENTS;
pub const FEM3D_PROCESS_BACKING_ITEMS: usize = FEM3D_SOLVER_PAGE_COUNT * 2 + 2;
pub const FEM3D_PROCESS_BACKING_BYTES: usize = FEM3D_SOLVER_PAGE_COUNT * (FEM3D_SOLVER_SCALAR_PAGE_BYTES + FEM3D_SOLVER_INITIALIZED_PAGE_BYTES) + FEM3D_REGION_ORDER_BYTES + FEM3D_ELEMENT_ORDER_BYTES;

#[derive(Clone, Copy)]
pub struct Fem3dBackingCredit {
    admitted_items: usize,
    admitted_bytes: usize,
    pub live_items: usize,
    pub live_bytes: usize,
}

impl Fem3dBackingCredit {
    pub fn new() -> Self {
        Self { admitted_items: FEM3D_PROCESS_BACKING_ITEMS, admitted_bytes: FEM3D_PROCESS_BACKING_BYTES, live_items: 0, live_bytes: 0 }
    }

    pub fn claim(&mut self, bytes: usize) -> bool {
        let Some(items) = self.live_items.checked_add(1) else { return false };
        let Some(live_bytes) = self.live_bytes.checked_add(bytes) else { return false };
        if items > self.admitted_items || live_bytes > self.admitted_bytes {
            return false;
        }
        self.live_items = items;
        self.live_bytes = live_bytes;
        true
    }

    pub fn release(&mut self, items: usize, bytes: usize) -> bool {
        let Some(live_items) = self.live_items.checked_sub(items) else { return false };
        let Some(live_bytes) = self.live_bytes.checked_sub(bytes) else { return false };
        self.live_items = live_items;
        self.live_bytes = live_bytes;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
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


impl Fem3dNumericalStage {
    pub fn progress_stage(self) -> &'static str {
        match self {
            Self::SolidMeshBegin | Self::SolidMesh => "fem3d.meshing",
            Self::MountAssembly | Self::PrepareAssembly | Self::Assembly | Self::BuildCsr => "fem3d.assembly",
            Self::BeginPcg | Self::PreparePcg | Self::Pcg => "fem3d.statics",
            Self::BeginModal | Self::PrepareModal | Self::BeginLdlt | Self::Ldlt | Self::BeginSubspace | Self::Subspace | Self::ReadModeScalar | Self::PublishModeScalar => "fem3d.modes",
            Self::ReadNodeScalar | Self::RecoverReaction | Self::PublishNodeScalar | Self::RecoverStressElement | Self::PublishStressScalar | Self::PublishProgress => "fem3d.validation",
            Self::Complete => "fem3d.validated",
            _ => "fem3d.preparation",
        }
    }
}

//#endregion 🔖️Contract

/// 🧮️ Source and mode indices belong to the admitted generation original document.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Fem3dResultRequest {
    pub source: Fem3dResultSource,
    pub mode: Option<usize>,
    pub buckling: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Fem3dResultSource {
    #[default]
    FirstCase,
    Case(usize),
    Combination(usize),
}
