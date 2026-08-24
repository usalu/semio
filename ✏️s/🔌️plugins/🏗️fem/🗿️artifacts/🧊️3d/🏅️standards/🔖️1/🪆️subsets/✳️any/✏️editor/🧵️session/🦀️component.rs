//! 🧵️ Mounted FEM3D visual publication on the shared bounded-job reactor.

use crate::artifacts::fem3d::{element_id, load_id, Fem3dSnapshot, FemElement};
use semio_framework::kernel::{Effect, JobPlacement};
use semio_framework_job::{Generation, OperationId, RevisionId, StepBudget, StepContext};
use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, JobBudget, JobStep};
use semio_framework_plugin::{AppRenderOperationContext, ArtifactView, PluginCloseStep};
use semio_framework_ui_scene::{
    world3d_snapshot_abort_write, world3d_snapshot_abort_write_step, world3d_snapshot_admit_page, world3d_snapshot_begin, world3d_snapshot_begin_close, world3d_snapshot_close_step, world3d_snapshot_seal, world3d_snapshot_terminal_is_empty,
    world3d_snapshot_with_page, world3d_snapshot_write_terminal_is_empty, World3dSnapshotDescriptor, World3dSnapshotFault, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPage, World3dSnapshotPageKind, World3dSnapshotWriteToken,
    WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY,
};
use std::cell::RefCell;
use std::rc::Rc;

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
    scalars: Option<Box<[std::mem::MaybeUninit<Fem3dSolverScalar>; MAXIMUM_FIELDS]>>,
    initialized: Option<Box<[bool; MAXIMUM_FIELDS]>>,
    len: usize,
    close_lane: u8,
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
            scalars: Some(Box::new([std::mem::MaybeUninit::uninit(); MAXIMUM_FIELDS])),
            initialized: Some(Box::new([false; MAXIMUM_FIELDS])),
            len,
            close_lane: 0,
        }
    }

    fn scalar(&self, index: usize) -> Option<Fem3dSolverScalar> {
        if index >= self.len || !self.initialized.as_ref().is_some_and(|initialized| initialized[index]) {
            return None;
        }
        self.scalars.as_ref().map(|scalars| unsafe { scalars[index].assume_init() })
    }

    pub fn publish_scalar(&mut self, freshness: Fem3dVisualFreshness, index: usize, scalar: Fem3dSolverScalar) -> Result<(), Fem3dSolverScalar> {
        if freshness != self.freshness || index >= self.len {
            return Err(scalar);
        }
        let (Some(scalars), Some(initialized)) = (self.scalars.as_mut(), self.initialized.as_mut()) else {
            return Err(scalar);
        };
        scalars[index].write(scalar);
        initialized[index] = true;
        self.completed = self.completed.max(index + 1);
        self.state = Fem3dVisualState::SolvingUnconverged;
        Ok(())
    }

    pub fn publish_progress(&mut self, freshness: Fem3dVisualFreshness, state: Fem3dVisualState, residual_norm: f64, tolerance: f64, completed: usize, total: usize) -> bool {
        if freshness != self.freshness || completed > total || total > MAXIMUM_FIELDS {
            return false;
        }
        self.state = state;
        self.residual_norm = residual_norm;
        self.tolerance = tolerance;
        self.completed = completed;
        self.total = total;
        true
    }

    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let bytes = match self.close_lane {
            0 => std::mem::size_of::<Fem3dSolverScalar>() * MAXIMUM_FIELDS,
            1 => std::mem::size_of::<bool>() * MAXIMUM_FIELDS,
            _ => return (true, 0, 0),
        };
        if bytes > maximum_bytes {
            return (false, 0, 0);
        }
        if self.close_lane == 0 {
            self.scalars = None;
        } else {
            self.initialized = None;
            self.len = 0;
        }
        self.close_lane += 1;
        (self.close_lane >= 2, 1, bytes)
    }
}

struct FixedOrder<const N: usize> {
    slots: Box<[Option<usize>; N]>,
    len: usize,
}

