//! 🎬️ Sequence artifact — the document entity this plugin's app edits (constitutional: general).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;
extern crate infinite_canvas as infinite_board_port_directed_dag;

use neural_engine::{Dictionary, Value};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{
    FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,
};
use std::collections::HashMap;

pub use crate::schema::mutations::SequenceMutation;

pub use crate::schema::diff::SequenceDiff;

pub const SEQUENCE_DOCUMENT_SCHEMA: &str = "sequence.sequence";
pub use crate::snapshot::schema::{default_snapshot, SequenceFixture, SequenceSnapshot};

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
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(transparent)]
pub struct StepParams(pub Dictionary);

impl neural_engine::ColdRetire for StepParams { fn retire_cold(self) { self.0.retire_cold(); } }
impl neural_engine::ColdRetire for SequenceStep { fn retire_cold(self) { self.params.retire_cold(); } }
impl neural_engine::ColdRetire for SequenceWorkingScene { fn retire_cold(self) { self.steps.retire_cold(); } }

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
        dsl::FieldValue::Text(dsl::os_pack::json::to_json_string(&self.0))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(text) => dsl::os_pack::json::from_json_str(text).map(Self).map_err(|err| err.to_string()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🎥️ Camera state for the sequence canvas — the DAG kernel's own `DagCamera` conversions
/// live in the sibling editor module (see its `🔖️Camera` region), not here: `dag`'s `From`/`Into` impls
/// would require this file to depend on the DAG layout kernel just to move a camera in and out,
/// which would pull graph-layout machinery into the plain entity component for no reason a data
/// schema needs — an artifact must never depend on an app either way.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SlotRef {
    #[dsl(refs = "step")]
    pub owner: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
    vec![p("params", dsl::os_pack::json::to_json_string(&step.params.0)), p("slot", dsl::os_pack::json::to_json_string(&step.slot)), p("collapsed", step.collapsed.to_string())]
}

/// 🌉 Inverse of [`sequence_step_params`] — reconstructs a `SequenceStep` from a `FlowNode`'s `id`/
/// `kind`/`position` plus its flattened params.
fn sequence_step_from_node(node: &SemioFlowNode) -> SequenceStep {
    let params: HashMap<&str, &str> = node.params.iter().map(|param| (param.key.as_str(), param.value.as_str())).collect();
    let get = |key: &str| params.get(key).copied().unwrap_or_default();
    SequenceStep {
        id: node.id.clone(),
        kind: node.kind.clone(),
        params: StepParams(dsl::os_pack::json::from_json_str(get("params")).unwrap_or_default()),
        x: node.position.x,
        y: node.position.y,
        slot: dsl::os_pack::json::from_json_str::<Option<SlotRef>>(get("slot")).unwrap_or(None),
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
    let content_json = dsl::os_pack::json::to_json_string(&snapshot);
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
pub fn require_sequence_working_scene(handle: &SequenceContentChild) -> Result<SequenceWorkingScene, store::ArtifactChildMaterializationError> {
    handle.require_local_owner::<SequenceWorkingScene>().map(|scene| scene.as_ref().clone())
}

/// 🔎 Reads the current document's live steps/edges off its `content` child handle — the single
/// read call site every mutation diff/inverse and app-layer host in this plugin uses instead of the
/// old `snapshot.steps`/`.edges` field access.
pub fn sequence_working_scene(snapshot: &SequenceSnapshot) -> SequenceWorkingScene {
    sequence_working_scene_for_handle(&snapshot.content).expect("sequence child scene must be materialized before use")
}

/// 🏗️ Mints one content-addressed child and transfers its immutable working scene into that
/// exact local owner. An equal child id in another snapshot cannot observe this payload.
pub fn sequence_content_child_with_owner(steps: Vec<SequenceStep>, edges: Vec<SequenceEdge>) -> SequenceContentChild {
    let handle = sequence_content_child_handle(&steps, &edges);
    handle.with_local_owner(std::sync::Arc::new(SequenceWorkingScene { steps, edges }))
}

/// 🔺️ Shared diff builder every mutation triad's `🔺️diff` leaf calls after computing its own new
/// steps/edges against the working scene — mints+caches a whole new content handle (the
/// "mint+cache whole handle, never apply-then-capture" pattern flow's `diff_replace_content`/
/// writer's `diff_set_text` established), never a structured steps/edges delta (the composed child
/// is opaque — a parent's diff never embeds a child diff, `📓️design-full-plan.md` §1's CHILD/LINK
/// split).
pub fn diff_replace_content(steps: Vec<SequenceStep>, edges: Vec<SequenceEdge>) -> SequenceDiff {
    SequenceDiff { content: Some(sequence_content_child_with_owner(steps, edges)), ..Default::default() }
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
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into()],
        import_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
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
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Sequence")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Sequence")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.sequence.sequence.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Sequenz")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Sequenz")?)?)
}

