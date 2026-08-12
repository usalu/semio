//! 🧠️ Wires artifact — the document entity this plugin's one app (🔌️wires) edits.
//!
//! `WiresSnapshot`'s `wires_fixture`/`board_fixture` fields stay opaque `dsl::DslValue` HERE,
//! deliberately: `⚙️engine`/`🖱️commands`/`🔧️op` all address board nodes/edges and wires relationships
//! generically by id (`array_mut`/`entity_id`/JSON-patch-style ops) for mergeable, granular edits, and
//! re-typing this struct's own fields would force all of that machinery onto typed field access. The
//! `.wires` TEXTUAL surface doesn't need that genericity, so it's fully typed via the `*Dsl` mirror
//! types in `🔖️DslMirror` below, converted at the `parse_dsl`/`print_dsl`/pack boundary — same "local
//! twin" pattern as `procedural_3d`'s `CameraJsonDsl`/`WidgetDsl`/`SynapseSpecDsl`.

use dsl::DslValue;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
pub use crate::artifacts::wires::schema::mutations::WiresMutation;

pub use crate::artifacts::wires::schema::diff::WiresDiff;

pub const MINDMAP_WIRES_SCHEMA: &str = "reasoning.wires.fixture";
/// 🕸️ Mindmap's own board fixture schema — recognized by the neutral force-graph-layout crate
/// (`infinite_board_normal_undirected`) as an undirected graph, distinct from puzzle's directed
/// `puzzle.2d.fixture` board.
pub const MINDMAP_BOARD_SCHEMA: &str = "reasoning.mindmap.fixture";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 📸️ Persisted wires snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::wires::schema::snapshot::WiresSnapshot;
pub use crate::artifacts::wires::schema::WiresArtifact;
//#endregion 🔖️Types

//#region 🔖️EmptyFixtures
/// 📭️ Empty `reasoning.mindmap.fixture` board blob for tests and fresh documents.
pub fn empty_board_fixture() -> DslValue {
    DslValue::object([
        ("schema".into(), DslValue::String(MINDMAP_BOARD_SCHEMA.into())),
        ("camera".into(), DslValue::object([("x".into(), DslValue::Number(0.0)), ("y".into(), DslValue::Number(0.0)), ("zoom".into(), DslValue::Number(1.0))])),
        ("nodes".into(), DslValue::Array(vec![])),
        ("edges".into(), DslValue::Array(vec![])),
        ("wires".into(), DslValue::Array(vec![])),
    ])
}

/// 📭️ Empty `reasoning.wires.fixture` blob for tests and fresh documents.
pub fn empty_wires_fixture() -> DslValue {
    DslValue::object([("schema".into(), DslValue::String(MINDMAP_WIRES_SCHEMA.into())), ("identities".into(), DslValue::Array(vec![])), ("relationships".into(), DslValue::Array(vec![])), ("board".into(), empty_board_fixture())])
}

/// 📭️ Fresh wires snapshot with empty fixtures.
pub fn empty_wires_snapshot() -> WiresSnapshot {
    WiresSnapshot { wires_fixture: empty_wires_fixture(), board_fixture: empty_board_fixture() }
}
//#endregion 🔖️EmptyFixtures

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::wires::create_wires_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "graph.wires".into(),
        name: "Wires Graph".into(),
        source_format: MINDMAP_WIRES_SCHEMA.into(),
        component_kind: "wires".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Dag },
        schema: MINDMAP_WIRES_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"],
    }
}
//#endregion 🔖️ArtifactKind

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
    pub handles: Vec<DslValue>,
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
/// reference (mirrors `crate::apps::wires::panels::inspection::DefaultWiresExtension::allowed_identities`).
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
    pub wires: Vec<DslValue>,
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
/// `kind` is one of `crate::apps::wires::panels::inspection::RelationshipKind::label()`'s four values
/// (`"owns"`/`"is"`/`"references"`/`"has"`), kept as a plain string here since that enum lives in the
/// engine component — see the module doc above for why this component stays generic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDsl {
    pub relationship_id: u64,
    pub kind: String,
    pub source_identity_id: u64,
    pub target_identity_id: u64,
    pub edge_id: String,
}

/// 📦️ `wires_fixture.source` — provenance of the kit this fixture was generated from;
/// absent for hand-authored fixtures with no kit origin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SourceDsl {
    pub kit_id: String,
    pub kit_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kit_path: Option<String>,
}

/// 🧠️ The `reasoning.wires.fixture` semantic layer — schema/identities/relationships, its own
/// nested board-fixture copy (`board`, the same `BoardFixtureDsl` shape as the top-level
/// `board_fixture`), and optional kit `source` provenance.
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
//#endregion 🔖️DslMirror
//#endregion 🔖️Dsl

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_uses_the_wires_fixture_schema() {
        assert_eq!(artifact_kind().schema, MINDMAP_WIRES_SCHEMA);
        assert_eq!(artifact_kind().id, "graph.wires");
    }

    #[test]
    fn empty_snapshot_has_empty_fixtures() {
        let snapshot = empty_wires_snapshot();
        assert_eq!(snapshot.wires_fixture.get("identities").and_then(|value| value.as_array()).map(|items| items.len()), Some(0));
        assert_eq!(snapshot.board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(0));
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::wires::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("WiresComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
