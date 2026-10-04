# Current Graph Product Identity Collision Audit

Read-only actual source/AST audit. Executed existing Tree-sitter Rust parser against full Jack manifest/root source; both ASTs parse without errors. No macro expansion/compiler/runtime run. Do not equate absence of textual call with absent derive-generated consumer.

## Concrete bound collisions

Jack root explicitly aliases semio_framework_os_kernel as dsl, then reexports Graph-owned PropertyDef/PortDirection/PropertyBag/PropertyValue. Its manifest source has NodeKindDef, EdgeKindDef and PortKindDef deriving dsl::DslRecord. All three have Vec<PropertyDef>; PortKindDef also has PortDirection. Root's Edge derives dsl::DslRecord with PropertyBag. These four direct derived containers consume Graph DslField impls through OS trait identity. Manifest derives indirectly compose those containers. Port and Node only derive neutral Value traits and do not themselves create this Record collision.

Full OS DSL derive source emits `<field as ::dsl::DslField>::shape/to_value/from_value`, `::dsl::Shape`, `::dsl::FieldValue` and Record carriers. OS DSL component locally defines DslField and reexports its physically separate OS schema. Graph direct-owner retirement changes PropertyDef/PortDirection/PropertyBag impls to neutral semio_framework_dsl_record::DslField; this does not satisfy the old OS trait bound on those Graph-owned field types. Value trait paths remain neutral and compatible.

Graph's own Manifest is NOT shown exposing a to_record API in current source; Jack's distinct Manifest is derived and owns typed grammar. Avoid assuming every GraphManifest validation/property use requires Record identity migration. OS Infinite normal/directed board uses Graph property containers and GraphManifest validation mostly through owned data/Value APIs; full inspected NodeData struct only derives Clone/Debug. No direct OS Pack Graph PropertyDef/Bag/Value DslField call found in narrowed source census; opaque generated binds remain UNKNOWN. All compiler error/macro-generated unresolved counts UNKNOWN until sole worker compiler evidence.

## Canonical consumer-closed retirement

No adapter, duplicated foreign trait impl, old facade or compatibility export can reconcile this by policy. Graph owns its types and only neutral Record owns the canonical Record trait/carriers. Jack must adopt neutral Record derive/field/spec/carrier names directly, plus required direct Value/Diagnostic crate dependencies emitted by neutral derive. However switching Jack derives alone can propagate neutral RecordValue into OS-owned ArtifactDsl/pack schema consumers. Inspect those actual method/signature/derive bodies and retire OS duplicate generic schema/binding ownership to the single neutral owner before publication if such propagation occurs.

The whole canonical OS DSL retirement cohort is materially larger than Graph's12 source rows: remove OS generic DslField/Shape/RecordValue copies and foreign ValueType/DslValue impls, import exact neutral Record APIs into OS product-specific functionality; migrate OS Record derive implementations/consumers to canonical neutral derive or keep only genuinely product-specific derives emitting direct neutral owner names. Pack codecs/schema native controlled paths must all consume the same canonical RecordSpec/Shape/FieldValue/RecordValue. Product-specific artifact/store operation traits may remain product-owned, while their generic grammar carriers must be neutral imports. Do not reexport a broad OS umbrella solely to satisfy old `::dsl` paths.

Before source publication compile Graph standalone/neutral closure and actual whole OS plus Jack consuming packages through admitted worker receipts. Current root does not include Jack as a member, so whole OS GREEN alone cannot prove its artifact macro consumer closure. Root must register/run explicit retained Jack consumer cohort if it wants compiler assurance. Code text/AST evidence establishes this actual blocker; no proof that all OS Record consumers are enumerated yet.

## AST execution receipt

Tool run `bun -e` used existing web-tree-sitter and tree-sitter-rust.wasm, parsed two current source files, selected real struct_item nodes containing Graph type fields. Results: manifest NodeKindDef/EdgeKindDef/PortKindDef and root Port/Node/Edge; parser errors false for both. Full source below records derive attributes to distinguish actual Record consumers from Value-only structs. No generated files or permanent scripts created.

## Full current source contexts

### ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs

SHA-256 `44f9fada2a0e5dffd4f5563bb67d2abca0c922eb2fc9121f4517f29c9abec895`; 77746 bytes.

```
//! 🔺️ `trinity.graph` artifact — in-memory directed property port graph with compile-time manifest.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`trinity→C:graph`, jack side — see this
//! region's own doc for why this is ONE composed child, not two: the design annotation "jack; rewriting
//! = 2 graph children" attributes the two-child shape to the SEPARATE `rewriting` app's LHS/RHS rule
//! windows, not to jack. jack's own persisted `nodes`/`edges` instance data is replaced by a single
//! composed `s.stdio.semio.graph` CHILD slot (`🔖️ContentBridge` below); the compile-time `manifest`
//! (kind/property/port DEFINITIONS, resolved from `manifestId` — see `semio_framework_graph::manifest::GraphManifest`)
//! is NOT graph-shaped in the `SemioGraphSnapshot` sense (that subset is an INSTANCE graph: nodes with
//! position/ports/properties, edges with source/target) and stays an ordinary inline field, unchanged.

#![allow(clippy::unnecessary_wraps)]

#[path = "🤖️generated/📇️registry/🦀️.rs"]
pub mod graph_manifest;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🦀️.rs"]
pub mod ast;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🦀️.rs"]
pub mod executor;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️.rs"]
pub mod language_service;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔤️lexer/🦀️.rs"]
pub mod lexer;
#[cfg(feature = "component-app-assembly")]
pub use crate::editor::jack::snapshot_to_workflow;
pub use crate::standards::v1::subsets::any::schema::inferences::flat_position::compute_flat_position;
pub use language_service as core;

use semio_framework_graph::manifest::{GraphManifest, ManifestValidationError};
use crate::graph_manifest::manifest_by_id;
use std::collections::{BTreeMap, BTreeSet};

pub use semio_framework_graph::manifest::{ManifestValidator, PortDirection, PropertyBag, PropertyDef, PropertyKind, PropertyValue};

#[path = "🛂️manifest/🦀️.rs"]
pub mod manifest;
pub use manifest::{Manifest, NodeKindDef, EdgeKindDef, PortKindDef};

//#region ⚠️ Errors
/// ⚠️ Trinity graph snapshot, manifest-validation, and mutation errors.
#[derive(Debug)]
pub enum TrinityRamError {
    /// 🧬️ JSON (de)serialization failure.
    Json(String),
    /// 🧭️ VCS store/dispatch failure.
    Vcs(vcs::VcsError),
    /// 🧬️ Persisted mutation diff rejection.
    MutationApply(protocol::MutationApplyError),
    /// 📜️ Compile-time manifest validation failure (path-qualified).
    Manifest(ManifestValidationError),
    SchemaMismatch {
        expected: &'static str,
        actual: String,
    },
    UnknownManifestId(String),
    ManifestMissing,
    NodeNotFound(String),
    EdgeNotFound(String),
    NodeAlreadyExists(String),
    EdgeAlreadyExists(String),
    InvalidSourcePortKey(String),
    InvalidTargetPortKey(String),
    SourceNodeNotFound(String),
    TargetNodeNotFound(String),
    PortKindNotDeclaredOnFixture {
        node_id: String,
        port_kind: String,
        node_kind: String,
    },
    PortKindNotDeclaredOnMutation {
        node_id: String,
        port_id: String,
        port_kind: String,
        node_kind: String,
    },
    UnknownNodeKind {
        kind: String,
    },
    UnknownEdgeKind {
        kind: String,
    },
    UnknownPortKind {
        kind: String,
    },
    UnknownEntityKind {
        path: String,
    },
    UnknownPropertyAtPath {
        path: String,
        key: String,
    },
    PropertyTypeMismatch {
        path: String,
        name: String,
        value_type: String,
    },
    UnknownPropertyInBag {
        path: String,
        key: String,
    },
    /// 📏️ A `set-query` beyond [`JACK_QUERY_MAXIMUM_BYTES`].
    QueryTooLarge {
        bytes: usize,
        maximum: usize,
    },
}

impl std::fmt::Display for TrinityRamError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::Vcs(error) => write!(formatter, "{error}"),
            Self::MutationApply(error) => write!(formatter, "{error}"),
            Self::Manifest(error) => write!(formatter, "{}: {}", error.path, error.message),
            Self::SchemaMismatch { expected, actual } => write!(formatter, "expected schema {expected}, got {actual}"),
            Self::UnknownManifestId(id) => write!(formatter, "unknown manifest id {id}"),
            Self::ManifestMissing => formatter.write_str("snapshot missing manifest or manifestId"),
            Self::NodeNotFound(id) => write!(formatter, "node {id} not found"),
            Self::EdgeNotFound(id) => write!(formatter, "edge {id} not found"),
            Self::NodeAlreadyExists(id) => write!(formatter, "node {id} already exists"),
            Self::EdgeAlreadyExists(id) => write!(formatter, "edge {id} already exists"),
            Self::InvalidSourcePortKey(key) => write!(formatter, "invalid source port key {key}"),
            Self::InvalidTargetPortKey(key) => write!(formatter, "invalid target port key {key}"),
            Self::SourceNodeNotFound(id) => write!(formatter, "source node {id} not found"),
            Self::TargetNodeNotFound(id) => write!(formatter, "target node {id} not found"),
            Self::PortKindNotDeclaredOnFixture { node_id, port_kind, node_kind } => write!(formatter, "nodes/{node_id}/ports/{port_kind}: port kind {port_kind} not declared on node kind {node_kind}"),
            Self::PortKindNotDeclaredOnMutation { node_id, port_id, port_kind, node_kind } => write!(formatter, "nodes/{node_id}/ports/{port_id}: port kind {port_kind} not declared on node kind {node_kind}"),
            Self::UnknownNodeKind { kind } => write!(formatter, "nodes/{kind}: unknown node kind {kind:?}"),
            Self::UnknownEdgeKind { kind } => write!(formatter, "edges/{kind}: unknown edge kind {kind:?}"),
            Self::UnknownPortKind { kind } => write!(formatter, "ports/{kind}: unknown port kind {kind:?}"),
            Self::UnknownEntityKind { path } => write!(formatter, "{path}: unknown kind"),
            Self::UnknownPropertyAtPath { path, key } => write!(formatter, "{path}: unknown property {key:?}"),
            Self::PropertyTypeMismatch { path, name, value_type } => write!(formatter, "{path}/{name}: property type mismatch for {value_type}"),
            Self::UnknownPropertyInBag { path, key } => write!(formatter, "{path}/{key}: unknown property {key:?}"),
            Self::QueryTooLarge { bytes, maximum } => write!(formatter, "query: {bytes} bytes exceed the {maximum}-byte bound"),
        }
    }
}

impl std::error::Error for TrinityRamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Vcs(error) => std::error::Error::source(error),
            Self::MutationApply(error) => std::error::Error::source(error),
            _ => None,
        }
    }
}

impl From<semio_framework_pack_json::JsonError> for TrinityRamError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<dsl::ValueError> for TrinityRamError {
    fn from(error: dsl::ValueError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<vcs::VcsError> for TrinityRamError {
    fn from(error: vcs::VcsError) -> Self {
        Self::Vcs(error)
    }
}

impl From<protocol::MutationApplyError> for TrinityRamError {
    fn from(error: protocol::MutationApplyError) -> Self {
        Self::MutationApply(error)
    }
}

/// 🔀️ [`ManifestValidationError`] carries no `std::error::Error` impl of its own (plain path/message struct), so this conversion remains explicit.
impl From<ManifestValidationError> for TrinityRamError {
    fn from(error: ManifestValidationError) -> Self {
        Self::Manifest(error)
    }
}
//#endregion ⚠️ Errors

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.graph` document — jack's `nodes`/`edges`
/// instance data now lives in this composed child's own `nodes`/`edges`, not on `JackSnapshot`.
pub type JackContentChild = store::ArtifactChild<SemioGraphSnapshot>;

use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{
    GraphEdgeId as SemioGraphEdgeId, GraphNodeId as SemioGraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA,
};
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

/// 🏷️ `jack.node` is the honest string boundary carrying the FULL [`Node`] (id/kind/name/x/y/
/// width/height/properties/ports — every field this plugin's own rich node model can hold, none of
/// which `SemioGraphNode`'s native fields alone can carry: `width`/`height` have no native slot at
/// all, and a port's own `kind`/`properties` don't survive the native `ports` projection below) as
/// JSON. `id`/`kind`/`label`/`position` are ALSO projected onto `SemioGraphNode`'s own native fields,
/// and `ports` is a best-effort projection (`Port.id` → `SemioGraphPort.name`, `Port.direction` →
/// `SemioGraphPortKind`), for genuine graph-shape tooling that only understands the neutral subset —
/// but the JSON blob is the round-trip SOURCE OF TRUTH on decode (matches `dag`'s own precedent, see
/// `📓️wave4-reports/dag-report.md`).
const JACK_NODE_JSON_PROPERTY: &str = "jack.node";

fn semio_port_kind_from_direction(direction: PortDirection) -> SemioGraphPortKind {
    match direction {
        PortDirection::In => SemioGraphPortKind::In,
        PortDirection::Out => SemioGraphPortKind::Out,
    }
}

fn port_direction_from_semio_port_kind(kind: SemioGraphPortKind) -> PortDirection {
    match kind {
        SemioGraphPortKind::In | SemioGraphPortKind::InOut => PortDirection::In,
        SemioGraphPortKind::Out => PortDirection::Out,
    }
}

fn semio_node_from_jack_node(node: &Node) -> SemioGraphNode {
    let ports = node.ports.iter().map(|port| SemioGraphPort { name: port.id.clone(), kind: semio_port_kind_from_direction(port.direction) }).collect();
    SemioGraphNode {
        id: SemioGraphNodeId::new(node.id.clone()),
        kind: node.kind.clone(),
        label: node.name.clone(),
        position: SemioPoint2 { x: node.x, y: node.y },
        ports,
        properties: vec![SemioValueEntry { key: JACK_NODE_JSON_PROPERTY.into(), value: SemioValue::Str { value: semio_framework_pack_json::to_json_string(node) } }],
    }
}

/// 🌉 Inverse of [`semio_node_from_jack_node`] — reconstructs the exact [`Node`] from its `jack.node`
/// JSON property. Falls back to a minimal node built from the graph-native `id`/`kind`/`label`/
/// `position`/`ports` fields only if the property is missing (content authored outside this plugin,
/// e.g. a hand-written `graph` doc) — never panics.
fn jack_node_from_semio_node(node: &SemioGraphNode) -> Node {
    for property in &node.properties {
        if property.key == JACK_NODE_JSON_PROPERTY {
            if let SemioValue::Str { value } = &property.value {
                if let Ok(parsed) = semio_framework_pack_json::from_json_str::<Node>(value, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                    return parsed;
                }
            }
        }
    }
    Node {
        id: node.id.value.clone(),
        kind: node.kind.clone(),
        name: node.label.clone(),
        x: node.position.x,
        y: node.position.y,
        width: 0.0,
        height: 0.0,
        properties: PropertyBag::new(),
        ports: node.ports.iter().map(|port| Port { id: port.name.clone(), kind: String::new(), direction: port_direction_from_semio_port_kind(port.kind), properties: PropertyBag::new() }).collect(),
    }
}

/// 🏷️ `SemioGraphEdge` has no `properties` slot (unlike `SemioGraphNode`) — its `label` field (which
/// this plugin's own [`Edge`] never populates on its own behalf) is repurposed to carry the FULL
/// `Edge` (port-qualified `source`/`target` endpoint strings, `properties`) as JSON, the round-trip
/// source of truth on decode. `source`/`target`/`kind` are also projected onto their native fields
/// (node-id only, port suffix stripped via [`crate::port_node_id`]) for genuine
/// graph-shape tooling.
fn semio_edge_from_jack_edge(edge: &Edge) -> SemioGraphEdge {
    let source_node = port_node_id(&edge.source).unwrap_or(&edge.source);
    let target_node = port_node_id(&edge.target).unwrap_or(&edge.target);
    SemioGraphEdge { id: SemioGraphEdgeId::new(edge.id.clone()), source: SemioGraphNodeId::new(source_node.to_string()), target: SemioGraphNodeId::new(target_node.to_string()), kind: edge.kind.clone(), label: semio_framework_pack_json::to_json_string(edge) }
}

/// 🌉 Inverse of [`semio_edge_from_jack_edge`] — falls back to a bare node-id (no port qualifier)
/// edge if `label` isn't valid `Edge` JSON (content authored outside this plugin) — never panics.
fn jack_edge_from_semio_edge(edge: &SemioGraphEdge) -> Edge {
    semio_framework_pack_json::from_json_str::<Edge>(&edge.label, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| Edge { id: edge.id.value.clone(), kind: edge.kind.clone(), source: edge.source.value.clone(), target: edge.target.value.clone(), properties: PropertyBag::new() })
}

/// 🌉 REAL bidirectional converter between jack's own live `Node`/`Edge` editing state and the
/// composed child's `SemioGraphSnapshot` node/edge graph (the "ModelBridge"/"DocumentBridge" pattern
/// — see `📓️wave3-reports/cad-report.md` and `📓️wave4-reports/dag-report.md`).
pub fn jack_content_snapshot_from_working(nodes: &[Node], edges: &[Edge]) -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: nodes.iter().map(semio_node_from_jack_node).collect(), edges: edges.iter().map(semio_edge_from_jack_edge).collect() }
}

/// 🌉 Inverse of [`jack_content_snapshot_from_working`].
pub fn working_from_jack_content_snapshot(content: &SemioGraphSnapshot) -> (Vec<Node>, Vec<Edge>) {
    (content.nodes.iter().map(jack_node_from_semio_node).collect(), content.edges.iter().map(jack_edge_from_semio_edge).collect())
}

/// 🕸️ Deterministic content-addressed CHILD handle for the jack content — same `(child_id, target)`
/// for identical `(nodes, edges)`, a different pair once the content actually changes; mirrors
/// `dag_content_child_handle`/`flow_content_child_handle`/`document_child_handle`.
pub fn jack_content_child_handle(nodes: &[Node], edges: &[Edge]) -> JackContentChild {
    let snapshot = jack_content_snapshot_from_working(nodes, edges);
    let content_json = semio_framework_pack_json::to_json_string(&snapshot);
    let child_id = store::content_id("jack-content", content_json.as_bytes());
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "graph".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral node/edge representation owned by one exact composed content child. It is
/// never serialized or process-global and retires with that owner.
#[derive(Clone, Debug, Default)]
pub struct JackWorkingScene {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

/// 📝 Transfers decoded or test-provided content into one exact child owner.
pub fn materialize_jack_content(handle: &mut JackContentChild, nodes: Vec<Node>, edges: Vec<Edge>) {
    handle.set_local_owner(std::sync::Arc::new(JackWorkingScene { nodes, edges }));
}

/// 🔎 Reads only the addressed child owner. A wire-only handle fails soft until host
/// materialization.
pub fn jack_working_scene_for_handle(handle: &JackContentChild) -> JackWorkingScene {
    handle.local_owner::<JackWorkingScene>().map(|scene| scene.as_ref().clone()).unwrap_or_default()
}

/// 🔎 Reads the current document's live nodes/edges off its `content` child handle — the single read
/// call site every mutation diff/inverse/app command in this plugin uses instead of the old
/// `snapshot.nodes`/`.edges` field access.
pub fn jack_working_scene(snapshot: &JackSnapshot) -> JackWorkingScene {
    jack_working_scene_for_handle(&snapshot.content)
}

/// 🌱️ The `content` member's genesis pack — the composed `s.stdio.semio` graph child a whole-document
/// load materialises; the shell sends `members: []`, so without it the archive closure is `Incomplete`.
pub fn genesis_jack_child_pack(snapshot: &JackSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use store::ArtifactPack;
    (slot == "content" && child_id == snapshot.content.child_id).then(|| {
        let scene = jack_working_scene(snapshot);
        <SemioGraphSnapshot as ArtifactPack>::encode_pack(&jack_content_snapshot_from_working(&scene.nodes, &scene.edges))
    })
}

/// 🏗️ Mints a new content-addressed handle and transfers its scene into that exact owner.
pub fn jack_content_child_with_owner(nodes: Vec<Node>, edges: Vec<Edge>) -> JackContentChild {
    let handle = jack_content_child_handle(&nodes, &edges);
    handle.with_local_owner(std::sync::Arc::new(JackWorkingScene { nodes, edges }))
}
//#endregion 🔖️WorkingScene

// #region 🔖️Runtime
/// 🔌️ Runtime port on a node.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Port {
    pub id: String,
    pub kind: String,
    pub direction: PortDirection,
    #[value(default)]
    pub properties: PropertyBag,
}

/// 🧩️ Runtime node (piece).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    #[value(default)]
    pub width: f64,
    #[value(default)]
    pub height: f64,
    #[value(default)]
    pub properties: PropertyBag,
    #[value(default)]
    pub ports: Vec<Port>,
}

/// 🔗️ Runtime edge (connection).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub target: String,
    #[value(default)]
    pub properties: PropertyBag,
}

/// 📷️ Camera for snapshot documents.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

impl JackSnapshot {
    pub const SCHEMA: &'static str = "trinity.graph";

    pub fn validate_schema(&self) -> Result<(), TrinityRamError> {
        if self.schema != Self::SCHEMA {
            return Err(TrinityRamError::SchemaMismatch { expected: Self::SCHEMA, actual: self.schema.clone() });
        }
        Ok(())
    }

    /// 📤️ JSON snapshot text — unlike `Serialize`'s derive (which would emit the opaque `content`
    /// handle only, unrecoverable once the working-scene cache that minted it is gone, e.g. across a
    /// process boundary or a persisted embedded snapshot string), this hand-rolled JSON shape embeds
    /// the REAL `nodes`/`edges` at the top level, mirroring the old pre-migration wire shape and
    /// matching the same "wire format carries real content, not just the handle" fix the hand-rolled
    /// `ArtifactDsl`/`ArtifactPack` codecs use (see `📸️snapshot/📝️text/🦀️.rs`'s own doc
    /// comment for the full rationale).
    pub fn to_json(&self) -> Result<String, TrinityRamError> {
        let scene = jack_working_scene(self);
        let value = semio_framework_pack_json::json!({
            "schema": self.schema,
            "name": self.name,
            "manifestId": self.manifest_id,
            "manifest": self.manifest,
            "camera": self.camera,
            "nodes": scene.nodes,
            "edges": scene.edges,
            "rootNodeId": self.root_node_id,
            "query": self.query,
        });
        Ok(semio_framework_pack_json::to_string_pretty(&value))
    }

    pub fn resolve_manifest(&mut self) -> Result<(), TrinityRamError> {
        if let Some(id) = self.manifest_id.as_deref() {
            self.manifest = Manifest::from_graph(&manifest_by_id(id).ok_or_else(|| TrinityRamError::UnknownManifestId(id.to_string()))?);
            return Ok(());
        }
        if self.manifest.node_kinds.is_empty() && self.manifest.edge_kinds.is_empty() && self.manifest.port_kinds.is_empty() {
            return Err(TrinityRamError::ManifestMissing);
        }
        Ok(())
    }

    /// 📥️ Inverse of [`Self::to_json`] — parses the real `nodes`/`edges` JSON arrays and mints+caches
    /// a fresh content-addressed handle from them (deterministic: identical `(nodes, edges)` always
    /// re-derives the same handle, so peers replaying the same JSON text converge).
    pub fn from_json(json: &str) -> Result<Self, TrinityRamError> {
        let value: semio_framework_pack_json::Value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        let schema = value.get("schema").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let name = value.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let manifest_id: Option<String> = value.get("manifestId").and_then(|v| v.as_str()).map(str::to_string);
        let manifest: Manifest = value.get("manifest").map(|v| dsl::FromValue::from_value(semio_framework_pack_json::to_dsl_value(v))).transpose()?.unwrap_or_default();
        let camera: Camera = value.get("camera").map(|v| dsl::FromValue::from_value(semio_framework_pack_json::to_dsl_value(v))).transpose()?.unwrap_or_default();
        let nodes: Vec<Node> = value.get("nodes").map(|v| dsl::FromValue::from_value(semio_framework_pack_json::to_dsl_value(v))).transpose()?.unwrap_or_default();
        let edges: Vec<Edge> = value.get("edges").map(|v| dsl::FromValue::from_value(semio_framework_pack_json::to_dsl_value(v))).transpose()?.unwrap_or_default();
        let root_node_id: Option<String> = value.get("rootNodeId").and_then(|v| v.as_str()).map(str::to_string);
        let query = value.get("query").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let mut snapshot = Self { query, ..Self::with_content(schema, name, manifest_id, manifest, camera, JackWorkingScene { nodes: nodes, edges: edges }, root_node_id) };
        snapshot.validate_schema()?;
        snapshot.resolve_manifest()?;
        Ok(snapshot)
    }

    /// 🏗️ Transfers one working scene into the snapshot's exact composed content owner. The query starts empty: a document
    /// constructor names its own (`Self { query, ..Self::with_content(..) }`).
    pub fn with_content(schema: String, name: String, manifest_id: Option<String>, manifest: Manifest, camera: Camera, scene: JackWorkingScene, root_node_id: Option<String>) -> Self {
        Self { schema, name, manifest_id, manifest, camera, content: jack_content_child_with_owner(scene.nodes, scene.edges), root_node_id, query: String::new() }
    }

    /// 🔎 Live node list, read through the working-scene cache — replaces the old direct `.nodes`
    /// field access (see `🔖️WorkingScene`'s module doc for why this indirection exists).
    pub fn nodes(&self) -> Vec<Node> {
        jack_working_scene(self).nodes
    }

    /// 🔎 Live edge list, read through the working-scene cache.
    pub fn edges(&self) -> Vec<Edge> {
        jack_working_scene(self).edges
    }
}

/// 🧠️ In-memory trinity graph.
#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
    pub name: String,
    pub manifest_id: Option<String>,
    pub manifest: Manifest,
    pub camera: Camera,
    pub nodes: BTreeMap<String, Node>,
    pub edges: BTreeMap<String, Edge>,
    pub root_node_id: Option<String>,
    pub query: String,
}

impl Graph {
    pub fn from_snapshot(mut snapshot: JackSnapshot) -> Result<Self, TrinityRamError> {
        snapshot.validate_schema()?;
        snapshot.resolve_manifest()?;
        if let Some(id) = snapshot.manifest_id.as_deref() {
            if let Some(gm) = manifest_by_id(id) {
                validate_trinity_snapshot(&gm, &snapshot)?;
            }
        }
        let scene = jack_working_scene(&snapshot);
        let mut nodes = BTreeMap::new();
        for node in scene.nodes {
            nodes.insert(node.id.clone(), node);
        }
        let mut edges = BTreeMap::new();
        for edge in scene.edges {
            edges.insert(edge.id.clone(), edge);
        }
        Ok(Self { name: snapshot.name, manifest_id: snapshot.manifest_id, manifest: snapshot.manifest, camera: snapshot.camera, nodes, edges, root_node_id: snapshot.root_node_id, query: snapshot.query })
    }

    pub fn to_snapshot(&self) -> JackSnapshot {
        JackSnapshot { query: self.query.clone(), ..JackSnapshot::with_content(JackSnapshot::SCHEMA.to_string(), self.name.clone(), self.manifest_id.clone(), self.manifest.clone(), self.camera.clone(), JackWorkingScene { nodes: self.nodes.values().cloned().collect(), edges: self.edges.values().cloned().collect() }, self.root_node_id.clone()) }
    }

    pub fn load_json(json: &str) -> Result<Self, TrinityRamError> {
        Self::from_snapshot(JackSnapshot::from_json(json)?)
    }

    pub fn host_snapshot_json(&self) -> Result<String, TrinityRamError> {
        self.to_snapshot().to_json()
    }

    /// 🧩️ Build a `trinity.graph` snapshot containing only the given node and edge ids.
    pub fn subgraph_fixture(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> JackSnapshot {
        let nodes: Vec<Node> = node_ids.iter().filter_map(|id| self.nodes.get(id).cloned()).collect();
        let edges: Vec<Edge> = edge_ids.iter().filter_map(|id| self.edges.get(id).cloned()).collect();
        let root_node_id = self.root_node_id.clone().filter(|id| node_ids.contains(id));
        JackSnapshot::with_content(JackSnapshot::SCHEMA.to_string(), format!("{} subgraph", self.name), self.manifest_id.clone(), self.manifest.clone(), self.camera.clone(), JackWorkingScene { nodes: nodes, edges: edges }, root_node_id)
    }

    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn node_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn edge(&self, id: &str) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn remove_node(&mut self, id: &str) -> bool {
        if self.nodes.remove(id).is_none() {
            return false;
        }
        let edge_ids: Vec<String> = self.edges.iter().filter(|(_, e)| port_node_id(&e.source) == Some(id) || port_node_id(&e.target) == Some(id)).map(|(id, _)| id.clone()).collect();
        for eid in edge_ids {
            self.edges.remove(&eid);
        }
        if self.root_node_id.as_deref() == Some(id) {
            self.root_node_id = None;
        }
        true
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge.id.clone(), edge);
    }

    pub fn remove_edge(&mut self, id: &str) -> bool {
        self.edges.remove(id).is_some()
    }

    pub fn set_property(&mut self, entity: EntityRef, key: &str, value: PropertyValue) -> Result<(), TrinityRamError> {
        match entity {
            EntityRef::Node(id) => {
                let node = self.nodes.get_mut(&id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
                node.properties.insert(key.to_string(), value);
            }
            EntityRef::Edge(id) => {
                let edge = self.edges.get_mut(&id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
                edge.properties.insert(key.to_string(), value);
            }
        }
        Ok(())
    }
}

/// 🛡️ Validates trinity snapshot instances against a compile-time graph manifest.
fn validate_trinity_snapshot(gm: &GraphManifest, snapshot: &JackSnapshot) -> Result<(), TrinityRamError> {
    let validator = ManifestValidator::new(gm);
    let scene = jack_working_scene(snapshot);
    for node in &scene.nodes {
        validator.validate_node_kind(&node.kind)?;
        validator.validate_node_properties(&node.kind, &node.properties)?;
        if let Some(node_def) = gm.node_kind(&node.kind) {
            for port in &node.ports {
                validator.validate_port_kind(&port.kind)?;
                if !node_def.ports.is_empty() && !node_def.ports.iter().any(|p| p == &port.kind) {
                    return Err(TrinityRamError::PortKindNotDeclaredOnFixture { node_id: node.id.clone(), port_kind: port.kind.clone(), node_kind: node.kind.clone() });
                }
            }
        }
    }
    for edge in &scene.edges {
        validator.validate_edge_kind(&edge.kind)?;
        validator.validate_edge_properties(&edge.kind, &edge.properties)?;
    }
    Ok(())
}

/// 🎯️ Entity reference for mutations.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", tag = "entity", content = "id")]
pub enum EntityRef {
    Node(String),
    Edge(String),
}

/// 🔑️ Parse `nodeId@portId` port key (`@` is the unified syntax's one port sigil — `:` is reserved for typing).
pub fn parse_port_key(key: &str) -> Option<(&str, &str)> {
    let (node, port) = key.split_once('@')?;
    if node.is_empty() || port.is_empty() {
        return None;
    }
    Some((node, port))
}

/// 🧩️ Node id from a port key.
pub fn port_node_id(key: &str) -> Option<&str> {
    parse_port_key(key).map(|(n, _)| n)
}

/// 🔌️ Port id from a port key.
pub fn port_port_id(key: &str) -> Option<&str> {
    parse_port_key(key).map(|(_, p)| p)
}

/// 🏗️ Build a port key.
pub fn port_key(node_id: &str, port_id: &str) -> String {
    format!("{node_id}@{port_id}")
}

pub const TRINITY_GRAPH_SCHEMA: &str = JackSnapshot::SCHEMA;

/// 🔎️ The Jack query a new document opens with — `JackSnapshot::query` is document content, undoable and shared like the graph.
pub const TRINITY_JACK_DEFAULT_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label";

/// 📏️ Largest query text a jack document holds: one `set-query` stays inside the 4 KiB document-mutation admission
/// (`TRINITY_JACK_ARTIFACT_MUTATION_MAXIMUM_BYTES`) and the store initializer's 4 KiB owned-field bound.
pub const JACK_QUERY_MAXIMUM_BYTES: usize = 3_584;

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (editor AND viewer) of this artifact shares — lives at the ARTIFACT level, not under
/// `editor`, so a viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind = "s.trinity.jack"` matches `#[artifact_schema(id = "s.trinity.jack")]` in this
/// subset's own `🧬️schema/🦀️component.rs`; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.trinity.jack@1/*#editor` / `s.trinity.jack@1/*#viewer` (contract §1 grammar).
pub const TRINITY_JACK_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.trinity.jack", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };

pub fn empty_trinity_graph_fixture() -> JackSnapshot {
    JackSnapshot { query: TRINITY_JACK_DEFAULT_QUERY.into(), ..JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "trinity".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: Vec::new(), edges: Vec::new() }, None) }
}

/// 🎯️ `ArtifactKindSpec` identity shared by every `jack`-family app that mounts this artifact.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "graph.trinity".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Trinity Graph", "Trinity-Graph"),
        source_format: "trinity.graph".into(),
        component_kind: "trinity".into(),
        dimension: "graph".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Graph, form: semio_framework_plugin::MediaForm::Trinity },
        schema: "trinity.graph".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
    }
}
// #endregion 🔖️Runtime

//#region 🔖️Register
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `io_registry::entries()`'s own `OnceLock` convention. `pub` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, fleet-trinity-recipe): the new declaration
/// tree's `🪆️subsets/✳️any/🦀️.rs` reads these same five `LanguageSpec`s to build its
/// `NativeCodecs` `LanguagePair`s (see that file's own doc for why it does not delegate to a sibling
/// `crate::standards::v1::subsets::any::io::io()` the way `🗒️note`/`🖍️draw` do).
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "jack.document",
                    extension: Some("trinity"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("jack.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's OLD-channel definition (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1).
/// KEPT unread by the new declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
/// debt D1 — deleted repo-wide only once every plugin has migrated, not this pass — `🗒️note`/`🖍️draw`
/// precedent, `📓️terra-fleet-trinity-recipe-report.md`): the real en/de localized names
/// (`"Jack"`/`"Buchse"`) still live only on these `ArtifactCapability` rows.
/// Jack query source and execution output now register through their exact window owners.
#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::jack::TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::jack::TrinityJackViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::jack::TrinityJackPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::jack::TrinityJackViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.trinity.jack.standard.v1", "standard", "1", &[], None),
        ("s.trinity.jack.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.trinity.jack.schema.artifact", "schema", "s.trinity.jack", &[("schema", "s.trinity.jack")], None),
        ("s.trinity.jack.inference.artifact", "inference", "s.trinity.jack.inference", &[("schema", "s.trinity.jack.inference")], None),
        ("s.trinity.jack.composer.native", "composer", "s.trinity.jack@1/*", &[("dialect", "s.trinity.jack@1/*")], None),
        ("s.trinity.jack.composer.format-5", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.trinity.jack.grammar.1", "grammar", "jack.document", &[("grammar", "jack.document")], None),
        ("s.trinity.jack.grammar.2", "grammar", "jack.op", &[("grammar", "jack.op")], None),
        ("s.trinity.jack.grammar.3", "grammar", "jack.diff", &[("grammar", "jack.diff")], None),
        ("s.trinity.jack.grammar.4", "grammar", "jack.pack", &[("grammar", "jack.pack")], None),
        ("s.trinity.jack.grammar.5", "grammar", "jack.spr", &[("grammar", "jack.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<TrinityJackPlayApp>>()`
        // derives its extension claim from `<JackSnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🧬️schema/📸️snapshot/📝️text/🦀️.rs`), which is `"trinity"`, not `"jack"`.
        ("s.trinity.jack.codec.document-1", "codec", "trinity.graph:trinity", &[("codec", "trinity.graph"), ("codec-extension", "13:trinity.graph:trinity")], None),
        ("s.trinity.jack.localization.en", "localization", "Jack", &[], Some(("en", "Jack"))),
        ("s.trinity.jack.localization.de", "localization", "Buchse", &[], Some(("de", "Buchse"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.trinity.jack")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🌳️ This artifact's declaration tree root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
/// MECHANISM design.md §2, fleet-trinity-recipe) — replaces the old `declaration()`
/// (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).languages(...)
/// .document_codec(...)` chain, deleted outright, no dual channel) as the ONLY registration channel
/// for schema/io/viewer/editor rows. `definition()` (old `ArtifactDefinition`/capability rows, above)
/// is kept per debt D1, and `artifact_kind()` is kept because this crate's own plugin-root
/// `.activation(...)` still reads `artifact_kind().id`; neither has any caller left in this function.
#[cfg(feature = "component-app-assembly")]
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.trinity.jack").expect("canonical jack kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}
//#endregion 🔖️Register

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[cfg(feature = "component-app-assembly")]
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        #[cfg(feature = "component-app-assembly")]
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[cfg(feature = "component-app-assembly")]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                #[cfg(feature = "component-app-assembly")]
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs"]
                        pub(crate) mod sqlite;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod topology {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod flat_position {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        pub use text::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "."]
                    pub mod operations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod wire_runtime {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod create_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-node/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-node/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_a_node_id_the_scene_already_holds;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-node/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_deleting_a_node_the_scene_never_had;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod create_edge {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-edge/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-edge/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-edge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-edge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-edge/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_an_edge_whose_endpoints_are_absent;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-edge/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod delete_edge {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_cutting_an_edge_the_scene_never_had;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod rename_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🧪️tests/✏️keeps/🦀️.rs"]
                            mod tests_keeps_the_name_a_node_already_carries;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod move_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🧪️tests/📍️keeps/🦀️.rs"]
                            mod tests_keeps_a_node_at_the_point_it_already_occupies;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod change_data_property {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🧪️tests/🏷️keeps/🦀️.rs"]
                            mod tests_keeps_a_node_property_at_the_value_it_already_holds;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod remove_data_property {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🧪️tests/🧹️keeps/🦀️.rs"]
                            mod tests_keeps_an_edge_without_the_property_it_never_had;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod set_query {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/🧪️tests/🔎️replaces-the-query/🦀️.rs"]
                            mod tests_replaces_the_query;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
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
        }
    }
}

pub use crate::standards::v1::subsets::any::schema::diff::JackDiff;
/// 📍️ Cross-artifact node movement constructor used by Rewriting's Jack-backed editor world.
pub use crate::standards::v1::subsets::any::schema::mutations::move_node::move_node;
pub use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
pub use crate::standards::v1::subsets::any::schema::operations::*;
/// 📸️ Persisted Jack snapshot shared by the artifact's schema and app surfaces.
pub use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;

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

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jack {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️lod/🦀️.rs"]
        pub mod lod;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
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

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🦀️.rs"]
        pub mod window_config;

        #[path = "."]
        pub mod transient {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub(crate) mod query {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            mod set_fixture_json_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧫️set-fixture-json/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_fixture_json_leaf::set_fixture_json;

            #[path = "."]
            mod delete_selection_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use delete_selection_leaf::delete_selection;

            #[path = "."]
            mod patch_nodes_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-nodes/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use patch_nodes_leaf::patch_nodes;

            #[path = "."]
            mod set_active_example_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-active-example/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_active_example_leaf::{preset_query, set_active_example};

            #[path = "."]
            mod format_document_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✨️format-document/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use format_document_leaf::format_document;

            #[path = "."]
            mod set_viewport_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖥️set-viewport/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_viewport_leaf::set_viewport;

            #[path = "."]
            mod text_edit_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️text-edit/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use text_edit_leaf::text_edit;

            #[path = "."]
            mod set_lod_mode_leaf {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️set-lod/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_lod_mode_leaf::set_lod_mode;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod tools {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs"]
                    pub mod reorganize;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub(crate) mod graph {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod editor {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod results {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub(crate) mod document {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jack {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️graph/🦀️.rs"]
                    pub mod graph;
                }
            }
        }
    }
}

//#region 🧬️ChildRestoreProjection
/// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields — the ONE
/// definition the editor and the viewer both declare. The `ArtifactEditor`/`ArtifactViewer` trait default
/// refuses it (`… did not declare a loaded-parent child projection`), and since PX1 the live envelope load
/// asks for it before the decoded document may replace the store, so an undeclared app fails every live load.
pub fn jack_child_restore_projection(snapshot: &crate::JackSnapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("jack.child-projection"), error.to_string()))
}
//#endregion 🧬️ChildRestoreProjection

#[cfg(test)]
#[path = "🛂️manifest/🧪️tests/🔬️unit/🦀️.rs"]
mod graph_manifest_tests;

```

### ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🛂️manifest/🦀️.rs

SHA-256 `61dd2a00cec2429fc16465348104f829c7113133bb908fa21f724347524ad745`; 3040 bytes.

```
//! 📜️ Jack manifest projection owned by the Jack artifact.
use crate::{GraphManifest, PortDirection, PropertyDef};

/// 🔺️ Trinity-shaped manifest projection for jack/ram consumers.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Manifest {
    #[value(default)]
    pub node_kinds: Vec<NodeKindDef>,
    #[value(default)]
    pub edge_kinds: Vec<EdgeKindDef>,
    #[value(default)]
    pub port_kinds: Vec<PortKindDef>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct NodeKindDef {
    pub name: String,
    #[value(default)]
    pub properties: Vec<PropertyDef>,
    #[value(default, rename = "portKinds")]
    pub port_kinds: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct EdgeKindDef {
    pub name: String,
    #[value(default)]
    pub properties: Vec<PropertyDef>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PortKindDef {
    pub name: String,
    pub direction: PortDirection,
    #[value(default)]
    pub properties: Vec<PropertyDef>,
}

impl Manifest {
    pub fn node_kind(&self, name: &str) -> Option<&NodeKindDef> {
        self.node_kinds.iter().find(|k| k.name == name)
    }

    pub fn edge_kind(&self, name: &str) -> Option<&EdgeKindDef> {
        self.edge_kinds.iter().find(|k| k.name == name)
    }

    pub fn port_kind(&self, name: &str) -> Option<&PortKindDef> {
        self.port_kinds.iter().find(|k| k.name == name)
    }

    /// 📜️ Nakagin capsule tower compile-time manifest.
    pub fn nakagin_default() -> Self {
        Self::from_graph(&crate::graph_manifest::nakagin::nakagin_manifest())
    }
}

impl Manifest {
    pub fn from_graph(graph: &GraphManifest) -> Self {
        Self {
            node_kinds: graph.node_kinds.iter().map(|k| NodeKindDef { name: k.id.clone(), properties: k.properties.clone(), port_kinds: k.ports.clone() }).collect(),
            edge_kinds: graph.edge_kinds.iter().map(|k| EdgeKindDef { name: k.id.clone(), properties: k.properties.clone() }).collect(),
            port_kinds: graph
                .port_kinds
                .iter()
                .filter_map(|k| {
                    let direction = k.direction.or_else(|| {
                        k.presentation.as_ref().and_then(|p| p.get("direction")).and_then(|d| match d.as_str()? {
                            "in" => Some(PortDirection::In),
                            "out" => Some(PortDirection::Out),
                            _ => None,
                        })
                    })?;
                    Some(PortKindDef { name: k.id.clone(), direction, properties: k.properties.clone() })
                })
                .collect(),
        }
    }
}

```

### ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

SHA-256 `c7df1174bb9fa98ae64cf93aa8652fe8bb07e7b6f525f0f2b08bdcfb9e4a798e`; 12692 bytes.

```
//! 🧬️ Jack snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` are gone from this STRUCT —
//! replaced by a single composed `content: JackContentChild` slot (`s.stdio.semio.graph`). See
//! `🗿️artifacts/🔌️jack/🦀️.rs`'s `🔖️ContentBridge`/`🔖️WorkingScene` regions for the
//! converter/handle/cache machinery this field depends on.

use crate::{Camera, JackContentChild, Manifest};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted trinity graph document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, dsl::DslRecord)]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub manifest_id: Option<String>,
    #[state(artifact)]
    pub manifest: Manifest,
    #[state(artifact)]
    pub camera: Camera,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: JackContentChild,
    #[state(artifact)]
    pub root_node_id: Option<String>,
    /// 🔎️ The document's Jack query — the query editor's text, document content like the graph it runs against.
    #[state(artifact)]
    pub query: String,
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `content` is a `store::ArtifactChild<S>` composed-artifact
/// handle — see `JackArtifact`'s identical trap in the sibling `🦀️.rs` (this struct's
/// non-child fields carried `#[serde(default, skip_serializing_if = "Option::is_none")]`
/// before this wave; `manifest_id`/`root_node_id` are `Option<String>`, and the blanket
/// `impl<T: FromValue> FromValue for Option<T>` already treats a missing key as `None` via the
/// derive macro's own generated `missing` arm being unreachable here since every field is
/// present below — no separate default handling needed in a hand-written impl).
impl dsl::ToValue for JackSnapshot {
    fn to_value_controlled(&self, c: &mut dsl::NativeEncodeControl<'_>) -> Result<dsl::DslValue, dsl::ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(8)?;
            let mut fields = dsl::DslValue::object_encoding_controlled(8, c)?;
            macro_rules! field {
                ($key:literal,$value:expr) => {{
                    let child = dsl::ToValue::to_value_controlled($value, c)?;
                    dsl::DslValue::push_encoding_controlled(fields.get_mut(), $key, child, c)?;
                    c.step()?;
                }};
            }
            field!("schema", &self.schema);
            field!("name", &self.name);
            if let Some(value) = &self.manifest_id {
                field!("manifestId", value)
            } else {
                c.step()?;
            }
            field!("manifest", &self.manifest);
            field!("camera", &self.camera);
            field!("content", &self.content);
            if let Some(value) = &self.root_node_id {
                field!("rootNodeId", value)
            } else {
                c.step()?;
            }
            field!("query", &self.query);
            Ok(dsl::DslValue::Object(fields.take()))
        })
    }

    /// 🕳️ `manifestId`/`rootNodeId` are SKIPPED while `None` (the old `skip_serializing_if` law the
    /// committed `📸️snapshot` fixture vectors are written against): decode→encode of a committed
    /// snapshot is a fixed point only if an absent id stays absent instead of surfacing as `null`.
    fn to_value(&self) -> dsl::DslValue {
        let mut entries: Vec<(String, dsl::DslValue)> = Vec::with_capacity(8);
        entries.push(("schema".to_string(), dsl::ToValue::to_value(&self.schema)));
        entries.push(("name".to_string(), dsl::ToValue::to_value(&self.name)));
        if let Some(manifest_id) = self.manifest_id.as_ref() {
            entries.push(("manifestId".to_string(), dsl::ToValue::to_value(manifest_id)));
        }
        entries.push(("manifest".to_string(), dsl::ToValue::to_value(&self.manifest)));
        entries.push(("camera".to_string(), dsl::ToValue::to_value(&self.camera)));
        entries.push(("content".to_string(), semio_framework_value::ToValue::to_value(&self.content)));
        if let Some(root_node_id) = self.root_node_id.as_ref() {
            entries.push(("rootNodeId".to_string(), dsl::ToValue::to_value(root_node_id)));
        }
        entries.push(("query".to_string(), dsl::ToValue::to_value(&self.query)));
        dsl::DslValue::object(entries)
    }
}
impl dsl::FromValue for JackSnapshot {
    fn from_value_controlled(value: &dsl::DslValue, c: &mut dsl::NativeDecodeControl<'_>) -> Result<Self, dsl::ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(8)?;
            let fields = value.object_controlled(c)?;
            c.charge(size_of::<Self>())?;
            fn optional_field<T: dsl::FromValue + Default>(fields: &[(String, dsl::DslValue)], key: &str, c: &mut dsl::NativeDecodeControl<'_>) -> Result<T, dsl::ValueError> {
                let value = match dsl::DslValue::field_controlled(fields, key, c)? {
                    Some(value) => T::from_value_controlled(value, c)?,
                    None => T::default(),
                };
                let value = dsl::DecodedValue::new(value, T::retire_decoded);
                c.step()?;
                Ok(value.take())
            }
            let schema = optional_field(fields, "schema", c)?;
            let name = optional_field(fields, "name", c)?;
            let manifest_id = optional_field(fields, "manifestId", c)?;
            let manifest = dsl::DecodedValue::new(optional_field::<Manifest>(fields, "manifest", c)?, <Manifest as dsl::FromValue>::retire_decoded);
            let camera = optional_field(fields, "camera", c)?;
            let child = dsl::DslValue::field_controlled(fields, "content", c)?.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field content"))?;
            let content = dsl::DecodedValue::new(<JackContentChild as dsl::FromValue>::from_value_controlled(child, c)?, <JackContentChild as dsl::FromValue>::retire_decoded);
            c.step()?;
            let root_node_id = optional_field(fields, "rootNodeId", c)?;
            let query = dsl::DslValue::field_controlled(fields, "query", c)?.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field query"))?;
            let query = <String as dsl::FromValue>::from_value_controlled(query, c)?;
            let output = dsl::DecodedValue::new(Self { schema, name, manifest_id, manifest: manifest.take(), camera, content: content.take(), root_node_id, query }, Self::retire_decoded);
            c.step()?;
            Ok(output.take())
        })
    }
    fn retire_decoded(self) {
        super::sqlite::retire_snapshot(self)
    }

    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        Ok(Self {
            schema: match get("schema") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => Default::default(),
            },
            name: match get("name") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => Default::default(),
            },
            manifest_id: match get("manifestId") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => None,
            },
            manifest: match get("manifest") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => Default::default(),
            },
            camera: match get("camera") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => Default::default(),
            },
            content: semio_framework_value::FromValue::from_value(get("content").ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `content`"))?)?,
            root_node_id: match get("rootNodeId") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => None,
            },
            query: dsl::FromValue::from_value(get("query").ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `query`"))?)?,
        })
    }
}
//#endregion 🔖️ValueCodec

impl Default for JackSnapshot {
    fn default() -> Self {
        Self {
            schema: crate::TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            content: crate::jack_content_child_with_owner(Vec::new(), Vec::new()),
            root_node_id: None,
            query: crate::TRINITY_JACK_DEFAULT_QUERY.into(),
        }
    }
}

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`JackSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `🔌️mutate-jack-1`'s scenarios are measured through, and the shape the committed
/// `../🧫️fixtures/🧬️mutations/<slug>/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. It carries `content` as a HANDLE, never as a scene, and
/// that handle's `childId` is a digest of the child — so it moves if and only if the working scene
/// moved, which is what makes it a usable observability surface here.
///
/// A thin `pack::json` wrapper over [`JackSnapshot`]'s own `ToValue`, bridged through
/// `pack::json_from_dsl_value` since `DslValue` and `pack::json::Value` are sibling trees (used
/// behind this interface per CLAUDE.md's "external libraries behind an interface" rule).
pub fn encode_jack_snapshot_json(snapshot: &JackSnapshot) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&crate::standards::v1::subsets::any::io::json_native::convert(dsl::ToValue::to_value(snapshot), false).expect("typed Jack camera words")))
}

/// 📥️ The inverse of [`encode_jack_snapshot_json`] — decodes those committed specification vectors
/// into real [`JackSnapshot`] values, so `🔌️mutate-jack-1`'s adapter reads the committed fixture
/// rather than re-declaring it as a Rust literal beside it.
pub fn decode_jack_snapshot_json(text: &str) -> Result<JackSnapshot, String> {
    let parsed = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <JackSnapshot as dsl::FromValue>::from_value(crate::standards::v1::subsets::any::io::json_native::convert(semio_framework_pack_json::to_dsl_value(&parsed), true)?).map_err(|error| error.to_string())
}

/// 📝️ Parses the literal Jack parent and its independent content-child address.
/// Child materialization belongs to the host's composed artifact boundary.
pub fn parse_jack_dsl(text: &str) -> Result<JackSnapshot, String> {
    <JackSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders the literal parent record with its native document preamble.
pub fn print_jack_dsl(snapshot: &JackSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ The scene's node names and `source -> target` edge ids the document's composed child currently
/// resolves to — the readable half of a divergence message, so a failing scenario names WHICH piece
/// moved rather than only that two content digests differ.
pub fn jack_scene_summary(snapshot: &JackSnapshot) -> String {
    let scene = crate::jack_working_scene(snapshot);
    let nodes = scene.nodes.iter().map(|node| format!("{}({})", node.name, node.id)).collect::<Vec<_>>().join(" ");
    let edges = scene.edges.iter().map(|edge| edge.id.clone()).collect::<Vec<_>>().join(" ");
    format!("nodes[{nodes}] edges[{edges}]")
}
//#endregion 🌉️ExternalCodecBridge

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

impl semio_framework_schema_composition::ChildFieldRefs for JackSnapshot {
    const MANY: bool = false;
    fn visit_child_field<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,slot:&'static str,visitor:&mut V)->Result<(),V::Error>{
        visitor.step()?;
        semio_framework_schema_composition::ChildFieldRefs::visit_child_field(&self.content,slot,visitor)
    }
}

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs

SHA-256 `6582add1349e8ec4d2fc4bcc837fe5a00c8b8f079d945cbffa01924c3a0a4368`; 167546 bytes.

```
//! 🧬️ `dsl_derive` — compiles `#[dsl(...)]`-annotated struct/enum declarations into
//! `dsl::DslField`/`dsl::DslVariants` bindings (nested usage composes through), so a technology
//! declares its grammar instead of hand-writing a parser/printer. Analyze → IR → emit.
//!
//! P6: `DslArtifact`/`DslOps` no longer emit `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` —
//! those traits are handcrafted per artifact. `DslRecord` stays for field helpers only.
//!
//! Whole crate is sync (E3): a proc-macro entry point's signature is language-fixed to
//! `fn(TokenStream) -> TokenStream` and rustc rejects an `async fn` here outright (a proc macro
//! runs inside rustc at compile time, where there is no executor to poll it). Bounded, no-follow
//! source-authority reads occur during expansion; every helper remains sync because there is no
//! executor to poll.

use proc_macro::TokenStream;
use quote::quote;
use std::{collections::{BTreeMap, HashSet}, fs, path::{Component, Path, PathBuf}};
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[cfg(test)]
#[path = "🧪️tests/📤️macro-exports/🦀️.rs"]
mod macro_export_tests;

#[cfg(test)]
#[path = "🧪️tests/🪪️mandatory-mutation-descriptor/🦀️.rs"]
mod mandatory_mutation_descriptor_tests;

//#region 🔖️MutationSourceAuthority
#[derive(Debug)]
struct MutationSourceAuthority {
    workspace_root: PathBuf,
    mutation_root: PathBuf,
    owner: String,
    expected_semantic_kind: Option<String>,
    source_path: PathBuf,
    descriptor_path: PathBuf,
    taxonomy_path: PathBuf,
}

type MutationDomainOperations = Vec<(String, String)>;

/// 🧭️ Workspace-relative locator of the taxonomy's generated mutation-source-authority projection (`bun nx run
/// @semio-tech/dsl-derive-rs:generate`), the only authority file an expansion reads and tracks, so an edit elsewhere in the taxonomy,
/// `nx.json` or any `📋️project.json` never invalidates a crate that derives mutations.
const MUTATION_AUTHORITY_LOCATOR: &str = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json";

/// 🪪️ Schema id the projection must declare (`MutationSourceAuthorityProjectionV1` in `🧬️schema/🔣️.json`).
const MUTATION_AUTHORITY_SCHEMA: &str = "semio.dsl.mutation-source-authority/v1";

#[derive(Debug)]
struct MutationAuthorityCommon {
    workspace_root: PathBuf,
    source_path: PathBuf,
    taxonomy_path: PathBuf,
    mutation_collection: String,
    mutation_payload_facet: String,
    source_filename: String,
    descriptor_filename: String,
    domain_owners: BTreeMap<String, MutationDomainOperations>,
    aggregate_sources: BTreeMap<String, Vec<String>>,
}

#[derive(Debug)]
struct MutationAggregateSourceAuthority {
    workspace_root: PathBuf,
    mutation_root: PathBuf,
    #[cfg(test)]
    source_path: PathBuf,
    taxonomy_path: PathBuf,
    mutation_payload_facet: String,
    source_filename: String,
    descriptor_filename: String,
    domain_operations: Option<MutationDomainOperations>,
    component_roots: Vec<(String, Option<MutationDomainOperations>)>,
}

fn mutation_authority_common(source: &Path, compiler_cwd: &Path) -> Result<MutationAuthorityCommon, String> {
    let source_path = mutation_authority_normalize(source, compiler_cwd)?;
    let workspace_root = mutation_authority_workspace_root(&source_path)?;
    mutation_authority_no_follow(&workspace_root, &source_path, false)?;
    let taxonomy_path = mutation_authority_locator(&workspace_root, MUTATION_AUTHORITY_LOCATOR)?;
    mutation_authority_no_follow(&workspace_root, &taxonomy_path, false).map_err(|error| format!("mutation authority projection {MUTATION_AUTHORITY_LOCATOR}: {error}; run bun nx run @semio-tech/dsl-derive-rs:generate"))?;
    let authority: serde_json::Value = serde_json::from_slice(&fs::read(&taxonomy_path).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
    if authority.get("schema").and_then(serde_json::Value::as_str) != Some(MUTATION_AUTHORITY_SCHEMA) { return Err("mutation authority projection declares another schema".to_string()); }
    let source_filename = mutation_authority_segment(&authority, "sourceFilename")?;
    let descriptor_filename = mutation_authority_segment(&authority, "descriptorFilename")?;
    let mutation_collection = mutation_authority_segment(&authority, "mutationCollection")?;
    let mutation_payload_facet = mutation_authority_segment(&authority, "mutationPayloadFacet")?;
    let domain_owners = mutation_authority_domain_owners(&authority, &mutation_collection)?;
    let aggregate_sources = mutation_authority_aggregate_sources(&authority, &mutation_collection)?;
    Ok(MutationAuthorityCommon { workspace_root, source_path, taxonomy_path, mutation_collection, mutation_payload_facet, source_filename, descriptor_filename, domain_owners, aggregate_sources })
}

fn mutation_source_authority(source: &Path, compiler_cwd: &Path) -> Result<MutationSourceAuthority, String> {
    let common = mutation_authority_common(source, compiler_cwd)?;
    let MutationAuthorityCommon { workspace_root, source_path, taxonomy_path, mutation_collection, mutation_payload_facet, source_filename, descriptor_filename, domain_owners, .. } = common;
    if source_path.file_name().and_then(|name| name.to_str()) != Some(source_filename.as_str()) { return Err("source is not the taxonomy canonical mutation primary".to_string()); }
    let source_parent = source_path.parent().ok_or_else(|| "source has no owner directory".to_string())?;
    let owner_path = if source_parent.file_name().and_then(|name| name.to_str()) == Some(mutation_payload_facet.as_str()) {
        source_parent.parent().ok_or_else(|| "mutation payload facet has no semantic owner".to_string())?
    } else {
        source_parent
    };
    let owner = mutation_authority_relative(&workspace_root, owner_path)?;
    let parent = owner_path.parent().ok_or_else(|| "source owner has no collection parent".to_string())?;
    let (mutation_root, expected_semantic_kind) = if parent.file_name().and_then(|name| name.to_str()) == Some(mutation_collection.as_str()) {
        let root = mutation_authority_relative(&workspace_root, parent)?;
        if domain_owners.contains_key(&root) { return Err("flat source owner is not registered under its domain-operation root".to_string()); }
        (parent.to_path_buf(), None)
    } else {
        let root = parent.parent().ok_or_else(|| "source owner has no domain-operation root".to_string())?;
        if root.file_name().and_then(|name| name.to_str()) != Some(mutation_collection.as_str()) { return Err("source is neither a flat mutation owner nor one registered domain operation".to_string()); }
        let root_name = mutation_authority_relative(&workspace_root, root)?;
        let operations = domain_owners.get(&root_name).ok_or_else(|| "domain-operation root is not explicitly registered".to_string())?;
        let identity = operations.iter().find(|(registered, _)| registered == &owner).map(|(_, identity)| identity.clone()).ok_or_else(|| "source owner is not an exact registered domain operation".to_string())?;
        (root.to_path_buf(), Some(identity))
    };
    let descriptor_path = owner_path.join(descriptor_filename);
    mutation_authority_no_follow(&workspace_root, &descriptor_path, false)?;
    let descriptor = fs::read(&descriptor_path).map_err(|error| error.to_string())?;
    let authority = MutationSourceAuthority { workspace_root, mutation_root, owner, expected_semantic_kind, source_path, descriptor_path, taxonomy_path };
    parse_mutation_leaf_descriptor(&descriptor, &authority)?;
    Ok(authority)
}

fn mutation_aggregate_source_authority(source: &Path, compiler_cwd: &Path) -> Result<MutationAggregateSourceAuthority, String> {
    let common = mutation_authority_common(source, compiler_cwd)?;
    if common.source_path.file_name().and_then(|name| name.to_str()) != Some(common.source_filename.as_str()) { return Err("aggregate source is not the taxonomy canonical mutation primary".to_string()); }
    let mutation_root = common.source_path.parent().ok_or_else(|| "aggregate source has no mutation collection directory".to_string())?;
    mutation_authority_no_follow(&common.workspace_root, mutation_root, true)?;
    if mutation_root.file_name().and_then(|name| name.to_str()) != Some(common.mutation_collection.as_str()) { return Err("aggregate source is not directly inside the taxonomy mutation collection".to_string()); }
    let root_name = mutation_authority_relative(&common.workspace_root, mutation_root)?;
    let domain_operations = common.domain_owners.get(&root_name).cloned();
    let mut component_roots = Vec::new();
    if let Some(roots) = common.aggregate_sources.get(&root_name) {
        if domain_operations.is_some() { return Err("a mutation aggregate must declare either direct domain owners or component sources".to_string()); }
        for root in roots {
            mutation_authority_no_follow(&common.workspace_root, &common.workspace_root.join(root), true)?;
            component_roots.push((root.clone(), common.domain_owners.get(root).cloned()));
        }
    }
    Ok(MutationAggregateSourceAuthority {
        workspace_root: common.workspace_root,
        mutation_root: mutation_root.to_path_buf(),
        #[cfg(test)]
        source_path: common.source_path,
        taxonomy_path: common.taxonomy_path,
        mutation_payload_facet: common.mutation_payload_facet,
        source_filename: common.source_filename,
        descriptor_filename: common.descriptor_filename,
        domain_operations,
        component_roots,
    })
}

fn mutation_authority_aggregate_sources(taxonomy: &serde_json::Value, collection: &str) -> Result<BTreeMap<String, Vec<String>>, String> {
    let mut result = BTreeMap::new();
    let Some(registry) = taxonomy.get("mutationAggregateSources") else { return Ok(result); };
    let roots = registry.as_object().ok_or_else(|| "mutationAggregateSources must be an exact-root object".to_string())?;
    let valid = |root: &str| root.ends_with(&format!("/{collection}")) && root.split('/').all(|part| mutation_authority_owner_segment(part) && !part.eq_ignore_ascii_case("compose"));
    for (root, sources) in roots {
        if !valid(root) { return Err("mutationAggregateSources contains an unsafe or non-mutation aggregate root".to_string()); }
        let sources = sources.as_array().filter(|sources| !sources.is_empty()).ok_or_else(|| "mutationAggregateSources requires non-empty source arrays".to_string())?;
        let mut members = Vec::new();
        for source in sources {
            let source = source.as_str().filter(|source| valid(source)).ok_or_else(|| "mutationAggregateSources contains an unsafe or non-mutation component source".to_string())?;
            if members.iter().any(|member| member == source) { return Err("mutationAggregateSources contains a duplicate component source".to_string()); }
            members.push(source.to_string());
        }
        result.insert(root.clone(), members);
    }
    Ok(result)
}

fn mutation_authority_domain_owners(taxonomy: &serde_json::Value, collection: &str) -> Result<BTreeMap<String, MutationDomainOperations>, String> {
    let mut result = BTreeMap::new();
    let Some(registry) = taxonomy.get("mutationDomainOwners") else { return Ok(result); };
    let roots = registry.as_object().ok_or_else(|| "mutationDomainOwners must be an exact-root object".to_string())?;
    for (root, domains) in roots {
        if !root.ends_with(&format!("/{collection}")) || root.split('/').any(|part| !mutation_authority_owner_segment(part)) { return Err("mutationDomainOwners contains an unsafe or non-mutation root".to_string()); }
        let domains = domains.as_object().filter(|value| !value.is_empty()).ok_or_else(|| "registered mutation root must declare non-empty domains".to_string())?;
        let mut owners = Vec::new();
        let mut identities = HashSet::new();
        for (domain, operations) in domains {
            if !mutation_authority_owner_segment(domain) { return Err("registered mutation domain must be one safe basename".to_string()); }
            let operations = operations.as_object().filter(|value| !value.is_empty()).ok_or_else(|| "registered domain must declare non-empty operations".to_string())?;
            for (operation, identity) in operations {
                if !mutation_authority_owner_segment(operation) { return Err("registered mutation operation must be one safe basename".to_string()); }
                let identity = identity.as_str().filter(|value| mutation_leaf_kebab(value)).ok_or_else(|| "registered mutation identity must be full kebab-case".to_string())?;
                if !identities.insert(identity.to_string()) { return Err("registered mutation root has a duplicate semantic identity".to_string()); }
                owners.push((format!("{root}/{domain}/{operation}"), identity.to_string()));
            }
        }
        result.insert(root.clone(), owners);
    }
    Ok(result)
}

fn mutation_authority_owner_segment(value: &str) -> bool {
    !value.is_empty() && value != "." && value != ".." && !value.eq_ignore_ascii_case("compose") && !value.chars().any(|character| character.is_control() || matches!(character, '/' | '\\' | ':' | '*' | '?' | '{' | '}' | '\u{2028}' | '\u{2029}'))
}

fn mutation_authority_normalize(source: &Path, compiler_cwd: &Path) -> Result<PathBuf, String> {
    if !compiler_cwd.is_absolute() { return Err("compiler cwd is not absolute".to_string()); }
    mutation_authority_raw_lexical(compiler_cwd)?;
    let input = if source.is_absolute() { source.to_path_buf() } else { compiler_cwd.join(source) };
    mutation_authority_raw_no_follow(&input)?;
    let mut normalized = PathBuf::new();
    for component in input.components() {
        if let Component::Normal(segment) = component {
            if segment.to_str().is_some_and(|value| value.eq_ignore_ascii_case("compose")) { return Err("opaque compose path rejected before I/O".to_string()); }
        }
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {},
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir => { if !normalized.pop() { return Err("source escapes filesystem root".to_string()); } },
        }
    }
    if !normalized.is_absolute() { return Err("source is not absolute after compiler cwd resolution".to_string()); }
    Ok(normalized)
}

fn mutation_authority_raw_no_follow(path: &Path) -> Result<(), String> {
    if !path.is_absolute() { return Err("raw source path is not absolute".to_string()); }
    mutation_authority_raw_lexical(path)?;
    let mut current = PathBuf::new();
    let components: Vec<Component<'_>> = path.components().collect();
    for (index, component) in components.iter().enumerate() {
        match component {
            Component::Prefix(prefix) => current.push(prefix.as_os_str()),
            Component::RootDir => { current.push(component.as_os_str()); let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?; if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() { return Err("raw source root is not a regular directory".to_string()); } },
            Component::CurDir => {},
            Component::Normal(segment) => {
                current.push(segment);
                let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
                if metadata.file_type().is_symlink() { return Err("raw source symlink component rejected before normalization".to_string()); }
                if components[index + 1..].iter().any(|next| matches!(next, Component::Normal(_) | Component::ParentDir)) && !metadata.file_type().is_dir() { return Err("raw source intermediate component is not a directory".to_string()); }
                if !components[index + 1..].iter().any(|next| matches!(next, Component::Normal(_) | Component::ParentDir)) && !metadata.file_type().is_file() { return Err("raw source terminal component is not a regular file".to_string()); }
            },
            Component::ParentDir => { if !current.pop() { return Err("source escapes filesystem root".to_string()); } },
        }
    }
    Ok(())
}

fn mutation_authority_raw_lexical(path: &Path) -> Result<(), String> {
    for component in path.components() {
        if let Component::Normal(segment) = component {
            let segment = segment.to_str().ok_or_else(|| "raw source path component is not UTF-8".to_string())?;
            if segment.contains('\0') || segment.contains('\\') { return Err("raw source path component is not a portable owner segment".to_string()); }
            if segment.eq_ignore_ascii_case("compose") { return Err("opaque compose path rejected before I/O".to_string()); }
        }
    }
    Ok(())
}

fn mutation_authority_workspace_root(source: &Path) -> Result<PathBuf, String> {
    for ancestor in source.parent().into_iter().flat_map(Path::ancestors) {
        let nx = ancestor.join("nx.json");
        let project = ancestor.join("📋️project.json");
        match mutation_authority_node(&nx) {
            "missing" => continue,
            "symlink" => return Err("workspace nx.json marker is a symlink".to_string()),
            "file" => {},
            _ => return Err("workspace nx.json marker is not a regular file".to_string()),
        }
        match mutation_authority_node(&project) {
            "file" => return Ok(ancestor.to_path_buf()),
            "missing" => return Err("workspace nx.json marker lacks paired 📋️project.json".to_string()),
            "symlink" => return Err("workspace 📋️project.json marker is a symlink".to_string()),
            _ => return Err("workspace 📋️project.json marker is not a regular file".to_string()),
        }
    }
    Err("source has no exact nx.json and 📋️project.json workspace pair".to_string())
}

fn mutation_authority_node(path: &Path) -> &'static str {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => "symlink",
        Ok(metadata) if metadata.file_type().is_file() => "file",
        Ok(_) => "other",
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => "missing",
        Err(_) => "other",
    }
}

fn mutation_authority_no_follow(root: &Path, target: &Path, directory: bool) -> Result<(), String> {
    let relative = target.strip_prefix(root).map_err(|_| "path escapes workspace root".to_string())?;
    let mut current = root.to_path_buf();
    let root_metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
    if root_metadata.file_type().is_symlink() || !root_metadata.file_type().is_dir() { return Err("workspace root is not a regular directory".to_string()); }
    for component in relative.components() {
        let Component::Normal(segment) = component else { return Err("path is not lexically normalized".to_string()); };
        if segment.to_str().is_some_and(|value| value.eq_ignore_ascii_case("compose")) { return Err("opaque compose path rejected before I/O".to_string()); }
        current.push(segment);
        let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
        if metadata.file_type().is_symlink() { return Err("symlink path component rejected".to_string()); }
    }
    let metadata = fs::symlink_metadata(target).map_err(|error| error.to_string())?;
    if directory && !metadata.file_type().is_dir() { return Err("expected regular directory".to_string()); }
    if !directory && !metadata.file_type().is_file() { return Err("expected regular file".to_string()); }
    Ok(())
}

fn mutation_authority_locator(root: &Path, locator: &str) -> Result<PathBuf, String> {
    if locator.is_empty() || locator.contains('\0') || locator.contains('\\') || locator.starts_with('/') || locator.as_bytes().get(1) == Some(&b':') { return Err("mutation authority locator is not a normalized repository-relative path".to_string()); }
    let mut target = root.to_path_buf();
    for segment in locator.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." || segment.eq_ignore_ascii_case("compose") { return Err("mutation authority locator has a rejected path component".to_string()); }
        target.push(segment);
    }
    Ok(target)
}

fn mutation_authority_segment(authority: &serde_json::Value, key: &str) -> Result<String, String> {
    authority.get(key).and_then(serde_json::Value::as_str).filter(|value| mutation_authority_owner_segment(value)).map(str::to_string).ok_or_else(|| format!("mutation authority projection {key} is not one portable path segment"))
}

fn mutation_authority_relative(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path.strip_prefix(root).map_err(|_| "owner escapes workspace root".to_string())?;
    let mut segments = Vec::new();
    for component in relative.components() {
        let Component::Normal(segment) = component else { return Err("owner is not normalized".to_string()); };
        let segment = segment.to_str().ok_or_else(|| "owner is not UTF-8".to_string())?;
        if segment.contains('\0') || segment.contains('\\') || segment.eq_ignore_ascii_case("compose") { return Err("owner has rejected cross-platform path component".to_string()); }
        segments.push(segment);
    }
    if segments.is_empty() { return Err("owner is workspace root".to_string()); }
    Ok(segments.join("/"))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-source-authority/🦀️.rs"]
mod mutation_source_authority_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-aggregate-source-authority/🦀️.rs"]
mod mutation_aggregate_source_authority_tests;
//#endregion 🔖️MutationSourceAuthority

//#region 🔣️MutationLeafJson
#[derive(Debug, PartialEq, Eq)]
enum MutationLeafInvertibility { SelfInvertible, ExplicitMutation, Plan, NonInvertible }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafDiffParticipation { Detect, ApplyOnly, Plan, None }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafOutcomeClass { Applied, NoOp, Empty, Disjoint, Rejected }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafComposition { Atomic, Composite }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafLanguageSurface { Rust, Typescript, Graphql, Protobuf, JsonSchema, Text, Binary }

#[derive(Debug, PartialEq, Eq)]
struct MutationLeafJson {
    schema_version: u32,
    owner: String,
    semantic_kind: String,
    display_name: String,
    emoji: String,
    aggregate_variant: String,
    payload_schema: String,
    text_opcode: Option<String>,
    binary_tag: Option<u32>,
    invertibility: MutationLeafInvertibility,
    diff_participation: MutationLeafDiffParticipation,
    outcome_classes: Vec<MutationLeafOutcomeClass>,
    composition: MutationLeafComposition,
    required_language_surfaces: Vec<MutationLeafLanguageSurface>,
}

const MUTATION_LEAF_DESCRIPTOR_KEYS: [&str; 14] = ["schemaVersion", "owner", "semanticKind", "displayName", "emoji", "aggregateVariant", "payloadSchema", "textOpcode", "binaryTag", "invertibility", "diffParticipation", "outcomeClasses", "composition", "requiredLanguageSurfaces"];

fn parse_mutation_leaf_descriptor(raw: &[u8], authority: &MutationSourceAuthority) -> Result<MutationLeafJson, String> {
    mutation_leaf_reject_duplicate_keys(raw)?;
    let value: serde_json::Value = serde_json::from_slice(raw).map_err(|error| format!("malformed mutation descriptor JSON: {error}"))?;
    let object = value.as_object().ok_or_else(|| "mutation descriptor must be an object".to_string())?;
    if object.len() != MUTATION_LEAF_DESCRIPTOR_KEYS.len() || MUTATION_LEAF_DESCRIPTOR_KEYS.iter().any(|key| !object.contains_key(*key)) || object.keys().any(|key| !MUTATION_LEAF_DESCRIPTOR_KEYS.contains(&key.as_str())) { return Err("mutation descriptor must contain exactly the fourteen schema fields".to_string()); }
    let string = |key| mutation_leaf_string(object.get(key).unwrap(), key);
    let schema_version = mutation_leaf_u32(object.get("schemaVersion").unwrap(), "schemaVersion")?;
    if schema_version != 1 { return Err("schemaVersion must equal 1".to_string()); }
    let owner = string("owner")?;
    if owner != authority.owner { return Err("descriptor owner does not exactly match source owner".to_string()); }
    let semantic_kind = string("semanticKind")?;
    if !mutation_leaf_kebab(&semantic_kind) { return Err("semanticKind must be lowercase kebab-case".to_string()); }
    if authority.expected_semantic_kind.as_ref().is_some_and(|expected| expected != &semantic_kind) { return Err("descriptor semanticKind does not match its exact registered domain owner".to_string()); }
    let display_name = string("displayName")?;
    let emoji = string("emoji")?;
    let aggregate_variant = string("aggregateVariant")?;
    if !mutation_leaf_pascal(&aggregate_variant) { return Err("aggregateVariant must be ASCII PascalCase".to_string()); }
    let payload_schema = string("payloadSchema")?;
    let text_opcode = match object.get("textOpcode").unwrap() { serde_json::Value::Null => None, value => { let value = mutation_leaf_string(value, "textOpcode")?; if !mutation_leaf_kebab(&value) { return Err("textOpcode must be lowercase kebab-case or null".to_string()); } Some(value) } };
    let binary_tag = match object.get("binaryTag").unwrap() { serde_json::Value::Null => None, value => Some(mutation_leaf_u32(value, "binaryTag")?) };
    let invertibility = match string("invertibility")?.as_str() { "self" => MutationLeafInvertibility::SelfInvertible, "explicit-mutation" => MutationLeafInvertibility::ExplicitMutation, "plan" => MutationLeafInvertibility::Plan, "non-invertible" => MutationLeafInvertibility::NonInvertible, _ => return Err("invertibility is not a schema enum value".to_string()) };
    let diff_participation = match string("diffParticipation")?.as_str() { "detect" => MutationLeafDiffParticipation::Detect, "apply-only" => MutationLeafDiffParticipation::ApplyOnly, "plan" => MutationLeafDiffParticipation::Plan, "none" => MutationLeafDiffParticipation::None, _ => return Err("diffParticipation is not a schema enum value".to_string()) };
    let outcome_classes = mutation_leaf_outcomes(object.get("outcomeClasses").unwrap())?;
    let composition = match string("composition")?.as_str() { "atomic" => MutationLeafComposition::Atomic, "composite" => MutationLeafComposition::Composite, _ => return Err("composition is not a schema enum value".to_string()) };
    let required_language_surfaces = mutation_leaf_surfaces(object.get("requiredLanguageSurfaces").unwrap())?;
    Ok(MutationLeafJson { schema_version, owner, semantic_kind, display_name, emoji, aggregate_variant, payload_schema, text_opcode, binary_tag, invertibility, diff_participation, outcome_classes, composition, required_language_surfaces })
}

fn mutation_leaf_string(value: &serde_json::Value, key: &str) -> Result<String, String> { value.as_str().filter(|value| !value.is_empty()).map(str::to_owned).ok_or_else(|| format!("{key} must be a nonempty string")) }

fn mutation_leaf_u32(value: &serde_json::Value, key: &str) -> Result<u32, String> {
    let number = value.as_f64().ok_or_else(|| format!("{key} must be an integer"))?;
    if !number.is_finite() || number.fract() != 0.0 || number < 0.0 || number > u32::MAX as f64 { return Err(format!("{key} must be a u32 integer")); }
    Ok(number as u32)
}

/// 🔤️ Lowercase kebab-case with at least one hyphen — a mutation kind is always multi-word.
///
/// 🧭️This mirrors `mutation_leaf_descriptor_kebab` in the kernel
/// (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`), whose final expression IS
/// `hyphen`: a kind without one is rejected there no matter what this copy says. The two must agree.
///
/// 🐛️This once dropped the hyphen clause, on the reasoning that "kebab-case includes a single word"
/// and that no consumer splits the kind on `-`. Both are true and both are beside the point — the
/// kernel is the authority, and relaxing only this copy made single-word kinds pass the derive and
/// then panic in const-eval at `validate_mutation_leaf_source`, which reports every field failure
/// through one message ("Mutations leaf source must match its aggregate workspace and direct owner")
/// and so pointed nowhere near the kind. `✳️drawing`'s `rotate`/`scale`/`group`/`ungroup`/`flatten`/
/// `unflatten` were renamed to `rotate-node`/`scale-node`/`group-nodes`/… instead; the VERB stays the
/// single word, which is a separate descriptor field and is what `APPROVED_VERBS` gates.
fn mutation_leaf_kebab(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty() && bytes[0].is_ascii_lowercase() && bytes.contains(&b'-') && bytes.split(|byte| *byte == b'-').all(|part| !part.is_empty() && part.iter().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

fn mutation_leaf_pascal(value: &str) -> bool { let bytes = value.as_bytes(); !bytes.is_empty() && bytes[0].is_ascii_uppercase() && bytes.iter().all(|byte| byte.is_ascii_alphanumeric()) }

fn mutation_leaf_outcomes(value: &serde_json::Value) -> Result<Vec<MutationLeafOutcomeClass>, String> {
    let values = value.as_array().filter(|values| !values.is_empty()).ok_or_else(|| "outcomeClasses must be a nonempty array".to_string())?;
    let mut seen = HashSet::new();
    values.iter().map(|value| { let value = mutation_leaf_string(value, "outcomeClasses")?; if !seen.insert(value.clone()) { return Err("outcomeClasses must not contain duplicates".to_string()); } match value.as_str() { "applied" => Ok(MutationLeafOutcomeClass::Applied), "no-op" => Ok(MutationLeafOutcomeClass::NoOp), "empty" => Ok(MutationLeafOutcomeClass::Empty), "disjoint" => Ok(MutationLeafOutcomeClass::Disjoint), "rejected" => Ok(MutationLeafOutcomeClass::Rejected), _ => Err("outcomeClasses contains a non-schema enum value".to_string()) } }).collect()
}

fn mutation_leaf_surfaces(value: &serde_json::Value) -> Result<Vec<MutationLeafLanguageSurface>, String> {
    let values = value.as_array().filter(|values| !values.is_empty()).ok_or_else(|| "requiredLanguageSurfaces must be a nonempty array".to_string())?;
    let mut seen = HashSet::new();
    let surfaces: Vec<_> = values.iter().map(|value| { let value = mutation_leaf_string(value, "requiredLanguageSurfaces")?; if !seen.insert(value.clone()) { return Err("requiredLanguageSurfaces must not contain duplicates".to_string()); } match value.as_str() { "rust" => Ok(MutationLeafLanguageSurface::Rust), "typescript" => Ok(MutationLeafLanguageSurface::Typescript), "graphql" => Ok(MutationLeafLanguageSurface::Graphql), "protobuf" => Ok(MutationLeafLanguageSurface::Protobuf), "json-schema" => Ok(MutationLeafLanguageSurface::JsonSchema), "text" => Ok(MutationLeafLanguageSurface::Text), "binary" => Ok(MutationLeafLanguageSurface::Binary), _ => Err("requiredLanguageSurfaces contains a non-schema enum value".to_string()) } }).collect::<Result<_, _>>()?;
    if !surfaces.iter().any(|surface| matches!(surface, MutationLeafLanguageSurface::Rust)) { return Err("requiredLanguageSurfaces must contain rust".to_string()); }
    Ok(surfaces)
}

fn mutation_leaf_reject_duplicate_keys(raw: &[u8]) -> Result<(), String> {
    let mut index = mutation_leaf_skip_ws(raw, 0);
    if raw.get(index) != Some(&b'{') { return Err("mutation descriptor must be a JSON object".to_string()); }
    index += 1;
    let mut keys = HashSet::new();
    loop {
        index = mutation_leaf_skip_ws(raw, index);
        if raw.get(index) == Some(&b'}') { return Ok(()); }
        let key_start = index;
        index = mutation_leaf_string_end(raw, index).ok_or_else(|| "malformed mutation descriptor JSON key".to_string())?;
        let key: String = serde_json::from_slice(&raw[key_start..index]).map_err(|_| "malformed mutation descriptor JSON key".to_string())?;
        if !keys.insert(key) { return Err("mutation descriptor has a duplicate key".to_string()); }
        index = mutation_leaf_skip_ws(raw, index);
        if raw.get(index) != Some(&b':') { return Err("malformed mutation descriptor JSON key separator".to_string()); }
        index = mutation_leaf_json_value_end(raw, mutation_leaf_skip_ws(raw, index + 1)).ok_or_else(|| "malformed mutation descriptor JSON value".to_string())?;
        index = mutation_leaf_skip_ws(raw, index);
        match raw.get(index) { Some(b',') => index += 1, Some(b'}') => return Ok(()), _ => return Err("malformed mutation descriptor JSON object".to_string()) }
    }
}

fn mutation_leaf_skip_ws(raw: &[u8], mut index: usize) -> usize { while raw.get(index).is_some_and(|byte| byte.is_ascii_whitespace()) { index += 1; } index }
fn mutation_leaf_string_end(raw: &[u8], mut index: usize) -> Option<usize> { if raw.get(index) != Some(&b'\"') { return None; } index += 1; while let Some(byte) = raw.get(index) { match byte { b'\"' => return Some(index + 1), b'\\' => index += 2, 0..=0x1f => return None, _ => index += 1 } } None }
fn mutation_leaf_json_value_end(raw: &[u8], index: usize) -> Option<usize> {
    match raw.get(index)? { b'\"' => mutation_leaf_string_end(raw, index), b'{' => mutation_leaf_balanced_end(raw, index, b'{', b'}'), b'[' => mutation_leaf_balanced_end(raw, index, b'[', b']'), _ => { let end = raw[index..].iter().position(|byte| matches!(*byte, b',' | b'}' | b']') || byte.is_ascii_whitespace()).map_or(raw.len(), |offset| index + offset); (end > index).then_some(end) } }
}
fn mutation_leaf_balanced_end(raw: &[u8], mut index: usize, open: u8, close: u8) -> Option<usize> { let mut depth = 0usize; while let Some(byte) = raw.get(index) { if *byte == b'\"' { index = mutation_leaf_string_end(raw, index)?; continue; }
        if *byte == open { depth += 1; } else if *byte == close { depth -= 1; if depth == 0 { return Some(index + 1); } } index += 1; } None }

fn emit_mutation_leaf_descriptor(contract: &syn::Path, descriptor: &MutationLeafJson) -> proc_macro2::TokenStream {
    let schema_version = descriptor.schema_version; let owner = &descriptor.owner; let semantic_kind = &descriptor.semantic_kind; let display_name = &descriptor.display_name; let emoji = &descriptor.emoji; let aggregate_variant = &descriptor.aggregate_variant; let payload_schema = &descriptor.payload_schema;
    let text_opcode = descriptor.text_opcode.as_ref().map_or_else(|| quote!(::core::option::Option::None), |value| quote!(::core::option::Option::Some(#value))); let binary_tag = descriptor.binary_tag.map_or_else(|| quote!(::core::option::Option::None), |value| quote!(::core::option::Option::Some(#value)));
    let invertibility = match &descriptor.invertibility { MutationLeafInvertibility::SelfInvertible => quote!(#contract::MutationInvertibility::SelfInvertible), MutationLeafInvertibility::ExplicitMutation => quote!(#contract::MutationInvertibility::ExplicitMutation), MutationLeafInvertibility::Plan => quote!(#contract::MutationInvertibility::Plan), MutationLeafInvertibility::NonInvertible => quote!(#contract::MutationInvertibility::NonInvertible) };
    let diff_participation = match &descriptor.diff_participation { MutationLeafDiffParticipation::Detect => quote!(#contract::MutationDiffParticipation::Detect), MutationLeafDiffParticipation::ApplyOnly => quote!(#contract::MutationDiffParticipation::ApplyOnly), MutationLeafDiffParticipation::Plan => quote!(#contract::MutationDiffParticipation::Plan), MutationLeafDiffParticipation::None => quote!(#contract::MutationDiffParticipation::None) };
    let outcome_classes = descriptor.outcome_classes.iter().map(|value| match value { MutationLeafOutcomeClass::Applied => quote!(#contract::MutationOutcomeClass::Applied), MutationLeafOutcomeClass::NoOp => quote!(#contract::MutationOutcomeClass::NoOp), MutationLeafOutcomeClass::Empty => quote!(#contract::MutationOutcomeClass::Empty), MutationLeafOutcomeClass::Disjoint => quote!(#contract::MutationOutcomeClass::Disjoint), MutationLeafOutcomeClass::Rejected => quote!(#contract::MutationOutcomeClass::Rejected) });
    let composition = match &descriptor.composition { MutationLeafComposition::Atomic => quote!(#contract::MutationComposition::Atomic), MutationLeafComposition::Composite => quote!(#contract::MutationComposition::Composite) };
    let required_language_surfaces = descriptor.required_language_surfaces.iter().map(|value| match value { MutationLeafLanguageSurface::Rust => quote!(#contract::MutationLanguageSurface::Rust), MutationLeafLanguageSurface::Typescript => quote!(#contract::MutationLanguageSurface::Typescript), MutationLeafLanguageSurface::Graphql => quote!(#contract::MutationLanguageSurface::Graphql), MutationLeafLanguageSurface::Protobuf => quote!(#contract::MutationLanguageSurface::Protobuf), MutationLeafLanguageSurface::JsonSchema => quote!(#contract::MutationLanguageSurface::JsonSchema), MutationLeafLanguageSurface::Text => quote!(#contract::MutationLanguageSurface::Text), MutationLeafLanguageSurface::Binary => quote!(#contract::MutationLanguageSurface::Binary) });
    quote!(#contract::MutationLeafDescriptor { schema_version: #schema_version, owner: #owner, semantic_kind: #semantic_kind, display_name: #display_name, emoji: #emoji, aggregate_variant: #aggregate_variant, payload_schema: #payload_schema, text_opcode: #text_opcode, binary_tag: #binary_tag, invertibility: #invertibility, diff_participation: #diff_participation, outcome_classes: &[#(#outcome_classes),*], composition: #composition, required_language_surfaces: &[#(#required_language_surfaces),*] })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-leaf-json/🦀️.rs"]
mod mutation_leaf_json_tests;
//#endregion 🔣️MutationLeafJson

//#region 🪪️MutationLeaf
#[derive(Debug)]
struct MutationLeafAttrs { contract: syn::Path, payload: Option<syn::Ident>, input_schema: Option<syn::Path> }

fn parse_mutation_leaf_attrs(input: &DeriveInput) -> syn::Result<MutationLeafAttrs> {
    let mut contract = None;
    let mut payload = None;
    let mut input_schema = None;
    let mut found = false;
    for attribute in &input.attrs {
        if !attribute.path().is_ident("mutation_leaf") { continue; }
        if found { return Err(syn::Error::new_spanned(attribute, "duplicate mutation_leaf attribute")); }
        found = true;
        if matches!(&attribute.meta, syn::Meta::Path(_)) { continue; }
        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("payload") {
                if payload.is_some() { return Err(meta.error("duplicate mutation_leaf payload")); }
                payload = Some(meta.value()?.parse::<syn::Ident>()?);
                return Ok(());
            }
            if meta.path.is_ident("input_schema") {
                if input_schema.is_some() { return Err(meta.error("duplicate mutation_leaf input_schema")); }
                input_schema = Some(meta.value()?.parse::<syn::Path>()?);
                return Ok(());
            }
            if !meta.path.is_ident("contract") { return Err(meta.error("unsupported mutation_leaf attribute")); }
            if contract.is_some() { return Err(meta.error("duplicate mutation_leaf contract")); }
            let path: syn::Path = meta.value()?.parse()?;
            if path.leading_colon.is_none() || path.segments.is_empty() || path.segments.iter().any(|segment| !matches!(segment.arguments, syn::PathArguments::None) || matches!(segment.ident.to_string().as_str(), "self" | "super" | "crate")) { return Err(meta.error("mutation_leaf contract must be an absolute non-generic Rust path without self, super, or crate segments")); }
            contract = Some(path);
            Ok(())
        })?;
    }
    if !found { return Err(syn::Error::new_spanned(input, "MutationLeaf requires #[mutation_leaf(contract = ::protocol)]")); }
    let contract = contract.ok_or_else(|| syn::Error::new_spanned(input, "MutationLeaf requires mutation_leaf contract"))?;
    if let Some(variant) = &payload {
        let Data::Enum(data) = &input.data else { return Err(syn::Error::new_spanned(variant, "mutation_leaf payload names a variant of an enum leaf")); };
        let wraps_one = data.variants.iter().find(|candidate| candidate.ident == *variant).is_some_and(|candidate| matches!(&candidate.fields, Fields::Unnamed(fields) if fields.unnamed.len() == 1));
        if !wraps_one { return Err(syn::Error::new_spanned(variant, "mutation_leaf payload names a variant that wraps exactly one payload")); }
        if let Some(path) = &input_schema { return Err(syn::Error::new_spanned(path, "mutation_leaf input_schema and payload are exclusive: a wrapped leaf's input schema is its payload variant's")); }
    }
    Ok(MutationLeafAttrs { contract, payload, input_schema })
}

fn mutation_leaf_portable_path(path: &Path) -> Result<String, String> { path.to_str().map(|path| path.replace('\\', "/")).filter(|path| !path.is_empty()).ok_or_else(|| "metadata path is not UTF-8".to_string()) }

/// 🧭️ Canonical path without the Windows verbatim prefix, so a canonical root and a canonical member share one prefix on every platform.
fn mutation_authority_canonical(path: &Path) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(path).map_err(|error| error.to_string())?;
    Ok(match canonical.to_str().and_then(|value| value.strip_prefix(r"\\?\")) { Some(plain) => PathBuf::from(plain), None => canonical })
}

fn mutation_authority_workspace_token(workspace_root: &Path, taxonomy_path: &Path) -> Result<[u8; 32], String> {
    let workspace_root = mutation_authority_canonical(workspace_root)?;
    let taxonomy_path = mutation_authority_canonical(taxonomy_path)?;
    let workspace = mutation_leaf_portable_path(&workspace_root)?;
    let taxonomy = mutation_authority_relative(&workspace_root, &taxonomy_path)?;
    let mut input = b"semio.mutation-source-provenance/v1\0".to_vec();
    for value in [workspace.as_bytes(), taxonomy.as_bytes()] { let length = u64::try_from(value.len()).map_err(|_| "metadata token component exceeds u64".to_string())?; input.extend_from_slice(&length.to_be_bytes()); input.extend_from_slice(value); }
    Ok(semio_framework_hash::Sha256::digest(&input))
}

fn mutation_leaf_workspace_token(authority: &MutationSourceAuthority) -> Result<[u8; 32], String> { mutation_authority_workspace_token(&authority.workspace_root, &authority.taxonomy_path) }

fn mutation_leaf_include_path(path: &Path) -> Result<String, String> { mutation_leaf_portable_path(path) }

pub fn expand_mutation_leaf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if matches!(input.data, Data::Union(_)) { return syn::Error::new_spanned(&input, "MutationLeaf does not support unions").to_compile_error().into(); }
    let attrs = match parse_mutation_leaf_attrs(&input) { Ok(attrs) => attrs, Err(error) => return error.to_compile_error().into() };
    let source = match input.ident.span().unwrap().local_file() { Some(source) => source, None => return syn::Error::new_spanned(&input, "MutationLeaf requires a local source file").to_compile_error().into() };
    let compiler_cwd = match std::env::current_dir() { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let authority = match mutation_source_authority(&source, &compiler_cwd) { Ok(authority) => authority, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf source authority failed: {error}")).to_compile_error().into() };
    let raw_descriptor = match fs::read(&authority.descriptor_path) { Ok(raw) => raw, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let descriptor = match parse_mutation_leaf_descriptor(&raw_descriptor, &authority) { Ok(descriptor) => descriptor, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf descriptor failed: {error}")).to_compile_error().into() };
    let workspace_token = match mutation_leaf_workspace_token(&authority) { Ok(token) => token, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf provenance failed: {error}")).to_compile_error().into() };
    let mutation_root = match mutation_authority_relative(&authority.workspace_root, &authority.mutation_root) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let source_path = match mutation_authority_relative(&authority.workspace_root, &authority.source_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let descriptor_path = match mutation_authority_relative(&authority.workspace_root, &authority.descriptor_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let taxonomy_path = match mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let payload_schema_path = match mutation_leaf_payload_schema_path(&authority, &descriptor.payload_schema) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf payload schema failed: {error}")).to_compile_error().into() };
    let inverse_rows = match mutation_leaf_inverse_rows(&payload_schema_path) { Ok(rows) => rows, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf x-semio-inverse-rows failed: {error}")).to_compile_error().into() };
    let referenced_documents = match mutation_leaf_referenced_documents(&payload_schema_path).and_then(|paths| paths.iter().map(|path| mutation_leaf_include_path(path)).collect::<Result<Vec<_>, _>>()) { Ok(paths) => paths, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf referenced schema documents failed: {error}")).to_compile_error().into() };
    let dependency_paths = [authority.taxonomy_path.clone(), authority.descriptor_path.clone(), payload_schema_path];
    let dependency_paths: Result<Vec<_>, _> = dependency_paths.iter().map(|path| mutation_leaf_include_path(path)).collect();
    let dependency_paths = match dependency_paths { Ok(paths) => paths, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let [taxonomy_dependency, descriptor_dependency, payload_schema_dependency]: [String; 3] = match dependency_paths.try_into() { Ok(paths) => paths, Err(_) => unreachable!() };
    let name = &input.ident;
    let contract = &attrs.contract;
    let editable = attrs.payload.as_ref().map(|variant| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            match self { Self::#variant(_) => ::core::option::Option::Some(<Self as #contract::MutationLeaf>::PAYLOAD_SCHEMA), _ => ::core::option::Option::None }
        }
        fn input_value(&self) -> #contract::DslValue {
            match self { Self::#variant(payload) => #contract::ToValue::to_value(payload), _ => #contract::ToValue::to_value(self) }
        }
        fn with_input_value(&self, value: #contract::DslValue) -> ::core::result::Result<Self, #contract::ValueError> {
            match self {
                Self::#variant(_) => #contract::FromValue::from_value(value).map(Self::#variant),
                _ => ::core::result::Result::Err(#contract::ValueError::new(#contract::ValueRefusalKind::InvalidValue,::std::format!("{} is editable only as {}", ::core::stringify!(#name), ::core::stringify!(#variant)))),
            }
        }
        fn from_input_value(value: #contract::DslValue) -> ::core::result::Result<Self, #contract::ValueError> {
            #contract::FromValue::from_value(value).map(Self::#variant)
        }
    });
    let instance_schema = attrs.input_schema.as_ref().map(|path| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            #path(self)
        }
    });
    let inverse_rows = mutation_leaf_inverse_rows_body(&inverse_rows, attrs.payload.as_ref());
    let owner = &authority.owner;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let descriptor = emit_mutation_leaf_descriptor(contract, &descriptor);
    let workspace_token = workspace_token.iter();
    quote! {
        const _: &str = ::core::include_str!(#taxonomy_dependency);
        const _: &str = ::core::include_str!(#descriptor_dependency);
        impl #impl_generics #contract::MutationLeaf for #name #ty_generics #where_clause {
            const DESCRIPTOR: #contract::MutationLeafDescriptor = #descriptor;
            const PROVENANCE: #contract::MutationSourceProvenance = #contract::MutationSourceProvenance { workspace_token: [#(#workspace_token),*], mutation_root: #mutation_root, owner: #owner, source_path: #source_path, descriptor_path: #descriptor_path, taxonomy_path: #taxonomy_path };
            const PAYLOAD_SCHEMA: &'static str = ::core::include_str!(#payload_schema_dependency);
            const PAYLOAD_SCHEMA_DOCUMENTS: &'static [&'static str] = &[#(::core::include_str!(#referenced_documents)),*];
            #editable
            #instance_schema
            fn inverse_rows(&self) -> usize {
                #inverse_rows
            }
        }
    }.into()
}

/// 🧾️ A leaf payload schema's `x-semio-inverse-rows` (design §20.5): `fixed` rows plus `perTarget` rows per item of each named
/// array field, or one `bounded` constant; absent, one row.
struct MutationLeafInverseRows {
    fixed: usize,
    per_target: Vec<(String, usize)>,
}

/// 📖️ Reads `x-semio-inverse-rows` from the root of the payload schema at `payload_schema`.
fn mutation_leaf_inverse_rows(payload_schema: &Path) -> Result<MutationLeafInverseRows, String> {
    let raw = fs::read(payload_schema).map_err(|error| error.to_string())?;
    let value: serde_json::Value = serde_json::from_slice(&raw).map_err(|error| format!("malformed payload schema: {error}"))?;
    let Some(rows) = value.get("x-semio-inverse-rows") else { return Ok(MutationLeafInverseRows { fixed: 1, per_target: Vec::new() }) };
    let object = rows.as_object().filter(|object| !object.is_empty()).ok_or_else(|| "x-semio-inverse-rows must be a nonempty object".to_string())?;
    if object.keys().any(|key| !["fixed", "perTarget", "bounded"].contains(&key.as_str())) { return Err("x-semio-inverse-rows admits only fixed, perTarget and bounded".to_string()); }
    if object.contains_key("bounded") && object.len() != 1 { return Err("x-semio-inverse-rows bounded excludes fixed and perTarget".to_string()); }
    let count = |key: &str| object.get(key).map(|value| value.as_u64().map(|count| count as usize).ok_or_else(|| format!("x-semio-inverse-rows {key} must be a nonnegative integer"))).transpose();
    let fixed = count("bounded")?.or(count("fixed")?).unwrap_or(0);
    let per_target = match object.get("perTarget") {
        None => Vec::new(),
        Some(fields) => fields
            .as_object()
            .filter(|fields| !fields.is_empty())
            .ok_or_else(|| "x-semio-inverse-rows perTarget must be a nonempty object".to_string())?
            .iter()
            .map(|(field, rows)| rows.as_u64().filter(|rows| *rows >= 1).map(|rows| (to_snake_ascii(field), rows as usize)).ok_or_else(|| format!("x-semio-inverse-rows perTarget {field} must be a positive integer")))
            .collect::<Result<_, _>>()?,
    };
    Ok(MutationLeafInverseRows { fixed, per_target })
}

/// 🐍️ The Rust field a camelCase payload property names.
fn to_snake_ascii(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 4);
    for character in value.chars() {
        if character.is_ascii_uppercase() {
            out.push('_');
            out.push(character.to_ascii_lowercase());
        } else {
            out.push(character);
        }
    }
    out
}

/// 🧮️ The generated `MutationLeaf::inverse_rows` body: the fixed rows plus each `perTarget` field's length times its rows, read
/// from the leaf itself or from its `payload` variant (any other variant declares the fixed rows).
fn mutation_leaf_inverse_rows_body(rows: &MutationLeafInverseRows, payload: Option<&syn::Ident>) -> proc_macro2::TokenStream {
    let fixed = proc_macro2::Literal::usize_unsuffixed(rows.fixed);
    if rows.per_target.is_empty() {
        return quote! { #fixed };
    }
    let terms = rows.per_target.iter().map(|(field, count)| {
        let field = syn::Ident::new(field, proc_macro2::Span::call_site());
        let count = proc_macro2::Literal::usize_unsuffixed(*count);
        quote! { + #count * payload.#field.len() }
    });
    match payload {
        None => quote! { let payload = self; #fixed #(#terms)* },
        Some(variant) => quote! { match self { Self::#variant(payload) => #fixed #(#terms)*, _ => #fixed } },
    }
}

/// 🧬️ The descriptor's `payloadSchema`, resolved beside the descriptor: a normalized relative path of portable
/// segments that stays inside the leaf and names a regular file reached without a symlink.
fn mutation_leaf_payload_schema_path(authority: &MutationSourceAuthority, payload_schema: &str) -> Result<PathBuf, String> {
    let leaf = authority.descriptor_path.parent().ok_or_else(|| "descriptor has no leaf directory".to_string())?;
    if payload_schema.starts_with('/') || payload_schema.contains('\\') || payload_schema.contains('\0') { return Err("payloadSchema is not a relative portable path".to_string()); }
    let mut path = leaf.to_path_buf();
    for segment in payload_schema.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." || segment.eq_ignore_ascii_case("compose") { return Err("payloadSchema has a rejected path segment".to_string()); }
        path.push(segment);
    }
    mutation_authority_no_follow(&authority.workspace_root, &path, false)?;
    Ok(path)
}

/// 🔗️ Every schema document the payload schema at `payload_schema` references by an absolute `$id` (fragment stripped),
/// transitively, among the `🧬️schema` JSON documents of its search tree ([`mutation_schema_search_root`]) — what the runtime
/// publishes beside the leaf (`MutationLeaf::PAYLOAD_SCHEMA_DOCUMENTS`), so an input that `$ref`s another facet of its scope
/// resolves. A reference the tree does not hold is left to the OS-wide registry, which framework scopes publish themselves.
/// Sorted, never the payload schema itself.
fn mutation_leaf_referenced_documents(payload_schema: &Path) -> Result<Vec<PathBuf>, String> {
    let index = mutation_schema_document_index(&mutation_schema_search_root(payload_schema));
    let (own, mut pending) = mutation_schema_document_references(payload_schema)?;
    let mut seen: HashSet<String> = own.into_iter().collect();
    let mut found = Vec::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id.clone()) {
            continue;
        }
        let Some(path) = index.get(&id) else { continue };
        pending.extend(mutation_schema_document_references(path)?.1);
        found.push(path.clone());
    }
    found.sort();
    Ok(found)
}

/// 🧭️ The tree a leaf's referenced documents are looked up in: its plugin (the directory under `🔌️plugins`, which bundles every
/// artifact a leaf may reference across), else its framework module (the nearest directory under `🔨️modules`), else the module
/// owning its `🧬️schema/🧬️mutations` root.
fn mutation_schema_search_root(payload_schema: &Path) -> PathBuf {
    let under = |parent: &str| payload_schema.ancestors().find(|ancestor| ancestor.parent().and_then(Path::file_name).is_some_and(|name| name == parent));
    let owner = payload_schema.ancestors().find(|ancestor| ancestor.file_name().is_some_and(|name| name == "🧬️schema") && ancestor.join("🧬️mutations").is_dir()).and_then(Path::parent);
    under("🔌️plugins").or_else(|| under("🔨️modules")).or(owner).or_else(|| payload_schema.parent()).unwrap_or(payload_schema).to_path_buf()
}

/// 🗂️ `$id` → path of every JSON document inside a `🧬️schema` directory of `root` (tests and build output skipped; a fixture
/// aggregate's schemas count), built once per root per compiler process.
fn mutation_schema_document_index(root: &Path) -> std::rc::Rc<BTreeMap<String, PathBuf>> {
    thread_local! {
        static INDEX: std::cell::RefCell<BTreeMap<PathBuf, std::rc::Rc<BTreeMap<String, PathBuf>>>> = const { std::cell::RefCell::new(BTreeMap::new()) };
    }
    fn walk(directory: &Path, schema: bool, index: &mut BTreeMap<String, PathBuf>) {
        let Ok(entries) = fs::read_dir(directory) else { return };
        for entry in entries.flatten() {
            let (path, name) = (entry.path(), entry.file_name().to_string_lossy().into_owned());
            let Ok(kind) = entry.file_type() else { continue };
            if kind.is_dir() && !name.starts_with('.') && !["🧪️tests", "target", "node_modules", "dist", "🗑️generated", "📦️packages"].contains(&name.as_str()) {
                walk(&path, schema || name == "🧬️schema", index);
            } else if schema && kind.is_file() && name.ends_with(".json") {
                let id = fs::read(&path).ok().and_then(|raw| serde_json::from_slice::<serde_json::Value>(&raw).ok()).and_then(|value| value.get("$id").and_then(serde_json::Value::as_str).map(str::to_string));
                if let Some(id) = id {
                    index.entry(id).or_insert(path);
                }
            }
        }
    }
    if let Some(index) = INDEX.with(|cache| cache.borrow().get(root).cloned()) {
        return index;
    }
    let mut index = BTreeMap::new();
    walk(root, root.components().any(|component| component.as_os_str() == "🧬️schema"), &mut index);
    let index = std::rc::Rc::new(index);
    INDEX.with(|cache| cache.borrow_mut().insert(root.to_path_buf(), index.clone()));
    index
}

/// 🔗️ A schema document's own `$id` and every absolute document id its `$ref`s name (fragments stripped).
fn mutation_schema_document_references(path: &Path) -> Result<(Option<String>, Vec<String>), String> {
    fn collect(value: &serde_json::Value, into: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(entries) => {
                for (key, value) in entries {
                    match (key.as_str(), value) {
                        ("$ref", serde_json::Value::String(reference)) if !reference.starts_with('#') => into.push(reference.split('#').next().unwrap_or_default().to_string()),
                        _ => collect(value, into),
                    }
                }
            }
            serde_json::Value::Array(items) => items.iter().for_each(|item| collect(item, into)),
            _ => {}
        }
    }
    let raw = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let document: serde_json::Value = serde_json::from_slice(&raw).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut references = Vec::new();
    collect(&document, &mut references);
    references.retain(|reference| !reference.is_empty());
    Ok((document.get("$id").and_then(serde_json::Value::as_str).map(str::to_string), references))
}
//#endregion 🪪️MutationLeaf

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-leaf-derive/🦀️.rs"]
mod mutation_leaf_derive_tests;

//#region 🔖️Attrs
#[derive(Default, Clone)]
struct ContainerAttrs {
    extension: Option<String>,
    id: Option<String>,
    keyword: Option<String>,
    lines_layout: bool,
    retire_with: Option<syn::Path>,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    key: Option<String>,
    positional: bool,
    list: bool,
    tuple: bool,
    statements: bool,
    block: bool,
    base64: bool,
    flatten: bool,
    table: bool,
    /// `#[dsl(unit = "GPa")]` — a scalar `f64`/`f32` field prints/parses as `Shape::Quantity`
    /// (glued unit suffix) instead of plain `Shape::Float`.
    unit: Option<String>,
    /// `#[dsl(angle = "deg")]` — same mechanism as `unit`, `Shape::Angle` instead.
    angle: Option<String>,
    /// `#[dsl(refs = "material")]` — a scalar `String`/`Option<String>` field prints/parses as
    /// `Shape::Ref(kind)` instead of plain `Shape::Text`.
    refs: Option<String>,
    /// `#[dsl(defines = "material")]` — the anchor side of `refs`: this field's `FieldSpec.defines`
    /// is set so `LanguageService::validate` knows which field, in a record of this kind, other
    /// records' `Shape::Ref("material")` fields are expected to resolve against.
    defines: Option<String>,
    /// `#[dsl(lang = "jack")]` — a scalar `String` field prints/parses as `Shape::Embed(lang)`
    /// (fenced verbatim in Document mode) instead of plain `Shape::Text`.
    lang: Option<String>,
    /// `#[dsl(lang_from = "language_id")]` — fence language from a sibling Text field at print/parse time.
    lang_from: Option<String>,
    /// `#[dsl(coord)]` — a `[f64; 3]` (or any `DslField` array) field prints/parses as
    /// `Shape::Coord(3)` (`@x,y,z`) instead of a bare comma tuple.
    coord: bool,
    /// `#[dsl(dir)]` — same mechanism as `coord`, `Shape::Dir` (`^x,y,z`) instead.
    dir: bool,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_container_attrs(input: &DeriveInput) -> ContainerAttrs {
    let mut out = ContainerAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("extension") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.extension = Some(value.value());
            } else if meta.path.is_ident("id") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.id = Some(value.value());
            } else if meta.path.is_ident("keyword") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.keyword = Some(value.value());
            } else if meta.path.is_ident("layout") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lines_layout = value.value() == "lines";
            } else if meta.path.is_ident("retire_with") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.retire_with = Some(value.parse()?);
            }
            Ok(())
        });
    }
    out
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_field_attrs(attrs: &[syn::Attribute]) -> FieldAttrs {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("key") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.key = Some(value.value());
            } else if meta.path.is_ident("positional") {
                out.positional = true;
            } else if meta.path.is_ident("list") {
                out.list = true;
            } else if meta.path.is_ident("tuple") {
                out.tuple = true;
            } else if meta.path.is_ident("statements") {
                out.statements = true;
            } else if meta.path.is_ident("block") {
                out.block = true;
            } else if meta.path.is_ident("base64") {
                out.base64 = true;
            } else if meta.path.is_ident("flatten") {
                out.flatten = true;
            } else if meta.path.is_ident("table") {
                out.table = true;
            } else if meta.path.is_ident("unit") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.unit = Some(value.value());
            } else if meta.path.is_ident("angle") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.angle = Some(value.value());
            } else if meta.path.is_ident("refs") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.refs = Some(value.value());
            } else if meta.path.is_ident("defines") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.defines = Some(value.value());
            } else if meta.path.is_ident("lang") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lang = Some(value.value());
            } else if meta.path.is_ident("lang_from") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lang_from = Some(value.value());
            } else if meta.path.is_ident("coord") {
                out.coord = true;
            } else if meta.path.is_ident("dir") {
                out.dir = true;
            }
            Ok(())
        });
    }
    out
}
//#endregion 🔖️Attrs

//#region 🔖️TypeShape
enum FieldKind {
    Scalar,
    OptionScalar(Box<Type>),
    VecList(Box<Type>),
    VecTuple(Box<Type>),
    VecStatements(Box<Type>),
    /// `#[dsl(statements, block)]` — same tagged-variant collection as `VecStatements`, but wrapped
    /// in `{ ... }` so it can sit anywhere in field order (not just as an unbounded trailing field).
    VecBlockStatements(Box<Type>),
    /// `BTreeMap<String, V>` — `V` must itself implement `DslField`; keys print sorted.
    MapField(Box<Type>),
    /// `#[dsl(statements)] Option<T>` — a "sum type" scalar field (`fill: Option<FillStyle>`,
    /// exactly one of several keyword-tagged variants, or none) rather than a collection. Reuses
    /// `Shape::Statements`/`DslVariants` at 0-or-1 length instead of a new shape: a record isn't
    /// allowed more than one *bare* `Statements` field, but two `Option<T>` fields of this kind can
    /// coexist because each is dispatched by its own field key (always paired with `#[dsl(block)]`
    /// in practice, since an un-blocked one would hit that same one-per-record limit).
    OptionStatements(Box<Type>),
    /// `#[dsl(statements)] Box<T>` (or bare `T`) — exactly one required tagged value (`layer:
    /// Box<DrawLayerNode>` on an `AddLayer` operation), the non-optional counterpart of
    /// `OptionStatements`: same `Shape::Statements` reuse, but errors if the count isn't exactly 1
    /// rather than treating 0 as `None`.
    RequiredStatements(Box<Type>),
    Bytes64,
    /// `#[dsl(table)] Vec<T>` (`T: DslRecord`) — Structure-of-Arrays columnar `Shape::Table`.
    /// `to_value`/`from_value` are identical to `VecList` (both produce `FieldValue::List(Vec<
    /// FieldValue::Record>)`) — only the `Shape` differs, so every binder/diff path downstream
    /// keeps working unchanged.
    VecTable(Box<Type>),
}

/// 🪆️ Strips `macro_rules!`-introduced invisible-delimiter `Type::Group` wrappers so a type
/// captured through a `:ty` metavariable — then re-emitted through another technology-local
/// declarative macro (e.g. an `entity_input!`-style struct-generating macro) before ever reaching
/// this derive — still structurally matches `Type::Path` here exactly like directly-written source.
/// Without this, `Option<T>`/`Vec<T>`/`Box<T>`/`BTreeMap<..>` fields declared through such a wrapping
/// macro silently fall through to plain `FieldKind::Scalar` instead of being classified as
/// optional/list/map, since the wrapper hides the outer `Path` segment from a bare `matches!`.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn strip_groups(ty: &Type) -> &Type {
    let mut ty = ty;
    while let Type::Group(group) = ty {
        ty = &group.elem;
    }
    ty
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn inner_of(ty: &Type, wrapper: &str) -> Option<Type> {
    let Type::Path(path) = strip_groups(ty) else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != wrapper {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    args.args.iter().find_map(|arg| match arg {
        syn::GenericArgument::Type(t) => Some(t.clone()),
        _ => None,
    })
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn is_vec_u8(ty: &Type) -> bool {
    inner_of(ty, "Vec").is_some_and(|inner| matches!(strip_groups(&inner), Type::Path(p) if p.path.is_ident("u8")))
}

/// 🗺️ Extracts `V` from `BTreeMap<String, V>` — `None` for any other type, including a
/// `BTreeMap` keyed by something other than `String` (the engine's `Shape::Map` is string-keyed
/// only, matching every hand-rolled `{ key=value }` grammar it replaces).
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn btreemap_string_value(ty: &Type) -> Option<Type> {
    let Type::Path(path) = strip_groups(ty) else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != "BTreeMap" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    let types: Vec<&Type> = args
        .args
        .iter()
        .filter_map(|arg| match arg {
            syn::GenericArgument::Type(t) => Some(t),
            _ => None,
        })
        .collect();
    let [key, value] = types.as_slice() else { return None };
    matches!(strip_groups(key), Type::Path(p) if p.path.is_ident("String")).then(|| (*value).clone())
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn classify_field(ty: &Type, attrs: &FieldAttrs) -> (FieldKind, Type) {
    if let Some(inner) = inner_of(ty, "Option") {
        if attrs.statements {
            return (FieldKind::OptionStatements(Box::new(inner.clone())), inner);
        }
        return (FieldKind::OptionScalar(Box::new(inner.clone())), inner);
    }
    if attrs.base64 && is_vec_u8(ty) {
        return (FieldKind::Bytes64, ty.clone());
    }
    if attrs.statements {
        if let Some(inner) = inner_of(ty, "Box") {
            return (FieldKind::RequiredStatements(Box::new(inner.clone())), inner);
        }
    }
    if let Some(value_ty) = btreemap_string_value(ty) {
        return (FieldKind::MapField(Box::new(value_ty.clone())), value_ty);
    }
    if let Some(inner) = inner_of(ty, "Vec") {
        if attrs.statements {
            let kind = if attrs.block { FieldKind::VecBlockStatements(Box::new(inner.clone())) } else { FieldKind::VecStatements(Box::new(inner.clone())) };
            return (kind, inner);
        }
        if attrs.tuple {
            return (FieldKind::VecTuple(Box::new(inner.clone())), inner);
        }
        if attrs.table {
            return (FieldKind::VecTable(Box::new(inner.clone())), inner);
        }
        return (FieldKind::VecList(Box::new(inner.clone())), inner);
    }
    (FieldKind::Scalar, ty.clone())
}
//#endregion 🔖️TypeShape

//#region 🔖️RecordCodegen
struct FieldPlan {
    ident: syn::Ident,
    id: u16,
    key: String,
    positional: Option<u16>,
    optional: bool,
    kind: FieldKind,
    elem_ty: Type,
    /// `#[dsl(block)]` on a field whose `FieldKind` doesn't already imply its own `{ }` wrapping
    /// (`VecBlockStatements` handles that itself) — wraps whatever shape that kind would otherwise
    /// produce in `Shape::Block`, e.g. a single nested `#[derive(DslRecord)]` field printed as a
    /// bare `camera { x=0 y=0 zoom=1 }` line instead of a `camera=...` attribute.
    block: bool,
    /// `#[dsl(unit = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    unit: Option<String>,
    /// `#[dsl(angle = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    angle: Option<String>,
    /// `#[dsl(refs = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    refs: Option<String>,
    /// `#[dsl(defines = "...")]` — sets `FieldSpec.defines`, independent of `Shape`.
    defines: Option<String>,
    /// `#[dsl(lang = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    lang: Option<String>,
    lang_from: Option<String>,
    /// `#[dsl(coord)]`, only meaningful for `FieldKind::Scalar`/`OptionScalar` on an array type.
    coord: bool,
    /// `#[dsl(dir)]`, ditto.
    dir: bool,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn plan_fields(fields: &Fields) -> Vec<FieldPlan> {
    let mut positional_counter: u16 = 0;
    let mut out = Vec::new();
    for (index, field) in fields.iter().enumerate() {
        let attrs = parse_field_attrs(&field.attrs);
        let ident = field.ident.clone().expect("dsl_derive only supports named fields");
        let (kind, elem_ty) = classify_field(&field.ty, &attrs);
        let key = attrs.key.clone().unwrap_or_else(|| to_kebab(&ident.to_string()));
        let optional = matches!(kind, FieldKind::OptionScalar(_) | FieldKind::OptionStatements(_));
        let positional = if attrs.positional {
            let p = positional_counter;
            positional_counter += 1;
            Some(p)
        } else {
            None
        };
        let block = attrs.block && !matches!(kind, FieldKind::VecBlockStatements(_));
        out.push(FieldPlan {
            ident,
            id: index as u16,
            key,
            positional,
            optional,
            kind,
            elem_ty,
            block,
            unit: attrs.unit.clone(),
            angle: attrs.angle.clone(),
            refs: attrs.refs.clone(),
            defines: attrs.defines.clone(),
            lang: attrs.lang.clone(),
            lang_from: attrs.lang_from.clone(),
            coord: attrs.coord,
            dir: attrs.dir,
        });
    }
    out
}

/// 🏗️ Builds the three code fragments shared by `DslRecord`/`DslArtifact`/`DslOps` variant
/// bodies: the `RecordSpec` field-spec expressions, the struct→`RecordValue` conversion, and the
/// `RecordValue`→struct conversion.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn record_codegen(fields: &Fields) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<syn::Ident>) {
    let plans = plan_fields(fields);
    let mut spec_exprs = Vec::new();
    let mut to_value_stmts = Vec::new();
    let mut from_value_stmts = Vec::new();
    let mut field_idents = Vec::new();

    for plan in &plans {
        let FieldPlan { ident, id, key, positional, optional, kind, elem_ty, block, unit, angle, refs, defines, lang, lang_from, coord, dir } = plan;
        // A `#[dsl(unit = "...")]`/`#[dsl(angle = "...")]` scalar field's Shape is resolved at
        // spec-build time via `dsl::__rt::unit_for_derive` — same lazy-per-call pattern every other
        // `fn() -> RecordSpec`-backed Shape in this engine already uses, so an unknown unit symbol
        // surfaces as a panic the first time the generated spec runs (caught by that app's own
        // RecordSpec-law tests), never silently.
        let quantity_shape_override: Option<proc_macro2::TokenStream> = if let Some(symbol) = unit {
            Some(quote! { ::dsl::Shape::Quantity(::dsl::__rt::unit_for_derive(#symbol)) })
        } else if let Some(symbol) = angle {
            Some(quote! { ::dsl::Shape::Angle(::dsl::__rt::unit_for_derive(#symbol)) })
        } else if let Some(kind) = refs {
            Some(quote! { ::dsl::Shape::Ref(#kind) })
        } else if let Some(from) = lang_from {
            let embed_lang_key = plans.iter().find(|p| p.ident == from.as_str()).map_or_else(|| to_kebab(from), |p| p.key.clone());
            Some(quote! { ::dsl::Shape::EmbedFrom(#embed_lang_key) })
        } else if let Some(l) = lang {
            Some(quote! { ::dsl::Shape::Embed(#l) })
        } else if *coord {
            Some(quote! { ::dsl::Shape::Coord(3) })
        } else if *dir {
            Some(quote! { ::dsl::Shape::Dir })
        } else {
            None
        };
        let defines_expr = match defines {
            Some(kind) => quote! { .defines(#kind) },
            None => quote! {},
        };
        field_idents.push(ident.clone());
        let pos_expr = match positional {
            Some(p) => quote! { .positional(#p as u8) },
            None => quote! {},
        };
        let opt_expr = if *optional {
            quote! { .optional() }
        } else {
            quote! {}
        };

        // `DslField::shape`/`DslVariants::variants` are E4 (sync, fn-pointer transitivity — see
        // R9), so `shape_expr` never needs `.await`. `DslField::to_value`/`from_value` and
        // `DslVariants::to_named_record`/`from_named_record` stay `async`, so every value-level
        // call below is `.await`ed; a `Vec`/`Map` field can't `.await` per-element inside
        // `Iterator::map` (R10 residue shape 1), so those go through a sequential loop instead of
        // `.map().collect()`.
        let (shape_expr, to_value_expr, from_value_expr): (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) = match kind {
            FieldKind::Scalar => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#elem_ty as ::dsl::DslField>::shape() }),
                quote! { ::dsl::DslField::to_value(&self.#ident) },
                quote! { <#elem_ty as ::dsl::DslField>::from_value(value).map_err(::dsl::__rt::field_error)? },
            ),
            FieldKind::Bytes64 => (
                quote! { ::dsl::Shape::Bytes64 },
                quote! { ::dsl::FieldValue::Bytes64(self.#ident.clone()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Bytes64(bytes) => bytes.clone(),
                        other => return Err(::dsl::__rt::field_error(format!("expected Bytes64, found {other:?}"))),
                    }
                },
            ),
            FieldKind::OptionScalar(inner) => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#inner as ::dsl::DslField>::shape() }),
                quote! {
                    match &self.#ident {
                        Some(v) => ::dsl::DslField::to_value(v),
                        None => ::dsl::FieldValue::Absent,
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Absent => None,
                        other => Some(<#inner as ::dsl::DslField>::from_value(other).map_err(::dsl::__rt::field_error)?),
                    }
                },
            ),
            FieldKind::VecList(inner) => (
                quote! { ::dsl::Shape::List(Box::new(<#inner as ::dsl::DslField>::shape())) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::List(__items)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::List(items) => {
                            let mut __out = Vec::with_capacity(items.len());
                            for v in items.iter() { __out.push(<#inner as ::dsl::DslField>::from_value(v).map_err(::dsl::__rt::field_error)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected List, found {other:?}"))),
                    }
                },
            ),
            // Same `to_value`/`from_value` as `VecList` (both produce `FieldValue::List(Record)`)
            // — only the `Shape` differs (`Table` vs `List(Record)`), which is what makes the
            // printer emit compact SoA instead of verbose AoS for this field.
            FieldKind::VecTable(inner) => (
                quote! { ::dsl::Shape::Table(<#inner>::__dsl_spec_producer()) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::List(__items)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::List(items) => {
                            let mut __out = Vec::with_capacity(items.len());
                            for v in items.iter() { __out.push(<#inner as ::dsl::DslField>::from_value(v).map_err(::dsl::__rt::field_error)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected List, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecTuple(inner) => (
                quote! { ::dsl::Shape::Tuple(Box::new(<#inner as ::dsl::DslField>::shape()), None) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::Tuple(__items)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Tuple(items) => {
                            let mut __out = Vec::with_capacity(items.len());
                            for v in items.iter() { __out.push(<#inner as ::dsl::DslField>::from_value(v).map_err(::dsl::__rt::field_error)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected Tuple, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslVariants::to_named_record(v)); }
                        ::dsl::FieldValue::Statements(__items)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Statements(items) => {
                            let mut __out = Vec::with_capacity(items.len());
                            for (keyword, record) in items.iter() { __out.push(<#inner as ::dsl::DslVariants>::from_named_record(keyword, record)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected Statements, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecBlockStatements(inner) => (
                quote! { ::dsl::Shape::Block(Box::new(::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()))) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslVariants::to_named_record(v)); }
                        ::dsl::FieldValue::Block(Box::new(::dsl::FieldValue::Statements(__items)))
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Block(inner_value) => match inner_value.as_ref() {
                            ::dsl::FieldValue::Statements(items) => {
                                let mut __out = Vec::with_capacity(items.len());
                                for (keyword, record) in items.iter() { __out.push(<#inner as ::dsl::DslVariants>::from_named_record(keyword, record)?); }
                                __out
                            }
                            other => return Err(::dsl::__rt::field_error(format!("expected Statements inside Block, found {other:?}"))),
                        },
                        other => return Err(::dsl::__rt::field_error(format!("expected Block, found {other:?}"))),
                    }
                },
            ),
            FieldKind::MapField(inner) => (
                quote! { ::dsl::Shape::Map(Box::new(<#inner as ::dsl::DslField>::shape())) },
                quote! {
                    {
                        let mut __entries = Vec::with_capacity(self.#ident.len());
                        for (k, v) in self.#ident.iter() { __entries.push((k.clone(), ::dsl::DslField::to_value(v))); }
                        ::dsl::FieldValue::Map(__entries)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Map(entries) => {
                            let mut __out = ::std::collections::BTreeMap::new();
                            for (k, v) in entries.iter() { __out.insert(k.clone(), <#inner as ::dsl::DslField>::from_value(v).map_err(::dsl::__rt::field_error)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected Map, found {other:?}"))),
                    }
                },
            ),
            FieldKind::OptionStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! {
                    ::dsl::FieldValue::Statements(match &self.#ident {
                        Some(v) => vec![::dsl::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Absent => None,
                        ::dsl::FieldValue::Statements(items) if items.is_empty() => None,
                        ::dsl::FieldValue::Statements(items) if items.len() == 1 => {
                            Some(<#inner as ::dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected 0 or 1 tagged values, found {other:?}"))),
                    }
                },
            ),
            FieldKind::RequiredStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! { ::dsl::FieldValue::Statements(vec![::dsl::DslVariants::to_named_record(self.#ident.as_ref())]) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Statements(items) if items.len() == 1 => {
                            Box::new(<#inner as ::dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected exactly 1 tagged value, found {other:?}"))),
                    }
                },
            ),
        };

        // `#[dsl(block)]` on a field whose own `FieldKind` doesn't already imply `{ }` wrapping
        // (`VecBlockStatements` does that itself) — generically wraps whatever shape the match
        // above produced, e.g. turning a nested `#[derive(DslRecord)]` scalar field into a bare
        // `camera { x=0 y=0 zoom=1 }` line instead of a `camera=...` attribute.
        //
        // `FieldValue::Absent` (an `Option<T>` field's `None`) is deliberately NOT wrapped: an
        // empty `stroke { }` would reparse as "a record whose every field is absent", not "no
        // record at all" — `StrokeStyle`'s own non-optional fields would then fail with "expected
        // a 4-item Tuple, found Absent" instead of the field itself just being omitted, exactly
        // like an ordinary (non-block) optional field already is.
        let (shape_expr, to_value_expr, from_value_expr) = if *block {
            (
                quote! { ::dsl::Shape::Block(Box::new(#shape_expr)) },
                quote! {
                    match #to_value_expr {
                        ::dsl::FieldValue::Absent => ::dsl::FieldValue::Absent,
                        other => ::dsl::FieldValue::Block(Box::new(other)),
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Block(inner) => { let value = inner.as_ref(); #from_value_expr },
                        ::dsl::FieldValue::Absent => { let value = &::dsl::FieldValue::Absent; #from_value_expr },
                        other => return Err(::dsl::__rt::field_error(format!("expected Block, found {other:?}"))),
                    }
                },
            )
        } else {
            (shape_expr, to_value_expr, from_value_expr)
        };

        spec_exprs.push(quote! {
            ::dsl::FieldSpec::new(#id, #key, #shape_expr) #pos_expr #opt_expr #defines_expr
        });
        to_value_stmts.push(quote! {
            record.fields.insert(#id, #to_value_expr);
        });
        let local = field_local(ident);
        from_value_stmts.push(quote! {
            let #local = {
                let value = record.get(#id).ok_or_else(|| ::dsl::__rt::field_error(format!("missing field '{}'", #key)))?;
                #from_value_expr
            };
        });
    }

    (spec_exprs, to_value_stmts, from_value_stmts, field_idents)
}

/// 🏭️ Emits declared field metadata without borrowing an ordinary allocating shape factory.
fn schema_record_codegen(fields:&Fields)->Vec<proc_macro2::TokenStream>{
    let plans=plan_fields(fields);
    plans.iter().map(|plan|{
        let FieldPlan{id,key,positional,optional,kind,elem_ty,block,unit,angle,refs,defines,lang,lang_from,coord,dir,..}=plan;
        let refinement=if let Some(symbol)=unit{Some(quote!{::dsl::Shape::Quantity(::dsl::__rt::unit_for_derive(#symbol))})}else if let Some(symbol)=angle{Some(quote!{::dsl::Shape::Angle(::dsl::__rt::unit_for_derive(#symbol))})}else if let Some(kind)=refs{Some(quote!{::dsl::Shape::Ref(#kind)})}else if let Some(from)=lang_from{let key=plans.iter().find(|plan|plan.ident==from.as_str()).map_or_else(||to_kebab(from),|plan|plan.key.clone());Some(quote!{::dsl::Shape::EmbedFrom(#key)})}else if let Some(language)=lang{Some(quote!{::dsl::Shape::Embed(#language)})}else if *coord{Some(quote!{::dsl::Shape::Coord(3)})}else if *dir{Some(quote!{::dsl::Shape::Dir})}else{None};
        let shape=match kind{
            FieldKind::Scalar=>refinement.unwrap_or_else(||quote!{<#elem_ty as ::dsl::DslField>::shape_controlled(control)?}),
            FieldKind::OptionScalar(inner)=>refinement.unwrap_or_else(||quote!{<#inner as ::dsl::DslField>::shape_controlled(control)?}),
            FieldKind::Bytes64=>quote!{::dsl::Shape::Bytes64},
            FieldKind::VecList(inner)=>quote!{::dsl::Shape::List(::dsl::schema::producer::boxed(<#inner as ::dsl::DslField>::shape_controlled(control)?,control)?)},
            FieldKind::VecTuple(inner)=>quote!{::dsl::Shape::Tuple(::dsl::schema::producer::boxed(<#inner as ::dsl::DslField>::shape_controlled(control)?,control)?,None)},
            FieldKind::VecTable(inner)=>quote!{::dsl::Shape::Table(<#inner>::__dsl_spec_producer())},
            FieldKind::MapField(inner)=>quote!{::dsl::Shape::Map(::dsl::schema::producer::boxed(<#inner as ::dsl::DslField>::shape_controlled(control)?,control)?)},
            FieldKind::VecStatements(inner)|FieldKind::OptionStatements(inner)|FieldKind::RequiredStatements(inner)=>quote!{::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants_controlled(control)?)},
            FieldKind::VecBlockStatements(inner)=>quote!{::dsl::Shape::Block(::dsl::schema::producer::boxed(::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants_controlled(control)?),control)?)},
        };
        let shape=if *block{quote!{::dsl::Shape::Block(::dsl::schema::producer::boxed(#shape,control)? )}}else{shape};
        let position=positional.map(|position|quote!{.positional(#position as u8)});let optional=if *optional{quote!{.optional()}}else{quote!{}};let defines=defines.as_ref().map(|kind|quote!{.defines(#kind)});
        quote!{let field=::dsl::schema::producer::field(#id,#key,#shape,control)? #position #optional #defines;fields.push(field);control.step()?;}
    }).collect()
}
//#endregion 🔖️RecordCodegen

/// 🛬️ Generates explicit controlled bindings from the same authored field plans.
fn controlled_record_codegen(fields:&Fields)->Vec<proc_macro2::TokenStream>{
    plan_fields(fields).iter().map(|plan|{
        let FieldPlan{ident,id,key,kind,elem_ty,block,..}=plan;
        let expression=match kind {
            FieldKind::Scalar=>quote!{control.scoped_stage(|control|<#elem_ty as ::dsl::DslField>::from_value_controlled(value,control))?},
            FieldKind::Bytes64=>quote!{match value{::dsl::FieldValue::Bytes64(bytes)=>{control.step()?;control.copy_bytes(bytes)?},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Bytes64"))}},
            FieldKind::OptionScalar(inner)=>quote!{match value{::dsl::FieldValue::Absent=>{control.step()?;None},other=>Some(control.scoped_stage(|control|<#inner as ::dsl::DslField>::from_value_controlled(other,control))?)}},
            FieldKind::VecList(inner)|FieldKind::VecTable(inner)=>quote!{match value{::dsl::FieldValue::List(items)=>::dsl::__rt::decode_list_controlled::<#inner>(items,control)?,_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected List"))}},
            FieldKind::VecTuple(inner)=>quote!{match value{::dsl::FieldValue::Tuple(items)=>::dsl::__rt::decode_list_controlled::<#inner>(items,control)?,_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Tuple"))}},
            FieldKind::MapField(inner)=>quote!{control.scoped_stage(|control|<::std::collections::BTreeMap<String,#inner> as ::dsl::DslField>::from_value_controlled(value,control))?},
            FieldKind::VecStatements(inner)=>controlled_statements(inner),
            FieldKind::VecBlockStatements(inner)=>{let expression=controlled_statements(inner);quote!{match value{::dsl::FieldValue::Block(inner)=>{let value=inner.as_ref();#expression},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Block"))}}},
            FieldKind::OptionStatements(inner)=>quote!{match value{::dsl::FieldValue::Absent=>None,::dsl::FieldValue::Statements(items) if items.is_empty()=>None,::dsl::FieldValue::Statements(items) if items.len()==1=>Some(control.scoped_stage(|control|<#inner as ::dsl::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control))?),_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected0or1 tagged values"))}},
            FieldKind::RequiredStatements(inner)=>quote!{match value{::dsl::FieldValue::Statements(items) if items.len()==1=>{control.charge(::std::mem::size_of::<#inner>())?;Box::new(control.scoped_stage(|control|<#inner as ::dsl::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control))?)},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected exactly1 tagged value"))}},
        };
        let expression=if *block {quote!{match value{::dsl::FieldValue::Block(inner)=>{let value=inner.as_ref();#expression},::dsl::FieldValue::Absent=>{let value=&::dsl::FieldValue::Absent;#expression},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Block"))}}}else{expression};
        let retire=retire_field_codegen(plan,quote!{value});
        let ident=field_local(ident);
        quote!{let #ident=::dsl::__rt::DecodedFieldOwner::new(control.scoped_stage(|control|{let value=record.get(#id).ok_or_else(||::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("missing field '{}'",#key)))?;Ok::<_,::semio_framework_value::ValueError>(#expression)})?,|value|{#retire});}
    }).collect()
}

/// 🛫️ Projects explicit declared fields with cumulative ownership and known workloads.
fn encoding_record_codegen(fields:&Fields,bindings:bool)->Vec<proc_macro2::TokenStream>{
    plan_fields(fields).iter().map(|plan|{
        let FieldPlan{ident,id,kind,elem_ty,block,..}=plan;
        let source=if bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};
        let projection=match kind{
            FieldKind::Scalar=>quote!{<#elem_ty as ::dsl::DslField>::to_value_controlled(value,control)?},
            FieldKind::Bytes64=>quote!{::dsl::FieldValue::Bytes64(control.copy_bytes(value)?)},
            FieldKind::OptionScalar(inner)=>quote!{match value{Some(value)=><#inner as ::dsl::DslField>::to_value_controlled(value,control)?,None=>::dsl::FieldValue::Absent}},
            FieldKind::VecList(_)|FieldKind::VecTable(_)=>quote!{::dsl::FieldValue::List(::dsl::native_encoding::project_list(value,control)?)},
            FieldKind::VecTuple(_)=>quote!{::dsl::FieldValue::Tuple(::dsl::native_encoding::project_list(value,control)?)},
            FieldKind::MapField(_)=>quote!{::dsl::native_encoding::project_map(value,control)?},
            FieldKind::VecStatements(_)=>quote!{::dsl::native_encoding::project_statements(value,control)?},
            FieldKind::VecBlockStatements(_)=>quote!{{control.charge(::std::mem::size_of::<::dsl::FieldValue>())?;::dsl::FieldValue::Block(Box::new(::dsl::native_encoding::project_statements(value,control)?))}},
            FieldKind::OptionStatements(_)=>quote!{match value{Some(value)=>::dsl::native_encoding::project_statements(::std::slice::from_ref(value),control)?,None=>::dsl::FieldValue::Statements(Vec::new())}},
            FieldKind::RequiredStatements(_)=>quote!{::dsl::native_encoding::project_statements(::std::slice::from_ref(value.as_ref()),control)?},
        };
        let projection=if *block{quote!{match #projection{::dsl::FieldValue::Absent=>::dsl::FieldValue::Absent,value=>{let value=::dsl::__rt::DecodedFieldOwner::new(value,::dsl::native_encoding::retire_field);control.charge(::std::mem::size_of::<::dsl::FieldValue>())?;::dsl::FieldValue::Block(Box::new(value.take()))}}}}else{projection};
        quote!{let field=control.scoped_stage(|control|{control.begin_stage(0)?;let value=#source;Ok::<_,::semio_framework_value::ValueError>(#projection)})?;record.insert(#id,field)?;control.step()?;}
    }).collect()
}

/// 🧹️ Delegates each completed field to its declared field or variant retirement.
fn retire_field_codegen(plan:&FieldPlan,value:proc_macro2::TokenStream)->proc_macro2::TokenStream{
    let FieldPlan{kind,elem_ty,..}=plan;
    match kind {
        FieldKind::Scalar=>quote!{<#elem_ty as ::dsl::DslField>::retire_decoded(#value);},
        FieldKind::Bytes64=>quote!{drop(#value);},
        FieldKind::OptionScalar(inner)=>quote!{if let Some(value)=#value{<#inner as ::dsl::DslField>::retire_decoded(value);}},
        FieldKind::VecList(inner)|FieldKind::VecTable(inner)|FieldKind::VecTuple(inner)=>quote!{for value in #value{<#inner as ::dsl::DslField>::retire_decoded(value);}},
        FieldKind::MapField(inner)=>quote!{<::std::collections::BTreeMap<String,#inner> as ::dsl::DslField>::retire_decoded(#value);},
        FieldKind::VecStatements(inner)|FieldKind::VecBlockStatements(inner)=>quote!{for value in #value{<#inner as ::dsl::DslVariants>::retire_decoded_variant(value);}},
        FieldKind::OptionStatements(inner)=>quote!{if let Some(value)=#value{<#inner as ::dsl::DslVariants>::retire_decoded_variant(value);}},
        FieldKind::RequiredStatements(inner)=>quote!{<#inner as ::dsl::DslVariants>::retire_decoded_variant(*#value);},
    }
}

/// 🌲️ Builds record retirement from explicit field plans or an authored domain lifecycle.
fn record_retirement_codegen(fields:&Fields,retire_with:Option<&syn::Path>)->proc_macro2::TokenStream{
    if let Some(owner)=retire_with{return quote!{#owner(self);};}
    let plans=plan_fields(fields);let idents=field_inits(&plans.iter().map(|plan|plan.ident.clone()).collect::<Vec<_>>());
    let retirements=plans.iter().map(|plan|{let ident=field_local(&plan.ident);retire_field_codegen(plan,quote!{#ident})});
    quote!{let Self{#(#idents),*}=self;#(#retirements)*}
}

/// 🌿️ Builds a controlled tagged-list binding without borrowing an unchecked constructor.
fn controlled_statements(inner:&Type)->proc_macro2::TokenStream{
    quote!{match value{::dsl::FieldValue::Statements(items)=>::dsl::__rt::decode_statements_controlled::<#inner>(items,control)?,_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Statements"))}}
}

//#region 🔖️DslRecord
pub fn expand_dsl_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslRecord only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let field_initializers = field_inits(&field_idents);
    let controlled_stmts=controlled_record_codegen(&data.fields);
    let schema_stmts=schema_record_codegen(&data.fields);let schema_count=schema_stmts.len();
    let encoding_stmts=encoding_record_codegen(&data.fields,false);
    let encoding_count=encoding_stmts.len();
    let controlled_fields=field_idents.iter().map(|ident|{let local=field_local(ident);quote!{#ident:#local.take()}});
    let retirement=record_retirement_codegen(&data.fields,container.retire_with.as_ref());
    let schema_keyword_expr=match &container.keyword{Some(keyword)=>quote!{Some(#keyword)},None=>quote!{None}};
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::dsl::RecordLayout::Lines }
    } else {
        quote! { ::dsl::RecordLayout::Inline }
    };

    let expanded = quote! {
        impl #name {
            // 🚫️async: E4 — its VALUE is stored as the fn pointer in `Shape::Record(Self::__dsl_spec)`
            // below (`DslField::shape` is itself E4 for the same reason — see R9), and `spec_exprs`
            // (built from the now-sync `DslField::shape`/`DslVariants::variants`) needs no executor.
            pub fn __dsl_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            /// 🏭️ Owns exactly the declared metadata fields under either native control.
            pub fn __dsl_spec_controlled<C: ::dsl::NativeSchemaControl>(control:&mut C)->Result<::dsl::RecordSpec,::dsl::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut fields=control.allocate_vec(#schema_count)?;#(#schema_stmts)*::dsl::schema::producer::record(#schema_keyword_expr,#layout_expr,fields,control)}))
            }
            /// 🪆️ Retains lazy owner factories without constructing their metadata.
            pub fn __dsl_spec_producer()->::dsl::RecordSpecProducer{::dsl::RecordSpecProducer{ordinary:Self::__dsl_spec,decoding:|control|Self::__dsl_spec_controlled(control),encoding:|control|Self::__dsl_spec_controlled(control)}}
            pub fn __dsl_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            /// 🛫️ Projects complete named fields under caller-owned output admission.
            pub fn __dsl_to_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::dsl::RecordValue,::semio_framework_value::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#encoding_count)?;let mut record=::dsl::native_encoding::EncodedRecord::new(#encoding_count,control)?;#(#encoding_stmts)*Ok::<_,::semio_framework_value::ValueError>(record.take())}))
            }
            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::semio_framework_diagnostic::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_initializers),* })
            }
            /// 🛬️ Binds owned typed fields with cumulative allocation and interior cancellation.
            pub fn __dsl_from_record_controlled(record:&::dsl::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                control.checkpoint()?;
                #(#controlled_stmts)*
                Ok(Self{#(#controlled_fields),*})
            }
        }

        impl ::dsl::DslField for #name {
            fn retire_decoded(self){#retirement}
            // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Record(Self::__dsl_spec_producer())
            }
            fn shape_controlled<C: ::dsl::NativeSchemaControl>(control:&mut C)->Result<::dsl::Shape,::dsl::ValueError>{control.checkpoint()?;Ok(::dsl::Shape::Record(Self::__dsl_spec_producer()))}
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Record(self.__dsl_to_record())
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
                    other => Err(format!("expected Record, found {other:?}")),
                }
            }
            fn from_value_controlled(value:&::dsl::FieldValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                match value{::dsl::FieldValue::Record(record)=>Self::__dsl_from_record_controlled(record,control),_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Record"))}
            }
            fn from_record_controlled(record:&::dsl::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{Self::__dsl_from_record_controlled(record,control)}
            fn to_value_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::dsl::FieldValue,::semio_framework_value::ValueError>{Self::__dsl_to_record_controlled(self,control).map(::dsl::FieldValue::Record)}
            fn to_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::dsl::RecordValue,::semio_framework_value::ValueError>{Self::__dsl_to_record_controlled(self,control)}
        }
    };
    expanded.into()
}
//#endregion 🔖️DslRecord

//#region 🔖️DslArtifact
pub fn expand_dsl_document(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let envelope_id = match container.id.clone().or_else(|| container.extension.clone()) {
        Some(id) => id,
        None => {
            return syn::Error::new_spanned(&input, "DslArtifact requires #[dsl(id = \"plugin.artifact\")] or #[dsl(extension = \"...\")]").to_compile_error().into();
        }
    };
    let extension_suffix = container.extension.as_deref().unwrap_or_else(|| envelope_id.rsplit('.').next().unwrap_or(&envelope_id));
    let envelope_id_lit = envelope_id.as_str();
    let extension_suffix_lit = extension_suffix;
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslArtifact only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let field_initializers = field_inits(&field_idents);
    let controlled_stmts=controlled_record_codegen(&data.fields);
    let schema_stmts=schema_record_codegen(&data.fields);let schema_count=schema_stmts.len();
    let encoding_stmts=encoding_record_codegen(&data.fields,false);
    let encoding_count=encoding_stmts.len();
    let controlled_fields=field_idents.iter().map(|ident|{let local=field_local(ident);quote!{#ident:#local.take()}});
    let retirement=record_retirement_codegen(&data.fields,container.retire_with.as_ref());
    let schema_keyword_expr=match &container.keyword{Some(keyword)=>quote!{Some(#keyword)},None=>quote!{None}};
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::dsl::RecordLayout::Lines }
    } else {
        quote! { ::dsl::RecordLayout::Inline }
    };

    let expanded = quote! {
        impl #name {
            // 🚫️async: E4 — see `DslRecord`'s `__dsl_spec` above; identical reasoning.
            pub fn __dsl_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            /// 🏭️ Owns exactly the declared metadata fields under either native control.
            pub fn __dsl_spec_controlled<C: ::dsl::NativeSchemaControl>(control:&mut C)->Result<::dsl::RecordSpec,::dsl::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut fields=control.allocate_vec(#schema_count)?;#(#schema_stmts)*::dsl::schema::producer::record(#schema_keyword_expr,#layout_expr,fields,control)}))
            }
            /// 🪆️ Retains lazy owner factories without constructing their metadata.
            pub fn __dsl_spec_producer()->::dsl::RecordSpecProducer{::dsl::RecordSpecProducer{ordinary:Self::__dsl_spec,decoding:|control|Self::__dsl_spec_controlled(control),encoding:|control|Self::__dsl_spec_controlled(control)}}
            pub fn __dsl_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            /// 🛫️ Projects complete named fields under caller-owned output admission.
            pub fn __dsl_to_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::dsl::RecordValue,::semio_framework_value::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#encoding_count)?;let mut record=::dsl::native_encoding::EncodedRecord::new(#encoding_count,control)?;#(#encoding_stmts)*Ok::<_,::semio_framework_value::ValueError>(record.take())}))
            }
            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::store::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_initializers),* })
            }
            /// 🛬️ Binds owned typed fields with cumulative allocation and interior cancellation.
            pub fn __dsl_from_record_controlled(record:&::dsl::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                control.checkpoint()?;
                #(#controlled_stmts)*
                Ok(Self{#(#controlled_fields),*})
            }
            /// ✉️ Envelope constants for handcrafted ArtifactDsl/ArtifactPack wiring (P6: derive no longer emits those traits).
            pub const __DSL_ENVELOPE_ID: &'static str = #envelope_id_lit;
            pub const __DSL_EXTENSION: &'static str = #extension_suffix_lit;
        }

        // A document type can also be nested as an ordinary field (e.g. a "whole document
        // snapshot" operation variant), so it needs `DslField` too, not just `store::ArtifactDsl`.
        impl ::dsl::DslField for #name {
            fn retire_decoded(self){#retirement}
            // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Record(Self::__dsl_spec_producer())
            }
            fn shape_controlled<C: ::dsl::NativeSchemaControl>(control:&mut C)->Result<::dsl::Shape,::dsl::ValueError>{control.checkpoint()?;Ok(::dsl::Shape::Record(Self::__dsl_spec_producer()))}
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Record(self.__dsl_to_record())
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
                    other => Err(format!("expected Record, found {other:?}")),
                }
            }
            fn from_value_controlled(value:&::dsl::FieldValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                match value{::dsl::FieldValue::Record(record)=>Self::__dsl_from_record_controlled(record,control),_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Record"))}
            }
            fn from_record_controlled(record:&::dsl::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{Self::__dsl_from_record_controlled(record,control)}
            fn to_value_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::dsl::FieldValue,::semio_framework_value::ValueError>{Self::__dsl_to_record_controlled(self,control).map(::dsl::FieldValue::Record)}
            fn to_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::dsl::RecordValue,::semio_framework_value::ValueError>{Self::__dsl_to_record_controlled(self,control)}
        }

    };
    expanded.into()
}
//#endregion 🔖️DslArtifact

//#region 🔖️DslDiff
pub fn expand_dsl_diff(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslDiff only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let field_initializers = field_inits(&field_idents);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::dsl::RecordLayout::Lines }
    } else {
        quote! { ::dsl::RecordLayout::Inline }
    };

    let expanded = quote! {
        impl #name {
            // 🚫️async: E1 pure accessor consumed by the same `spec_exprs` shape `DslRecord`'s
            // `__dsl_spec` uses — E4-transitively sync, see R9.
            pub fn __dsl_diff_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn __dsl_diff_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn __dsl_diff_from_record(record: &::dsl::RecordValue) -> Result<Self, ::semio_framework_diagnostic::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_initializers),* })
            }
        }

        impl ::semio_framework_os_kernel::DiffCodec for #name {
            fn print_diff(&self) -> String {
                ::dsl::print(&self.__dsl_diff_to_record(), &Self::__dsl_diff_spec(), ::dsl::JoinMode::Inline)
            }
            fn parse_diff(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
                let record = ::dsl::parse(line, &Self::__dsl_diff_spec(), &::dsl::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: ::dsl::SourceMode::Inline })?;
                Self::__dsl_diff_from_record(&record)
            }
            fn encode_diff(&self) -> Result<Vec<u8>, ::semio_framework_os_kernel::ProtocolError> {
                ::store::pack_rt::encode_document(&Self::__dsl_diff_spec(), &self.__dsl_diff_to_record(), &::store::PackEncodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)
            }
            fn decode_diff(bytes: &[u8]) -> Result<Self, ::semio_framework_os_kernel::ProtocolError> {
                let (record, _report) = ::store::pack_rt::decode_document(bytes, &Self::__dsl_diff_spec(), &::store::PackDecodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)?;
                Self::__dsl_diff_from_record(&record).map_err(|error| ::semio_framework_os_kernel::ProtocolError::Malformed { what: "diff record", offset: 0, detail: error.to_string() })
            }
        }
    };
    expanded.into()
}
//#endregion 🔖️DslDiff

//#region 🔖️DslScalar
pub fn expand_dsl_scalar(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslScalar only supports unit-variant enums").to_compile_error().into();
    };
    let mut variant_tags = Vec::new();
    let mut schema_tags=Vec::new();
    let mut match_to_ordinal = Vec::new();
    let mut match_from_ordinal = Vec::new();
    for (ordinal, variant) in data.variants.iter().enumerate() {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(variant, "DslScalar only supports unit variants").to_compile_error().into();
        }
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let tag = attrs.key.unwrap_or_else(|| to_kebab(&variant_ident.to_string()));
        let ordinal = ordinal as u32;
        variant_tags.push(quote! { (#tag.to_string(), #ordinal) });
        schema_tags.push(quote!{variants.push((control.copy_text(#tag)?,#ordinal));control.step()?;});
        match_to_ordinal.push(quote! { #name::#variant_ident => #ordinal });
        match_from_ordinal.push(quote! { #ordinal => Ok(#name::#variant_ident) });
    }

    let schema_count=schema_tags.len();
    let expanded = quote! {
        impl ::dsl::DslField for #name {
            // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Enum(vec![ #(#variant_tags),* ])
            }
            fn shape_controlled<C: ::dsl::NativeSchemaControl>(control:&mut C)->Result<::dsl::Shape,::dsl::ValueError>{control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut variants=control.allocate_vec(#schema_count)?;#(#schema_tags)*Ok(::dsl::Shape::Enum(variants))})}
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Enum(match self { #(#match_to_ordinal),* })
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Enum(ordinal) => match *ordinal {
                        #(#match_from_ordinal,)*
                        other => Err(format!("unknown enum ordinal {other}")),
                    },
                    other => Err(format!("expected Enum, found {other:?}")),
                }
            }
            fn from_value_controlled(value:&::dsl::FieldValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{control.step()?;match value{::dsl::FieldValue::Enum(ordinal)=>match *ordinal{#(#match_from_ordinal,)*other=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown enum ordinal {other}")))},_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected declared Enum"))}}
            fn to_value_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::dsl::FieldValue,::semio_framework_value::ValueError>{control.step()?;Ok(::dsl::FieldValue::Enum(match self{#(#match_to_ordinal),*}))}
        }
    };
    expanded.into()
}
//#endregion 🔖️DslScalar

//#region 🔖️DslOps
/// 🌿️ Builds the `impl ::dsl::DslVariants for #name` block shared by `DslEnum` (data-only
/// tagged enums, e.g. a recursive block tree) and `DslOps` (operation enums, which additionally get
/// `store::OpText` on top of this same `DslVariants` foundation).
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn dsl_variants_codegen(name: &syn::Ident, data: &syn::DataEnum, retire_with:Option<&syn::Path>) -> proc_macro2::TokenStream {
    let mut variants_exprs = Vec::new();
    let mut schema_variant_stmts=Vec::new();
    let mut schema_methods=Vec::new();
    let mut to_named_arms = Vec::new();
    let mut encoding_arms=Vec::new();
    let mut from_named_arms = Vec::new();
    let mut controlled_arms=Vec::new();
    let mut retirement_arms=Vec::new();

    for (variant_index,variant) in data.variants.iter().enumerate() {
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let keyword = attrs.key.clone().unwrap_or_else(|| to_kebab(&variant_ident.to_string()));
        let fields = &variant.fields;

        // A single-field tuple variant (`Shape(DrawShapeBody)`) delegates entirely to its inner
        // type's own `DslField` impl — its `RecordSpec` IS the inner type's, not a wrapper with one
        // positional field, so a body already declared with `#[derive(DslRecord)]` (its own keyword,
        // its own fields) prints/parses completely unchanged whether reached through the enum or on
        // its own.
        if let Fields::Unnamed(unnamed) = fields {
            if unnamed.unnamed.len() == 1 {
                let inner_ty = &unnamed.unnamed[0].ty;
                variants_exprs.push(quote!{(#keyword.to_string(),::dsl::__rt::newtype_variant_producer::<#inner_ty>())});
                schema_variant_stmts.push(quote!{variants.push((control.copy_text(#keyword)?,::dsl::__rt::newtype_variant_producer::<#inner_ty>()));control.step()?;});
                to_named_arms.push(quote! {
                    #name::#variant_ident(inner) => (#keyword.to_string(), ::dsl::__rt::newtype_variant_to_record(inner))
                });
                from_named_arms.push(quote! {
                    #keyword => Ok(#name::#variant_ident(::dsl::__rt::newtype_variant_from_record::<#inner_ty>(record)?))
                });
                encoding_arms.push(quote!{#name::#variant_ident(inner)=>{let keyword=control.copy_text(#keyword)?;let record=<#inner_ty as ::dsl::DslField>::to_record_controlled(inner,control)?;Ok((keyword,record))}});
                controlled_arms.push(quote!{#keyword=>Ok(#name::#variant_ident(control.scoped_stage(|control|<#inner_ty as ::dsl::DslField>::from_record_controlled(record,control))?))});
                retirement_arms.push(quote!{#name::#variant_ident(inner)=><#inner_ty as ::dsl::DslField>::retire_decoded(inner)});
                continue;
            }
        }

        let (spec_exprs, _to_value_stmts, from_value_stmts, field_idents) = record_codegen(fields);
        let controlled_stmts=controlled_record_codegen(fields);
        let controlled_fields=field_idents.iter().map(|ident|{let local=field_local(ident);quote!{#ident:#local.take()}});

        let schema_name=quote::format_ident!("__dsl_variant_spec_{}",variant_index);let controlled_name=quote::format_ident!("__dsl_variant_spec_{}_controlled",variant_index);let producer_name=quote::format_ident!("__dsl_variant_spec_{}_producer",variant_index);
        let schema_stmts=schema_record_codegen(fields);let schema_count=schema_stmts.len();
        schema_methods.push(quote!{
            fn #schema_name()->::dsl::RecordSpec{::dsl::RecordSpec::new_owned(Some(#keyword.to_string()),::dsl::RecordLayout::Inline,vec![#(#spec_exprs),*])}
            fn #controlled_name<C: ::dsl::NativeSchemaControl>(control:&mut C)->Result<::dsl::RecordSpec,::dsl::ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut fields=control.allocate_vec(#schema_count)?;#(#schema_stmts)*::dsl::schema::producer::record(Some(#keyword),::dsl::RecordLayout::Inline,fields,control)}))}
            fn #producer_name()->::dsl::RecordSpecProducer{::dsl::RecordSpecProducer{ordinary:Self::#schema_name,decoding:|control|Self::#controlled_name(control),encoding:|control|Self::#controlled_name(control)}}
        });
        variants_exprs.push(quote!{(#keyword.to_string(),Self::#producer_name())});
        schema_variant_stmts.push(quote!{variants.push((control.copy_text(#keyword)?,Self::#producer_name()));control.step()?;});

        // Build a per-variant to-record conversion using the field bindings from a `match` on
        // `self`, since (unlike `DslRecord`) the fields live inside an enum variant, not `self.field`.
        // A true unit variant (`Variant`, no braces at all) needs a bare match pattern — `Variant {}`
        // is only valid Rust for a variant that was itself declared with (empty) braces.
        let field_binds: Vec<proc_macro2::TokenStream> = field_idents.iter().map(|f| { let local = field_local(f); quote! { #f: #local } }).collect();
        let to_value_stmts_for_variant: Vec<proc_macro2::TokenStream> = record_codegen_to_value_from_bindings(fields);
        let is_unit = matches!(fields, Fields::Unit);
        let match_pattern = if is_unit {
            quote! { #name::#variant_ident }
        } else {
            quote! { #name::#variant_ident { #(#field_binds),* } }
        };
        let construct_expr = if is_unit {
            quote! { #name::#variant_ident }
        } else {
            let inits = field_inits(&field_idents);
            quote! { #name::#variant_ident { #(#inits),* } }
        };
        to_named_arms.push(quote! {
            #match_pattern => {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts_for_variant)*
                (#keyword.to_string(), record)
            }
        });
        from_named_arms.push(quote! {
            #keyword => {
                #(#from_value_stmts)*
                Ok(#construct_expr)
            }
        });
        let encoding_stmts=encoding_record_codegen(fields,true);let encoding_count=encoding_stmts.len();
        encoding_arms.push(quote!{#match_pattern=>{control.begin_stage(#encoding_count)?;let mut record=::dsl::native_encoding::EncodedRecord::new(#encoding_count,control)?;#(#encoding_stmts)*let keyword=control.copy_text(#keyword)?;Ok((keyword,record.take()))}});
        let controlled_construct=if is_unit{quote!{#name::#variant_ident}}else{quote!{#name::#variant_ident{#(#controlled_fields),*}}};
        controlled_arms.push(quote!{#keyword=>{#(#controlled_stmts)* Ok(#controlled_construct)}});
        let plans=plan_fields(fields);let retirements=plans.iter().map(|plan|{let ident=field_local(&plan.ident);retire_field_codegen(plan,quote!{#ident})});
        retirement_arms.push(quote!{#match_pattern=>{#(#retirements)*}});
    }

    let retirement=if let Some(owner)=retire_with{quote!{#owner(self);}}else{quote!{match self{#(#retirement_arms),*}}};

    let schema_variant_count=schema_variant_stmts.len();
    quote! {
        impl #name{#(#schema_methods)*}
        impl ::dsl::DslVariants for #name {
            fn retire_decoded_variant(self){#retirement}
            // 🚫️async: E4 — see `DslVariants::variants`'s tag on the trait.
            fn variants() -> Vec<(String, ::dsl::RecordSpecProducer)> {
                vec![ #(#variants_exprs),* ]
            }
            fn variants_controlled<C: ::dsl::NativeSchemaControl>(control:&mut C)->Result<Vec<(String,::dsl::RecordSpecProducer)>,::dsl::ValueError>{control.scoped_stage(|control|{control.begin_stage(#schema_variant_count)?;let mut variants=control.allocate_vec(#schema_variant_count)?;#(#schema_variant_stmts)*Ok(variants)})}
            fn to_named_record(&self) -> (String, ::dsl::RecordValue) {
                match self { #(#to_named_arms),* }
            }
            fn to_named_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<(String,::dsl::RecordValue),::semio_framework_value::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|match self{#(#encoding_arms),*}))
            }
            fn from_named_record(keyword: &str, record: &::dsl::RecordValue) -> Result<Self, ::semio_framework_diagnostic::TextError> {
                match keyword {
                    #(#from_named_arms,)*
                    other => Err(::dsl::__rt::field_error(format!("unknown keyword '{other}'"))),
                }
            }
            fn from_named_record_controlled(keyword:&str,record:&::dsl::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                control.step()?;
                match keyword{#(#controlled_arms,)*_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"unknown declared keyword"))}
            }
        }
    }
}

pub fn expand_dsl_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslOps only supports enums").to_compile_error().into();
    };
    let container=parse_container_attrs(&input);
    let variants_impl = dsl_variants_codegen(&name, data,container.retire_with.as_ref());

    // P6: DslOps emits DslVariants only — OpText/OpBinary must be handcrafted per artifact.
    variants_impl.into()
}
//#endregion 🔖️DslOps

//#region 🔖️DslEnum
pub fn expand_dsl_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslEnum only supports enums").to_compile_error().into();
    };
    let container=parse_container_attrs(&input);
    dsl_variants_codegen(&name, data,container.retire_with.as_ref()).into()
}
//#endregion 🔖️DslEnum

//#region 🔖️Mutations
/// 🗣️ `#[mutations(snapshot = ..., diff = ..., schema = "..." [, retire_cold = path])]` container
/// attrs for `#[derive(Mutations)]` — see that macro's doc. `retire_cold` names a `fn(Self)` that
/// disposes an operation owning fail-closed roots; the generated `Mutation::retire_cold` calls it.
#[derive(Default)]
struct MutationsAttrs {
    snapshot: Option<Type>,
    diff: Option<Type>,
    schema: Option<String>,
    retire_cold: Option<syn::Path>,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_mutations_attrs(input: &DeriveInput) -> syn::Result<MutationsAttrs> {
    let mut out = MutationsAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("mutations") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("snapshot") {
                if out.snapshot.is_some() { return Err(meta.error("duplicate mutations snapshot")); }
                out.snapshot = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("diff") {
                if out.diff.is_some() { return Err(meta.error("duplicate mutations diff")); }
                out.diff = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("schema") {
                if out.schema.is_some() { return Err(meta.error("duplicate mutations schema")); }
                let value: syn::LitStr = meta.value()?.parse()?;
                out.schema = Some(value.value());
            } else if meta.path.is_ident("retire_cold") {
                if out.retire_cold.is_some() { return Err(meta.error("duplicate mutations retire_cold")); }
                out.retire_cold = Some(meta.value()?.parse()?);
            } else { return Err(meta.error("unsupported mutations attribute")); }
            Ok(())
        })?;
    }
    Ok(out)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-attrs/🦀️.rs"]
mod mutation_attrs_tests;

pub fn expand_derive_mutations(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let source = match input.ident.span().unwrap().local_file() {
        Some(source) => source,
        None => return syn::Error::new_spanned(&input, "Mutations requires a local source file").to_compile_error().into(),
    };
    let compiler_cwd = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into(),
    };
    let authority = match mutation_aggregate_source_authority(&source, &compiler_cwd) {
        Ok(authority) => authority,
        Err(error) => return syn::Error::new_spanned(&input, format!("Mutations source authority failed: {error}")).to_compile_error().into(),
    };
    match expand_mutations(&input, &authority) {
        Ok(expanded) => expanded.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_mutations(input: &DeriveInput, authority: &MutationAggregateSourceAuthority) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(input, "#[derive(Mutations)] only supports enums"));
    };
    if data.variants.is_empty() {
        return Err(syn::Error::new_spanned(input, "Mutations requires at least one concrete leaf"));
    }
    if input.attrs.iter().any(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr")) {
        return Err(syn::Error::new_spanned(input, "Mutations does not permit conditional aggregate metadata"));
    }
    let attrs = parse_mutations_attrs(input)?;
    let retire_cold = attrs.retire_cold.map(|retire| quote! {
        fn retire_cold(self) {
            #retire(self)
        }
    });
    let (Some(snapshot_ty), Some(diff_ty), Some(schema)) = (attrs.snapshot, attrs.diff, attrs.schema) else {
        return Err(syn::Error::new_spanned(input, "#[derive(Mutations)] requires #[mutations(snapshot = YourSnapshot, diff = YourDiff, schema = \"your.doc.schema\")]"));
    };
    let map_error = |error: String| syn::Error::new_spanned(input, error);
    let workspace_token = mutation_authority_workspace_token(&authority.workspace_root, &authority.taxonomy_path).map_err(map_error)?;
    let mutation_root = mutation_authority_relative(&authority.workspace_root, &authority.mutation_root).map_err(map_error)?;
    let taxonomy_path = mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path).map_err(map_error)?;
    let dependency_paths = [authority.taxonomy_path.clone()];
    let dependencies = dependency_paths.iter().map(|path| mutation_leaf_include_path(path)).collect::<Result<Vec<_>, _>>().map_err(map_error)?;
    let source_filename = &authority.source_filename;
    let descriptor_filename = &authority.descriptor_filename;
    let mutation_payload_facet = &authority.mutation_payload_facet;
    let owner_layout = if let Some(operations) = &authority.domain_operations {
        let entries = operations.iter().map(|(owner, identity)| quote! { ::semio_framework_os_kernel::MutationDomainOperation { owner: #owner, semantic_kind: #identity } });
        quote! { ::semio_framework_os_kernel::MutationOwnerLayout::DomainOperations(&[#(#entries),*]) }
    } else {
        quote! { ::semio_framework_os_kernel::MutationOwnerLayout::Flat }
    };
    let components = if authority.component_roots.is_empty() { vec![(mutation_root.clone(), authority.domain_operations.clone())] } else { authority.component_roots.clone() };
    let mut component_completeness = Vec::new();
    let scopes = components.iter().map(|(root, operations)| {
        let layout = if let Some(operations) = operations {
            let entries = operations.iter().map(|(owner, identity)| quote! { ::semio_framework_os_kernel::MutationDomainOperation { owner: #owner, semantic_kind: #identity } });
            for (owner, _) in operations {
                component_completeness.push(quote! {
                    let mut found = false;
                    let mut index = 0;
                    while index < descriptors.len() {
                        if ::semio_framework_os_kernel::str_eq(descriptors[index].owner, #owner) { found = true; }
                        index += 1;
                    }
                    assert!(found, "Mutations requires every explicitly registered component domain operation");
                });
            }
            quote! { ::semio_framework_os_kernel::MutationOwnerLayout::DomainOperations(&[#(#entries),*]) }
        } else { quote! { ::semio_framework_os_kernel::MutationOwnerLayout::Flat } };
        quote! {
            match (::semio_framework_os_kernel::MutationLeafSourceScope {
                workspace_token: [#(#workspace_token),*], mutation_root: #root, owner_layout: #layout,
                taxonomy_path: #taxonomy_path, mutation_payload_facet: #mutation_payload_facet,
                source_filename: #source_filename, descriptor_filename: #descriptor_filename,
            }).validate() {
                Ok(scope) => scope,
                Err(_) => panic!("Mutations requires a valid aggregate component source scope"),
            }
        }
    }).collect::<Vec<_>>();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let aggregate_ty = quote! { #name #ty_generics };
    let mut diff_arms = Vec::new();
    let mut inverse_arms = Vec::new();
    let mut timestamp_arms = Vec::new();
    let mut descriptor_arms = Vec::new();
    let mut semantics_arms = Vec::new();
    let mut label_arms = Vec::new();
    let mut target_arms = Vec::new();
    let mut may_emit_foreign_steps_arms = Vec::new();
    let mut foreign_steps_arms = Vec::new();
    let mut input_schema_arms = Vec::new();
    let mut inverse_rows_arms = Vec::new();
    let mut payload_value_arms = Vec::new();
    let mut with_payload_value_arms = Vec::new();
    let mut from_payload_value_arms = Vec::new();
    let mut input_schemas = Vec::new();
    let mut input_schema_documents = Vec::new();
    let mut kind_consts = Vec::new();
    let mut leaf_descriptors = Vec::new();
    let mut leaf_checks = Vec::new();
    let mut conversions = Vec::new();
    let mut register_calls = Vec::new();

    for (index, variant) in data.variants.iter().enumerate() {
        let variant_ident = &variant.ident;
        let Fields::Unnamed(unnamed) = &variant.fields else {
            return Err(syn::Error::new_spanned(variant, "Mutations requires every variant to wrap exactly one direct MutationKind payload"));
        };
        if unnamed.unnamed.len() != 1 {
            return Err(syn::Error::new_spanned(variant, "Mutations requires every variant to wrap exactly one direct MutationKind payload"));
        }
        if variant.attrs.iter().chain(unnamed.unnamed[0].attrs.iter()).any(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr")) {
            return Err(syn::Error::new_spanned(variant, "Mutations does not permit conditional leaf metadata"));
        }
        let payload_ty = &unnamed.unnamed[0].ty;
        if !matches!(payload_ty, Type::Path(path) if path.qself.is_none()) {
            return Err(syn::Error::new_spanned(payload_ty, "Mutations requires a direct leaf type path"));
        }
        let expected_variant = variant_ident.to_string();
        let expected_kebab = to_kebab(&expected_variant);
        let kind = quote! { <#payload_ty as ::semio_framework_os_kernel::MutationKind<#snapshot_ty, #aggregate_ty>> };
        let leaf = quote! { <#payload_ty as ::semio_framework_os_kernel::MutationLeaf> };
        diff_arms.push(quote! { Self::#variant_ident(payload) => #kind::diff(payload, base) });
        inverse_arms.push(quote! { Self::#variant_ident(payload) => #kind::inverse(payload, base) });
        timestamp_arms.push(quote! { Self::#variant_ident(payload) => #kind::timestamp(payload) });
        descriptor_arms.push(quote! { Self::#variant_ident(_) => &<Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS[#index] });
        semantics_arms.push(quote! { Self::#variant_ident(_) => &#kind::SEMANTICS });
        label_arms.push(quote! { Self::#variant_ident(payload) => #kind::label(payload) });
        target_arms.push(quote! { Self::#variant_ident(payload) => #kind::target(payload) });
        may_emit_foreign_steps_arms.push(quote! { Self::#variant_ident(payload) => #kind::may_emit_foreign_steps(payload) });
        foreign_steps_arms.push(quote! { Self::#variant_ident(payload) => #kind::foreign_steps(payload, base) });
        input_schema_arms.push(quote! { Self::#variant_ident(payload) => #leaf::input_schema(payload) });
        inverse_rows_arms.push(quote! { Self::#variant_ident(payload) => #leaf::inverse_rows(payload) });
        payload_value_arms.push(quote! { Self::#variant_ident(payload) => #leaf::input_value(payload) });
        with_payload_value_arms.push(quote! { Self::#variant_ident(payload) => #leaf::with_input_value(payload, value).map(Self::#variant_ident) });
        from_payload_value_arms.push(quote! { if kind == #kind::SEMANTICS.kind { return #leaf::from_input_value(value).map(Self::#variant_ident); } });
        input_schemas.push(quote! { #leaf::PAYLOAD_SCHEMA });
        input_schema_documents.push(quote! { #leaf::PAYLOAD_SCHEMA_DOCUMENTS });
        kind_consts.push(quote! { #kind::SEMANTICS });
        leaf_descriptors.push(quote! { #leaf::DESCRIPTOR });
        leaf_checks.push(quote! { const {
            assert!(::semio_framework_os_kernel::str_eq(#kind::SEMANTICS.kind, #expected_kebab), "Mutations semantic kind must match its variant");
            assert!(::semio_framework_os_kernel::is_approved_verb(#kind::SEMANTICS.verb), "Mutations requires an approved semantic verb");
            assert!(::semio_framework_os_kernel::str_eq(#leaf::DESCRIPTOR.aggregate_variant, #expected_variant), "Mutations descriptor variant must match its wrapped leaf");
            assert!(::semio_framework_os_kernel::str_eq(#leaf::DESCRIPTOR.semantic_kind, #kind::SEMANTICS.kind), "Mutations descriptor and semantic kind must agree");
            let mut scope_index = 0;
            let mut matched = false;
            while scope_index < SCOPES.len() {
                if let Ok(()) = SCOPES[scope_index].validate_leaf(&#leaf::DESCRIPTOR, &#leaf::PROVENANCE) { matched = true; break; }
                scope_index += 1;
            }
            assert!(matched, "Mutations leaf source must match its aggregate workspace and declared component owner");
        }; });
        conversions.push(quote! {
            impl #impl_generics ::core::convert::From<#payload_ty> for #aggregate_ty #where_clause {
                fn from(payload: #payload_ty) -> Self {
                    let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                    Self::#variant_ident(payload)
                }
            }
        });
        register_calls.push(quote! {
            ::semio_framework_os_kernel::MutationDescriptor::new(
                ::semio_framework_os_kernel::SchemaId(format!("{}#{}", #schema, #kind::SEMANTICS.kind)),
                ::semio_framework_os_kernel::SchemaVersion(1),
                state_class,
                #leaf::DESCRIPTOR,
                #kind::SEMANTICS,
            )?
        });
    }

    let register_fn_ident = syn::Ident::new(&format!("register_{}_descriptors", to_kebab(&name.to_string()).replace('-', "_")), name.span());
    let payload_law = mutation_payload_law_test(name, &snapshot_ty, authority, &input.generics);
    let eager_check = input.generics.params.is_empty().then(|| quote! {
        const _: () = { let _ = <#name as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS; };
    });
    Ok(quote! {
        #(const _: &str = ::core::include_str!(#dependencies);)*
        #eager_check
        #(#conversions)*

        impl #impl_generics ::semio_framework_os_kernel::Mutation<#snapshot_ty> for #aggregate_ty #where_clause {
            type Diff = #diff_ty;
            const DESCRIPTORS: &'static [::semio_framework_os_kernel::MutationLeafDescriptor] = {
                const SCOPES: &[::semio_framework_os_kernel::ValidatedMutationLeafSourceScope] = &[#(#scopes),*];
                #(#leaf_checks)*
                let descriptors: &'static [::semio_framework_os_kernel::MutationLeafDescriptor] = &[#(#leaf_descriptors),*];
                #(#component_completeness)*
                match ::semio_framework_os_kernel::validate_mutation_leaf_descriptor_roster_uniqueness(#mutation_root, descriptors, #owner_layout) {
                    Ok(()) => descriptors,
                    Err(_) => panic!("Mutations requires a unique and complete direct leaf descriptor roster"),
                }
            };
            fn descriptor(&self) -> &'static ::semio_framework_os_kernel::MutationLeafDescriptor {
                match self { #(#descriptor_arms),* }
            }
            fn diff(&self, base: &#snapshot_ty) -> ::semio_framework_os_kernel::MutationOutcome<Self::Diff> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#diff_arms),* }
            }
            fn inverse(&self, base: &#snapshot_ty) -> Vec<Self> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#inverse_arms),* }
            }
            fn timestamp(&self) -> Option<::semio_framework_os_kernel::HybridLogicalTimestamp> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#timestamp_arms),* }
            }
            fn conflict_target(&self) -> Vec<String> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#target_arms),* }
            }
            fn may_emit_foreign_steps(&self) -> bool {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#may_emit_foreign_steps_arms),* }
            }
            fn foreign_steps(&self, base: &#snapshot_ty) -> Vec<::semio_framework_os_kernel::ForeignStep> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#foreign_steps_arms),* }
            }
            fn inverse_rows(&self) -> usize {
                match self { #(#inverse_rows_arms),* }
            }
            const INPUT_SCHEMAS: &'static [&'static str] = &[#(#input_schemas),*];
            const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = &[#(#input_schema_documents),*];
            fn input_schema(&self) -> ::core::option::Option<&'static str> {
                match self { #(#input_schema_arms),* }
            }
            fn payload_value(&self) -> ::semio_framework_os_kernel::DslValue {
                match self { #(#payload_value_arms),* }
            }
            fn with_payload_value(&self, value: ::semio_framework_os_kernel::DslValue) -> ::core::result::Result<Self, ::semio_framework_os_kernel::ValueError> {
                match self { #(#with_payload_value_arms),* }
            }
            fn from_payload_value(kind: &str, value: ::semio_framework_os_kernel::DslValue) -> ::core::result::Result<Self, ::semio_framework_os_kernel::ValueError> {
                #(#from_payload_value_arms)*
                ::core::result::Result::Err(::semio_framework_os_kernel::ValueError::new(::semio_framework_os_kernel::ValueRefusalKind::InvalidValue,::std::format!("{kind} is no leaf kind of {}", ::core::stringify!(#name))))
            }
            #retire_cold
        }

        impl #impl_generics ::semio_framework_os_kernel::SemanticMutation<#snapshot_ty> for #aggregate_ty #where_clause {
            fn kinds() -> &'static [::semio_framework_os_kernel::SemanticDescriptor] {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                &[#(#kind_consts),*]
            }
            fn semantics(&self) -> &'static ::semio_framework_os_kernel::SemanticDescriptor {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#semantics_arms),* }
            }
            fn label(&self) -> ::semio_framework_ui_locale::LocalizedLabel {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#label_arms),* }
            }
            fn target(&self) -> Vec<String> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#target_arms),* }
            }
        }

        /// 🪪️ Registers the validated leaf roster during owner startup.
        pub fn #register_fn_ident #impl_generics (
            state_class: ::semio_framework_schema_state::StateClass,
        ) -> ::core::result::Result<(), ::semio_framework_os_kernel::MutationDescriptorError> #where_clause {
            let _ = <#aggregate_ty as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
            let descriptors = [#(#register_calls),*];
            ::semio_framework_os_kernel::register_mutation_descriptors(descriptors)
        }

        #payload_law
    })
}

/// ⚖️ The `#[cfg(test)]` editable-payload law `#[derive(Mutations)]` emits for a non-generic aggregate: every leaf publishes one
/// payload schema (`::semio_framework_os_kernel::mutation_input_schema_failures`), and every committed fixture under the
/// aggregate's owner directory (the one holding its `🧬️schema`) that decodes as the aggregate and, when the aggregate's own file
/// defines a top-level `demo_mutation_cases() -> Vec<Aggregate>`, every demo case is labelled in every locale
/// (`mutation_label_failures`) and rebuilds itself from its own editable payload or, when inert, refuses to
/// (`mutation_payload_round_trip_failures`), and every fixture case's inverse fits the leaf's declared rows
/// (`mutation_inverse_rows_failures`, design §20.5) — walked over every subset of the owner's standard, since a subset's
/// fixtures replay through the aggregate too. Nothing is emitted when the owner directory cannot be addressed from the
/// compiling crate.
fn mutation_payload_law_test(name: &syn::Ident, snapshot_ty: &syn::Type, authority: &MutationAggregateSourceAuthority, generics: &syn::Generics) -> proc_macro2::TokenStream {
    if !generics.params.is_empty() {
        return quote! {};
    }
    let parent = authority.mutation_root.parent();
    let owner = match parent {
        Some(schema) if schema.file_name().and_then(|segment| segment.to_str()) == Some("🧬️schema") => schema.parent(),
        other => other,
    };
    let (Some(owner), Ok(manifest)) = (owner, std::env::var("CARGO_MANIFEST_DIR")) else { return quote! {} };
    let Some(relative) = mutation_relative_path(Path::new(&manifest), owner) else { return quote! {} };
    let subsets = owner.parent().filter(|subsets| subsets.file_name().and_then(|segment| segment.to_str()) == Some("🪆️subsets")).unwrap_or(owner);
    let Some(footprint_relative) = mutation_relative_path(Path::new(&manifest), subsets) else { return quote! {} };
    let source = fs::read_to_string(authority.mutation_root.join(&authority.source_filename)).unwrap_or_default();
    let signature = format!("demo_mutation_cases() -> Vec<{name}>");
    let demo_cases = source.lines().any(|line| ["fn ", "pub fn ", "pub(crate) fn "].iter().any(|prefix| line.strip_prefix(prefix).is_some_and(|rest| rest.starts_with(&signature)))).then(|| quote! { ops.extend(demo_mutation_cases()); });
    let test_ident = syn::Ident::new(&format!("semio_payload_law_{}", to_kebab(&name.to_string()).replace('-', "_")), name.span());
    quote! {
        #[cfg(test)]
        #[test]
        fn #test_ident() {
            let root = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join(#relative);
            let (mut ops, _) = ::semio_framework_os_kernel::mutation_fixture_ops::<#name>(&root);
            #demo_cases
            let count = ops.len();
            let mut failures = ::semio_framework_os_kernel::mutation_input_schema_failures::<#snapshot_ty, #name>();
            failures.extend(::semio_framework_os_kernel::mutation_label_failures::<#snapshot_ty, #name>(&ops));
            failures.extend(::semio_framework_os_kernel::mutation_payload_round_trip_failures::<#snapshot_ty, #name>(ops));
            let (footprint_failures, cases) = ::semio_framework_os_kernel::mutation_inverse_rows_failures::<#snapshot_ty, #name>(&::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join(#footprint_relative));
            failures.extend(footprint_failures);
            assert!(failures.is_empty(), "{} breaches of the editable-payload and fold-footprint laws over {} {} operations and {} fixture cases: {:#?}", failures.len(), count, ::core::stringify!(#name), cases, failures);
        }
    }
}

/// 🧭️ `to` relative to `from` (both canonical), in `/` segments — `None` when they share no root or a segment is not UTF-8.
fn mutation_relative_path(from: &Path, to: &Path) -> Option<String> {
    let (from, to) = (mutation_authority_canonical(from).ok()?, mutation_authority_canonical(to).ok()?);
    let (from, to): (Vec<_>, Vec<_>) = (from.components().collect(), to.components().collect());
    let shared = from.iter().zip(&to).take_while(|(left, right)| left == right).count();
    if shared == 0 {
        return None;
    }
    let mut segments: Vec<String> = std::iter::repeat_n("..".to_string(), from.len() - shared).collect();
    for component in &to[shared..] {
        segments.push(component.as_os_str().to_str()?.to_string());
    }
    Some(segments.join("/"))
}
//#endregion 🔖️Mutations

//#region 🧪️MandatoryMutations
#[cfg(test)]
#[path = "🧪️tests/🔬️mandatory-mutations/🦀️.rs"]
mod mandatory_mutations_tests;
//#endregion 🧪️MandatoryMutations

//#region 🔖️CompositeMutation
/// 🌉️ `#[composite(snapshot = ..., op = ...)]` container attrs for
/// `#[derive(CompositeMutation)]` — see that macro's doc.
#[derive(Default)]
struct CompositeAttrs {
    snapshot: Option<Type>,
    op: Option<Type>,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_composite_attrs(input: &DeriveInput) -> syn::Result<CompositeAttrs> {
    let mut out = CompositeAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("composite") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("snapshot") {
                if out.snapshot.is_some() { return Err(meta.error("duplicate composite snapshot")); }
                out.snapshot = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("op") {
                if out.op.is_some() { return Err(meta.error("duplicate composite op")); }
                out.op = Some(meta.value()?.parse()?);
            } else { return Err(meta.error("unsupported composite attribute")); }
            Ok(())
        })?;
    }
    Ok(out)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️composite-attrs/🦀️.rs"]
mod composite_attrs_tests;

pub fn expand_derive_composite_mutation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_composite_mutation(&input) {
        Ok(expanded) => expanded.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_composite_mutation(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = input.ident.clone();
    let attrs = parse_composite_attrs(input)?;
    let (Some(snapshot_ty), Some(op_ty)) = (attrs.snapshot, attrs.op) else {
        return Err(syn::Error::new_spanned(input, "#[derive(CompositeMutation)] requires #[composite(snapshot = YourSnapshot, op = YourOp)]"));
    };

    let expected_kebab = to_kebab(&name.to_string());
    let assert_kind_message = format!("#[derive(CompositeMutation)]: {}'s CompositeMutationKind::SEMANTICS.kind must equal \"{}\" (its own kebab form)", name, expected_kebab);
    let assert_verb_message = format!("#[derive(CompositeMutation)]: {}'s CompositeMutationKind::SEMANTICS.verb must be one of protocol::APPROVED_VERBS", name);

    let expanded = quote! {
        const _: () = assert!(::semio_framework_os_kernel::str_eq(<#name as ::semio_framework_os_kernel::CompositeMutationKind<#snapshot_ty, #op_ty>>::SEMANTICS.kind, #expected_kebab), #assert_kind_message);
        const _: () = assert!(::semio_framework_os_kernel::is_approved_verb(<#name as ::semio_framework_os_kernel::CompositeMutationKind<#snapshot_ty, #op_ty>>::SEMANTICS.verb), #assert_verb_message);

        impl ::semio_framework_os_kernel::MutationKind<#snapshot_ty, #op_ty> for #name {
            const SEMANTICS: ::semio_framework_os_kernel::SemanticDescriptor = <#name as ::semio_framework_os_kernel::CompositeMutationKind<#snapshot_ty, #op_ty>>::SEMANTICS;
            fn diff(&self, base: &#snapshot_ty) -> ::semio_framework_os_kernel::MutationOutcome<<#op_ty as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::Diff> {
                ::semio_framework_os_kernel::fold_plan_diff(self, base)
            }
            fn inverse(&self, base: &#snapshot_ty) -> Vec<#op_ty> {
                ::semio_framework_os_kernel::fold_plan_inverse(self, base)
            }
            fn label(&self) -> ::semio_framework_ui_locale::LocalizedLabel {
                ::semio_framework_os_kernel::CompositeMutationKind::label(self)
            }
            fn timestamp(&self) -> Option<::semio_framework_os_kernel::HybridLogicalTimestamp> {
                ::semio_framework_os_kernel::CompositeMutationKind::timestamp(self)
            }
            fn target(&self) -> Vec<String> {
                ::semio_framework_os_kernel::CompositeMutationKind::target(self)
            }
            fn may_emit_foreign_steps(&self) -> bool {
                true
            }
            fn foreign_steps(&self, base: &#snapshot_ty) -> Vec<::semio_framework_os_kernel::ForeignStep> {
                ::semio_framework_os_kernel::plan_foreign_steps(self, base)
            }
        }
    };
    Ok(expanded)
}
//#endregion 🔖️CompositeMutation

//#region 🧪️CompositeTimestamp
#[cfg(test)]
#[path = "🧪️tests/🔬️composite-timestamp/🦀️.rs"]
mod composite_timestamp_tests;
//#endregion 🧪️CompositeTimestamp

//#region 🔖️VariantHelpers
/// 🔡️ Converts a Rust identifier (`PascalCase`/`camelCase`/`snake_case`, any mix) into
/// lowercase `kebab-case` — the unified syntax law's key/keyword/tag convention. Falls back to
/// this whenever no explicit `#[dsl(key = "...")]` override is given, for variant keywords,
/// record field keys, and `DslScalar` variant tags alike, so `SetCamera` -> `set-camera`,
/// `airtightness_n50` -> `airtightness-n50`, `HTTPServer` -> `http-server`.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn to_kebab(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c == '_' || c == '-' {
            if !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            continue;
        }
        if c.is_uppercase() {
            let prev = if i == 0 { None } else { chars.get(i - 1).copied() };
            let next = chars.get(i + 1).copied();
            // A new word starts at an uppercase letter that follows a lowercase/digit
            // (`SetCamera` -> boundary before `C`) OR that follows another uppercase letter but
            // is itself followed by a lowercase one (`HTTPServer` -> boundary before the `S` that
            // starts "Server", not between every letter of the "HTTP" acronym).
            let boundary = match prev {
                Some(p) if p.is_lowercase() || p.is_ascii_digit() => true,
                Some(p) if p.is_uppercase() => next.is_some_and(|n| n.is_lowercase()),
                _ => false,
            };
            if boundary && !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// 🔒️ Reserved local for one authored field: generated statements bind authored fields only under
/// `__semio_field_<name>`, so no authored name (`field`, `record`, `control`, `value`, …) can shadow a
/// generated local or be captured by one. Law: `🗣️dsl/🧪️tests/🧪️hygienic-bindings`.
fn field_local(ident: &syn::Ident) -> syn::Ident {
    quote::format_ident!("__semio_field_{}", ident)
}

/// 🧷️ Struct/variant initializers `name: __semio_field_name` over the reserved locals.
fn field_inits(idents: &[syn::Ident]) -> Vec<proc_macro2::TokenStream> {
    idents.iter().map(|ident| { let local = field_local(ident); quote! { #ident: #local } }).collect()
}

/// 🏗️ Like the `to_value` half of `record_codegen`, but reading from bare local bindings
/// (`ident`) instead of `self.ident` — what a `match self { Variant { fields... } => ... }` arm
/// needs, since enum variant fields aren't reached through `self.field` syntax.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn record_codegen_to_value_from_bindings(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let plans = plan_fields(fields);
    plans
        .iter()
        .map(|plan| {
            let FieldPlan { ident, id, kind, block, .. } = plan;
            let ident = &field_local(ident);
            let to_value_expr: proc_macro2::TokenStream = match kind {
                FieldKind::Scalar => quote! { ::dsl::DslField::to_value(#ident) },
                FieldKind::Bytes64 => quote! { ::dsl::FieldValue::Bytes64(#ident.clone()) },
                FieldKind::OptionScalar(_) => quote! {
                    match #ident {
                        Some(v) => ::dsl::DslField::to_value(v),
                        None => ::dsl::FieldValue::Absent,
                    }
                },
                FieldKind::VecList(_) | FieldKind::VecTable(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::List(__items)
                    }
                },
                FieldKind::VecTuple(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::Tuple(__items)
                    }
                },
                FieldKind::VecStatements(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::dsl::DslVariants::to_named_record(v)); }
                        ::dsl::FieldValue::Statements(__items)
                    }
                },
                FieldKind::VecBlockStatements(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::dsl::DslVariants::to_named_record(v)); }
                        ::dsl::FieldValue::Block(Box::new(::dsl::FieldValue::Statements(__items)))
                    }
                },
                FieldKind::MapField(_) => quote! {
                    {
                        let mut __entries = Vec::with_capacity(#ident.len());
                        for (k, v) in #ident.iter() { __entries.push((k.clone(), ::dsl::DslField::to_value(v))); }
                        ::dsl::FieldValue::Map(__entries)
                    }
                },
                FieldKind::OptionStatements(_) => quote! {
                    ::dsl::FieldValue::Statements(match #ident {
                        Some(v) => vec![::dsl::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                FieldKind::RequiredStatements(_) => quote! { ::dsl::FieldValue::Statements(vec![::dsl::DslVariants::to_named_record(#ident.as_ref())]) },
            };
            let to_value_expr = if *block {
                quote! {
                    match #to_value_expr {
                        ::dsl::FieldValue::Absent => ::dsl::FieldValue::Absent,
                        other => ::dsl::FieldValue::Block(Box::new(other)),
                    }
                }
            } else {
                to_value_expr
            };
            quote! { record.fields.insert(#id, #to_value_expr); }
        })
        .collect()
}
//#endregion 🔖️VariantHelpers

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs

SHA-256 `4f0dd2fbfe8a2ad87b7223f28789f8caf441cef76315949c100e8e706857f9bd`; 55511 bytes.

```
//! 🧬️ `dsl` — facade for the token-native declarative DSL engine. Technologies depend on this one
//! crate (plus `vcs` for the `ArtifactDsl`/`OpText` trait definitions themselves) to get the
//! derive macros, the `DslField` binding trait primitive Rust types implement, and the `__rt`
//! runtime the generated code calls into.

use semio_framework_dsl::LanguageRole;
use semio_framework_dsl::LanguageSpec;
use semio_framework_dsl::language;
use semio_framework_dsl::language_for_extension;
use semio_framework_dsl::language_for_role_extension;
use semio_framework_diagnostic::TextSpan;
use semio_framework_dsl::UnitSpec;
use semio_framework_dsl::unit_by_symbol;
// The derive macros emit `::crate::os_dsl::...` paths so generated code reads identically regardless of
// which technology crate invokes them. That only resolves for the crates that depend on `dsl` as
// an external crate — which is every real consumer, but NOT this crate's own tests (a crate is
// never its own dependency). `// extern crate self removed after merge` is the standard fix: it makes `::dsl`
// resolve to this crate even when the derive is exercised in-crate, as the `🧪️Tests` region below does.
// Only needed for the in-crate tests, so it's cfg-gated to avoid an "unused extern crate" warning
// in ordinary (non-test) builds, where every real consumer already has `dsl` as a true dependency.
// extern crate self removed after merge

use semio_framework_dsl::*;

#[path = "🔗️reference/🦀️.rs"]
mod reference;

#[path = "🪟️viewport/🦀️.rs"]
mod viewport;

pub use crate::os_dsl::schema::*;
pub use dsl_derive::{DslArtifact, DslDiff, DslEnum, DslOps, DslRecord, DslScalar, MutationLeaf, Mutations};



pub use protocol::value::native_decoding::NativeDecodeControl;
use semio_framework_value::ValueRefusalKind;
pub use protocol::value::native_encoding::NativeEncodeControl;
#[path = "🛫️encode/🦀️.rs"]
pub mod native_encoding;

//#region 🔖️Field
/// 🔗️ Bridges a concrete Rust field type to the engine's `Shape`/`FieldValue` — every
/// primitive implements it directly; `#[derive(DslRecord)]`/`#[derive(DslScalar)]` implement it
/// for technology-declared nested types, so composition (a record field whose type is another
/// derived record or enum) works transparently through the same trait.
pub trait DslField: Sized {
    // 🚫️async: E4 fn-pointer transitivity — `Shape::Record`/`Table`/`Statements` hold
    // `fn() -> RecordSpec`; every `shape()` implementation ultimately feeds one, directly or
    // through a derived `__dsl_spec` — see R9.
    fn shape() -> Shape;
    /// 🏭️ Constructs only the explicitly declared shape metadata under caller admission.
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Err(ValueError::new(protocol::value::ValueRefusalKind::UnsupportedOwner,"field owner has no controlled native schema implementation"))}
    fn to_value(&self) -> FieldValue;
    /// 🛫️ Projects explicitly owned fields under cumulative output admission and cancellation.
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.checkpoint()?;Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled native projection implementation"))}
    /// 📑️ Projects a record without an intermediate boxed field carrier.
    fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{control.checkpoint()?;Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled record projection implementation"))}
    fn from_value(value: &FieldValue) -> Result<Self, String>;
    /// 🧹️ Retires a completed field according to its owner after partial reconstruction fails.
    fn retire_decoded(self) { drop(self); }
    /// 🛬️ Constructs an owned field under the caller's cumulative allocation and work control.
    fn from_value_controlled(_value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.checkpoint()?;
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled native construction implementation"))
    }
    /// 📑️ Binds a record view without cloning a temporary FieldValue carrier.
    fn from_record_controlled(_record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        control.checkpoint()?;
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled record construction implementation"))
    }
}

/// 📦️ Boxed ownership preserves the inner field's schema, value, and decoding errors.
impl<T: DslField> DslField for Box<T> {
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{T::to_value_controlled(self.as_ref(),control)}
    fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{T::to_record_controlled(self.as_ref(),control)}

    fn retire_decoded(self) { T::retire_decoded(*self); }
    fn shape() -> Shape {
        T::shape()
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{T::shape_controlled(control)}
    fn to_value(&self) -> FieldValue {
        T::to_value(self.as_ref())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        T::from_value(value).map(Box::new)
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.charge(std::mem::size_of::<T>())?;
        control.scoped_stage(|control|T::from_value_controlled(value, control)).map(Box::new)
    }
    fn from_record_controlled(record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.charge(std::mem::size_of::<T>())?;control.scoped_stage(|control|T::from_record_controlled(record,control)).map(Box::new)}
}

macro_rules! impl_dsl_field_int {
    ($ty:ty, $shape:expr, $variant:ident, $as_ty:ty) => {
        impl DslField for $ty {
            // 🚫️async: E4 — see `DslField::shape`'s tag above.
            fn shape() -> Shape {
                $shape
            }
            fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok($shape)}
            fn to_value(&self) -> FieldValue {
                FieldValue::$variant(*self as $as_ty)
            }
            fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::$variant(*self as $as_ty))}
            fn from_value(value: &FieldValue) -> Result<Self, String> {
                match value {
                    FieldValue::$variant(v) => <$ty>::try_from(*v).map_err(|_| format!("integer {v} out of range for {}", stringify!($ty))),
                    other => Err(format!("expected {}, found {other:?}", stringify!($variant))),
                }
            }
            fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
                control.step()?;
                match value { FieldValue::$variant(number)=><$ty>::try_from(*number).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,format!("integer {number} out of range for {}",stringify!($ty)))),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,concat!("expected ",stringify!($variant)))) }
            }
        }
    };
}

impl_dsl_field_int!(i8, Shape::Int, Int, i64);
impl_dsl_field_int!(i16, Shape::Int, Int, i64);
impl_dsl_field_int!(i32, Shape::Int, Int, i64);
impl_dsl_field_int!(i64, Shape::Int, Int, i64);
impl_dsl_field_int!(isize, Shape::Int, Int, i64);
impl_dsl_field_int!(u8, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u16, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u32, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u64, Shape::UInt, UInt, u64);
impl_dsl_field_int!(usize, Shape::UInt, UInt, u64);

impl DslField for bool {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Bool
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Bool)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Bool(*self)
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Bool(*self))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Bool(b) => Ok(*b),
            other => Err(format!("expected Bool, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> { control.step()?;match value {FieldValue::Bool(value)=>Ok(*value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Bool"))} }
}

impl DslField for f32 {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Float
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Float)}
    fn to_value(&self) -> FieldValue {
        let bits=self.to_bits();let value=if bits&0x7f800000==0x7f800000&&bits&0x7fffff!=0{f64::from_bits(((bits as u64&0x80000000)<<32)|0x7ff0000000000000|((bits as u64&0x7fffff)<<29))}else{*self as f64};FieldValue::Float(value)
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(<Self as DslField>::to_value(self))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f)=>{let bits=f.to_bits();if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err("NaN word is not exactly representable at binary32 width".into());}Ok(f32::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff)))}else{Ok(*f as f32)}},
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {FieldValue::Float(value)=>{let bits=value.to_bits();if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"NaN word is not exactly representable at binary32 width"));}Ok(f32::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff)))}else{Ok(*value as f32)}},_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Float"))}
    }
}

impl DslField for f64 {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Float
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Float)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Float(*self)
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Float(*self))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f) => Ok(*f),
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> { control.step()?;match value{FieldValue::Float(value)=>Ok(*value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Float"))} }
}

/// 🔤️ `String` binds as `Shape::Text` — the one string shape. The parser accepts either a
/// bare `Ident` token or a quoted `Text` token wherever `Text` is expected; the printer emits bare
/// (unquoted) whenever `crate::os_dsl::is_bare_ident` holds for the value, quoted+escaped otherwise —
/// so bare-vs-quoted is entirely a printing decision now, not a separate shape a field opts into.
impl DslField for String {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Text
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Text)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Text(self.clone())
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Text(control.copy_text(self)?))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Text(s) => Ok(s.clone()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value { FieldValue::Text(text)=>control.copy_text(text),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Text")) }
    }
}

/// 🔌️ A wire literal as a plain struct field (or inside a `#[dsl(table)]` `Vec` as a
/// `WIRE`-typed column) — thin `DslField` wrapper around `crate::os_dsl::schema::WireValue` so adopter
/// technologies never need to hand-roll their own `Shape::Wire` binding.
#[derive(Clone, Debug, PartialEq)]
pub struct Wire(pub WireValue);

impl DslField for Wire {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Wire
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Wire)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Wire(self.0.clone())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Wire(w) => Ok(Wire(w.clone())),
            other => Err(format!("expected Wire, found {other:?}")),
        }
    }
}
/// 📚️ General recursion seam: `#[derive(DslRecord)]`/`#[derive(DslScalar)]` fields classify
/// `Vec<T>`/`[T; N]` directly (so their own printed shape stays field-specific), but a NESTED
/// collection — `Vec<Vec<T>>`, a fixed-size array field, ... — needs its inner element type to
/// satisfy `DslField` itself. These two blanket impls close that gap generically instead of adding
/// a special-cased `FieldKind` for every depth of nesting.
impl<T: DslField> DslField for Vec<T> {
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_list(self,control).map(FieldValue::List)}

    fn retire_decoded(self) { for value in self { T::retire_decoded(value); } }
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::List(Box::new(T::shape()))
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::List(crate::os_dsl::schema::producer::boxed(T::shape_controlled(control)?,control)?)))}
    // 🔁 `Iterator::map` cannot await per-element (residue shape 1) and `T::to_value`/`from_value`
    // are AFIT over an arbitrary implementor, so — unlike a known-pure leaf fn — R9 does not apply;
    // the fix is a plain sequential loop that awaits each element in turn.
    fn to_value(&self) -> FieldValue {
        let mut items = Vec::with_capacity(self.len());
        for item in self {
            items.push(item.to_value());
        }
        FieldValue::List(items)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::List(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(T::from_value(item)?);
                }
                Ok(out)
            }
            other => Err(format!("expected List, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {
            FieldValue::List(items)=>__rt::decode_list_controlled(items,control),
            _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected List")),
        }
    }
}

/// 🗺️ Same recursion seam as `Vec<T>`, for a `BTreeMap<String, T>` that's itself nested
/// (e.g. `Option<BTreeMap<String, T>>`) rather than a bare top-level field — `#[derive(DslRecord)]`
/// classifies a *bare* `BTreeMap<String, T>` field directly via its own dedicated `FieldKind`
/// (same `Shape::Map` this produces), so the two never conflict.
impl<T: DslField> DslField for std::collections::BTreeMap<String, T> {
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_map(self,control)}

    fn retire_decoded(self) { for (_,value) in self { T::retire_decoded(value); } }
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Map(Box::new(T::shape()))
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::Map(crate::os_dsl::schema::producer::boxed(T::shape_controlled(control)?,control)?)))}
    // 🔁 Same R9-doesn't-apply reasoning as `Vec<T>` above: sequential loop, not `.map().collect()`.
    fn to_value(&self) -> FieldValue {
        let mut entries = Vec::with_capacity(self.len());
        for (k, v) in self {
            entries.push((k.clone(), v.to_value()));
        }
        FieldValue::Map(entries)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Map(entries) => {
                let mut out = Self::new();
                for (k, v) in entries {
                    out.insert(k.clone(), T::from_value(v)?);
                }
                Ok(out)
            }
            other => Err(format!("expected Map, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {
            FieldValue::Map(entries)=>{
                let slot=std::mem::size_of::<(String,T)>().checked_add(128).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"map slot size overflow"))?;
                control.charge(entries.len().checked_mul(slot).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"map ownership size overflow"))?)?;
                let mut output=__rt::DecodedFieldOwner::new(Self::new(),Self::retire_decoded);
                for (key,value) in entries {if let Some(previous)=output.as_mut().insert(control.copy_text(key)?,control.scoped_stage(|control|T::from_value_controlled(value,control))?){T::retire_decoded(previous);}}
                Ok(output.take())
            },
            _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Map")),
        }
    }
}

/// 📐️ Fixed-arity `Shape::Tuple(_, Some(N))` — a packed `x,y,z`-style literal for any `N`.
impl<T: DslField, const N: usize> DslField for [T; N] {
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_list(self,control).map(FieldValue::Tuple)}

    fn retire_decoded(self) { for value in self { T::retire_decoded(value); } }
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Tuple(Box::new(T::shape()), Some(N))
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::Tuple(crate::os_dsl::schema::producer::boxed(T::shape_controlled(control)?,control)?,Some(N))))}
    // 🔁 Same R9-doesn't-apply reasoning as `Vec<T>` above: sequential loop, not `.map().collect()`.
    fn to_value(&self) -> FieldValue {
        let mut items = Vec::with_capacity(N);
        for item in self {
            items.push(item.to_value());
        }
        FieldValue::Tuple(items)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Tuple(items) if items.len() == N => {
                let mut converted: Vec<T> = Vec::with_capacity(N);
                for item in items {
                    converted.push(T::from_value(item)?);
                }
                converted.try_into().map_err(|_| format!("expected {N} items, got a length mismatch"))
            }
            other => Err(format!("expected a {N}-item Tuple, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {
            FieldValue::Tuple(items) if items.len()==N=>{let mut output=__rt::DecodedFieldOwner::new(control.allocate_vec::<T>(N)?,<Vec<T> as DslField>::retire_decoded);for item in items {output.as_mut().push(control.scoped_stage(|control|T::from_value_controlled(item,control))?);}output.take().try_into().map_err(|values:Vec<T>|{<Vec<T> as DslField>::retire_decoded(values);ValueError::new(ValueRefusalKind::InvariantViolated,"tuple arity mismatch")})},
            _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("expected a {N}-item Tuple"))),
        }
    }
}

/// 🌱️ Schema-less dynamic literal — binds as `Shape::Value`.
impl DslField for DslValue {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Value
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Value)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Value(self.clone())
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{<Self as semio_framework_value::ToValue>::to_value_controlled(self,control).map(FieldValue::Value)}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{FieldValue::Value(value)=><Self as semio_framework_value::FromValue>::from_value_controlled(value,control),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected an intrinsic Value field"))}}
    fn retire_decoded(self){<Self as protocol::value::FromValue>::retire_decoded(self)}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Value(dsl_value) => Ok(dsl_value.clone()),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}
//#endregion 🔖️Field

//#region 🔖️Variants
/// 🌿️ Bridges an enum whose variants are each their own keyword-tagged record — the type
/// bound for `#[dsl(statements)] Vec<T>` collection fields and for `#[derive(DslOps)]` operation
/// enums. `#[derive(DslEnum)]`-with-struct-variants and `#[derive(DslOps)]` both implement this.
pub trait DslVariants: Sized {
    /// 🐌️ Lazy: each entry is a zero-capture `fn` pointer, not an eagerly-built `RecordSpec`
    /// — a self-referential grammar's own `variants()` would otherwise need to recurse infinitely
    /// just to construct this list. See [`Shape::Statements`]'s doc comment for the full rationale.
    // 🚫️async: E4 — the returned `Vec<(String, fn() -> RecordSpec)>` IS a fn-pointer table, and
    // `Shape::Statements(<T>::variants())` is itself called from inside a sync `__dsl_spec` — see R9.
    fn variants() -> Vec<(String, RecordSpecProducer)>;
    /// 🌿️ Owns literal variant labels and their lazy controlled schema producers.
    fn variants_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Vec<(String,RecordSpecProducer)>,ValueError>{control.checkpoint()?;Err(ValueError::new(protocol::value::ValueRefusalKind::UnsupportedOwner,"variant owner has no controlled native schema implementation"))}
    fn to_named_record(&self) -> (String, RecordValue);
    /// 🌿️ Projects a declared tagged variant under the same cumulative output control.
    fn to_named_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<(String,RecordValue),ValueError>{control.checkpoint()?;Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"variant owner has no controlled native projection implementation"))}
    /// ⚠️ Returns `TextError` (not `String`, unlike [`DslField::from_value`]) so
    /// generated bodies can `?`-propagate it directly — this is the same error type
    /// `crate::os_spr::OpText::parse_op`/`crate::os_store::ArtifactDsl::parse_dsl` already return, and the derive's
    /// `#[dsl(statements)]` field codegen composes it without any conversion at every nesting depth.
    fn from_named_record(keyword: &str, record: &RecordValue) -> Result<Self, TextError>;
    /// 🌲️ Retires a completed tagged value through its domain owner.
    fn retire_decoded_variant(self) { drop(self); }
    /// 🌿️ Constructs a declared variant without invoking an unchecked owner binding.
    fn from_named_record_controlled(_keyword:&str,_record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        control.checkpoint()?;
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"variant owner has no controlled native construction implementation"))
    }
}
//#endregion 🔖️Variants

//#region 🔖️Runtime
/// ⚙️ Helpers remaining after P6 flag day — DslField/DslVariants derive bodies only (codec paths deleted).
pub mod __rt {
    use super::*;

    /// 🧹️ Holds a completed typed field until construction commits or invokes its actual owner retirement.
    pub struct DecodedFieldOwner<T> { value:Option<T>, retire:fn(T) }
    impl<T> DecodedFieldOwner<T> {
        /// 📥️ Adopts one owned field with its declared retirement function.
        pub fn new(value:T,retire:fn(T))->Self { Self{value:Some(value),retire} }
        /// 🌿️ Allows bounded construction inside the guarded owned collection.
        pub fn as_mut(&mut self)->&mut T { self.value.as_mut().expect("decoded owner already transferred") }
        /// 📤️ Transfers ownership only after every required constructor succeeds.
        pub fn take(mut self)->T { self.value.take().expect("decoded owner already transferred") }
    }
    impl<T> Drop for DecodedFieldOwner<T> { fn drop(&mut self){if let Some(value)=self.value.take(){(self.retire)(value);}} }

    /// 📋️ Binds declared list elements with one known collection workload and cumulative ownership.
    pub fn decode_list_controlled<T:DslField>(items:&[FieldValue],control:&mut NativeDecodeControl<'_>)->Result<Vec<T>,ValueError>{
        control.scoped_stage(|control|{control.begin_stage(items.len())?;let mut output=DecodedFieldOwner::new(control.allocate_vec::<T>(items.len())?,<Vec<T> as DslField>::retire_decoded);for item in items{output.as_mut().push(control.scoped_stage(|control|{control.begin_stage(0)?;T::from_value_controlled(item,control)})?);control.step()?;}Ok(output.take())})
    }
    /// 🌿️ Binds tagged variants with exact collection progress and declared variant retirement.
    pub fn decode_statements_controlled<T:DslVariants>(items:&[(String,RecordValue)],control:&mut NativeDecodeControl<'_>)->Result<Vec<T>,ValueError>{
        control.scoped_stage(|control|{control.begin_stage(items.len())?;let mut output=DecodedFieldOwner::new(control.allocate_vec::<T>(items.len())?,|values:Vec<T>|{for value in values{T::retire_decoded_variant(value);}});for(keyword,record)in items{output.as_mut().push(control.scoped_stage(|control|{control.begin_stage(0)?;T::from_named_record_controlled(keyword,record,control)})?);control.step()?;}Ok(output.take())})
    }

    // 🚫️async: E1 pure error constructor, consumed by `Option::ok_or_else` sync closures in every
    // `#[derive(DslRecord)]`-generated body (`✨️derive/🦀️.rs`'s `quote!{}` templates) — see R9
    pub fn field_error(message: impl std::fmt::Display) -> TextError {
        TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),TextSpan::at(1, 1))
    }

    /// 📐️ Resolves a `#[dsl(unit = "...")]`/`#[dsl(angle = "...")]` symbol at spec-build
    /// time. An unknown symbol is a derive-time misuse (a typo'd unit string, caught the first time
    /// the generated `__dsl_spec` runs — every RecordSpec-law test exercises this), so it panics
    /// rather than threading a `Result` through the whole spec-building call chain, matching
    /// `newtype_variant_spec`'s convention above.
    pub fn unit_for_derive(symbol: &'static str) -> &'static UnitSpec {
        unit_by_symbol(symbol).unwrap_or_else(|| panic!("dsl: unknown unit symbol '{symbol}' in #[dsl(unit = ...)]/#[dsl(angle = ...)]"))
    }

    /// 📦️ Single-field tuple ("newtype") enum variant support — `Variant(Body)` delegates its
    /// whole `RecordSpec`/value to `Body`'s own `DslField` impl rather than wrapping it in one
    /// positional field, so `Body` prints/parses identically whether reached through the enum or on
    /// its own. `Body` must have `Shape::Record` (i.e. itself come from `#[derive(DslRecord)]` or
    /// `#[derive(DslArtifact)]`) — anything else is a derive-time misuse, hence the panic rather than
    /// a `Result` (there is no sensible recoverable path for a grammar that's wrong at compile time).
    // 🚫️async: E4 — this fn's VALUE is cast `as fn() -> RecordSpec` at every newtype-variant call
    // site (`✨️derive/🦀️.rs`'s `dsl_variants_codegen`), and it calls the now-sync `DslField::shape`.
    pub fn newtype_variant_spec<T: DslField>() -> RecordSpec {
        match T::shape() {
            Shape::Record(spec_fn) => (spec_fn.ordinary)(),
            other => panic!("newtype variant's inner type must have Record shape, found {other:?}"),
        }
    }

    /// 🪆️ Delegates a declared record variant through its explicit lazy schema producer.
    pub fn newtype_variant_producer<T:DslField>()->RecordSpecProducer{
        RecordSpecProducer{ordinary:newtype_variant_spec::<T>,decoding:|control|{match T::shape_controlled(control)?{Shape::Record(producer)=>producer.decode(control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires a controlled Record schema"))}},encoding:|control|{match T::shape_controlled(control)?{Shape::Record(producer)=>producer.encode(control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires a controlled Record schema"))}}}
    }

    pub fn newtype_variant_to_record<T: DslField>(inner: &T) -> RecordValue {
        match inner.to_value() {
            FieldValue::Record(record) => record,
            other => panic!("newtype variant's inner type must produce a Record value, found {other:?}"),
        }
    }

    pub fn newtype_variant_from_record<T: DslField>(record: &RecordValue) -> Result<T, TextError> {
        T::from_value(&FieldValue::Record(record.clone())).map_err(field_error)
    }
}

//#endregion 🔖️Runtime

//#region 🔖️OpTextRt
/// 🔤️ Handcrafted `OpText` helper — the text twin of [`variants_binary`].
///
/// An operation line is ONE terminal keyword-tagged record, so it parses through
/// [`parse_exact`], which rejects every token outside the variant's own schema body: a trailing
/// `unknown-field 1` is not a second statement, it is garbage the line must refuse. Plain
/// [`parse`] stops at the end of the record it recognises and silently drops the rest, which is
/// the document-mode contract, not the op-line one.
pub mod variants_text {
    use super::__rt::field_error;
    use super::{print, DslVariants, JoinMode, Limits, ParseOptions, SourceMode, TextError};

    pub fn parse_op<T: DslVariants>(line: &str) -> Result<T, TextError> {
        let variants = T::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword.as_str() || line.starts_with(&format!("{keyword} ")) {
                let record = super::parse_exact(line, &(spec_fn.ordinary)(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;
                return T::from_named_record(keyword, &record);
            }
        }
        Err(field_error(format!("unknown operation line '{line}'")))
    }

    pub fn print_op<T: DslVariants>(op: &T) -> String {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        print(&record, &(spec_fn.ordinary)(), JoinMode::Inline)
    }
}
//#endregion 🔖️OpTextRt

//#region 🏷️ProtocolRecord
/// 🏷️ The one source of a mutation vocabulary's op tags: the `record <kind> tag=<n>` lines of its
/// `💾️binary/📡️.protocol.semio`. Every codec derives its tags from here at compile time, so a kind whose
/// record is missing or duplicated fails the build instead of drifting from the wire.
/// See [`crate::os_dsl::grammar::parse_protocol`] for the full dialect this scanner agrees with.
pub mod protocol_record {
    const fn is_space(byte: u8) -> bool {
        byte == b' ' || byte == b'\t'
    }

    const fn is_name(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.'
    }

    const fn skip_space(bytes: &[u8], mut at: usize) -> usize {
        while at < bytes.len() && is_space(bytes[at]) {
            at += 1;
        }
        at
    }

    const fn line_end(bytes: &[u8], mut at: usize) -> usize {
        while at < bytes.len() && bytes[at] != b'\n' {
            at += 1;
        }
        at
    }

    const fn starts_with(bytes: &[u8], at: usize, prefix: &[u8]) -> bool {
        if at + prefix.len() > bytes.len() {
            return false;
        }
        let mut index = 0;
        while index < prefix.len() {
            if bytes[at + index] != prefix[index] {
                return false;
            }
            index += 1;
        }
        true
    }

    /// 🔎️ Parses the record header at `line`: `(name start, name end, tag)`, or `None` for any other line.
    const fn record_at(bytes: &[u8], line: usize) -> Option<(usize, usize, u64)> {
        let at = skip_space(bytes, line);
        if !starts_with(bytes, at, b"record") || at + 6 >= bytes.len() || !is_space(bytes[at + 6]) {
            return None;
        }
        let name_start = skip_space(bytes, at + 6);
        let mut name_end = name_start;
        while name_end < bytes.len() && is_name(bytes[name_end]) {
            name_end += 1;
        }
        let at = skip_space(bytes, name_end);
        if name_end == name_start || !starts_with(bytes, at, b"tag=") {
            return None;
        }
        let mut at = at + 4;
        let digits = at;
        let mut tag: u64 = 0;
        while at < bytes.len() && bytes[at].is_ascii_digit() {
            tag = tag * 10 + (bytes[at] - b'0') as u64;
            at += 1;
        }
        if at == digits {
            return None;
        }
        Some((name_start, name_end, tag))
    }

    const fn name_equals(bytes: &[u8], start: usize, end: usize, kind: &[u8]) -> bool {
        end - start == kind.len() && starts_with(bytes, start, kind)
    }

    /// 🔢️ `(tag, occurrences)` of `kind` across every record line.
    const fn scan(protocol: &str, kind: &str) -> (u64, usize) {
        let bytes = protocol.as_bytes();
        let kind = kind.as_bytes();
        let mut line = 0;
        let mut found = 0;
        let mut tag = 0;
        while line < bytes.len() {
            if let Some((start, end, value)) = record_at(bytes, line) {
                if name_equals(bytes, start, end, kind) {
                    found += 1;
                    tag = value;
                }
            }
            line = line_end(bytes, line) + 1;
        }
        (tag, found)
    }

    /// 🏷️ The tag `kind`'s record declares; a missing or duplicated record is a const-evaluation error.
    pub const fn tag(protocol: &str, kind: &str) -> u64 {
        match scan(protocol, kind) {
            (tag, 1) => tag,
            (_, 0) => panic!("📡️.protocol.semio declares no `record <kind> tag=<n>` for this mutation kind"),
            _ => panic!("📡️.protocol.semio declares this mutation kind more than once"),
        }
    }

    /// 🏷️ [`tag`] for a codec whose wire tag is one byte; a tag above 255 is a const-evaluation error.
    pub const fn tag_u8(protocol: &str, kind: &str) -> u8 {
        let tag = tag(protocol, kind);
        assert!(tag <= u8::MAX as u64, "📡️.protocol.semio record tag does not fit the codec's u8 tag field");
        tag as u8
    }

    /// 🏷️ [`tag`] for a codec whose wire tag is a `u32`; a tag above `u32::MAX` is a const-evaluation error.
    pub const fn tag_u32(protocol: &str, kind: &str) -> u32 {
        let tag = tag(protocol, kind);
        assert!(tag <= u32::MAX as u64, "📡️.protocol.semio record tag does not fit the codec's u32 tag field");
        tag as u32
    }

    /// 📇️ Every `(kind, tag)` record, in file order.
    pub fn records(protocol: &str) -> impl Iterator<Item = (&str, u64)> {
        let bytes = protocol.as_bytes();
        let mut line = 0;
        std::iter::from_fn(move || {
            while line < bytes.len() {
                let current = line;
                line = line_end(bytes, line) + 1;
                if let Some((start, end, tag)) = record_at(bytes, current) {
                    return Some((&protocol[start..end], tag));
                }
            }
            None
        })
    }

    /// 🔁️ The kind whose record declares `tag`.
    pub fn kind(protocol: &str, tag: u64) -> Option<&str> {
        records(protocol).find(|(_, value)| *value == tag).map(|(kind, _)| kind)
    }
}
//#endregion 🏷️ProtocolRecord

//#region 🔖️OpRt
/// 🎯️ Handcrafted OpBinary helper (P6): layout `format u8 (=1) | tag varint | record body`.
/// `encode_tagged_op`/`decode_tagged_op` take the tag from the vocabulary's `📡️.protocol.semio` record
/// ([`super::protocol_record`]); `encode_op`/`decode_op` serve the ephemeral layers that carry no wire
/// protocol facet, whose tag is the variant ordinal.
/// Called explicitly from handcrafted `protocol::OpBinary` impls — never re-emitted by derive.
pub mod variants_binary {
    use super::{protocol_record, DslVariants};
    use crate::os_pack::{decode_record_body_exact, encode_record_body, write_varint_u64, ByteReader, DecodeOptions, EncodeOptions};
    use crate::os_spr::ProtocolError;

    pub const OP_BINARY_FORMAT: u8 = 1;

    fn encode_with<T: DslVariants>(op: &T, tag_of: impl Fn(&str, usize) -> Result<u64, ProtocolError>) -> Result<Vec<u8>, ProtocolError> {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let ordinal = variants.iter().position(|(k, _)| k == &keyword).ok_or(ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword '{keyword}' missing from variants()") })?;
        let tag = tag_of(&keyword, ordinal)?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = encode_record_body(&spec, &record, &EncodeOptions::default()).map_err(ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        write_varint_u64(&mut out, tag);
        out.extend_from_slice(&body);
        Ok(out)
    }

    fn decode_with<T: DslVariants>(bytes: &[u8], index_of: impl Fn(u64, &[(String, super::RecordSpecProducer)]) -> Result<usize, ProtocolError>, reencode: impl Fn(&T) -> Result<Vec<u8>, ProtocolError>) -> Result<T, ProtocolError> {
        let mut reader = ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let tag = reader.read_varint_u64()?;
        let variants = T::variants();
        let index = index_of(tag, &variants)?;
        let (keyword, spec_fn) = &variants[index];
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let record = decode_record_body_exact(body, &spec, &DecodeOptions::default()).map_err(ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        let decoded = T::from_named_record(keyword, &record).map_err(|error| ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })?;
        if reencode(&decoded)?.as_slice() != bytes {
            return Err(ProtocolError::Malformed { what: "op encoding", offset: 0, detail: "operation bytes are not canonical".into() });
        }
        Ok(decoded)
    }

    /// 🏷️ Encodes `op` with the tag its kind's record declares in `protocol`.
    pub fn encode_tagged_op<T: DslVariants>(protocol: &str, op: &T) -> Result<Vec<u8>, ProtocolError> {
        encode_with(op, |keyword, _| protocol_record::records(protocol).find(|(kind, _)| *kind == keyword).map(|(_, tag)| tag).ok_or(ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record for '{keyword}'") }))
    }

    /// 🏷️ Decodes an op whose tag names its kind's record in `protocol`.
    pub fn decode_tagged_op<T: DslVariants>(protocol: &str, bytes: &[u8]) -> Result<T, ProtocolError> {
        decode_with(
            bytes,
            |tag, variants| {
                let kind = protocol_record::kind(protocol, tag).ok_or(ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record with tag {tag}") })?;
                variants.iter().position(|(keyword, _)| keyword == kind).ok_or(ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("record '{kind}' names no variant") })
            },
            |decoded| encode_tagged_op(protocol, decoded),
        )
    }

    pub fn encode_op<T: DslVariants>(op: &T) -> Result<Vec<u8>, ProtocolError> {
        encode_with(op, |_, ordinal| u64::try_from(ordinal).map_err(|_| ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} exceeds the u64 wire range") }))
    }

    pub fn decode_op<T: DslVariants>(bytes: &[u8]) -> Result<T, ProtocolError> {
        decode_with(
            bytes,
            |ordinal, variants| {
                let index = usize::try_from(ordinal).map_err(|_| ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} exceeds the native index range") })?;
                if index < variants.len() { Ok(index) } else { Err(ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) }) }
            },
            |decoded| encode_op(decoded),
        )
    }
}
//#endregion 🔖️OpRt

//#region 🏷️TaggedValueRt
/// 🏷️ Op frame for a mutation aggregate whose payload is its `ToValue` tree: `format u8 (=1) | tag varint
/// | wire value (`pack_rt::encode_wire_value`) of the variant's value with its variant name removed`. The tag is
/// the variant kind's `record <kind> tag=<n>` in the vocabulary's `📡️.protocol.semio` ([`super::protocol_record`]),
/// so the wire never spells the variant name and the protocol file is the only source of tags.
pub mod tagged_value_binary {
    use super::protocol_record;
    use crate::os_dsl::schema::{DslValue, FromValue, ToValue};
    use crate::os_pack::{write_varint_u64, ByteReader};
    use crate::os_spr::ProtocolError;
    use crate::os_store::pack_rt::{decode_wire_value, encode_wire_value};

    pub const OP_BINARY_FORMAT: u8 = 1;

    /// 🧭️ Where the aggregate's `ToValue` tree names its variant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VariantTag {
        /// `#[value(tag = "…")]`, camelCase variant names, with or without `content`.
        Field(&'static str),
        /// Externally tagged, PascalCase variant names: `{"Variant": payload}`.
        Key,
    }

    fn malformed(what: &'static str, offset: u64, detail: String) -> ProtocolError {
        ProtocolError::Malformed { what, offset, detail }
    }

    fn kebab(name: &str) -> String {
        let mut out = String::with_capacity(name.len() + 4);
        for (index, ch) in name.char_indices() {
            if ch.is_ascii_uppercase() {
                if index > 0 {
                    out.push('-');
                }
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn cased(kind: &str, pascal: bool) -> String {
        let mut out = String::with_capacity(kind.len());
        let mut upper = pascal;
        for ch in kind.chars() {
            if ch == '-' {
                upper = true;
            } else if upper {
                out.push(ch.to_ascii_uppercase());
                upper = false;
            } else {
                out.push(ch);
            }
        }
        out
    }

    /// 🏷️ Encodes `op` under its kind's record tag.
    pub fn encode_op<T: ToValue>(protocol: &str, tagging: VariantTag, op: &T) -> Result<Vec<u8>, ProtocolError> {
        let DslValue::Object(mut entries) = op.to_value() else { return Err(malformed("op value", 0, "a mutation aggregate's value must be an object".into())) };
        let (variant, payload) = match tagging {
            VariantTag::Field(key) => {
                let position = entries.iter().position(|(name, _)| name == key).ok_or_else(|| malformed("op value", 0, format!("value carries no `{key}` variant field")))?;
                let (_, variant) = entries.remove(position);
                let DslValue::String(variant) = variant else { return Err(malformed("op value", 0, format!("`{key}` is not a string"))) };
                (variant, DslValue::Object(entries))
            }
            VariantTag::Key => {
                let mut entries = entries.into_iter();
                let (Some((variant, payload)), None) = (entries.next(), entries.next()) else { return Err(malformed("op value", 0, "an externally tagged value must hold exactly one variant".into())) };
                (variant, payload)
            }
        };
        let kind = kebab(&variant);
        let tag = protocol_record::records(protocol).find(|(record, _)| *record == kind).map(|(_, tag)| tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record for '{kind}'")))?;
        let body = encode_wire_value(&payload);
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        write_varint_u64(&mut out, tag);
        out.extend_from_slice(&body);
        Ok(out)
    }

    /// 🏷️ Decodes an op whose tag names its kind's record.
    pub fn decode_op<T: FromValue>(protocol: &str, tagging: VariantTag, bytes: &[u8]) -> Result<T, ProtocolError> {
        let mut reader = ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported op format {format}")));
        }
        let tag = reader.read_varint_u64()?;
        let kind = protocol_record::kind(protocol, tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record with tag {tag}")))?;
        let offset = reader.position();
        let payload = decode_wire_value(&bytes[offset..]).map_err(|error| malformed("op payload", offset as u64, error.to_string()))?;
        let value = match tagging {
            VariantTag::Field(key) => {
                let DslValue::Object(mut entries) = payload else { return Err(malformed("op payload", offset as u64, "payload must be an object".into())) };
                entries.insert(0, (key.to_string(), DslValue::String(cased(kind, false))));
                DslValue::Object(entries)
            }
            VariantTag::Key => DslValue::Object(vec![(cased(kind, true), payload)]),
        };
        T::from_value(value).map_err(|error| malformed("op value", offset as u64, error.to_string()))
    }
}
//#endregion 🏷️TaggedValueRt

//#region 🏷️TaggedTextRt
/// 🏷️ Op frame for a mutation aggregate whose canonical payload is its own `OpText` line `<kind> <args>`:
/// `format u8 (=1) | tag varint | args utf-8`. The tag is the kind's `record <kind> tag=<n>`, so the keyword never
/// travels and the protocol file stays the only source of tags. Used where the `ToValue` tree is lossy.
pub mod tagged_text_binary {
    use super::protocol_record;
    use crate::os_pack::{write_varint_u64, ByteReader};
    use crate::os_spr::ProtocolError;

    pub const OP_BINARY_FORMAT: u8 = 1;

    /// 🏷️ Encodes one printed op line under its keyword's record tag.
    pub fn encode_line(protocol: &str, line: &str) -> Result<Vec<u8>, ProtocolError> {
        let (keyword, args) = line.split_once(' ').unwrap_or((line, ""));
        let tag = protocol_record::records(protocol).find(|(kind, _)| *kind == keyword).map(|(_, tag)| tag).ok_or_else(|| ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record for '{keyword}'") })?;
        let mut out = Vec::with_capacity(args.len() + 3);
        out.push(OP_BINARY_FORMAT);
        write_varint_u64(&mut out, tag);
        out.extend_from_slice(args.as_bytes());
        Ok(out)
    }

    /// 🏷️ Restores the op line whose keyword the tag's record names.
    pub fn decode_line(protocol: &str, bytes: &[u8]) -> Result<String, ProtocolError> {
        let mut reader = ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let tag = reader.read_varint_u64()?;
        let kind = protocol_record::kind(protocol, tag).ok_or_else(|| ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record with tag {tag}") })?;
        let offset = reader.position();
        let args = std::str::from_utf8(&bytes[offset..]).map_err(|error| ProtocolError::Malformed { what: "op args", offset: offset as u64, detail: error.to_string() })?;
        Ok(if args.is_empty() { kind.to_string() } else { format!("{kind} {args}") })
    }
}
//#endregion 🏷️TaggedTextRt

/// 🔍️ Resolves a registered language from `.semio` file bytes (content-derived envelope).
/// Text components (`dsl`/`op`) prefer grammar registrations; binary components (`pack`/`spr`)
/// prefer protocol registrations.
// 🚫️async: E1 pure accessor — see `language_registry` above
pub fn language_for_semio_content(bytes: &[u8]) -> Option<LanguageSpec> {
    let envelope = semio_format::sniff(bytes).ok()?;
    let base = envelope.envelope_id();
    let plugin = envelope.plugin.as_str();
    let artifact = envelope.artifact.as_str();
    match envelope.component {
        semio_format::Component::Dsl => language(&base).or_else(|| language_for_extension(artifact)).or_else(|| language_for_extension(plugin)),
        semio_format::Component::Op => language_for_suffix_candidates(&base, plugin, artifact, "op").or_else(|| {
            language_for_role_extension(LanguageRole::Ops, artifact)
        }),
        semio_format::Component::Pack => language_for_suffix_candidates(&base, plugin, artifact, "pack").or_else(|| language(&base).filter(|s| s.protocol.is_some())),
        semio_format::Component::Spr => language_for_suffix_candidates(&base, plugin, artifact, "spr"),
        _ => None,
    }
}

// 🚫️async: E1 pure accessor — see `language_registry` above
fn language_for_suffix_candidates(base: &str, plugin: &str, artifact: &str, suffix: &str) -> Option<LanguageSpec> {
    language(&format!("{base}.{suffix}")).or_else(|| language(&format!("{plugin}.{suffix}"))).or_else(|| language(&format!("{artifact}.{suffix}"))).or_else(|| language(&format!("{plugin}.{artifact}.{suffix}")))
}
//#endregion 🔖️Idiom

//#region 🔖️TestSupport
/// 🧪️ Round-trip/property helpers every derived (or hand-declared) grammar's own tests
/// call — the facade-level analogue of `crate::os_store::test_support`, scoped to the engine's own laws
/// rather than the VCS store's.
pub mod test_support {
    use super::*;

    /// 🔁️ `parse(print(value)) == value` for a `RecordSpec` and an already-built `RecordValue`.
    pub fn assert_schema_round_trip(value: &RecordValue, spec: &RecordSpec) {
        let printed = print(value, spec, JoinMode::Document);
        let opts = ParseOptions::default();
        let reparsed = parse(&printed, spec, &opts).unwrap_or_else(|e| panic!("reparse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(value, &reparsed, "schema round trip diverged;\nprinted:\n{printed}");
    }

    /// ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)`.
    pub fn assert_idempotent(text: &str, spec: &RecordSpec) {
        let once = crate::os_dsl::schema::canonicalize(text, spec, &ParseOptions::default()).unwrap_or_else(|e| panic!("canonicalize failed: {e}"));
        let twice = crate::os_dsl::schema::canonicalize(&once, spec, &ParseOptions::default()).unwrap_or_else(|e| panic!("second canonicalize failed: {e}"));
        assert_eq!(once, twice, "canonicalization must be idempotent");
    }

    /// 📏️ Document and Inline renders of the same value must parse back to equal values,
    /// and the Inline render must be exactly one line — the newline law, checked generically.
    pub fn assert_document_inline_agree(value: &RecordValue, spec: &RecordSpec) {
        let inline_text = print(value, spec, JoinMode::Inline);
        assert!(!inline_text.contains('\n'), "inline render must be one line: {inline_text:?}");
        let inline_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Inline };
        let reparsed = parse(&inline_text, spec, &inline_opts).unwrap_or_else(|e| panic!("inline reparse failed: {e}\ninline:\n{inline_text}"));
        assert_eq!(value, &reparsed, "Document and Inline renders must parse to the same value");
    }
}
//#endregion 🔖️TestSupport

//#region 🧪️Tests
#[cfg(test)]
#[path="🔢️ieee754/🧪️tests/🦀️.rs"]
mod ieee_payload_tests;

#[cfg(test)]
#[path = "🧪️tests/📦️boxed-fields/🦀️.rs"]
mod boxed_field_tests;

#[cfg(test)]
#[path = "🧪️tests/🔢️checked-integers/🦀️.rs"]
mod checked_integer_tests;

#[cfg(test)]
#[path = "🧪️tests/🏷️protocol-record/🦀️.rs"]
mod protocol_record_tests;

#[cfg(test)]
#[path = "🧪️tests/🧪️hygienic-bindings/🦀️.rs"]
mod hygienic_binding_tests;


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🏷️type/🦀️.rs"]
mod value_type_binding;

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs

SHA-256 `cb02f22a4cd0db5662e4522431b56e94b5f1fe8f6aa603d3763373fec39c4461`; 115534 bytes.

```
//! 🧬️ `dsl_schema` — the data-driven declarative grammar engine: technologies describe their
//! document/op grammar as `RecordSpec`/`Shape` DATA (not code), and this crate parses text against
//! that data into a generic `Cst` (walked by typed binders that `dsl_derive` will generate) and
//! prints it back via a chunk `Writer` that structurally guarantees the newline law: every
//! grammar renders both as multi-line canonical `Document` text and as one space-joined `Inline`
//! line, and both re-parse to the same value.

use semio_framework_dsl::format_f64;
use semio_framework_dsl::lex;
use semio_framework_dsl::parse_f64;
use semio_framework_diagnostic::Limits;
use semio_framework_dsl::SpannedToken;
use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use semio_framework_dsl::TokenClass;
use semio_framework_dsl::TokenKind;
use std::collections::{HashMap, HashSet};

#[path = "🛬️decoding/🦀️.rs"]
mod controlled_decoding;
pub use controlled_decoding::{parse_exact_controlled,parse_expr_text_controlled};

#[path = "🛫️encoding/🦀️.rs"]
mod controlled_encoding;
pub use controlled_encoding::{print_controlled,print_expr_controlled};

#[path = "🏭️producer/🦀️.rs"]
pub mod producer;
pub use producer::{NativeSchemaControl,RecordSpecProducer};

//#region 🔖️Shape
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordLayout {
    /// All fields printed as space-joined `key=value` tokens on one logical unit.
    Inline,
    /// Each field printed as its own line/statement (norm-family style) in Document mode;
    /// collapses to the same space-joined form as `Inline` when rendered in `JoinMode::Inline`.
    Lines,
    /// `<name> = <keyword>(arg1=val1 arg2=val2)` — a named call assignment (the graph-family
    /// construction-chain notation, e.g. `extrude = brep.solid.extrude(profile=w1 axis=v1)`).
    /// Requires exactly one field marked [`FieldSpec::call_name`] (printed before `=`, must have
    /// `Shape::Text`) and `RecordSpec.keyword` set (the dotted call target after `=`, e.g.
    /// `"brep.solid.extrude"` — printed and matched as one token since `.` is `dsl_core`
    /// ident-continue). Every other field prints/parses exactly as it would under `Inline`
    /// (positional bare, keyed as `key=value`), just inside the parens instead of bare after the
    /// keyword.
    Call,
}

/// 🧩️ What one field's value looks like, textually. Covers all 16 grammar-shape
/// primitives found across the 32 hand-rolled implementations this engine replaces.
#[derive(Clone, Debug)]
pub enum Shape {
    Bool,
    Int,
    UInt,
    Float,
    Text,
    Bytes64,
    /// Unit-variant keyword table: `(tag, ordinal)` pairs.
    Enum(Vec<(String, u32)>),
    /// Packed `x,y,z` — `len = Some(n)` enforces arity.
    Tuple(Box<Shape>, Option<usize>),
    /// 🧾️ Bracketed scalar lists `[a b c]`; each direct record item is braced `[ { fields } { } ]`.
    List(Box<Shape>),
    /// 📄️ Inline nested `key=value` run using another record's fields. Direct list items are braced. Lazy for the same
    /// reason `Statements` is: a self-referential `#[derive(DslRecord)]` struct (a field whose type
    /// recurses back to the struct itself, e.g. a dynamic-value type with a nested-dictionary-of-
    /// itself field) would otherwise recurse infinitely just building its own `RecordSpec`.
    Record(RecordSpecProducer),
    /// Wraps the inner shape in `{ ... }`.
    Block(Box<Shape>),
    /// Keyword-dispatched, order-preserving repeated records: `(keyword, spec_fn)` per variant.
    /// `spec_fn` is a zero-capture `fn` pointer, not an eagerly-built `RecordSpec` — a genuinely
    /// self-referential grammar (a recursive block tree whose own variant table contains itself)
    /// would otherwise recurse infinitely just building the table. Calling `(spec_fn.ordinary)()` one level at
    /// a time bottoms out naturally at real documents' finite depth instead.
    Statements(Vec<(String, RecordSpecProducer)>),
    /// `{ key=value ... }` block, keys sorted on canonical print.
    Map(Box<Shape>),
    /// Dynamic JSON-equivalent literal.
    Value,
    /// Structure-of-Arrays columnar table: `key [col:TYPE ...] { v11 v12 ...  v21 v22 ... }`.
    /// `fn() -> RecordSpec` is the SAME lazy self-referential seam `Record`/`Statements` use.
    /// Parses to `FieldValue::List(Vec<FieldValue::Record>)` — identical to `List(Record)` — so
    /// no binder/diff/derive path needs to know a field is a table rather than a verbose AoS list.
    /// Only a record's OWN keyword-prefixed field prints/parses the compact bare SoA form above
    /// (`print_record`/`parse_record_body`'s dedicated lookahead); a `Table` reached any other way
    /// (a table row's own column, a list element, the generic `key=` keyed dispatch) prints/parses
    /// as the bracketed AoS list `[ {...} {...} ]` instead — the bare form has no bracket of its
    /// own to mark where it ends, so it's only safe directly after a record's leading keyword.
    Table(RecordSpecProducer),
    /// Graph endpoint literal: `id[:kind][@port][->|--id2[:kind2][@port2]]{props}`.
    Wire,
    /// A `Shape::Float` refinement: prints/parses with a glued unit suffix (`210GPa`). The value
    /// is stored in `unit`'s declared unit; a compatible alien suffix on parse (`210000MPa`)
    /// converts into it, an incompatible one (wrong dimension) is a parse error. No suffix at all
    /// means the bare number is already in the declared unit.
    Quantity(&'static semio_framework_dsl::UnitSpec),
    /// A `Shape::Quantity` restricted to angle units (`deg`/`rad`/`turn`) — kept as its own variant
    /// (rather than reusing `Quantity` with an angle unit) so `shape_type_name`/table headers can
    /// tell a length from a rotation at a glance (`NUM` vs `QTY` vs `ANG`).
    Angle(&'static semio_framework_dsl::UnitSpec),
    /// A `Shape::Text` refinement: a checked reference to an entity of the named kind (e.g.
    /// `"material"`). Prints/parses identically to `Text` (bare-preferred) — the only difference
    /// is semantic (a paired `FieldSpec.defines` anchor lets `LanguageService::validate` flag a
    /// dangling reference), so it needs no dedicated parse/print arm, only a distinct type name.
    Ref(&'static str),
    /// `@x,y[,z,...]` — a placement/position literal, `dims` coordinates. Value is
    /// `FieldValue::Tuple` (same representation `Shape::Tuple` uses) with exactly `dims` floats.
    Coord(u8),
    /// `^x,y,z` — a unit direction/axis vector, always exactly 3 floats. Value is
    /// `FieldValue::Tuple` — distinct from `Coord(3)` only by its `^` sigil and `DIR` type tag,
    /// so a reader never confuses "where" from "which way".
    Dir,
    /// `WxHxD` (glued, no separator token — see `parse_dim`) — `dims` size components. Value is
    /// `FieldValue::Tuple` with exactly `dims` floats.
    Dim(u8),
    /// `(lo..hi)` or `(lo..hi,step)` — value is `FieldValue::Tuple` of 2 or 3 floats (no dedicated
    /// `RangeValue` type: a range IS a small tuple, just printed with `..` instead of `,` between
    /// the first two elements).
    Range,
    /// `xN` — a bare count/multiplicity literal. Value is `FieldValue::UInt`.
    Count,
    /// `(expr)` — an arithmetic formula literal, always outer-parenthesized. Value is the ONE
    /// genuinely new `FieldValue` variant this engine adds (`FieldValue::Expr`) — everything else
    /// in this Shape reuses an existing representation.
    Expr,
    /// Fenced verbatim text in Document mode (`` ```lang\ncontent\n``` ``), escaped-quoted `Text`
    /// in Inline mode — both parse to the same `FieldValue::Text`, the "Document/Inline agree" law
    /// applied to a shape whose Document form needs raw multi-line content. `lang` is this field's
    /// DECLARED embedded language (e.g. `"jack"`); an authored fence's own lang tag must be empty
    /// or match it.
    Embed(&'static str),
    /// Fence language taken from a sibling Text field named by this key (see `#[dsl(lang_from)]`).
    EmbedFrom(&'static str),
}

#[derive(Clone, Debug)]
pub struct FieldSpec {
    pub id: u16,
    /// Empty for positional-only fields.
    pub key: String,
    /// `Some(n)` = nth positional token right after the keyword, in declaration order among
    /// positional fields.
    pub position: Option<u8>,
    pub shape: Shape,
    pub optional: bool,
    /// Splice a nested record's fields directly into this record (shared doc/op field schemas).
    pub flatten: bool,
    /// Paired with a sibling field's `Shape::Ref(kind)`: this field's value is the canonical id of
    /// an entity of kind `kind`. `None` for every field that isn't such an anchor. Not wire/hash
    /// relevant (LanguageService-only, see `Shape::Ref`'s doc comment) — purely an authoring aid.
    pub defines: Option<&'static str>,
    /// The one field a `RecordLayout::Call` spec prints before `=` and parses as the assignment
    /// target — see [`RecordLayout::Call`]. Always `false` outside a `Call`-layout spec; ignored
    /// (never printed/parsed specially) for any other layout.
    pub is_call_name: bool,
}

impl FieldSpec {
    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn new(id: u16, key: &str, shape: Shape) -> Self {
        Self { id, key: key.to_string(), position: None, shape, optional: false, flatten: false, defines: None, is_call_name: false }
    }

    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn positional(mut self, index: u8) -> Self {
        self.position = Some(index);
        self
    }

    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn flatten(mut self) -> Self {
        self.flatten = true;
        self
    }

    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn defines(mut self, kind: &'static str) -> Self {
        self.defines = Some(kind);
        self
    }

    /// 📛️ Marks this field as the one printed before `=` / parsed as the assignment target
    /// in a `RecordLayout::Call` spec. See [`RecordLayout::Call`].
    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn call_name(mut self) -> Self {
        self.is_call_name = true;
        self
    }
}

#[derive(Clone, Debug)]
pub struct RecordSpec {
    pub keyword: Option<String>,
    pub layout: RecordLayout,
    pub fields: Vec<FieldSpec>,
}

impl RecordSpec {
    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn new(keyword: Option<&str>, layout: RecordLayout, fields: Vec<FieldSpec>) -> Self {
        Self { keyword: keyword.map(|k| k.to_string()), layout, fields }
    }

    /// 🏗️ Same as [`Self::new`] but takes an already-owned keyword — what
    /// `dsl_derive`-generated code builds from a spliced `String` literal.
    // 🚫️async: E1 pure spec builder consumed by E4 fn-pointer slots (Shape::Record) and derive-macro output — see R9
    pub fn new_owned(keyword: Option<String>, layout: RecordLayout, fields: Vec<FieldSpec>) -> Self {
        Self { keyword, layout, fields }
    }
}

pub struct GrammarSpec {
    pub name: String,
    pub root: RecordSpec,
}
//#endregion 🔖️Shape

//#region 🔖️JsonSchema
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema §3.2: the
// gateway's JSON Schema derivation is primarily `ActionArgDef::json_schema()` (manifest-declared
// args); THIS is the fallback for `app_commands!` payload structs whose only declared shape is a
// `RecordSpec` (`#[derive(dsl::DslRecord)]`) — the catalog compiler tags whatever it emits from here
// `x-semio-confidence: "payload"`, not this module's concern.
/// 📐️ JSON Schema 2020-12 for one `Shape` leaf/node — recurses through `Tuple`/`List`/
/// `Record`/`Block`/`Statements`/`Map`/`Table`. `Quantity`/`Angle` carry their unit as
/// `x-semio-unit`; `Ref(kind)` carries the referenced entity kind as `x-semio-ref`; every shape with
/// no native JSON Schema vocabulary (`Bytes64`/`Wire`/`Coord`/`Dir`/`Dim`/`Range`/`Count`/`Expr`/
/// `Embed`/`EmbedFrom`) additionally carries `x-semio-shape` naming the exact `Shape` variant.
pub fn shape_json_schema(shape: &Shape) -> semio_framework_pack_json::Value {
    use semio_framework_pack_json::{object, Value};
    let number_array = |min: u64, max: u64, extra: Option<(&str, &str)>| {
        let mut fields = vec![("type".to_string(), Value::from("array")), ("items".to_string(), object([("type".to_string(), Value::from("number"))])), ("minItems".to_string(), Value::from(min)), ("maxItems".to_string(), Value::from(max))];
        if let Some((key, value)) = extra {
            fields.push((key.to_string(), Value::from(value)));
        }
        object(fields)
    };
    match shape {
        Shape::Bool => object([("type".to_string(), Value::from("boolean"))]),
        Shape::Int => object([("type".to_string(), Value::from("integer"))]),
        Shape::UInt => object([("type".to_string(), Value::from("integer")), ("minimum".to_string(), Value::from(0u64))]),
        Shape::Float => object([("type".to_string(), Value::from("number"))]),
        Shape::Text => object([("type".to_string(), Value::from("string"))]),
        Shape::Bytes64 => object([("type".to_string(), Value::from("string")), ("contentEncoding".to_string(), Value::from("base64")), ("x-semio-shape".to_string(), Value::from("bytes64"))]),
        Shape::Enum(variants) => object([("type".to_string(), Value::from("string")), ("enum".to_string(), Value::Array(variants.iter().map(|(tag, _)| Value::from(tag.clone())).collect()))]),
        Shape::Tuple(inner, len) => {
            let items = shape_json_schema(inner);
            let mut fields = vec![("type".to_string(), Value::from("array")), ("items".to_string(), items)];
            if let Some(len) = len {
                fields.push(("minItems".to_string(), Value::from(*len as u64)));
                fields.push(("maxItems".to_string(), Value::from(*len as u64)));
            }
            object(fields)
        }
        Shape::List(inner) => {
            let items = shape_json_schema(inner);
            object([("type".to_string(), Value::from("array")), ("items".to_string(), items)])
        }
        Shape::Record(spec_fn) => record_spec_json_schema(&(spec_fn.ordinary)()),
        Shape::Block(inner) => shape_json_schema(inner),
        Shape::Statements(variants) => {
            let mut one_of = Vec::with_capacity(variants.len());
            for (keyword, spec_fn) in variants {
                let mut entry = record_spec_json_schema(&(spec_fn.ordinary)());
                if let Value::Object(map) = &mut entry {
                    map.insert("x-semio-keyword", Value::from(keyword.clone()));
                }
                one_of.push(entry);
            }
            object([("type".to_string(), Value::from("array")), ("items".to_string(), object([("oneOf".to_string(), Value::Array(one_of))]))])
        }
        Shape::Map(inner) => {
            let additional_properties = shape_json_schema(inner);
            object([("type".to_string(), Value::from("object")), ("additionalProperties".to_string(), additional_properties)])
        }
        Shape::Value => Value::Object(semio_framework_pack_json::Object::new()),
        Shape::Table(spec_fn) => {
            let items = record_spec_json_schema(&(spec_fn.ordinary)());
            object([("type".to_string(), Value::from("array")), ("items".to_string(), items)])
        }
        Shape::Wire => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("wire"))]),
        Shape::Quantity(unit) => object([("type".to_string(), Value::from("number")), ("x-semio-unit".to_string(), Value::from(unit.symbol))]),
        Shape::Angle(unit) => object([("type".to_string(), Value::from("number")), ("x-semio-unit".to_string(), Value::from(unit.symbol)), ("x-semio-shape".to_string(), Value::from("angle"))]),
        Shape::Ref(kind) => object([("type".to_string(), Value::from("string")), ("x-semio-ref".to_string(), Value::from(*kind))]),
        Shape::Coord(dims) => number_array(*dims as u64, *dims as u64, Some(("x-semio-shape", "coord"))),
        Shape::Dir => number_array(3, 3, Some(("x-semio-shape", "dir"))),
        Shape::Dim(dims) => number_array(*dims as u64, *dims as u64, Some(("x-semio-shape", "dim"))),
        Shape::Range => number_array(2, 3, Some(("x-semio-shape", "range"))),
        Shape::Count => object([("type".to_string(), Value::from("integer")), ("minimum".to_string(), Value::from(0u64)), ("x-semio-shape".to_string(), Value::from("count"))]),
        Shape::Expr => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("expr"))]),
        Shape::Embed(lang) => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("embed")), ("x-semio-lang".to_string(), Value::from(*lang))]),
        Shape::EmbedFrom(key) => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("embed")), ("x-semio-lang-from".to_string(), Value::from(*key))]),
    }
}

/// 📐️ JSON Schema 2020-12 object for one `RecordSpec` — one property per `FieldSpec.key`
/// (positional-only fields, whose `key` is empty, are omitted — no name to key a JSON object
/// property on), `flatten`ed nested-record fields splice their own fields into THIS SAME properties
/// map rather than nesting, mirroring what `flatten` means at parse/print altitude. `required` lists
/// every non-`optional`, non-empty-key field.
pub fn record_spec_json_schema(spec: &RecordSpec) -> semio_framework_pack_json::Value {
    use semio_framework_pack_json::{Object, Value};
    let mut properties = Object::new();
    let mut required: Vec<Value> = Vec::new();
    collect_record_spec_properties(spec, &mut properties, &mut required);
    let mut map = Object::new();
    map.insert("type", Value::from("object"));
    map.insert("properties", Value::Object(properties));
    if let Some(keyword) = &spec.keyword {
        map.insert("x-semio-keyword", Value::from(keyword.clone()));
    }
    if !required.is_empty() {
        map.insert("required", Value::Array(required));
    }
    Value::Object(map)
}

fn collect_record_spec_properties(spec: &RecordSpec, properties: &mut semio_framework_pack_json::Object, required: &mut Vec<semio_framework_pack_json::Value>) {
    use semio_framework_pack_json::Value;
    for field in &spec.fields {
        if field.flatten {
            if let Shape::Record(spec_fn) = &field.shape {
                collect_record_spec_properties(&(spec_fn.ordinary)(), properties, required);
                continue;
            }
        }
        if field.key.is_empty() {
            continue;
        }
        properties.insert(field.key.clone(), shape_json_schema(&field.shape));
        if !field.optional {
            required.push(Value::from(field.key.clone()));
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️json-schema/🦀️.rs"]
mod json_schema_tests;
//#endregion 🔖️JsonSchema

//#region 🔖️Value
/// 🌱️ `DslValue` and its serde bridge are owned by `🧰️framework/🔨️modules/🌱️value` and reach the
/// tree through the replication crate; the record/field/wire types below build on it.
///
/// `ToValue`/`FromValue` are the first-party `Serialize`/`DeserializeOwned` replacement
/// `crate::mutation::MutationDiff`/`Mutation` now bound on (see `🌱️value/🔁️codec`) — re-exported
/// here so `#[derive(ToValue, FromValue)]` (`semio-framework-value-derive`) generated code, which
/// runs inside plugin crates, can address them at the stable `::semio_framework_os_kernel::…`
/// path every plugin already depends on, exactly like `Mutation`/`MutationLeafDescriptor` do for
/// `#[derive(Mutations)]`. NOTE for callers: `DslField::to_value`/`from_value` (above, over
/// `FieldValue`) share these method names — a type deriving both `DslRecord`/`DslScalar` AND
/// `ToValue`/`FromValue` must disambiguate with UFCS (`<T as value::ToValue>::to_value(&x)`) at
/// any call site where both traits are in scope.
pub use protocol::value::{edit_through_value, ordered, DecodedValue, DslValue, FromValue, NativeEncodeControl, Number, ToValue, ValueEdit, ValueError, ValueRefusalKind, ValueShape};

/// 🕸️ One endpoint (and optional edge) of a wire-literal.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WireNode {
    pub id: String,
    pub kind: Option<String>,
    pub port: Option<String>,
}

/// 🏷️ Optional id/kind label on a wire edge (`-[e1:Connection]->` / fused `-e1:Connection>`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WireEdgeLabel {
    pub id: Option<String>,
    pub kind: Option<String>,
}

impl WireEdgeLabel {
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.kind.is_none()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WireValue {
    pub from: WireNode,
    /// `Some((directed, to))` if this line describes an edge, `None` for a bare node declaration.
    pub edge: Option<(bool, WireNode)>,
    pub edge_label: WireEdgeLabel,
    pub properties: DslValue,
}

/// 🌳️ The parsed representation of one field's value — what a typed binder converts
/// to/from a concrete Rust value. Doubles as this v1 engine's "Cst": simplified (semantic, not a
/// full lossless syntax tree) but sufficient for round-tripping, diagnostics, and highlighting;
/// a real green/red tree can replace it later behind the same `parse`/`Writer` API.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    Text(String),
    Bytes64(Vec<u8>),
    Enum(u32),
    Tuple(Vec<FieldValue>),
    List(Vec<FieldValue>),
    Record(RecordValue),
    Block(Box<FieldValue>),
    Statements(Vec<(String, RecordValue)>),
    Map(Vec<(String, FieldValue)>),
    Value(DslValue),
    Wire(WireValue),
    Expr(ExprValue),
    Absent,
}

#[path = "🧩️record/🗂️fields/🦀️.rs"]
mod record_fields;
pub use record_fields::RecordFields;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct RecordValue {
    pub fields: RecordFields,
}

impl RecordValue {
    // 🚫️async: E1 pure map lookup, consumed by `Iterator::any`/`Option::and_then` sync closures
    // (`print_record_fields`) and by dozens of `assert_eq!(value.get(id), ...)` test call sites
    // that compare its result directly (never ``ed) — see R9
    pub fn get(&self, id: u16) -> Option<&FieldValue> {
        self.fields.get(&id)
    }
}

/// 🌳️ Alias naming the parse product per the engine's design vocabulary.
pub type Cst = RecordValue;
//#endregion 🔖️Value

//#region 🔖️Expr
/// ➕️ Arithmetic operators `Shape::Expr` supports — standard left-associative precedence
/// (`*`/`/` bind tighter than `+`/`-`), plus a call form for named functions (`min(a, b)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ExprOp {
    // 🚫️async: E1 pure, consumed by `print_expr_prec` (forced sync — its `Call` arm feeds a
    // `Display`-formatted `Iterator::map(...).join(...)` closure chain, R9) and inlined directly
    // into `format!` args elsewhere in this impl — see R9
    fn precedence(self) -> u8 {
        match self {
            ExprOp::Add | ExprOp::Sub => 1,
            ExprOp::Mul | ExprOp::Div => 2,
        }
    }

    // 🚫️async: E1 pure, same R9 chain as `precedence` above
    fn symbol(self) -> &'static str {
        match self {
            ExprOp::Add => "+",
            ExprOp::Sub => "-",
            ExprOp::Mul => "*",
            ExprOp::Div => "/",
        }
    }
}

/// 🧮️ The parsed body of a `Shape::Expr` field — a small formula AST, e.g.
/// `1.35*G + 1.5*Q` parses to `Binary(Add, Binary(Mul, Num(1.35), Var("G")), Binary(Mul,
/// Num(1.5), Var("Q")))`. Deliberately NOT a general-purpose scripting language (no assignment, no
/// control flow, no boolean logic) — it's a formula literal, one notch above a bare number.
#[derive(Clone, Debug, PartialEq)]
pub enum ExprValue {
    Num(f64),
    /// A snake_case reference to a sibling field/symbol, resolved by the consuming technology
    /// (e.g. a norm calc-sheet's own `given`/prior `clause` definitions) — this engine only
    /// parses/prints the name, it never evaluates it.
    Var(String),
    Neg(Box<ExprValue>),
    Binary(ExprOp, Box<ExprValue>, Box<ExprValue>),
    Call(String, Vec<ExprValue>),
}
//#endregion 🔖️Expr

//#region 🔖️Cursor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceMode {
    Document,
    Inline,
}

struct Cursor {
    tokens: Vec<SpannedToken>,
    pos: usize,
    limits: Limits,
}

impl Cursor {
    // 🚫️async: E1 pure in-memory cursor consumed by `Iterator::position` sync closures (`:1345`, `:1354` via `at_keyword`) — see R9.
    // The whole impl block is one call graph (`peek`/`peek_at`/`span`/`advance`/`expect`/`at_attr_key`/`at_keyword` all call each
    // other with no suspension point ever possible), so the language barrier on `at_keyword` propagates to every method here.
    //
    // `SourceMode` no longer participates in parsing (its only consumer, `RawLines`, is gone —
    // `Shape::Text` now accepts `Ident|Text` identically regardless of Document/Inline); it stays
    // a `ParseOptions`/`parse` public-API distinction only, still meaningful to callers choosing
    // between `dsl::__rt::parse_document_record`/`parse_inline_record`.
    fn new(tokens: Vec<SpannedToken>, limits: Limits) -> Self {
        let tokens: Vec<SpannedToken> = tokens.into_iter().filter(|t| !t.kind.is_trivia()).collect();
        Self { tokens, pos: 0, limits }
    }

    fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn peek_at(&self, offset: usize) -> &SpannedToken {
        let idx = (self.pos + offset).min(self.tokens.len() - 1);
        &self.tokens[idx]
    }

    fn span(&self) -> TextSpan {
        self.peek().span
    }

    fn advance(&mut self) -> SpannedToken {
        let token = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, kind: TokenKind) -> Result<SpannedToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected {:?}, found {:?} '{}'", kind, self.peek().kind, self.peek().text.as_str()),self.span()))
        }
    }

    /// 🔎️ Whether the next token is an `Ident` that is followed by `=` — the LL(2)
    /// lookahead that makes the grammar newline-insensitive: a bare ident followed by `=` is
    /// always a `key=value` attribute, never the start of a new statement.
    fn at_attr_key(&self) -> Option<String> {
        if self.peek().kind == TokenKind::Ident && self.peek_at(1).kind == TokenKind::Equals {
            Some(self.peek().text.as_str().to_string())
        } else {
            None
        }
    }

    fn at_keyword(&self, keyword: &str) -> bool {
        self.peek().kind == TokenKind::Ident && self.peek().text.as_str().as_ref() == keyword
    }

    fn dynamic_attr_key(&self) -> Result<Option<String>, TextError> {
        if self.peek().kind == TokenKind::Text && self.peek_at(1).kind == TokenKind::Equals {
            semio_framework_dsl::unescape_text(&self.peek().text.as_str(), false).map(Some).map_err(|message| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message,self.span()))
        } else {
            Ok(self.at_attr_key())
        }
    }
}
//#endregion 🔖️Cursor

//#region 🔖️Parser
pub struct ParseOptions {
    pub limits: Limits,
    pub mode: SourceMode,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self { limits: Limits::default(), mode: SourceMode::Document }
    }
}

/// ✂️ The structural seam between lexing and parsing: everything downstream of a token
/// vector is grammar-only and needs no raw source bytes (the parser is token-only — no shape
/// still consumes verbatim source text the way the deleted `RawLines` shape once did). Exists so
/// a caller that already has tokens (e.g. an incremental relexer) can skip `parse`'s own lex pass.
pub fn parse_tokens(tokens: Vec<SpannedToken>, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let mut cursor = Cursor::new(tokens, opts.limits);
    parse_record_body(&mut cursor, spec, 0)
}

pub fn parse(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let tokens = lex(text, &opts.limits, false)?;
    parse_tokens(tokens, spec, opts)
}

/// 🛑️ Parses one terminal record and rejects every token outside its schema-owned body.
pub fn parse_exact(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let mut cursor = Cursor::new(lex(text, &opts.limits, false)?, opts.limits);
    let record = parse_record_body(&mut cursor, spec, 0)?;
    cursor.expect(TokenKind::Eof)?;
    Ok(record)
}

fn ident_like_text(token: &SpannedToken) -> String {
    token.text.as_str().to_string()
}

fn parse_scalar(cursor: &mut Cursor, shape: &Shape) -> Result<FieldValue, TextError> {
    match shape {
        Shape::Bool => {
            let token = cursor.expect(TokenKind::Ident)?;
            match token.text.as_str().as_ref() {
                "true" => Ok(FieldValue::Bool(true)),
                "false" => Ok(FieldValue::Bool(false)),
                other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected 'true' or 'false', found '{other}'"),token.span)),
            }
        }
        Shape::Int => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: i64 = token.text.as_str().parse().map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("invalid integer '{}'", token.text.as_str()),token.span))?;
            Ok(FieldValue::Int(value))
        }
        Shape::UInt => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: u64 = token.text.as_str().parse().map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("invalid unsigned integer '{}'", token.text.as_str()),token.span))?;
            Ok(FieldValue::UInt(value))
        }
        Shape::Float => {
            let is_float_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
            if !is_float_token {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a float, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
            }
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Float(value))
        }
        Shape::Text => parse_scalar_text(cursor),
        Shape::Bytes64 => {
            let token = cursor.expect(TokenKind::Text)?;
            let bytes = base64_decode(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Bytes64(bytes))
        }
        Shape::Enum(variants) => {
            let token = cursor.expect(TokenKind::Ident)?;
            let text = token.text.as_str();
            variants.iter().find(|(tag, _)| tag == text.as_ref()).map(|(_, ordinal)| FieldValue::Enum(*ordinal)).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown enum tag '{text}'"),token.span))
        }
        Shape::Quantity(declared) | Shape::Angle(declared) => parse_quantity(cursor, declared),
        Shape::Ref(_) => parse_scalar_text(cursor),
        Shape::Embed(declared_lang) => parse_embed(cursor, declared_lang),
        Shape::EmbedFrom(_) => Err(TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"EmbedFrom field must be parsed in record context",cursor.span())),
        Shape::Count => {
            if cursor.peek().kind != TokenKind::Ident {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a count literal like 'x24', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
            }
            let token = cursor.advance();
            let text = token.text.as_str();
            let digits = text.strip_prefix('x').ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a count literal like 'x24', found '{text}'"),token.span))?;
            let value: u64 = digits.parse().map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("invalid count literal 'x{digits}'"),token.span))?;
            Ok(FieldValue::UInt(value))
        }
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,format!("shape {other:?} is not a scalar"),cursor.span())),
    }
}

/// 🧮️ Precedence-climbing entry point for `Shape::Expr`'s body (called with the caller's
/// outer `(`/`)` already consumed). `min_prec` is the lowest operator precedence this call is
/// willing to keep consuming at — the standard technique for turning a flat token stream into a
/// precedence-correct tree without a separate tokenize-then-shunting-yard pass.
fn parse_expr(cursor: &mut Cursor, min_prec: u8) -> Result<ExprValue, TextError> {
    let lhs = parse_expr_unary(cursor)?;
    parse_expr_continue(cursor, min_prec, lhs)
}

/// 🧮️ The loop body of `parse_expr`, factored out so the glued-negative-number case below
/// can re-enter it with an ALREADY-PARSED left operand instead of calling `parse_expr_unary` again
/// (which would re-consume nothing, since the token was already consumed to build that operand).
fn parse_expr_continue(cursor: &mut Cursor, min_prec: u8, mut lhs: ExprValue) -> Result<ExprValue, TextError> {
    loop {
        // The shared lexer glues a leading `-` onto an immediately-following digit as ONE negative
        // number token (`y=-2`'s existing, load-bearing behavior — see dsl_core's lexer) — so
        // `10-2` lexes as `Int(10), Int(-2)`, not `Int(10), Minus, Int(2)`. Detect that shape here
        // and reinterpret it as `Sub` with a positive right operand, rather than requiring authors
        // to always space out `-` (canonical PRINT output always does; hand-written input may not).
        let glued_negative = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) && cursor.peek().text.as_str().starts_with('-');
        let (op, prec) = if glued_negative {
            (ExprOp::Sub, ExprOp::Sub.precedence())
        } else {
            match cursor.peek().kind {
                TokenKind::Plus => (ExprOp::Add, ExprOp::Add.precedence()),
                TokenKind::Minus => (ExprOp::Sub, ExprOp::Sub.precedence()),
                TokenKind::Star => (ExprOp::Mul, ExprOp::Mul.precedence()),
                TokenKind::Slash => (ExprOp::Div, ExprOp::Div.precedence()),
                _ => break,
            }
        };
        if prec < min_prec {
            break;
        }
        let rhs = if glued_negative {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            parse_expr_continue(cursor, prec + 1, ExprValue::Num(-value))?
        } else {
            cursor.advance();
            parse_expr(cursor, prec + 1)?
        };
        lhs = ExprValue::Binary(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_expr_unary(cursor: &mut Cursor) -> Result<ExprValue, TextError> {
    if cursor.peek().kind == TokenKind::Minus {
        cursor.advance();
        return Ok(ExprValue::Neg(Box::new(parse_expr_unary(cursor)?)));
    }
    parse_expr_primary(cursor)
}

fn parse_expr_primary(cursor: &mut Cursor) -> Result<ExprValue, TextError> {
    match cursor.peek().kind {
        TokenKind::Float | TokenKind::Int => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(ExprValue::Num(value))
        }
        TokenKind::Ident | TokenKind::Text => {
            let token = cursor.advance();
            let name = if token.kind==TokenKind::Text{semio_framework_dsl::unescape_text(&token.text.as_str(),false).map_err(|error|TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error,token.span))?}else{ident_like_text(&token)};
            if cursor.peek().kind == TokenKind::LParen {
                cursor.advance();
                let mut args = Vec::new();
                if cursor.peek().kind != TokenKind::RParen {
                    loop {
                        args.push(parse_expr(cursor, 0)?);
                        if cursor.peek().kind == TokenKind::Comma {
                            cursor.advance();
                            continue;
                        }
                        break;
                    }
                }
                cursor.expect(TokenKind::RParen)?;
                Ok(ExprValue::Call(name, args))
            } else {
                Ok(ExprValue::Var(name))
            }
        }
        TokenKind::LParen => {
            cursor.advance();
            let inner = parse_expr(cursor, 0)?;
            cursor.expect(TokenKind::RParen)?;
            Ok(inner)
        }
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a number, variable, or '(', found {other:?} '{}'", cursor.peek().text.as_str()),cursor.span())),
    }
}

/// 🧮️ Standalone entry point for parsing a bare expression body (no surrounding `(`/`)`,
/// unlike `Shape::Expr`'s own field-value grammar) — what `pack_value`'s decoder calls to turn the
/// canonical string it stored back into an `ExprValue`, since decode has no `Cursor` of its own.
pub fn parse_expr_text(text: &str) -> Result<ExprValue, TextError> {
    let tokens = lex(text, &Limits::default(), false)?;
    let mut cursor = Cursor::new(tokens, Limits::default());
    let value = parse_expr(&mut cursor, 0)?;
    cursor.expect(TokenKind::Eof)?;
    Ok(value)
}

/// 🎨️ Canonical `Shape::Expr` printer. Parenthesizes the minimum necessary to guarantee
/// `parse_expr(print_expr(e)) == e` for EVERY tree shape (not just canonically-left-nested ones):
/// a `Binary` right operand is parenthesized whenever its own precedence isn't STRICTLY higher
/// than the parent's (so even a commutative `a+(b+c)` keeps its parens — losing them would
/// reparse as the structurally different `(a+b)+c`), and a left operand only when strictly lower
/// (left-associativity already makes equal precedence safe there).
// 🚫️async: E1 pure AST pretty-printer — the `Call` arm's `Iterator::map(...).join(...)` recurses
// through `print_expr_prec` inside a sync closure, and both are inlined directly into `format!`
// args elsewhere in this fn, which requires `Display`, not `Future` — see R9
pub fn print_expr(expr: &ExprValue) -> String {
    print_expr_prec(expr, 0)
}

fn print_expr_prec(expr: &ExprValue, min_prec: u8) -> String {
    let (body, own_prec) = match expr {
        ExprValue::Num(v) => (format_f64(*v), 255),
        ExprValue::Var(name) => (expression_name(name), 255),
        ExprValue::Call(name, args) => {
            let joined = args.iter().map(|a| print_expr_prec(a, 0)).collect::<Vec<_>>().join(", ");
            (format!("{}({joined})",expression_name(name)), 255)
        }
        // min_prec=4 is higher than every binary op (max 2) and Neg's own rank (3), so a nested
        // Binary OR another Neg always gets parenthesized — the latter specifically avoids ever
        // printing adjacent `--`, which would relex as `DashArrow`, not two `Minus` tokens.
        ExprValue::Neg(inner) => (format!("-{}", print_expr_prec(inner, 4)), 3),
        ExprValue::Binary(op, l, r) => {
            let prec = op.precedence();
            let l_text = print_expr_prec(l, prec);
            let r_text = print_expr_prec(r, prec + 1);
            (format!("{l_text} {} {r_text}", op.symbol()), prec)
        }
    };
    if own_prec < min_prec {
        format!("({body})")
    } else {
        body
    }
}

fn expression_name(name:&str)->String{if semio_framework_dsl::is_bare_ident(name){name.to_string()}else{format!("\"{}\"",semio_framework_dsl::escape_text(name))}}

/// 📛️ `Shape::Text`'s own body, factored out so `Shape::Ref` (identical grammar, distinct
/// type only) can share it without a redundant match arm duplicating both branches.
fn parse_scalar_text(cursor: &mut Cursor) -> Result<FieldValue, TextError> {
    match cursor.peek().kind {
        TokenKind::Text => {
            let token = cursor.advance();
            let text = semio_framework_dsl::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Text(text))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            Ok(FieldValue::Text(ident_like_text(&token)))
        }
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Text, found {other:?} '{}'", cursor.peek().text.as_str()),cursor.span())),
    }
}

/// 🗣️ `Shape::Embed`'s parse: a `Fence` token (Document mode — see `dsl_core`'s lexer for
/// the `lang\u{0}content` encoding) with an empty or matching lang tag, OR anything
/// `parse_scalar_text` already accepts (Inline mode's escaped-quoted fallback) — both converge on
/// the same `FieldValue::Text`, which is what makes Document/Inline renders agree.
fn parse_embed(cursor: &mut Cursor, declared_lang: &str) -> Result<FieldValue, TextError> {
    if cursor.peek().kind == TokenKind::Fence {
        let token = cursor.advance();
        let raw = token.text.as_str();
        let (lang, content) = raw.split_once('\u{0}').ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"malformed fence token (missing separator)",token.span))?;
        if !lang.is_empty() && !declared_lang.is_empty() && lang != declared_lang {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("fence declares lang '{lang}', field expects '{declared_lang}'"),token.span));
        }
        return Ok(FieldValue::Text(content.to_string()));
    }
    parse_scalar_text(cursor)
}

fn sibling_text_field<'a>(record: &'a RecordValue, spec: &RecordSpec, lang_key: &str) -> Option<&'a str> {
    let field = spec.fields.iter().find(|f| f.key == lang_key)?;
    match record.get(field.id)? {
        FieldValue::Text(text) => Some(text.as_str()),
        _ => None,
    }
}

fn parse_field_shape(cursor: &mut Cursor, field: &FieldSpec, spec: &RecordSpec, record: &RecordValue, depth: usize) -> Result<FieldValue, TextError> {
    if let Shape::EmbedFrom(lang_key) = &field.shape {
        let declared = sibling_text_field(record, spec, lang_key).unwrap_or("");
        return parse_embed(cursor, declared);
    }
    parse_shape(cursor, &field.shape, depth)
}

/// 📐️ Shared parse for `Shape::Quantity`/`Shape::Angle`: a number, optionally followed by a
/// GLUED (no whitespace between — the lexer already ends a numeric token exactly where the next
/// `Ident` token begins for input like `210GPa`) unit-symbol ident. No suffix means the number is
/// already expressed in `declared`'s unit; a suffix converts, erroring if the dimensions differ.
fn parse_quantity(cursor: &mut Cursor, declared: &'static semio_framework_dsl::UnitSpec) -> Result<FieldValue, TextError> {
    let is_number_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
    if !is_number_token {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a quantity, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    let number_token = cursor.advance();
    let value = parse_f64(&number_token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,number_token.span))?;
    let suffix = cursor.peek();
    if suffix.kind == TokenKind::Ident && suffix.byte_range.0 == number_token.byte_range.1 {
        let suffix_token = cursor.advance();
        let symbol = suffix_token.text.as_str().to_string();
        let suffix_unit = semio_framework_dsl::unit_by_symbol(&symbol).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown unit '{symbol}'"),suffix_token.span))?;
        let converted = if value.is_nan(){if suffix_unit.dimension!=declared.dimension{return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unit '{symbol}' is not compatible with expected unit '{}'",declared.symbol),suffix_token.span));}value}else{semio_framework_dsl::convert(value,suffix_unit,declared).ok_or_else(||TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unit '{symbol}' is not compatible with expected unit '{}'",declared.symbol),suffix_token.span))?};
        Ok(FieldValue::Float(converted))
    } else {
        Ok(FieldValue::Float(value))
    }
}

/// 🔢️ Reads a complete numeric token for coordinate, direction, dimension and range components.
fn parse_plain_number(cursor: &mut Cursor) -> Result<f64, TextError> {
    if !(matches!(cursor.peek().kind,TokenKind::Float|TokenKind::Int)||(cursor.peek().kind==TokenKind::Ident&&matches!(cursor.peek().text.as_str().as_ref(),"nan"|"inf"|"-inf"))) {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a number, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    let token = cursor.advance();
    parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))
}

/// 📍️ Shared body for `Shape::Coord`/`Shape::Dir`: a fixed-arity comma-separated run of
/// plain numbers, with no delimiter of its own (the caller already consumed the `@`/`^` sigil).
fn parse_fixed_number_tuple(cursor: &mut Cursor, arity: usize, what: &str) -> Result<FieldValue, TextError> {
    let mut items = Vec::with_capacity(arity);
    loop {
        items.push(FieldValue::Float(parse_plain_number(cursor)?));
        if items.len() == arity {
            break;
        }
        cursor.expect(TokenKind::Comma)?;
    }
    if cursor.peek().kind == TokenKind::Comma {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("{what} literal expects exactly {arity} components"),cursor.span()));
    }
    Ok(FieldValue::Tuple(items))
}

/// 📏️ `Shape::Dim`'s `WxHxD` grammar: the FIRST number is an ordinary `Float|Int` token;
/// every number after it is glued (no whitespace, no comma) onto an `x`-prefixed ident — the
/// lexer has no notion of a bare `x` operator (digits/`.` are ident-continue, so `x0.12x0.24`
/// lexes as ONE `Ident` token), so this splits that single glued token on `x` itself rather than
/// looping token-by-token the way `parse_fixed_number_tuple` does.
fn parse_dim(cursor: &mut Cursor, dims: usize) -> Result<FieldValue, TextError> {
    let first_token = cursor.peek().clone();
    let first = parse_plain_number(cursor)?;
    let mut items = vec![FieldValue::Float(first)];
    if dims > 1 {
        let suffix = cursor.peek();
        if suffix.kind != TokenKind::Ident || suffix.byte_range.0 != first_token.byte_range.1 {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("dimension literal expects {dims} components glued with 'x' (e.g. '2x3'), found only one"),cursor.span()));
        }
        let suffix_token = cursor.advance();
        let suffix_text = suffix_token.text.as_str();
        let parts: Vec<&str> = suffix_text.split('x').collect();
        // `"x0.12x0.24".split('x')` yields `["", "0.12", "0.24"]` — the leading empty piece is the
        // text before the first `x`, which is always empty since the suffix itself starts with it.
        if parts.first() != Some(&"") || parts.len() != dims {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("dimension literal expects {dims} components glued with 'x', found '{}{}'", format_f64(first), suffix_text),suffix_token.span));
        }
        for part in &parts[1..] {
            let value = parse_f64(part).map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("invalid dimension component '{part}'"),suffix_token.span))?;
            items.push(FieldValue::Float(value));
        }
    }
    Ok(FieldValue::Tuple(items))
}

fn parse_shape(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match shape {
        Shape::Bool | Shape::Int | Shape::UInt | Shape::Float | Shape::Text | Shape::Bytes64 | Shape::Enum(_) | Shape::Quantity(_) | Shape::Angle(_) | Shape::Ref(_) | Shape::Count | Shape::Embed(_) => parse_scalar(cursor, shape),
        Shape::EmbedFrom(_) => Err(TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"EmbedFrom field must be parsed in record context",cursor.span())),
        Shape::Coord(dims) => {
            cursor.expect(TokenKind::At)?;
            parse_fixed_number_tuple(cursor, *dims as usize, "coordinate")
        }
        Shape::Dir => {
            cursor.expect(TokenKind::Caret)?;
            parse_fixed_number_tuple(cursor, 3, "direction")
        }
        Shape::Dim(dims) => parse_dim(cursor, *dims as usize),
        Shape::Range => {
            cursor.expect(TokenKind::LParen)?;
            let lo = parse_plain_number(cursor)?;
            cursor.expect(TokenKind::DotDot)?;
            let hi = parse_plain_number(cursor)?;
            let mut items = vec![FieldValue::Float(lo), FieldValue::Float(hi)];
            if cursor.peek().kind == TokenKind::Comma {
                cursor.advance();
                items.push(FieldValue::Float(parse_plain_number(cursor)?));
            }
            cursor.expect(TokenKind::RParen)?;
            Ok(FieldValue::Tuple(items))
        }
        Shape::Expr => {
            cursor.expect(TokenKind::LParen)?;
            let value = parse_expr(cursor, 0)?;
            cursor.expect(TokenKind::RParen)?;
            Ok(FieldValue::Expr(value))
        }
        Shape::Tuple(elem, len) => {
            let mut items = Vec::new();
            loop {
                items.push(parse_shape(cursor, elem, depth + 1)?);
                if cursor.peek().kind == TokenKind::Comma {
                    cursor.advance();
                    continue;
                }
                break;
            }
            if let Some(expected_len) = len {
                if items.len() != *expected_len {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("tuple expects {} elements, found {}", expected_len, items.len()),cursor.span()));
                }
            }
            Ok(FieldValue::Tuple(items))
        }
        Shape::List(elem) => {
            cursor.expect(TokenKind::LBracket)?;
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                let pos_before = cursor.pos;
                let value = if let Shape::Record(make) = elem.as_ref() {
                    cursor.expect(TokenKind::LBrace)?;
                    let record = parse_record_body(cursor, &(make.ordinary)(), depth + 1)?;
                    cursor.expect(TokenKind::RBrace)?;
                    FieldValue::Record(record)
                } else {
                    parse_shape(cursor, elem, depth + 1)?
                };
                items.push(value);
                if cursor.pos == pos_before {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("list element made no progress at {:?} '{}' — likely an unrecognized field key", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
                }
                cursor.limits.check_nodes(items.len(), cursor.span())?;
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(FieldValue::List(items))
        }
        Shape::Record(spec_fn) => Ok(FieldValue::Record(parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?)),
        Shape::Block(inner) => {
            cursor.expect(TokenKind::LBrace)?;
            let value = parse_shape(cursor, inner, depth + 1)?;
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Block(Box::new(value)))
        }
        Shape::Statements(variants) => {
            let mut out = Vec::new();
            while let Some(keyword) = current_keyword(cursor) {
                let Some((_, spec_fn)) = variants.iter().find(|(kw, _)| kw == &keyword) else { break };
                // `parse_record_body` consumes the keyword itself (see its own check below); we
                // only peek here to decide whether this token starts a known variant at all.
                let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
                out.push((keyword, record));
                cursor.limits.check_nodes(out.len(), cursor.span())?;
                if cursor.peek().kind == TokenKind::RBrace || cursor.peek().kind == TokenKind::Eof {
                    break;
                }
            }
            Ok(FieldValue::Statements(out))
        }
        Shape::Map(inner) => {
            cursor.expect(TokenKind::LBrace)?;
            let mut entries = Vec::new();
            while let Some(key) = cursor.dynamic_attr_key()? {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                let record = matches!(inner.as_ref(), Shape::Record(_));
                if record { cursor.expect(TokenKind::LBrace)?; }
                let value = parse_shape(cursor, inner, depth + 1)?;
                if record { cursor.expect(TokenKind::RBrace)?; }
                entries.push((key, value));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Map(entries))
        }
        Shape::Value => Ok(FieldValue::Value(parse_dsl_value(cursor, depth + 1)?)),
        Shape::Table(spec_fn) => {
            validate_table_columns(&(spec_fn.ordinary)())?;
            parse_table_list(cursor, *spec_fn, depth)
        }
        Shape::Wire => Ok(FieldValue::Wire(parse_wire(cursor)?)),
    }
}

fn current_keyword(cursor: &Cursor) -> Option<String> {
    if cursor.peek().kind == TokenKind::Ident && cursor.at_attr_key().is_none() {
        Some(cursor.peek().text.as_str().to_string())
    } else {
        None
    }
}

fn parse_dsl_value(cursor: &mut Cursor, depth: usize) -> Result<DslValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match cursor.peek().kind {
        TokenKind::LBrace => {
            cursor.advance();
            let mut entries = Vec::new();
            while let Some(key) = cursor.dynamic_attr_key()? {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                entries.push((key, parse_dsl_value(cursor, depth + 1)?));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(DslValue::Object(entries))
        }
        TokenKind::LBracket => {
            cursor.advance();
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                items.push(parse_dsl_value(cursor, depth + 1)?);
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(DslValue::Array(items))
        }
        TokenKind::Text => {
            let token = cursor.advance();
            let text = semio_framework_dsl::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(DslValue::String(text))
        }
        TokenKind::Int => {
            let token = cursor.advance();
            let text = token.text.as_str();
            if let Ok(v) = text.parse::<u64>() {
                Ok(DslValue::Number(Number::UInt(v)))
            } else if let Ok(v) = text.parse::<i64>() {
                Ok(DslValue::Number(Number::Int(v)))
            } else {
                let value = parse_f64(&text).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
                Ok(DslValue::Number(Number::Float(value)))
            }
        }
        TokenKind::Float => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(DslValue::Number(Number::Float(value)))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            match token.text.as_str().as_ref() {
                "bytes64"=>{cursor.expect(TokenKind::LParen)?;let token=cursor.expect(TokenKind::Text)?;let bytes=base64_decode(&token.text.as_str()).map_err(|error|TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error,token.span))?;cursor.expect(TokenKind::RParen)?;Ok(DslValue::Bytes(bytes))},
                "null" => Ok(DslValue::Null),
                "true" => Ok(DslValue::Bool(true)),
                "false" => Ok(DslValue::Bool(false)),
                "nan"|"inf"=>Ok(DslValue::Number(Number::Float(parse_f64(&token.text.as_str()).map_err(|error|TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error,token.span))?))),
                other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a value literal, found ident '{other}'"),token.span)),
            }
        }
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected a value literal, found {other:?}"),cursor.span())),
    }
}

/// 🕸️ Parses one wire literal. `<-` is accepted sugar only: normalized here by swapping
/// the two endpoints, so the stored `WireValue` (and everything reprinted from it) only ever
/// holds `->`/`--` or fused labeled arrows — `b<-a` and `a->b` parse to the identical value.
fn parse_wire(cursor: &mut Cursor) -> Result<WireValue, TextError> {
    fn parse_wire_label(cursor: &mut Cursor) -> Result<WireEdgeLabel, TextError> {
        cursor.expect(TokenKind::LBracket)?;
        let id = if cursor.peek().kind == TokenKind::Ident { Some(ident_like_text(&cursor.advance())) } else { None };
        let kind = if cursor.peek().kind == TokenKind::Colon {
            cursor.advance();
            Some(ident_like_text(&cursor.expect(TokenKind::Ident)?))
        } else {
            None
        };
        let label = WireEdgeLabel { id, kind };
        if label.is_empty() {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"edge label `[...]` must name an id and/or a `:kind`",cursor.span()));
        }
        cursor.expect(TokenKind::RBracket)?;
        Ok(label)
    }

    let mut from = parse_wire_node(cursor)?;
    let mut edge_label = WireEdgeLabel::default();
    let edge = match cursor.peek().kind {
        TokenKind::Arrow => {
            cursor.advance();
            let to = parse_wire_node(cursor)?;
            Some((true, to))
        }
        TokenKind::DashArrow => {
            cursor.advance();
            let to = parse_wire_node(cursor)?;
            Some((false, to))
        }
        TokenKind::BackArrow => {
            cursor.advance();
            if cursor.peek().kind == TokenKind::LBracket {
                edge_label = parse_wire_label(cursor)?;
                cursor.expect(TokenKind::Minus)?;
            }
            let to = parse_wire_node(cursor)?;
            let swapped_to = std::mem::replace(&mut from, to);
            Some((true, swapped_to))
        }
        TokenKind::Minus if cursor.peek_at(1).kind == TokenKind::LBracket => {
            cursor.advance();
            edge_label = parse_wire_label(cursor)?;
            let directed = match cursor.peek().kind {
                TokenKind::Arrow => {
                    cursor.advance();
                    true
                }
                TokenKind::DashArrow => {
                    cursor.advance();
                    false
                }
                other => {
                    return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected `->` or `--` to close a labeled edge, found {other:?}"),cursor.span()));
                }
            };
            let to = parse_wire_node(cursor)?;
            Some((directed, to))
        }
        TokenKind::EdgeArrow => {
            let token = cursor.advance();
            let (directed, label) = dsl_notation::decode_fused_edge_arrow(&token.text.as_str())?;
            edge_label = WireEdgeLabel { id: label.id, kind: label.kind };
            let to = parse_wire_node(cursor)?;
            Some((directed, to))
        }
        _ => None,
    };
    let properties = if cursor.peek().kind == TokenKind::LBrace { parse_dsl_value(cursor, 0)? } else { DslValue::Object(Vec::new()) };
    Ok(WireValue { from, edge, edge_label, properties })
}

/// 🔌️ Small public entry point other crates (the graph wire module, trinity) can call
/// directly to lex + parse one standalone wire literal, without needing a `RecordSpec` around it.
pub fn parse_wire_text(text: &str) -> Result<WireValue, TextError> {
    let limits = Limits::default();
    let tokens = lex(text, &limits, false)?;
    let mut cursor = Cursor::new(tokens, limits);
    parse_wire(&mut cursor)
}

fn parse_wire_node(cursor: &mut Cursor) -> Result<WireNode, TextError> {
    let FieldValue::Text(id) = parse_scalar_text(cursor)? else { unreachable!() };
    let kind = if cursor.peek().kind == TokenKind::Colon {
        cursor.advance();
        Some(match parse_scalar_text(cursor)? { FieldValue::Text(value) => value, _ => unreachable!() })
    } else {
        None
    };
    let port = if cursor.peek().kind == TokenKind::At {
        cursor.advance();
        Some(match parse_scalar_text(cursor)? { FieldValue::Text(value) => value, _ => unreachable!() })
    } else {
        None
    };
    Ok(WireNode { id, kind, port })
}

/// 🧾️ Parses one record: its own leading keyword if `spec.keyword` declares one (the
/// `Statements` dispatcher only peeks to choose a variant — consuming it is always this
/// function's job, so a spec is self-contained regardless of whether it's reached via `parse`
/// directly, `Shape::Record`, or a `Statements` variant), positional fields in declaration order,
/// then order-independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a
/// key), until a token that is neither a known key nor an unfilled positional slot — which ends
/// the record (it belongs to whatever comes next: a new statement, a closing brace, or EOF).
fn parse_record_body(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    if spec.layout == RecordLayout::Call {
        return parse_call_record(cursor, spec, depth);
    }
    if let Some(keyword) = &spec.keyword {
        if cursor.at_keyword(keyword) {
            cursor.advance();
        } else {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected keyword '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
        }
    }
    parse_record_fields(cursor, spec, depth)
}

/// 📛️ Parses a `RecordLayout::Call` record: `<name> = <keyword>(args)`. The parenthesized
/// argument list is parsed by the exact same [`parse_record_fields`] loop every other layout uses
/// — it naturally stops at the first token that matches neither a positional slot nor a known
/// key (here, always `)`), so no special "bounded sub-cursor" is needed to keep it from reading
/// past the closing paren.
fn parse_call_record(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    let name_field = spec.fields.iter().find(|f| f.is_call_name).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"RecordLayout::Call requires exactly one field marked call_name()",cursor.span()))?;
    let name = ident_like_text(&cursor.expect(TokenKind::Ident)?);
    cursor.expect(TokenKind::Equals)?;
    let keyword = spec.keyword.as_deref().ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"RecordLayout::Call requires RecordSpec.keyword (the call target)",cursor.span()))?;
    if !cursor.at_keyword(keyword) {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected call target '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    cursor.advance();
    cursor.expect(TokenKind::LParen)?;
    let mut record = parse_record_fields(cursor, spec, depth)?;
    cursor.expect(TokenKind::RParen)?;
    record.fields.insert(name_field.id, FieldValue::Text(name));
    Ok(record)
}

/// 🧾️ Parses a record's fields: positional fields in declaration order, then order-
/// independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a key), until a
/// token that is neither a known key nor an unfilled positional slot — which ends the record (it
/// belongs to whatever comes next: a new statement, a closing brace/paren, or EOF). Excludes any
/// field marked `call_name()` from both candidate sets: that field is consumed by the caller
/// (`RecordLayout::Call`'s `<name> =` prefix) before this function ever runs, for a Call-layout
/// spec, and no field is ever marked `call_name()` under any other layout.
fn parse_record_fields(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    let mut record = RecordValue::default();
    let positional: Vec<&FieldSpec> = {
        let mut p: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some() && !f.is_call_name).collect();
        p.sort_by_key(|f| f.position.unwrap());
        p
    };
    for field in &positional {
        if field.optional {
            // An explicit `_` placeholder always means "absent, but consume the slot" — this is
            // what keeps LATER positionals aligned when an earlier optional one is skipped (see
            // `print_record`'s matching print-side logic). Only positional contexts ever see a
            // `Placeholder` token; keyed optionals are simply omitted instead.
            if cursor.peek().kind == TokenKind::Placeholder {
                cursor.advance();
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
            if !can_start_positional(cursor, &field.shape) {
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
        }
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    // `Statements` fields have no field-level key at all — they're recognized purely by matching
    // one of their own variants' keywords, so at most one such field may appear per record.
    // `Block` fields are also excluded from the `key=value` loop below: their own key acts as a
    // bare leading keyword (`children { ... }`, no `=`) — `Table` fields (bare `key [...] {...}`
    // SoA form) are handled the same way, via their own lookahead branch below.
    let statements_field = spec.fields.iter().find(|f| f.position.is_none() && matches!(f.shape, Shape::Statements(_)));
    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !f.is_call_name && !matches!(f.shape, Shape::Statements(_))).collect();

    loop {
        if let Some(key) = cursor.dynamic_attr_key()? {
            let Some(index) = keyed.iter().position(|f| !matches!(f.shape, Shape::Block(_)) && f.key == key) else { break };
            let field = keyed.remove(index);
            cursor.advance();
            cursor.expect(TokenKind::Equals)?;
            let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
            record.fields.insert(field.id, value);
            continue;
        }
        // `Table`'s bare SoA form: the keyword directly followed by `[` (no `=`) — distinct from
        // the AoS-verbose `key=[...]` form already handled by the `at_attr_key` branch above.
        if let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Table(_)) && cursor.at_keyword(&f.key) && cursor.peek_at(1).kind == TokenKind::LBracket) {
            let field = keyed.remove(index);
            let Shape::Table(spec_fn) = &field.shape else { unreachable!() };
            let spec_fn = *spec_fn;
            cursor.advance();
            let value = parse_table_soa(cursor, spec_fn, depth + 1)?;
            record.fields.insert(field.id, value);
            continue;
        }
        let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Block(_)) && cursor.at_keyword(&f.key)) else { break };
        let field = keyed.remove(index);
        cursor.advance();
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }
    for field in keyed {
        if !record.fields.contains_key(&field.id){record.fields.insert(field.id,FieldValue::Absent);}
    }

    if let Some(field) = statements_field {
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    Ok(record)
}

// 🚫️async: E1 pure lookahead over the now-sync `Cursor`, called inline in a plain `if` with no
// await anywhere at its one call site — see R9
fn can_start_positional(cursor: &Cursor, shape: &Shape) -> bool {
    match shape {
        Shape::Bool | Shape::Enum(_) => cursor.peek().kind == TokenKind::Ident,
        Shape::Int | Shape::UInt => cursor.peek().kind == TokenKind::Int,
        Shape::Float | Shape::Quantity(_) | Shape::Angle(_) | Shape::Dim(_) => matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int),
        Shape::Ref(_) => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Count => cursor.peek().kind == TokenKind::Ident,
        Shape::Coord(_) => cursor.peek().kind == TokenKind::At,
        Shape::Dir => cursor.peek().kind == TokenKind::Caret,
        Shape::Range | Shape::Expr => cursor.peek().kind == TokenKind::LParen,
        Shape::Embed(_) | Shape::EmbedFrom(_) => matches!(cursor.peek().kind, TokenKind::Fence | TokenKind::Text | TokenKind::Placeholder),
        // Only `Text|Placeholder` — NOT bare `Ident` — may start an optional positional `Text`
        // field: an unquoted bare-ident value here would be indistinguishable from the next
        // statement's leading keyword, so this deliberately narrower check (versus `Shape::Text`
        // parsing `Ident|Text` everywhere else) resolves that ambiguity.
        Shape::Text => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Bytes64 => cursor.peek().kind == TokenKind::Text,
        Shape::List(_) => cursor.peek().kind == TokenKind::LBracket,
        Shape::Block(_) | Shape::Map(_) => cursor.peek().kind == TokenKind::LBrace,
        _ => true,
    }
}

//#region 🔖️Table
/// 🚧️ Which shapes have a fixed/bounded token extent and may therefore be a `Table`
/// column: an unbounded `Tuple` (`len: None`, comma-separated until... forever) and `Statements`
/// (repeats until a non-matching keyword) both need an external delimiter to know where they end
/// — fine inside `[ ]`/`{ }` brackets, fatal inside a table row where the ONLY thing marking a
/// row boundary is "we've now read exactly `columns.len()` values".
// 🚫️async: E1 pure, inlined directly into `format!` args alongside `shape_type_name` (Display,
// not Future) — see R9
fn shape_is_self_delimiting(shape: &Shape) -> bool {
    !matches!(shape, Shape::Statements(_) | Shape::Tuple(_, None))
}

/// 🚧️ Spec-build-time validation for a `Table`'s element `RecordSpec` — called wherever a
/// `Shape::Table(spec_fn)` is first evaluated (both parse paths, and printing), since `spec_fn` is
/// a lazy pointer rather than an eagerly-built value there is no earlier moment to check it at.
fn validate_table_columns(spec: &RecordSpec) -> Result<(), TextError> {
    for field in &spec.fields {
        if !shape_is_self_delimiting(&field.shape) {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,format!("table column '{}' has a non-self-delimiting shape ({}) and cannot be a table column", field.key, shape_type_name(&field.shape)),TextSpan::at(1, 1)));
        }
    }
    Ok(())
}

/// 🏷️ UPPERCASE schema type tag for a `Shape` — what a `Table` header prints per column
/// (`id:TEXT`), per the unified syntax law (`UPPERCASE` for engine shapes, `PascalCase` reserved
/// for technology-declared domain kinds).
// 🚫️async: E1 pure, inlined directly into `format!` args at both call sites (Display, not Future) — see R9
pub fn shape_type_name(shape: &Shape) -> &'static str {
    match shape {
        Shape::Bool => "BOOL",
        Shape::Int => "INT",
        Shape::UInt => "UINT",
        Shape::Float => "NUM",
        Shape::Text => "TEXT",
        Shape::Bytes64 => "BYTES",
        Shape::Enum(_) => "ENUM",
        Shape::Tuple(_, _) => "TUPLE",
        Shape::List(_) => "LIST",
        Shape::Record(_) => "REC",
        Shape::Block(_) => "BLOCK",
        Shape::Statements(_) => "STMT",
        Shape::Map(_) => "MAP",
        Shape::Value => "VAL",
        Shape::Table(_) => "TABLE",
        Shape::Wire => "WIRE",
        Shape::Quantity(_) => "QTY",
        Shape::Angle(_) => "ANG",
        Shape::Ref(_) => "REF",
        Shape::Coord(_) => "CRD",
        Shape::Dir => "DIR",
        Shape::Dim(_) => "DIM",
        Shape::Range => "RNG",
        Shape::Count => "CNT",
        Shape::Expr => "EXPR",
        Shape::Embed(_) | Shape::EmbedFrom(_) => "EMBED",
    }
}

/// 📊️ Parses the bare SoA form of a `Table` field: `[col:TYPE ...] { v11 v12 ...  v21 v22
/// ... }`, cursor positioned right after the field's own keyword has already been consumed. The
/// header names columns (in the order values then appear per row); a `:TYPE` suffix is accepted
/// but not required to resolve a column (it's a human/printer-facing tag, not load-bearing for
/// parsing — the column's real shape always comes from the element `RecordSpec`), which is what
/// lets a hand-written header omit types the engine can already infer. Rows have NO separator —
/// reading exactly `columns.len()` values per row is what makes a row self-delimiting, which is
/// also why every column shape must itself be self-delimiting (`validate_table_columns`).
fn parse_table_soa(cursor: &mut Cursor, spec_fn: RecordSpecProducer, depth: usize) -> Result<FieldValue, TextError> {
    let element_spec = (spec_fn.ordinary)();
    validate_table_columns(&element_spec)?;
    cursor.expect(TokenKind::LBracket)?;
    let mut columns: Vec<&FieldSpec> = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        let key_token = cursor.expect(TokenKind::Ident)?;
        let key = key_token.text.as_str().to_string();
        if cursor.peek().kind == TokenKind::Colon {
            cursor.advance();
            cursor.expect(TokenKind::Ident)?; // type tag — documentation only, not re-validated here
        }
        let field_spec = element_spec.fields.iter().find(|f| f.key == key).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown table column '{key}'"),key_token.span))?;
        columns.push(field_spec);
    }
    cursor.expect(TokenKind::RBracket)?;
    cursor.expect(TokenKind::LBrace)?;
    let mut rows = Vec::new();
    while cursor.peek().kind != TokenKind::RBrace {
        let mut record = RecordValue::default();
        for field_spec in &columns {
            if cursor.peek().kind == TokenKind::Placeholder {
                cursor.advance();
                record.fields.insert(field_spec.id, FieldValue::Absent);
                continue;
            }
            let value = parse_table_cell(cursor, &field_spec.shape, depth + 1)?;
            record.fields.insert(field_spec.id, value);
        }
        for field_spec in &element_spec.fields {
            if !record.fields.contains_key(&field_spec.id){record.fields.insert(field_spec.id,FieldValue::Absent);}
        }
        rows.push(FieldValue::Record(record));
        cursor.limits.check_nodes(rows.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBrace)?;
    Ok(FieldValue::List(rows))
}

/// 🧱️ Reads one table cell's value. Every table-safe shape is bounded by its own bracket or
/// a fixed token count (`validate_table_columns`/`shape_is_self_delimiting`) — EXCEPT a bare
/// `Shape::Record` column, which prints as a flat run of `key=value` tokens with no bracket of its
/// own (a table row has no `field=` prefix to give it one, unlike a Record-shaped field elsewhere).
/// Two adjacent columns of the SAME record type (or any two types sharing a field name) are then
/// genuinely ambiguous: `parse_record_body`'s keyed loop for column N keeps matching `key=value`
/// tokens for as long as the key is one of ITS OWN not-yet-filled fields, so a column-N field left
/// absent (never printed) silently lets column N's parse run on and swallow column N+1's
/// same-named token instead of stopping at the column boundary. Braced here for exactly that
/// reason — every other shape already round-trips through the ordinary `parse_shape`.
fn parse_table_cell(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    if let Shape::Record(spec_fn) = shape {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        return Ok(FieldValue::Record(record));
    }
    parse_shape(cursor, shape, depth)
}

/// 📋️ The AoS-list form for a `Table` value reached anywhere other than a record's own
/// leading keyword-prefixed field: `[ {row-fields} {row-fields} ... ]`. Each row is brace-wrapped
/// for the same reason a `Shape::Record` table COLUMN is (`parse_table_cell` above) — a table row
/// type is commonly declared with no keyword of its own (a header already gives every row its
/// column order, so SoA rows don't need one), so without a bracket of its own, one row's absent
/// field could let its parse run on into the next row's same-named token exactly like the
/// column-vs-column case. Bracing every row here removes that ambiguity regardless of whether the
/// row type happens to declare a keyword or not.
fn parse_table_list(cursor: &mut Cursor, spec_fn: RecordSpecProducer, depth: usize) -> Result<FieldValue, TextError> {
    cursor.expect(TokenKind::LBracket)?;
    let mut items = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        items.push(FieldValue::Record(record));
        cursor.limits.check_nodes(items.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBracket)?;
    Ok(FieldValue::List(items))
}

/// 📋️ Prints the braced AoS-list form `parse_table_list` reads back. Ordinary `[ ]` spacing
/// (a space just inside, per the general list rule — NOT the header's own tight-glued exception).
fn print_table_list(spec_fn: RecordSpecProducer, items: &[FieldValue], writer: &mut Writer) {
    writer.atom("[");
    for item in items {
        let FieldValue::Record(record) = item else { continue };
        writer.atom("{");
        writer.glue();
        print_record(record, &(spec_fn.ordinary)(), writer);
        writer.glue();
        writer.atom("}");
    }
    writer.atom("]");
}
//#endregion 🔖️Table

fn base64_decode(text:&str)->Result<Vec<u8>,String>{protocol::bytes::decode_base64(text)}
fn base64_encode(bytes:&[u8])->String{protocol::bytes::encode_base64(bytes)}
//#endregion 🔖️Parser

//#region 🔖️Writer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinMode {
    Document,
    Inline,
}

/// ✍️ A chunk tree that renders in either join mode — the structural half of the newline
/// law. `atom` asserts its argument contains no raw `\n` (Document mode still separates atoms with
/// synthesized whitespace, never embeds one inside an atom), so `render(Inline)` joining every
/// chunk with a single space can never produce an embedded newline.
///
/// 📏️ Canonical spacing rules (both join modes, structurally guaranteed — never hand-tuned
/// per callsite): never a space adjacent to `=` (`key=[ a b ]`, not `key= [ a b ]` — the printer
/// achieves this by pushing a bare `key=` atom, calling [`Writer::glue`], then printing the
/// value); exactly one space between sibling atoms; exactly one space just inside `[ ]`/`{ }` when
/// rendered inline (`[ a b ]`, not `[a b]`) — EXCEPT a `Table` header's `[ ]`, which is glued
/// tight on both sides (`[id:TEXT x:NUM]`) since it's a fixed one-shot header, not a
/// space-joined element list; a space appears before a keyword-led block's `{` (`children {
/// ... }`) but never before a glued composite's `{` (`data={ ... }`).
#[derive(Default)]
pub struct Writer {
    chunks: Vec<Chunk>,
    indent: usize,
}

enum Chunk {
    Atom(String),
    OpenBlock,
    CloseBlock,
    NewRecord,
    /// 🧲️ One-shot marker: the very next `Atom`/`OpenBlock` chunk renders with NO
    /// preceding separator (space in Inline mode, space-or-newline-continuation in Document mode)
    /// — consumed by that one chunk, then normal spacing resumes. See [`Writer::glue`].
    Glue,
    /// 📜️ `Shape::Embed`'s payload — the one chunk kind whose Document and Inline renders
    /// genuinely differ in FORM (fenced block vs. escaped quoted string), not just spacing.
    Verbatim {
        lang: String,
        content: String,
    },
}

impl Writer {
    pub fn new() -> Self {
        Self { chunks: Vec::new(), indent: 0 }
    }

    pub fn atom(&mut self, s: impl AsRef<str>) {
        let s = s.as_ref();
        debug_assert!(!s.contains('\n'), "Writer::atom must not contain a raw newline: {s:?}");
        self.chunks.push(Chunk::Atom(s.to_string()));
    }

    pub fn key_value(&mut self, key: &str, value: impl AsRef<str>) {
        self.atom(format!("{key}={}", value.as_ref()));
    }

    pub fn open_block(&mut self) {
        self.chunks.push(Chunk::OpenBlock);
        self.indent += 1;
    }

    pub fn close_block(&mut self) {
        self.indent = self.indent.saturating_sub(1);
        self.chunks.push(Chunk::CloseBlock);
    }

    pub fn new_record(&mut self) {
        self.chunks.push(Chunk::NewRecord);
    }

    /// 🧲️ Fuses the next pushed chunk onto whatever precedes it, with no separator, in
    /// BOTH join modes — the mechanism behind every `key=value`/`key=[...]`/`key={...}` fusion in
    /// this printer. Replaces the old approach of mutating an already-pushed atom's string in
    /// place (which only worked for single-atom scalar values): `glue()` composes with arbitrarily
    /// structured values (nested blocks, lists, whole sub-records) since it's a rendering-time
    /// join, not a string-splice.
    pub fn glue(&mut self) {
        self.chunks.push(Chunk::Glue);
    }

    /// 📜️ Pushes a `Shape::Embed` payload — content MAY contain raw newlines (unlike
    /// [`Self::atom`], which forbids them), since Document mode renders it as a fence.
    pub fn verbatim(&mut self, lang: &str, content: &str) {
        self.chunks.push(Chunk::Verbatim { lang: lang.to_string(), content: content.to_string() });
    }

    pub fn render(&self, mode: JoinMode) -> String {
        match mode {
            JoinMode::Inline => {
                let mut parts: Vec<String> = Vec::new();
                let mut glued = false;
                let mut push = |piece: String, glued: &mut bool| {
                    if *glued {
                        if let Some(last) = parts.last_mut() {
                            last.push_str(&piece);
                        } else {
                            parts.push(piece);
                        }
                    } else {
                        parts.push(piece);
                    }
                    *glued = false;
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => push(s.clone(), &mut glued),
                        Chunk::OpenBlock => push("{".to_string(), &mut glued),
                        Chunk::CloseBlock => push("}".to_string(), &mut glued),
                        Chunk::NewRecord => {}
                        Chunk::Verbatim { content, .. } => push(format!("\"{}\"", semio_framework_dsl::escape_text(content)), &mut glued),
                    }
                }
                parts.join(" ")
            }
            JoinMode::Document => {
                let mut out = String::new();
                let mut indent = 0usize;
                let mut line_open = false;
                let mut glued = false;
                let push_indent = |out: &mut String, indent: usize| {
                    for _ in 0..indent {
                        out.push_str("  ");
                    }
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => {
                            if !line_open {
                                push_indent(&mut out, indent);
                                line_open = true;
                            } else if !glued {
                                out.push(' ');
                            }
                            out.push_str(s);
                            glued = false;
                        }
                        Chunk::OpenBlock => {
                            if glued {
                                out.push('{');
                            } else {
                                out.push_str(" {");
                            }
                            out.push('\n');
                            line_open = false;
                            indent += 1;
                            glued = false;
                        }
                        Chunk::CloseBlock => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                            indent = indent.saturating_sub(1);
                            push_indent(&mut out, indent);
                            out.push('}');
                            out.push('\n');
                        }
                        Chunk::NewRecord => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                        }
                        Chunk::Verbatim { lang, content } => {
                            if !line_open {
                                push_indent(&mut out, indent);
                            } else if !glued {
                                out.push(' ');
                            }
                            out.push_str("```");
                            out.push_str(lang);
                            out.push('\n');
                            out.push_str(content);
                            if !content.is_empty() {
                                out.push('\n');
                            }
                            out.push_str("```");
                            line_open = true;
                            glued = false;
                        }
                    }
                }
                if line_open {
                    out.push('\n');
                }
                out
            }
        }
    }
}

/// 🥇️ Field print order within one record — NOT declaration order: keyword, then
/// positionals (unchanged), then keyed fields grouped scalar-before-composite-before-table-
/// before-statements, ties broken by original declaration order (a stable sort over an
/// already-declaration-order slice achieves this for free). Metadata/scalars land before large
/// nested/tabular blocks, which is friendlier to lazy loading/streaming readers — parsing stays
/// completely order-independent, so this is a print-only change.
// 🚫️async: E1 pure, consumed by `Iterator::sort_by_key`'s sync closure (its `u8` result must be
// `Ord`, which `impl Future<Output = u8>` is not) — see R9
fn keyed_field_rank(shape: &Shape) -> u8 {
    match shape {
        Shape::Bool
        | Shape::Int
        | Shape::UInt
        | Shape::Float
        | Shape::Text
        | Shape::Bytes64
        | Shape::Enum(_)
        | Shape::Tuple(_, _)
        | Shape::Quantity(_)
        | Shape::Angle(_)
        | Shape::Ref(_)
        | Shape::Coord(_)
        | Shape::Dir
        | Shape::Dim(_)
        | Shape::Range
        | Shape::Count
        | Shape::Expr => 0,
        Shape::List(_) | Shape::Map(_) | Shape::Record(_) | Shape::Block(_) | Shape::Value | Shape::Wire => 1,
        Shape::Table(_) => 2,
        Shape::Statements(_) => 3,
        // Ranks LAST of all: a multi-line fence dwarfs everything else in a record, so it should
        // print after every scalar/composite/table field, not interleaved among them.
        Shape::Embed(_) | Shape::EmbedFrom(_) => 4,
    }
}

pub fn print_record(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    if spec.layout == RecordLayout::Call {
        print_call_record(value, spec, writer);
        return;
    }
    if let Some(keyword) = &spec.keyword {
        writer.atom(keyword);
    }
    print_record_fields(value, spec, writer);
}

/// 📛️ Prints a `RecordLayout::Call` record: `<name> = <keyword>(args)`. The argument list
/// is built by [`print_record_fields`] — the exact same field-printing logic every other layout
/// uses — rendered to its own `JoinMode::Inline` string and glued onto the keyword inside parens,
/// so a positional/keyed field prints identically here as it would under `Inline` layout.
fn print_call_record(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    let Some(name_field) = spec.fields.iter().find(|f| f.is_call_name) else {
        debug_assert!(false, "RecordLayout::Call requires exactly one field marked call_name()");
        return;
    };
    let name_text = match value.get(name_field.id) {
        Some(fv @ FieldValue::Text(_)) => scalar_to_text(fv),
        _ => String::new(),
    };
    writer.atom(name_text);
    writer.atom("=");
    if let Some(keyword) = &spec.keyword {
        writer.atom(keyword);
    }
    let mut args_writer = Writer::new();
    print_record_fields(value, spec, &mut args_writer);
    let args_text = args_writer.render(JoinMode::Inline);
    writer.glue();
    writer.atom(format!("({args_text})"));
}

/// 🖨️ Prints a record's fields: positional bare in declaration order, then order-
/// independent `key=value` attributes. Excludes any field marked `call_name()` — see
/// [`parse_record_fields`]'s matching doc comment for why.
fn print_record_fields(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    let mut positional: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some() && !f.is_call_name).collect();
    positional.sort_by_key(|f| f.position.unwrap());
    for (index, field) in positional.iter().enumerate() {
        match value.get(field.id) {
            Some(fv) if !matches!(fv, FieldValue::Absent) => print_shape(fv, &field.shape, writer),
            _ => {
                // An absent OPTIONAL positional prints as `_` only if some LATER positional in
                // this same record is actually present — that's what keeps slots aligned for the
                // reader (and reparse). A run of trailing absents needs no placeholder at all.
                let later_present = positional[index + 1..].iter().any(|f| matches!(value.get(f.id), Some(fv) if !matches!(fv, FieldValue::Absent)));
                if later_present {
                    writer.atom("_");
                }
            }
        }
    }

    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !f.is_call_name).collect();
    keyed.sort_by_key(|f| keyed_field_rank(&f.shape));
    for field in keyed {
        match value.get(field.id) {
            Some(FieldValue::Absent) | None => continue,
            Some(fv) => match &field.shape {
                Shape::EmbedFrom(lang_key) => {
                    writer.new_record();
                    writer.atom(format!("{}=", field.key));
                    writer.glue();
                    let lang = spec
                        .fields
                        .iter()
                        .find(|f| f.key == *lang_key)
                        .and_then(|f| value.get(f.id))
                        .and_then(|v| match v {
                            FieldValue::Text(t) => Some(t.as_str()),
                            _ => None,
                        })
                        .unwrap_or("plaintext");
                    if let FieldValue::Text(content) = fv {
                        writer.verbatim(lang, content);
                    }
                }
                // `Statements` items each carry their own leading keyword — no field-level key at
                // all is ever printed for this shape.
                Shape::Statements(_) => print_shape(fv, &field.shape, writer),
                // `Block`'s own key is a bare leading keyword, not a `key=value` attribute
                // (`children { ... }`, never `children={...}`).
                Shape::Block(_) => {
                    writer.new_record();
                    writer.atom(&field.key);
                    print_shape(fv, &field.shape, writer);
                }
                // `Table`'s own key is likewise a bare leading keyword, but — unlike `Block` —
                // it must always go through the dedicated SoA writer (`print_table`), never the
                // generic `print_shape` dispatch: that dispatch renders `Table` as the bracketed
                // AoS list (see its `Shape::Table` arm below) so a `Table` value reached any OTHER
                // way (nested inside a table row, a list, ...) stays self-delimiting. Only here,
                // directly after a record's own leading keyword, is the bare `[col:TYPE ...]
                // {rows}` form reachable on the parse side (`parse_record_body`'s dedicated
                // bare-SoA lookahead) — printing it via `print_shape` here would silently regress
                // to the AoS form for every top-level table field.
                Shape::Table(spec_fn) => {
                    writer.new_record();
                    writer.atom(&field.key);
                    if let FieldValue::List(items) = fv {
                        print_table(*spec_fn, items, writer);
                    }
                }
                _ => {
                    writer.atom(format!("{}=", field.key));
                    print_key_value(field, fv, writer);
                }
            },
        }
    }
}

/// 🧲️ `key=` was just pushed by the caller — glue the value onto it with no separator,
/// then print it normally (composed, not string-spliced, so this handles arbitrarily structured
/// values exactly like a bare `print_shape` call would).
fn print_key_value(field: &FieldSpec, value: &FieldValue, writer: &mut Writer) {
    writer.glue();
    match (&field.shape, value) {
        (Shape::Enum(variants), FieldValue::Enum(ordinal)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        _ => print_shape(value, &field.shape, writer),
    }
}

fn scalar_to_text(value: &FieldValue) -> String {
    match value {
        FieldValue::Bool(b) => b.to_string(),
        FieldValue::Int(i) => i.to_string(),
        FieldValue::UInt(u) => u.to_string(),
        FieldValue::Float(f) => format_f64(*f),
        // Bare (unquoted) whenever the text lexes back as exactly this one ident — the printer's
        // half of the "strings bare-preferred" law; `is_bare_ident` also excludes reserved literal
        // idents (`_`/`true`/`false`/`null`/`nan`/`inf`) and number-shaped text, which always fall
        // through to the quoted+escaped form instead.
        FieldValue::Text(s) => {
            if semio_framework_dsl::is_bare_ident(s) {
                s.clone()
            } else {
                format!("\"{}\"", semio_framework_dsl::escape_text(s))
            }
        }
        FieldValue::Bytes64(bytes) => format!("\"{}\"", base64_encode(bytes)),
        FieldValue::Enum(_) => String::new(), // resolved by caller via variants table when needed
        _ => String::new(),
    }
}

/// 🔢️ Renders one `FieldValue::Tuple` element as bare text for the `Coord`/`Dir`/`Dim`/
/// `Range` printers above — every element of those tuples is always `FieldValue::Float` by
/// construction (their parsers only ever push `FieldValue::Float`), so this panics rather than
/// falling back on a malformed value, matching the rest of this module's "trust the parser built
/// this" convention for shapes whose `FieldValue` invariant is enforced entirely at parse time.
// 🚫️async: E1 pure, passed as a bare fn item into `Iterator::map` sync closures at every call site — see R9
fn number_tuple_component(value: &FieldValue) -> String {
    match value {
        FieldValue::Float(v) => format_f64(*v),
        other => panic!("Coord/Dir/Dim/Range tuple element must be Float, found {other:?}"),
    }
}

pub fn print_shape(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    match (value, shape) {
        // Must precede the generic scalar arm below: that arm's shape pattern is `_` and would
        // otherwise swallow every `FieldValue::Float` regardless of shape, printing a bare number
        // with no unit suffix even for a `Quantity`/`Angle` field.
        (FieldValue::Float(v), Shape::Quantity(unit) | Shape::Angle(unit)) => {
            writer.atom(format!("{}{}", format_f64(*v), unit.symbol));
        }
        (FieldValue::UInt(v), Shape::Count) => {
            writer.atom(format!("x{v}"));
        }
        (FieldValue::Tuple(items), Shape::Coord(_)) => {
            writer.atom(format!("@{}", items.iter().map(number_tuple_component).collect::<Vec<_>>().join(",")));
        }
        (FieldValue::Tuple(items), Shape::Dir) => {
            writer.atom(format!("^{}", items.iter().map(number_tuple_component).collect::<Vec<_>>().join(",")));
        }
        (FieldValue::Tuple(items), Shape::Dim(_)) => {
            writer.atom(items.iter().map(number_tuple_component).collect::<Vec<_>>().join("x"));
        }
        (FieldValue::Tuple(items), Shape::Range) => {
            let parts: Vec<String> = items.iter().map(number_tuple_component).collect();
            let body = match parts.as_slice() {
                [lo, hi] => format!("{lo}..{hi}"),
                [lo, hi, step] => format!("{lo}..{hi},{step}"),
                _ => parts.join(","),
            };
            writer.atom(format!("({body})"));
        }
        (FieldValue::Expr(expr), Shape::Expr) => {
            writer.atom(format!("({})", print_expr(expr)));
        }
        (FieldValue::Text(content), Shape::Embed(lang)) => {
            writer.verbatim(lang, content);
        }
        (FieldValue::Text(content), Shape::EmbedFrom(lang_key)) => {
            // Fallback when print_shape is called without sibling resolution — prefer plaintext fence.
            let _ = lang_key;
            writer.verbatim("plaintext", content);
        }
        (FieldValue::Bool(_) | FieldValue::Int(_) | FieldValue::UInt(_) | FieldValue::Float(_) | FieldValue::Text(_) | FieldValue::Bytes64(_), _) => {
            writer.atom(scalar_to_text(value));
        }
        (FieldValue::Enum(ordinal), Shape::Enum(variants)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        (FieldValue::Tuple(items), Shape::Tuple(elem, _)) => {
            let mut rendered: Vec<String> = Vec::with_capacity(items.len());
            for item in items {
                let mut sub = Writer::new();
                print_shape(item, elem, &mut sub);
                rendered.push(sub.render(JoinMode::Inline));
            }
            writer.atom(rendered.join(","));
        }
        // A `Table` reached here (NOT via `print_record`'s own keyed-field dispatch, which calls
        // `print_table` directly) is nested inside another shape — a table row's own column, a
        // list element, ... — where the bare `key [col:TYPE ...] {rows}` form has no bracket of
        // its own to mark where it ends. Render the braced-row AoS list instead (see
        // `print_table_list`), matching what `parse_shape`'s own `Shape::Table` arm parses in
        // every one of these same contexts.
        (FieldValue::List(items), Shape::Table(spec_fn)) => print_table_list(*spec_fn, items, writer),
        (FieldValue::List(items), Shape::List(elem)) => {
            writer.atom("[");
            for item in items {
                if matches!(elem.as_ref(), Shape::Record(_)) {
                    writer.atom("{");
                    print_shape(item, elem, writer);
                    writer.atom("}");
                } else {
                    print_shape(item, elem, writer);
                }
            }
            writer.atom("]");
        }
        (FieldValue::Record(record), Shape::Record(spec_fn)) => {
            print_record(record, &(spec_fn.ordinary)(), writer);
        }
        (FieldValue::Block(inner_value), Shape::Block(inner_shape)) => {
            writer.open_block();
            print_shape(inner_value, inner_shape, writer);
            writer.close_block();
        }
        (FieldValue::Statements(items), Shape::Statements(variants)) => {
            for (keyword, record) in items {
                writer.new_record();
                if let Some((_, spec_fn)) = variants.iter().find(|(kw, _)| kw == keyword) {
                    print_record(record, &(spec_fn.ordinary)(), writer);
                }
            }
        }
        (FieldValue::Map(entries), Shape::Map(inner)) => {
            writer.open_block();
            let mut sorted = entries.iter().collect::<Vec<_>>();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            for (key, value) in &sorted {
                writer.atom(dynamic_key_text(key));
                writer.glue();
                if matches!(inner.as_ref(), Shape::Record(_)) {
                    writer.atom("{");
                    print_shape(value, inner, writer);
                    writer.atom("}");
                } else { print_shape(value, inner, writer); }
            }
            writer.close_block();
        }
        (FieldValue::Value(dsl_value), Shape::Value) => print_dsl_value(dsl_value, writer),
        (FieldValue::Wire(wire), Shape::Wire) => print_wire(wire, writer),
        _ => {}
    }
}

/// 📊️ Always prints the compact SoA form — this (not the parser, which still accepts the
/// verbose AoS form too) is what makes `canonicalize` migrate old AoS documents to SoA
/// automatically. Header `[ ]` is glued tight on both sides (`[id:TEXT x:NUM]`); rows have no
/// separator, one row per line in Document mode purely for readability (`new_record` is a no-op
/// in Inline mode).
fn print_table(spec_fn: RecordSpecProducer, items: &[FieldValue], writer: &mut Writer) {
    let element_spec = (spec_fn.ordinary)();
    writer.atom("[");
    writer.glue();
    for field in &element_spec.fields {
        writer.atom(format!("{}:{}", field.key, shape_type_name(&field.shape)));
    }
    writer.glue();
    writer.atom("]");
    writer.open_block();
    for item in items {
        writer.new_record();
        let FieldValue::Record(record) = item else { continue };
        for field in &element_spec.fields {
            match record.get(field.id) {
                Some(fv) if !matches!(fv, FieldValue::Absent) => print_table_cell(fv, &field.shape, writer),
                _ => writer.atom("_"),
            }
        }
    }
    writer.close_block();
}

/// 🧱️ Prints one table cell's value. See `parse_table_cell` for why a bare `Shape::Record`
/// column is brace-wrapped here — `{ }` glued tight on both sides, the same technique the header's
/// own `[ ]` uses, so bracing never disturbs the "no space just inside" canonical spacing rule for
/// a one-shot wrapper — and every other shape is left to the ordinary `print_shape`, already
/// self-delimiting.
fn print_table_cell(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    if let (FieldValue::Record(record), Shape::Record(spec_fn)) = (value, shape) {
        writer.atom("{");
        writer.glue();
        print_record(record, &(spec_fn.ordinary)(), writer);
        writer.glue();
        writer.atom("}");
        return;
    }
    print_shape(value, shape, writer);
}

fn dynamic_key_text(key: &str) -> String {
    if semio_framework_dsl::is_bare_ident(key) { format!("{key}=") } else { format!("\"{}\"=", semio_framework_dsl::escape_text(key)) }
}

fn print_dsl_value(value: &DslValue, writer: &mut Writer) {
    match value {
        DslValue::Null => writer.atom("null"),
        DslValue::Bool(b) => writer.atom(b.to_string()),
        DslValue::Number(Number::UInt(n)) => writer.atom(n.to_string()),
        DslValue::Number(Number::Int(n)) => writer.atom(n.to_string()),
        DslValue::Number(Number::Float(n)) => {
            let mut value = format_f64(*n);
            if n.is_finite() && !value.contains(['.', 'e', 'E']) {
                value.push_str(".0");
            }
            writer.atom(value);
        }
        DslValue::String(s) => writer.atom(format!("\"{}\"", semio_framework_dsl::escape_text(s))),
        DslValue::Bytes(bytes)=>writer.atom(format!("bytes64(\"{}\")",base64_encode(bytes))),
        DslValue::Array(items) => {
            writer.atom("[");
            for item in items {
                print_dsl_value(item, writer);
            }
            writer.atom("]");
        }
        DslValue::Object(entries) => {
            let mut sorted = entries.iter().collect::<Vec<_>>();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            writer.open_block();
            for (key, value) in &sorted {
                writer.atom(dynamic_key_text(key));
                writer.glue();
                print_dsl_value(value, writer);
            }
            writer.close_block();
        }
    }
}

fn print_wire(wire: &WireValue, writer: &mut Writer) {
    let map_node = |node: &WireNode| dsl_notation::EdgeNode { id: node.id.clone(), kind: node.kind.clone(), port: node.port.clone() };
    let edge = dsl_notation::EdgeValue {
        from: map_node(&wire.from),
        link: wire.edge.as_ref().map(|(directed, to)| dsl_notation::EdgeLink { directed: *directed, label: dsl_notation::EdgeLabel { id: wire.edge_label.id.clone(), kind: wire.edge_label.kind.clone() }, to: map_node(to) }),
    };
    writer.atom(dsl_notation::print_edge(&edge));
    if !matches!(&wire.properties, DslValue::Object(entries) if entries.is_empty()) {
        print_dsl_value(&wire.properties, writer);
    }
}

/// 🔁️ Prints `value` against `spec` in the given join mode — the top-level entry point
/// `dsl_derive`-generated code calls from `ArtifactDsl::print_dsl`/`OpText::print_op`.
pub fn print(value: &RecordValue, spec: &RecordSpec, mode: JoinMode) -> String {
    let mut writer = Writer::new();
    print_record(value, spec, &mut writer);
    writer.render(mode)
}
//#endregion 🔖️Writer

//#region 🔖️Canonicalize
/// ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)`: reprints whatever `parse`
/// produces from `text`, which is the fixpoint every technology's `print_dsl` output must already
/// be at (the round-trip law), so this doubles as the idempotence check.
pub fn canonicalize(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<String, TextError> {
    let value = parse(text, spec, opts)?;
    Ok(print(&value, spec, JoinMode::Document))
}
//#endregion 🔖️Canonicalize

//#region 🔖️Language
/// 🎨️ Generic editor surface over any `RecordSpec` — the generalization of
/// `math::graph::dsl`'s hand-rolled `LanguageService`.
pub struct LanguageService<'g> {
    pub spec: &'g RecordSpec,
}

pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
}

impl<'g> LanguageService<'g> {
    pub fn new(spec: &'g RecordSpec) -> Self {
        Self { spec }
    }

    fn keywords(&self) -> Vec<String> {
        let mut out = Vec::new();
        collect_keywords(self.spec, &mut out, &mut HashSet::new(), &mut HashSet::new());
        out
    }

    pub fn semantic_tokens(&self, text: &str) -> Vec<(TokenClass, TextSpan)> {
        let limits = Limits::default();
        let tokens = lex(text, &limits, true).unwrap_or_default();
        let keywords = self.keywords();
        let keyword_refs: Vec<&str> = keywords.iter().map(String::as_str).collect();
        semio_framework_dsl::token_classes(&tokens, &keyword_refs)
    }

    pub fn diagnostics(&self, text: &str) -> Vec<TextError> {
        match parse(text, self.spec, &ParseOptions::default()) {
            Ok(_) => Vec::new(),
            Err(e) => vec![e],
        }
    }

    /// 💡️ Completions at `offset`: every key not yet used in the record enclosing the
    /// cursor, plus every keyword reachable from the root. A simple, always-available baseline —
    /// full context-sensitive narrowing is a natural follow-up once `Cst` gains node addressing.
    pub fn completions(&self, _text: &str, _offset: usize) -> Vec<CompletionItem> {
        let mut items: Vec<CompletionItem> = self.spec.fields.iter().filter(|f| !f.key.is_empty()).map(|f| CompletionItem { label: f.key.clone(), detail: Some(format!("{:?}", f.shape)) }).collect();
        for keyword in self.keywords() {
            items.push(CompletionItem { label: keyword, detail: None });
        }
        items
    }
}

// 🚫️async: E1 pure tree walk, mutually recursive with `collect_shape_keywords` below through match
// arms whose tail expression must resolve to the same `()` type in every arm — see R9
fn collect_keywords(spec: &RecordSpec, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    if let Some(kw) = &spec.keyword {
        out.push(kw.clone());
    }
    for field in &spec.fields {
        collect_shape_keywords(&field.shape, out, seen, seen_records);
    }
}

/// 🔁️ `seen` guards against a genuinely self-referential `Statements` table (a recursive
/// block tree whose own variant list contains itself): each `(spec_fn.ordinary)()` call is only expanded the
/// first time its keyword is reached, so the keyword set — which is always finite, even when the
/// grammar's real nesting isn't — is collected exactly once instead of infinitely. `seen_records`
/// is the same guard for a self-referential `Shape::Record` (a `#[derive(DslRecord)]` struct field
/// whose type recurses back to itself, e.g. a dynamic-value type nesting a map of itself) — a bare
/// Record has no keyword to key on, so this tracks the `fn() -> RecordSpec` pointer's own address
/// instead (two calls to the same generated `__dsl_spec` always share one code address).
// 🚫️async: E1 pure, same mutual-recursion R9 case as `collect_keywords` above
fn collect_shape_keywords(shape: &Shape, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    match shape {
        Shape::Record(spec_fn) => {
            if seen_records.insert(spec_fn.ordinary as usize) {
                collect_keywords(&(spec_fn.ordinary)(), out, seen, seen_records);
            }
        }
        Shape::Block(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        Shape::Statements(variants) => {
            for (kw, spec_fn) in variants {
                out.push(kw.clone());
                if seen.insert(kw.clone()) {
                    collect_keywords(&(spec_fn.ordinary)(), out, seen, seen_records);
                }
            }
        }
        Shape::List(inner) | Shape::Tuple(inner, _) | Shape::Map(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        _ => {}
    }
}
//#endregion 🔖️Language

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path="🧪️tests/🧬️intrinsic-bytes/🦀️.rs"]
mod intrinsic_bytes_tests;

#[cfg(test)]
#[path="🧪️tests/🧾️record-list/🦀️.rs"]
mod list_record_tests;

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs

SHA-256 `dc8d7c9fad37bc7a89d1b637615faa15bd3afc8eb3ce8820ecad4d5f652e0001`; 97482 bytes.

```
//! ➡️ Directed port graph base: engine aliases, scene descriptors, layouts, board types.

pub mod scene_json {
    // #region scene_json
    //! 🧾️ Directed port graph scene descriptors and fixture JSON helpers.

    use serde::{Deserialize, Serialize};

    pub use crate::infinite::board::ports::HandleDescJson;
    pub use crate::infinite::board::{CameraJson, NodeDescJson};

    /// 🌉️ Hand-written, not derived: `user_data: Option<serde_json::Value>` has no `ToValue`/
    /// `FromValue` for `serde_json::Value` — same reason as `NodeDescJson`/`HandleDescJson`.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EdgeDescJson {
        pub id: String,
        pub source: String,
        pub target: String,
        /// 🧩️ Semantic edge-kind id for compatibility at `edge` specificity.
        #[serde(default)]
        pub edge_kind: Option<String>,
        /// 🔺️ Per-instance source tip id from the edge tip registry (`none` disables).
        #[serde(default)]
        pub source_tip: Option<String>,
        /// 🔺️ Per-instance target tip id from the edge tip registry (`none` disables).
        #[serde(default)]
        pub target_tip: Option<String>,
        #[serde(default)]
        pub selected: Option<bool>,
        #[serde(default)]
        pub style: Option<String>,
        #[serde(default)]
        pub user_data: Option<serde_json::Value>,
        #[serde(default)]
        pub visible: Option<bool>,
        #[serde(default)]
        pub locked: Option<bool>,
    }

    impl dsl::ToValue for EdgeDescJson {
        fn to_value(&self) -> dsl::DslValue {
            dsl::DslValue::object([
                ("id".to_string(), dsl::ToValue::to_value(&self.id)),
                ("source".to_string(), dsl::ToValue::to_value(&self.source)),
                ("target".to_string(), dsl::ToValue::to_value(&self.target)),
                ("edgeKind".to_string(), dsl::ToValue::to_value(&self.edge_kind)),
                ("sourceTip".to_string(), dsl::ToValue::to_value(&self.source_tip)),
                ("targetTip".to_string(), dsl::ToValue::to_value(&self.target_tip)),
                ("selected".to_string(), dsl::ToValue::to_value(&self.selected)),
                ("style".to_string(), dsl::ToValue::to_value(&self.style)),
                (
                    "userData".to_string(),
                    match &self.user_data {
                        Some(v) => dsl::DslValue::from(v),
                        None => dsl::DslValue::Null,
                    },
                ),
                ("visible".to_string(), dsl::ToValue::to_value(&self.visible)),
                ("locked".to_string(), dsl::ToValue::to_value(&self.locked)),
            ])
        }
    }

    impl dsl::FromValue for EdgeDescJson {
        fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
            let dsl::DslValue::Object(fields) = value else {
                return Err(dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for EdgeDescJson, found {value:?}")));
            };
            let mut id = None;
            let mut source = None;
            let mut target = None;
            let mut edge_kind = None;
            let mut source_tip = None;
            let mut target_tip = None;
            let mut selected = None;
            let mut style = None;
            let mut user_data = None;
            let mut visible = None;
            let mut locked = None;
            for (key, entry) in fields {
                match key.as_str() {
                    "id" => id = Some(<String as dsl::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                    "source" => source = Some(<String as dsl::FromValue>::from_value(entry).map_err(|e| e.under("source"))?),
                    "target" => target = Some(<String as dsl::FromValue>::from_value(entry).map_err(|e| e.under("target"))?),
                    "edgeKind" => edge_kind = <Option<String> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("edgeKind"))?,
                    "sourceTip" => source_tip = <Option<String> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("sourceTip"))?,
                    "targetTip" => target_tip = <Option<String> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("targetTip"))?,
                    "selected" => selected = <Option<bool> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("selected"))?,
                    "style" => style = <Option<String> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("style"))?,
                    "userData" => user_data = if matches!(entry, dsl::DslValue::Null) { None } else { Some(serde_json::Value::from(&entry)) },
                    "visible" => visible = <Option<bool> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("visible"))?,
                    "locked" => locked = <Option<bool> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("locked"))?,
                    _ => {}
                }
            }
            Ok(EdgeDescJson {
                id: id.ok_or_else(|| dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "EdgeDescJson missing id"))?,
                source: source.ok_or_else(|| dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "EdgeDescJson missing source"))?,
                target: target.ok_or_else(|| dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "EdgeDescJson missing target"))?,
                edge_kind,
                source_tip,
                target_tip,
                selected,
                style,
                user_data,
                visible,
                locked,
            })
        }
    }

    /// 🧵️ Transient cubic link from a handle to another handle or a free world point (descriptor + link gesture).
    ///
    /// 🌉️ Hand-written, not derived: `user_data: Option<serde_json::Value>` — same reason as
    /// `EdgeDescJson` above.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WireDescJson {
        pub id: String,
        pub source: String,
        /// 🧩️ Semantic wire-kind id (defaults from catalog when omitted in fixtures).
        #[serde(default)]
        pub wire_kind: Option<String>,
        #[serde(default)]
        pub target: Option<String>,
        #[serde(default)]
        pub end_x: Option<f64>,
        #[serde(default)]
        pub end_y: Option<f64>,
        #[serde(default)]
        pub selected: Option<bool>,
        #[serde(default)]
        pub style: Option<String>,
        #[serde(default)]
        pub user_data: Option<serde_json::Value>,
        #[serde(default)]
        pub visible: Option<bool>,
        #[serde(default)]
        pub locked: Option<bool>,
    }

    impl dsl::ToValue for WireDescJson {
        fn to_value(&self) -> dsl::DslValue {
            dsl::DslValue::object([
                ("id".to_string(), dsl::ToValue::to_value(&self.id)),
                ("source".to_string(), dsl::ToValue::to_value(&self.source)),
                ("wireKind".to_string(), dsl::ToValue::to_value(&self.wire_kind)),
                ("target".to_string(), dsl::ToValue::to_value(&self.target)),
                ("endX".to_string(), dsl::ToValue::to_value(&self.end_x)),
                ("endY".to_string(), dsl::ToValue::to_value(&self.end_y)),
                ("selected".to_string(), dsl::ToValue::to_value(&self.selected)),
                ("style".to_string(), dsl::ToValue::to_value(&self.style)),
                (
                    "userData".to_string(),
                    match &self.user_data {
                        Some(v) => dsl::DslValue::from(v),
                        None => dsl::DslValue::Null,
                    },
                ),
                ("visible".to_string(), dsl::ToValue::to_value(&self.visible)),
                ("locked".to_string(), dsl::ToValue::to_value(&self.locked)),
            ])
        }
    }

    impl dsl::FromValue for WireDescJson {
        fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
            let dsl::DslValue::Object(fields) = value else {
                return Err(dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for WireDescJson, found {value:?}")));
            };
            let mut id = None;
            let mut source = None;
            let mut wire_kind = None;
            let mut target = None;
            let mut end_x = None;
            let mut end_y = None;
            let mut selected = None;
            let mut style = None;
            let mut user_data = None;
            let mut visible = None;
            let mut locked = None;
            for (key, entry) in fields {
                match key.as_str() {
                    "id" => id = Some(<String as dsl::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                    "source" => source = Some(<String as dsl::FromValue>::from_value(entry).map_err(|e| e.under("source"))?),
                    "wireKind" => wire_kind = <Option<String> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("wireKind"))?,
                    "target" => target = <Option<String> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,
                    "endX" => end_x = <Option<f64> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("endX"))?,
                    "endY" => end_y = <Option<f64> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("endY"))?,
                    "selected" => selected = <Option<bool> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("selected"))?,
                    "style" => style = <Option<String> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("style"))?,
                    "userData" => user_data = if matches!(entry, dsl::DslValue::Null) { None } else { Some(serde_json::Value::from(&entry)) },
                    "visible" => visible = <Option<bool> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("visible"))?,
                    "locked" => locked = <Option<bool> as dsl::FromValue>::from_value(entry).map_err(|e| e.under("locked"))?,
                    _ => {}
                }
            }
            Ok(WireDescJson {
                id: id.ok_or_else(|| dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "WireDescJson missing id"))?,
                source: source.ok_or_else(|| dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "WireDescJson missing source"))?,
                wire_kind,
                target,
                end_x,
                end_y,
                selected,
                style,
                user_data,
                visible,
                locked,
            })
        }
    }

    /// 🎯️ One `targetRegions` row of the owning document, as the descriptor spells it. Derived, not
    /// hand-written: unlike its node/edge siblings a region carries no free-form `userData`.
    #[derive(Clone, Debug, Default, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct RegionDescJson {
        pub id: String,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
        #[serde(default)]
        #[value(default)]
        pub label: Option<String>,
        #[serde(default)]
        #[value(default)]
        pub hidden: Option<bool>,
        #[serde(default)]
        #[value(default)]
        pub locked: Option<bool>,
        #[serde(default)]
        #[value(default)]
        pub selected: Option<bool>,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct SceneDescriptorJson {
        pub nodes: Vec<NodeDescJson>,
        pub handles: Vec<HandleDescJson>,
        pub edges: Vec<EdgeDescJson>,
        #[serde(default)]
        #[value(default)]
        pub wires: Vec<WireDescJson>,
        /// 🎯️ Fill-constraining rectangles painted beneath every entity and hit-tested after
        /// all of them — absent on a board that declares none.
        #[serde(default)]
        #[value(default)]
        pub regions: Vec<RegionDescJson>,
        /// 💠️ JS‑authored ids to paint with secondary “left selection” chrome (not in current `selected` flags).
        #[serde(default)]
        #[value(default)]
        pub selection_exit_highlight_ids: Vec<String>,
    }

    /// 🌉️ Hand-written, not derived: `nodes`/`edges: Vec<serde_json::Value>` and
    /// `meta: Option<serde_json::Value>` have no `ToValue`/`FromValue` for `serde_json::Value` —
    /// same reason as `EdgeDescJson` above.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FixtureJson {
        pub schema: String,
        /// 🎥️ Absent in every document whose camera is session state the host owns (puzzle 2d
        /// since its `setCamera` became a View-kind verb): the parse keeps the camera it is looking
        /// through instead of refusing the document and blanking the board.
        #[serde(default)]
        pub camera: Option<CameraJson>,
        pub nodes: Vec<serde_json::Value>,
        pub edges: Vec<serde_json::Value>,
        /// 🎯️ The document's fill-constraining rectangles. Absent (never `[]`) on a board that
        /// has none, exactly as the owning artifact's snapshot omits an empty `targetRegions`.
        #[serde(default, rename = "targetRegions")]
        pub target_regions: Vec<serde_json::Value>,
        #[serde(default)]
        pub meta: Option<serde_json::Value>,
    }

    impl dsl::ToValue for FixtureJson {
        fn to_value(&self) -> dsl::DslValue {
            dsl::DslValue::object([
                ("schema".to_string(), dsl::ToValue::to_value(&self.schema)),
                (
                    "camera".to_string(),
                    match &self.camera {
                        Some(camera) => dsl::ToValue::to_value(camera),
                        None => dsl::DslValue::Null,
                    },
                ),
                ("nodes".to_string(), dsl::DslValue::Array(self.nodes.iter().map(dsl::DslValue::from).collect())),
                ("edges".to_string(), dsl::DslValue::Array(self.edges.iter().map(dsl::DslValue::from).collect())),
                ("targetRegions".to_string(), dsl::DslValue::Array(self.target_regions.iter().map(dsl::DslValue::from).collect())),
                (
                    "meta".to_string(),
                    match &self.meta {
                        Some(v) => dsl::DslValue::from(v),
                        None => dsl::DslValue::Null,
                    },
                ),
            ])
        }
    }

    impl dsl::FromValue for FixtureJson {
        fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
            let dsl::DslValue::Object(fields) = value else {
                return Err(dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FixtureJson, found {value:?}")));
            };
            let mut schema = None;
            let mut camera = None;
            let mut nodes = Vec::new();
            let mut edges = Vec::new();
            let mut target_regions = Vec::new();
            let mut meta = None;
            for (key, entry) in fields {
                match key.as_str() {
                    "schema" => schema = Some(<String as dsl::FromValue>::from_value(entry).map_err(|e| e.under("schema"))?),
                    "camera" => camera = if matches!(entry, dsl::DslValue::Null) { None } else { Some(<CameraJson as dsl::FromValue>::from_value(entry).map_err(|e| e.under("camera"))?) },
                    "nodes" => {
                        let dsl::DslValue::Array(items) = entry else {
                            return Err(dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for nodes").under("nodes"));
                        };
                        nodes = items.iter().map(serde_json::Value::from).collect();
                    }
                    "edges" => {
                        let dsl::DslValue::Array(items) = entry else {
                            return Err(dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for edges").under("edges"));
                        };
                        edges = items.iter().map(serde_json::Value::from).collect();
                    }
                    "targetRegions" => {
                        let dsl::DslValue::Array(items) = entry else {
                            return Err(dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for targetRegions").under("targetRegions"));
                        };
                        target_regions = items.iter().map(serde_json::Value::from).collect();
                    }
                    "meta" => meta = if matches!(entry, dsl::DslValue::Null) { None } else { Some(serde_json::Value::from(&entry)) },
                    _ => {}
                }
            }
            Ok(FixtureJson { schema: schema.ok_or_else(|| dsl::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "FixtureJson missing schema"))?, camera, nodes, edges, target_regions, meta })
        }
    }

    /// 🧾️ Reads fixture edge endpoint handle ids from `source` and `target` string fields only.
    pub fn fixture_edge_handle_ids_from_object(eo: &serde_json::Map<String, serde_json::Value>) -> Option<(&str, &str)> {
        let source = eo.get("source").and_then(|v| v.as_str())?;
        let target = eo.get("target").and_then(|v| v.as_str())?;
        Some((source, target))
    }

    fn board_json_hidden_flag(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
        obj.get("hidden").and_then(|v| v.as_bool())
    }

    pub fn board_json_visible_option(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
        match board_json_hidden_flag(obj) {
            Some(hidden) => Some(!hidden),
            None => obj.get("visible").and_then(|v| v.as_bool()),
        }
    }

    pub fn board_json_visible_or_true(obj: &serde_json::Map<String, serde_json::Value>) -> bool {
        board_json_visible_option(obj).unwrap_or(true)
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub fn normalize_board_descriptor_hidden_to_visible(value: &mut serde_json::Value) {
        let Some(root) = value.as_object_mut() else {
            return;
        };
        for key in ["nodes", "handles", "edges", "wires"] {
            let Some(rows) = root.get_mut(key).and_then(|v| v.as_array_mut()) else {
                continue;
            };
            for row in rows {
                let Some(obj) = row.as_object_mut() else {
                    continue;
                };
                if let Some(visible) = board_json_visible_option(obj) {
                    obj.insert("visible".into(), serde_json::json!(visible));
                }
            }
        }
    }
    // #endregion scene_json
}

pub mod types {
    // #region types
    //! 🧩️ Directed port graph board types shared by normal and dag leaves.

    use std::collections::{BTreeMap, BTreeSet};

    use super::canvas::camera::Camera;
    use super::canvas::Color;
    use super::canvas::{Point, Vec2};
    use super::NodeKindHandleTemplate;

    // #region 🔖️GraphPortMode
    /// 🔌️ Runtime port-model axis: ported graphs use handles; normal graphs connect node ids directly.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum GraphPortMode {
        #[default]
        Ported,
        Normal,
    }

    impl GraphPortMode {
        pub fn has_ports(self) -> bool {
            self == GraphPortMode::Ported
        }
    }
    // #endregion 🔖️GraphPortMode

    pub use graph::NodeShape;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BoardElementStyleKind {
        Original,
        Neutral,
        Hovered,
        Selected,
        Highlighted,
        Disabled,
    }

    #[derive(Clone, Debug)]
    pub struct NodeData {
        pub id: String,
        pub x: f64,
        pub y: f64,
        pub shape: NodeShape,
        pub radius: f64,
        pub width: f64,
        pub height: f64,
        pub scale: f64,
        pub draggable: bool,
        pub selected: bool,
        pub visible: bool,
        pub locked: bool,
        pub root: bool,
        pub style: Option<String>,
        pub text: Option<String>,
        pub icon_kind: Option<String>,
        pub node_kind: String,
        pub properties: graph::PropertyBag,
    }

    #[derive(Clone, Debug)]
    pub struct WireKindDef {
        pub name: String,
        pub default_edge_kind: Option<String>,
    }

    #[derive(Clone, Debug)]
    pub struct NodeKindDef {
        pub name: String,
        pub scale: f64,
        pub shape: NodeShape,
        pub handles: Vec<NodeKindHandleTemplate>,
        pub icon: Option<String>,
        pub color_fill: Option<Color>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ActiveUtility {
        Select,
        Brush,
        /// 🖍️ Paints target regions: a click-drag rectangle, or a click that drops one of the
        /// configured brush extent. Never picks, never marquees.
        AreaBrush,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EdgeStrokePattern {
        Solid,
        Dashed,
        Dotted,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EdgeTipGeometry {
        Arrow,
        FineArrow,
        Diamond,
        Circle,
        Bar,
    }

    #[derive(Clone, Debug)]
    pub struct EdgeTipDef {
        pub geometry: EdgeTipGeometry,
        pub filled: bool,
        pub scale: f64,
    }

    impl EdgeTipDef {
        pub fn from_catalog_row(eo: &serde_json::Map<String, serde_json::Value>) -> Option<Self> {
            let id = eo.get("id").and_then(|x| x.as_str()).unwrap_or("");
            if eo.get("geometry").is_none() {
                return Self::builtin_for_id(id);
            }
            let geometry = match eo.get("geometry").and_then(|x| x.as_str()).map(str::trim) {
                Some("arrow") => EdgeTipGeometry::Arrow,
                Some("fine-arrow") | Some("fine_arrow") => EdgeTipGeometry::FineArrow,
                Some("diamond") => EdgeTipGeometry::Diamond,
                Some("circle") => EdgeTipGeometry::Circle,
                Some("bar") => EdgeTipGeometry::Bar,
                _ => return None,
            };
            let filled = eo.get("filled").and_then(|x| x.as_bool()).unwrap_or_else(|| match geometry {
                EdgeTipGeometry::FineArrow | EdgeTipGeometry::Bar => false,
                EdgeTipGeometry::Diamond => eo.get("id").and_then(|x| x.as_str()).is_some_and(|id| id.contains("open")),
                _ => true,
            });
            let scale = eo.get("scale").and_then(|x| x.as_f64()).filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0);
            Some(Self { geometry, filled, scale })
        }

        pub fn builtin_for_id(id: &str) -> Option<Self> {
            match id.trim().to_ascii_lowercase().as_str() {
                "arrow" | "filled-arrow" | "filled_arrow" => Some(Self { geometry: EdgeTipGeometry::Arrow, filled: true, scale: 1.0 }),
                "fine-arrow" | "fine_arrow" => Some(Self { geometry: EdgeTipGeometry::FineArrow, filled: false, scale: 1.0 }),
                "filled-diamond" | "filled_diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: true, scale: 1.0 }),
                "open-diamond" | "open_diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: false, scale: 1.0 }),
                _ => None,
            }
        }
    }

    pub fn builtin_edge_tips() -> BTreeMap<String, EdgeTipDef> {
        let ids = ["arrow", "filled-arrow", "fine-arrow", "filled-diamond", "open-diamond"];
        let mut m = BTreeMap::new();
        for id in ids {
            if let Some(def) = EdgeTipDef::builtin_for_id(id) {
                m.insert(id.to_string(), def);
            }
        }
        m
    }

    #[derive(Clone, Debug)]
    pub struct EdgeKindDef {
        pub name: String,
        pub color: Option<Color>,
        pub stroke_width: f64,
        pub pattern: EdgeStrokePattern,
        pub source_tip: Option<String>,
        pub target_tip: Option<String>,
        pub directed: bool,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CompatSpecificity {
        General = 0,
        Node = 1,
        Edge = 2,
        Handle = 3,
        Wire = 4,
    }

    #[derive(Clone, Debug)]
    pub struct LinkCompatRule {
        pub source: String,
        pub target: String,
        pub bidirectional: bool,
        pub important: bool,
        pub specificity: CompatSpecificity,
    }

    #[derive(Clone, Debug)]
    pub struct EdgeData {
        pub id: String,
        pub source: String,
        pub target: String,
        pub selected: bool,
        pub visible: bool,
        pub locked: bool,
        pub style: Option<String>,
        pub edge_kind: String,
        pub source_tip: Option<String>,
        pub target_tip: Option<String>,
        pub properties: graph::PropertyBag,
    }

    #[derive(Clone, Debug)]
    pub struct WireData {
        pub id: String,
        pub source: String,
        pub target: Option<String>,
        pub end_x: Option<f64>,
        pub end_y: Option<f64>,
        pub selected: bool,
        pub visible: bool,
        pub locked: bool,
        pub style: Option<String>,
        pub wire_kind: String,
        pub properties: graph::PropertyBag,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SelectionOptions {
        pub method: String,
        pub mode: String,
        pub select_nodes: bool,
        pub select_edges: bool,
        pub select_handles: bool,
    }

    /// 🪪️ One pointer gesture's identity and the selection change its press staged. The press changes the
    /// engine selection at once; its `select` row waits for the release, so `select` and the gesture record
    /// leave as ONE batch tagged with `id`, and a cancel restores `restore` and publishes nothing.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct GestureStage {
        pub id: String,
        pub select: Option<(Vec<String>, Option<String>)>,
        pub restore: Option<BTreeSet<String>>,
    }

    #[derive(Clone, Debug, Default)]
    pub enum Interaction {
        #[default]
        None,
        Pan { origin: Camera, start_screen: Point },
        DragNodes { offset: Vec2, primary_id: String, start_positions: BTreeMap<String, (f64, f64)>, proximity_pair: Option<(String, String)>, gesture: GestureStage, delta: Vec2 },
        SelectionPending { initial_ids: BTreeSet<String>, start: Point, start_screen: Point },
        Selection { initial_ids: BTreeSet<String>, points: Vec<Point>, screen_points: Vec<Point>, start: Point, start_screen: Point },
        LinkAtSourceHandle { source_id: String, start_screen: Point },
        LinkDragSnap { source_id: String, target_id: Option<String>, end_world: Point },
        LinkTargetNode { source_id: String, target_node_id: String },
        ExternalLinkPreview { source_id: String, end_world: Point, compatible_node_ids: Vec<String>, ring_node_id: Option<String>, ring_handle_ids: Vec<String> },
    }

    #[derive(Clone, Copy, Debug)]
    pub struct CanvasPalette {
        pub raster_clear: Color,
        pub grid_minor_stroke: Color,
        pub edge_stroke: Color,
        pub edge_stroke_hovered: Color,
        pub edge_stroke_selected: Color,
        pub edge_stroke_selection_exit: Color,
        pub edge_stroke_disabled: Color,
        pub node_fill: Color,
        pub node_stroke: Color,
        pub node_fill_hovered: Color,
        pub node_stroke_hovered: Color,
        pub node_fill_selected: Color,
        pub node_stroke_selected: Color,
        pub node_fill_selection_exit: Color,
        pub node_stroke_selection_exit: Color,
        pub node_fill_disabled: Color,
        pub node_stroke_disabled: Color,
        pub node_stroke_computing: Color,
        pub node_stroke_stale: Color,
        pub node_stroke_error: Color,
        pub node_stroke_blocked: Color,
        pub indirect_handle_fill: Color,
        pub indirect_handle_stroke: Color,
        pub handle_fill: Color,
        pub handle_stroke: Color,
        pub handle_fill_hovered: Color,
        pub handle_stroke_hovered: Color,
        pub handle_fill_selected: Color,
        pub handle_stroke_selected: Color,
        pub handle_fill_selection_exit: Color,
        pub handle_stroke_selection_exit: Color,
        pub handle_fill_disabled: Color,
        pub handle_stroke_disabled: Color,
        pub wire_stroke: Color,
        pub wire_stroke_hovered: Color,
        pub wire_stroke_selected: Color,
        pub wire_stroke_highlighted: Color,
        pub wire_stroke_disabled: Color,
        pub selection_preview_fill: Color,
        pub selection_preview_stroke: Color,
        pub label_fill: Color,
        pub label_fill_hovered: Color,
        pub label_halo: Color,
        pub minimap_widget_panel_fill: Color,
        pub minimap_widget_panel_stroke: Color,
        pub minimap_widget_viewport_fill: Color,
        pub minimap_widget_viewport_stroke: Color,
        pub minimap_widget_viewport_stroke_hovered: Color,
    }

    impl CanvasPalette {
        /// 🎨️ Builds a palette from centralized board theme tokens.
        pub fn from_board_palette(t: &ui_styling::BoardPalette) -> Self {
            Self {
                raster_clear: Color::new(t.raster_clear),
                grid_minor_stroke: Color::new(t.grid_minor_stroke),
                edge_stroke: Color::new(t.edge_stroke),
                edge_stroke_hovered: Color::new(t.edge_stroke_hovered),
                edge_stroke_selected: Color::new(t.edge_stroke_selected),
                edge_stroke_selection_exit: Color::new(t.edge_stroke_selection_exit),
                edge_stroke_disabled: Color::new(t.edge_stroke_disabled),
                node_fill: Color::new(t.node_fill),
                node_stroke: Color::new(t.node_stroke),
                node_fill_hovered: Color::new(t.node_fill_hovered),
                node_stroke_hovered: Color::new(t.node_stroke_hovered),
                node_fill_selected: Color::new(t.node_fill_selected),
                node_stroke_selected: Color::new(t.node_stroke_selected),
                node_fill_selection_exit: Color::new(t.node_fill_selection_exit),
                node_stroke_selection_exit: Color::new(t.node_stroke_selection_exit),
                node_fill_disabled: Color::new(t.node_fill_disabled),
                node_stroke_disabled: Color::new(t.node_stroke_disabled),
                node_stroke_computing: Color::new(t.node_stroke_computing),
                node_stroke_stale: Color::new(t.node_stroke_stale),
                node_stroke_error: Color::new(t.node_stroke_error),
                node_stroke_blocked: Color::new(t.node_stroke_blocked),
                indirect_handle_fill: Color::new(t.indirect_handle_fill),
                indirect_handle_stroke: Color::new(t.indirect_handle_stroke),
                handle_fill: Color::new(t.handle_fill),
                handle_stroke: Color::new(t.handle_stroke),
                handle_fill_hovered: Color::new(t.handle_fill_hovered),
                handle_stroke_hovered: Color::new(t.handle_stroke_hovered),
                handle_fill_selected: Color::new(t.handle_fill_selected),
                handle_stroke_selected: Color::new(t.handle_stroke_selected),
                handle_fill_selection_exit: Color::new(t.handle_fill_selection_exit),
                handle_stroke_selection_exit: Color::new(t.handle_stroke_selection_exit),
                handle_fill_disabled: Color::new(t.handle_fill_disabled),
                handle_stroke_disabled: Color::new(t.handle_stroke_disabled),
                wire_stroke: Color::new(t.wire_stroke),
                wire_stroke_hovered: Color::new(t.wire_stroke_hovered),
                wire_stroke_selected: Color::new(t.wire_stroke_selected),
                wire_stroke_highlighted: Color::new(t.wire_stroke_highlighted),
                wire_stroke_disabled: Color::new(t.wire_stroke_disabled),
                selection_preview_fill: Color::new(t.selection_preview_fill),
                selection_preview_stroke: Color::new(t.selection_preview_stroke),
                label_fill: Color::new(t.label_fill),
                label_fill_hovered: Color::new(t.label_fill_hovered),
                label_halo: Color::new(t.label_halo),
                minimap_widget_panel_fill: Color::new(t.minimap_widget_panel_fill),
                minimap_widget_panel_stroke: Color::new(t.minimap_widget_panel_stroke),
                minimap_widget_viewport_fill: Color::new(t.minimap_widget_viewport_fill),
                minimap_widget_viewport_stroke: Color::new(t.minimap_widget_viewport_stroke),
                minimap_widget_viewport_stroke_hovered: Color::new(t.minimap_widget_viewport_stroke_hovered),
            }
        }

        fn merge_color_field(next: &mut Color, v: &serde_json::Value, key: &str) {
            infinite::canvas::theme::merge_color_field(next, v, key);
        }

        /// 🎨️ Replaces this palette from the React host UI theme JSON payload.
        pub fn merge_from_json(&mut self, json: &str) -> Result<(), String> {
            let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
            let mut next = Self::default();
            Self::merge_color_field(&mut next.raster_clear, &v, "rasterClear");
            Self::merge_color_field(&mut next.grid_minor_stroke, &v, "gridMinorStroke");
            Self::merge_color_field(&mut next.edge_stroke, &v, "edgeStroke");
            Self::merge_color_field(&mut next.edge_stroke_hovered, &v, "edgeStrokeHovered");
            Self::merge_color_field(&mut next.edge_stroke_selected, &v, "edgeStrokeSelected");
            Self::merge_color_field(&mut next.edge_stroke_selection_exit, &v, "edgeStrokeSelectionExit");
            Self::merge_color_field(&mut next.edge_stroke_disabled, &v, "edgeStrokeDisabled");
            Self::merge_color_field(&mut next.node_fill, &v, "nodeFill");
            Self::merge_color_field(&mut next.node_stroke, &v, "nodeStroke");
            Self::merge_color_field(&mut next.node_fill_hovered, &v, "nodeFillHovered");
            Self::merge_color_field(&mut next.node_stroke_hovered, &v, "nodeStrokeHovered");
            Self::merge_color_field(&mut next.node_fill_selected, &v, "nodeFillSelected");
            Self::merge_color_field(&mut next.node_stroke_selected, &v, "nodeStrokeSelected");
            Self::merge_color_field(&mut next.node_fill_selection_exit, &v, "nodeFillSelectionExit");
            Self::merge_color_field(&mut next.node_stroke_selection_exit, &v, "nodeStrokeSelectionExit");
            Self::merge_color_field(&mut next.node_fill_disabled, &v, "nodeFillDisabled");
            Self::merge_color_field(&mut next.node_stroke_disabled, &v, "nodeStrokeDisabled");
            Self::merge_color_field(&mut next.indirect_handle_fill, &v, "indirectHandleFill");
            Self::merge_color_field(&mut next.indirect_handle_stroke, &v, "indirectHandleStroke");
            Self::merge_color_field(&mut next.handle_fill, &v, "handleFill");
            Self::merge_color_field(&mut next.handle_stroke, &v, "handleStroke");
            Self::merge_color_field(&mut next.handle_fill_hovered, &v, "handleFillHovered");
            Self::merge_color_field(&mut next.handle_stroke_hovered, &v, "handleStrokeHovered");
            Self::merge_color_field(&mut next.handle_fill_selected, &v, "handleFillSelected");
            Self::merge_color_field(&mut next.handle_stroke_selected, &v, "handleStrokeSelected");
            Self::merge_color_field(&mut next.handle_fill_selection_exit, &v, "handleFillSelectionExit");
            Self::merge_color_field(&mut next.handle_stroke_selection_exit, &v, "handleStrokeSelectionExit");
            Self::merge_color_field(&mut next.handle_fill_disabled, &v, "handleFillDisabled");
            Self::merge_color_field(&mut next.handle_stroke_disabled, &v, "handleStrokeDisabled");
            Self::merge_color_field(&mut next.wire_stroke, &v, "wireStroke");
            Self::merge_color_field(&mut next.wire_stroke_hovered, &v, "wireStrokeHovered");
            Self::merge_color_field(&mut next.wire_stroke_selected, &v, "wireStrokeSelected");
            Self::merge_color_field(&mut next.wire_stroke_highlighted, &v, "wireStrokeHighlighted");
            Self::merge_color_field(&mut next.wire_stroke_disabled, &v, "wireStrokeDisabled");
            Self::merge_color_field(&mut next.selection_preview_fill, &v, "selectionPreviewFill");
            Self::merge_color_field(&mut next.selection_preview_stroke, &v, "selectionPreviewStroke");
            Self::merge_color_field(&mut next.label_fill, &v, "labelFill");
            Self::merge_color_field(&mut next.label_fill_hovered, &v, "labelFillHovered");
            Self::merge_color_field(&mut next.label_halo, &v, "labelHalo");
            Self::merge_color_field(&mut next.minimap_widget_panel_fill, &v, "minimapWidgetPanelFill");
            Self::merge_color_field(&mut next.minimap_widget_panel_stroke, &v, "minimapWidgetPanelStroke");
            Self::merge_color_field(&mut next.minimap_widget_viewport_fill, &v, "minimapWidgetViewportFill");
            Self::merge_color_field(&mut next.minimap_widget_viewport_stroke, &v, "minimapWidgetViewportStroke");
            Self::merge_color_field(&mut next.minimap_widget_viewport_stroke_hovered, &v, "minimapWidgetViewportStrokeHovered");
            *self = next;
            Ok(())
        }
    }

    // #region 🔖️Icons
    use std::cell::{Cell, RefCell};
    #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
    use std::hash::{Hash, Hasher};
    use std::mem::ManuallyDrop;
    use std::sync::Arc;

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use super::canvas::{append_svg_document, SvgDocument};
    use super::canvas::{Affine, FillRule, RasterImage, Rect, Scene};

    pub enum CachedIconBody {
        Vector(Scene),
        Raster(Arc<RasterImage>),
    }

    struct CachedIconPaint {
        bx: f64,
        by: f64,
        bw: f64,
        bh: f64,
        body: CachedIconBody,
    }

    const ICON_PAINT_CACHE_CAPACITY: usize = 256;
    #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
    const ICON_PAINT_CACHE_KEY_BYTE_CAPACITY: usize = 256;
    #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
    const ICON_PAINT_SOURCE_BYTE_CAPACITY: usize = 16 * 1024;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
    struct IconPaintToken {
        slot: u16,
        generation: u64,
    }

    struct IconPaintSlot {
        key: Option<String>,
        epoch: u64,
        generation: u64,
        value: Option<CachedIconPaint>,
    }

    struct IconPaintRegistry {
        slots: Box<[IconPaintSlot; ICON_PAINT_CACHE_CAPACITY]>,
        epoch: u64,
        faulted: bool,
    }

    impl Default for IconPaintRegistry {
        fn default() -> Self {
            Self { slots: semio_framework_async::boxed_fixed_slots(|| IconPaintSlot { key: None, epoch: 0, generation: 0, value: None }), epoch: 1, faulted: false }
        }
    }

    impl IconPaintRegistry {
        #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
        fn index(&self, key: &str) -> Option<usize> {
            self.slots.iter().position(|slot| slot.epoch == self.epoch && slot.key.as_deref() == Some(key) && slot.value.is_some())
        }

        #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
        fn get(&self, key: &str) -> Option<&CachedIconPaint> {
            self.slots.get(self.index(key)?)?.value.as_ref()
        }

        #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
        fn reserve(&mut self, key: &str) -> Option<IconPaintToken> {
            if key.len() > ICON_PAINT_CACHE_KEY_BYTE_CAPACITY || self.get(key).is_some() {
                self.faulted = true;
                return None;
            }
            let Some(index) = self.slots.iter().position(|slot| slot.key.is_none() && slot.value.is_none()) else {
                self.faulted = true;
                return None;
            };
            let slot = &mut self.slots[index];
            slot.generation = slot.generation.wrapping_add(1).max(1);
            slot.epoch = self.epoch;
            slot.key = Some(key.to_owned());
            Some(IconPaintToken { slot: index as u16, generation: slot.generation })
        }

        #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
        fn publish(&mut self, token: IconPaintToken, value: CachedIconPaint) {
            let slot = self.slots.get_mut(usize::from(token.slot)).expect("reserved icon cache slot remains present");
            assert_eq!(slot.generation, token.generation, "reserved icon cache generation remains current");
            assert_eq!(slot.epoch, self.epoch, "reserved icon cache epoch remains current");
            assert!(slot.key.is_some() && slot.value.is_none(), "reserved icon cache slot remains unpublished");
            slot.value = Some(value);
        }

        #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
        fn abort(&mut self, token: IconPaintToken) {
            let slot = self.slots.get_mut(usize::from(token.slot)).expect("reserved icon cache slot remains present");
            assert_eq!(slot.generation, token.generation, "aborted icon cache generation remains current");
            assert!(slot.value.is_none(), "only an unpublished icon cache reservation can abort");
            slot.key = None;
            slot.epoch = 0;
            slot.generation = slot.generation.wrapping_add(1).max(1);
        }

        fn invalidate(&mut self) {
            self.epoch = self.epoch.wrapping_add(1).max(1);
        }
    }

    /// 🖼️ Center and available screen extents for fitting one icon without coordinate conversion.
    #[derive(Clone, Copy)]
    pub struct IconScreenRect {
        pub center: Point,
        pub width: f64,
        pub height: f64,
    }

    /// 🖼️ Shared SVG/raster icon decode cache for board and DAG hosts.
    pub struct IconPaintCache {
        cache: RefCell<ManuallyDrop<IconPaintRegistry>>,
        retirement_cursor: Cell<u16>,
        retirement_credited_bytes: Cell<usize>,
        retirement_scene: Cell<Option<infinite::canvas::OpaqueSceneRetirementToken>>,
        closing: Cell<bool>,
        pub themed_icon_lookup: infinite::canvas::icon_codec::ThemedSvgLookup,
    }

    /// 📸️ One icon-retirement turn with current credit and physical release split.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IconPaintRetirementStep {
        Blocked,
        Pending { released_items: usize, credited_bytes: usize, released_bytes: usize },
        Complete,
    }

    pub struct CachedIconPaintLease<'a> {
        cache: std::cell::Ref<'a, ManuallyDrop<IconPaintRegistry>>,
        slot: usize,
    }

    impl CachedIconPaintLease<'_> {
        pub fn bounds(&self) -> (f64, f64, f64, f64) {
            let paint = self.cache.slots[self.slot].value.as_ref().expect("leased icon cache slot remains published");
            (paint.bx, paint.by, paint.bw, paint.bh)
        }

        pub fn body(&self) -> &CachedIconBody {
            &self.cache.slots[self.slot].value.as_ref().expect("leased icon cache slot remains published").body
        }
    }

    impl Default for IconPaintCache {
        fn default() -> Self {
            Self {
                cache: RefCell::new(ManuallyDrop::new(IconPaintRegistry::default())),
                retirement_cursor: Cell::new(0),
                retirement_credited_bytes: Cell::new(0),
                retirement_scene: Cell::new(None),
                closing: Cell::new(false),
                themed_icon_lookup: |_| None,
            }
        }
    }

    impl Clone for IconPaintCache {
        fn clone(&self) -> Self {
            Self {
                cache: RefCell::new(ManuallyDrop::new(IconPaintRegistry::default())),
                retirement_cursor: Cell::new(0),
                retirement_credited_bytes: Cell::new(0),
                retirement_scene: Cell::new(None),
                closing: Cell::new(false),
                themed_icon_lookup: self.themed_icon_lookup,
            }
        }
    }

    impl Drop for IconPaintCache {
        fn drop(&mut self) {
            let terminal = self.terminal_is_empty();
            let never_admitted = self.retirement_scene.get().is_none() && self.cache.get_mut().slots.iter().all(|slot| slot.key.is_none() && slot.value.is_none());
            debug_assert!(terminal || never_admitted || std::thread::panicking(), "IconPaintCache with admitted resources must reach terminal-empty through close_step before release");
            if terminal || never_admitted {
                unsafe { ManuallyDrop::drop(self.cache.get_mut()) };
            }
        }
    }

    impl IconPaintCache {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn clear(&self) {
            assert!(self.retirement_scene.get().is_none(), "icon cache invalidation cannot detach an admitted scene retirement");
            assert_eq!(self.retirement_credited_bytes.get(), 0, "icon cache invalidation cannot detach admitted byte credit");
            self.cache.borrow_mut().invalidate();
            self.retirement_cursor.set(0);
        }

        pub fn close_step(&self) -> bool {
            matches!(self.close_page(1, usize::MAX), IconPaintRetirementStep::Complete)
        }

        pub fn close_page(&self, maximum_items: usize, maximum_bytes: usize) -> IconPaintRetirementStep {
            self.closing.set(true);
            if maximum_items == 0 || maximum_bytes == 0 {
                return IconPaintRetirementStep::Blocked;
            }
            if let Some(token) = self.retirement_scene.get() {
                return match infinite::canvas::advance_opaque_scene_retirement(token, maximum_items, maximum_bytes) {
                    infinite::canvas::OpaqueSceneRetirementStep::Blocked => IconPaintRetirementStep::Blocked,
                    infinite::canvas::OpaqueSceneRetirementStep::Pending { released_items, credited_bytes, released_bytes } => IconPaintRetirementStep::Pending { released_items, credited_bytes, released_bytes },
                    infinite::canvas::OpaqueSceneRetirementStep::Complete { released_items, credited_bytes, released_bytes } => {
                        self.retirement_scene.set(None);
                        IconPaintRetirementStep::Pending { released_items, credited_bytes, released_bytes }
                    }
                    infinite::canvas::OpaqueSceneRetirementStep::Fault => {
                        self.cache.borrow_mut().faulted = true;
                        IconPaintRetirementStep::Blocked
                    }
                };
            }
            let index = usize::from(self.retirement_cursor.get());
            if index == ICON_PAINT_CACHE_CAPACITY {
                return IconPaintRetirementStep::Complete;
            }
            let mut cache = self.cache.borrow_mut();
            let slot = &mut cache.slots[index];
            let released_bytes = slot.key.as_ref().map_or(0, String::capacity).saturating_add(match slot.value.as_ref().map(|value| &value.body) {
                Some(CachedIconBody::Raster(image)) if Arc::strong_count(image) == 1 => image.retirement_exclusive_backing_bytes(),
                Some(CachedIconBody::Raster(_)) | Some(CachedIconBody::Vector(_)) | None => 0,
            });
            let remaining_bytes = released_bytes.saturating_sub(self.retirement_credited_bytes.get());
            let credited_bytes = maximum_bytes.min(remaining_bytes);
            self.retirement_credited_bytes.set(self.retirement_credited_bytes.get().saturating_add(credited_bytes));
            if self.retirement_credited_bytes.get() != released_bytes {
                return IconPaintRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 };
            }
            if let Some(CachedIconPaint { body: CachedIconBody::Vector(_), .. }) = slot.value.as_ref() {
                let Some(token) = infinite::canvas::reserve_opaque_scene_retirement() else {
                    cache.faulted = true;
                    return IconPaintRetirementStep::Blocked;
                };
                let paint = slot.value.take().expect("vector icon retirement slot remains occupied");
                let CachedIconBody::Vector(scene) = paint.body else {
                    unreachable!("vector icon retirement was witnessed before ownership transfer");
                };
                infinite::canvas::publish_opaque_scene_retirement(token, scene);
                self.retirement_scene.set(Some(token));
            } else {
                slot.value = None;
            }
            slot.key = None;
            slot.epoch = 0;
            slot.generation = slot.generation.wrapping_add(1).max(1);
            self.retirement_cursor.set((index + 1) as u16);
            self.retirement_credited_bytes.set(0);
            IconPaintRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes }
        }

        pub fn terminal_is_empty(&self) -> bool {
            self.closing.get()
                && usize::from(self.retirement_cursor.get()) == ICON_PAINT_CACHE_CAPACITY
                && self.retirement_credited_bytes.get() == 0
                && self.retirement_scene.get().is_none()
                && self.cache.borrow().slots.iter().all(|slot| slot.key.is_none() && slot.value.is_none())
        }

        pub fn faulted(&self) -> bool {
            self.cache.borrow().faulted
        }

        #[cfg(test)]
        pub(super) fn has_retiring_scene(&self) -> bool {
            self.retirement_scene.get().is_some()
        }

        #[cfg(test)]
        pub(crate) fn occupied_slots(&self) -> usize {
            self.cache.borrow().slots.iter().filter(|slot| slot.key.is_some()).count()
        }

        #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
        fn icon_vector_cache_key(tag: &str, svg: &str, fg: Color, bg: Color) -> String {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            svg.hash(&mut hasher);
            let hx = hasher.finish();
            let f = fg.to_rgba8();
            let b = bg.to_rgba8();
            format!("v8|{tag}|{hx:x}|{}|{:02x}{:02x}{:02x}{:02x}|{:02x}{:02x}{:02x}{:02x}", svg.len(), f.r, f.g, f.b, f.a, b.r, b.g, b.b, b.a)
        }

        #[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
        fn icon_raster_cache_key(rgba: &Arc<[u8]>, w: u32, h: u32) -> String {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            rgba.as_ref().hash(&mut hasher);
            let hx = hasher.finish();
            format!("v8|r|{w}x{h}|{hx:x}|{}", rgba.len())
        }

        /// 🖌️ Builds (or reuses a cached) icon paint — rasterizes SVG via `usvg`/`vello_svg`
        /// or decodes raster bytes via `image`, producing real pixels/vector paint for `Scene`.
        /// Host/browser only: a `wasm32-wasip2` guest has no display to paint onto, so this
        /// target has its own arm below that returns `None` unconditionally — the same value
        /// every caller here already treats as "nothing to paint", so no caller
        /// (`append_icon_at_screen_rect`, `paint_scene`, `build_vector_scene`, and trinity's own
        /// `TrinityBridge::paint_scene`, which is unreachable repo-wide as of this ticket, see
        /// `🔍️research/📓️intrinsic-size-wiring.md`) needs to change. Ticket
        /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        pub fn get_or_build(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
            if self.closing.get() {
                return None;
            }
            if encoded.len() > ICON_PAINT_SOURCE_BYTE_CAPACITY {
                self.cache.borrow_mut().faulted = true;
                return None;
            }
            let resolved = infinite::canvas::icon_codec::board_resolve_icon_kind(encoded, self.themed_icon_lookup);
            let key = match &resolved {
                infinite::canvas::icon_codec::BoardResolvedIcon::None => return None,
                infinite::canvas::icon_codec::BoardResolvedIcon::SvgThemed(s) | infinite::canvas::icon_codec::BoardResolvedIcon::SvgPlain(s) => Self::icon_vector_cache_key(if preserve_original_style { "p" } else { "t" }, s.as_str(), fg, bg),
                infinite::canvas::icon_codec::BoardResolvedIcon::RasterRgba8 { rgba, w, h } => Self::icon_raster_cache_key(rgba, *w, *h),
            };
            {
                let g = self.cache.borrow();
                if let Some(slot) = g.index(&key) {
                    return Some(CachedIconPaintLease { cache: g, slot });
                }
            }
            let token = self.cache.borrow_mut().reserve(&key)?;
            let (bx, by, bw, bh, body) = match resolved {
                infinite::canvas::icon_codec::BoardResolvedIcon::None => {
                    self.cache.borrow_mut().abort(token);
                    return None;
                }
                infinite::canvas::icon_codec::BoardResolvedIcon::SvgThemed(s) => {
                    let Some(doc) = SvgDocument::parse_icons(s.trim()).ok() else {
                        self.cache.borrow_mut().abort(token);
                        return None;
                    };
                    let (bx, by, bw, bh) = doc.content_bounds();
                    if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                        self.cache.borrow_mut().abort(token);
                        return None;
                    }
                    let mut s = Scene::new();
                    if preserve_original_style {
                        append_svg_document(&mut s, &doc);
                    } else {
                        doc.render_themed(&mut s, fg, bg);
                    }
                    (bx, by, bw, bh, CachedIconBody::Vector(s))
                }
                infinite::canvas::icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                    let Some(doc) = SvgDocument::parse_icons(s.trim()).ok() else {
                        self.cache.borrow_mut().abort(token);
                        return None;
                    };
                    let (bx, by, bw, bh) = doc.content_bounds();
                    if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                        self.cache.borrow_mut().abort(token);
                        return None;
                    }
                    let mut s = Scene::new();
                    if preserve_original_style {
                        append_svg_document(&mut s, &doc);
                    } else {
                        doc.render_themed(&mut s, fg, bg);
                    }
                    (bx, by, bw, bh, CachedIconBody::Vector(s))
                }
                infinite::canvas::icon_codec::BoardResolvedIcon::RasterRgba8 { rgba, w, h } => {
                    let bx = 0.0_f64;
                    let by = 0.0_f64;
                    let bw = f64::from(w);
                    let bh = f64::from(h);
                    let img = RasterImage::rgba8(w, h, Arc::new(rgba.as_ref().to_vec()));
                    (bx, by, bw, bh, CachedIconBody::Raster(Arc::new(img)))
                }
            };
            self.cache.borrow_mut().publish(token, CachedIconPaint { bx, by, bw, bh, body });
            let cache = self.cache.borrow();
            let slot = cache.index(&key).expect("published icon cache key remains indexed");
            Some(CachedIconPaintLease { cache, slot })
        }

        /// 🚫️ `wasm32-wasip2` arm: icon *painting* (rasterizing SVG/raster icon sources to
        /// `Scene` pixels/paths) is host-only by nature — a WASI guest component has no display
        /// to paint onto, and every real caller of icon painting on this target is either a
        /// browser bridge already excluded from `wasm32-wasip2` (`target_arch = "wasm32"` is TRUE
        /// for `wasm32-wasip2`, so those bridges use the narrower `not(target_env = "p2")` gate)
        /// or, for `semio-s-plugin-trinity`'s `TrinityBridge::paint_scene`, unreachable from any
        /// caller repo-wide. `None` here is not a stub for exercised behavior — it is the value
        /// every caller already treats as "nothing to paint", so nothing regresses if it is
        /// literally the only value this target ever produces. Ticket
        /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
        /// `🔍️research/📓️intrinsic-size-wiring.md`.
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        pub fn get_or_build(&self, _encoded: &str, _fg: Color, _bg: Color, _preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
            None
        }

        /// 🖼️ Paints an icon centered in a screen-space rectangle.
        pub fn append_icon_at_screen_rect(&self, scene: &mut Scene, icon_kind: &str, rect: IconScreenRect, fg: Color, bg: Color, preserve_original_style: bool) {
            let IconScreenRect { center, width: avail_w, height: avail_h } = rect;
            let Some(paint) = self.get_or_build(icon_kind, fg, bg, preserve_original_style) else {
                return;
            };
            let (bx, by, bw, bh) = paint.bounds();
            if !(avail_w > 0.0 && avail_h > 0.0) {
                return;
            }
            let fit_inset = ui_styling::metrics::icon::FIT_INSET;
            let sx_half = avail_w * fit_inset * 0.5;
            let sy_half = avail_h * fit_inset * 0.5;
            let cx = bx + bw * 0.5;
            let cy = by + bh * 0.5;
            let scale = (2.0 * sx_half / bw).min(2.0 * sy_half / bh);
            let aff = Affine::IDENTITY.translate((center.x - scale * cx, center.y - scale * cy)) * Affine::IDENTITY.scale(scale);
            let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
            let hw = avail_w * clip_inset * 0.5;
            let hh = avail_h * clip_inset * 0.5;
            let clip_r = Rect::from_points(Point::new(center.x - hw, center.y - hh), Point::new(center.x + hw, center.y + hh));
            scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
            match paint.body() {
                CachedIconBody::Vector(icon_scene) => {
                    scene.append(icon_scene, Some(aff));
                }
                CachedIconBody::Raster(img) => {
                    scene.draw_image(img, aff);
                }
            }
            scene.pop_layer();
        }

        /// 🎨️ Themed SVG icon fg/bg from centralized canvas tokens (not node chrome stroke/fill).
        pub fn board_icon_paint_colors(canvas_theme: &CanvasPalette) -> (Color, Color) {
            let rgba = canvas_theme.raster_clear.to_rgba8();
            let lum = f64::from(rgba.r) * 0.299 + f64::from(rgba.g) * 0.587 + f64::from(rgba.b) * 0.114;
            let canvas = if lum < 128.0 { &ui_styling::CANVAS_DARK } else { &ui_styling::CANVAS_LIGHT };
            (Color::new(canvas.icon_fg), Color::new(canvas.icon_bg))
        }
    }
    // #endregion 🔖️Icons

    impl Default for CanvasPalette {
        fn default() -> Self {
            Self::from_board_palette(&ui_styling::BOARD_LIGHT)
        }
    }
    // #endregion types
}

pub use crate::infinite::board::ports::*;
pub use crate::infinite::board::{
    area_preselect_ids, merge_ids_into_selection, merge_pick_into_selection, normalize_selection_mode, pick_merge_mode_for_modifiers, region_bounds, region_grip_at, region_grip_drag, rotate_point_about, selection_contains_edge_curve,
    selection_contains_handle_point, selection_contains_node_bounds, selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, selection_screen_overlay_points, snap_region_scalar, snap_transform_angle,
    transform_pivot_of, transform_ring_angle_delta, transform_ring_hit, transform_ring_radius_world, RegionData, RegionGrip, TransformGumballFlags, REGION_GRIP_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD,
    SELECTION_CLICK_MAX_DISTANCE_PX, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX, TRANSFORM_RING_HIT_TOLERANCE_PX, TRANSFORM_ROTATE_SNAP_RADIANS,
};
pub use crate::infinite::canvas;
pub use scene_json::{board_json_visible_option, board_json_visible_or_true, fixture_edge_handle_ids_from_object, normalize_board_descriptor_hidden_to_visible, EdgeDescJson, FixtureJson, RegionDescJson, SceneDescriptorJson, WireDescJson};
pub use types::*;

/// ➡️ Port graph engine with directed handle endpoints.
pub type DirectedPortGraphEngine = GraphEngine<Ported, Directed>;

/// ⚙️ Puzzle 2d board engine alias.
pub type BoardEngine = DirectedPortGraphEngine;

/// 🪢️ Cubic edge connecting two handles (legacy field names).
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub id: EdgeId,
    pub source_handle: HandleId,
    pub target_handle: HandleId,
}

// #region 🔖️EdgeEndpointResolution
/// 🔗️ Resolves a ported edge endpoint to a node id (handle lookup, then node id).
fn resolve_endpoint_node_id(endpoint_id: &str, handle_to_node: &std::collections::HashMap<String, String>) -> String {
    handle_to_node.get(endpoint_id).cloned().unwrap_or_else(|| endpoint_id.to_string())
}
// #endregion 🔖️EdgeEndpointResolution

// #region 🕸️ForceGraphLayout
pub mod force_graph {
    use serde_json::Value;
    use std::collections::HashMap;

    use super::board_json_visible_or_true;
    pub use crate::infinite::board::normal::undirected::ForceGraphLayoutOptions;

    fn build_handle_to_node(nodes: &[Value]) -> HashMap<String, String> {
        let mut handle_to_node: HashMap<String, String> = HashMap::new();
        for node in nodes {
            let Some(obj) = node.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(obj) {
                continue;
            };
            let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) else {
                continue;
            };
            for h in handles {
                let Some(ho) = h.as_object() else {
                    continue;
                };
                if !board_json_visible_or_true(ho) {
                    continue;
                };
                if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
                    handle_to_node.insert(hid.to_string(), nid.to_string());
                }
            }
        }
        handle_to_node
    }

    /// 🕸️ Ported force layout: resolves handle endpoints, then delegates to normal undirected physics.
    pub fn apply_force_graph_layout_to_fixture_v1_value(fixture: &mut Value, opts: &ForceGraphLayoutOptions) -> Result<(), String> {
        let nodes = fixture.as_object().and_then(|root| root.get("nodes")).and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let handle_to_node = build_handle_to_node(&nodes);
        infinite::board::normal::undirected::apply_force_graph_layout_to_fixture_v1_value_resolved(fixture, opts, |endpoint, id_to_index| {
            let node_id = handle_to_node.get(endpoint).cloned().unwrap_or_else(|| endpoint.to_string());
            id_to_index.contains_key(&node_id).then_some(node_id)
        })
        .map_err(|e| e.to_string())
    }

    /// 🕸️ JSON entry for ported force layout (handle endpoints resolved before undirected physics).
    pub fn apply_force_graph_layout_to_fixture_v1_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
        let mut fixture: Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
        let opts: ForceGraphLayoutOptions = if options_json.trim().is_empty() { ForceGraphLayoutOptions::default() } else { serde_json::from_str(options_json).map_err(|e| e.to_string())? };
        apply_force_graph_layout_to_fixture_v1_value(&mut fixture, &opts)?;
        serde_json::to_string(&fixture).map_err(|e| e.to_string())
    }
}
// #endregion 🕸️ForceGraphLayout

// #region 🌳️HierarchicalTreeLayout
pub mod hierarchical_tree {
    use serde::Deserialize;
    use serde_json::Value;
    use std::collections::{HashMap, HashSet};

    use super::board_json_visible_or_true;
    use super::fixture_edge_handle_ids_from_object;

    /// 🌳️ Buchheim tidy-tree knobs: rank gap, sibling breadth, growth-axis string, optional world anchor for the laid subtree.
    #[derive(Clone, Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct HierarchicalTreeLayoutOptions {
        #[serde(default = "default_layer_spacing")]
        #[value(default = "default_layer_spacing")]
        pub layer_spacing: f64,
        #[serde(default = "default_sibling_gap")]
        #[value(default = "default_sibling_gap")]
        pub sibling_gap: f64,
        #[serde(default = "default_direction")]
        #[value(default = "default_direction")]
        pub direction: String,
        #[serde(default)]
        #[value(default)]
        pub center_x: Option<f64>,
        #[serde(default)]
        #[value(default)]
        pub center_y: Option<f64>,
        /// 📌️ Node ids whose incoming fixture centers are kept; Buchheim still runs for placement of unlocked nodes.
        #[serde(default)]
        #[value(default)]
        pub locked_node_ids: Vec<String>,
    }

    fn default_layer_spacing() -> f64 {
        120.0
    }
    fn default_sibling_gap() -> f64 {
        28.0
    }
    fn default_direction() -> String {
        "downwards".into()
    }

    impl Default for HierarchicalTreeLayoutOptions {
        fn default() -> Self {
            Self { layer_spacing: default_layer_spacing(), sibling_gap: default_sibling_gap(), direction: default_direction(), center_x: None, center_y: None, locked_node_ids: Vec::new() }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum TreeDirection {
        Downwards,
        Upwards,
        Right,
        Left,
    }

    impl TreeDirection {
        fn parse(s: &str) -> Result<Self, String> {
            match s.trim().to_ascii_lowercase().as_str() {
                "down" | "downwards" => Ok(Self::Downwards),
                "up" | "upwards" => Ok(Self::Upwards),
                "right" => Ok(Self::Right),
                "left" => Ok(Self::Left),
                _ => Err(format!("unknown hierarchical tree direction: {s}")),
            }
        }
    }

    fn half_extent(node: &Value) -> f64 {
        let Some(obj) = node.as_object() else {
            return ui_styling::radii::NODE_DEFAULT;
        };
        if obj.get("shape").and_then(|v| v.as_str()) == Some("rectangle") {
            let w = obj.get("width").and_then(|v| v.as_f64()).unwrap_or(40.0);
            let h = obj.get("height").and_then(|v| v.as_f64()).unwrap_or(40.0);
            return (w.max(h) * 0.5).max(8.0);
        }
        obj.get("radius").and_then(|v| v.as_f64()).filter(|r| r.is_finite() && *r > 0.0).unwrap_or(ui_styling::radii::NODE_DEFAULT)
    }

    const TREE_SUPER_ID: &str = "__tree_super__";

    /** 🌲️ Buchheim et al. (GD 2002) tidy tree: O(n) Reingold–Tilford with even sibling spacing (after pymag-trees listing 12). */
    #[derive(Debug)]
    struct BuchheimNode {
        id: String,
        parent: Option<usize>,
        children: Vec<usize>,
        x: f64,
        y: f64,
        mod_: f64,
        thread: Option<usize>,
        ancestor: usize,
        change: f64,
        shift: f64,
        number: i32,
        synthetic: bool,
    }

    fn buchheim_left_brother(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        let p = nodes[i].parent?;
        let ch = &nodes[p].children;
        let pos = ch.iter().position(|&c| c == i)?;
        if pos == 0 {
            return None;
        }
        Some(ch[pos - 1])
    }

    fn buchheim_leftmost_sibling(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        let p = nodes[i].parent?;
        let ch = &nodes[p].children;
        if ch.first() == Some(&i) {
            return None;
        }
        ch.first().copied()
    }

    fn buchheim_next_right(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        if let Some(t) = nodes[i].thread {
            return Some(t);
        }
        nodes[i].children.last().copied()
    }

    fn buchheim_next_left(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        if let Some(t) = nodes[i].thread {
            return Some(t);
        }
        nodes[i].children.first().copied()
    }

    fn buchheim_ancestor(nodes: &[BuchheimNode], vil: usize, v: usize, default_ancestor: usize) -> usize {
        let par = nodes[v].parent.expect("buchheim ancestor needs parent");
        let pa = nodes[vil].ancestor;
        if nodes[par].children.contains(&pa) {
            pa
        } else {
            default_ancestor
        }
    }

    fn buchheim_move_subtree(nodes: &mut [BuchheimNode], wl: usize, wr: usize, shift: f64) {
        let subtrees = (nodes[wr].number - nodes[wl].number) as f64;
        if subtrees <= 0.0 {
            return;
        }
        nodes[wr].change -= shift / subtrees;
        nodes[wr].shift += shift;
        nodes[wl].change += shift / subtrees;
        nodes[wr].x += shift;
        nodes[wr].mod_ += shift;
    }

    fn buchheim_execute_shifts(nodes: &mut [BuchheimNode], v: usize) {
        let mut shift = 0.0f64;
        let mut change = 0.0f64;
        for &w in nodes[v].children.iter().rev() {
            nodes[w].x += shift;
            nodes[w].mod_ += shift;
            change += nodes[w].change;
            shift += nodes[w].shift + change;
        }
    }

    fn buchheim_apportion(nodes: &mut [BuchheimNode], v: usize, default_ancestor: usize, distance: f64) -> usize {
        let mut default_ancestor = default_ancestor;
        let w = match buchheim_left_brother(nodes, v) {
            Some(w) => w,
            None => return default_ancestor,
        };
        let mut vir = v;
        let mut vor = v;
        let mut vil = w;
        let mut vol = match buchheim_leftmost_sibling(nodes, v) {
            Some(s) => s,
            None => return default_ancestor,
        };
        let mut sir = nodes[v].mod_;
        let mut sor = nodes[v].mod_;
        let mut sil = nodes[vil].mod_;
        let mut sol = nodes[vol].mod_;
        loop {
            let vil_r = buchheim_next_right(nodes, vil);
            let vir_l = buchheim_next_left(nodes, vir);
            if vil_r.is_none() || vir_l.is_none() {
                break;
            }
            vil = vil_r.unwrap();
            vir = vir_l.unwrap();
            let vol_l = buchheim_next_left(nodes, vol);
            let vor_r = buchheim_next_right(nodes, vor);
            if vol_l.is_none() || vor_r.is_none() {
                break;
            }
            vol = vol_l.unwrap();
            vor = vor_r.unwrap();
            nodes[vor].ancestor = v;
            let shift = (nodes[vil].x + sil) - (nodes[vir].x + sir) + distance;
            if shift > 0.0 {
                let a = buchheim_ancestor(nodes, vil, v, default_ancestor);
                buchheim_move_subtree(nodes, a, v, shift);
                sir += shift;
                sor += shift;
            }
            sil += nodes[vil].mod_;
            sir += nodes[vir].mod_;
            sol += nodes[vol].mod_;
            sor += nodes[vor].mod_;
        }
        if let Some(vil_r) = buchheim_next_right(nodes, vil) {
            if buchheim_next_right(nodes, vor).is_none() {
                nodes[vor].thread = Some(vil_r);
                nodes[vor].mod_ += sil - sor;
            }
        } else if buchheim_next_left(nodes, vir).is_some() && buchheim_next_left(nodes, vol).is_none() {
            if let Some(vir_l) = buchheim_next_left(nodes, vir) {
                nodes[vol].thread = Some(vir_l);
                nodes[vol].mod_ += sir - sol;
            }
            default_ancestor = v;
        }
        default_ancestor
    }

    fn buchheim_first_walk(nodes: &mut [BuchheimNode], v: usize, distance: f64) -> usize {
        if nodes[v].children.is_empty() {
            if buchheim_leftmost_sibling(nodes, v).is_some() {
                let lb = buchheim_left_brother(nodes, v).expect("leaf with leftmost sibling has left brother");
                nodes[v].x = nodes[lb].x + distance;
            } else {
                nodes[v].x = 0.0;
            }
            return v;
        }
        let mut default_ancestor = nodes[v].children[0];
        for &w in &nodes[v].children.clone() {
            buchheim_first_walk(nodes, w, distance);
            default_ancestor = buchheim_apportion(nodes, w, default_ancestor, distance);
        }
        buchheim_execute_shifts(nodes, v);
        let c0 = nodes[v].children[0];
        let c1 = *nodes[v].children.last().expect("internal node has children");
        let mid = (nodes[c0].x + nodes[c1].x) * 0.5;
        if let Some(w) = buchheim_left_brother(nodes, v) {
            nodes[v].x = nodes[w].x + distance;
            nodes[v].mod_ = nodes[v].x - mid;
        } else {
            nodes[v].x = mid;
        }
        v
    }

    fn buchheim_second_walk(nodes: &mut [BuchheimNode], v: usize, m: f64, depth: i32, min_x: f64) -> f64 {
        nodes[v].x += m;
        nodes[v].y = depth as f64;
        let mut min_x = min_x.min(nodes[v].x);
        for &w in &nodes[v].children.clone() {
            min_x = buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x);
        }
        min_x
    }

    fn buchheim_third_walk(nodes: &mut [BuchheimNode], v: usize, n: f64) {
        nodes[v].x += n;
        for &c in &nodes[v].children.clone() {
            buchheim_third_walk(nodes, c, n);
        }
    }

    fn run_buchheim_layout(id_to_node: &HashMap<String, Value>, roots: &[String], directed: &[(String, String)], depth: &HashMap<String, i32>) -> Result<HashMap<String, (f64, f64)>, String> {
        let roots_set: HashSet<String> = roots.iter().cloned().collect();
        let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
        for (u, v) in directed {
            incoming.entry(v.clone()).or_default().push(u.clone());
        }
        for v in incoming.values_mut() {
            v.sort();
            v.dedup();
        }
        let mut chosen_parent: HashMap<String, String> = HashMap::new();
        for id in id_to_node.keys() {
            if roots_set.contains(id) {
                continue;
            }
            let ps = incoming.get(id).cloned().unwrap_or_default();
            if ps.is_empty() {
                continue;
            }
            let best = ps
                .iter()
                .min_by_key(|p| {
                    let dp = depth.get(*p).copied().unwrap_or(0);
                    (dp, (*p).clone())
                })
                .expect("non-empty ps")
                .clone();
            chosen_parent.insert(id.clone(), best);
        }
        let mut ordered_ids: Vec<String> = id_to_node.keys().cloned().collect();
        ordered_ids.sort();
        let id_to_idx: HashMap<String, usize> = ordered_ids.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
        let super_idx = ordered_ids.len();
        let mut nodes: Vec<BuchheimNode> =
            ordered_ids.iter().map(|id| BuchheimNode { ancestor: 0, change: 0.0, children: vec![], id: id.clone(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: false, thread: None, x: -1.0, y: 0.0 }).collect();
        nodes.push(BuchheimNode { ancestor: super_idx, change: 0.0, children: vec![], id: TREE_SUPER_ID.to_string(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: true, thread: None, x: -1.0, y: 0.0 });
        for (i, oid) in ordered_ids.iter().enumerate() {
            let pidx = if roots_set.contains(oid) {
                super_idx
            } else {
                match chosen_parent.get(oid) {
                    Some(p) => *id_to_idx.get(p).ok_or_else(|| format!("missing parent index for {p}"))?,
                    None => super_idx,
                }
            };
            nodes[i].parent = Some(pidx);
        }
        for node in &mut nodes {
            node.children.clear();
        }
        for i in 0..super_idx {
            let pi = nodes[i].parent.ok_or_else(|| "tree node missing parent".to_string())?;
            nodes[pi].children.push(i);
        }
        for p in 0..=super_idx {
            let mut ch: Vec<usize> = nodes[p].children.clone();
            ch.sort_by_key(|&c| nodes[c].id.clone());
            nodes[p].children = ch;
        }
        for p in 0..=super_idx {
            if nodes[p].children.is_empty() {
                continue;
            }
            let ch = nodes[p].children.clone();
            for (k, &c) in ch.iter().enumerate() {
                nodes[c].number = (k + 1) as i32;
                nodes[c].ancestor = c;
            }
        }
        let dist = 1.0f64;
        buchheim_first_walk(&mut nodes, super_idx, dist);
        let min_x = buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY);
        if min_x.is_finite() && min_x < 0.0 {
            buchheim_third_walk(&mut nodes, super_idx, -min_x);
        }
        let mut out: HashMap<String, (f64, f64)> = HashMap::new();
        for (i, n) in nodes.iter().enumerate() {
            if i == super_idx || n.synthetic {
                continue;
            }
            out.insert(n.id.clone(), (n.x, n.y));
        }
        Ok(out)
    }

    /// 🌳️ Writes node centers: Buchheim tidy-tree on a spanning forest (min-depth parent tie-break id), synthetic multi-root; super-root not serialized.
    pub fn apply_hierarchical_tree_layout_to_fixture_v1_value(fixture: &mut Value, opts: &HierarchicalTreeLayoutOptions) -> Result<(), String> {
        let dir = TreeDirection::parse(&opts.direction)?;
        let Some(root) = fixture.as_object_mut() else {
            return Err("fixture root must be object".into());
        };
        if root.get("schema").and_then(|v| v.as_str()) != Some("puzzle.2d.fixture") {
            return Err("schema must be puzzle.2d.fixture".into());
        }
        let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
            return Err("nodes array missing".into());
        };
        if nodes.is_empty() {
            return Ok(());
        }
        let mut handle_to_node: HashMap<String, String> = HashMap::new();
        let mut id_to_node: HashMap<String, Value> = HashMap::new();
        for node in nodes.iter() {
            let Some(obj) = node.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(obj) {
                continue;
            }
            let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            id_to_node.insert(nid.to_string(), node.clone());
            let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) else {
                continue;
            };
            for h in handles {
                let Some(ho) = h.as_object() else {
                    continue;
                };
                if !board_json_visible_or_true(ho) {
                    continue;
                }
                if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
                    handle_to_node.insert(hid.to_string(), nid.to_string());
                }
            }
        }
        if id_to_node.is_empty() {
            return Ok(());
        }
        let mut directed: Vec<(String, String)> = Vec::new();
        let mut seen_dir: HashSet<(String, String)> = HashSet::new();
        for e in &edges_json {
            let Some(eo) = e.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(eo) {
                continue;
            }
            let Some((src_h, tgt_h)) = fixture_edge_handle_ids_from_object(eo) else {
                continue;
            };
            let source_node_id = super::resolve_endpoint_node_id(src_h, &handle_to_node);
            let target_node_id = super::resolve_endpoint_node_id(tgt_h, &handle_to_node);
            if source_node_id == target_node_id {
                continue;
            }
            if !id_to_node.contains_key(&source_node_id) || !id_to_node.contains_key(&target_node_id) {
                continue;
            }
            if seen_dir.insert((source_node_id.clone(), target_node_id.clone())) {
                directed.push((source_node_id, target_node_id));
            }
        }
        let mut incoming_edge_count_by_node: HashMap<String, u32> = HashMap::new();
        for id in id_to_node.keys() {
            incoming_edge_count_by_node.insert(id.clone(), 0);
        }
        for (_source_nid, target_nid) in &directed {
            *incoming_edge_count_by_node.entry(target_nid.clone()).or_insert(0) += 1;
        }
        let mut roots: Vec<String> = Vec::new();
        for node in nodes.iter() {
            let Some(obj) = node.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(obj) {
                continue;
            }
            if obj.get("root").and_then(|v| v.as_bool()) == Some(true) {
                if let Some(nid) = obj.get("id").and_then(|v| v.as_str()) {
                    roots.push(nid.to_string());
                }
            }
        }
        roots.sort();
        roots.dedup();
        if roots.is_empty() {
            for (id, &d) in &incoming_edge_count_by_node {
                if d == 0 {
                    roots.push(id.clone());
                }
            }
            roots.sort();
        }
        if roots.is_empty() {
            roots = id_to_node.keys().cloned().collect();
            roots.sort();
        }
        let mut depth: HashMap<String, i32> = HashMap::new();
        for r in &roots {
            depth.insert(r.clone(), 0);
        }
        let cap = directed.len().saturating_mul(3).saturating_add(nodes.len()).saturating_add(8);
        for _ in 0..cap {
            let mut changed = false;
            for (source_nid, target_nid) in &directed {
                let Some(&dp) = depth.get(source_nid) else {
                    continue;
                };
                let nd = dp + 1;
                let cur = *depth.get(target_nid).unwrap_or(&-1);
                if nd > cur {
                    depth.insert(target_nid.clone(), nd);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let max_depth = depth.values().copied().max().unwrap_or(0);
        for id in id_to_node.keys() {
            depth.entry(id.clone()).or_insert(max_depth + 1);
        }
        let raw = run_buchheim_layout(&id_to_node, &roots, &directed, &depth)?;
        let mean_half: f64 = id_to_node.values().map(half_extent).sum::<f64>() / id_to_node.len().max(1) as f64;
        let along_scale = (opts.sibling_gap + 2.0 * mean_half).max(8.0);
        let mut pos: HashMap<String, (f64, f64)> = HashMap::new();
        for (id, (bx, by)) in raw {
            let along = bx * along_scale;
            let orth = by * opts.layer_spacing;
            let (lx, ly) = match dir {
                TreeDirection::Downwards => (along, orth),
                TreeDirection::Upwards => (along, -orth),
                TreeDirection::Right => (orth, along),
                TreeDirection::Left => (-orth, along),
            };
            pos.insert(id, (lx, ly));
        }
        let mut minx = f64::INFINITY;
        let mut maxx = f64::NEG_INFINITY;
        let mut miny = f64::INFINITY;
        let mut maxy = f64::NEG_INFINITY;
        for (id, (x, y)) in &pos {
            let h = half_extent(id_to_node.get(id).unwrap());
            minx = minx.min(x - h);
            maxx = maxx.max(x + h);
            miny = miny.min(y - h);
            maxy = maxy.max(y + h);
        }
        if !minx.is_finite() {
            minx = 0.0;
            maxx = 1.0;
            miny = 0.0;
            maxy = 1.0;
        }
        let cx = (minx + maxx) * 0.5;
        let cy = (miny + maxy) * 0.5;
        let gx = opts.center_x.unwrap_or(0.0);
        let gy = opts.center_y.unwrap_or(0.0);
        let dx = gx - cx;
        let dy = gy - cy;
        let locked_set: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
        let mut pinned_world: HashMap<String, (f64, f64)> = HashMap::new();
        if !locked_set.is_empty() {
            for node in nodes.iter() {
                let Some(obj) = node.as_object() else {
                    continue;
                };
                if !board_json_visible_or_true(obj) {
                    continue;
                }
                let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
                    continue;
                };
                if !locked_set.contains(nid) {
                    continue;
                }
                if !id_to_node.contains_key(nid) {
                    continue;
                }
                let px = obj.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let py = obj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                pinned_world.insert(nid.to_string(), (px, py));
            }
        }
        for (id, (x, y)) in pos {
            let (fx, fy) = if let Some(&(px, py)) = pinned_world.get(&id) { (px, py) } else { (x + dx, y + dy) };
            let idx = nodes.iter().position(|n| n.get("id").and_then(|v| v.as_str()) == Some(id.as_str())).ok_or_else(|| format!("node index {id}"))?;
            let Some(obj) = nodes[idx].as_object_mut() else {
                continue;
            };
            obj.insert("x".into(), serde_json::json!(fx));
            obj.insert("y".into(), serde_json::json!(fy));
        }
        Ok(())
    }
}
// #endregion 🌳️HierarchicalTreeLayout

// #region 🔁️RedrawLayout
pub mod redraw_layout {
    use super::canvas::Point;
    use serde::Deserialize;
    use serde_json::Value;
    use std::collections::HashMap;

    use super::board_json_visible_or_true;
    use super::fixture_edge_handle_ids_from_object;
    use super::force_graph::{apply_force_graph_layout_to_fixture_v1_value, ForceGraphLayoutOptions};
    use super::hierarchical_tree::{apply_hierarchical_tree_layout_to_fixture_v1_value, HierarchicalTreeLayoutOptions};
    use super::{circle_handle_angle_toward, distance_between, rectangle_handle_angle_toward};

    #[derive(Debug, Clone, Copy)]
    enum NodeShapeSnap {
        Circle { cx: f64, cy: f64 },
        Rect { cx: f64, cy: f64, w: f64, h: f64 },
    }

    impl NodeShapeSnap {
        fn center(self) -> Point {
            match self {
                NodeShapeSnap::Circle { cx, cy, .. } | NodeShapeSnap::Rect { cx, cy, .. } => Point::new(cx, cy),
            }
        }

        fn handle_angle_toward(self, toward: Point) -> Option<f64> {
            let c = self.center();
            if distance_between(c, toward) <= 1e-9 {
                return None;
            }
            Some(match self {
                NodeShapeSnap::Circle { cx, cy, .. } => circle_handle_angle_toward(Point::new(cx, cy), toward),
                NodeShapeSnap::Rect { cx, cy, w, h } => rectangle_handle_angle_toward(Point::new(cx, cy), w, h, toward),
            })
        }
    }

    fn parse_node_shape_snap(node: &serde_json::Map<String, Value>) -> Option<NodeShapeSnap> {
        let cx = node.get("x").and_then(|v| v.as_f64())?;
        let cy = node.get("y").and_then(|v| v.as_f64())?;
        if node.get("shape").and_then(|v| v.as_str()) == Some("rectangle") {
            let w = node.get("width").and_then(|v| v.as_f64())?;
            let h = node.get("height").and_then(|v| v.as_f64())?;
            Some(NodeShapeSnap::Rect { cx, cy, w, h })
        } else {
            node.get("radius").and_then(|v| v.as_f64())?;
            Some(NodeShapeSnap::Circle { cx, cy })
        }
    }

    /// 🔗️ Sets each edge endpoint handle `angle` so the chord follows node centers; last edge wins on shared handles.
    pub fn apply_edge_handle_snap_to_fixture_v1_value(fixture: &mut Value) -> Result<(), String> {
        let Some(root) = fixture.as_object_mut() else {
            return Err("fixture root must be object".into());
        };
        if root.get("schema").and_then(|v| v.as_str()) != Some("puzzle.2d.fixture") {
            return Err("schema must be puzzle.2d.fixture".into());
        }
        let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
            return Err("nodes array missing".into());
        };
        let mut shapes: Vec<Option<NodeShapeSnap>> = Vec::with_capacity(nodes.len());
        let mut handle_loc: HashMap<String, (usize, usize)> = HashMap::new();
        for (ni, node_val) in nodes.iter().enumerate() {
            let Some(no) = node_val.as_object() else {
                shapes.push(None);
                continue;
            };
            if !board_json_visible_or_true(no) {
                shapes.push(None);
                continue;
            }
            shapes.push(parse_node_shape_snap(no));
            let Some(hs) = no.get("handles").and_then(|v| v.as_array()) else {
                continue;
            };
            for (hi, h) in hs.iter().enumerate() {
                let Some(ho) = h.as_object() else {
                    continue;
                };
                if !board_json_visible_or_true(ho) {
                    continue;
                }
                if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
                    handle_loc.insert(hid.to_string(), (ni, hi));
                }
            }
        }
        let mut angle_by_loc: HashMap<(usize, usize), f64> = HashMap::new();
        for e in &edges_json {
            let Some(eo) = e.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(eo) {
                continue;
            }
            let Some((src_h, tgt_h)) = fixture_edge_handle_ids_from_object(eo) else {
                continue;
            };
            let Some(&(ni_a, hi_a)) = handle_loc.get(src_h) else {
                continue;
            };
            let Some(&(ni_b, hi_b)) = handle_loc.get(tgt_h) else {
                continue;
            };
            let Some(sa) = shapes.get(ni_a).copied().flatten() else {
                continue;
            };
            let Some(sb) = shapes.get(ni_b).copied().flatten() else {
                continue;
            };
            if let Some(ang_a) = sa.handle_angle_toward(sb.center()) {
                angle_by_loc.insert((ni_a, hi_a), ang_a);
            }
            if let Some(ang_b) = sb.handle_angle_toward(sa.center()) {
                angle_by_loc.insert((ni_b, hi_b), ang_b);
            }
        }
        for ((ni, hi), ang) in angle_by_loc {
            let Some(node_val) = nodes.get_mut(ni) else {
                continue;
            };
            let Some(no) = node_val.as_object_mut() else {
                continue;
            };
            let Some(hs) = no.get_mut("handles").and_then(|v| v.as_array_mut()) else {
                continue;
            };
            let Some(h) = hs.get_mut(hi) else {
                continue;
            };
            let Some(ho) = h.as_object_mut() else {
                continue;
            };
            ho.insert("angle".into(), serde_json::json!(ang));
        }
        Ok(())
    }

    pub fn apply_edge_handle_snap_to_fixture_v1_json(fixture_json: &str) -> Result<String, String> {
        let mut fixture: Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
        apply_edge_handle_snap_to_fixture_v1_value(&mut fixture)?;
        serde_json::to_string(&fixture).map_err(|e| e.to_string())
    }

    #[derive(Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    struct RedrawFixtureOptions {
        mode: String,
        #[serde(default)]
        #[value(default)]
        center_x: Option<f64>,
        #[serde(default)]
        #[value(default)]
        center_y: Option<f64>,
        #[serde(default)]
        #[value(default)]
        random_seed: Option<u64>,
        #[serde(default)]
        #[value(default)]
        redraw_handles_after: bool,
        #[serde(default)]
        #[value(default)]
        locked_node_ids: Vec<String>,
        #[serde(default)]
        #[value(default)]
        force_graph: Option<ForceGraphLayoutOptions>,
        #[serde(default)]
        #[value(default)]
        hierarchical_tree: Option<HierarchicalTreeLayoutOptions>,
    }

    pub fn apply_redraw_layout_to_fixture_v1_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
        let opts: RedrawFixtureOptions = serde_json::from_str(options_json).map_err(|e| e.to_string())?;
        let mut fixture: Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
        match opts.mode.as_str() {
            "force-graph" => {
                let mut fo = opts.force_graph.clone().unwrap_or_default();
                if opts.center_x.is_some() {
                    fo.center_x = opts.center_x;
                }
                if opts.center_y.is_some() {
                    fo.center_y = opts.center_y;
                }
                if let Some(s) = opts.random_seed {
                    fo.random_seed = s;
                }
                for id in &opts.locked_node_ids {
                    if !fo.locked_node_ids.contains(id) {
                        fo.locked_node_ids.push(id.clone());
                    }
                }
                apply_force_graph_layout_to_fixture_v1_value(&mut fixture, &fo)?;
            }
            "hierarchical-tree" => {
                let mut hierarchical_opts = opts.hierarchical_tree.clone().unwrap_or_default();
                if opts.center_x.is_some() {
                    hierarchical_opts.center_x = opts.center_x;
                }
                if opts.center_y.is_some() {
                    hierarchical_opts.center_y = opts.center_y;
                }
                for id in &opts.locked_node_ids {
                    if !hierarchical_opts.locked_node_ids.contains(id) {
                        hierarchical_opts.locked_node_ids.push(id.clone());
                    }
                }
                apply_hierarchical_tree_layout_to_fixture_v1_value(&mut fixture, &hierarchical_opts)?;
            }
            other => return Err(format!("unknown redraw mode: {other}")),
        }
        if opts.redraw_handles_after {
            apply_edge_handle_snap_to_fixture_v1_value(&mut fixture)?;
        }
        serde_json::to_string(&fixture).map_err(|e| e.to_string())
    }
}
// #endregion 🔁️RedrawLayout

// #region 🔖️GraphExtension
/// 🧩️ Extension hook for domain-specific graph behavior.
pub trait GraphExtension: canvas::CanvasExtension {}

pub use force_graph::{apply_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value, ForceGraphLayoutOptions};
pub use redraw_layout::{apply_edge_handle_snap_to_fixture_v1_json, apply_redraw_layout_to_fixture_v1_json};
// #endregion 🔖️GraphExtension

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quadrant/🦀️.rs"]
mod quadrant_tests;
// #endregion 🔖️Tests

```

### Cargo.toml

SHA-256 `542604e8a827dcc38c2e0c24922fb46eaeafb3c1b11638aaa1d6dbfb18579d77`; 32897 bytes.

```
cargo-features = ["trim-paths"]

[workspace]
resolver = "2"
members = [
    "🧰️framework/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏪️time-travel/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎒️pack/⚠️error/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📏️intrinsic-size/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔄️machine/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🚪️io/🧬️schema/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/❓️quiz/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📝️todos/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🗣️languages/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧑️contributors/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧾️yaml/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🪪️identity/📦️packages/🦀️rust",
]
exclude = ["**/🏅️standards/**", "**/🔮️oracles/**", "**/👽️guest/**"]

[workspace.metadata.semio.repository]
schema-version = 1
member-manifests = ["🧰️framework/**/📦️packages/🦀️rust/Cargo.toml"]
owner-manifests = ["[!.]*/Cargo.toml"]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.95"

[workspace.dependencies]
semio-framework-schema-state = { path = "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust" }
semio-framework-schema-composition = { path = "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust" }
semio-framework-schema-validator = { path = "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust" }
semio-framework-plugin-host-fixture = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/📦️packages/🦀️rust" }
semio-framework-plugin-host = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust" }
semio-repo-test-host = { path = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust" }
semio-framework-os-renderer-wgpu = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust" }
semio-framework-artifact-infinite-dag = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust" }
semio-framework-artifact-flow-flow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust" }
semio-framework-artifact-playbook-playbook = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust" }
semio-framework-artifact-workflow-workflow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust" }
semio-framework-artifact-space-space = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust" }
semio-framework-artifact-space-collection = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust" }
semio-framework-artifact-workflow-run = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust" }
semio-framework-math = { path = "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust" }
semio-framework-number = { path = "🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust" }
semio-framework-geometry = { path = "🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust" }
semio-framework-raster = { path = "🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust" }
semio-framework-typeset = { path = "🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust" }
semio-framework-graph = { path = "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust" }
semio-framework-graph-layout-run = { path = "🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust" }
semio-framework-actor = { path = "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust" }
semio-framework-replication = { path = "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust" }
semio-framework-value = { path = "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust" }
semio-framework-value-resident = { path = "🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust" }
semio-framework-pack = { path = "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust" }
pack = { path = "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }
semio-framework-server = { path = "🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust" }
semio-framework-quiz = { path = "🧰️framework/🛍️products/❓️quiz/📦️packages/🦀️rust" }
semio-framework-async = { path = "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust" }
semio-framework-async-macros = { path = "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust" }
semio-framework-trace = { path = "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust" }
semio-framework-job = { path = "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust" }
semio-framework-tool-run = { path = "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust" }
semio-framework-time-travel = { path = "🧰️framework/🔨️modules/⏪️time-travel/📦️packages/🦀️rust" }
semio-framework-tool-machine = { path = "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust" }
semio-framework-dispatch-macros = { path = "🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust" }
semio-framework-hash = { path = "🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust" }
semio-framework-mesh-engine = { path = "🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust" }
semio-framework-schema = { path = "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust" }
semio-framework-schema-registry = { path = "🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust" }
schema = { path = "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust", package = "semio-framework-schema" }
semio-framework-value-derive = { path = "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust" }
semio-framework-os-services = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust" }
semio-framework-plugin-describe = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust" }
# 🌐️ Canonical versions for the highest-fanout external deps, chosen as the newest
# explicit requirement string already used somewhere in the 630 manifests (matches what
# Cargo.lock already resolves to, so adopting `.workspace = true` later is a no-op for
# resolution). Deps below the ~10-manifest survey bar are left alone (see ticket report).
serde = { version = "1.0.228", features = ["derive"] }
serde_json = { version = "1.0.149", features = ["raw_value"] }
wasm-bindgen = "0.2.106"
js-sys = "0.3.83"
tokio = { version = "1" }

# 🧭️ Internal path deps for crates that exist TODAY at their current location, surveyed
# by counting `path = "…"` dependency references across all Cargo.toml files (>5 other
# manifests). Not wired to any member yet — a later wave can adopt `.workspace = true` as
# a drop-in, or repoint ONE line here when a crate merges/moves instead of editing every
# consumer. `# N refs` is the survey count. Grouped: os-kernel/core, math, ui, plugin-internal.
# ---- core (24) ----
semio-framework = { path = "🧰️framework/📦️packages/🦀️rust" }  # 58 refs
semio-framework-os = { path = "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust" }
semio-framework-os-kernel = { path = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }
semio-s-kernel-flow-extension-brep = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
semio-framework-os-kernel-db-state = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
semio-framework-os-kernel-db-storage = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
semio-framework-os-kernel-db-wal = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
semio-framework-os-kernel-flow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
semio-framework-os-kernel-infinite-board-port-directed-dag = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }
semio-framework-os-kernel-infinite-canvas = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }
semio-framework-kernel-infinite-world = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }
semio-framework-os-infinite = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust" }
semio-s-kernel-flow-extension-wasm = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
semio-framework-os-flow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust" }
semio-framework-os-kernel-neural-engine = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust" }
semio-framework-os-kernel-db = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
# 🧭️ Keep alias on OLD impl until W8c plugin cut-over deletes the sandwich (packages path already exists on disk).
semio-framework-plugin = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust" }
semio-framework-os-config = { path = "🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust" }
semio-framework-os-shell = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust" }
semio-framework-os-mcp = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" }
# REMOVED missing: semio-s-kernel-flow-extension-wasm = { path = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }  # 10 refs

# ---- math (13) ----

# ---- ui (10) ----
semio-framework-ui-styling = { path = "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust" }  # 15 refs
semio-framework-ui-contract = { path = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust" }
semio-framework-ui-render = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust" }
semio-framework-ui-runtime = { path = "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust" }
semio-framework-ui-scene = { path = "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust" }
semio-framework-ui-viewport = { path = "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust" }
semio-framework-ui-backend-webgpu = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust" }
semio-framework-ui-backend-metal = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust" }
semio-framework-ui-backend-d3d12 = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust" }
semio-framework-ui-backend-vulkan = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust" }
semio-framework-ui-host = { path = "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust" }
semio-framework-ui = { path = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust" }
semio-framework-surface = { path = "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust" }

# ---- plugin (62) ----
semio-framework-2d = { path = "🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust" }
semio-framework-3d = { path = "🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust" }
[profile.dev]
debug = false
incremental = false

[profile.dev.build-override]
opt-level = 3

# ⚡️ The plugin runtime every native dev host embeds (the semio MCP gateway, the hub, native shells) is
# optimized even in the dev profile. At `opt-level = 0` the wasmtime component-model machinery around
# every guest crossing and host call, and cranelift compiling a 129 MB wasm-dev guest, dominated the
# guest's own work. Measured on the wfc genesis `inference_run` over the semio MCP: 112.7 s with this
# set unoptimized vs 63.9 s optimized (ticket 26/09/23, `📓️wp-g5.md`), and a first solve after a guest
# rebuild waited ~9 min for its unoptimized cranelift compile. Only these crates change, so a rebuild
# costs their own compile plus a relink of their dependents (1 min 44 s measured).
[profile.dev.package.wasmtime]
opt-level = 3

[profile.dev.package.wasmtime-environ]
opt-level = 3

[profile.dev.package.wasmtime-internal-core]
opt-level = 3

[profile.dev.package.wasmtime-internal-cranelift]
opt-level = 3

[profile.dev.package.wasmtime-internal-fiber]
opt-level = 3

[profile.dev.package.wasmtime-internal-unwinder]
opt-level = 3

[profile.dev.package.wasmtime-internal-cache]
opt-level = 3

[profile.dev.package.wasmtime-internal-component-util]
opt-level = 3

[profile.dev.package.wasmtime-internal-jit-debug]
opt-level = 3

[profile.dev.package.wasmtime-internal-jit-icache-coherence]
opt-level = 3

[profile.dev.package.wasmtime-wasi]
opt-level = 3

[profile.dev.package.wasmtime-wasi-io]
opt-level = 3

[profile.dev.package.cranelift-codegen]
opt-level = 3

[profile.dev.package.cranelift-frontend]
opt-level = 3

[profile.dev.package.cranelift-entity]
opt-level = 3

[profile.dev.package.cranelift-bforest]
opt-level = 3

[profile.dev.package.cranelift-bitset]
opt-level = 3

[profile.dev.package.cranelift-control]
opt-level = 3

[profile.dev.package.cranelift-native]
opt-level = 3

[profile.dev.package.cranelift-codegen-shared]
opt-level = 3

[profile.dev.package.cranelift-assembler-x64]
opt-level = 3

[profile.dev.package.regalloc2]
opt-level = 3

[profile.dev.package.wasmparser]
opt-level = 3

[profile.dev.package.pulley-interpreter]
opt-level = 3

[profile.dev.package.gimli]
opt-level = 3

[profile.dev.package.object]
opt-level = 3

[profile.dev.package.wit-parser]
opt-level = 3

# 🧮️ The owned wasm interpreter (`semio-framework-plugin-host::interpreter`) runs every trusted-catalog
# verification `codec.pack-schema-hash` on the hub's startup path. At `opt-level = 0` a debug hub spent
# >11 min of one core interpreting writer/draw/puzzle during catalog load and the candidate readiness
# wait gave up (ticket 26/09/23 W1 §4.6, sampled: 100 % in `CoreInstance::execute_machine`).
[profile.dev.package.semio-framework-plugin-host]
opt-level = 3

# 🔐️ The repository's own SHA-256 carries every hub credential check: a sign-in derives PBKDF2-HMAC-SHA256
# at 210 000 iterations. At `opt-level = 0` one derivation took ~2 s on an idle debug hub and 8.5 s on hub
# 7800 under load (ticket 26/09/23 H9 session 12), all inside the compression function; optimizing this
# one small crate makes a dev hub's sign-in cost what a release hub's does.
[profile.dev.package.semio-framework-hash]
opt-level = 3

# 🛡️ WASI component links alone select this mitigation for rust-lld's ElemSection crash.
# Native dev retains Cargo's parallel codegen policy; publication stays wasm-release.
[profile.wasm-dev]
inherits = "dev"
codegen-units = 1
# 🧾️ A component's described bytes must be its shipped bytes: `describe` and `component-dev` link the same
# `cargo rustc --crate-type cdylib` unit, and `incremental` is part of that unit's profile identity, so an
# inherited `incremental = true` split it by the caller's `CARGO_INCREMENTAL` into two compiles with two
# different wasm hashes (ticket 26/09/23 W1: descriptor `ce48…`/`2a5c…` vs staged `a740…`).
incremental = false

# 🎚️ The plugin guest must meet the framework's 8 ms interactive-step contract even in the dev
# profile: at `opt-level = 0` a lowpoly render turn tessellates its seeded mesh in 10-11.5 ms and
# the runtime traps it with `plugin.internal.interactive-ceiling`, so the window renders a fault
# instead of geometry. Scoped to this package rather than raised on the whole profile because the
# io layer (`semio-s-plugin-stdio`) is not on the render path and is far more expensive to compile.
[profile.wasm-dev.package.semio-framework-os-flow]
opt-level = 2

# 🎚️ `Evaluator::evaluate_channels_budgeted` — the topological dag walk itself, run once per hop.
[profile.wasm-dev.package.semio-framework-os-kernel-neural-engine]
opt-level = 2

# 🎚️ `DslValue`/`OrderedMap` — the per-neuron `Dictionary` every walk clones, merges and hashes.
[profile.wasm-dev.package.semio-framework-replication]
opt-level = 2

# 🎚️ `pack::json` — the codec that serializes each node's input/output payload on the same hop.
[profile.wasm-dev.package.semio-framework-pack]
opt-level = 2

# 🎚️ Answers `capability: evaluate`/`tessellate` for the BREP operators the example drives.
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = "debuginfo"
incremental = false
trim-paths = "object"

# 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT. wasm32-wasip2 plugin components only — os-dev's
# 📜️script.ts passes `--profile wasm-release` when building plugin crates. Inherits ship-oriented
# `[profile.release]` above, then overrides for wasm size/runtime (see below).
#
# - opt-level "s" (not "z"): "z" measurably slows hot numeric loops in geometry/FEM solvers for a
#   few extra percent of size; the wasm-opt -Oz post-pass (see buildPlugin/transpilePluginComponent)
#   gets the "z"-class shrink without paying that runtime cost in every plugin.
# - lto "thin" (not "fat"): fat LTO re-links ~25 plugin cdylibs individually — 2-4x slower per plugin
#   for a low single-digit percent size gain over thin. Thin still gets full cross-crate inlining.
# - codegen-units = 1: maximizes cross-crate dedup/inlining and also sidesteps the LLVM-22
#   ElemSection::writeBody crash noted above (a stable low CGU count, same fix as `store`'s override).
# - strip = "symbols": drops the wasm `name` custom section, which is pure debug/dev-tooling weight
#   (measured ~14MB on the largest single plugin, ~87MB across the built fleet) never used at runtime.
# - incremental = false: deterministic output; release units are rebuilt whole anyway, so incremental
#   session state would only cost disk in the shared build-dir (`.cargo/config.toml` `build.build-dir`).
# - trim-paths = "object": strips absolute build-host cargo-registry paths retained in panic
#   `Location` strings (panic = "unwind" is intentionally NOT overridden here — wasm32-wasip2's
#   target spec already defaults to panic-strategy "abort", so plugins already abort-on-panic and
#   `catch_unwind` already never catches on this target; setting it explicitly would be a no-op).
[profile.wasm-release]
inherits = "release"
opt-level = "s"
lto = "thin"
codegen-units = 1
strip = "symbols"
incremental = false
trim-paths = "object"

# Explicit: profile inheritance does NOT inherit package-specific overrides from the parent
# profile, so without this `store` would fall back to workspace defaults under `wasm-release`.
[profile.wasm-release.package.semio-framework-os-kernel]
codegen-units = 1

# 🧹️ RUST-WIDE-CLEAN-REFACTOR-CAMPAIGN baseline. Kept at "warn" (never "deny") in
# the manifest so live concurrent edits never hard-break; zero-warning is enforced
# at verification gates via `cargo clippy -- -D warnings`. NEVER set RUSTFLAGS to
# add -D warnings — it replaces (not merges) .cargo/config.toml's rustflags.
[workspace.lints.rust]
future_incompatible = { level = "warn", priority = -1 }
rust_2018_idioms = { level = "warn", priority = -1 }
unsafe_op_in_unsafe_fn = "warn"
unused_lifetimes = "warn"
unused_qualifications = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
cloned_instead_of_copied = "warn"
inefficient_to_string = "warn"
map_unwrap_or = "warn"
needless_pass_by_value = "warn"
semicolon_if_nothing_returned = "warn"
unnecessary_wraps = "warn"
redundant_clone = "warn"
# phase B (enable after the T2/T3 waves land): unwrap_used = "warn"

```
