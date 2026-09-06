// #region 🧲️Header
// 💻️ .storybook/stories/remodel/scene.ts
// Specs: The projection + reducer half of the `📸️remodel` scope — turns the story-local document/config
// fixtures (`./fixture.ts`) into exactly the payloads remodel's own Rust render functions build, and folds the
// host-dispatched `ActionDescriptor`s back into config the way `RemodelingCommand::dispatch` does.
// Summary: One projection per window kind, mirroring its Rust source line for line —
//   `remodelMainWorldScene`   ⇔ `✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️.rs` (`remodeling-main`, World3d)
//   `remodelFramesCanvasScene`⇔ `✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs` (`remodeling-frames`, Canvas2d)
//   `remodelReportTableScene` ⇔ `✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs` (`remodeling-report`, Table)
//   `remodelViewerWorldScene` ⇔ `👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs` (`remodeling-view-model`, World3d)
// plus one `BuiltNode` projection per panel tab (`✏️editor/📌️panels/*/🦀️.rs`), rendered through the REAL
// `InterpretedUiNode`. The panels' Rust still emits the legacy `UiNode` (`ui_stack_vertical`/`ui_text`/
// `ui_import_drop_zone`); no TS adapter from that shape to the retained `BuiltNode` document exists, so these
// helpers do the Stack→`container(plain)` / Text→`text` translation themselves and note it — the strings
// themselves are formatted exactly as the Rust `format!` calls do, in whichever locale the labels came from.
// Reducers cover the config-only command subset the hosts can actually reach without a plugin runtime
// (`setCamera`/`setLayerVisibility`/`setFrameCursor`/`setReportTable`/`setActiveUtility`/`setLocale`);
// an unrecognized action is ignored, matching `command_from_action` returning `None`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { ActionDescriptor, Canvas2dScene, TableScene, UiComponentSceneNode, World3dScene } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { AccessibilitySpec, BuiltNode, LayoutSpec, StyleSpec } from "../../../🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract";

import { remodelLabelsFor, type RemodelConfig, type RemodelLabels, type RemodelScene } from "./fixture";

//#region 🔖️Constants
/** 🆔️ `REMODELING_PLAY_APP_ID` — the editor's world/canvas/table controller id. */
export const REMODEL_EDITOR_CONTROLLER_ID = "remodeling-play";
/** 🆔️ `REMODELING_VIEW_CONTROLLER_ID` — kept distinct so a viewer surface is never mistaken for an editor one. */
export const REMODEL_VIEWER_CONTROLLER_ID = "remodeling-view";
const REMODEL_MESH_ID = "remodeling-result";
const REMODEL_SURFACE_MAIN = "remodeling.play";
const REMODEL_SURFACE_FRAMES = "remodeling.play.frames";
const REMODEL_SURFACE_REPORT = "remodeling.play.report";
const REMODEL_SURFACE_VIEW = "remodeling.view.scene3d/model";
/** 📥️ `REMODELING_MEDIA_ACCEPT` (`✏️editor/🎮️commands/🎞️import-frames/🦀️.rs`) — the media panel drop zone's accept list, verbatim. */
const REMODEL_MEDIA_ACCEPT = "image/png,image/jpeg,video/mp4,video/quicktime,video/webm,video/x-msvideo,.png,.jpg,.jpeg,.mp4,.mov,.webm,.avi";

/** 🚦️ `stage_display` (`🧬️schema/🦀️.rs`) — the kebab wire tag → its display string. */
const REMODEL_STAGE_DISPLAY: Readonly<Record<string, string>> = {
  idle: "Idle",
  ingesting: "Ingesting",
  calibrating: "Calibrating",
  "extracting-features": "Extracting Features",
  "matching-features": "Matching Features",
  "estimating-poses": "Estimating Poses",
  "bundle-adjusting": "Bundle Adjusting",
  georeferencing: "Georeferencing",
  "dense-stereo": "Dense Stereo",
  "fusing-volume": "Fusing Volume",
  "extracting-surface": "Extracting Surface",
  "cleaning-mesh": "Cleaning Mesh",
  texturing: "Texturing",
  "tracking-motion": "Tracking Motion",
  "deriving-geo-products": "Deriving Geo Products",
  "reporting-qc": "Reporting QC",
  done: "Done",
  failed: "Failed",
};

