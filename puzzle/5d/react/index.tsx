// #region 🧲Header
/** @emoji 🔗 `@puzzle/5d/react` — paired flat + volume topology surfaces and play harness (monolith). */
// #endregion 🧲Header

// #region 🔌Adapters
import { reactHostPort, type ContextMenuItem } from "@ui/react";
import type { ReactElement } from "react";

/** @emoji 🔗 Unified topology model with flat WASM + volume R3F projections and a shared {@link TopologyStore}. */

import {
  boardFixtureMetaKindCatalogBundle,
  parseBoardFixtureV1,
  DEFAULT_BOARD_GRID_FACTOR,
  DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
  type BoardCanvasProps,
  type BoardFixtureV1,
  type BoardForceGraphLayoutOptions,
  type BoardKindCatalogBundle,
  type BoardKindCompatEntry,
  type CameraState as BoardCameraState,
  type BoardLinkSessionSnapshot,
} from "@puzzle/2d/react";
import { BoardCanvas, Edge, Handle, Node, Wire } from "@puzzle/2d/react";
import {
  Canvas3D as VolumeView,
  Puzzle3dAttractions as VolumeBonds,
  Puzzle3dObjectStateProvider as VolumePartStateProvider,
  Puzzle3dObjects as VolumeParts,
  parseFixtureV1,
  blockedVortexFullIdsFromAttractions,
  usePuzzle3dObjectConnect as useVolumePartConnect,
  usePuzzle3dObjectRelocate as useVolumePartRelocate,
  type CanvasProps as VolumeViewProps,
  type FixtureV1 as VolumeFixtureV1,
  type KindCatalogBundle as VolumeKindCatalogBundle,
  type KindCompatEntry as VolumeKindCompatEntry,
  type RelocateMode as VolumeRelocateMode,
  type AttractionSessionSnapshot,
  type SelectionSnapshot as VolumeSelectionSnapshot,
  type DomainKind,
} from "../../3d/react/index.tsx";
// #endregion 🔌Adapters

//#region 🔖TopologyPairedPolicy
/** @emoji 🔗 How a bond is committed: direct handle pick, indirect ring finish, or proximity snap. */
export type TopologyConnectGestureKind = "direct" | "indirect" | "proximity";

/** @emoji ↔️ True only for {@link TopologyConnectGestureKind.indirect} — the gesture mirrored in {@link TopologyConnectSession}. */
export function topologyConnectGestureCrossSurface(kind: TopologyConnectGestureKind): boolean {
  return kind === "indirect";
}

/** @emoji 📶 Flat (@puzzle/2d) uses six discrete WASM draw LOD tiers from zoom thresholds. */
export const TOPOLOGY_FLAT_LOD_TIER_COUNT = 6 as const;

/** @emoji 📶 Volume (@puzzle/3d) uses continuous / camera-driven LOD (`automaticLod`, depth-variable, manual slider). */
export type TopologyVolumeLodPolicy = "continuous";

/** @emoji 🧲 Flat proximity: overlapping compatible handles while **dragging** a node (pointer-up snap). */
export const TOPOLOGY_FLAT_PROXIMITY_GESTURE: TopologyConnectGestureKind = "proximity";

/** @emoji 🧲 Volume proximity: compatible anchor within radius while **relocating** a part (gumball release). */
export const TOPOLOGY_VOLUME_PROXIMITY_GESTURE: TopologyConnectGestureKind = "proximity";

/** @emoji 🎯 Indirect link/attraction: start on one surface, finish on a compatible ring on either surface. */
export const TOPOLOGY_INDIRECT_CONNECT_GESTURE: TopologyConnectGestureKind = "indirect";
//#endregion 🔖TopologyPairedPolicy

//#region 🔖TopologyModel
export type TopologyPresentationMode = "2d" | "3d";

export interface TopologyFlatAnchorAspect {
  readonly angle: number;
  readonly anchorKind: string;
  readonly color?: string;
  readonly iconKind?: string;
  readonly radius?: number;
}

export interface TopologyVolumeAnchorAspect {
  readonly position: readonly [number, number, number];
  readonly direction?: readonly [number, number, number];
  readonly radius?: number;
  readonly label?: string;
  readonly handleMeshUrl?: string;
}

export interface TopologyAnchorV1 {
  readonly id: string;
  readonly anchorKind: string;
  readonly flat?: TopologyFlatAnchorAspect;
  readonly volume?: TopologyVolumeAnchorAspect;
}

