// #region 🧲Header
/** @emoji 🔗 `@puzzle/5d/react` — paired 2d + 3d puzzle 5d surfaces and play harness (monolith). */
// #endregion 🧲Header

// #region 🔌Adapters
import { reactHostPort, type ContextMenuItem, type GumballConfig } from "@ui/react";
import { PUZZLE_3D_GUMBALL_CONFIG, puzzle3dObjectGumballConfig } from "../../3d/react/index.tsx";
import type { ReactElement } from "react";

/** @emoji 🔗 Unified puzzle 5d model with 2d WASM + 3d R3F projections and a shared {@link Store}. */

import {
  Puzzle2dCanvas,
  DEFAULT_PUZZLE_2D_GRID_FACTOR,
  getPuzzle2dLodScale,
  Edge, Handle, Node, Wire,
  BUILTIN_PORT_HANDLE_KIND,
  fixtureMetaKindCatalogBundle,
  parsePuzzle2dFixtureV1,
  type CameraState as Puzzle2dCameraState,
  type Puzzle2dCanvasProps,
  type Puzzle2dFixtureV1,
  type Puzzle2dForceGraphLayoutOptions,
  type Puzzle2dRedrawLayoutOptions,
  puzzle2dApplyLiveForceGraphLayoutTick,
  type Puzzle2dLiveForceGraphDragState,
  buildPuzzle2dSceneDescriptorFromFixture,
  type KindCatalogBundle as Puzzle2dKindCatalogBundle,
  type KindCompatEntry as Puzzle2dKindCompatEntry,
  type Puzzle2dLinkSessionSnapshot,
  type EdgeKind as Puzzle2dEdgeKind,
  type HandleKind as Puzzle2dHandleKind,
  type NodeKind as Puzzle2dNodeKind,
  type WireKind as Puzzle2dWireKind,
  type Puzzle2dActiveTool,
  type Puzzle2dBrushPlacePayload,
  type Puzzle2dStructureDeletePayload,
  DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX,
  DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
  puzzle2dFixtureHandlesFromNodeKind,
  buildPaletteNodeDragFixture,
  mergePaletteNodeFromDrop,
  abortPuzzle2dFixturePaletteDrag,
  puzzle2dFixturePaletteDropCommittedRef,
  type Puzzle2dFixtureDropDetail,
  type Puzzle2dFixtureNodeV1,
  type Puzzle2dKindHover,
  type Puzzle2dHoverPayload,
} from "@puzzle/2d/react";
import {
  ObjectStateProvider as Puzzle3dPartStateProvider,
  Objects as Puzzle3dParts,
  Attractions as Puzzle3dTies,
  Canvas3D as Puzzle3dCanvas,
  blockedVortexFullIdsFromAttractions,
  cancelPuzzle3dFixturePalettePointerDrag,
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
  type BrushPlacePayload,
  type Puzzle3dBrushKindWeights,
  type FixtureObjectV1 as Puzzle3dFixtureObjectV1,
  type HoverTarget,
  type Puzzle3dHoverPayload,
  type Puzzle3dKindHover,
  buildBrushFillSequence,
  brushPlacementUsesHostOrientation,
  computeBrushPlacementPose,
  resolveObjectKindMeshUrl,
  vortexWorldCadFromObject,
  PUZZLE_3D_FILL_COUNT_MAX,
} from "../../3d/react/index.tsx";
import type { Object3D } from "three";
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

/** @emoji 🔗 In-progress **indirect** connect only (never proximity); synced across 2d {@link Puzzle2dLinkSessionSnapshot} and 3d {@link AttractionSessionSnapshot}. */
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

/** @emoji 🎯 Maps unified store selection to puzzle 3d controlled canvas selection. */
export function fiveD3dSelectionFromStore(selection: SelectionSnapshot): Puzzle3dSelectionSnapshot {
  return {
    objectIds: [...selection.partIds],
    vortexIds: [...selection.anchorIds],
    attractionIds: [],
  };
}

//#region 🔖Hover
export type Puzzle5dKindHoverDomain = "part" | "anchor" | "tie";

/** @emoji 🖱️ Transitive catalog-kind hover shared across paired 2d and 3d surfaces. */
export interface Puzzle5dKindHover {
  readonly domain: Puzzle5dKindHoverDomain;
  readonly kindId: string;
}

export type Puzzle5dHoverInstance =
  | { readonly kind: "part"; readonly id: string }
  | { readonly kind: "anchor"; readonly fullId: string }
  | { readonly kind: "tie"; readonly id: string };

export interface HoverFocusSnapshot {
  readonly instance: Puzzle5dHoverInstance | null;
  readonly kindHover: Puzzle5dKindHover | null;
}

const EMPTY_HOVER_FOCUS: HoverFocusSnapshot = { instance: null, kindHover: null };

/** @emoji 🖱️ Compares two unified puzzle 5d kind hovers for equality. */
export function puzzle5dKindHoversEqual(a: Puzzle5dKindHover | null, b: Puzzle5dKindHover | null): boolean {
  if (a === b) {
    return true;
  }
  if (!a || !b) {
    return false;
  }
  return a.domain === b.domain && a.kindId === b.kindId;
}

/** @emoji 🖱️ Compares two unified puzzle 5d hover instances for equality. */
export function puzzle5dHoverInstancesEqual(a: Puzzle5dHoverInstance | null, b: Puzzle5dHoverInstance | null): boolean {
  if (a === b) {
    return true;
  }
  if (!a || !b) {
    return false;
  }
  if (a.kind !== b.kind) {
    return false;
  }
  if (a.kind === "anchor") {
    return b.kind === "anchor" && a.fullId === b.fullId;
  }
  return a.id === (b as { readonly id: string }).id;
}

/** @emoji 🖱️ Compares two unified hover-focus snapshots for equality. */
export function puzzle5dHoverFocusEqual(a: HoverFocusSnapshot, b: HoverFocusSnapshot): boolean {
  return puzzle5dHoverInstancesEqual(a.instance, b.instance) && puzzle5dKindHoversEqual(a.kindHover, b.kindHover);
}

function puzzle5dKindHoverFrom2d(kind: Puzzle2dKindHover): Puzzle5dKindHover | null {
  switch (kind.domain) {
    case "node":
      return { domain: "part", kindId: kind.kindId };
    case "handle":
      return { domain: "anchor", kindId: kind.kindId };
    case "edge":
      return { domain: "tie", kindId: kind.kindId };
    default:
      return null;
  }
}

function puzzle5dKindHoverTo2d(kind: Puzzle5dKindHover): Puzzle2dKindHover {
  switch (kind.domain) {
    case "part":
      return { domain: "node", kindId: kind.kindId };
    case "anchor":
      return { domain: "handle", kindId: kind.kindId };
    case "tie":
      return { domain: "edge", kindId: kind.kindId };
  }
}

function puzzle5dKindHoverFrom3d(kind: Puzzle3dKindHover): Puzzle5dKindHover {
  switch (kind.domain) {
    case "object":
      return { domain: "part", kindId: kind.kindId };
    case "vortex":
      return { domain: "anchor", kindId: kind.kindId };
    case "attraction":
      return { domain: "tie", kindId: kind.kindId };
  }
}

function puzzle5dKindHoverTo3d(kind: Puzzle5dKindHover): Puzzle3dKindHover {
  switch (kind.domain) {
    case "part":
      return { domain: "object", kindId: kind.kindId };
    case "anchor":
      return { domain: "vortex", kindId: kind.kindId };
    case "tie":
      return { domain: "attraction", kindId: kind.kindId };
  }
}

/** @emoji 🖱️ Resolves a flat graph element id to a unified hover instance. */
export function puzzle5dHoverInstanceFrom2dGraphId(fixture2d: Puzzle2dFixtureV1, graphId: string): Puzzle5dHoverInstance | null {
  if (fixture2d.nodes.some((node) => node.id === graphId)) {
    return { kind: "part", id: graphId };
  }
  if (fixture2d.edges.some((edge) => edge.id === graphId)) {
    return { kind: "tie", id: graphId };
  }
  for (const node of fixture2d.nodes) {
    if (node.handles.some((handle) => handle.id === graphId)) {
      return { kind: "anchor", fullId: graphId };
    }
  }
  return null;
}

function puzzle5dHoverInstanceFrom3dTarget(target: HoverTarget): Puzzle5dHoverInstance | null {
  switch (target.kind) {
    case "object":
      return { kind: "part", id: target.id };
    case "vortex":
      return { kind: "anchor", fullId: target.fullId };
    case "attraction":
      return { kind: "tie", id: target.id };
    default:
      return null;
  }
}

/** @emoji 🖱️ Maps flat canvas hover to unified store hover focus. */
export function hoverFocusFrom2dPayload(
  fixture2d: Puzzle2dFixtureV1,
  payload: Pick<Puzzle2dHoverPayload, "id" | "kind">,
): HoverFocusSnapshot {
  if (!payload.id && !payload.kind) {
    return EMPTY_HOVER_FOCUS;
  }
  if (payload.id) {
    return { instance: puzzle5dHoverInstanceFrom2dGraphId(fixture2d, payload.id), kindHover: null };
  }
  const kindHover = payload.kind ? puzzle5dKindHoverFrom2d(payload.kind) : null;
  return { instance: null, kindHover };
}

/** @emoji 🖱️ Maps volume canvas hover to unified store hover focus. */
export function hoverFocusFrom3dPayload(payload: Puzzle3dHoverPayload): HoverFocusSnapshot {
  if (!payload.hoverTarget && !payload.kindHover) {
    return EMPTY_HOVER_FOCUS;
  }
  if (payload.hoverTarget) {
    return { instance: puzzle5dHoverInstanceFrom3dTarget(payload.hoverTarget), kindHover: null };
  }
  return { instance: null, kindHover: payload.kindHover ? puzzle5dKindHoverFrom3d(payload.kindHover) : null };
}

/** @emoji 🖱️ Projects unified hover focus onto flat controlled canvas props. */
export function fiveD2dHoverFromStore(focus: HoverFocusSnapshot): { hoveredId: string | null; kindHover: Puzzle2dKindHover | null } {
  if (focus.instance) {
    switch (focus.instance.kind) {
      case "part":
        return { hoveredId: focus.instance.id, kindHover: null };
      case "anchor":
        return { hoveredId: focus.instance.fullId, kindHover: null };
      case "tie":
        return { hoveredId: focus.instance.id, kindHover: null };
    }
  }
  return {
    hoveredId: null,
    kindHover: focus.kindHover ? puzzle5dKindHoverTo2d(focus.kindHover) : null,
  };
}

