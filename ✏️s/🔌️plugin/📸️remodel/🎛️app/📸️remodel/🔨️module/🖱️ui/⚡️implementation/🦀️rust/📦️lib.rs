//! 🏺️ Remodel app — DocumentApp impl, render, manifest (constitutional: ui). Wires `remodel`'s
//! field-granular scene schema to `remodel_engine`'s cooperative staged reconstruction pipeline via
//! the framework's `DispatchAction` re-dispatch loop: `runReconstruction` ingests already-imported
//! media into a fresh `ReconstructionEngine` and schedules the first `advanceReconstruction` tick;
//! each tick advances a bounded budget of work and reschedules itself until `Done`/`Failed`/cancelled,
//! coalescing the whole run into one undo step via a shared `coalesce_key` (every tick — including the
//! terminal one — uses `ActionEmit::amend` with the same key; see `🔖️StagedReconstruction` for why
//! `ActionEmit::commit` on the final tick would NOT satisfy the one-undo-step contract).

use base64::Engine as _;
use remodel::{
    default_remodel_scene, CameraCalibration, CameraPosePreview, CameraTrajectory, DenseParams, DenseResolution, FeatureDetector, FeatureParams, FrameRef, GcpObservation, GeoParams, GeoProducts, GroundControlPoint, ImageAsset, IngestParams,
    MatchParams, MatcherKind, MediaKind, MediaStream, MeshParams as DocumentMeshParams, MeshSource, MotionParams, PackedF32, ReconstructionJob, ReconstructionStage, RemodelMesh, RemodelScene,
    RobustLossKind, SfmParams, SparseCloud, VideoSource, REMODEL_DOCUMENT_SCHEMA,
};
use remodel_op::RemodelOperation;
use semio_framework_plugin::{
    app_labels, build_canvas_2d_scene, build_table_scene, build_world_3d_scene, create_default_layout, create_named_layout, is_de_locale, localized_label_map, mesh_from_kind, resolve_labels, ui_import_drop_zone, ui_stack_vertical, ui_text,
    world3d_camera_json, world3d_scene, world3d_selection_json, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionEmit, ActionKind, App, AppLabelsOverlay, AppLabelsOverlayExt, Canvas2dScene, DocumentApp, DocumentView,
    HostEffect, MeshData, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup, ArtifactKindSpec, SurfaceKind, TableScene, UiNode, UtilityCategory, UtilityDefinition, ViewState, WorldSunConfig,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, SET_ACTIVE_UTILITY_ACTION_ID,
};
use serde_json::{json, Value};
use std::collections::VecDeque;
use store::DocumentDsl;

//#region 🔖️Constants
const REMODEL_PLAY_APP_ID: &str = "remodel-play";
const REMODEL_PLAY_SURFACE_MAIN: &str = "remodel.play";
const REMODEL_PLAY_SURFACE_FRAMES: &str = "remodel.play.frames";
const REMODEL_PLAY_SURFACE_REPORT: &str = "remodel.play.report";
const REMODEL_PLAY_BODY_MAIN: &str = "remodel.play.main";
const REMODEL_PLAY_BODY_FRAMES: &str = "remodel.play.frames";
const REMODEL_PLAY_BODY_REPORT: &str = "remodel.play.report";
const REMODEL_PLAY_BODY_MEDIA: &str = "remodel.play.media";
const REMODEL_PLAY_BODY_PIPELINE: &str = "remodel.play.pipeline";
const REMODEL_PLAY_BODY_RESULTS: &str = "remodel.play.results";
const REMODEL_PLAY_BODY_PARAMETERS: &str = "remodel.play.parameters";
const REMODEL_PLAY_BODY_CALIBRATION: &str = "remodel.play.calibration";
const REMODEL_PLAY_BODY_TRACKS: &str = "remodel.play.tracks";
const REMODEL_PLAY_BODY_QC: &str = "remodel.play.qc";
const REMODEL_PLAY_WINDOW_MAIN: &str = "remodel-main";
const REMODEL_PLAY_WINDOW_FRAMES: &str = "remodel-frames";
const REMODEL_PLAY_WINDOW_REPORT: &str = "remodel-report";
const REMODEL_MESH_ID: &str = "remodel-result";
const REMODEL_PANEL_MEDIA_ID: &str = "remodel.media";
const REMODEL_PANEL_PIPELINE_ID: &str = "remodel.pipeline";
const REMODEL_PANEL_RESULTS_ID: &str = "remodel.results";
const REMODEL_PANEL_PARAMETERS_ID: &str = "remodel.parameters";
const REMODEL_PANEL_CALIBRATION_ID: &str = "remodel.calibration";
const REMODEL_PANEL_TRACKS_ID: &str = "remodel.tracks";
const REMODEL_PANEL_QC_ID: &str = "remodel.qc";
/// 🧰️ The utility active when the host has not yet set `view_state.active_utility_id` (first UtilityRef default).
const REMODEL_DEFAULT_UTILITY: &str = "select";
/// ⚙️ Bounded units of engine work performed per `advanceReconstruction` host tick — small enough that
/// one tick never blocks the host for long, large enough that a tiny test/demo scene finishes in a
/// handful of re-dispatches rather than hundreds.
const RECONSTRUCTION_STEP_BUDGET: usize = 8;
/// 📥️ The drop zone's accepted extensions: still-image formats plus every container `remodel_video`
/// can probe (decode is attempted in-process; an undecodable codec still records provenance).
const REMODEL_MEDIA_ACCEPT: &str = "image/png,image/jpeg,video/mp4,video/quicktime,video/webm,video/x-msvideo,.png,.jpg,.jpeg,.mp4,.mov,.webm,.avi";
const REMODEL_VIDEO_ACCEPT: &str = "video/mp4,video/quicktime,video/webm,video/x-msvideo,.mp4,.mov,.webm,.avi";
//#endregion 🔖️Constants

//#region 🔖️Runtime
/// 🎥️ Ephemeral viewport orbit camera — never persisted, mirrors `world3d_camera_json`'s shape.
#[derive(Clone, Debug, PartialEq)]
struct RemodelWorldCamera {
    position: [f64; 3],
    target: [f64; 3],
    fov: f64,
}

impl Default for RemodelWorldCamera {
    fn default() -> Self {
        Self { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }
    }
}

/// 🖱️ Ephemeral face/vertex/object selection — never persisted.
#[derive(Clone, Debug, Default, PartialEq)]
struct RemodelSelection {
    mode: String,
    ids: Vec<String>,
}

/// 👁️ Which `remodel-main` point-cloud/mesh layers are currently visible — toggled via the window's
/// `remodel.layers` measures group, never persisted.
#[derive(Clone, Debug, PartialEq)]
struct RemodelLayerVisibility {
    mesh: bool,
    dense: bool,
    sparse: bool,
    cameras: bool,
    gcps: bool,
}

impl Default for RemodelLayerVisibility {
    fn default() -> Self {
        Self { mesh: true, dense: true, sparse: true, cameras: true, gcps: true }
    }
}

/// 🎞️ Which frame `remodel-frames` currently shows.
#[derive(Clone, Debug, Default, PartialEq)]
struct RemodelFrameCursor {
    stream_id: Option<String>,
    frame_index: u32,
}

/// 📥️ Rolling blur-gate scratch for one in-progress `importVideo`/`RequestMediaFrames` batch — mirrors
/// `remodel_engine::FrameSource`'s own relative-sharpness gate (not reusable directly: that gate lives
/// inside a whole `FrameSource`, this one only needs the rolling-median scratch itself).
#[derive(Clone, Debug, Default, PartialEq)]
struct VideoImportScratch {
    rolling_scores: VecDeque<f32>,
}

const BLUR_GATE_ROLLING_WINDOW: usize = 15;
const BLUR_GATE_MIN_SAMPLES: usize = 3;

/// 🧭️ Gradient-energy sharpness proxy — a local mirror of `remodel_engine`'s private `sharpness_score`
/// (not exported by that crate), reused here so import-time frame gating uses the identical signal.
fn local_sharpness_score(image: &remodel_image::ImageRgba8) -> f32 {
    let gray = remodel_image::ImageGray::from_rgba8_luma(image);
    let grad = remodel_image::scharr_gradients(&gray);
    if grad.gx.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = grad.gx.iter().zip(grad.gy.iter()).map(|(&gx, &gy)| gx * gx + gy * gy).sum();
    sum_sq / grad.gx.len() as f32
}

fn local_rolling_median(scores: &VecDeque<f32>) -> f32 {
    let mut v: Vec<f32> = scores.iter().copied().collect();
    v.sort_by(f32::total_cmp);
    v[v.len() / 2]
}

/// 🚦️ Whether the sample should be rejected by the relative blur gate, given `scratch`'s rolling window
/// and `min_sharpness` (a fraction of the rolling median); also records the sample if accepted.
fn blur_gate_reject(scratch: &mut VideoImportScratch, score: f32, min_sharpness: f32) -> bool {
    if scratch.rolling_scores.len() >= BLUR_GATE_MIN_SAMPLES {
        let median = local_rolling_median(&scratch.rolling_scores);
        if score < min_sharpness * median {
            return true;
        }
    }
    if scratch.rolling_scores.len() >= BLUR_GATE_ROLLING_WINDOW {
        scratch.rolling_scores.pop_front();
    }
    scratch.rolling_scores.push_back(score);
    false
}

/// 🎛️ Ephemeral viewport/session state (orbit camera, selection, layer toggles, frame cursor, report
/// table selection, the live `ReconstructionEngine` for an in-progress run, and small local id
/// counters) — lives in the app struct, never in the document, so panning the camera, picking a face,
/// scrubbing frames, or ticking the reconstruction pipeline never lands in undo history nor syncs to
/// peers. The active utility is host-owned session state (`view_state.active_utility_id`), not stored
/// here.
struct RemodelPlayRuntime {
    camera: RemodelWorldCamera,
    selection: RemodelSelection,
    layers: RemodelLayerVisibility,
    frame_cursor: RemodelFrameCursor,
    report_table: String,
    /// ⚙️ The staged pipeline for the run started by the most recent `runReconstruction`/`retryStage`/
    /// `runStage`, if any is still in progress. `None` once a run reaches `Done`/`Failed`/cancelled.
    engine: Option<remodel_engine::ReconstructionEngine>,
    /// 📥️ Which `MediaStream` the current `importFrames`/`importVideo` file-picker batch is appending
    /// to — set on the batch's first (`index == 0`) payload dispatch, consumed by every following one.
    active_stream_id: Option<String>,
    active_video_import: Option<VideoImportScratch>,
    stream_counter: u32,
    job_counter: u32,
    gcp_counter: u32,
    import_counter: u32,
}

impl Default for RemodelPlayRuntime {
    fn default() -> Self {
        Self {
            camera: RemodelWorldCamera::default(),
            selection: RemodelSelection::default(),
            layers: RemodelLayerVisibility::default(),
            frame_cursor: RemodelFrameCursor::default(),
            report_table: "frames".into(),
            engine: None,
            active_stream_id: None,
            active_video_import: None,
            stream_counter: 0,
            job_counter: 0,
            gcp_counter: 0,
            import_counter: 0,
        }
    }
}
//#endregion 🔖️Runtime

//#region 🔖️ArgHelpers
/// 🔢️ Small accessors over a `handle_action` `args` object — every action handler below reads a
/// handful of typed fields out of the same staged-args shape these wrap.
fn arg_str<'a>(args: Option<&'a Value>, key: &str) -> Option<&'a str> {
    args.and_then(|value| value.get(key)).and_then(Value::as_str)
}
fn arg_u32(args: Option<&Value>, key: &str) -> Option<u32> {
    args.and_then(|value| value.get(key)).and_then(Value::as_u64).map(|value| value as u32)
}
fn arg_f32(args: Option<&Value>, key: &str) -> Option<f32> {
    args.and_then(|value| value.get(key)).and_then(Value::as_f64).map(|value| value as f32)
}
fn arg_f64(args: Option<&Value>, key: &str) -> Option<f64> {
    args.and_then(|value| value.get(key)).and_then(Value::as_f64)
}
fn arg_bool(args: Option<&Value>, key: &str) -> Option<bool> {
    args.and_then(|value| value.get(key)).and_then(Value::as_bool)
}
//#endregion 🔖️ArgHelpers

//#region 🔖️DocumentHelpers
fn world_meshes_json(scene: &RemodelScene) -> String {
    serde_json::to_string(&vec![json!({ "id": REMODEL_MESH_ID, "data": scene.results.mesh.mesh })]).unwrap_or_else(|_| "[]".into())
}

fn world_instances_json(runtime: &RemodelPlayRuntime) -> String {
    if !runtime.layers.mesh {
        return "[]".into();
    }
    serde_json::to_string(&vec![json!({
        "id": REMODEL_MESH_ID,
        "meshId": REMODEL_MESH_ID,
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "selected": false,
        "hovered": false,
    })])
    .unwrap_or_else(|_| "[]".into())
}

/// ☁️ `World3dScene.points_json` layers: the finished sparse/dense clouds once a run has produced
/// them, the live in-progress sparse preview streamed by `advanceReconstruction`, and every currently
/// recovered camera pose as its own (small, unattenuated) point layer — a documented simplification
/// standing in for a real camera-frustum gizmo, which `points_json` alone cannot express. Gcp world
/// positions are a fourth, always-static layer. `PackedF32`/`PackedU8`'s inner string is already a
/// base64 little-endian buffer, matching `positionsB64`/`colorsB64`'s wire shape byte-for-byte — no
/// decode/re-encode round trip needed.
fn world_points_json(scene: &RemodelScene, runtime: &RemodelPlayRuntime) -> Option<String> {
    let mut layers: Vec<Value> = Vec::new();
    if runtime.layers.sparse {
        if let Some(sparse) = &scene.results.sparse {
            if !sparse.points.is_empty() {
                layers.push(json!({
                    "id": "remodel-sparse",
                    "positionsB64": sparse.points.0,
                    "colorsB64": sparse.colors.as_ref().map(|colors| colors.0.clone()),
                    "size": 3.0,
                    "sizeAttenuation": true,
                }));
            }
        }
        if !scene.job.sparse_point_cloud_preview.is_empty() {
            layers.push(json!({
                "id": "remodel-live-preview",
                "positionsB64": scene.job.sparse_point_cloud_preview.0,
                "colorsB64": Value::Null,
                "size": 4.0,
                "sizeAttenuation": true,
            }));
        }
    }
    if runtime.layers.dense {
        if let Some(dense) = &scene.results.dense {
            if !dense.positions.is_empty() {
                layers.push(json!({
                    "id": "remodel-dense",
                    "positionsB64": dense.positions.0,
                    "colorsB64": dense.colors.as_ref().map(|colors| colors.0.clone()),
                    "size": 2.0,
                    "sizeAttenuation": true,
                }));
            }
        }
    }
    if runtime.layers.cameras && !scene.job.camera_poses_preview.is_empty() {
        let positions: Vec<f32> = scene.job.camera_poses_preview.iter().flat_map(|pose| pose.translation).collect();
        layers.push(json!({
            "id": "remodel-camera-poses",
            "positionsB64": PackedF32::from_f32_slice(&positions).0,
            "colorsB64": Value::Null,
            "size": 9.0,
            "sizeAttenuation": false,
        }));
    }
    if runtime.layers.gcps && !scene.gcps.is_empty() {
        let positions: Vec<f32> = scene.gcps.iter().flat_map(|gcp| gcp.world_position.map(|c| c as f32)).collect();
        layers.push(json!({
            "id": "remodel-gcps",
            "positionsB64": PackedF32::from_f32_slice(&positions).0,
            "colorsB64": Value::Null,
            "size": 10.0,
            "sizeAttenuation": false,
        }));
    }
    if layers.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into()))
    }
}

