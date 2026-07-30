//! 🛒 Sourcing curate app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

pub const SOURCING_CURATE_SCHEMA: &str = "sourcing.curate/v1";

//#region 🔖Geometry
/// 📦 A parametric geometry recipe an object kind is composed of — data describing shape, not a subclass.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GeometryRecipe {
    Box { width: f64, height: f64, depth: f64 },
    Frame { width: f64, height: f64, depth: f64, profile: f64 },
    Slab { width: f64, depth: f64, thickness: f64 },
    Mesh { positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32> },
}
//#endregion 🔖Geometry

//#region 🔖ObjectKind
/// 🧱 A catalogue object KIND: identity ∘ typology reference ∘ availability ∘ geometry (composition, not subclassing).
///
/// `geometry` is `Box<GeometryRecipe>` (not a bare `GeometryRecipe`) because `#[dsl(statements)]`'s
/// `RequiredStatements` shape — the "exactly one required tagged value" slot a `DslEnum` sum type
/// needs to occupy a plain (non-`Option`, non-`Vec`) field — only recognizes a `Box<T>` inner type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjectKind {
    pub id: String,
    pub name: String,
    pub module_id: String,
    pub typology_path: Vec<String>,
    pub availability: u32,
    #[dsl(statements)]
    pub geometry: Box<GeometryRecipe>,
}
//#endregion 🔖ObjectKind

//#region 🔖Document
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct TableSort {
    pub column_id: String,
    pub direction: SortDirection,
}

/// 🔍 The pool table's active filter set — narrows `CurateDocument::stock` down to `filtered_stock()`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub module_ids: Vec<String>,
    #[serde(default)]
    pub typology_path: Vec<String>,
    #[serde(default)]
    pub min_availability: u32,
    #[serde(default)]
    #[dsl(block)]
    pub sort: Option<TableSort>,
}

/// 🧺 One curated object kind and how many units of it have been picked.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CuratedItem {
    pub object_id: String,
    pub count: u32,
}

/// 🖱️ Ephemeral cross-window UI state — which single object is selected for the preview window.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CurateRuntime {
    #[serde(default)]
    pub selected_object_id: Option<String>,
}

/// 🛒 The curate document: a stock of catalogue kinds ∘ filters ∘ a curated set ∘ ephemeral runtime state.
///
/// Query/mutation logic over this document (`filtered_stock`, `curated_count`, `curate_delta`,
/// `curate_set`) lives in `sourcing_engine` as free functions, not as inherent methods here — Rust's
/// orphan rule forbids an `impl CurateDocument` block outside this crate, and `sourcing_engine` is
/// where "ALL pure compute over the document" belongs per the constitutional split.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "curate", layout = "lines")]
pub struct CurateDocument {
    #[serde(default)]
    pub stock: Vec<ObjectKind>,
    #[serde(default)]
    #[dsl(block)]
    pub filters: Filters,
    #[serde(default)]
    #[dsl(table)]
    pub curated: Vec<CuratedItem>,
    #[serde(default)]
    #[dsl(block)]
    pub runtime: CurateRuntime,
}
//#endregion 🔖Document
