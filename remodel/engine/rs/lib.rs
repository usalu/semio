//! ⚙️ Reconstruction engine: cooperative staged pipeline turning decoded frames into textured meshes,
//! previews and quality reports. Ties every `remodel_*` domain crate together into the actual
//! video-in → watertight-mesh-out pipeline, self-contained (no dependency on `remodel_document` — the
//! document crate will call into this engine once its own schema is rewritten, not the other way round).

// #region 🔖Input
use std::collections::VecDeque;

/// 🎚️ Frame-ingestion policy shared by [`FrameSource::push_frame`] and [`FrameSource::push_video`]:
/// `stride` keeps every `stride`-th *offered* frame (applied only to the direct [`FrameSource::push_frame`]
/// entry point — [`FrameSource::push_video`] instead relies on the container-level stride already applied
/// by `remodel_video::extract_frames`, so it isn't double-applied), `max_frames` caps the total accepted
/// count (`0` = unbounded), and `min_sharpness`/`rolling_window` drive the relative blur gate: a frame is
/// rejected when its gradient-based sharpness score falls below `min_sharpness * median(last
/// rolling_window accepted scores)`, once at least 3 accepted frames exist to form a baseline.
#[derive(Clone, Debug, PartialEq)]
pub struct IngestParams {
    pub stride: u32,
    pub max_frames: u32,
    pub min_sharpness: f32,
    pub rolling_window: usize,
}

impl Default for IngestParams {
    fn default() -> Self {
        Self { stride: 1, max_frames: 0, min_sharpness: 0.3, rolling_window: 15 }
    }
}

/// 🚦 What [`FrameSource::push_frame`]/[`FrameSource::push_video`] did with one offered frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameAcceptance {
    Accepted,
    RejectedStride,
    RejectedMaxFrames,
    RejectedBlur,
}

/// 🖼️ One accepted input frame: pixels, true media timestamp, originating stream/camera id (single
/// stream, id `0`, until [`FrameSource`] grows multi-stream support), and the sharpness score that let
/// it through the blur gate.
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptedFrame {
    pub index: u32,
    pub image: remodel_image::ImageRgba8,
    pub timestamp_ms: f64,
    pub stream_id: u32,
    pub sharpness: f32,
}

/// ⚠️ Errors from this crate's own fallible entry points — currently just video ingestion, re-exporting
/// `remodel_video::VideoError` so callers get the precise failure (truncated container, unsupported
/// codec, malformed box, …) rather than a lossy wrapper.
#[derive(Clone, Debug, PartialEq)]
pub enum EngineError {
    Video(remodel_video::VideoError),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Video(e) => write!(f, "video ingest error: {e}"),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<remodel_video::VideoError> for EngineError {
    fn from(e: remodel_video::VideoError) -> Self {
        Self::Video(e)
    }
}

/// 📊 Outcome of [`FrameSource::push_video`]: how many samples the container yielded, how many the
/// ingestion policy accepted vs. rejected (split by reason), and provenance from the probe.
#[derive(Clone, Debug, PartialEq)]
pub struct PushVideoReport {
    pub frames_extracted: u32,
    pub frames_accepted: u32,
    pub frames_rejected_blur: u32,
    pub frames_rejected_sampling: u32,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub duration_ms: f64,
}

/// 🧭 Gradient-energy sharpness proxy (mean squared Scharr gradient magnitude): high for crisp edges,
/// collapsing toward zero for a flat/blurred frame — the signal the relative blur gate thresholds
/// against.
fn sharpness_score(image: &remodel_image::ImageRgba8) -> f32 {
    let gray = remodel_image::ImageGray::from_rgba8_luma(image);
    let grad = remodel_image::scharr_gradients(&gray);
    if grad.gx.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = grad.gx.iter().zip(grad.gy.iter()).map(|(&gx, &gy)| gx * gx + gy * gy).sum();
    sum_sq / grad.gx.len() as f32
}

/// 📐 Median of a rolling score window (odd or even length both handled by taking the middle element of
/// the sorted copy — good enough for a soft gating threshold, no need for exact even-length averaging).
fn rolling_median(scores: &VecDeque<f32>) -> f32 {
    let mut v: Vec<f32> = scores.iter().copied().collect();
    v.sort_by(f32::total_cmp);
    v[v.len() / 2]
}

/// 🏷️ Human-facing codec/dimension/duration summary of a [`remodel_video::VideoProbe`], regardless of
/// container family, for [`PushVideoReport`].
fn describe_probe(probe: &remodel_video::VideoProbe) -> (String, u32, u32, f64) {
    match probe {
        remodel_video::VideoProbe::Mp4(info) => (format!("{:?}", info.codec), info.width, info.height, info.duration_ms),
        remodel_video::VideoProbe::Avi(info) => {
            let duration_ms = if info.fps > 0.0 { f64::from(info.frame_count) / info.fps * 1000.0 } else { 0.0 };
            (format!("{:?}", info.codec), info.width, info.height, duration_ms)
        }
    }
}

/// 📥 Accumulates accepted input frames for one reconstruction: the real ingestion point where
/// stride/max-frame sampling and the gradient-score blur-rejection gate actually run, for both
/// direct-frame ([`push_frame`](Self::push_frame)) and video ([`push_video`](Self::push_video)) input.
pub struct FrameSource {
    ingest: IngestParams,
    stream_id: u32,
    offered: u32,
    frames: Vec<AcceptedFrame>,
    rolling_scores: VecDeque<f32>,
}

impl FrameSource {
    /// 🆕 An empty frame source under the given ingestion policy.
    pub fn new(ingest: IngestParams) -> Self {
        Self { ingest, stream_id: 0, offered: 0, frames: Vec::new(), rolling_scores: VecDeque::new() }
    }

    /// 🔍 Every frame accepted so far, in ingestion order.
    pub fn frames(&self) -> &[AcceptedFrame] {
        &self.frames
    }

    /// 🔢 How many frames have been accepted so far.
    pub fn accepted_count(&self) -> usize {
        self.frames.len()
    }

    /// 📥 Offers one directly-provided frame (e.g. an imported image sequence): applies this source's
    /// `stride`/`max_frames` sampling, then the relative blur gate.
    pub fn push_frame(&mut self, index: u32, image: remodel_image::ImageRgba8, timestamp_ms: f64) -> FrameAcceptance {
        self.accept(index, image, timestamp_ms, true)
    }

    /// 🎞️ Probes `bytes` as a video container, lazily decodes sampled frames via
    /// `remodel_video::extract_frames` (container-level stride/max-frames/downscale already applied per
    /// `opts`), and offers each decoded frame through the same blur gate as [`push_frame`](Self::push_frame)
    /// (without re-applying this source's own stride counter, since the container already sampled).
    pub fn push_video(&mut self, bytes: &[u8], opts: &remodel_video::VideoIngestOptions) -> Result<PushVideoReport, EngineError> {
        let probe = remodel_video::probe(bytes)?;
        let (codec, width, height, duration_ms) = describe_probe(&probe);
        let iter = remodel_video::extract_frames(bytes, opts)?;
        let mut report = PushVideoReport { frames_extracted: 0, frames_accepted: 0, frames_rejected_blur: 0, frames_rejected_sampling: 0, codec, width, height, duration_ms };
        for extracted in iter {
            let extracted = extracted?;
            report.frames_extracted += 1;
            match self.accept(extracted.index, extracted.image, extracted.timestamp_ms, false) {
                FrameAcceptance::Accepted => report.frames_accepted += 1,
                FrameAcceptance::RejectedBlur => report.frames_rejected_blur += 1,
                FrameAcceptance::RejectedStride | FrameAcceptance::RejectedMaxFrames => report.frames_rejected_sampling += 1,
            }
        }
        Ok(report)
    }

