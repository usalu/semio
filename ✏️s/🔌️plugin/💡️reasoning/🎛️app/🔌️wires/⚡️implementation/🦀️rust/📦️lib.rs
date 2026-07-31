//! 🧠️ Reasoning wires app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Constants
pub const MINDMAP_WIRES_SCHEMA: &str = "reasoning.wires.fixture";
/// 🕸️ Mindmap's own board fixture schema — recognized by the neutral force-graph-layout crate
/// (`infinite_board_normal_undirected`) as an undirected graph, distinct from puzzle's directed
/// `puzzle.2d.fixture` board.
pub const MINDMAP_BOARD_SCHEMA: &str = "reasoning.mindmap.fixture";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🧠️ The mindmap-wires document: the semantic wires fixture (identities/relationships/kind catalogs)
/// paired with its own `reasoning.mindmap.fixture` board fixture (nodes/edges/camera). Both fields
/// stay opaque `serde_json::Value` HERE, deliberately: `reasoning_wires_engine`/`reasoning_wires_ui`/
/// `reasoning_wires_op` all address board nodes/edges and wires relationships generically by id
/// (`array_mut`/`entity_id`/JSON-patch-style ops) for mergeable, granular edits, and re-typing this
/// struct's own fields would force all of that machinery onto typed field access. The `.wires`
/// TEXTUAL surface doesn't need that genericity, so it's fully typed via the `*Dsl` mirror types in
/// `🔖️DslMirror` below, converted at the `parse_dsl`/`print_dsl`/pack boundary — same "local twin"
/// pattern as `procedural_3d`'s `CameraJsonDsl`/`WidgetDsl`/`SynapseSpecDsl`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapWiresDocument {
    pub wires_fixture: Value,
    pub board_fixture: Value,
}
//#endregion 🔖️Types

//#region 🔖️Dsl
//#region 🔖️DslMirror
/// 🎥️ Camera for a `reasoning.mindmap.fixture` board — pan/zoom, mirrors `puzzle_2d`'s
/// `Puzzle2dCamera` (the same generic board-fixture family, see the module doc above).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CameraDsl {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

/// 🔵️ One mindmap-board node — mirrors `puzzle_2d`'s `Puzzle2dNode` field-for-field (`shape:
/// "circle"` is radius-sized, `"rectangle"` is width/height-sized). `handles` is always an empty
/// array in every fixture and call site this app has (mindmap nodes have no ports) — kept as a
/// justified `Vec<Value>` escape hatch rather than typed purely so a future populated handle never
/// silently fails to round-trip; see `dsl::Shape::Value`'s doc for the escape-hatch contract.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct NodeDsl {
    #[dsl(defines = "node")]
    pub id: String,
    pub node_kind: String,
    pub shape: String,
    pub x: f64,
    pub y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    #[serde(default)]
    pub handles: Vec<Value>,
}

/// ➡️ One board edge — connects two `NodeDsl::id`s directly (mindmap nodes have no ports, unlike
/// `puzzle_2d`'s handle-to-handle edges).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct EdgeDsl {
    pub id: String,
    #[dsl(refs = "node")]
    pub source: String,
    #[dsl(refs = "node")]
    pub target: String,
}

/// 🎨️ One `meta.kindCatalogs.identityKinds` row — a node-kind's display style.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct IdentityKindDsl {
    pub id: String,
    pub name: String,
    pub shape: String,
    pub color: String,
}

/// 🔗️ One `meta.kindCatalogs.relationshipKinds` row — a relationship-kind's display style.
/// `stroke` is a CSS-style stroke-width string (e.g. `"2.5"`), not a number, in every real fixture.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipKindDsl {
    pub id: String,
    pub name: String,
    pub directed: bool,
    pub pattern: String,
    pub stroke: String,
    pub target_tip: String,
    pub color: String,
}

/// 🗂️ `board.meta.kindCatalogs` — the two style catalogs a WIRES board's identities/relationships
/// resolve their `identityKind`/relationship `kind` display against.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct KindCatalogsDsl {
    #[serde(default)]
    #[dsl(table)]
    pub identity_kinds: Vec<IdentityKindDsl>,
    #[serde(default)]
    #[dsl(table)]
    pub relationship_kinds: Vec<RelationshipKindDsl>,
}

/// 🔒️ `board.meta.wires` — the fixed identity-id vocabulary this WIRES board is allowed to
/// reference (mirrors `reasoning_wires_engine::DefaultWiresExtension::allowed_identities`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MetaWiresDsl {
    #[serde(default)]
    pub allowed_identity_ids: Vec<u64>,
}

/// 🗂️ `board.meta` — present whenever a board carries kind catalogs / an allowed-identity set;
/// absent for the degenerate empty document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MetaDsl {
    #[dsl(block)]
    pub kind_catalogs: KindCatalogsDsl,
    #[dsl(block)]
    pub wires: MetaWiresDsl,
}

/// 🕸️ The `reasoning.mindmap.fixture` board — schema/camera/nodes/edges/meta, plus an always-empty
/// `wires` routing-line array (see `NodeDsl::handles`'s doc — same "kept for lossless round-trip with
/// the shared generic board-fixture family" reasoning; nothing in this app ever populates it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BoardFixtureDsl {
    pub schema: String,
    #[dsl(block)]
    pub camera: CameraDsl,
    #[serde(default)]
    #[dsl(table)]
    pub nodes: Vec<NodeDsl>,
    #[serde(default)]
    #[dsl(table)]
    pub edges: Vec<EdgeDsl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub meta: Option<MetaDsl>,
    #[serde(default)]
    pub wires: Vec<Value>,
}

