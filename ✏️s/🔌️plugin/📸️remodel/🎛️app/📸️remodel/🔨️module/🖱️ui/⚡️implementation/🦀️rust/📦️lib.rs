//! 🏺️ Remodel app — DocumentApp impl, render, manifest (constitutional: ui). B1: full pure-trait
//! conversion (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE, config + port recipe),
//! mirroring `shooting_ui`'s pilot pattern — `RemodelPlayApp` is a unit struct; every former
//! `RemodelPlayRuntime` field (camera/selection/layers/frame cursor/report table) now lives in
//! `remodel_app_engine::RemodelConfig`, written via `remodel_op::RemodelConfigOperation`s (real
//! `backwards`, no ad hoc runtime mutation); every action dispatches through the single typed
//! `remodel_protocol::RemodelCommand` channel via `DocumentApp::handle`.
//!
//! 🚀️ The former cooperative, multi-tick `runReconstruction` → `advanceReconstruction` (host
//! `DispatchAction` re-dispatch loop, `RefCell<Option<remodel_engine::ReconstructionEngine>>` runtime
//! state) cannot survive B1: `handle` is `&self` and pure — there is no legal place left to park a live,
//! non-`Clone`/non-`Serialize` compute engine between calls, and no legal way for one call to "not
//! finish yet" and be resumed by a later, unrelated call. `RunReconstruction`/`RetryStage`/`RunStage`
//! therefore run the WHOLE staged pipeline synchronously inside one `handle()` call (bounded by
//! `REMODEL_MAX_RECONSTRUCTION_TICKS`, a pure-function totality safety valve, not a real-world limit).
//! `AdvanceReconstruction`/`CancelReconstruction` are deleted: there is nothing to advance or cancel once
//! a single call runs to completion before returning. Multi-tick `importVideoFramePayload` batches keep
//! working: the blur-gate's rolling sharpness window is rebuilt each tick from the already-persisted
//! stream's most recent frames (see `rebuild_video_import_scratch`) instead of carried in runtime scratch.

use base64::Engine as _;
use remodel::{
    default_remodel_scene, CameraCalibration, CameraPosePreview, CameraTrajectory, DenseParams, DenseResolution, FeatureDetector, FeatureParams, FrameRef, GcpObservation, GeoParams, GeoProducts, GroundControlPoint, ImageAsset, IngestParams,
    MatchParams, MatcherKind, MediaKind, MediaStream, MeshParams as DocumentMeshParams, MeshSource, MotionParams, PackedF32, ReconstructionJob, ReconstructionStage, RemodelMesh, RemodelScene, RobustLossKind, SfmParams, SparseCloud, VideoSource,
    REMODEL_DOCUMENT_SCHEMA,
};
use remodel_app_engine::{RemodelConfig, RemodelFrameCursor, RemodelLayerVisibility};
use remodel_op::{RemodelConfigOperation, RemodelOperation};
use remodel_protocol::RemodelCommand;
use semio_framework_plugin::{
        app_labels, build_canvas_2d_scene, build_table_scene, build_world_3d_scene, create_default_layout, create_named_layout, mesh_from_kind, ui_import_drop_zone, ui_stack_vertical, ui_text, world3d_camera_json, world3d_scene, world3d_selection_json,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppIo, AppLabels, ArtifactKindSpec, Canvas2dScene, ConfigView, DocumentApp, DocumentView, Emit, GlbExporter, HostEffect, Label, LabelText, Locale,
    LocalizedLabel, Media, MediaClass, MediaError, MediaPayload, MediaType, MeshData, MeshExporter, OsMediaCapability, OsMediaFormat, PanelGroup, SurfaceKind, TableScene, Terminology, UiNode, UtilityCategory, UtilityDefinition, WorldSunConfig,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, SET_ACTIVE_UTILITY_ACTION_ID,
};
use serde_json::{json, Value};
use std::collections::VecDeque;
use store::{DocumentDsl, DocumentPack};

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
/// ⚙️ Bounded units of engine work performed per internal `advance()` call within one synchronous
/// `RunReconstruction` — small enough that no single `advance` call does an unreasonable burst of work.
const RECONSTRUCTION_STEP_BUDGET: usize = 8;
/// 🛑️ Pure-function totality safety valve for the synchronous reconstruction loop (see the module doc
/// comment): a real project's total ticks is bounded by its frame/point/triangle counts, never this —
/// this only guards against an engine bug spinning `handle()` forever.
const REMODEL_MAX_RECONSTRUCTION_TICKS: u32 = 200_000;
/// 📥️ The drop zone's accepted extensions: still-image formats plus every container `remodel_video`
/// can probe (decode is attempted in-process; an undecodable codec still records provenance).
const REMODEL_MEDIA_ACCEPT: &str = "image/png,image/jpeg,video/mp4,video/quicktime,video/webm,video/x-msvideo,.png,.jpg,.jpeg,.mp4,.mov,.webm,.avi";
const REMODEL_VIDEO_ACCEPT: &str = "video/mp4,video/quicktime,video/webm,video/x-msvideo,.mp4,.mov,.webm,.avi";
/// 🔌️ Well-known stream id every `photos:in` import lands on — a stable identity so successive workflow
/// imports keep appending frames to the SAME stream (a pure `import_media` call has no runtime scratch
/// to remember "which stream did the last call use", unlike a UI drag-drop batch's `index == 0`/`> 0`
/// convention — see `RemodelCommand::ImportFramePayload`'s handler for that one).
const REMODEL_WORKFLOW_PHOTOS_STREAM_ID: &str = "workflow-photos";
//#endregion 🔖️Constants

//#region 🔖️VideoImportScratch
/// 📥️ Rolling blur-gate scratch for one in-progress `importVideoFramePayload`/`importVideoBytesPayload`
/// batch — mirrors `remodel_engine::FrameSource`'s own relative-sharpness gate (not reusable directly:
/// that gate lives inside a whole `FrameSource`, this one only needs the rolling-median scratch itself).
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

/// 🧩️ Pure reconstruction of the blur-gate rolling window from `stream_id`'s already-persisted frames
/// (most recent `BLUR_GATE_ROLLING_WINDOW` first, then scored oldest-to-newest so the window fills in
/// the same order the original per-tick `RefCell` scratch would have) — the B1 replacement for carrying
/// `VideoImportScratch` as hidden interior-mutable runtime state across `ImportVideoFramePayload` ticks.
fn rebuild_video_import_scratch(scene: &RemodelScene, stream_id: &str) -> VideoImportScratch {
    let mut scratch = VideoImportScratch::default();
    let Some(stream) = scene.streams.iter().find(|stream| stream.id == stream_id) else { return scratch };
    let mut recent: Vec<&FrameRef> = stream.frames.iter().rev().take(BLUR_GATE_ROLLING_WINDOW).collect();
    recent.reverse();
    for frame in recent {
        let Some(asset) = scene.assets.get(&frame.asset_id) else { continue };
        let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&asset.data) else { continue };
        let Ok(image) = decode_still_image(&asset.mime, &bytes) else { continue };
        scratch.rolling_scores.push_back(local_sharpness_score(&image));
    }
    scratch
}
//#endregion 🔖️VideoImportScratch

//#region 🔖️DocumentHelpers
fn world_meshes_json(scene: &RemodelScene) -> String {
    serde_json::to_string(&vec![json!({ "id": REMODEL_MESH_ID, "data": scene.results.mesh.mesh })]).unwrap_or_else(|_| "[]".into())
}

