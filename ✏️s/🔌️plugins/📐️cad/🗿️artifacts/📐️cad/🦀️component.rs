//! 📐️ CAD artifact — the `cad.scene` document schema: the `CadSnapshot` projection, its object/
//! reference/geometry/camera records, and the pane vocabulary every other cad node addresses them by.
//! The declarative `spatial.interaction` spec types live beside this file in
//! `🎬️interaction-spec/🦀️component.rs`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Domain
pub const CAD_DOCUMENT_SCHEMA: &str = "cad.scene";

pub const CAD_PLAY_DOCUMENT_SCHEMA: &str = "cad.document";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum CadPaneId {
    Shape,
    Building,
    Energy,
    StructureClassic,
}

impl CadPaneId {
    pub fn model_definition_id(self) -> &'static str {
        match self {
            Self::Shape => "spatial.shape",
            Self::Building => "aec.building",
            Self::Energy => "aec.building.energy",
            Self::StructureClassic => "aec.building.structure.classic",
        }
    }

    pub fn all() -> [Self; 4] {
        [Self::Shape, Self::Building, Self::Energy, Self::StructureClassic]
    }
}

/// @emoji 🧩️ Fixed per-pane composed `s.stdio.semio.model` child slot — one of the four fields the
/// derived `ArtifactSchema` on `CadSnapshot` classifies as a `#[child(...)]` slot. Kept as a plain
/// helper alias so accessor functions below read uniformly across all four panes.
pub type CadModelChild = store::ArtifactChild<SemioModelSnapshot>;

/// @emoji 📐️ Composed `s.stdio.semio.drawing` child — cad's forward-declared "engineering assembly"
/// composition slot (design-full-plan.md §4: `cad | engineering assembly | model, drawing`). Empty
/// today (cad carries no persisted 2D drawing content yet); real cardinality grows via
/// `create-drawing`/`delete-drawing` once a caller actually attaches one.
pub type CadDrawingChild = store::ArtifactChild<SemioDrawingSnapshot>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadReference {
    pub id: String,
    pub source_url: String,
    #[serde(default = "default_image_media_kind")]
    pub media_kind: String,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    /// 📐️ Uniform scale factor applied to the image plane (unlike `CadObject.scale`, references
    /// are flat and never scaled non-uniformly per axis — every call site only ever reads/writes
    /// a single number, see `apply_reference_patch`/`sample_reference` in `cad/op/rs`).
    #[serde(default)]
    pub scale: Option<f64>,
    #[serde(default = "default_width_world")]
    pub width_world: f64,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub opacity: Option<f64>,
}

fn default_image_media_kind() -> String {
    "image".into()
}

fn default_width_world() -> f64 {
    10.0
}

/// 📎 Map-value scalar for `referencesByModelDefinitionId` (schema parity).
pub type CadReferenceList = Vec<CadReference>;

/// 📐️ Local twin of `semio_framework_plugin::WorldProjectionConfig`'s flat 15-field classical
/// taxonomy (Parallel: Orthographic/Axonometric/Oblique, Perspective: 1/2/3-Point/Curvilinear) —
/// mirrored here rather than imported because `cad/rs` has no dependency on the plugin layer;
/// `cad/engine/rs`'s `cad_camera_projection_config`/`cad_camera_set_projection_config` convert
/// field-for-field between this and the real `WorldProjectionConfig` around the shared projection
/// helpers. See https://en.wikipedia.org/wiki/Axonometric_projection and
/// https://en.wikipedia.org/wiki/Oblique_projection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct CadProjectionDsl {
    pub kind: String,
    pub orthographic_view: String,
    pub axonometric_variant: String,
    pub axonometric_angle_a: f64,
    pub axonometric_angle_b: f64,
    pub axonometric_quadrant: String,
    pub oblique_variant: String,
    pub oblique_angle: f64,
    pub oblique_depth: f64,
    pub one_point_axis: String,
    pub fov: f64,
    pub two_point_shift: f64,
    pub curvilinear_fov: f64,
    pub curvilinear_strength: f64,
    pub curvilinear_mapping: String,
}

