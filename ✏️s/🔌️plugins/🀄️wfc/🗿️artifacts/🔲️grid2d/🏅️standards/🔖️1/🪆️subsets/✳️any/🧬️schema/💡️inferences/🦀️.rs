//! 💡️ `s.wfc.grid2d` inferences — THE SOLVE ITSELF IS AN INFERENCE. `Grid2dSnapshot` only ever
//! persists the PROBLEM (grid extent, cell size, periodicity, tiles, rules, pins, mask); the
//! SOLUTION, the contradiction verdict and the pre-propagation entropy map are derived here, never
//! mutation-authored state. The shared engine (`semio_s_plugin_wfc_engine`) supplies the tiled
//! model builder, the four-neighbour `Stencil2d` relation declaration, the dense `Grid2dTopology`
//! and the resumable `WfcJob` this facet drives.
//!
//! **Rule default.** A `(tileA, tileB, direction)` pair with no authored rule is FORBIDDEN: the
//! rule set is the complete adjacency whitelist, so an empty rule set is unsatisfiable the moment
//! the grid holds two adjacent unmasked cells. An authored row with `allowed = false` is a
//! first-class refusal an editor can toggle without losing the row's id.
//!
//! Determinism: `solve_with_job` reads only `snapshot` fields (`seed` included) and drives the same
//! resumable `WfcJob` interactive callers use; every step is watchdog-wrapped and explicitly
//! bounded. No ambient randomness enters, so `DepHash` caching over `Grid2dSolve`/
//! `Grid2dContradiction`/`Grid2dEntropy` is sound.

use crate::schema::snapshot::{Grid2dSnapshot, WfcDirection2d};
use semio_s_plugin_wfc_engine as wfc;
use std::collections::BTreeMap;

//#region 📦️RetainedPayload
/// 📦️ One single-page payload, with the job module's EXACT source handback on refusal. Dropping a
/// `JobPayloadRejectedPage` without taking its source back trips that module's own lifecycle
/// assertion and then aborts the process from a second panic inside `RetainedJobPayload::drop`. An
/// empty payload is the honest answer to a refused admission; leaking the page never was.
fn retained_payload(context: &mut semio_framework_job::StepContext<'_>, stream: semio_framework_job::JobPayloadStream, bytes: &[u8]) -> semio_framework_job::RetainedJobPayload {
    match context.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            semio_framework_job::RetainedJobPayload::empty(stream)
        }
    }
}
//#endregion 📦️RetainedPayload

//#region 🔖️Identity
pub const GRID2D_INFERENCE_JOB_KIND: &str = "semio.infer";
pub const GRID2D_INFERENCE_TOOL_ID: &str = "s.wfc.grid2d.solve";
pub const GRID2D_INFERENCE_PAYLOAD_SCHEMA: &str = "s.wfc.grid2d.inference.request.v1";

/// 🧭️ Stable host roster identity for the ActionBus-owned cold solve route.
pub const fn grid2d_inference_metadata() -> semio_framework_plugin::ArtifactInferenceServiceMetadata {
    semio_framework_plugin::ArtifactInferenceServiceMetadata {
        owner: "wfc",
        artifact_kind: "s.wfc.grid2d",
        artifact_schema: "s.wfc.grid2d",
        artifact_schema_version: 1,
        inference_schema: GRID2D_INFERENCE_TOOL_ID,
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
    }
}

const MAX_GRID2D_TILES: usize = 4_096;
const MAX_GRID2D_RULES: usize = 262_144;
const MAX_GRID2D_CELLS: usize = 262_144;
const MAX_GRID2D_ID_BYTES: usize = 1_024;
const MAX_GRID2D_OUTPUT_BYTES: usize = 1 << 20;
const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;

/// 🧭️ The four-neighbour stencil's relation slots, in `Stencil2d::VonNeumann::offsets()` order —
/// `[(1,0), (-1,0), (0,1), (0,-1)]`, i.e. RIGHT, LEFT, BOTTOM, TOP with `y` growing downward.
fn relation_slot(direction: WfcDirection2d) -> usize {
    match direction {
        WfcDirection2d::Right => 0,
        WfcDirection2d::Left => 1,
        WfcDirection2d::Bottom => 2,
        WfcDirection2d::Top => 3,
    }
}
//#endregion 🔖️Identity