fn world_instances_json(config: &RemodelConfig) -> String {
    if !config.layers.mesh {
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
/// them, and every recovered camera pose as its own (small, unattenuated) point layer — a documented
/// simplification standing in for a real camera-frustum gizmo, which `points_json` alone cannot express.
/// Gcp world positions are a fourth, always-static layer. The former in-progress live sparse preview
/// layer is gone along with the multi-tick run (see module doc comment): `RunReconstruction` now only
/// ever publishes the FINAL sparse cloud, never an interior one. `PackedF32`/`PackedU8`'s inner string
/// is already a base64 little-endian buffer, matching `positionsB64`/`colorsB64`'s wire shape
/// byte-for-byte — no decode/re-encode round trip needed.
fn world_points_json(scene: &RemodelScene, config: &RemodelConfig) -> Option<String> {
    let mut layers: Vec<Value> = Vec::new();
    if config.layers.sparse {
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
    }
    if config.layers.dense {
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
    if config.layers.cameras && !scene.job.camera_poses_preview.is_empty() {
        let positions: Vec<f32> = scene.job.camera_poses_preview.iter().flat_map(|pose| pose.translation).collect();
        layers.push(json!({
            "id": "remodel-camera-poses",
            "positionsB64": PackedF32::from_f32_slice(&positions).0,
            "colorsB64": Value::Null,
            "size": 9.0,
            "sizeAttenuation": false,
        }));
    }
    if config.layers.gcps && !scene.gcps.is_empty() {
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
            vec![json!({ "id": "id", "label": "Id" }), json!({ "id": "length", "label": "Length" }), json!({ "id": "class", "label": "Class" }), json!({ "id": "speed", "label": "Mean Speed (m/s)" })],
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
        "qcStages" => (vec![json!({ "id": "stage", "label": "Stage" }), json!({ "id": "status", "label": "Status" })], vec![json!({ "stage": format!("{:?}", scene.job.stage), "status": if scene.job.error.is_some() { "error" } else { "ok" } })]),
        "matches" => (vec![json!({ "id": "note", "label": "Note" })], vec![json!({ "note": "Pairwise match data is reconstruction-runtime scratch, never distilled into durable document state." })]),
        _ => (
            vec![json!({ "id": "streamId", "label": "Stream" }), json!({ "id": "index", "label": "Index" }), json!({ "id": "timestampMs", "label": "Timestamp (ms)" }), json!({ "id": "assetId", "label": "Asset" })],
            scene.streams.iter().flat_map(|stream| stream.frames.iter().map(move |frame| json!({ "streamId": stream.id, "index": frame.index, "timestampMs": frame.timestamp_ms, "assetId": frame.asset_id }))).collect(),
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
        model: native_en "Model", native_de "Modell", reuse_en "Model", reuse_de "Modell";
        capture: native_en "Capture", native_de "Aufnahme", reuse_en "Capture", reuse_de "Aufnahme";
        analyze: native_en "Analyze", native_de "Analyse", reuse_en "Analyze", reuse_de "Analyse";
        default_example: native_en "Default", native_de "Standard", reuse_en "Default", reuse_de "Standard";
        reconstruction: native_en "Reconstruction", native_de "Rekonstruktion", reuse_en "Reconstruction", reuse_de "Rekonstruktion";
        error: native_en "error", native_de "Fehler", reuse_en "error", reuse_de "Fehler";
        status: native_en "Status", native_de "Status", reuse_en "Status", reuse_de "Status";
        running: native_en "Running", native_de "Läuft", reuse_en "Running", reuse_de "Läuft";
        idle: native_en "Idle", native_de "Leerlauf", reuse_en "Idle", reuse_de "Leerlauf";
        utility: native_en "Utility", native_de "Werkzeug", reuse_en "Utility", reuse_de "Werkzeug";
        selection: native_en "selection", native_de "Auswahl", reuse_en "selection", reuse_de "Auswahl";
        mesh: native_en "Mesh", native_de "Mesh", reuse_en "Mesh", reuse_de "Mesh";
        vertices: native_en "vertices", native_de "Vertices", reuse_en "vertices", reuse_de "Vertices";
        triangles: native_en "triangles", native_de "Dreiecke", reuse_en "triangles", reuse_de "Dreiecke";
        streams: native_en "Streams", native_de "Streams", reuse_en "Streams", reuse_de "Streams";
        assets: native_en "Assets", native_de "Assets", reuse_en "Assets", reuse_de "Assets";
        no_streams: native_en "No media streams imported yet", native_de "Noch keine Medien-Streams importiert", reuse_en "No media streams imported yet", reuse_de "Noch keine Medien-Streams importiert";
        stream_kind_video: native_en "video", native_de "Video", reuse_en "video", reuse_de "Video";
        stream_kind_image_sequence: native_en "image sequence", native_de "Bildsequenz", reuse_en "image sequence", reuse_de "Bildsequenz";
        frames: native_en "frames", native_de "Frames", reuse_en "frames", reuse_de "Frames";
        sync_offset: native_en "sync offset", native_de "Sync-Versatz", reuse_en "sync offset", reuse_de "Sync-Versatz";
        sparse_cloud: native_en "Sparse point cloud", native_de "Dünne Punktwolke", reuse_en "Sparse point cloud", reuse_de "Dünne Punktwolke";
        dense_cloud: native_en "Dense point cloud", native_de "Dichte Punktwolke", reuse_en "Dense point cloud", reuse_de "Dichte Punktwolke";
        results_none: native_en "none", native_de "keine", reuse_en "none", reuse_de "keine";
        trajectory: native_en "Trajectory", native_de "Trajektorie", reuse_en "Trajectory", reuse_de "Trajektorie";
        poses: native_en "poses", native_de "Posen", reuse_en "poses", reuse_de "Posen";
        geo_products: native_en "Geo products", native_de "Geo-Produkte", reuse_en "Geo products", reuse_de "Geo-Produkte";
        available: native_en "available", native_de "verfügbar", reuse_en "available", reuse_de "verfügbar";
        params_ingest: native_en "Ingest", native_de "Ingest", reuse_en "Ingest", reuse_de "Ingest";
        params_feature: native_en "Feature", native_de "Feature", reuse_en "Feature", reuse_de "Feature";
        params_matching: native_en "Matching", native_de "Matching", reuse_en "Matching", reuse_de "Matching";
        params_sfm: native_en "SfM", native_de "SfM", reuse_en "SfM", reuse_de "SfM";
        params_dense: native_en "Dense", native_de "Dense", reuse_en "Dense", reuse_de "Dense";
        params_mesh: native_en "Mesh", native_de "Mesh", reuse_en "Mesh", reuse_de "Mesh";
        params_motion: native_en "Motion", native_de "Bewegung", reuse_en "Motion", reuse_de "Bewegung";
        params_geo: native_en "Geo", native_de "Geo", reuse_en "Geo", reuse_de "Geo";
        stride_short: native_en "stride", native_de "Schrittweite", reuse_en "stride", reuse_de "Schrittweite";
        max_short: native_en "max", native_de "max", reuse_en "max", reuse_de "max";
        downscale_short: native_en "downscale", native_de "Verkleinerung", reuse_en "downscale", reuse_de "Verkleinerung";
        target_short: native_en "target", native_de "Ziel", reuse_en "target", reuse_de "Ziel";
        octaves_short: native_en "octaves", native_de "Oktaven", reuse_en "octaves", reuse_de "Oktaven";
        ratio_short: native_en "ratio", native_de "Verhältnis", reuse_en "ratio", reuse_de "Verhältnis";
        window_short: native_en "window", native_de "Fenster", reuse_en "window", reuse_de "Fenster";
        ransac_short: native_en "ransac", native_de "Ransac", reuse_en "ransac", reuse_de "Ransac";
        min_track_short: native_en "min track", native_de "min. Spur", reuse_en "min track", reuse_de "min. Spur";
        ba_short: native_en "ba", native_de "BA", reuse_en "ba", reuse_de "BA";
        voxel_short: native_en "voxel", native_de "Voxel", reuse_en "voxel", reuse_de "Voxel";
        enabled: native_en "enabled", native_de "aktiviert", reuse_en "enabled", reuse_de "aktiviert";
        disabled: native_en "disabled", native_de "deaktiviert", reuse_en "disabled", reuse_de "deaktiviert";
        cameras_calibrated: native_en "Calibrated cameras", native_de "Kalibrierte Kameras", reuse_en "Calibrated cameras", reuse_de "Kalibrierte Kameras";
        rig_extrinsics: native_en "Rig extrinsics", native_de "Rig-Extrinsik", reuse_en "Rig extrinsics", reuse_de "Rig-Extrinsik";
        gcps: native_en "Ground control points", native_de "Passpunkte", reuse_en "Ground control points", reuse_de "Passpunkte";
        tracks: native_en "Motion tracks", native_de "Bewegungsspuren", reuse_en "Motion tracks", reuse_de "Bewegungsspuren";
        tracks_none: native_en "No motion tracks", native_de "Keine Bewegungsspuren", reuse_en "No motion tracks", reuse_de "Keine Bewegungsspuren";
        motion_not_implemented: native_en "Motion tracking is not yet driven by the reconstruction engine", native_de "Bewegungsverfolgung wird von der Rekonstruktions-Engine noch nicht ausgeführt", reuse_en "Motion tracking is not yet driven by the reconstruction engine", reuse_de "Bewegungsverfolgung wird von der Rekonstruktions-Engine noch nicht ausgeführt";
        qc_none: native_en "No quality report yet", native_de "Noch kein Qualitätsbericht", reuse_en "No quality report yet", reuse_de "Noch kein Qualitätsbericht";
        qc_reprojection: native_en "Mean reprojection error", native_de "Mittlerer Reprojektionsfehler", reuse_en "Mean reprojection error", reuse_de "Mittlerer Reprojektionsfehler";
        qc_track_length: native_en "Mean track length", native_de "Mittlere Spurlänge", reuse_en "Mean track length", reuse_de "Mittlere Spurlänge";
        qc_registered_ratio: native_en "Registered frame ratio", native_de "Anteil registrierter Frames", reuse_en "Registered frame ratio", reuse_de "Anteil registrierter Frames";
        qc_dense_coverage: native_en "Dense coverage ratio", native_de "Dense-Abdeckungsanteil", reuse_en "Dense coverage ratio", reuse_de "Dense-Abdeckungsanteil";
        qc_gcp_rmse: native_en "GCP checkpoint RMSE", native_de "Passpunkt-Kontroll-RMSE", reuse_en "GCP checkpoint RMSE", reuse_de "Passpunkt-Kontroll-RMSE";
        qc_watertight: native_en "Watertight", native_de "Wasserdicht", reuse_en "Watertight", reuse_de "Wasserdicht";
        qc_boundary_edges: native_en "Boundary edges", native_de "Ränder", reuse_en "Boundary edges", reuse_de "Ränder";
        qc_components: native_en "Connected components", native_de "Zusammenhangskomponenten", reuse_en "Connected components", reuse_de "Zusammenhangskomponenten";
        qc_euler: native_en "Euler characteristic", native_de "Euler-Charakteristik", reuse_en "Euler characteristic", reuse_de "Euler-Charakteristik";
        qc_genus: native_en "Genus", native_de "Genus", reuse_en "Genus", reuse_de "Genus";
        qc_closed_fallback: native_en "Closed via fallback", native_de "Über Fallback geschlossen", reuse_en "Closed via fallback", reuse_de "Über Fallback geschlossen";
        panel_media: native_en "Media", native_de "Medien", reuse_en "Media", reuse_de "Medien";
        panel_pipeline: native_en "Pipeline", native_de "Pipeline", reuse_en "Pipeline", reuse_de "Pipeline";
        panel_results: native_en "Results", native_de "Ergebnisse", reuse_en "Results", reuse_de "Ergebnisse";
        panel_parameters: native_en "Parameters", native_de "Parameter", reuse_en "Parameters", reuse_de "Parameter";
        panel_calibration: native_en "Calibration", native_de "Kalibrierung", reuse_en "Calibration", reuse_de "Kalibrierung";
        panel_tracks: native_en "Tracks", native_de "Spuren", reuse_en "Tracks", reuse_de "Spuren";
        panel_qc: native_en "Quality", native_de "Qualität", reuse_en "Quality", reuse_de "Qualität";
        window_frames: native_en "Frames", native_de "Frames", reuse_en "Frames", reuse_de "Frames";
        window_report: native_en "Report", native_de "Bericht", reuse_en "Report", reuse_de "Bericht";
        layers: native_en "Layers", native_de "Ebenen", reuse_en "Layers", reuse_de "Ebenen";
        layer_mesh: native_en "Mesh", native_de "Mesh", reuse_en "Mesh", reuse_de "Mesh";
        layer_dense: native_en "Dense cloud", native_de "Dichte Punktwolke", reuse_en "Dense cloud", reuse_de "Dichte Punktwolke";
        layer_sparse: native_en "Sparse cloud", native_de "Dünne Punktwolke", reuse_en "Sparse cloud", reuse_de "Dünne Punktwolke";
        layer_cameras: native_en "Cameras", native_de "Kameras", reuse_en "Cameras", reuse_de "Kameras";
        layer_gcps: native_en "GCPs", native_de "Passpunkte", reuse_en "GCPs", reuse_de "Passpunkte";
    }
}



fn remodel_labels(cfg: &RemodelConfig) -> &'static RemodelLabels {
    semio_framework_plugin::resolve_labels_for_locale::<RemodelLabels>(&cfg.locale)
}
//#endregion 🔖️Terminology

//#region 🔖️PanelBuilders
/// 🗂️ `remodel.media` — drop zone plus a summary line per imported stream/asset.
fn build_media_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let mut lines = vec![
        ui_import_drop_zone("remodel-media-drop", labels.panel_media.into(), labels.no_streams.into(), Some(REMODEL_MEDIA_ACCEPT), remodel_action("importFramePayload", None)),
        ui_text(Label::data(format!("{}: {} - {}: {}", labels.streams.as_str(), scene.streams.len(), labels.assets.as_str(), scene.assets.len()))),
    ];
    for stream in &scene.streams {
        let kind_label = match stream.kind {
            MediaKind::Video => labels.stream_kind_video,
            MediaKind::ImageSequence => labels.stream_kind_image_sequence,
        };
        lines.push(ui_text(Label::data(format!("{} ({}, {} {}, {}: {:.1}ms)", stream.name, kind_label.as_str(), stream.frames.len(), labels.frames.as_str(), labels.sync_offset.as_str(), stream.sync_offset_ms))));
        if let Some(source) = &stream.source {
            lines.push(ui_text(Label::data(format!("  {:?} {}x{} {:.0}ms", source.codec, source.width, source.height, source.duration_ms))));
        }
    }
    ui_stack_vertical(lines)
}

/// 🚦️ `remodel.pipeline` — job status/progress plus live viewport session state. `running` is derived
/// from the persisted job stage now (not a `RefCell` engine handle — see module doc comment): a
/// synchronous run never leaves the document in a non-terminal stage, so this is effectively always
/// "Idle" once a run finishes, which is the documented, accepted trade-off of the B1 conversion.
fn build_pipeline_panel(scene: &RemodelScene, config: &RemodelConfig, active_utility: &str, labels: &RemodelLabels) -> UiNode {
    let job = &scene.job;
    let job_label =
        format!("{}: {} ({:.0}%){}", labels.reconstruction.as_str(), remodel_app_engine::stage_display(job.stage), job.progress_0_1 * 100.0, job.error.as_ref().map(|error| format!(" - {}: {error}", labels.error.as_str())).unwrap_or_default());
    let running = !matches!(job.stage, ReconstructionStage::Idle | ReconstructionStage::Done | ReconstructionStage::Failed);
    let running_label = format!("{}: {}", labels.status.as_str(), if running { labels.running.as_str() } else { labels.idle.as_str() });
    let utility_label = format!("{}: {} - {}: {} ({})", labels.utility.as_str(), active_utility, labels.selection.as_str(), config.selection.mode, config.selection.ids.len());
    ui_stack_vertical(vec![ui_text(Label::data(job_label)), ui_text(Label::data(running_label)), ui_text(Label::data(utility_label))])
}

/// 🧵️ `remodel.results` — the products a run (partially) produced.
fn build_results_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let results = &scene.results;
    let mesh_label = format!("{}: {:?}, {} {}, {} {}", labels.mesh.as_str(), results.mesh.source, results.mesh.mesh.vertex_count(), labels.vertices.as_str(), results.mesh.mesh.triangle_count(), labels.triangles.as_str());
    let sparse_label = results.sparse.as_ref().map_or_else(|| format!("{}: {}", labels.sparse_cloud.as_str(), labels.results_none.as_str()), |sparse| format!("{}: {}", labels.sparse_cloud.as_str(), sparse.points.to_f32_vec().len() / 3));
    let dense_label = results.dense.as_ref().map_or_else(|| format!("{}: {}", labels.dense_cloud.as_str(), labels.results_none.as_str()), |dense| format!("{}: {}", labels.dense_cloud.as_str(), dense.positions.to_f32_vec().len() / 3));
    let trajectory_label =
        results.trajectory.as_ref().map_or_else(|| format!("{}: {}", labels.trajectory.as_str(), labels.results_none.as_str()), |trajectory| format!("{}: {} {}", labels.trajectory.as_str(), trajectory.poses.len(), labels.poses.as_str()));
    let geo_label = results.geo.as_ref().map_or_else(|| format!("{}: {}", labels.geo_products.as_str(), labels.results_none.as_str()), |_| format!("{}: {}", labels.geo_products.as_str(), labels.available.as_str()));
    ui_stack_vertical(vec![ui_text(Label::data(mesh_label)), ui_text(Label::data(sparse_label)), ui_text(Label::data(dense_label)), ui_text(Label::data(trajectory_label)), ui_text(Label::data(geo_label))])
}

/// ⚙️ `remodel.parameters` — a read-only dump of the 8 param sub-groups (editing happens via the
/// per-group `setXParams` command-palette actions' typed arg forms, not inline fields here).
fn build_parameters_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let p = &scene.params;
    ui_stack_vertical(vec![
        ui_text(Label::data(format!(
            "{}: {} {}, {} {}, {} {}px, min sharpness {:.2}",
            labels.params_ingest.as_str(),
            labels.stride_short.as_str(),
            p.ingest.frame_sample_stride,
            labels.max_short.as_str(),
            p.ingest.max_frames,
            labels.downscale_short.as_str(),
            p.ingest.downscale_long_edge_px,
            p.ingest.min_sharpness
        ))),
        ui_text(Label::data(format!("{}: {:?}, {} {}, {} {}", labels.params_feature.as_str(), p.feature.detector, labels.target_short.as_str(), p.feature.target_count, labels.octaves_short.as_str(), p.feature.octaves))),
        ui_text(Label::data(format!("{}: {:?}, {} {:.2}, {} {}", labels.params_matching.as_str(), p.matching.matcher, labels.ratio_short.as_str(), p.matching.ratio_test, labels.window_short.as_str(), p.matching.sequential_window))),
        ui_text(Label::data(format!(
            "{}: {} {}, {} {}, {} {}",
            labels.params_sfm.as_str(),
            labels.ransac_short.as_str(),
            p.sfm.ransac_iterations,
            labels.min_track_short.as_str(),
            p.sfm.min_track_length,
            labels.ba_short.as_str(),
            p.sfm.ba_max_iterations
        ))),
        ui_text(Label::data(format!("{}: {:?}, {} {}px", labels.params_dense.as_str(), p.dense.resolution, labels.window_short.as_str(), p.dense.window_radius_px))),
        ui_text(Label::data(format!(
            "{}: {} {:.1}mm, {} {}, watertight {}",
            labels.params_mesh.as_str(),
            labels.voxel_short.as_str(),
            p.mesh.tsdf_voxel_size_mm,
            labels.target_short.as_str(),
            p.mesh.decimate_target_triangles,
            p.mesh.guarantee_watertight
        ))),
        ui_text(Label::data(format!("{}: {}", labels.params_motion.as_str(), if p.motion.enabled { labels.enabled.as_str() } else { labels.disabled.as_str() }))),
        ui_text(Label::data(format!("{}: {}", labels.params_geo.as_str(), if p.geo.enabled { labels.enabled.as_str() } else { labels.disabled.as_str() }))),
    ])
}

