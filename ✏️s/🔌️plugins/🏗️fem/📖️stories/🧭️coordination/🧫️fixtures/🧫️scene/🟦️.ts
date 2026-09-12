/// <reference types="vite/client" />
// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/🏗️fem/📖️stories/🧭️coordination/🧫️fixtures/🧫️scene/🟦️.ts
// Specs: The `🏗️fem` scope's shared story fixtures, scene-node projections, window-render summaries,
// bilingual (en/de) labels and story-local command emulators — everything the
// `stories/fem/**/*.stories.tsx` files need that is NOT itself a story.
// Summary: Lives beside the story files rather than inside them because Storybook's CSF indexer treats
// EVERY named export of a `*.stories.*` module as a story, so a shared helper exported from a story file
// would be indexed as a broken story (same split `../block/scene.ts` uses). The real shipped example
// documents (`📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`, one per artifact) are `?raw`-imported here and
// parsed by `./dsl.ts`; the scene projections are line-for-line ports of the windows' own Rust builders —
// `fem2d_structure_layers`/`screen_2d`/`vector_layer`
// (`🗿️artifacts/◻️2d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs`) and
// `fem3d_structural_instances`/`quat_z_to`/`quat_roll_z`/`quat_mul`/`mesh_box`
// (`🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` + `🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`).
// Two halves of those windows are deliberately NOT reproduced because they are solver/mesher output the
// browser cannot compute without fem's plugin wasm: fem2d's `mesh-edge-*` overlay
// (`fem2d_region_triangles` → `fem2d_mesh_preview`) and fem3d's `solid-*` boundary meshes
// (`fem3d_solid_mesh_entries` → `fem3d_mesh_preview`), plus every results-window layer derived from
// `fem2d_solve_all`/`fem3d_solve_all`. Each is reported as a counted omission in the story's debug
// readout rather than faked — see `femStoryOmissions`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { parseViewport2d, type Viewport2d } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";
import { parseViewport3dOrbit, type Viewport3dOrbit } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts";


import { EMPTY_FEM2D_SNAPSHOT, EMPTY_FEM3D_SNAPSHOT, parseFem2dDsl, parseFem3dDsl, type Fem2dSnapshot, type Fem3dSnapshot, type FemVec2 } from "../🧫️dsl/🟦️.ts";

import fem2dDemoDsl from "../../../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🎬️demo/🗣️.dsl.semio?raw";
import fem3dDemoDsl from "../../../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🎬️demo/🗣️.dsl.semio?raw";

//#region 🔖️Locale
/** 🗣️ Explicit OS locale supplied to the story host; artifact commands do not mutate it. */
export type FemStoryLocale = "en-US" | "de-DE";

export const FEM_STORY_LOCALES: readonly FemStoryLocale[] = ["en-US", "de-DE"];

export type FemStoryTextKey =
  | "model"
  | "results"
  | "viewer"
  | "language"
  | "example"
  | "nodes"
  | "members"
  | "supports"
  | "regions"
  | "solids"
  | "materials"
  | "sections"
  | "loadCases"
  | "combinations"
  | "deformationScale"
  | "activeCase"
  | "noLoadCase"
  | "displayMode"
  | "addNode"
  | "loadExample"
  | "clearExample"
  | "omitted";

const FEM_STORY_TEXT: Readonly<Record<FemStoryTextKey, Readonly<Record<FemStoryLocale, string>>>> = {
  model: { "en-US": "Model", "de-DE": "Modell" },
  results: { "en-US": "Results", "de-DE": "Ergebnisse" },
  viewer: { "en-US": "Viewer", "de-DE": "Betrachter" },
  language: { "en-US": "Language", "de-DE": "Sprache" },
  example: { "en-US": "Example", "de-DE": "Beispiel" },
  nodes: { "en-US": "Nodes", "de-DE": "Knoten" },
  members: { "en-US": "Members", "de-DE": "Stäbe" },
  supports: { "en-US": "Supports", "de-DE": "Auflager" },
  regions: { "en-US": "Regions", "de-DE": "Flächen" },
  solids: { "en-US": "Solids", "de-DE": "Volumen" },
  materials: { "en-US": "Materials", "de-DE": "Materialien" },
  sections: { "en-US": "Sections", "de-DE": "Querschnitte" },
  loadCases: { "en-US": "Load cases", "de-DE": "Lastfälle" },
  combinations: { "en-US": "Combinations", "de-DE": "Kombinationen" },
  deformationScale: { "en-US": "Deformation scale", "de-DE": "Verformungsmaßstab" },
  activeCase: { "en-US": "Active case", "de-DE": "Aktiver Lastfall" },
  noLoadCase: { "en-US": "No load case defined", "de-DE": "Kein Lastfall definiert" },
  displayMode: { "en-US": "Display mode", "de-DE": "Anzeigemodus" },
  addNode: { "en-US": "Add node", "de-DE": "Knoten hinzufügen" },
  loadExample: { "en-US": "Load example", "de-DE": "Beispiel laden" },
  clearExample: { "en-US": "Clear document", "de-DE": "Dokument leeren" },
  omitted: { "en-US": "Not rendered here", "de-DE": "Hier nicht gezeichnet" },
};