impl<const N: usize> FixedOrder<N> {
    fn new() -> Self {
        Self { slots: Box::new([None; N]), len: 0 }
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
        World3dSnapshotDescriptor { revision: freshness.model_revision, generation: freshness.renderer_scene_generation, page_count: FEM3D_VISUAL_PAGES as u16, item_count: self.item_count, byte_count: self.byte_count }
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

    fn field_numbers(field: Fem3dSolverScalar, vector: [f64; 3], scalar: f64) -> [f64; 16] {
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

    fn step_one(&mut self, doc: &Fem3dSnapshot, solver: &Fem3dSolverView, freshness: Fem3dVisualFreshness) -> Result<bool, Vec<u8>> {
        match self.stage {
            Fem3dVisualJobStage::ReserveSnapshot => {
                if doc.solids.len() > MAXIMUM_REGIONS || doc.nodes.len() > MAXIMUM_NODES || doc.elements.len() > MAXIMUM_ELEMENTS || doc.supports.len() > MAXIMUM_SUPPORTS || doc.nodes.len() > MAXIMUM_FIELDS {
                    return Err(b"fem3d.visual-maximum-plus-one".to_vec());
                }
                if self.reserve_lane == 0 {
                    self.token = Some(world3d_snapshot_begin(self.credit.descriptor(self.freshness)).map_err(|_| b"fem3d.visual-page-preflight".to_vec())?);
                } else if self.reserve_lane == 1 {
                    self.region_order = Some(FixedOrder::new());
                } else if self.reserve_lane == 2 {
                    self.element_order = Some(FixedOrder::new());
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
                self.push_item(Self::field_page(self.stage, self.cursor), [Some(&node.id), None, None, None], Self::field_numbers(field, vector, scalar), 11, [0; 8], 0, flag)?;
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
            Fem3dVisualJobStage::RetireDisplacedLease => self.stage = Fem3dVisualJobStage::Complete,
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

struct MountedState {
    identity: Identity,
    snapshot: Option<store::SnapshotRead<Fem3dSnapshot>>,
    snapshot_return: Option<store::SnapshotReadReturn>,
    cancel: semio_framework_job::CancelToken,
    preview_sequence: u64,
    credit: Fem3dPageCredit,
    solver: Option<Fem3dSolverView>,
    candidate: Option<Fem3dPageVisualJob>,
    current: Option<Fem3dPageVisualLease>,
    displaced: Option<Fem3dPageVisualLease>,
    fault: Option<Vec<u8>>,
    close_lane: u8,
    done: bool,
}

impl MountedState {
    fn new(identity: Identity, snapshot: store::SnapshotRead<Fem3dSnapshot>, current: Option<Fem3dPageVisualLease>, credit: Fem3dPageCredit) -> Self {
        let solver = Fem3dSolverView::new(identity.freshness(0), snapshot.nodes.len());
        Self {
            identity,
            snapshot: Some(snapshot),
            snapshot_return: None,
            cancel: semio_framework_job::root_cancel_token(),
            preview_sequence: 0,
            credit,
            solver: Some(solver),
            candidate: None,
            current,
            displaced: None,
            fault: None,
            close_lane: 0,
            done: false,
        }
    }

    fn fail(&mut self, detail: Vec<u8>) -> JobStep {
        self.fault = Some(if detail.capacity() <= FAULT_BYTES { detail } else { b"fem3d.visual-fault-capacity".to_vec() });
        JobStep::Failed(self.fault.clone().unwrap_or_else(|| b"fem3d.visual-fault".to_vec()))
    }

    fn step(&mut self, budget: JobBudget) -> JobStep {
        if self.cancel.is_cancelled_now() {
            return self.fail(b"fem3d.visual-cancelled".to_vec());
        }
        if budget.fuel == 0 || budget.deadline_ms == 0 {
            return JobStep::Running(None);
        }
        let now = semio_framework_job::default_now_ms();
        let deadline = now.saturating_add(u64::from(budget.deadline_ms).min(8));
        let mut cx = StepContext::new(self.identity.operation, self.identity.generation, StepBudget::new(budget.fuel, deadline), self.cancel.clone(), semio_framework_job::default_now_ms, &mut self.preview_sequence);
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
        if self.candidate.is_none() {
            cx.consume_fuel(1);
            let fields_ready = self.solver.as_ref().is_some_and(|solver| solver.completed == self.snapshot.as_ref().map_or(0, |snapshot| snapshot.nodes.len()) && solver.total == solver.completed);
            if !fields_ready {
                return JobStep::Running(None);
            }
            self.candidate = Some(Fem3dPageVisualJob::new(self.identity.freshness(self.preview_sequence), self.credit));
            return JobStep::Running(None);
        }
        let freshness = self.identity.freshness(self.preview_sequence);
        let Some(snapshot) = self.snapshot.as_ref() else { return self.fail(b"fem3d.visual-snapshot-owner".to_vec()) };
        let Some(solver) = self.solver.as_ref() else { return self.fail(b"fem3d.visual-solver-owner".to_vec()) };
        cx.consume_fuel(1);
        let step = self.candidate.as_mut().map(|candidate| candidate.step_one(snapshot, solver, freshness));
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
                    let (terminal, items, bytes) = candidate.close_step(maximum_bytes);
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
                if let Some(solver) = self.solver.as_mut() {
                    let (terminal, items, bytes) = solver.close_step(maximum_bytes);
                    if !terminal {
                        return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
                    }
                    self.solver = None;
                    return PluginCloseStep::Pending { released_items: items, released_bytes: bytes };
                }
                self.close_lane = 4;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            4 => {
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
                self.close_lane = 5;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            5 => {
                if let Some(fault) = self.fault.as_mut() {
                    if fault.pop().is_some() {
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    let bytes = fault.capacity();
                    if bytes > maximum_bytes {
                        return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    *fault = Vec::new();
                    self.fault = None;
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
                }
                self.close_lane = 6;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            _ => PluginCloseStep::Complete,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_lane >= 6 && self.snapshot.is_none() && self.snapshot_return.is_none() && self.solver.is_none() && self.candidate.is_none() && self.current.is_none() && self.displaced.is_none() && self.fault.is_none()
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
                self.charge(10 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 0, 0)?;
                self.charge(12 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 0, 0)?;
                self.charge(14 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 0, 0)?;
                self.charge(16 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 0, 0)?;
                self.charge(18 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 0, 0)?;
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
    current: [Option<Current>; ACTIVE_CAPACITY],
    pending: [Option<PendingSnapshot>; ACTIVE_CAPACITY],
    retiring: [bool; SHELL_CAPACITY],
    retiring_app: [u32; SHELL_CAPACITY],
    retiring_count: [usize; ACTIVE_CAPACITY],
    retiring_owner: [u32; ACTIVE_CAPACITY],
    free: [u16; SHELL_CAPACITY],
    free_len: usize,
    maintenance_cursor: usize,
    next_job: u64,
}

impl Registry {
    fn new() -> Self {
        Self {
            shells: std::array::from_fn(|_| Rc::new(RefCell::new(None))),
            current: std::array::from_fn(|_| None),
            pending: std::array::from_fn(|_| None),
            retiring: [false; SHELL_CAPACITY],
            retiring_app: [0; SHELL_CAPACITY],
            retiring_count: [0; ACTIVE_CAPACITY],
            retiring_owner: [0; ACTIVE_CAPACITY],
            free: std::array::from_fn(|index| (SHELL_CAPACITY - 1 - index) as u16),
            free_len: SHELL_CAPACITY,
            maintenance_cursor: 0,
            next_job: 0,
        }
    }

    fn allocate(&mut self) -> Option<u16> {
        let next = self.free_len.checked_sub(1)?;
        self.free_len = next;
        Some(self.free[next])
    }

    fn release(&mut self, shell: u16) {
        if self.free_len < SHELL_CAPACITY {
            self.free[self.free_len] = shell;
            self.free_len += 1;
        }
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
    identity: Identity,
}

impl BoundedJob for MountedJob {
    fn step(&mut self, budget: JobBudget) -> JobStep {
        let Ok(mut shell) = self.shell.try_borrow_mut() else { return JobStep::Running(None) };
        let Some(state) = shell.as_mut().filter(|state| state.identity == self.identity) else { return JobStep::Failed(b"fem3d.visual-stale-shell".to_vec()) };
        state.step(budget)
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
        if !owner.try_borrow().is_ok_and(|state| state.as_ref().is_some_and(|state| state.identity == identity)) {
            return Err(b"fem3d.visual-stale-factory".to_vec());
        }
        Ok(Box::new(MountedJob { shell: owner, identity }) as Box<dyn BoundedJob>)
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
        let Some(counter) = registry.next_job.checked_add(1).filter(|counter| *counter <= JOB_COUNTER_MAXIMUM) else {
            registry.release(shell);
            return Vec::new();
        };
        let snapshot = match doc.take_snapshot_read() {
            Ok(snapshot) => snapshot,
            Err(_) => {
                registry.release(shell);
                return Vec::new();
            }
        };
        registry.next_job = counter;
        registry.pending[slot] = None;
        let job = JOB_TAG | counter;
        let identity = Identity { app_instance_id: render.app_instance_id, base_revision: render.base_revision, generation: render.generation, canonical_base_revision: render.canonical_base_revision, operation: OperationId(job), job };
        if let Some(previous) = previous {
            if !registry.retain_retiring(render.app_instance_id, previous.shell) {
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
        *registry.shells[shell as usize].borrow_mut() = Some(MountedState::new(identity, snapshot, former, credit));
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
            let Some(state) = owner.as_mut() else { return PluginCloseStep::Blocked { reason: "FEM3D retiring shell empty" } };
            state.close_step(maximum_bytes)
        };
        if matches!(step, PluginCloseStep::Complete) {
            let terminal = registry.shells[shell].borrow().as_ref().is_some_and(MountedState::terminal_is_empty);
            if !terminal {
                return PluginCloseStep::Blocked { reason: "FEM3D retiring state false terminal" };
            }
            *registry.shells[shell].borrow_mut() = None;
            registry.retiring[shell] = false;
            registry.retiring_app[shell] = 0;
            let slot = app_instance_id as usize % ACTIVE_CAPACITY;
            registry.retiring_count[slot] = registry.retiring_count[slot].saturating_sub(1);
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
    retire_one(app_instance_id, maximum_bytes)
}

pub fn close_step(app_instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
    if maximum_items == 0 {
        return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
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
        registry.pending[slot].is_none() && registry.current[slot].is_none_or(|current| current.app_instance_id != app_instance_id) && (registry.retiring_owner[slot] != app_instance_id || registry.retiring_count[slot] == 0)
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
        for index in 0..doc.nodes.len() {
            solver.publish_scalar(freshness(19), index, scalar).expect("generation-qualified scalar");
        }
        assert!(solver.publish_progress(freshness(19), Fem3dVisualState::ValidatedFinal, 0.125, 1e-8, doc.nodes.len(), doc.nodes.len()));
        solver
    }

    fn build(doc: &Fem3dSnapshot, solver: &Fem3dSolverView) -> Fem3dPageVisualLease {
        let mut job = Fem3dPageVisualJob::new(freshness(19), credit(doc));
        let mut turns = 0;
        while !job.step_one(doc, solver, freshness(19)).expect("one production opportunity") && turns < 4_096 {
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
        assert!(job.step_one(&doc, &solver, freshness(19)).is_err());
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
        let mut turns = 0;
        while stale.stage() != Fem3dVisualJobStage::ValidateFreshness && turns < 1_024 {
            stale.step_one(&doc, &solver, freshness(19)).expect("bounded step");
            turns += 1;
        }
        assert!(stale.step_one(&doc, &solver, freshness(23)).is_err());
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
        let started = std::time::Instant::now();
        let _ = job.step_one(&doc, &solver, freshness(19));
        assert!(started.elapsed().as_micros() < 8_000);
        while !job.close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).0 {}
        assert!(job.terminal_is_empty());
    }
}
//#endregion 🧪️Laws
