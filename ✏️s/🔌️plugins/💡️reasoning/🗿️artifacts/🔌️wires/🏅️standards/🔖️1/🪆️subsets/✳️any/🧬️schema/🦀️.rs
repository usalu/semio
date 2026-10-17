//! 🧬️ Wires artifact schema — every field of the artifact with its state class.

use semio_framework_value::DslValue;
use framework_schema::ArtifactSchema;


//#region 🔖️Artifact
/// 🧬️ Shared wires artifact content.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresArtifact {
    #[state(artifact)]
    pub wires_snapshot: DslValue,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: crate::WiresContentChild,
    #[state(artifact)]
    pub meta: DslValue,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for WiresArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::empty_wires_snapshot())
    }
}

impl WiresArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::WiresSnapshot {
        crate::WiresSnapshot { wires_snapshot: self.wires_snapshot.clone(), content: self.content.clone(), meta: self.meta.clone() }
    }

    /// 🧬️ Builds the shared artifact from its document snapshot.
    pub fn from_snapshot(snapshot: crate::WiresSnapshot) -> Self {
        Self { wires_snapshot: snapshot.wires_snapshot, content: snapshot.content, meta: snapshot.meta }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::WiresSnapshot) {
        self.wires_snapshot = snapshot.wires_snapshot;
        self.content = snapshot.content;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.reasoning.wires` — twenty handcrafted schema leaves.
pub fn wires_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.reasoning.wires",
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
//#region 🏗️Construction
/// 🏗️ Hand-rolled `ArtifactBuilder` — the generic `semio_framework_plugin::app::SnapshotBuilder<S, M>`
/// (ticket `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`, `📓️w4-sequence-report.md`'s
/// zero-boilerplate replacement) does NOT fit here: its `ArtifactBuilder` impl requires `S: Default`,
/// and `WiresSnapshot` deliberately has none — `content` is a composed `ArtifactChild` that needs a
/// freshly-minted, content-addressed handle (`empty_wires_snapshot()`'s
/// `wires_content_child_with_owner`), not a blanket zero value. This is the same class of
/// "the generic doesn't fit, keep the hand-rolled type" finding as `📓️w4-sequence-report.md`
/// `## recipeGaps` #1. No current caller
/// exercises `ArtifactBuilder` for this subset (confirmed: zero references outside this module,
/// `derive_artifact_facets!`'s deleted generated wrapper, and the deleted `io_registry`) — kept as
/// real, correctly-typed SDK equipment matching the fan-out's established `Construction` convention,
/// not dead API (mirrors how `SurfaceDeclaration.mutation_roster` is kept unread, per debt tracked in
/// `📓️w1-c-report.md` openQuestion 3).

//#endregion 🏗️Construction

//#region 🔖️DocumentHelpers
/// 🧬️ Pure helpers over `DslValue`-shaped documents — dissolved from the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): every fn here is generic over document shape
/// (never `WiresSnapshot`, never an app type) so it has no home more specific than the artifact schema; the
/// composed board they read comes from `crate::wires_composed`.
pub fn array_mut<'a>(fixture: &'a mut DslValue, key: &str) -> &'a mut Vec<DslValue> {
    if !matches!(fixture, DslValue::Object(_)) {
        *fixture = DslValue::Object(vec![]);
    }
    let DslValue::Object(entries) = fixture else {
        unreachable!("fixture coerced to object above");
    };
    if let Some(idx) = entries.iter().position(|(entry_key, _)| entry_key == key) {
        let value = &mut entries[idx].1;
        if !matches!(value, DslValue::Array(_)) {
            *value = DslValue::Array(vec![]);
        }
        match value {
            DslValue::Array(items) => items,
            _ => unreachable!("array coerced above"),
        }
    } else {
        entries.push((key.to_string(), DslValue::Array(vec![])));
        match &mut entries.last_mut().expect("just pushed").1 {
            DslValue::Array(items) => items,
            _ => unreachable!("just pushed array"),
        }
    }
}

/// 🧬️ Sets one field on the board node `node_id` inside a composed `board` in place (a render-time preview, never an edit).
pub fn set_node_field(board: &mut DslValue, node_id: &str, key: &str, value: DslValue) {
    if let Some(DslValue::Object(entries)) = array_mut(board, "nodes").iter_mut().find(|node| entity_id(node, "id") == Some(node_id)) {
        match entries.iter_mut().find(|(entry_key, _)| entry_key.as_str() == key) {
            Some((_, slot)) => *slot = value,
            None => entries.push((key.to_string(), value)),
        }
    }
}

