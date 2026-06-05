// #region 🧲Header
/** @emoji 🌍 `@infinite/world/r3f` — generic r3f infinite-world engine: layers, chunking, view radius, pooling, precision, LOD/grid, mesh borders. */
// #endregion 🧲Header

// #region 🔌Adapters
import { reactHostPort, sceneHostPort, type ReactNode } from "@ui/react";
import React, { Children, isValidElement, type CSSProperties, type MutableRefObject, type ReactElement } from "react";

const Canvas = sceneHostPort.fiber.canvas;
const useFrame = sceneHostPort.fiber.useFrame;
const useThree = sceneHostPort.fiber.useThree;
const OrbitControls = sceneHostPort.drei.OrbitControls;
const PerspectiveCamera = sceneHostPort.drei.PerspectiveCamera;
const {
  BufferGeometry,
  Color,
  EdgesGeometry,
  GridHelper,
  LineBasicMaterial,
  LineSegments,
  Matrix4,
  Mesh,
  MOUSE,
  Object3D,
  PerspectiveCamera: ThreePerspectiveCamera,
  Quaternion,
  Vector3,
} = sceneHostPort.three;
type Camera = import("three").Camera;
// #endregion 🔌Adapters

// #region 🔖Types
export type Vec3 = readonly [number, number, number];
export type Quat = readonly [number, number, number, number];

export interface LodMeshEntry {
  readonly lod: number;
  readonly url: string;
}

export interface LodGridLayer {
  readonly stepWorld: number;
  readonly opacity: number;
}

export interface WorldCameraState {
  readonly position: Vec3;
  readonly target: Vec3;
  readonly zoom: number;
}

export type SceneListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;
// #endregion 🔖Types

// #region 🔖EventBinding
/** @emoji 🎧 Tracks DOM listeners for deterministic teardown on world unmount. */
export class WorldEventBindingController {
  private readonly cleanups: Array<() => void> = [];

