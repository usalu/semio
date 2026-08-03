//! ⚙️ Sourcing curate app — headless compute (constitutional: engine).
//!
//! Query/mutation logic over `CurateDocument` lives here as free functions (`filtered_stock`,
//! `curated_count`, `curate_delta`, `curate_set`) rather than inherent methods — Rust's orphan rule
//! forbids an `impl CurateDocument` block outside the crate that defines the type (`sourcing`).

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sourcing::{CurateDocument, CuratedItem, Filters, GeometryRecipe};

/// 🔁️ Re-exported for `sourcing-module-{beams,windows,slabs}`, which depend on this crate (aliased
/// `sourcing_curate`) for both the entity type and the module trait/fixture data.
pub use sourcing::ObjectKind;

//#region 🔖️Config
/// 🧮️ B1: curate's real `DocumentApp::Config` — the pure-trait pilot's config artifact. Absorbs both
/// the former `CurateDocument.filters` (search/sort — the `Filters` type is unchanged, just relocated)
/// AND `CurateDocument.runtime.selected_object_id` (flattened to a plain field here, no `#[dsl(refs =
/// "object")]` — Config is a separate DSL document/store from `CurateDocument`, so the `#[dsl(defines =
/// "object")]` namespace `ObjectKind::id` establishes there can't be validated across the two anyway;
/// see `shooting_engine::ShootingConfig`'s analogous drop of `#[dsl(refs = ...)]` on its own former
/// `ShootingPlayRuntime` selection-id fields). `locale` is the `cfg.locale`-driven counterpart to the
/// deleted `ViewState.locale` — `DocumentApp::render`/`handle` no longer receive a `ViewState` at all,
/// so locale-aware label resolution now reads it off here (see `sourcing_ui::is_de_locale`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "sourcingcuratecfg")]
#[dsl(layout = "lines")]
pub struct SourcingCurateConfig {
    /// 🔍️ Was `CurateDocument.filters` — the pool table's active filter/search/sort state.
    #[dsl(block)]
    pub filters: Filters,
    /// 👁️ Was `CurateDocument.runtime.selected_object_id` — the single object selected for the preview window.
    pub selected_object_id: Option<String>,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for SourcingCurateConfig {
    fn default() -> Self {
        Self { filters: Filters::default(), selected_object_id: None, locale: "en-US".into() }
    }
}

impl store::ConfigRecord for SourcingCurateConfig {}

/// @emoji 🧮️ Whole-record diff for `sourcing_op::SourcingCurateConfigOperation` (lives here, not in
/// `sourcing_op`, since `protocol::OperationDiff`/`SourcingCurateConfig` are both foreign to that crate
/// — the orphan rule requires at least one local type). Mirrors `SourcingOperation::SetDocument`'s
/// existing "whole-document replace" pattern: `apply` ignores `base` entirely.
impl protocol::OperationDiff<SourcingCurateConfig> for SourcingCurateConfig {
    fn apply(&self, _base: &SourcingCurateConfig) -> SourcingCurateConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports (keyed off
/// `sourcing::SOURCING_CURATE_SCHEMA`, `MediaType{Kit,Kit}` matching the `"catalogue.sourcing"`
/// `ArtifactKindSpec`) plus the extra `catalog:out` output port: this app's `stock` (its
/// `"catalogue.kinds"`-shaped rows) mapped into the SAME `kit.catalog` JSON shape
/// `block_3d::puzzle3d_catalog_fragment` produces, so `s/plugin/puzzle`'s `kit:in` importer can consume
/// either producer identically without knowing which one it came from (see `sourcing_catalog_fragment`).
pub fn sourcing_curate_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: sourcing::SOURCING_CURATE_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Kit },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "catalog:out".into(),
            label: "Catalog".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: semio_framework_core::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "catalogue.sourcing".into(), name: "Sourcing Curation".into(), dimension: "data".into(), component_kind: "catalogue".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this app's stock (its `"catalogue.kinds"`-shaped rows) into the `s/plugin/puzzle` 3d catalog
/// shape (`objectKinds`/`vortexKinds`/`cableKinds`/`attractionKinds`/`kindCompatibility` — see
/// `block_3d::puzzle3d_catalog_fragment`, the sibling producer this mirrors byte-for-byte in shape), the
/// seam puzzle imports through its `Kit×Type` `kit:in` media port. Sourcing's `ObjectKind` carries no
/// mesh URL (geometry is a procedural `GeometryRecipe`, not an asset reference) or vortex/attachment
/// data, so every row's `meshUrl` is `null` and `vortices` is empty — puzzle's importer treats a missing
/// mesh as "no visual representation yet", not an error.
pub fn sourcing_catalog_fragment(document: &CurateDocument) -> Value {
    let object_kinds: Vec<Value> = document.stock.iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.name, "meshUrl": Value::Null, "vortices": Vec::<Value>::new() })).collect();
    json!({
        "schema": "manifest",
        "objectKinds": object_kinds,
        "vortexKinds": Vec::<Value>::new(),
        "cableKinds": Vec::<Value>::new(),
        "attractionKinds": Vec::<Value>::new(),
        "kindCompatibility": Vec::<Value>::new(),
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Typology
/// 🌳️ One node in a module's typology tree — object kinds reference a node by its path of segment ids.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypologyNode {
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<TypologyNode>,
}

impl TypologyNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>, children: Vec<TypologyNode>) -> Self {
        Self { id: id.into(), label: label.into(), children }
    }
}

