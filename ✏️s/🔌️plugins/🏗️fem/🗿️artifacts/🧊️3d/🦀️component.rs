//! 🏙️ FEM 3D artifact — document entity types (constitutional: general).

pub use crate::artifacts::fem3d::schema::diff::Fem3dDiff;
pub use crate::artifacts::fem3d::schema::mutations::Fem3dMutation;
pub use crate::artifacts::fem3d::schema::snapshot::Fem3dSnapshot;

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
///
/// 🔗️ Consolidated (11-type fem2d/fem3d dup consolidation, ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4): this used to be a byte-identical second copy of
/// `fem2d::FemDof`; both dimensions' DOF sets are the same 6 tags with the same `#[dsl(key = ...)]`
/// wire encoding, so this dimension now re-exports the canonical definition instead of duplicating it.
pub use crate::artifacts::fem2d::FemDof;
// `From<Dof> for FemDof` / `From<FemDof> for Dof` already live on `fem2d::FemDof` above — this is now
// the same concrete type, so re-implementing either direction here would be a duplicate trait impl.

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
///
/// 🔗️ Consolidated (11-type fem2d/fem3d dup consolidation, ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4): this used to be a byte-identical second copy of
/// `fem2d::FemAnalysisSettings` (same fields, same `Default`, same missing `#[dsl(keyword = ...)]` for
/// the reason the removed doc comment here used to explain) — re-exported from there now instead.
pub use crate::artifacts::fem2d::FemAnalysisSettings;

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
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.fem3d.standard.v1", "standard", "1", &[], None),
        ("s.fem3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.fem3d.schema.artifact", "schema", "s.fem.fem3d", &[("schema", "s.fem.fem3d")], None),
        ("s.fem3d.inference.artifact", "inference", "s.fem.fem3d.inference", &[("schema", "s.fem.fem3d.inference")], None),
        ("s.fem3d.composer.csv", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.fem3d.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.fem3d.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.fem3d.composer.stl", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.fem3d.composer.obj", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.fem3d.grammar.document", "grammar", "fem.fem3d", &[("grammar", "fem.fem3d")], None),
        ("s.fem3d.grammar.op", "grammar", "fem.fem3d.op", &[("grammar", "fem.fem3d.op")], None),
        ("s.fem3d.grammar.diff", "grammar", "fem.fem3d.diff", &[("grammar", "fem.fem3d.diff")], None),
        ("s.fem3d.grammar.pack", "grammar", "fem3d.pack", &[("grammar", "fem3d.pack")], None),
        ("s.fem3d.grammar.spr", "grammar", "fem3d.spr", &[("grammar", "fem3d.spr")], None),
        ("s.fem3d.codec.document.v1", "codec", "fem.fem3d:fem3d", &[("codec", "fem.fem3d"), ("extension", "fem3d")], None),
        ("s.fem3d.localization.en", "localization", "Finite element model 3D", &[], Some(("en", "Finite element model 3D"))),
        ("s.fem3d.localization.de", "localization", "Finite-Elemente-Modell 3D", &[], Some(("de", "Finite-Elemente-Modell 3D"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.fem3d")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::fem3d::schema::fem3d_artifact_schema_descriptor())
        .inferences([crate::artifacts::fem3d::standards::v1::subsets::any::schema::inferences::fem3d_artifact_inference_descriptor()])
        .composers(crate::artifacts::fem3d::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::fem3d::Fem3dPlayApp>()
        .try_build()
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
    use crate::model::Dof;

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
