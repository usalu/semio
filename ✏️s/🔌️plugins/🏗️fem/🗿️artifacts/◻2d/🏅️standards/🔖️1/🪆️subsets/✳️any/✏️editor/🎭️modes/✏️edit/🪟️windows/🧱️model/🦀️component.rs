//! 🧱️ Fem2d play app — the model window: the editable 2D structural canvas (nodes/members/supports,
//! mesh-edge preview overlay). Also hosts the screen-space draw helpers shared with the results window
//! (`crate::editor::fem2d::modes::edit::windows::results`) — kept here rather than in the artifact's
//! `⚙️engine` because they take/return app-facing `semio_framework_plugin` scene types and their only
//! two consumers are these two sibling windows, both at app level.

use crate::artifacts::fem2d::{element_id, Fem2dSnapshot, FemCamera, FemDof, FemElement, FemLoad};
use crate::model::Dof;
use semio_framework_plugin::{BuiltNode, Canvas2dScene};
use semio_framework_ui_scene::{
    canvas2d_snapshot_abort_write, canvas2d_snapshot_abort_write_step, canvas2d_snapshot_admit_page, canvas2d_snapshot_begin, canvas2d_snapshot_begin_close, canvas2d_snapshot_close_step, canvas2d_snapshot_seal,
    canvas2d_snapshot_terminal_is_empty, canvas2d_snapshot_with_page, canvas2d_snapshot_write_terminal_is_empty, Canvas2dSnapshotDescriptor, Canvas2dSnapshotLease, Canvas2dSnapshotPage, Canvas2dSnapshotWriteToken,
    CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY,
};
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Write;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "fem2d-model";
pub const BODY_KEY: &str = "fem2d.play.model";

/// 📐️ Model-meters -> screen-pixels scale for the 2D canvas (a 6m span shouldn't render as 6px wide).
pub(crate) const SCALE_2D: f64 = 20.0;
/// 📐️ Screen-space origin offset so a structure anchored at (0,0) isn't drawn at the canvas corner.
pub(crate) const ORIGIN_2D: f64 = 40.0;
/// 📐️ Exaggeration factor for offsetting the moment-diagram polyline perpendicular to a member — single
/// consumer: the results window's static-results moment diagram
/// (`crate::editor::fem2d::modes::edit::windows::results::render`).
pub(crate) const MOMENT_SCALE_2D: f64 = 0.001;

/// 🎨️ Muted color for the mesh-edge preview overlay drawn under this window's members.
const MESH_EDGE_COLOR: &str = "#475569";
//#endregion 🔖️Constants

//#region 👁️LiveVisualLanguage
/// 🎨️ Stable region-quality vocabulary shared by mesh-job previews and the 2D canvas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RegionVisualQuality {
    #[default]
    Unmeshed,
    Coarse,
    Refined,
    Final,
}

impl RegionVisualQuality {
    pub(crate) fn color(self) -> &'static str {
        match self {
            Self::Unmeshed => "#64748b",
            Self::Coarse => "#f59e0b",
            Self::Refined => "#38bdf8",
            Self::Final => "#22c55e",
        }
    }

    pub(crate) fn id(self) -> &'static str {
        match self {
            Self::Unmeshed => "unmeshed",
            Self::Coarse => "coarse",
            Self::Refined => "refined",
            Self::Final => "final",
        }
    }
}

/// 📈️ One node's replaceable iterative-solver field sample in model coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeLiveField {
    pub node_id: String,
    pub displacement: [f64; 2],
    pub residual: [f64; 2],
    pub reaction: [f64; 2],
    pub contour: f64,
    pub mode_shape: [f64; 2],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FemVisualState {
    #[default]
    Unmeshed,
    Coarse,
    Refined,
    Assembling,
    SolvingUnconverged,
    SolvingConverged,
    ValidatedFinal,
    FaultedCancelled,
}

impl FemVisualState {
    fn id(self) -> &'static str {
        match self {
            Self::Unmeshed => "unmeshed",
            Self::Coarse => "coarse-mesh",
            Self::Refined => "refined-mesh",
            Self::Assembling => "assembling",
            Self::SolvingUnconverged => "solving-unconverged",
            Self::SolvingConverged => "solving-converged",
            Self::ValidatedFinal => "validated-final",
            Self::FaultedCancelled => "faulted-cancelled-last-valid",
        }
    }
}

/// 👁️ Replaceable, non-authoritative progress consumed by the 2D surface.
const FEM2D_LIVE_REGION_CAPACITY: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct Fem2dRegionQualitySlots {
    slots: [Option<(String, RegionVisualQuality)>; FEM2D_LIVE_REGION_CAPACITY],
}

impl Fem2dRegionQualitySlots {
    pub fn insert(&mut self, id: String, quality: RegionVisualQuality) -> Result<Option<RegionVisualQuality>, String> {
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|(current, _)| *current == id)) {
            let previous = slot.as_mut().map(|(_, current)| std::mem::replace(current, quality));
            return Ok(previous);
        }
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) else { return Err(id) };
        *slot = Some((id, quality));
        Ok(None)
    }

    pub fn get(&self, id: &str) -> Option<&RegionVisualQuality> {
        self.slots.iter().find_map(|slot| slot.as_ref().filter(|(current, _)| current == id).map(|(_, quality)| quality))
    }

    pub fn update(&mut self, id: &str, quality: RegionVisualQuality) -> bool {
        let Some((_, current)) = self.slots.iter_mut().find_map(|slot| slot.as_mut().filter(|(current, _)| current == id)) else { return false };
        *current = quality;
        true
    }

    pub fn take_one(&mut self) -> Option<(String, RegionVisualQuality)> {
        self.slots.iter_mut().find(|slot| slot.is_some()).and_then(Option::take)
    }

    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(Option::is_none)
    }

    pub(crate) fn slot(&self, index: usize) -> Option<&(String, RegionVisualQuality)> {
        self.slots.get(index).and_then(Option::as_ref)
    }
}

impl Default for Fem2dRegionQualitySlots {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None) }
    }
}

