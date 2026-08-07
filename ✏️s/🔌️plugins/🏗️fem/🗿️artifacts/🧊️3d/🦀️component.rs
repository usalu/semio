//! 🏙️ FEM 3D artifact — document entity types (constitutional: general).

use crate::core::Dof;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const FEM_3D_SCHEMA: &str = "fem.3d";

// #region 🔖️Document
/// 🔁️ fem_3d's own DSL-printable mirror of `crate::core::Dof` — `crate::core::Dof` can't derive
/// `dsl::DslScalar` from outside its own defining module the same way a foreign crate couldn't (the
/// orphan rule blocks implementing a foreign `dsl::DslField` for it from here), so every DOF-typed
/// field in the `Fem3dDocument` grammar (`FemSupport::fixed`, `FemLoad::Nodal::dof`) uses this local tag
/// instead, converting to/from `crate::core::Dof` at the `crate::core::Model`/`Support`/`NodalLoad`
/// boundary (see `crate::artifacts::fem3d::engine::meshing::resolve_geometry`, `translate_loads`).
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

/// 📍️ A structural node: a stable id and a global position, plain SI meters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "node")]
pub struct FemNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🔩️ A two-node member: an axial `Bar` or a full 6-DOF `Frame` with a local-axis `roll` angle (radians).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemElement {
    #[serde(rename_all = "camelCase")]
    Bar { id: String, start: String, end: String, material_id: String, section_id: String },
    #[serde(rename_all = "camelCase")]
    Frame { id: String, start: String, end: String, material_id: String, section_id: String, roll: f64 },
}

/// 🪪️ A `FemElement`'s stable id, across its `Bar`/`Frame` variants.
pub fn element_id(element: &FemElement) -> &str {
    match element {
        FemElement::Bar { id, .. } | FemElement::Frame { id, .. } => id,
    }
}

/// 🧱️ Linear-elastic isotropic material: Young's modulus `e`, shear modulus `g` (Pa), Poisson's ratio
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

/// 📐️ Cross-section properties: area (m²), second moments of area about local y/z (m⁴), torsion constant (m⁴).
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

/// 🔒️ A support: the subset of a node's DOFs restrained to zero displacement.
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
/// (see `crate::artifacts::fem3d::engine::meshing::area_load_nodal_loads_3d`) — mirrors `fem_2d::FemLoad`.
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

/// 🪪️ A `FemLoad`'s stable id, across every variant.
pub fn load_id(load: &FemLoad) -> &str {
    match load {
        FemLoad::Nodal { id, .. } | FemLoad::MemberUdl { id, .. } | FemLoad::Area { id, .. } => id,
    }
}

/// 📦️ A named set of loads applied together for one analysis run, optionally including self-weight.
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

/// 📦️ A linear combination of load cases — case id → factor terms superposed from already-solved
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

/// 🧱️ A meshed continuum solid — a polygon footprint (with optional holes) extruded upward from
/// `base_z` by `height` across `layers` equal-height layers, filled with `Tet4` elements at solve time
/// (see `crate::artifacts::fem3d::engine::meshing::resolve_geometry`) — mirrors `fem_2d::FemRegion`,
/// extended into 3D via `crate::core::mesh`'s extrusion + tet-splitting.
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

/// 🎥️ Opaque camera state string; the plugin layer owns and interprets its shape. No
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

/// 🧾️ Persistent fem-3d document — nodes, members, catalogs, supports and load cases. The camera is
/// session-only view state (never a VCS-tracked document field) — see `Fem3dConfig::camera` in the
/// app's `🎚️config`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "fem.fem3d", layout = "lines")]
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

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for Fem3dDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for Fem3dDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec

// #endregion 🔖️Document

// #region 🔖️ArtifactKind
/// 🏷️ The `computation.fem3d` artifact kind — every load case/combination's solved
/// `crate::core::StaticResult`, pinned to this kind by the `results:out` media port (see
/// `crate::artifacts::fem3d::engine::fem3d_results_out_port`) and produced by
/// `crate::apps::fem3d::Fem3dPlayApp::export_media`. Lifted verbatim out of the old ui crate's
/// `create_fem3d_app`'s inline `.artifact_kind(...)` call so the app's manifest can reference it by name.
pub fn computation_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "computation.fem3d".into(),
        name: "FEM 3D Results".into(),
        source_format: "computation.fem3d".into(),
        component_kind: "fem3d-results".into(),
        dimension: "computation".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Value },
        schema: "computation.fem3d".into(),
        export_formats: vec![],
        import_formats: vec![],
    }
}
// #endregion 🔖️ArtifactKind

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fem_dof_round_trips_through_core_dof() {
        for dof in FemDof::ALL {
            assert_eq!(FemDof::from(Dof::from(dof)), dof);
        }
    }

    #[test]
    fn fem_analysis_settings_default_matches_pre_migration_values() {
        let settings = FemAnalysisSettings::default();
        assert_eq!(settings.modal_count, 3);
        assert_eq!(settings.buckling_count, 3);
        assert_eq!(settings.deformation_scale, 50.0);
    }

    #[test]
    fn fem_camera_default_is_empty_json_object() {
        assert_eq!(FemCamera::default().json, "{}");
    }

    #[test]
    fn computation_artifact_kind_matches_computation_fem3d() {
        let kind = computation_artifact_kind();
        assert_eq!(kind.id, "computation.fem3d");
        assert_eq!(kind.component_kind, "fem3d-results");
        assert_eq!(kind.schema, "computation.fem3d");
    }
}
// #endregion 🧪️Tests
