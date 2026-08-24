//! 🧱️ Fem2d play app — the model window: the editable 2D structural canvas (nodes/members/supports,
//! mesh-edge preview overlay). Also hosts the screen-space draw helpers shared with the results window
//! (`crate::editor::fem2d::modes::edit::windows::results`) — kept here rather than in the artifact's
//! `⚙️engine` because they take/return app-facing `semio_framework_plugin` scene types and their only
//! two consumers are these two sibling windows, both at app level.

use crate::artifacts::fem2d::{element_id, Fem2dSnapshot, FemCamera, FemDof, FemElement, FemLoad};
use crate::model::Dof;
use semio_framework_plugin::{BuiltNode, Canvas2dScene};
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
    pub converged: bool,
    pub validated_final: bool,
}

const FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES: usize = 16 * 1_024;
const FEM2D_MOUNTED_VISUAL_PAGE_BYTES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fem2dMountedVisualStage {
    ReserveOutput,
    ObserveOutput,
    ReserveOrderIndexes,
    OrderRegionKey,
    OrderAssemblyKey,
    OrderFieldKey,
    Begin,
    Node,
    Element,
    Support,
    Load,
    LoadAreaPoint,
    LoadAreaCommit,
    Region,
    RegionPoint,
    RegionClose,
    Assembly,
    Displacement,
    Residual,
    Status,
    Seal,
    Complete,
}

#[derive(Debug, PartialEq)]
pub struct Fem2dMountedVisualLease {
    app_instance_id: u32,
    base_revision: u64,
    generation: u64,
    operation: u64,
    preview_sequence: u64,
    layers_json: String,
    region_order: Vec<usize>,
    assembly_order: Vec<usize>,
    field_order: Vec<usize>,
    close_pages: usize,
}

impl Fem2dMountedVisualLease {
    pub(crate) fn matches(&self, app_instance_id: u32, base_revision: u64, generation: u64) -> bool {
        self.app_instance_id == app_instance_id && self.base_revision == base_revision && self.generation == generation
    }

    pub(crate) fn layers_json(&self) -> &str {
        &self.layers_json
    }

    pub(crate) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        for owner in [&mut self.region_order, &mut self.assembly_order, &mut self.field_order] {
            if owner.pop()?.is_some() {
                return (false, 1, 0);
            }
            let bytes = owner.capacity() * std::mem::size_of::<usize>();
            if bytes != 0 {
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                *owner = Vec::new();
                return (false, 1, bytes);
            }
        }
        if self.close_pages != 0 {
            let bytes = self.layers_json.len().min(FEM2D_MOUNTED_VISUAL_PAGE_BYTES);
            if bytes > maximum_bytes {
                return (false, 0, 0);
            }
            self.layers_json.truncate(self.layers_json.len() - bytes);
            self.close_pages -= 1;
            return (false, 1, bytes);
        }
        if self.layers_json.capacity() != 0 {
            self.layers_json = String::new();
            return (false, 1, 0);
        }
        (true, 0, 0)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.layers_json.capacity() == 0 && self.close_pages == 0 && self.region_order.capacity() == 0 && self.assembly_order.capacity() == 0 && self.field_order.capacity() == 0
    }
}

struct Fem2dBoundedJson {
    bytes: String,
    admitted_capacity: usize,
}

impl Fem2dBoundedJson {
    fn new() -> Self {
        Self { bytes: String::new(), admitted_capacity: 0 }
    }

    fn reserve(&mut self) -> Result<(), &'static [u8]> {
        self.bytes.try_reserve_exact(FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES).map_err(|_| b"fem2d.visual-output-allocation" as &'static [u8])
    }

    fn observe_and_admit(&mut self) -> Result<(), &'static [u8]> {
        let capacity = self.bytes.capacity();
        if capacity != FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES {
            return Err(b"fem2d.visual-output-observed-capacity");
        }
        self.admitted_capacity = capacity;
        Ok(())
    }
}

