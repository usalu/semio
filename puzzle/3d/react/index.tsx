// #region 🔌Adapters
import {
  reactHostPort,
  sceneHostPort,
  type ThreeEvent,
  type TreeDragAndDropController,
} from "@ui/react";
import React, { Children, isValidElement, type CSSProperties, type MutableRefObject, type ReactNode } from "react";
// #endregion 🔌Adapters

// #region 🔌PortWiring
const Canvas = sceneHostPort.fiber.canvas;
const createPortal = sceneHostPort.fiber.createPortal;
const useFrame = sceneHostPort.fiber.useFrame;
const useThree = sceneHostPort.fiber.useThree;
const Clone = sceneHostPort.drei.Clone;
const Line = sceneHostPort.drei.Line;
const OrbitControls = sceneHostPort.drei.OrbitControls;
const Outlines = sceneHostPort.drei.Outlines;
const PerspectiveCamera = sceneHostPort.drei.PerspectiveCamera;
const TransformControls = sceneHostPort.drei.TransformControls;
const useGLTF = sceneHostPort.drei.useGLTF;
const {
  Box3,
  BoxGeometry,
  BufferGeometry,
  Color,
  EdgesGeometry,
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

/** @emoji 📐 Default CAD anchor for the horizontal grid / palette drop plane (matches default orbit target Z). */
export const DEFAULT_PUZZLE3D_GRID_PLANE_ANCHOR_CAD: Vec3 = [0, 0, 40];

/** @emoji 📐 Linear slider domain for log-mapped scene LOD in play measures. */
export const PUZZLE_3D_LOD_SLIDER_MIN = 0;

/** @emoji 📐 Linear slider domain for log-mapped scene LOD in play measures. */
export const PUZZLE_3D_LOD_SLIDER_MAX = 1000;

/** @emoji 📐 Epsilon for scene LOD change notifications. */
export const PUZZLE_3D_LOD_EPSILON = 0.01;

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
export const PUZZLE_3D_LOD_GRID_MAX_LOD = 1000;

/** @emoji 📐 Scene LOD at or below which the medium grid band appears (puzzle 2d normal). */
export const PUZZLE_3D_LOD_GRID_MEDIUM_MAX_LOD = 50;

/** @emoji 📐 Scene LOD at or below which the small grid band appears (puzzle 2d detail). */
export const PUZZLE_3D_LOD_GRID_SMALL_MAX_LOD = 10;

/** @emoji 📐 Scene LOD at or below which the micro grid band appears (puzzle 2d micro). */
export const PUZZLE_3D_LOD_GRID_MICRO_MAX_LOD = 2;

/** @emoji 📐 Default grid factor (puzzle 2d `DEFAULT_PUZZLE_2D_GRID_FACTOR`). */
export const DEFAULT_LOD_GRID_FACTOR = 10;

/** @emoji 📐 One progressive LOD grid layer (world step + stroke opacity). */
export interface LodGridLayer {
  readonly stepWorld: number;
  readonly opacity: number;
}

export interface VortexProps {
  id: string;
  vortexKind?: string;
  /** @emoji 🏷️ Human-readable vortex label for play UI and hierarchy. */
  label?: string;
  position: Vec3;
  direction?: Vec3;
  radius?: number;
  visible?: boolean;
  vortexMeshUrl?: string;
  /** @emoji 🎨 Optional per-LOD GLB URLs for the vortex mesh; falls back to {@link vortexMeshUrl}. */
  vortexMeshByLod?: readonly LodMeshEntry[];
  children?: ReactNode;
}

export interface ObjectProps {
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
  visible?: boolean;
  relocate?: RelocateMode | false;
  /** @emoji ✋ When true, transform controls mount for this object (usually matches primary selected id). */
  relocateActive?: boolean;
  /** @emoji ­ƒº▓ Object ids attracted to this object in the resolved ownership tree. */
  attracting?: readonly string[];
  /** @emoji ­ƒò│´©Å Root of a connected attraction component (wormhole). */
  wormhole?: boolean;
  children?: ReactNode;
  userData?: Record<string, unknown>;
}

export interface AttractionProps {
  id: string;
  attracting: `${string}:${string}`;
  attracted: `${string}:${string}`;
  attractionKind?: string;
}

export const PLACEHOLDER_MESH_URL = "puzzle.3d.placeholder://box";

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

/** @emoji 🔌 Kit type connector row (CAD point + port handle kind) for catalog extraction. */
export interface KitConnectorCadRow {
  readonly point?: { readonly x: number; readonly y: number; readonly z: number };
  readonly direction?: { readonly x: number; readonly y: number; readonly z: number };
  readonly port?: { readonly handleKind?: string };
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

function cadVec3FromKitPoint(row: { readonly x: number; readonly y: number; readonly z: number }): Vec3 {
  return [row.x, row.y, row.z];
}

/** @emoji 🧲 Builds {@link ObjectKind.vortices} from kit connectors; keeps every distinct CAD position (same `vortexKind` allowed). */
export function puzzle3dObjectKindVorticesFromKitConnectors(
  connectors: readonly KitConnectorCadRow[],
  labelHandleKind: (handleKind: string) => string,
  defaultRadius = 0.36,
): ObjectKindVortexTemplate[] {
  const seenPositions = new Set<string>();
  const out: ObjectKindVortexTemplate[] = [];
  for (const connector of connectors) {
    const handleKind = connector.port?.handleKind?.trim() ?? "";
    const point = connector.point;
    if (handleKind === "" || !point) {
      continue;
    }
    const position = cadVec3FromKitPoint(point);
    const posKey = position.map((n) => n.toFixed(6)).join(",");
    if (seenPositions.has(posKey)) {
      continue;
    }
    seenPositions.add(posKey);
    const vortexKind = labelHandleKind(handleKind);
    out.push({
      vortexKind,
      position,
      ...(connector.direction ? { direction: cadVec3FromKitPoint(connector.direction) } : {}),
      radius: defaultRadius,
    });
  }
  return out;
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
}

/** @emoji 🎯 Compares selection snapshots (objects, vortices, attractions). */
export function selectionSnapshotsEqual(a: SelectionSnapshot, b: SelectionSnapshot): boolean {
  if (a.objectIds.length !== b.objectIds.length || a.vortexIds.length !== b.vortexIds.length || a.attractionIds.length !== b.attractionIds.length) {
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
  return true;
}

const EMPTY_SELECTION_SNAPSHOT: SelectionSnapshot = { objectIds: [], vortexIds: [], attractionIds: [] };

/** @emoji 🖱️ Pointer movement before a vortex press becomes an attraction drag (px). */
export const PUZZLE_3D_VORTEX_DRAG_THRESHOLD_PX = 6;

export type SelectionPick = { readonly kind: "object"; readonly id: string } | { readonly kind: "vortex"; readonly fullId: string } | { readonly kind: "attraction"; readonly id: string };

/** @emoji 🎯 Single-kind selection slice for one pick target. */
export function puzzle3dSelectionFromPick(pick: SelectionPick): SelectionSnapshot {
  switch (pick.kind) {
    case "object":
      return { objectIds: [pick.id], vortexIds: [], attractionIds: [] };
    case "vortex":
      return { objectIds: [], vortexIds: [pick.fullId], attractionIds: [] };
    case "attraction":
      return { objectIds: [], vortexIds: [], attractionIds: [pick.id] };
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

/** @emoji 🎯 Maps shift/ctrl modifiers to marquee selection mode (ctrl+shift → invertive). */
export function marqueeModeFromModifiers(modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean }): SelectionMode {
  const shift = modifiers.shiftKey === true;
  const ctrl = modifiers.ctrlKey === true || modifiers.metaKey === true;
  if (shift && ctrl) {
    return "invertive";
  }
  if (shift) {
    return "additive";
  }
  if (ctrl) {
    return "subtractive";
  }
  return "default";
}

/** @emoji 🎯 Applies selection mode when committing a canvas pick. */
export function mergeSelection(mode: SelectionMode, current: SelectionSnapshot, pick: SelectionPick): SelectionSnapshot {
  const piece = puzzle3dSelectionFromPick(pick);
  return mergeSelectionSnapshot(mode, current, piece);
}

/** @emoji 🎯 Applies selection mode when committing a marquee or multi-pick snapshot. */
export function mergeSelectionSnapshot(mode: SelectionMode, current: SelectionSnapshot, incoming: SelectionSnapshot): SelectionSnapshot {
  if (mode === "default") {
    return {
      objectIds: [...incoming.objectIds],
      vortexIds: [...incoming.vortexIds],
      attractionIds: [...incoming.attractionIds],
    };
  }
  return {
    objectIds: mergeIdList(mode, current.objectIds, incoming.objectIds),
    vortexIds: mergeIdList(mode, current.vortexIds, incoming.vortexIds),
    attractionIds: mergeIdList(mode, current.attractionIds, incoming.attractionIds),
  };
}

interface SelectionDerivation {
  readonly snapshot: SelectionSnapshot;
  readonly objectIdSet: ReadonlySet<string>;
  readonly vortexIdSet: ReadonlySet<string>;
  readonly vortexOwnerObjectIdSet: ReadonlySet<string>;
  readonly attractionIdSet: ReadonlySet<string>;
  readonly primaryObjectId: string | null;
}

function deriveSelectionSnapshot(snapshot: SelectionSnapshot): SelectionDerivation {
  const objectIdSet = new Set(snapshot.objectIds);
  const vortexIdSet = new Set(snapshot.vortexIds);
  const vortexOwnerObjectIdSet = new Set<string>();
  for (const fullId of snapshot.vortexIds) {
    vortexOwnerObjectIdSet.add(parseVortexFullId(fullId).objectId);
  }
  return {
    snapshot,
    objectIdSet,
    vortexIdSet,
    vortexOwnerObjectIdSet,
    attractionIdSet: new Set(snapshot.attractionIds),
    primaryObjectId: snapshot.objectIds[0] ?? (snapshot.vortexIds[0] ? parseVortexFullId(snapshot.vortexIds[0]).objectId : null),
  };
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
  const globalListeners = new Set<() => void>();
  const objectListeners = new Map<string, Set<() => void>>();
  const vortexListeners = new Map<string, Set<() => void>>();
  const attractionListeners = new Map<string, Set<() => void>>();
  const attractionBulkListeners = new Set<() => void>();
  const primaryListeners = new Set<() => void>();

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
    isObjectSelected(objectId: string): boolean {
      return derived.objectIdSet.has(objectId) || derived.vortexOwnerObjectIdSet.has(objectId);
    },
    isVortexSelected(fullId: string): boolean {
      return derived.vortexIdSet.has(fullId);
    },
    isAttractionSelected(attractionId: string): boolean {
      return derived.attractionIdSet.has(attractionId);
    },
    subscribeObject(objectId: string, listener: () => void): () => void {
      return addPerIdListener(objectListeners, objectId, listener);
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
    setSnapshot(next: SelectionSnapshot, equal: (left: SelectionSnapshot, right: SelectionSnapshot) => boolean = selectionSnapshotsEqual): void {
      if (equal(derived.snapshot, next)) {
        return;
      }
      const prev = derived;
      derived = deriveSelectionSnapshot(next);
      notifyPerIdListeners(objectListeners, selectionSetSymmetricDifference(prev.objectIdSet, derived.objectIdSet));
      notifyPerIdListeners(objectListeners, selectionSetSymmetricDifference(prev.vortexOwnerObjectIdSet, derived.vortexOwnerObjectIdSet));
      notifyPerIdListeners(vortexListeners, selectionSetSymmetricDifference(prev.vortexIdSet, derived.vortexIdSet));
      if (prev.attractionIdSet !== derived.attractionIdSet) {
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

/** @emoji 🎯 O(1) object highlight membership (direct object or parent of selected vortex). */
export function useObjectSelected(objectId: string): boolean {
  const store = useSelectionSnapshotStore();
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeObject(objectId, onStoreChange),
    () => store.isObjectSelected(objectId),
    () => store.isObjectSelected(objectId),
  );
}

/** @emoji 🎯 O(1) vortex highlight membership. */
export function useVortexSelected(fullId: string): boolean {
  const store = useSelectionSnapshotStore();
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => store.subscribeVortex(fullId, onStoreChange),
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

/** @emoji 🖱️ Exclusive scene hover target (at most one active). */
export type HoverTarget = { readonly kind: "object"; readonly id: string } | { readonly kind: "vortex"; readonly fullId: string } | { readonly kind: "attraction"; readonly id: string };

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
  domain?: DomainKind;
  chunkSize?: number;
  kindCatalogs?: KindCatalogBundle;
  kindCompatibility?: readonly KindCompatEntry[];
  /** @emoji ­ƒÜ½ Vortex full ids (`objectId:vortexId`) that already terminate an attraction and cannot start or receive a new attraction. */
  blockedVortexFullIds?: ReadonlySet<string>;
  proximityRadius?: number;
  /** @emoji 🔗 When false, skip O(vortices) proximity scan on gumball release (e.g. fixtures with no attractions). */
  proximityRelocateEnabled?: boolean;
  relocateMode?: RelocateMode;
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
  onConnect?: (p: AttractionPayload) => void;
  onIndirectConnect?: (p: AttractionPayload) => void;
  onProximityConnect?: (p: AttractionPayload) => void;
  onAttractionCompatibleObjects?: (p: AttractionCompatibleObjectsPayload) => void;
  onAttractionTargetRing?: (p: AttractionTargetRingPayload) => void;
  /** @emoji 🖌️ When true, hover free vortices to preview and flush compatible objects on leave. */
  brushActive?: boolean;
  /** @emoji 🖌️ Commits a brush placement (new object + attraction). */
  onBrushPlace?: (payload: BrushPlacePayload) => void;
  /** @emoji 📥 When true, accepts in-app fixture drags using {@link FIXTURE_DRAG_V1_MIME} (not OS file drops). */
  fixtureDragDrop?: boolean;
  /** @emoji 📥 Palette / shelf fixture dropped on the canvas at the grid-plane intersection. */
  onFixtureDrop?: (detail: Puzzle3dFixtureDropDetail) => void;
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
}

/** @emoji 📷 True when two camera states match within epsilon (avoids redundant fixture writes). */
export function cameraStateNearEqual(a: CameraState, b: CameraState, epsilon = 1e-3): boolean {
  for (let i = 0; i < 3; i += 1) {
    if (Math.abs(a.position[i]! - b.position[i]!) > epsilon) return false;
    if (Math.abs(a.target[i]! - b.target[i]!) > epsilon) return false;
  }
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

/** @emoji 🎨 Resolves a mesh URL from per-LOD entries with {@link pickClosestLod} and optional fallback. */
export function pickClosestMeshUrl(entries: readonly LodMeshEntry[] | undefined, desired: number, fallback?: string): string | undefined {
  if (!entries?.length) return fallback;
  const lods = entries.map((e) => e.lod).filter((lod) => Number.isFinite(lod) && lod > 0);
  const picked = pickClosestLod(lods, desired);
  if (picked == null) return fallback;
  const match = entries.find((e) => e.lod === picked);
  return match?.url ?? fallback;
}

/** @emoji 📶 Formats puzzle 3D LOD for `data-puzzle3d-lod` and play readouts. */
export function formatLod(lod: number): string {
  return Number.isFinite(lod) ? lod.toFixed(2) : "—";
}

/** @emoji 📶 Maps a linear slider position to log-spaced scene LOD. */
export function lodFromSliderValue(slider: number, range: { readonly min: number; readonly max: number } = DEFAULT_LOD_RANGE): number {
  const t = Math.max(0, Math.min(1, (slider - PUZZLE_3D_LOD_SLIDER_MIN) / (PUZZLE_3D_LOD_SLIDER_MAX - PUZZLE_3D_LOD_SLIDER_MIN)));
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
  return Math.round(PUZZLE_3D_LOD_SLIDER_MIN + t * (PUZZLE_3D_LOD_SLIDER_MAX - PUZZLE_3D_LOD_SLIDER_MIN));
}

/** @emoji 📶 Maps play / window LOD controls to {@link CanvasProps}. */
export function puzzle3dLodCanvasProps(state: { readonly automaticLod: boolean; readonly depthVariableLod: boolean; readonly manualLod: number }): Pick<CanvasProps, "automaticLod" | "depthVariableLod" | "lod"> {
  return {
    automaticLod: state.automaticLod,
    depthVariableLod: state.depthVariableLod,
    lod: !state.automaticLod && !state.depthVariableLod ? state.manualLod : undefined,
  };
}

/** @emoji 📐 Fixed LOD grid band steps in world units (`10` / `2.5` / `0.5` / `0.1` × {@link gridFactor}). */
export function lodGridBandStepsWorld(gridFactor: number): readonly [number, number, number, number] {
  const f = gridFactor;
  return [LOD_GRID_MAJOR_QUANTUM * f, LOD_GRID_MEDIUM_QUANTUM * f, LOD_GRID_SMALL_QUANTUM * f, LOD_GRID_MICRO_QUANTUM * f];
}

const LOD_GRID_LAYER_OPACITY = [1, 0.72, 0.48, 0.32] as const;

/** @emoji 📐 Progressive LOD grid layers to draw (puzzle 2d `stroke_world_step_grid` bands). */
export function lodProgressiveGridLayers(lod: number, gridFactor: number): readonly LodGridLayer[] {
  if (!Number.isFinite(lod) || lod <= 0 || lod > PUZZLE_3D_LOD_GRID_MAX_LOD) return [];
  const [large, medium, small, micro] = lodGridBandStepsWorld(gridFactor);
  const layers: LodGridLayer[] = [{ stepWorld: large, opacity: LOD_GRID_LAYER_OPACITY[0] }];
  if (lod <= PUZZLE_3D_LOD_GRID_MEDIUM_MAX_LOD) layers.push({ stepWorld: medium, opacity: LOD_GRID_LAYER_OPACITY[1] });
  if (lod <= PUZZLE_3D_LOD_GRID_SMALL_MAX_LOD) layers.push({ stepWorld: small, opacity: LOD_GRID_LAYER_OPACITY[2] });
  if (lod <= PUZZLE_3D_LOD_GRID_MICRO_MAX_LOD) layers.push({ stepWorld: micro, opacity: LOD_GRID_LAYER_OPACITY[3] });
  return layers;
}

/** @emoji 📐 Finest visible LOD grid / relocate snap step in world units. */
export function lodGridStepWorld(lod: number, gridFactor: number): number | null {
  const layers = lodProgressiveGridLayers(lod, gridFactor);
  if (!layers.length) return null;
  return layers[layers.length - 1]!.stepWorld;
}

/** @emoji 🌐 True when primary vortex visuals are drawn at the given scene LOD. */
export function lodVortexPrimaryVisible(lod: number): boolean {
  return lod <= 200;
}

/** @emoji 🌐 True when invisible vortex pick proxies are used instead of GLB vortex meshes. */
export function lodVortexPickProxy(lod: number): boolean {
  return lod > 200 && lod <= 1000;
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

/** @emoji 📶 Reads the live scene LOD band and grid snap step from canvas context. */
export function useLod(): LodContextValue {
  const v = reactHostPort.useContext(LodContext);
  if (!v) throw new Error("Puzzle 3D LOD missing");
  return v;
}

interface LodRuntimeCells {
  puzzle3dLod: number;
  depthVariable: boolean;
  distanceReference: number;
  camera: Camera | null;
  tmpWorld: Vector3;
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
}

function LodGridHelper() {
  const lod = useLod();
  const controls = useThree((s) => s.controls as { target?: Vector3 } | null);
  const anchor = controls?.target;
  const layers = reactHostPort.useMemo(() => lodProgressiveGridLayers(lod.lod, lod.gridFactor), [lod.lod, lod.gridFactor]);
  const grids = reactHostPort.useMemo(() => {
    const size = 12_000;
    return layers.map(({ stepWorld, opacity }) => {
      const divs = Math.min(512, Math.max(2, Math.round(size / stepWorld)));
      const grid = new GridHelper(size, divs, 0xb8c4d0, 0x6a7a8a);
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
  const px = anchor?.x ?? 0;
  const py = anchor?.y ?? 0;
  const pz = anchor?.z ?? 0;
  return (
    <>
      {grids.map((grid, i) => (
        <primitive key={`${layers[i]?.stepWorld ?? i}`} object={grid} position={[px, py, pz]} />
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
  readonly onLod: (patch: { readonly puzzle3dLod: number; readonly depthVariable: boolean; readonly gridStepWorld: number | null }) => void;
  readonly onLodChange?: (lod: number) => void;
}) {
  const cam = useThree((s) => s.camera);
  const controls = useThree((s) => s.controls as { target?: Vector3 } | null);
  const tmpT = reactHostPort.useMemo(() => new Vector3(), []);
  const prevLod = reactHostPort.useRef<number | null>(null);
  const ctxSig = reactHostPort.useRef("");
  useFrame(() => {
    const tgt = controls?.target ?? tmpT.set(0, 0, 0);
    const dist = cam.position.distanceTo(tgt);
    const autoLod = lodFromCameraDistance(dist, props.distanceReference);
    const puzzle3dLod = props.automaticLod ? autoLod : props.depthVariableLod ? autoLod : props.manualLod;
    props.lodRef.current = puzzle3dLod;
    const runtime = props.lodRuntimeRef.current;
    runtime.puzzle3dLod = puzzle3dLod;
    runtime.depthVariable = props.depthVariableLod;
    runtime.distanceReference = props.distanceReference;
    runtime.camera = cam;
    const gridStep = lodGridStepWorld(puzzle3dLod, props.gridFactor);
    const sig = `${puzzle3dLod}|${props.depthVariableLod ? 1 : 0}|${gridStep ?? "x"}|${props.gridFactor}|${props.gridSnapEnabled}`;
    if (ctxSig.current !== sig) {
      ctxSig.current = sig;
      props.onLod({ puzzle3dLod, depthVariable: props.depthVariableLod, gridStepWorld: gridStep });
    }
    if (prevLod.current === null || Math.abs(prevLod.current - puzzle3dLod) > PUZZLE_3D_LOD_EPSILON) {
      prevLod.current = puzzle3dLod;
      props.onLodChange?.(puzzle3dLod);
    }
  });
  return null;
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
  const tmpWorld = reactHostPort.useMemo(() => new Vector3(), []);
  const lodRuntimeRef = reactHostPort.useRef<LodRuntimeCells>({
    puzzle3dLod: DEFAULT_MANUAL_LOD,
    depthVariable: false,
    distanceReference: props.distanceReference,
    camera: null,
    tmpWorld,
  });
  const lodForWorldPosition = reactHostPort.useCallback((position: Vec3) => {
    const r = lodRuntimeRef.current;
    if (!r.depthVariable || !r.camera) return r.puzzle3dLod;
    r.tmpWorld.set(position[0], position[1], position[2]);
    return lodFromCameraDistance(r.camera.position.distanceTo(r.tmpWorld), r.distanceReference);
  }, []);
  const [puzzle3dLod, setLod] = reactHostPort.useState(DEFAULT_MANUAL_LOD);
  const [depthVariable, setDepthVariable] = reactHostPort.useState(false);
  const [gridStepWorld, setGridStepWorld] = reactHostPort.useState<number | null>(() => lodGridStepWorld(DEFAULT_MANUAL_LOD, props.gridFactor));
  const onLod = reactHostPort.useCallback((patch: { readonly puzzle3dLod: number; readonly depthVariable: boolean; readonly gridStepWorld: number | null }) => {
    setLod((prev) => (Math.abs(prev - patch.puzzle3dLod) > PUZZLE_3D_LOD_EPSILON ? patch.puzzle3dLod : prev));
    setDepthVariable((prev) => (prev === patch.depthVariable ? prev : patch.depthVariable));
    setGridStepWorld((prev) => (prev === patch.gridStepWorld ? prev : patch.gridStepWorld));
  }, []);
  const lodCtx = reactHostPort.useMemo<LodContextValue>(
    () => ({
      lod: puzzle3dLod,
      depthVariable,
      lodForWorldPosition,
      gridStepWorld,
      gridFactor: props.gridFactor,
      gridSnapEnabled: props.gridSnapEnabled,
    }),
    [puzzle3dLod, depthVariable, lodForWorldPosition, gridStepWorld, props.gridFactor, props.gridSnapEnabled],
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
      {props.showLodGrid ? <LodGridHelper /> : null}
      {props.children}
    </LodContext.Provider>
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
    objects: [{ id: "palette-seed-object", objectKind: objectKindId, meshUrl: "puzzle3d://palette-seed", origin: [0, 0, 0], vortices: [] }],
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

const puzzle3dGridPlaneUp = new Vector3(0, 1, 0);
const puzzle3dGridPlaneScratch = new Plane();
const puzzle3dGridPlaneHitScratch = new Vector3();
const puzzle3dGridPlanePointScratch = new Vector3();

/** @emoji 📍 Ray–grid-plane hit in CAD: cursor vs camera through the horizontal plane at {@link LodGridHelper} / orbit target. */
export function puzzle3dClientToGridPlaneCad(args: {
  readonly clientX: number;
  readonly clientY: number;
  readonly camera: Camera;
  readonly canvas: HTMLElement;
  readonly gridSnapEnabled?: boolean;
  readonly gridStepWorld?: number | null;
  /** @emoji 📐 Coplanar anchor in Three.js world space (orbit controls target). */
  readonly gridPlanePointThree?: readonly [number, number, number];
  /** @emoji 📐 Coplanar anchor in CAD when Three target is unavailable. */
  readonly gridPlaneAnchorCad?: Vec3;
}): Vec3 {
  const rect = args.canvas.getBoundingClientRect();
  const ndc = new Vector2(((args.clientX - rect.left) / rect.width) * 2 - 1, -((args.clientY - rect.top) / rect.height) * 2 + 1);
  const raycaster = new Raycaster();
  raycaster.setFromCamera(ndc, args.camera);
  if (args.gridPlanePointThree) {
    puzzle3dGridPlanePointScratch.set(args.gridPlanePointThree[0], args.gridPlanePointThree[1], args.gridPlanePointThree[2]);
  } else {
    const anchorCad = args.gridPlaneAnchorCad ?? DEFAULT_PUZZLE3D_GRID_PLANE_ANCHOR_CAD;
    const anchorThree = cadVec3ToThree(anchorCad);
    puzzle3dGridPlanePointScratch.set(anchorThree[0], anchorThree[1], anchorThree[2]);
  }
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

/** @emoji 🎨 Resolves a GLB URL for an object kind from the catalog, then from matching scene objects. */
export function resolveObjectKindMeshUrl(kindId: string, kindCatalogs: KindCatalogBundle | undefined, sceneFixture?: FixtureV1): string | undefined {
  const kind = catalogObjectKindById(kindCatalogs, kindId);
  const catalogMesh = kind?.meshUrl?.trim();
  if (catalogMesh) {
    return catalogMesh;
  }
  const lodMesh = pickClosestMeshUrl(kind?.meshByLod, DEFAULT_MANUAL_LOD, undefined);
  if (lodMesh) {
    return lodMesh;
  }
  if (sceneFixture) {
    for (const object of sceneFixture.objects) {
      if (object.objectKind === kindId) {
        const sceneMesh = object.meshUrl?.trim();
        if (sceneMesh) {
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

export interface ObjectRecord {
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

/** @emoji ⏱️ Defers fixture persist / proximity work until after pointer release paints. */
function scheduleRelocateCommit(run: () => void): void {
  if (typeof requestIdleCallback === "function") {
    requestIdleCallback(run, { timeout: 120 });
    return;
  }
  queueMicrotask(run);
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
  const syncedFixtureRevisionRef = reactHostPort.useRef<number | undefined>(undefined);
  const skipExternalPoseSyncRef = reactHostPort.useRef(false);
  const fixtureFingerprint = reactHostPort.useMemo(() => fixtureStateFingerprint(props.fixture), [props.fixture]);
  const poseFingerprint = reactHostPort.useMemo(() => fixturePoseFingerprint(props.fixture), [props.fixture]);
  reactHostPort.useEffect(() => {
    const puzzle3dStore = storeRef.current;
    if (!puzzle3dStore) {
      return;
    }
    if (skipExternalPoseSyncRef.current) {
      skipExternalPoseSyncRef.current = false;
      syncedPoseFingerprintRef.current = poseFingerprint;
      return;
    }
    if (props.fixtureRevision !== undefined && syncedFixtureRevisionRef.current !== props.fixtureRevision) {
      syncedFixtureRevisionRef.current = props.fixtureRevision;
      syncedFixtureFingerprintRef.current = fixtureFingerprint;
      syncedPoseFingerprintRef.current = poseFingerprint;
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
    if (syncedPoseFingerprintRef.current === poseFingerprint) {
      return;
    }
    syncedPoseFingerprintRef.current = poseFingerprint;
    puzzle3dStore.syncPosesFromFixture(props.fixture);
  }, [props.fixture, props.fixtureRevision, fixtureFingerprint, poseFingerprint]);
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
  readonly relocate?: RelocateMode | false;
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

/** @emoji 🎯 True when an object is part of the current selection (directly or via a selected vortex). */
export function objectMatchesSelection(objectId: string, selection: SelectionSnapshot | undefined): boolean {
  if (!selection) {
    return false;
  }
  if (selection.objectIds.includes(objectId)) {
    return true;
  }
  for (const vortexFullId of selection.vortexIds) {
    if (parseVortexFullId(vortexFullId).objectId === objectId) {
      return true;
    }
  }
  return false;
}

export interface ObjectsProps {
  readonly relocate?: RelocateMode | false;
}

/** @emoji ­ƒºè Renders all scene objects from central state (id-keyed; survives ownership changes). */
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

/** @emoji 🤝 WASM-style filtered attraction compatibility (important + specificity tiers); empty rules allow all. */
export function vorticesAttractionCompatibleForDrag(attracting: AttractionVortexContext, attracted: AttractionVortexContext, rules: readonly KindCompatEntry[] | undefined, catalogs: KindCatalogBundle | undefined): boolean {
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

function normalizeVec3Cad(v: Vec3): Vec3 {
  const len = Math.hypot(v[0], v[1], v[2]);
  if (len < 1e-9) {
    return [0, 0, -1];
  }
  return [v[0] / len, v[1] / len, v[2] / len] as Vec3;
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

function rotateVecByCadQuat(q: Quat, v: Vec3): Vec3 {
  const out = new Vector3(v[0], v[1], v[2]).applyQuaternion(quatToThree(q));
  return [out.x, out.y, out.z] as Vec3;
}

/** @emoji 🧭 World CAD position and direction of an object-local vortex. */
export function vortexWorldCadFromObject(record: Pick<ObjectRecord, "origin" | "orientation" | "vortices">, vortexIndex: number): { readonly position: Vec3; readonly direction: Vec3 } | null {
  const vortex = record.vortices[vortexIndex];
  if (!vortex) {
    return null;
  }
  const orientation = record.orientation ?? ([0, 0, 0, 1] as Quat);
  const position = vec3Add(record.origin, rotateVecByCadQuat(orientation, vortex.position));
  const direction = vortex.direction ? normalizeVec3Cad(rotateVecByCadQuat(orientation, vortex.direction)) : ([0, 0, -1] as Vec3);
  return { position, direction };
}

/** @emoji 🖌️ Lists catalog object kinds whose vortices can attract the target vortex. */
export function brushCompatibleCandidates(
  target: AttractionVortexContext,
  kindCatalogs: KindCatalogBundle | undefined,
  kindCompatibility: readonly KindCompatEntry[] | undefined,
): readonly BrushCompatibleCandidate[] {
  const objects = kindCatalogs?.objects;
  if (!objects?.length) {
    return [];
  }
  const out: BrushCompatibleCandidate[] = [];
  for (const kind of objects) {
    if (!kind.meshUrl || !kind.vortices?.length) {
      continue;
    }
    for (let sourceVortexIndex = 0; sourceVortexIndex < kind.vortices.length; sourceVortexIndex += 1) {
      const template = kind.vortices[sourceVortexIndex]!;
      const attracting: AttractionVortexContext = {
        objectId: "__brush__",
        objectKind: kind.id,
        vortexKind: template.vortexKind,
      };
      if (!vorticesAttractionCompatibleForDrag(attracting, target, kindCompatibility, kindCatalogs)) {
        continue;
      }
      out.push({ objectKindId: kind.id, sourceVortexIndex });
    }
  }
  return out;
}

/** @emoji 🖌️ Object pose so a source vortex coincides with the target point and directions oppose. */
export function computeBrushPlacementPose(args: {
  readonly sourceLocalPosition: Vec3;
  readonly sourceLocalDirection: Vec3;
  readonly scale?: number | Vec3;
  readonly targetWorldPositionCad: Vec3;
  readonly targetWorldDirectionCad: Vec3;
}): { readonly origin: Vec3; readonly orientation: Quat } {
  const localDir = normalizeVec3Cad(args.sourceLocalDirection);
  const targetDir = normalizeVec3Cad(args.targetWorldDirectionCad);
  const desiredWorldDir = negateVec3Cad(targetDir);
  const qThree = new Quaternion().setFromUnitVectors(new Vector3(...localDir), new Vector3(...desiredWorldDir));
  const orientation = threeQuatToCad(qThree);
  const scaledLocal = vec3ScaleCad(args.sourceLocalPosition, args.scale);
  const rotated = rotateVecByCadQuat(orientation, scaledLocal);
  const origin = vec3Sub(args.targetWorldPositionCad, rotated);
  return { origin, orientation };
}

/** @emoji 📦 True when two axis-aligned boxes overlap (with epsilon). */
export function boxesIntersect(a: Box3, b: Box3, epsilon = 1e-3): boolean {
  return a.min.x <= b.max.x + epsilon && a.max.x + epsilon >= b.min.x && a.min.y <= b.max.y + epsilon && a.max.y + epsilon >= b.min.y && a.min.z <= b.max.z + epsilon && a.max.z + epsilon >= b.min.z;
}

function brushPreviewFromCandidate(args: {
  readonly targetVortexFullId: string;
  readonly candidate: BrushCompatibleCandidate;
  readonly targetWorldPositionCad: Vec3;
  readonly targetWorldDirectionCad: Vec3;
  readonly kindCatalogs: KindCatalogBundle | undefined;
}): BrushPreviewState | null {
  const kind = catalogObjectKindById(args.kindCatalogs, args.candidate.objectKindId);
  const template = kind?.vortices?.[args.candidate.sourceVortexIndex];
  if (!kind?.meshUrl || !template) {
    return null;
  }
  const pose = computeBrushPlacementPose({
    sourceLocalPosition: template.position,
    sourceLocalDirection: template.direction ?? ([0, 0, -1] as Vec3),
    scale: kind.scale,
    targetWorldPositionCad: args.targetWorldPositionCad,
    targetWorldDirectionCad: args.targetWorldDirectionCad,
  });
  return {
    targetVortexFullId: args.targetVortexFullId,
    objectKindId: kind.id,
    sourceVortexIndex: args.candidate.sourceVortexIndex,
    meshUrl: kind.meshUrl,
    ...(kind.meshByLod ? { meshByLod: kind.meshByLod } : {}),
    ...(kind.scale !== undefined ? { scale: kind.scale } : {}),
    origin: pose.origin,
    orientation: pose.orientation,
  };
}

/** @emoji 🖌️ Appends a brush-placed object and its attraction to a fixture. */
export function applyBrushPlacementToFixture(fixture: FixtureV1, payload: BrushPlacePayload, kindCatalogs: KindCatalogBundle | undefined): FixtureV1 {
  const kind = catalogObjectKindById(kindCatalogs, payload.objectKindId);
  const template = kind?.vortices?.[payload.sourceVortexIndex];
  if (!kind?.meshUrl || !template) {
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
    meshUrl: kind.meshUrl,
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
//#endregion 🖌️Brush

//#region ­ƒÄ¿MeshPaint
const CSS_SELECTED_MESH = "color-mix(in oklab, var(--color-primary) 28%, var(--color-panel))";
const CSS_SELECTED_LINE = "var(--color-primary)";
const CSS_HIGHLIGHTED_MESH = "color-mix(in oklab, var(--color-secondary) 24%, var(--color-panel))";
const CSS_HIGHLIGHTED_LINE = "var(--color-secondary)";
const CSS_HOVERED_MESH = "var(--color-hover-panel)";
const CSS_HOVERED_LINE = "var(--color-hover-base)";
const CSS_NEUTRAL_MESH = "var(--color-panel)";
const CSS_NEUTRAL_LINE = "var(--color-element)";
const CSS_DISABLED_MESH = "color-mix(in oklab, var(--color-muted-foreground) 55%, var(--color-panel))";
const CSS_DISABLED_LINE = "var(--color-muted-foreground)";
const CSS_ATTRACTION_ENDPOINT_LINE = "var(--color-muted-foreground)";
const CSS_ATTRACTION_LINE = "var(--color-accent)";

const MESH_OUTLINE_USER_DATA_KEY = "__elementsMeshBodyOutline";

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
    meshColor: "#eeeadb",
    lineColor: "#001117",
    emissiveColor: "#000000",
    emissiveIntensity: 0,
    opacity: 1,
  },
  hovered: {
    meshColor: "#c0cdc5",
    lineColor: "#7b827d",
    emissiveColor: "#7b827d",
    emissiveIntensity: 0.08,
    opacity: 1,
  },
  selected: {
    meshColor: "#f0c8cc",
    lineColor: "#ff344f",
    emissiveColor: "#ff344f",
    emissiveIntensity: 0.35,
    opacity: 1,
  },
  highlighted: {
    meshColor: "#c4e4d5",
    lineColor: "#34d1bf",
    emissiveColor: "#34d1bf",
    emissiveIntensity: 0.2,
    opacity: 1,
  },
  disabled: {
    meshColor: "#c8ccc6",
    lineColor: "#7b827d",
    emissiveColor: "#000000",
    emissiveIntensity: 0,
    opacity: 0.45,
  },
};

function probeCssComputed(property: "color" | "backgroundColor", value: string): string {
  if (typeof document === "undefined") {
    return "";
  }
  const el = document.createElement("span");
  const key = property === "color" ? "color" : "background-color";
  el.setAttribute("style", `${key}:${value};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
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
    return "#808080";
  }
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    return "#808080";
  }
  ctx.fillStyle = "#000000";
  ctx.fillStyle = css;
  const converted = ctx.fillStyle;
  if (/^(oklab|oklch|lab|lch|color)\(/iu.test(converted)) {
    return "#808080";
  }
  return converted;
}

function resolveCssColor(property: "color" | "backgroundColor", expr: string, fallback: string): string {
  const raw = probeCssComputed(property, expr);
  if (!raw || raw === "rgba(0, 0, 0, 0)") {
    return fallback;
  }
  return cssColorForThree(raw);
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
  const resolved = {
    meshColor: resolveCssColor("backgroundColor", meshExprs[style], fb.meshColor),
    lineColor: resolveCssColor("color", lineExprs[style], fb.lineColor),
    emissiveColor: resolveCssColor("color", lineExprs[style], fb.emissiveColor),
    emissiveIntensity: fb.emissiveIntensity,
    opacity: fb.opacity,
  };
  meshStyleColorCache.set(style, resolved);
  return resolved;
}

function createStyledMeshMaterial(color: string, state: MeshStyleColors): MeshStandardMaterial {
  const mat = new MeshStandardMaterial({
    color: new Color(cssColorForThree(color)),
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
  const mat = new LineBasicMaterial({ color: new Color(cssColorForThree(color)) });
  mat.transparent = state.opacity < 1;
  mat.opacity = state.opacity;
  return mat;
}

function createMeshOutline(geometry: BufferGeometry, color: string, state: MeshStyleColors): LineSegments {
  const outline = new LineSegments(new EdgesGeometry(geometry), createStyledLineMaterial(color, state));
  outline.userData[MESH_OUTLINE_USER_DATA_KEY] = true;
  outline.scale.setScalar(1.001);
  return outline;
}

function applyMeshStyleToObject3D(root: Object3D, style: MeshStyleKind): void {
  const colors = meshStyleColors(style);
  if (!colors) {
    return;
  }
  root.traverse((object) => {
    if (object instanceof Mesh) {
      const meshMaterial = createStyledMeshMaterial(colors.meshColor, colors);
      if (Array.isArray(object.material)) {
        object.material = object.material.map(() => meshMaterial.clone());
      } else {
        object.material = meshMaterial;
      }
      const geometry = object.geometry;
      if (geometry && !object.children.some((c) => c.userData[MESH_OUTLINE_USER_DATA_KEY])) {
        object.add(createMeshOutline(geometry, colors.lineColor, colors));
      }
      return;
    }
    if (object instanceof ThreeLine || object instanceof LineSegments) {
      if (object.userData[MESH_OUTLINE_USER_DATA_KEY]) {
        return;
      }
      object.material = createStyledLineMaterial(colors.lineColor, colors);
      return;
    }
    if (object instanceof Points) {
      object.material = new PointsMaterial({
        color: new Color(cssColorForThree(colors.lineColor)),
        size: 1,
        transparent: colors.opacity < 1,
        opacity: colors.opacity,
      });
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
export function lineCssColor(expr: string, fallback: string): string {
  return resolveCssColor("color", expr, fallback);
}
//#endregion ­ƒÄ¿MeshPaint

//#region ­ƒÅèPool
const gltfRefCounts = new Map<string, number>();
const styledMeshRefCounts = new Map<string, number>();
const styledMeshTemplates = new Map<string, Object3D>();

function styledPoolKey(url: string, style: MeshStyleKind): string {
  return `${url}\0${style}`;
}

export function gltfPoolAcquire(url: string): void {
  gltfRefCounts.set(url, (gltfRefCounts.get(url) ?? 0) + 1);
}

export function gltfPoolRelease(url: string): void {
  const n = (gltfRefCounts.get(url) ?? 1) - 1;
  if (n <= 0) {
    gltfRefCounts.delete(url);
  } else {
    gltfRefCounts.set(url, n);
  }
}

export function styledMeshPoolAcquire(url: string, style: MeshStyleKind): void {
  const key = styledPoolKey(url, style);
  styledMeshRefCounts.set(key, (styledMeshRefCounts.get(key) ?? 0) + 1);
}

export function styledMeshPoolRelease(url: string, style: MeshStyleKind): void {
  const key = styledPoolKey(url, style);
  const n = (styledMeshRefCounts.get(key) ?? 1) - 1;
  if (n <= 0) {
    styledMeshRefCounts.delete(key);
    styledMeshTemplates.delete(key);
  } else {
    styledMeshRefCounts.set(key, n);
  }
}

/** @emoji ­ƒº╣ Drops pooled GLTF cache entries (call on scene teardown, not per-chunk unmount). */
export function gltfPoolClear(url: string): void {
  gltfRefCounts.delete(url);
  for (const key of [...styledMeshTemplates.keys()]) {
    if (key.startsWith(`${url}\0`)) {
      styledMeshTemplates.delete(key);
      styledMeshRefCounts.delete(key);
    }
  }
  useGLTF.clear(url);
}

/** @emoji ­ƒÅè Returns a cached styled GLTF template for {@link MeshBody} (refcount via acquire/release). */
export function styledMeshTemplate(url: string, style: MeshStyleKind, source: Object3D): Object3D {
  if (style === "original") {
    return source;
  }
  const key = styledPoolKey(url, style);
  let template = styledMeshTemplates.get(key);
  if (!template) {
    template = source.clone(true);
    applyMeshStyleToObject3D(template, style);
    styledMeshTemplates.set(key, template);
  }
  return template;
}

function usePooledGltf(url: string) {
  const gltf = useGLTF(url);
  reactHostPort.useEffect(() => {
    gltfPoolAcquire(url);
    return () => {
      gltfPoolRelease(url);
    };
  }, [url]);
  return gltf;
}

function usePooledStyledMesh(url: string, style: MeshStyleKind) {
  const gltf = usePooledGltf(url);
  reactHostPort.useEffect(() => {
    if (style === "original") {
      return undefined;
    }
    styledMeshPoolAcquire(url, style);
    return () => {
      styledMeshPoolRelease(url, style);
    };
  }, [url, style]);
  const renderRoot = reactHostPort.useMemo(() => {
    if (!gltf.scene) {
      return null;
    }
    const template = styledMeshTemplate(url, style, gltf.scene);
    return template.clone(true);
  }, [gltf.scene, url, style]);
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
  selectedObjectIds: readonly string[];
  setSelectedObjectIds(ids: readonly string[]): void;
  selectionMode: SelectionMode;
  relocateMode: RelocateMode;
  activeRelocateObjectId: string | null;
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
  setHover: (target: HoverTarget) => void;
  clearHover: (target: HoverTarget) => void;
  clearHoverAll: () => void;
  isHovered: (target: HoverTarget) => boolean;
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
  commitMarqueeSelection(args: {
    readonly startX: number;
    readonly startY: number;
    readonly endX: number;
    readonly endY: number;
    readonly path: readonly ScreenPoint[];
    readonly modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean };
  }): void;
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
  setHover(target: HoverTarget): void;
  clearHover(target: HoverTarget): void;
  clearHoverAll(): void;
  isHovered(target: HoverTarget): boolean;
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
  const { hoverTarget } = useRegistryHover();
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [hoverTarget, invalidate]);
  return null;
}

/** @emoji 🎯 Redraws the canvas when host-controlled selection changes. */
function SelectionInvalidateBridge(): null {
  const selection = useLiveSelection();
  const invalidate = useThree((state) => state.invalidate);
  const selectionKey = reactHostPort.useMemo(
    () => `${selection.objectIds.join("\0")}|${selection.vortexIds.join("\0")}|${selection.attractionIds.join("\0")}`,
    [selection.attractionIds, selection.objectIds, selection.vortexIds],
  );
  reactHostPort.useEffect(() => {
    invalidate();
  }, [selectionKey, invalidate]);
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
      if (event.button !== 0 || attractionBusyRef.current || puzzle3dRelocateDragActiveRef.current) {
        previous?.(event);
        return;
      }
      const hits = getState().internal.initialHits;
      if (hits.some((hit) => raycastHitTargetsPick(hit.object))) {
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

//#region ­ƒº▒Chunking
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

/** @emoji ­ƒôÅ Chunk bounding radius in world units (half-space diagonal of a cube chunk). */
export function chunkBoundsRadius(chunkSize: number): number {
  return chunkSize * 0.866;
}

/** @emoji ­ƒæü´©Å Distance-only chunk visibility with enter/exit hysteresis (avoids frustum-edge flicker). */
export function chunkDistanceVisible(args: { readonly camPos: Vector3; readonly chunkCenter: Vector3; readonly chunkSize: number; readonly maxDist: number; readonly wasVisible: boolean }): boolean {
  const boundsR = chunkBoundsRadius(args.chunkSize);
  const dist = args.camPos.distanceTo(args.chunkCenter);
  const enterDist = args.maxDist + boundsR;
  const exitDist = enterDist + args.chunkSize * 0.5;
  if (dist <= enterDist) return true;
  if (args.wasVisible && dist <= exitDist) return true;
  return false;
}

function useVisibleChunkKeys(chunkKeys: Iterable<string>, chunkSize: number, maxDist: number): ReadonlySet<string> {
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
//#endregion ­ƒº▒Chunking

//#region 🧭Coordinates
const _eulerCadToThree = new Euler(-Math.PI / 2, 0, 0, "XYZ");
const _mCadToThree = new Matrix4();
const _qCadToThree = new Quaternion();

/** @emoji 🧭 Fixed CAD Z-up → Three.js Y-up rotation matrix (matches sketchpad {@link toThreeRotation}). */
export function cadToThreeMatrix(): Matrix4 {
  return _mCadToThree.makeRotationFromEuler(_eulerCadToThree);
}

/** @emoji 🧭 Maps a CAD fixture point to Three.js world coordinates. */
export function cadVec3ToThree(v: Vec3): Vec3 {
  const out = new Vector3(v[0], v[1], v[2]).applyMatrix4(cadToThreeMatrix());
  return [out.x, out.y, out.z];
}

/** @emoji 🧭 Maps a Three.js point back to CAD fixture coordinates. */
export function threeVec3ToCad(v: Vector3): Vec3 {
  const out = v.clone().applyMatrix4(cadToThreeMatrix().clone().invert());
  return [out.x, out.y, out.z];
}

/** @emoji 🧭 Maps a CAD fixture quaternion to Three.js. */
export function cadQuatToThree(q: Quat): Quat {
  const qCad = new Quaternion(q[0], q[1], q[2], q[3]);
  const out = _qCadToThree.setFromRotationMatrix(cadToThreeMatrix()).multiply(qCad);
  return [out.x, out.y, out.z, out.w];
}

/** @emoji 🧭 Maps a Three.js quaternion back to CAD fixture coordinates. */
export function threeQuatToCad(q: Quaternion): Quat {
  const out = _qCadToThree.setFromRotationMatrix(cadToThreeMatrix()).invert().multiply(q);
  return [out.x, out.y, out.z, out.w];
}

function quatRotateVec(q: Quat, v: Vec3): Vec3 {
  const out = new Vector3(v[0], v[1], v[2]).applyQuaternion(new Quaternion(q[0], q[1], q[2], q[3]));
  return [out.x, out.y, out.z];
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

/** @emoji ➡️ Maps object-local CAD direction to a unit vector in the vortex parent Three.js group. */
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

/** @emoji 🧭 +90° X: glTF Y-up mesh → CAD object-local Z-up inside the pose group (metabolism kit meshes). */
export const GLB_MESH_FRAME_ROTATION_X = Math.PI / 2;
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

/** @emoji ­ƒî│ Updates world matrices from root to leaf so {@link Object3D.getWorldPosition} matches the current graph. */
export function updateWorldMatrixChain(leaf: Object3D): void {
  const chain: Object3D[] = [];
  for (let cur: Object3D | null = leaf; cur; cur = cur.parent) {
    chain.push(cur);
  }
  for (let i = chain.length - 1; i >= 0; i--) {
    chain[i]!.updateMatrixWorld(false);
  }
}

export type AutoFitBehavior = "initial" | "changes";

export function puzzle3dAutoFitShouldRun(behavior: AutoFitBehavior, key: string, lastKey: string, hasApplied: boolean): boolean {
  if (!key || key === lastKey) return false;
  return behavior === "changes" || !hasApplied;
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

/** @emoji 🛰️ Frames perspective orbit camera to fit scene object bounds (CAD center, Three world rig). */
export function applyAutoFitCamera(camera: ThreePerspectiveCamera, bounds: { readonly center: Vec3; readonly radius: number }, padding = 1.25, controls?: { readonly target: Vector3; update?: () => void } | null): void {
  const centerThree = cadVec3ToThree(bounds.center);
  const dist = Math.max(bounds.radius * padding, 2);
  camera.position.set(centerThree[0] + dist, centerThree[1] + dist, centerThree[2] + dist * 0.85);
  if (controls?.target) {
    controls.target.set(centerThree[0], centerThree[1], centerThree[2]);
    controls.update?.();
  } else {
    camera.lookAt(centerThree[0], centerThree[1], centerThree[2]);
  }
  camera.updateProjectionMatrix();
}

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
  readonly userData?: Record<string, unknown>;
  readonly scale?: number | [number, number, number];
}

/** @emoji 🧭 Inner group: glTF Y-up mesh geometry → CAD object-local Z-up (fixture pose stays CAD). */
function GlbMeshFrame(props: { readonly children: ReactNode }) {
  return <group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>{props.children}</group>;
}

/** @emoji ­ƒºè Pooled GLB body with {@link MeshStyleKind} recoloring aligned to Elements tokens. */
export const MeshBody = reactHostPort.memo(function MeshBody(props: MeshProps) {
  const style = props.style ?? DEFAULT_MESH_STYLE;
  const renderRoot = usePooledStyledMesh(props.meshUrl, style);
  if (!renderRoot) {
    return null;
  }
  const scale = props.scale;
  const outlineColor = meshStyleColors("selected")?.lineColor ?? "#ff344f";
  return (
    <GlbMeshFrame>
      <Clone
        object={renderRoot}
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

const PlaceholderMesh = reactHostPort.memo(function PlaceholderMesh(props: MeshPointerHandlers & { readonly style: MeshStyleKind; readonly showOutline?: boolean }) {
  const colors = meshStyleColors(props.style);
  const meshColor = colors?.meshColor ?? "#cbd5e1";
  const opacity = colors?.opacity ?? 1;
  const outlineColor = meshStyleColors("selected")?.lineColor ?? "#ff344f";
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
  readonly mode: RelocateMode;
  readonly translationSnap: number | undefined;
  readonly beforeRef: MutableRefObject<{ origin: Vector3; quat: Quaternion; scale: Vector3 } | null>;
}) {
  const { onRelocate, findNearestProximityRelocate, onProximityConnect, proximityRelocateEnabled } = useRegistryCore();
  const scene = useThree((s) => s.scene);
  return createPortal(
    <TransformControls
      object={props.object}
      mode={props.mode}
      translationSnap={props.translationSnap}
      onMouseDown={() => {
        puzzle3dRelocateDragActiveRef.current = true;
        cancelPuzzle3dMarqueeGesture();
        const g = props.object;
        props.beforeRef.current = {
          origin: g.position.clone(),
          quat: g.quaternion.clone(),
          scale: g.scale.clone(),
        };
      }}
      onDraggingChanged={(event) => {
        if (!event) {
          return;
        }
        puzzle3dRelocateDragActiveRef.current = event.value;
        if (event.value) {
          cancelPuzzle3dMarqueeGesture();
        }
      }}
      onMouseUp={() => {
        puzzle3dRelocateDragActiveRef.current = false;
        const before = props.beforeRef.current;
        if (!before) return;
        const g = props.object;
        const payload: RelocatePayload = {
          objectId: props.objectId,
          mode: props.mode,
          before: {
            origin: threeVec3ToCad(before.origin),
            orientation: threeQuatToCad(before.quat),
            scale: before.scale.toArray() as unknown as Vec3,
          },
          after: {
            origin: threeVec3ToCad(g.position),
            orientation: threeQuatToCad(g.quaternion),
            scale: g.scale.toArray() as unknown as Vec3,
          },
        };
        props.beforeRef.current = null;
        onRelocate?.(payload);
        if (!proximityRelocateEnabled) {
          return;
        }
        scheduleRelocateCommit(() => {
          const cand = findNearestProximityRelocate(g.position, props.objectId);
          if (cand) onProximityConnect?.(cand);
        });
      }}
    />,
    scene,
  );
});

export const ObjectItem = reactHostPort.memo(function ObjectItem(props: ObjectProps) {
  const group = reactHostPort.useRef<Group>(null);
  const registrySelected = useObjectSelected(props.id);
  const primaryObjectId = usePrimarySelectionObjectId();
  const { registerObject, relocateMode } = useRegistryCore();
  const { selectionMode, commitSelection, setActiveRelocateObjectId } = useRegistryInteraction();
  const { setHover, clearHover, isHovered } = useRegistryHover();
  const { attractionDragActive, attractionIndirectPickAwait, attractionCompatibleAttractedFullIds } = useRegistryDrag();
  const beforeRef = reactHostPort.useRef<{ origin: Vector3; quat: Quaternion; scale: Vector3 } | null>(null);
  const [tcTarget, setTcTarget] = reactHostPort.useState<Group | null>(null);
  const objectPointerHovered = isHovered({ kind: "object", id: props.id });
  const selected = props.selected === true || registrySelected;
  const relocateActive = props.relocateActive === true || primaryObjectId === props.id;

  reactHostPort.useEffect(() => {
    registerObject(props.id, props.objectKind, group.current);
    return () => {
      registerObject(props.id, props.objectKind, null);
    };
  }, [props.id, props.objectKind, registerObject]);

  reactHostPort.useEffect(() => {
    if (group.current) setTcTarget(group.current);
  }, [selected, relocateActive, props.id]);

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
        selected,
        highlighted: linkHighlighted,
        hovered: props.hovered === true || objectPointerHovered,
      }),
    [props.style, props.disabled, selected, props.hovered, linkHighlighted, objectPointerHovered],
  );
  const showSelectionOutline = selected && !props.disabled;
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    if (selected || showSelectionOutline) {
      invalidate();
    }
  }, [selected, showSelectionOutline, meshStyle, invalidate]);

  const selectObject = reactHostPort.useCallback(() => {
    if (attractionDragActive || attractionIndirectPickAwait || props.disabled || puzzle3dRelocateDragActiveRef.current) {
      return;
    }
    commitSelection({ kind: "object", id: props.id });
    setActiveRelocateObjectId(props.id);
  }, [attractionDragActive, attractionIndirectPickAwait, commitSelection, props.disabled, props.id, setActiveRelocateObjectId]);

  const meshPointerHandlers = reactHostPort.useMemo(
    () => ({
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
    }),
    [clearHover, props.disabled, props.id, selectObject, setHover],
  );

  const poseKey = reactHostPort.useMemo(() => objectPoseKey(props.id, props.origin, props.orientation, props.scale), [props.id, props.origin, props.orientation, props.scale]);
  reactHostPort.useLayoutEffect(() => {
    const g = group.current;
    if (!g || groupMatchesFixturePose(g, props.origin, props.orientation, props.scale)) {
      return;
    }
    applyObjectPose(g, props.origin, props.orientation, props.scale);
  }, [poseKey, props.origin, props.orientation, props.scale]);
  const lodCtx = useLod();
  const resolvedMeshUrl = useResolvedMeshUrl({
    origin: props.origin,
    meshByLod: props.meshByLod,
    fallbackMeshUrl: props.meshUrl,
  });
  const mode = props.relocate ?? relocateMode;
  const transSnap = mode === "translate" && lodCtx.gridSnapEnabled && lodCtx.gridStepWorld != null && lodCtx.gridStepWorld > 0 ? lodCtx.gridStepWorld : undefined;
  const showTc = selected && relocateActive && props.relocate !== false && tcTarget;

  return (
    <>
      <group
        ref={group}
        visible={props.visible !== false}
        userData={{
          puzzle3dObjectId: props.id,
          puzzle3dMeshStyle: meshStyle,
          ...(props.attracting?.length ? { puzzle3dAttracting: props.attracting } : {}),
          ...(props.wormhole ? { puzzle3dWormhole: true } : {}),
          ...props.userData,
        }}
      >
        {resolvedMeshUrl === PLACEHOLDER_MESH_URL ? (
          <PlaceholderMesh showOutline={showSelectionOutline} style={meshStyle} {...meshPointerHandlers} />
        ) : (
          <MeshBody meshUrl={resolvedMeshUrl} showOutline={showSelectionOutline} style={meshStyle} {...meshPointerHandlers} />
        )}
        <group userData={{ puzzle3dObjectAttachments: props.id }}>{props.children}</group>
      </group>
      {showTc && tcTarget && <ObjectTransformControls object={tcTarget} objectId={props.id} mode={mode} translationSnap={transSnap} beforeRef={beforeRef} />}
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
    () => lineCssColor(props.selected ? CSS_HOVERED_LINE : CSS_ATTRACTION_ENDPOINT_LINE, props.selected ? "#38bdf8" : "#94a3b8"),
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

  const selectVortex = reactHostPort.useCallback(() => {
    if (reg.attractionDragActive || reg.attractionIndirectPickAwait || reg.blockedVortexFullIds.has(fullId)) {
      return;
    }
    commitSelection({ kind: "vortex", fullId });
    setActiveRelocateObjectId(props.objectId);
  }, [commitSelection, fullId, props.objectId, reg, setActiveRelocateObjectId]);

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
      if (!gesture.dragStarted && !puzzle3dMarqueeSuppressClickRef.current && !puzzle3dRelocateDragActiveRef.current) {
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
  const drawVortexBody = trackVortexLod ? lodVisual.drawVortexBody || linger : lodVortexPrimaryVisible(lodCtx.lod) || linger;
  const meshUrl = trackVortexLod ? lodVisual.meshUrl : pickClosestMeshUrl(props.vortexMeshByLod, lodCtx.lod, props.vortexMeshUrl);

  const positionThree = reactHostPort.useMemo(() => cadObjectLocalToThreeGroupLocal(props.position, props.objectOrigin, props.objectOrientation), [props.position, props.objectOrigin, props.objectOrientation]);

  const vortexPointerHovered = reg.isHovered({ kind: "vortex", fullId });
  const vortexMeshStyle = highlight === "none" && vortexPointerHovered ? "hovered" : vortexHighlightMeshStyle(highlight);

  const vortexPointerHoverHandlers = reactHostPort.useMemo(
    () => ({
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
    }),
    [fullId, reg],
  );

  const vis = props.visible !== false;
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
      <mesh userData={{ puzzle3dVortexFullId: fullId }} raycast={vortexPickRaycast} renderOrder={-1} {...vortexPointerHoverHandlers}>
        <sphereGeometry args={[r * 1.15, 12, 12]} />
        <meshBasicMaterial transparent opacity={0} depthWrite={false} depthTest={false} />
      </mesh>
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
  const mat = reactHostPort.useMemo(() => {
    const color = lineCssColor(CSS_ATTRACTION_ENDPOINT_LINE, "#64748b");
    return new LineBasicMaterial({ color, transparent: true, opacity: 0.85, depthTest: true, vertexColors: true });
  }, []);
  const geo = reactHostPort.useMemo(() => new BufferGeometry(), []);
  const normalColor = reactHostPort.useMemo(() => new Color(lineCssColor(CSS_ATTRACTION_ENDPOINT_LINE, "#64748b")), []);
  const hoveredColor = reactHostPort.useMemo(() => new Color(lineCssColor(CSS_HOVERED_LINE, "#7b827d")), []);
  const selectedColor = reactHostPort.useMemo(() => new Color(lineCssColor(CSS_SELECTED_LINE, "#38bdf8")), []);
  reactHostPort.useLayoutEffect(() => {
    const vertexCount = Math.max(props.attractions.length * 2, 2);
    geo.setAttribute("position", new Float32BufferAttribute(new Float32Array(vertexCount * 3), 3));
    geo.setAttribute("color", new Float32BufferAttribute(new Float32Array(vertexCount * 3), 3));
  }, [geo, props.attractions.length]);
  useFrame(() => {
    const pos = geo.attributes.position as Float32BufferAttribute;
    const colors = geo.attributes.color as Float32BufferAttribute;
    const hoveredId = reg.hoverTarget?.kind === "attraction" ? reg.hoverTarget.id : null;
    let write = 0;
    for (const attraction of props.attractions) {
      const a = reg.getVortexWorld(attraction.attracting);
      const b = reg.getVortexWorld(attraction.attracted);
      const c = selectedAttractionIds.has(attraction.id) ? selectedColor : attraction.id === hoveredId ? hoveredColor : normalColor;
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
      if (reg.attractionDragActive || reg.attractionIndirectPickAwait) {
        return;
      }
      const idx = puzzle3dAttractionIndexFromPointerEvent(e);
      const attraction = props.attractions[idx];
      if (attraction) {
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
      if (reg.attractionDragActive || reg.attractionIndirectPickAwait) {
        return;
      }
      const idx = puzzle3dAttractionIndexFromPointerEvent(e);
      const attraction = props.attractions[idx];
      if (attraction) {
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
    mode: reg.relocateMode,
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

/** @emoji 🖱️ Crossing selection when the drag ends left of the start (partial overlap). */
export function marqueeIsCrossing(startX: number, endX: number): boolean {
  return endX < startX;
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
  readonly points: readonly ScreenPoint[];
}

export interface MarqueeSelectionInput {
  readonly method: SelectionMethod;
  readonly crossing: boolean;
  readonly rect: ScreenRect | null;
  readonly polygon: readonly ScreenPoint[];
  readonly kinds: MarqueeSelectableKinds;
  readonly candidates: readonly MarqueeCandidate[];
}

function marqueePointMatcher(method: SelectionMethod, rect: ScreenRect | null, polygon: readonly ScreenPoint[]): (point: ScreenPoint) => boolean {
  if (method === "rectangle" && rect) {
    return (point) => pointInScreenRect(point, rect);
  }
  return (point) => pointInPolygon(point, polygon);
}

function marqueeCandidateSelected(input: MarqueeSelectionInput, candidate: MarqueeCandidate): boolean {
  if (candidate.points.length === 0) {
    return false;
  }
  const contains = marqueePointMatcher(input.method, input.rect, input.polygon);
  if (input.crossing) {
    if (candidate.points.some(contains)) {
      return true;
    }
    if (input.method === "rectangle" && input.rect) {
      const objectRect = screenRectFromClientPoints(
        Math.min(...candidate.points.map((p) => p.x)),
        Math.min(...candidate.points.map((p) => p.y)),
        Math.max(...candidate.points.map((p) => p.x)),
        Math.max(...candidate.points.map((p) => p.y)),
      );
      if (screenRectIntersectsRect(input.rect, objectRect)) {
        return true;
      }
      for (let i = 0; i < candidate.points.length - 1; i += 1) {
        if (segmentIntersectsScreenRect(candidate.points[i]!, candidate.points[i + 1]!, input.rect)) {
          return true;
        }
      }
      return false;
    }
    for (let i = 0; i < candidate.points.length - 1; i += 1) {
      if (segmentIntersectsPolygon(candidate.points[i]!, candidate.points[i + 1]!, input.polygon)) {
        return true;
      }
    }
    return false;
  }
  if (input.method === "rectangle" && input.rect) {
    const objectRect = screenRectFromClientPoints(
      Math.min(...candidate.points.map((p) => p.x)),
      Math.min(...candidate.points.map((p) => p.y)),
      Math.max(...candidate.points.map((p) => p.x)),
      Math.max(...candidate.points.map((p) => p.y)),
    );
    return screenRectContainsRect(input.rect, objectRect);
  }
  return candidate.points.every(contains);
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

/** @emoji 🖱️ Projects a world point to client coordinates for marquee tests. */
export function projectWorldToClient(point: Vector3, camera: Camera, rect: DOMRect): ScreenPoint | null {
  const projected = point.clone().project(camera);
  if (!Number.isFinite(projected.x) || !Number.isFinite(projected.y) || !Number.isFinite(projected.z)) {
    return null;
  }
  if (projected.z < -1 || projected.z > 1) {
    return null;
  }
  return {
    x: rect.left + ((projected.x + 1) / 2) * rect.width,
    y: rect.top + ((1 - projected.y) / 2) * rect.height,
  };
}

/** @emoji 🖱️ Projects an object group's bounds corners to client space. */
export function projectObjectGroupToScreenPoints(group: Group, camera: Camera, rect: DOMRect): ScreenPoint[] {
  const box = new Box3().setFromObject(group, true);
  if (box.isEmpty()) {
    return [];
  }
  const corners = [
    new Vector3(box.min.x, box.min.y, box.min.z),
    new Vector3(box.max.x, box.min.y, box.min.z),
    new Vector3(box.min.x, box.max.y, box.min.z),
    new Vector3(box.max.x, box.max.y, box.min.z),
    new Vector3(box.min.x, box.min.y, box.max.z),
    new Vector3(box.max.x, box.min.y, box.max.z),
    new Vector3(box.min.x, box.max.y, box.max.z),
    new Vector3(box.max.x, box.max.y, box.max.z),
  ];
  const out: ScreenPoint[] = [];
  for (const corner of corners) {
    const client = projectWorldToClient(corner, camera, rect);
    if (client) {
      out.push(client);
    }
  }
  return out;
}

export interface MarqueeOverlaySnapshot {
  readonly active: boolean;
  readonly method: SelectionMethod;
  readonly start: ScreenPoint | null;
  readonly current: ScreenPoint | null;
  readonly path: readonly ScreenPoint[];
}

const MARQUEE_OVERLAY_IDLE: MarqueeOverlaySnapshot = { active: false, method: "rectangle", start: null, current: null, path: [] };

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

/** @emoji 🖌️ Brush menu + preview snapshot for DOM overlay and scene ghost. */
export interface BrushUiSnapshot {
  readonly preview: BrushPreviewState | null;
  readonly menu:
    | {
        readonly x: number;
        readonly y: number;
        readonly targetVortexFullId: string;
        readonly candidates: readonly BrushCompatibleCandidate[];
      }
    | null;
}

const BRUSH_UI_IDLE: BrushUiSnapshot = { preview: null, menu: null };

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

/** @emoji 🖌️ True while the brush tool is the active play tool. */
export const puzzle3dBrushToolActiveRef = { current: false };

/** @emoji 🖌️ True while the cursor is over a free vortex in brush mode (suppresses orbit right-drag). */
export const puzzle3dBrushVortexHoverRef = { current: false };

//#region 🎬Viewport
type OrbitControlsBinding = {
  readonly mouseButtons: { LEFT: number | null; MIDDLE: number; RIGHT: number };
  readonly enabled: boolean;
  readonly update?: () => void;
};

function OrbitGated(props: { readonly camera: ThreePerspectiveCamera | null; readonly zoom: number; readonly onCamera?: (state: CameraState) => void }) {
  const reg = useRegistry();
  const { camera, gl } = useThree();
  const controls = useThree((s) => s.controls as OrbitControlsBinding | null);
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const gate = reg.attractionDragActive || reg.attractionIndirectPickAwait !== null || (puzzle3dBrushToolActiveRef.current && puzzle3dBrushVortexHoverRef.current);
  const invalidate = useThree((s) => s.invalidate);
  const rightPointerRef = reactHostPort.useRef<{ readonly pointerId: number; readonly x: number; readonly y: number } | null>(null);
  const reportCamera = reactHostPort.useCallback(() => {
    if (!props.onCamera) {
      return;
    }
    const tgt = controls?.target ?? targetScratch.set(0, 0, 0);
    props.onCamera({
      position: threeVec3ToCad(camera.position),
      target: threeVec3ToCad(tgt),
      zoom: props.zoom,
    });
  }, [camera, controls, props.onCamera, props.zoom, targetScratch]);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [gate, invalidate]);
  reactHostPort.useLayoutEffect(() => {
    if (!controls) {
      return;
    }
    controls.mouseButtons.LEFT = null;
    controls.mouseButtons.MIDDLE = MOUSE.DOLLY;
    controls.mouseButtons.RIGHT = MOUSE.ROTATE;
    controls.update?.();
  }, [controls]);
  reactHostPort.useEffect(() => {
    const dom = gl.domElement;
    const assignRightMouse = (event: PointerEvent) => {
      if (!controls || event.button !== 2) {
        return;
      }
      if (event.shiftKey) {
        controls.mouseButtons.RIGHT = MOUSE.PAN;
      } else if (event.altKey) {
        controls.mouseButtons.RIGHT = MOUSE.DOLLY;
      } else {
        controls.mouseButtons.RIGHT = MOUSE.ROTATE;
      }
      controls.update?.();
    };
    const onPointerDown = (event: PointerEvent) => {
      if (event.button !== 2) {
        return;
      }
      if (puzzle3dBrushToolActiveRef.current && puzzle3dBrushVortexHoverRef.current) {
        return;
      }
      puzzle3dRightDragActiveRef.current = false;
      rightPointerRef.current = { pointerId: event.pointerId, x: event.clientX, y: event.clientY };
      assignRightMouse(event);
    };
    const onPointerMove = (event: PointerEvent) => {
      const start = rightPointerRef.current;
      if (!start || start.pointerId !== event.pointerId) {
        return;
      }
      if (Math.hypot(event.clientX - start.x, event.clientY - start.y) >= PUZZLE_3D_MARQUEE_DRAG_THRESHOLD_PX) {
        puzzle3dRightDragActiveRef.current = true;
      }
    };
    const onPointerUp = (event: PointerEvent) => {
      const start = rightPointerRef.current;
      if (!start || start.pointerId !== event.pointerId) {
        return;
      }
      rightPointerRef.current = null;
      if (controls) {
        controls.mouseButtons.RIGHT = MOUSE.ROTATE;
        controls.update?.();
      }
      window.setTimeout(() => {
        puzzle3dRightDragActiveRef.current = false;
      }, 0);
    };
    const bindings = new EventBindingController();
    bindings.listen(dom, "pointerdown", onPointerDown as EventListener, true);
    bindings.listen(window, "pointermove", onPointerMove as EventListener);
    bindings.listen(window, "pointerup", onPointerUp as EventListener, true);
    return () => bindings.dispose();
  }, [controls, gl]);
  if (!props.camera) {
    return null;
  }
  return (
    <OrbitControls
      camera={props.camera}
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
      mouseButtons={{ LEFT: null as unknown as number, MIDDLE: MOUSE.DOLLY, RIGHT: MOUSE.ROTATE }}
    />
  );
}

/** @emoji 🛰️ Frames orbit camera to loaded object bounds once meshes are measurable (initial load fit). */
function AutoFit(props: { readonly behavior?: AutoFitBehavior; readonly padding?: number; readonly zoom?: number; readonly onCamera?: (state: CameraState) => void }): null {
  const reg = useRegistry();
  const { camera, controls, invalidate } = useThree();
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const lastKey = reactHostPort.useRef("");
  const hasApplied = reactHostPort.useRef(false);
  const behavior = props.behavior ?? "initial";
  const padding = props.padding ?? 1.25;
  const zoom = props.zoom ?? 1;
  useFrame(() => {
    if (behavior === "initial" && hasApplied.current) {
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
    if (!(camera instanceof ThreePerspectiveCamera)) return;
    const orbit = controls as { target: Vector3; update?: () => void } | null;
    applyAutoFitCamera(camera, bounds, padding, orbit);
    invalidate();
    const tgt = orbit?.target ?? targetScratch.set(...cadVec3ToThree(bounds.center));
    props.onCamera?.({
      position: threeVec3ToCad(camera.position),
      target: threeVec3ToCad(tgt),
      zoom,
    });
  });
  return null;
}

/** @emoji ­ƒôÀ Seeds default camera + orbit target once; orbit owns the rig afterward (no controlled-camera feedback loop). */
function CameraSeed(props: { readonly camera: ThreePerspectiveCamera | null; readonly position: Vec3; readonly target: Vec3 }) {
  const controls = useThree((s) => s.controls as { target: Vector3; update: () => void } | null);
  const seededPositionFor = reactHostPort.useRef("");
  const seededTargetFor = reactHostPort.useRef("");
  const positionKey = props.position.join(",");
  const targetKey = props.target.join(",");
  reactHostPort.useLayoutEffect(() => {
    const camera = props.camera;
    if (!camera) {
      return;
    }
    if (seededPositionFor.current !== positionKey) {
      seededPositionFor.current = positionKey;
      const p = cadVec3ToThree(props.position);
      camera.position.set(p[0], p[1], p[2]);
      camera.updateProjectionMatrix();
    }
    if (controls?.target && seededTargetFor.current !== targetKey) {
      seededTargetFor.current = targetKey;
      const t = cadVec3ToThree(props.target);
      controls.target.set(t[0], t[1], t[2]);
      controls.update();
    }
  }, [controls, positionKey, props.camera, props.position, props.target, targetKey]);
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

function brushPreviewCollides(reg: RegistryCoreValue, previewGroup: Group): boolean {
  updateWorldMatrixChain(previewGroup);
  const previewBox = new Box3().setFromObject(previewGroup, true);
  if (!Number.isFinite(previewBox.min.x) || previewBox.isEmpty()) {
    return false;
  }
  for (const group of reg.collectObjectGroups()) {
    updateWorldMatrixChain(group);
    const other = new Box3().setFromObject(group, true);
    if (!Number.isFinite(other.min.x) || other.isEmpty()) {
      continue;
    }
    if (boxesIntersect(previewBox, other)) {
      return true;
    }
  }
  return false;
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

const BrushPreviewGhost = reactHostPort.memo(function BrushPreviewGhost(props: {
  readonly preview: BrushPreviewState;
  readonly onCollision: (collides: boolean) => void;
}) {
  const reg = useRegistryCore();
  const groupRef = reactHostPort.useRef<Group>(null);
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useLayoutEffect(() => {
    const group = groupRef.current;
    if (!group) {
      return;
    }
    applyObjectPose(group, props.preview.origin, props.preview.orientation, props.preview.scale);
    updateWorldMatrixChain(group);
    props.onCollision(brushPreviewCollides(reg, group));
    invalidate();
  }, [props.preview, props.onCollision, reg, invalidate]);
  return (
    <group ref={groupRef} raycast={() => null}>
      <MeshBody meshUrl={props.preview.meshUrl} style="highlighted" scale={props.preview.scale} />
    </group>
  );
});

function BrushSession(props: {
  readonly brushActive: boolean;
  readonly onBrushPlace?: (payload: BrushPlacePayload) => void;
  readonly kindCatalogs: KindCatalogBundle | undefined;
  readonly kindCompatibility: readonly KindCompatEntry[] | undefined;
}) {
  const reg = useRegistryCore();
  const { store } = useObjectState();
  const invalidate = useThree((state) => state.invalidate);
  const targetRef = reactHostPort.useRef<string | null>(null);
  const candidatesRef = reactHostPort.useRef<readonly BrushCompatibleCandidate[]>([]);
  const indexRef = reactHostPort.useRef(0);
  const targetWorldRef = reactHostPort.useRef<{ readonly position: Vec3; readonly direction: Vec3 } | null>(null);
  const ui = reactHostPort.useSyncExternalStore(puzzle3dBrushUiStore.subscribe, puzzle3dBrushUiStore.getSnapshot, puzzle3dBrushUiStore.getSnapshot);

  const clearBrush = reactHostPort.useCallback(() => {
    targetRef.current = null;
    candidatesRef.current = [];
    indexRef.current = 0;
    targetWorldRef.current = null;
    puzzle3dBrushVortexHoverRef.current = false;
    puzzle3dBrushUiStore.setSnapshot(BRUSH_UI_IDLE);
  }, []);

  const commitCurrentPreview = reactHostPort.useCallback(() => {
    const preview = puzzle3dBrushUiStore.getSnapshot().preview;
    if (!preview || !props.onBrushPlace) {
      return;
    }
    props.onBrushPlace(brushPlacePayloadFromPreview(preview));
  }, [props.onBrushPlace]);

  const applyCandidateIndex = reactHostPort.useCallback(
    (targetFullId: string, index: number) => {
      const world = targetWorldRef.current;
      const candidate = candidatesRef.current[index];
      if (!world || !candidate) {
        puzzle3dBrushUiStore.setSnapshot({ preview: null, menu: puzzle3dBrushUiStore.getSnapshot().menu });
        return;
      }
      const preview = brushPreviewFromCandidate({
        targetVortexFullId: targetFullId,
        candidate,
        targetWorldPositionCad: world.position,
        targetWorldDirectionCad: world.direction,
        kindCatalogs: props.kindCatalogs,
      });
      const snap = puzzle3dBrushUiStore.getSnapshot();
      puzzle3dBrushUiStore.setSnapshot({ preview, menu: snap.menu });
      invalidate();
    },
    [invalidate, props.kindCatalogs],
  );

  const advanceCandidate = reactHostPort.useCallback(() => {
    const list = candidatesRef.current;
    if (!list.length || !targetRef.current) {
      return;
    }
    indexRef.current = (indexRef.current + 1) % list.length;
    applyCandidateIndex(targetRef.current, indexRef.current);
  }, [applyCandidateIndex]);

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
      candidatesRef.current = brushCompatibleCandidates(targetCtx, props.kindCatalogs, props.kindCompatibility);
      indexRef.current = 0;
      applyCandidateIndex(fullId, 0);
    },
    [applyCandidateIndex, props.kindCatalogs, props.kindCompatibility, store],
  );

  const leaveTarget = reactHostPort.useCallback(() => {
    if (!targetRef.current) {
      return;
    }
    commitCurrentPreview();
    clearBrush();
  }, [clearBrush, commitCurrentPreview]);

  const onPreviewCollision = reactHostPort.useCallback(
    (collides: boolean) => {
      if (!collides || !props.brushActive || candidatesRef.current.length <= 1) {
        return;
      }
      advanceCandidate();
    },
    [advanceCandidate, props.brushActive],
  );

  reactHostPort.useEffect(() => {
    if (!props.brushActive) {
      clearBrush();
    }
  }, [clearBrush, props.brushActive]);

  reactHostPort.useEffect(() => {
    const onPick = (event: Event) => {
      const index = (event as CustomEvent<{ index: number }>).detail.index;
      if (!props.brushActive || !targetRef.current || index < 0) {
        return;
      }
      indexRef.current = index;
      applyCandidateIndex(targetRef.current, index);
      invalidate();
    };
    window.addEventListener("puzzle3d-brush-pick-candidate", onPick);
    return () => window.removeEventListener("puzzle3d-brush-pick-candidate", onPick);
  }, [applyCandidateIndex, invalidate, props.brushActive]);

  return (
    <>
      <BrushPointerBridge
        brushActive={props.brushActive}
        blockedVortexFullIds={reg.blockedVortexFullIds}
        targetRef={targetRef}
        candidatesRef={candidatesRef}
        enterTarget={enterTarget}
        leaveTarget={leaveTarget}
        commitCurrentPreview={commitCurrentPreview}
        clearBrush={clearBrush}
        advanceCandidate={advanceCandidate}
        invalidate={invalidate}
      />
      {ui.preview ? <BrushPreviewGhost preview={ui.preview} onCollision={onPreviewCollision} /> : null}
    </>
  );
}

function BrushPointerBridge(props: {
  readonly brushActive: boolean;
  readonly blockedVortexFullIds: ReadonlySet<string>;
  readonly targetRef: MutableRefObject<string | null>;
  readonly candidatesRef: MutableRefObject<readonly BrushCompatibleCandidate[]>;
  readonly enterTarget: (fullId: string, meta: VortexBindingMeta) => void;
  readonly leaveTarget: () => void;
  readonly commitCurrentPreview: () => void;
  readonly clearBrush: () => void;
  readonly advanceCandidate: () => void;
  readonly invalidate: () => void;
}) {
  const reg = useRegistryCore();
  const { camera, gl } = useThree();
  const raycasterRef = reactHostPort.useRef(new Raycaster());
  const ndcRef = reactHostPort.useRef(new Vector2());

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
    const onMove = (event: PointerEvent) => {
      const canvas = gl.domElement;
      const rect = canvas.getBoundingClientRect();
      ndcRef.current.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
      ndcRef.current.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
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
      const picked = pickNearestScreenVortex({
        cursorX: event.clientX,
        cursorY: event.clientY,
        surfaceDist,
        candidates: screenCandidates,
      });
      puzzle3dBrushVortexHoverRef.current = picked !== null;
      if (!picked) {
        if (props.targetRef.current) {
          props.leaveTarget();
        }
        return;
      }
      const meta = reg.listVortexBindings().find((entry) => entry.fullId === picked.fullId);
      if (!meta) {
        return;
      }
      if (picked.fullId !== props.targetRef.current) {
        if (props.targetRef.current) {
          props.commitCurrentPreview();
          props.clearBrush();
        }
        props.enterTarget(picked.fullId, meta);
      }
      props.invalidate();
    };
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Tab" || !props.targetRef.current) {
        return;
      }
      event.preventDefault();
      props.advanceCandidate();
      props.invalidate();
    };
    const onContextMenu = (event: MouseEvent) => {
      if (!props.targetRef.current || !puzzle3dBrushVortexHoverRef.current) {
        return;
      }
      event.preventDefault();
      const snap = puzzle3dBrushUiStore.getSnapshot();
      puzzle3dBrushUiStore.setSnapshot({
        preview: snap.preview,
        menu: {
          x: event.clientX,
          y: event.clientY,
          targetVortexFullId: props.targetRef.current,
          candidates: [...props.candidatesRef.current],
        },
      });
    };
    const bindings = new EventBindingController();
    bindings.listen(window, "pointermove", onMove);
    bindings.listen(window, "keydown", onKeyDown, true);
    bindings.listen(window, "contextmenu", onContextMenu, true);
    return () => bindings.dispose();
  }, [camera, gl, props, reg]);
  return null;
}

function Puzzle3dBrushContextMenu(props: { readonly rootRef: React.RefObject<HTMLDivElement | null> }): React.ReactElement | null {
  const snap = reactHostPort.useSyncExternalStore(puzzle3dBrushUiStore.subscribe, puzzle3dBrushUiStore.getSnapshot, puzzle3dBrushUiStore.getSnapshot);
  if (!snap.menu) {
    return null;
  }
  const root = props.rootRef.current;
  if (!root) {
    return null;
  }
  const rootRect = root.getBoundingClientRect();
  const left = snap.menu.x - rootRect.left;
  const top = snap.menu.y - rootRect.top;
  return (
    <div className="pointer-events-auto absolute z-20 min-w-[12rem] rounded-md border border-border bg-panel py-1 text-sm shadow-lg" style={{ left, top }} data-puzzle3d-brush-menu>
      {snap.menu.candidates.map((candidate) => {
        const label = `${candidate.objectKindId} · ${candidate.sourceVortexIndex}`;
        return (
          <button
            key={`${candidate.objectKindId}:${candidate.sourceVortexIndex}`}
            type="button"
            className="block w-full px-3 py-1.5 text-left hover:bg-muted"
            onClick={() => {
              const idx = snap.menu!.candidates.findIndex((entry) => entry.objectKindId === candidate.objectKindId && entry.sourceVortexIndex === candidate.sourceVortexIndex);
              puzzle3dBrushUiStore.setSnapshot({ preview: snap.preview, menu: null });
              window.dispatchEvent(new CustomEvent("puzzle3d-brush-pick-candidate", { detail: { index: idx >= 0 ? idx : 0 } }));
            }}
          >
            {label}
          </button>
        );
      })}
    </div>
  );
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

function AttractionWindowBridge() {
  const reg = useRegistry();
  const invalidate = useThree((s) => s.invalidate);
  const attractionBusy = reg.attractionDragActive || reg.attractionIndirectPickAwait !== null;
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
  const { commitMarqueeSelection } = useRegistryInteraction();
  const marquee = useRegistryMarquee();
  const gl = useThree((state) => state.gl);
  const invalidate = useThree((state) => state.invalidate);
  const gestureRef = reactHostPort.useRef<{
    readonly pointerId: number;
    readonly startX: number;
    readonly startY: number;
    active: boolean;
    path: ScreenPoint[];
  } | null>(null);
  reactHostPort.useEffect(() => {
    const canvas = gl.domElement;
    if (!canvas) {
      return;
    }
    const resetOverlay = () => {
      puzzle3dMarqueeOverlayStore.setSnapshot(MARQUEE_OVERLAY_IDLE);
    };
    const cancelGesture = () => {
      gestureRef.current = null;
      resetOverlay();
    };
    puzzle3dMarqueeGestureCancel = cancelGesture;
    const onPointerDown = (event: PointerEvent) => {
      if (event.button !== 0 || reg.attractionDragActive || reg.attractionIndirectPickAwait !== null || puzzle3dRelocateDragActiveRef.current || puzzle3dBrushToolActiveRef.current) {
        return;
      }
      gestureRef.current = { pointerId: event.pointerId, startX: event.clientX, startY: event.clientY, active: false, path: [{ x: event.clientX, y: event.clientY }] };
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
      gestureRef.current = { ...gesture, active: true, path };
      puzzle3dMarqueeOverlayStore.setSnapshot({
        active: true,
        method: marquee.selectionMethod,
        start: { x: gesture.startX, y: gesture.startY },
        current: { x: event.clientX, y: event.clientY },
        path,
      });
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
  }, [commitMarqueeSelection, invalidate, marquee.selectionMethod, reg]);
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

function Puzzle3dMarqueeOverlay(props: { readonly rootRef: MutableRefObject<HTMLDivElement | null> }) {
  const overlay = reactHostPort.useSyncExternalStore(puzzle3dMarqueeOverlayStore.subscribe, puzzle3dMarqueeOverlayStore.getSnapshot, puzzle3dMarqueeOverlayStore.getSnapshot);
  const [origin, setOrigin] = reactHostPort.useState<ScreenPoint>({ x: 0, y: 0 });
  reactHostPort.useLayoutEffect(() => {
    const update = () => {
      const rect = props.rootRef.current?.getBoundingClientRect();
      if (rect) {
        setOrigin({ x: rect.left, y: rect.top });
      }
    };
    update();
    window.addEventListener("resize", update);
    return () => window.removeEventListener("resize", update);
  }, [overlay.active, props.rootRef]);
  if (!overlay.active || !overlay.start || !overlay.current) {
    return null;
  }
  const toLocal = (point: ScreenPoint) => ({ x: point.x - origin.x, y: point.y - origin.y });
  if (overlay.method === "lasso" && overlay.path.length >= 2) {
    const points = overlay.path.map((point) => `${toLocal(point).x},${toLocal(point).y}`).join(" ");
    return (
      <svg className="pointer-events-none absolute inset-0 h-full w-full overflow-visible" aria-hidden>
        <polyline points={points} fill="color-mix(in oklab, var(--color-primary) 12%, transparent)" stroke="var(--color-primary)" strokeWidth={1.5} />
      </svg>
    );
  }
  const start = toLocal(overlay.start);
  const current = toLocal(overlay.current);
  const left = Math.min(start.x, current.x);
  const top = Math.min(start.y, current.y);
  const width = Math.abs(current.x - start.x);
  const height = Math.abs(current.y - start.y);
  const crossing = marqueeIsCrossing(overlay.start.x, overlay.current.x);
  return (
    <svg className="pointer-events-none absolute inset-0 h-full w-full overflow-visible" aria-hidden>
      <rect
        x={left}
        y={top}
        width={width}
        height={height}
        fill={crossing ? "color-mix(in oklab, var(--color-primary) 10%, transparent)" : "color-mix(in oklab, var(--color-primary) 16%, transparent)"}
        stroke="var(--color-primary)"
        strokeWidth={1.5}
        strokeDasharray={crossing ? "4 3" : undefined}
      />
    </svg>
  );
}

function AttractionRubberBand() {
  const reg = useRegistry();
  const geo = reactHostPort.useMemo(() => {
    const g = new BufferGeometry();
    g.setAttribute("position", new Float32BufferAttribute(new Float32Array(6), 3));
    return g;
  }, []);
  const mat = reactHostPort.useMemo(() => new LineBasicMaterial({ color: 0xf472b6, transparent: true, opacity: 0.92, depthTest: false }), []);
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

function RegistryProvider({
  children,
  lodRef,
  kindCatalogs,
  kindCompatibility,
  blockedVortexFullIds,
  proximityRadius,
  proximityRelocateEnabled = true,
  selectionMode,
  relocateMode,
  selection: controlledSelection,
  onSelect,
  onConnect,
  onProximityConnect,
  onIndirectConnect,
  onAttractionCompatibleObjects,
  onAttractionTargetRing,
  onRelocate,
  attractionSession: externalAttractionSession,
  selectionMethod = "rectangle",
  marqueeSelectableKinds = { object: true, vortex: true, attraction: true },
}: {
  children: ReactNode;
  lodRef: MutableRefObject<number>;
  kindCatalogs: KindCatalogBundle | undefined;
  kindCompatibility: readonly KindCompatEntry[] | undefined;
  blockedVortexFullIds: ReadonlySet<string>;
  proximityRadius: number;
  proximityRelocateEnabled?: boolean;
  selectionMode: SelectionMode;
  relocateMode: RelocateMode;
  selection?: SelectionSnapshot;
  onSelect?: (snap: SelectionSnapshot) => void;
  onConnect?: (p: AttractionPayload) => void;
  onProximityConnect?: (p: AttractionPayload) => void;
  onIndirectConnect?: (p: AttractionPayload) => void;
  onAttractionCompatibleObjects?: (p: AttractionCompatibleObjectsPayload) => void;
  onAttractionTargetRing?: (p: AttractionTargetRingPayload) => void;
  onRelocate?: (p: RelocatePayload) => void;
  attractionSession?: AttractionSessionSnapshot | null;
  selectionMethod?: SelectionMethod;
  marqueeSelectableKinds?: MarqueeSelectableKinds;
}) {
  const selectionStoreRef = reactHostPort.useRef<SelectionSnapshotStore>();
  if (!selectionStoreRef.current) {
    selectionStoreRef.current = createSelectionSnapshotStore(controlledSelection ?? EMPTY_SELECTION_SNAPSHOT);
  }
  const selectionStore = selectionStoreRef.current;
  const controlledSelectionRef = reactHostPort.useRef(controlledSelection);
  controlledSelectionRef.current = controlledSelection;
  const onSelectRef = reactHostPort.useRef(onSelect);
  onSelectRef.current = onSelect;
  const selectionModeRef = reactHostPort.useRef(selectionMode);
  selectionModeRef.current = selectionMode;
  const selectionMethodRef = reactHostPort.useRef(selectionMethod);
  selectionMethodRef.current = selectionMethod;
  const marqueeKindsRef = reactHostPort.useRef(marqueeSelectableKinds);
  marqueeKindsRef.current = marqueeSelectableKinds;
  const marqueeAttractionsRef = reactHostPort.useRef<readonly AttractionProps[]>([]);
  reactHostPort.useEffect(() => {
    if (controlledSelection !== undefined) {
      selectionStore.setSnapshot(controlledSelection);
    }
  }, [controlledSelection, selectionStore]);
  const publishSelection = reactHostPort.useCallback(
    (snap: SelectionSnapshot) => {
      selectionStore.setSnapshot(snap);
      const primary = primarySelectionObjectId(snap);
      activeRelocateObjectIdRef.current = primary;
      const controlled = controlledSelectionRef.current;
      if (controlled !== undefined) {
        if (!selectionSnapshotsEqual(controlled, snap)) {
          onSelectRef.current?.(snap);
        }
        return;
      }
      onSelectRef.current?.(snap);
    },
    [selectionStore],
  );

  const commitSelection = reactHostPort.useCallback(
    (pick: SelectionPick) => {
      const current = selectionStore.getSnapshot();
      const snap = mergeSelection(selectionModeRef.current, current, pick);
      publishSelection(snap);
    },
    [publishSelection, selectionStore],
  );

  const commitMarqueeSelection = reactHostPort.useCallback(
    (args: {
      readonly startX: number;
      readonly startY: number;
      readonly endX: number;
      readonly endY: number;
      readonly path: readonly ScreenPoint[];
      readonly modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean };
    }) => {
      const env = attractionThreeRef.current;
      if (!env) {
        return;
      }
      const domRect = env.gl.domElement.getBoundingClientRect();
      const crossing = marqueeIsCrossing(args.startX, args.endX);
      const screenRect = screenRectFromClientPoints(args.startX, args.startY, args.endX, args.endY);
      const polygon =
        selectionMethodRef.current === "lasso" && args.path.length >= 3
          ? args.path
          : [
              { x: screenRect.left, y: screenRect.top },
              { x: screenRect.right, y: screenRect.top },
              { x: screenRect.right, y: screenRect.bottom },
              { x: screenRect.left, y: screenRect.bottom },
            ];
      const candidates: MarqueeCandidate[] = [];
      for (const [id, group] of objectGroupMap.current) {
        if (!group) {
          continue;
        }
        const points = projectObjectGroupToScreenPoints(group, env.camera, domRect);
        if (points.length) {
          candidates.push({ kind: "object", id, points });
        }
      }
      for (const [fullId, getter] of vortexGettersRef.current) {
        const world = getter();
        if (!world) {
          continue;
        }
        const point = projectWorldToClient(world, env.camera, domRect);
        if (point) {
          candidates.push({ kind: "vortex", id: fullId, points: [point] });
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
        if (points.length) {
          candidates.push({ kind: "attraction", id: attraction.id, points });
        }
      }
      const incoming = marqueeSelectionFromCandidates({
        method: selectionMethodRef.current,
        crossing,
        rect: selectionMethodRef.current === "rectangle" ? screenRect : null,
        polygon,
        kinds: marqueeKindsRef.current,
        candidates,
      });
      const mode = marqueeModeFromModifiers(args.modifiers);
      const current = selectionStore.getSnapshot();
      publishSelection(mergeSelectionSnapshot(mode, current, incoming));
      puzzle3dMarqueeSuppressClickRef.current = true;
      window.setTimeout(() => {
        puzzle3dMarqueeSuppressClickRef.current = false;
      }, 0);
    },
    [publishSelection, selectionStore],
  );

  const setSelectedObjectIds = reactHostPort.useCallback(
    (ids: readonly string[] | ((prev: readonly string[]) => readonly string[])) => {
      const current = selectionStore.getSnapshot();
      const resolvedObjectIds = typeof ids === "function" ? ids(current.objectIds) : ids;
      const mode = selectionModeRef.current;
      const snap: SelectionSnapshot =
        mode === "default"
          ? { objectIds: resolvedObjectIds, vortexIds: [], attractionIds: [] }
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
  reactHostPort.useEffect(() => {
    if (controlledSelection === undefined) {
      return;
    }
    activeRelocateObjectIdRef.current = primarySelectionObjectId(controlledSelection);
  }, [controlledSelection]);
  const liveSelection = reactHostPort.useSyncExternalStore(selectionStore.subscribe, selectionStore.getSnapshot, selectionStore.getSnapshot);
  const selectedObjectIds = liveSelection.objectIds;
  const activeRelocateObjectId = primarySelectionObjectId(liveSelection);
  const [attractionDragActive, setAttractionDragActive] = reactHostPort.useState(false);
  const [attractionDragAttractingFullId, setAttractionDragAttractingFullId] = reactHostPort.useState<string | null>(null);
  const [attractionCompatibleAttractedFullIds, setAttractionCompatibleAttractedFullIds] = reactHostPort.useState<ReadonlySet<string>>(new Set());
  const [attractionHoverRingFullId, setAttractionHoverRingFullId] = reactHostPort.useState<string | null>(null);
  const [attractionIndirectPickAwait, setAttractionIndirectPickAwait] = reactHostPort.useState<AttractionIndirectPickAwait | null>(null);
  const [hoverTarget, setHoverTarget] = reactHostPort.useState<HoverTarget | null>(null);

  const setHover = reactHostPort.useCallback((target: HoverTarget) => {
    setHoverTarget((prev) => (puzzle3dHoverTargetsEqual(prev, target) ? prev : target));
  }, []);

  const clearHover = reactHostPort.useCallback((target: HoverTarget) => {
    setHoverTarget((prev) => (puzzle3dHoverTargetsEqual(prev, target) ? null : prev));
  }, []);

  const clearHoverAll = reactHostPort.useCallback(() => {
    setHoverTarget((prev) => (prev === null ? prev : null));
  }, []);

  const isHovered = reactHostPort.useCallback((target: HoverTarget) => puzzle3dHoverTargetsEqual(hoverTarget, target), [hoverTarget]);

  const clearSelection = reactHostPort.useCallback(() => {
    clearHoverAll();
    setActiveRelocateObjectId(null);
    const empty = EMPTY_SELECTION_SNAPSHOT;
    selectionStore.setSnapshot(empty);
    const controlled = controlledSelectionRef.current;
    if (controlled !== undefined) {
      if (controlled.objectIds.length === 0 && controlled.vortexIds.length === 0 && controlled.attractionIds.length === 0) {
        return;
      }
      onSelectRef.current?.(empty);
      return;
    }
    onSelectRef.current?.(empty);
  }, [clearHoverAll, selectionStore, setActiveRelocateObjectId]);

  const vortexGettersRef = reactHostPort.useRef(new Map<string, VortexGetter>());
  const vortexMetaRef = reactHostPort.useRef(new Map<string, VortexBindingMeta>());
  const vortexPickRef = reactHostPort.useRef(new Map<string, Object3D>());
  const objectGroupMap = reactHostPort.useRef(new Map<string, Group | null>());
  const objectKindsRef = reactHostPort.useRef(new Map<string, string | undefined>());
  const indirectPickRef = reactHostPort.useRef<AttractionIndirectPickAwait | null>(null);

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

  const registerObject = reactHostPort.useCallback((id: string, objectKind: string | undefined, group: Group | null) => {
    objectGroupMap.current.set(id, group);
    objectKindsRef.current.set(id, objectKind);
  }, []);

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
    onAttractionTargetRing?.({ attracting: "", objectId: null, vortexFullIds: [] });
  }, [onAttractionTargetRing]);

  reactHostPort.useEffect(() => {
    if (attractionSessionRef.current) {
      return;
    }
    const ext = externalAttractionSession;
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
    onAttractionTargetRing?.({
      attracting: ext.attracting,
      objectId: ext.ringObjectId,
      vortexFullIds: ext.ringVortexFullIds,
    });
  }, [externalAttractionSession, attractionDragAttractingFullId, onAttractionTargetRing]);

  const beginAttractionDragFromVortex = reactHostPort.useCallback(
    (fullId: string, objectId: string, objectKind: string | undefined, vortexKind: string | undefined) => {
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
      onAttractionCompatibleObjects?.({ attracting: fullId, objectIds: [...objectIds] });
    },
    [blockedVortexFullIds, kindCatalogs, kindCompatibility, onAttractionCompatibleObjects],
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
        onAttractionTargetRing?.({
          attracting: session.attractingFullId,
          objectId: meta?.objectId ?? null,
          vortexFullIds: ring ? [ring] : [],
        });
      } else {
        onAttractionTargetRing?.({ attracting: session.attractingFullId, objectId: null, vortexFullIds: [] });
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
    [blockedVortexFullIds, collectPickRoots, lodRef, onAttractionTargetRing],
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
        onConnect?.(p);
        onProximityConnect?.(p);
        cancelAttractionDrag();
        return;
      }

      const attractingFull = session.attractingFullId;
      for (const h of hits) {
        const vf = readVortexFullIdFromObject(h.object);
        if (vf && vf !== attractingFull && session.compat.has(vf) && !blockedVortexFullIds.has(vf) && vortexMetaRef.current.get(vf)?.objectId !== session.attractingObjectId) {
          onConnect?.({ attracting: attractingFull, attracted: vf });
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
            onConnect?.(p);
            onIndirectConnect?.(p);
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
            onAttractionTargetRing?.({
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
    [blockedVortexFullIds, cancelAttractionDrag, collectPickRoots, onConnect, onIndirectConnect, onAttractionTargetRing, onProximityConnect],
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
          onConnect?.(p);
          onIndirectConnect?.(p);
          cancelAttractionDrag();
          ev?.stopImmediatePropagation();
          return;
        }
      }
      cancelAttractionDrag();
    },
    [cancelAttractionDrag, collectPickRoots, onConnect, onIndirectConnect],
  );

  const attachAttractionThreeEnv = reactHostPort.useCallback((env: { camera: Camera; gl: WebGLRenderer; scene: ThreeScene } | null) => {
    attractionThreeRef.current = env;
  }, []);

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
      relocateMode,
      selectedObjectIds,
      selectionMode,
      activeRelocateObjectId,
      beginAttractionDragFromVortex,
      cancelAttractionDrag,
      findNearestProximityRelocate,
      onSelect,
      onConnect,
      onProximityConnect,
      onIndirectConnect,
      onAttractionCompatibleObjects,
      onAttractionTargetRing,
      onRelocate,
      attachAttractionThreeEnv,
      updateAttractionPointer,
      commitAttractionPointer,
      updateIndirectPickPointer,
      commitIndirectPickPointerDown,
      attractionEndWorldRef,
    }),
    [
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
      relocateMode,
      selectedObjectIds,
      selectionMode,
      activeRelocateObjectId,
      beginAttractionDragFromVortex,
      cancelAttractionDrag,
      findNearestProximityRelocate,
      onSelect,
      onConnect,
      onProximityConnect,
      onIndirectConnect,
      onAttractionCompatibleObjects,
      onAttractionTargetRing,
      onRelocate,
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
      commitMarqueeSelection,
      setSelectedObjectIds,
      setActiveRelocateObjectId,
      clearSelection,
    }),
    [clearSelection, commitMarqueeSelection, commitSelection, selectionMode, setActiveRelocateObjectId, setSelectedObjectIds],
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
      setHover,
      clearHover,
      clearHoverAll,
      isHovered,
    }),
    [hoverTarget, setHover, clearHover, clearHoverAll, isHovered],
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
                {children}
                <HoverMissBridge />
                <HoverInvalidateBridge />
                <SelectionInvalidateBridge />
                <SelectionMissBridge />
              </RegistryDragContext.Provider>
            </RegistryHoverContext.Provider>
          </RegistryMarqueeContext.Provider>
        </RegistryInteractionContext.Provider>
      </RegistryCoreContext.Provider>
    </SelectionStoreContext.Provider>
  );
}

function Chunks({ chunkSize, maxDistance, children }: { chunkSize: number; maxDistance: number; children: ReactNode }) {
  const buckets = reactHostPort.useMemo(() => {
    const map = new Map<string, ReactNode[]>();
    Children.forEach(children, (child) => {
      if (!isValidElement(child)) return;
      const p = child.props as { origin?: Vec3 };
      if (!p?.origin) return;
      const k = chunkKey(p.origin, chunkSize);
      const arr = map.get(k) ?? [];
      arr.push(child);
      map.set(k, arr);
    });
    return map;
  }, [children, chunkSize]);

  const visible = useVisibleChunkKeys(buckets.keys(), chunkSize, maxDistance);
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

function splitChunkedSceneChildren(children: ReactNode): { chunked: ReactNode[]; rest: ReactNode[] } {
  const chunked: ReactNode[] = [];
  const rest: ReactNode[] = [];
  Children.forEach(children, (c) => {
    if (isValidElement(c) && (c.props as { origin?: Vec3 }).origin !== undefined) chunked.push(c);
    else rest.push(c);
  });
  return { chunked, rest };
}

/** @emoji 🎞️ Kicks one R3F frame when `frameloop="demand"` so LOD and meshes initialize without user input. */
function DemandFrameloopKick(): null {
  const invalidate = useThree((state) => state.invalidate);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [invalidate]);
  return null;
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
      const target = controls?.target;
      const cad = puzzle3dClientToGridPlaneCad({
        clientX: event.clientX,
        clientY: event.clientY,
        camera,
        canvas: gl.domElement,
        gridSnapEnabled: lod.gridSnapEnabled,
        gridStepWorld: lod.gridStepWorld,
        gridPlanePointThree: target ? [target.x, target.y, target.z] : undefined,
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
    puzzle3dFixtureDropPointerToCadRef.current = (clientX, clientY) => {
      const target = controls?.target;
      return puzzle3dClientToGridPlaneCad({
        clientX,
        clientY,
        camera,
        canvas: gl.domElement,
        gridSnapEnabled: lod.gridSnapEnabled,
        gridStepWorld: lod.gridStepWorld,
        gridPlanePointThree: target ? [target.x, target.y, target.z] : undefined,
      });
    };
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
  const { camera: camProp, chunkSize = 256, proximityRadius = 12, proximityRelocateEnabled = true, children, brushActive = false, onBrushPlace, kindCatalogs, kindCompatibility, fixtureDragDrop, onFixtureDrop, sceneFixture, puzzle3dRootRef, fixtureDragActive, setFixtureDragActive, fixtureDragDepthRef } = props;
  reactHostPort.useEffect(() => {
    puzzle3dBrushToolActiveRef.current = brushActive;
    if (!brushActive) {
      puzzle3dBrushUiStore.setSnapshot(BRUSH_UI_IDLE);
      puzzle3dBrushVortexHoverRef.current = false;
    }
    return () => {
      puzzle3dBrushToolActiveRef.current = false;
    };
  }, [brushActive]);
  const lodRef = reactHostPort.useRef<number>(DEFAULT_MANUAL_LOD);
  const [puzzle3dCamera, setCamera] = reactHostPort.useState<ThreePerspectiveCamera | null>(null);
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
  const autoFitCamera = props.autoFitCamera !== false;
  const autoFitBehavior = props.autoFitBehavior ?? "initial";
  const { chunked, rest } = reactHostPort.useMemo(() => splitChunkedSceneChildren(children), [children]);
  const blockedFallback = props.blockedVortexFullIds ?? EMPTY_BLOCKED_VORTICES;
  const blocked = useLiveBlockedVortexFullIds(blockedFallback);
  return (
    <RegistryProvider
      lodRef={lodRef}
      kindCatalogs={props.kindCatalogs}
      kindCompatibility={props.kindCompatibility}
      blockedVortexFullIds={blocked}
      proximityRadius={proximityRadius}
      proximityRelocateEnabled={proximityRelocateEnabled}
      selectionMode={props.selectionMode ?? "default"}
      relocateMode={props.relocateMode ?? "translate"}
      selection={props.selection}
      selectionMethod={props.selectionMethod ?? "rectangle"}
      marqueeSelectableKinds={props.marqueeSelectableKinds ?? { object: true, vortex: true, attraction: true }}
      onSelect={props.onSelect}
      onConnect={props.onConnect}
      onProximityConnect={props.onProximityConnect}
      onIndirectConnect={props.onIndirectConnect}
      onAttractionCompatibleObjects={props.onAttractionCompatibleObjects}
      onAttractionTargetRing={props.onAttractionTargetRing}
      onRelocate={props.onRelocate}
      attractionSession={props.attractionSession}
    >
      <LodBridge
        lodRef={lodRef}
        distanceReference={distanceReference}
        gridFactor={gridFactor}
        gridSnapEnabled={gridSnapEnabled}
        showLodGrid={showLodGrid}
        automaticLod={automaticLod}
        depthVariableLod={depthVariableLod}
        manualLod={manualLod}
        onLodChange={props.onLodChange}
      >
        <PerspectiveCamera ref={setCamera} makeDefault near={0.2} far={500_000} fov={50} />
        <CameraSeed camera={puzzle3dCamera} position={pos} target={tgt} />
        <OrbitGated camera={puzzle3dCamera} onCamera={props.onCamera} zoom={zoom} />
        {autoFitCamera ? <AutoFit behavior={autoFitBehavior} zoom={zoom} onCamera={props.onCamera} /> : null}
        <AttractionThreeBinder />
        <AttractionWindowBridge />
        <MarqueeBridge />
        {fixtureDragDrop ?? Boolean(onFixtureDrop) ? (
          <FixtureDropPointerBridge
            enabled
            onFixtureDrop={onFixtureDrop}
            rootRef={puzzle3dRootRef}
            setFixtureDragActive={setFixtureDragActive}
            fixtureDragDepthRef={fixtureDragDepthRef}
          />
        ) : null}
        {fixtureDragDrop ?? Boolean(onFixtureDrop) ? <FixtureDropPreview kindCatalogs={kindCatalogs} sceneFixture={sceneFixture} /> : null}
        {brushActive ? <BrushSession brushActive={brushActive} onBrushPlace={onBrushPlace} kindCatalogs={kindCatalogs} kindCompatibility={kindCompatibility} /> : null}
        <AttractionRubberBand />
        <ambientLight intensity={0.45} />
        <directionalLight position={[120, 180, 80]} intensity={0.85} />
        <Chunks chunkSize={chunkSize} maxDistance={maxDist}>
          {chunked}
        </Chunks>
        <group data-puzzle3d-unchunked>{rest}</group>
      </LodBridge>
    </RegistryProvider>
  );
}

export interface PlayCanvasProps {
  readonly fixture: FixtureV1;
  readonly proximityRelocateEnabled?: boolean;
  readonly kindCatalogs?: KindCatalogBundle;
  readonly kindCompatibility?: readonly KindCompatEntry[];
  readonly blockedVortexFullIds?: ReadonlySet<string>;
  readonly lodTag?: number;
  readonly lodProps?: Pick<CanvasProps, "automaticLod" | "depthVariableLod" | "lod">;
  readonly relocateMode?: RelocateMode;
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
  readonly onIndirectConnect?: () => void;
  readonly onProximityConnect?: () => void;
  readonly onLodChange?: (lod: number) => void;
  readonly onCamera?: (s: CameraState) => void;
  readonly onAttractionCompatibleObjects?: () => void;
  readonly onAttractionTargetRing?: () => void;
  readonly brushActive?: boolean;
  readonly onBrushPlace?: (payload: BrushPlacePayload) => void;
  readonly fixtureDragDrop?: boolean;
  readonly onFixtureDrop?: (detail: Puzzle3dFixtureDropDetail) => void;
}

/** @emoji 🎬 Puzzle 3D play canvas: {@link Canvas3D} cabled to {@link ObjectStateProvider} and {@link Objects}. */
export function PlayCanvas(props: PlayCanvasProps): React.ReactElement {
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
  return (
    <Canvas3D
      className="absolute inset-0"
      camera={props.fixture.camera}
      domain={props.fixture.domain}
      chunkSize={props.chunkSize}
      kindCatalogs={props.kindCatalogs}
      kindCompatibility={props.kindCompatibility}
      blockedVortexFullIds={props.blockedVortexFullIds}
      proximityRadius={props.proximityRadius}
      proximityRelocateEnabled={props.proximityRelocateEnabled}
      relocateMode={props.relocateMode}
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
      onConnect={handleConnect}
      onRelocate={handleRelocate}
      onIndirectConnect={onIndirectConnect}
      onProximityConnect={onProximityConnect}
      onAttractionCompatibleObjects={onAttractionCompatibleObjects}
      onAttractionTargetRing={onAttractionTargetRing}
      brushActive={props.brushActive}
      onBrushPlace={props.onBrushPlace}
      fixtureDragDrop={props.fixtureDragDrop}
      onFixtureDrop={props.onFixtureDrop}
      sceneFixture={props.fixture}
      {...props.lodProps}
    >
      <Objects relocate={props.relocateMode} />
      <AttractionTreeRoots />
      <MarqueeAttractionSource />
      <PlayTestBridge setSelectedId={props.setSelectedId} />
    </Canvas3D>
  );
}

export function Canvas3D(props: CanvasProps & { className?: string; style?: CSSProperties }) {
  const { children, className, style, onLodChange, domain = DEFAULT_DOMAIN, fixtureDragDrop, onFixtureDrop, ...rest } = props;
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
    <div
      ref={rootRef}
      className={`${className ?? ""}${fixtureDragActive ? " ring-primary ring-2 ring-inset" : ""}`.trim()}
      style={{ width: "100%", height: "100%", touchAction: "none", overscrollBehavior: "contain", ...style }}
      data-puzzle3d-fixture-drag-active={fixtureDragActive ? "true" : undefined}
      onContextMenu={(event) => {
        if (puzzle3dRightDragActiveRef.current || (puzzle3dBrushToolActiveRef.current && puzzle3dBrushVortexHoverRef.current)) {
          event.preventDefault();
        }
      }}
      data-puzzle3d-domain={domain}
      data-puzzle3d-root
      data-puzzle3d-lod={shellLod}
    >
      <Canvas frameloop="demand" gl={{ antialias: true }} dpr={[1, 2]}>
        <DemandFrameloopKick />
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
      </Canvas>
      <Puzzle3dMarqueeOverlay rootRef={rootRef} />
      <Puzzle3dBrushContextMenu rootRef={rootRef} />
    </div>
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
  const { describe, expect, it } = import.meta.vitest;
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
  describe("cadVec3ToThree", () => {
    it("maps CAD Z-up to Three Y-up", () => {
      const zUp = cadVec3ToThree([0, 0, 1]);
      expect(zUp[0]).toBeCloseTo(0, 5);
      expect(zUp[1]).toBeCloseTo(1, 5);
      expect(zUp[2]).toBeCloseTo(0, 5);
      const yFront = cadVec3ToThree([0, 1, 0]);
      expect(yFront[0]).toBeCloseTo(0, 5);
      expect(yFront[1]).toBeCloseTo(0, 5);
      expect(yFront[2]).toBeCloseTo(-1, 5);
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
    it("matches object id and parent of selected vortex", () => {
      expect(objectMatchesSelection("a", { objectIds: ["a"], vortexIds: [], attractionIds: [] })).toBe(true);
      expect(objectMatchesSelection("b", { objectIds: [], vortexIds: ["b:link"], attractionIds: [] })).toBe(true);
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
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: [] });
      expect(count).toBe(1);
      expect(store.getSnapshot().objectIds).toEqual(["a"]);
      store.setSnapshot({ objectIds: ["a"], vortexIds: [], attractionIds: [] });
      expect(count).toBe(1);
      unsubscribe();
    });
  });
  describe("puzzle3dObjectKindVorticesFromKitConnectors", () => {
    it("keeps two connectors with the same vortexKind at different CAD positions", () => {
      const vortexKinds: VortexKind[] = [{ id: "core rectangular bottom", name: "core rectangular bottom", color: "#000" }];
      const handleRows = [{ id: "semio.kit.handle.core-rect-bottom", name: "core rectangular bottom" }];
      const label = (hk: string) => puzzle3dVortexKindLabelFromHandleKind(hk, vortexKinds, handleRows);
      const vortices = puzzle3dObjectKindVorticesFromKitConnectors(
        [
          { point: { x: -7.5, y: -7.7, z: 7.5 }, direction: { x: 0, y: 0, z: 1 }, port: { handleKind: handleRows[0]!.id } },
          { point: { x: -18.6, y: -7.7, z: 7.5 }, direction: { x: 0, y: 0, z: 1 }, port: { handleKind: handleRows[0]!.id } },
        ],
        label,
      );
      expect(vortices).toHaveLength(2);
      expect(vortices[0]?.vortexKind).toBe("core rectangular bottom");
      expect(vortices[0]?.position).toEqual([-7.5, -7.7, 7.5]);
      expect(vortices[1]?.position).toEqual([-18.6, -7.7, 7.5]);
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
      expect(selected?.lineColor).toMatch(/primary|#ff344f/i);
      expect(hovered?.lineColor).toMatch(/hover-base|#7b827d/i);
      expect(highlighted?.lineColor).toMatch(/secondary|#34d1bf/i);
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
  describe("marqueeSelectionFromCandidates", () => {
    const rect = screenRectFromClientPoints(0, 0, 100, 100);
    it("window mode requires full enclosure", () => {
      const snap = marqueeSelectionFromCandidates({
        method: "rectangle",
        crossing: false,
        rect,
        polygon: [],
        kinds: { object: true, vortex: true, attraction: true },
        candidates: [
          { kind: "object", id: "inside", points: [{ x: 10, y: 10 }, { x: 20, y: 20 }] },
          { kind: "object", id: "partial", points: [{ x: 90, y: 90 }, { x: 120, y: 120 }] },
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
        candidates: [{ kind: "object", id: "partial", points: [{ x: 90, y: 90 }, { x: 120, y: 120 }] }],
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
          { kind: "object", id: "obj", points: [{ x: 10, y: 10 }] },
          { kind: "vortex", id: "a:v1", points: [{ x: 12, y: 12 }] },
        ],
      });
      expect(snap.objectIds).toEqual([]);
      expect(snap.vortexIds).toEqual(["a:v1"]);
    });
  });
  describe("mergeSelectionSnapshot", () => {
    it("replaces on default and inverts membership on invertive", () => {
      const current = { objectIds: ["a"], vortexIds: [], attractionIds: [] };
      const incoming = { objectIds: ["b"], vortexIds: [], attractionIds: [] };
      expect(mergeSelectionSnapshot("default", current, incoming).objectIds).toEqual(["b"]);
      expect(mergeSelectionSnapshot("invertive", current, incoming).objectIds.sort()).toEqual(["a", "b"]);
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
    it("puzzle3dClientToGridPlaneCad uses orbit grid height not CAD Z=0", () => {
      const camera = new ThreePerspectiveCamera(50, 1, 0.1, 100_000);
      const anchorCad: Vec3 = [0, 0, 40];
      const anchorThree = cadVec3ToThree(anchorCad);
      camera.position.set(anchorThree[0] + 240, anchorThree[1] + 180, anchorThree[2] + 120);
      camera.lookAt(anchorThree[0], anchorThree[1], anchorThree[2]);
      camera.updateMatrixWorld();
      const canvas = document.createElement("canvas");
      canvas.getBoundingClientRect = () => ({ left: 0, top: 0, width: 800, height: 600, right: 800, bottom: 600, x: 0, y: 0, toJSON: () => ({}) }) as DOMRect;
      const atOriginPlane = puzzle3dClientToGridPlaneCad({
        clientX: 400,
        clientY: 300,
        camera,
        canvas,
        gridPlaneAnchorCad: [0, 0, 0],
      });
      const atGridPlane = puzzle3dClientToGridPlaneCad({
        clientX: 400,
        clientY: 300,
        camera,
        canvas,
        gridPlaneAnchorCad: anchorCad,
      });
      expect(atGridPlane[2] - atOriginPlane[2]).toBeGreaterThan(30);
      expect(Math.abs(atGridPlane[2] - 40)).toBeLessThan(2);
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
    const brushCatalogs: KindCatalogBundle = {
      objects: [
        {
          id: "J",
          meshUrl: "/meshes/capsule_J.glb",
          vortices: [{ vortexKind: "door capsule east", position: [-1.3, -1.25, 0], direction: [-1, 0, 0], radius: 0.36 }],
        },
        {
          id: "Tambour",
          meshUrl: "/meshes/tambour.glb",
          vortices: [{ vortexKind: "door tambour east", position: [0.9, 2.75, 0.2], direction: [0, 1, 0], radius: 0.36 }],
        },
      ],
      vortices: [
        { id: "door capsule east", defaultCableKind: "cable.link" },
        { id: "door tambour east", defaultCableKind: "cable.link" },
      ],
      cables: [{ id: "cable.link", defaultAttractionKind: "puzzle3d.attraction.link" }],
    };
    const brushCompat: readonly KindCompatEntry[] = [{ bidirectional: true, specificity: "vortex", source: "door capsule east", target: "door tambour east" }];
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
    it("brushCompatibleCandidates filters by kind compatibility", () => {
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour east" };
      const list = brushCompatibleCandidates(target, brushCatalogs, brushCompat);
      expect(list.some((entry) => entry.objectKindId === "J")).toBe(true);
      expect(list.some((entry) => entry.objectKindId === "Tambour")).toBe(false);
    });
    it("boxesIntersect detects overlapping axis-aligned boxes", () => {
      const a = new Box3(new Vector3(0, 0, 0), new Vector3(2, 2, 2));
      const b = new Box3(new Vector3(1, 1, 1), new Vector3(3, 3, 3));
      const c = new Box3(new Vector3(4, 4, 4), new Vector3(5, 5, 5));
      expect(boxesIntersect(a, b)).toBe(true);
      expect(boxesIntersect(a, c)).toBe(false);
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
            vortices: [{ id: "host:v0", vortexKind: "door tambour east", position: [0.9, 2.75, 0.2], direction: [0, 1, 0] }],
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
          objectKindId: "J",
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
}