/** @emoji 🗣️ One label in the requested locale. Throws on an unknown key so a typo surfaces at render time instead of printing `undefined` into the panel. */
export function femStoryLabel(key: FemStoryTextKey, locale: FemStoryLocale): string {
  const entry = FEM_STORY_TEXT[key];
  if (!entry) throw new Error(`[fem-story] unknown label key ${JSON.stringify(key)}`);
  return entry[locale];
}
//#endregion 🔖️Locale

//#region 🔖️Fixtures
/** @emoji 🎬️ The single `📚️examples/*` unit each fem subset registers (`🌐️any/🦀️.rs`'s `examples()`), keyed by the id its `ExampleSource` carries. */
export const FEM2D_STORY_EXAMPLE_ID = "demo";

/** 🎬️ The registered example and an explicit empty-document selection. */
export const FEM3D_STORY_EXAMPLE_ID = "demo";
export const FEM3D_CLEARED_EXAMPLE_ID = "cleared";

export const FEM2D_DEMO_SNAPSHOT: Fem2dSnapshot = parseFem2dDsl(fem2dDemoDsl);
export const FEM3D_DEMO_SNAPSHOT: Fem3dSnapshot = parseFem3dDsl(fem3dDemoDsl);

/** @emoji 📨️ `ActionDescriptor.args` is declared `unknown`, so every story reducer narrows it here once instead of casting at each read. A non-object payload becomes an empty bag, exactly as a Rust handler sees no named arguments. */
export function femStoryActionArgs(args: unknown): Record<string, unknown> {
  return typeof args === "object" && args !== null && !Array.isArray(args) ? (args as Record<string, unknown>) : {};
}

/** @emoji 🪪️ Port of `crate::app_surface::next_id` (`⚙️engine/🖥️app-surface/🦀️.rs`): the smallest `"{prefix}{n}"` not already taken, starting at the current count. */
export function femNextId(existing: Iterable<string>, prefix: string): string {
  const ids = new Set(existing);
  let index = ids.size;
  while (ids.has(`${prefix}${index}`)) index += 1;
  return `${prefix}${index}`;
}
//#endregion 🔖️Fixtures

//#region 🔖️Omissions
/** @emoji 🕳️ One half of a window's Rust render this story cannot reproduce in the browser, with the reason — surfaced in every debug readout so a blank region is never mistaken for a bug. */
export type FemStoryOmission = { readonly id: string; readonly reason: string };

export const FEM2D_MESH_PREVIEW_OMISSION: FemStoryOmission = {
  id: "mesh-edge-*",
  reason: "fem2d_region_triangles → fem2d_mesh_preview is Rust-only (plugin wasm); the region mesh overlay is omitted rather than approximated",
};

export const FEM2D_SOLVER_OMISSION: FemStoryOmission = {
  id: "deformed-* / reaction-* / moment-* / contour-*",
  reason: "fem2d_solve_all is Rust-only (plugin wasm); every displacement/reaction/moment/von-Mises layer is omitted",
};

export const FEM3D_SOLID_MESH_OMISSION: FemStoryOmission = {
  id: "solid-*",
  reason: "fem3d_solid_mesh_entries → fem3d_mesh_preview is Rust-only (plugin wasm); solid boundary meshes are omitted",
};

export const FEM3D_SOLVER_OMISSION: FemStoryOmission = {
  id: "displacement offsets / von-Mises vertex colors",
  reason: "fem3d_solve_all is Rust-only (plugin wasm); the results scene renders the undeformed, uncolored structure",
};

