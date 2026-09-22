//! 💡️ `s.wfc.grid3d` inferences — THE SOLVE ITSELF IS AN INFERENCE. `Grid3dSnapshot` only ever
//! persists the PROBLEM (extent, cell sizes, periodicity, tiles, rules, pins, masks); the
//! ASSIGNMENT, the contradiction verdict and the pre-propagation entropy map are all derived here,
//! never mutation-authored state.
//!
//! 🧱️ The engine wiring: `TiledModelBuilder` + `declare_stencil_relations_3d_tiled(Stencil3d::Face6)`
//! compile the closed adjacency allow-list into a `CompiledModel`, a `Grid3dTopology` carries the
//! mask and the per-axis boundaries, and the SAME resumable `WfcJob` interactive callers drive
//! solves it. Determinism: every input is read off the snapshot (seed included), every step is
//! fuel-bounded and watchdog-wrapped, no ambient randomness enters — so `DepHash` caching over
//! `Grid3dSolve`/`Grid3dContradiction`/`Grid3dEntropy` is sound.

use crate::schema::snapshot::{cell_key, Grid3dSnapshot};
use semio_s_plugin_wfc_engine as engine;
use std::collections::BTreeMap;

//#region 📦️RetainedPayload
/// 📦️ One single-page payload, with the job module's EXACT source handback on refusal. Dropping a
/// `JobPayloadRejectedPage` without taking its source back trips the job module's own lifecycle
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

//#region 🔖️Compile
pub const GRID3D_INFERENCE_JOB_KIND: &str = "semio.infer";
pub const GRID3D_INFERENCE_TOOL_ID: &str = "s.wfc.grid3d.solve";
pub const GRID3D_INFERENCE_PAYLOAD_SCHEMA: &str = "s.wfc.grid3d.inference.request.v1";

/// 🧭️ Stable host roster identity for the ActionBus-owned cold solve route.
pub const fn grid3d_inference_metadata() -> semio_framework_plugin::ArtifactInferenceServiceMetadata {
    semio_framework_plugin::ArtifactInferenceServiceMetadata {
        owner: "wfc",
        artifact_kind: "s.wfc.grid3d",
        artifact_schema: "s.wfc.grid3d",
        artifact_schema_version: 1,
        inference_schema: GRID3D_INFERENCE_TOOL_ID,
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
    }
}

const MAX_GRID3D_TILES: usize = 4_096;
const MAX_GRID3D_RULES: usize = 262_144;
const MAX_GRID3D_CELLS: usize = 262_144;
const MAX_GRID3D_ID_BYTES: usize = 1_024;
const MAX_GRID3D_OUTPUT_BYTES: usize = 1 << 20;
/// ⛽️ The headless adapter is a BLOCKING call (a render or an `InferredField::compute`), not an
/// interactive turn, so it buys whole solve phases per session step instead of single units. At one
/// unit per step a 48-cell grid spends minutes in session bookkeeping rather than in the solver —
/// measured, not assumed (ticket 26/09/18/EXTRACT-WFC-PLUGIN, A4). The encode stage still yields per
/// page, because page credit is per STEP and not per fuel unit.
const HEADLESS_FUEL_PER_STEP: u64 = 1 << 16;
const HEADLESS_STEP_BUDGET_US: u64 = 250_000;

const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;

#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct Grid3dInferenceRequest {
    pub snapshot: Grid3dSnapshot,
    pub checkpoint: Option<Vec<u8>>,
}

/// 🏁️ One solved cell. Masked cells never appear, so the row count IS the number of cells the grid
/// had to fill.
#[derive(Clone, Debug, Default, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid3dAssignment {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub tile_id: String,
}

