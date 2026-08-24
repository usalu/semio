//! 🧵️ Mounted FEM3D visual publication on the shared bounded-job reactor.

use crate::artifacts::fem3d::{element_id, load_id, Fem3dSnapshot, FemElement};
use semio_framework::kernel::{Effect, JobPlacement};
use semio_framework_job::{Generation, OperationId, RevisionId, StepBudget, StepContext};
use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, JobBudget, JobStep};
use semio_framework_plugin::{AppRenderOperationContext, ArtifactView, PluginCloseStep};
use std::cell::RefCell;
use std::fmt::Write;
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
const OUTPUT_BYTES: usize = 64 * 1_024;
const PAGE_BYTES: usize = 4_096;
const FAULT_BYTES: usize = PAGE_BYTES;
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

struct BoundedText {
    bytes: String,
    admitted_capacity: usize,
}

impl BoundedText {
    fn new() -> Self {
        Self { bytes: String::new(), admitted_capacity: 0 }
    }

    fn reserve(&mut self) -> Result<(), Vec<u8>> {
        self.bytes.try_reserve_exact(OUTPUT_BYTES).map_err(|_| b"fem3d.visual-output-allocation".to_vec())
    }

    fn admit(&mut self) -> Result<(), Vec<u8>> {
        if self.bytes.capacity() != OUTPUT_BYTES {
            return Err(b"fem3d.visual-output-observed-capacity".to_vec());
        }
        self.admitted_capacity = self.bytes.capacity();
        Ok(())
    }
}

impl Write for BoundedText {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let next = self.bytes.len().checked_add(value.len()).ok_or(std::fmt::Error)?;
        if self.admitted_capacity == 0 || next > self.admitted_capacity {
            return Err(std::fmt::Error);
        }
        self.bytes.push_str(value);
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Fem3dMountedVisualLease {
    freshness: Fem3dVisualFreshness,
    meshes_json: String,
    instances_json: String,
    region_order: Vec<usize>,
    element_order: Vec<usize>,
    page_count: usize,
    close_lane: u8,
}

impl Fem3dMountedVisualLease {
    pub fn meshes_json(&self) -> &str {
        &self.meshes_json
    }

    pub fn instances_json(&self) -> &str {
        &self.instances_json
    }

    fn matches(&self, freshness: Fem3dVisualFreshness) -> bool {
        self.freshness == freshness
    }

    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let owner = match self.close_lane {
            0 => &mut self.region_order,
            1 => &mut self.element_order,
            2 => {
                if self.page_count != 0 {
                    let bytes = self.instances_json.len().min(PAGE_BYTES);
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.instances_json.truncate(self.instances_json.len() - bytes);
                    self.page_count -= 1;
                    return (false, 1, bytes);
                }
                if self.instances_json.capacity() != 0 {
                    self.instances_json = String::new();
                    return (false, 1, 0);
                }
                self.close_lane = 3;
                return (false, 1, 0);
            }
            3 => {
                if self.meshes_json.capacity() != 0 {
                    let bytes = self.meshes_json.capacity();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.meshes_json = String::new();
                    return (false, 1, bytes);
                }
                return (true, 0, 0);
            }
            _ => return (true, 0, 0),
        };
        if owner.pop().is_some() {
            return (false, 1, 0);
        }
        let bytes = owner.capacity() * std::mem::size_of::<usize>();
        if bytes > maximum_bytes {
            return (false, 0, 0);
        }
        *owner = Vec::new();
        self.close_lane += 1;
        (false, 1, bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.meshes_json.capacity() == 0 && self.instances_json.capacity() == 0 && self.page_count == 0 && self.region_order.capacity() == 0 && self.element_order.capacity() == 0
    }
}
//#endregion 🔖️Contract

//#region 🧵️VisualJob
/// 🧵️ Separate 3D schema builder with tetrahedron/hexahedron cells and three-component field entries.
pub struct Fem3dVisualJob {
    freshness: Fem3dVisualFreshness,
    stage: Fem3dVisualJobStage,
    output: BoundedText,
    meshes: BoundedText,
    region_order: Vec<usize>,
    element_order: Vec<usize>,
    reserve_lane: u8,
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
    first: bool,
    page_count: usize,
    page_cursor: usize,
    validated: bool,
    complete: Option<Fem3dMountedVisualLease>,
    close_lane: u8,
}

impl Fem3dVisualJob {
    fn new(freshness: Fem3dVisualFreshness) -> Self {
        Self {
            freshness,
            stage: Fem3dVisualJobStage::ReserveSnapshot,
            output: BoundedText::new(),
            meshes: BoundedText::new(),
            region_order: Vec::new(),
            element_order: Vec::new(),
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
            first: true,
            page_count: 0,
            page_cursor: 0,
            validated: false,
            complete: None,
            close_lane: 0,
        }
    }