impl Default for CadProjectionDsl {
    fn default() -> Self {
        Self {
            kind: "threePoint".into(),
            orthographic_view: "top".into(),
            axonometric_variant: "isometric".into(),
            axonometric_angle_a: 15.0,
            axonometric_angle_b: 12.0,
            axonometric_quadrant: "ne".into(),
            oblique_variant: "cavalier".into(),
            oblique_angle: 45.0,
            oblique_depth: 1.0,
            one_point_axis: "y".into(),
            fov: 50.0,
            two_point_shift: 0.0,
            curvilinear_fov: 120.0,
            curvilinear_strength: 1.0,
            curvilinear_mapping: "fisheye".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadCamera {
    #[serde(default = "default_camera_position")]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default = "default_camera_target")]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[serde(default = "one_f64")]
    pub zoom: f64,
    #[serde(default = "default_fov")]
    pub fov: f64,
    #[serde(default)]
    #[dsl(block)]
    pub projection: CadProjectionDsl,
}

impl Default for CadCamera {
    fn default() -> Self {
        Self { position: default_camera_position(), target: default_camera_target(), zoom: one_f64(), fov: default_fov(), projection: CadProjectionDsl::default() }
    }
}

fn default_camera_position() -> [f64; 3] {
    [12.0, -12.0, 8.0]
}

fn default_camera_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

fn default_fov() -> f64 {
    50.0
}

fn one_f64() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadNode {
    pub id: String,
    pub label: String,
    pub kind: String,
}

/// @emoji 🧩️ Reads the fixed per-pane `s.stdio.semio.model` CHILD HANDLE (`child_id`/`target` only —
/// never the resolved content; a child is its own document, resolving it is a host/composition
/// concern, never something a pure `CadSnapshot` accessor can do — see `🔖️Composition` in
/// `🏪️store/🦀️component.rs`).
pub fn cad_pane_model(scene: &CadSnapshot, pane: CadPaneId) -> Option<&CadModelChild> {
    match pane {
        CadPaneId::Shape => scene.shape_model.as_ref(),
        CadPaneId::Building => scene.building_model.as_ref(),
        CadPaneId::Energy => scene.energy_model.as_ref(),
        CadPaneId::StructureClassic => scene.structure_classic_model.as_ref(),
    }
}

pub fn cad_pane_model_mut(scene: &mut CadSnapshot, pane: CadPaneId) -> &mut Option<CadModelChild> {
    match pane {
        CadPaneId::Shape => &mut scene.shape_model,
        CadPaneId::Building => &mut scene.building_model,
        CadPaneId::Energy => &mut scene.energy_model,
        CadPaneId::StructureClassic => &mut scene.structure_classic_model,
    }
}

fn default_model_definition_id() -> String {
    "spatial.shape".into()
}

pub fn empty_cad_snapshot() -> CadSnapshot {
    CadSnapshot {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        shape_model: None,
        building_model: None,
        energy_model: None,
        structure_classic_model: None,
        drawings: Vec::new(),
        references_by_model_definition_id: BTreeMap::new(),
        nodes: Vec::new(),
        active_model_definition_id: default_model_definition_id(),
    }
}

pub fn cad_pane_from_model_definition_id(model_definition_id: &str) -> Option<CadPaneId> {
    CadPaneId::all().into_iter().find(|pane| pane.model_definition_id() == model_definition_id)
}

//#region 🔖️Snapshot
/// 📸️ Re-export persisted snapshot type (defined in snapshot schema facet).
pub use crate::artifacts::cad::schema::snapshot::CadSnapshot;
pub use crate::artifacts::cad::schema::diff::CadDiff;
pub use crate::artifacts::cad::schema::mutations::CadMutation;
//#endregion 🔖️Snapshot

//#endregion 🔖️Domain

//#region 🔖️ArtifactKind
/// 🗿️ The `3d.cad` artifact kind this plugin contributes — lifted out of the app manifest builder's
/// `.artifact_kind(…)` so the artifact node owns its own identity (schema, media capability, and the
/// import/export format set the kernel exposes for it).
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.cad".into(),
        name: "3D CAD".into(),
        source_format: "cad.scene".into(),
        component_kind: "cad".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::Brep,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Brep },
        schema: "cad.scene".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["dwg", "glb", "gltf", "ifc", "json", "obj", "png", "step", "stl"],
        import_stdio_kinds: vec!["dwg", "glb", "gltf", "ifc", "json", "obj", "png", "step", "stl"],
    }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `🗒️note`'s own `pilot_languages()` convention (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// M1/W1b). Relocated from `⚙️engine/🦀️component.rs` alongside `declaration()` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — `declaration()`'s only caller, kept private.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "cad.document",
                    extension: Some("cad"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::cad::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::cad::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::cad::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::cad::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("cad.document"),
                },
                dsl::LanguageSpec {
                    id: "cad.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::cad::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::cad::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::cad::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::cad::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("cad.op"),
                },
                dsl::LanguageSpec {
                    id: "cad.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::cad::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::cad::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("cad.diff"),
                },
                dsl::LanguageSpec {
                    id: "cad.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::cad::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::cad::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("cad.pack"),
                },
                dsl::LanguageSpec {
                    id: "cad.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::cad::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::cad::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("cad.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1/W1b) —
/// replaces the old side-effecting `register()`, which called five different global registries
/// directly from a plugin `.setup()` callback. `crate::apps::cad::config::schema::register_app_schema()`
/// is the one exception, still called from `📐️cad/🦀️component.rs`'s own `.setup()`: it registers
/// `CadPlayApp`'s own config/presence schema, an app-scope concern `ArtifactDeclaration` deliberately
/// has no field for (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's
/// artifact-scoped function set.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.cad")
        .schema(crate::artifacts::cad::schema::cad_artifact_schema_descriptor())
        .inferences([crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::cad_artifact_inference_descriptor()])
        .composers(crate::artifacts::cad::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::cad::CadPlayApp>()
        .build()
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Testkit
/// 🧪️ Shared sample records for every cad artifact node's tests (diff/op/dsl/pack/spr) — one
/// definition instead of the four byte-identical copies the old per-module crates each carried.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;

    /// 🧩️ A sample composed `s.stdio.semio.model` CHILD HANDLE — `child_id` + `target` only, per
    /// `🔖️Composition`'s "a child handle is two strings" rule; never the resolved model content
    /// (that lives in the child's own document, out of `CadSnapshot`'s reach).
    pub fn sample_model_child(child_id: &str) -> CadModelChild {
        store::ArtifactChild::new(
            child_id.into(),
            store::os_io::ArtifactRef {
                artifact_id: format!("crate-{child_id}"),
                dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() },
            },
        )
    }

    pub fn sample_reference() -> CadReference {
        CadReference {
            id: "ref-1".into(),
            source_url: "https://example.test/plan.png".into(),
            media_kind: "image".into(),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: Some(1.5),
            width_world: 8.0,
            hidden: false,
            locked: true,
            opacity: Some(0.8),
        }
    }

    pub fn sample_scene() -> CadSnapshot {
        let mut scene = empty_cad_snapshot();
        scene.shape_model = Some(sample_model_child("shape-model-1"));
        scene.building_model = Some(sample_model_child("building-model-1"));
        scene.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
        scene.references_by_model_definition_id.insert(CadPaneId::Shape.model_definition_id().to_string(), vec![sample_reference()]);
        scene.active_model_definition_id = CadPaneId::Shape.model_definition_id().to_string();
        scene
    }
}
//#endregion 🧪️Testkit
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::cad::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("CadComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
