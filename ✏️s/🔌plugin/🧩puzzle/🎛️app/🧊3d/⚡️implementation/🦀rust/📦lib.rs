//! 🧩 Puzzle 3d app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

//#region ⚠️ Errors
/// 🧯 Puzzle 3d precompute session errors — JSON (de)serialization and brush/fill session state failures.
#[derive(Debug, thiserror::Error)]
pub enum Puzzle3dError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("brush placement rejected")]
    BrushPlacementRejected,
    #[error("fill session unavailable")]
    FillSessionUnavailable,
}
//#endregion ⚠️ Errors

pub const PUZZLE_3D_SCHEMA: &str = "puzzle.3d";

// #region 🔖Document
/// 🔘 One vortex on an object's rim — `vortex_kind` gates attraction compatibility, `position`/
/// `direction` place and orient it, `radius` sizes its brush-fill collision.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dVortex {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vortex_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
}

/// 🧱 One placed object — `origin`/`orientation`/`scale` (a freeform Vec3-or-scalar, see
/// `vec3_scale`) pose it, `vortices` are its rim attraction ports.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dObject {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_kind: Option<String>,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub vortices: Vec<Puzzle3dVortex>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
}

/// 🔗 One attraction between two full vortex ids (`object_id:vortex_id`), with the gap/shift/rise/
/// rotation/turn/tilt offsets `compute_brush_placement_pose` resolves into a world pose.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dAttraction {
    #[serde(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
    #[serde(default)]
    pub gap: f64,
    #[serde(default)]
    pub shift: f64,
    #[serde(default)]
    pub rise: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub turn: f64,
    #[serde(default)]
    pub tilt: f64,
}

/// 🧊 A persisted oriented box constraining fill placement (Volume Brush voxels or Transform-gumball
/// edited volumes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolume {
    pub id: String,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<serde_json::Value>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
}

/// 🌐 Where a reference image/media's bytes live and what kind of media it is.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dReferenceSource {
    #[serde(default)]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_kind: Option<String>,
}

/// 🖼️ A reference plane pinned in world space at `origin`, `width_world` meters wide.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dReference {
    pub id: String,
    #[serde(default)]
    pub source: Puzzle3dReferenceSource,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub width_world: f64,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub hidden: bool,
}

/// 🔗 How specifically two vortex/cable kinds are allowed to attract (mirrors `KindCompatEntry`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dKindCompatibility {
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub bidirectional: bool,
    #[serde(default)]
    pub important: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specificity: Option<String>,
}

/// 🌱 One rim-vortex template on a `Puzzle3dCatalogObjectKind` (no `label`/`hidden`/`locked` — those
/// are only per-instance `Puzzle3dVortex` fields, not catalog template fields).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexTemplate {
    pub vortex_kind: String,
    pub position: [f64; 3],
    pub direction: [f64; 3],
    pub radius: f64,
}

/// 🧱 One object-kind catalog row (mirrors this crate's internal `ObjectKind`, extended with the
/// fixture-observed `label`/`name` display fields).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogObjectKind {
    pub id: String,
    pub label: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub vortices: Vec<Puzzle3dCatalogVortexTemplate>,
}

/// 🔘 One vortex-kind catalog row (mirrors `VortexKindCatalog`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexKind {
    pub id: String,
    pub label: String,
    pub name: String,
    pub color: String,
    pub default_cable_kind: String,
}

/// 🧵 One cable-kind catalog row (mirrors `CableKindCatalog`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogCableKind {
    pub id: String,
    pub label: String,
    pub name: String,
    pub default_attraction_kind: String,
}

/// 🔗 One attraction-kind catalog row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCatalogAttractionKind {
    pub id: String,
    pub label: String,
    pub name: String,
}

/// 🗂️ The compile-time-catalog side of a self-contained fixture export: object/vortex/cable/
/// attraction kind rows — see `puzzle/3d/manifest/*.manifest.json` for the same schema at the
/// manifest layer.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dKindCatalogs {
    #[serde(default)]
    #[dsl(table)]
    pub objects: Vec<Puzzle3dCatalogObjectKind>,
    #[serde(default)]
    #[dsl(table)]
    pub vortices: Vec<Puzzle3dCatalogVortexKind>,
    #[serde(default)]
    #[dsl(table)]
    pub cables: Vec<Puzzle3dCatalogCableKind>,
    #[serde(default)]
    #[dsl(table)]
    pub attractions: Vec<Puzzle3dCatalogAttractionKind>,
}

/// 🗂️ Fixture-carried metadata: the explicit link-compatibility table (typed — a well-understood
/// small structured list, matching this crate's own `KindCompatEntry`) plus the object/vortex/cable/
/// attraction kind catalog bundle (typed — see `Puzzle3dKindCatalogs`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Puzzle3dKindCatalogs>,
    #[serde(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle3dKindCompatibility>,
}

/// 🧩 The puzzle-3d projection: a typed fixture document (schema/domain/meta/objects/
/// attractions/targetVolumes/references) — see `puzzle/3d/example/*.3d.json` for real-world shapes.
/// Camera is intentionally absent: it is session-only per-window runtime state (never a document
/// field), owned by the app's `Puzzle3dWindowOptions` — see that crate's ticket-driven cutover.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "puzzle3d", layout = "lines")]
pub struct Puzzle3dProjection {
    pub schema: String,
    #[serde(default)]
    pub domain: String,
    #[dsl(block)]
    #[serde(default)]
    pub meta: Puzzle3dMeta,
    #[serde(default)]
    #[dsl(table)]
    pub objects: Vec<Puzzle3dObject>,
    #[serde(default)]
    #[dsl(table)]
    pub attractions: Vec<Puzzle3dAttraction>,
    #[serde(default)]
    #[dsl(table)]
    pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[serde(default)]
    #[dsl(table)]
    pub references: Vec<Puzzle3dReference>,
}

impl Default for Puzzle3dProjection {
    fn default() -> Self {
        Self { schema: PUZZLE_3D_SCHEMA.to_string(), domain: "architecture".to_string(), meta: Puzzle3dMeta::default(), objects: Vec::new(), attractions: Vec::new(), target_volumes: Vec::new(), references: Vec::new() }
    }
}
