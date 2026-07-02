// #region 🧲Header
/** @emoji 🔗 `@semio-tech/puzzle-5d-react` — paired 2d + 3d puzzle 5d surfaces and play harness (monolith). */
// #endregion 🧲Header

// #region 🔌Adapters
import { reactHostPort, type ContextMenuItem, type GumballConfig } from "@semio-tech/ui-react";
import {
  mergeManifestCatalogBundles,
  puzzle2d_defaultManifestCatalogBundle,
  puzzle3d_defaultManifestCatalogBundle,
} from "@semio-tech/graph-manifest";
import { PUZZLE_3D_GUMBALL_CONFIG } from "../../3d/react/index.tsx";
import type { ReactElement } from "react";

/** @emoji 🔗 Unified puzzle 5d model with 2d WASM + 3d R3F projections and a shared {@link Store}. */

import {
  Puzzle2dCanvas,
  DEFAULT_PUZZLE_2D_GRID_FACTOR,
  getPuzzle2dLodScale,
  Edge, Handle, Node, Wire,
  BUILTIN_PORT_HANDLE_KIND,
  fixtureMetaKindCatalogBundle,
  parsePuzzle2dFixture,
  type CameraState as Puzzle2dCameraState,
  type Puzzle2dCanvasProps,
  type Puzzle2dFixture,
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
  DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
  DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
  puzzle2dFixtureHandlesFromNodeKind,
  buildPaletteNodeDragFixture,
  mergePaletteNodeFromDrop,
  abortPuzzle2dFixturePaletteDrag,
  puzzle2dFixturePaletteDropCommittedRef,
  type Puzzle2dFixtureDropDetail,
  type Puzzle2dFixtureNode,
  type Puzzle2dKindHover,
  type Puzzle2dHoverPayload,
  puzzle2dSubscribeBrushSession,
  puzzle2dGetBrushSessionSnapshot,
  puzzle2dApplyBrushSuggestionsCandidateIndex,
  puzzle2dBrushPlacePayloadFromSessionSnapshot,
  puzzle2dBrushSlotCenterFromNode,
  puzzle2dSyncBrushSessionToAllAuthoringPeers,
  puzzle2dSyncFixtureDescriptorToAllAuthoringPeers,
  puzzle2dPushAuthoritativeSceneToAllAuthoringPeers,
  puzzle2dFinalizeBrushSuggestionsPlacement,
  puzzle2dGuardBrushPlacementStructuralDeletes,
  usePuzzle2dRenderer,
  type Puzzle2dBrushSessionSnapshot,
  type Puzzle2dRenderer,
} from "@semio-tech/puzzle-2d-react";
import {
  DEFAULT_MESH_STYLE as PUZZLE_3D_DEFAULT_MESH_STYLE,
  ObjectStateProvider as Puzzle3dPartStateProvider,
  PlayCanvas as Puzzle3dPlayCanvas,
  PUZZLE_3D_MESH_PAINT,
  blockedVortexFullIdsFromAttractions,
  cancelPuzzle3dFixturePalettePointerDrag,
  meshStyleColors,
  parseFixture,
  type AttractionKind as Puzzle3dFastenerKind,
  type AttractionSessionSnapshot,
  type CableKind as Puzzle3dRopeKind,
  type DomainKind,
  type Fixture as Puzzle3dFixture,
  type KindCatalogBundle as Puzzle3dKindCatalogBundle,
  type KindCompatEntry as Puzzle3dKindCompatEntry,
  type ObjectKind as Puzzle3dPartKind,
  type RelocateMode as Puzzle3dRelocateMode,
  type SelectionSnapshot as Puzzle3dSelectionSnapshot,
  type VortexKind as Puzzle3dGripKind,
  type CanvasProps as Puzzle3dCanvasProps,
  type LodMeshEntry as Puzzle3dLodMeshEntry,
  type BrushPlacePayload,
  type Puzzle3dBrushKindWeights,
  type FixtureObject as Puzzle3dFixtureObject,
  type HoverTarget,
  type MeshStyleColors,
  type MeshStyleKind,
  type Puzzle3dHoverPayload,
  type Puzzle3dKindHover,
  type WorldReferenceProps as Puzzle3dWorldReferenceProps,
  type WorldVolumeProps as Puzzle3dWorldVolumeProps,
  buildBrushFillSequence,
  brushCompatibleCandidates,
  brushPlacementUsesHostOrientation,
  computeBrushPlacementPose,
  resolveObjectKindMeshUrl,
  vortexWorldCadFromObject,
  PUZZLE_3D_FILL_COUNT_MAX,
  puzzle3dBrushPairedSyncRef,
  puzzle3dBrushUiStore,
  subscribePuzzle3dBrushEngagementSource,
  getPuzzle3dBrushEngagementEpoch,
  type BrushPreviewState,
  type VortexBindingMeta,
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

/** @emoji 🎨 5d uses the 3d package as the only source of truth for volume mesh colors. */
export const PUZZLE_5D_3D_MESH_PAINT = PUZZLE_3D_MESH_PAINT;

/** @emoji 🎨 Default 5d volume mesh style delegated to puzzle 3d. */
export const PUZZLE_5D_3D_DEFAULT_MESH_STYLE = PUZZLE_3D_DEFAULT_MESH_STYLE;

/** @emoji 🎨 Resolves 5d volume mesh colors through puzzle 3d tokens. */
export const puzzle5d3dMeshStyleColors: (style: MeshStyleKind) => MeshStyleColors | null = meshStyleColors;

/** @emoji 🎨 Forwards only explicit 5d volume mesh styles so unset stays identical to puzzle 3d fixtures. */
export function puzzle5d3dMeshStyleProps(style: MeshStyleKind | undefined): { readonly style?: MeshStyleKind } {
  return style === undefined ? {} : { style };
}

export type Puzzle5d3dMeshStyleKind = MeshStyleKind;
//#endregion 🔖PairedPolicy

//#region 🔖Model
export type PresentationMode = "2d" | "3d";

export interface Grip2dAspect {
  readonly angle: number;
  readonly gripKind: string;
  readonly color?: string;
  readonly iconKind?: string;
  readonly radius?: number;
  /** @emoji 📐 Optional port perimeter parameter (0–1); defaults from {@link angle} / 360 when absent. */
  readonly t?: number;
}

export interface Grip3dAspect {
  readonly position: readonly [number, number, number];
  readonly direction?: readonly [number, number, number];
  readonly radius?: number;
  readonly label?: string;
  readonly handleMeshUrl?: string;
  readonly vortexMeshUrl?: string;
  readonly vortexMeshByLod?: readonly Puzzle3dLodMeshEntry[];
  readonly hidden?: boolean;
  readonly locked?: boolean;
}

export interface Grip {
  readonly id: string;
  readonly gripKind: string;
  readonly "2d"?: Grip2dAspect;
  readonly "3d"?: Grip3dAspect;
}

export interface Part2dAspect {
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

export interface Part3dAspect {
  readonly origin: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: number | readonly [number, number, number];
  readonly meshUrl: string;
  readonly meshByLod?: readonly Puzzle3dLodMeshEntry[];
  readonly style?: MeshStyleKind;
  readonly label?: string;
  readonly wormhole?: boolean;
  readonly hidden?: boolean;
  readonly locked?: boolean;
}

export interface Part {
  readonly id: string;
  readonly partKind?: string;
  readonly "2d"?: Part2dAspect;
  readonly "3d"?: Part3dAspect;
  readonly grips: readonly Grip[];
}

export interface Fastener extends PuzzleConnectionTransformParams {
  readonly id: string;
  readonly source: string;
  readonly target: string;
  readonly fastenerKind?: string;
}

/** @emoji 🔗 Compose connection transform params shared with puzzle 2d edges and puzzle 3d attractions. */
export interface PuzzleConnectionTransformParams {
  readonly gap?: number;
  readonly shift?: number;
  readonly rise?: number;
  readonly rotation?: number;
  readonly turn?: number;
  readonly tilt?: number;
  readonly u?: number;
  readonly v?: number;
}

const PUZZLE_CONNECTION_TRANSFORM_PARAM_KEYS = ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"] as const;

/** @emoji 🔗 Reads optional connection transform params from fixture JSON. */
export function parsePuzzleConnectionTransformParams(record: Record<string, unknown>): PuzzleConnectionTransformParams {
  const out: Record<string, number> = {};
  for (const key of PUZZLE_CONNECTION_TRANSFORM_PARAM_KEYS) {
    const value = record[key];
    if (typeof value === "number" && Number.isFinite(value)) {
      out[key] = value;
    }
  }
  return out;
}

/** @emoji 🔗 Spreads defined connection transform params onto a fixture record. */
export function spreadPuzzleConnectionTransformParams(params: PuzzleConnectionTransformParams): PuzzleConnectionTransformParams {
  const out: Record<string, number> = {};
  for (const key of PUZZLE_CONNECTION_TRANSFORM_PARAM_KEYS) {
    const value = params[key];
    if (typeof value === "number" && Number.isFinite(value)) {
      out[key] = value;
    }
  }
  return out;
}

function connectionTransformParam(params: PuzzleConnectionTransformParams, key: (typeof PUZZLE_CONNECTION_TRANSFORM_PARAM_KEYS)[number]): number {
  const value = params[key];
  return typeof value === "number" && Number.isFinite(value) ? value : 0;
}

/** @emoji 🔗 In-progress **indirect** connect only (never proximity); synced across 2d {@link Puzzle2dLinkSessionSnapshot} and 3d {@link AttractionSessionSnapshot}. */
export interface ConnectSession {
  readonly origin: PresentationMode;
  readonly sourceGrip: string;
  readonly endX: number;
  readonly endY: number;
  readonly end3d: readonly [number, number, number];
  readonly compatiblePartIds: readonly string[];
  readonly ringPartId: string | null;
  readonly ringGripIds: readonly string[];
}

export interface SelectionSnapshot {
  readonly partIds: readonly string[];
  readonly gripIds: readonly string[];
}

/** @emoji 🎯 Maps unified store selection to puzzle 3d controlled canvas selection. */
export function fiveD3dSelectionFromStore(selection: SelectionSnapshot): Puzzle3dSelectionSnapshot {
  return {
    objectIds: [...selection.partIds],
    vortexIds: [...selection.gripIds],
    attractionIds: [],
  };
}

//#region 🔖Hover
export type Puzzle5dKindHoverDomain = "part" | "grip" | "fastener";

/** @emoji 🖱️ Transitive catalog-kind hover shared across paired 2d and 3d surfaces. */
export interface Puzzle5dKindHover {
  readonly domain: Puzzle5dKindHoverDomain;
  readonly kindId: string;
}

export type Puzzle5dHoverInstance =
  | { readonly kind: "part"; readonly id: string }
  | { readonly kind: "grip"; readonly fullId: string }
  | { readonly kind: "fastener"; readonly id: string };

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
  if (a.kind === "grip") {
    return b.kind === "grip" && a.fullId === b.fullId;
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
      return { domain: "grip", kindId: kind.kindId };
    case "edge":
      return { domain: "fastener", kindId: kind.kindId };
    default:
      return null;
  }
}

function puzzle5dKindHoverTo2d(kind: Puzzle5dKindHover): Puzzle2dKindHover {
  switch (kind.domain) {
    case "part":
      return { domain: "node", kindId: kind.kindId };
    case "grip":
      return { domain: "handle", kindId: kind.kindId };
    case "fastener":
      return { domain: "edge", kindId: kind.kindId };
  }
}

function puzzle5dKindHoverFrom3d(kind: Puzzle3dKindHover): Puzzle5dKindHover {
  switch (kind.domain) {
    case "object":
      return { domain: "part", kindId: kind.kindId };
    case "vortex":
      return { domain: "grip", kindId: kind.kindId };
    case "attraction":
      return { domain: "fastener", kindId: kind.kindId };
  }
}

function puzzle5dKindHoverTo3d(kind: Puzzle5dKindHover): Puzzle3dKindHover {
  switch (kind.domain) {
    case "part":
      return { domain: "object", kindId: kind.kindId };
    case "grip":
      return { domain: "vortex", kindId: kind.kindId };
    case "fastener":
      return { domain: "attraction", kindId: kind.kindId };
  }
}

/** @emoji 🖱️ Resolves a flat graph element id to a unified hover instance. */
export function puzzle5dHoverInstanceFrom2dGraphId(fixture2d: Puzzle2dFixture, graphId: string): Puzzle5dHoverInstance | null {
  if (fixture2d.nodes.some((node) => node.id === graphId)) {
    return { kind: "part", id: graphId };
  }
  if (fixture2d.edges.some((edge) => edge.id === graphId)) {
    return { kind: "fastener", id: graphId };
  }
  for (const node of fixture2d.nodes) {
    if (node.handles.some((handle) => handle.id === graphId)) {
      return { kind: "grip", fullId: graphId };
    }
  }
  return null;
}

function puzzle5dHoverInstanceFrom3dTarget(target: HoverTarget): Puzzle5dHoverInstance | null {
  switch (target.kind) {
    case "object":
      return { kind: "part", id: target.id };
    case "vortex":
      return { kind: "grip", fullId: target.fullId };
    case "attraction":
      return { kind: "fastener", id: target.id };
    default:
      return null;
  }
}

/** @emoji 🖱️ Maps flat canvas hover to unified store hover focus. */
export function hoverFocusFrom2dPayload(
  fixture2d: Puzzle2dFixture,
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
      case "grip":
        return { hoveredId: focus.instance.fullId, kindHover: null };
      case "fastener":
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
      case "grip":
        return { hoverTarget: { kind: "vortex", fullId: focus.instance.fullId }, kindHover: null };
      case "fastener":
        return { hoverTarget: { kind: "attraction", id: focus.instance.id }, kindHover: null };
    }
  }
  return {
    hoverTarget: null,
    kindHover: focus.kindHover ? puzzle5dKindHoverTo3d(focus.kindHover) : null,
  };
}

//#endregion 🔖Hover

export const PUZZLE_5D_SCHEMA = "puzzle.5d" as const;

export interface Model {
  readonly schema: typeof PUZZLE_5D_SCHEMA;
  readonly label?: string;
  readonly domain: DomainKind;
  readonly meta?: Record<string, unknown>;
  readonly kindCatalogs?: KindCatalogBundle;
  readonly kindCompatibility?: readonly KindCompatEntry[];
  readonly camera2d: Puzzle2dCameraState;
  readonly camera3d: Puzzle3dFixture["camera"];
  readonly parts: readonly Part[];
  readonly fasteners: readonly Fastener[];
  readonly references3d?: readonly Puzzle3dWorldReferenceProps[];
  readonly targetVolumes3d?: readonly Puzzle3dWorldVolumeProps[];
}

export const PUZZLE_5D_GRIP_ID_SEPARATOR = ":";

/** @emoji 🔗 Builds a full grip id `partId:gripId`. */
export function gripFullId(partId: string, gripId: string): string {
  return `${partId}${PUZZLE_5D_GRIP_ID_SEPARATOR}${gripId}`;
}

/** @emoji 🔍 Splits a full grip id into part and grip local ids. */
export function parseGripFullId(fullId: string): { partId: string; gripId: string } | null {
  const i = fullId.indexOf(PUZZLE_5D_GRIP_ID_SEPARATOR);
  if (i <= 0 || i >= fullId.length - 1) return null;
  return { partId: fullId.slice(0, i), gripId: fullId.slice(i + 1) };
}

/** @emoji ✅ Validates unified puzzle 5d JSON. */
export function parseModel(raw: unknown): Model | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== PUZZLE_5D_SCHEMA) return null;
  if (!Array.isArray(r.parts) || !Array.isArray(r.fasteners)) return null;
  const domain = typeof r.domain === "string" ? (r.domain as DomainKind) : "architecture";
  const flatCam = r.camera2d as Puzzle2dCameraState | undefined;
  const volumeCam = r.camera3d as Puzzle3dFixture["camera"] | undefined;
  if (!flatCam || !volumeCam) return null;
  return {
    schema: PUZZLE_5D_SCHEMA,
    domain,
    camera2d: flatCam,
    camera3d: volumeCam,
    parts: r.parts as Part[],
    fasteners: r.fasteners as Fastener[],
    ...(Array.isArray(r.references3d) ? { references3d: r.references3d as Puzzle3dWorldReferenceProps[] } : {}),
    ...(Array.isArray(r.targetVolumes3d) ? { targetVolumes3d: r.targetVolumes3d as Puzzle3dWorldVolumeProps[] } : {}),
    ...(typeof r.label === "string" ? { label: r.label } : {}),
    ...(r.meta && typeof r.meta === "object" ? { meta: r.meta as Record<string, unknown> } : {}),
    ...(r.kindCatalogs && typeof r.kindCatalogs === "object" ? { kindCatalogs: normalizeKindCatalogBundle(r.kindCatalogs) } : {}),
    ...(Array.isArray(r.kindCompatibility) ? { kindCompatibility: r.kindCompatibility as KindCompatEntry[] } : {}),
  };
}

