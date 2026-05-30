// #region 🧲Header
/** @emoji 🔗 `@puzzle/5d/react` — paired 2d + 3d puzzle 5d surfaces and play harness (monolith). */
// #endregion 🧲Header

// #region 🔌Adapters
import { reactHostPort, type ContextMenuItem } from "@ui/react";
import type { ReactElement } from "react";

/** @emoji 🔗 Unified puzzle 5d model with 2d WASM + 3d R3F projections and a shared {@link Store}. */

import {
  BoardCanvas,
  DEFAULT_BOARD_GRID_FACTOR,
  DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
  Edge, Handle, Node, Wire,
  BUILTIN_PORT_HANDLE_KIND,
  fixtureMetaKindCatalogBundle,
  parseBoardFixtureV1,
  type CameraState as BoardCameraState,
  type BoardCanvasProps,
  type BoardFixtureV1,
  type BoardForceGraphLayoutOptions,
  type KindCatalogBundle as Puzzle2dKindCatalogBundle,
  type KindCompatEntry as Puzzle2dKindCompatEntry,
  type BoardLinkSessionSnapshot,
  type EdgeKind as Puzzle2dEdgeKind,
  type HandleKind as Puzzle2dHandleKind,
  type NodeKind as Puzzle2dNodeKind,
  type WireKind as Puzzle2dWireKind,
} from "@puzzle/2d/react";
import {
  ObjectStateProvider as Puzzle3dPartStateProvider,
  Objects as Puzzle3dParts,
  Attractions as Puzzle3dTies,
  Canvas3D as Puzzle3dCanvas,
  blockedVortexFullIdsFromAttractions,
  parseFixtureV1,
  useObjectConnect as usePuzzle3dPartConnect,
  useObjectRelocate as usePuzzle3dPartRelocate,
  type AttractionKind as Puzzle3dFastenerKind,
  type AttractionSessionSnapshot,
  type CableKind as Puzzle3dRopeKind,
  type DomainKind,
  type FixtureV1 as Puzzle3dFixtureV1,
  type KindCatalogBundle as Puzzle3dKindCatalogBundle,
  type KindCompatEntry as Puzzle3dKindCompatEntry,
  type ObjectKind as Puzzle3dPartKind,
  type RelocateMode as Puzzle3dRelocateMode,
  type SelectionSnapshot as Puzzle3dSelectionSnapshot,
  type VortexKind as Puzzle3dGripKind,
  type CanvasProps as Puzzle3dCanvasProps,
} from "../../3d/react/index.tsx";
// #endregion 🔌Adapters

//#region 🔖PairedPolicy
/** @emoji 🔗 How a tie is committed: direct handle pick, indirect ring finish, or proximity snap. */
export type ConnectGestureKind = "direct" | "indirect" | "proximity";

/** @emoji ↔️ True only for {@link ConnectGestureKind.indirect} — the gesture mirrored in {@link ConnectSession}. */
export function connectGestureCrossSurface(kind: ConnectGestureKind): boolean {
  return kind === "indirect";
}

/** @emoji 📶 Flat (@puzzle/2d) uses six discrete WASM draw LOD tiers from zoom thresholds. */
export const PUZZLE_5D_2D_LOD_TIER_COUNT = 6 as const;

/** @emoji 📶 Volume (@puzzle/3d) uses continuous / camera-driven LOD (`automaticLod`, depth-variable, manual slider). */
export type Puzzle3dLodPolicy = "continuous";

/** @emoji 🧲 Flat proximity: overlapping compatible handles while **dragging** a node (pointer-up snap). */
export const PUZZLE_5D_2D_PROXIMITY_GESTURE: ConnectGestureKind = "proximity";

/** @emoji 🧲 Volume proximity: compatible anchor within radius while **relocating** a part (gumball release). */
export const PUZZLE_5D_3D_PROXIMITY_GESTURE: ConnectGestureKind = "proximity";

/** @emoji 🎯 Indirect link/attraction: start on one surface, finish on a compatible ring on either surface. */
export const PUZZLE_5D_INDIRECT_CONNECT_GESTURE: ConnectGestureKind = "indirect";
//#endregion 🔖PairedPolicy

//#region 🔖Model
export type PresentationMode = "2d" | "3d";

export interface Puzzle2dAnchorAspect {
  readonly angle: number;
  readonly anchorKind: string;
  readonly color?: string;
  readonly iconKind?: string;
  readonly radius?: number;
}

export interface Puzzle3dAnchorAspect {
  readonly position: readonly [number, number, number];
  readonly direction?: readonly [number, number, number];
  readonly radius?: number;
  readonly label?: string;
  readonly handleMeshUrl?: string;
}

export interface AnchorV1 {
  readonly id: string;
  readonly anchorKind: string;
  readonly puzzle2d?: Puzzle2dAnchorAspect;
  readonly puzzle3d?: Puzzle3dAnchorAspect;
}

export interface NodeAspect {
  readonly x: number;
  readonly y: number;
  readonly shape: "circle" | "rectangle";
  readonly radius?: number;
  readonly width?: number;
  readonly height?: number;
  readonly text?: string;
  readonly textAlignment?: "top" | "center" | "bottom" | "left" | "right";
  readonly textAutofit?: boolean;
  readonly textFontFamily?: string;
  readonly textFontSize?: number;
  readonly iconKind?: string;
  readonly hidden?: boolean;
}

export interface Puzzle3dPartAspect {
  readonly origin: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: number | readonly [number, number, number];
  readonly meshUrl: string;
  readonly label?: string;
  readonly wormhole?: boolean;
}

export interface PartV1 {
  readonly id: string;
  readonly partKind?: string;
  readonly puzzle2d?: NodeAspect;
  readonly puzzle3d?: Puzzle3dPartAspect;
  readonly anchors: readonly AnchorV1[];
}

export interface TieV1 {
  readonly id: string;
  readonly source: string;
  readonly target: string;
  readonly tieKind?: string;
}

/** @emoji 🔗 In-progress **indirect** connect only (never proximity); synced across 2d {@link BoardLinkSessionSnapshot} and 3d {@link AttractionSessionSnapshot}. */
export interface ConnectSession {
  readonly origin: PresentationMode;
  readonly sourceAnchor: string;
  readonly endX: number;
  readonly endY: number;
  readonly end3d: readonly [number, number, number];
  readonly compatiblePartIds: readonly string[];
  readonly ringPartId: string | null;
  readonly ringAnchorIds: readonly string[];
}

export interface SelectionSnapshot {
  readonly partIds: readonly string[];
  readonly anchorIds: readonly string[];
}

export interface V1 {
  readonly schema: "puzzle.5d/v1";
  readonly label?: string;
  readonly domain: DomainKind;
  readonly meta?: Record<string, unknown>;
  readonly kindCatalogs?: KindCatalogBundle;
  readonly kindCompatibility?: readonly KindCompatEntry[];
  readonly camera2d: BoardCameraState;
  readonly camera3d: Puzzle3dFixtureV1["camera"];
  readonly parts: readonly PartV1[];
  readonly ties: readonly TieV1[];
}

export const PUZZLE_5D_ANCHOR_ID_SEPARATOR = ":";

/** @emoji 🔗 Builds a full anchor id `partId:anchorId`. */
export function anchorFullId(partId: string, anchorId: string): string {
  return `${partId}${PUZZLE_5D_ANCHOR_ID_SEPARATOR}${anchorId}`;
}

/** @emoji 🔍 Splits a full anchor id into part and anchor local ids. */
export function parseAnchorFullId(fullId: string): { partId: string; anchorId: string } | null {
  const i = fullId.indexOf(PUZZLE_5D_ANCHOR_ID_SEPARATOR);
  if (i <= 0 || i >= fullId.length - 1) return null;
  return { partId: fullId.slice(0, i), anchorId: fullId.slice(i + 1) };
}