export interface TopologyFlatPartAspect {
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

export interface TopologyVolumePartAspect {
  readonly origin: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: number | readonly [number, number, number];
  readonly meshUrl: string;
  readonly label?: string;
  readonly wormhole?: boolean;
}

export interface TopologyPartV1 {
  readonly id: string;
  readonly partKind?: string;
  readonly flat?: TopologyFlatPartAspect;
  readonly volume?: TopologyVolumePartAspect;
  readonly anchors: readonly TopologyAnchorV1[];
}

export interface TopologyBondV1 {
  readonly id: string;
  readonly source: string;
  readonly target: string;
  readonly bondKind?: string;
}

/** @emoji 🔗 In-progress **indirect** connect only (never proximity); synced across flat {@link BoardLinkSessionSnapshot} and volume {@link AttractionSessionSnapshot}. */
export interface TopologyConnectSession {
  readonly origin: TopologyPresentationMode;
  readonly sourceAnchor: string;
  readonly endX: number;
  readonly endY: number;
  readonly endVolume: readonly [number, number, number];
  readonly compatiblePartIds: readonly string[];
  readonly ringPartId: string | null;
  readonly ringAnchorIds: readonly string[];
}

export interface TopologySelectionSnapshot {
  readonly partIds: readonly string[];
  readonly anchorIds: readonly string[];
}

export interface TopologyV1 {
  readonly schema: "puzzle.5d.topology/v1";
  readonly label?: string;
  readonly domain: DomainKind;
  readonly meta?: Record<string, unknown>;
  readonly kindCatalogs?: BoardKindCatalogBundle;
  readonly kindCompatibility?: readonly BoardKindCompatEntry[];
  readonly flatCamera: BoardCameraState;
  readonly volumeCamera: VolumeFixtureV1["camera"];
  readonly parts: readonly TopologyPartV1[];
  readonly bonds: readonly TopologyBondV1[];
}

export const TOPOLOGY_ANCHOR_ID_SEPARATOR = ":";

/** @emoji 🔗 Builds a full anchor id `partId:anchorId`. */
export function topologyAnchorFullId(partId: string, anchorId: string): string {
  return `${partId}${TOPOLOGY_ANCHOR_ID_SEPARATOR}${anchorId}`;
}

/** @emoji 🔍 Splits a full anchor id into part and anchor local ids. */
export function topologyParseAnchorFullId(fullId: string): { partId: string; anchorId: string } | null {
  const i = fullId.indexOf(TOPOLOGY_ANCHOR_ID_SEPARATOR);
  if (i <= 0 || i >= fullId.length - 1) return null;
  return { partId: fullId.slice(0, i), anchorId: fullId.slice(i + 1) };
}

/** @emoji ✅ Validates unified topology JSON. */
export function parseTopologyV1(raw: unknown): TopologyV1 | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "puzzle.5d.topology/v1") return null;
  if (!Array.isArray(r.parts) || !Array.isArray(r.bonds)) return null;
  const domain = typeof r.domain === "string" ? (r.domain as DomainKind) : "architecture";
  const flatCam = r.flatCamera as BoardCameraState | undefined;
  const volumeCam = r.volumeCamera as VolumeFixtureV1["camera"] | undefined;
  if (!flatCam || !volumeCam) return null;
  return {
    schema: "puzzle.5d.topology/v1",
    domain,
    flatCamera: flatCam,
    volumeCamera: volumeCam,
    parts: r.parts as TopologyPartV1[],
    bonds: r.bonds as TopologyBondV1[],
    ...(typeof r.label === "string" ? { label: r.label } : {}),
    ...(r.meta && typeof r.meta === "object" ? { meta: r.meta as Record<string, unknown> } : {}),
    ...(r.kindCatalogs && typeof r.kindCatalogs === "object" ? { kindCatalogs: r.kindCatalogs as BoardKindCatalogBundle } : {}),
    ...(Array.isArray(r.kindCompatibility) ? { kindCompatibility: r.kindCompatibility as BoardKindCompatEntry[] } : {}),
  };
}

