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
use vcs::{DocumentDsl, Operation, OperationDiff, OpText, TextError, TextSpan};

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
//#endregion 🔖Packed

//#region 🔖Domain
/// 🖼️ One embedded pixel asset (video frame, ortho tile, texture) referenced by id from
/// `RemodelScene::assets`, `MediaStream.frames`, `RemodelMesh.texture_asset_id`, or
/// `GeoProducts.{dsm,dtm,ortho}_asset_id`. Sampled video frames use `image/jpeg` (~10x smaller than
/// PNG for photographic content); PNG stays reserved for exports/textures/rasters that need
/// lossless round trips.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MediaKind {
    #[default]
    ImageSequence,
    Video,
}

/// 🎞️ Codec a `VideoSource` was demuxed from — a plain mirror of `remodel_video::VideoCodec` without
/// its `FourCc` payload (an unrecognized four-character code collapses to `Unknown`, which is enough
/// provenance for a QC/diagnostic label).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameRef {
    pub index: u32,
    pub timestamp_ms: f64,
    pub asset_id: String,
}

/// 🎞️ One imported media source (an image sequence or a video), decoded into `FrameRef`s pointing at
/// `RemodelScene::assets`. Multiple cameras/angles are multiple streams, joined by `camera_id`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MediaStream {
    pub id: String,
    pub name: String,
    pub kind: MediaKind,
    pub camera_id: Option<String>,
    pub sync_offset_ms: f64,
    pub fps_hint: f64,
    pub frames: Vec<FrameRef>,
    pub source: Option<VideoSource>,
}

/// 🎯 Per-camera intrinsics/distortion, a plain-JSON mirror of `remodel_camera::{Intrinsics,
/// Distortion}` rather than a direct reuse of those types: `Distortion` is a Rust enum tuned for the
/// solver's math (`BrownConrady{k1,k2,k3,p1,p2}` / `FisheyeEquidistant{k1,k2,k3,k4}`), which doesn't
/// serialize into a stable arg-form-editable shape — the document instead always carries a flat
/// 5-slot `distortion` array plus a `model` label the plugin uses to decide which slots are live,
/// matching the "pinhole|brownConrady|fisheye" UI select.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CalibrationState {
    pub cameras: Vec<CameraCalibration>,
    pub rig: Vec<RigExtrinsic>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcpObservation {
    pub stream_id: String,
    pub frame_index: u32,
    pub pixel: [f32; 2],
}

/// 📍 A surveyed ground-control point used by `remodel_geo` to georeference the reconstruction.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GroundControlPoint {
    pub id: String,
    pub name: String,
    pub world_position: [f64; 3],
    pub observations: Vec<GcpObservation>,
}

/// ⏭️ Frame sampling/decode limits `remodel_engine` applies before feature extraction. `min_sharpness`
/// is the blur gate: a candidate frame is dropped when its sharpness falls below this fraction of the
/// rolling median sharpness of the last ~15 accepted frames.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FeatureDetector {
    #[default]
    Orb,
    Akaze,
    Harris,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MatcherKind {
    #[default]
    BruteForce,
    KdTree,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RobustLossKind {
    L2,
    #[default]
    Huber,
    Cauchy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DenseResolution {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionParams {
    pub ingest: IngestParams,
    pub feature: FeatureParams,
    pub matching: MatchParams,
    pub sfm: SfmParams,
    pub dense: DenseParams,
    pub mesh: MeshParams,
    pub motion: MotionParams,
    pub geo: GeoParams,
}

/// 🚦 Mirrors `remodel_engine`'s pipeline lifecycle so the document can render progress without
/// polling internals directly.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionJob {
    pub id: String,
    pub stage: ReconstructionStage,
    pub progress_0_1: f32,
    pub cancel_requested: bool,
    pub stage_cursor: u32,
    pub started_at_ms: Option<f64>,
    pub error: Option<String>,
    pub camera_poses_preview: Vec<CameraPosePreview>,
    pub sparse_point_cloud_preview: PackedF32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

/// ☁️ Sparse point cloud from bundle adjustment (`points` = flat xyz triples).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SparseCloud {
    pub points: PackedF32,
    pub colors: Option<PackedU8>,
}

/// ☁️ Dense point cloud with optional per-point LAS-style classification codes (0 unclassified, 2
/// ground, 6 building, …) — `remodel_dense::PointClass` is a bespoke enum without numeric LAS
/// discriminants, so `remodel_engine` maps it to LAS codes when it distills this snapshot.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DenseCloud {
    pub positions: PackedF32,
    pub colors: Option<PackedU8>,
    pub confidence: Option<PackedF32>,
    pub classification: Option<PackedU8>,
}

/// 🎥 Recovered camera trajectory across all registered frames.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CameraTrajectory {
    pub poses: Vec<CameraPosePreview>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrackClass {
    #[default]
    Static,
    Moving,
}

/// 🏃 A distilled summary of one `remodel_motion` track — full per-frame keyframe paths
/// (`Track2d`/`Trajectory3d` in the motion crate) are plugin-runtime scratch, not durable document
/// state; only enough is kept here to list/label tracks and drive the report table.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QcReportSnapshot {
    pub reprojection_rms_px: f64,
    pub gcp_checkpoint_rmse: Option<f64>,
    pub watertight: Option<WatertightReportSnapshot>,
    pub mean_track_length: f32,
    pub registered_frame_ratio: f32,
    pub dense_coverage_ratio: f32,
    pub warnings: Vec<String>,
}

/// 📦 Everything a completed (or partially completed) reconstruction run has produced so far.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionResults {
    pub sparse: Option<SparseCloud>,
    pub dense: Option<DenseCloud>,
    pub mesh: RemodelMesh,
    pub trajectory: Option<CameraTrajectory>,
    pub tracks: Vec<MotionTrackSummary>,
    pub geo: Option<GeoProducts>,
    pub qc: Option<QcReportSnapshot>,
}

/// 🗂️ Top-level remodel project document — only persistent, undoable reconstruction state. Ephemeral
/// viewport state (camera/selection/cursors), algorithm scratch (descriptors, match graphs, depth
/// maps, TSDF volumes), and the active utility (host-owned `view_state.active_utility_id`) all live in
/// the plugin runtime, never in this document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemodelScene {
    pub schema: String,
    pub id: String,
    #[serde(default)]
    pub streams: Vec<MediaStream>,
    #[serde(default)]
    pub assets: BTreeMap<String, ImageAsset>,
    #[serde(default)]
    pub calibration: CalibrationState,
    #[serde(default)]
    pub params: ReconstructionParams,
    #[serde(default)]
    pub gcps: Vec<GroundControlPoint>,
    #[serde(default)]
    pub job: ReconstructionJob,
    #[serde(default)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum RemodelOperation {
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
    /// 📦 Boxed: `RemodelMesh` (a full `MeshData` plus an optional watertight snapshot) is far larger
    /// than any sibling variant, and `clippy::large_enum_variant` flags the resulting size disparity
    /// across `RemodelOperation`/`RemodelDiff` — boxing keeps every other variant cheap to move.
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

