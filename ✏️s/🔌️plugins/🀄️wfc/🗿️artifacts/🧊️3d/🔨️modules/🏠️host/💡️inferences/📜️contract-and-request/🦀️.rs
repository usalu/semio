//! 📜️ Contract and request of the WFC 3D inference: the published ids and schemas, the request, the stage vocabulary and the job state. See [`super`].

use super::*;

pub const WFC3D_INFERENCE_JOB_KIND: &str = "semio.infer";

pub const WFC3D_INFERENCE_TOOL_ID: &str = "s.wfc.wfc3d.solve";

pub const WFC3D_INFERENCE_PAYLOAD_SCHEMA: &str = "s.wfc.wfc3d.inference.request.v1";

/// 📜️ The PUBLISHED request schema of `s.wfc.wfc3d.solve` — what a client has to send, readable
/// from `inference_list`/`capabilities_describe` without reading a line of this crate. Authored
/// here rather than as a facet leaf because the facet leaf beside it (`🔣️.json`) is the RESULT
/// schema; a request and its result are two schemas, and publishing only one was the gap
/// (`📓️ce3-four-mcp-gates-green.md` §3.3).
pub const WFC3D_INFERENCE_REQUEST_SCHEMA: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://json.schemas.assets.semio-tech.com/s/wfc/wfc3d/1/any/inference.request.json",
  "title": "Wfc3dInferenceRequest",
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
    "snapshot": { "type": "object", "description": "The problem stated in full instead of read from an artifact: the wfc3d solve's own document shape." },
    "checkpoint": { "type": "array", "description": "A previous run's checkpoint bytes, to resume instead of restart.", "items": { "type": "integer", "minimum": 0, "maximum": 255 } }
  }
}"#;

/// 📜️ The whole published contract for `s.wfc.wfc3d.solve`: request schema, result schema, the
/// unit its bounded job counts, and the artifact binding that makes it callable at all.
pub const WFC3D_INFERENCE_CONTRACT: semio_framework_plugin::ArtifactInferencePayloadContract = semio_framework_plugin::ArtifactInferencePayloadContract {
    payload_schema_id: WFC3D_INFERENCE_PAYLOAD_SCHEMA,
    input_schema: WFC3D_INFERENCE_REQUEST_SCHEMA,
    output_schema: include_str!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json"),
    progress_unit: "slots",
    artifact_binding: Some(semio_framework_plugin::ArtifactInferenceDocumentBinding { field: "document", encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64, required: true }),
    commit: None,
};

/// 🧭️ Stable host roster identity for the ActionBus-owned cold solve route.
pub const fn wfc3d_inference_metadata() -> semio_framework_plugin::ArtifactInferenceServiceMetadata {
    semio_framework_plugin::ArtifactInferenceServiceMetadata {
        owner: "wfc",
        artifact_kind: "s.wfc.wfc3d",
        artifact_schema: "s.wfc.wfc3d",
        artifact_schema_version: 1,
        inference_schema: WFC3D_INFERENCE_TOOL_ID,
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        payload: Some(WFC3D_INFERENCE_CONTRACT),
    }
}

#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
pub struct Wfc3dInferenceRequest {
    /// 📸️ The problem, stated in full by the caller. Mutually exclusive with `document`: exactly one
    /// of the two says which snapshot this solve runs over.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Wfc3dSnapshot>,
    /// 🔗️ The ARTIFACT this solve runs on, as its own canonical `pack`/`spr` pair. This is the
    /// field `WFC3D_INFERENCE_CONTRACT`'s artifact binding names, so an agent that calls
    /// `inference_run` with `artifactId` never has to state a snapshot it could not type: the
    /// gateway binds the document here and the guest decodes it into its own snapshot below.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<semio_framework_plugin::ArtifactDocumentPayload>,
    pub checkpoint: Option<Vec<u8>>,
}