/// 🔎️ Whether `path` (a sequence of segment ids from the root) resolves to a node in `root`'s tree.
pub fn typology_contains(root: &TypologyNode, path: &[String]) -> bool {
    match path.split_first() {
        None => true,
        Some((head, rest)) if *head == root.id => {
            if rest.is_empty() {
                true
            } else {
                root.children.iter().any(|child| typology_contains(child, rest))
            }
        }
        _ => false,
    }
}

/// 📋️ Flattens a typology tree into `(full path from root, label)` pairs, depth-first, for filter UIs.
pub fn typology_flatten(root: &TypologyNode) -> Vec<(Vec<String>, String)> {
    fn walk(node: &TypologyNode, prefix: &[String], out: &mut Vec<(Vec<String>, String)>) {
        let mut path = prefix.to_vec();
        path.push(node.id.clone());
        out.push((path.clone(), node.label.clone()));
        for child in &node.children {
            walk(child, &path, out);
        }
    }
    let mut out = Vec::new();
    walk(root, &[], &mut out);
    out
}
//#endregion 🔖️Typology

//#region 🔖️Geometry
/// 🧱️ Flat indexed triangle mesh data, ready for `mesh_from_indexed` at the plugin boundary.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshDataSpec {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
}

/// ➕️ Appends `other` onto `base`, offsetting `other`'s indices past `base`'s existing vertex count.
pub fn append_mesh_spec(base: &mut MeshDataSpec, other: MeshDataSpec) {
    let vertex_offset = (base.positions.len() / 3) as u32;
    base.positions.extend(other.positions);
    base.normals.extend(other.normals);
    base.indices.extend(other.indices.into_iter().map(|i| i + vertex_offset));
}