/** 🕒️ The three modes `create_remodeling_app` declares, each with the window kind its layout puts in the main slot. */
export const REMODEL_MODES = [
  { id: "capture", windowKindId: "remodeling-frames" },
  { id: "model", windowKindId: "remodeling-main" },
  { id: "analyze", windowKindId: "remodeling-report" },
] as const;

/** 🪟️ Every panel tab `create_remodeling_app` registers, in registration order, with its body key. */
export const REMODEL_PANELS = [
  { id: "pipeline", bodyKey: "remodeling.play.pipeline", labelKey: "panelPipeline" },
  { id: "media", bodyKey: "remodeling.play.media", labelKey: "panelMedia" },
  { id: "results", bodyKey: "remodeling.play.results", labelKey: "panelResults" },
  { id: "parameters", bodyKey: "remodeling.play.parameters", labelKey: "panelParameters" },
  { id: "calibration", bodyKey: "remodeling.play.calibration", labelKey: "panelCalibration" },
  { id: "tracks", bodyKey: "remodeling.play.tracks", labelKey: "panelTracks" },
  { id: "quality", bodyKey: "remodeling.play.qc", labelKey: "panelQc" },
] as const;

export type RemodelPanelId = (typeof REMODEL_PANELS)[number]["id"];
//#endregion 🔖️Constants

//#region 🔖️Formatting
/** @emoji 🔢️ Rust's `{:.N}` — fixed decimals, never JS's shortest round-trip. */
function fixed(value: number, decimals: number): string {
  return value.toFixed(decimals);
}

/** @emoji 🐍️ Rust's `{:?}` over a serde-kebab enum tag: the fixtures store `"brute-force"`, `Debug` prints `BruteForce`. */
function debugEnum(tag: string): string {
  return tag
    .split("-")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

/** @emoji 🔡️ `descriptor.args` narrowed to a plain record — the shape every host dispatches. */
export function remodelActionArgs(args: ActionDescriptor["args"]): Record<string, unknown> {
  return (args ?? {}) as Record<string, unknown>;
}
//#endregion 🔖️Formatting

//#region 🔖️WindowScenes
/** ☁️ `world_points_json` — the sparse/dense clouds, the recovered camera poses and the GCP world positions, each gated on its own `config.layers` toggle. `positionsB64`/`colorsB64` are already base64 LE-f32 / u8-rgb buffers on the document, so no re-encode happens here either. */
function remodelWorldPointsJson(scene: RemodelScene, layers: RemodelConfig["layers"], gateOnConfig: boolean): string | undefined {
  const out: Record<string, unknown>[] = [];
  if ((!gateOnConfig || layers.sparse) && scene.results.sparse && scene.results.sparse.points.length > 0) {
    out.push({ id: "remodeling-sparse", positionsB64: scene.results.sparse.points, colorsB64: scene.results.sparse.colors, size: 3, sizeAttenuation: true });
  }
  if ((!gateOnConfig || layers.dense) && scene.results.dense && scene.results.dense.positions.length > 0) {
    out.push({ id: "remodeling-dense", positionsB64: scene.results.dense.positions, colorsB64: scene.results.dense.colors, size: 2, sizeAttenuation: true });
  }
  if ((!gateOnConfig || layers.cameras) && scene.job.cameraPosesPreview.length > 0) {
    out.push({ id: "remodeling-camera-poses", positionsB64: packF32(scene.job.cameraPosesPreview.flatMap((pose) => [...pose.translation])), colorsB64: null, size: 9, sizeAttenuation: false });
  }
  if ((!gateOnConfig || layers.gcps) && scene.gcps.length > 0) {
    out.push({ id: "remodeling-gcps", positionsB64: packF32(scene.gcps.flatMap((gcp) => [...gcp.worldPosition])), colorsB64: null, size: 10, sizeAttenuation: false });
  }
  return out.length === 0 ? undefined : JSON.stringify(out);
}

/** @emoji 🧮️ `PackedF32::from_f32_slice` — little-endian f32 triples, base64'd, byte-for-byte the wire shape `WorldPointCloudLayer` decodes. */
function packF32(values: readonly number[]): string {
  const buffer = new ArrayBuffer(values.length * 4);
  const view = new DataView(buffer);
  values.forEach((value, index) => view.setFloat32(index * 4, value, true));
  let binary = "";
  for (const byte of new Uint8Array(buffer)) binary += String.fromCharCode(byte);
  return btoa(binary);
}

/**
 * 🧊️ `remodeling-main`'s `World3dScene`. `meshesJson` mirrors `world_meshes_json`: the composed
 * `s.stdio.semio@v1/mesh` CHILD resolves through `resolve_bounded_remodeling_mesh` against
 * `scene.durable_artifacts`, and NEITHER story document carries that map — so it is `"[]"`, exactly as the
 * plugin renders these documents. `instancesJson` still emits its instance (the Rust gates it only on
 * `config.layers.mesh`, never on whether the mesh resolved), so the host shows its own missing-mesh
 * placeholder for it — a faithful reproduction of the runtime, not a story shortcut.
 */
export function remodelMainWorldScene(scene: RemodelScene, config: RemodelConfig): World3dScene {
  return {
    cameraJson: JSON.stringify({ position: config.camera.position, target: config.camera.target, fov: config.camera.fov }),
    meshesJson: "[]",
    instancesJson: config.layers.mesh ? JSON.stringify([{ id: REMODEL_MESH_ID, meshId: REMODEL_MESH_ID, position: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], selected: false, hovered: false }]) : "[]",
    selectionJson: JSON.stringify({ mode: "rectangle", ids: [], primaryId: null }),
    pointsJson: remodelWorldPointsJson(scene, config.layers, true),
  };
}

