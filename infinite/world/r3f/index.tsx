// #region 🧲Header
/** @emoji 🌍 `@semio-tech/infinite-world-r3f` — generic r3f infinite-world engine: layers, chunking, view radius, pooling, precision, LOD/grid, mesh borders. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
  cn,
  floatingRibbonSurfaceClass,
  LevelProvider,
  reactHostPort,
  resolveSceneGizmoViewportPlacement,
  sceneHostPort,
  referenceMediaPort,
  Tree,
  UnifiedGumball,
  gumballConfigVisible,
  gumballHandleKindToTransformMode,
  useCanvasAppearanceSync,
  Icon,
  type GumballConfig,
  type GumballPlaneId,
  type GumballPose,
  type ReactNode,
  type ThreeEvent,
  type TreeDataItem,
} from "@semio-tech/ui-react";
import { clearColorResolveCache, resolveColorHex, resolveSpatialAxisColors, resolveThreeColor, semanticVar, themeColorVar, tokenHex, tokenVar } from "@semio-tech/ui-styling";
import React, { Children, isValidElement, type CSSProperties, type MutableRefObject, type ReactElement } from "react";
import { OrbitControls as ThreeOrbitControls } from "three/addons/controls/OrbitControls.js";
import { MeshBVH, type HitPointInfo } from "three-mesh-bvh";

const Canvas = sceneHostPort.fiber.canvas;
const useFrame = sceneHostPort.fiber.useFrame;
const useThree = sceneHostPort.fiber.useThree;
const GizmoHelper = sceneHostPort.drei.GizmoHelper;
const Grid = sceneHostPort.drei.Grid;
const OrthographicCamera = sceneHostPort.drei.OrthographicCamera;
const PerspectiveCamera = sceneHostPort.drei.PerspectiveCamera;
const {
  Box3,
  BoxGeometry,
  BufferGeometry,
  CanvasTexture,
  Color,
  DoubleSide,
  EdgesGeometry,
  Group,
  HalfFloatType,
  LineBasicMaterial,
  LineSegments,
  LinearFilter,
  LinearSRGBColorSpace,
  Matrix4,
  Mesh,
  MeshBasicMaterial,
  MOUSE,
  Object3D,
  OrthographicCamera: ThreeOrthographicCamera,
  PerspectiveCamera: ThreePerspectiveCamera,
  PlaneGeometry,
  Quaternion,
  Ray,
  RGBAFormat,
  Scene,
  ShaderMaterial,
  Vector2,
  Vector3,
  WebGLRenderTarget,
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

/** @emoji 📐 Orbit view projection mode for display templates. */
export type OrbitCameraProjection = "orthographic" | "perspective";

export interface WorldCameraState {
  readonly position: Vec3;
  readonly target: Vec3;
  readonly zoom: number;
  readonly up?: Vec3;
  readonly projection?: OrbitCameraProjection;
  /** @emoji 📐 Full classical-projection taxonomy spec (Parallel/Perspective families); see {@link WorldProjectionSpec}. */
  readonly projectionSpec?: WorldProjectionSpec;
}

/** @emoji 🧭 A camera-navigation gesture {@link classifyWorldNavigationGestures} can detect on an orbit control. */
export type WorldNavigationGesture = "pan" | "zoom" | "orbit";

/** @emoji 📸 The subset of camera state {@link classifyWorldNavigationGestures} diffs before/after a gesture. */
export interface WorldNavigationSnapshot {
  readonly position: Vec3;
  readonly target: Vec3;
  readonly zoom: number;
  readonly projection: OrbitCameraProjection;
}

/** @emoji 📏 Ratios/angle above which a camera-state delta counts as that gesture, tunable per caller. */
export interface WorldNavigationThresholds {
  readonly panRatio: number;
  readonly zoomRatio: number;
  readonly orbitRadians: number;
}

const WORLD_NAVIGATION_THRESHOLDS_DEFAULT: WorldNavigationThresholds = { panRatio: 0.02, zoomRatio: 0.03, orbitRadians: 0.05 };

function worldNavigationVec3Distance(a: Vec3, b: Vec3): number {
  return Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
}

function worldNavigationVec3Direction(from: Vec3, to: Vec3): Vec3 {
  const dx = to[0] - from[0];
  const dy = to[1] - from[1];
  const dz = to[2] - from[2];
  const length = Math.hypot(dx, dy, dz);
  return length > 1e-9 ? [dx / length, dy / length, dz / length] : [0, 0, 0];
}

/** @emoji 🧭 Classifies which navigation gestures a camera movement performed by diffing before/after
 * snapshots — pan (the orbit target moved), zoom (orthographic zoom factor or perspective dolly distance
 * changed), orbit (the camera's direction around the target rotated). A single drag may perform more than
 * one (e.g. a drag that pans while also slightly orbiting), so this returns a set, not one verdict. */
export function classifyWorldNavigationGestures(
  before: WorldNavigationSnapshot,
  after: WorldNavigationSnapshot,
  thresholds: Partial<WorldNavigationThresholds> = {},
): readonly WorldNavigationGesture[] {
  const { panRatio, zoomRatio, orbitRadians } = { ...WORLD_NAVIGATION_THRESHOLDS_DEFAULT, ...thresholds };
  const gestures: WorldNavigationGesture[] = [];
  const referenceDistance = Math.max(worldNavigationVec3Distance(before.position, before.target), 1);

  if (worldNavigationVec3Distance(before.target, after.target) > panRatio * referenceDistance) gestures.push("pan");

  const zoomDelta =
    before.projection === "orthographic"
      ? Math.abs(after.zoom / (before.zoom || 1) - 1)
      : Math.abs(worldNavigationVec3Distance(after.position, after.target) / referenceDistance - 1);
  if (zoomDelta > zoomRatio) gestures.push("zoom");

  const beforeDirection = worldNavigationVec3Direction(before.target, before.position);
  const afterDirection = worldNavigationVec3Direction(after.target, after.position);
  const dot = Math.min(1, Math.max(-1, beforeDirection[0] * afterDirection[0] + beforeDirection[1] * afterDirection[1] + beforeDirection[2] * afterDirection[2]));
  if (Math.acos(dot) > orbitRadians) gestures.push("orbit");

  return gestures;
}

export type SceneListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

/** @emoji 👁️ Persisted per-entity hide/lock flags shared by CAD and puzzle 3d. */
export interface WorldEntityFlags {
  readonly hidden?: boolean;
  readonly locked?: boolean;
}

/** @emoji 🔒 Opacity scale applied when an entity is locked but still rendered. */
export const WORLD_LOCKED_OPACITY_SCALE = 0.35;

/** @emoji 🔒 Desaturation factor applied when an entity is locked but still rendered. */
export const WORLD_LOCKED_DESATURATION = 0.55;

export interface WorldEntityRenderModeInput {
  readonly hovered?: boolean;
  readonly selected?: boolean;
  readonly revealed?: boolean;
}

export interface WorldEntityRenderMode {
  readonly visible: boolean;
  readonly asHover: boolean;
  readonly dim: boolean;
  readonly showSelectedOutline: boolean;
}

/** @emoji 👁️ Whether an entity participates in canvas pick, hover, and edit interactions.
 * Locked entities must not absorb raycasts — a click is equivalent to clicking the background
 * (deselect / pass-through to whatever is behind). */
export function worldEntitySelectable(flags: WorldEntityFlags | undefined): boolean {
  return flags?.hidden !== true && flags?.locked !== true;
}

/** @emoji 🔎 Whether an entity can appear in details/tree selection while locked.
 * Canvas pointer picking still uses {@link worldEntitySelectable} so locked clicks deselect like background. */
export function worldEntityInspectable(flags: WorldEntityFlags | undefined): boolean {
  return flags?.hidden !== true;
}

/** @emoji 👁️ Whether an entity should be drawn in the 3d scene. */
export function worldEntityRendered(flags: WorldEntityFlags | undefined, revealed = false): boolean {
  return flags?.hidden !== true || revealed;
}

/** @emoji 👁️ Resolves hover/dim/outline behavior for hidden-reveal and locked entities. */
export function worldEntityRenderMode(flags: WorldEntityFlags | undefined, input: WorldEntityRenderModeInput = {}): WorldEntityRenderMode {
  const revealed = input.revealed === true;
  const hovered = input.hovered === true;
  const selected = input.selected === true;
  const hidden = flags?.hidden === true;
  const locked = flags?.locked === true;
  const visible = worldEntityRendered(flags, revealed);
  const asHover = (hidden && revealed) || (hovered && !locked);
  const dim = locked && !hidden;
  const showSelectedOutline = selected && !locked && !hidden;
  return { visible, asHover, dim, showSelectedOutline };
}
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

// #endregion 🧭Precision

// #region Collision
/** @emoji 🧊 One mesh BVH part in pose-local coordinates (GLB frame rotation baked in). */
export interface CollisionMeshPart {
  readonly geometry: BufferGeometry;
  readonly bvh: MeshBVH;
  readonly localMatrix: Matrix4;
}

/** @emoji 🧊 Mesh-backed collision volume built from a GLB root. */
export interface CollisionBody {
  readonly parts: readonly CollisionMeshPart[];
  readonly localBounds: Box3;
}

const _collisionHit: HitPointInfo = { point: new Vector3(), distance: 0, faceIndex: 0 };
const _collisionBoxA = new Box3();
const _collisionBoxB = new Box3();
const _collisionInterBox = new Box3();
const _collisionSize = new Vector3();
const _collisionPoint = new Vector3();
const _collisionLocal = new Vector3();
const _collisionPartLocal = new Vector3();
const _collisionMatA = new Matrix4();
const _collisionMatB = new Matrix4();
const _collisionAtoB = new Matrix4();
const _collisionInvA = new Matrix4();
const _collisionInvB = new Matrix4();
const _collisionRay = new Ray();
const _collisionVa = new Vector3();
const _collisionVb = new Vector3();
const _collisionVc = new Vector3();
const _collisionNormal = new Vector3();
const _collisionToPoint = new Vector3();

/** @emoji 🧊 Builds a {@link CollisionBody} from a GLB root in pose-local space (includes {@link GLB_MESH_FRAME_ROTATION_X}). */
export function collisionBodyFromObject(root: Object3D): CollisionBody | null {
  const poseLocal = new Group();
  const frame = new Group();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.add(root.clone(true));
  poseLocal.add(frame);
  poseLocal.updateMatrixWorld(true);
  const poseInverse = poseLocal.matrixWorld.clone().invert();
  const parts: CollisionMeshPart[] = [];
  poseLocal.traverse((obj) => {
    const mesh = obj as Mesh;
    if (!mesh.isMesh || !mesh.geometry) {
      return;
    }
    const position = mesh.geometry.getAttribute("position");
    if (!position || position.count === 0) {
      return;
    }
    const geometry = mesh.geometry.clone();
    const meshToPose = new Matrix4().multiplyMatrices(poseInverse, mesh.matrixWorld);
    geometry.applyMatrix4(meshToPose);
    geometry.computeBoundingBox();
    const bvh = new MeshBVH(geometry);
    parts.push({ geometry, bvh, localMatrix: new Matrix4().identity() });
  });
  if (parts.length === 0) {
    return null;
  }
  const localBounds = new Box3();
  let hasBounds = false;
  for (const part of parts) {
    const partBox = new Box3();
    part.bvh.getBoundingBox(partBox);
    if (!Number.isFinite(partBox.min.x) || partBox.isEmpty()) {
      continue;
    }
    if (!hasBounds) {
      localBounds.copy(partBox);
      hasBounds = true;
    } else {
      localBounds.union(partBox);
    }
  }
  if (!hasBounds) {
    return null;
  }
  return { parts, localBounds };
}

/** @emoji 📦 World-space AABB for a posed {@link CollisionBody}. */
export function collisionBodyWorldBounds(body: CollisionBody, worldMatrix: Matrix4, target = new Box3()): Box3 {
  target.copy(body.localBounds).applyMatrix4(worldMatrix);
  return target;
}

function collisionPointInsidePart(part: CollisionMeshPart, pointPoseLocal: Vector3): boolean {
  _collisionPartLocal.copy(pointPoseLocal).applyMatrix4(part.localMatrix.clone().invert());
  const hit = part.bvh.closestPointToPoint(_collisionPartLocal, _collisionHit, 0, Infinity);
  if (!hit) {
    return false;
  }
  if (hit.distance < 1e-4) {
    return true;
  }
  const geometry = part.geometry;
  const index = geometry.index;
  const position = geometry.getAttribute("position");
  if (!index || !position) {
    return false;
  }
  const faceVertex = hit.faceIndex * 3;
  const ia = index.getX(faceVertex);
  const ib = index.getX(faceVertex + 1);
  const ic = index.getX(faceVertex + 2);
  _collisionVa.fromBufferAttribute(position, ia);
  _collisionVb.fromBufferAttribute(position, ib);
  _collisionVc.fromBufferAttribute(position, ic);
  _collisionNormal.subVectors(_collisionVb, _collisionVa).cross(_collisionToPoint.subVectors(_collisionVc, _collisionVa)).normalize();
  _collisionToPoint.subVectors(_collisionPartLocal, hit.point);
  return _collisionToPoint.dot(_collisionNormal) < 0;
}

function collisionPointInsideBody(body: CollisionBody, worldToPose: Matrix4, worldPoint: Vector3): boolean {
  _collisionLocal.copy(worldPoint).applyMatrix4(worldToPose);
  for (const part of body.parts) {
    if (collisionPointInsidePart(part, _collisionLocal)) {
      return true;
    }
  }
  return false;
}

function collisionBodiesContainmentOverlap(a: CollisionBody, worldA: Matrix4, b: CollisionBody, worldB: Matrix4): boolean {
  collisionBodyWorldBounds(a, worldA, _collisionBoxA);
  collisionBodyWorldBounds(b, worldB, _collisionBoxB);
  _collisionInterBox.copy(_collisionBoxA).intersect(_collisionBoxB);
  if (_collisionInterBox.isEmpty() || !Number.isFinite(_collisionInterBox.min.x)) {
    return false;
  }
  _collisionInterBox.getCenter(_collisionPoint);
  _collisionInvA.copy(worldA).invert();
  _collisionInvB.copy(worldB).invert();
  return collisionPointInsideBody(a, _collisionInvA, _collisionPoint) && collisionPointInsideBody(b, _collisionInvB, _collisionPoint);
}

/** @emoji 💥 True when two posed {@link CollisionBody} instances have intersecting mesh geometry. */
export function bodiesIntersect(a: CollisionBody, worldA: Matrix4, b: CollisionBody, worldB: Matrix4): boolean {
  collisionBodyWorldBounds(a, worldA, _collisionBoxA);
  collisionBodyWorldBounds(b, worldB, _collisionBoxB);
  if (!_collisionBoxA.intersectsBox(_collisionBoxB)) {
    return false;
  }
  for (const partA of a.parts) {
    _collisionMatA.multiplyMatrices(worldA, partA.localMatrix);
    for (const partB of b.parts) {
      _collisionMatB.multiplyMatrices(worldB, partB.localMatrix);
      _collisionAtoB.copy(_collisionMatB).invert().multiply(_collisionMatA);
      if (partB.bvh.intersectsGeometry(partA.geometry, _collisionAtoB)) {
        return true;
      }
    }
  }
  return collisionBodiesContainmentOverlap(a, worldA, b, worldB);
}

export interface SolidOverlapVolumeOptions {
  readonly sampleCount?: number;
}

/** @emoji 📐 Estimates solid overlap volume (m3) between two posed {@link CollisionBody} instances. */
export function solidOverlapVolume(a: CollisionBody, worldA: Matrix4, b: CollisionBody, worldB: Matrix4, opts?: SolidOverlapVolumeOptions): number {
  collisionBodyWorldBounds(a, worldA, _collisionBoxA);
  collisionBodyWorldBounds(b, worldB, _collisionBoxB);
  _collisionInterBox.copy(_collisionBoxA).intersect(_collisionBoxB);
  if (_collisionInterBox.isEmpty() || !Number.isFinite(_collisionInterBox.min.x)) {
    return 0;
  }
  if (!bodiesIntersect(a, worldA, b, worldB) && !collisionBodiesContainmentOverlap(a, worldA, b, worldB)) {
    return 0;
  }
  _collisionInterBox.getSize(_collisionSize);
  const boxVol = _collisionSize.x * _collisionSize.y * _collisionSize.z;
  if (boxVol <= 0) {
    return 0;
  }
  const sampleCount = opts?.sampleCount ?? Math.min(4096, Math.max(256, Math.round(boxVol * 200)));
  _collisionInvA.copy(worldA).invert();
  _collisionInvB.copy(worldB).invert();
  let insideBoth = 0;
  for (let i = 0; i < sampleCount; i += 1) {
    _collisionPoint.set(_collisionInterBox.min.x + Math.random() * _collisionSize.x, _collisionInterBox.min.y + Math.random() * _collisionSize.y, _collisionInterBox.min.z + Math.random() * _collisionSize.z);
    if (collisionPointInsideBody(a, _collisionInvA, _collisionPoint) && collisionPointInsideBody(b, _collisionInvB, _collisionPoint)) {
      insideBoth += 1;
    }
  }
  return (insideBoth / sampleCount) * boxVol;
}
// #endregion Collision

// #region 🎨MeshBorder
/** @emoji 📏 UI normal border token for infinite-world mesh edge strokes ({@link borderNormalClass}). */
export const WORLD_MESH_BORDER_CSS = semanticVar("border-normal-color");

/** @emoji 📏 Marks {@link LineSegments} added by {@link applyWorldMeshEdgeBorders}. */
export const WORLD_MESH_OUTLINE_USER_DATA_KEY = "__worldMeshBorderOutline";

let worldMeshBorderColorCache: string | null = null;

/** @emoji 📏 Resolves {@link WORLD_MESH_BORDER_CSS} to an sRGB color string for Three.js materials. */
export function worldMeshBorderColor(): string {
  if (worldMeshBorderColorCache) {
    return worldMeshBorderColorCache;
  }
  worldMeshBorderColorCache = resolveColorHex(WORLD_MESH_BORDER_CSS, "gray");
  return worldMeshBorderColorCache;
}

/** @emoji 🔄 Clears cached mesh border color (tests or theme switches). */
export function resetWorldMeshBorderColorCache(): void {
  worldMeshBorderColorCache = null;
  clearColorResolveCache();
}