/// 📐️ Builds an axis-aligned box mesh centered at the origin, with per-face flat normals.
fn box_mesh_spec(width: f64, height: f64, depth: f64) -> MeshDataSpec {
    let (hw, hh, hd) = ((width * 0.5) as f32, (height * 0.5) as f32, (depth * 0.5) as f32);
    // 6 faces * 4 verts, wound counter-clockwise when viewed from outside along the face normal.
    let faces: [([f32; 3], [[f32; 3]; 4]); 6] = [
        ([0.0, 0.0, 1.0], [[-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]]),
        ([0.0, 0.0, -1.0], [[hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]]),
        ([1.0, 0.0, 0.0], [[hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]]),
        ([-1.0, 0.0, 0.0], [[-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]]),
        ([0.0, 1.0, 0.0], [[-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]]),
        ([0.0, -1.0, 0.0], [[-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]]),
    ];
    let mut spec = MeshDataSpec::default();
    for (normal, corners) in faces {
        let base = (spec.positions.len() / 3) as u32;
        for corner in corners {
            spec.positions.extend(corner);
            spec.normals.extend(normal);
        }
        spec.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    spec
}

/// 🪟️ Builds a rectangular frame (4 mitred boxes: top/bottom rails, left/right stiles) around an opening.
fn frame_mesh_spec(width: f64, height: f64, depth: f64, profile: f64) -> MeshDataSpec {
    let mut spec = MeshDataSpec::default();
    let mut add = |w: f64, h: f64, cx: f64, cy: f64| {
        let mut piece = box_mesh_spec(w, h, depth);
        for i in (0..piece.positions.len()).step_by(3) {
            piece.positions[i] += cx as f32;
            piece.positions[i + 1] += cy as f32;
        }
        append_mesh_spec(&mut spec, piece);
    };
    let half_h = height * 0.5;
    let half_w = width * 0.5;
    add(width, profile, 0.0, half_h - profile * 0.5);
    add(width, profile, 0.0, -half_h + profile * 0.5);
    let stile_h = height - profile * 2.0;
    add(profile, stile_h, -half_w + profile * 0.5, 0.0);
    add(profile, stile_h, half_w - profile * 0.5, 0.0);
    spec
}

/// 🧱️ Realizes a `GeometryRecipe` into flat mesh data.
pub fn mesh_spec_for(recipe: &GeometryRecipe) -> MeshDataSpec {
    match recipe {
        GeometryRecipe::Box { width, height, depth } => box_mesh_spec(*width, *height, *depth),
        GeometryRecipe::Frame { width, height, depth, profile } => frame_mesh_spec(*width, *height, *depth, *profile),
        GeometryRecipe::Slab { width, depth, thickness } => box_mesh_spec(*width, *thickness, *depth),
        GeometryRecipe::Mesh { positions, normals, indices } => MeshDataSpec { positions: positions.clone(), normals: normals.clone(), indices: indices.clone() },
    }
}

/// 📏️ The largest bounding dimension of a recipe's geometry, used to normalize grid-cell scale.
pub fn bounding_extent(recipe: &GeometryRecipe) -> f64 {
    match recipe {
        GeometryRecipe::Box { width, height, depth } => width.max(*height).max(*depth),
        GeometryRecipe::Frame { width, height, depth, .. } => width.max(*height).max(*depth),
        GeometryRecipe::Slab { width, depth, thickness } => width.max(*depth).max(*thickness),
        GeometryRecipe::Mesh { positions, .. } => positions.chunks(3).flat_map(|p| p.iter().map(|v| v.abs() as f64 * 2.0)).fold(0.0_f64, f64::max).max(1e-6),
    }
}
//#endregion 🔖️Geometry

//#region 🔖️DocumentHelpers
/// 🔎️ The stock kinds that currently satisfy every active filter dimension. `filters` used to live on
/// `CurateDocument` itself; B1 moved it onto `SourcingCurateConfig` (session-only view state), so this
/// now takes it as a separate parameter rather than reading `document.filters`.
pub fn filtered_stock<'a>(document: &'a CurateDocument, filters: &Filters) -> Vec<&'a ObjectKind> {
    document
        .stock
        .iter()
        .filter(|kind| {
            let query = filters.query.trim().to_lowercase();
            let matches_query = query.is_empty() || kind.name.to_lowercase().contains(&query);
            let matches_module = filters.module_ids.is_empty() || filters.module_ids.contains(&kind.module_id);
            let matches_typology = filters.typology_path.is_empty() || kind.typology_path.starts_with(&filters.typology_path);
            let matches_availability = kind.availability >= filters.min_availability;
            matches_query && matches_module && matches_typology && matches_availability
        })
        .collect()
}

/// 🔢️ How many units of `object_id` are currently in the curated set (0 if absent).
pub fn curated_count(document: &CurateDocument, object_id: &str) -> u32 {
    document.curated.iter().find(|item| item.object_id == object_id).map(|item| item.count).unwrap_or(0)
}

/// ➕️➖️ Adjusts the curated count for `object_id` by `delta`, clamped to `0..=availability`; removes the
/// entry entirely when the count reaches 0. Silently no-operations if `object_id` isn't in the stock.
pub fn curate_delta(document: &mut CurateDocument, object_id: &str, delta: i64) {
    let Some(kind) = document.stock.iter().find(|kind| kind.id == object_id) else { return };
    let next = (curated_count(document, object_id) as i64 + delta).clamp(0, kind.availability as i64) as u32;
    curate_set(document, object_id, next);
}