/// 🎯️ `remodel.calibration` — per-camera calibration, rig extrinsics, ground control points.
fn build_calibration_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let mut lines = vec![ui_text(Label::data(format!("{}: {} - {}: {}", labels.cameras_calibrated.as_str(), scene.calibration.cameras.len(), labels.rig_extrinsics.as_str(), scene.calibration.rig.len())))];
    for camera in &scene.calibration.cameras {
        lines.push(ui_text(Label::data(format!("{} ({}): fx {:.1} fy {:.1}", camera.label, camera.model, camera.fx, camera.fy))));
    }
    lines.push(ui_text(Label::data(format!("{}: {}", labels.gcps.as_str(), scene.gcps.len()))));
    for gcp in &scene.gcps {
        lines.push(ui_text(Label::data(format!("{} [{:.2}, {:.2}, {:.2}] ({} obs)", gcp.name, gcp.world_position[0], gcp.world_position[1], gcp.world_position[2], gcp.observations.len()))));
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
    let mut lines = vec![ui_text(Label::data(format!("{}: {}", labels.tracks.as_str(), scene.results.tracks.len())))];
    for track in &scene.results.tracks {
        lines.push(ui_text(Label::data(format!("{} ({:?}): {} frames, {:.2} m/s", track.id, track.class, track.length, track.mean_speed_m_s))));
    }
    ui_stack_vertical(lines)
}

/// ✅️ `remodel.qc` — the whole-run quality report, including the watertight sub-report.
fn build_qc_panel(scene: &RemodelScene, labels: &RemodelLabels) -> UiNode {
    let Some(qc) = &scene.results.qc else {
        return ui_stack_vertical(vec![ui_text(labels.qc_none)]);
    };
    let mut lines = vec![
        ui_text(Label::data(format!("{}: {:.2}px", labels.qc_reprojection.as_str(), qc.reprojection_rms_px))),
        ui_text(Label::data(format!("{}: {:.1}", labels.qc_track_length.as_str(), qc.mean_track_length))),
        ui_text(Label::data(format!("{}: {:.0}%", labels.qc_registered_ratio.as_str(), qc.registered_frame_ratio * 100.0))),
        ui_text(Label::data(format!("{}: {:.0}%", labels.qc_dense_coverage.as_str(), qc.dense_coverage_ratio * 100.0))),
    ];
    if let Some(rmse) = qc.gcp_checkpoint_rmse {
        lines.push(ui_text(Label::data(format!("{}: {:.3}m", labels.qc_gcp_rmse.as_str(), rmse))));
    }
    if let Some(watertight) = &qc.watertight {
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_watertight.as_str(), watertight.is_watertight))));
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_boundary_edges.as_str(), watertight.boundary_edge_count))));
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_components.as_str(), watertight.connected_components))));
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_euler.as_str(), watertight.euler_characteristic))));
        if let Some(genus) = watertight.genus {
            lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_genus.as_str(), genus))));
        }
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_closed_fallback.as_str(), watertight.closed_fallback_used))));
    }
    for warning in &qc.warnings {
        lines.push(ui_text(Label::data(format!("⚠️ {warning}"))));
    }
    ui_stack_vertical(lines)
}
//#endregion 🔖️PanelBuilders

//#region 🔖️RemodelPlayApp
fn remodel_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: REMODEL_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

/// 🧪️ B1: unit struct — every former `RemodelPlayRuntime`/`self.runtime` field now lives in
/// `remodel_app_engine::RemodelConfig` (see `DocumentApp::Config`), written through
/// `remodel_op::RemodelConfigOperation`s. See the module doc comment for how the former live
/// `ReconstructionEngine`/`VideoImportScratch` runtime state is handled without a `RefCell`.
#[derive(Default)]
pub struct RemodelPlayApp;

impl RemodelPlayApp {
    //#region 🔖️Ingestion
    /// 📥️ `importFramePayload` — a still-image drop-zone/file-picker payload. `index == 0` starts a new
    /// stream (id minted via `next_remodel_id`); `index > 0` appends to `doc.projection.streams.last()`
    /// (the stream THIS batch's `index == 0` call just created — each call sees the prior call's already
    ///-committed `SetStreams`, since dispatches within one batch are strictly sequential).
    fn handle_import_frame_payload(&self, payload: &str, name: &str, index: u32, doc: &DocumentView<'_, RemodelScene>) -> Emit<RemodelOperation, RemodelConfigOperation> {
        let Some((mime, bytes)) = payload_from_data_url(payload) else { return Emit::default() };
        if mime.starts_with("video/") {
            return self.handle_import_video_bytes_payload(payload, name, doc);
        }
        let scene = doc.projection;
        let stream_id = if index == 0 { remodel_app_engine::next_remodel_id("stream") } else { scene.streams.last().map(|stream| stream.id.clone()).unwrap_or_else(|| remodel_app_engine::next_remodel_id("stream")) };

        let (width, height) = decode_still_image(&mime, &bytes).map_or((0, 0), |image| (image.width, image.height));
        let asset_key = format!("{stream_id}-frame-{index}");
        let asset = ImageAsset { mime, data: base64::engine::general_purpose::STANDARD.encode(&bytes), width, height };

        let mut streams = scene.streams.clone();
        match streams.iter_mut().find(|stream| stream.id == stream_id) {
            Some(stream) => {
                let frame_index = stream.frames.len() as u32;
                stream.frames.push(FrameRef { index: frame_index, timestamp_ms: f64::from(frame_index) * 1000.0 / 30.0, asset_id: asset_key.clone() });
            }
            None => {
                streams.push(MediaStream {
                    id: stream_id.clone(),
                    name: name.to_string(),
                    kind: MediaKind::ImageSequence,
                    camera_id: None,
                    sync_offset_ms: 0.0,
                    fps_hint: 30.0,
                    frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: asset_key.clone() }],
                    source: None,
                });
            }
        }
        Emit::amend(vec![RemodelOperation::SetAsset { key: asset_key, value: Some(asset) }, RemodelOperation::SetStreams { streams }], format!("remodel-import:{stream_id}"))
    }

    /// 🎞️ Host-decoded video frame tick (Tier 1/2 `RequestMediaFrames` frame dispatch): decodes the
    /// sampled JPEG, runs it through the relative blur gate (rebuilt from persisted frames each tick —
    /// see `rebuild_video_import_scratch`), and amends it into the active stream.
    fn handle_import_video_frame_payload(&self, payload: &str, name: &str, index: u32, frame_index: u32, timestamp_ms: f64, doc: &DocumentView<'_, RemodelScene>) -> Emit<RemodelOperation, RemodelConfigOperation> {
        let Some((_mime, bytes)) = payload_from_data_url(payload) else { return Emit::default() };
        let Ok(image) = remodel_image::decode_jpeg(&bytes) else { return Emit::default() };
        let scene = doc.projection;
        let stream_id = if index == 0 { remodel_app_engine::next_remodel_id("stream") } else { scene.streams.last().map(|stream| stream.id.clone()).unwrap_or_else(|| remodel_app_engine::next_remodel_id("stream")) };

        let score = local_sharpness_score(&image);
        let min_sharpness = scene.params.ingest.min_sharpness;
        let mut scratch = rebuild_video_import_scratch(scene, &stream_id);
        if blur_gate_reject(&mut scratch, score, min_sharpness) {
            return Emit::default();
        }

        let asset_key = format!("{stream_id}-frame-{frame_index}");
        let asset = ImageAsset { mime: "image/jpeg".into(), data: base64::engine::general_purpose::STANDARD.encode(&bytes), width: image.width, height: image.height };
        let mut streams = scene.streams.clone();
        match streams.iter_mut().find(|stream| stream.id == stream_id) {
            Some(stream) => {
                stream.kind = MediaKind::Video;
                stream.frames.push(FrameRef { index: frame_index, timestamp_ms, asset_id: asset_key.clone() });
            }
            None => streams.push(MediaStream {
                id: stream_id.clone(),
                name: name.to_string(),
                kind: MediaKind::Video,
                camera_id: None,
                sync_offset_ms: 0.0,
                fps_hint: 0.0,
                frames: vec![FrameRef { index: frame_index, timestamp_ms, asset_id: asset_key.clone() }],
                source: None,
            }),
        }
        Emit::amend(vec![RemodelOperation::SetAsset { key: asset_key, value: Some(asset) }, RemodelOperation::SetStreams { streams }], format!("remodel-import:{stream_id}"))
    }

    /// ✅️ Host-decoded video import finished: writes `VideoSource` provenance on the just-imported
    /// stream (`doc.projection.streams.last()` — the stream this batch's ticks just built). Uses the
    /// SAME coalesce key as every preceding `ImportVideoFramePayload` tick, so the whole import
    /// (every accepted frame plus this final metadata write) collapses into one undo step.
    fn handle_import_video_done(&self, name: &str, duration_ms: f64, frame_count: u32, width: u32, height: u32, codec: &str, doc: &DocumentView<'_, RemodelScene>) -> Emit<RemodelOperation, RemodelConfigOperation> {
        let scene = doc.projection;
        let Some(stream_id) = scene.streams.last().map(|stream| stream.id.clone()) else { return Emit::default() };
        let codec_value = remodel_app_engine::video_codec_from_label(codec);
        let mut streams = scene.streams.clone();
        let Some(stream) = streams.iter_mut().find(|stream| stream.id == stream_id) else { return Emit::default() };
        stream.source = Some(VideoSource { name: name.to_string(), container: "unknown".into(), codec: codec_value, duration_ms, frame_count, width, height });
        Emit::amend(vec![RemodelOperation::SetStreams { streams }], format!("remodel-import:{stream_id}"))
    }

    /// 🎞️ Tier-3 fallback (or `ImportFramePayload`'s own video-mime branch): the host couldn't decode the
    /// video, so it hands over the raw container bytes and this crate's own `remodel_video` demux/MJPEG/
    /// baseline-AVC decoder extracts frames fully in-process. The whole batch materializes inside this
    /// ONE pure call, so it needs no coalesce key (already exactly one `Emit`, hence one undo step). An
    /// undecodable codec surfaces as a `Notify` naming it, with provenance from the probe.
    fn handle_import_video_bytes_payload(&self, payload: &str, name: &str, doc: &DocumentView<'_, RemodelScene>) -> Emit<RemodelOperation, RemodelConfigOperation> {
        let Some((_mime, bytes)) = payload_from_data_url(payload) else { return Emit::default() };
        let probe = match remodel_video::probe(&bytes) {
            Ok(probe) => probe,
            Err(error) => return Emit::effect(HostEffect::Notify { message: format!("Could not probe video: {error}") }),
        };
        let (codec, width, height, duration_ms, container) = remodel_app_engine::describe_video_probe(&probe);
        let scene = doc.projection;
        let ingest = &scene.params.ingest;
        let opts = remodel_video::VideoIngestOptions { stride: ingest.frame_sample_stride.max(1), max_frames: ingest.max_frames, max_long_edge_px: ingest.downscale_long_edge_px };
        let iter = match remodel_video::extract_frames(&bytes, &opts) {
            Ok(iter) => iter,
            Err(error) => return Emit::effect(HostEffect::Notify { message: format!("Unsupported video codec ({codec:?}): {error} - probed {container} {width}x{height}") }),
        };

        let stream_id = remodel_app_engine::next_remodel_id("stream");
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
            operations.push(RemodelOperation::SetAsset {
                key: asset_key.clone(),
                value: Some(ImageAsset { mime: "image/jpeg".into(), data: base64::engine::general_purpose::STANDARD.encode(&jpeg), width: extracted.image.width, height: extracted.image.height }),
            });
            frames.push(FrameRef { index: extracted.index, timestamp_ms: extracted.timestamp_ms, asset_id: asset_key });
        }
        let mut streams = scene.streams.clone();
        streams.push(MediaStream {
            id: stream_id,
            name: name.to_string(),
            kind: MediaKind::Video,
            camera_id: None,
            sync_offset_ms: 0.0,
            fps_hint: 0.0,
            frames,
            source: Some(VideoSource { name: String::new(), container: container.into(), codec: remodel_app_engine::video_codec_to_document(codec), duration_ms, frame_count: 0, width, height }),
        });
        operations.push(RemodelOperation::SetStreams { streams });
        Emit::operations(operations)
    }
    //#endregion 🔖️Ingestion

    //#region 🔖️StagedReconstruction
    /// 🚀️ B1: runs the WHOLE staged pipeline synchronously (see module doc comment) — validates ≥2
    /// accepted frames, builds an engine from the current document params, pushes every stream's
    /// already-persisted frames into it, then loops `advance()` in-process until `Done`/`Failed` (bounded
    /// by `REMODEL_MAX_RECONSTRUCTION_TICKS`) and returns exactly one `Emit` carrying only the FINAL
    /// state — one call, one `Emit`, one undo step; no coalesce key needed.
    fn handle_run_reconstruction(&self, doc: &DocumentView<'_, RemodelScene>) -> Emit<RemodelOperation, RemodelConfigOperation> {
        let scene = doc.projection;
        let engine_params = remodel_app_engine::build_engine_params(&scene.params);
        let mut engine = remodel_engine::ReconstructionEngine::new(&engine_params);
        let mut pushed = 0u32;
        for stream in &scene.streams {
            for frame_ref in &stream.frames {
                let Some(asset) = scene.assets.get(&frame_ref.asset_id) else { continue };
                let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&asset.data) else { continue };
                if let Ok(image) = decode_still_image(&asset.mime, &bytes) {
                    engine.push_frame(frame_ref.index, image, frame_ref.timestamp_ms);
                    pushed += 1;
                }
            }
        }
        if pushed < 2 {
            return Emit::default(); // fewer than 2 accepted frames: too little to reconstruct from
        }
        let job_id = remodel_app_engine::next_remodel_id("job");
        let mut last_progress = 0.0f32;
        let mut ticks = 0u32;
        loop {
            ticks += 1;
            if ticks > REMODEL_MAX_RECONSTRUCTION_TICKS {
                let job = ReconstructionJob {
                    id: job_id,
                    stage: ReconstructionStage::Failed,
                    progress_0_1: last_progress,
                    cancel_requested: false,
                    stage_cursor: 0,
                    started_at_ms: None,
                    error: Some("reconstruction did not converge within the bounded tick budget".into()),
                    camera_poses_preview: Vec::new(),
                    sparse_point_cloud_preview: PackedF32::default(),
                };
                return Emit::operations(vec![RemodelOperation::SetJob { job }]);
            }
            match engine.advance(RECONSTRUCTION_STEP_BUDGET) {
                remodel_engine::EngineStatus::Working { progress, .. } => {
                    last_progress = progress;
                }
                remodel_engine::EngineStatus::Done => {
                    let accepted_count = engine.frame_source().accepted_count();
                    let preview = engine.sparse_preview();
                    let quality = engine.take_quality();
                    let mesh_data = engine.take_mesh();
                    let geo_products = engine.take_geo_products();

                    let registered_count = preview.camera_poses.len();
                    let camera_previews: Vec<CameraPosePreview> = preview.camera_poses.iter().enumerate().map(|(index, pose)| remodel_app_engine::camera_pose_preview(index as u32, pose)).collect();

                    let job = ReconstructionJob {
                        id: job_id.clone(),
                        stage: ReconstructionStage::Done,
                        progress_0_1: 1.0,
                        cancel_requested: false,
                        stage_cursor: 0,
                        started_at_ms: None,
                        error: None,
                        camera_poses_preview: camera_previews.clone(),
                        sparse_point_cloud_preview: PackedF32::from_f32_slice(&preview.packed_points),
                    };

                    let mut operations = vec![RemodelOperation::SetJob { job }];
                    operations.push(RemodelOperation::SetSparse { sparse: Some(SparseCloud { points: PackedF32::from_f32_slice(&preview.packed_points), colors: None }) });
                    if !camera_previews.is_empty() {
                        operations.push(RemodelOperation::SetTrajectory { trajectory: Some(CameraTrajectory { poses: camera_previews }) });
                    }
                    if let Some(mesh_data) = mesh_data {
                        let watertight = quality.as_ref().and_then(|quality| quality.watertight.as_ref()).map(remodel_app_engine::watertight_snapshot);
                        let mut texture_asset_id = None;
                        if let Some(texture) = &mesh_data.paint_texture_base64 {
                            let texture_size = scene.params.mesh.texture_size;
                            let asset_id = format!("mesh-texture-{job_id}");
                            operations.push(RemodelOperation::SetAsset { key: asset_id.clone(), value: Some(ImageAsset { mime: "image/png".into(), data: texture.clone(), width: texture_size, height: texture_size }) });
                            texture_asset_id = Some(asset_id);
                        }
                        operations.push(RemodelOperation::SetMeshResult { mesh: Box::new(RemodelMesh { mesh: mesh_data, source: MeshSource::Reconstructed, texture_asset_id, watertight }) });
                    }
                    if let Some(quality) = &quality {
                        operations.push(RemodelOperation::SetQc { qc: Some(remodel_app_engine::build_qc_snapshot(quality, registered_count, accepted_count, scene.gcps.len())) });
                    }
                    if let Some(geo) = geo_products {
                        let dsm_id = format!("geo-dsm-{job_id}");
                        let dtm_id = format!("geo-dtm-{job_id}");
                        operations.push(RemodelOperation::SetAsset { key: dsm_id.clone(), value: Some(remodel_app_engine::raster_to_png_asset(&geo.dsm)) });
                        operations.push(RemodelOperation::SetAsset { key: dtm_id.clone(), value: Some(remodel_app_engine::raster_to_png_asset(&geo.dtm)) });
                        operations.push(RemodelOperation::SetGeoProducts { geo: Some(GeoProducts { dsm_asset_id: Some(dsm_id), dtm_asset_id: Some(dtm_id), ortho_asset_id: None }) });
                    }
                    return Emit::operations(operations);
                }
                remodel_engine::EngineStatus::Failed(message) => {
                    let job = ReconstructionJob {
                        id: job_id,
                        stage: ReconstructionStage::Failed,
                        progress_0_1: last_progress,
                        cancel_requested: false,
                        stage_cursor: 0,
                        started_at_ms: None,
                        error: Some(message),
                        camera_poses_preview: Vec::new(),
                        sparse_point_cloud_preview: PackedF32::default(),
                    };
                    return Emit::operations(vec![RemodelOperation::SetJob { job }]);
                }
            }
        }
    }
    //#endregion 🔖️StagedReconstruction
}
//#endregion 🔖️RemodelPlayApp

