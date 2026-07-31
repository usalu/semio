//! 🏙️ FEM 3D app — document entities (constitutional: general).

use fem_core::Dof;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const FEM_3D_SCHEMA: &str = "fem.3d";

// #region 🔖Document
/// 🔁 fem_3d's own DSL-printable mirror of `fem_core::Dof` — `fem_core::Dof` can't derive
/// `dsl::DslScalar` from outside its own crate (the orphan rule blocks implementing a foreign
/// `dsl::DslField` for a foreign `fem_core::Dof`), so every DOF-typed field in the `Fem3dDocument`
/// grammar (`FemSupport::fixed`, `FemLoad::Nodal::dof`) uses this local tag instead, converting
/// to/from `fem_core::Dof` at the `fem_core::Model`/`Support`/`NodalLoad` boundary (see
/// `resolve_geometry`, `translate_loads`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum FemDof {
    #[dsl(key = "Tx")]
    Tx,
    #[dsl(key = "Ty")]
    Ty,
    #[dsl(key = "Tz")]
    Tz,
    #[dsl(key = "Rx")]
    Rx,
    #[dsl(key = "Ry")]
    Ry,
    #[dsl(key = "Rz")]
    Rz,
}

impl FemDof {
    pub const ALL: [FemDof; 6] = [FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz];
}

impl From<Dof> for FemDof {
    fn from(dof: Dof) -> Self {
        match dof {
            Dof::Tx => FemDof::Tx,
            Dof::Ty => FemDof::Ty,
            Dof::Tz => FemDof::Tz,
            Dof::Rx => FemDof::Rx,
            Dof::Ry => FemDof::Ry,
            Dof::Rz => FemDof::Rz,
        }
    }
}

impl From<FemDof> for Dof {
    fn from(dof: FemDof) -> Self {
        match dof {
            FemDof::Tx => Dof::Tx,
            FemDof::Ty => Dof::Ty,
            FemDof::Tz => Dof::Tz,
            FemDof::Rx => Dof::Rx,
            FemDof::Ry => Dof::Ry,
            FemDof::Rz => Dof::Rz,
        }
    }
}

/// 📍 A structural node: a stable id and a global position, plain SI meters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "node")]
pub struct FemNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🔩 A two-node member: an axial `Bar` or a full 6-DOF `Frame` with a local-axis `roll` angle (radians).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemElement {
    #[serde(rename_all = "camelCase")]
    Bar { id: String, start: String, end: String, material_id: String, section_id: String },
    #[serde(rename_all = "camelCase")]
    Frame { id: String, start: String, end: String, material_id: String, section_id: String, roll: f64 },
}

/// 🪪 A `FemElement`'s stable id, across its `Bar`/`Frame` variants.
pub fn element_id(element: &FemElement) -> &str {
    match element {
        FemElement::Bar { id, .. } | FemElement::Frame { id, .. } => id,
    }
}

/// 🧱 Linear-elastic isotropic material: Young's modulus `e`, shear modulus `g` (Pa), Poisson's ratio
/// `nu` (dimensionless, drives `Tet4` solid elements), and density `rho` (kg/m³, drives self-weight via
/// `Bar3`/`Frame3`/`Tet4`'s `mass()`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "material")]
pub struct FemMaterial {
    pub id: String,
    pub name: String,
    pub e: f64,
    pub g: f64,
    pub nu: f64,
    pub rho: f64,
}

/// 📐 Cross-section properties: area (m²), second moments of area about local y/z (m⁴), torsion constant (m⁴).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "section")]
pub struct FemSection {
    pub id: String,
    pub name: String,
    pub area: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
}

/// 🔒 A support: the subset of a node's DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "support")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    pub fixed: Vec<FemDof>,
}

/// 🏋️ A load — a concentrated nodal force/moment, a member UDL on a `Bar`/`Frame` element, or a normal
/// pressure (Pa) over a meshed `FemSolid`'s top face, simplified as a uniform global `-Z` nodal load
/// (see `area_load_nodal_loads_3d`) — mirrors `fem_2d::FemLoad`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemLoad {
    #[serde(rename_all = "camelCase")]
    Nodal { id: String, node_id: String, dof: FemDof, value: f64 },
    #[serde(rename_all = "camelCase")]
    MemberUdl { id: String, element_id: String, wx: f64, wy: f64, wz: f64 },
    #[serde(rename_all = "camelCase")]
    Area { id: String, solid_id: String, pressure: f64 },
}

