//! 📸 Remodel scene document — schema-only photogrammetry/videogrammetry project state (media
//! streams, calibration, ground control points, reconstruction params/job/results) shared as CRDT
//! ops. The actual algorithms live in sibling `remodel_image`/`remodel_camera`/`remodel_feature`/
//! `remodel_sfm`/`remodel_dense`/`remodel_mesh`/`remodel_motion`/`remodel_geo`/`remodel_engine`
//! crates, none of which this crate depends on.

use base64::Engine as _;
use semio_framework_core::MeshData;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use vcs::{Operation, OperationDiff};

pub const REMODEL_DOCUMENT_SCHEMA: &str = "remodel.scene";

//#region 🔖Packed
/// 📦 A flat `f32` buffer that serializes as a base64 string of its little-endian bytes rather than a
/// JSON array — point clouds and height grids commonly carry 10^5-10^6 elements, where per-element
/// JSON text is both far larger on the wire and far slower to parse than one base64 blob.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PackedF32(pub Vec<f32>);

impl PackedF32 {
    pub fn from_vec(values: Vec<f32>) -> Self {
        Self(values)
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Serialize for PackedF32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes: Vec<u8> = self.0.iter().flat_map(|value| value.to_le_bytes()).collect();
        serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes))
    }
}

impl<'de> Deserialize<'de> for PackedF32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded.as_bytes())
            .map_err(serde::de::Error::custom)?;
        let (chunks, remainder) = bytes.as_chunks::<4>();
        if !remainder.is_empty() {
            return Err(serde::de::Error::custom("packed f32 byte length not a multiple of 4"));
        }
        Ok(Self(chunks.iter().map(|chunk| f32::from_le_bytes(*chunk)).collect()))
    }
}

/// 📦 A flat `u8` buffer (vertex colors, classification codes) that serializes as a base64 string
/// directly — same rationale as {@link PackedF32}.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PackedU8(pub Vec<u8>);

impl PackedU8 {
    pub fn from_vec(values: Vec<u8>) -> Self {
        Self(values)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Serialize for PackedU8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for PackedU8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded.as_bytes())
            .map_err(serde::de::Error::custom)?;
        Ok(Self(bytes))
    }
}
//#endregion 🔖Packed

//#region 🔖Domain
/// 🖼️ One embedded pixel asset (video frame, ortho tile, texture) referenced by id from
/// `RemodelScene::assets`, `MediaStream.frames`, `RemodelMesh.texture_asset_id`, or
/// `GeoProducts.ortho_asset_id`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageAsset {
    pub mime: String,
    pub data: String,
    pub width: u32,
    pub height: u32,
}

/// 🗂️ Which shape a `MediaStream`'s frames were captured as.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MediaKind {
    #[default]
    ImageSequence,
    Video,
}

/// 🎞️ One imported media source (an image sequence or a video), decoded into `FrameRef`s pointing at
/// `RemodelScene::assets`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MediaStream {
    pub id: String,
    pub name: String,
    pub kind: MediaKind,
    pub camera_id: String,
    pub sync_offset_ms: f64,
    pub fps_hint: Option<f32>,
    pub frames: Vec<FrameRef>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameRef {
    pub index: u32,
    pub timestamp_ms: f64,
    pub asset_id: String,
}

/// 🎯 Lens model a `CameraCalibration` was solved under.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CameraModel {
    #[default]
    Pinhole,
    BrownConrady,
    Fisheye,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraCalibration {
    pub id: String,
    pub label: String,
    pub model: CameraModel,
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
    pub label: String,
    pub world: [f64; 3],
    pub enabled: bool,
    pub observations: Vec<GcpObservation>,
}

/// ⏭️ Frame sampling/decode limits `remodel_engine` applies before feature extraction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct IngestParams {
    pub frame_sample_stride: u32,
    pub max_frames: u32,
    pub downscale_long_edge_px: u32,
}