impl DocumentApp for RemodelPlayApp {
    type Projection = RemodelScene;
    type Operation = RemodelOperation;
    type Config = RemodelConfig;
    type ConfigOperation = RemodelConfigOperation;
    type Command = RemodelCommand;

    fn app_id(&self) -> &str {
        REMODEL_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        REMODEL_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> RemodelScene {
        default_remodel_scene()
    }

    fn io(&self) -> Option<AppIo> {
        Some(remodel_app_engine::remodel_io())
    }

    /// 🎞️ `mesh:out` (the current reconstructed mesh, GLB-encoded) plus the inherited `document:out`
    /// default (the pack of `doc.projection`, replicated inline — see `shooting_ui`'s identical override
    /// for why: overriding `export_media` shadows the trait's provided body for every port, not just the
    /// new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, RemodelScene>) -> Result<Media, MediaError> {
        match port {
            "mesh:out" => {
                let mesh = &doc.projection.results.mesh.mesh;
                let bytes = MeshExporter::export(&GlbExporter, mesh).map_err(|error| MediaError::Payload(port.to_string(), error))?;
                Ok(Media {
                    media_type: MediaType { class: MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
                    payload: MediaPayload::Structured { schema: "3d.mesh".into(), json: base64::engine::general_purpose::STANDARD.encode(bytes) },
                })
            }
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: semio_framework_plugin::MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `photos:in` — inserts an incoming photo as one new frame on the well-known
    /// `REMODEL_WORKFLOW_PHOTOS_STREAM_ID` image-sequence stream (creating it on the first import).
    /// `document:in` stays `MediaError::NotImplemented`, unchanged from the inherited default: remodel
    /// has no whole-document-replace `Operation` variant to satisfy `whole_document_operation`
    /// (`RemodelOperation` is deliberately field-granular — see that enum's doc comment), so it never
    /// worked before this port was added either.
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, RemodelScene>) -> Result<Emit<RemodelOperation, RemodelConfigOperation>, MediaError> {
        match port {
            "photos:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "photos:in only accepts a Structured base64-image payload".into()));
                };
                let bytes = base64::engine::general_purpose::STANDARD.decode(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let (width, height) = decode_still_image("image/png", &bytes).map(|image| (image.width, image.height)).unwrap_or((0, 0));
                let scene = doc.projection;
                let stream_id = REMODEL_WORKFLOW_PHOTOS_STREAM_ID;
                let frame_index = scene.streams.iter().find(|stream| stream.id == stream_id).map(|stream| stream.frames.len() as u32).unwrap_or(0);
                let asset_key = format!("{stream_id}-frame-{frame_index}");
                let asset = ImageAsset { mime: "image/png".into(), data: json.clone(), width, height };
                let mut streams = scene.streams.clone();
                match streams.iter_mut().find(|stream| stream.id == stream_id) {
                    Some(stream) => stream.frames.push(FrameRef { index: frame_index, timestamp_ms: f64::from(frame_index) * 1000.0 / 30.0, asset_id: asset_key.clone() }),
                    None => streams.push(MediaStream {
                        id: stream_id.to_string(),
                        name: "Workflow Photos".into(),
                        kind: MediaKind::ImageSequence,
                        camera_id: None,
                        sync_offset_ms: 0.0,
                        fps_hint: 30.0,
                        frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: asset_key.clone() }],
                        source: None,
                    }),
                }
                Ok(Emit::operations(vec![RemodelOperation::SetAsset { key: asset_key, value: Some(asset) }, RemodelOperation::SetStreams { streams }]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `RemodelCommand` variant back to the action id it was declared under in
    /// `create_remodel_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &RemodelCommand) -> &str {
        match command {
            RemodelCommand::RunReconstruction => "runReconstruction",
            RemodelCommand::RetryStage { .. } => "retryStage",
            RemodelCommand::RunStage { .. } => "runStage",
            RemodelCommand::ImportFramePayload { .. } => "importFramePayload",
            RemodelCommand::ImportVideoFramePayload { .. } => "importVideoFramePayload",
            RemodelCommand::ImportVideoDone { .. } => "importVideoDone",
            RemodelCommand::ImportVideoBytesPayload { .. } => "importVideoBytesPayload",
            RemodelCommand::AddStream { .. } => "addStream",
            RemodelCommand::RemoveStream { .. } => "removeStream",
            RemodelCommand::SetStreamSync { .. } => "setStreamSync",
            RemodelCommand::EditCalibration { .. } => "editCalibration",
            RemodelCommand::CalibrateCameras => "calibrateCameras",
            RemodelCommand::AddGcp { .. } => "addGcp",
            RemodelCommand::RemoveGcp { .. } => "removeGcp",
            RemodelCommand::PlaceGcpObservation { .. } => "placeGcpObservation",
            RemodelCommand::SetIngestParams { .. } => "setIngestParams",
            RemodelCommand::SetFeatureParams { .. } => "setFeatureParams",
            RemodelCommand::SetMatchParams { .. } => "setMatchParams",
            RemodelCommand::SetSfmParams { .. } => "setSfmParams",
            RemodelCommand::SetDenseParams { .. } => "setDenseParams",
            RemodelCommand::SetMeshParams { .. } => "setMeshParams",
            RemodelCommand::SetMotionParams { .. } => "setMotionParams",
            RemodelCommand::SetGeoParams { .. } => "setGeoParams",
            RemodelCommand::ResetPlaceholderMesh => "resetPlaceholderMesh",
            RemodelCommand::ClearSparse => "clearSparse",
            RemodelCommand::ClearDense => "clearDense",
            RemodelCommand::ClearMeshResult => "clearMeshResult",
            RemodelCommand::ClearTracks => "clearTracks",
            RemodelCommand::ClearGeoProducts => "clearGeoProducts",
            RemodelCommand::ClearResult => "clearResult",
            RemodelCommand::SetSelection { .. } => "setSelection",
            RemodelCommand::SetCamera { .. } => "setCamera",
            RemodelCommand::SetLayerVisibility { .. } => "setLayerVisibility",
            RemodelCommand::SetFrameCursor { .. } => "setFrameCursor",
            RemodelCommand::SetReportTable { .. } => "setReportTable",
            RemodelCommand::SetActiveUtility { .. } => SET_ACTIVE_UTILITY_ACTION_ID,
            RemodelCommand::SetLocale { .. } => "setLocale",
            RemodelCommand::ImportFrames => "importFrames",
            RemodelCommand::ImportVideo => "importVideo",
            RemodelCommand::ExportQcReport => "exportQcReport",
        }
    }

    fn handle(&self, command: &RemodelCommand, doc: &DocumentView<'_, RemodelScene>, _cfg: &ConfigView<'_, RemodelConfig>) -> Emit<RemodelOperation, RemodelConfigOperation> {
        let scene = doc.projection;
        match command {
            //#region 🔖️StagedReconstruction
            RemodelCommand::RunReconstruction | RemodelCommand::RetryStage { .. } | RemodelCommand::RunStage { .. } => self.handle_run_reconstruction(doc),
            //#endregion 🔖️StagedReconstruction

            //#region 🔖️Ingestion
            RemodelCommand::ImportFramePayload { payload, name, index } => self.handle_import_frame_payload(payload, name, *index, doc),
            RemodelCommand::ImportVideoFramePayload { payload, name, index, frame_index, timestamp_ms } => self.handle_import_video_frame_payload(payload, name, *index, *frame_index, *timestamp_ms, doc),
            RemodelCommand::ImportVideoDone { name, duration_ms, frame_count, width, height, codec } => self.handle_import_video_done(name, *duration_ms, *frame_count, *width, *height, codec, doc),
            RemodelCommand::ImportVideoBytesPayload { payload, name } => self.handle_import_video_bytes_payload(payload, name, doc),
            RemodelCommand::AddStream { name, kind, camera_id } => {
                let kind = if kind == "video" { MediaKind::Video } else { MediaKind::ImageSequence };
                let camera_id = if camera_id.is_empty() { None } else { Some(camera_id.clone()) };
                let id = remodel_app_engine::next_remodel_id("stream");
                let mut streams = scene.streams.clone();
                streams.push(MediaStream { id, name: name.clone(), kind, camera_id, sync_offset_ms: 0.0, fps_hint: 30.0, frames: Vec::new(), source: None });
                Emit::operations(vec![RemodelOperation::SetStreams { streams }])
            }
            RemodelCommand::RemoveStream { stream_id } => {
                let streams: Vec<MediaStream> = scene.streams.iter().filter(|stream| &stream.id != stream_id).cloned().collect();
                Emit::operations(vec![RemodelOperation::SetStreams { streams }])
            }
            RemodelCommand::SetStreamSync { stream_id, sync_offset_ms } => {
                let mut streams = scene.streams.clone();
                let Some(stream) = streams.iter_mut().find(|stream| &stream.id == stream_id) else { return Emit::default() };
                stream.sync_offset_ms = *sync_offset_ms;
                Emit::operations(vec![RemodelOperation::SetStreams { streams }])
            }
            //#endregion 🔖️Ingestion

            //#region 🔖️CalibrationAndGcps
            RemodelCommand::EditCalibration { camera_id, label, model, fx, fy, cx, cy, skew, k1, k2, k3, p1, p2, locked } => {
                let entry = CameraCalibration { id: camera_id.clone(), label: label.clone(), model: model.clone(), fx: *fx, fy: *fy, cx: *cx, cy: *cy, skew: *skew, distortion: [*k1, *k2, *k3, *p1, *p2], rms_reprojection_px: None, locked: *locked };
                let mut calibration = scene.calibration.clone();
                match calibration.cameras.iter_mut().find(|camera| &camera.id == camera_id) {
                    Some(existing) => *existing = entry,
                    None => calibration.cameras.push(entry),
                }
                Emit::operations(vec![RemodelOperation::SetCalibration { calibration }])
            }
            // 🎯️ Auto-derives placeholder pinhole intrinsics (`fx = fy = max(width, height)`, principal
            // point centered, no distortion — mirroring `remodel_engine`'s own uncalibrated-input
            // heuristic) for every camera id referenced by a stream that has no calibration entry yet.
            // A documented simplification standing in for a real Zhang/checkerboard calibration pass
            // (no calibration target detection is wired into this program).
            RemodelCommand::CalibrateCameras => {
                let mut calibration = scene.calibration.clone();
                for stream in &scene.streams {
                    let Some(camera_id) = &stream.camera_id else { continue };
                    if calibration.cameras.iter().any(|camera| &camera.id == camera_id) {
                        continue;
                    }
                    let Some(frame) = stream.frames.first() else { continue };
                    let Some(asset) = scene.assets.get(&frame.asset_id) else { continue };
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
                Emit::operations(vec![RemodelOperation::SetCalibration { calibration }])
            }
            RemodelCommand::AddGcp { name, world_x, world_y, world_z } => {
                let id = remodel_app_engine::next_remodel_id("gcp");
                let mut gcps = scene.gcps.clone();
                gcps.push(GroundControlPoint { id, name: name.clone(), world_position: [*world_x, *world_y, *world_z], observations: Vec::new() });
                Emit::operations(vec![RemodelOperation::SetGcps { gcps }])
            }
            RemodelCommand::RemoveGcp { gcp_id } => {
                let gcps: Vec<GroundControlPoint> = scene.gcps.iter().filter(|gcp| &gcp.id != gcp_id).cloned().collect();
                Emit::operations(vec![RemodelOperation::SetGcps { gcps }])
            }
            RemodelCommand::PlaceGcpObservation { gcp_id, stream_id, frame_index, pixel_x, pixel_y } => {
                let mut gcps = scene.gcps.clone();
                let Some(gcp) = gcps.iter_mut().find(|gcp| &gcp.id == gcp_id) else { return Emit::default() };
                gcp.observations.push(GcpObservation { stream_id: stream_id.clone(), frame_index: *frame_index, pixel: [*pixel_x, *pixel_y] });
                Emit::operations(vec![RemodelOperation::SetGcps { gcps }])
            }
            //#endregion 🔖️CalibrationAndGcps

            //#region 🔖️ParamSetters
            RemodelCommand::SetIngestParams { frame_sample_stride, max_frames, downscale_long_edge_px, min_sharpness } => {
                Emit::operations(vec![RemodelOperation::SetIngestParams { params: IngestParams { frame_sample_stride: *frame_sample_stride, max_frames: *max_frames, downscale_long_edge_px: *downscale_long_edge_px, min_sharpness: *min_sharpness } }])
            }
            RemodelCommand::SetFeatureParams { detector, target_count, octaves, edge_threshold } => Emit::operations(vec![RemodelOperation::SetFeatureParams {
                params: FeatureParams {
                    detector: match detector.as_str() {
                        "akaze" => FeatureDetector::Akaze,
                        "harris" => FeatureDetector::Harris,
                        _ => FeatureDetector::Orb,
                    },
                    target_count: *target_count,
                    octaves: *octaves,
                    edge_threshold: *edge_threshold,
                },
            }]),
            RemodelCommand::SetMatchParams { matcher, ratio_test, cross_check, sequential_window, max_pairs_per_frame, loop_closure } => Emit::operations(vec![RemodelOperation::SetMatchParams {
                params: MatchParams {
                    matcher: if matcher == "kd-tree" { MatcherKind::KdTree } else { MatcherKind::BruteForce },
                    ratio_test: *ratio_test,
                    cross_check: *cross_check,
                    sequential_window: *sequential_window,
                    max_pairs_per_frame: *max_pairs_per_frame,
                    loop_closure: *loop_closure,
                },
            }]),
            RemodelCommand::SetSfmParams { ransac_iterations, ransac_threshold_px, min_track_length, ba_max_iterations, robust_loss, huber_delta_px } => Emit::operations(vec![RemodelOperation::SetSfmParams {
                params: SfmParams {
                    ransac_iterations: *ransac_iterations,
                    ransac_threshold_px: *ransac_threshold_px,
                    min_track_length: *min_track_length,
                    ba_max_iterations: *ba_max_iterations,
                    robust_loss: match robust_loss.as_str() {
                        "l2" => RobustLossKind::L2,
                        "cauchy" => RobustLossKind::Cauchy,
                        _ => RobustLossKind::Huber,
                    },
                    huber_delta_px: *huber_delta_px,
                },
            }]),
            RemodelCommand::SetDenseParams { resolution, window_radius_px, min_view_consistency, confidence_threshold, max_points } => Emit::operations(vec![RemodelOperation::SetDenseParams {
                params: DenseParams {
                    resolution: match resolution.as_str() {
                        "low" => DenseResolution::Low,
                        "high" => DenseResolution::High,
                        _ => DenseResolution::Medium,
                    },
                    window_radius_px: *window_radius_px,
                    min_view_consistency: *min_view_consistency,
                    confidence_threshold: *confidence_threshold,
                    max_points: *max_points,
                },
            }]),
            RemodelCommand::SetMeshParams { tsdf_voxel_size_mm, tsdf_truncation_mm, decimate_target_triangles, smoothing_iterations, texture_enabled, texture_size, guarantee_watertight, hole_fill_max_boundary_verts, self_intersection_check } => {
                Emit::operations(vec![RemodelOperation::SetMeshParams {
                    params: DocumentMeshParams {
                        tsdf_voxel_size_mm: *tsdf_voxel_size_mm,
                        tsdf_truncation_mm: *tsdf_truncation_mm,
                        decimate_target_triangles: *decimate_target_triangles,
                        smoothing_iterations: *smoothing_iterations,
                        texture_enabled: *texture_enabled,
                        texture_size: *texture_size,
                        guarantee_watertight: *guarantee_watertight,
                        hole_fill_max_boundary_verts: *hole_fill_max_boundary_verts,
                        self_intersection_check: *self_intersection_check,
                    },
                }])
            }
            RemodelCommand::SetMotionParams { enabled, max_tracks, track_window_px, min_track_quality, min_track_length_frames } => Emit::operations(vec![RemodelOperation::SetMotionParams {
                params: MotionParams { enabled: *enabled, max_tracks: *max_tracks, track_window_px: *track_window_px, min_track_quality: *min_track_quality, min_track_length_frames: *min_track_length_frames },
            }]),
            RemodelCommand::SetGeoParams { enabled, origin_lon, origin_lat, origin_alt, gsd_m, dsm_cell_m, dtm_filter_radius_m, ortho_max_px } => Emit::operations(vec![RemodelOperation::SetGeoParams {
                params: GeoParams { enabled: *enabled, origin_lon: *origin_lon, origin_lat: *origin_lat, origin_alt: *origin_alt, gsd_m: *gsd_m, dsm_cell_m: *dsm_cell_m, dtm_filter_radius_m: *dtm_filter_radius_m, ortho_max_px: *ortho_max_px },
            }]),
            //#endregion 🔖️ParamSetters

            //#region 🔖️ClearReset
            RemodelCommand::ResetPlaceholderMesh => Emit::operations(vec![RemodelOperation::SetMeshResult { mesh: Box::new(placeholder_result()) }]),
            RemodelCommand::ClearSparse => Emit::operations(vec![RemodelOperation::SetSparse { sparse: None }]),
            RemodelCommand::ClearDense => Emit::operations(vec![RemodelOperation::SetDense { dense: None }]),
            RemodelCommand::ClearMeshResult => Emit::operations(vec![RemodelOperation::SetMeshResult { mesh: Box::new(empty_result()) }]),
            RemodelCommand::ClearTracks => Emit::operations(vec![RemodelOperation::SetTracks { tracks: Vec::new() }]),
            RemodelCommand::ClearGeoProducts => Emit::operations(vec![RemodelOperation::SetGeoProducts { geo: None }]),
            RemodelCommand::ClearResult => Emit::operations(vec![
                RemodelOperation::SetMeshResult { mesh: Box::new(empty_result()) },
                RemodelOperation::SetSparse { sparse: None },
                RemodelOperation::SetDense { dense: None },
                RemodelOperation::SetTrajectory { trajectory: None },
                RemodelOperation::SetTracks { tracks: Vec::new() },
                RemodelOperation::SetGeoProducts { geo: None },
                RemodelOperation::SetQc { qc: None },
            ]),
            //#endregion 🔖️ClearReset

            //#region 🔖️ViewActions
            RemodelCommand::SetSelection { mode, ids } => Emit::config(vec![RemodelConfigOperation::SetSelection { mode: mode.clone(), ids: ids.clone() }]),
            RemodelCommand::SetCamera { camera } => Emit::config(vec![RemodelConfigOperation::SetCamera { camera: camera.clone() }]),
            RemodelCommand::SetLayerVisibility { layer, visible } => Emit::config(vec![RemodelConfigOperation::SetLayerVisibility { layer: layer.clone(), visible: *visible }]),
            RemodelCommand::SetFrameCursor { stream_id, frame_index } => Emit::config(vec![RemodelConfigOperation::SetFrameCursor { stream_id: stream_id.clone(), frame_index: *frame_index }]),
            RemodelCommand::SetReportTable { table } => Emit::config(vec![RemodelConfigOperation::SetReportTable { table: table.clone() }]),
            RemodelCommand::SetActiveUtility { utility_id } => Emit::config(vec![RemodelConfigOperation::SetActiveUtility { utility_id: utility_id.clone() }]),
            RemodelCommand::SetLocale { value } => Emit::config(vec![RemodelConfigOperation::SetLocale { value: value.clone() }]),
            //#endregion 🔖️ViewActions

            //#region 🔖️Export
            RemodelCommand::ImportFrames => Emit::effect(HostEffect::RequestFileOpen { accept: REMODEL_MEDIA_ACCEPT.into(), read_as: Some("dataUrl".into()), import_action: "importFramePayload".into(), multiple: true }),
            RemodelCommand::ImportVideo => {
                let ingest = &scene.params.ingest;
                Emit::effect(HostEffect::RequestMediaFrames {
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
            RemodelCommand::ExportQcReport => match &scene.results.qc {
                Some(qc) => Emit::effect(HostEffect::DownloadMediaExport { filename: "remodel-qc-report.ops".into(), mime_type: "text/plain".into(), data: serde_json::to_string_pretty(qc).unwrap_or_default(), encoding: None }),
                None => Emit::default(),
            },
            //#endregion 🔖️Export
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, RemodelScene>, cfg: &ConfigView<'_, RemodelConfig>) -> UiNode {
        let scene = doc.projection;
        let config = cfg.projection;
        let active_utility = config.active_utility_id.as_str();
        let labels = remodel_labels(config);
        match body_key {
            REMODEL_PLAY_BODY_MAIN => {
                let mut world_scene = world3d_scene(
                    world3d_camera_json(config.camera.position, config.camera.target, config.camera.fov),
                    world_meshes_json(scene),
                    world_instances_json(config),
                    world3d_selection_json(&config.selection.mode, &[], None),
                    &WorldSunConfig::default(),
                );
                world_scene.points_json = world_points_json(scene, config);
                build_world_3d_scene(REMODEL_PLAY_SURFACE_MAIN, REMODEL_PLAY_APP_ID, world_scene)
            }
            REMODEL_PLAY_BODY_FRAMES => {
                let scene_2d = Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: frames_layers_json(scene, &config.frame_cursor) };
                build_canvas_2d_scene(REMODEL_PLAY_SURFACE_FRAMES, REMODEL_PLAY_APP_ID, scene_2d)
            }
            REMODEL_PLAY_BODY_REPORT => {
                let (columns_json, rows_json) = report_table_json(scene, &config.report_table);
                build_table_scene(REMODEL_PLAY_SURFACE_REPORT, REMODEL_PLAY_APP_ID, TableScene::base(columns_json, rows_json))
            }
            REMODEL_PLAY_BODY_MEDIA => build_media_panel(scene, labels),
            REMODEL_PLAY_BODY_PIPELINE => build_pipeline_panel(scene, config, active_utility, labels),
            REMODEL_PLAY_BODY_RESULTS => build_results_panel(scene, labels),
            REMODEL_PLAY_BODY_PARAMETERS => build_parameters_panel(scene, labels),
            REMODEL_PLAY_BODY_CALIBRATION => build_calibration_panel(scene, labels),
            REMODEL_PLAY_BODY_TRACKS => build_tracks_panel(scene, labels),
            REMODEL_PLAY_BODY_QC => build_qc_panel(scene, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 👁️ Dynamic per-render window measures — `remodel-main`'s `remodel.layers` toggle group must
    /// reflect the LIVE config (not a manifest-frozen snapshot), so it is supplied here rather than via
    /// `AppBuilder::window_kind_measures` (a static, build-once declaration — see `lowpoly-plugin`'s
    /// `world3d_sun_measures`/`window_measures` for the identical pattern this mirrors).
    fn window_measures(&self, _doc: &DocumentView<'_, RemodelScene>, cfg: &ConfigView<'_, RemodelConfig>) -> std::collections::HashMap<String, Vec<semio_framework_plugin::WindowMeasure>> {
        std::collections::HashMap::from([(REMODEL_PLAY_WINDOW_MAIN.to_string(), vec![remodel_layer_measures(&cfg.projection.layers, remodel_labels(cfg.projection))])])
    }
}

//#region 🔖️Manifest
/// 👁️ `remodel.layers` — `remodel-main`'s layer-visibility toggle group (`setLayerVisibility`).
fn remodel_layer_measures(layers: &RemodelLayerVisibility, labels: &RemodelLabels) -> semio_framework_plugin::WindowMeasure {
    let toggle = |id: &str, icon: &str, label: LabelText, pressed: bool, layer: &str| semio_framework_plugin::WindowMeasure::Toggle {
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
        App::builder(REMODEL_PLAY_APP_ID, LocalizedLabel::native("Remodel", "Remodel"))
            .document(["semio", "remodel"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.remodel".into(),
                name: "3D Remodel".into(),
                source_format: "remodel.scene".into(),
                component_kind: "remodel".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
                schema: "remodel.scene".into(),
                export_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj, OsMediaFormat::Stl, OsMediaFormat::Ply, OsMediaFormat::Las, OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj],
            })
            // 🔌️ `photos:in`/`mesh:out` — WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE
            // Wave 2 port recipe; `2d.image`/`3d.mesh` are declared by `shooting`/`lowpoly` respectively
            // (reused here, not redeclared).
            .media_input(remodel_app_engine::remodel_photos_in_port())
            .media_output(remodel_app_engine::remodel_mesh_out_port())
            .icon_id("remodel-app")
            .mode("capture", LocalizedLabel::native("Capture", "Aufnahme"), "camera")
            .mode("model", LocalizedLabel::native("Model", "Modell"), "box")
            .mode("analyze", LocalizedLabel::native("Analyze", "Analyse"), "search")
            .default_mode_id("model")
            .window_kind(REMODEL_PLAY_WINDOW_MAIN, LocalizedLabel::native("Model", "Modell"), REMODEL_PLAY_BODY_MAIN, SurfaceKind::World3d, "remodel-model")
            .window_kind(REMODEL_PLAY_WINDOW_FRAMES, LocalizedLabel::native("Frames", "Frames"), REMODEL_PLAY_BODY_FRAMES, SurfaceKind::Canvas2d, "layout-grid")
            .window_kind(REMODEL_PLAY_WINDOW_REPORT, LocalizedLabel::native("Report", "Bericht"), REMODEL_PLAY_BODY_REPORT, SurfaceKind::Table, "document-report")
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
                Some("table-2".into()),
                None,
            ))
            .mode_layout("capture", "remodel-capture")
            .mode_layout("analyze", "remodel-analyze")
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), PanelGroup::Workbench, REMODEL_PLAY_BODY_PIPELINE)
            .panel_tab(REMODEL_PANEL_MEDIA_ID, LocalizedLabel::native("Media", "Medien"), PanelGroup::Workbench, REMODEL_PLAY_BODY_MEDIA)
            .panel_tab(REMODEL_PANEL_RESULTS_ID, LocalizedLabel::native("Results", "Ergebnisse"), PanelGroup::Workbench, REMODEL_PLAY_BODY_RESULTS)
            .panel_tab(REMODEL_PANEL_PARAMETERS_ID, LocalizedLabel::native("Parameters", "Parameter"), PanelGroup::Details, REMODEL_PLAY_BODY_PARAMETERS)
            .panel_tab(REMODEL_PANEL_CALIBRATION_ID, LocalizedLabel::native("Calibration", "Kalibrierung"), PanelGroup::Details, REMODEL_PLAY_BODY_CALIBRATION)
            .panel_tab(REMODEL_PANEL_TRACKS_ID, LocalizedLabel::native("Tracks", "Spuren"), PanelGroup::Details, REMODEL_PLAY_BODY_TRACKS)
            .panel_tab(REMODEL_PANEL_QC_ID, LocalizedLabel::native("Quality", "Qualität"), PanelGroup::Settings, REMODEL_PLAY_BODY_QC)
            // 🚀️ Staged reconstruction — now fully synchronous (see module doc comment); there is no more
            // `advanceReconstruction`/`cancelReconstruction` action (nothing left to advance or cancel).
            .operation("runReconstruction", LocalizedLabel::native("Run Reconstruction", "Rekonstruktion starten"))
            .operation("retryStage", LocalizedLabel::native("Retry", "Wiederholen"))
            .operation("runStage", LocalizedLabel::native("Run Stage", "Stufe ausführen"))
            .action_args("runStage", vec![ActionArgDef::select(
                "stage",
                LocalizedLabel::native("Stage", "Stufe"),
                vec![
                    ActionArgOption::new("extracting-features", LocalizedLabel::native("Extracting Features", "Merkmale extrahieren")),
                    ActionArgOption::new("matching-features", LocalizedLabel::native("Matching Features", "Merkmale zuordnen")),
                    ActionArgOption::new("estimating-poses", LocalizedLabel::native("Estimating Poses", "Posen schätzen")),
                    ActionArgOption::new("bundle-adjusting", LocalizedLabel::native("Bundle Adjusting", "Bündelausgleich")),
                    ActionArgOption::new("dense-stereo", LocalizedLabel::native("Dense Stereo", "Dense-Stereo")),
                    ActionArgOption::new("fusing-volume", LocalizedLabel::native("Fusing Volume", "Volumen fusionieren")),
                    ActionArgOption::new("extracting-surface", LocalizedLabel::native("Extracting Surface", "Oberfläche extrahieren")),
                    ActionArgOption::new("texturing", LocalizedLabel::native("Texturing", "Texturierung")),
                ],
            )
            .default_value("extracting-features")])
            // 📥️ Ingestion.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new_catalog("importFrames", LocalizedLabel::native("Import Frames", "Frames importieren"), ActionKind::Shell) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importFramePayload", LocalizedLabel::native("Import Frame Payload", "Bild-Payload importieren"), ActionKind::Operation) })
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new_catalog("importVideo", LocalizedLabel::native("Import Video", "Video importieren"), ActionKind::Shell) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importVideoFramePayload", LocalizedLabel::native("Import Video Frame Payload", "Video-Frame-Payload importieren"), ActionKind::Operation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importVideoDone", LocalizedLabel::native("Import Video Done", "Video-Import abgeschlossen"), ActionKind::Operation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importVideoBytesPayload", LocalizedLabel::native("Import Video Bytes Payload", "Video-Byte-Payload importieren"), ActionKind::Operation) })
            .operation("addStream", LocalizedLabel::native("Add Stream", "Stream hinzufügen"))
            .action_args("addStream", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value("Stream"),
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![ActionArgOption::new("image-sequence", LocalizedLabel::native("Image Sequence", "Bildsequenz")), ActionArgOption::new("video", LocalizedLabel::native("Video", "Video"))]).default_value("image-sequence"),
                ActionArgDef::text("cameraId", LocalizedLabel::native("Camera Id", "Kamera-Id")).default_value("cam-0"),
            ])
            .operation("removeStream", LocalizedLabel::native("Remove Stream", "Stream entfernen"))
            .action_args("removeStream", vec![ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required()])
            .operation("setStreamSync", LocalizedLabel::native("Set Stream Sync", "Stream-Synchronisation festlegen"))
            .action_args("setStreamSync", vec![ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required(), ActionArgDef::number("syncOffsetMs", LocalizedLabel::native("Sync Offset (ms)", "Sync-Versatz (ms)")).default_value(0)])
            // 🎯️ Calibration / GCPs.
            .operation("editCalibration", LocalizedLabel::native("Edit Calibration", "Kalibrierung bearbeiten"))
            .action_args("editCalibration", vec![
                ActionArgDef::text("cameraId", LocalizedLabel::native("Camera Id", "Kamera-Id")).required(),
                ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")),
                ActionArgDef::select("model", LocalizedLabel::native("Model", "Modell"), vec![ActionArgOption::new("pinhole", LocalizedLabel::native("Pinhole", "Lochkamera")), ActionArgOption::new("brownConrady", LocalizedLabel::native("Brown-Conrady", "Brown-Conrady")), ActionArgOption::new("fisheye", LocalizedLabel::native("Fisheye", "Fischauge"))]).default_value("pinhole"),
                ActionArgDef::number("fx", LocalizedLabel::native("fx", "fx")).default_value(1000),
                ActionArgDef::number("fy", LocalizedLabel::native("fy", "fy")).default_value(1000),
                ActionArgDef::number("cx", LocalizedLabel::native("cx", "cx")).default_value(0),
                ActionArgDef::number("cy", LocalizedLabel::native("cy", "cy")).default_value(0),
                ActionArgDef::number("skew", LocalizedLabel::native("Skew", "Scherung")).default_value(0),
                ActionArgDef::number("k1", LocalizedLabel::native("k1", "k1")).default_value(0),
                ActionArgDef::number("k2", LocalizedLabel::native("k2", "k2")).default_value(0),
                ActionArgDef::number("k3", LocalizedLabel::native("k3", "k3")).default_value(0),
                ActionArgDef::number("p1", LocalizedLabel::native("p1", "p1")).default_value(0),
                ActionArgDef::number("p2", LocalizedLabel::native("p2", "p2")).default_value(0),
                ActionArgDef::toggle("locked", LocalizedLabel::native("Locked", "Gesperrt")).default_value(false),
            ])
            .operation("calibrateCameras", LocalizedLabel::native("Calibrate Cameras", "Kameras kalibrieren"))
            .operation("addGcp", LocalizedLabel::native("Add Ground Control Point", "Passpunkt hinzufügen"))
            .action_args("addGcp", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value("GCP"),
                ActionArgDef::number("worldX", LocalizedLabel::native("World X", "Welt X")).default_value(0),
                ActionArgDef::number("worldY", LocalizedLabel::native("World Y", "Welt Y")).default_value(0),
                ActionArgDef::number("worldZ", LocalizedLabel::native("World Z", "Welt Z")).default_value(0),
            ])
            .operation("removeGcp", LocalizedLabel::native("Remove Ground Control Point", "Passpunkt entfernen"))
            .action_args("removeGcp", vec![ActionArgDef::text("gcpId", LocalizedLabel::native("GCP Id", "Passpunkt-Id")).required()])
            .operation("placeGcpObservation", LocalizedLabel::native("Place GCP Observation", "Passpunkt-Beobachtung setzen"))
            .action_args("placeGcpObservation", vec![
                ActionArgDef::text("gcpId", LocalizedLabel::native("GCP Id", "Passpunkt-Id")).required(),
                ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required(),
                ActionArgDef::number("frameIndex", LocalizedLabel::native("Frame Index", "Frame-Index")).required(),
                ActionArgDef::number("pixelX", LocalizedLabel::native("Pixel X", "Pixel X")).required(),
                ActionArgDef::number("pixelY", LocalizedLabel::native("Pixel Y", "Pixel Y")).required(),
            ])
            // ⚙️ 8 param-group setters, one per `ReconstructionParams` sub-struct.
            .operation("setIngestParams", LocalizedLabel::native("Set Ingest Params", "Ingest-Parameter festlegen"))
            .action_args("setIngestParams", vec![
                ActionArgDef::number("frameSampleStride", LocalizedLabel::native("Frame Sample Stride", "Frame-Abtastschrittweite")).default_value(5),
                ActionArgDef::number("maxFrames", LocalizedLabel::native("Max Frames", "Max. Frames")).default_value(200),
                ActionArgDef::number("downscaleLongEdgePx", LocalizedLabel::native("Downscale Long Edge (px)", "Verkleinerung lange Kante (px)")).default_value(1600),
                ActionArgDef::slider("minSharpness", LocalizedLabel::native("Min Sharpness", "Min. Schärfe"), 0.0, 1.0).default_value(0.3),
            ])
            .operation("setFeatureParams", LocalizedLabel::native("Set Feature Params", "Feature-Parameter festlegen"))
            .action_args("setFeatureParams", vec![
                ActionArgDef::select("detector", LocalizedLabel::native("Detector", "Detektor"), vec![ActionArgOption::new("orb", LocalizedLabel::native("ORB", "ORB")), ActionArgOption::new("akaze", LocalizedLabel::native("AKAZE", "AKAZE")), ActionArgOption::new("harris", LocalizedLabel::native("Harris", "Harris"))]).default_value("orb"),
                ActionArgDef::number("targetCount", LocalizedLabel::native("Target Count", "Ziel-Anzahl")).default_value(4000),
                ActionArgDef::number("octaves", LocalizedLabel::native("Octaves", "Oktaven")).default_value(4),
                ActionArgDef::slider("edgeThreshold", LocalizedLabel::native("Edge Threshold", "Kanten-Schwelle"), 1.0, 50.0).default_value(10.0),
            ])
            .operation("setMatchParams", LocalizedLabel::native("Set Match Params", "Match-Parameter festlegen"))
            .action_args("setMatchParams", vec![
                ActionArgDef::select("matcher", LocalizedLabel::native("Matcher", "Matcher"), vec![ActionArgOption::new("brute-force", LocalizedLabel::native("Brute Force", "Brute Force")), ActionArgOption::new("kd-tree", LocalizedLabel::native("KD-Tree", "KD-Baum"))]).default_value("brute-force"),
                ActionArgDef::slider("ratioTest", LocalizedLabel::native("Ratio Test", "Verhältnistest"), 0.1, 1.0).default_value(0.8),
                ActionArgDef::toggle("crossCheck", LocalizedLabel::native("Cross Check", "Kreuzprüfung")).default_value(true),
                ActionArgDef::number("sequentialWindow", LocalizedLabel::native("Sequential Window", "Sequenzielles Fenster")).default_value(8),
                ActionArgDef::number("maxPairsPerFrame", LocalizedLabel::native("Max Pairs Per Frame", "Max. Paare pro Frame")).default_value(16),
                ActionArgDef::toggle("loopClosure", LocalizedLabel::native("Loop Closure", "Schleifenschluss")).default_value(true),
            ])
            .operation("setSfmParams", LocalizedLabel::native("Set SfM Params", "SfM-Parameter festlegen"))
            .action_args("setSfmParams", vec![
                ActionArgDef::number("ransacIterations", LocalizedLabel::native("RANSAC Iterations", "RANSAC-Iterationen")).default_value(1000),
                ActionArgDef::slider("ransacThresholdPx", LocalizedLabel::native("RANSAC Threshold (px)", "RANSAC-Schwelle (px)"), 0.1, 10.0).default_value(2.0),
                ActionArgDef::number("minTrackLength", LocalizedLabel::native("Min Track Length", "Min. Spurlänge")).default_value(3),
                ActionArgDef::number("baMaxIterations", LocalizedLabel::native("BA Max Iterations", "BA Max. Iterationen")).default_value(50),
                ActionArgDef::select("robustLoss", LocalizedLabel::native("Robust Loss", "Robuster Verlust"), vec![ActionArgOption::new("l2", LocalizedLabel::native("L2", "L2")), ActionArgOption::new("huber", LocalizedLabel::native("Huber", "Huber")), ActionArgOption::new("cauchy", LocalizedLabel::native("Cauchy", "Cauchy"))]).default_value("huber"),
                ActionArgDef::slider("huberDeltaPx", LocalizedLabel::native("Huber Delta (px)", "Huber-Delta (px)"), 0.1, 10.0).default_value(1.5),
            ])
            .operation("setDenseParams", LocalizedLabel::native("Set Dense Params", "Dense-Parameter festlegen"))
            .action_args("setDenseParams", vec![
                ActionArgDef::select("resolution", LocalizedLabel::native("Resolution", "Auflösung"), vec![ActionArgOption::new("low", LocalizedLabel::native("Low", "Niedrig")), ActionArgOption::new("medium", LocalizedLabel::native("Medium", "Mittel")), ActionArgOption::new("high", LocalizedLabel::native("High", "Hoch"))]).default_value("medium"),
                ActionArgDef::number("windowRadiusPx", LocalizedLabel::native("Window Radius (px)", "Fensterradius (px)")).default_value(3),
                ActionArgDef::number("minViewConsistency", LocalizedLabel::native("Min View Consistency", "Min. Ansichtskonsistenz")).default_value(3),
                ActionArgDef::slider("confidenceThreshold", LocalizedLabel::native("Confidence Threshold", "Konfidenzschwelle"), 0.0, 1.0).default_value(0.5),
                ActionArgDef::number("maxPoints", LocalizedLabel::native("Max Points", "Max. Punkte")).default_value(500_000),
            ])
            .operation("setMeshParams", LocalizedLabel::native("Set Mesh Params", "Mesh-Parameter festlegen"))
            .action_args("setMeshParams", vec![
                ActionArgDef::slider("tsdfVoxelSizeMm", LocalizedLabel::native("TSDF Voxel Size (mm)", "TSDF-Voxelgröße (mm)"), 1.0, 20.0).default_value(5.0),
                ActionArgDef::slider("tsdfTruncationMm", LocalizedLabel::native("TSDF Truncation (mm)", "TSDF-Trunkierung (mm)"), 2.0, 60.0).default_value(20.0),
                ActionArgDef::number("decimateTargetTriangles", LocalizedLabel::native("Decimate Target Triangles", "Ziel-Dreiecke (Dezimierung)")).default_value(200_000),
                ActionArgDef::number("smoothingIterations", LocalizedLabel::native("Smoothing Iterations", "Glättungs-Iterationen")).default_value(2),
                ActionArgDef::toggle("textureEnabled", LocalizedLabel::native("Texture Enabled", "Textur aktiviert")).default_value(true),
                ActionArgDef::select("textureSize", LocalizedLabel::native("Texture Size", "Texturgröße"), vec![ActionArgOption::new("1024", LocalizedLabel::native("1024", "1024")), ActionArgOption::new("2048", LocalizedLabel::native("2048", "2048")), ActionArgOption::new("4096", LocalizedLabel::native("4096", "4096"))]).default_value("2048"),
                ActionArgDef::toggle("guaranteeWatertight", LocalizedLabel::native("Guarantee Watertight", "Wasserdichtheit garantieren")).default_value(true),
                ActionArgDef::number("holeFillMaxBoundaryVerts", LocalizedLabel::native("Hole Fill Max Boundary Verts", "Max. Randpunkte für Lochfüllung")).default_value(512),
                ActionArgDef::toggle("selfIntersectionCheck", LocalizedLabel::native("Self-Intersection Check", "Selbstüberschneidungsprüfung")).default_value(false),
            ])
            .operation("setMotionParams", LocalizedLabel::native("Set Motion Params", "Bewegungs-Parameter festlegen"))
            .action_args("setMotionParams", vec![
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).default_value(false),
                ActionArgDef::number("maxTracks", LocalizedLabel::native("Max Tracks", "Max. Spuren")).default_value(64),
                ActionArgDef::number("trackWindowPx", LocalizedLabel::native("Track Window (px)", "Spurfenster (px)")).default_value(21),
                ActionArgDef::slider("minTrackQuality", LocalizedLabel::native("Min Track Quality", "Min. Spurqualität"), 0.0, 1.0).default_value(0.3),
                ActionArgDef::number("minTrackLengthFrames", LocalizedLabel::native("Min Track Length (frames)", "Min. Spurlänge (Frames)")).default_value(5),
            ])
            .operation("setGeoParams", LocalizedLabel::native("Set Geo Params", "Geo-Parameter festlegen"))
            .action_args("setGeoParams", vec![
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).default_value(false),
                ActionArgDef::number("originLon", LocalizedLabel::native("Origin Longitude", "Ursprung Längengrad")).default_value(0),
                ActionArgDef::number("originLat", LocalizedLabel::native("Origin Latitude", "Ursprung Breitengrad")).default_value(0),
                ActionArgDef::number("originAlt", LocalizedLabel::native("Origin Altitude", "Ursprung Höhe")).default_value(0),
                ActionArgDef::slider("gsdM", LocalizedLabel::native("Ground Sample Distance (m)", "Bodenauflösung (m)"), 0.01, 1.0).default_value(0.05),
                ActionArgDef::slider("dsmCellM", LocalizedLabel::native("DSM Cell Size (m)", "DOM-Zellgröße (m)"), 0.01, 5.0).default_value(0.1),
                ActionArgDef::slider("dtmFilterRadiusM", LocalizedLabel::native("DTM Filter Radius (m)", "DGM-Filterradius (m)"), 0.1, 10.0).default_value(2.0),
                ActionArgDef::number("orthoMaxPx", LocalizedLabel::native("Ortho Max (px)", "Ortho Max. (px)")).default_value(4096),
            ])
            // 🧹️ Clear/reset.
            .operation("resetPlaceholderMesh", LocalizedLabel::native("Reset Placeholder Mesh", "Platzhalter-Mesh zurücksetzen"))
            .operation("clearSparse", LocalizedLabel::native("Clear Sparse Cloud", "Dünne Punktwolke löschen"))
            .operation("clearDense", LocalizedLabel::native("Clear Dense Cloud", "Dichte Punktwolke löschen"))
            .operation("clearMeshResult", LocalizedLabel::native("Clear Mesh", "Mesh löschen"))
            .operation("clearTracks", LocalizedLabel::native("Clear Tracks", "Spuren löschen"))
            .operation("clearGeoProducts", LocalizedLabel::native("Clear Geo Products", "Geo-Produkte löschen"))
            .operation("clearResult", LocalizedLabel::native("Clear Result", "Ergebnis löschen"))
            // 👁️ View-only runtime actions.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setLayerVisibility", LocalizedLabel::native("Set Layer Visibility", "Ebenensichtbarkeit festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setFrameCursor", LocalizedLabel::native("Set Frame Cursor", "Frame-Cursor festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setReportTable", LocalizedLabel::native("Set Report Table", "Berichtstabelle festlegen"), ActionKind::View) })
            // 📤️ Export.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new_catalog("exportQcReport", LocalizedLabel::native("Export QC Report", "QC-Bericht exportieren"), ActionKind::Shell) })
            // 🧰️ Utility groups — an exclusive per-window set (active utility is host-owned).
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer-2") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("sculpt", LocalizedLabel::native("Sculpt", "Formen"), "paintbrush") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("measure", LocalizedLabel::native("Measure", "Messen"), "scaling") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("gcpPlace", LocalizedLabel::native("Place GCP", "Passpunkt setzen"), "crosshair") })
            .window_kind_utilities(REMODEL_PLAY_WINDOW_MAIN, vec!["select".into(), "measure".into(), "sculpt".into()])
            .window_kind_utilities(REMODEL_PLAY_WINDOW_FRAMES, vec!["select".into(), "gcpPlace".into()])
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE
            // config recipe) — `io()` is this same information's single source of truth, reused here
            // rather than duplicated (`command_grammar` stays `CommandGrammar::empty()`: this app's typed
            // commands are dispatched via `RemodelCommand`'s `OpBinary` codec directly).
            .io(remodel_app_engine::remodel_io()),
    )
    .example("default", LocalizedLabel::native("Default", "Standard"), &default_example, "file")
    .workflow("remodel", "Remodel", "mesh")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use remodel_app_engine::RemodelWorldCamera;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};

    //#region 🔖️Fixtures
    /// 🏁️ High-contrast `cell`-pixel checkerboard PNG, base64-wrapped as a `requestFileOpen(readAs:
    /// "dataUrl")` payload — mirrors `remodel_engine`'s own `checker_frame` test fixture, PNG-encoded so
    /// the plugin's `ImportFramePayload`/`RunReconstruction` decode path is exercised for real.
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

    /// 📥️ Imports `n` checker frames as one new image-sequence stream via `ImportFramePayload`, mirroring
    /// exactly what a real `importFrames` → `RequestFileOpen.multiple` re-dispatch loop sends.
    fn import_checker_stream(app: &mut VcsDocumentApp<RemodelPlayApp>, n: u32) {
        for index in 0..n {
            app.dispatch_typed(RemodelCommand::ImportFramePayload { payload: checker_data_url(24, 24, 3), name: format!("frame-{index}.png"), index }, &testkit::meta("local")).expect("import frame payload");
        }
    }

    fn new_app() -> VcsDocumentApp<RemodelPlayApp> {
        testkit::new_app::<RemodelPlayApp>()
    }
    //#endregion 🔖️Fixtures

    #[test]
    fn default_scene_seeds_the_world3d_mesh_json() {
        let scene = default_remodel_scene();
        assert!(world_meshes_json(&scene).contains(REMODEL_MESH_ID));
        let config = RemodelConfig::default();
        assert!(world_instances_json(&config).contains(REMODEL_MESH_ID));
    }

    /// 🖼️ Render smoke test: every window/panel body key this app declares must render without panicking.
    #[test]
    fn render_does_not_panic_for_known_body_keys() {
        let app = testkit::new_app::<RemodelPlayApp>();
        let store_projection = app.projection().expect("projection");
        let doc = DocumentView { projection: &store_projection, history: &semio_framework_plugin::HistoryView::empty() };
        let config = RemodelConfig::default();
        let cfg = ConfigView { projection: &config };
        let inner = RemodelPlayApp;
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
            let _ = inner.render(body_key, &doc, &cfg);
        }
    }

    #[test]
    fn clear_result_resets_all_seven_result_fields_and_reset_placeholder_restores_the_box() {
        let mut app = new_app();
        let result = app.dispatch_typed(RemodelCommand::ClearResult, &testkit::meta("local")).expect("clear");
        assert_eq!(result.operations.len(), 7, "clearResult resets all 7 ReconstructionResults fields");
        assert_eq!(app.projection().expect("materialize projection").results.mesh.mesh.vertex_count(), 0);
        app.dispatch_typed(RemodelCommand::ResetPlaceholderMesh, &testkit::meta("local")).expect("reset");
        assert_eq!(app.projection().expect("materialize projection").results.mesh.source, MeshSource::Placeholder);
        assert!(app.projection().expect("materialize projection").results.mesh.mesh.vertex_count() > 0);
    }

    #[test]
    fn view_actions_emit_config_operations_not_document_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(RemodelCommand::SetCamera { camera: RemodelWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 } }, &testkit::meta("local")).expect("set camera");
        assert!(result.operations.is_empty());
        let result = app.dispatch_typed(RemodelCommand::SetLayerVisibility { layer: "dense".into(), visible: false }, &testkit::meta("local")).expect("set layer");
        assert!(result.operations.is_empty());
    }

    #[test]
    fn set_active_utility_switches_host_view_state_without_ops_or_history() {
        let mut app = new_app();
        let result = app.dispatch_typed(RemodelCommand::SetActiveUtility { utility_id: "measure".into() }, &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switch is host-owned config state, never a document operation");
    }

    //#region 🔖️ArgFormTests
    #[test]
    fn set_sfm_params_command_materializes_typed_fields_into_operations() {
        let mut app = new_app();
        let result = app
            .dispatch_typed(RemodelCommand::SetSfmParams { ransac_iterations: 500, ransac_threshold_px: 1.5, min_track_length: 4, ba_max_iterations: 20, robust_loss: "cauchy".into(), huber_delta_px: 2.5 }, &testkit::meta("local"))
            .expect("set sfm params");
        assert_eq!(result.operations.len(), 1, "typed command produces one SetSfmParams operation");
        let params = app.projection().expect("materialize projection").params.sfm;
        assert_eq!(params.ransac_iterations, 500);
        assert_eq!(params.min_track_length, 4);
        assert_eq!(params.ba_max_iterations, 20);
        assert_eq!(params.robust_loss, RobustLossKind::Cauchy);
    }

    #[test]
    fn set_geo_params_command_materializes_typed_fields_into_operations() {
        let mut app = new_app();
        app.dispatch_typed(RemodelCommand::SetGeoParams { enabled: true, origin_lon: None, origin_lat: None, origin_alt: None, gsd_m: 0.02, dsm_cell_m: 0.2, dtm_filter_radius_m: 2.0, ortho_max_px: 2048 }, &testkit::meta("local"))
            .expect("set geo params");
        let params = app.projection().expect("materialize projection").params.geo;
        assert!(params.enabled);
        assert_eq!(params.gsd_m, 0.02);
        assert_eq!(params.dsm_cell_m, 0.2);
        assert_eq!(params.ortho_max_px, 2048);
    }

    #[test]
    fn set_mesh_params_command_materializes_watertight_knobs() {
        let mut app = new_app();
        app.dispatch_typed(
            RemodelCommand::SetMeshParams {
                tsdf_voxel_size_mm: 3.0,
                tsdf_truncation_mm: 20.0,
                decimate_target_triangles: 200_000,
                smoothing_iterations: 2,
                texture_enabled: true,
                texture_size: 2048,
                guarantee_watertight: false,
                hole_fill_max_boundary_verts: 256,
                self_intersection_check: true,
            },
            &testkit::meta("local"),
        )
        .expect("set mesh params");
        let params = app.projection().expect("materialize projection").params.mesh;
        assert_eq!(params.tsdf_voxel_size_mm, 3.0);
        assert!(!params.guarantee_watertight);
        assert_eq!(params.hole_fill_max_boundary_verts, 256);
        assert!(params.self_intersection_check);
    }

    #[test]
    fn set_ingest_params_command_materializes_min_sharpness() {
        let mut app = new_app();
        app.dispatch_typed(RemodelCommand::SetIngestParams { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.42 }, &testkit::meta("local")).expect("set ingest params");
        assert_eq!(app.projection().expect("materialize projection").params.ingest.min_sharpness, 0.42);
    }
    //#endregion 🔖️ArgFormTests

    #[test]
    fn import_frame_payload_creates_a_stream_and_asset() {
        let mut app = new_app();
        import_checker_stream(&mut app, 3);
        let scene = app.projection().expect("projection");
        assert_eq!(scene.streams.len(), 1, "one importFrames batch creates exactly one stream");
        assert_eq!(scene.streams[0].frames.len(), 3);
        assert_eq!(scene.assets.len(), 3);
    }

    /// 🎞️ In-process video import (the `ImportVideoBytesPayload` fallback path): a tiny synthesized
    /// MJPEG mp4 must decode into a new video stream whose frame count matches what was muxed in.
    #[test]
    fn import_video_bytes_payload_extracts_frames_in_process() {
        let mut app = new_app();
        // 🎯️ `IngestParams::default().frame_sample_stride == 5`; force stride 1 so all 5 synthesized
        // frames are kept (a stride-sampling test belongs to `remodel_video`/`remodel_engine`, not here).
        app.dispatch_typed(RemodelCommand::SetIngestParams { frame_sample_stride: 1, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 }, &testkit::meta("local")).expect("set ingest params");
        app.dispatch_typed(RemodelCommand::ImportVideoBytesPayload { payload: checker_video_data_url(5, 32, 32, 4), name: "clip.mp4".into() }, &testkit::meta("local")).expect("import video bytes");
        let scene = app.projection().expect("projection");
        assert_eq!(scene.streams.len(), 1);
        assert_eq!(scene.streams[0].kind, MediaKind::Video);
        assert_eq!(scene.streams[0].frames.len(), 5);
        assert_eq!(scene.assets.len(), 5);
    }

    /// 🎞️ Host-decoded video import path: `ImportVideoFramePayload` ticks followed by `ImportVideoDone`
    /// must accumulate into one stream and write `VideoSource` provenance, all under one coalesce key.
    #[test]
    fn import_video_frame_payload_then_done_writes_one_stream_with_video_source() {
        let mut app = new_app();
        for index in 0..4u32 {
            let payload = checker_data_url_jpeg(24, 24, 3);
            app.dispatch_typed(RemodelCommand::ImportVideoFramePayload { payload, name: "clip.mp4".into(), index, frame_index: index, timestamp_ms: f64::from(index) * 100.0 }, &testkit::meta("local")).expect("import video frame payload");
        }
        app.dispatch_typed(RemodelCommand::ImportVideoDone { name: "clip.mp4".into(), duration_ms: 400.0, frame_count: 4, width: 24, height: 24, codec: "mjpeg".into() }, &testkit::meta("local")).expect("import video done");
        let scene = app.projection().expect("projection");
        assert_eq!(scene.streams.len(), 1);
        assert_eq!(scene.streams[0].kind, MediaKind::Video);
        assert!(scene.streams[0].source.is_some());
        assert_eq!(scene.streams[0].source.as_ref().unwrap().frame_count, 4);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        let placeholder_vertex_count = app.projection().expect("materialize projection").results.mesh.mesh.vertex_count();
        assert!(placeholder_vertex_count > 0, "the seeded placeholder box must have vertices");
        testkit::assert_undo_redo_round_trip(&mut app, RemodelCommand::ClearResult, |app| app.projection().expect("materialize projection").results.mesh.mesh.vertex_count(), placeholder_vertex_count, 0);
    }

    //#region 🔖️StagedReconstructionTests
    /// 🚀️ B1: the staged execution model is now synchronous, end-to-end — `RunReconstruction` ingests
    /// two imported checker frames and runs the WHOLE pipeline to a terminal `Done`/`Failed` stage
    /// inside the ONE `dispatch_typed` call (no more `advanceReconstruction` re-dispatch loop).
    #[test]
    fn run_reconstruction_runs_synchronously_to_a_terminal_stage() {
        let mut app = new_app();
        import_checker_stream(&mut app, 2);
        let run = app.dispatch_typed(RemodelCommand::RunReconstruction, &testkit::meta("local")).expect("run reconstruction");
        assert!(!run.operations.is_empty(), "a completed run publishes at least the final SetJob");
        let scene = app.projection().expect("projection");
        assert!(scene.job.stage == ReconstructionStage::Done || scene.job.stage == ReconstructionStage::Failed, "a synchronous run always ends terminal");
        if scene.job.stage == ReconstructionStage::Done {
            assert_eq!(scene.job.progress_0_1, 1.0);
            assert!(scene.results.sparse.is_some(), "a Done run publishes a sparse cloud");
        } else {
            assert!(scene.job.error.is_some(), "a Failed run must carry an error message");
        }
    }

    /// 🔁️ `retryStage` starts a fresh run (a new job id) even after a prior run already reached a
    /// terminal stage.
    #[test]
    fn retry_stage_starts_a_fresh_run_with_a_new_job_id() {
        let mut app = new_app();
        import_checker_stream(&mut app, 2);
        app.dispatch_typed(RemodelCommand::RunReconstruction, &testkit::meta("local")).expect("run reconstruction");
        let first_job_id = app.projection().expect("projection").job.id;

        app.dispatch_typed(RemodelCommand::RetryStage { stage: "extracting-features".into() }, &testkit::meta("local")).expect("retry stage");
        let scene = app.projection().expect("projection");
        assert!(scene.job.stage == ReconstructionStage::Done || scene.job.stage == ReconstructionStage::Failed);
        assert_ne!(scene.job.id, first_job_id, "retryStage must start a new job");
    }

    /// 📦️ The whole run collapses into exactly one undo step: undoing once after a run reaches
    /// `Done`/`Failed` must fully revert the job (and any published results) back to the pristine
    /// pre-run document — trivially true now that one `dispatch_typed` call is one `Apply`.
    #[test]
    fn full_run_collapses_into_a_single_undo_step() {
        let mut app = new_app();
        import_checker_stream(&mut app, 2);
        let before_job = app.projection().expect("projection").job;
        app.dispatch_typed(RemodelCommand::RunReconstruction, &testkit::meta("local")).expect("run reconstruction");
        assert_ne!(app.projection().expect("projection").job, before_job, "run must have changed the job");

        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").job, before_job, "one undo must fully revert the run");
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
            RemodelCommand::SetFeatureParams { detector: "akaze".into(), target_count: 1000, octaves: 4, edge_threshold: 10.0 },
            RemodelCommand::AddGcp { name: "corner".into(), world_x: 1.0, world_y: 2.0, world_z: 3.0 },
            |app| {
                let projection = app.projection().expect("materialize projection");
                (projection.params.feature.detector, projection.gcps.first().map(|gcp| gcp.name.clone()))
            },
        );
    }

    //#region 🔖️MediaPortTests
    /// 🔌️ `photos:in` inserts an incoming photo as one new frame on the well-known workflow-photos
    /// stream, creating it on the first import and appending on subsequent ones.
    #[test]
    fn import_media_photos_in_creates_and_appends_to_the_workflow_stream() {
        let app = new_app();
        let projection = app.projection().expect("projection");
        let doc = DocumentView { projection: &projection, history: &semio_framework_plugin::HistoryView::empty() };
        let inner = RemodelPlayApp;
        let media = Media {
            media_type: MediaType { class: MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
            payload: MediaPayload::Structured { schema: "2d.image".into(), json: base64::engine::general_purpose::STANDARD.encode(remodel_image::encode_png(&remodel_image::ImageRgba8::new(4, 4)).unwrap()) },
        };
        let emit = inner.import_media("photos:in", &media, &doc).expect("photos:in import");
        assert_eq!(emit.document_operations.len(), 2, "one SetAsset + one SetStreams");
        let next = emit.document_operations.iter().fold(projection.clone(), |scene, operation| remodel_op::apply_remodel_operation(&scene, operation));
        assert_eq!(next.streams.len(), 1);
        assert_eq!(next.streams[0].id, REMODEL_WORKFLOW_PHOTOS_STREAM_ID);
        assert_eq!(next.streams[0].frames.len(), 1);

        let doc2 = DocumentView { projection: &next, history: &semio_framework_plugin::HistoryView::empty() };
        let emit2 = inner.import_media("photos:in", &media, &doc2).expect("second photos:in import");
        let next2 = emit2.document_operations.iter().fold(next.clone(), |scene, operation| remodel_op::apply_remodel_operation(&scene, operation));
        assert_eq!(next2.streams.len(), 1, "still one workflow-photos stream");
        assert_eq!(next2.streams[0].frames.len(), 2, "second import appends a second frame");
    }

    /// 🔌️ `mesh:out` exports the current reconstructed mesh as a GLB-encoded `3d.mesh` `Media`.
    #[test]
    fn export_media_mesh_out_exports_a_structured_3d_mesh() {
        let app = new_app();
        let projection = app.projection().expect("projection");
        let doc = DocumentView { projection: &projection, history: &semio_framework_plugin::HistoryView::empty() };
        let inner = RemodelPlayApp;
        let media = inner.export_media("mesh:out", &doc).expect("mesh:out export");
        assert_eq!(media.media_type.class, MediaClass::ThreeD);
        assert_eq!(media.media_type.form, semio_framework_plugin::MediaForm::Mesh);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "3d.mesh");
                assert!(!json.is_empty());
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }

    #[test]
    fn remodel_io_declares_photos_in_and_mesh_out_on_the_manifest() {
        let app = create_remodel_app();
        let media_inputs = &app.definition.media_inputs;
        let media_outputs = &app.definition.media_outputs;
        assert!(media_inputs.iter().any(|port| port.id == "photos:in"));
        assert!(media_outputs.iter().any(|port| port.id == "mesh:out"));
    }
    //#endregion 🔖️MediaPortTests
}
//#endregion 🧪️Tests