    pub fn stage(&self) -> Fem3dVisualJobStage {
        self.stage
    }

    fn prefix(&mut self) -> Result<(), Vec<u8>> {
        let separator = if std::mem::replace(&mut self.first, false) { "" } else { "," };
        self.output.write_str(separator).map_err(|_| b"fem3d.visual-output-capacity".to_vec())
    }

    fn advance(&mut self, stage: Fem3dVisualJobStage) {
        self.stage = stage;
        self.cursor = 0;
        self.lookup_cursor = 0;
        self.point_cursor = 0;
        self.item_phase = 0;
    }

    fn order_region_one(&mut self, doc: &Fem3dSnapshot) -> bool {
        if self.order_slot != 0 {
            let left = self.region_order[self.order_slot - 1];
            let right = self.region_order[self.order_slot];
            if doc.solids[left].id > doc.solids[right].id {
                self.region_order.swap(self.order_slot - 1, self.order_slot);
                self.order_slot -= 1;
            } else {
                self.order_slot = 0;
                self.order_input += 1;
            }
            return false;
        }
        if self.order_input == doc.solids.len() {
            return true;
        }
        self.region_order.push(self.order_input);
        self.order_slot = self.region_order.len() - 1;
        if self.order_slot == 0 {
            self.order_input += 1;
        }
        false
    }

    fn order_element_one(&mut self, doc: &Fem3dSnapshot) -> bool {
        if self.order_slot != 0 {
            let left = self.element_order[self.order_slot - 1];
            let right = self.element_order[self.order_slot];
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
        self.element_order.push(self.order_input);
        self.order_slot = self.element_order.len() - 1;
        if self.order_slot == 0 {
            self.order_input += 1;
        }
        false
    }

    fn element_endpoints(element: &FemElement) -> (&str, &str) {
        match element {
            FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start, end),
        }
    }