/** @emoji 🕳️ The omission list as flat JSON for a `data-testid`'d debug panel. */
export function femStoryOmissions(omissions: readonly FemStoryOmission[]): readonly FemStoryOmission[] {
  return omissions;
}
//#endregion 🔖️Omissions

//#region 🔖️Fem2dScene
/** @emoji 📐️ `SCALE_2D` — model metres to screen pixels (`🪟️windows/🧱️model/🦀️.rs`). */
const FEM2D_SCALE = 20;
/** @emoji 📐️ `ORIGIN_2D` — screen-space offset so a structure anchored at (0,0) is not drawn at the canvas corner. */
const FEM2D_ORIGIN = 40;

/** @emoji 📐️ Port of `screen_2d`: model coordinates to the canvas' own screen space (y flipped). */
export function femScreen2d(x: number, y: number): FemVec2 {
  return [x * FEM2D_SCALE + FEM2D_ORIGIN, -y * FEM2D_SCALE + FEM2D_ORIGIN];
}

type FemLayer = Record<string, unknown>;

/** @emoji ➡️ Port of `vector_layer`: a two-point polyline from `origin` along `vector`, y negated. */
function femVectorLayer(id: string, origin: FemVec2, vector: FemVec2, color: string): FemLayer {
  return { kind: "polyline", id, points: [[origin[0], origin[1]], [origin[0] + vector[0], origin[1] - vector[1]]], color };
}

function femSign(value: number): number {
  return value < 0 ? -1 : 1;
}

/**
 * @emoji 🖼️ Port of `fem2d_structure_layers`: one circle per node, one line per member, one circle per
 * support and one red vector per load, in exactly the Rust order and with the Rust's own ids
 * (`node-<id>` / `el-<id>` / `support-<id>` / `load-<id>`). The three colors are the caller's, matching the
 * two Rust call sites: bright (`#38bdf8`/`#94a3b8`/`#f97316`) for the model window, a single muted
 * `#334155` for the results window's undeformed backdrop.
 */
export function fem2dStructureLayers(snapshot: Fem2dSnapshot, nodeColor: string, lineColor: string, supportColor: string): readonly FemLayer[] {
  const layers: FemLayer[] = [];
  const nodeById = new Map(snapshot.nodes.map((node) => [node.id, node]));
  for (const node of snapshot.nodes) {
    const [sx, sy] = femScreen2d(node.x, node.y);
    layers.push({ kind: "circle", id: `node-${node.id}`, x: sx - 4, y: sy - 4, width: 8, height: 8, color: nodeColor });
  }
  for (const element of snapshot.elements) {
    const start = nodeById.get(element.start);
    const end = nodeById.get(element.end);
    if (!start || !end) continue;
    const [x0, y0] = femScreen2d(start.x, start.y);
    const [x1, y1] = femScreen2d(end.x, end.y);
    layers.push({ kind: "line", id: `el-${element.id}`, x0, y0, x1, y1, color: lineColor });
  }
  for (const support of snapshot.supports) {
    const node = nodeById.get(support.nodeId);
    if (!node) continue;
    const [sx, sy] = femScreen2d(node.x, node.y);
    layers.push({ kind: "circle", id: `support-${support.id}`, x: sx - 5, y: sy - 5, width: 10, height: 10, color: supportColor });
  }
  for (const loadCase of snapshot.loadCases) {
    for (const load of loadCase.loads) {
      if (load.kind === "nodal") {
        const node = nodeById.get(load.nodeId ?? "");
        if (!node) continue;
        const value = load.value ?? 0;
        const vector: FemVec2 = load.dof === "Tx" ? [femSign(value) * 18, 0] : load.dof === "Ty" ? [0, femSign(value) * 18] : [0, -12];
        layers.push(femVectorLayer(`load-${load.id}`, femScreen2d(node.x, node.y), vector, "#ef4444"));
        continue;
      }
      if (load.kind === "memberUdl") {
        const element = snapshot.elements.find((entry) => entry.id === load.elementId);
        const start = element ? nodeById.get(element.start) : undefined;
        const end = element ? nodeById.get(element.end) : undefined;
        if (!start || !end) continue;
        layers.push(femVectorLayer(`load-${load.id}`, femScreen2d((start.x + end.x) * 0.5, (start.y + end.y) * 0.5), [femSign(load.wx ?? 0) * 18, femSign(load.wy ?? 0) * 18], "#ef4444"));
        continue;
      }
      if (load.kind === "area") {
        const region = snapshot.regions.find((entry) => entry.id === load.regionId);
        if (!region || region.outline.length === 0) continue;
        const sum = region.outline.reduce((accumulator, point) => [accumulator[0] + point[0], accumulator[1] + point[1]] as const, [0, 0] as FemVec2);
        const count = region.outline.length;
        layers.push(femVectorLayer(`load-${load.id}`, femScreen2d(sum[0] / count, sum[1] / count), [0, -femSign(load.pressure ?? 0) * 18], "#ef4444"));
      }
    }
  }
  return layers;
}