impl Default for IngestParams {
    fn default() -> Self {
        Self { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600 }
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextureSize {
    S1024,
    #[default]
    S2048,
    S4096,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MeshParams {
    pub tsdf_voxel_size_mm: f32,
    pub tsdf_truncation_mm: f32,
    pub decimate_target_triangles: u32,
    pub smoothing_iterations: u32,
    pub texture_enabled: bool,
    pub texture_size: TextureSize,
}

impl Default for MeshParams {
    fn default() -> Self {
        Self {
            tsdf_voxel_size_mm: 5.0,
            tsdf_truncation_mm: 20.0,
            decimate_target_triangles: 200_000,
            smoothing_iterations: 2,
            texture_enabled: true,
            texture_size: TextureSize::default(),
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
/// these directly to configure `remodel_image`/`remodel_camera`/`remodel_feature`/`remodel_sfm`/
/// `remodel_dense`/`remodel_mesh`/`remodel_motion`/`remodel_geo` without this crate depending on any
/// of them.
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

/// 📷 A single recovered camera pose, streamed early for live preview during sparse reconstruction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraPosePreview {
    pub frame_index: u32,
    pub position: [f32; 3],
    pub target: [f32; 3],
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionJob {
    pub job_id: Option<String>,
    pub stage: ReconstructionStage,
    pub progress_0_1: f32,
    pub stage_label: String,
    pub error: Option<String>,
    pub cancel_requested: bool,
    pub stage_cursor: u32,
    pub started_at_ms: i64,
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

/// 🧵 The reconstructed (or placeholder/imported) mesh, reusing the canonical interchange type, plus
/// optional UVs/texture for a textured export.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelMesh {
    pub mesh: MeshData,
    pub uvs: Option<PackedF32>,
    pub texture_asset_id: Option<String>,
    pub source: MeshSource,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SparseCloud {
    pub positions: PackedF32,
    pub colors: PackedU8,
    pub mean_reprojection_px: f32,
    pub point_count: u32,
}

/// ☁️ Dense point cloud with per-point LAS-style classification (0 unclassified, 2 ground, 6 building).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DenseCloud {
    pub positions: PackedF32,
    pub colors: PackedU8,
    pub confidence: PackedF32,
    pub classification: PackedU8,
    pub point_count: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraPose {
    pub stream_id: String,
    pub frame_index: u32,
    pub timestamp_ms: f64,
    pub camera_id: String,
    pub position: [f32; 3],
    pub rotation_wxyz: [f32; 4],
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CameraTrajectory {
    pub poses: Vec<CameraPose>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrackClass {
    #[default]
    Static,
    Moving,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackKeyframe {
    pub timestamp_ms: f64,
    pub position: [f32; 3],
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MotionTrack {
    pub id: String,
    pub label: String,
    pub class: TrackClass,
    pub keyframes: Vec<TrackKeyframe>,
    pub mean_speed_m_s: f32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct HeightGrid {
    pub origin: [f64; 2],
    pub cell_size_m: f64,
    pub width: u32,
    pub height: u32,
    pub heights: PackedF32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GeoProducts {
    pub dsm: Option<HeightGrid>,
    pub dtm: Option<HeightGrid>,
    pub ortho_asset_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QcStageEntry {
    pub stage: ReconstructionStage,
    pub duration_ms: u32,
    pub item_count: u32,
}

/// ✅ Aggregate quality-control summary produced at the end of a run.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QcReport {
    pub per_stage: Vec<QcStageEntry>,
    pub mean_reprojection_error_px: f32,
    pub median_track_length: f32,
    pub registered_frame_ratio: f32,
    pub dense_coverage_ratio: f32,
    pub calibration_rms_px: f32,
    pub gcp_rmse_m: Option<f32>,
    pub warnings: Vec<String>,
}

/// 📦 Everything a completed (or partially completed) reconstruction run has produced so far.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ReconstructionResults {
    pub sparse: Option<SparseCloud>,
    pub dense: Option<DenseCloud>,
    pub mesh: Option<RemodelMesh>,
    pub trajectory: Option<CameraTrajectory>,
    pub tracks: Vec<MotionTrack>,
    pub geo: Option<GeoProducts>,
    pub qc: Option<QcReport>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionState {
    #[serde(default = "default_selection_mode")]
    pub mode: String,
    #[serde(default)]
    pub ids: Vec<u32>,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self { mode: default_selection_mode(), ids: Vec::new() }
    }
}

fn default_selection_mode() -> String {
    "face".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraState {
    #[serde(default = "default_camera_position")]
    pub position: [f64; 3],
    #[serde(default)]
    pub target: [f64; 3],
    #[serde(default = "default_camera_fov")]
    pub fov: f64,
}

impl Default for CameraState {
    fn default() -> Self {
        Self { position: default_camera_position(), target: [0.0, 0.0, 0.0], fov: default_camera_fov() }
    }
}

fn default_camera_position() -> [f64; 3] {
    [8.0, -8.0, 6.0]
}

fn default_camera_fov() -> f64 {
    45.0
}

/// 🗂️ Top-level remodel project document — only persistent, undoable reconstruction state. Ephemeral
/// viewport state (camera/selection) lives in the plugin runtime and the active utility is host-owned
/// session state (`view_state.active_utility_id`), never in the document.
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
            mesh: Some(RemodelMesh { mesh: semio_framework_core::mesh_from_kind("box"), source: MeshSource::Placeholder, ..RemodelMesh::default() }),
            ..ReconstructionResults::default()
        },
    }
}
//#endregion 🔖Domain

//#region 🔖Ops
/// 🔁 The document mutation vocabulary — one field-granular LWW register setter per independent
/// `RemodelScene` field/sub-field, so disjoint-field edits by concurrent instances converge cleanly.
/// There is no `setDocument` catch-all: reconstruction is field-granular (import a stream, tune one
/// param group, publish a partial result) and each op carries its own inverse from the pre-edit state.
/// `SetAsset` is per-key (not a whole-map replace) so two peers importing different frames converge
/// without clobbering each other's assets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum RemodelOp {
    SetStreams {
        streams: Vec<MediaStream>,
    },
    SetAsset {
        key: String,
        #[serde(default)]
        asset: Option<ImageAsset>,
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
        #[serde(default)]
        mesh: Option<RemodelMesh>,
    },
    SetTrajectory {
        #[serde(default)]
        trajectory: Option<CameraTrajectory>,
    },
    SetTracks {
        tracks: Vec<MotionTrack>,
    },
    SetGeoProducts {
        #[serde(default)]
        geo: Option<GeoProducts>,
    },
    SetQc {
        #[serde(default)]
        qc: Option<QcReport>,
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
        asset: Option<ImageAsset>,
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
        #[serde(default)]
        mesh: Option<RemodelMesh>,
    },
    SetTrajectory {
        #[serde(default)]
        trajectory: Option<CameraTrajectory>,
    },
    SetTracks {
        tracks: Vec<MotionTrack>,
    },
    SetGeoProducts {
        #[serde(default)]
        geo: Option<GeoProducts>,
    },
    SetQc {
        #[serde(default)]
        qc: Option<QcReport>,
    },
}

pub fn apply_remodel_op(scene: &RemodelScene, op: &RemodelOp) -> RemodelScene {
    let mut next = scene.clone();
    match op {
        RemodelOp::SetStreams { streams } => next.streams = streams.clone(),
        RemodelOp::SetAsset { key, asset } => match asset {
            Some(asset) => {
                next.assets.insert(key.clone(), asset.clone());
            }
            None => {
                next.assets.remove(key);
            }
        },
        RemodelOp::SetCalibration { calibration } => next.calibration = calibration.clone(),
        RemodelOp::SetGcps { gcps } => next.gcps = gcps.clone(),
        RemodelOp::SetIngestParams { params } => next.params.ingest = params.clone(),
        RemodelOp::SetFeatureParams { params } => next.params.feature = params.clone(),
        RemodelOp::SetMatchParams { params } => next.params.matching = params.clone(),
        RemodelOp::SetSfmParams { params } => next.params.sfm = params.clone(),
        RemodelOp::SetDenseParams { params } => next.params.dense = params.clone(),
        RemodelOp::SetMeshParams { params } => next.params.mesh = params.clone(),
        RemodelOp::SetMotionParams { params } => next.params.motion = params.clone(),
        RemodelOp::SetGeoParams { params } => next.params.geo = params.clone(),
        RemodelOp::SetJob { job } => next.job = job.clone(),
        RemodelOp::SetSparse { sparse } => next.results.sparse = sparse.clone(),
        RemodelOp::SetDense { dense } => next.results.dense = dense.clone(),
        RemodelOp::SetMeshResult { mesh } => next.results.mesh = mesh.clone(),
        RemodelOp::SetTrajectory { trajectory } => next.results.trajectory = trajectory.clone(),
        RemodelOp::SetTracks { tracks } => next.results.tracks = tracks.clone(),
        RemodelOp::SetGeoProducts { geo } => next.results.geo = geo.clone(),
        RemodelOp::SetQc { qc } => next.results.qc = qc.clone(),
    }
    next
}

impl OperationDiff<RemodelScene> for RemodelDiff {
    fn apply(&self, projection: &RemodelScene) -> RemodelScene {
        let op = match self {
            RemodelDiff::Empty => return projection.clone(),
            RemodelDiff::SetStreams { streams } => RemodelOp::SetStreams { streams: streams.clone() },
            RemodelDiff::SetAsset { key, asset } => RemodelOp::SetAsset { key: key.clone(), asset: asset.clone() },
            RemodelDiff::SetCalibration { calibration } => RemodelOp::SetCalibration { calibration: calibration.clone() },
            RemodelDiff::SetGcps { gcps } => RemodelOp::SetGcps { gcps: gcps.clone() },
            RemodelDiff::SetIngestParams { params } => RemodelOp::SetIngestParams { params: params.clone() },
            RemodelDiff::SetFeatureParams { params } => RemodelOp::SetFeatureParams { params: params.clone() },
            RemodelDiff::SetMatchParams { params } => RemodelOp::SetMatchParams { params: params.clone() },
            RemodelDiff::SetSfmParams { params } => RemodelOp::SetSfmParams { params: params.clone() },
            RemodelDiff::SetDenseParams { params } => RemodelOp::SetDenseParams { params: params.clone() },
            RemodelDiff::SetMeshParams { params } => RemodelOp::SetMeshParams { params: params.clone() },
            RemodelDiff::SetMotionParams { params } => RemodelOp::SetMotionParams { params: params.clone() },
            RemodelDiff::SetGeoParams { params } => RemodelOp::SetGeoParams { params: params.clone() },
            RemodelDiff::SetJob { job } => RemodelOp::SetJob { job: job.clone() },
            RemodelDiff::SetSparse { sparse } => RemodelOp::SetSparse { sparse: sparse.clone() },
            RemodelDiff::SetDense { dense } => RemodelOp::SetDense { dense: dense.clone() },
            RemodelDiff::SetMeshResult { mesh } => RemodelOp::SetMeshResult { mesh: mesh.clone() },
            RemodelDiff::SetTrajectory { trajectory } => RemodelOp::SetTrajectory { trajectory: trajectory.clone() },
            RemodelDiff::SetTracks { tracks } => RemodelOp::SetTracks { tracks: tracks.clone() },
            RemodelDiff::SetGeoProducts { geo } => RemodelOp::SetGeoProducts { geo: geo.clone() },
            RemodelDiff::SetQc { qc } => RemodelOp::SetQc { qc: qc.clone() },
        };
        apply_remodel_op(projection, &op)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, RemodelDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<RemodelScene> for RemodelOp {
    type Diff = RemodelDiff;

    fn diff(&self, _projection: &RemodelScene) -> RemodelDiff {
        match self {
            RemodelOp::SetStreams { streams } => RemodelDiff::SetStreams { streams: streams.clone() },
            RemodelOp::SetAsset { key, asset } => RemodelDiff::SetAsset { key: key.clone(), asset: asset.clone() },
            RemodelOp::SetCalibration { calibration } => RemodelDiff::SetCalibration { calibration: calibration.clone() },
            RemodelOp::SetGcps { gcps } => RemodelDiff::SetGcps { gcps: gcps.clone() },
            RemodelOp::SetIngestParams { params } => RemodelDiff::SetIngestParams { params: params.clone() },
            RemodelOp::SetFeatureParams { params } => RemodelDiff::SetFeatureParams { params: params.clone() },
            RemodelOp::SetMatchParams { params } => RemodelDiff::SetMatchParams { params: params.clone() },
            RemodelOp::SetSfmParams { params } => RemodelDiff::SetSfmParams { params: params.clone() },
            RemodelOp::SetDenseParams { params } => RemodelDiff::SetDenseParams { params: params.clone() },
            RemodelOp::SetMeshParams { params } => RemodelDiff::SetMeshParams { params: params.clone() },
            RemodelOp::SetMotionParams { params } => RemodelDiff::SetMotionParams { params: params.clone() },
            RemodelOp::SetGeoParams { params } => RemodelDiff::SetGeoParams { params: params.clone() },
            RemodelOp::SetJob { job } => RemodelDiff::SetJob { job: job.clone() },
            RemodelOp::SetSparse { sparse } => RemodelDiff::SetSparse { sparse: sparse.clone() },
            RemodelOp::SetDense { dense } => RemodelDiff::SetDense { dense: dense.clone() },
            RemodelOp::SetMeshResult { mesh } => RemodelDiff::SetMeshResult { mesh: mesh.clone() },
            RemodelOp::SetTrajectory { trajectory } => RemodelDiff::SetTrajectory { trajectory: trajectory.clone() },
            RemodelOp::SetTracks { tracks } => RemodelDiff::SetTracks { tracks: tracks.clone() },
            RemodelOp::SetGeoProducts { geo } => RemodelDiff::SetGeoProducts { geo: geo.clone() },
            RemodelOp::SetQc { qc } => RemodelDiff::SetQc { qc: qc.clone() },
        }
    }

    fn backwards(&self, projection: &RemodelScene) -> Vec<Self> {
        vec![match self {
            RemodelOp::SetStreams { .. } => RemodelOp::SetStreams { streams: projection.streams.clone() },
            RemodelOp::SetAsset { key, .. } => RemodelOp::SetAsset { key: key.clone(), asset: projection.assets.get(key).cloned() },
            RemodelOp::SetCalibration { .. } => RemodelOp::SetCalibration { calibration: projection.calibration.clone() },
            RemodelOp::SetGcps { .. } => RemodelOp::SetGcps { gcps: projection.gcps.clone() },
            RemodelOp::SetIngestParams { .. } => RemodelOp::SetIngestParams { params: projection.params.ingest.clone() },
            RemodelOp::SetFeatureParams { .. } => RemodelOp::SetFeatureParams { params: projection.params.feature.clone() },
            RemodelOp::SetMatchParams { .. } => RemodelOp::SetMatchParams { params: projection.params.matching.clone() },
            RemodelOp::SetSfmParams { .. } => RemodelOp::SetSfmParams { params: projection.params.sfm.clone() },
            RemodelOp::SetDenseParams { .. } => RemodelOp::SetDenseParams { params: projection.params.dense.clone() },
            RemodelOp::SetMeshParams { .. } => RemodelOp::SetMeshParams { params: projection.params.mesh.clone() },
            RemodelOp::SetMotionParams { .. } => RemodelOp::SetMotionParams { params: projection.params.motion.clone() },
            RemodelOp::SetGeoParams { .. } => RemodelOp::SetGeoParams { params: projection.params.geo.clone() },
            RemodelOp::SetJob { .. } => RemodelOp::SetJob { job: projection.job.clone() },
            RemodelOp::SetSparse { .. } => RemodelOp::SetSparse { sparse: projection.results.sparse.clone() },
            RemodelOp::SetDense { .. } => RemodelOp::SetDense { dense: projection.results.dense.clone() },
            RemodelOp::SetMeshResult { .. } => RemodelOp::SetMeshResult { mesh: projection.results.mesh.clone() },
            RemodelOp::SetTrajectory { .. } => RemodelOp::SetTrajectory { trajectory: projection.results.trajectory.clone() },
            RemodelOp::SetTracks { .. } => RemodelOp::SetTracks { tracks: projection.results.tracks.clone() },
            RemodelOp::SetGeoProducts { .. } => RemodelOp::SetGeoProducts { geo: projection.results.geo.clone() },
            RemodelOp::SetQc { .. } => RemodelOp::SetQc { qc: projection.results.qc.clone() },
        }]
    }
}
//#endregion 🔖Ops

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_scene_has_placeholder_mesh() {
        let scene = default_remodel_scene();
        let mesh = scene.results.mesh.clone().expect("placeholder result");
        assert_eq!(mesh.source, MeshSource::Placeholder);
        assert!(!mesh.mesh.positions.is_empty());
        assert!(!mesh.mesh.indices.is_empty());
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
        let mut scene = default_remodel_scene();
        scene.streams.push(MediaStream {
            id: "stream-1".into(),
            name: "front".into(),
            kind: MediaKind::Video,
            camera_id: "cam-1".into(),
            sync_offset_ms: 12.5,
            fps_hint: Some(30.0),
            frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: "asset-1".into() }],
        });
        scene.assets.insert("asset-1".into(), ImageAsset { mime: "image/png".into(), data: "abcd".into(), width: 4, height: 4 });
        scene.calibration.cameras.push(CameraCalibration {
            id: "cam-1".into(),
            label: "Front".into(),
            model: CameraModel::BrownConrady,
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
            label: "Corner".into(),
            world: [1.0, 2.0, 3.0],
            enabled: true,
            observations: vec![GcpObservation { stream_id: "stream-1".into(), frame_index: 0, pixel: [10.0, 20.0] }],
        });
        scene.results.sparse = Some(SparseCloud {
            positions: PackedF32::from_vec(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
            colors: PackedU8::from_vec(vec![255, 0, 0, 0, 255, 0]),
            mean_reprojection_px: 0.8,
            point_count: 2,
        });
        scene.results.dense = Some(DenseCloud {
            positions: PackedF32::from_vec(vec![0.0, 0.0, 0.0]),
            colors: PackedU8::from_vec(vec![0, 0, 255]),
            confidence: PackedF32::from_vec(vec![0.9]),
            classification: PackedU8::from_vec(vec![2]),
            point_count: 1,
        });
        scene.results.trajectory = Some(CameraTrajectory {
            poses: vec![
                CameraPose {
                    stream_id: "stream-1".into(),
                    frame_index: 0,
                    timestamp_ms: 0.0,
                    camera_id: "cam-1".into(),
                    position: [0.0, 0.0, 0.0],
                    rotation_wxyz: [1.0, 0.0, 0.0, 0.0],
                },
                CameraPose {
                    stream_id: "stream-1".into(),
                    frame_index: 1,
                    timestamp_ms: 33.3,
                    camera_id: "cam-1".into(),
                    position: [0.1, 0.0, 0.0],
                    rotation_wxyz: [0.999, 0.001, 0.0, 0.0],
                },
            ],
        });
        scene.results.tracks.push(MotionTrack {
            id: "track-1".into(),
            label: "walker".into(),
            class: TrackClass::Moving,
            keyframes: vec![
                TrackKeyframe { timestamp_ms: 0.0, position: [0.0, 0.0, 0.0] },
                TrackKeyframe { timestamp_ms: 100.0, position: [1.0, 0.0, 0.0] },
            ],
            mean_speed_m_s: 1.2,
        });
        scene.results.geo = Some(GeoProducts {
            dsm: Some(HeightGrid {
                origin: [0.0, 0.0],
                cell_size_m: 0.5,
                width: 2,
                height: 2,
                heights: PackedF32::from_vec(vec![1.0, 2.0, 3.0, 4.0]),
            }),
            dtm: None,
            ortho_asset_id: None,
        });
        scene.results.qc = Some(QcReport {
            per_stage: vec![QcStageEntry { stage: ReconstructionStage::Done, duration_ms: 1200, item_count: 4 }],
            mean_reprojection_error_px: 0.5,
            median_track_length: 6.0,
            registered_frame_ratio: 1.0,
            dense_coverage_ratio: 0.95,
            calibration_rms_px: 0.3,
            gcp_rmse_m: Some(0.02),
            warnings: vec!["low overlap on frame 12".into()],
        });

        let json = serde_json::to_string(&scene).expect("serialize");
        let parsed: RemodelScene = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, scene);
    }

    #[test]
    fn packed_f32_roundtrips_as_base64_string() {
        let packed = PackedF32::from_vec(vec![1.5, -2.25, 3.0]);
        let value = serde_json::to_value(&packed).expect("serialize");
        assert!(value.is_string(), "PackedF32 must serialize as a base64 string, got {value:?}");
        let parsed: PackedF32 = serde_json::from_value(value).expect("deserialize");
        assert_eq!(parsed, packed);

        let empty = PackedF32::default();
        let empty_value = serde_json::to_value(&empty).expect("serialize");
        assert_eq!(empty_value, serde_json::Value::String(String::new()));
        let empty_parsed: PackedF32 = serde_json::from_value(empty_value).expect("deserialize");
        assert!(empty_parsed.is_empty());
    }

    #[test]
    fn packed_u8_roundtrips_as_base64_string() {
        let packed = PackedU8::from_vec(vec![0, 128, 255, 64]);
        let value = serde_json::to_value(&packed).expect("serialize");
        assert!(value.is_string(), "PackedU8 must serialize as a base64 string, got {value:?}");
        let parsed: PackedU8 = serde_json::from_value(value).expect("deserialize");
        assert_eq!(parsed, packed);

        let empty = PackedU8::default();
        let empty_value = serde_json::to_value(&empty).expect("serialize");
        assert_eq!(empty_value, serde_json::Value::String(String::new()));
        let empty_parsed: PackedU8 = serde_json::from_value(empty_value).expect("deserialize");
        assert!(empty_parsed.is_empty());
    }

    #[test]
    fn set_asset_op_applies_and_reverts_including_absent_case() {
        let scene = default_remodel_scene();
        assert!(!scene.assets.contains_key("frame-1"));

        let asset = ImageAsset { mime: "image/png".into(), data: "zzz".into(), width: 2, height: 2 };
        let insert_op = RemodelOp::SetAsset { key: "frame-1".into(), asset: Some(asset.clone()) };
        let after_insert = apply_remodel_op(&scene, &insert_op);
        assert_eq!(after_insert.assets.get("frame-1"), Some(&asset));
        assert_eq!(insert_op.diff(&scene).apply(&scene).assets.get("frame-1"), Some(&asset));

        let insert_inverse = insert_op.backwards(&scene);
        assert_eq!(insert_inverse, vec![RemodelOp::SetAsset { key: "frame-1".into(), asset: None }]);
        let reverted = insert_inverse.iter().fold(after_insert.clone(), |current, op| apply_remodel_op(&current, op));
        assert_eq!(reverted, scene);

        let remove_op = RemodelOp::SetAsset { key: "frame-1".into(), asset: None };
        let remove_inverse = remove_op.backwards(&after_insert);
        assert_eq!(remove_inverse, vec![RemodelOp::SetAsset { key: "frame-1".into(), asset: Some(asset.clone()) }]);
        let after_remove = apply_remodel_op(&after_insert, &remove_op);
        assert!(!after_remove.assets.contains_key("frame-1"));
        let restored = remove_inverse.iter().fold(after_remove, |current, op| apply_remodel_op(&current, op));
        assert_eq!(restored.assets.get("frame-1"), Some(&asset));
    }

    #[test]
    fn set_feature_params_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mut params = scene.params.feature.clone();
        params.target_count = 8000;
        let op = RemodelOp::SetFeatureParams { params: params.clone() };
        let next = apply_remodel_op(&scene, &op);
        assert_eq!(next.params.feature.target_count, 8000);
        assert_eq!(op.diff(&scene).apply(&scene).params.feature.target_count, 8000);
        let inverse = op.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOp::SetFeatureParams { params: scene.params.feature.clone() }]);
        let reverted = inverse.iter().fold(next, |current, op| apply_remodel_op(&current, op));
        assert_eq!(reverted.params.feature, scene.params.feature);
    }

    #[test]
    fn set_gcps_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let gcps = vec![GroundControlPoint {
            id: "gcp-1".into(),
            label: "A".into(),
            world: [0.0, 0.0, 0.0],
            enabled: true,
            observations: Vec::new(),
        }];
        let op = RemodelOp::SetGcps { gcps: gcps.clone() };
        let next = apply_remodel_op(&scene, &op);
        assert_eq!(next.gcps, gcps);
        let inverse = op.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOp::SetGcps { gcps: scene.gcps.clone() }]);
        let reverted = inverse.iter().fold(next, |current, op| apply_remodel_op(&current, op));
        assert_eq!(reverted.gcps, scene.gcps);
    }

    #[test]
    fn set_sparse_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let sparse = SparseCloud {
            positions: PackedF32::from_vec(vec![0.0, 0.0, 0.0]),
            colors: PackedU8::from_vec(vec![255, 255, 255]),
            mean_reprojection_px: 0.3,
            point_count: 1,
        };
        let op = RemodelOp::SetSparse { sparse: Some(sparse.clone()) };
        let next = apply_remodel_op(&scene, &op);
        assert_eq!(next.results.sparse, Some(sparse));
        let inverse = op.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOp::SetSparse { sparse: scene.results.sparse.clone() }]);
        let reverted = inverse.iter().fold(next, |current, op| apply_remodel_op(&current, op));
        assert_eq!(reverted.results.sparse, scene.results.sparse);
    }

    #[test]
    fn set_job_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mut job = scene.job.clone();
        job.stage = ReconstructionStage::BundleAdjusting;
        job.progress_0_1 = 0.42;
        let op = RemodelOp::SetJob { job: job.clone() };
        let next = apply_remodel_op(&scene, &op);
        assert_eq!(next.job.stage, ReconstructionStage::BundleAdjusting);
        let inverse = op.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOp::SetJob { job: scene.job.clone() }]);
        let reverted = inverse.iter().fold(next, |current, op| apply_remodel_op(&current, op));
        assert_eq!(reverted.job, scene.job);
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
}
//#endregion 🧪Tests