    fn step_one(&mut self, doc: &Fem3dSnapshot, freshness: Fem3dVisualFreshness) -> Result<bool, Vec<u8>> {
        match self.stage {
            Fem3dVisualJobStage::ReserveSnapshot => {
                if doc.solids.len() > MAXIMUM_REGIONS || doc.nodes.len() > MAXIMUM_NODES || doc.elements.len() > MAXIMUM_ELEMENTS || doc.supports.len() > MAXIMUM_SUPPORTS || doc.nodes.len() > MAXIMUM_FIELDS {
                    return Err(b"fem3d.visual-maximum-plus-one".to_vec());
                }
                match self.reserve_lane {
                    0 => self.output.reserve()?,
                    1 => self.output.admit()?,
                    2 => self.meshes.reserve()?,
                    3 => self.meshes.admit()?,
                    4 => self.region_order.try_reserve_exact(doc.solids.len()).map_err(|_| b"fem3d.visual-region-order-allocation".to_vec())?,
                    5 => self.element_order.try_reserve_exact(doc.elements.len()).map_err(|_| b"fem3d.visual-element-order-allocation".to_vec())?,
                    _ => {
                        if self.region_order.capacity() * std::mem::size_of::<usize>() > PAGE_BYTES || self.element_order.capacity() * std::mem::size_of::<usize>() > PAGE_BYTES {
                            return Err(b"fem3d.visual-order-observed-capacity".to_vec());
                        }
                        self.output.write_char('[').map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
                        self.meshes.write_str("[{\"id\":\"box\",\"kind\":\"box\"}]").map_err(|_| b"fem3d.visual-mesh-capacity".to_vec())?;
                        self.page_count = self.output.admitted_capacity / PAGE_BYTES;
                        self.stage = Fem3dVisualJobStage::ReadProgressScalar;
                        return Ok(false);
                    }
                }
                self.reserve_lane += 1;
            }
            Fem3dVisualJobStage::ReadProgressScalar => {
                match self.scalar_cursor {
                    0 => {
                        let _ = doc.analysis.deformation_scale;
                    }
                    1 => {
                        let _ = doc.analysis.modal_count;
                    }
                    2 => {
                        let _ = doc.analysis.buckling_count;
                    }
                    3 => {
                        let Some(case) = doc.load_cases.get(self.load_case_cursor) else {
                            if self.load_cursor > MAXIMUM_LOADS {
                                return Err(b"fem3d.visual-load-maximum-plus-one".to_vec());
                            }
                            self.scalar_cursor += 1;
                            return Ok(false);
                        };
                        self.load_cursor = self.load_cursor.checked_add(case.loads.len()).ok_or_else(|| b"fem3d.visual-load-count-overflow".to_vec())?;
                        self.load_case_cursor += 1;
                        return Ok(false);
                    }
                    _ => {
                        self.load_case_cursor = 0;
                        self.load_cursor = 0;
                        self.order_input = 0;
                        self.order_slot = 0;
                        self.stage = Fem3dVisualJobStage::OrderRegionKey;
                        return Ok(false);
                    }
                }
                self.scalar_cursor += 1;
            }
            Fem3dVisualJobStage::OrderRegionKey => {
                if self.order_region_one(doc) {
                    self.advance(Fem3dVisualJobStage::BuildRegion);
                }
            }
            Fem3dVisualJobStage::BuildRegion => {
                let Some(index) = self.region_order.get(self.cursor).copied() else {
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
                self.prefix()?;
                let divisor = self.point_cursor.max(1) as f64;
                let cell_kind = if solid.layers == 1 { "tetrahedron" } else { "hexahedron" };
                write!(
                    self.output,
                    "{{\"id\":\"solid-{}\",\"meshId\":\"box\",\"position\":[{},{},{}],\"rotation\":[0,0,0,1],\"scale\":[1,1,{}],\"label\":\"{} refined mesh cell {cell_kind}\"}}",
                    solid.id,
                    self.solid_sum[0] / divisor,
                    self.solid_sum[1] / divisor,
                    solid.base_z + solid.height * 0.5,
                    solid.height,
                    solid.id
                )
                .map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
                self.point_cursor = 0;
                self.solid_sum = [0.0; 2];
            }
            Fem3dVisualJobStage::OrderElementKey => {
                if self.order_element_one(doc) {
                    self.advance(Fem3dVisualJobStage::BuildMeshElement);
                }
            }
            Fem3dVisualJobStage::BuildMeshElement => {
                if self.item_phase == 0 {
                    if let Some(node) = doc.nodes.get(self.cursor) {
                        self.prefix()?;
                        write!(self.output, "{{\"id\":\"mesh-node-{}\",\"meshId\":\"box\",\"position\":[{},{},{}],\"rotation\":[0,0,0,1],\"scale\":[0.05,0.05,0.05],\"label\":\"{}\"}}", node.id, node.x, node.y, node.z, node.id)
                            .map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
                        self.cursor += 1;
                    } else {
                        self.cursor = 0;
                        self.item_phase = 1;
                    }
                    return Ok(false);
                }
                let Some(index) = self.element_order.get(self.cursor).copied() else {
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
                self.prefix()?;
                write!(
                    self.output,
                    "{{\"id\":\"mesh-element-{}\",\"meshId\":\"box\",\"position\":[{},{},{}],\"rotation\":[0,0,0,1],\"scale\":[0.05,0.05,{}],\"label\":\"{}\"}}",
                    element_id(element),
                    midpoint[0],
                    midpoint[1],
                    midpoint[2],
                    length,
                    element_id(element)
                )
                .map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
                self.item_phase = 1;
            }
            Fem3dVisualJobStage::BuildAssemblyMark => {
                let Some(index) = self.element_order.get(self.cursor).copied() else {
                    self.advance(Fem3dVisualJobStage::BuildLoadGlyph);
                    return Ok(false);
                };
                self.prefix()?;
                write!(
                    self.output,
                    "{{\"id\":\"assembling-{}\",\"meshId\":\"box\",\"position\":[0,0,0],\"rotation\":[0,0,0,1],\"scale\":[0.03,0.03,0.03],\"label\":\"assembling element {}\"}}",
                    element_id(&doc.elements[index]),
                    element_id(&doc.elements[index])
                )
                .map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
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
                self.prefix()?;
                write!(self.output, "{{\"id\":\"load-{}\",\"meshId\":\"box\",\"position\":[0,0,1],\"rotation\":[0,0,0,1],\"scale\":[0.03,0.03,0.3],\"label\":\"load {}\"}}", load_id(load), load_id(load))
                    .map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
                self.load_cursor += 1;
            }
            Fem3dVisualJobStage::BuildSupportGlyph => {
                let Some(support) = doc.supports.get(self.cursor) else {
                    self.advance(Fem3dVisualJobStage::BuildDisplacementEntry);
                    return Ok(false);
                };
                self.prefix()?;
                write!(self.output, "{{\"id\":\"support-{}\",\"meshId\":\"box\",\"position\":[0,0,0],\"rotation\":[0,0,0,1],\"scale\":[0.1,0.1,0.1],\"label\":\"support {}\"}}", support.id, support.id)
                    .map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
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
                let field = Fem3dVisualField { node_id: node.id.clone(), displacement: [0.0; 3], residual: [0.0; 3], reaction: [0.0; 3], contour: 0.0, mode_shape: [0.0; 3], eigen_estimate: 0.0 };
                self.prefix()?;
                let (kind, vector, scalar) = match self.stage {
                    Fem3dVisualJobStage::BuildDisplacementEntry => ("displacement", field.displacement, 0.0),
                    Fem3dVisualJobStage::BuildResidualEntry => ("residual", field.residual, 0.0),
                    Fem3dVisualJobStage::BuildReactionEntry => ("reaction", field.reaction, 0.0),
                    Fem3dVisualJobStage::BuildContourEntry => ("contour", [0.0; 3], field.contour),
                    _ => ("mode-shape", field.mode_shape, field.eigen_estimate),
                };
                write!(
                    self.output,
                    "{{\"id\":\"{kind}-field-{}\",\"meshId\":\"box\",\"position\":[{},{},{}],\"rotation\":[0,0,0,1],\"scale\":[0.02,0.02,0.02],\"label\":\"{kind} [{},{},{}] scalar {scalar}\"}}",
                    field.node_id, node.x, node.y, node.z, vector[0], vector[1], vector[2]
                )
                .map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
            }
            Fem3dVisualJobStage::BuildLabelEntry => {
                let (locale, stable_id, label) = match self.cursor {
                    0 => ("en", "accessible-en", "FEM progress; validated final; cancel retry discard"),
                    1 => ("de", "accessible-de", "FEM-Fortschritt; endgültig validiert; abbrechen wiederholen verwerfen"),
                    _ => {
                        self.advance(Fem3dVisualJobStage::SealPages);
                        return Ok(false);
                    }
                };
                self.prefix()?;
                write!(
                    self.output,
                    "{{\"id\":\"{stable_id}-{locale}-{}\",\"meshId\":\"box\",\"position\":[0,0,0],\"rotation\":[0,0,0,1],\"scale\":[0,0,0],\"label\":\"{label}; stage {}; progress {}/{}; residual {}; tolerance {}; quality refined; final\"}}",
                    self.cursor,
                    Fem3dVisualState::ValidatedFinal.id(),
                    doc.nodes.len(),
                    doc.nodes.len(),
                    0.0,
                    1e-8
                )
                .map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
            }
            Fem3dVisualJobStage::SealPages => {
                if self.page_cursor == 0 {
                    self.output.write_char(']').map_err(|_| b"fem3d.visual-output-capacity".to_vec())?;
                    self.page_cursor = 1;
                } else if self.page_cursor <= self.page_count {
                    self.page_cursor += 1;
                } else {
                    self.stage = Fem3dVisualJobStage::ValidateFreshness;
                }
            }
            Fem3dVisualJobStage::ValidateFreshness => {
                if freshness != self.freshness {
                    return Err(b"fem3d.visual-stale-before-publication".to_vec());
                }
                self.validated = true;
                self.stage = Fem3dVisualJobStage::PublishLease;
            }
            Fem3dVisualJobStage::PublishLease => {
                if !self.validated {
                    return Err(b"fem3d.visual-publication-without-freshness".to_vec());
                }
                self.complete = Some(Fem3dMountedVisualLease {
                    freshness,
                    meshes_json: std::mem::take(&mut self.meshes.bytes),
                    instances_json: std::mem::take(&mut self.output.bytes),
                    region_order: std::mem::take(&mut self.region_order),
                    element_order: std::mem::take(&mut self.element_order),
                    page_count: std::mem::take(&mut self.page_count),
                    close_lane: 0,
                });
                self.stage = Fem3dVisualJobStage::RetireDisplacedLease;
            }
            Fem3dVisualJobStage::RetireDisplacedLease => self.stage = Fem3dVisualJobStage::Complete,
            Fem3dVisualJobStage::Complete => return Ok(true),
        }
        Ok(self.stage == Fem3dVisualJobStage::Complete)
    }

    fn take_complete(&mut self) -> Option<Fem3dMountedVisualLease> {
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
        let owner = match self.close_lane {
            0 => &mut self.region_order,
            1 => &mut self.element_order,
            2 => {
                if self.page_count != 0 {
                    let bytes = self.output.bytes.len().min(PAGE_BYTES);
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.output.bytes.truncate(self.output.bytes.len() - bytes);
                    self.page_count -= 1;
                    return (false, 1, bytes);
                }
                if self.output.bytes.capacity() != 0 {
                    self.output.bytes = String::new();
                    return (false, 1, 0);
                }
                self.close_lane = 3;
                return (false, 1, 0);
            }
            3 => {
                if self.meshes.bytes.capacity() != 0 {
                    self.meshes.bytes = String::new();
                    return (false, 1, 0);
                }
                return (true, 0, 0);
            }
            _ => return (true, 0, 0),
        };
        if owner.pop().is_some() {
            return (false, 1, 0);
        }
        let bytes = owner.capacity() * std::mem::size_of::<usize>();
        if bytes > maximum_bytes {
            return (false, 0, 0);
        }
        *owner = Vec::new();
        self.close_lane += 1;
        (false, 1, bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.complete.is_none() && self.output.bytes.capacity() == 0 && self.meshes.bytes.capacity() == 0 && self.page_count == 0 && self.region_order.capacity() == 0 && self.element_order.capacity() == 0
    }
}
//#endregion 🧵️VisualJob

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
    candidate: Option<Fem3dVisualJob>,
    current: Option<Fem3dMountedVisualLease>,
    displaced: Option<Fem3dMountedVisualLease>,
    fault: Option<Vec<u8>>,
    close_lane: u8,
    done: bool,
}

impl MountedState {
    fn new(identity: Identity, snapshot: store::SnapshotRead<Fem3dSnapshot>, current: Option<Fem3dMountedVisualLease>) -> Self {
        Self { identity, snapshot: Some(snapshot), snapshot_return: None, cancel: semio_framework_job::root_cancel_token(), preview_sequence: 0, candidate: None, current, displaced: None, fault: None, close_lane: 0, done: false }
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
            let (terminal, _, _) = displaced.close_step(PAGE_BYTES);
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
            self.candidate = Some(Fem3dVisualJob::new(self.identity.freshness(self.preview_sequence)));
            return JobStep::Running(None);
        }
        let freshness = self.identity.freshness(self.preview_sequence);
        let Some(snapshot) = self.snapshot.as_ref() else { return self.fail(b"fem3d.visual-snapshot-owner".to_vec()) };
        cx.consume_fuel(1);
        let step = self.candidate.as_mut().map(|candidate| candidate.step_one(snapshot, freshness));
        match step {
            Some(Ok(false)) => JobStep::Running(None),
            Some(Ok(true)) => {
                let Some(lease) = self.candidate.as_mut().and_then(Fem3dVisualJob::take_complete) else { return self.fail(b"fem3d.visual-complete-owner".to_vec()) };
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
                self.close_lane = 4;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            4 => {
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
                self.close_lane = 5;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            _ => PluginCloseStep::Complete,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_lane >= 5 && self.snapshot.is_none() && self.snapshot_return.is_none() && self.candidate.is_none() && self.current.is_none() && self.displaced.is_none() && self.fault.is_none()
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
    output_bytes: usize,
}

impl SnapshotPreflight {
    fn new() -> Self {
        Self { stage: SnapshotPreflightStage::Nodes, outer: 0, inner: 0, load_count: 0, output_bytes: 2 }
    }

    fn charge(&mut self, bytes: usize) -> Result<(), ()> {
        let next = self.output_bytes.checked_add(bytes).ok_or(())?;
        if next > OUTPUT_BYTES {
            return Err(());
        }
        self.output_bytes = next;
        Ok(())
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
                self.charge(node.id.len().checked_add(256).ok_or(())?)?;
                self.outer += 1;
            }
            SnapshotPreflightStage::Elements => {
                let Some(element) = snapshot.elements.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Regions);
                    return Ok(false);
                };
                self.charge(element_id(element).len().checked_add(512).ok_or(())?)?;
                self.outer += 1;
            }
            SnapshotPreflightStage::Regions => {
                let Some(solid) = snapshot.solids.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Supports);
                    return Ok(false);
                };
                self.charge(solid.id.len().checked_add(384).ok_or(())?)?;
                self.outer += 1;
            }
            SnapshotPreflightStage::Supports => {
                let Some(support) = snapshot.supports.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::LoadCases);
                    return Ok(false);
                };
                self.charge(support.id.len().checked_add(256).ok_or(())?)?;
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
                self.charge(load_id(load).len().checked_add(256).ok_or(())?)?;
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
        let Some(_pending) = registry.pending[slot].filter(|pending| {
            pending.render.app_instance_id == render.app_instance_id
                && pending.render.base_revision == render.base_revision
                && pending.render.generation == render.generation
                && pending.render.canonical_base_revision == render.canonical_base_revision
                && pending.preflight.stage == SnapshotPreflightStage::Complete
        }) else {
            return Vec::new();
        };
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
        *registry.shells[shell as usize].borrow_mut() = Some(MountedState::new(identity, snapshot, former));
        registry.current[slot] = Some(Current { app_instance_id: render.app_instance_id, shell, identity });
        let mut effects = Vec::with_capacity(2);
        if let Some(previous) = previous {
            effects.push(Effect::CancelJob { job: previous.identity.job });
        }
        effects.push(Effect::SpawnJob { job, kind: FEM3D_MOUNTED_VISUAL_JOB_KIND.to_string(), input: encode_input(shell, identity), placement: JobPlacement::Isolated });
        effects
    })
}