/// 🎯️ Sets the curated count for `object_id` directly, clamped to `0..=availability`; removes the
/// entry when the count is 0. Silently no-operations if `object_id` isn't in the stock.
pub fn curate_set(document: &mut CurateDocument, object_id: &str, count: u32) {
    let Some(kind) = document.stock.iter().find(|kind| kind.id == object_id) else { return };
    let clamped = count.min(kind.availability);
    match document.curated.iter_mut().find(|item| item.object_id == object_id) {
        Some(item) if clamped == 0 => {
            let id = item.object_id.clone();
            document.curated.retain(|item| item.object_id != id);
        }
        Some(item) => item.count = clamped,
        None if clamped > 0 => document.curated.push(CuratedItem { object_id: object_id.to_string(), count: clamped }),
        None => {}
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Modules
/// 🧩️ A sourcing module composes a typology subtree, demo catalogue kinds, and preview meshing for one
/// object family (e.g. beams, windows, slabs) — modules are trait objects, not subclasses of a base app.
pub trait SourcingModule {
    fn module_id(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn typology(&self) -> TypologyNode;
    fn demo_kinds(&self) -> Vec<ObjectKind>;
    /// 🧱️ Realizes a kind's preview mesh; defaults to the generic geometry recipe realization.
    fn preview_mesh(&self, kind: &ObjectKind) -> MeshDataSpec {
        mesh_spec_for(&kind.geometry)
    }
}

pub mod beams {
    use super::{GeometryRecipe, ObjectKind, SourcingModule, TypologyNode};

    pub struct BeamsModule;

    impl SourcingModule for BeamsModule {
        fn module_id(&self) -> &'static str {
            "beams"
        }
        fn label(&self) -> &'static str {
            "Beams"
        }
        fn typology(&self) -> TypologyNode {
            TypologyNode::new(
                "beams",
                "Beams",
                vec![
                    TypologyNode::new("solid-timber", "Solid Timber", vec![TypologyNode::new("glulam", "Glulam", vec![]), TypologyNode::new("kvh", "KVH", vec![])]),
                    TypologyNode::new("steel", "Steel", vec![TypologyNode::new("ipe", "IPE", vec![]), TypologyNode::new("hea", "HEA", vec![])]),
                ],
            )
        }
        fn demo_kinds(&self) -> Vec<ObjectKind> {
            vec![
                ObjectKind {
                    id: "beam-glulam-gl24h".into(),
                    name: "Glulam GL24h 200×400".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "solid-timber".into(), "glulam".into()],
                    availability: 24,
                    geometry: Box::new(GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 }),
                },
                ObjectKind {
                    id: "beam-kvh-c24".into(),
                    name: "KVH C24 100×200".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "solid-timber".into(), "kvh".into()],
                    availability: 60,
                    geometry: Box::new(GeometryRecipe::Box { width: 0.1, height: 0.2, depth: 4.0 }),
                },
                ObjectKind {
                    id: "beam-steel-ipe200".into(),
                    name: "Steel IPE 200".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "steel".into(), "ipe".into()],
                    availability: 12,
                    geometry: Box::new(GeometryRecipe::Box { width: 0.1, height: 0.2, depth: 5.0 }),
                },
                ObjectKind {
                    id: "beam-steel-hea160".into(),
                    name: "Steel HEA 160".into(),
                    module_id: "beams".into(),
                    typology_path: vec!["beams".into(), "steel".into(), "hea".into()],
                    availability: 8,
                    geometry: Box::new(GeometryRecipe::Box { width: 0.16, height: 0.152, depth: 5.0 }),
                },
            ]
        }
    }
}

pub mod windows {
    use super::{GeometryRecipe, ObjectKind, SourcingModule, TypologyNode};

    pub struct WindowsModule;