/** @emoji 🔀 Builds {@link TopologyV1} by merging flat and volume fixtures (same part ids unite). */
export function topologyCompose(flat: BoardFixtureV1, volume: VolumeFixtureV1): TopologyV1 {
  const partsMap = new Map<string, TopologyPartV1>();
  for (const node of flat.nodes) {
    const anchors: TopologyAnchorV1[] = node.handles.map((h) => {
      const parsed = topologyParseAnchorFullId(h.id);
      const localId = parsed?.anchorId ?? h.id;
      return {
        id: localId,
        anchorKind: h.handleKind,
        flat: {
          angle: h.angle,
          anchorKind: h.handleKind,
          ...(h.color !== undefined ? { color: h.color } : {}),
          ...(h.iconKind !== undefined ? { iconKind: h.iconKind } : {}),
          ...(h.radius !== undefined ? { radius: h.radius } : {}),
        },
      };
    });
    const flatAspect: TopologyFlatPartAspect =
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
      flat: flatAspect,
      anchors,
    });
  }
  for (const obj of volume.objects) {
    const volumeAspect: TopologyVolumePartAspect = {
      origin: obj.origin,
      meshUrl: obj.meshUrl,
      ...(obj.orientation !== undefined ? { orientation: obj.orientation } : {}),
      ...(obj.scale !== undefined ? { scale: obj.scale } : {}),
      ...(obj.label !== undefined ? { label: obj.label } : {}),
      ...(obj.wormhole === true ? { wormhole: true } : {}),
    };
    const volumeAnchors: TopologyAnchorV1[] = obj.vortices.map((v) => {
      const parsed = topologyParseAnchorFullId(v.id.includes(":") ? v.id : topologyAnchorFullId(obj.id, v.id));
      const localId = parsed?.anchorId ?? v.id;
      return {
        id: localId,
        anchorKind: v.vortexKind ?? "board.port",
        volume: {
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
        anchorById.set(a.id, prev ? { ...prev, volume: a.volume, anchorKind: a.anchorKind } : a);
      }
      partsMap.set(obj.id, {
        ...existing,
        ...(obj.objectKind !== undefined ? { partKind: obj.objectKind } : {}),
        volume: volumeAspect,
        anchors: [...anchorById.values()],
      });
    } else {
      partsMap.set(obj.id, {
        id: obj.id,
        ...(obj.objectKind !== undefined ? { partKind: obj.objectKind } : {}),
        volume: volumeAspect,
        anchors: volumeAnchors,
      });
    }
  }
  const bonds: TopologyBondV1[] = [];
  const bondIds = new Set<string>();
  for (const edge of flat.edges) {
    if (bondIds.has(edge.id)) continue;
    bondIds.add(edge.id);
    bonds.push({ id: edge.id, source: edge.source, target: edge.target });
  }
  for (const att of volume.attractions) {
    if (bondIds.has(att.id)) continue;
    bondIds.add(att.id);
    bonds.push({
      id: att.id,
      source: att.attracting,
      target: att.attracted,
      ...(att.attractionKind !== undefined ? { bondKind: att.attractionKind } : {}),
    });
  }
  const meta = {
    ...(flat.meta ?? {}),
    ...(volume.meta ?? {}),
  };
  const kindCatalogs = topologyKindCatalogsFromMetas({ flatMeta: flat.meta, volumeMeta: volume.meta });
  const kindCompatibility = topologyKindCompatibilityFromMetas({ flatMeta: flat.meta, volumeMeta: volume.meta });
  return {
    schema: "puzzle.5d.topology/v1",
    domain: volume.domain,
    flatCamera: { ...flat.camera },
    volumeCamera: { ...volume.camera },
    parts: [...partsMap.values()],
    bonds,
    ...(Object.keys(meta).length > 0 ? { meta } : {}),
    ...(kindCatalogs ? { kindCatalogs } : {}),
    ...(kindCompatibility.length > 0 ? { kindCompatibility } : {}),
  };
}

/** @emoji 📐 Projects {@link TopologyV1} to a board fixture for flat rendering. */
export function projectFlat(model: TopologyV1): BoardFixtureV1 {
  const nodes = model.parts
    .filter((p) => p.flat)
    .map((p) => {
      const flat = p.flat!;
      const handles = p.anchors
        .filter((a) => a.flat)
        .map((a) => ({
          id: topologyAnchorFullId(p.id, a.id),
          angle: a.flat!.angle,
          handleKind: a.flat!.anchorKind,
          ...(a.flat!.color !== undefined ? { color: a.flat!.color } : {}),
          ...(a.flat!.iconKind !== undefined ? { iconKind: a.flat!.iconKind } : {}),
          ...(a.flat!.radius !== undefined ? { radius: a.flat!.radius } : {}),
        }));
      if (flat.shape === "rectangle") {
        return {
          id: p.id,
          shape: "rectangle" as const,
          x: flat.x,
          y: flat.y,
          width: flat.width ?? 40,
          height: flat.height ?? 40,
          handles,
          ...(p.partKind !== undefined ? { nodeKind: p.partKind } : {}),
          ...(flat.text !== undefined ? { text: flat.text } : {}),
          ...(flat.textAlignment !== undefined ? { textAlignment: flat.textAlignment } : {}),
          ...(flat.textAutofit === true ? { textAutofit: true } : {}),
          ...(flat.textFontFamily !== undefined ? { textFontFamily: flat.textFontFamily } : {}),
          ...(flat.textFontSize !== undefined ? { textFontSize: flat.textFontSize } : {}),
          ...(flat.iconKind !== undefined ? { iconKind: flat.iconKind } : {}),
        };
      }
      return {
        id: p.id,
        shape: "circle" as const,
        x: flat.x,
        y: flat.y,
        radius: flat.radius ?? 20,
        handles,
        ...(p.partKind !== undefined ? { nodeKind: p.partKind } : {}),
        ...(flat.text !== undefined ? { text: flat.text } : {}),
        ...(flat.textAlignment !== undefined ? { textAlignment: flat.textAlignment } : {}),
        ...(flat.textAutofit === true ? { textAutofit: true } : {}),
        ...(flat.textFontFamily !== undefined ? { textFontFamily: flat.textFontFamily } : {}),
        ...(flat.textFontSize !== undefined ? { textFontSize: flat.textFontSize } : {}),
        ...(flat.iconKind !== undefined ? { iconKind: flat.iconKind } : {}),
      };
    });
  return {
    schema: "puzzle.2d.fixture/v1",
    camera: { ...model.flatCamera },
    nodes,
    edges: model.bonds.map((b) => ({ id: b.id, source: b.source, target: b.target })),
    ...(model.meta ? { meta: model.meta } : {}),
  };
}

/** @emoji 📐 Projects {@link TopologyV1} to a @puzzle/3d fixture for volume rendering. */
export function projectVolume(model: TopologyV1): VolumeFixtureV1 {
  const objects = model.parts
    .filter((p) => p.volume)
    .map((p) => {
      const s = p.volume!;
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
          .filter((a) => a.volume)
          .map((a) => ({
            id: topologyAnchorFullId(p.id, a.id),
            position: a.volume!.position,
            ...(a.anchorKind ? { vortexKind: a.anchorKind } : {}),
            ...(a.volume!.direction !== undefined ? { direction: a.volume!.direction } : {}),
            ...(a.volume!.radius !== undefined ? { radius: a.volume!.radius } : {}),
            ...(a.volume!.label !== undefined ? { label: a.volume!.label } : {}),
            ...(a.volume!.handleMeshUrl !== undefined ? { handleMeshUrl: a.volume!.handleMeshUrl } : {}),
          })),
      };
    });
  return {
    schema: "puzzle.3d.fixture/v1",
    domain: model.domain,
    camera: { ...model.volumeCamera },
    objects,
    attractions: model.bonds.map((b) => ({
      id: b.id,
      attracting: b.source as `${string}:${string}`,
      attracted: b.target as `${string}:${string}`,
      ...(b.bondKind !== undefined ? { attractionKind: b.bondKind } : {}),
    })),
    ...(model.meta ? { meta: model.meta } : {}),
  };
}
//#endregion 🔖TopologyModel