/** 🎥️ Initial navigation for a newly opened FEM 2D window. */
export const FEM2D_DEFAULT_CAMERA: Viewport2d = { x: 0, y: 0, zoom: 1 };

/** @emoji 📐️ The `canvas-2d` scene node a fem2d window renders, projected the way `crate::app_surface::canvas_2d_surface` encodes it — `surfaceId` is the window's own `BODY_KEY`. */
export function buildFem2dSceneNode(layers: readonly FemLayer[], camera: Viewport2d, bodyKey: string, controllerId: string) {
  return {
    type: "componentScene",
    surfaceId: bodyKey,
    controllerId,
    componentKind: "canvas-2d",
    canvas2d: { cameraX: camera.x, cameraY: camera.y, zoom: camera.zoom, layersJson: JSON.stringify(layers) },
  } as const;
}
//#endregion 🔖️Fem2dScene

//#region 🔖️Fem3dScene
/** @emoji 🧊️ `NODE_SIZE_3D` / `MEMBER_THICKNESS_3D` (`🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs`). */
const FEM3D_NODE_SIZE = 0.05;
const FEM3D_MEMBER_THICKNESS = 0.05;

type FemQuat = readonly [number, number, number, number];
type FemVec3 = readonly [number, number, number];

/** @emoji 🧭️ Port of `quat_mul`: Hamilton product `a * b`, both `[x,y,z,w]`. */
function quatMul(a: FemQuat, b: FemQuat): FemQuat {
  const [ax, ay, az, aw] = a;
  const [bx, by, bz, bw] = b;
  return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz];
}

/** @emoji 🧭️ Port of `quat_roll_z`: `roll` radians about the local +Z axis. */
function quatRollZ(roll: number): FemQuat {
  const half = roll / 2;
  return [0, 0, Math.sin(half), Math.cos(half)];
}

/** @emoji 🧭️ Port of `quat_z_to`: shortest-arc rotation taking local +Z (the `"box"` mesh's long axis) onto unit `dir`, with the antiparallel case flipped 180° about X. */
function quatZTo(dir: FemVec3): FemQuat {
  const dot = Math.min(1, Math.max(-1, dir[2]));
  if (dot > 0.999999) return [0, 0, 0, 1];
  if (dot < -0.999999) return [1, 0, 0, 0];
  const axisLength = Math.sqrt(dir[1] * dir[1] + dir[0] * dir[0]);
  const half = Math.acos(dot) / 2;
  const sin = Math.sin(half);
  return [(-dir[1] / axisLength) * sin, (dir[0] / axisLength) * sin, 0, Math.cos(half)];
}

/** @emoji 📦️ Port of `mesh_box(1,1,1)` + `compute_normals` (`🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`): six quads as twelve non-indexed triangles, so accumulated normals are flat per face. This is the `"box"` mesh `world3d_meshes_json_from_kinds(&["box"])` puts in every fem3d scene. */
function femUnitBoxMeshData(): { readonly positions: number[]; readonly normals: number[]; readonly indices: number[] } {
  const h = 0.5;
  const faces: readonly (readonly FemVec3[])[] = [
    [[-h, -h, h], [h, -h, h], [h, h, h], [-h, h, h]],
    [[h, -h, -h], [-h, -h, -h], [-h, h, -h], [h, h, -h]],
    [[-h, h, h], [h, h, h], [h, h, -h], [-h, h, -h]],
    [[-h, -h, -h], [h, -h, -h], [h, -h, h], [-h, -h, h]],
    [[h, -h, h], [h, -h, -h], [h, h, -h], [h, h, h]],
    [[-h, -h, -h], [-h, -h, h], [-h, h, h], [-h, h, -h]],
  ];
  const positions: number[] = [];
  const normals: number[] = [];
  const indices: number[] = [];
  const pushTriangle = (a: FemVec3, b: FemVec3, c: FemVec3): void => {
    const base = positions.length / 3;
    positions.push(a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]);
    const e0: FemVec3 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const e1: FemVec3 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    const raw: FemVec3 = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
    const length = Math.max(Math.hypot(raw[0], raw[1], raw[2]), 1e-8);
    for (let corner = 0; corner < 3; corner += 1) normals.push(raw[0] / length, raw[1] / length, raw[2] / length);
    indices.push(base, base + 1, base + 2);
  };
  for (const [a, b, c, d] of faces) {
    pushTriangle(a!, b!, c!);
    pushTriangle(a!, c!, d!);
  }
  return { positions, normals, indices };
}