//#region 🔖️Protocol
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct Grid2dInferenceRequest {
    pub snapshot: Grid2dSnapshot,
    pub checkpoint: Option<Vec<u8>>,
}

/// 🏁 The solve's committed answer: one `[x, y, tileId]` row per unmasked cell, the satisfiability
/// verdict, and the PRE-propagation Shannon entropy of every unmasked cell's tile distribution.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid2dInferenceCommit {
    pub assignments: Vec<(u32, u32, String)>,
    pub contradiction: bool,
    pub entropy: Vec<(u32, u32, f64)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Grid2dInferenceStage {
    Tiles,
    Rules,
    Model,
    Topology,
    Fixed,
    Restore,
    Solve,
    MapCommit,
    EncodeCommit,
    Complete,
}

/// 🧵 Worker-owned parent transaction for grid-2d compile, solve, restore and authoritative mapping.
pub struct Grid2dInferenceJob {
    operation: semio_framework_job::Operation,
    snapshot: Grid2dSnapshot,
    stage: Grid2dInferenceStage,
    cursor: usize,
    tile_of: BTreeMap<String, usize>,
    tile_ids: Vec<String>,
    builder: Option<wfc::tiled::TiledModelBuilder>,
    tiles: Vec<wfc::ids::TileId>,
    relations: Vec<wfc::ids::RelationId>,
    model: Option<wfc::model::CompiledModel>,
    topology: Option<wfc::grid2d::Grid2dTopology>,
    fixed: Vec<(wfc::ids::NodeId, wfc::ids::PatternId)>,
    checkpoint: Option<Vec<u8>>,
    restore: Option<wfc::job::WfcRestore<wfc::grid2d::Grid2dTopology>>,
    child: Option<wfc::job::WfcJob<wfc::grid2d::Grid2dTopology>>,
    child_commit: Option<wfc::job::WfcCommit>,
    final_checkpoint: Option<semio_framework_job::RetainedJobPayload>,
    assignments: Vec<(u32, u32, String)>,
    contradiction: bool,
    output: Option<semio_framework_job::RetainedJobPayloadWriter>,
    rejected_output_page: Option<semio_framework_job::JobPayloadPageSource>,
    encode_phase: u8,
    encoded_entries: usize,
    encode_total: usize,
    preview_units: u64,
    last_preview_ms: Option<u64>,
}