//#region 🔖TopologyStore
export interface TopologyStoreSnapshot {
  readonly model: TopologyV1;
  readonly selection: TopologySelectionSnapshot;
  readonly connectSession: TopologyConnectSession | null;
  readonly cameras: Readonly<Record<string, { flat: BoardCameraState; volume: VolumeFixtureV1["camera"] }>>;
}

class TopologyStore {
  private listeners = new Set<() => void>();
  private snapshot: TopologyStoreSnapshot;

  constructor(model: TopologyV1) {
    this.snapshot = {
      model,
      selection: { partIds: [], anchorIds: [] },
      connectSession: null,
      cameras: {},
    };
  }

  getSnapshot = (): TopologyStoreSnapshot => this.snapshot;

  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  private emit(): void {
    for (const l of this.listeners) l();
  }

  private setSnapshot(next: TopologyStoreSnapshot): void {
    this.snapshot = next;
    this.emit();
  }

  getModel(): TopologyV1 {
    return this.snapshot.model;
  }

  getFlatCamera(instanceId: string): BoardCameraState {
    return this.snapshot.cameras[instanceId]?.flat ?? this.snapshot.model.flatCamera;
  }

  getVolumeCamera(instanceId: string): VolumeFixtureV1["camera"] {
    return this.snapshot.cameras[instanceId]?.volume ?? this.snapshot.model.volumeCamera;
  }

  setFlatCamera(instanceId: string, camera: BoardCameraState): void {
    const prev = this.snapshot.cameras[instanceId];
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, flatCamera: camera },
      cameras: {
        ...this.snapshot.cameras,
        [instanceId]: { flat: camera, volume: prev?.volume ?? this.snapshot.model.volumeCamera },
      },
    });
  }

  setVolumeCamera(instanceId: string, camera: VolumeFixtureV1["camera"]): void {
    const prev = this.snapshot.cameras[instanceId];
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, volumeCamera: camera },
      cameras: {
        ...this.snapshot.cameras,
        [instanceId]: { flat: prev?.flat ?? this.snapshot.model.flatCamera, volume: camera },
      },
    });
  }

  setSelection(selection: TopologySelectionSnapshot): void {
    this.setSnapshot({ ...this.snapshot, selection });
  }

  /** @emoji 🔗 Sets cross-surface indirect preview state; callers must not use this for proximity snaps. */
  setConnectSession(session: TopologyConnectSession | null): void {
    this.setSnapshot({ ...this.snapshot, connectSession: session });
  }

  applyFlatPartMove(partId: string, x: number, y: number): void {
    const parts = this.snapshot.model.parts.map((p) => {
      if (p.id !== partId || !p.flat) return p;
      return { ...p, flat: { ...p.flat, x, y } };
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }

  applyVolumeRelocate(partId: string, origin: readonly [number, number, number], orientation: readonly [number, number, number, number]): void {
    const parts = this.snapshot.model.parts.map((p) => {
      if (p.id !== partId || !p.volume) return p;
      return { ...p, volume: { ...p.volume, origin, orientation } };
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }

  applyBond(source: string, target: string, bondKind?: string): void {
    const id = crypto.randomUUID();
    const bonds: TopologyBondV1[] = [...this.snapshot.model.bonds, { id, source, target, ...(bondKind ? { bondKind } : {}) }];
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, bonds },
      connectSession: null,
    });
  }

  replaceModel(model: TopologyV1): void {
    this.setSnapshot({
      ...this.snapshot,
      model,
      connectSession: null,
    });
  }
}

export function createTopologyStore(model: TopologyV1): TopologyStore {
  return new TopologyStore(model);
}

