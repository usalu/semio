//! 📸 Remodel scene document — schema-only photogrammetry/videogrammetry project state (media
//! streams, calibration, ground control points, reconstruction params/job/results) shared as CRDT
//! operations. The actual algorithms live in sibling `remodel_image`/`remodel_video`/`remodel_camera`/
//! `remodel_feature`/`remodel_sfm`/`remodel_dense`/`remodel_mesh`/`remodel_motion`/`remodel_geo`/
//! `remodel_engine` crates, none of which this crate depends on: heavier runtime types (`Se3`,
//! `Intrinsics`, `Distortion`, `WatertightReport`, decoded pyramids, match graphs, depth maps, TSDF
//! volumes) are not designed for durable CRDT persistence, so every reference to their shape below is
//! a plain-JSON (or `Packed*`) snapshot the plugin runtime fills in, never the library type itself.

use base64::Engine as _;
use semio_framework_core::MeshData;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use protocol::{Operation, OperationDiff};

pub const REMODEL_DOCUMENT_SCHEMA: &str = "remodel.scene";

//#region 🔖Packed
/// 📦 A flat `f32` buffer serialized as a base64 string of its little-endian bytes rather than a JSON
/// array — point clouds and height grids commonly carry 10^5-10^6 elements, where per-element JSON
/// text is both far larger on the wire and far slower to parse than one base64 blob.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PackedF32(pub String);

