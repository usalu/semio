//! 💡️ Bitmap inferences — THE SOLVE ITSELF IS AN INFERENCE. `BitmapSnapshot` only ever persists the
//! PROBLEM (input sample, palette, output extent, model parameters, pins, seed); the solved output
//! bitmap, the contradiction verdict and the entropy map are all derived here, never
//! mutation-authored state.
//!
//! The route is the classic overlapping model end to end: `extract_2d` learns `N × N` patterns from
//! the input sample (expanded under the first `model.symmetry` elements of D4), `Grid2dTopology`
//! gives the output its four-neighbour stencil with boundaries taken from `output.periodic`, and the
//! resumable `WfcJob` — the same one every interactive caller drives — collapses it. Determinism:
//! every input is a snapshot field, `seed` included, and no ambient randomness enters, so `DepHash`
//! caching over `BitmapSolve`/`BitmapContradiction`/`BitmapEntropy` is sound.

use crate::schema::snapshot::{encode_base64, BitmapSnapshot};
use semio_s_plugin_wfc_engine as engine;

//#region 📦️RetainedPayload
/// 📦️ One single-page payload, with the job module's EXACT source handback on refusal. Dropping a
/// rejected page without taking its source back trips that module's own lifecycle assertion and
/// then aborts the process from a second panic inside `RetainedJobPayload::drop`. An empty payload
/// is the honest answer to a refused admission; leaking the page never was.
/// 🧹️ Releases every page a retained payload still owns. A `CommitCandidate` carries TWO of them
/// (`state` AND `output`); dropping either without running its close ladder trips
/// `RetainedJobPayload`'s own Drop assertion from inside `step`, which aborts the process rather
/// than failing a test.
fn retire_payload(payload: &mut semio_framework_job::RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

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
    output_schema: include_str!("🔣️.json"),
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
pub fn bitmap_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: BITMAP_INFERENCE_TOOL_ID,
        inference: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Identity

//#region 🔖️Admission
/// 🚧️ The largest input sample one solve may learn from — 256 × 256 index bytes.
const MAX_BITMAP_INPUT_CELLS: usize = 65_536;
/// 🚧️ The largest output one solve may collapse — 256 × 256 cells, which also bounds the entropy
/// map the commit carries.
const MAX_BITMAP_OUTPUT_CELLS: usize = 65_536;
/// 🚧️ The largest pattern universe extraction may produce before the compatibility table itself
/// becomes the binding cost.
const MAX_BITMAP_PATTERNS: usize = 4_096;
const MAX_BITMAP_PINS: usize = 65_536;
const MAX_BITMAP_OUTPUT_BYTES: usize = 4 << 20;
/// 📝️ How many base64 characters one encode step appends — well under one payload page, so a step
/// never needs a page it cannot admit.
const BITMAP_ENCODE_CHUNK: usize = 1_024;
/// 🏎️ The headless adapter is a BATCH driver, not an interactive one: a single fuel unit per session
/// step turns one small collapse into tens of thousands of session round trips (a 6 × 4 output took
/// over a minute that way, measured). A large fuel budget per step keeps every step bounded while
/// letting one call do a whole propagation wave.
const HEADLESS_FUEL_PER_STEP: u64 = 4_096;
/// ⏱️ STRICTLY below `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 000 µs). A session step
/// that aims past the ceiling reports a contract violation on EVERY step, and four consecutive
/// violations quarantine the whole session with `job-session.terminal-fault` — which is what a 50 ms
/// budget did to every live solve in the react shell while the native tests stayed green (a test
/// binary installs no monotonic clock, so it records no violation at all). The budget bounds one step,
/// never the whole collapse: the driver resumes until the job is terminal.
const HEADLESS_STEP_BUDGET_US: u64 = 4_000;
const PARENT_PREVIEW_UNIT_INTERVAL: u64 = 16;
const PARENT_PREVIEW_TIME_INTERVAL_MS: u64 = 16;
//#endregion 🔖️Admission

//#region 🔖️Protocol
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
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

/// 🏁 What one solve concludes with: the output bitmap as base64 palette indices (row-major), the
/// satisfiability verdict, and the per-cell prior entropy map.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapInferenceCommit {
    pub pixels: String,
    pub contradiction: bool,
    pub entropy: Vec<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
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
enum EncodePhase {
    Open,
    Pixels,
    Verdict,
    Entropy,
    Close,
}
//#endregion 🔖️Protocol

//#region 🔖️Compile
/// 🔄️ The classic `1..=8` overlapping-model symmetry count: the first `count` elements of D4 listed
/// as the reference implementation lists them — rotations and reflections ALTERNATING, `e, m, r,
/// rm, r², r²m, r³, r³m` — not `Transform2d::ALL`'s rotations-then-reflections order. The
/// difference is load-bearing at every count below eight: the alternating order makes `2` a MIRROR
/// (identity plus a horizontal flip), which is what an author reaching for "2" means, while the
/// declaration order would make it identity plus a quarter turn and stand a meadow on its side.
/// `1` learns the sample verbatim and `8` is the full dihedral group either way.
pub const BITMAP_SYMMETRY_ORDER: [engine::symmetry::Transform2d; 8] = [
    engine::symmetry::Transform2d::Identity,
    engine::symmetry::Transform2d::FlipH,
    engine::symmetry::Transform2d::Rot90,
    engine::symmetry::Transform2d::FlipDiag,
    engine::symmetry::Transform2d::Rot180,
    engine::symmetry::Transform2d::FlipV,
    engine::symmetry::Transform2d::Rot270,
    engine::symmetry::Transform2d::FlipAntiDiag,
];

pub fn symmetry_group(count: u32) -> engine::symmetry::SymmetryGroup2d {
    let take = (count.clamp(1, 8)) as usize;
    engine::symmetry::SymmetryGroup2d::Custom(BITMAP_SYMMETRY_ORDER[..take].to_vec())
}

/// 🗺️ The four relation ids `extract_2d` declares, in `Stencil2d::VonNeumann.offsets()` order.
/// `extract_2d` builds its model on a FRESH `ModelBuilder` and calls `declare_stencil_relations`
/// before anything else, so the relations it registers are exactly `0..4` in that order — the
/// coupling this function makes explicit instead of leaving implicit at the topology call site.
fn stencil_relations() -> Vec<engine::ids::RelationId> {
    (0..4).map(engine::ids::RelationId::from_index).collect()
}

/// 🧵 Worker-owned parent transaction for bitmap extraction, collapse and commit encoding.
pub struct BitmapInferenceJob {
    operation: semio_framework_job::Operation,
    snapshot: BitmapSnapshot,
    stage: BitmapInferenceStage,
    cursor: usize,
    sample: Option<engine::extract::Sample2d>,
    extracted: Option<engine::extract::ExtractedModel2d>,
    decoder: Option<engine::extract::PatternDecoder2d>,
    topology: Option<engine::grid2d::Grid2dTopology>,
    fixed: Vec<(engine::ids::NodeId, engine::ids::PatternId)>,
    entropy: Vec<f64>,
    pixels: String,
    contradiction: bool,
    checkpoint: Option<Vec<u8>>,
    restore: Option<engine::job::WfcRestore<engine::grid2d::Grid2dTopology>>,
    child: Option<engine::job::WfcJob<engine::grid2d::Grid2dTopology>>,
    child_commit: Option<engine::job::WfcCommit>,
    final_checkpoint: Option<semio_framework_job::RetainedJobPayload>,
    output: Option<semio_framework_job::RetainedJobPayloadWriter>,
    rejected_output_page: Option<semio_framework_job::JobPayloadPageSource>,
    encode_phase: EncodePhase,
    encode_cursor: usize,
    preview_units: u64,
    last_preview_ms: Option<u64>,
    closing: bool,
}

impl BitmapInferenceJob {
    fn new(mut operation: semio_framework_job::Operation, request: BitmapInferenceRequest) -> Result<Self, String> {
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
            final_checkpoint: None,
            output: Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput)),
            rejected_output_page: None,
            encode_phase: EncodePhase::Open,
            encode_cursor: 0,
            preview_units: 0,
            last_preview_ms: None,
            closing: false,
        })
    }

    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    fn progress(&self) -> (usize, usize) {
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

    /// 🎨️ The first pattern whose anchor tile is that palette index — the anchor convention is what
    /// makes a per-cell colour pin expressible as a pattern pin at all.
    fn pattern_for_color(&self, color: u32) -> Option<engine::ids::PatternId> {
        let decoder = self.decoder.as_ref()?;
        let model = &self.extracted.as_ref()?.model;
        (0..model.pattern_count()).map(engine::ids::PatternId::from_index).find(|pattern| decoder.anchor_tile(*pattern).get() == color)
    }

    /// 🩺 A colour that is never a pattern's ANCHOR cannot be asked for: the decoder
    /// reads an assignment back through the anchor convention, so a pin on such a
    /// colour is unsatisfiable by construction and is reported rather than dropped.
    fn advance_compile(&mut self) -> Result<(), String> {
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
    fn push_ground_pins(&mut self) {
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
    fn prior_entropy(&self, model: &engine::model::CompiledModel) -> Vec<f64> {
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
    fn decode_commit(&mut self) -> Result<(), String> {
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

    /// 📝️ The next slice of commit JSON to append, with the cursor state it leaves behind. Peeked
    /// rather than consumed so a page that cannot take it is committed and the SAME slice is
    /// retried on the next page — the commit text is generated incrementally and never materialised
    /// as one oversized `String`, which is what keeps the guest's contiguous-allocation ceiling out
    /// of the picture for a 256 × 256 output.
    fn next_encode_chunk(&self) -> Option<(String, EncodePhase, usize)> {
        match self.encode_phase {
            EncodePhase::Open => Some((r#"{"pixels":""#.to_string(), EncodePhase::Pixels, 0)),
            EncodePhase::Pixels => {
                if self.encode_cursor >= self.pixels.len() {
                    return Some((String::new(), EncodePhase::Verdict, 0));
                }
                let end = (self.encode_cursor + BITMAP_ENCODE_CHUNK).min(self.pixels.len());
                Some((self.pixels[self.encode_cursor..end].to_string(), EncodePhase::Pixels, end))
            }
            EncodePhase::Verdict => {
                let verdict = if self.contradiction { r#"","contradiction":true,"entropy":["# } else { r#"","contradiction":false,"entropy":["# };
                Some((verdict.to_string(), EncodePhase::Entropy, 0))
            }
            EncodePhase::Entropy => {
                if self.encode_cursor >= self.entropy.len() {
                    return Some((String::new(), EncodePhase::Close, 0));
                }
                let separator = if self.encode_cursor == 0 { "" } else { "," };
                Some((format!("{separator}{}", protocol::json::to_json_string(&self.entropy[self.encode_cursor])), EncodePhase::Entropy, self.encode_cursor + 1))
            }
            EncodePhase::Close => None,
        }
    }

    /// 📝️ Fills ONE payload page to capacity before asking for another. One page per small write
    /// would burn the operation's 256-page budget on a handful of entropy values — the exact
    /// admission refusal this encoder is shaped to avoid.
    fn encode_one(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
        let source = self.rejected_output_page.take().unwrap_or_default();
        let mut writer = self.output.take().ok_or("bitmap-output-writer-missing")?;
        let outcome = self.encode_into(context, &mut writer, source);
        self.output = Some(writer);
        outcome
    }

    fn encode_into(&mut self, context: &mut semio_framework_job::StepContext<'_>, writer: &mut semio_framework_job::RetainedJobPayloadWriter, source: semio_framework_job::JobPayloadPageSource) -> Result<bool, String> {
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

impl semio_framework_job::InteractiveJob for BitmapInferenceJob {
    /// 🩺 `WfcJob` reports an exhausted search as a `wfc-unsatisfiable` FAULT. For this
    /// artifact that is an ANSWER, not a failure: the classic overlapping model asks
    /// "does this sample tile that output", and "no" is the verdict the output window
    /// shows. Every other fault stays a fault.
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        use semio_framework_job::StepOutcome;
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, b"stale-bitmap-inference-operation");
            return StepOutcome::Fault(semio_framework_job::JobFault { detail });
        }
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
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                BitmapInferenceStage::Restore => {
                    let outcome = self.restore.as_mut().expect("restore job").step(context);
                    return match outcome {
                        StepOutcome::Complete(mut candidate) => {
                            retire_payload(&mut candidate.state);
                            retire_payload(&mut candidate.output);
                            self.child = self.restore.as_mut().expect("restore job").take_job();
                            self.restore = None;
                            self.stage = BitmapInferenceStage::Solve;
                            self.emit_preview(context)
                        }
                        other => other,
                    };
                }
                BitmapInferenceStage::Solve => {
                    let mut outcome = self.child.as_mut().expect("WFC child").step(context);
                    if let StepOutcome::Fault(fault) = &outcome {
                        if engine::job::payload_bytes(&fault.detail) == b"wfc-unsatisfiable" {
                            engine::job::retire_outcome(&mut outcome);
                            if let Some(child) = self.child.as_mut() {
                                engine::job::close_job(child);
                            }
                            self.child = None;
                            self.contradiction = true;
                            self.cursor = 0;
                            self.stage = BitmapInferenceStage::Decode;
                            return self.emit_preview(context);
                        }
                    }
                    return match outcome {
                        StepOutcome::Complete(mut candidate) => {
                            retire_payload(&mut candidate.output);
                            self.final_checkpoint = Some(candidate.state);
                            self.child_commit = self.child.as_mut().expect("WFC child").take_completed_commit();
                            self.contradiction = self.child_commit.is_none();
                            self.child = None;
                            self.cursor = 0;
                            self.stage = BitmapInferenceStage::Decode;
                            self.emit_preview(context)
                        }
                        other => other,
                    };
                }
                BitmapInferenceStage::Decode => {
                    if self.contradiction {
                        self.pixels = String::new();
                        self.stage = BitmapInferenceStage::EncodeCommit;
                    } else if let Err(error) = self.decode_commit() {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                }
                BitmapInferenceStage::EncodeCommit => match self.encode_one(context) {
                    Ok(true) => {
                        let output = match self.output.take().expect("bitmap output writer").finish() {
                            Ok(output) => output,
                            Err(mut writer) => {
                                writer.begin_close();
                                self.output = Some(writer);
                                return StepOutcome::Yield;
                            }
                        };
                        return StepOutcome::Complete(semio_framework_job::CommitCandidate { state: self.final_checkpoint.take().unwrap_or_else(|| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState)), output });
                    }
                    Ok(false) => {}
                    Err(error) => {
                        let detail = retained_payload(context, semio_framework_job::JobPayloadStream::Fault, error.as_bytes());
                        return StepOutcome::Fault(semio_framework_job::JobFault { detail });
                    }
                },
                BitmapInferenceStage::Complete => unreachable!("complete returns immediately"),
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
        self.closing = true;
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

    /// 🚪️ Reports OWNERSHIP only, never a `closing` flag: the framework calls `begin_close()` itself
    /// at close stage 0 and answers `Blocked` for as long as this returns `false`, so gating on a
    /// locally set flag makes any `while !terminal_is_empty()` driver spin forever.
    fn terminal_is_empty(&self) -> bool {
        self.output.is_none() && self.final_checkpoint.is_none() && self.restore.is_none() && self.child.is_none() && self.rejected_output_page.is_none()
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
        let mut request: BitmapInferenceRequest = protocol::json::from_json_str(payload_text).map_err(|error| semio_framework::ToolJobFactoryError::new(format!("bitmap-inference-wire-decode:{error}")))?;
        if checkpoint.is_some() {
            request.checkpoint = checkpoint;
        }
        BitmapInferenceJob::new(operation, request).map_err(semio_framework::ToolJobFactoryError::new)
    }
}

pub fn register_bitmap_inference_factory(bus: &semio_framework::ActionBus) -> Result<(), semio_framework::ToolRegistrationError> {
    bus.register_once(BitmapInferenceJobFactory::default())
}

/// 🧱 Compiled overlapping collapse ready for an interactive `WfcJob` — shared by the headless
/// oracle and the fill tool run so both drive the same model, topology, pins and decoder.
pub struct BitmapCollapseParts {
    pub model: engine::model::CompiledModel,
    pub topology: engine::grid2d::Grid2dTopology,
    pub fixed: Vec<(engine::ids::NodeId, engine::ids::PatternId)>,
    pub decoder: engine::extract::PatternDecoder2d,
}

/// 🧱 Learn patterns, build the output grid and apply pins for one snapshot.
pub fn compile_bitmap_collapse(snapshot: &BitmapSnapshot) -> Result<BitmapCollapseParts, String> {
    let input_cells = (snapshot.input.width as usize).saturating_mul(snapshot.input.height as usize);
    let output_cells = (snapshot.output.width as usize).saturating_mul(snapshot.output.height as usize);
    if input_cells == 0 || output_cells == 0 || input_cells > MAX_BITMAP_INPUT_CELLS || output_cells > MAX_BITMAP_OUTPUT_CELLS || snapshot.pinned.len() > MAX_BITMAP_PINS || snapshot.input.palette.is_empty() {
        return Err("bitmap-inference-admission-exceeded".into());
    }
    let indices = snapshot.input.indices().ok_or("bitmap-inference-malformed-input")?;
    if indices.iter().any(|index| usize::from(*index) >= snapshot.input.palette.len()) {
        return Err("bitmap-inference-unknown-palette-index".into());
    }
    let tiles = indices.iter().map(|index| engine::ids::TileId(u32::from(*index))).collect();
    let sample = engine::extract::Sample2d::new(snapshot.input.width as usize, snapshot.input.height as usize, tiles);
    let config = engine::extract::Extract2dConfig { window: snapshot.model.pattern_size.max(1) as usize, periodic_input: snapshot.model.periodic_input, symmetry: symmetry_group(snapshot.model.symmetry) };
    let extracted = engine::extract::extract_2d(&[sample], &config).map_err(|error| format!("{error:?}"))?;
    if extracted.model.pattern_count() > MAX_BITMAP_PATTERNS {
        return Err("bitmap-inference-pattern-universe-exceeded".into());
    }
    let decoder = extracted.decoder.clone();
    let boundary = if snapshot.output.periodic { engine::grid2d::Boundary::Wrap } else { engine::grid2d::Boundary::Open };
    let topology = engine::grid2d::Grid2dTopology::new(snapshot.output.width as usize, snapshot.output.height as usize, &engine::grid2d::Stencil2d::VonNeumann, stencil_relations(), boundary, boundary, None).map_err(|error| format!("{error:?}"))?;
    let mut fixed = Vec::new();
    for pin in &snapshot.pinned {
        if pin.x >= snapshot.output.width || pin.y >= snapshot.output.height {
            return Err("bitmap-inference-pin-outside-output".into());
        }
        let pattern = (0..extracted.model.pattern_count()).map(engine::ids::PatternId::from_index).find(|pattern| decoder.anchor_tile(*pattern).get() == pin.color).ok_or("bitmap-inference-unreachable-pin")?;
        let node = engine::ids::NodeId::from_index((pin.y as usize) * (snapshot.output.width as usize) + pin.x as usize);
        fixed.push((node, pattern));
    }
    if let Some(ground) = snapshot.model.ground {
        if let Some(pattern) = (0..extracted.model.pattern_count()).map(engine::ids::PatternId::from_index).find(|pattern| decoder.anchor_tile(*pattern).get() == ground) {
            let width = snapshot.output.width as usize;
            let row = (snapshot.output.height as usize).saturating_sub(1);
            for x in 0..width {
                let node = engine::ids::NodeId::from_index(row * width + x);
                if !fixed.iter().any(|(fixed_node, _)| *fixed_node == node) {
                    fixed.push((node, pattern));
                }
            }
        }
    }
    Ok(BitmapCollapseParts { model: extracted.model, topology, fixed, decoder })
}

/// 🏁 Explicit headless adapter over the same complete parent job the public factory hands out.
pub fn solve_with_job(snapshot: &BitmapSnapshot) -> Result<BitmapInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    let job = BitmapInferenceJob::new(operation, BitmapInferenceRequest { snapshot: Some(snapshot.clone()), document: None, checkpoint: None })?;
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig { site: "wfc.bitmap.inference.headless", stage: semio_framework_job::InteractiveStage::BackgroundStep, fuel_per_step: HEADLESS_FUEL_PER_STEP, step_budget_us: HEADLESS_STEP_BUDGET_US },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return Err("bitmap-inference-headless-admission-rejected".into());
        }
    };
    loop {
        session.step().map_err(|error| format!("bitmap-inference-headless-contention:{error:?}"))?;
        let Some(mut outcome) = session.take_outcome() else { continue };
        let terminal = outcome.is_terminal();
        let payload_bytes = engine::job::payload_bytes;
        let result = match &outcome {
            semio_framework_job::StepOutcome::Complete(candidate) => Some(
                std::str::from_utf8(&payload_bytes(&candidate.output))
                    .map_err(|error| format!("bitmap-invalid-commit:{error}"))
                    .and_then(|text| protocol::json::from_json_str::<BitmapInferenceCommit>(text).map_err(|error| format!("bitmap-invalid-commit:{error}"))),
            ),
            semio_framework_job::StepOutcome::Cancelled => Some(Err("bitmap-inference-cancelled".into())),
            semio_framework_job::StepOutcome::Fault(fault) => Some(Err(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned())),
            semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) => None,
        };
        engine::job::retire_outcome(&mut outcome);
        if terminal {
            session.begin_close();
            let mut guard = 0u32;
            while !session.terminal_is_empty() {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                guard += 1;
                assert!(guard < 1_000_000, "the bitmap inference close ladder did not terminate");
            }
            return result.expect("terminal bitmap inference outcome has result");
        }
        session.resume().map_err(|error| format!("bitmap-inference-headless-resume:{error:?}"))?;
    }
}
//#endregion 🔖️Compile

//#region 🔖️Solve
/// 🏁 The solved output bitmap, or `Unsolved` for every non-`Solved` outcome (contradiction, budget,
/// cancellation) — see [`BitmapContradiction`] for the dedicated satisfiability verdict.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum BitmapSolveResult {
    #[default]
    Unsolved,
    Solved {
        pixels: String,
    },
}

pub struct BitmapSolve;

impl store::InferredField<BitmapSnapshot> for BitmapSolve {
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
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &BitmapSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(commit) if !commit.contradiction && !commit.pixels.is_empty() => BitmapSolveResult::Solved { pixels: commit.pixels },
            _ => BitmapSolveResult::Unsolved,
        }
    }
}
//#endregion 🔖️Solve