export const FEM3D_BOX_MESHES: readonly Record<string, unknown>[] = [{ id: "box", data: femUnitBoxMeshData() }];

/**
 * @emoji 🧊️ Port of `fem3d_structural_instances`: one small box per node, then one oriented box prism per
 * `Bar`/`Frame` member — positioned at the midpoint, scaled `[t,t,length]` so the mesh's long local Z axis
 * stretches along the member, rotated by `quat_z_to(dir) * quat_roll_z(roll)`. Ids are the Rust's own
 * (`node-<id>` / `el-<id>`). `displacements` are always absent here (no solver in the browser), so this is
 * always the undeformed structure.
 */
export function fem3dStructuralInstances(snapshot: Fem3dSnapshot): readonly Record<string, unknown>[] {
  const instances: Record<string, unknown>[] = [];
  const nodeById = new Map(snapshot.nodes.map((node) => [node.id, node]));
  for (const node of snapshot.nodes) {
    instances.push({ id: `node-${node.id}`, meshId: "box", position: [node.x, node.y, node.z], rotation: [0, 0, 0, 1], scale: [FEM3D_NODE_SIZE, FEM3D_NODE_SIZE, FEM3D_NODE_SIZE], label: node.id });
  }
  for (const element of snapshot.elements) {
    const start = nodeById.get(element.start);
    const end = nodeById.get(element.end);
    if (!start || !end) continue;
    const delta: FemVec3 = [end.x - start.x, end.y - start.y, end.z - start.z];
    const length = Math.max(Math.hypot(delta[0], delta[1], delta[2]), 1e-9);
    const direction: FemVec3 = [delta[0] / length, delta[1] / length, delta[2] / length];
    const rotation = quatMul(quatZTo(direction), quatRollZ(element.kind === "frame" ? element.roll : 0));
    instances.push({
      id: `el-${element.id}`,
      meshId: "box",
      position: [(start.x + end.x) / 2, (start.y + end.y) / 2, (start.z + end.z) / 2],
      rotation,
      scale: [FEM3D_MEMBER_THICKNESS, FEM3D_MEMBER_THICKNESS, length],
      label: element.id,
    });
  }
  return instances;
}

/** 🎥️ Initial navigation for a newly opened FEM 3D window. */
export const FEM3D_INITIAL_VIEWPORT: Viewport3dOrbit = { position: [4, -4, 3], target: [0, 0, 0], zoom: 1 };

/** @emoji ✅️ `world3d_selection_json("rectangle", &[], None)`. */
const FEM3D_SELECTION_JSON = JSON.stringify({ method: "rectangle", mode: "replace", ids: [], hoveredId: null });

/** @emoji 🌞️ `world3d_environment_json(&WorldSunConfig::default())`. */
const FEM3D_ENVIRONMENT_JSON = JSON.stringify({ sun: { enabled: false, azimuth: 45, elevation: 35, intensity: 0.85, color: "#ffffff" } });

/** @emoji 🌐️ The `world-3d` scene node a fem3d window renders, projected the way `crate::app_surface::world_3d_surface` encodes it — `surfaceId` is the window's own body key. */
export function buildFem3dSceneNode(instances: readonly Record<string, unknown>[], camera: Viewport3dOrbit, bodyKey: string, controllerId: string) {
  return {
    type: "componentScene",
    surfaceId: bodyKey,
    controllerId,
    componentKind: "world-3d",
    world3d: {
      cameraJson: JSON.stringify(camera),
      meshesJson: JSON.stringify(FEM3D_BOX_MESHES),
      instancesJson: JSON.stringify(instances),
      selectionJson: FEM3D_SELECTION_JSON,
      environmentJson: FEM3D_ENVIRONMENT_JSON,
      interactionJson: JSON.stringify({ activeUtility: "select" }),
    },
  } as const;
}
//#endregion 🔖️Fem3dScene