const TopologyStoreContext = reactHostPort.createContext<TopologyStore | null>(null);

export function TopologyStoreProvider(props: { readonly store: TopologyStore; readonly children: React.ReactNode }): ReactElement {
  return <TopologyStoreContext.Provider value={props.store}>{props.children}</TopologyStoreContext.Provider>;
}

export function useTopologyStore(): TopologyStore {
  const store = reactHostPort.useContext(TopologyStoreContext);
  if (!store) throw new Error("useTopologyStore requires TopologyStoreProvider");
  return store;
}

export function useTopologySnapshot(): TopologyStoreSnapshot {
  const store = useTopologyStore();
  return reactHostPort.useSyncExternalStore(store.subscribe, store.getSnapshot, store.getSnapshot);
}
//#endregion 🔖TopologyStore

//#region 🔖FiveD
export const FIVE_D_ROOT_CLASS = "flex h-full min-h-0 flex-1 flex-col";

/** @emoji 📶 Flat-only LOD/grid defaults ({@link TOPOLOGY_FLAT_LOD_TIER_COUNT} discrete tiers); do not pass to volume {@link VolumeView}. */
export const FIVE_D_FLAT_LOD_DEFAULTS = {
  lodZoomThresholds: DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
  gridFactor: DEFAULT_BOARD_GRID_FACTOR,
  gridSnapEnabled: true,
} as const;

/** @emoji 🎛 Volume chrome: continuous LOD comes from host `volume` props; proximity applies on relocate only. */
export const FIVE_D_VOLUME_CHROME_DEFAULTS: Pick<VolumeViewProps, "showLodGrid" | "proximityRadius" | "proximityRelocateEnabled" | "gridSnapEnabled"> = {
  showLodGrid: true,
  proximityRadius: 24,
  proximityRelocateEnabled: true,
  gridSnapEnabled: true,
};

/** @emoji 🖼️ Single topology editor (`2d` = @puzzle/2d board, `3d` = @puzzle/3d); pair via {@link TopologyStoreProvider}. */
export interface FiveDProps {
  readonly mode: TopologyPresentationMode;
  readonly instanceId: string;
  readonly className?: string;
  readonly lockedPartIds?: ReadonlySet<string>;
  readonly relocateMode?: VolumeRelocateMode;
  /** Flat surface overrides; LOD uses discrete tiers unless `automaticLod` is set on the canvas. */
  readonly flat?: Omit<BoardCanvasProps, "children">;
  /** Volume surface overrides; LOD is continuous/camera-driven — not the flat six-tier scale. */
  readonly volume?: Omit<VolumeViewProps, "children">;
}

function fiveDLinkSessionFromStore(session: TopologyConnectSession | null): BoardLinkSessionSnapshot | null {
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

function fiveDAttractionSessionFromStore(session: TopologyConnectSession | null): AttractionSessionSnapshot | null {
  if (!session) return null;
  return {
    attracting: session.sourceAnchor,
    end: session.endVolume,
    compatibleObjectIds: session.compatiblePartIds,
    ringObjectId: session.ringPartId,
    ringVortexFullIds: session.ringAnchorIds,
  };
}

function topologyFlatMarkersFromFixture(props: { readonly fixture: BoardFixtureV1; readonly lockedIds: ReadonlySet<string>; readonly selectedIds: ReadonlySet<string> }): ReactElement {
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

const FiveDFlat = reactHostPort.memo(function FiveDFlat(props: FiveDProps) {
  const store = useTopologyStore();
  const snap = useTopologySnapshot();
  const flatFixture = reactHostPort.useMemo(() => projectFlat(snap.model), [snap.model]);
  const locked = props.lockedPartIds ?? new Set<string>();
  const selectedIds = reactHostPort.useMemo(() => new Set([...snap.selection.partIds, ...snap.selection.anchorIds]), [snap.selection]);
  const markers = reactHostPort.useMemo(() => topologyFlatMarkersFromFixture({ fixture: flatFixture, lockedIds: locked, selectedIds }), [flatFixture, locked, selectedIds]);
  const camera = store.getFlatCamera(props.instanceId);
  const flatExtra = props.flat ?? {};
  const { onSelect: onSelectHost, onConnect: onConnectHost, onIndirectConnect: onIndirectConnectHost, onProximityConnect: onProximityConnectHost, onDrag: onDragHost, ...flatRest } = flatExtra;
  const linkSession = fiveDLinkSessionFromStore(snap.connectSession);
  return (
    <div className={FIVE_D_ROOT_CLASS} data-five-d-indirect-active={snap.connectSession ? "true" : "false"} data-five-d-mode="2d" data-five-d-instance={props.instanceId}>
      <BoardCanvas
        camera={flatRest.camera ?? camera}
        className={["min-h-0 flex-1", props.className, flatRest.className].filter(Boolean).join(" ") || undefined}
        {...FIVE_D_FLAT_LOD_DEFAULTS}
        kindCatalogs={snap.model.kindCatalogs}
        kindCompatibility={snap.model.kindCompatibility}
        linkSession={linkSession}
        onCamera={(c) => store.setFlatCamera(props.instanceId, c)}
        onConnect={(p) => {
          store.applyBond(p.source, p.target);
          onConnectHost?.(p);
        }}
        onDrag={(p) => {
          store.applyFlatPartMove(p.id, p.x, p.y);
          onDragHost?.(p);
        }}
        onIndirectConnect={(p) => {
          store.applyBond(p.source, p.target);
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
            endVolume: prev?.endVolume ?? [0, 0, 0],
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
            endVolume: prev?.endVolume ?? [0, 0, 0],
            compatiblePartIds: prev?.compatiblePartIds ?? [],
            ringPartId: p.nodeId,
            ringAnchorIds: [...p.handleIds],
          });
        }}
        onProximityConnect={(p) => {
          store.applyBond(p.source, p.target);
          onProximityConnectHost?.(p);
        }}
        onSelect={(s) => {
          store.setSelection({ partIds: s.ids, anchorIds: [] });
          onSelectHost?.(s);
        }}
        {...flatRest}
      >
        {markers}
      </BoardCanvas>
    </div>
  );
});