//#region 🔖️Contradiction
/// 🩺 The satisfiability verdict on its own, so a caller who only needs "can this sample tile that
/// output at all" never has to decode a whole bitmap to find out.
pub struct BitmapContradiction;

impl store::InferredField<BitmapSnapshot> for BitmapContradiction {
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
        protocol::json::to_json_string(snapshot).into_bytes()
    }
    fn compute(snapshot: &BitmapSnapshot, _key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        match solve_with_job(snapshot) {
            Ok(commit) => commit.contradiction,
            Err(_) => true,
        }
    }
}
//#endregion 🔖️Contradiction

//#region 🔖️Entropy
/// 🎲 The per-cell PRIOR entropy map, keyed by the cell's row-major index — the same vector the
/// cold-job commit carries, exposed as an ordinary `InferredField` for callers that want it without
/// running a full collapse.
pub struct BitmapEntropy;

impl store::InferredField<BitmapSnapshot> for BitmapEntropy {
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
        bytes.extend_from_slice(snapshot.input.pixels.as_bytes());
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

/// 🎲 The whole-universe Shannon entropy of the extracted pattern weight distribution — the value
/// every unfixed cell carries before propagation narrows anything.
pub fn pattern_universe_entropy(snapshot: &BitmapSnapshot) -> f64 {
    let Some(indices) = snapshot.input.indices() else { return 0.0 };
    let tiles = indices.iter().map(|index| engine::ids::TileId(u32::from(*index))).collect();
    let sample = engine::extract::Sample2d::new(snapshot.input.width as usize, snapshot.input.height as usize, tiles);
    let config = engine::extract::Extract2dConfig { window: snapshot.model.pattern_size.max(1) as usize, periodic_input: snapshot.model.periodic_input, symmetry: symmetry_group(snapshot.model.symmetry) };
    let Ok(extracted) = engine::extract::extract_2d(&[sample], &config) else { return 0.0 };
    let weights = extracted.model.weights();
    let total: f64 = (0..extracted.model.pattern_count()).map(|index| weights.w(engine::ids::PatternId::from_index(index))).sum();
    if total <= 0.0 {
        return 0.0;
    }
    -(0..extracted.model.pattern_count()).map(|index| weights.w(engine::ids::PatternId::from_index(index)) / total).filter(|share| *share > 0.0).map(|share| share * share.ln()).sum::<f64>()
}
//#endregion 🔖️Entropy

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