/// 🔎️ The board node `node_id` of a composed `board`.
pub fn board_node<'a>(board: &'a DslValue, node_id: &str) -> Option<&'a DslValue> {
    board_snapshot_nodes(board).iter().find(|node| entity_id(node, "id") == Some(node_id))
}

/// 🔎️ The board edge `edge_id` of a composed `board`.
pub fn board_edge<'a>(board: &'a DslValue, edge_id: &str) -> Option<&'a DslValue> {
    board_snapshot_edges(board).iter().find(|edge| entity_id(edge, "id") == Some(edge_id))
}

pub fn entity_id<'a>(entity: &'a DslValue, key: &str) -> Option<&'a str> {
    entity.get(key).and_then(|value| value.as_str())
}

/// 🔢️ `identityId`/`sourceIdentityId`/`targetIdentityId` (and similar numeric-id fields) read as a
/// whole `u64` regardless of whether the source JSON number is an integer or a float literal. Fixtures
/// round-tripped through the `.wires` DSL text arrive as exact JSON integers (`Number(1)`, see
/// `IdentityDsl`/`RelationshipDsl`'s plain `u64` fields), so this fallback stays for documents built or
/// patched outside that DSL path (e.g. hand-constructed `Value` fixtures), where nothing enforces the
/// integer representation.
pub fn dsl_id(value: Option<&DslValue>) -> Option<u64> {
    value.and_then(|value| value.as_f64().map(|float| float as u64))
}




pub fn board_snapshot_camera(fixture: &DslValue) -> (f64, f64, f64) {
    let camera = fixture.get("camera");
    (
        camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
    )
}