/// 🪪️ One `wires_fixture.identities` row — a board node wearing a semantic WIRES identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct IdentityDsl {
    pub identity_id: u64,
    pub identity_kind: String,
    pub label: String,
    pub node_id: String,
}

/// 🔗️ One `wires_fixture.relationships` row — a semantic WIRES relationship between two identities,
/// `kind` is one of `reasoning_wires_engine::RelationshipKind::label()`'s four values
/// (`"owns"`/`"is"`/`"references"`/`"has"`), kept as a plain string here since that enum lives in
/// the (downstream) engine crate — see the module doc above for why this crate stays generic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDsl {
    pub relationship_id: u64,
    pub kind: String,
    pub source_identity_id: u64,
    pub target_identity_id: u64,
    pub edge_id: String,
}

/// 📦️ `wires_fixture.source` — provenance of the compose kit this fixture was generated from;
/// absent for hand-authored fixtures with no kit origin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SourceDsl {
    pub kit_id: String,
    pub kit_name: String,
    pub kit_path: String,
}

/// 🧠️ The `reasoning.wires.fixture` semantic layer — schema/identities/relationships, its own
/// nested board-fixture copy (`board`, the same `BoardFixtureDsl` shape as `MindmapWiresDocument`'s
/// separate `board_fixture`), and optional kit `source` provenance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WiresFixtureDsl {
    pub schema: String,
    #[serde(default)]
    #[dsl(table)]
    pub identities: Vec<IdentityDsl>,
    #[serde(default)]
    #[dsl(table)]
    pub relationships: Vec<RelationshipDsl>,
    #[dsl(block)]
    pub board: BoardFixtureDsl,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub source: Option<SourceDsl>,
}

/// 🧾️ Local DSL-mirror twin of `MindmapWiresDocument` — see `🔖️DslMirror`'s intro doc above for why
/// the real struct keeps opaque `Value` fields while this twin (used only at the `parse_dsl`/
/// `print_dsl`/pack boundary) is fully typed.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "wires", layout = "lines")]
struct MindmapWiresDocumentDsl {
    #[dsl(key = "wires", block)]
    wires_fixture: WiresFixtureDsl,
    #[dsl(key = "board", block)]
    board_fixture: BoardFixtureDsl,
}

/// 🔀️ Real document (opaque `Value`) → DSL mirror (typed), for `print_dsl`/`encode_pack_with`. A
/// stored `Value` that doesn't match the fixed `reasoning.wires.fixture`/`reasoning.mindmap.fixture`
/// shape is a genuine data-corruption bug, not a case to silently coerce — hence the panic rather
/// than a `Result` (this direction can't return one: `store::DocumentDsl::print_dsl` is infallible).
fn mindmap_wires_document_to_dsl(document: &MindmapWiresDocument) -> MindmapWiresDocumentDsl {
    MindmapWiresDocumentDsl {
        wires_fixture: serde_json::from_value(document.wires_fixture.clone())
            .unwrap_or_else(|error| panic!("wires_fixture does not match the reasoning.wires.fixture schema: {error}")),
        board_fixture: serde_json::from_value(document.board_fixture.clone())
            .unwrap_or_else(|error| panic!("board_fixture does not match the reasoning.mindmap.fixture schema: {error}")),
    }
}

/// 🔀️ DSL mirror (typed) → real document (opaque `Value`), for `parse_dsl`/`decode_pack_with`.
fn mindmap_wires_document_from_dsl(parsed: MindmapWiresDocumentDsl) -> Result<MindmapWiresDocument, store::TextError> {
    Ok(MindmapWiresDocument {
        wires_fixture: serde_json::to_value(&parsed.wires_fixture)
            .map_err(|error| store::TextError::new(format!("invalid wires fixture: {error}"), store::TextSpan::at(1, 1)))?,
        board_fixture: serde_json::to_value(&parsed.board_fixture)
            .map_err(|error| store::TextError::new(format!("invalid board fixture: {error}"), store::TextSpan::at(1, 1)))?,
    })
}
//#endregion 🔖️DslMirror

/// 📜️ `.wires` textual document — derive-engine grammar via `MindmapWiresDocumentDsl` (see
/// `🔖️DslMirror`); `parse_dsl`/`print_dsl` convert at the boundary.
impl store::DocumentDsl for MindmapWiresDocument {
    const EXTENSION: &'static str = "wires";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <MindmapWiresDocumentDsl as store::DocumentDsl>::parse_dsl(text)?;
        mindmap_wires_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <MindmapWiresDocumentDsl as store::DocumentDsl>::print_dsl(&mindmap_wires_document_to_dsl(self))
    }
}

/// 📦️ `.wires` binary pack — same `MindmapWiresDocumentDsl` mirror as `DocumentDsl` above (see
/// `🔖️DslMirror`); `dsl::DslDocument`'s derive already gives `MindmapWiresDocumentDsl` its own
/// `DocumentPack` impl, so this just routes through the same to/from-dsl boundary functions.
impl store::DocumentPack for MindmapWiresDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <MindmapWiresDocumentDsl as store::DocumentPack>::encode_pack_with(&mindmap_wires_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <MindmapWiresDocumentDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        mindmap_wires_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }
}
//#endregion 🔖️Dsl