impl Write for Fem2dBoundedJson {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let next = self.bytes.len().checked_add(value.len()).ok_or(std::fmt::Error)?;
        if self.admitted_capacity == 0 || next > self.admitted_capacity {
            return Err(std::fmt::Error);
        }
        self.bytes.push_str(value);
        Ok(())
    }
}

/// 🧵 Mounted whole-scene encoder; one call emits one stable schema key, glyph, vector or control.
pub struct Fem2dMountedVisualBuild {
    app_instance_id: u32,
    base_revision: u64,
    generation: u64,
    operation: u64,
    preview_sequence: u64,
    stage: Fem2dMountedVisualStage,
    cursor: usize,
    load_case: usize,
    load: usize,
    point: usize,
    area_sum: [f64; 2],
    output: Fem2dBoundedJson,
    region_order: Vec<usize>,
    assembly_order: Vec<usize>,
    field_order: Vec<usize>,
    first: bool,
    close_pages: usize,
    complete: Option<Fem2dMountedVisualLease>,
}

impl Fem2dMountedVisualBuild {
    pub fn new(app_instance_id: u32, base_revision: u64, generation: u64, operation: u64, preview_sequence: u64) -> Self {
        Self {
            app_instance_id,
            base_revision,
            generation,
            operation,
            preview_sequence,
            stage: Fem2dMountedVisualStage::ReserveOutput,
            cursor: 0,
            load_case: 0,
            load: 0,
            point: 0,
            area_sum: [0.0; 2],
            output: Fem2dBoundedJson::new(),
            region_order: Vec::new(),
            assembly_order: Vec::new(),
            field_order: Vec::new(),
            first: true,
            close_pages: 0,
            complete: None,
        }
    }

    fn layer_prefix(&mut self) -> Result<(), Vec<u8>> {
        let separator = if std::mem::replace(&mut self.first, false) { "" } else { "," };
        self.output.write_str(separator).map_err(|_| b"fem2d.visual-output-capacity".to_vec())
    }

    fn finish_stage(&mut self, stage: Fem2dMountedVisualStage) {
        self.cursor = 0;
        self.stage = stage;
    }

