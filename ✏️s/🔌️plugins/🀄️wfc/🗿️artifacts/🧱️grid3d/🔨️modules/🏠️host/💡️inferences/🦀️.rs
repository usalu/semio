//! 🏠️ Inference admission, publication and retained job lifecycle.
use semio_framework_job::InteractiveJob;
use crate::schema::snapshot::{cell_key, Grid3dSnapshot};
use semio_s_plugin_wfc_engine as engine;
use std::collections::BTreeMap;
use crate::standards::v1::subsets::any::schema::inferences::*;

pub const GRID3D_INFERENCE_JOB_KIND: &str = "semio.infer";

pub const GRID3D_INFERENCE_TOOL_ID: &str = "s.wfc.grid3d.solve";

pub const GRID3D_INFERENCE_PAYLOAD_SCHEMA: &str = "s.wfc.grid3d.inference.request.v1";

/// 📜️ The PUBLISHED request schema of `s.wfc.grid3d.solve` — what a client has to send, readable
/// from `inference_list`/`capabilities_describe` without reading a line of this crate. Authored
/// here rather than as a facet leaf because the facet leaf beside it (`🔣️.json`) is the RESULT
/// schema; a request and its result are two schemas, and publishing only one was the gap
/// (`📓️ce3-four-mcp-gates-green.md` §3.3).
pub const GRID3D_INFERENCE_REQUEST_SCHEMA: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://json.schemas.assets.semio-tech.com/s/wfc/grid3d/1/any/inference.request.json",
  "title": "Grid3dInferenceRequest",
  "type": "object",
  "additionalProperties": false,
  "oneOf": [{ "required": ["document"] }, { "required": ["snapshot"] }],
  "properties": {
    "document": {
      "type": "object",
      "description": "The artifact this solve runs on, bound by the gateway from `inference_run`'s `artifactId` — a caller names the artifact, never these bytes.",
      "additionalProperties": false,
      "required": ["pack", "spr"],
      "properties": { "pack": { "type": "string", "contentEncoding": "base64" }, "spr": { "type": "string", "contentEncoding": "base64" } }
    },
    "snapshot": { "type": "object", "description": "The problem stated in full instead of read from an artifact: the grid3d solve's own document shape." },
    "checkpoint": { "type": "array", "description": "A previous run's checkpoint bytes, to resume instead of restart.", "items": { "type": "integer", "minimum": 0, "maximum": 255 } }
  }
}"#;

/// 📜️ The whole published contract for `s.wfc.grid3d.solve`: request schema, result schema, the
/// unit its bounded job counts, and the artifact binding that makes it callable at all.
pub const GRID3D_INFERENCE_CONTRACT: semio_framework_plugin::ArtifactInferencePayloadContract = semio_framework_plugin::ArtifactInferencePayloadContract {
    payload_schema_id: GRID3D_INFERENCE_PAYLOAD_SCHEMA,
    input_schema: GRID3D_INFERENCE_REQUEST_SCHEMA,
    output_schema: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json"),
    progress_unit: "cells",
    artifact_binding: Some(semio_framework_plugin::ArtifactInferenceDocumentBinding { field: "document", encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64, required: true }),
    commit: None,
};

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
        payload: Some(GRID3D_INFERENCE_CONTRACT),
    }
}

#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
pub struct Grid3dInferenceRequest {
    /// 📸️ The problem, stated in full by the caller. Mutually exclusive with `document`: exactly one
    /// of the two says which snapshot this solve runs over.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Grid3dSnapshot>,
    /// 🔗️ The ARTIFACT this solve runs on, as its own canonical `pack`/`spr` pair. This is the
    /// field `GRID3D_INFERENCE_CONTRACT`'s artifact binding names, so an agent that calls
    /// `inference_run` with `artifactId` never has to state a snapshot it could not type: the
    /// gateway binds the document here and the guest decodes it into its own snapshot below.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<semio_framework_plugin::ArtifactDocumentPayload>,
    pub checkpoint: Option<Vec<u8>>,
}