impl Grid2dInferenceJob {
    fn new(mut operation: semio_framework_job::Operation, request: Grid2dInferenceRequest) -> Result<Self, String> {
        let snapshot = request.snapshot;
        let cells = (snapshot.width as usize).saturating_mul(snapshot.height as usize);
        if snapshot.width == 0
            || snapshot.height == 0
            || cells == 0
            || cells > MAX_GRID2D_CELLS
            || snapshot.tiles.len() > MAX_GRID2D_TILES
            || snapshot.rules.len() > MAX_GRID2D_RULES
            || snapshot.pinned.len() > MAX_GRID2D_CELLS
            || snapshot.masked.len() > MAX_GRID2D_CELLS
            || request.checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.len() > wfc::job::MAX_CHECKPOINT_BYTES)
        {
            return Err("grid2d-inference-admission-exceeded".into());
        }
        operation.seed = snapshot.seed;
        Ok(Self {
            operation,
            snapshot,
            stage: Grid2dInferenceStage::Tiles,
            cursor: 0,
            tile_of: BTreeMap::new(),
            tile_ids: Vec::new(),
            builder: Some(wfc::tiled::TiledModelBuilder::new()),
            tiles: Vec::new(),
            relations: Vec::new(),
            model: None,
            topology: None,
            fixed: Vec::new(),
            checkpoint: request.checkpoint,
            restore: None,
            child: None,
            child_commit: None,
            final_checkpoint: None,
            assignments: Vec::new(),
            contradiction: false,
            output: Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput)),
            rejected_output_page: None,
            encode_phase: 0,
            encoded_entries: 0,
            encode_total: 0,
            preview_units: 0,
            last_preview_ms: None,
        })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    fn validate_id(value: &str) -> Result<(), String> {
        if value.len() > MAX_GRID2D_ID_BYTES {
            Err("grid2d-inference-id-admission-exceeded".into())
        } else {
            Ok(())
        }
    }

    fn cell_count(&self) -> usize {
        (self.snapshot.width as usize) * (self.snapshot.height as usize)
    }

    fn progress(&self) -> (usize, usize) {
        match self.stage {
            Grid2dInferenceStage::Tiles => (self.cursor, self.snapshot.tiles.len()),
            Grid2dInferenceStage::Rules => (self.cursor, self.snapshot.rules.len()),
            Grid2dInferenceStage::Model | Grid2dInferenceStage::Topology => (0, 1),
            Grid2dInferenceStage::Fixed => (self.cursor, self.snapshot.pinned.len() + self.snapshot.masked.len()),
            Grid2dInferenceStage::Restore | Grid2dInferenceStage::Solve => (0, 1),
            Grid2dInferenceStage::MapCommit => (self.cursor, self.cell_count()),
            Grid2dInferenceStage::EncodeCommit => (self.encoded_entries, self.encode_total),
            Grid2dInferenceStage::Complete => (1, 1),
        }
    }

    fn emit_preview(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        let (completed, total) = self.progress();
        let sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence,
            Err(_) => return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
        };
        let mut preview = [0; 25];
        preview[..8].copy_from_slice(&sequence.to_le_bytes());
        preview[8..16].copy_from_slice(&(completed as u64).to_le_bytes());
        preview[16..24].copy_from_slice(&(total as u64).to_le_bytes());
        preview[24] = self.stage as u8;
        self.preview_units = 0;
        self.last_preview_ms = context.now_us().map(|now_us| now_us / 1_000);
        let payload = retained_payload(context, semio_framework_job::JobPayloadStream::Preview, &preview);
        semio_framework_job::StepOutcome::PreviewReady(payload)
    }

    fn preview_due(&self, now_ms: u64) -> bool {
        self.last_preview_ms.is_none() || self.preview_units >= PARENT_PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PARENT_PREVIEW_TIME_INTERVAL_MS)
    }

    fn is_masked(&self, x: u32, y: u32) -> bool {
        self.snapshot.masked.iter().any(|cell| cell.x == x && cell.y == y)
    }

    fn advance_compile(&mut self) -> Result<(), String> {
        match self.stage {
            Grid2dInferenceStage::Tiles => {
                if let Some(tile) = self.snapshot.tiles.get(self.cursor) {
                    Self::validate_id(&tile.id)?;
                    let weight = if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 };
                    let builder = self.builder.as_mut().expect("tiled model builder");
                    let id = builder.tile(weight);
                    self.tiles.push(id);
                    self.tile_of.insert(tile.id.clone(), self.cursor);
                    self.tile_ids.push(tile.id.clone());
                    self.cursor += 1;
                } else {
                    if self.snapshot.tiles.is_empty() {
                        self.contradiction = true;
                        self.encode_total = 0;
                        self.stage = Grid2dInferenceStage::EncodeCommit;
                        return Ok(());
                    }
                    let builder = self.builder.as_mut().expect("tiled model builder");
                    self.relations = wfc::grid2d::declare_stencil_relations_tiled(builder, &wfc::grid2d::Stencil2d::VonNeumann).map_err(|error| format!("{error:?}"))?;
                    self.cursor = 0;
                    self.stage = Grid2dInferenceStage::Rules;
                }
            }
            Grid2dInferenceStage::Rules => {
                if let Some(rule) = self.snapshot.rules.get(self.cursor) {
                    Self::validate_id(&rule.tile_a_id)?;
                    Self::validate_id(&rule.tile_b_id)?;
                    if rule.allowed {
                        if let (Some(&a), Some(&b)) = (self.tile_of.get(&rule.tile_a_id), self.tile_of.get(&rule.tile_b_id)) {
                            let relation = self.relations[relation_slot(rule.direction)];
                            let builder = self.builder.as_mut().expect("tiled model builder");
                            builder.allow_mirrored(relation, self.tiles[a], self.tiles[b]);
                        }
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Grid2dInferenceStage::Model;
                }
            }
            Grid2dInferenceStage::Model => {
                let builder = self.builder.take().expect("tiled model builder");
                self.model = Some(builder.compile().map_err(|error| format!("{error:?}"))?);
                self.stage = Grid2dInferenceStage::Topology;
            }
            Grid2dInferenceStage::Topology => {
                let width = self.snapshot.width as usize;
                let height = self.snapshot.height as usize;
                let mask = if self.snapshot.masked.is_empty() {
                    None
                } else {
                    let mut mask = vec![true; width * height];
                    for cell in &self.snapshot.masked {
                        if (cell.x as usize) < width && (cell.y as usize) < height {
                            mask[(cell.y as usize) * width + (cell.x as usize)] = false;
                        }
                    }
                    Some(mask)
                };
                let boundary_x = if self.snapshot.periodic_x { wfc::grid2d::Boundary::Wrap } else { wfc::grid2d::Boundary::Open };
                let boundary_y = if self.snapshot.periodic_y { wfc::grid2d::Boundary::Wrap } else { wfc::grid2d::Boundary::Open };
                let topology = wfc::grid2d::Grid2dTopology::new(width, height, &wfc::grid2d::Stencil2d::VonNeumann, self.relations.clone(), boundary_x, boundary_y, mask).map_err(|error| format!("{error:?}"))?;
                self.topology = Some(topology);
                self.cursor = 0;
                self.stage = Grid2dInferenceStage::Fixed;
            }
            Grid2dInferenceStage::Fixed => {
                let pinned_count = self.snapshot.pinned.len();
                if self.cursor < pinned_count {
                    let cell = &self.snapshot.pinned[self.cursor];
                    Self::validate_id(&cell.tile_id)?;
                    let topology = self.topology.as_ref().expect("grid topology");
                    if let (Some(node), Some(&tile)) = (topology.node_at(cell.x as usize, cell.y as usize), self.tile_of.get(&cell.tile_id)) {
                        if topology.is_active(cell.x as usize, cell.y as usize) {
                            self.fixed.push((node, wfc::ids::PatternId::from_index(tile)));
                        }
                    }
                    self.cursor += 1;
                } else if self.cursor < pinned_count + self.snapshot.masked.len() {
                    // 🕳️ An inactive cell still owns a domain in the job's dense state, so it is
                    // pinned to the placeholder pattern 0 and simply omitted from the commit.
                    let cell = self.snapshot.masked[self.cursor - pinned_count];
                    let topology = self.topology.as_ref().expect("grid topology");
                    if let Some(node) = topology.node_at(cell.x as usize, cell.y as usize) {
                        self.fixed.push((node, wfc::ids::PatternId::from_index(0)));
                    }
                    self.cursor += 1;
                } else {
                    let model = self.model.take().expect("compiled model");
                    let topology = self.topology.take().expect("compiled topology");
                    let fixed = std::mem::take(&mut self.fixed);
                    if let Some(checkpoint) = self.checkpoint.take() {
                        self.restore = Some(wfc::job::WfcRestore::new(self.operation, model, topology, wfc::job::WfcJobConfig::default(), None, fixed, checkpoint)?);
                        self.stage = Grid2dInferenceStage::Restore;
                    } else {
                        self.child = Some(wfc::job::WfcJob::new(self.operation, model, topology, wfc::job::WfcJobConfig::default(), None, fixed));
                        self.stage = Grid2dInferenceStage::Solve;
                    }
                    self.cursor = 0;
                }
            }
            _ => unreachable!("non-compile grid2d inference stage"),
        }
        Ok(())
    }

    fn map_one(&mut self) -> Result<(), String> {
        let width = self.snapshot.width as usize;
        if self.cursor < self.cell_count() {
            let x = (self.cursor % width) as u32;
            let y = (self.cursor / width) as u32;
            if !self.is_masked(x, y) {
                let commit = self.child_commit.as_ref().ok_or("grid2d-commit-missing")?;
                let pattern = usize::try_from(*commit.assignment.get(self.cursor).ok_or("grid2d-commit-missing-cell")?).map_err(|_| "grid2d-commit-pattern-capacity")?;
                let tile = self.tile_ids.get(pattern).ok_or("grid2d-commit-pattern-out-of-range")?;
                self.assignments.push((x, y, tile.clone()));
            }
            self.cursor += 1;
        } else {
            self.child_commit = None;
            self.encode_total = self.assignments.len() + self.cell_count();
            self.stage = Grid2dInferenceStage::EncodeCommit;
        }
        Ok(())
    }

    /// 🎲 Prior Shannon entropy of the tile-weight distribution for one cell — `0.0` for a pinned or
    /// masked cell (fully determined), else the whole-catalogue prior. SCOPE, honestly stated: this
    /// is the PRE-propagation entropy, not the narrowed per-cell domain entropy a live "which cell
    /// collapses next" overlay would want.
    fn cell_entropy(&self, x: u32, y: u32) -> f64 {
        if self.is_masked(x, y) || self.snapshot.pinned.iter().any(|cell| cell.x == x && cell.y == y) {
            return 0.0;
        }
        shannon_entropy_over_tiles(&self.snapshot)
    }

    /// 📤️ The exact bytes the CURRENT `(phase, cursor)` owes the commit stream, computed BEFORE any
    /// page is admitted so a refused admission never leaves a half-advanced cursor behind. `None`
    /// means the object is closed and nothing is left to write.
    fn encode_chunk(&self) -> Option<Vec<u8>> {
        match self.encode_phase {
            0 => Some(br#"{"assignments":["#.to_vec()),
            1 => Some(match self.assignments.get(self.cursor) {
                Some((x, y, tile)) => {
                    let separator = if self.cursor == 0 { "" } else { "," };
                    format!("{separator}[{x},{y},{}]", protocol::json::to_json_string(tile)).into_bytes()
                }
                None => format!(r#"],"contradiction":{},"entropy":["#, self.contradiction).into_bytes(),
            }),
            2 => Some(if self.cursor < self.cell_count() {
                let width = self.snapshot.width as usize;
                let x = (self.cursor % width) as u32;
                let y = (self.cursor / width) as u32;
                let separator = if self.cursor == 0 { "" } else { "," };
                format!("{separator}[{x},{y},{}]", protocol::json::to_json_string(&self.cell_entropy(x, y))).into_bytes()
            } else {
                b"]}".to_vec()
            }),
            _ => None,
        }
    }

    /// ⏭️ Moves `(phase, cursor)` past the chunk that was just committed.
    fn advance_encode(&mut self) {
        match self.encode_phase {
            0 => {
                self.encode_phase = 1;
                self.cursor = 0;
            }
            1 => {
                if self.cursor < self.assignments.len() {
                    self.cursor += 1;
                    self.encoded_entries += 1;
                } else {
                    self.encode_phase = 2;
                    self.cursor = 0;
                }
            }
            2 => {
                if self.cursor < self.cell_count() {
                    self.cursor += 1;
                    self.encoded_entries += 1;
                } else {
                    self.encode_phase = 3;
                    self.stage = Grid2dInferenceStage::Complete;
                }
            }
            _ => {}
        }
    }

    fn encode_one(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
        let Some(chunk) = self.encode_chunk() else { return Ok(true) };
        let source = self.rejected_output_page.take().unwrap_or_default();
        let writer = self.output.as_mut().ok_or("grid2d-output-writer-missing")?;
        let mut page = match context.admit_payload_page(writer, source) {
            Ok(page) => page,
            Err(rejected) => {
                self.rejected_output_page = Some(rejected.into_source());
                return Err("grid2d-inference-output-admission-exceeded".into());
            }
        };
        page.write(&chunk).map_err(|_| "grid2d-inference-output-page")?;
        page.commit();
        self.advance_encode();
        Ok(self.encode_phase == 3)
    }
}

impl semio_framework_job::InteractiveJob for Grid2dInferenceJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        use semio_framework_job::StepOutcome;
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, b"stale-grid2d-inference-operation");
            return StepOutcome::Fault(semio_framework_job::JobFault { detail });
        }
        loop {
            context.set_stage(match self.stage {
                Grid2dInferenceStage::Tiles => "grid2d.infer.tiles",
                Grid2dInferenceStage::Rules => "grid2d.infer.rules",
                Grid2dInferenceStage::Model => "grid2d.infer.model",
                Grid2dInferenceStage::Topology => "grid2d.infer.topology",
                Grid2dInferenceStage::Fixed => "grid2d.infer.fixed",
                Grid2dInferenceStage::Restore => "grid2d.infer.restore",
                Grid2dInferenceStage::Solve => "grid2d.infer.solve",
                Grid2dInferenceStage::MapCommit => "grid2d.infer.map-commit",
                Grid2dInferenceStage::EncodeCommit => "grid2d.infer.encode-commit",
                Grid2dInferenceStage::Complete => "grid2d.infer.complete",
            });
            match self.stage {
                Grid2dInferenceStage::Tiles | Grid2dInferenceStage::Rules | Grid2dInferenceStage::Model | Grid2dInferenceStage::Topology | Grid2dInferenceStage::Fixed => {
                    if let Err(error) = self.advance_compile() {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                Grid2dInferenceStage::Restore => {
                    let mut outcome = self.restore.as_mut().expect("restore job").step(context);
                    if matches!(outcome, StepOutcome::Complete(_)) {
                        // ♻️ The restore's own commit candidate owns retained pages; a `RetainedJobPayload`
                        // that reaches `Drop` without a one-page close panics the worker outright.
                        wfc::job::retire_outcome(&mut outcome);
                        self.child = self.restore.as_mut().expect("restore job").take_job();
                        if let Some(mut restore) = self.restore.take() {
                            wfc::job::close_job(&mut restore);
                        }
                        self.stage = Grid2dInferenceStage::Solve;
                        return self.emit_preview(context);
                    }
                    return outcome;
                }
                Grid2dInferenceStage::Solve => {
                    let mut outcome = self.child.as_mut().expect("WFC child").step(context);
                    match &outcome {
                        StepOutcome::Complete(_) => {
                            self.child_commit = self.child.as_mut().expect("WFC child").take_completed_commit();
                            self.contradiction = self.child_commit.is_none();
                        }
                        // 🩺 An unsatisfiable grid is an OUTCOME, never an error: the child publishes
                        // `wfc-unsatisfiable` as a fault, and this facet turns it into the
                        // contradiction verdict its own commit is contracted to carry.
                        StepOutcome::Fault(fault) if wfc::job::payload_bytes(&fault.detail) == b"wfc-unsatisfiable" => {
                            self.child_commit = None;
                            self.contradiction = true;
                        }
                        _ => return outcome,
                    }
                    // ♻️ Take the child's own final state before its pages are released, then run the
                    // close ladder: every job must be closed before it is dropped.
                    if let StepOutcome::Complete(candidate) = &mut outcome {
                        self.final_checkpoint = Some(std::mem::replace(&mut candidate.state, semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState)));
                    }
                    wfc::job::retire_outcome(&mut outcome);
                    if let Some(mut child) = self.child.take() {
                        wfc::job::close_job(&mut child);
                    }
                    self.cursor = 0;
                    self.stage = if self.contradiction { Grid2dInferenceStage::EncodeCommit } else { Grid2dInferenceStage::MapCommit };
                    if self.contradiction {
                        self.encode_total = self.cell_count();
                    }
                    return self.emit_preview(context);
                }
                Grid2dInferenceStage::MapCommit => {
                    if let Err(error) = self.map_one() {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                Grid2dInferenceStage::EncodeCommit => match self.encode_one(context) {
                    Ok(true) => {
                        let output = match self.output.take().expect("grid2d output writer").finish() {
                            Ok(output) => output,
                            Err(mut writer) => {
                                writer.begin_close();
                                self.output = Some(writer);
                                return StepOutcome::Yield;
                            }
                        };
                        return StepOutcome::Complete(semio_framework_job::CommitCandidate {
                            state: self.final_checkpoint.take().unwrap_or_else(|| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState)),
                            output,
                        });
                    }
                    Ok(false) => {}
                    Err(error) => {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                },
                Grid2dInferenceStage::Complete => unreachable!("complete returns immediately"),
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if context.now_us().is_some_and(|now_us| self.preview_due(now_us / 1_000)) {
                return self.emit_preview(context);
            }
            if context.should_yield() {
                return StepOutcome::Yield;
            }
        }
    }

    fn begin_close(&mut self) {
        if let Some(restore) = self.restore.as_mut() {
            restore.begin_close();
        }
        if let Some(child) = self.child.as_mut() {
            child.begin_close();
        }
        if let Some(output) = self.output.as_mut() {
            output.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if let Some(output) = self.output.as_mut() {
            if !output.terminal_is_empty() {
                return match output.close_step(maximum_items, maximum_bytes) {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.output = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(checkpoint) = self.final_checkpoint.as_mut() {
            if !checkpoint.terminal_is_empty() {
                return match checkpoint.close_step(maximum_items, maximum_bytes) {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.final_checkpoint = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(restore) = self.restore.as_mut() {
            match restore.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Complete if restore.terminal_is_empty() => {
                    self.restore = None;
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                step => return step,
            }
        }
        if let Some(child) = self.child.as_mut() {
            match child.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Complete if child.terminal_is_empty() => {
                    self.child = None;
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                step => return step,
            }
        }
        if self.rejected_output_page.is_some() {
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.rejected_output_page = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    /// 🚪️ Ownership ONLY — deliberately NOT gated on this job's own `closing` flag. The framework
    /// calls `begin_close()` itself at close stage 0 and answers `Blocked` for as long as
    /// `terminal_is_empty()` is false, so a `closing`-gated predicate makes every
    /// `while !terminal_is_empty()` driver spin forever (`🧵️job/🦀️.rs:2724`).
    fn terminal_is_empty(&self) -> bool {
        self.output.is_none() && self.final_checkpoint.is_none() && self.restore.is_none() && self.child.is_none() && self.rejected_output_page.is_none()
    }
}
//#endregion 🔖️Protocol

//#region 🔖️Factory
pub struct Grid2dInferenceJobFactory {
    keys: [semio_framework::ToolFactoryKey; 1],
}

impl Default for Grid2dInferenceJobFactory {
    fn default() -> Self {
        Self { keys: [semio_framework::ToolFactoryKey::new(GRID2D_INFERENCE_JOB_KIND, GRID2D_INFERENCE_TOOL_ID)] }
    }
}

impl semio_framework::ToolJobFactory for Grid2dInferenceJobFactory {
    type Payload = Grid2dInferenceRequest;
    type Job = Grid2dInferenceJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GRID2D_INFERENCE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(16 << 20, MAX_GRID2D_TILES + MAX_GRID2D_RULES + MAX_GRID2D_CELLS, 4_096, MAX_GRID2D_OUTPUT_BYTES, 7_500, 1, 1)
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Grid2dInferenceJob::new(operation, payload).map_err(semio_framework::ToolJobFactoryError::new)
    }

    fn create_job_from_wire(&mut self, operation: semio_framework_job::Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        let payload_text = std::str::from_utf8(payload).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("grid2d-inference-wire-decode:{error}")))?;
        let mut request: Grid2dInferenceRequest = protocol::json::from_json_str(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("grid2d-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        Grid2dInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

/// 💡️ Descriptor for the `s.wfc.grid2d.solve` inference — five handcrafted facet leaves.
pub fn grid2d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.wfc.grid2d.solve",
        inference: ::semio_framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}

pub fn register_grid2d_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(Grid2dInferenceJobFactory::default())
}
//#endregion 🔖️Factory

//#region 🔖️Headless
/// 🏁 Explicit headless adapter over the same complete parent job the public factory builds.
pub fn solve_with_job(snapshot: &Grid2dSnapshot) -> Result<Grid2dInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = Grid2dInferenceJob::new(operation, Grid2dInferenceRequest { snapshot: snapshot.clone(), checkpoint: None })?;
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig { site: "wfc.grid2d.inference.headless", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 2000 },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            let mut guard = 0u32;
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                guard += 1;
                assert!(guard < 1_000_000, "the rejected grid2d session's close ladder never reached terminal-empty");
            }
            return Err("grid2d-inference-headless-admission-rejected".into());
        }
    };
    loop {
        session.step().map_err(|error| format!("grid2d-inference-headless-contention:{error:?}"))?;
        let Some(mut outcome) = session.take_outcome() else { continue };
        let terminal = outcome.is_terminal();
        let payload_bytes = |payload: &semio_framework_job::RetainedJobPayload| {
            let mut bytes = Vec::with_capacity(payload.len());
            for index in 0..payload.page_count() {
                if let Some(page) = payload.page(index) {
                    bytes.extend_from_slice(page);
                }
            }
            bytes
        };
        let result = match &outcome {
            semio_framework_job::StepOutcome::Complete(candidate) => Some(
                std::str::from_utf8(&payload_bytes(&candidate.output))
                    .map_err(|error| format!("grid2d-invalid-commit:{error}"))
                    .and_then(|text| protocol::json::from_json_str::<Grid2dInferenceCommit>(text).map_err(|error| format!("grid2d-invalid-commit:{error}"))),
            ),
            semio_framework_job::StepOutcome::Cancelled => Some(Err("grid2d-inference-cancelled".into())),
            semio_framework_job::StepOutcome::Fault(fault) => Some(Err(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned())),
            semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) => None,
        };
        wfc::job::retire_outcome(&mut outcome);
        if terminal {
            session.begin_close();
            let mut guard = 0u32;
            while !session.terminal_is_empty() {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                guard += 1;
                assert!(guard < 1_000_000, "the grid2d session's close ladder never reached terminal-empty");
            }
            return result.expect("terminal grid2d inference outcome has result");
        }
        session.resume().map_err(|error| format!("grid2d-inference-headless-resume:{error:?}"))?;
    }
}
//#endregion 🔖️Headless

//#region 🔖️Solve
/// 🏁 The solved assignment, or `Unsolved` for every non-solved outcome (contradiction, budget,
/// cancellation) — see `Grid2dContradiction` for the dedicated satisfiability verdict.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum Grid2dSolveResult {
    #[default]
    Unsolved,
    Solved {
        assignments: Vec<(u32, u32, String)>,
    },
}

pub struct Grid2dSolve;

impl store::InferredField<Grid2dSnapshot> for Grid2dSolve {
    type Key = String;
    type Value = Grid2dSolveResult;

    const FIELD_ID: &'static str = "s.wfc.grid2d.inference.solve";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "width", "height", "periodicX", "periodicY", "tiles", "rules", "pinned", "masked"]
    }
    fn plan(_snapshot: &Grid2dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "grid2d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Grid2dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &Grid2dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(commit) if !commit.contradiction => Grid2dSolveResult::Solved { assignments: commit.assignments },
            _ => Grid2dSolveResult::Unsolved,
        }
    }
}
//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺 The satisfiability verdict on its own, so a caller who only needs "is this grid solvable at
/// all" never decodes a full assignment list to find out.
pub struct Grid2dContradiction;

impl store::InferredField<Grid2dSnapshot> for Grid2dContradiction {
    type Key = String;
    type Value = bool;

    const FIELD_ID: &'static str = "s.wfc.grid2d.inference.contradiction";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "width", "height", "periodicX", "periodicY", "tiles", "rules", "pinned", "masked"]
    }
    fn plan(_snapshot: &Grid2dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "grid2d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Grid2dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &Grid2dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        matches!(solve_with_job(snapshot), Ok(commit) if commit.contradiction) || solve_with_job(snapshot).is_err()
    }
}
//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 Per-cell prior Shannon entropy over the tile-weight distribution — `0.0` for a pinned or
/// masked cell (fully determined), else the whole-catalogue prior.
pub struct Grid2dEntropy;

impl store::InferredField<Grid2dSnapshot> for Grid2dEntropy {
    type Key = String;
    type Value = f64;

    const FIELD_ID: &'static str = "s.wfc.grid2d.inference.entropy";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["width", "height", "tiles", "pinned", "masked"]
    }
    fn plan(snapshot: &Grid2dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        (0..snapshot.height).flat_map(|y| (0..snapshot.width).map(move |x| store::InferenceStep { key: format!("{x},{y}"), parents: Vec::new() })).collect()
    }
    fn dep_input(snapshot: &Grid2dSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let mut bytes = key.as_bytes().to_vec();
        bytes.push(0);
        for tile in &snapshot.tiles {
            bytes.extend_from_slice(tile.id.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&tile.weight.to_le_bytes());
        }
        for cell in &snapshot.pinned {
            bytes.extend_from_slice(format!("p{},{}", cell.x, cell.y).as_bytes());
            bytes.push(0);
        }
        for cell in &snapshot.masked {
            bytes.extend_from_slice(format!("m{},{}", cell.x, cell.y).as_bytes());
            bytes.push(0);
        }
        bytes
    }
    fn compute(snapshot: &Grid2dSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let Some((x, y)) = key.split_once(',').and_then(|(x, y)| Some((x.parse::<u32>().ok()?, y.parse::<u32>().ok()?))) else {
            return 0.0;
        };
        if snapshot.masked.iter().any(|cell| cell.x == x && cell.y == y) || snapshot.pinned.iter().any(|cell| cell.x == x && cell.y == y) {
            return 0.0;
        }
        shannon_entropy_over_tiles(snapshot)
    }
}

/// 🎲 Shannon entropy of the whole tile catalogue's weight distribution.
pub fn shannon_entropy_over_tiles(snapshot: &Grid2dSnapshot) -> f64 {
    let weights: Vec<f64> = snapshot.tiles.iter().map(|tile| if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 }).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|weight| weight / total).filter(|share| *share > 0.0).map(|share| share * share.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
