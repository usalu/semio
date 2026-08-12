//! 🎥️ Shooting artifact — the document entity this plugin's app edits: the real icon-studio snapshot
//! (assets, shots, saved cameras, scene lighting).
//!
//! `ShootingSnapshot` lives in `📸️snapshot/🧬️schema` and is re-exported here. Domain records and
//! patch types stay in this root component.

use dsl::DslRecord;
use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, };
use serde::{Deserialize, Serialize};

pub use crate::artifacts::shooting::schema::mutations::ShootingMutation;

pub use crate::artifacts::shooting::schema::diff::ShootingDiff;

pub const SHOOTING_DOCUMENT_SCHEMA: &str = "shooting.shooting";
pub use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::shooting::create_shooting_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.shooting".into(),
        name: "2D Shooting".into(),
        source_format: "shooting.scene".into(),
        component_kind: "shooting".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        schema: "shooting.scene".into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.bmp", "stdio.dwg", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.tiff"],
        import_stdio_kinds: vec!["stdio.bmp", "stdio.dwg", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.tiff"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention already used below. Sole caller is
/// `declaration()` below (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "shooting.document",
                    extension: Some("shooting"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::shooting::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::shooting::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::shooting::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::shooting::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("shooting.document"),
                },
                dsl::LanguageSpec {
                    id: "shooting.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::shooting::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::shooting::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::shooting::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::shooting::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("shooting.op"),
                },
                dsl::LanguageSpec {
                    id: "shooting.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::shooting::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::shooting::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("shooting.diff"),
                },
                dsl::LanguageSpec {
                    id: "shooting.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::shooting::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::shooting::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("shooting.pack"),
                },
                dsl::LanguageSpec {
                    id: "shooting.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::shooting::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::shooting::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("shooting.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called the io registry/schema/inference/language
/// registries and the document codec registrar directly from a plugin `.setup()` callback.
/// `crate::apps::shooting::config::schema::register_app_schema()` is the one exception, still called
/// from `🎥️shooting/🦀️component.rs`'s own `.setup()`: it registers `ShootingPlayApp`'s own
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set.
///
/// DEVIATION (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES reloc-g1): the `.composers(...)`
/// argument is qualified to `standards::v1::subsets::any::io::io_registry::entries()` (the `⚙️engine`
/// directory that used to own this module is gone — deleted, not relocated to a sibling engine) rather
/// than left as the bare `io_registry::entries()` this body used while `io_registry` still lived in that
/// file. Left bare it would now resolve to THIS file's own `io_registry` module below, which has a
/// different, incompatible return type (`&'static [&'static ComposerEntry]`, wrapping the real registry's
/// owned entries) — not the `&'static [ComposerEntry]` `.composers()` expects.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.shooting")
        .schema(crate::artifacts::shooting::standards::v1::subsets::any::schema::shooting_artifact_schema_descriptor())
        .inferences([crate::artifacts::shooting::standards::v1::subsets::any::schema::inferences::shooting_artifact_inference_descriptor()])
        .composers(crate::artifacts::shooting::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::shooting::ShootingPlayApp>()
        .build()
}
//#endregion 🔖️Declaration

//#region 🔖️Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ShootingCamera {
    #[serde(default = "default_camera_position")]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default = "default_camera_target")]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[serde(default = "one_f64")]
    pub zoom: f64,
    #[serde(default = "default_fov")]
    #[dsl(angle = "deg")]
    pub fov: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(dir)]
    pub up: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection: Option<String>,
}

impl Default for ShootingCamera {
    fn default() -> Self {
        Self { position: default_camera_position(), target: default_camera_target(), zoom: 1.0, fov: default_fov(), up: None, projection: None }
    }
}

pub fn default_camera_position() -> [f64; 3] {
    [420.0, -420.0, 320.0]
}

pub fn default_camera_target() -> [f64; 3] {
    [0.0, 0.0, 40.0]
}

pub fn default_fov() -> f64 {
    50.0
}

fn one_f64() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DslRecord)]
#[dsl(keyword = "saved-camera")]
#[serde(rename_all = "camelCase")]
pub struct ShootingSavedCamera {
    #[dsl(defines = "saved-camera")]
    pub id: String,
    pub label: String,
    #[dsl(block)]
    pub camera: ShootingCamera,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DslRecord)]