/** @emoji 🔀 Builds {@link Model} by merging 2d and 3d fixtures (same part ids unite). */
export function compose5d(fixture2d: Puzzle2dFixture, fixture3d: Puzzle3dFixture): Model {
  const partsMap = new Map<string, Part>();
  for (const node of fixture2d.nodes) {
    const grips: Grip[] = node.handles.map((h) => {
      const parsed = parseGripFullId(h.id);
      const localId = parsed?.gripId ?? h.id;
      return {
        id: localId,
        gripKind: h.handleKind,
        "2d": {
          angle: h.angle,
          gripKind: h.handleKind,
          ...(h.color !== undefined ? { color: h.color } : {}),
          ...(h.iconKind !== undefined ? { iconKind: h.iconKind } : {}),
          ...(h.radius !== undefined ? { radius: h.radius } : {}),
          ...(h.t !== undefined ? { t: h.t } : {}),
        },
      };
    });
    const flatAspect: Part2dAspect =
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
      "2d": flatAspect,
      grips,
    });
  }
  for (const obj of fixture3d.objects) {
    const volumeAspect: Part3dAspect = {
      origin: obj.origin,
      meshUrl: obj.meshUrl,
      ...(obj.orientation !== undefined ? { orientation: obj.orientation } : {}),
      ...(obj.scale !== undefined ? { scale: obj.scale } : {}),
      ...(obj.meshByLod !== undefined ? { meshByLod: obj.meshByLod } : {}),
      ...puzzle5d3dMeshStyleProps(obj.style),
      ...(obj.label !== undefined ? { label: obj.label } : {}),
      ...(obj.wormhole === true ? { wormhole: true } : {}),
      ...(obj.hidden === true ? { hidden: true } : {}),
      ...(obj.locked === true ? { locked: true } : {}),
    };
    const volumeGrips: Grip[] = obj.vortices.map((v) => {
      const parsed = parseGripFullId(v.id.includes(":") ? v.id : gripFullId(obj.id, v.id));
      const localId = parsed?.gripId ?? v.id;
      return {
        id: localId,
        gripKind: v.vortexKind ?? BUILTIN_PORT_HANDLE_KIND,
        "3d": {
          position: v.position,
          ...(v.direction !== undefined ? { direction: v.direction } : {}),
          ...(v.radius !== undefined ? { radius: v.radius } : {}),
          ...(v.label !== undefined ? { label: v.label } : {}),
          ...(v.handleMeshUrl !== undefined ? { handleMeshUrl: v.handleMeshUrl } : {}),
          ...(v.vortexMeshUrl !== undefined ? { vortexMeshUrl: v.vortexMeshUrl } : {}),
          ...(v.vortexMeshByLod !== undefined ? { vortexMeshByLod: v.vortexMeshByLod } : {}),
          ...(v.hidden === true ? { hidden: true } : {}),
          ...(v.locked === true ? { locked: true } : {}),
        },
      };
    });
    const existing = partsMap.get(obj.id);
    if (existing) {
      const gripById = new Map(existing.grips.map((a) => [a.id, a]));
      for (const a of volumeGrips) {
        const prev = gripById.get(a.id);
        gripById.set(a.id, prev ? { ...prev, "3d": a["3d"], gripKind: a.gripKind } : a);
      }
      partsMap.set(obj.id, {
        ...existing,
        ...(obj.objectKind !== undefined ? { partKind: obj.objectKind } : {}),
        "3d": volumeAspect,
        grips: [...gripById.values()],
      });
    } else {
      partsMap.set(obj.id, {
        id: obj.id,
        ...(obj.objectKind !== undefined ? { partKind: obj.objectKind } : {}),
        "3d": volumeAspect,
        grips: volumeGrips,
      });
    }
  }
  const fasteners: Fastener[] = [];
  const fastenerIds = new Set<string>();
  for (const edge of fixture2d.edges) {
    if (fastenerIds.has(edge.id)) continue;
    fastenerIds.add(edge.id);
    fasteners.push({
      id: edge.id,
      source: edge.source,
      target: edge.target,
      ...(edge.edgeKind !== undefined ? { fastenerKind: edge.edgeKind } : {}),
      ...spreadPuzzleConnectionTransformParams(edge),
    });
  }
  for (const att of fixture3d.attractions) {
    if (fastenerIds.has(att.id)) continue;
    fastenerIds.add(att.id);
    fasteners.push({
      id: att.id,
      source: att.attracting,
      target: att.attracted,
      ...(att.attractionKind !== undefined ? { fastenerKind: att.attractionKind } : {}),
      ...spreadPuzzleConnectionTransformParams(att),
    });
  }
  const meta = {
    ...(fixture2d.meta ?? {}),
    ...(fixture3d.meta ?? {}),
  };
  const kindCatalogs = kindCatalogsFromMetas({ meta2d: fixture2d.meta, meta3d: fixture3d.meta });
  const kindCompatibility = kindCompatibilityFromMetas({ meta2d: fixture2d.meta, meta3d: fixture3d.meta });
  return {
    schema: PUZZLE_5D_SCHEMA,
    domain: fixture3d.domain,
    camera2d: { ...fixture2d.camera },
    camera3d: { ...fixture3d.camera },
    parts: [...partsMap.values()],
    fasteners,
    ...(Object.keys(meta).length > 0 ? { meta } : {}),
    ...(kindCatalogs ? { kindCatalogs } : {}),
    ...(kindCompatibility.length > 0 ? { kindCompatibility } : {}),
    ...(fixture3d.references?.length ? { references3d: fixture3d.references } : {}),
    ...(fixture3d.targetVolumes?.length ? { targetVolumes3d: fixture3d.targetVolumes } : {}),
  };
}