const FiveDVolumeInner = reactHostPort.memo(function FiveDVolumeInner(props: FiveDProps) {
  const store = useTopologyStore();
  const snap = useTopologySnapshot();
  const volumeFixture = reactHostPort.useMemo(() => projectVolume(snap.model), [snap.model]);
  const volumeExtra = props.volume ?? {};
  const {
    onSelect: onSelectHost,
    onConnect: onConnectHost,
    onIndirectConnect: onIndirectConnectHost,
    onProximityConnect: onProximityConnectHost,
    onRelocate: onRelocateHost,
    onAttractionCompatibleObjects: onAttractionCompatibleObjectsHost,
    onAttractionTargetRing: onAttractionTargetRingHost,
    ...volumeRest
  } = volumeExtra;
  const camera = store.getVolumeCamera(props.instanceId);
  const selectedObjectId = snap.selection.partIds[0] ?? null;
  const attractionSession = fiveDAttractionSessionFromStore(snap.connectSession);
  const onRelocate = useVolumePartRelocate();
  const onConnect = useVolumePartConnect();
  return (
    <VolumeView
      camera={volumeRest.camera ?? camera}
      className={["min-h-0 flex-1", props.className, volumeRest.className].filter(Boolean).join(" ") || undefined}
      {...FIVE_D_VOLUME_CHROME_DEFAULTS}
      attractionSession={attractionSession}
      blockedVortexFullIds={blockedVortexFullIdsFromAttractions(volumeFixture.attractions)}
      gridFactor={FIVE_D_FLAT_LOD_DEFAULTS.gridFactor}
      gridSnapEnabled={FIVE_D_FLAT_LOD_DEFAULTS.gridSnapEnabled}
      kindCatalogs={snap.model.kindCatalogs as VolumeKindCatalogBundle | undefined}
      kindCompatibility={snap.model.kindCompatibility as VolumeKindCompatEntry[] | undefined}
      relocateMode={props.relocateMode ?? "translate"}
      onCamera={(c) => store.setVolumeCamera(props.instanceId, c)}
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
          endVolume: prev?.endVolume ?? [0, 0, 0],
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
          endVolume: prev?.endVolume ?? [0, 0, 0],
          compatiblePartIds: prev?.compatiblePartIds ?? [],
          ringPartId: p.objectId,
          ringAnchorIds: [...p.vortexFullIds],
        });
        onAttractionTargetRingHost?.(p);
      }}
      onIndirectConnect={(p) => {
        store.applyBond(p.attracting, p.attracted);
        onIndirectConnectHost?.(p);
        onConnect?.(p);
      }}
      onProximityConnect={(p) => {
        store.applyBond(p.attracting, p.attracted);
        onProximityConnectHost?.(p);
      }}
      onSelect={(s: VolumeSelectionSnapshot) => {
        store.setSelection({ partIds: [...s.objectIds], anchorIds: [...s.vortexIds] });
        onSelectHost?.(s);
      }}
      {...volumeRest}
    >
      <VolumeParts selectedObjectId={selectedObjectId} relocate={props.relocateMode ?? "translate"} />
      <VolumeBonds />
    </VolumeView>
  );
});

const FiveDVolume = reactHostPort.memo(function FiveDVolume(props: FiveDProps) {
  const snap = useTopologySnapshot();
  const volumeFixture = reactHostPort.useMemo(() => projectVolume(snap.model), [snap.model]);
  return (
    <div className={FIVE_D_ROOT_CLASS} data-five-d-indirect-active={snap.connectSession ? "true" : "false"} data-five-d-mode="3d" data-five-d-instance={props.instanceId}>
      <reactHostPort.Suspense fallback={<div className="flex min-h-0 flex-1 items-center justify-center p-4 text-sm text-muted-foreground">Loading meshes…</div>}>
        <VolumePartStateProvider fixture={volumeFixture} onConnect={props.volume?.onConnect} onRelocate={props.volume?.onRelocate}>
          <FiveDVolumeInner {...props} />
        </VolumePartStateProvider>
      </reactHostPort.Suspense>
    </div>
  );
});