#[dsl(keyword = "asset")]
#[serde(rename_all = "camelCase")]
pub struct ShootingAsset {
    pub id: String,
    pub name: String,
    pub url: String,
    #[serde(default = "default_glb_format")]
    pub format: String,
    #[serde(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    /// 🪄️ Uniform-vs-per-axis is a JSON-authoring shorthand only, not a persisted distinction —
    /// callers wanting a uniform scale write `[s, s, s]` (see `shooting_asset_scale`, the sole
    /// reader, which never distinguished the two shapes anyway).
    #[serde(default)]
    pub scale: Option<[f64; 3]>,
}

pub fn default_glb_format() -> String {
    "glb".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DslRecord)]
#[dsl(keyword = "shot")]
#[serde(rename_all = "camelCase")]
pub struct ShootingShot {
    pub id: String,
    pub label: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub shape: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "saved-camera")]
    pub camera_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingSun {
    pub enabled: bool,
    #[dsl(angle = "deg")]
    pub azimuth: f64,
    #[dsl(angle = "deg")]
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

impl Default for ShootingSun {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 2.4, color: "#ffffff".into() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingAmbient {
    pub intensity: f64,
    pub color: String,
}

impl Default for ShootingAmbient {
    fn default() -> Self {
        Self { intensity: 1.15, color: "#ffffff".into() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingShadow {
    pub enabled: bool,
    pub opacity: f64,
    pub softness: f64,
}

impl Default for ShootingShadow {
    fn default() -> Self {
        Self { enabled: true, opacity: 0.35, softness: 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingMaterial {
    pub color: String,
    pub metalness: f64,
    pub roughness: f64,
    pub emissive: String,
    pub emissive_intensity: f64,
}

impl Default for ShootingMaterial {
    fn default() -> Self {
        Self { color: "#9aa0ab".into(), metalness: 0.0, roughness: 1.0, emissive: "#000000".into(), emissive_intensity: 0.0 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ShootingSceneLighting {
    #[serde(default)]
    pub background: String,
    #[serde(default)]
    #[dsl(block)]
    pub sun: ShootingSun,
    #[serde(default)]
    #[dsl(block)]
    pub ambient: ShootingAmbient,
    #[serde(default)]
    #[dsl(block)]
    pub shadow: ShootingShadow,
    #[serde(default)]
    #[dsl(block)]
    pub material: ShootingMaterial,
    #[serde(default, rename = "emblemBase64")]
    pub emblem_base64: Option<String>,
}

pub fn empty_shooting_snapshot() -> ShootingSnapshot {
    ShootingSnapshot::default()
}

/// 🧮️ Resolves an asset's scale, defaulting an absent `scale` to identity `[1, 1, 1]`.
pub fn shooting_asset_scale(asset: &ShootingAsset) -> [f64; 3] {
    asset.scale.unwrap_or([1.0, 1.0, 1.0])
}

/// 🧭️ Quaternion (Hamilton product) multiply — `a * b`, both `[x, y, z, w]`. Shared by `op`'s
/// `RotateAssets` diff/backwards math and any other consumer that needs to compose orientations.
pub fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

pub fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

/// 🎯️ Resolves the effective camera for `shot`: the saved camera it references, or `fallback` — the
/// app's session-only live camera (never a document field; see `ShootingConfig::camera` in the app's
/// `🦀️config.rs`) when the shot has no saved camera of its own.
pub fn shooting_resolve_shot_camera(snapshot: &ShootingSnapshot, shot: &ShootingShot, fallback: &ShootingCamera) -> ShootingCamera {
    shot.camera_id.as_ref().and_then(|camera_id| snapshot.saved_cameras.iter().find(|entry| &entry.id == camera_id)).map_or_else(|| fallback.clone(), |entry| entry.camera.clone())
}
//#endregion 🔖️Domain

//#region 🔖️CollectionSupport
impl Identified<String> for ShootingAsset {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for ShootingShot {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for ShootingSavedCamera {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ShootingAssetPatch {
    pub name: Option<String>,
    pub url: Option<String>,
    #[dsl(coord)]
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<[f64; 3]>,
}

impl Patchable<ShootingAssetPatch> for ShootingAsset {
    fn apply_patch(&mut self, patch: &ShootingAssetPatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(url) = &patch.url {
            self.url = url.clone();
        }
        if let Some(origin) = patch.origin {
            self.origin = origin;
        }
        if let Some(orientation) = patch.orientation {
            self.orientation = Some(orientation);
        }
        if let Some(scale) = patch.scale {
            self.scale = Some(scale);
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<ShootingAssetPatch> {
        let patch = ShootingAssetPatch {
            name: (self.name != other.name).then(|| other.name.clone()),
            url: (self.url != other.url).then(|| other.url.clone()),
            origin: (self.origin != other.origin).then_some(other.origin),
            orientation: (self.orientation != other.orientation).then(|| other.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
            scale: (self.scale != other.scale).then(|| other.scale.unwrap_or([1.0, 1.0, 1.0])),
        };
        (patch != ShootingAssetPatch::default()).then_some(patch)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ShootingShotPatch {
    pub label: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub format: Option<String>,
    pub shape: Option<String>,
}

impl Patchable<ShootingShotPatch> for ShootingShot {
    fn apply_patch(&mut self, patch: &ShootingShotPatch) {
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(width) = patch.width {
            self.width = width;
        }
        if let Some(height) = patch.height {
            self.height = height;
        }
        if let Some(format) = &patch.format {
            self.format = format.clone();
        }
        if let Some(shape) = &patch.shape {
            self.shape = shape.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<ShootingShotPatch> {
        let patch = ShootingShotPatch {
            label: (self.label != other.label).then(|| other.label.clone()),
            width: (self.width != other.width).then_some(other.width),
            height: (self.height != other.height).then_some(other.height),
            format: (self.format != other.format).then(|| other.format.clone()),
            shape: (self.shape != other.shape).then(|| other.shape.clone()),
        };
        (patch != ShootingShotPatch::default()).then_some(patch)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ShootingSavedCameraPatch {
    pub label: Option<String>,
    #[dsl(block)]
    pub camera: Option<ShootingCamera>,
}

impl Patchable<ShootingSavedCameraPatch> for ShootingSavedCamera {
    fn apply_patch(&mut self, patch: &ShootingSavedCameraPatch) {
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(camera) = &patch.camera {
            self.camera = camera.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<ShootingSavedCameraPatch> {
        let patch = ShootingSavedCameraPatch { label: (self.label != other.label).then(|| other.label.clone()), camera: (self.camera != other.camera).then(|| other.camera.clone()) };
        (patch != ShootingSavedCameraPatch::default()).then_some(patch)
    }
}

/// 🩹️ The scene-lighting patch — needed both by `op`'s `PatchScene` operation and by the DSL/OpText
/// mirror in `op` (`ShootingMutationDsl::PatchScene`), so it lives here alongside the other `*Patch`
/// records rather than in `op` itself.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ShootingScenePatch {
    pub sun_enabled: Option<bool>,
    #[dsl(angle = "deg")]
    pub sun_azimuth: Option<f64>,
    #[dsl(angle = "deg")]
    pub sun_elevation: Option<f64>,
    pub sun_intensity: Option<f64>,
    pub ambient_intensity: Option<f64>,
    pub shadow_enabled: Option<bool>,
    pub material_roughness: Option<f64>,
}
//#endregion 🔖️CollectionSupport



//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("shooting.scene") is deliberately NOT
    /// `SHOOTING_DOCUMENT_SCHEMA` ("shooting.shooting") — the former names the artifact kind in the OS
    /// media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
    /// merge them.
    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "shooting.scene");
        assert_eq!(SHOOTING_DOCUMENT_SCHEMA, "shooting.shooting");
    }

    #[test]
    fn empty_snapshot_has_no_entities() {
        let snapshot = empty_shooting_snapshot();
        assert!(snapshot.assets.is_empty() && snapshot.shots.is_empty() && snapshot.saved_cameras.is_empty());
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::shooting::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("ShootingComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
