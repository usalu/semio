//! and combinations — the records every fem3d facet (snapshot, diff, mutations, io, editor) names.
//! 🏗️ FEM 3D structural model entities: nodes, members, solids, materials, sections, supports, loads, load cases

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
pub use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::{decode_fixed_dofs,decode_fixed_dofs_controlled,retire_fixed_dofs};
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use std::collections::BTreeMap;

pub const FEM_3D_SCHEMA: &str = "fem.3d";



/// 🔩️ A two-node member: an axial `Bar` or a full 6-DOF `Frame` with a local-axis `roll` angle (radians).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum FemElement {
    #[value(rename_all = "camelCase")]
    Bar { id: String, start: String, end: String, material_id: String, section_id: String },
    #[value(rename_all = "camelCase")]
    Frame { id: String, start: String, end: String, material_id: String, section_id: String, roll: f64 },
}

/// 🪪️ A `FemElement`'s stable id, across its `Bar`/`Frame` variants.
pub fn element_id(element: &FemElement) -> &str {
    match element {
        FemElement::Bar { id, .. } | FemElement::Frame { id, .. } => id,
    }
}



/// 📐️ Cross-section properties: area (m²), second moments of area about local y/z (m⁴), torsion constant (m⁴).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "section")]
pub struct FemSection {
    pub id: String,
    pub name: String,
    pub area: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
}

/// 🔒️ A support: the subset of a node's DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "support")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    #[value(deserialize_with="self::decode_fixed_dofs",deserialize_controlled_with="self::decode_fixed_dofs_controlled",retire_with="self::retire_fixed_dofs")]
    pub fixed: Vec<FemDof>,
}

/// 🏋️ A load — a concentrated nodal force/moment, a member UDL on a `Bar`/`Frame` element, or a normal
/// pressure (Pa) over a meshed `FemSolid`'s top face, simplified as a uniform global `-Z` nodal load
/// (see `semio_s_artifact_fem_3d_engine::fem3d_engine::meshing::area_load_nodal_loads_3d`) — mirrors `fem_2d::FemLoad`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum FemLoad {
    #[value(rename_all = "camelCase")]
    Nodal { id: String, node_id: String, dof: FemDof, value: f64 },
    #[value(rename_all = "camelCase")]
    MemberUdl { id: String, element_id: String, wx: f64, wy: f64, wz: f64 },
    #[value(rename_all = "camelCase")]
    Area { id: String, solid_id: String, pressure: f64 },
}

/// 🪪️ A `FemLoad`'s stable id, across every variant.
pub fn load_id(load: &FemLoad) -> &str {
    match load {
        FemLoad::Nodal { id, .. } | FemLoad::MemberUdl { id, .. } | FemLoad::Area { id, .. } => id,
    }
}

/// 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "loadcase")]
pub struct FemLoadCase {
    pub id: String,
    pub name: String,
    #[dsl(statements, block)]
    pub loads: Vec<FemLoad>,
    pub self_weight: bool,
}

/// 📦️ A linear combination of load cases — case id → factor terms superposed from already-solved
/// case results. `BTreeMap` (not `Vec<(String, f64)>`, which the DSL engine has no primitive for)
/// keyed by case id — duplicates collapse to the last value, which never happened in practice anyway.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "combination")]
pub struct FemCombination {
    pub id: String,
    pub name: String,
    pub terms: BTreeMap<String, f64>,
}


/// 🧭️ The world axis a `FemSolid`'s footprint is extruded along. The footprint plane is the one
/// perpendicular to it and its `(u, v)` coordinates are the two remaining world axes in world order —
/// `X`: `(y, z)`, `Y`: `(x, z)`, `Z`: `(x, y)` — so a wall drawn in elevation (`Y`, footprint `(x, z)`
/// with window holes, extruded through its thickness) and a gable roof (`Y`, a chevron section
/// extruded along the ridge) are the same record a floor slab is (`Z`, footprint `(x, y)` extruded
/// upward). `base_z` is the offset along this axis, `height` the extrusion length.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum FemAxis {
    #[dsl(key = "x")]
    X,
    #[dsl(key = "y")]
    Y,
    #[default]
    #[dsl(key = "z")]
    Z,
}

/// 🌉️ Hand-written like `FemDof`'s: a plain unit-only string enum whose wire shape is the bare key.
impl semio_framework_value::ToValue for FemAxis {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(self.key().to_string())
    }
}
impl semio_framework_value::FromValue for FemAxis {
    fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{
        control.checkpoint()?;
        match value.as_str(){Some("x")=>Ok(Self::X),Some("y")=>Ok(Self::Y),Some("z")=>Ok(Self::Z),_=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"expected a canonical extrusion axis key"))}
    }
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::String(text) => match text.as_str(){"x"=>Ok(Self::X),"y"=>Ok(Self::Y),"z"=>Ok(Self::Z),_=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"expected a canonical extrusion axis key"))},
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

impl FemAxis {
    pub const ALL: [FemAxis; 3] = [FemAxis::X, FemAxis::Y, FemAxis::Z];

    /// 🔤️ The wire key (`x`/`y`/`z`).
    pub fn key(self) -> &'static str {
        match self {
            FemAxis::X => "x",
            FemAxis::Y => "y",
            FemAxis::Z => "z",
        }
    }

    /// 🔤️ Parses the wire key, case-insensitively.
    pub fn from_key(text: &str) -> Option<Self> {
        match text.trim().to_ascii_lowercase().as_str() {
            "x" => Some(FemAxis::X),
            "y" => Some(FemAxis::Y),
            "z" => Some(FemAxis::Z),
            _ => None,
        }
    }

    /// 🌐️ Lifts a footprint point `(u, v)` at extrusion offset `w` into world `(x, y, z)`.
    pub fn to_world(self, u: f64, v: f64, w: f64) -> [f64; 3] {
        match self {
            FemAxis::X => [w, u, v],
            FemAxis::Y => [u, w, v],
            FemAxis::Z => [u, v, w],
        }
    }

    /// 🌐️ Projects a world point onto the footprint plane: `(u, v)` and the extrusion offset `w`.
    pub fn from_world(self, point: [f64; 3]) -> ([f64; 2], f64) {
        match self {
            FemAxis::X => ([point[1], point[2]], point[0]),
            FemAxis::Y => ([point[0], point[2]], point[1]),
            FemAxis::Z => ([point[0], point[1]], point[2]),
        }
    }

    /// 🧭️ The unit extrusion direction in world space.
    pub fn direction(self) -> [f64; 3] {
        self.to_world(0.0, 0.0, 1.0)
    }
}

/// 🧱️ A meshed continuum solid — a polygon footprint (with optional holes) extruded along `axis` from
/// offset `base_z` by `height` across `layers` equal-height layers, filled with `Tet4` elements at
/// solve time (see `semio_s_artifact_fem_3d_engine::fem3d_engine::meshing::resolve_geometry`) — mirrors `fem_2d::FemRegion`,
/// extended into 3D via `semio_s_fem_mesh_derived`'s extrusion + tet-splitting. Footprint coordinates are
/// the world axes `FemAxis` names for the extrusion axis.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
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
    pub axis: FemAxis,
}

pub use semio_s_fem_node::volumetric::FemNode;

pub use semio_s_fem_node::volumetric::FemNodeValue;

pub use semio_s_fem_material::volumetric::FemMaterial;

pub use semio_s_fem_material::volumetric::FemMaterialValue;

#[cfg(test)]
#[path="../../../../../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🏗️structure/🎟️fixed-dofs/🧪️tests/🦀️.rs"]
mod fixed_dof_tests;