    pub fn step_one(&mut self, doc: &Fem2dSnapshot, visual: &Fem2dLiveVisual) -> Result<bool, Vec<u8>> {
        match self.stage {
            Fem2dMountedVisualStage::ReserveOutput => {
                self.output.reserve().map_err(<[u8]>::to_vec)?;
                self.stage = Fem2dMountedVisualStage::ObserveOutput;
            }
            Fem2dMountedVisualStage::ObserveOutput => {
                self.output.observe_and_admit().map_err(<[u8]>::to_vec)?;
                self.close_pages = self.output.admitted_capacity / FEM2D_MOUNTED_VISUAL_PAGE_BYTES;
                self.stage = Fem2dMountedVisualStage::ReserveOrderIndexes;
            }
            Fem2dMountedVisualStage::ReserveOrderIndexes => {
                self.region_order.try_reserve_exact(doc.regions.len()).map_err(|_| b"fem2d.visual-region-order-backing".to_vec())?;
                self.assembly_order.try_reserve_exact(visual.assembling_element_ids.len()).map_err(|_| b"fem2d.visual-assembly-order-backing".to_vec())?;
                self.field_order.try_reserve_exact(visual.fields.len())?.map_err(|_| b"fem2d.visual-field-order-backing".to_vec())?;
                if [&self.region_order, &self.assembly_order, &self.field_order].into_iter()?.any(|owner| owner.capacity().checked_mul(std::mem::size_of::<usize>()).is_none_or(|bytes| bytes > FEM2D_MOUNTED_VISUAL_PAGE_BYTES)) {
                    return Err(b"fem2d.visual-order-observed-capacity".to_vec());
                }
                self.cursor = 0;
                self.stage = Fem2dMountedVisualStage::OrderRegionKey;
            }
            Fem2dMountedVisualStage::OrderRegionKey => {
                let Some(region) = doc.regions.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Fem2dMountedVisualStage::OrderAssemblyKey;
                    return Ok(false);
                };
                let index = self.region_order.binary_search_by(|current| doc.regions[*current].id.cmp(&region.id).then_with(|| current.cmp(&self.cursor))).unwrap_or_else(|index| index);
                self.region_order.insert(index, self.cursor);
                self.cursor += 1;
            }
            Fem2dMountedVisualStage::OrderAssemblyKey => {
                let Some(id) = visual.assembling_element_ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Fem2dMountedVisualStage::OrderFieldKey;
                    return Ok(false);
                };
                let index = self.assembly_order.binary_search_by(|current| visual.assembling_element_ids[*current].cmp(id).then_with(|| current.cmp(&self.cursor))).unwrap_or_else(|index| index);
                self.assembly_order.insert(index, self.cursor);
                self.cursor += 1;
            }
            Fem2dMountedVisualStage::OrderFieldKey => {
                let Some(field) = visual.fields.get(self.cursor)? else {
                    self.cursor = 0;
                    self.stage = Fem2dMountedVisualStage::Begin;
                    return Ok(false);
                };
                let index = self.field_order.binary_search_by(|current| visual.fields[*current].node_id.cmp(&field.node_id).then_with(|| current.cmp(&self.cursor)))?.unwrap_or_else(|index| index);
                self.field_order.insert(index, self.cursor)?;
                self.cursor += 1;
            }
            Fem2dMountedVisualStage::Begin => {
                self.output.write_char('[').map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.stage = Fem2dMountedVisualStage::Node;
            }
            Fem2dMountedVisualStage::Node => {
                let Some(node) = doc.nodes.get(self.cursor) else {
                    self.finish_stage(Fem2dMountedVisualStage::Element);
                    return Ok(false);
                };
                self.layer_prefix()?;
                let (x, y) = screen_2d(node.x, node.y);
                write!(self.output, "{{\"kind\":\"circle\",\"id\":\"node-{}\",\"x\":{},\"y\":{},\"width\":8,\"height\":8,\"color\":\"#38bdf8\"}}", node.id, x - 4.0, y - 4.0).map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
            }
            Fem2dMountedVisualStage::Element => {
                let Some(element) = doc.elements.get(self.cursor) else {
                    self.finish_stage(Fem2dMountedVisualStage::Support);
                    return Ok(false);
                };
                let (start, end) = fem2d_element_endpoints(element);
                if let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) {
                    self.layer_prefix()?;
                    let (x0, y0) = screen_2d(a.x, a.y);
                    let (x1, y1) = screen_2d(b.x, b.y);
                    write!(self.output, "{{\"kind\":\"line\",\"id\":\"el-{}\",\"x0\":{x0},\"y0\":{y0},\"x1\":{x1},\"y1\":{y1},\"color\":\"#94a3b8\"}}", element_id(element)).map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                }
                self.cursor += 1;
            }
            Fem2dMountedVisualStage::Support => {
                let Some(support) = doc.supports.get(self.cursor) else {
                    self.finish_stage(Fem2dMountedVisualStage::Load);
                    return Ok(false);
                };
                if let Some(node) = find_node_2d(&doc.nodes, &support.node_id) {
                    self.layer_prefix()?;
                    let (x, y) = screen_2d(node.x, node.y);
                    write!(self.output, "{{\"kind\":\"circle\",\"id\":\"support-{}\",\"x\":{},\"y\":{},\"width\":10,\"height\":10,\"color\":\"#f97316\"}}", support.id, x - 5.0, y - 5.0).map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                }
                self.cursor += 1;
            }
            Fem2dMountedVisualStage::Load => {
                let Some(case) = doc.load_cases.get(self.load_case) else {
                    self.finish_stage(Fem2dMountedVisualStage::Region);
                    return Ok(false);
                };
                let Some(load) = case.loads.get(self.load) else {
                    self.load_case += 1;
                    self.load = 0;
                    return Ok(false);
                };
                match load {
                    FemLoad::Nodal { id, node_id, dof, value } => {
                        if let Some(node) = find_node_2d(&doc.nodes, node_id) {
                            self.layer_prefix()?;
                            let origin = screen_2d(node.x, node.y);
                            let vector = match dof {
                                FemDof::Tx => [value.signum() * 18.0, 0.0],
                                FemDof::Ty => [0.0, value.signum() * 18.0],
                                _ => [0.0, -12.0],
                            };
                            write!(self.output, "{{\"kind\":\"polyline\",\"id\":\"load-{id}\",\"points\":[[{},{}],[{},{}]],\"color\":\"#ef4444\"}}", origin.0, origin.1, origin.0 + vector[0], origin.1 - vector[1])
                                .map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                        }
                    }
                    FemLoad::MemberUdl { id, element_id: target, wx, wy } => {
                        if let Some(element) = doc.elements.iter().find(|element| element_id(element) == target) {
                            let (start, end) = fem2d_element_endpoints(element);
                            if let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) {
                                self.layer_prefix()?;
                                let origin = screen_2d((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
                                let vector = [wx.signum() * 18.0, wy.signum() * 18.0];
                                write!(self.output, "{{\"kind\":\"polyline\",\"id\":\"load-{id}\",\"points\":[[{},{}],[{},{}]],\"color\":\"#ef4444\"}}", origin.0, origin.1, origin.0 + vector[0], origin.1 - vector[1])
                                    .map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                            }
                        }
                    }
                    FemLoad::Area { .. } => {
                        self.point = 0;
                        self.area_sum = [0.0; 2];
                        self.stage = Fem2dMountedVisualStage::LoadAreaPoint;
                        return Ok(false);
                    }
                }
                self.load += 1;
            }
            Fem2dMountedVisualStage::LoadAreaPoint => {
                let FemLoad::Area { region_id, .. } = &doc.load_cases[self.load_case].loads[self.load] else { return Err(b"fem2d.visual-area-load-cursor".to_vec()) };
                let Some(region) = doc.regions.iter().find(|region| region.id == *region_id) else {
                    self.load += 1;
                    self.stage = Fem2dMountedVisualStage::Load;
                    return Ok(false);
                };
                if let Some(point) = region.outline.get(self.point) {
                    self.area_sum[0] += point[0];
                    self.area_sum[1] += point[1];
                    self.point += 1;
                } else {
                    self.stage = Fem2dMountedVisualStage::LoadAreaCommit;
                }
            }
            Fem2dMountedVisualStage::LoadAreaCommit => {
                let FemLoad::Area { id, pressure, .. } = &doc.load_cases[self.load_case].loads[self.load] else { return Err(b"fem2d.visual-area-load-cursor".to_vec()) };
                if self.point != 0 {
                    self.layer_prefix()?;
                    let origin = screen_2d(self.area_sum[0] / self.point as f64, self.area_sum[1] / self.point as f64);
                    let vector = [0.0, -pressure.signum() * 18.0];
                    write!(self.output, "{{\"kind\":\"polyline\",\"id\":\"load-{id}\",\"points\":[[{},{}],[{},{}]],\"color\":\"#ef4444\"}}", origin.0, origin.1, origin.0 + vector[0], origin.1 - vector[1])
                        .map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                }
                self.load += 1;
                self.stage = Fem2dMountedVisualStage::Load;
            }
            Fem2dMountedVisualStage::Region => {
                let Some(region_index) = self.region_order.get(self.cursor).copied() else {
                    self.finish_stage(Fem2dMountedVisualStage::Assembly);
                    return Ok(false);
                };
                let region = &doc.regions[region_index];
                let quality = visual.region_quality.get(&region.id).copied().unwrap_or_default();
                self.layer_prefix()?;
                write!(self.output, "{{\"kind\":\"polyline\",\"id\":\"region-quality-{}-{}\",\"points\":[", quality.id(), region.id).map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.point = 0;
                self.stage = Fem2dMountedVisualStage::RegionPoint;
            }
            Fem2dMountedVisualStage::RegionPoint => {
                let region = &doc.regions[self.region_order[self.cursor]];
                if let Some(point) = region.outline.get(self.point) {
                    let separator = if self.point == 0 { "" } else { "," };
                    let (x, y) = screen_2d(point[0], point[1]);
                    write!(self.output, "{separator}[{x},{y}]").map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                    self.point += 1;
                } else {
                    self.stage = Fem2dMountedVisualStage::RegionClose;
                }
            }
            Fem2dMountedVisualStage::RegionClose => {
                let region = &doc.regions[self.region_order[self.cursor]];
                if let Some(point) = region.outline.first() {
                    let separator = if self.point == 0 { "" } else { "," };
                    let (x, y) = screen_2d(point[0], point[1]);
                    write!(self.output, "{separator}[{x},{y}]").map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                }
                let quality = visual.region_quality.get(&region.id).copied().unwrap_or_default();
                write!(self.output, "],\"color\":\"{}\"}}", quality.color()).map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.cursor += 1;
                self.stage = Fem2dMountedVisualStage::Region;
            }
            Fem2dMountedVisualStage::Assembly => {
                let Some(index) = self.assembly_order.get(self.cursor).copied() else {
                    self.finish_stage(Fem2dMountedVisualStage::Displacement);
                    return Ok(false);
                };
                let id = &visual.assembling_element_ids[index];
                if let Some(element) = doc.elements.iter().find(|element| element_id(element) == id) {
                    let (start, end) = fem2d_element_endpoints(element);
                    if let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) {
                        self.layer_prefix()?;
                        let (x0, y0) = screen_2d(a.x, a.y);
                        let (x1, y1) = screen_2d(b.x, b.y);
                        write!(self.output, "{{\"kind\":\"line\",\"id\":\"assembling-{id}\",\"x0\":{x0},\"y0\":{y0},\"x1\":{x1},\"y1\":{y1},\"color\":\"#a855f7\"}}").map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                    }
                }
                self.cursor += 1;
            }
            Fem2dMountedVisualStage::Displacement | Fem2dMountedVisualStage::Residual => {
                let Some(index) = self.field_order.get(self.cursor)?.copied() else {
                    self.finish_stage(if self.stage == Fem2dMountedVisualStage::Displacement { Fem2dMountedVisualStage::Residual } else { Fem2dMountedVisualStage::Status });
                    return Ok(false);
                };
                let field = &visual.fields[index];
                if let Some(node) = find_node_2d(&doc.nodes, &field.node_id) {
                    self.layer_prefix()?;
                    let origin = screen_2d(node.x, node.y);
                    let (prefix, vector, color) =
                        if self.stage == Fem2dMountedVisualStage::Displacement { ("displacement", [field.displacement[0] * SCALE_2D, field.displacement[1] * SCALE_2D], "#f472b6") } else { ("residual", field.residual, "#eab308") };
                    write!(self.output, "{{\"kind\":\"polyline\",\"id\":\"{prefix}-field-{}\",\"points\":[[{},{}],[{},{}]],\"color\":\"{color}\"}}", field.node_id, origin.0, origin.1, origin.0 + vector[0], origin.1 - vector[1])
                        .map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                }
                self.cursor += 1;
            }
            Fem2dMountedVisualStage::Status => {
                self.layer_prefix()?;
                let status = if visual.validated_final {
                    "validated-final"
                } else if visual.converged {
                    "converged"
                } else {
                    "unconverged"
                };
                write!(self.output, "{{\"id\":\"solve-status-{status}\",\"transform\":[1,0,0,1,10,18],\"text\":{{\"content\":\"{status}\",\"size\":11}}}}").map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                self.stage = Fem2dMountedVisualStage::Seal;
            }
            Fem2dMountedVisualStage::Seal => {
                self.output.write_char(']').map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
                let layers_json = std::mem::take(&mut self.output.bytes);
                let close_pages = self.close_pages;
                self.close_pages = 0;
                self.complete = Some(Fem2dMountedVisualLease {
                    app_instance_id: self.app_instance_id,
                    base_revision: self.base_revision,
                    generation: self.generation,
                    operation: self.operation,
                    preview_sequence: self.preview_sequence,
                    layers_json,
                    region_order: std::mem::take(&mut self.region_order),
                    assembly_order: std::mem::take(&mut self.assembly_order),
                    field_order: std::mem::take(&mut self.field_order),
                    close_pages,
                });
                self.stage = Fem2dMountedVisualStage::Complete;
            }
            Fem2dMountedVisualStage::Complete => return Ok(true)?,
        }
        Ok(self.stage == Fem2dMountedVisualStage::Complete)
    }

    pub fn take_complete(&mut self) -> Option<Fem2dMountedVisualLease> {
        (self.stage == Fem2dMountedVisualStage::Complete).then(|| self.complete.take()).flatten()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(lease) = self.complete.as_mut() {
            let step = lease.close_step(maximum_bytes);
            if !step.0 {
                return step;
            }
            self.complete = None;
            return (false, 1, 0);
        }
        for owner in [&mut self.region_order, &mut self.assembly_order, &mut self.field_order] {
            if owner.pop()?.is_some() {
                return (false, 1, 0);
            }
            let bytes = owner.capacity() * std::mem::size_of::<usize>();
            if bytes != 0 {
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                *owner = Vec::new();
                return (false, 1, bytes);
            }
        }
        if self.close_pages != 0 {
            if FEM2D_MOUNTED_VISUAL_PAGE_BYTES > maximum_bytes {
                return (false, 0, 0);
            }
            self.output.bytes.truncate(self.output.bytes.len().saturating_sub(FEM2D_MOUNTED_VISUAL_PAGE_BYTES));
            self.close_pages -= 1;
            return (false, 1, FEM2D_MOUNTED_VISUAL_PAGE_BYTES);
        }
        if self.output.bytes.capacity() != 0 {
            self.output.bytes = String::new();
            return (false, 1, 0);
        }
        (true, 0, 0)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete.is_none() && self.output.bytes.capacity() == 0 && self.close_pages == 0 && self.region_order.capacity() == 0 && self.assembly_order.capacity() == 0 && self.field_order.capacity() == 0
    }
}

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
    crate::app_surface::canvas_2d_surface(BODY_KEY, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}