/** @emoji 🖱️ Projects unified hover focus onto volume controlled canvas props. */
export function fiveD3dHoverFromStore(focus: HoverFocusSnapshot): { hoverTarget: HoverTarget | null; kindHover: Puzzle3dKindHover | null } {
  if (focus.instance) {
    switch (focus.instance.kind) {
      case "part":
        return { hoverTarget: { kind: "object", id: focus.instance.id }, kindHover: null };
      case "anchor":
        return { hoverTarget: { kind: "vortex", fullId: focus.instance.fullId }, kindHover: null };
      case "tie":
        return { hoverTarget: { kind: "attraction", id: focus.instance.id }, kindHover: null };
    }
  }
  return {
    hoverTarget: null,
    kindHover: focus.kindHover ? puzzle5dKindHoverTo3d(focus.kindHover) : null,
  };
}

//#endregion 🔖Hover

export interface V1 {
  readonly schema: "puzzle.5d/v1";
  readonly label?: string;
  readonly domain: DomainKind;
  readonly meta?: Record<string, unknown>;
  readonly kindCatalogs?: KindCatalogBundle;
  readonly kindCompatibility?: readonly KindCompatEntry[];
  readonly camera2d: Puzzle2dCameraState;
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
  const flatCam = r.camera2d as Puzzle2dCameraState | undefined;
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
export function compose5d(fixture2d: Puzzle2dFixtureV1, fixture3d: Puzzle3dFixtureV1): V1 {
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
    ties.push({
      id: edge.id,
      source: edge.source,
      target: edge.target,
      ...(edge.edgeKind !== undefined ? { tieKind: edge.edgeKind } : {}),
    });
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
export function project2d(model: V1): Puzzle2dFixtureV1 {
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
    edges: model.ties.map((b) => ({
      id: b.id,
      source: b.source,
      target: b.target,
      ...(b.tieKind !== undefined ? { edgeKind: b.tieKind } : {}),
    })),
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

//#region 🔖Brush
export type Puzzle5dActiveTool = Puzzle2dActiveTool;

export const PUZZLE_5D_FILL_COUNT_MAX = PUZZLE_3D_FILL_COUNT_MAX;

export const PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID = "puzzle5d.tool.brush";
export const PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID = "puzzle5d.tool.select";
export const PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID = "puzzle5d.tool.fill";

/** @emoji 🖌️ One unified brush placement growing a {@link PartV1} with both flat and volume aspects. */
export interface Puzzle5dBrushPlacement {
  readonly partId?: string;
  readonly partKind: string;
  readonly sourceAnchorFullId: string;
  readonly aspect2d?: Puzzle2dBrushPlacePayload;
  readonly aspect3d?: BrushPlacePayload;
  readonly tieId?: string;
}

/** @emoji 🪣 Cached fill prefix session at unified model level. */
export interface Puzzle5dFillSession {
  readonly baseModel: V1;
  readonly sequence: readonly Puzzle5dBrushPlacement[];
  readonly seed: number;
}

export type Puzzle5dBrushPlacementApplyResult =
  | { readonly kind: "unchanged" }
  | { readonly kind: "placed"; readonly model: V1; readonly partId: string; readonly tieId: string };

function cloneModel(model: V1): V1 {
  return JSON.parse(JSON.stringify(model)) as V1;
}

function partById(model: V1, partId: string): PartV1 | undefined {
  return model.parts.find((row) => row.id === partId);
}

function peerPartForKind(model: V1, partKind: string): PartV1 | undefined {
  return model.parts.find((row) => row.partKind === partKind && row.puzzle2d && row.puzzle3d);
}

function objectKind3d(catalogs: KindCatalogBundle | undefined, partKind: string): Puzzle3dPartKind | undefined {
  return project3dKindCatalogs(catalogs)?.objects?.find((row) => row.id === partKind);
}

function volumeTemplatesForPartKind(model: V1, partKind: string, catalogs: KindCatalogBundle | undefined): NonNullable<Puzzle3dPartKind["vortices"]> | undefined {
  const fromCatalog = objectKind3d(catalogs, partKind)?.vortices;
  if (fromCatalog?.length) return fromCatalog;
  const peer = peerPartForKind(model, partKind);
  if (!peer) return undefined;
  const templates = peer.anchors
    .filter((anchor) => anchor.puzzle3d)
    .map((anchor) => ({
      vortexKind: anchor.anchorKind,
      position: anchor.puzzle3d!.position,
      ...(anchor.puzzle3d!.direction ? { direction: anchor.puzzle3d!.direction } : {}),
      ...(anchor.puzzle3d!.radius !== undefined ? { radius: anchor.puzzle3d!.radius } : {}),
    }));
  return templates.length ? templates : undefined;
}

function anchorLocalIdFromIndex(index: number): string {
  return `v${index}`;
}

function slugAnchorLocalId(kind: string): string {
  const slug = kind.trim().replace(/\s+/g, "-").toLowerCase();
  return slug.length > 0 ? slug.slice(0, 48) : "link";
}

function flatTemplatesFromObjectKind(kind: Puzzle3dPartKind | undefined): readonly { readonly angle: number; readonly handleKind: string; readonly radius?: number }[] {
  const count = Math.max(kind?.vortices?.length ?? 0, 1);
  return Array.from({ length: count }, (_, index) => ({
    angle: flatHandleConnectorAngle(index, count),
    handleKind: kind?.vortices?.[index]?.vortexKind ?? BUILTIN_PORT_HANDLE_KIND,
    radius: 3,
  }));
}

function buildAnchorsUnified(
  partId: string,
  partKind: string,
  catalogs: KindCatalogBundle | undefined,
  flatTemplates?: readonly { readonly angle: number; readonly handleKind: string; readonly radius?: number }[],
  volumeTemplates?: Puzzle3dPartKind["vortices"],
): AnchorV1[] {
  const kind3d = objectKind3d(catalogs, partKind);
  const vortices = volumeTemplates ?? kind3d?.vortices ?? [];
  const flat = flatTemplates ?? flatTemplatesFromObjectKind(kind3d);
  const count = Math.max(vortices.length, flat.length, 1);
  const anchors: AnchorV1[] = [];
  for (let index = 0; index < count; index += 1) {
    const vortex = vortices[index];
    const handle = flat[index] ?? flat[0];
    const localId = vortex?.vortexKind ? slugAnchorLocalId(vortex.vortexKind) : anchorLocalIdFromIndex(index);
    anchors.push({
      id: localId,
      anchorKind: vortex?.vortexKind ?? handle?.handleKind ?? BUILTIN_PORT_HANDLE_KIND,
      ...(handle
        ? {
            puzzle2d: {
              angle: handle.angle,
              anchorKind: handle.handleKind,
              ...(handle.radius !== undefined ? { radius: handle.radius } : {}),
            },
          }
        : {}),
      ...(vortex
        ? {
            puzzle3d: {
              position: vortex.position,
              ...(vortex.direction ? { direction: vortex.direction } : {}),
              ...(vortex.radius !== undefined ? { radius: vortex.radius } : {}),
              ...(vortex.vortexKind ? { label: vortex.vortexKind } : {}),
            },
          }
        : {}),
    });
  }
  return anchors;
}

function mergeAnchorsFlatAndVolume(
  volumeAnchors: readonly AnchorV1[],
  flatHandles: readonly { readonly id: string; readonly angle: number; readonly handleKind: string; readonly radius?: number }[],
): AnchorV1[] {
  const count = Math.max(volumeAnchors.length, flatHandles.length);
  const out: AnchorV1[] = [];
  for (let index = 0; index < count; index += 1) {
    const volume = volumeAnchors[index];
    const flat = flatHandles[index];
    const localId = volume?.id ?? parseAnchorFullId(flat?.id ?? "")?.anchorId ?? anchorLocalIdFromIndex(index);
    out.push({
      id: localId,
      anchorKind: volume?.anchorKind ?? flat?.handleKind ?? BUILTIN_PORT_HANDLE_KIND,
      ...(flat
        ? {
            puzzle2d: {
              angle: flat.angle,
              anchorKind: flat.handleKind,
              ...(flat.radius !== undefined ? { radius: flat.radius } : {}),
            },
          }
        : volume?.puzzle2d
          ? { puzzle2d: volume.puzzle2d }
          : {}),
      ...(volume?.puzzle3d ? { puzzle3d: volume.puzzle3d } : {}),
    });
  }
  return out;
}

function volumeAnchorIndexOnPart(part: PartV1, anchorLocalId: string): number {
  return part.anchors.filter((anchor) => anchor.puzzle3d).findIndex((anchor) => anchor.id === anchorLocalId);
}

function synthesizeVolumeAspectFromFlat(
  model: V1,
  payload: Puzzle2dBrushPlacePayload,
  partKind: string,
  catalogs: KindCatalogBundle | undefined,
): { readonly aspect: Puzzle3dPartAspect; readonly anchors: AnchorV1[] } | null {
  const parsed = parseAnchorFullId(payload.sourceHandleId);
  if (!parsed) return null;
  const sourcePart = partById(model, parsed.partId);
  if (!sourcePart?.puzzle3d) return null;
  const fixture3d = project3d(model);
  const hostObject = fixture3d.objects.find((row) => row.id === parsed.partId);
  if (!hostObject) return null;
  const volumeIndex = volumeAnchorIndexOnPart(sourcePart, parsed.anchorId);
  if (volumeIndex < 0) return null;
  const world = vortexWorldCadFromObject(hostObject, volumeIndex);
  if (!world) return null;
  const cat3d = project3dKindCatalogs(catalogs);
  const kind = objectKind3d(catalogs, partKind);
  const templates = volumeTemplatesForPartKind(model, partKind, catalogs);
  const template = templates?.[payload.targetHandleIndex];
  const meshUrl = resolveObjectKindMeshUrl(partKind, cat3d, fixture3d) ?? peerPartForKind(model, partKind)?.puzzle3d?.meshUrl;
  if (!meshUrl || !template) return null;
  const targetAnchor = sourcePart.anchors.find((anchor) => anchor.id === parsed.anchorId);
  const sourceVk = template.vortexKind ?? "";
  const targetVk = targetAnchor?.anchorKind ?? "";
  const useHostOrientation = brushPlacementUsesHostOrientation(
    { objectId: parsed.partId, objectKind: sourcePart.partKind, vortexKind: targetVk },
    sourceVk,
    partKind,
  );
  const pose = computeBrushPlacementPose({
    sourceLocalPosition: template.position,
    sourceLocalDirection: template.direction ?? [0, 0, -1],
    ...(kind?.scale !== undefined ? { scale: kind.scale } : {}),
    targetWorldPositionCad: world.position,
    targetWorldDirectionCad: world.direction,
    ...(hostObject.orientation ? { referenceOrientationCad: hostObject.orientation } : {}),
    useHostOrientation,
  });
  const anchors = buildAnchorsUnified("", partKind, catalogs, payload.handles, templates);
  return {
    aspect: {
      origin: pose.origin,
      orientation: pose.orientation,
      meshUrl,
      ...(kind?.scale !== undefined ? { scale: kind.scale } : {}),
      ...(kind?.label ?? kind?.name ? { label: kind.label ?? kind.name } : {}),
    },
    anchors,
  };
}

function synthesizeVolumeAspectFromBrushPayload(
  model: V1,
  payload: BrushPlacePayload,
  partKind: string,
  catalogs: KindCatalogBundle | undefined,
): { readonly aspect: Puzzle3dPartAspect; readonly anchors: AnchorV1[] } | null {
  const cat3d = project3dKindCatalogs(catalogs);
  const kind = objectKind3d(catalogs, partKind);
  const meshUrl = resolveObjectKindMeshUrl(partKind, cat3d, project3d(model));
  if (!meshUrl) return null;
  const templates = volumeTemplatesForPartKind(model, partKind, catalogs);
  const anchors = buildAnchorsUnified("", partKind, catalogs, undefined, templates);
  return {
    aspect: {
      origin: payload.origin,
      orientation: payload.orientation,
      meshUrl,
      ...(payload.scale !== undefined ? { scale: payload.scale } : kind?.scale !== undefined ? { scale: kind.scale } : {}),
      label: kind?.label ?? kind?.name ?? partKind,
    },
    anchors,
  };
}

function synthesizeFlatAspectFromVolume(model: V1, payload: BrushPlacePayload, partKind: string): NodeAspect | null {
  const parsed = parseAnchorFullId(payload.targetVortexFullId);
  if (!parsed) return null;
  const sourcePart = partById(model, parsed.partId);
  if (!sourcePart?.puzzle2d) return null;
  const sourceAnchor = sourcePart.anchors.find((anchor) => anchor.id === parsed.anchorId);
  const angle = sourceAnchor?.puzzle2d?.angle ?? flatHandleConnectorAngle(0, 1);
  const gap = DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX / 2;
  const x = sourcePart.puzzle2d.x + gap * Math.cos(angle);
  const y = sourcePart.puzzle2d.y + gap * Math.sin(angle);
  const peer = peerPartForKind(model, partKind);
  const iconKind = peer?.puzzle2d?.iconKind;
  if ((peer?.puzzle2d?.shape ?? "rectangle") === "rectangle") {
    return {
      x,
      y,
      shape: "rectangle",
      width: peer?.puzzle2d?.width ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
      height: peer?.puzzle2d?.height ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
      ...(iconKind ? { iconKind } : {}),
    };
  }
  return {
    x,
    y,
    shape: "circle",
    radius: peer?.puzzle2d?.radius ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX / 2,
    ...(iconKind ? { iconKind } : {}),
  };
}

/** @emoji 🖌️ Builds a unified placement from a flat WASM brush payload. */
export function puzzle5dBrushPlacementFromFlat(payload: Puzzle2dBrushPlacePayload): Puzzle5dBrushPlacement {
  return {
    partKind: payload.nodeKind,
    sourceAnchorFullId: payload.sourceHandleId,
    aspect2d: payload,
    ...(payload.nodeId ? { partId: payload.nodeId } : {}),
  };
}

/** @emoji 🖌️ Builds a unified placement from a volume brush payload. */
export function puzzle5dBrushPlacementFromVolume(payload: BrushPlacePayload): Puzzle5dBrushPlacement {
  return {
    partKind: payload.objectKindId,
    sourceAnchorFullId: payload.targetVortexFullId,
    aspect3d: payload,
    ...(payload.objectId ? { partId: payload.objectId } : {}),
  };
}

/** @emoji 🖌️ Appends one unified part and tie from a brush placement. */
export function applyBrushPlacementToModel(model: V1, placement: Puzzle5dBrushPlacement): Puzzle5dBrushPlacementApplyResult {
  const partId =
    placement.partId?.trim() ||
    placement.aspect2d?.nodeId?.trim() ||
    placement.aspect3d?.objectId?.trim() ||
    `puzzle5d.brush.${crypto.randomUUID()}`;
  const partKind = placement.partKind;
  const catalogs = model.kindCatalogs;
  let puzzle2d: NodeAspect | undefined;
  let puzzle3d: Puzzle3dPartAspect | undefined;
  let anchors: AnchorV1[];
  let tieSource = "";
  let tieTarget = "";

  if (placement.aspect2d) {
    const payload = placement.aspect2d;
    const flatHandles = puzzle2dFixtureHandlesFromNodeKind(partId, payload.handles);
    const targetHandle = flatHandles[payload.targetHandleIndex];
    if (!targetHandle) return { kind: "unchanged" };
    tieSource = payload.sourceHandleId;
    tieTarget = targetHandle.id;
    const iconKind = payload.iconKind;
    puzzle2d =
      payload.shape === "rectangle"
        ? {
            x: payload.x,
            y: payload.y,
            shape: "rectangle",
            width: payload.width ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
            height: payload.height ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
            ...(iconKind ? { iconKind } : {}),
          }
        : {
            x: payload.x,
            y: payload.y,
            shape: "circle",
            radius: payload.radius ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX / 2,
            ...(iconKind ? { iconKind } : {}),
          };
    const volume = synthesizeVolumeAspectFromFlat(model, payload, partKind, catalogs);
    if (!volume) return { kind: "unchanged" };
    puzzle3d = volume.aspect;
    anchors = mergeAnchorsFlatAndVolume(volume.anchors, flatHandles);
  } else if (placement.aspect3d) {
    const payload = placement.aspect3d;
    const volume = synthesizeVolumeAspectFromBrushPayload(model, payload, partKind, catalogs);
    if (!volume) return { kind: "unchanged" };
    puzzle3d = volume.aspect;
    anchors = volume.anchors;
    const matingLocal = anchors[payload.sourceVortexIndex]?.id;
    if (!matingLocal) return { kind: "unchanged" };
    tieSource = anchorFullId(partId, matingLocal);
    tieTarget = payload.targetVortexFullId;
    const flat = synthesizeFlatAspectFromVolume(model, payload, partKind);
    if (!flat) return { kind: "unchanged" };
    puzzle2d = flat;
  } else {
    return { kind: "unchanged" };
  }

  if (model.parts.some((row) => row.id === partId)) return { kind: "unchanged" };
  if (model.ties.some((tie) => tie.source === tieSource && tie.target === tieTarget)) return { kind: "unchanged" };

  const tieId = placement.tieId?.trim() || `puzzle5d.brush.tie.${crypto.randomUUID()}`;
  const part: PartV1 = {
    id: partId,
    partKind,
    puzzle2d,
    puzzle3d,
    anchors,
  };
  return {
    kind: "placed",
    model: {
      ...model,
      parts: [...model.parts, part],
      ties: [...model.ties, { id: tieId, source: tieSource, target: tieTarget }],
    },
    partId,
    tieId,
  };
}

/** @emoji 🪣 Applies an ordered brush prefix onto a base unified model. */
export function applyFillPlacementsToModel(base: V1, placements: readonly Puzzle5dBrushPlacement[]): V1 {
  let next = base;
  for (const placement of placements) {
    const result = applyBrushPlacementToModel(next, placement);
    if (result.kind === "placed") {
      next = result.model;
    }
  }
  return next;
}

/** @emoji 🪣 Volume-authoritative fill sequence mapped to unified placements (2d aspects synthesized). */
export function buildPuzzle5dFillSequence(args: {
  readonly model: V1;
  readonly seed: number;
  readonly maxCount?: number;
  readonly overlapBudget?: number;
  readonly meshRootForUrl: (meshUrl: string) => Object3D | null | undefined;
  readonly weights?: Puzzle3dBrushKindWeights;
}): readonly Puzzle5dBrushPlacement[] {
  const fixture3d = project3d(args.model);
  const cat3d = project3dKindCatalogs(args.model.kindCatalogs);
  const payloads = buildBrushFillSequence({
    baseFixture: fixture3d,
    seed: args.seed,
    maxCount: args.maxCount,
    overlapBudget: args.overlapBudget,
    meshRootForUrl: args.meshRootForUrl,
    kindCatalogs: cat3d,
    kindCompatibility: args.model.kindCompatibility,
    ...(args.weights ? { weights: args.weights } : {}),
  });
  return payloads.map((payload) => puzzle5dBrushPlacementFromVolume(payload));
}
//#endregion 🔖Brush

//#region 🔖PaletteDrop
function nodeAspectFromPaletteNode(node: Puzzle2dFixtureNodeV1): NodeAspect {
  if (node.shape === "rectangle") {
    return {
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
    };
  }
  return {
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
}

function paletteVolumeOriginFromFlat(peer: PartV1, flatX: number, flatY: number): Vec3 {
  const flat = peer.puzzle2d!;
  const volume = peer.puzzle3d!;
  return [volume.origin[0] + (flatX - flat.x), volume.origin[1] - (flatY - flat.y), volume.origin[2]];
}

function paletteFlatCenterFromVolume(peer: PartV1, origin: Vec3): { readonly x: number; readonly y: number } {
  const flat = peer.puzzle2d!;
  const volume = peer.puzzle3d!;
  return { x: flat.x + (origin[0] - volume.origin[0]), y: flat.y - (origin[1] - volume.origin[1]) };
}

function clearFlatPaletteDragSession(): void {
  puzzle2dFixturePaletteDropCommittedRef.current = true;
  abortPuzzle2dFixturePaletteDrag();
}

function clearVolumePaletteDragSession(): void {
  cancelPuzzle3dFixturePalettePointerDrag();
}

/** @emoji 📥 Builds one unified part from a flat palette node drop. */
export function partFromPaletteNodeDrop(model: V1, node: Puzzle2dFixtureNodeV1): PartV1 | null {
  const partKind = node.nodeKind?.trim();
  if (!partKind) {
    return null;
  }
  const catalogs = model.kindCatalogs;
  const cat3d = project3dKindCatalogs(catalogs);
  const kind = objectKind3d(catalogs, partKind);
  const meshUrl = resolveObjectKindMeshUrl(partKind, cat3d, project3d(model));
  if (!kind || !meshUrl) {
    return null;
  }
  const peer = peerPartForKind(model, partKind);
  const origin = peer?.puzzle2d && peer?.puzzle3d ? paletteVolumeOriginFromFlat(peer, node.x, node.y) : ([node.x, -node.y, 0] as Vec3);
  const flatHandles = node.handles.map((handle) => ({
    angle: handle.angle,
    handleKind: handle.handleKind,
    ...(handle.radius !== undefined ? { radius: handle.radius } : {}),
  }));
  const volumeTemplates = volumeTemplatesForPartKind(model, partKind, catalogs);
  const anchors = buildAnchorsUnified(node.id, partKind, catalogs, flatHandles, volumeTemplates);
  return {
    id: node.id,
    partKind,
    puzzle2d: nodeAspectFromPaletteNode(node),
    puzzle3d: {
      origin,
      meshUrl,
      orientation: peer?.puzzle3d?.orientation ?? ([0, 0, 0, 1] as Quat),
      ...(kind.scale !== undefined ? { scale: kind.scale } : peer?.puzzle3d?.scale !== undefined ? { scale: peer.puzzle3d.scale } : {}),
      ...(kind.label ?? kind.name ? { label: kind.label ?? kind.name } : {}),
    },
    anchors,
  };
}

/** @emoji 📥 Builds one unified part from a volume palette object drop. */
export function partFromPaletteObjectDrop(model: V1, object: Puzzle3dFixtureObjectV1): PartV1 | null {
  const partKind = object.objectKind?.trim();
  if (!partKind) {
    return null;
  }
  const catalogs = model.kindCatalogs;
  const template = buildPaletteNodeDragFixture(partKind, catalogs).nodes[0];
  if (!template) {
    return null;
  }
  const peer = peerPartForKind(model, partKind);
  const center = peer?.puzzle2d && peer?.puzzle3d ? paletteFlatCenterFromVolume(peer, object.origin) : { x: object.origin[0], y: -object.origin[1] };
  const flatNode: Puzzle2dFixtureNodeV1 = {
    ...template,
    id: object.id,
    x: center.x,
    y: center.y,
    handles: template.handles.map((handle, index) => ({ ...handle, id: `${object.id}.h${index}` })),
  };
  const volumeAnchors: AnchorV1[] = object.vortices.map((vortex, index) => {
    const parsed = parseAnchorFullId(vortex.id.includes(":") ? vortex.id : anchorFullId(object.id, vortex.id));
    return {
      id: parsed?.anchorId ?? anchorLocalIdFromIndex(index),
      anchorKind: vortex.vortexKind ?? BUILTIN_PORT_HANDLE_KIND,
      puzzle3d: {
        position: vortex.position,
        ...(vortex.direction !== undefined ? { direction: vortex.direction } : {}),
        ...(vortex.radius !== undefined ? { radius: vortex.radius } : {}),
        ...(vortex.label !== undefined ? { label: vortex.label } : {}),
      },
    };
  });
  const flatHandles = flatNode.handles.map((handle) => ({
    id: handle.id,
    angle: handle.angle,
    handleKind: handle.handleKind,
    ...(handle.radius !== undefined ? { radius: handle.radius } : {}),
  }));
  const anchors = mergeAnchorsFlatAndVolume(volumeAnchors, flatHandles);
  return {
    id: object.id,
    partKind,
    puzzle2d: nodeAspectFromPaletteNode(flatNode),
    puzzle3d: {
      origin: object.origin,
      meshUrl: object.meshUrl,
      ...(object.orientation !== undefined ? { orientation: object.orientation } : {}),
      ...(object.scale !== undefined ? { scale: object.scale } : {}),
      ...(object.label !== undefined ? { label: object.label } : {}),
      ...(object.wormhole === true ? { wormhole: true } : {}),
    },
    anchors,
  };
}
//#endregion 🔖PaletteDrop

//#region 🔖StructuralDelete
function anchorFullIdsForPart(part: PartV1): readonly string[] {
  return part.anchors.map((anchor) => anchorFullId(part.id, anchor.id));
}

function tieTouchesPartOrAnchors(tie: TieV1, partId: string, anchorIds: ReadonlySet<string>): boolean {
  if (tie.source === partId || tie.target === partId) {
    return true;
  }
  return anchorIds.has(tie.source) || anchorIds.has(tie.target);
}

/** @emoji 🗑️ Removes one unified part and every tie touching it or its anchors. */
export function removePartFromModel(model: V1, partId: string): V1 | null {
  const part = model.parts.find((row) => row.id === partId);
  if (!part) {
    return null;
  }
  const anchorIds = new Set(anchorFullIdsForPart(part));
  return {
    ...model,
    parts: model.parts.filter((row) => row.id !== partId),
    ties: model.ties.filter((tie) => !tieTouchesPartOrAnchors(tie, partId, anchorIds)),
  };
}

/** @emoji 🗑️ Removes one anchor and ties that referenced it. */
export function removeAnchorFromModel(model: V1, fullAnchorId: string): V1 | null {
  const parsed = parseAnchorFullId(fullAnchorId);
  if (!parsed) {
    return null;
  }
  const part = model.parts.find((row) => row.id === parsed.partId);
  if (!part || !part.anchors.some((anchor) => anchor.id === parsed.anchorId)) {
    return null;
  }
  return {
    ...model,
    parts: model.parts.map((row) =>
      row.id !== parsed.partId ? row : { ...row, anchors: row.anchors.filter((anchor) => anchor.id !== parsed.anchorId) },
    ),
    ties: model.ties.filter((tie) => tie.source !== fullAnchorId && tie.target !== fullAnchorId),
  };
}

/** @emoji 🗑️ Removes one tie row by id. */
export function removeTieFromModel(model: V1, tieId: string): V1 | null {
  if (!model.ties.some((tie) => tie.id === tieId)) {
    return null;
  }
  return { ...model, ties: model.ties.filter((tie) => tie.id !== tieId) };
}

/** @emoji 🗑️ Applies a flat canvas structural delete onto the unified {@link V1} model. */
export function applyStructuralDelete2dToModel(model: V1, payload: Puzzle2dStructureDeletePayload): V1 | null {
  if (payload.kind === "wire") {
    return null;
  }
  if (payload.kind === "edge") {
    return removeTieFromModel(model, payload.id);
  }
  return removePartFromModel(model, payload.id);
}

function pruneSelectionAfterModelEdit(selection: SelectionSnapshot, _prevModel: V1, nextModel: V1): SelectionSnapshot {
  const remainingPartIds = new Set(nextModel.parts.map((part) => part.id));
  const remainingAnchorIds = new Set(nextModel.parts.flatMap((part) => anchorFullIdsForPart(part)));
  return {
    partIds: selection.partIds.filter((id) => remainingPartIds.has(id)),
    anchorIds: selection.anchorIds.filter((id) => remainingAnchorIds.has(id)),
  };
}
//#endregion 🔖StructuralDelete

//#region 🔖Store
export interface StoreSnapshot {
  readonly model: V1;
  readonly selection: SelectionSnapshot;
  readonly hoverFocus: HoverFocusSnapshot;
  readonly connectSession: ConnectSession | null;
  readonly cameras: Readonly<Record<string, { readonly "2d": Puzzle2dCameraState; readonly "3d": Puzzle3dFixtureV1["camera"] }>>;
  readonly fillCount: number;
  readonly fillBuildDone: boolean;
}

export class Store {
  private listeners = new Set<() => void>();
  private snapshot: StoreSnapshot;
  private fillSession: Puzzle5dFillSession | null = null;

  constructor(model: V1) {
    this.snapshot = {
      model,
      selection: { partIds: [], anchorIds: [] },
      hoverFocus: EMPTY_HOVER_FOCUS,
      connectSession: null,
      cameras: {},
      fillCount: 0,
      fillBuildDone: true,
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

  get2dCamera(instanceId: string): Puzzle2dCameraState {
    return this.snapshot.cameras[instanceId]?.["2d"] ?? this.snapshot.model.camera2d;
  }

  get3dCamera(instanceId: string): Puzzle3dFixtureV1["camera"] {
    return this.snapshot.cameras[instanceId]?.["3d"] ?? this.snapshot.model.camera3d;
  }

  set2dCamera(instanceId: string, camera: Puzzle2dCameraState): void {
    const prev = this.snapshot.cameras[instanceId]?.["2d"] ?? this.snapshot.model.camera2d;
    if (prev.x === camera.x && prev.y === camera.y && prev.zoom === camera.zoom) {
      return;
    }
    const prevEntry = this.snapshot.cameras[instanceId];
    this.snapshot = {
      ...this.snapshot,
      cameras: {
        ...this.snapshot.cameras,
        [instanceId]: { "2d": camera, "3d": prevEntry?.["3d"] ?? this.snapshot.model.camera3d },
      },
    };
  }

  set3dCamera(instanceId: string, camera: Puzzle3dFixtureV1["camera"]): void {
    const prev = this.snapshot.cameras[instanceId]?.["3d"] ?? this.snapshot.model.camera3d;
    if (
      prev.position[0] === camera.position[0] &&
      prev.position[1] === camera.position[1] &&
      prev.position[2] === camera.position[2] &&
      prev.target[0] === camera.target[0] &&
      prev.target[1] === camera.target[1] &&
      prev.target[2] === camera.target[2] &&
      prev.zoom === camera.zoom &&
      prev.projection === camera.projection
    ) {
      return;
    }
    const prevEntry = this.snapshot.cameras[instanceId];
    this.snapshot = {
      ...this.snapshot,
      cameras: {
        ...this.snapshot.cameras,
        [instanceId]: { "2d": prevEntry?.["2d"] ?? this.snapshot.model.camera2d, "3d": camera },
      },
    };
  }

  setSelection(selection: SelectionSnapshot): void {
    const prev = this.snapshot.selection;
    if (
      prev.partIds.length === selection.partIds.length &&
      prev.anchorIds.length === selection.anchorIds.length &&
      prev.partIds.every((id, index) => id === selection.partIds[index]) &&
      prev.anchorIds.every((id, index) => id === selection.anchorIds[index])
    ) {
      return;
    }
    this.setSnapshot({ ...this.snapshot, selection });
  }

  /** @emoji 🖱️ Updates shared hover focus synchronized across paired 2d and 3d surfaces. */
  setHoverFocus(focus: HoverFocusSnapshot): void {
    if (puzzle5dHoverFocusEqual(this.snapshot.hoverFocus, focus)) {
      return;
    }
    this.setSnapshot({ ...this.snapshot, hoverFocus: focus });
  }

  /** @emoji 🖱️ Commits flat canvas hover into the unified store. */
  setHoverFocusFrom2d(fixture2d: Puzzle2dFixtureV1, payload: Pick<Puzzle2dHoverPayload, "id" | "kind">): void {
    this.setHoverFocus(hoverFocusFrom2dPayload(fixture2d, payload));
  }

  /** @emoji 🖱️ Commits volume canvas hover into the unified store. */
  setHoverFocusFrom3d(payload: Puzzle3dHoverPayload): void {
    this.setHoverFocus(hoverFocusFrom3dPayload(payload));
  }

  /** @emoji 🔗 Sets cross-surface indirect preview state; callers must not use this for proximity snaps. */
  setConnectSession(session: ConnectSession | null): void {
    this.setSnapshot({ ...this.snapshot, connectSession: session });
  }

  applyNodeMove(partId: string, x: number, y: number): void {
    this.applyNodeMoves([{ id: partId, x, y }]);
  }

  applyNodeMoves(moves: ReadonlyArray<{ readonly id: string; readonly x: number; readonly y: number }>): void {
    if (moves.length === 0) {
      return;
    }
    const byId = new Map(moves.map((move) => [move.id, move]));
    const parts = this.snapshot.model.parts.map((p) => {
      const move = byId.get(p.id);
      if (!move || !p.puzzle2d) {
        return p;
      }
      return { ...p, puzzle2d: { ...p.puzzle2d, x: move.x, y: move.y } };
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }

  /** @emoji 🕸️ Batch-updates flat node centers after a WASM force-graph tick. */
  applyFlatNodeCenters(centers: ReadonlyMap<string, { readonly x: number; readonly y: number }>): void {
    if (centers.size === 0) return;
    const parts = this.snapshot.model.parts.map((p) => {
      const center = centers.get(p.id);
      if (center == null || !p.puzzle2d) return p;
      return { ...p, puzzle2d: { ...p.puzzle2d, x: center.x, y: center.y } };
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
    const ties = this.snapshot.model.ties;
    if (ties.some((tie) => tie.source === source && tie.target === target)) {
      this.setSnapshot({ ...this.snapshot, connectSession: null });
      return;
    }
    const id = crypto.randomUUID();
    const nextTies: TieV1[] = [...ties, { id, source, target, ...(tieKind ? { tieKind } : {}) }];
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, ties: nextTies },
      connectSession: null,
    });
  }

  replaceModel(model: V1): void {
    this.fillSession = null;
    this.setSnapshot({
      ...this.snapshot,
      model,
      connectSession: null,
      fillCount: 0,
      fillBuildDone: true,
    });
  }

  getFillSession(): Puzzle5dFillSession | null {
    return this.fillSession;
  }

  /** @emoji 🪣 Captures a fill prefix session against the current or supplied base model. */
  prepareFillSession(sequence: readonly Puzzle5dBrushPlacement[], baseModel?: V1, seed = 0): void {
    this.fillSession = {
      baseModel: cloneModel(baseModel ?? this.snapshot.model),
      sequence,
      seed,
    };
    this.setSnapshot({ ...this.snapshot, fillCount: 0, fillBuildDone: true });
  }

  /** @emoji 🖌️ Commits one unified brush placement and clears any active fill session. */
  applyBrushPlacement(placement: Puzzle5dBrushPlacement): boolean {
    const result = applyBrushPlacementToModel(this.snapshot.model, placement);
    if (result.kind !== "placed") return false;
    this.fillSession = null;
    this.setSnapshot({ ...this.snapshot, model: result.model, connectSession: null, fillCount: 0, fillBuildDone: true });
    return true;
  }

  /** @emoji 🗑️ Applies a flat structural delete (node → part, edge → tie) onto the unified model. */
  applyStructuralDelete2d(payload: Puzzle2dStructureDeletePayload): boolean {
    const prevModel = this.snapshot.model;
    const nextModel = applyStructuralDelete2dToModel(prevModel, payload);
    if (!nextModel) {
      return false;
    }
    this.fillSession = null;
    this.setSnapshot({
      ...this.snapshot,
      model: nextModel,
      selection: pruneSelectionAfterModelEdit(this.snapshot.selection, prevModel, nextModel),
      connectSession: null,
      fillCount: 0,
      fillBuildDone: true,
    });
    return true;
  }

  /** @emoji 🗑️ Deletes the current unified selection (parts, anchors, ties). */
  applySelectionDelete(): boolean {
    const selection = this.snapshot.selection;
    if (selection.partIds.length === 0 && selection.anchorIds.length === 0) {
      return false;
    }
    let model = this.snapshot.model;
    let changed = false;
    for (const partId of selection.partIds) {
      const next = removePartFromModel(model, partId);
      if (next) {
        model = next;
        changed = true;
      }
    }
    for (const anchorId of selection.anchorIds) {
      const next = removeAnchorFromModel(model, anchorId);
      if (next) {
        model = next;
        changed = true;
      }
    }
    if (!changed) {
      return false;
    }
    this.fillSession = null;
    this.setSnapshot({
      ...this.snapshot,
      model,
      selection: { partIds: [], anchorIds: [] },
      connectSession: null,
      fillCount: 0,
      fillBuildDone: true,
    });
    return true;
  }

  /** @emoji 📥 Appends a unified part from a flat palette fixture drop. */
  applyPaletteNodeDrop(detail: Puzzle2dFixtureDropDetail): string | null {
    const node = mergePaletteNodeFromDrop(detail);
    if (!node) {
      return null;
    }
    const part = partFromPaletteNodeDrop(this.snapshot.model, node);
    if (!part) {
      return null;
    }
    this.fillSession = null;
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, parts: [...this.snapshot.model.parts, part] },
      selection: { partIds: [part.id], anchorIds: [] },
      connectSession: null,
      fillCount: 0,
      fillBuildDone: true,
    });
    clearFlatPaletteDragSession();
    return part.id;
  }

  /** @emoji 📥 Appends a unified part from a volume palette object drop. */
  applyPaletteObjectDrop(object: Puzzle3dFixtureObjectV1): string | null {
    const part = partFromPaletteObjectDrop(this.snapshot.model, object);
    if (!part) {
      return null;
    }
    this.fillSession = null;
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, parts: [...this.snapshot.model.parts, part] },
      selection: { partIds: [part.id], anchorIds: [] },
      connectSession: null,
      fillCount: 0,
      fillBuildDone: true,
    });
    clearVolumePaletteDragSession();
    return part.id;
  }

  /** @emoji 🪣 Applies a fill prefix count from the cached session onto the store model. */
  applyFillCount(count: number): void {
    if (!this.fillSession) return;
    const clamped = Math.max(0, Math.min(PUZZLE_5D_FILL_COUNT_MAX, Math.round(count)));
    const next = applyFillPlacementsToModel(this.fillSession.baseModel, this.fillSession.sequence.slice(0, clamped));
    this.setSnapshot({ ...this.snapshot, model: next, connectSession: null, fillCount: clamped, fillBuildDone: true });
  }

  setFillBuildDone(done: boolean): void {
    if (this.snapshot.fillBuildDone === done) return;
    this.setSnapshot({ ...this.snapshot, fillBuildDone: done });
  }

  /** @emoji 🪣 Restores the model captured at fill-session start and clears fill state. */
  clearFill(): void {
    if (!this.fillSession) return;
    this.setSnapshot({
      ...this.snapshot,
      model: cloneModel(this.fillSession.baseModel),
      connectSession: null,
      fillCount: 0,
      fillBuildDone: true,
    });
    this.fillSession = null;
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
  gridFactor: DEFAULT_PUZZLE_2D_GRID_FACTOR,
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
  readonly gumballConfig?: GumballConfig;
  /** @emoji 🕸️ When true, runs a continuous WASM force-graph layout on the flat surface (e.g. kit WIRES). */
  readonly liveForceGraph?: boolean;
  /** @emoji 🔗 Flat graph port model; WIRES surfaces use `normal` (node-id edges, no handles). */
  readonly graphPortMode?: Puzzle2dCanvasProps["graphPortMode"];
  /** @emoji 🖌️ Shared authoring tool for both surfaces (`select` | `brush` | `fill`). */
  readonly activeTool?: Puzzle5dActiveTool;
  readonly brushFlushDistance?: number;
  readonly brushOverlapBudget?: number;
  /** 2d surface overrides; LOD uses discrete tiers unless `automaticLod` is set on the canvas. */
  readonly puzzle2d?: Omit<Puzzle2dCanvasProps, "children">;
  /** 3d surface overrides; LOD is continuous/camera-driven — not the flat six-tier scale. */
  readonly puzzle3d?: Omit<Puzzle3dCanvasProps, "children">;
}

const FIVE_D_LIVE_FORCE_ITERS_PER_FRAME = 24;

/** @emoji 🕸️ Applies one WASM force-graph tick to a puzzle 5d store snapshot (same path as WIRES play). */
export function fiveDApplyLiveForceGraphStep(store: Store, _instanceId: string, drag?: Puzzle2dLiveForceGraphDragState): void {
  const fixture = project2d(store.read());
  if (fixture.nodes.length === 0) {
    return;
  }
  const laid = puzzle2dApplyLiveForceGraphLayoutTick(
    fixture,
    {
      forceGraph: {
        gravity: 0,
        idealEdgeLength: 64,
        iterations: FIVE_D_LIVE_FORCE_ITERS_PER_FRAME,
        repulsionStrength: 80,
      },
      mode: "force-graph",
      redrawHandlesAfter: false,
    } satisfies Puzzle2dRedrawLayoutOptions,
    drag,
  );
  store.applyFlatNodeCenters(new Map(laid.nodes.map((node) => [node.id, { x: node.x, y: node.y }])));
}

function fiveDLinkSessionFromStore(session: ConnectSession | null): Puzzle2dLinkSessionSnapshot | null {
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

function markers2dFromFixture(props: { readonly fixture: Puzzle2dFixtureV1; readonly lockedIds: ReadonlySet<string>; readonly selectedIds: ReadonlySet<string> }): ReactElement {
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
        <Edge
          {...(edge.edgeKind !== undefined ? { edgeKind: edge.edgeKind } : {})}
          id={edge.id}
          key={edge.id}
          selected={selectedIds.has(edge.id)}
          source={edge.source}
          target={edge.target}
        />
      ))}
    </>
  );
}