impl Grid3dInferenceRequest {
    /// 📸️ The snapshot this request states, from whichever of its two carriers is present — the
    /// ONE resolution path every caller of this inference takes.
    pub fn resolve_snapshot(&self) -> Result<Grid3dSnapshot, String> {
        match (&self.snapshot, &self.document) {
            (Some(snapshot), _) => Ok(snapshot.clone()),
            (None, Some(document)) => document.settled_snapshot::<Grid3dSnapshot, crate::Grid3dMutation>(),
            (None, None) => Err("s.wfc.grid3d-inference-no-snapshot:state `snapshot`, or name the artifact with `artifactId` so `document` is bound".into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
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
    pub(crate) operation: semio_framework_job::Operation,
    pub(crate) snapshot: Grid3dSnapshot,
    pub(crate) stage: Grid3dInferenceStage,
    pub(crate) cursor: usize,
    pub(crate) tile_of: BTreeMap<String, usize>,
    pub(crate) tile_ids: Vec<String>,
    pub(crate) weights: Vec<f64>,
    pub(crate) builder: Option<engine::tiled::TiledModelBuilder>,
    pub(crate) relations: Vec<engine::ids::RelationId>,
    pub(crate) model: Option<engine::model::CompiledModel>,
    pub(crate) mask: Vec<bool>,
    pub(crate) topology: Option<engine::grid3d::Grid3dTopology>,
    pub(crate) fixed: Vec<(engine::ids::NodeId, engine::ids::PatternId)>,
    pub(crate) checkpoint: Option<Vec<u8>>,
    pub(crate) restore: Option<engine::job::WfcRestore<engine::grid3d::Grid3dTopology>>,
    pub(crate) child: Option<engine::job::WfcJob<engine::grid3d::Grid3dTopology>>,
    pub(crate) child_commit: Option<engine::job::WfcCommit>,
    pub(crate) assignments: Vec<Grid3dAssignment>,
    pub(crate) satisfiable: bool,
    pub(crate) output: Option<semio_framework_job::RetainedJobPayloadWriter>,
    pub(crate) rejected_output_page: Option<semio_framework_job::JobPayloadPageSource>,
    pub(crate) output_started: bool,
    pub(crate) encoded_entries: usize,
    pub(crate) encode_total: usize,
    pub(crate) preview_units: u64,
    pub(crate) last_preview_ms: Option<u64>,
    pub(crate) publication: Option<Box<engine::job::Publication>>,
    pub(crate) lent: Grid3dInferenceJobLent,
    pub(crate) restore_done: bool,
    pub(crate) child_end: Option<InferenceChildEnd>,
    pub(crate) output_payload: Option<semio_framework_job::RetainedJobPayload>,
}

impl Grid3dInferenceJob {
    pub(crate) fn new(mut operation: semio_framework_job::Operation, request: Grid3dInferenceRequest) -> Result<Self, String> {
        let snapshot = request.resolve_snapshot()?;
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
            assignments: Vec::new(),
            satisfiable: true,
            output: Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput)),
            rejected_output_page: None,
            output_started: false,
            encoded_entries: 0,
            encode_total: 0,
            preview_units: 0,
            last_preview_ms: None,
            publication: None,
            lent: Grid3dInferenceJobLent::Own,
            restore_done: false,
            child_end: None,
            output_payload: None,
        })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    pub(crate) fn validate_id(value: &str) -> Result<(), String> {
        if value.len() > MAX_GRID3D_ID_BYTES {
            Err("grid3d-inference-id-admission-exceeded".into())
        } else {
            Ok(())
        }
    }

    pub(crate) fn progress(&self) -> (usize, usize) {
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

    pub(crate) fn stage_preview(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Grid3dInferenceJobRun {
        let (completed, total) = self.progress();
        let sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence,
            Err(_) => return self.stage_fault(Vec::new()),
        };
        let mut preview = [0; 25];
        preview[..8].copy_from_slice(&sequence.to_le_bytes());
        preview[8..16].copy_from_slice(&(completed as u64).to_le_bytes());
        preview[16..24].copy_from_slice(&(total as u64).to_le_bytes());
        preview[24] = self.stage as u8;
        self.preview_units = 0;
        self.last_preview_ms = context.now_us().map(|now_us| now_us / 1_000);
        self.publication = Some(engine::job::Publication::new(engine::job::PublicationKind::Preview, preview.to_vec(), Vec::new()));
        Grid3dInferenceJobRun::Staged
    }

    /// 🧯️ Stages the fault detail as the next lent outcome.
    pub(crate) fn stage_fault(&mut self, detail: Vec<u8>) -> Grid3dInferenceJobRun {
        self.publication = Some(engine::job::Publication::new(engine::job::PublicationKind::Fault, detail, Vec::new()));
        Grid3dInferenceJobRun::Staged
    }

    pub(crate) fn preview_due(&self, now_ms: u64) -> bool {
        self.last_preview_ms.is_none() || self.preview_units >= PARENT_PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PARENT_PREVIEW_TIME_INTERVAL_MS)
    }

