//! 📐️ FEM 2D artifact — document entities (constitutional: general).


pub use crate::artifacts::fem2d::schema::snapshot::Fem2dSnapshot;
pub use crate::artifacts::fem2d::schema::mutations::Fem2dMutation;
pub use crate::artifacts::fem2d::schema::diff::Fem2dDiff;

use crate::model::Dof;
use serde::{Deserialize, Serialize};



pub const FEM_2D_SCHEMA: &str = "fem.2d";

// #region 🔖️Document
/// 📍️ A structural node in plan (x, y in meters).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FemNode {
    pub id: String,
    #[dsl(unit = "m")]
    pub x: f64,
    #[dsl(unit = "m")]
    pub y: f64,
}

/// 🔒️ A DOF tag mirroring `crate::model::Dof`'s 6 variants, kept locally: the DSL engine's `DslField`
/// binding can only be derived for a type/trait pair with a local half (orphan rule), and both
/// `Dof` and `DslField` are foreign to this crate. Converted at every `crate::core` boundary via `From`.
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemElement {
    #[serde(rename_all = "camelCase")]
    Bar { id: String, start: String, end: String, material_id: String, section_id: String },
    #[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FemSection {
    pub id: String,
    pub name: String,
    #[dsl(unit = "m2")]
    pub area: f64,
    #[dsl(unit = "m4")]
    pub iy: f64,
}

/// 🔒️ A support: the subset of a node's DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    pub fixed: Vec<FemDof>,
}

/// 🏋️ A load — a concentrated nodal force/moment, a member UDL, or a normal pressure (Pa) over a
/// meshed `FemRegion`, simplified as a uniform global `-Y` nodal load (see `area_load_nodal_loads`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemLoad {
    #[serde(rename_all = "camelCase")]
    Nodal { id: String, node_id: String, dof: FemDof, value: f64 },
    #[serde(rename_all = "camelCase")]
    MemberUdl { id: String, element_id: String, wx: f64, wy: f64 },
    #[serde(rename_all = "camelCase")]
    Area { id: String, region_id: String, pressure: f64 },
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
pub struct FemLoadCase {
    pub id: String,
    pub name: String,
    #[dsl(statements, block)]
    pub loads: Vec<FemLoad>,
    pub self_weight: bool,
}

/// 🟩️ A meshed continuum region — a polygon (with optional holes) filled with `Tri3Cst` elements at
/// solve time (see `crate::fem2d_engine::meshing::build_nodes_and_elements`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FemCombinationTerm {
    pub case_id: String,
    pub factor: f64,
}

/// 🧮️ A linear combination of load cases — terms superposed by `fem2d_solve_all`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FemCombination {
    pub id: String,
    pub name: String,
    pub terms: Vec<FemCombinationTerm>,
}

/// ⚙️ Analysis settings — modal/buckling mode counts and the viewport deformation scale factor.
/// `deformation_scale` exaggerates the STATIC results view's real (meter-scale) displacements only;
/// modal/buckling mode shapes are dimensionless (mass/Kg-orthonormalized) and the viewer normalizes
/// them to a fixed fraction of the model's own extent instead of using this factor.
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

/// 🎥️ The canvas camera (pan/zoom) for the plugin viewport.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FemCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for FemCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 📸️ `Fem2dSnapshot` lives in `📸️snapshot/🧬️schema` — re-exported here for crate consumers.
pub use crate::artifacts::fem2d::schema::Fem2dArtifact;

// #endregion 🔖️Document

// #region 🔖️ArtifactKind
/// 🔌️ The computed-results output artifact kind (`results:out`'s `kind_id`, see
/// `crate::apps::fem2d::fem2d_io`) — the OS-catalog-level resource descriptor for
/// `computation.fem2d`; deliberately a different `media_type` (`Computation`×`Value`) than the PORT's
/// wire-level `Data`×`Value` (see WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE's
/// port recipe). Lifted verbatim out of the pre-migration `fem2d_ui::create_fem2d_app`'s
/// `.artifact_kind(...)` call so the app's manifest can call this instead of inlining the literal.
pub fn computation_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "computation.fem2d".into(),
        name: "FEM 2D Results".into(),
        source_format: "computation.fem2d".into(),
        component_kind: "fem2d-results".into(),
        dimension: "computation".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Value },
        schema: "computation.fem2d".into(),
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
/// plugin `.setup()` callback. `crate::apps::fem2d::config::schema::register_app_schema()` is the one
/// exception, still called from `🏗️fem/🦀️component.rs`'s own narrowed `.setup()`: it registers
/// `Fem2dPlayApp`'s CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has
/// no field for (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's
/// artifact-scoped function set.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.fem2d")
        .schema(crate::artifacts::fem2d::schema::fem2d_artifact_schema_descriptor())
        .inferences([crate::artifacts::fem2d::standards::v1::subsets::any::schema::inferences::fem2d_artifact_inference_descriptor()])
        .composers(crate::artifacts::fem2d::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::fem2d::Fem2dPlayApp>()
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
                    id: "fem.fem2d",
                    extension: Some("fem2d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::fem2d::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::fem2d::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::fem2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::fem2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem.fem2d"),
                },
                dsl::LanguageSpec {
                    id: "fem.fem2d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::fem2d::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::fem2d::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem.fem2d.op"),
                },
                dsl::LanguageSpec {
                    id: "fem.fem2d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::fem2d::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::fem2d::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("fem.fem2d.diff"),
                },
                dsl::LanguageSpec {
                    id: "fem2d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::fem2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::fem2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem2d.pack"),
                },
                dsl::LanguageSpec {
                    id: "fem2d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem2d.spr"),
                },
            ]
        })
        .as_slice()
}
// #endregion 🔖️Register

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::fem2d::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Fem2dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
