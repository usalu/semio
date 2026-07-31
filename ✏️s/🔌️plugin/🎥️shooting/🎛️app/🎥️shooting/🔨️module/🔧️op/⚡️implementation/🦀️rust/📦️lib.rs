//! ⚡️ Shooting app — operation enum + laws (constitutional: op).

use shooting::{ShootingAsset, ShootingAssetPatch, ShootingCamera, ShootingFixture, ShootingSavedCamera, ShootingSavedCameraPatch, ShootingScenePatch, ShootingShot, ShootingShotPatch};
use protocol::{collection_diff_from_operation, CollectionDiff, CollectionOperation, Identified, ItemPatch, Operation, OperationDiff, Patchable};
use serde::{Deserialize, Serialize};

//#region 🔖️CollectionSupport
/// ▶️ Applies a `CollectionDiff` (removed → modified → added, matching `apply_collection_operation`'s
/// ordering) to an owned `Vec` — `protocol::CollectionDiff` has no generic apply helper of its own since
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

/// ➕️ Merges an incoming `CollectionDiff` into an existing one (coalescing two edits' diffs).
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
//#endregion 🔖️CollectionSupport

//#region 🔖️Operations
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
    /// 🎥️ Patches the saved camera `shot_id` references with `camera` — a no-op (empty diff) when that
    /// shot has no saved camera. The free/live viewport camera is session-only runtime state now (see
    /// `ShootingPlayRuntime::camera` in the ui crate) and never reaches this op enum at all.
    SetShotCamera { shot_id: String, camera: ShootingCamera },
    PatchScene { patch: ShootingScenePatch },
    TranslateAssets { asset_ids: Vec<String>, dx: f64, dy: f64, dz: f64 },
    RotateAssets { asset_ids: Vec<String>, ax: f64, ay: f64, az: f64, angle: f64 },
    ScaleAssets { asset_ids: Vec<String>, sx: f64, sy: f64, sz: f64 },
    SetFixture { fixture: ShootingFixture },
}