/// 🖼️ `remodel-frames`' Canvas2d layers for the currently cursored frame: the frame image itself (as a
/// data URL, decoded straight from the stored `ImageAsset`) plus every GCP observation planted on it,
/// as point markers. Keypoint circles/match lines/track polylines are a documented gap: those live only
/// in `remodel_engine`'s in-progress runtime scratch and are never distilled into durable document
/// state, so there is nothing to render for them once a run finishes (or before one starts).
fn frames_layers_json(scene: &RemodelScene, cursor: &RemodelFrameCursor) -> String {
    let mut layers: Vec<Value> = Vec::new();
    let Some(stream_id) = &cursor.stream_id else { return "[]".into() };
    let Some(stream) = scene.streams.iter().find(|stream| &stream.id == stream_id) else { return "[]".into() };
    if let Some(frame) = stream.frames.iter().find(|frame| frame.index == cursor.frame_index) {
        if let Some(asset) = scene.assets.get(&frame.asset_id) {
            layers.push(json!({
                "type": "image",
                "assetId": frame.asset_id,
                "dataUrl": format!("data:{};base64,{}", asset.mime, asset.data),
                "width": asset.width,
                "height": asset.height,
            }));
        }
    }
    let mut points: Vec<Value> = Vec::new();
    for gcp in &scene.gcps {
        for observation in &gcp.observations {
            if &observation.stream_id == stream_id && observation.frame_index == cursor.frame_index {
                points.push(json!({ "x": observation.pixel[0], "y": observation.pixel[1], "label": gcp.name }));
            }
        }
    }
    if !points.is_empty() {
        layers.push(json!({ "type": "points", "id": "remodel-gcp-observations", "points": points }));
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

/// 📊️ `remodel-report`'s runtime-selected `(columns_json, rows_json)` for one dataset name.
fn report_table_json(scene: &RemodelScene, table: &str) -> (String, String) {
    let (columns, rows): (Vec<Value>, Vec<Value>) = match table {
        "cameras" => (
            vec![json!({ "id": "id", "label": "Id" }), json!({ "id": "model", "label": "Model" }), json!({ "id": "fx", "label": "fx" }), json!({ "id": "fy", "label": "fy" }), json!({ "id": "rms", "label": "RMS (px)" })],
            scene.calibration.cameras.iter().map(|camera| json!({ "id": camera.id, "model": camera.model, "fx": camera.fx, "fy": camera.fy, "rms": camera.rms_reprojection_px })).collect(),
        ),
        "tracks" => (
            vec![
                json!({ "id": "id", "label": "Id" }),
                json!({ "id": "length", "label": "Length" }),
                json!({ "id": "class", "label": "Class" }),
                json!({ "id": "speed", "label": "Mean Speed (m/s)" }),
            ],
            scene.results.tracks.iter().map(|track| json!({ "id": track.id, "length": track.length, "class": format!("{:?}", track.class), "speed": track.mean_speed_m_s })).collect(),
        ),
        "gcps" => (
            vec![
                json!({ "id": "id", "label": "Id" }),
                json!({ "id": "name", "label": "Name" }),
                json!({ "id": "x", "label": "X" }),
                json!({ "id": "y", "label": "Y" }),
                json!({ "id": "z", "label": "Z" }),
                json!({ "id": "observations", "label": "Observations" }),
            ],
            scene.gcps.iter().map(|gcp| json!({ "id": gcp.id, "name": gcp.name, "x": gcp.world_position[0], "y": gcp.world_position[1], "z": gcp.world_position[2], "observations": gcp.observations.len() })).collect(),
        ),
        "qcStages" => (
            vec![json!({ "id": "stage", "label": "Stage" }), json!({ "id": "status", "label": "Status" })],
            vec![json!({ "stage": format!("{:?}", scene.job.stage), "status": if scene.job.error.is_some() { "error" } else { "ok" } })],
        ),
        "matches" => (
            vec![json!({ "id": "note", "label": "Note" })],
            vec![json!({ "note": "Pairwise match data is reconstruction-runtime scratch, never distilled into durable document state." })],
        ),
        _ => (
            vec![
                json!({ "id": "streamId", "label": "Stream" }),
                json!({ "id": "index", "label": "Index" }),
                json!({ "id": "timestampMs", "label": "Timestamp (ms)" }),
                json!({ "id": "assetId", "label": "Asset" }),
            ],
            scene
                .streams
                .iter()
                .flat_map(|stream| stream.frames.iter().map(move |frame| json!({ "streamId": stream.id, "index": frame.index, "timestampMs": frame.timestamp_ms, "assetId": frame.asset_id })))
                .collect(),
        ),
    };
    (serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()))
}

fn placeholder_result() -> RemodelMesh {
    RemodelMesh { mesh: mesh_from_kind("box"), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
}

fn empty_result() -> RemodelMesh {
    RemodelMesh { mesh: MeshData::default(), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
}

/// 📦️ Decodes a `requestFileOpen(readAs: "dataUrl")`/`RequestMediaFrames` payload into `(mime, bytes)`.
fn payload_from_data_url(data_url: &str) -> Option<(String, Vec<u8>)> {
    let (header, encoded) = data_url.split_once(',')?;
    let mime = header.strip_prefix("data:")?.split(';').next().unwrap_or("application/octet-stream").to_string();
    let bytes = base64::engine::general_purpose::STANDARD.decode(encoded).ok()?;
    Some((mime, bytes))
}

fn decode_still_image(mime: &str, bytes: &[u8]) -> Result<remodel_image::ImageRgba8, remodel_image::ImageError> {
    if mime.contains("jpeg") || mime.contains("jpg") {
        remodel_image::decode_jpeg(bytes)
    } else {
        remodel_image::decode_png(bytes)
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the remodel play app; one field per label makes every locale
    /// combination compile-checked. Native-only (no reuse-terminology variant): remodel's domain nouns
    /// (video/reconstruction/mesh/vertices/triangles) do not map onto the Object/Vortex/Attraction
    /// reuse vocabulary. House convention: no umlauts in German strings (ae/oe/ue/ss).
    struct RemodelLabels {
        model: &'static str = en: "Model", de: "Modell";
        capture: &'static str = en: "Capture", de: "Aufnahme";
        analyze: &'static str = en: "Analyze", de: "Analyse";
        default_example: &'static str = en: "Default", de: "Standard";
        reconstruction: &'static str = en: "Reconstruction", de: "Rekonstruktion";
        error: &'static str = en: "error", de: "Fehler";
        status: &'static str = en: "Status", de: "Status";
        running: &'static str = en: "Running", de: "Läuft";
        idle: &'static str = en: "Idle", de: "Leerlauf";
        utility: &'static str = en: "Utility", de: "Werkzeug";
        selection: &'static str = en: "selection", de: "Auswahl";
        mesh: &'static str = en: "Mesh", de: "Mesh";
        vertices: &'static str = en: "vertices", de: "Vertices";
        triangles: &'static str = en: "triangles", de: "Dreiecke";
        streams: &'static str = en: "Streams", de: "Streams";
        assets: &'static str = en: "Assets", de: "Assets";
        no_streams: &'static str = en: "No media streams imported yet", de: "Noch keine Medien-Streams importiert";
        stream_kind_video: &'static str = en: "video", de: "Video";
        stream_kind_image_sequence: &'static str = en: "image sequence", de: "Bildsequenz";
        frames: &'static str = en: "frames", de: "Frames";
        sync_offset: &'static str = en: "sync offset", de: "Sync-Versatz";
        sparse_cloud: &'static str = en: "Sparse point cloud", de: "Dünne Punktwolke";
        dense_cloud: &'static str = en: "Dense point cloud", de: "Dichte Punktwolke";
        results_none: &'static str = en: "none", de: "keine";
        trajectory: &'static str = en: "Trajectory", de: "Trajektorie";
        poses: &'static str = en: "poses", de: "Posen";
        geo_products: &'static str = en: "Geo products", de: "Geo-Produkte";
        available: &'static str = en: "available", de: "verfügbar";
        params_ingest: &'static str = en: "Ingest", de: "Ingest";
        params_feature: &'static str = en: "Feature", de: "Feature";
        params_matching: &'static str = en: "Matching", de: "Matching";
        params_sfm: &'static str = en: "SfM", de: "SfM";
        params_dense: &'static str = en: "Dense", de: "Dense";
        params_mesh: &'static str = en: "Mesh", de: "Mesh";
        params_motion: &'static str = en: "Motion", de: "Bewegung";
        params_geo: &'static str = en: "Geo", de: "Geo";
        stride_short: &'static str = en: "stride", de: "Schrittweite";
        max_short: &'static str = en: "max", de: "max";
        downscale_short: &'static str = en: "downscale", de: "Verkleinerung";
        target_short: &'static str = en: "target", de: "Ziel";
        octaves_short: &'static str = en: "octaves", de: "Oktaven";
        ratio_short: &'static str = en: "ratio", de: "Verhältnis";
        window_short: &'static str = en: "window", de: "Fenster";
        ransac_short: &'static str = en: "ransac", de: "Ransac";
        min_track_short: &'static str = en: "min track", de: "min. Spur";
        ba_short: &'static str = en: "ba", de: "BA";
        voxel_short: &'static str = en: "voxel", de: "Voxel";
        enabled: &'static str = en: "enabled", de: "aktiviert";
        disabled: &'static str = en: "disabled", de: "deaktiviert";
        cameras_calibrated: &'static str = en: "Calibrated cameras", de: "Kalibrierte Kameras";
        rig_extrinsics: &'static str = en: "Rig extrinsics", de: "Rig-Extrinsik";
        gcps: &'static str = en: "Ground control points", de: "Passpunkte";
        tracks: &'static str = en: "Motion tracks", de: "Bewegungsspuren";
        tracks_none: &'static str = en: "No motion tracks", de: "Keine Bewegungsspuren";
        motion_not_implemented: &'static str = en: "Motion tracking is not yet driven by the reconstruction engine", de: "Bewegungsverfolgung wird von der Rekonstruktions-Engine noch nicht ausgeführt";
        qc_none: &'static str = en: "No quality report yet", de: "Noch kein Qualitätsbericht";
        qc_reprojection: &'static str = en: "Mean reprojection error", de: "Mittlerer Reprojektionsfehler";
        qc_track_length: &'static str = en: "Mean track length", de: "Mittlere Spurlänge";
        qc_registered_ratio: &'static str = en: "Registered frame ratio", de: "Anteil registrierter Frames";
        qc_dense_coverage: &'static str = en: "Dense coverage ratio", de: "Dense-Abdeckungsanteil";
        qc_gcp_rmse: &'static str = en: "GCP checkpoint RMSE", de: "Passpunkt-Kontroll-RMSE";
        qc_watertight: &'static str = en: "Watertight", de: "Wasserdicht";
        qc_boundary_edges: &'static str = en: "Boundary edges", de: "Ränder";
        qc_components: &'static str = en: "Connected components", de: "Zusammenhangskomponenten";
        qc_euler: &'static str = en: "Euler characteristic", de: "Euler-Charakteristik";
        qc_genus: &'static str = en: "Genus", de: "Genus";
        qc_closed_fallback: &'static str = en: "Closed via fallback", de: "Über Fallback geschlossen";
        panel_media: &'static str = en: "Media", de: "Medien";
        panel_pipeline: &'static str = en: "Pipeline", de: "Pipeline";
        panel_results: &'static str = en: "Results", de: "Ergebnisse";
        panel_parameters: &'static str = en: "Parameters", de: "Parameter";
        panel_calibration: &'static str = en: "Calibration", de: "Kalibrierung";
        panel_tracks: &'static str = en: "Tracks", de: "Spuren";
        panel_qc: &'static str = en: "Quality", de: "Qualität";
        window_frames: &'static str = en: "Frames", de: "Frames";
        window_report: &'static str = en: "Report", de: "Bericht";
        layers: &'static str = en: "Layers", de: "Ebenen";
        layer_mesh: &'static str = en: "Mesh", de: "Mesh";
        layer_dense: &'static str = en: "Dense cloud", de: "Dichte Punktwolke";
        layer_sparse: &'static str = en: "Sparse cloud", de: "Dünne Punktwolke";
        layer_cameras: &'static str = en: "Cameras", de: "Kameras";
        layer_gcps: &'static str = en: "GCPs", de: "Passpunkte";
    }
}

/// 🗣️ Resolves the active label set from the shell-provided locale; unrecognized locales fall back to English.
fn remodel_labels(view_state: &ViewState) -> &'static RemodelLabels {
    resolve_labels::<RemodelLabels>(view_state)
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_remodel_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the builder chain.
fn remodel_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    localized_label_map(
        is_de,
        &[
            ("runReconstruction", "Run Reconstruction", "Rekonstruktion starten"),
            ("cancelReconstruction", "Cancel Reconstruction", "Rekonstruktion abbrechen"),
            ("retryStage", "Retry", "Wiederholen"),
            ("runStage", "Run Stage", "Stufe ausführen"),
            ("importFrames", "Import Frames", "Frames importieren"),
            ("importVideo", "Import Video", "Video importieren"),
            ("addStream", "Add Stream", "Stream hinzufügen"),
            ("removeStream", "Remove Stream", "Stream entfernen"),
            ("setStreamSync", "Set Stream Sync", "Stream-Synchronisation festlegen"),
            ("editCalibration", "Edit Calibration", "Kalibrierung bearbeiten"),
            ("calibrateCameras", "Calibrate Cameras", "Kameras kalibrieren"),
            ("addGcp", "Add Ground Control Point", "Passpunkt hinzufügen"),
            ("removeGcp", "Remove Ground Control Point", "Passpunkt entfernen"),
            ("placeGcpObservation", "Place GCP Observation", "Passpunkt-Beobachtung setzen"),
            ("setIngestParams", "Set Ingest Params", "Ingest-Parameter festlegen"),
            ("setFeatureParams", "Set Feature Params", "Feature-Parameter festlegen"),
            ("setMatchParams", "Set Match Params", "Match-Parameter festlegen"),
            ("setSfmParams", "Set SfM Params", "SfM-Parameter festlegen"),
            ("setDenseParams", "Set Dense Params", "Dense-Parameter festlegen"),
            ("setMeshParams", "Set Mesh Params", "Mesh-Parameter festlegen"),
            ("setMotionParams", "Set Motion Params", "Bewegungs-Parameter festlegen"),
            ("setGeoParams", "Set Geo Params", "Geo-Parameter festlegen"),
            ("resetPlaceholderMesh", "Reset Placeholder Mesh", "Platzhalter-Mesh zurücksetzen"),
            ("clearSparse", "Clear Sparse Cloud", "Dünne Punktwolke löschen"),
            ("clearDense", "Clear Dense Cloud", "Dichte Punktwolke löschen"),
            ("clearMeshResult", "Clear Mesh", "Mesh löschen"),
            ("clearTracks", "Clear Tracks", "Spuren löschen"),
            ("clearGeoProducts", "Clear Geo Products", "Geo-Produkte löschen"),
            ("clearResult", "Clear Result", "Ergebnis löschen"),
            ("exportQcReport", "Export QC Report", "QC-Bericht exportieren"),
        ],
    )
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_remodel_app`.
fn remodel_utility_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    localized_label_map(is_de, &[("select", "Select", "Auswählen"), ("sculpt", "Sculpt", "Formen"), ("measure", "Measure", "Messen"), ("gcpPlace", "Place GCP", "Passpunkt setzen")])
}
//#endregion 🔖️CommandLabels

//#region 🔖️PanelBuilders
/// 🗂️ `remodel.media` — drop zone plus a summary line per imported stream/asset.
fn build_media_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let mut lines = vec![
        ui_import_drop_zone("remodel-media-drop", labels.panel_media, labels.no_streams, Some(REMODEL_MEDIA_ACCEPT), remodel_action("importFramePayload", None)),
        ui_text(format!("{}: {} - {}: {}", labels.streams, scene.streams.len(), labels.assets, scene.assets.len())),
    ];
    for stream in &scene.streams {
        let kind_label = match stream.kind {
            MediaKind::Video => labels.stream_kind_video,
            MediaKind::ImageSequence => labels.stream_kind_image_sequence,
        };
        lines.push(ui_text(format!("{} ({kind_label}, {} {}, {}: {:.1}ms)", stream.name, stream.frames.len(), labels.frames, labels.sync_offset, stream.sync_offset_ms)));
        if let Some(source) = &stream.source {
            lines.push(ui_text(format!("  {:?} {}x{} {:.0}ms", source.codec, source.width, source.height, source.duration_ms)));
        }
    }
    ui_stack_vertical(lines)
}

/// 🚦️ `remodel.pipeline` — job status/progress plus live viewport session state.
fn build_pipeline_panel(scene: &RemodelScene, runtime: &RemodelPlayRuntime, active_utility: &str, labels: &RemodelLabels) -> UiNode {
    let job = &scene.job;
    let job_label = format!(
        "{}: {} ({:.0}%){}",
        labels.reconstruction,
        remodel_app_engine::stage_display(job.stage),
        job.progress_0_1 * 100.0,
        job.error.as_ref().map(|error| format!(" - {}: {error}", labels.error)).unwrap_or_default()
    );
    let running_label = format!("{}: {}", labels.status, if runtime.engine.is_some() { labels.running } else { labels.idle });
    let utility_label = format!("{}: {} - {}: {} ({})", labels.utility, active_utility, labels.selection, runtime.selection.mode, runtime.selection.ids.len());
    ui_stack_vertical(vec![ui_text(job_label), ui_text(running_label), ui_text(utility_label)])
}

/// 🧵️ `remodel.results` — the products a run (partially) produced.
fn build_results_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let results = &scene.results;
    let mesh_label = format!("{}: {:?}, {} {}, {} {}", labels.mesh, results.mesh.source, results.mesh.mesh.vertex_count(), labels.vertices, results.mesh.mesh.triangle_count(), labels.triangles);
    let sparse_label = results.sparse.as_ref().map_or_else(|| format!("{}: {}", labels.sparse_cloud, labels.results_none), |sparse| format!("{}: {}", labels.sparse_cloud, sparse.points.to_f32_vec().len() / 3));
    let dense_label = results.dense.as_ref().map_or_else(|| format!("{}: {}", labels.dense_cloud, labels.results_none), |dense| format!("{}: {}", labels.dense_cloud, dense.positions.to_f32_vec().len() / 3));
    let trajectory_label =
        results.trajectory.as_ref().map_or_else(|| format!("{}: {}", labels.trajectory, labels.results_none), |trajectory| format!("{}: {} {}", labels.trajectory, trajectory.poses.len(), labels.poses));
    let geo_label = results.geo.as_ref().map_or_else(|| format!("{}: {}", labels.geo_products, labels.results_none), |_| format!("{}: {}", labels.geo_products, labels.available));
    ui_stack_vertical(vec![ui_text(mesh_label), ui_text(sparse_label), ui_text(dense_label), ui_text(trajectory_label), ui_text(geo_label)])
}

/// ⚙️ `remodel.parameters` — a read-only dump of the 8 param sub-groups (editing happens via the
/// per-group `setXParams` command-palette actions' typed arg forms, not inline fields here).
fn build_parameters_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let p = &scene.params;
    ui_stack_vertical(vec![
        ui_text(format!(
            "{}: {} {}, {} {}, {} {}px, min sharpness {:.2}",
            labels.params_ingest, labels.stride_short, p.ingest.frame_sample_stride, labels.max_short, p.ingest.max_frames, labels.downscale_short, p.ingest.downscale_long_edge_px, p.ingest.min_sharpness
        )),
        ui_text(format!("{}: {:?}, {} {}, {} {}", labels.params_feature, p.feature.detector, labels.target_short, p.feature.target_count, labels.octaves_short, p.feature.octaves)),
        ui_text(format!("{}: {:?}, {} {:.2}, {} {}", labels.params_matching, p.matching.matcher, labels.ratio_short, p.matching.ratio_test, labels.window_short, p.matching.sequential_window)),
        ui_text(format!("{}: {} {}, {} {}, {} {}", labels.params_sfm, labels.ransac_short, p.sfm.ransac_iterations, labels.min_track_short, p.sfm.min_track_length, labels.ba_short, p.sfm.ba_max_iterations)),
        ui_text(format!("{}: {:?}, {} {}px", labels.params_dense, p.dense.resolution, labels.window_short, p.dense.window_radius_px)),
        ui_text(format!(
            "{}: {} {:.1}mm, {} {}, watertight {}",
            labels.params_mesh, labels.voxel_short, p.mesh.tsdf_voxel_size_mm, labels.target_short, p.mesh.decimate_target_triangles, p.mesh.guarantee_watertight
        )),
        ui_text(format!("{}: {}", labels.params_motion, if p.motion.enabled { labels.enabled } else { labels.disabled })),
        ui_text(format!("{}: {}", labels.params_geo, if p.geo.enabled { labels.enabled } else { labels.disabled })),
    ])
}

/// 🎯️ `remodel.calibration` — per-camera calibration, rig extrinsics, ground control points.
fn build_calibration_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let mut lines = vec![ui_text(format!("{}: {} - {}: {}", labels.cameras_calibrated, scene.calibration.cameras.len(), labels.rig_extrinsics, scene.calibration.rig.len()))];
    for camera in &scene.calibration.cameras {
        lines.push(ui_text(format!("{} ({}): fx {:.1} fy {:.1}", camera.label, camera.model, camera.fx, camera.fy)));
    }
    lines.push(ui_text(format!("{}: {}", labels.gcps, scene.gcps.len())));
    for gcp in &scene.gcps {
        lines.push(ui_text(format!("{} [{:.2}, {:.2}, {:.2}] ({} obs)", gcp.name, gcp.world_position[0], gcp.world_position[1], gcp.world_position[2], gcp.observations.len())));
    }
    ui_stack_vertical(lines)
}

/// 🏃️ `remodel.tracks` — moving-object motion tracks. `remodel_engine` does not yet drive
/// `remodel_motion` from `advance()` (its `motion_enabled` flag is accepted but unused), so this stays
/// empty today — a documented gap, not a UI bug.
fn build_tracks_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    if scene.results.tracks.is_empty() {
        return ui_stack_vertical(vec![ui_text(labels.tracks_none), ui_text(labels.motion_not_implemented)]);
    }
    let mut lines = vec![ui_text(format!("{}: {}", labels.tracks, scene.results.tracks.len()))];
    for track in &scene.results.tracks {
        lines.push(ui_text(format!("{} ({:?}): {} frames, {:.2} m/s", track.id, track.class, track.length, track.mean_speed_m_s)));
    }
    ui_stack_vertical(lines)
}

/// ✅️ `remodel.qc` — the whole-run quality report, including the watertight sub-report.
fn build_qc_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let Some(qc) = &scene.results.qc else {
        return ui_stack_vertical(vec![ui_text(labels.qc_none)]);
    };
    let mut lines = vec![
        ui_text(format!("{}: {:.2}px", labels.qc_reprojection, qc.reprojection_rms_px)),
        ui_text(format!("{}: {:.1}", labels.qc_track_length, qc.mean_track_length)),
        ui_text(format!("{}: {:.0}%", labels.qc_registered_ratio, qc.registered_frame_ratio * 100.0)),
        ui_text(format!("{}: {:.0}%", labels.qc_dense_coverage, qc.dense_coverage_ratio * 100.0)),
    ];
    if let Some(rmse) = qc.gcp_checkpoint_rmse {
        lines.push(ui_text(format!("{}: {:.3}m", labels.qc_gcp_rmse, rmse)));
    }
    if let Some(watertight) = &qc.watertight {
        lines.push(ui_text(format!("{}: {}", labels.qc_watertight, watertight.is_watertight)));
        lines.push(ui_text(format!("{}: {}", labels.qc_boundary_edges, watertight.boundary_edge_count)));
        lines.push(ui_text(format!("{}: {}", labels.qc_components, watertight.connected_components)));
        lines.push(ui_text(format!("{}: {}", labels.qc_euler, watertight.euler_characteristic)));
        if let Some(genus) = watertight.genus {
            lines.push(ui_text(format!("{}: {}", labels.qc_genus, genus)));
        }
        lines.push(ui_text(format!("{}: {}", labels.qc_closed_fallback, watertight.closed_fallback_used)));
    }
    for warning in &qc.warnings {
        lines.push(ui_text(format!("⚠️ {warning}")));
    }
    ui_stack_vertical(lines)
}
//#endregion 🔖️PanelBuilders

//#region 🔖️RemodelPlayApp
fn remodel_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: REMODEL_PLAY_APP_ID.into(), action: action.into(), args }
}

#[derive(Default)]
pub struct RemodelPlayApp {
    runtime: RemodelPlayRuntime,
}

impl RemodelPlayApp {
    //#region 🔖️Ingestion
    fn handle_import_frames(&mut self) -> ActionEmit<RemodelOperation> {
        ActionEmit::effect(HostEffect::RequestFileOpen { accept: REMODEL_MEDIA_ACCEPT.into(), read_as: Some("dataUrl".into()), import_action: "importFramePayload".into(), multiple: true })
    }

    fn handle_import_frame_payload(&mut self, args: Option<&Value>, doc: &DocumentView<'_, RemodelScene>) -> ActionEmit<RemodelOperation> {
        let Some(payload) = arg_str(args, "payload") else { return ActionEmit::default() };
        let Some((mime, bytes)) = payload_from_data_url(payload) else { return ActionEmit::default() };
        if mime.starts_with("video/") {
            return self.handle_import_video_bytes_payload(args, doc);
        }
        let name = arg_str(args, "name").unwrap_or("frame").to_string();
        let index = args.and_then(|value| value.get("index")).and_then(Value::as_u64).unwrap_or(0);

        let stream_id = if index == 0 || self.runtime.active_stream_id.is_none() {
            self.runtime.stream_counter += 1;
            self.runtime.import_counter += 1;
            let id = format!("stream-{}", self.runtime.stream_counter);
            self.runtime.active_stream_id = Some(id.clone());
            id
        } else {
            self.runtime.active_stream_id.clone().unwrap_or_default()
        };

        let (width, height) = decode_still_image(&mime, &bytes).map_or((0, 0), |image| (image.width, image.height));
        let asset_key = format!("{stream_id}-frame-{index}");
        let asset = ImageAsset { mime, data: base64::engine::general_purpose::STANDARD.encode(&bytes), width, height };

        let mut streams = doc.projection.streams.clone();
        match streams.iter_mut().find(|stream| stream.id == stream_id) {
            Some(stream) => {
                let frame_index = stream.frames.len() as u32;
                stream.frames.push(FrameRef { index: frame_index, timestamp_ms: f64::from(frame_index) * 1000.0 / 30.0, asset_id: asset_key.clone() });
            }
            None => {
                streams.push(MediaStream {
                    id: stream_id.clone(),
                    name,
                    kind: MediaKind::ImageSequence,
                    camera_id: None,
                    sync_offset_ms: 0.0,
                    fps_hint: 30.0,
                    frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: asset_key.clone() }],
                    source: None,
                });
            }
        }
        ActionEmit::amend(vec![RemodelOperation::SetAsset { key: asset_key, value: Some(asset) }, RemodelOperation::SetStreams { streams }], format!("remodel-import:{}", self.runtime.import_counter))
    }

    fn handle_import_video(&mut self, doc: &DocumentView<'_, RemodelScene>) -> ActionEmit<RemodelOperation> {
        let ingest = &doc.projection.params.ingest;
        ActionEmit::effect(HostEffect::RequestMediaFrames {
            accept: REMODEL_VIDEO_ACCEPT.into(),
            frame_action: "importVideoFramePayload".into(),
            done_action: "importVideoDone".into(),
            fallback_action: "importVideoBytesPayload".into(),
            sample_stride: ingest.frame_sample_stride,
            max_frames: ingest.max_frames,
            max_long_edge_px: ingest.downscale_long_edge_px,
            fps_hint: 0.0,
            payload: None,
            args: None,
        })
    }

    /// 🎞️ Host-decoded video frame tick (Tier 1/2 `RequestMediaFrames` frame dispatch): decodes the
    /// sampled JPEG, runs it through the relative blur gate, and amends it into the active stream.
    fn handle_import_video_frame_payload(&mut self, args: Option<&Value>, doc: &DocumentView<'_, RemodelScene>) -> ActionEmit<RemodelOperation> {
        let Some(payload) = arg_str(args, "payload") else { return ActionEmit::default() };
        let Some((_mime, bytes)) = payload_from_data_url(payload) else { return ActionEmit::default() };
        let Ok(image) = remodel_image::decode_jpeg(&bytes) else { return ActionEmit::default() };
        let index = args.and_then(|value| value.get("index")).and_then(Value::as_u64).unwrap_or(0);
        let frame_index = args.and_then(|value| value.get("frameIndex")).and_then(Value::as_u64).unwrap_or(index) as u32;
        let timestamp_ms = args.and_then(|value| value.get("timestampMs")).and_then(Value::as_f64).unwrap_or(0.0);
        let name = arg_str(args, "name").unwrap_or("video").to_string();

        if index == 0 || self.runtime.active_stream_id.is_none() {
            self.runtime.stream_counter += 1;
            self.runtime.import_counter += 1;
            self.runtime.active_stream_id = Some(format!("stream-{}", self.runtime.stream_counter));
            self.runtime.active_video_import = Some(VideoImportScratch::default());
        }
        let stream_id = self.runtime.active_stream_id.clone().unwrap_or_default();

        let score = local_sharpness_score(&image);
        let min_sharpness = doc.projection.params.ingest.min_sharpness;
        let scratch = self.runtime.active_video_import.get_or_insert_with(VideoImportScratch::default);
        if blur_gate_reject(scratch, score, min_sharpness) {
            return ActionEmit::default();
        }

        let asset_key = format!("{stream_id}-frame-{frame_index}");
        let asset = ImageAsset { mime: "image/jpeg".into(), data: base64::engine::general_purpose::STANDARD.encode(&bytes), width: image.width, height: image.height };
        let mut streams = doc.projection.streams.clone();
        match streams.iter_mut().find(|stream| stream.id == stream_id) {
            Some(stream) => {
                stream.kind = MediaKind::Video;
                stream.frames.push(FrameRef { index: frame_index, timestamp_ms, asset_id: asset_key.clone() });
            }
            None => streams.push(MediaStream {
                id: stream_id.clone(),
                name,
                kind: MediaKind::Video,
                camera_id: None,
                sync_offset_ms: 0.0,
                fps_hint: 0.0,
                frames: vec![FrameRef { index: frame_index, timestamp_ms, asset_id: asset_key.clone() }],
                source: None,
            }),
        }
        ActionEmit::amend(vec![RemodelOperation::SetAsset { key: asset_key, value: Some(asset) }, RemodelOperation::SetStreams { streams }], format!("remodel-import:{}", self.runtime.import_counter))
    }

    /// ✅️ Host-decoded video import finished: writes `VideoSource` provenance on the just-imported
    /// stream. Uses the SAME coalesce key as every preceding `importVideoFramePayload` tick, so the
    /// whole import (every accepted frame plus this final metadata write) collapses into one undo step.
    fn handle_import_video_done(&mut self, args: Option<&Value>, doc: &DocumentView<'_, RemodelScene>) -> ActionEmit<RemodelOperation> {
        let Some(stream_id) = self.runtime.active_stream_id.clone() else { return ActionEmit::default() };
        let name = arg_str(args, "name").unwrap_or("video").to_string();
        let duration_ms = args.and_then(|value| value.get("durationMs")).and_then(Value::as_f64).unwrap_or(0.0);
        let frame_count = args.and_then(|value| value.get("frameCount")).and_then(Value::as_u64).unwrap_or(0) as u32;
        let width = args.and_then(|value| value.get("width")).and_then(Value::as_u64).unwrap_or(0) as u32;
        let height = args.and_then(|value| value.get("height")).and_then(Value::as_u64).unwrap_or(0) as u32;
        let codec = remodel_app_engine::video_codec_from_label(arg_str(args, "codec").unwrap_or("unknown"));
        let import_counter = self.runtime.import_counter;
        self.runtime.active_stream_id = None;
        self.runtime.active_video_import = None;
        let mut streams = doc.projection.streams.clone();
        let Some(stream) = streams.iter_mut().find(|stream| stream.id == stream_id) else { return ActionEmit::default() };
        stream.source = Some(VideoSource { name, container: "unknown".into(), codec, duration_ms, frame_count, width, height });
        ActionEmit::amend(vec![RemodelOperation::SetStreams { streams }], format!("remodel-import:{import_counter}"))
    }

    /// 🎞️ Tier-3 fallback (or `importFrames`' own video-mime branch): the host couldn't decode the
    /// video, so it hands over the raw container bytes and this crate's own `remodel_video` demux/MJPEG/
    /// baseline-AVC decoder extracts frames fully in-process. Extraction here is a single bounded pass
    /// (already capped by `IngestParams.max_frames`/`frame_sample_stride`) rather than chunked
    /// tick-by-tick via `HostEffect::DispatchAction`: `remodel_video::extract_frames`'s `FrameIter`
    /// borrows its source bytes, so resuming it across separate `handle_action` calls would need
    /// self-referential storage — a documented simplification, not a correctness gap (the whole batch
    /// still collapses into one amended undo step). An undecodable codec surfaces as a `Notify` naming
    /// it, with provenance from the probe.
    fn handle_import_video_bytes_payload(&mut self, args: Option<&Value>, doc: &DocumentView<'_, RemodelScene>) -> ActionEmit<RemodelOperation> {
        let Some(payload) = arg_str(args, "payload") else { return ActionEmit::default() };
        let Some((_mime, bytes)) = payload_from_data_url(payload) else { return ActionEmit::default() };
        let probe = match remodel_video::probe(&bytes) {
            Ok(probe) => probe,
            Err(error) => return ActionEmit::effect(HostEffect::Notify { message: format!("Could not probe video: {error}") }),
        };
        let (codec, width, height, duration_ms, container) = remodel_app_engine::describe_video_probe(&probe);
        let ingest = &doc.projection.params.ingest;
        let opts = remodel_video::VideoIngestOptions { stride: ingest.frame_sample_stride.max(1), max_frames: ingest.max_frames, max_long_edge_px: ingest.downscale_long_edge_px };
        let iter = match remodel_video::extract_frames(&bytes, &opts) {
            Ok(iter) => iter,
            Err(error) => return ActionEmit::effect(HostEffect::Notify { message: format!("Unsupported video codec ({codec:?}): {error} - probed {container} {width}x{height}") }),
        };

        self.runtime.stream_counter += 1;
        self.runtime.import_counter += 1;
        let stream_id = format!("stream-{}", self.runtime.stream_counter);
        let name = arg_str(args, "name").unwrap_or("video").to_string();
        let min_sharpness = ingest.min_sharpness;
        let mut scratch = VideoImportScratch::default();
        let mut frames = Vec::new();
        let mut operations = Vec::new();
        for extracted in iter {
            let Ok(extracted) = extracted else { continue };
            let score = local_sharpness_score(&extracted.image);
            if blur_gate_reject(&mut scratch, score, min_sharpness) {
                continue;
            }
            let jpeg = remodel_image::encode_jpeg(&extracted.image, 90);
            let asset_key = format!("{stream_id}-frame-{}", extracted.index);
            operations.push(RemodelOperation::SetAsset { key: asset_key.clone(), value: Some(ImageAsset { mime: "image/jpeg".into(), data: base64::engine::general_purpose::STANDARD.encode(&jpeg), width: extracted.image.width, height: extracted.image.height }) });
            frames.push(FrameRef { index: extracted.index, timestamp_ms: extracted.timestamp_ms, asset_id: asset_key });
        }
        let mut streams = doc.projection.streams.clone();
        streams.push(MediaStream {
            id: stream_id,
            name,
            kind: MediaKind::Video,
            camera_id: None,
            sync_offset_ms: 0.0,
            fps_hint: 0.0,
            frames,
            source: Some(VideoSource { name: String::new(), container: container.into(), codec: remodel_app_engine::video_codec_to_document(codec), duration_ms, frame_count: 0, width, height }),
        });
        operations.push(RemodelOperation::SetStreams { streams });
        ActionEmit::amend(operations, format!("remodel-import:{}", self.runtime.import_counter))
    }
    //#endregion 🔖️Ingestion

    //#region 🔖️StagedReconstruction
    /// 🚀️ Validates ≥2 accepted frames, builds an engine from the current document params, pushes every
    /// stream's already-persisted frames into it (video streams are, by the time they reach this
    /// document, already an image sequence with true timestamps — see `remodel`'s own doc comment — so
    /// both `MediaKind` variants push identically), and schedules the first `advanceReconstruction` tick.
    fn handle_run_reconstruction(&mut self, doc: &DocumentView<'_, RemodelScene>) -> ActionEmit<RemodelOperation> {
        if self.runtime.engine.is_some() {
            return ActionEmit::default(); // a run is already in progress
        }
        let engine_params = remodel_app_engine::build_engine_params(&doc.projection.params);
        let mut engine = remodel_engine::ReconstructionEngine::new(&engine_params);
        let mut pushed = 0u32;
        for stream in &doc.projection.streams {
            for frame_ref in &stream.frames {
                let Some(asset) = doc.projection.assets.get(&frame_ref.asset_id) else { continue };
                let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&asset.data) else { continue };
                if let Ok(image) = decode_still_image(&asset.mime, &bytes) {
                    engine.push_frame(frame_ref.index, image, frame_ref.timestamp_ms);
                    pushed += 1;
                }
            }
        }
        if pushed < 2 {
            return ActionEmit::default(); // fewer than 2 accepted frames: too little to reconstruct from
        }
        self.runtime.job_counter += 1;
        let job_id = format!("job-{}", self.runtime.job_counter);
        self.runtime.engine = Some(engine);
        let job = ReconstructionJob { id: job_id.clone(), stage: ReconstructionStage::Ingesting, progress_0_1: 0.0, cancel_requested: false, stage_cursor: 0, started_at_ms: None, error: None, camera_poses_preview: Vec::new(), sparse_point_cloud_preview: PackedF32::default() };
        ActionEmit { operations: vec![RemodelOperation::SetJob { job }], coalesce_key: Some(format!("remodel-reconstruction:{job_id}")), effects: vec![HostEffect::DispatchAction { action: "advanceReconstruction".into(), args: None, delay_ms: 0 }], ..ActionEmit::default() }
    }

    /// ⚙️ Advances the pipeline by one bounded chunk, mirrors `EngineStatus` into an amended `SetJob`,
    /// distills result operations once `Done`, and re-dispatches itself unless terminal. Every emit here
    /// (including the terminal one) uses `ActionEmit::amend` with the SAME `remodel-reconstruction:{id}`
    /// coalesce key: `DocumentCommand::AmendLast` (see `vcs`) only folds a new edit into the
    /// previous one when both its coalesce key AND its position (still the last, still-uncommitted
    /// edit) match, so keeping the key identical end-to-end is what makes the whole multi-tick run
    /// collapse into exactly one undo step — using `ActionEmit::commit` on the terminal tick instead
    /// would start a brand-new `Apply`-based edit and defeat that contract (verified directly by
    /// `full_run_collapses_into_a_single_undo_step` below).
    fn handle_advance_reconstruction(&mut self, doc: &DocumentView<'_, RemodelScene>) -> ActionEmit<RemodelOperation> {
        if self.runtime.engine.is_none() {
            return ActionEmit::default();
        }
        let job_id = doc.projection.job.id.clone();
        if job_id.is_empty() {
            return ActionEmit::default();
        }
        let coalesce_key = format!("remodel-reconstruction:{job_id}");
        if doc.projection.job.cancel_requested {
            self.runtime.engine = None;
            let mut job = doc.projection.job.clone();
            job.stage = ReconstructionStage::Idle;
            job.cancel_requested = false;
            job.error = Some("Cancelled by user".into());
            return ActionEmit::amend(vec![RemodelOperation::SetJob { job }], coalesce_key);
        }
        let engine = self.runtime.engine.as_mut().expect("checked above");
        match engine.advance(RECONSTRUCTION_STEP_BUDGET) {
            remodel_engine::EngineStatus::Working { stage, progress } => {
                let preview = self.runtime.engine.as_ref().expect("engine present").sparse_preview();
                let mut job = doc.projection.job.clone();
                job.stage = remodel_app_engine::map_engine_stage(stage);
                job.progress_0_1 = progress;
                job.stage_cursor += 1;
                job.camera_poses_preview = preview.camera_poses.iter().enumerate().map(|(index, pose)| remodel_app_engine::camera_pose_preview(index as u32, pose)).collect();
                job.sparse_point_cloud_preview = PackedF32::from_f32_slice(&preview.packed_points);
                ActionEmit { operations: vec![RemodelOperation::SetJob { job }], coalesce_key: Some(coalesce_key), effects: vec![HostEffect::DispatchAction { action: "advanceReconstruction".into(), args: None, delay_ms: 0 }], ..ActionEmit::default() }
            }
            remodel_engine::EngineStatus::Done => {
                let accepted_count = engine.frame_source().accepted_count();
                let mut engine = self.runtime.engine.take().expect("checked above");
                let preview = engine.sparse_preview();
                let quality = engine.take_quality();
                let mesh_data = engine.take_mesh();
                let geo_products = engine.take_geo_products();
                drop(engine);

                let registered_count = preview.camera_poses.len();
                let camera_previews: Vec<CameraPosePreview> = preview.camera_poses.iter().enumerate().map(|(index, pose)| remodel_app_engine::camera_pose_preview(index as u32, pose)).collect();

                let mut job = doc.projection.job.clone();
                job.stage = ReconstructionStage::Done;
                job.progress_0_1 = 1.0;
                job.error = None;
                job.camera_poses_preview = camera_previews.clone();
                job.sparse_point_cloud_preview = PackedF32::from_f32_slice(&preview.packed_points);

                let mut operations = vec![RemodelOperation::SetJob { job }];
                operations.push(RemodelOperation::SetSparse { sparse: Some(SparseCloud { points: PackedF32::from_f32_slice(&preview.packed_points), colors: None }) });
                if !camera_previews.is_empty() {
                    operations.push(RemodelOperation::SetTrajectory { trajectory: Some(CameraTrajectory { poses: camera_previews }) });
                }
                if let Some(mesh_data) = mesh_data {
                    let watertight = quality.as_ref().and_then(|quality| quality.watertight.as_ref()).map(remodel_app_engine::watertight_snapshot);
                    let mut texture_asset_id = None;
                    if let Some(texture) = &mesh_data.paint_texture_base64 {
                        let texture_size = doc.projection.params.mesh.texture_size;
                        let asset_id = format!("mesh-texture-{job_id}");
                        operations.push(RemodelOperation::SetAsset { key: asset_id.clone(), value: Some(ImageAsset { mime: "image/png".into(), data: texture.clone(), width: texture_size, height: texture_size }) });
                        texture_asset_id = Some(asset_id);
                    }
                    operations.push(RemodelOperation::SetMeshResult { mesh: Box::new(RemodelMesh { mesh: mesh_data, source: MeshSource::Reconstructed, texture_asset_id, watertight }) });
                }
                if let Some(quality) = &quality {
                    operations.push(RemodelOperation::SetQc { qc: Some(remodel_app_engine::build_qc_snapshot(quality, registered_count, accepted_count, doc.projection.gcps.len())) });
                }
                if let Some(geo) = geo_products {
                    let dsm_id = format!("geo-dsm-{job_id}");
                    let dtm_id = format!("geo-dtm-{job_id}");
                    operations.push(RemodelOperation::SetAsset { key: dsm_id.clone(), value: Some(remodel_app_engine::raster_to_png_asset(&geo.dsm)) });
                    operations.push(RemodelOperation::SetAsset { key: dtm_id.clone(), value: Some(remodel_app_engine::raster_to_png_asset(&geo.dtm)) });
                    operations.push(RemodelOperation::SetGeoProducts { geo: Some(GeoProducts { dsm_asset_id: Some(dsm_id), dtm_asset_id: Some(dtm_id), ortho_asset_id: None }) });
                }
                ActionEmit::amend(operations, coalesce_key)
            }
            remodel_engine::EngineStatus::Failed(message) => {
                self.runtime.engine = None;
                let mut job = doc.projection.job.clone();
                job.stage = ReconstructionStage::Failed;
                job.error = Some(message);
                ActionEmit::amend(vec![RemodelOperation::SetJob { job }], coalesce_key)
            }
        }
    }

    fn handle_cancel_reconstruction(&mut self, doc: &DocumentView<'_, RemodelScene>) -> ActionEmit<RemodelOperation> {
        if self.runtime.engine.is_none() {
            return ActionEmit::default();
        }
        let mut job = doc.projection.job.clone();
        job.cancel_requested = true;
        let job_id = job.id.clone();
        ActionEmit::amend(vec![RemodelOperation::SetJob { job }], format!("remodel-reconstruction:{job_id}"))
    }

    /// 🔁️ The cooperative engine cannot resume from an arbitrary interior stage (`advance` is a strict
    /// forward state machine with no checkpoint/rewind) — a genuine `remodel_engine` constraint, not a
    /// program oversight. `retryStage`/`runStage` therefore both start a brand-new full run, exactly like
    /// `runReconstruction`; `runStage`'s `stage` arg is accepted but currently has no effect beyond that
    /// — a documented scope-down.
    fn handle_retry_or_run_stage(&mut self, doc: &DocumentView<'_, RemodelScene>) -> ActionEmit<RemodelOperation> {
        self.handle_run_reconstruction(doc)
    }
    //#endregion 🔖️StagedReconstruction
}
//#endregion 🔖️RemodelPlayApp

impl DocumentApp for RemodelPlayApp {
    type Projection = RemodelScene;
    type Operation = RemodelOperation;

    fn app_id(&self) -> &str {
        REMODEL_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        REMODEL_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> RemodelScene {
        default_remodel_scene()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, RemodelScene>, _view_state: &ViewState) -> ActionEmit<RemodelOperation> {
        match action {
            //#region 🔖️ViewActions
            SET_ACTIVE_UTILITY_ACTION_ID => ActionEmit::default(),
            "setSelection" => {
                if let Some(mode) = arg_str(args, "mode") {
                    self.runtime.selection.mode = mode.into();
                }
                self.runtime.selection.ids = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                ActionEmit::default()
            }
            "setCamera" => {
                if let Some(position) = args.and_then(|value| value.get("position")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()) {
                    self.runtime.camera.position = position;
                }
                if let Some(target) = args.and_then(|value| value.get("target")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()) {
                    self.runtime.camera.target = target;
                }
                if let Some(fov) = arg_f64(args, "fov") {
                    self.runtime.camera.fov = fov;
                }
                ActionEmit::default()
            }
            "setLayerVisibility" => {
                if let (Some(layer), Some(visible)) = (arg_str(args, "layer"), arg_bool(args, "visible")) {
                    match layer {
                        "mesh" => self.runtime.layers.mesh = visible,
                        "dense" => self.runtime.layers.dense = visible,
                        "sparse" => self.runtime.layers.sparse = visible,
                        "cameras" => self.runtime.layers.cameras = visible,
                        "gcps" => self.runtime.layers.gcps = visible,
                        _ => {}
                    }
                }
                ActionEmit::default()
            }
            "setFrameCursor" => {
                if let Some(stream_id) = arg_str(args, "streamId") {
                    self.runtime.frame_cursor.stream_id = Some(stream_id.into());
                }
                if let Some(frame_index) = arg_u32(args, "frameIndex") {
                    self.runtime.frame_cursor.frame_index = frame_index;
                }
                ActionEmit::default()
            }
            "setReportTable" => {
                if let Some(table) = arg_str(args, "table") {
                    self.runtime.report_table = table.into();
                }
                ActionEmit::default()
            }
            //#endregion 🔖️ViewActions

            //#region 🔖️Ingestion
            "importFrames" => self.handle_import_frames(),
            "importFramePayload" => self.handle_import_frame_payload(args, doc),
            "importVideo" => self.handle_import_video(doc),
            "importVideoFramePayload" => self.handle_import_video_frame_payload(args, doc),
            "importVideoDone" => self.handle_import_video_done(args, doc),
            "importVideoBytesPayload" => self.handle_import_video_bytes_payload(args, doc),
            "addStream" => {
                let name = arg_str(args, "name").unwrap_or("Stream").to_string();
                let kind = if arg_str(args, "kind") == Some("video") { MediaKind::Video } else { MediaKind::ImageSequence };
                let camera_id = arg_str(args, "cameraId").map(str::to_string);
                self.runtime.stream_counter += 1;
                let id = format!("stream-{}", self.runtime.stream_counter);
                let mut streams = doc.projection.streams.clone();
                streams.push(MediaStream { id, name, kind, camera_id, sync_offset_ms: 0.0, fps_hint: 30.0, frames: Vec::new(), source: None });
                ActionEmit::operations(vec![RemodelOperation::SetStreams { streams }])
            }
            "removeStream" => {
                let Some(stream_id) = arg_str(args, "streamId") else { return ActionEmit::default() };
                let streams: Vec<MediaStream> = doc.projection.streams.iter().filter(|stream| stream.id != stream_id).cloned().collect();
                ActionEmit::operations(vec![RemodelOperation::SetStreams { streams }])
            }
            "setStreamSync" => {
                let (Some(stream_id), Some(offset)) = (arg_str(args, "streamId"), arg_f64(args, "syncOffsetMs")) else { return ActionEmit::default() };
                let mut streams = doc.projection.streams.clone();
                let Some(stream) = streams.iter_mut().find(|stream| stream.id == stream_id) else { return ActionEmit::default() };
                stream.sync_offset_ms = offset;
                ActionEmit::operations(vec![RemodelOperation::SetStreams { streams }])
            }
            //#endregion 🔖️Ingestion

            //#region 🔖️CalibrationAndGcps
            "editCalibration" => {
                let Some(camera_id) = arg_str(args, "cameraId") else { return ActionEmit::default() };
                let label = arg_str(args, "label").unwrap_or(camera_id).to_string();
                let model = arg_str(args, "model").unwrap_or("pinhole").to_string();
                let entry = CameraCalibration {
                    id: camera_id.into(),
                    label,
                    model,
                    fx: arg_f64(args, "fx").unwrap_or(1000.0),
                    fy: arg_f64(args, "fy").unwrap_or(1000.0),
                    cx: arg_f64(args, "cx").unwrap_or(0.0),
                    cy: arg_f64(args, "cy").unwrap_or(0.0),
                    skew: arg_f64(args, "skew").unwrap_or(0.0),
                    distortion: [arg_f32(args, "k1").unwrap_or(0.0), arg_f32(args, "k2").unwrap_or(0.0), arg_f32(args, "k3").unwrap_or(0.0), arg_f32(args, "p1").unwrap_or(0.0), arg_f32(args, "p2").unwrap_or(0.0)],
                    rms_reprojection_px: None,
                    locked: arg_bool(args, "locked").unwrap_or(false),
                };
                let mut calibration = doc.projection.calibration.clone();
                match calibration.cameras.iter_mut().find(|camera| camera.id == camera_id) {
                    Some(existing) => *existing = entry,
                    None => calibration.cameras.push(entry),
                }
                ActionEmit::operations(vec![RemodelOperation::SetCalibration { calibration }])
            }
            // 🎯️ Auto-derives placeholder pinhole intrinsics (`fx = fy = max(width, height)`, principal
            // point centered, no distortion — mirroring `remodel_engine`'s own uncalibrated-input
            // heuristic) for every camera id referenced by a stream that has no calibration entry yet.
            // A documented simplification standing in for a real Zhang/checkerboard calibration pass
            // (no calibration target detection is wired into this program).
            "calibrateCameras" => {
                let mut calibration = doc.projection.calibration.clone();
                for stream in &doc.projection.streams {
                    let Some(camera_id) = &stream.camera_id else { continue };
                    if calibration.cameras.iter().any(|camera| &camera.id == camera_id) {
                        continue;
                    }
                    let Some(frame) = stream.frames.first() else { continue };
                    let Some(asset) = doc.projection.assets.get(&frame.asset_id) else { continue };
                    let (width, height) = (asset.width.max(1), asset.height.max(1));
                    let f = f64::from(width.max(height));
                    calibration.cameras.push(CameraCalibration {
                        id: camera_id.clone(),
                        label: camera_id.clone(),
                        model: "pinhole".into(),
                        fx: f,
                        fy: f,
                        cx: f64::from(width) / 2.0,
                        cy: f64::from(height) / 2.0,
                        skew: 0.0,
                        distortion: [0.0; 5],
                        rms_reprojection_px: None,
                        locked: false,
                    });
                }
                ActionEmit::operations(vec![RemodelOperation::SetCalibration { calibration }])
            }
            "addGcp" => {
                let name = arg_str(args, "name").unwrap_or("GCP").to_string();
                let world_position = [arg_f64(args, "worldX").unwrap_or(0.0), arg_f64(args, "worldY").unwrap_or(0.0), arg_f64(args, "worldZ").unwrap_or(0.0)];
                self.runtime.gcp_counter += 1;
                let mut gcps = doc.projection.gcps.clone();
                gcps.push(GroundControlPoint { id: format!("gcp-{}", self.runtime.gcp_counter), name, world_position, observations: Vec::new() });
                ActionEmit::operations(vec![RemodelOperation::SetGcps { gcps }])
            }
            "removeGcp" => {
                let Some(gcp_id) = arg_str(args, "gcpId") else { return ActionEmit::default() };
                let gcps: Vec<GroundControlPoint> = doc.projection.gcps.iter().filter(|gcp| gcp.id != gcp_id).cloned().collect();
                ActionEmit::operations(vec![RemodelOperation::SetGcps { gcps }])
            }
            "placeGcpObservation" => {
                let Some(gcp_id) = arg_str(args, "gcpId") else { return ActionEmit::default() };
                let (Some(stream_id), Some(frame_index)) = (arg_str(args, "streamId"), arg_u32(args, "frameIndex")) else { return ActionEmit::default() };
                let pixel = [arg_f32(args, "pixelX").unwrap_or(0.0), arg_f32(args, "pixelY").unwrap_or(0.0)];
                let mut gcps = doc.projection.gcps.clone();
                let Some(gcp) = gcps.iter_mut().find(|gcp| gcp.id == gcp_id) else { return ActionEmit::default() };
                gcp.observations.push(GcpObservation { stream_id: stream_id.into(), frame_index, pixel });
                ActionEmit::operations(vec![RemodelOperation::SetGcps { gcps }])
            }
            //#endregion 🔖️CalibrationAndGcps

            //#region 🔖️ParamSetters
            "setIngestParams" => ActionEmit::operations(vec![RemodelOperation::SetIngestParams {
                params: IngestParams {
                    frame_sample_stride: arg_u32(args, "frameSampleStride").unwrap_or(5),
                    max_frames: arg_u32(args, "maxFrames").unwrap_or(200),
                    downscale_long_edge_px: arg_u32(args, "downscaleLongEdgePx").unwrap_or(1600),
                    min_sharpness: arg_f32(args, "minSharpness").unwrap_or(0.3),
                },
            }]),
            "setFeatureParams" => ActionEmit::operations(vec![RemodelOperation::SetFeatureParams {
                params: FeatureParams {
                    detector: match arg_str(args, "detector") {
                        Some("akaze") => FeatureDetector::Akaze,
                        Some("harris") => FeatureDetector::Harris,
                        _ => FeatureDetector::Orb,
                    },
                    target_count: arg_u32(args, "targetCount").unwrap_or(4000),
                    octaves: arg_u32(args, "octaves").unwrap_or(4),
                    edge_threshold: arg_f32(args, "edgeThreshold").unwrap_or(10.0),
                },
            }]),
            "setMatchParams" => ActionEmit::operations(vec![RemodelOperation::SetMatchParams {
                params: MatchParams {
                    matcher: if arg_str(args, "matcher") == Some("kd-tree") { MatcherKind::KdTree } else { MatcherKind::BruteForce },
                    ratio_test: arg_f32(args, "ratioTest").unwrap_or(0.8),
                    cross_check: arg_bool(args, "crossCheck").unwrap_or(true),
                    sequential_window: arg_u32(args, "sequentialWindow").unwrap_or(8),
                    max_pairs_per_frame: arg_u32(args, "maxPairsPerFrame").unwrap_or(16),
                    loop_closure: arg_bool(args, "loopClosure").unwrap_or(true),
                },
            }]),
            "setSfmParams" => ActionEmit::operations(vec![RemodelOperation::SetSfmParams {
                params: SfmParams {
                    ransac_iterations: arg_u32(args, "ransacIterations").unwrap_or(1000),
                    ransac_threshold_px: arg_f32(args, "ransacThresholdPx").unwrap_or(2.0),
                    min_track_length: arg_u32(args, "minTrackLength").unwrap_or(3),
                    ba_max_iterations: arg_u32(args, "baMaxIterations").unwrap_or(50),
                    robust_loss: match arg_str(args, "robustLoss") {
                        Some("l2") => RobustLossKind::L2,
                        Some("cauchy") => RobustLossKind::Cauchy,
                        _ => RobustLossKind::Huber,
                    },
                    huber_delta_px: arg_f32(args, "huberDeltaPx").unwrap_or(1.5),
                },
            }]),
            "setDenseParams" => ActionEmit::operations(vec![RemodelOperation::SetDenseParams {
                params: DenseParams {
                    resolution: match arg_str(args, "resolution") {
                        Some("low") => DenseResolution::Low,
                        Some("high") => DenseResolution::High,
                        _ => DenseResolution::Medium,
                    },
                    window_radius_px: arg_u32(args, "windowRadiusPx").unwrap_or(3),
                    min_view_consistency: arg_u32(args, "minViewConsistency").unwrap_or(3),
                    confidence_threshold: arg_f32(args, "confidenceThreshold").unwrap_or(0.5),
                    max_points: arg_u32(args, "maxPoints").unwrap_or(500_000),
                },
            }]),
            "setMeshParams" => ActionEmit::operations(vec![RemodelOperation::SetMeshParams {
                params: DocumentMeshParams {
                    tsdf_voxel_size_mm: arg_f32(args, "tsdfVoxelSizeMm").unwrap_or(5.0),
                    tsdf_truncation_mm: arg_f32(args, "tsdfTruncationMm").unwrap_or(20.0),
                    decimate_target_triangles: arg_u32(args, "decimateTargetTriangles").unwrap_or(200_000),
                    smoothing_iterations: arg_u32(args, "smoothingIterations").unwrap_or(2),
                    texture_enabled: arg_bool(args, "textureEnabled").unwrap_or(true),
                    texture_size: arg_str(args, "textureSize").and_then(|value| value.parse::<u32>().ok()).unwrap_or(2048),
                    guarantee_watertight: arg_bool(args, "guaranteeWatertight").unwrap_or(true),
                    hole_fill_max_boundary_verts: arg_u32(args, "holeFillMaxBoundaryVerts").unwrap_or(512),
                    self_intersection_check: arg_bool(args, "selfIntersectionCheck").unwrap_or(false),
                },
            }]),
            "setMotionParams" => ActionEmit::operations(vec![RemodelOperation::SetMotionParams {
                params: MotionParams {
                    enabled: arg_bool(args, "enabled").unwrap_or(false),
                    max_tracks: arg_u32(args, "maxTracks").unwrap_or(64),
                    track_window_px: arg_u32(args, "trackWindowPx").unwrap_or(21),
                    min_track_quality: arg_f32(args, "minTrackQuality").unwrap_or(0.3),
                    min_track_length_frames: arg_u32(args, "minTrackLengthFrames").unwrap_or(5),
                },
            }]),
            "setGeoParams" => ActionEmit::operations(vec![RemodelOperation::SetGeoParams {
                params: GeoParams {
                    enabled: arg_bool(args, "enabled").unwrap_or(false),
                    origin_lon: arg_f64(args, "originLon"),
                    origin_lat: arg_f64(args, "originLat"),
                    origin_alt: arg_f64(args, "originAlt"),
                    gsd_m: arg_f32(args, "gsdM").unwrap_or(0.05),
                    dsm_cell_m: arg_f32(args, "dsmCellM").unwrap_or(0.1),
                    dtm_filter_radius_m: arg_f32(args, "dtmFilterRadiusM").unwrap_or(2.0),
                    ortho_max_px: arg_u32(args, "orthoMaxPx").unwrap_or(4096),
                },
            }]),
            //#endregion 🔖️ParamSetters

            //#region 🔖️StagedReconstruction
            "runReconstruction" => self.handle_run_reconstruction(doc),
            "advanceReconstruction" => self.handle_advance_reconstruction(doc),
            "cancelReconstruction" => self.handle_cancel_reconstruction(doc),
            "retryStage" | "runStage" => self.handle_retry_or_run_stage(doc),
            //#endregion 🔖️StagedReconstruction

            //#region 🔖️ClearReset
            "resetPlaceholderMesh" => ActionEmit::operations(vec![RemodelOperation::SetMeshResult { mesh: Box::new(placeholder_result()) }]),
            "clearSparse" => ActionEmit::operations(vec![RemodelOperation::SetSparse { sparse: None }]),
            "clearDense" => ActionEmit::operations(vec![RemodelOperation::SetDense { dense: None }]),
            "clearMeshResult" => ActionEmit::operations(vec![RemodelOperation::SetMeshResult { mesh: Box::new(empty_result()) }]),
            "clearTracks" => ActionEmit::operations(vec![RemodelOperation::SetTracks { tracks: Vec::new() }]),
            "clearGeoProducts" => ActionEmit::operations(vec![RemodelOperation::SetGeoProducts { geo: None }]),
            "clearResult" => ActionEmit::operations(vec![
                RemodelOperation::SetMeshResult { mesh: Box::new(empty_result()) },
                RemodelOperation::SetSparse { sparse: None },
                RemodelOperation::SetDense { dense: None },
                RemodelOperation::SetTrajectory { trajectory: None },
                RemodelOperation::SetTracks { tracks: Vec::new() },
                RemodelOperation::SetGeoProducts { geo: None },
                RemodelOperation::SetQc { qc: None },
            ]),
            //#endregion 🔖️ClearReset

            //#region 🔖️Export
            "exportQcReport" => {
                let Some(qc) = &doc.projection.results.qc else { return ActionEmit::default() };
                let data = serde_json::to_string_pretty(qc).unwrap_or_default();
                ActionEmit::effect(HostEffect::DownloadMediaExport { filename: "remodel-qc-report.json".into(), mime_type: "application/json".into(), data, encoding: None })
            }
            //#endregion 🔖️Export
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, RemodelScene>, view_state: &ViewState) -> UiNode {
        let scene = doc.projection;
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or(REMODEL_DEFAULT_UTILITY);
        let labels = remodel_labels(view_state);
        match body_key {
            REMODEL_PLAY_BODY_MAIN => {
                let mut world_scene = world3d_scene(
                    world3d_camera_json(self.runtime.camera.position, self.runtime.camera.target, self.runtime.camera.fov),
                    world_meshes_json(scene),
                    world_instances_json(&self.runtime),
                    world3d_selection_json(&self.runtime.selection.mode, &[], None),
                    &WorldSunConfig::default(),
                );
                world_scene.points_json = world_points_json(scene, &self.runtime);
                build_world_3d_scene(REMODEL_PLAY_SURFACE_MAIN, REMODEL_PLAY_APP_ID, world_scene)
            }
            REMODEL_PLAY_BODY_FRAMES => {
                let scene_2d = Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: frames_layers_json(scene, &self.runtime.frame_cursor) };
                build_canvas_2d_scene(REMODEL_PLAY_SURFACE_FRAMES, REMODEL_PLAY_APP_ID, scene_2d)
            }
            REMODEL_PLAY_BODY_REPORT => {
                let (columns_json, rows_json) = report_table_json(scene, &self.runtime.report_table);
                build_table_scene(REMODEL_PLAY_SURFACE_REPORT, REMODEL_PLAY_APP_ID, TableScene::base(columns_json, rows_json))
            }
            REMODEL_PLAY_BODY_MEDIA => build_media_panel(scene, labels),
            REMODEL_PLAY_BODY_PIPELINE => build_pipeline_panel(scene, &self.runtime, active_utility, labels),
            REMODEL_PLAY_BODY_RESULTS => build_results_panel(scene, labels),
            REMODEL_PLAY_BODY_PARAMETERS => build_parameters_panel(scene, labels),
            REMODEL_PLAY_BODY_CALIBRATION => build_calibration_panel(scene, labels),
            REMODEL_PLAY_BODY_TRACKS => build_tracks_panel(scene, labels),
            REMODEL_PLAY_BODY_QC => build_qc_panel(scene, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = remodel_labels(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(REMODEL_PLAY_WINDOW_MAIN, labels.model)
            .window_kind_label(REMODEL_PLAY_WINDOW_FRAMES, labels.window_frames)
            .window_kind_label(REMODEL_PLAY_WINDOW_REPORT, labels.window_report)
            .mode_label("capture", labels.capture)
            .mode_label("model", labels.model)
            .mode_label("analyze", labels.analyze)
            .action_labels(remodel_action_labels(is_de))
            .utility_labels(remodel_utility_labels(is_de))
            .example_labels(std::collections::HashMap::from([("default".to_string(), labels.default_example.to_string())]))
            .panel_tab_label(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL)
            .panel_tab_label(REMODEL_PANEL_MEDIA_ID, labels.panel_media)
            .panel_tab_label(REMODEL_PANEL_PIPELINE_ID, labels.panel_pipeline)
            .panel_tab_label(REMODEL_PANEL_RESULTS_ID, labels.panel_results)
            .panel_tab_label(REMODEL_PANEL_PARAMETERS_ID, labels.panel_parameters)
            .panel_tab_label(REMODEL_PANEL_CALIBRATION_ID, labels.panel_calibration)
            .panel_tab_label(REMODEL_PANEL_TRACKS_ID, labels.panel_tracks)
            .panel_tab_label(REMODEL_PANEL_QC_ID, labels.panel_qc)
    }

    /// 👁️ Dynamic per-render window measures — `remodel-main`'s `remodel.layers` toggle group must
    /// reflect the LIVE `self.runtime.layers` state (not a manifest-frozen snapshot), so it is supplied
    /// here rather than via `AppBuilder::window_kind_measures` (a static, build-once declaration — see
    /// `lowpoly-plugin`'s `world3d_sun_measures`/`window_measures` for the identical pattern this mirrors).
    fn window_measures(&self, _doc: &DocumentView<'_, RemodelScene>, view_state: &ViewState) -> std::collections::HashMap<String, Vec<semio_framework_plugin::WindowMeasure>> {
        std::collections::HashMap::from([(REMODEL_PLAY_WINDOW_MAIN.to_string(), vec![remodel_layer_measures(&self.runtime.layers, remodel_labels(view_state))])])
    }
}

//#region 🔖️Manifest
/// 👁️ `remodel.layers` — `remodel-main`'s layer-visibility toggle group (`setLayerVisibility`).
fn remodel_layer_measures(layers: &RemodelLayerVisibility, labels: &RemodelLabels) -> semio_framework_plugin::WindowMeasure {
    let toggle = |id: &str, icon: &str, label: &'static str, pressed: bool, layer: &str| semio_framework_plugin::WindowMeasure::Toggle {
        id: format!("remodel-measure-layer-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: remodel_action("setLayerVisibility", Some(json!({ "layer": layer, "visible": !pressed }))),
    };
    semio_framework_plugin::WindowMeasure::Group {
        id: "remodel-measure-layers".into(),
        label: labels.layers.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            toggle("mesh", "box", labels.layer_mesh, layers.mesh, "mesh"),
            toggle("dense", "cloud", labels.layer_dense, layers.dense, "dense"),
            toggle("sparse", "sparkles", labels.layer_sparse, layers.sparse, "sparse"),
            toggle("cameras", "camera", labels.layer_cameras, layers.cameras, "cameras"),
            toggle("gcps", "crosshair", labels.layer_gcps, layers.gcps, "gcps"),
        ],
    }
}

pub fn create_remodel_app() -> App {
    let default_example = default_remodel_scene().print_dsl();
    App::from_builder(
        App::builder(REMODEL_PLAY_APP_ID, "Remodel")
            .document(["semio", "remodel"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.remodel".into(),
                name: "3D Remodel".into(),
                source_format: "remodel.scene".into(),
                component_kind: "remodel".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
                schema: "remodel.scene".into(),
                export_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj, OsMediaFormat::Stl, OsMediaFormat::Ply, OsMediaFormat::Las, OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj],
            })
            .icon_id("remodel-app")
            .mode("capture", "Capture")
            .mode("model", "Model")
            .mode("analyze", "Analyze")
            .default_mode_id("model")
            .window_kind(REMODEL_PLAY_WINDOW_MAIN, "Model", REMODEL_PLAY_BODY_MAIN, SurfaceKind::World3d, "remodel-model")
            .window_kind(REMODEL_PLAY_WINDOW_FRAMES, "Frames", REMODEL_PLAY_BODY_FRAMES, SurfaceKind::Canvas2d, "layout-grid")
            .window_kind(REMODEL_PLAY_WINDOW_REPORT, "Report", REMODEL_PLAY_BODY_REPORT, SurfaceKind::Table, "document-report")
            .default_layout(create_default_layout(&[REMODEL_PLAY_WINDOW_MAIN.into(), REMODEL_PLAY_WINDOW_FRAMES.into()], "row", Some(&[70.0, 30.0]), Some(&["Model".into(), "Frames".into()])))
            .named_layout(create_named_layout(
                "remodel-capture",
                "Capture",
                create_default_layout(&[REMODEL_PLAY_WINDOW_FRAMES.into(), REMODEL_PLAY_WINDOW_MAIN.into()], "row", Some(&[60.0, 40.0]), Some(&["Frames".into(), "Model".into()])),
                "builtin",
                Some("video".into()),
                None,
            ))
            .named_layout(create_named_layout(
                "remodel-analyze",
                "Analyze",
                create_default_layout(&[REMODEL_PLAY_WINDOW_MAIN.into(), REMODEL_PLAY_WINDOW_REPORT.into()], "row", Some(&[60.0, 40.0]), Some(&["Model".into(), "Report".into()])),
                "builtin",
                Some("table".into()),
                None,
            ))
            .mode_layout("capture", "remodel-capture")
            .mode_layout("analyze", "remodel-analyze")
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, REMODEL_PLAY_BODY_PIPELINE)
            .panel_tab(REMODEL_PANEL_MEDIA_ID, "Media", PanelGroup::Workbench, REMODEL_PLAY_BODY_MEDIA)
            .panel_tab(REMODEL_PANEL_RESULTS_ID, "Results", PanelGroup::Workbench, REMODEL_PLAY_BODY_RESULTS)
            .panel_tab(REMODEL_PANEL_PARAMETERS_ID, "Parameters", PanelGroup::Details, REMODEL_PLAY_BODY_PARAMETERS)
            .panel_tab(REMODEL_PANEL_CALIBRATION_ID, "Calibration", PanelGroup::Details, REMODEL_PLAY_BODY_CALIBRATION)
            .panel_tab(REMODEL_PANEL_TRACKS_ID, "Tracks", PanelGroup::Details, REMODEL_PLAY_BODY_TRACKS)
            .panel_tab(REMODEL_PANEL_QC_ID, "Quality", PanelGroup::Settings, REMODEL_PLAY_BODY_QC)
            // 🚀️ Staged reconstruction: `runReconstruction` starts the run, `advanceReconstruction` is the
            // internal `DispatchAction` re-dispatch target (never user-invoked, so `in_palette: false`).
            .operation("runReconstruction", "Run Reconstruction")
            .operation("cancelReconstruction", "Cancel Reconstruction")
            .operation("retryStage", "Retry")
            .operation("runStage", "Run Stage")
            .action_args("runStage", vec![ActionArgDef::select(
                "stage",
                "Stage",
                vec![
                    ActionArgOption::new("extracting-features", "Extracting Features"),
                    ActionArgOption::new("matching-features", "Matching Features"),
                    ActionArgOption::new("estimating-poses", "Estimating Poses"),
                    ActionArgOption::new("bundle-adjusting", "Bundle Adjusting"),
                    ActionArgOption::new("dense-stereo", "Dense Stereo"),
                    ActionArgOption::new("fusing-volume", "Fusing Volume"),
                    ActionArgOption::new("extracting-surface", "Extracting Surface"),
                    ActionArgOption::new("texturing", "Texturing"),
                ],
            )
            .default_value("extracting-features")])
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("advanceReconstruction", "Advance Reconstruction", ActionKind::Operation) })
            // 📥️ Ingestion.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new("importFrames", "Import Frames", ActionKind::Shell) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("importFramePayload", "Import Frame Payload", ActionKind::Operation) })
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new("importVideo", "Import Video", ActionKind::Shell) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("importVideoFramePayload", "Import Video Frame Payload", ActionKind::Operation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("importVideoDone", "Import Video Done", ActionKind::Operation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("importVideoBytesPayload", "Import Video Bytes Payload", ActionKind::Operation) })
            .operation("addStream", "Add Stream")
            .action_args("addStream", vec![
                ActionArgDef::text("name", "Name").default_value("Stream"),
                ActionArgDef::select("kind", "Kind", vec![ActionArgOption::new("image-sequence", "Image Sequence"), ActionArgOption::new("video", "Video")]).default_value("image-sequence"),
                ActionArgDef::text("cameraId", "Camera Id").default_value("cam-0"),
            ])
            .operation("removeStream", "Remove Stream")
            .action_args("removeStream", vec![ActionArgDef::text("streamId", "Stream Id").required()])
            .operation("setStreamSync", "Set Stream Sync")
            .action_args("setStreamSync", vec![ActionArgDef::text("streamId", "Stream Id").required(), ActionArgDef::number("syncOffsetMs", "Sync Offset (ms)").default_value(0)])
            // 🎯️ Calibration / GCPs.
            .operation("editCalibration", "Edit Calibration")
            .action_args("editCalibration", vec![
                ActionArgDef::text("cameraId", "Camera Id").required(),
                ActionArgDef::text("label", "Label"),
                ActionArgDef::select("model", "Model", vec![ActionArgOption::new("pinhole", "Pinhole"), ActionArgOption::new("brownConrady", "Brown-Conrady"), ActionArgOption::new("fisheye", "Fisheye")]).default_value("pinhole"),
                ActionArgDef::number("fx", "fx").default_value(1000),
                ActionArgDef::number("fy", "fy").default_value(1000),
                ActionArgDef::number("cx", "cx").default_value(0),
                ActionArgDef::number("cy", "cy").default_value(0),
                ActionArgDef::number("skew", "Skew").default_value(0),
                ActionArgDef::number("k1", "k1").default_value(0),
                ActionArgDef::number("k2", "k2").default_value(0),
                ActionArgDef::number("k3", "k3").default_value(0),
                ActionArgDef::number("p1", "p1").default_value(0),
                ActionArgDef::number("p2", "p2").default_value(0),
                ActionArgDef::toggle("locked", "Locked").default_value(false),
            ])
            .operation("calibrateCameras", "Calibrate Cameras")
            .operation("addGcp", "Add Ground Control Point")
            .action_args("addGcp", vec![
                ActionArgDef::text("name", "Name").default_value("GCP"),
                ActionArgDef::number("worldX", "World X").default_value(0),
                ActionArgDef::number("worldY", "World Y").default_value(0),
                ActionArgDef::number("worldZ", "World Z").default_value(0),
            ])
            .operation("removeGcp", "Remove Ground Control Point")
            .action_args("removeGcp", vec![ActionArgDef::text("gcpId", "GCP Id").required()])
            .operation("placeGcpObservation", "Place GCP Observation")
            .action_args("placeGcpObservation", vec![
                ActionArgDef::text("gcpId", "GCP Id").required(),
                ActionArgDef::text("streamId", "Stream Id").required(),
                ActionArgDef::number("frameIndex", "Frame Index").required(),
                ActionArgDef::number("pixelX", "Pixel X").required(),
                ActionArgDef::number("pixelY", "Pixel Y").required(),
            ])
            // ⚙️ 8 param-group setters, one per `ReconstructionParams` sub-struct.
            .operation("setIngestParams", "Set Ingest Params")
            .action_args("setIngestParams", vec![
                ActionArgDef::number("frameSampleStride", "Frame Sample Stride").default_value(5),
                ActionArgDef::number("maxFrames", "Max Frames").default_value(200),
                ActionArgDef::number("downscaleLongEdgePx", "Downscale Long Edge (px)").default_value(1600),
                ActionArgDef::slider("minSharpness", "Min Sharpness", 0.0, 1.0).default_value(0.3),
            ])
            .operation("setFeatureParams", "Set Feature Params")
            .action_args("setFeatureParams", vec![
                ActionArgDef::select("detector", "Detector", vec![ActionArgOption::new("orb", "ORB"), ActionArgOption::new("akaze", "AKAZE"), ActionArgOption::new("harris", "Harris")]).default_value("orb"),
                ActionArgDef::number("targetCount", "Target Count").default_value(4000),
                ActionArgDef::number("octaves", "Octaves").default_value(4),
                ActionArgDef::slider("edgeThreshold", "Edge Threshold", 1.0, 50.0).default_value(10.0),
            ])
            .operation("setMatchParams", "Set Match Params")
            .action_args("setMatchParams", vec![
                ActionArgDef::select("matcher", "Matcher", vec![ActionArgOption::new("brute-force", "Brute Force"), ActionArgOption::new("kd-tree", "KD-Tree")]).default_value("brute-force"),
                ActionArgDef::slider("ratioTest", "Ratio Test", 0.1, 1.0).default_value(0.8),
                ActionArgDef::toggle("crossCheck", "Cross Check").default_value(true),
                ActionArgDef::number("sequentialWindow", "Sequential Window").default_value(8),
                ActionArgDef::number("maxPairsPerFrame", "Max Pairs Per Frame").default_value(16),
                ActionArgDef::toggle("loopClosure", "Loop Closure").default_value(true),
            ])
            .operation("setSfmParams", "Set SfM Params")
            .action_args("setSfmParams", vec![
                ActionArgDef::number("ransacIterations", "RANSAC Iterations").default_value(1000),
                ActionArgDef::slider("ransacThresholdPx", "RANSAC Threshold (px)", 0.1, 10.0).default_value(2.0),
                ActionArgDef::number("minTrackLength", "Min Track Length").default_value(3),
                ActionArgDef::number("baMaxIterations", "BA Max Iterations").default_value(50),
                ActionArgDef::select("robustLoss", "Robust Loss", vec![ActionArgOption::new("l2", "L2"), ActionArgOption::new("huber", "Huber"), ActionArgOption::new("cauchy", "Cauchy")]).default_value("huber"),
                ActionArgDef::slider("huberDeltaPx", "Huber Delta (px)", 0.1, 10.0).default_value(1.5),
            ])
            .operation("setDenseParams", "Set Dense Params")
            .action_args("setDenseParams", vec![
                ActionArgDef::select("resolution", "Resolution", vec![ActionArgOption::new("low", "Low"), ActionArgOption::new("medium", "Medium"), ActionArgOption::new("high", "High")]).default_value("medium"),
                ActionArgDef::number("windowRadiusPx", "Window Radius (px)").default_value(3),
                ActionArgDef::number("minViewConsistency", "Min View Consistency").default_value(3),
                ActionArgDef::slider("confidenceThreshold", "Confidence Threshold", 0.0, 1.0).default_value(0.5),
                ActionArgDef::number("maxPoints", "Max Points").default_value(500_000),
            ])
            .operation("setMeshParams", "Set Mesh Params")
            .action_args("setMeshParams", vec![
                ActionArgDef::slider("tsdfVoxelSizeMm", "TSDF Voxel Size (mm)", 1.0, 20.0).default_value(5.0),
                ActionArgDef::slider("tsdfTruncationMm", "TSDF Truncation (mm)", 2.0, 60.0).default_value(20.0),
                ActionArgDef::number("decimateTargetTriangles", "Decimate Target Triangles").default_value(200_000),
                ActionArgDef::number("smoothingIterations", "Smoothing Iterations").default_value(2),
                ActionArgDef::toggle("textureEnabled", "Texture Enabled").default_value(true),
                ActionArgDef::select("textureSize", "Texture Size", vec![ActionArgOption::new("1024", "1024"), ActionArgOption::new("2048", "2048"), ActionArgOption::new("4096", "4096")]).default_value("2048"),
                ActionArgDef::toggle("guaranteeWatertight", "Guarantee Watertight").default_value(true),
                ActionArgDef::number("holeFillMaxBoundaryVerts", "Hole Fill Max Boundary Verts").default_value(512),
                ActionArgDef::toggle("selfIntersectionCheck", "Self-Intersection Check").default_value(false),
            ])
            .operation("setMotionParams", "Set Motion Params")
            .action_args("setMotionParams", vec![
                ActionArgDef::toggle("enabled", "Enabled").default_value(false),
                ActionArgDef::number("maxTracks", "Max Tracks").default_value(64),
                ActionArgDef::number("trackWindowPx", "Track Window (px)").default_value(21),
                ActionArgDef::slider("minTrackQuality", "Min Track Quality", 0.0, 1.0).default_value(0.3),
                ActionArgDef::number("minTrackLengthFrames", "Min Track Length (frames)").default_value(5),
            ])
            .operation("setGeoParams", "Set Geo Params")
            .action_args("setGeoParams", vec![
                ActionArgDef::toggle("enabled", "Enabled").default_value(false),
                ActionArgDef::number("originLon", "Origin Longitude").default_value(0),
                ActionArgDef::number("originLat", "Origin Latitude").default_value(0),
                ActionArgDef::number("originAlt", "Origin Altitude").default_value(0),
                ActionArgDef::slider("gsdM", "Ground Sample Distance (m)", 0.01, 1.0).default_value(0.05),
                ActionArgDef::slider("dsmCellM", "DSM Cell Size (m)", 0.01, 5.0).default_value(0.1),
                ActionArgDef::slider("dtmFilterRadiusM", "DTM Filter Radius (m)", 0.1, 10.0).default_value(2.0),
                ActionArgDef::number("orthoMaxPx", "Ortho Max (px)").default_value(4096),
            ])
            // 🧹️ Clear/reset.
            .operation("resetPlaceholderMesh", "Reset Placeholder Mesh")
            .operation("clearSparse", "Clear Sparse Cloud")
            .operation("clearDense", "Clear Dense Cloud")
            .operation("clearMeshResult", "Clear Mesh")
            .operation("clearTracks", "Clear Tracks")
            .operation("clearGeoProducts", "Clear Geo Products")
            .operation("clearResult", "Clear Result")
            // 👁️ View-only runtime actions.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setSelection", "Set Selection", ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", "Set Camera", ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setLayerVisibility", "Set Layer Visibility", ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setFrameCursor", "Set Frame Cursor", ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setReportTable", "Set Report Table", ActionKind::View) })
            // 📤️ Export.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new("exportQcReport", "Export QC Report", ActionKind::Shell) })
            // 🧰️ Utility groups — an exclusive per-window set (active utility is host-owned).
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", "Select", "mouse-pointer-2") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("sculpt", "Sculpt", "paintbrush") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("measure", "Measure", "scaling") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("gcpPlace", "Place GCP", "crosshair") })
            .window_kind_utilities(REMODEL_PLAY_WINDOW_MAIN, vec!["select".into(), "measure".into(), "sculpt".into()])
            .window_kind_utilities(REMODEL_PLAY_WINDOW_FRAMES, vec!["select".into(), "gcpPlace".into()]),
    )
    .example("default", "Default", &default_example)
    .workflow("remodel", "Remodel", "mesh")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp};

    //#region 🔖️Fixtures
    /// 🏁️ High-contrast `cell`-pixel checkerboard PNG, base64-wrapped as a `requestFileOpen(readAs:
    /// "dataUrl")` payload — mirrors `remodel_engine`'s own `checker_frame` test fixture, PNG-encoded so
    /// the plugin's `importFramePayload`/`runReconstruction` decode path is exercised for real.
    fn checker_data_url(w: u32, h: u32, cell: u32) -> String {
        let mut image = remodel_image::ImageRgba8::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let on = ((x / cell.max(1)) + (y / cell.max(1))).is_multiple_of(2);
                let v = if on { 235u8 } else { 20u8 };
                let idx = ((y * w + x) * 4) as usize;
                image.data[idx] = v;
                image.data[idx + 1] = v;
                image.data[idx + 2] = v;
                image.data[idx + 3] = 255;
            }
        }
        let bytes = remodel_image::encode_png(&image).expect("encode checker png");
        format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    /// 🏁️ The same checkerboard, real-JPEG-encoded — mirrors what a `RequestMediaFrames` host actually
    /// dispatches to `frame_action` (`payload: dataUrl(image/jpeg)`).
    fn checker_data_url_jpeg(w: u32, h: u32, cell: u32) -> String {
        let mut image = remodel_image::ImageRgba8::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let on = ((x / cell.max(1)) + (y / cell.max(1))).is_multiple_of(2);
                let v = if on { 235u8 } else { 20u8 };
                let idx = ((y * w + x) * 4) as usize;
                image.data[idx] = v;
                image.data[idx + 1] = v;
                image.data[idx + 2] = v;
                image.data[idx + 3] = 255;
            }
        }
        let bytes = remodel_image::encode_jpeg(&image, 90);
        format!("data:image/jpeg;base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    /// 🎞️ A tiny synthesized MJPEG-in-MP4 video (n frames of the same checker pattern) as a
    /// `RequestMediaFrames`-fallback-style raw base64 data URL payload.
    fn checker_video_data_url(n: u32, w: u32, h: u32, cell: u32) -> String {
        let mut image = remodel_image::ImageRgba8::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let on = ((x / cell.max(1)) + (y / cell.max(1))).is_multiple_of(2);
                let v = if on { 235u8 } else { 20u8 };
                let idx = ((y * w + x) * 4) as usize;
                image.data[idx] = v;
                image.data[idx + 1] = v;
                image.data[idx + 2] = v;
                image.data[idx + 3] = 255;
            }
        }
        let jpeg = remodel_image::encode_jpeg(&image, 90);
        let frames: Vec<Vec<u8>> = (0..n).map(|_| jpeg.clone()).collect();
        let bytes = remodel_video::write_mp4_mjpeg(&frames, 10.0);
        format!("data:video/mp4;base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    /// 📥️ Imports `n` checker frames as one new image-sequence stream via `importFramePayload`, mirroring
    /// exactly what a real `importFrames` → `RequestFileOpen.multiple` re-dispatch loop sends.
    fn import_checker_stream(app: &mut testkit_app::App, n: u32) {
        for index in 0..n {
            app.handle_action("importFramePayload", Some(&json!({ "payload": checker_data_url(24, 24, 3), "name": format!("frame-{index}.png"), "index": index, "total": n })), &ViewState::default(), &testkit::meta("local"))
                .expect("import frame payload");
        }
    }

    /// 🔁️ Drives `advanceReconstruction` until the job reaches a terminal stage (`Done`/`Failed`) or
    /// `max_ticks` is exhausted (returns `false` in that case — the caller decides whether that's a
    /// test failure).
    fn drive_to_terminal(app: &mut testkit_app::App, max_ticks: u32) -> bool {
        for _ in 0..max_ticks {
            let stage = app.projection().expect("projection").job.stage;
            if stage == ReconstructionStage::Done || stage == ReconstructionStage::Failed {
                return true;
            }
            app.handle_action("advanceReconstruction", None, &ViewState::default(), &testkit::meta("local")).expect("advance reconstruction");
        }
        matches!(app.projection().expect("projection").job.stage, ReconstructionStage::Done | ReconstructionStage::Failed)
    }

    /// 🧬️ Type alias so the fixtures above don't need to spell out `VcsDocumentApp<RemodelPlayApp>`.
    mod testkit_app {
        pub type App = semio_framework_plugin::VcsDocumentApp<super::RemodelPlayApp>;
    }
    //#endregion 🔖️Fixtures

    #[test]
    fn default_scene_seeds_the_world3d_mesh_json() {
        let scene = default_remodel_scene();
        assert!(world_meshes_json(&scene).contains(REMODEL_MESH_ID));
        let runtime = RemodelPlayRuntime::default();
        assert!(world_instances_json(&runtime).contains(REMODEL_MESH_ID));
    }

    /// 🖼️ Render smoke test: every window/panel body key this app declares must render without panicking.
    #[test]
    fn render_does_not_panic_for_known_body_keys() {
        let app = testkit::new_app::<RemodelPlayApp>();
        let store_projection = app.projection().expect("projection");
        let doc = DocumentView { projection: &store_projection, history: &semio_framework_plugin::HistoryView::empty() };
        let inner = RemodelPlayApp::default();
        for body_key in [
            REMODEL_PLAY_BODY_MAIN,
            REMODEL_PLAY_BODY_FRAMES,
            REMODEL_PLAY_BODY_REPORT,
            REMODEL_PLAY_BODY_MEDIA,
            REMODEL_PLAY_BODY_PIPELINE,
            REMODEL_PLAY_BODY_RESULTS,
            REMODEL_PLAY_BODY_PARAMETERS,
            REMODEL_PLAY_BODY_CALIBRATION,
            REMODEL_PLAY_BODY_TRACKS,
            REMODEL_PLAY_BODY_QC,
        ] {
            let _ = inner.render(body_key, &doc, &ViewState::default());
        }
    }

    #[test]
    fn clear_result_resets_all_seven_result_fields_and_reset_placeholder_restores_the_box() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        let result = app.handle_action("clearResult", None, &ViewState::default(), &testkit::meta("local")).expect("clear");
        assert_eq!(result.operations.len(), 7, "clearResult resets all 7 ReconstructionResults fields");
        assert_eq!(app.projection().expect("materialize projection").results.mesh.mesh.vertex_count(), 0);
        app.handle_action("resetPlaceholderMesh", None, &ViewState::default(), &testkit::meta("local")).expect("reset");
        assert_eq!(app.projection().expect("materialize projection").results.mesh.source, MeshSource::Placeholder);
        assert!(app.projection().expect("materialize projection").results.mesh.mesh.vertex_count() > 0);
    }

    #[test]
    fn view_actions_mutate_runtime_without_emitting_operations() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        let result = app.handle_action("setCamera", Some(&json!({ "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "fov": 60.0 })), &ViewState::default(), &testkit::meta("local")).expect("set camera");
        assert!(result.operations.is_empty());
        let result = app.handle_action("setLayerVisibility", Some(&json!({ "layer": "dense", "visible": false })), &ViewState::default(), &testkit::meta("local")).expect("set layer");
        assert!(result.operations.is_empty());
    }

    #[test]
    fn set_active_utility_switches_host_view_state_without_ops_or_history() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        let result = app.handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "measure" })), &ViewState::default(), &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switch is host-owned view state, never a document operation");
    }

    //#region 🔖️ArgFormTests
    #[test]
    fn set_sfm_params_arg_form_materializes_typed_args_into_operations() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        let result = app
            .handle_action("setSfmParams", Some(&json!({ "ransacIterations": 500, "ransacThresholdPx": 1.5, "minTrackLength": 4, "baMaxIterations": 20, "robustLoss": "cauchy", "huberDeltaPx": 2.5 })), &ViewState::default(), &testkit::meta("local"))
            .expect("set sfm params");
        assert_eq!(result.operations.len(), 1, "typed args produce one SetSfmParams operation");
        let params = app.projection().expect("materialize projection").params.sfm;
        assert_eq!(params.ransac_iterations, 500);
        assert_eq!(params.min_track_length, 4);
        assert_eq!(params.ba_max_iterations, 20);
        assert_eq!(params.robust_loss, RobustLossKind::Cauchy);
    }

    #[test]
    fn set_geo_params_arg_form_materializes_typed_args_into_operations() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        app.handle_action("setGeoParams", Some(&json!({ "enabled": true, "gsdM": 0.02, "dsmCellM": 0.2, "orthoMaxPx": 2048 })), &ViewState::default(), &testkit::meta("local")).expect("set geo params");
        let params = app.projection().expect("materialize projection").params.geo;
        assert!(params.enabled);
        assert_eq!(params.gsd_m, 0.02);
        assert_eq!(params.dsm_cell_m, 0.2);
        assert_eq!(params.ortho_max_px, 2048);
    }

    #[test]
    fn set_mesh_params_arg_form_materializes_watertight_knobs() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        app.handle_action("setMeshParams", Some(&json!({ "tsdfVoxelSizeMm": 3.0, "guaranteeWatertight": false, "holeFillMaxBoundaryVerts": 256, "selfIntersectionCheck": true })), &ViewState::default(), &testkit::meta("local"))
            .expect("set mesh params");
        let params = app.projection().expect("materialize projection").params.mesh;
        assert_eq!(params.tsdf_voxel_size_mm, 3.0);
        assert!(!params.guarantee_watertight);
        assert_eq!(params.hole_fill_max_boundary_verts, 256);
        assert!(params.self_intersection_check);
    }

    #[test]
    fn set_ingest_params_arg_form_materializes_min_sharpness() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        app.handle_action("setIngestParams", Some(&json!({ "minSharpness": 0.42 })), &ViewState::default(), &testkit::meta("local")).expect("set ingest params");
        assert_eq!(app.projection().expect("materialize projection").params.ingest.min_sharpness, 0.42);
    }
    //#endregion 🔖️ArgFormTests

    #[test]
    fn import_frame_payload_creates_a_stream_and_asset() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        import_checker_stream(&mut app, 3);
        let scene = app.projection().expect("projection");
        assert_eq!(scene.streams.len(), 1, "one importFrames batch creates exactly one stream");
        assert_eq!(scene.streams[0].frames.len(), 3);
        assert_eq!(scene.assets.len(), 3);
    }

    /// 🎞️ In-process video import (the `importVideoBytesPayload` fallback path): a tiny synthesized
    /// MJPEG mp4 must decode into a new video stream whose frame count matches what was muxed in.
    #[test]
    fn import_video_bytes_payload_extracts_frames_in_process() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        // 🎯️ `IngestParams::default().frame_sample_stride == 5`; force stride 1 so all 5 synthesized
        // frames are kept (a stride-sampling test belongs to `remodel_video`/`remodel_engine`, not here).
        app.handle_action("setIngestParams", Some(&json!({ "frameSampleStride": 1 })), &ViewState::default(), &testkit::meta("local")).expect("set ingest params");
        app.handle_action("importVideoBytesPayload", Some(&json!({ "payload": checker_video_data_url(5, 32, 32, 4), "name": "clip.mp4" })), &ViewState::default(), &testkit::meta("local")).expect("import video bytes");
        let scene = app.projection().expect("projection");
        assert_eq!(scene.streams.len(), 1);
        assert_eq!(scene.streams[0].kind, MediaKind::Video);
        assert_eq!(scene.streams[0].frames.len(), 5);
        assert_eq!(scene.assets.len(), 5);
    }

    /// 🎞️ Host-decoded video import path: `importVideoFramePayload` ticks followed by `importVideoDone`
    /// must accumulate into one stream and write `VideoSource` provenance, all under one coalesce key.
    #[test]
    fn import_video_frame_payload_then_done_writes_one_stream_with_video_source() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        for index in 0..4u32 {
            let payload = checker_data_url_jpeg(24, 24, 3);
            app.handle_action(
                "importVideoFramePayload",
                Some(&json!({ "payload": payload, "name": "clip.mp4", "index": index, "frameIndex": index, "timestampMs": f64::from(index) * 100.0 })),
                &ViewState::default(),
                &testkit::meta("local"),
            )
            .expect("import video frame payload");
        }
        app.handle_action("importVideoDone", Some(&json!({ "name": "clip.mp4", "durationMs": 400.0, "frameCount": 4, "width": 24, "height": 24, "codec": "mjpeg" })), &ViewState::default(), &testkit::meta("local")).expect("import video done");
        let scene = app.projection().expect("projection");
        assert_eq!(scene.streams.len(), 1);
        assert_eq!(scene.streams[0].kind, MediaKind::Video);
        assert!(scene.streams[0].source.is_some());
        assert_eq!(scene.streams[0].source.as_ref().unwrap().frame_count, 4);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        let placeholder_vertex_count = app.projection().expect("materialize projection").results.mesh.mesh.vertex_count();
        assert!(placeholder_vertex_count > 0, "the seeded placeholder box must have vertices");
        testkit::assert_undo_redo_round_trip(&mut app, "clearResult", None, |app| app.projection().expect("materialize projection").results.mesh.mesh.vertex_count(), placeholder_vertex_count, 0);
    }

    //#region 🔖️StagedReconstructionTests
    /// 🚀️ The staged execution model, end-to-end: `runReconstruction` ingests two imported checker
    /// frames into a fresh engine and schedules the first `advanceReconstruction` tick via
    /// `DispatchAction`; repeatedly feeding that tick back in (as the host's `requestedEffects` loop
    /// would) must reach a terminal `Done`/`Failed` stage.
    #[test]
    fn run_and_advance_reconstruction_reaches_a_terminal_stage() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        import_checker_stream(&mut app, 2);
        let run = app.handle_action("runReconstruction", None, &ViewState::default(), &testkit::meta("local")).expect("run reconstruction");
        assert_eq!(run.operations.len(), 1, "starting a run is one SetJob operation");
        assert!(matches!(run.requested_effects.first(), Some(HostEffect::DispatchAction { action, .. }) if action == "advanceReconstruction"), "runReconstruction must schedule the first advanceReconstruction tick: {:?}", run.requested_effects);
        assert_eq!(app.projection().expect("projection").job.stage, ReconstructionStage::Ingesting);

        let reached_terminal = drive_to_terminal(&mut app, 2000);
        assert!(reached_terminal, "a tiny 2-frame reconstruction must reach Done or Failed within 2000 ticks");
        let scene = app.projection().expect("projection");
        assert!(scene.job.stage == ReconstructionStage::Done || scene.job.stage == ReconstructionStage::Failed);
        if scene.job.stage == ReconstructionStage::Done {
            assert_eq!(scene.job.progress_0_1, 1.0);
            assert!(scene.results.sparse.is_some(), "a Done run publishes a sparse cloud");
        } else {
            assert!(scene.job.error.is_some(), "a Failed run must carry an error message");
        }
    }

    /// 🛑️ Cancelling mid-run must fold into the SAME undo entry the run itself started (both amends
    /// share the `remodel-reconstruction:{jobId}` coalesce key), clear the runtime engine, and leave
    /// the job in a non-running `Idle` state with an explanatory error.
    #[test]
    fn cancel_mid_run_clears_engine_and_marks_job_idle() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        import_checker_stream(&mut app, 2);
        app.handle_action("runReconstruction", None, &ViewState::default(), &testkit::meta("local")).expect("run reconstruction");
        // 🎯️ Cancel immediately after the run starts, before any `advanceReconstruction` tick: the tiny
        // 2-frame checker fixture can reach a terminal stage (and clear `runtime.engine`) within a single
        // tick under `RECONSTRUCTION_STEP_BUDGET`, which would make cancellation a no-operation — cancelling
        // right after start is the only tick-count-independent way to exercise "mid-run" cancellation.
        app.handle_action("cancelReconstruction", None, &ViewState::default(), &testkit::meta("local")).expect("cancel");
        assert!(app.projection().expect("projection").job.cancel_requested);

        app.handle_action("advanceReconstruction", None, &ViewState::default(), &testkit::meta("local")).expect("cancelling tick");
        let scene = app.projection().expect("projection");
        assert_eq!(scene.job.stage, ReconstructionStage::Idle);
        assert!(scene.job.error.is_some());
    }

    /// 🔁️ `retryStage` after a run reached a terminal stage starts a fresh run (a new job id, back at
    /// `Ingesting`).
    #[test]
    fn retry_stage_starts_a_fresh_run_after_a_terminal_stage() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        import_checker_stream(&mut app, 2);
        app.handle_action("runReconstruction", None, &ViewState::default(), &testkit::meta("local")).expect("run reconstruction");
        assert!(drive_to_terminal(&mut app, 2000), "must terminate");
        let first_job_id = app.projection().expect("projection").job.id;

        app.handle_action("retryStage", Some(&json!({ "stage": "extracting-features" })), &ViewState::default(), &testkit::meta("local")).expect("retry stage");
        let scene = app.projection().expect("projection");
        assert_eq!(scene.job.stage, ReconstructionStage::Ingesting);
        assert_ne!(scene.job.id, first_job_id, "retryStage must start a new job");
    }

    /// 📦️ The whole run — start tick, every working tick, and the terminal tick — must collapse into
    /// exactly one undo step: undoing once after a run reaches `Done`/`Failed` must fully revert the
    /// job (and any published results) back to the pristine pre-run document. This directly verifies
    /// the CRDT/undo contract described on `handle_advance_reconstruction`.
    #[test]
    fn full_run_collapses_into_a_single_undo_step() {
        let mut app = testkit::new_app::<RemodelPlayApp>();
        import_checker_stream(&mut app, 2);
        let before_job = app.projection().expect("projection").job;
        app.handle_action("runReconstruction", None, &ViewState::default(), &testkit::meta("local")).expect("run reconstruction");
        assert!(drive_to_terminal(&mut app, 2000), "reconstruction must terminate");
        assert_ne!(app.projection().expect("projection").job, before_job, "run must have changed the job");

        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").job, before_job, "one undo must fully revert the coalesced run");
    }
    //#endregion 🔖️StagedReconstructionTests

    /// 🧪️ The definitional proof: two independent instances start from the same document, apply
    /// DISJOINT field edits (A tunes feature params, B adds a ground control point), and exchanging operations
    /// over a `MemoryBackbone` converges both sides to contain BOTH edits — impossible under a
    /// whole-document `setDocument` snapshot, where one side's write would clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<RemodelPlayApp, _>(
            "mem://remodel-convergence",
            ("setFeatureParams", Some(&json!({ "detector": "akaze", "targetCount": 1000 }))),
            ("addGcp", Some(&json!({ "name": "corner", "worldX": 1.0, "worldY": 2.0, "worldZ": 3.0 }))),
            |app| {
                let projection = app.projection().expect("materialize projection");
                (projection.params.feature.detector, projection.gcps.first().map(|gcp| gcp.name.clone()))
            },
        );
    }
}
//#endregion 🧪️Tests
