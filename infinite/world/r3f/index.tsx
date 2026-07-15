// #region 🧲Header
/** @emoji 🌍 `@semio-tech/infinite-world-r3f` — generic r3f infinite-world engine: layers, chunking, view radius, pooling, precision, LOD/grid, mesh borders. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
  cn,
  floatingToolbarSurfaceClass,
  menuListItemClassName,
  reactHostPort,
  resolveSceneGizmoViewportPlacement,
  sceneHostPort,
  referenceMediaPort,
  UnifiedGumball,
  gumballConfigVisible,
  gumballHandleKindToTransformMode,
  useCanvasAppearanceSync,
  type GumballConfig,
  type GumballPose,
  type ReactNode,
  type ThreeEvent,
} from "@semio-tech/ui-react";
import { clearColorResolveCache, resolveColorHex, resolveThreeColor, semanticVar, themeColorVar, tokenHex, tokenVar } from "@semio-tech/ui-styling";
import React, { Children, isValidElement, type CSSProperties, type MutableRefObject, type ReactElement } from "react";
import { OrbitControls as ThreeOrbitControls } from "three/addons/controls/OrbitControls.js";
import { MeshBVH, type HitPointInfo } from "three-mesh-bvh";

const Canvas = sceneHostPort.fiber.canvas;
const useFrame = sceneHostPort.fiber.useFrame;
const useThree = sceneHostPort.fiber.useThree;
const GizmoHelper = sceneHostPort.drei.GizmoHelper;
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
  GridHelper,
  Group,
  LineBasicMaterial,
  LineSegments,
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

/** @emoji 📐 Orbit view projection mode for display templates. */
export type OrbitCameraProjection = "orthographic" | "perspective";

