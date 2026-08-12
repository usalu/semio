//! 🏙️ FEM 3D artifact — document entity types (constitutional: general).


pub use crate::artifacts::fem3d::schema::snapshot::Fem3dSnapshot;
pub use crate::artifacts::fem3d::schema::mutations::Fem3dMutation;
pub use crate::artifacts::fem3d::schema::diff::Fem3dDiff;

use crate::model::Dof;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;



pub const FEM_3D_SCHEMA: &str = "fem.3d";

// #region 🔖️Document
/// 🔁️ fem_3d's own DSL-printable mirror of `crate::model::Dof` — `crate::model::Dof` can't derive
/// `dsl::DslScalar` from outside its own defining module the same way a foreign crate couldn't (the
/// orphan rule blocks implementing a foreign `dsl::DslField` for it from here), so every DOF-typed
/// field in the `Fem3dSnapshot` grammar (`FemSupport::fixed`, `FemLoad::Nodal::dof`) uses this local tag
/// instead, converting to/from `crate::model::Dof` at the `crate::model::Model`/`Support`/`NodalLoad`
/// boundary (see `crate::fem3d_engine::meshing::resolve_geometry`, `translate_loads`).
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
/// (see `crate::fem3d_engine::meshing::area_load_nodal_loads_3d`) — mirrors `fem_2d::FemLoad`.
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
// No `#[dsl(keyword = ...)]` here: the only fields embedding this type (`Fem3dSnapshot::analysis`,
// `mutations::update_analysis_settings::mutation::UpdateAnalysisSettings::settings`) are themselves
// `#[dsl(block)]`, which already supplies the bare leading keyword from the FIELD's own name — an
// inner keyword too would double it.
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
/// (see `crate::fem3d_engine::meshing::resolve_geometry`) — mirrors `fem_2d::FemRegion`,
/// extended into 3D via `crate::model::mesh`'s extrusion + tet-splitting.
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

/// 📸️ `Fem3dSnapshot` lives in `📸️snapshot/🧬️schema` — re-exported here for crate consumers.
pub use crate::artifacts::fem3d::schema::Fem3dArtifact;

// #endregion 🔖️Document

// #region 🔖️ArtifactKind
/// 🏷️ The `computation.fem3d` artifact kind — every load case/combination's solved
/// `crate::model::StaticResult`, pinned to this kind by the `results:out` media port (see
/// `crate::apps::fem3d::fem3d_results_out_port`) and produced by
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
            export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md"],
    }
}
// #endregion 🔖️ArtifactKind

// #region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from a
/// plugin `.setup()` callback. `crate::apps::fem3d::config::schema::register_app_schema()` is the one
/// exception, still called from `🏗️fem/🦀️component.rs`'s own narrowed `.setup()`: it registers
/// `Fem3dPlayApp`'s CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has
/// no field for (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's
/// artifact-scoped function set.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.fem3d")
        .schema(crate::artifacts::fem3d::schema::fem3d_artifact_schema_descriptor())
        .inferences([crate::artifacts::fem3d::standards::v1::subsets::any::schema::inferences::fem3d_artifact_inference_descriptor()])
        .composers(crate::artifacts::fem3d::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::fem3d::Fem3dPlayApp>()
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `🗒️note`'s own `pilot_languages()` convention.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "fem.fem3d",
                    extension: Some("fem3d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::fem3d::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::fem3d::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("fem.fem3d"),
                },
                dsl::LanguageSpec {
                    id: "fem.fem3d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::fem3d::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::fem3d::op::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("fem.fem3d.op"),
                },
                dsl::LanguageSpec {
                    id: "fem.fem3d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::fem3d::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::fem3d::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("fem.fem3d.diff"),
                },
                dsl::LanguageSpec {
                    id: "fem3d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::fem3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::fem3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem3d.pack"),
                },
                dsl::LanguageSpec {
                    id: "fem3d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::fem3d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::fem3d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem3d.spr"),
                },
            ]
        })
        .as_slice()
}
// #endregion 🔖️Register

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
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::fem3d::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Fem3dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