/** 👁️ `remodeling-view-model`'s `World3dScene` — same projection with a hardcoded camera and every layer unconditionally on (a viewer keeps no per-session layer state). */
export function remodelViewerWorldScene(scene: RemodelScene): World3dScene {
  return {
    cameraJson: JSON.stringify({ position: [4, -4, 3], target: [0, 0, 0], fov: 45 }),
    meshesJson: "[]",
    instancesJson: JSON.stringify([{ id: REMODEL_MESH_ID, meshId: REMODEL_MESH_ID, position: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], selected: false, hovered: false }]),
    selectionJson: JSON.stringify({ mode: "rectangle", ids: [], primaryId: null }),
    pointsJson: remodelWorldPointsJson(scene, { mesh: true, dense: true, sparse: true, cameras: true, gcps: true }, false),
  };
}

/**
 * 🖼️ `frames_layers_json` VERBATIM — the cursored frame's image layer plus every GCP observation planted on it.
 * ⚠️ Both layer objects are keyed `type` (`"image"`/`"points"`), and the point layer carries
 * `points: [{x, y, label}]`. `Canvas2dHost`'s `JsonLayersCanvasSession` keys on `kind` and expects
 * `points: [[x, y], …]` — so this payload draws nothing recognizable. See `remodelFramesCanvasSceneHostShaped`.
 */
export function remodelFramesLayersJson(scene: RemodelScene, cursor: RemodelConfig["frameCursor"]): string {
  if (cursor.streamId === null) return "[]";
  const stream = scene.streams.find((candidate) => candidate.id === cursor.streamId);
  if (!stream) return "[]";
  const layers: Record<string, unknown>[] = [];
  const frame = stream.frames.find((candidate) => candidate.index === cursor.frameIndex);
  // 🖼️ `remodeling_asset` resolves the asset CHILD handle through `durable_artifacts`, which neither story
  // document carries — so, as in the plugin, no image layer is emitted for these documents.
  if (frame !== undefined && scene.assets[frame.assetId] === undefined) void frame;
  const points = scene.gcps.flatMap((gcp) => gcp.observations.filter((observation) => observation.streamId === cursor.streamId && observation.frameIndex === cursor.frameIndex).map((observation) => ({ x: observation.pixel[0], y: observation.pixel[1], label: gcp.name })));
  if (points.length > 0) layers.push({ type: "points", id: "remodeling-gcp-observations", points });
  return JSON.stringify(layers);
}

/** 🖼️ `remodeling-frames`' `Canvas2dScene` — the plugin's own payload, unchanged. */
export function remodelFramesCanvasScene(scene: RemodelScene, config: RemodelConfig): Canvas2dScene {
  return { cameraX: 0, cameraY: 0, zoom: 1, layersJson: remodelFramesLayersJson(scene, config.frameCursor) };
}

