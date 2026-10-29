//! 📄️ Paged world-snapshot publication: process backing recovery, fixed order, page credit, page lease and the page visual job.

use super::*;

//#region 📦️PageVisualJob

pub struct FixedOrder<const N: usize> {
    pub slots: Box<[Option<usize>; N]>,
    pub len: usize,
}

pub(super) enum Fem3dRecoveredBacking {
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
    pub(super) const fn new() -> Self {
        Self { owners: [const { None }; FEM3D_RECOVERED_BACKING_CAPACITY] }
    }
}

thread_local! {
    static FEM3D_BACKING_RECOVERY: RefCell<Fem3dBackingRecovery> = const { RefCell::new(Fem3dBackingRecovery::new()) };
}

pub(super) fn recover_fem3d_backing(owner: Fem3dRecoveredBacking) -> bool {
    FEM3D_BACKING_RECOVERY.with(|recovery| {
        let Ok(mut recovery) = recovery.try_borrow_mut() else { return false };
        let Some(slot) = recovery.owners.iter_mut().find(|slot| slot.is_none()) else { return false };
        *slot = Some(owner);
        true
    })
}

pub fn close_recovered_fem3d_backing(maximum_bytes: usize) -> Option<(usize, usize)> {
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
    pub fn new(backing: &mut Fem3dBackingCredit) -> Result<Self, ()> {
        let bytes = size_of::<Option<usize>>() * N;
        if !backing.claim(bytes) {
            return Err(());
        }
        Ok(Self { slots: Box::new([None; N]), len: 0 })
    }

    pub fn push(&mut self, value: usize) -> Result<(), ()> {
        let Some(slot) = self.slots.get_mut(self.len) else { return Err(()) };
        *slot = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        self.slots.get(index).copied().flatten()
    }

    fn swap(&mut self, left: usize, right: usize) {
        self.slots.swap(left, right);
    }
}

#[derive(Clone, Copy)]
pub struct Fem3dPageCredit {
    pub item_count: u32,
    pub byte_count: u32,
    pub draw_count: u32,
    pub draw_bytes: u32,
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
    pub(super) freshness: Fem3dVisualFreshness,
    pub snapshot: World3dSnapshotLease,
    close_started: bool,
}

impl Fem3dPageVisualLease {
    pub fn snapshot(&self) -> World3dSnapshotLease {
        self.snapshot
    }

