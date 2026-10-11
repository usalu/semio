//! 🏠️ Inference admission, publication and retained job lifecycle.
use semio_framework_job::InteractiveJob;
use crate::schema::snapshot::{BitmapSnapshot};
use crate::standards::v1::subsets::any::io::text::snapshot::{encode_base64};
use semio_s_plugin_wfc_engine as engine;
use crate::standards::v1::subsets::any::schema::inferences::*;

pub const BITMAP_INFERENCE_JOB_KIND: &str = "semio.infer";

pub const BITMAP_INFERENCE_TOOL_ID: &str = "s.wfc.bitmap.solve";

pub const BITMAP_INFERENCE_PAYLOAD_SCHEMA: &str = "s.wfc.bitmap.inference.request.v1";

/// 📜️ The PUBLISHED request schema of `s.wfc.bitmap.solve` — what a client has to send, readable
/// from `inference_list`/`capabilities_describe` without reading a line of this crate. Authored
/// here rather than as a facet leaf because the facet leaf beside it (`🔣️.json`) is the RESULT
/// schema; a request and its result are two schemas, and publishing only one was the gap
/// (`📓️ce3-four-mcp-gates-green.md` §3.3).
pub const BITMAP_INFERENCE_REQUEST_SCHEMA: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://json.schemas.assets.semio-tech.com/s/wfc/bitmap/1/any/inference.request.json",
  "title": "BitmapInferenceRequest",
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
    "snapshot": { "type": "object", "description": "The problem stated in full instead of read from an artifact: the bitmap solve's own document shape." },
    "checkpoint": { "type": "array", "description": "A previous run's checkpoint bytes, to resume instead of restart.", "items": { "type": "integer", "minimum": 0, "maximum": 255 } }
  }
}"#;

/// 📌️ The editor action that commits a finished solve into the document by pinning its pixels —
/// the one action id the contract below declares and the editor's `pin-solution` command answers to.
pub const BITMAP_INFERENCE_COMMIT_ACTION: &str = "pin-solution";

/// 📜️ The whole published contract for `s.wfc.bitmap.solve`: request schema, result schema, the
/// unit its bounded job counts, and the artifact binding that makes it callable at all.
pub const BITMAP_INFERENCE_CONTRACT: semio_framework_plugin::ArtifactInferencePayloadContract = semio_framework_plugin::ArtifactInferencePayloadContract {
    payload_schema_id: BITMAP_INFERENCE_PAYLOAD_SCHEMA,
    input_schema: BITMAP_INFERENCE_REQUEST_SCHEMA,
    output_schema: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json"),
    progress_unit: "cells",
    artifact_binding: Some(semio_framework_plugin::ArtifactInferenceDocumentBinding { field: "document", encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64, required: true }),
    commit: Some(semio_framework_plugin::ArtifactInferenceCommitBinding { action: BITMAP_INFERENCE_COMMIT_ACTION }),
};

/// 🧭️ Stable host roster identity for the ActionBus-owned cold solve route.
pub const fn bitmap_inference_metadata() -> semio_framework_plugin::ArtifactInferenceServiceMetadata {
    semio_framework_plugin::ArtifactInferenceServiceMetadata {
        owner: "wfc",
        artifact_kind: "s.wfc.bitmap",
        artifact_schema: "s.wfc.bitmap",
        artifact_schema_version: 1,
        inference_schema: BITMAP_INFERENCE_TOOL_ID,
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        payload: Some(BITMAP_INFERENCE_CONTRACT),
    }
}

/// 💡️ Descriptor for the `s.wfc.bitmap` solve inference — five handcrafted facet leaves.
pub fn bitmap_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: BITMAP_INFERENCE_TOOL_ID,
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"),
            typescript: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts"),
            graphql: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️.graphql"),
            json_schema: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json"),
            proto: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️.proto"),
        },
    }
}