/** 🩹️ The same content re-keyed into the schema `JsonLayersCanvasSession` actually speaks (`kind: "circle"` markers with bounds) — the story's own adapter, showing what the window WOULD paint once the plugin emits a host-shaped layer list. Not a plugin behavior. */
export function remodelFramesCanvasSceneHostShaped(scene: RemodelScene, config: RemodelConfig): Canvas2dScene {
  const cursor = config.frameCursor;
  const stream = cursor.streamId === null ? undefined : scene.streams.find((candidate) => candidate.id === cursor.streamId);
  const layers = !stream
    ? []
    : scene.gcps.flatMap((gcp) =>
        gcp.observations
          .filter((observation) => observation.streamId === cursor.streamId && observation.frameIndex === cursor.frameIndex)
          .map((observation) => ({ id: `${gcp.id}@${observation.frameIndex}`, kind: "circle", role: "handle", name: gcp.name, x: observation.pixel[0] - 8, y: observation.pixel[1] - 8, width: 16, height: 16 })),
      );
  return { cameraX: 0, cameraY: 0, zoom: 4, layersJson: JSON.stringify(layers) };
}

/** 📊️ `report_table_json` — the `(columns, rows)` pair for one dataset name; any unknown name falls back to the frame list. */
export function remodelReportTableJson(scene: RemodelScene, table: string): { readonly columnsJson: string; readonly rowsJson: string } {
  switch (table) {
    case "cameras":
      return {
        columnsJson: JSON.stringify([
          { id: "id", label: "Id" },
          { id: "model", label: "Model" },
          { id: "fx", label: "fx" },
          { id: "fy", label: "fy" },
          { id: "rms", label: "RMS (px)" },
        ]),
        rowsJson: JSON.stringify(scene.calibration.cameras.map((camera) => ({ id: camera.id, model: camera.model, fx: camera.fx, fy: camera.fy, rms: camera.rmsReprojectionPx }))),
      };
    case "tracks":
      return {
        columnsJson: JSON.stringify([
          { id: "id", label: "Id" },
          { id: "length", label: "Length" },
          { id: "class", label: "Class" },
          { id: "speed", label: "Mean Speed (m/s)" },
        ]),
        rowsJson: JSON.stringify(scene.results.tracks.map((track) => ({ id: track.id, length: track.length, class: debugEnum(track.class), speed: track.meanSpeedMS }))),
      };
    case "gcps":
      return {
        columnsJson: JSON.stringify([
          { id: "id", label: "Id" },
          { id: "name", label: "Name" },
          { id: "x", label: "X" },
          { id: "y", label: "Y" },
          { id: "z", label: "Z" },
          { id: "observations", label: "Observations" },
        ]),
        rowsJson: JSON.stringify(scene.gcps.map((gcp) => ({ id: gcp.id, name: gcp.name, x: gcp.worldPosition[0], y: gcp.worldPosition[1], z: gcp.worldPosition[2], observations: gcp.observations.length }))),
      };
    case "qcStages":
      return {
        columnsJson: JSON.stringify([
          { id: "stage", label: "Stage" },
          { id: "status", label: "Status" },
        ]),
        rowsJson: JSON.stringify([{ stage: debugEnum(scene.job.stage), status: scene.job.error !== null ? "error" : "ok" }]),
      };
    case "matches":
      return { columnsJson: JSON.stringify([{ id: "note", label: "Note" }]), rowsJson: JSON.stringify([{ note: "Pairwise match data is reconstruction-runtime scratch, never distilled into durable document state." }]) };
    default:
      return {
        columnsJson: JSON.stringify([
          { id: "streamId", label: "Stream" },
          { id: "index", label: "Index" },
          { id: "timestampMs", label: "Timestamp (ms)" },
          { id: "assetId", label: "Asset" },
        ]),
        rowsJson: JSON.stringify(scene.streams.flatMap((stream) => stream.frames.map((frame) => ({ id: `${stream.id}#${frame.index}`, streamId: stream.id, index: frame.index, timestampMs: frame.timestampMs, assetId: frame.assetId })))),
      };
  }
}

