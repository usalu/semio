//! 🏠️ Inference admission, publication and retained job lifecycle.
use semio_framework_job::InteractiveJob;
use crate::schema::snapshot::{BitmapSnapshot};
use crate::standards::v1::subsets::any::io::text::snapshot::{encode_base64};
use semio_s_plugin_wfc_engine as engine;
use crate::standards::v1::subsets::any::schema::inferences::*;
use semio_framework_value::retirement::RetireOwned;

#[path = "📥️route/🦀️.rs"]
mod route;

/// 🛣️ Installs the original bitmap inference operation with its published request contract.
pub fn bitmap_inference_service() -> semio_plugin::ArtifactInferenceService {
    semio_plugin::ArtifactInferenceService::new_retained::<route::BitmapRoute>(bitmap_inference_metadata())
}

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
pub const BITMAP_INFERENCE_CONTRACT: semio_plugin::ArtifactInferencePayloadContract = semio_plugin::ArtifactInferencePayloadContract {
    payload_schema_id: BITMAP_INFERENCE_PAYLOAD_SCHEMA,
    input_schema: BITMAP_INFERENCE_REQUEST_SCHEMA,
    output_schema: include_str!("../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json"),
    progress_unit: "cells",
    artifact_binding: Some(semio_plugin::ArtifactInferenceDocumentBinding { field: "document", encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64, required: true }),
    commit: Some(semio_plugin::ArtifactInferenceCommitBinding { action: BITMAP_INFERENCE_COMMIT_ACTION }),
};