    /// 🚦 Shared gate: optional stride counting, then `max_frames`, then the relative blur threshold
    /// against the rolling median of recently accepted scores.
    fn accept(&mut self, index: u32, image: remodel_image::ImageRgba8, timestamp_ms: f64, apply_stride: bool) -> FrameAcceptance {
        let offered = self.offered;
        self.offered += 1;
        if apply_stride {
            let stride = self.ingest.stride.max(1);
            if offered % stride != 0 {
                return FrameAcceptance::RejectedStride;
            }
        }
        if self.ingest.max_frames != 0 && self.frames.len() as u32 >= self.ingest.max_frames {
            return FrameAcceptance::RejectedMaxFrames;
        }
        let score = sharpness_score(&image);
        if self.rolling_scores.len() >= 3 {
            let median = rolling_median(&self.rolling_scores);
            if score < self.ingest.min_sharpness * median {
                return FrameAcceptance::RejectedBlur;
            }
        }
        if self.rolling_scores.len() >= self.ingest.rolling_window.max(1) {
            self.rolling_scores.pop_front();
        }
        self.rolling_scores.push_back(score);
        self.frames.push(AcceptedFrame { index, image, timestamp_ms, stream_id: self.stream_id, sharpness: score });
        FrameAcceptance::Accepted
    }
}
// #endregion 🔖Input

// #region 🔖Params
/// 🎛️ Every knob the staged pipeline needs, bundled with sane defaults: ingestion policy, the assumed
/// pinhole focal-length ratio (no calibration stage exists yet, so intrinsics are derived from frame
/// dimensions), feature/matching/SfM/dense/mesh sub-configs (reusing each domain crate's own config type
/// directly rather than re-declaring their fields), and toggles for the optional motion/geo analyses.
#[derive(Clone, Debug, PartialEq)]
pub struct EngineParams {
    pub ingest: IngestParams,
    pub assumed_focal_ratio: f64,
    pub target_feature_count: usize,
    pub match_ratio: f32,
    pub match_mutual: bool,
    pub sequential_window: usize,
    pub sfm: remodel_sfm::SfmConfig,
    pub dense: remodel_dense::PatchMatchConfig,
    pub dense_source_views: usize,
    pub tsdf_voxel_size: f64,
    pub tsdf_truncation: f64,
    pub mesh: remodel_mesh::MeshParams,
    pub texture_enabled: bool,
    pub motion_enabled: bool,
    pub geo_enabled: bool,
    pub geo_cell_size: f64,
}

impl Default for EngineParams {
    fn default() -> Self {
        Self {
            ingest: IngestParams::default(),
            assumed_focal_ratio: 1.0,
            target_feature_count: 500,
            match_ratio: 0.8,
            match_mutual: true,
            sequential_window: 2,
            sfm: remodel_sfm::SfmConfig::default(),
            dense: remodel_dense::PatchMatchConfig::default(),
            dense_source_views: 4,
            tsdf_voxel_size: 0.05,
            tsdf_truncation: 0.15,
            mesh: remodel_mesh::MeshParams::default(),
            texture_enabled: true,
            motion_enabled: false,
            geo_enabled: false,
            geo_cell_size: 0.5,
        }
    }
}

/// 📷 Default pinhole intrinsics assumed for uncalibrated input: `fx = fy = focal_ratio *
/// max(width, height)`, principal point at the image center, no distortion — a documented
/// simplification standing in for the calibration stage the base plan scopes separately.
fn default_intrinsics(width: u32, height: u32, focal_ratio: f64) -> remodel_camera::Intrinsics {
    let f = focal_ratio * f64::from(width.max(height));
    remodel_camera::Intrinsics { fx: f, fy: f, cx: f64::from(width) / 2.0, cy: f64::from(height) / 2.0, skew: 0.0, distortion: remodel_camera::Distortion::None }
}
// #endregion 🔖Params

// #region 🔖Pipeline
/// 🚦 Named stage of the cooperative reconstruction state machine — mirrors the stage *names* the
/// not-yet-rewritten `remodel_document::ReconstructionStage` plans to expose, without depending on that
/// crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineStage {
    Idle,
    ExtractingFeatures,
    MatchingFeatures,
    EstimatingPoses,
    BundleAdjusting,
    DenseStereo,
    FusingVolume,
    ExtractingSurface,
    CleaningMesh,
    Texturing,
    Done,
    Failed,
}

/// 📡 What [`ReconstructionEngine::advance`] returns: still working (with the current stage and a coarse
/// `[0, 1]` progress estimate), finished, or failed with a human-readable reason.
#[derive(Clone, Debug, PartialEq)]
pub enum EngineStatus {
    Working { stage: EngineStage, progress: f32 },
    Done,
    Failed(String),
}

/// 🔢 Ordinal of a non-terminal stage in the fixed 9-stage pipeline, for [`ReconstructionEngine::progress`].
fn stage_ordinal(stage: EngineStage) -> usize {
    match stage {
        EngineStage::Idle => 0,
        EngineStage::ExtractingFeatures => 1,
        EngineStage::MatchingFeatures => 2,
        EngineStage::EstimatingPoses => 3,
        EngineStage::BundleAdjusting => 4,
        EngineStage::DenseStereo => 5,
        EngineStage::FusingVolume => 6,
        EngineStage::ExtractingSurface => 7,
        EngineStage::CleaningMesh => 8,
        EngineStage::Texturing => 9,
        EngineStage::Done | EngineStage::Failed => 10,
    }
}

/// 🗺️ Maps `remodel_mesh::mesh_pipeline_step`'s internal stage name to the engine-level stage it falls
/// under, so driving the mesh pipeline (Amendment: engine delegates meshing directly to
/// `remodel_mesh::mesh_pipeline_step`) still reports through the coarser [`EngineStage`] vocabulary.
fn mesh_stage_to_engine_stage(name: &str) -> EngineStage {
    match name {
        "marching_cubes" => EngineStage::ExtractingSurface,
        "unwrap" | "texture_bake" | "interchange" => EngineStage::Texturing,
        _ => EngineStage::CleaningMesh,
    }
}

/// 🏘️ Up to `k` other camera slot indices nearest to `ci` (by registration-order distance, which tracks
/// frame order for an [`remodel_sfm::IncrementalSfm`] reconstruction), sorted ascending for determinism —
/// the source-view selection for [`remodel_dense::patchmatch_mvs`]/TSDF fusion.
fn neighbor_camera_indices(ci: usize, n: usize, k: usize) -> Vec<usize> {
    let mut idxs: Vec<usize> = (0..n).filter(|&c| c != ci).collect();
    idxs.sort_by_key(|&c| (c as i64 - ci as i64).abs());
    idxs.truncate(k);
    idxs.sort_unstable();
    idxs
}

/// 📦 Voxel-index bounds covering `points` with a 20% margin plus a 2-voxel padding shell, for
/// `remodel_mesh::MeshPipeline::new`'s `bounds_min`/`bounds_max`. Falls back to a small centered cube
/// when there are no points yet (degenerate input).
fn compute_voxel_bounds(points: &[[f64; 3]], voxel_size: f64) -> ([i32; 3], [i32; 3]) {
    const MAX_CELLS_PER_AXIS: i32 = 120;
    if points.is_empty() || voxel_size <= 0.0 {
        return ([-4, -4, -4], [4, 4, 4]);
    }
    // 🎯 5th/95th-percentile bounds per axis rather than raw min/max: a single badly-triangulated
    // outlier point (a real risk from a noisy incremental reconstruction) must not be able to blow the
    // TSDF/marching-cubes grid up to an unbounded size.
    let mut lo = [0.0; 3];
    let mut hi = [0.0; 3];
    for k in 0..3 {
        let mut vs: Vec<f64> = points.iter().map(|p| p[k]).collect();
        vs.sort_by(f64::total_cmp);
        let p05 = vs[((vs.len() as f64 - 1.0) * 0.05).round() as usize];
        let p95 = vs[((vs.len() as f64 - 1.0) * 0.95).round() as usize];
        lo[k] = p05;
        hi[k] = p95;
    }
    let mut bounds_min = [0i32; 3];
    let mut bounds_max = [0i32; 3];
    for k in 0..3 {
        let span = (hi[k] - lo[k]).max(voxel_size);
        let pad = span * 0.2;
        let raw_min = ((lo[k] - pad) / voxel_size).floor() as i32 - 2;
        let raw_max = ((hi[k] + pad) / voxel_size).ceil() as i32 + 2;
        let center = (raw_min + raw_max) / 2;
        let half_span = ((raw_max - raw_min) / 2).min(MAX_CELLS_PER_AXIS / 2).max(4);
        bounds_min[k] = center - half_span;
        bounds_max[k] = center + half_span;
    }
    (bounds_min, bounds_max)
}