/** @emoji 📏 Edge-segment outline for one mesh geometry using the UI normal border color. */
export function createWorldMeshEdgeOutline(geometry: BufferGeometry, borderColor?: string): LineSegments {
  const color = borderColor ?? worldMeshBorderColor();
  const outline = new LineSegments(new EdgesGeometry(geometry), new LineBasicMaterial({ color: new Color(resolveColorHex(color, "gray")) }));
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
export function chunkDistanceVisible(args: { readonly camPos: Vector3; readonly chunkCenter: Vector3; readonly chunkSize: number; readonly maxDist: number; readonly wasVisible: boolean }): boolean {
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
export function WorldChunkedSceneChildren(props: { readonly chunkSize: number; readonly maxDistance: number; readonly children: ReactNode; readonly unchunkedDataAttr?: string }): ReactElement {
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
export const WORLD_LOD_GRID_BASE_LOD = 2;
export const WORLD_LOD_GRID_MIN_FADE_CELLS = 24;
export const WORLD_LOD_GRID_FADE_HEIGHT_FACTOR = 32;
export const WORLD_LOD_GRID_FADE_STRENGTH = 1.5;
export const WORLD_ORBIT_CAMERA_MIN_FAR = 524_288;
export const WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR = 1024;

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

/** @emoji 📐 Sparse, unbounded LOD grid step using the configured spacing and regular 1–2.5–5 multipliers. */
export function lodGridStepWorld(lod: number, gridFactor: number): number | null {
  if (!Number.isFinite(lod) || lod <= 0 || !Number.isFinite(gridFactor) || gridFactor <= 0) return null;
  const targetMultiplier = Math.max(1, lod / WORLD_LOD_GRID_BASE_LOD);
  const magnitude = 10 ** Math.floor(Math.log10(targetMultiplier));
  const normalized = targetMultiplier / magnitude;
  const quantum = normalized <= 1 ? 1 : normalized <= 2.5 ? 2.5 : normalized <= 5 ? 5 : 10;
  return gridFactor * quantum * magnitude;
}

/** @emoji 🌫️ Stable world-space fade radius for the procedural grid before camera clipping becomes visible. */
export function cameraGridFadeDistance(camera: Camera, planeZ: number, stepWorld: number): number {
  const height = Math.abs(camera.position.z - planeZ);
  const target = Math.max(stepWorld * WORLD_LOD_GRID_MIN_FADE_CELLS, height * WORLD_LOD_GRID_FADE_HEIGHT_FACTOR);
  const cells = 2 ** Math.ceil(Math.log2(target / stepWorld));
  return Math.min(stepWorld * cells, camera.far * 0.25);
}

/** @emoji 📷 Quantized far clipping distance that follows arbitrarily large orbit distances. */
export function adaptiveOrbitCameraFar(distance: number): number {
  const target = Math.max(WORLD_ORBIT_CAMERA_MIN_FAR, Math.max(0, distance) * WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR);
  return 2 ** Math.ceil(Math.log2(target));
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

/** @emoji 📐 Sparse procedural world grid that follows the camera and fades without a finite edge. */
export function WorldLodGridHelper(props: { readonly gridDatum?: Vec3 }): ReactElement | null {
  const lod = useLod();
  const camera = useThree((s) => s.camera);
  const invalidate = useThree((s) => s.invalidate);
  const datum = props.gridDatum ?? DEFAULT_GRID_PLANE_ANCHOR_CAD;
  const stepWorld = lodGridStepWorld(lod.lod, lod.gridFactor);
  const [fadeDistance, setFadeDistance] = reactHostPort.useState(() => (stepWorld == null ? 1 : cameraGridFadeDistance(camera, datum[2], stepWorld)));
  const gridRef = reactHostPort.useRef<Mesh | null>(null);
  useFrame(() => {
    if (stepWorld == null) return;
    const next = cameraGridFadeDistance(camera, datum[2], stepWorld);
    setFadeDistance((previous) => (previous === next ? previous : next));
  });
  const [gridColor, setGridColor] = reactHostPort.useState<number>(() => resolveThreeColor(themeColorVar("element"), "gray"));
  useCanvasAppearanceSync(reactHostPort.useCallback(() => setGridColor(resolveThreeColor(themeColorVar("element"), "gray")), []));
  reactHostPort.useLayoutEffect(() => {
    const grid = gridRef.current;
    if (!grid) return;
    const materials = Array.isArray(grid.material) ? grid.material : [grid.material];
    for (const material of materials) {
      material.depthTest = true;
      material.depthWrite = false;
    }
    grid.renderOrder = -5;
    grid.raycast = worldRaycastNone;
    invalidate();
  }, [gridColor, invalidate]);
  if (stepWorld == null) return null;
  return (
    <Grid
      ref={gridRef}
      args={[2, 2]}
      position={[0, 0, datum[2] + 0.001]}
      rotation={[Math.PI / 2, 0, 0]}
      cellSize={stepWorld}
      cellThickness={0.6}
      cellColor={gridColor}
      sectionSize={stepWorld}
      sectionThickness={0}
      sectionColor={gridColor}
      fadeDistance={fadeDistance}
      fadeStrength={WORLD_LOD_GRID_FADE_STRENGTH}
      followCamera
      infiniteGrid
      side={DoubleSide}
    />
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
  const invalidate = useThree((s) => s.invalidate);
  const tmpT = reactHostPort.useMemo(() => new Vector3(), []);
  const ctxSig = reactHostPort.useRef("");
  useFrame(() => {
    const tgt = controls?.target ?? tmpT.set(0, 0, 0);
    const dist = cam.position.distanceTo(tgt);
    if (cam instanceof ThreePerspectiveCamera || cam instanceof ThreeOrthographicCamera) {
      const far = adaptiveOrbitCameraFar(dist);
      if (cam.far !== far) {
        cam.far = far;
        cam.updateProjectionMatrix();
        invalidate();
      }
    }
    const autoLod = lodFromCameraDistance(dist, props.distanceReference);
    const sceneLod = props.automaticLod ? autoLod : props.depthVariableLod ? autoLod : props.manualLod;
    props.lodRef.current = sceneLod;
    const runtime = props.lodRuntimeRef.current;
    runtime.sceneLod = sceneLod;
    runtime.depthVariable = props.depthVariableLod;
    runtime.distanceReference = props.distanceReference;
    runtime.camera = cam;
    const gridStep = lodGridStepWorld(sceneLod, props.gridFactor);
    const sig = `${props.depthVariableLod ? 1 : 0}|${gridStep ?? "x"}|${props.gridFactor}|${props.gridSnapEnabled}`;
    if (ctxSig.current !== sig) {
      ctxSig.current = sig;
      props.onLod({ sceneLod, depthVariable: props.depthVariableLod, gridStepWorld: gridStep });
      props.onLodChange?.(sceneLod);
      invalidate();
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
export type OrbitCameraViewId = "top" | "bottom" | "front" | "back" | "right" | "left" | "north" | "south" | "east" | "west" | "isometricNe" | "isometricNw" | "isometricSe" | "isometricSw" | "perspective" | "twoPointPerspective";

/** @emoji 📡 Command name products should handle to apply {@link OrbitCameraViewId} presets. */
export const ORBIT_CAMERA_VIEW_COMMAND = "setOrbitCameraView";

const ORBIT_CAMERA_VIEW_LABELS: Record<OrbitCameraViewId, string> = {
  top: "Top",
  bottom: "Below",
  front: "Front",
  back: "Back",
  right: "Right",
  left: "Left",
  north: "North",
  south: "South",
  east: "East",
  west: "West",
  isometricNe: "NE",
  isometricNw: "NW",
  isometricSe: "SE",
  isometricSw: "SW",
  perspective: "Perspective",
  twoPointPerspective: "Two Point Perspective",
};

const ORBIT_CAMERA_VIEW_DIRECTION: Record<OrbitCameraViewId, Vec3> = {
  top: [0, 0, 1],
  bottom: [0, 0, -1],
  front: [0, -1, 0],
  back: [0, 1, 0],
  right: [1, 0, 0],
  left: [-1, 0, 0],
  north: [0, 1, 0],
  south: [0, -1, 0],
  east: [1, 0, 0],
  west: [-1, 0, 0],
  isometricNe: [1, 1, 1],
  isometricNw: [-1, 1, 1],
  isometricSe: [1, -1, 1],
  isometricSw: [-1, -1, 1],
  perspective: [0.75, -0.75, 0.55],
  twoPointPerspective: [1, -1, 0],
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

const ORBIT_CAMERA_Z_UP: Vec3 = [0, 0, 1];

/** @emoji 📐 Maps a view id to orthographic vs perspective projection. */
export function orbitCameraProjectionForView(view: OrbitCameraViewId): OrbitCameraProjection {
  return view === "perspective" || view === "twoPointPerspective" ? "perspective" : "orthographic";
}

const ORBIT_CAMERA_VIEW_IDS: readonly OrbitCameraViewId[] = ["top", "bottom", "front", "back", "right", "left", "north", "south", "east", "west", "isometricNe", "isometricNw", "isometricSe", "isometricSw", "perspective", "twoPointPerspective"];

const ORBIT_CAMERA_TEMPLATE_BRANCH_VIEWS: Record<string, OrbitCameraViewId> = {
  orthographic: "top",
  "orthographic-2d": "top",
  "orthographic-3d": "isometricNe",
  isometry: "isometricNe",
};

/** @emoji 🪟 Maps a display-tree window template id to an {@link OrbitCameraViewId}. */
export function resolveOrbitCameraViewFromTemplateId(templateId: string): OrbitCameraViewId | null {
  if ((ORBIT_CAMERA_VIEW_IDS as readonly string[]).includes(templateId)) {
    return templateId as OrbitCameraViewId;
  }
  return ORBIT_CAMERA_TEMPLATE_BRANCH_VIEWS[templateId] ?? null;
}

function orbitCameraZoomForProjection(projection: OrbitCameraProjection, zoom?: number): number {
  if (projection === "orthographic") {
    return zoom !== undefined && zoom !== 1 ? zoom : 50;
  }
  return zoom ?? 1;
}

/** @emoji 🔎 Duck-typed camera zoom — `instanceof Three*Camera` fails across duplicate `three` package copies. */
export function worldCameraZoom(camera: Camera): number {
  const zoomable = camera as Camera & { readonly isPerspectiveCamera?: boolean; readonly isOrthographicCamera?: boolean; zoom?: number };
  if ((zoomable.isPerspectiveCamera || zoomable.isOrthographicCamera) && typeof zoomable.zoom === "number") {
    return zoomable.zoom;
  }
  return 1;
}

/** @emoji 🔎 Duck-typed orthographic check — same duplicate-`three` constraint as {@link worldCameraZoom}. */
export function worldCameraIsOrthographic(camera: Camera): boolean {
  return (camera as Camera & { readonly isOrthographicCamera?: boolean }).isOrthographicCamera === true;
}

/** @emoji 🔎 Duck-typed live perspective FOV — `undefined` for orthographic cameras, same duplicate-`three` constraint as {@link worldCameraZoom}. */
export function worldCameraFov(camera: Camera): number | undefined {
  const perspective = camera as Camera & { readonly isPerspectiveCamera?: boolean; readonly fov?: number };
  return perspective.isPerspectiveCamera && typeof perspective.fov === "number" ? perspective.fov : undefined;
}

/** @emoji 🔎 Zoom carried through a projection gizmo snap — preserves live parallel zoom; maps perspective's unit zoom onto the orthographic default. */
export function worldProjectionSnapZoom(pendingSpec: WorldProjectionSpec, currentZoom: number, currentIsOrthographic: boolean): number {
  const family = worldProjectionFamily(pendingSpec);
  if (family === "parallel") {
    return currentIsOrthographic ? currentZoom : orbitCameraZoomForProjection("orthographic", currentZoom);
  }
  return orbitCameraZoomForProjection("perspective", currentZoom);
}

/** @emoji 📷 Computes a Z-up orbit camera state for a named view around `target`. */
export function computeOrbitCameraViewState(view: OrbitCameraViewId, options: ComputeOrbitCameraViewOptions): WorldCameraState {
  const distance = options.distance ?? 600;
  const target = options.target;
  const projection = orbitCameraProjectionForView(view);
  const zoom = orbitCameraZoomForProjection(projection, options.zoom);
  switch (view) {
    case "top":
      return { position: [target[0], target[1], target[2] + distance], target, up: [0, 1, 0], zoom, projection };
    case "bottom":
      return { position: [target[0], target[1], target[2] - distance], target, up: [0, -1, 0], zoom, projection };
    case "front":
    case "south":
      return { position: [target[0], target[1] - distance, target[2]], target, up: ORBIT_CAMERA_Z_UP, zoom, projection };
    case "back":
    case "north":
      return { position: [target[0], target[1] + distance, target[2]], target, up: ORBIT_CAMERA_Z_UP, zoom, projection };
    case "right":
    case "east":
      return { position: [target[0] + distance, target[1], target[2]], target, up: ORBIT_CAMERA_Z_UP, zoom, projection };
    case "left":
    case "west":
      return { position: [target[0] - distance, target[1], target[2]], target, up: ORBIT_CAMERA_Z_UP, zoom, projection };
    default: {
      const direction = vec3Normalize(ORBIT_CAMERA_VIEW_DIRECTION[view]);
      return {
        position: [target[0] + direction[0] * distance, target[1] + direction[1] * distance, target[2] + direction[2] * distance],
        target,
        up: ORBIT_CAMERA_Z_UP,
        zoom,
        projection,
      };
    }
  }
}

type OrbitControlsTarget = { readonly target: Vector3; update: () => void };

function applyWorldCameraState(camera: Camera, state: WorldCameraState, controls: OrbitControlsTarget | null): void {
  const position = cadVec3ToThree(state.position);
  const target = cadVec3ToThree(state.target);
  camera.position.set(position[0], position[1], position[2]);
  const up = cadVec3ToThree(state.up ?? ORBIT_CAMERA_Z_UP);
  camera.up.set(up[0], up[1], up[2]);
  camera.lookAt(target[0], target[1], target[2]);
  if (controls?.target) {
    controls.target.set(target[0], target[1], target[2]);
    controls.update();
  }
  // Duck-type zoomable cameras — `instanceof` fails across duplicate `three` package copies.
  const zoomable = camera as Camera & { readonly isPerspectiveCamera?: boolean; readonly isOrthographicCamera?: boolean; zoom: number; updateProjectionMatrix: () => void };
  if (zoomable.isPerspectiveCamera || zoomable.isOrthographicCamera) {
    zoomable.zoom = state.zoom;
    zoomable.updateProjectionMatrix();
  }
}

/** @emoji 🧭 Maps a CAD viewport gizmo axis click to a named orbit view (Z-up). */
export function resolveOrbitGizmoViewFromDirection(direction: { readonly x: number; readonly y: number; readonly z: number }): OrbitCameraViewId {
  const spec = resolveProjectionGizmoSpec(resolveProjectionGizmoHitFromDirection(direction));
  return worldProjectionSpecToOrbitView(spec) ?? "top";
}

const WORLD_ORBIT_VIEW_SNAP_DURATION_MS = 280;

function easeInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - (-2 * t + 2) ** 3 / 2;
}

function lerpVec3(a: Vec3, b: Vec3, t: number): Vec3 {
  return [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
}

/** @emoji 📐 Keeps orbit pose while swapping orthographic vs perspective zoom defaults. */
export function applyOrbitProjectionToCameraState(state: WorldCameraState, projection: OrbitCameraProjection): WorldCameraState {
  return { ...state, projection, zoom: orbitCameraZoomForProjection(projection, state.zoom) };
}

/** @emoji 🎞️ Live projection-matrix morph driven by {@link WorldProjectionSnapDriver} — read by {@link WorldProjectionMatrixDriver} and
 * {@link WorldCurvilinearPass} so their per-frame effects ramp with the same tween instead of popping on remount. */
export interface WorldProjectionMorphState {
  readonly fromSpec: WorldProjectionSpec | undefined;
  readonly toSpec: WorldProjectionSpec;
  readonly fromMatrix: Matrix4;
  readonly fromFov: number;
  readonly toFov: number;
  eased: number;
  holding: boolean;
}

const WORLD_PROJECTION_MORPH_FALLBACK_REF: MutableRefObject<WorldProjectionMorphState | null> = { current: null };

export interface WorldOrbitViewSnapGateContextValue {
  readonly snapGate: boolean;
  readonly setSnapGate: (active: boolean) => void;
  readonly morphRef: MutableRefObject<WorldProjectionMorphState | null>;
}

const WorldOrbitViewSnapGateContext = reactHostPort.createContext<WorldOrbitViewSnapGateContextValue | null>(null);

/** @emoji 🧭 Enables {@link useWorldOrbitViewSnapGate} for orbit controls during gizmo view snaps. */
export function WorldOrbitViewSnapGateProvider(props: { readonly children?: ReactNode }): ReactElement {
  const [snapGate, setSnapGate] = reactHostPort.useState(false);
  const morphRef = reactHostPort.useRef<WorldProjectionMorphState | null>(null);
  const value = reactHostPort.useMemo(() => ({ snapGate, setSnapGate, morphRef }), [snapGate, morphRef]);
  return <WorldOrbitViewSnapGateContext.Provider value={value}>{props.children}</WorldOrbitViewSnapGateContext.Provider>;
}

/** @emoji 🧭 Reads the active gizmo view-snap gate for {@link WorldOrbitGated}. */
export function useWorldOrbitViewSnapGate(): WorldOrbitViewSnapGateContextValue {
  return reactHostPort.useContext(WorldOrbitViewSnapGateContext) ?? { snapGate: false, setSnapGate: () => {}, morphRef: WORLD_PROJECTION_MORPH_FALLBACK_REF };
}

export interface WorldOrbitViewGizmoProps {
  readonly show?: boolean;
  readonly projectionSpec?: WorldProjectionSpec;
  readonly onViewSelect?: (view: OrbitCameraViewId) => void;
  readonly onSpecSelect: (spec: WorldProjectionSpec) => void;
}

function resolveWorldGizmoNeutralColor(): string {
  return resolveColorHex(semanticVar("muted-foreground"), "gray");
}

type ProjectionGizmoVisualState = "idle" | "hover" | "dimmed";

interface ProjectionGizmoVisualPalette {
  readonly labelColor: string;
  readonly neutralHover: string;
  readonly brighten: string;
  readonly idleOpacity: number;
  readonly hoverOpacity: number;
  readonly dimmedOpacity: number;
}

function resolveProjectionGizmoVisualPalette(): ProjectionGizmoVisualPalette {
  return {
    labelColor: resolveColorHex(tokenVar("light"), "light"),
    neutralHover: resolveColorHex(themeColorVar("emphasized"), "dark"),
    brighten: resolveColorHex(tokenVar("light"), "light"),
    idleOpacity: 0.88,
    hoverOpacity: 1,
    dimmedOpacity: 0.4,
  };
}

function projectionGizmoHitsEqual(a: ProjectionGizmoHit, b: ProjectionGizmoHit): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}

function projectionGizmoHitVisualState(hit: ProjectionGizmoHit, hovered: ProjectionGizmoHit | null): ProjectionGizmoVisualState {
  if (!hovered) {
    return "idle";
  }
  return projectionGizmoHitsEqual(hit, hovered) ? "hover" : "dimmed";
}

function brightenProjectionGizmoColor(baseColor: string, accentColor: string, amount: number): string {
  const base = new Color(baseColor);
  const accent = new Color(accentColor);
  base.lerp(accent, amount);
  return `#${base.getHexString()}`;
}

function projectionGizmoHeadFillColor(baseColor: string, state: ProjectionGizmoVisualState, palette: ProjectionGizmoVisualPalette, neutral: boolean): string {
  if (state !== "hover") {
    return baseColor;
  }
  if (neutral) {
    return palette.neutralHover;
  }
  return brightenProjectionGizmoColor(baseColor, palette.brighten, 0.45);
}

function projectionGizmoHeadOpacity(state: ProjectionGizmoVisualState, palette: ProjectionGizmoVisualPalette): number {
  if (state === "hover") {
    return palette.hoverOpacity;
  }
  if (state === "dimmed") {
    return palette.dimmedOpacity;
  }
  return palette.idleOpacity;
}

//#region 🧭WorldOrbitViewGizmoViewport
export type ProjectionGizmoHit =
  | { readonly type: "face"; readonly axis: "x" | "y" | "z"; readonly sign: 1 | -1 }
  | { readonly type: "corner"; readonly quadrant: WorldAxonometricQuadrant; readonly hemisphere: WorldAxonometricHemisphere }
  | { readonly type: "center" };

interface WorldProjectionGizmoViewportProps {
  readonly axisColors: { readonly x: string; readonly y: string; readonly z: string };
  readonly neutralColor: string;
  readonly axisScale: [number, number, number];
  readonly font: string;
  readonly onHitSelect: (hit: ProjectionGizmoHit) => void;
}

function WorldOrbitViewGizmoViewportAxis(props: { readonly scale: [number, number, number]; readonly color: string; readonly rotation: [number, number, number]; readonly opacity?: number }): ReactElement {
  return (
    <group rotation={props.rotation}>
      <mesh position={[0.4, 0, 0]}>
        <boxGeometry args={props.scale} />
        <meshBasicMaterial color={props.color} toneMapped={false} transparent opacity={props.opacity ?? 0.72} />
      </mesh>
      <mesh position={[-0.4, 0, 0]}>
        <boxGeometry args={props.scale} />
        <meshBasicMaterial color={props.color} toneMapped={false} transparent opacity={(props.opacity ?? 0.72) * 0.92} />
      </mesh>
    </group>
  );
}

/** @emoji 📏 Thin shaft from the gizmo origin to a corner hit (matches axis shafts for the eight diagonals). */
function WorldProjectionGizmoCornerShaft(props: { readonly to: readonly [number, number, number]; readonly color: string; readonly opacity?: number }): ReactElement {
  const quaternion = reactHostPort.useMemo(() => {
    const dir = new Vector3(props.to[0], props.to[1], props.to[2]);
    const length = dir.length();
    if (length < 1e-6) {
      return new Quaternion();
    }
    dir.multiplyScalar(1 / length);
    return new Quaternion().setFromUnitVectors(new Vector3(1, 0, 0), dir);
  }, [props.to]);
  const length = Math.hypot(props.to[0], props.to[1], props.to[2]);
  return (
    <mesh position={[props.to[0] * 0.5, props.to[1] * 0.5, props.to[2] * 0.5]} quaternion={quaternion}>
      <boxGeometry args={[length * 0.88, 0.022, 0.022]} />
      <meshBasicMaterial color={props.color} toneMapped={false} transparent opacity={props.opacity ?? 0.72} />
    </mesh>
  );
}

function WorldProjectionGizmoHitHead(props: {
  readonly position: [number, number, number];
  readonly hit: ProjectionGizmoHit;
  readonly color: string;
  readonly label?: string;
  readonly font: string;
  readonly palette: ProjectionGizmoVisualPalette;
  readonly visualState: ProjectionGizmoVisualState;
  readonly neutral: boolean;
  readonly axisHeadScale: number;
  readonly onHitSelect: (hit: ProjectionGizmoHit) => void;
  readonly onPointerOver: (hit: ProjectionGizmoHit) => void;
  readonly onPointerOut: () => void;
}): ReactElement {
  const fillColor = projectionGizmoHeadFillColor(props.color, props.visualState, props.palette, props.neutral);
  const headOpacity = projectionGizmoHeadOpacity(props.visualState, props.palette);
  const texture = reactHostPort.useMemo(() => {
    const canvas = document.createElement("canvas");
    canvas.width = 64;
    canvas.height = 64;
    const context = canvas.getContext("2d");
    if (!context) {
      return new CanvasTexture(canvas);
    }
    context.beginPath();
    context.arc(32, 32, props.label ? 16 : 12, 0, 2 * Math.PI);
    context.closePath();
    context.fillStyle = fillColor;
    context.fill();
    if (props.label) {
      context.font = props.font;
      context.textAlign = "center";
      context.fillStyle = props.palette.labelColor;
      context.fillText(props.label, 32, props.label.length > 2 ? 38 : 41);
    }
    return new CanvasTexture(canvas);
  }, [fillColor, props.font, props.label, props.palette.labelColor]);
  reactHostPort.useEffect(() => () => texture.dispose(), [texture]);
  const scale = (props.label ? 1 : 0.65) * props.axisHeadScale * (props.visualState === "hover" ? 1.1 : 1);
  return (
    <sprite
      scale={scale}
      position={props.position}
      userData={{ projectionGizmoHit: props.hit }}
      onPointerOver={(event) => {
        event.stopPropagation();
        props.onPointerOver(props.hit);
      }}
      onPointerOut={(event) => {
        event.stopPropagation();
        props.onPointerOut();
      }}
      onPointerDown={(event) => {
        event.stopPropagation();
      }}
      onClick={(event) => {
        event.stopPropagation();
        const hit = (event.object.userData?.projectionGizmoHit as ProjectionGizmoHit | undefined) ?? props.hit;
        props.onHitSelect(hit);
      }}
    >
      <spriteMaterial map={texture} alphaTest={0.3} opacity={headOpacity} toneMapped={false} />
    </sprite>
  );
}

/** @emoji 🎯 Navigation cube with face, corner, and center hit targets for projection snapping. */
function WorldProjectionGizmoViewport(props: WorldProjectionGizmoViewportProps): ReactElement {
  const invalidate = useThree((state) => state.invalidate);
  const [hoveredHit, setHoveredHit] = reactHostPort.useState<ProjectionGizmoHit | null>(null);
  const [palette, setPalette] = reactHostPort.useState(() => resolveProjectionGizmoVisualPalette());
  useCanvasAppearanceSync(
    reactHostPort.useCallback(() => {
      clearColorResolveCache();
      setPalette(resolveProjectionGizmoVisualPalette());
    }, []),
  );
  reactHostPort.useEffect(() => {
    invalidate();
  }, [hoveredHit, invalidate, palette]);
  const axisHeadScale = 0.62;
  const headBase = {
    font: props.font,
    palette,
    axisHeadScale,
    onHitSelect: props.onHitSelect,
    onPointerOver: setHoveredHit,
    onPointerOut: () => setHoveredHit(null),
  };
  const visualState = (hit: ProjectionGizmoHit) => projectionGizmoHitVisualState(hit, hoveredHit);
  const shaftOpacity = hoveredHit ? 0.38 : 0.72;
  const axisOpacity = (axis: "x" | "y" | "z") => {
    if (!hoveredHit || hoveredHit.type !== "face" || hoveredHit.axis !== axis) {
      return shaftOpacity;
    }
    return 1;
  };
  const faceHit = (axis: "x" | "y" | "z", sign: 1 | -1): ProjectionGizmoHit => ({ type: "face", axis, sign });
  // 🧭 Upper corners keep labeled heads; lower corners match negative-axis ends (small unlabeled circles) — less relevant under-views.
  const cornerHits: readonly { readonly position: [number, number, number]; readonly hit: ProjectionGizmoHit; readonly label?: string }[] = [
    { position: [0.72, 0.72, 0.72], hit: { type: "corner", quadrant: "ne", hemisphere: "upper" }, label: "NE" },
    { position: [-0.72, 0.72, 0.72], hit: { type: "corner", quadrant: "nw", hemisphere: "upper" }, label: "NW" },
    { position: [0.72, -0.72, 0.72], hit: { type: "corner", quadrant: "se", hemisphere: "upper" }, label: "SE" },
    { position: [-0.72, -0.72, 0.72], hit: { type: "corner", quadrant: "sw", hemisphere: "upper" }, label: "SW" },
    { position: [0.72, 0.72, -0.72], hit: { type: "corner", quadrant: "ne", hemisphere: "lower" } },
    { position: [-0.72, 0.72, -0.72], hit: { type: "corner", quadrant: "nw", hemisphere: "lower" } },
    { position: [0.72, -0.72, -0.72], hit: { type: "corner", quadrant: "se", hemisphere: "lower" } },
    { position: [-0.72, -0.72, -0.72], hit: { type: "corner", quadrant: "sw", hemisphere: "lower" } },
  ];
  return (
    <group scale={22}>
      <WorldOrbitViewGizmoViewportAxis color={props.axisColors.x} rotation={[0, 0, 0]} scale={props.axisScale} opacity={axisOpacity("x")} />
      <WorldOrbitViewGizmoViewportAxis color={props.axisColors.y} rotation={[0, 0, Math.PI / 2]} scale={props.axisScale} opacity={axisOpacity("y")} />
      <WorldOrbitViewGizmoViewportAxis color={props.axisColors.z} rotation={[0, -Math.PI / 2, 0]} scale={props.axisScale} opacity={axisOpacity("z")} />
      {cornerHits.map((corner) => (
        <WorldProjectionGizmoCornerShaft key={`shaft-${corner.hit.quadrant}-${corner.hit.hemisphere}`} to={corner.position} color={props.neutralColor} opacity={shaftOpacity} />
      ))}
      <WorldProjectionGizmoHitHead position={[1, 0, 0]} hit={faceHit("x", 1)} label="X" color={props.axisColors.x} neutral={false} visualState={visualState(faceHit("x", 1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[-1, 0, 0]} hit={faceHit("x", -1)} color={props.axisColors.x} neutral={false} visualState={visualState(faceHit("x", -1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[0, 1, 0]} hit={faceHit("y", 1)} label="Y" color={props.axisColors.y} neutral={false} visualState={visualState(faceHit("y", 1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[0, -1, 0]} hit={faceHit("y", -1)} color={props.axisColors.y} neutral={false} visualState={visualState(faceHit("y", -1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[0, 0, 1]} hit={faceHit("z", 1)} label="Z" color={props.axisColors.z} neutral={false} visualState={visualState(faceHit("z", 1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[0, 0, -1]} hit={faceHit("z", -1)} color={props.axisColors.z} neutral={false} visualState={visualState(faceHit("z", -1))} {...headBase} />
      {cornerHits.map((corner) => (
        <WorldProjectionGizmoHitHead key={`${corner.hit.type}-${corner.hit.quadrant}-${corner.hit.hemisphere}`} position={corner.position} hit={corner.hit} label={corner.label} color={props.neutralColor} neutral visualState={visualState(corner.hit)} {...headBase} />
      ))}
      <WorldProjectionGizmoHitHead position={[0, 0, 0]} hit={{ type: "center" }} label="3D" color={props.neutralColor} neutral visualState={visualState({ type: "center" })} {...headBase} />
    </group>
  );
}
//#endregion 🧭WorldOrbitViewGizmoViewport

/** @emoji 🧭 CAD Z-up viewport navigation cube (faces / corners / center) anchored bottom-right above the folded projection pane. */
export function WorldOrbitViewGizmo(props: WorldOrbitViewGizmoProps): ReactElement | null {
  const { size } = useThree();
  const [axisColors, setAxisColors] = reactHostPort.useState(() => resolveSpatialAxisColors());
  const [neutralColor, setNeutralColor] = reactHostPort.useState(() => resolveWorldGizmoNeutralColor());
  const placement = reactHostPort.useMemo(() => resolveSceneGizmoViewportPlacement(size), [size]);
  const axisScale = reactHostPort.useMemo(() => [0.88, 0.036, 0.036] as [number, number, number], []);

  useCanvasAppearanceSync(
    reactHostPort.useCallback(() => {
      setAxisColors(resolveSpatialAxisColors());
      setNeutralColor(resolveWorldGizmoNeutralColor());
    }, []),
  );

  if (props.show === false) {
    return null;
  }
  return (
    <GizmoHelper alignment={placement.alignment} margin={placement.margin}>
      <WorldProjectionGizmoViewport
        axisColors={axisColors}
        neutralColor={neutralColor}
        axisScale={axisScale}
        font="12px Inter var, Arial, sans-serif"
        onHitSelect={(hit) => {
          const spec = resolveProjectionGizmoSpec(hit, props.projectionSpec);
          // 🧭 Skip only true no-operations (e.g. center while already on the same perspective kind).
          if (props.projectionSpec && worldProjectionSpecsEqual(spec, props.projectionSpec)) return;
          props.onSpecSelect(spec);
          const view = worldProjectionSpecToOrbitView(spec);
          if (view) {
            props.onViewSelect?.(view);
          }
        }}
      />
    </GizmoHelper>
  );
}

export interface WorldProjectionSnapDriverProps {
  readonly pendingSpec: WorldProjectionSpec | null;
  readonly currentProjectionSpec?: WorldProjectionSpec;
  readonly onPendingSpecClear: () => void;
  readonly onCameraChange?: (state: WorldCameraState) => void;
  readonly onSpecSnap?: (spec: WorldProjectionSpec, state: WorldCameraState) => void;
}

interface WorldProjectionSnapAnim {
  readonly spec: WorldProjectionSpec;
  readonly fromPos: Vector3;
  readonly fromUp: Vector3;
  readonly fromMatrix: Matrix4;
  readonly to: WorldCameraState;
  readonly toFov: number;
  readonly start: number;
  holding: boolean;
}

/** @emoji 🎞️ Interpolates the orbit camera to a {@link WorldProjectionSpec} when `pendingSpec` is set — morphs pose AND the
 * projection matrix itself (via {@link worldProjectionGoalMatrix} / {@link worldProjectionMorphMatrix}) over the same 280ms
 * tween, so perspective↔orthographic, FOV, oblique shear and two-point shift all interpolate instead of popping at remount. */
export function WorldProjectionSnapDriver(props: WorldProjectionSnapDriverProps): null {
  const { camera, size } = useThree();
  const controls = useThree((state) => state.controls as OrbitControlsTarget | null);
  const invalidate = useThree((state) => state.invalidate);
  const { setSnapGate, morphRef } = useWorldOrbitViewSnapGate();
  const animRef = reactHostPort.useRef<WorldProjectionSnapAnim | null>(null);

  reactHostPort.useEffect(() => {
    if (!props.pendingSpec || !camera || !controls?.target) {
      return;
    }
    const pendingSpec = props.pendingSpec;
    const fromSpec = props.currentProjectionSpec;
    const targetCad = threeVec3ToCad(controls.target);
    const fromZoom = worldCameraZoom(camera);
    const isOrthographic = worldCameraIsOrthographic(camera);
    const fromFov = worldCameraFov(camera);
    const to = worldProjectionTransitionPose(pendingSpec, {
      position: threeVec3ToCad(camera.position),
      target: targetCad,
      up: threeVec3ToCad(camera.up),
      zoom: fromZoom,
      isOrthographic,
      projectionSpec: fromSpec,
      viewport: size,
      fov: fromFov,
    });
    const toFov = worldProjectionPerspectiveFov(pendingSpec);
    animRef.current = { spec: pendingSpec, fromPos: camera.position.clone(), fromUp: camera.up.clone(), fromMatrix: camera.projectionMatrix.clone(), to, toFov, start: performance.now(), holding: false };
    morphRef.current = { fromSpec, toSpec: pendingSpec, fromMatrix: animRef.current.fromMatrix, fromFov: fromFov ?? worldProjectionPerspectiveFov(fromSpec ?? pendingSpec), toFov, eased: 0, holding: false };
    setSnapGate(true);
    props.onPendingSpecClear();
    invalidate();
  }, [camera, controls, invalidate, morphRef, props.currentProjectionSpec, props.pendingSpec, props.onPendingSpecClear, setSnapGate, size]);

  useFrame(() => {
    const anim = animRef.current;
    if (!anim || !camera || !controls) {
      return;
    }
    if (anim.holding) {
      const goal = worldProjectionGoalMatrix(anim.spec, { zoom: anim.to.zoom, fov: anim.toFov, viewport: size, near: 0.2, far: WORLD_ORBIT_CAMERA_MIN_FAR });
      camera.projectionMatrix.copy(goal);
      camera.projectionMatrixInverse.copy(goal).invert();
      invalidate();
      return;
    }
    const progress = Math.min(1, (performance.now() - anim.start) / WORLD_ORBIT_VIEW_SNAP_DURATION_MS);
    const eased = easeInOutCubic(progress);
    const nextPos = cadVec3ToThree(lerpVec3(threeVec3ToCad(anim.fromPos), anim.to.position, eased));
    const nextUp = cadVec3ToThree(lerpVec3(threeVec3ToCad(anim.fromUp), anim.to.up ?? ORBIT_CAMERA_Z_UP, eased));
    camera.position.set(nextPos[0], nextPos[1], nextPos[2]);
    camera.up.set(nextUp[0], nextUp[1], nextUp[2]);
    const target = cadVec3ToThree(anim.to.target);
    camera.lookAt(target[0], target[1], target[2]);
    controls.target.set(target[0], target[1], target[2]);
    controls.update();
    const goal = worldProjectionGoalMatrix(anim.spec, { zoom: anim.to.zoom, fov: anim.toFov, viewport: size, near: 0.2, far: WORLD_ORBIT_CAMERA_MIN_FAR });
    const morphed = worldProjectionMorphMatrix(anim.fromMatrix, goal, eased);
    camera.projectionMatrix.copy(morphed);
    camera.projectionMatrixInverse.copy(morphed).invert();
    if (morphRef.current) {
      morphRef.current.eased = eased;
    }
    if (progress < 1) {
      invalidate();
      return;
    }
    anim.holding = true;
    if (morphRef.current) {
      morphRef.current.eased = 1;
      morphRef.current.holding = true;
    }
    // Zoom/FOV fields are synced without `updateProjectionMatrix()` — that would rebuild the matrix from the camera's
    // stale left/right/top/bottom/aspect fields and stomp the pinned morph goal above.
    const zoomable = camera as Camera & { readonly isPerspectiveCamera?: boolean; readonly isOrthographicCamera?: boolean; zoom?: number; fov?: number };
    if (zoomable.isPerspectiveCamera || zoomable.isOrthographicCamera) {
      zoomable.zoom = anim.to.zoom;
    }
    if (zoomable.isPerspectiveCamera) {
      zoomable.fov = anim.toFov;
    }
    setSnapGate(false);
    props.onCameraChange?.(anim.to);
    props.onSpecSnap?.(anim.spec, anim.to);
    invalidate();
  });

  reactHostPort.useEffect(() => {
    if (animRef.current?.holding) {
      animRef.current = null;
    }
    if (morphRef.current?.holding) {
      morphRef.current = null;
    }
  }, [camera, morphRef]);

  return null;
}

export interface WorldOrbitViewSnapDriverProps {
  readonly pendingView: OrbitCameraViewId | null;
  readonly onPendingViewClear: () => void;
  readonly onCameraChange?: (state: WorldCameraState) => void;
  readonly onProjectionChange?: (projection: OrbitCameraProjection) => void;
  readonly onViewSnap?: (view: OrbitCameraViewId, state: WorldCameraState) => void;
}

/** @emoji 🎞️ Interpolates the orbit camera to a named view when `pendingView` is set. */
export function WorldOrbitViewSnapDriver(props: WorldOrbitViewSnapDriverProps): null {
  const pendingViewRef = reactHostPort.useRef<OrbitCameraViewId | null>(null);
  reactHostPort.useEffect(() => {
    if (props.pendingView) {
      pendingViewRef.current = props.pendingView;
    }
  }, [props.pendingView]);
  const pendingSpec = props.pendingView ? orbitViewToWorldProjectionSpec(props.pendingView) : null;
  return (
    <WorldProjectionSnapDriver
      pendingSpec={pendingSpec}
      onPendingSpecClear={props.onPendingViewClear}
      onCameraChange={(state) => {
        props.onCameraChange?.(state);
        const view = pendingViewRef.current;
        if (view) {
          props.onViewSnap?.(view, state);
          pendingViewRef.current = null;
        }
        if (state.projection) {
          props.onProjectionChange?.(state.projection);
        }
      }}
    />
  );
}

export interface WorldOrbitViewControlsProps {
  readonly show?: boolean;
  readonly projectionSpec?: WorldProjectionSpec;
  readonly externalPendingSpec?: WorldProjectionSpec | null;
  readonly onExternalPendingSpecClear?: () => void;
  readonly onCameraChange?: (state: WorldCameraState) => void;
  readonly onProjectionChange?: (projection: OrbitCameraProjection) => void;
  readonly onViewSnap?: (view: OrbitCameraViewId, state: WorldCameraState) => void;
  readonly onSpecSnap?: (spec: WorldProjectionSpec, state: WorldCameraState) => void;
  /** @emoji 🎞️ Reports the unified pane-or-gizmo pending spec so the host can pre-mount {@link WorldProjectionRig}'s
   * curvilinear pass before the camera remount (see `pendingSpec` there). */
  readonly onPendingSpecChange?: (spec: WorldProjectionSpec | null) => void;
}

/** @emoji 🧭 Bundles {@link WorldOrbitViewGizmo} and {@link WorldProjectionSnapDriver}. */
export function WorldOrbitViewControls(props: WorldOrbitViewControlsProps): ReactElement {
  const [internalPendingSpec, setInternalPendingSpec] = reactHostPort.useState<WorldProjectionSpec | null>(null);
  const pendingSpec = props.externalPendingSpec ?? internalPendingSpec;
  const clearPendingSpec = reactHostPort.useCallback(() => {
    if (props.externalPendingSpec) {
      props.onExternalPendingSpecClear?.();
      return;
    }
    setInternalPendingSpec(null);
  }, [props.externalPendingSpec, props.onExternalPendingSpecClear]);
  const onPendingSpecChange = props.onPendingSpecChange;
  reactHostPort.useEffect(() => {
    onPendingSpecChange?.(pendingSpec);
  }, [onPendingSpecChange, pendingSpec]);
  return (
    <>
      <WorldProjectionSnapDriver
        pendingSpec={pendingSpec}
        currentProjectionSpec={props.projectionSpec}
        onPendingSpecClear={clearPendingSpec}
        onCameraChange={props.onCameraChange}
        onSpecSnap={props.onSpecSnap}
      />
      <WorldOrbitViewGizmo show={props.show} projectionSpec={props.projectionSpec} onSpecSelect={setInternalPendingSpec} />
    </>
  );
}

export const WORLD_PROJECTION_KINDS: readonly WorldProjectionKind[] = ["orthographic", "axonometric", "oblique", "onePoint", "twoPoint", "threePoint", "curvilinear"];

export interface WorldProjectionModeOption {
  readonly id: string;
  readonly label: string;
  readonly spec: WorldProjectionSpec;
  readonly active: boolean;
}

/** @emoji 🔀 Non-spatial mode variants that sit beside the template tree (kept for callers that still want a flat list).
 * Prefer {@link createWorldProjectionTemplates} / {@link WorldProjectionKindSwitch} for the canonical taxonomy. */
export function worldProjectionModeOptions(spec: WorldProjectionSpec): readonly WorldProjectionModeOption[] {
  switch (spec.mode.kind) {
    case "axonometric": {
      const variants: readonly { readonly variant: WorldAxonometricVariant; readonly label: string; readonly angleA: number; readonly angleB: number }[] = [
        { variant: "isometric", label: "Iso", angleA: 30, angleB: 30 },
        { variant: "dimetric", label: "Di", angleA: 15, angleB: 15 },
        { variant: "trimetric", label: "Tri", angleA: 12, angleB: 42 },
      ];
      return variants.map((row) => ({
        id: `axonometric-${row.variant}`,
        label: row.label,
        spec: { mode: { kind: "axonometric", variant: row.variant, angleA: row.angleA, angleB: row.angleB }, orientation: spec.orientation },
        active: spec.mode.variant === row.variant,
      }));
    }
    case "oblique": {
      const variants: readonly { readonly variant: WorldObliqueVariant; readonly label: string; readonly depthScale: number }[] = [
        { variant: "cabinet", label: "Cab", depthScale: 0.5 },
        { variant: "cavalier", label: "Cav", depthScale: 1 },
        { variant: "military", label: "Mil", depthScale: 1 },
      ];
      return variants.map((row) => ({
        id: `oblique-${row.variant}`,
        label: row.label,
        spec: { mode: { kind: "oblique", variant: row.variant, angle: spec.mode.angle, depthScale: row.depthScale }, orientation: spec.orientation },
        active: spec.mode.variant === row.variant,
      }));
    }
    case "curvilinear":
      return (["fisheye", "panini"] as const).map((mapping) => ({
        id: `curvilinear-${mapping}`,
        label: mapping === "fisheye" ? "Fish" : "Pan",
        spec: { mode: { kind: "curvilinear", fov: spec.mode.fov, strength: spec.mode.strength, mapping }, orientation: spec.orientation },
        active: spec.mode.mapping === mapping,
      }));
    default:
      return [];
  }
}

/** @emoji 🌲 Template-tree node id for the active *mode* (orientation is gizmo-owned and does not change selection). */
export function worldProjectionTemplateSelectionId(spec: WorldProjectionSpec): string {
  switch (spec.mode.kind) {
    case "orthographic":
      return "orthographic";
    case "axonometric":
      return `axonometric-${spec.mode.variant}`;
    case "oblique":
      return `oblique-${spec.mode.variant}`;
    case "onePoint":
      return "one-point";
    case "twoPoint":
      return "two-point";
    case "threePoint":
      return "three-point";
    case "curvilinear":
      return "curvilinear";
  }
}

/** @emoji 🌲 Applies a template-tree *mode* while preserving the current gizmo orientation. */
export function worldProjectionTemplateApplySpec(templateSpec: WorldProjectionSpec, currentSpec?: WorldProjectionSpec): WorldProjectionSpec {
  return {
    mode: templateSpec.mode,
    orientation: currentSpec?.orientation ?? templateSpec.orientation,
  };
}

/** @emoji 🌲 Maps {@link createWorldProjectionTemplates} into selectable {@link TreeDataItem}s for the live projection pane. */
export function worldProjectionSwitchTreeItems(templates: readonly WorldProjectionTemplateDescriptor[], onSelect: (spec: WorldProjectionSpec) => void, currentSpec?: WorldProjectionSpec): TreeDataItem[] {
  return templates.map((template) => ({
    id: template.id,
    label: template.label,
    icon: <Icon icon={template.iconId} size={12} className="size-tiny shrink-0" />,
    defaultOpen: true,
    onClick: () => onSelect(worldProjectionTemplateApplySpec(template.args.spec, currentSpec)),
    ...(template.children?.length ? { items: worldProjectionSwitchTreeItems(template.children, onSelect, currentSpec) } : {}),
  }));
}

export interface WorldProjectionKindSwitchProps {
  readonly spec: WorldProjectionSpec;
  readonly onSpecChange: (spec: WorldProjectionSpec) => void;
  readonly className?: string;
}

/** @emoji 🌲 Projection-mode switcher — same Parallel/Perspective taxonomy as {@link createWorldProjectionTemplates}; cube owns spatial angles. */
export function WorldProjectionKindSwitch(props: WorldProjectionKindSwitchProps): ReactElement {
  const shellClass = props.className ?? cn("pointer-events-auto min-w-40 text-2xs font-medium", floatingRibbonSurfaceClass);
  const templates = reactHostPort.useMemo(() => createWorldProjectionTemplates({ controllerId: "projection-switch" }), []);
  const items = reactHostPort.useMemo(() => worldProjectionSwitchTreeItems(templates, props.onSpecChange, props.spec), [templates, props.onSpecChange, props.spec]);
  const selectedId = worldProjectionTemplateSelectionId(props.spec);
  return (
    <div className={shellClass} data-world-projection-kind-switch data-level="window">
      <LevelProvider level="window">
        <Tree showLines={false} sortableSections={false} selectionMode="single" selectedIds={[selectedId]} sections={[{ id: "projection-modes", items }]} />
      </LevelProvider>
    </div>
  );
}

export interface WorldOrbitProjectionSwitchProps {
  readonly projection: OrbitCameraProjection;
  readonly onProjectionChange: (projection: OrbitCameraProjection) => void;
  readonly className?: string;
  readonly spec?: WorldProjectionSpec;
  readonly onSpecChange?: (spec: WorldProjectionSpec) => void;
}

/** @emoji 🔀 Orthographic / perspective toggle, or full {@link WorldProjectionKindSwitch} when `spec` is provided. */
export function WorldOrbitProjectionSwitch(props: WorldOrbitProjectionSwitchProps): ReactElement {
  if (props.spec && props.onSpecChange) {
    return <WorldProjectionKindSwitch spec={props.spec} onSpecChange={props.onSpecChange} className={props.className} />;
  }
  const shellClass = props.className ?? cn("pointer-events-auto flex text-2xs font-medium", floatingRibbonSurfaceClass);
  const buttonClass = (active: boolean) => cn("px-2 py-1 transition-colors text-muted-foreground hover:text-emphasized", active && "text-emphasized");
  return (
    <div className={shellClass} data-world-projection-switch data-level="window">
      <button type="button" className={buttonClass(props.projection === "orthographic")} aria-pressed={props.projection === "orthographic"} onClick={() => props.onProjectionChange("orthographic")}>
        Ortho
      </button>
      <button type="button" className={buttonClass(props.projection === "perspective")} aria-pressed={props.projection === "perspective"} onClick={() => props.onProjectionChange("perspective")}>
        Persp
      </button>
    </div>
  );
}

export interface OrbitCameraViewTemplateDescriptor {
  readonly id: string;
  readonly label: string;
  readonly controllerId: string;
  readonly command: string;
  readonly args: { readonly view: OrbitCameraViewId };
  readonly children?: readonly OrbitCameraViewTemplateDescriptor[];
}

export interface CreateOrbitCameraViewTemplatesConfig {
  readonly controllerId: string;
  readonly command?: string;
  readonly views?: readonly OrbitCameraViewId[];
}

function orbitCameraViewTemplateLeaf(controllerId: string, command: string, view: OrbitCameraViewId, id = view, label = ORBIT_CAMERA_VIEW_LABELS[view]): OrbitCameraViewTemplateDescriptor {
  return { id, label, controllerId, command, args: { view } };
}

function orbitCameraViewTemplateBranch(controllerId: string, command: string, id: string, label: string, view: OrbitCameraViewId, children: readonly OrbitCameraViewTemplateDescriptor[]): OrbitCameraViewTemplateDescriptor {
  return { id, label, controllerId, command, args: { view }, children };
}

/** @emoji 🪟 Builds the orthographic/perspective window-template tree for the display panel. */
export function createOrbitCameraViewTemplates(config: CreateOrbitCameraViewTemplatesConfig): readonly OrbitCameraViewTemplateDescriptor[] {
  const command = config.command ?? ORBIT_CAMERA_VIEW_COMMAND;
  if (config.views) {
    return config.views.map((view) => orbitCameraViewTemplateLeaf(config.controllerId, command, view));
  }
  const orthographic2d: OrbitCameraViewTemplateDescriptor[] = (["top", "bottom", "front", "back", "right", "left"] as const).map((view) => orbitCameraViewTemplateLeaf(config.controllerId, command, view));
  const isometryCorners: OrbitCameraViewTemplateDescriptor[] = (["isometricNe", "isometricNw", "isometricSe", "isometricSw"] as const).map((view) => orbitCameraViewTemplateLeaf(config.controllerId, command, view));
  return [
    orbitCameraViewTemplateBranch(config.controllerId, command, "orthographic", "Orthographic", "top", [
      orbitCameraViewTemplateBranch(config.controllerId, command, "orthographic-2d", "2D", "top", orthographic2d),
      orbitCameraViewTemplateBranch(config.controllerId, command, "orthographic-3d", "3D", "isometricNe", [orbitCameraViewTemplateBranch(config.controllerId, command, "isometry", "Isometry", "isometricNe", isometryCorners)]),
    ]),
    orbitCameraViewTemplateBranch(config.controllerId, command, "perspective", "Perspective", "perspective", [orbitCameraViewTemplateLeaf(config.controllerId, command, "twoPointPerspective")]),
  ];
}

export interface OrbitCameraViewLayoutPane {
  readonly view: OrbitCameraViewId;
  readonly title?: string;
  readonly size?: number;
}

export type OrbitCameraViewLayoutArrangement =
  | { readonly kind: "stack"; readonly panes: readonly [OrbitCameraViewLayoutPane] }
  | { readonly kind: "row"; readonly panes: readonly OrbitCameraViewLayoutPane[] }
  | { readonly kind: "column"; readonly panes: readonly OrbitCameraViewLayoutPane[] }
  | { readonly kind: "grid"; readonly rows: readonly { readonly size?: number; readonly panes: readonly OrbitCameraViewLayoutPane[] }[] };

export interface OrbitCameraViewLayoutDescriptor {
  readonly id: string;
  readonly label: string;
  readonly groupPath: readonly string[];
  readonly arrangement: OrbitCameraViewLayoutArrangement;
}

function orbitLayoutPane(view: OrbitCameraViewId, title?: string, size?: number): OrbitCameraViewLayoutPane {
  return { view, ...(title ? { title } : {}), ...(size !== undefined ? { size } : {}) };
}

function orbitLayoutSingle(view: OrbitCameraViewId, group: string): OrbitCameraViewLayoutDescriptor {
  return {
    id: `view-single-${view}`,
    label: ORBIT_CAMERA_VIEW_LABELS[view],
    groupPath: ["Single", group],
    arrangement: { kind: "stack", panes: [orbitLayoutPane(view)] },
  };
}

/** @emoji 🧭 Catalog of reusable orbit-view window arrangements for named layouts. */
export function createOrbitCameraViewLayoutDescriptors(): readonly OrbitCameraViewLayoutDescriptor[] {
  const singles2d = (["top", "bottom", "front", "back", "right", "left"] as const).map((view) => orbitLayoutSingle(view, "2D"));
  const singles3d = (["isometricNe", "isometricNw", "isometricSe", "isometricSw", "perspective", "twoPointPerspective"] as const).map((view) => orbitLayoutSingle(view, "3D"));
  const dualRow = (id: string, label: string, group: string, panes: readonly OrbitCameraViewLayoutPane[]): OrbitCameraViewLayoutDescriptor => ({
    id,
    label,
    groupPath: ["Dual", group],
    arrangement: { kind: "row", panes },
  });
  return [
    ...singles2d,
    ...singles3d,
    dualRow("view-dual-top-front", "Top | Front", "Plan + Elevation", [orbitLayoutPane("top"), orbitLayoutPane("front")]),
    dualRow("view-dual-front-right", "Front | Right", "Elevation", [orbitLayoutPane("front"), orbitLayoutPane("right")]),
    dualRow("view-dual-top-ne", "Top | NE", "Plan + Isometry", [orbitLayoutPane("top"), orbitLayoutPane("isometricNe")]),
    dualRow("view-dual-plan-perspective", "Top | Perspective", "Plan + Perspective", [orbitLayoutPane("top", undefined, 100 / 3), orbitLayoutPane("perspective", undefined, 200 / 3)]),
    dualRow("view-dual-front-back", "Front | Back", "Opposite Elevations", [orbitLayoutPane("front"), orbitLayoutPane("back")]),
    dualRow("view-dual-right-left", "Right | Left", "Side Elevations", [orbitLayoutPane("right"), orbitLayoutPane("left")]),
    {
      id: "view-triple-top-front-right",
      label: "Top / Front / Right",
      groupPath: ["Triple", "Standard"],
      arrangement: {
        kind: "column",
        panes: [orbitLayoutPane("top", undefined, 34), orbitLayoutPane("front", undefined, 33), orbitLayoutPane("right", undefined, 33)],
      },
    },
    {
      id: "view-quad-standard",
      label: "Standard",
      groupPath: ["Quad", "Mixed"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 50, panes: [orbitLayoutPane("top"), orbitLayoutPane("front")] },
          { size: 50, panes: [orbitLayoutPane("right"), orbitLayoutPane("isometricNe")] },
        ],
      },
    },
    {
      id: "view-quad-ortho-faces",
      label: "Ortho Faces",
      groupPath: ["Quad", "2D"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 50, panes: [orbitLayoutPane("top"), orbitLayoutPane("front")] },
          { size: 50, panes: [orbitLayoutPane("right"), orbitLayoutPane("left")] },
        ],
      },
    },
    {
      id: "view-quad-isometry",
      label: "Isometry Corners",
      groupPath: ["Quad", "3D"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 50, panes: [orbitLayoutPane("isometricNe"), orbitLayoutPane("isometricNw")] },
          { size: 50, panes: [orbitLayoutPane("isometricSe"), orbitLayoutPane("isometricSw")] },
        ],
      },
    },
    {
      id: "view-2d-six",
      label: "Six Ortho",
      groupPath: ["2D", "Full"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 34, panes: [orbitLayoutPane("top"), orbitLayoutPane("bottom")] },
          { size: 33, panes: [orbitLayoutPane("front"), orbitLayoutPane("back")] },
          { size: 33, panes: [orbitLayoutPane("right"), orbitLayoutPane("left")] },
        ],
      },
    },
    {
      id: "view-3d-isometry",
      label: "Four Isometry",
      groupPath: ["3D", "Isometry"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 50, panes: [orbitLayoutPane("isometricNe"), orbitLayoutPane("isometricNw")] },
          { size: 50, panes: [orbitLayoutPane("isometricSe"), orbitLayoutPane("isometricSw")] },
        ],
      },
    },
  ];
}

/** @emoji 📷 Applies an orbit view preset when `seedKey` changes (owned-camera canvases). */
export function WorldOrbitCameraViewApplier(props: { readonly view: OrbitCameraViewId; readonly seedKey: string | number; readonly projectionOverride?: OrbitCameraProjection }): ReactElement {
  const { camera } = useThree();
  const controls = useThree((s) => s.controls as OrbitControlsTarget | null);
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const state = reactHostPort.useMemo(() => {
    const target = controls?.target ?? targetScratch.set(0, 0, 0);
    const targetCad = threeVec3ToCad(target);
    const distance = camera ? Math.hypot(camera.position.x - target.x, camera.position.y - target.y, camera.position.z - target.z) || 600 : 600;
    const base = computeOrbitCameraViewState(props.view, { target: targetCad, distance });
    return props.projectionOverride ? applyOrbitProjectionToCameraState(base, props.projectionOverride) : base;
  }, [camera, controls, props.projectionOverride, props.seedKey, props.view, targetScratch]);
  return <WorldOrbitCameraViewRig state={state} seedKey={props.seedKey} />;
}

/** @emoji 📷 Seeds orbit camera + target when `seedKey` changes (fixture presets, display templates). */
export function OrbitCameraViewSeed(props: { readonly camera: Camera | null; readonly state: WorldCameraState; readonly seedKey: string | number }): null {
  const controls = useThree((s) => s.controls as OrbitControlsTarget | null);
  const lastSeedKey = reactHostPort.useRef<string | number | null>(null);
  reactHostPort.useLayoutEffect(() => {
    const camera = props.camera;
    if (!camera || lastSeedKey.current === props.seedKey) {
      return;
    }
    lastSeedKey.current = props.seedKey;
    applyWorldCameraState(camera, props.state, controls);
  }, [controls, props.camera, props.seedKey, props.state.position, props.state.target, props.state.up, props.state.zoom, props.state.projection]);
  return null;
}

/** @emoji 📷 Mounts perspective/orthographic camera + seeds orbit state for display templates. */
export function WorldOrbitCameraViewRig(props: { readonly state: WorldCameraState; readonly seedKey: string | number; readonly onCamera?: (camera: Camera | null) => void; readonly perspectiveFov?: number }): ReactElement {
  const projection = props.state.projection ?? "perspective";
  const up = cadVec3ToThree(props.state.up ?? ORBIT_CAMERA_Z_UP);
  const cameraKey = `${projection}:${props.seedKey}`;
  return (
    <>
      {projection === "orthographic" ? (
        <OrthographicCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} zoom={props.state.zoom} />
      ) : (
        <PerspectiveCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} fov={props.perspectiveFov ?? 50} zoom={props.state.zoom} />
      )}
      <WorldOrbitCameraViewRigSeed state={props.state} seedKey={props.seedKey} />
    </>
  );
}

/** @emoji 🔑 Stable apply token for {@link WorldOrbitCameraViewRigSeed}; keyed by seed + projection, not camera uuid remounts. */
export function orbitCameraViewRigApplyToken(seedKey: string | number, projection: OrbitCameraProjection = "perspective"): string {
  return `${seedKey}:${projection}`;
}

/** @emoji 🔑 Whether {@link WorldOrbitCameraViewRigSeed} should apply props.state (false on camera remount with same token). */
export function shouldApplyOrbitCameraViewRigSeed(lastToken: string | null, nextToken: string): boolean {
  return lastToken !== nextToken;
}

function WorldOrbitCameraViewRigSeed(props: { readonly state: WorldCameraState; readonly seedKey: string | number }): null {
  const getThree = useThree((s) => s.get);
  const invalidate = useThree((s) => s.invalidate);
  const lastApplyToken = reactHostPort.useRef<string | null>(null);
  const stateRef = reactHostPort.useRef(props.state);
  stateRef.current = props.state;
  const projection = props.state.projection ?? "perspective";
  const controlsReady = useThree((s) => s.controls) != null;
  reactHostPort.useLayoutEffect(() => {
    const { camera, controls } = getThree();
    if (!camera) {
      return;
    }
    const token = `${orbitCameraViewRigApplyToken(props.seedKey, projection)}:controls:${controlsReady ? 1 : 0}`;
    if (!shouldApplyOrbitCameraViewRigSeed(lastApplyToken.current, token)) {
      return;
    }
    lastApplyToken.current = token;
    applyWorldCameraState(camera, stateRef.current, controls as OrbitControlsTarget | null);
    invalidate();
  }, [controlsReady, getThree, invalidate, projection, props.seedKey]);
  return null;
}
// #endregion 📷OrbitCameraView

// #region 📐WorldProjection
/** @emoji 📐 Parallel vs. perspective camera family a {@link WorldProjectionSpec} belongs to. */
export type WorldProjectionFamily = "parallel" | "perspective";

export type WorldOrthographicViewId = "plan" | "top" | "bottom" | "front" | "back" | "left" | "right";
export type WorldAxonometricVariant = "isometric" | "dimetric" | "trimetric";
export type WorldAxonometricQuadrant = "ne" | "nw" | "se" | "sw";
export type WorldAxonometricHemisphere = "upper" | "lower";
export type WorldObliqueVariant = "cabinet" | "cavalier" | "military";
export type WorldCurvilinearMapping = "fisheye" | "panini";

/** @emoji 🧭 Spatial look the navigation cube owns — cardinal faces, axonometric corners, or free 3D. */
export type WorldProjectionOrientation =
  | { readonly type: "cardinal"; readonly view: WorldOrthographicViewId }
  | { readonly type: "corner"; readonly quadrant: WorldAxonometricQuadrant; readonly hemisphere?: WorldAxonometricHemisphere }
  | { readonly type: "free" };

/** @emoji 📐 Projection *mode* the pane owns (kind + non-spatial params). Composes with {@link WorldProjectionOrientation}. */
export type WorldProjectionMode =
  | { readonly kind: "orthographic" }
  | { readonly kind: "axonometric"; readonly variant: WorldAxonometricVariant; readonly angleA: number; readonly angleB: number }
  | { readonly kind: "oblique"; readonly variant: WorldObliqueVariant; readonly angle: number; readonly depthScale: number }
  | { readonly kind: "onePoint"; readonly fov: number }
  | { readonly kind: "twoPoint"; readonly fov: number; readonly verticalShift: number }
  | { readonly kind: "threePoint"; readonly fov: number }
  | { readonly kind: "curvilinear"; readonly fov: number; readonly strength: number; readonly mapping: WorldCurvilinearMapping };

/** @emoji 📐 Full classical projection taxonomy as mode ⊗ orientation — every mode works with every gizmo view.
 * See https://en.wikipedia.org/wiki/Axonometric_projection and https://en.wikipedia.org/wiki/Oblique_projection. */
export type WorldProjectionSpec = {
  readonly mode: WorldProjectionMode;
  readonly orientation: WorldProjectionOrientation;
};

export type WorldProjectionKind = WorldProjectionMode["kind"];

/** @emoji 📡 Command name products handle to apply a {@link WorldProjectionSpec}. */
export const WORLD_PROJECTION_COMMAND = "setProjection";

/** @emoji 📐 Kind discriminator for a composed {@link WorldProjectionSpec}. */
export function worldProjectionModeKind(spec: WorldProjectionSpec): WorldProjectionKind {
  return spec.mode.kind;
}

/** @emoji 📐 Perspective FOV when the active mode carries one. */
export function worldProjectionModeFov(spec: WorldProjectionSpec): number | undefined {
  const mode = spec.mode;
  return mode.kind === "onePoint" || mode.kind === "twoPoint" || mode.kind === "threePoint" || mode.kind === "curvilinear" ? mode.fov : undefined;
}

/** @emoji 📐 Effective `PerspectiveCamera.fov` for a spec — curvilinear is capped at 160° (matches {@link WorldCurvilinearPass}'s capture), others default 50°. */
export function worldProjectionPerspectiveFov(spec: WorldProjectionSpec): number {
  const fov = worldProjectionModeFov(spec);
  return spec.mode.kind === "curvilinear" ? Math.min(fov ?? 120, 160) : (fov ?? 50);
}

/** @emoji 📐 Parallel family = orthographic camera (Orthographic/Axonometric/Oblique); everything else is perspective. */
export function worldProjectionFamily(spec: WorldProjectionSpec | undefined): WorldProjectionFamily {
  if (!spec) {
    return "perspective";
  }
  const kind = spec.mode.kind;
  return kind === "orthographic" || kind === "axonometric" || kind === "oblique" ? "parallel" : "perspective";
}

/** @emoji 📐 Baseline mode + default orientation for a freshly-selected projection kind. */
export function worldProjectionDefaults(kind: WorldProjectionKind): WorldProjectionSpec {
  switch (kind) {
    case "orthographic":
      return { mode: { kind }, orientation: { type: "cardinal", view: "plan" } };
    case "axonometric":
      return { mode: { kind, variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } };
    case "oblique":
      return { mode: { kind, variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } };
    case "onePoint":
      return { mode: { kind, fov: 50 }, orientation: { type: "cardinal", view: "front" } };
    case "twoPoint":
      return { mode: { kind, fov: 50, verticalShift: 0 }, orientation: { type: "free" } };
    case "curvilinear":
      return { mode: { kind, fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } };
    default:
      return { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } };
  }
}

const WORLD_PROJECTION_DEFAULT_DISTANCE = 600;
/** @emoji 📷 Fallback canvas size when framing a projection before the live viewport is known. */
const WORLD_PROJECTION_FRAME_FALLBACK_VIEWPORT = 640;
/** @emoji 📷 Default padding around content when framing a projection pane. */
const WORLD_PROJECTION_FRAME_PADDING = 1.35;

export type WorldSceneContentBounds = {
  readonly center: Vec3;
  readonly halfExtent: Vec3;
};

/** @emoji 📷 Axis-aligned bounds of instances and visible reference planes, for framing projection panes. */
export function worldSceneContentBounds(
  instances: readonly { readonly position?: Vec3; readonly x?: number; readonly y?: number; readonly z?: number }[],
  references: readonly { readonly origin: Vec3; readonly widthWorld?: number; readonly hidden?: boolean }[] = [],
): WorldSceneContentBounds | null {
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let minZ = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  let maxZ = Number.NEGATIVE_INFINITY;
  let any = false;
  const expand = (x: number, y: number, z: number) => {
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    minZ = Math.min(minZ, z);
    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
    maxZ = Math.max(maxZ, z);
    any = true;
  };
  for (const instance of instances) {
    const position = instance.position ?? ([instance.x ?? 0, instance.y ?? 0, instance.z ?? 0] as Vec3);
    expand(position[0], position[1], position[2]);
  }
  for (const reference of references) {
    if (reference.hidden) continue;
    const half = Math.max(reference.widthWorld ?? 1, 1e-3) * 0.5;
    const [x, y, z] = reference.origin;
    expand(x - half, y - half, z);
    expand(x + half, y + half, z);
  }
  if (!any) return null;
  return {
    center: [(minX + maxX) * 0.5, (minY + maxY) * 0.5, (minZ + maxZ) * 0.5],
    halfExtent: [Math.max((maxX - minX) * 0.5, 0.5), Math.max((maxY - minY) * 0.5, 0.5), Math.max((maxZ - minZ) * 0.5, 0.5)],
  };
}

/** @emoji 📷 Cardinal face look direction (Z-up CAD). */
export function worldProjectionCardinalLook(view: WorldOrthographicViewId): { readonly dir: Vec3; readonly up: Vec3 } {
  switch (view) {
    case "bottom":
      return { dir: [0, 0, -1], up: [0, -1, 0] };
    case "front":
      return { dir: [0, -1, 0], up: [0, 0, 1] };
    case "back":
      return { dir: [0, 1, 0], up: [0, 0, 1] };
    case "left":
      return { dir: [-1, 0, 0], up: [0, 0, 1] };
    case "right":
      return { dir: [1, 0, 0], up: [0, 0, 1] };
    default: // "plan" | "top"
      return { dir: [0, 0, 1], up: [0, 1, 0] };
  }
}

/** @emoji 📷 Corner look from axonometric elevation/azimuth (also used as spatial corner for non-axo modes). */
export function worldProjectionCornerLook(
  quadrant: WorldAxonometricQuadrant,
  hemisphere: WorldAxonometricHemisphere | undefined,
  angleADeg: number,
  angleBDeg: number,
): { readonly dir: Vec3; readonly up: Vec3 } {
  const angleA = angleADeg * (Math.PI / 180);
  const angleB = angleBDeg * (Math.PI / 180);
  const elevation = Math.asin(Math.sqrt(Math.tan(angleA) * Math.tan(angleB)));
  const azimuth = Math.atan(Math.sqrt(Math.tan(angleA) / Math.tan(angleB)));
  const signX = quadrant === "nw" || quadrant === "sw" ? -1 : 1;
  const signY = quadrant === "se" || quadrant === "sw" ? -1 : 1;
  const hemi = hemisphere ?? "upper";
  const dir: Vec3 = [signX * Math.cos(elevation) * Math.sin(azimuth), signY * Math.cos(elevation) * Math.cos(azimuth), Math.sin(elevation) * (hemi === "lower" ? -1 : 1)];
  return { dir, up: hemi === "lower" ? [0, 0, -1] : [0, 0, 1] };
}

/** @emoji 📷 Resolves look direction from mode ⊗ orientation — every orientation works with every mode. */
export function worldProjectionOrientationLook(spec: WorldProjectionSpec): { readonly dir: Vec3; readonly up: Vec3 } {
  const { mode, orientation } = spec;
  switch (orientation.type) {
    case "cardinal":
      if (mode.kind === "oblique" && mode.variant === "military" && (orientation.view === "plan" || orientation.view === "top")) {
        const rotation = mode.angle * (Math.PI / 180);
        return { dir: [0, 0, 1], up: [Math.sin(rotation), Math.cos(rotation), 0] };
      }
      return worldProjectionCardinalLook(orientation.view);
    case "corner": {
      const angleA = mode.kind === "axonometric" ? (mode.variant === "isometric" ? 30 : mode.angleA) : 30;
      const angleB = mode.kind === "axonometric" ? (mode.variant === "isometric" ? 30 : mode.variant === "dimetric" ? mode.angleA : mode.angleB) : 30;
      return worldProjectionCornerLook(orientation.quadrant, orientation.hemisphere, angleA, angleB);
    }
    case "free":
      if (mode.kind === "twoPoint") {
        return { dir: [Math.SQRT1_2, -Math.SQRT1_2, 0], up: [0, 0, 1] };
      }
      if (mode.kind === "oblique") {
        if (mode.variant === "military") {
          const rotation = mode.angle * (Math.PI / 180);
          return { dir: [0, 0, 1], up: [Math.sin(rotation), Math.cos(rotation), 0] };
        }
        return { dir: [0, -1, 0], up: [0, 0, 1] };
      }
      return { dir: vec3Normalize([0.75, -0.75, 0.55]), up: [0, 0, 1] };
  }
}

/** @emoji 📷 Half-width/height of `bounds` in the projection's view plane (CAD axes). */
export function worldProjectionViewHalfExtent(spec: WorldProjectionSpec, bounds: WorldSceneContentBounds): { readonly halfWidth: number; readonly halfHeight: number } {
  const [hx, hy, hz] = bounds.halfExtent;
  if (spec.orientation.type === "cardinal") {
    switch (spec.orientation.view) {
      case "front":
      case "back":
        return { halfWidth: hx, halfHeight: hz };
      case "left":
      case "right":
        return { halfWidth: hy, halfHeight: hz };
      default:
        return { halfWidth: hx, halfHeight: hy };
    }
  }
  if (spec.mode.kind === "oblique" && spec.mode.variant !== "military" && spec.orientation.type === "free") {
    return { halfWidth: hx, halfHeight: hz };
  }
  const span = Math.max(hx, hy, hz);
  return { halfWidth: span, halfHeight: span };
}

/** @emoji 📷 Orthographic zoom that fits content in a drei/R3F pixel-sized frustum (`left = -width/2`, …). */
export function worldProjectionOrthoZoom(halfWidth: number, halfHeight: number, viewportWidth: number, viewportHeight: number, padding = WORLD_PROJECTION_FRAME_PADDING): number {
  const paddedHalfW = Math.max(halfWidth * padding, 0.5);
  const paddedHalfH = Math.max(halfHeight * padding, 0.5);
  const zoomX = viewportWidth * 0.5 / paddedHalfW;
  const zoomY = viewportHeight * 0.5 / paddedHalfH;
  return Math.max(Math.min(zoomX, zoomY), 1e-3);
}

/** @emoji 📷 Frames a projection to scene content — centers the target and sets viewport-aware orthographic zoom. */
export function frameWorldProjectionPose(
  spec: WorldProjectionSpec,
  bounds: WorldSceneContentBounds,
  options?: { readonly padding?: number; readonly viewportWidth?: number; readonly viewportHeight?: number; readonly distance?: number },
): WorldCameraState {
  const padding = options?.padding ?? WORLD_PROJECTION_FRAME_PADDING;
  const { halfWidth, halfHeight } = worldProjectionViewHalfExtent(spec, bounds);
  const span = Math.max(halfWidth, halfHeight) * 2 * padding;
  const distance = options?.distance ?? Math.max(span * 1.5, 2);
  const family = worldProjectionFamily(spec);
  const zoom =
    family === "parallel"
      ? worldProjectionOrthoZoom(halfWidth, halfHeight, options?.viewportWidth ?? WORLD_PROJECTION_FRAME_FALLBACK_VIEWPORT, options?.viewportHeight ?? WORLD_PROJECTION_FRAME_FALLBACK_VIEWPORT, padding)
      : 1;
  return computeWorldProjectionPose(spec, { target: bounds.center, distance, zoom });
}

/** @emoji 📷 Computes a Z-up camera pose for mode ⊗ orientation — look from orientation, camera family from mode. */
export function computeWorldProjectionPose(spec: WorldProjectionSpec, options: { readonly target: Vec3; readonly distance?: number; readonly zoom?: number }): WorldCameraState {
  const distance = options.distance ?? WORLD_PROJECTION_DEFAULT_DISTANCE;
  const target = options.target;
  const family = worldProjectionFamily(spec);
  const zoom = family === "parallel" ? (options.zoom ?? 50) : (options.zoom ?? 1);
  const { dir, up } = worldProjectionOrientationLook(spec);
  return {
    position: [target[0] + dir[0] * distance, target[1] + dir[1] * distance, target[2] + dir[2] * distance],
    target,
    up,
    zoom,
    projection: family === "parallel" ? "orthographic" : "perspective",
    projectionSpec: spec,
  };
}

/** @emoji 🔒 Orbit-interaction constraints implied by a projection — drafting-locked orientations and modes disable rotation; two-point keeps the horizon level when free. */
export function worldProjectionOrbitConstraints(spec: WorldProjectionSpec | undefined): { readonly rotate: boolean; readonly minPolar?: number; readonly maxPolar?: number } {
  if (!spec) {
    return { rotate: true };
  }
  if (spec.orientation.type === "cardinal" && spec.orientation.view === "plan" && (spec.mode.kind === "orthographic" || spec.mode.kind === "oblique")) {
    return { rotate: false };
  }
  if (spec.mode.kind === "oblique" || spec.mode.kind === "onePoint") {
    return { rotate: false };
  }
  if (spec.mode.kind === "twoPoint" && spec.orientation.type === "free") {
    return { rotate: true, minPolar: Math.PI / 2, maxPolar: Math.PI / 2 };
  }
  return { rotate: true };
}

/** @emoji 🎛 Drafting plane from cardinal orientation — `undefined` for corner/free 3D. */
export function worldProjectionGumballPlane(spec: WorldProjectionSpec | undefined): GumballPlaneId | undefined {
  if (!spec || spec.orientation.type !== "cardinal") return undefined;
  switch (spec.orientation.view) {
    case "front":
    case "back":
      return "xz";
    case "left":
    case "right":
      return "yz";
    default:
      return "xy";
  }
}

/** @emoji 🎛 Drafting plane for a legacy {@link OrbitCameraViewId} preset (CAD panes that still seed via orbit views). */
export function orbitCameraViewGumballPlane(view: OrbitCameraViewId | undefined): GumballPlaneId | undefined {
  if (!view) return undefined;
  switch (view) {
    case "top":
    case "bottom":
      return "xy";
    case "front":
    case "back":
    case "north":
    case "south":
      return "xz";
    case "left":
    case "right":
    case "east":
    case "west":
      return "yz";
    default:
      return undefined;
  }
}

/** @emoji 📷 Applies a projection kind/variant switch, keeping the current pose when the caller supplies one
 * (pure parameter tweaks like angle/depth/fov never move the camera — only the matrix driver re-shears). */
export function applyWorldProjectionToCameraState(state: WorldCameraState, spec: WorldProjectionSpec): WorldCameraState {
  const family = worldProjectionFamily(spec);
  return { ...state, projection: family === "parallel" ? "orthographic" : "perspective", projectionSpec: spec, zoom: family === "parallel" ? (state.zoom === 1 ? 50 : state.zoom) : state.zoom };
}

/** @emoji 🧭 Structural equality for {@link WorldProjectionOrientation}. */
export function worldProjectionOrientationsEqual(a: WorldProjectionOrientation, b: WorldProjectionOrientation): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}

function halfFovTan(fovDeg: number): number {
  return Math.tan((fovDeg * Math.PI) / 360);
}

/** @emoji 📐 Orthographic zoom (pixels-per-world-unit in a drei pixel frustum) reproducing a perspective camera's apparent
 * scale at `distance` from the target, so a persp→ortho snap doesn't jump to the legacy zoom-50 default. */
export function worldProjectionMatchedOrthoZoom(fovDeg: number, distance: number, viewportHeight: number): number {
  return viewportHeight / (2 * Math.max(distance, ORBIT_CAMERA_VIEW_EPSILON) * halfFovTan(fovDeg));
}

/** @emoji 🪞 Oblique receding-axis shear matrix — extracted from {@link WorldProjectionMatrixDriver} so
 * {@link worldProjectionGoalMatrix} and the live per-frame shear share one formula; `strength` ramps the shear 0→1. */
export function worldObliqueShearMatrix(mode: Extract<WorldProjectionMode, { kind: "oblique" }>, strength = 1): Matrix4 {
  const angle = mode.angle * (Math.PI / 180);
  const l = mode.variant === "military" ? 1 : mode.depthScale;
  const alpha = mode.variant === "military" ? Math.PI / 2 : angle;
  const shear = new Matrix4().identity();
  shear.elements[8] = -l * Math.cos(alpha) * strength;
  shear.elements[9] = -l * Math.sin(alpha) * strength;
  return shear;
}

/** @emoji 📐 Destination projection matrix for `spec` at the live viewport size — the "goal" a projection-matrix morph
 * lerps toward each frame. Built from real three.js camera classes so it matches exactly what mounting that camera would
 * produce (drei's `OrthographicCamera`/`PerspectiveCamera` seed left/right/top/bottom or aspect the same way). */
export function worldProjectionGoalMatrix(spec: WorldProjectionSpec, options: { readonly zoom: number; readonly fov?: number; readonly viewport: { readonly width: number; readonly height: number }; readonly near?: number; readonly far?: number }): Matrix4 {
  const near = options.near ?? 0.2;
  const far = options.far ?? WORLD_ORBIT_CAMERA_MIN_FAR;
  const { width, height } = options.viewport;
  if (worldProjectionFamily(spec) === "parallel") {
    const camera = new ThreeOrthographicCamera(width / -2, width / 2, height / 2, height / -2, near, far);
    camera.zoom = options.zoom;
    camera.updateProjectionMatrix();
    const matrix = camera.projectionMatrix.clone();
    return spec.mode.kind === "oblique" ? matrix.multiply(worldObliqueShearMatrix(spec.mode)) : matrix;
  }
  const fov = options.fov ?? worldProjectionPerspectiveFov(spec);
  const camera = new ThreePerspectiveCamera(fov, width / Math.max(1, height), near, far);
  camera.zoom = options.zoom;
  camera.updateProjectionMatrix();
  const matrix = camera.projectionMatrix.clone();
  if (spec.mode.kind === "twoPoint") {
    matrix.elements[9] += spec.mode.verticalShift;
  }
  return matrix;
}

/** @emoji 🎞️ Element-wise projection-matrix lerp — valid across every pair this engine morphs between: persp↔ortho
 * lerps the w-row (`-z ↔ 1`, the standard projective blend), persp↔persp is cot-space FOV interpolation, and
 * oblique-shear/two-point-shift elements ramp linearly since they're already linear terms of the base matrix. */
export function worldProjectionMorphMatrix(from: Matrix4, to: Matrix4, t: number): Matrix4 {
  const result = new Matrix4();
  for (let i = 0; i < 16; i++) {
    result.elements[i] = from.elements[i] + (to.elements[i] - from.elements[i]) * t;
  }
  return result;
}

/** @emoji 🎞️ Destination pose for a projection snap — preserves eye exactly when only mode changes (matches
 * {@link applyWorldProjectionToCameraState}'s "pure parameter tweaks never move the camera" invariant); re-looks only
 * when orientation changes. When `live.viewport` is supplied, a persp→ortho mode-only switch picks a zoom that matches
 * the live apparent scale via {@link worldProjectionMatchedOrthoZoom} instead of the legacy zoom-50 default — this only
 * changes *which* zoom value is chosen, never the camera position, so it cannot reposition the camera. */
export function worldProjectionTransitionPose(
  pendingSpec: WorldProjectionSpec,
  live: {
    readonly position: Vec3;
    readonly target: Vec3;
    readonly up?: Vec3;
    readonly zoom: number;
    readonly isOrthographic: boolean;
    readonly projectionSpec?: WorldProjectionSpec;
    readonly viewport?: { readonly width: number; readonly height: number };
    readonly fov?: number;
  },
): WorldCameraState {
  const previousSpec = live.projectionSpec;
  const orientationUnchanged = previousSpec !== undefined && worldProjectionOrientationsEqual(previousSpec.orientation, pendingSpec.orientation);
  if (orientationUnchanged && previousSpec && live.viewport && worldProjectionFamily(previousSpec) === "perspective" && worldProjectionFamily(pendingSpec) === "parallel") {
    const fromFov = live.fov ?? worldProjectionPerspectiveFov(previousSpec);
    const distance = orbitCameraDistance({ position: live.position, target: live.target, zoom: live.zoom });
    const zoom = worldProjectionMatchedOrthoZoom(fromFov, distance, live.viewport.height);
    return applyWorldProjectionToCameraState({ position: live.position, target: live.target, up: live.up ?? ORBIT_CAMERA_Z_UP, zoom, projection: "orthographic", projectionSpec: pendingSpec }, pendingSpec);
  }
  const zoom = worldProjectionSnapZoom(pendingSpec, live.zoom, live.isOrthographic);
  if (orientationUnchanged || !previousSpec) {
    return applyWorldProjectionToCameraState(
      {
        position: live.position,
        target: live.target,
        up: live.up ?? ORBIT_CAMERA_Z_UP,
        zoom,
        projection: live.isOrthographic ? "orthographic" : "perspective",
        projectionSpec: pendingSpec,
      },
      pendingSpec,
    );
  }
  const distance = orbitCameraDistance({ position: live.position, target: live.target, zoom: live.zoom });
  return computeWorldProjectionPose(pendingSpec, { target: live.target, distance, zoom });
}

/** @emoji 🧭 Maps a projection-gizmo face to an orthographic view id. */
export function projectionGizmoFaceToOrthographicView(axis: "x" | "y" | "z", sign: 1 | -1): WorldOrthographicViewId {
  if (axis === "x") {
    return sign > 0 ? "right" : "left";
  }
  if (axis === "y") {
    return sign > 0 ? "back" : "front";
  }
  return sign > 0 ? "top" : "bottom";
}

/** @emoji 🧭 Structural equality for {@link WorldProjectionSpec} (gizmo no-operation when a hit cannot change angle). */
export function worldProjectionSpecsEqual(a: WorldProjectionSpec, b: WorldProjectionSpec): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}

/** @emoji 🧭 Resolves a navigation-cube hit into orientation only — never changes projection mode.
 * Faces → cardinal views; corners → quadrant/hemisphere; center → free 3D. */
export function resolveProjectionGizmoSpec(hit: ProjectionGizmoHit, currentSpec?: WorldProjectionSpec): WorldProjectionSpec {
  const base = currentSpec ?? worldProjectionDefaults("threePoint");
  switch (hit.type) {
    case "face":
      return { mode: base.mode, orientation: { type: "cardinal", view: projectionGizmoFaceToOrthographicView(hit.axis, hit.sign) } };
    case "corner":
      return { mode: base.mode, orientation: { type: "corner", quadrant: hit.quadrant, hemisphere: hit.hemisphere } };
    case "center":
      return { mode: base.mode, orientation: { type: "free" } };
  }
}

/** @emoji 🧭 Maps a direction vector to a navigation-cube hit (face, corner, or center). */
export function resolveProjectionGizmoHitFromDirection(direction: { readonly x: number; readonly y: number; readonly z: number }): ProjectionGizmoHit {
  const absX = Math.abs(direction.x);
  const absY = Math.abs(direction.y);
  const absZ = Math.abs(direction.z);
  if (absX > 0.45 && absY > 0.45 && absZ > 0.45) {
    const sx = direction.x >= 0 ? 1 : -1;
    const sy = direction.y >= 0 ? 1 : -1;
    const quadrant: WorldAxonometricQuadrant = sx > 0 && sy > 0 ? "ne" : sx < 0 && sy > 0 ? "nw" : sx > 0 && sy < 0 ? "se" : "sw";
    const hemisphere: WorldAxonometricHemisphere = direction.z >= 0 ? "upper" : "lower";
    return { type: "corner", quadrant, hemisphere };
  }
  if (absX < 0.2 && absY < 0.2 && absZ < 0.2) {
    return { type: "center" };
  }
  const dominant = [
    { axis: "x" as const, magnitude: absX, sign: (direction.x >= 0 ? 1 : -1) as 1 | -1 },
    { axis: "y" as const, magnitude: absY, sign: (direction.y >= 0 ? 1 : -1) as 1 | -1 },
    { axis: "z" as const, magnitude: absZ, sign: (direction.z >= 0 ? 1 : -1) as 1 | -1 },
  ].sort((a, b) => b.magnitude - a.magnitude)[0] ?? { axis: "z" as const, magnitude: 1, sign: 1 as const };
  return { type: "face", axis: dominant.axis, sign: dominant.sign };
}

/** @emoji 🧭 Converts a legacy {@link OrbitCameraViewId} into a {@link WorldProjectionSpec}. */
export function orbitViewToWorldProjectionSpec(view: OrbitCameraViewId): WorldProjectionSpec {
  switch (view) {
    case "top":
    case "bottom":
    case "front":
    case "back":
    case "left":
    case "right":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view } };
    case "north":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "back" } };
    case "south":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } };
    case "east":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "right" } };
    case "west":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "left" } };
    case "isometricNe":
      return { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } };
    case "isometricNw":
      return { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "nw", hemisphere: "upper" } };
    case "isometricSe":
      return { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "se", hemisphere: "upper" } };
    case "isometricSw":
      return { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "sw", hemisphere: "upper" } };
    case "twoPointPerspective":
      return worldProjectionDefaults("twoPoint");
    case "perspective":
    default:
      return worldProjectionDefaults("threePoint");
  }
}