/** 📊️ `remodeling-report`'s `TableScene`, with the plugin's own header row made sortable so `TableHost`'s sort round-trips visibly. */
export function remodelReportTableScene(scene: RemodelScene, config: RemodelConfig): TableScene {
  const { columnsJson, rowsJson } = remodelReportTableJson(scene, config.reportTable);
  return { columnsJson: JSON.stringify((JSON.parse(columnsJson) as Record<string, unknown>[]).map((column) => ({ ...column, sortable: true }))), rowsJson };
}

/** 🪟️ The `UiComponentSceneNode` each window kind mounts against. */
export function remodelWindowNode(windowKindId: string, scene: RemodelScene, config: RemodelConfig, hostShapedFrames = false): UiComponentSceneNode {
  switch (windowKindId) {
    case "remodeling-frames":
      return { type: "componentScene", surfaceId: REMODEL_SURFACE_FRAMES, controllerId: REMODEL_EDITOR_CONTROLLER_ID, componentKind: "canvas-2d", canvas2d: hostShapedFrames ? remodelFramesCanvasSceneHostShaped(scene, config) : remodelFramesCanvasScene(scene, config) };
    case "remodeling-report":
      return { type: "componentScene", surfaceId: REMODEL_SURFACE_REPORT, controllerId: REMODEL_EDITOR_CONTROLLER_ID, componentKind: "table", table: remodelReportTableScene(scene, config) };
    case "remodeling-view-model":
      return { type: "componentScene", surfaceId: REMODEL_SURFACE_VIEW, controllerId: REMODEL_VIEWER_CONTROLLER_ID, componentKind: "world-3d", world3d: remodelViewerWorldScene(scene) };
    default:
      return { type: "componentScene", surfaceId: REMODEL_SURFACE_MAIN, controllerId: REMODEL_EDITOR_CONTROLLER_ID, componentKind: "world-3d", world3d: remodelMainWorldScene(scene, config) };
  }
}
//#endregion 🔖️WindowScenes

//#region 🔖️PanelDocuments
const PANEL_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const PANEL_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };
const PANEL_STACK_LAYOUT: LayoutSpec = { kind: "stack", axis: "vertical", gap: "sm", padding: { all: "sm" }, align: "stretch", justify: "start", grow: false, wrap: false };
const PANEL_LEAF_LAYOUT: LayoutSpec = { kind: "leaf", width: "fill", height: "hug" };

/** @emoji 🔤️ `ui_text(Label::data(…))` translated to the retained document's `Component::Text`. */
function panelText(key: string, value: string): BuiltNode {
  return { key, component: { type: "text", value, emphasize: null, dataAttributes: null }, layout: PANEL_LEAF_LAYOUT, style: PANEL_STYLE, activity: "idle", disabled: false, accessibility: PANEL_ACCESSIBILITY, bindings: [], menu: null, children: [] };
}

/** @emoji 📚️ `ui_stack_vertical(children)` translated to the retained document's `Component::Container` in its `plain` role. */
function panelStack(key: string, children: readonly BuiltNode[]): BuiltNode {
  return {
    key,
    component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
    layout: PANEL_STACK_LAYOUT,
    style: PANEL_STYLE,
    activity: "idle",
    disabled: false,
    accessibility: PANEL_ACCESSIBILITY,
    bindings: [],
    menu: null,
    children: [...children],
  };
}

/** @emoji 📥️ `ui_import_drop_zone` translated: a `plain` container carrying its `dropOverlay` title/hint/accept plus the two text children the Rust builder nests inside it, with `importFramePayload` bound to `Trigger::Drop`. */
function panelDropZone(key: string, title: string, hint: string, accept: string): BuiltNode {
  return {
    key,
    component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: { title, hint, accept } },
    layout: { kind: "stack", axis: "vertical", gap: "sm", padding: { all: "md" }, align: "stretch", justify: "start", grow: false, wrap: false },
    style: PANEL_STYLE,
    activity: "idle",
    disabled: false,
    accessibility: PANEL_ACCESSIBILITY,
    bindings: [{ trigger: "drop", action: { scope: REMODEL_EDITOR_CONTROLLER_ID, name: "importFramePayload", version: 1 }, args: null, capability: null }],
    menu: null,
    children: [panelText(`${key}-title`, title), panelText(`${key}-hint`, hint)],
  };
}

