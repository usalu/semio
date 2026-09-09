//! 🏙️ FEM 3D artifact — document entity types (constitutional: general).

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[cfg(feature = "component-app-assembly")]
pub use semio_s_artifact_fem_2d::app_surface;
pub use semio_s_artifact_fem_2d::{algebra, analyses, elements2d, elements3d, formulation, mesh, model, sparse};
#[path = "."]
pub mod fem3d_engine {
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🗺️mesh-preview/🦀️.rs"]
    pub mod mesh_preview;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🕸️meshing/🦀️.rs"]
    pub mod meshing;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🎵️modal-buckling/🦀️.rs"]
    pub mod modal_buckling;
}

use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

pub const FEM_3D_SCHEMA: &str = "fem.3d";

/// 🪪️ W2 packet P7 (26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET): the canonical `ArtifactEditor`/
/// `ArtifactViewer::DIALECT` for this artifact — `artifact_kind` is the 3-part schema id
/// (`#[artifact_schema(id = "s.fem.fem3d")]` on `Fem3dSnapshot`), NOT the 2-part
/// `ArtifactIdentity::parse("s.fem3d")` string `definition()` below uses, and NOT the module-private
/// `FEM3D_DIALECT` in this subset's own `🚪️io/🦀️.rs` (an older, unrelated 2-part io/composer
/// dialect — different file, different scope, no collision). Lives at the ARTIFACT root so a viewer
/// file can read it without ever importing through the sibling editor module.
pub const FEM3D_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.fem.fem3d", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

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
pub use semio_s_artifact_fem_2d::FemDof;
// `From<Dof> for FemDof` / `From<FemDof> for Dof` already live on `fem2d::FemDof` above — this is now
// the same concrete type, so re-implementing either direction here would be a duplicate trait impl.

/// 📍️ A structural node: a stable id and a global position, plain SI meters.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "node")]
pub struct FemNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🔩️ A two-node member: an axial `Bar` or a full 6-DOF `Frame` with a local-axis `roll` angle (radians).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum)]
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

/// 🧱️ Linear-elastic isotropic material: Young's modulus `e`, shear modulus `g` (Pa), Poisson's ratio
/// `nu` (dimensionless, drives `Tet4` solid elements), and density `rho` (kg/m³, drives self-weight via
/// `Bar3`/`Frame3`/`Tet4`'s `mass()`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "support")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    pub fixed: Vec<FemDof>,
}

/// 🏋️ A load — a concentrated nodal force/moment, a member UDL on a `Bar`/`Frame` element, or a normal
/// pressure (Pa) over a meshed `FemSolid`'s top face, simplified as a uniform global `-Z` nodal load
/// (see `crate::fem3d_engine::meshing::area_load_nodal_loads_3d`) — mirrors `fem_2d::FemLoad`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
pub use semio_s_artifact_fem_2d::FemAnalysisSettings;

/// 🧱️ A meshed continuum solid — a polygon footprint (with optional holes) extruded upward from
/// `base_z` by `height` across `layers` equal-height layers, filled with `Tet4` elements at solve time
/// (see `crate::fem3d_engine::meshing::resolve_geometry`) — mirrors `fem_2d::FemRegion`,
/// extended into 3D via `crate::model::mesh`'s extrusion + tet-splitting.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
}

/// 🎥️ Opaque camera state string; the plugin layer owns and interprets its shape. No
/// `#[dsl(keyword = ...)]`: every field embedding this type is itself `#[dsl(block)]` (see
/// `FemAnalysisSettings`'s doc comment above for why that means the keyword stays off here).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct FemCamera {
    pub json: String,
}

impl Default for FemCamera {
    fn default() -> Self {
        Self { json: "{}".to_string() }
    }
}

/// 📸️ `Fem3dSnapshot` lives in `📸️snapshot/🧬️schema` — re-exported here for crate consumers.
pub use crate::standards::v1::subsets::any::schema::Fem3dArtifact;

// #endregion 🔖️Document