function puzzle5dModelStructureEpoch(model: V1): number {
  let hash = model.parts.length * 4099 + model.ties.length * 97;
  for (const part of model.parts) {
    hash = (hash * 31 + part.id.charCodeAt(0)) | 0;
  }
  for (const tie of model.ties) {
    hash = (hash * 31 + tie.id.charCodeAt(0)) | 0;
  }
  return hash;
}

const FiveD2d = reactHostPort.memo(function FiveD2d(props: FiveDProps) {
  const store = useStore();
  const snap = useSnapshot();
  const fixture2d = reactHostPort.useMemo(() => project2d(snap.model), [snap.model]);
  const declarativeSceneDescriptor = reactHostPort.useMemo(() => buildPuzzle2dSceneDescriptorFromFixture(fixture2d), [fixture2d]);
  const sceneAuthoringEpoch = reactHostPort.useMemo(() => puzzle5dModelStructureEpoch(snap.model), [snap.model]);
  const flatKindCatalogs = reactHostPort.useMemo(() => project2dKindCatalogs(snap.model.kindCatalogs), [snap.model.kindCatalogs]);
  const canvasSelection = reactHostPort.useMemo(
    () => ({ ids: [...snap.selection.partIds, ...snap.selection.anchorIds] }),
    [snap.selection.partIds, snap.selection.anchorIds],
  );
  const flatHover = reactHostPort.useMemo(() => fiveD2dHoverFromStore(snap.hoverFocus), [snap.hoverFocus]);
  const camera = store.get2dCamera(props.instanceId);
  const extra2d = props.puzzle2d ?? {};
  const {
    onSelect: onSelectHost,
    onConnect: onConnectHost,
    onIndirectConnect: onIndirectConnectHost,
    onProximityConnect: onProximityConnectHost,
    onDrag: onDragHost,
    onDragEnd: onDragEndHost,
    onBrushPlace: onBrushPlaceHost,
    onBrushCandidates: onBrushCandidatesHost,
    onDelete: onDeleteHost,
    activeTool: activeToolHost,
    brushFlushDistance: brushFlushDistanceHost,
    graphPortMode: puzzle2dGraphPortMode,
    ...rest2d
  } = extra2d;
  const activeTool = props.activeTool ?? activeToolHost ?? "select";
  const brushFlushDistance = props.brushFlushDistance ?? brushFlushDistanceHost;
  const graphPortMode = props.graphPortMode ?? puzzle2dGraphPortMode;
  const linkSession = fiveDLinkSessionFromStore(snap.connectSession);
  const liveForceGraph = props.liveForceGraph === true;
  const liveForceDragNodeIdsRef = reactHostPort.useRef<Set<string>>(new Set());
  const liveForceDragAnchorsRef = reactHostPort.useRef<Map<string, { x: number; y: number }>>(new Map());
  const storeRef = reactHostPort.useRef(store);
  const onSelectHostRef = reactHostPort.useRef(onSelectHost);
  const onConnectHostRef = reactHostPort.useRef(onConnectHost);
  const onIndirectConnectHostRef = reactHostPort.useRef(onIndirectConnectHost);
  const onProximityConnectHostRef = reactHostPort.useRef(onProximityConnectHost);
  const onDragHostRef = reactHostPort.useRef(onDragHost);
  const onDragEndHostRef = reactHostPort.useRef(onDragEndHost);
  const onBrushPlaceHostRef = reactHostPort.useRef(onBrushPlaceHost);
  const onBrushCandidatesHostRef = reactHostPort.useRef(onBrushCandidatesHost);
  const onDeleteHostRef = reactHostPort.useRef(onDeleteHost);
  storeRef.current = store;
  onSelectHostRef.current = onSelectHost;
  onConnectHostRef.current = onConnectHost;
  onIndirectConnectHostRef.current = onIndirectConnectHost;
  onProximityConnectHostRef.current = onProximityConnectHost;
  onDragHostRef.current = onDragHost;
  onDragEndHostRef.current = onDragEndHost;
  onBrushPlaceHostRef.current = onBrushPlaceHost;
  onBrushCandidatesHostRef.current = onBrushCandidatesHost;
  onDeleteHostRef.current = onDeleteHost;
  const onCamera = reactHostPort.useCallback((c: Puzzle2dCameraState) => {
    storeRef.current.set2dCamera(props.instanceId, c);
  }, [props.instanceId]);
  const onConnect = reactHostPort.useCallback((p: { source: string; target: string }) => {
    storeRef.current.applyTie(p.source, p.target);
    onConnectHostRef.current?.(p);
  }, []);
  const onIndirectConnect = reactHostPort.useCallback((p: { source: string; target: string }) => {
    storeRef.current.applyTie(p.source, p.target);
    onIndirectConnectHostRef.current?.(p);
  }, []);
  const onProximityConnect = reactHostPort.useCallback((p: { source: string; target: string }) => {
    storeRef.current.applyTie(p.source, p.target);
    onProximityConnectHostRef.current?.(p);
  }, []);
  const onDrag = reactHostPort.useCallback((p: { id: string; x: number; y: number }) => {
    liveForceDragNodeIdsRef.current.add(p.id);
    liveForceDragAnchorsRef.current.set(p.id, { x: p.x, y: p.y });
    if (liveForceGraph) {
      storeRef.current.applyNodeMove(p.id, p.x, p.y);
    }
    onDragHostRef.current?.(p);
  }, [liveForceGraph]);
  const onDragEnd = reactHostPort.useCallback((p: { moves: Array<{ id: string; x: number; y: number }> }) => {
    liveForceDragNodeIdsRef.current.clear();
    liveForceDragAnchorsRef.current.clear();
    storeRef.current.applyNodeMoves(p.moves);
    onDragEndHostRef.current?.(p);
  }, []);
  const onSelect = reactHostPort.useCallback((s: { ids: readonly string[] }) => {
    storeRef.current.setSelection({ partIds: s.ids, anchorIds: [] });
    onSelectHostRef.current?.(s);
  }, []);
  const onDelete = reactHostPort.useCallback((payload: Puzzle2dStructureDeletePayload) => {
    if (payload.kind === "wire") {
      return;
    }
    storeRef.current.applyStructuralDelete2d(payload);
    onDeleteHostRef.current?.(payload);
  }, []);
  const onCanvasHover = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => {
    storeRef.current.setHoverFocusFrom2d(project2d(storeRef.current.read()), payload);
  }, []);
  const onLinkCompatibleNodes = reactHostPort.useCallback((p: { source: string | null; nodeIds: readonly string[] }) => {
    if (!p.source) {
      storeRef.current.setConnectSession(null);
      return;
    }
    const prev = storeRef.current.getSnapshot().connectSession;
    storeRef.current.setConnectSession({
      origin: "2d",
      sourceAnchor: p.source,
      endX: prev?.endX ?? 0,
      endY: prev?.endY ?? 0,
      end3d: prev?.end3d ?? [0, 0, 0],
      compatiblePartIds: [...p.nodeIds],
      ringPartId: prev?.ringPartId ?? null,
      ringAnchorIds: prev?.ringAnchorIds ?? [],
    });
  }, []);
  const onLinkTargetRing = reactHostPort.useCallback((p: { source: string | null; nodeId: string | null; handleIds: readonly string[] }) => {
    const prev = storeRef.current.getSnapshot().connectSession;
    if (!p.source) {
      storeRef.current.setConnectSession(null);
      return;
    }
    storeRef.current.setConnectSession({
      origin: prev?.origin ?? "2d",
      sourceAnchor: p.source,
      endX: prev?.endX ?? 0,
      endY: prev?.endY ?? 0,
      end3d: prev?.end3d ?? [0, 0, 0],
      compatiblePartIds: prev?.compatiblePartIds ?? [],
      ringPartId: p.nodeId,
      ringAnchorIds: [...p.handleIds],
    });
  }, []);
  const onBrushPlace = reactHostPort.useCallback((payload: Puzzle2dBrushPlacePayload) => {
    onBrushPlaceHostRef.current?.(payload);
  }, []);
  const onBrushCandidates = reactHostPort.useCallback((payload: Parameters<NonNullable<Puzzle2dCanvasProps["onBrushCandidates"]>>[0]) => {
    onBrushCandidatesHostRef.current?.(payload);
  }, []);

  reactHostPort.useEffect(() => {
    if (!liveForceGraph || fixture2d.nodes.length === 0) return;
    let raf = 0;
    const step = () => {
      const lockedNodeIds = liveForceDragNodeIdsRef.current;
      const drag: Puzzle2dLiveForceGraphDragState | undefined =
        lockedNodeIds.size > 0 ? { dragAnchors: liveForceDragAnchorsRef.current, lockedNodeIds: [...lockedNodeIds] } : undefined;
      fiveDApplyLiveForceGraphStep(storeRef.current, props.instanceId, drag);
      raf = requestAnimationFrame(step);
    };
    raf = requestAnimationFrame(step);
    return () => cancelAnimationFrame(raf);
  }, [fixture2d.nodes.length, liveForceGraph, props.instanceId]);

  return (
    <div className={FIVE_D_ROOT_CLASS} data-five-d-indirect-active={snap.connectSession ? "true" : "false"} data-five-d-mode="2d" data-five-d-instance={props.instanceId}>
      <Puzzle2dCanvas
        camera={rest2d.camera ?? camera}
        className={["min-h-0 flex-1", props.className, rest2d.className].filter(Boolean).join(" ") || undefined}
        {...FIVE_D_FLAT_LOD_DEFAULTS}
        {...(graphPortMode !== undefined ? { graphPortMode } : {})}
        declarativeSceneDescriptor={declarativeSceneDescriptor}
        sceneAuthoringEpoch={sceneAuthoringEpoch}
        selection={canvasSelection}
        kindCatalogs={flatKindCatalogs}
        kindCompatibility={snap.model.kindCompatibility}
        linkSession={linkSession}
        onCamera={onCamera}
        onConnect={onConnect}
        onDrag={onDrag}
        onDragEnd={onDragEnd}
        onIndirectConnect={onIndirectConnect}
        onLinkCompatibleNodes={onLinkCompatibleNodes}
        onLinkTargetRing={onLinkTargetRing}
        onProximityConnect={onProximityConnect}
        onSelect={onSelect}
        onDelete={onDelete}
        activeTool={activeTool}
        {...(brushFlushDistance !== undefined ? { brushFlushDistance } : {})}
        {...(onBrushPlaceHost ? { onBrushPlace } : {})}
        {...(onBrushCandidatesHost ? { onBrushCandidates } : {})}
        {...rest2d}
        hoveredId={flatHover.hoveredId}
        kindHover={flatHover.kindHover}
        onHover={onCanvasHover}
      />
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
    onBrushPlace: onBrushPlaceHost,
    onFillMeshesReady: onFillMeshesReadyHost,
    brushActive: brushActiveHost,
    fillActive: fillActiveHost,
    brushPlacementOverlapBudget: brushOverlapHost,
    ...rest3d
  } = extra3d;
  const activeTool = props.activeTool ?? "select";
  const brushActive = brushActiveHost ?? activeTool === "brush";
  const fillActive = fillActiveHost ?? activeTool === "fill";
  const brushPlacementOverlapBudget = brushOverlapHost ?? props.brushOverlapBudget;
  const camera = store.get3dCamera(props.instanceId);
  const canvasSelection = reactHostPort.useMemo(() => fiveD3dSelectionFromStore(snap.selection), [snap.selection.partIds, snap.selection.anchorIds]);
  const volumeHover = reactHostPort.useMemo(() => fiveD3dHoverFromStore(snap.hoverFocus), [snap.hoverFocus]);
  const attractionSession = fiveDAttractionSessionFromStore(snap.connectSession);
  const onRelocate = usePuzzle3dPartRelocate();
  const onConnect = usePuzzle3dPartConnect();
  const storeRef = reactHostPort.useRef(store);
  storeRef.current = store;
  const onCamera = reactHostPort.useCallback((c: Puzzle3dFixtureV1["camera"]) => {
    storeRef.current.set3dCamera(props.instanceId, c);
  }, [props.instanceId]);
  const onCanvasHover = reactHostPort.useCallback((payload: Puzzle3dHoverPayload) => {
    storeRef.current.setHoverFocusFrom3d(payload);
  }, []);
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
      gumballConfig={props.gumballConfig ?? PUZZLE_3D_GUMBALL_CONFIG}
      onCamera={onCamera}
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
      brushActive={brushActive}
      fillActive={fillActive}
      {...(brushPlacementOverlapBudget !== undefined ? { brushPlacementOverlapBudget } : {})}
      {...(onBrushPlaceHost ? { onBrushPlace: onBrushPlaceHost } : {})}
      {...(onFillMeshesReadyHost ? { onFillMeshesReady: onFillMeshesReadyHost } : {})}
      {...rest3d}
      hoverTarget={volumeHover.hoverTarget}
      kindHover={volumeHover.kindHover}
      onHover={onCanvasHover}
      sceneFixture={fixture3d}
      selection={canvasSelection}
    >
      <Puzzle3dParts relocate={puzzle3dObjectGumballConfig(props.gumballConfig)} />
      <Puzzle3dTies />
    </Puzzle3dCanvas>
  );
});