//#endregion 🔖️Declaration

//#region 🔖️ArtifactDeclaration
/// 🌳️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): the whole
/// `s.sequence.sequence` artifact through the declaration tree — one standard (`1`), one subset
/// (`any`). Replaces the OLD `declaration()`/`ArtifactDeclaration::builder(...)` channel outright
/// (atomic cutover; both channels never coexist). `localization: &[]` is a documented shortfall,
/// not an oversight — the real en/de localized descriptors still live on `definition()`'s
/// `ArtifactCapability` rows above (kept per debt D1, deleted repo-wide only in W6); wiring them
/// into this field too is real follow-up work, not required for the tree to register or for any
/// law to hold (mirrors the stdio pilot's own documented deviation, `📓️w2-p-report.md`).
pub fn artifact<A: SequenceApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.sequence.sequence").expect("canonical sequence.sequence kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait SequenceApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::sequence::SequencePlayApp>>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::sequence::SequenceViewer>>>
{
}

impl<A> SequenceApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::sequence::SequencePlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::sequence::SequenceViewer>>>
{
}

//#endregion 🔖️ArtifactDeclaration

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod step {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod create_step {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🌱️create-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🌱️create-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🌱️create-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🌱️create-step/🧪️tests/🚫️rejects-a-547ea4/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_step_id;
                                }
                                #[path = "."]
                                pub mod delete_step {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🗑️delete-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🗑️delete-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🗑️delete-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/🚫️rejects-deleting-1e4599/🦀️.rs"]
                                    mod tests_rejects_deleting_a_missing_step;
                                }
                                #[path = "."]
                                pub mod move_step {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/📍️move-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/📍️move-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/📍️move-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/📍️move-step/🧪️tests/🟰️no-ops-when-the-b925f0/🦀️.rs"]
                                    mod tests_no_ops_when_the_step_is_already_at_that_position;
                                }
                                #[path = "."]
                                pub mod edit_step_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🔧️edit-step-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🔧️edit-step-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🔧️edit-step-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🔧️edit-step-params/🧪️tests/🟰️no-ops-when-the-70413d/🦀️.rs"]
                                    mod tests_no_ops_when_the_params_are_already_identical;
                                }
                                #[path = "."]
                                pub mod change_step_collapsed {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🗂️change-step-collapsed/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🗂️change-step-collapsed/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🗂️change-step-collapsed/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🗂️change-step-collapsed/🧪️tests/🟰️no-ops-when-the-bd4f9d/🦀️.rs"]
                                    mod tests_no_ops_when_the_step_is_already_collapsed;
                                }
                                #[path = "."]
                                pub mod duplicate_step {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🧬️duplicate-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🧬️duplicate-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🧬️duplicate-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🧬️duplicate-step/🧪️tests/🚫️rejects-when-the-48fafc/🦀️.rs"]
                                    mod tests_rejects_when_the_new_id_already_exists;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod dependency {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod connect_steps {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔗️dependency/🧬️schema/🧬️mutations/🔗️connect-steps/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔗️dependency/🧬️schema/🧬️mutations/🔗️connect-steps/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔗️dependency/🧬️schema/🧬️mutations/🔗️connect-steps/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔗️dependency/🧬️schema/🧬️mutations/🔗️connect-steps/🧪️tests/🚫️rejects-connectin-17a073/🦀️.rs"]
                                    mod tests_rejects_connecting_a_step_to_itself;
                                }
                                #[path = "."]
                                pub mod disconnect_steps {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔗️dependency/🧬️schema/🧬️mutations/✂️disconnect-steps/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔗️dependency/🧬️schema/🧬️mutations/✂️disconnect-steps/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔗️dependency/🧬️schema/🧬️mutations/✂️disconnect-steps/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔗️dependency/🧬️schema/🧬️mutations/✂️disconnect-steps/🧪️tests/🚫️rejects-disconnec-c7496b/🦀️.rs"]
                                    mod tests_rejects_disconnecting_a_missing_edge;
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::io::mutations::text::*;
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sequence {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[cfg(target_arch = "wasm32")]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️connection/🦀️.rs"]
            pub mod connection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️layout/🦀️.rs"]
            pub mod layout;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph/🦀️.rs"]
            pub mod node_graph;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏃️run/🦀️.rs"]
            pub mod playback;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️step/🦀️.rs"]
            pub mod step;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️compiled/🦀️.rs"]
                    pub mod compiled;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🦀️.rs"]
                    pub mod main;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📜️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod sequence {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📽️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}

//#region 📚️Examples
pub use standards::v1::subsets::any::examples;
//#endregion 📚️Examples
