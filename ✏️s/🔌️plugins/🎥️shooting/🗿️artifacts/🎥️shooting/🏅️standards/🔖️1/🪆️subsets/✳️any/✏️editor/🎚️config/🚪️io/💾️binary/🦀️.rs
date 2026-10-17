//! 🚪️ Artifact binary representations.


//#region 🧩️ArtifactItems
pub mod artifact_items {
pub const SHOOTING_DOCUMENT_SCHEMA: &str = "shooting.shooting";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use semio_framework_dsl_record_derive::DslRecord;
use protocol::Identified;

//#region 🔖️Domain
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ShootingCamera {
    #[cfg_attr(test, serde(default = "default_camera_position"))]
    #[value(default = "default_camera_position")]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[cfg_attr(test, serde(default = "default_camera_target"))]
    #[value(default = "default_camera_target")]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[cfg_attr(test, serde(default = "one_f64"))]
    #[value(default = "one_f64")]
    pub zoom: f64,
    #[cfg_attr(test, serde(default = "default_fov"))]
    #[value(default = "default_fov")]
    #[dsl(angle = "deg")]
    pub fov: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(dir)]
    pub up: Option<[f64; 3]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
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

pub fn one_f64() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "saved-camera")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ShootingSavedCamera {
    #[dsl(defines = "saved-camera")]
    pub id: String,
    pub label: String,
    #[dsl(block)]
    pub camera: ShootingCamera,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "asset")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ShootingAsset {
    pub id: String,
    pub name: String,
    pub url: String,
    #[cfg_attr(test, serde(default = "default_glb_format"))]
    #[value(default = "default_glb_format")]
    pub format: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    /// 🪄️ Uniform-vs-per-axis is a JSON-authoring shorthand only, not a persisted distinction —
    /// callers wanting a uniform scale write `[s, s, s]` (see `shooting_asset_scale`, the sole
    /// reader, which never distinguished the two shapes anyway).
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f64; 3]>,
}

pub fn default_glb_format() -> String {
    "glb".into()
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "shot")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ShootingShot {
    pub id: String,
    pub label: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub shape: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "saved-camera")]
    pub camera_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct ShootingAmbient {
    pub intensity: f64,
    pub color: String,
}

impl Default for ShootingAmbient {
    fn default() -> Self {
        Self { intensity: 1.15, color: "#ffffff".into() }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct ShootingMaterial {
    pub color: String,
    pub metalness: f64,
    pub roughness: f64,
    pub emissive: String,
    pub emissive_intensity: f64,
    pub stroke: String,
}

impl Default for ShootingMaterial {
    fn default() -> Self {
        Self { color: "#9aa0ab".into(), metalness: 0.0, roughness: 1.0, emissive: "#000000".into(), emissive_intensity: 0.0, stroke: "#000000".into() }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ShootingSceneLighting {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub background: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(block)]
    pub sun: ShootingSun,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(block)]
    pub ambient: ShootingAmbient,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(block)]
    pub shadow: ShootingShadow,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(block)]
    pub material: ShootingMaterial,
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
//#endregion 🔖️Domain
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{ShootingCamera, ShootingShot};
pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;

//#region 🔖️SnapshotHelpers
pub fn empty_shooting_snapshot() -> ShootingSnapshot {
    ShootingSnapshot::default()
}

/// 🎯️ Resolves the effective camera for `shot`: the saved camera it references, or `fallback` — the
/// app's session-only live camera (never a document field; see `ShootingConfig::camera` in the app's
/// `🦀️config.rs`) when the shot has no saved camera of its own.
pub fn shooting_resolve_shot_camera(snapshot: &ShootingSnapshot, shot: &ShootingShot, fallback: &ShootingCamera) -> ShootingCamera {
    shot.camera_id.as_ref().and_then(|camera_id| snapshot.saved_cameras.iter().find(|entry| &entry.id == camera_id)).map_or_else(|| fallback.clone(), |entry| entry.camera.clone())
}
//#endregion 🔖️SnapshotHelpers
}
//#endregion 🧩️ArtifactItems6

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;