pub fn board_snapshot_nodes(fixture: &DslValue) -> &[DslValue] {
    fixture.get("nodes").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn board_snapshot_edges(fixture: &DslValue) -> &[DslValue] {
    fixture.get("edges").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn wires_identities(wires: &DslValue) -> &[DslValue] {
    wires.get("identities").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn wires_relationships(wires: &DslValue) -> &[DslValue] {
    wires.get("relationships").and_then(|value| value.as_array()).unwrap_or(&[])
}

/// 📐️ A JSON node's position, defaulting missing coordinates to the origin.
pub fn node_position(node: &DslValue) -> (f64, f64) {
    (node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0), node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0))
}

/// ⭕️ Radius a board node without an explicit one is drawn with — the value `addNode` mints.
pub const WIRES_DEFAULT_NODE_RADIUS: f64 = 24.0;

/// 📐️ A board node's drawn box `(x, y, width, height)`: its position is the box's top-left corner, its
/// extent the explicit `width`/`height` or else the circle's diameter.
pub fn node_box(node: &DslValue) -> (f64, f64, f64, f64) {
    let (x, y) = node_position(node);
    let diameter = 2.0 * node.get("radius").and_then(|value| value.as_f64()).unwrap_or(WIRES_DEFAULT_NODE_RADIUS);
    (x, y, node.get("width").and_then(|value| value.as_f64()).unwrap_or(diameter), node.get("height").and_then(|value| value.as_f64()).unwrap_or(diameter))
}

/// 🧾️ Keys the framework `Canvas2dScene` layer record gives its own meaning (`CanvasLayerRecord`). A board
/// field of the same name must not ride into a layer: a node's `text` STRING read as the record's
/// `text: {content, size}` routes every node to the scene-node painter, which then draws nothing
/// (measured live 2026-09-23 on the 05:13 activation: seven circle layers, zero arcs).
const CANVAS_LAYER_RESERVED_KEYS: &[&str] = &[
    "kind", "role", "utility", "name", "color", "selected", "width", "height", "x0", "y0", "x1", "y1", "dataUrl", "points", "seams", "base", "transform", "segments", "fill", "stroke", "opacity", "blendMode", "fillRule", "visible", "text", "image",
];

/// 🖼️ Projects the board into the framework `Canvas2dScene` layer contract (`kind`, box `x/y/width/
/// height`, `name`; lines as `x0/y0/x1/y1`) that the editor and viewer canvases both render.
///
/// 🐛️ Both canvases used to hand the host the RAW board records — circles carrying only a `radius`
/// and edges carrying only `source`/`target` ids. The host draws a box from `x/y/width/height` and a
/// line from `x0/y0/x1/y1`, so every node and edge of the reasoning-wires pane was skipped and the
/// canvas showed nothing but its grid (measured live 2026-09-23: seven parsed node layers, zero
/// shapes drawn). Every other board field rides along untouched, so hit ids and positions still read
/// exactly as the document stores them; the node's `text` becomes the record's `name`.
pub fn wires_canvas_layers(board: &DslValue, wires: &DslValue) -> Vec<DslValue> {
    let nodes = board_snapshot_nodes(board);
    let centre = |id: &str| nodes.iter().find(|node| entity_id(node, "id") == Some(id)).map(|node| {
        let (x, y, width, height) = node_box(node);
        (x + width * 0.5, y + height * 0.5)
    });
    let relationship_kind = |edge_id: &str| wires_relationships(wires).iter().find(|relationship| entity_id(relationship, "edgeId") == Some(edge_id)).and_then(|relationship| relationship.get("kind")).cloned();
    let line = |id: &str, source: (f64, f64), target: (f64, f64), name: Option<DslValue>| {
        let mut entries = vec![
            ("id".to_string(), DslValue::String(id.to_string())),
            ("kind".to_string(), DslValue::String("line".into())),
            ("x0".to_string(), DslValue::float(source.0)),
            ("y0".to_string(), DslValue::float(source.1)),
            ("x1".to_string(), DslValue::float(target.0)),
            ("y1".to_string(), DslValue::float(target.1)),
        ];
        entries.extend(name.map(|name| ("name".to_string(), name)));
        DslValue::Object(entries)
    };
    let mut layers: Vec<DslValue> = nodes
        .iter()
        .map(|node| {
            let (_, _, width, height) = node_box(node);
            let mut entries: Vec<(String, DslValue)> = match node {
                DslValue::Object(entries) => entries.iter().filter(|(key, _)| !CANVAS_LAYER_RESERVED_KEYS.contains(&key.as_str())).cloned().collect(),
                _ => Vec::new(),
            };
            let kind = if node.get("shape").and_then(|value| value.as_str()) == Some("circle") { "circle" } else { "rect" };
            entries.push(("kind".to_string(), DslValue::String(kind.into())));
            entries.push(("width".to_string(), DslValue::float(width)));
            entries.push(("height".to_string(), DslValue::float(height)));
            entries.extend(node.get("text").cloned().map(|text| ("name".to_string(), text)));
            DslValue::Object(entries)
        })
        .collect();
    for edge in board_snapshot_edges(board) {
        let (Some(id), Some(source), Some(target)) = (entity_id(edge, "id"), entity_id(edge, "source").and_then(|id| centre(id)), entity_id(edge, "target").and_then(|id| centre(id))) else { continue };
        layers.push(line(id, source, target, relationship_kind(id)));
    }
    let identity_node = |identity: Option<u64>| wires_identities(wires).iter().find(|row| dsl_id(row.get("identityId")) == identity && identity.is_some()).and_then(|row| entity_id(row, "nodeId")).and_then(|id| centre(id));
    for relationship in wires_relationships(wires) {
        let Some(edge_id) = entity_id(relationship, "edgeId").filter(|id| !id.is_empty()) else { continue };
        if board_snapshot_edges(board).iter().any(|edge| entity_id(edge, "id") == Some(edge_id)) {
            continue;
        }
        if let (Some(source), Some(target)) = (identity_node(dsl_id(relationship.get("sourceIdentityId"))), identity_node(dsl_id(relationship.get("targetIdentityId")))) {
            layers.push(line(edge_id, source, target, relationship.get("kind").cloned()));
        }
    }
    layers
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️ExampleFixture


//#endregion 🔖️ExampleFixture

//#region 🧩️ArtifactItems
pub mod artifact_items {
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

//#region 🔖️Constants
/// 🪪️ This artifact's coordinate (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract
/// §2.1) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so `👁️viewer` can
/// read it without ever importing through the sibling editor module. `artifact_kind` matches
/// `#[artifact_schema(id = "s.reasoning.wires")]` on `WiresArtifact`
/// (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`); `standard`/`subset` match this
/// file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.reasoning.wires@1/*#editor` / `s.reasoning.wires@1/*#viewer`.
pub const WIRES_DIALECT: Dialect = Dialect { artifact_kind: "s.reasoning.wires", standard: StandardId("1"), subset: SubsetId::ANY };
pub use crate::schema::mutations::WiresMutation;

pub use crate::schema::diff::WiresDiff;

pub const MINDMAP_WIRES_SCHEMA: &str = "reasoning.wires.identity.snapshot";
/// 🕸️ Mindmap's own board fixture schema — recognized by the neutral force-graph-layout crate
/// (`infinite_board_normal_undirected`) as an undirected graph, distinct from puzzle's directed
/// `board.ports.directed.v1` board.
pub const MINDMAP_BOARD_SCHEMA: &str = "board.normal.undirected.v1";
/// 🧩️ The composed-child slot the board lives in.
pub const WIRES_CONTENT_SLOT: &str = "content";
/// 🔗️ The edge property carrying the wires relationship an edge expresses: its `kind` and the identities of its endpoints.
pub const WIRES_RELATIONSHIP_PROPERTY: &str = "relationship";
//#endregion 🔖️Constants
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{GraphEdgeId, GraphNodeId, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA, SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot, WiresContentChild};
use crate::{MINDMAP_BOARD_SCHEMA, MINDMAP_WIRES_SCHEMA, WIRES_CONTENT_SLOT, WIRES_RELATIONSHIP_PROPERTY, WiresMutation};
use semio_framework_value::DslValue;
pub use crate::schema::snapshot::WiresSnapshot;

//#region 🔖️EmptyFixtures
/// 📭️ Empty `reasoning.wires.identity.snapshot` blob: the parent's identity layer, no identity yet.
pub fn empty_wires_identity_snapshot() -> DslValue {
    DslValue::object([("schema".into(), DslValue::String(MINDMAP_WIRES_SCHEMA.into())), ("identities".into(), DslValue::Array(vec![]))])
}

/// 📭️ `{x:0, y:0, zoom:1}`, the neutral viewport a composed board carries (each canvas window owns its camera).
pub fn empty_camera() -> DslValue {
    DslValue::object([("x".into(), DslValue::float(0.0)), ("y".into(), DslValue::float(0.0)), ("zoom".into(), DslValue::float(1.0))])
}

/// 📭️ The empty board content.
pub fn empty_wires_content() -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

/// 📭️ Fresh wires snapshot: no identity, the empty board child.
pub fn empty_wires_snapshot() -> WiresSnapshot {
    WiresSnapshot { wires_snapshot: empty_wires_identity_snapshot(), content: wires_content_handle(&empty_wires_content()), meta: DslValue::Null }
}
//#endregion 🔖️EmptyFixtures

//#region 🔖️ContentBridge
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
pub use semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation;
pub use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
pub use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;

/// 🧾️ Board-node keys the graph node carries natively; every other board key is one keyed node property.
pub const WIRES_NATIVE_NODE_KEYS: &[&str] = &["id", "nodeKind", "text", "x", "y", "width", "height"];
/// 🧾️ Board-edge keys the graph edge carries natively; every other board key is one keyed edge property.
pub const WIRES_NATIVE_EDGE_KEYS: &[&str] = &["id", "source", "target", "edgeKind"];

/// 🔤️ One board value as a typed graph value (numbers keep their lexeme, objects their key order).
pub fn semio_value_from_dsl(value: &DslValue) -> SemioValue {
    match value {
        DslValue::Null => SemioValue::Null,
        DslValue::Bool(value) => SemioValue::Bool { value: *value },
        DslValue::Number(semio_framework_value::Number::UInt(value)) => SemioValue::Int { lexeme: value.to_string() },
        DslValue::Number(semio_framework_value::Number::Int(value)) => SemioValue::Int { lexeme: value.to_string() },
        DslValue::Number(semio_framework_value::Number::Float(value)) => SemioValue::Float { lexeme: value.to_string() },
        DslValue::String(value) => SemioValue::Str { value: value.clone() },
        DslValue::Bytes(value) => SemioValue::Bytes { value: value.clone() },
        DslValue::Array(items) => SemioValue::List { items: items.iter().map(semio_value_from_dsl).collect() },
        DslValue::Object(entries) => SemioValue::Map { entries: entries.iter().map(|(key, value)| SemioValueEntry { key: key.clone(), value: semio_value_from_dsl(value) }).collect() },
    }
}

/// 🔤️ [`semio_value_from_dsl`]'s inverse on every value it produces; a graph value reference reads as null on the board.
pub fn dsl_from_semio_value(value: &SemioValue) -> DslValue {
    match value {
        SemioValue::Null | SemioValue::Ref { .. } => DslValue::Null,
        SemioValue::Bool { value } => DslValue::Bool(*value),
        SemioValue::Int { lexeme } => lexeme.parse::<u64>().map(|value| DslValue::Number(semio_framework_value::Number::UInt(value))).or_else(|_| lexeme.parse::<i64>().map(|value| DslValue::Number(semio_framework_value::Number::Int(value)))).unwrap_or_else(|_| DslValue::String(lexeme.clone())),
        SemioValue::Float { lexeme } => lexeme.parse::<f64>().map(DslValue::float).unwrap_or_else(|_| DslValue::String(lexeme.clone())),
        SemioValue::Str { value } => DslValue::String(value.clone()),
        SemioValue::Bytes { value } => DslValue::Bytes(value.clone()),
        SemioValue::List { items } => DslValue::Array(items.iter().map(dsl_from_semio_value).collect()),
        SemioValue::Map { entries } => DslValue::Object(entries.iter().map(|entry| (entry.key.clone(), dsl_from_semio_value(&entry.value))).collect()),
    }
}

/// 🧾️ The keyed properties of a board entity: every key it carries beyond `native`, ascending.
pub fn wires_properties(entity: &DslValue, native: &[&str]) -> Vec<SemioValueEntry> {
    let DslValue::Object(entries) = canonical_board_value(entity) else { return Vec::new() };
    entries.into_iter().filter(|(key, _)| !native.contains(&key.as_str())).map(|(key, value)| SemioValueEntry { key, value: semio_value_from_dsl(&value) }).collect()
}

/// 🌉️ One board node as the composed graph node: native identity, kind, label, position and extent, the rest as properties.
pub fn wires_graph_node(node: &DslValue) -> SemioGraphNode {
    let text = |key: &str| node.get(key).and_then(DslValue::as_str).unwrap_or("").to_string();
    let number = |key: &str| node.get(key).and_then(DslValue::as_f64).unwrap_or(0.0);
    SemioGraphNode {
        id: GraphNodeId::new(text("id")),
        kind: text("nodeKind"),
        label: text("text"),
        position: SemioPoint2 { x: number("x"), y: number("y") },
        width: number("width"),
        height: number("height"),
        ports: Vec::new(),
        properties: wires_properties(node, WIRES_NATIVE_NODE_KEYS),
    }
}

/// 🌉️ One board edge as the composed graph edge: native identity, endpoints and kind, the rest (its relationship) as properties.
pub fn wires_graph_edge(edge: &DslValue) -> SemioGraphEdge {
    let text = |key: &str| edge.get(key).and_then(DslValue::as_str).unwrap_or("").to_string();
    SemioGraphEdge {
        id: GraphEdgeId::new(text("id")),
        source: GraphNodeId::new(text("source")),
        target: GraphNodeId::new(text("target")),
        kind: text("edgeKind"),
        label: String::new(),
        source_port: None,
        target_port: None,
        properties: wires_properties(edge, WIRES_NATIVE_EDGE_KEYS),
    }
}

/// 🌉️ [`wires_graph_node`]'s inverse: the board node a graph node reads as, keys ascending (an empty kind or label and a zero
/// extent are absent keys).
pub fn wires_board_node(node: &SemioGraphNode) -> DslValue {
    let mut entries: Vec<(String, DslValue)> = vec![("id".into(), DslValue::String(node.id.value.clone())), ("x".into(), DslValue::float(node.position.x)), ("y".into(), DslValue::float(node.position.y))];
    entries.extend((!node.kind.is_empty()).then(|| ("nodeKind".to_string(), DslValue::String(node.kind.clone()))));
    entries.extend((!node.label.is_empty()).then(|| ("text".to_string(), DslValue::String(node.label.clone()))));
    entries.extend((node.width != 0.0).then(|| ("width".to_string(), DslValue::float(node.width))));
    entries.extend((node.height != 0.0).then(|| ("height".to_string(), DslValue::float(node.height))));
    entries.extend(node.properties.iter().map(|entry| (entry.key.clone(), dsl_from_semio_value(&entry.value))));
    canonical_board_value(&DslValue::Object(entries))
}

/// 🌉️ [`wires_graph_edge`]'s inverse: the board edge a graph edge reads as, keys ascending.
pub fn wires_board_edge(edge: &SemioGraphEdge) -> DslValue {
    let mut entries: Vec<(String, DslValue)> = vec![("id".into(), DslValue::String(edge.id.value.clone())), ("source".into(), DslValue::String(edge.source.value.clone())), ("target".into(), DslValue::String(edge.target.value.clone()))];
    entries.extend((!edge.kind.is_empty()).then(|| ("edgeKind".to_string(), DslValue::String(edge.kind.clone()))));
    entries.extend(edge.properties.iter().map(|entry| (entry.key.clone(), dsl_from_semio_value(&entry.value))));
    canonical_board_value(&DslValue::Object(entries))
}

/// 🌉️ The graph content of a board roster.
pub fn wires_content_snapshot(nodes: &[DslValue], edges: &[DslValue]) -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: nodes.iter().map(wires_graph_node).collect(), edges: edges.iter().map(wires_graph_edge).collect() }
}

/// 🕸️ The content-addressed handle of the child minted from `content`: equal content, equal `(child_id, target)`. Once
/// minted the id names that child's store for good; its content then moves only through child leaves.
pub fn wires_content_handle(content: &SemioGraphSnapshot) -> WiresContentChild {
    let child_id = store::content_id("wires-content", &<SemioGraphSnapshot as pack::snapshot::ArtifactPack>::encode_pack(content));
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "graph".into() };
    store::os_store::artifact_child::ArtifactChild::new(child_id.clone(), semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id, dialect })
}