/** @emoji 🧭 Best-effort reverse of {@link orbitViewToWorldProjectionSpec} for legacy orbit callbacks. */
export function worldProjectionSpecToOrbitView(spec: WorldProjectionSpec): OrbitCameraViewId | null {
  if (spec.orientation.type === "cardinal") {
    return spec.orientation.view === "plan" ? "top" : spec.orientation.view;
  }
  if (spec.orientation.type === "corner") {
    switch (spec.orientation.quadrant) {
      case "nw":
        return "isometricNw";
      case "se":
        return "isometricSe";
      case "sw":
        return "isometricSw";
      default:
        return "isometricNe";
    }
  }
  if (spec.mode.kind === "twoPoint") return "twoPointPerspective";
  return "perspective";
}

/** @emoji 🔀 Switches projection *mode* while preserving gizmo orientation (and FOV when possible). */
export function worldProjectionKindSwitchSpec(kind: WorldProjectionKind, currentSpec?: WorldProjectionSpec): WorldProjectionSpec {
  const next = worldProjectionDefaults(kind);
  const orientation = currentSpec?.orientation ?? next.orientation;
  if (kind === "onePoint" || kind === "twoPoint" || kind === "threePoint" || kind === "curvilinear") {
    const fov = worldProjectionModeFov(currentSpec ?? next) ?? worldProjectionModeFov(next)!;
    if (kind === "twoPoint") {
      const verticalShift = currentSpec?.mode.kind === "twoPoint" ? currentSpec.mode.verticalShift : 0;
      return { mode: { kind, fov, verticalShift }, orientation };
    }
    if (kind === "curvilinear") {
      const mapping = currentSpec?.mode.kind === "curvilinear" ? currentSpec.mode.mapping : "fisheye";
      const strength = currentSpec?.mode.kind === "curvilinear" ? currentSpec.mode.strength : 1;
      return { mode: { kind, fov: currentSpec?.mode.kind === "curvilinear" ? currentSpec.mode.fov : 120, strength, mapping }, orientation };
    }
    return { mode: { kind, fov }, orientation };
  }
  if (kind === "axonometric") {
    if (currentSpec?.mode.kind === "axonometric") {
      return { mode: currentSpec.mode, orientation };
    }
    return { mode: next.mode, orientation };
  }
  if (kind === "oblique") {
    if (currentSpec?.mode.kind === "oblique") {
      return { mode: currentSpec.mode, orientation };
    }
    return { mode: next.mode, orientation };
  }
  return { mode: next.mode, orientation };
}

