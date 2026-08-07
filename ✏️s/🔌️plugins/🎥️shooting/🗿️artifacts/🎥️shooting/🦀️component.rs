//! 🎥️ Shooting artifact — the document entity this plugin's app edits: the real icon-studio fixture
//! (assets, shots, saved cameras, scene lighting).
//!
//! `store::DocumentDsl`/`store::DocumentPack` for [`ShootingFixture`] are implemented directly here
//! (rather than being re-exported from a shared kernel crate — this plugin has none) via the private
//! [`ShootingFixtureDsl`] mirror below: `ShootingFixture`'s `assets: Vec<ShootingAsset>` (etc.) can't
//! carry `#[dsl(statements, block)]` directly (that needs `Vec<T: DslVariants>`, an enum bound;
//! `ShootingAsset` is a plain record), so this document-shaped twin swaps each collection's element type
//! for its wrapper node and converts at the boundary — same idiom as `imperative::ImperativeDocumentDsl`.

use dsl::DslRecord;
use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat};
use serde::{Deserialize, Serialize};

pub const SHOOTING_FIXTURE_SCHEMA: &str = "shooting.fixture";

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
        export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
        import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
    }
}
//#endregion 🔖️ArtifactKind

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingFixture {
    pub schema: String,
    #[serde(default)]
    pub assets: Vec<ShootingAsset>,
    #[serde(default)]
    pub saved_cameras: Vec<ShootingSavedCamera>,
    #[serde(default)]
    pub scene: ShootingSceneLighting,
    #[serde(default)]
    pub shots: Vec<ShootingShot>,
    #[serde(default)]
    pub active_shot_id: String,
    #[serde(default)]
    pub active_asset_id: String,
}

pub fn empty_shooting_fixture() -> ShootingFixture {
    ShootingFixture { schema: SHOOTING_FIXTURE_SCHEMA.into(), assets: Vec::new(), saved_cameras: Vec::new(), scene: ShootingSceneLighting::default(), shots: Vec::new(), active_shot_id: String::new(), active_asset_id: String::new() }
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
pub fn shooting_resolve_shot_camera(fixture: &ShootingFixture, shot: &ShootingShot, fallback: &ShootingCamera) -> ShootingCamera {
    shot.camera_id.as_ref().and_then(|camera_id| fixture.saved_cameras.iter().find(|entry| &entry.id == camera_id)).map_or_else(|| fallback.clone(), |entry| entry.camera.clone())
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
/// mirror in `op` (`ShootingOperationDsl::PatchScene`), so it lives here alongside the other `*Patch`
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

//#region 🔖️Dsl
/// 📄️ Local mirror of `ShootingFixture` — the real struct's `assets: Vec<ShootingAsset>` (etc.)
/// can't carry `#[dsl(statements, block)]` directly (that needs `Vec<T: DslVariants>`, an enum
/// bound; `ShootingAsset` is a plain record), so this document-shaped twin swaps each collection's
/// element type for its `*Node` wrapper and `ShootingFixture`'s own `DocumentDsl` impl converts at
/// the boundary — same idiom as `imperative::ImperativeDocumentDsl`.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "shooting")]
#[dsl(layout = "lines")]
struct ShootingFixtureDsl {
    schema: String,
    active_shot_id: String,
    active_asset_id: String,
    #[dsl(block)]
    scene: ShootingSceneLighting,
    #[dsl(table)]
    assets: Vec<ShootingAsset>,
    #[dsl(table)]
    shots: Vec<ShootingShot>,
    #[dsl(table)]
    saved_cameras: Vec<ShootingSavedCamera>,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for ShootingFixtureDsl {
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
impl store::DocumentPack for ShootingFixtureDsl {
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


fn shooting_fixture_to_dsl(fixture: &ShootingFixture) -> ShootingFixtureDsl {
    ShootingFixtureDsl {
        schema: fixture.schema.clone(),
        active_shot_id: fixture.active_shot_id.clone(),
        active_asset_id: fixture.active_asset_id.clone(),
        scene: fixture.scene.clone(),
        assets: fixture.assets.clone(),
        shots: fixture.shots.clone(),
        saved_cameras: fixture.saved_cameras.clone(),
    }
}

fn shooting_fixture_from_dsl(dsl_fixture: ShootingFixtureDsl) -> ShootingFixture {
    ShootingFixture {
        schema: dsl_fixture.schema,
        assets: dsl_fixture.assets,
        saved_cameras: dsl_fixture.saved_cameras,
        scene: dsl_fixture.scene,
        shots: dsl_fixture.shots,
        active_shot_id: dsl_fixture.active_shot_id,
        active_asset_id: dsl_fixture.active_asset_id,
    }
}

impl store::DocumentDsl for ShootingFixture {
    const EXTENSION: &'static str = "shooting";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <ShootingFixtureDsl as store::DocumentDsl>::parse_dsl(text)?;
        Ok(shooting_fixture_from_dsl(parsed))
    }

    fn print_dsl(&self) -> String {
        <ShootingFixtureDsl as store::DocumentDsl>::print_dsl(&shooting_fixture_to_dsl(self))
    }
}

/// 📦️ Hand-written `store::DocumentPack` mirror of the `DocumentDsl` impl above — `ShootingFixture`
/// itself doesn't derive `dsl::DslDocument` (see `ShootingFixtureDsl`'s doc comment), so it doesn't
/// pick up the blanket derive-emitted `DocumentPack` impl either; this converts through the same
/// `ShootingFixtureDsl` mirror, which does derive it.
impl store::DocumentPack for ShootingFixture {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <ShootingFixtureDsl as store::DocumentPack>::encode_pack_with(&shooting_fixture_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <ShootingFixtureDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        Ok(shooting_fixture_from_dsl(parsed))
    }
}
//#endregion 🔖️Dsl

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("shooting.scene") is deliberately NOT
    /// `SHOOTING_FIXTURE_SCHEMA` ("shooting.fixture") — the former names the artifact kind in the OS
    /// media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
    /// merge them.
    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "shooting.scene");
        assert_eq!(SHOOTING_FIXTURE_SCHEMA, "shooting.fixture");
    }

    #[test]
    fn empty_fixture_has_no_entities() {
        let fixture = empty_shooting_fixture();
        assert!(fixture.assets.is_empty() && fixture.shots.is_empty() && fixture.saved_cameras.is_empty());
    }
}
//#endregion 🧪️Tests