    pub fn matches(&self, freshness: Fem3dVisualFreshness) -> bool {
        self.freshness == freshness
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
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

    pub fn terminal_is_empty(&self) -> bool {
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
    pub(super) freshness: Fem3dVisualFreshness,
    pub credit: Fem3dPageCredit,
    pub stage: Fem3dVisualJobStage,
    token: Option<World3dSnapshotWriteToken>,
    pages: [Option<World3dSnapshotPage>; FEM3D_VISUAL_PAGES],
    pub region_order: Option<FixedOrder<MAXIMUM_REGIONS>>,
    pub element_order: Option<FixedOrder<MAXIMUM_ELEMENTS>>,
    reserve_lane: usize,
    scalar_cursor: u8,
    order_input: usize,
    order_slot: usize,
    cursor: usize,
    pub(super) lookup_cursor: usize,
    pub(super) point_cursor: usize,
    item_phase: u8,
    load_case_cursor: usize,
    pub(super) load_cursor: usize,
    solid_sum: [f64; 2],
    endpoint_a: [f64; 3],
    endpoint_b: [f64; 3],
    seal_cursor: usize,
    admit_cursor: usize,
    validated: bool,
    pub complete: Option<Fem3dPageVisualLease>,
    abort_started: bool,
}

/// 📦️ Scalar payload and active lengths for one visual page item.
#[derive(Clone, Copy)]
struct Fem3dVisualItemPayload {
    pub numbers: [f64; 16],
    number_len: u8,
    indexes: [u32; 8],
    index_len: u8,
    flags: u16,
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

    fn push_item(&mut self, page: usize, strings: [Option<&str>; 4], payload: Fem3dVisualItemPayload) -> Result<(), Vec<u8>> {
        let Fem3dVisualItemPayload { numbers, number_len, indexes, index_len, flags } = payload;
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
                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("solid", &solid.id).map_err(|_| b"fem3d.visual-identity".to_vec())?;
                self.push_item(1, [Some(&identity), Some(if kind == 1 { "tetrahedron" } else { "hexahedron" }), None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers(solid.axis.to_world(self.solid_sum[0] / divisor, self.solid_sum[1] / divisor, solid.base_z + solid.height * 0.5), [1.0, 1.0, solid.height]), number_len: 14, indexes: [kind, 0, 0, 0, 0, 0, 0, 0], index_len: 1, flags: 1 })?;
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
                        let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("node", &node.id).map_err(|_| b"fem3d.visual-identity".to_vec())?;
                        self.push_item(2 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([node.x, node.y, node.z], [0.05; 3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 2 })?;
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
                    if node.id.as_str() == start {
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
                    if node.id.as_str() == end {
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
                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("element", element_id(element)).map_err(|_| b"fem3d.visual-identity".to_vec())?;
                self.push_item(4 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers(midpoint, [0.05, 0.05, length]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 3 })?;
                self.cursor += 1;
                self.item_phase = 1;
            }
            Fem3dVisualJobStage::BuildAssemblyMark => {
                let Some(index) = self.element_order.as_ref().and_then(|order| order.get(self.cursor)) else {
                    self.advance(Fem3dVisualJobStage::BuildLoadGlyph);
                    return Ok(false);
                };
                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("element", element_id(&doc.elements[index])).map_err(|_| b"fem3d.visual-identity".to_vec())?;
                self.push_item(6 + self.cursor / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([0.0; 3], [0.03; 3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 4 })?;
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
                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_load_target(&case.id, load_id(load)).map_err(|_| b"fem3d.visual-identity".to_vec())?;
                self.push_item(8, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([0.0, 0.0, 1.0], [0.03, 0.03, 0.3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 5 })?;
                self.load_cursor += 1;
            }
            Fem3dVisualJobStage::BuildSupportGlyph => {
                let Some(support) = doc.supports.get(self.cursor) else {
                    self.advance(Fem3dVisualJobStage::BuildDisplacementEntry);
                    return Ok(false);
                };
                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("support", &support.id).map_err(|_| b"fem3d.visual-identity".to_vec())?;
                self.push_item(9, [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::instance_numbers([0.0; 3], [0.1; 3]), number_len: 14, indexes: [0; 8], index_len: 0, flags: 6 })?;
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
                let identity = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("node", &node.id).map_err(|_| b"fem3d.visual-identity".to_vec())?;
                self.push_item(Self::field_page(self.stage, self.cursor), [Some(&identity), None, None, None], Fem3dVisualItemPayload { numbers: Self::field_numbers(field, [node.x, node.y, node.z], vector, scalar), number_len: 14, indexes: [0; 8], index_len: 0, flags: flag })?;
                self.cursor += 1;
            }
            Fem3dVisualJobStage::BuildLabelEntry => {
                if self.cursor == 0 {
                    self.push_item(20, [Some("en"), Some(FEM3D_VISUAL_LABEL_EN), None, None], Fem3dVisualItemPayload { numbers: Self::progress_numbers(solver), number_len: 3, indexes: [solver.total as u32, solver.state as u32, 0, 0, 0, 0, 0, 0], index_len: 2, flags: 20 })?;
                    self.cursor = 1;
                } else if self.cursor == 1 {
                    self.push_item(20, [Some("de"), Some(FEM3D_VISUAL_LABEL_DE), None, None], Fem3dVisualItemPayload { numbers: Self::progress_numbers(solver), number_len: 3, indexes: [solver.total as u32, solver.state as u32, 0, 0, 0, 0, 0, 0], index_len: 2, flags: 20 })?;
                    self.cursor = 2;
                } else if self.cursor == 2 {
                    self.push_item(0, [Some("box"), None, None, None], Fem3dVisualItemPayload { numbers: [0.0; 16], number_len: 0, indexes: [self.credit.draw_count, self.credit.draw_bytes, 0, 0, 0, 0, 0, 0], index_len: 2, flags: 30 })?;
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

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
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
            if maximum_bytes < WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY {
                return (false, 0, 0);
            }
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
        if self.region_order.is_some() {
            if maximum_bytes < FEM3D_REGION_ORDER_BYTES {
                return (false, 0, 0);
            }
            self.region_order = None;
            return (false, 1, FEM3D_REGION_ORDER_BYTES);
        }
        if self.element_order.is_some() {
            if maximum_bytes < FEM3D_ELEMENT_ORDER_BYTES {
                return (false, 0, 0);
            }
            self.element_order = None;
            return (false, 1, FEM3D_ELEMENT_ORDER_BYTES);
        }
        (true, 0, 0)
    }

    pub fn terminal_is_empty(&self) -> bool {
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