/** @emoji ✅ Validates unified puzzle 5d JSON. */
export function parseV1(raw: unknown): V1 | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "puzzle.5d/v1") return null;
  if (!Array.isArray(r.parts) || !Array.isArray(r.ties)) return null;
  const domain = typeof r.domain === "string" ? (r.domain as DomainKind) : "architecture";
  const flatCam = r.camera2d as BoardCameraState | undefined;
  const volumeCam = r.camera3d as Puzzle3dFixtureV1["camera"] | undefined;
  if (!flatCam || !volumeCam) return null;
  return {
    schema: "puzzle.5d/v1",
    domain,
    camera2d: flatCam,
    camera3d: volumeCam,
    parts: r.parts as PartV1[],
    ties: r.ties as TieV1[],
    ...(typeof r.label === "string" ? { label: r.label } : {}),
    ...(r.meta && typeof r.meta === "object" ? { meta: r.meta as Record<string, unknown> } : {}),
    ...(r.kindCatalogs && typeof r.kindCatalogs === "object" ? { kindCatalogs: normalizeKindCatalogBundle(r.kindCatalogs) } : {}),
    ...(Array.isArray(r.kindCompatibility) ? { kindCompatibility: r.kindCompatibility as KindCompatEntry[] } : {}),
  };
}

/** @emoji 🔀 Builds {@link V1} by merging 2d and 3d fixtures (same part ids unite). */
export function compose5d(fixture2d: BoardFixtureV1, fixture3d: Puzzle3dFixtureV1): V1 {
  const partsMap = new Map<string, PartV1>();
  for (const node of fixture2d.nodes) {
    const anchors: AnchorV1[] = node.handles.map((h) => {
      const parsed = parseAnchorFullId(h.id);
      const localId = parsed?.anchorId ?? h.id;
      return {
        id: localId,
        anchorKind: h.handleKind,
        puzzle2d: {
          angle: h.angle,
          anchorKind: h.handleKind,
          ...(h.color !== undefined ? { color: h.color } : {}),
          ...(h.iconKind !== undefined ? { iconKind: h.iconKind } : {}),
          ...(h.radius !== undefined ? { radius: h.radius } : {}),
        },
      };
    });
    const flatAspect: NodeAspect =
      node.shape === "rectangle"
        ? {
            x: node.x,
            y: node.y,
            shape: "rectangle",
            width: node.width,
            height: node.height,
            ...(node.text !== undefined ? { text: node.text } : {}),
            ...(node.textAlignment !== undefined ? { textAlignment: node.textAlignment } : {}),
            ...(node.textAutofit === true ? { textAutofit: true } : {}),
            ...(node.textFontFamily !== undefined ? { textFontFamily: node.textFontFamily } : {}),
            ...(node.textFontSize !== undefined ? { textFontSize: node.textFontSize } : {}),
            ...(node.iconKind !== undefined ? { iconKind: node.iconKind } : {}),
          }
        : {
            x: node.x,
            y: node.y,
            shape: "circle",
            radius: node.radius,
            ...(node.text !== undefined ? { text: node.text } : {}),
            ...(node.textAlignment !== undefined ? { textAlignment: node.textAlignment } : {}),
            ...(node.textAutofit === true ? { textAutofit: true } : {}),
            ...(node.textFontFamily !== undefined ? { textFontFamily: node.textFontFamily } : {}),
            ...(node.textFontSize !== undefined ? { textFontSize: node.textFontSize } : {}),
            ...(node.iconKind !== undefined ? { iconKind: node.iconKind } : {}),
          };
    partsMap.set(node.id, {
      id: node.id,
      ...(node.nodeKind !== undefined ? { partKind: node.nodeKind } : {}),
      puzzle2d: flatAspect,
      anchors,
    });
  }
  for (const obj of fixture3d.objects) {
    const volumeAspect: Puzzle3dPartAspect = {
      origin: obj.origin,
      meshUrl: obj.meshUrl,
      ...(obj.orientation !== undefined ? { orientation: obj.orientation } : {}),
      ...(obj.scale !== undefined ? { scale: obj.scale } : {}),
      ...(obj.label !== undefined ? { label: obj.label } : {}),
      ...(obj.wormhole === true ? { wormhole: true } : {}),
    };
    const volumeAnchors: AnchorV1[] = obj.vortices.map((v) => {
      const parsed = parseAnchorFullId(v.id.includes(":") ? v.id : anchorFullId(obj.id, v.id));
      const localId = parsed?.anchorId ?? v.id;
      return {
        id: localId,
        anchorKind: v.vortexKind ?? BUILTIN_PORT_HANDLE_KIND,
        puzzle3d: {
          position: v.position,
          ...(v.direction !== undefined ? { direction: v.direction } : {}),
          ...(v.radius !== undefined ? { radius: v.radius } : {}),
          ...(v.label !== undefined ? { label: v.label } : {}),
          ...(v.handleMeshUrl !== undefined ? { handleMeshUrl: v.handleMeshUrl } : {}),
        },
      };
    });
    const existing = partsMap.get(obj.id);
    if (existing) {
      const anchorById = new Map(existing.anchors.map((a) => [a.id, a]));
      for (const a of volumeAnchors) {
        const prev = anchorById.get(a.id);
        anchorById.set(a.id, prev ? { ...prev, puzzle3d: a.puzzle3d, anchorKind: a.anchorKind } : a);
      }
      partsMap.set(obj.id, {
        ...existing,
        ...(obj.objectKind !== undefined ? { partKind: obj.objectKind } : {}),
        puzzle3d: volumeAspect,
        anchors: [...anchorById.values()],
      });
    } else {
      partsMap.set(obj.id, {
        id: obj.id,
        ...(obj.objectKind !== undefined ? { partKind: obj.objectKind } : {}),
        puzzle3d: volumeAspect,
        anchors: volumeAnchors,
      });
    }
  }
  const ties: TieV1[] = [];
  const tieIds = new Set<string>();
  for (const edge of fixture2d.edges) {
    if (tieIds.has(edge.id)) continue;
    tieIds.add(edge.id);
    ties.push({ id: edge.id, source: edge.source, target: edge.target });
  }
  for (const att of fixture3d.attractions) {
    if (tieIds.has(att.id)) continue;
    tieIds.add(att.id);
    ties.push({
      id: att.id,
      source: att.attracting,
      target: att.attracted,
      ...(att.attractionKind !== undefined ? { tieKind: att.attractionKind } : {}),
    });
  }
  const meta = {
    ...(fixture2d.meta ?? {}),
    ...(fixture3d.meta ?? {}),
  };
  const kindCatalogs = kindCatalogsFromMetas({ meta2d: fixture2d.meta, meta3d: fixture3d.meta });
  const kindCompatibility = kindCompatibilityFromMetas({ meta2d: fixture2d.meta, meta3d: fixture3d.meta });
  return {
    schema: "puzzle.5d/v1",
    domain: fixture3d.domain,
    camera2d: { ...fixture2d.camera },
    camera3d: { ...fixture3d.camera },
    parts: [...partsMap.values()],
    ties,
    ...(Object.keys(meta).length > 0 ? { meta } : {}),
    ...(kindCatalogs ? { kindCatalogs } : {}),
    ...(kindCompatibility.length > 0 ? { kindCompatibility } : {}),
  };
}

/** @emoji 📐 Projects {@link V1} to a 2d fixture for WASM rendering. */
export function project2d(model: V1): BoardFixtureV1 {
  const nodes = model.parts
    .filter((p) => p.puzzle2d)
    .map((p) => {
      const aspect2d = p.puzzle2d!;
      const handles = p.anchors
        .filter((a) => a.puzzle2d)
        .map((a) => ({
          id: anchorFullId(p.id, a.id),
          angle: a.puzzle2d!.angle,
          handleKind: a.puzzle2d!.anchorKind,
          ...(a.puzzle2d!.color !== undefined ? { color: a.puzzle2d!.color } : {}),
          ...(a.puzzle2d!.iconKind !== undefined ? { iconKind: a.puzzle2d!.iconKind } : {}),
          ...(a.puzzle2d!.radius !== undefined ? { radius: a.puzzle2d!.radius } : {}),
        }));
      if (aspect2d.shape === "rectangle") {
        return {
          id: p.id,
          shape: "rectangle" as const,
          x: aspect2d.x,
          y: aspect2d.y,
          width: aspect2d.width ?? 40,
          height: aspect2d.height ?? 40,
          handles,
          ...(p.partKind !== undefined ? { nodeKind: p.partKind } : {}),
          ...(aspect2d.text !== undefined ? { text: aspect2d.text } : {}),
          ...(aspect2d.textAlignment !== undefined ? { textAlignment: aspect2d.textAlignment } : {}),
          ...(aspect2d.textAutofit === true ? { textAutofit: true } : {}),
          ...(aspect2d.textFontFamily !== undefined ? { textFontFamily: aspect2d.textFontFamily } : {}),
          ...(aspect2d.textFontSize !== undefined ? { textFontSize: aspect2d.textFontSize } : {}),
          ...(aspect2d.iconKind !== undefined ? { iconKind: aspect2d.iconKind } : {}),
        };
      }
      return {
        id: p.id,
        shape: "circle" as const,
        x: aspect2d.x,
        y: aspect2d.y,
        radius: aspect2d.radius ?? 20,
        handles,
        ...(p.partKind !== undefined ? { nodeKind: p.partKind } : {}),
        ...(aspect2d.text !== undefined ? { text: aspect2d.text } : {}),
        ...(aspect2d.textAlignment !== undefined ? { textAlignment: aspect2d.textAlignment } : {}),
        ...(aspect2d.textAutofit === true ? { textAutofit: true } : {}),
        ...(aspect2d.textFontFamily !== undefined ? { textFontFamily: aspect2d.textFontFamily } : {}),
        ...(aspect2d.textFontSize !== undefined ? { textFontSize: aspect2d.textFontSize } : {}),
        ...(aspect2d.iconKind !== undefined ? { iconKind: aspect2d.iconKind } : {}),
      };
    });
  return {
    schema: "puzzle.2d.fixture/v1",
    camera: { ...model.camera2d },
    nodes,
    edges: model.ties.map((b) => ({ id: b.id, source: b.source, target: b.target })),
    ...(model.meta ? { meta: model.meta } : {}),
  };
}