#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
pub struct BitmapInferenceRequest {
    /// 📸️ The problem, stated in full by the caller. Mutually exclusive with `document`: exactly one
    /// of the two says which snapshot this solve runs over.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<BitmapSnapshot>,
    /// 🔗️ The ARTIFACT this solve runs on, as its own canonical `pack`/`spr` pair. This is the
    /// field `BITMAP_INFERENCE_CONTRACT`'s artifact binding names, so an agent that calls
    /// `inference_run` with `artifactId` never has to state a snapshot it could not type: the
    /// gateway binds the document here and the guest decodes it into its own snapshot below.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<semio_framework_plugin::ArtifactDocumentPayload>,
    pub checkpoint: Option<Vec<u8>>,
}

impl BitmapInferenceRequest {
    /// 📸️ The snapshot this request states, from whichever of its two carriers is present — the
    /// ONE resolution path every caller of this inference takes.
    pub fn resolve_snapshot(&self) -> Result<BitmapSnapshot, String> {
        match (&self.snapshot, &self.document) {
            (Some(snapshot), _) => Ok(snapshot.clone()),
            (None, Some(document)) => document.settled_snapshot::<BitmapSnapshot, crate::BitmapMutation>(),
            (None, None) => Err("s.wfc.bitmap-inference-no-snapshot:state `snapshot`, or name the artifact with `artifactId` so `document` is bound".into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum BitmapInferenceStage {
    Sample,
    Extract,
    Topology,
    Fixed,
    Restore,
    Solve,
    Decode,
    EncodeCommit,
    Complete,
}

/// 📝️ Which part of the commit JSON the encoder is currently writing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EncodePhase {
    Open,
    Pixels,
    Verdict,
    Entropy,
    Close,
}

/// 🧵 Worker-owned parent transaction for bitmap extraction, collapse and commit encoding.
pub struct BitmapInferenceJob {
    pub(crate) operation: semio_framework_job::Operation,
    pub(crate) snapshot: BitmapSnapshot,
    pub(crate) stage: BitmapInferenceStage,
    pub(crate) cursor: usize,
    pub(crate) sample: Option<engine::extract::Sample2d>,
    pub(crate) extracted: Option<engine::extract::ExtractedModel2d>,
    pub(crate) decoder: Option<engine::extract::PatternDecoder2d>,
    pub(crate) topology: Option<engine::grid2d::Grid2dTopology>,
    pub(crate) fixed: Vec<(engine::ids::NodeId, engine::ids::PatternId)>,
    pub(crate) entropy: Vec<f64>,
    pub(crate) pixels: String,
    pub(crate) contradiction: bool,
    pub(crate) checkpoint: Option<Vec<u8>>,
    pub(crate) restore: Option<engine::job::WfcRestore<engine::grid2d::Grid2dTopology>>,
    pub(crate) child: Option<engine::job::WfcJob<engine::grid2d::Grid2dTopology>>,
    pub(crate) child_commit: Option<engine::job::WfcCommit>,
    pub(crate) output: Option<semio_framework_job::RetainedJobPayloadWriter>,
    pub(crate) rejected_output_page: Option<semio_framework_job::JobPayloadPageSource>,
    pub(crate) encode_phase: EncodePhase,
    pub(crate) encode_cursor: usize,
    pub(crate) preview_units: u64,
    pub(crate) last_preview_ms: Option<u64>,
    pub(crate) publication: Option<Box<engine::job::Publication>>,
    pub(crate) lent: BitmapInferenceJobLent,
    pub(crate) restore_done: bool,
    pub(crate) child_end: Option<InferenceChildEnd>,
    pub(crate) output_payload: Option<semio_framework_job::RetainedJobPayload>,
    pub(crate) closing: bool,
}

impl BitmapInferenceJob {
    pub(crate) fn new(mut operation: semio_framework_job::Operation, request: BitmapInferenceRequest) -> Result<Self, String> {
        let snapshot = request.resolve_snapshot()?;
        let input_cells = (snapshot.input.width as usize).saturating_mul(snapshot.input.height as usize);
        let output_cells = (snapshot.output.width as usize).saturating_mul(snapshot.output.height as usize);
        let pattern = snapshot.model.pattern_size.max(1) as usize;
        let input_min = (snapshot.input.width as usize).min(snapshot.input.height as usize);
        if input_cells == 0
            || output_cells == 0
            || input_cells > MAX_BITMAP_INPUT_CELLS
            || output_cells > MAX_BITMAP_OUTPUT_CELLS
            || snapshot.pinned.len() > MAX_BITMAP_PINS
            || snapshot.input.palette.is_empty()
            || (!snapshot.model.periodic_input && pattern > input_min)
            || request.checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.len() > engine::job::MAX_CHECKPOINT_BYTES)
        {
            return Err("bitmap-inference-admission-exceeded".into());
        }
        operation.seed = snapshot.seed;
        Ok(Self {
            operation,
            snapshot,
            stage: BitmapInferenceStage::Sample,
            cursor: 0,
            sample: None,
            extracted: None,
            decoder: None,
            topology: None,
            fixed: Vec::new(),
            entropy: Vec::new(),
            pixels: String::new(),
            contradiction: false,
            checkpoint: request.checkpoint,
            restore: None,
            child: None,
            child_commit: None,
            output: Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput)),
            rejected_output_page: None,
            encode_phase: EncodePhase::Open,
            encode_cursor: 0,
            preview_units: 0,
            last_preview_ms: None,
            publication: None,
            lent: BitmapInferenceJobLent::Own,
            restore_done: false,
            child_end: None,
            output_payload: None,
            closing: false,
        })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    pub(crate) fn progress(&self) -> (usize, usize) {
        match self.stage {
            BitmapInferenceStage::Sample => (0, 1),
            BitmapInferenceStage::Extract => (0, 1),
            BitmapInferenceStage::Topology => (0, 1),
            BitmapInferenceStage::Fixed => (self.cursor, self.snapshot.pinned.len()),
            BitmapInferenceStage::Restore | BitmapInferenceStage::Solve => (0, 1),
            BitmapInferenceStage::Decode => (0, 1),
            BitmapInferenceStage::EncodeCommit => (self.encode_cursor, self.pixels.len()),
            BitmapInferenceStage::Complete => (1, 1),
        }
    }

    pub(crate) fn stage_preview(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BitmapInferenceJobRun {
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
        BitmapInferenceJobRun::Staged
    }

    /// 🧯️ Stages the fault detail as the next lent outcome.
    pub(crate) fn stage_fault(&mut self, detail: Vec<u8>) -> BitmapInferenceJobRun {
        self.publication = Some(engine::job::Publication::new(engine::job::PublicationKind::Fault, detail, Vec::new()));
        BitmapInferenceJobRun::Staged
    }

    pub(crate) fn preview_due(&self, now_ms: u64) -> bool {
        self.last_preview_ms.is_none() || self.preview_units >= PARENT_PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PARENT_PREVIEW_TIME_INTERVAL_MS)
    }

    /// 🎨️ The first pattern whose anchor tile is that palette index — the anchor convention is what
    /// makes a per-cell colour pin expressible as a pattern pin at all.
    pub(crate) fn pattern_for_color(&self, color: u32) -> Option<engine::ids::PatternId> {
        let decoder = self.decoder.as_ref()?;
        let model = &self.extracted.as_ref()?.model;
        (0..model.pattern_count()).map(engine::ids::PatternId::from_index).find(|pattern| decoder.anchor_tile(*pattern).get() == color)
    }

    /// 🩺 A colour that is never a pattern's ANCHOR cannot be asked for: the decoder
    /// reads an assignment back through the anchor convention, so a pin on such a
    /// colour is unsatisfiable by construction and is reported rather than dropped.
    pub(crate) fn advance_compile(&mut self) -> Result<(), String> {
        match self.stage {
            BitmapInferenceStage::Sample => {
                let indices = self.snapshot.input.indices().ok_or("bitmap-inference-malformed-input")?;
                if indices.iter().any(|index| usize::from(*index) >= self.snapshot.input.palette.len()) {
                    return Err("bitmap-inference-unknown-palette-index".into());
                }
                let tiles = indices.iter().map(|index| engine::ids::TileId(u32::from(*index))).collect();
                self.sample = Some(engine::extract::Sample2d::new(self.snapshot.input.width as usize, self.snapshot.input.height as usize, tiles));
                self.stage = BitmapInferenceStage::Extract;
            }
            BitmapInferenceStage::Extract => {
                let sample = self.sample.take().ok_or("bitmap-inference-sample-missing")?;
                let config = engine::extract::Extract2dConfig { window: self.snapshot.model.pattern_size.max(1) as usize, periodic_input: self.snapshot.model.periodic_input, symmetry: symmetry_group(self.snapshot.model.symmetry) };
                let extracted = engine::extract::extract_2d(&[sample], &config).map_err(|error| format!("{error:?}"))?;
                if extracted.model.pattern_count() > MAX_BITMAP_PATTERNS {
                    return Err("bitmap-inference-pattern-universe-exceeded".into());
                }
                self.decoder = Some(extracted.decoder.clone());
                self.extracted = Some(extracted);
                self.stage = BitmapInferenceStage::Topology;
            }
            BitmapInferenceStage::Topology => {
                let boundary = if self.snapshot.output.periodic { engine::grid2d::Boundary::Wrap } else { engine::grid2d::Boundary::Open };
                let topology = engine::grid2d::Grid2dTopology::new(self.snapshot.output.width as usize, self.snapshot.output.height as usize, &engine::grid2d::Stencil2d::VonNeumann, stencil_relations(), boundary, boundary, None).map_err(|error| format!("{error:?}"))?;
                self.topology = Some(topology);
                self.cursor = 0;
                self.stage = BitmapInferenceStage::Fixed;
            }
            BitmapInferenceStage::Fixed => {
                if let Some(pin) = self.snapshot.pinned.get(self.cursor).copied() {
                    if pin.x >= self.snapshot.output.width || pin.y >= self.snapshot.output.height {
                        return Err("bitmap-inference-pin-outside-output".into());
                    }
                    let pattern = self.pattern_for_color(pin.color).ok_or("bitmap-inference-unreachable-pin")?;
                    let node = engine::ids::NodeId::from_index((pin.y as usize) * (self.snapshot.output.width as usize) + pin.x as usize);
                    self.fixed.push((node, pattern));
                    self.cursor += 1;
                } else {
                    self.push_ground_pins();
                    let model = self.extracted.take().ok_or("bitmap-inference-model-missing")?.model;
                    let topology = self.topology.take().ok_or("bitmap-inference-topology-missing")?;
                    self.entropy = self.prior_entropy(&model);
                    let fixed = std::mem::take(&mut self.fixed);
                    if let Some(checkpoint) = self.checkpoint.take() {
                        self.restore = Some(engine::job::WfcRestore::new(self.operation, model, topology, engine::job::WfcJobConfig::default(), None, fixed, checkpoint)?);
                        self.stage = BitmapInferenceStage::Restore;
                    } else {
                        self.child = Some(engine::job::WfcJob::new(self.operation, model, topology, engine::job::WfcJobConfig::default(), None, fixed));
                        self.stage = BitmapInferenceStage::Solve;
                    }
                    self.cursor = 0;
                }
            }
            _ => unreachable!("non-compile bitmap inference stage"),
        }
        Ok(())
    }

    /// 🌱 The ground colour, when set, fixes every cell of the OUTPUT's bottom row — the classic
    /// overlapping-model "ground" constraint, expressed as ordinary pins so it shares one code path
    /// with the authored ones.
    pub(crate) fn push_ground_pins(&mut self) {
        let Some(ground) = self.snapshot.model.ground else { return };
        let Some(pattern) = self.pattern_for_color(ground) else { return };
        let width = self.snapshot.output.width as usize;
        let row = (self.snapshot.output.height as usize).saturating_sub(1);
        for x in 0..width {
            let node = engine::ids::NodeId::from_index(row * width + x);
            if !self.fixed.iter().any(|(fixed, _)| *fixed == node) {
                self.fixed.push((node, pattern));
            }
        }
    }

    /// 🎲 Per-cell PRIOR Shannon entropy over the extracted pattern weights: `0.0` for a fixed cell
    /// (fully determined), the whole-universe entropy for every other. Honestly scoped: this is the
    /// entropy BEFORE arc consistency narrows any cell's domain, the same statement the sibling
    /// assembly artifact's own entropy field makes.
    pub(crate) fn prior_entropy(&self, model: &engine::model::CompiledModel) -> Vec<f64> {
        let weights = model.weights();
        let total: f64 = (0..model.pattern_count()).map(|index| weights.w(engine::ids::PatternId::from_index(index))).sum();
        let universe = if total <= 0.0 {
            0.0
        } else {
            -(0..model.pattern_count())
                .map(|index| weights.w(engine::ids::PatternId::from_index(index)) / total)
                .filter(|share| *share > 0.0)
                .map(|share| share * share.ln())
                .sum::<f64>()
        };
        let cells = (self.snapshot.output.width as usize) * (self.snapshot.output.height as usize);
        (0..cells).map(|cell| if self.fixed.iter().any(|(node, _)| node.index() == cell) { 0.0 } else { universe }).collect()
    }

    /// 🖼️ Turns the collapsed pattern assignment back into palette indices through the extraction
    /// decoder's anchor convention, then into the base64 buffer the commit carries.
    pub(crate) fn decode_commit(&mut self) -> Result<(), String> {
        let commit = self.child_commit.take().ok_or("bitmap-inference-commit-missing")?;
        let decoder = self.decoder.as_ref().ok_or("bitmap-inference-decoder-missing")?;
        let cells = (self.snapshot.output.width as usize) * (self.snapshot.output.height as usize);
        if commit.assignment.len() != cells {
            return Err("bitmap-inference-commit-shape".into());
        }
        let palette_len = self.snapshot.input.palette.len() as u32;
        let mut indices = Vec::with_capacity(cells);
        for pattern in &commit.assignment {
            let tile = decoder.anchor_tile(engine::ids::PatternId(*pattern));
            if tile.get() >= palette_len {
                return Err("bitmap-inference-decoded-palette-index".into());
            }
            indices.push(tile.get() as u8);
        }
        self.pixels = encode_base64(&indices);
        self.stage = BitmapInferenceStage::EncodeCommit;
        Ok(())
    }



    /// 📝️ Fills ONE payload page to capacity before asking for another. One page per small write
    /// would burn the operation's 256-page budget on a handful of entropy values — the exact
    /// admission refusal this encoder is shaped to avoid.
    pub(crate) fn encode_one(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
        let source = self.rejected_output_page.take().unwrap_or_default();
        let mut writer = self.output.take().ok_or("bitmap-output-writer-missing")?;
        let outcome = self.encode_into(context, &mut writer, source);
        self.output = Some(writer);
        outcome
    }

    pub(crate) fn encode_into(&mut self, context: &mut semio_framework_job::StepContext<'_>, writer: &mut semio_framework_job::RetainedJobPayloadWriter, source: semio_framework_job::JobPayloadPageSource) -> Result<bool, String> {
        let mut page = match context.admit_payload_page(writer, source) {
            Ok(page) => page,
            Err(rejected) => {
                self.rejected_output_page = Some(rejected.into_source());
                return Err("bitmap-inference-output-admission-exceeded".into());
            }
        };
        let mut wrote_any = false;
        loop {
            let Some((chunk, phase, cursor)) = self.next_encode_chunk() else {
                page.write(b"]}").map_err(|_| "bitmap-inference-output-page")?;
                page.commit();
                self.stage = BitmapInferenceStage::Complete;
                return Ok(true);
            };
            if !chunk.is_empty() {
                if page.write(chunk.as_bytes()).is_err() {
                    if !wrote_any {
                        return Err("bitmap-inference-output-chunk-exceeds-a-page".into());
                    }
                    page.commit();
                    return Ok(false);
                }
                wrote_any = true;
            }
            self.encode_phase = phase;
            self.encode_cursor = cursor;
        }
    }
}

/// 🚦️ What one bounded run decided before any outcome is lent.
pub(crate) enum BitmapInferenceJobRun {
    Yield,
    Cancelled,
    Staged,
    StepRestore,
    StepChild,
}

/// 🤝️ Which retained owner produced the outcome currently on loan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BitmapInferenceJobLent {
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

impl BitmapInferenceJob {
    /// ⏭️ Consumes the child's end: its commit is taken, its sealed state payload stays lent from the child until close.
    fn finish_child(&mut self, context: &mut semio_framework_job::StepContext<'_>, end: InferenceChildEnd) -> BitmapInferenceJobRun {
        match end {
            InferenceChildEnd::Complete => {
                self.child_commit = self.child.as_mut().expect("WFC child").take_completed_commit();
                self.contradiction = self.child_commit.is_none();
            }
            InferenceChildEnd::Unsatisfiable => {
                self.contradiction = true;
            }
        }
        self.cursor = 0;
        self.stage = BitmapInferenceStage::Decode;
        self.stage_preview(context)
    }