const FiveD3d = reactHostPort.memo(function FiveD3d(props: FiveDProps) {
  const snap = useSnapshot();
  const fixture3d = reactHostPort.useMemo(() => project3d(snap.model), [snap.model]);
  const fixtureRevision = snap.model.parts.length + snap.model.ties.length * 4099;
  return (
    <div className={FIVE_D_ROOT_CLASS} data-five-d-indirect-active={snap.connectSession ? "true" : "false"} data-five-d-mode="3d" data-five-d-instance={props.instanceId}>
      <reactHostPort.Suspense fallback={<div className="flex min-h-0 flex-1 items-center justify-center p-4 text-sm text-muted-foreground">Loading meshes…</div>}>
        <Puzzle3dPartStateProvider
          fixture={fixture3d}
          fixtureRevision={fixtureRevision}
          onConnect={props.puzzle3d?.onConnect}
          onRelocate={props.puzzle3d?.onRelocate}
        >
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
    color: row.color ?? "var(--color-muted-foreground)",
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
  "gridFactor" | "gridSnapEnabled"
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

//#region 🔖Puzzle2dLayout
/** @emoji 🔗 Default separator for 2d handle ids (`piece::connector`). */
export const FLAT_HANDLE_COMPOUND_SEPARATOR = "::";

/** @emoji 🔗 Builds a compound 2d handle id from two parts. */
export function flatHandleCompoundId(left: string, right: string, separator: string = FLAT_HANDLE_COMPOUND_SEPARATOR): string {
  return `${left}${separator}${right}`;
}

/** @emoji ┬¡ãÆ├Â├¼ Parses a compound puzzle 2d handle id into left/right parts. */
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

/** @emoji ┬¡ãÆ├▓┬®┬┤┬®├à Maps diagram force sliders to {@link layoutPuzzle2dFixtureForceGraph} options. */
export function puzzle2dForceGraphOptions(weights: DiagramForceWeights): Puzzle2dForceGraphLayoutOptions {
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

/** @emoji ┬¡ãÆ├┤├Ç Centers the puzzle 2d camera on the average of node centers. */
export function camera2dFromPartCenters(centers: readonly { x: number; y: number }[]): Puzzle2dCameraState {
  if (centers.length === 0) return { x: 0, y: 0, zoom: 1 };
  const avgX = centers.reduce((sum, point) => sum + point.x, 0) / centers.length;
  const avgY = centers.reduce((sum, point) => sum + point.y, 0) / centers.length;
  return { x: -avgX, y: -avgY, zoom: 1 };
}

/** @emoji ┬¡ãÆ├┤├¼ Writes WASM layout node centers back into top-left layout positions. */
export function flatApplyFixtureCentersToTopLeft<T extends { readonly id: string; readonly position: { x: number; y: number } }>(
  items: readonly T[],
  fixture: Puzzle2dFixtureV1,
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
//#endregion 🔖Puzzle2dLayout

/** @emoji 🎛 Default 3d chrome for puzzle 5d surfaces (alias of {@link FIVE_D_3D_CHROME_DEFAULTS}). */
export function chrome3dDefaults(): typeof FIVE_D_3D_CHROME_DEFAULTS {
  return FIVE_D_3D_CHROME_DEFAULTS;
}

//#region 🔖Puzzle2dMarkers
export interface FlatWireRecord {
  readonly id: string;
  readonly source: string;
  readonly target?: string;
  readonly wireKind?: string;
  readonly endX?: number;
  readonly endY?: number;
  readonly hidden?: boolean;
}

/** @emoji 🧩 Builds puzzle2d host markers from a puzzle 2d fixture (same static shape walk as puzzle 2d play). */
export function flatMarkersFromFixture(props: {
  readonly fixture: Puzzle2dFixtureV1;
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
//#endregion 🔖Puzzle2dMarkers

export { DEFAULT_PUZZLE_2D_GRID_FACTOR, getPuzzle2dLodScale, blockedVortexFullIdsFromAttractions, parsePuzzle2dFixtureV1, parseFixtureV1 };

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
      const fixture2d: Puzzle2dFixtureV1 = {
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

    it("preserves edge kinds as tie kinds for wires-style fixtures", () => {
      const fixture2d: Puzzle2dFixtureV1 = {
        schema: "puzzle.2d.fixture/v1",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [
          { id: "a", shape: "circle", x: 0, y: 0, radius: 10, handles: [] },
          { id: "b", shape: "circle", x: 100, y: 0, radius: 10, handles: [] },
        ],
        edges: [{ id: "e1", source: "a", target: "b", edgeKind: "wires.owns" }],
      };
      const fixture3d: Puzzle3dFixtureV1 = {
        schema: "puzzle.3d.fixture/v1",
        domain: "architecture",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        objects: [],
        attractions: [],
      };
      const model = compose5d(fixture2d, fixture3d);
      expect(model.ties[0]?.tieKind).toBe("wires.owns");
      expect(project2d(model).edges[0]?.edgeKind).toBe("wires.owns");
    });
  });

  describe("Store applyTie", () => {
    it("ignores duplicate source-target pairs", () => {
      const store = createStore({
        schema: "puzzle.5d/v1",
        domain: "architecture",
        camera2d: { x: 0, y: 0, zoom: 1 },
        camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        parts: [],
        ties: [{ id: "t1", source: "a", target: "b" }],
      });
      store.applyTie("a", "b");
      expect(store.getSnapshot().model.ties).toHaveLength(1);
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
  describe("fiveD3dSelectionFromStore", () => {
    it("maps unified part and anchor ids to puzzle 3d selection", () => {
      expect(fiveD3dSelectionFromStore({ partIds: ["tower-a", "tower-b"], anchorIds: ["tower-a:port"] })).toEqual({
        objectIds: ["tower-a", "tower-b"],
        vortexIds: ["tower-a:port"],
        attractionIds: [],
      });
    });
  });
  describe("paired hover focus", () => {
    const fixture2d: Puzzle2dFixtureV1 = {
      schema: "puzzle.2d.fixture/v1",
      camera: { x: 0, y: 0, zoom: 1 },
      nodes: [
        { id: "p1", shape: "circle", x: 0, y: 0, radius: 10, handles: [{ id: "p1:h", angle: 0, handleKind: "port" }] },
      ],
      edges: [{ id: "t1", source: "p1:h", target: "p1" }],
    };

    it("maps flat part hover to volume object hover", () => {
      const focus = hoverFocusFrom2dPayload(fixture2d, { id: "p1", kind: null });
      expect(fiveD3dHoverFromStore(focus)).toEqual({ hoverTarget: { kind: "object", id: "p1" }, kindHover: null });
      expect(fiveD2dHoverFromStore(focus)).toEqual({ hoveredId: "p1", kindHover: null });
    });

    it("maps flat handle hover to volume vortex hover", () => {
      const focus = hoverFocusFrom2dPayload(fixture2d, { id: "p1:h", kind: null });
      expect(fiveD3dHoverFromStore(focus)).toEqual({ hoverTarget: { kind: "vortex", fullId: "p1:h" }, kindHover: null });
    });

    it("maps volume object hover back to flat node hover", () => {
      const focus = hoverFocusFrom3dPayload({ hoverTarget: { kind: "object", id: "p1" }, kindHover: null });
      expect(fiveD2dHoverFromStore(focus)).toEqual({ hoveredId: "p1", kindHover: null });
    });

    it("syncs transitive kind hover across surfaces", () => {
      const focus = hoverFocusFrom3dPayload({ hoverTarget: null, kindHover: { domain: "object", kindId: "tower" } });
      expect(fiveD2dHoverFromStore(focus)).toEqual({ hoveredId: null, kindHover: { domain: "node", kindId: "tower" } });
      expect(fiveD3dHoverFromStore(focus)).toEqual({ hoverTarget: null, kindHover: { domain: "object", kindId: "tower" } });
    });

    it("stores hover focus in the shared store", () => {
      const store = createStore({
        schema: "puzzle.5d/v1",
        domain: "architecture",
        camera2d: { x: 0, y: 0, zoom: 1 },
        camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        parts: [],
        ties: [],
      });
      store.setHoverFocusFrom2d(fixture2d, { id: "p1", kind: null });
      expect(store.getSnapshot().hoverFocus).toEqual({ instance: { kind: "part", id: "p1" }, kindHover: null });
      store.setHoverFocusFrom3d({ hoverTarget: null, kindHover: null });
      expect(store.getSnapshot().hoverFocus).toEqual({ instance: null, kindHover: null });
    });
  });
  describe("nodeCenterFromTopLeft", () => {
    it("offsets by half frame", () => {
      expect(nodeCenterFromTopLeft({ x: 10, y: 20 }, { width: 40, height: 60 })).toEqual({ x: 30, y: 50 });
    });
  });
  describe("flatApplyFixtureCentersToTopLeft", () => {
    it("converts centers to top-left using frame size", () => {
      const fixture: Puzzle2dFixtureV1 = {
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

  describe("structural delete", () => {
    it("removes a unified part from both projections when a flat node is deleted", () => {
      const model = compose5d(
        {
          schema: "puzzle.2d.fixture/v1",
          camera: { x: 0, y: 0, zoom: 1 },
          nodes: [
            { id: "p1", shape: "circle", x: 0, y: 0, radius: 10, handles: [{ id: "p1:h", angle: 0, handleKind: "port" }] },
            { id: "p2", shape: "circle", x: 10, y: 0, radius: 10, handles: [] },
          ],
          edges: [{ id: "e1", source: "p1:h", target: "p2" }],
        },
        {
          schema: "puzzle.3d.fixture/v1",
          domain: "architecture",
          camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
          objects: [
            { id: "p1", meshUrl: "a.glb", origin: [0, 0, 0], vortices: [{ id: "p1:h", position: [0, 0, 0] }] },
            { id: "p2", meshUrl: "b.glb", origin: [1, 0, 0], vortices: [] },
          ],
          attractions: [{ id: "e1", attracting: "p1:p1:h", attracted: "p2" }],
        },
      );
      const store = createStore(model);
      expect(store.applyStructuralDelete2d({ kind: "node", id: "p1" })).toBe(true);
      expect(store.read().parts.map((part) => part.id)).toEqual(["p2"]);
      expect(project3d(store.read()).objects.map((object) => object.id)).toEqual(["p2"]);
      expect(project2d(store.read()).nodes.map((node) => node.id)).toEqual(["p2"]);
      expect(store.read().ties).toHaveLength(0);
    });
  });

  describe("applyBrushPlacementToModel", () => {
    const brushHostModel = (): V1 => ({
      schema: "puzzle.5d/v1",
      domain: "architecture",
      camera2d: { x: 0, y: 0, zoom: 1 },
      camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
      parts: [
        {
          id: "host",
          partKind: "Host",
          puzzle2d: { x: 0, y: 0, shape: "rectangle", width: 40, height: 40 },
          puzzle3d: { origin: [0, 0, 0], meshUrl: "/mesh/host.glb", orientation: [0, 0, 0, 1] },
          anchors: [
            {
              id: "port",
              anchorKind: "port",
              puzzle2d: { angle: 0, anchorKind: "port" },
              puzzle3d: { position: [1, 0, 0], direction: [-1, 0, 0] },
            },
          ],
        },
        {
          id: "peer",
          partKind: "Capsule",
          puzzle2d: { x: 100, y: 0, shape: "rectangle", width: 40, height: 40 },
          puzzle3d: { origin: [10, 0, 0], meshUrl: "/mesh/capsule.glb", orientation: [0, 0, 0, 1] },
          anchors: [
            {
              id: "mate",
              anchorKind: "port",
              puzzle2d: { angle: Math.PI, anchorKind: "port" },
              puzzle3d: { position: [-1, 0, 0], direction: [1, 0, 0] },
            },
          ],
        },
      ],
      ties: [],
    });

    it("appends a unified part with both aspects from a volume brush payload", () => {
      const model = brushHostModel();
      const result = applyBrushPlacementToModel(
        model,
        puzzle5dBrushPlacementFromVolume({
          targetVortexFullId: "host:port",
          objectKindId: "Capsule",
          sourceVortexIndex: 0,
          origin: [2, 0, 0],
          orientation: [0, 0, 0, 1],
        }),
      );
      expect(result.kind).toBe("placed");
      if (result.kind !== "placed") return;
      const placed = result.model.parts.find((part) => part.id === result.partId);
      expect(placed?.puzzle2d).toBeTruthy();
      expect(placed?.puzzle3d).toBeTruthy();
      expect(result.model.ties.some((tie) => tie.source.endsWith(":mate") || tie.source.includes("Capsule"))).toBe(true);
    });

    it("appends a unified part with both aspects from a flat brush payload", () => {
      const model = brushHostModel();
      const result = applyBrushPlacementToModel(
        model,
        puzzle5dBrushPlacementFromFlat({
          nodeKind: "Capsule",
          shape: "rectangle",
          sourceHandleId: "host:port",
          targetHandleIndex: 0,
          x: 80,
          y: 0,
          handles: [{ angle: Math.PI, handleKind: "port" }],
        }),
      );
      expect(result.kind).toBe("placed");
      if (result.kind !== "placed") return;
      const placed = result.model.parts.find((part) => part.id === result.partId);
      expect(placed?.puzzle2d?.x).toBe(80);
      expect(placed?.puzzle3d?.meshUrl).toBe("/mesh/capsule.glb");
    });
  });

  describe("Store fill session", () => {
    it("applies fill prefix with unified parts", () => {
      const store = createStore({
        schema: "puzzle.5d/v1",
        domain: "architecture",
        camera2d: { x: 0, y: 0, zoom: 1 },
        camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        parts: [
          {
            id: "host",
            partKind: "Host",
            puzzle2d: { x: 0, y: 0, shape: "rectangle", width: 40, height: 40 },
            puzzle3d: { origin: [0, 0, 0], meshUrl: "/mesh/host.glb", orientation: [0, 0, 0, 1] },
            anchors: [
              {
                id: "port",
                anchorKind: "port",
                puzzle2d: { angle: 0, anchorKind: "port" },
                puzzle3d: { position: [1, 0, 0], direction: [-1, 0, 0] },
              },
            ],
          },
          {
            id: "peer",
            partKind: "Capsule",
            puzzle2d: { x: 100, y: 0, shape: "rectangle", width: 40, height: 40 },
            puzzle3d: { origin: [10, 0, 0], meshUrl: "/mesh/capsule.glb", orientation: [0, 0, 0, 1] },
            anchors: [
              {
                id: "mate",
                anchorKind: "port",
                puzzle2d: { angle: Math.PI, anchorKind: "port" },
                puzzle3d: { position: [-1, 0, 0], direction: [1, 0, 0] },
              },
            ],
          },
        ],
        ties: [],
      });
      const sequence = [
        puzzle5dBrushPlacementFromVolume({
          targetVortexFullId: "host:port",
          objectKindId: "Capsule",
          sourceVortexIndex: 0,
          origin: [2, 0, 0],
          orientation: [0, 0, 0, 1],
          objectId: "fill-1",
        }),
      ];
      store.prepareFillSession(sequence);
      store.applyFillCount(1);
      expect(store.read().parts).toHaveLength(3);
      const added = store.read().parts.find((part) => part.id === "fill-1");
      expect(added?.puzzle2d).toBeTruthy();
      expect(added?.puzzle3d).toBeTruthy();
    });
  });

  describe("fiveDApplyLiveForceGraphStep", () => {
    it("updates flat node centers for linked normal-graph nodes", () => {
      const store = createStore(
        compose5d(
          {
            schema: "puzzle.2d.fixture/v1",
            camera: { x: 0, y: 0, zoom: 1 },
            nodes: [
              { id: "a", shape: "circle", x: 0, y: 0, radius: 20, handles: [] },
              { id: "b", shape: "circle", x: 4, y: 0, radius: 20, handles: [] },
            ],
            edges: [{ id: "e1", source: "a", target: "b" }],
          },
          {
            schema: "puzzle.3d.fixture/v1",
            domain: "architecture",
            camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
            objects: [],
            attractions: [],
          },
        ),
      );
      fiveDApplyLiveForceGraphStep(store, "kit:kit:wires");
      const a = store.read().parts.find((part) => part.id === "a")?.puzzle2d;
      const b = store.read().parts.find((part) => part.id === "b")?.puzzle2d;
      expect(a?.x).toBeDefined();
      expect(b?.x).toBeDefined();
      expect(Math.abs((a?.x ?? 0) - (b?.x ?? 0))).toBeGreaterThan(8);
    });
  });
}