/** @emoji 📐 Projects {@link V1} to a @puzzle/3d fixture for 3d rendering. */
export function project3d(model: V1): Puzzle3dFixtureV1 {
  const objects = model.parts
    .filter((p) => p.puzzle3d)
    .map((p) => {
      const s = p.puzzle3d!;
      return {
        id: p.id,
        meshUrl: s.meshUrl,
        origin: s.origin,
        ...(p.partKind !== undefined ? { objectKind: p.partKind } : {}),
        ...(s.orientation !== undefined ? { orientation: s.orientation } : {}),
        ...(s.scale !== undefined ? { scale: s.scale } : {}),
        ...(s.label !== undefined ? { label: s.label } : {}),
        ...(s.wormhole === true ? { wormhole: true } : {}),
        vortices: p.anchors
          .filter((a) => a.puzzle3d)
          .map((a) => ({
            id: anchorFullId(p.id, a.id),
            position: a.puzzle3d!.position,
            ...(a.anchorKind ? { vortexKind: a.anchorKind } : {}),
            ...(a.puzzle3d!.direction !== undefined ? { direction: a.puzzle3d!.direction } : {}),
            ...(a.puzzle3d!.radius !== undefined ? { radius: a.puzzle3d!.radius } : {}),
            ...(a.puzzle3d!.label !== undefined ? { label: a.puzzle3d!.label } : {}),
            ...(a.puzzle3d!.handleMeshUrl !== undefined ? { handleMeshUrl: a.puzzle3d!.handleMeshUrl } : {}),
          })),
      };
    });
  return {
    schema: "puzzle.3d.fixture/v1",
    domain: model.domain,
    camera: { ...model.camera3d },
    objects,
    attractions: model.ties.map((b) => ({
      id: b.id,
      attracting: b.source as `${string}:${string}`,
      attracted: b.target as `${string}:${string}`,
      ...(b.tieKind !== undefined ? { attractionKind: b.tieKind } : {}),
    })),
    ...(model.meta ? { meta: model.meta } : {}),
  };
}
//#endregion 🔖Model

//#region 🔖Store
export interface StoreSnapshot {
  readonly model: V1;
  readonly selection: SelectionSnapshot;
  readonly connectSession: ConnectSession | null;
  readonly cameras: Readonly<Record<string, { readonly "2d": BoardCameraState; readonly "3d": Puzzle3dFixtureV1["camera"] }>>;
}

export class Store {
  private listeners = new Set<() => void>();
  private snapshot: StoreSnapshot;

  constructor(model: V1) {
    this.snapshot = {
      model,
      selection: { partIds: [], anchorIds: [] },
      connectSession: null,
      cameras: {},
    };
  }

  getSnapshot = (): StoreSnapshot => this.snapshot;

  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  private emit(): void {
    for (const l of this.listeners) l();
  }

  private setSnapshot(next: StoreSnapshot): void {
    this.snapshot = next;
    this.emit();
  }

  /** @emoji 🧬  document from the current store snapshot. */
  read(): V1 {
    return this.snapshot.model;
  }

  get2dCamera(instanceId: string): BoardCameraState {
    return this.snapshot.cameras[instanceId]?.["2d"] ?? this.snapshot.model.camera2d;
  }

  get3dCamera(instanceId: string): Puzzle3dFixtureV1["camera"] {
    return this.snapshot.cameras[instanceId]?.["3d"] ?? this.snapshot.model.camera3d;
  }

  set2dCamera(instanceId: string, camera: BoardCameraState): void {
    const prev = this.snapshot.cameras[instanceId];
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, camera2d: camera },
      cameras: {
        ...this.snapshot.cameras,
        [instanceId]: { "2d": camera, "3d": prev?.["3d"] ?? this.snapshot.model.camera3d },
      },
    });
  }

  set3dCamera(instanceId: string, camera: Puzzle3dFixtureV1["camera"]): void {
    const prev = this.snapshot.cameras[instanceId];
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, camera3d: camera },
      cameras: {
        ...this.snapshot.cameras,
        [instanceId]: { "2d": prev?.["2d"] ?? this.snapshot.model.camera2d, "3d": camera },
      },
    });
  }

  setSelection(selection: SelectionSnapshot): void {
    this.setSnapshot({ ...this.snapshot, selection });
  }

  /** @emoji 🔗 Sets cross-surface indirect preview state; callers must not use this for proximity snaps. */
  setConnectSession(session: ConnectSession | null): void {
    this.setSnapshot({ ...this.snapshot, connectSession: session });
  }

  applyNodeMove(partId: string, x: number, y: number): void {
    const parts = this.snapshot.model.parts.map((p) => {
      if (p.id !== partId || !p.puzzle2d) return p;
      return { ...p, puzzle2d: { ...p.puzzle2d, x, y } };
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }

  apply3dRelocate(partId: string, origin: readonly [number, number, number], orientation: readonly [number, number, number, number]): void {
    const parts = this.snapshot.model.parts.map((p) => {
      if (p.id !== partId || !p.puzzle3d) return p;
      return { ...p, puzzle3d: { ...p.puzzle3d, origin, orientation } };
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }

  applyTie(source: string, target: string, tieKind?: string): void {
    const id = crypto.randomUUID();
    const ties: TieV1[] = [...this.snapshot.model.ties, { id, source, target, ...(tieKind ? { tieKind } : {}) }];
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, ties },
      connectSession: null,
    });
  }

  replaceModel(model: V1): void {
    this.setSnapshot({
      ...this.snapshot,
      model,
      connectSession: null,
    });
  }
}

export function createStore(model: V1): Store {
  return new Store(model);
}

const StoreContext = reactHostPort.createContext<Store | null>(null);

export function StoreProvider(props: { readonly store: Store; readonly children: React.ReactNode }): ReactElement {
  return <StoreContext.Provider value={props.store}>{props.children}</StoreContext.Provider>;
}

export function useStore(): Store {
  const store = reactHostPort.useContext(StoreContext);
  if (!store) throw new Error("useStore requires StoreProvider");
  return store;
}

export function useSnapshot(): StoreSnapshot {
  const store = useStore();
  return reactHostPort.useSyncExternalStore(store.subscribe, store.getSnapshot, store.getSnapshot);
}
//#endregion 🔖Store

//#region 🔖FiveD
export const FIVE_D_ROOT_CLASS = "flex h-full min-h-0 flex-1 flex-col";

/** @emoji 📶 Flat-only LOD/grid defaults ({@link PUZZLE_5D_2D_LOD_TIER_COUNT} discrete tiers); do not pass to 3d {@link Puzzle3dCanvas}. */
export const FIVE_D_FLAT_LOD_DEFAULTS = {
  lodZoomThresholds: DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
  gridFactor: DEFAULT_BOARD_GRID_FACTOR,
  gridSnapEnabled: true,
} as const;

/** @emoji 🎛 3d chrome: continuous LOD comes from host `puzzle3d` props; proximity applies on relocate only. */
export const FIVE_D_3D_CHROME_DEFAULTS: Pick<Puzzle3dCanvasProps, "showLodGrid" | "proximityRadius" | "proximityRelocateEnabled" | "gridSnapEnabled"> = {
  showLodGrid: true,
  proximityRadius: 24,
  proximityRelocateEnabled: true,
  gridSnapEnabled: true,
};