/// 🧵 `(camera_slot_index, point_index, observed_pixel)` triples for `remodel_geo::build_quality_report`,
/// derived from a finished [`remodel_sfm::Reconstruction`]'s tracks and each frame's detected keypoints.
fn build_observations(recon: &remodel_sfm::Reconstruction, tracks: Option<&remodel_sfm::FeatureTracks>, keypoints_per_frame: &[Vec<remodel_feature::Keypoint>]) -> Vec<(usize, usize, [f64; 2])> {
    let Some(tracks) = tracks else { return Vec::new() };
    let camera_index_of: std::collections::BTreeMap<usize, usize> = recon.cameras.iter().enumerate().map(|(ci, &(f, _))| (f, ci)).collect();
    let mut out = Vec::new();
    for (point_index, &track_id) in recon.point_track_ids.iter().enumerate() {
        let Some(track) = tracks.tracks.get(track_id) else { continue };
        for &(frame, kp) in track {
            let Some(&ci) = camera_index_of.get(&frame) else { continue };
            let Some(kp_list) = keypoints_per_frame.get(frame) else { continue };
            let Some(k) = kp_list.get(kp as usize) else { continue };
            out.push((ci, point_index, [f64::from(k.x), f64::from(k.y)]));
        }
    }
    out
}

/// ⚙️ The cooperative staged pipeline: [`advance`](Self::advance) performs one bounded slice of work per
/// call (one image featurized, one pair matched, one PnP registration, one bundle-adjustment solve, one
/// depth map, one TSDF integration batch, one `remodel_mesh` pipeline stage) and is genuinely resumable —
/// calling it repeatedly with a small budget or once with a huge budget reaches the same [`EngineStatus::Done`]
/// result, only the call count differs.
pub struct ReconstructionEngine {
    params: EngineParams,
    frame_source: FrameSource,
    stage: EngineStage,
    frames: Vec<AcceptedFrame>,
    cursor: usize,

    keypoints_per_frame: Vec<Vec<remodel_feature::Keypoint>>,
    descriptors_per_frame: Vec<Vec<remodel_feature::Descriptor256>>,

    match_pairs: Vec<(usize, usize)>,
    pair_cursor: usize,
    pairwise_matches: Vec<(usize, usize, Vec<remodel_feature::Match>)>,
    tracks: Option<remodel_sfm::FeatureTracks>,

    sfm: Option<remodel_sfm::IncrementalSfm>,
    pose_cursor: usize,
    ba_substep: usize,
    reconstruction: Option<remodel_sfm::Reconstruction>,
    observations: Vec<(usize, usize, [f64; 2])>,

    stage_cursor: usize,
    depth_maps: Vec<Option<remodel_dense::DepthMap>>,
    tsdf: Option<remodel_dense::TsdfVolume>,
    fusion_finalized: bool,
    dense_cloud: Option<remodel_dense::PointCloud>,

    mesh_pipeline: Option<remodel_mesh::MeshPipeline>,
    mesh_data: Option<semio_framework_core::MeshData>,
    watertight_report: Option<remodel_mesh::WatertightReport>,

    failure: Option<String>,
}

impl ReconstructionEngine {
    /// 🆕 A fresh engine in [`EngineStage::Idle`], with an internal empty [`FrameSource`] under
    /// `params.ingest`. Push frames via [`push_frame`](Self::push_frame)/[`push_video`](Self::push_video)
    /// before the first [`advance`](Self::advance) call.
    pub fn new(params: &EngineParams) -> Self {
        Self {
            params: params.clone(),
            frame_source: FrameSource::new(params.ingest.clone()),
            stage: EngineStage::Idle,
            frames: Vec::new(),
            cursor: 0,
            keypoints_per_frame: Vec::new(),
            descriptors_per_frame: Vec::new(),
            match_pairs: Vec::new(),
            pair_cursor: 0,
            pairwise_matches: Vec::new(),
            tracks: None,
            sfm: None,
            pose_cursor: 0,
            ba_substep: 0,
            reconstruction: None,
            observations: Vec::new(),
            stage_cursor: 0,
            depth_maps: Vec::new(),
            tsdf: None,
            fusion_finalized: false,
            dense_cloud: None,
            mesh_pipeline: None,
            mesh_data: None,
            watertight_report: None,
            failure: None,
        }
    }

    /// 📥 Delegates to the internal [`FrameSource::push_frame`].
    pub fn push_frame(&mut self, index: u32, image: remodel_image::ImageRgba8, timestamp_ms: f64) -> FrameAcceptance {
        self.frame_source.push_frame(index, image, timestamp_ms)
    }

    /// 🎞️ Delegates to the internal [`FrameSource::push_video`].
    pub fn push_video(&mut self, bytes: &[u8], opts: &remodel_video::VideoIngestOptions) -> Result<PushVideoReport, EngineError> {
        self.frame_source.push_video(bytes, opts)
    }

    /// 🔍 The internal frame source, for inspecting accepted frames/counts without driving the pipeline.
    pub fn frame_source(&self) -> &FrameSource {
        &self.frame_source
    }

    /// 🚦 Current stage.
    pub fn stage(&self) -> EngineStage {
        self.stage
    }

    /// 📸 Snapshots [`FrameSource::frames`] into the engine's own working set; fails if fewer than 2
    /// frames were accepted (the minimum an [`remodel_sfm::IncrementalSfm`] two-view init needs).
    fn start(&mut self) -> Result<(), String> {
        self.frames = self.frame_source.frames().to_vec();
        if self.frames.len() < 2 {
            return Err(format!("reconstruction requires at least 2 accepted frames, got {}", self.frames.len()));
        }
        self.cursor = 0;
        Ok(())
    }

    /// 🎯 One frame's pyramid/detect/describe; returns whether more frames remain in this stage.
    fn step_extracting_features(&mut self) -> bool {
        let i = self.cursor;
        if i >= self.frames.len() {
            return false;
        }
        let gray = remodel_image::ImageGray::from_rgba8_luma(&self.frames[i].image);
        let pyramid = remodel_image::build_pyramid(&gray, 3);
        let keypoints = remodel_feature::detect_orb_keypoints(&pyramid, self.params.target_feature_count);
        let descriptors = remodel_feature::describe_orb(&pyramid, &keypoints);
        self.keypoints_per_frame.push(keypoints);
        self.descriptors_per_frame.push(descriptors);
        self.cursor += 1;
        self.cursor < self.frames.len()
    }

    /// 🕸️ Sequential-window pair list `(i, j)` for `j in (i, i + sequential_window]`, built once when
    /// entering [`EngineStage::MatchingFeatures`].
    /// 🕸️ Sequential-window pairs `(i, j)` for `j in (i, i + sequential_window]`, plus an explicit
    /// `(0, f)`/`(1, f)` "anchor" pair for every later frame `f`: `IncrementalSfm::init_pair` triangulates
    /// only tracks directly spanning both anchor frames, so without a direct anchor↔f pair a later frame's
    /// `register_next` PnP can only find 2D-3D correspondences through incidental hub-chained tracks —
    /// sparse enough on real matches to starve most frames of the 6 correspondences PnP needs. Explicit
    /// anchor pairs make every registerable frame's correspondence-to-the-seed-pair direct instead of
    /// coincidental.
    fn build_match_pairs(&mut self) {
        let n = self.frames.len();
        let window = self.params.sequential_window.max(1);
        let mut pairs: std::collections::BTreeSet<(usize, usize)> = std::collections::BTreeSet::new();
        for i in 0..n {
            let hi = (i + window).min(n.saturating_sub(1));
            for j in (i + 1)..=hi {
                pairs.insert((i, j));
            }
        }
        for f in 2..n {
            pairs.insert((0, f));
            pairs.insert((1, f));
        }
        self.match_pairs = pairs.into_iter().collect();
        self.pair_cursor = 0;
    }