/// 🧭️ Stable host roster identity shared by the ActionBus factory and the retained cold route.
pub const fn bitmap_inference_metadata() -> semio_plugin::ArtifactInferenceServiceMetadata {
    semio_plugin::ArtifactInferenceServiceMetadata {
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
    pub document: Option<semio_plugin::ArtifactDocumentPayload>,
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

/// 🪙️ Runs one payload close turn inside one actual original effect and charges its performed work to the job turn.
fn close_effect<E: Into<semio_framework_job::OriginalTurnPrimary>>(
    cx: &mut semio_framework_job::StepContext<'_>,
    grant: semio_framework_job::RetainedCloneGrant,
    run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::RetainedWorkOutcome<semio_framework_value::retained_clone::RetainedCloneStep, E>,
) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_job::OriginalTurnFailure> {
    match cx.with_original_effect(grant, |_, effect| run(effect)) {
        Ok(Some((turn, performed))) => {
            cx.consume_work(performed);
            semio_framework_job::receive_original_retained_work(cx, semio_framework_value::RetainedWorkOutcome { result: turn.result, work: 0 })
        }
        Ok(None) => Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())),
        Err(error) => Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
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
    snapshot: Option<BitmapSnapshot>,
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
    pub(crate) output: Option<semio_framework_job_payload::RetainedPayloadBuilder>,
    pub(crate) output_chunk: Option<Vec<u8>>,
    pub(crate) output_cursor: usize,
    pub(crate) output_next: Option<(EncodePhase, usize)>,
    pub(crate) encode_phase: EncodePhase,
    pub(crate) encode_cursor: usize,
    pub(crate) preview_units: u64,
    publish_intermediate: bool,
    pub(crate) last_preview_ms: Option<u64>,
    pub(crate) publication: Option<Box<engine::job::Publication>>,
    pub(crate) lent: BitmapInferenceJobLent,
    pub(crate) restore_done: bool,
    pub(crate) child_end: Option<InferenceChildEnd>,
    pub(crate) output_payload: Option<semio_framework_job_payload::RetainedJobPayload>,
    pub(crate) closing: bool,
    scratch_seed: Option<BitmapInferenceScratch>,
    scratch_retirement: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
}

#[derive(semio_framework_value::RetireOwned)]
enum BitmapInferenceScratch {
    Snapshot(BitmapSnapshot),
    Sample(engine::extract::Sample2d),
    Extracted(engine::extract::ExtractedModel2d),
    Decoder(engine::extract::PatternDecoder2d),
    Topology(engine::grid2d::Grid2dTopology),
    Fixed(Vec<(engine::ids::NodeId, engine::ids::PatternId)>),
    Entropy(Vec<f64>),
    Pixels(String),
    Checkpoint(Vec<u8>),
    Commit(engine::job::WfcCommit),
    Child(engine::job::WfcJob<engine::grid2d::Grid2dTopology>),
    Restore(engine::job::WfcRestore<engine::grid2d::Grid2dTopology>),
}

impl BitmapInferenceJob {
    pub fn new(operation: semio_framework_job::Operation, request: BitmapInferenceRequest) -> Result<Self, String> {
        let snapshot = request.resolve_snapshot()?;
        Self::validate_snapshot(&snapshot, request.checkpoint.as_deref())?;
        Ok(Self::from_admitted_snapshot(operation, snapshot, request.checkpoint))
    }

    /// 📋️ Validates the exact snapshot and optional resume bytes before taking their ownership.
    pub fn validate_snapshot(snapshot: &BitmapSnapshot, checkpoint: Option<&[u8]>) -> Result<(), &'static str> {
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
            || checkpoint.is_some_and(|checkpoint| checkpoint.len() > engine::job::MAX_CHECKPOINT_BYTES)
        {
            return Err("bitmap-inference-admission-exceeded");
        }
        Ok(())
    }

    /// 🧺️ Takes an admitted problem and checkpoint without copying either source.
    pub fn from_admitted_snapshot(mut operation: semio_framework_job::Operation, snapshot: BitmapSnapshot, checkpoint: Option<Vec<u8>>) -> Self {
        operation.seed = snapshot.seed;
        Self {
            operation,
            snapshot: Some(snapshot),
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
            checkpoint,
            restore: None,
            child: None,
            child_commit: None,
            output: Some(semio_framework_job_payload::RetainedPayloadBuilder::new(semio_framework_job_payload::JobPayloadStream::CommitOutput)),
            output_chunk: None,
            output_cursor: 0,
            output_next: None,
            encode_phase: EncodePhase::Open,
            encode_cursor: 0,
            preview_units: 0,
            publish_intermediate: true,
            last_preview_ms: None,
            publication: None,
            lent: BitmapInferenceJobLent::Own,
            restore_done: false,
            child_end: None,
            output_payload: None,
            closing: false,
            scratch_seed: None,
            scratch_retirement: None,
        }
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    fn snapshot(&self) -> &BitmapSnapshot {
        self.snapshot.as_ref().expect("live bitmap inference snapshot")
    }

    fn has_scratch(&self) -> bool {
        self.snapshot.is_some() || self.sample.is_some() || self.extracted.is_some()
            || self.decoder.is_some() || self.topology.is_some() || self.fixed.capacity()!=0
            || self.entropy.capacity()!=0 || self.pixels.capacity()!=0
            || self.checkpoint.is_some() || self.child_commit.is_some()
            || self.child.is_some() || self.restore.is_some()
    }

    fn take_scratch(&mut self) -> Option<BitmapInferenceScratch> {
        use BitmapInferenceScratch::*;
        self.snapshot.take().map(Snapshot)
            .or_else(||self.sample.take().map(Sample))
            .or_else(||self.extracted.take().map(Extracted))
            .or_else(||self.decoder.take().map(Decoder))
            .or_else(||self.topology.take().map(Topology))
            .or_else(||(self.fixed.capacity()!=0).then(||Fixed(std::mem::take(&mut self.fixed))))
            .or_else(||(self.entropy.capacity()!=0).then(||Entropy(std::mem::take(&mut self.entropy))))
            .or_else(||(self.pixels.capacity()!=0).then(||Pixels(std::mem::take(&mut self.pixels))))
            .or_else(||self.checkpoint.take().map(Checkpoint))
            .or_else(||self.child_commit.take().map(Commit))
            .or_else(||self.child.take().map(Child))
            .or_else(||self.restore.take().map(Restore))
    }

    pub(crate) fn progress(&self) -> (usize, usize) {
        match self.stage {
            BitmapInferenceStage::Sample => (0, 1),
            BitmapInferenceStage::Extract => (0, 1),
            BitmapInferenceStage::Topology => (0, 1),
            BitmapInferenceStage::Fixed => (self.cursor, self.snapshot().pinned.len()),
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
        let extracted = self.extracted.as_ref()?;
        let decoder = &extracted.decoder;
        let model = &extracted.model;
        (0..model.pattern_count()).map(engine::ids::PatternId::from_index).find(|pattern| decoder.anchor_tile(*pattern).get() == color)
    }

    /// 🩺 A colour that is never a pattern's ANCHOR cannot be asked for: the decoder
    /// reads an assignment back through the anchor convention, so a pin on such a
    /// colour is unsatisfiable by construction and is reported rather than dropped.
    pub(crate) fn advance_compile(&mut self) -> Result<(), String> {
        match self.stage {
            BitmapInferenceStage::Sample => {
                let input = &self.snapshot().input;
                let indices = &input.pixels;
                if indices.len() != (input.width as usize) * (input.height as usize) {
                    return Err("bitmap-inference-malformed-input".into());
                }
                if indices.iter().any(|index| usize::from(*index) >= input.palette.len()) {
                    return Err("bitmap-inference-unknown-palette-index".into());
                }
                let tiles = indices.iter().map(|index| engine::ids::TileId(u32::from(*index))).collect();
                self.sample = Some(engine::extract::Sample2d::new(input.width as usize, input.height as usize, tiles));
                self.stage = BitmapInferenceStage::Extract;
            }
            BitmapInferenceStage::Extract => {
                let sample = self.sample.as_ref().ok_or("bitmap-inference-sample-missing")?;
                let model = &self.snapshot().model;
                let config = engine::extract::Extract2dConfig { window: model.pattern_size.max(1) as usize, periodic_input: model.periodic_input, symmetry: symmetry_group(model.symmetry) };
                let extracted = engine::extract::extract_2d(std::slice::from_ref(sample), &config).map_err(|error| format!("{error:?}"))?;
                if extracted.model.pattern_count() > MAX_BITMAP_PATTERNS {
                    return Err("bitmap-inference-pattern-universe-exceeded".into());
                }
                self.extracted = Some(extracted);
                self.stage = BitmapInferenceStage::Topology;
            }
            BitmapInferenceStage::Topology => {
                let output = &self.snapshot().output;
                let boundary = if output.periodic { engine::grid2d::Boundary::Wrap } else { engine::grid2d::Boundary::Open };
                let topology = engine::grid2d::Grid2dTopology::new(output.width as usize, output.height as usize, &engine::grid2d::Stencil2d::VonNeumann, stencil_relations(), boundary, boundary, None).map_err(|error| format!("{error:?}"))?;
                self.topology = Some(topology);
                self.cursor = 0;
                self.stage = BitmapInferenceStage::Fixed;
            }
            BitmapInferenceStage::Fixed => {
                if let Some(pin) = self.snapshot().pinned.get(self.cursor).copied() {
                    let output = &self.snapshot().output;
                    if pin.x >= output.width || pin.y >= output.height {
                        return Err("bitmap-inference-pin-outside-output".into());
                    }
                    let pattern = self.pattern_for_color(pin.color).ok_or("bitmap-inference-unreachable-pin")?;
                    let node = engine::ids::NodeId::from_index((pin.y as usize) * (output.width as usize) + pin.x as usize);
                    self.fixed.push((node, pattern));
                    self.cursor += 1;
                } else {
                    self.push_ground_pins();
                    let topology = self.topology.take().ok_or("bitmap-inference-topology-missing")?;
                    let engine::extract::ExtractedModel2d { model, decoder } = self.extracted.take().ok_or("bitmap-inference-model-missing")?;
                    self.decoder = Some(decoder);
                    self.entropy = self.prior_entropy(&model);
                    let fixed = std::mem::take(&mut self.fixed);
                    if let Some(checkpoint) = self.checkpoint.take() {
                        self.restore = Some(engine::job::WfcRestore::new(self.operation, model, topology, engine::job::WfcJobConfig::default(), None, fixed, checkpoint)?);
                        self.stage = BitmapInferenceStage::Restore;
                    } else {
                        let child = engine::job::WfcJob::new(self.operation, model, topology, engine::job::WfcJobConfig::default(), None, fixed);
                        self.child = Some(if self.publish_intermediate { child } else { child.with_terminal_publications() });
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
        let Some(ground) = self.snapshot().model.ground else { return };
        let Some(pattern) = self.pattern_for_color(ground) else { return };
        let output = &self.snapshot().output;
        let width = output.width as usize;
        let row = (output.height as usize).saturating_sub(1);
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
        let universe = compiled_pattern_entropy(model);
        let output = &self.snapshot().output;
        let cells = (output.width as usize) * (output.height as usize);
        (0..cells).map(|cell| if self.fixed.iter().any(|(node, _)| node.index() == cell) { 0.0 } else { universe }).collect()
    }

    /// 🖼️ Turns the collapsed pattern assignment back into palette indices through the extraction
    /// decoder's anchor convention, then into the base64 buffer the commit carries.
    pub(crate) fn decode_commit(&mut self) -> Result<(), String> {
        let commit = self.child_commit.as_ref().ok_or("bitmap-inference-commit-missing")?;
        let decoder = self.decoder.as_ref().ok_or("bitmap-inference-decoder-missing")?;
        let snapshot = self.snapshot();
        let cells = (snapshot.output.width as usize) * (snapshot.output.height as usize);
        if commit.assignment.len() != cells {
            return Err("bitmap-inference-commit-shape".into());
        }
        let palette_len = snapshot.input.palette.len() as u32;
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
    pub(crate) fn encode_one(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::receive_original_retained_work;
        if !self.output.as_ref().expect("output builder").is_initialized() {
            let turn = self.output.as_mut().unwrap().advance_initialization(context); receive_original_retained_work(context, turn)?; return Ok(());
        }
        if let Some(chunk) = self.output_chunk.as_ref() {
            if self.output_cursor < chunk.len() {
                let turn = self.output.as_mut().unwrap().append_original(context, chunk, &mut self.output_cursor);
                receive_original_retained_work(context, turn)?; return Ok(());
            }
            let step = semio_framework_job::close_original_granted_step(context, |grant| {
                let released_bytes = self.output_chunk.as_ref().unwrap().capacity();
                if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_release_bytes < released_bytes { return semio_framework_value::RetainedWorkOutcome::<_, semio_framework_value::ValueError> { result: Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() }), work: 0 }; }
                self.output_chunk = None;
                let (phase, cursor) = self.output_next.take().expect("next bitmap encoder");
                self.encode_phase = phase; self.encode_cursor = cursor;
                semio_framework_value::RetainedWorkOutcome { result: Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_job::RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() } }), work: 1 }
            })?;
            let _ = step;
            return Ok(());
        }
        if self.encode_phase == EncodePhase::Close && self.encode_cursor == 1 {
            let output = self.output.as_mut().unwrap();
            if output.published().is_none() { let turn = output.seal(context); receive_original_retained_work(context, turn)?; return Ok(()); }
            let turn = output.take_finished(&mut self.output_payload, context.retained_grant()); receive_original_retained_work(context, turn)?;
            if self.output_payload.is_some() { self.stage = BitmapInferenceStage::Complete; }
            return Ok(());
        }
        let grant = context.retained_grant();
        if context.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_capacity_bytes < BITMAP_ENCODE_CHUNK + 128 { return Ok(()); }
        let (chunk, phase, cursor) = self.next_encode_chunk().unwrap_or_else(|| ("]}".to_string(), EncodePhase::Close, 1));
        self.output_chunk = Some(chunk.into_bytes()); self.output_cursor = 0; self.output_next = Some((phase, cursor));
        let capacity = self.output_chunk.as_ref().unwrap().capacity();
        context.consume_fuel(1);
        context.consume_retained(semio_framework_job::RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() })?;
        Ok(())
    }

}

/// 🚦️ What one bounded run decided before any outcome is lent.
pub(crate) enum BitmapInferenceJobRun {
    Yield,
    Cancelled,
    Staged,
    StepRestore,
    StepChild,
    StepOutput,
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
                        if let Some(restore) = self.restore.as_mut() { restore.begin_close(); }
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
                BitmapInferenceStage::EncodeCommit => return BitmapInferenceJobRun::StepOutput,
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
    fn retire_delivered<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        semio_s_plugin_wfc_engine::job::close_publication_step(&mut self.publication, context)?;
        Ok(None)
    }

    /// 📏️ Quotes the next close turn in the order the ladder spends them.
    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inference close depth overflow"))?;
            Ok(demand)
        };
        if self.publication.is_some() { return semio_s_plugin_wfc_engine::job::publication_retirement_demands(&self.publication); }
        if let Some(output) = self.output.as_ref() {
            return if output.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }) } else { nested(output.retirement_demands()?) };
        }
        if let Some(payload) = self.output_payload.as_ref() {
            return if payload.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }) } else { nested(payload.retirement_demands()?) };
        }
        if let Some(restore) = self.restore.as_ref().filter(|restore|!restore.retirement_ready()) {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: restore.next_close_copy_byte_demand()?, capacity_bytes: restore.next_close_capacity_byte_demand(0)?, release_bytes: restore.next_close_release_byte_demand()?, depth: restore.next_close_depth_demand()?.saturating_add(1) });
        }
        if let Some(child) = self.child.as_ref().filter(|child|!child.retirement_ready()) {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: child.next_close_copy_byte_demand()?, capacity_bytes: child.next_close_capacity_byte_demand(0)?, release_bytes: child.next_close_release_byte_demand()?, depth: child.next_close_depth_demand()?.saturating_add(1) });
        }
        if let Some(chunk) = self.output_chunk.as_ref() { return Ok(semio_framework_value::RetirementDemand { release_bytes: chunk.capacity(), depth: 1, ..Default::default() }); }
        if let Some(ticket) = &self.scratch_retirement { return semio_framework_value::factory_ticket_demands(ticket, 0); }
        if self.scratch_seed.is_some() || self.has_scratch() {
            let capacity_bytes=semio_framework_value::retirement::owned_retirement_birth_bytes::<BitmapInferenceScratch>();
            return Ok(semio_framework_value::RetirementDemand {
                copy_bytes: std::mem::size_of::<Option<BitmapInferenceScratch>>()+capacity_bytes,
                capacity_bytes,
                depth: 1,
                ..Default::default()
            });
        }
        Ok(semio_framework_value::RetirementDemand::default())
    }
}