/// 🪪 A `FemLoad`'s stable id, across every variant.
pub fn load_id(load: &FemLoad) -> &str {
    match load {
        FemLoad::Nodal { id, .. } | FemLoad::MemberUdl { id, .. } | FemLoad::Area { id, .. } => id,
    }
}

/// 📦 A named set of loads applied together for one analysis run, optionally including self-weight.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "loadcase")]
pub struct FemLoadCase {
    pub id: String,
    pub name: String,
    #[dsl(statements, block)]
    pub loads: Vec<FemLoad>,
    pub self_weight: bool,
}

/// 📦 A linear combination of load cases — case id → factor terms superposed from already-solved
/// case results. `BTreeMap` (not `Vec<(String, f64)>`, which the DSL engine has no primitive for)
/// keyed by case id — duplicates collapse to the last value, which never happened in practice anyway.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "combination")]
pub struct FemCombination {
    pub id: String,
    pub name: String,
    pub terms: BTreeMap<String, f64>,
}

/// ⚙️ Analysis settings: mode/factor counts for modal and buckling analyses, plus a deformation
/// display scale for the UI layer. `deformation_scale` exaggerates the STATIC results view's real
/// (meter-scale) displacements only; modal/buckling mode shapes are dimensionless (mass/Kg-
/// orthonormalized) and the viewer normalizes them to a fixed fraction of the model's own extent
/// instead of using this factor.
// No `#[dsl(keyword = ...)]` here: the only field embedding this type (`Fem3dDocument::analysis`,
// `Fem3dOperation::SetAnalysisSettings::settings`) is itself `#[dsl(block)]`, which already supplies
// the bare leading keyword from the FIELD's own name — an inner keyword too would double it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FemAnalysisSettings {
    pub modal_count: usize,
    pub buckling_count: usize,
    pub deformation_scale: f64,
}

impl Default for FemAnalysisSettings {
    fn default() -> Self {
        Self { modal_count: 3, buckling_count: 3, deformation_scale: 50.0 }
    }
}

/// 🧱 A meshed continuum solid — a polygon footprint (with optional holes) extruded upward from
/// `base_z` by `height` across `layers` equal-height layers, filled with `Tet4` elements at solve time
/// (see `resolve_geometry`) — mirrors `fem_2d::FemRegion`, extended into 3D via `fem_core::mesh`'s
/// extrusion + tet-splitting.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "solid")]
pub struct FemSolid {
    pub id: String,
    pub name: String,
    pub outline: Vec<[f64; 2]>,
    pub holes: Vec<Vec<[f64; 2]>>,
    pub base_z: f64,
    pub height: f64,
    pub layers: usize,
    pub mesh_size: f64,
    pub material_id: String,
}

/// 🎥 Opaque camera state string; the plugin layer owns and interprets its shape. No
/// `#[dsl(keyword = ...)]`: every field embedding this type is itself `#[dsl(block)]` (see
/// `FemAnalysisSettings`'s doc comment above for why that means the keyword stays off here).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FemCamera {
    pub json: String,
}

impl Default for FemCamera {
    fn default() -> Self {
        Self { json: "{}".to_string() }
    }
}

/// 🧾 Persistent fem-3d document — nodes, members, catalogs, supports and load cases. The camera is
/// session-only view state (never a VCS-tracked document field) — see `Fem3dPlayApp::camera` in the
/// ui crate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "fem3d", layout = "lines")]
pub struct Fem3dDocument {
    #[dsl(table)]
    pub nodes: Vec<FemNode>,
    #[dsl(statements, block)]
    pub elements: Vec<FemElement>,
    #[dsl(table)]
    pub materials: Vec<FemMaterial>,
    #[dsl(table)]
    pub sections: Vec<FemSection>,
    #[dsl(table)]
    pub solids: Vec<FemSolid>,
    #[dsl(table)]
    pub supports: Vec<FemSupport>,
    #[dsl(table)]
    pub load_cases: Vec<FemLoadCase>,
    #[dsl(table)]
    pub combinations: Vec<FemCombination>,
    #[dsl(block)]
    pub analysis: FemAnalysisSettings,
}
// #endregion 🔖Document