//#region 🔖️ResultDisplay
/** 👁️ Result presentation owned by the current window and translated at the render boundary. */
export type FemStoryResultDisplay = { readonly sourceId: string | null; readonly mode: string; readonly modeIndex: number };

export const FEM_DEFAULT_RESULT_DISPLAY: FemStoryResultDisplay = { sourceId: null, mode: "static", modeIndex: 0 };

/** @emoji 📊️ The case id `render_static` resolves: the selected `sourceId` when the document knows it, else the first load case, else none. The Rust filters against `fem2d_solve_all`'s result keys; with no solver here the document's own case/combination ids stand in, which is the same set for a solvable document. */
export function femResolveResultCase(display: FemStoryResultDisplay, caseIds: readonly string[]): string | null {
  if (display.sourceId !== null && caseIds.includes(display.sourceId)) return display.sourceId;
  return caseIds[0] ?? null;
}
//#endregion 🔖️ResultDisplay

//#region 🔖️Fem2dReducer
export type Fem2dStoryState = {
  readonly exampleId: string;
  readonly locale: FemStoryLocale;
  readonly snapshot: Fem2dSnapshot;
  readonly camera: Viewport2d;
  readonly resultDisplay: FemStoryResultDisplay;
};

/** @emoji 🎬️ The state a fem2d session starts in for one example id — `"demo"` loads the bundled fixture, every other id an empty document, exactly as `setActiveExample`'s handler branches. */
export function fem2dStoryStateFor(exampleId: string, locale: FemStoryLocale): Fem2dStoryState {
  return {
    exampleId,
    locale,
    snapshot: exampleId === FEM2D_STORY_EXAMPLE_ID ? FEM2D_DEMO_SNAPSHOT : EMPTY_FEM2D_SNAPSHOT,
    camera: FEM2D_DEFAULT_CAMERA,
    resultDisplay: FEM_DEFAULT_RESULT_DISPLAY,
  };
}

/** 🧩️ Mirrors document commands and exact-window preferences while preserving the supplied OS locale. */
export function reduceFem2dStoryAction(state: Fem2dStoryState, action: string, rawArgs: unknown): Fem2dStoryState {
  const args = femStoryActionArgs(rawArgs);
  switch (action) {
    case "setActiveExample": {
      const exampleId = typeof args.exampleId === "string" ? args.exampleId : undefined;
      return exampleId === undefined ? state : { ...state, exampleId, snapshot: fem2dStoryStateFor(exampleId, state.locale).snapshot };
    }
    case "addNode": {
      const x = typeof args.x === "number" ? args.x : 0;
      const y = typeof args.y === "number" ? args.y : 0;
      const id = femNextId(
        state.snapshot.nodes.map((node) => node.id),
        "n",
      );
      return { ...state, snapshot: { ...state.snapshot, nodes: [...state.snapshot.nodes, { id, x, y }] } };
    }
    case "setCamera": {
      return { ...state, camera: parseViewport2d({ x: args.x, y: args.y, zoom: args.zoom }) };
    }
    case "setResultDisplay": {
      return {
        ...state,
        resultDisplay: {
          sourceId: typeof args.sourceId === "string" ? args.sourceId : null,
          mode: typeof args.mode === "string" ? args.mode : "static",
          modeIndex: typeof args.modeIndex === "number" ? args.modeIndex : 0,
        },
      };
    }
    default:
      return state;
  }
}
//#endregion 🔖️Fem2dReducer

//#region 🔖️Fem3dReducer
export type Fem3dStoryState = {
  readonly exampleId: string;
  readonly locale: FemStoryLocale;
  readonly snapshot: Fem3dSnapshot;
  readonly camera: Viewport3dOrbit;
  readonly resultDisplay: FemStoryResultDisplay;
};

/** 🎬️ Opens a story window with the registered example and explicit OS locale. */
export function fem3dStoryStateFor(exampleId: string, locale: FemStoryLocale): Fem3dStoryState {
  return {
    exampleId,
    locale,
    snapshot: exampleId === FEM3D_STORY_EXAMPLE_ID ? FEM3D_DEMO_SNAPSHOT : EMPTY_FEM3D_SNAPSHOT,
    camera: FEM3D_INITIAL_VIEWPORT,
    resultDisplay: FEM_DEFAULT_RESULT_DISPLAY,
  };
}