    /// 🤝 Matches one pair's descriptors; returns whether more pairs remain.
    fn step_matching_features(&mut self) -> bool {
        let i = self.pair_cursor;
        if i >= self.match_pairs.len() {
            return false;
        }
        let (a, b) = self.match_pairs[i];
        let matches = remodel_feature::match_brute(&self.descriptors_per_frame[a], &self.descriptors_per_frame[b], self.params.match_ratio, self.params.match_mutual);
        self.pairwise_matches.push((a, b, matches));
        self.pair_cursor += 1;
        self.pair_cursor < self.match_pairs.len()
    }

    /// 🏗️ Either seeds [`remodel_sfm::IncrementalSfm`] via `init_pair(0, 1, ..)` (first call), or
    /// registers+triangulates the next frame (subsequent calls) — one frame's worth of pose estimation
    /// per call, mirroring `run_all`'s best-effort policy (a frame that fails to register is skipped, not
    /// fatal) except for the initial pair, whose failure genuinely aborts the reconstruction.
    fn step_estimating_poses(&mut self) -> Result<bool, String> {
        if self.sfm.is_none() {
            let intr = default_intrinsics(self.frames[0].image.width, self.frames[0].image.height, self.params.assumed_focal_ratio);
            let tracks = self.tracks.as_ref().expect("tracks built before EstimatingPoses").clone();
            let mut sfm = remodel_sfm::IncrementalSfm::new(intr, tracks, self.keypoints_per_frame.clone(), self.params.sfm.clone());
            let pair01 = self.pairwise_matches.iter().find(|&&(a, b, _)| a == 0 && b == 1).map(|(_, _, m)| m.clone()).ok_or_else(|| "no matches between frame 0 and 1".to_string())?;
            println!("[DEBUG] frame0 kp={} frame1 kp={} pair01 matches={}", self.keypoints_per_frame[0].len(), self.keypoints_per_frame[1].len(), pair01.len());
            sfm.init_pair(0, 1, &pair01).map_err(|e| e.to_string())?;
            let after_init = sfm.reconstruction();
            println!("[DEBUG] after init_pair: cameras={} points={}", after_init.cameras.len(), after_init.points.len());
            self.sfm = Some(sfm);
            self.pose_cursor = 2;
            return Ok(self.pose_cursor < self.frames.len());
        }
        let n = self.frames.len();
        if self.pose_cursor >= n {
            return Ok(false);
        }
        let frame = self.pose_cursor;
        if let Some(sfm) = self.sfm.as_mut() {
            let r = sfm.register_next(frame);
            println!("[DEBUG] register_next({frame}) = {r:?}");
            sfm.triangulate_new(frame);
        }
        self.pose_cursor += 1;
        Ok(self.pose_cursor < n)
    }

    /// 🎯 One bundle-adjustment substep from the fixed cycle `local_ba -> prune_outliers ->
    /// retriangulate -> global_ba` (the finest granularity `remodel_sfm::IncrementalSfm` exposes — each
    /// of these already runs its own internal solve/pass to completion in one call, same "whole stage per
    /// call" chunking granularity `remodel_mesh::mesh_pipeline_step` uses). Returns whether more substeps
    /// remain.
    fn step_bundle_adjusting(&mut self) -> bool {
        let Some(sfm) = self.sfm.as_mut() else { return false };
        let n = self.frames.len();
        match self.ba_substep {
            0 => sfm.local_ba(n),
            1 => sfm.prune_outliers(),
            2 => sfm.retriangulate(),
            3 => sfm.global_ba(),
            _ => return false,
        }
        self.ba_substep += 1;
        self.ba_substep <= 3
    }

    /// 📦 Snapshots the finished `Reconstruction` and its observation list, and resets the dense-stereo
    /// cursor/scratch.
    fn finalize_reconstruction(&mut self) {
        if let Some(sfm) = &self.sfm {
            let recon = sfm.reconstruction();
            self.observations = build_observations(&recon, self.tracks.as_ref(), &self.keypoints_per_frame);
            let n_cameras = recon.cameras.len();
            println!("[DEBUG] finalize_reconstruction: cameras={} points={}", recon.cameras.len(), recon.points.len());
            self.reconstruction = Some(recon);
            self.depth_maps = vec![None; n_cameras];
        }
        self.stage_cursor = 0;
    }

    /// 🌫️ One registered camera's `remodel_dense::patchmatch_mvs` depth map against its nearest
    /// registered neighbors; returns whether more cameras remain.
    fn step_dense_stereo(&mut self) -> bool {
        let n_cameras = match &self.reconstruction {
            Some(r) => r.cameras.len(),
            None => return false,
        };
        let ci = self.stage_cursor;
        if ci >= n_cameras {
            return false;
        }
        let (frame_a, pose_a, intrinsics, neighbor_frames) = {
            let recon = self.reconstruction.as_ref().expect("checked above");
            let (frame_a, pose_a) = recon.cameras[ci];
            let intrinsics = recon.intrinsics;
            let neighbor_frames: Vec<(usize, remodel_camera::CameraPose)> = neighbor_camera_indices(ci, n_cameras, self.params.dense_source_views).into_iter().map(|cj| recon.cameras[cj]).collect();
            (frame_a, pose_a, intrinsics, neighbor_frames)
        };
        let ref_gray = remodel_image::ImageGray::from_rgba8_luma(&self.frames[frame_a].image);
        let mut src_views = Vec::with_capacity(neighbor_frames.len());
        for (frame_b, pose_b) in neighbor_frames {
            let gray_b = remodel_image::ImageGray::from_rgba8_luma(&self.frames[frame_b].image);
            src_views.push((gray_b, pose_b, intrinsics));
        }
        let dm = remodel_dense::patchmatch_mvs(&ref_gray, &(pose_a, intrinsics), &src_views, &self.params.dense);
        self.depth_maps[ci] = Some(dm);
        self.stage_cursor += 1;
        self.stage_cursor < n_cameras
    }

    /// 🧊 One camera's depth map integrated into the TSDF (or, once every camera is integrated, the
    /// final `fuse_depth_maps` aggregate for the QC/geo point cloud); returns whether more work remains
    /// in this stage.
    fn step_fusing_volume(&mut self) -> bool {
        let n_cameras = match &self.reconstruction {
            Some(r) => r.cameras.len(),
            None => return false,
        };
        // 🧊 Always ensures a (possibly still-empty) TSDF exists once this stage starts, even when
        // `n_cameras == 0` (a degenerate but legitimate outcome — every registered camera got pruned by
        // bundle adjustment): without this, `begin_meshing` used to find `self.tsdf` still `None` and
        // silently skip building a `MeshPipeline`, later surfacing as the confusing, wiring-looking
        // `"mesh pipeline not initialized"` failure instead of an honest empty-reconstruction outcome.
        if self.tsdf.is_none() {
            self.tsdf = Some(remodel_dense::TsdfVolume::new(self.params.tsdf_voxel_size, self.params.tsdf_truncation));
        }
        if self.stage_cursor < n_cameras {
            let ci = self.stage_cursor;
            let (pose, intrinsics) = {
                let recon = self.reconstruction.as_ref().expect("checked above");
                let (_, pose) = recon.cameras[ci];
                (pose, recon.intrinsics)
            };
            let tsdf = self.tsdf.as_mut().expect("just ensured");
            if let Some(dm) = &self.depth_maps[ci] {
                tsdf.integrate(dm, &(pose, intrinsics), true);
            }
            self.stage_cursor += 1;
            return true;
        }
        if !self.fusion_finalized {
            let recon = self.reconstruction.as_ref().expect("checked above");
            let views: Vec<(remodel_camera::CameraPose, remodel_camera::Intrinsics)> = recon.cameras.iter().map(|&(_, p)| (p, recon.intrinsics)).collect();
            let depth_maps: Vec<remodel_dense::DepthMap> = self.depth_maps.iter().map(|d| d.clone().unwrap_or_else(|| remodel_dense::DepthMap::new(1, 1))).collect();
            self.dense_cloud = Some(remodel_dense::fuse_depth_maps(&views, &depth_maps, &remodel_dense::FusionConfig::default()));
            self.fusion_finalized = true;
            return false;
        }
        false
    }