/** @emoji 🖼️ Single topology editor surface (`2d` board WASM or `3d` R3F); share state via {@link TopologyStoreProvider}. */
export const FiveD = reactHostPort.memo(function FiveD(props: FiveDProps) {
  if (props.mode === "2d") return <FiveDFlat {...props} />;
  return <FiveDVolume {...props} />;
});
//#endregion 🔖FiveD

//#region 🔖KindMeta
function isTopologyMetaRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

/** @emoji 📚 Reads `kindCompatibility` rows from fixture meta. */
export function topologyKindCompatibilityRowsFromMeta(meta: Record<string, unknown> | undefined): BoardKindCompatEntry[] {
  if (!isTopologyMetaRecord(meta)) return [];
  const arr = meta.kindCompatibility;
  if (!Array.isArray(arr)) return [];
  const out: BoardKindCompatEntry[] = [];
  for (const entry of arr) {
    if (!isTopologyMetaRecord(entry)) continue;
    const source = typeof entry.source === "string" ? entry.source.trim() : "";
    const target = typeof entry.target === "string" ? entry.target.trim() : "";
    if (!source || !target) continue;
    const specificity =
      entry.specificity === "general" || entry.specificity === "node" || entry.specificity === "edge" || entry.specificity === "handle" || entry.specificity === "wire" || entry.specificity === "object" || entry.specificity === "attraction"
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

export function topologyKindCatalogBundleFromVolumeFixtureMeta(meta: Record<string, unknown> | undefined): VolumeKindCatalogBundle | undefined {
  if (!isTopologyMetaRecord(meta)) return undefined;
  const kc = meta.kindCatalogs;
  if (!kc || typeof kc !== "object" || Array.isArray(kc)) return undefined;
  return kc as VolumeKindCatalogBundle;
}

export function topologyKindCatalogsFromMetas(inp: { readonly flatMeta: Record<string, unknown> | undefined; readonly volumeMeta: Record<string, unknown> | undefined }): BoardKindCatalogBundle | undefined {
  const fromFlat = boardFixtureMetaKindCatalogBundle(inp.flatMeta);
  if (fromFlat) return fromFlat;
  return topologyKindCatalogBundleFromVolumeFixtureMeta(inp.volumeMeta) as BoardKindCatalogBundle | undefined;
}

export function topologyKindCompatibilityFromMetas(inp: { readonly flatMeta: Record<string, unknown> | undefined; readonly volumeMeta: Record<string, unknown> | undefined }): readonly BoardKindCompatEntry[] {
  const fromFlat = topologyKindCompatibilityRowsFromMeta(inp.flatMeta);
  if (fromFlat.length > 0) return fromFlat;
  return topologyKindCompatibilityRowsFromMeta(inp.volumeMeta);
}

export function topologySharedKindsFromMetas(inp: { readonly flatMeta: Record<string, unknown> | undefined; readonly volumeMeta: Record<string, unknown> | undefined }): Pick<
  typeof FIVE_D_FLAT_LOD_DEFAULTS,
  "lodZoomThresholds" | "gridFactor" | "gridSnapEnabled"
> & {
  kindCatalogs?: BoardKindCatalogBundle;
  kindCompatibility?: readonly BoardKindCompatEntry[];
} {
  return {
    ...FIVE_D_FLAT_LOD_DEFAULTS,
    kindCatalogs: topologyKindCatalogsFromMetas(inp),
    kindCompatibility: topologyKindCompatibilityFromMetas(inp),
  };
}
//#endregion 🔖KindMeta

//#region 🔖BoardLayout
/** @emoji 🔗 Default separator for flat handle ids (`piece::connector`). */
export const FLAT_HANDLE_COMPOUND_SEPARATOR = "::";

/** @emoji 🔗 Builds a compound flat handle id from two parts. */
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

export type KitFlatHandleSide = "top" | "right" | "bottom" | "left";

/** @emoji ┬¡ãÆ├┤├ë Kit diagram snap side to board handle angle (rectangle vs circle rim). */
export function kitFlatHandleAngle(side: KitFlatHandleSide, shape: "circle" | "rectangle"): number {
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
export function flatPartCenterFromTopLeft(position: { readonly x: number; readonly y: number }, frame: { readonly width: number; readonly height: number }): { x: number; y: number } {
  return { x: position.x + frame.width / 2, y: position.y + frame.height / 2 };
}

/** @emoji ┬¡ãÆ├▓┬®┬┤┬®├à Diagram force-slider weights shared by sketchpad kit/design hosts. */
export interface TopologyDiagramForceWeights {
  readonly centerStrength: number;
  readonly linkDistance: number;
  readonly chargeStrength: number;
}

/** @emoji ┬¡ãÆ├▓┬®┬┤┬®├à Maps diagram force sliders to {@link layoutBoardFixtureForceGraph} options. */
export function flatDiagramForceGraphOptions(weights: TopologyDiagramForceWeights): BoardForceGraphLayoutOptions {
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
export function flatCameraFromPartCenters(centers: readonly { x: number; y: number }[]): BoardCameraState {
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

/** @emoji 🎛 Default volume chrome for topology surfaces (alias of {@link FIVE_D_VOLUME_CHROME_DEFAULTS}). */
export function volumeChromeDefaults(): typeof FIVE_D_VOLUME_CHROME_DEFAULTS {
  return FIVE_D_VOLUME_CHROME_DEFAULTS;
}

//#region 🔖BoardMarkers
export interface TopologyFlatWireRecord {
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
  readonly wires: readonly TopologyFlatWireRecord[];
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

export { parseBoardFixtureV1, parseFixtureV1, blockedVortexFullIdsFromAttractions };
export { DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS, DEFAULT_BOARD_GRID_FACTOR };

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("topologyConnectGestureCrossSurface", () => {
    it("only indirect syncs across flat and volume", () => {
      expect(topologyConnectGestureCrossSurface("indirect")).toBe(true);
      expect(topologyConnectGestureCrossSurface("direct")).toBe(false);
      expect(topologyConnectGestureCrossSurface("proximity")).toBe(false);
    });
  });

  describe("parseTopologyV1", () => {
    it("accepts unified topology", () => {
      const t = parseTopologyV1({
        schema: "puzzle.5d.topology/v1",
        domain: "architecture",
        flatCamera: { x: 0, y: 0, zoom: 1 },
        volumeCamera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        parts: [],
        bonds: [],
        label: "x",
      });
      expect(t?.schema).toBe("puzzle.5d.topology/v1");
      expect(t?.label).toBe("x");
    });
  });
  describe("topologyCompose", () => {
    it("merges flat nodes and volume objects by id", () => {
      const flatFixture: BoardFixtureV1 = {
        schema: "puzzle.2d.fixture/v1",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [{ id: "p1", shape: "circle", x: 1, y: 2, radius: 10, handles: [{ id: "p1:h", angle: 0, handleKind: "board.port" }] }],
        edges: [],
      };
      const volumeFixture: VolumeFixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        domain: "architecture",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        objects: [{ id: "p1", meshUrl: "m.glb", origin: [0, 0, 0], vortices: [{ id: "p1:h", position: [0, 0, 0] }] }],
        attractions: [],
      };
      const t = topologyCompose(flatFixture, volumeFixture);
      expect(t.parts.some((p) => p.id === "p1" && p.flat && p.volume)).toBe(true);
    });
  });
  describe("topology volume projection", () => {
    it("projects parts with volume aspects to a 3d fixture", () => {
      const model = topologyCompose(
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
      const volume = projectVolume(model);
      expect(volume.objects).toHaveLength(1);
      expect(volume.objects[0]?.origin).toEqual([1, 2, 3]);
    });
  });

  describe("projectFlat", () => {
    it("round-trips part centers", () => {
      const model = topologyCompose(
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
      const flat = projectFlat(model);
      expect(flat.nodes[0]?.x).toBe(5);
    });
  });
  describe("topologyKindCompatibilityFromMetas", () => {
    it("prefers flat meta rows when present", () => {
      const rows = topologyKindCompatibilityFromMetas({
        flatMeta: { kindCompatibility: [{ source: "a", target: "b" }] },
        volumeMeta: { kindCompatibility: [{ source: "x", target: "y" }] },
      });
      expect(rows.some((r) => r.source === "a")).toBe(true);
    });
    it("falls back to volume meta when flat has no rows", () => {
      const rows = topologyKindCompatibilityFromMetas({
        flatMeta: {},
        volumeMeta: { kindCompatibility: [{ source: "x", target: "y" }] },
      });
      expect(rows.some((r) => r.source === "x")).toBe(true);
    });
  });
  describe("topologySharedKindsFromMetas", () => {
    it("includes lod defaults", () => {
      const s = topologySharedKindsFromMetas({ flatMeta: undefined, volumeMeta: undefined });
      expect(s.gridSnapEnabled).toBe(true);
    });
  });
  describe("flatHandleCompoundId", () => {
    it("round-trips handle ids", () => {
      const id = flatHandleCompoundId("piece-a", "conn-b");
      expect(flatParseHandleCompoundId(id)).toEqual({ left: "piece-a", right: "conn-b" });
    });
  });
  describe("kitFlatHandleAngle", () => {
    it("maps rectangle sides to axis angles", () => {
      expect(kitFlatHandleAngle("top", "rectangle")).toBe(0);
      expect(kitFlatHandleAngle("right", "rectangle")).toBeCloseTo(Math.PI / 2);
    });
  });
  describe("flatPartCenterFromTopLeft", () => {
    it("offsets by half frame", () => {
      expect(flatPartCenterFromTopLeft({ x: 10, y: 20 }, { width: 40, height: 60 })).toEqual({ x: 30, y: 50 });
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
  describe("flatDiagramForceGraphOptions", () => {
    it("maps charge strength to repulsion", () => {
      const o = flatDiagramForceGraphOptions({ centerStrength: 0.1, linkDistance: 120, chargeStrength: -400 });
      expect(o.repulsionStrength).toBe(400);
      expect(o.idealEdgeLength).toBe(120);
    });
  });
}