/// 🪪️ The child id the demo's parent asset names for its bundled board.
pub const WIRES_DEMO_CONTENT_ID: &str = "metabolism-content";

/// 📚️ The board contents this artifact ships, by the child id a parent names them with: the empty board under its content
/// address and the demo's committed child asset under [`WIRES_DEMO_CONTENT_ID`] — the only contents a parent-only load can
/// compose its child from.
pub fn wires_bundled_contents() -> &'static [(String, SemioGraphSnapshot)] {
    static CONTENTS: std::sync::OnceLock<Vec<(String, SemioGraphSnapshot)>> = std::sync::OnceLock::new();
    CONTENTS
        .get_or_init(|| {
            let demo = <SemioGraphSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::demo::CONTENT_TEXT).expect("the demo's committed wires board parses");
            vec![(wires_content_handle(&empty_wires_content()).child_id, empty_wires_content()), (WIRES_DEMO_CONTENT_ID.to_string(), demo)]
        })
        .as_slice()
}

/// 🌱️ Mints the composed `content` child's pack for a load that ships the parent alone (genesis store, `setActiveExample`,
/// a parent-only archive): the bundled content the parent names. A child a saved document carries arrives as an archive
/// member instead.
pub fn genesis_wires_child_pack(snapshot: &WiresSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    if slot != WIRES_CONTENT_SLOT || child_id != snapshot.content.child_id {
        return None;
    }
    wires_bundled_contents().iter().find(|(id, _)| id == child_id).map(|(_, content)| <SemioGraphSnapshot as pack::snapshot::ArtifactPack>::encode_pack(content))
}
//#endregion 🔖️ContentBridge