/// 🏁️ The solve's commit payload — the authoritative assignment list plus the satisfiability
/// verdict. A grid with no consistent assignment is an ANSWER (`satisfiable: false`, no rows), never
/// an inference failure: the engine publishes `wfc-unsatisfiable` as a job FAULT
/// (`⚙️engine/💼️job/🦀️.rs`, `PublicationKind::Fault`), and this job intercepts exactly that detail and
/// turns it back into a verdict.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid3dInferenceCommit {
    pub satisfiable: bool,
    pub assignments: Vec<Grid3dAssignment>,
}

impl Default for Grid3dInferenceCommit {
    fn default() -> Self {
        Self { satisfiable: true, assignments: Vec::new() }
    }
}

/// 🩺️ The exact engine fault detail that means "this grid has no consistent assignment" rather than
/// "the solve broke".
const WFC_UNSATISFIABLE: &[u8] = b"wfc-unsatisfiable";

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Grid3dInferenceStage {
    Tiles,
    Rules,
    Model,
    Mask,
    Topology,
    Fixed,
    Restore,
    Solve,
    MapCommit,
    EncodeCommit,
    Complete,
}

/// 🧵️ Worker-owned parent transaction for grid3d compile, solve, restore and authoritative mapping.
pub struct Grid3dInferenceJob {
    operation: semio_framework_job::Operation,
    snapshot: Grid3dSnapshot,
    stage: Grid3dInferenceStage,
    cursor: usize,
    tile_of: BTreeMap<String, usize>,
    tile_ids: Vec<String>,
    weights: Vec<f64>,
    builder: Option<engine::tiled::TiledModelBuilder>,
    relations: Vec<engine::ids::RelationId>,
    model: Option<engine::model::CompiledModel>,
    mask: Vec<bool>,
    topology: Option<engine::grid3d::Grid3dTopology>,
    fixed: Vec<(engine::ids::NodeId, engine::ids::PatternId)>,
    checkpoint: Option<Vec<u8>>,
    restore: Option<engine::job::WfcRestore<engine::grid3d::Grid3dTopology>>,
    child: Option<engine::job::WfcJob<engine::grid3d::Grid3dTopology>>,
    child_commit: Option<engine::job::WfcCommit>,
    final_checkpoint: Option<semio_framework_job::RetainedJobPayload>,
    assignments: Vec<Grid3dAssignment>,
    satisfiable: bool,
    output: Option<semio_framework_job::RetainedJobPayloadWriter>,
    rejected_output_page: Option<semio_framework_job::JobPayloadPageSource>,
    output_started: bool,
    encoded_entries: usize,
    encode_total: usize,
    preview_units: u64,
    last_preview_ms: Option<u64>,
}

/// 🧊️ The cell count this snapshot declares, saturating so an absurd extent is refused by the
/// admission check rather than overflowing.
fn cell_count(snapshot: &Grid3dSnapshot) -> usize {
    (snapshot.width as usize).saturating_mul(snapshot.height as usize).saturating_mul(snapshot.depth as usize)
}

/// 🧭️ The per-axis boundary a periodicity flag selects.
fn boundary(periodic: bool) -> engine::grid2d::Boundary {
    if periodic {
        engine::grid2d::Boundary::Wrap
    } else {
        engine::grid2d::Boundary::Open
    }
}

/// 🚪️ Runs one owned job through the engine's close ladder and drops it. A `WfcJob` (or a
/// `WfcRestore`) holds retained payload pages, and an ordinary `Drop` on one of those ABORTS the
/// process ("RetainedJobPayload requires one-page close to terminal-empty") — so no child of this
/// parent is ever released any other way.
fn retire(payload: &mut semio_framework_job::RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

/// 🧾️ One retained payload's bytes, concatenated across its pages.
fn payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).flat_map(|index| payload.page(index).map(<[u8]>::to_vec).unwrap_or_default()).collect()
}