impl BitmapInferenceJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::JobOutcomeBorrow;
        if self.publication.as_ref().is_some_and(|publication| publication.needs_retirement()) {
            return self.retire_delivered(context);
        }
        if self.closing || context.is_cancelled() {
            return Ok(JobOutcomeBorrow::admit_cancelled(context)?);
        }
        if context.should_yield() {
            return Ok(None);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            self.stage_fault(b"stale-bitmap-inference-operation".to_vec());
        }
        if self.publication.is_some() {
            self.lent = BitmapInferenceJobLent::Own;
            return self.publication.as_mut().expect("a staged inference publication").poll(context);
        }
        if self.output_payload.is_some() {
            return Ok(JobOutcomeBorrow::admit_complete(context, self.child.as_ref().and_then(|child| child.commit_state()), self.output_payload.as_ref())?);
        }
        match self.run(context) {
            BitmapInferenceJobRun::Yield | BitmapInferenceJobRun::Staged => Ok(None),
            BitmapInferenceJobRun::Cancelled => Ok(JobOutcomeBorrow::admit_cancelled(context)?),
            BitmapInferenceJobRun::StepRestore => {
                let result = self.restore.as_mut().expect("restore job").step_value(context)?;
                if matches!(result, Some(JobOutcomeBorrow::Complete { .. })) {
                    self.restore_done = true;
                    return Ok(None);
                }
                self.lent = BitmapInferenceJobLent::Restore;
                Ok(result)
            }
            BitmapInferenceJobRun::StepOutput => { self.encode_one(context)?; Ok(None) }
            BitmapInferenceJobRun::StepChild => {
                let result = self.child.as_mut().expect("WFC child").step_value(context)?;
                let end = if matches!(result, Some(JobOutcomeBorrow::Complete { .. })) {
                    Some(InferenceChildEnd::Complete)
                } else {
                    if let Some(JobOutcomeBorrow::Fault { detail, .. }) = &result {
            if engine::job::payload_equals(detail, b"wfc-unsatisfiable") {
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
}

impl InteractiveJob for BitmapInferenceJob {
    fn step<'a>(&'a mut self, context:&mut semio_framework_job::StepContext<'_>, _claim:&mut semio_framework_job::OriginalJobReturnClaim)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(context))}

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
        self.closing = true;
        if let Some(restore) = self.restore.as_mut() {
            restore.begin_close();
        }
        if let Some(child) = self.child.as_mut() {
            child.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_nested_step(cx,|cx,grant|{
            use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress};
            self.begin_close();
            let demand = match self.close_demands() {
                Ok(demand) => demand,
                Err(error) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx)),
            };
            if self.terminal_is_empty() {
                return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
            }
            if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
            }
            let one = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            let child_grant = semio_framework_job::RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
            if self.publication.is_some() {
                let step = semio_s_plugin_wfc_engine::job::close_publication_step(&mut self.publication, cx)?;
                return Ok(match step { InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress }, step => step });
            }
            if let Some(output) = self.output.as_mut() {
                if output.terminal_is_empty() {
                    cx.consume_fuel(1);
                    self.output = None;
                    return Ok(InteractiveJobCloseStep::Pending { progress: one });
                }
                let step = close_effect(cx, child_grant, |effect| output.close_step(effect))?;
                return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
            }
            if let Some(payload) = self.output_payload.as_mut() {
                if payload.terminal_is_empty() {
                    cx.consume_fuel(1);
                    self.output_payload = None;
                    return Ok(InteractiveJobCloseStep::Pending { progress: one });
                }
                let step = close_effect(cx, child_grant, |effect| payload.close_step(effect))?;
                return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
            }
            if let Some(restore) = self.restore.as_mut().filter(|restore|!restore.retirement_ready()) {
                let step = cx.with_retained_loan(child_grant, |same| restore.close_step(same))?;
                return Ok(match step {
                    InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                    step => step,
                });
            }
            if let Some(child) = self.child.as_mut().filter(|child|!child.retirement_ready()) {
                let step = cx.with_retained_loan(child_grant, |same| child.close_step(same))?;
                return Ok(match step {
                    InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                    step => step,
                });
            }
            if let Some(chunk) = self.output_chunk.take() {
                let released_bytes = chunk.capacity(); cx.consume_fuel(1); drop(chunk);
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { released_bytes, ..one } });
            }
            if self.scratch_retirement.is_some() {
                return semio_framework_job::close_original_effect_step(cx,demand,|effect|
                    semio_framework_value::close_factory_ticket(&mut self.scratch_retirement,0,effect));
            }
            semio_framework_job::close_original_retained_effect_step(cx,demand,|effect|{
                use semio_framework_value::retained_clone::RetainedCloneStep;
                if self.scratch_seed.is_none() { self.scratch_seed=self.take_scratch(); }
                let adopted=semio_framework_value::retirement::admit_owned_retirement(&mut self.scratch_seed,effect).result?;
                let Some((ticket,progress))=adopted else { return Ok(RetainedCloneStep::Progress(Default::default())); };
                self.scratch_retirement=Some(ticket);
                Ok::<_,semio_framework_value::ValueError>(RetainedCloneStep::Progress(progress))
            })
        })
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

    /// 🚪️ Checks that every owned worker resource has drained.
    fn terminal_is_empty(&self) -> bool {
        self.publication.is_none() && self.output.is_none() && self.output_payload.is_none()
            && self.output_chunk.is_none()
            && self.scratch_seed.is_none() && self.scratch_retirement.is_none() && !self.has_scratch()
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

    semio_plugin::retained_command::typed_tool_job_factory_admission!(|operation, payload| BitmapInferenceJob::new(operation, payload));
}