    pub(crate) fn advance_compile(&mut self) -> Result<(), String> {
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

    pub(crate) fn map_one(&mut self) -> Result<(), String> {
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

    pub(crate) fn encode_one(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
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
            let row = crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(&row);
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

/// 🚦️ What one bounded run decided before any outcome is lent.
pub(crate) enum Grid3dInferenceJobRun {
    Yield,
    Cancelled,
    Staged,
    StepRestore,
    StepChild,
}

/// 🤝️ Which retained owner produced the outcome currently on loan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Grid3dInferenceJobLent {
    Own,
    Restore,
    Child,
}

/// 🏁️ How the child solve ended; both ends are consumed here and never lent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InferenceChildEnd {
    Complete,
    Unsatisfiable,
}

impl Grid3dInferenceJob {
    /// ⏭️ Consumes the child's end: its commit is taken, its sealed state payload stays lent from the child until close.
    fn finish_child(&mut self, context: &mut semio_framework_job::StepContext<'_>, end: InferenceChildEnd) -> Grid3dInferenceJobRun {
        match end {
            InferenceChildEnd::Complete => {
                self.child_commit = self.child.as_mut().expect("WFC child").take_completed_commit();
                self.cursor = 0;
                self.stage = Grid3dInferenceStage::MapCommit;
            }
            InferenceChildEnd::Unsatisfiable => {
                self.satisfiable = false;
                self.encode_total = 0;
                self.stage = Grid3dInferenceStage::EncodeCommit;
            }
        }
        self.stage_preview(context)
    }

    /// ⏭️ Runs the bounded compile, map and encode spans; a child or restore turn is requested, never taken here.
    fn run(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Grid3dInferenceJobRun {
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
                        return self.stage_fault(error.into_bytes());
                    }
                }
                Grid3dInferenceStage::Restore => {
                    if self.restore_done {
                        self.child = self.restore.as_mut().expect("restore job").take_job();
                        if let Some(mut restore) = self.restore.take() {
                            engine::job::close_job(&mut restore);
                        }
                        self.restore_done = false;
                        self.stage = Grid3dInferenceStage::Solve;
                        return self.stage_preview(context);
                    }
                    return Grid3dInferenceJobRun::StepRestore;
                }
                Grid3dInferenceStage::Solve => {
                    let Some(end) = self.child_end.take() else {
                        return Grid3dInferenceJobRun::StepChild;
                    };
                    return self.finish_child(context, end);
                }
                Grid3dInferenceStage::MapCommit => {
                    if let Err(error) = self.map_one() {
                        return self.stage_fault(error.into_bytes());
                    }
                }
                Grid3dInferenceStage::EncodeCommit => match self.encode_one(context) {
                    Ok(true) => {
                        let output = match self.output.take().expect("grid3d output writer").finish() {
                            Ok(output) => output,
                            Err(mut writer) => {
                                writer.begin_close();
                                self.output = Some(writer);
                                return Grid3dInferenceJobRun::Yield;
                            }
                        };
                        self.output_payload = Some(output);
                        return Grid3dInferenceJobRun::Yield;
                    }
                    Ok(false) => return Grid3dInferenceJobRun::Yield,
                    Err(error) => return self.stage_fault(error.into_bytes()),
                },
                Grid3dInferenceStage::Complete => unreachable!("complete returns immediately"),
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return Grid3dInferenceJobRun::Cancelled;
            }
            if context.now_us().is_some_and(|now_us| self.preview_due(now_us / 1_000)) {
                return self.stage_preview(context);
            }
            if context.should_yield() {
                return Grid3dInferenceJobRun::Yield;
            }
        }
    }

    /// 🤝️ Closes the lent preview or fault turn by turn from the next step's own wallet.
    fn retire_delivered<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        let publication = self.publication.as_mut().expect("a delivered inference publication is staged");
        let step = publication.close_step(context.retained_grant());
        context.consume_retained(step.progress())?;
        if let semio_framework_job::InteractiveJobCloseStep::Refused { kind, progress } = step {
            return Err(semio_framework_value::ValueError::literal(kind, "inference publication close was refused").with_retained_progress(progress));
        }
        if publication.terminal_is_empty() {
            self.publication = None;
        }
        Ok(None)
    }

    /// 📏️ Quotes the next close turn in the order the ladder spends them.
    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inference close depth overflow"))?;
            Ok(demand)
        };
        if let Some(publication) = self.publication.as_ref() {
            return publication.retirement_demands();
        }
        if let Some(output) = self.output.as_ref() {
            return if output.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }) } else { nested(output.retirement_demands()?) };
        }
        if let Some(payload) = self.output_payload.as_ref() {
            return if payload.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }) } else { nested(payload.retirement_demands()?) };
        }
        if let Some(restore) = self.restore.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: restore.next_close_copy_byte_demand()?, capacity_bytes: restore.next_close_capacity_byte_demand(0)?, release_bytes: restore.next_close_release_byte_demand()?, depth: restore.next_close_depth_demand()?.saturating_add(1) });
        }
        if let Some(child) = self.child.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: child.next_close_copy_byte_demand()?, capacity_bytes: child.next_close_capacity_byte_demand(0)?, release_bytes: child.next_close_release_byte_demand()?, depth: child.next_close_depth_demand()?.saturating_add(1) });
        }
        if let Some(source) = self.rejected_output_page.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: source.allocated_capacity_bytes(), depth: 1, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand::default())
    }
}