fn close_owned<T: semio_framework_job::InteractiveJob>(mut job: T) {
    job.begin_close();
    while !job.terminal_is_empty() {
        job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

impl Grid3dInferenceJob {
    fn new(mut operation: semio_framework_job::Operation, request: Grid3dInferenceRequest) -> Result<Self, String> {
        let snapshot = request.snapshot;
        let cells = cell_count(&snapshot);
        if snapshot.tiles.len() > MAX_GRID3D_TILES
            || snapshot.rules.len() > MAX_GRID3D_RULES
            || cells > MAX_GRID3D_CELLS
            || snapshot.width == 0
            || snapshot.height == 0
            || snapshot.depth == 0
            || request.checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.len() > engine::job::MAX_CHECKPOINT_BYTES)
        {
            return Err("grid3d-inference-admission-exceeded".into());
        }
        operation.seed = snapshot.seed;
        Ok(Self {
            operation,
            snapshot,
            stage: Grid3dInferenceStage::Tiles,
            cursor: 0,
            tile_of: BTreeMap::new(),
            tile_ids: Vec::new(),
            weights: Vec::new(),
            builder: None,
            relations: Vec::new(),
            model: None,
            mask: Vec::new(),
            topology: None,
            fixed: Vec::new(),
            checkpoint: request.checkpoint,
            restore: None,
            child: None,
            child_commit: None,
            final_checkpoint: None,
            assignments: Vec::new(),
            satisfiable: true,
            output: Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput)),
            rejected_output_page: None,
            output_started: false,
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
        if value.len() > MAX_GRID3D_ID_BYTES {
            Err("grid3d-inference-id-admission-exceeded".into())
        } else {
            Ok(())
        }
    }

    fn progress(&self) -> (usize, usize) {
        match self.stage {
            Grid3dInferenceStage::Tiles => (self.cursor, self.snapshot.tiles.len()),
            Grid3dInferenceStage::Rules | Grid3dInferenceStage::Model => (self.cursor, self.snapshot.rules.len()),
            Grid3dInferenceStage::Mask => (self.cursor, self.snapshot.masked.len()),
            Grid3dInferenceStage::Topology => (0, 1),
            Grid3dInferenceStage::Fixed => (self.cursor, self.snapshot.pinned.len()),
            Grid3dInferenceStage::Restore | Grid3dInferenceStage::Solve => (0, 1),
            Grid3dInferenceStage::MapCommit => (self.cursor, cell_count(&self.snapshot)),
            Grid3dInferenceStage::EncodeCommit => (self.encoded_entries, self.encode_total),
            Grid3dInferenceStage::Complete => (1, 1),
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

    fn advance_compile(&mut self) -> Result<(), String> {
        match self.stage {
            Grid3dInferenceStage::Tiles => {
                if let Some(tile) = self.snapshot.tiles.get(self.cursor) {
                    Self::validate_id(&tile.id)?;
                    self.tile_of.insert(tile.id.clone(), self.cursor);
                    self.tile_ids.push(tile.id.clone());
                    self.weights.push(if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 });
                    self.cursor += 1;
                } else if self.snapshot.tiles.is_empty() {
                    self.stage = Grid3dInferenceStage::EncodeCommit;
                } else {
                    let mut builder = engine::tiled::TiledModelBuilder::new();
                    for weight in &self.weights {
                        builder.tile(*weight);
                    }
                    self.relations = engine::grid3d::declare_stencil_relations_3d_tiled(&mut builder, &engine::grid3d::Stencil3d::Face6).map_err(|error| format!("{error:?}"))?;
                    self.builder = Some(builder);
                    self.cursor = 0;
                    self.stage = Grid3dInferenceStage::Rules;
                }
            }
            Grid3dInferenceStage::Rules => {
                if let Some(rule) = self.snapshot.rules.get(self.cursor) {
                    Self::validate_id(&rule.tile_a_id)?;
                    Self::validate_id(&rule.tile_b_id)?;
                    if let (Some(&a), Some(&b)) = (self.tile_of.get(&rule.tile_a_id), self.tile_of.get(&rule.tile_b_id)) {
                        let builder = self.builder.as_mut().ok_or("grid3d-model-builder-missing")?;
                        let forward = self.relations[rule.direction.stencil_index()];
                        let backward = self.relations[rule.direction.opposite().stencil_index()];
                        let (a, b) = (engine::ids::TileId::from_index(a), engine::ids::TileId::from_index(b));
                        if rule.allowed {
                            builder.allow(forward, a, b);
                            builder.allow(backward, b, a);
                        } else {
                            builder.deny(forward, a, b);
                            builder.deny(backward, b, a);
                        }
                    }
                    self.cursor += 1;
                } else {
                    self.stage = Grid3dInferenceStage::Model;
                }
            }
            Grid3dInferenceStage::Model => {
                let builder = self.builder.take().ok_or("grid3d-model-builder-missing")?;
                self.model = Some(builder.compile().map_err(|error| format!("{error:?}"))?);
                self.mask = vec![true; cell_count(&self.snapshot)];
                self.cursor = 0;
                self.stage = Grid3dInferenceStage::Mask;
            }
            Grid3dInferenceStage::Mask => {
                if let Some(cell) = self.snapshot.masked.get(self.cursor) {
                    if cell.x < self.snapshot.width && cell.y < self.snapshot.height && cell.z < self.snapshot.depth {
                        let index = (cell.z as usize) * (self.snapshot.width as usize) * (self.snapshot.height as usize) + (cell.y as usize) * (self.snapshot.width as usize) + cell.x as usize;
                        self.mask[index] = false;
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = Grid3dInferenceStage::Topology;
                }
            }
            Grid3dInferenceStage::Topology => {
                let mask = (!self.snapshot.masked.is_empty()).then(|| std::mem::take(&mut self.mask));
                self.topology = Some(
                    engine::grid3d::Grid3dTopology::new(
                        self.snapshot.width as usize,
                        self.snapshot.height as usize,
                        self.snapshot.depth as usize,
                        &engine::grid3d::Stencil3d::Face6,
                        self.relations.clone(),
                        boundary(self.snapshot.periodic_x),
                        boundary(self.snapshot.periodic_y),
                        boundary(self.snapshot.periodic_z),
                        mask,
                    )
                    .map_err(|error| format!("{error:?}"))?,
                );
                self.cursor = 0;
                self.stage = Grid3dInferenceStage::Fixed;
            }
            Grid3dInferenceStage::Fixed => {
                if let Some(cell) = self.snapshot.pinned.get(self.cursor) {
                    Self::validate_id(&cell.tile_id)?;
                    if let (Some(topology), Some(&tile)) = (self.topology.as_ref(), self.tile_of.get(&cell.tile_id)) {
                        if let Some(node) = topology.node_at(cell.x as usize, cell.y as usize, cell.z as usize) {
                            self.fixed.push((node, engine::ids::PatternId::from_index(tile)));
                        }
                    }
                    self.cursor += 1;
                } else {
                    let model = self.model.take().ok_or("grid3d-compiled-model-missing")?;
                    let topology = self.topology.take().ok_or("grid3d-compiled-topology-missing")?;
                    let fixed = std::mem::take(&mut self.fixed);
                    if let Some(checkpoint) = self.checkpoint.take() {
                        self.restore = Some(engine::job::WfcRestore::new(self.operation, model, topology.clone(), engine::job::WfcJobConfig::default(), None, fixed, checkpoint)?);
                        self.topology = Some(topology);
                        self.stage = Grid3dInferenceStage::Restore;
                    } else {
                        self.topology = Some(topology.clone());
                        self.child = Some(engine::job::WfcJob::new(self.operation, model, topology, engine::job::WfcJobConfig::default(), None, fixed));
                        self.stage = Grid3dInferenceStage::Solve;
                    }
                    self.cursor = 0;
                }
            }
            _ => unreachable!("non-compile wfc grid3d inference stage"),
        }
        Ok(())
    }

    fn map_one(&mut self) -> Result<(), String> {
        let commit = self.child_commit.as_ref().ok_or("grid3d-commit-missing")?;
        let cells = cell_count(&self.snapshot);
        if self.cursor < cells {
            let width = self.snapshot.width as usize;
            let height = self.snapshot.height as usize;
            let plane = width * height;
            let z = self.cursor / plane;
            let rest = self.cursor % plane;
            let (x, y) = (rest % width, rest / width);
            let masked = self.snapshot.masked.iter().any(|cell| (cell.x as usize, cell.y as usize, cell.z as usize) == (x, y, z));
            if !masked {
                let pattern = usize::try_from(*commit.assignment.get(self.cursor).ok_or("grid3d-commit-missing-cell")?).map_err(|_| "grid3d-commit-pattern-capacity")?;
                let tile = self.tile_ids.get(pattern).ok_or("grid3d-commit-pattern-out-of-range")?;
                self.assignments.push(Grid3dAssignment { x: x as u32, y: y as u32, z: z as u32, tile_id: tile.clone() });
            }
            self.cursor += 1;
        } else {
            self.child_commit = None;
            self.encode_total = self.assignments.len();
            self.assignments.reverse();
            self.stage = Grid3dInferenceStage::EncodeCommit;
        }
        Ok(())
    }

    fn encode_one(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
        let source = self.rejected_output_page.take().unwrap_or_default();
        let writer = self.output.as_mut().ok_or("grid3d-output-writer-missing")?;
        let mut page = match context.admit_payload_page(writer, source) {
            Ok(page) => page,
            Err(rejected) => {
                self.rejected_output_page = Some(rejected.into_source());
                return Err("grid3d-inference-output-admission-exceeded".into());
            }
        };
        if !self.output_started {
            let head = format!(r#"{{"satisfiable":{},"assignments":["#, self.satisfiable);
            page.write(head.as_bytes()).map_err(|_| "grid3d-inference-output-page")?;
            page.commit();
            self.output_started = true;
            return Ok(false);
        }
        if let Some(row) = self.assignments.pop() {
            let row = protocol::json::to_json_string(&row);
            if self.encoded_entries != 0 {
                page.write(b",").map_err(|_| "grid3d-inference-output-page")?;
            }
            page.write(row.as_bytes()).map_err(|_| "grid3d-inference-output-page")?;
            page.commit();
            self.encoded_entries += 1;
            return Ok(false);
        }
        page.write(b"]}").map_err(|_| "grid3d-inference-output-page")?;
        page.commit();
        self.stage = Grid3dInferenceStage::Complete;
        Ok(true)
    }
}

impl semio_framework_job::InteractiveJob for Grid3dInferenceJob {
    /// 🪜️ One bounded stage of the solve, with two retained-ownership laws the stages depend on. A
    /// `CommitCandidate` carries TWO retained payloads: the checkpoint becomes this job's own state,
    /// while the child's raw output is not the shape this artifact commits, so the `Solve` stage
    /// RETIRES it rather than dropping it — an ordinary `Drop` on a retained payload aborts the
    /// process. And a `StepContext` grants exactly ONE payload page per step (`payload_page_granted`),
    /// so the encode stage yields after every committed page instead of looping: a second
    /// `admit_payload_page` in the same step is refused as `OpportunityExhausted`, and the refusal's
    /// own fault detail then cannot be admitted either, which is how that surfaces as an EMPTY fault.
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        use semio_framework_job::StepOutcome;
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, b"stale-wfc-grid3d-inference-operation");
            return StepOutcome::Fault(semio_framework_job::JobFault { detail });
        }
        loop {
            context.set_stage(match self.stage {
                Grid3dInferenceStage::Tiles => "wfc.grid3d.infer.tiles",
                Grid3dInferenceStage::Rules => "wfc.grid3d.infer.rules",
                Grid3dInferenceStage::Model => "wfc.grid3d.infer.model",
                Grid3dInferenceStage::Mask => "wfc.grid3d.infer.mask",
                Grid3dInferenceStage::Topology => "wfc.grid3d.infer.topology",
                Grid3dInferenceStage::Fixed => "wfc.grid3d.infer.fixed",
                Grid3dInferenceStage::Restore => "wfc.grid3d.infer.restore",
                Grid3dInferenceStage::Solve => "wfc.grid3d.infer.solve",
                Grid3dInferenceStage::MapCommit => "wfc.grid3d.infer.map-commit",
                Grid3dInferenceStage::EncodeCommit => "wfc.grid3d.infer.encode-commit",
                Grid3dInferenceStage::Complete => "wfc.grid3d.infer.complete",
            });
            match self.stage {
                Grid3dInferenceStage::Tiles | Grid3dInferenceStage::Rules | Grid3dInferenceStage::Model | Grid3dInferenceStage::Mask | Grid3dInferenceStage::Topology | Grid3dInferenceStage::Fixed => {
                    if let Err(error) = self.advance_compile() {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                Grid3dInferenceStage::Restore => {
                    let mut outcome = self.restore.as_mut().expect("restore job").step(context);
                    return match &mut outcome {
                        StepOutcome::Complete(candidate) => {
                            retire(&mut candidate.state);
                            retire(&mut candidate.output);
                            let mut restore = self.restore.take().expect("restore job");
                            self.child = restore.take_job();
                            close_owned(restore);
                            self.stage = Grid3dInferenceStage::Solve;
                            self.emit_preview(context)
                        }
                        _ => outcome,
                    };
                }
                Grid3dInferenceStage::Solve => {
                    let mut outcome = self.child.as_mut().expect("WFC child").step(context);
                    match &mut outcome {
                        StepOutcome::Complete(candidate) => {
                            retire(&mut candidate.output);
                            self.final_checkpoint = Some(std::mem::replace(&mut candidate.state, semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState)));
                            let mut child = self.child.take().expect("WFC child");
                            self.child_commit = child.take_completed_commit();
                            close_owned(child);
                            self.cursor = 0;
                            self.stage = Grid3dInferenceStage::MapCommit;
                            return self.emit_preview(context);
                        }
                        StepOutcome::Fault(fault) if payload_bytes(&fault.detail) == WFC_UNSATISFIABLE => {
                            retire(&mut fault.detail);
                            close_owned(self.child.take().expect("WFC child"));
                            self.satisfiable = false;
                            self.encode_total = 0;
                            self.stage = Grid3dInferenceStage::EncodeCommit;
                            return self.emit_preview(context);
                        }
                        _ => {}
                    }
                    return outcome;
                }
                Grid3dInferenceStage::MapCommit => {
                    if let Err(error) = self.map_one() {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                Grid3dInferenceStage::EncodeCommit => match self.encode_one(context) {
                    Ok(true) => {
                        let output = match self.output.take().expect("grid3d output writer").finish() {
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
                    Ok(false) => return StepOutcome::Yield,
                    Err(error) => {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                },
                Grid3dInferenceStage::Complete => unreachable!("complete returns immediately"),
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

    /// 🚪️ Ownership only — never gated on this job's own `closing` flag. The framework calls
    /// `begin_close` itself at close stage 0 and answers `Blocked` for as long as this reports false,
    /// so a flag-gated answer makes every `while !terminal_is_empty()` driver spin forever.
    fn terminal_is_empty(&self) -> bool {
        self.output.is_none() && self.final_checkpoint.is_none() && self.restore.is_none() && self.child.is_none() && self.rejected_output_page.is_none()
    }
}

pub struct Grid3dInferenceJobFactory {
    keys: [semio_framework::ToolFactoryKey; 1],
}

impl Default for Grid3dInferenceJobFactory {
    fn default() -> Self {
        Self { keys: [semio_framework::ToolFactoryKey::new(GRID3D_INFERENCE_JOB_KIND, GRID3D_INFERENCE_TOOL_ID)] }
    }
}

impl semio_framework::ToolJobFactory for Grid3dInferenceJobFactory {
    type Payload = Grid3dInferenceRequest;
    type Job = Grid3dInferenceJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GRID3D_INFERENCE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(16 << 20, MAX_GRID3D_TILES + MAX_GRID3D_RULES + MAX_GRID3D_CELLS, 4_096, MAX_GRID3D_OUTPUT_BYTES, 7_500, 1, 1)
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Grid3dInferenceJob::new(operation, payload).map_err(semio_framework::ToolJobFactoryError::new)
    }

    fn create_job_from_wire(&mut self, operation: semio_framework_job::Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        let payload_text = std::str::from_utf8(payload).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("grid3d-inference-wire-decode:{error}")))?;
        let mut request: Grid3dInferenceRequest = protocol::json::from_json_str(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("grid3d-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        Grid3dInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

/// 💡️ Descriptor for the `s.wfc.grid3d` solve inference — five handcrafted facet leaves.
pub fn grid3d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.wfc.grid3d.solve",
        inference: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}

pub fn register_grid3d_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(Grid3dInferenceJobFactory::default())
}

/// 🏁️ Explicit headless adapter over the same complete parent job the public factory hands out.
pub fn solve_with_job(snapshot: &Grid3dSnapshot) -> Result<Grid3dInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = Grid3dInferenceJob::new(operation, Grid3dInferenceRequest { snapshot: snapshot.clone(), checkpoint: None })?;
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig { site: "wfc.grid3d.inference.headless", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: HEADLESS_FUEL_PER_STEP, step_budget_us: HEADLESS_STEP_BUDGET_US },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return Err("grid3d-inference-headless-admission-rejected".into());
        }
    };
    loop {
        session.step().map_err(|error| format!("grid3d-inference-headless-contention:{error:?}"))?;
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
                    .map_err(|error| format!("grid3d-invalid-commit:{error}"))
                    .and_then(|text| protocol::json::from_json_str::<Grid3dInferenceCommit>(text).map_err(|error| format!("grid3d-invalid-commit:{error}"))),
            ),
            semio_framework_job::StepOutcome::Cancelled => Some(Err("grid3d-inference-cancelled".into())),
            semio_framework_job::StepOutcome::Fault(fault) => Some(Err(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned())),
            semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) => None,
        };
        while !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        if terminal {
            session.begin_close();
            for _ in 0..2_000_000 {
                if session.terminal_is_empty() {
                    return result.expect("terminal wfc grid3d inference outcome has result");
                }
                assert_ne!(session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::WorkerJobCloseStep::Blocked, "a locally owned headless session close has no external owner");
            }
            panic!("wfc grid3d headless session did not close");
        }
        session.resume().map_err(|error| format!("grid3d-inference-headless-resume:{error:?}"))?;
    }
}

/// 🏁️ The solve as any caller (editor preview window included) reaches it: a plain function of the
/// persisted problem, never a cached side effect.
pub fn solve(snapshot: &Grid3dSnapshot) -> Result<Grid3dInferenceCommit, String> {
    solve_with_job(snapshot)
}
//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁️ The solved assignment, or `Unsolved` for every non-solved outcome (contradiction, budget,
/// cancellation) — see `Grid3dContradiction` for the dedicated satisfiability verdict.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum Grid3dSolveResult {
    #[default]
    Unsolved,
    Solved {
        assignments: Vec<Grid3dAssignment>,
    },
}

pub struct Grid3dSolve;

impl store::InferredField<Grid3dSnapshot> for Grid3dSolve {
    type Key = String;
    type Value = Grid3dSolveResult;

    const FIELD_ID: &'static str = "s.wfc.grid3d.inference.solve";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "width", "height", "depth", "periodicX", "periodicY", "periodicZ", "tiles", "rules", "pinned", "masked"]
    }
    fn plan(_snapshot: &Grid3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "grid3d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Grid3dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &Grid3dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(solution) if solution.satisfiable => Grid3dSolveResult::Solved { assignments: solution.assignments },
            _ => Grid3dSolveResult::Unsolved,
        }
    }
}
//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺️ The satisfiability verdict on its own, so a caller who only needs "is this grid solvable at
/// all" never has to decode a full assignment list to find out.
pub struct Grid3dContradiction;

