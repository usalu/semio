//! 🧩️ Puzzle 2d app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

pub const PUZZLE_2D_SCHEMA: &str = "puzzle.2d.fixture";

// #region 🔖️Document
/// 🎥️ The canvas camera (pan/zoom) for a puzzle 2d fixture.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Puzzle2dCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🔘️ One port on a node's rim — `handle_kind` gates link compatibility, `angle`/`radius` place it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dHandle {
    #[dsl(defines = "handle")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle_kind: Option<String>,
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🔵️ One node — `shape: "circle"` (default, radius-sized) or `"rectangle"` (width/height-sized);
/// `handles` are its rim ports. Mirrors `infinite_board_port_directed::scene_json::FixtureJson`'s
/// per-node fields, the canonical parser this fixture format round-trips through.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dNode {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    pub x: f64,
    pub y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(default)]
    pub handles: Vec<Puzzle2dHandle>,
}

/// ➡️ One directed link between two handle ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dEdge {
    pub id: String,
    #[dsl(refs = "handle")]
    pub source: String,
    #[dsl(refs = "handle")]
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_tip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_tip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🔗️ How specifically two handle/wire kinds are allowed to link — `vortex` is a ported-graph alias
/// for `handle` (see `infinite_board_port_directed_normal::parse_compat_specificity`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "lowercase")]
pub enum Puzzle2dCompatSpecificity {
    General,
    Node,
    Edge,
    Handle,
    Wire,
    Vortex,
}

/// 🧩️ One allowed (or, unidirectional, one-way-allowed) link pair between two handle/wire kind ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dKindCompatibility {
    #[serde(default)]
    pub bidirectional: bool,
    pub specificity: Puzzle2dCompatSpecificity,
    pub source: String,
    pub target: String,
}

/// 🗂️ Fixture-carried metadata: the manifest this fixture's kinds resolve against, its explicit
/// link-compatibility table, and (rarely) a self-contained `kindCatalogs` payload for fixtures
/// exported standalone — that catalog shape is genuinely freeform (handle/wire/edge kind rows vary
/// per manifest), so it stays untyped rather than duplicating the manifest schema here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[serde(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle2dKindCompatibility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<serde_json::Value>,
}

/// 🧩️ The puzzle-2d projection: a typed fixture document (schema/camera/nodes/edges/meta) — see
/// `infinite_board_port_directed::scene_json::FixtureJson` for the canonical parser it round-trips
/// through and the two example fixtures under `puzzle/2d/example/` for real-world shapes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "puzzle2d", layout = "lines")]
pub struct Puzzle2dProjection {
    pub schema: String,
    #[dsl(block)]
    pub camera: Puzzle2dCamera,
    #[serde(default)]
    #[dsl(table)]
    pub nodes: Vec<Puzzle2dNode>,
    #[serde(default)]
    #[dsl(table)]
    pub edges: Vec<Puzzle2dEdge>,
    #[serde(default)]
    #[dsl(block)]
    pub meta: Puzzle2dMeta,
}

impl Default for Puzzle2dProjection {
    fn default() -> Self {
        Self { schema: PUZZLE_2D_SCHEMA.to_string(), camera: Puzzle2dCamera::default(), nodes: Vec::new(), edges: Vec::new(), meta: Puzzle2dMeta::default() }
    }
}