    impl SourcingModule for WindowsModule {
        fn module_id(&self) -> &'static str {
            "windows"
        }
        fn label(&self) -> &'static str {
            "Windows"
        }
        fn typology(&self) -> TypologyNode {
            TypologyNode::new("windows", "Windows", vec![TypologyNode::new("casement", "Casement", vec![]), TypologyNode::new("fixed", "Fixed", vec![]), TypologyNode::new("tilt-turn", "Tilt & Turn", vec![])])
        }
        fn demo_kinds(&self) -> Vec<ObjectKind> {
            vec![
                ObjectKind {
                    id: "window-casement-100x120".into(),
                    name: "Casement Window 100×120".into(),
                    module_id: "windows".into(),
                    typology_path: vec!["windows".into(), "casement".into()],
                    availability: 18,
                    geometry: Box::new(GeometryRecipe::Frame { width: 1.0, height: 1.2, depth: 0.08, profile: 0.08 }),
                },
                ObjectKind {
                    id: "window-fixed-150x150".into(),
                    name: "Fixed Window 150×150".into(),
                    module_id: "windows".into(),
                    typology_path: vec!["windows".into(), "fixed".into()],
                    availability: 10,
                    geometry: Box::new(GeometryRecipe::Frame { width: 1.5, height: 1.5, depth: 0.06, profile: 0.06 }),
                },
                ObjectKind {
                    id: "window-tilt-turn-120x140".into(),
                    name: "Tilt & Turn Window 120×140".into(),
                    module_id: "windows".into(),
                    typology_path: vec!["windows".into(), "tilt-turn".into()],
                    availability: 14,
                    geometry: Box::new(GeometryRecipe::Frame { width: 1.2, height: 1.4, depth: 0.09, profile: 0.09 }),
                },
            ]
        }
    }
}

pub mod slabs {
    use super::{GeometryRecipe, ObjectKind, SourcingModule, TypologyNode};

    pub struct SlabsModule;

    impl SourcingModule for SlabsModule {
        fn module_id(&self) -> &'static str {
            "slabs"
        }
        fn label(&self) -> &'static str {
            "Slabs"
        }
        fn typology(&self) -> TypologyNode {
            TypologyNode::new("slabs", "Slabs", vec![TypologyNode::new("concrete", "Concrete", vec![]), TypologyNode::new("clt", "CLT", vec![]), TypologyNode::new("hollow-core", "Hollow Core", vec![])])
        }
        fn demo_kinds(&self) -> Vec<ObjectKind> {
            vec![
                ObjectKind {
                    id: "slab-concrete-240".into(),
                    name: "Concrete Slab 240mm".into(),
                    module_id: "slabs".into(),
                    typology_path: vec!["slabs".into(), "concrete".into()],
                    availability: 30,
                    geometry: Box::new(GeometryRecipe::Slab { width: 2.4, depth: 1.2, thickness: 0.24 }),
                },
                ObjectKind {
                    id: "slab-clt-160".into(),
                    name: "CLT Slab 160mm".into(),
                    module_id: "slabs".into(),
                    typology_path: vec!["slabs".into(), "clt".into()],
                    availability: 20,
                    geometry: Box::new(GeometryRecipe::Slab { width: 2.95, depth: 1.25, thickness: 0.16 }),
                },
                ObjectKind {
                    id: "slab-hollow-core-265".into(),
                    name: "Hollow Core Slab 265mm".into(),
                    module_id: "slabs".into(),
                    typology_path: vec!["slabs".into(), "hollow-core".into()],
                    availability: 16,
                    geometry: Box::new(GeometryRecipe::Slab { width: 1.2, depth: 6.0, thickness: 0.265 }),
                },
            ]
        }
    }
}

/// 🧩️ Every sourcing module known to this crate, in stable order.
pub fn sourcing_modules() -> Vec<Box<dyn SourcingModule>> {
    vec![Box::new(beams::BeamsModule), Box::new(windows::WindowsModule), Box::new(slabs::SlabsModule)]
}

/// 🔎️ Looks up a single module by id.
pub fn module_for(module_id: &str) -> Option<Box<dyn SourcingModule>> {
    sourcing_modules().into_iter().find(|module| module.module_id() == module_id)
}
//#endregion 🔖️Modules

//#region 🔖️GridLayout
/// 🔢️ Places item `index` of `count` total on a `ceil(sqrt(count))`-column grid, centered at the origin,
/// with `cell` spacing between slots — used to lay out the "all objects" 3D grid window.
pub fn grid_placement(count: usize, index: usize, cell: f64) -> (f64, f64) {
    if count == 0 {
        return (0.0, 0.0);
    }
    let columns = (count as f64).sqrt().ceil().max(1.0) as usize;
    let rows = count.div_ceil(columns);
    let column = index % columns;
    let row = index / columns;
    let x = (column as f64 - (columns as f64 - 1.0) * 0.5) * cell;
    let z = (row as f64 - (rows as f64 - 1.0) * 0.5) * cell;
    (x, z)
}