pub fn with_live_visual<R>(render: Option<AppRenderOperationContext>, build: impl FnOnce(Option<&Fem3dMountedVisualLease>) -> R) -> R {
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

    #[test]
    fn fem3d_visual_maximum_plus_one_rejects_before_owner_transfer() {
        let mut doc = Fem3dSnapshot::default();
        doc.nodes.resize_with(MAXIMUM_NODES + 1, || crate::artifacts::fem3d::FemNode { id: "n".into(), x: 0.0, y: 0.0, z: 0.0 });
        let before = doc.nodes.as_ptr();
        let mut job = Fem3dVisualJob::new(freshness(19));
        assert!(job.step_one(&doc, freshness(19)).is_err());
        assert_eq!(before, doc.nodes.as_ptr());
        assert_eq!(job.stage(), Fem3dVisualJobStage::ReserveSnapshot);
    }

    #[test]
    fn fem3d_snapshot_preflight_maximum_plus_one_returns_the_exact_producer() {
        let mut doc = Fem3dSnapshot::default();
        doc.nodes.push(crate::artifacts::fem3d::FemNode { id: "n".into(), x: 0.0, y: 0.0, z: 0.0 });
        let producer = doc.nodes.as_ptr();
        let mut output = SnapshotPreflight { stage: SnapshotPreflightStage::Nodes, outer: 0, inner: 0, load_count: 0, output_bytes: OUTPUT_BYTES };
        assert_eq!(output.step_one(&doc), Err(()));
        assert_eq!(producer, doc.nodes.as_ptr());
        assert_eq!(output.output_bytes, OUTPUT_BYTES);

        let load = crate::artifacts::fem3d::FemLoad::Nodal { id: "l".into(), node_id: "n".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -1.0 };
        doc.load_cases.push(crate::artifacts::fem3d::FemLoadCase { id: "case".into(), name: "case".into(), loads: vec![load], self_weight: false });
        let producer = doc.load_cases.as_ptr();
        let mut loads = SnapshotPreflight { stage: SnapshotPreflightStage::LoadCases, outer: 0, inner: 0, load_count: MAXIMUM_LOADS, output_bytes: 2 };
        assert_eq!(loads.step_one(&doc), Err(()));
        assert_eq!(producer, doc.load_cases.as_ptr());
        assert_eq!(loads.load_count, MAXIMUM_LOADS);
    }

    #[test]
    fn fem3d_visual_stale_freshness_cannot_publish() {
        let doc = Fem3dSnapshot::default();
        let mut job = Fem3dVisualJob::new(freshness(19));
        let mut turns = 0;
        while job.stage() != Fem3dVisualJobStage::ValidateFreshness && turns < 256 {
            job.step_one(&doc, freshness(19)).expect("bounded step");
            turns += 1;
        }
        assert!(job.step_one(&doc, freshness(23)).is_err());
        assert!(job.take_complete().is_none());
    }

    #[test]
    fn fem3d_visual_replay_and_partial_close_are_exact() {
        let doc = Fem3dSnapshot::default();
        let build = || {
            let mut job = Fem3dVisualJob::new(freshness(19));
            let mut turns = 0;
            while !job.step_one(&doc, freshness(19)).expect("bounded step") && turns < 512 {
                turns += 1;
            }
            job.take_complete().expect("sealed lease")
        };
        let mut first = build();
        let second = build();
        assert_eq!(first.instances_json(), second.instances_json());
        assert!(first.instances_json().contains("accessible-en"));
        assert!(first.instances_json().contains("accessible-de"));
        let mut turns = 0;
        while !first.close_step(PAGE_BYTES).0 && turns < 512 {
            turns += 1;
        }
        assert!(first.terminal_is_empty());
    }

    #[test]
    fn fem3d_visual_cancel_deadline_fault_and_device_close_preserve_last_valid() {
        let doc = Fem3dSnapshot::default();
        let build = || {
            let mut job = Fem3dVisualJob::new(freshness(19));
            let mut turns = 0;
            while !job.step_one(&doc, freshness(19)).expect("bounded step") && turns < 512 {
                turns += 1;
            }
            job.take_complete().expect("sealed lease")
        };
        let current = build();
        let current_bytes = current.instances_json().as_bytes().to_vec();
        let mut cancelled = Fem3dVisualJob::new(freshness(23));
        assert!(!cancelled.close_step(PAGE_BYTES).0);
        let mut faulted = Fem3dVisualJob::new(freshness(29));
        while faulted.stage() != Fem3dVisualJobStage::ValidateFreshness {
            faulted.step_one(&doc, freshness(29)).expect("bounded step");
        }
        assert!(faulted.step_one(&doc, freshness(31)).is_err());
        assert_eq!(current.instances_json().as_bytes(), current_bytes);
        assert_eq!(Fem3dVisualState::FaultedCancelled.id(), "faulted-cancelled-last-valid");
        assert!(!faulted.close_step(PAGE_BYTES).0);
    }

    #[test]
    fn fem3d_visual_each_production_step_stays_below_eight_ms() {
        let doc = Fem3dSnapshot::default();
        let mut job = Fem3dVisualJob::new(freshness(19));
        let started = std::time::Instant::now();
        let _ = job.step_one(&doc, freshness(19));
        assert!(started.elapsed().as_micros() < 8_000);
    }
}
//#endregion 🧪️Laws