impl FromIterator<(String, RegionVisualQuality)> for Fem2dRegionQualitySlots {
    fn from_iter<T: IntoIterator<Item = (String, RegionVisualQuality)>>(iter: T) -> Self {
        let mut slots = Self::default();
        for (id, quality) in iter.into_iter().take(FEM2D_LIVE_REGION_CAPACITY) {
            let _ = slots.insert(id, quality);
        }
        slots
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fem2dLiveVisual {
    pub region_quality: Fem2dRegionQualitySlots,
    pub assembling_element_ids: Vec<String>,
    pub fields: Vec<NodeLiveField>,
    pub state: FemVisualState,
    pub residual_norm: f64,
    pub tolerance: f64,
    pub progress_completed: usize,
    pub progress_total: usize,
    pub converged: bool,
    pub validated_final: bool,
}

const FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES: usize = 16 * 1_024;
const FEM2D_MOUNTED_VISUAL_PAGE_BYTES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fem2dVisualFreshness {
    pub app_instance_id: u32,
    pub model_revision: u64,
    pub document_generation: u64,
    pub operation: u64,
    pub numerical_preview_sequence: u64,
    pub surface_generation: u64,
    pub renderer_scene_generation: u64,
}

const FEM2D_MOUNTED_VISUAL_PAGE_COUNT: usize = FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES / FEM2D_MOUNTED_VISUAL_PAGE_BYTES;

#[derive(Debug)]
struct Fem2dFixedPacketPages {
    pages: [Option<Canvas2dSnapshotPage>; FEM2D_MOUNTED_VISUAL_PAGE_COUNT],
    write_page: usize,
    len: usize,
}

impl Fem2dFixedPacketPages {
    fn new() -> Self {
        Self { pages: std::array::from_fn(|_| None), write_page: 0, len: 0 }
    }

    fn admit_page(&mut self, page: usize) -> Result<(), &'static [u8]> {
        let Some(owner) = self.pages.get_mut(page) else { return Err(b"fem2d.visual-page-index") };
        if owner.is_some() {
            return Err(b"fem2d.visual-page-duplicate");
        }
        *owner = Some(Canvas2dSnapshotPage::new());
        Ok(())
    }

    fn take_page(&mut self, page: usize) -> Option<Canvas2dSnapshotPage> {
        self.pages.get_mut(page)?.take()
    }

    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let Some(page) = self.pages.iter_mut().find(|page| page.is_some()) else { return (true, 0, 0) };
        if maximum_bytes < FEM2D_MOUNTED_VISUAL_PAGE_BYTES {
            return (false, 0, 0);
        }
        *page = None;
        (self.pages.iter().all(Option::is_none), 1, FEM2D_MOUNTED_VISUAL_PAGE_BYTES)
    }

    fn terminal_is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none)
    }
}