impl store::InferredField<Grid3dSnapshot> for Grid3dContradiction {
    type Key = String;
    type Value = bool;

    const FIELD_ID: &'static str = "s.wfc.grid3d.inference.contradiction";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "width", "height", "depth", "periodicX", "periodicY", "periodicZ", "tiles", "rules", "pinned", "masked"]
    }
    fn plan(_snapshot: &Grid3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "grid3d".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &Grid3dSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &Grid3dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        solve_with_job(snapshot).is_ok_and(|commit| commit.satisfiable)
    }
}
//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲️ Per-cell Shannon entropy of the tile WEIGHT distribution — `0.0` for a pinned or masked cell
/// (fully determined, or not in the topology at all), else the prior entropy over every tile's
/// weight. SCOPE, honestly stated: this is the PRIOR entropy BEFORE arc consistency narrows any
/// cell's domain — a real WFC heuristic, but not the post-propagation entropy a live "which cell
/// collapses next" overlay would want.
pub struct Grid3dEntropy;

impl store::InferredField<Grid3dSnapshot> for Grid3dEntropy {
    type Key = String;
    type Value = f64;

    const FIELD_ID: &'static str = "s.wfc.grid3d.inference.entropy";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["width", "height", "depth", "tiles", "pinned", "masked"]
    }
    fn plan(snapshot: &Grid3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        let mut steps = Vec::with_capacity(cell_count(snapshot));
        for z in 0..snapshot.depth {
            for y in 0..snapshot.height {
                for x in 0..snapshot.width {
                    steps.push(store::InferenceStep { key: cell_key(x, y, z), parents: Vec::new() });
                }
            }
        }
        steps
    }
    fn dep_input(snapshot: &Grid3dSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let mut bytes = key.as_bytes().to_vec();
        bytes.push(0);
        bytes.push(u8::from(snapshot.pinned.iter().any(|cell| &cell_key(cell.x, cell.y, cell.z) == key)));
        bytes.push(u8::from(snapshot.masked.iter().any(|cell| &cell_key(cell.x, cell.y, cell.z) == key)));
        for tile in &snapshot.tiles {
            bytes.extend_from_slice(tile.id.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&tile.weight.to_le_bytes());
        }
        bytes
    }
    fn compute(snapshot: &Grid3dSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let determined = snapshot.pinned.iter().any(|cell| &cell_key(cell.x, cell.y, cell.z) == key) || snapshot.masked.iter().any(|cell| &cell_key(cell.x, cell.y, cell.z) == key);
        if determined {
            return 0.0;
        }
        shannon_entropy_over_tiles(snapshot)
    }
}

/// 🎲️ Shannon entropy of the tile weight distribution, the same weighted-distribution math
/// `engine::weights::WeightTable` encodes.
pub fn shannon_entropy_over_tiles(snapshot: &Grid3dSnapshot) -> f64 {
    let weights: Vec<f64> = snapshot.tiles.iter().map(|tile| if tile.weight.is_finite() && tile.weight > 0.0 { tile.weight } else { 1.0 }).collect();
    let total: f64 = weights.iter().sum();
    if weights.is_empty() || total <= 0.0 {
        return 0.0;
    }
    -weights.iter().map(|weight| weight / total).filter(|probability| *probability > 0.0).map(|probability| probability * probability.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