/** @emoji 📐 Projects {@link Model} to a 2d fixture for WASM rendering. */
export function project2d(model: Model): Puzzle2dFixture {
  const nodes = model.parts
    .filter((p) => p["2d"])
    .map((p) => {
      const aspect2d = p["2d"]!;
      const handles = p.grips
        .filter((a) => a["2d"])
        .map((a) => ({
          id: gripFullId(p.id, a.id),
          angle: a["2d"]!.angle,
          handleKind: a["2d"]!.gripKind,
          ...(a["2d"]!.color !== undefined ? { color: a["2d"]!.color } : {}),
          ...(a["2d"]!.iconKind !== undefined ? { iconKind: a["2d"]!.iconKind } : {}),
          ...(a["2d"]!.radius !== undefined ? { radius: a["2d"]!.radius } : {}),
          ...(a["2d"]!.t !== undefined ? { t: a["2d"]!.t } : {}),
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
    schema: "puzzle.2d.fixture",
    camera: { ...model.camera2d },
    nodes,
    edges: model.fasteners.map((b) => ({
      id: b.id,
      source: b.source,
      target: b.target,
      ...(b.fastenerKind !== undefined ? { edgeKind: b.fastenerKind } : {}),
      ...spreadPuzzleConnectionTransformParams(b),
    })),
    ...(model.meta ? { meta: model.meta } : {}),
  };
}

/** @emoji 📐 Projects {@link Model} to a @puzzle/3d fixture for 3d rendering. */
export function project3d(model: Model): Puzzle3dFixture {
  const objects = model.parts
    .filter((p) => p["3d"])
    .map((p) => {
      const s = p["3d"]!;
      return {
        id: p.id,
        meshUrl: s.meshUrl,
        origin: s.origin,
        ...(p.partKind !== undefined ? { objectKind: p.partKind } : {}),
        ...(s.orientation !== undefined ? { orientation: s.orientation } : {}),
        ...(s.scale !== undefined ? { scale: s.scale } : {}),
        ...(s.meshByLod !== undefined ? { meshByLod: s.meshByLod } : {}),
        ...puzzle5d3dMeshStyleProps(s.style),
        ...(s.label !== undefined ? { label: s.label } : {}),
        ...(s.wormhole === true ? { wormhole: true } : {}),
        ...(s.hidden === true ? { hidden: true } : {}),
        ...(s.locked === true ? { locked: true } : {}),
        vortices: p.grips
          .filter((a) => a["3d"])
          .map((a) => ({
            id: gripFullId(p.id, a.id),
            position: a["3d"]!.position,
            ...(a.gripKind ? { vortexKind: a.gripKind } : {}),
            ...(a["3d"]!.direction !== undefined ? { direction: a["3d"]!.direction } : {}),
            ...(a["3d"]!.radius !== undefined ? { radius: a["3d"]!.radius } : {}),
            ...(a["3d"]!.label !== undefined ? { label: a["3d"]!.label } : {}),
            ...(a["3d"]!.handleMeshUrl !== undefined ? { handleMeshUrl: a["3d"]!.handleMeshUrl } : {}),
            ...(a["3d"]!.vortexMeshUrl !== undefined ? { vortexMeshUrl: a["3d"]!.vortexMeshUrl } : {}),
            ...(a["3d"]!.vortexMeshByLod !== undefined ? { vortexMeshByLod: a["3d"]!.vortexMeshByLod } : {}),
            ...(a["3d"]!.hidden === true ? { hidden: true } : {}),
            ...(a["3d"]!.locked === true ? { locked: true } : {}),
          })),
      };
    });
  return {
    schema: "puzzle.3d.fixture",
    domain: model.domain,
    camera: { ...model.camera3d },
    objects,
    attractions: model.fasteners.map((b) => ({
      id: b.id,
      attracting: b.source as `${string}:${string}`,
      attracted: b.target as `${string}:${string}`,
      ...(b.fastenerKind !== undefined ? { attractionKind: b.fastenerKind } : {}),
      ...spreadPuzzleConnectionTransformParams(b),
    })),
    references: [...(model.references3d ?? [])],
    targetVolumes: [...(model.targetVolumes3d ?? [])],
    ...(model.meta ? { meta: model.meta } : {}),
  };
}

//#region 🌤️Flatten

const FLATTEN_TOLERANCE = 0.01;
const DIAGRAM_RADIUS = 2.697;
const DIAGRAM_VERTICAL_V_EXTRA = 1.0;
const DIAGRAM_HORIZONTAL_SCALE = 3.0633;

type FlattenVec3 = readonly [number, number, number];
type FlattenQuat = readonly [number, number, number, number];
type FlattenMat4 = readonly [
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
];

interface FlattenPlaneInput {
  readonly origin: { readonly x: number; readonly y: number; readonly z: number };
  readonly xAxis: { readonly x: number; readonly y: number; readonly z: number };
  readonly yAxis: { readonly x: number; readonly y: number; readonly z: number };
}

interface FlattenCoordInput {
  readonly u: number;
  readonly v: number;
}

function flattenNormalize(v: FlattenVec3): FlattenVec3 {
  const len = Math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
  if (len > 0) {
    return [v[0] / len, v[1] / len, v[2] / len];
  }
  return [0, 0, 0];
}

function flattenCross(a: FlattenVec3, b: FlattenVec3): FlattenVec3 {
  return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
}

function flattenDot(a: FlattenVec3, b: FlattenVec3): number {
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

function flattenDegToRad(deg: number): number {
  return (deg * Math.PI) / 180;
}

function flattenRound6(v: number): number {
  return Math.round(v * 1_000_000) / 1_000_000;
}

function flattenDefaultPlane(): FlattenPlaneInput {
  return {
    origin: { x: 0, y: 0, z: 0 },
    xAxis: { x: 1, y: 0, z: 0 },
    yAxis: { x: 0, y: 1, z: 0 },
  };
}

function flattenPlaneToMatrix(plane: FlattenPlaneInput): FlattenMat4 {
  const x: FlattenVec3 = [plane.xAxis.x, plane.xAxis.y, plane.xAxis.z];
  const y: FlattenVec3 = [plane.yAxis.x, plane.yAxis.y, plane.yAxis.z];
  const z = flattenCross(x, y);
  return [x[0], y[0], z[0], plane.origin.x, x[1], y[1], z[1], plane.origin.y, x[2], y[2], z[2], plane.origin.z, 0, 0, 0, 1];
}

function flattenMatrixToPlane(matrix: FlattenMat4): FlattenPlaneInput {
  return {
    origin: { x: matrix[3], y: matrix[7], z: matrix[11] },
    xAxis: { x: matrix[0], y: matrix[4], z: matrix[8] },
    yAxis: { x: matrix[1], y: matrix[5], z: matrix[9] },
  };
}

function flattenMulMat(a: FlattenMat4, b: FlattenMat4): FlattenMat4 {
  const out = new Array<number>(16).fill(0);
  for (let col = 0; col < 4; col += 1) {
    for (let row = 0; row < 4; row += 1) {
      out[col * 4 + row] =
        a[row] * b[col * 4] + a[4 + row] * b[col * 4 + 1] + a[8 + row] * b[col * 4 + 2] + a[12 + row] * b[col * 4 + 3];
    }
  }
  return out as FlattenMat4;
}

function flattenTranslation(x: number, y: number, z: number): FlattenMat4 {
  return [1, 0, 0, x, 0, 1, 0, y, 0, 0, 1, z, 0, 0, 0, 1];
}

function flattenRotationAxis(axis: FlattenVec3, angle: number): FlattenMat4 {
  const [x, y, z] = axis;
  const c = Math.cos(angle);
  const s = Math.sin(angle);
  const t = 1 - c;
  return [
    t * x * x + c,
    t * x * y + s * z,
    t * x * z - s * y,
    0,
    t * x * y - s * z,
    t * y * y + c,
    t * y * z + s * x,
    0,
    t * x * z + s * y,
    t * y * z - s * x,
    t * z * z + c,
    0,
    0,
    0,
    0,
    1,
  ];
}

function flattenApplyMatVec3(matrix: FlattenMat4, vector: FlattenVec3): FlattenVec3 {
  return [
    matrix[0] * vector[0] + matrix[4] * vector[1] + matrix[8] * vector[2],
    matrix[1] * vector[0] + matrix[5] * vector[1] + matrix[9] * vector[2],
    matrix[2] * vector[0] + matrix[6] * vector[1] + matrix[10] * vector[2],
  ];
}

function flattenQuaternionFromUnitVectors(from: FlattenVec3, to: FlattenVec3): FlattenQuat {
  const r = flattenDot(from, to) + 1;
  let quat: FlattenQuat;
  if (r < 0.000_001) {
    if (Math.abs(from[0]) > Math.abs(from[2])) {
      quat = [-from[1], from[0], 0, 0];
    } else {
      quat = [0, -from[2], from[1], 0];
    }
  } else {
    const c = flattenCross(from, to);
    quat = [c[0], c[1], c[2], r];
  }
  const len = Math.sqrt(quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]);
  return [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len];
}

function flattenQuaternionToMatrix(quaternion: FlattenQuat): FlattenMat4 {
  const [x, y, z, w] = quaternion;
  const x2 = x + x;
  const y2 = y + y;
  const z2 = z + z;
  const xx = x * x2;
  const xy = x * y2;
  const xz = x * z2;
  const yy = y * y2;
  const yz = y * z2;
  const zz = z * z2;
  const wx = w * x2;
  const wy = w * y2;
  const wz = w * z2;
  return [
    1 - (yy + zz),
    xy + wz,
    xz - wy,
    0,
    xy - wz,
    1 - (xx + zz),
    yz + wx,
    0,
    xz + wy,
    yz - wx,
    1 - (xx + yy),
    0,
    0,
    0,
    0,
    1,
  ];
}

function flattenPlaneToQuaternion(plane: FlattenPlaneInput): FlattenQuat {
  const m00 = plane.xAxis.x;
  const m01 = plane.yAxis.x;
  const m02 = plane.xAxis.y * plane.yAxis.z - plane.xAxis.z * plane.yAxis.y;
  const m10 = plane.xAxis.y;
  const m11 = plane.yAxis.y;
  const m12 = plane.xAxis.z * plane.yAxis.y - plane.xAxis.y * plane.yAxis.z;
  const m20 = plane.xAxis.z;
  const m21 = plane.yAxis.z;
  const m22 = plane.xAxis.x * plane.yAxis.y - plane.xAxis.y * plane.yAxis.x;
  const trace = m00 + m11 + m22;
  if (trace > 0) {
    const s = Math.sqrt(trace + 1) * 2;
    return [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s];
  }
  if (m00 > m11 && m00 > m22) {
    const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
    return [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
  }
  if (m11 > m22) {
    const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
    return [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s];
  }
  const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
  return [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s];
}

function flattenQuatToPlane(origin: FlattenVec3, quaternion: FlattenQuat): FlattenPlaneInput {
  const matrix = flattenQuaternionToMatrix(quaternion);
  return {
    origin: { x: origin[0], y: origin[1], z: origin[2] },
    xAxis: { x: matrix[0], y: matrix[4], z: matrix[8] },
    yAxis: { x: matrix[1], y: matrix[5], z: matrix[9] },
  };
}

function flattenComputeChildPlane(
  parentPlane: FlattenPlaneInput,
  parentPoint: FlattenVec3,
  parentDirection: FlattenVec3,
  childPoint: FlattenVec3,
  childDirection: FlattenVec3,
  params: PuzzleConnectionTransformParams,
): FlattenPlaneInput {
  const parentMatrix = flattenPlaneToMatrix(parentPlane);
  const parentDir = flattenNormalize(parentDirection);
  const childDir = flattenNormalize(childDirection);
  const gap = connectionTransformParam(params, "gap");
  const shift = connectionTransformParam(params, "shift");
  const rise = connectionTransformParam(params, "rise");
  const rotationRad = flattenDegToRad(connectionTransformParam(params, "rotation"));
  const turnRad = flattenDegToRad(connectionTransformParam(params, "turn"));
  const tiltRad = flattenDegToRad(connectionTransformParam(params, "tilt"));
  const reverseChild: FlattenVec3 = [-childDir[0], -childDir[1], -childDir[2]];
  const crossVec = flattenCross(parentDir, reverseChild);
  const crossLen = Math.sqrt(crossVec[0] * crossVec[0] + crossVec[1] * crossVec[1] + crossVec[2] * crossVec[2]);
  let alignQuat: FlattenQuat;
  if (crossLen < FLATTEN_TOLERANCE) {
    if (Math.abs(parentDir[2]) < FLATTEN_TOLERANCE) {
      alignQuat = flattenQuaternionFromUnitVectors([0, 1, 0], [0, 0, -1]);
    } else {
      let axis = flattenCross([0, 0, 1], parentDir);
      axis = flattenNormalize(axis);
      const half = Math.PI / 2;
      alignQuat = [axis[0] * Math.sin(half), axis[1] * Math.sin(half), axis[2] * Math.sin(half), Math.cos(half)];
    }
  } else {
    alignQuat = flattenQuaternionFromUnitVectors(reverseChild, parentDir);
  }
  const directionT = flattenQuaternionToMatrix(alignQuat);
  const parentRotationT = flattenQuaternionToMatrix(flattenQuaternionFromUnitVectors([0, 1, 0], parentDir));
  const gapDirection = flattenApplyMatVec3(parentRotationT, [0, 1, 0]);
  const shiftDirection = flattenApplyMatVec3(parentRotationT, [1, 0, 0]);
  const raiseDirection = flattenApplyMatVec3(parentRotationT, [0, 0, 1]);
  let turnAxis = flattenApplyMatVec3(parentRotationT, [0, 0, 1]);
  let tiltAxis = flattenApplyMatVec3(parentRotationT, [1, 0, 0]);
  let orientationT = directionT;
  const rotateT = flattenRotationAxis(parentDir, -rotationRad);
  orientationT = flattenMulMat(rotateT, orientationT);
  turnAxis = flattenApplyMatVec3(rotateT, turnAxis);
  tiltAxis = flattenApplyMatVec3(rotateT, tiltAxis);
  orientationT = flattenMulMat(flattenRotationAxis(turnAxis, turnRad), orientationT);
  orientationT = flattenMulMat(flattenRotationAxis(tiltAxis, tiltRad), orientationT);
  const centerChildT = flattenTranslation(-childPoint[0], -childPoint[1], -childPoint[2]);
  let transform = flattenMulMat(orientationT, centerChildT);
  const gapTransform = flattenTranslation(gapDirection[0] * gap, gapDirection[1] * gap, gapDirection[2] * gap);
  const shiftTransform = flattenTranslation(shiftDirection[0] * shift, shiftDirection[1] * shift, shiftDirection[2] * shift);
  const raiseTransform = flattenTranslation(raiseDirection[0] * rise, raiseDirection[1] * rise, raiseDirection[2] * rise);
  transform = flattenMulMat(flattenMulMat(raiseTransform, flattenMulMat(shiftTransform, gapTransform)), transform);
  transform = flattenMulMat(flattenTranslation(parentPoint[0], parentPoint[1], parentPoint[2]), transform);
  return flattenMatrixToPlane(flattenMulMat(parentMatrix, transform));
}

function flattenGripTParam(grip: Grip): number {
  const t = grip["2d"]?.t;
  if (typeof t === "number" && Number.isFinite(t)) {
    return t;
  }
  const angle = grip["2d"]?.angle;
  if (typeof angle === "number" && Number.isFinite(angle)) {
    return angle / 360;
  }
  return 0;
}

function flattenResolveGrip3d(grip: Grip): { readonly point: FlattenVec3; readonly direction: FlattenVec3; readonly t: number } {
  const position = grip["3d"]?.position ?? [0, 0, 0];
  const direction = grip["3d"]?.direction ?? [0, 0, 1];
  return {
    point: [position[0], position[1], position[2]],
    direction: flattenNormalize([direction[0], direction[1], direction[2]]),
    t: flattenGripTParam(grip),
  };
}

function flattenStoredRootPlane(part: Part): FlattenPlaneInput {
  const aspect3d = part["3d"];
  if (!aspect3d) {
    return flattenDefaultPlane();
  }
  const origin = aspect3d.origin ?? [0, 0, 0];
  const orientation = aspect3d.orientation ?? [0, 0, 0, 1];
  return flattenQuatToPlane([origin[0], origin[1], origin[2]], orientation);
}

function flattenStoredRootCenter(part: Part): FlattenCoordInput {
  const aspect2d = part["2d"];
  return {
    u: aspect2d?.x ?? 0,
    v: aspect2d?.y ?? 0,
  };
}

/** @emoji 🌤️ Computes absolute part 3d origins/orientations and 2d centers from fasteners and local grip geometry. */
export function flatten5d(model: Model): Model {
  const partMap = new Map(model.parts.map((part) => [part.id, part]));
  const adjacency = new Map<string, { readonly neighborId: string; readonly fastener: Fastener; readonly sourceOnCurrent: boolean }[]>();
  for (const fastener of model.fasteners) {
    const sourceGrip = parseGripFullId(fastener.source);
    const targetGrip = parseGripFullId(fastener.target);
    if (!sourceGrip || !targetGrip) {
      continue;
    }
    if (!partMap.has(sourceGrip.partId) || !partMap.has(targetGrip.partId)) {
      continue;
    }
    const push = (fromId: string, toId: string, sourceOnCurrent: boolean) => {
      const row = adjacency.get(fromId) ?? [];
      row.push({ neighborId: toId, fastener, sourceOnCurrent });
      adjacency.set(fromId, row);
    };
    push(sourceGrip.partId, targetGrip.partId, true);
    push(targetGrip.partId, sourceGrip.partId, false);
  }
  const piecePlanes = new Map<string, FlattenPlaneInput>();
  const pieceCenters = new Map<string, FlattenCoordInput>();
  const visited = new Set<string>();
  const bfsRoot = (rootId: string) => {
    const queue: string[] = [rootId];
    visited.add(rootId);
    const rootPart = partMap.get(rootId);
    if (!rootPart) {
      return;
    }
    piecePlanes.set(rootId, flattenStoredRootPlane(rootPart));
    pieceCenters.set(rootId, flattenStoredRootCenter(rootPart));
    while (queue.length > 0) {
      const currentId = queue.shift()!;
      const currentPlane = piecePlanes.get(currentId) ?? flattenDefaultPlane();
      const parentCenter = pieceCenters.get(currentId) ?? { u: 0, v: 0 };
      const currentPart = partMap.get(currentId);
      if (!currentPart) {
        continue;
      }
      for (const edge of adjacency.get(currentId) ?? []) {
        if (visited.has(edge.neighborId)) {
          continue;
        }
        visited.add(edge.neighborId);
        const sourceGrip = parseGripFullId(edge.fastener.source);
        const targetGrip = parseGripFullId(edge.fastener.target);
        if (!sourceGrip || !targetGrip) {
          piecePlanes.set(edge.neighborId, flattenDefaultPlane());
          pieceCenters.set(edge.neighborId, { u: 0, v: 0 });
          queue.push(edge.neighborId);
          continue;
        }
        const parentGripId = edge.sourceOnCurrent ? sourceGrip : targetGrip;
        const childGripId = edge.sourceOnCurrent ? targetGrip : sourceGrip;
        const parentGrip = currentPart.grips.find((grip) => grip.id === parentGripId.gripId);
        const neighborPart = partMap.get(edge.neighborId);
        const childGrip = neighborPart?.grips.find((grip) => grip.id === childGripId.gripId);
        if (!parentGrip?.["3d"] || !childGrip?.["3d"]) {
          piecePlanes.set(edge.neighborId, flattenDefaultPlane());
          pieceCenters.set(edge.neighborId, { u: 0, v: 0 });
          queue.push(edge.neighborId);
          continue;
        }
        const parentGeom = flattenResolveGrip3d(parentGrip);
        const childGeom = flattenResolveGrip3d(childGrip);
        const childPlane = flattenComputeChildPlane(currentPlane, parentGeom.point, parentGeom.direction, childGeom.point, childGeom.direction, edge.fastener);
        piecePlanes.set(edge.neighborId, childPlane);
        const connectionU = connectionTransformParam(edge.fastener, "u");
        const connectionV = connectionTransformParam(edge.fastener, "v");
        let childU: number;
        let childV: number;
        if (parentCenter.u === 0 && parentCenter.v === 0) {
          const angle = 2 * Math.PI * parentGeom.t;
          childU = DIAGRAM_RADIUS * Math.sin(angle);
          childV = DIAGRAM_RADIUS * Math.cos(angle);
        } else if (Math.abs(parentGeom.direction[2]) > 0.5) {
          childU = parentCenter.u + connectionU;
          childV = parentCenter.v + connectionV + DIAGRAM_VERTICAL_V_EXTRA;
        } else {
          childU = parentCenter.u + connectionU * DIAGRAM_HORIZONTAL_SCALE;
          childV = parentCenter.v + connectionV * DIAGRAM_HORIZONTAL_SCALE;
        }
        pieceCenters.set(edge.neighborId, { u: flattenRound6(childU), v: flattenRound6(childV) });
        queue.push(edge.neighborId);
      }
    }
  };
  for (const part of model.parts) {
    if (!visited.has(part.id)) {
      bfsRoot(part.id);
    }
  }
  const parts = model.parts.map((part) => {
    const plane = piecePlanes.get(part.id) ?? flattenDefaultPlane();
    const center = pieceCenters.get(part.id) ?? { u: 0, v: 0 };
    const origin: [number, number, number] = [plane.origin.x, plane.origin.y, plane.origin.z];
    const orientation = flattenPlaneToQuaternion(plane);
    return {
      ...part,
      ...(part["3d"] ? { "3d": { ...part["3d"], origin, orientation } } : {}),
      ...(part["2d"] ? { "2d": { ...part["2d"], x: center.u, y: center.v } } : {}),
    };
  });
  return { ...model, parts };
}

/** @emoji 📷 Frames a 2d camera around flattened part centers. */
export function puzzle5dCamera2dFromModel(model: Model): Model["camera2d"] {
  const centers = model.parts
    .map((part) => part["2d"])
    .filter((aspect): aspect is NonNullable<typeof aspect> => aspect != null)
    .map((aspect) => ({ x: aspect.x, y: aspect.y }));
  if (centers.length === 0) {
    return { x: 0, y: 0, zoom: 1 };
  }
  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;
  for (const point of centers) {
    minX = Math.min(minX, point.x);
    maxX = Math.max(maxX, point.x);
    minY = Math.min(minY, point.y);
    maxY = Math.max(maxY, point.y);
  }
  const cx = (minX + maxX) / 2;
  const cy = (minY + maxY) / 2;
  const halfSpan = Math.max((maxX - minX) / 2, (maxY - minY) / 2, 40, 1);
  const refShort = 48 * 8;
  const zoom = Math.min(1, refShort / (2 * halfSpan));
  return { x: cx, y: cy, zoom };
}

/** @emoji 📷 Frames a 3d camera around flattened part origins. */
export function puzzle5dCamera3dFromModel(model: Model): Model["camera3d"] {
  const origins = model.parts
    .map((part) => part["3d"]?.origin)
    .filter((origin): origin is readonly [number, number, number] => origin != null);
  if (origins.length === 0) {
    return { position: [8, 8, 8], target: [0, 0, 0], zoom: 1 };
  }
  let sx = 0;
  let sy = 0;
  let sz = 0;
  for (const origin of origins) {
    sx += origin[0];
    sy += origin[1];
    sz += origin[2];
  }
  const target: [number, number, number] = [sx / origins.length, sy / origins.length, sz / origins.length];
  return { position: [target[0] + 8, target[1] + 8, target[2] + 8], target, zoom: 1 };
}

/** @emoji 🌤️ Flattens, scales diagram coordinates, and reframes cameras for topology rendering. */
export function prepareTopologyModel(model: Model, diagramScale = 48): Model {
  const flattened = flatten5d(model);
  const parts = flattened.parts.map((part) => {
    if (!part["2d"]) {
      return part;
    }
    return {
      ...part,
      "2d": {
        ...part["2d"],
        x: part["2d"].x * diagramScale,
        y: -part["2d"].y * diagramScale,
      },
    };
  });
  const nextModel = { ...flattened, parts };
  return {
    ...nextModel,
    camera2d: puzzle5dCamera2dFromModel(nextModel),
    camera3d: puzzle5dCamera3dFromModel(nextModel),
  };
}

//#endregion 🌤️Flatten
//#endregion 🔖Model

//#region 🔖Brush
export type Puzzle5dActiveTool = Puzzle2dActiveTool;

export const PUZZLE_5D_FILL_COUNT_MAX = PUZZLE_3D_FILL_COUNT_MAX;

export const PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID = "puzzle5d.tool.brush";
export const PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID = "puzzle5d.tool.select";
export const PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID = "puzzle5d.tool.fill";

/** @emoji 🖌️ One unified brush placement growing a {@link Part} with both flat and volume aspects. */
export interface Puzzle5dBrushPlacement {
  readonly partId?: string;
  readonly partKind: string;
  readonly sourceGripFullId: string;
  readonly aspect2d?: Puzzle2dBrushPlacePayload;
  readonly aspect3d?: BrushPlacePayload;
  readonly fastenerId?: string;
}

/** @emoji 🪣 Cached fill prefix session at unified model level. */
export interface Puzzle5dFillSession {
  readonly baseModel: Model;
  readonly sequence: readonly Puzzle5dBrushPlacement[];
  readonly seed: number;
}

export type Puzzle5dBrushPlacementApplyResult =
  | { readonly kind: "unchanged" }
  | { readonly kind: "placed"; readonly model: Model; readonly partId: string; readonly fastenerId: string };

function cloneModel(model: Model): Model {
  return JSON.parse(JSON.stringify(model)) as Model;
}

function partById(model: Model, partId: string): Part | undefined {
  return model.parts.find((row) => row.id === partId);
}

function peerPartForKind(model: Model, partKind: string): Part | undefined {
  return model.parts.find((row) => row.partKind === partKind && row["2d"] && row["3d"]);
}

function objectKind3d(catalogs: KindCatalogBundle | undefined, partKind: string): Puzzle3dPartKind | undefined {
  return project3dKindCatalogs(catalogs)?.objects?.find((row) => row.id === partKind);
}

type Puzzle5dVolumeGripTemplate = NonNullable<Puzzle3dPartKind["vortices"]>[number] & { readonly id?: string };

function volumeTemplatesForPartKind(model: Model, partKind: string, catalogs: KindCatalogBundle | undefined): readonly Puzzle5dVolumeGripTemplate[] | undefined {
  const fromCatalog = objectKind3d(catalogs, partKind)?.vortices;
  if (fromCatalog?.length) return fromCatalog;
  const peer = peerPartForKind(model, partKind);
  if (!peer) return undefined;
  const templates = peer.grips
    .filter((grip) => grip["3d"])
    .map((grip) => ({
      id: grip.id,
      vortexKind: grip.gripKind,
      position: grip["3d"]!.position,
      ...(grip["3d"]!.direction ? { direction: grip["3d"]!.direction } : {}),
      ...(grip["3d"]!.radius !== undefined ? { radius: grip["3d"]!.radius } : {}),
    }));
  return templates.length ? templates : undefined;
}

function flatTemplatesForPartKind(
  model: Model,
  partKind: string,
  catalogs: KindCatalogBundle | undefined,
): readonly { readonly angle: number; readonly handleKind: string; readonly radius?: number }[] | undefined {
  const fromCatalog = catalogs?.parts
    ?.find((row) => row.id === partKind)
    ?.grips?.filter((grip) => grip["2d"])
    .map((grip) => ({
      angle: grip["2d"]!.angle,
      handleKind: grip["2d"]!.gripKind,
      ...(grip["2d"]!.radius !== undefined ? { radius: grip["2d"]!.radius } : {}),
    }));
  if (fromCatalog?.length) return fromCatalog;
  const peer = peerPartForKind(model, partKind);
  const fromPeer = peer?.grips
    .filter((grip) => grip["2d"])
    .map((grip) => ({
      angle: grip["2d"]!.angle,
      handleKind: grip["2d"]!.gripKind,
      ...(grip["2d"]!.radius !== undefined ? { radius: grip["2d"]!.radius } : {}),
    }));
  return fromPeer?.length ? fromPeer : undefined;
}

function gripLocalIdFromIndex(index: number): string {
  return `v${index}`;
}

function slugGripLocalId(kind: string): string {
  const slug = kind.trim().replace(/\s+/g, "-").toLowerCase();
  return slug.length > 0 ? slug.slice(0, 48) : "link";
}

function uniqueGripLocalId(base: string | undefined, index: number, used: Set<string>): string {
  const seed = (base?.trim() || gripLocalIdFromIndex(index)).replaceAll(PUZZLE_5D_GRIP_ID_SEPARATOR, "-");
  let id = seed;
  let suffix = index;
  while (used.has(id)) {
    id = `${seed}-${suffix}`;
    suffix += 1;
  }
  used.add(id);
  return id;
}

function flatTemplatesFromObjectKind(kind: Puzzle3dPartKind | undefined): readonly { readonly angle: number; readonly handleKind: string; readonly radius?: number }[] {
  const count = Math.max(kind?.vortices?.length ?? 0, 1);
  return Array.from({ length: count }, (_, index) => ({
    angle: flatHandleConnectorAngle(index, count),
    handleKind: kind?.vortices?.[index]?.vortexKind ?? BUILTIN_PORT_HANDLE_KIND,
    radius: kind?.vortices?.[index]?.radius ?? 3,
  }));
}

function buildGripsUnified(
  partId: string,
  partKind: string,
  catalogs: KindCatalogBundle | undefined,
  flatTemplates?: readonly { readonly angle: number; readonly handleKind: string; readonly radius?: number }[],
  volumeTemplates?: readonly Puzzle5dVolumeGripTemplate[],
): Grip[] {
  const kind3d = objectKind3d(catalogs, partKind);
  const vortices = volumeTemplates ?? kind3d?.vortices ?? [];
  const flat = flatTemplates ?? flatTemplatesFromObjectKind(kind3d);
  const count = Math.max(vortices.length, flat.length, 1);
  const grips: Grip[] = [];
  const usedIds = new Set<string>();
  for (let index = 0; index < count; index += 1) {
    const vortex = vortices[index];
    const handle = flat[index] ?? flat[0];
    const localId = uniqueGripLocalId(vortex?.id ?? (vortex?.vortexKind ? slugGripLocalId(vortex.vortexKind) : undefined), index, usedIds);
    grips.push({
      id: localId,
      gripKind: vortex?.vortexKind ?? handle?.handleKind ?? BUILTIN_PORT_HANDLE_KIND,
      ...(handle
        ? {
            "2d": {
              angle: handle.angle,
              gripKind: handle.handleKind,
              ...(handle.radius !== undefined ? { radius: handle.radius } : {}),
            },
          }
        : {}),
      ...(vortex
        ? {
            "3d": {
              position: vortex.position,
              ...(vortex.direction ? { direction: vortex.direction } : {}),
              ...(vortex.radius !== undefined ? { radius: vortex.radius } : {}),
              ...(vortex.vortexKind ? { label: vortex.vortexKind } : {}),
            },
          }
        : {}),
    });
  }
  return grips;
}

function mergeGripsFlatAndVolume(
  volumeGrips: readonly Grip[],
  flatHandles: readonly { readonly id: string; readonly angle: number; readonly handleKind: string; readonly radius?: number }[],
): Grip[] {
  const count = Math.max(volumeGrips.length, flatHandles.length);
  const out: Grip[] = [];
  const usedIds = new Set<string>();
  for (let index = 0; index < count; index += 1) {
    const volume = volumeGrips[index];
    const flat = flatHandles[index];
    const localId = uniqueGripLocalId(volume?.id ?? parseGripFullId(flat?.id ?? "")?.gripId, index, usedIds);
    out.push({
      id: localId,
      gripKind: volume?.gripKind ?? flat?.handleKind ?? BUILTIN_PORT_HANDLE_KIND,
      ...(flat
        ? {
            "2d": {
              angle: flat.angle,
              gripKind: flat.handleKind,
              ...(flat.radius !== undefined ? { radius: flat.radius } : {}),
            },
          }
        : volume && volume["2d"]
          ? { "2d": volume["2d"] }
          : {}),
      ...(volume?.["3d"] ? { "3d": volume["3d"] } : {}),
    });
  }
  return out;
}

function volumeGripIndexOnPart(part: Part, gripLocalId: string): number {
  return part.grips.filter((grip) => grip["3d"]).findIndex((grip) => grip.id === gripLocalId);
}

function synthesizeVolumeAspectFromFlat(
  model: Model,
  payload: Puzzle2dBrushPlacePayload,
  partKind: string,
  catalogs: KindCatalogBundle | undefined,
): { readonly aspect: Part3dAspect; readonly grips: Grip[] } | null {
  const parsed = parseGripFullId(payload.sourceHandleId);
  if (!parsed) return null;
  const sourcePart = partById(model, parsed.partId);
  if (!sourcePart?.["3d"]) return null;
  const fixture3d = project3d(model);
  const hostObject = fixture3d.objects.find((row) => row.id === parsed.partId);
  if (!hostObject) return null;
  const volumeIndex = volumeGripIndexOnPart(sourcePart, parsed.gripId);
  if (volumeIndex < 0) return null;
  const world = vortexWorldCadFromObject(hostObject, volumeIndex);
  if (!world) return null;
  const cat3d = project3dKindCatalogs(catalogs);
  const kind = objectKind3d(catalogs, partKind);
  const templates = volumeTemplatesForPartKind(model, partKind, catalogs);
  const template = templates?.[payload.targetHandleIndex];
  const meshUrl = resolveObjectKindMeshUrl(partKind, cat3d, fixture3d) ?? peerPartForKind(model, partKind)?.["3d"]?.meshUrl;
  if (!meshUrl || !template) return null;
  const targetGrip = sourcePart.grips.find((grip) => grip.id === parsed.gripId);
  const sourceVk = template.vortexKind ?? "";
  const targetVk = targetGrip?.gripKind ?? "";
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
  const grips = buildGripsUnified("", partKind, catalogs, payload.handles, templates);
  return {
    aspect: {
      origin: pose.origin,
      orientation: pose.orientation,
      meshUrl,
      ...(kind?.scale !== undefined ? { scale: kind.scale } : {}),
      ...(kind?.label ?? kind?.name ? { label: kind.label ?? kind.name } : {}),
    },
    grips,
  };
}

function synthesizeVolumeAspectFromBrushPayload(
  model: Model,
  payload: BrushPlacePayload,
  partKind: string,
  catalogs: KindCatalogBundle | undefined,
): { readonly aspect: Part3dAspect; readonly grips: Grip[] } | null {
  const cat3d = project3dKindCatalogs(catalogs);
  const kind = objectKind3d(catalogs, partKind);
  const meshUrl = resolveObjectKindMeshUrl(partKind, cat3d, project3d(model));
  if (!meshUrl) return null;
  const templates = volumeTemplatesForPartKind(model, partKind, catalogs);
  const flatTemplates = flatTemplatesForPartKind(model, partKind, catalogs);
  const grips = buildGripsUnified("", partKind, catalogs, flatTemplates, templates);
  return {
    aspect: {
      origin: payload.origin,
      orientation: payload.orientation,
      meshUrl,
      ...(payload.scale !== undefined ? { scale: payload.scale } : kind?.scale !== undefined ? { scale: kind.scale } : {}),
      label: kind?.label ?? kind?.name ?? partKind,
    },
    grips,
  };
}

function synthesizeFlatAspectFromVolume(
  model: Model,
  payload: BrushPlacePayload,
  partKind: string,
  suggestionOffset = DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
): Part2dAspect | null {
  const parsed = parseGripFullId(payload.targetVortexFullId);
  if (!parsed) return null;
  const sourcePart = partById(model, parsed.partId);
  if (!sourcePart?.["2d"]) return null;
  const source2d = sourcePart["2d"];
  const sourceGrip = sourcePart.grips.find((grip) => grip.id === parsed.gripId);
  const angle = sourceGrip?.["2d"]?.angle ?? flatHandleConnectorAngle(0, 1);
  const center = puzzle2dBrushSlotCenterFromNode(
    {
      x: source2d.x,
      y: source2d.y,
      shape: source2d.shape,
      ...(source2d.radius !== undefined ? { radius: source2d.radius } : {}),
      ...(source2d.width !== undefined ? { width: source2d.width } : {}),
      ...(source2d.height !== undefined ? { height: source2d.height } : {}),
    },
    angle,
    suggestionOffset,
  );
  const x = center.x;
  const y = center.y;
  const peer = peerPartForKind(model, partKind);
  const iconKind = peer?.["2d"]?.iconKind;
  if ((peer?.["2d"]?.shape ?? "rectangle") === "rectangle") {
    return {
      x,
      y,
      shape: "rectangle",
      width: peer?.["2d"]?.width ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
      height: peer?.["2d"]?.height ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
      ...(iconKind ? { iconKind } : {}),
    };
  }
  return {
    x,
    y,
    shape: "circle",
    radius: peer?.["2d"]?.radius ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX / 2,
    ...(iconKind ? { iconKind } : {}),
  };
}

/** @emoji 🖌️ Builds a unified placement from a flat WASM brush payload. */
export function puzzle5dBrushPlacementFromFlat(payload: Puzzle2dBrushPlacePayload): Puzzle5dBrushPlacement {
  return {
    partKind: payload.nodeKind,
    sourceGripFullId: payload.sourceHandleId,
    aspect2d: payload,
    ...(payload.nodeId ? { partId: payload.nodeId } : {}),
  };
}

/** @emoji 🖌️ Commits a flat brush placement into the unified model and syncs every flat authoring pane. */
export function puzzle5dCommitBrushPlacementToPlay(store: Store, payload: Puzzle2dBrushPlacePayload): boolean {
  const placed = store.applyBrushPlacementDetailed(puzzle5dBrushPlacementFromFlat(payload));
  if (!placed) {
    return false;
  }
  puzzle2dGuardBrushPlacementStructuralDeletes(placed.partId, placed.fastenerId);
  puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(project2d(store.read()));
  puzzle2dPushAuthoritativeSceneToAllAuthoringPeers();
  puzzle2dFinalizeBrushSuggestionsPlacement();
  return true;
}

/** @emoji 🖌️ Builds a unified placement from a volume brush payload. */
export function puzzle5dBrushPlacementFromVolume(payload: BrushPlacePayload): Puzzle5dBrushPlacement {
  return {
    partKind: payload.objectKindId,
    sourceGripFullId: payload.targetVortexFullId,
    aspect3d: payload,
    ...(payload.objectId ? { partId: payload.objectId } : {}),
    ...(payload.attractionId ? { fastenerId: payload.attractionId } : {}),
  };
}

/** @emoji 🖌️ Commits a volume brush placement into the unified model and clears paired suggestion previews. */
export function puzzle5dCommitVolumeBrushPlacementToPlay(store: Store, payload: BrushPlacePayload): boolean {
  const session = puzzle2dGetBrushSessionSnapshot();
  const flatPayload = session ? puzzle2dBrushPlacePayloadFromSessionSnapshot(session) : null;
  const placement = flatPayload
    ? { ...puzzle5dBrushPlacementFromVolume(payload), aspect2d: flatPayload }
    : puzzle5dBrushPlacementFromVolume(payload);
  const placed = store.applyBrushPlacementDetailed(placement);
  if (!placed) {
    return false;
  }
  puzzle5dBrushPairedSyncOrigin = null;
  puzzle3dBrushPairedSyncRef.current.clear();
  puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(project2d(store.read()));
  puzzle2dPushAuthoritativeSceneToAllAuthoringPeers();
  puzzle2dFinalizeBrushSuggestionsPlacement();
  return true;
}

/** @emoji 🖌️ Appends one unified part and tie from a brush placement. */
export function applyBrushPlacementToModel(model: Model, placement: Puzzle5dBrushPlacement): Puzzle5dBrushPlacementApplyResult {
  const partId =
    placement.partId?.trim() ||
    placement.aspect2d?.nodeId?.trim() ||
    placement.aspect3d?.objectId?.trim() ||
    `puzzle5d.brush.${crypto.randomUUID()}`;
  const partKind = placement.partKind;
  const catalogs = model.kindCatalogs;
  let flatAspect: Part2dAspect | undefined;
  let volumeAspect: Part3dAspect | undefined;
  let grips: Grip[];
  let fastenerSource = "";
  let fastenerTarget = "";

  if (placement.aspect2d && placement.aspect3d) {
    const flatPayload = placement.aspect2d;
    const volumePayload = placement.aspect3d;
    const flatHandles = puzzle2dFixtureHandlesFromNodeKind(partId, flatPayload.handles);
    if (flatPayload.targetHandleIndex < 0 || flatPayload.targetHandleIndex >= flatHandles.length) {
      return { kind: "unchanged" };
    }
    const iconKind = flatPayload.iconKind;
    flatAspect =
      flatPayload.shape === "rectangle"
        ? {
            x: flatPayload.x,
            y: flatPayload.y,
            shape: "rectangle",
            width: flatPayload.width ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
            height: flatPayload.height ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
            ...(iconKind ? { iconKind } : {}),
          }
        : {
            x: flatPayload.x,
            y: flatPayload.y,
            shape: "circle",
            radius: flatPayload.radius ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX / 2,
            ...(iconKind ? { iconKind } : {}),
          };
    const volume = synthesizeVolumeAspectFromBrushPayload(model, volumePayload, partKind, catalogs);
    if (!volume) return { kind: "unchanged" };
    volumeAspect = volume.aspect;
    grips = mergeGripsFlatAndVolume(volume.grips, flatHandles);
    const matingLocal = grips[volumePayload.sourceVortexIndex]?.id;
    if (!matingLocal) return { kind: "unchanged" };
    fastenerSource = gripFullId(partId, matingLocal);
    fastenerTarget = volumePayload.targetVortexFullId;
  } else if (placement.aspect2d) {
    const payload = placement.aspect2d;
    const flatHandles = puzzle2dFixtureHandlesFromNodeKind(partId, payload.handles);
    if (payload.targetHandleIndex < 0 || payload.targetHandleIndex >= flatHandles.length) {
      return { kind: "unchanged" };
    }
    const iconKind = payload.iconKind;
    flatAspect =
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
    volumeAspect = volume.aspect;
    grips = mergeGripsFlatAndVolume(volume.grips, flatHandles);
    fastenerSource = payload.sourceHandleId;
    fastenerTarget = gripFullId(partId, grips[payload.targetHandleIndex].id);
  } else if (placement.aspect3d) {
    const payload = placement.aspect3d;
    const volume = synthesizeVolumeAspectFromBrushPayload(model, payload, partKind, catalogs);
    if (!volume) return { kind: "unchanged" };
    volumeAspect = volume.aspect;
    grips = volume.grips;
    const matingLocal = grips[payload.sourceVortexIndex]?.id;
    if (!matingLocal) return { kind: "unchanged" };
    fastenerSource = gripFullId(partId, matingLocal);
    fastenerTarget = payload.targetVortexFullId;
    const flat = synthesizeFlatAspectFromVolume(model, payload, partKind);
    if (!flat) return { kind: "unchanged" };
    flatAspect = flat;
  } else {
    return { kind: "unchanged" };
  }

  if (model.parts.some((row) => row.id === partId)) return { kind: "unchanged" };
  if (model.fasteners.some((f) => f.source === fastenerSource && f.target === fastenerTarget)) return { kind: "unchanged" };

  const fastenerId = placement.fastenerId?.trim() || `puzzle5d.brush.fastener.${crypto.randomUUID()}`;
  const part: Part = {
    id: partId,
    partKind,
    ...(flatAspect ? { "2d": flatAspect } : {}),
    ...(volumeAspect ? { "3d": volumeAspect } : {}),
    grips,
  };
  return {
    kind: "placed",
    model: {
      ...model,
      parts: [...model.parts, part],
      fasteners: [...model.fasteners, { id: fastenerId, source: fastenerSource, target: fastenerTarget }],
    },
    partId,
    fastenerId,
  };
}

/** @emoji 🪣 Applies an ordered brush prefix onto a base unified model. */
export function applyFillPlacementsToModel(base: Model, placements: readonly Puzzle5dBrushPlacement[]): Model {
  let next = base;
  for (const placement of placements) {
    const result = applyBrushPlacementToModel(next, placement);
    if (result.kind === "placed") {
      next = result.model;
    }
  }
  return next;
}

export type Puzzle5dModelEditOp =
  | { readonly op: "setDocument"; readonly document: Model }
  | { readonly op: "patchParts"; readonly partIds: readonly string[]; readonly field: Puzzle5dPartPatchField; readonly value: unknown }
  | { readonly op: "patchGrips"; readonly gripFullIds: readonly string[]; readonly field: Puzzle5dGripPatchField; readonly value: unknown }
  | { readonly op: "applyBrushPlacement"; readonly placement: Puzzle5dBrushPlacement }
  | { readonly op: "deletePartsAndGrips"; readonly partIds: readonly string[]; readonly gripIds: readonly string[] }
  | {
      readonly op: "applyFillPrefix";
      readonly baseModel: Model;
      readonly placements: readonly Puzzle5dBrushPlacement[];
      readonly count: number;
    };

/** @emoji 🚪 Applies one semantic puzzle 5d model edit (CQRS projection applier). */
export function applyPuzzle5dModelEditOp(model: Model, op: Puzzle5dModelEditOp): Model {
  switch (op.op) {
    case "setDocument":
      return op.document;
    case "patchParts": {
      if (!op.partIds.length) return model;
      const idSet = new Set(op.partIds);
      return {
        ...model,
        parts: model.parts.map((part) => (idSet.has(part.id) ? patchPuzzle5dPartRow(part, op.field, op.value) : part)),
      };
    }
    case "patchGrips": {
      if (!op.gripFullIds.length) return model;
      const idSet = new Set(op.gripFullIds);
      return {
        ...model,
        parts: model.parts.map((part) => {
          let changed = false;
          const grips = part.grips.map((grip) => {
            const fullId = gripFullId(part.id, grip.id);
            if (!idSet.has(fullId)) return grip;
            changed = true;
            return patchPuzzle5dGripRow(grip, op.field, op.value);
          });
          return changed ? { ...part, grips } : part;
        }),
      };
    }
    case "applyBrushPlacement": {
      const result = applyBrushPlacementToModel(model, op.placement);
      return result.kind === "placed" ? result.model : model;
    }
    case "deletePartsAndGrips": {
      let next = model;
      for (const partId of op.partIds) {
        const removed = removePartFromModel(next, partId);
        if (removed) next = removed;
      }
      for (const gripId of op.gripIds) {
        const removed = removeGripFromModel(next, gripId);
        if (removed) next = removed;
      }
      return next;
    }
    case "applyFillPrefix": {
      const clamped = Math.max(0, Math.min(PUZZLE_5D_FILL_COUNT_MAX, Math.round(op.count)));
      return applyFillPlacementsToModel(op.baseModel, op.placements.slice(0, clamped));
    }
  }
}

/** @emoji ↩️ Inverts a puzzle 5d model edit from the pre-apply projection. */
export function backwardsPuzzle5dModelEditOp(model: Model, op: Puzzle5dModelEditOp): readonly Puzzle5dModelEditOp[] {
  switch (op.op) {
    case "setDocument":
      return [{ op: "setDocument", document: model }];
    case "patchParts":
    case "patchGrips":
    case "applyBrushPlacement":
    case "deletePartsAndGrips":
    case "applyFillPrefix":
      return [{ op: "setDocument", document: model }];
  }
}

/** @emoji 📊 Returns the puzzle 5d model edit payload for persistence diffs. */
export function diffPuzzle5dModelEditOp(_model: Model, operation: Puzzle5dModelEditOp): unknown {
  return operation;
}

/** @emoji 🪣 Volume-authoritative fill sequence mapped to unified placements (2d aspects synthesized). */
export function buildPuzzle5dFillSequence(args: {
  readonly model: Model;
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

//#region 🔖BrushPairedSync
export const puzzle5dFlatRendererRef: { current: Puzzle2dRenderer | null } = { current: null };

let puzzle5dBrushPairedSyncGuard: "flat" | "volume" | null = null;
let puzzle5dBrushPairedSyncOrigin: "flat" | "volume" | null = null;

function puzzle5dBrushPairedCandidateKey(args: {
  readonly gripFullId: string;
  readonly objectKindId: string;
  readonly sourceVortexIndex: number;
}): string {
  return `${args.gripFullId}\u0001${args.objectKindId}\u0001${args.sourceVortexIndex}`;
}

function puzzle5dBrushPairedKeyFromFlatSession(session: Puzzle2dBrushSessionSnapshot | null): string | null {
  if (!session?.sourceHandleId) {
    return null;
  }
  const candidate = session.candidates[session.candidateIndex];
  if (!candidate) {
    return null;
  }
  return puzzle5dBrushPairedCandidateKey({
    gripFullId: session.sourceHandleId,
    objectKindId: candidate.nodeKind,
    sourceVortexIndex: candidate.targetHandleIndex,
  });
}

function puzzle5dBrushPairedKeyFromVolumePreview(preview: BrushPreviewState | null): string | null {
  if (!preview) {
    return null;
  }
  return puzzle5dBrushPairedCandidateKey({
    gripFullId: preview.targetVortexFullId,
    objectKindId: preview.objectKindId,
    sourceVortexIndex: preview.sourceVortexIndex,
  });
}

function puzzle5dFlatBrushMirrorsVolumePreview(
  session: Puzzle2dBrushSessionSnapshot | null,
  preview: BrushPreviewState | null,
): boolean {
  return puzzle5dBrushPairedKeyFromFlatSession(session) === puzzle5dBrushPairedKeyFromVolumePreview(preview);
}

function puzzle5dGripBindingMeta(model: Model, gripFullId: string): VortexBindingMeta | null {
  const parsed = parseGripFullId(gripFullId);
  if (!parsed) {
    return null;
  }
  const part = model.parts.find((row) => row.id === parsed.partId);
  const grip = part?.grips.find((row) => row.id === parsed.gripId);
  if (!part || !grip) {
    return null;
  }
  return {
    fullId: gripFullId,
    objectId: parsed.partId,
    objectKind: part.partKind,
    vortexKind: grip.gripKind,
    radiusWorld: grip["3d"]?.radius ?? 0.36,
  };
}

/** @emoji 🔗 Mirrors flat brush suggestion preview onto the paired volume surface. */
export function puzzle5dSyncBrushPreviewFromFlat(
  session: Puzzle2dBrushSessionSnapshot | null,
  model: Model,
): void {
  if (puzzle5dBrushPairedSyncGuard === "volume") {
    return;
  }
  puzzle5dBrushPairedSyncGuard = "flat";
  try {
    if (!session?.sourceHandleId) {
      puzzle5dBrushPairedSyncOrigin = null;
      if (!puzzle3dBrushUiStore.getSnapshot().preview) {
        return;
      }
      puzzle3dBrushPairedSyncRef.current.clear();
      return;
    }
    const meta = puzzle5dGripBindingMeta(model, session.sourceHandleId);
    const candidate = session.candidates[session.candidateIndex];
    if (!meta || !candidate) {
      return;
    }
    const semanticCandidate = { objectKindId: candidate.nodeKind, sourceVortexIndex: candidate.targetHandleIndex };
    if (puzzle5dFlatBrushMirrorsVolumePreview(session, puzzle3dBrushUiStore.getSnapshot().preview)) {
      return;
    }
    puzzle5dBrushPairedSyncOrigin = "flat";
    puzzle3dBrushPairedSyncRef.current.syncFromFlat({
      targetGripFullId: session.sourceHandleId,
      meta,
      candidate: semanticCandidate,
    });
  } finally {
    puzzle5dBrushPairedSyncGuard = null;
  }
}

/** @emoji 🔗 Mirrors volume brush suggestion preview onto the paired flat surface. */
export function puzzle5dSyncBrushPreviewFromVolume(
  preview: BrushPreviewState | null,
  flatCatalogs: Puzzle2dKindCatalogBundle | undefined,
): void {
  if (puzzle5dBrushPairedSyncGuard === "flat") {
    return;
  }
  const flatRenderer = puzzle5dFlatRendererRef.current;
  if (!flatRenderer) {
    return;
  }
  puzzle5dBrushPairedSyncGuard = "volume";
  try {
    if (!preview) {
      const flatSession = puzzle2dGetBrushSessionSnapshot();
      if (!flatSession?.sourceHandleId) {
        puzzle5dBrushPairedSyncOrigin = null;
        return;
      }
      if (puzzle5dBrushPairedSyncOrigin === "flat") {
        return;
      }
      flatRenderer.brushCancelSlot();
      puzzle5dBrushPairedSyncOrigin = null;
      return;
    }
    const flatSession = puzzle2dGetBrushSessionSnapshot();
    if (puzzle5dFlatBrushMirrorsVolumePreview(flatSession, preview)) {
      return;
    }
    puzzle5dBrushPairedSyncOrigin = "volume";
    if (flatSession?.sourceHandleId !== preview.targetVortexFullId) {
      flatRenderer.brushOpenSlot(preview.targetVortexFullId);
    }
    const nextSession = puzzle2dGetBrushSessionSnapshot();
    const index =
      nextSession?.candidates.findIndex(
        (row) => row.nodeKind === preview.objectKindId && row.targetHandleIndex === preview.sourceVortexIndex,
      ) ?? -1;
    if (index >= 0 && nextSession?.candidateIndex !== index) {
      flatRenderer.setBrushCandidateIndex(index);
      puzzle2dApplyBrushSuggestionsCandidateIndex(index, { kindCatalogs: flatCatalogs });
    }
  } finally {
    puzzle5dBrushPairedSyncGuard = null;
  }
}

function Puzzle5dFlatRendererRegistrar(): null {
  const renderer = usePuzzle2dRenderer();
  reactHostPort.useEffect(() => {
    puzzle5dFlatRendererRef.current = renderer;
    return () => {
      if (puzzle5dFlatRendererRef.current === renderer) {
        puzzle5dFlatRendererRef.current = null;
      }
    };
  }, [renderer]);
  return null;
}

/** @emoji 🔗 Keeps brush suggestion highlight preview aligned across paired flat and volume surfaces. */
export function Puzzle5dBrushPairedSync(): null {
  const store = useStore();
  const snap = useSnapshot();
  const flatSession = reactHostPort.useSyncExternalStore(puzzle2dSubscribeBrushSession, puzzle2dGetBrushSessionSnapshot, puzzle2dGetBrushSessionSnapshot);
  const volumeBrushEpoch = reactHostPort.useSyncExternalStore(
    subscribePuzzle3dBrushEngagementSource,
    getPuzzle3dBrushEngagementEpoch,
    getPuzzle3dBrushEngagementEpoch,
  );
  const flatCatalogs = reactHostPort.useMemo(() => project2dKindCatalogs(snap.model.kindCatalogs), [snap.model.kindCatalogs]);
  const model = snap.model;
  reactHostPort.useEffect(() => {
    puzzle5dSyncBrushPreviewFromFlat(flatSession, model);
  }, [flatSession, model]);
  reactHostPort.useEffect(() => {
    puzzle5dSyncBrushPreviewFromVolume(puzzle3dBrushUiStore.getSnapshot().preview, flatCatalogs);
  }, [volumeBrushEpoch, flatCatalogs]);
  void store;
  return null;
}
//#endregion 🔖BrushPairedSync

//#region 🔖PaletteDrop
function nodeAspectFromPaletteNode(node: Puzzle2dFixtureNode): Part2dAspect {
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

function paletteVolumeOriginFromFlat(peer: Part, flatX: number, flatY: number): Vec3 {
  const flat = peer["2d"]!;
  const volume = peer["3d"]!;
  return [volume.origin[0] + (flatX - flat.x), volume.origin[1] - (flatY - flat.y), volume.origin[2]];
}

function paletteFlatCenterFromVolume(peer: Part, origin: Vec3): { readonly x: number; readonly y: number } {
  const flat = peer["2d"]!;
  const volume = peer["3d"]!;
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
export function partFromPaletteNodeDrop(model: Model, node: Puzzle2dFixtureNode): Part | null {
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
  const origin = peer?.["2d"] && peer?.["3d"] ? paletteVolumeOriginFromFlat(peer, node.x, node.y) : ([node.x, -node.y, 0] as Vec3);
  const flatHandles = node.handles.map((handle) => ({
    angle: handle.angle,
    handleKind: handle.handleKind,
    ...(handle.radius !== undefined ? { radius: handle.radius } : {}),
  }));
  const volumeTemplates = volumeTemplatesForPartKind(model, partKind, catalogs);
  const grips = buildGripsUnified(node.id, partKind, catalogs, flatHandles, volumeTemplates);
  return {
    id: node.id,
    partKind,
    "2d": nodeAspectFromPaletteNode(node),
    "3d": {
      origin,
      meshUrl,
      orientation: peer?.["3d"]?.orientation ?? ([0, 0, 0, 1] as Quat),
      ...puzzle5d3dMeshStyleProps(peer?.["3d"]?.style),
      ...(kind.scale !== undefined ? { scale: kind.scale } : peer?.["3d"]?.scale !== undefined ? { scale: peer["3d"].scale } : {}),
      ...(kind.label ?? kind.name ? { label: kind.label ?? kind.name } : {}),
    },
    grips,
  };
}

/** @emoji 📥 Builds one unified part from a volume palette object drop. */
export function partFromPaletteObjectDrop(model: Model, object: Puzzle3dFixtureObject): Part | null {
  const partKind = object.objectKind?.trim();
  if (!partKind) {
    return null;
  }
  const catalogs = model.kindCatalogs;
  const template = buildPaletteNodeDragFixture(partKind, project2dKindCatalogs(catalogs)).nodes[0];
  if (!template) {
    return null;
  }
  const peer = peerPartForKind(model, partKind);
  const center = peer?.["2d"] && peer?.["3d"] ? paletteFlatCenterFromVolume(peer, object.origin) : { x: object.origin[0], y: -object.origin[1] };
  const flatNode: Puzzle2dFixtureNode = {
    ...template,
    id: object.id,
    x: center.x,
    y: center.y,
    handles: template.handles.map((handle, index) => ({ ...handle, id: `${object.id}.h${index}` })),
  };
  const volumeGrips: Grip[] = object.vortices.map((vortex, index) => {
    const parsed = parseGripFullId(vortex.id.includes(":") ? vortex.id : gripFullId(object.id, vortex.id));
    return {
      id: parsed?.gripId ?? gripLocalIdFromIndex(index),
      gripKind: vortex.vortexKind ?? BUILTIN_PORT_HANDLE_KIND,
      "3d": {
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
  const grips = mergeGripsFlatAndVolume(volumeGrips, flatHandles);
  return {
    id: object.id,
    partKind,
    "2d": nodeAspectFromPaletteNode(flatNode),
    "3d": {
      origin: object.origin,
      meshUrl: object.meshUrl,
      ...(object.orientation !== undefined ? { orientation: object.orientation } : {}),
      ...(object.scale !== undefined ? { scale: object.scale } : {}),
      ...puzzle5d3dMeshStyleProps(object.style),
      ...(object.label !== undefined ? { label: object.label } : {}),
      ...(object.wormhole === true ? { wormhole: true } : {}),
    },
    grips,
  };
}
//#endregion 🔖PaletteDrop

//#region 🔖StructuralDelete
function gripFullIdsForPart(part: Part): readonly string[] {
  return part.grips.map((grip) => gripFullId(part.id, grip.id));
}

function fastenerTouchesPartOrGrips(fastener: Fastener, partId: string, gripIds: ReadonlySet<string>): boolean {
  if (fastener.source === partId || fastener.target === partId) {
    return true;
  }
  return gripIds.has(fastener.source) || gripIds.has(fastener.target);
}

/** @emoji 🗑️ Removes one unified part and every tie touching it or its anchors. */
export function removePartFromModel(model: Model, partId: string): Model | null {
  const part = model.parts.find((row) => row.id === partId);
  if (!part) {
    return null;
  }
  const gripIds = new Set(gripFullIdsForPart(part));
  return {
    ...model,
    parts: model.parts.filter((row) => row.id !== partId),
    fasteners: model.fasteners.filter((f) => !fastenerTouchesPartOrGrips(f, partId, gripIds)),
  };
}

/** @emoji 🗑️ Removes one anchor and ties that referenced it. */
export function removeGripFromModel(model: Model, fullGripId: string): Model | null {
  const parsed = parseGripFullId(fullGripId);
  if (!parsed) {
    return null;
  }
  const part = model.parts.find((row) => row.id === parsed.partId);
  if (!part || !part.grips.some((grip) => grip.id === parsed.gripId)) {
    return null;
  }
  return {
    ...model,
    parts: model.parts.map((row) =>
      row.id !== parsed.partId ? row : { ...row, grips: row.grips.filter((grip) => grip.id !== parsed.gripId) },
    ),
    fasteners: model.fasteners.filter((f) => f.source !== fullGripId && f.target !== fullGripId),
  };
}

/** @emoji 🗑️ Removes one tie row by id. */
export function removeFastenerFromModel(model: Model, fastenerId: string): Model | null {
  if (!model.fasteners.some((f) => f.id === fastenerId)) {
    return null;
  }
  return { ...model, fasteners: model.fasteners.filter((f) => f.id !== fastenerId) };
}

/** @emoji 🗑️ Applies a flat canvas structural delete onto the unified {@link Model} model. */
export function applyStructuralDelete2dToModel(model: Model, payload: Puzzle2dStructureDeletePayload): Model | null {
  if (payload.kind === "wire") {
    return null;
  }
  if (payload.kind === "edge") {
    return removeFastenerFromModel(model, payload.id);
  }
  return removePartFromModel(model, payload.id);
}

function pruneSelectionAfterModelEdit(selection: SelectionSnapshot, _prevModel: Model, nextModel: Model): SelectionSnapshot {
  const remainingPartIds = new Set(nextModel.parts.map((part) => part.id));
  const remainingGripIds = new Set(nextModel.parts.flatMap((part) => gripFullIdsForPart(part)));
  return {
    partIds: selection.partIds.filter((id) => remainingPartIds.has(id)),
    gripIds: selection.gripIds.filter((id) => remainingGripIds.has(id)),
  };
}
//#endregion 🔖StructuralDelete

//#region 🔖InspectorPatch
export type Puzzle5dPartPatchField = "partKind" | "text" | "label" | "x" | "y" | "origin";
export type Puzzle5dGripPatchField = "gripKind" | "angle" | "radius" | "position" | "direction" | "label";

function puzzle5dPatchNumber(value: unknown): number | null {
  if (typeof value === "number" && Number.isFinite(value)) {
    return value;
  }
  if (typeof value === "string") {
    const parsed = Number(value);
    if (Number.isFinite(parsed)) {
      return parsed;
    }
  }
  return null;
}

function puzzle5dPatchVec3(value: unknown): [number, number, number] | null {
  if (!Array.isArray(value) || value.length !== 3) {
    return null;
  }
  const x = Number(value[0]);
  const y = Number(value[1]);
  const z = Number(value[2]);
  if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) {
    return null;
  }
  return [x, y, z];
}

function patchPuzzle5dPartRow(part: Part, field: Puzzle5dPartPatchField, value: unknown): Part {
  if (field === "partKind" && typeof value === "string") {
    return { ...part, partKind: value.trim() || undefined };
  }
  if (field === "text" && typeof value === "string" && part["2d"]) {
    return { ...part, "2d": { ...part["2d"], text: value } };
  }
  if (field === "label" && typeof value === "string" && part["3d"]) {
    return { ...part, "3d": { ...part["3d"], label: value } };
  }
  if (field === "x") {
    const next = puzzle5dPatchNumber(value);
    if (next === null || !part["2d"]) {
      return part;
    }
    return { ...part, "2d": { ...part["2d"], x: next } };
  }
  if (field === "y") {
    const next = puzzle5dPatchNumber(value);
    if (next === null || !part["2d"]) {
      return part;
    }
    return { ...part, "2d": { ...part["2d"], y: next } };
  }
  if (field === "origin") {
    const next = puzzle5dPatchVec3(value);
    if (!next || !part["3d"]) {
      return part;
    }
    return { ...part, "3d": { ...part["3d"], origin: next } };
  }
  return part;
}

function patchPuzzle5dGripRow(grip: Grip, field: Puzzle5dGripPatchField, value: unknown): Grip {
  if (field === "gripKind" && typeof value === "string") {
    return { ...grip, gripKind: value.trim() || grip.gripKind };
  }
  if (field === "label" && typeof value === "string" && grip["3d"]) {
    return { ...grip, "3d": { ...grip["3d"], label: value } };
  }
  if (field === "angle") {
    const next = puzzle5dPatchNumber(value);
    if (next === null || !grip["2d"]) {
      return grip;
    }
    return { ...grip, "2d": { ...grip["2d"], angle: next } };
  }
  if (field === "radius") {
    const next = puzzle5dPatchNumber(value);
    if (next === null) {
      return grip;
    }
    if (grip["2d"]) {
      return { ...grip, "2d": { ...grip["2d"], radius: next }, ...(grip["3d"] ? { "3d": { ...grip["3d"], radius: next } } : {}) };
    }
    if (grip["3d"]) {
      return { ...grip, "3d": { ...grip["3d"], radius: next } };
    }
    return grip;
  }
  if (field === "position") {
    const next = puzzle5dPatchVec3(value);
    if (!next || !grip["3d"]) {
      return grip;
    }
    return { ...grip, "3d": { ...grip["3d"], position: next } };
  }
  if (field === "direction") {
    const next = puzzle5dPatchVec3(value);
    if (!next || !grip["3d"]) {
      return grip;
    }
    return { ...grip, "3d": { ...grip["3d"], direction: next } };
  }
  return grip;
}
//#endregion 🔖InspectorPatch

//#region 🔖Store
export interface StoreSnapshot {
  readonly model: Model;
  readonly selection: SelectionSnapshot;
  readonly hoverFocus: HoverFocusSnapshot;
  readonly connectSession: ConnectSession | null;
  readonly cameras: Readonly<Record<string, { readonly "2d": Puzzle2dCameraState; readonly "3d": Puzzle3dFixture["camera"] }>>;
  readonly fillCount: number;
  readonly fillBuildDone: boolean;
}

export class Store {
  private listeners = new Set<() => void>();
  private snapshot: StoreSnapshot;
  private fillSession: Puzzle5dFillSession | null = null;

  constructor(model: Model) {
    this.snapshot = {
      model,
      selection: { partIds: [], gripIds: [] },
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
  read(): Model {
    return this.snapshot.model;
  }

  get2dCamera(instanceId: string): Puzzle2dCameraState {
    return this.snapshot.cameras[instanceId]?.["2d"] ?? this.snapshot.model.camera2d;
  }

  get3dCamera(instanceId: string): Puzzle3dFixture["camera"] {
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

  set3dCamera(instanceId: string, camera: Puzzle3dFixture["camera"]): void {
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
      prev.gripIds.length === selection.gripIds.length &&
      prev.partIds.every((id, index) => id === selection.partIds[index]) &&
      prev.gripIds.every((id, index) => id === selection.gripIds[index])
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
  setHoverFocusFrom2d(fixture2d: Puzzle2dFixture, payload: Pick<Puzzle2dHoverPayload, "id" | "kind">): void {
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

  applyPart2dMove(partId: string, x: number, y: number): void {
    this.applyPart2dMoves([{ id: partId, x, y }]);
  }

  applyPart2dMoves(moves: ReadonlyArray<{ readonly id: string; readonly x: number; readonly y: number }>): void {
    if (moves.length === 0) {
      return;
    }
    const byId = new Map(moves.map((move) => [move.id, move]));
    const parts = this.snapshot.model.parts.map((p) => {
      const move = byId.get(p.id);
      if (!move || !p["2d"]) {
        return p;
      }
      return { ...p, "2d": { ...p["2d"], x: move.x, y: move.y } };
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }

  /** @emoji 🕸️ Batch-updates flat node centers after a WASM force-graph tick. */
  applyPart2dCenters(centers: ReadonlyMap<string, { readonly x: number; readonly y: number }>): void {
    if (centers.size === 0) return;
    const parts = this.snapshot.model.parts.map((p) => {
      const center = centers.get(p.id);
      if (center == null || !p["2d"]) return p;
      return { ...p, "2d": { ...p["2d"], x: center.x, y: center.y } };
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }

  applyPart3dRelocate(partId: string, origin: readonly [number, number, number], orientation: readonly [number, number, number, number]): void {
    const parts = this.snapshot.model.parts.map((p) => {
      if (p.id !== partId || !p["3d"]) return p;
      return { ...p, "3d": { ...p["3d"], origin, orientation } };
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }

  applyFastener(source: string, target: string, fastenerKind?: string): void {
    const fasteners = this.snapshot.model.fasteners;
    if (fasteners.some((f) => f.source === source && f.target === target)) {
      this.setSnapshot({ ...this.snapshot, connectSession: null });
      return;
    }
    const id = crypto.randomUUID();
    const nextFasteners: Fastener[] = [...fasteners, { id, source, target, ...(fastenerKind ? { fastenerKind } : {}) }];
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, fasteners: nextFasteners },
      connectSession: null,
    });
  }

  replaceModel(model: Model): void {
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
  prepareFillSession(sequence: readonly Puzzle5dBrushPlacement[], baseModel?: Model, seed = 0): void {
    this.fillSession = {
      baseModel: cloneModel(baseModel ?? this.snapshot.model),
      sequence,
      seed,
    };
    this.setSnapshot({ ...this.snapshot, fillCount: 0, fillBuildDone: true });
  }

  /** @emoji 🖌️ Commits one unified brush placement and clears any active fill session. */
  applyBrushPlacement(placement: Puzzle5dBrushPlacement): boolean {
    return this.applyBrushPlacementDetailed(placement) !== null;
  }

  /** @emoji 🖌️ Commits one unified brush placement and returns placed ids for flat peer sync. */
  applyBrushPlacementDetailed(placement: Puzzle5dBrushPlacement): { readonly partId: string; readonly fastenerId: string } | null {
    const result = applyBrushPlacementToModel(this.snapshot.model, placement);
    if (result.kind !== "placed") return null;
    this.fillSession = null;
    this.setSnapshot({ ...this.snapshot, model: result.model, connectSession: null, fillCount: 0, fillBuildDone: true });
    return { partId: result.partId, fastenerId: result.fastenerId };
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

  /** @emoji 🗑️ Deletes the current unified selection (parts, grips, ties). */
  applySelectionDelete(): boolean {
    const selection = this.snapshot.selection;
    if (selection.partIds.length === 0 && selection.gripIds.length === 0) {
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
    for (const gripId of selection.gripIds) {
      const next = removeGripFromModel(model, gripId);
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
      selection: { partIds: [], gripIds: [] },
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
      selection: { partIds: [part.id], gripIds: [] },
      connectSession: null,
      fillCount: 0,
      fillBuildDone: true,
    });
    clearFlatPaletteDragSession();
    return part.id;
  }

  /** @emoji 📥 Appends a unified part from a volume palette object drop. */
  applyPaletteObjectDrop(object: Puzzle3dFixtureObject): string | null {
    const part = partFromPaletteObjectDrop(this.snapshot.model, object);
    if (!part) {
      return null;
    }
    this.fillSession = null;
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, parts: [...this.snapshot.model.parts, part] },
      selection: { partIds: [part.id], gripIds: [] },
      connectSession: null,
      fillCount: 0,
      fillBuildDone: true,
    });
    clearVolumePaletteDragSession();
    return part.id;
  }

  /** @emoji 🪣 Syncs fill-count UI state after play commits a fill-prefix model edit. */
  syncFillCount(count: number): void {
    const clamped = Math.max(0, Math.min(PUZZLE_5D_FILL_COUNT_MAX, Math.round(count)));
    if (this.snapshot.fillCount === clamped && this.snapshot.fillBuildDone) {
      return;
    }
    this.setSnapshot({ ...this.snapshot, fillCount: clamped, fillBuildDone: true });
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

  /** @emoji ✏️ Patches editable fields on one or more unified parts. */
  patchParts(partIds: readonly string[], field: Puzzle5dPartPatchField, value: unknown): void {
    if (!partIds.length) {
      return;
    }
    const idSet = new Set(partIds);
    const parts = this.snapshot.model.parts.map((part) => {
      if (!idSet.has(part.id)) {
        return part;
      }
      return patchPuzzle5dPartRow(part, field, value);
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }

  /** @emoji ✏️ Patches editable fields on one or more unified grips. */
  patchGrips(gripFullIds: readonly string[], field: Puzzle5dGripPatchField, value: unknown): void {
    if (!gripFullIds.length) {
      return;
    }
    const idSet = new Set(gripFullIds);
    const parts = this.snapshot.model.parts.map((part) => {
      let changed = false;
      const grips = part.grips.map((grip) => {
        const fullId = gripFullId(part.id, grip.id);
        if (!idSet.has(fullId)) {
          return grip;
        }
        changed = true;
        return patchPuzzle5dGripRow(grip, field, value);
      });
      return changed ? { ...part, grips } : part;
    });
    this.setSnapshot({ ...this.snapshot, model: { ...this.snapshot.model, parts } });
  }
}

export function createStore(model: Model): Store {
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

/** @emoji 📶 Flat-only LOD/grid defaults ({@link PUZZLE_5D_2D_LOD_TIER_COUNT} discrete tiers); do not pass to 3d {@link Puzzle3dPlayCanvas}. */
export const FIVE_D_FLAT_LOD_DEFAULTS = {
  gridFactor: DEFAULT_PUZZLE_2D_GRID_FACTOR,
  gridSnapEnabled: true,
} as const;

/** @emoji 🎛 3d chrome: continuous LOD comes from host `"3d"` props; proximity applies on relocate only. */
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
  readonly suggestionOffset?: number;
  readonly brushOverlapBudget?: number;
  /** 2d surface overrides; LOD uses discrete tiers unless `automaticLod` is set on the canvas. */
  readonly puzzle2d?: Omit<Puzzle2dCanvasProps, "children">;
  /** 3d surface overrides; LOD is continuous/camera-driven — not the flat six-tier scale. */
  readonly puzzle3d?: Omit<Puzzle3dCanvasProps, "children">;
}

import { mergeLiveForceGraphTopologyModel, fiveDApplyLiveForceGraphStep } from "@semio-tech/puzzle-5d-core";

export { mergeLiveForceGraphTopologyModel, fiveDApplyLiveForceGraphStep };

function fiveDLinkSessionFromStore(session: ConnectSession | null): Puzzle2dLinkSessionSnapshot | null {
  if (!session) return null;
  return {
    source: session.sourceGrip,
    endX: session.endX,
    endY: session.endY,
    compatiblePartIds: session.compatiblePartIds,
    ringPartId: session.ringPartId,
    ringGripIds: session.ringGripIds,
  };
}

function fiveDAttractionSessionFromStore(session: ConnectSession | null): AttractionSessionSnapshot | null {
  if (!session) return null;
  return {
    attracting: session.sourceGrip,
    end: session.end3d,
    compatibleObjectIds: session.compatiblePartIds,
    ringObjectId: session.ringPartId,
    ringVortexFullIds: session.ringGripIds,
  };
}

function markers2dFromFixture(props: { readonly fixture: Puzzle2dFixture; readonly lockedIds: ReadonlySet<string>; readonly selectedIds: ReadonlySet<string> }): ReactElement {
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

function puzzle5dModelStructureEpoch(model: Model): number {
  let hash = model.parts.length * 4099 + model.fasteners.length * 97;
  for (const part of model.parts) {
    hash = (hash * 31 + part.id.charCodeAt(0)) | 0;
  }
  for (const fastener of model.fasteners) {
    hash = (hash * 31 + fastener.id.charCodeAt(0)) | 0;
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
    () => ({ ids: [...snap.selection.partIds, ...snap.selection.gripIds] }),
    [snap.selection.partIds, snap.selection.gripIds],
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
    onHover: onHoverHost,
    onPreselect: onPreselectHost,
    selection: selectionHost,
    hoveredId: hoveredIdHost,
    kindHover: kindHoverHost,
    activeTool: activeToolHost,
    suggestionOffset: suggestionOffsetHost,
    graphPortMode: puzzle2dGraphPortMode,
    ...rest2d
  } = extra2d;
  const selectionHostControlled = selectionHost !== undefined;
  const hoverHostControlled = hoveredIdHost !== undefined || kindHoverHost !== undefined || onHoverHost !== undefined;
  const resolvedSelection = selectionHostControlled ? selectionHost : canvasSelection;
  const resolvedHover = hoverHostControlled
    ? { hoveredId: hoveredIdHost ?? null, kindHover: kindHoverHost ?? null }
    : flatHover;
  const activeTool = props.activeTool ?? activeToolHost ?? "select";
  const suggestionOffset = props.suggestionOffset ?? suggestionOffsetHost;
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
  const onHoverHostRef = reactHostPort.useRef(onHoverHost);
  const onPreselectHostRef = reactHostPort.useRef(onPreselectHost);
  const selectionHostControlledRef = reactHostPort.useRef(selectionHostControlled);
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
  onHoverHostRef.current = onHoverHost;
  onPreselectHostRef.current = onPreselectHost;
  selectionHostControlledRef.current = selectionHostControlled;
  const onCamera = reactHostPort.useCallback((c: Puzzle2dCameraState) => {
    storeRef.current.set2dCamera(props.instanceId, c);
  }, [props.instanceId]);
  const onConnect = reactHostPort.useCallback((p: { source: string; target: string }) => {
    storeRef.current.applyFastener(p.source, p.target);
    onConnectHostRef.current?.(p);
  }, []);
  const onIndirectConnect = reactHostPort.useCallback((p: { source: string; target: string }) => {
    storeRef.current.applyFastener(p.source, p.target);
    onIndirectConnectHostRef.current?.(p);
  }, []);
  const onProximityConnect = reactHostPort.useCallback((p: { source: string; target: string }) => {
    storeRef.current.applyFastener(p.source, p.target);
    onProximityConnectHostRef.current?.(p);
  }, []);
  const onDrag = reactHostPort.useCallback((p: { id: string; x: number; y: number }) => {
    liveForceDragNodeIdsRef.current.add(p.id);
    liveForceDragAnchorsRef.current.set(p.id, { x: p.x, y: p.y });
    if (liveForceGraph) {
      storeRef.current.applyPart2dMove(p.id, p.x, p.y);
    }
    onDragHostRef.current?.(p);
  }, [liveForceGraph]);
  const onDragEnd = reactHostPort.useCallback((p: { moves: Array<{ id: string; x: number; y: number }> }) => {
    liveForceDragNodeIdsRef.current.clear();
    liveForceDragAnchorsRef.current.clear();
    storeRef.current.applyPart2dMoves(p.moves);
    onDragEndHostRef.current?.(p);
  }, []);
  const onSelect = reactHostPort.useCallback((s: { ids: readonly string[] }) => {
    if (!selectionHostControlledRef.current) {
      storeRef.current.setSelection({ partIds: s.ids, gripIds: [] });
    }
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
  const onHover = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => {
    if (onHoverHostRef.current) {
      onHoverHostRef.current(payload);
      return;
    }
    onCanvasHover(payload);
  }, [onCanvasHover]);
  const onPreselect = reactHostPort.useCallback((snapshot: { readonly ids: readonly string[]; readonly removedIds: readonly string[] }) => {
    onPreselectHostRef.current?.(snapshot);
  }, []);
  const onLinkCompatibleNodes = reactHostPort.useCallback((p: { source: string | null; nodeIds: readonly string[] }) => {
    if (!p.source) {
      storeRef.current.setConnectSession(null);
      return;
    }
    const prev = storeRef.current.getSnapshot().connectSession;
    storeRef.current.setConnectSession({
      origin: "2d",
      sourceGrip: p.source,
      endX: prev?.endX ?? 0,
      endY: prev?.endY ?? 0,
      end3d: prev?.end3d ?? [0, 0, 0],
      compatiblePartIds: [...p.nodeIds],
      ringPartId: prev?.ringPartId ?? null,
      ringGripIds: prev?.ringGripIds ?? [],
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
      sourceGrip: p.source,
      endX: prev?.endX ?? 0,
      endY: prev?.endY ?? 0,
      end3d: prev?.end3d ?? [0, 0, 0],
      compatiblePartIds: prev?.compatiblePartIds ?? [],
      ringPartId: p.nodeId,
      ringGripIds: [...p.handleIds],
    });
  }, []);
  const onBrushPlace = reactHostPort.useCallback((payload: Puzzle2dBrushPlacePayload) => {
    onBrushPlaceHostRef.current?.(payload);
  }, []);
  const onBrushCandidates = reactHostPort.useCallback((payload: Parameters<NonNullable<Puzzle2dCanvasProps["onBrushCandidates"]>>[0]) => {
    onBrushCandidatesHostRef.current?.(payload);
  }, []);

  const liveForceNodeIds = reactHostPort.useMemo(
    () =>
      fixture2d.nodes
        .map((node) => node.id)
        .sort()
        .join(","),
    [fixture2d.nodes],
  );
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
  }, [fixture2d.nodes.length, liveForceGraph, liveForceNodeIds, props.instanceId]);

  return (
    <div className={FIVE_D_ROOT_CLASS} data-five-d-indirect-active={snap.connectSession ? "true" : "false"} data-five-d-mode="2d" data-five-d-instance={props.instanceId}>
      <Puzzle2dCanvas
        camera={rest2d.camera ?? camera}
        className={["min-h-0 flex-1", props.className, rest2d.className].filter(Boolean).join(" ") || undefined}
        {...FIVE_D_FLAT_LOD_DEFAULTS}
        {...(graphPortMode !== undefined ? { graphPortMode } : {})}
        declarativeSceneDescriptor={declarativeSceneDescriptor}
        sceneAuthoringEpoch={sceneAuthoringEpoch}
        selection={resolvedSelection}
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
        {...(suggestionOffset !== undefined ? { suggestionOffset } : {})}
        {...(onBrushPlaceHost ? { onBrushPlace } : {})}
        {...(onBrushCandidatesHost ? { onBrushCandidates } : {})}
        {...rest2d}
        hoveredId={resolvedHover.hoveredId}
        kindHover={resolvedHover.kindHover}
        onHover={onHover}
        onPreselect={onPreselectHost ? onPreselect : undefined}
      >
        <Puzzle5dFlatRendererRegistrar />
      </Puzzle2dCanvas>
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
    onIndirectConnect: onIndirectConnectHost,
    onProximityConnect: onProximityConnectHost,
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
  const canvasSelection = reactHostPort.useMemo(() => fiveD3dSelectionFromStore(snap.selection), [snap.selection.partIds, snap.selection.gripIds]);
  const volumeHover = reactHostPort.useMemo(() => fiveD3dHoverFromStore(snap.hoverFocus), [snap.hoverFocus]);
  const attractionSession = fiveDAttractionSessionFromStore(snap.connectSession);
  const storeRef = reactHostPort.useRef(store);
  storeRef.current = store;
  const onCamera = reactHostPort.useCallback((c: Puzzle3dFixture["camera"]) => {
    storeRef.current.set3dCamera(props.instanceId, c);
  }, [props.instanceId]);
  const onCanvasHover = reactHostPort.useCallback((payload: Puzzle3dHoverPayload) => {
    storeRef.current.setHoverFocusFrom3d(payload);
  }, []);
  const onBrushPlace = reactHostPort.useCallback((payload: BrushPlacePayload) => {
    if (onBrushPlaceHost) {
      onBrushPlaceHost(payload);
      return;
    }
    puzzle5dCommitVolumeBrushPlacementToPlay(storeRef.current, payload);
  }, [onBrushPlaceHost]);
  return (
    <Puzzle3dPlayCanvas
      fixture={fixture3d}
      setSelectedId={(id) => {
        store.setSelection({ partIds: id ? [id] : [], gripIds: [] });
      }}
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
      onAttractionCompatibleObjects={(p) => {
        if (!p.attracting) {
          store.setConnectSession(null);
          onAttractionCompatibleObjectsHost?.(p);
          return;
        }
        const prev = store.getSnapshot().connectSession;
        store.setConnectSession({
          origin: "3d",
          sourceGrip: p.attracting,
          endX: prev?.endX ?? 0,
          endY: prev?.endY ?? 0,
          end3d: prev?.end3d ?? [0, 0, 0],
          compatiblePartIds: [...p.objectIds],
          ringPartId: prev?.ringPartId ?? null,
          ringGripIds: prev?.ringGripIds ?? [],
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
          sourceGrip: p.attracting,
          endX: prev?.endX ?? 0,
          endY: prev?.endY ?? 0,
          end3d: prev?.end3d ?? [0, 0, 0],
          compatiblePartIds: prev?.compatiblePartIds ?? [],
          ringPartId: p.objectId,
          ringGripIds: [...p.vortexFullIds],
        });
        onAttractionTargetRingHost?.(p);
      }}
      onIndirectConnect={(p) => {
        store.applyFastener(p.attracting, p.attracted);
        onIndirectConnectHost?.(p);
      }}
      onProximityConnect={(p) => {
        store.applyFastener(p.attracting, p.attracted);
        onProximityConnectHost?.(p);
      }}
      onSelect={(s: Puzzle3dSelectionSnapshot) => {
        store.setSelection({ partIds: [...s.objectIds], gripIds: [...s.vortexIds] });
        onSelectHost?.(s);
      }}
      brushActive={brushActive}
      fillActive={fillActive}
      {...(brushPlacementOverlapBudget !== undefined ? { brushPlacementOverlapBudget } : {})}
      onBrushPlace={onBrushPlace}
      {...(onFillMeshesReadyHost ? { onFillMeshesReady: onFillMeshesReadyHost } : {})}
      {...rest3d}
      hoverTarget={volumeHover.hoverTarget}
      kindHover={volumeHover.kindHover}
      onHover={onCanvasHover}
      selection={canvasSelection}
    />
  );
});

const FiveD3d = reactHostPort.memo(function FiveD3d(props: FiveDProps) {
  const snap = useSnapshot();
  const fixture3d = reactHostPort.useMemo(() => project3d(snap.model), [snap.model]);
  const fixtureRevision = snap.model.parts.length + snap.model.fasteners.length * 4099;
  return (
    <div className={FIVE_D_ROOT_CLASS} data-five-d-indirect-active={snap.connectSession ? "true" : "false"} data-five-d-mode="3d" data-five-d-instance={props.instanceId}>
      <Puzzle3dPartStateProvider
        fixture={fixture3d}
        fixtureRevision={fixtureRevision}
        onConnect={props.puzzle3d?.onConnect}
        onRelocate={props.puzzle3d?.onRelocate}
      >
        <FiveD3dInner {...props} />
      </Puzzle3dPartStateProvider>
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
/** @emoji 🎯 Local grip template on a {@link PartKind} (flat angle + volume pose in part-local space). */
export interface PartKindGrip {
  readonly gripKind: string;
  readonly "2d"?: Grip2dAspect;
  readonly "3d"?: Grip3dAspect;
}

/** @emoji 🟠 Part-kind catalog row for unified puzzle 5d fixtures. */
export interface PartKind {
  color?: string;
  defaultGripKind?: string;
  defaultShapeProps?: Record<string, unknown>;
  grips?: readonly PartKindGrip[];
  icon?: string;
  id: string;
  label?: string;
  meshUrl?: string;
  name: string;
  scale?: number | readonly [number, number, number];
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
  specificity?: "general" | "part" | "grip" | "fastener" | "rope";
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

function partKindGripsFrom2dHandles(handles: readonly { readonly handleKind: string; readonly angle: number; readonly radius?: number }[]): PartKindGrip[] {
  return handles.map((handle) => ({
    gripKind: handle.handleKind,
    "2d": {
      angle: handle.angle,
      gripKind: handle.handleKind,
      ...(handle.radius !== undefined ? { radius: handle.radius } : {}),
    },
  }));
}

function partKindGripsFrom3dVortices(
  vortices: readonly {
    readonly vortexKind?: string;
    readonly position: readonly [number, number, number];
    readonly direction?: readonly [number, number, number];
    readonly radius?: number;
    readonly label?: string;
  }[],
): PartKindGrip[] {
  return vortices.map((vortex) => ({
    gripKind: vortex.vortexKind ?? BUILTIN_PORT_HANDLE_KIND,
    "3d": {
      position: vortex.position,
      ...(vortex.direction !== undefined ? { direction: vortex.direction } : {}),
      ...(vortex.radius !== undefined ? { radius: vortex.radius } : {}),
      ...(vortex.label !== undefined ? { label: vortex.label } : {}),
    },
  }));
}

function mergePartKindGrips(flat: readonly PartKindGrip[] | undefined, volume: readonly PartKindGrip[] | undefined): PartKindGrip[] | undefined {
  if (!flat?.length && !volume?.length) {
    return undefined;
  }
  const count = Math.max(flat?.length ?? 0, volume?.length ?? 0);
  const out: PartKindGrip[] = [];
  for (let index = 0; index < count; index += 1) {
    const flatGrip = flat?.[index];
    const volumeGrip = volume?.[index];
    const gripKind = volumeGrip?.gripKind ?? flatGrip?.gripKind ?? BUILTIN_PORT_HANDLE_KIND;
    out.push({
      gripKind,
      ...(flatGrip?.["2d"] ? { "2d": flatGrip["2d"] } : {}),
      ...(volumeGrip?.["3d"] ? { "3d": volumeGrip["3d"] } : {}),
    });
  }
  return out.length ? out : undefined;
}

function mergePartKinds(flat: PartKind | undefined, volume: PartKind | undefined): PartKind | undefined {
  if (!flat && !volume) {
    return undefined;
  }
  const id = flat?.id ?? volume!.id;
  const grips = mergePartKindGrips(flat?.grips, volume?.grips);
  return {
    id,
    name: flat?.name ?? volume!.name ?? id,
    ...(flat?.label ?? volume?.label ? { label: flat?.label ?? volume?.label } : {}),
    ...(flat?.color ?? volume?.color ? { color: flat?.color ?? volume?.color } : {}),
    ...(flat?.defaultGripKind ?? volume?.defaultGripKind ? { defaultGripKind: flat?.defaultGripKind ?? volume?.defaultGripKind } : {}),
    ...(flat?.defaultShapeProps ?? volume?.defaultShapeProps ? { defaultShapeProps: flat?.defaultShapeProps ?? volume?.defaultShapeProps } : {}),
    ...(flat?.icon ?? volume?.icon ? { icon: flat?.icon ?? volume?.icon } : {}),
    ...(flat?.shape ?? volume?.shape ? { shape: flat?.shape ?? volume?.shape } : {}),
    ...(flat?.stroke ?? volume?.stroke ? { stroke: flat?.stroke ?? volume?.stroke } : {}),
    ...(flat?.meshUrl ?? volume?.meshUrl ? { meshUrl: flat?.meshUrl ?? volume?.meshUrl } : {}),
    ...(flat?.scale ?? volume?.scale ? { scale: flat?.scale ?? volume?.scale } : {}),
    ...(grips ? { grips } : {}),
  };
}

function dedupeCatalogRowsById<T extends { id: string }>(rows: readonly T[]): T[] {
  const map = new Map<string, T>();
  for (const row of rows) {
    if (row.id) {
      map.set(row.id, row);
    }
  }
  return [...map.values()];
}

function mergeKindCatalogBundles(flat: Puzzle2dKindCatalogBundle | undefined, volume: Puzzle3dKindCatalogBundle | undefined): KindCatalogBundle | undefined {
  const flatParts = flat?.nodes?.map((row) => partKindFrom2dNode(row)) ?? [];
  const volumeParts = volume?.objects?.map((row) => partKindFrom3dObject(row)) ?? [];
  const partIds = new Set([...flatParts.map((row) => row.id), ...volumeParts.map((row) => row.id)]);
  const flatById = new Map(flatParts.map((row) => [row.id, row]));
  const volumeById = new Map(volumeParts.map((row) => [row.id, row]));
  const parts: PartKind[] = [];
  for (const id of partIds) {
    const merged = mergePartKinds(flatById.get(id), volumeById.get(id));
    if (merged) {
      parts.push(merged);
    }
  }
  parts.sort((left, right) => left.id.localeCompare(right.id));
  const grips = dedupeCatalogRowsById([
    ...(flat?.handles?.map((row) => gripKindFrom2dHandle(row)) ?? []),
    ...(volume?.vortices?.map((row) => gripKindFrom3dVortex(row)) ?? []),
  ]);
  const fasteners = dedupeCatalogRowsById([
    ...(flat?.edges?.map((row) => fastenerKindFrom2dEdge(row)) ?? []),
    ...(volume?.attractions?.map((row) => fastenerKindFrom3dAttraction(row)) ?? []),
  ]);
  const ropes = dedupeCatalogRowsById([
    ...(flat?.wires?.map((row) => ropeKindFrom2dWire(row)) ?? []),
    ...(volume?.cables?.map((row) => ropeKindFrom3dCable(row)) ?? []),
  ]);
  if (!parts.length && !grips.length && !fasteners.length && !ropes.length) {
    return undefined;
  }
  return {
    ...(parts.length ? { parts } : {}),
    ...(grips.length ? { grips } : {}),
    ...(fasteners.length ? { fasteners } : {}),
    ...(ropes.length ? { ropes } : {}),
  };
}

function partKindFrom2dNode(row: Puzzle2dNodeKind): PartKind {
  const rowEx = row as Puzzle2dNodeKind & { readonly meshUrl?: string };
  return {
    id: row.id,
    name: row.name,
    ...(row.color !== undefined ? { color: row.color } : {}),
    ...(row.defaultHandleKind !== undefined ? { defaultGripKind: row.defaultHandleKind } : {}),
    ...(row.defaultShapeProps !== undefined ? { defaultShapeProps: row.defaultShapeProps } : {}),
    ...(row.icon !== undefined ? { icon: row.icon } : {}),
    ...(row.shape !== undefined ? { shape: row.shape } : {}),
    ...(row.stroke !== undefined ? { stroke: row.stroke } : {}),
    ...(rowEx.meshUrl !== undefined ? { meshUrl: rowEx.meshUrl } : {}),
    ...(row.handles?.length ? { grips: partKindGripsFrom2dHandles(row.handles) } : {}),
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
    ...(row.meshUrl !== undefined ? { meshUrl: row.meshUrl } : {}),
    ...(row.scale !== undefined ? { scale: row.scale } : {}),
    ...(row.vortices?.length ? { grips: partKindGripsFrom3dVortices(row.vortices) } : {}),
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
  const flat: Puzzle2dKindCatalogBundle = {};
  const volume: Puzzle3dKindCatalogBundle = {};
  if (Array.isArray(box.nodes)) flat.nodes = box.nodes as Puzzle2dNodeKind[];
  if (Array.isArray(box.handles)) flat.handles = box.handles as Puzzle2dHandleKind[];
  if (Array.isArray(box.edges)) flat.edges = box.edges as Puzzle2dEdgeKind[];
  if (Array.isArray(box.wires)) flat.wires = box.wires as Puzzle2dWireKind[];
  if (Array.isArray(box.objects)) volume.objects = box.objects as Puzzle3dPartKind[];
  if (Array.isArray(box.vortices)) volume.vortices = box.vortices as Puzzle3dGripKind[];
  if (Array.isArray(box.attractions)) volume.attractions = box.attractions as Puzzle3dFastenerKind[];
  if (Array.isArray(box.cables)) volume.cables = box.cables as Puzzle3dRopeKind[];
  if (flat.nodes || flat.handles || flat.edges || flat.wires) {
    if (volume.objects || volume.vortices || volume.attractions || volume.cables) {
      return mergeKindCatalogBundles(flat, volume);
    }
    const out: KindCatalogBundle = {};
    if (flat.nodes) out.parts = flat.nodes.map((row) => partKindFrom2dNode(row));
    if (flat.handles) out.grips = flat.handles.map((row) => gripKindFrom2dHandle(row));
    if (flat.edges) out.fasteners = flat.edges.map((row) => fastenerKindFrom2dEdge(row));
    if (flat.wires) out.ropes = flat.wires.map((row) => ropeKindFrom2dWire(row));
    return out;
  }
  if (volume.objects || volume.vortices || volume.attractions || volume.cables) {
    const out: KindCatalogBundle = {};
    if (volume.objects) out.parts = volume.objects.map((row) => partKindFrom3dObject(row));
    if (volume.vortices) out.grips = volume.vortices.map((row) => gripKindFrom3dVortex(row));
    if (volume.attractions) out.fasteners = volume.attractions.map((row) => fastenerKindFrom3dAttraction(row));
    if (volume.cables) out.ropes = volume.cables.map((row) => ropeKindFrom3dCable(row));
    return out;
  }
  return undefined;
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
      ...(part.meshUrl !== undefined ? { meshUrl: part.meshUrl } : {}),
      ...(part.grips?.length
        ? {
            handles: part.grips
              .filter((grip) => grip["2d"])
              .map((grip) => ({
                handleKind: grip["2d"]!.gripKind,
                angle: grip["2d"]!.angle,
                ...(grip["2d"]!.radius !== undefined ? { radius: grip["2d"]!.radius } : {}),
              })),
          }
        : {}),
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
      ...(part.meshUrl !== undefined ? { meshUrl: part.meshUrl } : {}),
      ...(part.scale !== undefined ? { scale: part.scale } : {}),
      ...(part.grips?.length
        ? {
            vortices: part.grips
              .filter((grip) => grip["3d"])
              .map((grip) => ({
                vortexKind: grip.gripKind,
                position: grip["3d"]!.position,
                ...(grip["3d"]!.direction !== undefined ? { direction: grip["3d"]!.direction } : {}),
                ...(grip["3d"]!.radius !== undefined ? { radius: grip["3d"]!.radius } : {}),
                ...(grip["3d"]!.label !== undefined ? { label: grip["3d"]!.label } : {}),
              })),
          }
        : {}),
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
      entry.specificity === "general" || entry.specificity === "part" || entry.specificity === "grip" || entry.specificity === "fastener" || entry.specificity === "rope"
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

export function puzzle5dDefaultManifestCatalogBundle(): KindCatalogBundle | undefined {
  return normalizeKindCatalogBundle(mergeManifestCatalogBundles(puzzle2d_defaultManifestCatalogBundle(), puzzle3d_defaultManifestCatalogBundle()));
}

export function kindCatalogsFromMetas(inp: { readonly meta2d: Record<string, unknown> | undefined; readonly meta3d: Record<string, unknown> | undefined }): KindCatalogBundle | undefined {
  const fromFlat = kindCatalogFrom2dMeta(inp.meta2d);
  const fromVolume = kindCatalogFrom3dMeta(inp.meta3d);
  if (fromFlat && fromVolume) {
    return mergeKindCatalogBundles(fromFlat, fromVolume);
  }
  if (fromFlat) {
    return normalizeKindCatalogBundle(fromFlat) ?? mergeKindCatalogBundles(fromFlat, undefined);
  }
  if (fromVolume) {
    return normalizeKindCatalogBundle(fromVolume);
  }
  return puzzle5dDefaultManifestCatalogBundle();
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
  fixture: Puzzle2dFixture,
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

/** @emoji 🧩 Builds "2d" host markers from a puzzle 2d fixture (same static shape walk as puzzle 2d play). */
export function flatMarkersFromFixture(props: {
  readonly fixture: Puzzle2dFixture;
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

export { DEFAULT_PUZZLE_2D_GRID_FACTOR, getPuzzle2dLodScale, blockedVortexFullIdsFromAttractions, parsePuzzle2dFixture, parseFixture };

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("connectGestureCrossSurface", () => {
    it("only indirect syncs across 2d and 3d", () => {
      expect(connectGestureCrossSurface("indirect")).toBe(true);
      expect(connectGestureCrossSurface("direct")).toBe(false);
      expect(connectGestureCrossSurface("proximity")).toBe(false);
    });
  });

  describe("puzzle 5d 3d mesh paint", () => {
    it("delegates volume mesh colors to puzzle 3d", () => {
      expect(PUZZLE_5D_3D_MESH_PAINT).toBe(PUZZLE_3D_MESH_PAINT);
      expect(PUZZLE_5D_3D_DEFAULT_MESH_STYLE).toBe(PUZZLE_3D_DEFAULT_MESH_STYLE);
      expect(puzzle5d3dMeshStyleColors).toBe(meshStyleColors);
      for (const style of ["neutral", "hovered", "selected", "highlighted", "disabled"] as const) {
        expect(puzzle5d3dMeshStyleColors(style)).toBe(meshStyleColors(style));
      }
      expect(PUZZLE_5D_3D_MESH_PAINT.style.neutral.mesh).toBe("var(--panel)");
    });
  });

  describe("parseModel", () => {
    it("accepts unified puzzle 5d model", () => {
      const t = parseModel({
        schema: "puzzle.5d",
        domain: "architecture",
        camera2d: { x: 0, y: 0, zoom: 1 },
        camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        parts: [],
        fasteners: [],
        label: "x",
      });
      expect(t?.schema).toBe("puzzle.5d");
      expect(t?.label).toBe("x");
    });
  });
  describe("compose5d", () => {
    it("merges 2d nodes and 3d objects by id", () => {
      const fixture2d: Puzzle2dFixture = {
        schema: "puzzle.2d.fixture",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [{ id: "p1", shape: "circle", x: 1, y: 2, radius: 10, handles: [{ id: "p1:h", angle: 0, handleKind: "port" }] }],
        edges: [],
      };
      const fixture3d: Puzzle3dFixture = {
        schema: "puzzle.3d.fixture",
        domain: "architecture",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        objects: [{ id: "p1", meshUrl: "m.glb", origin: [0, 0, 0], vortices: [{ id: "p1:h", position: [0, 0, 0] }] }],
        attractions: [],
      };
      const t = compose5d(fixture2d, fixture3d);
      expect(t.parts.some((p) => p.id === "p1" && p["2d"] && p["3d"])).toBe(true);
    });

    it("preserves edge kinds as tie kinds for wires-style fixtures", () => {
      const fixture2d: Puzzle2dFixture = {
        schema: "puzzle.2d.fixture",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [
          { id: "a", shape: "circle", x: 0, y: 0, radius: 10, handles: [] },
          { id: "b", shape: "circle", x: 100, y: 0, radius: 10, handles: [] },
        ],
        edges: [{ id: "e1", source: "a", target: "b", edgeKind: "wires.owns" }],
      };
      const fixture3d: Puzzle3dFixture = {
        schema: "puzzle.3d.fixture",
        domain: "architecture",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        objects: [],
        attractions: [],
      };
      const model = compose5d(fixture2d, fixture3d);
      expect(model.fasteners[0]?.fastenerKind).toBe("wires.owns");
      expect(project2d(model).edges[0]?.edgeKind).toBe("wires.owns");
    });
  });

  describe("flatten5d", () => {
    it("resolves linked piece absolute pose from local grip geometry", () => {
      const fixture2d: Puzzle2dFixture = {
        schema: "puzzle.2d.fixture",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [
          {
            id: "fixed-root",
            shape: "circle",
            x: 1,
            y: 2,
            radius: 10,
            handles: [{ id: "fixed-root:conn-a", angle: 0, handleKind: "port", t: 0 }],
          },
          {
            id: "linked-child",
            shape: "circle",
            x: 0,
            y: 0,
            radius: 10,
            handles: [{ id: "linked-child:conn-a", angle: 0, handleKind: "port", t: 0 }],
          },
        ],
        edges: [
          {
            id: "link-1",
            source: "fixed-root:conn-a",
            target: "linked-child:conn-a",
            u: 0.5,
            v: 0.25,
          },
        ],
      };
      const fixture3d: Puzzle3dFixture = {
        schema: "puzzle.3d.fixture",
        domain: "architecture",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        objects: [
          {
            id: "fixed-root",
            meshUrl: "m.glb",
            origin: [1, 2, 3],
            orientation: [0, 0, 0, 1],
            vortices: [{ id: "fixed-root:conn-a", position: [0, 0, 0], direction: [0, 0, 1] }],
          },
          {
            id: "linked-child",
            meshUrl: "m.glb",
            origin: [0, 0, 0],
            vortices: [{ id: "linked-child:conn-a", position: [0, 0, 0], direction: [0, 0, 1] }],
          },
        ],
        attractions: [
          {
            id: "link-1",
            attracting: "fixed-root:conn-a",
            attracted: "linked-child:conn-a",
            u: 0.5,
            v: 0.25,
          },
        ],
      };
      const flattened = flatten5d(compose5d(fixture2d, fixture3d));
      const child = flattened.parts.find((part) => part.id === "linked-child");
      expect(child?.["3d"]?.origin?.[0]).toBeCloseTo(1, 2);
      expect(child?.["2d"]?.x).toBeCloseTo(1.5, 2);
    });
  });

  describe("Store applyFastener", () => {
    it("ignores duplicate source-target pairs", () => {
      const store = createStore({
        schema: "puzzle.5d",
        domain: "architecture",
        camera2d: { x: 0, y: 0, zoom: 1 },
        camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        parts: [],
        fasteners: [{ id: "t1", source: "a", target: "b" }],
      });
      store.applyFastener("a", "b");
      expect(store.getSnapshot().model.fasteners).toHaveLength(1);
    });
  });

  describe("puzzle 3d projection", () => {
    it("projects parts with 3d aspects to a 3d fixture", () => {
      const model = compose5d(
        {
          schema: "puzzle.2d.fixture",
          camera: { x: 0, y: 0, zoom: 1 },
          nodes: [{ id: "p1", shape: "circle", x: 0, y: 0, radius: 10, handles: [] }],
          edges: [],
        },
        {
          schema: "puzzle.3d.fixture",
          domain: "architecture",
          camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
          objects: [{ id: "p1", meshUrl: "m.glb", origin: [1, 2, 3], vortices: [] }],
          attractions: [],
        },
      );
      const fixture3d = project3d(model);
      expect(fixture3d.objects).toHaveLength(1);
      expect(fixture3d.objects[0]?.origin).toEqual([1, 2, 3]);
      expect(fixture3d.objects[0]?.style).toBeUndefined();
    });

    it("round-trips native 3d appearance fields through 5d", () => {
      const fixture2d: Puzzle2dFixture = {
        schema: "puzzle.2d.fixture",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [],
        edges: [],
      };
      const fixture3d: Puzzle3dFixture = {
        schema: "puzzle.3d.fixture",
        domain: "architecture",
        camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        objects: [
          {
            id: "p1",
            objectKind: "kind-a",
            meshUrl: "/mesh/a.glb",
            meshByLod: [{ lod: 1000, url: "/mesh/a-lod.glb" }],
            style: "selected",
            origin: [1, 2, 3],
            locked: true,
            vortices: [
              {
                id: "p1:h",
                vortexKind: "port-a",
                position: [0, 0, 0],
                vortexMeshUrl: "/mesh/port.glb",
                vortexMeshByLod: [{ lod: 500, url: "/mesh/port-lod.glb" }],
                hidden: true,
              },
            ],
          },
        ],
        attractions: [],
        references: [
          {
            id: "ref-a",
            source: { url: "/reference/a.png", mediaKind: "image" },
            origin: [0, 0, 0],
            widthWorld: 10,
            locked: true,
          },
        ],
        targetVolumes: [{ id: "volume-a", origin: [0, 0, 0], scale: [1, 2, 3], color: "#123456", opacity: 0.5 }],
      };
      const projected = project3d(compose5d(fixture2d, fixture3d));
      expect(projected.objects[0]?.meshByLod).toEqual(fixture3d.objects[0]?.meshByLod);
      expect(projected.objects[0]?.style).toBe("selected");
      expect(projected.objects[0]?.locked).toBe(true);
      expect(projected.objects[0]?.vortices[0]?.vortexMeshUrl).toBe("/mesh/port.glb");
      expect(projected.objects[0]?.vortices[0]?.vortexMeshByLod).toEqual(fixture3d.objects[0]?.vortices[0]?.vortexMeshByLod);
      expect(projected.objects[0]?.vortices[0]?.hidden).toBe(true);
      expect(projected.references).toEqual(fixture3d.references);
      expect(projected.targetVolumes).toEqual(fixture3d.targetVolumes);
    });

    it("keeps unset concrete forest mesh style unset like native 3d", async () => {
      const [{ default: fixture2dRaw }, { default: fixture3dRaw }] = await Promise.all([
        import("../../2d/example/concrete-forest.2d.json"),
        import("../../3d/example/concrete-forest.3d.json"),
      ]);
      const fixture2d = parsePuzzle2dFixture(fixture2dRaw as unknown);
      const fixture3d = parseFixture(fixture3dRaw as unknown);
      expect(fixture2d).toBeTruthy();
      expect(fixture3d).toBeTruthy();
      expect(fixture3d!.objects.every((object) => object.style === undefined)).toBe(true);
      expect(project3d(compose5d(fixture2d!, fixture3d!)).objects.every((object) => object.style === undefined)).toBe(true);
    });
  });

  describe("project2d", () => {
    it("round-trips part centers", () => {
      const model = compose5d(
        {
          schema: "puzzle.2d.fixture",
          camera: { x: 0, y: 0, zoom: 1 },
          nodes: [{ id: "n", shape: "circle", x: 5, y: 6, radius: 4, handles: [] }],
          edges: [],
        },
        {
          schema: "puzzle.3d.fixture",
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

  describe("kindCatalog suggestion parity", () => {
    const flatMeta = {
      kindCatalogs: {
        nodes: [
          {
            id: "Hexagonal Cut Concrete Forest Left",
            name: "Hexagonal Cut Concrete Forest Left",
            meshUrl: "/mesh/hexagonal-cut-concrete-forest-left.glb",
            handles: [{ handleKind: "b-l", angle: -1.5707963267948966, radius: 0.36 }],
          },
        ],
        handles: [{ id: "b-l", name: "b-l", color: "hsl(206 52% 48%)" }],
      },
    };
    const volumeMeta = {
      kindCatalogs: {
        objects: [
          {
            id: "Hexagonal Cut Concrete Forest Left",
            name: "Hexagonal Cut Concrete Forest Left",
            meshUrl: "/mesh/hexagonal-cut-concrete-forest-left.glb",
            vortices: [{ vortexKind: "b-l", position: [4.05, 4.67, 3] as const, direction: [0, 1, 0] as const, radius: 0.36 }],
          },
        ],
        vortices: [{ id: "b-l", name: "b-l", color: "hsl(206 52% 48%)" }],
      },
    };

    it("kindCatalogsFromMetas merges flat and volume grip templates on one part kind", () => {
      const bundle = kindCatalogsFromMetas({ meta2d: flatMeta, meta3d: volumeMeta });
      const part = bundle?.parts?.find((row) => row.id === "Hexagonal Cut Concrete Forest Left");
      expect(part?.meshUrl).toBe("/mesh/hexagonal-cut-concrete-forest-left.glb");
      expect(part?.grips?.[0]?.["2d"]?.angle).toBeCloseTo(-1.5707963267948966);
      expect(part?.grips?.[0]?.["3d"]?.position).toEqual([4.05, 4.67, 3]);
    });

    it("project3dKindCatalogs emits meshUrl and vortex templates", () => {
      const bundle = kindCatalogsFromMetas({ meta2d: flatMeta, meta3d: volumeMeta });
      const cat3d = project3dKindCatalogs(bundle);
      const object = cat3d?.objects?.find((row) => row.id === "Hexagonal Cut Concrete Forest Left");
      expect(object?.meshUrl).toBe("/mesh/hexagonal-cut-concrete-forest-left.glb");
      expect(object?.vortices?.[0]?.vortexKind).toBe("b-l");
      expect(object?.vortices?.[0]?.position).toEqual([4.05, 4.67, 3]);
    });

    it("project2dKindCatalogs emits handle templates", () => {
      const bundle = kindCatalogsFromMetas({ meta2d: flatMeta, meta3d: volumeMeta });
      const cat2d = project2dKindCatalogs(bundle);
      const node = cat2d?.nodes?.find((row) => row.id === "Hexagonal Cut Concrete Forest Left");
      expect(node?.handles?.[0]?.handleKind).toBe("b-l");
      expect(node?.handles?.[0]?.angle).toBeCloseTo(-1.5707963267948966);
    });

    it("projected 3d catalog yields brush-compatible candidates", () => {
      const bundle = kindCatalogsFromMetas({ meta2d: flatMeta, meta3d: volumeMeta });
      const cat3d = project3dKindCatalogs(bundle);
      const candidates = brushCompatibleCandidates(
        { objectId: "seed-left-001", objectKind: "Hexagonal Cut Concrete Forest Left", vortexKind: "b-l" },
        cat3d,
        [{ source: "b-l", target: "b-l", bidirectional: true }],
      );
      expect(candidates.length).toBeGreaterThan(0);
      expect(candidates[0]?.objectKindId).toBe("Hexagonal Cut Concrete Forest Left");
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
      expect(fiveD3dSelectionFromStore({ partIds: ["tower-a", "tower-b"], gripIds: ["tower-a:port"] })).toEqual({
        objectIds: ["tower-a", "tower-b"],
        vortexIds: ["tower-a:port"],
        attractionIds: [],
      });
    });
  });
  describe("paired hover focus", () => {
    const fixture2d: Puzzle2dFixture = {
      schema: "puzzle.2d.fixture",
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
        schema: "puzzle.5d",
        domain: "architecture",
        camera2d: { x: 0, y: 0, zoom: 1 },
        camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        parts: [],
        fasteners: [],
      });
      store.setHoverFocusFrom2d(fixture2d, { id: "p1", kind: null });
      expect(store.getSnapshot().hoverFocus).toEqual({ instance: { kind: "part", id: "p1" }, kindHover: null });
      store.setHoverFocusFrom3d({ hoverTarget: null, kindHover: null });
      expect(store.getSnapshot().hoverFocus).toEqual({ instance: null, kindHover: null });
    });
  });
  describe("brush paired sync", () => {
    const brushSyncModel: Model = {
      schema: "puzzle.5d",
      domain: "architecture",
      camera2d: { x: 0, y: 0, zoom: 1 },
      camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
      parts: [
        {
          id: "p1",
          partKind: "tower",
          grips: [{ id: "g0", gripKind: "port", "2d": { angle: 0 }, "3d": { radius: 0.4 } }],
        },
      ],
      fasteners: [],
    };

    it("puzzle5dSyncBrushPreviewFromFlat forwards semantic candidate to volume brush", () => {
      const synced: Array<{ targetGripFullId: string; candidate: { objectKindId: string; sourceVortexIndex: number } }> = [];
      const prev = puzzle3dBrushPairedSyncRef.current;
      puzzle3dBrushPairedSyncRef.current = {
        syncFromFlat: (args) => {
          synced.push({ targetGripFullId: args.targetGripFullId, candidate: args.candidate });
        },
        clear: () => {},
      };
      try {
        puzzle5dSyncBrushPreviewFromFlat(
          {
            sourceHandleId: "p1:g0",
            candidateIndex: 1,
            candidates: [
              { nodeKind: "kind-a", targetHandleIndex: 0 },
              { nodeKind: "kind-b", targetHandleIndex: 2 },
            ],
          } as Puzzle2dBrushSessionSnapshot,
          brushSyncModel,
        );
        expect(synced).toEqual([{ targetGripFullId: "p1:g0", candidate: { objectKindId: "kind-b", sourceVortexIndex: 2 } }]);
      } finally {
        puzzle3dBrushPairedSyncRef.current = prev;
      }
    });

    it("puzzle5dSyncBrushPreviewFromFlat clears volume brush when flat session ends", () => {
      let cleared = false;
      const prev = puzzle3dBrushPairedSyncRef.current;
      puzzle3dBrushPairedSyncRef.current = {
        syncFromFlat: () => {},
        clear: () => {
          cleared = true;
        },
      };
      try {
        puzzle5dSyncBrushPreviewFromFlat(null, brushSyncModel);
        expect(cleared).toBe(false);
      } finally {
        puzzle3dBrushPairedSyncRef.current = prev;
      }
    });

    it("puzzle5dSyncBrushPreviewFromFlat skips when volume preview already mirrors flat session", () => {
      let synced = false;
      const prev = puzzle3dBrushPairedSyncRef.current;
      const prevPreview = puzzle3dBrushUiStore.getSnapshot().preview;
      puzzle3dBrushPairedSyncRef.current = {
        syncFromFlat: () => {
          synced = true;
        },
        clear: () => {},
      };
      puzzle3dBrushUiStore.setSnapshot({
        ...puzzle3dBrushUiStore.getSnapshot(),
        preview: {
          targetVortexFullId: "p1:g0",
          objectKindId: "kind-b",
          sourceVortexIndex: 2,
          meshUrl: "",
          origin: [0, 0, 0],
          orientation: [0, 0, 0, 1],
        },
      });
      try {
        puzzle5dSyncBrushPreviewFromFlat(
          {
            sourceHandleId: "p1:g0",
            candidateIndex: 1,
            candidates: [
              { nodeKind: "kind-a", targetHandleIndex: 0 },
              { nodeKind: "kind-b", targetHandleIndex: 2 },
            ],
          } as Puzzle2dBrushSessionSnapshot,
          brushSyncModel,
        );
        expect(synced).toBe(false);
      } finally {
        puzzle3dBrushPairedSyncRef.current = prev;
        puzzle3dBrushUiStore.setSnapshot({ ...puzzle3dBrushUiStore.getSnapshot(), preview: prevPreview });
      }
    });

    it("puzzle5dSyncBrushPreviewFromVolume preserves flat-originated suggestions while volume preview is still null", () => {
      let cancelCount = 0;
      const cancel = () => {
        cancelCount += 1;
      };
      const prevFlatRenderer = puzzle5dFlatRendererRef.current;
      puzzle5dFlatRendererRef.current = { brushCancelSlot: cancel } as unknown as Puzzle2dRenderer;
      const prev = puzzle3dBrushPairedSyncRef.current;
      puzzle3dBrushPairedSyncRef.current = { syncFromFlat: () => {}, clear: () => {} };
      try {
        puzzle5dSyncBrushPreviewFromFlat(
          {
            sourceHandleId: "p1:g0",
            candidateIndex: 0,
            candidates: [{ nodeKind: "kind-a", targetHandleIndex: 0 }],
            preview: {
              edge: { sourceHandleId: "p1:g0", targetHandleIndex: 0 },
              node: { nodeKind: "kind-a", radius: 20, shape: "circle", x: 80, y: 0 },
            },
            suggestionsActive: true,
          } as Puzzle2dBrushSessionSnapshot,
          brushSyncModel,
        );
        puzzle5dSyncBrushPreviewFromVolume(null, undefined);
        expect(cancelCount).toBe(0);
      } finally {
        puzzle5dFlatRendererRef.current = prevFlatRenderer;
        puzzle3dBrushPairedSyncRef.current = prev;
        puzzle5dSyncBrushPreviewFromFlat(null, brushSyncModel);
        puzzle2dSyncBrushSessionToAllAuthoringPeers(null, undefined, { force: true });
      }
    });

    it("puzzle5dSyncBrushPreviewFromVolume cancels flat slot when volume-originated preview clears", () => {
      let cancelCount = 0;
      const cancel = () => {
        cancelCount += 1;
      };
      const prevFlatRenderer = puzzle5dFlatRendererRef.current;
      puzzle5dFlatRendererRef.current = {
        brushCancelSlot: cancel,
        brushOpenSlot: () => {},
        setBrushCandidateIndex: () => {},
      } as unknown as Puzzle2dRenderer;
      const volumePreview = {
        targetVortexFullId: "p1:g0",
        objectKindId: "kind-b",
        sourceVortexIndex: 2,
        meshUrl: "",
        origin: [0, 0, 0] as [number, number, number],
        orientation: [0, 0, 0, 1] as [number, number, number, number],
      };
      try {
        puzzle5dSyncBrushPreviewFromVolume(volumePreview, undefined);
        puzzle2dSyncBrushSessionToAllAuthoringPeers(
          {
            sourceHandleId: "p1:g0",
            candidateIndex: 0,
            candidates: [{ nodeKind: "kind-b", targetHandleIndex: 2 }],
            preview: null,
          },
          undefined,
          { force: true },
        );
        puzzle5dSyncBrushPreviewFromVolume(null, undefined);
        expect(cancelCount).toBe(1);
      } finally {
        puzzle5dFlatRendererRef.current = prevFlatRenderer;
        puzzle5dSyncBrushPreviewFromFlat(null, brushSyncModel);
        puzzle2dSyncBrushSessionToAllAuthoringPeers(null, undefined, { force: true });
      }
    });
  });
  describe("nodeCenterFromTopLeft", () => {
    it("offsets by half frame", () => {
      expect(nodeCenterFromTopLeft({ x: 10, y: 20 }, { width: 40, height: 60 })).toEqual({ x: 30, y: 50 });
    });
  });
  describe("flatApplyFixtureCentersToTopLeft", () => {
    it("converts centers to top-left using frame size", () => {
      const fixture: Puzzle2dFixture = {
        schema: "puzzle.2d.fixture",
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
          schema: "puzzle.2d.fixture",
          camera: { x: 0, y: 0, zoom: 1 },
          nodes: [
            { id: "p1", shape: "circle", x: 0, y: 0, radius: 10, handles: [{ id: "p1:h", angle: 0, handleKind: "port" }] },
            { id: "p2", shape: "circle", x: 10, y: 0, radius: 10, handles: [] },
          ],
          edges: [{ id: "e1", source: "p1:h", target: "p2" }],
        },
        {
          schema: "puzzle.3d.fixture",
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
      expect(store.read().fasteners).toHaveLength(0);
    });
  });

  describe("applyBrushPlacementToModel", () => {
    const brushHostModel = (): Model => ({
      schema: "puzzle.5d",
      domain: "architecture",
      camera2d: { x: 0, y: 0, zoom: 1 },
      camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
      parts: [
        {
          id: "host",
          partKind: "Host",
          "2d": { x: 0, y: 0, shape: "rectangle", width: 40, height: 40 },
          "3d": { origin: [0, 0, 0], meshUrl: "/mesh/host.glb", orientation: [0, 0, 0, 1] },
          grips: [
            {
              id: "port",
              gripKind: "port",
              "2d": { angle: 0, gripKind: "port" },
              "3d": { position: [1, 0, 0], direction: [-1, 0, 0] },
            },
          ],
        },
        {
          id: "peer",
          partKind: "Capsule",
          "2d": { x: 100, y: 0, shape: "rectangle", width: 40, height: 40 },
          "3d": { origin: [10, 0, 0], meshUrl: "/mesh/capsule.glb", orientation: [0, 0, 0, 1] },
          grips: [
            {
              id: "mate",
              gripKind: "port",
              "2d": { angle: Math.PI, gripKind: "port" },
              "3d": { position: [-1, 0, 0], direction: [1, 0, 0] },
            },
          ],
        },
      ],
      fasteners: [],
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
          scale: 1.5,
          attractionId: "accepted-preview-fastener",
        }),
      );
      expect(result.kind).toBe("placed");
      if (result.kind !== "placed") return;
      const placed = result.model.parts.find((part) => part.id === result.partId);
      expect(placed?.["2d"]).toBeTruthy();
      expect(placed?.["3d"]).toBeTruthy();
      expect(placed?.["3d"]?.origin).toEqual([2, 0, 0]);
      expect(placed?.["3d"]?.orientation).toEqual([0, 0, 0, 1]);
      expect(placed?.["3d"]?.scale).toBe(1.5);
      expect(placed?.["2d"]?.y).toBeLessThan(-DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX);
      expect(Math.abs(placed?.["2d"]?.x ?? 0)).toBeLessThan(1e-6);
      expect(result.model.fasteners.some((f) => f.id === "accepted-preview-fastener")).toBe(true);
      expect(result.model.fasteners.some((f) => f.source.endsWith(":mate") || f.source.includes("Capsule"))).toBe(true);
    });

    it("puzzle5dCommitVolumeBrushPlacementToPlay uses paired flat session preview coordinates", () => {
      const store = createStore(brushHostModel());
      const previewX = 12;
      const previewY = -96;
      puzzle2dSyncBrushSessionToAllAuthoringPeers({
        candidateIndex: 0,
        candidates: [{ nodeKind: "Capsule", targetHandleIndex: 0 }],
        preview: {
          edge: { sourceHandleId: "host:port", targetHandleIndex: 0 },
          node: {
            handles: [{ angle: Math.PI, handleKind: "port" }],
            nodeKind: "Capsule",
            radius: DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX / 2,
            shape: "circle",
            x: previewX,
            y: previewY,
          },
        },
        sourceHandleId: "host:port",
        suggestionsActive: true,
      });
      const placed = puzzle5dCommitVolumeBrushPlacementToPlay(store, {
        targetVortexFullId: "host:port",
        objectKindId: "Capsule",
        sourceVortexIndex: 0,
        origin: [2, 0, 0],
        orientation: [0, 0, 0, 1],
      });
      expect(placed).toBe(true);
      const fixture = project2d(store.read());
      const node = fixture.nodes.find((row) => row.x === previewX && row.y === previewY);
      expect(node?.nodeKind).toBe("Capsule");
    });

    it("preserves unified catalog grip radii on accepted volume suggestions", () => {
      const model: Model = {
        ...brushHostModel(),
        kindCatalogs: {
          parts: [
            {
              id: "Capsule",
              name: "Capsule",
              meshUrl: "/mesh/capsule.glb",
              grips: [
                {
                  gripKind: "b-l",
                  "2d": { angle: Math.PI, gripKind: "b-l", radius: 0.36 },
                  "3d": { position: [-1, 0, 0], direction: [1, 0, 0], radius: 0.36 },
                },
              ],
            },
          ],
        },
      };
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
      expect(placed?.grips[0]?.["2d"]?.radius).toBe(0.36);
      expect(placed?.grips[0]?.["3d"]?.radius).toBe(0.36);
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
      expect(placed?.["2d"]?.x).toBe(80);
      expect(placed?.["3d"]?.meshUrl).toBe("/mesh/capsule.glb");
      expect(result.model.fasteners[0]?.target).toBe(`${result.partId}:mate`);
    });

    it("puzzle5dCommitBrushPlacementToPlay updates unified model and flat projection", () => {
      const store = createStore(brushHostModel());
      const placed = puzzle5dCommitBrushPlacementToPlay(store, {
        nodeKind: "Capsule",
        sourceHandleId: "host:port",
        targetHandleIndex: 0,
        x: 80,
        y: 0,
        handles: [{ angle: Math.PI, handleKind: "port" }],
      });
      expect(placed).toBe(true);
      const fixture = project2d(store.read());
      expect(fixture.nodes).toHaveLength(3);
      expect(fixture.edges).toHaveLength(1);
      expect(fixture.nodes.some((node) => node.x === 80 && node.y === 0)).toBe(true);
      const newPartId = fixture.nodes.find((n) => n.x === 80 && n.y === 0)?.id;
      expect(fixture.edges[0]?.target).toBe(`${newPartId}:mate`);
    });

    it("keeps repeated brush grip kinds unique across flat and volume projections", () => {
      const model: Model = {
        ...brushHostModel(),
        kindCatalogs: {
          parts: [
            {
              id: "Capsule",
              name: "Capsule",
              meshUrl: "/mesh/capsule.glb",
              grips: [
                {
                  gripKind: "b-l",
                  "2d": { angle: Math.PI, gripKind: "b-l" },
                  "3d": { position: [-1, 0, 0], direction: [1, 0, 0] },
                },
                {
                  gripKind: "b-l",
                  "2d": { angle: 0, gripKind: "b-l" },
                  "3d": { position: [1, 0, 0], direction: [-1, 0, 0] },
                },
              ],
            },
          ],
        },
      };
      const result = applyBrushPlacementToModel(
        model,
        puzzle5dBrushPlacementFromFlat({
          nodeKind: "Capsule",
          shape: "circle",
          sourceHandleId: "host:port",
          targetHandleIndex: 0,
          x: 80,
          y: 0,
          handles: [
            { angle: Math.PI, handleKind: "b-l" },
            { angle: 0, handleKind: "b-l" },
          ],
        }),
      );
      expect(result.kind).toBe("placed");
      if (result.kind !== "placed") return;
      const fixture2d = project2d(result.model);
      const fixture3d = project3d(result.model);
      const placed2d = fixture2d.nodes.find((node) => node.id === result.partId);
      const placed3d = fixture3d.objects.find((object) => object.id === result.partId);
      expect(new Set(placed2d?.handles.map((handle) => handle.id)).size).toBe(placed2d?.handles.length);
      expect(new Set(placed3d?.vortices.map((vortex) => vortex.id)).size).toBe(placed3d?.vortices.length);
    });
  });

  describe("Store fill session", () => {
    it("applies fill prefix with unified parts", () => {
      const store = createStore({
        schema: "puzzle.5d",
        domain: "architecture",
        camera2d: { x: 0, y: 0, zoom: 1 },
        camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
        parts: [
          {
            id: "host",
            partKind: "Host",
            "2d": { x: 0, y: 0, shape: "rectangle", width: 40, height: 40 },
            "3d": { origin: [0, 0, 0], meshUrl: "/mesh/host.glb", orientation: [0, 0, 0, 1] },
            grips: [
              {
                id: "port",
                gripKind: "port",
                "2d": { angle: 0, gripKind: "port" },
                "3d": { position: [1, 0, 0], direction: [-1, 0, 0] },
              },
            ],
          },
          {
            id: "peer",
            partKind: "Capsule",
            "2d": { x: 100, y: 0, shape: "rectangle", width: 40, height: 40 },
            "3d": { origin: [10, 0, 0], meshUrl: "/mesh/capsule.glb", orientation: [0, 0, 0, 1] },
            grips: [
              {
                id: "mate",
                gripKind: "port",
                "2d": { angle: Math.PI, gripKind: "port" },
                "3d": { position: [-1, 0, 0], direction: [1, 0, 0] },
              },
            ],
          },
        ],
        fasteners: [],
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
      expect(added?.["2d"]).toBeTruthy();
      expect(added?.["3d"]).toBeTruthy();
    });
  });

  describe("mergeLiveForceGraphTopologyModel", () => {
    it("preserves settled flat centers and camera while placing new nodes near their parent", () => {
      const existing = compose5d(
        {
          schema: "puzzle.2d.fixture",
          camera: { x: 10, y: 20, zoom: 0.8 },
          nodes: [
            { id: "parent", shape: "circle", x: 0, y: 0, radius: 20, handles: [] },
            { id: "child-a", shape: "circle", x: 4, y: 0, radius: 20, handles: [] },
          ],
          edges: [{ id: "e1", source: "parent", target: "child-a" }],
        },
        {
          schema: "puzzle.3d.fixture",
          domain: "architecture",
          camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
          objects: [],
          attractions: [],
        },
      );
      const settled = prepareTopologyModel(existing);
      const incoming = prepareTopologyModel(
        compose5d(
          {
            schema: "puzzle.2d.fixture",
            camera: { x: 0, y: 0, zoom: 1 },
            nodes: [
              { id: "parent", shape: "circle", x: 200, y: 200, radius: 20, handles: [] },
              { id: "child-a", shape: "circle", x: 300, y: 300, radius: 20, handles: [] },
              { id: "child-b", shape: "circle", x: 0, y: 0, radius: 20, handles: [] },
            ],
            edges: [
              { id: "e1", source: "parent", target: "child-a" },
              { id: "e2", source: "parent", target: "child-b" },
            ],
          },
          {
            schema: "puzzle.3d.fixture",
            domain: "architecture",
            camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
            objects: [],
            attractions: [],
          },
        ),
      );
      const merged = mergeLiveForceGraphTopologyModel(incoming, settled);
      const parent = merged.parts.find((part) => part.id === "parent")?.["2d"];
      const childA = merged.parts.find((part) => part.id === "child-a")?.["2d"];
      const childB = merged.parts.find((part) => part.id === "child-b")?.["2d"];
      const settledParent = settled.parts.find((part) => part.id === "parent")?.["2d"];
      const settledChildA = settled.parts.find((part) => part.id === "child-a")?.["2d"];
      expect(parent?.x).toBe(settledParent?.x);
      expect(parent?.y).toBe(settledParent?.y);
      expect(childA?.x).toBe(settledChildA?.x);
      expect(childA?.y).toBe(settledChildA?.y);
      expect(childB?.x).toBeCloseTo((parent?.x ?? 0), 0);
      expect(childB?.y).toBeLessThan(parent?.y ?? 0);
      expect(merged.camera2d).toEqual(settled.camera2d);
    });
  });

  describe("fiveDApplyLiveForceGraphStep", () => {
    it("updates flat node centers for linked normal-graph nodes", () => {
      const store = createStore(
        compose5d(
          {
            schema: "puzzle.2d.fixture",
            camera: { x: 0, y: 0, zoom: 1 },
            nodes: [
              { id: "a", shape: "circle", x: 0, y: 0, radius: 20, handles: [] },
              { id: "b", shape: "circle", x: 4, y: 0, radius: 20, handles: [] },
            ],
            edges: [{ id: "e1", source: "a", target: "b" }],
          },
          {
            schema: "puzzle.3d.fixture",
            domain: "architecture",
            camera: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
            objects: [],
            attractions: [],
          },
        ),
      );
      fiveDApplyLiveForceGraphStep(store, "kit:kit:wires");
      const a = store.read().parts.find((part) => part.id === "a")?.["2d"];
      const b = store.read().parts.find((part) => part.id === "b")?.["2d"];
      expect(a?.x).toBeDefined();
      expect(b?.x).toBeDefined();
      expect(Math.abs((a?.x ?? 0) - (b?.x ?? 0))).toBeGreaterThan(8);
    });
  });
}