/** 🧩️ Mirrors document commands and exact-window preferences while preserving the supplied OS locale. */
export function reduceFem3dStoryAction(state: Fem3dStoryState, action: string, rawArgs: unknown): Fem3dStoryState {
  const args = femStoryActionArgs(rawArgs);
  switch (action) {
    case "setActiveExample": {
      const exampleId = typeof args.exampleId === "string" ? args.exampleId : undefined;
      return exampleId === undefined ? state : { ...state, exampleId, snapshot: fem3dStoryStateFor(exampleId, state.locale).snapshot };
    }
    case "addNode": {
      const x = typeof args.x === "number" ? args.x : 0;
      const y = typeof args.y === "number" ? args.y : 0;
      const z = typeof args.z === "number" ? args.z : 0;
      const id = femNextId(
        state.snapshot.nodes.map((node) => node.id),
        "n",
      );
      return { ...state, snapshot: { ...state.snapshot, nodes: [...state.snapshot.nodes, { id, x, y, z }] } };
    }
    case "setCamera": {
      const camera = args.camera;
      return { ...state, camera: parseViewport3dOrbit(camera) };
    }
    case "setResultDisplay": {
      return {
        ...state,
        resultDisplay: {
          sourceId: typeof args.sourceId === "string" ? args.sourceId : null,
          mode: typeof args.mode === "string" ? args.mode : "static",
          modeIndex: typeof args.modeIndex === "number" ? args.modeIndex : 0,
        },
      };
    }
    default:
      return state;
  }
}
//#endregion 🔖️Fem3dReducer

//#region 🔖️RenderSummaries
/** @emoji 📋️ The bilingual document readout every fem2d story shows beside its canvas — the counts the window's own scene is built from, so the panel stays assertable even when a layer kind is empty. */
export function fem2dSummaryLines(snapshot: Fem2dSnapshot, locale: FemStoryLocale): readonly string[] {
  return [
    `${femStoryLabel("nodes", locale)}: ${snapshot.nodes.length}`,
    `${femStoryLabel("members", locale)}: ${snapshot.elements.length}`,
    `${femStoryLabel("supports", locale)}: ${snapshot.supports.length}`,
    `${femStoryLabel("regions", locale)}: ${snapshot.regions.length}`,
    `${femStoryLabel("materials", locale)}: ${snapshot.materials.length}`,
    `${femStoryLabel("sections", locale)}: ${snapshot.sections.length}`,
    `${femStoryLabel("loadCases", locale)}: ${snapshot.loadCases.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("combinations", locale)}: ${snapshot.combinations.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("deformationScale", locale)}: ${snapshot.analysis.deformationScale}`,
  ];
}

/** @emoji 📋️ The fem3d counterpart, with `solids` in place of `regions`. */
export function fem3dSummaryLines(snapshot: Fem3dSnapshot, locale: FemStoryLocale): readonly string[] {
  return [
    `${femStoryLabel("nodes", locale)}: ${snapshot.nodes.length}`,
    `${femStoryLabel("members", locale)}: ${snapshot.elements.length}`,
    `${femStoryLabel("supports", locale)}: ${snapshot.supports.length}`,
    `${femStoryLabel("solids", locale)}: ${snapshot.solids.length}`,
    `${femStoryLabel("materials", locale)}: ${snapshot.materials.length}`,
    `${femStoryLabel("sections", locale)}: ${snapshot.sections.length}`,
    `${femStoryLabel("loadCases", locale)}: ${snapshot.loadCases.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("combinations", locale)}: ${snapshot.combinations.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("deformationScale", locale)}: ${snapshot.analysis.deformationScale}`,
  ];
}

/** @emoji 📊️ The results-window caption line: `render_static`'s resolved case, or its `"No load case defined"` placeholder branch. */
export function femResultCaptionLine(caseId: string | null, display: FemStoryResultDisplay, locale: FemStoryLocale): string {
  if (caseId === null) return femStoryLabel("noLoadCase", locale);
  const suffix = display.mode === "static" ? "" : ` #${display.modeIndex}`;
  return `${femStoryLabel("activeCase", locale)}: ${caseId} — ${femStoryLabel("displayMode", locale)} ${display.mode}${suffix}`;
}
//#endregion 🔖️RenderSummaries