/** @emoji 📷 Mounts the camera for a {@link WorldProjectionSpec} and the matrix/curvilinear post-processing it needs. */
export function WorldProjectionRig(props: { readonly spec: WorldProjectionSpec; readonly state: WorldCameraState; readonly seedKey: string | number; readonly onCamera?: (camera: Camera | null) => void; readonly pendingSpec?: WorldProjectionSpec | null }): ReactElement {
  const family = worldProjectionFamily(props.spec);
  const up = cadVec3ToThree(props.state.up ?? ORBIT_CAMERA_Z_UP);
  const cameraKey = `${props.spec.mode.kind}:${props.seedKey}`;
  const perspectiveFov = worldProjectionPerspectiveFov(props.spec);
  // 🐟 Pre-mount the curvilinear pass while only the *pending* spec is curvilinear so its render target is warm and
  // its strength can ramp in via `morphRef` before the camera remount, instead of popping in at full strength.
  const curvilinearMode = props.spec.mode.kind === "curvilinear" ? props.spec.mode : props.pendingSpec?.mode.kind === "curvilinear" ? props.pendingSpec.mode : null;
  return (
    <>
      {family === "parallel" ? (
        <OrthographicCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} zoom={props.state.zoom} />
      ) : (
        <PerspectiveCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} fov={perspectiveFov} zoom={props.state.zoom} />
      )}
      <WorldOrbitCameraViewRigSeed state={props.state} seedKey={`${cameraKey}`} />
      {props.spec.mode.kind === "oblique" || props.spec.mode.kind === "twoPoint" ? <WorldProjectionMatrixDriver mode={props.spec.mode} /> : null}
      {curvilinearMode ? <WorldCurvilinearPass mode={curvilinearMode} /> : null}
    </>
  );
}