/** @emoji 🖼️ Single puzzle 5d editor (`2d` = @puzzle/2d, `3d` = @puzzle/3d); pair via {@link StoreProvider}. */
export interface FiveDProps {
  readonly mode: PresentationMode;
  readonly instanceId: string;
  readonly className?: string;
  readonly lockedPartIds?: ReadonlySet<string>;
  readonly relocateMode?: Puzzle3dRelocateMode;
  /** 2d surface overrides; LOD uses discrete tiers unless `automaticLod` is set on the canvas. */
  readonly puzzle2d?: Omit<BoardCanvasProps, "children">;
  /** 3d surface overrides; LOD is continuous/camera-driven — not the flat six-tier scale. */
  readonly puzzle3d?: Omit<Puzzle3dCanvasProps, "children">;
}

function fiveDLinkSessionFromStore(session: ConnectSession | null): BoardLinkSessionSnapshot | null {
  if (!session) return null;
  return {
    source: session.sourceAnchor,
    endX: session.endX,
    endY: session.endY,
    compatiblePartIds: session.compatiblePartIds,
    ringPartId: session.ringPartId,
    ringAnchorIds: session.ringAnchorIds,
  };
}

function fiveDAttractionSessionFromStore(session: ConnectSession | null): AttractionSessionSnapshot | null {
  if (!session) return null;
  return {
    attracting: session.sourceAnchor,
    end: session.end3d,
    compatibleObjectIds: session.compatiblePartIds,
    ringObjectId: session.ringPartId,
    ringVortexFullIds: session.ringAnchorIds,
  };
}

