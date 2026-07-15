//! 📸 Remodel scene document — photogrammetry project state (source video, reconstruction job, resulting mesh).

use semio_framework_core::MeshData;
use serde::{Deserialize, Serialize};
use vcs::{Operation, OperationDiff};

pub const REMODEL_DOCUMENT_SCHEMA: &str = "remodel.scene";

//#region 🔖Domain
/// 🎬 A single imported source video reference (asset handle + decode metadata, not raw bytes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceVideo {
    pub asset_id: String,
    pub filename: String,
    #[serde(default)]
    pub duration_ms: u64,
    #[serde(default)]
    pub frame_count: u32,
    #[serde(default)]
    pub fps: f32,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
}

/// ⚙️ Parameters controlling frame sampling + reconstruction quality/speed tradeoffs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconstructionParams {
    #[serde(default = "default_frame_sample_stride")]
    pub frame_sample_stride: u32,
    #[serde(default = "default_max_frames")]
    pub max_frames: u32,
    #[serde(default = "default_feature_target_count")]
    pub feature_target_count: u32,
    #[serde(default)]
    pub dense_mvs_resolution: DenseResolution,
    #[serde(default = "default_tsdf_voxel_size_mm")]
    pub tsdf_voxel_size_mm: f32,
}

impl Default for ReconstructionParams {
    fn default() -> Self {
        Self {
            frame_sample_stride: default_frame_sample_stride(),
            max_frames: default_max_frames(),
            feature_target_count: default_feature_target_count(),
            dense_mvs_resolution: DenseResolution::default(),
            tsdf_voxel_size_mm: default_tsdf_voxel_size_mm(),
        }
    }
}

fn default_frame_sample_stride() -> u32 {
    5
}

fn default_max_frames() -> u32 {
    200
}

fn default_feature_target_count() -> u32 {
    4000
}

fn default_tsdf_voxel_size_mm() -> f32 {
    5.0
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DenseResolution {
    Low,
    #[default]
    Medium,
    High,
}

/// 🚦 Mirrors remodel-native's job lifecycle so the document can render progress without polling internals directly.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReconstructionStage {
    #[default]
    Idle,
    ExtractingFrames,
    ExtractingFeatures,
    MatchingFeatures,
    EstimatingPoses,
    BundleAdjusting,
    DenseStereo,
    FusingVolume,
    ExtractingSurface,
    CleaningMesh,
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
#[serde(rename_all = "camelCase")]
pub struct ReconstructionJob {
    #[serde(default)]
    pub job_id: Option<String>,
    #[serde(default)]
    pub native_port: Option<u16>,
    #[serde(default)]
    pub stage: ReconstructionStage,
    #[serde(default)]
    pub progress_0_1: f32,
    #[serde(default)]
    pub stage_label: String,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub camera_poses_preview: Vec<CameraPosePreview>,
    #[serde(default)]
    pub sparse_point_cloud_preview: Vec<f32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MeshSource {
    #[default]
    Placeholder,
    Reconstructed,
    Imported,
}

/// 🧵 The reconstructed (or placeholder/imported) mesh, reusing the canonical interchange type.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemodelMesh {
    pub mesh: MeshData,
    #[serde(default)]
    pub source: MeshSource,
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
        Self {
            mode: default_selection_mode(),
            ids: Vec::new(),
        }
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
        Self {
            position: default_camera_position(),
            target: [0.0, 0.0, 0.0],
            fov: default_camera_fov(),
        }
    }
}

fn default_camera_position() -> [f64; 3] {
    [8.0, -8.0, 6.0]
}

fn default_camera_fov() -> f64 {
    45.0
}

/// 🗂️ Top-level remodel project document — only persistent, undoable reconstruction state. Ephemeral
/// viewport state (camera/selection) lives in the plugin runtime and the active tool is host-owned
/// session state (`view_state.active_utility_id`), never in the document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemodelScene {
    pub schema: String,
    pub id: String,
    #[serde(default)]
    pub source_video: Option<SourceVideo>,
    #[serde(default)]
    pub params: ReconstructionParams,
    #[serde(default)]
    pub job: ReconstructionJob,
    #[serde(default)]
    pub result: Option<RemodelMesh>,
}

/// 🌱 An empty scene seeded with a placeholder box mesh, so the 3D editor/preview always has
/// something to render before a video has been imported/reconstructed.
pub fn default_remodel_scene() -> RemodelScene {
    RemodelScene {
        schema: REMODEL_DOCUMENT_SCHEMA.into(),
        id: "remodel".into(),
        source_video: None,
        params: ReconstructionParams::default(),
        job: ReconstructionJob::default(),
        result: Some(RemodelMesh {
            mesh: semio_framework_core::mesh_from_kind("box"),
            source: MeshSource::Placeholder,
        }),
    }
}
//#endregion 🔖Domain