impl semio_framework_job::InteractiveJob for Grid3dInferenceJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeBorrow;
        if self.publication.as_ref().is_some_and(|publication| publication.is_delivered()) {
            return self.retire_delivered(context);
        }
        if context.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(context);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            self.stage_fault(b"stale-wfc-grid3d-inference-operation".to_vec());
        }
        if self.publication.is_some() {
            self.lent = Grid3dInferenceJobLent::Own;
            return self.publication.as_mut().expect("a staged inference publication").poll(context);
        }
        if self.output_payload.is_some() {
            return JobOutcomeBorrow::admit_complete(context, self.child.as_ref().and_then(|child| child.commit_state()), self.output_payload.as_ref());
        }
        match self.run(context) {
            Grid3dInferenceJobRun::Yield | Grid3dInferenceJobRun::Staged => Ok(None),
            Grid3dInferenceJobRun::Cancelled => JobOutcomeBorrow::admit_cancelled(context),
            Grid3dInferenceJobRun::StepRestore => {
                let result = self.restore.as_mut().expect("restore job").step(context)?;
                if matches!(result, Some(JobOutcomeBorrow::Complete { .. })) {
                    self.restore_done = true;
                    return Ok(None);
                }
                self.lent = Grid3dInferenceJobLent::Restore;
                Ok(result)
            }
            Grid3dInferenceJobRun::StepChild => {
                let result = self.child.as_mut().expect("WFC child").step(context)?;
                let end = if matches!(result, Some(JobOutcomeBorrow::Complete { .. })) {
                    Some(InferenceChildEnd::Complete)
                } else {
                    if let Some(JobOutcomeBorrow::Fault { detail, .. }) = &result {
            if engine::job::payload_bytes(detail) == b"wfc-unsatisfiable" {
                Some(InferenceChildEnd::Unsatisfiable)
            } else {
                None
            }
        } else {
            None
        }
                };
                if let Some(end) = end {
                    self.child_end = Some(end);
                    return Ok(None);
                }
                self.lent = Grid3dInferenceJobLent::Child;
                Ok(result)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeKind;
        let absent = || semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "inference outcome has no retained owner");
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(self.child.as_ref().and_then(|child| child.commit_state()), self.output_payload.as_ref()),
            _ => match self.lent {
                Grid3dInferenceJobLent::Own => self.publication.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
                Grid3dInferenceJobLent::Restore => self.restore.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
                Grid3dInferenceJobLent::Child => self.child.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
            },
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

    fn close_step(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress};
        let refused = |error: semio_framework_value::ValueError| InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() };
        let demand = match self.close_demands() {
            Ok(demand) => demand,
            Err(error) => return refused(error),
        };
        if self.terminal_is_empty() {
            return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        }
        let one = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let child_grant = semio_framework_job::RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(publication) = self.publication.as_mut() {
            let step = publication.close_step(grant);
            if publication.terminal_is_empty() {
                self.publication = None;
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if let Some(output) = self.output.as_mut() {
            if output.terminal_is_empty() {
                self.output = None;
                return InteractiveJobCloseStep::Pending { progress: one };
            }
            return match output.close_step(child_grant) {
                Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
                Err(error) => refused(error),
            };
        }
        if let Some(payload) = self.output_payload.as_mut() {
            if payload.terminal_is_empty() {
                self.output_payload = None;
                return InteractiveJobCloseStep::Pending { progress: one };
            }
            return match payload.close_step(child_grant) {
                Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
                Err(error) => refused(error),
            };
        }
        if let Some(restore) = self.restore.as_mut() {
            let step = restore.close_step(child_grant);
            if restore.terminal_is_empty() {
                self.restore = None;
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: step.progress().copied_items.max(1), ..step.progress() } };
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if let Some(child) = self.child.as_mut() {
            let step = child.close_step(child_grant);
            if child.terminal_is_empty() {
                self.child = None;
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: step.progress().copied_items.max(1), ..step.progress() } };
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if let Some(source) = self.rejected_output_page.take() {
            let released_bytes = source.allocated_capacity_bytes();
            drop(source);
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { released_bytes, ..one } };
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.depth)
    }

    /// 🚪️ Ownership ONLY — deliberately NOT gated on a closing flag: the retained session calls `begin_close`
    /// itself, and a flag-gated predicate would spin its own close ladder forever.
    fn terminal_is_empty(&self) -> bool {
        self.publication.is_none() && self.output.is_none() && self.output_payload.is_none() && self.restore.is_none() && self.child.is_none() && self.rejected_output_page.is_none()
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
        let mut request: Grid3dInferenceRequest = crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("grid3d-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        Grid3dInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

/// 💡️ Descriptor for the `s.wfc.grid3d` solve inference — five handcrafted facet leaves.
pub fn grid3d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.wfc.grid3d.solve",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"),
            typescript: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts"),
            graphql: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️.graphql"),
            json_schema: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json"),
            proto: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️.proto"),
        },
    }
}