    /// ⏭️ Runs the bounded compile, map and encode spans; a child or restore turn is requested, never taken here.
    fn run(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BitmapInferenceJobRun {
        loop {
            context.set_stage(match self.stage {
                BitmapInferenceStage::Sample => "bitmap.infer.sample",
                BitmapInferenceStage::Extract => "bitmap.infer.extract",
                BitmapInferenceStage::Topology => "bitmap.infer.topology",
                BitmapInferenceStage::Fixed => "bitmap.infer.fixed",
                BitmapInferenceStage::Restore => "bitmap.infer.restore",
                BitmapInferenceStage::Solve => "bitmap.infer.solve",
                BitmapInferenceStage::Decode => "bitmap.infer.decode",
                BitmapInferenceStage::EncodeCommit => "bitmap.infer.encode-commit",
                BitmapInferenceStage::Complete => "bitmap.infer.complete",
            });
            match self.stage {
                BitmapInferenceStage::Sample | BitmapInferenceStage::Extract | BitmapInferenceStage::Topology | BitmapInferenceStage::Fixed => {
                    if let Err(error) = self.advance_compile() {
                        return self.stage_fault(error.into_bytes());
                    }
                }
                BitmapInferenceStage::Restore => {
                    if self.restore_done {
                        self.child = self.restore.as_mut().expect("restore job").take_job();
                        if let Some(mut restore) = self.restore.take() {
                            engine::job::close_job(&mut restore);
                        }
                        self.restore_done = false;
                        self.stage = BitmapInferenceStage::Solve;
                        return self.stage_preview(context);
                    }
                    return BitmapInferenceJobRun::StepRestore;
                }
                BitmapInferenceStage::Solve => {
                    let Some(end) = self.child_end.take() else {
                        return BitmapInferenceJobRun::StepChild;
                    };
                    return self.finish_child(context, end);
                }
                BitmapInferenceStage::Decode => {
                    if self.contradiction {
                        self.pixels = String::new();
                        self.stage = BitmapInferenceStage::EncodeCommit;
                    } else if let Err(error) = self.decode_commit() {
                        return self.stage_fault(error.into_bytes());
                    }
                }
                BitmapInferenceStage::EncodeCommit => match self.encode_one(context) {
                    Ok(true) => {
                        let output = match self.output.take().expect("bitmap output writer").finish() {
                            Ok(output) => output,
                            Err(mut writer) => {
                                writer.begin_close();
                                self.output = Some(writer);
                                return BitmapInferenceJobRun::Yield;
                            }
                        };
                        self.output_payload = Some(output);
                        return BitmapInferenceJobRun::Yield;
                    }
                    Ok(false) => {},
                    Err(error) => return self.stage_fault(error.into_bytes()),
                },
                BitmapInferenceStage::Complete => unreachable!("complete returns immediately"),
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return BitmapInferenceJobRun::Cancelled;
            }
            if context.now_us().is_some_and(|now_us| self.preview_due(now_us / 1_000)) {
                return self.stage_preview(context);
            }
            if context.should_yield() {
                return BitmapInferenceJobRun::Yield;
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

impl semio_framework_job::InteractiveJob for BitmapInferenceJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeBorrow;
        if self.publication.as_ref().is_some_and(|publication| publication.is_delivered()) {
            return self.retire_delivered(context);
        }
        if context.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(context);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            self.stage_fault(b"stale-bitmap-inference-operation".to_vec());
        }
        if self.publication.is_some() {
            self.lent = BitmapInferenceJobLent::Own;
            return self.publication.as_mut().expect("a staged inference publication").poll(context);
        }
        if self.output_payload.is_some() {
            return JobOutcomeBorrow::admit_complete(context, self.child.as_ref().and_then(|child| child.commit_state()), self.output_payload.as_ref());
        }
        match self.run(context) {
            BitmapInferenceJobRun::Yield | BitmapInferenceJobRun::Staged => Ok(None),
            BitmapInferenceJobRun::Cancelled => JobOutcomeBorrow::admit_cancelled(context),
            BitmapInferenceJobRun::StepRestore => {
                let result = self.restore.as_mut().expect("restore job").step(context)?;
                if matches!(result, Some(JobOutcomeBorrow::Complete { .. })) {
                    self.restore_done = true;
                    return Ok(None);
                }
                self.lent = BitmapInferenceJobLent::Restore;
                Ok(result)
            }
            BitmapInferenceJobRun::StepChild => {
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
                self.lent = BitmapInferenceJobLent::Child;
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
                BitmapInferenceJobLent::Own => self.publication.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
                BitmapInferenceJobLent::Restore => self.restore.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
                BitmapInferenceJobLent::Child => self.child.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
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

pub struct BitmapInferenceJobFactory {
    keys: [semio_framework::ToolFactoryKey; 1],
}

impl Default for BitmapInferenceJobFactory {
    fn default() -> Self {
        Self { keys: [semio_framework::ToolFactoryKey::new(BITMAP_INFERENCE_JOB_KIND, BITMAP_INFERENCE_TOOL_ID)] }
    }
}

impl semio_framework::ToolJobFactory for BitmapInferenceJobFactory {
    type Payload = BitmapInferenceRequest;
    type Job = BitmapInferenceJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        BITMAP_INFERENCE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(16 << 20, MAX_BITMAP_INPUT_CELLS + MAX_BITMAP_OUTPUT_CELLS + MAX_BITMAP_PINS, 4_096, MAX_BITMAP_OUTPUT_BYTES, 7_500, 1, 1)
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        BitmapInferenceJob::new(operation, payload).map_err(semio_framework::ToolJobFactoryError::new)
    }

    fn create_job_from_wire(&mut self, operation: semio_framework_job::Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        let payload_text = std::str::from_utf8(payload).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("bitmap-inference-wire-decode:{error}")))?;
        let mut request: BitmapInferenceRequest = crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("bitmap-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        BitmapInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

pub fn register_bitmap_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(BitmapInferenceJobFactory::default())
}

/// 🏁 Explicit headless adapter over the same complete parent job the public factory hands out.
pub fn solve_with_job(snapshot: &BitmapSnapshot) -> Result<BitmapInferenceCommit, String> {
    solve_with_clock(snapshot, semio_framework_job::default_now_us)
}

/// 🧮️ The same headless adapter driven by an injected clock, so correctness laws run on
/// [`semio_framework_job::logical_now_us`] and never on a descheduled thread's wall clock.
pub fn solve_with_clock(snapshot: &BitmapSnapshot, now_us: fn() -> Option<u64>) -> Result<BitmapInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = BitmapInferenceJob::new(operation, BitmapInferenceRequest { snapshot: Some(snapshot.clone()), document: None, checkpoint: None })?;
    engine::job::run_headless(
        job,
        operation,
        now_us,
        engine::job::HeadlessSite { site: "wfc.bitmap.inference.headless", label: "bitmap-inference-headless", stage: semio_framework_job::InteractiveStage::BackgroundStep, fuel_per_step: HEADLESS_FUEL_PER_STEP, step_budget_us: HEADLESS_STEP_BUDGET_US },
        |_job, output| {
            std::str::from_utf8(output)
                .map_err(|error| format!("bitmap-invalid-commit:{error}"))
                .and_then(|text| crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value::<BitmapInferenceCommit>(text).map_err(|error| format!("bitmap-invalid-commit:{error}")))
        },
    )
}

impl store::InferredField<BitmapSnapshot> for BitmapSolve {
    type Dependency = Vec<u8>;
    type Key = String;
    type Value = BitmapSolveResult;

    const FIELD_ID: &'static str = "s.wfc.bitmap.inference.solve";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "input", "output", "model", "pinned"]
    }
    fn plan(_snapshot: &BitmapSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "bitmap".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &BitmapSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(snapshot).into_bytes()
    }
    fn compute(snapshot: &BitmapSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(commit) if !commit.contradiction && !commit.pixels.is_empty() => BitmapSolveResult::Solved { pixels: commit.pixels },
            _ => BitmapSolveResult::Unsolved,
        }
    }
}

impl store::InferredField<BitmapSnapshot> for BitmapContradiction {
    type Dependency = Vec<u8>;
    type Key = String;
    type Value = bool;

    const FIELD_ID: &'static str = "s.wfc.bitmap.inference.contradiction";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["seed", "input", "output", "model", "pinned"]
    }
    fn plan(_snapshot: &BitmapSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        vec![store::InferenceStep { key: "bitmap".to_string(), parents: Vec::new() }]
    }
    fn dep_input(snapshot: &BitmapSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        crate::standards::v1::subsets::any::io::text::inferences::encode_inference_value(snapshot).into_bytes()
    }
    fn compute(snapshot: &BitmapSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(commit) => commit.contradiction,
            Err(_) => true,
        }
    }
}

impl store::InferredField<BitmapSnapshot> for BitmapEntropy {
    type Dependency = Vec<u8>;
    type Key = String;
    type Value = f64;