pub fn register_bitmap_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(BitmapInferenceJobFactory::default())
}

/// 🏁 Explicit headless adapter over the same complete parent job the public factory hands out.
#[cfg(any(test, feature = "testkit"))]
pub fn solve_with_job(snapshot: &BitmapSnapshot) -> Result<BitmapInferenceCommit, semio_s_plugin_wfc_engine::job::HeadlessFailure<BitmapInferenceJob, BitmapInferenceCommit>> {
    solve_with_clock(snapshot, semio_framework_job::default_now_us)
}

/// 🧮️ The same headless adapter driven by an injected clock, so correctness laws run on
/// [`semio_framework_job::logical_now_us`] and never on a descheduled thread's wall clock.
#[cfg(any(test, feature = "testkit"))]
pub fn solve_with_clock(snapshot: &BitmapSnapshot, now_us: fn() -> Option<u64>) -> Result<BitmapInferenceCommit, semio_s_plugin_wfc_engine::job::HeadlessFailure<BitmapInferenceJob, BitmapInferenceCommit>> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(1), snapshot.seed);
    let mut job = BitmapInferenceJob::new(operation, BitmapInferenceRequest { snapshot: Some(snapshot.clone()), document: None, checkpoint: None }).map_err(semio_s_plugin_wfc_engine::job::HeadlessFailure::rejected)?;
    job.publish_intermediate = false;
    engine::job::run_headless_supervised(
        job,
        operation,
        now_us,
        engine::job::HeadlessSite { site: "wfc.bitmap.inference.headless", label: "bitmap-inference-headless", stage: semio_framework_job::InteractiveStage::BackgroundStep, fuel_per_step: HEADLESS_FUEL_PER_STEP, step_budget_us: HEADLESS_STEP_BUDGET_US,
            retained_stock: semio_framework_job::RetainedCloneGrant { maximum_items: 16_000_000, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 256 },
            lifetime_fuel: 100_000_000, lifetime_budget_us: 60_000_000,
        },
        |_job| None,
        |_job, outcome| {
            let output = match outcome {
                semio_s_plugin_wfc_engine::job::HeadlessOutcome::Complete(bytes) => bytes,
                semio_s_plugin_wfc_engine::job::HeadlessOutcome::Fault(bytes) => return Err(String::from_utf8_lossy(bytes).into_owned()),
                semio_s_plugin_wfc_engine::job::HeadlessOutcome::Cancelled => return Err("bitmap-inference-cancelled".into()),
            };
            std::str::from_utf8(output)
                .map_err(|error| format!("bitmap-invalid-commit:{error}"))
                .and_then(|text| crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value::<BitmapInferenceCommit>(text).map_err(|error| format!("bitmap-invalid-commit:{error}")))
        },
    )
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

