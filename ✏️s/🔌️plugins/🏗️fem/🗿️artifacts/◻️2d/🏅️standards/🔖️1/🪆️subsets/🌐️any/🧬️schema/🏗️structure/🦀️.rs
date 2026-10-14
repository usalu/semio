//! 🏗️ FEM 2D structural model entities: nodes, members, materials, sections, supports, loads, load cases, regions,
//! combinations and analysis settings — the records every fem2d facet (snapshot, diff, mutations, io, editor) names.

use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
use semio_s_fem_model::Dof;

pub const FEM_2D_SCHEMA: &str = "fem.2d";

/// 📍️ A structural node in plan (x, y in meters).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FemNode {
    pub id: String,
    #[dsl(unit = "m")]
    pub x: f64,
    #[dsl(unit = "m")]
    pub y: f64,
}

/// 🔒️ A DOF tag mirroring `semio_s_fem_model::Dof`'s 6 variants, kept locally: the DSL engine's `DslField`
/// binding can only be derived for a type/trait pair with a local half (orphan rule), and both
/// `Dof` and `DslField` are foreign to this crate. Converted at every `crate::core` boundary via `From`.
///
/// 🔗️ Canonical shared definition (11-type fem2d/fem3d dup consolidation, ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4): `fem3d`'s `FemDof` used to be a byte-identical
/// second copy of this exact enum; it now re-exports this one (`crate::standards::v1::subsets::any::schema::structure::FemDof`)
/// instead — see `🗿️artifacts/🧊️3d/🦀️.rs`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]`'s enum path only supports
/// internally-tagged (`#[value(tag = "…")]`) representations, but `FemDof` is a plain unit-only
/// "string enum" — the value codec's untagged bare-string representation for an enum with no
/// `#[value(...)]` attribute at all — so the wire shape here is just the bare variant name.
impl ToValue for FemDof {
    fn to_value(&self) -> DslValue {
        let name = match self {
            FemDof::Tx => "Tx",
            FemDof::Ty => "Ty",
            FemDof::Tz => "Tz",
            FemDof::Rx => "Rx",
            FemDof::Ry => "Ry",
            FemDof::Rz => "Rz",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for FemDof {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "Tx" => Ok(FemDof::Tx),
                "Ty" => Ok(FemDof::Ty),
                "Tz" => Ok(FemDof::Tz),
                "Rx" => Ok(FemDof::Rx),
                "Ry" => Ok(FemDof::Ry),
                "Rz" => Ok(FemDof::Rz),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown FemDof variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

impl FemDof {
    pub const ALL: [FemDof; 6] = [FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz];
}

impl From<FemDof> for Dof {
    fn from(value: FemDof) -> Self {
        match value {
            FemDof::Tx => Dof::Tx,
            FemDof::Ty => Dof::Ty,
            FemDof::Tz => Dof::Tz,
            FemDof::Rx => Dof::Rx,
            FemDof::Ry => Dof::Ry,
            FemDof::Rz => Dof::Rz,
        }
    }
}

impl From<Dof> for FemDof {
    fn from(value: Dof) -> Self {
        match value {
            Dof::Tx => FemDof::Tx,
            Dof::Ty => FemDof::Ty,
            Dof::Tz => FemDof::Tz,
            Dof::Rx => FemDof::Rx,
            Dof::Ry => FemDof::Ry,
            Dof::Rz => FemDof::Rz,
        }
    }
}

/// 🔩️ A 2-node structural member — axial-only `Bar` or axial+bending `Beam`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum FemElement {
    #[value(rename_all = "camelCase")]
    Bar { id: String, start: String, end: String, material_id: String, section_id: String },
    #[value(rename_all = "camelCase")]
    Beam { id: String, start: String, end: String, material_id: String, section_id: String },
}

/// 🪪️ A `FemElement`'s stable id, across both variants.
pub fn element_id(element: &FemElement) -> &str {
    match element {
        FemElement::Bar { id, .. } | FemElement::Beam { id, .. } => id,
    }
}

/// 🧱️ An isotropic material — Young's modulus `e` in Pascals, Poisson's ratio `nu`, density `rho`
/// in kg/m³ (the latter two required for continuum `FemRegion` elements and self-weight).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FemMaterial {
    pub id: String,
    pub name: String,
    #[dsl(unit = "Pa")]
    pub e: f64,
    pub nu: f64,
    #[dsl(unit = "kg/m3")]
    pub rho: f64,
}

/// 📏️ A cross-section — area in m², strong-axis moment of inertia `iy` in m⁴.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FemSection {
    pub id: String,
    pub name: String,
    #[dsl(unit = "m2")]
    pub area: f64,
    #[dsl(unit = "m4")]
    pub iy: f64,
}

/// 🔒️ A support: the subset of a node's DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    pub fixed: Vec<FemDof>,
}

/// 🏋️ A load — a concentrated nodal force/moment, a member UDL, or a normal pressure (Pa) over a
/// meshed `FemRegion`, simplified as a uniform global `-Y` nodal load (see `area_load_nodal_loads`).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum FemLoad {
    #[value(rename_all = "camelCase")]
    Nodal { id: String, node_id: String, dof: FemDof, value: f64 },
    #[value(rename_all = "camelCase")]
    MemberUdl { id: String, element_id: String, wx: f64, wy: f64 },
    #[value(rename_all = "camelCase")]
    Area { id: String, region_id: String, pressure: f64 },
}

/// 🪪️ A `FemLoad`'s stable id, across every variant.
pub fn load_id(load: &FemLoad) -> &str {
    match load {
        FemLoad::Nodal { id, .. } | FemLoad::MemberUdl { id, .. } | FemLoad::Area { id, .. } => id,
    }
}

/// 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FemLoadCase {
    pub id: String,
    pub name: String,
    #[dsl(statements, block)]
    pub loads: Vec<FemLoad>,
    pub self_weight: bool,
}

/// 🟩️ A meshed continuum region — a polygon (with optional holes) filled with `Tri3Cst` elements at
/// solve time (see `crate::fem2d_engine::meshing::build_nodes_and_elements`).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FemRegion {
    pub id: String,
    pub name: String,
    pub outline: Vec<[f64; 2]>,
    pub holes: Vec<Vec<[f64; 2]>>,
    pub thickness: f64,
    pub material_id: String,
    pub mesh_size: f64,
}

/// 🔗️ One combination term — a referenced load case (or nested combination) id and its scale
/// factor. A named record instead of a bare `(String, f64)` tuple: the DSL engine's `DslField`
/// binding has no impl for raw Rust tuples, only for named types deriving `DslRecord`/`DslScalar`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FemCombinationTerm {
    pub case_id: String,
    pub factor: f64,
}

/// 🧮️ A linear combination of load cases — terms superposed by `fem2d_solve_all`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FemCombination {
    pub id: String,
    pub name: String,
    pub terms: Vec<FemCombinationTerm>,
}

/// ⚙️ Analysis settings — modal/buckling mode counts and the viewport deformation scale factor.
/// `deformation_scale` exaggerates the STATIC results view's real (meter-scale) displacements only;
/// modal/buckling mode shapes are dimensionless (mass/Kg-orthonormalized) and the viewer normalizes
/// them to a fixed fraction of the model's own extent instead of using this factor.
///
/// 🔗️ Canonical shared definition (11-type fem2d/fem3d dup consolidation, ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4): `fem3d`'s `FemAnalysisSettings` used to be a
/// byte-identical second copy of this exact struct; it now re-exports this one
/// (`crate::standards::v1::subsets::any::schema::structure::FemAnalysisSettings`) instead — see `🗿️artifacts/🧊️3d/🦀️.rs`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