/** @emoji 📷 Applies a projection preset when `seedKey` changes (owned-camera canvases) — mirrors {@link WorldOrbitCameraViewApplier}. */
export function WorldProjectionApplier(props: { readonly spec: WorldProjectionSpec; readonly seedKey: string | number }): ReactElement {
  const { camera } = useThree();
  const controls = useThree((s) => s.controls as OrbitControlsTarget | null);
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const state = reactHostPort.useMemo(() => {
    const target = controls?.target ?? targetScratch.set(0, 0, 0);
    const targetCad = threeVec3ToCad(target);
    const distance = camera ? Math.hypot(camera.position.x - target.x, camera.position.y - target.y, camera.position.z - target.z) || WORLD_PROJECTION_DEFAULT_DISTANCE : WORLD_PROJECTION_DEFAULT_DISTANCE;
    return computeWorldProjectionPose(props.spec, { target: targetCad, distance });
  }, [camera, controls, props.seedKey, props.spec, targetScratch]);
  return <WorldProjectionRig spec={props.spec} state={state} seedKey={props.seedKey} />;
}

/** @emoji 🪞 Post-multiplies the oblique receding-axis shear or two-point vertical lens-shift onto the active
 * camera's projection matrix every frame (after recomputing the pristine matrix, so resize/zoom never compounds
 * the shear) and refreshes `projectionMatrixInverse` — required because r3f raycasting/picking reads the inverse.
 * Defers to {@link WorldProjectionSnapDriver} while a projection morph is in flight (`morphRef.current`), since the
 * morph's goal matrix already bakes in the destination shear/shift via {@link worldObliqueShearMatrix} — running both
 * would double-apply the transform. */
function WorldProjectionMatrixDriver(props: { readonly mode: Extract<WorldProjectionMode, { kind: "oblique" | "twoPoint" }> }): null {
  const { camera, invalidate } = useThree();
  const { morphRef } = useWorldOrbitViewSnapGate();
  useFrame(() => {
    if (morphRef.current) {
      return;
    }
    camera.updateProjectionMatrix();
    if (props.mode.kind === "oblique" && camera instanceof ThreeOrthographicCamera) {
      camera.projectionMatrix.multiply(worldObliqueShearMatrix(props.mode));
    } else if (props.mode.kind === "twoPoint" && camera instanceof ThreePerspectiveCamera) {
      camera.projectionMatrix.elements[9] += props.mode.verticalShift;
    }
    camera.projectionMatrixInverse.copy(camera.projectionMatrix).invert();
    invalidate();
  });
  return null;
}

/** @emoji 🐟 Remaps a pointer NDC coordinate through the inverse curvilinear radius mapping so raycasting against
 * the exact (undistorted) capture-space render stays pixel-accurate under fisheye/panini distortion. */
export function worldCurvilinearUnproject(ndc: readonly [number, number], mode: Extract<WorldProjectionMode, { kind: "curvilinear" }>, aspect = 1): readonly [number, number] {
  const halfFov = (Math.min(mode.fov, 160) * (Math.PI / 180)) / 2;
  const scaled: [number, number] = [ndc[0] * aspect, ndc[1]];
  const r = Math.hypot(scaled[0], scaled[1]);
  if (r < 1e-6) {
    return ndc;
  }
  const theta = r * halfFov;
  const rectilinearR = Math.tan(theta) / Math.tan(halfFov);
  const sourceR = r + (rectilinearR - r) * mode.strength;
  const scale = sourceR / r;
  return [(scaled[0] * scale) / aspect, scaled[1] * scale];
}

const WORLD_CURVILINEAR_VERTEX_SHADER = `
  varying vec2 vUv;
  void main() {
    vUv = uv;
    gl_Position = vec4(position.xy, 0.0, 1.0);
  }
`;

/** @emoji 🐟 Fisheye/panini remap blit — samples the linear capture and applies Three's output color-space convert. */
export const WORLD_CURVILINEAR_FRAGMENT_SHADER = `
  uniform sampler2D tCapture;
  uniform float uFov;
  uniform float uStrength;
  uniform float uAspect;
  uniform float uMapping; // 0 = fisheye, 1 = panini
  varying vec2 vUv;
  void main() {
    vec2 ndc = vUv * 2.0 - 1.0;
    vec2 scaled = vec2(ndc.x * uAspect, ndc.y);
    float halfFov = uFov * 0.5;
    float r = length(scaled);
    vec2 sourceNdc = ndc;
    if (r > 1e-5) {
      float theta = r * halfFov;
      float rectR = tan(theta) / tan(halfFov);
      float sourceR = mix(r, rectR, uStrength);
      float scale = sourceR / r;
      sourceNdc = vec2((scaled.x * scale) / uAspect, scaled.y * scale);
    }
    vec2 sourceUv = sourceNdc * 0.5 + 0.5;
    if (sourceUv.x < 0.0 || sourceUv.x > 1.0 || sourceUv.y < 0.0 || sourceUv.y > 1.0) {
      gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
      #include <colorspace_fragment>
      return;
    }
    gl_FragColor = texture2D(tCapture, sourceUv);
    #include <colorspace_fragment>
  }
`;

/** @emoji 🐟 Capture-target defaults for the curvilinear blit: linear working-space storage, linear filtering
 * (fisheye UV warp must interpolate), half-float precision (same as pmndrs/drei `RenderCubeTexture`), sized by the
 * caller at drawing-buffer resolution. Not drei `<Fisheye>` — that component portals children into a cubemap sphere
 * and has no FOV/strength/panini; our taxonomy needs a sibling planar remap pass instead. */
export const WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS = {
  minFilter: LinearFilter,
  magFilter: LinearFilter,
  format: RGBAFormat,
  type: HalfFloatType,
  colorSpace: LinearSRGBColorSpace,
} as const;

/** @emoji 🐟 Wide-FOV planar capture (capped at 160°, exact and material-agnostic below full 180°) remapped through
 * a fullscreen fisheye/panini shader — chosen over pmndrs/drei `<Fisheye>` / a 6-face cubemap unwrap because it needs
 * one extra render target instead of six, keeps orbit/gizmo chrome undistorted as siblings, supports strength+panini,
 * and the taxonomy never promises >=180° coverage. */
function WorldCurvilinearPass(props: { readonly mode: Extract<WorldProjectionMode, { kind: "curvilinear" }> }): null {
  const { gl, camera, scene, size, invalidate } = useThree();
  const { morphRef } = useWorldOrbitViewSnapGate();
  const targetRef = reactHostPort.useRef<InstanceType<typeof WebGLRenderTarget> | null>(null);
  const quadRef = reactHostPort.useRef<{ readonly scene: InstanceType<typeof Scene>; readonly camera: InstanceType<typeof ThreeOrthographicCamera>; readonly material: InstanceType<typeof ShaderMaterial> } | null>(null);

  useFrame(() => {
    if (!(camera instanceof ThreePerspectiveCamera)) {
      return;
    }
    const pixelRatio = gl.getPixelRatio();
    const width = Math.max(1, Math.floor(size.width * pixelRatio));
    const height = Math.max(1, Math.floor(size.height * pixelRatio));
    if (!targetRef.current || targetRef.current.width !== width || targetRef.current.height !== height) {
      targetRef.current?.dispose();
      targetRef.current = new WebGLRenderTarget(width, height, { ...WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS });
    }
    if (!quadRef.current) {
      const material = new ShaderMaterial({
        vertexShader: WORLD_CURVILINEAR_VERTEX_SHADER,
        fragmentShader: WORLD_CURVILINEAR_FRAGMENT_SHADER,
        uniforms: { tCapture: { value: targetRef.current.texture }, uFov: { value: 1 }, uStrength: { value: 1 }, uAspect: { value: 1 }, uMapping: { value: 0 } },
        depthTest: false,
        depthWrite: false,
        toneMapped: false,
      });
      const quadScene = new Scene();
      quadScene.add(new Mesh(new PlaneGeometry(2, 2), material));
      const quadCamera = new ThreeOrthographicCamera(-1, 1, 1, -1, 0, 1);
      quadRef.current = { scene: quadScene, camera: quadCamera, material };
    }
    const target = targetRef.current;
    const { material } = quadRef.current;
    // 🐟 Fade `uStrength`/`uFov` with the live projection morph — entering curvilinear ramps 0→target strength,
    // leaving ramps target→0; `uStrength=0` is an exact identity blit so the fade can never pop.
    const morph = morphRef.current;
    const enteringCurvilinear = morph?.toSpec.mode.kind === "curvilinear";
    const leavingCurvilinear = morph?.fromSpec?.mode.kind === "curvilinear";
    let ramp = 1;
    let fov = props.mode.fov;
    if (morph && enteringCurvilinear && !leavingCurvilinear) {
      ramp = morph.eased;
      fov = morph.fromFov + (morph.toFov - morph.fromFov) * morph.eased;
    } else if (morph && leavingCurvilinear && !enteringCurvilinear) {
      ramp = 1 - morph.eased;
      fov = morph.fromFov + (morph.toFov - morph.fromFov) * morph.eased;
    } else if (morph && !enteringCurvilinear && !leavingCurvilinear) {
      ramp = 0;
    }
    material.uniforms.tCapture.value = target.texture;
    material.uniforms.uFov.value = Math.min(fov, 160) * (Math.PI / 180);
    material.uniforms.uStrength.value = props.mode.strength * ramp;
    material.uniforms.uAspect.value = size.width / Math.max(1, size.height);
    material.uniforms.uMapping.value = props.mode.mapping === "panini" ? 1 : 0;

    gl.setRenderTarget(target);
    gl.render(scene, camera);
    gl.setRenderTarget(null);
    gl.render(quadRef.current.scene, quadRef.current.camera);
    invalidate();
  }, 1);

  reactHostPort.useEffect(() => {
    return () => {
      targetRef.current?.dispose();
      quadRef.current?.material.dispose();
    };
  }, []);
  return null;
}

export interface WorldProjectionTemplateDescriptor {
  readonly id: string;
  readonly label: string;
  readonly iconId: string;
  readonly controllerId: string;
  readonly command: string;
  readonly args: { readonly spec: WorldProjectionSpec };
  readonly children?: readonly WorldProjectionTemplateDescriptor[];
}

export interface CreateWorldProjectionTemplatesConfig {
  readonly controllerId: string;
  readonly command?: string;
}

function worldProjectionTemplateLeaf(controllerId: string, command: string, id: string, label: string, iconId: string, spec: WorldProjectionSpec): WorldProjectionTemplateDescriptor {
  return { id, label, iconId, controllerId, command, args: { spec } };
}

function worldProjectionTemplateBranch(controllerId: string, command: string, id: string, label: string, iconId: string, spec: WorldProjectionSpec, children: readonly WorldProjectionTemplateDescriptor[]): WorldProjectionTemplateDescriptor {
  return { id, label, iconId, controllerId, command, args: { spec }, children };
}

/** @emoji 🪟 Builds the projection-mode taxonomy tree (no Top/Front — those are gizmo orientations):
 * `Parallel > Orthographic | Axonometric (Isometric/Dimetric/Trimetric) | Oblique (Cabinet/Cavalier/Military)`
 * and `Perspective > 1-Point/2-Point/3-Point/Curvilinear`. */
export function createWorldProjectionTemplates(config: CreateWorldProjectionTemplatesConfig): readonly WorldProjectionTemplateDescriptor[] {
  const { controllerId } = config;
  const command = config.command ?? WORLD_PROJECTION_COMMAND;
  const leaf = (id: string, label: string, iconId: string, spec: WorldProjectionSpec) => worldProjectionTemplateLeaf(controllerId, command, id, label, iconId, spec);
  const branch = (id: string, label: string, iconId: string, spec: WorldProjectionSpec, children: readonly WorldProjectionTemplateDescriptor[]) => worldProjectionTemplateBranch(controllerId, command, id, label, iconId, spec, children);

  const orthographic = leaf("orthographic", "Orthographic", "projection-orthographic", worldProjectionDefaults("orthographic"));

  const axonometricVariants: readonly [WorldAxonometricVariant, string, string][] = [
    ["isometric", "Isometric", "projection-isometric"],
    ["dimetric", "Dimetric", "projection-dimetric"],
    ["trimetric", "Trimetric", "projection-trimetric"],
  ];
  const axonometric = branch(
    "axonometric",
    "Axonometric",
    "projection-axonometric",
    worldProjectionDefaults("axonometric"),
    axonometricVariants.map(([variant, label, iconId]) =>
      leaf(`axonometric-${variant}`, label, iconId, {
        mode: { kind: "axonometric", variant, angleA: variant === "trimetric" ? 12 : 15, angleB: variant === "trimetric" ? 42 : variant === "isometric" ? 30 : 15 },
        orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" },
      }),
    ),
  );

  const obliqueVariants: readonly [WorldObliqueVariant, string, string][] = [
    ["cabinet", "Cabinet", "projection-oblique-cabinet"],
    ["cavalier", "Cavalier", "projection-oblique-cavalier"],
    ["military", "Military", "projection-oblique-military"],
  ];
  const oblique = branch(
    "oblique",
    "Oblique",
    "projection-oblique",
    worldProjectionDefaults("oblique"),
    obliqueVariants.map(([variant, label, iconId]) =>
      leaf(`oblique-${variant}`, label, iconId, {
        mode: { kind: "oblique", variant, angle: 45, depthScale: variant === "cabinet" ? 0.5 : 1 },
        orientation: { type: "cardinal", view: variant === "military" ? "plan" : "front" },
      }),
    ),
  );

  const parallel = branch("parallel", "Parallel", "projection-parallel", worldProjectionDefaults("orthographic"), [orthographic, axonometric, oblique]);

  const perspective = branch("perspective", "Perspective", "projection-perspective", worldProjectionDefaults("threePoint"), [
    leaf("one-point", "1-Point", "projection-one-point", worldProjectionDefaults("onePoint")),
    leaf("two-point", "2-Point", "projection-two-point", worldProjectionDefaults("twoPoint")),
    leaf("three-point", "3-Point", "projection-three-point", worldProjectionDefaults("threePoint")),
    leaf("curvilinear", "Curvilinear", "projection-curvilinear", worldProjectionDefaults("curvilinear")),
  ]);

  return [parallel, perspective];
}

const WORLD_PROJECTION_TEMPLATE_PREFIX = "world-projection:";

/** @emoji 🪟 Encodes a {@link WorldProjectionSpec} into a `WindowTemplateDropPayload.templateId` string,
 * for the Display "Windows" drag palette to seed a freshly-opened pane's initial camera. */
export function encodeWorldProjectionTemplateId(spec: WorldProjectionSpec): string {
  return `${WORLD_PROJECTION_TEMPLATE_PREFIX}${JSON.stringify(spec)}`;
}

/** @emoji 🪟 Inverse of {@link encodeWorldProjectionTemplateId}; `null` for anything else (including `undefined`). */
export function decodeWorldProjectionTemplateId(templateId: string | undefined): WorldProjectionSpec | null {
  if (!templateId || !templateId.startsWith(WORLD_PROJECTION_TEMPLATE_PREFIX)) {
    return null;
  }
  try {
    const parsed = JSON.parse(templateId.slice(WORLD_PROJECTION_TEMPLATE_PREFIX.length)) as { readonly mode?: { readonly kind?: unknown }; readonly orientation?: unknown };
    return typeof parsed.mode?.kind === "string" && parsed.orientation ? (parsed as WorldProjectionSpec) : null;
  } catch {
    return null;
  }
}

const WORLD_ORTHOGRAPHIC_VIEW_LABELS: Record<WorldOrthographicViewId, string> = { plan: "Plan", top: "Top", bottom: "Bottom", front: "Front", back: "Back", left: "Left", right: "Right" };
const WORLD_AXONOMETRIC_VARIANT_LABELS: Record<WorldAxonometricVariant, string> = { isometric: "Isometric", dimetric: "Dimetric", trimetric: "Trimetric" };
const WORLD_OBLIQUE_VARIANT_LABELS: Record<WorldObliqueVariant, string> = { cabinet: "Cabinet", cavalier: "Cavalier", military: "Military" };

/** @emoji 🧭 Short label for a gizmo orientation. */
export function worldProjectionOrientationLabel(orientation: WorldProjectionOrientation): string {
  switch (orientation.type) {
    case "cardinal":
      return WORLD_ORTHOGRAPHIC_VIEW_LABELS[orientation.view];
    case "corner": {
      const q = orientation.quadrant.toUpperCase();
      return (orientation.hemisphere ?? "upper") === "lower" ? `${q}↓` : q;
    }
    case "free":
      return "3D";
  }
}

/** @emoji 📐 Mode label matching {@link createWorldProjectionTemplates} leaves. */
export function worldProjectionModeLabel(mode: WorldProjectionMode): string {
  switch (mode.kind) {
    case "orthographic":
      return "Orthographic";
    case "axonometric":
      return WORLD_AXONOMETRIC_VARIANT_LABELS[mode.variant];
    case "oblique":
      return WORLD_OBLIQUE_VARIANT_LABELS[mode.variant];
    case "onePoint":
      return "1-Point";
    case "twoPoint":
      return "2-Point";
    case "threePoint":
      return "3-Point";
    case "curvilinear":
      return "Curvilinear";
  }
}

/** @emoji 🪟 Window title for a live projection — matches {@link createWorldProjectionTemplates} mode labels. */
export function worldProjectionSpecLabel(spec: WorldProjectionSpec): string {
  return worldProjectionModeLabel(spec.mode);
}

/** @emoji 🖼️ Catalog icon for a live projection — matches {@link createWorldProjectionTemplates} icons. */
export function worldProjectionSpecIconId(spec: WorldProjectionSpec): string {
  switch (spec.mode.kind) {
    case "orthographic":
      return "projection-orthographic";
    case "axonometric":
      switch (spec.mode.variant) {
        case "isometric":
          return "projection-isometric";
        case "dimetric":
          return "projection-dimetric";
        case "trimetric":
          return "projection-trimetric";
      }
      break;
    case "oblique":
      switch (spec.mode.variant) {
        case "cabinet":
          return "projection-oblique-cabinet";
        case "cavalier":
          return "projection-oblique-cavalier";
        case "military":
          return "projection-oblique-military";
      }
      break;
    case "onePoint":
      return "projection-one-point";
    case "twoPoint":
      return "projection-two-point";
    case "threePoint":
      return "projection-three-point";
    case "curvilinear":
      return "projection-curvilinear";
  }
}
// #endregion 📐WorldProjection

// #region 🖱️OrbitMouseBindings
/** @emoji 🖱️ Orbit-controls instance with mutable mouse button map. */
export type WorldOrbitControlsBinding = {
  readonly mouseButtons: { LEFT: number | null; MIDDLE: number; RIGHT: number | null };
  readonly enabled?: boolean;
  readonly update?: () => void;
};

/** @emoji 🖱️ Default orbit mouse map: middle always pans; Alt+right orbits when rotation is enabled. */
export function resolveWorldOrbitMouseButtonsIdle(_projection: OrbitCameraProjection = "perspective", _rotateEnabled = true): {
  readonly LEFT: number | null;
  readonly MIDDLE: number;
  readonly RIGHT: number | null;
} {
  return { LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null };
}

/** @emoji 🖱️ Resets orbit mouse buttons to {@link resolveWorldOrbitMouseButtonsIdle}. */
export function applyWorldOrbitMouseButtonsIdle(controls: WorldOrbitControlsBinding, projection: OrbitCameraProjection = "perspective", rotateEnabled = true): void {
  const idle = resolveWorldOrbitMouseButtonsIdle(projection, rotateEnabled);
  controls.mouseButtons.LEFT = idle.LEFT;
  controls.mouseButtons.MIDDLE = idle.MIDDLE;
  controls.mouseButtons.RIGHT = idle.RIGHT;
  controls.update?.();
}

/** @emoji 🖱️ Maps right-button modifiers: plain right → context menu, Shift+right → pan, Alt+right → orbit. */
export function resolveWorldOrbitRightMouseAction(event: Pick<PointerEvent, "button" | "altKey" | "shiftKey">, _projection: OrbitCameraProjection = "perspective"): number | null {
  if (event.button !== 2) {
    return null;
  }
  if (event.shiftKey) {
    return MOUSE.PAN;
  }
  if (event.altKey) {
    return MOUSE.ROTATE;
  }
  return null;
}

export interface WorldOrbitRightMouseBindingsOptions {
  readonly projection?: OrbitCameraProjection;
  readonly rotateEnabled?: boolean;
  readonly onRightPointerDown?: (event: PointerEvent) => boolean;
  readonly onRightPointerDrag?: (event: PointerEvent, distancePx: number) => void;
  readonly onRightPointerUp?: (event: PointerEvent) => void;
  readonly dragThresholdPx?: number;
}

/** @emoji 🖱️ False from `onRightPointerDown` fully suppresses orbit's own button assignment for that gesture (e.g. Alt+right-click over a vortex opens a suggestion popup instead of orbiting). */
export function shouldAssignWorldOrbitRightMouse(event: PointerEvent, onRightPointerDown?: (event: PointerEvent) => boolean): boolean {
  return onRightPointerDown?.(event) !== false;
}

/** @emoji 🖱️ Binds projection-aware Alt+right and Shift+right actions while leaving plain right click for context menus. */
export function useWorldOrbitRightMouseBindings(controls: WorldOrbitControlsBinding | null, domElement: HTMLElement | undefined, options?: WorldOrbitRightMouseBindingsOptions): void {
  const projection = options?.projection ?? "perspective";
  const rotateEnabled = options?.rotateEnabled ?? true;
  const optionsRef = reactHostPort.useRef(options);
  optionsRef.current = options;
  reactHostPort.useLayoutEffect(() => {
    if (controls) {
      applyWorldOrbitMouseButtonsIdle(controls, projection, rotateEnabled);
    }
  }, [controls, projection, rotateEnabled]);
  reactHostPort.useEffect(() => {
    if (!controls || !domElement) {
      return;
    }
    const dragThresholdPx = options?.dragThresholdPx ?? 0;
    let rightPointer: { readonly pointerId: number; readonly x: number; readonly y: number } | null = null;
    const assignRightMouse = (event: PointerEvent) => {
      if (event.button !== 2) {
        return;
      }
      controls.mouseButtons.RIGHT = (optionsRef.current?.rotateEnabled ?? true) ? resolveWorldOrbitRightMouseAction(event, optionsRef.current?.projection ?? "perspective") : event.shiftKey ? MOUSE.PAN : null;
      controls.update?.();
    };
    const resetRightMouse = () => {
      controls.mouseButtons.RIGHT = resolveWorldOrbitMouseButtonsIdle(optionsRef.current?.projection ?? "perspective", optionsRef.current?.rotateEnabled ?? true).RIGHT;
      controls.update?.();
    };
    const onPointerDown = (event: PointerEvent) => {
      if (event.button !== 2) {
        return;
      }
      if (!shouldAssignWorldOrbitRightMouse(event, optionsRef.current?.onRightPointerDown)) {
        return;
      }
      rightPointer = { pointerId: event.pointerId, x: event.clientX, y: event.clientY };
      assignRightMouse(event);
    };
    const onPointerMove = (event: PointerEvent) => {
      const start = rightPointer;
      if (!start || start.pointerId !== event.pointerId) {
        return;
      }
      const distancePx = Math.hypot(event.clientX - start.x, event.clientY - start.y);
      if (distancePx >= dragThresholdPx) {
        optionsRef.current?.onRightPointerDrag?.(event, distancePx);
      }
    };
    const onPointerUp = (event: PointerEvent) => {
      const start = rightPointer;
      if (!start || start.pointerId !== event.pointerId) {
        return;
      }
      rightPointer = null;
      optionsRef.current?.onRightPointerUp?.(event);
      resetRightMouse();
    };
    const bindings = new WorldEventBindingController();
    bindings.listen(domElement, "pointerdown", onPointerDown as EventListener, true);
    bindings.listen(window, "pointermove", onPointerMove as EventListener);
    bindings.listen(window, "pointerup", onPointerUp as EventListener, true);
    return () => bindings.dispose();
  }, [controls, domElement, options?.dragThresholdPx, projection]);
}

// #endregion 🖱️OrbitMouseBindings

// #region 🎬WorldCanvas
type OrbitControlsBinding = WorldOrbitControlsBinding;

/** @emoji 🎞️ Kicks demand-frameloop renders across mount + orbit/grid setup frames and whenever the
 * store reports a pending invalidate (async GLB/texture commits after the initial kick). */