//#region 🔖️Composed
/// 🔤 Rewrites one free-form board value into the DSL's CANONICAL form: object keys ascending at every nesting level, the
/// form the document-text printer emits, so a board value compares equal to its own printed line.
pub fn canonical_board_value(value: &DslValue) -> DslValue {
    match value {
        DslValue::Array(items) => DslValue::Array(items.iter().map(canonical_board_value).collect()),
        DslValue::Object(entries) => {
            let mut sorted: Vec<(String, DslValue)> = entries.iter().map(|(key, value)| (key.clone(), canonical_board_value(value))).collect();
            sorted.sort_by(|left, right| left.0.cmp(&right.0));
            DslValue::Object(sorted)
        }
        other => other.clone(),
    }
}

/// 🪆️ The wires document every reader works on (§20.15): `fixture` is the parent's identity layer with the relationships the
/// child's edges carry; `board` the legacy board shape (`schema`/`camera`/`nodes`/
/// `edges`/`meta`?/`wires`) the canvases, panels and layout read.
#[derive(Clone, Debug, PartialEq)]
pub struct WiresComposed {
    pub identity_snapshot: DslValue,
    pub board: DslValue,
}

/// 🪆️ Composes `snapshot` with its board content.
pub fn wires_composed(snapshot: &WiresSnapshot, content: &SemioGraphSnapshot) -> WiresComposed {
    let nodes: Vec<DslValue> = content.nodes.iter().map(wires_board_node).collect();
    let edges: Vec<DslValue> = content.edges.iter().map(wires_board_edge).collect();
    let relationships = edges
        .iter()
        .filter_map(|edge| {
            let DslValue::Object(fields) = edge.get(WIRES_RELATIONSHIP_PROPERTY)? else { return None };
            let mut row = fields.clone();
            row.push(("edgeId".into(), edge.get("id")?.clone()));
            Some(canonical_board_value(&DslValue::Object(row)))
        })
        .collect();
    let identities = crate::schema::wires_identities(&snapshot.wires_snapshot).to_vec();
    let fixture = DslValue::object([("schema".into(), DslValue::String(MINDMAP_WIRES_SCHEMA.into())), ("identities".into(), DslValue::Array(identities)), ("relationships".into(), DslValue::Array(relationships))]);
    let mut board: Vec<(String, DslValue)> = vec![("schema".into(), DslValue::String(MINDMAP_BOARD_SCHEMA.into())), ("camera".into(), empty_camera()), ("nodes".into(), DslValue::Array(nodes)), ("edges".into(), DslValue::Array(edges))];
    board.extend((!matches!(snapshot.meta, DslValue::Null)).then(|| ("meta".to_string(), snapshot.meta.clone())));
    board.push(("wires".into(), DslValue::Array(vec![])));
    WiresComposed { identity_snapshot: fixture, board: DslValue::Object(board) }
}