    const FIELD_ID: &'static str = "s.wfc.bitmap.inference.entropy";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["input", "output", "model", "pinned"]
    }
    fn plan(snapshot: &BitmapSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        let cells = (snapshot.output.width as usize) * (snapshot.output.height as usize);
        (0..cells).map(|cell| store::InferenceStep { key: cell.to_string(), parents: Vec::new() }).collect()
    }
    fn dep_input(snapshot: &BitmapSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let mut bytes = key.clone().into_bytes();
        bytes.push(0);
        bytes.extend_from_slice(snapshot.input.pixels.as_slice());
        bytes.push(0);
        bytes.extend_from_slice(&snapshot.model.pattern_size.to_le_bytes());
        bytes.extend_from_slice(&snapshot.model.symmetry.to_le_bytes());
        for pin in &snapshot.pinned {
            bytes.extend_from_slice(&pin.x.to_le_bytes());
            bytes.extend_from_slice(&pin.y.to_le_bytes());
        }
        bytes
    }
    fn compute(snapshot: &BitmapSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let Ok(cell) = key.parse::<usize>() else { return 0.0 };
        let width = snapshot.output.width as usize;
        if width == 0 {
            return 0.0;
        }
        let (x, y) = ((cell % width) as u32, (cell / width) as u32);
        if snapshot.pinned.iter().any(|pin| pin.x == x && pin.y == y) {
            return 0.0;
        }
        if snapshot.model.ground.is_some() && y + 1 == snapshot.output.height {
            return 0.0;
        }
        pattern_universe_entropy(snapshot)
    }
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