/// 📏️ The uniform scale factor that fits a recipe's largest dimension inside a `cell`-sized grid slot.
pub fn grid_scale(recipe: &GeometryRecipe, cell: f64) -> f64 {
    let extent = bounding_extent(recipe);
    if extent <= 0.0 {
        1.0
    } else {
        cell / extent
    }
}
//#endregion 🔖️GridLayout

//#region 🔖️Fixtures
/// 📄️ The demo-stock example, parsed once from `sourcing_dsl::DEMO_STOCK_TEXT` — the source of truth
/// for every "demo stock" call site (`setActiveExample`, `initial_projection`, tests).
pub fn default_document() -> CurateDocument {
    <CurateDocument as store::DocumentDsl>::parse_dsl(sourcing_dsl::DEMO_STOCK_TEXT).unwrap_or_default()
}

/// 📄️ The empty-curation example, parsed once from `sourcing_dsl::EMPTY_CURATION_TEXT`.
pub fn empty_document() -> CurateDocument {
    <CurateDocument as store::DocumentDsl>::parse_dsl(sourcing_dsl::EMPTY_CURATION_TEXT).unwrap_or_default()
}
//#endregion 🔖️Fixtures

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_document() -> CurateDocument {
        CurateDocument { stock: sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn filtered_stock_matches_query() {
        let document = sample_document();
        let filters = Filters { query: "glulam".into(), ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "beam-glulam-gl24h");
    }

    #[test]
    fn filtered_stock_matches_module() {
        let document = sample_document();
        let filters = Filters { module_ids: vec!["slabs".into()], ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert!(filtered.iter().all(|kind| kind.module_id == "slabs"));
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn filtered_stock_matches_typology_prefix() {
        let document = sample_document();
        let filters = Filters { typology_path: vec!["beams".into(), "steel".into()], ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|kind| kind.typology_path.starts_with(&["beams".to_string(), "steel".to_string()])));
    }

    #[test]
    fn filtered_stock_matches_min_availability() {
        let document = sample_document();
        let filters = Filters { min_availability: 20, ..Default::default() };
        let filtered = filtered_stock(&document, &filters);
        assert!(filtered.iter().all(|kind| kind.availability >= 20));
        assert!(!filtered.is_empty());
    }

    #[test]
    fn sourcing_curate_config_default_matches_the_prior_document_defaults() {
        let config = SourcingCurateConfig::default();
        assert_eq!(config.filters, Filters::default());
        assert_eq!(config.selected_object_id, None);
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn sourcing_curate_io_declares_the_catalog_out_port_alongside_the_implicit_document_ports() {
        let io = sourcing_curate_io();
        assert_eq!(io.document_schema, sourcing::SOURCING_CURATE_SCHEMA);
        let ports = io.all_ports();
        assert_eq!(ports.len(), 3, "document:in, document:out, catalog:out");
        let catalog_out = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog_out.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog_out.media_type.class, semio_framework_plugin::MediaClass::Kit);
        assert_eq!(catalog_out.media_type.form, semio_framework_plugin::MediaForm::Type);
    }

    #[test]
    fn sourcing_catalog_fragment_maps_stock_into_the_puzzle3d_kit_catalog_shape() {
        let document = sample_document();
        let fragment = sourcing_catalog_fragment(&document);
        assert_eq!(fragment["schema"], "manifest");
        let object_kinds = fragment["objectKinds"].as_array().expect("objectKinds array");
        assert_eq!(object_kinds.len(), document.stock.len());
        assert_eq!(object_kinds[0]["id"], document.stock[0].id);
        assert_eq!(object_kinds[0]["meshUrl"], Value::Null);
        assert!(object_kinds[0]["vortices"].as_array().unwrap().is_empty());
        assert!(fragment["vortexKinds"].as_array().unwrap().is_empty());
        assert!(fragment["cableKinds"].as_array().unwrap().is_empty());
        assert!(fragment["attractionKinds"].as_array().unwrap().is_empty());
        assert!(fragment["kindCompatibility"].as_array().unwrap().is_empty());
    }

    #[test]
    fn curate_delta_clamps_to_availability_and_zero_floor() {
        let mut document = sample_document();
        curate_delta(&mut document, "beam-steel-hea160", 100);
        assert_eq!(curated_count(&document, "beam-steel-hea160"), 8);
        curate_delta(&mut document, "beam-steel-hea160", -1000);
        assert_eq!(curated_count(&document, "beam-steel-hea160"), 0);
        assert!(document.curated.is_empty());
    }

    #[test]
    fn curate_delta_unknown_object_is_noop() {
        let mut document = sample_document();
        curate_delta(&mut document, "does-not-exist", 5);
        assert!(document.curated.is_empty());
    }

    #[test]
    fn curate_set_removes_entry_at_zero() {
        let mut document = sample_document();
        curate_set(&mut document, "slab-clt-160", 5);
        assert_eq!(curated_count(&document, "slab-clt-160"), 5);
        curate_set(&mut document, "slab-clt-160", 0);
        assert_eq!(curated_count(&document, "slab-clt-160"), 0);
        assert!(document.curated.is_empty());
    }

    #[test]
    fn typology_contains_and_flatten() {
        let module = beams::BeamsModule;
        let tree = module.typology();
        assert!(typology_contains(&tree, &["beams".into(), "steel".into(), "ipe".into()]));
        assert!(!typology_contains(&tree, &["beams".into(), "concrete".into()]));
        let flattened = typology_flatten(&tree);
        assert!(flattened.iter().any(|(path, _)| path == &vec!["beams".to_string(), "solid-timber".to_string(), "glulam".to_string()]));
    }

    fn assert_mesh_spec_is_valid(spec: &MeshDataSpec) {
        assert!(!spec.positions.is_empty());
        assert_eq!(spec.positions.len() % 3, 0);
        assert_eq!(spec.positions.len(), spec.normals.len());
        assert_eq!(spec.indices.len() % 3, 0);
        let vertex_count = (spec.positions.len() / 3) as u32;
        assert!(spec.indices.iter().all(|&i| i < vertex_count));
    }

    #[test]
    fn box_recipe_produces_valid_mesh() {
        assert_mesh_spec_is_valid(&mesh_spec_for(&GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 }));
    }

    #[test]
    fn frame_recipe_concatenates_four_pieces_into_a_valid_mesh() {
        let spec = mesh_spec_for(&GeometryRecipe::Frame { width: 1.0, height: 1.2, depth: 0.08, profile: 0.08 });
        assert_mesh_spec_is_valid(&spec);
        let single_box = box_mesh_spec(1.0, 0.08, 0.08);
        assert_eq!(spec.positions.len(), single_box.positions.len() * 4);
        assert_eq!(spec.indices.len(), single_box.indices.len() * 4);
    }

    #[test]
    fn grid_placement_centers_around_origin() {
        let positions: Vec<(f64, f64)> = (0..9).map(|i| grid_placement(9, i, 2.0)).collect();
        let sum_x: f64 = positions.iter().map(|(x, _)| x).sum();
        let sum_z: f64 = positions.iter().map(|(_, z)| z).sum();
        assert!(sum_x.abs() < 1e-9);
        assert!(sum_z.abs() < 1e-9);
        let unique: std::collections::HashSet<(i64, i64)> = positions.iter().map(|(x, z)| ((x * 1000.0) as i64, (z * 1000.0) as i64)).collect();
        assert_eq!(unique.len(), 9);
    }

    #[test]
    fn grid_scale_normalizes_to_cell_size() {
        let recipe = GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 };
        let scale = grid_scale(&recipe, 2.0);
        assert!((bounding_extent(&recipe) * scale - 2.0).abs() < 1e-9);
    }

    #[test]
    fn curate_document_dsl_round_trips_sample_and_empty() {
        store::test_support::assert_dsl_round_trip(&sample_document());
        store::test_support::assert_dsl_round_trip(&CurateDocument::default());
        store::test_support::assert_dsl_pack_equivalence(&sample_document());
        store::test_support::assert_dsl_pack_equivalence(&CurateDocument::default());
    }
}
//#endregion 🧪️Tests
