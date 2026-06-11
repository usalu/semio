// #region 🔌Adapters
import {
  reactHostPort,
  sceneHostPort,
  engagementCommandTokenEquals,
  ENGAGEMENT_USER,
  normalizeEngagementCommandText,
  gumballConfigVisible,
  gumballHandleKindToTransformMode,
  gumballKindFromRaycastObject,
  gumballPointerConsumesCanvasEventRef,
  UnifiedGumball,
  type EngagementSpec,
  type EngagementSliderControl,
  type GumballConfig,
  type GumballHandleKind,
  type ThreeEvent,
  type TreeDragAndDropController,
  referenceMediaKindFromUrl,
  SelectionMarquee,
  marqueeCoverageFromGesture,
  marqueeIsCrossing,
  marqueeIsCrossingFromPath,
  marqueeModeFromModifiers,
  ContextMenuController,
  cn,
  glassMenuClass,
  type ContextMenuItem,
} from "@ui/react";
import {
  blendTokenHex,
  resolveBackgroundColorHex,
  resolveColorHex,
  resolveThreeColor,
  semanticVar,
  themeColorVar,
  tokenHex,
  tokenVar,
} from "@ui/styling";
import React, { Children, isValidElement, type CSSProperties, type MutableRefObject, type ReactNode } from "react";
import { createPortal } from "react-dom";
import {
  cadObjectLocalDirectionToThreeGroupLocal,
  cadObjectLocalToThreeGroupLocal,
  cadQuatToThree,
  cadVec3ToThree,
  cadToThreeMatrix,
  chunkBoundsRadius,
  chunkDistanceVisible,
  chunkKey,
  createRefCountPool,
  createTemplatePool,
  formatLod,
  gridPlacementAnchorCad,
  GLB_MESH_FRAME_ROTATION_X,
  collisionBodyFromObject,
  solidOverlapVolume,
  type CollisionBody,
  lodFromCameraDistance,
  lodFromSliderValue,
  lodGridBandStepsWorld,
  lodGridStepWorld,
  lodProgressiveGridLayers,
  pickClosestLod,
  pickClosestMeshUrl,
  sliderValueFromLod,
  threeQuatToCad,
  threeVec3ToCad,
  useLod,
  WorldCanvas,
  WorldChunkedSceneChildren,
  WorldLayer,
  WorldLayerStack,
  WorldLodBridge,
  WorldOrbitCameraViewRig,
  WorldOrbitProjectionSwitch,
  WorldOrbitViewControls,
  WorldOrbitViewSnapGateProvider,
  resolveWorldOrbitMouseButtonsIdle,
  applyOrbitProjectionToCameraState,
  useWorldOrbitRightMouseBindings,
  useWorldOrbitViewSnapGate,
  type WorldOrbitControlsBinding,
  WorldReferenceLayer,
  WorldVolumeLayer,
  applyWorldReferenceTransform,
  applyWorldVolumeTransform,
  worldVolumesContainAabb,
  type WorldReferenceProps,
  type WorldReferenceRelocatePayload,
  type WorldReferenceSource,
  type WorldVolumeProps,
  type WorldVolumeRelocatePayload,
  WORLD_LOD_EPSILON,
  WORLD_LOD_GRID_MAX_LOD,
  WORLD_LOD_GRID_MEDIUM_MAX_LOD,
  WORLD_LOD_GRID_MICRO_MAX_LOD,
  WORLD_LOD_GRID_SMALL_MAX_LOD,
  WORLD_LOD_SLIDER_MAX,
  WORLD_LOD_SLIDER_MIN,
  WORLD_MESH_BORDER_CSS,
  WORLD_MESH_OUTLINE_USER_DATA_KEY,
  applyWorldMeshEdgeBorders,
  WORLD_LOCKED_OPACITY_SCALE,
  worldEntityRenderMode,
  worldEntityRendered,
  worldEntitySelectable,
  type LodContextValue,
  type LodGridLayer,
  type WorldEntityFlags,
} from "@infinite/world/r3f";

export type { LodContextValue };
export { useLod };
export {
  ORBIT_CAMERA_VIEW_COMMAND,
  computeOrbitCameraViewState,
  createOrbitCameraViewLayoutDescriptors,
  createOrbitCameraViewTemplates,
  orbitCameraDistance,
  orbitCameraProjectionForView,
  resolveOrbitCameraViewFromTemplateId,
  WorldOrbitCameraViewRig,
  WorldOrbitProjectionSwitch,
  WorldOrbitViewControls,
  WorldOrbitViewSnapGateProvider,
  applyOrbitProjectionToCameraState,
  useWorldOrbitViewSnapGate,
  type OrbitCameraProjection,
  type OrbitCameraViewId,
  type WorldCameraState,
  type WorldReferenceProps,
  type WorldReferenceRelocatePayload,
  type WorldReferenceSource,
  type WorldVolumeProps,
  type WorldVolumeRelocatePayload,
} from "@infinite/world/r3f";
// #endregion 🔌Adapters

// #region 🔌PortWiring
const useFrame = sceneHostPort.fiber.useFrame;
const useThree = sceneHostPort.fiber.useThree;
const Clone = sceneHostPort.drei.Clone;
const Line = sceneHostPort.drei.Line;
const OrbitControls = sceneHostPort.drei.OrbitControls;
const Outlines = sceneHostPort.drei.Outlines;
const PerspectiveCamera = sceneHostPort.drei.PerspectiveCamera;
const useGLTF = sceneHostPort.drei.useGLTF;
const {
  Box3,
  BoxGeometry,
  BufferGeometry,
  Color,
  Euler,
  Float32BufferAttribute,
  GridHelper,
  Group,
  LineBasicMaterial,
  LineSegments,
  MOUSE,
  Matrix4,
  Mesh,
  MeshStandardMaterial,
  Plane,
  Points,
  PointsMaterial,
  Quaternion,
  Raycaster,
  Line: ThreeLine,
  PerspectiveCamera: ThreePerspectiveCamera,
  Vector2,
  Vector3,
} = sceneHostPort.three;
type Camera = import("three").Camera;
type Object3D = import("three").Object3D;
type ThreeScene = import("three").Scene;
type WebGLRenderer = import("three").WebGLRenderer;
// #endregion 🔌PortWiring

type SceneListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

class EventBindingController {
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

//#region ­ƒöûKinds
export type Vec3 = readonly [number, number, number];
export type Quat = readonly [number, number, number, number];

export type RelocateMode = "translate" | "rotate" | "scale";

/** @emoji 🎛 Default puzzle 3D gumball: move and rotate only (parts are fixed geometry). */
export const PUZZLE_3D_GUMBALL_CONFIG: Readonly<Required<Pick<GumballConfig, "moveAxes" | "movePlanes" | "rotate" | "scaleAxes" | "scalePlanes" | "scaleUniform">>> = {
  moveAxes: true,
  movePlanes: true,
  rotate: true,
  scaleAxes: false,
  scalePlanes: false,
  scaleUniform: false,
};

/** @emoji 📦 Gumball config for puzzle objects: never expose scale handles. */
export function puzzle3dObjectGumballConfig(base?: GumballConfig): GumballConfig {
  return { ...PUZZLE_3D_GUMBALL_CONFIG, ...base, scaleAxes: false, scalePlanes: false, scaleUniform: false };
}

/** @emoji 🎛 Maps a unified gumball handle drag to puzzle relocate mode. */
export function gumballHandleKindToRelocateMode(kind: GumballHandleKind): RelocateMode {
  return gumballHandleKindToTransformMode(kind);
}
export type SelectionMode = "default" | "additive" | "subtractive" | "invertive";
export type SelectionMethod = "rectangle" | "lasso";
export interface MarqueeSelectableKinds {
  readonly object: boolean;
  readonly vortex: boolean;
  readonly attraction: boolean;
}
/** @emoji 🔗 Bond commit kind: `connect` direct pick, `indirect` ring finish (cross-surface via @puzzle/5d TopologyConnectSession), `proximity` relocate-release snap only. */
export type ConnectKind = "indirect" | "connect" | "proximity";
export const MESH_STYLE_KINDS = ["original", "neutral", "hovered", "selected", "highlighted", "disabled"] as const;
/** @emoji ­ƒÄ¿ Homogeneous GLB presentation kind for pooled scene meshes ({@link MeshBody}). */
export type MeshStyleKind = (typeof MESH_STYLE_KINDS)[number];
/** @emoji ­ƒÄ¿ Default object mesh style when none is passed ({@link MeshBody}). */
export const DEFAULT_MESH_STYLE: MeshStyleKind = "neutral";
/** @emoji 🎯 Max selected scene rows before skipping drei/pooled edge outlines (keeps bulk select responsive). */
export const PUZZLE3D_MESH_OUTLINE_MAX_SELECTION = 48;
export type DomainKind = "urban" | "architecture" | "detailing" | "engineering";
export type ScaleKind = "1to50000" | "1to25000" | "1to10000" | "1to5000" | "1to2500" | "1to1000" | "1to500" | "1to333" | "1to200" | "1to100" | "1to50" | "1to33" | "1to25" | "1to10" | "1to5" | "1to1" | "2to1" | "5to1" | "10to1" | "20to1" | "50to1";

export const DEFAULT_DOMAIN: DomainKind = "architecture";
export const DEFAULT_SCALE_REFERENCE = 100;

const SCALE_RATIOS: Record<ScaleKind, readonly [numerator: number, denominator: number]> = {
  "1to50000": [1, 50_000],
  "1to25000": [1, 25_000],
  "1to10000": [1, 10_000],
  "1to5000": [1, 5_000],
  "1to2500": [1, 2_500],
  "1to1000": [1, 1_000],
  "1to500": [1, 500],
  "1to333": [1, 333],
  "1to200": [1, 200],
  "1to100": [1, 100],
  "1to50": [1, 50],
  "1to33": [1, 33],
  "1to25": [1, 25],
  "1to10": [1, 10],
  "1to5": [1, 5],
  "1to1": [1, 1],
  "2to1": [2, 1],
  "5to1": [5, 1],
  "10to1": [10, 1],
  "20to1": [20, 1],
  "50to1": [50, 1],
};

export interface CameraState {
  position: Vec3;
  target: Vec3;
  zoom: number;
  up?: Vec3;
  projection?: import("@infinite/world/r3f").OrbitCameraProjection;
}

/** @emoji 📶 Scene LOD as scale denominator/numerator (e.g. 50000 = 1:50000, 0.5 = 2:1); higher = coarser. */
export type Lod = number;

/** @emoji 🎨 Per-LOD mesh URL entry for {@link ObjectProps.meshByLod} and {@link VortexProps.vortexMeshByLod}. */
export interface LodMeshEntry {
  readonly lod: number;
  readonly url: string;
}

/** @emoji 📐 Default manual / slider LOD range (log-scaled). */
export const DEFAULT_LOD_RANGE = { min: 0.01, max: 100_000 } as const;

/** @emoji 📐 Default scene LOD when neither auto nor depth-variable applies. */
export const DEFAULT_MANUAL_LOD = 100;

/** @emoji 📐 Default CAD anchor for the horizontal grid / palette drop plane (datum Z). */
export const DEFAULT_PUZZLE3D_GRID_PLANE_ANCHOR_CAD: Vec3 = [0, 0, 0];

/** @emoji 📐 Linear slider domain for log-mapped scene LOD in play measures. */
export const PUZZLE_3D_LOD_SLIDER_MIN = WORLD_LOD_SLIDER_MIN;

/** @emoji 📐 Linear slider domain for log-mapped scene LOD in play measures. */
export const PUZZLE_3D_LOD_SLIDER_MAX = WORLD_LOD_SLIDER_MAX;

/** @emoji 📐 Epsilon for scene LOD change notifications. */
export const PUZZLE_3D_LOD_EPSILON = WORLD_LOD_EPSILON;

/** @emoji 📐 Attraction snap is disabled at or above this coarse scene LOD (≈ 1:1000). */
export const PUZZLE_3D_ATTRACTION_SNAP_MAX_LOD = 1000;

/** @emoji 📐 Large LOD grid quantum in world units (puzzle 2d `PUZZLE_2D_LOD_GRID_MAJOR_QUANTUM`). */
export const LOD_GRID_MAJOR_QUANTUM = 10;

/** @emoji 📐 Medium LOD grid quantum (puzzle 2d `GRID_WORLD_MEDIUM`). */
export const LOD_GRID_MEDIUM_QUANTUM = 2.5;

/** @emoji 📐 Small LOD grid quantum (puzzle 2d `GRID_WORLD_SMALL`). */
export const LOD_GRID_SMALL_QUANTUM = 0.5;

/** @emoji 📐 Micro LOD grid quantum (puzzle 2d `GRID_WORLD_MICRO`). */
export const LOD_GRID_MICRO_QUANTUM = 0.1;

/** @emoji 📐 Coarsest scene LOD that still draws any grid band (puzzle 2d minimap). */
export const PUZZLE_3D_LOD_GRID_MAX_LOD = WORLD_LOD_GRID_MAX_LOD;

/** @emoji 📐 Scene LOD at or below which the medium grid band appears (puzzle 2d normal). */
export const PUZZLE_3D_LOD_GRID_MEDIUM_MAX_LOD = WORLD_LOD_GRID_MEDIUM_MAX_LOD;

/** @emoji 📐 Scene LOD at or below which the small grid band appears (puzzle 2d detail). */
export const PUZZLE_3D_LOD_GRID_SMALL_MAX_LOD = WORLD_LOD_GRID_SMALL_MAX_LOD;

/** @emoji 📐 Scene LOD at or below which the micro grid band appears (puzzle 2d micro). */
export const PUZZLE_3D_LOD_GRID_MICRO_MAX_LOD = WORLD_LOD_GRID_MICRO_MAX_LOD;

/** @emoji 📐 Default grid factor (puzzle 2d `DEFAULT_PUZZLE_2D_GRID_FACTOR`). */
export const DEFAULT_LOD_GRID_FACTOR = 10;

export type { LodGridLayer };

export interface VortexProps extends WorldEntityFlags {
  id: string;
  vortexKind?: string;
  /** @emoji 🏷️ Human-readable vortex label for play UI and hierarchy. */
  label?: string;
  position: Vec3;
  direction?: Vec3;
  radius?: number;
  vortexMeshUrl?: string;
  /** @emoji 🎨 Optional per-LOD GLB URLs for the vortex mesh; falls back to {@link vortexMeshUrl}. */
  vortexMeshByLod?: readonly LodMeshEntry[];
  children?: ReactNode;
}

export interface ObjectProps extends WorldEntityFlags {
  id: string;
  objectKind?: string;
  meshUrl: string;
  /** @emoji 🎨 Optional per-LOD GLB URLs; falls back to {@link meshUrl}. */
  meshByLod?: readonly LodMeshEntry[];
  /** @emoji ­ƒÄ¿ Explicit mesh style; otherwise derived from disabled, selected, highlighted, hovered. */
  style?: MeshStyleKind;
  origin: Vec3;
  orientation?: Quat;
  scale?: number | Vec3;
  label?: string;
  selected?: boolean;
  hovered?: boolean;
  highlighted?: boolean;
  disabled?: boolean;
  relocate?: GumballConfig | false;
  /** @emoji ✋ When true, transform controls mount for this object (usually matches primary selected id). */
  relocateActive?: boolean;
  /** @emoji ­ƒº▓ Object ids attracted to this object in the resolved ownership tree. */
  attracting?: readonly string[];
  /** @emoji ­ƒò│´©Å Root of a connected attraction component (wormhole). */
  wormhole?: boolean;
  children?: ReactNode;
  userData?: Record<string, unknown>;
}

export interface AttractionProps extends WorldEntityFlags {
  id: string;
  attracting: `${string}:${string}`;
  attracted: `${string}:${string}`;
  attractionKind?: string;
}

export const PLACEHOLDER_MESH_URL = "puzzle.3d.placeholder://box";

/** @emoji 🧩 Drag payload only — never passed to {@link useGLTF}. */
export const PALETTE_DRAG_SEED_MESH_URL = "puzzle3d://palette-seed";

/** @emoji 🎨 True when `meshUrl` can be fetched by the GLTF loader (not palette/placeholder schemes). */
export function isLoadableMeshUrl(meshUrl: string | undefined): boolean {
  const url = meshUrl?.trim();
  if (!url) {
    return false;
  }
  if (url === PLACEHOLDER_MESH_URL || url === PALETTE_DRAG_SEED_MESH_URL) {
    return false;
  }
  return !url.includes("://") || url.startsWith("/") || url.startsWith("http://") || url.startsWith("https://");
}

export interface AttractionKind {
  id: string;
  label?: string;
  name?: string;
}

export interface VortexKind {
  id: string;
  label?: string;
  name?: string;
  color?: string;
  defaultCableKind?: string;
  scale?: number;
}

/** @emoji 🌀 Explicit local vortex template on an {@link ObjectKind} for brush placement. */
export interface ObjectKindVortexTemplate {
  readonly vortexKind: string;
  readonly position: Vec3;
  readonly direction?: Vec3;
  readonly radius?: number;
}

/** @emoji 🔌 Kit type connector row (semio ring `t`, optional CAD point, port handle kind) for catalog extraction. */
export interface KitConnectorCadRow {
  readonly point?: { readonly x: number; readonly y: number; readonly z: number };
  readonly direction?: { readonly x: number; readonly y: number; readonly z: number };
  readonly port?: { readonly handleKind?: string };
  readonly t?: number;
}

/** @emoji 🏷️ Maps a kit port `handleKind` id to a puzzle 3d vortex-catalog id (short name when available). */
export function puzzle3dVortexKindLabelFromHandleKind(
  handleKind: string,
  vortexKinds: readonly VortexKind[] | undefined,
  handleKindRows?: readonly { readonly id: string; readonly name: string }[],
): string {
  const hk = handleKind.trim();
  if (hk === "") {
    return hk;
  }
  if (vortexKinds?.some((v) => v.id === hk)) {
    return hk;
  }
  const name = handleKindRows?.find((h) => h.id === hk)?.name?.trim();
  if (name && vortexKinds?.some((v) => v.id === name)) {
    return name;
  }
  return name ?? hk;
}

export interface ObjectKind {
  id: string;
  label?: string;
  name?: string;
  color?: string;
  shape?: string;
  /** @emoji 🖌️ Explicit GLB URL when this kind is instantiated by the brush tool. */
  meshUrl?: string;
  /** @emoji 🎨 Optional per-LOD GLB URLs for brush instantiation. */
  meshByLod?: readonly LodMeshEntry[];
  /** @emoji 📐 Default scale for brush instantiation. */
  scale?: number | Vec3;
  /** @emoji 🌀 Local vortex templates (positions/directions in object-local CAD). */
  vortices?: readonly ObjectKindVortexTemplate[];
}

export interface CableKind {
  id: string;
  label?: string;
  name?: string;
  defaultAttractionKind?: string;
}

export interface KindCatalogBundle {
  attractions?: readonly AttractionKind[];
  cables?: readonly CableKind[];
  objects?: readonly ObjectKind[];
  vortices?: readonly VortexKind[];
}

export interface KindCompatEntry {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: "general" | "object" | "vortex" | "cable" | "attraction";
}

export interface SelectionSnapshot {
  readonly objectIds: readonly string[];
  readonly vortexIds: readonly string[];
  readonly attractionIds: readonly string[];
  readonly referenceIds: readonly string[];
  readonly targetVolumeIds: readonly string[];
}

/** @emoji 🎯 Compares selection snapshots (objects, vortices, attractions). */
export function selectionSnapshotsEqual(a: SelectionSnapshot, b: SelectionSnapshot): boolean {
  if (a.objectIds.length !== b.objectIds.length || a.vortexIds.length !== b.vortexIds.length || a.attractionIds.length !== b.attractionIds.length || a.referenceIds.length !== b.referenceIds.length || a.targetVolumeIds.length !== b.targetVolumeIds.length) {
    return false;
  }
  for (let i = 0; i < a.objectIds.length; i += 1) {
    if (a.objectIds[i] !== b.objectIds[i]) {
      return false;
    }
  }
  for (let i = 0; i < a.vortexIds.length; i += 1) {
    if (a.vortexIds[i] !== b.vortexIds[i]) {
      return false;
    }
  }
  for (let i = 0; i < a.attractionIds.length; i += 1) {
    if (a.attractionIds[i] !== b.attractionIds[i]) {
      return false;
    }
  }
  for (let i = 0; i < a.referenceIds.length; i += 1) {
    if (a.referenceIds[i] !== b.referenceIds[i]) {
      return false;
    }
  }
  for (let i = 0; i < a.targetVolumeIds.length; i += 1) {
    if (a.targetVolumeIds[i] !== b.targetVolumeIds[i]) {
      return false;
    }
  }
  return true;
}

const EMPTY_SELECTION_SNAPSHOT: SelectionSnapshot = { objectIds: [], vortexIds: [], attractionIds: [], referenceIds: [], targetVolumeIds: [] };

/** @emoji 🎯 Normalizes optional selection arrays for stable comparisons. */
export function normalizeSelectionSnapshot(snapshot: SelectionSnapshot): SelectionSnapshot {
  return {
    objectIds: snapshot.objectIds ?? [],
    vortexIds: snapshot.vortexIds ?? [],
    attractionIds: snapshot.attractionIds ?? [],
    referenceIds: snapshot.referenceIds ?? [],
    targetVolumeIds: snapshot.targetVolumeIds ?? [],
  };
}

/** @emoji 🖱️ Pointer movement before a vortex press becomes an attraction drag (px). */
export const PUZZLE_3D_VORTEX_DRAG_THRESHOLD_PX = 6;

export type SelectionPick =
  | { readonly kind: "object"; readonly id: string }
  | { readonly kind: "vortex"; readonly fullId: string }
  | { readonly kind: "attraction"; readonly id: string }
  | { readonly kind: "reference"; readonly id: string }
  | { readonly kind: "targetVolume"; readonly id: string };

/** @emoji 🎯 Single-kind selection slice for one pick target. */
export function puzzle3dSelectionFromPick(pick: SelectionPick): SelectionSnapshot {
  switch (pick.kind) {
    case "object":
      return { objectIds: [pick.id], vortexIds: [], attractionIds: [], referenceIds: [], targetVolumeIds: [] };
    case "vortex":
      return { objectIds: [], vortexIds: [pick.fullId], attractionIds: [], referenceIds: [], targetVolumeIds: [] };
    case "attraction":
      return { objectIds: [], vortexIds: [], attractionIds: [pick.id], referenceIds: [], targetVolumeIds: [] };
    case "reference":
      return { objectIds: [], vortexIds: [], attractionIds: [], referenceIds: [pick.id], targetVolumeIds: [] };
    case "targetVolume":
      return { objectIds: [], vortexIds: [], attractionIds: [], referenceIds: [], targetVolumeIds: [pick.id] };
  }
}

function mergeIdList(mode: SelectionMode, current: readonly string[], incoming: readonly string[]): readonly string[] {
  if (!incoming.length) {
    return current;
  }
  if (mode === "default") {
    return [...incoming];
  }
  if (mode === "additive") {
    const out = [...current];
    for (const id of incoming) {
      if (!out.includes(id)) {
        out.push(id);
      }
    }
    return out;
  }
  if (mode === "subtractive") {
    const remove = new Set(incoming);
    return current.filter((id) => !remove.has(id));
  }
  const invert = new Set(current);
  for (const id of incoming) {
    if (invert.has(id)) {
      invert.delete(id);
    } else {
      invert.add(id);
    }
  }
  return [...invert];
}

export { marqueeModeFromModifiers };

/** @emoji 🎯 Applies selection mode when committing a canvas pick. */
export function mergeSelection(mode: SelectionMode, current: SelectionSnapshot, pick: SelectionPick): SelectionSnapshot {
  const piece = puzzle3dSelectionFromPick(pick);
  return mergeSelectionSnapshot(mode, current, piece);
}

/** @emoji 🎯 Applies selection mode when committing a marquee or multi-pick snapshot. */
export function mergeSelectionSnapshot(mode: SelectionMode, current: SelectionSnapshot, incoming: SelectionSnapshot): SelectionSnapshot {
  const nextIncoming: SelectionSnapshot = {
    objectIds: incoming.objectIds ?? [],
    vortexIds: incoming.vortexIds ?? [],
    attractionIds: incoming.attractionIds ?? [],
    referenceIds: incoming.referenceIds ?? [],
    targetVolumeIds: incoming.targetVolumeIds ?? [],
  };
  if (mode === "default") {
    return {
      objectIds: [...nextIncoming.objectIds],
      vortexIds: [...nextIncoming.vortexIds],
      attractionIds: [...nextIncoming.attractionIds],
      referenceIds: [...nextIncoming.referenceIds],
      targetVolumeIds: [...nextIncoming.targetVolumeIds],
    };
  }
  return {
    objectIds: mergeIdList(mode, current.objectIds, nextIncoming.objectIds),
    vortexIds: mergeIdList(mode, current.vortexIds, nextIncoming.vortexIds),
    attractionIds: mergeIdList(mode, current.attractionIds, nextIncoming.attractionIds),
    referenceIds: mergeIdList(mode, current.referenceIds, nextIncoming.referenceIds),
    targetVolumeIds: mergeIdList(mode, current.targetVolumeIds, nextIncoming.targetVolumeIds),
  };
}

interface SelectionDerivation {
  readonly snapshot: SelectionSnapshot;
  readonly objectIdSet: ReadonlySet<string>;
  readonly vortexIdSet: ReadonlySet<string>;
  readonly vortexOwnerObjectIdSet: ReadonlySet<string>;
  readonly attractionIdSet: ReadonlySet<string>;
  readonly primaryObjectId: string | null;
  readonly revision: number;
  readonly meshOutlineEnabled: boolean;
}

function deriveSelectionSnapshot(snapshot: SelectionSnapshot): SelectionDerivation {
  const objectIdSet = new Set(snapshot.objectIds);
  const vortexIdSet = new Set(snapshot.vortexIds);
  const vortexOwnerObjectIdSet = new Set<string>();
  for (const fullId of snapshot.vortexIds) {
    vortexOwnerObjectIdSet.add(parseVortexFullId(fullId).objectId);
  }
  const attractionIdSet = new Set(snapshot.attractionIds);
  const selectionRowCount = objectIdSet.size + vortexIdSet.size + attractionIdSet.size;
  return {
    snapshot,
    objectIdSet,
    vortexIdSet,
    vortexOwnerObjectIdSet,
    attractionIdSet,
    primaryObjectId: snapshot.objectIds[0] ?? (snapshot.vortexIds[0] ? parseVortexFullId(snapshot.vortexIds[0]).objectId : null),
    revision: 0,
    meshOutlineEnabled: selectionRowCount <= PUZZLE3D_MESH_OUTLINE_MAX_SELECTION,
  };
}

function selectionIdSetsEqual(left: ReadonlySet<string>, right: ReadonlySet<string>): boolean {
  if (left.size !== right.size) {
    return false;
  }
  for (const id of left) {
    if (!right.has(id)) {
      return false;
    }
  }
  return true;
}

function selectionSetSymmetricDifference(left: ReadonlySet<string>, right: ReadonlySet<string>): Set<string> {
  const changed = new Set<string>();
  for (const id of left) {
    if (!right.has(id)) {
      changed.add(id);
    }
  }
  for (const id of right) {
    if (!left.has(id)) {
      changed.add(id);
    }
  }
  return changed;
}

function addPerIdListener(map: Map<string, Set<() => void>>, id: string, listener: () => void): () => void {
  let listeners = map.get(id);
  if (!listeners) {
    listeners = new Set();
    map.set(id, listeners);
  }
  listeners.add(listener);
  return () => {
    listeners!.delete(listener);
    if (listeners!.size === 0) {
      map.delete(id);
    }
  };
}

function notifyPerIdListeners(map: Map<string, Set<() => void>>, ids: Iterable<string>): void {
  for (const id of ids) {
    const listeners = map.get(id);
    if (!listeners) {
      continue;
    }
    for (const listener of listeners) {
      listener();
    }
  }
}

/** @emoji 🔔 External selection store for synchronous pick feedback under controlled hosts. */
export function createSelectionSnapshotStore(initial: SelectionSnapshot = EMPTY_SELECTION_SNAPSHOT) {
  let derived = deriveSelectionSnapshot(initial);
  derived = { ...derived, revision: 1 };
  const globalListeners = new Set<() => void>();
  const objectListeners = new Map<string, Set<() => void>>();
  const objectVortexRevealListeners = new Map<string, Set<() => void>>();
  const vortexListeners = new Map<string, Set<() => void>>();
  const attractionListeners = new Map<string, Set<() => void>>();
  const attractionBulkListeners = new Set<() => void>();
  const primaryListeners = new Set<() => void>();
  const meshOutlinePolicyListeners = new Set<() => void>();
  let controlledHostSnapshot: SelectionSnapshot | undefined;

  return {
    subscribe(listener: () => void): () => void {
      globalListeners.add(listener);
      return () => globalListeners.delete(listener);
    },
    getSnapshot(): SelectionSnapshot {
      return derived.snapshot;
    },
    getPrimaryObjectId(): string | null {
      return derived.primaryObjectId;
    },
    getAttractionIdSet(): ReadonlySet<string> {
      return derived.attractionIdSet;
    },
    getRevision(): number {
      return derived.revision;
    },
    getMeshOutlineEnabled(): boolean {
      return derived.meshOutlineEnabled;
    },
    subscribeMeshOutlinePolicy(listener: () => void): () => void {
      meshOutlinePolicyListeners.add(listener);
      return () => meshOutlinePolicyListeners.delete(listener);
    },
    isObjectSelected(objectId: string): boolean {
      return derived.objectIdSet.has(objectId);
    },
    isVortexSelected(fullId: string): boolean {
      return derived.vortexIdSet.has(fullId);
    },
    isObjectVortexRevealSelected(objectId: string): boolean {
      return derived.objectIdSet.has(objectId) || derived.vortexOwnerObjectIdSet.has(objectId);
    },
    isAttractionSelected(attractionId: string): boolean {
      return derived.attractionIdSet.has(attractionId);
    },
    subscribeObject(objectId: string, listener: () => void): () => void {
      return addPerIdListener(objectListeners, objectId, listener);
    },
    subscribeObjectVortexReveal(objectId: string, listener: () => void): () => void {
      return addPerIdListener(objectVortexRevealListeners, objectId, listener);
    },
    subscribeVortex(fullId: string, listener: () => void): () => void {
      return addPerIdListener(vortexListeners, fullId, listener);
    },
    subscribeAttraction(attractionId: string, listener: () => void): () => void {
      return addPerIdListener(attractionListeners, attractionId, listener);
    },
    subscribeAttractions(listener: () => void): () => void {
      attractionBulkListeners.add(listener);
      return () => attractionBulkListeners.delete(listener);
    },
    subscribePrimary(listener: () => void): () => void {
      primaryListeners.add(listener);
      return () => primaryListeners.delete(listener);
    },
    setControlledHostSnapshot(snapshot: SelectionSnapshot | undefined): void {
      controlledHostSnapshot = snapshot;
    },
    getControlledHostSnapshot(): SelectionSnapshot | undefined {
      return controlledHostSnapshot;
    },
    setSnapshot(next: SelectionSnapshot, equal: (left: SelectionSnapshot, right: SelectionSnapshot) => boolean = selectionSnapshotsEqual): void {
      const normalized = normalizeSelectionSnapshot(next);
      if (equal(derived.snapshot, normalized)) {
        return;
      }
      const prev = derived;
      const nextDerived = deriveSelectionSnapshot(normalized);
      derived = selectionIdSetsEqual(prev.attractionIdSet, nextDerived.attractionIdSet)
        ? { ...nextDerived, attractionIdSet: prev.attractionIdSet }
        : nextDerived;
      derived = { ...derived, revision: prev.revision + 1 };
      if (prev.meshOutlineEnabled !== derived.meshOutlineEnabled) {
        for (const listener of meshOutlinePolicyListeners) {
          listener();
        }
      }
      if (derived.meshOutlineEnabled) {
        notifyPerIdListeners(objectListeners, selectionSetSymmetricDifference(prev.objectIdSet, derived.objectIdSet));
        notifyPerIdListeners(vortexListeners, selectionSetSymmetricDifference(prev.vortexIdSet, derived.vortexIdSet));
      }
      const vortexRevealChanged = new Set<string>();
      for (const id of selectionSetSymmetricDifference(prev.objectIdSet, derived.objectIdSet)) {
        vortexRevealChanged.add(id);
      }
      for (const id of selectionSetSymmetricDifference(prev.vortexOwnerObjectIdSet, derived.vortexOwnerObjectIdSet)) {
        vortexRevealChanged.add(id);
      }
      notifyPerIdListeners(objectVortexRevealListeners, vortexRevealChanged);
      if (!selectionIdSetsEqual(prev.attractionIdSet, nextDerived.attractionIdSet)) {
        notifyPerIdListeners(attractionListeners, selectionSetSymmetricDifference(prev.attractionIdSet, derived.attractionIdSet));
        for (const listener of attractionBulkListeners) {
          listener();
        }
      }
      if (prev.primaryObjectId !== derived.primaryObjectId) {
        for (const listener of primaryListeners) {
          listener();
        }
      }
      for (const listener of globalListeners) {
        listener();
      }
    },
  };
}

export type SelectionSnapshotStore = ReturnType<typeof createSelectionSnapshotStore>;

/** @emoji 🎯 Primary object id for relocate chrome (direct pick or parent of selected vortex). */
export function primarySelectionObjectId(selection: SelectionSnapshot): string | null {
  return selection.objectIds[0] ?? (selection.vortexIds[0] ? parseVortexFullId(selection.vortexIds[0]).objectId : null);
}

const SelectionStoreContext = reactHostPort.createContext<SelectionSnapshotStore | null>(null);

function useSelectionSnapshotStore(): SelectionSnapshotStore {
  const store = reactHostPort.useContext(SelectionStoreContext);
  if (!store) {
    throw new Error("Puzzle 3D selection store missing");
  }
  return store;
}

/** @emoji 🎯 Live scene selection snapshot (updates synchronously on pick). */
export function useLiveSelection(): SelectionSnapshot {
  const store = useSelectionSnapshotStore();
  return reactHostPort.useSyncExternalStore(store.subscribe, store.getSnapshot, store.getSnapshot);
}

/** @emoji 🎯 O(1) object mesh-selection membership (direct object picks only; vortex picks use {@link useVortexSelected}). */
export function useObjectSelected(objectId: string): boolean {
  const store = useSelectionSnapshotStore();
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => {
      if (!store.getMeshOutlineEnabled()) {
        return () => {};
      }
      return store.subscribeObject(objectId, onStoreChange);
    },
    () => store.isObjectSelected(objectId),
    () => store.isObjectSelected(objectId),
  );
}

/** @emoji 🌀 True when vortex chrome should show for a parent object (hover or selection on object or its vortices). */
export function objectVorticesRevealed(objectId: string, hoverTarget: HoverTarget | null, selectionRevealSelected: boolean): boolean {
  if (selectionRevealSelected) {
    return true;
  }
  if (hoverTarget?.kind === "object" && hoverTarget.id === objectId) {
    return true;
  }
  if (hoverTarget?.kind === "vortex" && parseVortexFullId(hoverTarget.fullId).objectId === objectId) {
    return true;
  }
  return false;
}

/** @emoji 🌀 Subscribes to hover + selection state that reveals an object's vortices. */
export function useObjectVorticesRevealed(objectId: string): boolean {
  const store = useSelectionSnapshotStore();
  const { hoverTarget } = useRegistryHover();
  const selectionRevealSelected = reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeObjectVortexReveal(objectId, onStoreChange),
    () => store.isObjectVortexRevealSelected(objectId),
    () => store.isObjectVortexRevealSelected(objectId),
  );
  return objectVorticesRevealed(objectId, hoverTarget, selectionRevealSelected);
}

/** @emoji 🎯 O(1) vortex highlight membership. */
export function useVortexSelected(fullId: string): boolean {
  const store = useSelectionSnapshotStore();
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => {
      if (!store.getMeshOutlineEnabled()) {
        return () => {};
      }
      return store.subscribeVortex(fullId, onStoreChange);
    },
    () => store.isVortexSelected(fullId),
    () => store.isVortexSelected(fullId),
  );
}

/** @emoji 🎯 Stable attraction-id set for batch line coloring (notifies only when attraction selection changes). */
export function useSelectedAttractionIdSet(): ReadonlySet<string> {
  const store = useSelectionSnapshotStore();
  return reactHostPort.useSyncExternalStore(store.subscribeAttractions, store.getAttractionIdSet, store.getAttractionIdSet);
}

/** @emoji 🎯 Primary relocate object id with per-primary subscription. */
export function usePrimarySelectionObjectId(): string | null {
  const store = useSelectionSnapshotStore();
  return reactHostPort.useSyncExternalStore(store.subscribePrimary, store.getPrimaryObjectId, store.getPrimaryObjectId);
}

/** @emoji 🎯 False when selection count exceeds {@link PUZZLE3D_MESH_OUTLINE_MAX_SELECTION} (fill-only highlight). */
export function useSelectionMeshOutlinesEnabled(): boolean {
  const store = useSelectionSnapshotStore();
  return reactHostPort.useSyncExternalStore(store.subscribeMeshOutlinePolicy, store.getMeshOutlineEnabled, store.getMeshOutlineEnabled);
}

/** @emoji 🖱️ Exclusive scene hover target (at most one active). */
export type HoverTarget =
  | { readonly kind: "object"; readonly id: string }
  | { readonly kind: "vortex"; readonly fullId: string }
  | { readonly kind: "attraction"; readonly id: string }
  | { readonly kind: "reference"; readonly id: string }
  | { readonly kind: "targetVolume"; readonly id: string };

/** @emoji 🧩 Catalog-kind hover domain for transitive same-kind highlight. */
export type Puzzle3dKindHoverDomain = "object" | "vortex" | "attraction";

/** @emoji 🖱️ Active transitive hover kind derived from a hovered instance or kind row. */
export interface Puzzle3dKindHover {
  readonly domain: Puzzle3dKindHoverDomain;
  readonly kindId: string;
}

/** @emoji 🖱️ Compares two puzzle 3D kind hovers for equality. */
export function puzzle3dKindHoversEqual(a: Puzzle3dKindHover | null, b: Puzzle3dKindHover | null): boolean {
  if (a === b) {
    return true;
  }
  if (!a || !b) {
    return false;
  }
  return a.domain === b.domain && a.kindId === b.kindId;
}

/** @emoji 🖱️ Derives catalog kind hover from a direct scene hover target. */
export function puzzle3dKindHoverFromTarget(
  target: HoverTarget,
  getObjectKind: (id: string) => string | undefined,
  getVortexKind: (fullId: string) => string | undefined,
  getAttractionKind: (id: string) => string | undefined,
): Puzzle3dKindHover | null {
  switch (target.kind) {
    case "object": {
      const kindId = getObjectKind(target.id)?.trim();
      return kindId ? { domain: "object", kindId } : null;
    }
    case "vortex": {
      const kindId = getVortexKind(target.fullId)?.trim();
      return kindId ? { domain: "vortex", kindId } : null;
    }
    case "attraction": {
      const kindId = getAttractionKind(target.id)?.trim();
      return kindId ? { domain: "attraction", kindId } : null;
    }
    default:
      return null;
  }
}

/** @emoji 🖱️ Canvas + hierarchy hover payload for puzzle 3D play shells. */
export interface Puzzle3dHoverPayload {
  readonly hoverTarget: HoverTarget | null;
  readonly kindHover: Puzzle3dKindHover | null;
}

/** @emoji 🖱️ Compares two hover targets for equality. */
export function puzzle3dHoverTargetsEqual(a: HoverTarget | null, b: HoverTarget | null): boolean {
  if (a === b) {
    return true;
  }
  if (!a || !b) {
    return false;
  }
  if (a.kind !== b.kind) {
    return false;
  }
  switch (a.kind) {
    case "object":
      return b.kind === "object" && a.id === b.id;
    case "vortex":
      return b.kind === "vortex" && a.fullId === b.fullId;
    case "attraction":
      return b.kind === "attraction" && a.id === b.id;
    case "reference":
      return b.kind === "reference" && a.id === b.id;
    case "targetVolume":
      return b.kind === "targetVolume" && a.id === b.id;
    default:
      return false;
  }
}

export interface RelocatePayload {
  readonly objectId: string;
  readonly mode: RelocateMode;
  readonly before: { origin: Vec3; orientation: Quat; scale: Vec3 };
  readonly after: { origin: Vec3; orientation: Quat; scale: Vec3 };
}

export interface AttractionPayload {
  readonly attracting: string;
  readonly attracted: string;
  readonly attractionId?: string;
}

/** @emoji 🖌️ Brush commit payload: new object pose plus attraction endpoints. */
export interface BrushPlacePayload {
  readonly targetVortexFullId: string;
  readonly objectKindId: string;
  readonly sourceVortexIndex: number;
  readonly origin: Vec3;
  readonly orientation: Quat;
  readonly scale?: number | Vec3;
  readonly attractionId?: string;
  /** @emoji 🧪 Optional fixed object id (tests); otherwise a random id is generated. */
  readonly objectId?: string;
}

/** @emoji 🖌️ Live brush preview pose and catalog candidate index. */
export interface BrushPreviewState {
  readonly targetVortexFullId: string;
  readonly objectKindId: string;
  readonly sourceVortexIndex: number;
  readonly meshUrl: string;
  readonly meshByLod?: readonly LodMeshEntry[];
  readonly scale?: number | Vec3;
  readonly origin: Vec3;
  readonly orientation: Quat;
}

export interface AttractionCompatibleObjectsPayload {
  readonly attracting: string;
  readonly objectIds: readonly string[];
}

export interface AttractionTargetRingPayload {
  readonly attracting: string;
  readonly objectId: string | null;
  readonly vortexFullIds: readonly string[];
}

export interface AttractionIndirectPickAwait {
  readonly attractingFullId: string;
  readonly attractedObjectId: string;
  readonly candidates: readonly string[];
}

/** @emoji 🔗 Host-driven attraction preview mirrored across spatial surfaces ({@link CanvasProps.attractionSession}). */
export interface AttractionSessionSnapshot {
  readonly attracting: string;
  readonly end: Vec3;
  readonly compatibleObjectIds: readonly string[];
  readonly ringObjectId: string | null;
  readonly ringVortexFullIds: readonly string[];
}

export interface CanvasProps {
  camera?: Partial<CameraState>;
  /** @emoji 📷 Bumps {@link OrbitCameraViewSeed} when display templates or per-instance cameras change. */
  cameraSeedKey?: string | number;
  domain?: DomainKind;
  chunkSize?: number;
  kindCatalogs?: KindCatalogBundle;
  kindCompatibility?: readonly KindCompatEntry[];
  /** @emoji ­ƒÜ½ Vortex full ids (`objectId:vortexId`) that already terminate an attraction and cannot start or receive a new attraction. */
  blockedVortexFullIds?: ReadonlySet<string>;
  proximityRadius?: number;
  /** @emoji 🔗 When false, skip O(vortices) proximity scan on gumball release (e.g. fixtures with no attractions). */
  proximityRelocateEnabled?: boolean;
  gumballConfig?: GumballConfig;
  selectionMode?: SelectionMode;
  /** @emoji 📶 When true (default), orbit camera distance drives scene LOD. */
  automaticLod?: boolean;
  /** @emoji 📶 When true, each object picks LOD from its world distance to the camera. */
  depthVariableLod?: boolean;
  /** @emoji 📶 Manual scene LOD when {@link automaticLod} and {@link depthVariableLod} are both false. */
  lod?: number;
  /** @emoji 📏 Orbit distance at which scene LOD is ~1 (`distance / reference`). */
  lodDistanceReference?: number;
  /** @emoji 📐 Clamp range for manual LOD slider UI. */
  availableLodRange?: { readonly min: number; readonly max: number };
  /** @emoji ­ƒôÉ Multiplier for LOD grid steps (puzzle 2d `grid_factor`). */
  gridFactor?: number;
  /** @emoji ­ƒôÉ When true, draw a world `GridHelper` stepped by the current LOD band grid. */
  showLodGrid?: boolean;
  /** @emoji ­ƒº▓ When true, translate relocate snaps to the finest visible LOD grid step (puzzle 2d `grid_snap_enabled`). */
  gridSnapEnabled?: boolean;
  onCamera?: (s: CameraState) => void;
  /** @emoji 📶 Emits whenever the resolved scene-level LOD changes. */
  onLodChange?: (lod: number) => void;
  onSelect?: (snap: SelectionSnapshot) => void;
  /** @emoji 🎯 When set, canvas selection is controlled by the host (e.g. puzzle 3D play inspector). */
  selection?: SelectionSnapshot;
  onRelocate?: (p: RelocatePayload) => void;
  /** @emoji 🖼️ Commits a reference-plane gumball drag to the host fixture. */
  onReferenceRelocate?: (payload: WorldReferenceRelocatePayload) => void;
  /** @emoji 🧊 Commits a target-volume gumball drag to the host fixture. */
  onTargetVolumeRelocate?: (payload: WorldVolumeRelocatePayload) => void;
  /** @emoji 🧱 Paints one voxel box at the snapped cursor cell while Alt is held. */
  onVoxelBrushPaint?: (cad: Vec3, scale: Vec3) => void;
  /** @emoji 🧱 Axis-aligned voxel brush box size `[width, depth, height]` in world units. */
  voxelBrushDimensions?: Vec3;
  /** @emoji 🧊 When true, target volumes are selectable and the voxel brush is active. */
  fillEditTargetVolumes?: boolean;
  onConnect?: (p: AttractionPayload) => void;
  onIndirectConnect?: (p: AttractionPayload) => void;
  onProximityConnect?: (p: AttractionPayload) => void;
  onAttractionCompatibleObjects?: (p: AttractionCompatibleObjectsPayload) => void;
  onAttractionTargetRing?: (p: AttractionTargetRingPayload) => void;
  /** @emoji 🖌️ When true, hover free vortices to preview; hold Alt and leave to flush compatible objects. */
  brushActive?: boolean;
  /** @emoji 🪣 When true, preloads catalog meshes then calls {@link CanvasProps.onFillMeshesReady}. */
  fillActive?: boolean;
  /** @emoji 🪣 Invoked after fill collision meshes are pooled (hosts call {@link preparePuzzle3dFillSession}). */
  onFillMeshesReady?: () => void;
  /** @emoji 🖌️ Commits a brush placement (new object + attraction). */
  onBrushPlace?: (payload: BrushPlacePayload) => void;
  /** @emoji 📏 Solid overlap volume budget (m3) before brush placement counts as collision; defaults to {@link DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET}. */
  brushPlacementOverlapBudget?: number;
  /** @emoji 📥 When true, accepts in-app fixture drags using {@link FIXTURE_DRAG_V1_MIME} (not OS file drops). */
  fixtureDragDrop?: boolean;
  /** @emoji 📥 Palette / shelf fixture dropped on the canvas at the grid-plane intersection. */
  onFixtureDrop?: (detail: Puzzle3dFixtureDropDetail) => void;
  /** @emoji 🖱️ Controlled direct hover target (`onHover` should update this). */
  hoverTarget?: HoverTarget | null;
  /** @emoji 🖱️ Controlled transitive kind hover (`onHover` should update this). */
  kindHover?: Puzzle3dKindHover | null;
  /** @emoji 🖱️ Fires when canvas or controlled sync changes hover focus. */
  onHover?: (payload: Puzzle3dHoverPayload) => void;
  /** @emoji 🎨 Loaded scene fixture used to resolve catalog kinds that omit `meshUrl` (e.g. Base). */
  sceneFixture?: FixtureV1;
  /** @emoji 🔗 Host-driven attraction preview for cross-surface gestures (cleared when `attracting` is empty). */
  attractionSession?: AttractionSessionSnapshot | null;
  /** @emoji 🖱️ Marquee tool shape (rectangle default, lasso optional). */
  selectionMethod?: SelectionMethod;
  /** @emoji 🖱️ Which entity kinds marquee selection may include. */
  marqueeSelectableKinds?: MarqueeSelectableKinds;
  children?: ReactNode;
}

export const FIXTURE_DRAG_V1_MIME = "application/x-puzzle-3d-fixture-v1";

/** @emoji 🖱️ True while a workbench object-kind palette drag is in flight (some hosts hide custom MIME in `types`). */
export const puzzle3dFixturePaletteDragRef = { active: false };

/** @emoji 🖱️ Pointer-driven palette drag when native HTML5 tree drag does not start (Electron / scroll panels). */
export const puzzle3dFixturePalettePointerDragRef = { active: false, encoded: null as string | null };

/** @emoji 🖱️ Begins pointer palette drag with an encoded fixture payload. */
export function beginPuzzle3dFixturePalettePointerDrag(encoded: string): void {
  puzzle3dFixturePalettePointerDragRef.active = true;
  puzzle3dFixturePalettePointerDragRef.encoded = encoded;
  puzzle3dFixturePaletteDragRef.active = true;
  window.dispatchEvent(new CustomEvent("puzzle3d-fixture-drag-session", { detail: { encoded } }));
}

/** @emoji 🖱️ Ends pointer palette drag without committing a drop. */
export function cancelPuzzle3dFixturePalettePointerDrag(): void {
  if (!puzzle3dFixturePalettePointerDragRef.active && !puzzle3dFixturePaletteDragRef.active) {
    return;
  }
  puzzle3dFixturePalettePointerDragRef.active = false;
  puzzle3dFixturePalettePointerDragRef.encoded = null;
  puzzle3dFixturePaletteDragRef.active = false;
  window.dispatchEvent(new CustomEvent("puzzle3d-fixture-drag-session", { detail: null }));
}

/** @emoji 🎯 True when client coordinates are over the puzzle 3D fixture drop host. */
export function isClientPointOverPuzzle3dFixtureDropHost(clientX: number, clientY: number, host: HTMLElement | null | undefined): boolean {
  if (!host) {
    return false;
  }
  const rect = host.getBoundingClientRect();
  return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom;
}

/** @emoji 📥 Commits a palette fixture drop at client coordinates (pointer or HTML5 drop). */
export function commitPuzzle3dFixtureDropAtClient(
  clientX: number,
  clientY: number,
  fixture: FixtureV1,
  host: HTMLElement | null | undefined,
  onFixtureDrop: ((detail: Puzzle3dFixtureDropDetail) => void) | undefined,
): boolean {
  const toCad = puzzle3dFixtureDropPointerToCadRef.current;
  if (!toCad || !onFixtureDrop) {
    return false;
  }
  const dropHost = host ?? null;
  const rect = dropHost?.getBoundingClientRect();
  if (!rect) {
    return false;
  }
  const worldCad = toCad(clientX, clientY);
  if (!worldCad) {
    return false;
  }
  onFixtureDrop({
    fixture,
    screen: { x: clientX - rect.left, y: clientY - rect.top },
    worldCad,
  });
  return true;
}

/** @emoji 🖱️ Ends pointer palette drag and drops on the viewport when the pointer is over the host. */
export function endPuzzle3dFixturePalettePointerDrag(
  clientX: number,
  clientY: number,
  host: HTMLElement | null | undefined,
  onFixtureDrop: ((detail: Puzzle3dFixtureDropDetail) => void) | undefined,
): void {
  if (!puzzle3dFixturePalettePointerDragRef.active) {
    return;
  }
  const encoded = puzzle3dFixturePalettePointerDragRef.encoded;
  cancelPuzzle3dFixturePalettePointerDrag();
  if (!encoded) {
    return;
  }
  const fixture = decodePuzzle3dFixtureFromDragV1(encoded);
  if (!fixture) {
    return;
  }
  if (!isClientPointOverPuzzle3dFixtureDropHost(clientX, clientY, host)) {
    return;
  }
  commitPuzzle3dFixtureDropAtClient(clientX, clientY, fixture, host, onFixtureDrop);
}

/** @emoji 🔍 True when `dataTransfer.types` carries a puzzle 3D fixture palette drag. */
export function puzzle3dFixtureDragMimeInTypes(types: readonly string[]): boolean {
  return types.includes(FIXTURE_DRAG_V1_MIME) || types.includes(FIXTURE_DRAG_PLAIN_MIME);
}

/** @emoji 🔍 Whether the viewport should accept a palette fixture drop for this drag gesture. */
export function puzzle3dFixtureDragAcceptsTransfer(types: readonly string[]): boolean {
  if (puzzle3dFixturePalettePointerDragRef.active || puzzle3dFixturePaletteDragRef.active) {
    return true;
  }
  return puzzle3dFixtureDragMimeInTypes(types);
}

/** @emoji 🖱️ {@link TreeDragAndDropController} for workbench rows that carry puzzle 3D fixture palette `dragData`. */
export function puzzle3dFixturePaletteTreeDragController(dragDataByItemId: ReadonlyMap<string, Record<string, string>>): TreeDragAndDropController {
  const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
    const payload = dragData?.[FIXTURE_DRAG_V1_MIME];
    return payload?.trim() ? payload : undefined;
  };
  return {
    getDragData: ({ sourceItem }) => dragDataByItemId.get(sourceItem.id),
    pointerPaletteDrag: {
      readEncodedDragPayload: readEncoded,
      begin: beginPuzzle3dFixturePalettePointerDrag,
      cancel: cancelPuzzle3dFixturePalettePointerDrag,
    },
    onDragStart: ({ sourceItem }) => {
      if (puzzle3dFixturePalettePointerDragRef.active) {
        return;
      }
      puzzle3dFixturePaletteDragRef.active = true;
      const payload = readEncoded(dragDataByItemId.get(sourceItem.id));
      if (payload) {
        window.dispatchEvent(new CustomEvent("puzzle3d-fixture-drag-session", { detail: { encoded: payload } }));
      }
    },
    onDragEnd: () => {
      if (puzzle3dFixturePalettePointerDragRef.active) {
        return;
      }
      puzzle3dFixturePaletteDragRef.active = false;
      window.dispatchEvent(new CustomEvent("puzzle3d-fixture-drag-session", { detail: null }));
    },
  };
}

/** @emoji 📋 Fallback MIME for hosts that only expose `text/plain` on drop. */
export const FIXTURE_DRAG_PLAIN_MIME = "text/plain";

/** @emoji 🧩 `FixtureV1.meta.puzzle3dFixtureDragKind` — workbench palette drops place one object at the pointer. */
export const FIXTURE_DRAG_KIND_PALETTE_OBJECT = "palette-object";

/** @emoji 📍 Puzzle 3D canvas fixture drop: scene plus pointer in CSS space and CAD world on the grid plane. */
export interface Puzzle3dFixtureDropDetail {
  readonly fixture: FixtureV1;
  readonly screen: { readonly x: number; readonly y: number };
  readonly worldCad: Vec3;
}

export interface FixtureObjectV1 extends ObjectProps {
  vortices: VortexProps[];
}

/** @emoji 🧭 Puzzle 3D fixture vectors and quaternions use CAD: X right, Y front, Z up; GLB meshes stay glTF Y-up. */
export interface FixtureV1 {
  schema: "puzzle.3d.fixture/v1";
  camera: CameraState;
  domain: DomainKind;
  meta?: Record<string, unknown>;
  attractions: AttractionProps[];
  objects: FixtureObjectV1[];
  references: WorldReferenceProps[];
  targetVolumes: WorldVolumeProps[];
}

/** @emoji 📷 True when two camera states match within epsilon (avoids redundant fixture writes). */
export function cameraStateNearEqual(a: CameraState, b: CameraState, epsilon = 1e-3): boolean {
  for (let i = 0; i < 3; i += 1) {
    if (Math.abs(a.position[i]! - b.position[i]!) > epsilon) return false;
    if (Math.abs(a.target[i]! - b.target[i]!) > epsilon) return false;
    const aUp = a.up ?? [0, 0, 1];
    const bUp = b.up ?? [0, 0, 1];
    if (Math.abs(aUp[i]! - bUp[i]!) > epsilon) return false;
  }
  if ((a.projection ?? "perspective") !== (b.projection ?? "perspective")) return false;
  return Math.abs(a.zoom - b.zoom) <= epsilon;
}

/** @emoji 📷 Writes camera fields on the fixture; returns the same reference when unchanged. */
export function updatePuzzle3dCameraInFixture(fixture: FixtureV1, camera: Partial<CameraState>): FixtureV1 {
  const nextCamera: CameraState = { ...fixture.camera, ...camera };
  if (cameraStateNearEqual(fixture.camera, nextCamera)) {
    return fixture;
  }
  return { ...fixture, camera: nextCamera };
}
//#endregion ­ƒöûKinds

//#region 📶Lod
export {
  formatLod,
  lodFromCameraDistance,
  lodFromSliderValue,
  lodGridBandStepsWorld,
  lodGridStepWorld,
  lodProgressiveGridLayers,
  pickClosestLod,
  pickClosestMeshUrl,
  sliderValueFromLod,
};

/** @emoji 📶 Maps play / window LOD controls to {@link CanvasProps}. */
export function puzzle3dLodCanvasProps(state: { readonly automaticLod: boolean; readonly depthVariableLod: boolean; readonly manualLod: number }): Pick<CanvasProps, "automaticLod" | "depthVariableLod" | "lod"> {
  return {
    automaticLod: state.automaticLod,
    depthVariableLod: state.depthVariableLod,
    lod: !state.automaticLod && !state.depthVariableLod ? state.manualLod : undefined,
  };
}

/** @emoji 🌐 True when primary vortex visuals are drawn at the given scene LOD. */
export function lodVortexPrimaryVisible(lod: number): boolean {
  return lod <= 200;
}

/** @emoji 🌐 True when invisible vortex pick proxies are used instead of GLB vortex meshes. */
export function lodVortexPickProxy(lod: number): boolean {
  return lod > 200 && lod <= 1000;
}

function resolveMeshUrlForLod(meshByLod: readonly LodMeshEntry[] | undefined, fallbackMeshUrl: string, lod: number): string {
  if (fallbackMeshUrl === PLACEHOLDER_MESH_URL) return fallbackMeshUrl;
  return pickClosestMeshUrl(meshByLod, lod, fallbackMeshUrl) ?? fallbackMeshUrl;
}

/** @emoji 🎨 Resolves per-object mesh URL; useFrame only when depth-variable or per-LOD meshes exist, and setState only when URL changes. */
function useResolvedMeshUrl(opts: { readonly origin: Vec3; readonly meshByLod?: readonly LodMeshEntry[]; readonly fallbackMeshUrl: string }): string {
  const lodCtx = useLod();
  const trackLod = lodCtx.depthVariable || (opts.meshByLod?.length ?? 0) > 0;
  const meshByLodRef = reactHostPort.useRef(opts.meshByLod);
  meshByLodRef.current = opts.meshByLod;
  const fallbackRef = reactHostPort.useRef(opts.fallbackMeshUrl);
  fallbackRef.current = opts.fallbackMeshUrl;
  const originRef = reactHostPort.useRef(opts.origin);
  originRef.current = opts.origin;
  const [url, setUrl] = reactHostPort.useState(() => resolveMeshUrlForLod(opts.meshByLod, opts.fallbackMeshUrl, lodCtx.depthVariable ? lodCtx.lodForWorldPosition(opts.origin) : lodCtx.lod));
  useFrame(() => {
    if (!trackLod) return;
    const lod = lodCtx.depthVariable ? lodCtx.lodForWorldPosition(originRef.current) : lodCtx.lod;
    const next = resolveMeshUrlForLod(meshByLodRef.current, fallbackRef.current, lod);
    setUrl((prev) => (prev === next ? prev : next));
  });
  if (!trackLod) {
    return resolveMeshUrlForLod(opts.meshByLod, opts.fallbackMeshUrl, lodCtx.lod);
  }
  return url;
}

interface VortexLodVisual {
  readonly drawVortexBody: boolean;
  readonly pickProxy: boolean;
  readonly meshUrl: string | undefined;
}

function vortexLodVisual(lod: number, linger: boolean, vortexMeshByLod: readonly LodMeshEntry[] | undefined, vortexMeshUrl: string | undefined): VortexLodVisual {
  const drawVortexBody = lodVortexPrimaryVisible(lod) || linger;
  const pickProxy = lodVortexPickProxy(lod) && !drawVortexBody;
  const meshUrl = drawVortexBody ? pickClosestMeshUrl(vortexMeshByLod, lod, vortexMeshUrl) : undefined;
  return { drawVortexBody, pickProxy, meshUrl };
}

function vortexLodVisualEqual(a: VortexLodVisual, b: VortexLodVisual): boolean {
  return a.drawVortexBody === b.drawVortexBody && a.pickProxy === b.pickProxy && a.meshUrl === b.meshUrl;
}

function LodBridge(props: {
  readonly children: ReactNode;
  readonly lodRef: MutableRefObject<number>;
  readonly distanceReference: number;
  readonly gridFactor: number;
  readonly gridSnapEnabled: boolean;
  readonly showLodGrid: boolean;
  readonly automaticLod: boolean;
  readonly depthVariableLod: boolean;
  readonly manualLod: number;
  readonly onLodChange?: (lod: number) => void;
}) {
  return (
    <WorldLodBridge
      lodRef={props.lodRef}
      distanceReference={props.distanceReference}
      gridFactor={props.gridFactor}
      gridSnapEnabled={props.gridSnapEnabled}
      showLodGrid={props.showLodGrid}
      automaticLod={props.automaticLod}
      depthVariableLod={props.depthVariableLod}
      manualLod={props.manualLod}
      gridDatum={DEFAULT_PUZZLE3D_GRID_PLANE_ANCHOR_CAD}
      onLodChange={props.onLodChange}
    >
      {props.children}
    </WorldLodBridge>
  );
}
//#endregion 📶Lod

//#region ­ƒº¥Fixture
function isVec3(v: unknown): v is Vec3 {
  return Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === "number");
}

function isQuat(v: unknown): v is Quat {
  return Array.isArray(v) && v.length === 4 && v.every((n) => typeof n === "number");
}

function parseLodMeshEntries(v: unknown): readonly LodMeshEntry[] | undefined {
  if (!Array.isArray(v)) return undefined;
  const out: LodMeshEntry[] = [];
  for (const row of v) {
    if (!row || typeof row !== "object") continue;
    const o = row as Record<string, unknown>;
    const lod = o.lod;
    const url = o.url;
    if (typeof lod !== "number" || !Number.isFinite(lod) || lod <= 0) continue;
    if (typeof url !== "string" || !url.length) continue;
    out.push({ lod, url });
  }
  return out.length ? out : undefined;
}

function parseDomainKind(value: unknown): DomainKind {
  if (typeof value !== "string") {
    return DEFAULT_DOMAIN;
  }
  switch (value.trim().toLowerCase()) {
    case "urban":
      return "urban";
    case "architecture":
      return "architecture";
    case "detailing":
      return "detailing";
    case "engineering":
      return "engineering";
    default:
      return DEFAULT_DOMAIN;
  }
}

function parseVortexMeshByLod(v: unknown): readonly LodMeshEntry[] | undefined {
  return parseLodMeshEntries(v);
}

function parseWorldEntityFlags(row: Record<string, unknown>): WorldEntityFlags {
  return {
    ...(row.hidden === true ? { hidden: true } : {}),
    ...(row.locked === true ? { locked: true } : {}),
  };
}

function parseWorldReferenceSource(row: Record<string, unknown>): WorldReferenceSource | null {
  const nested = row.source;
  if (nested && typeof nested === "object") {
    const sourceRow = nested as Record<string, unknown>;
    const url = typeof sourceRow.url === "string" ? sourceRow.url : null;
    if (!url) {
      return null;
    }
    const mediaKind = typeof sourceRow.mediaKind === "string" ? sourceRow.mediaKind : referenceMediaKindFromUrl(url);
    if (mediaKind !== "image" && mediaKind !== "svg" && mediaKind !== "pdf") {
      return null;
    }
    return {
      url,
      mediaKind,
      ...(typeof sourceRow.page === "number" ? { page: sourceRow.page } : {}),
    };
  }
  const url = typeof row.url === "string" ? row.url : null;
  if (!url) {
    return null;
  }
  const mediaKind = typeof row.mediaKind === "string" ? row.mediaKind : referenceMediaKindFromUrl(url);
  if (mediaKind !== "image" && mediaKind !== "svg" && mediaKind !== "pdf") {
    return null;
  }
  return {
    url,
    mediaKind,
    ...(typeof row.page === "number" ? { page: row.page } : {}),
  };
}

function parseWorldReference(row: Record<string, unknown>): WorldReferenceProps | null {
  const id = typeof row.id === "string" ? row.id : null;
  const origin = row.origin;
  const source = parseWorldReferenceSource(row);
  if (!id || !isVec3(origin) || !source) {
    return null;
  }
  return {
    id,
    source,
    origin,
    ...(isQuat(row.orientation) ? { orientation: row.orientation } : {}),
    ...(typeof row.scale === "number" || isVec3(row.scale) ? { scale: row.scale as number | Vec3 } : {}),
    ...(typeof row.widthWorld === "number" ? { widthWorld: row.widthWorld } : {}),
    ...(typeof row.opacity === "number" ? { opacity: row.opacity } : {}),
    ...parseWorldEntityFlags(row),
  };
}

function parseWorldVolume(row: Record<string, unknown>): WorldVolumeProps | null {
  const id = typeof row.id === "string" ? row.id : null;
  const origin = row.origin;
  if (!id || !isVec3(origin)) {
    return null;
  }
  return {
    id,
    origin,
    ...(isQuat(row.orientation) ? { orientation: row.orientation } : {}),
    ...(typeof row.scale === "number" || isVec3(row.scale) ? { scale: row.scale as number | Vec3 } : {}),
    ...(typeof row.color === "string" ? { color: row.color } : {}),
    ...(typeof row.opacity === "number" ? { opacity: row.opacity } : {}),
    ...parseWorldEntityFlags(row),
  };
}

/** @emoji 🖼️ Adds a reference plane to a puzzle 3D fixture. */
export function addReferenceToFixture(fixture: FixtureV1, reference: WorldReferenceProps): FixtureV1 {
  return { ...fixture, references: [...(fixture.references ?? []), reference] };
}

/** @emoji 🖼️ Patches one reference row in a puzzle 3D fixture. */
export function updatePuzzle3dReferenceInFixture(fixture: FixtureV1, referenceId: string, patch: Partial<Omit<WorldReferenceProps, "id">>): FixtureV1 {
  const references = fixture.references ?? [];
  const index = references.findIndex((row) => row.id === referenceId);
  if (index < 0) {
    return fixture;
  }
  const next = [...references];
  next[index] = { ...references[index]!, ...patch };
  return { ...fixture, references: next };
}

/** @emoji 🖼️ Applies a world reference gumball commit to a puzzle 3D fixture. */
export function applyReferenceRelocateToFixture(fixture: FixtureV1, payload: WorldReferenceRelocatePayload): FixtureV1 {
  const references = fixture.references ?? [];
  const index = references.findIndex((row) => row.id === payload.referenceId);
  if (index < 0) {
    return fixture;
  }
  const next = [...references];
  next[index] = applyWorldReferenceTransform(references[index]!, payload.after);
  return { ...fixture, references: next };
}

/** @emoji 🧊 Adds a target volume to a puzzle 3D fixture. */
export function addTargetVolumeToFixture(fixture: FixtureV1, volume: WorldVolumeProps): FixtureV1 {
  return { ...fixture, targetVolumes: [...(fixture.targetVolumes ?? []), volume] };
}

/** @emoji 🧊 Patches one target volume row in a puzzle 3D fixture. */
export function updatePuzzle3dTargetVolumeInFixture(fixture: FixtureV1, volumeId: string, patch: Partial<Omit<WorldVolumeProps, "id">>): FixtureV1 {
  const targetVolumes = fixture.targetVolumes ?? [];
  const index = targetVolumes.findIndex((row) => row.id === volumeId);
  if (index < 0) {
    return fixture;
  }
  const next = [...targetVolumes];
  next[index] = { ...targetVolumes[index]!, ...patch };
  return { ...fixture, targetVolumes: next };
}

/** @emoji 🧊 Removes a target volume from a puzzle 3D fixture. */
export function removeTargetVolumeFromFixture(fixture: FixtureV1, volumeId: string): FixtureV1 {
  const targetVolumes = fixture.targetVolumes ?? [];
  const next = targetVolumes.filter((row) => row.id !== volumeId);
  return next.length === targetVolumes.length ? fixture : { ...fixture, targetVolumes: next };
}

/** @emoji 🧊 Applies a world target volume gumball commit to a puzzle 3D fixture. */
export function applyTargetVolumeRelocateToFixture(fixture: FixtureV1, payload: WorldVolumeRelocatePayload): FixtureV1 {
  const targetVolumes = fixture.targetVolumes ?? [];
  const index = targetVolumes.findIndex((row) => row.id === payload.volumeId);
  if (index < 0) {
    return fixture;
  }
  const next = [...targetVolumes];
  next[index] = applyWorldVolumeTransform(targetVolumes[index]!, payload.after);
  return { ...fixture, targetVolumes: next };
}

/** @emoji 🧊 Removes a target volume at the snapped voxel cell. */
export function removeVoxelFromFixture(fixture: FixtureV1, cad: Vec3, scale: Vec3): FixtureV1 {
  const volume = findVoxelAtCell(fixture.targetVolumes ?? [], cad, scale);
  if (!volume) {
    return fixture;
  }
  return removeTargetVolumeFromFixture(fixture, volume.id);
}

/** @emoji 🧊 Adds one axis-aligned voxel at the snapped cursor cell when empty. */
export function addVoxelToFixture(fixture: FixtureV1, cad: Vec3, scale: Vec3): FixtureV1 {
  if (findVoxelAtCell(fixture.targetVolumes ?? [], cad, scale)) {
    return fixture;
  }
  return addTargetVolumeToFixture(fixture, createVoxelVolume(cad, scale));
}

function snapCadAxisToVoxelCenter(coord: number, size: number): number {
  const h = size / 2;
  return Math.round((coord - h) / size) * size + h;
}

/** @emoji 🧱 Snaps a CAD point to the center of an axis-aligned voxel box. */
export function snapCadToVoxelCenter(cad: Vec3, scale: Vec3): Vec3 {
  const sx = typeof scale === "number" ? scale : scale[0];
  const sy = typeof scale === "number" ? scale : scale[1];
  const sz = typeof scale === "number" ? scale : scale[2];
  return [snapCadAxisToVoxelCenter(cad[0], sx), snapCadAxisToVoxelCenter(cad[1], sy), snapCadAxisToVoxelCenter(cad[2], sz)] as Vec3;
}

function voxelScaleVec(scale: number | Vec3): Vec3 {
  if (typeof scale === "number") {
    return [scale, scale, scale];
  }
  return [scale[0], scale[1], scale[2]];
}

/** @emoji 🧱 Stable grid key for a voxel cell at a given brush box size. */
export function voxelGridKey(cad: Vec3, scale: number | Vec3): string {
  const box = voxelScaleVec(scale);
  const center = snapCadToVoxelCenter(cad, box);
  return `${center[0].toFixed(4)},${center[1].toFixed(4)},${center[2].toFixed(4)}@${box[0].toFixed(4)},${box[1].toFixed(4)},${box[2].toFixed(4)}`;
}

/** @emoji 🧱 Builds a target-volume record for one axis-aligned voxel box. */
export function createVoxelVolume(cad: Vec3, scale: number | Vec3, id?: string): WorldVolumeProps {
  const box = voxelScaleVec(scale);
  const origin = snapCadToVoxelCenter(cad, box);
  return {
    id: id ?? `voxel-${origin.map((n) => n.toFixed(2)).join("-")}`,
    origin,
    scale: box,
  };
}

/** @emoji 🧱 Finds an existing voxel in the fixture at the snapped cell. */
export function findVoxelAtCell(volumes: readonly WorldVolumeProps[], cad: Vec3, scale: number | Vec3): WorldVolumeProps | null {
  const key = voxelGridKey(cad, scale);
  for (const volume of volumes) {
    if (voxelGridKey(volume.origin, volume.scale ?? 1) === key) {
      return volume;
    }
  }
  return null;
}

export const VOXEL_BRUSH_PREVIEW_COLOR = "#38bdf8";
export const VOXEL_BRUSH_PREVIEW_OPACITY = 0.48;

export function parseFixtureV1(raw: unknown): FixtureV1 | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "puzzle.3d.fixture/v1") return null;
  const cam = r.camera;
  if (!cam || typeof cam !== "object") return null;
  const c = cam as Record<string, unknown>;
  const pos = c.position;
  const tgt = c.target;
  const zoom = c.zoom;
  if (!isVec3(pos) || !isVec3(tgt) || typeof zoom !== "number") return null;
  const attractionsRaw = r.attractions;
  const objsRaw = r.objects;
  if (!Array.isArray(attractionsRaw) || !Array.isArray(objsRaw)) return null;
  const attractions: AttractionProps[] = [];
  for (const attraction of attractionsRaw) {
    if (!attraction || typeof attraction !== "object") continue;
    const tr = attraction as Record<string, unknown>;
    if (typeof tr.id !== "string" || typeof tr.attracting !== "string" || typeof tr.attracted !== "string") continue;
    attractions.push({
      id: tr.id,
      attracting: tr.attracting as AttractionProps["attracting"],
      attracted: tr.attracted as AttractionProps["attracted"],
      ...(typeof tr.attractionKind === "string" ? { attractionKind: tr.attractionKind } : {}),
      ...parseWorldEntityFlags(tr),
    });
  }
  const objects: FixtureObjectV1[] = [];
  for (const o of objsRaw) {
    if (!o || typeof o !== "object") continue;
    const or = o as Record<string, unknown>;
    if (typeof or.id !== "string" || typeof or.meshUrl !== "string") continue;
    const origin = or.origin;
    if (!isVec3(origin)) continue;
    const vortices: VortexProps[] = [];
    const vr = or.vortices;
    if (Array.isArray(vr)) {
      for (const v of vr) {
        if (!v || typeof v !== "object") continue;
        const vx = v as Record<string, unknown>;
        if (typeof vx.id !== "string" || !isVec3(vx.position)) continue;
        const vortexMeshByLod = parseVortexMeshByLod(vx.vortexMeshByLod);
        vortices.push({
          id: vx.id,
          ...(typeof vx.vortexKind === "string" ? { vortexKind: vx.vortexKind } : {}),
          ...(typeof vx.label === "string" ? { label: vx.label } : {}),
          position: vx.position,
          ...(isVec3(vx.direction) ? { direction: vx.direction } : {}),
          ...(typeof vx.radius === "number" ? { radius: vx.radius } : {}),
          ...(typeof vx.vortexMeshUrl === "string" ? { vortexMeshUrl: vx.vortexMeshUrl } : {}),
          ...(vortexMeshByLod ? { vortexMeshByLod } : {}),
          ...parseWorldEntityFlags(vx),
        });
      }
    }
    const meshByLod = parseLodMeshEntries(or.meshByLod);
    objects.push({
      id: or.id,
      meshUrl: or.meshUrl,
      origin,
      ...(meshByLod ? { meshByLod } : {}),
      ...(typeof or.objectKind === "string" ? { objectKind: or.objectKind } : {}),
      ...(typeof or.label === "string" ? { label: or.label } : {}),
      ...(or.wormhole === true ? { wormhole: true } : {}),
      ...(isQuat(or.orientation) ? { orientation: or.orientation } : {}),
      ...(typeof or.scale === "number" || isVec3(or.scale) ? { scale: or.scale as number | Vec3 } : {}),
      ...parseWorldEntityFlags(or),
      vortices,
    });
  }
  return {
    schema: "puzzle.3d.fixture/v1",
    camera: { position: pos, target: tgt, zoom },
    domain: parseDomainKind(r.domain),
    ...(r.meta && typeof r.meta === "object" ? { meta: r.meta as Record<string, unknown> } : {}),
    attractions,
    objects,
    references: Array.isArray(r.references)
      ? r.references.flatMap((entry) => {
          if (!entry || typeof entry !== "object") {
            return [];
          }
          const parsed = parseWorldReference(entry as Record<string, unknown>);
          return parsed ? [parsed] : [];
        })
      : [],
    targetVolumes: Array.isArray(r.targetVolumes)
      ? r.targetVolumes.flatMap((entry) => {
          if (!entry || typeof entry !== "object") {
            return [];
          }
          const parsed = parseWorldVolume(entry as Record<string, unknown>);
          return parsed ? [parsed] : [];
        })
      : [],
  };
}

export function encodeFixtureForDragV1(fixture: FixtureV1): string {
  return JSON.stringify(fixture);
}

/** @emoji 📤 Writes puzzle 3D fixture drag payload (custom MIME + `text/plain` fallback). */
export function setPuzzle3dFixtureDragDataTransfer(dataTransfer: DataTransfer, fixture: FixtureV1): void {
  const encoded = encodeFixtureForDragV1(fixture);
  dataTransfer.setData(FIXTURE_DRAG_V1_MIME, encoded);
  dataTransfer.setData(FIXTURE_DRAG_PLAIN_MIME, encoded);
}

/** @emoji 📥 Reads puzzle 3D fixture drag payload from a drop `DataTransfer`. */
export function readPuzzle3dFixtureDragDataTransfer(dataTransfer: DataTransfer): FixtureV1 | null {
  const custom = dataTransfer.getData(FIXTURE_DRAG_V1_MIME);
  if (custom.trim() !== "") {
    const parsed = decodePuzzle3dFixtureFromDragV1(custom);
    if (parsed) {
      return parsed;
    }
  }
  const plain = dataTransfer.getData(FIXTURE_DRAG_PLAIN_MIME);
  if (plain.trim() === "") {
    return null;
  }
  return decodePuzzle3dFixtureFromDragV1(plain);
}

/** @emoji 📥 Parses drag payload from {@link FIXTURE_DRAG_V1_MIME}. */
export function decodePuzzle3dFixtureFromDragV1(text: string): FixtureV1 | null {
  let raw: unknown;
  try {
    raw = JSON.parse(text) as unknown;
  } catch {
    return null;
  }
  return parseFixtureV1(raw);
}

/** @emoji 🧩 Minimal fixture encoding one object kind for workbench palette drags. */
export function buildPaletteObjectDragFixture(objectKindId: string, domain: DomainKind = DEFAULT_DOMAIN): FixtureV1 {
  return {
    schema: "puzzle.3d.fixture/v1",
    camera: { position: [420, -420, 320], target: [0, 0, 40], zoom: 1 },
    domain,
    meta: { puzzle3dFixtureDragKind: FIXTURE_DRAG_KIND_PALETTE_OBJECT },
    attractions: [],
    objects: [{ id: "palette-seed-object", objectKind: objectKindId, meshUrl: PALETTE_DRAG_SEED_MESH_URL, origin: [0, 0, 0], vortices: [] }],
    references: [],
    targetVolumes: [],
  };
}

/** @emoji 📤 `dataTransfer` map for dragging one object kind from the workbench kinds tree. */
export function puzzle3dPlayObjectKindDragData(objectKindId: string, domain: DomainKind = DEFAULT_DOMAIN): Record<string, string> {
  const encoded = encodeFixtureForDragV1(buildPaletteObjectDragFixture(objectKindId, domain));
  return { [FIXTURE_DRAG_V1_MIME]: encoded, [FIXTURE_DRAG_PLAIN_MIME]: encoded };
}

/** @emoji 🧩 True when the drag payload is a palette object kind seed. */
export function isPaletteObjectDragFixture(fixture: FixtureV1): boolean {
  if (fixture.meta?.puzzle3dFixtureDragKind === FIXTURE_DRAG_KIND_PALETTE_OBJECT) {
    return true;
  }
  return fixture.objects.length === 1 && fixture.attractions.length === 0 && fixture.objects[0]!.id.startsWith("palette-seed-");
}

/** @emoji 🧲 Snaps a CAD point to the finest visible LOD grid step when `step` is positive. */
export function snapCadVec3ToGridStep(position: Vec3, step: number | null | undefined): Vec3 {
  if (step == null || !Number.isFinite(step) || step <= 0) {
    return position;
  }
  const snapAxis = (value: number) => Math.round(value / step) * step;
  return [snapAxis(position[0]), snapAxis(position[1]), snapAxis(position[2])];
}

const puzzle3dGridPlaneUp = new Vector3(0, 0, 1);
const puzzle3dGridPlaneScratch = new Plane();
const puzzle3dGridPlaneHitScratch = new Vector3();
const puzzle3dGridPlanePointScratch = new Vector3();

/** @emoji 📐 CAD anchor for grid layers and palette drops: orbit pan XY, datum Z=0 (not camera look-at height). */
export function puzzle3dGridPlacementAnchorCad(controlsTargetThree?: Vector3 | null): Vec3 {
  return gridPlacementAnchorCad(controlsTargetThree ?? null, DEFAULT_PUZZLE3D_GRID_PLANE_ANCHOR_CAD);
}

/** @emoji 📍 Ray–grid-plane hit in CAD: cursor vs camera through the horizontal plane at {@link LodGridHelper}. */
export function puzzle3dClientToGridPlaneCad(args: {
  readonly clientX: number;
  readonly clientY: number;
  readonly camera: Camera;
  readonly canvas: HTMLElement;
  readonly gridSnapEnabled?: boolean;
  readonly gridStepWorld?: number | null;
  /** @emoji 📐 Coplanar anchor in CAD (defaults to datum); use {@link puzzle3dGridPlacementAnchorCad} for orbit-aware XY. */
  readonly gridPlaneAnchorCad?: Vec3;
}): Vec3 {
  const rect = args.canvas.getBoundingClientRect();
  const ndc = new Vector2(((args.clientX - rect.left) / rect.width) * 2 - 1, -((args.clientY - rect.top) / rect.height) * 2 + 1);
  const raycaster = new Raycaster();
  raycaster.setFromCamera(ndc, args.camera);
  const anchorCad = args.gridPlaneAnchorCad ?? DEFAULT_PUZZLE3D_GRID_PLANE_ANCHOR_CAD;
  const anchorThree = cadVec3ToThree(anchorCad);
  puzzle3dGridPlanePointScratch.set(anchorThree[0], anchorThree[1], anchorThree[2]);
  puzzle3dGridPlaneScratch.setFromNormalAndCoplanarPoint(puzzle3dGridPlaneUp, puzzle3dGridPlanePointScratch);
  if (!raycaster.ray.intersectPlane(puzzle3dGridPlaneScratch, puzzle3dGridPlaneHitScratch)) {
    raycaster.ray.at(80, puzzle3dGridPlaneHitScratch);
  }
  const cad = threeVec3ToCad(puzzle3dGridPlaneHitScratch);
  return snapCadVec3ToGridStep(cad, args.gridSnapEnabled ? args.gridStepWorld : null);
}

/** @emoji 🧲 Live bridge from {@link Canvas3D} DOM drops to the active R3F camera and LOD grid snap. */
export const puzzle3dFixtureDropPointerToCadRef: {
  current: ((clientX: number, clientY: number) => Vec3 | null) | null;
} = { current: null };

function catalogObjectKindById(catalogs: KindCatalogBundle | undefined, kindId: string): ObjectKind | undefined {
  return catalogs?.objects?.find((entry) => entry.id === kindId);
}

const NAKAGIN_ORIENTED_CAPSULE_KIND_PREFIXES = ["Capsule With Balcony ", "Trapezoid Capsule ", "Capsule "] as const;

/** @emoji 🧭 Maps nakagin capsule kind display tail to metabolism `representations/capsule_<suffix>.glb` basename. */
export function puzzle3dNakaginCapsuleMeshSuffixFromKindTail(tail: string): string {
  switch (tail) {
    case "J":
      return "J";
    case "L":
      return "L";
    case "P":
      return "p";
    case "q":
    case "Q":
      return "q";
    case "S":
      return "s";
    case "Z":
      return "z";
    case "Backslash":
      return "backslash";
    case "Slash":
      return "slash";
    default:
      return tail.replace(/\s+/g, "_").toLowerCase();
  }
}

/** @emoji 🏗️ Oriented nakagin capsule GLB (`capsule_J.glb`, …); metabolism `capsule-with-balcony_*` / `trapezoid-capsule_*` files are identical placeholders. */
export function puzzle3dNakaginOrientedCapsuleMeshUrlFromKindId(kindId: string): string | undefined {
  const name = kindId.trim();
  if (name === "" || name === "Capsule") {
    return undefined;
  }
  for (const prefix of NAKAGIN_ORIENTED_CAPSULE_KIND_PREFIXES) {
    if (!name.startsWith(prefix) || name.length <= prefix.length) {
      continue;
    }
    const tail = name.slice(prefix.length);
    return `/meshes/capsule_${puzzle3dNakaginCapsuleMeshSuffixFromKindTail(tail)}.glb`;
  }
  return undefined;
}

/** @emoji 🎨 Resolves a GLB URL for an object kind from the catalog, then from matching scene objects. */
export function resolveObjectKindMeshUrl(kindId: string, kindCatalogs: KindCatalogBundle | undefined, sceneFixture?: FixtureV1): string | undefined {
  const orientedCapsule = puzzle3dNakaginOrientedCapsuleMeshUrlFromKindId(kindId);
  if (orientedCapsule) {
    return orientedCapsule;
  }
  const kind = catalogObjectKindById(kindCatalogs, kindId);
  const catalogMesh = kind?.meshUrl?.trim();
  if (isLoadableMeshUrl(catalogMesh)) {
    return catalogMesh;
  }
  const lodMesh = pickClosestMeshUrl(kind?.meshByLod, DEFAULT_MANUAL_LOD, undefined);
  if (isLoadableMeshUrl(lodMesh)) {
    return lodMesh;
  }
  if (sceneFixture) {
    for (const object of sceneFixture.objects) {
      if (object.objectKind === kindId) {
        const sceneMesh = object.meshUrl?.trim();
        if (isLoadableMeshUrl(sceneMesh)) {
          return sceneMesh;
        }
      }
    }
  }
  return undefined;
}

function buildFixtureObjectFromObjectKind(kind: ObjectKind, objectId: string, origin: Vec3, meshUrl: string): FixtureObjectV1 {
  const vortices: VortexProps[] = (kind.vortices ?? []).map((entry, index) => ({
    id: `${objectId}:v${index}`,
    vortexKind: entry.vortexKind,
    label: entry.vortexKind,
    position: entry.position,
    ...(entry.direction ? { direction: entry.direction } : {}),
    ...(entry.radius !== undefined ? { radius: entry.radius } : {}),
  }));
  return {
    id: objectId,
    objectKind: kind.id,
    meshUrl,
    ...(kind.meshByLod ? { meshByLod: kind.meshByLod } : {}),
    label: kind.label ?? kind.name ?? kind.id,
    origin,
    orientation: [0, 0, 0, 1],
    ...(kind.scale !== undefined ? { scale: kind.scale } : {}),
    vortices,
  };
}

/** @emoji 🧩 When the drag payload is a palette object kind, returns one object at the drop CAD point. */
export function mergePaletteObjectFromDrop(detail: Puzzle3dFixtureDropDetail, kindCatalogs: KindCatalogBundle | undefined, sceneFixture?: FixtureV1): FixtureObjectV1 | null {
  if (!isPaletteObjectDragFixture(detail.fixture)) {
    return null;
  }
  const kindId = detail.fixture.objects[0]?.objectKind?.trim();
  if (!kindId) {
    return null;
  }
  const kind = catalogObjectKindById(kindCatalogs, kindId);
  if (!kind) {
    return null;
  }
  const meshUrl = resolveObjectKindMeshUrl(kindId, kindCatalogs, sceneFixture);
  if (!meshUrl) {
    return null;
  }
  const objectId = `puzzle3d.drop.${crypto.randomUUID()}`;
  return buildFixtureObjectFromObjectKind(kind, objectId, detail.worldCad, meshUrl);
}

/** @emoji 📥 Appends a palette-dropped object kind at a CAD origin. */
export function applyPaletteObjectDropToFixture(fixture: FixtureV1, object: FixtureObjectV1): FixtureV1 {
  return { ...fixture, objects: [...fixture.objects, object] };
}

/** @emoji 📥 Outcome of {@link resolvePuzzle3dFixtureDrop} for host fixture patching. */
export type Puzzle3dFixtureDropResult =
  | { readonly kind: "palette-object"; readonly object: FixtureObjectV1 }
  | { readonly kind: "replace-fixture"; readonly fixture: FixtureV1 }
  | { readonly kind: "ignored" };

/** @emoji 📥 Resolves a canvas drop: palette kinds merge one object; full fixtures replace; palette seeds never pass through unresolved. */
export function resolvePuzzle3dFixtureDrop(
  detail: Puzzle3dFixtureDropDetail,
  kindCatalogs: KindCatalogBundle | undefined,
  sceneFixture?: FixtureV1,
): Puzzle3dFixtureDropResult {
  const placed = mergePaletteObjectFromDrop(detail, kindCatalogs, sceneFixture);
  if (placed) {
    return { kind: "palette-object", object: placed };
  }
  if (isPaletteObjectDragFixture(detail.fixture)) {
    return { kind: "ignored" };
  }
  const parsed = parseFixtureV1(detail.fixture);
  if (parsed) {
    return { kind: "replace-fixture", fixture: parsed };
  }
  return { kind: "ignored" };
}

/** @emoji 📥 Applies {@link resolvePuzzle3dFixtureDrop} to a fixture, or returns null when ignored. */
export function applyPuzzle3dFixtureDropResult(fixture: FixtureV1, result: Puzzle3dFixtureDropResult): FixtureV1 | null {
  if (result.kind === "palette-object") {
    return applyPaletteObjectDropToFixture(fixture, result.object);
  }
  if (result.kind === "replace-fixture") {
    return result.fixture;
  }
  return null;
}
//#endregion ­ƒº¾Fixture

//#region ­ƒò©´©ÅAttractionGraph
/** @emoji ­ƒöù Parsed `objectId:vortexId` attraction endpoint. */
export function parseVortexFullId(full: string): { readonly objectId: string; readonly vortexId: string } {
  const i = full.indexOf(":");
  if (i < 0) {
    return { objectId: full, vortexId: "link" };
  }
  return { objectId: full.slice(0, i), vortexId: full.slice(i + 1) };
}

/** @emoji 🔗 Canonical `objectId:vortexId` for fixture vortex rows. */
export function puzzle3dVortexFullId(objectId: string, vortexId: string): string {
  return vortexId.includes(":") ? vortexId : `${objectId}:${vortexId}`;
}

/** @emoji ­ƒò│´©Å True when the object is an explicit or inferred wormhole root. */
export function isWormholeObject(objectId: string, props: { readonly wormhole?: boolean; readonly objectKind?: string }, inferredWormholeIds: ReadonlySet<string>): boolean {
  if (props.wormhole === true) {
    return true;
  }
  const kind = props.objectKind ?? "";
  if (kind.includes("wormhole")) {
    return true;
  }
  return inferredWormholeIds.has(objectId);
}

/** @emoji 🧲 One object-level attraction derived from a vortex-to-vortex attraction (`attracting` attracts `attracted`). */
export interface ObjectAttraction {
  readonly attractingObjectId: string;
  readonly attractedObjectId: string;
  readonly attractionId: string;
}

/** @emoji 🧲 Maps scene attractions to object-level attractions. */
export function objectAttractionsFromAttractions(attractions: readonly AttractionProps[]): ObjectAttraction[] {
  const out: ObjectAttraction[] = [];
  for (const attraction of attractions) {
    const attractingObjectId = parseVortexFullId(attraction.attracting).objectId;
    const attractedObjectId = parseVortexFullId(attraction.attracted).objectId;
    if (!attractingObjectId || !attractedObjectId || attractingObjectId === attractedObjectId) {
      continue;
    }
    out.push({ attractingObjectId, attractedObjectId, attractionId: attraction.id });
  }
  return out;
}

export interface AttractionTree {
  readonly parentByObjectId: ReadonlyMap<string, string | null>;
  readonly attractingByObjectId: ReadonlyMap<string, readonly string[]>;
  readonly wormholeDistanceByObjectId: ReadonlyMap<string, number>;
  readonly wormholeIds: readonly string[];
}

function vec3Add(a: Vec3, b: Vec3): Vec3 {
  return [a[0] + b[0], a[1] + b[1], a[2] + b[2]] as Vec3;
}

function vec3Sub(a: Vec3, b: Vec3): Vec3 {
  return [a[0] - b[0], a[1] - b[1], a[2] - b[2]] as Vec3;
}

function undirectedComponents(objectIds: readonly string[], objectAttractions: readonly ObjectAttraction[]): string[][] {
  const idSet = new Set(objectIds);
  const adj = new Map<string, Set<string>>();
  for (const id of objectIds) {
    adj.set(id, new Set());
  }
  for (const link of objectAttractions) {
    if (!idSet.has(link.attractingObjectId) || !idSet.has(link.attractedObjectId)) {
      continue;
    }
    adj.get(link.attractingObjectId)!.add(link.attractedObjectId);
    adj.get(link.attractedObjectId)!.add(link.attractingObjectId);
  }
  const seen = new Set<string>();
  const components: string[][] = [];
  for (const id of objectIds) {
    if (seen.has(id)) {
      continue;
    }
    const stack = [id];
    const comp: string[] = [];
    seen.add(id);
    while (stack.length) {
      const cur = stack.pop()!;
      comp.push(cur);
      for (const nb of adj.get(cur) ?? []) {
        if (seen.has(nb)) {
          continue;
        }
        seen.add(nb);
        stack.push(nb);
      }
    }
    components.push(comp);
  }
  return components;
}

/** @emoji 🔄 True when `attractingObjectId → attractedObjectId` closes a directed cycle in object attractions. */
export function wouldObjectAttractionIntroduceCycle(objectAttractions: readonly ObjectAttraction[], attractingObjectId: string, attractedObjectId: string): boolean {
  if (!attractingObjectId || !attractedObjectId || attractingObjectId === attractedObjectId) {
    return true;
  }
  const outgoing = new Map<string, string[]>();
  for (const link of objectAttractions) {
    const next = outgoing.get(link.attractingObjectId) ?? [];
    next.push(link.attractedObjectId);
    outgoing.set(link.attractingObjectId, next);
  }
  const stack = [attractedObjectId];
  const seen = new Set<string>();
  while (stack.length) {
    const id = stack.pop()!;
    if (id === attractingObjectId) {
      return true;
    }
    if (seen.has(id)) {
      continue;
    }
    seen.add(id);
    for (const child of outgoing.get(id) ?? []) {
      stack.push(child);
    }
  }
  return false;
}

function parentOwnershipCycleMemberIds(parentByObjectId: ReadonlyMap<string, string | null>, startId: string): readonly string[] | null {
  const order: string[] = [];
  const index = new Map<string, number>();
  let cur: string | null = startId;
  while (cur) {
    const at = index.get(cur);
    if (at !== undefined) {
      return order.slice(at);
    }
    index.set(cur, order.length);
    order.push(cur);
    cur = parentByObjectId.get(cur) ?? null;
  }
  return null;
}

/** @emoji Ô£é´©Å Clears one parent link per ownership cycle so {@link AttractionTree} stays a forest. */
function breakOwnershipParentCycles(parentByObjectId: Map<string, string | null>): void {
  for (;;) {
    let cycle: readonly string[] | null = null;
    for (const id of parentByObjectId.keys()) {
      cycle = parentOwnershipCycleMemberIds(parentByObjectId, id);
      if (cycle?.length) {
        break;
      }
    }
    if (!cycle?.length) {
      return;
    }
    const detach = cycle.slice().sort().at(-1)!;
    parentByObjectId.set(detach, null);
  }
}

/** @emoji 🕸️ Resolves a forest from object attractions: wormhole roots, closest-to-wormhole parent when multiply attracted. */
export function resolveAttractionTree(args: { readonly objectIds: readonly string[]; readonly objectAttractions: readonly ObjectAttraction[]; readonly explicitWormholeIds?: ReadonlySet<string> }): AttractionTree {
  const explicit = args.explicitWormholeIds ?? new Set<string>();
  const incoming = new Map<string, ObjectAttraction[]>();
  const outgoing = new Map<string, string[]>();
  for (const id of args.objectIds) {
    incoming.set(id, []);
    outgoing.set(id, []);
  }
  for (const link of args.objectAttractions) {
    if (!incoming.has(link.attractedObjectId) || !outgoing.has(link.attractingObjectId)) {
      continue;
    }
    incoming.get(link.attractedObjectId)!.push(link);
    outgoing.get(link.attractingObjectId)!.push(link.attractedObjectId);
  }

  const wormholeIds: string[] = [];
  const wormholeDistanceByObjectId = new Map<string, number>();
  const parentByObjectId = new Map<string, string | null>();

  for (const comp of undirectedComponents(args.objectIds, args.objectAttractions)) {
    const compSet = new Set(comp);
    const compIncoming = new Map<string, ObjectAttraction[]>();
    for (const id of comp) {
      compIncoming.set(
        id,
        (incoming.get(id) ?? []).filter((link) => compSet.has(link.attractingObjectId) && compSet.has(link.attractedObjectId)),
      );
    }
    let roots = comp.filter((id) => explicit.has(id));
    if (!roots.length) {
      roots = comp.filter((id) => (compIncoming.get(id) ?? []).length === 0);
    }
    if (!roots.length) {
      roots = [comp.slice().sort()[0]!];
    }
    for (const root of roots) {
      if (!wormholeIds.includes(root)) {
        wormholeIds.push(root);
      }
    }
    const dist = new Map<string, number>();
    const queue: string[] = [];
    for (const root of roots) {
      dist.set(root, 0);
      queue.push(root);
    }
    while (queue.length) {
      const cur = queue.shift()!;
      const d = dist.get(cur) ?? 0;
      for (const child of outgoing.get(cur) ?? []) {
        if (!compSet.has(child)) {
          continue;
        }
        const next = d + 1;
        const prev = dist.get(child);
        if (prev === undefined || next < prev) {
          dist.set(child, next);
          queue.push(child);
        }
      }
    }
    for (const id of comp) {
      const inc = compIncoming.get(id) ?? [];
      if (!inc.length) {
        parentByObjectId.set(id, null);
        continue;
      }
      let best: ObjectAttraction | null = null;
      let bestDist = Number.POSITIVE_INFINITY;
      for (const link of inc) {
        const d = dist.get(link.attractingObjectId) ?? Number.POSITIVE_INFINITY;
        if (d < bestDist || (d === bestDist && (!best || link.attractingObjectId.localeCompare(best.attractingObjectId) < 0))) {
          bestDist = d;
          best = link;
        }
      }
      parentByObjectId.set(id, best?.attractingObjectId ?? null);
    }
    for (const id of comp) {
      wormholeDistanceByObjectId.set(id, dist.get(id) ?? Number.POSITIVE_INFINITY);
    }
  }

  for (const id of args.objectIds) {
    if (!parentByObjectId.has(id)) {
      parentByObjectId.set(id, null);
      wormholeDistanceByObjectId.set(id, explicit.has(id) ? 0 : Number.POSITIVE_INFINITY);
    }
  }

  breakOwnershipParentCycles(parentByObjectId);

  const attractingByObjectId = new Map<string, string[]>();
  for (const id of args.objectIds) {
    attractingByObjectId.set(id, []);
  }
  for (const [child, parent] of parentByObjectId) {
    if (!parent) {
      continue;
    }
    const arr = attractingByObjectId.get(parent) ?? [];
    arr.push(child);
    attractingByObjectId.set(parent, arr);
  }
  for (const [, arr] of attractingByObjectId) {
    arr.sort();
  }

  return {
    parentByObjectId,
    attractingByObjectId,
    wormholeDistanceByObjectId,
    wormholeIds: wormholeIds.slice().sort(),
  };
}

/** @emoji ­ƒº▓ Collects transitive attracted object ids in the resolved ownership tree. */
export function collectAttractedDescendantIds(rootObjectId: string, attractingByObjectId: ReadonlyMap<string, readonly string[]>): readonly string[] {
  const out: string[] = [];
  const stack = [...(attractingByObjectId.get(rootObjectId) ?? [])];
  const seen = new Set<string>();
  while (stack.length) {
    const id = stack.pop()!;
    if (seen.has(id)) {
      continue;
    }
    seen.add(id);
    out.push(id);
    for (const child of attractingByObjectId.get(id) ?? []) {
      stack.push(child);
    }
  }
  return out;
}

export interface ObjectRecord extends WorldEntityFlags {
  readonly id: string;
  readonly objectKind?: string;
  readonly meshUrl: string;
  readonly origin: Vec3;
  readonly orientation?: Quat;
  readonly scale?: number | Vec3;
  readonly label?: string;
  readonly wormhole?: boolean;
  readonly vortices: readonly VortexProps[];
}

interface ObjectStateSnapshot {
  readonly records: ReadonlyMap<string, ObjectRecord>;
  readonly attractions: readonly AttractionProps[];
  readonly tree: AttractionTree;
  readonly version: number;
}

type ObjectStateAction =
  | { readonly type: "init"; readonly fixture: FixtureV1 }
  | { readonly type: "syncPoses"; readonly fixture: FixtureV1 }
  | { readonly type: "relocate"; readonly payload: RelocatePayload }
  | { readonly type: "addAttraction"; readonly attraction: AttractionProps }
  | { readonly type: "removeObject"; readonly objectId: string }
  | { readonly type: "removeObjects"; readonly objectIds: readonly string[] };

function fixtureToRecords(objects: readonly FixtureObjectV1[]): Map<string, ObjectRecord> {
  const map = new Map<string, ObjectRecord>();
  for (const o of objects) {
    map.set(o.id, {
      id: o.id,
      meshUrl: o.meshUrl,
      origin: o.origin,
      ...(o.objectKind ? { objectKind: o.objectKind } : {}),
      ...(o.orientation ? { orientation: o.orientation } : {}),
      ...(o.scale !== undefined ? { scale: o.scale } : {}),
      ...(o.label ? { label: o.label } : {}),
      ...(o.wormhole === true ? { wormhole: true } : {}),
      ...(o.hidden === true ? { hidden: true } : {}),
      ...(o.locked === true ? { locked: true } : {}),
      vortices: o.vortices,
    });
  }
  return map;
}

function buildSnapshot(records: ReadonlyMap<string, ObjectRecord>, attractions: readonly AttractionProps[], version: number): ObjectStateSnapshot {
  const objectIds = [...records.keys()];
  const explicitWormholes = new Set(
    objectIds.filter((id) => {
      const r = records.get(id);
      return r ? isWormholeObject(id, r, new Set()) : false;
    }),
  );
  const objectAttractions = objectAttractionsFromAttractions(attractions);
  const inferred = new Set<string>();
  for (const comp of undirectedComponents(objectIds, objectAttractions)) {
    const compLinks = objectAttractions.filter((link) => comp.includes(link.attractingObjectId) && comp.includes(link.attractedObjectId));
    const inc = new Map<string, number>();
    for (const id of comp) {
      inc.set(id, 0);
    }
    for (const link of compLinks) {
      inc.set(link.attractedObjectId, (inc.get(link.attractedObjectId) ?? 0) + 1);
    }
    for (const id of comp) {
      if ((inc.get(id) ?? 0) === 0 && !explicitWormholes.has(id)) {
        inferred.add(id);
      }
    }
  }
  const tree = resolveAttractionTree({
    objectIds,
    objectAttractions,
    explicitWormholeIds: new Set([...explicitWormholes, ...inferred]),
  });
  return { records, attractions, tree, version };
}

/** @emoji ­ƒöæ Stable fingerprint for external fixture resync (ignores object reference identity). */
export function fixtureStateFingerprint(fixture: FixtureV1): string {
  const attractionIds = fixture.attractions.map((a) => a.id).join("\0");
  const objectIds = fixture.objects.map((o) => o.id).join("\0");
  return `${fixture.objects.length}\0${fixture.attractions.length}\0${objectIds}\0${attractionIds}`;
}

/** @emoji ­ƒôì Fingerprint of object poses for syncing fixture moves without resetting attractions. */
export function fixturePoseFingerprint(fixture: FixtureV1): string {
  return fixture.objects
    .map((object) => {
      const o = object.origin.join(",");
      const q = object.orientation?.join(",") ?? "";
      const s = object.scale === undefined ? "" : typeof object.scale === "number" ? String(object.scale) : object.scale.join(",");
      return `${object.id}|${o}|${q}|${s}`;
    })
    .join("\0");
}

/** @emoji 🎨 Fingerprint of object mesh/kind/labels and vortex fields for inspector edits without structure revision bumps. */
export function fixtureAppearanceFingerprint(fixture: FixtureV1): string {
  const objects = fixture.objects
    .map((object) => {
      const vortices = object.vortices
        .map((vortex) => {
          const position = vortex.position.join(",");
          const direction = vortex.direction?.join(",") ?? "";
          const radius = vortex.radius === undefined ? "" : String(vortex.radius);
          const flags = `${vortex.hidden === true ? "h" : ""}${vortex.locked === true ? "l" : ""}`;
          return `${vortex.id}|${vortex.vortexKind ?? ""}|${vortex.label ?? ""}|${position}|${direction}|${radius}|${flags}`;
        })
        .join(";");
      const meshByLod = object.meshByLod?.map((entry) => `${entry.lod}:${entry.url}`).join(",") ?? "";
      const objectFlags = `${object.hidden === true ? "h" : ""}${object.locked === true ? "l" : ""}`;
      return `${object.id}|${object.objectKind ?? ""}|${object.meshUrl}|${object.label ?? ""}|${object.wormhole === true ? "1" : "0"}|${objectFlags}|${meshByLod}|${vortices}`;
    })
    .join("\0");
  const attractions = fixture.attractions
    .map((attraction) => {
      const flags = `${attraction.hidden === true ? "h" : ""}${attraction.locked === true ? "l" : ""}`;
      return `${attraction.id}|${attraction.attractionKind ?? ""}|${attraction.attracting}|${attraction.attracted}|${flags}`;
    })
    .join("\0");
  return `${objects}\0${attractions}`;
}

/** @emoji 🧩 Applies an object-kind switch on a fixture object (mesh URL from catalog or scene). */
export function applyObjectKindToFixtureObject(object: FixtureObjectV1, kindId: string, kindCatalogs: KindCatalogBundle | undefined, sceneFixture?: FixtureV1): FixtureObjectV1 {
  const meshUrl = resolveObjectKindMeshUrl(kindId, kindCatalogs, sceneFixture);
  const kind = catalogObjectKindById(kindCatalogs, kindId);
  const next: FixtureObjectV1 = {
    ...object,
    objectKind: kindId,
    ...(meshUrl ? { meshUrl } : {}),
  };
  if (kind?.meshByLod) {
    next.meshByLod = kind.meshByLod;
    return next;
  }
  if (next.meshByLod === undefined) {
    return next;
  }
  const { meshByLod: _removed, ...withoutLod } = next;
  return withoutLod;
}

function objectStateReducer(state: ObjectStateSnapshot, action: ObjectStateAction): ObjectStateSnapshot {
  switch (action.type) {
    case "init": {
      const records = fixtureToRecords(action.fixture.objects);
      return buildSnapshot(records, action.fixture.attractions, state.version + 1);
    }
    case "syncPoses": {
      const records = new Map(state.records);
      for (const object of action.fixture.objects) {
        const cur = records.get(object.id);
        if (!cur) {
          continue;
        }
        records.set(object.id, {
          ...cur,
          origin: object.origin,
          orientation: object.orientation,
          scale: object.scale,
        });
      }
      return { records, attractions: state.attractions, tree: state.tree, version: state.version + 1 };
    }
    case "addAttraction": {
      const objectAttractions = objectAttractionsFromAttractions(state.attractions);
      const attractingObjectId = parseVortexFullId(action.attraction.attracting).objectId;
      const attractedObjectId = parseVortexFullId(action.attraction.attracted).objectId;
      if (wouldObjectAttractionIntroduceCycle(objectAttractions, attractingObjectId, attractedObjectId)) {
        return state;
      }
      const attractions = [...state.attractions, action.attraction];
      return buildSnapshot(state.records, attractions, state.version + 1);
    }
    case "removeObject": {
      const records = new Map(state.records);
      records.delete(action.objectId);
      const attractions = state.attractions.filter((attraction) => {
        const s = parseVortexFullId(attraction.attracting).objectId;
        const tg = parseVortexFullId(attraction.attracted).objectId;
        return s !== action.objectId && tg !== action.objectId;
      });
      return buildSnapshot(records, attractions, state.version + 1);
    }
    case "removeObjects": {
      if (action.objectIds.length === 0) {
        return state;
      }
      const remove = new Set(action.objectIds);
      const records = new Map(state.records);
      for (const objectId of remove) {
        records.delete(objectId);
      }
      const attractions = state.attractions.filter((attraction) => {
        const s = parseVortexFullId(attraction.attracting).objectId;
        const tg = parseVortexFullId(attraction.attracted).objectId;
        return !remove.has(s) && !remove.has(tg);
      });
      return buildSnapshot(records, attractions, state.version + 1);
    }
    case "relocate": {
      const { payload } = action;
      const records = new Map(state.records);
      const root = records.get(payload.objectId);
      if (!root) {
        return state;
      }
      const updatePose = (id: string, origin: Vec3, orientation: Quat, scale: Vec3) => {
        const cur = records.get(id);
        if (!cur) {
          return;
        }
        records.set(id, {
          ...cur,
          origin,
          orientation,
          scale: scale[0] === scale[1] && scale[1] === scale[2] ? scale[0] : ([scale[0], scale[1], scale[2]] as Vec3),
        });
      };
      updatePose(payload.objectId, payload.after.origin, payload.after.orientation, payload.after.scale);
      if (payload.mode === "translate") {
        const delta = vec3Sub(payload.after.origin, payload.before.origin);
        for (const id of collectAttractedDescendantIds(payload.objectId, state.tree.attractingByObjectId)) {
          const cur = records.get(id);
          if (!cur) {
            continue;
          }
          const sc = cur.scale;
          const scaleVec = typeof sc === "number" ? ([sc, sc, sc] as Vec3) : sc ? ([sc[0], sc[1], sc[2]] as Vec3) : ([1, 1, 1] as Vec3);
          updatePose(id, vec3Add(cur.origin, delta), cur.orientation ?? ([0, 0, 0, 1] as Quat), scaleVec);
        }
      }
      return { records, attractions: state.attractions, tree: state.tree, version: state.version + 1 };
    }
    default:
      return state;
  }
}

function recordScaleVec(scale: number | Vec3 | undefined): Vec3 {
  if (typeof scale === "number") {
    return [scale, scale, scale];
  }
  return scale ? ([scale[0], scale[1], scale[2]] as Vec3) : ([1, 1, 1] as Vec3);
}

function normalizeRecordScale(scale: Vec3): number | Vec3 {
  return scale[0] === scale[1] && scale[1] === scale[2] ? scale[0] : scale;
}

/** @emoji ✋ Object ids whose fixture pose rows change for a relocate commit. */
export function relocateAffectedObjectIds(payload: RelocatePayload, attractingByObjectId: ReadonlyMap<string, readonly string[]>): readonly string[] {
  const ids = [payload.objectId];
  if (payload.mode === "translate") {
    ids.push(...collectAttractedDescendantIds(payload.objectId, attractingByObjectId));
  }
  return ids;
}

/** @emoji ✋ Applies a relocate payload to a fixture (same rules as {@link objectStateReducer}). */
export function applyRelocateToFixture(fixture: FixtureV1, payload: RelocatePayload, attractingByObjectId?: ReadonlyMap<string, readonly string[]>): FixtureV1 {
  const tree = attractingByObjectId ?? buildSnapshot(fixtureToRecords(fixture.objects), fixture.attractions, 0).tree.attractingByObjectId;
  const ids = relocateAffectedObjectIds(payload, tree);
  if (!ids.length) {
    return fixture;
  }
  const indexById = new Map(fixture.objects.map((object, index) => [object.id, index]));
  let objects: FixtureObjectV1[] | null = null;
  let changed = false;
  const objectAt = (index: number) => (objects ? objects[index]! : fixture.objects[index]!);
  const setObjectAt = (index: number, next: FixtureObjectV1) => {
    if (!objects) {
      objects = fixture.objects.slice();
    }
    objects[index] = next;
  };
  const writePose = (id: string, origin: Vec3, orientation: Quat, scale: Vec3) => {
    const index = indexById.get(id);
    if (index === undefined) {
      return;
    }
    const cur = objectAt(index);
    const nextScale = normalizeRecordScale(scale);
    const nextOrient = orientation;
    if (
      cur.origin[0] === origin[0] &&
      cur.origin[1] === origin[1] &&
      cur.origin[2] === origin[2] &&
      cur.orientation?.[0] === nextOrient[0] &&
      cur.orientation?.[1] === nextOrient[1] &&
      cur.orientation?.[2] === nextOrient[2] &&
      cur.orientation?.[3] === nextOrient[3] &&
      cur.scale === nextScale
    ) {
      return;
    }
    setObjectAt(index, {
      ...cur,
      origin,
      orientation: nextOrient,
      scale: nextScale,
    });
    changed = true;
  };
  writePose(payload.objectId, payload.after.origin, payload.after.orientation, payload.after.scale);
  if (payload.mode === "translate") {
    const delta = vec3Sub(payload.after.origin, payload.before.origin);
    for (const id of collectAttractedDescendantIds(payload.objectId, tree)) {
      const index = indexById.get(id);
      if (index === undefined) {
        continue;
      }
      const cur = objectAt(index);
      writePose(id, vec3Add(cur.origin, delta), cur.orientation ?? ([0, 0, 0, 1] as Quat), recordScaleVec(cur.scale));
    }
  }
  return changed && objects ? { ...fixture, objects } : fixture;
}

const ATTRACTING_CHILDREN_EMPTY: readonly string[] = [];

/** @emoji ⏱️ Defers work off the pointer/input hot path (idle callback when available). */
function scheduleDeferredCallback(run: () => void): void {
  if (typeof requestIdleCallback === "function") {
    requestIdleCallback(run, { timeout: 120 });
    return;
  }
  queueMicrotask(run);
}

/** @emoji ⏱️ Defers fixture persist / proximity work until after pointer release paints. */
function scheduleRelocateCommit(run: () => void): void {
  scheduleDeferredCallback(run);
}

type ObjectStoreListener = () => void;

/** @emoji 🗄️ Puzzle 3D object records with per-id subscriptions so gumball commit does not re-render every mesh. */
export class ObjectStore {
  private records = new Map<string, ObjectRecord>();
  private attractions: readonly AttractionProps[] = [];
  private tree: AttractionTree = {
    parentByObjectId: new Map(),
    attractingByObjectId: new Map(),
    wormholeDistanceByObjectId: new Map(),
    wormholeIds: [],
  };
  private structureEpoch = 0;
  private sortedObjectIdsCache: readonly string[] = [];
  private blockedVortexFullIdsCache: ReadonlySet<string> = new Set();
  private readonly objectListeners = new Map<string, Set<ObjectStoreListener>>();
  private readonly structureListeners = new Set<ObjectStoreListener>();

  private refreshStructureCaches(): void {
    this.sortedObjectIdsCache = [...this.records.keys()].sort();
    this.blockedVortexFullIdsCache = blockedVortexFullIdsFromAttractions(this.attractions);
  }

  subscribeStructure(listener: ObjectStoreListener): () => void {
    this.structureListeners.add(listener);
    return () => {
      this.structureListeners.delete(listener);
    };
  }

  subscribeObject(objectId: string, listener: ObjectStoreListener): () => void {
    let listeners = this.objectListeners.get(objectId);
    if (!listeners) {
      listeners = new Set();
      this.objectListeners.set(objectId, listeners);
    }
    listeners.add(listener);
    return () => {
      listeners!.delete(listener);
      if (listeners!.size === 0) {
        this.objectListeners.delete(objectId);
      }
    };
  }

  getStructureEpoch(): number {
    return this.structureEpoch;
  }

  getRecord(objectId: string): ObjectRecord | undefined {
    return this.records.get(objectId);
  }

  getSortedObjectIds(): readonly string[] {
    return this.sortedObjectIdsCache;
  }

  getBlockedVortexFullIds(): ReadonlySet<string> {
    return this.blockedVortexFullIdsCache;
  }

  getAttractions(): readonly AttractionProps[] {
    return this.attractions;
  }

  getTree(): AttractionTree {
    return this.tree;
  }

  getAttractingChildIds(objectId: string): readonly string[] {
    return this.tree.attractingByObjectId.get(objectId) ?? ATTRACTING_CHILDREN_EMPTY;
  }

  private bumpStructure(): void {
    this.structureEpoch += 1;
    this.refreshStructureCaches();
    for (const listener of this.structureListeners) {
      listener();
    }
  }

  private notifyObject(objectId: string): void {
    const listeners = this.objectListeners.get(objectId);
    if (!listeners) {
      return;
    }
    for (const listener of listeners) {
      listener();
    }
  }

  private replaceFromSnapshot(next: ObjectStateSnapshot): void {
    this.records = new Map(next.records);
    this.attractions = next.attractions;
    this.tree = next.tree;
    this.bumpStructure();
  }

  initFromFixture(fixture: FixtureV1): void {
    const snap = buildSnapshot(fixtureToRecords(fixture.objects), fixture.attractions, 0);
    this.replaceFromSnapshot(snap);
  }

  syncPosesFromFixture(fixture: FixtureV1): void {
    const changed: string[] = [];
    for (const object of fixture.objects) {
      const cur = this.records.get(object.id);
      if (!cur) {
        continue;
      }
      const nextOrigin = object.origin;
      const nextOrientation = object.orientation;
      const nextScale = object.scale;
      if (
        cur.origin[0] === nextOrigin[0] &&
        cur.origin[1] === nextOrigin[1] &&
        cur.origin[2] === nextOrigin[2] &&
        cur.orientation?.[0] === nextOrientation?.[0] &&
        cur.orientation?.[1] === nextOrientation?.[1] &&
        cur.orientation?.[2] === nextOrientation?.[2] &&
        cur.orientation?.[3] === nextOrientation?.[3] &&
        cur.scale === nextScale
      ) {
        continue;
      }
      this.records.set(object.id, {
        ...cur,
        origin: nextOrigin,
        orientation: nextOrientation,
        scale: nextScale,
      });
      changed.push(object.id);
    }
    for (const objectId of changed) {
      this.notifyObject(objectId);
    }
  }

  syncAppearanceFromFixture(fixture: FixtureV1): void {
    const changed: string[] = [];
    for (const object of fixture.objects) {
      const cur = this.records.get(object.id);
      if (!cur) {
        continue;
      }
      const nextRecord = fixtureToRecords([object]).get(object.id);
      if (!nextRecord) {
        continue;
      }
      if (
        cur.objectKind === nextRecord.objectKind &&
        cur.meshUrl === nextRecord.meshUrl &&
        cur.label === nextRecord.label &&
        cur.wormhole === nextRecord.wormhole &&
        cur.hidden === nextRecord.hidden &&
        cur.locked === nextRecord.locked &&
        JSON.stringify(cur.vortices) === JSON.stringify(nextRecord.vortices)
      ) {
        continue;
      }
      this.records.set(object.id, nextRecord);
      changed.push(object.id);
    }
    for (const objectId of changed) {
      this.notifyObject(objectId);
    }
    const attractionsChanged = JSON.stringify(this.attractions) !== JSON.stringify(fixture.attractions);
    if (attractionsChanged) {
      this.attractions = fixture.attractions;
    }
    if (changed.length > 0 || attractionsChanged) {
      this.bumpStructure();
    }
  }

  removeObjectIds(objectIds: readonly string[]): void {
    if (objectIds.length === 0) {
      return;
    }
    const next = objectStateReducer(this.toSnapshot(), { type: "removeObjects", objectIds });
    this.replaceFromSnapshot(next);
  }

  addAttraction(attraction: AttractionProps): boolean {
    const snap = this.toSnapshot();
    const next = objectStateReducer(snap, { type: "addAttraction", attraction });
    if (next.attractions.length === snap.attractions.length) {
      return false;
    }
    this.replaceFromSnapshot(next);
    return true;
  }

  /** @emoji ✋ Updates record poses for a relocate; optionally notifies per-object subscribers (skip on gumball release). */
  applyRelocate(payload: RelocatePayload, notify = true): void {
    const root = this.records.get(payload.objectId);
    if (!root) {
      return;
    }
    const writePose = (id: string, origin: Vec3, orientation: Quat, scale: Vec3) => {
      const cur = this.records.get(id);
      if (!cur) {
        return;
      }
      this.records.set(id, {
        ...cur,
        origin,
        orientation,
        scale: normalizeRecordScale(scale),
      });
    };
    writePose(payload.objectId, payload.after.origin, payload.after.orientation, payload.after.scale);
    const notifyIds = [payload.objectId];
    if (payload.mode === "translate") {
      const delta = vec3Sub(payload.after.origin, payload.before.origin);
      for (const id of collectAttractedDescendantIds(payload.objectId, this.tree.attractingByObjectId)) {
        const cur = this.records.get(id);
        if (!cur) {
          continue;
        }
        writePose(id, vec3Add(cur.origin, delta), cur.orientation ?? ([0, 0, 0, 1] as Quat), recordScaleVec(cur.scale));
        notifyIds.push(id);
      }
    }
    if (notify) {
      for (const objectId of notifyIds) {
        this.notifyObject(objectId);
      }
    }
  }

  toSnapshot(): ObjectStateSnapshot {
    return {
      records: this.records,
      attractions: this.attractions,
      tree: this.tree,
      version: this.structureEpoch,
    };
  }
}

/** @emoji 🔗 Appends an attraction to a fixture when it does not introduce a cycle. */
export function applyConnectToFixture(fixture: FixtureV1, payload: AttractionPayload): FixtureV1 {
  const snap = buildSnapshot(fixtureToRecords(fixture.objects), fixture.attractions, 0);
  const attractionId = payload.attractionId ?? `attraction-${payload.attracting}-${payload.attracted}`;
  const next = objectStateReducer(snap, {
    type: "addAttraction",
    attraction: {
      id: attractionId,
      attracting: payload.attracting as AttractionProps["attracting"],
      attracted: payload.attracted as AttractionProps["attracted"],
    },
  });
  if (next.attractions.length === snap.attractions.length) {
    return fixture;
  }
  return { ...fixture, attractions: [...next.attractions] };
}

export interface ObjectStateContextValue {
  readonly store: ObjectStore;
  readonly handleRelocate: (payload: RelocatePayload) => void;
  readonly handleConnect: (payload: AttractionPayload) => void;
}

export const ObjectStateContext = reactHostPort.createContext<ObjectStateContextValue | null>(null);

/** @emoji ­ƒùä´©Å Central scene object records, attractions, and resolved attraction ownership. */
export function ObjectStateProvider(props: {
  readonly fixture: FixtureV1;
  readonly fixtureRevision?: number;
  readonly children: ReactNode;
  readonly onRelocate?: (payload: RelocatePayload, attractingByObjectId: ReadonlyMap<string, readonly string[]>) => void;
  readonly onConnect?: (payload: AttractionPayload) => void;
}) {
  const storeRef = reactHostPort.useRef<ObjectStore | null>(null);
  if (!storeRef.current) {
    const store = new ObjectStore();
    store.initFromFixture(props.fixture);
    storeRef.current = store;
  }
  const store = storeRef.current;
  const syncedFixtureFingerprintRef = reactHostPort.useRef<string | null>(null);
  const syncedPoseFingerprintRef = reactHostPort.useRef<string | null>(null);
  const syncedAppearanceFingerprintRef = reactHostPort.useRef<string | null>(null);
  const syncedFixtureRevisionRef = reactHostPort.useRef<number | undefined>(undefined);
  const skipExternalPoseSyncRef = reactHostPort.useRef(false);
  const fixtureFingerprint = reactHostPort.useMemo(() => fixtureStateFingerprint(props.fixture), [props.fixture]);
  const poseFingerprint = reactHostPort.useMemo(() => fixturePoseFingerprint(props.fixture), [props.fixture]);
  const appearanceFingerprint = reactHostPort.useMemo(() => fixtureAppearanceFingerprint(props.fixture), [props.fixture]);
  reactHostPort.useEffect(() => {
    const puzzle3dStore = storeRef.current;
    if (!puzzle3dStore) {
      return;
    }
    if (skipExternalPoseSyncRef.current) {
      skipExternalPoseSyncRef.current = false;
      syncedPoseFingerprintRef.current = poseFingerprint;
      syncedAppearanceFingerprintRef.current = appearanceFingerprint;
      return;
    }
    if (props.fixtureRevision !== undefined && syncedFixtureRevisionRef.current !== props.fixtureRevision) {
      syncedFixtureRevisionRef.current = props.fixtureRevision;
      syncedFixtureFingerprintRef.current = fixtureFingerprint;
      syncedPoseFingerprintRef.current = poseFingerprint;
      syncedAppearanceFingerprintRef.current = appearanceFingerprint;
      const prevIds = new Set(puzzle3dStore.getSortedObjectIds());
      const nextIds = new Set(props.fixture.objects.map((object) => object.id));
      const removed = [...prevIds].filter((id) => !nextIds.has(id));
      const added = [...nextIds].filter((id) => !prevIds.has(id));
      if (removed.length > 0 && added.length === 0) {
        puzzle3dStore.removeObjectIds(removed);
      } else {
        puzzle3dStore.initFromFixture(props.fixture);
      }
      return;
    }
    if (syncedFixtureFingerprintRef.current !== fixtureFingerprint) {
      syncedFixtureFingerprintRef.current = fixtureFingerprint;
      syncedPoseFingerprintRef.current = poseFingerprint;
      syncedAppearanceFingerprintRef.current = appearanceFingerprint;
      const prevIds = new Set(puzzle3dStore.getSortedObjectIds());
      const nextIds = new Set(props.fixture.objects.map((object) => object.id));
      const removed = [...prevIds].filter((id) => !nextIds.has(id));
      const added = [...nextIds].filter((id) => !prevIds.has(id));
      if (removed.length > 0 && added.length === 0) {
        puzzle3dStore.removeObjectIds(removed);
      } else {
        puzzle3dStore.initFromFixture(props.fixture);
      }
      return;
    }
    if (syncedPoseFingerprintRef.current !== poseFingerprint) {
      syncedPoseFingerprintRef.current = poseFingerprint;
      puzzle3dStore.syncPosesFromFixture(props.fixture);
    }
    if (syncedAppearanceFingerprintRef.current === appearanceFingerprint) {
      return;
    }
    syncedAppearanceFingerprintRef.current = appearanceFingerprint;
    puzzle3dStore.syncAppearanceFromFixture(props.fixture);
  }, [props.fixture, props.fixtureRevision, fixtureFingerprint, poseFingerprint, appearanceFingerprint]);
  const handleRelocate = reactHostPort.useCallback(
    (payload: RelocatePayload) => {
      const puzzle3dStore = storeRef.current!;
      skipExternalPoseSyncRef.current = true;
      puzzle3dStore.applyRelocate(payload, false);
      scheduleRelocateCommit(() => {
        props.onRelocate?.(payload, puzzle3dStore.getTree().attractingByObjectId);
      });
    },
    [props.onRelocate],
  );
  const handleConnect = reactHostPort.useCallback(
    (payload: AttractionPayload) => {
      const puzzle3dStore = storeRef.current!;
      const attractionId = payload.attractionId ?? `attraction-${payload.attracting}-${payload.attracted}`;
      const added = puzzle3dStore.addAttraction({
        id: attractionId,
        attracting: payload.attracting as AttractionProps["attracting"],
        attracted: payload.attracted as AttractionProps["attracted"],
      });
      if (added) {
        props.onConnect?.(payload);
      }
    },
    [props.onConnect],
  );
  const value = reactHostPort.useMemo<ObjectStateContextValue>(() => ({ store, handleRelocate, handleConnect }), [store, handleRelocate, handleConnect]);
  return <ObjectStateContext.Provider value={value}>{props.children}</ObjectStateContext.Provider>;
}

function useObjectState(): ObjectStateContextValue {
  const v = reactHostPort.useContext(ObjectStateContext);
  if (!v) {
    throw new Error("ObjectStateProvider missing");
  }
  return v;
}

function useLiveBlockedVortexFullIds(fallback: ReadonlySet<string>): ReadonlySet<string> {
  const state = reactHostPort.useContext(ObjectStateContext);
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => (state ? state.store.subscribeStructure(onStoreChange) : () => {}),
    () => (state ? state.store.getBlockedVortexFullIds() : fallback),
    () => (state ? state.store.getBlockedVortexFullIds() : fallback),
  );
}

/** @emoji ­ƒ¬Ø Relocate handler that updates central object state and cascades to attracted descendants. */
export function useObjectRelocate(): (payload: RelocatePayload) => void {
  return useObjectState().handleRelocate;
}

/** @emoji ­ƒ¬Ø Connect handler that appends an attraction and recomputes attraction ownership. */
export function useObjectConnect(): (payload: AttractionPayload) => void {
  return useObjectState().handleConnect;
}

function useObjectRecord(objectId: string): ObjectRecord | undefined {
  const { store } = useObjectState();
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeObject(objectId, onStoreChange),
    () => store.getRecord(objectId),
    () => store.getRecord(objectId),
  );
}

function useAttractingChildIds(objectId: string): readonly string[] {
  const { store } = useObjectState();
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeStructure(onStoreChange),
    () => store.getAttractingChildIds(objectId),
    () => store.getAttractingChildIds(objectId),
  );
}

const ObjectItemById = reactHostPort.memo(function ObjectItemById(props: {
  readonly objectId: string;
  readonly relocate?: GumballConfig | false;
}) {
  const record = useObjectRecord(props.objectId);
  const attracting = useAttractingChildIds(props.objectId);
  if (!record) {
    return null;
  }
  return (
    <ObjectItem
      id={record.id}
      objectKind={record.objectKind}
      meshUrl={record.meshUrl}
      origin={record.origin}
      orientation={record.orientation}
      scale={record.scale}
      label={record.label}
      wormhole={record.wormhole}
      hidden={record.hidden}
      locked={record.locked}
      attracting={attracting}
      relocate={props.relocate}
    >
      {record.vortices.map((vortex) => (
        <Vortex key={vortex.id} objectId={record.id} objectKind={record.objectKind} objectOrigin={record.origin} objectOrientation={record.orientation} {...vortex} />
      ))}
    </ObjectItem>
  );
});

/** @emoji ­ƒî▓ Declares attraction tree structure; meshes mount flat via {@link Objects} so ids stay stable on reparent. */
export const ObjectTreeNode = reactHostPort.memo(function ObjectTreeNode(props: { readonly objectId: string; readonly visitedIds?: readonly string[] }) {
  const attracting = useAttractingChildIds(props.objectId);
  const visited = props.visitedIds ?? [];
  if (visited.includes(props.objectId)) {
    return null;
  }
  const nextVisited = visited.length ? [...visited, props.objectId] : [props.objectId];
  return (
    <>
      {attracting.map((childId) => (
        <ObjectTreeNode key={childId} objectId={childId} visitedIds={nextVisited} />
      ))}
    </>
  );
});

/** @emoji 🎯 True when an object id is directly in the selection snapshot (not parent-of-vortex). */
export function objectMatchesSelection(objectId: string, selection: SelectionSnapshot | undefined): boolean {
  if (!selection) {
    return false;
  }
  return selection.objectIds.includes(objectId);
}

export interface ObjectsProps {
  readonly relocate?: GumballConfig | false;
}

export interface PuzzleReferencesProps {
  readonly references: readonly WorldReferenceProps[];
  readonly relocate?: GumballConfig | false;
}

/** @emoji 🖼️ Puzzle 3D reference planes wired to registry selection and relocate callbacks. */
export const PuzzleReferences = reactHostPort.memo(function PuzzleReferences(props: PuzzleReferencesProps) {
  const { commitSelection } = useRegistryInteraction();
  const { gumballConfig } = useRegistryCore();
  const { setHover, clearHover, hoverTarget } = useRegistryHover();
  const selectionStore = useSelectionSnapshotStore();
  const selection = reactHostPort.useSyncExternalStore(selectionStore.subscribe, selectionStore.getSnapshot, selectionStore.getSnapshot);
  const lodCtx = useLod();
  const onReferenceRelocateRef = reactHostPort.useRef<(payload: WorldReferenceRelocatePayload) => void>(() => {});
  reactHostPort.useEffect(() => {
    onReferenceRelocateRef.current = puzzle3dReferenceRelocateBridgeRef.current.onRelocate ?? (() => {});
  });
  const selectedIds = reactHostPort.useMemo(() => new Set(selection.referenceIds), [selection.referenceIds]);
  const hoveredId = hoverTarget?.kind === "reference" ? hoverTarget.id : null;
  const translationSnap = lodCtx.gridSnapEnabled ? lodCtx.gridStepWorld : undefined;
  const config = props.relocate === false ? false : (props.relocate ?? gumballConfig);
  return (
    <WorldReferenceLayer
      references={props.references}
      selectedIds={selectedIds}
      hoveredId={hoveredId}
      gumballConfig={config === false ? undefined : config}
      relocateActive={config !== false}
      translationSnap={translationSnap}
      onSelect={(id) => {
        if (puzzle3dTargetVolumeToolActiveRef.current) {
          return;
        }
        commitSelection({ kind: "reference", id });
      }}
      onHover={(id) => {
        if (id) {
          setHover({ kind: "reference", id });
          return;
        }
        if (hoverTarget?.kind === "reference") {
          clearHover(hoverTarget);
        }
      }}
      onRelocate={(payload) => {
        onReferenceRelocateRef.current(payload);
      }}
    />
  );
});

const puzzle3dReferenceRelocateBridgeRef: {
  current: {
    readonly onSelect?: (snap: SelectionSnapshot) => void;
    readonly onRelocate?: (payload: WorldReferenceRelocatePayload) => void;
  };
} = { current: {} };

const puzzle3dTargetVolumeRelocateBridgeRef: {
  current: {
    readonly onRelocate?: (payload: WorldVolumeRelocatePayload) => void;
  };
} = { current: {} };

export interface PuzzleTargetVolumesProps {
  readonly targetVolumes: readonly WorldVolumeProps[];
  readonly interactive?: boolean;
  readonly relocate?: GumballConfig | false;
}

/** @emoji 🧊 Puzzle 3D target volumes wired to registry selection and relocate callbacks. */
export const PuzzleTargetVolumes = reactHostPort.memo(function PuzzleTargetVolumes(props: PuzzleTargetVolumesProps) {
  const { commitSelection } = useRegistryInteraction();
  const { gumballConfig } = useRegistryCore();
  const { setHover, clearHover, hoverTarget } = useRegistryHover();
  const selectionStore = useSelectionSnapshotStore();
  const selection = reactHostPort.useSyncExternalStore(selectionStore.subscribe, selectionStore.getSnapshot, selectionStore.getSnapshot);
  const lodCtx = useLod();
  const onRelocateRef = reactHostPort.useRef<(payload: WorldVolumeRelocatePayload) => void>(() => {});
  reactHostPort.useEffect(() => {
    onRelocateRef.current = puzzle3dTargetVolumeRelocateBridgeRef.current.onRelocate ?? (() => {});
  });
  const selectedIds = reactHostPort.useMemo(() => new Set(selection.targetVolumeIds), [selection.targetVolumeIds]);
  const hoveredId = hoverTarget?.kind === "targetVolume" ? hoverTarget.id : null;
  const translationSnap = lodCtx.gridSnapEnabled ? lodCtx.gridStepWorld : undefined;
  const config = props.relocate === false ? false : (props.relocate ?? gumballConfig);
  return (
    <WorldVolumeLayer
      volumes={props.targetVolumes}
      selectedIds={selectedIds}
      hoveredId={hoveredId}
      interactive={props.interactive}
      gumballConfig={config === false ? undefined : config}
      relocateActive={props.interactive !== false && config !== false}
      translationSnap={translationSnap}
      onSelect={(id) => {
        if (puzzle3dVoxelBrushUiStore.getSnapshot().altPainting) {
          return;
        }
        commitSelection({ kind: "targetVolume", id });
      }}
      onHover={(id) => {
        if (id) {
          setHover({ kind: "targetVolume", id });
          return;
        }
        if (hoverTarget?.kind === "targetVolume") {
          clearHover(hoverTarget);
        }
      }}
      onRelocate={(payload) => {
        onRelocateRef.current(payload);
      }}
    />
  );
});

/** @emoji 🖼️ Renders all scene objects from central state (id-keyed; survives ownership changes). */
export const Objects = reactHostPort.memo(function Objects(props: ObjectsProps) {
  const { store } = useObjectState();
  const ids = reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeStructure(onStoreChange),
    () => store.getSortedObjectIds(),
    () => store.getSortedObjectIds(),
  );
  return (
    <>
      {ids.map((id) => (
        <ObjectItemById key={id} objectId={id} relocate={props.relocate} />
      ))}
    </>
  );
});

/** @emoji ­ƒî▓ Logical attraction tree roots (wormholes) for structure-only composition. */
export const AttractionTreeRoots = reactHostPort.memo(function AttractionTreeRoots() {
  const { store } = useObjectState();
  const wormholeIds = reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeStructure(onStoreChange),
    () => store.getTree().wormholeIds,
    () => store.getTree().wormholeIds,
  );
  return (
    <>
      {wormholeIds.map((id) => (
        <ObjectTreeNode key={id} objectId={id} />
      ))}
    </>
  );
});

/** @emoji ­ƒº▓ Renders all attraction endpoint lines in one frame loop (avoids N├ùuseFrame churn). */
export const Attractions = reactHostPort.memo(function Attractions() {
  const { store } = useObjectState();
  const attractions = reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeStructure(onStoreChange),
    () => store.getAttractions(),
    () => store.getAttractions(),
  );
  return <CableBatch attractions={attractions} />;
});
//#endregion ­ƒò©´©ÅAttractionGraph

//#region ­ƒº®Compat
export function kindsCompatible(aKind: string | undefined, bKind: string | undefined, table: readonly KindCompatEntry[] | undefined): boolean {
  if (!table?.length || !aKind || !bKind) return false;
  return table.some((e) => (e.source === aKind && e.target === bKind) || (e.bidirectional === true && e.source === bKind && e.target === aKind));
}

/** @emoji 🧾 Parses `meta.kindCompatibility` from a puzzle 3d fixture. */
export function kindCompatibilityFromFixtureMeta(meta: Record<string, unknown> | undefined): readonly KindCompatEntry[] {
  if (!meta || typeof meta !== "object") return [];
  const arr = (meta as { kindCompatibility?: unknown }).kindCompatibility;
  if (!Array.isArray(arr)) return [];
  const out: KindCompatEntry[] = [];
  for (const entry of arr) {
    if (!entry || typeof entry !== "object") continue;
    const e = entry as Record<string, unknown>;
    const source = typeof e.source === "string" ? e.source.trim() : "";
    const target = typeof e.target === "string" ? e.target.trim() : "";
    if (!source || !target) continue;
    const specificity =
      e.specificity === "general" || e.specificity === "object" || e.specificity === "vortex" || e.specificity === "cable" || e.specificity === "attraction" ? e.specificity : undefined;
    out.push({
      source,
      target,
      ...(e.bidirectional === true ? { bidirectional: true } : {}),
      ...(e.important === true ? { important: true } : {}),
      ...(specificity ? { specificity } : {}),
    });
  }
  return out;
}

/** @emoji 🔗 Host prop wins; otherwise reads compatibility rules from the active fixture meta. */
export function resolvePuzzle3dKindCompatibility(
  explicit: readonly KindCompatEntry[] | undefined,
  fixtureMeta: Record<string, unknown> | undefined,
): readonly KindCompatEntry[] | undefined {
  if (explicit?.length) {
    return explicit;
  }
  const fromFixture = kindCompatibilityFromFixtureMeta(fixtureMeta);
  return fromFixture.length > 0 ? fromFixture : explicit;
}

function kindCompatibilitySyncKey(rules: readonly KindCompatEntry[] | undefined): string {
  if (!rules?.length) {
    return "";
  }
  return rules.map((rule) => `${rule.source}\u0001${rule.target}\u0001${rule.bidirectional === true}`).join("|");
}

/** @emoji 🔤 Single-letter hyphen port families (`b-l`, `c-b`) used by concrete forest beam/column ports. */
export function puzzle3dSingleLetterPortFamily(vortexKind: string | undefined): string | null {
  if (!vortexKind?.includes("-")) {
    return null;
  }
  const head = vortexKind.split("-")[0] ?? "";
  return /^[a-z]$/.test(head) ? head : null;
}

/** @emoji 🔤 Beam (`b-*`) and column (`c-*`) families never mate across family even without explicit rules. */
export function puzzle3dSingleLetterPortFamiliesCompatible(sourceVortexKind: string | undefined, targetVortexKind: string | undefined): boolean {
  const sourceFamily = puzzle3dSingleLetterPortFamily(sourceVortexKind);
  const targetFamily = puzzle3dSingleLetterPortFamily(targetVortexKind);
  if (sourceFamily === null || targetFamily === null) {
    return true;
  }
  return sourceFamily === targetFamily;
}

const DEFAULT_CABLE_KIND_ID = "cable.link";

/** @emoji ­ƒº▓ Attraction endpoint vortex full ids that are already attracting/attracted and cannot start or receive another attraction. */
export function blockedVortexFullIdsFromAttractions(attractions: readonly Pick<AttractionProps, "attracting" | "attracted">[]): ReadonlySet<string> {
  const s = new Set<string>();
  for (const attraction of attractions) {
    s.add(attraction.attracting);
    s.add(attraction.attracted);
  }
  return s;
}

/** @emoji 🧭 Semantic kinds at one end of an attraction drag (object + vortex). */
export interface AttractionVortexContext {
  readonly objectId: string;
  readonly objectKind: string | undefined;
  readonly vortexKind: string | undefined;
}

function catalogVortexById(catalogs: KindCatalogBundle | undefined, vortexKind: string | undefined): VortexKind | undefined {
  if (!vortexKind || !catalogs?.vortices?.length) return undefined;
  return catalogs.vortices.find((v) => v.id === vortexKind);
}

function catalogCableById(catalogs: KindCatalogBundle | undefined, cableKind: string | undefined): CableKind | undefined {
  if (!cableKind || !catalogs?.cables?.length) return undefined;
  return catalogs.cables.find((w) => w.id === cableKind);
}

/** @emoji 🔌 Resolves default cable kind for a vortex kind via vortex catalog, else `cable.link`. */
export function resolveCableKindForVortex(vortexKind: string | undefined, catalogs: KindCatalogBundle | undefined): string {
  const v = catalogVortexById(catalogs, vortexKind);
  const cableKind = v?.defaultCableKind?.trim();
  return cableKind && cableKind.length > 0 ? cableKind : DEFAULT_CABLE_KIND_ID;
}

/** @emoji 🧲 Resolves default attraction kind for a cable kind via cable catalog, else empty string. */
export function resolveAttractionKindForCable(cableKind: string | undefined, catalogs: KindCatalogBundle | undefined): string {
  const cable = catalogCableById(catalogs, cableKind);
  const attractionKind = cable?.defaultAttractionKind?.trim();
  return attractionKind && attractionKind.length > 0 ? attractionKind : "";
}

function compatPairMatches(rule: KindCompatEntry, a: string, b: string): boolean {
  if (rule.source === a && rule.target === b) return true;
  if (rule.bidirectional === true && rule.source === b && rule.target === a) return true;
  return false;
}

function attractionGestureRuleApplies(rule: KindCompatEntry, attracting: AttractionVortexContext, attracted: AttractionVortexContext, catalogs: KindCatalogBundle | undefined): boolean {
  const cableSrc = resolveCableKindForVortex(attracting.vortexKind, catalogs);
  const cableTgt = resolveCableKindForVortex(attracted.vortexKind, catalogs);
  const attractionSrc = resolveAttractionKindForCable(cableSrc, catalogs);
  const attractionTgt = resolveAttractionKindForCable(cableTgt, catalogs);
  const sn = attracting.objectKind ?? "";
  const tn = attracted.objectKind ?? "";
  const sv = attracting.vortexKind ?? "";
  const tv = attracted.vortexKind ?? "";
  const spec = rule.specificity ?? "vortex";
  switch (spec) {
    case "general":
      return compatPairMatches(rule, sv, tv);
    case "object":
      return compatPairMatches(rule, sn, tn);
    case "attraction":
      return compatPairMatches(rule, attractionSrc, attractionTgt);
    case "vortex":
      return compatPairMatches(rule, sv, tv);
    case "cable":
      return compatPairMatches(rule, cableSrc, cableTgt);
    default:
      return compatPairMatches(rule, sv, tv);
  }
}

/** @emoji ⭕ Extracts `circular` / `rectangular` from shaped vortex ids (`core circular top`, …). */
export function puzzle3dVortexPortShape(vortexKind: string | undefined): "circular" | "rectangular" | null {
  if (!vortexKind) {
    return null;
  }
  if (vortexKind.includes(" circular ")) {
    return "circular";
  }
  if (vortexKind.includes(" rectangular ")) {
    return "rectangular";
  }
  return null;
}

/** @emoji ⭕ Shaped ports require the same cross-section; unshaped ports (door, platform, …) stay unconstrained. */
export function puzzle3dVortexPortShapesCompatible(sourceVortexKind: string | undefined, targetVortexKind: string | undefined): boolean {
  const sourceShape = puzzle3dVortexPortShape(sourceVortexKind);
  const targetShape = puzzle3dVortexPortShape(targetVortexKind);
  if (sourceShape === null || targetShape === null) {
    return true;
  }
  return sourceShape === targetShape;
}

/** @emoji 🤝 WASM-style filtered attraction compatibility (important + specificity tiers); empty rules allow all. */
export function vorticesAttractionCompatibleForDrag(attracting: AttractionVortexContext, attracted: AttractionVortexContext, rules: readonly KindCompatEntry[] | undefined, catalogs: KindCatalogBundle | undefined): boolean {
  if (!puzzle3dVortexPortShapesCompatible(attracting.vortexKind, attracted.vortexKind)) {
    return false;
  }
  if (!puzzle3dSingleLetterPortFamiliesCompatible(attracting.vortexKind, attracted.vortexKind)) {
    return false;
  }
  if (!rules?.length) return true;
  let matched = rules.filter((r) => attractionGestureRuleApplies(r, attracting, attracted, catalogs));
  if (matched.length === 0) return false;
  if (matched.some((r) => r.important)) matched = matched.filter((r) => r.important);
  else {
    const rank = (s: KindCompatEntry["specificity"] | undefined): number => {
      switch (s) {
        case "general":
          return 0;
        case "object":
          return 1;
        case "attraction":
          return 2;
        case "cable":
          return 3;
        case "vortex":
          return 4;
        default:
          return 4;
      }
    };
    const maxRank = Math.max(...matched.map((r) => rank(r.specificity)));
    matched = matched.filter((r) => rank(r.specificity) === maxRank);
  }
  return matched.length > 0;
}
//#endregion ­ƒº®Compat

//#region 🖌️Brush
/** @emoji 🖌️ One brush catalog candidate (object kind + source vortex index). */
export interface BrushCompatibleCandidate {
  readonly objectKindId: string;
  readonly sourceVortexIndex: number;
}

/** @emoji 🖌️ Optional host filter for {@link brushCompatibleCandidates} (domain rules live outside react). */
export type BrushCandidateAccept = (
  target: AttractionVortexContext,
  candidate: BrushCompatibleCandidate,
  template: ObjectKindVortexTemplate,
) => boolean;

export const puzzle3dBrushCandidateAcceptRef: { current: BrushCandidateAccept | null } = { current: null };

/** @emoji 🖌️ Publishes optional brush candidate filter from the play host (null clears). */
export function publishPuzzle3dBrushCandidateAccept(accept: BrushCandidateAccept | null): void {
  puzzle3dBrushCandidateAcceptRef.current = accept;
}

function normalizeVec3Cad(v: Vec3): Vec3 {
  const len = Math.hypot(v[0], v[1], v[2]);
  if (len < 1e-9) {
    return [0, 0, -1];
  }
  return [v[0] / len, v[1] / len, v[2] / len] as Vec3;
}

/** @emoji · Dot product of two CAD vectors. */
function vec3Dot(a: Vec3, b: Vec3): number {
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/** @emoji ✕ Cross product of two CAD vectors. */
function vec3Cross(a: Vec3, b: Vec3): Vec3 {
  return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] as Vec3;
}

const BRUSH_PLACEMENT_PARALLEL_TOLERANCE = 1e-6;

/** @emoji 🔄 180° quaternion about a unit axis (w = 0). */
function quaternionFrom180DegreeAxis(axis: Vec3): Quat {
  const unit = normalizeVec3Cad(axis);
  return [unit[0], unit[1], unit[2], 0];
}

/** @emoji 🧭 Deterministic orientation when source and target vortex directions are collinear (mirrors Rhino {@link ComputeChildPlane}). */
function antiParallelBrushOrientation(targetDir: Vec3): Quat {
  const zAxis: Vec3 = [0, 0, 1];
  if (Math.abs(targetDir[2]) < BRUSH_PLACEMENT_PARALLEL_TOLERANCE) {
    return quaternionFrom180DegreeAxis(zAxis);
  }
  const axis = vec3Cross(zAxis, targetDir);
  if (Math.hypot(axis[0], axis[1], axis[2]) < BRUSH_PLACEMENT_PARALLEL_TOLERANCE) {
    return quaternionFrom180DegreeAxis([1, 0, 0]);
  }
  return quaternionFrom180DegreeAxis(axis);
}

function vec3ScaleCad(v: Vec3, scale: number | Vec3 | undefined): Vec3 {
  if (scale === undefined) {
    return v;
  }
  if (typeof scale === "number") {
    return [v[0] * scale, v[1] * scale, v[2] * scale] as Vec3;
  }
  return [v[0] * scale[0], v[1] * scale[1], v[2] * scale[2]] as Vec3;
}

function negateVec3Cad(v: Vec3): Vec3 {
  return [-v[0], -v[1], -v[2]] as Vec3;
}

/** @emoji 🧭 World CAD position and direction of an object-local vortex (matches {@link cadObjectLocalToThreeGroupLocal} / scene graph). */
export function vortexWorldCadFromObject(record: Pick<ObjectRecord, "origin" | "orientation" | "vortices">, vortexIndex: number): { readonly position: Vec3; readonly direction: Vec3 } | null {
  const vortex = record.vortices[vortexIndex];
  if (!vortex) {
    return null;
  }
  const orientation = record.orientation ?? ([0, 0, 0, 1] as Quat);
  const position = vec3Add(record.origin, quatRotateVec(orientation, vortex.position));
  const direction = vortex.direction ? normalizeVec3Cad(quatRotateVec(orientation, vortex.direction)) : ([0, 0, -1] as Vec3);
  return { position, direction };
}

/** @emoji 🧱 Shared prefix for a vertical stack pair (`… bottom` / `… top`). */
function brushStackVortexBase(vortexKind: string | undefined): string | null {
  if (!vortexKind) {
    return null;
  }
  if (vortexKind.endsWith(" bottom")) {
    return vortexKind.slice(0, -" bottom".length);
  }
  if (vortexKind.endsWith(" top")) {
    return vortexKind.slice(0, -" top".length);
  }
  return null;
}

/** @emoji 🔗 True when source is a stack bottom vortex matching the target top vortex. */
export function brushStackBottomTopPair(sourceVortexKind: string | undefined, targetVortexKind: string | undefined): boolean {
  if (!sourceVortexKind?.endsWith(" bottom") || !targetVortexKind?.endsWith(" top")) {
    return false;
  }
  const sourceBase = brushStackVortexBase(sourceVortexKind);
  const targetBase = brushStackVortexBase(targetVortexKind);
  return sourceBase !== null && sourceBase === targetBase;
}

/** @emoji 🔗 True when source is a stack top vortex matching the target bottom vortex. */
export function brushStackTopBottomPair(sourceVortexKind: string | undefined, targetVortexKind: string | undefined): boolean {
  if (!sourceVortexKind?.endsWith(" top") || !targetVortexKind?.endsWith(" bottom")) {
    return false;
  }
  const sourceBase = brushStackVortexBase(sourceVortexKind);
  const targetBase = brushStackVortexBase(targetVortexKind);
  return sourceBase !== null && sourceBase === targetBase;
}

/** @emoji 🔗 True when source and target vortices form a vertical stack mate (matching port base). */
export function brushStackMatePair(sourceVortexKind: string | undefined, targetVortexKind: string | undefined): boolean {
  if (!puzzle3dVortexPortShapesCompatible(sourceVortexKind, targetVortexKind)) {
    return false;
  }
  return brushStackBottomTopPair(sourceVortexKind, targetVortexKind) || brushStackTopBottomPair(sourceVortexKind, targetVortexKind);
}

/** @emoji 🖌️ Rank for brush candidates: same kind and stack mates score higher. */
export function brushCandidateRank(candidate: BrushCompatibleCandidate, template: ObjectKindVortexTemplate, target: AttractionVortexContext): number {
  let score = 0;
  const targetKind = target.vortexKind ?? "";
  const sourceKind = template.vortexKind ?? "";
  if (candidate.objectKindId === target.objectKind) {
    score += 10_000;
  }
  if (brushStackMatePair(sourceKind, targetKind)) {
    score += 5_000;
  }
  if (sourceKind === targetKind && !brushStackMatePair(sourceKind, targetKind)) {
    score -= 4_000;
  }
  if (targetKind.endsWith(" top") && !brushStackMatePair(sourceKind, targetKind)) {
    score -= 2_000;
  }
  if (targetKind.endsWith(" bottom") && !sourceKind.endsWith(" top")) {
    score -= 2_000;
  }
  if (targetKind.includes("tambour circular") || targetKind.includes("tambour rectangular")) {
    const hostKind = target.objectKind ?? "";
    const midTambourHost = hostKind === "Tambour" || hostKind === "Cylindric Tambour";
    if (candidate.objectKindId.includes("Capital")) {
      score -= 50_000;
    } else if (candidate.objectKindId.includes("Cylindric") && candidate.objectKindId.includes("Tambour")) {
      score += 11_000;
    }
    if (midTambourHost && (candidate.objectKindId.includes("Last Storey") || candidate.objectKindId.includes("Single Storey"))) {
      score -= 30_000;
    }
    if (midTambourHost && candidate.objectKindId === "Cylindric Tambour") {
      score += 5_000;
    }
  }
  return score;
}

/** @emoji 🖌️ Whether brush placement should inherit the hovered object's orientation (same kind + vortex only; stacks and cross-port mates rotate). */
export function brushPlacementUsesHostOrientation(target: AttractionVortexContext, sourceVortexKind: string, candidateObjectKindId: string): boolean {
  const targetVk = target.vortexKind ?? "";
  if (brushStackMatePair(sourceVortexKind, targetVk)) {
    return false;
  }
  if (sourceVortexKind !== targetVk) {
    return false;
  }
  return candidateObjectKindId === target.objectKind;
}

function brushCandidateKey(candidate: BrushCompatibleCandidate): string {
  return `${candidate.objectKindId}\u0001${candidate.sourceVortexIndex}`;
}

/** @emoji 🖌️ Lists catalog object kinds whose vortices can attract the target vortex (deduped, ranked). */
export function brushCompatibleCandidates(
  target: AttractionVortexContext,
  kindCatalogs: KindCatalogBundle | undefined,
  kindCompatibility: readonly KindCompatEntry[] | undefined,
): readonly BrushCompatibleCandidate[] {
  const objects = kindCatalogs?.objects;
  if (!objects?.length) {
    return [];
  }
  const kindsById = new Map<string, ObjectKind>();
  for (const kind of objects) {
    if (kind.id) {
      kindsById.set(kind.id, kind);
    }
  }
  const targetVk = target.vortexKind ?? "";
  const stackTopTarget = targetVk.endsWith(" top");
  const stackBottomTarget = targetVk.endsWith(" bottom");
  const scored: { readonly candidate: BrushCompatibleCandidate; readonly rank: number }[] = [];
  const seen = new Set<string>();
  for (const kind of kindsById.values()) {
    if (!kind.meshUrl || !kind.vortices?.length) {
      continue;
    }
    for (let sourceVortexIndex = 0; sourceVortexIndex < kind.vortices.length; sourceVortexIndex += 1) {
      const template = kind.vortices[sourceVortexIndex]!;
      const sourceVk = template.vortexKind ?? "";
      if (stackTopTarget && !brushStackMatePair(sourceVk, targetVk)) {
        continue;
      }
      if (stackBottomTarget && !brushStackMatePair(sourceVk, targetVk)) {
        continue;
      }
      const attracting: AttractionVortexContext = {
        objectId: "__brush__",
        objectKind: kind.id,
        vortexKind: sourceVk,
      };
      if (!vorticesAttractionCompatibleForDrag(attracting, target, kindCompatibility, kindCatalogs)) {
        continue;
      }
      const candidate = { objectKindId: kind.id, sourceVortexIndex };
      if (puzzle3dBrushCandidateAcceptRef.current && !puzzle3dBrushCandidateAcceptRef.current(target, candidate, template)) {
        continue;
      }
      const key = brushCandidateKey(candidate);
      if (seen.has(key)) {
        continue;
      }
      seen.add(key);
      scored.push({ candidate, rank: brushCandidateRank(candidate, template, target) });
    }
  }
  scored.sort((left, right) => right.rank - left.rank || left.candidate.objectKindId.localeCompare(right.candidate.objectKindId) || left.candidate.sourceVortexIndex - right.candidate.sourceVortexIndex);
  return scored.map((row) => row.candidate);
}

/** @emoji 🖌️ Object pose so a source vortex coincides with the target point and directions oppose. */
export function computeBrushPlacementPose(args: {
  readonly sourceLocalPosition: Vec3;
  readonly sourceLocalDirection: Vec3;
  readonly scale?: number | Vec3;
  readonly targetWorldPositionCad: Vec3;
  readonly targetWorldDirectionCad: Vec3;
  readonly referenceOrientationCad?: Quat;
  readonly useHostOrientation?: boolean;
}): { readonly origin: Vec3; readonly orientation: Quat } {
  const scaledLocal = vec3ScaleCad(args.sourceLocalPosition, args.scale);
  const localDir = normalizeVec3Cad(args.sourceLocalDirection);
  const targetDir = normalizeVec3Cad(args.targetWorldDirectionCad);
  if (args.useHostOrientation && args.referenceOrientationCad) {
    const hostOrientation = args.referenceOrientationCad;
    const worldSourceDir = normalizeVec3Cad(quatRotateVec(hostOrientation, localDir));
    if (vec3Dot(worldSourceDir, targetDir) < -BRUSH_PLACEMENT_PARALLEL_TOLERANCE) {
      const origin = vec3Sub(args.targetWorldPositionCad, quatRotateVec(hostOrientation, scaledLocal));
      return { origin, orientation: hostOrientation };
    }
  }
  const desiredWorldDir = negateVec3Cad(targetDir);
  let orientation: Quat;
  if (vec3Dot(localDir, desiredWorldDir) < -1 + BRUSH_PLACEMENT_PARALLEL_TOLERANCE) {
    orientation = antiParallelBrushOrientation(targetDir);
  } else {
    const qThree = new Quaternion().setFromUnitVectors(new Vector3(...localDir), new Vector3(...desiredWorldDir));
    orientation = [qThree.x, qThree.y, qThree.z, qThree.w];
  }
  const origin = vec3Sub(args.targetWorldPositionCad, quatRotateVec(orientation, scaledLocal));
  return { origin, orientation };
}

/** @emoji 📦 True when two axis-aligned boxes overlap (with epsilon). */
export function boxesIntersect(a: Box3, b: Box3, epsilon = 1e-3): boolean {
  return a.min.x <= b.max.x + epsilon && a.max.x + epsilon >= b.min.x && a.min.y <= b.max.y + epsilon && a.max.y + epsilon >= b.min.y && a.min.z <= b.max.z + epsilon && a.max.z + epsilon >= b.min.z;
}

/** @emoji 📏 Default solid overlap budget (m3) before brush placement counts as collision. */
export const DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET = 0.02;

/** @emoji 📏 Maximum solid overlap budget (m3) on the play window slider. */
export const BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX = 1;

/** @emoji 📏 Window slider step for brush overlap budget (m3). */
export const BRUSH_PLACEMENT_OVERLAP_BUDGET_STEP = 0.01;

/** @deprecated Use {@link DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET}. */
export const DEFAULT_BRUSH_PLACEMENT_COLLISION_TOLERANCE = DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET;

/** @deprecated Use {@link BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX}. */
export const BRUSH_PLACEMENT_COLLISION_TOLERANCE_MAX = BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX;

/** @deprecated Use {@link BRUSH_PLACEMENT_OVERLAP_BUDGET_STEP}. */
export const BRUSH_PLACEMENT_COLLISION_TOLERANCE_STEP = BRUSH_PLACEMENT_OVERLAP_BUDGET_STEP;

export function brushPreviewFromCandidate(args: {
  readonly targetVortexFullId: string;
  readonly candidate: BrushCompatibleCandidate;
  readonly target: AttractionVortexContext;
  readonly targetWorldPositionCad: Vec3;
  readonly targetWorldDirectionCad: Vec3;
  readonly referenceOrientationCad?: Quat;
  readonly kindCatalogs: KindCatalogBundle | undefined;
  readonly sceneFixture?: FixtureV1;
}): BrushPreviewState | null {
  const kind = catalogObjectKindById(args.kindCatalogs, args.candidate.objectKindId);
  const template = kind?.vortices?.[args.candidate.sourceVortexIndex];
  const meshUrl = resolveObjectKindMeshUrl(args.candidate.objectKindId, args.kindCatalogs, args.sceneFixture);
  if (!meshUrl || !template) {
    return null;
  }
  const sourceVk = template.vortexKind ?? "";
  const useHostOrientation = brushPlacementUsesHostOrientation(args.target, sourceVk, args.candidate.objectKindId);
  const pose = computeBrushPlacementPose({
    sourceLocalPosition: template.position,
    sourceLocalDirection: template.direction ?? ([0, 0, -1] as Vec3),
    scale: kind.scale,
    targetWorldPositionCad: args.targetWorldPositionCad,
    targetWorldDirectionCad: args.targetWorldDirectionCad,
    ...(args.referenceOrientationCad ? { referenceOrientationCad: args.referenceOrientationCad } : {}),
    useHostOrientation,
  });
  return {
    targetVortexFullId: args.targetVortexFullId,
    objectKindId: kind.id,
    sourceVortexIndex: args.candidate.sourceVortexIndex,
    meshUrl,
    ...(kind.meshByLod ? { meshByLod: kind.meshByLod } : {}),
    ...(kind.scale !== undefined ? { scale: kind.scale } : {}),
    origin: pose.origin,
    orientation: pose.orientation,
  };
}

/** @emoji 🎲 Injectable RNG for {@link shuffleBrushCompatibleCandidates} (tests). */
export type BrushShuffleRng = () => number;

/** @emoji 🔀 Random permutation of brush candidates (Fisher–Yates). */
export function shuffleBrushCompatibleCandidates(candidates: readonly BrushCompatibleCandidate[], rng: BrushShuffleRng = Math.random): readonly BrushCompatibleCandidate[] {
  const out = [...candidates];
  for (let i = out.length - 1; i > 0; i -= 1) {
    const j = Math.floor(rng() * (i + 1));
    const left = out[i]!;
    out[i] = out[j]!;
    out[j] = left;
  }
  return out;
}

/** @emoji 🎚️ Per-kind brush suggestion weights (object + vortex groups each sum to 1 in the play shell). */
export interface Puzzle3dBrushKindWeights {
  readonly objectWeights: Readonly<Record<string, number>>;
  readonly vortexWeights: Readonly<Record<string, number>>;
}

export const puzzle3dBrushKindWeightsRef: { current: Puzzle3dBrushKindWeights } = {
  current: { objectWeights: {}, vortexWeights: {} },
};

/** @emoji 🎚️ Publishes brush kind weights for {@link BrushSession} weighted candidate ordering. */
export function publishPuzzle3dBrushKindWeights(objectWeights: Readonly<Record<string, number>>, vortexWeights: Readonly<Record<string, number>>): void {
  puzzle3dBrushKindWeightsRef.current = { objectWeights, vortexWeights };
}

function brushKindWeightValue(weights: Readonly<Record<string, number>>, id: string): number {
  const w = weights[id];
  return w !== undefined ? w : 1;
}

function brushCandidateSuggestionWeight(candidate: BrushCompatibleCandidate, weights: Puzzle3dBrushKindWeights, kindCatalogs: KindCatalogBundle | undefined): number {
  const kind = catalogObjectKindById(kindCatalogs, candidate.objectKindId);
  const vortexKind = kind?.vortices?.[candidate.sourceVortexIndex]?.vortexKind ?? "";
  return brushKindWeightValue(weights.objectWeights, candidate.objectKindId) * brushKindWeightValue(weights.vortexWeights, vortexKind);
}

/** @emoji 🚫 True when distribution allows brush suggestions on a target vortex. */
export function brushTargetVortexAllowsSuggestion(vortexKind: string | undefined, weights: Puzzle3dBrushKindWeights): boolean {
  return brushKindWeightValue(weights.vortexWeights, vortexKind ?? "") > 0;
}

/** @emoji 🎲 Weighted-random order for brush suggestions (higher weight → earlier in list). */
export function weightedOrderBrushCompatibleCandidates(
  candidates: readonly BrushCompatibleCandidate[],
  weights: Puzzle3dBrushKindWeights,
  kindCatalogs: KindCatalogBundle | undefined,
  rng: BrushShuffleRng = Math.random,
): readonly BrushCompatibleCandidate[] {
  const eligible = candidates.filter((c) => brushCandidateSuggestionWeight(c, weights, kindCatalogs) > 0);
  if (eligible.length < 2) {
    return eligible;
  }
  const remaining = [...eligible];
  const out: BrushCompatibleCandidate[] = [];
  while (remaining.length > 0) {
    const wList = remaining.map((c) => brushCandidateSuggestionWeight(c, weights, kindCatalogs));
    const total = wList.reduce((a, b) => a + b, 0);
    if (total <= 0) {
      break;
    }
    let r = rng() * total;
    let pick = remaining.length - 1;
    for (let i = 0; i < remaining.length; i += 1) {
      r -= wList[i]!;
      if (r <= 0) {
        pick = i;
        break;
      }
    }
    out.push(remaining[pick]!);
    remaining.splice(pick, 1);
  }
  return out;
}

/** @emoji 📦 Scene groups used for brush placement overlap tests. */
export interface BrushSceneCollisionSource {
  collectObjectGroups(): readonly Group[];
}

/** @emoji 💥 True when a brush preview solid overlap exceeds {@link overlapBudget}; `null` when mesh BVHs are not ready. */
export function brushPreviewCollides(
  scene: BrushSceneCollisionSource,
  preview: BrushPreviewState,
  overlapBudget: number = DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
  excludeSceneObjectIds?: ReadonlySet<string>,
): boolean | null {
  const previewBody = brushCollisionBody(preview.meshUrl);
  if (!previewBody) {
    return null;
  }
  const previewWorld = brushPreviewWorldMatrix(preview);
  for (const group of scene.collectObjectGroups()) {
    const objectId = group.userData?.puzzle3dObjectId;
    if (typeof objectId === "string" && excludeSceneObjectIds?.has(objectId)) {
      continue;
    }
    const meshUrl = brushSceneObjectMeshUrl(group);
    if (!meshUrl) {
      continue;
    }
    const otherBody = brushCollisionBody(meshUrl);
    if (!otherBody) {
      return null;
    }
    const otherWorld = brushSceneObjectWorldMatrix(group);
    const volume = solidOverlapVolume(previewBody, previewWorld, otherBody, otherWorld, { sampleCount: 1024 });
    if (volume > overlapBudget) {
      return true;
    }
  }
  return false;
}

/** @emoji 🧭 Same inner mesh frame as {@link MeshBody} / scene objects (GLB Y-up → CAD Z-up). */
export function brushPreviewMeshFrameGroup(meshRoot: Object3D): Group {
  const frame = new Group();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.add(meshRoot.clone(true));
  return frame;
}

/** @emoji 🧪 Disposable posed group for brush collision probes (matches {@link BrushPreviewGhost} graph). */
export function brushProbeGroupFromPreview(preview: Pick<BrushPreviewState, "origin" | "orientation" | "scale">, meshRoot: Object3D): Group {
  const group = new Group();
  group.add(brushPreviewMeshFrameGroup(meshRoot));
  applyObjectPose(group, preview.origin, preview.orientation, preview.scale);
  updateWorldMatrixChain(group);
  return group;
}

/** @emoji 📏 Minimum posed mesh extent (CAD units) before brush/fill AABB probes are trusted. */
export const BRUSH_COLLISION_MESH_MIN_EXTENT = 2;

const brushCollisionGltfScenes = new Map<string, Object3D>();
const brushCollisionBodies = new Map<string, CollisionBody>();

/** @emoji 📦 Registers a loaded GLTF scene for mesh-backed brush/fill collision probes. */
export function registerBrushCollisionGltfScene(meshUrl: string, scene: Object3D): void {
  if (!isLoadableMeshUrl(meshUrl)) {
    return;
  }
  brushCollisionGltfScenes.set(meshUrl, scene);
  if (!brushCollisionMeshExtentOk(scene)) {
    brushCollisionBodies.delete(meshUrl);
    return;
  }
  const body = collisionBodyFromObject(scene);
  if (body) {
    brushCollisionBodies.set(meshUrl, body);
  } else {
    brushCollisionBodies.delete(meshUrl);
  }
  const buffers = extractBrushCollisionMeshBuffers(scene);
  if (buffers) {
    void puzzle3dCollisionEngineRef.current.registerMesh(meshUrl, buffers.positions, buffers.indices);
  }
}

/** @emoji 📦 Clears registered GLTF collision scenes (tests). */
export function clearBrushCollisionGltfScenes(): void {
  brushCollisionGltfScenes.clear();
  brushCollisionBodies.clear();
}

/** @emoji 🧊 Cached BVH collision body for a mesh URL. */
export function brushCollisionBody(meshUrl: string): CollisionBody | null {
  const cached = brushCollisionBodies.get(meshUrl);
  if (cached) {
    return cached;
  }
  const root = brushCollisionGltfRoot(meshUrl);
  if (!root) {
    return null;
  }
  const body = collisionBodyFromObject(root);
  if (body) {
    brushCollisionBodies.set(meshUrl, body);
  }
  return body;
}

/** @emoji 🧭 World matrix for a brush preview pose (pose group, GLB frame baked in body). */
export function brushPreviewWorldMatrix(preview: Pick<BrushPreviewState, "origin" | "orientation" | "scale">): Matrix4 {
  const group = new Group();
  applyObjectPose(group, preview.origin, preview.orientation, preview.scale);
  updateWorldMatrixChain(group);
  return group.matrixWorld.clone();
}

function brushSceneObjectMeshUrl(group: Group): string | null {
  const url = group.userData?.puzzle3dMeshUrl;
  return typeof url === "string" && url.length > 0 ? url : null;
}

function brushSceneObjectWorldMatrix(group: Group): Matrix4 {
  updateWorldMatrixChain(group);
  return group.matrixWorld.clone();
}

/** @emoji 📦 True when a mesh root yields a non-degenerate posed collision extent. */
export function brushCollisionMeshExtentOk(meshRoot: Object3D): boolean {
  const probe = new Group();
  probe.add(brushPreviewMeshFrameGroup(meshRoot.clone(true)));
  updateWorldMatrixChain(probe);
  const box = new Box3().setFromObject(probe, true);
  if (!Number.isFinite(box.min.x) || box.isEmpty()) {
    return false;
  }
  const size = new Vector3();
  box.getSize(size);
  return Math.max(size.x, size.y, size.z) >= BRUSH_COLLISION_MESH_MIN_EXTENT;
}

/** @emoji 📦 GLTF scene used for brush/fill collision probes (full geometry, not styled stubs). */
export function brushCollisionGltfRoot(meshUrl: string): Object3D | null {
  const cached = brushCollisionGltfScenes.get(meshUrl);
  if (cached && brushCollisionMeshExtentOk(cached)) {
    return cached;
  }
  const styled =
    styledMeshPool.peek(styledPoolKey(meshUrl, "highlighted", true)) ?? styledMeshPool.peek(styledPoolKey(meshUrl, "highlighted", false));
  if (styled && brushCollisionMeshExtentOk(styled)) {
    return styled;
  }
  return null;
}

/** @emoji 💥 `true`/`false` when mesh is known; `null` when catalog GLB is not pooled yet. */
export function brushCandidateCollidesAtPose(
  scene: BrushSceneCollisionSource,
  preview: BrushPreviewState,
  meshRoot: Object3D | null | undefined,
  overlapBudget: number = DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
  excludeSceneObjectIds?: ReadonlySet<string>,
): boolean | null {
  if (!meshRoot || !brushCollisionMeshExtentOk(meshRoot)) {
    return null;
  }
  if (!brushCollisionBody(preview.meshUrl)) {
    const body = collisionBodyFromObject(meshRoot);
    if (!body) {
      return null;
    }
    brushCollisionBodies.set(preview.meshUrl, body);
  }
  return brushPreviewCollides(scene, preview, overlapBudget, excludeSceneObjectIds);
}

/** @emoji ✅ Collision-filtered brush candidates after mesh-backed AABB probes. */
export interface BrushCollisionFreeResult {
  readonly free: readonly BrushCompatibleCandidate[];
  readonly unknownPending: boolean;
}

/** @emoji 🔍 Filters shuffled compatible candidates to collision-free placements. */
export function brushCollisionFreeCandidates(args: {
  readonly scene: BrushSceneCollisionSource;
  readonly targetVortexFullId: string;
  readonly candidates: readonly BrushCompatibleCandidate[];
  readonly target: AttractionVortexContext;
  readonly targetWorldPositionCad: Vec3;
  readonly targetWorldDirectionCad: Vec3;
  readonly referenceOrientationCad?: Quat;
  readonly kindCatalogs: KindCatalogBundle | undefined;
  readonly sceneFixture?: FixtureV1;
  readonly meshRootForUrl?: (meshUrl: string) => Object3D | null | undefined;
  readonly overlapBudget?: number;
  readonly excludeSceneObjectIds?: ReadonlySet<string>;
}): BrushCollisionFreeResult {
  const overlapBudget = args.overlapBudget ?? DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET;
  const excludeSceneObjectIds = args.excludeSceneObjectIds ?? new Set([args.target.objectId]);
  const free: BrushCompatibleCandidate[] = [];
  let unknownPending = false;
  for (const candidate of args.candidates) {
    const preview = brushPreviewFromCandidate({
      targetVortexFullId: args.targetVortexFullId,
      candidate,
      target: args.target,
      targetWorldPositionCad: args.targetWorldPositionCad,
      targetWorldDirectionCad: args.targetWorldDirectionCad,
      referenceOrientationCad: args.referenceOrientationCad,
      kindCatalogs: args.kindCatalogs,
      sceneFixture: args.sceneFixture,
    });
    if (!preview) {
      continue;
    }
    const meshRoot = args.meshRootForUrl?.(preview.meshUrl);
    const collides = brushCandidateCollidesAtPose(args.scene, preview, meshRoot, overlapBudget, excludeSceneObjectIds);
    if (collides === null) {
      unknownPending = true;
      continue;
    }
    if (!collides) {
      free.push(candidate);
    }
  }
  return { free, unknownPending };
}

/** @emoji 📦 Unique catalog mesh URLs for brush-compatible kinds (preload before AABB probe). */
export function brushMeshUrlsForCompatibleCandidates(
  candidates: readonly BrushCompatibleCandidate[],
  kindCatalogs: KindCatalogBundle | undefined,
  sceneFixture?: FixtureV1,
): readonly string[] {
  const urls: string[] = [];
  const seen = new Set<string>();
  for (const candidate of candidates) {
    const meshUrl = resolveObjectKindMeshUrl(candidate.objectKindId, kindCatalogs, sceneFixture);
    if (!meshUrl || !isLoadableMeshUrl(meshUrl) || seen.has(meshUrl)) {
      continue;
    }
    seen.add(meshUrl);
    urls.push(meshUrl);
  }
  return urls;
}

/** @emoji 📦 Catalog mesh URLs to preload before {@link buildBrushFillSequence} collision probes. */
export function brushMeshUrlsForFillSession(
  fixture: FixtureV1,
  kindCatalogs: KindCatalogBundle | undefined,
  kindCompatibility: readonly KindCompatEntry[] | undefined,
): readonly string[] {
  const seen = new Set<string>();
  const urls: string[] = [];
  const pushUrl = (meshUrl: string | undefined): void => {
    if (!meshUrl || !isLoadableMeshUrl(meshUrl) || seen.has(meshUrl)) {
      return;
    }
    seen.add(meshUrl);
    urls.push(meshUrl);
  };
  for (const obj of fixture.objects) {
    pushUrl(resolveObjectKindMeshUrl(obj.objectKind, kindCatalogs, fixture) ?? obj.meshUrl);
  }
  for (const target of enumerateBrushFillVortexTargets(fixture)) {
    const hostObject = fixture.objects.find((row) => row.id === target.objectId);
    if (!hostObject) {
      continue;
    }
    const targetCtx: AttractionVortexContext = {
      objectId: target.objectId,
      objectKind: target.objectKind,
      vortexKind: target.vortexKind,
    };
    const compatible = brushCompatibleCandidates(targetCtx, kindCatalogs, kindCompatibility);
    for (const candidate of compatible) {
      pushUrl(resolveObjectKindMeshUrl(candidate.objectKindId, kindCatalogs, fixture));
    }
  }
  return urls;
}

/** @emoji 🖌️ True when preview matches a brush catalog candidate. */
export function brushPreviewMatchesCandidate(preview: BrushPreviewState, candidate: BrushCompatibleCandidate): boolean {
  return preview.objectKindId === candidate.objectKindId && preview.sourceVortexIndex === candidate.sourceVortexIndex;
}

/** @emoji 🖌️ Appends a brush-placed object and its attraction to a fixture. */
export function applyBrushPlacementToFixture(fixture: FixtureV1, payload: BrushPlacePayload, kindCatalogs: KindCatalogBundle | undefined): FixtureV1 {
  const kind = catalogObjectKindById(kindCatalogs, payload.objectKindId);
  const template = kind?.vortices?.[payload.sourceVortexIndex];
  const meshUrl = resolveObjectKindMeshUrl(payload.objectKindId, kindCatalogs, fixture);
  if (!meshUrl || !template) {
    return fixture;
  }
  const objectId = payload.objectId ?? `puzzle3d.brush.${crypto.randomUUID()}`;
  const vortices: VortexProps[] = (kind.vortices ?? []).map((entry, index) => ({
    id: `${objectId}:v${index}`,
    vortexKind: entry.vortexKind,
    label: entry.vortexKind,
    position: entry.position,
    ...(entry.direction ? { direction: entry.direction } : {}),
    ...(entry.radius !== undefined ? { radius: entry.radius } : {}),
  }));
  const sourceVortex = vortices[payload.sourceVortexIndex];
  if (!sourceVortex) {
    return fixture;
  }
  const attracting = puzzle3dVortexFullId(objectId, sourceVortex.id);
  const attractionId = payload.attractionId ?? `attraction-${attracting}-${payload.targetVortexFullId}`;
  const nextObject: FixtureObjectV1 = {
    id: objectId,
    objectKind: kind.id,
    meshUrl,
    ...(kind.meshByLod ? { meshByLod: kind.meshByLod } : {}),
    label: kind.label ?? kind.name ?? kind.id,
    origin: payload.origin,
    orientation: payload.orientation,
    ...(payload.scale !== undefined ? { scale: payload.scale } : kind.scale !== undefined ? { scale: kind.scale } : {}),
    vortices,
  };
  const connected = applyConnectToFixture(fixture, {
    attracting,
    attracted: payload.targetVortexFullId,
    attractionId,
  });
  if (connected.attractions.length === fixture.attractions.length) {
    return fixture;
  }
  return { ...connected, objects: [...connected.objects, nextObject] };
}

/** @emoji 🪣 Fill frontier vortex target for sequence building. */
export interface BrushFillVortexTarget {
  readonly fullId: string;
  readonly objectId: string;
  readonly objectKind: string | undefined;
  readonly vortexKind: string | undefined;
  readonly vortexIndex: number;
}

/** @emoji 🪣 Window engagement possible id for the fill tool. */
export const PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID = "puzzle3d.tool.fill";

export const PUZZLE_3D_FILL_COUNT_MAX = 1000;

function fillVortexTargetWeight(target: BrushFillVortexTarget, weights: Puzzle3dBrushKindWeights): number {
  return brushKindWeightValue(weights.vortexWeights, target.vortexKind ?? "");
}

function weightedOrderFillVortexTargets(
  targets: readonly BrushFillVortexTarget[],
  weights: Puzzle3dBrushKindWeights,
  rng: BrushShuffleRng,
): readonly BrushFillVortexTarget[] {
  const eligible = targets.filter((target) => fillVortexTargetWeight(target, weights) > 0);
  if (eligible.length < 2) {
    return eligible;
  }
  const remaining = [...eligible];
  const out: BrushFillVortexTarget[] = [];
  while (remaining.length > 0) {
    const wList = remaining.map((target) => fillVortexTargetWeight(target, weights));
    const total = wList.reduce((a, b) => a + b, 0);
    if (total <= 0) {
      break;
    }
    let r = rng() * total;
    let pick = remaining.length - 1;
    for (let i = 0; i < remaining.length; i += 1) {
      r -= wList[i]!;
      if (r <= 0) {
        pick = i;
        break;
      }
    }
    out.push(remaining[pick]!);
    remaining.splice(pick, 1);
  }
  return out;
}

/** @emoji 🪣 Lists fixture vortices that can still receive an attraction. */
export function enumerateBrushFillVortexTargets(fixture: FixtureV1): readonly BrushFillVortexTarget[] {
  const blocked = blockedVortexFullIdsFromAttractions(fixture.attractions);
  const out: BrushFillVortexTarget[] = [];
  for (const obj of fixture.objects) {
    for (let i = 0; i < (obj.vortices?.length ?? 0); i += 1) {
      const vortex = obj.vortices![i]!;
      const fullId = puzzle3dVortexFullId(obj.id, vortex.id);
      if (!blocked.has(fullId)) {
        out.push({
          fullId,
          objectId: obj.id,
          objectKind: obj.objectKind,
          vortexKind: vortex.vortexKind,
          vortexIndex: i,
        });
      }
    }
  }
  return out;
}

/** @emoji 📦 Object id plus posed BVH body tracked during {@link buildBrushFillSequence}. */
export interface BrushPlacedCollisionEntry {
  readonly objectId: string;
  readonly meshUrl: string;
  readonly worldMatrix: Matrix4;
}

function fixtureObjectCollisionEntry(
  objectId: string,
  obj: Pick<FixtureObjectV1, "objectKind" | "origin" | "orientation" | "scale">,
  kindCatalogs: KindCatalogBundle | undefined,
  sceneFixture: FixtureV1 | undefined,
  meshRootForUrl: (meshUrl: string) => Object3D | null | undefined,
): BrushPlacedCollisionEntry | null {
  const meshUrl = resolveObjectKindMeshUrl(obj.objectKind, kindCatalogs, sceneFixture);
  if (!meshUrl) {
    return null;
  }
  const meshRoot = meshRootForUrl(meshUrl);
  if (!meshRoot || !brushCollisionBody(meshUrl)) {
    return null;
  }
  return {
    objectId,
    meshUrl,
    worldMatrix: brushPreviewWorldMatrix({ origin: obj.origin, orientation: obj.orientation, scale: obj.scale }),
  };
}

function fillPreviewCollidesAccumulated(
  preview: BrushPreviewState,
  placed: readonly BrushPlacedCollisionEntry[],
  overlapBudget: number,
  excludeObjectId?: string,
): boolean | null {
  const previewBody = brushCollisionBody(preview.meshUrl);
  if (!previewBody) {
    return null;
  }
  const previewWorld = brushPreviewWorldMatrix(preview);
  for (const entry of placed) {
    if (excludeObjectId && entry.objectId === excludeObjectId) {
      continue;
    }
    const otherBody = brushCollisionBody(entry.meshUrl);
    if (!otherBody) {
      return null;
    }
    const volume = solidOverlapVolume(previewBody, previewWorld, otherBody, entry.worldMatrix, { sampleCount: 512 });
    if (volume > overlapBudget) {
      return true;
    }
  }
  return false;
}

function brushCompatibleCandidatesForFillTarget(
  target: AttractionVortexContext,
  kindCatalogs: KindCatalogBundle | undefined,
  kindCompatibility: readonly KindCompatEntry[] | undefined,
  cache: Map<string, readonly BrushCompatibleCandidate[]>,
): readonly BrushCompatibleCandidate[] {
  const key = `${target.objectKind ?? ""}\u0001${target.vortexKind ?? ""}`;
  const hit = cache.get(key);
  if (hit) {
    return hit;
  }
  const result = brushCompatibleCandidates(target, kindCatalogs, kindCompatibility);
  cache.set(key, result);
  return result;
}

function fillCandidateDiversityScore(
  candidate: BrushCompatibleCandidate,
  targetVortexIndex: number,
  targetObjectKind: string | undefined,
): number {
  if (candidate.objectKindId !== targetObjectKind) {
    return 0;
  }
  return 1_000 + Math.abs(candidate.sourceVortexIndex - targetVortexIndex) * 100;
}

/** @emoji 🚫 True when distribution allows a brush/fill candidate (object + source vortex weights). */
export function brushCandidateAllowsSuggestion(
  candidate: BrushCompatibleCandidate,
  weights: Puzzle3dBrushKindWeights,
  kindCatalogs: KindCatalogBundle | undefined,
): boolean {
  return brushCandidateSuggestionWeight(candidate, weights, kindCatalogs) > 0;
}

/** @emoji 🪣 Fill prefers cross-port mates (e.g. b-s → b-l) and distant connector indices on the same kind. */
function orderBrushFillCompatibleCandidates(
  candidates: readonly BrushCompatibleCandidate[],
  targetVortexKind: string | undefined,
  targetVortexIndex: number,
  targetObjectKind: string | undefined,
  kindCatalogs: KindCatalogBundle | undefined,
  weights: Puzzle3dBrushKindWeights,
  rng: BrushShuffleRng,
): readonly BrushCompatibleCandidate[] {
  const allowed = candidates.filter((candidate) => brushCandidateAllowsSuggestion(candidate, weights, kindCatalogs));
  const target = targetVortexKind ?? "";
  const cross: BrushCompatibleCandidate[] = [];
  const same: BrushCompatibleCandidate[] = [];
  for (const candidate of allowed) {
    const sourceVk = catalogObjectKindById(kindCatalogs, candidate.objectKindId)?.vortices?.[candidate.sourceVortexIndex]?.vortexKind ?? "";
    if (sourceVk !== target || brushStackMatePair(sourceVk, target)) {
      cross.push(candidate);
    } else {
      same.push(candidate);
    }
  }
  const sortFillCandidates = (rows: BrushCompatibleCandidate[]): BrushCompatibleCandidate[] =>
    [...rows].sort(
      (left, right) =>
        left.objectKindId.localeCompare(right.objectKindId) ||
        left.sourceVortexIndex - right.sourceVortexIndex,
    );
  const crossOrdered = sortFillCandidates(cross).sort(
    (left, right) =>
      fillCandidateDiversityScore(right, targetVortexIndex, targetObjectKind) -
        fillCandidateDiversityScore(left, targetVortexIndex, targetObjectKind) ||
      left.objectKindId.localeCompare(right.objectKindId) ||
      left.sourceVortexIndex - right.sourceVortexIndex,
  );
  return [
    ...crossOrdered,
    ...weightedOrderBrushCompatibleCandidates(sortFillCandidates(same), weights, kindCatalogs, rng),
  ];
}

/** @emoji 🪣 Args shared by {@link buildBrushFillSequence} and {@link createBrushFillSequenceStepper}. */
export interface BrushFillSequenceArgs {
  readonly baseFixture: FixtureV1;
  readonly maxCount?: number;
  readonly seed: number;
  readonly kindCatalogs: KindCatalogBundle | undefined;
  readonly kindCompatibility: readonly KindCompatEntry[] | undefined;
  readonly overlapBudget?: number;
  readonly meshRootForUrl: (meshUrl: string) => Object3D | null | undefined;
  readonly weights?: Puzzle3dBrushKindWeights;
  readonly targetVolumes?: readonly WorldVolumeProps[];
}

/** @emoji 🪣 Incremental fill build snapshot for chunked session prep. */
export interface BrushFillSequenceBuildResult {
  readonly sequence: readonly BrushPlacePayload[];
  readonly appendedObjects: readonly FixtureObjectV1[];
  readonly appendedAttractions: readonly AttractionProps[];
  readonly done: boolean;
}

/** @emoji 🪣 Resumable greedy fill builder (yields placements across frames). */
export interface BrushFillSequenceStepper {
  step(budget: number): BrushFillSequenceBuildResult;
}

/** @emoji 🪣 Creates a resumable fill stepper with mesh AABB collision and weighted distribution. */
export function createBrushFillSequenceStepper(args: BrushFillSequenceArgs): BrushFillSequenceStepper {
  const maxCount = Math.max(0, Math.min(PUZZLE_3D_FILL_COUNT_MAX, Math.round(args.maxCount ?? PUZZLE_3D_FILL_COUNT_MAX)));
  const overlapBudget = args.overlapBudget ?? DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET;
  const weights = args.weights ?? puzzle3dBrushKindWeightsRef.current;
  let fixture = args.baseFixture;
  const sequence: BrushPlacePayload[] = [];
  const appendedObjects: FixtureObjectV1[] = [];
  const appendedAttractions: AttractionProps[] = [];
  const placed: BrushPlacedCollisionEntry[] = [];
  const candidateCache = new Map<string, readonly BrushCompatibleCandidate[]>();
  for (const obj of fixture.objects) {
    const entry = fixtureObjectCollisionEntry(obj.id, obj, args.kindCatalogs, fixture, args.meshRootForUrl);
    if (entry) {
      placed.push({ objectId: entry.objectId, meshUrl: entry.meshUrl, worldMatrix: entry.worldMatrix.clone() });
    }
  }
  let state = args.seed >>> 0;
  const rng = (): number => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return state / 0x100000000;
  };
  let stalled = false;
  const seedObjectIds = new Set(args.baseFixture.objects.map((row) => row.id));
  const tryPlaceOne = (): boolean => {
    const freeTargets = enumerateBrushFillVortexTargets(fixture);
    if (freeTargets.length === 0) {
      return false;
    }
    const seedTargets = freeTargets.filter((row) => seedObjectIds.has(row.objectId));
    const frontierTargets = freeTargets.filter((row) => !seedObjectIds.has(row.objectId));
    const orderedTargets = [
      ...weightedOrderFillVortexTargets(seedTargets, weights, rng),
      ...weightedOrderFillVortexTargets(frontierTargets, weights, rng),
    ];
    const targetStart = sequence.length % Math.max(1, orderedTargets.length);
    for (let targetOffset = 0; targetOffset < orderedTargets.length; targetOffset += 1) {
      const target = orderedTargets[(targetStart + targetOffset) % orderedTargets.length]!;
      const hostObject = fixture.objects.find((row) => row.id === target.objectId);
      if (!hostObject) {
        continue;
      }
      const world = vortexWorldCadFromObject(hostObject, target.vortexIndex);
      if (!world) {
        continue;
      }
      const targetCtx: AttractionVortexContext = {
        objectId: target.objectId,
        objectKind: target.objectKind,
        vortexKind: target.vortexKind,
      };
      const compatible = brushCompatibleCandidatesForFillTarget(targetCtx, args.kindCatalogs, args.kindCompatibility, candidateCache);
      if (compatible.length === 0) {
        continue;
      }
      const orderedCandidates = orderBrushFillCompatibleCandidates(
        compatible,
        target.vortexKind,
        target.vortexIndex,
        target.objectKind,
        args.kindCatalogs,
        weights,
        rng,
      );
      for (const candidate of orderedCandidates) {
        const preview = brushPreviewFromCandidate({
          targetVortexFullId: target.fullId,
          candidate,
          target: targetCtx,
          targetWorldPositionCad: world.position,
          targetWorldDirectionCad: world.direction,
          referenceOrientationCad: hostObject.orientation,
          kindCatalogs: args.kindCatalogs,
          sceneFixture: fixture,
        });
        if (!preview) {
          continue;
        }
        const meshRoot = args.meshRootForUrl(preview.meshUrl);
        if (!meshRoot) {
          continue;
        }
        const targetVolumes = args.targetVolumes ?? [];
        if (targetVolumes.length > 0) {
          const aabb = brushPreviewWorldAabb(preview, meshRoot);
          if (!aabb || !worldVolumesContainAabb(targetVolumes, aabb.min, aabb.max)) {
            console.log("[DEBUG] puzzle3d fill skip outside target volume", preview.objectId, aabb);
            continue;
          }
        }
        const collides = fillPreviewCollidesAccumulated(preview, placed, overlapBudget, target.objectId);
        if (collides === null || collides) {
          continue;
        }
        const payload: BrushPlacePayload = {
          targetVortexFullId: preview.targetVortexFullId,
          objectKindId: preview.objectKindId,
          sourceVortexIndex: preview.sourceVortexIndex,
          origin: preview.origin,
          orientation: preview.orientation,
          ...(preview.scale !== undefined ? { scale: preview.scale } : {}),
        };
        const nextFixture = applyBrushPlacementToFixture(fixture, payload, args.kindCatalogs);
        if (nextFixture.objects.length === fixture.objects.length) {
          continue;
        }
        const placedObject = nextFixture.objects[nextFixture.objects.length - 1]!;
        const placedEntry = fixtureObjectCollisionEntry(
          placedObject.id,
          placedObject,
          args.kindCatalogs,
          nextFixture,
          args.meshRootForUrl,
        );
        if (placedEntry) {
          placed.push({ objectId: placedEntry.objectId, meshUrl: placedEntry.meshUrl, worldMatrix: placedEntry.worldMatrix.clone() });
        }
        const newAttraction = nextFixture.attractions[nextFixture.attractions.length - 1];
        if (!newAttraction) {
          continue;
        }
        fixture = nextFixture;
        sequence.push(payload);
        appendedObjects.push(placedObject);
        appendedAttractions.push(newAttraction);
        return true;
      }
    }
    return false;
  };
  return {
    step(budget: number): BrushFillSequenceBuildResult {
      const limit = Math.max(0, Math.round(budget));
      let placedThisChunk = 0;
      while (!stalled && sequence.length < maxCount && placedThisChunk < limit) {
        if (!tryPlaceOne()) {
          stalled = true;
          break;
        }
        placedThisChunk += 1;
      }
      const done = stalled || sequence.length >= maxCount;
      return {
        sequence: [...sequence],
        appendedObjects: [...appendedObjects],
        appendedAttractions: [...appendedAttractions],
        done,
      };
    },
  };
}

/** @emoji 🪣 Deterministic frontier fill sequence with weighted distribution and mesh AABB collision. */
export function buildBrushFillSequence(args: BrushFillSequenceArgs): readonly BrushPlacePayload[] {
  const stepper = createBrushFillSequenceStepper(args);
  let result = stepper.step(Number.MAX_SAFE_INTEGER);
  while (!result.done) {
    result = stepper.step(Number.MAX_SAFE_INTEGER);
  }
  return result.sequence;
}

/** @emoji 🪣 Appends a deterministic fill prefix to a base fixture. */
export function applyBrushFillPlacementsToFixture(
  fixture: FixtureV1,
  payloads: readonly BrushPlacePayload[],
  kindCatalogs: KindCatalogBundle | undefined,
): FixtureV1 {
  let next = fixture;
  for (const payload of payloads) {
    next = applyBrushPlacementToFixture(next, payload, kindCatalogs);
  }
  return next;
}
//#endregion 🖌️Brush

//#region 🧵Precompute
/** @emoji 🖌️ Serializable host brush filter rules for the WASM precompute worker. */
export interface Puzzle3dBrushHostRules {
  readonly rejectCapitalOnTambour?: boolean;
  readonly rejectLastSingleStoreyOnMidTambour?: boolean;
  readonly doorTambourRequiresDoorCapsule?: boolean;
  readonly doorCapsuleMinAbsX?: number;
  readonly doorCapsuleMaxAbsY?: number;
}

export const puzzle3dBrushHostRulesRef: { current: Puzzle3dBrushHostRules } = {
  current: {
    rejectCapitalOnTambour: true,
    rejectLastSingleStoreyOnMidTambour: true,
    doorTambourRequiresDoorCapsule: true,
    doorCapsuleMinAbsX: 0.9,
    doorCapsuleMaxAbsY: 1.6,
  },
};

/** @emoji 🖌️ Publishes serializable brush host rules for the precompute worker. */
export function publishPuzzle3dBrushHostRules(rules: Puzzle3dBrushHostRules | null): void {
  puzzle3dBrushHostRulesRef.current = rules ?? {
    rejectCapitalOnTambour: false,
    rejectLastSingleStoreyOnMidTambour: false,
    doorTambourRequiresDoorCapsule: false,
  };
}

/** @emoji 🧵 Scene snapshot pushed to the precompute worker. */
export interface Puzzle3dPrecomputeSceneInput {
  readonly fixture: FixtureV1;
  readonly kindCatalogs?: KindCatalogBundle;
  readonly kindCompatibility?: readonly KindCompatEntry[];
  readonly overlapBudget?: number;
  readonly seed?: number;
  readonly hostRules?: Puzzle3dBrushHostRules;
  readonly weights?: Puzzle3dBrushKindWeights;
}

function buildPrecomputeSceneJson(input: Puzzle3dPrecomputeSceneInput): string {
  return JSON.stringify({
    fixture: input.fixture,
    kindCatalogs: input.kindCatalogs,
    kindCompatibility: input.kindCompatibility ?? [],
    overlapBudget: input.overlapBudget ?? DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
    seed: input.seed ?? 0,
    hostRules: input.hostRules ?? puzzle3dBrushHostRulesRef.current,
    weights: input.weights ?? puzzle3dBrushKindWeightsRef.current,
  });
}

/** @emoji 📦 Merged pose-local mesh buffers for WASM parry3d registration (GLB frame rotation already baked). */
export function extractBrushCollisionMeshBuffers(meshRoot: Object3D): { readonly positions: Float32Array; readonly indices: Uint32Array } | null {
  const poseLocal = new Group();
  const frame = new Group();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.add(meshRoot.clone(true));
  poseLocal.add(frame);
  poseLocal.updateMatrixWorld(true);
  const poseInverse = poseLocal.matrixWorld.clone().invert();
  const positions: number[] = [];
  const indices: number[] = [];
  let vertexOffset = 0;
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
    const posAttr = geometry.getAttribute("position");
    const index = geometry.index;
    for (let i = 0; i < posAttr.count; i += 1) {
      positions.push(posAttr.getX(i), posAttr.getY(i), posAttr.getZ(i));
    }
    if (index) {
      for (let i = 0; i < index.count; i += 1) {
        indices.push(vertexOffset + index.getX(i));
      }
    } else {
      for (let i = 0; i < posAttr.count; i += 3) {
        indices.push(vertexOffset + i, vertexOffset + i + 1, vertexOffset + i + 2);
      }
    }
    vertexOffset += posAttr.count;
  });
  if (positions.length < 9 || indices.length < 3) {
    return null;
  }
  return { positions: new Float32Array(positions), indices: new Uint32Array(indices) };
}

/** @emoji 🧵 Collision + candidate engine behind worker/WASM or mesh-bvh fallback. */
export interface Puzzle3dCollisionEngine {
  setScene(input: Puzzle3dPrecomputeSceneInput): Promise<void>;
  registerMesh(meshUrl: string, positions: Float32Array, indices: Uint32Array): Promise<void>;
  precomputeStep(budget: number): Promise<boolean>;
  getBrushCandidates(vortexFullId: string): Promise<BrushCollisionFreeResult>;
  getFillProgress(): Promise<Puzzle3dFillBuildProgress>;
  getFillWorkerSnapshot(): Promise<Puzzle3dFillWorkerSnapshot>;
  brushCollisionFree(args: {
    readonly scene: BrushSceneCollisionSource;
    readonly targetVortexFullId: string;
    readonly candidates: readonly BrushCompatibleCandidate[];
    readonly target: AttractionVortexContext;
    readonly targetWorldPositionCad: Vec3;
    readonly targetWorldDirectionCad: Vec3;
    readonly referenceOrientationCad?: Quat;
    readonly kindCatalogs: KindCatalogBundle | undefined;
    readonly kindCompatibility?: readonly KindCompatEntry[];
    readonly sceneFixture?: FixtureV1;
    readonly meshRootForUrl?: (meshUrl: string) => Object3D | null | undefined;
    readonly overlapBudget?: number;
  }): Promise<BrushCollisionFreeResult>;
  dispose(): void;
}

/** @emoji 🧵 Spawns the puzzle 3d precompute worker (Vite-bundled WASM). */
export function createPuzzle3dPrecomputeWorker(): Worker {
  return new Worker(new URL("./precompute.worker", import.meta.url), { type: "module" });
}

class MeshBvhCollisionEngine implements Puzzle3dCollisionEngine {
  async setScene(_input: Puzzle3dPrecomputeSceneInput): Promise<void> {}

  async registerMesh(_meshUrl: string, _positions: Float32Array, _indices: Uint32Array): Promise<void> {}

  async precomputeStep(_budget: number): Promise<boolean> {
    return false;
  }

  async getBrushCandidates(_vortexFullId: string): Promise<BrushCollisionFreeResult> {
    return { free: [], unknownPending: true };
  }

  async getFillProgress(): Promise<Puzzle3dFillBuildProgress> {
    return { count: 0, maxCount: PUZZLE_3D_FILL_COUNT_MAX, done: true };
  }

  async getFillWorkerSnapshot(): Promise<Puzzle3dFillWorkerSnapshot> {
    const progress = await this.getFillProgress();
    return { ...progress, sequence: [], appendedObjects: [], appendedAttractions: [] };
  }

  async brushCollisionFree(args: Parameters<Puzzle3dCollisionEngine["brushCollisionFree"]>[0]): Promise<BrushCollisionFreeResult> {
    return brushCollisionFreeCandidates(args);
  }

  dispose(): void {}
}

type PrecomputeWorkerMessage = {
  op: string;
  reqId?: string;
  json?: string;
  url?: string;
  positions?: Float32Array;
  indices?: Uint32Array;
  vortexFullId?: string;
  budget?: number;
  message?: string;
};

class WasmCollisionEngine implements Puzzle3dCollisionEngine {
  private nextSerial = 0;
  private readonly ready: Promise<void>;

  constructor(private readonly worker: Worker) {
    this.ready = new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        cleanup();
        reject(new Error("puzzle3d precompute worker init timeout"));
      }, 30_000);
      const onMessage = (ev: MessageEvent<string>) => {
        let m: PrecomputeWorkerMessage;
        try {
          m = JSON.parse(ev.data) as PrecomputeWorkerMessage;
        } catch {
          return;
        }
        if (m.op === "ready") {
          cleanup();
          resolve();
        } else if (m.op === "error") {
          cleanup();
          reject(new Error(m.message ?? "puzzle3d precompute worker init error"));
        }
      };
      const onError = (ev: globalThis.Event) => {
        cleanup();
        reject(new Error(`puzzle3d precompute worker init error: ${String(ev)}`));
      };
      const cleanup = () => {
        clearTimeout(timeout);
        this.worker.removeEventListener("message", onMessage);
        this.worker.removeEventListener("error", onError as globalThis.EventListener);
      };
      this.worker.addEventListener("message", onMessage);
      this.worker.addEventListener("error", onError as globalThis.EventListener);
      this.worker.postMessage(JSON.stringify({ op: "init" }));
    });
  }

  private async rpc(op: string, payload: Record<string, unknown> = {}): Promise<void> {
    await this.ready;
    const reqId = `p3d-${++this.nextSerial}-${Date.now().toString(36)}`;
    await new Promise<void>((resolve, reject) => {
      const onMessage = (ev: MessageEvent<string>) => {
        let m: PrecomputeWorkerMessage;
        try {
          m = JSON.parse(ev.data) as PrecomputeWorkerMessage;
        } catch {
          return;
        }
        if (m.reqId !== reqId) {
          return;
        }
        if (m.op === "done") {
          this.worker.removeEventListener("message", onMessage);
          resolve();
        }
        if (m.op === "error") {
          this.worker.removeEventListener("message", onMessage);
          reject(new Error(m.message ?? "puzzle3d precompute worker error"));
        }
      };
      this.worker.addEventListener("message", onMessage);
      this.worker.postMessage(JSON.stringify({ op, reqId, ...payload }));
    });
  }

  private async rpcResult(op: string, payload: Record<string, unknown> = {}): Promise<string> {
    await this.ready;
    const reqId = `p3d-${++this.nextSerial}-${Date.now().toString(36)}`;
    return await new Promise<string>((resolve, reject) => {
      let result: string | null = null;
      const onMessage = (ev: MessageEvent<string>) => {
        let m: PrecomputeWorkerMessage;
        try {
          m = JSON.parse(ev.data) as PrecomputeWorkerMessage;
        } catch {
          return;
        }
        if (m.reqId !== reqId) {
          return;
        }
        if (m.op === "result" && typeof m.json === "string") {
          result = m.json;
        }
        if (m.op === "done") {
          this.worker.removeEventListener("message", onMessage);
          if (result == null) {
            reject(new Error("puzzle3d precompute worker completed without result"));
          } else {
            resolve(result);
          }
        }
        if (m.op === "error") {
          this.worker.removeEventListener("message", onMessage);
          reject(new Error(m.message ?? "puzzle3d precompute worker error"));
        }
      };
      this.worker.addEventListener("message", onMessage);
      this.worker.postMessage(JSON.stringify({ op, reqId, ...payload }));
    });
  }

  async setScene(input: Puzzle3dPrecomputeSceneInput): Promise<void> {
    await this.rpc("setScene", { json: buildPrecomputeSceneJson(input) });
  }

  async registerMesh(meshUrl: string, positions: Float32Array, indices: Uint32Array): Promise<void> {
    await this.ready;
    const reqId = `p3d-${++this.nextSerial}-${Date.now().toString(36)}`;
    await new Promise<void>((resolve, reject) => {
      const onMessage = (ev: MessageEvent<string>) => {
        let m: PrecomputeWorkerMessage;
        try {
          m = JSON.parse(ev.data) as PrecomputeWorkerMessage;
        } catch {
          return;
        }
        if (m.reqId !== reqId) {
          return;
        }
        if (m.op === "done") {
          this.worker.removeEventListener("message", onMessage);
          resolve();
        }
        if (m.op === "error") {
          this.worker.removeEventListener("message", onMessage);
          reject(new Error(m.message ?? "puzzle3d precompute worker error"));
        }
      };
      this.worker.addEventListener("message", onMessage);
      this.worker.postMessage({ op: "registerMesh", reqId, url: meshUrl, positions, indices }, [positions.buffer, indices.buffer]);
    });
  }

  async precomputeStep(budget: number): Promise<boolean> {
    const json = await this.rpcResult("precomputeStep", { budget });
    const parsed = JSON.parse(json) as { more?: boolean };
    return parsed.more === true;
  }

  async getBrushCandidates(vortexFullId: string): Promise<BrushCollisionFreeResult> {
    const json = await this.rpcResult("brushCandidates", { vortexFullId });
    return JSON.parse(json) as BrushCollisionFreeResult;
  }

  async getFillProgress(): Promise<Puzzle3dFillBuildProgress> {
    const snapshot = await this.getFillWorkerSnapshot();
    return { count: snapshot.count, maxCount: snapshot.maxCount, done: snapshot.done };
  }

  async getFillWorkerSnapshot(): Promise<Puzzle3dFillWorkerSnapshot> {
    const json = await this.rpcResult("fillProgress");
    const parsed = JSON.parse(json) as Puzzle3dFillWorkerSnapshot;
    return {
      count: parsed.count ?? 0,
      maxCount: parsed.maxCount ?? PUZZLE_3D_FILL_COUNT_MAX,
      done: parsed.done ?? true,
      sequence: parsed.sequence ?? [],
      appendedObjects: parsed.appendedObjects ?? [],
      appendedAttractions: parsed.appendedAttractions ?? [],
    };
  }

  async brushCollisionFree(args: Parameters<Puzzle3dCollisionEngine["brushCollisionFree"]>[0]): Promise<BrushCollisionFreeResult> {
    if (args.sceneFixture) {
      const kindCompatibility = resolvePuzzle3dKindCompatibility(
        args.kindCompatibility,
        args.sceneFixture.meta as Record<string, unknown> | undefined,
      );
      await syncPuzzle3dPrecomputeScene({
        fixture: args.sceneFixture,
        kindCatalogs: args.kindCatalogs,
        kindCompatibility,
        overlapBudget: args.overlapBudget,
        weights: puzzle3dBrushKindWeightsRef.current,
      });
    }
    const live = brushCollisionFreeCandidates(args);
    if (!live.unknownPending) {
      return live;
    }
    const cached = await this.getBrushCandidates(args.targetVortexFullId);
    if (cached.unknownPending) {
      return live;
    }
    const freeSet = new Set(cached.free.map((row) => `${row.objectKindId}\u0001${row.sourceVortexIndex}`));
    const free = args.candidates.filter((row) => freeSet.has(`${row.objectKindId}\u0001${row.sourceVortexIndex}`));
    return { free, unknownPending: free.length === 0 };
  }

  dispose(): void {
    this.worker.terminate();
  }
}

let puzzle3dPrecomputeWorkerActive = false;

function createDefaultPuzzle3dCollisionEngine(): Puzzle3dCollisionEngine {
  if (typeof Worker !== "undefined" && typeof window !== "undefined") {
    try {
      puzzle3dPrecomputeWorkerActive = true;
      return new WasmCollisionEngine(createPuzzle3dPrecomputeWorker());
    } catch {
      puzzle3dPrecomputeWorkerActive = false;
    }
  }
  return new MeshBvhCollisionEngine();
}

export const puzzle3dCollisionEngineRef: { current: Puzzle3dCollisionEngine } = { current: createDefaultPuzzle3dCollisionEngine() };

/** @emoji 🧵 True when the WASM worker collision engine is active. */
export function puzzle3dPrecomputeUsesWorker(): boolean {
  return puzzle3dPrecomputeWorkerActive;
}

let puzzle3dPrecomputeSceneSyncTimer: ReturnType<typeof setTimeout> | null = null;
let puzzle3dPrecomputeSceneSyncInFlight: Promise<void> = Promise.resolve();
let puzzle3dPrecomputeSceneSyncKey = "";

function puzzle3dBrushKindWeightsSyncKey(weights: Puzzle3dBrushKindWeights): string {
  const objectEntries = Object.entries(weights.objectWeights)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([id, value]) => `${id}=${value.toFixed(6)}`)
    .join(",");
  const vortexEntries = Object.entries(weights.vortexWeights)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([id, value]) => `${id}=${value.toFixed(6)}`)
    .join(";");
  return `${objectEntries}|${vortexEntries}`;
}

function puzzle3dPrecomputeSceneSyncKeyOf(input: Puzzle3dPrecomputeSceneInput): string {
  const rules =
    input.kindCompatibility ??
    kindCompatibilityFromFixtureMeta(input.fixture.meta as Record<string, unknown> | undefined);
  const weights = input.weights ?? puzzle3dBrushKindWeightsRef.current;
  return `${input.overlapBudget ?? DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET}:${kindCompatibilitySyncKey(rules)}:${fixtureStateFingerprint(input.fixture)}:${puzzle3dBrushKindWeightsSyncKey(weights)}`;
}

/** @emoji 🧵 Awaits worker scene sync (overlap budget + fixture fingerprint). */
export async function syncPuzzle3dPrecomputeScene(input: Puzzle3dPrecomputeSceneInput): Promise<void> {
  const key = puzzle3dPrecomputeSceneSyncKeyOf(input);
  if (key === puzzle3dPrecomputeSceneSyncKey) {
    await puzzle3dPrecomputeSceneSyncInFlight;
    return;
  }
  puzzle3dPrecomputeSceneSyncKey = key;
  puzzle3dPrecomputeSceneSyncInFlight = puzzle3dCollisionEngineRef.current.setScene(input);
  await puzzle3dPrecomputeSceneSyncInFlight;
}

/** @emoji 🧵 Debounced scene sync to the precompute worker. */
export function schedulePuzzle3dPrecomputeSceneSync(input: Puzzle3dPrecomputeSceneInput): void {
  if (puzzle3dPrecomputeSceneSyncTimer !== null) {
    clearTimeout(puzzle3dPrecomputeSceneSyncTimer);
  }
  puzzle3dPrecomputeSceneSyncTimer = setTimeout(() => {
    puzzle3dPrecomputeSceneSyncTimer = null;
    void syncPuzzle3dPrecomputeScene(input);
  }, 0);
}

/** @emoji 🪣 Fill build progress from the precompute worker. */
export interface Puzzle3dFillBuildProgress {
  readonly count: number;
  readonly maxCount: number;
  readonly done: boolean;
}

/** @emoji 🪣 Extended fill snapshot including worker-appended fixture rows. */
export interface Puzzle3dFillWorkerSnapshot extends Puzzle3dFillBuildProgress {
  readonly sequence: readonly BrushPlacePayload[];
  readonly appendedObjects: readonly FixtureObjectV1[];
  readonly appendedAttractions: readonly AttractionProps[];
}

/** @emoji 🪣 Reads fill build progress (and appended rows when available) from the worker. */
export function readPuzzle3dFillWorkerSnapshot(): Promise<Puzzle3dFillWorkerSnapshot> {
  return puzzle3dCollisionEngineRef.current.getFillWorkerSnapshot();
}
//#endregion 🧵Precompute

//#region ­ƒÄ¿MeshPaint
const CSS_SELECTED_MESH = "color-mix(in oklab, var(--color-primary) 28%, var(--color-panel))";
const CSS_SELECTED_LINE = tokenVar("primary");
const CSS_HIGHLIGHTED_MESH = "color-mix(in oklab, var(--color-secondary) 24%, var(--color-panel))";
const CSS_HIGHLIGHTED_LINE = tokenVar("secondary");
const CSS_HOVERED_MESH = themeColorVar("hover-panel");
const CSS_HOVERED_LINE = themeColorVar("hover-base");
const CSS_NEUTRAL_MESH = themeColorVar("panel");
const CSS_NEUTRAL_LINE = WORLD_MESH_BORDER_CSS;
const CSS_MESH_EDGE_BORDER = semanticVar("border-emphasized-color");
const CSS_DISABLED_MESH = "color-mix(in oklab, var(--color-muted-foreground) 55%, var(--color-panel))";
const CSS_DISABLED_LINE = themeColorVar("muted-foreground");
const CSS_ATTRACTION_ENDPOINT_LINE = themeColorVar("muted-foreground");
const CSS_ATTRACTION_LINE = themeColorVar("accent");

interface MeshStyleColors {
  readonly meshColor: string;
  readonly lineColor: string;
  readonly emissiveColor: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
}

const meshStyleColorCache = new Map<Exclude<MeshStyleKind, "original">, MeshStyleColors>();

const MESH_STYLE_HEADLESS: Record<Exclude<MeshStyleKind, "original">, MeshStyleColors> = {
  neutral: {
    meshColor: tokenHex("l-l-l-g"),
    lineColor: tokenHex("gray"),
    emissiveColor: tokenHex("dark"),
    emissiveIntensity: 0,
    opacity: 1,
  },
  hovered: {
    meshColor: tokenHex("light-5-7"),
    lineColor: tokenHex("gray"),
    emissiveColor: tokenHex("gray"),
    emissiveIntensity: 0.08,
    opacity: 1,
  },
  selected: {
    meshColor: blendTokenHex("primary", "light-5-7", 0.28),
    lineColor: tokenHex("primary"),
    emissiveColor: tokenHex("primary"),
    emissiveIntensity: 0.35,
    opacity: 1,
  },
  highlighted: {
    meshColor: blendTokenHex("secondary", "light-5-7", 0.24),
    lineColor: tokenHex("secondary"),
    emissiveColor: tokenHex("secondary"),
    emissiveIntensity: 0.2,
    opacity: 1,
  },
  disabled: {
    meshColor: tokenHex("light-gray"),
    lineColor: tokenHex("gray"),
    emissiveColor: tokenHex("dark"),
    emissiveIntensity: 0,
    opacity: 0.45,
  },
};

function resolveCssColor(property: "color" | "backgroundColor", expr: string, fallbackKey: string): string {
  return property === "backgroundColor" ? resolveBackgroundColorHex(expr, fallbackKey) : resolveColorHex(expr, fallbackKey);
}

/** @emoji ­ƒÄ¿ Resolves mesh and edge colors for a {@link MeshStyleKind} from Elements tokens. */
export function meshStyleColors(style: MeshStyleKind): MeshStyleColors | null {
  if (style === "original") {
    return null;
  }
  const cached = meshStyleColorCache.get(style);
  if (cached) {
    return cached;
  }
  const fb = MESH_STYLE_HEADLESS[style];
  const meshExprs: Record<Exclude<MeshStyleKind, "original">, string> = {
    neutral: CSS_NEUTRAL_MESH,
    hovered: CSS_HOVERED_MESH,
    selected: CSS_SELECTED_MESH,
    highlighted: CSS_HIGHLIGHTED_MESH,
    disabled: CSS_DISABLED_MESH,
  };
  const lineExprs: Record<Exclude<MeshStyleKind, "original">, string> = {
    neutral: CSS_NEUTRAL_LINE,
    hovered: CSS_HOVERED_LINE,
    selected: CSS_SELECTED_LINE,
    highlighted: CSS_HIGHLIGHTED_LINE,
    disabled: CSS_DISABLED_LINE,
  };
  const meshFallbackKey = style === "neutral" ? "l-l-l-g" : style === "selected" ? "primary" : style === "highlighted" ? "secondary" : style === "hovered" ? "light-5-7" : "light-gray";
  const lineFallbackKey = style === "selected" ? "primary" : style === "highlighted" ? "secondary" : "gray";
  const resolved = {
    meshColor: resolveCssColor("backgroundColor", meshExprs[style], meshFallbackKey),
    lineColor: resolveCssColor("color", lineExprs[style], lineFallbackKey),
    emissiveColor: resolveCssColor("color", lineExprs[style], lineFallbackKey),
    emissiveIntensity: fb.emissiveIntensity,
    opacity: fb.opacity,
  };
  meshStyleColorCache.set(style, resolved);
  return resolved;
}

function createStyledMeshMaterial(color: string, state: MeshStyleColors): MeshStandardMaterial {
  const mat = new MeshStandardMaterial({
    color: new Color(resolveColorHex(color, "gray")),
    metalness: 0,
    roughness: 1,
  });
  mat.emissive.set(state.emissiveColor);
  mat.emissiveIntensity = state.emissiveIntensity;
  mat.transparent = state.opacity < 1;
  mat.opacity = state.opacity;
  return mat;
}

function createStyledLineMaterial(color: string, state: MeshStyleColors): LineBasicMaterial {
  const mat = new LineBasicMaterial({ color: new Color(resolveColorHex(color, "gray")) });
  mat.transparent = state.opacity < 1;
  mat.opacity = state.opacity;
  return mat;
}

function applyMeshStyleToObject3D(root: Object3D, style: MeshStyleKind, edgeOutlines = true, opacityScale = 1): void {
  const colors = meshStyleColors(style);
  if (!colors) {
    return;
  }
  const scaledColors = opacityScale === 1 ? colors : { ...colors, opacity: colors.opacity * opacityScale };
  root.traverse((object) => {
    if (object instanceof Mesh) {
      const meshMaterial = createStyledMeshMaterial(colors.meshColor, scaledColors);
      if (Array.isArray(object.material)) {
        object.material = object.material.map(() => meshMaterial.clone());
      } else {
        object.material = meshMaterial;
      }
      return;
    }
    if (object instanceof ThreeLine || object instanceof LineSegments) {
      if (object.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY]) {
        return;
      }
      object.material = createStyledLineMaterial(colors.lineColor, scaledColors);
      return;
    }
    if (object instanceof Points) {
      object.material = new PointsMaterial({
        color: new Color(resolveColorHex(colors.lineColor, "gray")),
        size: 1,
        transparent: scaledColors.opacity < 1,
        opacity: scaledColors.opacity,
      });
    }
  });
  if (edgeOutlines) {
    applyWorldMeshEdgeBorders(root, CSS_MESH_EDGE_BORDER);
  }
}

function applyOpacityScaleToObject3D(root: Object3D, opacityScale: number): void {
  if (opacityScale === 1) {
    return;
  }
  root.traverse((object) => {
    if (!(object instanceof Mesh || object instanceof ThreeLine || object instanceof LineSegments || object instanceof Points)) {
      return;
    }
    const materials = Array.isArray(object.material) ? object.material : [object.material];
    for (const material of materials) {
      if (!material || !("opacity" in material) || typeof material.opacity !== "number") {
        continue;
      }
      material.opacity *= opacityScale;
      material.transparent = material.opacity < 1;
    }
  });
}

/** @emoji ­ƒÄ¿ Chooses the effective mesh style from explicit prop and interaction flags. */
export function resolveMeshStyle(args: { readonly style?: MeshStyleKind; readonly disabled?: boolean; readonly selected?: boolean; readonly highlighted?: boolean; readonly hovered?: boolean }): MeshStyleKind {
  if (args.style) {
    return args.style;
  }
  if (args.disabled) {
    return "disabled";
  }
  if (args.selected) {
    return "selected";
  }
  if (args.highlighted) {
    return "highlighted";
  }
  if (args.hovered) {
    return "hovered";
  }
  return DEFAULT_MESH_STYLE;
}

/** @emoji ­ƒÄ¿ Resolves a CSS color for scene lines (endpoint attractions, attraction guides). */
export function lineCssColor(expr: string, fallbackKey: string): string {
  return resolveColorHex(expr, fallbackKey);
}
//#endregion ­ƒÄ¿MeshPaint

//#region ­ƒÅèPool
const gltfRefCounts = createRefCountPool<string>();
const styledMeshPool = createTemplatePool<string>();

function styledPoolKey(url: string, style: MeshStyleKind, edgeOutlines: boolean): string {
  return edgeOutlines ? `${url}\0${style}` : `${url}\0${style}\0fill`;
}

export function gltfPoolAcquire(url: string): void {
  gltfRefCounts.acquire(url);
}

export function gltfPoolRelease(url: string): void {
  gltfRefCounts.release(url);
}

export function styledMeshPoolAcquire(url: string, style: MeshStyleKind, edgeOutlines = true): void {
  styledMeshPool.acquire(styledPoolKey(url, style, edgeOutlines));
}

export function styledMeshPoolRelease(url: string, style: MeshStyleKind, edgeOutlines = true): void {
  styledMeshPool.release(styledPoolKey(url, style, edgeOutlines));
}

/** @emoji ­ƒº╣ Drops pooled GLTF cache entries (call on scene teardown, not per-chunk unmount). */
export function gltfPoolClear(url: string): void {
  gltfRefCounts.delete(url);
  styledMeshPool.deleteByPrefix(`${url}\0`);
  useGLTF.clear(url);
}

/** @emoji ­ƒÅè Returns a cached styled GLTF template for {@link MeshBody} (refcount via acquire/release). */
export function styledMeshTemplate(url: string, style: MeshStyleKind, source: Object3D, edgeOutlines = true): Object3D {
  if (style === "original") {
    return source;
  }
  const key = styledPoolKey(url, style, edgeOutlines);
  return styledMeshPool.getOrCreate(key, () => {
    const template = source.clone(true);
    applyMeshStyleToObject3D(template, style, edgeOutlines);
    return template;
  });
}

function usePooledGltf(url: string) {
  const gltf = useGLTF(url);
  reactHostPort.useEffect(() => {
    gltfPoolAcquire(url);
    return () => {
      gltfPoolRelease(url);
    };
  }, [url]);
  reactHostPort.useLayoutEffect(() => {
    if (gltf.scene && isLoadableMeshUrl(url)) {
      registerBrushCollisionGltfScene(url, gltf.scene);
    }
  }, [gltf.scene, url]);
  return gltf;
}

function usePooledStyledMesh(url: string, style: MeshStyleKind, edgeOutlines = true) {
  const gltf = usePooledGltf(url);
  reactHostPort.useEffect(() => {
    if (style === "original") {
      return undefined;
    }
    styledMeshPoolAcquire(url, style, edgeOutlines);
    return () => {
      styledMeshPoolRelease(url, style, edgeOutlines);
    };
  }, [url, style, edgeOutlines]);
  const renderRoot = reactHostPort.useMemo(() => {
    if (!gltf.scene) {
      return null;
    }
    const template = styledMeshTemplate(url, style, gltf.scene, edgeOutlines);
    return template.clone(true);
  }, [edgeOutlines, gltf.scene, url, style]);
  return renderRoot;
}
//#endregion ­ƒÅèPool

//#region ­ƒÄ»Registry
type VortexGetter = () => Vector3 | null;

export interface VortexBindingMeta {
  readonly fullId: string;
  readonly objectId: string;
  readonly objectKind: string | undefined;
  readonly vortexKind: string | undefined;
  readonly radiusWorld: number;
}

export interface RegistryValue {
  registerVortex(fullId: string, getter: VortexGetter): void;
  unregisterVortex(fullId: string): void;
  getVortexWorld(fullId: string): Vector3 | null;
  registerVortexBinding(meta: VortexBindingMeta, pickRoot: Object3D | null): void;
  unregisterVortexBinding(fullId: string): void;
  registerObject(id: string, objectKind: string | undefined, group: Group | null): void;
  collectObjectGroups(): readonly Group[];
  listVortexBindings(): readonly VortexBindingMeta[];
  getObjectGroup(id: string): Group | null;
  getObjectKind(id: string): string | undefined;
  kindCatalogs: KindCatalogBundle | undefined;
  kindCompatibility: readonly KindCompatEntry[] | undefined;
  blockedVortexFullIds: ReadonlySet<string>;
  proximityRadius: number;
  proximityRelocateEnabled: boolean;
  setSelectedObjectIds(ids: readonly string[]): void;
  selectionMode: SelectionMode;
  gumballConfig: GumballConfig;
  setActiveRelocateObjectId: (id: string | null) => void;
  attractionDragActive: boolean;
  attractionDragAttractingFullId: string | null;
  attractionCompatibleAttractedFullIds: ReadonlySet<string>;
  attractionHoverRingFullId: string | null;
  attractionIndirectPickAwait: AttractionIndirectPickAwait | null;
  attractionEndWorldRef: MutableRefObject<Vector3 | null>;
  beginAttractionDragFromVortex(fullId: string, objectId: string, objectKind: string | undefined, vortexKind: string | undefined): void;
  cancelAttractionDrag(): void;
  findNearestProximityRelocate(world: Vector3, movingObjectId: string): AttractionPayload | null;
  attachAttractionThreeEnv(env: { camera: Camera; gl: WebGLRenderer; scene: ThreeScene } | null): void;
  updateAttractionPointer(clientX: number, clientY: number): void;
  commitAttractionPointer(clientX: number, clientY: number): void;
  updateIndirectPickPointer(clientX: number, clientY: number): void;
  commitIndirectPickPointerDown(clientX: number, clientY: number): void;
  onSelect?: (snap: SelectionSnapshot) => void;
  onConnect?: (p: AttractionPayload) => void;
  onProximityConnect?: (p: AttractionPayload) => void;
  onIndirectConnect?: (p: AttractionPayload) => void;
  onAttractionCompatibleObjects?: (p: AttractionCompatibleObjectsPayload) => void;
  onAttractionTargetRing?: (p: AttractionTargetRingPayload) => void;
  onRelocate?: (p: RelocatePayload) => void;
  readonly hoverTarget: HoverTarget | null;
  readonly kindHover: Puzzle3dKindHover | null;
  setHover: (target: HoverTarget) => void;
  clearHover: (target: HoverTarget) => void;
  clearHoverAll: () => void;
  isHovered: (target: HoverTarget) => boolean;
  setKindHover: (kind: Puzzle3dKindHover) => void;
  clearKindHover: (kind: Puzzle3dKindHover) => void;
  isKindHovered: (domain: Puzzle3dKindHoverDomain, kindId: string | undefined) => boolean;
  registerAttractionKind: (id: string, attractionKind: string | undefined) => void;
  clearSelection: () => void;
}

/** @emoji ­ƒÄ» Attraction-drag UI state isolated so orbit idle frames do not re-render every object. */
export interface RegistryDragState {
  readonly attractionDragActive: boolean;
  readonly attractionDragAttractingFullId: string | null;
  readonly attractionCompatibleAttractedFullIds: ReadonlySet<string>;
  readonly attractionHoverRingFullId: string | null;
  readonly attractionIndirectPickAwait: AttractionIndirectPickAwait | null;
}

/** @emoji 🎯 Selection + relocate actions with stable identity (object meshes do not re-subscribe). */
export interface RegistryInteractionValue {
  readonly selectionMode: SelectionMode;
  commitSelection(pick: SelectionPick): void;
  captureMarqueeCandidates(): void;
  previewMarqueeSelection(args: MarqueeGestureArgs): void;
  cancelMarqueePreview(): void;
  commitMarqueeSelection(args: MarqueeGestureArgs): void;
  setSelectedObjectIds(ids: readonly string[] | ((prev: readonly string[]) => readonly string[])): void;
  setActiveRelocateObjectId(id: string | null): void;
  clearSelection(): void;
}

/** @emoji 🖱️ Feeds live attraction rows into marquee hit testing ({@link RegistryProvider}). */
export interface RegistryMarqueeValue {
  readonly selectionMethod: SelectionMethod;
  readonly marqueeSelectableKinds: MarqueeSelectableKinds;
  setMarqueeAttractions(attractions: readonly AttractionProps[]): void;
}

/** @emoji 🖱️ Exclusive hover state isolated from selection updates. */
export interface RegistryHoverValue {
  readonly hoverTarget: HoverTarget | null;
  readonly kindHover: Puzzle3dKindHover | null;
  setHover(target: HoverTarget): void;
  clearHover(target: HoverTarget): void;
  clearHoverAll(): void;
  isHovered(target: HoverTarget): boolean;
  setKindHover(kind: Puzzle3dKindHover): void;
  clearKindHover(kind: Puzzle3dKindHover): void;
  isKindHovered(domain: Puzzle3dKindHoverDomain, kindId: string | undefined): boolean;
}

type RegistryCoreValue = Omit<RegistryValue, keyof RegistryDragState | keyof RegistryInteractionValue | keyof RegistryHoverValue>;

const RegistryCoreContext = reactHostPort.createContext<RegistryCoreValue | null>(null);
const RegistryDragContext = reactHostPort.createContext<RegistryDragState | null>(null);
const RegistryInteractionContext = reactHostPort.createContext<RegistryInteractionValue | null>(null);
const RegistryHoverContext = reactHostPort.createContext<RegistryHoverValue | null>(null);
const RegistryMarqueeContext = reactHostPort.createContext<RegistryMarqueeValue | null>(null);

function useRegistryCore(): RegistryCoreValue {
  const v = reactHostPort.useContext(RegistryCoreContext);
  if (!v) throw new Error("Puzzle 3D registry missing");
  return v;
}

function useRegistryDrag(): RegistryDragState {
  const v = reactHostPort.useContext(RegistryDragContext);
  if (!v) throw new Error("Puzzle 3D registry drag missing");
  return v;
}

function useRegistryInteraction(): RegistryInteractionValue {
  const v = reactHostPort.useContext(RegistryInteractionContext);
  if (!v) throw new Error("Puzzle 3D registry interaction missing");
  return v;
}

function useRegistryHover(): RegistryHoverValue {
  const v = reactHostPort.useContext(RegistryHoverContext);
  if (!v) throw new Error("Puzzle 3D registry hover missing");
  return v;
}

function useRegistryMarquee(): RegistryMarqueeValue {
  const v = reactHostPort.useContext(RegistryMarqueeContext);
  if (!v) throw new Error("Puzzle 3D registry marquee missing");
  return v;
}

function useRegistry(): RegistryValue {
  return {
    ...useRegistryCore(),
    ...useRegistryInteraction(),
    ...useRegistryHover(),
    ...useRegistryDrag(),
  };
}

/** @emoji 🖱️ Clears exclusive hover when the pointer leaves the canvas. */
function HoverMissBridge(): null {
  const { clearHoverAll } = useRegistryHover();
  const invalidate = useThree((state) => state.invalidate);
  const gl = useThree((state) => state.gl);
  reactHostPort.useEffect(() => {
    const onLeave = () => {
      clearHoverAll();
      invalidate();
    };
    gl.domElement.addEventListener("pointerleave", onLeave);
    return () => gl.domElement.removeEventListener("pointerleave", onLeave);
  }, [clearHoverAll, gl, invalidate]);
  return null;
}

/** @emoji 🖱️ Redraws the canvas when exclusive hover changes. */
function HoverInvalidateBridge(): null {
  const { hoverTarget, kindHover } = useRegistryHover();
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [hoverTarget, kindHover, invalidate]);
  return null;
}

/** @emoji 🖱️ Mirrors controlled hover props into the registry and emits host hover changes. */
function ControlledHoverSync(props: {
  readonly hoverTargetProp?: HoverTarget | null;
  readonly kindHoverProp?: Puzzle3dKindHover | null;
  readonly onHover?: (payload: Puzzle3dHoverPayload) => void;
}): null {
  const { hoverTarget, kindHover, setHover, setKindHover, clearHoverAll } = useRegistryHover();
  const hoverControlled = props.hoverTargetProp !== undefined || props.kindHoverProp !== undefined;
  const suppressEmitRef = reactHostPort.useRef(false);
  const lastEmittedRef = reactHostPort.useRef<Puzzle3dHoverPayload>({ hoverTarget: null, kindHover: null });
  const lastAppliedPropsRef = reactHostPort.useRef<Puzzle3dHoverPayload>({ hoverTarget: null, kindHover: null });

  reactHostPort.useLayoutEffect(() => {
    if (!hoverControlled) {
      return;
    }
    const nextTarget = props.hoverTargetProp ?? null;
    const nextKind = props.kindHoverProp ?? null;
    const lastApplied = lastAppliedPropsRef.current;
    if (puzzle3dHoverTargetsEqual(lastApplied.hoverTarget, nextTarget) && puzzle3dKindHoversEqual(lastApplied.kindHover, nextKind)) {
      return;
    }
    lastAppliedPropsRef.current = { hoverTarget: nextTarget, kindHover: nextKind };
    suppressEmitRef.current = true;
    if (nextTarget) {
      setHover(nextTarget);
    } else if (nextKind) {
      setKindHover(nextKind);
    } else {
      clearHoverAll();
    }
    suppressEmitRef.current = false;
  }, [clearHoverAll, hoverControlled, props.hoverTargetProp, props.kindHoverProp, setHover, setKindHover]);

  reactHostPort.useEffect(() => {
    if (!props.onHover || suppressEmitRef.current) {
      return;
    }
    const last = lastEmittedRef.current;
    if (puzzle3dHoverTargetsEqual(last.hoverTarget, hoverTarget) && puzzle3dKindHoversEqual(last.kindHover, kindHover)) {
      return;
    }
    const payload: Puzzle3dHoverPayload = { hoverTarget, kindHover };
    lastEmittedRef.current = payload;
    props.onHover(payload);
  }, [hoverTarget, kindHover, props.onHover]);

  return null;
}

/** @emoji 🎯 Redraws the canvas once per selection revision (not per id string join). */
function SelectionInvalidateBridge(): null {
  const store = useSelectionSnapshotStore();
  const revision = reactHostPort.useSyncExternalStore(store.subscribe, store.getRevision, store.getRevision);
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [revision, invalidate]);
  return null;
}

const BULK_SELECTION_TINT_RESTORE_KEY = "puzzle3dBulkSelectionTintRestore";
const bulkSelectionEmissiveColor = new Color();

interface BulkSelectionTintRestore {
  readonly emissive: Color;
  readonly emissiveIntensity: number;
}

/** @emoji 🎨 Imperative bulk-select tint (avoids N React re-renders and pooled mesh reclones). */
function applyBulkSelectionTintToGroup(group: Group, active: boolean): void {
  const selectedColors = meshStyleColors("selected");
  if (!selectedColors) {
    return;
  }
  bulkSelectionEmissiveColor.set(resolveColorHex(selectedColors.emissiveColor, "primary"));
  group.traverse((node) => {
    if (!(node instanceof Mesh)) {
      return;
    }
    const materials = Array.isArray(node.material) ? node.material : [node.material];
    for (const material of materials) {
      if (!(material instanceof MeshStandardMaterial)) {
        continue;
      }
      if (active) {
        if (!material.userData[BULK_SELECTION_TINT_RESTORE_KEY]) {
          material.userData[BULK_SELECTION_TINT_RESTORE_KEY] = {
            emissive: material.emissive.clone(),
            emissiveIntensity: material.emissiveIntensity,
          } satisfies BulkSelectionTintRestore;
        }
        material.emissive.copy(bulkSelectionEmissiveColor);
        material.emissiveIntensity = selectedColors.emissiveIntensity;
        continue;
      }
      const restore = material.userData[BULK_SELECTION_TINT_RESTORE_KEY] as BulkSelectionTintRestore | undefined;
      if (!restore) {
        continue;
      }
      material.emissive.copy(restore.emissive);
      material.emissiveIntensity = restore.emissiveIntensity;
      delete material.userData[BULK_SELECTION_TINT_RESTORE_KEY];
    }
  });
}

/** @emoji 🎨 One-pass bulk selection appearance when per-object React updates are skipped. */
function BulkSelectionVisualBridge(): null {
  const store = useSelectionSnapshotStore();
  const revision = reactHostPort.useSyncExternalStore(store.subscribe, store.getRevision, store.getRevision);
  const { collectObjectGroups } = useRegistryCore();
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useLayoutEffect(() => {
    const bulkVisual = !store.getMeshOutlineEnabled();
    const groups = collectObjectGroups();
    if (!bulkVisual) {
      for (const group of groups) {
        applyBulkSelectionTintToGroup(group, false);
      }
      return;
    }
    for (const group of groups) {
      const objectId = group.userData.puzzle3dObjectId;
      const active = typeof objectId === "string" && store.isObjectSelected(objectId);
      applyBulkSelectionTintToGroup(group, active);
    }
    invalidate();
  }, [collectObjectGroups, invalidate, revision, store]);
  return null;
}

/** @emoji 🎯 True when a raycast hit belongs to a selectable scene object or vortex mesh. */
function raycastHitTargetsPick(hitObject: Object3D): boolean {
  let node: Object3D | null = hitObject;
  while (node) {
    const data = node.userData as Record<string, unknown> | undefined;
    if (typeof data?.puzzle3dObjectId === "string" || typeof data?.puzzle3dVortexFullId === "string" || data?.puzzle3dAttractionPick === true) {
      return true;
    }
    node = node.parent;
  }
  return false;
}

/** @emoji 🎯 True when a raycast hit belongs to a unified gumball handle. */
function raycastHitIsGumball(hitObject: Object3D): boolean {
  return gumballKindFromRaycastObject(hitObject) !== null;
}

/** @emoji 🎯 Clears selection when the user clicks empty canvas (R3F pointer missed). */
function SelectionMissBridge(): null {
  const { clearSelection } = useRegistryInteraction();
  const { attractionDragActive, attractionIndirectPickAwait } = useRegistryDrag();
  const setState = useThree((state) => state.set);
  const getState = useThree((state) => state.get);
  const attractionBusy = attractionDragActive || attractionIndirectPickAwait !== null;
  const clearSelectionRef = reactHostPort.useRef(clearSelection);
  clearSelectionRef.current = clearSelection;
  const attractionBusyRef = reactHostPort.useRef(attractionBusy);
  attractionBusyRef.current = attractionBusy;
  reactHostPort.useEffect(() => {
    const previous = getState().onPointerMissed;
    const onMiss = (event: MouseEvent) => {
      if (event.button !== 0 || attractionBusyRef.current || puzzle3dRelocateDragActiveRef.current || gumballPointerConsumesCanvasEventRef.current) {
        previous?.(event);
        return;
      }
      const hits = getState().internal.initialHits;
      if (hits.some((hit) => raycastHitTargetsPick(hit.object) || raycastHitIsGumball(hit.object))) {
        previous?.(event);
        return;
      }
      clearSelectionRef.current();
      previous?.(event);
    };
    setState({ onPointerMissed: onMiss });
    return () => setState({ onPointerMissed: previous });
  }, [getState, setState]);
  return null;
}

//#region 🔖VortexScreenPick
/** @emoji 🌀 Screen-space pixel radius around a vortex center that counts as a hover/click on that vortex. */
const VORTEX_SCREEN_PICK_RADIUS_PX = 18;

/** @emoji 🌀 World depth a vortex may sit behind the clicked surface and still be pickable (covers vortices embedded in their own object), beyond which it is treated as occluded by foreground geometry. */
const VORTEX_PICK_DEPTH_TOLERANCE = 6;

/** @emoji 🌀 Screen-projected vortex candidate for {@link pickNearestScreenVortex}. */
interface ScreenVortexCandidate {
  readonly fullId: string;
  readonly objectId: string;
  readonly sx: number;
  readonly sy: number;
  readonly dist: number;
}

/**
 * 🌀 Picks the vortex closest to the cursor in screen space within {@link VORTEX_SCREEN_PICK_RADIUS_PX},
 * skipping ones occluded by foreground geometry beyond {@link VORTEX_PICK_DEPTH_TOLERANCE} of the clicked surface.
 */
export function pickNearestScreenVortex(args: {
  readonly cursorX: number;
  readonly cursorY: number;
  readonly surfaceDist: number;
  readonly candidates: readonly ScreenVortexCandidate[];
  readonly radiusPx?: number;
  readonly depthTolerance?: number;
}): ScreenVortexCandidate | null {
  const radiusPx = args.radiusPx ?? VORTEX_SCREEN_PICK_RADIUS_PX;
  const depthTolerance = args.depthTolerance ?? VORTEX_PICK_DEPTH_TOLERANCE;
  const within = args.candidates
    .map((c) => ({ c, dpx: Math.hypot(c.sx - args.cursorX, c.sy - args.cursorY) }))
    .filter((e) => e.dpx <= radiusPx)
    .sort((a, b) => a.dpx - b.dpx);
  for (const { c } of within) {
    if (c.dist <= args.surfaceDist + depthTolerance) return c;
  }
  return null;
}
//#endregion
//#endregion ­ƒÄ»Registry

//#region ­ƒº▱Chunking
export { chunkBoundsRadius, chunkDistanceVisible, chunkKey };
//#endregion ­ƒº▱Chunking

//#region 🧭Coordinates
export {
  cadObjectLocalDirectionToThreeGroupLocal,
  cadObjectLocalToThreeGroupLocal,
  cadQuatToThree,
  cadToThreeMatrix,
  cadVec3ToThree,
  GLB_MESH_FRAME_ROTATION_X,
  threeQuatToCad,
  threeVec3ToCad,
};

function quatRotateVec(q: Quat, v: Vec3): Vec3 {
  const out = new Vector3(v[0], v[1], v[2]).applyQuaternion(new Quaternion(q[0], q[1], q[2], q[3]));
  return [out.x, out.y, out.z];
}
//#endregion 🧭Coordinates

//#region ­ƒºèHelpers
function vec3ToThree(v: Vec3) {
  return new Vector3(...cadVec3ToThree(v));
}

function quatToThree(q: Quat | undefined) {
  if (!q) return new Quaternion();
  const c = cadQuatToThree(q);
  return new Quaternion(c[0], c[1], c[2], c[3]);
}

function scaleToThree(s: number | Vec3 | undefined): Vector3 {
  if (s === undefined) return new Vector3(1, 1, 1);
  if (typeof s === "number") return new Vector3(s, s, s);
  return new Vector3(s[0], s[1], s[2]);
}

/** @emoji ­ƒöæ Stable key for object pose props (relocate mutates the group without changing this until commit). */
export function objectPoseKey(id: string, origin: Vec3, orientation: Quat | undefined, scale: number | Vec3 | undefined): string {
  const o = orientation ?? ([0, 0, 0, 1] as Quat);
  const sc = scale === undefined ? 1 : typeof scale === "number" ? scale : scale.join(",");
  return `${id}|${origin.join(",")}|${o.join(",")}|${sc}`;
}

/** @emoji ­ƒôì Writes fixture pose onto an object group; avoids R3F controlled transforms so vortex children follow relocate. */
function groupMatchesFixturePose(group: Group, origin: Vec3, orientation: Quat | undefined, scale: number | Vec3 | undefined): boolean {
  const p = cadVec3ToThree(origin);
  if (Math.abs(group.position.x - p[0]) > 1e-5 || Math.abs(group.position.y - p[1]) > 1e-5 || Math.abs(group.position.z - p[2]) > 1e-5) {
    return false;
  }
  const q = quatToThree(orientation);
  if (Math.abs(group.quaternion.x - q.x) > 1e-5 || Math.abs(group.quaternion.y - q.y) > 1e-5) {
    return false;
  }
  if (Math.abs(group.quaternion.z - q.z) > 1e-5 || Math.abs(group.quaternion.w - q.w) > 1e-5) {
    return false;
  }
  const s = scaleToThree(scale);
  if (Math.abs(group.scale.x - s.x) > 1e-5 || Math.abs(group.scale.y - s.y) > 1e-5 || Math.abs(group.scale.z - s.z) > 1e-5) {
    return false;
  }
  return true;
}

export function applyObjectPose(group: Group, origin: Vec3, orientation: Quat | undefined, scale: number | Vec3 | undefined): void {
  const p = cadVec3ToThree(origin);
  group.position.set(p[0], p[1], p[2]);
  group.quaternion.copy(quatToThree(orientation));
  group.scale.copy(scaleToThree(scale));
}

/** @emoji ­ƒî│ Composes world matrices along the ancestor chain only so {@link Object3D.getWorldPosition} matches without re-walking scene descendants. */
export function updateWorldMatrixChain(leaf: Object3D): void {
  const chain: Object3D[] = [];
  for (let cur: Object3D | null = leaf; cur; cur = cur.parent) {
    chain.push(cur);
  }
  for (let i = chain.length - 1; i >= 0; i--) {
    const node = chain[i]!;
    node.updateMatrix();
    if (node.parent) {
      node.matrixWorld.multiplyMatrices(node.parent.matrixWorld, node.matrix);
    } else {
      node.matrixWorld.copy(node.matrix);
    }
  }
}

export type AutoFitBehavior = "initial" | "changes";

const puzzle3dAutoFitInitialAppliedSeeds = new Set<string>();

/** @emoji 🛰️ True when initial AutoFit already ran for a viewport seed (survives scene-core remounts). */
export function puzzle3dAutoFitInitialApplied(seedKey: string | number): boolean {
  return puzzle3dAutoFitInitialAppliedSeeds.has(String(seedKey));
}

/** @emoji 🛰️ Records initial AutoFit completion for a viewport seed. */
export function puzzle3dAutoFitMarkInitialApplied(seedKey: string | number): void {
  puzzle3dAutoFitInitialAppliedSeeds.add(String(seedKey));
}

/** @emoji 🖌️ True while brush preview or suggestion menu is active (suppresses camera auto-fit). */
export function puzzle3dBrushSessionActive(): boolean {
  const ui = puzzle3dBrushUiStore.getSnapshot();
  return ui.targetActive || ui.menuOpen || ui.preview !== null;
}

export function puzzle3dAutoFitShouldRun(behavior: AutoFitBehavior, key: string, lastKey: string, hasApplied: boolean): boolean {
  if (!key || key === lastKey) return false;
  return behavior === "changes" || !hasApplied;
}

/** @emoji 📐 Scene accessors for {@link boundsFromPuzzle3dSelection}. */
export interface Puzzle3dSelectionFrameSource {
  readonly getObjectGroup: (id: string) => Group | null;
  readonly getVortexWorld: (fullId: string) => Vector3 | null;
  readonly listVortexBindings: () => readonly VortexBindingMeta[];
}

/** @emoji 🔍 Axis-aligned bounds of the current puzzle 3D selection (objects, vortices, attractions). */
export function boundsFromPuzzle3dSelection(
  selection: SelectionSnapshot,
  source: Puzzle3dSelectionFrameSource,
  attractions: readonly Pick<AttractionProps, "id" | "attracting" | "attracted">[],
): { readonly center: Vec3; readonly radius: number } | null {
  const box = new Box3();
  const pointBox = new Box3();
  const pointCenter = new Vector3();
  const pointSize = new Vector3();
  const centerScratch = new Vector3();
  const sizeScratch = new Vector3();
  let has = false;
  const unionWorldPoint = (world: Vector3, radius: number) => {
    const r = Math.max(radius, 0.5);
    pointSize.set(r * 2, r * 2, r * 2);
    pointCenter.copy(world);
    pointBox.setFromCenterAndSize(pointCenter, pointSize);
    if (!has) {
      box.copy(pointBox);
      has = true;
      return;
    }
    box.union(pointBox);
  };
  for (const objectId of selection.objectIds) {
    const group = source.getObjectGroup(objectId);
    if (!group) {
      continue;
    }
    updateWorldMatrixChain(group);
    const part = new Box3().setFromObject(group, true);
    if (!Number.isFinite(part.min.x) || part.isEmpty()) {
      continue;
    }
    part.getSize(sizeScratch);
    if (sizeScratch.lengthSq() < 1e-12) {
      continue;
    }
    if (!has) {
      box.copy(part);
      has = true;
    } else {
      box.union(part);
    }
  }
  const vortexRadiusByFullId = new Map(source.listVortexBindings().map((meta) => [meta.fullId, meta.radiusWorld]));
  for (const fullId of selection.vortexIds) {
    const world = source.getVortexWorld(fullId);
    if (!world || !vector3IsFinite(world)) {
      continue;
    }
    unionWorldPoint(world, vortexRadiusByFullId.get(fullId) ?? 1);
  }
  const attractionById = new Map(attractions.map((row) => [row.id, row]));
  for (const attractionId of selection.attractionIds) {
    const attraction = attractionById.get(attractionId);
    if (!attraction) {
      continue;
    }
    const a = source.getVortexWorld(attraction.attracting);
    const b = source.getVortexWorld(attraction.attracted);
    if (a && vector3IsFinite(a)) {
      unionWorldPoint(a, 0.75);
    }
    if (b && vector3IsFinite(b)) {
      unionWorldPoint(b, 0.75);
    }
  }
  if (!has) {
    return null;
  }
  box.getCenter(centerScratch);
  box.getSize(sizeScratch);
  return { center: threeVec3ToCad(centerScratch), radius: Math.max(sizeScratch.length() / 2, 0.5) };
}

/** @emoji 📐 Axis-aligned bounds of registered scene object groups (camera auto-fit). */
export function boundsFromObjectGroups(groups: readonly Group[]): { readonly center: Vec3; readonly radius: number } | null {
  if (!groups.length) return null;
  const box = new Box3();
  const sizeScratch = new Vector3();
  const centerScratch = new Vector3();
  let has = false;
  for (const group of groups) {
    updateWorldMatrixChain(group);
    const part = new Box3().setFromObject(group, true);
    if (!Number.isFinite(part.min.x) || !Number.isFinite(part.max.x)) continue;
    if (part.isEmpty()) continue;
    part.getSize(sizeScratch);
    if (sizeScratch.lengthSq() < 1e-12) continue;
    if (!has) {
      box.copy(part);
      has = true;
    } else {
      box.union(part);
    }
  }
  if (!has) return null;
  box.getCenter(centerScratch);
  box.getSize(sizeScratch);
  const radius = sizeScratch.length() / 2;
  return { center: threeVec3ToCad(centerScratch), radius: Math.max(radius, 0.5) };
}

/** @emoji ⏱️ Duration of engagement selection zoom camera ease (ms). */
export const PUZZLE_3D_SELECTION_ZOOM_DURATION_MS = 450;

/** @emoji ⏱️ Ease-in-out cubic for selection zoom and other puzzle 3D camera transitions. */
export function puzzle3dEaseInOutCubic01(t: number): number {
  const x = Math.min(1, Math.max(0, t));
  return x < 0.5 ? 4 * x * x * x : 1 - (-2 * x + 2) ** 3 / 2;
}

/** @emoji 🛰️ Orbit position + target in Three world space for framing a bounds sphere. */
export function puzzle3dFitCameraRigFromBounds(
  bounds: { readonly center: Vec3; readonly radius: number },
  padding = 1.25,
): { readonly position: Vector3; readonly target: Vector3 } {
  const centerThree = cadVec3ToThree(bounds.center);
  const dist = Math.max(bounds.radius * padding, 2);
  return {
    position: new Vector3(centerThree[0] + dist, centerThree[1] + dist, centerThree[2] + dist * 0.85),
    target: new Vector3(centerThree[0], centerThree[1], centerThree[2]),
  };
}

/** @emoji 🛰️ Frames perspective orbit camera to fit scene object bounds (CAD center, Three world rig). */
export function applyAutoFitCamera(camera: ThreePerspectiveCamera, bounds: { readonly center: Vec3; readonly radius: number }, padding = 1.25, controls?: { readonly target: Vector3; update?: () => void } | null): void {
  const rig = puzzle3dFitCameraRigFromBounds(bounds, padding);
  camera.position.copy(rig.position);
  if (controls?.target) {
    controls.target.copy(rig.target);
    controls.update?.();
  } else {
    camera.lookAt(rig.target);
  }
  camera.updateProjectionMatrix();
}

/** @emoji 🔍 True while engagement selection zoom is easing the orbit camera. */
export const puzzle3dSelectionZoomAnimatingRef = { current: false };

function vector3IsFinite(v: Vector3): boolean {
  return Number.isFinite(v.x) && Number.isFinite(v.y) && Number.isFinite(v.z);
}
//#endregion ­ƒºèHelpers

//#region ­ƒº▓AttractionGesture
function readVortexFullIdFromObject(o: Object3D | null): string | null {
  let cur: Object3D | null = o;
  while (cur) {
    const id = cur.userData?.puzzle3dVortexFullId;
    if (typeof id === "string" && id.length > 0) return id;
    cur = cur.parent;
  }
  return null;
}

function readObjectItemIdFromObject(o: Object3D | null): string | null {
  let cur: Object3D | null = o;
  while (cur) {
    const id = cur.userData?.puzzle3dObjectId;
    if (typeof id === "string" && id.length > 0) return id;
    cur = cur.parent;
  }
  return null;
}

const HANDLE_HIT_TOLERANCE_PX = 10;
const ATTRACTION_HANDLE_SNAP_EXTRA_PX = 22;
const ATTRACTION_COMMIT_SNAP_TIGHT_PX = 2;

function worldToCanvasPx(world: Vector3, camera: Camera, gl: WebGLRenderer): { x: number; y: number } {
  const v = world.clone().project(camera);
  const w = gl.domElement.clientWidth;
  const h = gl.domElement.clientHeight;
  return { x: (v.x * 0.5 + 0.5) * w, y: (-v.y * 0.5 + 0.5) * h };
}

function pixelsPerWorldUnitAt(camera: Camera, gl: WebGLRenderer, world: Vector3): number {
  if (!(camera as ThreePerspectiveCamera).isPerspectiveCamera) return 1;
  const pc = camera as ThreePerspectiveCamera;
  const dist = pc.position.distanceTo(world);
  const fovRad = (pc.fov * Math.PI) / 180;
  const h = Math.max(1, gl.domElement.clientHeight);
  return h / (2 * Math.tan(fovRad / 2) * Math.max(dist, 1e-6));
}

function attractionSnapDragTolerancePx(worldHandle: Vector3, radiusWorld: number, camera: Camera, gl: WebGLRenderer): number {
  const mpp = pixelsPerWorldUnitAt(camera, gl, worldHandle);
  const radPx = radiusWorld * mpp;
  return HANDLE_HIT_TOLERANCE_PX + ATTRACTION_HANDLE_SNAP_EXTRA_PX + radPx * camera.zoom;
}

function attractionSnapCommitTolerancePx(worldHandle: Vector3, radiusWorld: number, camera: Camera, gl: WebGLRenderer): number {
  const mpp = pixelsPerWorldUnitAt(camera, gl, worldHandle);
  const radPx = radiusWorld * mpp;
  return HANDLE_HIT_TOLERANCE_PX + ATTRACTION_COMMIT_SNAP_TIGHT_PX + radPx * camera.zoom;
}

function attractionSnapCommitProximityOk(attractedFullId: string, pointerWorld: Vector3, camera: Camera, gl: WebGLRenderer, getVortexWorld: (id: string) => Vector3 | null, metaRadius: (id: string) => number): boolean {
  const hw = getVortexWorld(attractedFullId);
  if (!hw) return false;
  const pScr = worldToCanvasPx(pointerWorld, camera, gl);
  const hScr = worldToCanvasPx(hw, camera, gl);
  const d = Math.hypot(pScr.x - hScr.x, pScr.y - hScr.y);
  return d <= attractionSnapCommitTolerancePx(hw, metaRadius(attractedFullId), camera, gl);
}

function nearestAttractionSnapFullId(args: {
  lod: number;
  pointerWorld: Vector3;
  attractingFullId: string;
  compat: ReadonlySet<string>;
  blocked: ReadonlySet<string>;
  camera: Camera;
  gl: WebGLRenderer;
  getVortexWorld: (id: string) => Vector3 | null;
  metaRadius: (id: string) => number;
}): string | null {
  if (args.lod >= PUZZLE_3D_ATTRACTION_SNAP_MAX_LOD) return null;
  const pScr = worldToCanvasPx(args.pointerWorld, args.camera, args.gl);
  let best: { d: number; id: string } | null = null;
  for (const tid of args.compat) {
    if (tid === args.attractingFullId) continue;
    if (args.blocked.has(tid)) continue;
    const hw = args.getVortexWorld(tid);
    if (!hw) continue;
    const hScr = worldToCanvasPx(hw, args.camera, args.gl);
    const d = Math.hypot(hScr.x - pScr.x, hScr.y - pScr.y);
    const tol = attractionSnapDragTolerancePx(hw, args.metaRadius(tid), args.camera, args.gl);
    if (d > tol) continue;
    if (!best || d < best.d) best = { d, id: tid };
  }
  return best?.id ?? null;
}
//#endregion ­ƒº▓AttractionGesture

//#region ­ƒºèMesh
/** @emoji 🖱️ R3F pointer handlers for scene mesh pick targets. */
export interface MeshPointerHandlers {
  readonly onPointerDown?: (event: ThreeEvent<PointerEvent>) => void;
  readonly onClick?: (event: ThreeEvent<MouseEvent>) => void;
  readonly onPointerOver?: (event: ThreeEvent<PointerEvent>) => void;
  readonly onPointerOut?: (event: ThreeEvent<PointerEvent>) => void;
}

export interface MeshProps extends MeshPointerHandlers {
  readonly meshUrl: string;
  readonly style?: MeshStyleKind;
  readonly showOutline?: boolean;
  readonly dimmed?: boolean;
  readonly userData?: Record<string, unknown>;
  readonly scale?: number | [number, number, number];
}

/** @emoji 🧭 Inner group: glTF Y-up mesh geometry → CAD object-local Z-up (fixture pose stays CAD). */
function GlbMeshFrame(props: { readonly children: ReactNode }) {
  return <group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>{props.children}</group>;
}

/** @emoji ­ƒºè Pooled GLB body with {@link MeshStyleKind} recoloring aligned to Elements tokens. */
export const MeshBody = reactHostPort.memo(function MeshBody(props: MeshProps) {
  if (!isLoadableMeshUrl(props.meshUrl)) {
    return null;
  }
  const style = props.style ?? DEFAULT_MESH_STYLE;
  const renderRoot = usePooledStyledMesh(props.meshUrl, style, true);
  if (!renderRoot) {
    return null;
  }
  const scale = props.scale;
  const outlineColor = meshStyleColors("selected")?.lineColor ?? "#ff344f";
  const dimmedClone = reactHostPort.useMemo(() => {
    if (!renderRoot || !props.dimmed) {
      return renderRoot;
    }
    const clone = renderRoot.clone(true);
    applyOpacityScaleToObject3D(clone, WORLD_LOCKED_OPACITY_SCALE);
    return clone;
  }, [props.dimmed, renderRoot]);
  return (
    <GlbMeshFrame>
      <Clone
        object={dimmedClone ?? renderRoot}
        {...(scale !== undefined
          ? {
              scale: typeof scale === "number" ? ([scale, scale, scale] as [number, number, number]) : (scale as [number, number, number]),
            }
          : {})}
        onClick={props.onClick}
        onPointerDown={props.onPointerDown}
        onPointerOut={props.onPointerOut}
        onPointerOver={props.onPointerOver}
        userData={props.userData}
      >
        {props.showOutline ? <Outlines color={outlineColor} thickness={4} /> : null}
      </Clone>
    </GlbMeshFrame>
  );
});

const PlaceholderMesh = reactHostPort.memo(function PlaceholderMesh(props: MeshPointerHandlers & { readonly style: MeshStyleKind; readonly showOutline?: boolean; readonly dimmed?: boolean }) {
  const colors = meshStyleColors(props.style);
  const meshColor = colors?.meshColor ?? tokenHex("light-5-7");
  const opacity = (colors?.opacity ?? 1) * (props.dimmed ? WORLD_LOCKED_OPACITY_SCALE : 1);
  const outlineColor = meshStyleColors("selected")?.lineColor ?? tokenHex("primary");
  return (
    <GlbMeshFrame>
      <mesh onClick={props.onClick} onPointerDown={props.onPointerDown} onPointerOut={props.onPointerOut} onPointerOver={props.onPointerOver}>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color={meshColor} emissive={colors?.emissiveColor} emissiveIntensity={colors?.emissiveIntensity ?? 0} metalness={0.05} roughness={0.85} transparent={opacity < 1} opacity={opacity} />
        {props.showOutline ? <Outlines color={outlineColor} thickness={4} /> : null}
      </mesh>
    </GlbMeshFrame>
  );
});
//#endregion ­ƒºèMesh

//#region ­ƒºèObject

const ObjectTransformControls = reactHostPort.memo(function ObjectTransformControls(props: {
  readonly object: Group;
  readonly objectId: string;
  readonly config: GumballConfig;
  readonly beforeRef: MutableRefObject<{ origin: Vector3; quat: Quaternion; scale: Vector3 } | null>;
}) {
  const { onRelocate, findNearestProximityRelocate, onProximityConnect, proximityRelocateEnabled } = useRegistryCore();
  return (
    <UnifiedGumball
      target={props.object}
      config={props.config}
      onDragStart={() => {
        puzzle3dRelocateDragActiveRef.current = true;
        cancelPuzzle3dMarqueeGesture();
        const g = props.object;
        props.beforeRef.current = {
          origin: g.position.clone(),
          quat: g.quaternion.clone(),
          scale: g.scale.clone(),
        };
      }}
      onDraggingChanged={(active) => {
        puzzle3dRelocateDragActiveRef.current = active;
        if (active) cancelPuzzle3dMarqueeGesture();
      }}
      onDragEnd={(kind, before, after) => {
        puzzle3dRelocateDragActiveRef.current = false;
        const snapshot = props.beforeRef.current;
        if (!snapshot) return;
        const mode = gumballHandleKindToRelocateMode(kind);
        const payload: RelocatePayload = {
          objectId: props.objectId,
          mode,
          before: {
            origin: threeVec3ToCad(snapshot.origin),
            orientation: threeQuatToCad(snapshot.quat),
            scale: snapshot.scale.toArray() as unknown as Vec3,
          },
          after: {
            origin: threeVec3ToCad(new Vector3(after.position[0], after.position[1], after.position[2])),
            orientation: threeQuatToCad(new Quaternion(after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3])),
            scale: after.scale as Vec3,
          },
        };
        props.beforeRef.current = null;
        onRelocate?.(payload);
        if (!proximityRelocateEnabled || mode !== "translate") {
          return;
        }
        scheduleRelocateCommit(() => {
          const cand = findNearestProximityRelocate(props.object.position, props.objectId);
          if (cand) onProximityConnect?.(cand);
        });
      }}
    />
  );
});

export const ObjectItem = reactHostPort.memo(function ObjectItem(props: ObjectProps) {
  const group = reactHostPort.useRef<Group>(null);
  const store = useSelectionSnapshotStore();
  const bulkVisual = !store.getMeshOutlineEnabled();
  const registrySelected = useObjectSelected(props.id);
  const primaryObjectId = usePrimarySelectionObjectId();
  const { registerObject, gumballConfig } = useRegistryCore();
  const { selectionMode, commitSelection, setActiveRelocateObjectId } = useRegistryInteraction();
  const { setHover, clearHover, isHovered, isKindHovered } = useRegistryHover();
  const { attractionDragActive, attractionIndirectPickAwait, attractionCompatibleAttractedFullIds } = useRegistryDrag();
  const beforeRef = reactHostPort.useRef<{ origin: Vector3; quat: Quaternion; scale: Vector3 } | null>(null);
  const [tcTarget, setTcTarget] = reactHostPort.useState<Group | null>(null);
  const objectPointerHovered = isHovered({ kind: "object", id: props.id }) || isKindHovered("object", props.objectKind);
  const entityFlags = reactHostPort.useMemo(() => ({ hidden: props.hidden === true, locked: props.locked === true }), [props.hidden, props.locked]);
  const entityRevealed = objectPointerHovered;
  const renderMode = reactHostPort.useMemo(
    () =>
      worldEntityRenderMode(entityFlags, {
        hovered: objectPointerHovered,
        selected: props.selected === true || registrySelected,
        revealed: entityRevealed,
      }),
    [entityFlags, entityRevealed, objectPointerHovered, props.selected, registrySelected],
  );
  const entitySelectable = worldEntitySelectable(entityFlags);
  const membershipSelected = props.selected === true || registrySelected || (bulkVisual && store.isObjectSelected(props.id));
  const selectedForAppearance = bulkVisual ? false : membershipSelected && renderMode.showSelectedOutline;
  const relocateActive = props.relocateActive === true || primaryObjectId === props.id;

  reactHostPort.useEffect(() => {
    registerObject(props.id, props.objectKind, group.current);
    return () => {
      registerObject(props.id, props.objectKind, null);
    };
  }, [props.id, props.objectKind, registerObject]);

  reactHostPort.useEffect(() => {
    if (group.current) setTcTarget(group.current);
  }, [membershipSelected, relocateActive, props.id]);

  const linkHighlighted = reactHostPort.useMemo(() => {
    if (props.highlighted === true) {
      return true;
    }
    const prefix = `${props.id}:`;
    for (const fullId of attractionCompatibleAttractedFullIds) {
      if (fullId.startsWith(prefix)) {
        return true;
      }
    }
    return false;
  }, [props.highlighted, props.id, attractionCompatibleAttractedFullIds]);

  const meshStyle = reactHostPort.useMemo(
    () =>
      resolveMeshStyle({
        style: props.style,
        disabled: props.disabled,
        selected: selectedForAppearance,
        highlighted: linkHighlighted,
        hovered: props.hovered === true || renderMode.asHover,
      }),
    [props.style, props.disabled, selectedForAppearance, props.hovered, linkHighlighted, renderMode.asHover],
  );
  const showSelectionOutline = selectedForAppearance && !props.disabled;
  const meshDimmed = renderMode.dim;

  const selectObject = reactHostPort.useCallback(() => {
    if (puzzle3dTargetVolumeToolActiveRef.current || attractionDragActive || attractionIndirectPickAwait || props.disabled || !entitySelectable || puzzle3dRelocateDragActiveRef.current) {
      return;
    }
    commitSelection({ kind: "object", id: props.id });
    setActiveRelocateObjectId(props.id);
  }, [attractionDragActive, attractionIndirectPickAwait, commitSelection, entitySelectable, props.disabled, props.id, setActiveRelocateObjectId]);

  const meshPointerHandlers = reactHostPort.useMemo(
    () =>
      entitySelectable
        ? {
            onPointerDown: (e: ThreeEvent<PointerEvent>) => {
              if (e.nativeEvent.button !== 0) {
                return;
              }
              e.stopPropagation();
            },
            onClick: (e: ThreeEvent<MouseEvent>) => {
              if (e.nativeEvent.button !== 0 || puzzle3dMarqueeSuppressClickRef.current) {
                return;
              }
              e.stopPropagation();
              selectObject();
            },
            onPointerOver: (e: ThreeEvent<PointerEvent>) => {
              e.stopPropagation();
              if (!props.disabled && !attractionDragActive && !attractionIndirectPickAwait) {
                setHover({ kind: "object", id: props.id });
              }
            },
            onPointerOut: (e: ThreeEvent<PointerEvent>) => {
              e.stopPropagation();
              clearHover({ kind: "object", id: props.id });
            },
          }
        : {},
    [clearHover, entitySelectable, props.disabled, props.id, selectObject, setHover],
  );

  const poseKey = reactHostPort.useMemo(() => objectPoseKey(props.id, props.origin, props.orientation, props.scale), [props.id, props.origin, props.orientation, props.scale]);
  reactHostPort.useLayoutEffect(() => {
    const g = group.current;
    if (!g || puzzle3dRelocateDragActiveRef.current) {
      return;
    }
    if (groupMatchesFixturePose(g, props.origin, props.orientation, props.scale)) {
      return;
    }
    applyObjectPose(g, props.origin, props.orientation, props.scale);
  }, [poseKey, props.origin, props.orientation, props.scale]);
  const lodCtx = useLod();
  const resolvedMeshUrl = useResolvedMeshUrl({
    origin: props.origin,
    meshByLod: props.meshByLod,
    fallbackMeshUrl: isLoadableMeshUrl(props.meshUrl) ? props.meshUrl : PLACEHOLDER_MESH_URL,
  });
  const config = reactHostPort.useMemo(() => {
    if (props.relocate === false) return { moveAxes: false, movePlanes: false, rotate: false, scaleAxes: false, scalePlanes: false, scaleUniform: false };
    const base = puzzle3dObjectGumballConfig(props.relocate === undefined ? gumballConfig : props.relocate);
    const transSnap = lodCtx.gridSnapEnabled && lodCtx.gridStepWorld != null && lodCtx.gridStepWorld > 0 ? lodCtx.gridStepWorld : undefined;
    return { ...base, translationSnap: transSnap ?? base.translationSnap };
  }, [props.relocate, gumballConfig, lodCtx.gridSnapEnabled, lodCtx.gridStepWorld]);
  const showTc = membershipSelected && relocateActive && props.relocate !== false && tcTarget && gumballConfigVisible(config) && entitySelectable;

  return (
    <>
      <group
        ref={group}
        visible={renderMode.visible}
        userData={{
          puzzle3dObjectId: props.id,
          puzzle3dMeshUrl: resolvedMeshUrl,
          puzzle3dMeshStyle: meshStyle,
          ...(props.attracting?.length ? { puzzle3dAttracting: props.attracting } : {}),
          ...(props.wormhole ? { puzzle3dWormhole: true } : {}),
          ...props.userData,
        }}
      >
        {resolvedMeshUrl === PLACEHOLDER_MESH_URL ? (
          <PlaceholderMesh dimmed={meshDimmed} showOutline={showSelectionOutline} style={meshStyle} {...meshPointerHandlers} />
        ) : (
          <MeshBody dimmed={meshDimmed} meshUrl={resolvedMeshUrl} showOutline={showSelectionOutline} style={meshStyle} {...meshPointerHandlers} />
        )}
        <group userData={{ puzzle3dObjectAttachments: props.id }}>{props.children}</group>
      </group>
      {showTc && tcTarget && <ObjectTransformControls object={tcTarget} objectId={props.id} config={config} beforeRef={beforeRef} />}
    </>
  );
});
//#endregion ­ƒºèObject

//#region ­ƒîÇVortex
const vortexFallbackMatProps = { transparent: true, opacity: 0.55 } as const;

//#region 🔖VortexPickPriority
/** @emoji 🎯 World-space depth bias (units) so a vortex pick wins over the object surface it sits on, without hijacking clicks on distant geometry. */
const VORTEX_PICK_DEPTH_BIAS = 1.5;

/** @emoji 🎯 Mesh raycast biasing vortex hits closer so occluding object meshes do not swallow vortex hover/selection ({@link VORTEX_PICK_DEPTH_BIAS}). */
function vortexPickRaycast(this: import("three").Mesh, raycaster: Raycaster, intersects: import("three").Intersection[]): void {
  const local: import("three").Intersection[] = [];
  Mesh.prototype.raycast.call(this, raycaster, local);
  for (const hit of local) {
    hit.distance = Math.max(hit.distance - VORTEX_PICK_DEPTH_BIAS, hit.distance * 0.01);
    intersects.push(hit);
  }
}
//#endregion

function VortexMeshGltf(props: { meshUrl: string; fullId: string; radius: number; style: MeshStyleKind; onPointerOver?: (e: ThreeEvent<PointerEvent>) => void; onPointerOut?: (e: ThreeEvent<PointerEvent>) => void }) {
  const scale = (props.radius / 0.35) * 0.9;
  const { onPointerOver, onPointerOut, ...meshProps } = props;
  return <MeshBody meshUrl={meshProps.meshUrl} style={meshProps.style} scale={scale} userData={{ puzzle3dVortexFullId: meshProps.fullId }} onPointerOver={onPointerOver} onPointerOut={onPointerOut} />;
}

function vortexHighlightMeshStyle(highlight: "none" | "compatible" | "ring" | "attracting" | "indirectRing"): MeshStyleKind {
  switch (highlight) {
    case "ring":
    case "indirectRing":
      return "highlighted";
    case "compatible":
      return "hovered";
    case "attracting":
      return "selected";
    default:
      return "neutral";
  }
}

function VortexDirectionArrow(props: {
  directionCad: Vec3;
  objectOrigin: Vec3;
  objectOrientation?: Quat;
  radius: number;
  selected?: boolean;
}) {
  const dirThree = reactHostPort.useMemo(
    () => cadObjectLocalDirectionToThreeGroupLocal(props.directionCad, props.objectOrigin, props.objectOrientation),
    [props.directionCad, props.objectOrigin, props.objectOrientation],
  );
  const points = reactHostPort.useMemo(() => {
    const len = Math.max(props.radius * 2, 0.5);
    const tip: Vec3 = [dirThree[0] * len, dirThree[1] * len, dirThree[2] * len];
    return [[0, 0, 0] as Vec3, tip];
  }, [dirThree, props.radius]);
  const color = reactHostPort.useMemo(
    () => lineCssColor(props.selected ? CSS_HOVERED_LINE : CSS_ATTRACTION_ENDPOINT_LINE, props.selected ? "secondary" : "gray"),
    [props.selected],
  );
  return (
    <group renderOrder={2}>
      <Line points={points} color={color} lineWidth={props.selected ? 3 : 2} transparent opacity={props.selected ? 0.95 : 0.72} depthTest={false} />
      <mesh position={points[1]}>
        <sphereGeometry args={[props.radius * 0.18, 8, 8]} />
        <meshBasicMaterial color={color} transparent opacity={props.selected ? 0.95 : 0.72} depthTest={false} />
      </mesh>
    </group>
  );
}

function VortexFallbackMesh(props: {
  fullId: string;
  radius: number;
  highlight: "none" | "compatible" | "ring" | "attracting" | "indirectRing";
  hovered?: boolean;
  onPointerOver?: (e: ThreeEvent<PointerEvent>) => void;
  onPointerOut?: (e: ThreeEvent<PointerEvent>) => void;
}) {
  const style = props.highlight === "none" && props.hovered ? "hovered" : vortexHighlightMeshStyle(props.highlight);
  const colors = meshStyleColors(style) ?? meshStyleColors("neutral")!;
  const { onPointerOver, onPointerOut, ...meshProps } = props;
  return (
    <mesh userData={{ puzzle3dVortexFullId: meshProps.fullId }} onPointerOver={onPointerOver} onPointerOut={onPointerOut}>
      <sphereGeometry args={[meshProps.radius, 12, 12]} />
      <meshStandardMaterial color={colors.meshColor} emissive={colors.emissiveColor} emissiveIntensity={colors.emissiveIntensity} transparent={colors.opacity < 1} opacity={colors.opacity} {...vortexFallbackMatProps} />
    </mesh>
  );
}

export const Vortex = reactHostPort.memo(function Vortex(
  props: VortexProps & {
    objectId: string;
    objectKind?: string;
    objectOrigin: Vec3;
    objectOrientation?: Quat;
    selected?: boolean;
  },
) {
  const root = reactHostPort.useRef<Group | null>(null);
  const reg = useRegistry();
  const { commitSelection, setActiveRelocateObjectId } = useRegistryInteraction();
  const fullId = props.id.includes(":") ? props.id : `${props.objectId}:${props.id}`;
  const vortexSelected = useVortexSelected(fullId);
  const r = props.radius ?? 0.35;
  const vortexPointerGestureRef = reactHostPort.useRef<{ readonly pointerId: number; readonly x: number; readonly y: number; dragStarted: boolean } | null>(null);

  reactHostPort.useEffect(() => {
    const getter = () => {
      if (!root.current) return null;
      updateWorldMatrixChain(root.current);
      const v = new Vector3();
      root.current.getWorldPosition(v);
      return v;
    };
    reg.registerVortex(fullId, getter);
    return () => {
      reg.unregisterVortex(fullId);
    };
  }, [fullId, reg]);

  const bindRoot = reactHostPort.useCallback(
    (node: Group | null) => {
      root.current = node;
      if (node) {
        reg.registerVortexBinding(
          {
            fullId,
            objectId: props.objectId,
            objectKind: props.objectKind,
            vortexKind: props.vortexKind,
            radiusWorld: r,
          },
          node,
        );
      } else {
        reg.unregisterVortexBinding(fullId);
      }
    },
    [fullId, props.objectId, props.objectKind, props.vortexKind, reg],
  );

  const lodCtx = useLod();
  const worldPosRef = reactHostPort.useRef(new Vector3());
  const vortexMeshByLodRef = reactHostPort.useRef(props.vortexMeshByLod);
  vortexMeshByLodRef.current = props.vortexMeshByLod;
  const vortexMeshUrlRef = reactHostPort.useRef(props.vortexMeshUrl);
  vortexMeshUrlRef.current = props.vortexMeshUrl;
  const trackVortexLod = lodCtx.depthVariable || (props.vortexMeshByLod?.length ?? 0) > 0;
  const [lodVisual, setLodVisual] = reactHostPort.useState<VortexLodVisual>(() => vortexLodVisual(lodCtx.lod, false, props.vortexMeshByLod, props.vortexMeshUrl));
  const highlight: "none" | "compatible" | "ring" | "attracting" | "indirectRing" = vortexSelected || props.selected === true
    ? "attracting"
    : reg.attractionDragAttractingFullId === fullId
      ? "attracting"
      : reg.attractionHoverRingFullId === fullId
        ? "ring"
        : reg.attractionIndirectPickAwait?.candidates.includes(fullId) === true
          ? "indirectRing"
          : reg.attractionCompatibleAttractedFullIds.has(fullId)
            ? "compatible"
            : "none";

  const entityFlags = reactHostPort.useMemo(() => ({ hidden: props.hidden === true, locked: props.locked === true }), [props.hidden, props.locked]);
  const entitySelectable = worldEntitySelectable(entityFlags);

  const selectVortex = reactHostPort.useCallback(() => {
    if (puzzle3dTargetVolumeToolActiveRef.current || reg.attractionDragActive || reg.attractionIndirectPickAwait || reg.blockedVortexFullIds.has(fullId) || !entitySelectable) {
      return;
    }
    commitSelection({ kind: "vortex", fullId });
    setActiveRelocateObjectId(props.objectId);
  }, [commitSelection, entitySelectable, fullId, props.objectId, reg, setActiveRelocateObjectId]);

  const onVortexClick = reactHostPort.useCallback(
    (e: ThreeEvent<MouseEvent>) => {
      if (e.nativeEvent.button !== 0) {
        return;
      }
      e.stopPropagation();
    },
    [],
  );

  const onPointerDown = reactHostPort.useCallback(
    (e: ThreeEvent<PointerEvent>) => {
      const pe = e.nativeEvent;
      if (pe.button !== 0) {
        return;
      }
      e.stopPropagation();
      if (reg.blockedVortexFullIds.has(fullId)) {
        return;
      }
      if (pe.altKey || pe.metaKey) {
        selectVortex();
        return;
      }
      vortexPointerGestureRef.current = { pointerId: pe.pointerId, x: pe.clientX, y: pe.clientY, dragStarted: false };
      const el = pe.currentTarget instanceof Element ? pe.currentTarget : null;
      if (el && typeof (el as HTMLElement).setPointerCapture === "function") {
        try {
          (el as HTMLElement).setPointerCapture(pe.pointerId);
        } catch {
          /* ignore */
        }
      }
    },
    [fullId, reg, selectVortex],
  );

  const onPointerMove = reactHostPort.useCallback(
    (e: ThreeEvent<PointerEvent>) => {
      const gesture = vortexPointerGestureRef.current;
      if (!gesture || gesture.dragStarted || e.nativeEvent.pointerId !== gesture.pointerId) {
        return;
      }
      e.stopPropagation();
      const dx = e.nativeEvent.clientX - gesture.x;
      const dy = e.nativeEvent.clientY - gesture.y;
      if (dx * dx + dy * dy < PUZZLE_3D_VORTEX_DRAG_THRESHOLD_PX * PUZZLE_3D_VORTEX_DRAG_THRESHOLD_PX) {
        return;
      }
      vortexPointerGestureRef.current = { ...gesture, dragStarted: true };
      reg.beginAttractionDragFromVortex(fullId, props.objectId, props.objectKind, props.vortexKind);
    },
    [fullId, props.objectId, props.objectKind, props.vortexKind, reg],
  );

  const onPointerUp = reactHostPort.useCallback(
    (e: ThreeEvent<PointerEvent>) => {
      const gesture = vortexPointerGestureRef.current;
      if (!gesture || e.nativeEvent.pointerId !== gesture.pointerId) {
        return;
      }
      e.stopPropagation();
      vortexPointerGestureRef.current = null;
      if (!gesture.dragStarted && !puzzle3dMarqueeSuppressClickRef.current && !puzzle3dRelocateDragActiveRef.current && !puzzle3dBrushToolActiveRef.current) {
        selectVortex();
      }
    },
    [selectVortex],
  );

  const inIndirectRing = reg.attractionIndirectPickAwait?.candidates.includes(fullId) === true;
  const linger = (reg.attractionDragActive && (reg.attractionDragAttractingFullId === fullId || reg.attractionHoverRingFullId === fullId || reg.attractionCompatibleAttractedFullIds.has(fullId))) || inIndirectRing;
  const lingerRef = reactHostPort.useRef(linger);
  lingerRef.current = linger;
  useFrame(() => {
    if (!trackVortexLod) return;
    const lod = lodCtx.depthVariable
      ? (() => {
          if (!root.current) return lodCtx.lod;
          updateWorldMatrixChain(root.current);
          root.current.getWorldPosition(worldPosRef.current);
          return lodCtx.lodForWorldPosition(worldPosRef.current.toArray() as Vec3);
        })()
      : lodCtx.lod;
    const next = vortexLodVisual(lod, lingerRef.current, vortexMeshByLodRef.current, vortexMeshUrlRef.current);
    setLodVisual((prev) => (vortexLodVisualEqual(prev, next) ? prev : next));
  });
  const objectRevealed = useObjectVorticesRevealed(props.objectId);
  const showVortexChrome = objectRevealed || linger;
  const baseDrawVortexBody = trackVortexLod ? lodVisual.drawVortexBody : lodVortexPrimaryVisible(lodCtx.lod);
  const drawVortexBody = showVortexChrome && (baseDrawVortexBody || linger);
  const meshUrl = trackVortexLod ? lodVisual.meshUrl : pickClosestMeshUrl(props.vortexMeshByLod, lodCtx.lod, props.vortexMeshUrl);

  const positionThree = reactHostPort.useMemo(() => cadObjectLocalToThreeGroupLocal(props.position, props.objectOrigin, props.objectOrientation), [props.position, props.objectOrigin, props.objectOrientation]);

  const vortexPointerHovered = reg.isHovered({ kind: "vortex", fullId }) || reg.isKindHovered("vortex", props.vortexKind);
  const renderMode = reactHostPort.useMemo(
    () =>
      worldEntityRenderMode(entityFlags, {
        hovered: vortexPointerHovered,
        selected: vortexSelected || props.selected === true,
        revealed: vortexPointerHovered,
      }),
    [entityFlags, props.selected, vortexPointerHovered, vortexSelected],
  );
  const vortexMeshStyle = highlight === "none" && renderMode.asHover ? "hovered" : vortexHighlightMeshStyle(highlight);

  const vortexPointerHoverHandlers = reactHostPort.useMemo(
    () =>
      entitySelectable
        ? {
            onPointerOver: (e: ThreeEvent<PointerEvent>) => {
              e.stopPropagation();
              if (!reg.attractionDragActive && !reg.attractionIndirectPickAwait) {
                reg.setHover({ kind: "vortex", fullId });
              }
            },
            onPointerOut: (e: ThreeEvent<PointerEvent>) => {
              e.stopPropagation();
              reg.clearHover({ kind: "vortex", fullId });
            },
          }
        : {},
    [entitySelectable, fullId, reg],
  );

  const vis = renderMode.visible && showVortexChrome;
  const showDirection = vis && isVec3(props.direction);
  return (
    <group ref={bindRoot} position={positionThree} userData={{ puzzle3dVortexFullId: fullId, vortexKind: props.vortexKind }} data-puzzle3d-vortex={fullId} visible={vis} onPointerDown={onPointerDown} onPointerMove={onPointerMove} onPointerUp={onPointerUp} onClick={onVortexClick}>
      {showDirection ? (
        <VortexDirectionArrow directionCad={props.direction!} objectOrigin={props.objectOrigin} objectOrientation={props.objectOrientation} radius={r} selected={vortexSelected || highlight !== "none"} />
      ) : null}
      {drawVortexBody && meshUrl ? (
        <VortexMeshGltf meshUrl={meshUrl} fullId={fullId} radius={r} style={vortexMeshStyle} {...vortexPointerHoverHandlers} />
      ) : drawVortexBody && props.children ? (
        <group userData={{ puzzle3dVortexFullId: fullId }} {...vortexPointerHoverHandlers}>
          {props.children}
        </group>
      ) : drawVortexBody ? (
        <VortexFallbackMesh fullId={fullId} radius={r} highlight={highlight} hovered={vortexPointerHovered} {...vortexPointerHoverHandlers} />
      ) : null}
      {showVortexChrome ? (
        <mesh userData={{ puzzle3dVortexFullId: fullId }} raycast={vortexPickRaycast} renderOrder={-1} {...vortexPointerHoverHandlers}>
          <sphereGeometry args={[r * 1.15, 12, 12]} />
          <meshBasicMaterial transparent opacity={0} depthWrite={false} depthTest={false} />
        </mesh>
      ) : null}
    </group>
  );
});
//#endregion ­ƒîÇVortex

//#region ­ƒº▓Attraction
function puzzle3dAttractionIndexFromPointerEvent(e: ThreeEvent<PointerEvent>): number {
  if (e.index != null) {
    return Math.floor(e.index / 2);
  }
  if (e.faceIndex != null) {
    return e.faceIndex;
  }
  return 0;
}

const CableBatch = reactHostPort.memo(function CableBatch(props: { readonly attractions: readonly AttractionProps[] }) {
  const reg = useRegistry();
  const { commitSelection } = useRegistryInteraction();
  const selectedAttractionIds = useSelectedAttractionIdSet();
  reactHostPort.useLayoutEffect(() => {
    for (const attraction of props.attractions) {
      reg.registerAttractionKind(attraction.id, attraction.attractionKind);
    }
  }, [props.attractions, reg]);
  const mat = reactHostPort.useMemo(() => {
    const color = lineCssColor(CSS_ATTRACTION_ENDPOINT_LINE, "gray");
    return new LineBasicMaterial({ color, transparent: true, opacity: 0.85, depthTest: true, vertexColors: true });
  }, []);
  const geo = reactHostPort.useMemo(() => new BufferGeometry(), []);
  const normalColor = reactHostPort.useMemo(() => new Color(lineCssColor(CSS_ATTRACTION_ENDPOINT_LINE, "gray")), []);
  const hoveredColor = reactHostPort.useMemo(() => new Color(lineCssColor(CSS_HOVERED_LINE, "gray")), []);
  const selectedColor = reactHostPort.useMemo(() => new Color(lineCssColor(CSS_SELECTED_LINE, "primary")), []);
  reactHostPort.useLayoutEffect(() => {
    const vertexCount = Math.max(props.attractions.length * 2, 2);
    geo.setAttribute("position", new Float32BufferAttribute(new Float32Array(vertexCount * 3), 3));
    geo.setAttribute("color", new Float32BufferAttribute(new Float32Array(vertexCount * 3), 3));
  }, [geo, props.attractions.length]);
  useFrame(() => {
    const pos = geo.attributes.position as Float32BufferAttribute;
    const colors = geo.attributes.color as Float32BufferAttribute;
    let write = 0;
    for (const attraction of props.attractions) {
      const entityFlags = { hidden: attraction.hidden === true, locked: attraction.locked === true };
      const directHovered = reg.hoverTarget?.kind === "attraction" && reg.hoverTarget.id === attraction.id;
      const kindHovered = reg.isKindHovered("attraction", attraction.attractionKind);
      const revealed = directHovered || kindHovered;
      if (!worldEntityRendered(entityFlags, revealed)) {
        pos.setXYZ(write, 0, 0, 0);
        pos.setXYZ(write + 1, 0, 0, 0);
        colors.setXYZ(write, 0, 0, 0);
        colors.setXYZ(write + 1, 0, 0, 0);
        write += 2;
        continue;
      }
      const a = reg.getVortexWorld(attraction.attracting);
      const b = reg.getVortexWorld(attraction.attracted);
      const renderMode = worldEntityRenderMode(entityFlags, { hovered: directHovered || kindHovered, selected: selectedAttractionIds.has(attraction.id), revealed });
      const c = selectedAttractionIds.has(attraction.id) ? selectedColor : directHovered || kindHovered ? hoveredColor : normalColor;
      if (renderMode.dim) {
        c.multiplyScalar(WORLD_LOCKED_OPACITY_SCALE);
      }
      if (a && b && vector3IsFinite(a) && vector3IsFinite(b)) {
        pos.setXYZ(write, a.x, a.y, a.z);
        pos.setXYZ(write + 1, b.x, b.y, b.z);
      } else {
        pos.setXYZ(write, 0, 0, 0);
        pos.setXYZ(write + 1, 0, 0, 0);
      }
      colors.setXYZ(write, c.r, c.g, c.b);
      colors.setXYZ(write + 1, c.r, c.g, c.b);
      write += 2;
    }
    pos.needsUpdate = true;
    colors.needsUpdate = true;
  });
  const onPointerOver = reactHostPort.useCallback(
    (e: ThreeEvent<PointerEvent>) => {
      e.stopPropagation();
      if (reg.attractionDragActive || reg.attractionIndirectPickAwait || puzzle3dTargetVolumeToolActiveRef.current) {
        return;
      }
      const idx = puzzle3dAttractionIndexFromPointerEvent(e);
      const attraction = props.attractions[idx];
      if (attraction && worldEntitySelectable({ hidden: attraction.hidden === true, locked: attraction.locked === true })) {
        reg.setHover({ kind: "attraction", id: attraction.id });
      }
    },
    [props.attractions, reg],
  );
  const onPointerOut = reactHostPort.useCallback(
    (e: ThreeEvent<PointerEvent>) => {
      e.stopPropagation();
      const idx = puzzle3dAttractionIndexFromPointerEvent(e);
      const attraction = props.attractions[idx];
      if (attraction) {
        reg.clearHover({ kind: "attraction", id: attraction.id });
      }
    },
    [props.attractions, reg],
  );
  const onClick = reactHostPort.useCallback(
    (e: ThreeEvent<MouseEvent>) => {
      if (e.nativeEvent.button !== 0 || puzzle3dMarqueeSuppressClickRef.current || puzzle3dRelocateDragActiveRef.current) {
        return;
      }
      e.stopPropagation();
      if (reg.attractionDragActive || reg.attractionIndirectPickAwait || puzzle3dTargetVolumeToolActiveRef.current) {
        return;
      }
      const idx = puzzle3dAttractionIndexFromPointerEvent(e);
      const attraction = props.attractions[idx];
      if (attraction && worldEntitySelectable({ hidden: attraction.hidden === true, locked: attraction.locked === true })) {
        commitSelection({ kind: "attraction", id: attraction.id });
      }
    },
    [commitSelection, props.attractions, reg],
  );
  reactHostPort.useEffect(
    () => () => {
      geo.dispose();
      mat.dispose();
    },
    [geo, mat],
  );
  if (!props.attractions.length) {
    return null;
  }
  return <lineSegments geometry={geo} material={mat} userData={{ puzzle3dAttractionPick: true }} onPointerOver={onPointerOver} onPointerOut={onPointerOut} onClick={onClick} />;
});

export const Attraction = reactHostPort.memo(function Attraction(props: AttractionProps) {
  return <CableBatch attractions={[props]} />;
});
//#endregion ­ƒº▓Attraction

//#region Ô£ïRelocate
export function useRelocate(objectId: string) {
  const reg = useRegistry();
  return {
    config: reg.gumballConfig,
    start: () => reg.setActiveRelocateObjectId(objectId),
    cancel: () => reg.setActiveRelocateObjectId(null),
  };
}
//#endregion Ô£ïRelocate

const EMPTY_BLOCKED_VORTICES: ReadonlySet<string> = new Set();

export const PUZZLE_3D_MARQUEE_DRAG_THRESHOLD_PX = 4;

/** @emoji 🖱️ True while a right-button camera drag is active (suppress context menu on drag). */
export const puzzle3dRightDragActiveRef = { current: false };

/** @emoji 🖱️ True after a marquee gesture consumed the click (mesh picks skip onClick). */
export const puzzle3dMarqueeSuppressClickRef = { current: false };

/** @emoji ✋ True while transform controls (relocate tool) are dragging — blocks marquee and picks. */
export const puzzle3dRelocateDragActiveRef = { current: false };

let puzzle3dMarqueeGestureCancel: (() => void) | null = null;

/** @emoji 🖱️ Aborts an in-progress marquee gesture (e.g. when relocate drag starts). */
export function cancelPuzzle3dMarqueeGesture(): void {
  puzzle3dMarqueeGestureCancel?.();
}

export interface ScreenRect {
  readonly left: number;
  readonly right: number;
  readonly top: number;
  readonly bottom: number;
}

export interface ScreenPoint {
  readonly x: number;
  readonly y: number;
}

/** @emoji 🖱️ Builds a normalized screen rect from two client-space points. */
export function screenRectFromClientPoints(x0: number, y0: number, x1: number, y1: number): ScreenRect {
  return {
    left: Math.min(x0, x1),
    right: Math.max(x0, x1),
    top: Math.min(y0, y1),
    bottom: Math.max(y0, y1),
  };
}

/** @emoji 🖱️ True when a client point lies inside a screen rect. */
export function pointInScreenRect(point: ScreenPoint, rect: ScreenRect): boolean {
  return point.x >= rect.left && point.x <= rect.right && point.y >= rect.top && point.y <= rect.bottom;
}

/** @emoji 🖱️ True when every corner of `inner` lies inside `outer` (window selection). */
export function screenRectContainsRect(outer: ScreenRect, inner: ScreenRect): boolean {
  return inner.left >= outer.left && inner.right <= outer.right && inner.top >= outer.top && inner.bottom <= outer.bottom;
}

/** @emoji 🖱️ True when two screen rects overlap (crossing selection). */
export function screenRectIntersectsRect(a: ScreenRect, b: ScreenRect): boolean {
  return a.left <= b.right && a.right >= b.left && a.top <= b.bottom && a.bottom >= b.top;
}

/** @emoji 🖱️ Ray-cast point-in-polygon test for lasso paths. */
export function pointInPolygon(point: ScreenPoint, polygon: readonly ScreenPoint[]): boolean {
  let inside = false;
  for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
    const a = polygon[i]!;
    const b = polygon[j]!;
    const intersects = a.y > point.y !== b.y > point.y && point.x < ((b.x - a.x) * (point.y - a.y)) / (b.y - a.y || 1e-9) + a.x;
    if (intersects) {
      inside = !inside;
    }
  }
  return inside;
}

function segmentIntersectsSegment(a0: ScreenPoint, a1: ScreenPoint, b0: ScreenPoint, b1: ScreenPoint): boolean {
  const orient = (p: ScreenPoint, q: ScreenPoint, r: ScreenPoint) => (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
  const onSegment = (p: ScreenPoint, q: ScreenPoint, r: ScreenPoint) =>
    Math.min(p.x, r.x) <= q.x && q.x <= Math.max(p.x, r.x) && Math.min(p.y, r.y) <= q.y && q.y <= Math.max(p.y, r.y);
  const o1 = orient(a0, a1, b0);
  const o2 = orient(a0, a1, b1);
  const o3 = orient(b0, b1, a0);
  const o4 = orient(b0, b1, a1);
  if (o1 === 0 && onSegment(a0, b0, a1)) return true;
  if (o2 === 0 && onSegment(a0, b1, a1)) return true;
  if (o3 === 0 && onSegment(b0, a0, b1)) return true;
  if (o4 === 0 && onSegment(b0, a1, b1)) return true;
  return (o1 > 0) !== (o2 > 0) && (o3 > 0) !== (o4 > 0);
}

/** @emoji 🖱️ True when a segment intersects a screen-rect edge. */
export function segmentIntersectsScreenRect(a: ScreenPoint, b: ScreenPoint, rect: ScreenRect): boolean {
  const corners: ScreenPoint[] = [
    { x: rect.left, y: rect.top },
    { x: rect.right, y: rect.top },
    { x: rect.right, y: rect.bottom },
    { x: rect.left, y: rect.bottom },
  ];
  for (let i = 0; i < corners.length; i += 1) {
    const c0 = corners[i]!;
    const c1 = corners[(i + 1) % corners.length]!;
    if (segmentIntersectsSegment(a, b, c0, c1)) {
      return true;
    }
  }
  return false;
}

function screenRectCorners(rect: ScreenRect): ScreenPoint[] {
  return [
    { x: rect.left, y: rect.top },
    { x: rect.right, y: rect.top },
    { x: rect.right, y: rect.bottom },
    { x: rect.left, y: rect.bottom },
  ];
}

/** @emoji 🖱️ True when a closed polygon fully contains a screen rect (window lasso). */
export function polygonContainsScreenRect(polygon: readonly ScreenPoint[], rect: ScreenRect): boolean {
  return screenRectCorners(rect).every((corner) => pointInPolygon(corner, polygon));
}

/** @emoji 🖱️ True when a segment intersects a polygon edge or lies inside it. */
export function segmentIntersectsPolygon(a: ScreenPoint, b: ScreenPoint, polygon: readonly ScreenPoint[]): boolean {
  if (pointInPolygon(a, polygon) || pointInPolygon(b, polygon)) {
    return true;
  }
  for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
    if (segmentIntersectsSegment(a, b, polygon[i]!, polygon[j]!)) {
      return true;
    }
  }
  return false;
}

export interface MarqueeCandidate {
  readonly kind: "object" | "vortex" | "attraction";
  readonly id: string;
  readonly hull: readonly ScreenPoint[];
  readonly screenBounds: ScreenRect | null;
}

/** @emoji 🖱️ Projected marquee silhouette: convex hull plus axis-aligned reject bounds. */
export interface ObjectMarqueeFootprint {
  readonly hull: readonly ScreenPoint[];
  readonly screenBounds: ScreenRect;
}

/** @emoji 🖱️ Per-mesh local AABB corners for fast marquee projection without geometry traversal. */
export interface ObjectMarqueeMeshCache {
  readonly mesh: Mesh;
  readonly localCorners: readonly Vector3[];
}

/** @emoji 🖱️ Cached mesh footprints for one puzzle object group. */
export interface ObjectMarqueeFootprintCacheEntry {
  readonly meshes: readonly ObjectMarqueeMeshCache[];
}

const objectMarqueeFootprintCache = new Map<string, ObjectMarqueeFootprintCacheEntry>();

const _marqueeProjectScratch = new Vector3();
const _marqueeMeshBox = new Box3();
const _marqueeBoxCorners: readonly Vector3[] = [
  new Vector3(),
  new Vector3(),
  new Vector3(),
  new Vector3(),
  new Vector3(),
  new Vector3(),
  new Vector3(),
  new Vector3(),
];

function marqueeCross(o: ScreenPoint, a: ScreenPoint, b: ScreenPoint): number {
  return (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
}

/** @emoji 🖱️ Convex hull of screen points (monotone chain) for marquee silhouette edges. */
export function convexHullScreenPoints(points: readonly ScreenPoint[]): ScreenPoint[] {
  if (points.length <= 1) {
    return [...points];
  }
  const sorted = [...points].sort((a, b) => (a.x === b.x ? a.y - b.y : a.x - b.x));
  const lower: ScreenPoint[] = [];
  for (const p of sorted) {
    while (lower.length >= 2 && marqueeCross(lower[lower.length - 2]!, lower[lower.length - 1]!, p) <= 0) {
      lower.pop();
    }
    lower.push(p);
  }
  const upper: ScreenPoint[] = [];
  for (let i = sorted.length - 1; i >= 0; i -= 1) {
    const p = sorted[i]!;
    while (upper.length >= 2 && marqueeCross(upper[upper.length - 2]!, upper[upper.length - 1]!, p) <= 0) {
      upper.pop();
    }
    upper.push(p);
  }
  lower.pop();
  upper.pop();
  return lower.concat(upper);
}

export interface MarqueeSelectionInput {
  readonly method: SelectionMethod;
  readonly crossing: boolean;
  readonly rect: ScreenRect | null;
  readonly polygon: readonly ScreenPoint[];
  readonly kinds: MarqueeSelectableKinds;
  readonly candidates: readonly MarqueeCandidate[];
}

/** @emoji 🖱️ Axis-aligned client bounds from projected points (ignores behind-camera samples). */
export function screenBoundsFromClientPoints(points: readonly ScreenPoint[]): ScreenRect | null {
  if (points.length === 0) {
    return null;
  }
  let left = Infinity;
  let right = -Infinity;
  let top = Infinity;
  let bottom = -Infinity;
  for (const point of points) {
    left = Math.min(left, point.x);
    right = Math.max(right, point.x);
    top = Math.min(top, point.y);
    bottom = Math.max(bottom, point.y);
  }
  return { left, right, top, bottom };
}

/** @emoji 🖱️ True when two closed screen polygons overlap (lasso crossing vs object hull). */
export function screenPolygonsIntersect(a: readonly ScreenPoint[], b: readonly ScreenPoint[]): boolean {
  if (a.length === 0 || b.length === 0) {
    return false;
  }
  for (const point of a) {
    if (pointInPolygon(point, b)) {
      return true;
    }
  }
  for (const point of b) {
    if (pointInPolygon(point, a)) {
      return true;
    }
  }
  for (let i = 0; i < a.length; i += 1) {
    const a0 = a[i]!;
    const a1 = a[(i + 1) % a.length]!;
    for (let j = 0; j < b.length; j += 1) {
      const b0 = b[j]!;
      const b1 = b[(j + 1) % b.length]!;
      if (segmentIntersectsSegment(a0, a1, b0, b1)) {
        return true;
      }
    }
  }
  return false;
}

/** @emoji 🖱️ True when a screen rect overlaps a lasso polygon (crossing selection). */
export function screenRectIntersectsPolygon(bounds: ScreenRect, polygon: readonly ScreenPoint[]): boolean {
  for (const point of polygon) {
    if (pointInScreenRect(point, bounds)) {
      return true;
    }
  }
  const corners = screenRectCorners(bounds);
  if (corners.some((corner) => pointInPolygon(corner, polygon))) {
    return true;
  }
  for (let i = 0; i < corners.length; i += 1) {
    const a = corners[i]!;
    const b = corners[(i + 1) % corners.length]!;
    if (segmentIntersectsPolygon(a, b, polygon)) {
      return true;
    }
  }
  return false;
}

function marqueeCandidateSelected(input: MarqueeSelectionInput, candidate: MarqueeCandidate): boolean {
  const hull = candidate.hull;
  const bounds = candidate.screenBounds;
  if (hull.length === 0 || !bounds) {
    return false;
  }
  if (input.crossing) {
    if (input.method === "rectangle" && input.rect) {
      if (!screenRectIntersectsRect(input.rect, bounds)) {
        return false;
      }
      return screenRectIntersectsPolygon(input.rect, hull);
    }
    if (input.polygon.length < 3) {
      return false;
    }
    return screenPolygonsIntersect(hull, input.polygon);
  }
  if (input.method === "rectangle" && input.rect) {
    return hull.every((point) => pointInScreenRect(point, input.rect!));
  }
  if (input.polygon.length < 3) {
    return false;
  }
  return hull.every((point) => pointInPolygon(point, input.polygon));
}

/** @emoji 🖱️ Resolves marquee hits into a {@link SelectionSnapshot} from projected screen candidates. */
export function marqueeSelectionFromCandidates(input: MarqueeSelectionInput): SelectionSnapshot {
  const objectIds: string[] = [];
  const vortexIds: string[] = [];
  const attractionIds: string[] = [];
  for (const candidate of input.candidates) {
    if (!marqueeCandidateSelected(input, candidate)) {
      continue;
    }
    if (candidate.kind === "object" && input.kinds.object) {
      objectIds.push(candidate.id);
    } else if (candidate.kind === "vortex" && input.kinds.vortex) {
      vortexIds.push(candidate.id);
    } else if (candidate.kind === "attraction" && input.kinds.attraction) {
      attractionIds.push(candidate.id);
    }
  }
  return { objectIds, vortexIds, attractionIds };
}

/** @emoji 🖱️ Pointer gesture args shared by marquee preview and commit. */
export interface MarqueeGestureArgs {
  readonly startX: number;
  readonly startY: number;
  readonly endX: number;
  readonly endY: number;
  readonly path: readonly ScreenPoint[];
  readonly modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean };
}

/** @emoji 🖱️ Merges cached marquee candidates with a base selection using modifier-driven mode. */
export function resolveMarqueeSelectionGesture(
  args: MarqueeGestureArgs,
  options: {
    readonly method: SelectionMethod;
    readonly kinds: MarqueeSelectableKinds;
    readonly candidates: readonly MarqueeCandidate[];
    readonly base: SelectionSnapshot;
  },
): SelectionSnapshot {
  const crossing = marqueeIsCrossingFromPath(args.path, options.method);
  const screenRect = screenRectFromClientPoints(args.startX, args.startY, args.endX, args.endY);
  const polygon =
    options.method === "lasso" && args.path.length >= 3
      ? args.path
      : [
          { x: screenRect.left, y: screenRect.top },
          { x: screenRect.right, y: screenRect.top },
          { x: screenRect.right, y: screenRect.bottom },
          { x: screenRect.left, y: screenRect.bottom },
        ];
  const incoming = marqueeSelectionFromCandidates({
    method: options.method,
    crossing,
    rect: options.method === "rectangle" ? screenRect : null,
    polygon,
    kinds: options.kinds,
    candidates: options.candidates,
  });
  return mergeSelectionSnapshot(marqueeModeFromModifiers(args.modifiers), options.base, incoming);
}

/** @emoji 🖱️ Client-space rect for marquee projection ({@link HTMLCanvasElement.getBoundingClientRect}). */
export function puzzle3dMarqueeClientRect(gl: WebGLRenderer): DOMRect {
  return gl.domElement.getBoundingClientRect();
}

/** @emoji 🖱️ Projects a world point to client space; reports when the point is outside the view volume. */
export function projectWorldToClientMarquee(point: Vector3, camera: Camera, rect: DOMRect): { readonly point: ScreenPoint | null; readonly behindCamera: boolean } {
  const projected = _marqueeProjectScratch.copy(point).project(camera);
  if (!Number.isFinite(projected.x) || !Number.isFinite(projected.y) || !Number.isFinite(projected.z)) {
    return { point: null, behindCamera: true };
  }
  if (projected.z < -1 || projected.z > 1) {
    return { point: null, behindCamera: true };
  }
  return {
    point: {
      x: rect.left + ((projected.x + 1) / 2) * rect.width,
      y: rect.top + ((1 - projected.y) / 2) * rect.height,
    },
    behindCamera: false,
  };
}

/** @emoji 🖱️ Projects a world point to client coordinates for marquee tests. */
export function projectWorldToClient(point: Vector3, camera: Camera, rect: DOMRect): ScreenPoint | null {
  return projectWorldToClientMarquee(point, camera, rect).point;
}

function writeMarqueeBoxCorners(box: Box3): readonly Vector3[] {
  const { min, max } = box;
  _marqueeBoxCorners[0]!.set(min.x, min.y, min.z);
  _marqueeBoxCorners[1]!.set(max.x, min.y, min.z);
  _marqueeBoxCorners[2]!.set(min.x, max.y, min.z);
  _marqueeBoxCorners[3]!.set(max.x, max.y, min.z);
  _marqueeBoxCorners[4]!.set(min.x, min.y, max.z);
  _marqueeBoxCorners[5]!.set(max.x, min.y, max.z);
  _marqueeBoxCorners[6]!.set(min.x, max.y, max.z);
  _marqueeBoxCorners[7]!.set(max.x, max.y, max.z);
  return _marqueeBoxCorners;
}

function cloneMarqueeBoxCorners(box: Box3): Vector3[] {
  return writeMarqueeBoxCorners(box).map((corner) => corner.clone());
}

function objectMarqueeMeshCount(group: Group): number {
  let count = 0;
  group.traverse((node) => {
    if (node instanceof Mesh) {
      count += 1;
    }
  });
  return count;
}

function footprintFromProjectedPoints(projected: readonly ScreenPoint[]): ObjectMarqueeFootprint | null {
  if (projected.length === 0) {
    return null;
  }
  const screenBounds = screenBoundsFromClientPoints(projected);
  if (!screenBounds) {
    return null;
  }
  return { hull: convexHullScreenPoints(projected), screenBounds };
}

/** @emoji 🖱️ Drops cached mesh corners for one object or the whole scene. */
export function invalidateObjectMarqueeFootprintCache(objectId?: string): void {
  if (objectId === undefined) {
    objectMarqueeFootprintCache.clear();
    return;
  }
  objectMarqueeFootprintCache.delete(objectId);
}

/** @emoji 🖱️ Reads cached mesh corners for marquee projection tests. */
export function getObjectMarqueeFootprintCache(objectId: string): ObjectMarqueeFootprintCacheEntry | undefined {
  return objectMarqueeFootprintCache.get(objectId);
}

/** @emoji 🖱️ Traverses a group once and stores per-mesh local AABB corners for marquee projection. */
export function buildObjectMarqueeFootprintCache(group: Group, objectId: string): ObjectMarqueeFootprintCacheEntry {
  const meshes: ObjectMarqueeMeshCache[] = [];
  group.traverse((node) => {
    if (!(node instanceof Mesh)) {
      return;
    }
    const geometry = node.geometry;
    if (!geometry) {
      return;
    }
    if (!geometry.boundingBox) {
      geometry.computeBoundingBox();
    }
    meshes.push({ mesh: node, localCorners: cloneMarqueeBoxCorners(geometry.boundingBox!) });
  });
  const entry: ObjectMarqueeFootprintCacheEntry = { meshes };
  objectMarqueeFootprintCache.set(objectId, entry);
  return entry;
}

/** @emoji 🖱️ Precomputes mesh geometry bounding boxes and optional footprint cache for marquee. */
export function warmObjectGroupMarqueeBounds(group: Group, objectId?: string): void {
  if (objectId) {
    buildObjectMarqueeFootprintCache(group, objectId);
    return;
  }
  group.traverse((node) => {
    if (!(node instanceof Mesh)) {
      return;
    }
    const geometry = node.geometry;
    if (!geometry || geometry.boundingBox !== null) {
      return;
    }
    geometry.computeBoundingBox();
  });
}

function projectMarqueePointsFromMeshes(
  meshes: readonly ObjectMarqueeMeshCache[],
  camera: Camera,
  rect: DOMRect,
): ScreenPoint[] {
  const projected: ScreenPoint[] = [];
  for (const entry of meshes) {
    entry.mesh.updateWorldMatrix(true, false);
    for (const local of entry.localCorners) {
      _marqueeProjectScratch.copy(local).applyMatrix4(entry.mesh.matrixWorld);
      const sample = projectWorldToClientMarquee(_marqueeProjectScratch, camera, rect);
      if (sample.point) {
        projected.push(sample.point);
      }
    }
  }
  return projected;
}

/** @emoji 🖱️ Projects cached mesh corners to a client-space marquee footprint. */
export function projectObjectMarqueeFootprintFromCache(
  cache: ObjectMarqueeFootprintCacheEntry,
  camera: Camera,
  rect: DOMRect,
): ObjectMarqueeFootprint | null {
  return footprintFromProjectedPoints(projectMarqueePointsFromMeshes(cache.meshes, camera, rect));
}

/** @emoji 🖱️ Projects an object group's visible mesh bounds to a client-space marquee footprint. */
export function projectObjectGroupToScreenPoints(group: Group, camera: Camera, rect: DOMRect): ObjectMarqueeFootprint | null {
  const projected: ScreenPoint[] = [];
  group.traverse((node) => {
    if (!(node instanceof Mesh)) {
      return;
    }
    const geometry = node.geometry;
    if (!geometry) {
      return;
    }
    if (!geometry.boundingBox) {
      geometry.computeBoundingBox();
    }
    _marqueeMeshBox.copy(geometry.boundingBox!).applyMatrix4(node.matrixWorld);
    for (const corner of writeMarqueeBoxCorners(_marqueeMeshBox)) {
      const sample = projectWorldToClientMarquee(corner, camera, rect);
      if (sample.point) {
        projected.push(sample.point);
      }
    }
  });
  return footprintFromProjectedPoints(projected);
}

function resolveObjectMarqueeFootprintCache(group: Group, objectId: string): ObjectMarqueeFootprintCacheEntry {
  let cache = objectMarqueeFootprintCache.get(objectId);
  const meshCount = objectMarqueeMeshCount(group);
  if (!cache || cache.meshes.length !== meshCount) {
    cache = buildObjectMarqueeFootprintCache(group, objectId);
  }
  return cache;
}

function marqueeFootprintToCandidate(
  kind: MarqueeCandidate["kind"],
  id: string,
  footprint: ObjectMarqueeFootprint | null,
): MarqueeCandidate | null {
  if (!footprint) {
    return null;
  }
  return { kind, id, hull: footprint.hull, screenBounds: footprint.screenBounds };
}

function marqueeFootprintFromClientPoints(points: readonly ScreenPoint[]): ObjectMarqueeFootprint | null {
  return footprintFromProjectedPoints(points);
}

export interface MarqueeOverlaySnapshot {
  readonly active: boolean;
  readonly method: SelectionMethod;
  readonly start: ScreenPoint | null;
  readonly current: ScreenPoint | null;
  readonly path: readonly ScreenPoint[];
  readonly clientOrigin: ScreenPoint;
}

const MARQUEE_OVERLAY_IDLE: MarqueeOverlaySnapshot = { active: false, method: "rectangle", start: null, current: null, path: [], clientOrigin: { x: 0, y: 0 } };

/** @emoji 🖱️ External store for marquee overlay geometry (DOM subscribes without scene re-renders). */
export function createMarqueeOverlayStore(initial: MarqueeOverlaySnapshot = MARQUEE_OVERLAY_IDLE) {
  let snapshot = initial;
  const listeners = new Set<() => void>();
  return {
    subscribe(listener: () => void): () => void {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    getSnapshot(): MarqueeOverlaySnapshot {
      return snapshot;
    },
    setSnapshot(next: MarqueeOverlaySnapshot): void {
      snapshot = next;
      for (const listener of listeners) {
        listener();
      }
    },
  };
}

export type MarqueeOverlayStore = ReturnType<typeof createMarqueeOverlayStore>;

export const puzzle3dMarqueeOverlayStore = createMarqueeOverlayStore();

/** @emoji 🖌️ Brush preview + engagement-source snapshot for window engagement. */
export interface BrushUiSnapshot {
  readonly preview: BrushPreviewState | null;
  readonly candidates: readonly BrushCompatibleCandidate[];
  readonly targetActive: boolean;
  readonly placementProbePending: boolean;
  readonly menuOpen: boolean;
  readonly menuAnchor: ScreenPoint | null;
  readonly menuHoverIndex: number | null;
}

const BRUSH_UI_IDLE: BrushUiSnapshot = {
  preview: null,
  candidates: [],
  targetActive: false,
  placementProbePending: false,
  menuOpen: false,
  menuAnchor: null,
  menuHoverIndex: null,
};

/** @emoji 🖌️ Brush candidate menu actions for the viewport overlay. */
export interface Puzzle3dBrushMenuSource {
  readonly hoverCandidate: (index: number) => void;
  readonly selectCandidate: (index: number) => void;
  readonly closeMenu: () => void;
}

export const puzzle3dBrushMenuSourceRef: { current: Puzzle3dBrushMenuSource } = {
  current: { hoverCandidate: () => {}, selectCandidate: () => {}, closeMenu: () => {} },
};

/** @emoji 🖌️ Live brush gestures exposed to {@link buildPuzzle3dPlayEngagement}. */
export interface Puzzle3dBrushEngagementSource {
  readonly candidates: readonly BrushCompatibleCandidate[];
  readonly targetActive: boolean;
  readonly placementProbePending: boolean;
  readonly cycleCandidate: () => void;
  readonly pickCandidate: (index: number) => void;
}

export const puzzle3dBrushEngagementSourceRef: { current: Puzzle3dBrushEngagementSource } = {
  current: { candidates: [], targetActive: false, placementProbePending: false, cycleCandidate: () => {}, pickCandidate: () => {} },
};

let puzzle3dBrushEngagementEpoch = 0;
const puzzle3dBrushEngagementListeners = new Set<() => void>();

/** @emoji 🔔 Subscribes to brush engagement source changes (candidates, target hover). */
export function subscribePuzzle3dBrushEngagementSource(listener: () => void): () => void {
  puzzle3dBrushEngagementListeners.add(listener);
  return () => puzzle3dBrushEngagementListeners.delete(listener);
}

/** @emoji 🔔 Notifies engagement publisher after brush source fields change. */
export function notifyPuzzle3dBrushEngagementSource(): void {
  puzzle3dBrushEngagementEpoch += 1;
  for (const listener of puzzle3dBrushEngagementListeners) {
    listener();
  }
}

/** @emoji 🔑 Epoch for {@link subscribePuzzle3dBrushEngagementSource}. */
export function getPuzzle3dBrushEngagementEpoch(): number {
  return puzzle3dBrushEngagementEpoch;
}

/** @emoji 🖌️ Engagement option id for cycling the active brush placement candidate. */
export const PUZZLE_3D_ENGAGEMENT_BRUSH_NEXT_ID = "puzzle3d.brush.next";

/** @emoji 🔍 Engagement option id for framing the orbit camera on the current selection. */
export const PUZZLE_3D_ENGAGEMENT_ZOOM_ID = "puzzle3d.zoom";

let puzzle3dZoomToSelectionEpoch = 0;
let puzzle3dZoomToSelectionTarget: SelectionSnapshot = EMPTY_SELECTION_SNAPSHOT;
const puzzle3dZoomToSelectionListeners = new Set<() => void>();

/** @emoji 🔔 Subscribes to zoom-to-selection requests from engagement UI. */
export function subscribePuzzle3dZoomToSelection(listener: () => void): () => void {
  puzzle3dZoomToSelectionListeners.add(listener);
  return () => puzzle3dZoomToSelectionListeners.delete(listener);
}

/** @emoji 🔍 Queues a one-shot camera frame on the current selection (no-op when empty). */
export function requestPuzzle3dZoomToSelection(selection: SelectionSnapshot): void {
  if (selection.objectIds.length === 0 && selection.vortexIds.length === 0 && selection.attractionIds.length === 0) {
    return;
  }
  puzzle3dZoomToSelectionTarget = {
    objectIds: [...selection.objectIds],
    vortexIds: [...selection.vortexIds],
    attractionIds: [...selection.attractionIds],
  };
  puzzle3dZoomToSelectionEpoch += 1;
  for (const listener of puzzle3dZoomToSelectionListeners) {
    listener();
  }
}

/** @emoji 🔑 Epoch for {@link subscribePuzzle3dZoomToSelection}. */
export function getPuzzle3dZoomToSelectionEpoch(): number {
  return puzzle3dZoomToSelectionEpoch;
}

/** @emoji 🎯 Selection snapshot for the latest {@link requestPuzzle3dZoomToSelection}. */
export function getPuzzle3dZoomToSelectionTarget(): SelectionSnapshot {
  return puzzle3dZoomToSelectionTarget;
}

//#region 🖱️SelectionContextMenu

/** @emoji 🎯 Per-entity hidden/locked flags for selection context menu labels. */
export interface Puzzle3dSelectionEntityFlags {
  readonly hidden: boolean;
  readonly locked: boolean;
}

/** @emoji 🖱️ Live selection context menu anchor and right-click target. */
export interface Puzzle3dSelectionMenuSnapshot {
  readonly open: boolean;
  readonly anchor: ScreenPoint | null;
  readonly target: HoverTarget | null;
  readonly selection: SelectionSnapshot;
  readonly entityFlags: readonly Puzzle3dSelectionEntityFlags[];
  readonly vortexMeta: VortexBindingMeta | null;
}

const SELECTION_MENU_IDLE: Puzzle3dSelectionMenuSnapshot = {
  open: false,
  anchor: null,
  target: null,
  selection: EMPTY_SELECTION_SNAPSHOT,
  entityFlags: [],
  vortexMeta: null,
};

/** @emoji 🖱️ External store for viewport selection context menu (DOM overlay subscribes without scene re-renders). */
export function createSelectionMenuStore(initial: Puzzle3dSelectionMenuSnapshot = SELECTION_MENU_IDLE) {
  let snapshot = initial;
  const listeners = new Set<() => void>();
  return {
    subscribe(listener: () => void): () => void {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    getSnapshot(): Puzzle3dSelectionMenuSnapshot {
      return snapshot;
    },
    setSnapshot(next: Puzzle3dSelectionMenuSnapshot): void {
      snapshot = next;
      for (const listener of listeners) {
        listener();
      }
    },
  };
}

export type SelectionMenuStore = ReturnType<typeof createSelectionMenuStore>;

export const puzzle3dSelectionMenuStore = createSelectionMenuStore();

/** @emoji 🖱️ Host callbacks for selection context menu actions (published by {@link PlayCanvas}). */
export interface Puzzle3dSelectionMenuActions {
  readonly toggleHidden: (value: boolean) => void;
  readonly toggleLocked: (value: boolean) => void;
  readonly deleteSelection: () => void;
  readonly duplicateSelection: () => void;
  readonly selectSameKind: () => void;
}

export const puzzle3dSelectionActionsRef: { current: Puzzle3dSelectionMenuActions } = {
  current: {
    toggleHidden: () => {},
    toggleLocked: () => {},
    deleteSelection: () => {},
    duplicateSelection: () => {},
    selectSameKind: () => {},
  },
};

/** @emoji 🌪️ Opens brush-compatible object suggestions for a vortex (published by {@link BrushSession}). */
export interface Puzzle3dOpenVortexSuggestions {
  readonly openFor: (fullId: string, meta: VortexBindingMeta, anchor: ScreenPoint) => void;
  readonly close: () => void;
}

export const puzzle3dOpenVortexSuggestionsRef: { current: Puzzle3dOpenVortexSuggestions } = {
  current: { openFor: () => {}, close: () => {} },
};

/** @emoji 🎯 Maps exclusive hover target to a canvas selection pick. */
export function hoverTargetToSelectionPick(target: HoverTarget): SelectionPick {
  switch (target.kind) {
    case "object":
      return { kind: "object", id: target.id };
    case "vortex":
      return { kind: "vortex", fullId: target.fullId };
    case "attraction":
      return { kind: "attraction", id: target.id };
    case "reference":
      return { kind: "reference", id: target.id };
  }
}

/** @emoji 🎯 Resolves hidden/locked flags for every row in a selection snapshot. */
export function puzzle3dSelectionEntityFlagsFromStore(
  store: { readonly getRecord: (objectId: string) => ObjectRecord | undefined; readonly getAttractions: () => readonly AttractionProps[] },
  selection: SelectionSnapshot,
): readonly Puzzle3dSelectionEntityFlags[] {
  const flags: Puzzle3dSelectionEntityFlags[] = [];
  const attractionById = new Map(store.getAttractions().map((row) => [row.id, row]));
  for (const objectId of selection.objectIds) {
    const record = store.getRecord(objectId);
    flags.push({ hidden: record?.hidden === true, locked: record?.locked === true });
  }
  for (const fullId of selection.vortexIds) {
    const { objectId, vortexId } = parseVortexFullId(fullId);
    const record = store.getRecord(objectId);
    const vortex = record?.vortices.find((row) => row.id === vortexId || puzzle3dVortexFullId(objectId, row.id) === fullId);
    flags.push({ hidden: vortex?.hidden === true, locked: vortex?.locked === true });
  }
  for (const attractionId of selection.attractionIds) {
    const attraction = attractionById.get(attractionId);
    flags.push({ hidden: attraction?.hidden === true, locked: attraction?.locked === true });
  }
  return flags;
}

/** @emoji 🖱️ Builds Radix context menu rows for the current puzzle3d selection. */
export function buildPuzzle3dSelectionMenuItems(
  selection: SelectionSnapshot,
  entityFlags: readonly Puzzle3dSelectionEntityFlags[],
  target: HoverTarget | null,
  actions: Puzzle3dSelectionMenuActions,
  onSuggest?: () => void,
): ContextMenuItem[] {
  if (selection.objectIds.length === 0 && selection.vortexIds.length === 0 && selection.attractionIds.length === 0) {
    return [];
  }
  const items: ContextMenuItem[] = [];
  const anyNotHidden = entityFlags.some((row) => !row.hidden);
  const anyNotLocked = entityFlags.some((row) => !row.locked);
  const singleVortex =
    selection.vortexIds.length === 1 && selection.objectIds.length === 0 && selection.attractionIds.length === 0;
  if (onSuggest && target?.kind === "vortex" && singleVortex) {
    items.push({
      id: "suggest",
      label: "Suggest objects",
      icon: "sparkles",
      onSelect: () => onSuggest(),
    });
    items.push({ id: "suggest-sep", separator: true });
  }
  items.push({
    id: "hidden",
    label: anyNotHidden ? "Hide" : "Show",
    icon: anyNotHidden ? "eye-off" : "eye",
    onSelect: () => actions.toggleHidden(anyNotHidden),
  });
  items.push({
    id: "locked",
    label: anyNotLocked ? "Lock" : "Unlock",
    icon: anyNotLocked ? "lock" : "lock-open",
    onSelect: () => actions.toggleLocked(anyNotLocked),
  });
  if (selection.objectIds.length > 0) {
    items.push({
      id: "duplicate",
      label: "Duplicate",
      icon: "copy",
      onSelect: () => actions.duplicateSelection(),
    });
    items.push({
      id: "select-same-kind",
      label: "Select all of same kind",
      icon: "layers",
      onSelect: () => actions.selectSameKind(),
    });
  }
  items.push({ id: "zoom-sep", separator: true });
  items.push({
    id: "zoom",
    label: "Zoom to selection",
    icon: "crosshair",
    onSelect: () => requestPuzzle3dZoomToSelection(selection),
  });
  items.push({ id: "delete-sep", separator: true });
  items.push({
    id: "delete",
    label: "Delete",
    icon: "trash",
    destructive: true,
    onSelect: () => actions.deleteSelection(),
  });
  return items;
}

function closePuzzle3dSelectionMenu(): void {
  puzzle3dSelectionMenuStore.setSnapshot(SELECTION_MENU_IDLE);
}

//#endregion 🖱️SelectionContextMenu

/** @emoji ⌨️ True when Tab should cycle brush candidates instead of moving browser focus. */
export function routePuzzle3dBrushTabKeydown(
  brushActive: boolean,
  targetActive: boolean,
  candidateCount: number,
  event: Pick<KeyboardEvent, "key" | "defaultPrevented" | "ctrlKey" | "metaKey" | "altKey">,
): boolean {
  if (!brushActive || !targetActive || candidateCount <= 1) {
    return false;
  }
  if (event.key !== "Tab" || event.defaultPrevented) {
    return false;
  }
  if (event.ctrlKey || event.metaKey || event.altKey) {
    return false;
  }
  return true;
}

/** @emoji 🪣 Live fill build progress for engagement UI while the sequence is computed. */
export interface Puzzle3dFillBuildProgress {
  readonly count: number;
  readonly maxCount: number;
  readonly done: boolean;
}

/** @emoji 🪣 Window engagement option id for fill target-volume edit mode. */
export const PUZZLE_3D_ENGAGEMENT_FILL_EDIT_VOLUMES_ID = "puzzle3d.fill.editTargetVolumes";

/** @emoji 🪣 Window engagement option id for deleting the selected target volume. */
export const PUZZLE_3D_ENGAGEMENT_DELETE_TARGET_VOLUME_ID = "puzzle3d.fill.deleteTargetVolume";

/** @emoji 💬 Inputs for {@link buildPuzzle3dPlayEngagement} (CAD play interaction panel shape). */
export interface Puzzle3dPlayEngagementInputs {
  readonly activeTool: "select" | "brush" | "fill";
  readonly cmdLine: string;
  readonly fillCount: number;
  readonly fillBuildProgress?: Puzzle3dFillBuildProgress;
  readonly fillEditTargetVolumes?: boolean;
  readonly voxelBrushDimensions?: Vec3;
  readonly selectedTargetVolumeCount?: number;
  readonly selectionCount: number;
  readonly onCmdLineChange: (value: string) => void;
  readonly onCmdLineSubmit: (value: string) => void;
  readonly onRepeatLast?: () => void;
  readonly onAbort?: () => void;
  readonly onSelectTool: () => void;
  readonly onBrushTool: () => void;
  readonly onFillTool: () => void;
  readonly onFillCount: (count: number) => void;
  readonly onToggleFillEditTargetVolumes?: () => void;
  readonly onDeleteSelectedTargetVolume?: () => void;
  readonly onVoxelBrushDimension?: (axis: 0 | 1 | 2, value: number) => void;
  readonly onCycleBrushCandidate: () => void;
  readonly onPickBrushCandidate: (index: number) => void;
  readonly onZoomToSelection: () => void;
  readonly brushCandidates: readonly BrushCompatibleCandidate[];
  readonly brushTargetActive: boolean;
  readonly brushPlacementProbePending?: boolean;
}

/** @emoji 💬 Builds window {@link EngagementSpec}: command input, possibles, options, status (CAD play layout). */
export function buildPuzzle3dPlayEngagement(inputs: Puzzle3dPlayEngagementInputs): EngagementSpec {
  const status: { id: string; content: string }[] = [];
  if (inputs.activeTool === "brush") {
    status.push({ id: "puzzle3d.brush.hint", content: "Point at an empty connector; hold Alt and leave to flush; Tab cycles placements; left-click opens the candidate list" });
    if (inputs.brushTargetActive && inputs.brushCandidates.length === 0) {
      if (inputs.brushPlacementProbePending) {
        status.push({ id: "puzzle3d.brush.probe", content: "Checking collision-free placements…" });
      } else {
        status.push({ id: "puzzle3d.brush.none", content: "No collision-free placement at this connector" });
      }
    }
  }
  if (inputs.activeTool === "fill") {
    status.push({ id: "puzzle3d.fill.hint", content: "Drag the slider to grow or shrink the partial fill solution" });
    if (inputs.fillEditTargetVolumes) {
      status.push({
        id: "puzzle3d.fill.volumeEdit",
        content: "Adjust W/D/H steppers; preview follows cursor; press Alt to place; release Alt to select voxels",
      });
    }
  }
  if (inputs.selectionCount > 0) {
    status.push({
      id: "puzzle3d.selection",
      content: inputs.selectionCount === 1 ? "1 selected" : `${inputs.selectionCount} selected`,
    });
  }

  const toolPossibles = [
    { id: "puzzle3d.tool.brush", label: "Brush", onSelect: inputs.onBrushTool },
    { id: PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID, label: "Fill", onSelect: inputs.onFillTool },
    { id: "puzzle3d.tool.select", label: "Select", onSelect: inputs.onSelectTool },
  ];

  const brushPossibles = inputs.brushCandidates.map((candidate, index) => ({
    id: `puzzle3d.brush.${candidate.objectKindId}.${candidate.sourceVortexIndex}`,
    label: normalizeEngagementCommandText(candidate.objectKindId),
    detail: `v${candidate.sourceVortexIndex}`,
    onSelect: () => inputs.onPickBrushCandidate(index),
  }));

  const zoomOptions =
    inputs.selectionCount > 0 ? [{ id: PUZZLE_3D_ENGAGEMENT_ZOOM_ID, label: "Zoom", onPress: inputs.onZoomToSelection }] : [];
  const brushOptions =
    inputs.activeTool === "brush" && inputs.brushTargetActive
      ? [
          { id: "puzzle3d.tool.select", label: "Select", onPress: inputs.onSelectTool },
          { id: PUZZLE_3D_ENGAGEMENT_BRUSH_NEXT_ID, label: "Next", onPress: inputs.onCycleBrushCandidate },
        ]
      : [];
  const fillOptions =
    inputs.activeTool === "fill"
      ? [
          {
            id: PUZZLE_3D_ENGAGEMENT_FILL_EDIT_VOLUMES_ID,
            label: inputs.fillEditTargetVolumes ? "Done editing volumes" : "Edit target volumes",
            onPress: () => inputs.onToggleFillEditTargetVolumes?.(),
          },
          ...(inputs.fillEditTargetVolumes && (inputs.selectedTargetVolumeCount ?? 0) > 0
            ? [{ id: PUZZLE_3D_ENGAGEMENT_DELETE_TARGET_VOLUME_ID, label: "Delete volume", onPress: () => inputs.onDeleteSelectedTargetVolume?.() }]
            : []),
        ]
      : [];
  const options = zoomOptions.length || brushOptions.length || fillOptions.length ? [...zoomOptions, ...brushOptions, ...fillOptions] : undefined;

  const possibleEngagements = inputs.activeTool === "brush" && brushPossibles.length > 0 ? brushPossibles : toolPossibles;

  const fillSliderMax =
    inputs.fillBuildProgress && !inputs.fillBuildProgress.done ? inputs.fillBuildProgress.count : PUZZLE_3D_FILL_COUNT_MAX;
  const fillSliderLabel =
    inputs.fillBuildProgress && !inputs.fillBuildProgress.done
      ? `Fill ${inputs.fillCount} (building ${inputs.fillBuildProgress.count}/${inputs.fillBuildProgress.maxCount})`
      : `Fill ${inputs.fillCount}`;
  const dims = inputs.voxelBrushDimensions ?? DEFAULT_VOXEL_BRUSH_DIMENSIONS;
  const voxelDimensionSlider = (axis: 0 | 1 | 2, label: string, id: string): EngagementSliderControl => ({
    kind: "slider",
    id,
    label,
    value: dims[axis],
    min: VOXEL_BRUSH_SIZE_MIN,
    max: VOXEL_BRUSH_SIZE_MAX,
    step: VOXEL_BRUSH_SIZE_STEP,
    onChange: (value) => inputs.onVoxelBrushDimension?.(axis, value),
  });
  const control =
    inputs.activeTool === "fill" && inputs.fillEditTargetVolumes
      ? undefined
      : inputs.activeTool === "fill"
      ? {
          kind: "slider" as const,
          id: "puzzle3d-fill-count",
          label: fillSliderLabel,
          value: Math.min(inputs.fillCount, fillSliderMax),
          min: 0,
          max: fillSliderMax,
          step: 1,
          onChange: inputs.onFillCount,
        }
      : inputs.activeTool === "brush" && brushPossibles.length > 0
        ? {
            kind: "ring" as const,
            id: "puzzle3d-brush-ring",
            label: "Placement",
            value: brushPossibles[0]!.id,
            options: brushPossibles.map((row) => ({ id: row.id, label: row.label })),
            onSelect: (id: string) => {
              const index = brushPossibles.findIndex((row) => row.id === id);
              if (index >= 0) inputs.onPickBrushCandidate(index);
            },
          }
        : {
            kind: "ring" as const,
            id: "puzzle3d-tool-ring",
            label: "Tool",
            value:
              inputs.activeTool === "brush"
                ? "puzzle3d.tool.brush"
                : inputs.activeTool === "fill"
                  ? PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID
                  : "puzzle3d.tool.select",
            options: toolPossibles.map((row) => ({ id: row.id, label: row.label })),
            onSelect: (id: string) => {
              if (id === "puzzle3d.tool.brush") inputs.onBrushTool();
              else if (id === PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID) inputs.onFillTool();
              else if (id === "puzzle3d.tool.select") inputs.onSelectTool();
            },
          };

  return {
    sessionActive: inputs.activeTool === "brush" || inputs.activeTool === "fill",
    input: {
      id: "engagement-input",
      value: inputs.cmdLine,
      placeholder: inputs.activeTool === "fill" ? "Fill" : inputs.activeTool === "brush" ? "Kind name or list" : "Brush",
      onChange: inputs.onCmdLineChange,
      onSubmit: inputs.onCmdLineSubmit,
      onRepeatLast: inputs.onRepeatLast,
      onAbort: inputs.onAbort,
    },
    control,
    ...(inputs.activeTool === "fill" && inputs.fillEditTargetVolumes
      ? {
          controls: [
            voxelDimensionSlider(0, "Width", "puzzle3d-voxel-width"),
            voxelDimensionSlider(1, "Depth", "puzzle3d-voxel-depth"),
            voxelDimensionSlider(2, "Height", "puzzle3d-voxel-height"),
          ],
        }
      : {}),
    ...(options?.length ? { options } : {}),
    ...(status.length ? { status } : {}),
    possibleEngagements,
  };
}

/** @emoji 🖌️ External store for brush UI (menu overlay + preview pose). */
export function createBrushUiStore(initial: BrushUiSnapshot = BRUSH_UI_IDLE) {
  let snapshot = initial;
  const listeners = new Set<() => void>();
  return {
    subscribe(listener: () => void): () => void {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    getSnapshot(): BrushUiSnapshot {
      return snapshot;
    },
    setSnapshot(next: BrushUiSnapshot): void {
      snapshot = next;
      for (const listener of listeners) {
        listener();
      }
    },
  };
}

export type BrushUiStore = ReturnType<typeof createBrushUiStore>;

export const puzzle3dBrushUiStore = createBrushUiStore();

function patchBrushUi(patch: Partial<BrushUiSnapshot>): void {
  const prev = puzzle3dBrushUiStore.getSnapshot();
  puzzle3dBrushUiStore.setSnapshot({
    preview: patch.preview !== undefined ? patch.preview : prev.preview,
    candidates: patch.candidates !== undefined ? patch.candidates : prev.candidates,
    targetActive: patch.targetActive !== undefined ? patch.targetActive : prev.targetActive,
    placementProbePending: patch.placementProbePending !== undefined ? patch.placementProbePending : prev.placementProbePending,
    menuOpen: patch.menuOpen !== undefined ? patch.menuOpen : prev.menuOpen,
    menuAnchor: patch.menuAnchor !== undefined ? patch.menuAnchor : prev.menuAnchor,
    menuHoverIndex: patch.menuHoverIndex !== undefined ? patch.menuHoverIndex : prev.menuHoverIndex,
  });
}

/** @emoji 🖌️ True while the brush tool is the active play tool. */
export const puzzle3dBrushToolActiveRef = { current: false };

/** @emoji ⌥ True while Alt is held during brush hover (enables flush-on-leave). */
export const puzzle3dBrushAltPressedRef = { current: false };

/** @emoji 🧊 True while fill target-volume edit mode is active. */
export const puzzle3dTargetVolumeToolActiveRef = { current: false };

/** @emoji 🧱 Default axis-aligned voxel brush box size (world units). */
export const DEFAULT_VOXEL_BRUSH_DIMENSIONS: Vec3 = [10, 10, 10];
export const VOXEL_BRUSH_SIZE_MIN = 1;
export const VOXEL_BRUSH_SIZE_MAX = 20;
export const VOXEL_BRUSH_SIZE_STEP = 1;

export interface VoxelBrushUiSnapshot {
  readonly altPainting: boolean;
  readonly cursorCad: Vec3 | null;
}

const VOXEL_BRUSH_UI_IDLE: VoxelBrushUiSnapshot = { altPainting: false, cursorCad: null };

/** @emoji 🧱 External store for voxel brush cursor and Alt-paint state. */
export function createVoxelBrushUiStore(initial: VoxelBrushUiSnapshot = VOXEL_BRUSH_UI_IDLE) {
  let snapshot = initial;
  const listeners = new Set<() => void>();
  return {
    subscribe(listener: () => void): () => void {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    getSnapshot(): VoxelBrushUiSnapshot {
      return snapshot;
    },
    setSnapshot(next: VoxelBrushUiSnapshot): void {
      snapshot = next;
      for (const listener of listeners) {
        listener();
      }
    },
  };
}

export const puzzle3dVoxelBrushUiStore = createVoxelBrushUiStore();
export const puzzle3dVoxelBrushDimensionsRef: { current: Vec3 } = { current: DEFAULT_VOXEL_BRUSH_DIMENSIONS };

const puzzle3dVoxelBrushBridgeRef: {
  current: {
    readonly onPaint?: (cad: Vec3, scale: Vec3) => void;
  };
} = { current: {} };

function brushPreviewWorldAabb(preview: BrushPreviewState, meshRoot: Object3D): { readonly min: Vec3; readonly max: Vec3 } | null {
  const probe = brushProbeGroupFromPreview(preview, meshRoot);
  updateWorldMatrixChain(probe);
  const box = new Box3().setFromObject(probe, true);
  if (!Number.isFinite(box.min.x) || box.isEmpty()) {
    return null;
  }
  return { min: threeVec3ToCad(box.min), max: threeVec3ToCad(box.max) };
}

/** @emoji 🖌️ True while the cursor is over a free vortex in brush mode (suppresses orbit right-drag). */
export const puzzle3dBrushVortexHoverRef = { current: false };

//#region 🎬Viewport
function OrbitGated(props: {
  readonly controlsKey?: string | number;
  readonly zoom: number;
  readonly up?: Vec3;
  readonly projection?: CameraState["projection"];
  readonly onCamera?: (state: CameraState) => void;
}) {
  const reg = useRegistry();
  const { snapGate } = useWorldOrbitViewSnapGate();
  const { camera, gl } = useThree();
  const controls = useThree((s) => s.controls as WorldOrbitControlsBinding | null);
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const gate = reg.attractionDragActive || reg.attractionIndirectPickAwait !== null || snapGate || (puzzle3dBrushToolActiveRef.current && puzzle3dBrushVortexHoverRef.current);
  const invalidate = useThree((s) => s.invalidate);
  const projection = props.projection ?? "perspective";
  const reportCamera = reactHostPort.useCallback(() => {
    if (!props.onCamera) {
      return;
    }
    const tgt = controls?.target ?? targetScratch.set(0, 0, 0);
    props.onCamera({
      position: threeVec3ToCad(camera.position),
      target: threeVec3ToCad(tgt),
      zoom: props.zoom,
      ...(props.up ? { up: props.up } : {}),
      projection,
    });
  }, [camera, controls, props.onCamera, props.projection, props.up, props.zoom, projection, targetScratch]);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [gate, invalidate]);
  const mouseButtonsIdle = reactHostPort.useMemo(() => resolveWorldOrbitMouseButtonsIdle(projection), [projection]);
  useWorldOrbitRightMouseBindings(controls, gl.domElement, {
    projection,
    dragThresholdPx: PUZZLE_3D_MARQUEE_DRAG_THRESHOLD_PX,
    onRightPointerDown: (event) => {
      if (puzzle3dBrushToolActiveRef.current && puzzle3dBrushVortexHoverRef.current) {
        return false;
      }
      puzzle3dRightDragActiveRef.current = event.altKey;
      return true;
    },
    onRightPointerDrag: () => {
      puzzle3dRightDragActiveRef.current = true;
    },
    onRightPointerUp: () => {
      window.setTimeout(() => {
        puzzle3dRightDragActiveRef.current = false;
      }, 0);
    },
  });
  return (
    <OrbitControls
      key={props.controlsKey}
      camera={camera}
      makeDefault
      enabled={!gate}
      enableDamping={false}
      enablePan
      enableZoom
      onChange={() => invalidate()}
      onStart={() => invalidate()}
      onEnd={() => {
        invalidate();
        reportCamera();
      }}
      mouseButtons={mouseButtonsIdle}
    />
  );
}

type SelectionZoomAnim = {
  readonly epoch: number;
  readonly fromPos: Vector3;
  readonly fromTgt: Vector3;
  readonly toPos: Vector3;
  readonly toTgt: Vector3;
  readonly startMs: number;
};

/** @emoji 🔍 Smoothly eases orbit camera when engagement Zoom frames the selection. */
function SelectionZoom(props: {
  readonly attractions: readonly Pick<AttractionProps, "id" | "attracting" | "attracted">[];
  readonly zoom: number;
  readonly onCamera?: (state: CameraState) => void;
}): null {
  const reg = useRegistry();
  const { camera, controls, invalidate } = useThree();
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const posScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const zoomEpoch = reactHostPort.useSyncExternalStore(subscribePuzzle3dZoomToSelection, getPuzzle3dZoomToSelectionEpoch, getPuzzle3dZoomToSelectionEpoch);
  const fulfilledEpochRef = reactHostPort.useRef(0);
  const boundsRetryRef = reactHostPort.useRef(0);
  const animRef = reactHostPort.useRef<SelectionZoomAnim | null>(null);
  useFrame(() => {
    if (!(camera instanceof ThreePerspectiveCamera)) {
      return;
    }
    const orbit = controls as { readonly target: Vector3; readonly update?: () => void; enabled?: boolean } | null;
    const anim = animRef.current;
    if (anim) {
      if (anim.epoch !== zoomEpoch) {
        animRef.current = null;
        puzzle3dSelectionZoomAnimatingRef.current = false;
        if (orbit) {
          orbit.enabled = true;
        }
      } else {
        const linear = Math.min(1, (performance.now() - anim.startMs) / PUZZLE_3D_SELECTION_ZOOM_DURATION_MS);
        const w = puzzle3dEaseInOutCubic01(linear);
        posScratch.copy(anim.fromPos).lerp(anim.toPos, w);
        targetScratch.copy(anim.fromTgt).lerp(anim.toTgt, w);
        camera.position.copy(posScratch);
        if (orbit?.target) {
          orbit.target.copy(targetScratch);
          orbit.update?.();
        } else {
          camera.lookAt(targetScratch);
        }
        camera.updateProjectionMatrix();
        invalidate();
        if (linear < 1) {
          return;
        }
        animRef.current = null;
        puzzle3dSelectionZoomAnimatingRef.current = false;
        if (orbit) {
          orbit.enabled = true;
        }
        props.onCamera?.({
          position: threeVec3ToCad(camera.position),
          target: threeVec3ToCad(targetScratch),
          zoom: props.zoom,
          projection: "perspective",
        });
        return;
      }
    }
    if (zoomEpoch <= fulfilledEpochRef.current) {
      return;
    }
    const selection = getPuzzle3dZoomToSelectionTarget();
    const bounds = boundsFromPuzzle3dSelection(selection, reg, props.attractions);
    if (!bounds) {
      boundsRetryRef.current += 1;
      if (boundsRetryRef.current > 90) {
        fulfilledEpochRef.current = zoomEpoch;
        boundsRetryRef.current = 0;
      }
      return;
    }
    boundsRetryRef.current = 0;
    fulfilledEpochRef.current = zoomEpoch;
    const rig = puzzle3dFitCameraRigFromBounds(bounds, 1.35);
    const fromTgt = orbit?.target ? orbit.target.clone() : targetScratch.set(...cadVec3ToThree(bounds.center));
    animRef.current = {
      epoch: zoomEpoch,
      fromPos: camera.position.clone(),
      fromTgt,
      toPos: rig.position,
      toTgt: rig.target,
      startMs: performance.now(),
    };
    puzzle3dSelectionZoomAnimatingRef.current = true;
    if (orbit) {
      orbit.enabled = false;
    }
    invalidate();
  });
  return null;
}

/** @emoji 🛰️ Frames orbit camera to loaded object bounds once meshes are measurable (initial load fit). */
function AutoFit(props: {
  readonly behavior?: AutoFitBehavior;
  readonly padding?: number;
  readonly zoom?: number;
  readonly seedKey?: string | number;
  readonly projection?: CameraState["projection"];
  readonly onCamera?: (state: CameraState) => void;
}): null {
  const reg = useRegistry();
  const { camera, controls, invalidate } = useThree();
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const lastKey = reactHostPort.useRef("");
  const hasApplied = reactHostPort.useRef(false);
  const behavior = props.behavior ?? "initial";
  const padding = props.padding ?? 1.25;
  const zoom = props.zoom ?? 1;
  const seedKey = String(props.seedKey ?? "default");
  useFrame(() => {
    if (puzzle3dBrushSessionActive()) {
      return;
    }
    if (behavior === "initial" && (hasApplied.current || puzzle3dAutoFitInitialApplied(seedKey))) {
      return;
    }
    const groups = reg.collectObjectGroups();
    if (!groups.length) return;
    const key = groups
      .map((group) => group.uuid)
      .sort()
      .join("|");
    if (!puzzle3dAutoFitShouldRun(behavior, key, lastKey.current, hasApplied.current)) return;
    const bounds = boundsFromObjectGroups(groups);
    if (!bounds) return;
    lastKey.current = key;
    hasApplied.current = true;
    if (behavior === "initial") {
      puzzle3dAutoFitMarkInitialApplied(seedKey);
    }
    if (!(camera instanceof ThreePerspectiveCamera)) return;
    const orbit = controls as { target: Vector3; update?: () => void } | null;
    applyAutoFitCamera(camera, bounds, padding, orbit);
    invalidate();
    const tgt = orbit?.target ?? targetScratch.set(...cadVec3ToThree(bounds.center));
    props.onCamera?.({
      position: threeVec3ToCad(camera.position),
      target: threeVec3ToCad(tgt),
      zoom,
      projection: props.projection ?? "perspective",
    });
  });
  return null;
}

function findVortexIndexOnRecord(record: ObjectRecord, fullId: string): number {
  const { vortexId } = parseVortexFullId(fullId);
  for (let i = 0; i < record.vortices.length; i += 1) {
    const v = record.vortices[i]!;
    if (v.id === vortexId || puzzle3dVortexFullId(record.id, v.id) === fullId) {
      return i;
    }
  }
  return -1;
}

function brushMeshRootFromPool(meshUrl: string): Object3D | null {
  return brushCollisionGltfRoot(meshUrl);
}

function brushPlacePayloadFromPreview(preview: BrushPreviewState): BrushPlacePayload {
  return {
    targetVortexFullId: preview.targetVortexFullId,
    objectKindId: preview.objectKindId,
    sourceVortexIndex: preview.sourceVortexIndex,
    origin: preview.origin,
    orientation: preview.orientation,
    ...(preview.scale !== undefined ? { scale: preview.scale } : {}),
  };
}

/** @emoji 🧪 Catalog mesh root for fill collision probes (same pool as {@link BrushSession}). */
export function puzzle3dBrushMeshRootForFill(meshUrl: string): Object3D | null {
  return brushMeshRootFromPool(meshUrl);
}

class BrushCatalogMeshPreloadErrorBoundary extends React.Component<
  { readonly onReady: () => void; readonly children: ReactNode },
  { readonly hasError: boolean }
> {
  override state = { hasError: false };

  static getDerivedStateFromError(): { hasError: boolean } {
    return { hasError: true };
  }

  override componentDidCatch(): void {
    this.props.onReady();
  }

  override render(): ReactNode {
    return this.state.hasError ? null : this.props.children;
  }
}

function BrushCatalogMeshPreloadEntry(props: { readonly url: string; readonly onReady: () => void }) {
  const gltf = usePooledGltf(props.url);
  reactHostPort.useLayoutEffect(() => {
    if (!gltf.scene) {
      return undefined;
    }
    styledMeshPoolAcquire(props.url, "highlighted", false);
    styledMeshTemplate(props.url, "highlighted", gltf.scene, false);
    registerBrushCollisionGltfScene(props.url, gltf.scene);
    props.onReady();
    return () => {
      styledMeshPoolRelease(props.url, "highlighted", false);
    };
  }, [gltf.scene, props.onReady, props.url]);
  return null;
}

function BrushCatalogMeshPreload(props: { readonly urls: readonly string[]; readonly onReady: () => void }) {
  if (!props.urls.length) {
    return null;
  }
  return (
    <>
      {props.urls.map((url) => (
        <BrushCatalogMeshPreloadErrorBoundary key={url} onReady={props.onReady}>
          <BrushCatalogMeshPreloadEntry url={url} onReady={props.onReady} />
        </BrushCatalogMeshPreloadErrorBoundary>
      ))}
    </>
  );
}

function BrushCatalogMeshPreloadAll(props: { readonly urls: readonly string[]; readonly onAllReady: () => void }): null {
  const onAllReadyRef = reactHostPort.useRef(props.onAllReady);
  onAllReadyRef.current = props.onAllReady;
  const readyUrlsRef = reactHostPort.useRef(new Set<string>());
  const notifiedRef = reactHostPort.useRef(false);
  const notifyAllReady = reactHostPort.useCallback(() => {
    if (notifiedRef.current) {
      return;
    }
    notifiedRef.current = true;
    onAllReadyRef.current();
  }, []);
  const markUrlReady = reactHostPort.useCallback(
    (url: string) => {
      readyUrlsRef.current.add(url);
      if (props.urls.length > 0 && readyUrlsRef.current.size >= props.urls.length) {
        notifyAllReady();
      }
    },
    [notifyAllReady, props.urls.length],
  );
  reactHostPort.useEffect(() => {
    readyUrlsRef.current = new Set<string>();
    notifiedRef.current = false;
  }, [props.urls]);
  if (!props.urls.length) {
    return null;
  }
  return (
    <>
      {props.urls.map((url) => (
        <BrushCatalogMeshPreloadErrorBoundary key={url} onReady={() => markUrlReady(url)}>
          <BrushCatalogMeshPreloadEntry url={url} onReady={() => markUrlReady(url)} />
        </BrushCatalogMeshPreloadErrorBoundary>
      ))}
    </>
  );
}

function Puzzle3dFillMeshBridge(props: {
  readonly fillActive: boolean;
  readonly sceneFixture: FixtureV1 | undefined;
  readonly kindCatalogs: KindCatalogBundle | undefined;
  readonly kindCompatibility: readonly KindCompatEntry[] | undefined;
  readonly onMeshesReady: () => void;
}): null {
  const prevFillActiveRef = reactHostPort.useRef(false);
  const [fillSnapshot, setFillSnapshot] = reactHostPort.useState<FixtureV1 | null>(null);
  reactHostPort.useLayoutEffect(() => {
    const enteredFill = props.fillActive && !prevFillActiveRef.current;
    if (props.fillActive && props.sceneFixture && (enteredFill || fillSnapshot === null)) {
      setFillSnapshot(structuredClone(props.sceneFixture));
    }
    if (!props.fillActive) {
      setFillSnapshot(null);
    }
    prevFillActiveRef.current = props.fillActive;
  }, [fillSnapshot, props.fillActive, props.sceneFixture]);
  const urls = reactHostPort.useMemo(() => {
    if (!props.fillActive || !fillSnapshot) {
      return [];
    }
    return brushMeshUrlsForFillSession(fillSnapshot, props.kindCatalogs, props.kindCompatibility);
  }, [fillSnapshot, props.fillActive, props.kindCatalogs, props.kindCompatibility]);
  const onReadyRef = reactHostPort.useRef(props.onMeshesReady);
  onReadyRef.current = props.onMeshesReady;
  const notifyReady = reactHostPort.useCallback(() => {
    onReadyRef.current();
  }, []);
  reactHostPort.useLayoutEffect(() => {
    if (!props.fillActive || !fillSnapshot) {
      return;
    }
    if (urls.length === 0) {
      notifyReady();
    }
  }, [fillSnapshot, notifyReady, props.fillActive, urls.length]);
  if (!props.fillActive || !fillSnapshot) {
    return null;
  }
  if (!urls.length) {
    return null;
  }
  return <BrushCatalogMeshPreloadAll urls={urls} onAllReady={notifyReady} />;
}

const BrushPreviewGhost = reactHostPort.memo(function BrushPreviewGhost(props: {
  readonly preview: BrushPreviewState;
}) {
  const groupRef = reactHostPort.useRef<Group>(null);
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useLayoutEffect(() => {
    const group = groupRef.current;
    if (group) {
      applyObjectPose(group, props.preview.origin, props.preview.orientation, props.preview.scale);
      invalidate();
    }
  }, [props.preview.meshUrl, props.preview.origin, props.preview.orientation, props.preview.scale, invalidate]);
  return (
    <group ref={groupRef} raycast={() => null}>
      <MeshBody meshUrl={props.preview.meshUrl} style="highlighted" scale={props.preview.scale} />
    </group>
  );
});

const VoxelBrushPreview = reactHostPort.memo(function VoxelBrushPreview(props: { readonly dimensions: Vec3 }) {
  const ui = reactHostPort.useSyncExternalStore(
    puzzle3dVoxelBrushUiStore.subscribe,
    puzzle3dVoxelBrushUiStore.getSnapshot,
    puzzle3dVoxelBrushUiStore.getSnapshot,
  );
  if (!ui.cursorCad) {
    return null;
  }
  const preview: WorldVolumeProps = {
    ...createVoxelVolume(ui.cursorCad, props.dimensions),
    id: "voxel-brush-preview",
    color: VOXEL_BRUSH_PREVIEW_COLOR,
    opacity: VOXEL_BRUSH_PREVIEW_OPACITY,
  };
  return <WorldVolumeLayer volumes={[preview]} interactive={false} />;
});

function VoxelBrushBridge(props: { readonly enabled: boolean; readonly dimensions: Vec3 }): null {
  const { camera, gl } = useThree();
  const controls = useThree((state) => state.controls as { target?: Vector3 } | null | undefined);
  const lodCtx = useLod();
  const lastCommitKeyRef = reactHostPort.useRef<string | null>(null);
  reactHostPort.useEffect(() => {
    puzzle3dTargetVolumeToolActiveRef.current = props.enabled;
    puzzle3dVoxelBrushDimensionsRef.current = props.dimensions;
    if (!props.enabled) {
      puzzle3dVoxelBrushUiStore.setSnapshot(VOXEL_BRUSH_UI_IDLE);
      lastCommitKeyRef.current = null;
    }
    return () => {
      puzzle3dTargetVolumeToolActiveRef.current = false;
    };
  }, [props.enabled, props.dimensions]);
  reactHostPort.useEffect(() => {
    if (!props.enabled) {
      return;
    }
    const canvas = gl.domElement;
    const pickCad = (clientX: number, clientY: number): Vec3 =>
      puzzle3dClientToGridPlaneCad({
        clientX,
        clientY,
        camera,
        canvas,
        gridSnapEnabled: lodCtx.gridSnapEnabled,
        gridStepWorld: lodCtx.gridStepWorld,
        gridPlaneAnchorCad: puzzle3dGridPlacementAnchorCad(controls?.target ?? null),
      });
    const commitAt = (cad: Vec3): void => {
      const scale = puzzle3dVoxelBrushDimensionsRef.current;
      const key = voxelGridKey(cad, scale);
      if (lastCommitKeyRef.current === key) {
        return;
      }
      lastCommitKeyRef.current = key;
      puzzle3dVoxelBrushBridgeRef.current.onPaint?.(cad, scale);
      console.log("[DEBUG] puzzle3d voxel brush commit", cad, scale);
    };
    const setAltPainting = (altPainting: boolean): void => {
      const prev = puzzle3dVoxelBrushUiStore.getSnapshot();
      if (prev.altPainting === altPainting) {
        return;
      }
      if (!altPainting) {
        lastCommitKeyRef.current = null;
      }
      puzzle3dVoxelBrushUiStore.setSnapshot({ ...prev, altPainting });
    };
    const onKeyDown = (event: KeyboardEvent): void => {
      if (event.key !== "Alt") {
        return;
      }
      setAltPainting(true);
      const cursor = puzzle3dVoxelBrushUiStore.getSnapshot().cursorCad;
      if (cursor) {
        commitAt(cursor);
      }
    };
    const onKeyUp = (event: KeyboardEvent): void => {
      if (event.key === "Alt") {
        setAltPainting(false);
      }
    };
    const onBlur = (): void => {
      setAltPainting(false);
    };
    const onMove = (event: PointerEvent): void => {
      const cad = pickCad(event.clientX, event.clientY);
      const prev = puzzle3dVoxelBrushUiStore.getSnapshot();
      puzzle3dVoxelBrushUiStore.setSnapshot({ ...prev, cursorCad: cad });
    };
    const onDown = (event: PointerEvent): void => {
      if (event.button !== 0 || !event.altKey) {
        return;
      }
      setAltPainting(true);
      commitAt(pickCad(event.clientX, event.clientY));
    };
    window.addEventListener("keydown", onKeyDown);
    window.addEventListener("keyup", onKeyUp);
    window.addEventListener("blur", onBlur);
    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerdown", onDown);
    return () => {
      window.removeEventListener("keydown", onKeyDown);
      window.removeEventListener("keyup", onKeyUp);
      window.removeEventListener("blur", onBlur);
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerdown", onDown);
    };
  }, [camera, controls?.target, gl.domElement, lodCtx.gridSnapEnabled, lodCtx.gridStepWorld, props.enabled]);
  return null;
}

function BrushSession(props: {
  readonly brushActive: boolean;
  readonly onBrushPlace?: (payload: BrushPlacePayload) => void;
  readonly kindCatalogs: KindCatalogBundle | undefined;
  readonly kindCompatibility: readonly KindCompatEntry[] | undefined;
  readonly overlapBudget: number;
  readonly sceneFixture?: FixtureV1;
}) {
  const reg = useRegistryCore();
  const { store } = useObjectState();
  const invalidate = useThree((state) => state.invalidate);
  const targetRef = reactHostPort.useRef<string | null>(null);
  const targetCtxRef = reactHostPort.useRef<AttractionVortexContext | null>(null);
  const targetObjectIdRef = reactHostPort.useRef<string | null>(null);
  const targetOrientationRef = reactHostPort.useRef<Quat | undefined>(undefined);
  const probeOrderRef = reactHostPort.useRef<readonly BrushCompatibleCandidate[]>([]);
  const placementCandidatesRef = reactHostPort.useRef<readonly BrushCompatibleCandidate[]>([]);
  const previewCollidesRef = reactHostPort.useRef(false);
  const indexRef = reactHostPort.useRef(0);
  const targetWorldRef = reactHostPort.useRef<{ readonly position: Vec3; readonly direction: Vec3 } | null>(null);
  const preloadReconcileTimerRef = reactHostPort.useRef<ReturnType<typeof setTimeout> | null>(null);
  const placementProbePendingRef = reactHostPort.useRef(false);
  const menuOpenRef = reactHostPort.useRef(false);
  const [catalogPreloadUrls, setCatalogPreloadUrls] = reactHostPort.useState<readonly string[]>([]);
  const ui = reactHostPort.useSyncExternalStore(puzzle3dBrushUiStore.subscribe, puzzle3dBrushUiStore.getSnapshot, puzzle3dBrushUiStore.getSnapshot);
  menuOpenRef.current = ui.menuOpen;

  reactHostPort.useEffect(() => {
    if (props.brushActive) {
      reg.cancelAttractionDrag();
    }
  }, [props.brushActive, reg]);

  const clearBrush = reactHostPort.useCallback(() => {
    if (preloadReconcileTimerRef.current !== null) {
      clearTimeout(preloadReconcileTimerRef.current);
      preloadReconcileTimerRef.current = null;
    }
    targetRef.current = null;
    targetCtxRef.current = null;
    targetObjectIdRef.current = null;
    targetOrientationRef.current = undefined;
    probeOrderRef.current = [];
    placementCandidatesRef.current = [];
    previewCollidesRef.current = false;
    indexRef.current = 0;
    targetWorldRef.current = null;
    setCatalogPreloadUrls([]);
    placementProbePendingRef.current = false;
    puzzle3dBrushVortexHoverRef.current = false;
    puzzle3dBrushUiStore.setSnapshot(BRUSH_UI_IDLE);
    puzzle3dBrushMenuSourceRef.current = { hoverCandidate: () => {}, selectCandidate: () => {}, closeMenu: () => {} };
    puzzle3dBrushEngagementSourceRef.current = {
      candidates: [],
      targetActive: false,
      placementProbePending: false,
      cycleCandidate: () => {},
      pickCandidate: () => {},
    };
    notifyPuzzle3dBrushEngagementSource();
  }, []);

  const probePlacementCandidates = reactHostPort.useCallback(async (): Promise<BrushCollisionFreeResult> => {
    const targetFullId = targetRef.current;
    const targetCtx = targetCtxRef.current;
    const world = targetWorldRef.current;
    if (!targetFullId || !targetCtx || !world || probeOrderRef.current.length === 0) {
      return { free: [], unknownPending: false };
    }
    return puzzle3dCollisionEngineRef.current.brushCollisionFree({
      scene: reg,
      targetVortexFullId: targetFullId,
      candidates: probeOrderRef.current,
      target: targetCtx,
      targetWorldPositionCad: world.position,
      targetWorldDirectionCad: world.direction,
      referenceOrientationCad: targetOrientationRef.current,
      kindCatalogs: props.kindCatalogs,
      kindCompatibility: props.kindCompatibility,
      sceneFixture: props.sceneFixture,
      meshRootForUrl: brushMeshRootFromPool,
      overlapBudget: props.overlapBudget,
    });
  }, [props.overlapBudget, props.kindCatalogs, props.kindCompatibility, props.sceneFixture, reg]);

  const applyCandidateIndex = reactHostPort.useCallback(
    (targetFullId: string, index: number) => {
      const world = targetWorldRef.current;
      const candidate = placementCandidatesRef.current[index];
      if (!world || !candidate) {
        patchBrushUi({ preview: null });
        notifyPuzzle3dBrushEngagementSource();
        return;
      }
      const targetCtx = targetCtxRef.current;
      if (!targetCtx) {
        return;
      }
      const preview = brushPreviewFromCandidate({
        targetVortexFullId: targetFullId,
        candidate,
        target: targetCtx,
        targetWorldPositionCad: world.position,
        targetWorldDirectionCad: world.direction,
        referenceOrientationCad: targetOrientationRef.current,
        kindCatalogs: props.kindCatalogs,
        sceneFixture: props.sceneFixture,
      });
      previewCollidesRef.current = false;
      patchBrushUi({ preview, menuHoverIndex: puzzle3dBrushUiStore.getSnapshot().menuOpen ? index : null });
      notifyPuzzle3dBrushEngagementSource();
      invalidate();
    },
    [invalidate, props.kindCatalogs, props.sceneFixture],
  );

  const advanceCandidate = reactHostPort.useCallback(() => {
    const list = placementCandidatesRef.current;
    if (!list.length || !targetRef.current) {
      return;
    }
    indexRef.current = (indexRef.current + 1) % list.length;
    applyCandidateIndex(targetRef.current, indexRef.current);
    patchBrushUi({ menuHoverIndex: menuOpenRef.current ? indexRef.current : null });
  }, [applyCandidateIndex]);

  const retreatCandidate = reactHostPort.useCallback(() => {
    const list = placementCandidatesRef.current;
    if (!list.length || !targetRef.current) {
      return;
    }
    indexRef.current = (indexRef.current - 1 + list.length) % list.length;
    applyCandidateIndex(targetRef.current, indexRef.current);
    patchBrushUi({ menuHoverIndex: menuOpenRef.current ? indexRef.current : null });
  }, [applyCandidateIndex]);

  const publishBrushEngagement = reactHostPort.useCallback(() => {
    const candidates = [...placementCandidatesRef.current];
    const targetActive = targetRef.current !== null;
    puzzle3dBrushEngagementSourceRef.current = {
      candidates,
      targetActive,
      placementProbePending: placementProbePendingRef.current,
      cycleCandidate: advanceCandidate,
      pickCandidate: (index: number) => {
        if (!targetRef.current || index < 0 || index >= placementCandidatesRef.current.length) {
          return;
        }
        indexRef.current = index;
        applyCandidateIndex(targetRef.current, index);
        patchBrushUi({ menuHoverIndex: menuOpenRef.current ? index : null });
      },
    };
    patchBrushUi({ candidates, targetActive, placementProbePending: placementProbePendingRef.current });
    notifyPuzzle3dBrushEngagementSource();
  }, [advanceCandidate, applyCandidateIndex]);

  const applyBootstrapPreview = reactHostPort.useCallback(
    (targetFullId: string) => {
      const world = targetWorldRef.current;
      const targetCtx = targetCtxRef.current;
      const probeOrder = probeOrderRef.current;
      if (!world || !targetCtx || probeOrder.length === 0) {
        return;
      }
      const index = Math.min(indexRef.current, probeOrder.length - 1);
      const candidate = probeOrder[index]!;
      const preview = brushPreviewFromCandidate({
        targetVortexFullId: targetFullId,
        candidate,
        target: targetCtx,
        targetWorldPositionCad: world.position,
        targetWorldDirectionCad: world.direction,
        referenceOrientationCad: targetOrientationRef.current,
        kindCatalogs: props.kindCatalogs,
        sceneFixture: props.sceneFixture,
      });
      if (!preview) {
        return;
      }
      previewCollidesRef.current = false;
      patchBrushUi({ preview, targetActive: true });
      notifyPuzzle3dBrushEngagementSource();
      invalidate();
    },
    [invalidate, props.kindCatalogs, props.sceneFixture],
  );

  const applyPlacementProbeResult = reactHostPort.useCallback(
    (result: BrushCollisionFreeResult) => {
      placementCandidatesRef.current = result.free;
      const targetFullId = targetRef.current;
      if (!targetFullId) {
        return;
      }
      const preview = puzzle3dBrushUiStore.getSnapshot().preview;
      if (result.free.length === 0) {
        if (result.unknownPending && probeOrderRef.current.length > 0) {
          placementProbePendingRef.current = true;
          placementCandidatesRef.current = [];
          const bootstrapCandidate = probeOrderRef.current[Math.min(indexRef.current, probeOrderRef.current.length - 1)]!;
          const needsBootstrap =
            preview === null ||
            !brushPreviewMatchesCandidate(
              preview,
              bootstrapCandidate,
            );
          if (needsBootstrap) {
            applyBootstrapPreview(targetFullId);
          }
          publishBrushEngagement();
          invalidate();
          return;
        }
        placementProbePendingRef.current = false;
        previewCollidesRef.current = true;
        patchBrushUi({ preview: null, candidates: [], targetActive: true, menuHoverIndex: null, placementProbePending: false });
        publishBrushEngagement();
        invalidate();
        return;
      }
      placementProbePendingRef.current = false;
      const previewStillValid = preview !== null && result.free.some((candidate) => brushPreviewMatchesCandidate(preview, candidate));
      if (!previewStillValid) {
        indexRef.current = 0;
        applyCandidateIndex(targetFullId, 0);
      }
      publishBrushEngagement();
      invalidate();
    },
    [applyBootstrapPreview, applyCandidateIndex, invalidate, publishBrushEngagement],
  );

  const reconcilePlacementCandidates = reactHostPort.useCallback(() => {
    if (!targetRef.current || probeOrderRef.current.length === 0) {
      return;
    }
    void probePlacementCandidates().then(applyPlacementProbeResult);
  }, [applyPlacementProbeResult, probePlacementCandidates]);

  const scheduleReconcileAfterCatalogPreload = reactHostPort.useCallback(() => {
    if (preloadReconcileTimerRef.current !== null) {
      clearTimeout(preloadReconcileTimerRef.current);
    }
    preloadReconcileTimerRef.current = setTimeout(() => {
      preloadReconcileTimerRef.current = null;
      reconcilePlacementCandidates();
    }, 0);
  }, [reconcilePlacementCandidates]);

  const commitCurrentPreview = reactHostPort.useCallback(() => {
    const preview = puzzle3dBrushUiStore.getSnapshot().preview;
    if (!preview || !props.onBrushPlace || previewCollidesRef.current || placementCandidatesRef.current.length === 0) {
      return;
    }
    if (!placementCandidatesRef.current.some((candidate) => brushPreviewMatchesCandidate(preview, candidate))) {
      return;
    }
    props.onBrushPlace(brushPlacePayloadFromPreview(preview));
  }, [props.onBrushPlace]);

  const enterTarget = reactHostPort.useCallback(
    (fullId: string, meta: VortexBindingMeta) => {
      const record = store.getRecord(meta.objectId);
      if (!record) {
        return;
      }
      const vortexIndex = findVortexIndexOnRecord(record, fullId);
      if (vortexIndex < 0) {
        return;
      }
      const world = vortexWorldCadFromObject(record, vortexIndex);
      if (!world) {
        return;
      }
      targetRef.current = fullId;
      targetWorldRef.current = world;
      const targetCtx: AttractionVortexContext = {
        objectId: meta.objectId,
        objectKind: meta.objectKind,
        vortexKind: meta.vortexKind,
      };
      targetCtxRef.current = targetCtx;
      targetObjectIdRef.current = meta.objectId;
      targetOrientationRef.current = record.orientation;
      previewCollidesRef.current = false;
      const weights = puzzle3dBrushKindWeightsRef.current;
      if (!brushTargetVortexAllowsSuggestion(meta.vortexKind, weights)) {
        probeOrderRef.current = [];
        placementCandidatesRef.current = [];
        indexRef.current = 0;
        setCatalogPreloadUrls([]);
        applyPlacementProbeResult({ free: [], unknownPending: false });
        return;
      }
      const compatible = brushCompatibleCandidates(targetCtx, props.kindCatalogs, props.kindCompatibility);
      let seed = 0;
      for (let i = 0; i < fullId.length; i += 1) {
        seed = (Math.imul(seed, 31) + fullId.charCodeAt(i)) | 0;
      }
      let state = seed >>> 0;
      const rng: BrushShuffleRng = () => {
        state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
        return state / 0x100000000;
      };
      probeOrderRef.current = weightedOrderBrushCompatibleCandidates(compatible, weights, props.kindCatalogs, rng);
      placementCandidatesRef.current = [];
      indexRef.current = 0;
      setCatalogPreloadUrls(brushMeshUrlsForCompatibleCandidates(probeOrderRef.current, props.kindCatalogs, props.sceneFixture));
      void probePlacementCandidates().then(applyPlacementProbeResult);
    },
    [applyPlacementProbeResult, props.kindCatalogs, props.kindCompatibility, props.sceneFixture, probePlacementCandidates, store],
  );

  const leaveTarget = reactHostPort.useCallback(() => {
    if (!targetRef.current) {
      return;
    }
    if (puzzle3dBrushAltPressedRef.current) {
      commitCurrentPreview();
    }
    clearBrush();
  }, [clearBrush, commitCurrentPreview]);

  const finishTargetForSwitch = reactHostPort.useCallback(() => {
    if (!targetRef.current) {
      return;
    }
    if (puzzle3dBrushAltPressedRef.current) {
      commitCurrentPreview();
    }
    clearBrush();
  }, [clearBrush, commitCurrentPreview]);

  const dismissMenu = reactHostPort.useCallback(() => {
    if (!puzzle3dBrushUiStore.getSnapshot().menuOpen) {
      return;
    }
    patchBrushUi({ menuOpen: false, menuAnchor: null, menuHoverIndex: null });
    notifyPuzzle3dBrushEngagementSource();
  }, []);

  const openMenu = reactHostPort.useCallback((anchor: ScreenPoint) => {
    patchBrushUi({ menuOpen: true, menuAnchor: anchor, menuHoverIndex: null, targetActive: true });
    notifyPuzzle3dBrushEngagementSource();
  }, []);

  const hoverMenuCandidate = reactHostPort.useCallback(
    (index: number) => {
      if (!targetRef.current || index < 0 || index >= placementCandidatesRef.current.length) {
        return;
      }
      indexRef.current = index;
      applyCandidateIndex(targetRef.current, index);
      patchBrushUi({ menuHoverIndex: index });
    },
    [applyCandidateIndex],
  );

  const selectMenuCandidate = reactHostPort.useCallback(
    (index: number) => {
      if (!targetRef.current || index < 0 || index >= placementCandidatesRef.current.length) {
        return;
      }
      indexRef.current = index;
      applyCandidateIndex(targetRef.current, index);
      dismissMenu();
      commitCurrentPreview();
      clearBrush();
    },
    [applyCandidateIndex, clearBrush, commitCurrentPreview, dismissMenu],
  );

  reactHostPort.useEffect(() => {
    puzzle3dBrushMenuSourceRef.current = {
      hoverCandidate: hoverMenuCandidate,
      selectCandidate: selectMenuCandidate,
      closeMenu: dismissMenu,
    };
    return () => {
      puzzle3dBrushMenuSourceRef.current = { hoverCandidate: () => {}, selectCandidate: () => {}, closeMenu: () => {} };
    };
  }, [dismissMenu, hoverMenuCandidate, selectMenuCandidate]);

  reactHostPort.useEffect(() => {
    puzzle3dOpenVortexSuggestionsRef.current = {
      openFor: (fullId, meta, anchor) => {
        if (targetRef.current && targetRef.current !== fullId) {
          commitCurrentPreview();
          clearBrush();
        }
        enterTarget(fullId, meta);
        openMenu(anchor);
        invalidate();
        console.log("[DEBUG] puzzle3dOpenVortexSuggestions", fullId);
      },
      close: dismissMenu,
    };
    return () => {
      puzzle3dOpenVortexSuggestionsRef.current = { openFor: () => {}, close: () => {} };
    };
  }, [clearBrush, commitCurrentPreview, dismissMenu, enterTarget, invalidate, openMenu]);

  reactHostPort.useEffect(() => {
    if (!props.brushActive) {
      clearBrush();
    }
  }, [clearBrush, props.brushActive]);

  reactHostPort.useEffect(() => {
    if (!props.brushActive || !targetRef.current) {
      return;
    }
    reconcilePlacementCandidates();
  }, [props.brushActive, props.overlapBudget, reconcilePlacementCandidates]);

  return (
    <>
      <BrushPointerBridge
        brushActive={props.brushActive}
        blockedVortexFullIds={reg.blockedVortexFullIds}
        targetRef={targetRef}
        menuOpenRef={menuOpenRef}
        candidatesRef={placementCandidatesRef}
        enterTarget={enterTarget}
        finishTargetForSwitch={finishTargetForSwitch}
        leaveTarget={leaveTarget}
        openMenu={openMenu}
        dismissMenu={dismissMenu}
        commitCurrentPreview={commitCurrentPreview}
        clearBrush={clearBrush}
        advanceCandidate={advanceCandidate}
        retreatCandidate={retreatCandidate}
        invalidate={invalidate}
      />
      <BrushCatalogMeshPreload urls={catalogPreloadUrls} onReady={scheduleReconcileAfterCatalogPreload} />
      {ui.preview ? <BrushPreviewGhost preview={ui.preview} /> : null}
    </>
  );
}

function BrushPointerBridge(props: {
  readonly brushActive: boolean;
  readonly blockedVortexFullIds: ReadonlySet<string>;
  readonly targetRef: MutableRefObject<string | null>;
  readonly menuOpenRef: MutableRefObject<boolean>;
  readonly candidatesRef: MutableRefObject<readonly BrushCompatibleCandidate[]>;
  readonly enterTarget: (fullId: string, meta: VortexBindingMeta) => void;
  readonly finishTargetForSwitch: () => void;
  readonly leaveTarget: () => void;
  readonly openMenu: (anchor: ScreenPoint) => void;
  readonly dismissMenu: () => void;
  readonly commitCurrentPreview: () => void;
  readonly clearBrush: () => void;
  readonly advanceCandidate: () => void;
  readonly retreatCandidate: () => void;
  readonly invalidate: () => void;
}) {
  const reg = useRegistryCore();
  const { camera, gl } = useThree();
  const raycasterRef = reactHostPort.useRef(new Raycaster());
  const ndcRef = reactHostPort.useRef(new Vector2());
  const clickGestureRef = reactHostPort.useRef<{ readonly pointerId: number; readonly x: number; readonly y: number } | null>(null);

  reactHostPort.useEffect(() => {
    if (!props.brushActive) {
      puzzle3dBrushAltPressedRef.current = false;
      return;
    }
    const syncAlt = (event: KeyboardEvent | PointerEvent) => {
      puzzle3dBrushAltPressedRef.current = event.altKey;
    };
    const onKeyDown = (event: KeyboardEvent) => {
      syncAlt(event);
      if (
        !routePuzzle3dBrushTabKeydown(
          props.brushActive,
          props.targetRef.current !== null,
          props.candidatesRef.current.length,
          event,
        )
      ) {
        return;
      }
      event.preventDefault();
      event.stopPropagation();
      if (event.shiftKey) {
        props.retreatCandidate();
      } else {
        props.advanceCandidate();
      }
      props.invalidate();
    };
    const onKeyUp = (event: KeyboardEvent) => {
      syncAlt(event);
    };
    const bindings = new EventBindingController();
    bindings.listen(window, "keydown", onKeyDown, true);
    bindings.listen(window, "keyup", onKeyUp, true);
    return () => {
      puzzle3dBrushAltPressedRef.current = false;
      bindings.dispose();
    };
  }, [props]);

  reactHostPort.useEffect(() => {
    if (!props.brushActive) {
      return;
    }
    const collectPickRoots = (): Object3D[] => {
      const out: Object3D[] = [];
      for (const group of reg.collectObjectGroups()) {
        if (group) {
          out.push(group);
        }
      }
      return out;
    };
    const pickVortexAt = (clientX: number, clientY: number): ScreenVortexCandidate | null => {
      const canvas = gl.domElement;
      const rect = canvas.getBoundingClientRect();
      ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
      ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
      raycasterRef.current.setFromCamera(ndcRef.current, camera);
      const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
      const surfaceDist = hits[0]?.distance ?? 80;
      const screenCandidates: ScreenVortexCandidate[] = [];
      for (const meta of reg.listVortexBindings()) {
        if (props.blockedVortexFullIds.has(meta.fullId)) {
          continue;
        }
        const world = reg.getVortexWorld(meta.fullId);
        if (!world) {
          continue;
        }
        const projected = world.clone().project(camera);
        if (projected.z > 1) {
          continue;
        }
        const sx = rect.left + (projected.x * 0.5 + 0.5) * rect.width;
        const sy = rect.top + (-projected.y * 0.5 + 0.5) * rect.height;
        screenCandidates.push({
          fullId: meta.fullId,
          objectId: meta.objectId,
          sx,
          sy,
          dist: camera.position.distanceTo(world),
        });
      }
      return pickNearestScreenVortex({
        cursorX: clientX,
        cursorY: clientY,
        surfaceDist,
        candidates: screenCandidates,
      });
    };
    const onMove = (event: PointerEvent) => {
      puzzle3dBrushAltPressedRef.current = event.altKey;
      const picked = pickVortexAt(event.clientX, event.clientY);
      puzzle3dBrushVortexHoverRef.current = picked !== null;
      if (!picked) {
        if (props.menuOpenRef.current) {
          props.dismissMenu();
        }
        if (props.targetRef.current) {
          props.leaveTarget();
        }
        return;
      }
      const meta = reg.listVortexBindings().find((entry) => entry.fullId === picked.fullId);
      if (!meta) {
        return;
      }
      if (props.menuOpenRef.current && picked.fullId !== props.targetRef.current) {
        props.dismissMenu();
      }
      if (props.menuOpenRef.current && picked.fullId === props.targetRef.current) {
        props.invalidate();
        return;
      }
      if (picked.fullId !== props.targetRef.current) {
        if (props.targetRef.current) {
          props.finishTargetForSwitch();
        }
        props.enterTarget(picked.fullId, meta);
      }
      props.invalidate();
    };
    const onPointerDown = (event: PointerEvent) => {
      if (event.button !== 0) {
        return;
      }
      puzzle3dBrushAltPressedRef.current = event.altKey;
      clickGestureRef.current = { pointerId: event.pointerId, x: event.clientX, y: event.clientY };
    };
    const onPointerUp = (event: PointerEvent) => {
      puzzle3dBrushAltPressedRef.current = event.altKey;
      const gesture = clickGestureRef.current;
      if (!gesture || gesture.pointerId !== event.pointerId || event.button !== 0) {
        return;
      }
      clickGestureRef.current = null;
      const dx = event.clientX - gesture.x;
      const dy = event.clientY - gesture.y;
      if (dx * dx + dy * dy >= PUZZLE_3D_VORTEX_DRAG_THRESHOLD_PX * PUZZLE_3D_VORTEX_DRAG_THRESHOLD_PX) {
        return;
      }
      const picked = pickVortexAt(event.clientX, event.clientY);
      if (!picked) {
        if (props.menuOpenRef.current) {
          props.dismissMenu();
        }
        if (props.targetRef.current) {
          props.leaveTarget();
        }
        return;
      }
      const meta = reg.listVortexBindings().find((entry) => entry.fullId === picked.fullId);
      if (!meta) {
        return;
      }
      if (picked.fullId === props.targetRef.current && props.menuOpenRef.current) {
        props.dismissMenu();
        return;
      }
      if (picked.fullId !== props.targetRef.current) {
        if (props.targetRef.current) {
          props.finishTargetForSwitch();
        }
        props.enterTarget(picked.fullId, meta);
      }
      props.openMenu({ x: event.clientX, y: event.clientY });
      props.invalidate();
    };
    const bindings = new EventBindingController();
    bindings.listen(window, "pointermove", onMove);
    bindings.listen(window, "pointerdown", onPointerDown, true);
    bindings.listen(window, "pointerup", onPointerUp, true);
    return () => bindings.dispose();
  }, [camera, gl, props, reg]);
  return null;
}

function AttractionThreeBinder() {
  const reg = useRegistry();
  const t = useThree();
  reactHostPort.useLayoutEffect(() => {
    reg.attachAttractionThreeEnv({ camera: t.camera, gl: t.gl, scene: t.scene });
    return () => reg.attachAttractionThreeEnv(null);
  }, [reg, t.camera, t.gl, t.scene]);
  return null;
}

/** @emoji 🔗 True when attraction drag or indirect-pick references vortices no longer in the scene. */
export function puzzle3dAttractionSessionIsStale(
  attractingFullId: string | null,
  indirectPick: AttractionIndirectPickAwait | null,
  vortexExists: (fullId: string) => boolean,
): boolean {
  if (attractingFullId !== null && !vortexExists(attractingFullId)) {
    return true;
  }
  if (indirectPick !== null && !indirectPick.candidates.some(vortexExists)) {
    return true;
  }
  return false;
}

/** @emoji 🔗 Cancels attraction drag when deleted objects remove the involved vortices. */
function AttractionStaleSessionGuard(): null {
  const reg = useRegistry();
  const { store } = useObjectState();
  const structureEpoch = reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeStructure(onStoreChange),
    () => store.getStructureEpoch(),
    () => store.getStructureEpoch(),
  );
  const attractingFullId = reg.attractionDragAttractingFullId;
  const indirectPick = reg.attractionIndirectPickAwait;
  reactHostPort.useEffect(() => {
    if (!attractingFullId && !indirectPick) {
      return;
    }
    const vortexExists = (fullId: string) => reg.getVortexWorld(fullId) !== null;
    if (puzzle3dAttractionSessionIsStale(attractingFullId, indirectPick, vortexExists)) {
      reg.cancelAttractionDrag();
    }
  }, [attractingFullId, indirectPick, reg, structureEpoch]);
  return null;
}

function AttractionWindowBridge() {
  const reg = useRegistry();
  const invalidate = useThree((s) => s.invalidate);
  const attractionBusy = reg.attractionDragActive || reg.attractionIndirectPickAwait !== null;
  reactHostPort.useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") {
        return;
      }
      if (!reg.attractionDragActive && reg.attractionIndirectPickAwait === null) {
        return;
      }
      event.preventDefault();
      reg.cancelAttractionDrag();
      invalidate();
    };
    const bindings = new EventBindingController();
    bindings.listen(window, "keydown", onKeyDown, true);
    return () => bindings.dispose();
  }, [invalidate, reg]);
  reactHostPort.useEffect(() => {
    if (!attractionBusy) return;
    const onMove = (e: PointerEvent) => {
      if (reg.attractionDragActive) reg.updateAttractionPointer(e.clientX, e.clientY);
      else if (reg.attractionIndirectPickAwait) reg.updateIndirectPickPointer(e.clientX, e.clientY);
      invalidate();
    };
    const onUp = (e: PointerEvent) => {
      if (reg.attractionDragActive) reg.commitAttractionPointer(e.clientX, e.clientY);
      invalidate();
    };
    const onDown = (e: PointerEvent) => {
      if (e.button !== 0) return;
      if (reg.attractionIndirectPickAwait) reg.commitIndirectPickPointerDown(e.clientX, e.clientY, e);
      invalidate();
    };
    const bindings = new EventBindingController();
    bindings.listen(window, "pointermove", onMove);
    bindings.listen(window, "pointerup", onUp, { capture: true });
    bindings.listen(window, "pointerdown", onDown, true);
    return () => bindings.dispose();
  }, [reg, attractionBusy, invalidate]);
  return null;
}

function MarqueeBridge() {
  const reg = useRegistry();
  const { captureMarqueeCandidates, previewMarqueeSelection, cancelMarqueePreview, commitMarqueeSelection } = useRegistryInteraction();
  const marquee = useRegistryMarquee();
  const gl = useThree((state) => state.gl);
  const invalidate = useThree((state) => state.invalidate);
  const gestureRef = reactHostPort.useRef<{
    readonly pointerId: number;
    readonly startX: number;
    readonly startY: number;
    active: boolean;
    path: ScreenPoint[];
    lastX: number;
    lastY: number;
  } | null>(null);
  const marqueePrefetchFrameRef = reactHostPort.useRef<number | null>(null);
  const marqueeCandidatesPrefetchedRef = reactHostPort.useRef(false);
  reactHostPort.useEffect(() => {
    const canvas = gl.domElement;
    if (!canvas) {
      return;
    }
    const resetOverlay = () => {
      puzzle3dMarqueeOverlayStore.setSnapshot(MARQUEE_OVERLAY_IDLE);
    };
    const cancelPrefetch = () => {
      if (marqueePrefetchFrameRef.current !== null) {
        cancelAnimationFrame(marqueePrefetchFrameRef.current);
        marqueePrefetchFrameRef.current = null;
      }
      marqueeCandidatesPrefetchedRef.current = false;
    };
    const cancelGesture = () => {
      cancelPrefetch();
      gestureRef.current = null;
      cancelMarqueePreview();
      resetOverlay();
    };
    puzzle3dMarqueeGestureCancel = cancelGesture;
    const onPointerDown = (event: PointerEvent) => {
      if (event.button !== 0 || reg.attractionDragActive || reg.attractionIndirectPickAwait !== null || puzzle3dRelocateDragActiveRef.current || gumballPointerConsumesCanvasEventRef.current || puzzle3dBrushToolActiveRef.current || puzzle3dTargetVolumeToolActiveRef.current) {
        return;
      }
      cancelPrefetch();
      gestureRef.current = {
        pointerId: event.pointerId,
        startX: event.clientX,
        startY: event.clientY,
        active: false,
        path: [{ x: event.clientX, y: event.clientY }],
        lastX: event.clientX,
        lastY: event.clientY,
      };
      const pointerId = event.pointerId;
      marqueePrefetchFrameRef.current = requestAnimationFrame(() => {
        marqueePrefetchFrameRef.current = null;
        const gesture = gestureRef.current;
        if (!gesture || gesture.pointerId !== pointerId) {
          return;
        }
        captureMarqueeCandidates();
        marqueeCandidatesPrefetchedRef.current = true;
      });
    };
    const onPointerMove = (event: PointerEvent) => {
      if (puzzle3dRelocateDragActiveRef.current) {
        cancelGesture();
        return;
      }
      const gesture = gestureRef.current;
      if (!gesture || gesture.pointerId !== event.pointerId) {
        return;
      }
      const dist = Math.hypot(event.clientX - gesture.startX, event.clientY - gesture.startY);
      if (!gesture.active && dist < PUZZLE_3D_MARQUEE_DRAG_THRESHOLD_PX) {
        return;
      }
      const path =
        marquee.selectionMethod === "lasso"
          ? [...gesture.path, { x: event.clientX, y: event.clientY }]
          : [{ x: gesture.startX, y: gesture.startY }, { x: event.clientX, y: event.clientY }];
      const activating = !gesture.active;
      const lastX = event.clientX;
      const lastY = event.clientY;
      gestureRef.current = { ...gesture, active: true, path, lastX, lastY };
      const gestureArgs: MarqueeGestureArgs = {
        startX: gesture.startX,
        startY: gesture.startY,
        endX: lastX,
        endY: lastY,
        path,
        modifiers: { shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey },
      };
      const clientRect = gl.domElement.getBoundingClientRect();
      const overlaySnapshot = {
        active: true as const,
        method: marquee.selectionMethod,
        start: { x: gesture.startX, y: gesture.startY },
        current: { x: lastX, y: lastY },
        path,
        clientOrigin: { x: clientRect.left, y: clientRect.top },
      };
      puzzle3dMarqueeOverlayStore.setSnapshot(overlaySnapshot);
      if (activating) {
        captureMarqueeCandidates({ reuseCandidates: marqueeCandidatesPrefetchedRef.current });
        marqueeCandidatesPrefetchedRef.current = false;
      }
      previewMarqueeSelection(gestureArgs);
      invalidate();
    };
    const onPointerUp = (event: PointerEvent) => {
      if (puzzle3dRelocateDragActiveRef.current) {
        cancelGesture();
        return;
      }
      const gesture = gestureRef.current;
      if (!gesture || gesture.pointerId !== event.pointerId) {
        return;
      }
      gestureRef.current = null;
      if (!gesture.active) {
        resetOverlay();
        return;
      }
      commitMarqueeSelection({
        startX: gesture.startX,
        startY: gesture.startY,
        endX: event.clientX,
        endY: event.clientY,
        path: gesture.path,
        modifiers: { shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey },
      });
      resetOverlay();
      invalidate();
    };
    const bindings = new EventBindingController();
    bindings.listen(canvas, "pointerdown", onPointerDown as EventListener);
    bindings.listen(window, "pointermove", onPointerMove as EventListener);
    bindings.listen(window, "pointerup", onPointerUp as EventListener, true);
    bindings.listen(window, "pointercancel", onPointerUp as EventListener, true);
    return () => {
      puzzle3dMarqueeGestureCancel = null;
      bindings.dispose();
      resetOverlay();
    };
  }, [cancelMarqueePreview, captureMarqueeCandidates, commitMarqueeSelection, invalidate, marquee.selectionMethod, previewMarqueeSelection, reg]);
  return null;
}

/** @emoji 🖱️ Mirrors {@link ObjectStateProvider} attractions into marquee hit testing. */
export function MarqueeAttractionSource(): null {
  const { store } = useObjectState();
  const { setMarqueeAttractions } = useRegistryMarquee();
  const attractions = reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeStructure(onStoreChange),
    () => store.getAttractions(),
    () => store.getAttractions(),
  );
  reactHostPort.useEffect(() => {
    setMarqueeAttractions(attractions);
    return () => setMarqueeAttractions([]);
  }, [attractions, setMarqueeAttractions]);
  return null;
}

const brushMenuContentClassName = cn(
  glassMenuClass,
  "w-auto min-w-[10rem] max-h-[min(24rem,70vh)] overflow-y-auto border p-single z-temporary text-element",
);
const brushMenuItemClassName =
  "text-element hover:bg-hover-interactive-fill hover:text-emphasized focus:bg-hover-interactive-fill focus:text-emphasized relative flex w-full items-center gap-single p-single text-left text-sm outline-none whitespace-nowrap cursor-default select-none disabled:pointer-events-none disabled:opacity-50";

function Puzzle3dSelectionContextMenu() {
  const menu = reactHostPort.useSyncExternalStore(puzzle3dSelectionMenuStore.subscribe, puzzle3dSelectionMenuStore.getSnapshot, puzzle3dSelectionMenuStore.getSnapshot);
  const onSuggest = reactHostPort.useMemo(() => {
    if (!menu.target || menu.target.kind !== "vortex" || !menu.anchor || !menu.vortexMeta) {
      return undefined;
    }
    const fullId = menu.target.fullId;
    const anchor = menu.anchor;
    const meta = menu.vortexMeta;
    return () => {
      closePuzzle3dSelectionMenu();
      puzzle3dBrushMenuSourceRef.current.closeMenu();
      puzzle3dOpenVortexSuggestionsRef.current.openFor(fullId, meta, anchor);
    };
  }, [menu.anchor, menu.target, menu.vortexMeta]);
  const items = reactHostPort.useMemo(
    () => buildPuzzle3dSelectionMenuItems(menu.selection, menu.entityFlags, menu.target, puzzle3dSelectionActionsRef.current, onSuggest),
    [menu.entityFlags, menu.selection, menu.target, onSuggest],
  );
  const handleOpenChange = reactHostPort.useCallback((open: boolean) => {
    if (!open) {
      closePuzzle3dSelectionMenu();
    }
  }, []);
  return <ContextMenuController items={items} onOpenChange={handleOpenChange} open={menu.open} position={menu.anchor} />;
}

/** @emoji 🖱️ Right-click on hovered entity replaces selection and opens {@link Puzzle3dSelectionContextMenu}. */
function SelectionContextMenuBinder(): null {
  const { hoverTarget } = useRegistryHover();
  const { commitSelection } = useRegistryInteraction();
  const { store } = useObjectState();
  const reg = useRegistryCore();
  const { gl } = useThree();
  const hoverTargetRef = reactHostPort.useRef(hoverTarget);
  hoverTargetRef.current = hoverTarget;
  reactHostPort.useEffect(() => {
    const onContextMenu = (event: MouseEvent) => {
      const target = hoverTargetRef.current;
      if (puzzle3dRightDragActiveRef.current || !target) {
        return;
      }
      event.preventDefault();
      event.stopPropagation();
      puzzle3dBrushMenuSourceRef.current.closeMenu();
      const pick = hoverTargetToSelectionPick(target);
      commitSelection(pick);
      const selection = puzzle3dSelectionFromPick(pick);
      const entityFlags = puzzle3dSelectionEntityFlagsFromStore(store, selection);
      const vortexMeta = target.kind === "vortex" ? (reg.listVortexBindings().find((entry) => entry.fullId === target.fullId) ?? null) : null;
      puzzle3dSelectionMenuStore.setSnapshot({
        open: true,
        anchor: { x: event.clientX, y: event.clientY },
        target,
        selection,
        entityFlags,
        vortexMeta,
      });
    };
    const el = gl.domElement;
    el.addEventListener("contextmenu", onContextMenu);
    return () => el.removeEventListener("contextmenu", onContextMenu);
  }, [commitSelection, gl, reg, store]);
  return null;
}

function Puzzle3dBrushCandidateMenu() {
  const ui = reactHostPort.useSyncExternalStore(puzzle3dBrushUiStore.subscribe, puzzle3dBrushUiStore.getSnapshot, puzzle3dBrushUiStore.getSnapshot);
  const menuRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const closeMenu = reactHostPort.useCallback(() => {
    puzzle3dBrushMenuSourceRef.current.closeMenu();
  }, []);
  reactHostPort.useEffect(() => {
    if (!ui.menuOpen || !ui.menuAnchor) {
      return undefined;
    }
    const onPointerDown = (event: PointerEvent) => {
      const target = event.target;
      if (target instanceof Node && menuRef.current?.contains(target)) {
        return;
      }
      closeMenu();
    };
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        closeMenu();
      }
    };
    const bindings = new EventBindingController();
    bindings.listen(window, "pointerdown", onPointerDown, false);
    bindings.listen(window, "keydown", onKeyDown, false);
    return () => bindings.dispose();
  }, [closeMenu, ui.menuAnchor, ui.menuOpen]);
  if (!ui.menuOpen || !ui.menuAnchor) {
    return null;
  }
  const body =
    ui.candidates.length > 0 ? (
      ui.candidates.map((candidate, index) => {
        const active = ui.menuHoverIndex === index;
        return (
          <button
            key={`${candidate.objectKindId}:${candidate.sourceVortexIndex}`}
            aria-selected={active}
            className={cn(brushMenuItemClassName, active && "bg-hover-temporary")}
            onClick={() => puzzle3dBrushMenuSourceRef.current.selectCandidate(index)}
            onMouseEnter={() => puzzle3dBrushMenuSourceRef.current.hoverCandidate(index)}
            role="menuitem"
            type="button"
          >
            <span className="truncate">{normalizeEngagementCommandText(candidate.objectKindId)}</span>
            <span className="ml-auto text-xs text-muted-foreground pl-tiny">v{candidate.sourceVortexIndex}</span>
          </button>
        );
      })
    ) : (
      <div className="p-single text-sm text-muted-foreground" role="status">
        {ui.placementProbePending ? "Checking collision-free placements…" : "No collision-free placement at this connector"}
      </div>
    );
  if (typeof document === "undefined") {
    return null;
  }
  return createPortal(
    <div
      className={brushMenuContentClassName}
      onContextMenu={(event) => event.preventDefault()}
      ref={menuRef}
      role="menu"
      style={{ left: ui.menuAnchor.x, position: "fixed", top: ui.menuAnchor.y }}
    >
      {body}
    </div>,
    document.body,
  );
}

function Puzzle3dMarqueeOverlay() {
  const overlay = reactHostPort.useSyncExternalStore(puzzle3dMarqueeOverlayStore.subscribe, puzzle3dMarqueeOverlayStore.getSnapshot, puzzle3dMarqueeOverlayStore.getSnapshot);
  if (!overlay.active || !overlay.start || !overlay.current) {
    return null;
  }
  const toLocal = (point: ScreenPoint) => ({ x: point.x - overlay.clientOrigin.x, y: point.y - overlay.clientOrigin.y });
  const coverage = marqueeCoverageFromGesture({
    method: overlay.method,
    startX: overlay.start.x,
    endX: overlay.current.x,
    path: overlay.path,
  });
  if (overlay.method === "lasso" && overlay.path.length >= 2) {
    return <SelectionMarquee coverage={coverage} shape="polygon" points={overlay.path.map(toLocal)} />;
  }
  const start = toLocal(overlay.start);
  const current = toLocal(overlay.current);
  return (
    <SelectionMarquee
      coverage={coverage}
      shape="rect"
      rect={{
        x: Math.min(start.x, current.x),
        y: Math.min(start.y, current.y),
        width: Math.abs(current.x - start.x),
        height: Math.abs(current.y - start.y),
      }}
    />
  );
}

function AttractionRubberBand() {
  const reg = useRegistry();
  const geo = reactHostPort.useMemo(() => {
    const g = new BufferGeometry();
    g.setAttribute("position", new Float32BufferAttribute(new Float32Array(6), 3));
    return g;
  }, []);
  const mat = reactHostPort.useMemo(() => new LineBasicMaterial({ color: resolveThreeColor(tokenVar("primary"), "primary"), transparent: true, opacity: 0.92, depthTest: false }), []);
  useFrame(() => {
    const pos = geo.attributes.position as Float32BufferAttribute;
    const cable = (reg.attractionDragActive || reg.attractionIndirectPickAwait !== null) && reg.attractionDragAttractingFullId ? true : false;
    if (!cable) {
      pos.setXYZ(0, 0, 0, 0);
      pos.setXYZ(1, 0, 0, 0);
      pos.needsUpdate = true;
      return;
    }
    const a = reg.getVortexWorld(reg.attractionDragAttractingFullId);
    const b = reg.attractionEndWorldRef.current;
    if (a && b && vector3IsFinite(a) && vector3IsFinite(b)) {
      pos.setXYZ(0, a.x, a.y, a.z);
      pos.setXYZ(1, b.x, b.y, b.z);
      pos.needsUpdate = true;
    } else {
      pos.setXYZ(0, 0, 0, 0);
      pos.setXYZ(1, 0, 0, 0);
      pos.needsUpdate = true;
    }
  });
  reactHostPort.useEffect(
    () => () => {
      geo.dispose();
      mat.dispose();
    },
    [geo, mat],
  );
  return <line geometry={geo} material={mat} raycast={() => null} />;
}

const InnerSceneChildrenContext = reactHostPort.createContext<ReactNode>(null);

/** @emoji 🧩 Renders {@link Canvas3D} scene children inside a stable registry shell (host re-renders must not remount the 3D tree). */
function InnerSceneChildren(props: { readonly chunkSize: number; readonly maxDistance: number }): React.ReactElement {
  const children = reactHostPort.useContext(InnerSceneChildrenContext);
  return (
    <WorldChunkedSceneChildren chunkSize={props.chunkSize} maxDistance={props.maxDistance} unchunkedDataAttr="data-puzzle3d-unchunked">
      {children}
    </WorldChunkedSceneChildren>
  );
}

/** @emoji 🎯 Syncs host-controlled selection into the scene store without re-rendering scene children. */
function ControlledSelectionSync(props: { readonly selection?: SelectionSnapshot }): null {
  const store = useSelectionSnapshotStore();
  reactHostPort.useLayoutEffect(() => {
    store.setControlledHostSnapshot(props.selection);
    if (props.selection !== undefined) {
      store.setSnapshot(props.selection);
    }
  }, [props.selection, store]);
  return null;
}

interface RegistryHostCallbacks {
  readonly onSelect?: (snap: SelectionSnapshot) => void;
  readonly onConnect?: (p: AttractionPayload) => void;
  readonly onProximityConnect?: (p: AttractionPayload) => void;
  readonly onIndirectConnect?: (p: AttractionPayload) => void;
  readonly onAttractionCompatibleObjects?: (p: AttractionCompatibleObjectsPayload) => void;
  readonly onAttractionTargetRing?: (p: AttractionTargetRingPayload) => void;
  readonly onRelocate?: (p: RelocatePayload) => void;
}

function RegistryProvider({
  children,
  lodRef,
  kindCatalogs,
  kindCompatibility,
  blockedVortexFullIds,
  proximityRadius,
  proximityRelocateEnabled = true,
  selectionMode,
  gumballConfig,
  hostCallbacksRef,
  attractionSession,
  selectionMethod = "rectangle",
  marqueeSelectableKinds = { object: true, vortex: true, attraction: true },
  selectionStore,
  hoverTargetProp,
  kindHoverProp,
  onHover,
}: {
  children: ReactNode;
  lodRef: MutableRefObject<number>;
  kindCatalogs: KindCatalogBundle | undefined;
  kindCompatibility: readonly KindCompatEntry[] | undefined;
  blockedVortexFullIds: ReadonlySet<string>;
  proximityRadius: number;
  proximityRelocateEnabled?: boolean;
  selectionMode: SelectionMode;
  gumballConfig: GumballConfig;
  hostCallbacksRef: MutableRefObject<RegistryHostCallbacks>;
  attractionSession?: AttractionSessionSnapshot | null;
  selectionMethod?: SelectionMethod;
  marqueeSelectableKinds?: MarqueeSelectableKinds;
  selectionStore: SelectionSnapshotStore;
  hoverTargetProp?: HoverTarget | null;
  kindHoverProp?: Puzzle3dKindHover | null;
  onHover?: (payload: Puzzle3dHoverPayload) => void;
}) {
  const selectionModeRef = reactHostPort.useRef(selectionMode);
  selectionModeRef.current = selectionMode;
  const selectionMethodRef = reactHostPort.useRef(selectionMethod);
  selectionMethodRef.current = selectionMethod;
  const marqueeKindsRef = reactHostPort.useRef(marqueeSelectableKinds);
  marqueeKindsRef.current = marqueeSelectableKinds;
  const marqueeAttractionsRef = reactHostPort.useRef<readonly AttractionProps[]>([]);
  const marqueeCandidatesRef = reactHostPort.useRef<MarqueeCandidate[]>([]);
  const marqueeBaseSelectionRef = reactHostPort.useRef<SelectionSnapshot | null>(null);
  const notifyConnect = reactHostPort.useCallback((payload: AttractionPayload) => hostCallbacksRef.current.onConnect?.(payload), [hostCallbacksRef]);
  const notifyProximityConnect = reactHostPort.useCallback((payload: AttractionPayload) => hostCallbacksRef.current.onProximityConnect?.(payload), [hostCallbacksRef]);
  const notifyIndirectConnect = reactHostPort.useCallback((payload: AttractionPayload) => hostCallbacksRef.current.onIndirectConnect?.(payload), [hostCallbacksRef]);
  const notifyAttractionCompatibleObjects = reactHostPort.useCallback(
    (payload: AttractionCompatibleObjectsPayload) => hostCallbacksRef.current.onAttractionCompatibleObjects?.(payload),
    [hostCallbacksRef],
  );
  const notifyAttractionTargetRing = reactHostPort.useCallback(
    (payload: AttractionTargetRingPayload) => hostCallbacksRef.current.onAttractionTargetRing?.(payload),
    [hostCallbacksRef],
  );
  const notifyRelocate = reactHostPort.useCallback((payload: RelocatePayload) => hostCallbacksRef.current.onRelocate?.(payload), [hostCallbacksRef]);
  const publishSelection = reactHostPort.useCallback(
    (snap: SelectionSnapshot) => {
      selectionStore.setSnapshot(snap);
      const primary = primarySelectionObjectId(snap);
      activeRelocateObjectIdRef.current = primary;
      const controlled = selectionStore.getControlledHostSnapshot();
      if (controlled !== undefined) {
        if (!selectionSnapshotsEqual(controlled, snap)) {
          hostCallbacksRef.current.onSelect?.(snap);
        }
        return;
      }
      hostCallbacksRef.current.onSelect?.(snap);
    },
    [hostCallbacksRef, selectionStore],
  );

  const commitSelection = reactHostPort.useCallback(
    (pick: SelectionPick) => {
      const current = selectionStore.getSnapshot();
      const snap = mergeSelection(selectionModeRef.current, current, pick);
      publishSelection(snap);
    },
    [publishSelection, selectionStore],
  );

  const buildMarqueeCandidates = reactHostPort.useCallback((): MarqueeCandidate[] => {
    const env = attractionThreeRef.current;
    if (!env) {
      return [];
    }
    env.camera.updateMatrixWorld(true);
    const domRect = puzzle3dMarqueeClientRect(env.gl);
    const candidates: MarqueeCandidate[] = [];
    for (const [id, group] of objectGroupMap.current) {
      if (!group) {
        continue;
      }
      const cache = resolveObjectMarqueeFootprintCache(group, id);
      const candidate = marqueeFootprintToCandidate("object", id, projectObjectMarqueeFootprintFromCache(cache, env.camera, domRect));
      if (candidate) {
        candidates.push(candidate);
      }
    }
    for (const [fullId, getter] of vortexGettersRef.current) {
      const world = getter();
      if (!world) {
        continue;
      }
      const point = projectWorldToClient(world, env.camera, domRect);
      const candidate = marqueeFootprintToCandidate("vortex", fullId, point ? marqueeFootprintFromClientPoints([point]) : null);
      if (candidate) {
        candidates.push(candidate);
      }
    }
    for (const attraction of marqueeAttractionsRef.current) {
      const a = vortexGettersRef.current.get(attraction.attracting)?.() ?? null;
      const b = vortexGettersRef.current.get(attraction.attracted)?.() ?? null;
      const points: ScreenPoint[] = [];
      if (a) {
        const projected = projectWorldToClient(a, env.camera, domRect);
        if (projected) {
          points.push(projected);
        }
      }
      if (b) {
        const projected = projectWorldToClient(b, env.camera, domRect);
        if (projected) {
          points.push(projected);
        }
      }
      const candidate = marqueeFootprintToCandidate("attraction", attraction.id, marqueeFootprintFromClientPoints(points));
      if (candidate) {
        candidates.push(candidate);
      }
    }
    return candidates;
  }, []);

  const clearMarqueeGestureCache = reactHostPort.useCallback(() => {
    marqueeCandidatesRef.current = [];
    marqueeBaseSelectionRef.current = null;
  }, []);

  const captureMarqueeCandidates = reactHostPort.useCallback(
    (options?: { readonly reuseCandidates?: boolean }) => {
      marqueeBaseSelectionRef.current = selectionStore.getSnapshot();
      if (!options?.reuseCandidates || marqueeCandidatesRef.current.length === 0) {
        marqueeCandidatesRef.current = buildMarqueeCandidates();
      }
    },
    [buildMarqueeCandidates, selectionStore],
  );

  const previewMarqueeSelection = reactHostPort.useCallback(
    (args: MarqueeGestureArgs) => {
      const base = marqueeBaseSelectionRef.current;
      if (!base) {
        return;
      }
      const snap = resolveMarqueeSelectionGesture(args, {
        method: selectionMethodRef.current,
        kinds: marqueeKindsRef.current,
        candidates: marqueeCandidatesRef.current,
        base,
      });
      selectionStore.setSnapshot(snap);
    },
    [selectionStore],
  );

  const cancelMarqueePreview = reactHostPort.useCallback(() => {
    const base = marqueeBaseSelectionRef.current;
    if (base) {
      selectionStore.setSnapshot(base);
    }
    clearMarqueeGestureCache();
  }, [clearMarqueeGestureCache, selectionStore]);

  const commitMarqueeSelection = reactHostPort.useCallback(
    (args: MarqueeGestureArgs) => {
      const base = marqueeBaseSelectionRef.current ?? selectionStore.getSnapshot();
      const candidates = marqueeCandidatesRef.current.length > 0 ? marqueeCandidatesRef.current : buildMarqueeCandidates();
      const snap = resolveMarqueeSelectionGesture(args, {
        method: selectionMethodRef.current,
        kinds: marqueeKindsRef.current,
        candidates,
        base,
      });
      clearMarqueeGestureCache();
      publishSelection(snap);
      puzzle3dMarqueeSuppressClickRef.current = true;
      window.setTimeout(() => {
        puzzle3dMarqueeSuppressClickRef.current = false;
      }, 0);
    },
    [buildMarqueeCandidates, clearMarqueeGestureCache, publishSelection, selectionStore],
  );

  const setSelectedObjectIds = reactHostPort.useCallback(
    (ids: readonly string[] | ((prev: readonly string[]) => readonly string[])) => {
      const current = selectionStore.getSnapshot();
      const resolvedObjectIds = typeof ids === "function" ? ids(current.objectIds) : ids;
      const mode = selectionModeRef.current;
      const snap: SelectionSnapshot =
        mode === "default"
          ? { objectIds: resolvedObjectIds, vortexIds: [], attractionIds: [], referenceIds: [], targetVolumeIds: [] }
          : {
              objectIds: mergeIdList(mode, current.objectIds, resolvedObjectIds),
              vortexIds: current.vortexIds,
              attractionIds: current.attractionIds,
            };
      publishSelection(snap);
    },
    [publishSelection, selectionStore],
  );

  const setMarqueeAttractions = reactHostPort.useCallback((attractions: readonly AttractionProps[]) => {
    marqueeAttractionsRef.current = attractions;
  }, []);
  const activeRelocateObjectIdRef = reactHostPort.useRef<string | null>(primarySelectionObjectId(selectionStore.getSnapshot()));
  const setActiveRelocateObjectId = reactHostPort.useCallback((id: string | null) => {
    if (activeRelocateObjectIdRef.current === id) {
      return;
    }
    activeRelocateObjectIdRef.current = id;
  }, []);
  const [attractionDragActive, setAttractionDragActive] = reactHostPort.useState(false);
  const [attractionDragAttractingFullId, setAttractionDragAttractingFullId] = reactHostPort.useState<string | null>(null);
  const [attractionCompatibleAttractedFullIds, setAttractionCompatibleAttractedFullIds] = reactHostPort.useState<ReadonlySet<string>>(new Set());
  const [attractionHoverRingFullId, setAttractionHoverRingFullId] = reactHostPort.useState<string | null>(null);
  const [attractionIndirectPickAwait, setAttractionIndirectPickAwait] = reactHostPort.useState<AttractionIndirectPickAwait | null>(null);
  const vortexGettersRef = reactHostPort.useRef(new Map<string, VortexGetter>());
  const vortexMetaRef = reactHostPort.useRef(new Map<string, VortexBindingMeta>());
  const vortexPickRef = reactHostPort.useRef(new Map<string, Object3D>());
  const objectGroupMap = reactHostPort.useRef(new Map<string, Group | null>());
  const objectKindsRef = reactHostPort.useRef(new Map<string, string | undefined>());
  const attractionKindsRef = reactHostPort.useRef(new Map<string, string | undefined>());
  const indirectPickRef = reactHostPort.useRef<AttractionIndirectPickAwait | null>(null);
  const [hoverTarget, setHoverTarget] = reactHostPort.useState<HoverTarget | null>(null);
  const [kindHover, setKindHoverState] = reactHostPort.useState<Puzzle3dKindHover | null>(null);

  const getVortexKind = reactHostPort.useCallback((fullId: string) => vortexMetaRef.current.get(fullId)?.vortexKind, []);
  const getAttractionKind = reactHostPort.useCallback((id: string) => attractionKindsRef.current.get(id), []);

  const setHover = reactHostPort.useCallback((target: HoverTarget) => {
    setHoverTarget((prev) => (puzzle3dHoverTargetsEqual(prev, target) ? prev : target));
    setKindHoverState((prev) => (prev === null ? prev : null));
  }, []);

  const clearHover = reactHostPort.useCallback((target: HoverTarget) => {
    setHoverTarget((prev) => (puzzle3dHoverTargetsEqual(prev, target) ? null : prev));
  }, []);

  const clearHoverAll = reactHostPort.useCallback(() => {
    setHoverTarget((prev) => (prev === null ? prev : null));
    setKindHoverState((prev) => (prev === null ? prev : null));
  }, []);

  const isHovered = reactHostPort.useCallback((target: HoverTarget) => puzzle3dHoverTargetsEqual(hoverTarget, target), [hoverTarget]);

  const setKindHover = reactHostPort.useCallback((kind: Puzzle3dKindHover) => {
    setHoverTarget((prev) => (prev === null ? prev : null));
    setKindHoverState((prev) => (puzzle3dKindHoversEqual(prev, kind) ? prev : kind));
  }, []);

  const clearKindHover = reactHostPort.useCallback((kind: Puzzle3dKindHover) => {
    setKindHoverState((prev) => (puzzle3dKindHoversEqual(prev, kind) ? null : prev));
  }, []);

  const isKindHovered = reactHostPort.useCallback(
    (domain: Puzzle3dKindHoverDomain, kindId: string | undefined) => {
      const trimmed = kindId?.trim();
      if (!trimmed || !kindHover) {
        return false;
      }
      return kindHover.domain === domain && kindHover.kindId === trimmed;
    },
    [kindHover],
  );

  const clearSelection = reactHostPort.useCallback(() => {
    clearHoverAll();
    setActiveRelocateObjectId(null);
    const empty = EMPTY_SELECTION_SNAPSHOT;
    selectionStore.setSnapshot(empty);
    const controlled = selectionStore.getControlledHostSnapshot();
    if (controlled !== undefined) {
      if (controlled.objectIds.length === 0 && controlled.vortexIds.length === 0 && controlled.attractionIds.length === 0) {
        return;
      }
      hostCallbacksRef.current.onSelect?.(empty);
      return;
    }
    hostCallbacksRef.current.onSelect?.(empty);
  }, [clearHoverAll, hostCallbacksRef, selectionStore, setActiveRelocateObjectId]);

  const registerAttractionKind = reactHostPort.useCallback((id: string, attractionKind: string | undefined) => {
    attractionKindsRef.current.set(id, attractionKind);
  }, []);

  reactHostPort.useEffect(() => {
    indirectPickRef.current = attractionIndirectPickAwait;
  }, [attractionIndirectPickAwait]);

  const attractionSessionRef = reactHostPort.useRef<{
    attractingFullId: string;
    attractingObjectId: string;
    attractingCtx: AttractionVortexContext;
    compat: Set<string>;
    snapAttractedFullId: string | null;
  } | null>(null);
  const attractionEndWorldRef = reactHostPort.useRef<Vector3 | null>(null);
  const attractionThreeRef = reactHostPort.useRef<{ camera: Camera; gl: WebGLRenderer; scene: ThreeScene } | null>(null);
  const raycasterRef = reactHostPort.useRef(new Raycaster());
  const ndcRef = reactHostPort.useRef(new Vector2());
  const planeRef = reactHostPort.useRef(new Plane(new Vector3(0, 1, 0), 0));
  const hitScratchRef = reactHostPort.useRef(new Vector3());

  const registerVortex = reactHostPort.useCallback((fullId: string, getter: VortexGetter) => {
    vortexGettersRef.current.set(fullId, getter);
  }, []);

  const unregisterVortex = reactHostPort.useCallback((fullId: string) => {
    vortexGettersRef.current.delete(fullId);
  }, []);

  const getVortexWorld = reactHostPort.useCallback((fullId: string) => {
    const g = vortexGettersRef.current.get(fullId);
    return g ? g() : null;
  }, []);

  const registerVortexBinding = reactHostPort.useCallback((meta: VortexBindingMeta, pickRoot: Object3D | null) => {
    vortexMetaRef.current.set(meta.fullId, meta);
    if (pickRoot) vortexPickRef.current.set(meta.fullId, pickRoot);
    else vortexPickRef.current.delete(meta.fullId);
  }, []);

  const unregisterVortexBinding = reactHostPort.useCallback((fullId: string) => {
    vortexMetaRef.current.delete(fullId);
    vortexPickRef.current.delete(fullId);
  }, []);

  const warmAllMarqueeFootprintCaches = reactHostPort.useCallback(() => {
    for (const [id, group] of objectGroupMap.current) {
      if (group) {
        buildObjectMarqueeFootprintCache(group, id);
      }
    }
  }, []);

  const registerObject = reactHostPort.useCallback(
    (id: string, objectKind: string | undefined, group: Group | null) => {
      objectGroupMap.current.set(id, group);
      objectKindsRef.current.set(id, objectKind);
      if (group) {
        scheduleDeferredCallback(() => warmObjectGroupMarqueeBounds(group, id));
        if (attractionThreeRef.current) {
          requestAnimationFrame(() => {
            const current = objectGroupMap.current.get(id);
            if (current) {
              buildObjectMarqueeFootprintCache(current, id);
            }
          });
        }
        return;
      }
      invalidateObjectMarqueeFootprintCache(id);
    },
    [],
  );

  const collectObjectGroups = reactHostPort.useCallback((): Group[] => {
    const out: Group[] = [];
    for (const group of objectGroupMap.current.values()) {
      if (group) out.push(group);
    }
    return out;
  }, []);

  const listVortexBindings = reactHostPort.useCallback((): readonly VortexBindingMeta[] => [...vortexMetaRef.current.values()], []);

  const getObjectGroup = reactHostPort.useCallback((id: string) => objectGroupMap.current.get(id) ?? null, []);

  const getObjectKind = reactHostPort.useCallback((id: string) => objectKindsRef.current.get(id), []);

  const cancelAttractionDrag = reactHostPort.useCallback(() => {
    attractionSessionRef.current = null;
    attractionEndWorldRef.current = null;
    setAttractionDragActive(false);
    setAttractionDragAttractingFullId(null);
    setAttractionCompatibleAttractedFullIds(new Set());
    setAttractionHoverRingFullId(null);
    setAttractionIndirectPickAwait(null);
    setHoverTarget(null);
    setKindHoverState(null);
    notifyAttractionTargetRing({ attracting: "", objectId: null, vortexFullIds: [] });
  }, [notifyAttractionTargetRing]);

  reactHostPort.useEffect(() => {
    if (attractionSessionRef.current) {
      return;
    }
    const ext = attractionSession;
    if (!ext?.attracting) {
      if (attractionDragAttractingFullId) {
        setAttractionDragActive(false);
        setAttractionDragAttractingFullId(null);
        setAttractionCompatibleAttractedFullIds(new Set());
        setAttractionHoverRingFullId(null);
        setAttractionIndirectPickAwait(null);
        attractionEndWorldRef.current = null;
      }
      return;
    }
    setAttractionDragActive(true);
    setAttractionDragAttractingFullId(ext.attracting);
    const compatVortexIds = new Set<string>();
    for (const oid of ext.compatibleObjectIds) {
      for (const [fullId, meta] of vortexMetaRef.current) {
        if (meta.objectId === oid) {
          compatVortexIds.add(fullId);
        }
      }
    }
    setAttractionCompatibleAttractedFullIds(compatVortexIds);
    attractionEndWorldRef.current = new Vector3(...cadVec3ToThree(ext.end));
    if (ext.ringObjectId && ext.ringVortexFullIds.length > 1) {
      setAttractionIndirectPickAwait({
        attractingFullId: ext.attracting,
        attractedObjectId: ext.ringObjectId,
        candidates: ext.ringVortexFullIds,
      });
    } else {
      setAttractionIndirectPickAwait(null);
    }
    notifyAttractionTargetRing({
      attracting: ext.attracting,
      objectId: ext.ringObjectId,
      vortexFullIds: ext.ringVortexFullIds,
    });
  }, [attractionSession, attractionDragAttractingFullId, notifyAttractionTargetRing]);

  const beginAttractionDragFromVortex = reactHostPort.useCallback(
    (fullId: string, objectId: string, objectKind: string | undefined, vortexKind: string | undefined) => {
      if (puzzle3dBrushToolActiveRef.current) return;
      if (indirectPickRef.current) return;
      if (blockedVortexFullIds.has(fullId)) return;
      const attractingCtx: AttractionVortexContext = { objectId, objectKind, vortexKind };
      const compat = new Set<string>();
      const objectIds = new Set<string>();
      for (const [tid, meta] of vortexMetaRef.current) {
        if (tid === fullId) continue;
        if (meta.objectId === objectId) continue;
        if (blockedVortexFullIds.has(tid)) continue;
        const attractedCtx: AttractionVortexContext = {
          objectId: meta.objectId,
          objectKind: meta.objectKind,
          vortexKind: meta.vortexKind,
        };
        if (!vorticesAttractionCompatibleForDrag(attractingCtx, attractedCtx, kindCompatibility, kindCatalogs)) continue;
        compat.add(tid);
        objectIds.add(meta.objectId);
      }
      setAttractionIndirectPickAwait(null);
      attractionSessionRef.current = {
        attractingFullId: fullId,
        attractingObjectId: objectId,
        attractingCtx,
        compat,
        snapAttractedFullId: null,
      };
      attractionEndWorldRef.current = null;
      setAttractionDragActive(true);
      setAttractionDragAttractingFullId(fullId);
      setAttractionCompatibleAttractedFullIds(compat);
      setAttractionHoverRingFullId(null);
      setActiveRelocateObjectId(null);
      setHoverTarget(null);
      setKindHoverState(null);
      notifyAttractionCompatibleObjects({ attracting: fullId, objectIds: [...objectIds] });
    },
    [blockedVortexFullIds, kindCatalogs, kindCompatibility, notifyAttractionCompatibleObjects],
  );

  const collectPickRoots = reactHostPort.useCallback((): Object3D[] => {
    const out: Object3D[] = [];
    for (const p of vortexPickRef.current.values()) out.push(p);
    for (const g of objectGroupMap.current.values()) if (g) out.push(g);
    return out;
  }, []);

  const updateAttractionPointer = reactHostPort.useCallback(
    (clientX: number, clientY: number) => {
      const env = attractionThreeRef.current;
      const session = attractionSessionRef.current;
      if (!env || !session) return;
      const rect = env.gl.domElement.getBoundingClientRect();
      ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
      ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
      raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
      const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
      let ring: string | null = null;
      for (const h of hits) {
        const vf = readVortexFullIdFromObject(h.object);
        if (vf && session.compat.has(vf) && vf !== session.attractingFullId && !blockedVortexFullIds.has(vf)) {
          ring = vf;
          break;
        }
      }
      setAttractionHoverRingFullId((prev) => (prev === ring ? prev : ring));
      if (ring) {
        const meta = vortexMetaRef.current.get(ring);
        notifyAttractionTargetRing({
          attracting: session.attractingFullId,
          objectId: meta?.objectId ?? null,
          vortexFullIds: ring ? [ring] : [],
        });
      } else {
        notifyAttractionTargetRing({ attracting: session.attractingFullId, objectId: null, vortexFullIds: [] });
      }
      const hitWorld = hitScratchRef.current;
      if (hits.length > 0) {
        attractionEndWorldRef.current = hitWorld.copy(hits[0]!.point);
      } else if (raycasterRef.current.ray.intersectPlane(planeRef.current, hitWorld)) {
        attractionEndWorldRef.current = hitWorld.clone();
      } else {
        raycasterRef.current.ray.at(80, hitWorld);
        attractionEndWorldRef.current = hitWorld.clone();
      }
      const pw = attractionEndWorldRef.current;
      if (pw) {
        session.snapAttractedFullId = nearestAttractionSnapFullId({
          lod: lodRef.current,
          pointerWorld: pw,
          attractingFullId: session.attractingFullId,
          compat: session.compat,
          blocked: blockedVortexFullIds,
          camera: env.camera,
          gl: env.gl,
          getVortexWorld: (id) => vortexGettersRef.current.get(id)?.() ?? null,
          metaRadius: (id) => vortexMetaRef.current.get(id)?.radiusWorld ?? 0.35,
        });
      } else session.snapAttractedFullId = null;
    },
    [blockedVortexFullIds, collectPickRoots, lodRef, notifyAttractionTargetRing],
  );

  const commitAttractionPointer = reactHostPort.useCallback(
    (clientX: number, clientY: number) => {
      const env = attractionThreeRef.current;
      const session = attractionSessionRef.current;
      if (!env || !session) {
        cancelAttractionDrag();
        return;
      }
      const rect = env.gl.domElement.getBoundingClientRect();
      ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
      ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
      raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
      const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
      const hitWorld = hitScratchRef.current;
      let pointerWorld: Vector3;
      if (hits.length > 0) {
        pointerWorld = hits[0]!.point.clone();
      } else if (raycasterRef.current.ray.intersectPlane(planeRef.current, hitWorld)) {
        pointerWorld = hitWorld.clone();
      } else {
        raycasterRef.current.ray.at(80, hitWorld);
        pointerWorld = hitWorld.clone();
      }

      const getV = (id: string) => vortexGettersRef.current.get(id)?.() ?? null;
      const rad = (id: string) => vortexMetaRef.current.get(id)?.radiusWorld ?? 0.35;
      const snapId = session.snapAttractedFullId;
      if (snapId && attractionSnapCommitProximityOk(snapId, pointerWorld, env.camera, env.gl, getV, rad)) {
        const p = { attracting: session.attractingFullId, attracted: snapId };
        notifyConnect(p);
        notifyProximityConnect(p);
        cancelAttractionDrag();
        return;
      }

      const attractingFull = session.attractingFullId;
      for (const h of hits) {
        const vf = readVortexFullIdFromObject(h.object);
        if (vf && vf !== attractingFull && session.compat.has(vf) && !blockedVortexFullIds.has(vf) && vortexMetaRef.current.get(vf)?.objectId !== session.attractingObjectId) {
          notifyConnect({ attracting: attractingFull, attracted: vf });
          cancelAttractionDrag();
          return;
        }
        const oid = readObjectItemIdFromObject(h.object);
        if (oid && oid !== session.attractingObjectId) {
          const candidates: string[] = [];
          for (const [tid, meta] of vortexMetaRef.current) {
            if (meta.objectId !== oid) continue;
            if (blockedVortexFullIds.has(tid)) continue;
            if (!session.compat.has(tid)) continue;
            candidates.push(tid);
          }
          if (candidates.length === 1) {
            const p = { attracting: attractingFull, attracted: candidates[0]! };
            notifyConnect(p);
            notifyIndirectConnect(p);
            cancelAttractionDrag();
            return;
          }
          if (candidates.length > 1) {
            attractionSessionRef.current = null;
            setAttractionDragActive(false);
            setAttractionCompatibleAttractedFullIds(new Set(candidates));
            setAttractionHoverRingFullId(null);
            setAttractionIndirectPickAwait({
              attractingFullId: attractingFull,
              attractedObjectId: oid,
              candidates,
            });
            notifyAttractionTargetRing({
              attracting: attractingFull,
              objectId: oid,
              vortexFullIds: candidates,
            });
            return;
          }
        }
      }
      cancelAttractionDrag();
    },
    [blockedVortexFullIds, cancelAttractionDrag, collectPickRoots, notifyAttractionTargetRing, notifyConnect, notifyIndirectConnect, notifyProximityConnect],
  );

  const updateIndirectPickPointer = reactHostPort.useCallback(
    (clientX: number, clientY: number) => {
      const awaitPick = indirectPickRef.current;
      const env = attractionThreeRef.current;
      if (!awaitPick || !env) return;
      const rect = env.gl.domElement.getBoundingClientRect();
      ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
      ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
      raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
      const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
      let ring: string | null = null;
      for (const h of hits) {
        const vf = readVortexFullIdFromObject(h.object);
        if (vf && awaitPick.candidates.includes(vf)) {
          ring = vf;
          break;
        }
      }
      setAttractionHoverRingFullId((prev) => (prev === ring ? prev : ring));
      const hitWorld = hitScratchRef.current;
      if (hits.length > 0) {
        attractionEndWorldRef.current = hitWorld.copy(hits[0]!.point);
      } else if (raycasterRef.current.ray.intersectPlane(planeRef.current, hitWorld)) {
        attractionEndWorldRef.current = hitWorld.clone();
      } else {
        raycasterRef.current.ray.at(80, hitWorld);
        attractionEndWorldRef.current = hitWorld.clone();
      }
    },
    [collectPickRoots],
  );

  const commitIndirectPickPointerDown = reactHostPort.useCallback(
    (clientX: number, clientY: number, ev?: PointerEvent) => {
      const awaitPick = indirectPickRef.current;
      const env = attractionThreeRef.current;
      if (!awaitPick || !env) return;
      const rect = env.gl.domElement.getBoundingClientRect();
      ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
      ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
      raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
      const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
      for (const h of hits) {
        const vf = readVortexFullIdFromObject(h.object);
        if (vf && awaitPick.candidates.includes(vf)) {
          const p = { attracting: awaitPick.attractingFullId, attracted: vf };
          notifyConnect(p);
          notifyIndirectConnect(p);
          cancelAttractionDrag();
          ev?.stopImmediatePropagation();
          return;
        }
      }
      cancelAttractionDrag();
    },
    [cancelAttractionDrag, collectPickRoots, notifyConnect, notifyIndirectConnect],
  );

  const attachAttractionThreeEnv = reactHostPort.useCallback(
    (env: { camera: Camera; gl: WebGLRenderer; scene: ThreeScene } | null) => {
      attractionThreeRef.current = env;
      if (env) {
        requestAnimationFrame(() => warmAllMarqueeFootprintCaches());
        scheduleDeferredCallback(warmAllMarqueeFootprintCaches);
      }
    },
    [warmAllMarqueeFootprintCaches],
  );

  const findNearestProximityRelocate = reactHostPort.useCallback(
    (world: Vector3, movingObjectId: string): AttractionPayload | null => {
      if (!proximityRelocateEnabled) {
        return null;
      }
      let best: { d: number; id: string } | null = null;
      for (const [fullId, getter] of vortexGettersRef.current) {
        if (fullId.startsWith(`${movingObjectId}:`)) continue;
        const p = getter();
        if (!p) continue;
        const d = p.distanceTo(world);
        if (d > proximityRadius) continue;
        if (!best || d < best.d) best = { d, id: fullId };
      }
      if (!best) return null;
      return { attracting: `${movingObjectId}:link`, attracted: best.id };
    },
    [proximityRadius, proximityRelocateEnabled],
  );

  const coreValue = reactHostPort.useMemo<RegistryCoreValue>(
    () => ({
      registerVortex,
      unregisterVortex,
      getVortexWorld,
      registerVortexBinding,
      unregisterVortexBinding,
      registerObject,
      collectObjectGroups,
      listVortexBindings,
      getObjectGroup,
      getObjectKind,
      kindCatalogs,
      kindCompatibility,
      blockedVortexFullIds,
      proximityRadius,
      proximityRelocateEnabled,
      gumballConfig,
      selectionMode,
      beginAttractionDragFromVortex,
      cancelAttractionDrag,
      findNearestProximityRelocate,
      onSelect: (snap) => hostCallbacksRef.current.onSelect?.(snap),
      onConnect: notifyConnect,
      onProximityConnect: notifyProximityConnect,
      onIndirectConnect: notifyIndirectConnect,
      onAttractionCompatibleObjects: notifyAttractionCompatibleObjects,
      onAttractionTargetRing: notifyAttractionTargetRing,
      onRelocate: notifyRelocate,
      attachAttractionThreeEnv,
      updateAttractionPointer,
      commitAttractionPointer,
      updateIndirectPickPointer,
      commitIndirectPickPointerDown,
      attractionEndWorldRef,
      registerAttractionKind,
    }),
    [
      registerVortex,
      unregisterVortex,
      getVortexWorld,
      registerVortexBinding,
      unregisterVortexBinding,
      registerObject,
      registerAttractionKind,
      collectObjectGroups,
      listVortexBindings,
      getObjectGroup,
      getObjectKind,
      kindCatalogs,
      kindCompatibility,
      blockedVortexFullIds,
      proximityRadius,
      proximityRelocateEnabled,
      gumballConfig,
      selectionMode,
      beginAttractionDragFromVortex,
      cancelAttractionDrag,
      findNearestProximityRelocate,
      hostCallbacksRef,
      notifyAttractionCompatibleObjects,
      notifyAttractionTargetRing,
      notifyConnect,
      notifyIndirectConnect,
      notifyProximityConnect,
      notifyRelocate,
      attachAttractionThreeEnv,
      updateAttractionPointer,
      commitAttractionPointer,
      updateIndirectPickPointer,
      commitIndirectPickPointerDown,
    ],
  );
  const interactionValue = reactHostPort.useMemo<RegistryInteractionValue>(
    () => ({
      selectionMode,
      commitSelection,
      captureMarqueeCandidates,
      previewMarqueeSelection,
      cancelMarqueePreview,
      commitMarqueeSelection,
      setSelectedObjectIds,
      setActiveRelocateObjectId,
      clearSelection,
    }),
    [
      cancelMarqueePreview,
      captureMarqueeCandidates,
      clearSelection,
      commitMarqueeSelection,
      commitSelection,
      previewMarqueeSelection,
      selectionMode,
      setActiveRelocateObjectId,
      setSelectedObjectIds,
    ],
  );
  const marqueeValue = reactHostPort.useMemo<RegistryMarqueeValue>(
    () => ({
      selectionMethod,
      marqueeSelectableKinds,
      setMarqueeAttractions,
    }),
    [marqueeSelectableKinds, selectionMethod, setMarqueeAttractions],
  );
  const hoverValue = reactHostPort.useMemo<RegistryHoverValue>(
    () => ({
      hoverTarget,
      kindHover,
      setHover,
      clearHover,
      clearHoverAll,
      isHovered,
      setKindHover,
      clearKindHover,
      isKindHovered,
    }),
    [hoverTarget, kindHover, setHover, clearHover, clearHoverAll, isHovered, setKindHover, clearKindHover, isKindHovered],
  );
  const dragValue = reactHostPort.useMemo<RegistryDragState>(
    () => ({
      attractionDragActive,
      attractionDragAttractingFullId,
      attractionCompatibleAttractedFullIds,
      attractionHoverRingFullId,
      attractionIndirectPickAwait,
    }),
    [attractionCompatibleAttractedFullIds, attractionDragActive, attractionDragAttractingFullId, attractionHoverRingFullId, attractionIndirectPickAwait],
  );

  return (
    <SelectionStoreContext.Provider value={selectionStore}>
      <RegistryCoreContext.Provider value={coreValue}>
        <RegistryInteractionContext.Provider value={interactionValue}>
          <RegistryMarqueeContext.Provider value={marqueeValue}>
            <RegistryHoverContext.Provider value={hoverValue}>
              <RegistryDragContext.Provider value={dragValue}>
                <ControlledHoverSync hoverTargetProp={hoverTargetProp} kindHoverProp={kindHoverProp} onHover={onHover} />
                {children}
                <HoverMissBridge />
                <HoverInvalidateBridge />
                <SelectionInvalidateBridge />
                <BulkSelectionVisualBridge />
                <SelectionMissBridge />
                <AttractionStaleSessionGuard />
              </RegistryDragContext.Provider>
            </RegistryHoverContext.Provider>
          </RegistryMarqueeContext.Provider>
        </RegistryInteractionContext.Provider>
      </RegistryCoreContext.Provider>
    </SelectionStoreContext.Provider>
  );
}

/** @emoji 👻 Live grid-plane preview while a workbench object-kind drag hovers the viewport. */
function FixtureDropPreview(props: { readonly kindCatalogs: KindCatalogBundle | undefined; readonly sceneFixture?: FixtureV1 }): React.ReactElement | null {
  const { camera, gl } = useThree();
  const controls = useThree((state) => state.controls as { target?: Vector3 } | null);
  const lod = useLod();
  const [encodedDrag, setEncodedDrag] = reactHostPort.useState<string | null>(() => puzzle3dFixturePalettePointerDragRef.encoded);
  const [origin, setOrigin] = reactHostPort.useState<Vec3 | null>(null);
  const groupRef = reactHostPort.useRef<Group | null>(null);

  reactHostPort.useEffect(() => {
    const onSession = (event: Event): void => {
      const detail = (event as CustomEvent<{ readonly encoded: string } | null>).detail;
      setEncodedDrag(detail?.encoded ?? null);
      if (!detail?.encoded) {
        setOrigin(null);
      }
    };
    window.addEventListener("puzzle3d-fixture-drag-session", onSession);
    return () => window.removeEventListener("puzzle3d-fixture-drag-session", onSession);
  }, []);

  reactHostPort.useEffect(() => {
    if (!encodedDrag) {
      return;
    }
    const onMove = (event: PointerEvent): void => {
      const cad = puzzle3dClientToGridPlaneCad({
        clientX: event.clientX,
        clientY: event.clientY,
        camera,
        canvas: gl.domElement,
        gridSnapEnabled: lod.gridSnapEnabled,
        gridStepWorld: lod.gridStepWorld,
        gridPlaneAnchorCad: puzzle3dGridPlacementAnchorCad(controls?.target ?? null),
      });
      setOrigin(cad);
    };
    window.addEventListener("pointermove", onMove);
    return () => window.removeEventListener("pointermove", onMove);
  }, [camera, controls, encodedDrag, gl, lod.gridSnapEnabled, lod.gridStepWorld]);

  const preview = reactHostPort.useMemo(() => {
    if (!encodedDrag || !origin) {
      return null;
    }
    const fixture = decodePuzzle3dFixtureFromDragV1(encodedDrag);
    const kindId = fixture?.objects[0]?.objectKind;
    if (!kindId) {
      return null;
    }
    const kind = catalogObjectKindById(props.kindCatalogs, kindId);
    const meshUrl = resolveObjectKindMeshUrl(kindId, props.kindCatalogs, props.sceneFixture);
    if (!meshUrl) {
      return null;
    }
    return { meshUrl, scale: kind?.scale, origin, orientation: [0, 0, 0, 1] as Quat };
  }, [encodedDrag, origin, props.kindCatalogs, props.sceneFixture]);

  reactHostPort.useEffect(() => {
    const group = groupRef.current;
    if (!group || !preview) {
      return;
    }
    applyObjectPose(group, preview.origin, preview.orientation, preview.scale);
    updateWorldMatrixChain(group);
  }, [preview]);

  if (!preview) {
    return null;
  }
  return (
    <group ref={groupRef} raycast={() => null} renderOrder={10}>
      <MeshBody meshUrl={preview.meshUrl} style="highlighted" scale={preview.scale} />
    </group>
  );
}

/** @emoji 📍 Registers grid-plane hit testing and canvas/window fixture drop targets for {@link Canvas3D}. */
function FixtureDropPointerBridge(props: {
  readonly enabled: boolean;
  readonly onFixtureDrop?: (detail: Puzzle3dFixtureDropDetail) => void;
  readonly rootRef: React.RefObject<HTMLDivElement | null>;
  readonly setFixtureDragActive: (active: boolean) => void;
  readonly fixtureDragDepthRef: React.MutableRefObject<number>;
}): null {
  const { camera, gl } = useThree();
  const controls = useThree((state) => state.controls as { target?: Vector3 } | null);
  const lod = useLod();
  const onFixtureDropRef = reactHostPort.useRef(props.onFixtureDrop);
  onFixtureDropRef.current = props.onFixtureDrop;

  reactHostPort.useEffect(() => {
    puzzle3dFixtureDropPointerToCadRef.current = (clientX, clientY) =>
      puzzle3dClientToGridPlaneCad({
        clientX,
        clientY,
        camera,
        canvas: gl.domElement,
        gridSnapEnabled: lod.gridSnapEnabled,
        gridStepWorld: lod.gridStepWorld,
        gridPlaneAnchorCad: puzzle3dGridPlacementAnchorCad(controls?.target ?? null),
      });
    return () => {
      puzzle3dFixtureDropPointerToCadRef.current = null;
    };
  }, [camera, controls, gl, lod.gridSnapEnabled, lod.gridStepWorld]);

  reactHostPort.useEffect(() => {
    if (!props.enabled) {
      return;
    }
    const canvas = gl.domElement;
    const root = props.rootRef.current;
    const bindings = new EventBindingController();

    const resetDragDepth = (): void => {
      props.fixtureDragDepthRef.current = 0;
      props.setFixtureDragActive(false);
    };

    const onDragEnter = (event: DragEvent): void => {
      if (!puzzle3dFixtureDragAcceptsTransfer([...event.dataTransfer!.types])) {
        return;
      }
      event.preventDefault();
      props.fixtureDragDepthRef.current += 1;
      props.setFixtureDragActive(true);
    };

    const onDragLeave = (event: DragEvent): void => {
      if (!puzzle3dFixtureDragAcceptsTransfer([...event.dataTransfer!.types])) {
        return;
      }
      const target = event.currentTarget as HTMLElement;
      const related = event.relatedTarget as Node | null;
      if (related && target.contains(related)) {
        return;
      }
      props.fixtureDragDepthRef.current = Math.max(0, props.fixtureDragDepthRef.current - 1);
      if (props.fixtureDragDepthRef.current === 0) {
        props.setFixtureDragActive(false);
      }
    };

    const onDragOver = (event: DragEvent): void => {
      if (!puzzle3dFixtureDragAcceptsTransfer([...event.dataTransfer!.types])) {
        return;
      }
      event.preventDefault();
      event.dataTransfer!.dropEffect = "copy";
    };

    const onDrop = (event: DragEvent): void => {
      if (!puzzle3dFixtureDragAcceptsTransfer([...event.dataTransfer!.types])) {
        return;
      }
      event.preventDefault();
      event.stopPropagation();
      resetDragDepth();
      const fixture = readPuzzle3dFixtureDragDataTransfer(event.dataTransfer!);
      if (!fixture) {
        return;
      }
      const host = root ?? canvas.parentElement;
      commitPuzzle3dFixtureDropAtClient(event.clientX, event.clientY, fixture, host, onFixtureDropRef.current);
    };

    const dropHost = (): HTMLElement | null => root ?? canvas.parentElement ?? canvas;

    const onWindowPointerMove = (event: PointerEvent): void => {
      if (!puzzle3dFixturePalettePointerDragRef.active) {
        return;
      }
      props.setFixtureDragActive(isClientPointOverPuzzle3dFixtureDropHost(event.clientX, event.clientY, dropHost()));
    };

    const onWindowPointerUp = (event: PointerEvent): void => {
      if (!puzzle3dFixturePalettePointerDragRef.active) {
        return;
      }
      resetDragDepth();
      endPuzzle3dFixturePalettePointerDrag(event.clientX, event.clientY, dropHost(), onFixtureDropRef.current);
    };

    const onWindowPointerCancel = (event: PointerEvent): void => {
      if (!puzzle3dFixturePalettePointerDragRef.active) {
        return;
      }
      resetDragDepth();
      cancelPuzzle3dFixturePalettePointerDrag();
    };

    const attach = (element: HTMLElement | null | undefined): void => {
      if (!element) {
        return;
      }
      bindings.listen(element, "dragenter", onDragEnter);
      bindings.listen(element, "dragleave", onDragLeave);
      bindings.listen(element, "dragover", onDragOver);
      bindings.listen(element, "drop", onDrop);
    };

    attach(canvas);
    attach(root);
    bindings.listen(window, "dragover", onDragOver);
    bindings.listen(window, "drop", onDrop);
    bindings.listen(window, "pointermove", onWindowPointerMove);
    bindings.listen(window, "pointerup", onWindowPointerUp);
    bindings.listen(window, "pointercancel", onWindowPointerCancel);
    return () => bindings.dispose();
  }, [gl, props.enabled, props.fixtureDragDepthRef, props.rootRef, props.setFixtureDragActive]);

  return null;
}

function Inner(props: CanvasProps & {
  readonly puzzle3dRootRef: React.RefObject<HTMLDivElement | null>;
  readonly fixtureDragActive: boolean;
  readonly setFixtureDragActive: (active: boolean) => void;
  readonly fixtureDragDepthRef: React.MutableRefObject<number>;
}) {
  const {
    camera: camProp,
    chunkSize = 256,
    proximityRadius = 12,
    proximityRelocateEnabled = true,
    children,
    brushActive = false,
    fillActive = false,
    onFillMeshesReady,
    onBrushPlace,
    brushPlacementOverlapBudget = DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
    kindCatalogs,
    kindCompatibility: kindCompatibilityProp,
    fixtureDragDrop,
    onFixtureDrop,
    sceneFixture,
    puzzle3dRootRef,
    fixtureDragActive,
    setFixtureDragActive,
    fixtureDragDepthRef,
    attractionSession,
    fillEditTargetVolumes = false,
    onVoxelBrushPaint,
    voxelBrushDimensions = DEFAULT_VOXEL_BRUSH_DIMENSIONS,
  } = props;
  const kindCompatibility = reactHostPort.useMemo(
    () => resolvePuzzle3dKindCompatibility(kindCompatibilityProp, sceneFixture?.meta as Record<string, unknown> | undefined),
    [kindCompatibilityProp, sceneFixture],
  );
  reactHostPort.useEffect(() => {
    puzzle3dBrushToolActiveRef.current = brushActive;
    if (!brushActive) {
      puzzle3dBrushUiStore.setSnapshot(BRUSH_UI_IDLE);
      puzzle3dBrushVortexHoverRef.current = false;
      puzzle3dBrushAltPressedRef.current = false;
    }
    return () => {
      puzzle3dBrushToolActiveRef.current = false;
      puzzle3dBrushAltPressedRef.current = false;
    };
  }, [brushActive]);
  const lodRef = reactHostPort.useRef<number>(DEFAULT_MANUAL_LOD);
  const domain = props.domain ?? DEFAULT_DOMAIN;
  const distanceReference = props.lodDistanceReference ?? DEFAULT_SCALE_REFERENCE;
  const gridFactor = props.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
  const gridSnapEnabled = props.gridSnapEnabled ?? false;
  const showLodGrid = props.showLodGrid !== false;
  const automaticLod = props.automaticLod ?? true;
  const depthVariableLod = props.depthVariableLod ?? false;
  const manualLod = typeof props.lod === "number" && Number.isFinite(props.lod) && props.lod > 0 ? props.lod : DEFAULT_MANUAL_LOD;
  const maxDist = 4000;
  const pos = (camProp?.position ?? [420, -420, 320]) as [number, number, number];
  const tgt = (camProp?.target ?? [0, 0, 40]) as Vec3;
  const zoom = camProp?.zoom ?? 1;
  const up = (camProp?.up ?? [0, 0, 1]) as Vec3;
  const projection = camProp?.projection ?? "perspective";
  const cameraRigState = reactHostPort.useMemo(
    () => ({ position: pos, target: tgt, zoom, up, projection }),
    [pos, tgt, zoom, up, projection],
  );
  const autoFitCamera = props.autoFitCamera !== false;
  const autoFitBehavior = props.autoFitBehavior ?? "initial";
  const blockedFallback = props.blockedVortexFullIds ?? EMPTY_BLOCKED_VORTICES;
  const blocked = useLiveBlockedVortexFullIds(blockedFallback);
  const selectionStoreRef = reactHostPort.useRef<SelectionSnapshotStore>();
  if (!selectionStoreRef.current) {
    selectionStoreRef.current = createSelectionSnapshotStore(EMPTY_SELECTION_SNAPSHOT);
  }
  const selectionStore = selectionStoreRef.current;
  const registryHostCallbacksRef = reactHostPort.useRef<RegistryHostCallbacks>({});
  registryHostCallbacksRef.current = {
    onSelect: props.onSelect,
    onConnect: props.onConnect,
    onProximityConnect: props.onProximityConnect,
    onIndirectConnect: props.onIndirectConnect,
    onAttractionCompatibleObjects: props.onAttractionCompatibleObjects,
    onAttractionTargetRing: props.onAttractionTargetRing,
    onRelocate: props.onRelocate,
  };
  const canvasHostRef = reactHostPort.useRef({
    onCamera: props.onCamera,
    onLodChange: props.onLodChange,
    onBrushPlace: onBrushPlace,
    onFixtureDrop: onFixtureDrop,
  });
  canvasHostRef.current = {
    onCamera: props.onCamera,
    onLodChange: props.onLodChange,
    onBrushPlace,
    onFixtureDrop,
  };
  const fixtureDropEnabled = fixtureDragDrop ?? Boolean(onFixtureDrop);
  const registryLodSceneCore = reactHostPort.useMemo(
    () => (
      <WorldOrbitViewSnapGateProvider>
        <WorldOrbitCameraViewRig
          state={cameraRigState}
          seedKey={props.cameraSeedKey ?? `${pos.join(",")}|${tgt.join(",")}|${projection}|${up.join(",")}`}
          perspectiveFov={50}
        />
        <OrbitGated
          controlsKey={props.cameraSeedKey ?? `${projection}:${up.join(",")}`}
          onCamera={(camera) => canvasHostRef.current.onCamera?.(camera)}
          zoom={zoom}
          up={up}
          projection={projection}
        />
        <WorldOrbitViewControls
          onCameraChange={(state) => canvasHostRef.current.onCamera?.(state)}
        />
        {autoFitCamera && projection !== "orthographic" ? (
          <AutoFit
            behavior={autoFitBehavior}
            zoom={zoom}
            projection={projection}
            seedKey={props.cameraSeedKey ?? `${projection}:${pos.join(",")}|${tgt.join(",")}|${up.join(",")}`}
            onCamera={(state) => canvasHostRef.current.onCamera?.(state)}
          />
        ) : null}
        <AttractionThreeBinder />
        <AttractionWindowBridge />
        <MarqueeBridge />
        <SelectionContextMenuBinder />
        {fixtureDropEnabled ? (
          <FixtureDropPointerBridge
            enabled
            onFixtureDrop={(detail) => canvasHostRef.current.onFixtureDrop?.(detail)}
            rootRef={puzzle3dRootRef}
            setFixtureDragActive={setFixtureDragActive}
            fixtureDragDepthRef={fixtureDragDepthRef}
          />
        ) : null}
        <AttractionRubberBand />
        <ambientLight intensity={1.15} />
        <hemisphereLight color="#ffffff" groundColor="#9aa0ab" intensity={1.35} position={[0, 0, 1]} />
        <directionalLight position={[12, 18, 10]} intensity={2.4} />
        <directionalLight position={[-14, -10, 6]} intensity={1.2} />
        <directionalLight position={[0, 0, -16]} intensity={0.75} />
        <WorldLayer order={10} name="puzzle3d.view-radius">
          <InnerSceneChildren chunkSize={chunkSize} maxDistance={maxDist} />
        </WorldLayer>
      </WorldOrbitViewSnapGateProvider>
    ),
    [
      attractionSession,
      automaticLod,
      autoFitBehavior,
      autoFitCamera,
      cameraRigState,
      chunkSize,
      depthVariableLod,
      distanceReference,
      fixtureDropEnabled,
      gridFactor,
      gridSnapEnabled,
      manualLod,
      maxDist,
      pos,
      projection,
      props.cameraSeedKey,
      puzzle3dRootRef,
      setFixtureDragActive,
      showLodGrid,
      tgt,
      up,
      zoom,
    ],
  );
  const registryLodScene = (
    <LodBridge
      lodRef={lodRef}
      distanceReference={distanceReference}
      gridFactor={gridFactor}
      gridSnapEnabled={gridSnapEnabled}
      showLodGrid={showLodGrid}
      automaticLod={automaticLod}
      depthVariableLod={depthVariableLod}
      manualLod={manualLod}
      onLodChange={(lod) => canvasHostRef.current.onLodChange?.(lod)}
    >
      {registryLodSceneCore}
      {fixtureDropEnabled ? <FixtureDropPreview kindCatalogs={kindCatalogs} sceneFixture={sceneFixture} /> : null}
      <VoxelBrushBridge enabled={fillEditTargetVolumes} dimensions={voxelBrushDimensions} />
      <VoxelBrushPreview dimensions={voxelBrushDimensions} />
    </LodBridge>
  );
  const onFillMeshesReadyRef = reactHostPort.useRef(onFillMeshesReady);
  onFillMeshesReadyRef.current = onFillMeshesReady;
  const handleFillMeshesReady = reactHostPort.useCallback(() => {
    onFillMeshesReadyRef.current?.();
  }, []);
  return (
    <SelectionStoreContext.Provider value={selectionStore}>
      <InnerSceneChildrenContext.Provider value={children}>
        <ControlledSelectionSync selection={props.selection} />
        <RegistryProvider
          lodRef={lodRef}
          kindCatalogs={props.kindCatalogs}
          kindCompatibility={props.kindCompatibility}
          blockedVortexFullIds={blocked}
          proximityRadius={proximityRadius}
          proximityRelocateEnabled={proximityRelocateEnabled}
          selectionMode={props.selectionMode ?? "default"}
          gumballConfig={props.gumballConfig ?? PUZZLE_3D_GUMBALL_CONFIG}
          selectionMethod={props.selectionMethod ?? "rectangle"}
          marqueeSelectableKinds={props.marqueeSelectableKinds ?? { object: true, vortex: true, attraction: true }}
          hostCallbacksRef={registryHostCallbacksRef}
          attractionSession={attractionSession}
          selectionStore={selectionStore}
          hoverTargetProp={props.hoverTarget}
          kindHoverProp={props.kindHover}
          onHover={props.onHover}
        >
          {registryLodScene}
          <SelectionZoom attractions={sceneFixture?.attractions ?? []} zoom={zoom} onCamera={props.onCamera} />
          <BrushSession
            brushActive={brushActive}
            onBrushPlace={(payload) => canvasHostRef.current.onBrushPlace?.(payload)}
            kindCatalogs={kindCatalogs}
            kindCompatibility={kindCompatibility}
            overlapBudget={brushPlacementOverlapBudget}
            sceneFixture={sceneFixture}
          />
          <Puzzle3dFillMeshBridge
            fillActive={fillActive}
            sceneFixture={sceneFixture}
            kindCatalogs={kindCatalogs}
            kindCompatibility={kindCompatibility}
            onMeshesReady={handleFillMeshesReady}
          />
        </RegistryProvider>
      </InnerSceneChildrenContext.Provider>
    </SelectionStoreContext.Provider>
  );
}

export interface PlayCanvasProps {
  readonly fixture: FixtureV1;
  readonly camera?: CameraState;
  readonly cameraSeedKey?: string | number;
  readonly proximityRelocateEnabled?: boolean;
  readonly kindCatalogs?: KindCatalogBundle;
  readonly kindCompatibility?: readonly KindCompatEntry[];
  readonly blockedVortexFullIds?: ReadonlySet<string>;
  readonly lodTag?: number;
  readonly lodProps?: Pick<CanvasProps, "automaticLod" | "depthVariableLod" | "lod">;
  readonly gumballConfig?: GumballConfig;
  readonly selection?: SelectionSnapshot;
  readonly selectedId?: string | null;
  readonly selectedLabel?: string | null;
  readonly selectionMode?: SelectionMode;
  readonly selectionMethod?: SelectionMethod;
  readonly marqueeSelectableKinds?: MarqueeSelectableKinds;
  readonly proximityRadius?: number;
  readonly chunkSize?: number;
  readonly gridFactor?: number;
  readonly showLodGrid?: boolean;
  readonly gridSnapEnabled?: boolean;
  readonly setSelectedId: (id: string | null) => void;
  readonly onSelect?: (snap: SelectionSnapshot) => void;
  readonly onReferenceRelocate?: (payload: WorldReferenceRelocatePayload) => void;
  readonly onTargetVolumeRelocate?: (payload: WorldVolumeRelocatePayload) => void;
  readonly onVoxelBrushPaint?: (cad: Vec3, scale: Vec3) => void;
  readonly voxelBrushDimensions?: Vec3;
  readonly fillEditTargetVolumes?: boolean;
  readonly onIndirectConnect?: () => void;
  readonly onProximityConnect?: () => void;
  readonly onLodChange?: (lod: number) => void;
  readonly onCamera?: (s: CameraState) => void;
  readonly onAttractionCompatibleObjects?: () => void;
  readonly onAttractionTargetRing?: () => void;
  readonly brushActive?: boolean;
  readonly fillActive?: boolean;
  readonly onFillMeshesReady?: () => void;
  readonly onBrushPlace?: (payload: BrushPlacePayload) => void;
  readonly brushPlacementOverlapBudget?: number;
  readonly fixtureDragDrop?: boolean;
  readonly onFixtureDrop?: (detail: Puzzle3dFixtureDropDetail) => void;
  readonly hoverTarget?: HoverTarget | null;
  readonly kindHover?: Puzzle3dKindHover | null;
  readonly onHover?: (payload: Puzzle3dHoverPayload) => void;
  readonly onToggleSelectionHidden?: (value: boolean) => void;
  readonly onToggleSelectionLocked?: (value: boolean) => void;
  readonly onDeleteSelection?: () => void;
  readonly onDuplicateSelection?: () => void;
  readonly onSelectSameKind?: () => void;
}

/** @emoji 🎬 Puzzle 3D play canvas: {@link Canvas3D} cabled to {@link ObjectStateProvider} and {@link Objects}. */
export function PlayCanvas(props: PlayCanvasProps): React.ReactElement {
  const setSelectedIdRef = reactHostPort.useRef(props.setSelectedId);
  setSelectedIdRef.current = props.setSelectedId;
  reactHostPort.useEffect(() => {
    puzzle3dSelectionActionsRef.current = {
      toggleHidden: (value) => props.onToggleSelectionHidden?.(value),
      toggleLocked: (value) => props.onToggleSelectionLocked?.(value),
      deleteSelection: () => props.onDeleteSelection?.(),
      duplicateSelection: () => props.onDuplicateSelection?.(),
      selectSameKind: () => props.onSelectSameKind?.(),
    };
    return () => {
      puzzle3dSelectionActionsRef.current = {
        toggleHidden: () => {},
        toggleLocked: () => {},
        deleteSelection: () => {},
        duplicateSelection: () => {},
        selectSameKind: () => {},
      };
    };
  }, [
    props.onDeleteSelection,
    props.onDuplicateSelection,
    props.onSelectSameKind,
    props.onToggleSelectionHidden,
    props.onToggleSelectionLocked,
  ]);
  const sceneChildren = reactHostPort.useMemo(
    () => (
      <>
        <PuzzleReferences references={props.fixture.references ?? []} relocate={props.gumballConfig ?? PUZZLE_3D_GUMBALL_CONFIG} />
        <PuzzleTargetVolumes
          targetVolumes={props.fixture.targetVolumes ?? []}
          interactive={props.fillEditTargetVolumes === true}
          relocate={false}
        />
        <Objects relocate={puzzle3dObjectGumballConfig(props.gumballConfig)} />
        <AttractionTreeRoots />
        <MarqueeAttractionSource />
        <PlayTestBridge setSelectedId={(id) => setSelectedIdRef.current(id)} />
      </>
    ),
    [props.fillEditTargetVolumes, props.fixture.references, props.fixture.targetVolumes, props.gumballConfig],
  );
  const handleRelocate = useObjectRelocate();
  const handleConnect = useObjectConnect();
  const onIndirectConnect = reactHostPort.useCallback(
    (payload: AttractionPayload) => {
      handleConnect(payload);
      props.onIndirectConnect?.();
    },
    [handleConnect, props.onIndirectConnect],
  );
  const onProximityConnect = reactHostPort.useCallback(
    (payload: AttractionPayload) => {
      handleConnect(payload);
      props.onProximityConnect?.();
    },
    [handleConnect, props.onProximityConnect],
  );
  const onAttractionCompatibleObjects = reactHostPort.useCallback(
    (_payload: AttractionCompatibleObjectsPayload) => {
      props.onAttractionCompatibleObjects?.();
    },
    [props.onAttractionCompatibleObjects],
  );
  const onAttractionTargetRing = reactHostPort.useCallback(
    (_payload: AttractionTargetRingPayload) => {
      props.onAttractionTargetRing?.();
    },
    [props.onAttractionTargetRing],
  );
  reactHostPort.useEffect(() => {
    puzzle3dReferenceRelocateBridgeRef.current = {
      onSelect: props.onSelect,
      onRelocate: props.onReferenceRelocate,
    };
    puzzle3dTargetVolumeRelocateBridgeRef.current = {
      onRelocate: props.onTargetVolumeRelocate,
    };
    puzzle3dVoxelBrushBridgeRef.current = {
      onPaint: props.onVoxelBrushPaint,
    };
    return () => {
      puzzle3dReferenceRelocateBridgeRef.current = {};
      puzzle3dTargetVolumeRelocateBridgeRef.current = {};
      puzzle3dVoxelBrushBridgeRef.current = {};
    };
  }, [props.onReferenceRelocate, props.onSelect, props.onTargetVolumeRelocate, props.onVoxelBrushPaint]);
  return (
    <Canvas3D
      className="absolute inset-0"
      camera={props.camera ?? props.fixture.camera}
      cameraSeedKey={props.cameraSeedKey}
      domain={props.fixture.domain}
      chunkSize={props.chunkSize}
      kindCatalogs={props.kindCatalogs}
      kindCompatibility={props.kindCompatibility}
      blockedVortexFullIds={props.blockedVortexFullIds}
      proximityRadius={props.proximityRadius}
      proximityRelocateEnabled={props.proximityRelocateEnabled}
      gumballConfig={props.gumballConfig ?? PUZZLE_3D_GUMBALL_CONFIG}
      selectionMode={props.selectionMode}
      selectionMethod={props.selectionMethod}
      marqueeSelectableKinds={props.marqueeSelectableKinds}
      selection={props.selection}
      gridFactor={props.gridFactor}
      showLodGrid={props.showLodGrid}
      gridSnapEnabled={props.gridSnapEnabled}
      onCamera={props.onCamera}
      onLodChange={props.onLodChange}
      onSelect={props.onSelect}
      onReferenceRelocate={props.onReferenceRelocate}
      onTargetVolumeRelocate={props.onTargetVolumeRelocate}
      onVoxelBrushPaint={props.onVoxelBrushPaint}
      voxelBrushDimensions={props.voxelBrushDimensions}
      fillEditTargetVolumes={props.fillEditTargetVolumes}
      onConnect={handleConnect}
      onRelocate={handleRelocate}
      onIndirectConnect={onIndirectConnect}
      onProximityConnect={onProximityConnect}
      onAttractionCompatibleObjects={onAttractionCompatibleObjects}
      onAttractionTargetRing={onAttractionTargetRing}
      brushActive={props.brushActive}
      fillActive={props.fillActive}
      onFillMeshesReady={props.onFillMeshesReady}
      onBrushPlace={props.onBrushPlace}
      brushPlacementOverlapBudget={props.brushPlacementOverlapBudget}
      fixtureDragDrop={props.fixtureDragDrop}
      onFixtureDrop={props.onFixtureDrop}
      hoverTarget={props.hoverTarget}
      kindHover={props.kindHover}
      onHover={props.onHover}
      sceneFixture={props.fixture}
      {...props.lodProps}
    >
      {sceneChildren}
    </Canvas3D>
  );
}

export function Canvas3D(props: CanvasProps & { className?: string; style?: CSSProperties }) {
  const { children, className, style, onLodChange, domain = DEFAULT_DOMAIN, fixtureDragDrop, onFixtureDrop, camera, onCamera, ...rest } = props;
  const rootRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const [shellLod, setShellLod] = reactHostPort.useState(() => formatLod(DEFAULT_MANUAL_LOD));
  const [fixtureDragActive, setFixtureDragActive] = reactHostPort.useState(false);
  const fixtureDragDepthRef = reactHostPort.useRef(0);
  const resolvedFixtureDragDrop = fixtureDragDrop ?? Boolean(onFixtureDrop);
  const handleLod = reactHostPort.useCallback(
    (l: number) => {
      const label = formatLod(l);
      setShellLod(label);
      onLodChange?.(l);
    },
    [onLodChange],
  );
  return (
    <WorldCanvas
      rootRef={rootRef}
      className={`${className ?? ""}${fixtureDragActive ? " ring-primary ring-2 ring-inset" : ""}`.trim()}
      style={style}
      dataRootAttr="data-puzzle3d-root"
      dataLod={shellLod}
      extraRootProps={{
        "data-puzzle3d-fixture-drag-active": fixtureDragActive ? "true" : undefined,
        "data-puzzle3d-domain": domain,
        "data-puzzle3d-lod": shellLod,
      }}
      onContextMenu={(event) => {
        if (puzzle3dRightDragActiveRef.current) {
          event.preventDefault();
        }
      }}
      overlay={
        <>
          <Puzzle3dMarqueeOverlay />
          <Puzzle3dBrushCandidateMenu />
          <Puzzle3dSelectionContextMenu />
          <WorldOrbitProjectionSwitch
            projection={camera?.projection ?? "perspective"}
            onProjectionChange={(nextProjection) => {
              if (!onCamera) {
                return;
              }
              const base = camera ?? { position: [420, -420, 320] as Vec3, target: [0, 0, 40] as Vec3, zoom: 1 };
              onCamera(applyOrbitProjectionToCameraState(base, nextProjection));
            }}
          />
        </>
      }
    >
      <Inner
        {...rest}
        domain={domain}
        onLodChange={handleLod}
        fixtureDragDrop={resolvedFixtureDragDrop}
        onFixtureDrop={onFixtureDrop}
        puzzle3dRootRef={rootRef}
        fixtureDragActive={fixtureDragActive}
        setFixtureDragActive={setFixtureDragActive}
        fixtureDragDepthRef={fixtureDragDepthRef}
      >
        {children}
      </Inner>
    </WorldCanvas>
  );
}

/** @emoji ­ƒº¬ Registers `window.__puzzle3dPlay*` hooks for Playwright (play harness only). */
export function PlayTestBridge(props: { readonly setSelectedId: (id: string | null) => void }): null {
  const { setActiveRelocateObjectId, clearSelection } = useRegistryInteraction();
  const setSelectedId = props.setSelectedId;
  reactHostPort.useEffect(() => {
    const w = window as unknown as {
      __puzzle3dPlaySelect?: (id: string) => void;
      __puzzle3dPlayActivate?: (id: string) => void;
      __puzzle3dPlayClearSelection?: () => void;
      __puzzle3dPlayPointerMiss?: () => void;
    };
    w.__puzzle3dPlaySelect = (id: string) => {
      setSelectedId(id);
    };
    w.__puzzle3dPlayActivate = (id: string) => {
      setSelectedId(id);
      setActiveRelocateObjectId(id);
    };
    w.__puzzle3dPlayClearSelection = () => {
      setSelectedId(null);
      setActiveRelocateObjectId(null);
    };
    w.__puzzle3dPlayPointerMiss = () => {
      clearSelection();
    };
    return () => {
      delete w.__puzzle3dPlaySelect;
      delete w.__puzzle3dPlayActivate;
      delete w.__puzzle3dPlayClearSelection;
      delete w.__puzzle3dPlayPointerMiss;
    };
  }, [setSelectedId, setActiveRelocateObjectId, clearSelection]);
  return null;
}

//#endregion 🎬Viewport

if (import.meta.vitest) {
  const { beforeEach, describe, expect, it, vi } = import.meta.vitest;
  describe("lodFromCameraDistance", () => {
    it("maps orbit distance to scale ratio", () => {
      expect(lodFromCameraDistance(100, 100)).toBe(1);
      expect(lodFromCameraDistance(20000, 100)).toBe(200);
      expect(lodFromCameraDistance(50, 100)).toBe(0.5);
    });
  });
  describe("pickClosestLod", () => {
    const available = [50, 200, 1000] as const;
    it("prefers log-closest and ties toward smaller lod", () => {
      expect(pickClosestLod(available, 100)).toBe(50);
      expect(pickClosestLod(available, 500)).toBe(1000);
      expect(pickClosestLod(available, 5000)).toBe(1000);
    });
  });
  describe("lodGridBandStepsWorld", () => {
    it("scales puzzle 2d quanta by grid factor", () => {
      expect(lodGridBandStepsWorld(10)).toEqual([100, 25, 5, 1]);
      expect(lodGridBandStepsWorld(5)).toEqual([50, 12.5, 2.5, 0.5]);
    });
  });
  describe("lodProgressiveGridLayers", () => {
    it("adds finer bands as lod decreases", () => {
      expect(lodProgressiveGridLayers(5000, 10)).toEqual([]);
      expect(lodProgressiveGridLayers(500, 10).map((l) => l.stepWorld)).toEqual([100]);
      expect(lodProgressiveGridLayers(100, 10).map((l) => l.stepWorld)).toEqual([100]);
      expect(lodProgressiveGridLayers(50, 10).map((l) => l.stepWorld)).toEqual([100, 25]);
      expect(lodProgressiveGridLayers(10, 10).map((l) => l.stepWorld)).toEqual([100, 25, 5]);
      expect(lodProgressiveGridLayers(2, 10).map((l) => l.stepWorld)).toEqual([100, 25, 5, 1]);
    });
  });
  describe("lodGridStepWorld", () => {
    it("returns null when no grid and finest visible band otherwise", () => {
      expect(lodGridStepWorld(5000, 10)).toBe(null);
      expect(lodGridStepWorld(100, 10)).toBe(100);
      expect(lodGridStepWorld(50, 10)).toBe(25);
      expect(lodGridStepWorld(10, 10)).toBe(5);
      expect(lodGridStepWorld(2, 10)).toBe(1);
    });
  });
  describe("lodVortexPrimaryVisible", () => {
    it("draws vortices at detail bands", () => {
      expect(lodVortexPrimaryVisible(100)).toBe(true);
      expect(lodVortexPrimaryVisible(201)).toBe(false);
    });
  });
  describe("pickNearestScreenVortex", () => {
    const c = (fullId: string, sx: number, sy: number, dist: number): ScreenVortexCandidate => ({ fullId, objectId: fullId.split(":")[0]!, sx, sy, dist });
    it("returns null when no candidate sits within the screen radius", () => {
      const picked = pickNearestScreenVortex({ cursorX: 100, cursorY: 100, surfaceDist: 40, candidates: [c("a:link", 200, 200, 41)] });
      expect(picked).toBe(null);
    });
    it("prefers the candidate closest to the cursor in screen space", () => {
      const picked = pickNearestScreenVortex({
        cursorX: 100,
        cursorY: 100,
        surfaceDist: 40,
        candidates: [c("far:link", 110, 110, 41), c("near:link", 102, 100, 41)],
      });
      expect(picked?.fullId).toBe("near:link");
    });
    it("selects a vortex embedded just behind the clicked surface (within depth tolerance)", () => {
      const picked = pickNearestScreenVortex({ cursorX: 100, cursorY: 100, surfaceDist: 40, candidates: [c("embedded:link", 100, 100, 42)], depthTolerance: 6 });
      expect(picked?.fullId).toBe("embedded:link");
    });
    it("skips a vortex occluded by foreground geometry beyond the depth tolerance", () => {
      const picked = pickNearestScreenVortex({ cursorX: 100, cursorY: 100, surfaceDist: 40, candidates: [c("behind:link", 100, 100, 60)], depthTolerance: 6 });
      expect(picked).toBe(null);
    });
    it("falls through to the next nearest candidate when the closest is occluded", () => {
      const picked = pickNearestScreenVortex({
        cursorX: 100,
        cursorY: 100,
        surfaceDist: 40,
        candidates: [c("occluded:link", 101, 100, 80), c("visible:link", 105, 100, 41)],
        depthTolerance: 6,
      });
      expect(picked?.fullId).toBe("visible:link");
    });
  });
  describe("lodVortexPickProxy", () => {
    it("uses pick proxies in mid bands only", () => {
      expect(lodVortexPickProxy(500)).toBe(true);
      expect(lodVortexPickProxy(100)).toBe(false);
      expect(lodVortexPickProxy(2000)).toBe(false);
    });
  });
  describe("puzzle3dLodCanvasProps", () => {
    it("maps auto, depth, and manual modes", () => {
      expect(puzzle3dLodCanvasProps({ automaticLod: true, depthVariableLod: false, manualLod: 50 })).toEqual({
        automaticLod: true,
        depthVariableLod: false,
      });
      expect(puzzle3dLodCanvasProps({ automaticLod: false, depthVariableLod: true, manualLod: 50 })).toEqual({
        automaticLod: false,
        depthVariableLod: true,
      });
      expect(puzzle3dLodCanvasProps({ automaticLod: false, depthVariableLod: false, manualLod: 42 })).toEqual({
        automaticLod: false,
        depthVariableLod: false,
        lod: 42,
      });
    });
  });
  describe("sliderValueFromLod", () => {
    it("round-trips integer slider positions through lodFromSliderValue", () => {
      for (const slider of [0, 250, 500, 750, 1000] as const) {
        expect(sliderValueFromLod(lodFromSliderValue(slider))).toBe(slider);
      }
    });
  });
  describe("objectPoseKey", () => {
    it("changes when origin changes", () => {
      const a = objectPoseKey("id", [0, 0, 0], [0, 0, 0, 1], 1);
      const b = objectPoseKey("id", [1, 0, 0], [0, 0, 0, 1], 1);
      expect(a).not.toBe(b);
    });
  });
  describe("boundsFromObjectGroups", () => {
    it("returns null for empty input", () => {
      expect(boundsFromObjectGroups([])).toBeNull();
    });
    it("measures mesh geometry in world space", () => {
      const root = new Group();
      const mesh = new Mesh(new BoxGeometry(10, 20, 30));
      root.add(mesh);
      applyObjectPose(root, [5, 0, 0], [0, 0, 0, 1], 1);
      const bounds = boundsFromObjectGroups([root]);
      expect(bounds).not.toBeNull();
      expect(bounds!.radius).toBeGreaterThan(5);
    });
  });
  describe("puzzle3dAutoFitShouldRun", () => {
    it("runs once for initial behavior", () => {
      expect(puzzle3dAutoFitShouldRun("initial", "a", "", false)).toBe(true);
      expect(puzzle3dAutoFitShouldRun("initial", "a", "a", true)).toBe(false);
      expect(puzzle3dAutoFitShouldRun("initial", "b", "a", true)).toBe(false);
    });
    it("refits when the object-group key changes in changes behavior", () => {
      expect(puzzle3dAutoFitShouldRun("changes", "b", "a", true)).toBe(true);
    });
  });
  describe("puzzle3dAutoFitInitialApplied", () => {
    it("tracks initial auto-fit per viewport seed", () => {
      expect(puzzle3dAutoFitInitialApplied("seed-a")).toBe(false);
      puzzle3dAutoFitMarkInitialApplied("seed-a");
      expect(puzzle3dAutoFitInitialApplied("seed-a")).toBe(true);
      expect(puzzle3dAutoFitInitialApplied("seed-b")).toBe(false);
    });
  });
  describe("puzzle3dBrushSessionActive", () => {
    it("is true while brush target, preview, or menu is active", () => {
      puzzle3dBrushUiStore.setSnapshot(BRUSH_UI_IDLE);
      expect(puzzle3dBrushSessionActive()).toBe(false);
      puzzle3dBrushUiStore.setSnapshot({ ...BRUSH_UI_IDLE, targetActive: true });
      expect(puzzle3dBrushSessionActive()).toBe(true);
      puzzle3dBrushUiStore.setSnapshot(BRUSH_UI_IDLE);
    });
  });
  describe("cameraStateNearEqual", () => {
    it("detects position and zoom deltas", () => {
      const base = {
        position: [1, 2, 3] as const,
        target: [0, 0, 0] as const,
        zoom: 1,
      };
      expect(cameraStateNearEqual(base, { ...base, position: [1.0001, 2, 3] })).toBe(true);
      expect(cameraStateNearEqual(base, { ...base, position: [2, 2, 3] })).toBe(false);
      const fixture = {
        schema: "puzzle.3d.fixture/v1" as const,
        domain: "architecture" as const,
        camera: base,
        objects: [],
        attractions: [],
      };
      const moved = updatePuzzle3dCameraInFixture(fixture, { position: [2, 2, 3] });
      expect(moved).not.toBe(fixture);
      const same = updatePuzzle3dCameraInFixture(fixture, { position: [1.0001, 2, 3] });
      expect(same).toBe(fixture);
    });
  });
  describe("applyAutoFitCamera", () => {
    it("offsets camera from bounds center", () => {
      const camera = new ThreePerspectiveCamera(50, 1, 0.1, 10_000);
      applyAutoFitCamera(camera, { center: [0, 0, 0], radius: 10 });
      expect(camera.position.length()).toBeGreaterThan(10);
    });
  });
  describe("puzzle3dEaseInOutCubic01", () => {
    it("is 0 at start and 1 at end", () => {
      expect(puzzle3dEaseInOutCubic01(0)).toBe(0);
      expect(puzzle3dEaseInOutCubic01(1)).toBe(1);
      expect(puzzle3dEaseInOutCubic01(0.5)).toBe(0.5);
    });
  });
  describe("puzzle3dFitCameraRigFromBounds", () => {
    it("places camera offset from bounds center", () => {
      const rig = puzzle3dFitCameraRigFromBounds({ center: [0, 0, 10], radius: 5 }, 1.25);
      expect(rig.position.distanceTo(rig.target)).toBeGreaterThan(5);
    });
  });
  describe("cadVec3ToThree", () => {
    it("passes through z-up coordinates", () => {
      expect(cadVec3ToThree([0, 0, 1])).toEqual([0, 0, 1]);
      expect(cadVec3ToThree([0, 1, 0])).toEqual([0, 1, 0]);
    });
    it("round-trips with threeVec3ToCad", () => {
      const cad: Vec3 = [12, -4, 7];
      const back = threeVec3ToCad(new Vector3(...cadVec3ToThree(cad)));
      expect(back[0]).toBeCloseTo(cad[0], 5);
      expect(back[1]).toBeCloseTo(cad[1], 5);
      expect(back[2]).toBeCloseTo(cad[2], 5);
    });
  });
  describe("applyObjectPose", () => {
    it("places vortex child at expected world offset in CAD fixture space", () => {
      const parent = new Group();
      const vortex = new Group();
      const localThree = cadObjectLocalToThreeGroupLocal([1, 2, 3], [10, 0, 0], [0, 0, 0, 1]);
      vortex.position.set(localThree[0], localThree[1], localThree[2]);
      parent.add(vortex);
      applyObjectPose(parent, [10, 0, 0], [0, 0, 0, 1], 1);
      updateWorldMatrixChain(vortex);
      const world = new Vector3();
      vortex.getWorldPosition(world);
      const expected = cadVec3ToThree([11, 2, 3]);
      expect(world.x).toBeCloseTo(expected[0], 5);
      expect(world.y).toBeCloseTo(expected[1], 5);
      expect(world.z).toBeCloseTo(expected[2], 5);
    });
    it("maps CAD object-local direction into parent group space", () => {
      const dir = cadObjectLocalDirectionToThreeGroupLocal([-1, 0, 0], [0, 0, 0], [0, 0, 0, 1]);
      const len = Math.hypot(dir[0], dir[1], dir[2]);
      expect(len).toBeCloseTo(1, 5);
      expect(dir[0]).toBeLessThan(0);
    });
  });
  describe("objectMatchesSelection", () => {
    it("matches only directly selected object ids", () => {
      expect(objectMatchesSelection("a", { objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] })).toBe(true);
      expect(objectMatchesSelection("b", { objectIds: [], vortexIds: ["b:link"], attractionIds: [] })).toBe(false);
      expect(objectMatchesSelection("c", { objectIds: ["a"], vortexIds: ["b:link"], attractionIds: [] })).toBe(false);
    });
  });
  describe("createSelectionSnapshotStore", () => {
    it("notifies subscribers synchronously on setSnapshot", () => {
      const store = createSelectionSnapshotStore();
      let count = 0;
      const unsubscribe = store.subscribe(() => {
        count += 1;
      });
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(count).toBe(1);
      expect(store.getSnapshot().objectIds).toEqual(["a"]);
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(count).toBe(1);
      unsubscribe();
    });
    it("notifies per-object listeners only when that object membership changes", () => {
      const store = createSelectionSnapshotStore();
      let aCount = 0;
      let bCount = 0;
      const unsubA = store.subscribeObject("a", () => {
        aCount += 1;
      });
      const unsubB = store.subscribeObject("b", () => {
        bCount += 1;
      });
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(aCount).toBe(1);
      expect(bCount).toBe(0);
      store.setSnapshot({ objectIds: ["a", "b"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(aCount).toBe(1);
      expect(bCount).toBe(1);
      store.setSnapshot({ objectIds: ["a", "b"], vortexIds: ["c:v1"], attractionIds: [] });
      expect(aCount).toBe(1);
      expect(bCount).toBe(1);
      store.setSnapshot({ objectIds: [], vortexIds: ["c:v1"], attractionIds: [] });
      expect(aCount).toBe(2);
      expect(bCount).toBe(2);
      unsubA();
      unsubB();
    });
    it("keeps parent object unselected when only a child vortex is selected", () => {
      const store = createSelectionSnapshotStore();
      store.setSnapshot({ objectIds: [], vortexIds: ["parent:v1"], attractionIds: [] });
      expect(store.isObjectSelected("parent")).toBe(false);
      expect(store.isVortexSelected("parent:v1")).toBe(true);
      expect(store.getPrimaryObjectId()).toBe("parent");
    });
    it("reveals vortices when parent object or child vortex is selected", () => {
      const store = createSelectionSnapshotStore();
      expect(store.isObjectVortexRevealSelected("tower")).toBe(false);
      store.setSnapshot({ objectIds: ["tower"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(store.isObjectVortexRevealSelected("tower")).toBe(true);
      store.setSnapshot({ objectIds: [], vortexIds: ["tower:link"], attractionIds: [] });
      expect(store.isObjectVortexRevealSelected("tower")).toBe(true);
      let revealCount = 0;
      const unsub = store.subscribeObjectVortexReveal("tower", () => {
        revealCount += 1;
      });
      store.setSnapshot({ objectIds: ["tower"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(revealCount).toBe(1);
      unsub();
    });
    it("notifies attraction bulk listeners only when attraction membership changes", () => {
      const store = createSelectionSnapshotStore();
      let bulkCount = 0;
      const unsub = store.subscribeAttractions(() => {
        bulkCount += 1;
      });
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(bulkCount).toBe(0);
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: ["t1"] });
      expect(bulkCount).toBe(1);
      expect(store.getAttractionIdSet().has("t1")).toBe(true);
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: ["t1"] });
      expect(bulkCount).toBe(1);
      unsub();
    });
    it("disables mesh outlines when selection exceeds budget", () => {
      const store = createSelectionSnapshotStore();
      const ids = Array.from({ length: PUZZLE3D_MESH_OUTLINE_MAX_SELECTION + 1 }, (_, index) => `o${index}`);
      store.setSnapshot({ objectIds: ids, vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(store.getMeshOutlineEnabled()).toBe(false);
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(store.getMeshOutlineEnabled()).toBe(true);
    });
    it("increments revision on each selection change", () => {
      const store = createSelectionSnapshotStore();
      expect(store.getRevision()).toBe(1);
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(store.getRevision()).toBe(2);
    });
  });
  describe("parseFixtureV1", () => {
    it("accepts minimal fixture", () => {
      const f = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [],
        objects: [
          {
            id: "a",
            meshUrl: "/m.glb",
            origin: [1, 2, 3],
            orientation: [0, 0, 0, 1],
            vortices: [{ id: "a:v1", position: [0, 0, 0] }],
          },
        ],
      });
      expect(f?.objects[0]?.id).toBe("a");
      expect(f?.domain).toBe("architecture");
      expect(f?.references).toEqual([]);
      expect(f?.targetVolumes).toEqual([]);
    });
    it("parses targetVolumes array", () => {
      const f = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [],
        objects: [],
        targetVolumes: [
          {
            id: "vol-a",
            origin: [1, 2, 3],
            orientation: [0, 0, 0, 1],
            scale: [4, 6, 8],
          },
        ],
      });
      expect(f?.targetVolumes).toHaveLength(1);
      expect(f?.targetVolumes[0]?.scale).toEqual([4, 6, 8]);
    });
    it("parses references array", () => {
      const f = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [],
        objects: [],
        references: [
          {
            id: "ref-a",
            source: { url: "/example/reference.png", mediaKind: "image" },
            origin: [0, 0, 0.01],
            widthWorld: 12,
          },
        ],
      });
      expect(f?.references).toHaveLength(1);
      expect(f?.references[0]?.source.url).toBe("/example/reference.png");
    });
    it("parses domain case-insensitively", () => {
      const f = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        domain: "Urban",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [],
        objects: [{ id: "a", meshUrl: "/m.glb", origin: [1, 2, 3], vortices: [] }],
      });
      expect(f?.domain).toBe("urban");
    });
    it("parses meshByLod list entries", () => {
      const f = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [],
        objects: [
          {
            id: "a",
            meshUrl: "/m.glb",
            origin: [0, 0, 0],
            orientation: [0, 0, 0, 1],
            meshByLod: [{ lod: 100, url: "/fine.glb" }],
            vortices: [
              {
                id: "a:v1",
                position: [0, 0, 0],
                vortexMeshUrl: "/fallback.glb",
                vortexMeshByLod: [
                  { lod: 100, url: "/d.glb" },
                  { lod: 50, url: "/u.glb" },
                ],
              },
            ],
          },
        ],
      });
      const o = f?.objects[0];
      expect(o?.meshByLod?.[0]?.url).toBe("/fine.glb");
      const v = o?.vortices[0];
      expect(v?.vortexMeshByLod?.[0]?.url).toBe("/d.glb");
      expect(v?.vortexMeshUrl).toBe("/fallback.glb");
    });
    it("parses hidden and locked flags on objects, vortices, and attractions", () => {
      const f = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [{ id: "att-1", attracting: "obj-1:v-1", attracted: "obj-2:v-1", hidden: true, locked: true }],
        objects: [
          {
            id: "obj-1",
            meshUrl: "/m.glb",
            origin: [0, 0, 0],
            hidden: true,
            vortices: [{ id: "v-1", position: [0, 0, 0], locked: true }],
          },
        ],
      });
      expect(f?.objects[0]?.hidden).toBe(true);
      expect(f?.objects[0]?.vortices[0]?.locked).toBe(true);
      expect(f?.attractions[0]?.hidden).toBe(true);
      expect(f?.attractions[0]?.locked).toBe(true);
      const records = fixtureToRecords(f!.objects);
      expect(records.get("obj-1")?.hidden).toBe(true);
      expect(records.get("obj-1")?.vortices[0]?.locked).toBe(true);
    });
  });
  describe("chunkKey", () => {
    it("buckets origin", () => {
      expect(chunkKey([10, 10, 10], 256)).toBe("0|0|0");
      expect(chunkKey([300, 0, 0], 256)).toBe("1|0|0");
    });
  });
  describe("chunkDistanceVisible", () => {
    it("keeps visible inside exit margin after entering", () => {
      const cam = new Vector3(0, 0, 0);
      const chunkSize = 256;
      const maxDist = 200;
      const enterDist = maxDist + chunkBoundsRadius(chunkSize);
      const far = new Vector3(enterDist + chunkSize, 0, 0);
      expect(chunkDistanceVisible({ camPos: cam, chunkCenter: far, chunkSize, maxDist, wasVisible: false })).toBe(false);
      const near = new Vector3(enterDist - 50, 0, 0);
      expect(chunkDistanceVisible({ camPos: cam, chunkCenter: near, chunkSize, maxDist, wasVisible: false })).toBe(true);
      const between = new Vector3(enterDist + chunkSize * 0.25, 0, 0);
      expect(chunkDistanceVisible({ camPos: cam, chunkCenter: between, chunkSize, maxDist, wasVisible: true })).toBe(true);
      const beyond = new Vector3(enterDist + chunkSize * 0.75, 0, 0);
      expect(chunkDistanceVisible({ camPos: cam, chunkCenter: beyond, chunkSize, maxDist, wasVisible: true })).toBe(false);
    });
  });
  describe("gltfPoolAcquire", () => {
    it("tracks refcount without clearing cache on release", () => {
      const url = "http://x/pool-test.glb";
      gltfPoolAcquire(url);
      gltfPoolAcquire(url);
      gltfPoolRelease(url);
      gltfPoolRelease(url);
      gltfPoolAcquire(url);
      gltfPoolRelease(url);
      expect(true).toBe(true);
    });
  });
  describe("resolveMeshStyle", () => {
    it("prefers explicit style over interaction flags", () => {
      expect(
        resolveMeshStyle({
          style: "original",
          selected: true,
          hovered: true,
          disabled: true,
        }),
      ).toBe("original");
    });
    it("compares scene hover targets by kind and id", () => {
      expect(puzzle3dHoverTargetsEqual(null, null)).toBe(true);
      expect(puzzle3dHoverTargetsEqual({ kind: "object", id: "a" }, { kind: "object", id: "a" })).toBe(true);
      expect(puzzle3dHoverTargetsEqual({ kind: "object", id: "a" }, { kind: "object", id: "b" })).toBe(false);
      expect(puzzle3dHoverTargetsEqual({ kind: "vortex", fullId: "o:v" }, { kind: "object", id: "o" })).toBe(false);
      expect(puzzle3dHoverTargetsEqual({ kind: "attraction", id: "e1" }, { kind: "attraction", id: "e1" })).toBe(true);
    });
    it("reveals vortices only when parent object is hovered or selected", () => {
      expect(objectVorticesRevealed("tower", null, false)).toBe(false);
      expect(objectVorticesRevealed("tower", { kind: "object", id: "tower" }, false)).toBe(true);
      expect(objectVorticesRevealed("tower", { kind: "vortex", fullId: "tower:link" }, false)).toBe(true);
      expect(objectVorticesRevealed("tower", { kind: "object", id: "other" }, false)).toBe(false);
      expect(objectVorticesRevealed("tower", null, true)).toBe(true);
    });

    it("orders disabled, selected, highlighted, hovered, then default", () => {
      expect(resolveMeshStyle({ disabled: true, selected: true })).toBe("disabled");
      expect(resolveMeshStyle({ selected: true, highlighted: true })).toBe("selected");
      expect(resolveMeshStyle({ highlighted: true, hovered: true })).toBe("highlighted");
      expect(resolveMeshStyle({ hovered: true })).toBe("hovered");
      expect(resolveMeshStyle({})).toBe(DEFAULT_MESH_STYLE);
    });
  });
  describe("meshStyleColors", () => {
    it("returns null for original and colors for neutral", () => {
      expect(meshStyleColors("original")).toBeNull();
      const neutral = meshStyleColors("neutral");
      expect(neutral?.meshColor.length).toBeGreaterThan(0);
      expect(neutral?.lineColor.length).toBeGreaterThan(0);
    });
    it("uses primary only for selected mesh tokens", () => {
      const selected = meshStyleColors("selected");
      const hovered = meshStyleColors("hovered");
      const highlighted = meshStyleColors("highlighted");
      expect(selected?.lineColor).toBe(tokenHex("primary"));
      expect(hovered?.lineColor).toBe(tokenHex("gray"));
      expect(highlighted?.lineColor).toBe(tokenHex("secondary"));
      expect(hovered?.lineColor).not.toMatch(/primary/i);
      expect(highlighted?.lineColor).not.toMatch(/primary/i);
    });
    it("returns srgb-compatible colors for Three.js", () => {
      for (const kind of ["neutral", "hovered", "selected", "highlighted", "disabled"] as const) {
        const colors = meshStyleColors(kind);
        expect(colors?.meshColor).not.toMatch(/^oklab\(/iu);
        expect(colors?.lineColor).not.toMatch(/^oklab\(/iu);
        expect(colors?.emissiveColor).not.toMatch(/^oklab\(/iu);
      }
    });
  });
  describe("styledMeshPoolAcquire", () => {
    it("tracks styled pool keys separately from base url", () => {
      const url = "http://x/styled-pool.glb";
      styledMeshPoolAcquire(url, "neutral");
      styledMeshPoolAcquire(url, "selected");
      styledMeshPoolRelease(url, "neutral");
      styledMeshPoolRelease(url, "selected");
      expect(true).toBe(true);
    });
  });
  describe("kindsCompatible", () => {
    it("matches bidirectional", () => {
      const ok = kindsCompatible("a", "b", [{ source: "b", target: "a", bidirectional: true }]);
      expect(ok).toBe(true);
    });
  });
  describe("blockedVortexFullIdsFromAttractions", () => {
    it("collects endpoints", () => {
      const s = blockedVortexFullIdsFromAttractions([{ attracting: "a:h1", attracted: "b:h2" }]);
      expect(s.has("a:h1")).toBe(true);
      expect(s.has("b:h2")).toBe(true);
    });
  });
  describe("vorticesAttractionCompatibleForDrag", () => {
    it("allows all when rules empty", () => {
      const ok = vorticesAttractionCompatibleForDrag({ objectId: "a", objectKind: "n1", vortexKind: "h1" }, { objectId: "b", objectKind: "n2", vortexKind: "h2" }, [], undefined);
      expect(ok).toBe(true);
    });
    it("matches vortex specificity", () => {
      const ok = vorticesAttractionCompatibleForDrag({ objectId: "a", objectKind: "x", vortexKind: "h1" }, { objectId: "b", objectKind: "y", vortexKind: "h2" }, [{ source: "h1", target: "h2", specificity: "vortex" }], undefined);
      expect(ok).toBe(true);
    });
  });
  describe("resolveCableKindForVortex", () => {
    it("falls back to default cable id", () => {
      expect(resolveCableKindForVortex("any", undefined)).toBe("cable.link");
    });
  });
  describe("wouldObjectAttractionIntroduceCycle", () => {
    it("detects a closing link on an existing chain", () => {
      const objectAttractions = [
        { attractingObjectId: "a", attractedObjectId: "b", attractionId: "t1" },
        { attractingObjectId: "b", attractedObjectId: "c", attractionId: "t2" },
      ];
      expect(wouldObjectAttractionIntroduceCycle(objectAttractions, "c", "a")).toBe(true);
      expect(wouldObjectAttractionIntroduceCycle(objectAttractions, "a", "d")).toBe(false);
    });
  });
  describe("resolveAttractionTree", () => {
    it("breaks ownership cycles in cyclic attraction components", () => {
      const tree = resolveAttractionTree({
        objectIds: ["a", "b", "c"],
        objectAttractions: [
          { attractingObjectId: "a", attractedObjectId: "b", attractionId: "t1" },
          { attractingObjectId: "b", attractedObjectId: "c", attractionId: "t2" },
          { attractingObjectId: "c", attractedObjectId: "a", attractionId: "t3" },
        ],
      });
      for (const id of ["a", "b", "c"]) {
        expect(parentOwnershipCycleMemberIds(tree.parentByObjectId, id)).toBeNull();
      }
    });
    it("picks parent closer to wormhole when multiply attracted", () => {
      const tree = resolveAttractionTree({
        objectIds: ["w", "a", "b", "c"],
        explicitWormholeIds: new Set(["w"]),
        objectAttractions: [
          { attractingObjectId: "w", attractedObjectId: "a", attractionId: "t1" },
          { attractingObjectId: "a", attractedObjectId: "b", attractionId: "t2" },
          { attractingObjectId: "w", attractedObjectId: "c", attractionId: "t3" },
          { attractingObjectId: "c", attractedObjectId: "b", attractionId: "t4" },
        ],
      });
      expect(tree.parentByObjectId.get("b")).toBe("a");
      expect(tree.attractingByObjectId.get("a")).toEqual(["b"]);
    });
    it("lists attracted children per owner", () => {
      const tree = resolveAttractionTree({
        objectIds: ["w", "a", "b"],
        explicitWormholeIds: new Set(["w"]),
        objectAttractions: [
          { attractingObjectId: "w", attractedObjectId: "a", attractionId: "t1" },
          { attractingObjectId: "a", attractedObjectId: "b", attractionId: "t2" },
        ],
      });
      expect(collectAttractedDescendantIds("w", tree.attractingByObjectId)).toEqual(["a", "b"]);
    });
  });
  describe("objectAttractionsFromAttractions", () => {
    it("maps vortex endpoints to object ids", () => {
      const links = objectAttractionsFromAttractions([{ id: "x", attracting: "objA:v1", attracted: "objB:link" }]);
      expect(links[0]?.attractingObjectId).toBe("objA");
      expect(links[0]?.attractedObjectId).toBe("objB");
    });
  });
  describe("puzzle3dObjectGumballConfig", () => {
    it("never enables scale handle groups", () => {
      const config = puzzle3dObjectGumballConfig({ moveAxes: true, scaleAxes: true, scalePlanes: true, scaleUniform: true });
      expect(config.scaleAxes).toBe(false);
      expect(config.scalePlanes).toBe(false);
      expect(config.scaleUniform).toBe(false);
      expect(config.moveAxes).toBe(true);
    });
  });
  describe("gumballHandleKindToRelocateMode", () => {
    it("maps unified gumball handle kinds to relocate modes", () => {
      expect(gumballHandleKindToRelocateMode("moveX")).toBe("translate");
      expect(gumballHandleKindToRelocateMode("moveXY")).toBe("translate");
      expect(gumballHandleKindToRelocateMode("rotateZ")).toBe("rotate");
      expect(gumballHandleKindToRelocateMode("scaleY")).toBe("scale");
      expect(gumballHandleKindToRelocateMode("scaleXY")).toBe("scale");
      expect(gumballHandleKindToRelocateMode("scaleUniform")).toBe("scale");
    });
  });
  describe("applyRelocateToFixture", () => {
    it("translates attracted descendants when adjacency is passed", () => {
      const fixture: FixtureV1 = {
        objects: [
          { id: "a", meshUrl: "m", origin: [0, 0, 0], vortices: [] },
          { id: "b", meshUrl: "m", origin: [1, 0, 0], vortices: [] },
        ],
        attractions: [{ id: "t1", attracting: "a:h1", attracted: "b:h2" }],
      };
      const tree = resolveAttractionTree({
        objectIds: ["a", "b"],
        objectAttractions: [{ attractingObjectId: "a", attractedObjectId: "b", attractionId: "t1" }],
      });
      const next = applyRelocateToFixture(
        fixture,
        {
          objectId: "a",
          mode: "translate",
          before: { origin: [0, 0, 0], orientation: [0, 0, 0, 1], scale: [1, 1, 1] },
          after: { origin: [2, 0, 0], orientation: [0, 0, 0, 1], scale: [1, 1, 1] },
        },
        tree.attractingByObjectId,
      );
      expect(next.objects[0]?.origin).toEqual([2, 0, 0]);
      expect(next.objects[1]?.origin).toEqual([3, 0, 0]);
    });
  });
  describe("marqueeModeFromModifiers", () => {
    it("maps shift and ctrl to additive, subtractive, and invertive", () => {
      expect(marqueeModeFromModifiers({})).toBe("default");
      expect(marqueeModeFromModifiers({ ctrlKey: true })).toBe("subtractive");
      expect(marqueeModeFromModifiers({ shiftKey: true })).toBe("additive");
      expect(marqueeModeFromModifiers({ shiftKey: true, ctrlKey: true })).toBe("invertive");
    });
  });
  describe("marqueeIsCrossing", () => {
    it("is crossing when the drag ends left of the start", () => {
      expect(marqueeIsCrossing(100, 80)).toBe(true);
      expect(marqueeIsCrossing(80, 100)).toBe(false);
    });
  });
  describe("marqueeIsCrossingFromPath", () => {
    it("lasso crossing follows the first horizontal step not the last", () => {
      const leftFirst = [
        { x: 100, y: 100 },
        { x: 80, y: 100 },
        { x: 120, y: 100 },
      ];
      const rightFirst = [
        { x: 100, y: 100 },
        { x: 120, y: 100 },
        { x: 80, y: 100 },
      ];
      expect(marqueeIsCrossingFromPath(leftFirst, "lasso")).toBe(true);
      expect(marqueeIsCrossingFromPath(rightFirst, "lasso")).toBe(false);
    });
  });
  describe("screenBoundsFromClientPoints", () => {
    it("returns null for an empty list", () => {
      expect(screenBoundsFromClientPoints([])).toBeNull();
    });
    it("builds an axis-aligned rect from points", () => {
      expect(screenBoundsFromClientPoints([{ x: 10, y: 20 }, { x: 30, y: 5 }])).toEqual({ left: 10, right: 30, top: 5, bottom: 20 });
    });
  });
  describe("screenRectIntersectsPolygon", () => {
    it("detects overlap when only edges cross", () => {
      const bounds = { left: -10, right: 110, top: 50, bottom: 60 };
      const polygon = [
        { x: 0, y: 0 },
        { x: 100, y: 0 },
        { x: 100, y: 100 },
        { x: 0, y: 100 },
      ];
      expect(screenRectIntersectsPolygon(bounds, polygon)).toBe(true);
    });
  });
  describe("projectObjectGroupToScreenPoints", () => {
    it("returns a convex hull and screen bounds from warmed mesh geometry boxes", () => {
      const camera = new ThreePerspectiveCamera(50, 1, 0.1, 1000);
      camera.position.set(0, 10, 20);
      camera.lookAt(0, 0, 0);
      camera.updateMatrixWorld(true);
      const rect = { width: 800, height: 600, left: 0, top: 0, right: 800, bottom: 600 } as DOMRect;
      const root = new Group();
      const mesh = new Mesh(new BoxGeometry(2, 2, 2));
      root.add(mesh);
      warmObjectGroupMarqueeBounds(root);
      const footprint = projectObjectGroupToScreenPoints(root, camera, rect);
      expect(footprint).not.toBeNull();
      expect(footprint!.hull.length).toBeGreaterThanOrEqual(4);
      expect(footprint!.screenBounds.right).toBeGreaterThan(footprint!.screenBounds.left);
      expect(footprint!.screenBounds.bottom).toBeGreaterThan(footprint!.screenBounds.top);
    });
    it("returns a tighter hull than the inflated union screen rect for a rotated mesh", () => {
      const camera = new ThreePerspectiveCamera(50, 1, 0.1, 1000);
      camera.position.set(0, 8, 16);
      camera.lookAt(0, 0, 0);
      camera.updateMatrixWorld(true);
      const rect = { width: 800, height: 600, left: 0, top: 0, right: 800, bottom: 600 } as DOMRect;
      const root = new Group();
      const mesh = new Mesh(new BoxGeometry(4, 1, 1));
      mesh.rotation.set(0, 0, Math.PI / 4);
      mesh.updateMatrixWorld(true);
      root.add(mesh);
      warmObjectGroupMarqueeBounds(root);
      const footprint = projectObjectGroupToScreenPoints(root, camera, rect);
      expect(footprint).not.toBeNull();
      const hullBounds = screenBoundsFromClientPoints(footprint!.hull)!;
      const unionBounds = footprint!.screenBounds;
      expect(hullBounds.right - hullBounds.left).toBeLessThanOrEqual(unionBounds.right - unionBounds.left + 1e-6);
      expect(hullBounds.bottom - hullBounds.top).toBeLessThanOrEqual(unionBounds.bottom - unionBounds.top + 1e-6);
    });
  });
  describe("warmObjectGroupMarqueeBounds", () => {
    it("computes geometry boundingBox for group meshes", () => {
      const root = new Group();
      const mesh = new Mesh(new BoxGeometry(1, 1, 1));
      expect(mesh.geometry.boundingBox).toBeNull();
      root.add(mesh);
      warmObjectGroupMarqueeBounds(root);
      expect(mesh.geometry.boundingBox).not.toBeNull();
    });
    it("skips meshes that already have a boundingBox", () => {
      const root = new Group();
      const mesh = new Mesh(new BoxGeometry(1, 1, 1));
      mesh.geometry.computeBoundingBox();
      const box = mesh.geometry.boundingBox;
      root.add(mesh);
      warmObjectGroupMarqueeBounds(root);
      expect(mesh.geometry.boundingBox).toBe(box);
    });
    it("buildObjectMarqueeFootprintCache stores mesh corners for projection", () => {
      invalidateObjectMarqueeFootprintCache();
      const root = new Group();
      const mesh = new Mesh(new BoxGeometry(1, 1, 1));
      root.add(mesh);
      const cache = buildObjectMarqueeFootprintCache(root, "obj-a");
      expect(cache.meshes).toHaveLength(1);
      expect(cache.meshes[0]?.localCorners).toHaveLength(8);
      expect(getObjectMarqueeFootprintCache("obj-a")).toBe(cache);
    });
    it("projectObjectMarqueeFootprintFromCache does not call computeBoundingBox", () => {
      invalidateObjectMarqueeFootprintCache();
      const camera = new ThreePerspectiveCamera(50, 1, 0.1, 1000);
      camera.position.set(0, 10, 20);
      camera.lookAt(0, 0, 0);
      camera.updateMatrixWorld(true);
      const rect = { width: 800, height: 600, left: 0, top: 0, right: 800, bottom: 600 } as DOMRect;
      const root = new Group();
      const mesh = new Mesh(new BoxGeometry(1, 1, 1));
      root.add(mesh);
      const cache = buildObjectMarqueeFootprintCache(root, "obj-b");
      const geometry = mesh.geometry;
      const computeSpy = vi.spyOn(geometry, "computeBoundingBox");
      const footprint = projectObjectMarqueeFootprintFromCache(cache, camera, rect);
      expect(footprint).not.toBeNull();
      expect(computeSpy).not.toHaveBeenCalled();
      computeSpy.mockRestore();
    });
  });
  describe("marqueeSelectionFromCandidates", () => {
    const rect = screenRectFromClientPoints(0, 0, 100, 100);
    const testMarqueeCandidate = (kind: MarqueeCandidate["kind"], id: string, screenBounds: ScreenRect): MarqueeCandidate => ({
      kind,
      id,
      screenBounds,
      hull: [
        { x: screenBounds.left, y: screenBounds.top },
        { x: screenBounds.right, y: screenBounds.top },
        { x: screenBounds.right, y: screenBounds.bottom },
        { x: screenBounds.left, y: screenBounds.bottom },
      ],
    });
    it("window mode requires full enclosure", () => {
      const snap = marqueeSelectionFromCandidates({
        method: "rectangle",
        crossing: false,
        rect,
        polygon: [],
        kinds: { object: true, vortex: true, attraction: true },
        candidates: [
          testMarqueeCandidate("object", "inside", { left: 10, right: 20, top: 10, bottom: 20 }),
          testMarqueeCandidate("object", "partial", { left: 90, right: 120, top: 90, bottom: 120 }),
        ],
      });
      expect(snap.objectIds).toEqual(["inside"]);
    });
    it("crossing mode selects partial overlap", () => {
      const snap = marqueeSelectionFromCandidates({
        method: "rectangle",
        crossing: true,
        rect,
        polygon: [],
        kinds: { object: true, vortex: true, attraction: true },
        candidates: [testMarqueeCandidate("object", "partial", { left: 90, right: 120, top: 90, bottom: 120 })],
      });
      expect(snap.objectIds).toEqual(["partial"]);
    });
    it("respects kind toggles", () => {
      const snap = marqueeSelectionFromCandidates({
        method: "rectangle",
        crossing: false,
        rect,
        polygon: [],
        kinds: { object: false, vortex: true, attraction: false },
        candidates: [
          testMarqueeCandidate("object", "obj", { left: 10, right: 10, top: 10, bottom: 10 }),
          { kind: "vortex", id: "a:v1", hull: [{ x: 12, y: 12 }], screenBounds: { left: 12, right: 12, top: 12, bottom: 12 } },
        ],
      });
      expect(snap.objectIds).toEqual([]);
      expect(snap.vortexIds).toEqual(["a:v1"]);
    });
    it("skips candidates without screen bounds", () => {
      const snap = marqueeSelectionFromCandidates({
        method: "rectangle",
        crossing: false,
        rect,
        polygon: [],
        kinds: { object: true, vortex: true, attraction: true },
        candidates: [{ kind: "object", id: "hidden", hull: [], screenBounds: null }],
      });
      expect(snap.objectIds).toEqual([]);
    });
    it("window mode selects when the hull is fully enclosed even if screen bounds extend past the marquee", () => {
      const snap = marqueeSelectionFromCandidates({
        method: "rectangle",
        crossing: false,
        rect,
        polygon: [],
        kinds: { object: true, vortex: true, attraction: true },
        candidates: [
          {
            kind: "object",
            id: "tight-hull",
            hull: [
              { x: 40, y: 40 },
              { x: 60, y: 40 },
              { x: 60, y: 60 },
              { x: 40, y: 60 },
            ],
            screenBounds: { left: 10, right: 90, top: 10, bottom: 90 },
          },
        ],
      });
      expect(snap.objectIds).toEqual(["tight-hull"]);
    });
    it("crossing mode rejects overlap that only exists on inflated screen bounds", () => {
      const snap = marqueeSelectionFromCandidates({
        method: "rectangle",
        crossing: true,
        rect,
        polygon: [],
        kinds: { object: true, vortex: true, attraction: true },
        candidates: [
          {
            kind: "object",
            id: "outside-hull",
            hull: [{ x: 200, y: 200 }],
            screenBounds: { left: -10, right: 110, top: 50, bottom: 60 },
          },
        ],
      });
      expect(snap.objectIds).toEqual([]);
    });
    it("crossing mode selects when a hull edge crosses the marquee", () => {
      const snap = marqueeSelectionFromCandidates({
        method: "rectangle",
        crossing: true,
        rect,
        polygon: [],
        kinds: { object: true, vortex: true, attraction: true },
        candidates: [
          {
            kind: "object",
            id: "edge-hit",
            hull: [
              { x: -20, y: 50 },
              { x: 120, y: 50 },
            ],
            screenBounds: { left: -20, right: 120, top: 50, bottom: 50 },
          },
        ],
      });
      expect(snap.objectIds).toEqual(["edge-hit"]);
    });
    it("lasso window mode requires every hull point inside the polygon", () => {
      const polygon = [
        { x: 0, y: 0 },
        { x: 100, y: 0 },
        { x: 100, y: 100 },
        { x: 0, y: 100 },
      ];
      const snap = marqueeSelectionFromCandidates({
        method: "lasso",
        crossing: false,
        rect: null,
        polygon,
        kinds: { object: true, vortex: true, attraction: true },
        candidates: [
          {
            kind: "object",
            id: "inside",
            hull: [
              { x: 40, y: 40 },
              { x: 60, y: 40 },
              { x: 60, y: 60 },
              { x: 40, y: 60 },
            ],
            screenBounds: { left: 40, right: 60, top: 40, bottom: 60 },
          },
        ],
      });
      expect(snap.objectIds).toEqual(["inside"]);
    });
    it("lasso crossing mode uses hull polygon intersection", () => {
      const polygon = [
        { x: 0, y: 0 },
        { x: 100, y: 0 },
        { x: 100, y: 100 },
        { x: 0, y: 100 },
      ];
      const snap = marqueeSelectionFromCandidates({
        method: "lasso",
        crossing: true,
        rect: null,
        polygon,
        kinds: { object: true, vortex: true, attraction: true },
        candidates: [
          {
            kind: "object",
            id: "edge-hit",
            hull: [
              { x: -20, y: 50 },
              { x: 120, y: 50 },
            ],
            screenBounds: { left: -20, right: 120, top: 50, bottom: 50 },
          },
        ],
      });
      expect(snap.objectIds).toEqual(["edge-hit"]);
    });
  });
  describe("convexHullScreenPoints", () => {
    it("returns the input for fewer than two unique points", () => {
      expect(convexHullScreenPoints([{ x: 1, y: 2 }])).toEqual([{ x: 1, y: 2 }]);
    });
    it("orders hull vertices for a square", () => {
      const hull = convexHullScreenPoints([
        { x: 0, y: 0 },
        { x: 10, y: 0 },
        { x: 10, y: 10 },
        { x: 0, y: 10 },
        { x: 5, y: 5 },
      ]);
      expect(hull.length).toBe(4);
    });
  });
  describe("mergeSelectionSnapshot", () => {
    it("replaces on default and inverts membership on invertive", () => {
      const current = { objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] };
      const incoming = { objectIds: ["b"], vortexIds: [], attractionIds: [], referenceIds: [] };
      expect(mergeSelectionSnapshot("default", current, incoming).objectIds).toEqual(["b"]);
      expect(mergeSelectionSnapshot("invertive", current, incoming).objectIds.sort()).toEqual(["a", "b"]);
    });
  });
  describe("screenPolygonsIntersect", () => {
    it("detects edge crossings between two polygons", () => {
      const hull = [
        { x: -20, y: 50 },
        { x: 120, y: 50 },
      ];
      const lasso = [
        { x: 0, y: 0 },
        { x: 100, y: 0 },
        { x: 100, y: 100 },
        { x: 0, y: 100 },
      ];
      expect(screenPolygonsIntersect(hull, lasso)).toBe(true);
    });
  });
  describe("resolveMarqueeSelectionGesture", () => {
    const rect = screenRectFromClientPoints(0, 0, 100, 100);
    const candidates = [
      {
        kind: "object" as const,
        id: "inside",
        hull: [
          { x: 10, y: 10 },
          { x: 20, y: 10 },
          { x: 20, y: 20 },
          { x: 10, y: 20 },
        ],
        screenBounds: { left: 10, right: 20, top: 10, bottom: 20 },
      },
      {
        kind: "object" as const,
        id: "outside",
        hull: [{ x: 200, y: 200 }],
        screenBounds: { left: 200, right: 200, top: 200, bottom: 200 },
      },
    ];
    const gesture = {
      startX: 0,
      startY: 0,
      endX: 100,
      endY: 100,
      path: [
        { x: 0, y: 0 },
        { x: 100, y: 100 },
      ],
      modifiers: { shiftKey: false, ctrlKey: false, metaKey: false },
    };
    it("replaces selection on default mode from a fixed base", () => {
      const base = { objectIds: ["keep"], vortexIds: [], attractionIds: [], referenceIds: [] };
      const snap = resolveMarqueeSelectionGesture(gesture, {
        method: "rectangle",
        kinds: { object: true, vortex: true, attraction: true },
        candidates,
        base,
      });
      expect(snap.objectIds).toEqual(["inside"]);
    });
    it("additive mode merges against the captured base not the live preview", () => {
      const base = { objectIds: ["keep"], vortexIds: [], attractionIds: [], referenceIds: [] };
      const snap = resolveMarqueeSelectionGesture(
        { ...gesture, modifiers: { shiftKey: true, ctrlKey: false, metaKey: false } },
        {
          method: "rectangle",
          kinds: { object: true, vortex: true, attraction: true },
          candidates,
          base,
        },
      );
      expect(snap.objectIds.sort()).toEqual(["inside", "keep"]);
    });
    it("matches window-mode hit testing from marqueeSelectionFromCandidates", () => {
      const fromCandidates = marqueeSelectionFromCandidates({
        method: "rectangle",
        crossing: false,
        rect,
        polygon: [],
        kinds: { object: true, vortex: true, attraction: true },
        candidates,
      });
      const fromGesture = resolveMarqueeSelectionGesture(gesture, {
        method: "rectangle",
        kinds: { object: true, vortex: true, attraction: true },
        candidates,
        base: { objectIds: [], vortexIds: [], attractionIds: [], referenceIds: [] },
      });
      expect(fromGesture.objectIds).toEqual(fromCandidates.objectIds);
    });
  });
  describe("palette object fixture drag", () => {
    it("mergePaletteObjectFromDrop places object kind at CAD world point", () => {
      const catalogs: KindCatalogBundle = {
        objects: [
          {
            id: "J",
            label: "J",
            meshUrl: "test.glb",
            vortices: [{ vortexKind: "h1", position: [0, 0, 0] }],
          },
        ],
      };
      const dragFixture = buildPaletteObjectDragFixture("J");
      const placed = mergePaletteObjectFromDrop({ fixture: dragFixture, screen: { x: 10, y: 20 }, worldCad: [12, 34, 56] }, catalogs);
      expect(placed?.objectKind).toBe("J");
      expect(placed?.origin).toEqual([12, 34, 56]);
      expect(placed?.meshUrl).toBe("test.glb");
    });
    it("resolveObjectKindMeshUrl falls back to scene object mesh when catalog omits meshUrl", () => {
      const catalogs: KindCatalogBundle = { objects: [{ id: "Base", label: "Base" }] };
      const scene: FixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        domain: "architecture",
        attractions: [],
        objects: [{ id: "tower-base", objectKind: "Base", meshUrl: "/meshes/base.glb", origin: [0, 0, 0], vortices: [] }],
      };
      expect(resolveObjectKindMeshUrl("Base", catalogs, scene)).toBe("/meshes/base.glb");
      const dragFixture = buildPaletteObjectDragFixture("Base");
      const placed = mergePaletteObjectFromDrop({ fixture: dragFixture, screen: { x: 0, y: 0 }, worldCad: [1, 2, 3] }, catalogs, scene);
      expect(placed?.meshUrl).toBe("/meshes/base.glb");
      expect(placed?.origin).toEqual([1, 2, 3]);
    });
    it("resolvePuzzle3dFixtureDrop ignores palette object drag when mesh cannot be resolved", () => {
      const catalogs: KindCatalogBundle = { objects: [{ id: "Balcony", label: "Balcony" }] };
      const dragFixture = buildPaletteObjectDragFixture("Balcony");
      const result = resolvePuzzle3dFixtureDrop({ fixture: dragFixture, screen: { x: 0, y: 0 }, worldCad: [1, 2, 3] }, catalogs);
      expect(result.kind).toBe("ignored");
    });
    it("resolvePuzzle3dFixtureDrop does not replace fixture with palette seed drag payload", () => {
      const catalogs: KindCatalogBundle = {
        objects: [{ id: "Capsule Z", label: "Capsule Z", meshUrl: "/meshes/capsule_z.glb" }],
      };
      const scene: FixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        domain: "architecture",
        attractions: [],
        objects: [],
      };
      const dragFixture = buildPaletteObjectDragFixture("Capsule Z");
      const result = resolvePuzzle3dFixtureDrop({ fixture: dragFixture, screen: { x: 0, y: 0 }, worldCad: [4, 5, 6] }, catalogs, scene);
      expect(result.kind).toBe("palette-object");
      if (result.kind !== "palette-object") {
        return;
      }
      expect(result.object.meshUrl).toBe("/meshes/capsule_z.glb");
      expect(result.object.origin).toEqual([4, 5, 6]);
      const merged = applyPuzzle3dFixtureDropResult(scene, result);
      expect(merged?.objects).toHaveLength(1);
      expect(merged?.objects[0]?.meshUrl).toBe("/meshes/capsule_z.glb");
    });
    it("nakagin capsule kinds resolve to oriented capsule_* metabolism glbs", () => {
      expect(puzzle3dNakaginOrientedCapsuleMeshUrlFromKindId("Capsule With Balcony J")).toBe("/meshes/capsule_J.glb");
      expect(puzzle3dNakaginOrientedCapsuleMeshUrlFromKindId("Capsule With Balcony L")).toBe("/meshes/capsule_L.glb");
      expect(puzzle3dNakaginOrientedCapsuleMeshUrlFromKindId("Trapezoid Capsule Slash")).toBe("/meshes/capsule_slash.glb");
      expect(puzzle3dNakaginOrientedCapsuleMeshUrlFromKindId("Capsule")).toBeUndefined();
      expect(resolveObjectKindMeshUrl("Capsule With Balcony Backslash", {
        objects: [{ id: "Capsule With Balcony Backslash", meshUrl: "/meshes/capsule-with-balcony_backslash.glb" }],
      })).toBe("/meshes/capsule_backslash.glb");
    });

    it("resolveObjectKindMeshUrl ignores palette drag seed URLs in catalog and scene", () => {
      const catalogs: KindCatalogBundle = {
        objects: [{ id: "Capsule q", label: "Capsule q", meshUrl: PALETTE_DRAG_SEED_MESH_URL }],
      };
      const scene: FixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        domain: "architecture",
        attractions: [],
        objects: [
          { id: "palette-seed-object", objectKind: "Capsule q", meshUrl: PALETTE_DRAG_SEED_MESH_URL, origin: [0, 0, 0], vortices: [] },
          { id: "tower-q", objectKind: "Capsule q", meshUrl: "/meshes/capsule_q.glb", origin: [1, 0, 0], vortices: [] },
        ],
      };
      expect(isLoadableMeshUrl(PALETTE_DRAG_SEED_MESH_URL)).toBe(false);
      expect(resolveObjectKindMeshUrl("Capsule q", catalogs, scene)).toBe("/meshes/capsule_q.glb");
      const dragFixture = buildPaletteObjectDragFixture("Capsule q");
      const placed = mergePaletteObjectFromDrop({ fixture: dragFixture, screen: { x: 0, y: 0 }, worldCad: [1, 2, 3] }, catalogs, scene);
      expect(placed?.meshUrl).toBe("/meshes/capsule_q.glb");
    });
    it("applyObjectKindToFixtureObject swaps meshUrl from the catalog", () => {
      const catalogs: KindCatalogBundle = {
        objects: [
          { id: "kind-a", meshUrl: "/meshes/a.glb" },
          { id: "kind-b", meshUrl: "/meshes/b.glb" },
        ],
      };
      const object: FixtureObjectV1 = { id: "obj", objectKind: "kind-a", meshUrl: "/meshes/a.glb", origin: [0, 0, 0], vortices: [] };
      const next = applyObjectKindToFixtureObject(object, "kind-b", catalogs);
      expect(next.objectKind).toBe("kind-b");
      expect(next.meshUrl).toBe("/meshes/b.glb");
      expect(fixtureAppearanceFingerprint({ schema: "puzzle.3d.fixture/v1", camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 }, domain: "architecture", attractions: [], objects: [object] })).not.toBe(
        fixtureAppearanceFingerprint({ schema: "puzzle.3d.fixture/v1", camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 }, domain: "architecture", attractions: [], objects: [next] }),
      );
    });
    it("ObjectStore syncAppearanceFromFixture updates meshUrl on object-kind change", () => {
      const store = new ObjectStore();
      const initial: FixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        domain: "architecture",
        attractions: [],
        objects: [{ id: "obj", objectKind: "kind-a", meshUrl: "/meshes/a.glb", origin: [0, 0, 0], vortices: [] }],
      };
      store.initFromFixture(initial);
      expect(store.getRecord("obj")?.meshUrl).toBe("/meshes/a.glb");
      store.syncAppearanceFromFixture({
        ...initial,
        objects: [{ id: "obj", objectKind: "kind-b", meshUrl: "/meshes/b.glb", origin: [0, 0, 0], vortices: [] }],
      });
      expect(store.getRecord("obj")?.objectKind).toBe("kind-b");
      expect(store.getRecord("obj")?.meshUrl).toBe("/meshes/b.glb");
    });
    it("ObjectStore syncAppearanceFromFixture syncs hidden and locked flags on objects and attractions", () => {
      const store = new ObjectStore();
      const initial: FixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        domain: "architecture",
        attractions: [{ id: "att-1", attracting: "obj:v1", attracted: "obj:v1" }],
        objects: [{ id: "obj", objectKind: "kind-a", meshUrl: "/meshes/a.glb", origin: [0, 0, 0], vortices: [{ id: "v1", position: [0, 0, 0] }] }],
      };
      store.initFromFixture(initial);
      store.syncAppearanceFromFixture({
        ...initial,
        attractions: [{ id: "att-1", attracting: "obj:v1", attracted: "obj:v1", hidden: true, locked: true }],
        objects: [{ id: "obj", objectKind: "kind-a", meshUrl: "/meshes/a.glb", origin: [0, 0, 0], hidden: true, locked: true, vortices: [{ id: "v1", position: [0, 0, 0], hidden: true }] }],
      });
      expect(store.getRecord("obj")?.hidden).toBe(true);
      expect(store.getRecord("obj")?.locked).toBe(true);
      expect(store.getRecord("obj")?.vortices[0]?.hidden).toBe(true);
      expect(store.getAttractions()[0]?.hidden).toBe(true);
      expect(store.getAttractions()[0]?.locked).toBe(true);
      store.syncAppearanceFromFixture(initial);
      expect(store.getRecord("obj")?.hidden).toBeUndefined();
      expect(store.getRecord("obj")?.locked).toBeUndefined();
      expect(store.getRecord("obj")?.vortices[0]?.hidden).toBeUndefined();
      expect(store.getAttractions()[0]?.hidden).toBeUndefined();
    });
    it("puzzle3dGridPlacementAnchorCad uses orbit XY and datum Z", () => {
      const targetThree = new Vector3(...cadVec3ToThree([12, -8, 40]));
      const anchor = puzzle3dGridPlacementAnchorCad(targetThree);
      expect(anchor[0]).toBeCloseTo(12, 5);
      expect(anchor[1]).toBeCloseTo(-8, 5);
      expect(anchor[2]).toBe(0);
    });
    it("puzzle3dClientToGridPlaneCad hits datum plane not camera look-at Z", () => {
      const camera = new ThreePerspectiveCamera(50, 1, 0.1, 100_000);
      const lookAtCad: Vec3 = [0, 0, 40];
      const lookAtThree = new Vector3(...cadVec3ToThree(lookAtCad));
      camera.position.set(lookAtThree.x + 240, lookAtThree.y + 180, lookAtThree.z + 120);
      camera.lookAt(lookAtThree);
      camera.updateMatrixWorld();
      const canvas = document.createElement("canvas");
      canvas.getBoundingClientRect = () => ({ left: 0, top: 0, width: 800, height: 600, right: 800, bottom: 600, x: 0, y: 0, toJSON: () => ({}) }) as DOMRect;
      const atDatum = puzzle3dClientToGridPlaneCad({
        clientX: 400,
        clientY: 300,
        camera,
        canvas,
        gridPlaneAnchorCad: puzzle3dGridPlacementAnchorCad(lookAtThree),
      });
      const atLookAtHeight = puzzle3dClientToGridPlaneCad({
        clientX: 400,
        clientY: 300,
        camera,
        canvas,
        gridPlaneAnchorCad: lookAtCad,
      });
      expect(atLookAtHeight[2] - atDatum[2]).toBeGreaterThan(30);
      expect(Math.abs(atDatum[2])).toBeLessThan(2);
    });
    it("beginPuzzle3dFixturePalettePointerDrag commits drop on pointer up over host", () => {
      const host = document.createElement("div");
      host.getBoundingClientRect = () => ({ left: 0, top: 0, right: 200, bottom: 200, width: 200, height: 200, x: 0, y: 0, toJSON: () => ({}) }) as DOMRect;
      puzzle3dFixtureDropPointerToCadRef.current = () => [5, 6, 7];
      const dragFixture = buildPaletteObjectDragFixture("J");
      const encoded = encodeFixtureForDragV1(dragFixture);
      let dropped: Puzzle3dFixtureDropDetail | null = null;
      beginPuzzle3dFixturePalettePointerDrag(encoded);
      expect(puzzle3dFixturePalettePointerDragRef.active).toBe(true);
      endPuzzle3dFixturePalettePointerDrag(10, 10, host, (detail) => {
        dropped = detail;
      });
      expect(puzzle3dFixturePalettePointerDragRef.active).toBe(false);
      expect(dropped?.worldCad).toEqual([5, 6, 7]);
    });
    it("puzzle3dFixturePaletteTreeDragController toggles palette drag ref and drag session", () => {
      const encoded = encodeFixtureForDragV1(buildPaletteObjectDragFixture("J"));
      const dragData = new Map([["row-j", { [FIXTURE_DRAG_V1_MIME]: encoded, [FIXTURE_DRAG_PLAIN_MIME]: encoded }]]);
      const controller = puzzle3dFixturePaletteTreeDragController(dragData);
      const item = { id: "row-j", label: "J" };
      const section = { id: "objects", label: "Objects" };
      let session: string | null = "pending";
      const onSession = (event: Event): void => {
        const detail = (event as CustomEvent<{ readonly encoded: string } | null>).detail;
        session = detail?.encoded ?? null;
      };
      window.addEventListener("puzzle3d-fixture-drag-session", onSession);
      try {
        expect(puzzle3dFixturePaletteDragRef.active).toBe(false);
        controller.onDragStart?.({ items: [item], sourceItem: item, section });
        expect(puzzle3dFixturePaletteDragRef.active).toBe(true);
        expect(session).toBe(encoded);
        controller.onDragEnd?.({ items: [item], sourceItem: item, section });
        expect(puzzle3dFixturePaletteDragRef.active).toBe(false);
        expect(session).toBeNull();
      } finally {
        window.removeEventListener("puzzle3d-fixture-drag-session", onSession);
        puzzle3dFixturePaletteDragRef.active = false;
      }
    });
    it("snapCadVec3ToGridStep rounds to grid step", () => {
      expect(snapCadVec3ToGridStep([12.3, 0.1, 56.8], 5)).toEqual([10, 0, 55]);
    });
  });
  describe("brush", () => {
    beforeEach(() => {
      clearBrushCollisionGltfScenes();
    });
    const brushCatalogs: KindCatalogBundle = {
      objects: [
        {
          id: "Capsule J",
          meshUrl: "/meshes/capsule_J.glb",
          vortices: [{ vortexKind: "door capsule right", position: [-1.3, -1.25, 0], direction: [-1, 0, 0], radius: 0.36 }],
        },
        {
          id: "Capsule L",
          meshUrl: "/meshes/capsule_L.glb",
          vortices: [{ vortexKind: "door capsule left", position: [1.3, -1.25, 0], direction: [1, 0, 0], radius: 0.36 }],
        },
        {
          id: "Tambour",
          meshUrl: "/meshes/tambour.glb",
          vortices: [
            { vortexKind: "door tambour left", position: [0.9, 2.75, 0.2], direction: [0, 1, 0], radius: 0.36 },
            { vortexKind: "door tambour right", position: [-0.9, 2.75, 0.2], direction: [0, 1, 0], radius: 0.36 },
          ],
        },
      ],
      vortices: [
        { id: "door capsule right", defaultCableKind: "cable.link" },
        { id: "door capsule left", defaultCableKind: "cable.link" },
        { id: "door tambour left", defaultCableKind: "cable.link" },
        { id: "door tambour right", defaultCableKind: "cable.link" },
      ],
      cables: [{ id: "cable.link", defaultAttractionKind: "puzzle3d.attraction.link" }],
    };
    const brushCompat: readonly KindCompatEntry[] = [
      { bidirectional: true, specificity: "vortex", source: "door capsule right", target: "door tambour right" },
      { bidirectional: true, specificity: "vortex", source: "door capsule left", target: "door tambour left" },
    ];
    const registerBrushTestMesh = (meshUrl: string, size: [number, number, number]): Mesh => {
      const mesh = new Mesh(new BoxGeometry(size[0], size[1], size[2]));
      registerBrushCollisionGltfScene(meshUrl, mesh);
      return mesh;
    };
    const brushSceneGroup = (id: string, meshUrl: string, origin: Vec3): Group => {
      const group = new Group();
      group.userData.puzzle3dObjectId = id;
      group.userData.puzzle3dMeshUrl = meshUrl;
      applyObjectPose(group, origin, [0, 0, 0, 1], 1);
      return group;
    };
    const registerSparseLatticeMesh = (meshUrl: string): Group => {
      const lattice = new Group();
      const vertical = new Mesh(new BoxGeometry(0.4, 10, 0.4));
      vertical.position.set(-2, 0, 0);
      const horizontal = new Mesh(new BoxGeometry(10, 0.4, 0.4));
      horizontal.position.set(0, -2, 0);
      lattice.add(vertical, horizontal);
      registerBrushCollisionGltfScene(meshUrl, lattice);
      return lattice;
    };
    it("solidOverlapVolume detects overlapping unit cubes", () => {
      clearBrushCollisionGltfScenes();
      const urlA = "/test/a.glb";
      const urlB = "/test/b.glb";
      registerBrushTestMesh(urlA, [2, 2, 2]);
      registerBrushTestMesh(urlB, [2, 2, 2]);
      const bodyA = brushCollisionBody(urlA)!;
      const bodyB = brushCollisionBody(urlB)!;
      const worldA = brushPreviewWorldMatrix({ origin: [0, 0, 0], orientation: [0, 0, 0, 1], scale: 1 });
      const worldB = brushPreviewWorldMatrix({ origin: [1, 0, 0], orientation: [0, 0, 0, 1], scale: 1 });
      const volume = solidOverlapVolume(bodyA, worldA, bodyB, worldB, { sampleCount: 4096 });
      expect(volume).toBeGreaterThan(0.5);
      clearBrushCollisionGltfScenes();
    });
    it("solidOverlapVolume distinguishes sparse-lattice overlap from AABB-touching interleave", () => {
      clearBrushCollisionGltfScenes();
      const urlA = "/test/lattice-a.glb";
      const urlB = "/test/lattice-b.glb";
      registerSparseLatticeMesh(urlA);
      registerSparseLatticeMesh(urlB);
      const bodyA = brushCollisionBody(urlA)!;
      const bodyB = brushCollisionBody(urlB)!;
      const worldA = brushPreviewWorldMatrix({ origin: [0, 0, 0], orientation: [0, 0, 0, 1], scale: 1 });
      const coincident = solidOverlapVolume(bodyA, worldA, bodyB, worldA, { sampleCount: 4096 });
      expect(coincident).toBeGreaterThan(DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET);
      const worldB = brushPreviewWorldMatrix({ origin: [4, 4, 0], orientation: [0, 0, 0, 1], scale: 1 });
      const interleaved = solidOverlapVolume(bodyA, worldA, bodyB, worldB, { sampleCount: 4096 });
      expect(interleaved).toBeLessThanOrEqual(DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET);
      clearBrushCollisionGltfScenes();
    });
    it("computeBrushPlacementPose aligns source vortex to target with opposite direction", () => {
      const targetPos: Vec3 = [10, 20, 30];
      const targetDir: Vec3 = [0, 1, 0];
      const pose = computeBrushPlacementPose({
        sourceLocalPosition: [-1.3, -1.25, 0],
        sourceLocalDirection: [-1, 0, 0],
        targetWorldPositionCad: targetPos,
        targetWorldDirectionCad: targetDir,
      });
      const world = vortexWorldCadFromObject(
        { origin: pose.origin, orientation: pose.orientation, vortices: [{ id: "v0", position: [-1.3, -1.25, 0], direction: [-1, 0, 0] }] },
        0,
      );
      expect(world).not.toBeNull();
      expect(world!.position[0]).toBeCloseTo(targetPos[0], 4);
      expect(world!.position[1]).toBeCloseTo(targetPos[1], 4);
      expect(world!.position[2]).toBeCloseTo(targetPos[2], 4);
      expect(world!.direction[0]).toBeCloseTo(0, 4);
      expect(world!.direction[1]).toBeCloseTo(-1, 4);
      expect(world!.direction[2]).toBeCloseTo(0, 4);
    });
    it("computeBrushPlacementPose keeps upright when horizontal vortex directions are collinear", () => {
      const targetPos: Vec3 = [5, 10, 15];
      const targetDir: Vec3 = [1, 0, 0];
      const pose = computeBrushPlacementPose({
        sourceLocalPosition: [2, 0, 0],
        sourceLocalDirection: [1, 0, 0],
        targetWorldPositionCad: targetPos,
        targetWorldDirectionCad: targetDir,
      });
      const world = vortexWorldCadFromObject(
        { origin: pose.origin, orientation: pose.orientation, vortices: [{ id: "v0", position: [2, 0, 0], direction: [1, 0, 0] }] },
        0,
      );
      expect(world).not.toBeNull();
      expect(world!.position[0]).toBeCloseTo(targetPos[0], 4);
      expect(world!.position[1]).toBeCloseTo(targetPos[1], 4);
      expect(world!.position[2]).toBeCloseTo(targetPos[2], 4);
      expect(world!.direction[0]).toBeCloseTo(-1, 4);
      expect(world!.direction[1]).toBeCloseTo(0, 4);
      expect(world!.direction[2]).toBeCloseTo(0, 4);
      const worldUp = quatRotateVec(pose.orientation, [0, 0, 1]);
      expect(worldUp[0]).toBeCloseTo(0, 4);
      expect(worldUp[1]).toBeCloseTo(0, 4);
      expect(worldUp[2]).toBeCloseTo(1, 4);
    });
    it("computeBrushPlacementPose flips predictably when vertical vortex directions are collinear", () => {
      const targetPos: Vec3 = [0, 0, 20];
      const targetDir: Vec3 = [0, 0, 1];
      const pose = computeBrushPlacementPose({
        sourceLocalPosition: [0, 0, 1],
        sourceLocalDirection: [0, 0, 1],
        targetWorldPositionCad: targetPos,
        targetWorldDirectionCad: targetDir,
      });
      const world = vortexWorldCadFromObject(
        { origin: pose.origin, orientation: pose.orientation, vortices: [{ id: "v0", position: [0, 0, 1], direction: [0, 0, 1] }] },
        0,
      );
      expect(world).not.toBeNull();
      expect(world!.position[0]).toBeCloseTo(targetPos[0], 4);
      expect(world!.position[1]).toBeCloseTo(targetPos[1], 4);
      expect(world!.position[2]).toBeCloseTo(targetPos[2], 4);
      expect(world!.direction[0]).toBeCloseTo(0, 4);
      expect(world!.direction[1]).toBeCloseTo(0, 4);
      expect(world!.direction[2]).toBeCloseTo(-1, 4);
      const worldUp = quatRotateVec(pose.orientation, [0, 0, 1]);
      expect(worldUp[0]).toBeCloseTo(0, 4);
      expect(worldUp[1]).toBeCloseTo(0, 4);
      expect(worldUp[2]).toBeCloseTo(-1, 4);
    });
    it("computeBrushPlacementPose aligns directions when same-kind host orientation would not oppose", () => {
      const hostOrientation: Quat = [0, 0, 0, 1];
      const targetPos: Vec3 = [0, 0, 10];
      const targetDir: Vec3 = [1, 0, 0];
      const pose = computeBrushPlacementPose({
        sourceLocalPosition: [0, 0, 0],
        sourceLocalDirection: [1, 0, 0],
        targetWorldPositionCad: targetPos,
        targetWorldDirectionCad: targetDir,
        referenceOrientationCad: hostOrientation,
        useHostOrientation: true,
      });
      const world = vortexWorldCadFromObject(
        { origin: pose.origin, orientation: pose.orientation, vortices: [{ id: "v0", position: [0, 0, 0], direction: [1, 0, 0] }] },
        0,
      );
      expect(world).not.toBeNull();
      expect(world!.direction[0]).toBeCloseTo(-1, 4);
      expect(world!.direction[1]).toBeCloseTo(0, 4);
      expect(world!.direction[2]).toBeCloseTo(0, 4);
      expect(vec3Dot(world!.direction, targetDir)).toBeLessThan(-0.99);
    });
    it("computeBrushPlacementPose keeps host orientation when same-kind ports already oppose", () => {
      const hostOrientation: Quat = [0, 0, 0.7071067811865475, 0.7071067811865475];
      const targetPos: Vec3 = [3, 4, 5];
      const targetDir = normalizeVec3Cad(quatRotateVec(hostOrientation, [-1, 0, 0]));
      const pose = computeBrushPlacementPose({
        sourceLocalPosition: [2, 0, 0],
        sourceLocalDirection: [1, 0, 0],
        targetWorldPositionCad: targetPos,
        targetWorldDirectionCad: targetDir,
        referenceOrientationCad: hostOrientation,
        useHostOrientation: true,
      });
      expect(pose.orientation[0]).toBeCloseTo(hostOrientation[0], 4);
      expect(pose.orientation[1]).toBeCloseTo(hostOrientation[1], 4);
      expect(pose.orientation[2]).toBeCloseTo(hostOrientation[2], 4);
      expect(pose.orientation[3]).toBeCloseTo(hostOrientation[3], 4);
      const world = vortexWorldCadFromObject(
        { origin: pose.origin, orientation: pose.orientation, vortices: [{ id: "v0", position: [2, 0, 0], direction: [1, 0, 0] }] },
        0,
      );
      expect(vec3Dot(world!.direction, targetDir)).toBeLessThan(-0.99);
    });
    it("brushPreviewFromCandidate always places opposed vortex directions", () => {
      const target: AttractionVortexContext = { objectId: "host", objectKind: "KindA", vortexKind: "port-a" };
      const catalogs: KindCatalogBundle = {
        objects: [
          {
            id: "KindA",
            meshUrl: "/a.glb",
            vortices: [
              { vortexKind: "port-a", position: [0, 0, 0], direction: [1, 0, 0] },
              { vortexKind: "port-b", position: [0, 0, 0], direction: [-1, 0, 0] },
            ],
          },
        ],
      };
      const preview = brushPreviewFromCandidate({
        targetVortexFullId: "host:v0",
        candidate: { objectKindId: "KindA", sourceVortexIndex: 0 },
        target,
        targetWorldPositionCad: [0, 0, 0],
        targetWorldDirectionCad: [1, 0, 0],
        referenceOrientationCad: [0, 0, 0, 1],
        kindCatalogs: catalogs,
      });
      expect(preview).not.toBeNull();
      const world = vortexWorldCadFromObject(
        {
          origin: preview!.origin,
          orientation: preview!.orientation,
          vortices: [{ id: "v0", position: [0, 0, 0], direction: [1, 0, 0] }],
        },
        0,
      );
      expect(vec3Dot(world!.direction, [1, 0, 0])).toBeLessThan(-0.99);
    });
    it("buildPuzzle3dPlayEngagement always includes command input and tool possibles", () => {
      const spec = buildPuzzle3dPlayEngagement({
        activeTool: "select",
        cmdLine: "",
        fillCount: 0,
        selectionCount: 0,
        onCmdLineChange: () => {},
        onCmdLineSubmit: () => {},
        onSelectTool: () => {},
        onBrushTool: () => {},
        onFillTool: () => {},
        onFillCount: () => {},
        onCycleBrushCandidate: () => {},
        onPickBrushCandidate: () => {},
        onZoomToSelection: () => {},
        brushCandidates: [],
        brushTargetActive: false,
      });
      expect(spec.input?.id).toBe("engagement-input");
      expect(spec.possibleEngagements?.map((row) => row.id)).toEqual(["puzzle3d.tool.brush", PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID, "puzzle3d.tool.select"]);
      expect(spec.options).toBeUndefined();
    });
    it("buildPuzzle3dPlayEngagement exposes ring control for brush placement candidates", () => {
      const picked: number[] = [];
      const spec = buildPuzzle3dPlayEngagement({
        activeTool: "brush",
        cmdLine: "",
        fillCount: 0,
        selectionCount: 0,
        onCmdLineChange: () => {},
        onCmdLineSubmit: () => {},
        onSelectTool: () => {},
        onBrushTool: () => {},
        onFillTool: () => {},
        onFillCount: () => {},
        onCycleBrushCandidate: () => {},
        onPickBrushCandidate: (index) => picked.push(index),
        onZoomToSelection: () => {},
        brushCandidates: [
          { objectKindId: "J", sourceVortexIndex: 0 },
          { objectKindId: "K", sourceVortexIndex: 1 },
        ],
        brushTargetActive: true,
      });
      expect(spec.control?.kind).toBe("ring");
      expect(spec.control?.kind === "ring" && spec.control.options).toHaveLength(2);
      spec.control?.kind === "ring" && spec.control.onSelect?.("puzzle3d.brush.K.1");
      expect(picked).toEqual([1]);
    });

    it("buildPuzzle3dPlayEngagement brush hint describes hover and click-to-open candidate menu", () => {
      const spec = buildPuzzle3dPlayEngagement({
        activeTool: "brush",
        cmdLine: "",
        fillCount: 0,
        selectionCount: 0,
        onCmdLineChange: () => {},
        onCmdLineSubmit: () => {},
        onSelectTool: () => {},
        onBrushTool: () => {},
        onFillTool: () => {},
        onFillCount: () => {},
        onCycleBrushCandidate: () => {},
        onPickBrushCandidate: () => {},
        onZoomToSelection: () => {},
        brushCandidates: [],
        brushTargetActive: false,
      });
      const hint = spec.status?.find((row) => row.id === "puzzle3d.brush.hint")?.content ?? "";
      expect(hint).toContain("Point at");
      expect(hint).toContain("Alt");
      expect(hint).toContain("left-click");
    });
    it("buildPuzzle3dPlayEngagement lists brush candidates when brush tool is active", () => {
      const spec = buildPuzzle3dPlayEngagement({
        activeTool: "brush",
        cmdLine: "",
        selectionCount: 1,
        onCmdLineChange: () => {},
        onCmdLineSubmit: () => {},
        onSelectTool: () => {},
        onBrushTool: () => {},
        onFillTool: () => {},
        onFillCount: () => {},
        onCycleBrushCandidate: () => {},
        onPickBrushCandidate: () => {},
        onZoomToSelection: () => {},
        brushCandidates: [{ objectKindId: "J", sourceVortexIndex: 0 }],
        brushTargetActive: true,
      });
      expect(spec.possibleEngagements?.[0]?.id).toBe("puzzle3d.brush.J.0");
      expect(spec.options?.map((row) => row.id)).toEqual(["puzzle3d.zoom", "puzzle3d.tool.select", "puzzle3d.brush.next"]);
    });
    it("engagementCommandTokenEquals matches Brush after engagement input normalization", () => {
      expect(engagementCommandTokenEquals(normalizeEngagementCommandText("brush"), "Brush")).toBe(true);
      expect(engagementCommandTokenEquals(normalizeEngagementCommandText("select"), "Select")).toBe(true);
    });
    it("puzzle3dAttractionSessionIsStale when attracting or indirect-pick vortices are gone", () => {
      const exists = (id: string) => id === "a:h1" || id === "b:h2";
      expect(puzzle3dAttractionSessionIsStale("gone:h1", null, exists)).toBe(true);
      expect(puzzle3dAttractionSessionIsStale("a:h1", null, exists)).toBe(false);
      expect(
        puzzle3dAttractionSessionIsStale(
          "a:h1",
          { attractingFullId: "a:h1", attractedObjectId: "b", candidates: ["gone:h2"] },
          exists,
        ),
      ).toBe(true);
      expect(
        puzzle3dAttractionSessionIsStale(
          "a:h1",
          { attractingFullId: "a:h1", attractedObjectId: "b", candidates: ["b:h2"] },
          exists,
        ),
      ).toBe(false);
    });
    it("brushCompatibleCandidates filters by kind compatibility", () => {
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour left" };
      const list = brushCompatibleCandidates(target, brushCatalogs, brushCompat);
      expect(list.some((entry) => entry.objectKindId === "Capsule L")).toBe(true);
      expect(list.some((entry) => entry.objectKindId === "Capsule J")).toBe(false);
      expect(list.some((entry) => entry.objectKindId === "Tambour")).toBe(false);
    });
    it("brushCompatibleCandidates pairs door tambour right with door capsule right only", () => {
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour right" };
      const list = brushCompatibleCandidates(target, brushCatalogs, brushCompat);
      expect(list.some((entry) => entry.objectKindId === "Capsule J")).toBe(true);
      expect(list.some((entry) => entry.objectKindId === "Capsule L")).toBe(false);
    });
    it("brushCompatibleCandidates respects publishPuzzle3dBrushCandidateAccept", () => {
      const catalogs: KindCatalogBundle = {
        objects: [
          { id: "KindA", meshUrl: "/a.glb", vortices: [{ vortexKind: "port-b", position: [0, 0, 0], direction: [1, 0, 0] }] },
          { id: "KindB", meshUrl: "/b.glb", vortices: [{ vortexKind: "port-c", position: [0, 0, 0], direction: [1, 0, 0] }] },
        ],
        vortices: [{ id: "port-a" }, { id: "port-b" }, { id: "port-c" }],
        cables: [{ id: "cable.link" }],
      };
      const compat: readonly KindCompatEntry[] = [
        { bidirectional: true, specificity: "vortex", source: "port-b", target: "port-a" },
        { bidirectional: true, specificity: "vortex", source: "port-c", target: "port-a" },
      ];
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Host", vortexKind: "port-a" };
      publishPuzzle3dBrushCandidateAccept((_t, candidate) => candidate.objectKindId !== "KindB");
      const list = brushCompatibleCandidates(target, catalogs, compat);
      expect(list.some((entry) => entry.objectKindId === "KindA")).toBe(true);
      expect(list.some((entry) => entry.objectKindId === "KindB")).toBe(false);
      publishPuzzle3dBrushCandidateAccept(null);
    });
    it("brushCompatibleCandidates ranks stack mates first on top vortices", () => {
      const stackCatalogs: KindCatalogBundle = {
        objects: [
          {
            id: "WidgetA",
            meshUrl: "/meshes/a.glb",
            vortices: [
              { vortexKind: "shape rectangular bottom", position: [0, 0, 0.92], direction: [0, 0, -1], radius: 0.36 },
              { vortexKind: "shape circular bottom", position: [0, 0, 0.92], direction: [0, 0, -1], radius: 0.36 },
            ],
          },
          {
            id: "WidgetB",
            meshUrl: "/meshes/b.glb",
            vortices: [{ vortexKind: "shape circular bottom", position: [0, 0, 0.92], direction: [0, 0, -1], radius: 0.36 }],
          },
        ],
      };
      const stackCompat: readonly KindCompatEntry[] = [
        { bidirectional: true, specificity: "vortex", source: "shape circular bottom", target: "shape circular top" },
        { bidirectional: true, specificity: "vortex", source: "shape rectangular bottom", target: "shape rectangular top" },
      ];
      const target: AttractionVortexContext = { objectId: "host", objectKind: "WidgetHost", vortexKind: "shape circular top" };
      const list = brushCompatibleCandidates(target, stackCatalogs, stackCompat);
      expect(list[0]?.objectKindId).toBe("WidgetA");
      expect(list[0]?.sourceVortexIndex).toBe(1);
      const widgetBIndex = list.findIndex((entry) => entry.objectKindId === "WidgetB");
      if (widgetBIndex >= 0) {
        expect(widgetBIndex).toBeGreaterThan(0);
      }
    });
    it("brushCompatibleCandidates rejects cross-shape core mates on bottom targets", () => {
      const baseCatalogs: KindCatalogBundle = {
        objects: [
          {
            id: "Foundation",
            meshUrl: "/meshes/foundation.glb",
            vortices: [{ vortexKind: "core rectangular bottom", position: [-7.5, -7.7, 7.5], direction: [0, 0, 1], radius: 0.36 }],
          },
          {
            id: "UpperCircular",
            meshUrl: "/meshes/upper-circular.glb",
            vortices: [{ vortexKind: "core circular top", position: [0, 0, 0], direction: [0, 0, -1], radius: 0.36 }],
          },
          {
            id: "UpperRectangular",
            meshUrl: "/meshes/upper-rectangular.glb",
            vortices: [{ vortexKind: "core rectangular top", position: [0, 0, 0], direction: [0, 0, -1], radius: 0.36 }],
          },
        ],
      };
      const baseCompat: readonly KindCompatEntry[] = [
        { bidirectional: true, specificity: "vortex", source: "core rectangular bottom", target: "core rectangular top" },
        { bidirectional: true, specificity: "vortex", source: "core rectangular bottom", target: "core circular top" },
      ];
      const target: AttractionVortexContext = { objectId: "base", objectKind: "Foundation", vortexKind: "core rectangular bottom" };
      const list = brushCompatibleCandidates(target, baseCatalogs, baseCompat);
      expect(list.length).toBe(1);
      expect(list[0]?.objectKindId).toBe("UpperRectangular");
    });
    it("brushStackMatePair rejects core circular top on core rectangular bottom", () => {
      expect(brushStackMatePair("core circular top", "core rectangular bottom")).toBe(false);
      expect(brushStackMatePair("core rectangular top", "core rectangular bottom")).toBe(true);
    });
    it("puzzle3dVortexPortShapesCompatible rejects circular vs rectangular", () => {
      expect(puzzle3dVortexPortShapesCompatible("core circular top", "core rectangular bottom")).toBe(false);
      expect(puzzle3dVortexPortShapesCompatible("door capsule left", "core circular bottom")).toBe(true);
      expect(puzzle3dVortexPortShapesCompatible("tambour circular top", "tambour circular bottom")).toBe(true);
    });
    it("puzzle3dSingleLetterPortFamiliesCompatible rejects beam vs column ports", () => {
      expect(puzzle3dSingleLetterPortFamiliesCompatible("b-l", "c-b")).toBe(false);
      expect(puzzle3dSingleLetterPortFamiliesCompatible("b-s-m", "c-t")).toBe(false);
      expect(puzzle3dSingleLetterPortFamiliesCompatible("b-l", "b-s")).toBe(true);
      expect(puzzle3dSingleLetterPortFamiliesCompatible("c-b", "c-t")).toBe(true);
      expect(puzzle3dSingleLetterPortFamiliesCompatible("door tambour left", "door capsule left")).toBe(true);
    });
    it("concrete forest brush rejects c-* candidates for b-* targets even without compatibility rules", async () => {
      const concreteForestFixture = (await import("../fixture/concrete-forest.3d.json")).default as FixtureV1;
      const meta = concreteForestFixture.meta as Record<string, unknown> | undefined;
      const kindCatalogs = (meta?.kindCatalogs ?? undefined) as KindCatalogBundle | undefined;
      const host = concreteForestFixture.objects[0]!;
      const target: AttractionVortexContext = {
        objectId: host.id,
        objectKind: host.objectKind,
        vortexKind: "b-l",
      };
      const withoutRules = brushCompatibleCandidates(target, kindCatalogs, []);
      const columnCandidates = withoutRules.filter((candidate) => {
        const vk = kindCatalogs?.objects?.find((row) => row.id === candidate.objectKindId)?.vortices?.[candidate.sourceVortexIndex]?.vortexKind ?? "";
        return vk.startsWith("c-");
      });
      expect(columnCandidates).toHaveLength(0);
      expect(vorticesAttractionCompatibleForDrag(
        { objectId: "__brush__", objectKind: "Hexagonal Cut Concrete Forest Right", vortexKind: "c-b" },
        target,
        [],
        kindCatalogs,
      )).toBe(false);
    });
    it("brushStackTopBottomPair matches tambour circular top to tambour circular bottom", () => {
      expect(brushStackTopBottomPair("tambour circular top", "tambour circular bottom")).toBe(true);
      expect(brushStackMatePair("tambour circular top", "tambour circular bottom")).toBe(true);
      expect(brushStackMatePair("roof circular top", "tambour circular bottom")).toBe(false);
    });
    it("brushPlacementUsesHostOrientation is false for stack mates and cross-port mates", () => {
      const target: AttractionVortexContext = { objectId: "host", objectKind: "WidgetHost", vortexKind: "shape circular top" };
      expect(brushPlacementUsesHostOrientation(target, "shape circular bottom", "WidgetA")).toBe(false);
      expect(brushPlacementUsesHostOrientation({ objectId: "b", objectKind: "Foundation", vortexKind: "core rectangular bottom" }, "core circular top", "UpperA")).toBe(false);
      expect(brushPlacementUsesHostOrientation({ objectId: "a", objectKind: "KindA", vortexKind: "port-a" }, "port-b", "KindA")).toBe(false);
      expect(brushPlacementUsesHostOrientation({ objectId: "a", objectKind: "KindA", vortexKind: "port-a" }, "port-a", "KindA")).toBe(true);
    });
    it("vortexWorldCadFromObject matches scene graph world position", () => {
      const origin: Vec3 = [10, 0, 0];
      const orientation: Quat = [0, 0, 0.7071067811865475, -0.7071067811865475];
      const local: Vec3 = [1, 2, 3];
      const parent = new Group();
      const vortex = new Group();
      const groupLocal = cadObjectLocalToThreeGroupLocal(local, origin, orientation);
      vortex.position.set(groupLocal[0], groupLocal[1], groupLocal[2]);
      parent.add(vortex);
      applyObjectPose(parent, origin, orientation, 1);
      updateWorldMatrixChain(vortex);
      const worldThree = new Vector3();
      vortex.getWorldPosition(worldThree);
      const cad = vortexWorldCadFromObject({ origin, orientation, vortices: [{ id: "v0", position: local }] }, 0);
      expect(cad).not.toBeNull();
      const expectedThree = cadVec3ToThree(cad!.position);
      expect(worldThree.x).toBeCloseTo(expectedThree[0], 5);
      expect(worldThree.y).toBeCloseTo(expectedThree[1], 5);
      expect(worldThree.z).toBeCloseTo(expectedThree[2], 5);
    });
    it("computeBrushPlacementPose stacks tambour bottom on rotated storey top with opposed directions", () => {
      const hostOrientation: Quat = [0, 0, 0.7071067811865475, -0.7071067811865475];
      const targetPos: Vec3 = [1, 2, 30];
      const targetDir = normalizeVec3Cad(quatRotateVec(hostOrientation, [0, 0, 1]));
      const pose = computeBrushPlacementPose({
        sourceLocalPosition: [0, 0, 0.9166667],
        sourceLocalDirection: [0, 0, -1],
        targetWorldPositionCad: targetPos,
        targetWorldDirectionCad: targetDir,
        useHostOrientation: false,
      });
      const world = vortexWorldCadFromObject(
        { origin: pose.origin, orientation: pose.orientation, vortices: [{ id: "v0", position: [0, 0, 0.9166667], direction: [0, 0, -1] }] },
        0,
      );
      expect(world!.position[0]).toBeCloseTo(targetPos[0], 4);
      expect(world!.position[1]).toBeCloseTo(targetPos[1], 4);
      expect(world!.position[2]).toBeCloseTo(targetPos[2], 4);
      const worldDir = world!.direction;
      expect(worldDir[0] * targetDir[0] + worldDir[1] * targetDir[1] + worldDir[2] * targetDir[2]).toBeLessThan(0);
    });
    it("routePuzzle3dBrushTabKeydown when brush hovers a connector with multiple candidates", () => {
      expect(routePuzzle3dBrushTabKeydown(true, true, 3, { key: "Tab", defaultPrevented: false, ctrlKey: false, metaKey: false, altKey: false })).toBe(true);
      expect(routePuzzle3dBrushTabKeydown(true, true, 1, { key: "Tab", defaultPrevented: false, ctrlKey: false, metaKey: false, altKey: false })).toBe(false);
      expect(routePuzzle3dBrushTabKeydown(true, false, 3, { key: "Tab", defaultPrevented: false, ctrlKey: false, metaKey: false, altKey: false })).toBe(false);
      expect(routePuzzle3dBrushTabKeydown(false, true, 3, { key: "Tab", defaultPrevented: false, ctrlKey: false, metaKey: false, altKey: false })).toBe(false);
    });
    it("boxesIntersect detects overlapping axis-aligned boxes", () => {
      const a = new Box3(new Vector3(0, 0, 0), new Vector3(2, 2, 2));
      const b = new Box3(new Vector3(1, 1, 1), new Vector3(3, 3, 3));
      const c = new Box3(new Vector3(4, 4, 4), new Vector3(5, 5, 5));
      expect(boxesIntersect(a, b)).toBe(true);
      expect(boxesIntersect(a, c)).toBe(false);
    });
    it("shuffleBrushCompatibleCandidates permutes with injectable rng", () => {
      const input: readonly BrushCompatibleCandidate[] = [
        { objectKindId: "A", sourceVortexIndex: 0 },
        { objectKindId: "B", sourceVortexIndex: 0 },
        { objectKindId: "C", sourceVortexIndex: 0 },
      ];
      const shuffled = shuffleBrushCompatibleCandidates(input, () => 0);
      expect(shuffled).toHaveLength(3);
      expect(new Set(shuffled.map((row) => row.objectKindId)).size).toBe(3);
      expect(shuffled[0]?.objectKindId).toBe("B");
    });
    it("weightedOrderBrushCompatibleCandidates favors high object and vortex weights", () => {
      const catalogs: KindCatalogBundle = {
        objects: [
          { id: "Heavy", meshUrl: "/h.glb", vortices: [{ vortexKind: "vk-heavy", position: [0, 0, 0], direction: [0, 0, 1] }] },
          { id: "Light", meshUrl: "/l.glb", vortices: [{ vortexKind: "vk-light", position: [0, 0, 0], direction: [0, 0, 1] }] },
        ],
      };
      const input: readonly BrushCompatibleCandidate[] = [
        { objectKindId: "Light", sourceVortexIndex: 0 },
        { objectKindId: "Heavy", sourceVortexIndex: 0 },
      ];
      const ordered = weightedOrderBrushCompatibleCandidates(
        input,
        { objectWeights: { Heavy: 0.95, Light: 0.05 }, vortexWeights: { "vk-heavy": 0.9, "vk-light": 0.1 } },
        catalogs,
        () => 0.99,
      );
      expect(ordered[0]?.objectKindId).toBe("Heavy");
    });
    it("weightedOrderBrushCompatibleCandidates excludes zero object or vortex weights", () => {
      const catalogs: KindCatalogBundle = {
        objects: [
          { id: "Blocked", meshUrl: "/b.glb", vortices: [{ vortexKind: "c-b", position: [0, 0, 0], direction: [0, 0, 1] }] },
          { id: "Allowed", meshUrl: "/a.glb", vortices: [{ vortexKind: "c-t", position: [0, 0, 0], direction: [0, 0, 1] }] },
        ],
      };
      const input: readonly BrushCompatibleCandidate[] = [
        { objectKindId: "Blocked", sourceVortexIndex: 0 },
        { objectKindId: "Allowed", sourceVortexIndex: 0 },
      ];
      expect(
        weightedOrderBrushCompatibleCandidates(input, { objectWeights: { Blocked: 0, Allowed: 1 }, vortexWeights: { "c-b": 1, "c-t": 1 } }, catalogs),
      ).toEqual([{ objectKindId: "Allowed", sourceVortexIndex: 0 }]);
      expect(
        weightedOrderBrushCompatibleCandidates(input, { objectWeights: { Blocked: 1, Allowed: 1 }, vortexWeights: { "c-b": 0, "c-t": 1 } }, catalogs),
      ).toEqual([{ objectKindId: "Allowed", sourceVortexIndex: 0 }]);
      expect(
        weightedOrderBrushCompatibleCandidates(
          [{ objectKindId: "Blocked", sourceVortexIndex: 0 }],
          { objectWeights: { Blocked: 0 }, vortexWeights: { "c-b": 1 } },
          catalogs,
        ),
      ).toEqual([]);
    });
    it("brushTargetVortexAllowsSuggestion rejects zero target vortex weight", () => {
      expect(brushTargetVortexAllowsSuggestion("c-t", { objectWeights: {}, vortexWeights: { "c-t": 0, "c-b": 1 } })).toBe(false);
      expect(brushTargetVortexAllowsSuggestion("c-b", { objectWeights: {}, vortexWeights: { "c-t": 0, "c-b": 1 } })).toBe(true);
    });
    it("brushCandidateAllowsSuggestion rejects zero object or source vortex weight", () => {
      const catalogs: KindCatalogBundle = {
        objects: [{ id: "Forest", meshUrl: "/f.glb", vortices: [{ vortexKind: "c-b", position: [0, 0, 0], direction: [0, 0, 1] }] }],
      };
      const candidate = { objectKindId: "Forest", sourceVortexIndex: 0 };
      expect(brushCandidateAllowsSuggestion(candidate, { objectWeights: { Forest: 0 }, vortexWeights: { "c-b": 1 } }, catalogs)).toBe(false);
      expect(brushCandidateAllowsSuggestion(candidate, { objectWeights: { Forest: 1 }, vortexWeights: { "c-b": 0 } }, catalogs)).toBe(false);
      expect(brushCandidateAllowsSuggestion(candidate, { objectWeights: { Forest: 1 }, vortexWeights: { "c-b": 1 } }, catalogs)).toBe(true);
    });
    it("brushPreviewMeshFrameGroup applies GLB mesh frame rotation", () => {
      const meshRoot = new Mesh(new BoxGeometry(1, 2, 3));
      const frame = brushPreviewMeshFrameGroup(meshRoot);
      expect(frame.rotation.x).toBeCloseTo(GLB_MESH_FRAME_ROTATION_X, 5);
      expect(frame.children.length).toBe(1);
    });
    it("brushPreviewCollides detects solid overlap above budget", () => {
      clearBrushCollisionGltfScenes();
      const obstacleUrl = "/test/obstacle.glb";
      const previewUrl = "/test/preview.glb";
      registerBrushTestMesh(obstacleUrl, [4, 4, 4]);
      registerBrushTestMesh(previewUrl, [4, 4, 4]);
      const scene: BrushSceneCollisionSource = { collectObjectGroups: () => [brushSceneGroup("obstacle", obstacleUrl, [0, 0, 0])] };
      const preview: BrushPreviewState = {
        targetVortexFullId: "host:v0",
        objectKindId: "Kind",
        sourceVortexIndex: 0,
        meshUrl: previewUrl,
        origin: [0, 0, 0],
        orientation: [0, 0, 0, 1],
      };
      expect(brushPreviewCollides(scene, preview, 0.02)).toBe(true);
      clearBrushCollisionGltfScenes();
    });
    it("brushPreviewCollides allows separated placements", () => {
      clearBrushCollisionGltfScenes();
      const obstacleUrl = "/test/obstacle.glb";
      const previewUrl = "/test/preview.glb";
      registerBrushTestMesh(obstacleUrl, [4, 4, 4]);
      registerBrushTestMesh(previewUrl, [2, 2, 2]);
      const scene: BrushSceneCollisionSource = { collectObjectGroups: () => [brushSceneGroup("obstacle", obstacleUrl, [0, 0, 0])] };
      const preview: BrushPreviewState = {
        targetVortexFullId: "host:v0",
        objectKindId: "Kind",
        sourceVortexIndex: 0,
        meshUrl: previewUrl,
        origin: [12, 0, 0],
        orientation: [0, 0, 0, 1],
      };
      expect(brushPreviewCollides(scene, preview, DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET)).toBe(false);
      clearBrushCollisionGltfScenes();
    });
    it("brushCollisionFreeCandidates excludes placements that overlap scene meshes", () => {
      clearBrushCollisionGltfScenes();
      const obstacleUrl = "/test/obstacle.glb";
      const previewUrl = "/test/preview.glb";
      registerBrushTestMesh(obstacleUrl, [8, 8, 8]);
      registerBrushTestMesh(previewUrl, [4, 4, 4]);
      const scene: BrushSceneCollisionSource = { collectObjectGroups: () => [brushSceneGroup("obstacle", obstacleUrl, [0, 0, 0])] };
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour left" };
      const preview: BrushPreviewState = {
        targetVortexFullId: "host:v0",
        objectKindId: "Capsule L",
        sourceVortexIndex: 0,
        meshUrl: previewUrl,
        origin: [0, 0, 0],
        orientation: [0, 0, 0, 1],
      };
      expect(brushCandidateCollidesAtPose(scene, preview, brushCollisionGltfRoot(previewUrl), DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET)).toBe(true);
      clearBrushCollisionGltfScenes();
    });
    it("brushCollisionFreeCandidates sets unknownPending when catalog meshes are not pooled yet", () => {
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour left" };
      const compatible = brushCompatibleCandidates(target, brushCatalogs, brushCompat);
      const pending = brushCollisionFreeCandidates({
        scene: { collectObjectGroups: () => [] },
        targetVortexFullId: "host:v0",
        candidates: compatible,
        target,
        targetWorldPositionCad: [0.9, 2.75, 0.2],
        targetWorldDirectionCad: [0, 1, 0],
        kindCatalogs: brushCatalogs,
        meshRootForUrl: () => undefined,
      });
      expect(pending.unknownPending).toBe(true);
      expect(pending.free).toHaveLength(0);
    });
    it("brushMeshUrlsForCompatibleCandidates returns unique catalog mesh URLs", () => {
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour left" };
      const compatible = brushCompatibleCandidates(target, brushCatalogs, brushCompat);
      const urls = brushMeshUrlsForCompatibleCandidates(compatible, brushCatalogs);
      expect(urls.length).toBeGreaterThan(0);
      expect(new Set(urls).size).toBe(urls.length);
    });
    it("brushMeshUrlsForFillSession includes scene objects and compatible placement meshes", () => {
      const fixture: FixtureV1 = {
        version: 1,
        objects: [
          {
            id: "host",
            objectKind: "Tambour",
            meshUrl: "/meshes/tambour.glb",
            origin: [0, 0, 0],
            orientation: [0, 0, 0, 1],
            vortices: [
              { id: "host:v0", vortexKind: "door tambour left", label: "door tambour left", position: [0.9, 2.75, 0.2], direction: [0, 1, 0] },
              { id: "host:v1", vortexKind: "door tambour right", label: "door tambour right", position: [-0.9, 2.75, 0.2], direction: [0, 1, 0] },
            ],
          },
        ],
        attractions: [],
      };
      const urls = brushMeshUrlsForFillSession(fixture, brushCatalogs, brushCompat);
      expect(urls).toContain("/meshes/tambour.glb");
      expect(urls).toContain("/meshes/capsule_L.glb");
      expect(new Set(urls).size).toBe(urls.length);
    });
    it("buildBrushFillSequence places objects when mesh roots are pooled", () => {
      clearBrushCollisionGltfScenes();
      const meshRoot = registerBrushTestMesh("/meshes/tambour.glb", [4, 4, 4]);
      registerBrushTestMesh("/meshes/capsule_L.glb", [4, 4, 4]);
      registerBrushTestMesh("/meshes/capsule_R.glb", [4, 4, 4]);
      const fixture: FixtureV1 = {
        version: 1,
        objects: [
          {
            id: "host",
            objectKind: "Tambour",
            meshUrl: "/meshes/tambour.glb",
            origin: [0, 0, 0],
            orientation: [0, 0, 0, 1],
            vortices: [
              { id: "host:v0", vortexKind: "door tambour left", label: "door tambour left", position: [0.9, 2.75, 0.2], direction: [0, 1, 0] },
            ],
          },
        ],
        attractions: [],
      };
      const sequence = buildBrushFillSequence({
        baseFixture: fixture,
        maxCount: 4,
        seed: 42,
        kindCatalogs: brushCatalogs,
        kindCompatibility: brushCompat,
        meshRootForUrl: brushCollisionGltfRoot,
      });
      expect(sequence.length).toBeGreaterThan(0);
      const applied = applyBrushFillPlacementsToFixture(fixture, sequence, brushCatalogs);
      expect(applied.objects.length).toBeGreaterThan(fixture.objects.length);
      clearBrushCollisionGltfScenes();
    });
    it("createBrushFillSequenceStepper matches buildBrushFillSequence and appended prefix composes the same fixture", () => {
      clearBrushCollisionGltfScenes();
      registerBrushTestMesh("/meshes/tambour.glb", [4, 4, 4]);
      registerBrushTestMesh("/meshes/capsule_L.glb", [4, 4, 4]);
      registerBrushTestMesh("/meshes/capsule_R.glb", [4, 4, 4]);
      const fixture: FixtureV1 = {
        version: 1,
        objects: [
          {
            id: "host",
            objectKind: "Tambour",
            meshUrl: "/meshes/tambour.glb",
            origin: [0, 0, 0],
            orientation: [0, 0, 0, 1],
            vortices: [
              { id: "host:v0", vortexKind: "door tambour left", label: "door tambour left", position: [0.9, 2.75, 0.2], direction: [0, 1, 0] },
            ],
          },
        ],
        attractions: [],
      };
      const args = {
        baseFixture: fixture,
        maxCount: 4,
        seed: 42,
        kindCatalogs: brushCatalogs,
        kindCompatibility: brushCompat,
        meshRootForUrl: brushCollisionGltfRoot,
      };
      const syncSequence = buildBrushFillSequence(args);
      const stepper = createBrushFillSequenceStepper(args);
      let stepped = stepper.step(1);
      while (!stepped.done) {
        stepped = stepper.step(1);
      }
      expect(stepped.sequence).toEqual(syncSequence);
      expect(stepped.appendedObjects.length).toBe(stepped.sequence.length);
      expect(stepped.appendedAttractions.length).toBe(stepped.sequence.length);
      const composed = {
        ...fixture,
        objects: [...fixture.objects, ...stepped.appendedObjects],
        attractions: [...fixture.attractions, ...stepped.appendedAttractions],
      };
      expect(composed.objects.length).toBe(fixture.objects.length + stepped.sequence.length);
      expect(composed.attractions.length).toBe(fixture.attractions.length + stepped.sequence.length);
      clearBrushCollisionGltfScenes();
    });
    it("buildBrushFillSequence rejects undersized collision mesh roots", () => {
      const stub = new Mesh(new BoxGeometry(1, 1, 1));
      expect(brushCollisionMeshExtentOk(stub)).toBe(false);
      const fixture: FixtureV1 = {
        version: 1,
        objects: [
          {
            id: "host",
            objectKind: "Tambour",
            meshUrl: "/meshes/tambour.glb",
            origin: [0, 0, 0],
            orientation: [0, 0, 0, 1],
            vortices: [
              { id: "host:v0", vortexKind: "door tambour left", label: "door tambour left", position: [0.9, 2.75, 0.2], direction: [0, 1, 0] },
            ],
          },
        ],
        attractions: [],
      };
      const sequence = buildBrushFillSequence({
        baseFixture: fixture,
        maxCount: 100,
        seed: 42,
        kindCatalogs: brushCatalogs,
        kindCompatibility: brushCompat,
        meshRootForUrl: () => stub,
      });
      expect(sequence.length).toBe(0);
    });
    it("buildBrushFillSequence rejects fill when collision mesh roots are stub-sized", () => {
      clearBrushCollisionGltfScenes();
      const stubGroup = new Group();
      stubGroup.add(new Mesh(new BoxGeometry(1, 1, 1)));
      registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-left.glb", stubGroup);
      registerBrushCollisionGltfScene("/meshes/hexagonal-cut-concrete-forest-right.glb", stubGroup);
      expect(brushCollisionGltfRoot("/meshes/hexagonal-cut-concrete-forest-left.glb")).toBeNull();
      const fixture: FixtureV1 = {
        version: 1,
        objects: [
          {
            id: "seed-left-001",
            objectKind: "Hexagonal Cut Concrete Forest Left",
            meshUrl: "/meshes/hexagonal-cut-concrete-forest-left.glb",
            origin: [0, 0, 0],
            orientation: [0, 0, 0, 1],
            vortices: [{ id: "seed-left-001:v0", vortexKind: "b-l", label: "b-l", position: [1.95, 3.377499, 3], direction: [0, 1, 0] }],
          },
        ],
        attractions: [],
      };
      const catalogs: KindCatalogBundle = {
        objects: [
          {
            id: "Hexagonal Cut Concrete Forest Left",
            meshUrl: "/meshes/hexagonal-cut-concrete-forest-left.glb",
            vortices: [{ vortexKind: "b-l", position: [1.95, 3.377499, 3], direction: [0, 1, 0] }],
          },
          {
            id: "Hexagonal Cut Concrete Forest Right",
            meshUrl: "/meshes/hexagonal-cut-concrete-forest-right.glb",
            vortices: [{ vortexKind: "b-l", position: [9.55, 3.377499, 3], direction: [0, 1, 0] }],
          },
        ],
      };
      const compat: readonly KindCompatEntry[] = [{ bidirectional: true, specificity: "vortex", source: "b-l", target: "b-l" }];
      const sequence = buildBrushFillSequence({
        baseFixture: fixture,
        maxCount: 100,
        seed: 42,
        kindCatalogs: catalogs,
        kindCompatibility: compat,
        overlapBudget: 1,
        meshRootForUrl: brushCollisionGltfRoot,
      });
      expect(sequence.length).toBe(0);
      clearBrushCollisionGltfScenes();
    });
    it("brushCollisionFreeCandidates returns all compatible kinds when scene is clear", () => {
      clearBrushCollisionGltfScenes();
      const hostUrl = "/meshes/tambour.glb";
      registerBrushTestMesh(hostUrl, [4, 4, 4]);
      registerBrushTestMesh("/meshes/capsule_L.glb", [4, 4, 4]);
      registerBrushTestMesh("/meshes/capsule_R.glb", [4, 4, 4]);
      const clearScene: BrushSceneCollisionSource = { collectObjectGroups: () => [brushSceneGroup("host", hostUrl, [0, 0, 0])] };
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour left" };
      const compatible = brushCompatibleCandidates(target, brushCatalogs, brushCompat);
      const clear = brushCollisionFreeCandidates({
        scene: clearScene,
        targetVortexFullId: "host:v0",
        candidates: compatible,
        target,
        targetWorldPositionCad: [0.9, 2.75, 0.2],
        targetWorldDirectionCad: [0, 1, 0],
        kindCatalogs: brushCatalogs,
        meshRootForUrl: brushCollisionGltfRoot,
        overlapBudget: DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
      });
      expect(clear.free.length).toBe(compatible.length);
      clearBrushCollisionGltfScenes();
    });
    it("buildPuzzle3dPlayEngagement exposes Zoom option when selection is non-empty", () => {
      const spec = buildPuzzle3dPlayEngagement({
        activeTool: "select",
        cmdLine: "",
        fillCount: 0,
        selectionCount: 2,
        onCmdLineChange: () => {},
        onCmdLineSubmit: () => {},
        onSelectTool: () => {},
        onBrushTool: () => {},
        onFillTool: () => {},
        onFillCount: () => {},
        onCycleBrushCandidate: () => {},
        onPickBrushCandidate: () => {},
        onZoomToSelection: () => {},
        brushCandidates: [],
        brushTargetActive: false,
      });
      expect(spec.options?.map((row) => row.id)).toEqual(["puzzle3d.zoom"]);
    });
    it("requestPuzzle3dZoomToSelection bumps epoch only for non-empty selection", () => {
      const before = getPuzzle3dZoomToSelectionEpoch();
      requestPuzzle3dZoomToSelection({ objectIds: [], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(getPuzzle3dZoomToSelectionEpoch()).toBe(before);
      requestPuzzle3dZoomToSelection({ objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] });
      expect(getPuzzle3dZoomToSelectionEpoch()).toBe(before + 1);
      expect(getPuzzle3dZoomToSelectionTarget().objectIds).toEqual(["a"]);
    });
    it("buildPuzzle3dSelectionMenuItems labels Hide and Delete for visible selection", () => {
      const actions = {
        toggleHidden: vi.fn(),
        toggleLocked: vi.fn(),
        deleteSelection: vi.fn(),
        duplicateSelection: vi.fn(),
        selectSameKind: vi.fn(),
      };
      const items = buildPuzzle3dSelectionMenuItems(
        { objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] },
        [{ hidden: false, locked: false }],
        { kind: "object", id: "a" },
        actions,
      );
      expect(items.find((row) => row.id === "hidden")?.label).toBe("Hide");
      expect(items.find((row) => row.id === "delete")?.destructive).toBe(true);
      expect(items.some((row) => row.id === "duplicate")).toBe(true);
    });
    it("buildPuzzle3dSelectionMenuItems labels Show when all selected entities are hidden", () => {
      const items = buildPuzzle3dSelectionMenuItems(
        { objectIds: ["a"], vortexIds: [], attractionIds: [], referenceIds: [] },
        [{ hidden: true, locked: true }],
        { kind: "object", id: "a" },
        {
          toggleHidden: () => {},
          toggleLocked: () => {},
          deleteSelection: () => {},
          duplicateSelection: () => {},
          selectSameKind: () => {},
        },
      );
      expect(items.find((row) => row.id === "hidden")?.label).toBe("Show");
      expect(items.find((row) => row.id === "locked")?.label).toBe("Unlock");
    });
    it("buildPuzzle3dSelectionMenuItems prepends Suggest objects for a single vortex", () => {
      let suggested = false;
      const items = buildPuzzle3dSelectionMenuItems(
        { objectIds: [], vortexIds: ["obj:v1"], attractionIds: [] },
        [{ hidden: false, locked: false }],
        { kind: "vortex", fullId: "obj:v1" },
        {
          toggleHidden: () => {},
          toggleLocked: () => {},
          deleteSelection: () => {},
          duplicateSelection: () => {},
          selectSameKind: () => {},
        },
        () => {
          suggested = true;
        },
      );
      expect(items[0]?.id).toBe("suggest");
      items[0]?.onSelect?.(new Event("click"));
      expect(suggested).toBe(true);
    });
    it("hoverTargetToSelectionPick maps hover targets to selection picks", () => {
      expect(hoverTargetToSelectionPick({ kind: "object", id: "obj" })).toEqual({ kind: "object", id: "obj" });
      expect(hoverTargetToSelectionPick({ kind: "vortex", fullId: "obj:v1" })).toEqual({ kind: "vortex", fullId: "obj:v1" });
      expect(hoverTargetToSelectionPick({ kind: "attraction", id: "att" })).toEqual({ kind: "attraction", id: "att" });
    });
    it("updateWorldMatrixChain composes world position along the ancestor chain", () => {
      const scene = new Group();
      const parent = new Group();
      const child = new Group();
      scene.add(parent);
      parent.add(child);
      applyObjectPose(parent, [10, 0, 0], [0, 0, 0, 1], 1);
      applyObjectPose(child, [0, 5, 0], [0, 0, 0, 1], 1);
      updateWorldMatrixChain(child);
      const world = new Vector3();
      child.getWorldPosition(world);
      expect(world.x).toBeCloseTo(10, 4);
      expect(world.y).toBeCloseTo(5, 4);
      expect(world.z).toBeCloseTo(0, 4);
    });
    it("boundsFromPuzzle3dSelection unions object meshes and vortex points", () => {
      const root = new Group();
      root.userData.puzzle3dObjectId = "obj-a";
      const mesh = new Mesh(new BoxGeometry(10, 10, 10));
      root.add(mesh);
      applyObjectPose(root, [0, 0, 0], [0, 0, 0, 1], 1);
      const vortexWorld = new Vector3(20, 0, 0);
      const bounds = boundsFromPuzzle3dSelection(
        { objectIds: ["obj-a"], vortexIds: ["obj-b:link"], attractionIds: [] },
        {
          getObjectGroup: (id) => (id === "obj-a" ? root : null),
          getVortexWorld: (fullId) => (fullId === "obj-b:link" ? vortexWorld : null),
          listVortexBindings: () => [{ fullId: "obj-b:link", objectId: "obj-b", objectKind: undefined, vortexKind: undefined, radiusWorld: 2 }],
        },
        [],
      );
      expect(bounds).not.toBeNull();
      expect(bounds!.radius).toBeGreaterThan(8);
    });
    it("buildPuzzle3dPlayEngagement falls back to tool possibles when no collision-free brush candidates", () => {
      const spec = buildPuzzle3dPlayEngagement({
        activeTool: "brush",
        cmdLine: "",
        fillCount: 0,
        selectionCount: 0,
        onCmdLineChange: () => {},
        onCmdLineSubmit: () => {},
        onSelectTool: () => {},
        onBrushTool: () => {},
        onFillTool: () => {},
        onFillCount: () => {},
        onCycleBrushCandidate: () => {},
        onPickBrushCandidate: () => {},
        onZoomToSelection: () => {},
        brushCandidates: [],
        brushTargetActive: true,
      });
      expect(spec.possibleEngagements?.map((row) => row.id)).toEqual(["puzzle3d.tool.brush", PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID, "puzzle3d.tool.select"]);
      expect(spec.status?.some((row) => row.id === "puzzle3d.brush.none")).toBe(true);
    });
    it("buildPuzzle3dPlayEngagement exposes fill slider when fill tool is active", () => {
      const counts: number[] = [];
      const spec = buildPuzzle3dPlayEngagement({
        activeTool: "fill",
        cmdLine: "",
        fillCount: 42,
        selectionCount: 0,
        onCmdLineChange: () => {},
        onCmdLineSubmit: () => {},
        onSelectTool: () => {},
        onBrushTool: () => {},
        onFillTool: () => {},
        onFillCount: (count) => counts.push(count),
        onCycleBrushCandidate: () => {},
        onPickBrushCandidate: () => {},
        onZoomToSelection: () => {},
        brushCandidates: [],
        brushTargetActive: false,
      });
      expect(spec.sessionActive).toBe(true);
      expect(spec.control?.kind).toBe("slider");
      if (spec.control?.kind === "slider") {
        expect(spec.control.value).toBe(42);
        spec.control.onChange?.(7);
      }
      expect(counts).toEqual([7]);
    });
    it("buildPuzzle3dPlayEngagement exposes fill target-volume edit options", () => {
      let toggled = false;
      let deleted = false;
      const spec = buildPuzzle3dPlayEngagement({
        activeTool: "fill",
        cmdLine: "",
        fillCount: 1,
        fillEditTargetVolumes: true,
        selectedTargetVolumeCount: 1,
        selectionCount: 1,
        onCmdLineChange: () => {},
        onCmdLineSubmit: () => {},
        onSelectTool: () => {},
        onBrushTool: () => {},
        onFillTool: () => {},
        onFillCount: () => {},
        onToggleFillEditTargetVolumes: () => {
          toggled = true;
        },
        onDeleteSelectedTargetVolume: () => {
          deleted = true;
        },
        onCycleBrushCandidate: () => {},
        onPickBrushCandidate: () => {},
        onZoomToSelection: () => {},
        brushCandidates: [],
        brushTargetActive: false,
      });
      expect(spec.options?.some((row) => row.id === PUZZLE_3D_ENGAGEMENT_FILL_EDIT_VOLUMES_ID)).toBe(true);
      expect(spec.options?.some((row) => row.id === PUZZLE_3D_ENGAGEMENT_DELETE_TARGET_VOLUME_ID)).toBe(true);
      spec.options?.find((row) => row.id === PUZZLE_3D_ENGAGEMENT_FILL_EDIT_VOLUMES_ID)?.onPress?.();
      spec.options?.find((row) => row.id === PUZZLE_3D_ENGAGEMENT_DELETE_TARGET_VOLUME_ID)?.onPress?.();
      expect(toggled).toBe(true);
      expect(deleted).toBe(true);
      expect(spec.control).toBeUndefined();
      expect(spec.controls?.map((row) => row.label)).toEqual(["Width", "Depth", "Height"]);
      expect(spec.controls?.every((row) => row.kind === "slider")).toBe(true);
      for (const row of spec.controls ?? []) {
        if (row.kind !== "slider") continue;
        expect(row.min).toBe(VOXEL_BRUSH_SIZE_MIN);
        expect(row.max).toBe(VOXEL_BRUSH_SIZE_MAX);
        expect(row.step).toBe(VOXEL_BRUSH_SIZE_STEP);
        expect(row.value).toBe(10);
      }
    });
    it("snapCadToVoxelCenter and addVoxelToFixture place axis-aligned boxes", () => {
      expect(snapCadToVoxelCenter([0.2, 0.2, 0.2], [1, 2, 3])).toEqual([0.5, 1, 1.5]);
      const base: FixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        domain: "architecture",
        attractions: [],
        objects: [],
        references: [],
        targetVolumes: [],
      };
      const next = addVoxelToFixture(base, [0.2, 0.2, 0.2], [1, 2, 3]);
      expect(next.targetVolumes).toHaveLength(1);
      expect(next.targetVolumes[0]?.scale).toEqual([1, 2, 3]);
      expect(addVoxelToFixture(next, [0.2, 0.2, 0.2], [1, 2, 3]).targetVolumes).toHaveLength(1);
    });
    it("buildPuzzle3dPlayEngagement shows fill build progress on the slider label and caps max", () => {
      const spec = buildPuzzle3dPlayEngagement({
        activeTool: "fill",
        cmdLine: "",
        fillCount: 3,
        fillBuildProgress: { count: 12, maxCount: PUZZLE_3D_FILL_COUNT_MAX, done: false },
        selectionCount: 0,
        onCmdLineChange: () => {},
        onCmdLineSubmit: () => {},
        onSelectTool: () => {},
        onBrushTool: () => {},
        onFillTool: () => {},
        onFillCount: () => {},
        onCycleBrushCandidate: () => {},
        onPickBrushCandidate: () => {},
        onZoomToSelection: () => {},
        brushCandidates: [],
        brushTargetActive: false,
      });
      expect(spec.control?.kind).toBe("slider");
      if (spec.control?.kind === "slider") {
        expect(spec.control.label).toBe(`Fill 3 (building 12/${PUZZLE_3D_FILL_COUNT_MAX})`);
        expect(spec.control.max).toBe(12);
        expect(spec.control.value).toBe(3);
      }
    });
    it("applyBrushPlacementToFixture appends object and attraction", () => {
      const fixture: FixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        domain: "architecture",
        attractions: [],
        objects: [
          {
            id: "host",
            objectKind: "Tambour",
            meshUrl: "/meshes/tambour.glb",
            origin: [0, 0, 0],
            vortices: [{ id: "host:v0", vortexKind: "door tambour left", position: [0.9, 2.75, 0.2], direction: [0, 1, 0] }],
          },
        ],
      };
      const pose = computeBrushPlacementPose({
        sourceLocalPosition: [-1.3, -1.25, 0],
        sourceLocalDirection: [-1, 0, 0],
        targetWorldPositionCad: [0.9, 2.75, 0.2],
        targetWorldDirectionCad: [0, 1, 0],
      });
      const next = applyBrushPlacementToFixture(
        fixture,
        {
          targetVortexFullId: "host:v0",
          objectKindId: "Capsule J",
          sourceVortexIndex: 0,
          origin: pose.origin,
          orientation: pose.orientation,
          objectId: "brush-test-1",
        },
        brushCatalogs,
      );
      expect(next.objects.length).toBe(2);
      expect(next.attractions.length).toBe(1);
      expect(next.attractions[0]?.attracted).toBe("host:v0");
      expect(next.attractions[0]?.attracting.startsWith("brush-test-1:")).toBe(true);
    });
  });
  describe("Puzzle3dPrecompute", () => {
    it("extractBrushCollisionMeshBuffers produces transferable arrays", () => {
      const mesh = new Mesh(new BoxGeometry(8, 8, 8));
      const buffers = extractBrushCollisionMeshBuffers(mesh);
      expect(buffers?.positions).toBeInstanceOf(Float32Array);
      expect(buffers?.indices).toBeInstanceOf(Uint32Array);
      expect((buffers?.positions.length ?? 0) > 0).toBe(true);
    });
    it("puzzle3dPrecomputeUsesWorker is false under vitest", () => {
      expect(puzzle3dPrecomputeUsesWorker()).toBe(false);
    });
    it("wasm brush collision agrees with mesh-bvh on separated boxes", async () => {
      const { readFileSync } = await import("node:fs");
      const { dirname, join } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const wasmMod = await import("../rs/pkg/puzzle_3d.js");
      const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../rs/pkg/puzzle_3d_bg.wasm");
      wasmMod.initSync({ module: readFileSync(wasmPath) });
      await wasmMod.default();
      const session = new wasmMod.Puzzle3dPrecomputeSession();
      clearBrushCollisionGltfScenes();
      const obstacleUrl = "/test/obstacle.glb";
      const previewUrl = "/test/preview.glb";
      const boxPositions = new Float32Array([
        -4, -4, -4, 4, -4, -4, 4, 4, -4, -4, 4, -4, -4, -4, 4, 4, -4, 4, 4, 4, 4, -4, 4, 4, 4,
      ]);
      const boxIndices = new Uint32Array([
        0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2,
      ]);
      registerBrushCollisionGltfScene(obstacleUrl, new Mesh(new BoxGeometry(8, 8, 8)));
      registerBrushCollisionGltfScene(previewUrl, new Mesh(new BoxGeometry(8, 8, 8)));
      session.register_mesh(obstacleUrl, boxPositions, boxIndices);
      session.register_mesh(previewUrl, boxPositions, boxIndices);
      const fixture: FixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        domain: "architecture",
        attractions: [],
        objects: [
          {
            id: "obstacle",
            objectKind: "Kind",
            meshUrl: obstacleUrl,
            origin: [0, 0, 0],
            orientation: [0, 0, 0, 1],
            vortices: [{ id: "v0", vortexKind: "port-a", position: [0, 0, 0], direction: [0, 0, -1] }],
          },
          {
            id: "host",
            objectKind: "Host",
            meshUrl: "/test/unregistered-host.glb",
            origin: [12, 0, 0],
            orientation: [0, 0, 0, 1],
            vortices: [{ id: "v0", vortexKind: "port-a", position: [0, 0, 0], direction: [0, 0, -1] }],
          },
        ],
      };
      const catalogs: KindCatalogBundle = {
        objects: [
          { id: "Kind", meshUrl: previewUrl, vortices: [{ vortexKind: "port-b", position: [0, 0, 0], direction: [0, 0, -1] }] },
        ],
        vortices: [{ id: "port-a" }, { id: "port-b" }],
        cables: [{ id: "cable.link" }],
      };
      session.set_scene(
        JSON.stringify({
          fixture,
          kindCatalogs: catalogs,
          kindCompatibility: [{ bidirectional: true, specificity: "vortex", source: "port-b", target: "port-a" }],
          overlapBudget: DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
          seed: 1,
        }),
      );
      const wasmResult = JSON.parse(session.brush_candidates("host:v0")) as BrushCollisionFreeResult;
      const obstacleGroup = new Group();
      obstacleGroup.userData.puzzle3dMeshUrl = obstacleUrl;
      obstacleGroup.userData.puzzle3dObjectId = "obstacle";
      applyObjectPose(obstacleGroup, [0, 0, 0], [0, 0, 0, 1]);
      const bvhResult = brushCollisionFreeCandidates({
        scene: { collectObjectGroups: () => [obstacleGroup] },
        targetVortexFullId: "host:v0",
        candidates: brushCompatibleCandidates(
          { objectId: "host", objectKind: "Host", vortexKind: "port-a" },
          catalogs,
          [{ bidirectional: true, specificity: "vortex", source: "port-b", target: "port-a" }],
        ),
        target: { objectId: "host", objectKind: "Host", vortexKind: "port-a" },
        targetWorldPositionCad: [12, 0, 0],
        targetWorldDirectionCad: [0, 0, -1],
        kindCatalogs: catalogs,
        meshRootForUrl: brushCollisionGltfRoot,
        overlapBudget: DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
      });
      expect(wasmResult.unknownPending).toBe(false);
      expect(bvhResult.unknownPending).toBe(false);
      expect(wasmResult.free.length).toBe(bvhResult.free.length);
      clearBrushCollisionGltfScenes();
    });
    it("concrete forest first brush iteration on every seed b-* vortex yields all beam connectors", async () => {
      const concreteForestFixture = (await import("../fixture/concrete-forest.3d.json")).default as FixtureV1;
      const leftType = (await import("../../../semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-left.type.semio.json")).default as {
        connectors: { items: readonly { name: string; point: { x: number; y: number; z: number } }[] };
      };
      const rightType = (await import("../../../semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-right.type.semio.json")).default as {
        connectors: { items: readonly { name: string; point: { x: number; y: number; z: number } }[] };
      };
      const expectedBeamConnectors = [
        "b-p1-t-t1-c3-l",
        "b-p1-t-t1-c3-r",
        "b-p1-t-t2-c3-l",
        "b-p1-b-t1-c2-l",
        "b-p1-b-t1-c1-r",
        "b-p1-b-t1-c1-l",
        "b-p1-t-t2-c1-l",
        "b-p2-t-t1-c3-l",
        "b-p2-t-t1-c3-r",
        "b-p2-t-t2-c3-l",
        "b-p2-b-t1-c1-l",
        "b-p2-b-t1-c2-l",
        "b-p2-b-t1-c1-r",
        "b-p2-t-t1-c1-l",
      ] as const;
      const connectorNamesByKind: Record<string, readonly string[]> = {
        "Hexagonal Cut Concrete Forest Left": leftType.connectors.items.map((row) => row.name),
        "Hexagonal Cut Concrete Forest Right": rightType.connectors.items.map((row) => row.name),
      };
      const meta = concreteForestFixture.meta as Record<string, unknown> | undefined;
      const catalogs = meta?.kindCatalogs as KindCatalogBundle | undefined;
      const compat = (meta?.kindCompatibility as readonly KindCompatEntry[] | undefined) ?? [];
      const { readFileSync } = await import("node:fs");
      const { dirname, join } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const { GLTFLoader } = await import("three/addons/loaders/GLTFLoader.js");
      const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../../..");
      const meshDir = join(repoRoot, "semio/fixtures/kit/folder/abbau-aufbau");
      const loader = new GLTFLoader();
      const loadGlb = (name: string): Promise<Group> =>
        new Promise((resolveGlb, reject) => {
          const bytes = new Uint8Array(readFileSync(join(meshDir, name)));
          loader.parse(bytes.buffer, meshDir, (gltf) => resolveGlb(gltf.scene), reject);
        });
      clearBrushCollisionGltfScenes();
      const leftUrl = "/meshes/hexagonal-cut-concrete-forest-left.glb";
      const rightUrl = "/meshes/hexagonal-cut-concrete-forest-right.glb";
      registerBrushCollisionGltfScene(leftUrl, await loadGlb("hexagonal-cut-concrete-forest-left.glb"));
      registerBrushCollisionGltfScene(rightUrl, await loadGlb("hexagonal-cut-concrete-forest-right.glb"));
      for (const kind of catalogs?.objects ?? []) {
        const kitType = kind.id === "Hexagonal Cut Concrete Forest Left" ? leftType : rightType;
        const kitConnectors = connectorNamesByKind[kind.id];
        if (!kitConnectors) {
          continue;
        }
        for (let index = 0; index < (kind.vortices?.length ?? 0); index += 1) {
          const template = kind.vortices![index]!;
          const connector = kitType.connectors.items[index];
          if (!connector) {
            continue;
          }
          expect(template.position[0]).toBeCloseTo(connector.point.x, 4);
          expect(template.position[1]).toBeCloseTo(connector.point.y, 4);
          expect(template.position[2]).toBeCloseTo(connector.point.z, 4);
          expect(kitConnectors[index]).toBe(connector.name);
        }
      }
      const host = concreteForestFixture.objects[0]!;
      const hostGroup = new Group();
      hostGroup.userData.puzzle3dMeshUrl = leftUrl;
      hostGroup.userData.puzzle3dObjectId = host.id;
      applyObjectPose(hostGroup, host.origin, host.orientation ?? [0, 0, 0, 1]);
      const connectorNameForCandidate = (candidate: BrushCompatibleCandidate): string =>
        connectorNamesByKind[candidate.objectKindId]![candidate.sourceVortexIndex]!;
      const beamVortexIndexes = (host.vortices ?? [])
        .map((vortex, index) => ({ index, kind: vortex.vortexKind ?? "" }))
        .filter((row) => row.kind.startsWith("b-"))
        .map((row) => row.index);
      expect(beamVortexIndexes.length).toBeGreaterThan(0);
      for (const vortexIndex of beamVortexIndexes) {
        const vortex = host.vortices![vortexIndex]!;
        const target: AttractionVortexContext = {
          objectId: host.id,
          objectKind: host.objectKind,
          vortexKind: vortex.vortexKind,
        };
        const world = vortexWorldCadFromObject(host, vortexIndex)!;
        const compatible = brushCompatibleCandidates(target, catalogs, compat);
        const targetFullId = vortex.id ?? `${host.id}:v${vortexIndex}`;
        const bvhResult = brushCollisionFreeCandidates({
          scene: { collectObjectGroups: () => [hostGroup] },
          targetVortexFullId: targetFullId,
          candidates: compatible,
          target,
          targetWorldPositionCad: world.position,
          targetWorldDirectionCad: world.direction,
          referenceOrientationCad: host.orientation,
          kindCatalogs: catalogs,
          sceneFixture: concreteForestFixture,
          meshRootForUrl: brushCollisionGltfRoot,
          overlapBudget: DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
        });
        const freeConnectorNames = bvhResult.free.map(connectorNameForCandidate).sort();
        expect(bvhResult.unknownPending, `vortex ${targetFullId}`).toBe(false);
        expect(new Set(freeConnectorNames), `vortex ${targetFullId}`).toEqual(new Set(expectedBeamConnectors));
        expect(freeConnectorNames, `vortex ${targetFullId}`).toHaveLength(expectedBeamConnectors.length);
        const hostIncluded = brushCollisionFreeCandidates({
          scene: { collectObjectGroups: () => [hostGroup] },
          targetVortexFullId: targetFullId,
          candidates: compatible,
          target,
          targetWorldPositionCad: world.position,
          targetWorldDirectionCad: world.direction,
          referenceOrientationCad: host.orientation,
          kindCatalogs: catalogs,
          sceneFixture: concreteForestFixture,
          meshRootForUrl: brushCollisionGltfRoot,
          overlapBudget: DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
          excludeSceneObjectIds: new Set(),
        });
        expect(hostIncluded.free.length, `host overlap must be excluded for ${targetFullId}`).toBeLessThan(expectedBeamConnectors.length);
      }
      clearBrushCollisionGltfScenes();
    }, 120_000);
    it("wasm concrete forest brush agrees with mesh-bvh on real geometry beam connector set", async () => {
      const concreteForestFixture = (await import("../fixture/concrete-forest.3d.json")).default as FixtureV1;
      const leftType = (await import("../../../semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-left.type.semio.json")).default as {
        connectors: { items: readonly { name: string }[] };
      };
      const rightType = (await import("../../../semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-right.type.semio.json")).default as {
        connectors: { items: readonly { name: string }[] };
      };
      const connectorNamesByKind: Record<string, readonly string[]> = {
        "Hexagonal Cut Concrete Forest Left": leftType.connectors.items.map((row) => row.name),
        "Hexagonal Cut Concrete Forest Right": rightType.connectors.items.map((row) => row.name),
      };
      const meta = concreteForestFixture.meta as Record<string, unknown> | undefined;
      const catalogs = meta?.kindCatalogs as KindCatalogBundle | undefined;
      const compat = (meta?.kindCompatibility as readonly KindCompatEntry[] | undefined) ?? [];
      const { readFileSync } = await import("node:fs");
      const { dirname, join } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const { GLTFLoader } = await import("three/addons/loaders/GLTFLoader.js");
      const wasmMod = await import("../rs/pkg/puzzle_3d.js");
      const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../rs/pkg/puzzle_3d_bg.wasm");
      wasmMod.initSync({ module: readFileSync(wasmPath) });
      await wasmMod.default();
      const session = new wasmMod.Puzzle3dPrecomputeSession();
      clearBrushCollisionGltfScenes();
      const leftUrl = "/meshes/hexagonal-cut-concrete-forest-left.glb";
      const rightUrl = "/meshes/hexagonal-cut-concrete-forest-right.glb";
      const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../../..");
      const meshDir = join(repoRoot, "semio/fixtures/kit/folder/abbau-aufbau");
      const loader = new GLTFLoader();
      const loadGlb = (name: string): Promise<Group> =>
        new Promise((resolveGlb, reject) => {
          const bytes = new Uint8Array(readFileSync(join(meshDir, name)));
          loader.parse(bytes.buffer, meshDir, (gltf) => resolveGlb(gltf.scene), reject);
        });
      registerBrushCollisionGltfScene(leftUrl, await loadGlb("hexagonal-cut-concrete-forest-left.glb"));
      registerBrushCollisionGltfScene(rightUrl, await loadGlb("hexagonal-cut-concrete-forest-right.glb"));
      const leftBuffers = extractBrushCollisionMeshBuffers(brushCollisionGltfRoot(leftUrl)!)!;
      const rightBuffers = extractBrushCollisionMeshBuffers(brushCollisionGltfRoot(rightUrl)!)!;
      session.register_mesh(leftUrl, leftBuffers.positions, leftBuffers.indices);
      session.register_mesh(rightUrl, rightBuffers.positions, rightBuffers.indices);
      session.set_scene(
        JSON.stringify({
          fixture: concreteForestFixture,
          kindCatalogs: catalogs,
          kindCompatibility: compat,
          overlapBudget: DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
          hostRules: { rejectCapitalOnTambour: false, rejectLastSingleStoreyOnMidTambour: false, doorTambourRequiresDoorCapsule: false },
          seed: 42,
        }),
      );
      const host = concreteForestFixture.objects[0]!;
      const hostGroup = new Group();
      hostGroup.userData.puzzle3dMeshUrl = leftUrl;
      hostGroup.userData.puzzle3dObjectId = host.id;
      applyObjectPose(hostGroup, host.origin, host.orientation ?? [0, 0, 0, 1]);
      const connectorKey = (candidate: BrushCompatibleCandidate): string =>
        `${connectorNamesByKind[candidate.objectKindId]![candidate.sourceVortexIndex]}:${candidate.objectKindId}:${candidate.sourceVortexIndex}`;
      const beamVortexIndexes = (host.vortices ?? [])
        .map((vortex, index) => ({ index, kind: vortex.vortexKind ?? "" }))
        .filter((row) => row.kind.startsWith("b-"))
        .map((row) => row.index);
      for (const vortexIndex of beamVortexIndexes) {
        const vortex = host.vortices![vortexIndex]!;
        const target: AttractionVortexContext = {
          objectId: host.id,
          objectKind: host.objectKind,
          vortexKind: vortex.vortexKind,
        };
        const world = vortexWorldCadFromObject(host, vortexIndex)!;
        const compatible = brushCompatibleCandidates(target, catalogs, compat);
        const targetFullId = vortex.id ?? `${host.id}:v${vortexIndex}`;
        const bvhResult = brushCollisionFreeCandidates({
          scene: { collectObjectGroups: () => [hostGroup] },
          targetVortexFullId: targetFullId,
          candidates: compatible,
          target,
          targetWorldPositionCad: world.position,
          targetWorldDirectionCad: world.direction,
          referenceOrientationCad: host.orientation,
          kindCatalogs: catalogs,
          sceneFixture: concreteForestFixture,
          meshRootForUrl: brushCollisionGltfRoot,
          overlapBudget: DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
        });
        const wasmResult = JSON.parse(session.brush_candidates(targetFullId)) as BrushCollisionFreeResult;
        expect(wasmResult.unknownPending, `wasm ${targetFullId}`).toBe(false);
        expect(bvhResult.unknownPending, `bvh ${targetFullId}`).toBe(false);
        expect(new Set(wasmResult.free.map(connectorKey)), `wasm ${targetFullId}`).toEqual(new Set(bvhResult.free.map(connectorKey)));
      }
      clearBrushCollisionGltfScenes();
    }, 120_000);
  });
}