/// 🔖️ A composed read that found no exact board child: named and localized through `ReasoningWiresPlayApp::fault_notices`.
pub fn wires_content_fault(code: &'static str, message: String) -> semio_framework_plugin::Fault {
    semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}

/// 🧸️ The board content `snapshot` names, read through its live composed children (exact `s.stdio.semio@v1/graph` dialect).
pub fn wires_content<'a>(snapshot: &WiresSnapshot, children: &'a semio_framework_plugin::app::ChildContentView) -> Result<impl std::ops::Deref<Target = SemioGraphSnapshot> + 'a, semio_framework_plugin::Fault> {
    let child_id = &snapshot.content.child_id;
    let dialect = children.dialect(WIRES_CONTENT_SLOT, child_id).ok_or_else(|| wires_content_fault("wires.content.unavailable", format!("the wires board child \"{child_id}\" is not composed")))?;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "graph" {
        return Err(wires_content_fault("wires.content.dialect", format!("the wires board child \"{child_id}\" is {}@{}/{}, not s.stdio.semio@v1/graph", dialect.artifact_kind, dialect.standard, dialect.subset)));
    }
    children.typed_read::<SemioGraphSnapshot>(WIRES_CONTENT_SLOT, child_id)
}

/// 🪆️ [`wires_composed`] over the board content `children` holds for `snapshot`.
pub fn wires_composed_from_children(snapshot: &WiresSnapshot, children: &semio_framework_plugin::app::ChildContentView) -> Result<WiresComposed, semio_framework_plugin::Fault> {
    let content = wires_content(snapshot, children)?;
    Ok(wires_composed(snapshot, &*content))
}

/// 🌱️ Publishes graph leaves as ONE edit of the exact composed board child; no leaf is the empty emission.
pub fn wires_child_emit<C, D>(snapshot: &WiresSnapshot, leaves: Vec<SemioGraphMutation>) -> semio_framework_plugin::Emit<WiresMutation, C, D> {
    if leaves.is_empty() {
        return semio_framework_plugin::Emit::default();
    }
    semio_framework_plugin::Emit { child_preparations: std::collections::VecDeque::from([semio_framework_plugin::app::ChildEmitPreparation::of::<SemioGraphSnapshot, _>(WIRES_CONTENT_SLOT, &snapshot.content.child_id, leaves)]), ..semio_framework_plugin::Emit::default() }
}
//#endregion 🔖️Composed
}
//#endregion 🧩️ArtifactItems3