export function DemandFrameloopKick(): null {
  const invalidate = useThree((state) => state.invalidate);
  const gl = useThree((state) => state.gl);
  reactHostPort.useLayoutEffect(() => {
    invalidate();
    let frame = 0;
    let raf = 0;
    const tick = () => {
      invalidate();
      frame += 1;
      if (frame < 4) raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(raf);
  }, [invalidate]);
  // 🎞️ Keep demand mode alive briefly while WebGL textures/meshes settle; loaders resolve after the
  // mount kick and React commits without always scheduling `invalidate`, leaving transparent empty panes.
  // Per-asset `invalidate` in reference/GLB loaders covers late arrivals; this pump only bridges the first half-second.
  reactHostPort.useEffect(() => {
    let cancelled = false;
    let raf = 0;
    let frames = 0;
    const pump = () => {
      if (cancelled) return;
      invalidate();
      frames += 1;
      if (frames < 24) raf = requestAnimationFrame(pump);
    };
    raf = requestAnimationFrame(pump);
    const onContextRestored = () => invalidate();
    gl.domElement.addEventListener("webglcontextrestored", onContextRestored);
    return () => {
      cancelled = true;
      cancelAnimationFrame(raf);
      gl.domElement.removeEventListener("webglcontextrestored", onContextRestored);
    };
  }, [gl, invalidate]);
  return null;
}

export interface WorldOrbitGatedProps {
  readonly camera?: ThreePerspectiveCamera | null;
  readonly zoom?: number;
  readonly projection?: OrbitCameraProjection;
  /** @emoji 🔒 Rotation/polar-angle limits implied by the active {@link WorldProjectionSpec}; see {@link worldProjectionOrbitConstraints}. */
  readonly constraints?: { readonly rotate: boolean; readonly minPolar?: number; readonly maxPolar?: number };
  readonly onCamera?: (state: WorldCameraState) => void;
  readonly controlsGate?: boolean;
  readonly onCameraNavigate?: (active: boolean) => void;
  readonly controlsKey?: string | number;
  readonly onRightPointerDown?: (event: PointerEvent) => boolean;
  /** @emoji 🧭 Reports which gestures (pan/zoom/orbit) a drag/scroll performed, classified between `start` and `end`. */
  readonly onNavigationGestures?: (gestures: readonly WorldNavigationGesture[]) => void;
}

const WORLD_ORBIT_CONSTRAINTS_DEFAULT = { rotate: true } as const;

/** @emoji 🛰️ Canvas-local Three orbit-control binding that never crosses the optional Drei runtime boundary. */
function WorldOrbitControlsBridge({
  camera,
  enabled,
  mouseButtons,
  constraints,
  controlsKey,
  onChange,
  onStart,
  onEnd,
}: {
  readonly camera: Camera;
  readonly enabled: boolean;
  readonly mouseButtons: ReturnType<typeof resolveWorldOrbitMouseButtonsIdle>;
  readonly constraints?: { readonly rotate: boolean; readonly minPolar?: number; readonly maxPolar?: number };
  readonly controlsKey?: string | number;
  readonly onChange: () => void;
  readonly onStart: () => void;
  readonly onEnd: () => void;
}): null {
  const { gl } = useThree();
  const set = useThree((state) => state.set);
  const get = useThree((state) => state.get);
  const controlsRef = reactHostPort.useRef<ThreeOrbitControls | null>(null);
  const callbacksRef = reactHostPort.useRef({ onChange, onStart, onEnd });
  callbacksRef.current = { onChange, onStart, onEnd };
  const resolvedConstraints = constraints ?? WORLD_ORBIT_CONSTRAINTS_DEFAULT;
  reactHostPort.useEffect(() => {
    const controls = new ThreeOrbitControls(camera, gl.domElement);
    controls.enableDamping = false;
    controls.enablePan = true;
    controls.enableZoom = true;
    controls.enabled = enabled;
    controls.mouseButtons = { ...mouseButtons };
    const change = () => callbacksRef.current.onChange();
    const start = () => callbacksRef.current.onStart();
    const end = () => callbacksRef.current.onEnd();
    controls.addEventListener("change", change);
    controls.addEventListener("start", start);
    controls.addEventListener("end", end);
    controlsRef.current = controls;
    set({ controls });
    controls.update();
    return () => {
      controls.removeEventListener("change", change);
      controls.removeEventListener("start", start);
      controls.removeEventListener("end", end);
      controls.dispose();
      controlsRef.current = null;
      if (get().controls === controls) set({ controls: null });
    };
  }, [camera, controlsKey, get, gl.domElement, set]);
  reactHostPort.useEffect(() => {
    const controls = controlsRef.current;
    if (!controls) return;
    controls.enabled = enabled;
    controls.mouseButtons = { ...mouseButtons };
    controls.enableRotate = resolvedConstraints.rotate;
    controls.minPolarAngle = resolvedConstraints.minPolar ?? 0;
    controls.maxPolarAngle = resolvedConstraints.maxPolar ?? Math.PI;
    controls.update();
  }, [enabled, mouseButtons, resolvedConstraints]);
  return null;
}

/** @emoji 🛰️ Orbit controls with injectable gate (specializations disable during drag/tools). */
export function WorldOrbitGated(props: WorldOrbitGatedProps): ReactElement | null {
  const { camera: sceneCamera, gl } = useThree();
  const camera = props.camera === undefined ? sceneCamera : props.camera;
  const controls = useThree((s) => s.controls as OrbitControlsBinding | null);
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const gate = props.controlsGate ?? false;
  const { snapGate } = useWorldOrbitViewSnapGate();
  const invalidate = useThree((s) => s.invalidate);
  const projection = props.projection ?? (camera instanceof ThreeOrthographicCamera ? "orthographic" : "perspective");
  const rotateEnabled = props.constraints?.rotate ?? true;
  const mouseButtonsIdle = reactHostPort.useMemo(() => resolveWorldOrbitMouseButtonsIdle(projection, rotateEnabled), [projection, rotateEnabled]);
  useWorldOrbitRightMouseBindings(controls, gl.domElement, { projection, rotateEnabled, onRightPointerDown: props.onRightPointerDown });
  const navigationSnapshotRef = reactHostPort.useRef<WorldNavigationSnapshot | null>(null);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [gate, invalidate]);
  if (!camera || props.camera === null) return null;
  const reportCamera = () => {
    if (!props.onCamera || !camera) return;
    const tgt = controls?.target ?? targetScratch.set(0, 0, 0);
    props.onCamera({
      position: threeVec3ToCad(camera.position),
      target: threeVec3ToCad(tgt),
      zoom: props.zoom ?? 1,
      up: threeVec3ToCad(camera.up),
      projection,
    });
  };
  // 🧭 Live zoom (not `props.zoom`, stale mid-gesture — see `reportCamera` above) so an orthographic
  // scroll-zoom classifies correctly even before the shell's own camera-state prop round-trips.
  const captureNavigationSnapshot = (): WorldNavigationSnapshot | null => {
    if (!camera) return null;
    const tgt = controls?.target ?? targetScratch.set(0, 0, 0);
    return {
      position: threeVec3ToCad(camera.position),
      target: threeVec3ToCad(tgt),
      zoom: camera instanceof ThreeOrthographicCamera ? camera.zoom : 1,
      projection,
    };
  };
  return (
    <WorldOrbitControlsBridge
      camera={camera}
      enabled={!gate && !snapGate}
      mouseButtons={mouseButtonsIdle}
      constraints={props.constraints}
      controlsKey={props.controlsKey}
      onChange={() => invalidate()}
      onStart={() => {
        invalidate();
        navigationSnapshotRef.current = captureNavigationSnapshot();
        props.onCameraNavigate?.(true);
      }}
      onEnd={() => {
        invalidate();
        props.onCameraNavigate?.(false);
        reportCamera();
        const before = navigationSnapshotRef.current;
        navigationSnapshotRef.current = null;
        const after = captureNavigationSnapshot();
        if (before && after && props.onNavigationGestures) {
          const gestures = classifyWorldNavigationGestures(before, after);
          if (gestures.length > 0) props.onNavigationGestures(gestures);
        }
      }}
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
  /** Fires only when a click/pointer event hits no interactive object (r3f raycast miss) — the correct signal for "background click deselects", since r3f's per-object `stopPropagation()` does not stop the underlying native DOM event from bubbling to a host-level `onClick`. */
  readonly onPointerMissed?: (event: MouseEvent) => void;
}

/** @emoji 🌍 Generic infinite-world r3f canvas shell (`frameloop="demand"`). */
export function WorldCanvas(props: WorldCanvasProps): ReactElement {
  const extra = props.extraRootProps ?? {};
  const frameloop = props.frameloop ?? "demand";
  const cameraUp = props.cameraUp ?? ([0, 0, 1] as Vec3);
  const ownedCamera = props.cameraPosition !== undefined;
  const onWheelRef = reactHostPort.useRef(props.onWheel);
  const wheelCleanupRef = reactHostPort.useRef<(() => void) | null>(null);
  onWheelRef.current = props.onWheel;
  reactHostPort.useEffect(() => () => wheelCleanupRef.current?.(), []);
  const setRootRef = reactHostPort.useCallback(
    (element: HTMLDivElement | null) => {
      const external = props.rootRef;
      if (!external) return;
      if (typeof external === "function") external(element);
      else external.current = element;
    },
    [props.rootRef],
  );
  return (
    <div
      ref={setRootRef}
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
        onPointerMissed={props.onPointerMissed}
        onCreated={({ camera, gl: renderer }) => {
          wheelCleanupRef.current?.();
          const canvas = renderer.domElement;
          const onWheel = (event: WheelEvent) => {
            event.preventDefault();
            onWheelRef.current?.(event);
          };
          canvas.addEventListener("wheel", onWheel, { passive: false });
          wheelCleanupRef.current = () => canvas.removeEventListener("wheel", onWheel);
          props.onCanvasReady?.({ camera, domElement: canvas });
        }}
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

// #region 🖼️Reference
/** @emoji 🖼️ Persisted media source for a world reference plane. */
export interface WorldReferenceSource {
  readonly url: string;
  readonly mediaKind: import("@semio-tech/ui-react").ReferenceMediaKind;
  readonly page?: number;
}

/** @emoji 🖼️ Serializable world reference plane on the infinite grid. */
export interface WorldReferenceProps extends WorldEntityFlags {
  readonly id: string;
  readonly source: WorldReferenceSource;
  readonly origin: Vec3;
  readonly orientation?: Quat;
  readonly scale?: number | Vec3;
  readonly widthWorld?: number;
  readonly opacity?: number;
  readonly relocate?: GumballConfig | false;
  readonly relocateActive?: boolean;
}

/** @emoji 🖼️ Gumball relocate commit for a world reference plane. */
export interface WorldReferenceRelocatePayload {
  readonly referenceId: string;
  readonly mode: "translate" | "rotate" | "scale";
  readonly before: GumballPose;
  readonly after: GumballPose;
}

export const WORLD_REFERENCE_DEFAULT_WIDTH = 10;

export const WORLD_REFERENCE_SELECTED_BACKGROUND_CSS = semanticVar("active-base");
export const WORLD_REFERENCE_SELECTED_OUTLINE_CSS = tokenVar("primary");
export const WORLD_REFERENCE_HOVER_BACKGROUND_CSS = semanticVar("hover-base");
export const WORLD_REFERENCE_SELECTED_CONTENT_OPACITY = 0.5;

export interface WorldReferenceAppearance {
  readonly backgroundColor: string | null;
  readonly contentOpacity: number;
  readonly outlineColor: string | null;
}

/** @emoji 🖼️ Resolves hover/selection fill, content opacity, and border for reference planes. */
export function worldReferenceAppearance(renderMode: Pick<WorldEntityRenderMode, "asHover" | "showSelectedOutline">, baseOpacity: number): WorldReferenceAppearance {
  if (renderMode.showSelectedOutline) {
    return {
      backgroundColor: resolveColorHex(WORLD_REFERENCE_SELECTED_BACKGROUND_CSS, "primary"),
      contentOpacity: baseOpacity * WORLD_REFERENCE_SELECTED_CONTENT_OPACITY,
      outlineColor: resolveColorHex(WORLD_REFERENCE_SELECTED_OUTLINE_CSS, "primary"),
    };
  }
  if (renderMode.asHover) {
    return {
      backgroundColor: resolveColorHex(WORLD_REFERENCE_HOVER_BACKGROUND_CSS, "gray"),
      contentOpacity: baseOpacity,
      outlineColor: resolveColorHex(semanticVar("accent-secondary"), "gray"),
    };
  }
  return {
    backgroundColor: null,
    contentOpacity: baseOpacity,
    outlineColor: null,
  };
}

/** @emoji 🖼️ Applies CAD pose fields onto a reference group node. */
export function applyWorldReferencePose(group: Group, reference: Pick<WorldReferenceProps, "origin" | "orientation" | "scale">): void {
  const position = cadVec3ToThree(reference.origin);
  group.position.set(position[0], position[1], position[2]);
  const quat = reference.orientation ?? ([0, 0, 0, 1] as Quat);
  group.quaternion.set(quat[0], quat[1], quat[2], quat[3]);
  const scale = worldReferenceScaleVec(reference.scale);
  group.scale.set(scale[0], scale[1], scale[2]);
}

/** @emoji 🖼️ Writes a gumball pose back onto persisted reference props. */
export function applyWorldReferenceTransform(reference: WorldReferenceProps, after: GumballPose): WorldReferenceProps {
  return {
    ...reference,
    origin: [after.position[0], after.position[1], after.position[2]],
    orientation: [after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]],
    scale: [after.scale[0], after.scale[1], after.scale[2]],
  };
}

const WORLD_REFERENCE_DEFAULT_QUAT: Quat = [0, 0, 0, 1];

/** @emoji 🖼️ Resolves reference orientation with CAD default identity quaternion. */
export function worldReferenceOrientation(reference: Pick<WorldReferenceProps, "orientation">): Quat {
  return reference.orientation ?? WORLD_REFERENCE_DEFAULT_QUAT;
}

/** @emoji 🖼️ Expands reference scale to a uniform XYZ tuple. */
export function worldReferenceScaleVec(scale: number | Vec3 | undefined): Vec3 {
  if (typeof scale === "number") {
    return [scale, scale, scale];
  }
  if (scale) {
    return [scale[0], scale[1], scale[2]];
  }
  return [1, 1, 1];
}

/** @emoji 🧭 Converts a CAD quaternion to tilt Euler degrees (roll, pitch, yaw). */
export function worldQuatToEulerDegrees(quat: Quat): Vec3 {
  const [x, y, z, w] = quat;
  const sinRoll = 2 * (w * x + y * z);
  const cosRoll = 1 - 2 * (x * x + y * y);
  const roll = Math.atan2(sinRoll, cosRoll);
  const sinPitch = 2 * (w * y - z * x);
  const pitch = Math.abs(sinPitch) >= 1 ? Math.sign(sinPitch) * (Math.PI / 2) : Math.asin(sinPitch);
  const sinYaw = 2 * (w * z + x * y);
  const cosYaw = 1 - 2 * (y * y + z * z);
  const yaw = Math.atan2(sinYaw, cosYaw);
  const radToDeg = 180 / Math.PI;
  return [roll * radToDeg, pitch * radToDeg, yaw * radToDeg];
}

/** @emoji 🧭 Converts tilt Euler degrees (roll, pitch, yaw) to a CAD quaternion. */
export function worldEulerDegreesToQuat(euler: readonly [number, number, number]): Quat {
  const degToRad = Math.PI / 180;
  const roll = euler[0] * degToRad;
  const pitch = euler[1] * degToRad;
  const yaw = euler[2] * degToRad;
  const cy = Math.cos(yaw * 0.5);
  const sy = Math.sin(yaw * 0.5);
  const cp = Math.cos(pitch * 0.5);
  const sp = Math.sin(pitch * 0.5);
  const cr = Math.cos(roll * 0.5);
  const sr = Math.sin(roll * 0.5);
  return [sr * cp * cy - cr * sp * sy, cr * sp * cy + sr * cp * sy, cr * cp * sy - sr * sp * cy, cr * cp * cy + sr * sp * sy];
}

/** @emoji 🖼️ Applies a declarative inspector patch to one reference plane. */
export function patchWorldReferenceProps(reference: WorldReferenceProps, field: "origin" | "rotation" | "scale" | "scaleUniform" | "widthWorld" | "opacity", value: unknown): WorldReferenceProps | null {
  const patch: Partial<Omit<WorldReferenceProps, "id">> = {};
  if (field === "origin" && Array.isArray(value) && value.length === 3) {
    patch.origin = [Number(value[0]), Number(value[1]), Number(value[2])];
  } else if (field === "rotation" && Array.isArray(value) && value.length === 3) {
    patch.orientation = worldEulerDegreesToQuat([Number(value[0]), Number(value[1]), Number(value[2])]);
  } else if (field === "scale" && Array.isArray(value) && value.length === 3) {
    const sx = Number(value[0]);
    const sy = Number(value[1]);
    const sz = Number(value[2]);
    patch.scale = sx === sy && sy === sz ? sx : ([sx, sy, sz] as Vec3);
  } else if (field === "scaleUniform") {
    const parsed = typeof value === "number" ? value : Number(value);
    if (Number.isFinite(parsed)) {
      patch.scale = parsed;
    }
  } else if (field === "widthWorld") {
    const parsed = typeof value === "number" ? value : Number(value);
    if (Number.isFinite(parsed)) {
      patch.widthWorld = parsed;
    }
  } else if (field === "opacity") {
    const parsed = typeof value === "number" ? value : Number(value);
    if (Number.isFinite(parsed)) {
      patch.opacity = parsed;
    }
  } else {
    return null;
  }
  return { ...reference, ...patch };
}

function worldReferencePlaneSize(reference: WorldReferenceProps, aspect: number): readonly [number, number] {
  const width = reference.widthWorld ?? WORLD_REFERENCE_DEFAULT_WIDTH;
  return [width, width / Math.max(aspect, 1e-6)];
}

const WorldReferenceGumball = reactHostPort.memo(function WorldReferenceGumball(props: { readonly referenceId: string; readonly target: Group; readonly config: GumballConfig; readonly onRelocate?: (payload: WorldReferenceRelocatePayload) => void }) {
  const beforeRef = reactHostPort.useRef<GumballPose | null>(null);
  return (
    <UnifiedGumball
      target={props.target}
      config={props.config}
      onDragStart={(_kind, before) => {
        beforeRef.current = before;
      }}
      onDragEnd={(kind, before, after) => {
        props.onRelocate?.({
          referenceId: props.referenceId,
          mode: gumballHandleKindToTransformMode(kind),
          before: beforeRef.current ?? before,
          after,
        });
        beforeRef.current = null;
      }}
    />
  );
});

const WorldReferencePlaneItem = reactHostPort.memo(function WorldReferencePlaneItem(props: {
  readonly reference: WorldReferenceProps;
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly revealed: boolean;
  readonly gumballConfig?: GumballConfig;
  readonly relocateActive?: boolean;
  readonly translationSnap?: number;
  readonly onSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onHover?: (id: string | null) => void;
  readonly onRelocate?: (payload: WorldReferenceRelocatePayload) => void;
}) {
  const groupRef = reactHostPort.useRef<Group>(null);
  const [pointerHovered, setPointerHovered] = reactHostPort.useState(false);
  const [media, setMedia] = reactHostPort.useState<{ readonly width: number; readonly height: number; readonly texture: import("three").Texture } | null>(null);
  const selectable = worldEntitySelectable(props.reference);
  const inspectable = worldEntityInspectable(props.reference);
  const interactionHovered = selectable && (props.hovered || pointerHovered);
  const renderMode = {
    ...worldEntityRenderMode(props.reference, {
      hovered: interactionHovered,
      selected: props.selected,
      revealed: props.revealed || (selectable && pointerHovered),
    }),
    showSelectedOutline: props.selected && inspectable,
  };
  reactHostPort.useEffect(() => {
    let cancelled = false;
    referenceMediaPort
      .loadReferenceTexture(props.reference.source)
      .then((loaded) => {
        if (cancelled) {
          loaded.texture.dispose();
          return;
        }
        setMedia((prev) => {
          prev?.texture.dispose();
          return loaded;
        });
      })
      .catch(() => {
        // texture load failure — plane stays hidden
      });
    return () => {
      cancelled = true;
    };
  }, [props.reference.id, props.reference.source.url, props.reference.source.mediaKind, props.reference.source.page]);
  const invalidate = useThree((state) => state.invalidate);
  // 🎞️ `WorldCanvas` uses `frameloop="demand"` — async texture arrival must kick a draw or panes stay transparent.
  reactHostPort.useLayoutEffect(() => {
    if (media) invalidate();
  }, [invalidate, media]);
  reactHostPort.useLayoutEffect(() => {
    if (groupRef.current) {
      applyWorldReferencePose(groupRef.current, props.reference);
    }
  }, [props.reference.origin, props.reference.orientation, props.reference.scale]);
  const mediaAspect = media ? media.width / media.height : 1;
  const [planeWidth, planeHeight] = reactHostPort.useMemo(() => worldReferencePlaneSize(props.reference, mediaAspect), [props.reference, mediaAspect]);
  const opacityBase = props.reference.opacity ?? 1;
  const opacityDimmed = renderMode.dim ? opacityBase * WORLD_LOCKED_OPACITY_SCALE : opacityBase;
  const appearance = worldReferenceAppearance(renderMode, opacityDimmed);
  const planeOutlineGeo = reactHostPort.useMemo(() => {
    const plane = new PlaneGeometry(planeWidth, planeHeight);
    const edges = new EdgesGeometry(plane);
    plane.dispose();
    return edges;
  }, [planeWidth, planeHeight]);
  reactHostPort.useEffect(() => () => planeOutlineGeo.dispose(), [planeOutlineGeo]);
  if (!renderMode.visible || !media) {
    return null;
  }
  const config = props.reference.relocate === false ? null : { ...(props.reference.relocate ?? {}), translationSnap: props.translationSnap };
  const showGumball = props.selected && selectable && props.relocateActive !== false && groupRef.current && config && gumballConfigVisible(config);
  return (
    <>
      <group ref={groupRef} visible={renderMode.visible} userData={{ worldReferenceId: props.reference.id }}>
        {appearance.backgroundColor ? (
          <mesh raycast={worldRaycastNone} renderOrder={-11}>
            <planeGeometry args={[planeWidth, planeHeight]} />
            <meshBasicMaterial attach="material" color={appearance.backgroundColor} transparent opacity={1} depthWrite={false} side={DoubleSide} toneMapped={false} />
          </mesh>
        ) : null}
        <mesh
          renderOrder={-10}
          raycast={selectable ? undefined : worldRaycastNone}
          onPointerDown={(event) => {
            if (!selectable || event.button !== 0) {
              return;
            }
            event.stopPropagation();
            props.onSelect?.(props.reference.id, { shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
          }}
          onPointerOver={(event) => {
            if (!selectable) {
              return;
            }
            event.stopPropagation();
            setPointerHovered(true);
            props.onHover?.(props.reference.id);
          }}
          onPointerOut={(event) => {
            if (!selectable) {
              return;
            }
            event.stopPropagation();
            setPointerHovered(false);
            props.onHover?.(null);
          }}
        >
          <planeGeometry args={[planeWidth, planeHeight]} />
          <meshBasicMaterial attach="material" map={media.texture} transparent opacity={appearance.contentOpacity} depthWrite={false} side={DoubleSide} toneMapped={false} />
        </mesh>
        {appearance.outlineColor ? (
          <lineSegments raycast={worldRaycastNone} geometry={planeOutlineGeo} renderOrder={-9} scale={[1.002, 1.002, 1.002]}>
            <lineBasicMaterial attach="material" color={appearance.outlineColor} transparent opacity={renderMode.showSelectedOutline ? 1 : 0.9} depthWrite={false} />
          </lineSegments>
        ) : null}
      </group>
      {showGumball && groupRef.current ? <WorldReferenceGumball referenceId={props.reference.id} target={groupRef.current} config={config} onRelocate={props.onRelocate} /> : null}
    </>
  );
});

/** @emoji 🖼️ Renders persisted world reference planes for CAD and puzzle 3d hosts. */
export function WorldReferenceLayer(props: {
  readonly references: readonly WorldReferenceProps[];
  readonly selectedIds?: ReadonlySet<string>;
  readonly hoveredId?: string | null;
  readonly revealedIds?: ReadonlySet<string>;
  readonly gumballConfig?: GumballConfig;
  readonly relocateActive?: boolean;
  readonly translationSnap?: number;
  readonly onSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onHover?: (id: string | null) => void;
  readonly onRelocate?: (payload: WorldReferenceRelocatePayload) => void;
}): ReactElement | null {
  if (!props.references.length) {
    return null;
  }
  const selected = props.selectedIds ?? new Set<string>();
  const revealed = props.revealedIds ?? new Set<string>();
  return (
    <>
      {props.references.map((reference) => (
        <WorldReferencePlaneItem
          key={reference.id}
          reference={reference}
          selected={selected.has(reference.id)}
          hovered={props.hoveredId === reference.id}
          revealed={revealed.has(reference.id)}
          gumballConfig={props.gumballConfig}
          relocateActive={props.relocateActive}
          translationSnap={props.translationSnap}
          onSelect={props.onSelect}
          onHover={props.onHover}
          onRelocate={props.onRelocate}
        />
      ))}
    </>
  );
}
// #endregion 🖼️Reference

// #region 🧊Volume
/** @emoji 🧊 Serializable oriented target volume box for fill constraints. */
export interface WorldVolumeProps extends WorldEntityFlags {
  readonly id: string;
  readonly origin: Vec3;
  readonly orientation?: Quat;
  readonly scale?: number | Vec3;
  readonly color?: string;
  readonly opacity?: number;
  readonly relocate?: GumballConfig | false;
  readonly relocateActive?: boolean;
}

/** @emoji 🧊 Gumball relocate commit for a world target volume. */
export interface WorldVolumeRelocatePayload {
  readonly volumeId: string;
  readonly mode: "translate" | "rotate" | "scale";
  readonly before: GumballPose;
  readonly after: GumballPose;
}

export const WORLD_VOLUME_DEFAULT_COLOR = "#22c55e";
export const WORLD_VOLUME_DEFAULT_OPACITY = 0.22;

function worldVolumeScaleVec(scale: number | Vec3 | undefined): Vec3 {
  if (typeof scale === "number") {
    return [scale, scale, scale];
  }
  if (scale) {
    return [scale[0], scale[1], scale[2]];
  }
  return [1, 1, 1];
}

/** @emoji 🧊 Applies CAD pose fields onto a volume group node. */
export function applyWorldVolumePose(group: Group, volume: Pick<WorldVolumeProps, "origin" | "orientation" | "scale">): void {
  const position = cadVec3ToThree(volume.origin);
  group.position.set(position[0], position[1], position[2]);
  const quat = volume.orientation ?? ([0, 0, 0, 1] as Quat);
  group.quaternion.set(quat[0], quat[1], quat[2], quat[3]);
  const scale = worldVolumeScaleVec(volume.scale);
  group.scale.set(scale[0], scale[1], scale[2]);
}

/** @emoji 🧊 Writes a gumball pose back onto persisted volume props. */
export function applyWorldVolumeTransform(volume: WorldVolumeProps, after: GumballPose): WorldVolumeProps {
  return {
    ...volume,
    origin: [after.position[0], after.position[1], after.position[2]],
    orientation: [after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]],
    scale: [after.scale[0], after.scale[1], after.scale[2]],
  };
}

const _worldVolumeMatrix = new Matrix4();
const _worldVolumeInverse = new Matrix4();
const _worldVolumeCorner = new Vector3();
const _worldVolumeLocal = new Vector3();

/** @emoji 🧊 True when every corner of a world AABB lies inside any oriented volume (union). */
export function worldVolumesContainAabb(volumes: readonly WorldVolumeProps[], aabbMin: Vec3, aabbMax: Vec3, epsilon = 1e-3): boolean {
  if (!volumes.length) {
    return true;
  }
  const corners: Vec3[] = [
    [aabbMin[0], aabbMin[1], aabbMin[2]],
    [aabbMax[0], aabbMin[1], aabbMin[2]],
    [aabbMin[0], aabbMax[1], aabbMin[2]],
    [aabbMax[0], aabbMax[1], aabbMin[2]],
    [aabbMin[0], aabbMin[1], aabbMax[2]],
    [aabbMax[0], aabbMin[1], aabbMax[2]],
    [aabbMin[0], aabbMax[1], aabbMax[2]],
    [aabbMax[0], aabbMax[1], aabbMax[2]],
  ];
  for (const volume of volumes) {
    const group = new Group();
    applyWorldVolumePose(group, volume);
    group.updateMatrixWorld(true);
    _worldVolumeInverse.copy(group.matrixWorld).invert();
    const hx = 0.5 + epsilon;
    const hy = 0.5 + epsilon;
    const hz = 0.5 + epsilon;
    let inside = true;
    for (const corner of corners) {
      _worldVolumeCorner.set(corner[0], corner[1], corner[2]);
      _worldVolumeLocal.copy(_worldVolumeCorner).applyMatrix4(_worldVolumeInverse);
      if (Math.abs(_worldVolumeLocal.x) > hx || Math.abs(_worldVolumeLocal.y) > hy || Math.abs(_worldVolumeLocal.z) > hz) {
        inside = false;
        break;
      }
    }
    if (inside) {
      return true;
    }
  }
  return false;
}

const WorldVolumeGumball = reactHostPort.memo(function WorldVolumeGumball(props: { readonly volumeId: string; readonly target: Group; readonly config: GumballConfig; readonly onRelocate?: (payload: WorldVolumeRelocatePayload) => void }) {
  const beforeRef = reactHostPort.useRef<GumballPose | null>(null);
  return (
    <UnifiedGumball
      target={props.target}
      config={props.config}
      onDragStart={(_kind, before) => {
        beforeRef.current = before;
      }}
      onDragEnd={(kind, before, after) => {
        props.onRelocate?.({
          volumeId: props.volumeId,
          mode: gumballHandleKindToTransformMode(kind),
          before: beforeRef.current ?? before,
          after,
        });
        beforeRef.current = null;
      }}
    />
  );
});

const _worldVolumeEdgeGeo = new EdgesGeometry(new BoxGeometry(1, 1, 1));

const WorldVolumeBoxItem = reactHostPort.memo(function WorldVolumeBoxItem(props: {
  readonly volume: WorldVolumeProps;
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly revealed: boolean;
  readonly interactive?: boolean;
  readonly gumballConfig?: GumballConfig;
  readonly relocateActive?: boolean;
  readonly translationSnap?: number;
  readonly onSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onHover?: (id: string | null) => void;
  readonly onRelocate?: (payload: WorldVolumeRelocatePayload) => void;
}) {
  const groupRef = reactHostPort.useRef<Group>(null);
  const renderMode = worldEntityRenderMode(props.volume, { hovered: props.hovered, selected: props.selected, revealed: props.revealed });
  const selectable = props.interactive !== false && worldEntitySelectable(props.volume);
  reactHostPort.useLayoutEffect(() => {
    if (groupRef.current) {
      applyWorldVolumePose(groupRef.current, props.volume);
    }
  }, [props.volume.origin, props.volume.orientation, props.volume.scale]);
  if (!renderMode.visible) {
    return null;
  }
  const opacityBase = props.volume.opacity ?? WORLD_VOLUME_DEFAULT_OPACITY;
  const opacity = renderMode.dim ? opacityBase * WORLD_LOCKED_OPACITY_SCALE : opacityBase;
  const color = props.volume.color ?? WORLD_VOLUME_DEFAULT_COLOR;
  const config = props.volume.relocate === false ? null : { ...(props.volume.relocate ?? {}), translationSnap: props.translationSnap };
  const showGumball = props.interactive !== false && props.selected && selectable && props.relocateActive !== false && groupRef.current && config && gumballConfigVisible(config);
  return (
    <>
      <group
        ref={groupRef}
        visible={renderMode.visible}
        userData={{ worldVolumeId: props.volume.id }}
        onPointerDown={(event) => {
          if (!selectable) {
            return;
          }
          event.stopPropagation();
          props.onSelect?.(props.volume.id, { shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
        }}
        onPointerOver={(event) => {
          if (!selectable) {
            return;
          }
          event.stopPropagation();
          props.onHover?.(props.volume.id);
        }}
        onPointerOut={(event) => {
          if (!selectable) {
            return;
          }
          event.stopPropagation();
          props.onHover?.(null);
        }}
      >
        <mesh raycast={selectable ? undefined : worldRaycastNone}>
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial color={color} transparent opacity={opacity} depthWrite={false} side={DoubleSide} />
        </mesh>
        <lineSegments raycast={worldRaycastNone} geometry={_worldVolumeEdgeGeo}>
          <lineBasicMaterial color={color} transparent opacity={Math.min(1, opacity + 0.35)} depthWrite={false} />
        </lineSegments>
      </group>
      {showGumball && groupRef.current ? <WorldVolumeGumball volumeId={props.volume.id} target={groupRef.current} config={config} onRelocate={props.onRelocate} /> : null}
    </>
  );
});

/** @emoji 🧊 Renders persisted target volume boxes for puzzle 3d fill constraints. */
export function WorldVolumeLayer(props: {
  readonly volumes: readonly WorldVolumeProps[];
  readonly selectedIds?: ReadonlySet<string>;
  readonly hoveredId?: string | null;
  readonly revealedIds?: ReadonlySet<string>;
  readonly interactive?: boolean;
  readonly gumballConfig?: GumballConfig;
  readonly relocateActive?: boolean;
  readonly translationSnap?: number;
  readonly onSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onHover?: (id: string | null) => void;
  readonly onRelocate?: (payload: WorldVolumeRelocatePayload) => void;
}): ReactElement | null {
  if (!props.volumes.length) {
    return null;
  }
  const selected = props.selectedIds ?? new Set<string>();
  const revealed = props.revealedIds ?? new Set<string>();
  return (
    <>
      {props.volumes.map((volume) => (
        <WorldVolumeBoxItem
          key={volume.id}
          volume={volume}
          selected={selected.has(volume.id)}
          hovered={props.hoveredId === volume.id}
          revealed={revealed.has(volume.id)}
          interactive={props.interactive}
          gumballConfig={props.gumballConfig}
          relocateActive={props.relocateActive}
          translationSnap={props.translationSnap}
          onSelect={props.onSelect}
          onHover={props.onHover}
          onRelocate={props.onRelocate}
        />
      ))}
    </>
  );
}
// #endregion 🧊Volume

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

  describe("resolveWorldOrbitMouseButtonsIdle", () => {
    it("maps an unmodified middle click to pan in every projection", () => {
      expect(resolveWorldOrbitMouseButtonsIdle("orthographic")).toEqual({ LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null });
      expect(resolveWorldOrbitMouseButtonsIdle("perspective")).toEqual({ LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null });
    });

    it("pans an orthographic plan camera on an unmodified middle drag", () => {
      const canvas = document.createElement("canvas");
      Object.defineProperties(canvas, {
        clientWidth: { value: 400 },
        clientHeight: { value: 300 },
        setPointerCapture: { value: () => undefined },
        releasePointerCapture: { value: () => undefined },
      });
      const camera = new ThreeOrthographicCamera(-200, 200, 150, -150, 0.1, 1_000);
      camera.position.set(0, 0, 10);
      camera.lookAt(0, 0, 0);
      const controls = new ThreeOrbitControls(camera, canvas);
      applyWorldOrbitMouseButtonsIdle(controls, "orthographic");
      const offsetBefore = camera.position.clone().sub(controls.target);
      const pointer = (kind: string, x: number, y: number, button: number) => {
        const event = new MouseEvent(kind, { bubbles: true, button, clientX: x, clientY: y });
        Object.defineProperties(event, { pointerId: { value: 1 }, pointerType: { value: "mouse" }, pageX: { value: x }, pageY: { value: y } });
        return event;
      };
      canvas.dispatchEvent(pointer("pointerdown", 100, 100, 1));
      document.dispatchEvent(pointer("pointermove", 140, 125, 1));
      document.dispatchEvent(pointer("pointerup", 140, 125, 1));
      const offsetAfter = camera.position.clone().sub(controls.target);
      expect(controls.target.length()).toBeGreaterThan(0);
      expect(offsetAfter.distanceTo(offsetBefore)).toBeLessThan(1e-9);
      controls.dispose();
    });

    it("always maps middle click to pan when rotation is disabled (plan/oblique/one-point projections), even in the orthographic family", () => {
      expect(resolveWorldOrbitMouseButtonsIdle("orthographic", false)).toEqual({ LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null });
      expect(resolveWorldOrbitMouseButtonsIdle("perspective", false)).toEqual({ LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null });
    });
  });

  describe("classifyWorldNavigationGestures", () => {
    const base: WorldNavigationSnapshot = { position: [0, 0, 10], target: [0, 0, 0], zoom: 1, projection: "perspective" };

    it("detects pan when only the target translates", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [5, 0, 10], target: [5, 0, 0] };
      expect(classifyWorldNavigationGestures(base, after)).toEqual(["pan"]);
    });

    it("detects zoom via distance change in perspective projection", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [0, 0, 5] };
      expect(classifyWorldNavigationGestures(base, after)).toEqual(["zoom"]);
    });

    it("detects zoom via the zoom factor in orthographic projection, ignoring unchanged distance", () => {
      const orthoBase: WorldNavigationSnapshot = { ...base, projection: "orthographic", zoom: 1 };
      const after: WorldNavigationSnapshot = { ...orthoBase, zoom: 1.5 };
      expect(classifyWorldNavigationGestures(orthoBase, after)).toEqual(["zoom"]);
    });

    it("detects orbit when the camera direction around the target rotates", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [10, 0, 0] };
      expect(classifyWorldNavigationGestures(base, after)).toEqual(["orbit"]);
    });

    it("returns no gestures when every delta is below threshold", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [0, 0, 10.001], target: [0.001, 0, 0] };
      expect(classifyWorldNavigationGestures(base, after)).toEqual([]);
    });

    it("never misreads a pure pan as orbit, and a pure orbit as pan", () => {
      const panned: WorldNavigationSnapshot = { ...base, position: [5, 0, 10], target: [5, 0, 0] };
      expect(classifyWorldNavigationGestures(base, panned)).not.toContain("orbit");
      const orbited: WorldNavigationSnapshot = { ...base, position: [10, 0, 0] };
      expect(classifyWorldNavigationGestures(base, orbited)).not.toContain("pan");
    });

    it("can report multiple gestures from a single combined movement", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [10, 0, 0], target: [0, 0, 0] };
      const gestures = classifyWorldNavigationGestures({ ...base, position: [0, 0, 20] }, after);
      expect(gestures).toContain("orbit");
      expect(gestures).toContain("zoom");
    });
  });

  describe("resolveWorldOrbitRightMouseAction", () => {
    it("reserves plain right click for context menu and maps modifiers to orbit and pan", () => {
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: false, shiftKey: false })).toBeNull();
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: true, shiftKey: false }, "perspective")).toBe(MOUSE.ROTATE);
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: true, shiftKey: false }, "orthographic")).toBe(MOUSE.ROTATE);
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: false, shiftKey: true })).toBe(MOUSE.PAN);
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: true, shiftKey: true })).toBe(MOUSE.PAN);
      expect(resolveWorldOrbitRightMouseAction({ button: 0, altKey: true, shiftKey: false })).toBeNull();
    });
  });

  describe("shouldAssignWorldOrbitRightMouse", () => {
    it("suppresses orbit's own button assignment when onRightPointerDown returns false", () => {
      const event = { clientX: 10, clientY: 20 } as PointerEvent;
      expect(shouldAssignWorldOrbitRightMouse(event, () => false)).toBe(false);
      expect(shouldAssignWorldOrbitRightMouse(event, () => true)).toBe(true);
      expect(shouldAssignWorldOrbitRightMouse(event, undefined)).toBe(true);
    });
  });

  describe("computeOrbitCameraViewState", () => {
    it("places top view directly above the target with orthographic projection", () => {
      const state = computeOrbitCameraViewState("top", { target: [0, 0, 40], distance: 800 });
      expect(state).toMatchObject({ position: [0, 0, 840], target: [0, 0, 40], up: [0, 1, 0], projection: "orthographic", zoom: 50 });
    });

    it("maps north to +Y and perspective to an oblique direction", () => {
      const north = computeOrbitCameraViewState("north", { target: [0, 0, 0], distance: 100 });
      expect(north.position).toEqual([0, 100, 0]);
      expect(north.projection).toBe("orthographic");
      const perspective = computeOrbitCameraViewState("perspective", { target: [0, 0, 0], distance: 100 });
      expect(perspective.projection).toBe("perspective");
      expect(Math.hypot(...perspective.position)).toBeGreaterThan(90);
    });
  });

  describe("resolveOrbitGizmoViewFromDirection", () => {
    it("maps dominant axis clicks to orthographic orbit views", () => {
      expect(resolveOrbitGizmoViewFromDirection({ x: 1, y: 0.1, z: 0.05 })).toBe("right");
      expect(resolveOrbitGizmoViewFromDirection({ x: -1, y: 0, z: 0 })).toBe("left");
      expect(resolveOrbitGizmoViewFromDirection({ x: 0.1, y: 0.2, z: 1 })).toBe("top");
      expect(resolveOrbitGizmoViewFromDirection({ x: 0, y: 0, z: -1 })).toBe("bottom");
      expect(resolveOrbitGizmoViewFromDirection({ x: 0, y: -1, z: 0.1 })).toBe("front");
      expect(resolveOrbitGizmoViewFromDirection({ x: 0.05, y: 1, z: 0.1 })).toBe("back");
    });
  });

  describe("resolveProjectionGizmoSpec", () => {
    it("sets orientation only and preserves the active projection mode", () => {
      expect(resolveProjectionGizmoSpec({ type: "face", axis: "z", sign: 1 }, { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })).toEqual({
        mode: { kind: "threePoint", fov: 50 },
        orientation: { type: "cardinal", view: "top" },
      });
      expect(resolveProjectionGizmoSpec({ type: "face", axis: "x", sign: -1 }, { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } })).toEqual({
        mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 },
        orientation: { type: "cardinal", view: "left" },
      });
      expect(resolveProjectionGizmoSpec({ type: "corner", quadrant: "se", hemisphere: "upper" }, { mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } })).toEqual({
        mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" },
        orientation: { type: "corner", quadrant: "se", hemisphere: "upper" },
      });
      expect(resolveProjectionGizmoSpec({ type: "center" }, { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } })).toEqual({
        mode: { kind: "orthographic" },
        orientation: { type: "free" },
      });
      expect(resolveProjectionGizmoSpec({ type: "center" }, { mode: { kind: "twoPoint", fov: 42, verticalShift: 0 }, orientation: { type: "cardinal", view: "top" } })).toEqual({
        mode: { kind: "twoPoint", fov: 42, verticalShift: 0 },
        orientation: { type: "free" },
      });
    });

    it("preserves axonometric mode params when snapping corners", () => {
      const dimetric = { mode: { kind: "axonometric" as const, variant: "dimetric" as const, angleA: 15, angleB: 15 }, orientation: { type: "corner" as const, quadrant: "ne" as const, hemisphere: "upper" as const } };
      expect(resolveProjectionGizmoSpec({ type: "corner", quadrant: "sw", hemisphere: "lower" }, dimetric)).toEqual({
        mode: dimetric.mode,
        orientation: { type: "corner", quadrant: "sw", hemisphere: "lower" },
      });
    });

    it("composes every mode with gizmo top orientation (top works for fisheye, 2pt, axo, …)", () => {
      const top = { type: "cardinal" as const, view: "top" as const };
      for (const kind of WORLD_PROJECTION_KINDS) {
        const withTop = { mode: worldProjectionDefaults(kind).mode, orientation: top };
        const pose = computeWorldProjectionPose(withTop, { target: [0, 0, 0], distance: 100 });
        expect(pose.position[2]).toBeGreaterThan(0);
        expect(pose.projectionSpec).toEqual(withTop);
      }
      const fishTop = resolveProjectionGizmoSpec({ type: "face", axis: "z", sign: 1 }, worldProjectionDefaults("curvilinear"));
      expect(fishTop).toEqual({ mode: worldProjectionDefaults("curvilinear").mode, orientation: { type: "cardinal", view: "top" } });
    });
  });

  describe("orbitViewToWorldProjectionSpec / worldProjectionSpecToOrbitView", () => {
    it("round-trips orthographic and isometric orbit views", () => {
      expect(orbitViewToWorldProjectionSpec("front")).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } });
      expect(orbitViewToWorldProjectionSpec("isometricNw")).toEqual({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "nw", hemisphere: "upper" } });
      expect(worldProjectionSpecToOrbitView({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "right" } })).toBe("right");
      expect(worldProjectionSpecToOrbitView({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "se", hemisphere: "upper" } })).toBe("isometricSe");
    });
  });

  describe("worldProjectionKindSwitchSpec", () => {
    it("emits defaults for every projection kind", () => {
      for (const kind of WORLD_PROJECTION_KINDS) {
        expect(worldProjectionKindSwitchSpec(kind).mode.kind).toBe(kind);
      }
      expect(worldProjectionKindSwitchSpec("orthographic")).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } });
    });

    it("preserves perspective fov when switching kinds", () => {
      expect(worldProjectionKindSwitchSpec("twoPoint", { mode: { kind: "threePoint", fov: 72 }, orientation: { type: "free" } })).toEqual({ mode: { kind: "twoPoint", fov: 72, verticalShift: 0 }, orientation: { type: "free" } });
    });

    it("preserves gizmo orientation when switching mode", () => {
      expect(worldProjectionKindSwitchSpec("orthographic", { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } })).toEqual({
        mode: { kind: "orthographic" },
        orientation: { type: "cardinal", view: "front" },
      });
      expect(worldProjectionKindSwitchSpec("curvilinear", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toEqual({
        mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" },
        orientation: { type: "cardinal", view: "top" },
      });
    });
  });

  describe("worldProjectionTemplateSelectionId / worldProjectionTemplateApplySpec / worldProjectionSwitchTreeItems", () => {
    it("selects the same leaf ids the display template tree uses", () => {
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } })).toBe("orthographic");
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } })).toBe("orthographic");
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "axonometric", variant: "dimetric", angleA: 15, angleB: 15 }, orientation: { type: "corner", quadrant: "sw", hemisphere: "lower" } })).toBe("axonometric-dimetric");
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "onePoint", fov: 40 }, orientation: { type: "cardinal", view: "left" } })).toBe("one-point");
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "panini" }, orientation: { type: "free" } })).toBe("curvilinear");
    });

    it("preserves gizmo orientation when applying a template mode", () => {
      const axo = { mode: { kind: "axonometric" as const, variant: "isometric" as const, angleA: 30, angleB: 30 }, orientation: { type: "corner" as const, quadrant: "nw" as const, hemisphere: "lower" as const } };
      expect(worldProjectionTemplateApplySpec({ mode: { kind: "axonometric", variant: "dimetric", angleA: 15, angleB: 15 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, axo)).toEqual({
        mode: { kind: "axonometric", variant: "dimetric", angleA: 15, angleB: 15 },
        orientation: { type: "corner", quadrant: "nw", hemisphere: "lower" },
      });
      expect(worldProjectionTemplateApplySpec({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } }, { mode: { kind: "onePoint", fov: 72 }, orientation: { type: "cardinal", view: "top" } })).toEqual({
        mode: { kind: "onePoint", fov: 50 },
        orientation: { type: "cardinal", view: "top" },
      });
    });

    it("mirrors createWorldProjectionTemplates labels and ids in the switch tree", () => {
      const templates = createWorldProjectionTemplates({ controllerId: "demo" });
      const items = worldProjectionSwitchTreeItems(templates, () => undefined);
      expect(items.map((row) => row.id)).toEqual(["parallel", "perspective"]);
      expect(items[0]!.items!.map((row) => row.label)).toEqual(["Orthographic", "Axonometric", "Oblique"]);
      expect(items[0]!.items![0]!.items).toBeUndefined();
      expect(items[0]!.items![1]!.items!.map((row) => row.label)).toEqual(["Isometric", "Dimetric", "Trimetric"]);
      expect(items[1]!.items!.map((row) => row.label)).toEqual(["1-Point", "2-Point", "3-Point", "Curvilinear"]);
    });
  });

  describe("worldProjectionModeOptions", () => {
    it("lists axonometric/oblique/curvilinear flat variants for callers that still want a ribbon", () => {
      expect(worldProjectionModeOptions({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toEqual([]);
      expect(worldProjectionModeOptions({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }).map((row) => row.label)).toEqual(["Iso", "Di", "Tri"]);
      expect(worldProjectionModeOptions({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } }).map((row) => row.label)).toEqual(["Cab", "Cav", "Mil"]);
      expect(worldProjectionModeOptions({ mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } }).map((row) => row.label)).toEqual(["Fish", "Pan"]);
    });
  });

  describe("resolveOrbitCameraViewFromTemplateId", () => {
    it("maps display-tree template ids to orbit views", () => {
      expect(resolveOrbitCameraViewFromTemplateId("top")).toBe("top");
      expect(resolveOrbitCameraViewFromTemplateId("orthographic-2d")).toBe("top");
      expect(resolveOrbitCameraViewFromTemplateId("perspective")).toBe("perspective");
      expect(resolveOrbitCameraViewFromTemplateId("missing")).toBeNull();
    });
  });

  describe("shouldApplyOrbitCameraViewRigSeed", () => {
    it("applies only when the seed token changes", () => {
      const token = orbitCameraViewRigApplyToken("win-a:3", "perspective");
      expect(shouldApplyOrbitCameraViewRigSeed(null, token)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(token, token)).toBe(false);
      expect(shouldApplyOrbitCameraViewRigSeed(token, orbitCameraViewRigApplyToken("win-a:4", "perspective"))).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(token, orbitCameraViewRigApplyToken("win-a:3", "orthographic"))).toBe(true);
    });

    it("re-applies when orbit controls become ready after the rig camera mounts", () => {
      const beforeControls = `${orbitCameraViewRigApplyToken("preview", "perspective")}:controls:0`;
      const afterControls = `${orbitCameraViewRigApplyToken("preview", "perspective")}:controls:1`;
      expect(shouldApplyOrbitCameraViewRigSeed(null, beforeControls)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(beforeControls, afterControls)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(afterControls, afterControls)).toBe(false);
    });
  });

  describe("applyOrbitProjectionToCameraState", () => {
    it("applies orthographic zoom defaults while preserving pose", () => {
      const state = applyOrbitProjectionToCameraState({ position: [100, 0, 50], target: [0, 0, 0], zoom: 1, projection: "perspective" }, "orthographic");
      expect(state.projection).toBe("orthographic");
      expect(state.zoom).toBe(50);
      expect(state.position).toEqual([100, 0, 50]);
    });
  });

  describe("createOrbitCameraViewTemplates", () => {
    it("emits the orthographic/perspective template tree", () => {
      const templates = createOrbitCameraViewTemplates({ controllerId: "demo" });
      expect(templates.map((row) => row.id)).toEqual(["orthographic", "perspective"]);
      const ortho2d = templates[0]!.children![0]!;
      expect(ortho2d.id).toBe("orthographic-2d");
      expect(ortho2d.children!.map((row) => row.id)).toEqual(["top", "bottom", "front", "back", "right", "left"]);
      const isometry = templates[0]!.children![1]!.children![0]!;
      expect(isometry.children!.map((row) => row.id)).toEqual(["isometricNe", "isometricNw", "isometricSe", "isometricSw"]);
      expect(templates[0]).toMatchObject({ controllerId: "demo", command: ORBIT_CAMERA_VIEW_COMMAND, args: { view: "top" } });
    });

    it("still supports flat custom view lists", () => {
      const templates = createOrbitCameraViewTemplates({ controllerId: "demo", views: ["top", "front"] });
      expect(templates.map((row) => row.id)).toEqual(["top", "front"]);
    });
  });

  describe("createOrbitCameraViewLayoutDescriptors", () => {
    it("includes grouped single and quad layouts", () => {
      const layouts = createOrbitCameraViewLayoutDescriptors();
      expect(layouts.some((row) => row.id === "view-quad-standard")).toBe(true);
      expect(layouts.find((row) => row.id === "view-single-top")?.groupPath).toEqual(["Single", "2D"]);
    });

    it("sizes Top | Perspective as one-third / two-thirds", () => {
      const layout = createOrbitCameraViewLayoutDescriptors().find((row) => row.id === "view-dual-plan-perspective");
      expect(layout?.arrangement).toEqual({
        kind: "row",
        panes: [
          { view: "top", size: 100 / 3 },
          { view: "perspective", size: 200 / 3 },
        ],
      });
    });
  });

  describe("worldProjectionFamily", () => {
    it("treats orthographic/axonometric/oblique as parallel and everything else as perspective", () => {
      expect(worldProjectionFamily({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toBe("parallel");
      expect(worldProjectionFamily({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } })).toBe("parallel");
      expect(worldProjectionFamily({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } })).toBe("parallel");
      expect(worldProjectionFamily({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })).toBe("perspective");
      expect(worldProjectionFamily({ mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } })).toBe("perspective");
      expect(worldProjectionFamily(undefined)).toBe("perspective");
    });
  });

  describe("worldProjectionGumballPlane", () => {
    it("maps planar orthographic views to drafting planes and leaves 3D projections unconstrained", () => {
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "bottom" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } })).toBe("xz");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "back" } })).toBe("xz");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "left" } })).toBe("yz");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "right" } })).toBe("yz");
      expect(worldProjectionGumballPlane({ mode: { kind: "oblique", variant: "military", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "plan" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } })).toBe("xz");
      expect(worldProjectionGumballPlane({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } })).toBe("xz");
      expect(worldProjectionGumballPlane({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "left" } })).toBe("yz");
      expect(worldProjectionGumballPlane({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "top" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } })).toBeUndefined();
      expect(worldProjectionGumballPlane({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })).toBeUndefined();
      expect(worldProjectionGumballPlane(undefined)).toBeUndefined();
    });

    it("maps legacy orbit camera view ids the same way", () => {
      expect(orbitCameraViewGumballPlane("top")).toBe("xy");
      expect(orbitCameraViewGumballPlane("front")).toBe("xz");
      expect(orbitCameraViewGumballPlane("right")).toBe("yz");
      expect(orbitCameraViewGumballPlane("isometricNe")).toBeUndefined();
      expect(orbitCameraViewGumballPlane("perspective")).toBeUndefined();
      expect(orbitCameraViewGumballPlane(undefined)).toBeUndefined();
    });
  });

  describe("computeWorldProjectionPose", () => {
    it("places plan/top directly above the target with orthographic projection", () => {
      const state = computeWorldProjectionPose({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } }, { target: [0, 0, 40], distance: 800 });
      expect(state).toMatchObject({ position: [0, 0, 840], target: [0, 0, 40], up: [0, 1, 0], projection: "orthographic", zoom: 50 });
      const plan = computeWorldProjectionPose({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } }, { target: [0, 0, 40], distance: 800 });
      expect(plan.position).toEqual(state.position);
    });

    it("accepts an explicit orthographic zoom including values below the legacy default", () => {
      const state = computeWorldProjectionPose({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } }, { target: [7, 0, 0], distance: 100, zoom: 6.4 });
      expect(state.zoom).toBe(6.4);
    });

    it("preserves live parallel zoom across gizmo snaps and upgrades perspective unit zoom", () => {
      expect(worldProjectionSnapZoom({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, 12.5, true)).toBe(12.5);
      expect(worldProjectionSnapZoom({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } }, 1, true)).toBe(1);
      expect(worldProjectionSnapZoom({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "sw", hemisphere: "upper" } }, 1, false)).toBe(50);
      expect(worldProjectionSnapZoom({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }, 50, true)).toBe(50);
    });

    it("derives the classic 35.264/45 isometric direction from the 30/30 axis angles", () => {
      const state = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, { target: [0, 0, 0], distance: 10 });
      expect(state.position[2] / 10).toBeCloseTo(Math.sin((35.264 * Math.PI) / 180), 3);
      const azimuth = (Math.atan2(state.position[0], state.position[1]) * 180) / Math.PI;
      expect(azimuth).toBeCloseTo(45, 2);
    });

    it("mirrors quadrant sign flips for axonometric corners", () => {
      const ne = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, { target: [0, 0, 0], distance: 10 });
      const sw = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "sw", hemisphere: "upper" } }, { target: [0, 0, 0], distance: 10 });
      expect(sw.position[0]).toBeCloseTo(-ne.position[0], 5);
      expect(sw.position[1]).toBeCloseTo(-ne.position[1], 5);
    });

    it("places lower-hemisphere axonometric corners below the target", () => {
      const upper = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, { target: [0, 0, 0], distance: 10 });
      const lower = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "lower" } }, { target: [0, 0, 0], distance: 10 });
      expect(upper.position[2]).toBeGreaterThan(0);
      expect(lower.position[2]).toBeLessThan(0);
      expect(lower.position[0]).toBeCloseTo(upper.position[0], 5);
      expect(lower.position[1]).toBeCloseTo(upper.position[1], 5);
    });

    it("keeps oblique cabinet/cavalier at the front pose and rotates military's up vector by its angle", () => {
      const cavalier = computeWorldProjectionPose({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } }, { target: [0, 0, 0], distance: 10 });
      expect(cavalier.position).toEqual([0, -10, 0]);
      const military = computeWorldProjectionPose({ mode: { kind: "oblique", variant: "military", angle: 30, depthScale: 1 }, orientation: { type: "cardinal", view: "plan" } }, { target: [0, 0, 0], distance: 10 });
      expect(military.position).toEqual([0, 0, 10]);
      expect(military.up![0]).toBeCloseTo(Math.sin((30 * Math.PI) / 180), 5);
    });

    it("reports perspective projection for the perspective family", () => {
      const threePoint = computeWorldProjectionPose({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }, { target: [0, 0, 0], distance: 100 });
      expect(threePoint.projection).toBe("perspective");
      expect(Math.hypot(...threePoint.position)).toBeGreaterThan(90);
    });
  });

  describe("worldProjectionTransitionPose", () => {
    it("keeps eye and target when only mode changes with the same orientation", () => {
      const live = {
        position: [12, -8, 40] as const,
        target: [3, 5, 10] as const,
        up: [0, 0, 1] as const,
        zoom: 6.4,
        isOrthographic: true,
        projectionSpec: { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "top" } } as const,
      };
      const pending = { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } } as const;
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.position).toEqual(live.position);
      expect(next.target).toEqual(live.target);
      expect(next.up).toEqual(live.up);
      expect(next.projection).toBe("orthographic");
      expect(next.projectionSpec).toEqual(pending);
      expect(next.zoom).toBe(6.4);
    });

    it("re-looks from the same target and distance when orientation changes", () => {
      const live = {
        position: [40, 20, 80] as const,
        target: [5, 5, 5] as const,
        up: [0, 0, 1] as const,
        zoom: 1,
        isOrthographic: false,
        projectionSpec: { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } } as const,
      };
      const pending = { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "top" } } as const;
      const distance = orbitCameraDistance({ position: [...live.position], target: [...live.target], zoom: live.zoom });
      const expected = computeWorldProjectionPose(pending, { target: [...live.target], distance, zoom: 1 });
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.target).toEqual(live.target);
      expect(next.position).toEqual(expected.position);
      expect(next.up).toEqual(expected.up);
      expect(next.projectionSpec).toEqual(pending);
    });

    it("matches apparent scale (not the legacy zoom-50 default) for a persp→ortho mode-only switch when a viewport is supplied", () => {
      const live = {
        position: [0, -100, 0] as const,
        target: [0, 0, 0] as const,
        up: [0, 0, 1] as const,
        zoom: 1,
        isOrthographic: false,
        projectionSpec: { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } } as const,
        viewport: { width: 800, height: 600 },
      };
      const pending = { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } } as const;
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.position).toEqual(live.position);
      expect(next.zoom).toBeCloseTo(worldProjectionMatchedOrthoZoom(50, 100, 600), 6);
      expect(next.zoom).not.toBe(50);
    });

    it("never moves the camera for an ortho→persp mode-only switch even when a viewport is supplied", () => {
      const live = {
        position: [0, -50, 0] as const,
        target: [0, 0, 0] as const,
        up: [0, 0, 1] as const,
        zoom: 10,
        isOrthographic: true,
        projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } } as const,
        viewport: { width: 800, height: 600 },
      };
      const pending = { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } } as const;
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.position).toEqual(live.position);
      expect(next.zoom).toBe(live.zoom);
    });

    it("never moves the camera across a perspective FOV change with the same orientation even when a viewport is supplied", () => {
      const live = {
        position: [0, -100, 0] as const,
        target: [0, 0, 0] as const,
        up: [0, 0, 1] as const,
        zoom: 1,
        isOrthographic: false,
        projectionSpec: { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } } as const,
        viewport: { width: 800, height: 600 },
      };
      const pending = { mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" as const }, orientation: { type: "cardinal", view: "front" } } as const;
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.position).toEqual(live.position);
    });
  });

  describe("worldProjectionPerspectiveFov", () => {
    it("defaults threePoint/onePoint/twoPoint fov and caps curvilinear at 160°", () => {
      expect(worldProjectionPerspectiveFov({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })).toBe(50);
      expect(worldProjectionPerspectiveFov({ mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } })).toBe(120);
      expect(worldProjectionPerspectiveFov({ mode: { kind: "curvilinear", fov: 200, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } })).toBe(160);
    });
  });

  describe("worldProjectionMatchedOrthoZoom", () => {
    it("scales inversely with distance and directly with viewport height", () => {
      const zoom = worldProjectionMatchedOrthoZoom(50, 120, 720);
      expect(worldProjectionMatchedOrthoZoom(50, 240, 720)).toBeCloseTo(zoom / 2, 6);
      expect(worldProjectionMatchedOrthoZoom(50, 120, 1440)).toBeCloseTo(zoom * 2, 6);
    });
  });

  describe("worldObliqueShearMatrix", () => {
    it("shears cavalier/cabinet by depthScale·angle and military by a fixed unit length at a right angle", () => {
      const cavalier = worldObliqueShearMatrix({ kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 });
      expect(cavalier.elements[8]).toBeCloseTo(-Math.cos(Math.PI / 4), 5);
      expect(cavalier.elements[9]).toBeCloseTo(-Math.sin(Math.PI / 4), 5);
      const military = worldObliqueShearMatrix({ kind: "oblique", variant: "military", angle: 30, depthScale: 1 });
      expect(military.elements[8]).toBeCloseTo(0, 5);
      expect(military.elements[9]).toBeCloseTo(-1, 5);
    });

    it("ramps shear elements linearly with strength", () => {
      const full = worldObliqueShearMatrix({ kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, 1);
      const half = worldObliqueShearMatrix({ kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, 0.5);
      expect(half.elements[8]).toBeCloseTo(full.elements[8] * 0.5, 5);
      expect(half.elements[9]).toBeCloseTo(full.elements[9] * 0.5, 5);
    });
  });

  describe("worldProjectionGoalMatrix", () => {
    const viewport = { width: 800, height: 600 };

    it("matches a real OrthographicCamera's pixel frustum for parallel specs", () => {
      const spec = { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } } as const;
      const goal = worldProjectionGoalMatrix(spec, { zoom: 12.5, viewport, near: 0.2, far: WORLD_ORBIT_CAMERA_MIN_FAR });
      const cam = new ThreeOrthographicCamera(viewport.width / -2, viewport.width / 2, viewport.height / 2, viewport.height / -2, 0.2, WORLD_ORBIT_CAMERA_MIN_FAR);
      cam.zoom = 12.5;
      cam.updateProjectionMatrix();
      for (let i = 0; i < 16; i++) expect(goal.elements[i]).toBeCloseTo(cam.projectionMatrix.elements[i], 6);
    });

    it("matches a real PerspectiveCamera for perspective specs", () => {
      const spec = { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } } as const;
      const goal = worldProjectionGoalMatrix(spec, { zoom: 1, viewport, near: 0.2, far: WORLD_ORBIT_CAMERA_MIN_FAR });
      const cam = new ThreePerspectiveCamera(50, viewport.width / viewport.height, 0.2, WORLD_ORBIT_CAMERA_MIN_FAR);
      cam.zoom = 1;
      cam.updateProjectionMatrix();
      for (let i = 0; i < 16; i++) expect(goal.elements[i]).toBeCloseTo(cam.projectionMatrix.elements[i], 6);
    });

    it("post-multiplies the oblique shear onto the orthographic base", () => {
      const spec = { mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } } as const;
      const goal = worldProjectionGoalMatrix(spec, { zoom: 1, viewport });
      const base = worldProjectionGoalMatrix({ mode: { kind: "orthographic" }, orientation: spec.orientation }, { zoom: 1, viewport });
      const expected = base.clone().multiply(worldObliqueShearMatrix(spec.mode));
      for (let i = 0; i < 16; i++) expect(goal.elements[i]).toBeCloseTo(expected.elements[i], 6);
    });

    it("shifts the two-point vertical projection element", () => {
      const spec = { mode: { kind: "twoPoint", fov: 50, verticalShift: 0.3 }, orientation: { type: "free" } } as const;
      const goal = worldProjectionGoalMatrix(spec, { zoom: 1, viewport });
      const base = worldProjectionGoalMatrix({ mode: { kind: "threePoint", fov: 50 }, orientation: spec.orientation }, { zoom: 1, viewport });
      expect(goal.elements[9]).toBeCloseTo(base.elements[9] + 0.3, 6);
    });
  });

  describe("worldProjectionMorphMatrix", () => {
    it("equals from exactly at t=0 and closely approximates to at t=1", () => {
      const from = new Matrix4().makePerspective(-1, 1, 1, -1, 0.1, 100);
      const to = new Matrix4().makeOrthographic(-1, 1, 1, -1, 0.1, 100);
      expect(worldProjectionMorphMatrix(from, to, 0).elements).toEqual(from.elements);
      const atOne = worldProjectionMorphMatrix(from, to, 1);
      for (let i = 0; i < 16; i++) expect(atOne.elements[i]).toBeCloseTo(to.elements[i], 10);
    });

    it("interpolates every element linearly", () => {
      const from = new Matrix4().identity();
      const to = new Matrix4().identity().multiplyScalar(2);
      const mid = worldProjectionMorphMatrix(from, to, 0.5);
      for (let i = 0; i < 16; i++) expect(mid.elements[i]).toBeCloseTo((from.elements[i] + to.elements[i]) / 2, 10);
    });

    it("ramps oblique shear to half-magnitude at t=0.5 when morphing from an unsheared ortho matrix", () => {
      const viewport = { width: 800, height: 600 };
      const from = worldProjectionGoalMatrix({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } }, { zoom: 10, viewport });
      const goal = worldProjectionGoalMatrix({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } }, { zoom: 10, viewport });
      const mid = worldProjectionMorphMatrix(from, goal, 0.5);
      expect(mid.elements[8]).toBeCloseTo(goal.elements[8] * 0.5, 6);
      expect(mid.elements[9]).toBeCloseTo(goal.elements[9] * 0.5, 6);
    });
  });

  describe("projectionGizmoHover", () => {
    const topFace: ProjectionGizmoHit = { type: "face", axis: "z", sign: 1 };
    const freeCenter: ProjectionGizmoHit = { type: "center" };
    const palette = resolveProjectionGizmoVisualPalette();

    it("marks the hovered hit and dims the rest", () => {
      expect(projectionGizmoHitVisualState(topFace, topFace)).toBe("hover");
      expect(projectionGizmoHitVisualState(freeCenter, topFace)).toBe("dimmed");
      expect(projectionGizmoHitVisualState(topFace, null)).toBe("idle");
    });

    it("brightens axis and neutral fills on hover", () => {
      const axisHover = projectionGizmoHeadFillColor("#ff344f", "hover", palette, false);
      const neutralHover = projectionGizmoHeadFillColor("#9aa0ab", "hover", palette, true);
      expect(axisHover).not.toBe("#ff344f");
      expect(neutralHover).toBe(palette.neutralHover);
    });
  });

  describe("frameWorldProjectionPose", () => {
    it("centers top orthographic on reference bounds and fits width in the viewport", () => {
      const bounds = worldSceneContentBounds([], [{ origin: [7, 0, 0.01], widthWorld: 50 }]);
      expect(bounds).toEqual({ center: [7, 0, 0.01], halfExtent: [25, 25, 0.5] });
      const state = frameWorldProjectionPose({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } }, bounds!, { viewportWidth: 400, viewportHeight: 800, padding: 1.35 });
      expect(state.target).toEqual([7, 0, 0.01]);
      expect(state.position[0]).toBe(7);
      expect(state.position[1]).toBe(0);
      expect(state.position[2]).toBeGreaterThan(0.01);
      expect(state.projection).toBe("orthographic");
      // visible half-width = (viewportWidth/2) / zoom = 25 * 1.35 ⇒ zoom = 200 / 33.75
      expect(state.zoom).toBeCloseTo(200 / (25 * 1.35), 5);
    });
  });

  describe("worldProjectionOrbitConstraints", () => {
    it("locks rotation for plan, oblique, and one-point; locks polar for two-point; frees the rest", () => {
      expect(worldProjectionOrbitConstraints({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } }).rotate).toBe(false);
      expect(worldProjectionOrbitConstraints({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } }).rotate).toBe(true);
      expect(worldProjectionOrbitConstraints({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } }).rotate).toBe(false);
      expect(worldProjectionOrbitConstraints({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } }).rotate).toBe(false);
      const twoPoint = worldProjectionOrbitConstraints({ mode: { kind: "twoPoint", fov: 50, verticalShift: 0 }, orientation: { type: "free" } });
      expect(twoPoint.minPolar).toBeCloseTo(Math.PI / 2);
      expect(twoPoint.maxPolar).toBeCloseTo(Math.PI / 2);
      expect(worldProjectionOrbitConstraints({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }).rotate).toBe(true);
    });
  });

  describe("worldCurvilinearUnproject", () => {
    it("is the identity mapping at strength 0 (rectilinear passthrough)", () => {
      const mode = { kind: "curvilinear" as const, fov: 120, strength: 0, mapping: "fisheye" as const };
      const [x, y] = worldCurvilinearUnproject([0.3, 0.2], mode);
      expect(x).toBeCloseTo(0.3, 5);
      expect(y).toBeCloseTo(0.2, 5);
    });
  });

  describe("WorldCurvilinearPass capture quality", () => {
    it("stores the capture in linear working space with linear filtering (no nearest-pixelation)", () => {
      expect(WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS.magFilter).toBe(LinearFilter);
      expect(WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS.minFilter).toBe(LinearFilter);
      expect(WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS.colorSpace).toBe(LinearSRGBColorSpace);
      expect(WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS.type).toBe(HalfFloatType);
    });

    it("applies Three output color-space conversion on the fisheye blit", () => {
      expect(WORLD_CURVILINEAR_FRAGMENT_SHADER).toContain("#include <colorspace_fragment>");
      expect(WORLD_CURVILINEAR_FRAGMENT_SHADER).toContain("texture2D(tCapture, sourceUv)");
    });
  });

  describe("worldProjectionSpecIconId", () => {
    it("maps orthographic and three-point specs to distinct projection icons", () => {
      expect(worldProjectionSpecIconId(worldProjectionDefaults("orthographic"))).toBe("projection-orthographic");
      expect(worldProjectionSpecIconId(worldProjectionDefaults("threePoint"))).toBe("projection-three-point");
      expect(worldProjectionSpecIconId(worldProjectionDefaults("orthographic"))).not.toBe(worldProjectionSpecIconId(worldProjectionDefaults("threePoint")));
    });
  });

  describe("createWorldProjectionTemplates", () => {
    it("emits Parallel/Perspective mode tree without gizmo orientations", () => {
      const templates = createWorldProjectionTemplates({ controllerId: "demo" });
      expect(templates.map((row) => row.id)).toEqual(["parallel", "perspective"]);
      const [parallel, perspective] = templates;
      expect(parallel!.children!.map((row) => row.id)).toEqual(["orthographic", "axonometric", "oblique"]);
      expect(parallel!.children![0]!.children).toBeUndefined();
      expect(parallel!.children![1]!.children!.map((row) => row.label)).toEqual(["Isometric", "Dimetric", "Trimetric"]);
      expect(parallel!.children![2]!.children!.map((row) => row.label)).toEqual(["Cabinet", "Cavalier", "Military"]);
      expect(perspective!.children!.map((row) => row.label)).toEqual(["1-Point", "2-Point", "3-Point", "Curvilinear"]);
      expect(parallel!.children![0]!.iconId).toBe("projection-orthographic");
      expect(perspective!.children![2]!.iconId).toBe("projection-three-point");
      expect(templates[0]).toMatchObject({ controllerId: "demo", command: WORLD_PROJECTION_COMMAND });
    });
  });

  describe("encodeWorldProjectionTemplateId / decodeWorldProjectionTemplateId", () => {
    it("round-trips a spec through the template id string", () => {
      const spec: WorldProjectionSpec = { mode: { kind: "axonometric", variant: "dimetric", angleA: 15, angleB: 15 }, orientation: { type: "corner", quadrant: "nw", hemisphere: "upper" } };
      const encoded = encodeWorldProjectionTemplateId(spec);
      expect(typeof encoded).toBe("string");
      expect(decodeWorldProjectionTemplateId(encoded)).toEqual(spec);
    });

    it("returns null for undefined, unrelated, or malformed template ids", () => {
      expect(decodeWorldProjectionTemplateId(undefined)).toBeNull();
      expect(decodeWorldProjectionTemplateId("top")).toBeNull();
      expect(decodeWorldProjectionTemplateId("world-projection:not-json")).toBeNull();
    });
  });

  describe("worldProjectionSpecLabel", () => {
    it("matches the exact labels createWorldProjectionTemplates uses for every leaf, so a dragged pane's title matches the tree entry it came from", () => {
      const templates = createWorldProjectionTemplates({ controllerId: "demo" });
      const collectLeafLabels = (nodes: readonly WorldProjectionTemplateDescriptor[]): readonly [string, WorldProjectionSpec][] =>
        nodes.flatMap((node) => (node.children?.length ? collectLeafLabels(node.children) : [[node.label, node.args.spec] as const]));
      for (const [label, spec] of collectLeafLabels(templates)) {
        expect(worldProjectionSpecLabel(spec)).toBe(label);
      }
      expect(worldProjectionSpecLabel({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } })).toBe("Orthographic");
    });
  });

  describe("floatingOriginRebase", () => {
    it("subtracts anchor", () => {
      expect(floatingOriginRebase([1000, 2000, 3], [1000, 1990, 0])).toEqual([0, 10, 3]);
    });
  });

  describe("lodFromCameraDistance", () => {
    it("maps orbit distance to scale ratio", () => {
      expect(lodFromCameraDistance(100, 100)).toBe(1);
      expect(lodFromCameraDistance(20000, 100)).toBe(200);
    });
  });

  describe("lodGridStepWorld", () => {
    it("keeps the configured spacing at close range and stays sparse at every larger LOD", () => {
      expect(lodGridStepWorld(1, 7.5)).toBe(7.5);
      expect(lodGridStepWorld(2, 10)).toBe(10);
      expect(lodGridStepWorld(10, 10)).toBe(50);
      expect(lodGridStepWorld(50, 10)).toBe(250);
      expect(lodGridStepWorld(500, 10)).toBe(2_500);
      expect(lodGridStepWorld(5_000, 10)).toBe(25_000);
      expect(lodGridStepWorld(Number.POSITIVE_INFINITY, 10)).toBeNull();
    });
  });

  describe("cameraGridFadeDistance", () => {
    it("grows with camera Z while fading fully before the far clipping plane", () => {
      const camera = new ThreePerspectiveCamera(45, 16 / 9, 0.1, 500_000);
      camera.position.set(0, -100, 100);
      const near = cameraGridFadeDistance(camera, 0, 10);
      camera.position.z = 2_000;
      const far = cameraGridFadeDistance(camera, 0, 10);
      expect(far).toBeGreaterThan(near);
      expect(far).toBeLessThanOrEqual(camera.far * 0.25);
    });
  });

  describe("adaptiveOrbitCameraFar", () => {
    it("keeps the far plane ahead of extreme orbit zoom in stable power-of-two bands", () => {
      expect(adaptiveOrbitCameraFar(100)).toBe(524_288);
      expect(adaptiveOrbitCameraFar(2_000)).toBe(2_097_152);
      expect(adaptiveOrbitCameraFar(20_000)).toBe(33_554_432);
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

  describe("applyWorldReferenceTransform", () => {
    it("writes gumball pose onto reference props", () => {
      const base: WorldReferenceProps = {
        id: "ref-a",
        source: { url: "/infinite-fixture/sketch.png", mediaKind: "image" },
        origin: [0, 0, 0],
      };
      const next = applyWorldReferenceTransform(base, {
        position: [1, 2, 3],
        quaternion: [0, 0, 0, 1],
        scale: [2, 2, 2],
      });
      expect(next.origin).toEqual([1, 2, 3]);
      expect(next.scale).toEqual([2, 2, 2]);
    });
  });

  describe("worldReferenceAppearance", () => {
    it("returns hover and selection fills with halved content opacity when selected", () => {
      expect(worldReferenceAppearance({ asHover: false, showSelectedOutline: false }, 1)).toMatchObject({
        backgroundColor: null,
        contentOpacity: 1,
        outlineColor: null,
      });
      expect(worldReferenceAppearance({ asHover: true, showSelectedOutline: false }, 0.8)).toMatchObject({
        backgroundColor: expect.stringMatching(/^#/),
        contentOpacity: 0.8,
        outlineColor: expect.stringMatching(/^#/),
      });
      expect(worldReferenceAppearance({ asHover: true, showSelectedOutline: true }, 1)).toMatchObject({
        backgroundColor: tokenHex("primary"),
        contentOpacity: WORLD_REFERENCE_SELECTED_CONTENT_OPACITY,
        outlineColor: tokenHex("primary"),
      });
    });
  });

  describe("applyWorldVolumeTransform", () => {
    it("writes gumball pose onto volume props", () => {
      const base: WorldVolumeProps = { id: "vol-a", origin: [0, 0, 0] };
      const next = applyWorldVolumeTransform(base, {
        position: [1, 2, 3],
        quaternion: [0, 0, 0, 1],
        scale: [4, 6, 8],
      });
      expect(next.origin).toEqual([1, 2, 3]);
      expect(next.scale).toEqual([4, 6, 8]);
    });
  });

  describe("worldVolumesContainAabb", () => {
    it("returns true when no volumes are defined", () => {
      expect(worldVolumesContainAabb([], [0, 0, 0], [1, 1, 1])).toBe(true);
    });

    it("accepts an AABB fully inside a unit volume at the origin", () => {
      const volumes: WorldVolumeProps[] = [{ id: "v1", origin: [0, 0, 0], scale: [10, 10, 10] }];
      expect(worldVolumesContainAabb(volumes, [-2, -2, -2], [2, 2, 2])).toBe(true);
    });

    it("rejects an AABB that extends outside the volume", () => {
      const volumes: WorldVolumeProps[] = [{ id: "v1", origin: [0, 0, 0], scale: [4, 4, 4] }];
      expect(worldVolumesContainAabb(volumes, [-3, -3, -3], [3, 3, 3])).toBe(false);
    });
  });

  describe("worldReferencePose", () => {
    it("patchWorldReferenceProps updates origin rotation and scale", () => {
      const base: WorldReferenceProps = {
        id: "ref-a",
        source: { url: "/plan.png", mediaKind: "image" },
        origin: [0, 0, 0],
      };
      expect(patchWorldReferenceProps(base, "origin", [1, 2, 3])?.origin).toEqual([1, 2, 3]);
      const rotated = patchWorldReferenceProps(base, "rotation", [0, 90, 0]);
      expect(rotated?.orientation?.length).toBe(4);
      expect(patchWorldReferenceProps(base, "scaleUniform", 2)?.scale).toBe(2);
      expect(patchWorldReferenceProps(base, "widthWorld", 42)?.widthWorld).toBe(42);
    });
  });

  describe("worldEntityFlags", () => {
    it("treats hidden and locked entities as non-selectable", () => {
      expect(worldEntitySelectable(undefined)).toBe(true);
      expect(worldEntitySelectable({ hidden: true })).toBe(false);
      expect(worldEntitySelectable({ locked: true })).toBe(false);
    });

    it("excludes locked entities from canvas pick so a click equals background", () => {
      expect(worldEntitySelectable({ locked: true })).toBe(false);
      expect(worldEntityInspectable({ locked: true })).toBe(true);
    });

    it("treats hidden entities as non-inspectable and locked entities as inspectable", () => {
      expect(worldEntityInspectable(undefined)).toBe(true);
      expect(worldEntityInspectable({ hidden: true })).toBe(false);
      expect(worldEntityInspectable({ locked: true })).toBe(true);
    });

    it("reveals hidden entities on demand", () => {
      expect(worldEntityRendered({ hidden: true }, false)).toBe(false);
      expect(worldEntityRendered({ hidden: true }, true)).toBe(true);
      expect(worldEntityRendered({ locked: true }, false)).toBe(true);
    });

    it("resolves render mode for hidden reveal and locked dim", () => {
      expect(worldEntityRenderMode({ hidden: true }, { revealed: true, hovered: false })).toMatchObject({
        visible: true,
        asHover: true,
        dim: false,
        showSelectedOutline: false,
      });
      expect(worldEntityRenderMode({ locked: true }, { hovered: true, selected: true })).toMatchObject({
        visible: true,
        asHover: false,
        dim: true,
        showSelectedOutline: false,
      });
      expect(worldEntityRenderMode({ hidden: true, locked: true }, { revealed: true, hovered: true, selected: true })).toMatchObject({
        visible: true,
        asHover: true,
        dim: false,
        showSelectedOutline: false,
      });
    });
  });
}
// #endregion 🧪Tests