//#region 🔖Ops
/// 🔁 The document mutation vocabulary — one whole-field LWW register setter per independent
/// `RemodelScene` field, so disjoint-field edits by concurrent instances converge cleanly. There is
/// no `setDocument` catch-all: reconstruction is field-granular (import a video, tune params, publish
/// the resulting mesh) and each field carries its own inverse from the pre-edit state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum RemodelOp {
    SetSourceVideo {
        #[serde(default)]
        video: Option<SourceVideo>,
    },
    SetParams {
        params: ReconstructionParams,
    },
    SetJob {
        job: ReconstructionJob,
    },
    SetResult {
        #[serde(default)]
        result: Option<RemodelMesh>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RemodelDiff {
    #[default]
    Empty,
    SetSourceVideo {
        #[serde(default)]
        video: Option<SourceVideo>,
    },
    SetParams {
        params: ReconstructionParams,
    },
    SetJob {
        job: ReconstructionJob,
    },
    SetResult {
        #[serde(default)]
        result: Option<RemodelMesh>,
    },
}

pub fn apply_remodel_op(scene: &RemodelScene, op: &RemodelOp) -> RemodelScene {
    let mut next = scene.clone();
    match op {
        RemodelOp::SetSourceVideo { video } => next.source_video = video.clone(),
        RemodelOp::SetParams { params } => next.params = params.clone(),
        RemodelOp::SetJob { job } => next.job = job.clone(),
        RemodelOp::SetResult { result } => next.result = result.clone(),
    }
    next
}

impl OperationDiff<RemodelScene> for RemodelDiff {
    fn apply(&self, projection: &RemodelScene) -> RemodelScene {
        let op = match self {
            RemodelDiff::Empty => return projection.clone(),
            RemodelDiff::SetSourceVideo { video } => RemodelOp::SetSourceVideo { video: video.clone() },
            RemodelDiff::SetParams { params } => RemodelOp::SetParams { params: params.clone() },
            RemodelDiff::SetJob { job } => RemodelOp::SetJob { job: job.clone() },
            RemodelDiff::SetResult { result } => RemodelOp::SetResult { result: result.clone() },
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
            RemodelOp::SetSourceVideo { video } => RemodelDiff::SetSourceVideo { video: video.clone() },
            RemodelOp::SetParams { params } => RemodelDiff::SetParams { params: params.clone() },
            RemodelOp::SetJob { job } => RemodelDiff::SetJob { job: job.clone() },
            RemodelOp::SetResult { result } => RemodelDiff::SetResult { result: result.clone() },
        }
    }

    fn backwards(&self, projection: &RemodelScene) -> Vec<Self> {
        vec![match self {
            RemodelOp::SetSourceVideo { .. } => RemodelOp::SetSourceVideo { video: projection.source_video.clone() },
            RemodelOp::SetParams { .. } => RemodelOp::SetParams { params: projection.params.clone() },
            RemodelOp::SetJob { .. } => RemodelOp::SetJob { job: projection.job.clone() },
            RemodelOp::SetResult { .. } => RemodelOp::SetResult { result: projection.result.clone() },
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
        let result = scene.result.expect("placeholder result");
        assert_eq!(result.source, MeshSource::Placeholder);
        assert!(!result.mesh.positions.is_empty());
        assert!(!result.mesh.indices.is_empty());
    }

    #[test]
    fn scene_roundtrips_through_json() {
        let scene = default_remodel_scene();
        let json = serde_json::to_string(&scene).expect("serialize");
        let parsed: RemodelScene = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, scene);
    }

    #[test]
    fn set_params_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mut params = scene.params.clone();
        params.max_frames = 42;
        let op = RemodelOp::SetParams { params: params.clone() };
        let next = apply_remodel_op(&scene, &op);
        assert_eq!(next.params.max_frames, 42);
        assert_eq!(op.diff(&scene).apply(&scene).params.max_frames, 42);
        let inverse = op.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOp::SetParams { params: scene.params.clone() }]);
        let reverted = inverse.iter().fold(next, |current, op| apply_remodel_op(&current, op));
        assert_eq!(reverted.params, scene.params);
    }

    #[test]
    fn set_result_op_clears_and_restores_mesh() {
        let scene = default_remodel_scene();
        let cleared = apply_remodel_op(&scene, &RemodelOp::SetResult { result: None });
        assert!(cleared.result.is_none());
        let inverse = RemodelOp::SetResult { result: None }.backwards(&scene);
        let restored = inverse.iter().fold(cleared, |current, op| apply_remodel_op(&current, op));
        assert_eq!(restored.result, scene.result);
    }
}
//#endregion 🧪Tests
