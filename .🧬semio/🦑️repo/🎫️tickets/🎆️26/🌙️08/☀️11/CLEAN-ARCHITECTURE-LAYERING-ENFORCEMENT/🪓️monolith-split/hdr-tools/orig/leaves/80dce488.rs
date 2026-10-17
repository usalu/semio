//! 🧬️ Sequence artifact schema — every field of the artifact with its state class.

use crate::{SequenceContentChild, SequenceMutation, SequenceSnapshot, SEQUENCE_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;


//#region 🔖️Artifact
/// 🧬️ sequence document artifact state. Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): `steps`/`edges` are replaced
/// by the same composed `content` CHILD slot `SequenceSnapshot` carries, mirroring `WriterArtifact`/
/// `FlowArtifact`'s precedent so `to_snapshot`/`from_snapshot`/`set_snapshot` stay consistent.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: SequenceContentChild,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SequenceArtifact {
    fn default() -> Self {
        Self { schema: SEQUENCE_DOCUMENT_SCHEMA.into(), content: crate::sequence_content_child_with_owner(Vec::new(), Vec::new()) }
    }
}

impl SequenceArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> SequenceSnapshot {
        SequenceSnapshot { schema: self.schema.clone(), content: self.content.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: SequenceSnapshot) -> Self {
        Self { schema: snapshot.schema, content: snapshot.content }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: SequenceSnapshot) {
        self.schema = snapshot.schema;
        self.content = snapshot.content;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.sequence.sequence` — twenty handcrafted schema leaves.
pub fn sequence_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.sequence.sequence",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️Example

//#endregion 🔖️Example

//#region 🏗️Construction
/// 🏗️ W1-C's generic `SnapshotBuilder<Snapshot, Mutation>` (design.md §5 step 3) — replaces the
/// deleted `derive_artifact_facets!`-generated `SequenceBuilder`/`SequenceAnalyzer`/
/// `SequenceComposer` cluster outright: construction is a plain snapshot+mutation build (no custom
/// analysis/composition logic this subset needs beyond the ordinary `Mutation`/`MutationDiff`
/// algebra), so the trivial-subset shape applies verbatim.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<SequenceSnapshot, SequenceMutation>;
//#endregion 🏗️Construction

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::SequenceCamera;
//#endregion 🔁️Re-exports

//#region 🧩️ArtifactItems
pub mod artifact_items {
pub const SEQUENCE_DOCUMENT_SCHEMA: &str = "sequence.sequence";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use neural_engine::{Dictionary, Value};
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use std::collections::HashMap;
pub use crate::snapshot::schema::{default_host_snapshot, SequenceSnapshot};

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
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(transparent, retire_with="std::mem::drop")]
pub struct StepParams(pub Dictionary);

impl semio_framework_value::retirement::RetireOwned for StepParams {
    fn controlled_retirement_supported() -> bool { <Dictionary as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported() }
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let owner = std::mem::ManuallyDrop::new(self);
        let dictionary = unsafe { std::ptr::read(&owner.0) };
        semio_framework_value::retirement::RetireOwned::retirement(dictionary)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0)
    }
}

/// 🧊️ The same fail-closed boundary `imperative_engine::Step` declares for its own `params`
/// (`✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️.rs:53`): a `StepParams` is cloned into host snapshots,
/// working scenes, mutation payloads, edit history and editor transients, and is dropped by the
/// framework wherever a snapshot dies — there is no single call site that could own that
/// retirement, and in the wasm guest a missed one is an abort rather than a test failure. Measured
/// by the play session on 2026-09-22: five `final Dictionary ownership must be explicitly retired
/// or owned by a cold boundary` panics, every one of them with `drop_glue::<StepParams>` in the
/// same frame, one of them reached through an `Arc<dyn Any>` inside the framework's own child-owner
/// machinery where no call site exists to fix. Because the type now has a `Drop`, `.0` can no
/// longer be moved out of an owned value — `std::mem::take(&mut …)` it instead.
impl Drop for StepParams {
    fn drop(&mut self) {
        neural_engine::ColdRetire::retire_cold(std::mem::take(&mut self.0));
    }
}

/// 🧊️ Kept so `SequenceStep`/`Vec<SequenceStep>` still satisfy `ColdRetire`; the work itself is the
/// `Drop` boundary above, so consuming the value IS the retirement.
impl neural_engine::ColdRetire for StepParams {
    fn retire_cold(self) {
        drop(self);
    }
}
impl neural_engine::ColdRetire for SequenceStep {
    fn retire_cold(self) {
        self.params.retire_cold();
    }
}
impl neural_engine::ColdRetire for SequenceWorkingScene {
    fn retire_cold(self) {
        self.steps.retire_cold();
    }
}

impl StepParams {
    pub fn new() -> Self {
        Self(Dictionary::new())
    }

    pub fn insert(mut self, key: impl Into<String>, value: Value) -> Self {
        Self(std::mem::take(&mut self.0).insert(key, value))
    }
}

impl std::ops::Deref for StepParams {
    type Target = Dictionary;
    fn deref(&self) -> &Dictionary {
        &self.0
    }
}

impl semio_framework_dsl_record::BorrowedDslField for StepParams {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Text;
}

impl semio_framework_dsl_record::DslField for StepParams {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Text
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Text(semio_framework_pack_json::to_json_string(&self.0))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Text(text) => semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map(Self).map_err(|err| err.to_string()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🎥️ Camera state for the sequence canvas — the DAG kernel's own `DagCamera` conversions
/// live in the sibling editor module (see its `🔖️Camera` region), not here: `dag`'s `From`/`Into` impls
/// would require this file to depend on the DAG layout kernel just to move a camera in and out,
/// which would pull graph-layout machinery into the plain entity component for no reason a data
/// schema needs — an artifact must never depend on an app either way.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SlotRef {
    #[dsl(refs = "step")]
    pub owner: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct SequenceStep {
    #[dsl(defines = "step")]
    pub id: String,
    pub kind: String,
    #[value(default)]
    pub params: StepParams,
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
    #[value(default)]
    #[dsl(block)]
    pub slot: Option<SlotRef>,
    #[value(default)]
    pub collapsed: bool,
}

/// 🔌️ Runtime edge shape (id/from/to step ids) — kept plain `ToValue`/`FromValue` only; the
/// `.sequence` DSL text and op-log representations go through the `SequenceEdgeDsl` mirror (see
/// `🗣️dsl`) instead of deriving `dsl::DslRecord` here directly, so this struct (and every consumer
/// matching on `.from`/`.to` — `connect_steps`, `sync_edges_from_dag`, ...) stays untouched by the
/// unified `dsl::Wire` connection syntax.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SequenceEdge {
    pub id: String,
    pub from: String,
    pub to: String,
}
//#endregion 🔖️Domain

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.flow` document — ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): the old inline
/// `steps: Vec<SequenceStep>` / `edges: Vec<SequenceEdge>` snapshot fields are replaced by this
/// composed child slot — this plugin no longer defines its own node-graph content model, it
/// composes stdio's `flow` subset instead.
pub type SequenceContentChild = store::os_store::artifact_child::ArtifactChild<SemioFlowSnapshot>;

/// 🎛️ Every `SequenceStep` field flattened into id-ordered string key/value `FlowParam`s —
/// structured sub-values (`params: StepParams(Dictionary)`, the optional `slot`) are JSON-encoded
/// into the string value, the same "honest string boundary" `SemioFlowSnapshot`'s own doc comment
/// describes for a generic flow DAG's per-node config. Every `SequenceStep` field is covered — a
/// real lossless mapping, not a stub.
pub fn sequence_step_params(step: &SequenceStep) -> Vec<SemioFlowParam> {
    fn p(key: &str, value: String) -> SemioFlowParam {
        SemioFlowParam { key: key.into(), value }
    }
    vec![p("params", semio_framework_pack_json::to_json_string(&step.params.0)), p("slot", semio_framework_pack_json::to_json_string(&step.slot)), p("collapsed", step.collapsed.to_string())]
}

/// 🌉 Inverse of [`sequence_step_params`] — reconstructs a `SequenceStep` from a `FlowNode`'s `id`/
/// `kind`/`position` plus its flattened params.
pub fn sequence_step_from_node(node: &SemioFlowNode) -> SequenceStep {
    let params: HashMap<&str, &str> = node.params.iter().map(|param| (param.key.as_str(), param.value.as_str())).collect();
    let get = |key: &str| params.get(key).copied().unwrap_or_default();
    SequenceStep {
        id: node.id.clone(),
        kind: node.kind.clone(),
        params: StepParams(semio_framework_pack_json::from_json_str(get("params"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default()),
        x: node.position.x,
        y: node.position.y,
        slot: semio_framework_pack_json::from_json_str::<Option<SlotRef>>(get("slot"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(None),
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
/// mirrors flow's `flow_content_child_handle`. The digest is the handle's `artifact_id` as much as
/// its `child_id`: `ChildRestoreProjection::child` refuses any member whose two ids differ
/// (`InvalidReference`), which aborts the guest inside `ArtifactApp::genesis_child_pack` before the
/// first window renders.
pub fn sequence_content_child_handle(steps: &[SequenceStep], edges: &[SequenceEdge]) -> SequenceContentChild {
    let snapshot = sequence_content_snapshot_from_working(steps, edges);
    let content_json = semio_framework_pack_json::to_json_string(&snapshot);
    let child_id = store::content_id("sequence-content", content_json.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral typed representation owned by one exact composed content child. It is never
/// persisted, never process-global, never keyed through an identity map, and retires with the child
/// handle. A wire-only handle has no local owner until the host materializes the child document.
#[derive(Clone, Debug, Default)]
pub struct SequenceWorkingScene {
    pub steps: Vec<SequenceStep>,
    pub edges: Vec<SequenceEdge>,
}

/// 🔎 Reads the exact child's typed live scene. A wire-only child remains explicitly unresolved.
pub fn sequence_working_scene_for_handle(handle: &SequenceContentChild) -> Option<SequenceWorkingScene> {
    handle.local_owner::<SequenceWorkingScene>().map(|scene| scene.as_ref().clone())
}

/// 📤️ Requires the exact scene at an export or another content-reading boundary.
pub fn require_sequence_working_scene(handle: &SequenceContentChild) -> Result<SequenceWorkingScene, store::os_store::artifact_child::ArtifactChildMaterializationError> {
    handle.require_local_owner::<SequenceWorkingScene>().map(|scene| scene.as_ref().clone())
}

/// 🌱️ The scene a document's `content` child derives without a member store — the bundled genesis document's. A decoded,
/// reloaded or remote parent carries no scene of its own (design §20.15), so readers without a child view (the `topology`
/// inference) read this and answer the empty scene for any other child, never a materialization fault.
pub fn sequence_derivable_scene(snapshot: &SequenceSnapshot) -> Option<neural_engine::ColdOwner<SequenceWorkingScene>> {
    let genesis = neural_engine::ColdOwner::new(default_host_snapshot());
    (sequence_content_child_handle(&genesis.steps, &genesis.edges).child_id == snapshot.content.child_id).then(|| neural_engine::ColdOwner::new(SequenceWorkingScene { steps: genesis.steps.clone(), edges: genesis.edges.clone() }))
}

/// 🏗️ Mints one content-addressed child and transfers its immutable working scene into that
/// exact local owner. An equal child id in another snapshot cannot observe this payload.
pub fn sequence_content_child_with_owner(steps: Vec<SequenceStep>, edges: Vec<SequenceEdge>) -> SequenceContentChild {
    let handle = sequence_content_child_handle(&steps, &edges);
    handle.with_local_owner(std::sync::Arc::new(SequenceWorkingScene { steps, edges }))
}

/// 🌱️ Packs the `content` member this document derives, for the two hosts that ship a parent
/// envelope with no members at all: the genesis store and the react shell's `loadDocumentPair`
/// (which sends `members: []`). Without it the member is never opened, `ChildContentView` stays
/// empty and every `sequence_working_scene_from_children` call faults
/// `sequence-content-child-dialect-required` before a single window publishes.
pub fn genesis_sequence_child_pack(document: &SequenceSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    if slot != "content" || child_id != document.content.child_id {
        return None;
    }
    let genesis = sequence_derivable_scene(document)?;
    Some(<SemioFlowSnapshot as ArtifactPack>::encode_pack(&sequence_content_snapshot_from_working(&genesis.steps, &genesis.edges)))
}

//#endregion 🔖️WorkingScene
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.sequence.sequence")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.sequence.sequence")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.sequence.sequence")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.sequence.sequence.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.sequence.sequence.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.sequence.sequence@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.sequence.sequence@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.composer.csv")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.csv@rfc4180/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.csv@rfc4180/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"sequence.sequence:sequence")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "sequence.sequence")?)?
                .claim(ArtifactIdentityClaim::codec_extension("sequence.sequence", "sequence")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Sequence")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Sequence")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Sequenz")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Sequenz")?)?,
        )
}

//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems5