//#region 🔖️Composition
/// 🧩️ Composed `s.stdio.semio.image` child slot for the scene's emblem overlay (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, `📓️design-full-plan.md` §4:
/// `shooting→C:video,image,audio,table`). Of that four-subset menu, ONLY `image` maps onto real
/// duplicated content in this plugin — the former `ShootingSceneLighting.emblem_base64:
/// Option<String>` (a raw base64 PNG overlay) duplicated exactly what `s.stdio.semio.image` already
/// generalizes.
///
/// `video`/`audio` are deliberately NOT composed: an exhaustive grep of every `.rs` file in this
/// plugin for `video`/`Video`/`audio`/`Audio`/`recording`/`capture`/`footage`/`waveform`/`clip`/
/// `take` returned zero hits. This "shooting" plugin is a 3D icon/product-render studio (assets,
/// saved cameras, a shot list, scene lighting) — never a literal video/audio recorder — so there is
/// no inline content of either shape to kill; composing empty stub children for them would invent
/// vocabulary the plugin has no use for, which `📌️important.md`'s own "leave a facet empty rather
/// than fabricate" precedent (for mutation triads) argues against by direct analogy. `📓️gismap`'s
/// own `image` slot ("honestly always absent... the slot exists, real and typed, for the day a
/// basemap capture lands, not as a stub") is the closest precedent for "compose only what's real."
///
/// `table` is NOT composed either, despite `assets`/`shots`/`savedCameras` being row-shaped
/// collections: each already carries its own real, granular, non-duplicative collection diff
/// (`ShootingAssetsDelta`/`ShootingShotsDelta`/`ShootingSavedCamerasDelta` — `added`/`removed`/
/// `patched`/`reordered`, see `🔺️diff/🦀️.rs`), sparse-built directly from `(payload, base)`
/// at every one of their mutation triads — exactly the shape `📌️important.md`'s own D2/Concern-B
/// section asks the REST of the repo to move TOWARD, not away from. Flattening any of them into a
/// single composed `SemioTableSnapshot` child would force a whole-handle-replace diff shape (the
/// migration recipe §8's "always-present slot" convention), regressing real per-row add/remove/
/// patch/reorder granularity into an all-or-nothing re-mint on every edit — a concrete technical
/// reason to decline, per the recipe's own allowance ("unless you find a concrete technical reason
/// they can't — document precisely if so, don't generalize from one blocked field to the whole
/// plugin").
use semio_s_artifact_stdio_semio_image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};

// 🖼️ Emblem codec of the shooting artifact: the composed image child slot, its base64 codec and the emblem accessors. See [`super`].


//#region 🔖️Composition
pub type ShootingEmblemChild = store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>;

