//! 📐️ FEM 2D artifact — document entities (constitutional: general).


use crate::model::Dof;
use serde::{Deserialize, Serialize};

pub const FEM_2D_SCHEMA: &str = "fem.2d";

/// 🪪️ W2 packet P7 (26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET): the canonical `ArtifactEditor`/
/// `ArtifactViewer::DIALECT` for this artifact — `artifact_kind` is the 3-part schema id
/// (`#[artifact_schema(id = "s.fem.fem2d")]` on `Fem2dSnapshot`), NOT the 2-part
/// `ArtifactIdentity::parse("s.fem2d")` string `definition()` below uses, and NOT the module-private
/// `FEM2D_DIALECT` in this subset's own `🚪️io/🦀️component.rs` (an older, unrelated 2-part io/composer
/// dialect — different file, different scope, no collision). Lives at the ARTIFACT root so a viewer
/// file can read it without ever importing through the sibling editor module.
pub const FEM2D_DIALECT: semio_framework_plugin::app::Dialect =
    semio_framework_plugin::app::Dialect { artifact_kind: "s.fem.fem2d", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

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
///
/// 🔗️ Canonical shared definition (11-type fem2d/fem3d dup consolidation, ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4): `fem3d`'s `FemDof` used to be a byte-identical
/// second copy of this exact enum; it now re-exports this one (`crate::artifacts::fem2d::FemDof`)
/// instead — see `🗿️artifacts/🧊️3d/🦀️component.rs`.
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
///
/// 🔗️ Canonical shared definition (11-type fem2d/fem3d dup consolidation, ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4): `fem3d`'s `FemAnalysisSettings` used to be a
/// byte-identical second copy of this exact struct; it now re-exports this one
/// (`crate::artifacts::fem2d::FemAnalysisSettings`) instead — see `🗿️artifacts/🧊️3d/🦀️component.rs`.
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
/// `crate::editor::fem2d::fem2d_io`) — the OS-catalog-level resource descriptor for
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
/// plugin `.setup()` callback. `crate::editor::fem2d::config::schema::register_app_schema()` is the one
/// pre-existing exception referenced here (module path only updated for ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET's `apps::fem2d` → `editor::fem2d` rename); it
/// registers `Fem2dPlayApp`'s CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has
/// no field for (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's
/// artifact-scoped function set.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.fem2d.standard.v1", "standard", "1", &[], None),
        ("s.fem2d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.fem2d.schema.artifact", "schema", "s.fem.fem2d", &[("schema", "s.fem.fem2d")], None),
        ("s.fem2d.inference.artifact", "inference", "s.fem.fem2d.inference", &[("schema", "s.fem.fem2d.inference")], None),
        ("s.fem2d.composer.csv", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.fem2d.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.fem2d.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.fem2d.composer.stl", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.fem2d.composer.obj", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.fem2d.grammar.document", "grammar", "fem.fem2d", &[("grammar", "fem.fem2d")], None),
        ("s.fem2d.grammar.op", "grammar", "fem.fem2d.op", &[("grammar", "fem.fem2d.op")], None),
        ("s.fem2d.grammar.diff", "grammar", "fem.fem2d.diff", &[("grammar", "fem.fem2d.diff")], None),
        ("s.fem2d.grammar.pack", "grammar", "fem2d.pack", &[("grammar", "fem2d.pack")], None),
        ("s.fem2d.grammar.spr", "grammar", "fem2d.spr", &[("grammar", "fem2d.spr")], None),
        ("s.fem2d.codec.document.v1", "codec", "fem.fem2d:fem2d", &[("codec", "fem.fem2d"), ("extension", "fem2d")], None),
        ("s.fem2d.localization.en", "localization", "Finite element model 2D", &[], Some(("en", "Finite element model 2D"))),
        ("s.fem2d.localization.de", "localization", "Finite-Elemente-Modell 2D", &[], Some(("de", "Finite-Elemente-Modell 2D"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.fem2d")?);
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
        .schema(crate::artifacts::fem2d::schema::fem2d_artifact_schema_descriptor())
        .inferences([crate::artifacts::fem2d::standards::v1::subsets::any::schema::inferences::fem2d_artifact_inference_descriptor()])
        .composers(crate::artifacts::fem2d::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::fem2d::Fem2dPlayApp>>()
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