impl Wfc3dInferenceRequest {
    /// 📸️ The snapshot this request states, from whichever of its two carriers is present — the
    /// ONE resolution path every caller of this inference takes.
    pub fn resolve_snapshot(&self) -> Result<Wfc3dSnapshot, String> {
        match (&self.snapshot, &self.document) {
            (Some(snapshot), _) => Ok(snapshot.clone()),
            (None, Some(document)) => document.settled_snapshot::<Wfc3dSnapshot, crate::Wfc3dMutation>(),
            (None, None) => Err("s.wfc.wfc3d-inference-no-snapshot:state `snapshot`, or name the artifact with `artifactId` so `document` is bound".into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum Wfc3dInferenceStage {
    Tiles,
    Relations,
    Rules,
    Model,
    Compile,
    Slots,
    Edges,
    Topology,
    Fixed,
    Restore,
    Solve,
    MapCommit,
    EncodeCommit,
    Complete,
}

/// 🧵 Worker-owned parent transaction for wfc3d compile, solve, restore and authoritative mapping.
///
/// 🚪️ Two lifecycle laws the `Solve`/`Restore` arms of [`Wfc3dInferenceJob::step`] keep, both measured
/// rather than assumed. A `semio_framework_job::CommitCandidate` carries TWO retained payloads
/// (`state` and `output`): the child's checkpoint becomes THIS job's commit state, its commit output
/// is the raw assignment vector `map_one` reads from `take_completed_commit()` instead, and whichever
/// is not kept is retired through the payload close ladder — a dropped payload with pages outstanding trips
/// `RetainedJobPayload`'s own Drop assertion and aborts the process. And a child `WfcJob`/`WfcRestore`
/// DROPPED without `engine::job::close_job` leaves its admitted pages registered in the session's
/// payload ledger, after which the session's own close reports `Blocked` forever: that is why the
/// ancestor `assembly` artifact could never run its solve inference in a test at all.
///
/// ⛓️ The `Rules` stage lowers each authored rule to `(allowed, relation or None = all, a, b)`; a rule
/// naming a relation this document never authored binds to no arc, so it is kept in the document and
/// simply skipped. The `Model` stage then replays ONE lowered rule per fuel unit onto the EMPTY
/// builder — `allow` for an admitted pair, `deny` for a forbidden one (deny wins at compile), both
/// directed orders because a rule is written for an unordered pair, and every relation when the rule
/// names none.
pub struct Wfc3dInferenceJob {
    pub(crate) operation: semio_framework_job::Operation,
    pub(crate) snapshot: Wfc3dSnapshot,
    pub(crate) stage: Wfc3dInferenceStage,
    pub(crate) cursor: usize,
    pub(crate) pattern_of: BTreeMap<String, engine::ids::PatternId>,
    pub(crate) node_of: BTreeMap<String, engine::ids::NodeId>,
    pub(crate) tile_ids: Vec<String>,
    pub(crate) raw_weights: Vec<f64>,
    pub(crate) relation_ids: BTreeMap<String, engine::ids::RelationId>,
    pub(crate) relation_names: Vec<String>,
    /// ⛓️ Every authored rule, lowered to `(allowed, relation or None = all, tile a, tile b)` — the
    /// `Model` stage replays one of these per fuel unit onto the builder.
    pub(crate) compatibility: Vec<(bool, Option<usize>, u32, u32)>,
    pub(crate) builder: Option<engine::model::ModelBuilder>,
    pub(crate) model: Option<engine::model::CompiledModel>,
    pub(crate) topology_build: Option<engine::topology::GraphTopologyBuild>,
    pub(crate) topology: Option<engine::topology::GraphTopology>,
    pub(crate) fixed: Vec<(engine::ids::NodeId, engine::ids::PatternId)>,
    pub(crate) checkpoint: Option<Vec<u8>>,
    pub(crate) restore: Option<engine::job::WfcRestore<engine::topology::GraphTopology>>,
    pub(crate) child: Option<engine::job::WfcJob<engine::topology::GraphTopology>>,
    pub(crate) child_commit: Option<engine::job::WfcCommit>,
    pub(crate) assignments: BTreeMap<String, String>,
    pub(crate) output: Option<semio_framework_job_payload::RetainedJobPayloadWriter>,
    pub(crate) rejected_output_page: Option<semio_framework_job_payload::JobPayloadPageSource>,
    pub(crate) output_started: bool,
    pub(crate) encoded_entries: usize,
    pub(crate) encode_total: usize,
    pub(crate) preview_units: u64,
    pub(crate) last_preview_ms: Option<u64>,
    pub(crate) publication: Option<Box<engine::job::Publication>>,
    pub(crate) lent: Wfc3dInferenceJobLent,
    pub(crate) restore_done: bool,
    pub(crate) child_end: Option<InferenceChildEnd>,
    pub(crate) output_payload: Option<semio_framework_job_payload::RetainedJobPayload>,
}