/// 👁️ Renders the model plus an optional replaceable worker-job progress snapshot.
pub fn render_with_progress(_doc: &Fem2dSnapshot, camera: &FemCamera, progress: Option<&Fem2dMountedVisualLease>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let layers_json = progress.map(Fem2dMountedVisualLease::layers_json).unwrap_or("[]").to_owned();
    crate::app_surface::canvas_2d_surface(BODY_KEY, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::testkit::{fem2d_app, render as render_body};

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
                fields: vec![NodeLiveField { node_id: node_id.clone(), displacement: [0.01, -0.02], residual: [3.0, -4.0] }],
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
            fields: (0..256).rev().map(|index| NodeLiveField { node_id: node_id.clone(), displacement: [index as f64 * 1e-6, -1e-4], residual: [index as f64 * 1e-3, -0.5] }).collect(),
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
    fn mounted_visual_output_exact_maximum_plus_one_and_displacement_handback() {
        let mut output = Fem2dBoundedJson::new();
        output.reserve().expect("fixed output backing");
        output.observe_and_admit().expect("observed backing is the admitted backing");
        let exact = "x".repeat(FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES);
        output.write_str(&exact).expect("exact maximum");
        let before = output.bytes.as_ptr();
        assert!(output.write_char('x').is_err(), "maximum plus one must reject before reallocating");
        assert_eq!(output.bytes.len(), FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES);
        assert_eq!(output.bytes.as_ptr(), before, "rejection returns the exact unchanged output owner");

        let doc = Fem2dSnapshot::default();
        let visual = Fem2dLiveVisual::default();
        let mut build = Fem2dMountedVisualBuild::new(1, 2, 3, 4, 5);
        while !build.step_one(&doc, &visual).expect("bounded visual step") {}
        let mut lease = build.take_complete().expect("sealed output lease");
        assert_eq!(lease.layers_json(), "[{\"id\":\"solve-status-unconverged\",\"transform\":[1,0,0,1,10,18],\"text\":{\"content\":\"unconverged\",\"size\":11}}]");
        assert!(build.terminal_is_empty(), "all order/output backing moved into the exact lease");
        while !lease.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0 {}
        assert!(lease.terminal_is_empty());
    }
}
//#endregion 🧪️Tests