impl Write for Fem2dFixedPacketPages {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        if value.len() > FEM2D_MOUNTED_VISUAL_PAGE_BYTES || self.len.checked_add(value.len()).is_none_or(|len| len > FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES) {
            return Err(std::fmt::Error);
        }
        if self.pages.get(self.write_page).and_then(Option::as_ref).is_none_or(|page| page.remaining() < value.len()) {
            self.write_page += 1;
        }
        self.pages.get_mut(self.write_page).and_then(Option::as_mut).ok_or(std::fmt::Error)?.push(value.as_bytes()).map_err(|_| std::fmt::Error)?;
        self.len += value.len();
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct Fem2dFixedOrder<const N: usize> {
    slots: [Option<usize>; N],
    len: usize,
}

impl<const N: usize> Fem2dFixedOrder<N> {
    fn new() -> Self {
        Self { slots: [None; N], len: 0 }
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

    fn pop(&mut self) -> Option<usize> {
        let index = self.len.checked_sub(1)?;
        self.len = index;
        self.slots[index].take()
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<const N: usize> Default for Fem2dFixedOrder<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct Fem2dMountedVisualLease {
    app_instance_id: u32,
    base_revision: u64,
    generation: u64,
    operation: u64,
    preview_sequence: u64,
    surface_generation: u64,
    renderer_scene_generation: u64,
    snapshot: Canvas2dSnapshotLease,
    close_started: bool,
    region_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_REGIONS>,
    assembly_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_ELEMENTS>,
    field_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_FIELDS>,
}

impl Fem2dMountedVisualLease {
    pub(crate) fn matches(&self, app_instance_id: u32, base_revision: u64, generation: u64) -> bool {
        self.app_instance_id == app_instance_id && self.base_revision == base_revision && self.generation == generation
    }

    pub(crate) fn snapshot(&self) -> Canvas2dSnapshotLease {
        self.snapshot
    }

    pub(crate) fn matches_freshness(&self, freshness: Fem2dVisualFreshness) -> bool {
        self.app_instance_id == freshness.app_instance_id
            && self.base_revision == freshness.model_revision
            && self.generation == freshness.document_generation
            && self.operation == freshness.operation
            && self.preview_sequence == freshness.numerical_preview_sequence
            && self.surface_generation == freshness.surface_generation
            && self.renderer_scene_generation == freshness.renderer_scene_generation
    }

    pub(crate) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if self.region_order.pop().is_some() || self.assembly_order.pop().is_some() || self.field_order.pop().is_some() {
            return (false, 1, std::mem::size_of::<usize>());
        }
        if maximum_bytes < CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY {
            return (false, 0, 0);
        }
        if !self.close_started {
            if canvas2d_snapshot_begin_close(self.snapshot).is_err() {
                return (false, 0, 0);
            }
            self.close_started = true;
            return (false, 1, 0);
        }
        match canvas2d_snapshot_close_step(self.snapshot) {
            Ok(terminal) => (terminal, usize::from(!terminal), usize::from(!terminal) * CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY),
            Err(_) => (false, 0, 0),
        }
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        canvas2d_snapshot_terminal_is_empty(self.snapshot) && self.region_order.is_empty() && self.assembly_order.is_empty() && self.field_order.is_empty()
    }
}

//#region 🧵️MountedVisualJob
const FEM2D_VISUAL_MAXIMUM_REGIONS: usize = 64;
const FEM2D_VISUAL_MAXIMUM_NODES: usize = 16;
const FEM2D_VISUAL_MAXIMUM_ELEMENTS: usize = 16;
const FEM2D_VISUAL_MAXIMUM_SUPPORTS: usize = 64;
const FEM2D_VISUAL_MAXIMUM_LOADS: usize = 64;
const FEM2D_VISUAL_MAXIMUM_FIELDS: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fem2dVisualJobStage {
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

/// 🧵️ Retained live-visual construction; every call advances one scalar, comparison, entry, fragment, page, or control owner.
pub struct Fem2dVisualJob {
    freshness: Fem2dVisualFreshness,
    stage: Fem2dVisualJobStage,
    output: Fem2dFixedPacketPages,
    token: Option<Canvas2dSnapshotWriteToken>,
    abort_started: bool,
    region_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_REGIONS>,
    element_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_ELEMENTS>,
    field_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_FIELDS>,
    reserve_lane: u8,
    scalar_cursor: u8,
    order_input: usize,
    order_slot: usize,
    cursor: usize,
    point_cursor: usize,
    lookup_cursor: usize,
    item_phase: u8,
    load_case_cursor: usize,
    load_cursor: usize,
    quality: RegionVisualQuality,
    endpoint_a: [f64; 2],
    endpoint_b: [f64; 2],
    first: bool,
    page_cursor: usize,
    validated: bool,
    complete: Option<Fem2dMountedVisualLease>,
}

impl Fem2dVisualJob {
    pub fn new(freshness: Fem2dVisualFreshness) -> Self {
        Self {
            freshness,
            stage: Fem2dVisualJobStage::ReserveSnapshot,
            output: Fem2dFixedPacketPages::new(),
            token: None,
            abort_started: false,
            region_order: Fem2dFixedOrder::new(),
            element_order: Fem2dFixedOrder::new(),
            field_order: Fem2dFixedOrder::new(),
            reserve_lane: 0,
            scalar_cursor: 0,
            order_input: 0,
            order_slot: 0,
            cursor: 0,
            point_cursor: 0,
            lookup_cursor: 0,
            item_phase: 0,
            load_case_cursor: 0,
            load_cursor: 0,
            quality: RegionVisualQuality::Unmeshed,
            endpoint_a: [0.0; 2],
            endpoint_b: [0.0; 2],
            first: true,
            page_cursor: 0,
            validated: false,
            complete: None,
        }
    }

    pub fn stage(&self) -> Fem2dVisualJobStage {
        self.stage
    }

    fn layer_prefix(&mut self) -> Result<(), Vec<u8>> {
        let separator = if std::mem::replace(&mut self.first, false) { "" } else { "\n" };
        self.output.write_str(separator).map_err(|_| b"fem2d.visual-output-capacity".to_vec())
    }

    fn advance(&mut self, stage: Fem2dVisualJobStage) {
        self.stage = stage;
        self.cursor = 0;
        self.point_cursor = 0;
        self.lookup_cursor = 0;
        self.item_phase = 0;
    }

    fn load_count_step(doc: &Fem2dSnapshot, case_cursor: usize, total: usize) -> Result<(bool, usize), Vec<u8>> {
        let Some(case) = doc.load_cases.get(case_cursor) else { return Ok((true, total)) };
        let total = total.checked_add(case.loads.len()).ok_or_else(|| b"fem2d.visual-load-count-overflow".to_vec())?;
        Ok((false, total))
    }

    fn order_region_one(&mut self, doc: &Fem2dSnapshot) -> bool {
        if self.order_slot != 0 {
            let Some(left) = self.region_order.get(self.order_slot - 1) else { return false };
            let Some(right) = self.region_order.get(self.order_slot) else { return false };
            if doc.regions[left].id > doc.regions[right].id {
                self.region_order.swap(self.order_slot - 1, self.order_slot);
                self.order_slot -= 1;
            } else {
                self.order_slot = 0;
                self.order_input += 1;
            }
            return false;
        }
        if self.order_input == doc.regions.len() {
            return true;
        }
        if self.region_order.push(self.order_input).is_err() {
            return true;
        }
        self.order_slot = self.region_order.len - 1;
        if self.order_slot == 0 {
            self.order_input += 1;
        }
        false
    }

    fn order_element_one(&mut self, doc: &Fem2dSnapshot) -> bool {
        if self.order_slot != 0 {
            let Some(left) = self.element_order.get(self.order_slot - 1) else { return false };
            let Some(right) = self.element_order.get(self.order_slot) else { return false };
            if element_id(&doc.elements[left]) > element_id(&doc.elements[right]) {
                self.element_order.swap(self.order_slot - 1, self.order_slot);
                self.order_slot -= 1;
            } else {
                self.order_slot = 0;
                self.order_input += 1;
            }
            return false;
        }
        if self.order_input == doc.elements.len() {
            return true;
        }
        if self.element_order.push(self.order_input).is_err() {
            return true;
        }
        self.order_slot = self.element_order.len - 1;
        if self.order_slot == 0 {
            self.order_input += 1;
        }
        false
    }

    fn order_field_one(&mut self, visual: &Fem2dLiveVisual) -> bool {
        if self.order_slot != 0 {
            let Some(left) = self.field_order.get(self.order_slot - 1) else { return false };
            let Some(right) = self.field_order.get(self.order_slot) else { return false };
            if visual.fields[left].node_id > visual.fields[right].node_id {
                self.field_order.swap(self.order_slot - 1, self.order_slot);
                self.order_slot -= 1;
            } else {
                self.order_slot = 0;
                self.order_input += 1;
            }
            return false;
        }
        if self.order_input == visual.fields.len() {
            return true;
        }
        if self.field_order.push(self.order_input).is_err() {
            return true;
        }
        self.order_slot = self.field_order.len - 1;
        if self.order_slot == 0 {
            self.order_input += 1;
        }
        false
    }

    pub fn step_one(&mut self, doc: &Fem2dSnapshot, visual: &Fem2dLiveVisual, freshness: Fem2dVisualFreshness) -> Result<bool, Vec<u8>> {
        match self.stage {
            Fem2dVisualJobStage::ReserveSnapshot => {
                if doc.regions.len() > FEM2D_VISUAL_MAXIMUM_REGIONS
                    || doc.nodes.len() > FEM2D_VISUAL_MAXIMUM_NODES
                    || doc.elements.len() > FEM2D_VISUAL_MAXIMUM_ELEMENTS
                    || doc.supports.len() > FEM2D_VISUAL_MAXIMUM_SUPPORTS
                    || visual.fields.len() > FEM2D_VISUAL_MAXIMUM_FIELDS
                {
                    return Err(b"fem2d.visual-maximum-plus-one".to_vec());
                }
                if self.reserve_lane == 0 {
                    self.token = Some(
                        canvas2d_snapshot_begin(Canvas2dSnapshotDescriptor {
                            revision: self.freshness.model_revision,
                            generation: self.freshness.renderer_scene_generation,
                            page_count: FEM2D_MOUNTED_VISUAL_PAGE_COUNT as u8,
                            byte_count: FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES as u32,
                        })
                        .map_err(|_| b"fem2d.visual-snapshot-preflight".to_vec())?,
                    );
                } else if usize::from(self.reserve_lane) <= FEM2D_MOUNTED_VISUAL_PAGE_COUNT {
                    self.output.admit_page(usize::from(self.reserve_lane) - 1).map_err(<[u8]>::to_vec)?;
                } else {
                    self.stage = Fem2dVisualJobStage::ReadProgressScalar;
                    return Ok(false);
                }
                self.reserve_lane += 1;
            }
            Fem2dVisualJobStage::ReadProgressScalar => {
                match self.scalar_cursor {
                    0 => {
                        let _ = visual.state;
                    }
                    1 => {
                        let _ = visual.progress_completed;
                    }
                    2 => {
                        let _ = visual.progress_total;
                    }
                    3 => {
                        let _ = visual.residual_norm;
                    }
                    4 => {
                        let _ = visual.tolerance;
                    }
                    5 => {
                        let (done, total) = Self::load_count_step(doc, self.load_case_cursor, self.load_cursor)?;
                        self.load_cursor = total;
                        if !done {
                            self.load_case_cursor += 1;
                            return Ok(false);
                        }
                        if total > FEM2D_VISUAL_MAXIMUM_LOADS {
                            return Err(b"fem2d.visual-load-maximum-plus-one".to_vec());
                        }
                    }
                    _ => {
                        self.load_case_cursor = 0;
                        self.load_cursor = 0;
                        self.order_input = 0;
                        self.order_slot = 0;
                        self.stage = Fem2dVisualJobStage::OrderRegionKey;
                        return Ok(false);
                    }
                }
                self.scalar_cursor += 1;
            }
            Fem2dVisualJobStage::OrderRegionKey => {
                if self.order_region_one(doc) {
                    self.advance(Fem2dVisualJobStage::BuildRegion);
                }
            }
            Fem2dVisualJobStage::BuildRegion => {
                let Some(region_index) = self.region_order.get(self.cursor) else {
                    self.order_input = 0;
                    self.order_slot = 0;
                    self.advance(Fem2dVisualJobStage::OrderElementKey);
                    return Ok(false);
                };
                let region = &doc.regions[region_index];
                match self.item_phase {
                    0 => {
                        if let Some((id, quality)) = visual.region_quality.slot(self.lookup_cursor) {
                            if *id == region.id {
                                self.quality = *quality;
                                self.lookup_cursor = 0;
                                self.item_phase = 1;
                            } else {
                                self.lookup_cursor += 1;
                            }
                        } else if self.lookup_cursor == FEM2D_LIVE_REGION_CAPACITY {
                            self.quality = RegionVisualQuality::Unmeshed;
                            self.lookup_cursor = 0;
                            self.item_phase = 1;
                        } else {
                            self.lookup_cursor += 1;
                        }
                    }
                    _ => {
                        if let Some(point) = region.outline.get(self.point_cursor) {
                            let Some(next) = region.outline.get((self.point_cursor + 1) % region.outline.len().max(1)) else {
                                self.cursor += 1;
                                self.point_cursor = 0;
                                self.item_phase = 0;
                                return Ok(false);
                            };
                            let (x0, y0) = screen_2d(point[0], point[1]);
                            let (x1, y1) = screen_2d(next[0], next[1]);
                            self.layer_prefix()?;
                            write!(
                                self.output,
                                "{{\"kind\":\"line\",\"id\":\"region-quality-{}-{}-{}\",\"x0\":{x0},\"y0\":{y0},\"x1\":{x1},\"y1\":{y1},\"color\":\"{}\"}}",
                                self.quality.id(),
                                region.id,
                                self.point_cursor,
                                self.quality.color()
                            )
                            .map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                            self.point_cursor += 1;
                            return Ok(false);
                        }
                        self.cursor += 1;
                        self.point_cursor = 0;
                        self.item_phase = 0;
                    }
                }
            }
            Fem2dVisualJobStage::OrderElementKey => {
                if self.item_phase == 0 {
                    if self.order_element_one(doc) {
                        self.order_input = 0;
                        self.order_slot = 0;
                        self.item_phase = 1;
                    }
                } else if self.order_field_one(visual) {
                    self.advance(Fem2dVisualJobStage::BuildMeshElement);
                }
            }
            Fem2dVisualJobStage::BuildMeshElement => {
                if self.item_phase == 0 {
                    if let Some(node) = doc.nodes.get(self.cursor) {
                        self.layer_prefix()?;
                        let (x, y) = screen_2d(node.x, node.y);
                        write!(self.output, "{{\"kind\":\"circle\",\"id\":\"mesh-node-{}\",\"x\":{},\"y\":{},\"width\":8,\"height\":8,\"color\":\"#38bdf8\"}}", node.id, x - 4.0, y - 4.0).map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                        self.cursor += 1;
                    } else {
                        self.cursor = 0;
                        self.item_phase = 1;
                    }
                    return Ok(false);
                }
                let Some(index) = self.element_order.get(self.cursor) else {
                    self.advance(Fem2dVisualJobStage::BuildAssemblyMark);
                    return Ok(false);
                };
                let element = &doc.elements[index];
                let (start, end) = fem2d_element_endpoints(element);
                if self.item_phase == 1 {
                    let Some(node) = doc.nodes.get(self.lookup_cursor) else { return Err(b"fem2d.visual-element-start".to_vec()) };
                    if node.id == start {
                        self.endpoint_a = [node.x, node.y];
                        self.lookup_cursor = 0;
                        self.item_phase = 2;
                    } else {
                        self.lookup_cursor += 1;
                    }
                    return Ok(false);
                }
                if self.item_phase == 2 {
                    let Some(node) = doc.nodes.get(self.lookup_cursor) else { return Err(b"fem2d.visual-element-end".to_vec()) };
                    if node.id == end {
                        self.endpoint_b = [node.x, node.y];
                        self.lookup_cursor = 0;
                        self.item_phase = 3;
                    } else {
                        self.lookup_cursor += 1;
                    }
                    return Ok(false);
                }
                self.layer_prefix()?;
                let (x0, y0) = screen_2d(self.endpoint_a[0], self.endpoint_a[1]);
                let (x1, y1) = screen_2d(self.endpoint_b[0], self.endpoint_b[1]);
                write!(self.output, "{{\"kind\":\"line\",\"id\":\"mesh-element-{}\",\"x0\":{x0},\"y0\":{y0},\"x1\":{x1},\"y1\":{y1},\"color\":\"#94a3b8\"}}", element_id(element)).map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
                self.item_phase = 1;
            }
            Fem2dVisualJobStage::BuildAssemblyMark => {
                let Some(id) = visual.assembling_element_ids.get(self.cursor) else {
                    self.advance(Fem2dVisualJobStage::BuildLoadGlyph);
                    return Ok(false);
                };
                self.layer_prefix()?;
                write!(self.output, "{{\"kind\":\"text\",\"id\":\"assembling-{id}\",\"x\":10,\"y\":36,\"text\":{{\"content\":\"assembling {id}\",\"size\":11}}}}")
                    .map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
            }
            Fem2dVisualJobStage::BuildLoadGlyph => {
                let Some(case) = doc.load_cases.get(self.load_case_cursor) else {
                    self.advance(Fem2dVisualJobStage::BuildSupportGlyph);
                    return Ok(false);
                };
                let Some(load) = case.loads.get(self.load_cursor) else {
                    self.load_case_cursor += 1;
                    self.load_cursor = 0;
                    return Ok(false);
                };
                let (id, vector) = match load {
                    FemLoad::Nodal { id, dof, value, .. } => (id, if *dof == FemDof::Tx { [value.signum() * 18.0, 0.0] } else { [0.0, value.signum() * 18.0] }),
                    FemLoad::MemberUdl { id, wx, wy, .. } => (id, [wx.signum() * 18.0, wy.signum() * 18.0]),
                    FemLoad::Area { id, pressure, .. } => (id, [0.0, -pressure.signum() * 18.0]),
                };
                self.layer_prefix()?;
                write!(self.output, "{{\"kind\":\"line\",\"id\":\"load-{id}\",\"x0\":20,\"y0\":52,\"x1\":{},\"y1\":{}}}", 20.0 + vector[0], 52.0 - vector[1])
                    .map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.load_cursor += 1;
            }
            Fem2dVisualJobStage::BuildSupportGlyph => {
                let Some(support) = doc.supports.get(self.cursor) else {
                    self.advance(Fem2dVisualJobStage::BuildDisplacementEntry);
                    return Ok(false);
                };
                self.layer_prefix()?;
                write!(self.output, "{{\"kind\":\"circle\",\"id\":\"support-{}\",\"x\":12,\"y\":64,\"width\":10,\"height\":10,\"color\":\"#f97316\"}}", support.id).map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
            }
            Fem2dVisualJobStage::BuildDisplacementEntry | Fem2dVisualJobStage::BuildResidualEntry | Fem2dVisualJobStage::BuildReactionEntry | Fem2dVisualJobStage::BuildContourEntry | Fem2dVisualJobStage::BuildModeEntry => {
                let Some(index) = self.field_order.get(self.cursor) else {
                    let next = match self.stage {
                        Fem2dVisualJobStage::BuildDisplacementEntry => Fem2dVisualJobStage::BuildResidualEntry,
                        Fem2dVisualJobStage::BuildResidualEntry => Fem2dVisualJobStage::BuildReactionEntry,
                        Fem2dVisualJobStage::BuildReactionEntry => Fem2dVisualJobStage::BuildContourEntry,
                        Fem2dVisualJobStage::BuildContourEntry => Fem2dVisualJobStage::BuildModeEntry,
                        _ => Fem2dVisualJobStage::BuildLabelEntry,
                    };
                    self.advance(next);
                    return Ok(false);
                };
                let field = &visual.fields[index];
                self.layer_prefix()?;
                match self.stage {
                    Fem2dVisualJobStage::BuildDisplacementEntry => write!(
                        self.output,
                        "{{\"kind\":\"line\",\"id\":\"displacement-field-{}\",\"x0\":30,\"y0\":80,\"x1\":{},\"y1\":{}}}",
                        field.node_id,
                        30.0 + field.displacement[0] * SCALE_2D,
                        80.0 - field.displacement[1] * SCALE_2D
                    ),
                    Fem2dVisualJobStage::BuildResidualEntry => {
                        write!(self.output, "{{\"kind\":\"line\",\"id\":\"residual-field-{}\",\"x0\":30,\"y0\":96,\"x1\":{},\"y1\":{}}}", field.node_id, 30.0 + field.residual[0], 96.0 - field.residual[1])
                    }
                    Fem2dVisualJobStage::BuildReactionEntry => {
                        write!(self.output, "{{\"kind\":\"line\",\"id\":\"reaction-field-{}\",\"x0\":30,\"y0\":112,\"x1\":{},\"y1\":{}}}", field.node_id, 30.0 + field.reaction[0], 112.0 - field.reaction[1])
                    }
                    Fem2dVisualJobStage::BuildContourEntry => write!(self.output, "{{\"kind\":\"circle\",\"id\":\"contour-field-{}\",\"x\":{},\"y\":124,\"width\":8,\"height\":8,\"color\":\"#22d3ee\"}}", field.node_id, 30.0 + field.contour),
                    _ => write!(self.output, "{{\"kind\":\"line\",\"id\":\"mode-field-{}\",\"x0\":30,\"y0\":140,\"x1\":{},\"y1\":{}}}", field.node_id, 30.0 + field.mode_shape[0], 140.0 - field.mode_shape[1]),
                }
                .map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
            }
            Fem2dVisualJobStage::BuildLabelEntry => {
                let (locale, stable_id, text) = match self.cursor {
                    0 => ("en", "accessible-en", "FEM progress"),
                    1 => ("de", "accessible-de", "FEM-Fortschritt"),
                    2 => ("en", "accessible-controls-en", "Cancel Retry Discard"),
                    3 => ("de", "accessible-controls-de", "Abbrechen Wiederholen Verwerfen"),
                    _ => {
                        self.advance(Fem2dVisualJobStage::SealPages);
                        return Ok(false);
                    }
                };
                self.layer_prefix()?;
                write!(
                    self.output,
                    "{{\"kind\":\"text\",\"id\":\"{stable_id}-{locale}-{}-{}\",\"x\":10,\"y\":18,\"text\":{{\"content\":\"{text}: {}; {}/{}; residual {}; tolerance {}; {}\",\"size\":11}}}}",
                    visual.state.id(),
                    self.cursor,
                    visual.state.id(),
                    visual.progress_completed,
                    visual.progress_total,
                    visual.residual_norm,
                    visual.tolerance,
                    if visual.validated_final { "validated" } else { "provisional" }
                )
                .map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
            }
            Fem2dVisualJobStage::SealPages => {
                if self.page_cursor < FEM2D_MOUNTED_VISUAL_PAGE_COUNT {
                    let mut page = self.output.take_page(self.page_cursor).ok_or_else(|| b"fem2d.visual-seal-page".to_vec())?;
                    page.seal().map_err(|_| b"fem2d.visual-seal-page".to_vec())?;
                    let token = self.token.ok_or_else(|| b"fem2d.visual-page-token".to_vec())?;
                    if let Err(rejected) = canvas2d_snapshot_admit_page(token, page) {
                        self.output.pages[self.page_cursor] = Some(rejected.page);
                        return Err(b"fem2d.visual-page-admission".to_vec());
                    }
                    self.page_cursor += 1;
                } else {
                    self.stage = Fem2dVisualJobStage::ValidateFreshness;
                }
            }
            Fem2dVisualJobStage::ValidateFreshness => {
                if freshness != self.freshness {
                    return Err(b"fem2d.visual-stale-before-publication".to_vec());
                }
                self.validated = true;
                self.stage = Fem2dVisualJobStage::PublishLease;
            }
            Fem2dVisualJobStage::PublishLease => {
                if !self.validated {
                    return Err(b"fem2d.visual-publication-without-freshness".to_vec());
                }
                let token = self.token.take().ok_or_else(|| b"fem2d.visual-page-token".to_vec())?;
                let snapshot = canvas2d_snapshot_seal(token).map_err(|_| b"fem2d.visual-page-seal".to_vec())?;
                self.complete = Some(Fem2dMountedVisualLease {
                    app_instance_id: freshness.app_instance_id,
                    base_revision: freshness.model_revision,
                    generation: freshness.document_generation,
                    operation: freshness.operation,
                    preview_sequence: freshness.numerical_preview_sequence,
                    surface_generation: freshness.surface_generation,
                    renderer_scene_generation: freshness.renderer_scene_generation,
                    snapshot,
                    close_started: false,
                    region_order: std::mem::take(&mut self.region_order),
                    assembly_order: std::mem::take(&mut self.element_order),
                    field_order: std::mem::take(&mut self.field_order),
                });
                self.stage = Fem2dVisualJobStage::RetireDisplacedLease;
            }
            Fem2dVisualJobStage::RetireDisplacedLease => self.stage = Fem2dVisualJobStage::Complete,
            Fem2dVisualJobStage::Complete => return Ok(true),
        }
        Ok(self.stage == Fem2dVisualJobStage::Complete)
    }

    pub fn take_complete(&mut self) -> Option<Fem2dMountedVisualLease> {
        if self.stage == Fem2dVisualJobStage::Complete {
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
        if self.region_order.pop().is_some() || self.element_order.pop().is_some() || self.field_order.pop().is_some() {
            return (false, 1, std::mem::size_of::<usize>());
        }
        if let Some(token) = self.token {
            if !self.abort_started {
                if canvas2d_snapshot_abort_write(token).is_err() {
                    return (false, 0, 0);
                }
                self.abort_started = true;
                return (false, 1, 0);
            }
            match canvas2d_snapshot_abort_write_step(token) {
                Ok(true) => {
                    self.token = None;
                    return (false, 1, 0);
                }
                Ok(false) => return (false, 1, CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY),
                Err(_) => return (false, 0, 0),
            }
        }
        self.output.close_step(maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete.is_none()
            && self.token.is_none_or(canvas2d_snapshot_write_terminal_is_empty)
            && self.output.terminal_is_empty()
            && self.region_order.is_empty()
            && self.element_order.is_empty()
            && self.field_order.is_empty()
    }
}
//#endregion 🧵️MountedVisualJob

fn vector_layer(id: String, origin: (f64, f64), vector: [f64; 2], color: &str) -> serde_json::Value {
    json!({
        "kind": "polyline",
        "id": id,
        "points": [[origin.0, origin.1], [origin.0 + vector[0], origin.1 - vector[1]]],
        "color": color,
    })
}

/// 👁️ Deterministic live overlays for mesh, assembly and iterative solve progress.
pub fn fem2d_live_visual_layers(doc: &Fem2dSnapshot, visual: &Fem2dLiveVisual) -> Vec<serde_json::Value> {
    let mut layers = Vec::new();
    let mut regions: Vec<_> = doc.regions.iter().collect();
    regions.sort_by(|a, b| a.id.cmp(&b.id));
    for region in regions {
        let quality = visual.region_quality.get(&region.id).copied().unwrap_or_default();
        let mut points: Vec<[f64; 2]> = region
            .outline
            .iter()
            .map(|point| {
                let (x, y) = screen_2d(point[0], point[1]);
                [x, y]
            })
            .collect();
        if let Some(first) = points.first().copied() {
            points.push(first);
        }
        layers.push(json!({ "kind": "polyline", "id": format!("region-quality-{}-{}", quality.id(), region.id), "points": points, "color": quality.color() }));
    }
    let mut assembling = visual.assembling_element_ids.clone();
    assembling.sort();
    for id in assembling {
        let Some(element) = doc.elements.iter().find(|element| element_id(element) == id) else { continue };
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(a.x, a.y);
        let (x1, y1) = screen_2d(b.x, b.y);
        layers.push(json!({ "kind": "line", "id": format!("assembling-{id}"), "x0": x0, "y0": y0, "x1": x1, "y1": y1, "color": "#a855f7" }));
    }
    let mut fields = visual.fields.clone()?;
    fields.sort_by(|a, b| a.node_id.cmp(&b.node_id));
    for field in fields {
        let Some(node) = find_node_2d(&doc.nodes, &field.node_id) else { continue };
        let origin = screen_2d(node.x, node.y);
        layers.push(vector_layer(format!("displacement-field-{}", field.node_id), origin, [field.displacement[0] * SCALE_2D, field.displacement[1] * SCALE_2D], "#f472b6"));
        layers.push(vector_layer(format!("residual-field-{}", field.node_id), origin, field.residual, "#eab308"));
    }
    let status = if visual.validated_final {
        "validated-final"
    } else if visual.converged {
        "converged"
    } else {
        "unconverged"
    };
    layers.push(json!({ "id": format!("solve-status-{status}"), "transform": [1.0, 0.0, 0.0, 1.0, 10.0, 18.0], "text": { "content": status, "size": 11.0 } }));
    layers
}
//#endregion 👁️LiveVisualLanguage

//#region 🔖️SharedDrawHelpers
pub(crate) fn screen_2d(x: f64, y: f64) -> (f64, f64) {
    (x * SCALE_2D + ORIGIN_2D, -y * SCALE_2D + ORIGIN_2D)
}

pub(crate) fn find_node_2d<'a>(nodes: &'a [crate::artifacts::fem2d::FemNode], id: &str) -> Option<&'a crate::artifacts::fem2d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

pub(crate) fn fem2d_element_endpoints(element: &FemElement) -> (&str, &str) {
    match element {
        FemElement::Bar { start, end, .. } | FemElement::Beam { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 📐️ Bounding-box diagonal (in model meters) over every node plus every region outline vertex — the
/// reference length `MODE_SHAPE_AMPLITUDE_RATIO` scales a normalized mode shape against. Falls back to
/// `1.0` for a degenerate (empty or point-like) model so mode-shape rendering never divides by zero.
pub(crate) fn fem2d_model_extent(doc: &Fem2dSnapshot) -> f64 {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    let mut expand = |x: f64, y: f64| {
        min[0] = min[0].min(x);
        min[1] = min[1].min(y);
        max[0] = max[0].max(x);
        max[1] = max[1].max(y);
    };
    for node in &doc.nodes {
        expand(node.x, node.y);
    }
    for region in &doc.regions {
        for p in &region.outline {
            expand(p[0], p[1]);
        }
    }
    if min[0] > max[0] {
        return 1.0;
    }
    let d = [max[0] - min[0], max[1] - min[1]];
    (d[0] * d[0] + d[1] * d[1]).sqrt().max(1.0)
}

/// 🖼️ Nodes/members/supports as Canvas2d layers — shared by this window (bright colors) and the results
/// window's faint undeformed backdrop (a single muted color for every layer kind).
pub(crate) fn fem2d_structure_layers(doc: &Fem2dSnapshot, node_color: &str, line_color: &str, support_color: &str) -> Vec<serde_json::Value> {
    let mut layers = Vec::new();
    for node in &doc.nodes {
        let (sx, sy) = screen_2d(node.x, node.y);
        layers.push(json!({ "kind": "circle", "id": format!("node-{}", node.id), "x": sx - 4.0, "y": sy - 4.0, "width": 8.0, "height": 8.0, "color": node_color }));
    }
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        if let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) {
            let (x0, y0) = screen_2d(n1.x, n1.y);
            let (x1, y1) = screen_2d(n2.x, n2.y);
            layers.push(json!({ "kind": "line", "id": format!("el-{}", element_id(element)), "x0": x0, "y0": y0, "x1": x1, "y1": y1, "color": line_color }));
        }
    }
    for support in &doc.supports {
        if let Some(node) = find_node_2d(&doc.nodes, &support.node_id) {
            let (sx, sy) = screen_2d(node.x, node.y);
            layers.push(json!({ "kind": "circle", "id": format!("support-{}", support.id), "x": sx - 5.0, "y": sy - 5.0, "width": 10.0, "height": 10.0, "color": support_color }));
        }
    }
    for case in &doc.load_cases {
        for load in &case.loads {
            match load {
                FemLoad::Nodal { id, node_id, dof, value } => {
                    let Some(node) = find_node_2d(&doc.nodes, node_id) else { continue };
                    let vector = match dof {
                        FemDof::Tx => [value.signum() * 18.0, 0.0],
                        FemDof::Ty => [0.0, value.signum() * 18.0],
                        _ => [0.0, -12.0],
                    };
                    layers.push(vector_layer(format!("load-{id}"), screen_2d(node.x, node.y), vector, "#ef4444"));
                }
                FemLoad::MemberUdl { id, element_id: target, wx, wy } => {
                    let Some(element) = doc.elements.iter().find(|element| element_id(element) == target) else { continue };
                    let (start, end) = fem2d_element_endpoints(element);
                    let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
                    layers.push(vector_layer(format!("load-{id}"), screen_2d((a.x + b.x) * 0.5, (a.y + b.y) * 0.5), [wx.signum() * 18.0, wy.signum() * 18.0], "#ef4444"));
                }
                FemLoad::Area { id, region_id, pressure } => {
                    let Some(region) = doc.regions.iter().find(|region| region.id == *region_id) else { continue };
                    if region.outline.is_empty() {
                        continue;
                    }
                    let center = region.outline.iter().fold([0.0, 0.0], |sum, point| [sum[0] + point[0], sum[1] + point[1]]);
                    let count = region.outline.len() as f64;
                    layers.push(vector_layer(format!("load-{id}"), screen_2d(center[0] / count, center[1] / count), [0.0, -pressure.signum() * 18.0], "#ef4444"));
                }
            }
        }
    }
    layers
}

/// 🗺️ Every meshed region's triangles as `(element_id, [screen_p0, screen_p1, screen_p2])` — the
/// element id matches `fem2d_solve`/`fem2d_solve_all`'s `Tri3Cst` ids (`"{region_id}_t{tri_index}"`),
/// so callers can correlate a solved `ElementResult::Plane` back to on-screen triangle geometry. A
/// mesh failure for one region silently yields fewer triangles rather than failing the whole render.
pub(crate) fn fem2d_region_triangles(doc: &Fem2dSnapshot) -> Vec<(String, [(f64, f64); 3])> {
    let mut out = Vec::new();
    let Ok(meshes) = crate::fem2d_engine::mesh_preview::fem2d_mesh_preview(doc) else { return out };
    for mesh in &meshes {
        for (tri_index, tri) in mesh.tris.iter().enumerate() {
            let id = format!("{}_t{}", mesh.region_id, tri_index);
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            out.push((id, [screen_2d(p0[0], p0[1]), screen_2d(p1[0], p1[1]), screen_2d(p2[0], p2[1])]));
        }
    }
    out
}

/// 🗺️ Every meshed region's triangles as `(element_id, screen points, node ids)` — like
/// `fem2d_region_triangles` but also carrying each vertex's mesh node id, needed to look values up in
/// `fem2d_nodal_von_mises`'s node-keyed map for banded contour rendering.
pub(crate) fn fem2d_region_mesh_triangles(doc: &Fem2dSnapshot) -> Vec<(String, [(f64, f64); 3], [String; 3])> {
    let mut out = Vec::new();
    let Ok(meshes) = crate::fem2d_engine::mesh_preview::fem2d_mesh_preview(doc) else { return out };
    for mesh in &meshes {
        for (tri_index, tri) in mesh.tris.iter().enumerate() {
            let id = format!("{}_t{}", mesh.region_id, tri_index);
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            let node_ids = [mesh.node_ids[tri[0] as usize].clone(), mesh.node_ids[tri[1] as usize].clone(), mesh.node_ids[tri[2] as usize].clone()];
            out.push((id, [screen_2d(p0[0], p0[1]), screen_2d(p1[0], p1[1]), screen_2d(p2[0], p2[1])], node_ids));
        }
    }
    out
}

/// 🖼️ Every element's deformed-shape polyline (pink), given a node-id-keyed displacement map and a
/// display scale — shared by the static, modal, and buckling results renders.
pub(crate) fn fem2d_deformed_shape_layers(doc: &Fem2dSnapshot, disp_map: &HashMap<String, [f64; 6]>, deform_scale: f64) -> Vec<serde_json::Value> {
    let mut layers = Vec::new();
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(n1.x, n1.y);
        let (x1, y1) = screen_2d(n2.x, n2.y);
        let d1 = disp_map.get(&n1.id).copied().unwrap_or([0.0; 6]);
        let d2 = disp_map.get(&n2.id).copied().unwrap_or([0.0; 6]);
        let dx0 = d1[Dof::Tx.index()] * deform_scale * SCALE_2D;
        let dy0 = -d1[Dof::Ty.index()] * deform_scale * SCALE_2D;
        let dx1 = d2[Dof::Tx.index()] * deform_scale * SCALE_2D;
        let dy1 = -d2[Dof::Ty.index()] * deform_scale * SCALE_2D;
        layers.push(json!({
            "kind": "polyline",
            "id": format!("deformed-{}", element_id(element)),
            "points": [[x0 + dx0, y0 + dy0], [x1 + dx1, y1 + dy1]],
            "color": "#f472b6",
        }));
    }
    layers
}
//#endregion 🔖️SharedDrawHelpers

//#region 🔖️Render
pub fn render(doc: &Fem2dSnapshot, camera: &FemCamera) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut layers = fem2d_structure_layers(doc, "#38bdf8", "#94a3b8", "#f97316");
    for (tri_index, (_, tri)) in fem2d_region_triangles(doc).iter().enumerate() {
        let [(x0, y0), (x1, y1), (x2, y2)] = *tri;
        layers.push(json!({
            "kind": "polyline",
            "id": format!("mesh-edge-{tri_index}"),
            "points": [[x0, y0], [x1, y1], [x1, y1], [x2, y2], [x2, y2], [x0, y0]],
            "color": MESH_EDGE_COLOR,
        }));
    }
    let layers_json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into());
    crate::app_surface::canvas_2d_surface(BODY_KEY, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: None })
}

/// 👁️ Renders the model plus an optional replaceable worker-job progress snapshot.
pub fn render_with_progress(_doc: &Fem2dSnapshot, camera: &FemCamera, progress: Option<&Fem2dMountedVisualLease>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::canvas_2d_surface(
        BODY_KEY,
        Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json: String::new(), snapshot: progress.map(Fem2dMountedVisualLease::snapshot) },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::testkit::{fem2d_app, render as render_body};

    fn visual_freshness(generation: u64) -> Fem2dVisualFreshness {
        Fem2dVisualFreshness { app_instance_id: 7, model_revision: 11, document_generation: generation, operation: 13, numerical_preview_sequence: 17, surface_generation: generation, renderer_scene_generation: generation }
    }

    fn sealed_visual(doc: &Fem2dSnapshot, visual: &Fem2dLiveVisual) -> Fem2dMountedVisualLease {
        let mut job = Fem2dVisualJob::new(visual_freshness(19));
        let mut turns = 0;
        while !job.step_one(doc, visual, visual_freshness(19)).expect("bounded production step") && turns < 2_048 {
            turns += 1;
        }
        job.take_complete().expect("sealed visual")
    }

    fn packet_contains(lease: &Fem2dMountedVisualLease, needle: &[u8]) -> bool {
        (0..lease.snapshot.page_count).any(|page| {
            canvas2d_snapshot_with_page(lease.snapshot, page, |owner| owner.bytes().windows(needle.len()).any(|window| window == needle)).unwrap_or(false)
        })
    }

    fn packet_equal(left: &Fem2dMountedVisualLease, right: &Fem2dMountedVisualLease) -> bool {
        left.snapshot.page_count == right.snapshot.page_count
            && (0..left.snapshot.page_count).all(|page| {
                let left_hash = canvas2d_snapshot_with_page(left.snapshot, page, |owner| owner.bytes().iter().fold((0xcbf2_9ce4_8422_2325_u64, 0_usize), |(hash, len), byte| ((hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3), len + 1))).ok();
                let right_hash = canvas2d_snapshot_with_page(right.snapshot, page, |owner| owner.bytes().iter().fold((0xcbf2_9ce4_8422_2325_u64, 0_usize), |(hash, len), byte| ((hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3), len + 1))).ok();
                left_hash == right_hash
            })
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_fem2d_model_scene() {
        let mut app = fem2d_app();
        assert!(render_body(&mut app, BODY_KEY).contains("canvas-2d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_preview_renders_region_edges() {
        let mut app = fem2d_app();
        crate::editor::fem2d::testkit::dispatch(&mut app, crate::editor::fem2d::Fem2dCommand::SetActiveExample(crate::editor::fem2d::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
        let snapshot = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let node = render(&snapshot, &FemCamera::default());
        let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
        let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("decode canvas scene");
        assert!(scene.layers_json.contains("mesh-edge-"), "expected mesh-edge preview layers in the model scene");
    }

    #[semio_framework_async_macros::async_test]
    async fn fem2d_model_extent_degenerate_model_returns_one() {
        assert_eq!(fem2d_model_extent(&crate::artifacts::fem2d::schema::empty_fem2d_snapshot()), 1.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn live_visual_language_distinguishes_every_progress_state() {
        use store::ArtifactDsl;
        let doc = Fem2dSnapshot::parse_dsl(crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT).expect("parse example");
        let region_id = doc.regions.first().expect("example region").id.clone();
        let element_id = element_id(doc.elements.first().expect("example element")).to_string();
        let node_id = doc.nodes.first().expect("example node").id.clone();
        for quality in [RegionVisualQuality::Unmeshed, RegionVisualQuality::Coarse, RegionVisualQuality::Refined, RegionVisualQuality::Final] {
            let visual = Fem2dLiveVisual {
                region_quality: [(region_id.clone(), quality)].into_iter().collect(),
                assembling_element_ids: vec![element_id.clone()],
                fields: vec![NodeLiveField { node_id: node_id.clone(), displacement: [0.01, -0.02], residual: [3.0, -4.0], reaction: [-3.0, 4.0], contour: 5.0, mode_shape: [0.2, -0.1] }],
                state: if quality == RegionVisualQuality::Final { FemVisualState::ValidatedFinal } else { FemVisualState::Coarse },
                residual_norm: 5.0,
                tolerance: 1e-8,
                progress_completed: 1,
                progress_total: 2,
                converged: quality == RegionVisualQuality::Final,
                validated_final: quality == RegionVisualQuality::Final,
            };
            let encoded = serde_json::to_string(&fem2d_live_visual_layers(&doc, &visual)).expect("visual serializes");
            assert!(encoded.contains(&format!("region-quality-{}", quality.id())));
            assert!(encoded.contains("assembling-"));
            assert!(encoded.contains("displacement-field-"));
            assert!(encoded.contains("residual-field-"));
            assert!(encoded.contains(if quality == RegionVisualQuality::Final { "solve-status-validated-final" } else { "solve-status-unconverged" }));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn model_visual_language_includes_load_and_support_glyphs() {
        use store::ArtifactDsl;
        let doc = Fem2dSnapshot::parse_dsl(crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT).expect("parse example");
        let encoded = serde_json::to_string(&fem2d_structure_layers(&doc, "#38bdf8", "#94a3b8", "#f97316")).expect("visual serializes");
        assert!(encoded.contains("support-"));
        assert!(encoded.contains("load-"));
    }

    #[semio_framework_async_macros::async_test]
    async fn live_visual_replay_is_deterministic_and_bounded() {
        use std::time::Instant;
        use store::ArtifactDsl;

        let doc = Fem2dSnapshot::parse_dsl(crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT).expect("parse example");
        let node_id = doc.nodes.first().expect("example node").id.clone();
        let visual = Fem2dLiveVisual {
            region_quality: doc.regions.iter().rev().enumerate().map(|(index, region)| (region.id.clone(), if index % 2 == 0 { RegionVisualQuality::Coarse } else { RegionVisualQuality::Refined })).collect(),
            assembling_element_ids: doc.elements.iter().rev().map(|element| element_id(element).to_string()).collect(),
            fields: (0..256)
                .rev()
                .map(|index| NodeLiveField {
                    node_id: node_id.clone(),
                    displacement: [index as f64 * 1e-6, -1e-4],
                    residual: [index as f64 * 1e-3, -0.5],
                    reaction: [0.5, -(index as f64) * 1e-3],
                    contour: index as f64,
                    mode_shape: [index as f64 * 1e-4, -1e-4],
                })
                .collect(),
            state: FemVisualState::SolvingUnconverged,
            residual_norm: 0.5,
            tolerance: 1e-8,
            progress_completed: 128,
            progress_total: 256,
            converged: false,
            validated_final: false,
        };
        let started = Instant::now();
        let first = fem2d_live_visual_layers(&doc, &visual);
        let elapsed = started.elapsed();
        let second = fem2d_live_visual_layers(&doc, &visual);
        assert_eq!(first, second, "the same accepted preview must replay byte-stably");
        assert!(elapsed.as_micros() < 8_000, "live overlay step took {} us", elapsed.as_micros());
    }

    #[test]
    fn mounted_visual_output_exact_maximum_plus_one_and_page_handback() {
        let mut output = Fem2dFixedPacketPages::new();
        output.admit_page(0).expect("page zero");
        output.admit_page(1).expect("page one");
        output.admit_page(2).expect("page two");
        output.admit_page(3).expect("page three");
        let exact = "x".repeat(FEM2D_MOUNTED_VISUAL_PAGE_BYTES);
        output.write_str(&exact).expect("page zero exact");
        output.write_str(&exact).expect("page one exact");
        output.write_str(&exact).expect("page two exact");
        output.write_str(&exact).expect("page three exact");
        let before = output.pages[0].as_ref().expect("page retained").backing_identity();
        assert!(output.write_char('x').is_err(), "maximum plus one must reject before allocating");
        assert_eq!(output.pages[0].as_ref().expect("same page").backing_identity(), before);

        let mut released = 0;
        while !output.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0 {
            released += 1;
        }
        assert_eq!(released + 1, FEM2D_MOUNTED_VISUAL_PAGE_COUNT);
        assert!(output.terminal_is_empty());
    }

    #[test]
    fn fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer() {
        let mut doc = Fem2dSnapshot::default();
        doc.nodes.resize_with(FEM2D_VISUAL_MAXIMUM_NODES + 1, || crate::artifacts::fem2d::FemNode { id: "n".into(), x: 0.0, y: 0.0 });
        let before = doc.nodes.as_ptr();
        let mut job = Fem2dVisualJob::new(visual_freshness(19));
        assert!(job.step_one(&doc, &Fem2dLiveVisual::default(), visual_freshness(19)).is_err());
        assert_eq!(before, doc.nodes.as_ptr());
        assert_eq!(job.stage(), Fem2dVisualJobStage::ReserveSnapshot);
    }

    #[test]
    fn fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid() {
        let doc = Fem2dSnapshot::default();
        let visual = Fem2dLiveVisual::default();
        let mut stale = Fem2dVisualJob::new(visual_freshness(19));
        let mut turns = 0;
        while stale.stage() != Fem2dVisualJobStage::ValidateFreshness && turns < 512 {
            stale.step_one(&doc, &visual, visual_freshness(19)).expect("bounded production step");
            turns += 1;
        }
        assert!(stale.step_one(&doc, &visual, visual_freshness(23)).is_err());
        assert!(stale.take_complete().is_none());
        while !stale.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0 && turns < 2_048 {
            turns += 1;
        }
        assert!(stale.terminal_is_empty());

        let current = sealed_visual(&doc, &visual);
        let current_snapshot = current.snapshot();
        let mut rejected = Fem2dVisualJob::new(visual_freshness(29));
        assert!(!rejected.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0);
        assert!(canvas2d_snapshot_with_page(current_snapshot, 0, |_| ()).is_ok());
    }

    #[test]
    fn fem2d_visual_job_replay_accessibility_and_each_step_are_bounded() {
        let doc = Fem2dSnapshot::default();
        let visual = Fem2dLiveVisual { state: FemVisualState::ValidatedFinal, validated_final: true, progress_completed: 1, progress_total: 1, tolerance: 1e-8, ..Default::default() };
        let first = sealed_visual(&doc, &visual);
        let second = sealed_visual(&doc, &visual);
        assert!(packet_equal(&first, &second));
        assert!(packet_contains(&first, b"accessible-en"));
        assert!(packet_contains(&first, b"accessible-de"));
        assert!(packet_contains(&first, b"Cancel Retry Discard"));
        assert!(packet_contains(&first, "Abbrechen Wiederholen Verwerfen".as_bytes()));

        let mut job = Fem2dVisualJob::new(visual_freshness(19));
        let started = std::time::Instant::now();
        let _ = job.step_one(&doc, &visual, visual_freshness(19));
        assert!(started.elapsed().as_micros() < 8_000);
    }
}
//#endregion 🧪️Tests