  listen(target: SceneListenerTarget | null | undefined, kind: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void {
    if (!target) return;
    target.addEventListener(kind, listener, options);
    this.cleanups.push(() => target.removeEventListener(kind, listener, options));
  }

  dispose(): void {
    while (this.cleanups.length > 0) {
      this.cleanups.pop()?.();
    }
  }
}
// #endregion 🔖EventBinding

// #region 🧭Precision
const _mCadToThree = new Matrix4();

/** @emoji 🧭 Identity matrix: CAD Z-up and Three.js scene share the same frame. */
export function cadToThreeMatrix(): Matrix4 {
  return _mCadToThree.identity();
}

/** @emoji 🧭 Maps a CAD fixture point to Three.js world coordinates (identity in z-up scenes). */
export function cadVec3ToThree(v: Vec3): Vec3 {
  return [v[0], v[1], v[2]];
}

/** @emoji 🧭 Maps a Three.js point back to CAD fixture coordinates (identity in z-up scenes). */
export function threeVec3ToCad(v: Vector3): Vec3 {
  return [v.x, v.y, v.z];
}

/** @emoji 🧭 Maps a CAD fixture quaternion to Three.js (identity in z-up scenes). */
export function cadQuatToThree(q: Quat): Quat {
  return [q[0], q[1], q[2], q[3]];
}

/** @emoji 🧭 Maps a Three.js quaternion back to CAD fixture coordinates (identity in z-up scenes). */
export function threeQuatToCad(q: Quaternion): Quat {
  return [q.x, q.y, q.z, q.w];
}

function quatRotateVec(q: Quat, v: Vec3): Vec3 {
  const out = new Vector3(v[0], v[1], v[2]).applyQuaternion(new Quaternion(q[0], q[1], q[2], q[3]));
  return [out.x, out.y, out.z];
}

function vec3Add(a: Vec3, b: Vec3): Vec3 {
  return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

/** @emoji 🧭 Maps object-local CAD offset to Three.js parent-group coordinates. */
export function cadObjectLocalToThreeGroupLocal(local: Vec3, originCad: Vec3, orientationCad: Quat | undefined): Vec3 {
  const q = orientationCad ?? ([0, 0, 0, 1] as Quat);
  const worldCad = vec3Add(originCad, quatRotateVec(q, local));
  const worldThree = new Vector3(...cadVec3ToThree(worldCad));
  const originThree = new Vector3(...cadVec3ToThree(originCad));
  const qThree = new Quaternion(...cadQuatToThree(q));
  const out = worldThree.sub(originThree).applyQuaternion(qThree.invert());
  return [out.x, out.y, out.z];
}

/** @emoji 🧭 Maps object-local CAD direction to a unit vector in the parent Three.js group. */
export function cadObjectLocalDirectionToThreeGroupLocal(localDir: Vec3, originCad: Vec3, orientationCad: Quat | undefined): Vec3 {
  const tip = cadObjectLocalToThreeGroupLocal(localDir, originCad, orientationCad);
  const base = cadObjectLocalToThreeGroupLocal([0, 0, 0], originCad, orientationCad);
  const d = new Vector3(tip[0] - base[0], tip[1] - base[1], tip[2] - base[2]);
  const len = d.length();
  if (len < 1e-9) {
    return [0, 0, 1];
  }
  d.divideScalar(len);
  return [d.x, d.y, d.z];
}

/** @emoji 🧭 +90° X: glTF Y-up mesh → CAD object-local Z-up inside the pose group. */
export const GLB_MESH_FRAME_ROTATION_X = Math.PI / 2;

/** @emoji 🎯 Rebases world positions near the camera to reduce floating-point jitter at large coordinates. */
export function floatingOriginRebase(worldCad: Vec3, anchorCad: Vec3): Vec3 {
  return [worldCad[0] - anchorCad[0], worldCad[1] - anchorCad[1], worldCad[2] - anchorCad[2]];
}

/** @emoji 📐 CAD anchor for grid layers: orbit pan XY, datum Z from {@link gridPlacementAnchorCad}. */
export function gridPlacementAnchorCad(controlsTargetThree: Vector3 | null | undefined, datum: Vec3 = [0, 0, 0]): Vec3 {
  if (!controlsTargetThree) {
    return datum;
  }
  const orbitCad = threeVec3ToCad(controlsTargetThree);
  return [orbitCad[0], orbitCad[1], datum[2]];
}
// #endregion 🧭Precision

// #region 🎨MeshBorder
/** @emoji 📏 UI normal border token for infinite-world mesh edge strokes ({@link borderNormalClass}). */
export const WORLD_MESH_BORDER_CSS = "var(--border-normal-color)";

/** @emoji 📏 Marks {@link LineSegments} added by {@link applyWorldMeshEdgeBorders}. */
export const WORLD_MESH_OUTLINE_USER_DATA_KEY = "__worldMeshBorderOutline";

const WORLD_MESH_BORDER_HEADLESS = "#808080";

let worldMeshBorderColorCache: string | null = null;

function probeWorldCssColor(property: "color" | "backgroundColor", expr: string): string {
  if (typeof document === "undefined") {
    return "";
  }
  const el = document.createElement("span");
  const key = property === "color" ? "color" : "background-color";
  el.setAttribute("style", `${key}:${expr};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
  if (document.documentElement.classList.contains("dark")) {
    el.classList.add("dark");
  }
  document.documentElement.appendChild(el);
  const out = getComputedStyle(el)[property];
  el.remove();
  return out;
}

function cssColorForThree(css: string): string {
  if (!css) {
    return css;
  }
  if (!/^(oklab|oklch|lab|lch|color)\(/iu.test(css)) {
    return css;
  }
  if (typeof document === "undefined") {
    return WORLD_MESH_BORDER_HEADLESS;
  }
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    return WORLD_MESH_BORDER_HEADLESS;
  }
  ctx.fillStyle = "#000000";
  ctx.fillStyle = css;
  const converted = ctx.fillStyle;
  if (/^(oklab|oklch|lab|lch|color)\(/iu.test(converted)) {
    return WORLD_MESH_BORDER_HEADLESS;
  }
  return converted;
}

function resolveWorldMeshBorderCssColor(): string {
  const raw = probeWorldCssColor("color", WORLD_MESH_BORDER_CSS);
  if (!raw || raw === "rgba(0, 0, 0, 0)") {
    return WORLD_MESH_BORDER_HEADLESS;
  }
  return cssColorForThree(raw);
}

/** @emoji 📏 Resolves {@link WORLD_MESH_BORDER_CSS} to an sRGB color string for Three.js materials. */
export function worldMeshBorderColor(): string {
  if (worldMeshBorderColorCache) {
    return worldMeshBorderColorCache;
  }
  worldMeshBorderColorCache = resolveWorldMeshBorderCssColor();
  return worldMeshBorderColorCache;
}

/** @emoji 🔄 Clears cached mesh border color (tests or theme switches). */
export function resetWorldMeshBorderColorCache(): void {
  worldMeshBorderColorCache = null;
}

/** @emoji 📏 Edge-segment outline for one mesh geometry using the UI normal border color. */
export function createWorldMeshEdgeOutline(geometry: BufferGeometry, borderColor?: string): LineSegments {
  const color = borderColor ?? worldMeshBorderColor();
  const outline = new LineSegments(new EdgesGeometry(geometry), new LineBasicMaterial({ color: new Color(cssColorForThree(color)) }));
  outline.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY] = true;
  outline.scale.setScalar(1.001);
  return outline;
}

/** @emoji 📏 Adds normal-border edge outlines to every mesh under `root` (idempotent per mesh). */
export function applyWorldMeshEdgeBorders(root: Object3D, borderColor?: string): void {
  const color = borderColor ?? worldMeshBorderColor();
  root.traverse((object) => {
    if (!(object instanceof Mesh)) {
      return;
    }
    const geometry = object.geometry;
    if (!geometry || object.children.some((child) => child.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY])) {
      return;
    }
    object.add(createWorldMeshEdgeOutline(geometry, color));
  });
}
// #endregion 🎨MeshBorder

// #region 🧱Chunking
/** @emoji 🧱 Stable chunk bucket key for a world-space origin. */
export function chunkKey(origin: Vec3, chunkSize: number): string {
  const ix = Math.floor(origin[0] / chunkSize);
  const iy = Math.floor(origin[1] / chunkSize);
  const iz = Math.floor(origin[2] / chunkSize);
  return `${ix}|${iy}|${iz}`;
}

function setEquals(a: ReadonlySet<string>, b: ReadonlySet<string>): boolean {
  if (a.size !== b.size) return false;
  for (const v of a) if (!b.has(v)) return false;
  return true;
}

/** @emoji 📏 Chunk bounding radius in world units (half-space diagonal of a cube chunk). */
export function chunkBoundsRadius(chunkSize: number): number {
  return chunkSize * 0.866;
}

/** @emoji 👁️ Distance-only chunk visibility with enter/exit hysteresis (avoids frustum-edge flicker). */
export function chunkDistanceVisible(args: {
  readonly camPos: Vector3;
  readonly chunkCenter: Vector3;
  readonly chunkSize: number;
  readonly maxDist: number;
  readonly wasVisible: boolean;
}): boolean {
  const boundsR = chunkBoundsRadius(args.chunkSize);
  const dist = args.camPos.distanceTo(args.chunkCenter);
  const enterDist = args.maxDist + boundsR;
  const exitDist = enterDist + args.chunkSize * 0.5;
  if (dist <= enterDist) return true;
  if (args.wasVisible && dist <= exitDist) return true;
  return false;
}

/** @emoji 👁️ Tracks which chunk keys are within view radius (per-frame, hysteresis). */
export function useVisibleChunkKeys(chunkKeys: Iterable<string>, chunkSize: number, maxDist: number): ReadonlySet<string> {
  const { camera } = useThree();
  const centerTmp = reactHostPort.useMemo(() => new Vector3(), []);
  const [visible, setVisible] = reactHostPort.useState<ReadonlySet<string>>(() => new Set());
  useFrame(() => {
    const camPos = camera.position;
    setVisible((prev) => {
      const next = new Set(prev);
      for (const key of chunkKeys) {
        const [ix, iy, iz] = key.split("|").map(Number);
        centerTmp.set((ix + 0.5) * chunkSize, (iy + 0.5) * chunkSize, (iz + 0.5) * chunkSize);
        const show = chunkDistanceVisible({
          camPos,
          chunkCenter: centerTmp,
          chunkSize,
          maxDist,
          wasVisible: next.has(key),
        });
        if (show) next.add(key);
        else next.delete(key);
      }
      return setEquals(prev, next) ? prev : next;
    });
  });
  return visible;
}

/** @emoji 🧩 Splits scene children with `origin` into chunked vs unchunked buckets. */
export function splitChunkedSceneChildren(children: ReactNode): { chunked: ReactNode[]; rest: ReactNode[] } {
  const chunked: ReactNode[] = [];
  const rest: ReactNode[] = [];
  Children.forEach(children, (c) => {
    if (isValidElement(c) && (c.props as { origin?: Vec3 }).origin !== undefined) chunked.push(c);
    else rest.push(c);
  });
  return { chunked, rest };
}

/** @emoji 🧱 Renders origin-tagged children grouped by {@link chunkKey} with view-radius visibility. */
export function WorldChunks(props: { readonly chunkSize: number; readonly maxDistance: number; readonly children: ReactNode }): ReactElement {
  const buckets = reactHostPort.useMemo(() => {
    const map = new Map<string, ReactNode[]>();
    Children.forEach(props.children, (child) => {
      if (!isValidElement(child)) return;
      const p = child.props as { origin?: Vec3 };
      if (!p?.origin) return;
      const k = chunkKey(p.origin, props.chunkSize);
      const arr = map.get(k) ?? [];
      arr.push(child);
      map.set(k, arr);
    });
    return map;
  }, [props.children, props.chunkSize]);

  const visible = useVisibleChunkKeys(buckets.keys(), props.chunkSize, props.maxDistance);
  return (
    <>
      {[...buckets].map(([key, items]) => (
        <group key={key} userData={{ sceneChunk: key }} visible={visible.has(key)}>
          {items}
        </group>
      ))}
    </>
  );
}

/** @emoji 🧩 Chunked scene subtree + unchunked sibling group (view-radius streaming). */
export function WorldChunkedSceneChildren(props: {
  readonly chunkSize: number;
  readonly maxDistance: number;
  readonly children: ReactNode;
  readonly unchunkedDataAttr?: string;
}): ReactElement {
  const { chunked, rest } = reactHostPort.useMemo(() => splitChunkedSceneChildren(props.children), [props.children]);
  const attr = props.unchunkedDataAttr ?? "data-world-unchunked";
  return (
    <>
      <WorldChunks chunkSize={props.chunkSize} maxDistance={props.maxDistance}>
        {chunked}
      </WorldChunks>
      <group {...{ [attr]: true }}>{rest}</group>
    </>
  );
}
// #endregion 🧱Chunking

// #region 👁️ViewRadius
export interface ViewRadiusLayerProps {
  readonly chunkSize: number;
  readonly maxDistance: number;
  readonly children: ReactNode;
}

/** @emoji 👁️ View-radius layer: alias for {@link WorldChunkedSceneChildren} in the layer stack. */
export function ViewRadiusLayer(props: ViewRadiusLayerProps): ReactElement {
  return <WorldChunkedSceneChildren chunkSize={props.chunkSize} maxDistance={props.maxDistance} children={props.children} />;
}
// #endregion 👁️ViewRadius

// #region 🏊Pool
/** @emoji 🏊 Generic refcount pool for asset keys (caller supplies template factory). */
export function createRefCountPool<TKey extends string>(): {
  acquire(key: TKey): void;
  release(key: TKey): void;
  count(key: TKey): number;
  delete(key: TKey): void;
  keys(): IterableIterator<TKey>;
} {
  const counts = new Map<TKey, number>();
  return {
    acquire(key) {
      counts.set(key, (counts.get(key) ?? 0) + 1);
    },
    release(key) {
      const n = (counts.get(key) ?? 1) - 1;
      if (n <= 0) counts.delete(key);
      else counts.set(key, n);
    },
    count(key) {
      return counts.get(key) ?? 0;
    },
    delete(key) {
      counts.delete(key);
    },
    keys() {
      return counts.keys();
    },
  };
}

/** @emoji 🏊 Template cache with refcount (style variants keyed by caller). */
export function createTemplatePool<TKey extends string>(): {
  acquire(key: TKey): void;
  release(key: TKey): void;
  getOrCreate(key: TKey, factory: () => Object3D): Object3D;
  delete(key: TKey): void;
  deleteByPrefix(prefix: string): void;
  has(key: TKey): boolean;
  peek(key: TKey): Object3D | undefined;
} {
  const refCounts = new Map<TKey, number>();
  const templates = new Map<TKey, Object3D>();
  return {
    acquire(key) {
      refCounts.set(key, (refCounts.get(key) ?? 0) + 1);
    },
    release(key) {
      const n = (refCounts.get(key) ?? 1) - 1;
      if (n <= 0) {
        refCounts.delete(key);
        templates.delete(key);
      } else {
        refCounts.set(key, n);
      }
    },
    getOrCreate(key, factory) {
      let template = templates.get(key);
      if (!template) {
        template = factory();
        templates.set(key, template);
      }
      return template;
    },
    delete(key) {
      refCounts.delete(key);
      templates.delete(key);
    },
    deleteByPrefix(prefix) {
      for (const key of [...templates.keys()]) {
        if (String(key).startsWith(prefix)) {
          templates.delete(key);
          refCounts.delete(key);
        }
      }
    },
    has(key) {
      return templates.has(key);
    },
    peek(key) {
      return templates.get(key);
    },
  };
}

/** @emoji 🔌 Port for leaf bundles that load assets (e.g. GLTF via drei). */
export interface AssetPoolPort {
  clear(url: string): void;
}
// #endregion 🏊Pool

// #region 📶Lod
export const DEFAULT_LOD_RANGE = { min: 0.01, max: 100_000 } as const;
export const DEFAULT_MANUAL_LOD = 100;
export const DEFAULT_GRID_PLANE_ANCHOR_CAD: Vec3 = [0, 0, 0];
export const DEFAULT_LOD_GRID_FACTOR = 10;
export const WORLD_LOD_SLIDER_MIN = 0;
export const WORLD_LOD_SLIDER_MAX = 1000;
export const WORLD_LOD_EPSILON = 0.01;
export const LOD_GRID_MAJOR_QUANTUM = 10;
export const LOD_GRID_MEDIUM_QUANTUM = 2.5;
export const LOD_GRID_SMALL_QUANTUM = 0.5;
export const LOD_GRID_MICRO_QUANTUM = 0.1;
export const WORLD_LOD_GRID_MAX_LOD = 1000;
export const WORLD_LOD_GRID_MEDIUM_MAX_LOD = 50;
export const WORLD_LOD_GRID_SMALL_MAX_LOD = 10;
export const WORLD_LOD_GRID_MICRO_MAX_LOD = 2;

/** @emoji 📶 Maps orbit camera distance to scene LOD (`distance / reference`). */
export function lodFromCameraDistance(distance: number, reference: number): number {
  const d = Math.max(distance, 1e-6);
  const ref = Math.max(reference, 1e-6);
  return d / ref;
}

/** @emoji 📶 Picks the closest available LOD; on log-distance ties prefers the smaller (more detailed) LOD. */
export function pickClosestLod(available: readonly number[], desired: number): number | null {
  if (!available.length || !Number.isFinite(desired) || desired <= 0) return null;
  let best = available[0]!;
  let bestDist = Math.abs(Math.log(best) - Math.log(desired));
  for (let i = 1; i < available.length; i++) {
    const rep = available[i]!;
    if (!Number.isFinite(rep) || rep <= 0) continue;
    const dist = Math.abs(Math.log(rep) - Math.log(desired));
    if (dist < bestDist - 1e-12 || (Math.abs(dist - bestDist) <= 1e-12 && rep < best)) {
      best = rep;
      bestDist = dist;
    }
  }
  return best;
}

/** @emoji 🎨 Resolves a mesh URL from per-LOD entries with {@link pickClosestLod}. */
export function pickClosestMeshUrl(entries: readonly LodMeshEntry[] | undefined, desired: number, fallback?: string): string | undefined {
  if (!entries?.length) return fallback;
  const lods = entries.map((e) => e.lod).filter((lod) => Number.isFinite(lod) && lod > 0);
  const picked = pickClosestLod(lods, desired);
  if (picked == null) return fallback;
  const match = entries.find((e) => e.lod === picked);
  return match?.url ?? fallback;
}

/** @emoji 📶 Formats scene LOD for host readouts. */
export function formatLod(lod: number): string {
  return Number.isFinite(lod) ? lod.toFixed(2) : "—";
}

/** @emoji 📶 Maps a linear slider position to log-spaced scene LOD. */
export function lodFromSliderValue(slider: number, range: { readonly min: number; readonly max: number } = DEFAULT_LOD_RANGE): number {
  const t = Math.max(0, Math.min(1, (slider - WORLD_LOD_SLIDER_MIN) / (WORLD_LOD_SLIDER_MAX - WORLD_LOD_SLIDER_MIN)));
  const logMin = Math.log(range.min);
  const logMax = Math.log(range.max);
  return Math.exp(logMin + t * (logMax - logMin));
}

/** @emoji 📶 Maps scene LOD to a linear slider position. */
export function sliderValueFromLod(lod: number, range: { readonly min: number; readonly max: number } = DEFAULT_LOD_RANGE): number {
  const clamped = Math.max(range.min, Math.min(range.max, lod));
  const logMin = Math.log(range.min);
  const logMax = Math.log(range.max);
  const t = (Math.log(clamped) - logMin) / (logMax - logMin);
  return Math.round(WORLD_LOD_SLIDER_MIN + t * (WORLD_LOD_SLIDER_MAX - WORLD_LOD_SLIDER_MIN));
}

/** @emoji 📐 Fixed LOD grid band steps in world units. */
export function lodGridBandStepsWorld(gridFactor: number): readonly [number, number, number, number] {
  const f = gridFactor;
  return [LOD_GRID_MAJOR_QUANTUM * f, LOD_GRID_MEDIUM_QUANTUM * f, LOD_GRID_SMALL_QUANTUM * f, LOD_GRID_MICRO_QUANTUM * f];
}

const LOD_GRID_LAYER_OPACITY = [1, 0.72, 0.48, 0.32] as const;

/** @emoji 📐 Progressive LOD grid layers (finer bands as LOD decreases). */
export function lodProgressiveGridLayers(lod: number, gridFactor: number): readonly LodGridLayer[] {
  if (!Number.isFinite(lod) || lod <= 0 || lod > WORLD_LOD_GRID_MAX_LOD) return [];
  const [large, medium, small, micro] = lodGridBandStepsWorld(gridFactor);
  const layers: LodGridLayer[] = [{ stepWorld: large, opacity: LOD_GRID_LAYER_OPACITY[0] }];
  if (lod <= WORLD_LOD_GRID_MEDIUM_MAX_LOD) layers.push({ stepWorld: medium, opacity: LOD_GRID_LAYER_OPACITY[1] });
  if (lod <= WORLD_LOD_GRID_SMALL_MAX_LOD) layers.push({ stepWorld: small, opacity: LOD_GRID_LAYER_OPACITY[2] });
  if (lod <= WORLD_LOD_GRID_MICRO_MAX_LOD) layers.push({ stepWorld: micro, opacity: LOD_GRID_LAYER_OPACITY[3] });
  return layers;
}

/** @emoji 📐 Finest visible LOD grid step in world units. */
export function lodGridStepWorld(lod: number, gridFactor: number): number | null {
  const layers = lodProgressiveGridLayers(lod, gridFactor);
  if (!layers.length) return null;
  return layers[layers.length - 1]!.stepWorld;
}

export interface LodContextValue {
  readonly lod: number;
  readonly depthVariable: boolean;
  readonly lodForWorldPosition: (position: Vec3) => number;
  readonly gridStepWorld: number | null;
  readonly gridFactor: number;
  readonly gridSnapEnabled: boolean;
}

const LodContext = reactHostPort.createContext<LodContextValue | null>(null);

/** @emoji 📶 Reads the live scene LOD band and grid snap step from world context. */
export function useLod(): LodContextValue {
  const v = reactHostPort.useContext(LodContext);
  if (!v) throw new Error("World LOD context missing");
  return v;
}

interface LodRuntimeCells {
  sceneLod: number;
  depthVariable: boolean;
  distanceReference: number;
  camera: Camera | null;
  tmpWorld: Vector3;
}

const worldRaycastNone: Object3D["raycast"] = () => undefined;

function applyLodGridLayerStyle(grid: GridHelper, opacity: number): void {
  const materials = Array.isArray(grid.material) ? grid.material : [grid.material];
  for (const raw of materials) {
    const mat = raw as LineBasicMaterial;
    mat.transparent = true;
    mat.opacity = opacity;
    mat.depthTest = true;
    mat.depthWrite = false;
  }
  grid.renderOrder = -5;
  grid.frustumCulled = false;
  grid.raycast = worldRaycastNone;
  grid.traverse((child) => {
    child.raycast = worldRaycastNone;
  });
}

/** @emoji 📐 Progressive multi-band world grid at the placement anchor. */
export function WorldLodGridHelper(props: { readonly gridDatum?: Vec3 }): ReactElement | null {
  const lod = useLod();
  const controls = useThree((s) => s.controls as { target?: Vector3 } | null);
  const anchor = controls?.target;
  const layers = reactHostPort.useMemo(() => lodProgressiveGridLayers(lod.lod, lod.gridFactor), [lod.lod, lod.gridFactor]);
  const grids = reactHostPort.useMemo(() => {
    const size = 12_000;
    return layers.map(({ stepWorld, opacity }) => {
      const divs = Math.min(512, Math.max(2, Math.round(size / stepWorld)));
      const grid = new GridHelper(size, divs, 0xb8c4d0, 0x6a7a8a);
      grid.rotation.x = Math.PI / 2;
      applyLodGridLayerStyle(grid, opacity);
      return grid;
    });
  }, [layers]);
  reactHostPort.useEffect(
    () => () => {
      for (const grid of grids) grid.dispose();
    },
    [grids],
  );
  if (!grids.length) return null;
  const placementCad = gridPlacementAnchorCad(anchor ?? null, props.gridDatum ?? DEFAULT_GRID_PLANE_ANCHOR_CAD);
  const [px, py, pz] = placementCad;
  return (
    <>
      {grids.map((grid, i) => (
        <primitive key={`${layers[i]?.stepWorld ?? i}`} object={grid} position={[px, py, pz + 0.002]} />
      ))}
    </>
  );
}

function LodFrameRunner(props: {
  readonly lodRef: MutableRefObject<number>;
  readonly lodRuntimeRef: MutableRefObject<LodRuntimeCells>;
  readonly distanceReference: number;
  readonly gridFactor: number;
  readonly gridSnapEnabled: boolean;
  readonly automaticLod: boolean;
  readonly depthVariableLod: boolean;
  readonly manualLod: number;
  readonly onLod: (patch: { readonly sceneLod: number; readonly depthVariable: boolean; readonly gridStepWorld: number | null }) => void;
  readonly onLodChange?: (lod: number) => void;
}): null {
  const cam = useThree((s) => s.camera);
  const controls = useThree((s) => s.controls as { target?: Vector3 } | null);
  const tmpT = reactHostPort.useMemo(() => new Vector3(), []);
  const prevLod = reactHostPort.useRef<number | null>(null);
  const ctxSig = reactHostPort.useRef("");
  useFrame(() => {
    const tgt = controls?.target ?? tmpT.set(0, 0, 0);
    const dist = cam.position.distanceTo(tgt);
    const autoLod = lodFromCameraDistance(dist, props.distanceReference);
    const sceneLod = props.automaticLod ? autoLod : props.depthVariableLod ? autoLod : props.manualLod;
    props.lodRef.current = sceneLod;
    const runtime = props.lodRuntimeRef.current;
    runtime.sceneLod = sceneLod;
    runtime.depthVariable = props.depthVariableLod;
    runtime.distanceReference = props.distanceReference;
    runtime.camera = cam;
    const gridStep = lodGridStepWorld(sceneLod, props.gridFactor);
    const sig = `${sceneLod}|${props.depthVariableLod ? 1 : 0}|${gridStep ?? "x"}|${props.gridFactor}|${props.gridSnapEnabled}`;
    if (ctxSig.current !== sig) {
      ctxSig.current = sig;
      props.onLod({ sceneLod, depthVariable: props.depthVariableLod, gridStepWorld: gridStep });
    }
    if (prevLod.current === null || Math.abs(prevLod.current - sceneLod) > WORLD_LOD_EPSILON) {
      prevLod.current = sceneLod;
      props.onLodChange?.(sceneLod);
    }
  });
  return null;
}

export interface WorldLodBridgeProps {
  readonly children: ReactNode;
  readonly lodRef: MutableRefObject<number>;
  readonly distanceReference: number;
  readonly gridFactor: number;
  readonly gridSnapEnabled: boolean;
  readonly showLodGrid: boolean;
  readonly automaticLod: boolean;
  readonly depthVariableLod: boolean;
  readonly manualLod: number;
  readonly gridDatum?: Vec3;
  readonly onLodChange?: (lod: number) => void;
}

/** @emoji 📶 LOD runtime bridge: context + optional {@link WorldLodGridHelper}. */
export function WorldLodBridge(props: WorldLodBridgeProps): ReactElement {
  const tmpWorld = reactHostPort.useMemo(() => new Vector3(), []);
  const lodRuntimeRef = reactHostPort.useRef<LodRuntimeCells>({
    sceneLod: DEFAULT_MANUAL_LOD,
    depthVariable: false,
    distanceReference: props.distanceReference,
    camera: null,
    tmpWorld,
  });
  const lodForWorldPosition = reactHostPort.useCallback((position: Vec3) => {
    const r = lodRuntimeRef.current;
    if (!r.depthVariable || !r.camera) return r.sceneLod;
    r.tmpWorld.set(position[0], position[1], position[2]);
    return lodFromCameraDistance(r.camera.position.distanceTo(r.tmpWorld), r.distanceReference);
  }, []);
  const [sceneLod, setLod] = reactHostPort.useState(DEFAULT_MANUAL_LOD);
  const [depthVariable, setDepthVariable] = reactHostPort.useState(false);
  const [gridStepWorld, setGridStepWorld] = reactHostPort.useState<number | null>(() => lodGridStepWorld(DEFAULT_MANUAL_LOD, props.gridFactor));
  const onLod = reactHostPort.useCallback((patch: { readonly sceneLod: number; readonly depthVariable: boolean; readonly gridStepWorld: number | null }) => {
    setLod((prev) => (Math.abs(prev - patch.sceneLod) > WORLD_LOD_EPSILON ? patch.sceneLod : prev));
    setDepthVariable((prev) => (prev === patch.depthVariable ? prev : patch.depthVariable));
    setGridStepWorld((prev) => (prev === patch.gridStepWorld ? prev : patch.gridStepWorld));
  }, []);
  const lodCtx = reactHostPort.useMemo<LodContextValue>(
    () => ({
      lod: sceneLod,
      depthVariable,
      lodForWorldPosition,
      gridStepWorld,
      gridFactor: props.gridFactor,
      gridSnapEnabled: props.gridSnapEnabled,
    }),
    [sceneLod, depthVariable, lodForWorldPosition, gridStepWorld, props.gridFactor, props.gridSnapEnabled],
  );
  return (
    <LodContext.Provider value={lodCtx}>
      <LodFrameRunner
        lodRef={props.lodRef}
        lodRuntimeRef={lodRuntimeRef}
        distanceReference={props.distanceReference}
        gridFactor={props.gridFactor}
        gridSnapEnabled={props.gridSnapEnabled}
        automaticLod={props.automaticLod}
        depthVariableLod={props.depthVariableLod}
        manualLod={props.manualLod}
        onLod={onLod}
        onLodChange={props.onLodChange}
      />
      {props.showLodGrid ? <WorldLodGridHelper gridDatum={props.gridDatum} /> : null}
      {props.children}
    </LodContext.Provider>
  );
}

/** @emoji 📐 Grid layer slot for {@link WorldLayerStack}. */
export function GridLayer(props: { readonly gridDatum?: Vec3 }): ReactElement | null {
  return <WorldLodGridHelper gridDatum={props.gridDatum} />;
}
// #endregion 📶Lod

// #region 🗂️LayerStack
const WorldLayerStackContext = reactHostPort.createContext(true);

export interface WorldLayerProps {
  readonly order: number;
  readonly name?: string;
  readonly children: ReactNode;
}

/** @emoji 🗂️ Ordered scene layer; renders in-place so React context (providers) stays valid. */
export function WorldLayer(props: WorldLayerProps): ReactElement {
  reactHostPort.useContext(WorldLayerStackContext);
  return (
    <group name={props.name} userData={{ worldLayerOrder: props.order }}>
      {props.children}
    </group>
  );
}

/** @emoji 🗂️ Root for {@link WorldLayer} children inside {@link WorldCanvas}. */
export function WorldLayerStack(props: { readonly children: ReactNode }): ReactElement {
  return <WorldLayerStackContext.Provider value={true}>{props.children}</WorldLayerStackContext.Provider>;
}
// #endregion 🗂️LayerStack

// #region 📷OrbitCameraView
/** @emoji 🧭 Standard orbit orthographic-style view ids for Z-up CAD scenes. */
export type OrbitCameraViewId =
  | "top"
  | "bottom"
  | "front"
  | "back"
  | "north"
  | "south"
  | "east"
  | "west"
  | "perspective";

/** @emoji 📡 Command name products should handle to apply {@link OrbitCameraViewId} presets. */
export const ORBIT_CAMERA_VIEW_COMMAND = "setOrbitCameraView";

const ORBIT_CAMERA_VIEW_LABELS: Record<OrbitCameraViewId, string> = {
  top: "Top",
  bottom: "Bottom",
  front: "Front",
  back: "Back",
  north: "North",
  south: "South",
  east: "East",
  west: "West",
  perspective: "Perspective",
};

const ORBIT_CAMERA_VIEW_DIRECTION: Record<OrbitCameraViewId, Vec3> = {
  top: [0, 0, 1],
  bottom: [0, 0, -1],
  front: [0, -1, 0],
  back: [0, 1, 0],
  north: [0, 1, 0],
  south: [0, -1, 0],
  east: [1, 0, 0],
  west: [-1, 0, 0],
  perspective: [0.75, -0.75, 0.55],
};

const ORBIT_CAMERA_VIEW_EPSILON = 1e-3;

function vec3Len(v: Vec3): number {
  return Math.hypot(v[0], v[1], v[2]);
}

function vec3Scale(v: Vec3, scale: number): Vec3 {
  return [v[0] * scale, v[1] * scale, v[2] * scale];
}

function vec3Normalize(v: Vec3): Vec3 {
  const len = vec3Len(v);
  if (len < ORBIT_CAMERA_VIEW_EPSILON) {
    return [0, 0, 1];
  }
  return vec3Scale(v, 1 / len);
}

function vec3Dot(a: Vec3, b: Vec3): number {
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/** @emoji 🧭 Nudges axis-aligned view directions off the orbit up vector to avoid gimbal lock. */
export function stabilizeOrbitViewDirection(direction: Vec3, up: Vec3 = [0, 0, 1]): Vec3 {
  const unit = vec3Normalize(direction);
  if (Math.abs(vec3Dot(unit, up)) < 0.995) {
    return unit;
  }
  return vec3Normalize([unit[0] + ORBIT_CAMERA_VIEW_EPSILON, unit[1] + 0.02, unit[2]]);
}

/** @emoji 📏 Distance from orbit position to target; falls back when degenerate. */
export function orbitCameraDistance(state: WorldCameraState, fallback = 600): number {
  const dx = state.position[0] - state.target[0];
  const dy = state.position[1] - state.target[1];
  const dz = state.position[2] - state.target[2];
  const distance = Math.hypot(dx, dy, dz);
  return distance > ORBIT_CAMERA_VIEW_EPSILON ? distance : fallback;
}

export interface ComputeOrbitCameraViewOptions {
  readonly target: Vec3;
  readonly distance?: number;
  readonly up?: Vec3;
  readonly zoom?: number;
}

/** @emoji 📷 Computes a Z-up orbit camera state for a named view around `target`. */
export function computeOrbitCameraViewState(view: OrbitCameraViewId, options: ComputeOrbitCameraViewOptions): WorldCameraState {
  const up = options.up ?? ([0, 0, 1] as Vec3);
  const distance = options.distance ?? 600;
  const direction = stabilizeOrbitViewDirection(ORBIT_CAMERA_VIEW_DIRECTION[view], up);
  const target = options.target;
  return {
    position: [target[0] + direction[0] * distance, target[1] + direction[1] * distance, target[2] + direction[2] * distance],
    target,
    zoom: options.zoom ?? 1,
  };
}

export interface OrbitCameraViewTemplateDescriptor {
  readonly id: string;
  readonly label: string;
  readonly controllerId: string;
  readonly command: string;
  readonly args: { readonly view: OrbitCameraViewId };
}

export interface CreateOrbitCameraViewTemplatesConfig {
  readonly controllerId: string;
  readonly command?: string;
  readonly views?: readonly OrbitCameraViewId[];
}

/** @emoji 🪟 Builds display-panel window templates for standard orbit camera views. */
export function createOrbitCameraViewTemplates(config: CreateOrbitCameraViewTemplatesConfig): readonly OrbitCameraViewTemplateDescriptor[] {
  const command = config.command ?? ORBIT_CAMERA_VIEW_COMMAND;
  const views = config.views ?? (["top", "front", "north", "perspective"] as const);
  return views.map((view) => ({
    id: view,
    label: ORBIT_CAMERA_VIEW_LABELS[view],
    controllerId: config.controllerId,
    command,
    args: { view },
  }));
}

/** @emoji 📷 Seeds orbit camera + target when `seedKey` changes (fixture presets, display templates). */
export function OrbitCameraViewSeed(props: {
  readonly camera: ThreePerspectiveCamera | null;
  readonly state: WorldCameraState;
  readonly seedKey: string | number;
}): null {
  const controls = useThree((s) => s.controls as { target: Vector3; update: () => void } | null);
  const lastSeedKey = reactHostPort.useRef<string | number | null>(null);
  reactHostPort.useLayoutEffect(() => {
    const camera = props.camera;
    if (!camera || lastSeedKey.current === props.seedKey) {
      return;
    }
    lastSeedKey.current = props.seedKey;
    const position = cadVec3ToThree(props.state.position);
    camera.position.set(position[0], position[1], position[2]);
    camera.updateProjectionMatrix();
    if (controls?.target) {
      const target = cadVec3ToThree(props.state.target);
      controls.target.set(target[0], target[1], target[2]);
      controls.update();
    }
  }, [controls, props.camera, props.seedKey, props.state.position, props.state.target]);
  return null;
}
// #endregion 📷OrbitCameraView

// #region 🎬WorldCanvas
type OrbitControlsBinding = {
  readonly mouseButtons: { LEFT: number | null; MIDDLE: number; RIGHT: number };
  readonly enabled: boolean;
  readonly update?: () => void;
};

/** @emoji 🎞️ Kicks one R3F frame when `frameloop="demand"`. */
export function DemandFrameloopKick(): null {
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [invalidate]);
  return null;
}

export interface WorldOrbitGatedProps {
  readonly camera?: ThreePerspectiveCamera | null;
  readonly zoom?: number;
  readonly onCamera?: (state: WorldCameraState) => void;
  readonly controlsGate?: boolean;
  readonly onCameraNavigate?: (active: boolean) => void;
}

/** @emoji 🛰️ Orbit controls with injectable gate (specializations disable during drag/tools). */
export function WorldOrbitGated(props: WorldOrbitGatedProps): ReactElement | null {
  const { camera: sceneCamera } = useThree();
  const camera = props.camera === undefined ? sceneCamera : props.camera;
  const controls = useThree((s) => s.controls as OrbitControlsBinding | null);
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const gate = props.controlsGate ?? false;
  const invalidate = useThree((s) => s.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [gate, invalidate]);
  reactHostPort.useLayoutEffect(() => {
    if (!controls) return;
    controls.mouseButtons.LEFT = null;
    controls.mouseButtons.MIDDLE = MOUSE.DOLLY;
    controls.mouseButtons.RIGHT = MOUSE.ROTATE;
    controls.update?.();
  }, [controls]);
  if (props.camera === null) return null;
  const reportCamera = () => {
    if (!props.onCamera || !camera) return;
    const tgt = controls?.target ?? targetScratch.set(0, 0, 0);
    props.onCamera({
      position: threeVec3ToCad(camera.position),
      target: threeVec3ToCad(tgt),
      zoom: props.zoom ?? 1,
    });
  };
  return (
    <OrbitControls
      {...(props.camera ? { camera: props.camera } : {})}
      makeDefault
      enabled={!gate}
      enableDamping={false}
      enablePan
      enableZoom
      onChange={() => invalidate()}
      onStart={() => {
        invalidate();
        props.onCameraNavigate?.(true);
      }}
      onEnd={() => {
        invalidate();
        props.onCameraNavigate?.(false);
        reportCamera();
      }}
      mouseButtons={{ LEFT: null as unknown as number, MIDDLE: MOUSE.DOLLY, RIGHT: MOUSE.ROTATE }}
    />
  );
}

/** @emoji 🔄 Keeps demand frameloop alive while the camera moves. */
export function WorldCameraInvalidator(): null {
  const { controls, camera } = useThree();
  const lastPos = reactHostPort.useRef(new Vector3());
  const lastTarget = reactHostPort.useRef(new Vector3());
  useFrame(({ invalidate }) => {
    const ctrl = controls as { readonly target?: Vector3 } | null;
    if (!ctrl) return;
    const target = ctrl.target;
    const moved = !camera.position.equals(lastPos.current) || (target ? !target.equals(lastTarget.current) : false);
    if (moved) {
      lastPos.current.copy(camera.position);
      if (target) lastTarget.current.copy(target);
      invalidate();
    }
  });
  return null;
}

export interface WorldCanvasProps {
  readonly children?: ReactNode;
  readonly className?: string;
  readonly style?: CSSProperties;
  readonly overlay?: ReactNode;
  readonly dataRootAttr?: string;
  readonly dataLod?: string;
  readonly rootRef?: React.RefObject<HTMLDivElement | null>;
  readonly onContextMenu?: (event: React.MouseEvent) => void;
  readonly extraRootProps?: Record<string, string | undefined>;
  readonly frameloop?: "always" | "demand";
  readonly background?: string;
  readonly cameraUp?: Vec3;
  readonly cameraPosition?: Vec3;
  readonly cameraFov?: number;
  readonly cameraNear?: number;
  readonly cameraFar?: number;
  readonly dpr?: number | [number, number];
  readonly shadows?: boolean | "basic" | "percentage" | "soft" | "variance";
  readonly gl?: React.ComponentProps<typeof Canvas>["gl"];
  readonly onCanvasReady?: (binding: { readonly camera: Camera; readonly domElement: HTMLCanvasElement }) => void;
  readonly onPointerDown?: (event: PointerEvent) => void;
  readonly onPointerMove?: (event: PointerEvent) => void;
  readonly onPointerUp?: (event: PointerEvent) => void;
  readonly onPointerLeave?: (event: PointerEvent) => void;
  readonly onPointerCancel?: (event: PointerEvent) => void;
  readonly onWheel?: (event: WheelEvent) => void;
  readonly onDoubleClick?: (event: MouseEvent) => void;
  readonly onLostPointerCapture?: (event: PointerEvent) => void;
}

/** @emoji 🌍 Generic infinite-world r3f canvas shell (`frameloop="demand"`). */
export function WorldCanvas(props: WorldCanvasProps): ReactElement {
  const extra = props.extraRootProps ?? {};
  const frameloop = props.frameloop ?? "demand";
  const cameraUp = props.cameraUp ?? ([0, 0, 1] as Vec3);
  const ownedCamera = props.cameraPosition !== undefined;
  return (
    <div
      ref={props.rootRef}
      className={props.className ?? "relative h-full w-full"}
      style={{ width: "100%", height: "100%", touchAction: "none", overscrollBehavior: "contain", ...props.style }}
      onContextMenu={props.onContextMenu}
      {...(props.dataRootAttr ? { [props.dataRootAttr]: true } : {})}
      {...(props.dataLod ? { "data-world-lod": props.dataLod } : {})}
      {...extra}
    >
      <Canvas
        frameloop={frameloop}
        style={{ height: "100%", width: "100%" }}
        dpr={props.dpr ?? [1, 2]}
        shadows={props.shadows}
        camera={
          ownedCamera
            ? {
                up: [...cameraUp] as [number, number, number],
                position: [...props.cameraPosition!] as [number, number, number],
                fov: props.cameraFov ?? 45,
                ...(props.cameraNear !== undefined ? { near: props.cameraNear } : {}),
                ...(props.cameraFar !== undefined ? { far: props.cameraFar } : {}),
              }
            : undefined
        }
        gl={props.gl ?? { antialias: true }}
        onPointerDown={(event) => props.onPointerDown?.(event.nativeEvent)}
        onPointerMove={(event) => props.onPointerMove?.(event.nativeEvent)}
        onPointerUp={(event) => props.onPointerUp?.(event.nativeEvent)}
        onPointerLeave={(event) => props.onPointerLeave?.(event.nativeEvent)}
        onPointerCancel={(event) => props.onPointerCancel?.(event.nativeEvent)}
        onWheel={(event) => props.onWheel?.(event.nativeEvent)}
        onContextMenu={(event) => props.onContextMenu?.(event.nativeEvent)}
        onDoubleClick={(event) => props.onDoubleClick?.(event.nativeEvent)}
        onLostPointerCapture={(event) => props.onLostPointerCapture?.(event.nativeEvent)}
        onCreated={({ camera, gl: renderer }) => props.onCanvasReady?.({ camera, domElement: renderer.domElement })}
      >
        {frameloop === "demand" ? <DemandFrameloopKick /> : null}
        {props.background ? <color attach="background" args={[props.background]} /> : null}
        <WorldLayerStack>{props.children}</WorldLayerStack>
      </Canvas>
      {props.overlay}
    </div>
  );
}

export { Canvas, PerspectiveCamera, useFrame, useThree, Vector3 };
// #endregion 🎬WorldCanvas

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("chunkKey", () => {
    it("buckets origins by chunk size", () => {
      expect(chunkKey([0, 0, 0], 256)).toBe("0|0|0");
      expect(chunkKey([256, 0, 0], 256)).toBe("1|0|0");
    });
  });

  describe("chunkDistanceVisible", () => {
    it("uses hysteresis for exit", () => {
      const cam = new Vector3(0, 0, 0);
      const center = new Vector3(500, 0, 0);
      expect(chunkDistanceVisible({ camPos: cam, chunkCenter: center, chunkSize: 256, maxDist: 100, wasVisible: false })).toBe(false);
      expect(chunkDistanceVisible({ camPos: cam, chunkCenter: center, chunkSize: 256, maxDist: 1000, wasVisible: true })).toBe(true);
    });
  });

  describe("computeOrbitCameraViewState", () => {
    it("places top view above the target on +Z with a stable offset", () => {
      const state = computeOrbitCameraViewState("top", { target: [0, 0, 40], distance: 800 });
      expect(state.target).toEqual([0, 0, 40]);
      expect(state.position[2]).toBeGreaterThan(state.target[2]);
      expect(Math.abs(state.position[0] - state.target[0])).toBeGreaterThan(0);
    });

    it("maps north to +Y and perspective to an oblique direction", () => {
      const north = computeOrbitCameraViewState("north", { target: [0, 0, 0], distance: 100 });
      expect(north.position[1]).toBeGreaterThan(0);
      const perspective = computeOrbitCameraViewState("perspective", { target: [0, 0, 0], distance: 100 });
      expect(Math.hypot(...perspective.position)).toBeGreaterThan(90);
    });
  });

  describe("createOrbitCameraViewTemplates", () => {
    it("emits display templates for the configured controller", () => {
      const templates = createOrbitCameraViewTemplates({ controllerId: "demo" });
      expect(templates.map((row) => row.id)).toEqual(["top", "front", "north", "perspective"]);
      expect(templates[0]).toMatchObject({ controllerId: "demo", command: ORBIT_CAMERA_VIEW_COMMAND, args: { view: "top" } });
    });
  });

  describe("floatingOriginRebase", () => {
    it("subtracts anchor", () => {
      expect(floatingOriginRebase([1000, 2000, 3], [1000, 1990, 0])).toEqual([0, 10, 3]);
    });
  });

  describe("cadVec3ToThree", () => {
    it("passes through z-up coordinates", () => {
      expect(cadVec3ToThree([1, 2, 3])).toEqual([1, 2, 3]);
      expect(threeVec3ToCad(new Vector3(4, 5, 6))).toEqual([4, 5, 6]);
    });
  });

  describe("lodFromCameraDistance", () => {
    it("maps orbit distance to scale ratio", () => {
      expect(lodFromCameraDistance(100, 100)).toBe(1);
      expect(lodFromCameraDistance(20000, 100)).toBe(200);
    });
  });

  describe("lodProgressiveGridLayers", () => {
    it("adds finer bands as lod decreases", () => {
      expect(lodProgressiveGridLayers(5000, 10)).toEqual([]);
      expect(lodProgressiveGridLayers(500, 10).map((l) => l.stepWorld)).toEqual([100]);
      expect(lodProgressiveGridLayers(50, 10).map((l) => l.stepWorld)).toEqual([100, 25]);
      expect(lodProgressiveGridLayers(2, 10).map((l) => l.stepWorld)).toEqual([100, 25, 5, 1]);
    });
  });

  describe("applyLodGridLayerStyle", () => {
    it("disables raycasting on grid helpers", () => {
      const grid = new GridHelper(100, 10);
      applyLodGridLayerStyle(grid, 0.5);
      expect(grid.raycast).toBe(worldRaycastNone);
      let childCount = 0;
      grid.traverse((child) => {
        childCount += 1;
        expect(child.raycast).toBe(worldRaycastNone);
      });
      expect(childCount).toBeGreaterThan(0);
    });
  });

  describe("worldMeshBorderColor", () => {
    it("returns an srgb-compatible color", () => {
      resetWorldMeshBorderColorCache();
      const color = worldMeshBorderColor();
      expect(color.length).toBeGreaterThan(0);
      expect(color).not.toMatch(/^oklab\(/iu);
    });
  });

  describe("applyWorldMeshEdgeBorders", () => {
    it("adds one outline child per mesh", () => {
      const { BoxGeometry } = sceneHostPort.three;
      const root = new Object3D();
      const mesh = new Mesh(new BoxGeometry(1, 1, 1), new LineBasicMaterial());
      root.add(mesh);
      applyWorldMeshEdgeBorders(root, "#336699");
      expect(mesh.children).toHaveLength(1);
      expect(mesh.children[0]?.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY]).toBe(true);
      applyWorldMeshEdgeBorders(root, "#336699");
      expect(mesh.children).toHaveLength(1);
    });
  });
}
// #endregion 🧪Tests