// #region 🔖️ArtifactKind
/// 🏷️ The `computation.fem3d` artifact kind — every load case/combination's solved
/// `crate::model::StaticResult`, pinned to this kind by the `results:out` media port (see
/// `crate::editor::fem3d::fem3d_results_out_port`) and produced by
/// `crate::editor::fem3d::Fem3dPlayApp::export_media`. Lifted verbatim out of the old ui crate's
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
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into(), "stdio.obj".into(), "stdio.stl".into(), "stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into(), "stdio.obj".into(), "stdio.stl".into(), "stdio.txt".into()],
    }
}
// #endregion 🔖️ArtifactKind

// #region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from a
/// plugin `.setup()` callback. `crate::editor::fem3d::config::schema::register_app_schema()` is the one
/// pre-existing exception referenced here (module path only updated for ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET's `apps::fem3d` → `editor::fem3d` rename); it
/// registers
/// `Fem3dPlayApp`'s CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has
/// no field for (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's
/// artifact-scoped function set.
#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::fem3d::Fem3dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::fem3d::Fem3dViewer>>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::fem3d::Fem3dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::fem3d::Fem3dViewer>>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.fem.fem3d.standard.v1", "standard", "1", &[], None),
        ("s.fem.fem3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.fem.fem3d.schema.artifact", "schema", "s.fem.fem3d", &[("schema", "s.fem.fem3d")], None),
        ("s.fem.fem3d.inference.artifact", "inference", "s.fem.fem3d.inference", &[("schema", "s.fem.fem3d.inference")], None),
        // 🚪️ One composer claim per dialect `🚪️io/🦀️.rs::io()` registers on the io mechanism: the
        // native `s.fem.fem3d@1/*` plus the six stdio carriers (txt/json/csv/md both ways, stl/obj
        // export), the same shape `🗒️note` and `🧱️block` claim.
        ("s.fem.fem3d.composer.fem3d", "composer", "s.fem.fem3d@1/*", &[("dialect", "s.fem.fem3d@1/*")], None),
        ("s.fem.fem3d.composer.csv", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.fem.fem3d.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.fem.fem3d.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.fem.fem3d.composer.txt", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.fem.fem3d.composer.stl", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.fem.fem3d.composer.obj", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.fem.fem3d.grammar.document", "grammar", "fem.fem3d", &[("grammar", "fem.fem3d")], None),
        ("s.fem.fem3d.grammar.op", "grammar", "fem.fem3d.op", &[("grammar", "fem.fem3d.op")], None),
        ("s.fem.fem3d.grammar.diff", "grammar", "fem.fem3d.diff", &[("grammar", "fem.fem3d.diff")], None),
        ("s.fem.fem3d.grammar.pack", "grammar", "fem3d.pack", &[("grammar", "fem3d.pack")], None),
        ("s.fem.fem3d.grammar.spr", "grammar", "fem3d.spr", &[("grammar", "fem3d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Fem3dPlayApp>>()` derives its
        // codec claim from `Fem3dPlayApp::DOCUMENT_SCHEMA` (= `FEM_3D_SCHEMA`, `🦀️.rs`),
        // which is `"fem.3d"`, not `"fem.fem3d"`.
        ("s.fem.fem3d.codec.document.v1", "codec", "fem.3d:fem3d", &[("codec", "fem.3d"), ("codec-extension", "6:fem.3d:fem3d")], None),
        ("s.fem.fem3d.localization.en", "localization", "Finite element model 3D", &[], Some(("en", "Finite element model 3D"))),
        ("s.fem.fem3d.localization.de", "localization", "Finite-Elemente-Modell 3D", &[], Some(("de", "Finite-Elemente-Modell 3D"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.fem.fem3d")?);
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
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.fem.fem3d").expect("canonical fem3d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
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
                    id: "fem.fem3d",
                    extension: Some("fem3d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("fem.fem3d"),
                },
                dsl::LanguageSpec {
                    id: "fem.fem3d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("fem.fem3d.op"),
                },
                dsl::LanguageSpec {
                    id: "fem.fem3d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem3d.pack"),
                },
                dsl::LanguageSpec {
                    id: "fem3d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("fem3d.spr"),
                },
            ]
        })
        .as_slice()
}
// #endregion 🔖️Register

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs"]
pub mod live_visual;

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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-the-column-head-node-n3/🦀️.rs"]
                            mod tests_appends_the_column_head_node_n3;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚨️dup-node-id-86f2e1/🦀️.rs"]
                            mod tests_edge_dup_node_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏗️hall-new-node-b26700/🦀️.rs"]
                            mod tests_hall_hall_new_node;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚨️no-such-node-4027a8/🦀️.rs"]
                            mod tests_edge_no_such_node;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🏗️hall-cut-node-8350fd/🦀️.rs"]
                            mod tests_hall_hall_cut_node;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-the-column-head-056295/🦀️.rs"]
                            mod tests_removes_the_column_head_node_under_a_live_frame;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends-a-diagonal-bracing-bar/🦀️.rs"]
                            mod tests_appends_a_diagonal_bracing_bar;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚨️dangling-start-ab4132/🦀️.rs"]
                            mod tests_edge_dangling_start;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🏗️hall-new-tie-074a69/🦀️.rs"]
                            mod tests_hall_hall_new_tie;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚨️no-such-element-eb788c/🦀️.rs"]
                            mod tests_edge_no_such_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛓️rafter-under-udl-e0342d/🦀️.rs"]
                            mod tests_edge_rafter_under_udl;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🏗️hall-cut-tie-c268d4/🦀️.rs"]
                            mod tests_hall_hall_cut_tie;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes-the-bracing-be89d2/🦀️.rs"]
                            mod tests_removes_the_bracing_bar_and_leaves_the_frame;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚨️dangling-sec-70b168/🦀️.rs"]
                            mod tests_edge_dangling_sec;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️renames-brace-219be2/🦀️.rs"]
                            mod tests_edge_renames_brace;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⏸️same-element-61adb2/🦀️.rs"]
                            mod tests_edge_same_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🏗️hall-strut-d0e4b7/🦀️.rs"]
                            mod tests_hall_hall_strut;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔄️rolls-the-column-50f732/🦀️.rs"]
                            mod tests_rolls_the_column_about_its_own_axis;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🪙️appends-an-9fdced/🦀️.rs"]
                            mod tests_appends_an_aluminium_alloy;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🚨️dup-material-id-1c0787/🦀️.rs"]
                            mod tests_edge_dup_material_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🧨️nu-at-a-half-8253d2/🦀️.rs"]
                            mod tests_edge_nu_at_a_half;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🏗️hall-new-steel-0d2572/🦀️.rs"]
                            mod tests_hall_hall_new_steel;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/⛓️glulam-in-use-1208e1/🦀️.rs"]
                            mod tests_edge_glulam_in_use;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚨️no-such-material-494b10/🦀️.rs"]
                            mod tests_edge_no_such_material;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🏗️hall-cut-gl32c-264bca/🦀️.rs"]
                            mod tests_hall_hall_cut_gl32c;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚫️removes-the-b7b56a/🦀️.rs"]
                            mod tests_removes_the_unreferenced_aluminium_alloy;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🧨️negative-e-84dad7/🦀️.rs"]
                            mod tests_edge_negative_e;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🪪️renames-c24-b60696/🦀️.rs"]
                            mod tests_edge_renames_c24;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/⏸️same-material-950f90/🦀️.rs"]
                            mod tests_edge_same_material;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🏗️hall-regrades-8dbc23/🦀️.rs"]
                            mod tests_hall_hall_regrades;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/📉️softens-the-2cd183/🦀️.rs"]
                            mod tests_softens_the_steel_shear_modulus_in_place;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🔳️appends-a-square-bd0e4e/🦀️.rs"]
                            mod tests_appends_a_square_hollow_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚨️dup-section-id-a76686/🦀️.rs"]
                            mod tests_edge_dup_section_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🧨️zero-area-475a19/🦀️.rs"]
                            mod tests_edge_zero_area;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🏗️hall-new-beam-251a92/🦀️.rs"]
                            mod tests_hall_hall_new_beam;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚨️no-such-section-50d29b/🦀️.rs"]
                            mod tests_edge_no_such_section;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛓️purlin-in-use-99eb01/🦀️.rs"]
                            mod tests_edge_purlin_in_use;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🏗️hall-cut-shs-d44b9e/🦀️.rs"]
                            mod tests_hall_hall_cut_shs;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-30ecfb/🦀️.rs"]
                            mod tests_removes_the_spare_square_hollow_profile;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🧨️negative-iy-d4e0a8/🦀️.rs"]
                            mod tests_edge_negative_iy;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️renames-purlin-dfe160/🦀️.rs"]
                            mod tests_edge_renames_purlin;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⏸️same-section-d1d013/🦀️.rs"]
                            mod tests_edge_same_section;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🏗️hall-deep-purlin-176fd0/🦀️.rs"]
                            mod tests_hall_hall_deep_purlin;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🌀️raises-the-torsion-296ef0/🦀️.rs"]
                            mod tests_raises_the_torsion_constant_of_hea200;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🔒️clamps-the-column-f801c9/🦀️.rs"]
                            mod tests_clamps_the_column_base_in_all_six_dofs;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🚨️dangling-node-af37e2/🦀️.rs"]
                            mod tests_edge_dangling_node;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🏗️hall-new-pin-c033f2/🦀️.rs"]
                            mod tests_hall_hall_new_pin;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🚨️no-such-support-edd22a/🦀️.rs"]
                            mod tests_edge_no_such_support;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🏗️hall-cut-pin-66d795/🦀️.rs"]
                            mod tests_hall_hall_cut_pin;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🔓️releases-the-b3ebb0/🦀️.rs"]
                            mod tests_releases_the_pinned_node_n2;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🚨️dangling-node-d44469/🦀️.rs"]
                            mod tests_edge_dangling_node;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🪪️renames-pin-29f41a/🦀️.rs"]
                            mod tests_edge_renames_pin;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/⏸️same-support-bff8b3/🦀️.rs"]
                            mod tests_edge_same_support;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🔄️frees-the-three-7783c9/🦀️.rs"]
                            mod tests_frees_the_three_rotations_at_the_column_base;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🏗️hall-fixes-base-b5aa1b/🦀️.rs"]
                            mod tests_hall_hall_fixes_base;
                        }
                        #[path = "."]
                        pub mod create_solid {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/🏠️appends-an-extruded-roof-slab/🦀️.rs"]
                            mod tests_appends_an_extruded_roof_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/🚨️dangling-mat-1ebd78/🦀️.rs"]
                            mod tests_edge_dangling_mat;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/📐️sliver-outline-316a7c/🦀️.rs"]
                            mod tests_edge_sliver_outline;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/🏗️hall-new-slab-d79da4/🦀️.rs"]
                            mod tests_hall_hall_new_slab;
                        }
                        #[path = "."]
                        pub mod delete_solid {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/🚨️no-such-solid-f08d23/🦀️.rs"]
                            mod tests_edge_no_such_solid;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/⛓️raft-under-load-e4ea39/🦀️.rs"]
                            mod tests_edge_raft_under_load;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/🏗️hall-cut-apron-6c79d3/🦀️.rs"]
                            mod tests_hall_hall_cut_apron;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/🚫️removes-the-roof-slab-f0fb64/🦀️.rs"]
                            mod tests_removes_the_roof_slab_and_keeps_its_material;
                        }
                        #[path = "."]
                        pub mod replace_solid {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/🚨️dangling-mat-9c89da/🦀️.rs"]
                            mod tests_edge_dangling_mat;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/🪪️renames-apron-7bfadd/🦀️.rs"]
                            mod tests_edge_renames_apron;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/⏸️same-solid-8ad12c/🦀️.rs"]
                            mod tests_edge_same_solid;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/📐️zero-height-2b131a/🦀️.rs"]
                            mod tests_edge_zero_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/🏗️hall-thick-raft-cddc0f/🦀️.rs"]
                            mod tests_hall_hall_thick_raft;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/📚️thickens-the-slab-and-b51ef0/🦀️.rs"]
                            mod tests_thickens_the_slab_and_adds_a_mesh_layer;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🌬️appends-a-wind-case-a6c267/🦀️.rs"]
                            mod tests_appends_a_wind_case_pushing_on_the_column_head;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🚨️dangling-solid-5e04d9/🦀️.rs"]
                            mod tests_edge_dangling_solid;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🏗️hall-snow-drift-068d9b/🦀️.rs"]
                            mod tests_hall_hall_snow_drift;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/⛓️dead-in-combos-e73167/🦀️.rs"]
                            mod tests_edge_dead_in_combos;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚨️no-such-case-ef1fde/🦀️.rs"]
                            mod tests_edge_no_such_case;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🏗️hall-cut-crane-52270d/🦀️.rs"]
                            mod tests_hall_hall_cut_crane;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚫️removes-the-wind-caeb06/🦀️.rs"]
                            mod tests_removes_the_wind_case_together_with_its_load;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/⏸️dup-load-id-4f4a0a/🦀️.rs"]
                            mod tests_edge_dup_load_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🚨️no-such-member-3fe6e9/🦀️.rs"]
                            mod tests_edge_no_such_member;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🏗️hall-adds-udl-e345cb/🦀️.rs"]
                            mod tests_hall_hall_adds_udl;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🏠️lays-an-area-pressure-over-769710/🦀️.rs"]
                            mod tests_lays_an_area_pressure_over_the_roof_slab;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/➖️drops-the-trailing-member-b73b25/🦀️.rs"]
                            mod tests_drops_the_trailing_member_udl_from_the_dead_case;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/🚨️no-such-load-5bab2d/🦀️.rs"]
                            mod tests_edge_no_such_load;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/🏗️hall-cut-wind-6cf528/🦀️.rs"]
                            mod tests_hall_hall_cut_wind;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🚨️sw-no-such-case-bfe5bc/🦀️.rs"]
                            mod tests_edge_sw_no_such_case;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🏗️hall-crane-sw-978370/🦀️.rs"]
                            mod tests_hall_hall_crane_sw;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/⏸️switches-self-7e0cda/🦀️.rs"]
                            mod tests_switches_self_weight_off_for_the_dead_case;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🔗️appends-a-8ede20/🦀️.rs"]
                            mod tests_appends_a_serviceability_combination_keyed_by_case_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🚨️dangling-term-b9d144/🦀️.rs"]
                            mod tests_edge_dangling_term;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🏗️hall-new-acc-4099b2/🦀️.rs"]
                            mod tests_hall_hall_new_acc;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🚨️no-such-combo-f42cd6/🦀️.rs"]
                            mod tests_edge_no_such_combo;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🏗️hall-cut-qp-ebd806/🦀️.rs"]
                            mod tests_hall_hall_cut_qp;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/✂️removes-the-182f7b/🦀️.rs"]
                            mod tests_removes_the_serviceability_combination_and_keeps_both_cases;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-7b5381/🦀️.rs"]
                            mod tests_doubles_the_buckling_mode_count;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/⏸️same-settings-fdb832/🦀️.rs"]
                            mod tests_edge_same_settings;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🧨️zero-modes-a27c74/🦀️.rs"]
                            mod tests_edge_zero_modes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🏗️hall-more-modes-ecbb5c/🦀️.rs"]
                            mod tests_hall_hall_more_modes;
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

pub use crate::standards::v1::subsets::any::schema::diff::Fem3dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

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
    pub mod fem3d {
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

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🏋️add-area-load/🦀️.rs"]
            pub mod add_area_load;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/➖️add-bar/🦀️.rs"]
            pub mod add_bar;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🔗️add-combination/🦀️.rs"]
            pub mod add_combination;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🖼️add-frame/🦀️.rs"]
            pub mod add_frame;
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
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📐️add-section/🦀️.rs"]
            pub mod add_section;
            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧊️add-solid/🦀️.rs"]
            pub mod add_solid;
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
    pub mod fem3d {
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