/** 🗿️ `📌️panels/🗿️artifact` — the document/pipeline tab: job stage + progress (+ error), derived running status, active utility. */
function pipelinePanel(scene: RemodelScene, config: RemodelConfig, labels: RemodelLabels): BuiltNode {
  const stage = REMODEL_STAGE_DISPLAY[scene.job.stage] ?? scene.job.stage;
  const jobLabel = `${labels.reconstruction}: ${stage} (${fixed(scene.job.progress01 * 100, 0)}%)${scene.job.error === null ? "" : ` - ${labels.error}: ${scene.job.error}`}`;
  const running = !["idle", "done", "failed"].includes(scene.job.stage);
  return panelStack("pipeline", [panelText("pipeline-job", jobLabel), panelText("pipeline-status", `${labels.status}: ${running ? labels.running : labels.idle}`), panelText("pipeline-utility", `${labels.utility}: ${config.activeUtilityId}`)]);
}

/** 🗂️ `📌️panels/🗂️media` — the import drop zone, the stream/asset counts, and one line per stream (plus its decoded container facts). */
function mediaPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const lines: BuiltNode[] = [panelDropZone("remodeling-media-drop", labels.panelMedia, labels.noStreams, REMODEL_MEDIA_ACCEPT), panelText("media-counts", `${labels.streams}: ${scene.streams.length} - ${labels.assets}: ${Object.keys(scene.assets).length}`)];
  for (const stream of scene.streams) {
    const kindLabel = stream.kind === "video" ? labels.streamKindVideo : labels.streamKindImageSequence;
    lines.push(panelText(`media-${stream.id}`, `${stream.name} (${kindLabel}, ${stream.frames.length} ${labels.frames}, ${labels.syncOffset}: ${fixed(stream.syncOffsetMs, 1)}ms)`));
    if (stream.source) lines.push(panelText(`media-${stream.id}-source`, `  ${debugEnum(stream.source.codec)} ${stream.source.width}x${stream.source.height} ${fixed(stream.source.durationMs, 0)}ms`));
  }
  return panelStack("media", lines);
}

/** 🧵️ `📌️panels/🧵️results` — mesh source + vertex/triangle counts, then sparse/dense/trajectory/geo. */
function resultsPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const results = scene.results;
  // 🧩️ The composed mesh CHILD is unresolvable without a `durableArtifacts` map, so `unwrap_or_default()`
  // reports 0/0 rather than fabricating a count — exactly what the Rust panel does here.
  const meshLabel = `${labels.mesh}: ${debugEnum(results.mesh.source)}, 0 ${labels.vertices}, 0 ${labels.triangles}`;
  const sparse = results.sparse === null ? `${labels.sparseCloud}: ${labels.resultsNone}` : `${labels.sparseCloud}: ${Math.floor(base64ByteLength(results.sparse.points) / 4 / 3)}`;
  const dense = results.dense === null ? `${labels.denseCloud}: ${labels.resultsNone}` : `${labels.denseCloud}: ${Math.floor(base64ByteLength(results.dense.positions) / 4 / 3)}`;
  const trajectory = results.trajectory === null ? `${labels.trajectory}: ${labels.resultsNone}` : `${labels.trajectory}: ${results.trajectory.poses.length} ${labels.poses}`;
  const geo = results.geo === null ? `${labels.geoProducts}: ${labels.resultsNone}` : `${labels.geoProducts}: ${labels.available}`;
  return panelStack("results", [panelText("results-mesh", meshLabel), panelText("results-sparse", sparse), panelText("results-dense", dense), panelText("results-trajectory", trajectory), panelText("results-geo", geo)]);
}

/** @emoji 🧮️ Byte length behind a base64 payload — the story's stand-in for `PackedF32::to_f32_vec().len()`. */
function base64ByteLength(value: string): number {
  const padding = value.endsWith("==") ? 2 : value.endsWith("=") ? 1 : 0;
  return (value.length / 4) * 3 - padding;
}