pub fn register_grid3d_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(Grid3dInferenceJobFactory::default())
}

/// 🏁️ Explicit headless adapter over the same complete parent job the public factory hands out.
pub fn solve(snapshot: &Grid3dSnapshot) -> Result<Grid3dInferenceCommit, String> {
    solve_with_clock(snapshot, semio_framework_job::default_now_us)
}

/// 🧮️ The same headless adapter driven by an injected clock, so correctness laws run on
/// [`semio_framework_job::logical_now_us`] and never on a descheduled thread's wall clock.
pub fn solve_with_clock(snapshot: &Grid3dSnapshot, now_us: fn() -> Option<u64>) -> Result<Grid3dInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = Grid3dInferenceJob::new(operation, Grid3dInferenceRequest { snapshot: Some(snapshot.clone()), document: None, checkpoint: None })?;
    engine::job::run_headless(
        job,
        operation,
        now_us,
        engine::job::HeadlessSite { site: "wfc.grid3d.inference.headless", label: "grid3d-inference-headless", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: HEADLESS_FUEL_PER_STEP, step_budget_us: HEADLESS_STEP_BUDGET_US },
        |_job, output| {
            std::str::from_utf8(output)
                .map_err(|error| format!("grid3d-invalid-commit:{error}"))
                .and_then(|text| crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value::<Grid3dInferenceCommit>(text).map_err(|error| format!("grid3d-invalid-commit:{error}")))
        },
    )
}

impl store::InferredField<Grid3dSnapshot> for Grid3dSolve {
    type Dependency = Vec<u8>;
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
        crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(snapshot).into_bytes()
    }
    fn compute(snapshot: &Grid3dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve(snapshot) {
            Ok(solution) if solution.satisfiable => Grid3dSolveResult::Solved { assignments: solution.assignments },
            _ => Grid3dSolveResult::Unsolved,
        }
    }
}

impl store::InferredField<Grid3dSnapshot> for Grid3dContradiction {
    type Dependency = Vec<u8>;
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
        crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(snapshot).into_bytes()
    }
    fn compute(snapshot: &Grid3dSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        solve(snapshot).is_ok_and(|commit| commit.satisfiable)
    }
}

impl store::InferredField<Grid3dSnapshot> for Grid3dEntropy {
    type Dependency = Vec<u8>;
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
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