/// 🪪️ Mint a deterministic, content-addressed `s.stdio.semio.image` CHILD HANDLE from `content` —
/// same `store::os_store::artifact_child::ArtifactChild::new`/`ArtifactDialect` shape as every other wave-4 exemplar
/// (`process3d::brep_child_handle`, `gismap::gis_map_drawing_child_handle`). Two callers with
/// byte-identical content mint the same handle.
pub fn shooting_emblem_child_handle(content: &SemioImageSnapshot) -> ShootingEmblemChild {
    let child_id = store::content_id("shooting-emblem", semio_framework_pack_json::to_json_string(content).as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "shooting-emblem".into(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🌉️ WRITE direction, real (not a stub): wraps raw emblem bytes verbatim as a single opaque
/// `SemioImageFrame`. HONEST BOUNDARY, not a shortcut: this plugin never decoded the emblem's PNG
/// pixels before this migration either — `shooting_scene_to_semio_drawing` (`🧬️schema/🦀️component.rs`),
/// the sole reader, passed the raw base64-decoded bytes straight into a `DrawNode::Image` for
/// re-encoding, never inspecting a single channel value. Decoding real PNG pixel data into `image`'s
/// normal decoded-RGBA8 convention is a new capability this migration does not add. Round-trips
/// exactly (byte-identical) because encode/decode never interpret the bytes as pixels, only
/// store+retrieve them verbatim; `width`/`height`/`bit_depth` are `0` (genuinely unknown pre-decode)
/// and one metadata entry (`encoding=opaque-bytes`) records the honest boundary for any future reader.
pub fn shooting_emblem_image_from_bytes(bytes: Vec<u8>) -> SemioImageSnapshot {
    SemioImageSnapshot {
        schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
        width: 0,
        height: 0,
        colorspace: SemioColorspace::default(),
        bit_depth: 0,
        frames: vec![SemioImageFrame { delay_ms: 0, rgba8: bytes }],
        icc: None,
        metadata: vec![SemioImageMetadataEntry { key: "encoding".into(), value: "opaque-bytes".into() }],
    }
}

/// 🌉️ READ direction, real: the exact inverse of `shooting_emblem_image_from_bytes` — the first
/// (only) frame's verbatim bytes, or empty if the image carries no frame.
pub fn shooting_emblem_bytes_from_image(image: &SemioImageSnapshot) -> Vec<u8> {
    image.frames.first().map(|frame| frame.rgba8.clone()).unwrap_or_default()
}

/// 🔤️ Minimal, dependency-free base64 DECODE for the emblem's raw base64 payload — moved here from
/// the schema module alongside the rest of the emblem's composition machinery (every leaf in this
/// codebase hand-rolls this exact algorithm rather than pull in an external crate).
pub fn shooting_base64_decode(data: &str) -> Option<Vec<u8>> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let clean: Vec<u8> = data.bytes().filter(|&b| b != b'=' && !b.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    for chunk in clean.chunks(4) {
        let vals: Vec<u8> = chunk.iter().map(|&b| val(b)).collect::<Option<Vec<u8>>>()?;
        let n = vals.len();
        let combined = vals.iter().fold(0u32, |acc, &v| (acc << 6) | v as u32) << ((4 - n) * 6);
        out.push((combined >> 16) as u8);
        if n > 2 {
            out.push((combined >> 8) as u8);
        }
        if n > 3 {
            out.push(combined as u8);
        }
    }
    Some(out)
}

/// 🔤️ The ENCODE direction — standard base64 with `=` padding, the exact inverse alphabet of
/// `shooting_base64_decode`. New this migration (the pre-migration field only ever needed decode).
pub fn shooting_base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        let n = (b0 as u32) << 16 | (b1 as u32) << 8 | b2 as u32;
        out.push(ALPHABET[(n >> 18 & 0x3f) as usize] as char);
        out.push(ALPHABET[(n >> 12 & 0x3f) as usize] as char);
        out.push(if chunk.len() > 1 { ALPHABET[(n >> 6 & 0x3f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[(n & 0x3f) as usize] as char } else { '=' });
    }
    out
}

/// 🌉️ WRITE direction, real: decodes `base64` (the plugin's existing raw-base64-payload convention,
/// never a `data:` URI prefix — matches the pre-migration `emblem_base64` field's own callers),
/// mints a content-addressed handle, attaches the immutable content to that exact child owner, and
/// sets `snapshot.emblem`. Independent snapshots cannot observe or replace one another's emblem.
/// `None`/empty input clears the slot.
pub fn shooting_set_emblem_from_base64(snapshot: &mut ShootingSnapshot, base64: Option<&str>) {
    let bytes = base64.filter(|data| !data.is_empty()).and_then(shooting_base64_decode);
    match bytes {
        None => snapshot.emblem = None,
        Some(bytes) => {
            let content = shooting_emblem_image_from_bytes(bytes);
            let handle = shooting_emblem_child_handle(&content).with_local_owner(std::sync::Arc::new(content));
            snapshot.emblem = Some(handle);
        }
    }
}

/// 🌉️ READ direction: retains the materialization owned by this exact child handle; degrades to
/// `None` when a wire-decoded handle has not yet been materialized by the host child resolver.
pub fn shooting_emblem_image(snapshot: &ShootingSnapshot) -> Option<SemioImageSnapshot> {
    let handle = snapshot.emblem.as_ref()?;
    handle.local_owner::<SemioImageSnapshot>().map(|content| content.as_ref().clone())
}

/// 🌉️ `shooting_emblem_image` + `shooting_emblem_bytes_from_image` in one call — the accessor
/// `shooting_scene_to_semio_drawing` (`🧬️schema/🦀️component.rs`) funnels through instead of the old
/// direct `snapshot.scene.emblem_base64` field read.
pub fn shooting_emblem_bytes(snapshot: &ShootingSnapshot) -> Option<Vec<u8>> {
    shooting_emblem_image(snapshot).map(|image| shooting_emblem_bytes_from_image(&image))
}
//#endregion 🔖️Composition
}
//#endregion 🧩️ArtifactItems7

//#region 🧩️ArtifactItems9
pub mod artifact_items_9 {
use crate::{ShootingAsset, ShootingShot};
pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;

//#region 🔖️DocumentHelpers
/// 🔢️ Mints a fresh, process-unique id (`"{prefix}-{n}"`) — shared by every mutation that creates a
/// new shot/asset/saved-camera record.
pub fn next_shooting_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let next = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{prefix}-{next}")
}





/// 📸️ The active shot — falls back to the first shot when `active_shot_id` names nothing (an empty
/// document, or a stale id left over after a delete).
pub fn active_shot(snapshot: &ShootingSnapshot) -> Option<&ShootingShot> {
    snapshot.shots.iter().find(|shot| shot.id == snapshot.active_shot_id).or_else(|| snapshot.shots.first())
}

/// 📦️ The active asset — same fallback rule as `active_shot`.
pub fn active_asset(snapshot: &ShootingSnapshot) -> Option<&ShootingAsset> {
    snapshot.assets.iter().find(|asset| asset.id == snapshot.active_asset_id).or_else(|| snapshot.assets.first())
}

/// 🌫️ A background of `""`/`"transparent"` means "let the surface show through" — shared by the scene
/// window's environment JSON and the icon-render request below (two consumers).
pub fn is_transparent_shooting_background(background: &str) -> bool {
    background.is_empty() || background == "transparent"
}
//#endregion 🔖️DocumentHelpers
}
//#endregion 🧩️ArtifactItems9