/** ⚙️ `📌️panels/⚙️parameters` — one line per parameter group, formatted exactly as the Rust `format!` calls. */
function parametersPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const p = scene.params;
  return panelStack("parameters", [
    panelText("params-ingest", `${labels.paramsIngest}: ${labels.strideShort} ${p.ingest.frameSampleStride}, ${labels.maxShort} ${p.ingest.maxFrames}, ${labels.downscaleShort} ${p.ingest.downscaleLongEdgePx}px, min sharpness ${fixed(p.ingest.minSharpness, 2)}`),
    panelText("params-feature", `${labels.paramsFeature}: ${debugEnum(p.feature.detector)}, ${labels.targetShort} ${p.feature.targetCount}, ${labels.octavesShort} ${p.feature.octaves}`),
    panelText("params-matching", `${labels.paramsMatching}: ${debugEnum(p.matching.matcher)}, ${labels.ratioShort} ${fixed(p.matching.ratioTest, 2)}, ${labels.windowShort} ${p.matching.sequentialWindow}`),
    panelText("params-sfm", `${labels.paramsSfm}: ${labels.ransacShort} ${p.sfm.ransacIterations}, ${labels.minTrackShort} ${p.sfm.minTrackLength}, ${labels.baShort} ${p.sfm.baMaxIterations}`),
    panelText("params-dense", `${labels.paramsDense}: ${debugEnum(p.dense.resolution)}, ${labels.windowShort} ${p.dense.windowRadiusPx}px`),
    panelText("params-mesh", `${labels.paramsMesh}: ${labels.voxelShort} ${fixed(p.mesh.tsdfVoxelSizeMm, 1)}mm, ${labels.targetShort} ${p.mesh.decimateTargetTriangles}, watertight ${p.mesh.guaranteeWatertight}`),
    panelText("params-motion", `${labels.paramsMotion}: ${p.motion.enabled ? labels.enabled : labels.disabled}`),
    panelText("params-geo", `${labels.paramsGeo}: ${p.geo.enabled ? labels.enabled : labels.disabled}`),
  ]);
}

/** 🎯️ `📌️panels/🎯️calibration` — camera/rig counts, one line per calibrated camera, then the GCP list. */
function calibrationPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const lines: BuiltNode[] = [panelText("calibration-counts", `${labels.camerasCalibrated}: ${scene.calibration.cameras.length} - ${labels.rigExtrinsics}: ${scene.calibration.rig.length}`)];
  for (const camera of scene.calibration.cameras) lines.push(panelText(`calibration-${camera.id}`, `${camera.label} (${camera.model}): fx ${fixed(camera.fx, 1)} fy ${fixed(camera.fy, 1)}`));
  lines.push(panelText("calibration-gcps", `${labels.gcps}: ${scene.gcps.length}`));
  for (const gcp of scene.gcps) lines.push(panelText(`calibration-${gcp.id}`, `${gcp.name} [${fixed(gcp.worldPosition[0], 2)}, ${fixed(gcp.worldPosition[1], 2)}, ${fixed(gcp.worldPosition[2], 2)}] (${gcp.observations.length} obs)`));
  return panelStack("calibration", lines);
}

/** 🏃️ `📌️panels/🏃️tracks` — the track list, or the documented "not yet driven by the engine" empty state. */
function tracksPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  if (scene.results.tracks.length === 0) return panelStack("tracks", [panelText("tracks-none", labels.tracksNone), panelText("tracks-gap", labels.motionNotImplemented)]);
  const lines: BuiltNode[] = [panelText("tracks-count", `${labels.tracks}: ${scene.results.tracks.length}`)];
  for (const track of scene.results.tracks) lines.push(panelText(`tracks-${track.id}`, `${track.id} (${debugEnum(track.class)}): ${track.length} frames, ${fixed(track.meanSpeedMS, 2)} m/s`));
  return panelStack("tracks", lines);
}

