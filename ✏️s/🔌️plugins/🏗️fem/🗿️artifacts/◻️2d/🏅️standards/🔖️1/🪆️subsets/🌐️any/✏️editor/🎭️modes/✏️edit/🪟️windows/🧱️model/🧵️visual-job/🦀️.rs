//! 🧵️ Visual job: the visual limits, the job stage and the visual job with its stepping. See [`super`].

use super::*;

//#region 👁️LiveVisualLanguage
//#region 🧵️MountedVisualJob
pub(super) const FEM2D_VISUAL_MAXIMUM_REGIONS: usize = 64;

const FEM2D_VISUAL_MAXIMUM_NODES: usize = semio_s_fem_analyses_model::MOUNTED_ANALYSIS_NODE_SLOTS;

pub(super) const FEM2D_VISUAL_MAXIMUM_ELEMENTS: usize = semio_s_fem_analyses_model::MOUNTED_ANALYSIS_ELEMENT_SLOTS;

const FEM2D_VISUAL_MAXIMUM_SUPPORTS: usize = 64;

const FEM2D_VISUAL_MAXIMUM_LOADS: usize = 64;

pub(super) const FEM2D_VISUAL_MAXIMUM_FIELDS: usize = semio_s_fem_analyses_model::MOUNTED_ANALYSIS_NODE_SLOTS;

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
    pub(super) freshness: Fem2dVisualFreshness,
    stage: Fem2dVisualJobStage,
    output: Fem2dFixedPacketPages,
    token: Option<Canvas2dSnapshotWriteToken>,
    abort_started: bool,
    pub(super) region_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_REGIONS>,
    element_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_ELEMENTS>,
    pub(super) field_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_FIELDS>,
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
    pub(super) quality: RegionVisualQuality,
    endpoint_a: [f64; 2],
    endpoint_b: [f64; 2],
    pub(super) first: bool,
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
                            write!(self.output, "{{\"kind\":\"line\",\"id\":\"region-quality-{}-{}-{}\",\"x0\":{x0},\"y0\":{y0},\"x1\":{x1},\"y1\":{y1},\"color\":\"{}\"}}", self.quality.id(), region.id, self.point_cursor, self.quality.color())
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
                write!(self.output, "{{\"kind\":\"text\",\"id\":\"assembling-{id}\",\"x\":10,\"y\":36,\"text\":{{\"content\":\"assembling {id}\",\"size\":11}}}}").map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
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
                write!(self.output, "{{\"kind\":\"line\",\"id\":\"load-{id}\",\"x0\":20,\"y0\":52,\"x1\":{},\"y1\":{}}}", 20.0 + vector[0], 52.0 - vector[1]).map_err(|_| b"fem2d.visual-output-capacity".to_vec())?;
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
                    Fem2dVisualJobStage::BuildDisplacementEntry => {
                        write!(self.output, "{{\"kind\":\"line\",\"id\":\"displacement-field-{}\",\"x0\":30,\"y0\":80,\"x1\":{},\"y1\":{}}}", field.node_id, 30.0 + field.displacement[0] * SCALE_2D, 80.0 - field.displacement[1] * SCALE_2D)
                    }
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
            return (false, 1, size_of::<usize>());
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
        self.complete.is_none() && self.token.is_none_or(canvas2d_snapshot_write_terminal_is_empty) && self.output.terminal_is_empty() && self.region_order.is_empty() && self.element_order.is_empty() && self.field_order.is_empty()
    }
}
//#endregion 🧵️MountedVisualJob
//#endregion 👁️LiveVisualLanguage