function markers2dFromFixture(props: { readonly fixture: BoardFixtureV1; readonly lockedIds: ReadonlySet<string>; readonly selectedIds: ReadonlySet<string> }): ReactElement {
  const { fixture, lockedIds, selectedIds } = props;
  return (
    <>
      {fixture.nodes.map((node) =>
        node.shape === "rectangle" ? (
          <Node
            draggable={!lockedIds.has(node.id)}
            height={node.height}
            id={node.id}
            key={node.id}
            {...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
            shape="rectangle"
            selected={selectedIds.has(node.id)}
            text={node.text}
            textAlignment={node.textAlignment}
            textAutofit={node.textAutofit === true}
            textFontFamily={node.textFontFamily}
            textFontSize={node.textFontSize}
            width={node.width}
            x={node.x}
            y={node.y}
            {...(node.iconKind ? { iconKind: node.iconKind } : {})}
          >
            {node.handles.map((handle) => (
              <Handle angle={handle.angle} color={handle.color} handleKind={handle.handleKind} id={handle.id} key={handle.id} radius={handle.radius} selected={selectedIds.has(handle.id)} {...(handle.iconKind ? { iconKind: handle.iconKind } : {})} />
            ))}
          </Node>
        ) : (
          <Node
            draggable={!lockedIds.has(node.id)}
            id={node.id}
            key={node.id}
            {...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
            radius={node.radius}
            selected={selectedIds.has(node.id)}
            text={node.text}
            textAlignment={node.textAlignment}
            textAutofit={node.textAutofit === true}
            textFontFamily={node.textFontFamily}
            textFontSize={node.textFontSize}
            x={node.x}
            y={node.y}
            {...(node.iconKind ? { iconKind: node.iconKind } : {})}
          >
            {node.handles.map((handle) => (
              <Handle angle={handle.angle} color={handle.color} handleKind={handle.handleKind} id={handle.id} key={handle.id} radius={handle.radius} selected={selectedIds.has(handle.id)} {...(handle.iconKind ? { iconKind: handle.iconKind } : {})} />
            ))}
          </Node>
        ),
      )}
      {fixture.edges.map((edge) => (
        <Edge edgeKind={undefined} id={edge.id} key={edge.id} selected={selectedIds.has(edge.id)} source={edge.source} target={edge.target} />
      ))}
    </>
  );
}

const FiveD2d = reactHostPort.memo(function FiveD2d(props: FiveDProps) {
  const store = useStore();
  const snap = useSnapshot();
  const fixture2d = reactHostPort.useMemo(() => project2d(snap.model), [snap.model]);
  const locked = props.lockedPartIds ?? new Set<string>();
  const selectedIds = reactHostPort.useMemo(() => new Set([...snap.selection.partIds, ...snap.selection.anchorIds]), [snap.selection]);
  const markers = reactHostPort.useMemo(() => markers2dFromFixture({ fixture: fixture2d, lockedIds: locked, selectedIds }), [fixture2d, locked, selectedIds]);
  const camera = store.get2dCamera(props.instanceId);
  const extra2d = props.puzzle2d ?? {};
  const { onSelect: onSelectHost, onConnect: onConnectHost, onIndirectConnect: onIndirectConnectHost, onProximityConnect: onProximityConnectHost, onDrag: onDragHost, ...rest2d } = extra2d;
  const linkSession = fiveDLinkSessionFromStore(snap.connectSession);
  return (
    <div className={FIVE_D_ROOT_CLASS} data-five-d-indirect-active={snap.connectSession ? "true" : "false"} data-five-d-mode="2d" data-five-d-instance={props.instanceId}>
      <BoardCanvas
        camera={rest2d.camera ?? camera}
        className={["min-h-0 flex-1", props.className, rest2d.className].filter(Boolean).join(" ") || undefined}
        {...FIVE_D_FLAT_LOD_DEFAULTS}
        kindCatalogs={project2dKindCatalogs(snap.model.kindCatalogs)}
        kindCompatibility={snap.model.kindCompatibility}
        linkSession={linkSession}
        onCamera={(c) => store.set2dCamera(props.instanceId, c)}
        onConnect={(p) => {
          store.applyTie(p.source, p.target);
          onConnectHost?.(p);
        }}
        onDrag={(p) => {
          store.applyNodeMove(p.id, p.x, p.y);
          onDragHost?.(p);
        }}
        onIndirectConnect={(p) => {
          store.applyTie(p.source, p.target);
          onIndirectConnectHost?.(p);
        }}
        onLinkCompatibleNodes={(p) => {
          if (!p.source) {
            store.setConnectSession(null);
            return;
          }
          const prev = store.getSnapshot().connectSession;
          store.setConnectSession({
            origin: "2d",
            sourceAnchor: p.source,
            endX: prev?.endX ?? 0,
            endY: prev?.endY ?? 0,
            end3d: prev?.end3d ?? [0, 0, 0],
            compatiblePartIds: [...p.nodeIds],
            ringPartId: prev?.ringPartId ?? null,
            ringAnchorIds: prev?.ringAnchorIds ?? [],
          });
        }}
        onLinkTargetRing={(p) => {
          const prev = store.getSnapshot().connectSession;
          if (!p.source) {
            store.setConnectSession(null);
            return;
          }
          store.setConnectSession({
            origin: prev?.origin ?? "2d",
            sourceAnchor: p.source,
            endX: prev?.endX ?? 0,
            endY: prev?.endY ?? 0,
            end3d: prev?.end3d ?? [0, 0, 0],
            compatiblePartIds: prev?.compatiblePartIds ?? [],
            ringPartId: p.nodeId,
            ringAnchorIds: [...p.handleIds],
          });
        }}
        onProximityConnect={(p) => {
          store.applyTie(p.source, p.target);
          onProximityConnectHost?.(p);
        }}
        onSelect={(s) => {
          store.setSelection({ partIds: s.ids, anchorIds: [] });
          onSelectHost?.(s);
        }}
        {...rest2d}
      >
        {markers}
      </BoardCanvas>
    </div>
  );
});

const FiveD3dInner = reactHostPort.memo(function FiveD3dInner(props: FiveDProps) {
  const store = useStore();
  const snap = useSnapshot();
  const fixture3d = reactHostPort.useMemo(() => project3d(snap.model), [snap.model]);
  const extra3d = props.puzzle3d ?? {};
  const {
    onSelect: onSelectHost,
    onConnect: onConnectHost,
    onIndirectConnect: onIndirectConnectHost,
    onProximityConnect: onProximityConnectHost,
    onRelocate: onRelocateHost,
    onAttractionCompatibleObjects: onAttractionCompatibleObjectsHost,
    onAttractionTargetRing: onAttractionTargetRingHost,
    ...rest3d
  } = extra3d;
  const camera = store.get3dCamera(props.instanceId);
  const selectedObjectId = snap.selection.partIds[0] ?? null;
  const attractionSession = fiveDAttractionSessionFromStore(snap.connectSession);
  const onRelocate = usePuzzle3dPartRelocate();
  const onConnect = usePuzzle3dPartConnect();
  return (
    <Puzzle3dCanvas
      camera={rest3d.camera ?? camera}
      className={["min-h-0 flex-1", props.className, rest3d.className].filter(Boolean).join(" ") || undefined}
      {...FIVE_D_3D_CHROME_DEFAULTS}
      attractionSession={attractionSession}
      blockedVortexFullIds={blockedVortexFullIdsFromAttractions(fixture3d.attractions)}
      gridFactor={FIVE_D_FLAT_LOD_DEFAULTS.gridFactor}
      gridSnapEnabled={FIVE_D_FLAT_LOD_DEFAULTS.gridSnapEnabled}
      kindCatalogs={project3dKindCatalogs(snap.model.kindCatalogs)}
      kindCompatibility={snap.model.kindCompatibility as Puzzle3dKindCompatEntry[] | undefined}
      relocateMode={props.relocateMode ?? "translate"}
      onCamera={(c) => store.set3dCamera(props.instanceId, c)}
      onConnect={(p) => {
        onConnect?.(p);
        onConnectHost?.(p);
      }}
      onRelocate={(p) => {
        onRelocate?.(p);
        onRelocateHost?.(p);
      }}
      onAttractionCompatibleObjects={(p) => {
        if (!p.attracting) {
          store.setConnectSession(null);
          onAttractionCompatibleObjectsHost?.(p);
          return;
        }
        const prev = store.getSnapshot().connectSession;
        store.setConnectSession({
          origin: "3d",
          sourceAnchor: p.attracting,
          endX: prev?.endX ?? 0,
          endY: prev?.endY ?? 0,
          end3d: prev?.end3d ?? [0, 0, 0],
          compatiblePartIds: [...p.objectIds],
          ringPartId: prev?.ringPartId ?? null,
          ringAnchorIds: prev?.ringAnchorIds ?? [],
        });
        onAttractionCompatibleObjectsHost?.(p);
      }}
      onAttractionTargetRing={(p) => {
        const prev = store.getSnapshot().connectSession;
        if (!p.attracting) {
          store.setConnectSession(null);
          onAttractionTargetRingHost?.(p);
          return;
        }
        store.setConnectSession({
          origin: prev?.origin ?? "3d",
          sourceAnchor: p.attracting,
          endX: prev?.endX ?? 0,
          endY: prev?.endY ?? 0,
          end3d: prev?.end3d ?? [0, 0, 0],
          compatiblePartIds: prev?.compatiblePartIds ?? [],
          ringPartId: p.objectId,
          ringAnchorIds: [...p.vortexFullIds],
        });
        onAttractionTargetRingHost?.(p);
      }}
      onIndirectConnect={(p) => {
        store.applyTie(p.attracting, p.attracted);
        onIndirectConnectHost?.(p);
        onConnect?.(p);
      }}
      onProximityConnect={(p) => {
        store.applyTie(p.attracting, p.attracted);
        onProximityConnectHost?.(p);
      }}
      onSelect={(s: Puzzle3dSelectionSnapshot) => {
        store.setSelection({ partIds: [...s.objectIds], anchorIds: [...s.vortexIds] });
        onSelectHost?.(s);
      }}
      {...rest3d}
    >
      <Puzzle3dParts selectedObjectId={selectedObjectId} relocate={props.relocateMode ?? "translate"} />
      <Puzzle3dTies />
    </Puzzle3dCanvas>
  );
});

const FiveD3d = reactHostPort.memo(function FiveD3d(props: FiveDProps) {
  const snap = useSnapshot();
  const fixture3d = reactHostPort.useMemo(() => project3d(snap.model), [snap.model]);
  return (
    <div className={FIVE_D_ROOT_CLASS} data-five-d-indirect-active={snap.connectSession ? "true" : "false"} data-five-d-mode="3d" data-five-d-instance={props.instanceId}>
      <reactHostPort.Suspense fallback={<div className="flex min-h-0 flex-1 items-center justify-center p-4 text-sm text-muted-foreground">Loading meshes…</div>}>
        <Puzzle3dPartStateProvider fixture={fixture3d} onConnect={props.puzzle3d?.onConnect} onRelocate={props.puzzle3d?.onRelocate}>
          <FiveD3dInner {...props} />
        </Puzzle3dPartStateProvider>
      </reactHostPort.Suspense>
    </div>
  );
});

/** @emoji 🖼️ Single puzzle 5d surface (`2d` WASM or `3d` R3F); share state via {@link StoreProvider}. */
export const FiveD = reactHostPort.memo(function FiveD(props: FiveDProps) {
  if (props.mode === "2d") return <FiveD2d {...props} />;
  return <FiveD3d {...props} />;
});
//#endregion 🔖FiveD

//#region 🔖KindMeta
/** @emoji 🟠 Part-kind catalog row for unified puzzle 5d fixtures. */
export interface PartKind {
  color?: string;
  defaultGripKind?: string;
  defaultShapeProps?: Record<string, unknown>;
  icon?: string;
  id: string;
  label?: string;
  name: string;
  shape?: "circle" | "rectangle";
  stroke?: string;
}

/** @emoji 🎨 Grip-kind catalog row for unified puzzle 5d fixtures. */
export interface GripKind {
  color: string;
  defaultRopeKind?: string;
  id: string;
  label?: string;
  name: string;
}

/** @emoji 🪢 Fastener-kind catalog row for unified puzzle 5d fixtures. */
export interface FastenerKind {
  color?: string;
  defaultShapeProps?: Record<string, unknown>;
  id: string;
  label?: string;
  name: string;
  pattern?: string;
  shape?: "bezier" | "line";
  stroke?: string;
}

/** @emoji 🧵 Rope-kind catalog row for unified puzzle 5d fixtures. */
export interface RopeKind {
  defaultFastenerKind?: string;
  id: string;
  label?: string;
  name: string;
}

/** @emoji 📚 Unified puzzle 5d kind registries (`parts`, `grips`, `fasteners`, `ropes`). */
export interface KindCatalogBundle {
  fasteners?: readonly FastenerKind[];
  grips?: readonly GripKind[];
  parts?: readonly PartKind[];
  ropes?: readonly RopeKind[];
}

/** @emoji 🔗 One allowed directed pair between semantic kind ids in puzzle 5d fixtures. */
export interface KindCompatEntry {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: "edge" | "general" | "handle" | "node" | "wire" | "object" | "attraction" | "part" | "grip" | "fastener" | "rope" | "vortex" | "cable";
}

function isMetaRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function kindCatalogFrom2dMeta(meta: Record<string, unknown> | undefined): Puzzle2dKindCatalogBundle | undefined {
  return fixtureMetaKindCatalogBundle(meta);
}

function kindCatalogFrom3dMeta(meta: Record<string, unknown> | undefined): Puzzle3dKindCatalogBundle | undefined {
  if (!isMetaRecord(meta)) return undefined;
  const kc = meta.kindCatalogs;
  if (!kc || typeof kc !== "object" || Array.isArray(kc)) return undefined;
  return kc as Puzzle3dKindCatalogBundle;
}

function partKindFrom2dNode(row: Puzzle2dNodeKind): PartKind {
  return {
    id: row.id,
    name: row.name,
    ...(row.color !== undefined ? { color: row.color } : {}),
    ...(row.defaultHandleKind !== undefined ? { defaultGripKind: row.defaultHandleKind } : {}),
    ...(row.defaultShapeProps !== undefined ? { defaultShapeProps: row.defaultShapeProps } : {}),
    ...(row.icon !== undefined ? { icon: row.icon } : {}),
    ...(row.shape !== undefined ? { shape: row.shape } : {}),
    ...(row.stroke !== undefined ? { stroke: row.stroke } : {}),
  };
}

function gripKindFrom2dHandle(row: Puzzle2dHandleKind): GripKind {
  return {
    id: row.id,
    name: row.name,
    color: row.color,
    ...(row.defaultWireKind !== undefined ? { defaultRopeKind: row.defaultWireKind } : {}),
  };
}

function fastenerKindFrom2dEdge(row: Puzzle2dEdgeKind): FastenerKind {
  return {
    id: row.id,
    name: row.name,
    ...(row.color !== undefined ? { color: row.color } : {}),
    ...(row.defaultShapeProps !== undefined ? { defaultShapeProps: row.defaultShapeProps } : {}),
    ...(row.pattern !== undefined ? { pattern: row.pattern } : {}),
    ...(row.shape !== undefined ? { shape: row.shape } : {}),
    ...(row.stroke !== undefined ? { stroke: row.stroke } : {}),
  };
}

function ropeKindFrom2dWire(row: Puzzle2dWireKind): RopeKind {
  return {
    id: row.id,
    name: row.name,
    ...(row.defaultEdgeKind !== undefined ? { defaultFastenerKind: row.defaultEdgeKind } : {}),
  };
}

function partKindFrom3dObject(row: Puzzle3dPartKind): PartKind {
  return {
    id: row.id,
    name: row.name ?? row.id,
    ...(row.label !== undefined ? { label: row.label } : {}),
    ...(row.color !== undefined ? { color: row.color } : {}),
    ...(row.shape !== undefined ? { shape: row.shape as PartKind["shape"] } : {}),
  };
}

function gripKindFrom3dVortex(row: Puzzle3dGripKind): GripKind {
  return {
    id: row.id,
    name: row.name ?? row.id,
    color: row.color ?? "#94a3b8",
    ...(row.label !== undefined ? { label: row.label } : {}),
    ...(row.defaultCableKind !== undefined ? { defaultRopeKind: row.defaultCableKind } : {}),
  };
}

function fastenerKindFrom3dAttraction(row: Puzzle3dFastenerKind): FastenerKind {
  return {
    id: row.id,
    name: row.name ?? row.id,
    ...(row.label !== undefined ? { label: row.label } : {}),
  };
}

function ropeKindFrom3dCable(row: Puzzle3dRopeKind): RopeKind {
  return {
    id: row.id,
    name: row.name ?? row.id,
    ...(row.label !== undefined ? { label: row.label } : {}),
    ...(row.defaultAttractionKind !== undefined ? { defaultFastenerKind: row.defaultAttractionKind } : {}),
  };
}

/** @emoji 🔀 Normalizes raw `kindCatalogs` JSON into puzzle 5d bundle keys. */
export function normalizeKindCatalogBundle(raw: unknown): KindCatalogBundle | undefined {
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return undefined;
  const box = raw as Record<string, unknown>;
  if (box.parts || box.grips || box.fasteners || box.ropes) {
    return box as KindCatalogBundle;
  }
  const out: KindCatalogBundle = {};
  if (Array.isArray(box.nodes)) out.parts = (box.nodes as Puzzle2dNodeKind[]).map(partKindFrom2dNode);
  if (Array.isArray(box.handles)) out.grips = (box.handles as Puzzle2dHandleKind[]).map(gripKindFrom2dHandle);
  if (Array.isArray(box.edges)) out.fasteners = (box.edges as Puzzle2dEdgeKind[]).map(fastenerKindFrom2dEdge);
  if (Array.isArray(box.wires)) out.ropes = (box.wires as Puzzle2dWireKind[]).map(ropeKindFrom2dWire);
  if (Array.isArray(box.objects)) out.parts = (box.objects as Puzzle3dPartKind[]).map(partKindFrom3dObject);
  if (Array.isArray(box.vortices)) out.grips = (box.vortices as Puzzle3dGripKind[]).map(gripKindFrom3dVortex);
  if (Array.isArray(box.attractions)) out.fasteners = (box.attractions as Puzzle3dFastenerKind[]).map(fastenerKindFrom3dAttraction);
  if (Array.isArray(box.cables)) out.ropes = (box.cables as Puzzle3dRopeKind[]).map(ropeKindFrom3dCable);
  if (!out.parts && !out.grips && !out.fasteners && !out.ropes) return undefined;
  return out;
}

/** @emoji 📐 Projects puzzle 5d kind catalogs onto puzzle 2d bundle keys. */
export function project2dKindCatalogs(bundle: KindCatalogBundle | undefined): Puzzle2dKindCatalogBundle | undefined {
  if (!bundle) return undefined;
  const out: Puzzle2dKindCatalogBundle = {};
  if (bundle.parts?.length) {
    out.nodes = bundle.parts.map((part) => ({
      id: part.id,
      name: part.name,
      ...(part.color !== undefined ? { color: part.color } : {}),
      ...(part.defaultGripKind !== undefined ? { defaultHandleKind: part.defaultGripKind } : {}),
      ...(part.defaultShapeProps !== undefined ? { defaultShapeProps: part.defaultShapeProps } : {}),
      ...(part.icon !== undefined ? { icon: part.icon } : {}),
      ...(part.shape !== undefined ? { shape: part.shape } : {}),
      ...(part.stroke !== undefined ? { stroke: part.stroke } : {}),
    }));
  }
  if (bundle.grips?.length) {
    out.handles = bundle.grips.map((grip) => ({
      id: grip.id,
      name: grip.name,
      color: grip.color,
      ...(grip.defaultRopeKind !== undefined ? { defaultWireKind: grip.defaultRopeKind } : {}),
    }));
  }
  if (bundle.fasteners?.length) {
    out.edges = bundle.fasteners.map((fastener) => ({
      id: fastener.id,
      name: fastener.name,
      ...(fastener.color !== undefined ? { color: fastener.color } : {}),
      ...(fastener.defaultShapeProps !== undefined ? { defaultShapeProps: fastener.defaultShapeProps } : {}),
      ...(fastener.pattern !== undefined ? { pattern: fastener.pattern } : {}),
      ...(fastener.shape !== undefined ? { shape: fastener.shape } : {}),
      ...(fastener.stroke !== undefined ? { stroke: fastener.stroke } : {}),
    }));
  }
  if (bundle.ropes?.length) {
    out.wires = bundle.ropes.map((rope) => ({
      id: rope.id,
      name: rope.name,
      ...(rope.defaultFastenerKind !== undefined ? { defaultEdgeKind: rope.defaultFastenerKind } : {}),
    }));
  }
  return Object.keys(out).length ? out : undefined;
}

/** @emoji 📐 Projects puzzle 5d kind catalogs onto puzzle 3d bundle keys. */
export function project3dKindCatalogs(bundle: KindCatalogBundle | undefined): Puzzle3dKindCatalogBundle | undefined {
  if (!bundle) return undefined;
  const out: Puzzle3dKindCatalogBundle = {};
  if (bundle.parts?.length) {
    out.objects = bundle.parts.map((part) => ({
      id: part.id,
      name: part.name,
      ...(part.label !== undefined ? { label: part.label } : {}),
      ...(part.color !== undefined ? { color: part.color } : {}),
      ...(part.shape !== undefined ? { shape: part.shape } : {}),
    }));
  }
  if (bundle.grips?.length) {
    out.vortices = bundle.grips.map((grip) => ({
      id: grip.id,
      name: grip.name,
      color: grip.color,
      ...(grip.label !== undefined ? { label: grip.label } : {}),
      ...(grip.defaultRopeKind !== undefined ? { defaultCableKind: grip.defaultRopeKind } : {}),
    }));
  }
  if (bundle.fasteners?.length) {
    out.attractions = bundle.fasteners.map((fastener) => ({
      id: fastener.id,
      name: fastener.name,
      ...(fastener.label !== undefined ? { label: fastener.label } : {}),
    }));
  }
  if (bundle.ropes?.length) {
    out.cables = bundle.ropes.map((rope) => ({
      id: rope.id,
      name: rope.name,
      ...(rope.label !== undefined ? { label: rope.label } : {}),
      ...(rope.defaultFastenerKind !== undefined ? { defaultAttractionKind: rope.defaultFastenerKind } : {}),
    }));
  }
  return Object.keys(out).length ? out : undefined;
}

/** @emoji 📚 Reads `kindCompatibility` rows from fixture meta. */
export function kindCompatibilityRowsFromMeta(meta: Record<string, unknown> | undefined): KindCompatEntry[] {
  if (!isMetaRecord(meta)) return [];
  const arr = meta.kindCompatibility;
  if (!Array.isArray(arr)) return [];
  const out: KindCompatEntry[] = [];
  for (const entry of arr) {
    if (!isMetaRecord(entry)) continue;
    const source = typeof entry.source === "string" ? entry.source.trim() : "";
    const target = typeof entry.target === "string" ? entry.target.trim() : "";
    if (!source || !target) continue;
    const specificity =
      entry.specificity === "general" || entry.specificity === "node" || entry.specificity === "edge" || entry.specificity === "handle" || entry.specificity === "wire" || entry.specificity === "object" || entry.specificity === "attraction" || entry.specificity === "part" || entry.specificity === "grip" || entry.specificity === "fastener" || entry.specificity === "rope" || entry.specificity === "vortex" || entry.specificity === "cable"
        ? entry.specificity
        : undefined;
    out.push({
      source,
      target,
      ...(entry.bidirectional === true ? { bidirectional: true } : {}),
      ...(entry.important === true ? { important: true } : {}),
      ...(specificity ? { specificity } : {}),
    });
  }
  return out;
}

export function kindCatalogsFromMetas(inp: { readonly meta2d: Record<string, unknown> | undefined; readonly meta3d: Record<string, unknown> | undefined }): KindCatalogBundle | undefined {
  const fromFlat = kindCatalogFrom2dMeta(inp.meta2d);
  if (fromFlat) return normalizeKindCatalogBundle(fromFlat);
  const fromVolume = kindCatalogFrom3dMeta(inp.meta3d);
  if (fromVolume) return normalizeKindCatalogBundle(fromVolume);
  return undefined;
}

export function kindCompatibilityFromMetas(inp: { readonly meta2d: Record<string, unknown> | undefined; readonly meta3d: Record<string, unknown> | undefined }): readonly KindCompatEntry[] {
  const fromFlat = kindCompatibilityRowsFromMeta(inp.meta2d);
  if (fromFlat.length > 0) return fromFlat;
  return kindCompatibilityRowsFromMeta(inp.meta3d);
}

export function sharedKindsFromMetas(inp: { readonly meta2d: Record<string, unknown> | undefined; readonly meta3d: Record<string, unknown> | undefined }): Pick<
  typeof FIVE_D_FLAT_LOD_DEFAULTS,
  "lodZoomThresholds" | "gridFactor" | "gridSnapEnabled"
> & {
  kindCatalogs?: KindCatalogBundle;
  kindCompatibility?: readonly KindCompatEntry[];
} {
  return {
    ...FIVE_D_FLAT_LOD_DEFAULTS,
    kindCatalogs: kindCatalogsFromMetas(inp),
    kindCompatibility: kindCompatibilityFromMetas(inp),
  };
}
//#endregion 🔖KindMeta

//#region 🔖BoardLayout
/** @emoji 🔗 Default separator for 2d handle ids (`piece::connector`). */
export const FLAT_HANDLE_COMPOUND_SEPARATOR = "::";

/** @emoji 🔗 Builds a compound 2d handle id from two parts. */
export function flatHandleCompoundId(left: string, right: string, separator: string = FLAT_HANDLE_COMPOUND_SEPARATOR): string {
  return `${left}${separator}${right}`;
}

/** @emoji ┬¡ãÆ├Â├¼ Parses a compound board handle id into left/right parts. */
export function flatParseHandleCompoundId(value: string, separator: string = FLAT_HANDLE_COMPOUND_SEPARATOR): { left: string; right: string } | null {
  const separatorIndex = value.indexOf(separator);
  if (separatorIndex <= 0 || separatorIndex >= value.length - separator.length) return null;
  return {
    left: value.slice(0, separatorIndex),
    right: value.slice(separatorIndex + separator.length),
  };
}

/** @emoji ┬¡ãÆ├┤├ë Evenly distributes connector angles around a node rim (starts at top). */
export function flatHandleConnectorAngle(index: number, total: number): number {
  return -Math.PI / 2 + (index * Math.PI * 2) / Math.max(total, 1);
}

export type FlatGripSide = "top" | "right" | "bottom" | "left";

/** @emoji 📐 Flat grip snap side to handle angle (rectangle vs circle rim). */
export function flatGripAngle(side: FlatGripSide, shape: "circle" | "rectangle"): number {
  if (shape === "rectangle") {
    if (side === "top") return 0;
    if (side === "right") return Math.PI / 2;
    if (side === "bottom") return Math.PI;
    return (3 * Math.PI) / 2;
  }
  if (side === "right") return 0;
  if (side === "bottom") return Math.PI / 2;
  if (side === "left") return Math.PI;
  return -Math.PI / 2;
}

/** @emoji ┬¡ãÆ├┤├¼ Node center from top-left layout position and frame size. */
export function nodeCenterFromTopLeft(position: { readonly x: number; readonly y: number }, frame: { readonly width: number; readonly height: number }): { x: number; y: number } {
  return { x: position.x + frame.width / 2, y: position.y + frame.height / 2 };
}

/** @emoji ┬¡ãÆ├▓┬®┬┤┬®├à Diagram force-slider weights shared by sketchpad kit/design hosts. */
export interface DiagramForceWeights {
  readonly centerStrength: number;
  readonly linkDistance: number;
  readonly chargeStrength: number;
}

/** @emoji ┬¡ãÆ├▓┬®┬┤┬®├à Maps diagram force sliders to {@link layoutBoardFixtureForceGraph} options. */
export function puzzle2dForceGraphOptions(weights: DiagramForceWeights): BoardForceGraphLayoutOptions {
  return {
    centerX: 0,
    centerY: 0,
    gravity: weights.centerStrength,
    idealEdgeLength: weights.linkDistance,
    iterations: 280,
    randomSeed: 1,
    repulsionStrength: Math.abs(weights.chargeStrength),
  };
}

/** @emoji ┬¡ãÆ├┤├Ç Centers the board camera on the average of node centers. */
export function camera2dFromPartCenters(centers: readonly { x: number; y: number }[]): BoardCameraState {
  if (centers.length === 0) return { x: 0, y: 0, zoom: 1 };
  const avgX = centers.reduce((sum, point) => sum + point.x, 0) / centers.length;
  const avgY = centers.reduce((sum, point) => sum + point.y, 0) / centers.length;
  return { x: -avgX, y: -avgY, zoom: 1 };
}

/** @emoji ┬¡ãÆ├┤├¼ Writes WASM layout node centers back into top-left layout positions. */
export function flatApplyFixtureCentersToTopLeft<T extends { readonly id: string; readonly position: { x: number; y: number } }>(
  items: readonly T[],
  fixture: BoardFixtureV1,
  frameForItem: (item: T) => { width: number; height: number },
): T[] {
  const centerById = new Map(fixture.nodes.map((node) => [node.id, { x: node.x, y: node.y }]));
  return items.map((item) => {
    const center = centerById.get(item.id);
    if (!center) return item;
    const frame = frameForItem(item);
    return {
      ...item,
      position: { x: center.x - frame.width / 2, y: center.y - frame.height / 2 },
    };
  });
}
//#endregion 🔖BoardLayout

/** @emoji 🎛 Default 3d chrome for puzzle 5d surfaces (alias of {@link FIVE_D_3D_CHROME_DEFAULTS}). */
export function chrome3dDefaults(): typeof FIVE_D_3D_CHROME_DEFAULTS {
  return FIVE_D_3D_CHROME_DEFAULTS;
}

//#region 🔖BoardMarkers
export interface FlatWireRecord {
  readonly id: string;
  readonly source: string;
  readonly target?: string;
  readonly wireKind?: string;
  readonly endX?: number;
  readonly endY?: number;
  readonly hidden?: boolean;
}

/** @emoji 🧩 Builds board host markers from a board fixture (same static shape walk as board play). */
export function flatMarkersFromFixture(props: {
  readonly fixture: BoardFixtureV1;
  readonly lockedIds: ReadonlySet<string>;
  readonly selectedIds: ReadonlySet<string>;
  readonly contextMenuById: (id: string | null) => ContextMenuItem[];
  readonly wires: readonly FlatWireRecord[];
}): ReactElement {
  const { contextMenuById, fixture, lockedIds, selectedIds, wires } = props;
  return (
    <>
      {fixture.nodes.map((node) =>
        node.shape === "rectangle" ? (
          <Node
            contextMenu={contextMenuById(node.id)}
            draggable={!lockedIds.has(node.id)}
            height={node.height}
            id={node.id}
            key={node.id}
            {...(node.hidden === true ? { hidden: true } : {})}
            {...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
            shape="rectangle"
            selected={selectedIds.has(node.id)}
            text={node.text}
            textAlignment={node.textAlignment}
            textAutofit={node.textAutofit === true}
            textFontFamily={node.textFontFamily}
            textFontSize={node.textFontSize}
            width={node.width}
            x={node.x}
            y={node.y}
            {...(node.iconKind ? { iconKind: node.iconKind } : {})}
          >
            {node.handles.map((handle) => (
              <Handle
                angle={handle.angle}
                color={handle.color}
                contextMenu={contextMenuById(handle.id)}
                handleKind={handle.handleKind}
                {...(handle.hidden === true ? { hidden: true } : {})}
                id={handle.id}
                key={handle.id}
                radius={handle.radius}
                selected={selectedIds.has(handle.id)}
                {...(handle.iconKind ? { iconKind: handle.iconKind } : {})}
              />
            ))}
          </Node>
        ) : (
          <Node
            contextMenu={contextMenuById(node.id)}
            draggable={!lockedIds.has(node.id)}
            id={node.id}
            key={node.id}
            {...(node.hidden === true ? { hidden: true } : {})}
            {...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
            radius={node.radius}
            selected={selectedIds.has(node.id)}
            text={node.text}
            textAlignment={node.textAlignment}
            textAutofit={node.textAutofit === true}
            textFontFamily={node.textFontFamily}
            textFontSize={node.textFontSize}
            x={node.x}
            y={node.y}
            {...(node.iconKind ? { iconKind: node.iconKind } : {})}
          >
            {node.handles.map((handle) => (
              <Handle
                angle={handle.angle}
                color={handle.color}
                contextMenu={contextMenuById(handle.id)}
                handleKind={handle.handleKind}
                {...(handle.hidden === true ? { hidden: true } : {})}
                id={handle.id}
                key={handle.id}
                radius={handle.radius}
                selected={selectedIds.has(handle.id)}
                {...(handle.iconKind ? { iconKind: handle.iconKind } : {})}
              />
            ))}
          </Node>
        ),
      )}
      {fixture.edges.map((edge) => (
        <Edge contextMenu={contextMenuById(edge.id)} id={edge.id} key={edge.id} selected={selectedIds.has(edge.id)} source={edge.source} target={edge.target} />
      ))}
      {wires.map((wire) => (
        <Wire
          contextMenu={contextMenuById(wire.id)}
          {...(typeof wire.endX === "number" ? { endX: wire.endX } : {})}
          {...(typeof wire.endY === "number" ? { endY: wire.endY } : {})}
          {...(wire.hidden === true ? { hidden: true } : {})}
          id={wire.id}
          key={wire.id}
          selected={selectedIds.has(wire.id)}
          source={wire.source}
          {...(wire.target ? { target: wire.target } : {})}
          {...(wire.wireKind ? { wireKind: wire.wireKind } : {})}
        />
      ))}
    </>
  );
}
//#endregion 🔖BoardMarkers

export { DEFAULT_BOARD_GRID_FACTOR, DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS, blockedVortexFullIdsFromAttractions, parseBoardFixtureV1, parseFixtureV1 };

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("connectGestureCrossSurface", () => {
    it("only indirect syncs across 2d and 3d", () => {
      expect(connectGestureCrossSurface("indirect")).toBe(true);
      expect(connectGestureCrossSurface("direct")).toBe(false);
      expect(connectGestureCrossSurface("proximity")).toBe(false);
    });
  });

  describe("parseV1", () => {
    it("accepts unified puzzle 5d model", () => {
      const t = parseV1({
        schema: "puzzle.5d/v1",
        domain: "architecture",
        camera2d: { x: 0, y: 0, zoom: 1 },
        camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        parts: [],
        ties: [],
        label: "x",
      });
      expect(t?.schema).toBe("puzzle.5d/v1");
      expect(t?.label).toBe("x");
    });
  });
  describe("compose5d", () => {
    it("merges 2d nodes and 3d objects by id", () => {
      const fixture2d: BoardFixtureV1 = {
        schema: "puzzle.2d.fixture/v1",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [{ id: "p1", shape: "circle", x: 1, y: 2, radius: 10, handles: [{ id: "p1:h", angle: 0, handleKind: "port" }] }],
        edges: [],
      };
      const fixture3d: Puzzle3dFixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        domain: "architecture",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        objects: [{ id: "p1", meshUrl: "m.glb", origin: [0, 0, 0], vortices: [{ id: "p1:h", position: [0, 0, 0] }] }],
        attractions: [],
      };
      const t = compose5d(fixture2d, fixture3d);
      expect(t.parts.some((p) => p.id === "p1" && p.puzzle2d && p.puzzle3d)).toBe(true);
    });
  });
  describe("puzzle 3d projection", () => {
    it("projects parts with puzzle3d aspects to a 3d fixture", () => {
      const model = compose5d(
        {
          schema: "puzzle.2d.fixture/v1",
          camera: { x: 0, y: 0, zoom: 1 },
          nodes: [{ id: "p1", shape: "circle", x: 0, y: 0, radius: 10, handles: [] }],
          edges: [],
        },
        {
          schema: "puzzle.3d.fixture/v1",
          domain: "architecture",
          camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
          objects: [{ id: "p1", meshUrl: "m.glb", origin: [1, 2, 3], vortices: [] }],
          attractions: [],
        },
      );
      const fixture3d = project3d(model);
      expect(fixture3d.objects).toHaveLength(1);
      expect(fixture3d.objects[0]?.origin).toEqual([1, 2, 3]);
    });
  });

  describe("project2d", () => {
    it("round-trips part centers", () => {
      const model = compose5d(
        {
          schema: "puzzle.2d.fixture/v1",
          camera: { x: 0, y: 0, zoom: 1 },
          nodes: [{ id: "n", shape: "circle", x: 5, y: 6, radius: 4, handles: [] }],
          edges: [],
        },
        {
          schema: "puzzle.3d.fixture/v1",
          domain: "architecture",
          camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
          objects: [],
          attractions: [],
        },
      );
      const fixture2d = project2d(model);
      expect(fixture2d.nodes[0]?.x).toBe(5);
    });
  });
  describe("kindCompatibilityFromMetas", () => {
    it("prefers 2d meta rows when present", () => {
      const rows = kindCompatibilityFromMetas({
        meta2d: { kindCompatibility: [{ source: "a", target: "b" }] },
        meta3d: { kindCompatibility: [{ source: "x", target: "y" }] },
      });
      expect(rows.some((r) => r.source === "a")).toBe(true);
    });
    it("falls back to 3d meta when 2d has no rows", () => {
      const rows = kindCompatibilityFromMetas({
        meta2d: {},
        meta3d: { kindCompatibility: [{ source: "x", target: "y" }] },
      });
      expect(rows.some((r) => r.source === "x")).toBe(true);
    });
  });
  describe("sharedKindsFromMetas", () => {
    it("includes lod defaults", () => {
      const s = sharedKindsFromMetas({ meta2d: undefined, meta3d: undefined });
      expect(s.gridSnapEnabled).toBe(true);
    });
  });
  describe("flatHandleCompoundId", () => {
    it("round-trips handle ids", () => {
      const id = flatHandleCompoundId("piece-a", "conn-b");
      expect(flatParseHandleCompoundId(id)).toEqual({ left: "piece-a", right: "conn-b" });
    });
  });
  describe("flatGripAngle", () => {
    it("maps rectangle sides to axis angles", () => {
      expect(flatGripAngle("top", "rectangle")).toBe(0);
      expect(flatGripAngle("right", "rectangle")).toBeCloseTo(Math.PI / 2);
    });
  });
  describe("nodeCenterFromTopLeft", () => {
    it("offsets by half frame", () => {
      expect(nodeCenterFromTopLeft({ x: 10, y: 20 }, { width: 40, height: 60 })).toEqual({ x: 30, y: 50 });
    });
  });
  describe("flatApplyFixtureCentersToTopLeft", () => {
    it("converts centers to top-left using frame size", () => {
      const fixture: BoardFixtureV1 = {
        schema: "puzzle.2d.fixture/v1",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [{ id: "n1", shape: "rectangle", width: 40, height: 20, x: 50, y: 30, handles: [] }],
        edges: [],
      };
      const next = flatApplyFixtureCentersToTopLeft([{ id: "n1", position: { x: 0, y: 0 } }], fixture, () => ({ width: 40, height: 20 }));
      expect(next[0]?.position).toEqual({ x: 30, y: 20 });
    });
  });
  describe("puzzle2dForceGraphOptions", () => {
    it("maps charge strength to repulsion", () => {
      const o = puzzle2dForceGraphOptions({ centerStrength: 0.1, linkDistance: 120, chargeStrength: -400 });
      expect(o.repulsionStrength).toBe(400);
      expect(o.idealEdgeLength).toBe(120);
    });
  });
}
