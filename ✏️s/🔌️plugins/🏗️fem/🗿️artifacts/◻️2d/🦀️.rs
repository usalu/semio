//! 📐️ FEM 2D artifact — document entities (constitutional: general).

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/➕️algebra/🦀️.rs"]
pub mod algebra;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs"]
pub mod analyses;
#[cfg(feature = "component-app-assembly")]
#[path = "../../⚙️engine/🖥️app-surface/🦀️.rs"]
pub mod app_surface;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs"]
pub mod elements2d;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs"]
pub mod elements3d;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/➗️formulation/🦀️.rs"]
pub mod formulation;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs"]
pub mod mesh;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️.rs"]
pub mod model;
#[cfg(test)]
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧪️tests/⚙️engine/🦀️.rs"]
pub(crate) mod engine_test_vectors;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs"]
pub mod sparse;
#[path = "."]
pub mod fem2d_engine {
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🗺️mesh-preview/🦀️.rs"]
    pub mod mesh_preview;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🕸️meshing/🦀️.rs"]
    pub mod meshing;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🎵️modal-buckling/🦀️.rs"]
    pub mod modal_buckling;
}

use crate::model::Dof;
use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};

pub const FEM_2D_SCHEMA: &str = "fem.2d";

/// 🪪️ W2 packet P7 (26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET): the canonical `ArtifactEditor`/
/// `ArtifactViewer::DIALECT` for this artifact — `artifact_kind` is the 3-part schema id
/// (`#[artifact_schema(id = "s.fem.fem2d")]` on `Fem2dSnapshot`), NOT the 2-part
/// `ArtifactIdentity::parse("s.fem2d")` string `definition()` below uses, and NOT the module-private
/// `FEM2D_DIALECT` in this subset's own `🚪️io/🦀️.rs` (an older, unrelated 2-part io/composer
/// dialect — different file, different scope, no collision). Lives at the ARTIFACT root so a viewer
/// file can read it without ever importing through the sibling editor module.
pub const FEM2D_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.fem.fem2d", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

// #region 🔖️Document
/// 📍️ A structural node in plan (x, y in meters).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
/// second copy of this exact enum; it now re-exports this one (`crate::FemDof`)
/// instead — see `🗿️artifacts/🧊️3d/🦀️.rs`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar)]
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
                other => Err(ValueError::new(format!("unknown FemDof variant `{other}`"))),
            },
            other => Err(ValueError::new(format!("expected a string, found {other:?}"))),
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    pub fixed: Vec<FemDof>,
}

/// 🏋️ A load — a concentrated nodal force/moment, a member UDL, or a normal pressure (Pa) over a
/// meshed `FemRegion`, simplified as a uniform global `-Y` nodal load (see `area_load_nodal_loads`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct FemCombinationTerm {
    pub case_id: String,
    pub factor: f64,
}

/// 🧮️ A linear combination of load cases — terms superposed by `fem2d_solve_all`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
/// (`crate::FemAnalysisSettings`) instead — see `🗿️artifacts/🧊️3d/🦀️.rs`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

pub use semio_framework_os_kernel::Viewport2d;

/// 📸️ `Fem2dSnapshot` lives in `📸️snapshot/🧬️schema` — re-exported here for crate consumers.
pub use crate::standards::v1::subsets::any::schema::Fem2dArtifact;

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
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into(), "stdio.obj".into(), "stdio.stl".into(), "stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into(), "stdio.obj".into(), "stdio.stl".into(), "stdio.txt".into()],
    }
}
// #endregion 🔖️ArtifactKind