    /// 🏗️ Builds the `remodel_mesh::MeshPipeline` from the accumulated TSDF once dense fusion is done:
    /// voxel bounds from the union of sparse+dense points, optional per-camera `TextureView`s.
    fn begin_meshing(&mut self) {
        let Some(tsdf) = self.tsdf.take() else { return };
        let mut all_points: Vec<[f64; 3]> = Vec::new();
        if let Some(r) = &self.reconstruction {
            all_points.extend(r.points.iter().copied());
        }
        if let Some(cloud) = &self.dense_cloud {
            all_points.extend(cloud.positions.iter().copied());
        }
        let (bounds_min, bounds_max) = compute_voxel_bounds(&all_points, self.params.tsdf_voxel_size);
        let mut pipeline = remodel_mesh::MeshPipeline::new(&tsdf, 0.0, bounds_min, bounds_max, self.params.mesh.clone());
        if self.params.texture_enabled {
            if let Some(r) = &self.reconstruction {
                let views: Vec<remodel_mesh::TextureView> = r.cameras.iter().map(|&(frame, pose)| remodel_mesh::TextureView { pose, intrinsics: r.intrinsics, image: self.frames[frame].image.clone() }).collect();
                pipeline = pipeline.with_views(views);
            }
        }
        self.mesh_pipeline = Some(pipeline);
    }

    /// 🕸️ Drives `remodel_mesh::mesh_pipeline_step` one whole internal stage per call, mapping its
    /// stage name back onto [`EngineStage`].
    fn step_meshing(&mut self) -> MeshStepOutcome {
        let Some(pipeline) = self.mesh_pipeline.as_mut() else {
            return MeshStepOutcome::Failed("mesh pipeline not initialized".to_string());
        };
        match remodel_mesh::mesh_pipeline_step(pipeline, 1) {
            remodel_mesh::MeshPipelineStatus::Working { stage, .. } => MeshStepOutcome::Working(mesh_stage_to_engine_stage(stage)),
            remodel_mesh::MeshPipelineStatus::Done => MeshStepOutcome::Done,
            remodel_mesh::MeshPipelineStatus::Failed(msg) => MeshStepOutcome::Failed(msg),
        }
    }

    /// 📈 Coarse `[0, 1]` progress from the current stage's ordinal alone (no intra-stage fraction — the
    /// per-stage cursors have wildly different, not-necessarily-comparable totals).
    fn progress(&self) -> f32 {
        stage_ordinal(self.stage) as f32 / 10.0
    }

    /// ⚙️ Advances the pipeline through at most `step_budget` bounded units of work (never fewer than 1),
    /// crossing stage boundaries within the same call whenever a stage finishes with budget still left —
    /// the same style `remodel_mesh::mesh_pipeline_step` uses internally. Genuinely resumable: calling
    /// this repeatedly with a small budget or once with `usize::MAX` reaches the same terminal
    /// [`EngineStatus`], only the call count differs.
    pub fn advance(&mut self, step_budget: usize) -> EngineStatus {
        for _ in 0..step_budget.max(1) {
            match self.stage {
                EngineStage::Idle => {
                    if let Err(msg) = self.start() {
                        self.stage = EngineStage::Failed;
                        self.failure = Some(msg.clone());
                        return EngineStatus::Failed(msg);
                    }
                    self.stage = EngineStage::ExtractingFeatures;
                }
                EngineStage::ExtractingFeatures => {
                    if !self.step_extracting_features() {
                        self.build_match_pairs();
                        self.stage = EngineStage::MatchingFeatures;
                    }
                }
                EngineStage::MatchingFeatures => {
                    if !self.step_matching_features() {
                        let tracks = remodel_sfm::build_tracks(self.frames.len(), &self.pairwise_matches);
                        let len_ge3 = tracks.tracks.iter().filter(|t| t.len() >= 3).count();
                        println!("[DEBUG] build_tracks: total_tracks={} len>=2={} len>=3={}", tracks.tracks.len(), tracks.tracks.iter().filter(|t| t.len() >= 2).count(), len_ge3);
                        self.tracks = Some(tracks);
                        self.stage = EngineStage::EstimatingPoses;
                    }
                }
                EngineStage::EstimatingPoses => match self.step_estimating_poses() {
                    Ok(true) => {}
                    Ok(false) => {
                        self.ba_substep = 0;
                        self.stage = EngineStage::BundleAdjusting;
                    }
                    Err(msg) => {
                        self.stage = EngineStage::Failed;
                        self.failure = Some(msg.clone());
                        return EngineStatus::Failed(msg);
                    }
                },
                EngineStage::BundleAdjusting => {
                    if !self.step_bundle_adjusting() {
                        self.finalize_reconstruction();
                        self.stage = EngineStage::DenseStereo;
                    }
                }
                EngineStage::DenseStereo => {
                    if !self.step_dense_stereo() {
                        self.stage_cursor = 0;
                        self.stage = EngineStage::FusingVolume;
                    }
                }
                EngineStage::FusingVolume => {
                    if !self.step_fusing_volume() {
                        self.begin_meshing();
                        self.stage = EngineStage::ExtractingSurface;
                    }
                }
                EngineStage::ExtractingSurface | EngineStage::CleaningMesh | EngineStage::Texturing => match self.step_meshing() {
                    MeshStepOutcome::Working(stage) => self.stage = stage,
                    MeshStepOutcome::Done => {
                        self.mesh_data = self.mesh_pipeline.as_ref().and_then(remodel_mesh::MeshPipeline::result).cloned();
                        self.watertight_report = self.mesh_pipeline.as_ref().and_then(remodel_mesh::MeshPipeline::report).cloned();
                        self.stage = EngineStage::Done;
                        return EngineStatus::Done;
                    }
                    MeshStepOutcome::Failed(msg) => {
                        self.stage = EngineStage::Failed;
                        self.failure = Some(msg.clone());
                        return EngineStatus::Failed(msg);
                    }
                },
                EngineStage::Done => return EngineStatus::Done,
                EngineStage::Failed => return EngineStatus::Failed(self.failure.clone().unwrap_or_default()),
            }
        }
        match self.stage {
            EngineStage::Done => EngineStatus::Done,
            EngineStage::Failed => EngineStatus::Failed(self.failure.clone().unwrap_or_default()),
            stage => EngineStatus::Working { stage, progress: self.progress() },
        }
    }
}

/// 🕸️ Internal result of one `mesh_pipeline_step` call, translated to engine vocabulary.
enum MeshStepOutcome {
    Working(EngineStage),
    Done,
    Failed(String),
}
// #endregion 🔖Pipeline

// #region 🔖Preview
/// 🔭 A lightweight incremental-progress snapshot for downstream UI rendering, callable mid-reconstruction
/// (not just once [`EngineStatus::Done`]): every currently-known camera pose, and every currently
/// triangulated point packed as a flat `[x0, y0, z0, x1, y1, z1, ..]` `f32` buffer.
#[derive(Clone, Debug, PartialEq)]
pub struct ScenePreview {
    pub camera_poses: Vec<remodel_camera::CameraPose>,
    pub packed_points: Vec<f32>,
}

impl ReconstructionEngine {
    /// 🔭 Snapshots whichever reconstruction state is currently available: the finalized
    /// `Reconstruction` once bundle adjustment has run, else the in-progress `IncrementalSfm`'s own
    /// snapshot, else empty (before `EstimatingPoses` has produced anything).
    pub fn sparse_preview(&self) -> ScenePreview {
        if let Some(r) = &self.reconstruction {
            return pack_reconstruction(r);
        }
        if let Some(sfm) = &self.sfm {
            return pack_reconstruction(&sfm.reconstruction());
        }
        ScenePreview { camera_poses: Vec::new(), packed_points: Vec::new() }
    }
}