export interface WorldCameraState {
  readonly position: Vec3;
  readonly target: Vec3;
  readonly zoom: number;
  readonly up?: Vec3;
  readonly projection?: OrbitCameraProjection;
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

/** @emoji 👁️ Whether an entity participates in canvas pick, hover, and edit interactions. */
export function worldEntitySelectable(flags: WorldEntityFlags | undefined): boolean {
  return flags?.hidden !== true && flags?.locked !== true;
}

/** @emoji 🔎 Whether an entity can be selected for inspection (details panel) while locked. */
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

/** @emoji 📐 CAD anchor for grid layers: orbit pan XY, datum Z from {@link gridPlacementAnchorCad}. */
export function gridPlacementAnchorCad(controlsTargetThree: Vector3 | null | undefined, datum: Vec3 = [0, 0, 0]): Vec3 {
  if (!controlsTargetThree) {
    return datum;
  }
  const orbitCad = threeVec3ToCad(controlsTargetThree);
  return [orbitCad[0], orbitCad[1], datum[2]];
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

/** @emoji 🔑 Stable identity for progressive grid topology (ignores continuous camera LOD drift). */
export function lodProgressiveGridLayerKey(lod: number, gridFactor: number): string {
  const layers = lodProgressiveGridLayers(lod, gridFactor);
  if (!layers.length) return "";
  return layers.map((layer) => layer.stepWorld).join("|");
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
  const invalidate = useThree((s) => s.invalidate);
  const controls = useThree((s) => s.controls as { target?: Vector3 } | null);
  const anchor = controls?.target;
  const gridLayerKey = reactHostPort.useMemo(() => lodProgressiveGridLayerKey(lod.lod, lod.gridFactor), [lod.lod, lod.gridFactor]);
  const layers = reactHostPort.useMemo(() => lodProgressiveGridLayers(lod.lod, lod.gridFactor), [gridLayerKey, lod.gridFactor]);
  const [gridColor, setGridColor] = reactHostPort.useState<number>(() => resolveThreeColor(themeColorVar("element"), "gray"));
  useCanvasAppearanceSync(reactHostPort.useCallback(() => setGridColor(resolveThreeColor(themeColorVar("element"), "gray")), []));
  const grids = reactHostPort.useMemo(() => {
    const size = 12_000;
    return layers.map(({ stepWorld, opacity }) => {
      const divs = Math.min(512, Math.max(2, Math.round(size / stepWorld)));
      const grid = new GridHelper(size, divs, gridColor, gridColor);
      grid.rotation.x = Math.PI / 2;
      applyLodGridLayerStyle(grid, opacity);
      return grid;
    });
  }, [layers, gridColor]);
  reactHostPort.useEffect(
    () => () => {
      for (const grid of grids) grid.dispose();
    },
    [grids],
  );
  reactHostPort.useLayoutEffect(() => {
    if (!grids.length) return;
    invalidate();
  }, [grids, invalidate]);
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
  const invalidate = useThree((s) => s.invalidate);
  const tmpT = reactHostPort.useMemo(() => new Vector3(), []);
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
    const gridLayerKey = lodProgressiveGridLayerKey(sceneLod, props.gridFactor);
    const sig = `${gridLayerKey}|${props.depthVariableLod ? 1 : 0}|${gridStep ?? "x"}|${props.gridFactor}|${props.gridSnapEnabled}`;
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
  if (camera instanceof ThreePerspectiveCamera || camera instanceof ThreeOrthographicCamera) {
    camera.zoom = state.zoom;
    camera.updateProjectionMatrix();
  }
}

/** @emoji 🧭 Maps a CAD viewport gizmo axis click to a named orbit view (Z-up). */
export function resolveOrbitGizmoViewFromDirection(direction: { readonly x: number; readonly y: number; readonly z: number }): OrbitCameraViewId {
  const dominant = [
    { axis: "x" as const, magnitude: Math.abs(direction.x), sign: direction.x >= 0 ? 1 : -1 },
    { axis: "y" as const, magnitude: Math.abs(direction.y), sign: direction.y >= 0 ? 1 : -1 },
    { axis: "z" as const, magnitude: Math.abs(direction.z), sign: direction.z >= 0 ? 1 : -1 },
  ].sort((a, b) => b.magnitude - a.magnitude)[0] ?? { axis: "z" as const, magnitude: 1, sign: 1 };
  if (dominant.axis === "x") {
    return dominant.sign > 0 ? "right" : "left";
  }
  if (dominant.axis === "y") {
    return dominant.sign > 0 ? "back" : "front";
  }
  return dominant.sign > 0 ? "top" : "bottom";
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

export interface WorldOrbitViewSnapGateContextValue {
  readonly snapGate: boolean;
  readonly setSnapGate: (active: boolean) => void;
}

const WorldOrbitViewSnapGateContext = reactHostPort.createContext<WorldOrbitViewSnapGateContextValue | null>(null);

/** @emoji 🧭 Enables {@link useWorldOrbitViewSnapGate} for orbit controls during gizmo view snaps. */
export function WorldOrbitViewSnapGateProvider(props: { readonly children?: ReactNode }): ReactElement {
  const [snapGate, setSnapGate] = reactHostPort.useState(false);
  const value = reactHostPort.useMemo(() => ({ snapGate, setSnapGate }), [snapGate]);
  return <WorldOrbitViewSnapGateContext.Provider value={value}>{props.children}</WorldOrbitViewSnapGateContext.Provider>;
}

/** @emoji 🧭 Reads the active gizmo view-snap gate for {@link WorldOrbitGated}. */
export function useWorldOrbitViewSnapGate(): WorldOrbitViewSnapGateContextValue {
  return reactHostPort.useContext(WorldOrbitViewSnapGateContext) ?? { snapGate: false, setSnapGate: () => {} };
}

export interface WorldOrbitViewGizmoProps {
  readonly show?: boolean;
  readonly onViewSelect: (view: OrbitCameraViewId) => void;
}

function resolveWorldCadAxisColors(): [string, string, string] {
  return [resolveColorHex(semanticVar("accent"), "gray"), resolveColorHex(semanticVar("accent-secondary"), "gray"), resolveColorHex(semanticVar("accent-tertiary"), "gray")];
}

//#region 🧭WorldOrbitViewGizmoViewport
interface WorldOrbitViewGizmoViewportProps {
  readonly labels: [string, string, string];
  readonly axisColors: [string, string, string];
  readonly axisScale: [number, number, number];
  readonly axisHeadScale?: number;
  readonly hideNegativeAxes?: boolean;
  readonly labelColor: string;
  readonly font: string;
  readonly onClick?: (event: ThreeEvent<MouseEvent>) => unknown;
}

function WorldOrbitViewGizmoViewportAxis(props: { readonly scale: [number, number, number]; readonly color: string; readonly rotation: [number, number, number] }): ReactElement {
  return (
    <group rotation={props.rotation}>
      <mesh position={[0.4, 0, 0]}>
        <boxGeometry args={props.scale} />
        <meshBasicMaterial color={props.color} toneMapped={false} />
      </mesh>
    </group>
  );
}

function WorldOrbitViewGizmoViewportAxisHead(props: {
  readonly arcStyle: string;
  readonly position: [number, number, number];
  readonly label?: string;
  readonly font: string;
  readonly labelColor: string;
  readonly axisHeadScale: number;
  readonly onClick?: (event: ThreeEvent<MouseEvent>) => unknown;
}): ReactElement {
  const gl = useThree((state) => state.gl);
  const texture = reactHostPort.useMemo(() => {
    const canvas = document.createElement("canvas");
    canvas.width = 64;
    canvas.height = 64;
    const context = canvas.getContext("2d");
    if (!context) {
      return new CanvasTexture(canvas);
    }
    context.beginPath();
    context.arc(32, 32, 16, 0, 2 * Math.PI);
    context.closePath();
    context.fillStyle = props.arcStyle;
    context.fill();
    if (props.label) {
      context.font = props.font;
      context.textAlign = "center";
      context.fillStyle = props.labelColor;
      context.fillText(props.label, 32, 41);
    }
    return new CanvasTexture(canvas);
  }, [props.arcStyle, props.font, props.label, props.labelColor]);
  const [active, setActive] = reactHostPort.useState(false);
  const scale = (props.label ? 1 : 0.75) * (active ? 1.2 : 1) * props.axisHeadScale;
  return (
    <sprite
      scale={scale}
      position={props.position}
      onPointerOver={(event) => {
        event.stopPropagation();
        setActive(true);
      }}
      onPointerOut={(event) => {
        event.stopPropagation();
        setActive(false);
      }}
      onPointerDown={(event) => {
        event.stopPropagation();
      }}
      onClick={props.onClick}
    >
      <spriteMaterial map={texture} alphaTest={0.3} opacity={props.label ? 1 : 0.75} toneMapped={false} />
    </sprite>
  );
}

/** @emoji 🎯 Viewport gizmo heads with hover feedback only; camera snaps on click via {@link WorldOrbitViewSnapDriver}. */
function WorldOrbitViewGizmoViewport(props: WorldOrbitViewGizmoViewportProps): ReactElement {
  const [colorX, colorY, colorZ] = props.axisColors;
  const axisHeadScale = props.axisHeadScale ?? 1;
  const headProps = {
    font: props.font,
    labelColor: props.labelColor,
    axisHeadScale,
    onClick: props.onClick,
  };
  return (
    <group scale={40}>
      <WorldOrbitViewGizmoViewportAxis color={colorX} rotation={[0, 0, 0]} scale={props.axisScale} />
      <WorldOrbitViewGizmoViewportAxis color={colorY} rotation={[0, 0, Math.PI / 2]} scale={props.axisScale} />
      <WorldOrbitViewGizmoViewportAxis color={colorZ} rotation={[0, -Math.PI / 2, 0]} scale={props.axisScale} />
      <WorldOrbitViewGizmoViewportAxisHead arcStyle={colorX} position={[1, 0, 0]} label={props.labels[0]} {...headProps} />
      <WorldOrbitViewGizmoViewportAxisHead arcStyle={colorY} position={[0, 1, 0]} label={props.labels[1]} {...headProps} />
      <WorldOrbitViewGizmoViewportAxisHead arcStyle={colorZ} position={[0, 0, 1]} label={props.labels[2]} {...headProps} />
      {!props.hideNegativeAxes ? (
        <>
          <WorldOrbitViewGizmoViewportAxisHead arcStyle={colorX} position={[-1, 0, 0]} {...headProps} />
          <WorldOrbitViewGizmoViewportAxisHead arcStyle={colorY} position={[0, -1, 0]} {...headProps} />
          <WorldOrbitViewGizmoViewportAxisHead arcStyle={colorZ} position={[0, 0, -1]} {...headProps} />
        </>
      ) : null}
    </group>
  );
}
//#endregion 🧭WorldOrbitViewGizmoViewport

/** @emoji 🧭 CAD Z-up viewport gizmo (X / Y / Z) anchored bottom-right. */
export function WorldOrbitViewGizmo(props: WorldOrbitViewGizmoProps): ReactElement | null {
  const { size } = useThree();
  const [colors, setColors] = reactHostPort.useState<[string, string, string]>(() => resolveWorldCadAxisColors());
  const labels = reactHostPort.useMemo(() => ["X", "Y", "Z"] as [string, string, string], []);
  const placement = reactHostPort.useMemo(() => resolveSceneGizmoViewportPlacement(size), [size]);
  const axisScale = reactHostPort.useMemo(() => [0.88, 0.036, 0.036] as [number, number, number], []);
  const [labelColor, setLabelColor] = reactHostPort.useState<string>(() => resolveColorHex(semanticVar("foreground"), "gray"));

  useCanvasAppearanceSync(
    reactHostPort.useCallback(() => {
      setColors(resolveWorldCadAxisColors());
      setLabelColor(resolveColorHex(semanticVar("foreground"), "gray"));
    }, []),
  );

  if (props.show === false) {
    return null;
  }
  return (
    <GizmoHelper alignment={placement.alignment} margin={placement.margin}>
      <WorldOrbitViewGizmoViewport
        labels={labels}
        axisColors={colors}
        axisScale={axisScale}
        axisHeadScale={0.92}
        hideNegativeAxes
        labelColor={labelColor}
        font="16px Inter var, Arial, sans-serif"
        onClick={(event: ThreeEvent<MouseEvent>) => {
          props.onViewSelect(resolveOrbitGizmoViewFromDirection(event.object.position));
          return null;
        }}
      />
    </GizmoHelper>
  );
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
  const { camera } = useThree();
  const controls = useThree((state) => state.controls as OrbitControlsTarget | null);
  const invalidate = useThree((state) => state.invalidate);
  const { setSnapGate } = useWorldOrbitViewSnapGate();
  const animRef = reactHostPort.useRef<{ readonly view: OrbitCameraViewId; readonly fromPos: Vector3; readonly fromUp: Vector3; readonly to: WorldCameraState; readonly start: number } | null>(null);

  reactHostPort.useEffect(() => {
    if (!props.pendingView || !camera || !controls?.target) {
      return;
    }
    const pendingView = props.pendingView;
    const targetCad = threeVec3ToCad(controls.target);
    const distance = orbitCameraDistance({
      position: threeVec3ToCad(camera.position),
      target: targetCad,
      zoom: "zoom" in camera ? (camera as ThreePerspectiveCamera).zoom : 1,
    });
    const zoom = camera instanceof ThreeOrthographicCamera || camera instanceof ThreePerspectiveCamera ? camera.zoom : 1;
    const to = computeOrbitCameraViewState(pendingView, { target: targetCad, distance, zoom });
    animRef.current = { view: pendingView, fromPos: camera.position.clone(), fromUp: camera.up.clone(), to, start: performance.now() };
    setSnapGate(true);
    props.onPendingViewClear();
    invalidate();
  }, [camera, controls, invalidate, props.pendingView, props.onPendingViewClear, setSnapGate]);

  useFrame(() => {
    const anim = animRef.current;
    if (!anim || !camera || !controls) {
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
    if (camera instanceof ThreePerspectiveCamera || camera instanceof ThreeOrthographicCamera) {
      camera.zoom = anim.to.zoom;
      camera.updateProjectionMatrix();
    }
    if (progress < 1) {
      invalidate();
      return;
    }
    animRef.current = null;
    setSnapGate(false);
    applyWorldCameraState(camera, anim.to, controls);
    props.onCameraChange?.(anim.to);
    props.onViewSnap?.(anim.view, anim.to);
    if (anim.to.projection) {
      props.onProjectionChange?.(anim.to.projection);
    }
    invalidate();
  });
  return null;
}

export interface WorldOrbitViewControlsProps {
  readonly show?: boolean;
  readonly onCameraChange?: (state: WorldCameraState) => void;
  readonly onProjectionChange?: (projection: OrbitCameraProjection) => void;
  readonly onViewSnap?: (view: OrbitCameraViewId, state: WorldCameraState) => void;
}

/** @emoji 🧭 Bundles {@link WorldOrbitViewGizmo} and {@link WorldOrbitViewSnapDriver}. */
export function WorldOrbitViewControls(props: WorldOrbitViewControlsProps): ReactElement {
  const [pendingView, setPendingView] = reactHostPort.useState<OrbitCameraViewId | null>(null);
  return (
    <>
      <WorldOrbitViewSnapDriver pendingView={pendingView} onPendingViewClear={() => setPendingView(null)} onCameraChange={props.onCameraChange} onProjectionChange={props.onProjectionChange} onViewSnap={props.onViewSnap} />
      <WorldOrbitViewGizmo show={props.show} onViewSelect={setPendingView} />
    </>
  );
}

export interface WorldOrbitProjectionSwitchProps {
  readonly projection: OrbitCameraProjection;
  readonly onProjectionChange: (projection: OrbitCameraProjection) => void;
  readonly className?: string;
}

/** @emoji 🔀 Small orthographic / perspective toggle for infinite-world viewports. */
export function WorldOrbitProjectionSwitch(props: WorldOrbitProjectionSwitchProps): ReactElement {
  const shellClass = props.className ?? cn("pointer-events-auto absolute bottom-[4.75rem] right-3 z-10 flex text-2xs font-medium", floatingToolbarSurfaceClass);
  const buttonClass = (active: boolean) => cn("px-2 py-1 transition-colors", active ? "bg-active-base text-emphasized" : cn(menuListItemClassName, "text-muted-foreground"));
  return (
    <div className={shellClass} data-world-projection-switch>
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
    dualRow("view-dual-plan-perspective", "Top | Perspective", "Plan + Perspective", [orbitLayoutPane("top"), orbitLayoutPane("perspective")]),
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
        <orthographicCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={500_000} zoom={props.state.zoom} />
      ) : (
        <perspectiveCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={500_000} fov={props.perspectiveFov ?? 50} zoom={props.state.zoom} />
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
  const { camera } = useThree();
  const controls = useThree((s) => s.controls as OrbitControlsTarget | null);
  const invalidate = useThree((s) => s.invalidate);
  const lastApplyToken = reactHostPort.useRef<string | null>(null);
  const stateRef = reactHostPort.useRef(props.state);
  stateRef.current = props.state;
  const projection = props.state.projection ?? "perspective";
  const controlsReady = controls != null;
  reactHostPort.useLayoutEffect(() => {
    if (!camera) {
      return;
    }
    const token = `${orbitCameraViewRigApplyToken(props.seedKey, projection)}:controls:${controlsReady ? 1 : 0}`;
    if (!shouldApplyOrbitCameraViewRigSeed(lastApplyToken.current, token)) {
      return;
    }
    lastApplyToken.current = token;
    applyWorldCameraState(camera, stateRef.current, controls);
    invalidate();
  }, [camera, controls, controlsReady, invalidate, projection, props.seedKey]);
  return null;
}
// #endregion 📷OrbitCameraView

// #region 🖱️OrbitMouseBindings
/** @emoji 🖱️ Orbit-controls instance with mutable mouse button map. */
export type WorldOrbitControlsBinding = {
  readonly mouseButtons: { LEFT: number | null; MIDDLE: number; RIGHT: number | null };
  readonly enabled?: boolean;
  readonly update?: () => void;
};

/** @emoji 🖱️ Default orbit mouse map: orthographic middle orbit / alt+right pan; perspective middle pan / alt+right orbit. */
export function resolveWorldOrbitMouseButtonsIdle(projection: OrbitCameraProjection = "perspective"): {
  readonly LEFT: number | null;
  readonly MIDDLE: number;
  readonly RIGHT: number | null;
} {
  if (projection === "orthographic") {
    return { LEFT: null, MIDDLE: MOUSE.ROTATE, RIGHT: null };
  }
  return { LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null };
}

/** @emoji 🖱️ Resets orbit mouse buttons to {@link resolveWorldOrbitMouseButtonsIdle}. */
export function applyWorldOrbitMouseButtonsIdle(controls: WorldOrbitControlsBinding, projection: OrbitCameraProjection = "perspective"): void {
  const idle = resolveWorldOrbitMouseButtonsIdle(projection);
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
  const optionsRef = reactHostPort.useRef(options);
  optionsRef.current = options;
  reactHostPort.useLayoutEffect(() => {
    if (controls) {
      applyWorldOrbitMouseButtonsIdle(controls, projection);
    }
  }, [controls, projection]);
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
      controls.mouseButtons.RIGHT = resolveWorldOrbitRightMouseAction(event, optionsRef.current?.projection ?? "perspective");
      controls.update?.();
    };
    const resetRightMouse = () => {
      controls.mouseButtons.RIGHT = resolveWorldOrbitMouseButtonsIdle(optionsRef.current?.projection ?? "perspective").RIGHT;
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

/** @emoji 🎞️ Kicks demand-frameloop renders across mount + orbit/grid setup frames. */
export function DemandFrameloopKick(): null {
  const invalidate = useThree((state) => state.invalidate);
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
  return null;
}

export interface WorldOrbitGatedProps {
  readonly camera?: ThreePerspectiveCamera | null;
  readonly zoom?: number;
  readonly projection?: OrbitCameraProjection;
  readonly onCamera?: (state: WorldCameraState) => void;
  readonly controlsGate?: boolean;
  readonly onCameraNavigate?: (active: boolean) => void;
  readonly controlsKey?: string | number;
  readonly onRightPointerDown?: (event: PointerEvent) => boolean;
}

/** @emoji 🛰️ Canvas-local Three orbit-control binding that never crosses the optional Drei runtime boundary. */
function WorldOrbitControlsBridge({
  camera,
  enabled,
  mouseButtons,
  controlsKey,
  onChange,
  onStart,
  onEnd,
}: {
  readonly camera: Camera;
  readonly enabled: boolean;
  readonly mouseButtons: ReturnType<typeof resolveWorldOrbitMouseButtonsIdle>;
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
    controls.update();
  }, [enabled, mouseButtons]);
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
  const mouseButtonsIdle = reactHostPort.useMemo(() => resolveWorldOrbitMouseButtonsIdle(projection), [projection]);
  useWorldOrbitRightMouseBindings(controls, gl.domElement, { projection, onRightPointerDown: props.onRightPointerDown });
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
  return (
    <WorldOrbitControlsBridge
      camera={camera}
      enabled={!gate && !snapGate}
      mouseButtons={mouseButtonsIdle}
      controlsKey={props.controlsKey}
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
  const pickable = worldEntityInspectable(props.reference);
  const interactionHovered = pickable && (props.hovered || pointerHovered);
  const renderMode = {
    ...worldEntityRenderMode(props.reference, {
      hovered: interactionHovered,
      selected: props.selected,
      revealed: props.revealed || (pickable && pointerHovered),
    }),
    showSelectedOutline: props.selected && pickable,
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
        console.log("[DEBUG] world reference texture loaded", props.reference.id, props.reference.source.url);
      })
      .catch((error) => {
        console.log("[DEBUG] world reference texture failed", props.reference.id, error);
      });
    return () => {
      cancelled = true;
    };
  }, [props.reference.id, props.reference.source.url, props.reference.source.mediaKind, props.reference.source.page]);
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
          raycast={pickable ? undefined : worldRaycastNone}
          onPointerDown={(event) => {
            if (!pickable || event.button !== 0) {
              return;
            }
            event.stopPropagation();
            props.onSelect?.(props.reference.id, { shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
          }}
          onPointerOver={(event) => {
            event.stopPropagation();
            if (!pickable) {
              return;
            }
            setPointerHovered(true);
            props.onHover?.(props.reference.id);
          }}
          onPointerOut={(event) => {
            event.stopPropagation();
            if (!pickable) {
              return;
            }
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
          event.stopPropagation();
          if (selectable) {
            props.onHover?.(props.volume.id);
          }
        }}
        onPointerOut={(event) => {
          event.stopPropagation();
          props.onHover?.(null);
        }}
      >
        <mesh raycast={props.interactive === false ? worldRaycastNone : undefined}>
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
    it("maps middle click to orbit in orthographic and pan in perspective", () => {
      expect(resolveWorldOrbitMouseButtonsIdle("orthographic")).toEqual({ LEFT: null, MIDDLE: MOUSE.ROTATE, RIGHT: null });
      expect(resolveWorldOrbitMouseButtonsIdle("perspective")).toEqual({ LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null });
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

  describe("resolveOrbitCameraViewFromTemplateId", () => {
    it("maps display-tree template ids to orbit views", () => {
      expect(resolveOrbitCameraViewFromTemplateId("top")).toBe("top");
      expect(resolveOrbitCameraViewFromTemplateId("orthographic-2d")).toBe("top");
      expect(resolveOrbitCameraViewFromTemplateId("perspective")).toBe("perspective");
      expect(resolveOrbitCameraViewFromTemplateId("missing")).toBeNull();
    });
  });

  describe("orbitCameraViewRigApplyToken", () => {
    it("keys apply-once behavior by seed and projection, not camera remounts", () => {
      expect(orbitCameraViewRigApplyToken("inst:1", "orthographic")).toBe("inst:1:orthographic");
      expect(orbitCameraViewRigApplyToken("inst:1", "perspective")).toBe("inst:1:perspective");
      expect(orbitCameraViewRigApplyToken(7, "perspective")).toBe("7:perspective");
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

  describe("lodProgressiveGridLayerKey", () => {
    it("is stable across continuous lod drift inside one band", () => {
      const keyA = lodProgressiveGridLayerKey(50, 10);
      const keyB = lodProgressiveGridLayerKey(49.2, 10);
      const keyC = lodProgressiveGridLayerKey(11.4, 10);
      expect(keyA).toBe("100|25");
      expect(keyB).toBe(keyA);
      expect(keyC).toBe(keyA);
      expect(lodProgressiveGridLayerKey(9.8, 10)).toBe("100|25|5");
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