impl PackedF32 {
    /// 📦 Encodes a `f32` slice as a base64 string of its little-endian bytes.
    pub fn from_f32_slice(values: &[f32]) -> Self {
        let bytes: Vec<u8> = values.iter().flat_map(|value| value.to_le_bytes()).collect();
        Self(base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    /// 📦 Decodes back into a `f32` vec; a malformed payload (bad base64, length not a multiple of 4)
    /// decodes as empty rather than panicking, since packed buffers only ever round-trip in-process.
    pub fn to_f32_vec(&self) -> Vec<f32> {
        let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(self.0.as_bytes()) else {
            return Vec::new();
        };
        let (chunks, remainder) = bytes.as_chunks::<4>();
        if !remainder.is_empty() {
            return Vec::new();
        }
        chunks.iter().map(|chunk| f32::from_le_bytes(*chunk)).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// 📦 A flat `u8` buffer (vertex colors, classification codes) that serializes as a base64 string
/// directly — same rationale as {@link PackedF32}.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PackedU8(pub String);

impl PackedU8 {
    /// 📦 Encodes a `u8` slice as a base64 string.
    pub fn from_u8_slice(values: &[u8]) -> Self {
        Self(base64::engine::general_purpose::STANDARD.encode(values))
    }

    /// 📦 Decodes back into a `u8` vec; a malformed payload decodes as empty.
    pub fn to_u8_vec(&self) -> Vec<u8> {
        base64::engine::general_purpose::STANDARD.decode(self.0.as_bytes()).unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// 🌉 `PackedF32`'s inner string is ALREADY the wire format (base64 text), so it binds as a plain
/// `Shape::Text` rather than `#[dsl(base64)]` (which is for raw `Vec<u8>` fields only) — no double
/// encoding, no `-` sentinel: an empty buffer is just an empty quoted string.
impl dsl::DslField for PackedF32 {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(Self(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🌉 Same reasoning as `PackedF32`'s impl above.
impl dsl::DslField for PackedU8 {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(Self(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}
//#endregion 🔖Packed

//#region 🔖Domain
/// 🖼️ One embedded pixel asset (video frame, ortho tile, texture) referenced by id from
/// `RemodelScene::assets`, `MediaStream.frames`, `RemodelMesh.texture_asset_id`, or
/// `GeoProducts.{dsm,dtm,ortho}_asset_id`. Sampled video frames use `image/jpeg` (~10x smaller than
/// PNG for photographic content); PNG stays reserved for exports/textures/rasters that need
/// lossless round trips.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ImageAsset {
    pub mime: String,
    pub data: String,
    pub width: u32,
    pub height: u32,
}

/// 🗂️ Which shape a `MediaStream`'s frames were captured as. Video input is always eagerly extracted
/// into individually-addressable `FrameRef`s before persistence (video bytes themselves are never
/// stored) — `MediaKind::Video` only records that provenance, `MediaStream.source` carries the detail.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum MediaKind {
    #[default]
    ImageSequence,
    Video,
}

/// 🎞️ Codec a `VideoSource` was demuxed from — a plain mirror of `remodel_video::VideoCodec` without
/// its `FourCc` payload (an unrecognized four-character code collapses to `Unknown`, which is enough
/// provenance for a QC/diagnostic label).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum VideoCodec {
    Avc,
    Hevc,
    Vp9,
    Av1,
    Mjpeg,
    #[default]
    Unknown,
}

/// 🎥 Provenance of a `MediaStream` that originated from an actual video file (as opposed to a raw
/// image-sequence import) — a lightweight mirror of `remodel_video::{Mp4Info, AviInfo}`, populated
/// once at import time from `remodel_video::probe`. "Video input = image sequence with timestamps":
/// by the time a stream reaches this document its frames are already individually-addressable
/// `ImageAsset`s with true media timestamps; this struct only records where they came from.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct VideoSource {
    pub name: String,
    pub container: String,
    pub codec: VideoCodec,
    pub duration_ms: f64,
    pub frame_count: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FrameRef {
    pub index: u32,
    pub timestamp_ms: f64,
    pub asset_id: String,
}

/// 🎞️ One imported media source (an image sequence or a video), decoded into `FrameRef`s pointing at
/// `RemodelScene::assets`. Multiple cameras/angles are multiple streams, joined by `camera_id`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct MediaStream {
    pub id: String,
    pub name: String,
    pub kind: MediaKind,
    pub camera_id: Option<String>,
    pub sync_offset_ms: f64,
    pub fps_hint: f64,
    #[dsl(table)]
    pub frames: Vec<FrameRef>,
    #[dsl(block)]
    pub source: Option<VideoSource>,
}

/// 🎯 Per-camera intrinsics/distortion, a plain-JSON mirror of `remodel_camera::{Intrinsics,
/// Distortion}` rather than a direct reuse of those types: `Distortion` is a Rust enum tuned for the
/// solver's math (`BrownConrady{k1,k2,k3,p1,p2}` / `FisheyeEquidistant{k1,k2,k3,k4}`), which doesn't
/// serialize into a stable arg-form-editable shape — the document instead always carries a flat
/// 5-slot `distortion` array plus a `model` label the plugin uses to decide which slots are live,
/// matching the "pinhole|brownConrady|fisheye" UI select.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct CameraCalibration {
    pub id: String,
    pub label: String,
    pub model: String,
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub skew: f64,
    /// 🔢 `[k1, k2, k3, p1, p2]`.
    pub distortion: [f32; 5],
    pub rms_reprojection_px: Option<f32>,
    pub locked: bool,
}

/// 🎯 One rig member's pose relative to the rig origin — a plain mirror of `remodel_camera`'s
/// `RigExtrinsic{camera_id, pose_in_rig: Se3}`, flattened to a quaternion + translation since `Se3`
/// (a `mathematical_lie` manifold type) is a plugin-runtime concern, not a document one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RigExtrinsic {
    pub camera_id: String,
    pub rotation_wxyz: [f32; 4],
    pub translation_m: [f32; 3],
}

impl Default for RigExtrinsic {
    fn default() -> Self {
        Self { camera_id: String::new(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation_m: [0.0; 3] }
    }
}

/// 🎯 Per-camera intrinsics/distortion plus rig extrinsics, refined by `remodel_camera`/`remodel_sfm`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct CalibrationState {
    #[dsl(table)]
    pub cameras: Vec<CameraCalibration>,
    #[dsl(table)]
    pub rig: Vec<RigExtrinsic>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GcpObservation {
    pub stream_id: String,
    pub frame_index: u32,
    pub pixel: [f32; 2],
}

/// 📍 A surveyed ground-control point used by `remodel_geo` to georeference the reconstruction.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct GroundControlPoint {
    pub id: String,
    pub name: String,
    pub world_position: [f64; 3],
    #[dsl(table)]
    pub observations: Vec<GcpObservation>,
}

/// ⏭️ Frame sampling/decode limits `remodel_engine` applies before feature extraction. `min_sharpness`
/// is the blur gate: a candidate frame is dropped when its sharpness falls below this fraction of the
/// rolling median sharpness of the last ~15 accepted frames.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct IngestParams {
    pub frame_sample_stride: u32,
    pub max_frames: u32,
    pub downscale_long_edge_px: u32,
    pub min_sharpness: f32,
}

impl Default for IngestParams {
    fn default() -> Self {
        Self { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum FeatureDetector {
    #[default]
    Orb,
    Akaze,
    Harris,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct FeatureParams {
    pub detector: FeatureDetector,
    pub target_count: u32,
    pub octaves: u32,
    pub edge_threshold: f32,
}

impl Default for FeatureParams {
    fn default() -> Self {
        Self { detector: FeatureDetector::default(), target_count: 4000, octaves: 4, edge_threshold: 10.0 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum MatcherKind {
    #[default]
    BruteForce,
    KdTree,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct MatchParams {
    pub matcher: MatcherKind,
    pub ratio_test: f32,
    pub cross_check: bool,
    pub sequential_window: u32,
    pub max_pairs_per_frame: u32,
    pub loop_closure: bool,
}

impl Default for MatchParams {
    fn default() -> Self {
        Self {
            matcher: MatcherKind::default(),
            ratio_test: 0.8,
            cross_check: true,
            sequential_window: 8,
            max_pairs_per_frame: 16,
            loop_closure: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum RobustLossKind {
    L2,
    #[default]
    Huber,
    Cauchy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct SfmParams {
    pub ransac_iterations: u32,
    pub ransac_threshold_px: f32,
    pub min_track_length: u32,
    pub ba_max_iterations: u32,
    pub robust_loss: RobustLossKind,
    pub huber_delta_px: f32,
}

impl Default for SfmParams {
    fn default() -> Self {
        Self {
            ransac_iterations: 1000,
            ransac_threshold_px: 2.0,
            min_track_length: 3,
            ba_max_iterations: 50,
            robust_loss: RobustLossKind::default(),
            huber_delta_px: 1.5,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum DenseResolution {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct DenseParams {
    pub resolution: DenseResolution,
    pub window_radius_px: u32,
    pub min_view_consistency: u32,
    pub confidence_threshold: f32,
    pub max_points: u32,
}

impl Default for DenseParams {
    fn default() -> Self {
        Self {
            resolution: DenseResolution::default(),
            window_radius_px: 3,
            min_view_consistency: 3,
            confidence_threshold: 0.5,
            max_points: 500_000,
        }
    }
}

/// 🧊 UI-facing meshing knobs `remodel_engine` translates into `remodel_mesh`'s own internal
/// `MeshParams`/`TsdfVolume` construction args (this document does not depend on `remodel_mesh`, so
/// the two `MeshParams` types are intentionally separate). `guarantee_watertight`,
/// `hole_fill_max_boundary_verts`, and `self_intersection_check` are the watertight-guarantee knobs:
/// when `guarantee_watertight` is set and repair/hole-fill can't recover a closed 2-manifold, the
/// `🔖Close` fallback triggers and re-validates until the result passes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct MeshParams {
    pub tsdf_voxel_size_mm: f32,
    pub tsdf_truncation_mm: f32,
    pub decimate_target_triangles: u32,
    pub smoothing_iterations: u32,
    pub texture_enabled: bool,
    pub texture_size: u32,
    pub guarantee_watertight: bool,
    pub hole_fill_max_boundary_verts: u32,
    pub self_intersection_check: bool,
}

impl Default for MeshParams {
    fn default() -> Self {
        Self {
            tsdf_voxel_size_mm: 5.0,
            tsdf_truncation_mm: 20.0,
            decimate_target_triangles: 200_000,
            smoothing_iterations: 2,
            texture_enabled: true,
            texture_size: 2048,
            guarantee_watertight: true,
            hole_fill_max_boundary_verts: 512,
            self_intersection_check: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct MotionParams {
    pub enabled: bool,
    pub max_tracks: u32,
    pub track_window_px: u32,
    pub min_track_quality: f32,
    pub min_track_length_frames: u32,
}

impl Default for MotionParams {
    fn default() -> Self {
        Self { enabled: false, max_tracks: 64, track_window_px: 21, min_track_quality: 0.3, min_track_length_frames: 5 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct GeoParams {
    pub enabled: bool,
    pub origin_lon: Option<f64>,
    pub origin_lat: Option<f64>,
    pub origin_alt: Option<f64>,
    pub gsd_m: f32,
    pub dsm_cell_m: f32,
    pub dtm_filter_radius_m: f32,
    pub ortho_max_px: u32,
}

impl Default for GeoParams {
    fn default() -> Self {
        Self {
            enabled: false,
            origin_lon: None,
            origin_lat: None,
            origin_alt: None,
            gsd_m: 0.05,
            dsm_cell_m: 0.1,
            dtm_filter_radius_m: 2.0,
            ortho_max_px: 4096,
        }
    }
}

/// ⚙️ Full reconstruction parameter set, one sub-struct per pipeline stage — `remodel_engine` reads
/// these directly to configure `remodel_image`/`remodel_video`/`remodel_camera`/`remodel_feature`/
/// `remodel_sfm`/`remodel_dense`/`remodel_mesh`/`remodel_motion`/`remodel_geo` without this crate
/// depending on any of them.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionParams {
    #[dsl(block)]
    pub ingest: IngestParams,
    #[dsl(block)]
    pub feature: FeatureParams,
    #[dsl(block)]
    pub matching: MatchParams,
    #[dsl(block)]
    pub sfm: SfmParams,
    #[dsl(block)]
    pub dense: DenseParams,
    #[dsl(block)]
    pub mesh: MeshParams,
    #[dsl(block)]
    pub motion: MotionParams,
    #[dsl(block)]
    pub geo: GeoParams,
}

/// 🚦 Mirrors `remodel_engine`'s pipeline lifecycle so the document can render progress without
/// polling internals directly.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum ReconstructionStage {
    #[default]
    Idle,
    Ingesting,
    Calibrating,
    ExtractingFeatures,
    MatchingFeatures,
    EstimatingPoses,
    BundleAdjusting,
    Georeferencing,
    DenseStereo,
    FusingVolume,
    ExtractingSurface,
    CleaningMesh,
    Texturing,
    TrackingMotion,
    DerivingGeoProducts,
    ReportingQc,
    Done,
    Failed,
}

/// 📷 A single recovered camera pose — streamed early into `ReconstructionJob.camera_poses_preview`
/// for live preview during sparse reconstruction, and reused verbatim as `CameraTrajectory.poses` once
/// the run finishes (no separate heavier pose type: both are the same lightweight snapshot).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CameraPosePreview {
    pub camera_id: String,
    pub rotation_wxyz: [f32; 4],
    pub translation: [f32; 3],
}

impl Default for CameraPosePreview {
    fn default() -> Self {
        Self { camera_id: String::new(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [0.0; 3] }
    }
}

/// 🚧 Live reconstruction run state — deliberately holds no algorithm scratch (descriptors, match
/// graphs, depth maps, TSDF volumes; those stay in the plugin's `PipelineScratch`), only what the UI
/// needs to render progress and what undo/redo needs to restore. `native_port` (a phantom pointer at
/// a `remodel-native` service that was never implemented) has been removed entirely — there is no
/// out-of-process reconstruction backend, only in-process WASM-safe classical algorithms.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionJob {
    pub id: String,
    pub stage: ReconstructionStage,
    pub progress_0_1: f32,
    pub cancel_requested: bool,
    pub stage_cursor: u32,
    pub started_at_ms: Option<f64>,
    pub error: Option<String>,
    #[dsl(table)]
    pub camera_poses_preview: Vec<CameraPosePreview>,
    pub sparse_point_cloud_preview: PackedF32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum MeshSource {
    #[default]
    Placeholder,
    Reconstructed,
    Imported,
}

/// ✅ A plain-JSON mirror of `remodel_mesh::WatertightReport`'s summary fields (all scalars — the
/// report itself carries no array data, so this is a snapshot only in the sense of avoiding a hard
/// dependency on `remodel_mesh`, not in the sense of trimming size).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct WatertightReportSnapshot {
    pub vertex_count: u32,
    pub triangle_count: u32,
    pub boundary_edge_count: u32,
    pub boundary_loop_count: u32,
    pub non_manifold_edge_count: u32,
    pub non_manifold_vertex_count: u32,
    pub connected_components: u32,
    pub consistently_oriented: bool,
    pub euler_characteristic: i64,
    pub genus: Option<i64>,
    pub signed_volume: f64,
    pub self_intersection_pairs: Option<u32>,
    pub closed_fallback_used: bool,
    pub is_closed: bool,
    pub is_two_manifold: bool,
    pub is_watertight: bool,
}

/// 🧵 The reconstructed (or placeholder/imported) mesh, reusing the canonical interchange type
/// (`MeshData` already carries its own `uvs`, so `RemodelMesh` doesn't duplicate them). Always present
/// (never `Option`) so the 3D view always has something to render — `default_remodel_scene()` seeds it
/// with a placeholder box.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelMesh {
    pub mesh: MeshData,
    pub source: MeshSource,
    pub texture_asset_id: Option<String>,
    pub watertight: Option<WatertightReportSnapshot>,
}

//#region 🔖MeshBridge
/// 🔢 Base64-packed `u32` buffer, the `indices`/`faceIds`/`vertexIds`/`edgeIds` counterpart to
/// {@link PackedF32}/{@link PackedU8} — kept private to {@link MeshDataTwin} since nothing else in
/// this document needs a `u32` buffer.
#[derive(Clone, Debug, Default, PartialEq)]
struct PackedU32(String);

impl PackedU32 {
    fn from_u32_slice(values: &[u32]) -> Self {
        let bytes: Vec<u8> = values.iter().flat_map(|value| value.to_le_bytes()).collect();
        Self(base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    fn to_u32_vec(&self) -> Vec<u32> {
        let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(self.0.as_bytes()) else {
            return Vec::new();
        };
        let (chunks, remainder) = bytes.as_chunks::<4>();
        if !remainder.is_empty() {
            return Vec::new();
        }
        chunks.iter().map(|chunk| u32::from_le_bytes(*chunk)).collect()
    }
}

impl dsl::DslField for PackedU32 {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(Self(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🌉 Local structural twin of `semio_framework_core::MeshData`'s numeric buffers, for the DSL
/// boundary only — `MeshData` is foreign (`framework/core`, out of scope for this conversion), so
/// `RemodelMesh.mesh` can't get a derive-generated `dsl::DslField` impl directly (the orphan rule
/// blocks `impl dsl::DslField for MeshData` here: both the trait and the type are foreign to this
/// crate). Every buffer packs base64 exactly like the old hand-rolled parser did; see
/// `RemodelMesh`'s own hand `dsl::DslField` impl below for how this twin is used without ever
/// changing `mesh: MeshData`'s public Rust type (`remodel/plugin` calls `.vertex_count()`/`.aabb()`
/// and builds `RemodelMesh { mesh: mesh_from_kind(..), .. }` directly against it).
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
struct MeshDataTwin {
    positions: PackedF32,
    normals: PackedF32,
    colors: PackedF32,
    indices: PackedU32,
    uvs: PackedF32,
    face_ids: PackedU32,
    vertex_ids: PackedU32,
    edge_positions: PackedF32,
    edge_ids: PackedU32,
    edge_uvs: PackedF32,
    edge_is_seam: PackedU8,
    paint_texture_base64: Option<String>,
}

impl From<&MeshData> for MeshDataTwin {
    fn from(mesh: &MeshData) -> Self {
        Self {
            positions: PackedF32::from_f32_slice(&mesh.positions),
            normals: PackedF32::from_f32_slice(&mesh.normals),
            colors: PackedF32::from_f32_slice(&mesh.colors),
            indices: PackedU32::from_u32_slice(&mesh.indices),
            uvs: PackedF32::from_f32_slice(&mesh.uvs),
            face_ids: PackedU32::from_u32_slice(&mesh.face_ids),
            vertex_ids: PackedU32::from_u32_slice(&mesh.vertex_ids),
            edge_positions: PackedF32::from_f32_slice(&mesh.edge_positions),
            edge_ids: PackedU32::from_u32_slice(&mesh.edge_ids),
            edge_uvs: PackedF32::from_f32_slice(&mesh.edge_uvs),
            edge_is_seam: PackedU8::from_u8_slice(&mesh.edge_is_seam),
            paint_texture_base64: mesh.paint_texture_base64.clone(),
        }
    }
}

impl From<MeshDataTwin> for MeshData {
    fn from(twin: MeshDataTwin) -> Self {
        Self {
            positions: twin.positions.to_f32_vec(),
            normals: twin.normals.to_f32_vec(),
            colors: twin.colors.to_f32_vec(),
            indices: twin.indices.to_u32_vec(),
            uvs: twin.uvs.to_f32_vec(),
            face_ids: twin.face_ids.to_u32_vec(),
            vertex_ids: twin.vertex_ids.to_u32_vec(),
            edge_positions: twin.edge_positions.to_f32_vec(),
            edge_ids: twin.edge_ids.to_u32_vec(),
            edge_uvs: twin.edge_uvs.to_f32_vec(),
            edge_is_seam: twin.edge_is_seam.to_u8_vec(),
            paint_texture_base64: twin.paint_texture_base64,
        }
    }
}

/// 🌉 Hand-written (NOT `#[derive(dsl::DslRecord)]`) `dsl::DslField`/spec/record trio for
/// `RemodelMesh` — mirrors exactly what the derive macro would generate for a plain record, except
/// the `mesh: MeshData` field routes through `MeshDataTwin` above instead of `<MeshData as
/// dsl::DslField>`, which can't exist here (orphan rule). Field ids are purely internal wiring
/// between these three functions, not a wire-compatibility concern.
impl RemodelMesh {
    fn __dsl_spec() -> dsl::RecordSpec {
        dsl::RecordSpec::new_owned(
            None,
            dsl::RecordLayout::Inline,
            vec![
                dsl::FieldSpec::new(0, "source", <MeshSource as dsl::DslField>::shape()),
                dsl::FieldSpec::new(1, "texture-asset-id", <String as dsl::DslField>::shape()).optional(),
                dsl::FieldSpec::new(2, "geometry", dsl::Shape::Block(Box::new(<MeshDataTwin as dsl::DslField>::shape()))),
                dsl::FieldSpec::new(3, "watertight", dsl::Shape::Block(Box::new(<WatertightReportSnapshot as dsl::DslField>::shape()))).optional(),
            ],
        )
    }

    fn __dsl_to_record(&self) -> dsl::RecordValue {
        let mut record = dsl::RecordValue::default();
        record.fields.insert(0, dsl::DslField::to_value(&self.source));
        record.fields.insert(
            1,
            match &self.texture_asset_id {
                Some(v) => dsl::DslField::to_value(v),
                None => dsl::FieldValue::Absent,
            },
        );
        record.fields.insert(2, dsl::FieldValue::Block(Box::new(dsl::DslField::to_value(&MeshDataTwin::from(&self.mesh)))));
        record.fields.insert(
            3,
            match &self.watertight {
                Some(v) => dsl::FieldValue::Block(Box::new(dsl::DslField::to_value(v))),
                None => dsl::FieldValue::Absent,
            },
        );
        record
    }

    fn __dsl_from_record(record: &dsl::RecordValue) -> Result<Self, dsl::TextError> {
        let source = {
            let value = record.get(0).ok_or_else(|| dsl::__rt::field_error("missing field 'source'"))?;
            <MeshSource as dsl::DslField>::from_value(value).map_err(dsl::__rt::field_error)?
        };
        let texture_asset_id = {
            let value = record.get(1).ok_or_else(|| dsl::__rt::field_error("missing field 'texture-asset-id'"))?;
            match value {
                dsl::FieldValue::Absent => None,
                other => Some(<String as dsl::DslField>::from_value(other).map_err(dsl::__rt::field_error)?),
            }
        };
        let mesh = {
            let value = record.get(2).ok_or_else(|| dsl::__rt::field_error("missing field 'geometry'"))?;
            let twin = match value {
                dsl::FieldValue::Block(inner) => <MeshDataTwin as dsl::DslField>::from_value(inner.as_ref()).map_err(dsl::__rt::field_error)?,
                dsl::FieldValue::Absent => <MeshDataTwin as dsl::DslField>::from_value(&dsl::FieldValue::Absent).map_err(dsl::__rt::field_error)?,
                other => return Err(dsl::__rt::field_error(format!("expected Block, found {other:?}"))),
            };
            MeshData::from(twin)
        };
        let watertight = {
            let value = record.get(3).ok_or_else(|| dsl::__rt::field_error("missing field 'watertight'"))?;
            match value {
                dsl::FieldValue::Block(inner) => match inner.as_ref() {
                    dsl::FieldValue::Absent => None,
                    other => Some(<WatertightReportSnapshot as dsl::DslField>::from_value(other).map_err(dsl::__rt::field_error)?),
                },
                dsl::FieldValue::Absent => None,
                other => return Err(dsl::__rt::field_error(format!("expected Block, found {other:?}"))),
            }
        };
        Ok(RemodelMesh { mesh, source, texture_asset_id, watertight })
    }
}

impl dsl::DslField for RemodelMesh {
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(Self::__dsl_spec)
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Record(self.__dsl_to_record())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

/// 🌉 `Box<T>` is a `#[fundamental]` std type, so implementing the foreign `dsl::DslField` trait for
/// `Box<RemodelMesh>` (a local type parameter) here is coherence-legal — needed because
/// `RemodelOperation::SetMeshResult`/`RemodelDiff::SetMeshResult` carry `mesh: Box<RemodelMesh>`
/// (boxed only to shrink the enum's overall size; `RemodelMesh` itself is a plain record, not a
/// `DslEnum`, so the derive's `#[dsl(statements)] Box<T>` "exactly-one-tagged-value" idiom doesn't
/// apply — this is the ordinary boxed-scalar case instead).
impl dsl::DslField for Box<RemodelMesh> {
    fn shape() -> dsl::Shape {
        <RemodelMesh as dsl::DslField>::shape()
    }
    fn to_value(&self) -> dsl::FieldValue {
        <RemodelMesh as dsl::DslField>::to_value(self.as_ref())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        <RemodelMesh as dsl::DslField>::from_value(value).map(Box::new)
    }
}
//#endregion 🔖MeshBridge

/// ☁️ Sparse point cloud from bundle adjustment (`points` = flat xyz triples).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct SparseCloud {
    pub points: PackedF32,
    pub colors: Option<PackedU8>,
}

/// ☁️ Dense point cloud with optional per-point LAS-style classification codes (0 unclassified, 2
/// ground, 6 building, …) — `remodel_dense::PointClass` is a bespoke enum without numeric LAS
/// discriminants, so `remodel_engine` maps it to LAS codes when it distills this snapshot.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct DenseCloud {
    pub positions: PackedF32,
    pub colors: Option<PackedU8>,
    pub confidence: Option<PackedF32>,
    pub classification: Option<PackedU8>,
}

/// 🎥 Recovered camera trajectory across all registered frames.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct CameraTrajectory {
    #[dsl(table)]
    pub poses: Vec<CameraPosePreview>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum TrackClass {
    #[default]
    Static,
    Moving,
}

/// 🏃 A distilled summary of one `remodel_motion` track — full per-frame keyframe paths
/// (`Track2d`/`Trajectory3d` in the motion crate) are plugin-runtime scratch, not durable document
/// state; only enough is kept here to list/label tracks and drive the report table.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct MotionTrackSummary {
    pub id: String,
    pub length: u32,
    pub class: TrackClass,
    pub mean_speed_m_s: f32,
}

/// 🗺️ Georeferenced raster products, each stored as a pixel `ImageAsset` (DSM/DTM as 16-bit-encoded
/// PNG, ortho as an RGB PNG) rather than an embedded float grid — rasters are pixels, so they follow
/// the same persistence rule as every other image in this document instead of a bespoke height-grid
/// packed-array shape.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct GeoProducts {
    pub dsm_asset_id: Option<String>,
    pub dtm_asset_id: Option<String>,
    pub ortho_asset_id: Option<String>,
}

/// ✅ A plain-JSON mirror of the QC-relevant fields of `remodel_geo::QualityReport`, plus the
/// watertight snapshot (mirroring `QualityReport.watertight: Option<WatertightReport>`) and a few
/// cheap scalar summaries (`remodel_engine` computes these once at the end of a run; the underlying
/// per-camera covariance/per-point-sigma arrays and density/overlap rasters stay plugin-runtime).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct QcReportSnapshot {
    pub reprojection_rms_px: f64,
    pub gcp_checkpoint_rmse: Option<f64>,
    #[dsl(block)]
    pub watertight: Option<WatertightReportSnapshot>,
    pub mean_track_length: f32,
    pub registered_frame_ratio: f32,
    pub dense_coverage_ratio: f32,
    pub warnings: Vec<String>,
}

/// 📦 Everything a completed (or partially completed) reconstruction run has produced so far.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionResults {
    #[dsl(block)]
    pub sparse: Option<SparseCloud>,
    #[dsl(block)]
    pub dense: Option<DenseCloud>,
    #[dsl(block)]
    pub mesh: RemodelMesh,
    #[dsl(block)]
    pub trajectory: Option<CameraTrajectory>,
    #[dsl(table)]
    pub tracks: Vec<MotionTrackSummary>,
    #[dsl(block)]
    pub geo: Option<GeoProducts>,
    #[dsl(block)]
    pub qc: Option<QcReportSnapshot>,
}

/// 🗂️ Top-level remodel project document — only persistent, undoable reconstruction state. Ephemeral
/// viewport state (camera/selection/cursors), algorithm scratch (descriptors, match graphs, depth
/// maps, TSDF volumes), and the active utility (host-owned `view_state.active_utility_id`) all live in
/// the plugin runtime, never in this document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[dsl(extension = "remodel")]
#[serde(rename_all = "camelCase")]
pub struct RemodelScene {
    pub schema: String,
    pub id: String,
    #[serde(default)]
    #[dsl(table)]
    pub streams: Vec<MediaStream>,
    #[serde(default)]
    pub assets: BTreeMap<String, ImageAsset>,
    #[serde(default)]
    #[dsl(block)]
    pub calibration: CalibrationState,
    #[serde(default)]
    #[dsl(block)]
    pub params: ReconstructionParams,
    #[serde(default)]
    #[dsl(table)]
    pub gcps: Vec<GroundControlPoint>,
    #[serde(default)]
    #[dsl(block)]
    pub job: ReconstructionJob,
    #[serde(default)]
    #[dsl(block)]
    pub results: ReconstructionResults,
}

/// 🌱 An empty scene seeded with a placeholder box mesh, so the 3D editor/preview always has
/// something to render before any media has been imported/reconstructed.
pub fn default_remodel_scene() -> RemodelScene {
    RemodelScene {
        schema: REMODEL_DOCUMENT_SCHEMA.into(),
        id: "remodel".into(),
        streams: Vec::new(),
        assets: BTreeMap::new(),
        calibration: CalibrationState::default(),
        params: ReconstructionParams::default(),
        gcps: Vec::new(),
        job: ReconstructionJob::default(),
        results: ReconstructionResults {
            mesh: RemodelMesh { mesh: semio_framework_core::mesh_from_kind("box"), source: MeshSource::Placeholder, ..RemodelMesh::default() },
            ..ReconstructionResults::default()
        },
    }
}
//#endregion 🔖Domain

//#region 🔖Operations
/// 🔁 The document mutation vocabulary — one field-granular LWW register setter per independent
/// `RemodelScene` field/sub-field, so disjoint-field edits by concurrent instances converge cleanly.
/// There is no `setDocument` catch-all: reconstruction is field-granular (import a stream, tune one
/// param group, publish a partial result) and each operation carries its own inverse from the pre-edit state.
/// `SetAsset` is per-key (not a whole-map replace) so two peers importing different frames converge
/// without clobbering each other's assets — see `concurrent_set_asset_ops_converge_regardless_of_order`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum RemodelOperation {
    SetStreams {
        streams: Vec<MediaStream>,
    },
    SetAsset {
        key: String,
        #[serde(default)]
        #[dsl(block)]
        value: Option<ImageAsset>,
    },
    SetCalibration {
        #[dsl(block)]
        calibration: CalibrationState,
    },
    SetGcps {
        gcps: Vec<GroundControlPoint>,
    },
    SetIngestParams {
        #[dsl(block)]
        params: IngestParams,
    },
    SetFeatureParams {
        #[dsl(block)]
        params: FeatureParams,
    },
    SetMatchParams {
        #[dsl(block)]
        params: MatchParams,
    },
    SetSfmParams {
        #[dsl(block)]
        params: SfmParams,
    },
    SetDenseParams {
        #[dsl(block)]
        params: DenseParams,
    },
    SetMeshParams {
        #[dsl(block)]
        params: MeshParams,
    },
    SetMotionParams {
        #[dsl(block)]
        params: MotionParams,
    },
    SetGeoParams {
        #[dsl(block)]
        params: GeoParams,
    },
    SetJob {
        #[dsl(block)]
        job: ReconstructionJob,
    },
    SetSparse {
        #[serde(default)]
        #[dsl(block)]
        sparse: Option<SparseCloud>,
    },
    SetDense {
        #[serde(default)]
        #[dsl(block)]
        dense: Option<DenseCloud>,
    },
    /// 📦 Boxed: `RemodelMesh` (a full `MeshData` plus an optional watertight snapshot) is far larger
    /// than any sibling variant, and `clippy::large_enum_variant` flags the resulting size disparity
    /// across `RemodelOperation`/`RemodelDiff` — boxing keeps every other variant cheap to move.
    SetMeshResult {
        #[dsl(block)]
        mesh: Box<RemodelMesh>,
    },
    SetTrajectory {
        #[serde(default)]
        #[dsl(block)]
        trajectory: Option<CameraTrajectory>,
    },
    SetTracks {
        tracks: Vec<MotionTrackSummary>,
    },
    SetGeoProducts {
        #[serde(default)]
        #[dsl(block)]
        geo: Option<GeoProducts>,
    },
    SetQc {
        #[serde(default)]
        #[dsl(block)]
        qc: Option<QcReportSnapshot>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RemodelDiff {
    #[default]
    Empty,
    SetStreams {
        streams: Vec<MediaStream>,
    },
    SetAsset {
        key: String,
        #[serde(default)]
        value: Option<ImageAsset>,
    },
    SetCalibration {
        calibration: CalibrationState,
    },
    SetGcps {
        gcps: Vec<GroundControlPoint>,
    },
    SetIngestParams {
        params: IngestParams,
    },
    SetFeatureParams {
        params: FeatureParams,
    },
    SetMatchParams {
        params: MatchParams,
    },
    SetSfmParams {
        params: SfmParams,
    },
    SetDenseParams {
        params: DenseParams,
    },
    SetMeshParams {
        params: MeshParams,
    },
    SetMotionParams {
        params: MotionParams,
    },
    SetGeoParams {
        params: GeoParams,
    },
    SetJob {
        job: ReconstructionJob,
    },
    SetSparse {
        #[serde(default)]
        sparse: Option<SparseCloud>,
    },
    SetDense {
        #[serde(default)]
        dense: Option<DenseCloud>,
    },
    SetMeshResult {
        mesh: Box<RemodelMesh>,
    },
    SetTrajectory {
        #[serde(default)]
        trajectory: Option<CameraTrajectory>,
    },
    SetTracks {
        tracks: Vec<MotionTrackSummary>,
    },
    SetGeoProducts {
        #[serde(default)]
        geo: Option<GeoProducts>,
    },
    SetQc {
        #[serde(default)]
        qc: Option<QcReportSnapshot>,
    },
}

pub fn apply_remodel_operation(scene: &RemodelScene, operation: &RemodelOperation) -> RemodelScene {
    let mut next = scene.clone();
    match operation {
        RemodelOperation::SetStreams { streams } => next.streams = streams.clone(),
        RemodelOperation::SetAsset { key, value } => match value {
            Some(value) => {
                next.assets.insert(key.clone(), value.clone());
            }
            None => {
                next.assets.remove(key);
            }
        },
        RemodelOperation::SetCalibration { calibration } => next.calibration = calibration.clone(),
        RemodelOperation::SetGcps { gcps } => next.gcps = gcps.clone(),
        RemodelOperation::SetIngestParams { params } => next.params.ingest = params.clone(),
        RemodelOperation::SetFeatureParams { params } => next.params.feature = params.clone(),
        RemodelOperation::SetMatchParams { params } => next.params.matching = params.clone(),
        RemodelOperation::SetSfmParams { params } => next.params.sfm = params.clone(),
        RemodelOperation::SetDenseParams { params } => next.params.dense = params.clone(),
        RemodelOperation::SetMeshParams { params } => next.params.mesh = params.clone(),
        RemodelOperation::SetMotionParams { params } => next.params.motion = params.clone(),
        RemodelOperation::SetGeoParams { params } => next.params.geo = params.clone(),
        RemodelOperation::SetJob { job } => next.job = job.clone(),
        RemodelOperation::SetSparse { sparse } => next.results.sparse = sparse.clone(),
        RemodelOperation::SetDense { dense } => next.results.dense = dense.clone(),
        RemodelOperation::SetMeshResult { mesh } => next.results.mesh = mesh.as_ref().clone(),
        RemodelOperation::SetTrajectory { trajectory } => next.results.trajectory = trajectory.clone(),
        RemodelOperation::SetTracks { tracks } => next.results.tracks = tracks.clone(),
        RemodelOperation::SetGeoProducts { geo } => next.results.geo = geo.clone(),
        RemodelOperation::SetQc { qc } => next.results.qc = qc.clone(),
    }
    next
}

impl OperationDiff<RemodelScene> for RemodelDiff {
    fn apply(&self, projection: &RemodelScene) -> RemodelScene {
        let operation = match self {
            RemodelDiff::Empty => return projection.clone(),
            RemodelDiff::SetStreams { streams } => RemodelOperation::SetStreams { streams: streams.clone() },
            RemodelDiff::SetAsset { key, value } => RemodelOperation::SetAsset { key: key.clone(), value: value.clone() },
            RemodelDiff::SetCalibration { calibration } => RemodelOperation::SetCalibration { calibration: calibration.clone() },
            RemodelDiff::SetGcps { gcps } => RemodelOperation::SetGcps { gcps: gcps.clone() },
            RemodelDiff::SetIngestParams { params } => RemodelOperation::SetIngestParams { params: params.clone() },
            RemodelDiff::SetFeatureParams { params } => RemodelOperation::SetFeatureParams { params: params.clone() },
            RemodelDiff::SetMatchParams { params } => RemodelOperation::SetMatchParams { params: params.clone() },
            RemodelDiff::SetSfmParams { params } => RemodelOperation::SetSfmParams { params: params.clone() },
            RemodelDiff::SetDenseParams { params } => RemodelOperation::SetDenseParams { params: params.clone() },
            RemodelDiff::SetMeshParams { params } => RemodelOperation::SetMeshParams { params: params.clone() },
            RemodelDiff::SetMotionParams { params } => RemodelOperation::SetMotionParams { params: params.clone() },
            RemodelDiff::SetGeoParams { params } => RemodelOperation::SetGeoParams { params: params.clone() },
            RemodelDiff::SetJob { job } => RemodelOperation::SetJob { job: job.clone() },
            RemodelDiff::SetSparse { sparse } => RemodelOperation::SetSparse { sparse: sparse.clone() },
            RemodelDiff::SetDense { dense } => RemodelOperation::SetDense { dense: dense.clone() },
            RemodelDiff::SetMeshResult { mesh } => RemodelOperation::SetMeshResult { mesh: mesh.clone() },
            RemodelDiff::SetTrajectory { trajectory } => RemodelOperation::SetTrajectory { trajectory: trajectory.clone() },
            RemodelDiff::SetTracks { tracks } => RemodelOperation::SetTracks { tracks: tracks.clone() },
            RemodelDiff::SetGeoProducts { geo } => RemodelOperation::SetGeoProducts { geo: geo.clone() },
            RemodelDiff::SetQc { qc } => RemodelOperation::SetQc { qc: qc.clone() },
        };
        apply_remodel_operation(projection, &operation)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, RemodelDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<RemodelScene> for RemodelOperation {
    type Diff = RemodelDiff;

    fn diff(&self, _projection: &RemodelScene) -> RemodelDiff {
        match self {
            RemodelOperation::SetStreams { streams } => RemodelDiff::SetStreams { streams: streams.clone() },
            RemodelOperation::SetAsset { key, value } => RemodelDiff::SetAsset { key: key.clone(), value: value.clone() },
            RemodelOperation::SetCalibration { calibration } => RemodelDiff::SetCalibration { calibration: calibration.clone() },
            RemodelOperation::SetGcps { gcps } => RemodelDiff::SetGcps { gcps: gcps.clone() },
            RemodelOperation::SetIngestParams { params } => RemodelDiff::SetIngestParams { params: params.clone() },
            RemodelOperation::SetFeatureParams { params } => RemodelDiff::SetFeatureParams { params: params.clone() },
            RemodelOperation::SetMatchParams { params } => RemodelDiff::SetMatchParams { params: params.clone() },
            RemodelOperation::SetSfmParams { params } => RemodelDiff::SetSfmParams { params: params.clone() },
            RemodelOperation::SetDenseParams { params } => RemodelDiff::SetDenseParams { params: params.clone() },
            RemodelOperation::SetMeshParams { params } => RemodelDiff::SetMeshParams { params: params.clone() },
            RemodelOperation::SetMotionParams { params } => RemodelDiff::SetMotionParams { params: params.clone() },
            RemodelOperation::SetGeoParams { params } => RemodelDiff::SetGeoParams { params: params.clone() },
            RemodelOperation::SetJob { job } => RemodelDiff::SetJob { job: job.clone() },
            RemodelOperation::SetSparse { sparse } => RemodelDiff::SetSparse { sparse: sparse.clone() },
            RemodelOperation::SetDense { dense } => RemodelDiff::SetDense { dense: dense.clone() },
            RemodelOperation::SetMeshResult { mesh } => RemodelDiff::SetMeshResult { mesh: mesh.clone() },
            RemodelOperation::SetTrajectory { trajectory } => RemodelDiff::SetTrajectory { trajectory: trajectory.clone() },
            RemodelOperation::SetTracks { tracks } => RemodelDiff::SetTracks { tracks: tracks.clone() },
            RemodelOperation::SetGeoProducts { geo } => RemodelDiff::SetGeoProducts { geo: geo.clone() },
            RemodelOperation::SetQc { qc } => RemodelDiff::SetQc { qc: qc.clone() },
        }
    }

    fn backwards(&self, projection: &RemodelScene) -> Vec<Self> {
        vec![match self {
            RemodelOperation::SetStreams { .. } => RemodelOperation::SetStreams { streams: projection.streams.clone() },
            RemodelOperation::SetAsset { key, .. } => RemodelOperation::SetAsset { key: key.clone(), value: projection.assets.get(key).cloned() },
            RemodelOperation::SetCalibration { .. } => RemodelOperation::SetCalibration { calibration: projection.calibration.clone() },
            RemodelOperation::SetGcps { .. } => RemodelOperation::SetGcps { gcps: projection.gcps.clone() },
            RemodelOperation::SetIngestParams { .. } => RemodelOperation::SetIngestParams { params: projection.params.ingest.clone() },
            RemodelOperation::SetFeatureParams { .. } => RemodelOperation::SetFeatureParams { params: projection.params.feature.clone() },
            RemodelOperation::SetMatchParams { .. } => RemodelOperation::SetMatchParams { params: projection.params.matching.clone() },
            RemodelOperation::SetSfmParams { .. } => RemodelOperation::SetSfmParams { params: projection.params.sfm.clone() },
            RemodelOperation::SetDenseParams { .. } => RemodelOperation::SetDenseParams { params: projection.params.dense.clone() },
            RemodelOperation::SetMeshParams { .. } => RemodelOperation::SetMeshParams { params: projection.params.mesh.clone() },
            RemodelOperation::SetMotionParams { .. } => RemodelOperation::SetMotionParams { params: projection.params.motion.clone() },
            RemodelOperation::SetGeoParams { .. } => RemodelOperation::SetGeoParams { params: projection.params.geo.clone() },
            RemodelOperation::SetJob { .. } => RemodelOperation::SetJob { job: projection.job.clone() },
            RemodelOperation::SetSparse { .. } => RemodelOperation::SetSparse { sparse: projection.results.sparse.clone() },
            RemodelOperation::SetDense { .. } => RemodelOperation::SetDense { dense: projection.results.dense.clone() },
            RemodelOperation::SetMeshResult { .. } => RemodelOperation::SetMeshResult { mesh: Box::new(projection.results.mesh.clone()) },
            RemodelOperation::SetTrajectory { .. } => RemodelOperation::SetTrajectory { trajectory: projection.results.trajectory.clone() },
            RemodelOperation::SetTracks { .. } => RemodelOperation::SetTracks { tracks: projection.results.tracks.clone() },
            RemodelOperation::SetGeoProducts { .. } => RemodelOperation::SetGeoProducts { geo: projection.results.geo.clone() },
            RemodelOperation::SetQc { .. } => RemodelOperation::SetQc { qc: projection.results.qc.clone() },
        }]
    }
}
//#endregion 🔖Operations

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region Domain
    #[test]
    fn default_scene_has_placeholder_mesh() {
        let scene = default_remodel_scene();
        assert_eq!(scene.results.mesh.source, MeshSource::Placeholder);
        assert!(!scene.results.mesh.mesh.positions.is_empty());
        assert!(!scene.results.mesh.mesh.indices.is_empty());
        assert_eq!(scene.results.mesh.watertight, None);
        assert!(scene.streams.is_empty());
        assert!(scene.assets.is_empty());
        assert!(scene.gcps.is_empty());
        assert_eq!(scene.job, ReconstructionJob::default());
        assert_eq!(scene.results.sparse, None);
        assert_eq!(scene.results.dense, None);
        assert_eq!(scene.results.trajectory, None);
        assert!(scene.results.tracks.is_empty());
        assert_eq!(scene.results.geo, None);
        assert_eq!(scene.results.qc, None);
    }

    #[test]
    fn scene_roundtrips_through_json() {
        let scene = default_remodel_scene();
        let json = serde_json::to_string(&scene).expect("serialize");
        let parsed: RemodelScene = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, scene);
    }

    #[test]
    fn populated_scene_roundtrips_through_json() {
        let scene = populated_scene_fixture();
        let json = serde_json::to_string(&scene).expect("serialize");
        let parsed: RemodelScene = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, scene);
    }

    #[test]
    fn packed_f32_roundtrips_exactly() {
        let values = vec![1.5_f32, -2.25, 3.0, f32::MIN_POSITIVE, -0.0];
        let packed = PackedF32::from_f32_slice(&values);
        let value = serde_json::to_value(&packed).expect("serialize");
        assert!(value.is_string(), "PackedF32 must serialize as a base64 string, got {value:?}");
        let parsed: PackedF32 = serde_json::from_value(value).expect("deserialize");
        assert_eq!(parsed, packed);
        assert_eq!(parsed.to_f32_vec(), values);

        let empty = PackedF32::default();
        assert!(empty.is_empty());
        assert_eq!(empty.to_f32_vec(), Vec::<f32>::new());
    }

    #[test]
    fn packed_u8_roundtrips_exactly() {
        let values = vec![0_u8, 128, 255, 64];
        let packed = PackedU8::from_u8_slice(&values);
        let value = serde_json::to_value(&packed).expect("serialize");
        assert!(value.is_string(), "PackedU8 must serialize as a base64 string, got {value:?}");
        let parsed: PackedU8 = serde_json::from_value(value).expect("deserialize");
        assert_eq!(parsed, packed);
        assert_eq!(parsed.to_u8_vec(), values);

        let empty = PackedU8::default();
        assert!(empty.is_empty());
        assert_eq!(empty.to_u8_vec(), Vec::<u8>::new());
    }

    #[test]
    fn reconstruction_stage_serde_is_stable() {
        let cases: [(ReconstructionStage, &str); 18] = [
            (ReconstructionStage::Idle, "\"idle\""),
            (ReconstructionStage::Ingesting, "\"ingesting\""),
            (ReconstructionStage::Calibrating, "\"calibrating\""),
            (ReconstructionStage::ExtractingFeatures, "\"extracting-features\""),
            (ReconstructionStage::MatchingFeatures, "\"matching-features\""),
            (ReconstructionStage::EstimatingPoses, "\"estimating-poses\""),
            (ReconstructionStage::BundleAdjusting, "\"bundle-adjusting\""),
            (ReconstructionStage::Georeferencing, "\"georeferencing\""),
            (ReconstructionStage::DenseStereo, "\"dense-stereo\""),
            (ReconstructionStage::FusingVolume, "\"fusing-volume\""),
            (ReconstructionStage::ExtractingSurface, "\"extracting-surface\""),
            (ReconstructionStage::CleaningMesh, "\"cleaning-mesh\""),
            (ReconstructionStage::Texturing, "\"texturing\""),
            (ReconstructionStage::TrackingMotion, "\"tracking-motion\""),
            (ReconstructionStage::DerivingGeoProducts, "\"deriving-geo-products\""),
            (ReconstructionStage::ReportingQc, "\"reporting-qc\""),
            (ReconstructionStage::Done, "\"done\""),
            (ReconstructionStage::Failed, "\"failed\""),
        ];
        for (stage, expected) in cases {
            assert_eq!(serde_json::to_string(&stage).expect("serialize"), expected);
        }
    }
    //#endregion Domain

    //#region Operations
    #[test]
    fn set_streams_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let streams = vec![MediaStream { id: "s1".into(), name: "cam".into(), ..MediaStream::default() }];
        let operation = RemodelOperation::SetStreams { streams: streams.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.streams, streams);
        assert_eq!(operation.diff(&scene).apply(&scene).streams, streams);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetStreams { streams: scene.streams.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.streams, scene.streams);
    }

    #[test]
    fn set_asset_op_applies_and_reverts_including_absent_case() {
        let scene = default_remodel_scene();
        assert!(!scene.assets.contains_key("frame-1"));

        let asset = ImageAsset { mime: "image/jpeg".into(), data: "zzz".into(), width: 2, height: 2 };
        let insert_operation = RemodelOperation::SetAsset { key: "frame-1".into(), value: Some(asset.clone()) };
        let after_insert = apply_remodel_operation(&scene, &insert_operation);
        assert_eq!(after_insert.assets.get("frame-1"), Some(&asset));
        assert_eq!(insert_operation.diff(&scene).apply(&scene).assets.get("frame-1"), Some(&asset));

        let insert_inverse = insert_operation.backwards(&scene);
        assert_eq!(insert_inverse, vec![RemodelOperation::SetAsset { key: "frame-1".into(), value: None }]);
        let reverted = insert_inverse.iter().fold(after_insert.clone(), |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted, scene);

        let remove_operation = RemodelOperation::SetAsset { key: "frame-1".into(), value: None };
        let remove_inverse = remove_operation.backwards(&after_insert);
        assert_eq!(remove_inverse, vec![RemodelOperation::SetAsset { key: "frame-1".into(), value: Some(asset.clone()) }]);
        let after_remove = apply_remodel_operation(&after_insert, &remove_operation);
        assert!(!after_remove.assets.contains_key("frame-1"));
        let restored = remove_inverse.iter().fold(after_remove, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(restored.assets.get("frame-1"), Some(&asset));
    }

    #[test]
    fn set_calibration_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mut calibration = scene.calibration.clone();
        calibration.cameras.push(CameraCalibration { id: "cam-1".into(), model: "pinhole".into(), ..CameraCalibration::default() });
        let operation = RemodelOperation::SetCalibration { calibration: calibration.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.calibration, calibration);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetCalibration { calibration: scene.calibration.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.calibration, scene.calibration);
    }

    #[test]
    fn set_gcps_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let gcps = vec![GroundControlPoint { id: "gcp-1".into(), name: "A".into(), world_position: [0.0, 0.0, 0.0], observations: Vec::new() }];
        let operation = RemodelOperation::SetGcps { gcps: gcps.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.gcps, gcps);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetGcps { gcps: scene.gcps.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.gcps, scene.gcps);
    }

    #[test]
    fn set_job_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mut job = scene.job.clone();
        job.stage = ReconstructionStage::BundleAdjusting;
        job.progress_0_1 = 0.42;
        job.started_at_ms = Some(1000.0);
        let operation = RemodelOperation::SetJob { job: job.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.job, job);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetJob { job: scene.job.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.job, scene.job);
    }

    #[test]
    fn set_sparse_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let sparse = SparseCloud { points: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0]), colors: Some(PackedU8::from_u8_slice(&[255, 255, 255])) };
        let operation = RemodelOperation::SetSparse { sparse: Some(sparse.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.sparse, Some(sparse));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetSparse { sparse: scene.results.sparse.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.sparse, scene.results.sparse);
    }

    #[test]
    fn set_dense_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let dense = DenseCloud {
            positions: PackedF32::from_f32_slice(&[1.0, 2.0, 3.0]),
            colors: None,
            confidence: Some(PackedF32::from_f32_slice(&[0.8])),
            classification: Some(PackedU8::from_u8_slice(&[2])),
        };
        let operation = RemodelOperation::SetDense { dense: Some(dense.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.dense, Some(dense));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetDense { dense: scene.results.dense.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.dense, scene.results.dense);
    }

    #[test]
    fn set_mesh_result_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mesh = RemodelMesh {
            mesh: semio_framework_core::mesh_from_kind("box"),
            source: MeshSource::Reconstructed,
            texture_asset_id: Some("tex-1".into()),
            watertight: Some(WatertightReportSnapshot { is_watertight: true, is_two_manifold: true, is_closed: true, ..WatertightReportSnapshot::default() }),
        };
        let operation = RemodelOperation::SetMeshResult { mesh: Box::new(mesh.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.mesh, mesh);
        assert_eq!(operation.diff(&scene).apply(&scene).results.mesh, mesh);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetMeshResult { mesh: Box::new(scene.results.mesh.clone()) }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.mesh, scene.results.mesh);
    }

    #[test]
    fn set_trajectory_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let trajectory = CameraTrajectory { poses: vec![CameraPosePreview { camera_id: "cam-1".into(), ..CameraPosePreview::default() }] };
        let operation = RemodelOperation::SetTrajectory { trajectory: Some(trajectory.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.trajectory, Some(trajectory));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetTrajectory { trajectory: scene.results.trajectory.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.trajectory, scene.results.trajectory);
    }

    #[test]
    fn set_tracks_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let tracks = vec![MotionTrackSummary { id: "t1".into(), length: 12, class: TrackClass::Moving, mean_speed_m_s: 0.5 }];
        let operation = RemodelOperation::SetTracks { tracks: tracks.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.tracks, tracks);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetTracks { tracks: scene.results.tracks.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.tracks, scene.results.tracks);
    }

    #[test]
    fn set_geo_products_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let geo = GeoProducts { dsm_asset_id: Some("dsm".into()), dtm_asset_id: None, ortho_asset_id: Some("ortho".into()) };
        let operation = RemodelOperation::SetGeoProducts { geo: Some(geo.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.geo, Some(geo));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetGeoProducts { geo: scene.results.geo.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.geo, scene.results.geo);
    }

    #[test]
    fn set_qc_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let qc = QcReportSnapshot { reprojection_rms_px: 0.6, warnings: vec!["w".into()], ..QcReportSnapshot::default() };
        let operation = RemodelOperation::SetQc { qc: Some(qc.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.qc, Some(qc));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetQc { qc: scene.results.qc.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.qc, scene.results.qc);
    }

    /// 🔁 The 8 `Set<Stage>Params` operations are mechanically identical (LWW-replace one
    /// `ReconstructionParams` sub-field, inverse restores the pre-edit value) — generated once per
    /// param family instead of copy-pasted, per the "concise code" rule.
    macro_rules! param_op_roundtrip_test {
        ($test_name:ident, $variant:ident, $field:ident, $params_ty:ty, $mutate:expr) => {
            #[test]
            fn $test_name() {
                let scene = default_remodel_scene();
                let mut params: $params_ty = scene.params.$field.clone();
                let mutate: fn(&mut $params_ty) = $mutate;
                mutate(&mut params);
                let operation = RemodelOperation::$variant { params: params.clone() };
                let next = apply_remodel_operation(&scene, &operation);
                assert_eq!(next.params.$field, params);
                assert_eq!(operation.diff(&scene).apply(&scene).params.$field, params);
                let inverse = operation.backwards(&scene);
                assert_eq!(inverse, vec![RemodelOperation::$variant { params: scene.params.$field.clone() }]);
                let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
                assert_eq!(reverted.params.$field, scene.params.$field);
            }
        };
    }

    param_op_roundtrip_test!(set_ingest_params_op_applies_and_reverts, SetIngestParams, ingest, IngestParams, |p| p.min_sharpness = 0.6);
    param_op_roundtrip_test!(set_feature_params_op_applies_and_reverts, SetFeatureParams, feature, FeatureParams, |p| p.target_count = 8000);
    param_op_roundtrip_test!(set_match_params_op_applies_and_reverts, SetMatchParams, matching, MatchParams, |p| p.ratio_test = 0.6);
    param_op_roundtrip_test!(set_sfm_params_op_applies_and_reverts, SetSfmParams, sfm, SfmParams, |p| p.ransac_iterations = 2000);
    param_op_roundtrip_test!(set_dense_params_op_applies_and_reverts, SetDenseParams, dense, DenseParams, |p| p.max_points = 100);
    param_op_roundtrip_test!(set_mesh_params_op_applies_and_reverts, SetMeshParams, mesh, MeshParams, |p| p.guarantee_watertight = false);
    param_op_roundtrip_test!(set_motion_params_op_applies_and_reverts, SetMotionParams, motion, MotionParams, |p| p.enabled = true);
    param_op_roundtrip_test!(set_geo_params_op_applies_and_reverts, SetGeoParams, geo, GeoParams, |p| p.enabled = true);
    //#endregion Operations

    //#region Convergence
    /// 🔀 The CRDT convergence contract: two collaborators concurrently importing different frames
    /// (`SetAsset` on disjoint keys) must converge to an identical scene regardless of application
    /// order, and neither import may clobber the other's key. This is what makes `SetAsset` per-key
    /// rather than a whole-`assets`-map replace — a whole-map design would fail this test, since
    /// applying instance B's operation after instance A's would silently drop A's key if B's captured map
    /// snapshot predates A's insert.
    #[test]
    fn concurrent_set_asset_ops_converge_regardless_of_order() {
        let base = default_remodel_scene();
        let asset_a = ImageAsset { mime: "image/jpeg".into(), data: "frame-one".into(), width: 8, height: 8 };
        let asset_b = ImageAsset { mime: "image/jpeg".into(), data: "frame-two".into(), width: 8, height: 8 };
        let op_a = RemodelOperation::SetAsset { key: "frame-1".into(), value: Some(asset_a.clone()) };
        let op_b = RemodelOperation::SetAsset { key: "frame-2".into(), value: Some(asset_b.clone()) };

        let a_then_b = apply_remodel_operation(&apply_remodel_operation(&base, &op_a), &op_b);
        let b_then_a = apply_remodel_operation(&apply_remodel_operation(&base, &op_b), &op_a);

        assert_eq!(a_then_b, b_then_a, "concurrent SetAsset on disjoint keys must converge regardless of order");
        assert_eq!(a_then_b.assets.get("frame-1"), Some(&asset_a), "instance A's import must survive instance B's operation");
        assert_eq!(a_then_b.assets.get("frame-2"), Some(&asset_b), "instance B's import must survive instance A's operation");
        assert_eq!(a_then_b.assets.len(), base.assets.len() + 2);
    }

    /// 🔀 Same convergence contract across two *disjoint operation families* at once (one instance tunes
    /// feature-detector params, the other adds a GCP) — proves field-granular LWW converges not just
    /// within one operation family but across the whole operation vocabulary.
    #[test]
    fn concurrent_edits_across_different_op_families_converge() {
        let base = default_remodel_scene();
        let mut feature_params = base.params.feature.clone();
        feature_params.target_count = 9000;
        let op_feature = RemodelOperation::SetFeatureParams { params: feature_params.clone() };
        let gcps = vec![GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [1.0, 2.0, 3.0], observations: Vec::new() }];
        let op_gcp = RemodelOperation::SetGcps { gcps: gcps.clone() };

        let feature_then_gcp = apply_remodel_operation(&apply_remodel_operation(&base, &op_feature), &op_gcp);
        let gcp_then_feature = apply_remodel_operation(&apply_remodel_operation(&base, &op_gcp), &op_feature);

        assert_eq!(feature_then_gcp, gcp_then_feature);
        assert_eq!(feature_then_gcp.params.feature, feature_params);
        assert_eq!(feature_then_gcp.gcps, gcps);
    }
    //#endregion Convergence

    //#region 🔖DslAndOpText
    /// 🏗️ Shared fixture for both the JSON and the `.remodel` DSL round-trip tests: a scene that
    /// exercises every optional/collection field at least once, so `assert_dsl_round_trip` (and the
    /// pre-existing `populated_scene_roundtrips_through_json`) actually walk the full document shape
    /// instead of just `default_remodel_scene()`'s mostly-empty surface.
    fn populated_scene_fixture() -> RemodelScene {
        let mut scene = default_remodel_scene();
        scene.streams.push(MediaStream {
            id: "stream-1".into(),
            name: "front".into(),
            kind: MediaKind::Video,
            camera_id: Some("cam-1".into()),
            sync_offset_ms: 12.5,
            fps_hint: 30.0,
            frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: "asset-1".into() }],
            source: Some(VideoSource {
                name: "front.mp4".into(),
                container: "mp4".into(),
                codec: VideoCodec::Avc,
                duration_ms: 6633.3,
                frame_count: 199,
                width: 1920,
                height: 1080,
            }),
        });
        scene.assets.insert("asset-1".into(), ImageAsset { mime: "image/jpeg".into(), data: "abcd".into(), width: 4, height: 4 });
        scene.calibration.cameras.push(CameraCalibration {
            id: "cam-1".into(),
            label: "Front".into(),
            model: "brownConrady".into(),
            fx: 1000.0,
            fy: 1000.0,
            cx: 512.0,
            cy: 384.0,
            skew: 0.0,
            distortion: [0.01, -0.02, 0.0, 0.0, 0.0],
            rms_reprojection_px: Some(0.4),
            locked: false,
        });
        scene.calibration.rig.push(RigExtrinsic::default());
        scene.gcps.push(GroundControlPoint {
            id: "gcp-1".into(),
            name: "Corner".into(),
            world_position: [1.0, 2.0, 3.0],
            observations: vec![GcpObservation { stream_id: "stream-1".into(), frame_index: 0, pixel: [10.0, 20.0] }],
        });
        scene.params.ingest.min_sharpness = 0.4;
        scene.params.mesh.texture_size = 4096;
        scene.job.stage = ReconstructionStage::BundleAdjusting;
        scene.job.progress_0_1 = 0.42;
        scene.job.started_at_ms = Some(1000.0);
        scene.job.error = Some("retry needed".into());
        scene.job.camera_poses_preview.push(CameraPosePreview { camera_id: "cam-1".into(), ..CameraPosePreview::default() });
        scene.job.sparse_point_cloud_preview = PackedF32::from_f32_slice(&[0.1, 0.2, 0.3]);
        scene.results.sparse = Some(SparseCloud {
            points: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
            colors: Some(PackedU8::from_u8_slice(&[255, 0, 0, 0, 255, 0])),
        });
        scene.results.dense = Some(DenseCloud {
            positions: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0]),
            colors: Some(PackedU8::from_u8_slice(&[0, 0, 255])),
            confidence: Some(PackedF32::from_f32_slice(&[0.9])),
            classification: Some(PackedU8::from_u8_slice(&[2])),
        });
        scene.results.mesh = RemodelMesh {
            mesh: semio_framework_core::mesh_from_kind("box"),
            source: MeshSource::Reconstructed,
            texture_asset_id: Some("tex-1".into()),
            watertight: Some(WatertightReportSnapshot {
                vertex_count: 512,
                triangle_count: 1020,
                boundary_edge_count: 0,
                boundary_loop_count: 0,
                non_manifold_edge_count: 0,
                non_manifold_vertex_count: 0,
                connected_components: 1,
                consistently_oriented: true,
                euler_characteristic: 2,
                genus: Some(0),
                signed_volume: 12.5,
                self_intersection_pairs: Some(0),
                closed_fallback_used: false,
                is_closed: true,
                is_two_manifold: true,
                is_watertight: true,
            }),
        };
        scene.results.trajectory = Some(CameraTrajectory {
            poses: vec![
                CameraPosePreview { camera_id: "cam-1".into(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [0.0, 0.0, 0.0] },
                CameraPosePreview { camera_id: "cam-1".into(), rotation_wxyz: [0.999, 0.001, 0.0, 0.0], translation: [0.1, 0.0, 0.0] },
            ],
        });
        scene.results.tracks.push(MotionTrackSummary { id: "track-1".into(), length: 42, class: TrackClass::Moving, mean_speed_m_s: 1.2 });
        scene.results.geo = Some(GeoProducts {
            dsm_asset_id: Some("asset-dsm".into()),
            dtm_asset_id: Some("asset-dtm".into()),
            ortho_asset_id: Some("asset-ortho".into()),
        });
        scene.results.qc = Some(QcReportSnapshot {
            reprojection_rms_px: 0.5,
            gcp_checkpoint_rmse: Some(0.02),
            watertight: scene.results.mesh.watertight.clone(),
            mean_track_length: 6.0,
            registered_frame_ratio: 1.0,
            dense_coverage_ratio: 0.95,
            warnings: vec!["low overlap on frame 12".into()],
        });
        scene
    }

    #[test]
    fn default_scene_roundtrips_through_dsl() {
        vcs::test_support::assert_dsl_round_trip(&default_remodel_scene());
        vcs::test_support::assert_dsl_pack_equivalence(&default_remodel_scene());
    }

    #[test]
    fn populated_scene_roundtrips_through_dsl() {
        vcs::test_support::assert_dsl_round_trip(&populated_scene_fixture());
        vcs::test_support::assert_dsl_pack_equivalence(&populated_scene_fixture());
    }

    /// ⚡ One `assert_op_line_round_trip` per `RemodelOperation` variant, per the mechanism contract.
    #[test]
    fn every_operation_variant_roundtrips_through_op_text() {
        let scene = populated_scene_fixture();

        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetStreams { streams: scene.streams.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetAsset { key: "asset-1".into(), value: scene.assets.get("asset-1").cloned() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetAsset { key: "asset-2".into(), value: None });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetCalibration { calibration: scene.calibration.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetGcps { gcps: scene.gcps.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetIngestParams { params: scene.params.ingest.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetFeatureParams { params: scene.params.feature.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetMatchParams { params: scene.params.matching.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetSfmParams { params: scene.params.sfm.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetDenseParams { params: scene.params.dense.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetMeshParams { params: scene.params.mesh.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetMotionParams { params: scene.params.motion.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetGeoParams { params: scene.params.geo.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetJob { job: scene.job.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetSparse { sparse: scene.results.sparse.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetSparse { sparse: None });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetDense { dense: scene.results.dense.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetDense { dense: None });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetMeshResult { mesh: Box::new(scene.results.mesh.clone()) });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetTrajectory { trajectory: scene.results.trajectory.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetTrajectory { trajectory: None });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetTracks { tracks: scene.results.tracks.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetGeoProducts { geo: scene.results.geo.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetGeoProducts { geo: None });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetQc { qc: scene.results.qc.clone() });
        vcs::test_support::assert_op_line_round_trip(&RemodelOperation::SetQc { qc: None });
    }

    /// 📄 Full `print_document_text`/`parse_document_text` round trip through a live `DocumentVcsStore`
    /// with an applied edit, the ground-truth contract for replacing the JSON envelope with text files.
    #[test]
    fn store_roundtrips_through_document_text() {
        let initial = default_remodel_scene();
        let envelope = vcs::create_document_vcs_envelope("test/v1", "test", initial, None);
        let mut store = vcs::DocumentVcsStore::new(envelope);
        let mut feature_params = store.projection().expect("initial projection").params.feature.clone();
        feature_params.target_count = 12345;
        store
            .dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![RemodelOperation::SetFeatureParams { params: feature_params }], description: None })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
        vcs::test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖DslAndOpText
}
//#endregion 🧪Tests