// #region 🔖️Register
/// 🧩️ Application wrappers supported by this artifact declaration.
#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::fem2d::Fem2dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::fem2d::Fem2dViewer>>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::fem2d::Fem2dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::fem2d::Fem2dViewer>>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.fem.fem2d.standard.v1", "standard", "1", &[], None),
        ("s.fem.fem2d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.fem.fem2d.schema.artifact", "schema", "s.fem.fem2d", &[("schema", "s.fem.fem2d")], None),
        ("s.fem.fem2d.inference.artifact", "inference", "s.fem.fem2d.inference", &[("schema", "s.fem.fem2d.inference")], None),
        // 🚪️ One composer claim per dialect `🚪️io/🦀️.rs::io()` registers on the io mechanism: the
        // native `s.fem.fem2d@1/*` plus the six stdio carriers (txt/json/csv/md both ways, stl/obj
        // export), the same shape `🗒️note` and `🧱️block` claim.
        ("s.fem.fem2d.composer.fem2d", "composer", "s.fem.fem2d@1/*", &[("dialect", "s.fem.fem2d@1/*")], None),
        ("s.fem.fem2d.composer.csv", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.fem.fem2d.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.fem.fem2d.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.fem.fem2d.composer.txt", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.fem.fem2d.composer.stl", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.fem.fem2d.composer.obj", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.fem.fem2d.grammar.document", "grammar", "fem.fem2d", &[("grammar", "fem.fem2d")], None),
        ("s.fem.fem2d.grammar.op", "grammar", "fem.fem2d.op", &[("grammar", "fem.fem2d.op")], None),
        ("s.fem.fem2d.grammar.diff", "grammar", "fem.fem2d.diff", &[("grammar", "fem.fem2d.diff")], None),
        ("s.fem.fem2d.grammar.pack", "grammar", "fem2d.pack", &[("grammar", "fem2d.pack")], None),
        ("s.fem.fem2d.grammar.spr", "grammar", "fem2d.spr", &[("grammar", "fem2d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Fem2dPlayApp>>()` derives its
        // codec claim from `Fem2dPlayApp::DOCUMENT_SCHEMA` (= `FEM_2D_SCHEMA`, `🦀️.rs`),
        // which is `"fem.2d"`, not `"fem.fem2d"`.
        ("s.fem.fem2d.codec.document.v1", "codec", "fem.2d:fem2d", &[("codec", "fem.2d"), ("codec-extension", "6:fem.2d:fem2d")], None),
        ("s.fem.fem2d.localization.en", "localization", "Finite element model 2D", &[], Some(("en", "Finite element model 2D"))),
        ("s.fem.fem2d.localization.de", "localization", "Finite-Elemente-Modell 2D", &[], Some(("de", "Finite-Elemente-Modell 2D"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.fem.fem2d")?);
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

/// 🌳️ This artifact's declaration tree root (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
/// RUNTIME`, `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe) —
/// replaces the old `declaration()` (`ArtifactDeclaration::builder(...).schema(...).inferences(...)
/// .composers(...).languages(...).document_codec(...)` chain, deleted outright, no dual channel) as
/// the ONLY registration channel for schema/io/viewer/editor rows. `definition()` (old
/// `ArtifactDefinition`/capability rows, above) is kept per debt D1.
#[cfg(feature = "component-app-assembly")]
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.fem.fem2d").expect("canonical fem2d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `🗒️note`'s own `pilot_languages()` convention.
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "fem.fem2d",
                    extension: Some("fem2d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem.fem2d"),
                },
                dsl::LanguageSpec {
                    id: "fem.fem2d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem.fem2d.op"),
                },
                dsl::LanguageSpec {
                    id: "fem.fem2d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem2d.pack"),
                },
                dsl::LanguageSpec {
                    id: "fem2d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem2d.spr"),
                },
            ]
        })
        .as_slice()
}
// #endregion 🔖️Register

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[cfg(feature = "component-app-assembly")]
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        #[cfg(feature = "component-app-assembly")]
        pub use component::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[cfg(feature = "component-app-assembly")]
                #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🦀️.rs"]
                mod component;
                #[cfg(feature = "component-app-assembly")]
                pub use component::*;

                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        pub use text::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod create_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-node-n3/🦀️.rs"]
                            mod tests_appends_node_n3;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏢️appends-the-canopy-226a20/🦀️.rs"]
                            mod tests_appends_the_canopy_strut_head_node_to_the_steel_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚫️rejects-a-duplicate-eb0df0/🦀️.rs"]
                            mod tests_rejects_a_duplicate_node_id_on_the_steel_frame;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🗑️drops-the-spare-6c285d/🦀️.rs"]
                            mod tests_drops_the_spare_canopy_node_from_the_steel_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/⛔️rejects-a-missing-429801/🦀️.rs"]
                            mod tests_rejects_deleting_a_node_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-6eab3f/🦀️.rs"]
                            mod tests_removes_node_n3_without_cascading_to_its_support;
                        }
                        #[path = "."]
                        pub mod create_element {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends-bar-e2-between-fc1c09/🦀️.rs"]
                            mod tests_appends_bar_e2_between_n2_and_n3;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/📐️braces-the-upper-d96634/🦀️.rs"]
                            mod tests_braces_the_upper_storey_with_a_chs_diagonal;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚫️rejects-a-dangling-b9e64c/🦀️.rs"]
                            mod tests_rejects_an_element_whose_start_node_is_missing;
                        }
                        #[path = "."]
                        pub mod delete_element {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/✂️cuts-the-lower-e4a250/🦀️.rs"]
                            mod tests_cuts_the_lower_storey_bracing_diagonal;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🔗️blocks-udl-a1df8e/🦀️.rs"]
                            mod tests_refuses_to_delete_the_floor_beam_two_member_udls_still_load;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛔️rejects-a-missing-611215/🦀️.rs"]
                            mod tests_rejects_deleting_an_element_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes-bar-e2-and-3c0260/🦀️.rs"]
                            mod tests_removes_bar_e2_and_keeps_its_end_nodes;
                        }
                        #[path = "."]
                        pub mod replace_element {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/♻️converts-beam-e1-into-a-5d21f5/🦀️.rs"]
                            mod tests_converts_beam_e1_into_a_bar_in_place;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️denies-rename-0d46d8/🦀️.rs"]
                            mod tests_refuses_to_rename_the_roof_beam_through_a_replace_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚫️dangling-start-cda887/🦀️.rs"]
                            mod tests_refuses_to_replace_the_brace_onto_a_start_node_that_does_not_exist;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔧️regrades-the-roof-fb20eb/🦀️.rs"]
                            mod tests_regrades_the_roof_beam_onto_the_ipe270_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⛔️rejects-a-missing-bd448c/🦀️.rs"]
                            mod tests_rejects_replacing_an_element_the_steel_frame_never_had;
                        }
                        #[path = "."]
                        pub mod create_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🏗️adds-the-c25-slab-11d8df/🦀️.rs"]
                            mod tests_adds_the_c25_30_material_for_the_ground_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🧱️appends-concrete-c30/🦀️.rs"]
                            mod tests_appends_concrete_c30;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/⚗️denies-poisson-329e35/🦀️.rs"]
                            mod tests_refuses_an_elastomeric_bearing_at_the_incompressible_poisson_limit;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🚫️rejects-a-duplicate-f3220b/🦀️.rs"]
                            mod tests_rejects_a_duplicate_material_id_on_the_steel_frame;
                        }
                        #[path = "."]
                        pub mod delete_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🗑️drops-the-spare-00e964/🦀️.rs"]
                            mod tests_drops_the_unreferenced_s235_material_from_the_steel_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🔗️blocks-in-use-e99619/🦀️.rs"]
                            mod tests_refuses_to_delete_the_s355_grade_seven_members_are_made_of;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/⛔️rejects-a-missing-d5b18f/🦀️.rs"]
                            mod tests_rejects_deleting_a_material_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚫️removes-the-30f7a2/🦀️.rs"]
                            mod tests_removes_the_unreferenced_timber_material;
                        }
                        #[path = "."]
                        pub mod replace_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/📉️cracks-the-c30-b2b220/🦀️.rs"]
                            mod tests_cracks_the_c30_37_stiffness_in_half_for_the_infill_panel;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/⚗️denies-zero-modulus-71e69a/🦀️.rs"]
                            mod tests_refuses_a_concrete_row_whose_modulus_was_left_at_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🪪️denies-rename-a0d7aa/🦀️.rs"]
                            mod tests_refuses_to_regrade_the_concrete_by_renaming_it_through_a_replace;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/⛔️rejects-a-missing-b3adee/🦀️.rs"]
                            mod tests_rejects_replacing_a_material_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🏗️restates-steel-7c22bc/🦀️.rs"]
                            mod tests_restates_steel_as_s355_in_its_original_slot;
                        }
                        #[path = "."]
                        pub mod create_section {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/➕️adds-the-hea220-dfdf34/🦀️.rs"]
                            mod tests_adds_the_hea220_profile_to_the_steel_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/📐️appends-the-ipe300-profile/🦀️.rs"]
                            mod tests_appends_the_ipe300_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/⚗️denies-zero-area-58b5ca/🦀️.rs"]
                            mod tests_refuses_an_ipe_100_profile_whose_area_was_left_at_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚫️rejects-a-duplicate-e91bc7/🦀️.rs"]
                            mod tests_rejects_a_duplicate_section_id_on_the_steel_frame;
                        }
                        #[path = "."]
                        pub mod delete_section {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/✂️drops-the-spare-dcf609/🦀️.rs"]
                            mod tests_drops_the_unreferenced_ipe200_section_from_the_steel_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🔗️blocks-in-use-0a6a3c/🦀️.rs"]
                            mod tests_refuses_to_delete_the_heb_200_profile_four_columns_still_carry;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛔️rejects-a-missing-dbd0a4/🦀️.rs"]
                            mod tests_rejects_deleting_a_section_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-1c235a/🦀️.rs"]
                            mod tests_removes_the_spare_hollow_section;
                        }
                        #[path = "."]
                        pub mod replace_section {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⚗️denies-zero-iy-404e31/🦀️.rs"]
                            mod tests_refuses_a_roof_beam_profile_whose_second_moment_was_left_at_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️denies-rename-1d02dd/🦀️.rs"]
                            mod tests_refuses_to_rename_the_roof_beam_profile_through_a_replace_section;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⛔️rejects-a-missing-b468f4/🦀️.rs"]
                            mod tests_rejects_replacing_a_section_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/💪️stiffens-ipe200-with-5e9c08/🦀️.rs"]
                            mod tests_stiffens_ipe200_with_a_reinforced_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🛠️thickens-the-chs-e235a5/🦀️.rs"]
                            mod tests_thickens_the_chs_brace_wall_to_five_millimetres;
                        }
                        #[path = "."]
                        pub mod create_support {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🛞️adds-a-vertical-6161a1/🦀️.rs"]
                            mod tests_adds_a_vertical_roller_at_node_n2;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🔻️props-the-canopy-b9d719/🦀️.rs"]
                            mod tests_props_the_canopy_tip_on_a_vertical_roller;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🚫️rejects-a-dangling-b0d60b/🦀️.rs"]
                            mod tests_rejects_a_support_on_a_node_the_steel_frame_never_had;
                        }
                        #[path = "."]
                        pub mod delete_support {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🕊️frees-the-roof-tie-44562b/🦀️.rs"]
                            mod tests_frees_the_roof_level_lateral_tie_of_the_steel_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/⛔️rejects-a-missing-23f3c3/🦀️.rs"]
                            mod tests_rejects_deleting_a_support_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🔓️releases-the-82b34f/🦀️.rs"]
                            mod tests_releases_the_roller_at_node_n2;
                        }
                        #[path = "."]
                        pub mod replace_support {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🔩️pins-the-left-base-7891ec/🦀️.rs"]
                            mod tests_pins_the_left_column_base_by_releasing_its_rotation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/👻️dangling-node-98d979/🦀️.rs"]
                            mod tests_refuses_to_move_the_roof_tie_onto_a_node_that_does_not_exist;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🪪️denies-rename-63ec90/🦀️.rs"]
                            mod tests_refuses_to_rename_the_roof_tie_support_through_a_replace_support;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/⛔️rejects-a-missing-afbf6d/🦀️.rs"]
                            mod tests_rejects_replacing_a_support_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🔒️upgrades-the-834e4a/🦀️.rs"]
                            mod tests_upgrades_the_roller_at_n2_to_a_full_fixity;
                        }
                        #[path = "."]
                        pub mod create_region {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🧱️appends-a-solid-d78275/🦀️.rs"]
                            mod tests_appends_a_solid_rectangular_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🏢️infills-the-upper-6cc520/🦀️.rs"]
                            mod tests_infills_the_upper_storey_bay_with_a_concrete_panel;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/📐️denies-two-point-99954a/🦀️.rs"]
                            mod tests_refuses_a_floor_slab_region_outlined_by_only_two_points;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🕳️denies-loose-hole-d9efa1/🦀️.rs"]
                            mod tests_refuses_an_upper_bay_panel_whose_door_opening_lies_outside_it;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🚫️rejects-a-duplicate-11ca0d/🦀️.rs"]
                            mod tests_rejects_a_duplicate_region_id_on_the_steel_frame;
                        }
                        #[path = "."]
                        pub mod delete_region {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🧹️drops-the-spare-460714/🦀️.rs"]
                            mod tests_drops_the_spare_side_panel_region_from_the_steel_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🔗️blocks-in-use-7c7862/🦀️.rs"]
                            mod tests_refuses_to_delete_the_infill_panel_the_wind_case_still_presses_on;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/⛔️rejects-a-missing-a83a6d/🦀️.rs"]
                            mod tests_rejects_deleting_a_region_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🚫️removes-the-slab-and-5b301a/🦀️.rs"]
                            mod tests_removes_the_slab_and_keeps_its_material;
                        }
                        #[path = "."]
                        pub mod replace_region {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪜️punches-a-stair-f7b3b1/🦀️.rs"]
                            mod tests_punches_a_stair_opening_through_the_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/📐️denies-zero-thick-7d805e/🦀️.rs"]
                            mod tests_refuses_an_infill_panel_whose_thickness_was_set_to_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪪️denies-rename-574c91/🦀️.rs"]
                            mod tests_refuses_to_rename_the_infill_panel_through_a_replace_region;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/👻️dangling-mat-7ef81b/🦀️.rs"]
                            mod tests_refuses_to_repour_the_infill_panel_in_a_grade_the_model_lacks;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/⛔️rejects-a-missing-6e0d70/🦀️.rs"]
                            mod tests_rejects_replacing_a_region_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪟️widens-the-window-09a8ec/🦀️.rs"]
                            mod tests_widens_the_window_opening_in_the_infill_wall_panel;
                        }
                        #[path = "."]
                        pub mod create_load_case {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/📍️appends-a-live-case-59118a/🦀️.rs"]
                            mod tests_appends_a_live_case_carrying_one_nodal_load;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/❄️appends-the-snow-4c007c/🦀️.rs"]
                            mod tests_appends_the_snow_case_over_the_roof_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🚫️rejects-a-dangling-4904f4/🦀️.rs"]
                            mod tests_rejects_a_load_case_whose_udl_names_a_missing_element;
                        }
                        #[path = "."]
                        pub mod delete_load_case {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🗑️drops-the-spare-49435f/🦀️.rs"]
                            mod tests_drops_the_spare_snow_case_with_its_single_load;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🔗️blocks-in-use-7cdc5f/🦀️.rs"]
                            mod tests_refuses_to_delete_the_dead_case_three_combinations_still_weight;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/⛔️rejects-a-missing-79ed15/🦀️.rs"]
                            mod tests_rejects_deleting_a_load_case_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚫️removes-the-live-06415d/🦀️.rs"]
                            mod tests_removes_the_live_case_together_with_its_loads;
                        }
                        #[path = "."]
                        pub mod add_load {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/📏️appends-a-member-udl-to-the-dead-case/🦀️.rs"]
                            mod tests_appends_a_member_udl_to_the_dead_case;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/💨️pushes-a-wind-load-5c3f1e/🦀️.rs"]
                            mod tests_pushes_a_wind_point_load_onto_the_first_floor_of_the_steel_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/👻️dangling-node-8d113b/🦀️.rs"]
                            mod tests_refuses_to_push_a_wind_load_at_a_node_the_frame_does_not_have;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🚫️rejects-a-missing-4271bc/🦀️.rs"]
                            mod tests_rejects_adding_a_load_to_a_load_case_that_does_not_exist;
                        }
                        #[path = "."]
                        pub mod remove_load {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/⛔️rejects-a-missing-1a8a80/🦀️.rs"]
                            mod tests_rejects_removing_a_load_the_dead_case_never_carried;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/➖️strips-the-trailing-member-133914/🦀️.rs"]
                            mod tests_strips_the_trailing_member_udl_from_the_dead_case;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/✂️strips-the-roof-udl-0c1b3c/🦀️.rs"]
                            mod tests_strips_the_trailing_roof_udl_from_the_dead_case;
                        }
                        #[path = "."]
                        pub mod change_load_case_self_weight {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🔁️keeps-self-weight-ff696b/🦀️.rs"]
                            mod tests_keeps_self_weight_on_for_the_dead_case;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/⚖️switches-self-abbff2/🦀️.rs"]
                            mod tests_switches_self_weight_on_for_the_dead_case;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🏋️switches-self-5977a5/🦀️.rs"]
                            mod tests_switches_self_weight_on_for_the_imposed_case;
                        }
                        #[path = "."]
                        pub mod create_combination {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🔗️appends-an-uls-0c18bb/🦀️.rs"]
                            mod tests_appends_an_uls_combination_over_both_cases;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/➕️appends-the-6-10a-eefe01/🦀️.rs"]
                            mod tests_appends_the_six_ten_a_combination_over_dead_and_snow;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🚫️rejects-a-dangling-2aa3ea/🦀️.rs"]
                            mod tests_rejects_a_combination_term_naming_a_load_case_that_is_absent;
                        }
                        #[path = "."]
                        pub mod delete_combination {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🗑️drops-the-spare-60fda7/🦀️.rs"]
                            mod tests_drops_the_spare_uls_combination_from_the_steel_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🔗️blocks-in-use-0b898b/🦀️.rs"]
                            mod tests_refuses_to_delete_an_uls_combination_a_design_envelope_nests;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/⛔️rejects-a-missing-d4bc03/🦀️.rs"]
                            mod tests_rejects_deleting_a_combination_the_steel_frame_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/✂️removes-the-uls-438c0c/🦀️.rs"]
                            mod tests_removes_the_uls_combination_and_keeps_both_cases;
                        }
                        #[path = "."]
                        pub mod update_analysis_settings {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-modal-3fbb1a/🦀️.rs"]
                            mod tests_doubles_the_modal_count_and_halves_the_deformation_scale;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔁️keeps-the-analysis-196e4a/🦀️.rs"]
                            mod tests_keeps_the_analysis_settings_exactly_as_they_are;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🎚️raises-the-mode-908c2b/🦀️.rs"]
                            mod tests_raises_the_mode_counts_and_tightens_the_deformation_scale;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🚫️denies-zero-modes-babc1d/🦀️.rs"]
                            mod tests_refuses_an_analysis_configured_to_extract_zero_modal_modes;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod stl {
                                    #[path = "."]
                                    pub mod v_ascii {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod obj {
                                    #[path = "."]
                                    pub mod v3_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::schema::diff::Fem2dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod fem2d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }





        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs"]
        mod wasm;

        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs"]
        pub mod session;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🏋️add-area-load/🦀️.rs"]
            pub mod add_area_load;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/➖️add-bar/🦀️.rs"]
            pub mod add_bar;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🌉️add-beam/🦀️.rs"]
            pub mod add_beam;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🔗️add-combination/🦀️.rs"]
            pub mod add_combination;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📋️add-load-case/🦀️.rs"]
            pub mod add_load_case;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧱️add-material/🦀️.rs"]
            pub mod add_material;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📏️add-member-udl/🦀️.rs"]
            pub mod add_member_udl;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs"]
            pub mod add_nodal_load;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚪️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗺️add-region/🦀️.rs"]
            pub mod add_region;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📐️add-section/🦀️.rs"]
            pub mod add_section;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🛡️add-support/🦀️.rs"]
            pub mod add_support;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs"]
            pub mod remove_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧮️set-analysis-settings/🦀️.rs"]
            pub mod set_analysis_settings;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/👁️set-result-display/🦀️.rs"]
            pub mod set_result_display;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚖️set-self-weight/🦀️.rs"]
            pub mod set_self_weight;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs"]
                    pub mod model;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"]
                    pub mod results;
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod fem2d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs"]
                    pub mod model;
                }
            }
        }
    }
}