fn apply_scene_patch(scene: &mut shooting::ShootingSceneLighting, patch: &ShootingScenePatch) {
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

fn reverse_scene_patch(before: &shooting::ShootingSceneLighting, patch: &ShootingScenePatch) -> ShootingScenePatch {
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

/// 🎯️ Resolves which `SavedCameras` entry (if any) `shot_id` targets, for `SetShotCamera` diffing: a
/// shot referencing a saved camera patches that entry; a shot with no saved camera has nothing for
/// `SetShotCamera` to touch (the free/live viewport camera is session-only runtime state — see
/// `ShootingPlayRuntime::camera` in the ui crate — never a document field).
fn resolve_camera_target(fixture: &ShootingFixture, shot_id: &str) -> Option<String> {
    fixture.shots.iter().find(|shot| shot.id == shot_id).and_then(|shot| shot.camera_id.clone())
}

fn camera_diff_for_shot(fixture: &ShootingFixture, shot_id: &str, camera: &ShootingCamera) -> ShootingDiff {
    match resolve_camera_target(fixture, shot_id) {
        Some(camera_id) => {
            ShootingDiff { saved_cameras: Some(CollectionDiff { modified: vec![ItemPatch { id: camera_id, patch: ShootingSavedCameraPatch { label: None, camera: Some(camera.clone()) } }], ..Default::default() }), ..Default::default() }
        }
        None => ShootingDiff::default(),
    }
}

fn camera_for_shot(fixture: &ShootingFixture, shot_id: &str) -> Option<ShootingCamera> {
    let camera_id = resolve_camera_target(fixture, shot_id)?;
    fixture.saved_cameras.iter().find(|entry| entry.id == camera_id).map(|entry| entry.camera.clone())
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
            ShootingOperation::SetShotCamera { shot_id, camera } => camera_diff_for_shot(projection, shot_id, camera),
            ShootingOperation::PatchScene { patch } => ShootingDiff { scene: Some(patch.clone()), ..Default::default() },
            ShootingOperation::TranslateAssets { asset_ids, dx, dy, dz } => transform_assets_diff(projection, asset_ids, |asset| ShootingAssetPatch { origin: Some([asset.origin[0] + dx, asset.origin[1] + dy, asset.origin[2] + dz]), ..Default::default() }),
            ShootingOperation::RotateAssets { asset_ids, ax, ay, az, angle } => {
                let delta = shooting::quat_from_axis_angle(*ax, *ay, *az, *angle);
                transform_assets_diff(projection, asset_ids, |asset| {
                    let current = asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    ShootingAssetPatch { orientation: Some(shooting::quat_mul(delta, current)), ..Default::default() }
                })
            }
            ShootingOperation::ScaleAssets { asset_ids, sx, sy, sz } => transform_assets_diff(projection, asset_ids, |asset| {
                let current = shooting::shooting_asset_scale(asset);
                ShootingAssetPatch { scale: Some([current[0] * sx, current[1] * sy, current[2] * sz]), ..Default::default() }
            }),
            ShootingOperation::SetFixture { fixture } => ShootingDiff { fixture: Some(fixture.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &ShootingFixture) -> Vec<Self> {
        match self {
            ShootingOperation::Assets(operation) => vec![ShootingOperation::Assets(protocol::invert_collection_operation(&projection.assets, operation))],
            ShootingOperation::Shots(operation) => vec![ShootingOperation::Shots(protocol::invert_collection_operation(&projection.shots, operation))],
            ShootingOperation::SavedCameras(operation) => {
                vec![ShootingOperation::SavedCameras(protocol::invert_collection_operation(&projection.saved_cameras, operation))]
            }
            ShootingOperation::SetActiveShot { .. } => vec![ShootingOperation::SetActiveShot { shot_id: if projection.active_shot_id.is_empty() { None } else { Some(projection.active_shot_id.clone()) } }],
            ShootingOperation::SetActiveAsset { .. } => vec![ShootingOperation::SetActiveAsset { asset_id: if projection.active_asset_id.is_empty() { None } else { Some(projection.active_asset_id.clone()) } }],
            ShootingOperation::SetShotCamera { shot_id, .. } => camera_for_shot(projection, shot_id).map(|camera| vec![ShootingOperation::SetShotCamera { shot_id: shot_id.clone(), camera }]).unwrap_or_default(),
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
//#endregion 🔖️Operations

//#region 🔖️OpText
/// 🌿️ `ShootingFixture`'s three collections (`assets`/`shots`/`saved_cameras`) are `Vec<T>` of a
/// plain `#[derive(dsl::DslRecord)]` struct, but `#[dsl(statements, block)]` needs its element type
/// to implement `dsl::DslVariants` (enum-only) — these one-variant newtype-tuple wrappers close
/// that gap without duplicating any field: the newtype-tuple codegen delegates entirely to the
/// wrapped type's own `RecordSpec` (including its `#[dsl(keyword = "...")]`), so `ShootingAsset`
/// prints/parses byte-identically whether reached through `ShootingAssetNode` or on its own. Only
/// consumed by `ShootingOperationDsl` below, so they live here (in `op`) rather than in the fixture
/// DSL mirror.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum ShootingAssetNode {
    Asset(ShootingAsset),
}

#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum ShootingShotNode {
    Shot(ShootingShot),
}

/// 🔡️ The variant key must stay textually in sync with `ShootingSavedCamera`'s own
/// `#[dsl(keyword = "saved-camera")]` — this newtype delegates its `RecordSpec` entirely to the
/// wrapped type (see the doc comment above), so the `Shape::Statements` dispatcher's outer tag
/// (this key) and `parse_record_body`'s inner leading-keyword check (the struct's `keyword`) must
/// agree, or parsing that variant would require two different tokens where only one is written.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum ShootingSavedCameraNode {
    #[dsl(key = "saved-camera")]
    SavedCamera(ShootingSavedCamera),
}

/// 📄️ Local mirror of `ShootingFixture` for `SetFixture`'s `#[dsl(block)]` payload — reuses the
/// derive-generated shape, independent of `shooting`'s own private fixture mirror.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "shooting")]
#[dsl(layout = "lines")]
struct ShootingFixtureDsl {
    schema: String,
    active_shot_id: String,
    active_asset_id: String,
    #[dsl(block)]
    scene: shooting::ShootingSceneLighting,
    #[dsl(table)]
    assets: Vec<ShootingAsset>,
    #[dsl(table)]
    shots: Vec<ShootingShot>,
    #[dsl(table)]
    saved_cameras: Vec<ShootingSavedCamera>,
}

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

/// ⚡️ Local mirror of `ShootingOperation` — the real enum's `Assets`/`Shots`/`SavedCameras` variants
/// each wrap a single `protocol::CollectionOperation<..>` field, a foreign generic type (orphan rule:
/// can't `impl dsl::DslField` for it here) that also isn't the tagged-enum shape `#[derive(dsl::DslOps)]`
/// needs anyway — so each `CollectionOperation` variant (`Add`/`Remove`/`Move`/`Patch`) is flattened
/// into its own DSL-facing operation variant instead, exactly the `imperative::ImperativeOperationDsl`
/// idiom.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
#[allow(clippy::large_enum_variant, reason = "mirror-only enum used solely at the print_op/parse_op boundary, never stored or passed around")]
enum ShootingOperationDsl {
    AssetsAdd {
        index: usize,
        #[dsl(statements)]
        item: Box<ShootingAssetNode>,
    },
    AssetsRemove { id: String },
    AssetsMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    AssetsPatch {
        id: String,
        #[dsl(block)]
        patch: ShootingAssetPatch,
    },
    ShotsAdd {
        index: usize,
        #[dsl(statements)]
        item: Box<ShootingShotNode>,
    },
    ShotsRemove { id: String },
    ShotsMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    ShotsPatch {
        id: String,
        #[dsl(block)]
        patch: ShootingShotPatch,
    },
    SavedCamerasAdd {
        index: usize,
        #[dsl(statements)]
        item: Box<ShootingSavedCameraNode>,
    },
    SavedCamerasRemove { id: String },
    SavedCamerasMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    SavedCamerasPatch {
        id: String,
        #[dsl(block)]
        patch: ShootingSavedCameraPatch,
    },
    #[dsl(key = "active-shot")]
    SetActiveShot { shot_id: Option<String> },
    #[dsl(key = "active-asset")]
    SetActiveAsset { asset_id: Option<String> },
    #[dsl(key = "shot-camera")]
    SetShotCamera {
        shot_id: String,
        #[dsl(block)]
        camera: ShootingCamera,
    },
    #[dsl(key = "scene-patch")]
    PatchScene {
        #[dsl(block)]
        patch: ShootingScenePatch,
    },
    #[dsl(key = "translate")]
    TranslateAssets { asset_ids: Vec<String>, dx: f64, dy: f64, dz: f64 },
    #[dsl(key = "rotate")]
    RotateAssets { asset_ids: Vec<String>, ax: f64, ay: f64, az: f64, angle: f64 },
    #[dsl(key = "scale")]
    ScaleAssets { asset_ids: Vec<String>, sx: f64, sy: f64, sz: f64 },
    #[dsl(key = "fixture")]
    SetFixture {
        #[dsl(block)]
        fixture: ShootingFixtureDsl,
    },
}

fn shooting_operation_to_dsl(operation: &ShootingOperation) -> ShootingOperationDsl {
    match operation {
        ShootingOperation::Assets(op) => match op {
            CollectionOperation::Add { id: _id, item, at } => ShootingOperationDsl::AssetsAdd { index: *at, item: Box::new(ShootingAssetNode::Asset(item.clone())) },
            CollectionOperation::Remove { id } => ShootingOperationDsl::AssetsRemove { id: id.clone() },
            CollectionOperation::Move { id, to } => ShootingOperationDsl::AssetsMove { id: id.clone(), to_index: *to },
            CollectionOperation::Patch { id, patch } => ShootingOperationDsl::AssetsPatch { id: id.clone(), patch: patch.clone() },
        },
        ShootingOperation::Shots(op) => match op {
            CollectionOperation::Add { id: _id, item, at } => ShootingOperationDsl::ShotsAdd { index: *at, item: Box::new(ShootingShotNode::Shot(item.clone())) },
            CollectionOperation::Remove { id } => ShootingOperationDsl::ShotsRemove { id: id.clone() },
            CollectionOperation::Move { id, to } => ShootingOperationDsl::ShotsMove { id: id.clone(), to_index: *to },
            CollectionOperation::Patch { id, patch } => ShootingOperationDsl::ShotsPatch { id: id.clone(), patch: patch.clone() },
        },
        ShootingOperation::SavedCameras(op) => match op {
            CollectionOperation::Add { id: _id, item, at } => ShootingOperationDsl::SavedCamerasAdd { index: *at, item: Box::new(ShootingSavedCameraNode::SavedCamera(item.clone())) },
            CollectionOperation::Remove { id } => ShootingOperationDsl::SavedCamerasRemove { id: id.clone() },
            CollectionOperation::Move { id, to } => ShootingOperationDsl::SavedCamerasMove { id: id.clone(), to_index: *to },
            CollectionOperation::Patch { id, patch } => ShootingOperationDsl::SavedCamerasPatch { id: id.clone(), patch: patch.clone() },
        },
        ShootingOperation::SetActiveShot { shot_id } => ShootingOperationDsl::SetActiveShot { shot_id: shot_id.clone() },
        ShootingOperation::SetActiveAsset { asset_id } => ShootingOperationDsl::SetActiveAsset { asset_id: asset_id.clone() },
        ShootingOperation::SetShotCamera { shot_id, camera } => ShootingOperationDsl::SetShotCamera { shot_id: shot_id.clone(), camera: camera.clone() },
        ShootingOperation::PatchScene { patch } => ShootingOperationDsl::PatchScene { patch: patch.clone() },
        ShootingOperation::TranslateAssets { asset_ids, dx, dy, dz } => ShootingOperationDsl::TranslateAssets { asset_ids: asset_ids.clone(), dx: *dx, dy: *dy, dz: *dz },
        ShootingOperation::RotateAssets { asset_ids, ax, ay, az, angle } => ShootingOperationDsl::RotateAssets { asset_ids: asset_ids.clone(), ax: *ax, ay: *ay, az: *az, angle: *angle },
        ShootingOperation::ScaleAssets { asset_ids, sx, sy, sz } => ShootingOperationDsl::ScaleAssets { asset_ids: asset_ids.clone(), sx: *sx, sy: *sy, sz: *sz },
        ShootingOperation::SetFixture { fixture } => ShootingOperationDsl::SetFixture { fixture: shooting_fixture_to_dsl(fixture) },
    }
}

fn shooting_operation_from_dsl(dsl_op: ShootingOperationDsl) -> ShootingOperation {
    match dsl_op {
        ShootingOperationDsl::AssetsAdd { index, item } => {
            let ShootingAssetNode::Asset(asset) = *item;
            ShootingOperation::Assets(CollectionOperation::Add { id: asset.id.clone(), item: asset, at: index })
        }
        ShootingOperationDsl::AssetsRemove { id } => ShootingOperation::Assets(CollectionOperation::Remove { id }),
        ShootingOperationDsl::AssetsMove { id, to_index } => ShootingOperation::Assets(CollectionOperation::Move { id, to: to_index }),
        ShootingOperationDsl::AssetsPatch { id, patch } => ShootingOperation::Assets(CollectionOperation::Patch { id, patch }),
        ShootingOperationDsl::ShotsAdd { index, item } => {
            let ShootingShotNode::Shot(shot) = *item;
            ShootingOperation::Shots(CollectionOperation::Add { id: shot.id.clone(), item: shot, at: index })
        }
        ShootingOperationDsl::ShotsRemove { id } => ShootingOperation::Shots(CollectionOperation::Remove { id }),
        ShootingOperationDsl::ShotsMove { id, to_index } => ShootingOperation::Shots(CollectionOperation::Move { id, to: to_index }),
        ShootingOperationDsl::ShotsPatch { id, patch } => ShootingOperation::Shots(CollectionOperation::Patch { id, patch }),
        ShootingOperationDsl::SavedCamerasAdd { index, item } => {
            let ShootingSavedCameraNode::SavedCamera(entry) = *item;
            ShootingOperation::SavedCameras(CollectionOperation::Add { id: entry.id.clone(), item: entry, at: index })
        }
        ShootingOperationDsl::SavedCamerasRemove { id } => ShootingOperation::SavedCameras(CollectionOperation::Remove { id }),
        ShootingOperationDsl::SavedCamerasMove { id, to_index } => ShootingOperation::SavedCameras(CollectionOperation::Move { id, to: to_index }),
        ShootingOperationDsl::SavedCamerasPatch { id, patch } => ShootingOperation::SavedCameras(CollectionOperation::Patch { id, patch }),
        ShootingOperationDsl::SetActiveShot { shot_id } => ShootingOperation::SetActiveShot { shot_id },
        ShootingOperationDsl::SetActiveAsset { asset_id } => ShootingOperation::SetActiveAsset { asset_id },
        ShootingOperationDsl::SetShotCamera { shot_id, camera } => ShootingOperation::SetShotCamera { shot_id, camera },
        ShootingOperationDsl::PatchScene { patch } => ShootingOperation::PatchScene { patch },
        ShootingOperationDsl::TranslateAssets { asset_ids, dx, dy, dz } => ShootingOperation::TranslateAssets { asset_ids, dx, dy, dz },
        ShootingOperationDsl::RotateAssets { asset_ids, ax, ay, az, angle } => ShootingOperation::RotateAssets { asset_ids, ax, ay, az, angle },
        ShootingOperationDsl::ScaleAssets { asset_ids, sx, sy, sz } => ShootingOperation::ScaleAssets { asset_ids, sx, sy, sz },
        ShootingOperationDsl::SetFixture { fixture } => ShootingOperation::SetFixture { fixture: shooting_fixture_from_dsl(fixture) },
    }
}

impl protocol::OpText for ShootingOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(shooting_operation_from_dsl(<ShootingOperationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <ShootingOperationDsl as protocol::OpText>::print_op(&shooting_operation_to_dsl(self))
    }
}

/// @emoji 🎞️ Binary mirror of the `OpText` bridge above — `ShootingOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for ShootingOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        shooting_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(shooting_operation_from_dsl(ShootingOperationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use shooting::SHOOTING_FIXTURE_SCHEMA;

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

    /// 🎞️ A fixture exercising every field/variant — duplicated verbatim across the `dsl`/`op`/`pack`
    /// crates' test modules (each is its own compilation unit, so a shared cross-crate test-only
    /// helper isn't worth a dependency).
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
                    scale: Some([2.0, 2.0, 2.0]),
                },
                ShootingAsset { id: "a2".into(), name: "Plain".into(), url: "/mesh/a2.glb".into(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None },
            ],
            saved_cameras: vec![ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera { position: [9.0, 9.0, 9.0], ..Default::default() } }],
            scene: shooting::ShootingSceneLighting {
                background: "#111111".into(),
                sun: shooting::ShootingSun { enabled: true, azimuth: 12.5, elevation: 33.0, intensity: 3.0, color: "#ff00ff".into() },
                ambient: shooting::ShootingAmbient { intensity: 0.9, color: "#00ffff".into() },
                shadow: shooting::ShootingShadow { enabled: false, opacity: 0.5, softness: 0.2 },
                material: shooting::ShootingMaterial { color: "#abcdef".into(), metalness: 0.3, roughness: 0.7, emissive: "#123456".into(), emissive_intensity: 0.1 },
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
    fn assets_add_remove_patch_round_trip() {
        let fixture = shooting::empty_shooting_fixture();
        let add = ShootingOperation::Assets(CollectionOperation::Add { id: "a1".into(), item: sample_asset("a1"), at: 0 });
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
        let mut fixture = shooting::empty_shooting_fixture();
        fixture.shots.push(sample_shot("s1"));
        let patch = ShootingOperation::Shots(CollectionOperation::Patch { id: "s1".into(), patch: ShootingShotPatch { label: Some("Hero".into()), width: Some(512), ..Default::default() } });
        let patched = round_trip(&fixture, &patch);
        assert_eq!(patched.shots[0].label, "Hero");
        assert_eq!(patched.shots[0].width, 512);
    }

    #[test]
    fn saved_cameras_add_round_trip() {
        let fixture = shooting::empty_shooting_fixture();
        let add = ShootingOperation::SavedCameras(CollectionOperation::Add { id: "cam1".into(), item: ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() }, at: 0 });
        let added = round_trip(&fixture, &add);
        assert_eq!(added.saved_cameras.len(), 1);
    }

    #[test]
    fn set_active_shot_and_asset_round_trip() {
        let mut fixture = shooting::empty_shooting_fixture();
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
    fn set_shot_camera_is_a_no_op_when_shot_has_no_saved_camera() {
        // 🎥️ The free/live viewport camera is session-only runtime state now (never a document field) —
        // `SetShotCamera` against a shot with no saved-camera reference has nothing to patch.
        let mut fixture = shooting::empty_shooting_fixture();
        fixture.shots.push(sample_shot("s1"));
        let camera = ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() };
        let operation = ShootingOperation::SetShotCamera { shot_id: "s1".into(), camera };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next, fixture, "no saved camera referenced by the shot means no document change");
    }

    #[test]
    fn set_shot_camera_patches_the_saved_camera_it_references() {
        let mut fixture = shooting::empty_shooting_fixture();
        fixture.saved_cameras.push(ShootingSavedCamera { id: "cam1".into(), label: "A".into(), camera: ShootingCamera::default() });
        let mut shot = sample_shot("s1");
        shot.camera_id = Some("cam1".into());
        fixture.shots.push(shot);
        fixture.active_shot_id = "s1".into();
        let camera = ShootingCamera { position: [9.0, 9.0, 9.0], ..Default::default() };
        let operation = ShootingOperation::SetShotCamera { shot_id: "s1".into(), camera };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next.saved_cameras[0].camera.position, [9.0, 9.0, 9.0]);
    }

    #[test]
    fn patch_scene_round_trip() {
        let fixture = shooting::empty_shooting_fixture();
        let operation = ShootingOperation::PatchScene { patch: ShootingScenePatch { sun_azimuth: Some(90.0), shadow_enabled: Some(false), ..Default::default() } };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next.scene.sun.azimuth, 90.0);
        assert!(!next.scene.shadow.enabled);
    }

    #[test]
    fn translate_rotate_scale_assets_round_trip() {
        let mut fixture = shooting::empty_shooting_fixture();
        let mut asset = sample_asset("a1");
        // ScaleAssets always writes an explicit `Some([..])` scale, so backwards() restoring an
        // originally-`None` scale lands on `Some([1,1,1])` — the same effective scale (see
        // `shooting_asset_scale`) but not byte-identical. Start from an explicit identity scale so
        // the round-trip assertion checks real equality instead of that representation quirk.
        asset.scale = Some([1.0, 1.0, 1.0]);
        fixture.assets.push(asset);
        let translate = ShootingOperation::TranslateAssets { asset_ids: vec!["a1".into()], dx: 1.0, dy: 2.0, dz: 3.0 };
        let translated = round_trip(&fixture, &translate);
        assert_eq!(translated.assets[0].origin, [1.0, 2.0, 3.0]);

        let rotate = ShootingOperation::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 };
        let rotated = round_trip(&translated, &rotate);
        assert_ne!(rotated.assets[0].orientation, Some([0.0, 0.0, 0.0, 1.0]));

        let scale = ShootingOperation::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 };
        let scaled = round_trip(&rotated, &scale);
        assert_eq!(shooting::shooting_asset_scale(&scaled.assets[0]), [2.0, 2.0, 2.0]);
    }

    #[test]
    fn set_fixture_replaces_whole_document_and_restores() {
        let fixture = shooting::empty_shooting_fixture();
        let mut replacement = shooting::empty_shooting_fixture();
        replacement.assets.push(sample_asset("a1"));
        replacement.shots.push(sample_shot("s1"));
        let operation = ShootingOperation::SetFixture { fixture: replacement.clone() };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next, replacement);
    }

    #[test]
    fn shooting_op_text_round_trips_collection_variants() {
        let asset = sample_asset("a1");
        store::test_support::assert_op_line_round_trip(&ShootingOperation::Assets(CollectionOperation::Add { id: "a1".into(), item: asset.clone(), at: 0 }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::Assets(CollectionOperation::Remove { id: "a1".into() }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::Assets(CollectionOperation::Move { id: "a1".into(), to: 2 }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::Assets(CollectionOperation::Patch {
            id: "a1".into(),
            patch: ShootingAssetPatch { name: Some("Renamed".into()), url: None, origin: Some([1.0, 2.0, 3.0]), orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: Some([2.5, 2.5, 2.5]) },
        }));

        let shot = sample_shot("s1");
        store::test_support::assert_op_line_round_trip(&ShootingOperation::Shots(CollectionOperation::Add { id: "s1".into(), item: shot.clone(), at: 0 }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::Shots(CollectionOperation::Remove { id: "s1".into() }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::Shots(CollectionOperation::Move { id: "s1".into(), to: 1 }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::Shots(CollectionOperation::Patch {
            id: "s1".into(),
            patch: ShootingShotPatch { label: Some("Hero".into()), width: Some(512), height: None, format: None, shape: Some("ellipse".into()) },
        }));

        let saved_camera = ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() };
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Add { id: "cam1".into(), item: saved_camera.clone(), at: 0 }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Remove { id: "cam1".into() }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Move { id: "cam1".into(), to: 0 }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Patch {
            id: "cam1".into(),
            patch: ShootingSavedCameraPatch { label: Some("Renamed".into()), camera: Some(ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() }) },
        }));
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SavedCameras(CollectionOperation::Patch { id: "cam1".into(), patch: ShootingSavedCameraPatch { label: None, camera: None } }));
    }

    #[test]
    fn shooting_op_text_round_trips_every_other_variant() {
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SetActiveShot { shot_id: Some("s1".into()) });
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SetActiveShot { shot_id: None });
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SetActiveAsset { asset_id: Some("a1".into()) });
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SetActiveAsset { asset_id: None });
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SetShotCamera { shot_id: "s1".into(), camera: ShootingCamera::default() });
        store::test_support::assert_op_line_round_trip(&ShootingOperation::PatchScene {
            patch: ShootingScenePatch { sun_enabled: Some(true), sun_azimuth: Some(90.0), sun_elevation: None, sun_intensity: Some(1.0), ambient_intensity: None, shadow_enabled: Some(false), material_roughness: Some(0.4) },
        });
        store::test_support::assert_op_line_round_trip(&ShootingOperation::TranslateAssets { asset_ids: vec!["a1".into(), "a2".into()], dx: 1.0, dy: -2.0, dz: 3.5 });
        store::test_support::assert_op_line_round_trip(&ShootingOperation::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 });
        store::test_support::assert_op_line_round_trip(&ShootingOperation::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 });
        store::test_support::assert_op_line_round_trip(&ShootingOperation::SetFixture { fixture: representative_fixture() });
    }
}
//#endregion 🧪️Tests