//#region 🔖Dsl
/// 📜 Hand-rolled lexer, parser and printer for the `.remodel` DSL (`RemodelScene`) and for
/// `RemodelOperation`'s single-line op encoding — replaces the JSON envelope for both the initial
/// projection and the op log. Whitespace (including newlines) is never significant to the parser:
/// `print_dsl` inserts a newline between top-level sections purely for readability, `print_op`
/// renders the identical grammar on one line. See {@link vcs::DocumentDsl} and {@link vcs::OpText}.
mod remodel_text {
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
        span: TextSpan,
    }

    /// 🔤 Scans `input` into tokens. A bareword `Word` runs until whitespace/`{`/`}`/`"`, so `=` and
    /// `,` are ordinary word characters — `key=value` collapses into one token (split later by
    /// {@link Parser::parse_kv_map}), and only a quoted value forces a token boundary right after `key=`.
    fn lex(input: &str) -> Result<Vec<Lexed>, TextError> {
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
                    out.push(Lexed { tok: Tok::LBrace, span: TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '}' => {
                    out.push(Lexed { tok: Tok::RBrace, span: TextSpan::at(line, col) });
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
                        return Err(TextError::new("unterminated string literal", TextSpan::at(start_line, start_col)));
                    }
                    out.push(Lexed { tok: Tok::Str(s), span: TextSpan::at(start_line, start_col) });
                }
                _ => {
                    let (start_line, start_col, start) = (line, col, i);
                    while i < chars.len() && !matches!(chars[i], ' ' | '\t' | '\r' | '\n' | '{' | '}' | '"') {
                        i += 1;
                        col += 1;
                    }
                    let word: String = chars[start..i].iter().collect();
                    out.push(Lexed { tok: Tok::Word(word), span: TextSpan::at(start_line, start_col) });
                }
            }
        }
        out.push(Lexed { tok: Tok::Eof, span: TextSpan::at(line, col) });
        Ok(out)
    }
    //#endregion Lexer

    //#region Parser
    #[derive(Clone, Debug)]
    enum FieldValue {
        Str(String),
        Word(String),
    }

    type FieldMap = HashMap<String, (FieldValue, TextSpan)>;

    struct Parser {
        toks: Vec<Lexed>,
        pos: usize,
    }

    impl Parser {
        fn peek(&self) -> &Tok {
            &self.toks[self.pos].tok
        }

        fn span(&self) -> TextSpan {
            self.toks[self.pos].span
        }

        fn bump(&mut self) -> Tok {
            let tok = self.toks[self.pos].tok.clone();
            if self.pos + 1 < self.toks.len() {
                self.pos += 1;
            }
            tok
        }

        fn at_lbrace(&self) -> bool {
            matches!(self.peek(), Tok::LBrace)
        }

        fn at_rbrace(&self) -> bool {
            matches!(self.peek(), Tok::RBrace)
        }

        fn at_eof(&self) -> bool {
            matches!(self.peek(), Tok::Eof)
        }

        /// 🔎 True when the next token is the bare `-` sentinel word (never a `key=value` pair) used
        /// throughout this grammar to mean "this optional construct is absent".
        fn at_dash(&self) -> bool {
            matches!(self.peek(), Tok::Word(w) if w == "-")
        }

        fn expect_word(&mut self) -> Result<String, TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Word(w) => Ok(w),
                other => Err(TextError::expected(format!("expected a word, found {other:?}"), span, "word")),
            }
        }

        fn expect_lbrace(&mut self) -> Result<(), TextError> {
            let span = self.span();
            match self.bump() {
                Tok::LBrace => Ok(()),
                other => Err(TextError::expected(format!("expected '{{', found {other:?}"), span, "{")),
            }
        }

        fn expect_rbrace(&mut self) -> Result<(), TextError> {
            let span = self.span();
            match self.bump() {
                Tok::RBrace => Ok(()),
                other => Err(TextError::expected(format!("expected '}}', found {other:?}"), span, "}")),
            }
        }

        /// 🗺️ Greedily reads `key=value` tokens (order-independent) until a token that isn't one — the
        /// generic header-field reader every construct is built on.
        fn parse_kv_map(&mut self) -> Result<FieldMap, TextError> {
            let mut map = HashMap::new();
            loop {
                let word = match self.peek() {
                    Tok::Word(w) if w.contains('=') => w.clone(),
                    _ => break,
                };
                let span = self.span();
                self.bump();
                let (key, rest) = word.split_once('=').expect("word already checked to contain '='");
                let value = if rest.is_empty() { FieldValue::Str(self.expect_str_at(span)?) } else { FieldValue::Word(rest.to_string()) };
                map.insert(key.to_string(), (value, span));
            }
            Ok(map)
        }

        fn expect_str_at(&mut self, span: TextSpan) -> Result<String, TextError> {
            match self.bump() {
                Tok::Str(s) => Ok(s),
                other => Err(TextError::expected(format!("expected a quoted string, found {other:?}"), span, "string")),
            }
        }

        /// 📚 Greedily reads quoted strings — the list grammar for `warnings`.
        fn greedy_str_list(&mut self) -> Vec<String> {
            let mut out = Vec::new();
            while let Tok::Str(_) = self.peek() {
                if let Tok::Str(s) = self.bump() {
                    out.push(s);
                }
            }
            out
        }
    }

    fn kv_str(map: &FieldMap, key: &str, span: TextSpan) -> Result<String, TextError> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Ok(s.clone()),
            Some((FieldValue::Word(_), field_span)) => Err(TextError::expected(format!("field '{key}' must be a quoted string"), *field_span, "string")),
            None => Err(TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_opt_str(map: &FieldMap, key: &str) -> Option<String> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Some(s.clone()),
            _ => None,
        }
    }

    fn kv_word(map: &FieldMap, key: &str, span: TextSpan) -> Result<String, TextError> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) => Ok(w.clone()),
            Some((FieldValue::Str(_), field_span)) => Err(TextError::expected(format!("field '{key}' must not be quoted"), *field_span, "word")),
            None => Err(TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_opt_word(map: &FieldMap, key: &str) -> Option<String> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) if w != "-" => Some(w.clone()),
            _ => None,
        }
    }

    fn kv_bool(map: &FieldMap, key: &str, span: TextSpan) -> Result<bool, TextError> {
        match kv_word(map, key, span)?.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(TextError::expected(format!("field '{key}' must be 'true' or 'false'"), span, "true|false")),
        }
    }

    fn kv_num<T: std::str::FromStr>(map: &FieldMap, key: &str, span: TextSpan, label: &str) -> Result<T, TextError> {
        let word = kv_word(map, key, span)?;
        word.parse::<T>().map_err(|_| TextError::expected(format!("field '{key}' must be a {label}"), span, label.to_string()))
    }

    fn kv_opt_num<T: std::str::FromStr>(map: &FieldMap, key: &str) -> Option<T> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) if w != "-" => w.parse::<T>().ok(),
            _ => None,
        }
    }
    //#endregion Parser

    //#region Scalars
    /// 🔐 Minimal escape for free-text fields (names/labels/error messages) — mirrors `vcs`'s own
    /// `escape_text_field`/`unescape_text_field` (unescaping happens inline in the lexer's `"` branch).
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

    fn csv<T: std::fmt::Display>(values: &[T]) -> String {
        values.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(",")
    }

    fn parse_csv<T: std::str::FromStr, const N: usize>(word: &str, span: TextSpan) -> Result<[T; N], TextError> {
        let parts: Vec<&str> = word.split(',').collect();
        if parts.len() != N {
            return Err(TextError::expected(format!("expected {N} comma-separated numbers, got {}", parts.len()), span, format!("{N} numbers")));
        }
        let mut values: Vec<T> = Vec::with_capacity(N);
        for part in &parts {
            values.push(part.parse::<T>().map_err(|_| TextError::expected(format!("invalid number '{part}'"), span, "number"))?);
        }
        values.try_into().map_err(|_| TextError::new("internal csv arity mismatch", span))
    }

    /// 📦 Base64-packs a `u32` slice as little-endian bytes — the `indices`/`faceIds`/`vertexIds`/
    /// `edgeIds` counterpart to {@link PackedF32}/{@link PackedU8}, kept local since `u32` arithmetic
    /// (unlike `f32` bit patterns) has no existing wrapper type in `🔖Packed`.
    fn pack_u32(values: &[u32]) -> String {
        let bytes: Vec<u8> = values.iter().flat_map(|value| value.to_le_bytes()).collect();
        base64::engine::general_purpose::STANDARD.encode(bytes)
    }

    fn unpack_u32(text: &str) -> Vec<u32> {
        let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(text.as_bytes()) else {
            return Vec::new();
        };
        let (chunks, remainder) = bytes.as_chunks::<4>();
        if !remainder.is_empty() {
            return Vec::new();
        }
        chunks.iter().map(|chunk| u32::from_le_bytes(*chunk)).collect()
    }

    fn print_packed_f32(values: &PackedF32) -> String {
        if values.is_empty() { "-".to_string() } else { values.0.clone() }
    }

    fn parse_packed_f32(word: &str) -> PackedF32 {
        if word == "-" { PackedF32::default() } else { PackedF32(word.to_string()) }
    }

    fn print_packed_u8(values: &PackedU8) -> String {
        if values.is_empty() { "-".to_string() } else { values.0.clone() }
    }

    fn parse_packed_u8(word: &str) -> PackedU8 {
        if word == "-" { PackedU8::default() } else { PackedU8(word.to_string()) }
    }

    fn print_packed_u32(values: &[u32]) -> String {
        if values.is_empty() { "-".to_string() } else { pack_u32(values) }
    }

    fn parse_packed_u32(word: &str) -> Vec<u32> {
        if word == "-" { Vec::new() } else { unpack_u32(word) }
    }

    /// 🔤 Every closed enum in this document round-trips through its serde kebab-case string — hand
    /// mirrored here (rather than routed through `serde_json`) to keep the DSL a self-contained grammar.
    fn media_kind_str(value: MediaKind) -> &'static str {
        match value {
            MediaKind::ImageSequence => "image-sequence",
            MediaKind::Video => "video",
        }
    }
    fn parse_media_kind(word: &str, span: TextSpan) -> Result<MediaKind, TextError> {
        match word {
            "image-sequence" => Ok(MediaKind::ImageSequence),
            "video" => Ok(MediaKind::Video),
            other => Err(TextError::expected(format!("unknown media kind '{other}'"), span, "image-sequence|video")),
        }
    }

    fn video_codec_str(value: VideoCodec) -> &'static str {
        match value {
            VideoCodec::Avc => "avc",
            VideoCodec::Hevc => "hevc",
            VideoCodec::Vp9 => "vp9",
            VideoCodec::Av1 => "av1",
            VideoCodec::Mjpeg => "mjpeg",
            VideoCodec::Unknown => "unknown",
        }
    }
    fn parse_video_codec(word: &str, span: TextSpan) -> Result<VideoCodec, TextError> {
        match word {
            "avc" => Ok(VideoCodec::Avc),
            "hevc" => Ok(VideoCodec::Hevc),
            "vp9" => Ok(VideoCodec::Vp9),
            "av1" => Ok(VideoCodec::Av1),
            "mjpeg" => Ok(VideoCodec::Mjpeg),
            "unknown" => Ok(VideoCodec::Unknown),
            other => Err(TextError::expected(format!("unknown video codec '{other}'"), span, "avc|hevc|vp9|av1|mjpeg|unknown")),
        }
    }

    fn feature_detector_str(value: FeatureDetector) -> &'static str {
        match value {
            FeatureDetector::Orb => "orb",
            FeatureDetector::Akaze => "akaze",
            FeatureDetector::Harris => "harris",
        }
    }
    fn parse_feature_detector(word: &str, span: TextSpan) -> Result<FeatureDetector, TextError> {
        match word {
            "orb" => Ok(FeatureDetector::Orb),
            "akaze" => Ok(FeatureDetector::Akaze),
            "harris" => Ok(FeatureDetector::Harris),
            other => Err(TextError::expected(format!("unknown feature detector '{other}'"), span, "orb|akaze|harris")),
        }
    }

    fn matcher_kind_str(value: MatcherKind) -> &'static str {
        match value {
            MatcherKind::BruteForce => "brute-force",
            MatcherKind::KdTree => "kd-tree",
        }
    }
    fn parse_matcher_kind(word: &str, span: TextSpan) -> Result<MatcherKind, TextError> {
        match word {
            "brute-force" => Ok(MatcherKind::BruteForce),
            "kd-tree" => Ok(MatcherKind::KdTree),
            other => Err(TextError::expected(format!("unknown matcher kind '{other}'"), span, "brute-force|kd-tree")),
        }
    }

    fn robust_loss_str(value: RobustLossKind) -> &'static str {
        match value {
            RobustLossKind::L2 => "l2",
            RobustLossKind::Huber => "huber",
            RobustLossKind::Cauchy => "cauchy",
        }
    }
    fn parse_robust_loss(word: &str, span: TextSpan) -> Result<RobustLossKind, TextError> {
        match word {
            "l2" => Ok(RobustLossKind::L2),
            "huber" => Ok(RobustLossKind::Huber),
            "cauchy" => Ok(RobustLossKind::Cauchy),
            other => Err(TextError::expected(format!("unknown robust loss '{other}'"), span, "l2|huber|cauchy")),
        }
    }

    fn dense_resolution_str(value: DenseResolution) -> &'static str {
        match value {
            DenseResolution::Low => "low",
            DenseResolution::Medium => "medium",
            DenseResolution::High => "high",
        }
    }
    fn parse_dense_resolution(word: &str, span: TextSpan) -> Result<DenseResolution, TextError> {
        match word {
            "low" => Ok(DenseResolution::Low),
            "medium" => Ok(DenseResolution::Medium),
            "high" => Ok(DenseResolution::High),
            other => Err(TextError::expected(format!("unknown dense resolution '{other}'"), span, "low|medium|high")),
        }
    }

    fn reconstruction_stage_str(value: ReconstructionStage) -> &'static str {
        match value {
            ReconstructionStage::Idle => "idle",
            ReconstructionStage::Ingesting => "ingesting",
            ReconstructionStage::Calibrating => "calibrating",
            ReconstructionStage::ExtractingFeatures => "extracting-features",
            ReconstructionStage::MatchingFeatures => "matching-features",
            ReconstructionStage::EstimatingPoses => "estimating-poses",
            ReconstructionStage::BundleAdjusting => "bundle-adjusting",
            ReconstructionStage::Georeferencing => "georeferencing",
            ReconstructionStage::DenseStereo => "dense-stereo",
            ReconstructionStage::FusingVolume => "fusing-volume",
            ReconstructionStage::ExtractingSurface => "extracting-surface",
            ReconstructionStage::CleaningMesh => "cleaning-mesh",
            ReconstructionStage::Texturing => "texturing",
            ReconstructionStage::TrackingMotion => "tracking-motion",
            ReconstructionStage::DerivingGeoProducts => "deriving-geo-products",
            ReconstructionStage::ReportingQc => "reporting-qc",
            ReconstructionStage::Done => "done",
            ReconstructionStage::Failed => "failed",
        }
    }
    fn parse_reconstruction_stage(word: &str, span: TextSpan) -> Result<ReconstructionStage, TextError> {
        match word {
            "idle" => Ok(ReconstructionStage::Idle),
            "ingesting" => Ok(ReconstructionStage::Ingesting),
            "calibrating" => Ok(ReconstructionStage::Calibrating),
            "extracting-features" => Ok(ReconstructionStage::ExtractingFeatures),
            "matching-features" => Ok(ReconstructionStage::MatchingFeatures),
            "estimating-poses" => Ok(ReconstructionStage::EstimatingPoses),
            "bundle-adjusting" => Ok(ReconstructionStage::BundleAdjusting),
            "georeferencing" => Ok(ReconstructionStage::Georeferencing),
            "dense-stereo" => Ok(ReconstructionStage::DenseStereo),
            "fusing-volume" => Ok(ReconstructionStage::FusingVolume),
            "extracting-surface" => Ok(ReconstructionStage::ExtractingSurface),
            "cleaning-mesh" => Ok(ReconstructionStage::CleaningMesh),
            "texturing" => Ok(ReconstructionStage::Texturing),
            "tracking-motion" => Ok(ReconstructionStage::TrackingMotion),
            "deriving-geo-products" => Ok(ReconstructionStage::DerivingGeoProducts),
            "reporting-qc" => Ok(ReconstructionStage::ReportingQc),
            "done" => Ok(ReconstructionStage::Done),
            "failed" => Ok(ReconstructionStage::Failed),
            other => Err(TextError::expected(format!("unknown reconstruction stage '{other}'"), span, "reconstruction stage")),
        }
    }

    fn mesh_source_str(value: MeshSource) -> &'static str {
        match value {
            MeshSource::Placeholder => "placeholder",
            MeshSource::Reconstructed => "reconstructed",
            MeshSource::Imported => "imported",
        }
    }
    fn parse_mesh_source(word: &str, span: TextSpan) -> Result<MeshSource, TextError> {
        match word {
            "placeholder" => Ok(MeshSource::Placeholder),
            "reconstructed" => Ok(MeshSource::Reconstructed),
            "imported" => Ok(MeshSource::Imported),
            other => Err(TextError::expected(format!("unknown mesh source '{other}'"), span, "placeholder|reconstructed|imported")),
        }
    }

    fn track_class_str(value: TrackClass) -> &'static str {
        match value {
            TrackClass::Static => "static",
            TrackClass::Moving => "moving",
        }
    }
    fn parse_track_class(word: &str, span: TextSpan) -> Result<TrackClass, TextError> {
        match word {
            "static" => Ok(TrackClass::Static),
            "moving" => Ok(TrackClass::Moving),
            other => Err(TextError::expected(format!("unknown track class '{other}'"), span, "static|moving")),
        }
    }
    //#endregion Scalars

    //#region Constructs
    fn print_frame(frame: &FrameRef) -> String {
        format!("frame index={} t={} asset={}", frame.index, frame.timestamp_ms, frame.asset_id)
    }
    fn parse_frame(p: &mut Parser) -> Result<FrameRef, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(FrameRef { index: kv_num(&map, "index", span, "integer")?, timestamp_ms: kv_num(&map, "t", span, "number")?, asset_id: kv_word(&map, "asset", span)? })
    }

    fn print_video_source(source: &VideoSource) -> String {
        format!(
            "source {{ name={} container={} codec={} durationMs={} frameCount={} width={} height={} }}",
            quote(&source.name),
            source.container,
            video_codec_str(source.codec),
            source.duration_ms,
            source.frame_count,
            source.width,
            source.height
        )
    }
    fn parse_video_source(p: &mut Parser) -> Result<VideoSource, TextError> {
        p.expect_lbrace()?;
        let span = p.span();
        let map = p.parse_kv_map()?;
        p.expect_rbrace()?;
        Ok(VideoSource {
            name: kv_str(&map, "name", span)?,
            container: kv_word(&map, "container", span)?,
            codec: parse_video_codec(&kv_word(&map, "codec", span)?, span)?,
            duration_ms: kv_num(&map, "durationMs", span, "number")?,
            frame_count: kv_num(&map, "frameCount", span, "integer")?,
            width: kv_num(&map, "width", span, "integer")?,
            height: kv_num(&map, "height", span, "integer")?,
        })
    }

    fn print_stream(stream: &MediaStream) -> String {
        let mut out = format!(
            "stream id={} name={} kind={} camera={} syncOffsetMs={} fpsHint={} {{",
            stream.id,
            quote(&stream.name),
            media_kind_str(stream.kind),
            stream.camera_id.as_deref().unwrap_or("-"),
            stream.sync_offset_ms,
            stream.fps_hint
        );
        if let Some(source) = &stream.source {
            out.push(' ');
            out.push_str(&print_video_source(source));
        }
        for frame in &stream.frames {
            out.push(' ');
            out.push_str(&print_frame(frame));
        }
        out.push_str(" }");
        out
    }
    fn parse_stream(p: &mut Parser) -> Result<MediaStream, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        let id = kv_word(&map, "id", span)?;
        let name = kv_str(&map, "name", span)?;
        let kind = parse_media_kind(&kv_word(&map, "kind", span)?, span)?;
        let camera_id = kv_opt_word(&map, "camera");
        let sync_offset_ms = kv_num(&map, "syncOffsetMs", span, "number")?;
        let fps_hint = kv_num(&map, "fpsHint", span, "number")?;
        p.expect_lbrace()?;
        let mut source = None;
        let mut frames = Vec::new();
        while !p.at_rbrace() {
            match p.expect_word()?.as_str() {
                "source" => source = Some(parse_video_source(p)?),
                "frame" => frames.push(parse_frame(p)?),
                other => return Err(TextError::new(format!("unknown stream child '{other}'"), span)),
            }
        }
        p.expect_rbrace()?;
        Ok(MediaStream { id, name, kind, camera_id, sync_offset_ms, fps_hint, frames, source })
    }

    fn print_asset_fields(asset: &ImageAsset) -> String {
        format!("mime={} width={} height={} data={}", asset.mime, asset.width, asset.height, asset.data)
    }
    fn print_asset(id: &str, asset: &ImageAsset) -> String {
        format!("asset id={id} {}", print_asset_fields(asset))
    }
    fn parse_asset(p: &mut Parser) -> Result<(String, ImageAsset), TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        let id = kv_word(&map, "id", span)?;
        let asset = ImageAsset { mime: kv_word(&map, "mime", span)?, width: kv_num(&map, "width", span, "integer")?, height: kv_num(&map, "height", span, "integer")?, data: kv_word(&map, "data", span)? };
        Ok((id, asset))
    }

    fn print_camera(camera: &CameraCalibration) -> String {
        format!(
            "camera id={} label={} model={} fx={} fy={} cx={} cy={} skew={} distortion={} rms={} locked={}",
            camera.id,
            quote(&camera.label),
            camera.model,
            camera.fx,
            camera.fy,
            camera.cx,
            camera.cy,
            camera.skew,
            csv(&camera.distortion),
            camera.rms_reprojection_px.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            camera.locked
        )
    }
    fn parse_camera(p: &mut Parser) -> Result<CameraCalibration, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(CameraCalibration {
            id: kv_word(&map, "id", span)?,
            label: kv_str(&map, "label", span)?,
            model: kv_word(&map, "model", span)?,
            fx: kv_num(&map, "fx", span, "number")?,
            fy: kv_num(&map, "fy", span, "number")?,
            cx: kv_num(&map, "cx", span, "number")?,
            cy: kv_num(&map, "cy", span, "number")?,
            skew: kv_num(&map, "skew", span, "number")?,
            distortion: parse_csv::<f32, 5>(&kv_word(&map, "distortion", span)?, span)?,
            rms_reprojection_px: kv_opt_num(&map, "rms"),
            locked: kv_bool(&map, "locked", span)?,
        })
    }

    fn print_rig(rig: &RigExtrinsic) -> String {
        format!("rig camera={} rot={} t={}", rig.camera_id, csv(&rig.rotation_wxyz), csv(&rig.translation_m))
    }
    fn parse_rig(p: &mut Parser) -> Result<RigExtrinsic, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(RigExtrinsic {
            camera_id: kv_word(&map, "camera", span)?,
            rotation_wxyz: parse_csv::<f32, 4>(&kv_word(&map, "rot", span)?, span)?,
            translation_m: parse_csv::<f32, 3>(&kv_word(&map, "t", span)?, span)?,
        })
    }

    fn print_calibration_fields(calibration: &CalibrationState) -> String {
        let mut out = "{".to_string();
        for camera in &calibration.cameras {
            out.push(' ');
            out.push_str(&print_camera(camera));
        }
        for rig in &calibration.rig {
            out.push(' ');
            out.push_str(&print_rig(rig));
        }
        out.push_str(" }");
        out
    }
    fn print_calibration(calibration: &CalibrationState) -> String {
        format!("calibration {}", print_calibration_fields(calibration))
    }
    fn parse_calibration(p: &mut Parser) -> Result<CalibrationState, TextError> {
        let span = p.span();
        p.expect_lbrace()?;
        let mut cameras = Vec::new();
        let mut rig = Vec::new();
        while !p.at_rbrace() {
            match p.expect_word()?.as_str() {
                "camera" => cameras.push(parse_camera(p)?),
                "rig" => rig.push(parse_rig(p)?),
                other => return Err(TextError::new(format!("unknown calibration child '{other}'"), span)),
            }
        }
        p.expect_rbrace()?;
        Ok(CalibrationState { cameras, rig })
    }

    fn print_obs(obs: &GcpObservation) -> String {
        format!("obs stream={} frame={} pixel={}", obs.stream_id, obs.frame_index, csv(&obs.pixel))
    }
    fn parse_obs(p: &mut Parser) -> Result<GcpObservation, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(GcpObservation { stream_id: kv_word(&map, "stream", span)?, frame_index: kv_num(&map, "frame", span, "integer")?, pixel: parse_csv::<f32, 2>(&kv_word(&map, "pixel", span)?, span)? })
    }

    fn print_gcp(gcp: &GroundControlPoint) -> String {
        let mut out = format!("gcp id={} name={} pos={} {{", gcp.id, quote(&gcp.name), csv(&gcp.world_position));
        for obs in &gcp.observations {
            out.push(' ');
            out.push_str(&print_obs(obs));
        }
        out.push_str(" }");
        out
    }
    fn parse_gcp(p: &mut Parser) -> Result<GroundControlPoint, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        let id = kv_word(&map, "id", span)?;
        let name = kv_str(&map, "name", span)?;
        let world_position = parse_csv::<f64, 3>(&kv_word(&map, "pos", span)?, span)?;
        p.expect_lbrace()?;
        let mut observations = Vec::new();
        while !p.at_rbrace() {
            p.expect_word()?;
            observations.push(parse_obs(p)?);
        }
        p.expect_rbrace()?;
        Ok(GroundControlPoint { id, name, world_position, observations })
    }

    fn print_ingest_fields(params: &IngestParams) -> String {
        format!("stride={} maxFrames={} downscale={} minSharpness={}", params.frame_sample_stride, params.max_frames, params.downscale_long_edge_px, params.min_sharpness)
    }
    fn print_ingest(params: &IngestParams) -> String {
        format!("ingest {}", print_ingest_fields(params))
    }
    fn parse_ingest(p: &mut Parser) -> Result<IngestParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(IngestParams {
            frame_sample_stride: kv_num(&map, "stride", span, "integer")?,
            max_frames: kv_num(&map, "maxFrames", span, "integer")?,
            downscale_long_edge_px: kv_num(&map, "downscale", span, "integer")?,
            min_sharpness: kv_num(&map, "minSharpness", span, "number")?,
        })
    }

    fn print_feature_fields(params: &FeatureParams) -> String {
        format!("detector={} targetCount={} octaves={} edgeThreshold={}", feature_detector_str(params.detector), params.target_count, params.octaves, params.edge_threshold)
    }
    fn print_feature(params: &FeatureParams) -> String {
        format!("feature {}", print_feature_fields(params))
    }
    fn parse_feature(p: &mut Parser) -> Result<FeatureParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(FeatureParams {
            detector: parse_feature_detector(&kv_word(&map, "detector", span)?, span)?,
            target_count: kv_num(&map, "targetCount", span, "integer")?,
            octaves: kv_num(&map, "octaves", span, "integer")?,
            edge_threshold: kv_num(&map, "edgeThreshold", span, "number")?,
        })
    }

    fn print_match_fields(params: &MatchParams) -> String {
        format!(
            "matcher={} ratio={} crossCheck={} seqWindow={} maxPairs={} loopClosure={}",
            matcher_kind_str(params.matcher),
            params.ratio_test,
            params.cross_check,
            params.sequential_window,
            params.max_pairs_per_frame,
            params.loop_closure
        )
    }
    fn print_match(params: &MatchParams) -> String {
        format!("match {}", print_match_fields(params))
    }
    fn parse_match(p: &mut Parser) -> Result<MatchParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(MatchParams {
            matcher: parse_matcher_kind(&kv_word(&map, "matcher", span)?, span)?,
            ratio_test: kv_num(&map, "ratio", span, "number")?,
            cross_check: kv_bool(&map, "crossCheck", span)?,
            sequential_window: kv_num(&map, "seqWindow", span, "integer")?,
            max_pairs_per_frame: kv_num(&map, "maxPairs", span, "integer")?,
            loop_closure: kv_bool(&map, "loopClosure", span)?,
        })
    }

    fn print_sfm_fields(params: &SfmParams) -> String {
        format!(
            "ransacIter={} ransacThresh={} minTrackLen={} baIter={} robustLoss={} huberDelta={}",
            params.ransac_iterations,
            params.ransac_threshold_px,
            params.min_track_length,
            params.ba_max_iterations,
            robust_loss_str(params.robust_loss),
            params.huber_delta_px
        )
    }
    fn print_sfm(params: &SfmParams) -> String {
        format!("sfm {}", print_sfm_fields(params))
    }
    fn parse_sfm(p: &mut Parser) -> Result<SfmParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(SfmParams {
            ransac_iterations: kv_num(&map, "ransacIter", span, "integer")?,
            ransac_threshold_px: kv_num(&map, "ransacThresh", span, "number")?,
            min_track_length: kv_num(&map, "minTrackLen", span, "integer")?,
            ba_max_iterations: kv_num(&map, "baIter", span, "integer")?,
            robust_loss: parse_robust_loss(&kv_word(&map, "robustLoss", span)?, span)?,
            huber_delta_px: kv_num(&map, "huberDelta", span, "number")?,
        })
    }

    fn print_dense_params_fields(params: &DenseParams) -> String {
        format!(
            "resolution={} windowRadius={} minViewConsistency={} confidence={} maxPoints={}",
            dense_resolution_str(params.resolution),
            params.window_radius_px,
            params.min_view_consistency,
            params.confidence_threshold,
            params.max_points
        )
    }
    fn print_dense_params(params: &DenseParams) -> String {
        format!("dense {}", print_dense_params_fields(params))
    }
    fn parse_dense_params(p: &mut Parser) -> Result<DenseParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(DenseParams {
            resolution: parse_dense_resolution(&kv_word(&map, "resolution", span)?, span)?,
            window_radius_px: kv_num(&map, "windowRadius", span, "integer")?,
            min_view_consistency: kv_num(&map, "minViewConsistency", span, "integer")?,
            confidence_threshold: kv_num(&map, "confidence", span, "number")?,
            max_points: kv_num(&map, "maxPoints", span, "integer")?,
        })
    }

    fn print_mesh_params_fields(params: &MeshParams) -> String {
        format!(
            "voxel={} truncation={} decimateTarget={} smoothing={} texture={} textureSize={} guaranteeWatertight={} holeFillMax={} selfIntersectionCheck={}",
            params.tsdf_voxel_size_mm,
            params.tsdf_truncation_mm,
            params.decimate_target_triangles,
            params.smoothing_iterations,
            params.texture_enabled,
            params.texture_size,
            params.guarantee_watertight,
            params.hole_fill_max_boundary_verts,
            params.self_intersection_check
        )
    }
    fn print_mesh_params(params: &MeshParams) -> String {
        format!("mesh {}", print_mesh_params_fields(params))
    }
    fn parse_mesh_params(p: &mut Parser) -> Result<MeshParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(MeshParams {
            tsdf_voxel_size_mm: kv_num(&map, "voxel", span, "number")?,
            tsdf_truncation_mm: kv_num(&map, "truncation", span, "number")?,
            decimate_target_triangles: kv_num(&map, "decimateTarget", span, "integer")?,
            smoothing_iterations: kv_num(&map, "smoothing", span, "integer")?,
            texture_enabled: kv_bool(&map, "texture", span)?,
            texture_size: kv_num(&map, "textureSize", span, "integer")?,
            guarantee_watertight: kv_bool(&map, "guaranteeWatertight", span)?,
            hole_fill_max_boundary_verts: kv_num(&map, "holeFillMax", span, "integer")?,
            self_intersection_check: kv_bool(&map, "selfIntersectionCheck", span)?,
        })
    }

    fn print_motion_params_fields(params: &MotionParams) -> String {
        format!(
            "enabled={} maxTracks={} windowPx={} minQuality={} minLenFrames={}",
            params.enabled, params.max_tracks, params.track_window_px, params.min_track_quality, params.min_track_length_frames
        )
    }
    fn print_motion_params(params: &MotionParams) -> String {
        format!("motion {}", print_motion_params_fields(params))
    }
    fn parse_motion_params(p: &mut Parser) -> Result<MotionParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(MotionParams {
            enabled: kv_bool(&map, "enabled", span)?,
            max_tracks: kv_num(&map, "maxTracks", span, "integer")?,
            track_window_px: kv_num(&map, "windowPx", span, "integer")?,
            min_track_quality: kv_num(&map, "minQuality", span, "number")?,
            min_track_length_frames: kv_num(&map, "minLenFrames", span, "integer")?,
        })
    }

    fn print_geo_params_fields(params: &GeoParams) -> String {
        format!(
            "enabled={} originLon={} originLat={} originAlt={} gsd={} dsmCell={} dtmRadius={} orthoMax={}",
            params.enabled,
            params.origin_lon.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            params.origin_lat.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            params.origin_alt.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            params.gsd_m,
            params.dsm_cell_m,
            params.dtm_filter_radius_m,
            params.ortho_max_px
        )
    }
    fn print_geo_params(params: &GeoParams) -> String {
        format!("geo {}", print_geo_params_fields(params))
    }
    fn parse_geo_params(p: &mut Parser) -> Result<GeoParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(GeoParams {
            enabled: kv_bool(&map, "enabled", span)?,
            origin_lon: kv_opt_num(&map, "originLon"),
            origin_lat: kv_opt_num(&map, "originLat"),
            origin_alt: kv_opt_num(&map, "originAlt"),
            gsd_m: kv_num(&map, "gsd", span, "number")?,
            dsm_cell_m: kv_num(&map, "dsmCell", span, "number")?,
            dtm_filter_radius_m: kv_num(&map, "dtmRadius", span, "number")?,
            ortho_max_px: kv_num(&map, "orthoMax", span, "integer")?,
        })
    }

    fn print_params(params: &ReconstructionParams) -> String {
        format!(
            "params {{ {} {} {} {} {} {} {} {} }}",
            print_ingest(&params.ingest),
            print_feature(&params.feature),
            print_match(&params.matching),
            print_sfm(&params.sfm),
            print_dense_params(&params.dense),
            print_mesh_params(&params.mesh),
            print_motion_params(&params.motion),
            print_geo_params(&params.geo)
        )
    }
    fn parse_params(p: &mut Parser) -> Result<ReconstructionParams, TextError> {
        let span = p.span();
        p.expect_lbrace()?;
        let mut params = ReconstructionParams::default();
        while !p.at_rbrace() {
            match p.expect_word()?.as_str() {
                "ingest" => params.ingest = parse_ingest(p)?,
                "feature" => params.feature = parse_feature(p)?,
                "match" => params.matching = parse_match(p)?,
                "sfm" => params.sfm = parse_sfm(p)?,
                "dense" => params.dense = parse_dense_params(p)?,
                "mesh" => params.mesh = parse_mesh_params(p)?,
                "motion" => params.motion = parse_motion_params(p)?,
                "geo" => params.geo = parse_geo_params(p)?,
                other => return Err(TextError::new(format!("unknown params child '{other}'"), span)),
            }
        }
        p.expect_rbrace()?;
        Ok(params)
    }

    fn print_pose(pose: &CameraPosePreview) -> String {
        format!("pose camera={} rot={} t={}", pose.camera_id, csv(&pose.rotation_wxyz), csv(&pose.translation))
    }
    fn parse_pose(p: &mut Parser) -> Result<CameraPosePreview, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(CameraPosePreview {
            camera_id: kv_word(&map, "camera", span)?,
            rotation_wxyz: parse_csv::<f32, 4>(&kv_word(&map, "rot", span)?, span)?,
            translation: parse_csv::<f32, 3>(&kv_word(&map, "t", span)?, span)?,
        })
    }

    fn print_job_fields(job: &ReconstructionJob) -> String {
        let mut out = format!(
            "id={} stage={} progress={} cancel={} cursor={} started={} error={} sparsePreview={} {{",
            job.id,
            reconstruction_stage_str(job.stage),
            job.progress_0_1,
            job.cancel_requested,
            job.stage_cursor,
            job.started_at_ms.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            job.error.as_deref().map(quote).unwrap_or_else(|| "-".to_string()),
            print_packed_f32(&job.sparse_point_cloud_preview)
        );
        for pose in &job.camera_poses_preview {
            out.push(' ');
            out.push_str(&print_pose(pose));
        }
        out.push_str(" }");
        out
    }
    fn print_job(job: &ReconstructionJob) -> String {
        format!("job {}", print_job_fields(job))
    }
    fn parse_job(p: &mut Parser) -> Result<ReconstructionJob, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        let id = kv_word(&map, "id", span)?;
        let stage = parse_reconstruction_stage(&kv_word(&map, "stage", span)?, span)?;
        let progress_0_1 = kv_num(&map, "progress", span, "number")?;
        let cancel_requested = kv_bool(&map, "cancel", span)?;
        let stage_cursor = kv_num(&map, "cursor", span, "integer")?;
        let started_at_ms = kv_opt_num(&map, "started");
        let error = kv_opt_str(&map, "error");
        let sparse_point_cloud_preview = parse_packed_f32(&kv_word(&map, "sparsePreview", span)?);
        p.expect_lbrace()?;
        let mut camera_poses_preview = Vec::new();
        while !p.at_rbrace() {
            p.expect_word()?;
            camera_poses_preview.push(parse_pose(p)?);
        }
        p.expect_rbrace()?;
        Ok(ReconstructionJob { id, stage, progress_0_1, cancel_requested, stage_cursor, started_at_ms, error, camera_poses_preview, sparse_point_cloud_preview })
    }

    fn print_watertight(report: &WatertightReportSnapshot) -> String {
        format!(
            "{{ vertexCount={} triangleCount={} boundaryEdgeCount={} boundaryLoopCount={} nonManifoldEdgeCount={} nonManifoldVertexCount={} connectedComponents={} consistentlyOriented={} euler={} genus={} signedVolume={} selfIntersectionPairs={} closedFallbackUsed={} isClosed={} isTwoManifold={} isWatertight={} }}",
            report.vertex_count,
            report.triangle_count,
            report.boundary_edge_count,
            report.boundary_loop_count,
            report.non_manifold_edge_count,
            report.non_manifold_vertex_count,
            report.connected_components,
            report.consistently_oriented,
            report.euler_characteristic,
            report.genus.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            report.signed_volume,
            report.self_intersection_pairs.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            report.closed_fallback_used,
            report.is_closed,
            report.is_two_manifold,
            report.is_watertight
        )
    }
    fn parse_watertight(p: &mut Parser) -> Result<WatertightReportSnapshot, TextError> {
        let span = p.span();
        p.expect_lbrace()?;
        let map = p.parse_kv_map()?;
        p.expect_rbrace()?;
        Ok(WatertightReportSnapshot {
            vertex_count: kv_num(&map, "vertexCount", span, "integer")?,
            triangle_count: kv_num(&map, "triangleCount", span, "integer")?,
            boundary_edge_count: kv_num(&map, "boundaryEdgeCount", span, "integer")?,
            boundary_loop_count: kv_num(&map, "boundaryLoopCount", span, "integer")?,
            non_manifold_edge_count: kv_num(&map, "nonManifoldEdgeCount", span, "integer")?,
            non_manifold_vertex_count: kv_num(&map, "nonManifoldVertexCount", span, "integer")?,
            connected_components: kv_num(&map, "connectedComponents", span, "integer")?,
            consistently_oriented: kv_bool(&map, "consistentlyOriented", span)?,
            euler_characteristic: kv_num(&map, "euler", span, "integer")?,
            genus: kv_opt_num(&map, "genus"),
            signed_volume: kv_num(&map, "signedVolume", span, "number")?,
            self_intersection_pairs: kv_opt_num(&map, "selfIntersectionPairs"),
            closed_fallback_used: kv_bool(&map, "closedFallbackUsed", span)?,
            is_closed: kv_bool(&map, "isClosed", span)?,
            is_two_manifold: kv_bool(&map, "isTwoManifold", span)?,
            is_watertight: kv_bool(&map, "isWatertight", span)?,
        })
    }

    /// 🧵 Prints `RemodelMesh`'s flat fields (no leading keyword — callers prefix `mesh`/`setMeshResult`)
    /// followed by an optional trailing `{ ... }` watertight block; `MeshData`'s numeric buffers are
    /// always base64-packed (`🔖Packed`) regardless of size, so this never emits per-element text.
    fn print_mesh_body(mesh: &RemodelMesh) -> String {
        let data = &mesh.mesh;
        let mut out = format!(
            "source={} texture={} positions={} normals={} colors={} indices={} uvs={} faceIds={} vertexIds={} edgePositions={} edgeIds={} edgeUvs={} edgeIsSeam={} paintTexture={}",
            mesh_source_str(mesh.source),
            mesh.texture_asset_id.as_deref().unwrap_or("-"),
            print_packed_f32(&PackedF32::from_f32_slice(&data.positions)),
            print_packed_f32(&PackedF32::from_f32_slice(&data.normals)),
            print_packed_f32(&PackedF32::from_f32_slice(&data.colors)),
            print_packed_u32(&data.indices),
            print_packed_f32(&PackedF32::from_f32_slice(&data.uvs)),
            print_packed_u32(&data.face_ids),
            print_packed_u32(&data.vertex_ids),
            print_packed_f32(&PackedF32::from_f32_slice(&data.edge_positions)),
            print_packed_u32(&data.edge_ids),
            print_packed_f32(&PackedF32::from_f32_slice(&data.edge_uvs)),
            print_packed_u8(&PackedU8::from_u8_slice(&data.edge_is_seam)),
            data.paint_texture_base64.as_deref().unwrap_or("-")
        );
        if let Some(watertight) = &mesh.watertight {
            out.push(' ');
            out.push_str(&print_watertight(watertight));
        }
        out
    }
    fn parse_mesh_body(p: &mut Parser) -> Result<RemodelMesh, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        let mesh = MeshData {
            positions: parse_packed_f32(&kv_word(&map, "positions", span)?).to_f32_vec(),
            normals: parse_packed_f32(&kv_word(&map, "normals", span)?).to_f32_vec(),
            colors: parse_packed_f32(&kv_word(&map, "colors", span)?).to_f32_vec(),
            indices: parse_packed_u32(&kv_word(&map, "indices", span)?),
            uvs: parse_packed_f32(&kv_word(&map, "uvs", span)?).to_f32_vec(),
            face_ids: parse_packed_u32(&kv_word(&map, "faceIds", span)?),
            vertex_ids: parse_packed_u32(&kv_word(&map, "vertexIds", span)?),
            edge_positions: parse_packed_f32(&kv_word(&map, "edgePositions", span)?).to_f32_vec(),
            edge_ids: parse_packed_u32(&kv_word(&map, "edgeIds", span)?),
            edge_uvs: parse_packed_f32(&kv_word(&map, "edgeUvs", span)?).to_f32_vec(),
            edge_is_seam: parse_packed_u8(&kv_word(&map, "edgeIsSeam", span)?).to_u8_vec(),
            paint_texture_base64: kv_opt_word(&map, "paintTexture"),
        };
        let watertight = if p.at_lbrace() { Some(parse_watertight(p)?) } else { None };
        Ok(RemodelMesh { mesh, source: parse_mesh_source(&kv_word(&map, "source", span)?, span)?, texture_asset_id: kv_opt_word(&map, "texture"), watertight })
    }

    fn print_track(track: &MotionTrackSummary) -> String {
        format!("track id={} length={} class={} speed={}", track.id, track.length, track_class_str(track.class), track.mean_speed_m_s)
    }
    fn parse_track(p: &mut Parser) -> Result<MotionTrackSummary, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(MotionTrackSummary {
            id: kv_word(&map, "id", span)?,
            length: kv_num(&map, "length", span, "integer")?,
            class: parse_track_class(&kv_word(&map, "class", span)?, span)?,
            mean_speed_m_s: kv_num(&map, "speed", span, "number")?,
        })
    }

    fn print_results(results: &ReconstructionResults) -> String {
        let mut out = "results {".to_string();
        out.push(' ');
        out.push_str(&match &results.sparse {
            None => "sparse -".to_string(),
            Some(sparse) => format!("sparse points={} colors={}", print_packed_f32(&sparse.points), sparse.colors.as_ref().map(print_packed_u8).unwrap_or_else(|| "-".to_string())),
        });
        out.push(' ');
        out.push_str(&match &results.dense {
            None => "dense -".to_string(),
            Some(dense) => format!(
                "dense positions={} colors={} confidence={} classification={}",
                print_packed_f32(&dense.positions),
                dense.colors.as_ref().map(print_packed_u8).unwrap_or_else(|| "-".to_string()),
                dense.confidence.as_ref().map(print_packed_f32).unwrap_or_else(|| "-".to_string()),
                dense.classification.as_ref().map(print_packed_u8).unwrap_or_else(|| "-".to_string()),
            ),
        });
        out.push(' ');
        out.push_str("mesh ");
        out.push_str(&print_mesh_body(&results.mesh));
        out.push(' ');
        out.push_str(&match &results.trajectory {
            None => "trajectory -".to_string(),
            Some(trajectory) => {
                let mut t = "trajectory {".to_string();
                for pose in &trajectory.poses {
                    t.push(' ');
                    t.push_str(&print_pose(pose));
                }
                t.push_str(" }");
                t
            }
        });
        for track in &results.tracks {
            out.push(' ');
            out.push_str(&print_track(track));
        }
        out.push(' ');
        out.push_str(&match &results.geo {
            None => "geoProducts -".to_string(),
            Some(geo) => format!(
                "geoProducts dsm={} dtm={} ortho={}",
                geo.dsm_asset_id.as_deref().unwrap_or("-"),
                geo.dtm_asset_id.as_deref().unwrap_or("-"),
                geo.ortho_asset_id.as_deref().unwrap_or("-")
            ),
        });
        out.push(' ');
        out.push_str(&match &results.qc {
            None => "qc -".to_string(),
            Some(qc) => {
                let mut q = format!(
                    "qc reprojRms={} gcpRmse={} meanTrackLen={} registeredRatio={} denseCoverage={}",
                    qc.reprojection_rms_px,
                    qc.gcp_checkpoint_rmse.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
                    qc.mean_track_length,
                    qc.registered_frame_ratio,
                    qc.dense_coverage_ratio
                );
                for warning in &qc.warnings {
                    q.push(' ');
                    q.push_str(&quote(warning));
                }
                if let Some(watertight) = &qc.watertight {
                    q.push(' ');
                    q.push_str(&print_watertight(watertight));
                }
                q
            }
        });
        out.push_str(" }");
        out
    }
    fn parse_results(p: &mut Parser) -> Result<ReconstructionResults, TextError> {
        let span = p.span();
        p.expect_lbrace()?;
        let mut results = ReconstructionResults::default();
        while !p.at_rbrace() {
            match p.expect_word()?.as_str() {
                "sparse" => {
                    results.sparse = if p.at_dash() {
                        p.bump();
                        None
                    } else {
                        let inner_span = p.span();
                        let map = p.parse_kv_map()?;
                        Some(SparseCloud { points: parse_packed_f32(&kv_word(&map, "points", inner_span)?), colors: kv_opt_word(&map, "colors").map(|w| parse_packed_u8(&w)) })
                    };
                }
                "dense" => {
                    results.dense = if p.at_dash() {
                        p.bump();
                        None
                    } else {
                        let inner_span = p.span();
                        let map = p.parse_kv_map()?;
                        Some(DenseCloud {
                            positions: parse_packed_f32(&kv_word(&map, "positions", inner_span)?),
                            colors: kv_opt_word(&map, "colors").map(|w| parse_packed_u8(&w)),
                            confidence: kv_opt_word(&map, "confidence").map(|w| parse_packed_f32(&w)),
                            classification: kv_opt_word(&map, "classification").map(|w| parse_packed_u8(&w)),
                        })
                    };
                }
                "mesh" => results.mesh = parse_mesh_body(p)?,
                "trajectory" => {
                    results.trajectory = if p.at_dash() {
                        p.bump();
                        None
                    } else {
                        p.expect_lbrace()?;
                        let mut poses = Vec::new();
                        while !p.at_rbrace() {
                            p.expect_word()?;
                            poses.push(parse_pose(p)?);
                        }
                        p.expect_rbrace()?;
                        Some(CameraTrajectory { poses })
                    };
                }
                "track" => results.tracks.push(parse_track(p)?),
                "geoProducts" => {
                    results.geo = if p.at_dash() {
                        p.bump();
                        None
                    } else {
                        let map = p.parse_kv_map()?;
                        Some(GeoProducts { dsm_asset_id: kv_opt_word(&map, "dsm"), dtm_asset_id: kv_opt_word(&map, "dtm"), ortho_asset_id: kv_opt_word(&map, "ortho") })
                    };
                }
                "qc" => {
                    results.qc = if p.at_dash() {
                        p.bump();
                        None
                    } else {
                        let inner_span = p.span();
                        let map = p.parse_kv_map()?;
                        let warnings = p.greedy_str_list();
                        let watertight = if p.at_lbrace() { Some(parse_watertight(p)?) } else { None };
                        Some(QcReportSnapshot {
                            reprojection_rms_px: kv_num(&map, "reprojRms", inner_span, "number")?,
                            gcp_checkpoint_rmse: kv_opt_num(&map, "gcpRmse"),
                            watertight,
                            mean_track_length: kv_num(&map, "meanTrackLen", inner_span, "number")?,
                            registered_frame_ratio: kv_num(&map, "registeredRatio", inner_span, "number")?,
                            dense_coverage_ratio: kv_num(&map, "denseCoverage", inner_span, "number")?,
                            warnings,
                        })
                    };
                }
                other => return Err(TextError::new(format!("unknown results child '{other}'"), span)),
            }
        }
        p.expect_rbrace()?;
        Ok(results)
    }
    //#endregion Constructs

    //#region Document
    pub(super) fn print_document(scene: &RemodelScene) -> String {
        let mut out = format!("remodel schema={} id={} {{\n", scene.schema, scene.id);
        for stream in &scene.streams {
            out.push_str("  ");
            out.push_str(&print_stream(stream));
            out.push('\n');
        }
        for (id, asset) in &scene.assets {
            out.push_str("  ");
            out.push_str(&print_asset(id, asset));
            out.push('\n');
        }
        out.push_str("  ");
        out.push_str(&print_calibration(&scene.calibration));
        out.push('\n');
        for gcp in &scene.gcps {
            out.push_str("  ");
            out.push_str(&print_gcp(gcp));
            out.push('\n');
        }
        out.push_str("  ");
        out.push_str(&print_params(&scene.params));
        out.push('\n');
        out.push_str("  ");
        out.push_str(&print_job(&scene.job));
        out.push('\n');
        out.push_str("  ");
        out.push_str(&print_results(&scene.results));
        out.push('\n');
        out.push('}');
        out
    }

    pub(super) fn parse_document(text: &str) -> Result<RemodelScene, TextError> {
        let toks = lex(text)?;
        let mut p = Parser { toks, pos: 0 };
        let span = p.span();
        p.expect_word().and_then(|w| if w == "remodel" { Ok(()) } else { Err(TextError::expected(format!("expected 'remodel', found '{w}'"), span, "remodel")) })?;
        let map = p.parse_kv_map()?;
        let schema = kv_word(&map, "schema", span)?;
        let id = kv_word(&map, "id", span)?;
        p.expect_lbrace()?;
        let mut streams = Vec::new();
        let mut assets = BTreeMap::new();
        let mut calibration = CalibrationState::default();
        let mut gcps = Vec::new();
        let mut params = ReconstructionParams::default();
        let mut job = ReconstructionJob::default();
        let mut results = ReconstructionResults::default();
        while !p.at_rbrace() {
            match p.expect_word()?.as_str() {
                "stream" => streams.push(parse_stream(&mut p)?),
                "asset" => {
                    let (asset_id, asset) = parse_asset(&mut p)?;
                    assets.insert(asset_id, asset);
                }
                "calibration" => calibration = parse_calibration(&mut p)?,
                "gcp" => gcps.push(parse_gcp(&mut p)?),
                "params" => params = parse_params(&mut p)?,
                "job" => job = parse_job(&mut p)?,
                "results" => results = parse_results(&mut p)?,
                other => return Err(TextError::new(format!("unknown remodel document child '{other}'"), span)),
            }
        }
        p.expect_rbrace()?;
        let _ = p.at_eof();
        Ok(RemodelScene { schema, id, streams, assets, calibration, params, gcps, job, results })
    }
    //#endregion Document

    //#region Ops
    pub(super) fn print_operation(operation: &RemodelOperation) -> String {
        match operation {
            RemodelOperation::SetStreams { streams } => {
                let mut out = "setStreams {".to_string();
                for stream in streams {
                    out.push(' ');
                    out.push_str(&print_stream(stream));
                }
                out.push_str(" }");
                out
            }
            RemodelOperation::SetAsset { key, value } => match value {
                None => format!("setAsset key={key} value=-"),
                Some(asset) => format!("setAsset key={key} {}", print_asset_fields(asset)),
            },
            RemodelOperation::SetCalibration { calibration } => format!("setCalibration {}", print_calibration_fields(calibration)),
            RemodelOperation::SetGcps { gcps } => {
                let mut out = "setGcps {".to_string();
                for gcp in gcps {
                    out.push(' ');
                    out.push_str(&print_gcp(gcp));
                }
                out.push_str(" }");
                out
            }
            RemodelOperation::SetIngestParams { params } => format!("setIngestParams {}", print_ingest_fields(params)),
            RemodelOperation::SetFeatureParams { params } => format!("setFeatureParams {}", print_feature_fields(params)),
            RemodelOperation::SetMatchParams { params } => format!("setMatchParams {}", print_match_fields(params)),
            RemodelOperation::SetSfmParams { params } => format!("setSfmParams {}", print_sfm_fields(params)),
            RemodelOperation::SetDenseParams { params } => format!("setDenseParams {}", print_dense_params_fields(params)),
            RemodelOperation::SetMeshParams { params } => format!("setMeshParams {}", print_mesh_params_fields(params)),
            RemodelOperation::SetMotionParams { params } => format!("setMotionParams {}", print_motion_params_fields(params)),
            RemodelOperation::SetGeoParams { params } => format!("setGeoParams {}", print_geo_params_fields(params)),
            RemodelOperation::SetJob { job } => format!("setJob {}", print_job_fields(job)),
            RemodelOperation::SetSparse { sparse } => match sparse {
                None => "setSparse -".to_string(),
                Some(sparse) => format!("setSparse points={} colors={}", print_packed_f32(&sparse.points), sparse.colors.as_ref().map(print_packed_u8).unwrap_or_else(|| "-".to_string())),
            },
            RemodelOperation::SetDense { dense } => match dense {
                None => "setDense -".to_string(),
                Some(dense) => format!(
                    "setDense positions={} colors={} confidence={} classification={}",
                    print_packed_f32(&dense.positions),
                    dense.colors.as_ref().map(print_packed_u8).unwrap_or_else(|| "-".to_string()),
                    dense.confidence.as_ref().map(print_packed_f32).unwrap_or_else(|| "-".to_string()),
                    dense.classification.as_ref().map(print_packed_u8).unwrap_or_else(|| "-".to_string()),
                ),
            },
            RemodelOperation::SetMeshResult { mesh } => format!("setMeshResult {}", print_mesh_body(mesh)),
            RemodelOperation::SetTrajectory { trajectory } => match trajectory {
                None => "setTrajectory -".to_string(),
                Some(trajectory) => {
                    let mut out = "setTrajectory {".to_string();
                    for pose in &trajectory.poses {
                        out.push(' ');
                        out.push_str(&print_pose(pose));
                    }
                    out.push_str(" }");
                    out
                }
            },
            RemodelOperation::SetTracks { tracks } => {
                let mut out = "setTracks {".to_string();
                for track in tracks {
                    out.push(' ');
                    out.push_str(&print_track(track));
                }
                out.push_str(" }");
                out
            }
            RemodelOperation::SetGeoProducts { geo } => match geo {
                None => "setGeoProducts -".to_string(),
                Some(geo) => format!(
                    "setGeoProducts dsm={} dtm={} ortho={}",
                    geo.dsm_asset_id.as_deref().unwrap_or("-"),
                    geo.dtm_asset_id.as_deref().unwrap_or("-"),
                    geo.ortho_asset_id.as_deref().unwrap_or("-")
                ),
            },
            RemodelOperation::SetQc { qc } => match qc {
                None => "setQc -".to_string(),
                Some(qc) => {
                    let mut out = format!(
                        "setQc reprojRms={} gcpRmse={} meanTrackLen={} registeredRatio={} denseCoverage={}",
                        qc.reprojection_rms_px,
                        qc.gcp_checkpoint_rmse.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
                        qc.mean_track_length,
                        qc.registered_frame_ratio,
                        qc.dense_coverage_ratio
                    );
                    for warning in &qc.warnings {
                        out.push(' ');
                        out.push_str(&quote(warning));
                    }
                    if let Some(watertight) = &qc.watertight {
                        out.push(' ');
                        out.push_str(&print_watertight(watertight));
                    }
                    out
                }
            },
        }
    }

    pub(super) fn parse_operation(line: &str) -> Result<RemodelOperation, TextError> {
        let toks = lex(line)?;
        let mut p = Parser { toks, pos: 0 };
        let verb = p.expect_word()?;
        match verb.as_str() {
            "setStreams" => {
                p.expect_lbrace()?;
                let mut streams = Vec::new();
                while !p.at_rbrace() {
                    p.expect_word()?;
                    streams.push(parse_stream(&mut p)?);
                }
                p.expect_rbrace()?;
                Ok(RemodelOperation::SetStreams { streams })
            }
            "setAsset" => {
                let span = p.span();
                let map = p.parse_kv_map()?;
                let key = kv_word(&map, "key", span)?;
                let value = if kv_opt_word(&map, "value").is_none() && map.contains_key("value") {
                    None
                } else {
                    Some(ImageAsset { mime: kv_word(&map, "mime", span)?, width: kv_num(&map, "width", span, "integer")?, height: kv_num(&map, "height", span, "integer")?, data: kv_word(&map, "data", span)? })
                };
                Ok(RemodelOperation::SetAsset { key, value })
            }
            "setCalibration" => {
                p.expect_lbrace()?;
                let mut cameras = Vec::new();
                let mut rig = Vec::new();
                while !p.at_rbrace() {
                    match p.expect_word()?.as_str() {
                        "camera" => cameras.push(parse_camera(&mut p)?),
                        "rig" => rig.push(parse_rig(&mut p)?),
                        other => return Err(TextError::new(format!("unknown calibration child '{other}'"), p.span())),
                    }
                }
                p.expect_rbrace()?;
                Ok(RemodelOperation::SetCalibration { calibration: CalibrationState { cameras, rig } })
            }
            "setGcps" => {
                p.expect_lbrace()?;
                let mut gcps = Vec::new();
                while !p.at_rbrace() {
                    p.expect_word()?;
                    gcps.push(parse_gcp(&mut p)?);
                }
                p.expect_rbrace()?;
                Ok(RemodelOperation::SetGcps { gcps })
            }
            "setIngestParams" => Ok(RemodelOperation::SetIngestParams { params: parse_ingest_fields(&mut p)? }),
            "setFeatureParams" => Ok(RemodelOperation::SetFeatureParams { params: parse_feature_fields(&mut p)? }),
            "setMatchParams" => Ok(RemodelOperation::SetMatchParams { params: parse_match_fields(&mut p)? }),
            "setSfmParams" => Ok(RemodelOperation::SetSfmParams { params: parse_sfm_fields(&mut p)? }),
            "setDenseParams" => Ok(RemodelOperation::SetDenseParams { params: parse_dense_params_fields(&mut p)? }),
            "setMeshParams" => Ok(RemodelOperation::SetMeshParams { params: parse_mesh_params_fields(&mut p)? }),
            "setMotionParams" => Ok(RemodelOperation::SetMotionParams { params: parse_motion_params_fields(&mut p)? }),
            "setGeoParams" => Ok(RemodelOperation::SetGeoParams { params: parse_geo_params_fields(&mut p)? }),
            "setJob" => {
                let span = p.span();
                let map = p.parse_kv_map()?;
                let id = kv_word(&map, "id", span)?;
                let stage = parse_reconstruction_stage(&kv_word(&map, "stage", span)?, span)?;
                let progress_0_1 = kv_num(&map, "progress", span, "number")?;
                let cancel_requested = kv_bool(&map, "cancel", span)?;
                let stage_cursor = kv_num(&map, "cursor", span, "integer")?;
                let started_at_ms = kv_opt_num(&map, "started");
                let error = kv_opt_str(&map, "error");
                let sparse_point_cloud_preview = parse_packed_f32(&kv_word(&map, "sparsePreview", span)?);
                p.expect_lbrace()?;
                let mut camera_poses_preview = Vec::new();
                while !p.at_rbrace() {
                    p.expect_word()?;
                    camera_poses_preview.push(parse_pose(&mut p)?);
                }
                p.expect_rbrace()?;
                Ok(RemodelOperation::SetJob { job: ReconstructionJob { id, stage, progress_0_1, cancel_requested, stage_cursor, started_at_ms, error, camera_poses_preview, sparse_point_cloud_preview } })
            }
            "setSparse" => {
                if p.at_dash() {
                    p.bump();
                    return Ok(RemodelOperation::SetSparse { sparse: None });
                }
                let span = p.span();
                let map = p.parse_kv_map()?;
                Ok(RemodelOperation::SetSparse {
                    sparse: Some(SparseCloud { points: parse_packed_f32(&kv_word(&map, "points", span)?), colors: kv_opt_word(&map, "colors").map(|w| parse_packed_u8(&w)) }),
                })
            }
            "setDense" => {
                if p.at_dash() {
                    p.bump();
                    return Ok(RemodelOperation::SetDense { dense: None });
                }
                let span = p.span();
                let map = p.parse_kv_map()?;
                Ok(RemodelOperation::SetDense {
                    dense: Some(DenseCloud {
                        positions: parse_packed_f32(&kv_word(&map, "positions", span)?),
                        colors: kv_opt_word(&map, "colors").map(|w| parse_packed_u8(&w)),
                        confidence: kv_opt_word(&map, "confidence").map(|w| parse_packed_f32(&w)),
                        classification: kv_opt_word(&map, "classification").map(|w| parse_packed_u8(&w)),
                    }),
                })
            }
            "setMeshResult" => Ok(RemodelOperation::SetMeshResult { mesh: Box::new(parse_mesh_body(&mut p)?) }),
            "setTrajectory" => {
                if p.at_dash() {
                    p.bump();
                    return Ok(RemodelOperation::SetTrajectory { trajectory: None });
                }
                p.expect_lbrace()?;
                let mut poses = Vec::new();
                while !p.at_rbrace() {
                    p.expect_word()?;
                    poses.push(parse_pose(&mut p)?);
                }
                p.expect_rbrace()?;
                Ok(RemodelOperation::SetTrajectory { trajectory: Some(CameraTrajectory { poses }) })
            }
            "setTracks" => {
                p.expect_lbrace()?;
                let mut tracks = Vec::new();
                while !p.at_rbrace() {
                    p.expect_word()?;
                    tracks.push(parse_track(&mut p)?);
                }
                p.expect_rbrace()?;
                Ok(RemodelOperation::SetTracks { tracks })
            }
            "setGeoProducts" => {
                if p.at_dash() {
                    p.bump();
                    return Ok(RemodelOperation::SetGeoProducts { geo: None });
                }
                let map = p.parse_kv_map()?;
                Ok(RemodelOperation::SetGeoProducts {
                    geo: Some(GeoProducts { dsm_asset_id: kv_opt_word(&map, "dsm"), dtm_asset_id: kv_opt_word(&map, "dtm"), ortho_asset_id: kv_opt_word(&map, "ortho") }),
                })
            }
            "setQc" => {
                if p.at_dash() {
                    p.bump();
                    return Ok(RemodelOperation::SetQc { qc: None });
                }
                let span = p.span();
                let map = p.parse_kv_map()?;
                let warnings = p.greedy_str_list();
                let watertight = if p.at_lbrace() { Some(parse_watertight(&mut p)?) } else { None };
                Ok(RemodelOperation::SetQc {
                    qc: Some(QcReportSnapshot {
                        reprojection_rms_px: kv_num(&map, "reprojRms", span, "number")?,
                        gcp_checkpoint_rmse: kv_opt_num(&map, "gcpRmse"),
                        watertight,
                        mean_track_length: kv_num(&map, "meanTrackLen", span, "number")?,
                        registered_frame_ratio: kv_num(&map, "registeredRatio", span, "number")?,
                        dense_coverage_ratio: kv_num(&map, "denseCoverage", span, "number")?,
                        warnings,
                    }),
                })
            }
            other => Err(TextError::new(format!("unknown operation '{other}'"), p.span())),
        }
    }

    fn parse_ingest_fields(p: &mut Parser) -> Result<IngestParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(IngestParams {
            frame_sample_stride: kv_num(&map, "stride", span, "integer")?,
            max_frames: kv_num(&map, "maxFrames", span, "integer")?,
            downscale_long_edge_px: kv_num(&map, "downscale", span, "integer")?,
            min_sharpness: kv_num(&map, "minSharpness", span, "number")?,
        })
    }
    fn parse_feature_fields(p: &mut Parser) -> Result<FeatureParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(FeatureParams {
            detector: parse_feature_detector(&kv_word(&map, "detector", span)?, span)?,
            target_count: kv_num(&map, "targetCount", span, "integer")?,
            octaves: kv_num(&map, "octaves", span, "integer")?,
            edge_threshold: kv_num(&map, "edgeThreshold", span, "number")?,
        })
    }
    fn parse_match_fields(p: &mut Parser) -> Result<MatchParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(MatchParams {
            matcher: parse_matcher_kind(&kv_word(&map, "matcher", span)?, span)?,
            ratio_test: kv_num(&map, "ratio", span, "number")?,
            cross_check: kv_bool(&map, "crossCheck", span)?,
            sequential_window: kv_num(&map, "seqWindow", span, "integer")?,
            max_pairs_per_frame: kv_num(&map, "maxPairs", span, "integer")?,
            loop_closure: kv_bool(&map, "loopClosure", span)?,
        })
    }
    fn parse_sfm_fields(p: &mut Parser) -> Result<SfmParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(SfmParams {
            ransac_iterations: kv_num(&map, "ransacIter", span, "integer")?,
            ransac_threshold_px: kv_num(&map, "ransacThresh", span, "number")?,
            min_track_length: kv_num(&map, "minTrackLen", span, "integer")?,
            ba_max_iterations: kv_num(&map, "baIter", span, "integer")?,
            robust_loss: parse_robust_loss(&kv_word(&map, "robustLoss", span)?, span)?,
            huber_delta_px: kv_num(&map, "huberDelta", span, "number")?,
        })
    }
    fn parse_dense_params_fields(p: &mut Parser) -> Result<DenseParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(DenseParams {
            resolution: parse_dense_resolution(&kv_word(&map, "resolution", span)?, span)?,
            window_radius_px: kv_num(&map, "windowRadius", span, "integer")?,
            min_view_consistency: kv_num(&map, "minViewConsistency", span, "integer")?,
            confidence_threshold: kv_num(&map, "confidence", span, "number")?,
            max_points: kv_num(&map, "maxPoints", span, "integer")?,
        })
    }
    fn parse_mesh_params_fields(p: &mut Parser) -> Result<MeshParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(MeshParams {
            tsdf_voxel_size_mm: kv_num(&map, "voxel", span, "number")?,
            tsdf_truncation_mm: kv_num(&map, "truncation", span, "number")?,
            decimate_target_triangles: kv_num(&map, "decimateTarget", span, "integer")?,
            smoothing_iterations: kv_num(&map, "smoothing", span, "integer")?,
            texture_enabled: kv_bool(&map, "texture", span)?,
            texture_size: kv_num(&map, "textureSize", span, "integer")?,
            guarantee_watertight: kv_bool(&map, "guaranteeWatertight", span)?,
            hole_fill_max_boundary_verts: kv_num(&map, "holeFillMax", span, "integer")?,
            self_intersection_check: kv_bool(&map, "selfIntersectionCheck", span)?,
        })
    }
    fn parse_motion_params_fields(p: &mut Parser) -> Result<MotionParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(MotionParams {
            enabled: kv_bool(&map, "enabled", span)?,
            max_tracks: kv_num(&map, "maxTracks", span, "integer")?,
            track_window_px: kv_num(&map, "windowPx", span, "integer")?,
            min_track_quality: kv_num(&map, "minQuality", span, "number")?,
            min_track_length_frames: kv_num(&map, "minLenFrames", span, "integer")?,
        })
    }
    fn parse_geo_params_fields(p: &mut Parser) -> Result<GeoParams, TextError> {
        let span = p.span();
        let map = p.parse_kv_map()?;
        Ok(GeoParams {
            enabled: kv_bool(&map, "enabled", span)?,
            origin_lon: kv_opt_num(&map, "originLon"),
            origin_lat: kv_opt_num(&map, "originLat"),
            origin_alt: kv_opt_num(&map, "originAlt"),
            gsd_m: kv_num(&map, "gsd", span, "number")?,
            dsm_cell_m: kv_num(&map, "dsmCell", span, "number")?,
            dtm_filter_radius_m: kv_num(&map, "dtmRadius", span, "number")?,
            ortho_max_px: kv_num(&map, "orthoMax", span, "integer")?,
        })
    }
    //#endregion Ops
}

impl DocumentDsl for RemodelScene {
    const EXTENSION: &'static str = "remodel";
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        remodel_text::parse_document(text)
    }
    fn print_dsl(&self) -> String {
        remodel_text::print_document(self)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
impl OpText for RemodelOperation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        remodel_text::parse_operation(line)
    }
    fn print_op(&self) -> String {
        remodel_text::print_operation(self)
    }
}
//#endregion 🔖OpText

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
    }

    #[test]
    fn populated_scene_roundtrips_through_dsl() {
        vcs::test_support::assert_dsl_round_trip(&populated_scene_fixture());
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
    }
    //#endregion 🔖DslAndOpText
}
//#endregion 🧪Tests