/// 📦 Packs a `Reconstruction`'s camera poses and points into a [`ScenePreview`].
fn pack_reconstruction(r: &remodel_sfm::Reconstruction) -> ScenePreview {
    ScenePreview { camera_poses: r.cameras.iter().map(|&(_, p)| p).collect(), packed_points: r.points.iter().flat_map(|p| p.iter().map(|&c| c as f32)).collect() }
}
// #endregion 🔖Preview

// #region 🔖Products
/// 🌍 Optional georeferencing-adjacent rasters, populated only when `EngineParams::geo_enabled` and
/// there's a fused dense point cloud to derive them from.
#[derive(Clone, Debug, PartialEq)]
pub struct GeoProducts {
    pub dsm: remodel_geo::Raster,
    pub dtm: remodel_geo::Raster,
}

/// 📦 World-space `(x, y)`/`(z)` bounding box of a point cloud's positions, or `None` when empty.
fn point_cloud_bbox(cloud: &remodel_dense::PointCloud) -> Option<([f64; 3], [f64; 3])> {
    let mut it = cloud.positions.iter();
    let first = *it.next()?;
    let mut lo = first;
    let mut hi = first;
    for &p in it {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    Some((lo, hi))
}

impl ReconstructionEngine {
    /// 🕸️ The finished, watertight-guaranteed `MeshData`, once [`EngineStatus::Done`] — consumes it (a
    /// second call returns `None`), mirroring `remodel_mesh::MeshPipeline::result`'s own take-once shape
    /// at the product-extraction boundary.
    pub fn take_mesh(&mut self) -> Option<semio_framework_core::MeshData> {
        self.mesh_data.take()
    }

    /// 📊 The whole-reconstruction quality report: reprojection accuracy, track health, camera/point
    /// uncertainty, and (once the mesh pipeline has run) the watertight report — available as soon as
    /// bundle adjustment has produced a `Reconstruction`, not only at `Done`, since it's cheap to
    /// recompute from already-finished data.
    pub fn take_quality(&mut self) -> Option<remodel_geo::QualityReport> {
        let recon = self.reconstruction.as_ref()?;
        Some(remodel_geo::build_quality_report(recon, &self.observations, None, None, None, self.watertight_report.clone()))
    }

    /// 🌍 DSM/DTM rasters derived from the fused dense point cloud, when `EngineParams::geo_enabled`.
    /// `None` when geo products weren't requested, or there's no (or an empty) dense cloud yet.
    pub fn take_geo_products(&mut self) -> Option<GeoProducts> {
        if !self.params.geo_enabled {
            return None;
        }
        let cloud = self.dense_cloud.as_ref()?;
        if cloud.is_empty() {
            return None;
        }
        let (lo, hi) = point_cloud_bbox(cloud)?;
        let cell = self.params.geo_cell_size.max(1e-6);
        let width = (((hi[0] - lo[0]) / cell).ceil() as u32 + 1).clamp(1, 512);
        let height = (((hi[1] - lo[1]) / cell).ceil() as u32 + 1).clamp(1, 512);
        let origin = [lo[0], lo[1]];
        let dsm = remodel_geo::build_dsm(cloud, cell, origin, width, height);
        let dtm = remodel_geo::build_dtm(cloud, cell, origin, width, height);
        Some(GeoProducts { dsm, dtm })
    }
}
// #endregion 🔖Products

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖TestFixtures
    /// 🎨 Flat mid-gray `w x h` frame with zero gradient energy — a stand-in for a heavily blurred/
    /// defocused capture, deliberately below any sensible relative-sharpness threshold.
    fn flat_frame(w: u32, h: u32) -> remodel_image::ImageRgba8 {
        let mut img = remodel_image::ImageRgba8::new(w, h);
        for px in img.data.chunks_mut(4) {
            px[0] = 128;
            px[1] = 128;
            px[2] = 128;
            px[3] = 255;
        }
        img
    }

    /// 🏁 High-contrast `cell`-pixel checkerboard — strong Scharr gradient energy everywhere, a stand-in
    /// for a crisp, well-focused frame.
    fn checker_frame(w: u32, h: u32, cell: u32) -> remodel_image::ImageRgba8 {
        let mut img = remodel_image::ImageRgba8::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let on = ((x / cell.max(1)) + (y / cell.max(1))) % 2 == 0;
                let v = if on { 235u8 } else { 20u8 };
                let idx = ((y * w + x) * 4) as usize;
                img.data[idx] = v;
                img.data[idx + 1] = v;
                img.data[idx + 2] = v;
                img.data[idx + 3] = 255;
            }
        }
        img
    }
    // #endregion 🔖TestFixtures

    // #region 🔖InputTests
    #[test]
    fn push_frame_blur_gate_rejects_planted_blurred_frame() {
        let mut source = FrameSource::new(IngestParams::default());
        let mut outcomes = Vec::new();
        for i in 0..10u32 {
            let img = if i == 5 { flat_frame(32, 32) } else { checker_frame(32, 32, 4) };
            outcomes.push(source.push_frame(i, img, f64::from(i) * 33.3));
        }
        assert_eq!(outcomes[5], FrameAcceptance::RejectedBlur, "planted flat frame at index 5 must be rejected as blur, got {:?}", outcomes[5]);
        for (i, outcome) in outcomes.iter().enumerate() {
            if i != 5 {
                assert_eq!(*outcome, FrameAcceptance::Accepted, "sharp frame at index {i} should be accepted, got {outcome:?}");
            }
        }
        assert_eq!(source.accepted_count(), 9);
    }

    #[test]
    fn push_frame_stride_and_max_frames_sample() {
        let mut source = FrameSource::new(IngestParams { stride: 2, max_frames: 3, min_sharpness: 0.0, rolling_window: 15 });
        let mut accepted = 0;
        for i in 0..10u32 {
            if source.push_frame(i, checker_frame(16, 16, 4), f64::from(i)) == FrameAcceptance::Accepted {
                accepted += 1;
            }
        }
        assert_eq!(accepted, 3, "stride 2 + max_frames 3 should accept exactly 3 of 10 offered frames");
        assert_eq!(source.accepted_count(), 3);
    }

    #[test]
    fn push_video_blur_gate_reports_counts() {
        let frames: Vec<Vec<u8>> = (0..9u32)
            .map(|i| {
                let img = if i == 4 { flat_frame(24, 24) } else { checker_frame(24, 24, 3) };
                remodel_image::encode_jpeg(&img, 90)
            })
            .collect();
        let bytes = remodel_video::write_mp4_mjpeg(&frames, 10.0);
        let mut source = FrameSource::new(IngestParams::default());
        let opts = remodel_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        let report = source.push_video(&bytes, &opts).expect("mjpeg mp4 push_video should succeed");
        assert_eq!(report.frames_extracted, 9);
        assert_eq!(report.frames_accepted, 8);
        assert_eq!(report.frames_rejected_blur, 1);
        assert_eq!(report.frames_rejected_sampling, 0);
        assert_eq!(source.accepted_count(), 8);
    }
    // #endregion 🔖InputTests

    // #region 🔖SyntheticScene
    fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
    }
    fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }
    fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
        [a[0] * s, a[1] * s, a[2] * s]
    }
    fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
    }
    fn norm3(a: [f64; 3]) -> f64 {
        (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
    }
    fn normalize3(a: [f64; 3]) -> [f64; 3] {
        let n = norm3(a);
        if n < 1e-15 {
            [0.0, 0.0, 0.0]
        } else {
            scale3(a, 1.0 / n)
        }
    }

    /// 🎥 Look-at camera pose (world→camera), mirroring `remodel_mesh`'s own test helper of the same
    /// shape: right-handed, `y`-up unless looking near-vertically.
    fn look_at_pose(eye: [f64; 3], target: [f64; 3]) -> remodel_camera::CameraPose {
        let forward = normalize3(sub3(target, eye));
        let world_up = if forward[1].abs() > 0.95 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let right = normalize3(cross3(forward, world_up));
        let up = cross3(right, forward);
        let rotation = mathematical_algebra::Mat3d::from_axes(right, up, forward).transpose();
        let translation = scale3(rotation.mul_vec3(eye), -1.0);
        remodel_camera::CameraPose(mathematical_lie::Se3 { r: mathematical_lie::So3(rotation), t: translation })
    }

    /// 📦 Ray/axis-aligned-box slab intersection: nearest `t >= 0` hit point plus which axis (0=x, 1=y,
    /// 2=z) the hit face is perpendicular to, or `None` for a miss.
    fn ray_box_intersect(origin: [f64; 3], dir: [f64; 3], half: f64) -> Option<([f64; 3], usize)> {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;
        let mut hit_axis = 0usize;
        for axis in 0..3 {
            let o = origin[axis];
            let d = dir[axis];
            if d.abs() < 1e-12 {
                if o < -half || o > half {
                    return None;
                }
            } else {
                let (mut t1, mut t2) = ((-half - o) / d, (half - o) / d);
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }
                if t1 > tmin {
                    tmin = t1;
                    hit_axis = axis;
                }
                if t2 < tmax {
                    tmax = t2;
                }
                if tmin > tmax {
                    return None;
                }
            }
        }
        if tmin < 0.0 {
            return None;
        }
        Some((add3(origin, scale3(dir, tmin)), hit_axis))
    }

    /// 🔵 One isolated, fixed-appearance marker painted on a cube face at local `(u, v)` — the same
    /// design `remodel_sfm::render_textured_scene` uses (small high-contrast patches at fixed world/point
    /// locations rather than a periodic texture), just anchored to a solid cube face instead of floating
    /// in space, so features stay isolated and locally unique (good for matching) while the surface stays
    /// solid (needed for dense stereo/TSDF fusion).
    #[derive(Clone, Copy)]
    struct FaceMarker {
        u: f64,
        v: f64,
        radius: f64,
        color: [u8; 3],
    }

    /// 🎲 `count` random markers per cube face (6 faces, axis 0/1/2 × sign), from a fixed seed so every
    /// render call across every synthesized frame sees the identical marker layout.
    fn generate_face_markers(seed: u64, count: usize, half: f64) -> [Vec<FaceMarker>; 6] {
        let mut rng = mathematical_random::Rng::from_seed(seed);
        std::array::from_fn(|_face| {
            (0..count)
                .map(|_| FaceMarker {
                    u: (rng.next_f64() * 2.0 - 1.0) * half * 0.85,
                    v: (rng.next_f64() * 2.0 - 1.0) * half * 0.85,
                    radius: half * 0.09,
                    color: [rng.next_range(60, 255) as u8, rng.next_range(60, 255) as u8, rng.next_range(60, 255) as u8],
                })
                .collect()
        })
    }

    const FACE_BASE_COLORS: [[u8; 3]; 6] = [[150, 60, 60], [60, 60, 150], [60, 150, 60], [150, 150, 60], [150, 60, 150], [60, 150, 150]];

    fn face_index(axis: usize, positive: bool) -> usize {
        axis * 2 + usize::from(!positive)
    }

    /// 🎨 A flat per-face base color, with any nearby [`FaceMarker`] drawn on top — isolated, high-contrast,
    /// locally-unique corner-rich features at fixed world positions.
    fn cube_face_color(p: [f64; 3], axis: usize, markers: &[Vec<FaceMarker>; 6]) -> [u8; 3] {
        let positive = p[axis] > 0.0;
        let (u, v) = match axis {
            0 => (p[1], p[2]),
            1 => (p[0], p[2]),
            _ => (p[0], p[1]),
        };
        let idx = face_index(axis, positive);
        for m in &markers[idx] {
            if ((u - m.u).powi(2) + (v - m.v).powi(2)).sqrt() <= m.radius {
                return m.color;
            }
        }
        FACE_BASE_COLORS[idx]
    }

    /// 🖼️ Renders one view of a `half`-extent axis-aligned textured cube (analytic ray/box intersection,
    /// no rasterizer needed) from `pose`/`intr` — the local minimal synthetic multi-view scene backing
    /// both the chunking-invariance test and the `mod long` end-to-end contract test.
    fn render_cube_frame(width: u32, height: u32, intr: &remodel_camera::Intrinsics, pose: &remodel_camera::CameraPose, half: f64, markers: &[Vec<FaceMarker>; 6]) -> remodel_image::ImageRgba8 {
        let mut img = remodel_image::ImageRgba8::new(width, height);
        let to_world = pose.0.inverse();
        let origin_world = to_world.act([0.0, 0.0, 0.0]);
        for y in 0..height {
            for x in 0..width {
                let ray_cam = intr.unproject_ray([f64::from(x) + 0.5, f64::from(y) + 0.5]);
                let ray_world = normalize3(sub3(to_world.act(ray_cam), origin_world));
                let color = match ray_box_intersect(origin_world, ray_world, half) {
                    Some((p, axis)) => cube_face_color(p, axis, markers),
                    None => [35u8, 35, 40],
                };
                let idx = ((y * width + x) * 4) as usize;
                img.data[idx] = color[0];
                img.data[idx + 1] = color[1];
                img.data[idx + 2] = color[2];
                img.data[idx + 3] = 255;
            }
        }
        img
    }

    /// 🌐 `n` frames orbiting a `half`-extent cube at `radius` and fixed image size, plus the cube's own
    /// known world-space bounding box (for downstream bbox-tolerance assertions).
    /// 📷 Focal-length-to-frame-size ratio the synthetic renderer's camera uses — shared with
    /// [`tiny_engine_params`]'s `assumed_focal_ratio` so the engine's calibration-free default intrinsics
    /// heuristic matches the camera that actually rendered the frames; a mismatch here silently biases
    /// every recovered depth/scale (a real bug this file hit once: default `assumed_focal_ratio` of `1.0`
    /// against a `0.85` rendering camera produced a reconstruction ~3x too large).
    const CUBE_CAMERA_FOCAL_RATIO: f64 = 0.85;

    fn orbiting_cube_frames(n: usize, size: u32, half: f64, radius: f64) -> (Vec<remodel_image::ImageRgba8>, [f64; 3], [f64; 3]) {
        let f = CUBE_CAMERA_FOCAL_RATIO * f64::from(size);
        let intr = remodel_camera::Intrinsics { fx: f, fy: f, cx: f64::from(size) / 2.0, cy: f64::from(size) / 2.0, skew: 0.0, distortion: remodel_camera::Distortion::None };
        let markers = generate_face_markers(0x5EED_CAFE, 14, half);
        let mut frames = Vec::with_capacity(n);
        for i in 0..n {
            let angle = std::f64::consts::TAU * (i as f64) / (n as f64);
            let eye = [radius * angle.cos(), radius * 0.25, radius * angle.sin()];
            let pose = look_at_pose(eye, [0.0, 0.0, 0.0]);
            frames.push(render_cube_frame(size, size, &intr, &pose, half, &markers));
        }
        (frames, [-half, -half, -half], [half, half, half])
    }
    // #endregion 🔖SyntheticScene

    // #region 🔖ChunkingInvariance
    fn tiny_engine_params(half: f64, radius: f64) -> EngineParams {
        let mut params = EngineParams::default();
        params.ingest.min_sharpness = 0.0;
        params.assumed_focal_ratio = CUBE_CAMERA_FOCAL_RATIO;
        params.target_feature_count = 500;
        params.match_ratio = 0.85;
        params.match_mutual = true;
        params.sequential_window = 3;
        params.sfm.min_track_length = 2;
        params.dense_source_views = 3;
        params.dense.depth_min = ((radius - half * 2.0).max(0.05)) as f32;
        params.dense.depth_max = (radius + half * 2.0) as f32;
        params.dense.window_radius = 2;
        params.dense.iterations = 2;
        params.tsdf_voxel_size = half / 10.0;
        params.tsdf_truncation = half / 3.0;
        params.texture_enabled = false;
        params
    }

    fn run_to_done(engine: &mut ReconstructionEngine, budget: usize) -> semio_framework_core::MeshData {
        loop {
            match engine.advance(budget) {
                EngineStatus::Working { .. } => {}
                EngineStatus::Done => break,
                EngineStatus::Failed(msg) => panic!("engine unexpectedly failed: {msg}"),
            }
        }
        engine.take_mesh().expect("Done status must yield a mesh")
    }

    #[test]
    fn chunking_does_not_change_the_final_mesh() {
        // 🎯 Reuses the exact per-frame camera geometry proven to register in `mod long`'s full 48-frame
        // orbit (128px, half=1.0, radius=3.2 -> a 7.5deg step between consecutive frames) but keeps only
        // the first 12 of those 48 frames: identical small-baseline epipolar geometry, far cheaper to run
        // than the whole orbit — a wide 45deg-step 8-frame orbit was tried first and left every pair too
        // wide-baseline for `remodel_sfm`'s two-view init to triangulate any points at all.
        let (frames, bbox_lo, bbox_hi) = orbiting_cube_frames(48, 64, 1.0, 3.2);
        let mut params = tiny_engine_params(1.0, 3.2);
        params.sequential_window = 6;
        params.match_ratio = 0.82;
        params.target_feature_count = 500;

        let mut small = ReconstructionEngine::new(&params);
        for (i, f) in frames.iter().enumerate() {
            small.push_frame(i as u32, f.clone(), f64::from(i as u32) * 100.0);
        }
        let mesh_small_budget = run_to_done(&mut small, 1);

        let mut big = ReconstructionEngine::new(&params);
        for (i, f) in frames.iter().enumerate() {
            big.push_frame(i as u32, f.clone(), f64::from(i as u32) * 100.0);
        }
        let mesh_huge_budget = run_to_done(&mut big, usize::MAX);

        assert_eq!(mesh_small_budget.indices.len(), mesh_huge_budget.indices.len(), "chunking must not change triangle count");
        assert_eq!(mesh_small_budget.positions.len(), mesh_huge_budget.positions.len(), "chunking must not change vertex count");
        assert_eq!(mesh_small_budget.positions, mesh_huge_budget.positions, "chunking must not change vertex positions");
        assert!(!mesh_small_budget.positions.is_empty(), "expected a non-empty mesh");
        let _ = (bbox_lo, bbox_hi);
    }
    // #endregion 🔖ChunkingInvariance

    // #region 🔖ParamsAndPreviewTests
    #[test]
    fn sparse_preview_is_empty_before_any_advance() {
        let engine = ReconstructionEngine::new(&EngineParams::default());
        let preview = engine.sparse_preview();
        assert!(preview.camera_poses.is_empty());
        assert!(preview.packed_points.is_empty());
    }

    #[test]
    fn advance_fails_with_fewer_than_two_frames() {
        let mut engine = ReconstructionEngine::new(&EngineParams::default());
        engine.push_frame(0, checker_frame(16, 16, 4), 0.0);
        match engine.advance(10) {
            EngineStatus::Failed(msg) => assert!(msg.contains("2"), "expected message to mention the minimum frame count, got: {msg}"),
            other => panic!("expected Failed, got {other:?}"),
        }
    }
    // #endregion 🔖ParamsAndPreviewTests

    // #region 🔖LongContract
    mod long {
        use super::*;

        /// 🎬 THE end-to-end contract: synthesize an orbiting-textured-cube video (rasterize → JPEG →
        /// MP4/MJPEG mux), `push_video` the raw bytes, drive `advance` to `Done` with zero host and zero
        /// file fixtures, then assert the extracted mesh is non-empty, its bounding box roughly matches
        /// the cube's known extent, and — the literal "watertight" half of the contract —
        /// `remodel_mesh::validate_watertight` on the mesh's own positions/indices reports
        /// `is_watertight == true`.
        #[test]
        fn video_in_yields_watertight_mesh_out() {
            const N_FRAMES: usize = 48;
            const SIZE: u32 = 128;
            const HALF: f64 = 1.0;
            const RADIUS: f64 = 3.2;

            let (frames, bbox_lo, bbox_hi) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
            let jpegs: Vec<Vec<u8>> = frames.iter().map(|f| remodel_image::encode_jpeg(f, 92)).collect();
            let mp4_bytes = remodel_video::write_mp4_mjpeg(&jpegs, 12.0);
            println!("[long] muxed {} mjpeg frames into {} mp4 bytes", jpegs.len(), mp4_bytes.len());

            let mut params = tiny_engine_params(HALF, RADIUS);
            params.sequential_window = 6;
            params.match_ratio = 0.82;
            params.target_feature_count = 500;
            params.texture_enabled = true;
            let mut engine = ReconstructionEngine::new(&params);

            let opts = remodel_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
            let report = engine.push_video(&mp4_bytes, &opts).expect("push_video on a synthesized mjpeg mp4 must succeed");
            println!("[long] push_video report: {report:?}");
            assert_eq!(report.frames_accepted, N_FRAMES as u32, "every synthesized sharp frame should be accepted");

            let mut calls = 0usize;
            let status = loop {
                calls += 1;
                match engine.advance(4) {
                    EngineStatus::Working { stage, progress } => {
                        if calls % 20 == 0 {
                            println!("[long] call {calls}: stage={stage:?} progress={progress:.2}");
                        }
                        if calls > 10_000 {
                            panic!("engine did not reach a terminal status within 10000 advance() calls");
                        }
                    }
                    terminal => break terminal,
                }
            };
            println!("[long] reached terminal status after {calls} advance() calls: {status:?}");

            let mesh = match status {
                EngineStatus::Done => engine.take_mesh().expect("Done status must yield a mesh"),
                EngineStatus::Failed(msg) => panic!("engine failed instead of reaching Done: {msg}"),
                EngineStatus::Working { .. } => unreachable!("loop only exits on a terminal status"),
            };

            assert!(!mesh.indices.is_empty(), "expected a non-empty mesh, got 0 triangles");
            assert!(!mesh.positions.is_empty(), "expected a non-empty mesh, got 0 vertices");
            println!("[long] mesh vertices={} triangles={}", mesh.positions.len() / 3, mesh.indices.len() / 3);

            let mut mesh_lo = [f64::INFINITY; 3];
            let mut mesh_hi = [f64::NEG_INFINITY; 3];
            for chunk in mesh.positions.chunks(3) {
                for k in 0..3 {
                    mesh_lo[k] = mesh_lo[k].min(f64::from(chunk[k]));
                    mesh_hi[k] = mesh_hi[k].max(f64::from(chunk[k]));
                }
            }
            println!("[long] mesh bbox lo={mesh_lo:?} hi={mesh_hi:?}, cube bbox lo={bbox_lo:?} hi={bbox_hi:?}");
            let cube_diag = ((bbox_hi[0] - bbox_lo[0]).powi(2) + (bbox_hi[1] - bbox_lo[1]).powi(2) + (bbox_hi[2] - bbox_lo[2]).powi(2)).sqrt();
            let mesh_diag = ((mesh_hi[0] - mesh_lo[0]).powi(2) + (mesh_hi[1] - mesh_lo[1]).powi(2) + (mesh_hi[2] - mesh_lo[2]).powi(2)).sqrt();
            let tolerance = 0.20;
            assert!(
                (mesh_diag - cube_diag).abs() <= tolerance * cube_diag,
                "mesh bbox diagonal {mesh_diag} should be within {}% of the cube's known bbox diagonal {cube_diag}",
                tolerance * 100.0
            );

            let tri_mesh = remodel_mesh::TriMesh {
                positions: mesh.positions.chunks(3).map(|c| [f64::from(c[0]), f64::from(c[1]), f64::from(c[2])]).collect(),
                triangles: mesh.indices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect(),
            };
            let watertight_report = remodel_mesh::validate_watertight(&tri_mesh, false);
            println!("[long] watertight report: {watertight_report:?}");
            assert!(watertight_report.is_watertight, "the video-in -> watertight-mesh-out contract requires is_watertight == true, got report: {watertight_report:?}");
        }
    }
    // #endregion 🔖LongContract
}
// #endregion 🔖Tests
