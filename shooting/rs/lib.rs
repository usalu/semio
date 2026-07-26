//! 📸 Shooting scene document + typed VCS on `vcs` — the real icon-studio fixture (assets, shots,
//! saved cameras, scene lighting) shared by `shooting-plugin`'s `DocumentApp` implementation.

use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(any(test, target_arch = "wasm32"))]
use vcs::create_document_vcs_envelope;
#[cfg(test)]
use vcs::DocumentVcsCommand;
use vcs::{collection_diff_from_operation, CollectionDiff, CollectionOperation, DocumentVcsEnvelope, DocumentVcsStore, Identified, ItemPatch, Operation, OperationDiff, Patchable};

pub const SHOOTING_FIXTURE_SCHEMA: &str = "shooting.fixture";

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingCamera {
    #[serde(default = "default_camera_position")]
    pub position: [f64; 3],
    #[serde(default = "default_camera_target")]
    pub target: [f64; 3],
    #[serde(default = "one_f64")]
    pub zoom: f64,
    #[serde(default = "default_fov")]
    pub fov: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingSavedCamera {
    pub id: String,
    pub label: String,
    pub camera: ShootingCamera,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingAsset {
    pub id: String,
    pub name: String,
    pub url: String,
    #[serde(default = "default_glb_format")]
    pub format: String,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    #[serde(default)]
    pub scale: Option<Value>,
}

pub fn default_glb_format() -> String {
    "glb".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub camera_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingSun {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

impl Default for ShootingSun {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 2.4, color: "#ffffff".into() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShootingSceneLighting {
    #[serde(default)]
    pub background: String,
    #[serde(default)]
    pub sun: ShootingSun,
    #[serde(default)]
    pub ambient: ShootingAmbient,
    #[serde(default)]
    pub shadow: ShootingShadow,
    #[serde(default)]
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
    pub camera: ShootingCamera,
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

pub type ShootingEnvelope = DocumentVcsEnvelope<ShootingFixture, ShootingOperation>;
pub type ShootingStore = DocumentVcsStore<ShootingFixture, ShootingOperation>;

pub fn empty_shooting_fixture() -> ShootingFixture {
    ShootingFixture {
        schema: SHOOTING_FIXTURE_SCHEMA.into(),
        assets: Vec::new(),
        camera: ShootingCamera::default(),
        saved_cameras: Vec::new(),
        scene: ShootingSceneLighting::default(),
        shots: Vec::new(),
        active_shot_id: String::new(),
        active_asset_id: String::new(),
    }
}

/// 🧮 Resolves an asset's uniform-or-per-axis scale (`scale` is `null`/number/`[x,y,z]`) to `[x, y, z]`.
pub fn shooting_asset_scale(asset: &ShootingAsset) -> [f64; 3] {
    match &asset.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(Value::Number(value)) => {
            let scale = value.as_f64().unwrap_or(1.0);
            [scale, scale, scale]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

/// 🎯 Resolves the effective camera for `shot`: the saved camera it references, or the fixture's own.
pub fn shooting_resolve_shot_camera(fixture: &ShootingFixture, shot: &ShootingShot) -> ShootingCamera {
    shot.camera_id.as_ref().and_then(|camera_id| fixture.saved_cameras.iter().find(|entry| &entry.id == camera_id)).map(|entry| entry.camera.clone()).unwrap_or_else(|| fixture.camera.clone())
}
//#endregion 🔖Domain

//#region 🔖CollectionSupport
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingAssetPatch {
    pub name: Option<String>,
    pub url: Option<String>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<Value>,
}

impl Patchable<ShootingAssetPatch> for ShootingAsset {
    fn apply_patch(&mut self, patch: &ShootingAssetPatch) -> ShootingAssetPatch {
        let inverse = ShootingAssetPatch {
            name: patch.name.as_ref().map(|_| self.name.clone()),
            url: patch.url.as_ref().map(|_| self.url.clone()),
            origin: patch.origin.map(|_| self.origin),
            orientation: patch.orientation.map(|_| self.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
            scale: patch.scale.as_ref().map(|_| self.scale.clone().unwrap_or(Value::Null)),
        };
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
        if let Some(scale) = &patch.scale {
            self.scale = Some(scale.clone());
        }
        inverse
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingShotPatch {
    pub label: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub format: Option<String>,
    pub shape: Option<String>,
}

impl Patchable<ShootingShotPatch> for ShootingShot {
    fn apply_patch(&mut self, patch: &ShootingShotPatch) -> ShootingShotPatch {
        let inverse = ShootingShotPatch {
            label: patch.label.as_ref().map(|_| self.label.clone()),
            width: patch.width.map(|_| self.width),
            height: patch.height.map(|_| self.height),
            format: patch.format.as_ref().map(|_| self.format.clone()),
            shape: patch.shape.as_ref().map(|_| self.shape.clone()),
        };
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
        inverse
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingSavedCameraPatch {
    pub label: Option<String>,
    pub camera: Option<ShootingCamera>,
}

impl Patchable<ShootingSavedCameraPatch> for ShootingSavedCamera {
    fn apply_patch(&mut self, patch: &ShootingSavedCameraPatch) -> ShootingSavedCameraPatch {
        let inverse = ShootingSavedCameraPatch { label: patch.label.as_ref().map(|_| self.label.clone()), camera: patch.camera.as_ref().map(|_| self.camera.clone()) };
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(camera) = &patch.camera {
            self.camera = camera.clone();
        }
        inverse
    }
}

/// ▶️ Applies a `CollectionDiff` (removed → modified → added, matching `apply_collection_operation`'s
/// ordering) to an owned `Vec` — `vcs::CollectionDiff` has no generic apply helper of its own since
/// `modified` patches require the item's `Patchable` impl.
fn apply_collection_diff<TId, TItem, TPatch>(items: &mut Vec<TItem>, diff: &CollectionDiff<TId, TPatch, TItem>)
where
    TId: PartialEq,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    for id in &diff.removed {
        items.retain(|item| item.id() != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id() == &patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

/// ➕ Merges an incoming `CollectionDiff` into an existing one (coalescing two edits' diffs).
fn absorb_collection_diff<TId: Clone, TItem: Clone, TPatch: Clone>(target: &mut Option<CollectionDiff<TId, TPatch, TItem>>, incoming: Option<CollectionDiff<TId, TPatch, TItem>>) {
    if let Some(b) = incoming {
        match target {
            Some(a) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            None => *target = Some(b),
        }
    }
}
//#endregion 🔖CollectionSupport

//#region 🔖Operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
#[allow(
    clippy::large_enum_variant,
    reason = "boxing SetFixture.fixture is a public field-type change; shooting/plugin/rs (its only external constructor, 3 call sites) has a live concurrent edit in progress right now (see CONFLICTS.md) and cannot be safely updated in the same pass — revisit once that edit lands"
)]
pub enum ShootingOperation {
    Assets(CollectionOperation<String, ShootingAsset, ShootingAssetPatch>),
    Shots(CollectionOperation<String, ShootingShot, ShootingShotPatch>),
    SavedCameras(CollectionOperation<String, ShootingSavedCamera, ShootingSavedCameraPatch>),
    SetActiveShot { shot_id: Option<String> },
    SetActiveAsset { asset_id: Option<String> },
    SetCamera { camera: ShootingCamera },
    SetShotCamera { shot_id: String, camera: ShootingCamera },
    PatchScene { patch: ShootingScenePatch },
    TranslateAssets { asset_ids: Vec<String>, dx: f64, dy: f64, dz: f64 },
    RotateAssets { asset_ids: Vec<String>, ax: f64, ay: f64, az: f64, angle: f64 },
    ScaleAssets { asset_ids: Vec<String>, sx: f64, sy: f64, sz: f64 },
    SetFixture { fixture: ShootingFixture },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingScenePatch {
    pub sun_enabled: Option<bool>,
    pub sun_azimuth: Option<f64>,
    pub sun_elevation: Option<f64>,
    pub sun_intensity: Option<f64>,
    pub ambient_intensity: Option<f64>,
    pub shadow_enabled: Option<bool>,
    pub material_roughness: Option<f64>,
}

fn apply_scene_patch(scene: &mut ShootingSceneLighting, patch: &ShootingScenePatch) {
    if let Some(value) = patch.sun_enabled {
        scene.sun.enabled = value;
    }
    if let Some(value) = patch.sun_azimuth {
        scene.sun.azimuth = value;
    }
    if let Some(value) = patch.sun_elevation {
        scene.sun.elevation = value;
    }
    if let Some(value) = patch.sun_intensity {
        scene.sun.intensity = value;
    }
    if let Some(value) = patch.ambient_intensity {
        scene.ambient.intensity = value;
    }
    if let Some(value) = patch.shadow_enabled {
        scene.shadow.enabled = value;
    }
    if let Some(value) = patch.material_roughness {
        scene.material.roughness = value;
    }
}

fn reverse_scene_patch(before: &ShootingSceneLighting, patch: &ShootingScenePatch) -> ShootingScenePatch {
    ShootingScenePatch {
        sun_enabled: patch.sun_enabled.map(|_| before.sun.enabled),
        sun_azimuth: patch.sun_azimuth.map(|_| before.sun.azimuth),
        sun_elevation: patch.sun_elevation.map(|_| before.sun.elevation),
        sun_intensity: patch.sun_intensity.map(|_| before.sun.intensity),
        ambient_intensity: patch.ambient_intensity.map(|_| before.ambient.intensity),
        shadow_enabled: patch.shadow_enabled.map(|_| before.shadow.enabled),
        material_roughness: patch.material_roughness.map(|_| before.material.roughness),
    }
}

fn absorb_scene_patch(target: &mut Option<ShootingScenePatch>, incoming: Option<ShootingScenePatch>) {
    if let Some(b) = incoming {
        let t = target.get_or_insert_with(ShootingScenePatch::default);
        if b.sun_enabled.is_some() {
            t.sun_enabled = b.sun_enabled;
        }
        if b.sun_azimuth.is_some() {
            t.sun_azimuth = b.sun_azimuth;
        }
        if b.sun_elevation.is_some() {
            t.sun_elevation = b.sun_elevation;
        }
        if b.sun_intensity.is_some() {
            t.sun_intensity = b.sun_intensity;
        }
        if b.ambient_intensity.is_some() {
            t.ambient_intensity = b.ambient_intensity;
        }
        if b.shadow_enabled.is_some() {
            t.shadow_enabled = b.shadow_enabled;
        }
        if b.material_roughness.is_some() {
            t.material_roughness = b.material_roughness;
        }
    }
}

/// 🎯 Resolves which `SavedCameras` entry (if any) `shot_id` targets, for `SetCamera`/`SetShotCamera`
/// diffing: a shot referencing a saved camera patches that entry, otherwise the fixture's own camera.
fn resolve_camera_target(fixture: &ShootingFixture, shot_id: Option<&str>) -> Option<String> {
    shot_id.and_then(|id| fixture.shots.iter().find(|shot| shot.id == id)).and_then(|shot| shot.camera_id.clone())
}

fn active_shot_id(fixture: &ShootingFixture) -> Option<String> {
    if !fixture.active_shot_id.is_empty() {
        Some(fixture.active_shot_id.clone())
    } else {
        fixture.shots.first().map(|shot| shot.id.clone())
    }
}

fn camera_diff_for_target(fixture: &ShootingFixture, shot_id: Option<&str>, camera: &ShootingCamera) -> ShootingDiff {
    match resolve_camera_target(fixture, shot_id) {
        Some(camera_id) => {
            ShootingDiff { saved_cameras: Some(CollectionDiff { modified: vec![ItemPatch { id: camera_id, patch: ShootingSavedCameraPatch { label: None, camera: Some(camera.clone()) } }], ..Default::default() }), ..Default::default() }
        }
        None => ShootingDiff { camera: Some(camera.clone()), ..Default::default() },
    }
}

fn camera_for_target(fixture: &ShootingFixture, shot_id: Option<&str>) -> ShootingCamera {
    match resolve_camera_target(fixture, shot_id) {
        Some(camera_id) => fixture.saved_cameras.iter().find(|entry| entry.id == camera_id).map(|entry| entry.camera.clone()).unwrap_or_else(|| fixture.camera.clone()),
        None => fixture.camera.clone(),
    }
}

fn transform_assets_diff(projection: &ShootingFixture, asset_ids: &[String], patch_for: impl Fn(&ShootingAsset) -> ShootingAssetPatch) -> ShootingDiff {
    let modified: Vec<ItemPatch<String, ShootingAssetPatch>> = projection.assets.iter().filter(|asset| asset_ids.contains(&asset.id)).map(|asset| ItemPatch { id: asset.id.clone(), patch: patch_for(asset) }).collect();
    if modified.is_empty() {
        return ShootingDiff::default();
    }
    ShootingDiff { assets: Some(CollectionDiff { modified, ..Default::default() }), ..Default::default() }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingDiff {
    pub assets: Option<CollectionDiff<String, ShootingAssetPatch, ShootingAsset>>,
    pub shots: Option<CollectionDiff<String, ShootingShotPatch, ShootingShot>>,
    pub saved_cameras: Option<CollectionDiff<String, ShootingSavedCameraPatch, ShootingSavedCamera>>,
    pub active_shot_id: Option<String>,
    pub active_asset_id: Option<String>,
    pub camera: Option<ShootingCamera>,
    pub scene: Option<ShootingScenePatch>,
    pub fixture: Option<ShootingFixture>,
}

impl OperationDiff<ShootingFixture> for ShootingDiff {
    fn apply(&self, projection: &ShootingFixture) -> ShootingFixture {
        if let Some(fixture) = &self.fixture {
            return fixture.clone();
        }
        let mut next = projection.clone();
        if let Some(diff) = &self.assets {
            apply_collection_diff(&mut next.assets, diff);
        }
        if let Some(diff) = &self.shots {
            apply_collection_diff(&mut next.shots, diff);
        }
        if let Some(diff) = &self.saved_cameras {
            apply_collection_diff(&mut next.saved_cameras, diff);
        }
        if let Some(id) = &self.active_shot_id {
            next.active_shot_id = id.clone();
        }
        if let Some(id) = &self.active_asset_id {
            next.active_asset_id = id.clone();
        }
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        if let Some(patch) = &self.scene {
            apply_scene_patch(&mut next.scene, patch);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.fixture.is_some() {
            self.fixture = other.fixture;
            return;
        }
        absorb_collection_diff(&mut self.assets, other.assets);
        absorb_collection_diff(&mut self.shots, other.shots);
        absorb_collection_diff(&mut self.saved_cameras, other.saved_cameras);
        if other.active_shot_id.is_some() {
            self.active_shot_id = other.active_shot_id;
        }
        if other.active_asset_id.is_some() {
            self.active_asset_id = other.active_asset_id;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        absorb_scene_patch(&mut self.scene, other.scene);
    }
}

impl Operation<ShootingFixture> for ShootingOperation {
    type Diff = ShootingDiff;

    fn diff(&self, projection: &ShootingFixture) -> ShootingDiff {
        match self {
            ShootingOperation::Assets(operation) => ShootingDiff { assets: Some(collection_diff_from_operation(&projection.assets, operation)), ..Default::default() },
            ShootingOperation::Shots(operation) => ShootingDiff { shots: Some(collection_diff_from_operation(&projection.shots, operation)), ..Default::default() },
            ShootingOperation::SavedCameras(operation) => ShootingDiff { saved_cameras: Some(collection_diff_from_operation(&projection.saved_cameras, operation)), ..Default::default() },
            ShootingOperation::SetActiveShot { shot_id } => ShootingDiff { active_shot_id: Some(shot_id.clone().unwrap_or_default()), ..Default::default() },
            ShootingOperation::SetActiveAsset { asset_id } => ShootingDiff { active_asset_id: Some(asset_id.clone().unwrap_or_default()), ..Default::default() },
            ShootingOperation::SetCamera { camera } => camera_diff_for_target(projection, active_shot_id(projection).as_deref(), camera),
            ShootingOperation::SetShotCamera { shot_id, camera } => camera_diff_for_target(projection, Some(shot_id), camera),
            ShootingOperation::PatchScene { patch } => ShootingDiff { scene: Some(patch.clone()), ..Default::default() },
            ShootingOperation::TranslateAssets { asset_ids, dx, dy, dz } => transform_assets_diff(projection, asset_ids, |asset| ShootingAssetPatch { origin: Some([asset.origin[0] + dx, asset.origin[1] + dy, asset.origin[2] + dz]), ..Default::default() }),
            ShootingOperation::RotateAssets { asset_ids, ax, ay, az, angle } => {
                let delta = quat_from_axis_angle(*ax, *ay, *az, *angle);
                transform_assets_diff(projection, asset_ids, |asset| {
                    let current = asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    ShootingAssetPatch { orientation: Some(quat_mul(delta, current)), ..Default::default() }
                })
            }
            ShootingOperation::ScaleAssets { asset_ids, sx, sy, sz } => transform_assets_diff(projection, asset_ids, |asset| {
                let current = shooting_asset_scale(asset);
                ShootingAssetPatch { scale: Some(serde_json::json!([current[0] * sx, current[1] * sy, current[2] * sz])), ..Default::default() }
            }),
            ShootingOperation::SetFixture { fixture } => ShootingDiff { fixture: Some(fixture.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &ShootingFixture) -> Vec<Self> {
        match self {
            ShootingOperation::Assets(operation) => vec![ShootingOperation::Assets(vcs::invert_collection_operation(&projection.assets, operation))],
            ShootingOperation::Shots(operation) => vec![ShootingOperation::Shots(vcs::invert_collection_operation(&projection.shots, operation))],
            ShootingOperation::SavedCameras(operation) => {
                vec![ShootingOperation::SavedCameras(vcs::invert_collection_operation(&projection.saved_cameras, operation))]
            }
            ShootingOperation::SetActiveShot { .. } => vec![ShootingOperation::SetActiveShot { shot_id: if projection.active_shot_id.is_empty() { None } else { Some(projection.active_shot_id.clone()) } }],
            ShootingOperation::SetActiveAsset { .. } => vec![ShootingOperation::SetActiveAsset { asset_id: if projection.active_asset_id.is_empty() { None } else { Some(projection.active_asset_id.clone()) } }],
            ShootingOperation::SetCamera { .. } => vec![ShootingOperation::SetCamera { camera: camera_for_target(projection, active_shot_id(projection).as_deref()) }],
            ShootingOperation::SetShotCamera { shot_id, .. } => vec![ShootingOperation::SetShotCamera { shot_id: shot_id.clone(), camera: camera_for_target(projection, Some(shot_id)) }],
            ShootingOperation::PatchScene { patch } => vec![ShootingOperation::PatchScene { patch: reverse_scene_patch(&projection.scene, patch) }],
            ShootingOperation::TranslateAssets { asset_ids, dx, dy, dz } => vec![ShootingOperation::TranslateAssets { asset_ids: asset_ids.clone(), dx: -dx, dy: -dy, dz: -dz }],
            ShootingOperation::RotateAssets { asset_ids, ax, ay, az, angle } => vec![ShootingOperation::RotateAssets { asset_ids: asset_ids.clone(), ax: *ax, ay: *ay, az: *az, angle: -angle }],
            ShootingOperation::ScaleAssets { asset_ids, sx, sy, sz } => {
                let inv = |value: f64| if value.abs() < 1e-8 { 1.0 } else { 1.0 / value };
                vec![ShootingOperation::ScaleAssets { asset_ids: asset_ids.clone(), sx: inv(*sx), sy: inv(*sy), sz: inv(*sz) }]
            }
            ShootingOperation::SetFixture { .. } => vec![ShootingOperation::SetFixture { fixture: projection.clone() }],
        }
    }
}
//#endregion 🔖Operations

//#region 🔖Dsl
/// 📜 Hand-rolled lexer, parser and printer for `ShootingFixture`'s `.shooting` DSL and
/// `ShootingOperation`'s compact single-line op encoding (`SetFixture` reprints the same document
/// grammar on one line; `Assets`/`Shots`/`SavedCameras` reuse the same per-item field grammar the
/// document's own collection sections use). Whitespace (including newlines) is never significant to
/// the parser — `print_dsl` inserts newlines/indentation purely for readability, `print_op` renders the
/// identical grammar with spaces only. See {@link vcs::DocumentDsl} and {@link vcs::OpText}.
mod shooting_text {
    use super::*;
    use std::collections::HashMap;

    //#region Lexer
    #[derive(Clone, Debug, PartialEq)]
    enum Tok {
        Word(String),
        Str(String),
        LBrace,
        RBrace,
        Eof,
    }

    #[derive(Clone, Debug)]
    struct Lexed {
        tok: Tok,
        span: vcs::TextSpan,
    }

    /// 🔤 Scans `input` into tokens. A bareword `Word` runs until whitespace/`{`/`}`/`"`, so `=` and `,`
    /// are ordinary word characters — `key=value` collapses into one token (split later by
    /// {@link Parser::parse_kv_map}), and only a quoted value forces a token boundary right after `key=`.
    fn lex(input: &str) -> Result<Vec<Lexed>, vcs::TextError> {
        let chars: Vec<char> = input.chars().collect();
        let mut out = Vec::new();
        let mut i = 0usize;
        let mut line = 1u32;
        let mut col = 1u32;
        while i < chars.len() {
            match chars[i] {
                ' ' | '\t' | '\r' => {
                    i += 1;
                    col += 1;
                }
                '\n' => {
                    i += 1;
                    line += 1;
                    col = 1;
                }
                '{' => {
                    out.push(Lexed { tok: Tok::LBrace, span: vcs::TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '}' => {
                    out.push(Lexed { tok: Tok::RBrace, span: vcs::TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '"' => {
                    let (start_line, start_col) = (line, col);
                    i += 1;
                    col += 1;
                    let mut s = String::new();
                    let mut closed = false;
                    while i < chars.len() {
                        let ch = chars[i];
                        if ch == '\\' && i + 1 < chars.len() {
                            match chars[i + 1] {
                                'n' => s.push('\n'),
                                '"' => s.push('"'),
                                '\\' => s.push('\\'),
                                other => {
                                    s.push('\\');
                                    s.push(other);
                                }
                            }
                            i += 2;
                            col += 2;
                        } else if ch == '"' {
                            i += 1;
                            col += 1;
                            closed = true;
                            break;
                        } else if ch == '\n' {
                            s.push(ch);
                            i += 1;
                            line += 1;
                            col = 1;
                        } else {
                            s.push(ch);
                            i += 1;
                            col += 1;
                        }
                    }
                    if !closed {
                        return Err(vcs::TextError::new("unterminated string literal", vcs::TextSpan::at(start_line, start_col)));
                    }
                    out.push(Lexed { tok: Tok::Str(s), span: vcs::TextSpan::at(start_line, start_col) });
                }
                _ => {
                    let (start_line, start_col, start) = (line, col, i);
                    while i < chars.len() && !matches!(chars[i], ' ' | '\t' | '\r' | '\n' | '{' | '}' | '"') {
                        i += 1;
                        col += 1;
                    }
                    let word: String = chars[start..i].iter().collect();
                    out.push(Lexed { tok: Tok::Word(word), span: vcs::TextSpan::at(start_line, start_col) });
                }
            }
        }
        out.push(Lexed { tok: Tok::Eof, span: vcs::TextSpan::at(line, col) });
        Ok(out)
    }
    //#endregion Lexer

    //#region Parser
    #[derive(Clone, Debug)]
    enum FieldValue {
        Str(String),
        Word(String),
    }

    struct Parser {
        toks: Vec<Lexed>,
        pos: usize,
    }

    impl Parser {
        fn peek(&self) -> &Tok {
            &self.toks[self.pos].tok
        }

        fn span(&self) -> vcs::TextSpan {
            self.toks[self.pos].span
        }

        fn bump(&mut self) -> Tok {
            let tok = self.toks[self.pos].tok.clone();
            if self.pos + 1 < self.toks.len() {
                self.pos += 1;
            }
            tok
        }

        fn at_rbrace(&self) -> bool {
            matches!(self.peek(), Tok::RBrace)
        }

        fn expect_word(&mut self) -> Result<String, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Word(w) => Ok(w),
                other => Err(vcs::TextError::expected(format!("expected a word, found {other:?}"), span, "word")),
            }
        }

        fn expect_keyword(&mut self, keyword: &str) -> Result<(), vcs::TextError> {
            let span = self.span();
            let word = self.expect_word()?;
            if word != keyword {
                return Err(vcs::TextError::expected(format!("expected '{keyword}', found '{word}'"), span, keyword.to_string()));
            }
            Ok(())
        }

        fn expect_lbrace(&mut self) -> Result<(), vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::LBrace => Ok(()),
                other => Err(vcs::TextError::expected(format!("expected '{{', found {other:?}"), span, "{")),
            }
        }

        fn expect_rbrace(&mut self) -> Result<(), vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::RBrace => Ok(()),
                other => Err(vcs::TextError::expected(format!("expected '}}', found {other:?}"), span, "}")),
            }
        }

        fn expect_str(&mut self) -> Result<String, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Str(s) => Ok(s),
                other => Err(vcs::TextError::expected(format!("expected a quoted string, found {other:?}"), span, "string")),
            }
        }

        /// 🔀 Reads a single standalone `-`/quoted-string token — the grammar for `SetActiveShot`/
        /// `SetActiveAsset`'s optional id argument (no `key=` prefix, just the bare value).
        fn expect_opt_str(&mut self) -> Result<Option<String>, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Word(w) if w == "-" => Ok(None),
                Tok::Str(s) => Ok(Some(s)),
                other => Err(vcs::TextError::expected(format!("expected a quoted string or '-', found {other:?}"), span, "string|-")),
            }
        }

        /// 🗺️ Greedily reads `key=value` tokens (order-independent) until a token that isn't one — the
        /// generic header-field reader every construct (document/camera/scene/asset/shot/patch) is built on.
        fn parse_kv_map(&mut self) -> Result<HashMap<String, (FieldValue, vcs::TextSpan)>, vcs::TextError> {
            let mut map = HashMap::new();
            loop {
                let word = match self.peek() {
                    Tok::Word(w) if w.contains('=') => w.clone(),
                    _ => break,
                };
                let span = self.span();
                self.bump();
                let (key, rest) = word.split_once('=').expect("word already checked to contain '='");
                let value = if rest.is_empty() {
                    FieldValue::Str(self.expect_str()?)
                } else {
                    FieldValue::Word(rest.to_string())
                };
                map.insert(key.to_string(), (value, span));
            }
            Ok(map)
        }
    }

    type FieldMap = HashMap<String, (FieldValue, vcs::TextSpan)>;

    fn kv_str(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Ok(s.clone()),
            Some((FieldValue::Word(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must be a quoted string"), *field_span, "string")),
            None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_opt_str(map: &FieldMap, key: &str) -> Option<String> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Some(s.clone()),
            _ => None,
        }
    }

    fn kv_word(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) => Ok(w.clone()),
            Some((FieldValue::Str(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must not be quoted"), *field_span, "word")),
            None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_num(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<f64, vcs::TextError> {
        let word = kv_word(map, key, span)?;
        word.parse::<f64>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a number"), span, "number"))
    }

    fn kv_opt_num(map: &FieldMap, key: &str) -> Option<f64> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) => w.parse::<f64>().ok(),
            _ => None,
        }
    }

    fn kv_u32(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<u32, vcs::TextError> {
        let word = kv_word(map, key, span)?;
        word.parse::<u32>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a non-negative integer"), span, "u32"))
    }

    fn kv_usize(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<usize, vcs::TextError> {
        let word = kv_word(map, key, span)?;
        word.parse::<usize>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a non-negative integer"), span, "usize"))
    }

    fn kv_bool(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<bool, vcs::TextError> {
        match kv_word(map, key, span)?.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(vcs::TextError::expected(format!("field '{key}' must be 'true' or 'false'"), span, "true|false")),
        }
    }

    fn kv_opt_bool(map: &FieldMap, key: &str) -> Option<bool> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) if w == "true" => Some(true),
            Some((FieldValue::Word(w), _)) if w == "false" => Some(false),
            _ => None,
        }
    }

    /// 🧮 Parses `scale`'s arbitrary `serde_json::Value` from its quoted-JSON encoding (a bare `-`
    /// word, or an absent field, means `None` — see {@link fmt_opt_json}).
    fn kv_opt_json(map: &FieldMap, key: &str) -> Option<Value> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => serde_json::from_str::<Value>(s).ok(),
            _ => None,
        }
    }

    fn parse_vec3(word: &str, span: vcs::TextSpan) -> Result<[f64; 3], vcs::TextError> {
        let parts: Vec<&str> = word.split(',').collect();
        if parts.len() != 3 {
            return Err(vcs::TextError::expected("expected 3 comma-separated numbers", span, "x,y,z"));
        }
        let mut out = [0.0; 3];
        for (index, part) in parts.iter().enumerate() {
            out[index] = part.parse::<f64>().map_err(|_| vcs::TextError::expected(format!("invalid vector component '{part}'"), span, "number"))?;
        }
        Ok(out)
    }

    fn parse_vec4(word: &str, span: vcs::TextSpan) -> Result<[f64; 4], vcs::TextError> {
        let parts: Vec<&str> = word.split(',').collect();
        if parts.len() != 4 {
            return Err(vcs::TextError::expected("expected 4 comma-separated numbers", span, "x,y,z,w"));
        }
        let mut out = [0.0; 4];
        for (index, part) in parts.iter().enumerate() {
            out[index] = part.parse::<f64>().map_err(|_| vcs::TextError::expected(format!("invalid vector component '{part}'"), span, "number"))?;
        }
        Ok(out)
    }

    fn kv_vec3(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<[f64; 3], vcs::TextError> {
        parse_vec3(&kv_word(map, key, span)?, span)
    }

    fn kv_opt_vec3(map: &FieldMap, key: &str) -> Option<[f64; 3]> {
        match map.get(key) {
            Some((FieldValue::Word(w), span)) if w != "-" => parse_vec3(w, *span).ok(),
            _ => None,
        }
    }

    fn kv_opt_vec4(map: &FieldMap, key: &str) -> Option<[f64; 4]> {
        match map.get(key) {
            Some((FieldValue::Word(w), span)) if w != "-" => parse_vec4(w, *span).ok(),
            _ => None,
        }
    }

    fn parse_ids(text: &str) -> Vec<String> {
        if text.is_empty() {
            Vec::new()
        } else {
            text.split(',').map(|part| part.to_string()).collect()
        }
    }
    //#endregion Parser

    //#region Printer
    fn quote(value: &str) -> String {
        let mut out = String::with_capacity(value.len() + 2);
        out.push('"');
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                _ => out.push(ch),
            }
        }
        out.push('"');
        out
    }

    fn fmt_num(value: f64) -> String {
        value.to_string()
    }

    fn fmt_vec3(value: [f64; 3]) -> String {
        format!("{},{},{}", fmt_num(value[0]), fmt_num(value[1]), fmt_num(value[2]))
    }

    fn fmt_vec4(value: [f64; 4]) -> String {
        format!("{},{},{},{}", fmt_num(value[0]), fmt_num(value[1]), fmt_num(value[2]), fmt_num(value[3]))
    }

    fn fmt_opt_vec3(value: Option<[f64; 3]>) -> String {
        value.map(fmt_vec3).unwrap_or_else(|| "-".to_string())
    }

    fn fmt_opt_vec4(value: Option<[f64; 4]>) -> String {
        value.map(fmt_vec4).unwrap_or_else(|| "-".to_string())
    }

    fn fmt_opt_str(value: &Option<String>) -> String {
        value.as_deref().map(quote).unwrap_or_else(|| "-".to_string())
    }

    fn fmt_opt_num(value: Option<f64>) -> String {
        value.map(fmt_num).unwrap_or_else(|| "-".to_string())
    }

    fn fmt_opt_bool(value: Option<bool>) -> String {
        value.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string())
    }

    /// 🧮 Prints `scale`'s arbitrary `serde_json::Value` as quoted JSON (`-` when `None`) — the only
    /// field in this DSL that falls back on `serde_json` for its own (already-arbitrary) content.
    fn fmt_opt_json(value: &Option<Value>) -> String {
        match value {
            Some(v) => quote(&serde_json::to_string(v).unwrap_or_else(|_| "null".to_string())),
            None => "-".to_string(),
        }
    }

    fn indent_str(depth: usize) -> String {
        "  ".repeat(depth)
    }

    /// 🧱 Wraps `items` (each already rendered, without its own leading indentation) in `{ }`, one per
    /// line indented at `depth + 1` when `pretty`, or space-joined on one line otherwise.
    fn wrap_body(items: &[String], depth: usize, pretty: bool) -> String {
        if pretty {
            let inner_pad = indent_str(depth + 1);
            let outer_pad = indent_str(depth);
            let body: String = items.iter().map(|item| format!("{inner_pad}{item}\n")).collect();
            format!("{{\n{body}{outer_pad}}}")
        } else {
            format!("{{ {} }}", items.join(" "))
        }
    }
    //#endregion Printer

    //#region Camera
    fn print_camera_kv(camera: &ShootingCamera) -> String {
        format!(
            "position={} target={} zoom={} fov={} up={} projection={}",
            fmt_vec3(camera.position),
            fmt_vec3(camera.target),
            fmt_num(camera.zoom),
            fmt_num(camera.fov),
            fmt_opt_vec3(camera.up),
            fmt_opt_str(&camera.projection),
        )
    }

    fn parse_camera_kv(map: &FieldMap, span: vcs::TextSpan) -> Result<ShootingCamera, vcs::TextError> {
        Ok(ShootingCamera {
            position: kv_vec3(map, "position", span)?,
            target: kv_vec3(map, "target", span)?,
            zoom: kv_num(map, "zoom", span)?,
            fov: kv_num(map, "fov", span)?,
            up: kv_opt_vec3(map, "up"),
            projection: kv_opt_str(map, "projection"),
        })
    }
    //#endregion Camera

    //#region Asset
    fn print_asset_fields(asset: &ShootingAsset) -> String {
        format!(
            "id={} name={} url={} format={} origin={} orientation={} scale={}",
            quote(&asset.id),
            quote(&asset.name),
            quote(&asset.url),
            quote(&asset.format),
            fmt_vec3(asset.origin),
            fmt_opt_vec4(asset.orientation),
            fmt_opt_json(&asset.scale),
        )
    }

    fn parse_asset_fields(map: &FieldMap, span: vcs::TextSpan) -> Result<ShootingAsset, vcs::TextError> {
        Ok(ShootingAsset {
            id: kv_str(map, "id", span)?,
            name: kv_str(map, "name", span)?,
            url: kv_str(map, "url", span)?,
            format: kv_str(map, "format", span)?,
            origin: kv_vec3(map, "origin", span)?,
            orientation: kv_opt_vec4(map, "orientation"),
            scale: kv_opt_json(map, "scale"),
        })
    }

    fn print_asset_patch_fields(patch: &ShootingAssetPatch) -> String {
        format!(
            "name={} url={} origin={} orientation={} scale={}",
            fmt_opt_str(&patch.name),
            fmt_opt_str(&patch.url),
            fmt_opt_vec3(patch.origin),
            fmt_opt_vec4(patch.orientation),
            fmt_opt_json(&patch.scale),
        )
    }

    fn parse_asset_patch_fields(map: &FieldMap, _span: vcs::TextSpan) -> Result<ShootingAssetPatch, vcs::TextError> {
        Ok(ShootingAssetPatch {
            name: kv_opt_str(map, "name"),
            url: kv_opt_str(map, "url"),
            origin: kv_opt_vec3(map, "origin"),
            orientation: kv_opt_vec4(map, "orientation"),
            scale: kv_opt_json(map, "scale"),
        })
    }
    //#endregion Asset

    //#region Shot
    fn print_shot_fields(shot: &ShootingShot) -> String {
        format!(
            "id={} label={} width={} height={} format={} shape={} background={} cameraId={}",
            quote(&shot.id),
            quote(&shot.label),
            shot.width,
            shot.height,
            quote(&shot.format),
            quote(&shot.shape),
            fmt_opt_str(&shot.background),
            fmt_opt_str(&shot.camera_id),
        )
    }

    fn parse_shot_fields(map: &FieldMap, span: vcs::TextSpan) -> Result<ShootingShot, vcs::TextError> {
        Ok(ShootingShot {
            id: kv_str(map, "id", span)?,
            label: kv_str(map, "label", span)?,
            width: kv_u32(map, "width", span)?,
            height: kv_u32(map, "height", span)?,
            format: kv_str(map, "format", span)?,
            shape: kv_str(map, "shape", span)?,
            background: kv_opt_str(map, "background"),
            camera_id: kv_opt_str(map, "cameraId"),
        })
    }

    fn print_shot_patch_fields(patch: &ShootingShotPatch) -> String {
        format!(
            "label={} width={} height={} format={} shape={}",
            fmt_opt_str(&patch.label),
            fmt_opt_num(patch.width.map(|value| value as f64)),
            fmt_opt_num(patch.height.map(|value| value as f64)),
            fmt_opt_str(&patch.format),
            fmt_opt_str(&patch.shape),
        )
    }

    fn parse_shot_patch_fields(map: &FieldMap, _span: vcs::TextSpan) -> Result<ShootingShotPatch, vcs::TextError> {
        Ok(ShootingShotPatch {
            label: kv_opt_str(map, "label"),
            width: kv_opt_num(map, "width").map(|value| value as u32),
            height: kv_opt_num(map, "height").map(|value| value as u32),
            format: kv_opt_str(map, "format"),
            shape: kv_opt_str(map, "shape"),
        })
    }
    //#endregion Shot

    //#region SavedCamera
    fn print_saved_camera_fields(entry: &ShootingSavedCamera) -> String {
        format!("id={} label={} {}", quote(&entry.id), quote(&entry.label), print_camera_kv(&entry.camera))
    }

    fn parse_saved_camera_fields(map: &FieldMap, span: vcs::TextSpan) -> Result<ShootingSavedCamera, vcs::TextError> {
        Ok(ShootingSavedCamera {
            id: kv_str(map, "id", span)?,
            label: kv_str(map, "label", span)?,
            camera: parse_camera_kv(map, span)?,
        })
    }

    /// 🎥 `patch.camera` is a whole-struct `Option<ShootingCamera>` — encoded as a `cameraSet` flag
    /// followed (only when `true`) by the same flat camera fields `print_camera_kv` renders, so a
    /// `false` patch line never carries any camera-shaped keys at all.
    fn print_saved_camera_patch_fields(patch: &ShootingSavedCameraPatch) -> String {
        match &patch.camera {
            Some(camera) => format!("label={} cameraSet=true {}", fmt_opt_str(&patch.label), print_camera_kv(camera)),
            None => format!("label={} cameraSet=false", fmt_opt_str(&patch.label)),
        }
    }

    fn parse_saved_camera_patch_fields(map: &FieldMap, span: vcs::TextSpan) -> Result<ShootingSavedCameraPatch, vcs::TextError> {
        let camera_set = kv_bool(map, "cameraSet", span)?;
        Ok(ShootingSavedCameraPatch {
            label: kv_opt_str(map, "label"),
            camera: if camera_set { Some(parse_camera_kv(map, span)?) } else { None },
        })
    }
    //#endregion SavedCamera

    //#region Scene
    fn print_scene(scene: &ShootingSceneLighting, depth: usize, pretty: bool) -> String {
        let header = format!("scene background={} emblemBase64={}", quote(&scene.background), fmt_opt_str(&scene.emblem_base64));
        let items = vec![
            format!(
                "sun enabled={} azimuth={} elevation={} intensity={} color={}",
                scene.sun.enabled,
                fmt_num(scene.sun.azimuth),
                fmt_num(scene.sun.elevation),
                fmt_num(scene.sun.intensity),
                quote(&scene.sun.color),
            ),
            format!("ambient intensity={} color={}", fmt_num(scene.ambient.intensity), quote(&scene.ambient.color)),
            format!(
                "shadow enabled={} opacity={} softness={}",
                scene.shadow.enabled,
                fmt_num(scene.shadow.opacity),
                fmt_num(scene.shadow.softness),
            ),
            format!(
                "material color={} metalness={} roughness={} emissive={} emissiveIntensity={}",
                quote(&scene.material.color),
                fmt_num(scene.material.metalness),
                fmt_num(scene.material.roughness),
                quote(&scene.material.emissive),
                fmt_num(scene.material.emissive_intensity),
            ),
        ];
        format!("{header} {}", wrap_body(&items, depth, pretty))
    }

    fn parse_scene(p: &mut Parser) -> Result<ShootingSceneLighting, vcs::TextError> {
        let span = p.span();
        p.expect_keyword("scene")?;
        let map = p.parse_kv_map()?;
        let background = kv_str(&map, "background", span)?;
        let emblem_base64 = kv_opt_str(&map, "emblemBase64");
        p.expect_lbrace()?;

        let sun_span = p.span();
        p.expect_keyword("sun")?;
        let sun_map = p.parse_kv_map()?;
        let sun = ShootingSun {
            enabled: kv_bool(&sun_map, "enabled", sun_span)?,
            azimuth: kv_num(&sun_map, "azimuth", sun_span)?,
            elevation: kv_num(&sun_map, "elevation", sun_span)?,
            intensity: kv_num(&sun_map, "intensity", sun_span)?,
            color: kv_str(&sun_map, "color", sun_span)?,
        };

        let ambient_span = p.span();
        p.expect_keyword("ambient")?;
        let ambient_map = p.parse_kv_map()?;
        let ambient = ShootingAmbient {
            intensity: kv_num(&ambient_map, "intensity", ambient_span)?,
            color: kv_str(&ambient_map, "color", ambient_span)?,
        };

        let shadow_span = p.span();
        p.expect_keyword("shadow")?;
        let shadow_map = p.parse_kv_map()?;
        let shadow = ShootingShadow {
            enabled: kv_bool(&shadow_map, "enabled", shadow_span)?,
            opacity: kv_num(&shadow_map, "opacity", shadow_span)?,
            softness: kv_num(&shadow_map, "softness", shadow_span)?,
        };

        let material_span = p.span();
        p.expect_keyword("material")?;
        let material_map = p.parse_kv_map()?;
        let material = ShootingMaterial {
            color: kv_str(&material_map, "color", material_span)?,
            metalness: kv_num(&material_map, "metalness", material_span)?,
            roughness: kv_num(&material_map, "roughness", material_span)?,
            emissive: kv_str(&material_map, "emissive", material_span)?,
            emissive_intensity: kv_num(&material_map, "emissiveIntensity", material_span)?,
        };

        p.expect_rbrace()?;
        Ok(ShootingSceneLighting { background, sun, ambient, shadow, material, emblem_base64 })
    }

    fn print_scene_patch_fields(patch: &ShootingScenePatch) -> String {
        format!(
            "sunEnabled={} sunAzimuth={} sunElevation={} sunIntensity={} ambientIntensity={} shadowEnabled={} materialRoughness={}",
            fmt_opt_bool(patch.sun_enabled),
            fmt_opt_num(patch.sun_azimuth),
            fmt_opt_num(patch.sun_elevation),
            fmt_opt_num(patch.sun_intensity),
            fmt_opt_num(patch.ambient_intensity),
            fmt_opt_bool(patch.shadow_enabled),
            fmt_opt_num(patch.material_roughness),
        )
    }

    fn parse_scene_patch_fields(map: &FieldMap) -> ShootingScenePatch {
        ShootingScenePatch {
            sun_enabled: kv_opt_bool(map, "sunEnabled"),
            sun_azimuth: kv_opt_num(map, "sunAzimuth"),
            sun_elevation: kv_opt_num(map, "sunElevation"),
            sun_intensity: kv_opt_num(map, "sunIntensity"),
            ambient_intensity: kv_opt_num(map, "ambientIntensity"),
            shadow_enabled: kv_opt_bool(map, "shadowEnabled"),
            material_roughness: kv_opt_num(map, "materialRoughness"),
        }
    }
    //#endregion Scene

    //#region Document
    /// 📥 Parses a full `.shooting` document: `shooting` header, `camera`, `scene { ... }`, then the
    /// three collection sections `assets { asset ... }` / `shots { shot ... }` / `savedCameras { camera
    /// ... }` — a fixed order matching {@link print_document}, since `print_document` is this grammar's
    /// only producer.
    pub(super) fn parse_document(text: &str) -> Result<ShootingFixture, vcs::TextError> {
        let toks = lex(text)?;
        let mut p = Parser { toks, pos: 0 };

        let header_span = p.span();
        p.expect_keyword("shooting")?;
        let header_map = p.parse_kv_map()?;
        let schema = kv_str(&header_map, "schema", header_span)?;
        let active_shot_id = kv_str(&header_map, "activeShot", header_span)?;
        let active_asset_id = kv_str(&header_map, "activeAsset", header_span)?;

        let camera_span = p.span();
        p.expect_keyword("camera")?;
        let camera_map = p.parse_kv_map()?;
        let camera = parse_camera_kv(&camera_map, camera_span)?;

        let scene = parse_scene(&mut p)?;

        p.expect_keyword("assets")?;
        p.expect_lbrace()?;
        let mut assets = Vec::new();
        while !p.at_rbrace() {
            let span = p.span();
            p.expect_keyword("asset")?;
            let map = p.parse_kv_map()?;
            assets.push(parse_asset_fields(&map, span)?);
        }
        p.expect_rbrace()?;

        p.expect_keyword("shots")?;
        p.expect_lbrace()?;
        let mut shots = Vec::new();
        while !p.at_rbrace() {
            let span = p.span();
            p.expect_keyword("shot")?;
            let map = p.parse_kv_map()?;
            shots.push(parse_shot_fields(&map, span)?);
        }
        p.expect_rbrace()?;

        p.expect_keyword("savedCameras")?;
        p.expect_lbrace()?;
        let mut saved_cameras = Vec::new();
        while !p.at_rbrace() {
            let span = p.span();
            p.expect_keyword("camera")?;
            let map = p.parse_kv_map()?;
            saved_cameras.push(parse_saved_camera_fields(&map, span)?);
        }
        p.expect_rbrace()?;

        Ok(ShootingFixture { schema, assets, camera, saved_cameras, scene, shots, active_shot_id, active_asset_id })
    }

    /// 📤 Renders `fixture` as `shooting`/`camera`/`scene` (always present) followed by the three
    /// collection sections (always present, possibly with an empty body) — mirrors {@link parse_document}.
    pub(super) fn print_document(fixture: &ShootingFixture, pretty: bool) -> String {
        let mut parts = Vec::new();
        parts.push(format!(
            "shooting schema={} activeShot={} activeAsset={}",
            quote(&fixture.schema),
            quote(&fixture.active_shot_id),
            quote(&fixture.active_asset_id),
        ));
        parts.push(format!("camera {}", print_camera_kv(&fixture.camera)));
        parts.push(print_scene(&fixture.scene, 0, pretty));

        let asset_items: Vec<String> = fixture.assets.iter().map(|asset| format!("asset {}", print_asset_fields(asset))).collect();
        parts.push(format!("assets {}", wrap_body(&asset_items, 0, pretty)));

        let shot_items: Vec<String> = fixture.shots.iter().map(|shot| format!("shot {}", print_shot_fields(shot))).collect();
        parts.push(format!("shots {}", wrap_body(&shot_items, 0, pretty)));

        let saved_camera_items: Vec<String> = fixture.saved_cameras.iter().map(|entry| format!("camera {}", print_saved_camera_fields(entry))).collect();
        parts.push(format!("savedCameras {}", wrap_body(&saved_camera_items, 0, pretty)));

        parts.join(if pretty { "\n" } else { " " })
    }
    //#endregion Document

    //#region Operation
    /// 🧺 Shared printer for the three `CollectionOperation<String, _, _>`-wrapped operation variants
    /// (`Assets`/`Shots`/`SavedCameras`) — `{keyword}-add`/`-remove`/`-move`/`-patch`, reusing each
    /// item/patch type's own field grammar so the collection ops never duplicate a parsing rule.
    fn print_collection_op<TItem, TPatch>(keyword: &str, op: &CollectionOperation<String, TItem, TPatch>, print_item: impl Fn(&TItem) -> String, print_patch: impl Fn(&TPatch) -> String) -> String {
        match op {
            CollectionOperation::Add { index, item } => format!("{keyword}-add index={index} {}", print_item(item)),
            CollectionOperation::Remove { id } => format!("{keyword}-remove id={}", quote(id)),
            CollectionOperation::Move { id, to_index } => format!("{keyword}-move id={} to={to_index}", quote(id)),
            CollectionOperation::Patch { id, patch } => format!("{keyword}-patch id={} {}", quote(id), print_patch(patch)),
        }
    }

    fn parse_collection_op_from_map<TItem, TPatch>(
        map: &FieldMap,
        span: vcs::TextSpan,
        suffix: &str,
        parse_item: impl Fn(&FieldMap, vcs::TextSpan) -> Result<TItem, vcs::TextError>,
        parse_patch: impl Fn(&FieldMap, vcs::TextSpan) -> Result<TPatch, vcs::TextError>,
    ) -> Result<CollectionOperation<String, TItem, TPatch>, vcs::TextError> {
        match suffix {
            "add" => Ok(CollectionOperation::Add { index: kv_usize(map, "index", span)?, item: parse_item(map, span)? }),
            "remove" => Ok(CollectionOperation::Remove { id: kv_str(map, "id", span)? }),
            "move" => Ok(CollectionOperation::Move { id: kv_str(map, "id", span)?, to_index: kv_usize(map, "to", span)? }),
            "patch" => Ok(CollectionOperation::Patch { id: kv_str(map, "id", span)?, patch: parse_patch(map, span)? }),
            other => Err(vcs::TextError::expected(format!("unknown '{other}' collection operation, expected add|remove|move|patch"), span, "add|remove|move|patch")),
        }
    }

    /// ⚡ Renders one `ShootingOperation` as a single line — `SetFixture` reuses the compact
    /// (space-joined) form of {@link print_document}.
    pub(super) fn print_operation(operation: &ShootingOperation) -> String {
        match operation {
            ShootingOperation::Assets(op) => print_collection_op("assets", op, print_asset_fields, print_asset_patch_fields),
            ShootingOperation::Shots(op) => print_collection_op("shots", op, print_shot_fields, print_shot_patch_fields),
            ShootingOperation::SavedCameras(op) => print_collection_op("savedCameras", op, print_saved_camera_fields, print_saved_camera_patch_fields),
            ShootingOperation::SetActiveShot { shot_id } => format!("active-shot {}", fmt_opt_str(shot_id)),
            ShootingOperation::SetActiveAsset { asset_id } => format!("active-asset {}", fmt_opt_str(asset_id)),
            ShootingOperation::SetCamera { camera } => format!("camera {}", print_camera_kv(camera)),
            ShootingOperation::SetShotCamera { shot_id, camera } => format!("shot-camera id={} {}", quote(shot_id), print_camera_kv(camera)),
            ShootingOperation::PatchScene { patch } => format!("scene-patch {}", print_scene_patch_fields(patch)),
            ShootingOperation::TranslateAssets { asset_ids, dx, dy, dz } => {
                format!("translate ids={} dx={} dy={} dz={}", quote(&asset_ids.join(",")), fmt_num(*dx), fmt_num(*dy), fmt_num(*dz))
            }
            ShootingOperation::RotateAssets { asset_ids, ax, ay, az, angle } => {
                format!("rotate ids={} ax={} ay={} az={} angle={}", quote(&asset_ids.join(",")), fmt_num(*ax), fmt_num(*ay), fmt_num(*az), fmt_num(*angle))
            }
            ShootingOperation::ScaleAssets { asset_ids, sx, sy, sz } => {
                format!("scale ids={} sx={} sy={} sz={}", quote(&asset_ids.join(",")), fmt_num(*sx), fmt_num(*sy), fmt_num(*sz))
            }
            ShootingOperation::SetFixture { fixture } => format!("fixture {}", print_document(fixture, false)),
        }
    }

    /// 📥 Parses one op-log line. `fixture ...` (which embeds a whole compact document — itself a
    /// nested instance of this same grammar) is handled as a direct string slice before tokenizing,
    /// mirroring the "one technology, one grammar" reuse from {@link print_operation}.
    pub(super) fn parse_operation(line: &str) -> Result<ShootingOperation, vcs::TextError> {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("fixture ") {
            return Ok(ShootingOperation::SetFixture { fixture: parse_document(rest)? });
        }

        let toks = lex(line)?;
        let mut p = Parser { toks, pos: 0 };
        let span = p.span();
        let keyword = p.expect_word()?;

        if let Some(suffix) = keyword.strip_prefix("assets-") {
            let map = p.parse_kv_map()?;
            return Ok(ShootingOperation::Assets(parse_collection_op_from_map(&map, span, suffix, parse_asset_fields, parse_asset_patch_fields)?));
        }
        if let Some(suffix) = keyword.strip_prefix("shots-") {
            let map = p.parse_kv_map()?;
            return Ok(ShootingOperation::Shots(parse_collection_op_from_map(&map, span, suffix, parse_shot_fields, parse_shot_patch_fields)?));
        }
        if let Some(suffix) = keyword.strip_prefix("savedCameras-") {
            let map = p.parse_kv_map()?;
            return Ok(ShootingOperation::SavedCameras(parse_collection_op_from_map(&map, span, suffix, parse_saved_camera_fields, parse_saved_camera_patch_fields)?));
        }

        match keyword.as_str() {
            "active-shot" => Ok(ShootingOperation::SetActiveShot { shot_id: p.expect_opt_str()? }),
            "active-asset" => Ok(ShootingOperation::SetActiveAsset { asset_id: p.expect_opt_str()? }),
            "camera" => {
                let map = p.parse_kv_map()?;
                Ok(ShootingOperation::SetCamera { camera: parse_camera_kv(&map, span)? })
            }
            "shot-camera" => {
                let map = p.parse_kv_map()?;
                Ok(ShootingOperation::SetShotCamera { shot_id: kv_str(&map, "id", span)?, camera: parse_camera_kv(&map, span)? })
            }
            "scene-patch" => {
                let map = p.parse_kv_map()?;
                Ok(ShootingOperation::PatchScene { patch: parse_scene_patch_fields(&map) })
            }
            "translate" => {
                let map = p.parse_kv_map()?;
                Ok(ShootingOperation::TranslateAssets {
                    asset_ids: parse_ids(&kv_str(&map, "ids", span)?),
                    dx: kv_num(&map, "dx", span)?,
                    dy: kv_num(&map, "dy", span)?,
                    dz: kv_num(&map, "dz", span)?,
                })
            }
            "rotate" => {
                let map = p.parse_kv_map()?;
                Ok(ShootingOperation::RotateAssets {
                    asset_ids: parse_ids(&kv_str(&map, "ids", span)?),
                    ax: kv_num(&map, "ax", span)?,
                    ay: kv_num(&map, "ay", span)?,
                    az: kv_num(&map, "az", span)?,
                    angle: kv_num(&map, "angle", span)?,
                })
            }
            "scale" => {
                let map = p.parse_kv_map()?;
                Ok(ShootingOperation::ScaleAssets {
                    asset_ids: parse_ids(&kv_str(&map, "ids", span)?),
                    sx: kv_num(&map, "sx", span)?,
                    sy: kv_num(&map, "sy", span)?,
                    sz: kv_num(&map, "sz", span)?,
                })
            }
            other => Err(vcs::TextError::expected(format!("unknown operation '{other}'"), span, "operation keyword")),
        }
    }
    //#endregion Operation
}

impl vcs::DocumentDsl for ShootingFixture {
    const EXTENSION: &'static str = "shooting";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        shooting_text::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        shooting_text::print_document(self, true)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
impl vcs::OpText for ShootingOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        shooting_text::parse_operation(line)
    }

    fn print_op(&self) -> String {
        shooting_text::print_operation(self)
    }
}
//#endregion 🔖OpText

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct ShootingDocumentVcs {
        store: RefCell<ShootingStore>,
    }

    #[wasm_bindgen]
    impl ShootingDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<ShootingDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: ShootingEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    ShootingStore::new(envelope)
                }
                None => ShootingStore::new(create_document_vcs_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", empty_shooting_fixture(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use vcs::DocumentDsl;

    fn sample_asset(id: &str) -> ShootingAsset {
        ShootingAsset { id: id.into(), name: format!("Asset {id}"), url: format!("/mesh/{id}.glb"), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None }
    }

    fn sample_shot(id: &str) -> ShootingShot {
        ShootingShot { id: id.into(), label: format!("Shot {id}"), width: 256, height: 256, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: None }
    }

    fn round_trip(fixture: &ShootingFixture, operation: &ShootingOperation) -> ShootingFixture {
        let forward = vcs::apply_operation(fixture, operation);
        let backwards = operation.backwards(fixture);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = vcs::apply_operation(&restored, back);
        }
        assert_eq!(&restored, fixture, "backwards() must exactly restore the pre-operation fixture");
        forward
    }

    #[test]
    fn shooting_projection_round_trip() {
        let mut store = ShootingStore::new(create_document_vcs_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", empty_shooting_fixture(), None));
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![ShootingOperation::Assets(CollectionOperation::Add { index: 0, item: sample_asset("a1") })], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").assets.len(), 1);
    }

    #[test]
    fn assets_add_remove_patch_round_trip() {
        let fixture = empty_shooting_fixture();
        let add = ShootingOperation::Assets(CollectionOperation::Add { index: 0, item: sample_asset("a1") });
        let with_asset = round_trip(&fixture, &add);
        assert_eq!(with_asset.assets.len(), 1);

        let patch = ShootingOperation::Assets(CollectionOperation::Patch { id: "a1".into(), patch: ShootingAssetPatch { name: Some("Renamed".into()), ..Default::default() } });
        let patched = round_trip(&with_asset, &patch);
        assert_eq!(patched.assets[0].name, "Renamed");

        let remove = ShootingOperation::Assets(CollectionOperation::Remove { id: "a1".into() });
        let removed = round_trip(&patched, &remove);
        assert!(removed.assets.is_empty());
    }

    #[test]
    fn shots_patch_round_trip() {
        let mut fixture = empty_shooting_fixture();
        fixture.shots.push(sample_shot("s1"));
        let patch = ShootingOperation::Shots(CollectionOperation::Patch { id: "s1".into(), patch: ShootingShotPatch { label: Some("Hero".into()), width: Some(512), ..Default::default() } });
        let patched = round_trip(&fixture, &patch);
        assert_eq!(patched.shots[0].label, "Hero");
        assert_eq!(patched.shots[0].width, 512);
    }

    #[test]
    fn saved_cameras_add_round_trip() {
        let fixture = empty_shooting_fixture();
        let add = ShootingOperation::SavedCameras(CollectionOperation::Add { index: 0, item: ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() } });
        let added = round_trip(&fixture, &add);
        assert_eq!(added.saved_cameras.len(), 1);
    }

    #[test]
    fn set_active_shot_and_asset_round_trip() {
        let mut fixture = empty_shooting_fixture();
        fixture.shots.push(sample_shot("s1"));
        fixture.assets.push(sample_asset("a1"));
        let operation = ShootingOperation::SetActiveShot { shot_id: Some("s1".into()) };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next.active_shot_id, "s1");
        let operation = ShootingOperation::SetActiveAsset { asset_id: Some("a1".into()) };
        let next2 = round_trip(&next, &operation);
        assert_eq!(next2.active_asset_id, "a1");
    }

    #[test]
    fn set_camera_targets_fixture_camera_without_shot_reference() {
        let fixture = empty_shooting_fixture();
        let camera = ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() };
        let operation = ShootingOperation::SetCamera { camera: camera.clone() };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next.camera.position, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn set_camera_targets_saved_camera_when_active_shot_references_one() {
        let mut fixture = empty_shooting_fixture();
        fixture.saved_cameras.push(ShootingSavedCamera { id: "cam1".into(), label: "A".into(), camera: ShootingCamera::default() });
        let mut shot = sample_shot("s1");
        shot.camera_id = Some("cam1".into());
        fixture.shots.push(shot);
        fixture.active_shot_id = "s1".into();
        let camera = ShootingCamera { position: [9.0, 9.0, 9.0], ..Default::default() };
        let operation = ShootingOperation::SetCamera { camera };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next.saved_cameras[0].camera.position, [9.0, 9.0, 9.0]);
        assert_eq!(next.camera.position, fixture.camera.position, "fixture camera untouched when shot references saved camera");
    }

    #[test]
    fn patch_scene_round_trip() {
        let fixture = empty_shooting_fixture();
        let operation = ShootingOperation::PatchScene { patch: ShootingScenePatch { sun_azimuth: Some(90.0), shadow_enabled: Some(false), ..Default::default() } };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next.scene.sun.azimuth, 90.0);
        assert!(!next.scene.shadow.enabled);
    }

    #[test]
    fn translate_rotate_scale_assets_round_trip() {
        let mut fixture = empty_shooting_fixture();
        let mut asset = sample_asset("a1");
        // ScaleAssets always writes an explicit `Some([..])` scale, so backwards() restoring an
        // originally-`None` scale lands on `Some([1,1,1])` — the same effective scale (see
        // `shooting_asset_scale`) but not byte-identical. Start from an explicit identity scale so
        // the round-trip assertion checks real equality instead of that representation quirk.
        asset.scale = Some(serde_json::json!([1.0, 1.0, 1.0]));
        fixture.assets.push(asset);
        let translate = ShootingOperation::TranslateAssets { asset_ids: vec!["a1".into()], dx: 1.0, dy: 2.0, dz: 3.0 };
        let translated = round_trip(&fixture, &translate);
        assert_eq!(translated.assets[0].origin, [1.0, 2.0, 3.0]);

        let rotate = ShootingOperation::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 };
        let rotated = round_trip(&translated, &rotate);
        assert_ne!(rotated.assets[0].orientation, Some([0.0, 0.0, 0.0, 1.0]));

        let scale = ShootingOperation::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 };
        let scaled = round_trip(&rotated, &scale);
        assert_eq!(shooting_asset_scale(&scaled.assets[0]), [2.0, 2.0, 2.0]);
    }

    #[test]
    fn set_fixture_replaces_whole_document_and_restores() {
        let fixture = empty_shooting_fixture();
        let mut replacement = empty_shooting_fixture();
        replacement.assets.push(sample_asset("a1"));
        replacement.shots.push(sample_shot("s1"));
        let operation = ShootingOperation::SetFixture { fixture: replacement.clone() };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next, replacement);
    }

    #[test]
    fn coalesced_camera_drag_produces_one_edit() {
        let mut store = ShootingStore::new(create_document_vcs_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", empty_shooting_fixture(), None));
        store.dispatch(DocumentVcsCommand::AmendLast { operations: vec![ShootingOperation::SetCamera { camera: ShootingCamera { position: [1.0, 0.0, 0.0], ..Default::default() } }], coalesce_key: Some("camera".into()) }).expect("first drag tick");
        store.dispatch(DocumentVcsCommand::AmendLast { operations: vec![ShootingOperation::SetCamera { camera: ShootingCamera { position: [2.0, 0.0, 0.0], ..Default::default() } }], coalesce_key: Some("camera".into()) }).expect("second drag tick");
        assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced drag must produce exactly one edit");
        assert_eq!(store.projection().expect("projection").camera.position, [2.0, 0.0, 0.0]);
    }

    //#region 🔖DslAndOpText
    fn representative_fixture() -> ShootingFixture {
        ShootingFixture {
            schema: SHOOTING_FIXTURE_SCHEMA.into(),
            assets: vec![
                ShootingAsset {
                    id: "a1".into(),
                    name: "Base \"Mesh\"".into(),
                    url: "/mesh/a1.glb".into(),
                    format: "glb".into(),
                    origin: [1.0, 2.0, 3.0],
                    orientation: Some([0.0, 0.0, 0.7071, 0.7071]),
                    scale: Some(serde_json::json!([2.0, 2.0, 2.0])),
                },
                ShootingAsset { id: "a2".into(), name: "Plain".into(), url: "/mesh/a2.glb".into(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None },
            ],
            camera: ShootingCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], zoom: 1.5, fov: 45.0, up: Some([0.0, 0.0, 1.0]), projection: Some("perspective".into()) },
            saved_cameras: vec![ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera { position: [9.0, 9.0, 9.0], ..Default::default() } }],
            scene: ShootingSceneLighting {
                background: "#111111".into(),
                sun: ShootingSun { enabled: true, azimuth: 12.5, elevation: 33.0, intensity: 3.0, color: "#ff00ff".into() },
                ambient: ShootingAmbient { intensity: 0.9, color: "#00ffff".into() },
                shadow: ShootingShadow { enabled: false, opacity: 0.5, softness: 0.2 },
                material: ShootingMaterial { color: "#abcdef".into(), metalness: 0.3, roughness: 0.7, emissive: "#123456".into(), emissive_intensity: 0.1 },
                emblem_base64: Some("data:image/png;base64,abc==".into()),
            },
            shots: vec![
                ShootingShot { id: "s1".into(), label: "Overview".into(), width: 256, height: 256, format: "svg".into(), shape: "rectangle".into(), background: Some("#ffffff".into()), camera_id: Some("cam1".into()) },
                ShootingShot { id: "s2".into(), label: "Detail".into(), width: 512, height: 512, format: "png".into(), shape: "ellipse".into(), background: None, camera_id: None },
            ],
            active_shot_id: "s1".into(),
            active_asset_id: "a1".into(),
        }
    }

    #[test]
    fn shooting_dsl_round_trips_representative_fixture() {
        vcs::test_support::assert_dsl_round_trip(&representative_fixture());
    }

    #[test]
    fn shooting_dsl_round_trips_empty_fixture() {
        vcs::test_support::assert_dsl_round_trip(&empty_shooting_fixture());
    }

    #[test]
    fn shooting_dsl_round_trips_base_icon_example() {
        const BASE_ICON_EXAMPLE_DSL: &str = include_str!("../example/base-icon.shooting");
        let fixture = ShootingFixture::parse_dsl(BASE_ICON_EXAMPLE_DSL).expect("base-icon example parses");
        vcs::test_support::assert_dsl_round_trip(&fixture);
    }

    #[test]
    fn shooting_op_text_round_trips_collection_variants() {
        let asset = sample_asset("a1");
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::Assets(CollectionOperation::Add { index: 0, item: asset.clone() }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::Assets(CollectionOperation::Remove { id: "a1".into() }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::Assets(CollectionOperation::Move { id: "a1".into(), to_index: 2 }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::Assets(CollectionOperation::Patch {
            id: "a1".into(),
            patch: ShootingAssetPatch { name: Some("Renamed".into()), url: None, origin: Some([1.0, 2.0, 3.0]), orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: Some(serde_json::json!(2.5)) },
        }));

        let shot = sample_shot("s1");
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::Shots(CollectionOperation::Add { index: 0, item: shot.clone() }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::Shots(CollectionOperation::Remove { id: "s1".into() }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::Shots(CollectionOperation::Move { id: "s1".into(), to_index: 1 }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::Shots(CollectionOperation::Patch {
            id: "s1".into(),
            patch: ShootingShotPatch { label: Some("Hero".into()), width: Some(512), height: None, format: None, shape: Some("ellipse".into()) },
        }));

        let saved_camera = ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() };
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Add { index: 0, item: saved_camera.clone() }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Remove { id: "cam1".into() }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Move { id: "cam1".into(), to_index: 0 }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Patch {
            id: "cam1".into(),
            patch: ShootingSavedCameraPatch { label: Some("Renamed".into()), camera: Some(ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() }) },
        }));
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Patch { id: "cam1".into(), patch: ShootingSavedCameraPatch { label: None, camera: None } }));
    }

    #[test]
    fn shooting_op_text_round_trips_every_other_variant() {
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SetActiveShot { shot_id: Some("s1".into()) });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SetActiveShot { shot_id: None });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SetActiveAsset { asset_id: Some("a1".into()) });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SetActiveAsset { asset_id: None });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SetCamera {
            camera: ShootingCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], zoom: 2.0, fov: 60.0, up: Some([0.0, 1.0, 0.0]), projection: Some("orthographic".into()) },
        });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SetShotCamera { shot_id: "s1".into(), camera: ShootingCamera::default() });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::PatchScene {
            patch: ShootingScenePatch { sun_enabled: Some(true), sun_azimuth: Some(90.0), sun_elevation: None, sun_intensity: Some(1.0), ambient_intensity: None, shadow_enabled: Some(false), material_roughness: Some(0.4) },
        });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::TranslateAssets { asset_ids: vec!["a1".into(), "a2".into()], dx: 1.0, dy: -2.0, dz: 3.5 });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 });
        vcs::test_support::assert_op_line_round_trip(&ShootingOperation::SetFixture { fixture: representative_fixture() });
    }

    #[test]
    fn shooting_document_text_round_trips_store_with_applied_operation() {
        let mut store = ShootingStore::new(create_document_vcs_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", empty_shooting_fixture(), None));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![ShootingOperation::Assets(CollectionOperation::Add { index: 0, item: sample_asset("a1") })],
                description: None,
            })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DslAndOpText
}
//#endregion 🧪Tests
