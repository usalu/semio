//! 🎬️ Sequence artifact — the document entity this plugin's app edits (constitutional: general).

use neural_engine::{Dictionary, Value};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{
    FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

pub use crate::artifacts::sequence::schema::mutations::SequenceMutation;

pub use crate::artifacts::sequence::schema::diff::SequenceDiff;

pub const SEQUENCE_DOCUMENT_SCHEMA: &str = "sequence.sequence";
pub use crate::artifacts::sequence::snapshot::schema::{default_snapshot, SequenceFixture, SequenceSnapshot};

//#region 🔖️Constants
/// 🪪️ The canonical dialect for this artifact's one subset (`✳️any`) — lives at the ARTIFACT level
/// (not under `editor`/`viewer`) specifically so the sibling `viewer` module can read it without ever
/// importing through the `editor` module (contract §1/§7.4). `artifact_kind` matches this schema's own
/// `#[artifact_schema(id = "s.sequence.sequence")]` / `definition()`'s `s.sequence.schema.artifact`
/// capability row; `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any`
/// location — i.e. the canonical surface id is `s.sequence.sequence@1/*#editor` /
/// `s.sequence.sequence@1/*#viewer`, the contract §1 grammar.
pub const SEQUENCE_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.sequence.sequence", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };
//#endregion 🔖️Constants

//#region 🔖️Domain
/// 📦️ Local newtype around {@link neural_engine::Dictionary} — dynamic/schema-less step params
/// can't be shape-derived field-by-field (arbitrary keys, recursive `Value`), and `Dictionary`
/// itself can't gain a `dsl::DslField` impl directly (foreign trait, foreign type, no local anchor
/// for the orphan rule). Wrapping it as one opaque JSON-text field reuses the exact `serde_json`
/// round trip {@link SequenceHost::to_json}/{@link SequenceHost::load_json} already depend on for
/// fidelity — unlike a schema-less `dsl::Shape::Value`, this never collapses `Atom::Integer` and
/// `Atom::Decimal` into the same wire number. Deliberately `dsl::Shape::Text` (escaped quoted
/// string), NOT `dsl::Shape::Embed("json")` (fenced block): this field is only ever reached as a
/// `#[dsl(table)]` column (`SequenceStep` is `SequenceSnapshotDsl.steps`'s row type), and an
/// `Embed`'s Document-mode fence needs its closing ` ``` ` on its own line — the table row printer
/// glues the remaining row cells (`x y slot collapsed`) onto that same line right after it,
/// producing a fence the lexer can't close and a confirmed parse failure ("unterminated fenced
/// block"). Genuine ENGINE GAP (`Shape::Embed` inside a `Shape::Table` column), out of scope here —
/// verified empirically, not worked around.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StepParams(pub Dictionary);

impl StepParams {
    pub fn new() -> Self {
        Self(Dictionary::new())
    }

    pub fn insert(self, key: impl Into<String>, value: Value) -> Self {
        Self(self.0.insert(key, value))
    }
}

impl std::ops::Deref for StepParams {
    type Target = Dictionary;
    fn deref(&self) -> &Dictionary {
        &self.0
    }
}

impl dsl::DslField for StepParams {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(serde_json::to_string(&self.0).unwrap_or_else(|_| "{}".into()))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(text) => serde_json::from_str(text).map(Self).map_err(|err| err.to_string()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🎥️ Camera state for the sequence canvas — the DAG kernel's own `DagCamera` conversions
/// live in the sibling editor module (see its `🔖️Camera` region), not here: `dag`'s `From`/`Into` impls
/// would require this file to depend on the DAG layout kernel just to move a camera in and out,
/// which would pull graph-layout machinery into the plain entity component for no reason a data
/// schema needs — an artifact must never depend on an app either way.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for SequenceCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🎯️ Only ever embedded `#[dsl(block)]`-wrapped (on `SequenceStep::slot`), so it carries no
/// `#[dsl(keyword = "...")]` of its own — the embedding field already supplies the bare `slot`
/// leading keyword.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SlotRef {
    #[dsl(refs = "step")]
    pub owner: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SequenceStep {
    #[dsl(defines = "step")]
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub params: StepParams,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    #[dsl(block)]
    pub slot: Option<SlotRef>,
    #[serde(default)]
    pub collapsed: bool,
}

/// 🔌️ Runtime edge shape (id/from/to step ids) — kept plain `Serialize`/`Deserialize` only; the
/// `.sequence` DSL text and op-log representations go through the `SequenceEdgeDsl` mirror (see
/// `🗣️dsl`) instead of deriving `dsl::DslRecord` here directly, so this struct (and every consumer
/// matching on `.from`/`.to` — `connect_steps`, `sync_edges_from_dag`, ...) stays untouched by the
/// unified `dsl::Wire` connection syntax.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceEdge {
    pub id: String,
    pub from: String,
    pub to: String,
}
//#endregion 🔖️Domain

//#region 🔖️Collections
impl protocol::Identified<String> for SequenceStep {
    fn id(&self) -> &String {
        &self.id
    }
}

impl protocol::Identified<String> for SequenceEdge {
    fn id(&self) -> &String {
        &self.id
    }
}

//#endregion 🔖️Collections

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.flow` document — ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): the old inline
/// `steps: Vec<SequenceStep>` / `edges: Vec<SequenceEdge>` snapshot fields are replaced by this
/// composed child slot — this plugin no longer defines its own node-graph content model, it
/// composes stdio's `flow` subset instead.
pub type SequenceContentChild = store::ArtifactChild<SemioFlowSnapshot>;

/// 🎛️ Every `SequenceStep` field flattened into id-ordered string key/value `FlowParam`s —
/// structured sub-values (`params: StepParams(Dictionary)`, the optional `slot`) are JSON-encoded
/// into the string value, the same "honest string boundary" `SemioFlowSnapshot`'s own doc comment
/// describes for a generic flow DAG's per-node config. Every `SequenceStep` field is covered — a
/// real lossless mapping, not a stub.
fn sequence_step_params(step: &SequenceStep) -> Vec<SemioFlowParam> {
    fn p(key: &str, value: String) -> SemioFlowParam {
        SemioFlowParam { key: key.into(), value }
    }
    vec![p("params", serde_json::to_string(&step.params.0).unwrap_or_default()), p("slot", serde_json::to_string(&step.slot).unwrap_or_else(|_| "null".into())), p("collapsed", step.collapsed.to_string())]
}

/// 🌉 Inverse of [`sequence_step_params`] — reconstructs a `SequenceStep` from a `FlowNode`'s `id`/
/// `kind`/`position` plus its flattened params.
fn sequence_step_from_node(node: &SemioFlowNode) -> SequenceStep {
    let params: HashMap<&str, &str> = node.params.iter().map(|param| (param.key.as_str(), param.value.as_str())).collect();
    let get = |key: &str| params.get(key).copied().unwrap_or_default();
    SequenceStep {
        id: node.id.clone(),
        kind: node.kind.clone(),
        params: StepParams(serde_json::from_str(get("params")).unwrap_or_default()),
        x: node.position.x,
        y: node.position.y,
        slot: serde_json::from_str::<Option<SlotRef>>(get("slot")).unwrap_or(None),
        collapsed: get("collapsed").parse().unwrap_or(false),
    }
}

/// 🌉 REAL bidirectional converter between the app's live `SequenceStep`/`SequenceEdge` editing
/// state and the composed child's own `SemioFlowSnapshot` node/edge graph (the "ModelBridge"/
/// "DocumentBridge" pattern from `📓️wave3-reports/cad-report.md`/`📓️wave4-reports/flow-report.md`)
/// — every step field round-trips through [`sequence_step_params`]/[`sequence_step_from_node`];
/// `SequenceEdge` maps onto `FlowEdge` 1:1 through an empty-port `PortRef` (sequence edges are
/// plain step-to-step flow, not port-addressed) — the constant `kind: "sequence"` tag is written on
/// encode and discarded on decode (lossless, since `SequenceEdge` carries no `kind` of its own).
pub fn sequence_content_snapshot_from_working(steps: &[SequenceStep], edges: &[SequenceEdge]) -> SemioFlowSnapshot {
    let nodes = steps.iter().map(|step| SemioFlowNode { id: step.id.clone(), kind: step.kind.clone(), label: step.kind.clone(), params: sequence_step_params(step), position: SemioPoint2 { x: step.x, y: step.y } }).collect();
    let edges = edges.iter().map(|edge| SemioFlowEdge { id: edge.id.clone(), from: SemioPortRef { node: edge.from.clone(), port: String::new() }, to: SemioPortRef { node: edge.to.clone(), port: String::new() }, kind: "sequence".into() }).collect();
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🌉 Inverse of [`sequence_content_snapshot_from_working`].
pub fn working_from_sequence_content_snapshot(content: &SemioFlowSnapshot) -> (Vec<SequenceStep>, Vec<SequenceEdge>) {
    let steps = content.nodes.iter().map(sequence_step_from_node).collect();
    let edges = content.edges.iter().map(|edge| SequenceEdge { id: edge.id.clone(), from: edge.from.node.clone(), to: edge.to.node.clone() }).collect();
    (steps, edges)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the sequence content — same `(child_id,
/// target)` for identical `(steps, edges)`, a different pair once the content actually changes;
/// mirrors flow's `flow_content_child_handle`/writer's `document_child_handle`.
pub fn sequence_content_child_handle(steps: &[SequenceStep], edges: &[SequenceEdge]) -> SequenceContentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = sequence_content_snapshot_from_working(steps, edges);
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("sequence-content-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "sequence-content".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, session-side working representation of the composed content child's live steps/
/// edges — NEVER persisted, NEVER a durable field on `SequenceSnapshot` itself (matches the
/// `EngineRep` contract: wholly derived, droppable at any instant, rebuilt from base). Exists
/// because no `LinkResolver`/child-dispatch seam is wired into `ArtifactApp::handle` yet (checked
/// directly against `🔌️plugin/🦀️component.rs` — same standing gap cad/lowpoly/writer/flow's reports
/// all document); until one exists, the only way a persisted content-addressed HANDLE can round-trip
/// to real steps/edges within one process is this cache, keyed by `SequenceContentChild::child_id`
/// — mirrors `FlowWorkingScene`/`WriterWorkingScene`/`LowpolyScratch.mesh_workspace`.
///
/// ⚠️ Same documented gap as every other exemplar: store-level undo/redo bypasses
/// `ArtifactApp::handle` entirely, and a bare `parse_dsl`/`decode_pack` of persisted bytes recovers
/// only the opaque handle, never the content (the child's real payload lives in its own, not-yet-
/// resolvable, child store). `sequence_working_scene`/`sequence_working_scene_for_handle` fail soft
/// (an empty scene) rather than panicking. A real fix needs child-document resolution, which no
/// WASM-guest plugin in this repo has yet.
#[derive(Clone, Debug, Default)]
pub struct SequenceWorkingScene {
    pub steps: Vec<SequenceStep>,
    pub edges: Vec<SequenceEdge>,
}

thread_local! {
    static SEQUENCE_SCRATCH: RefCell<HashMap<String, SequenceWorkingScene>> = RefCell::new(HashMap::new());
}

/// 📝 Seeds the scratch cache for a handle — call whenever new steps/edges content is about to
/// become a document's `content` field (every mutation-diff/fixture builder in this plugin does,
/// via [`sequence_content_child_handle_and_cache`]).
pub fn cache_sequence_content(child_id: &str, steps: Vec<SequenceStep>, edges: Vec<SequenceEdge>) {
    SEQUENCE_SCRATCH.with(|cache| cache.borrow_mut().insert(child_id.to_string(), SequenceWorkingScene { steps, edges }));
}

/// 🔎 Reads the cached live scene for a content child handle — an empty scene (never a panic) when
/// nothing has cached it yet (see this region's module doc comment for why that can happen).
pub fn sequence_working_scene_for_handle(handle: &SequenceContentChild) -> SequenceWorkingScene {
    SEQUENCE_SCRATCH.with(|cache| cache.borrow().get(&handle.child_id).cloned()).unwrap_or_default()
}

/// 🔎 Reads the current document's live steps/edges off its `content` child handle — the single
/// read call site every mutation diff/inverse and app-layer host in this plugin uses instead of the
/// old `snapshot.steps`/`.edges` field access.
pub fn sequence_working_scene(snapshot: &SequenceSnapshot) -> SequenceWorkingScene {
    sequence_working_scene_for_handle(&snapshot.content)
}

/// 🏗️ Mints a new content-addressed handle AND seeds the scratch cache with its scene in one call —
/// the standard way every mutation-diff/fixture builder in this plugin creates a `content` field
/// value; never construct a handle without also caching, or [`sequence_working_scene`] will read
/// back empty.
pub fn sequence_content_child_handle_and_cache(steps: Vec<SequenceStep>, edges: Vec<SequenceEdge>) -> SequenceContentChild {
    let handle = sequence_content_child_handle(&steps, &edges);
    cache_sequence_content(&handle.child_id, steps, edges);
    handle
}

/// 🔺️ Shared diff builder every mutation triad's `🔺️diff` leaf calls after computing its own new
/// steps/edges against the working scene — mints+caches a whole new content handle (the
/// "mint+cache whole handle, never apply-then-capture" pattern flow's `diff_replace_content`/
/// writer's `diff_set_text` established), never a structured steps/edges delta (the composed child
/// is opaque — a parent's diff never embeds a child diff, `📓️design-full-plan.md` §1's CHILD/LINK
/// split).
pub fn diff_replace_content(steps: Vec<SequenceStep>, edges: Vec<SequenceEdge>) -> SequenceDiff {
    SequenceDiff { content: Some(sequence_content_child_handle_and_cache(steps, edges)), ..Default::default() }
}
//#endregion 🔖️WorkingScene

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::sequence::create_sequence_app`'s `🔖️Manifest` region. Lifted verbatim out of the
/// old `.artifact_kind(...)` builder call.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.sequence".into(),
        name: "Sequence".into(),
        source_format: "sequence.sequence".into(),
        component_kind: "sequence".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Sequence },
        schema: "sequence.sequence".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_snapshot_has_steps() {
        assert_eq!(default_snapshot().to_fixture().steps.len(), 2);
    }

    #[test]
    fn step_content_round_trips_through_the_composed_child_snapshot() {
        let fixture = default_snapshot().to_fixture();
        let content = sequence_content_snapshot_from_working(&fixture.steps, &fixture.edges);
        let (steps, edges) = working_from_sequence_content_snapshot(&content);
        assert_eq!(steps, fixture.steps);
        assert_eq!(edges, fixture.edges);
    }

    #[test]
    fn artifact_kind_keeps_the_media_schema_consistent_with_the_store_schema() {
        assert_eq!(artifact_kind().schema, "sequence.sequence");
        assert_eq!(SEQUENCE_DOCUMENT_SCHEMA, "sequence.sequence");
    }
}
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.sequence")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.sequence.sequence")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.sequence.sequence")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.sequence.sequence.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.sequence.sequence.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.sequence@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.sequence@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.composer.csv")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.csv@rfc4180/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.csv@rfc4180/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"sequence.sequence:sequence")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "sequence.sequence")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "sequence")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Sequence")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Sequence")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Sequenz")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Sequenz")?)?)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::sequence::schema::sequence_artifact_schema_descriptor())
        .inferences([crate::artifacts::sequence::standards::v1::subsets::any::schema::inferences::sequence_artifact_inference_descriptor()])
        .composers(crate::artifacts::sequence::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::sequence::SequencePlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration
