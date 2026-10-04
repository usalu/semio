//! 🧠️ Wires artifact — the document entity this plugin's one app (🔌️wires) edits.
//!
//! The board (nodes and edges) lives ONLY in the composed `s.stdio.semio@v1/graph` child `content` (ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12, §20.15): every board edit is a child-lane graph leaf in that child's
//! store, the parent owns no leaf (`WiresMutation` is uninhabited), and every reader composes the parent's `wires_fixture`
//! (identities) with the child on read ([`wires_composed`]). A decoded, reloaded or remote parent therefore needs no
//! materialization step.


#[path = "🤖️generated/📇️registry/🦀️.rs"]
pub mod graph_manifest;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_wires_demo_tests;
extern crate semio_framework_schema as framework_schema;

use semio_framework_value::DslValue;
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

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

pub const MINDMAP_WIRES_SCHEMA: &str = "reasoning.wires.fixture";
/// 🕸️ Mindmap's own board fixture schema — recognized by the neutral force-graph-layout crate
/// (`infinite_board_normal_undirected`) as an undirected graph, distinct from puzzle's directed
/// `puzzle.2d.fixture` board.
pub const MINDMAP_BOARD_SCHEMA: &str = "reasoning.mindmap.fixture";
/// 🧩️ The composed-child slot the board lives in.
pub const WIRES_CONTENT_SLOT: &str = "content";
/// 🔗️ The edge property carrying the wires relationship an edge expresses: its `kind` and the identities of its endpoints.
pub const WIRES_RELATIONSHIP_PROPERTY: &str = "relationship";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 📸️ Persisted wires snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::schema::snapshot::WiresSnapshot;
pub use crate::schema::WiresArtifact;
//#endregion 🔖️Types

//#region 🔖️EmptyFixtures
/// 📭️ Empty `reasoning.wires.fixture` blob: the parent's identity layer, no identity yet.
pub fn empty_wires_fixture() -> DslValue {
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
    WiresSnapshot { wires_fixture: empty_wires_fixture(), content: wires_content_handle(&empty_wires_content()), meta: DslValue::Null }
}
//#endregion 🔖️EmptyFixtures

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio@v1/graph` board.
pub type WiresContentChild = store::ArtifactChild<SemioGraphSnapshot>;

use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
pub use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
pub use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
pub use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

/// 🧾️ Board-node keys the graph node carries natively; every other board key is one keyed node property.
const WIRES_NATIVE_NODE_KEYS: &[&str] = &["id", "nodeKind", "text", "x", "y", "width", "height"];
/// 🧾️ Board-edge keys the graph edge carries natively; every other board key is one keyed edge property.
const WIRES_NATIVE_EDGE_KEYS: &[&str] = &["id", "source", "target", "edgeKind"];

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
fn wires_properties(entity: &DslValue, native: &[&str]) -> Vec<SemioValueEntry> {
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
    let child_id = store::content_id("wires-content", &<SemioGraphSnapshot as store::ArtifactPack>::encode_pack(content));
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "graph".into() };
    store::ArtifactChild::new(child_id.clone(), store::os_io::ArtifactRef { artifact_id: child_id, dialect })
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
            let demo = <SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(examples::demo::CONTENT_TEXT).expect("the demo's committed wires board parses");
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
    wires_bundled_contents().iter().find(|(id, _)| id == child_id).map(|(_, content)| <SemioGraphSnapshot as store::ArtifactPack>::encode_pack(content))
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
    pub fixture: DslValue,
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
    let identities = schema::wires_identities(&snapshot.wires_fixture).to_vec();
    let fixture = DslValue::object([("schema".into(), DslValue::String(MINDMAP_WIRES_SCHEMA.into())), ("identities".into(), DslValue::Array(identities)), ("relationships".into(), DslValue::Array(relationships))]);
    let mut board: Vec<(String, DslValue)> = vec![("schema".into(), DslValue::String(MINDMAP_BOARD_SCHEMA.into())), ("camera".into(), empty_camera()), ("nodes".into(), DslValue::Array(nodes)), ("edges".into(), DslValue::Array(edges))];
    board.extend((!matches!(snapshot.meta, DslValue::Null)).then(|| ("meta".to_string(), snapshot.meta.clone())));
    board.push(("wires".into(), DslValue::Array(vec![])));
    WiresComposed { fixture, board: DslValue::Object(board) }
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
pub fn wires_child_emit<C, D>(snapshot: &WiresSnapshot, leaves: &[SemioGraphMutation]) -> semio_framework_plugin::Emit<WiresMutation, C, D> {
    if leaves.is_empty() {
        return semio_framework_plugin::Emit::default();
    }
    semio_framework_plugin::Emit { child_emits: vec![semio_framework_plugin::app::ChildEmit::of::<SemioGraphSnapshot, _>(WIRES_CONTENT_SLOT, &snapshot.content.child_id, leaves)], ..semio_framework_plugin::Emit::default() }
}
//#endregion 🔖️Composed

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::wires::create_wires_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "graph.wires".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Wires Graph", "Leitungsgraph"),
        source_format: MINDMAP_WIRES_SCHEMA.into(),
        component_kind: "wires".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Dag },
        schema: MINDMAP_WIRES_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
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
    ArtifactDefinition::new(ArtifactIdentity::parse("s.reasoning.wires")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.reasoning.wires")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.reasoning.wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.reasoning.wires.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.reasoning.wires.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.reasoning.wires@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.reasoning.wires@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"reasoning.wires.fixture:wires")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "reasoning.wires.fixture")?)?
                .claim(ArtifactIdentityClaim::codec_extension("reasoning.wires.fixture", "wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Mindmap Wires")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Mindmap Wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Mindmap-Wires")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Mindmap-Wires")?)?,
        )
}

/// 🗿️ New declaration-tree registration channel (ticket
/// `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` design.md §1/§2) — the ONLY channel: the old
/// `declaration()` (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...)
/// .document_codec(...)` chain) is deleted outright, not kept alongside this, per the ticket's own
/// "Rejected approaches" ruling against dual registration. `localization: &[]` is a documented
/// shortfall: the real en/de localized names (`"Mindmap Wires"`/`"Mindmap-Wires"`) still live on
/// `definition()`'s `ArtifactCapability` rows above (kept, per debt D1) — wiring them into this
/// field is real follow-up work, not required for this pass (mirrors `📓️w4-sequence-report.md`
/// `## openQuestions` #2).
pub fn artifact<A: WiresApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.reasoning.wires").expect("canonical reasoning.wires kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait WiresApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::wires::ReasoningWiresPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::wires::WiresViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> WiresApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::wires::ReasoningWiresPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::wires::WiresViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

//#endregion 🔖️Declaration

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
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
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
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
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

pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod io {
    pub use super::standards::v1::subsets::any::io::*;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod snapshot {
    pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
    }
    pub mod binary {
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
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wires {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔵️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️add-relationship/🦀️.rs"]
            pub mod add_relationship;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👇️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs"]
            pub mod canvas_pointer_move;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👆️canvas-pointer-up/🦀️.rs"]
            pub mod canvas_pointer_up;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🦀️.rs"]
                    pub mod canvas;
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
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wires {
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🕸️canvas/🦀️.rs"]
                    pub mod canvas;
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
pub fn wires_child_restore_projection(snapshot: &crate::WiresSnapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wires.child.projection"), error.to_string()))
}
//#endregion 🧬️ChildRestoreProjection
