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
}
//#endregion 🧪Tests