/** ✅️ `📌️panels/✅️quality` — the QC report's metrics, its optional watertight block, and every warning. */
function qualityPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const qc = scene.results.qc;
  if (qc === null) return panelStack("quality", [panelText("quality-none", labels.qcNone)]);
  const lines: BuiltNode[] = [
    panelText("quality-reprojection", `${labels.qcReprojection}: ${fixed(qc.reprojectionRmsPx, 2)}px`),
    panelText("quality-track-length", `${labels.qcTrackLength}: ${fixed(qc.meanTrackLength, 1)}`),
    panelText("quality-registered", `${labels.qcRegisteredRatio}: ${fixed(qc.registeredFrameRatio * 100, 0)}%`),
    panelText("quality-dense", `${labels.qcDenseCoverage}: ${fixed(qc.denseCoverageRatio * 100, 0)}%`),
  ];
  if (qc.gcpCheckpointRmse !== null) lines.push(panelText("quality-gcp-rmse", `${labels.qcGcpRmse}: ${fixed(qc.gcpCheckpointRmse, 3)}m`));
  if (qc.watertight) {
    lines.push(panelText("quality-watertight", `${labels.qcWatertight}: ${qc.watertight.isWatertight}`));
    lines.push(panelText("quality-boundary", `${labels.qcBoundaryEdges}: ${qc.watertight.boundaryEdgeCount}`));
    lines.push(panelText("quality-components", `${labels.qcComponents}: ${qc.watertight.connectedComponents}`));
    lines.push(panelText("quality-euler", `${labels.qcEuler}: ${qc.watertight.eulerCharacteristic}`));
    if (qc.watertight.genus !== null) lines.push(panelText("quality-genus", `${labels.qcGenus}: ${qc.watertight.genus}`));
    lines.push(panelText("quality-fallback", `${labels.qcClosedFallback}: ${qc.watertight.closedFallbackUsed}`));
  }
  qc.warnings.forEach((warning, index) => lines.push(panelText(`quality-warning-${index}`, `⚠️ ${warning}`)));
  return panelStack("quality", lines);
}

/** 🪟️ `ArtifactEditor::render`'s panel half — one `BuiltNode` document per registered panel tab, keyed by tab id. */
export function remodelPanelDocument(panelId: RemodelPanelId, scene: RemodelScene, config: RemodelConfig): BuiltNode {
  const labels = remodelLabelsFor(config.locale);
  switch (panelId) {
    case "media":
      return mediaPanel(scene, labels);
    case "results":
      return resultsPanel(scene, labels);
    case "parameters":
      return parametersPanel(scene, labels);
    case "calibration":
      return calibrationPanel(scene, labels);
    case "tracks":
      return tracksPanel(scene, labels);
    case "quality":
      return qualityPanel(scene, labels);
    default:
      return pipelinePanel(scene, config, labels);
  }
}
//#endregion 🔖️PanelDocuments

//#region 🔖️Reducer
/**
 * 🎮️ Story-local mirror of `command_from_action` → `RemodelingCommand::dispatch` for the config-only subset a
 * host surface can reach without a plugin runtime. Document-mutating commands (`addGcp`, `importFramePayload`,
 * `runReconstruction`, …) are deliberately NOT emulated: they emit artifact mutations through the event-sourced
 * store, which no story owns. An unrecognized action is ignored, exactly as `command_from_action` returning
 * `None` leaves the dispatch a no-op.
 */
export function reduceRemodelStoryAction(config: RemodelConfig, descriptor: ActionDescriptor): RemodelConfig {
  const args = remodelActionArgs(descriptor.args);
  switch (descriptor.action) {
    case "setCamera": {
      const camera = args.camera as { readonly position?: readonly [number, number, number]; readonly target?: readonly [number, number, number]; readonly fov?: number } | undefined;
      if (!camera) return config;
      return { ...config, camera: { position: camera.position ?? config.camera.position, target: camera.target ?? config.camera.target, fov: camera.fov ?? config.camera.fov } };
    }
    case "setLayerVisibility": {
      const layer = String(args.layer ?? "");
      if (!(layer in config.layers)) return config;
      return { ...config, layers: { ...config.layers, [layer]: args.visible === undefined ? !config.layers[layer as keyof RemodelConfig["layers"]] : Boolean(args.visible) } };
    }
    case "setFrameCursor":
      return { ...config, frameCursor: { streamId: typeof args.streamId === "string" ? args.streamId : null, frameIndex: Number(args.frameIndex ?? 0) } };
    case "setReportTable":
      return { ...config, reportTable: String(args.table ?? config.reportTable) };
    case "setActiveUtility":
      return { ...config, activeUtilityId: String(args.utilityId ?? config.activeUtilityId) };
    case "setLocale":
      return { ...config, locale: String(args.locale ?? config.locale) };
    default:
      return config;
  }
}
//#endregion 🔖️Reducer
