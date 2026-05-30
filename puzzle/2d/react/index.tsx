// #region 🧲Header
/** @emoji 📋 `@puzzle/2d/react` — WASM board renderer + React canvas (depends only on `@ui/react`). */
// #endregion 🧲Header

// #region 🔌Adapters
import { ContextMenuController, reactHostPort, type ContextMenuItem } from "@ui/react";
import React from "react";
import Reconciler from "react-reconciler";
import { ContinuousEventPriority, DefaultEventPriority, DiscreteEventPriority, LegacyRoot, NoEventPriority } from "react-reconciler/constants";
// #endregion 🔌Adapters

type BoardListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

class BoardEventBindingController {
  private readonly cleanups: Array<() => void> = [];

  listen(target: BoardListenerTarget | null | undefined, kind: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void {
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

// #region 🔖GpuWasmBridge
import initBoardWasm, { boardComputeEdgeBezier, boardHandlePositionCircle, boardHandlePositionRectangle, boardRedrawHandlesFixtureJson, boardRedrawLayoutFixtureJson, BoardSession, initSync } from "../rs/pkg/puzzle_2d.js";

if (typeof process !== "undefined" && process.env.VITEST === "true") {
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../rs/pkg/puzzle_2d_bg.wasm");
  initSync({ module: readFileSync(wasmPath) });
} else {
  await initBoardWasm();
}

/** @emoji 🌐 Idempotent: resolves after the wasm-bindgen `web` target has finished instantiating. */
export async function ensureElementsBoardWasmLoaded(): Promise<void> {
  await initBoardWasm();
}

export { BoardSession };
// #endregion 🔖GpuWasmBridge

//#region 🔖Kinds
export type BoardObjectKind = "node" | "handle" | "edge" | "wire";
export type RenderMode = "main-thread" | "worker-offscreen" | "headless-test";
export type BoardSelectionMethod = "lasso" | "rectangle";
export type BoardSelectionMode = "additive" | "default" | "invertive" | "subtractive";

/** @emoji 🎯 Which graph kinds participate in rectangle/lasso selection and hit picking. */
export interface BoardSelectionTargets {
  edges: boolean;
  handles: boolean;
  nodes: boolean;
}

/** @emoji 🎯 Default: nodes, edges, and handles all participate (matches prior `nodes&edges`). */
export const BOARD_SELECTION_TARGETS_DEFAULT: BoardSelectionTargets = {
  edges: true,
  handles: true,
  nodes: true,
};
/** @emoji 🎯 Specificity axis for a {@link KindCompatEntry} (weakest → strongest: general < node < edge < handle < wire). */
export type SemanticSpecificity = "edge" | "general" | "handle" | "node" | "wire";

/** @emoji 🔗 One allowed directed pair between semantic kind ids; `important` bypasses specificity ordering among matches. */
export interface KindCompatEntry {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: SemanticSpecificity;
}

/** @emoji 🎨 Handle-kind catalog row: defaults for new handles plus optional gesture {@link WireKind} id. */
export interface HandleKind {
  color: string;
  defaultWireKind?: string;
  id: string;
  name: string;
}

/** @emoji 🧵 Wire-kind catalog row for link gestures and default promoted {@link EdgeKind} id. */
export interface WireKind {
  defaultEdgeKind?: string;
  id: string;
  name: string;
}

/** @emoji 🟠 Node-kind catalog row (defaults for instances; richer fields reserved for future paint). */
export interface NodeKind {
  color?: string;
  defaultHandleKind?: string;
  defaultShapeProps?: Record<string, unknown>;
  icon?: string;
  id: string;
  name: string;
  shape?: "circle" | "rectangle";
  stroke?: string;
}

/** @emoji 🪢 Edge-kind catalog row (defaults for instances; richer fields reserved for future stroke). */
export interface EdgeKind {
  color?: string;
  defaultShapeProps?: Record<string, unknown>;
  id: string;
  name: string;
  pattern?: string;
  shape?: "bezier" | "line";
  stroke?: string;
}

/** @emoji 📚 Central WASM+host registries for semantic board kinds (omit slices to leave prior catalog entries untouched when pushing partial updates is not supported — always send full merged bundle from callers). */
export interface KindCatalogBundle {
  edges?: readonly EdgeKind[];
  handles?: readonly HandleKind[];
  nodes?: readonly NodeKind[];
  wires?: readonly WireKind[];
}

/** @emoji 🧷 Builtin handle kind id used when fixture JSON omits `handleKind` (aligned with Rust `parse_fixture_v1`). */
export const BUILTIN_PORT_HANDLE_KIND = "port";

/** @emoji 🧷 Default wire kind id resolved for link gestures when a handle catalog omits `defaultWireKind`. */
export const BUILTIN_LINK_WIRE_KIND = "wire.link";

/** @emoji 🧷 Default edge kind id assigned to WASM-created link edges when a wire catalog omits `defaultEdgeKind`. */
export const BUILTIN_LINK_EDGE_KIND = "edge.link";

/** @emoji 🎨 Default WASM handle catalog so `port` resolves a fill color and gesture wire kind without host configuration. */
export const DEFAULT_HANDLE_KIND_CATALOG: readonly HandleKind[] = [
  {
    color: "#94a3b8",
    defaultWireKind: BUILTIN_LINK_WIRE_KIND,
    id: BUILTIN_PORT_HANDLE_KIND,
    name: "Port",
  },
];

/** @emoji 🎨 Default wire catalog entry paired with {@link DEFAULT_HANDLE_KIND_CATALOG}. */
export const DEFAULT_WIRE_KIND_CATALOG: readonly WireKind[] = [{ defaultEdgeKind: BUILTIN_LINK_EDGE_KIND, id: BUILTIN_LINK_WIRE_KIND, name: "Link wire" }];

/** @emoji 🎨 Default edge catalog entry paired with {@link DEFAULT_WIRE_KIND_CATALOG}. */
export const DEFAULT_EDGE_KIND_CATALOG: readonly EdgeKind[] = [{ id: BUILTIN_LINK_EDGE_KIND, name: "Link edge" }];

/** @emoji 📚 Default {@link KindCatalogBundle} for {@link BoardCanvas} when callers omit `kindCatalogs`. */
export const DEFAULT_KIND_CATALOG_BUNDLE: KindCatalogBundle = {
  edges: DEFAULT_EDGE_KIND_CATALOG,
  handles: DEFAULT_HANDLE_KIND_CATALOG,
  wires: DEFAULT_WIRE_KIND_CATALOG,
};

/** @emoji 🔀 Merges catalog slices by stable row `id` (patch rows replace same-id base rows); empty patch slices keep the base slice. */
export function mergeKindCatalogBundleByRowId(base: KindCatalogBundle, patch: KindCatalogBundle): KindCatalogBundle {
  function mergedSlice<T extends { id: string }>(baseSlice: readonly T[] | undefined, patchSlice: readonly T[] | undefined): readonly T[] | undefined {
    if (patchSlice === undefined) {
      return baseSlice;
    }
    if (patchSlice.length === 0) {
      return baseSlice;
    }
    const byId = new Map<string, T>();
    for (const row of baseSlice ?? []) {
      byId.set(row.id, row);
    }
    for (const row of patchSlice) {
      byId.set(row.id, row);
    }
    return [...byId.values()].sort((a, b) => a.id.localeCompare(b.id));
  }
  return {
    edges: mergedSlice(base.edges, patch.edges) ?? base.edges,
    handles: mergedSlice(base.handles, patch.handles) ?? base.handles,
    nodes: mergedSlice(base.nodes, patch.nodes) ?? base.nodes,
    wires: mergedSlice(base.wires, patch.wires) ?? base.wires,
  };
}

/** @emoji 🗂️ Returns `meta.kindCatalogs` from raw board fixture JSON when present (nodes/handles slices only). */
export function fixtureMetaKindCatalogBundle(raw: unknown): KindCatalogBundle | undefined {
  if (!raw || typeof raw !== "object") {
    return undefined;
  }
  const root = raw as Record<string, unknown>;
  const meta = root.meta;
  if (!meta || typeof meta !== "object") {
    return undefined;
  }
  const kc = (meta as Record<string, unknown>).kindCatalogs;
  if (!kc || typeof kc !== "object") {
    return undefined;
  }
  const box = kc as Record<string, unknown>;
  const nodesRaw = box.nodes;
  const handlesRaw = box.handles;
  const out: KindCatalogBundle = {};
  if (Array.isArray(nodesRaw)) {
    out.nodes = nodesRaw as readonly NodeKind[];
  }
  if (Array.isArray(handlesRaw)) {
    out.handles = handlesRaw as readonly HandleKind[];
  }
  if (out.nodes === undefined && out.handles === undefined) {
    return undefined;
  }
  return out;
}

/** @emoji 🔗 Returns `meta.kindCompatibility` from raw board fixture JSON when present. */
export function boardFixtureMetaKindCompatibility(raw: unknown): readonly KindCompatEntry[] | undefined {
  if (!raw || typeof raw !== "object") return undefined;
  const meta = (raw as Record<string, unknown>).meta;
  if (!meta || typeof meta !== "object") return undefined;
  const entries = (meta as Record<string, unknown>).kindCompatibility;
  if (!Array.isArray(entries)) return undefined;
  return entries as readonly KindCompatEntry[];
}

function serializeKindCatalogBundle(bundle: KindCatalogBundle): string {
  const handles = (bundle.handles ?? [])
    .map((e) => {
      const id = String(e.id ?? "").trim();
      const color = String(e.color ?? "").trim();
      const name = String(e.name ?? "").trim() || id;
      const dw = e.defaultWireKind != null ? String(e.defaultWireKind).trim() : "";
      if (id === "" || color === "") {
        return null;
      }
      return {
        id,
        name,
        color,
        ...(dw !== "" ? { defaultWireKind: dw } : {}),
      };
    })
    .filter((x): x is NonNullable<typeof x> => x !== null);
  const wires = (bundle.wires ?? [])
    .map((e) => {
      const id = String(e.id ?? "").trim();
      const name = String(e.name ?? "").trim() || id;
      const de = e.defaultEdgeKind != null ? String(e.defaultEdgeKind).trim() : "";
      if (id === "") {
        return null;
      }
      return { id, name, ...(de !== "" ? { defaultEdgeKind: de } : {}) };
    })
    .filter((x): x is NonNullable<typeof x> => x !== null);
  const nodes = (bundle.nodes ?? [])
    .map((e) => {
      const id = String(e.id ?? "").trim();
      const name = String(e.name ?? "").trim() || id;
      if (id === "") {
        return null;
      }
      const row: Record<string, unknown> = { id, name };
      if (e.shape) {
        row.shape = e.shape;
      }
      if (e.color != null && String(e.color).trim() !== "") {
        row.color = String(e.color).trim();
      }
      if (e.stroke != null && String(e.stroke).trim() !== "") {
        row.stroke = String(e.stroke).trim();
      }
      if (e.icon != null && String(e.icon).trim() !== "") {
        row.icon = String(e.icon).trim();
      }
      if (e.defaultShapeProps) {
        row.defaultShapeProps = e.defaultShapeProps;
      }
      if (e.defaultHandleKind != null && String(e.defaultHandleKind).trim() !== "") {
        row.defaultHandleKind = String(e.defaultHandleKind).trim();
      }
      return row;
    })
    .filter((x): x is NonNullable<typeof x> => x !== null);
  const edges = (bundle.edges ?? [])
    .map((e) => {
      const id = String(e.id ?? "").trim();
      const name = String(e.name ?? "").trim() || id;
      if (id === "") {
        return null;
      }
      const row: Record<string, unknown> = { id, name };
      if (e.shape) {
        row.shape = e.shape;
      }
      if (e.color != null && String(e.color).trim() !== "") {
        row.color = String(e.color).trim();
      }
      if (e.stroke != null && String(e.stroke).trim() !== "") {
        row.stroke = String(e.stroke).trim();
      }
      if (e.pattern != null && String(e.pattern).trim() !== "") {
        row.pattern = String(e.pattern).trim();
      }
      if (e.defaultShapeProps) {
        row.defaultShapeProps = e.defaultShapeProps;
      }
      return row;
    })
    .filter((x): x is NonNullable<typeof x> => x !== null);
  return JSON.stringify({ edgeKinds: edges, handleKinds: handles, nodeKinds: nodes, wireKinds: wires });
}
/** 🧱 World-space clip tiling for the Vello WASM canvas (`world-clip`) or monolithic encoding (`none`). */
export type WorldRasterTilingKind = "none" | "world-clip";

export interface Point {
  x: number;
  y: number;
}

export interface CameraState {
  x: number;
  y: number;
  zoom: number;
}

export interface BoardSelectionSnapshot {
  ids: string[];
}

/** @emoji 👁️ Rectangle/lasso drag preview ids plus anchor ids leaving the committed selection during the gesture. */
export interface BoardPreselectSnapshot {
  ids: string[];
  removedIds: string[];
}

/** @emoji 👁️ Empty area-select preview (no ids highlighted, none marked removed). */
export const BOARD_PRESELECT_EMPTY: BoardPreselectSnapshot = { ids: [], removedIds: [] };

/** @emoji 🎯 Committed selection vs area-select preview chrome (`preselect∖selection` selected, `removedIds` highlighted). */
export function boardElementInteractionChrome(selectionIds: Iterable<string>, preselection: BoardPreselectSnapshot): { highlightedIds: Set<string>; selectedIds: Set<string> } {
  const selection = new Set(selectionIds);
  if (preselection.ids.length === 0) {
    return { selectedIds: selection, highlightedIds: new Set() };
  }
  const selectedIds = new Set(preselection.ids.filter((id) => !selection.has(id)));
  const highlightedIds = new Set(preselection.removedIds);
  return { selectedIds, highlightedIds };
}

/** @emoji 🎨 Resolves headless / fallback style key from interaction chrome flags (selected beats highlighted). */
export function boardObjectChromeStyleKey(base: "edge" | "handle" | "node", object: BoardObject): string {
  if (object.selected) {
    return `${base}.selected`;
  }
  if (object.highlighted) {
    return `${base}.highlighted`;
  }
  return base;
}

/** @emoji 🎨 Style key from committed selection / preselect only (not scene object flags). */
export function boardInteractionChromeStyleKey(base: "edge" | "handle" | "node", id: string, chrome: { highlightedIds: Set<string>; selectedIds: Set<string> }): string {
  if (chrome.selectedIds.has(id)) {
    return `${base}.selected`;
  }
  if (chrome.highlightedIds.has(id)) {
    return `${base}.highlighted`;
  }
  return base;
}

export interface BoardSelectionOptions {
  method?: BoardSelectionMethod;
  mode?: BoardSelectionMode;
  targets?: Partial<BoardSelectionTargets>;
}

/** @emoji 🎯 Resolved selection options passed to WASM (`targets` fully specified). */
export type ResolvedBoardSelectionOptions = {
  method: BoardSelectionMethod;
  mode: BoardSelectionMode;
  targets: BoardSelectionTargets;
};

export interface BoardStyle {
  fill?: string;
  stroke?: string;
  strokeWidth?: number;
}

export interface FrameState {
  camera: CameraState;
  renderer: BoardRenderer;
  selection: BoardSelectionSnapshot;
}

export interface CubicBezierCurve {
  p0: Point;
  p1: Point;
  p2: Point;
  p3: Point;
}

/** @emoji 📄 Handle record inside {@link BoardFixtureV1}; optional `radius` overrides default world-space hit/draw size. */
export interface BoardFixtureHandleV1 {
  angle: number;
  /** @emoji 🔗 Required after {@link parseBoardFixtureV1}; JSON may omit it and receive {@link BUILTIN_PORT_HANDLE_KIND}. */
  handleKind: string;
  id: string;
  /** @emoji 🎨 Optional CSS `#rgb` / `#rrggbb` / `#rrggbbaa` overriding the catalog color for this handle. */
  color?: string;
  /** @emoji 🏷️ Optional WASM detail LOD icon string (`typst:` / `$…`, `emoji:`, `data:` / raster data URLs, catalog id, or inline SVG). */
  iconKind?: string;
  radius?: number;
}

/** @emoji 📄 Circle node: {@link BoardFixtureCircleNodeV1.x}/{@link BoardFixtureCircleNodeV1.y} are the disk center in layout space; handle {@link BoardFixtureHandleV1.angle} aims at the connected neighbor (radians). */
export interface BoardFixtureCircleNodeV1 {
  cad?: { x: number; y: number; z: number } | null;
  handles: BoardFixtureHandleV1[];
  id: string;
  /** @emoji 🌳 When true, directed edges {@link Edge.source}→{@link Edge.target} form parent→child links; subtree membership derives from this root. */
  root?: boolean;
  radius: number;
  shape?: "circle";
  /** @emoji 🏷️ On-canvas caption (kit piece `name`, e.g. `cs_sl1_…`). */
  text?: string;
  /** @emoji 🏷️ WASM detail LOD icon string (`typst:` / `$…`, `emoji:`, `data:` / raster data URLs, catalog id, or inline SVG). */
  iconKind?: string;
  /** @emoji 🧩 Optional semantic node kind id (e.g. kit `nodeKind` string). */
  nodeKind?: string;
  /** @emoji 📏 Optional: scale overlay text to fit inside the node; drawn at node center to avoid jitter. */
  textAutofit?: boolean;
  /** @emoji 🧭 Caption alignment inside the node box when not using autofit. */
  textAlignment?: BoardNodeTextAlignment;
  /** @emoji 🔤 Optional CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Optional caption size in layout px when not using autofit. */
  textFontSize?: number;
  x: number;
  y: number;
}

/** @emoji 📄 Axis-aligned rectangle: center (x,y) in layout space, full width/height (not half-extents); handle `angle` is **0 at north** (top center), **CCW** in `[0,2π)` (`π/4` NW corner, `π/2` west, …); circles use **east-zero** polar `atan2(dy,dx)`. */
export interface BoardFixtureRectangleNodeV1 {
  cad?: { x: number; y: number; z: number } | null;
  handles: BoardFixtureHandleV1[];
  height: number;
  id: string;
  /** @emoji 🌳 When true, directed edges {@link Edge.source}→{@link Edge.target} form parent→child links; subtree membership derives from this root. */
  root?: boolean;
  shape: "rectangle";
  /** @emoji 🏷️ On-canvas caption (kit piece `name`, e.g. `cs_sl1_…`). */
  text?: string;
  /** @emoji 🏷️ WASM detail LOD icon string (`typst:` / `$…`, `emoji:`, `data:` / raster data URLs, catalog id, or inline SVG). */
  iconKind?: string;
  /** @emoji 🧩 Optional semantic node kind id (e.g. kit `nodeKind` string). */
  nodeKind?: string;
  /** @emoji 📏 Optional: scale overlay text to fit inside the node; drawn at node center to avoid jitter. */
  textAutofit?: boolean;
  /** @emoji 🧭 Caption alignment inside the node box when not using autofit. */
  textAlignment?: BoardNodeTextAlignment;
  /** @emoji 🔤 Optional CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Optional caption size in layout px when not using autofit. */
  textFontSize?: number;
  width: number;
  x: number;
  y: number;
}

/** @emoji 📄 Node record inside {@link BoardFixtureV1} (circle or rectangle body). */
export type BoardFixtureNodeV1 = BoardFixtureCircleNodeV1 | BoardFixtureRectangleNodeV1;

/** @emoji 📄 Edge record inside {@link BoardFixtureV1}. */
export interface BoardFixtureEdgeV1 {
  id: string;
  source: string;
  target: string;
}

/** @emoji 📄 Parsed `puzzle.2d.fixture/v1` JSON for declarative board scenes. */
export interface BoardFixtureV1 {
  camera: CameraState;
  edges: BoardFixtureEdgeV1[];
  meta?: Record<string, unknown>;
  nodes: BoardFixtureNodeV1[];
  schema: string;
}

// #region 🏷️IconSelectorMode

/** @emoji 🎛️ Board `iconKind` editor tab (`math` = `typst:` / leading `$`, `data` = data URLs, `emoji` = `emoji:` …, `vector` = catalog / inline SVG). */
export type ElementsBoardIconSelectorMode = "data" | "emoji" | "math" | "vector";

function stripLegacyImageDataPrefixForBoardIcon(raw: string): string {
  const t = raw.trim();
  return t.startsWith("image:") ? t.slice("image:".length).trim() : t;
}

function isRasterDataUrlPayloadForBoardIcon(s: string): boolean {
  const u = s.trim().toLowerCase();
  return u.startsWith("data:image/png;base64,") || u.startsWith("data:image/jpeg;base64,") || u.startsWith("data:image/jpg;base64,");
}

function looksLikeAsciiCatalogishVectorStemForBoardIcon(s: string): boolean {
  const t = s.trim();
  if (t === "") {
    return false;
  }
  if (!/^[\w.-]+$/.test(t)) {
    return false;
  }
  return /[.-_]/.test(t) || t.length > 48;
}

/** @emoji 🧭 Picks a {@link ElementsBoardIconSelectorMode} tab for a stored board icon string (align with `board_resolve_icon_kind` in `elements/client/lib/board/rs/lib.rs`). */
export function classifyElementsBoardIconSelectorMode(raw: string): ElementsBoardIconSelectorMode {
  const t = raw.trim();
  if (t === "") {
    return "math";
  }
  if (t.startsWith("typst:") || t.startsWith("$")) {
    return "math";
  }
  if (t.startsWith("emoji:")) {
    return "emoji";
  }
  const lower = t.toLowerCase();
  if (lower.startsWith("data:") || isRasterDataUrlPayloadForBoardIcon(stripLegacyImageDataPrefixForBoardIcon(t))) {
    return "data";
  }
  if (lower.startsWith("<?xml") || lower.includes("<svg")) {
    return "vector";
  }
  if (looksLikeAsciiCatalogishVectorStemForBoardIcon(t)) {
    return "vector";
  }
  return "emoji";
}

// #endregion 🏷️IconSelectorMode

/** @emoji 🕸️ JSON options for {@link layoutBoardFixtureForceGraph} (camelCase; matches Rust `ForceGraphLayoutOptions` / dimforge `nalgebra` spring layout). */
export interface BoardForceGraphLayoutOptions {
  centerX?: number;
  centerY?: number;
  gravity?: number;
  idealEdgeLength?: number;
  iterations?: number;
  maxSpeed?: number;
  randomSeed?: number;
  repulsionStrength?: number;
  springStrength?: number;
  timeStep?: number;
  velocityDamping?: number;
}

/** @emoji 🌳 Rank growth axis for hierarchical redraw (`downwards` | `upwards` | `right` | `left`). */
export type BoardHierarchicalTreeDirectionKind = "downwards" | "left" | "right" | "upwards";

/** @emoji 🕸️ WASM redraw dispatcher mode for {@link layoutBoardFixtureRedrawNodes}. */
export type BoardRedrawModeKind = "force-graph" | "hierarchical-tree";

/** @emoji 🧩 Options for {@link layoutBoardFixtureRedrawNodes} (camelCase; mirrors Rust `RedrawFixtureOptions`). */
export interface BoardRedrawLayoutOptions {
  mode: BoardRedrawModeKind;
  redrawHandlesAfter: boolean;
  centerX?: number;
  centerY?: number;
  randomSeed?: number;
  forceGraph?: BoardForceGraphLayoutOptions;
  hierarchicalTree?: {
    direction: BoardHierarchicalTreeDirectionKind;
    layerSpacing: number;
    siblingGap: number;
  };
}

/** @emoji 🕸️ Runs WASM force-directed layout on fixture node centers (edges via handle ids); uses dimforge `nalgebra` in Rust. */
export function layoutBoardFixtureForceGraph(fixture: BoardFixtureV1, options?: BoardForceGraphLayoutOptions): BoardFixtureV1 {
  const out = boardRedrawLayoutFixtureJson(
    JSON.stringify(fixture),
    JSON.stringify({
      forceGraph: options ?? {},
      mode: "force-graph",
      redrawHandlesAfter: false,
    }),
  );
  return JSON.parse(out) as BoardFixtureV1;
}

/** @emoji 🧩 Runs WASM fixture redraw (force graph or hierarchical tree) with optional chained handle snap. */
export function layoutBoardFixtureRedrawNodes(fixture: BoardFixtureV1, options: BoardRedrawLayoutOptions): BoardFixtureV1 {
  const out = boardRedrawLayoutFixtureJson(JSON.stringify(fixture), JSON.stringify(options));
  return JSON.parse(out) as BoardFixtureV1;
}

/** @emoji 🔗 Snaps fixture handle angles to straight chords between linked node centers (WASM). */
export function layoutBoardFixtureRedrawHandles(fixture: BoardFixtureV1): BoardFixtureV1 {
  const out = boardRedrawHandlesFixtureJson(JSON.stringify(fixture));
  return JSON.parse(out) as BoardFixtureV1;
}

/** @emoji 🖱️ Hit-under-pointer payload for {@link BoardEventMap.hover} (tooltips, status, …). */
export interface BoardHoverPayload {
  clientX: number;
  clientY: number;
  id: string | null;
  /** @emoji 📐 Canvas-local CSS pixels passed to {@link BoardRenderer.screenToWorld}. */
  screenX: number;
  screenY: number;
  worldX: number;
  worldY: number;
}

/** @emoji 🪪 Payload for {@link BoardEventMap.nodeChange} and other single-node graph notifications. */
export interface BoardGraphNodeIdPayload {
  id: string;
}

/** @emoji 🪪 Payload for {@link BoardEventMap.childEdgeChange} and {@link BoardEventMap.parentEdgeChange}. */
export interface BoardGraphEdgeIdPayload {
  id: string;
}

/** @emoji 🌳 Emitted when the multiset of subtree child node ids under all {@link Node.root} nodes changes. */
export interface BoardChildNodesChangePayload {
  rootIds: string[];
  nodeIds: string[];
}

/** @emoji 🌳 Emitted when the multiset of subtree edge ids under roots changes (see {@link BoardChildNodesChangePayload}). */
export interface BoardChildEdgesChangePayload {
  rootIds: string[];
  edgeIds: string[];
}

/** @emoji 🪢 Payload for {@link BoardEventMap.edgeCreate} and gesture connect aliases. */
export interface BoardEdgeLinkPayload {
  id: string;
  source: string;
  target: string;
}

/** @emoji 🧵 Payload for {@link BoardEventMap.wireCreate} (declarative / scene wire). */
export interface BoardWireSnapshotPayload {
  endX: number | null;
  endY: number | null;
  id: string;
  source: string;
  target: string | null;
  wireKind: string;
}

/** @emoji 🪪 Payload for {@link BoardEventMap.wireChange} / {@link BoardEventMap.wireDestroy}. */
export interface BoardGraphWireIdPayload {
  id: string;
}

/** @emoji 📦 Optional aggregate for {@link BoardCanvasProps.onCreate} (node, edge, or wire). */
export type BoardStructureCreatePayload = { kind: "edge"; id: string; source: string; target: string } | { kind: "node"; id: string } | { kind: "wire"; payload: BoardWireSnapshotPayload };

/** @emoji 📦 Optional aggregate for {@link BoardCanvasProps.onDelete} (node, edge, or wire). */
export type BoardStructureDeletePayload = { kind: "edge"; id: string } | { kind: "node"; id: string } | { kind: "wire"; id: string };

export interface BoardLinkCompatibleNodesPayload {
  readonly source: string;
  readonly nodeIds: readonly string[];
}

export interface BoardLinkTargetRingPayload {
  readonly source: string;
  readonly nodeId: string | null;
  readonly handleIds: readonly string[];
}

/** @emoji 🔗 Host-driven link gesture preview mirrored across flat surfaces (see {@link BoardCanvasProps.linkSession}). */
export interface BoardLinkSessionSnapshot {
  readonly source: string;
  readonly endX: number;
  readonly endY: number;
  readonly compatiblePartIds: readonly string[];
  readonly ringPartId: string | null;
  readonly ringAnchorIds: readonly string[];
}

export interface BoardEventMap {
  camera: CameraState;
  change: undefined;
  childEdgeChange: BoardGraphEdgeIdPayload;
  childEdgesChange: BoardChildEdgesChangePayload;
  childNodeChange: BoardGraphNodeIdPayload;
  childNodesChange: BoardChildNodesChangePayload;
  contextmenu: { clientX: number; clientY: number; id: string | null; x: number; y: number };
  edgeChange: BoardGraphEdgeIdPayload;
  edgeCreate: BoardEdgeLinkPayload;
  edgeDelete: { id: string };
  fixtureDrop: BoardFixtureDropDetail;
  hover: BoardHoverPayload;
  indirectConnect: BoardEdgeLinkPayload;
  linkCompatibleNodes: BoardLinkCompatibleNodesPayload;
  linkTargetRing: BoardLinkTargetRingPayload;
  invalidate: undefined;
  nodeChange: BoardGraphNodeIdPayload;
  nodeCreate: BoardGraphNodeIdPayload;
  nodeDelete: { id: string };
  nodeMove: { id: string; x: number; y: number };
  parentEdgeChange: BoardGraphEdgeIdPayload;
  parentNodeChange: BoardGraphNodeIdPayload;
  proximityConnect: BoardEdgeLinkPayload;
  select: BoardSelectionSnapshot;
  preselect: BoardPreselectSnapshot;
  preselectCancel: BoardPreselectSnapshot;
  wireChange: BoardGraphWireIdPayload;
  wireCreate: BoardWireSnapshotPayload;
  wireDestroy: BoardGraphWireIdPayload;
}

export interface BoardObjectOptions {
  draggable?: boolean;
  highlighted?: boolean;
  id: string;
  selected?: boolean;
  style?: string;
  userData?: Record<string, unknown>;
  visible?: boolean;
}

/** @emoji 🔵 World-space circle node (center + radius). */
export type CircleNodeOptions = BoardObjectOptions & {
  handles?: BoardSceneHandle[];
  /** @emoji 🏷️ Runtime icon string for WASM detail LOD vector paint (baked catalog id or inline SVG). */
  iconKind?: string;
  /** @emoji 🧩 Semantic node-kind id for catalog defaults and compatibility (`node` specificity). */
  nodeKind?: string;
  /** @emoji 🌳 Marks this node as a hierarchy root; edges follow parent {@link Handle} {@link Edge.source} → child {@link Edge.target}. */
  root?: boolean;
  radius: number;
  shape?: "circle";
  text?: string;
  /** @emoji 📏 When true, overlay label scales to fit inside the circle (layout px); drawn at node center. */
  textAutofit?: boolean;
  /** @emoji 🧭 Caption alignment inside the node box when not using autofit. */
  textAlignment?: BoardNodeTextAlignment;
  /** @emoji 🔤 CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Caption size in layout px when not using autofit. */
  textFontSize?: number;
  x: number;
  y: number;
};

/** @emoji 🟩 World-space axis-aligned rectangle node (center + full width and height). */
export type RectangleNodeOptions = BoardObjectOptions & {
  handles?: BoardSceneHandle[];
  height: number;
  /** @emoji 🏷️ Runtime icon string for WASM detail LOD vector paint (baked catalog id or inline SVG). */
  iconKind?: string;
  /** @emoji 🧩 Semantic node-kind id for catalog defaults and compatibility (`node` specificity). */
  nodeKind?: string;
  /** @emoji 🌳 Marks this node as a hierarchy root; edges follow parent {@link Handle} {@link Edge.source} → child {@link Edge.target}. */
  root?: boolean;
  shape: "rectangle";
  text?: string;
  /** @emoji 📏 When true, overlay label scales to fit inside the rectangle (layout px); drawn at node center. */
  textAutofit?: boolean;
  /** @emoji 🧭 Caption alignment inside the node box when not using autofit. */
  textAlignment?: BoardNodeTextAlignment;
  /** @emoji 🔤 CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Caption size in layout px when not using autofit. */
  textFontSize?: number;
  width: number;
  x: number;
  y: number;
};

/** @emoji 🧩 Constructor payload for {@link Node} (circle or rectangle). */
export type NodeOptions = CircleNodeOptions | RectangleNodeOptions;

export interface HandleOptions extends BoardObjectOptions {
  angle: number;
  /** @emoji 🎨 Optional CSS hex fill overriding the handle-kind catalog color on the WASM host. */
  color?: string | null;
  /** @emoji 🏷️ Optional WASM detail LOD icon string (`typst:` / `$…`, `emoji:`, `data:` / raster data URLs, catalog id, or inline SVG). */
  iconKind?: string;
  /** @emoji 🔗 Semantic handle kind for WASM link compatibility (not {@link BoardObject.kind}). */
  handleKind: string;
  node: BoardSceneNode;
  radius?: number;
}

/** @emoji 🟣 Declarative handle marker props (React + reconciler). */
export interface BoardHandleProps {
  angle: number;
  color?: string | null;
  contextMenu?: ContextMenuItem[];
  /** @emoji 🔗 Semantic handle kind for WASM link compatibility (not the host intrinsic object kind). */
  handleKind: string;
  id: string;
  /** @emoji 🏷️ Optional WASM detail LOD icon string (`typst:` / `$…`, `emoji:`, `data:` / raster data URLs, catalog id, or inline SVG). */
  iconKind?: string;
  radius?: number;
  highlighted?: boolean;
  selected?: boolean;
  style?: string;
  userData?: Record<string, unknown>;
  visible?: boolean;
}

/** @emoji 🪢 Declarative edge marker props. */
export interface BoardEdgeProps {
  contextMenu?: ContextMenuItem[];
  /** @emoji 🧩 Semantic edge-kind id for catalog defaults and compatibility (`edge` specificity). */
  edgeKind?: string;
  id: string;
  highlighted?: boolean;
  selected?: boolean;
  source: string;
  style?: string;
  target: string;
  userData?: Record<string, unknown>;
  visible?: boolean;
}

/** @emoji 🧵 Declarative wire props: anchored at {@link BoardWireProps.source}; either {@link BoardWireProps.target} handle id **or** {@link BoardWireProps.endX}/{@link BoardWireProps.endY} world end. */
export interface BoardWireProps {
  contextMenu?: ContextMenuItem[];
  endX?: number;
  endY?: number;
  id: string;
  highlighted?: boolean;
  selected?: boolean;
  source: string;
  style?: string;
  target?: string;
  /** @emoji 🧩 Semantic wire-kind id for catalog defaults and compatibility (`wire` specificity). */
  wireKind?: string;
  userData?: Record<string, unknown>;
  visible?: boolean;
}

export interface EdgeOptions extends BoardObjectOptions {
  edgeKind?: string;
  source: BoardSceneHandle;
  target: BoardSceneHandle;
}

export interface WireOptions extends BoardObjectOptions {
  endX?: number | null;
  endY?: number | null;
  source: BoardSceneHandle;
  target: BoardSceneHandle | null;
  wireKind?: string;
}

type FrameListener = (state: FrameState, dt: number) => void;
type BoardCanvasElement = HTMLCanvasElement & { __boardRenderer?: BoardRenderer };
type BoardCanvasContext = Pick<
  CanvasRenderingContext2D,
  "arc" | "beginPath" | "bezierCurveTo" | "clearRect" | "clip" | "closePath" | "fill" | "fillRect" | "fillText" | "lineTo" | "measureText" | "moveTo" | "rect" | "restore" | "save" | "setLineDash" | "setTransform" | "stroke" | "strokeRect"
> & {
  fillStyle: string | CanvasGradient | CanvasPattern;
  font: string;
  lineCap: CanvasLineCap;
  lineJoin: CanvasLineJoin;
  lineWidth: number;
  strokeStyle: string | CanvasGradient | CanvasPattern;
  textAlign: CanvasTextAlign;
  textBaseline: CanvasTextBaseline;
};

//#endregion 🔖Kinds

//#region 🔖Utilities
const DEFAULT_CAMERA: CameraState = { x: 0, y: 0, zoom: 1 };
/** @emoji 🔍 Smallest allowed world scale (most zoomed-out). */
export const BOARD_CAMERA_ZOOM_MIN = 0.05;
/** @emoji 🔎 Largest allowed world scale (most zoomed-in). */
export const BOARD_CAMERA_ZOOM_MAX = 32;

const MIN_ZOOM = BOARD_CAMERA_ZOOM_MIN;
const MAX_ZOOM = BOARD_CAMERA_ZOOM_MAX;

/** @emoji ⌨️ True when Delete/Backspace should reach the board instead of staying in a focused text control. */
function shouldBoardHandleDeleteShortcut(): boolean {
  const el = document.activeElement;
  if (!el || !(el instanceof HTMLElement)) {
    return true;
  }
  const tag = el.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") {
    return false;
  }
  if (el.isContentEditable) {
    return false;
  }
  return true;
}
/** 📐 Quantized large grid step in world units (LOD grids scale `10` / `2.5` / `0.5` / `0.1` by {@link DEFAULT_BOARD_GRID_FACTOR}). */
export const BOARD_LOD_GRID_MAJOR_QUANTUM = 10;

/** @emoji 📐 Positive multiplier for LOD world grid steps (`10×` / `2.5×` / `0.5×` / `0.1×` world units per band); default `10` yields `100` / `25` / `5` / `1`. */
export const DEFAULT_BOARD_GRID_FACTOR = 10;

/** @emoji 📐 Default LOD zoom boundaries (world scale / CSS pixels); minimap < `minimapMaxZoom` < overview < `overviewMaxZoom` < compact < `compactMaxZoom` < normal < `normalMaxZoom` < detail < `detailMaxZoom` ≤ micro. */
export interface BoardLodZoomThresholds {
  minimapMaxZoom: number;
  overviewMaxZoom: number;
  compactMaxZoom: number;
  normalMaxZoom: number;
  detailMaxZoom: number;
}

export const DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS: BoardLodZoomThresholds = {
  minimapMaxZoom: 0.15,
  overviewMaxZoom: 0.35,
  compactMaxZoom: 0.55,
  normalMaxZoom: 1.25,
  detailMaxZoom: 2.5,
};

/** 📐 Alias of {@link DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS.minimapMaxZoom}. */
export const BOARD_LOD_MINIMAP_MAX_ZOOM = DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS.minimapMaxZoom;

/** 📐 Alias of {@link DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS.normalMaxZoom} (detail band starts here). */
export const BOARD_LOD_DETAIL_MIN_ZOOM = DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS.normalMaxZoom;

/** 📐 Alias of {@link DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS.detailMaxZoom} (micro band starts here). */
export const BOARD_LOD_MICRO_MIN_ZOOM = DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS.detailMaxZoom;

/** @emoji 📶 LOD label for `data-board-lod` using explicit thresholds. */
export function resolveBoardLodLabelFromThresholds(zoom: number, t: BoardLodZoomThresholds): BoardDrawLodKind {
  const z = zoom;
  if (z < t.minimapMaxZoom) {
    return "minimap";
  }
  if (z < t.overviewMaxZoom) {
    return "overview";
  }
  if (z < t.compactMaxZoom) {
    return "compact";
  }
  if (z < t.normalMaxZoom) {
    return "normal";
  }
  if (z < t.detailMaxZoom) {
    return "detail";
  }
  return "micro";
}

/** @emoji 📶 LOD tier for `data-board-lod` using {@link DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS}. */
export function resolveBoardLodLabel(zoom: number): BoardDrawLodKind {
  return resolveBoardLodLabelFromThresholds(zoom, DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS);
}

/** @emoji 📶 WASM draw LOD tier label (matches `data-board-lod` / `setForcedDrawLodLabel`). */
export type BoardDrawLodKind = "compact" | "detail" | "micro" | "minimap" | "normal" | "overview";

/** @emoji 📶 Select value: camera zoom picks the draw LOD band. */
export const BOARD_LOD_MODE_AUTOMATIC = "automatic" as const;

/** @emoji 📶 Board play / window LOD select value (`automatic` or a pinned {@link BoardDrawLodKind}). */
export type BoardLodModeKind = typeof BOARD_LOD_MODE_AUTOMATIC | BoardDrawLodKind;

const BOARD_DRAW_LOD_KINDS: readonly BoardDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

/** @emoji ✅ True when `label` is a pinned WASM draw LOD tier. */
export function isBoardDrawLodKind(label: string): label is BoardDrawLodKind {
  return (BOARD_DRAW_LOD_KINDS as readonly string[]).includes(label);
}

/** @emoji 📶 Maps a window LOD select value to {@link BoardCanvasProps} LOD fields. */
export function boardLodCanvasProps(mode: BoardLodModeKind): { automaticLod: boolean; lod?: BoardDrawLodKind } {
  if (mode === BOARD_LOD_MODE_AUTOMATIC) {
    return { automaticLod: true };
  }
  return { automaticLod: false, lod: mode };
}

/** @emoji 📶 Automatic LOD select row label showing the live zoom-derived tier. */
export function boardLodAutomaticSelectLabel(effectiveTier: BoardDrawLodKind): string {
  return `Automatic · ${effectiveTier.charAt(0).toUpperCase()}${effectiveTier.slice(1)}`;
}

/** @emoji 🎨 Offline / headless paint defaults aligned with `elements/core/styling/tokens.json` `board_vello_canvas` sRGB (Vello host defaults before DOM tokens sync). */
const BOARD_STYLES_HEADLESS_FALLBACK: Record<string, BoardStyle> = {
  edge: { stroke: "#7b827d", strokeWidth: 2 },
  "edge.highlighted": { stroke: "#34d1bf", strokeWidth: 2 },
  "edge.selected": { stroke: "#ff344f", strokeWidth: 3 },
  handle: { fill: "#f7f3e3", stroke: "#001117", strokeWidth: 2 },
  "handle.highlighted": { fill: "#c4e4d5", stroke: "#34d1bf", strokeWidth: 1.5 },
  "handle.selected": { fill: "#ff344f", stroke: "#ff344f", strokeWidth: 2 },
  node: { fill: "#eeeadb", stroke: "#001117", strokeWidth: 2 },
  "node.highlighted": { fill: "#c4e4d5", stroke: "#34d1bf", strokeWidth: 2 },
  "node.selected": { fill: "#f0c8cc", stroke: "#ff344f", strokeWidth: 3 },
};

const DEFAULT_STYLES: Record<string, BoardStyle> = BOARD_STYLES_HEADLESS_FALLBACK;

//#region 🎨ElementsUiBoardPaint
/** @emoji 🎨 Elements semantic tokens for committed selection chrome (primary, not secondary). */
const BOARD_CSS_COLOR_PRIMARY = "var(--color-primary)";
const BOARD_CSS_SELECTED_FILL = "color-mix(in oklab, var(--color-primary) 28%, var(--color-panel))";
/** @emoji 🎨 Secondary-tinted fill for preselect exit / highlight chrome only. */
const BOARD_CSS_HIGHLIGHTED_FILL = "color-mix(in oklab, var(--color-secondary) 24%, var(--color-panel))";

/** @emoji 🎨 Resolves UI semantic CSS (`@ui/styling/ui.css` / `@theme`) for 2d canvas + Vello: only `var(--…)` tokens wired here — no ad-hoc palettes. */
const BOARD_VELLO_THEME_FALLBACK_RGBA = {
  rasterClear: [247, 243, 227, 255] as [number, number, number, number],
  gridMinorStroke: [123, 130, 125, 56] as [number, number, number, number],
  edgeStroke: [123, 130, 125, 255] as [number, number, number, number],
  edgeStrokeHovered: [123, 130, 125, 255] as [number, number, number, number],
  edgeStrokeSelected: [255, 52, 79, 255] as [number, number, number, number],
  edgeStrokeSelectionExit: [52, 209, 191, 255] as [number, number, number, number],
  edgeStrokeDisabled: [123, 130, 125, 96] as [number, number, number, number],
  nodeFill: [238, 234, 219, 255] as [number, number, number, number],
  nodeStroke: [0, 17, 23, 255] as [number, number, number, number],
  nodeFillHovered: [192, 205, 197, 255] as [number, number, number, number],
  nodeStrokeHovered: [123, 130, 125, 255] as [number, number, number, number],
  nodeFillSelected: [240, 200, 204, 255] as [number, number, number, number],
  nodeStrokeSelected: [255, 52, 79, 255] as [number, number, number, number],
  nodeFillSelectionExit: [196, 228, 213, 255] as [number, number, number, number],
  nodeStrokeSelectionExit: [52, 209, 191, 255] as [number, number, number, number],
  nodeFillDisabled: [238, 234, 219, 128] as [number, number, number, number],
  nodeStrokeDisabled: [123, 130, 125, 96] as [number, number, number, number],
  indirectHandleFill: [196, 228, 213, 255] as [number, number, number, number],
  indirectHandleStroke: [52, 209, 191, 255] as [number, number, number, number],
  handleFill: [247, 243, 227, 255] as [number, number, number, number],
  handleStroke: [0, 17, 23, 255] as [number, number, number, number],
  handleFillHovered: [192, 205, 197, 255] as [number, number, number, number],
  handleStrokeHovered: [123, 130, 125, 255] as [number, number, number, number],
  handleFillSelected: [255, 52, 79, 255] as [number, number, number, number],
  handleStrokeSelected: [255, 52, 79, 255] as [number, number, number, number],
  handleFillSelectionExit: [196, 228, 213, 255] as [number, number, number, number],
  handleStrokeSelectionExit: [52, 209, 191, 255] as [number, number, number, number],
  handleFillDisabled: [238, 234, 219, 128] as [number, number, number, number],
  handleStrokeDisabled: [123, 130, 125, 96] as [number, number, number, number],
  wireStroke: [123, 130, 125, 255] as [number, number, number, number],
  wireStrokeHovered: [123, 130, 125, 255] as [number, number, number, number],
  wireStrokeSelected: [255, 52, 79, 255] as [number, number, number, number],
  wireStrokeHighlighted: [52, 209, 191, 255] as [number, number, number, number],
  wireStrokeDisabled: [123, 130, 125, 96] as [number, number, number, number],
  selectionPreviewFill: [255, 52, 79, 36] as [number, number, number, number],
  selectionPreviewStroke: [255, 52, 79, 191] as [number, number, number, number],
};

function boardParseCssColorToRgba8888(css: string, fallback: [number, number, number, number]): [number, number, number, number] {
  const m = css.match(/rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)(?:\s*,\s*([\d.]+%?)\s*)?\)/u);
  if (!m) {
    return fallback;
  }
  const r = Math.min(255, Math.max(0, Math.round(Number(m[1]))));
  const g = Math.min(255, Math.max(0, Math.round(Number(m[2]))));
  const b = Math.min(255, Math.max(0, Math.round(Number(m[3]))));
  let a = 255;
  if (m[4] !== undefined && m[4] !== "") {
    const raw = m[4];
    if (raw.endsWith("%")) {
      a = Math.min(255, Math.max(0, Math.round((Number(raw.slice(0, -1)) / 100) * 255)));
    } else {
      const n = Number(raw);
      a = Math.min(255, Math.max(0, Math.round(n <= 1 ? n * 255 : n)));
    }
  }
  return [r, g, b, a];
}

function boardProbeCssComputed(property: "color" | "backgroundColor", value: string): string {
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

function boardDefaultStylesFromElementsUiTokens(): Record<string, BoardStyle> {
  const f = BOARD_STYLES_HEADLESS_FALLBACK;
  const c = (prop: "color" | "backgroundColor", expr: string, fb: string): string => {
    const raw = boardProbeCssComputed(prop, expr);
    if (!raw || raw === "rgba(0, 0, 0, 0)") {
      return fb;
    }
    return raw;
  };
  return {
    edge: { stroke: c("color", "var(--color-muted-foreground)", f.edge.stroke ?? "#7b827d"), strokeWidth: 2 },
    "edge.highlighted": {
      stroke: c("color", "var(--color-secondary)", f["edge.highlighted"]?.stroke ?? "#34d1bf"),
      strokeWidth: 2,
    },
    "edge.selected": { stroke: c("color", BOARD_CSS_COLOR_PRIMARY, f["edge.selected"].stroke ?? "#ff344f"), strokeWidth: 3 },
    handle: {
      fill: c("backgroundColor", "var(--color-base)", f.handle.fill ?? "#f7f3e3"),
      stroke: c("color", "var(--color-element)", f.handle.stroke ?? "#001117"),
      strokeWidth: 2,
    },
    "handle.highlighted": {
      fill: c("backgroundColor", BOARD_CSS_HIGHLIGHTED_FILL, f["handle.highlighted"]?.fill ?? "#c4e4d5"),
      stroke: c("color", "var(--color-secondary)", f["handle.highlighted"]?.stroke ?? "#34d1bf"),
      strokeWidth: 1.5,
    },
    "handle.selected": {
      fill: c("backgroundColor", BOARD_CSS_SELECTED_FILL, f["handle.selected"].fill ?? "#ff344f"),
      stroke: c("color", BOARD_CSS_COLOR_PRIMARY, f["handle.selected"].stroke ?? "#ff344f"),
      strokeWidth: 2,
    },
    node: {
      fill: c("backgroundColor", "var(--color-panel)", f.node.fill ?? "#eeeadb"),
      stroke: c("color", "var(--color-element)", f.node.stroke ?? "#001117"),
      strokeWidth: 2,
    },
    "node.highlighted": {
      fill: c("backgroundColor", BOARD_CSS_HIGHLIGHTED_FILL, f["node.highlighted"]?.fill ?? "#c4e4d5"),
      stroke: c("color", "var(--color-secondary)", f["node.highlighted"]?.stroke ?? "#34d1bf"),
      strokeWidth: 2,
    },
    "node.selected": {
      fill: c("backgroundColor", BOARD_CSS_SELECTED_FILL, f["node.selected"].fill ?? "#f0c8cc"),
      stroke: c("color", BOARD_CSS_COLOR_PRIMARY, f["node.selected"].stroke ?? "#ff344f"),
      strokeWidth: 3,
    },
  };
}

function serializeElementsBoardVelloThemeJson(): string {
  const fb = BOARD_VELLO_THEME_FALLBACK_RGBA;
  const pc = (prop: "color" | "backgroundColor", expr: string, fall: [number, number, number, number]): number[] => {
    const raw = boardProbeCssComputed(prop, expr);
    return [...boardParseCssColorToRgba8888(raw, fall)];
  };
  const payload = {
    rasterClear: pc("backgroundColor", "var(--base)", fb.rasterClear),
    gridMinorStroke: (() => {
      const border = boardParseCssColorToRgba8888(boardProbeCssComputed("color", "var(--color-border)"), [fb.gridMinorStroke[0], fb.gridMinorStroke[1], fb.gridMinorStroke[2], 255]);
      return [border[0], border[1], border[2], fb.gridMinorStroke[3]];
    })(),
    edgeStroke: pc("color", "var(--color-muted-foreground)", fb.edgeStroke),
    edgeStrokeHovered: pc("color", "var(--color-hover-base)", fb.edgeStrokeHovered),
    edgeStrokeSelected: pc("color", BOARD_CSS_COLOR_PRIMARY, fb.edgeStrokeSelected),
    edgeStrokeSelectionExit: pc("color", "var(--color-secondary)", fb.edgeStrokeSelectionExit),
    edgeStrokeDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 38%, transparent)", fb.edgeStrokeDisabled),
    nodeFill: pc("backgroundColor", "var(--color-panel)", fb.nodeFill),
    nodeStroke: pc("color", "var(--color-element)", fb.nodeStroke),
    nodeFillHovered: pc("backgroundColor", "var(--color-hover-panel)", fb.nodeFillHovered),
    nodeStrokeHovered: pc("color", "var(--color-hover-base)", fb.nodeStrokeHovered),
    nodeFillSelected: pc("backgroundColor", BOARD_CSS_SELECTED_FILL, fb.nodeFillSelected),
    nodeStrokeSelected: pc("color", BOARD_CSS_COLOR_PRIMARY, fb.nodeStrokeSelected),
    nodeFillSelectionExit: pc("backgroundColor", BOARD_CSS_HIGHLIGHTED_FILL, fb.nodeFillSelectionExit),
    nodeStrokeSelectionExit: pc("color", "var(--color-secondary)", fb.nodeStrokeSelectionExit),
    nodeFillDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-panel) 50%, transparent)", fb.nodeFillDisabled),
    nodeStrokeDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 38%, transparent)", fb.nodeStrokeDisabled),
    indirectHandleFill: pc("backgroundColor", BOARD_CSS_HIGHLIGHTED_FILL, fb.indirectHandleFill),
    indirectHandleStroke: pc("color", "var(--color-secondary)", fb.indirectHandleStroke),
    handleFill: pc("backgroundColor", "var(--color-base)", fb.handleFill),
    handleStroke: pc("color", "var(--color-element)", fb.handleStroke),
    handleFillHovered: pc("backgroundColor", "var(--color-hover-panel)", fb.handleFillHovered),
    handleStrokeHovered: pc("color", "var(--color-hover-base)", fb.handleStrokeHovered),
    handleFillSelected: pc("backgroundColor", BOARD_CSS_COLOR_PRIMARY, fb.handleFillSelected),
    handleStrokeSelected: pc("color", BOARD_CSS_COLOR_PRIMARY, fb.handleStrokeSelected),
    handleFillSelectionExit: pc("backgroundColor", BOARD_CSS_HIGHLIGHTED_FILL, fb.handleFillSelectionExit),
    handleStrokeSelectionExit: pc("color", "var(--color-secondary)", fb.handleStrokeSelectionExit),
    handleFillDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-panel) 50%, transparent)", fb.handleFillDisabled),
    handleStrokeDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 38%, transparent)", fb.handleStrokeDisabled),
    wireStroke: pc("color", "var(--color-muted-foreground)", fb.wireStroke),
    wireStrokeHovered: pc("color", "var(--color-hover-base)", fb.wireStrokeHovered),
    wireStrokeSelected: pc("color", BOARD_CSS_COLOR_PRIMARY, fb.wireStrokeSelected),
    wireStrokeHighlighted: pc("color", "var(--color-secondary)", fb.wireStrokeHighlighted),
    wireStrokeDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 38%, transparent)", fb.wireStrokeDisabled),
    selectionPreviewFill: pc("backgroundColor", "color-mix(in oklab, var(--color-accent) 14%, transparent)", fb.selectionPreviewFill),
    selectionPreviewStroke: pc("backgroundColor", "color-mix(in oklab, var(--color-accent) 75%, transparent)", fb.selectionPreviewStroke),
  };
  return JSON.stringify(payload);
}
//#endregion 🎨ElementsUiBoardPaint

/** @emoji 🧭 Caption anchor inside the node box (compass, origin at node center). */
export const BOARD_NODE_TEXT_ALIGNMENTS = ["c", "e", "n", "ne", "nw", "s", "se", "sw", "w"] as const;
export type BoardNodeTextAlignment = (typeof BOARD_NODE_TEXT_ALIGNMENTS)[number];

/** @emoji ⬅️ Default: reading-order start at west edge, vertically centered (`w`). */
export const BOARD_NODE_TEXT_ALIGNMENT_DEFAULT: BoardNodeTextAlignment = "w";

/** @emoji 🔤 Default overlay caption size (layout px) when `textAutofit` is false. */
export const BOARD_NODE_TEXT_FONT_PX_DEFAULT = 14;

/** @emoji 🔤 Default sans stack for overlay captions. */
export const BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT = "system-ui,Segoe UI,sans-serif";

/** @emoji ✅ True when `value` is a known {@link BoardNodeTextAlignment} token. */
export function isBoardNodeTextAlignment(value: string): value is BoardNodeTextAlignment {
  return (BOARD_NODE_TEXT_ALIGNMENTS as readonly string[]).includes(value);
}

/** @emoji 🖋️ Builds a `CanvasRenderingContext2D.font` string from size and family. */
export function boardBuildCanvasFontSpec(px: number, fontFamily: string): string {
  return `${px}px ${fontFamily}`;
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

function nearlyEqual(left: number, right: number, tolerance = 0.0001): boolean {
  return Math.abs(left - right) <= tolerance;
}

function pointsEqual(left: Point, right: Point, tolerance = 0.0001): boolean {
  return nearlyEqual(left.x, right.x, tolerance) && nearlyEqual(left.y, right.y, tolerance);
}

function subtractPoint(left: Point, right: Point): Point {
  return { x: left.x - right.x, y: left.y - right.y };
}

function addPoint(left: Point, right: Point): Point {
  return { x: left.x + right.x, y: left.y + right.y };
}

function scalePoint(point: Point, scalar: number): Point {
  return { x: point.x * scalar, y: point.y * scalar };
}

function lengthOf(point: Point): number {
  return Math.hypot(point.x, point.y);
}

function normalizePoint(point: Point): Point {
  const magnitude = lengthOf(point);
  if (magnitude <= Number.EPSILON) {
    return { x: 0, y: 0 };
  }
  return { x: point.x / magnitude, y: point.y / magnitude };
}

function distanceBetween(left: Point, right: Point): number {
  return Math.hypot(left.x - right.x, left.y - right.y);
}

function shallowEqualRecord(left: Record<string, unknown>, right: Record<string, unknown>): boolean {
  const leftKeys = Object.keys(left);
  const rightKeys = Object.keys(right);
  if (leftKeys.length !== rightKeys.length) {
    return false;
  }
  for (const key of leftKeys) {
    if (left[key] !== right[key]) {
      return false;
    }
  }
  return true;
}

function arrayEqual(left: string[], right: string[]): boolean {
  if (left.length !== right.length) {
    return false;
  }
  return left.every((value, index) => value === right[index]);
}

function preselectSnapshotsEqual(left: BoardPreselectSnapshot, right: BoardPreselectSnapshot): boolean {
  return arrayEqual(left.ids, right.ids) && arrayEqual(left.removedIds, right.removedIds);
}

/** @emoji 🧩 Compares committed selection snapshots by sorted id list. */
export function boardSelectionSnapshotsEqual(left: BoardSelectionSnapshot, right: BoardSelectionSnapshot): boolean {
  return arrayEqual(left.ids, right.ids);
}

/** @emoji 🧩 Compares preselect snapshots by ids and removedIds. */
export function boardPreselectSnapshotsEqual(left: BoardPreselectSnapshot, right: BoardPreselectSnapshot): boolean {
  return preselectSnapshotsEqual(left, right);
}

function cubicBezierPoint(curve: CubicBezierCurve, step: number): Point {
  const oneMinusStep = 1 - step;
  const oneMinusSquared = oneMinusStep * oneMinusStep;
  const oneMinusCubed = oneMinusSquared * oneMinusStep;
  const stepSquared = step * step;
  const stepCubed = stepSquared * step;
  return {
    x: curve.p0.x * oneMinusCubed + 3 * curve.p1.x * oneMinusSquared * step + 3 * curve.p2.x * oneMinusStep * stepSquared + curve.p3.x * stepCubed,
    y: curve.p0.y * oneMinusCubed + 3 * curve.p1.y * oneMinusSquared * step + 3 * curve.p2.y * oneMinusStep * stepSquared + curve.p3.y * stepCubed,
  };
}

function sortedSelectionIds(ids: Iterable<string>): string[] {
  return Array.from(ids).sort((left, right) => left.localeCompare(right));
}

function createSelectionSnapshot(ids: Iterable<string>): BoardSelectionSnapshot {
  return { ids: sortedSelectionIds(ids) };
}

function createPreselectSnapshot(ids: Iterable<string>, removedIds: Iterable<string>): BoardPreselectSnapshot {
  return { ids: sortedSelectionIds(ids), removedIds: sortedSelectionIds(removedIds) };
}

/** @emoji 🧩 Normalizes {@link BoardSelectionSnapshot} or a bare id list into a sorted snapshot. */
export function normalizeBoardSelectionProp(value: BoardSelectionSnapshot | readonly string[] | undefined): BoardSelectionSnapshot {
  if (value === undefined) {
    return { ids: [] };
  }
  if (Array.isArray(value)) {
    return createSelectionSnapshot(value);
  }
  return createSelectionSnapshot(value.ids);
}

/** @emoji 🧩 Normalizes {@link BoardPreselectSnapshot} props for controlled board interaction state. */
export function normalizeBoardPreselectProp(value: BoardPreselectSnapshot | undefined): BoardPreselectSnapshot {
  if (value === undefined) {
    return BOARD_PRESELECT_EMPTY;
  }
  return createPreselectSnapshot(value.ids, value.removedIds);
}

function resolveSelectionOptions(options: BoardSelectionOptions | undefined): ResolvedBoardSelectionOptions {
  return {
    method: options?.method ?? "rectangle",
    mode: options?.mode ?? "default",
    targets: {
      edges: options?.targets?.edges ?? BOARD_SELECTION_TARGETS_DEFAULT.edges,
      handles: options?.targets?.handles ?? BOARD_SELECTION_TARGETS_DEFAULT.handles,
      nodes: options?.targets?.nodes ?? BOARD_SELECTION_TARGETS_DEFAULT.nodes,
    },
  };
}

function boardSelectionModeForHost(mode: BoardSelectionMode): string {
  return mode === "default" ? "replace" : mode;
}

/** @emoji 🏷️ Resolves optional node caption from raw fixture JSON (`text` only). */
function fixtureNodeTextFromJson(node: Record<string, unknown>): string | undefined {
  if (typeof node.text !== "string") {
    return undefined;
  }
  const trimmed = node.text.trim();
  return trimmed !== "" ? trimmed : undefined;
}

/** @emoji 🏷️ On-canvas / inspector caption for a parsed fixture node. */
export function boardFixtureNodeCaption(node: BoardFixtureNodeV1): string | undefined {
  const text = node.text?.trim();
  return text !== "" ? text : undefined;
}

function fixtureOptionalTextFontFamily(node: Record<string, unknown>): string | undefined {
  const raw = node.textFontFamily;
  if (typeof raw !== "string") {
    return undefined;
  }
  const trimmed = raw.trim();
  return trimmed !== "" ? trimmed : undefined;
}

function fixtureOptionalTextFontSize(node: Record<string, unknown>): number | undefined {
  const n = Number(node.textFontSize);
  return Number.isFinite(n) && n > 0 ? n : undefined;
}

function fixtureOptionalTextAlignment(node: Record<string, unknown>): BoardNodeTextAlignment | undefined {
  const v = node.textAlignment;
  return typeof v === "string" && isBoardNodeTextAlignment(v) ? v : undefined;
}

/** @emoji 🧾 Validates unknown JSON into {@link BoardFixtureV1} or returns null. */
export function parseBoardFixtureV1(raw: unknown): BoardFixtureV1 | null {
  if (!raw || typeof raw !== "object") {
    return null;
  }
  const root = raw as Record<string, unknown>;
  if (root.schema !== "puzzle.2d.fixture/v1") {
    return null;
  }
  const cam = root.camera;
  if (!cam || typeof cam !== "object") {
    return null;
  }
  const cameraRecord = cam as Record<string, unknown>;
  const camera: CameraState = {
    x: Number(cameraRecord.x),
    y: Number(cameraRecord.y),
    zoom: Number(cameraRecord.zoom),
  };
  if (!Number.isFinite(camera.x) || !Number.isFinite(camera.y) || !Number.isFinite(camera.zoom)) {
    return null;
  }
  if (!Array.isArray(root.nodes) || !Array.isArray(root.edges)) {
    return null;
  }
  const nodes: BoardFixtureNodeV1[] = [];
  for (const entry of root.nodes) {
    if (!entry || typeof entry !== "object") {
      return null;
    }
    const node = entry as Record<string, unknown>;
    if (Object.hasOwn(node, "label")) {
      return null;
    }
    const id = typeof node.id === "string" ? node.id : null;
    const x = Number(node.x);
    const y = Number(node.y);
    if (!id || !Number.isFinite(x) || !Number.isFinite(y)) {
      return null;
    }
    if (!Array.isArray(node.handles)) {
      return null;
    }
    const handles: BoardFixtureHandleV1[] = [];
    for (const h of node.handles) {
      if (!h || typeof h !== "object") {
        return null;
      }
      const hr = h as Record<string, unknown>;
      const hid = typeof hr.id === "string" ? hr.id : null;
      const angle = Number(hr.angle);
      if (!hid || !Number.isFinite(angle)) {
        return null;
      }
      const rawKind = typeof hr.handleKind === "string" ? hr.handleKind.trim() : "";
      const handleKind = rawKind !== "" ? rawKind : BUILTIN_PORT_HANDLE_KIND;
      const colorRaw = hr.color;
      const colorTrim = typeof colorRaw === "string" && colorRaw.trim() !== "" ? colorRaw.trim() : undefined;
      const hradius = Number(hr.radius);
      const withRadius = Number.isFinite(hradius) && hradius > 0;
      const iconRaw = hr.iconKind;
      const iconTrim = typeof iconRaw === "string" && iconRaw.trim() !== "" ? iconRaw.trim() : undefined;
      const base: BoardFixtureHandleV1 = {
        angle,
        handleKind,
        id: hid,
        ...(colorTrim !== undefined ? { color: colorTrim } : {}),
        ...(withRadius ? { radius: hradius } : {}),
        ...(iconTrim !== undefined ? { iconKind: iconTrim } : {}),
      };
      handles.push(base);
    }
    const textFromJson = fixtureNodeTextFromJson(node);
    const textAutofit = node.textAutofit === true;
    const textFontFamily = fixtureOptionalTextFontFamily(node);
    const textFontSize = fixtureOptionalTextFontSize(node);
    const textAlignment = fixtureOptionalTextAlignment(node);
    const rootFlag = node.root === true;
    const iconKindRaw = (node as Record<string, unknown>).iconKind;
    const iconKind = typeof iconKindRaw === "string" && iconKindRaw.trim() !== "" ? iconKindRaw.trim() : undefined;
    const nodeKindRaw = node.nodeKind;
    const nodeKind = typeof nodeKindRaw === "string" && nodeKindRaw.trim() !== "" ? nodeKindRaw.trim() : undefined;
    const cad =
      node.cad && typeof node.cad === "object"
        ? {
            x: Number((node.cad as Record<string, unknown>).x),
            y: Number((node.cad as Record<string, unknown>).y),
            z: Number((node.cad as Record<string, unknown>).z),
          }
        : node.cad === null
          ? null
          : undefined;
    const shapeRaw = node.shape;
    if (shapeRaw === "rectangle") {
      const width = Number(node.width);
      const height = Number(node.height);
      if (!Number.isFinite(width) || !Number.isFinite(height) || width <= 0 || height <= 0) {
        return null;
      }
      nodes.push({
        ...(cad !== undefined ? { cad } : {}),
        ...(textFromJson !== undefined ? { text: textFromJson } : {}),
        ...(textAutofit ? { textAutofit: true } : {}),
        ...(textFontFamily !== undefined ? { textFontFamily } : {}),
        ...(textFontSize !== undefined ? { textFontSize } : {}),
        ...(textAlignment !== undefined ? { textAlignment } : {}),
        ...(rootFlag ? { root: true } : {}),
        ...(iconKind !== undefined ? { iconKind } : {}),
        ...(nodeKind !== undefined ? { nodeKind } : {}),
        handles,
        height,
        id,
        shape: "rectangle",
        width,
        x,
        y,
      });
      continue;
    }
    if (shapeRaw !== undefined && shapeRaw !== "circle") {
      return null;
    }
    const radius = Number(node.radius);
    if (!Number.isFinite(radius) || radius <= 0) {
      return null;
    }
    nodes.push({
      ...(cad !== undefined ? { cad } : {}),
      ...(textFromJson !== undefined ? { text: textFromJson } : {}),
      ...(textAutofit ? { textAutofit: true } : {}),
      ...(textFontFamily !== undefined ? { textFontFamily } : {}),
      ...(textFontSize !== undefined ? { textFontSize } : {}),
      ...(textAlignment !== undefined ? { textAlignment } : {}),
      ...(rootFlag ? { root: true } : {}),
      ...(iconKind !== undefined ? { iconKind } : {}),
      ...(nodeKind !== undefined ? { nodeKind } : {}),
      handles,
      id,
      radius,
      shape: "circle",
      x,
      y,
    });
  }
  const edges: BoardFixtureEdgeV1[] = [];
  for (const entry of root.edges) {
    if (!entry || typeof entry !== "object") {
      return null;
    }
    const edge = entry as Record<string, unknown>;
    const id = typeof edge.id === "string" ? edge.id : null;
    const sourceRaw = edge.source;
    const targetRaw = edge.target;
    const source = typeof sourceRaw === "string" ? sourceRaw : null;
    const target = typeof targetRaw === "string" ? targetRaw : null;
    if (!id || !source || !target) {
      return null;
    }
    edges.push({ id, source, target });
  }
  const meta = root.meta && typeof root.meta === "object" ? (root.meta as Record<string, unknown>) : undefined;
  return { camera, edges, meta, nodes, schema: "puzzle.2d.fixture/v1" };
}

/** @emoji 📌 MIME for in-app board fixture drags (not host filesystem file drops). */
export const BOARD_FIXTURE_DRAG_V1_MIME = "application/x-puzzle-2d-fixture-v1";

/** @emoji 🧩 `BoardFixtureV1.meta.boardFixtureDragKind` — shelf palette drops merge one node at the pointer; any other payload replaces the scene. */
export const BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE = "palette-node";

/** @emoji 📍 Payload for board canvas fixture drops: scene plus pointer in canvas CSS space and mapped world coordinates. */
export interface BoardFixtureDropDetail {
  fixture: BoardFixtureV1;
  screen: { x: number; y: number };
  world: { x: number; y: number };
}

/** @emoji 📦 Serializes a validated fixture for {@link BOARD_FIXTURE_DRAG_V1_MIME}. */
export function encodeBoardFixtureForDragV1(fixture: BoardFixtureV1): string {
  return JSON.stringify(fixture);
}

/** @emoji 📥 Parses drag payload from {@link BOARD_FIXTURE_DRAG_V1_MIME}. */
export function decodeBoardFixtureFromDragV1(text: string): BoardFixtureV1 | null {
  let raw: unknown;
  try {
    raw = JSON.parse(text) as unknown;
  } catch {
    return null;
  }
  return parseBoardFixtureV1(raw);
}

/** @emoji 📍 Handle anchor on node perimeter: **rectangle** uses north-zero CCW angle; **circle** uses east-zero `atan2` convention (matches {@link boardHandlePositionCircle}). */
export function computeHandlePosition(node: { height: number; radius: number; shape: "circle" | "rectangle"; width: number; x: number; y: number }, angle: number): Point {
  if (node.shape === "rectangle") {
    const flat = boardHandlePositionRectangle(node.x, node.y, node.width, node.height, angle);
    return { x: flat[0], y: flat[1] };
  }
  const flat = boardHandlePositionCircle(node.x, node.y, node.radius, angle);
  return { x: flat[0], y: flat[1] };
}

export function computeHandleTangent(angle: number): Point {
  return {
    x: -Math.sin(angle),
    y: Math.cos(angle),
  };
}

/** @emoji 📏 Largest font size (px) so a single-line string fits `maxW`×`maxH` in layout pixels (binary search). */
export function boardFitTextFontPx(ctx: CanvasTextMeasuring, text: string, maxW: number, maxH: number, minPx: number, maxPx: number, fontFamily: string = BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT): number {
  const lo = Math.max(4, minPx);
  const hi = Math.max(lo, maxPx);
  let best = lo;
  let low = lo;
  let high = hi;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = boardBuildCanvasFontSpec(mid, fontFamily);
    const w = ctx.measureText(text).width;
    const h = mid * 1.2;
    if (w <= maxW && h <= maxH) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

/** @emoji ✂️ Single-line tail truncation with `…` so measured width ≤ `maxWidth` (`ctx.font` must be set). */
export function boardEllipsisTextToWidth(ctx: CanvasTextMeasuring, text: string, maxWidth: number): string {
  if (text === "") {
    return text;
  }
  const ell = "…";
  const widthOf = (value: string): number => ctx.measureText(value).width;
  if (widthOf(text) <= maxWidth) {
    return text;
  }
  if (widthOf(ell) > maxWidth) {
    return "";
  }
  let low = 0;
  let high = text.length;
  let best = 0;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    const candidate = `${text.slice(0, mid)}${ell}`;
    if (widthOf(candidate) <= maxWidth) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best === 0 ? ell : `${text.slice(0, best)}${ell}`;
}

/** @emoji 📍 Maps a node-centered box to `fillText` origin + canvas alignment (layout px). */
export function boardNodeTextPlacementAnchor(centerX: number, centerY: number, maxW: number, maxH: number, alignment: BoardNodeTextAlignment): { fillX: number; fillY: number; textAlign: CanvasTextAlign; textBaseline: CanvasTextBaseline } {
  const halfW = maxW / 2;
  const halfH = maxH / 2;
  const left = centerX - halfW;
  const right = centerX + halfW;
  const top = centerY - halfH;
  const bottom = centerY + halfH;
  switch (alignment) {
    case "nw":
      return { fillX: left, fillY: top, textAlign: "left", textBaseline: "top" };
    case "n":
      return { fillX: centerX, fillY: top, textAlign: "center", textBaseline: "top" };
    case "ne":
      return { fillX: right, fillY: top, textAlign: "right", textBaseline: "top" };
    case "w":
      return { fillX: left, fillY: centerY, textAlign: "left", textBaseline: "middle" };
    case "c":
      return { fillX: centerX, fillY: centerY, textAlign: "center", textBaseline: "middle" };
    case "e":
      return { fillX: right, fillY: centerY, textAlign: "right", textBaseline: "middle" };
    case "sw":
      return { fillX: left, fillY: bottom, textAlign: "left", textBaseline: "bottom" };
    case "s":
      return { fillX: centerX, fillY: bottom, textAlign: "center", textBaseline: "bottom" };
    case "se":
      return { fillX: right, fillY: bottom, textAlign: "right", textBaseline: "bottom" };
    default: {
      const _: never = alignment;
      return _;
    }
  }
}

/** @emoji 🧾 Minimal 2D canvas text metrics surface for {@link boardFitTextFontPx}. */
export type CanvasTextMeasuring = Pick<CanvasRenderingContext2D, "font" | "measureText">;

/** 🧭 Builds a cubic whose control arms leave/arrive along circle normals (radial), not along handle tangents. */
export function computeEdgeBezier(sourceHandle: BoardSceneHandle, targetHandle: BoardSceneHandle): CubicBezierCurve {
  const sourcePoint = sourceHandle.position;
  const targetPoint = targetHandle.position;
  const sourceCenter = { x: sourceHandle.node.x, y: sourceHandle.node.y };
  const targetCenter = { x: targetHandle.node.x, y: targetHandle.node.y };
  const flat = boardComputeEdgeBezier(sourcePoint.x, sourcePoint.y, sourceCenter.x, sourceCenter.y, targetPoint.x, targetPoint.y, targetCenter.x, targetCenter.y);
  return {
    p0: { x: flat[0], y: flat[1] },
    p1: { x: flat[2], y: flat[3] },
    p2: { x: flat[4], y: flat[5] },
    p3: { x: flat[6], y: flat[7] },
  };
}

/** 🧵 Same radial cubic as {@link computeEdgeBezier} but the far end is a free world point (transient link / {@link Wire}). */
export function computeWireBezier(sourceHandle: BoardSceneHandle, endWorld: Point): CubicBezierCurve {
  const sourcePoint = sourceHandle.position;
  const sourceCenter = { x: sourceHandle.node.x, y: sourceHandle.node.y };
  const flat = boardComputeEdgeBezier(sourcePoint.x, sourcePoint.y, sourceCenter.x, sourceCenter.y, endWorld.x, endWorld.y, endWorld.x, endWorld.y);
  return {
    p0: { x: flat[0], y: flat[1] },
    p1: { x: flat[2], y: flat[3] },
    p2: { x: flat[4], y: flat[5] },
    p3: { x: flat[6], y: flat[7] },
  };
}
//#endregion 🔖Utilities

//#region 🔖Stores
class SnapshotStore<TSnapshot> {
  private listeners = new Set<() => void>();

  constructor(private snapshot: TSnapshot) {}

  getSnapshot = (): TSnapshot => this.snapshot;

  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  };

  setSnapshot(nextSnapshot: TSnapshot, equal: (left: TSnapshot, right: TSnapshot) => boolean): void {
    if (equal(this.snapshot, nextSnapshot)) {
      return;
    }
    this.snapshot = nextSnapshot;
    for (const listener of this.listeners) {
      listener();
    }
  }
}

class TypedEmitter<TEvents extends object> {
  private listeners = new Map<keyof TEvents, Set<(payload: TEvents[keyof TEvents]) => void>>();

  on<TKey extends keyof TEvents>(name: TKey, handler: (payload: TEvents[TKey]) => void): () => void {
    const handlers = (this.listeners.get(name) ?? new Set()) as Set<(payload: TEvents[TKey]) => void>;
    handlers.add(handler);
    this.listeners.set(name, handlers as Set<(payload: TEvents[keyof TEvents]) => void>);
    return () => {
      handlers.delete(handler);
      if (handlers.size === 0) {
        this.listeners.delete(name);
      }
    };
  }

  emit<TKey extends keyof TEvents>(name: TKey, payload: TEvents[TKey]): void {
    const handlers = this.listeners.get(name);
    if (!handlers) {
      return;
    }
    for (const handler of handlers) {
      (handler as (value: TEvents[TKey]) => void)(payload);
    }
  }
}
//#endregion 🔖Stores

//#region 🔖Objects
/** 🧱 Base retained board object with scene identity and shared flags. */
export class BoardObject {
  draggable: boolean;
  highlighted: boolean;
  parent: BoardScene | null = null;
  selected: boolean;
  style: string | null;
  userData: Record<string, unknown>;
  visible: boolean;

  protected renderer: BoardRenderer | null = null;

  constructor(
    public readonly id: string,
    options: BoardObjectOptions,
  ) {
    this.draggable = options.draggable ?? false;
    this.highlighted = options.highlighted ?? false;
    this.selected = options.selected ?? false;
    this.style = options.style ?? null;
    this.userData = { ...(options.userData ?? {}) };
    this.visible = options.visible ?? true;
  }

  get kind(): BoardObjectKind {
    throw new Error("BoardObject.kind must be implemented by subclasses.");
  }

  attachRenderer(renderer: BoardRenderer | null): void {
    this.renderer = renderer;
  }

  dispose(): void {
    this.parent?.remove(this);
  }
}

/** 🟠 Board node: circle (radius) or axis-aligned rectangle (width × height) centered at (x,y). */
export class BoardSceneNode extends BoardObject {
  handles: BoardSceneHandle[] = [];
  height: number;
  radius: number;
  shape: "circle" | "rectangle";
  text: string | null;
  /** @emoji 🏷️ Runtime icon string forwarded to WASM detail LOD (`typst:` / `$…`, `emoji:`, `data:` / raster data URLs, baked catalog id, or inline SVG). */
  iconKind: string | null;
  /** @emoji 📏 When true, {@link BoardRenderer} scales overlay text to the node interior (always drawn at node center). */
  textAutofit: boolean;
  /** @emoji 🧭 When not autofitting, anchors single-line caption inside the node-centered box. */
  textAlignment: BoardNodeTextAlignment;
  /** @emoji 🔤 CSS font family string for overlay captions. */
  textFontFamily: string;
  /** @emoji 🔤 Font size in layout px when not autofitting. */
  textFontSize: number;
  width: number;
  x: number;
  y: number;
  /** @emoji 🌳 When true, {@link computeBoardGraphObservationSnapshot} treats each {@link Edge} as parent {@link Edge.source} → child {@link Edge.target} along node ids. */
  root: boolean;
  /** @emoji 🧩 Semantic node-kind id forwarded to WASM for catalog defaults and compatibility. */
  nodeKind: string;

  constructor(options: BoardSceneNodeOptions) {
    super(options.id, {
      draggable: options.draggable ?? true,
      selected: options.selected,
      style: options.style,
      userData: options.userData,
      visible: options.visible,
    });
    this.root = options.root === true;
    const nk = typeof options.nodeKind === "string" ? options.nodeKind.trim() : "";
    this.nodeKind = nk;
    this.x = options.x;
    this.y = options.y;
    this.text = options.text ?? null;
    this.iconKind = typeof options.iconKind === "string" && options.iconKind.trim() !== "" ? options.iconKind.trim() : null;
    this.textAutofit = options.textAutofit ?? false;
    this.textAlignment = options.textAlignment ?? BOARD_NODE_TEXT_ALIGNMENT_DEFAULT;
    this.textFontFamily = typeof options.textFontFamily === "string" && options.textFontFamily.trim() !== "" ? options.textFontFamily.trim() : BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT;
    const rawSize = options.textFontSize;
    this.textFontSize = typeof rawSize === "number" && Number.isFinite(rawSize) && rawSize > 0 ? rawSize : BOARD_NODE_TEXT_FONT_PX_DEFAULT;
    if (options.shape === "rectangle") {
      this.shape = "rectangle";
      this.width = options.width;
      this.height = options.height;
      this.radius = 0;
    } else {
      this.shape = "circle";
      this.radius = options.radius;
      this.width = 0;
      this.height = 0;
    }
    for (const handle of options.handles ?? []) {
      this.attachHandle(handle);
    }
  }

  get kind(): BoardObjectKind {
    return "node";
  }

  setPosition(x: number, y: number): this {
    this.x = x;
    this.y = y;
    return this;
  }

  setRadius(radius: number): this {
    if (this.shape !== "circle") {
      return this;
    }
    this.radius = radius;
    return this;
  }

  setRectangleSize(width: number, height: number): this {
    if (this.shape !== "rectangle") {
      return this;
    }
    this.width = width;
    this.height = height;
    return this;
  }

  setRoot(root: boolean): this {
    this.root = root;
    return this;
  }

  setText(text: string | null): this {
    this.text = text;
    return this;
  }

  setTextAutofit(value: boolean): this {
    this.textAutofit = value;
    return this;
  }

  attachHandle(handle: BoardSceneHandle): void {
    if (this.handles.includes(handle)) {
      return;
    }
    handle.node = this;
    this.handles.push(handle);
  }

  detachHandle(handle: BoardSceneHandle): void {
    this.handles = this.handles.filter((candidate) => candidate !== handle);
  }
}

/** 🟣 Tangent handle anchored to a node boundary at a polar angle. */
export class BoardSceneHandle extends BoardObject {
  angle: number;
  /** @emoji 🎨 CSS `#…` fill override for the WASM host; `null` uses catalog / theme only. */
  color: string | null;
  /** @emoji 🔗 Semantic kind for ordered link compatibility on the host (JSON `handleKind`). */
  handleKind: string;
  node: BoardSceneNode;
  radius: number;

  constructor(options: BoardSceneHandleOptions) {
    super(options.id, options);
    this.angle = options.angle;
    const ck = String(options.handleKind ?? "").trim();
    this.handleKind = ck;
    const rawC = options.color;
    const cs = rawC === undefined || rawC === null ? "" : String(rawC).trim();
    this.color = cs !== "" ? cs : null;
    this.node = options.node;
    this.radius = options.radius ?? 8;
    this.node.attachHandle(this);
  }

  get kind(): BoardObjectKind {
    return "handle";
  }

  get position(): Point {
    return computeHandlePosition(this.node, this.angle);
  }

  get tangent(): Point {
    return computeHandleTangent(this.angle);
  }

  setAngle(angle: number): this {
    this.angle = angle;
    return this;
  }
}

/** 🪢 Cubic edge between two boundary handles; control arms stay on the radial **outside** of each node so the stroke does not cut through the disk interior. {@link Edge.source} is the parent-side anchor and {@link Edge.target} the child-side anchor for {@link Node.root} subtree reachability. */
export class BoardSceneEdge extends BoardObject {
  /** @emoji 🧩 Semantic edge-kind id forwarded to WASM. */
  edgeKind: string;
  source: BoardSceneHandle;
  target: BoardSceneHandle;

  constructor(options: BoardSceneEdgeOptions) {
    super(options.id, options);
    this.source = options.source;
    this.target = options.target;
    const ek = typeof options.edgeKind === "string" ? options.edgeKind.trim() : "";
    this.edgeKind = ek;
  }

  get kind(): BoardObjectKind {
    return "edge";
  }

  get curve(): CubicBezierCurve {
    return computeEdgeBezier(this.source, this.target);
  }

  setEndpoints(sourceHandle: BoardSceneHandle, targetHandle: BoardSceneHandle): this {
    this.source = sourceHandle;
    this.target = targetHandle;
    return this;
  }
}

/** 🧵 Transient cubic from one {@link Handle} to another handle or a free world point (in‑progress link drag). */
export class BoardSceneWire extends BoardObject {
  endX: number | null;
  endY: number | null;
  source: BoardSceneHandle;
  target: BoardSceneHandle | null;
  /** @emoji 🧩 Semantic wire-kind id forwarded to WASM. */
  wireKind: string;

  constructor(options: BoardSceneWireOptions) {
    super(options.id, options);
    this.source = options.source;
    this.target = options.target;
    const wk = typeof options.wireKind === "string" ? options.wireKind.trim() : "";
    this.wireKind = wk;
    const ex = options.endX;
    const ey = options.endY;
    this.endX = typeof ex === "number" && Number.isFinite(ex) ? ex : null;
    this.endY = typeof ey === "number" && Number.isFinite(ey) ? ey : null;
  }

  get kind(): BoardObjectKind {
    return "wire";
  }

  get curve(): CubicBezierCurve {
    if (this.target) {
      return computeEdgeBezier(this.source, this.target);
    }
    const x = this.endX ?? this.source.position.x;
    const y = this.endY ?? this.source.position.y;
    return computeWireBezier(this.source, { x, y });
  }

  setAnchors(sourceHandle: BoardSceneHandle, targetHandle: BoardSceneHandle | null, endWorld?: Point | null): this {
    this.source = sourceHandle;
    this.target = targetHandle;
    if (endWorld && Number.isFinite(endWorld.x) && Number.isFinite(endWorld.y)) {
      this.endX = endWorld.x;
      this.endY = endWorld.y;
    } else if (!targetHandle) {
      this.endX = null;
      this.endY = null;
    } else {
      this.endX = null;
      this.endY = null;
    }
    return this;
  }
}
//#endregion 🔖Objects

type BoardNodeObject = BoardSceneNode;
type BoardHandleObject = BoardSceneHandle;
type BoardEdgeObject = BoardSceneEdge;
type BoardWireObject = BoardSceneWire;

//#region 🔖Scene
/** 🧭 Retained scene catalog owning nodes, handles, edges, and wires by stable id. */
export class BoardScene {
  readonly edges = new Map<string, BoardSceneEdge>();
  readonly handles = new Map<string, BoardSceneHandle>();
  readonly nodes = new Map<string, BoardSceneNode>();
  readonly wires = new Map<string, BoardSceneWire>();

  constructor(private renderer: BoardRenderer | null = null) {}

  setRenderer(renderer: BoardRenderer | null): void {
    this.renderer = renderer;
    for (const object of this.getAllObjects()) {
      object.attachRenderer(renderer);
    }
  }

  add(object: BoardObject): this {
    if (object instanceof BoardSceneNode) {
      const prior = this.nodes.get(object.id);
      if (prior && prior !== object) {
        this.remove(prior);
      }
      this.nodes.set(object.id, object);
      object.parent = this;
      object.attachRenderer(this.renderer);
      for (const handle of object.handles) {
        this.add(handle);
      }
      this.renderer?.markDirty();
      return this;
    }

    if (object instanceof BoardSceneHandle) {
      if (!this.nodes.has(object.node.id)) {
        this.add(object.node);
      }
      this.handles.set(object.id, object);
      object.parent = this;
      object.attachRenderer(this.renderer);
      object.node.attachHandle(object);
      this.renderer?.markDirty();
      return this;
    }

    if (object instanceof BoardSceneWire) {
      if (!this.nodes.has(object.source.node.id)) {
        this.add(object.source.node);
      }
      if (object.target && !this.nodes.has(object.target.node.id)) {
        this.add(object.target.node);
      }
      const wire = object as Wire;
      const existed = this.wires.has(wire.id);
      this.wires.set(wire.id, wire);
      wire.parent = this;
      wire.attachRenderer(this.renderer);
      if (!existed) {
        this.renderer?.emit("wireCreate", {
          endX: wire.endX,
          endY: wire.endY,
          id: wire.id,
          source: wire.source.id,
          target: wire.target?.id ?? null,
          wireKind: wire.wireKind,
        });
      }
      this.renderer?.markDirty();
      return this;
    }

    this.edges.set(object.id, object as Edge);
    object.parent = this;
    object.attachRenderer(this.renderer);
    this.renderer?.emit("edgeCreate", { id: object.id, source: (object as Edge).source.id, target: (object as Edge).target.id });
    this.renderer?.markDirty();
    return this;
  }

  /** @emoji 🔗 Inserts a WASM‑drained edge without emitting {@link BoardEventMap.edgeCreate} (the renderer applies that once per drain row). */
  ingestWasmEdge(edge: BoardSceneEdge): this {
    this.edges.set(edge.id, edge);
    edge.parent = this;
    edge.attachRenderer(this.renderer);
    this.renderer?.markDirty();
    return this;
  }

  remove(object: BoardObject): this {
    if (object instanceof BoardSceneNode) {
      for (const edge of Array.from(this.edges.values())) {
        if (edge.source.node === object || edge.target.node === object) {
          this.renderer?.clearWasmHostAuthorshipForEdge(edge.id);
          this.remove(edge);
        }
      }
      for (const wire of Array.from(this.wires.values())) {
        if (wire.source.node === object || wire.target?.node === object) {
          this.remove(wire);
        }
      }
      for (const handle of Array.from(object.handles)) {
        this.remove(handle);
      }
      this.renderer?.emitSceneDeleteEvent("nodeDelete", { id: object.id });
      this.nodes.delete(object.id);
      object.parent = null;
      object.attachRenderer(null);
      this.renderer?.evictNodeAuthoringPosition(object.id);
      this.renderer?.markDirty();
      return this;
    }

    if (object instanceof BoardSceneHandle) {
      for (const edge of Array.from(this.edges.values())) {
        if (edge.source === object || edge.target === object) {
          this.renderer?.clearWasmHostAuthorshipForEdge(edge.id);
          this.remove(edge);
        }
      }
      for (const wire of Array.from(this.wires.values())) {
        if (wire.source === object || wire.target === object) {
          this.remove(wire);
        }
      }
      object.node.detachHandle(object);
      this.handles.delete(object.id);
      object.parent = null;
      object.attachRenderer(null);
      this.renderer?.markDirty();
      return this;
    }

    if (object instanceof BoardSceneWire) {
      this.renderer?.emitSceneDeleteEvent("wireDestroy", { id: object.id });
      this.wires.delete(object.id);
      object.parent = null;
      object.attachRenderer(null);
      this.renderer?.markDirty();
      return this;
    }

    this.renderer?.emitSceneDeleteEvent("edgeDelete", { id: object.id });
    this.edges.delete(object.id);
    object.parent = null;
    object.attachRenderer(null);
    this.renderer?.markDirty();
    return this;
  }

  clear(): void {
    const runSilent =
      this.renderer?.runWithoutSceneDeleteEvents.bind(this.renderer) ??
      ((fn: () => void) => {
        fn();
      });
    runSilent(() => {
      for (const edge of Array.from(this.edges.values())) {
        this.remove(edge);
      }
      for (const wire of Array.from(this.wires.values())) {
        this.remove(wire);
      }
      for (const handle of Array.from(this.handles.values())) {
        this.remove(handle);
      }
      for (const node of Array.from(this.nodes.values())) {
        this.remove(node);
      }
    });
  }

  getObjectById(id: string): BoardObject | undefined {
    return this.nodes.get(id) ?? this.handles.get(id) ?? this.edges.get(id) ?? this.wires.get(id);
  }

  getAllObjects(): BoardObject[] {
    return [...this.nodes.values(), ...this.handles.values(), ...this.edges.values(), ...this.wires.values()];
  }
}
//#endregion 🔖Scene

//#region 🔖DirectedGraphObservation
/** @emoji 🧮 Immutable snapshot for {@link BoardRenderer} hierarchy callbacks (roots + directed reachability along {@link Edge.source}→{@link Edge.target}). */
export interface BoardGraphObservationSnapshot {
  childEdgeIds: string[];
  childNodeIds: string[];
  edgeSigById: Map<string, string>;
  nodeSigById: Map<string, string>;
  parentEdgeIds: string[];
  rootIds: string[];
  wireSigById: Map<string, string>;
}

function sortIds(ids: Iterable<string>): string[] {
  return [...ids].sort();
}

function sortedStringArraysEqual(a: string[], b: string[]): boolean {
  if (a.length !== b.length) {
    return false;
  }
  for (let i = 0; i < a.length; i += 1) {
    if (a[i] !== b[i]) {
      return false;
    }
  }
  return true;
}

function boardGraphNodeSig(node: BoardSceneNode): string {
  return JSON.stringify({
    draggable: node.draggable,
    height: node.height,
    iconKind: node.iconKind,
    id: node.id,
    radius: node.radius,
    root: node.root,
    shape: node.shape,
    style: node.style,
    text: node.text,
    textAlignment: node.textAlignment,
    textAutofit: node.textAutofit,
    textFontFamily: node.textFontFamily,
    textFontSize: node.textFontSize,
    visible: node.visible,
    width: node.width,
    x: node.x,
    y: node.y,
  });
}

function boardGraphEdgeSig(edge: BoardSceneEdge): string {
  return JSON.stringify({
    source: edge.source.id,
    id: edge.id,
    selected: edge.selected,
    style: edge.style,
    target: edge.target.id,
    visible: edge.visible,
  });
}

function boardGraphWireSig(wire: BoardSceneWire): string {
  return JSON.stringify({
    endX: wire.endX,
    endY: wire.endY,
    id: wire.id,
    selected: wire.selected,
    source: wire.source.id,
    style: wire.style,
    target: wire.target?.id ?? null,
    visible: wire.visible,
    wireKind: wire.wireKind,
  });
}

/** @emoji 🌳 Builds subtree membership: BFS from every {@link Node.root} following directed edges from parent handle node to child handle node. */
export function computeBoardGraphObservationSnapshot(scene: BoardScene): BoardGraphObservationSnapshot {
  const rootIds = sortIds([...scene.nodes.values()].filter((n) => n.root).map((n) => n.id));
  const rootSet = new Set(rootIds);
  const reachable = new Set<string>(rootSet);
  const queue = [...rootSet];
  while (queue.length > 0) {
    const u = queue.shift()!;
    for (const edge of scene.edges.values()) {
      if (edge.source.node.id !== u) {
        continue;
      }
      const v = edge.target.node.id;
      if (!reachable.has(v)) {
        reachable.add(v);
        queue.push(v);
      }
    }
  }
  const childNodeIds = sortIds([...reachable].filter((id) => !rootSet.has(id)));
  const childEdgeIds = sortIds([...scene.edges.values()].filter((e) => reachable.has(e.source.node.id) && reachable.has(e.target.node.id)).map((e) => e.id));
  const parentEdgeIds = sortIds([...scene.edges.values()].filter((e) => rootSet.has(e.source.node.id)).map((e) => e.id));
  const nodeSigById = new Map<string, string>();
  for (const node of scene.nodes.values()) {
    nodeSigById.set(node.id, boardGraphNodeSig(node));
  }
  const edgeSigById = new Map<string, string>();
  for (const edge of scene.edges.values()) {
    edgeSigById.set(edge.id, boardGraphEdgeSig(edge));
  }
  const wireSigById = new Map<string, string>();
  for (const wire of scene.wires.values()) {
    wireSigById.set(wire.id, boardGraphWireSig(wire));
  }
  return { childEdgeIds, childNodeIds, edgeSigById, nodeSigById, parentEdgeIds, rootIds, wireSigById };
}
//#endregion 🔖DirectedGraphObservation

/** @emoji 🧯 Normalizes WebGPU errors for `data-board-surface-failure` (E2E + local debugging). */
function summarizeRasterSurfaceFailure(err: unknown): string {
  if (err instanceof Error) {
    return `${err.name}: ${err.message}`.slice(0, 512);
  }
  if (typeof err === "string") {
    return err.slice(0, 512);
  }
  try {
    return JSON.stringify(err).slice(0, 512);
  } catch {
    return String(err).slice(0, 512);
  }
}

function boardAbbreviateCaption(raw: string, maxChars: number): string {
  return raw.length <= maxChars ? raw : `${raw.slice(0, Math.max(1, maxChars - 1))}…`;
}

/** @emoji 🏷️ Abbreviated node caption for the text overlay canvas, or null when the LOD band hides node labels. */
export function boardTextOverlayCaptionForLod(raw: string, lod: BoardDrawLodKind, iconKind: string | null): string | null {
  const t = raw.trim();
  if (t === "") {
    return null;
  }
  if (lod === "minimap" || lod === "overview") {
    return null;
  }
  if (lod === "compact" || lod === "normal") {
    return boardAbbreviateCaption(t, 8);
  }
  if (lod === "detail") {
    return boardAbbreviateCaption(t, (iconKind?.trim() ?? "") !== "" ? 8 : 10);
  }
  if (lod === "micro") {
    return boardAbbreviateCaption(t, 12);
  }
  return null;
}

/** @emoji 🏷️ Abbreviated handle caption for the text overlay canvas, or null when the LOD band hides handle labels. */
export function boardHandleOverlayCaptionForLod(raw: string, lod: BoardDrawLodKind): string | null {
  const t = raw.trim();
  if (t === "") {
    return null;
  }
  if (lod !== "detail" && lod !== "micro") {
    return null;
  }
  return boardAbbreviateCaption(t, lod === "detail" ? 6 : 8);
}

/** @emoji 🧩 Resolves a handle-kind catalog label for overlay captions. */
export function boardHandleKindOverlayLabel(handleKind: string, catalogs: KindCatalogBundle): string {
  const id = handleKind.trim();
  if (id === "") {
    return "";
  }
  for (const row of catalogs.handles ?? []) {
    if (row.id === id) {
      return (row.name ?? id).trim() || id;
    }
  }
  return id;
}

//#region 🔖Renderer
/** 🎛️ Slim imperative shell: DOM/RAF, one {@link BoardSession} (WASM `BoardHost` + optional GPU), JSON scene sync, and event drains mirroring WASM onto the JS scene graph for React/tests. */
export class BoardRenderer {
  static activeRenderer: BoardRenderer | null = null;

  readonly camera: CameraState = { ...DEFAULT_CAMERA };
  readonly scene: BoardScene;
  readonly session: BoardSession;
  /** @emoji 🔗 Edge ids created by the WASM host (link gesture) until the same id appears in React `children`; merged into the descriptor passed to {@link syncBoardScene}. */
  readonly wasmHostAuthoredEdgeIds = new Set<string>();
  /** @emoji 🔗 Endpoint ids for each {@link BoardRenderer.wasmHostAuthoredEdgeIds} entry so merge can rebuild the descriptor if the scene edge was removed transiently (e.g. handle purge ordering). */
  readonly wasmHostAuthoredLinkByEdgeId = new Map<string, { source: string; target: string }>();

  private batchDepth = 0;
  /** @emoji 🔇 While >0, {@link BoardScene.remove} does not emit delete events (dispose / JSX resync). */
  private suppressSceneDeleteEvents = 0;
  /** @emoji 🔁 Nesting depth for {@link BoardRenderer.render}; defers {@link BoardRenderer.invalidate} so ResizeObserver / layout cannot re-enter WASM during `renderFrame` (`borrow_fail`). */
  private renderPipelineDepth = 0;
  /** @emoji ⛓️ Tracks async WASM session borrows such as {@link BoardSession.attach_canvas} so sync probes like `gpuReady()` do not re-enter the same `RefCell`. */
  private wasmSessionBorrowDepth = 0;
  /** @emoji 🧷 Tracks `pushSceneToWasmDriver` + {@link BoardSession.renderFrame} where `device.poll` may synchronously re-enter JS while WASM still borrows `BoardSession`. */
  private wasmGpuFrameDepth = 0;

  /** @emoji 🚧 True while wasm-bindgen holds `&mut BoardSession`; any other JS→wasm call on this session must defer (see commit 379 + follow-up). */
  private wasmSessionCallBlockedForReentry(): boolean {
    return this.wasmSessionBorrowDepth > 0 || this.wasmGpuFrameDepth > 0;
  }

  /** @emoji 🔇 Runs `fn` without {@link BoardEventMap.nodeDelete} / edge / wire delete emissions (internal teardown or descriptor resync). */
  runWithoutSceneDeleteEvents(fn: () => void): void {
    this.suppressSceneDeleteEvents += 1;
    try {
      fn();
    } finally {
      this.suppressSceneDeleteEvents -= 1;
    }
  }

  /** @emoji 📣 Forwards structural delete events to play/fixture listeners unless suppressed or disposed. */
  emitSceneDeleteEvent<TKey extends "nodeDelete" | "edgeDelete" | "wireDestroy">(name: TKey, payload: BoardEventMap[TKey]): void {
    if (this.suppressSceneDeleteEvents > 0 || this.isDisposed) {
      return;
    }
    this.emit(name, payload);
  }
  /** @emoji 💾 Last `gpuReady` snapshot; used while {@link BoardRenderer.wasmGpuFrameDepth} is non-zero to avoid `RefCell` conflicts with in-flight `renderFrame`. */
  private cachedWasmGpuReady = false;
  private cameraStore = new SnapshotStore<CameraState>({ ...DEFAULT_CAMERA });
  private drawLodStore = new SnapshotStore<BoardDrawLodKind>(resolveBoardLodLabelFromThresholds(DEFAULT_CAMERA.zoom, DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS));
  private canvas: HTMLCanvasElement | null;
  private dpr = 1;
  private emitter = new TypedEmitter<BoardEventMap>();
  private frameListeners = new Set<FrameListener>();
  private hoveredId: string | null = null;
  private lastPointerClientX = 0;
  private lastPointerClientY = 0;
  private lastPointerScreenX = 0;
  private lastPointerScreenY = 0;
  private invalidated = true;
  private isDisposed = false;
  private lastRenderTimestamp: number | null = null;
  private rafId: number | null = null;
  private selectionIds = new Set<string>();
  private selectionOptions: ResolvedBoardSelectionOptions;
  private selectionStore = new SnapshotStore<BoardSelectionSnapshot>({ ids: [] });
  private preselectIds = new Set<string>();
  private preselectRemovedIds = new Set<string>();
  private preselectStore = new SnapshotStore<BoardPreselectSnapshot>(BOARD_PRESELECT_EMPTY);
  private styles = new Map<string, BoardStyle>(Object.entries(DEFAULT_STYLES));
  private gpuSurfaceErrorDetail = "";
  private gpuSurfaceInitPromise: Promise<void> | null = null;
  private gpuSurfacePresentedFrame = false;
  private gpuSurfaceUnavailable = false;
  private lastPushedDescriptorJson: string | null = null;
  private lastVelloThemeJson = "";
  private lastDescriptorPushDeferred = false;
  private kindCompatJson = "[]";
  private kindCatalogsBundle: KindCatalogBundle = DEFAULT_KIND_CATALOG_BUNDLE;
  private kindCatalogsJson = serializeKindCatalogBundle(DEFAULT_KIND_CATALOG_BUNDLE);
  private lastPushedKindCatalogsJson: string | null = null;
  private wasmHostSceneMergeResyncStore = new SnapshotStore<number>(0);
  private lastNodeAuthoringPositionById = new Map<string, { x: number; y: number }>();
  private suppressSceneToWasmPush = false;
  private graphObservationFlushPending = false;
  private lastGraphObservation: BoardGraphObservationSnapshot | null = null;
  private width = 1;
  private height = 1;
  private textOverlayCanvas: HTMLCanvasElement | null = null;

  private lodZoomThresholds: BoardLodZoomThresholds = { ...DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS };
  private automaticLod = true;
  private forcedDrawLodLabel: BoardDrawLodKind | undefined = undefined;
  private gridSnapEnabled = false;
  private gridFactor = DEFAULT_BOARD_GRID_FACTOR;
  private lastLodThresholdsJsonForWasm: string | null = null;
  private lastAutomaticLodForWasm: boolean | null = null;
  private lastForcedDrawLodLabelForWasm: string | null = null;
  private lastGridSnapEnabledForWasm: boolean | null = null;
  private lastGridFactorForWasm: number | null = null;

  worldRasterTiling: WorldRasterTilingKind;

  constructor(
    options: {
      canvas?: HTMLCanvasElement | null;
      renderMode?: RenderMode;
      selection?: BoardSelectionOptions;
      worldRasterTiling?: WorldRasterTilingKind;
      lodZoomThresholds?: BoardLodZoomThresholds;
      /** @emoji 📶 When true (default), WASM draw LOD follows camera zoom; when false, {@link lod} pins the tier when set. */
      automaticLod?: boolean;
      /** @emoji 📶 Pinned draw LOD when `automaticLod` is false. */
      lod?: BoardDrawLodKind;
      gridSnapEnabled?: boolean;
      gridFactor?: number;
    } = {},
  ) {
    this.canvas = options.canvas ?? null;
    this.renderMode = options.renderMode ?? (this.canvas ? "main-thread" : "headless-test");
    this.selectionOptions = resolveSelectionOptions(options.selection);
    this.worldRasterTiling = options.worldRasterTiling ?? "world-clip";
    this.lodZoomThresholds = options.lodZoomThresholds
      ? {
          minimapMaxZoom: options.lodZoomThresholds.minimapMaxZoom,
          overviewMaxZoom: options.lodZoomThresholds.overviewMaxZoom,
          compactMaxZoom: options.lodZoomThresholds.compactMaxZoom,
          normalMaxZoom: options.lodZoomThresholds.normalMaxZoom,
          detailMaxZoom: options.lodZoomThresholds.detailMaxZoom,
        }
      : { ...DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS };
    this.gridSnapEnabled = options.gridSnapEnabled ?? false;
    const gf = options.gridFactor;
    this.gridFactor = typeof gf === "number" && Number.isFinite(gf) && gf > 0 && gf <= 1e6 ? gf : DEFAULT_BOARD_GRID_FACTOR;
    this.automaticLod = options.automaticLod ?? true;
    const optLod = options.lod;
    this.forcedDrawLodLabel = !this.automaticLod && optLod !== undefined && isBoardDrawLodKind(optLod) ? optLod : undefined;
    if (this.automaticLod) {
      this.forcedDrawLodLabel = undefined;
    }
    this.scene = new BoardScene(this);
    this.session = new BoardSession();
    const initialSel = this.selectionOptions;
    this.session.setSelectionOptions(initialSel.method, boardSelectionModeForHost(initialSel.mode), initialSel.targets.nodes, initialSel.targets.edges, initialSel.targets.handles);
    this.session.setHandleLinkCompatJson(this.kindCompatJson);
    try {
      this.session.setKindCatalogsJson(this.kindCatalogsJson);
    } catch (err) {
      console.error("[DEBUG] setKindCatalogsJson failed during BoardRenderer init", err);
    }
    this.lastPushedKindCatalogsJson = this.kindCatalogsJson;
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.attachCanvasListeners();
    if (this.canvas) {
      (this.canvas as BoardCanvasElement).__boardRenderer = this;
      const initialWidth = this.canvas.clientWidth || this.canvas.width || 1;
      const initialHeight = this.canvas.clientHeight || this.canvas.height || 1;
      this.setSize(initialWidth, initialHeight, globalThis.devicePixelRatio || 1);
    }
    this.lastGraphObservation = computeBoardGraphObservationSnapshot(this.scene);
  }

  readonly renderMode: RenderMode;

  /** @emoji 🔗 Subscribes to an epoch bumped when WASM event drains mutate edges/nodes so the React host can re-merge {@link BoardRenderer.wasmHostAuthoredEdgeIds} into JSX sync without waiting for `children` identity changes. */
  subscribeWasmHostSceneMergeResync = (listener: () => void): (() => void) => this.wasmHostSceneMergeResyncStore.subscribe(listener);

  /** @emoji 🔗 Snapshot for {@link BoardRenderer.subscribeWasmHostSceneMergeResync} (use with `useSyncExternalStore`). */
  getWasmHostSceneMergeResyncEpoch = (): number => this.wasmHostSceneMergeResyncStore.getSnapshot();

  get selection(): {
    getSnapshot: () => BoardSelectionSnapshot;
    has: (id: string) => boolean;
    ids: string[];
    subscribe: (listener: () => void) => () => void;
  } {
    return {
      getSnapshot: this.getSelectionSnapshot,
      has: (id) => this.selectionIds.has(id),
      ids: this.selectionStore.getSnapshot().ids,
      subscribe: this.subscribeSelection,
    };
  }

  /** @emoji 🪟 Binds a stacked 2D canvas used for node captions (WebGPU path does not draw text). */
  attachTextOverlayCanvas(canvas: HTMLCanvasElement | null): void {
    this.textOverlayCanvas = canvas;
    this.invalidate();
  }

  subscribeSelection = (listener: () => void): (() => void) => this.selectionStore.subscribe(listener);

  getSelectionSnapshot = (): BoardSelectionSnapshot => this.selectionStore.getSnapshot();

  get preselection(): {
    getSnapshot: () => BoardPreselectSnapshot;
    subscribe: (listener: () => void) => () => void;
  } {
    return {
      getSnapshot: this.getPreselectSnapshot,
      subscribe: this.subscribePreselect,
    };
  }

  subscribePreselect = (listener: () => void): (() => void) => this.preselectStore.subscribe(listener);

  getPreselectSnapshot = (): BoardPreselectSnapshot => this.preselectStore.getSnapshot();

  /** @emoji ✅ Replaces the active selection set and syncs `selected` flags on scene objects. */
  setSelectionIds(ids: Iterable<string>): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      this.updateSelection(ids, true);
      return;
    }
    this.pushSceneToWasmDriver();
    this.session.setSelectionIdsJson(JSON.stringify([...ids]));
    this.applyWasmDrainToScene(this.session.drainEventsJson());
  }

  /** @emoji 🔇 Controlled sync: updates WASM + JS selection without emitting `select`. */
  setSelectionIdsSilent(ids: Iterable<string>): void {
    const payload = JSON.stringify([...ids]);
    if (this.wasmSessionCallBlockedForReentry()) {
      this.updateSelection(ids, false);
      return;
    }
    this.pushSceneToWasmDriver();
    try {
      this.session.setSelectionIdsJsonSilent(payload);
    } catch (err) {
      console.error("[DEBUG] setSelectionIdsJsonSilent failed", err);
    }
    this.updateSelection(ids, false);
    this.markDirty();
  }

  /** @emoji 👁️ Controlled sync: mirrors area-select preview chrome on this canvas without emitting `preselect`. */
  syncPreselectionSilent(snapshot: BoardPreselectSnapshot): void {
    const normalized = normalizeBoardPreselectProp(snapshot);
    if (preselectSnapshotsEqual(normalized, this.preselectStore.getSnapshot())) {
      return;
    }
    this.updatePreselection(normalized.ids, normalized.removedIds, false);
    if (this.wasmSessionCallBlockedForReentry()) {
      return;
    }
    try {
      this.session.setPreselectStateJsonSilent(JSON.stringify(normalized));
    } catch (err) {
      console.error("[DEBUG] setPreselectStateJsonSilent failed", err);
    }
    this.markDirty();
  }

  /** @emoji 🖱️ Controlled sync: mirrors hover chrome without emitting `hover`. */
  syncHoveredIdSilent(id: string | null): void {
    if (this.hoveredId === id) {
      return;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.updateHover(id);
      return;
    }
    this.session.setHoveredIdSilent(id);
    this.updateHover(id);
    this.markDirty();
  }

  getSelectionOptions(): ResolvedBoardSelectionOptions {
    return { ...this.selectionOptions, targets: { ...this.selectionOptions.targets } };
  }

  /** @emoji 🎯 Updates area-selection behavior for left-button drag gestures. */
  setSelectionOptions(options: BoardSelectionOptions): void {
    const next = resolveSelectionOptions({ ...this.selectionOptions, ...options });
    const tn = next.targets;
    const tc = this.selectionOptions.targets;
    if (next.method === this.selectionOptions.method && next.mode === this.selectionOptions.mode && tn.nodes === tc.nodes && tn.edges === tc.edges && tn.handles === tc.handles) {
      return;
    }
    this.selectionOptions = next;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.session.setSelectionOptions(next.method, boardSelectionModeForHost(next.mode), next.targets.nodes, next.targets.edges, next.targets.handles);
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.markDirty();
  }

  /** @emoji 🧩 Updates world-space Vello clip tiling mode (`none` | `world-clip`) without recreating the renderer shell. */
  setWorldRasterTilingOption(kind: WorldRasterTilingKind | undefined): void {
    const next = kind ?? "world-clip";
    if (this.worldRasterTiling === next) {
      return;
    }
    this.worldRasterTiling = next;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.session.setWorldRasterTiling(next);
    this.markDirty();
  }

  /** @emoji 📶 Updates LOD zoom thresholds on the WASM host and text overlay; mirrors `setLodZoomThresholdsJson` (`minimapMaxZoom` / `overviewMaxZoom` / `compactMaxZoom` / `normalMaxZoom` / `detailMaxZoom`). */
  setLodZoomThresholds(next: BoardLodZoomThresholds): void {
    const c: BoardLodZoomThresholds = {
      minimapMaxZoom: next.minimapMaxZoom,
      overviewMaxZoom: next.overviewMaxZoom,
      compactMaxZoom: next.compactMaxZoom,
      normalMaxZoom: next.normalMaxZoom,
      detailMaxZoom: next.detailMaxZoom,
    };
    if (
      c.minimapMaxZoom === this.lodZoomThresholds.minimapMaxZoom &&
      c.overviewMaxZoom === this.lodZoomThresholds.overviewMaxZoom &&
      c.compactMaxZoom === this.lodZoomThresholds.compactMaxZoom &&
      c.normalMaxZoom === this.lodZoomThresholds.normalMaxZoom &&
      c.detailMaxZoom === this.lodZoomThresholds.detailMaxZoom
    ) {
      return;
    }
    this.lodZoomThresholds = c;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.lastLodThresholdsJsonForWasm = null;
    this.markDirty();
  }

  /** @emoji 📐 Positive multiplier for LOD world grid steps on the WASM host (see {@link DEFAULT_BOARD_GRID_FACTOR}). */
  setGridFactor(next: number): void {
    const n = typeof next === "number" && Number.isFinite(next) && next > 0 && next <= 1e6 ? next : DEFAULT_BOARD_GRID_FACTOR;
    if (n === this.gridFactor) {
      return;
    }
    this.gridFactor = n;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.lastGridFactorForWasm = null;
    this.markDirty();
  }

  private lastLinkSessionJsonForWasm: string | null = null;

  /** @emoji 🔗 Mirrors a host {@link BoardLinkSessionSnapshot} into WASM for cross-surface link preview. */
  setLinkSession(snapshot: BoardLinkSessionSnapshot | null): void {
    const json = snapshot
      ? JSON.stringify({
          source: snapshot.source,
          endX: snapshot.endX,
          endY: snapshot.endY,
          compatiblePartIds: snapshot.compatiblePartIds,
          ringPartId: snapshot.ringPartId,
          ringAnchorIds: snapshot.ringAnchorIds,
        })
      : "";
    if (json === this.lastLinkSessionJsonForWasm) {
      return;
    }
    this.lastLinkSessionJsonForWasm = json;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    try {
      if (json.length === 0) {
        this.session.clearLinkSessionJson();
      } else {
        this.session.setLinkSessionJson(json);
      }
      this.applyWasmDrainToScene(this.session.drainEventsJson());
    } catch (err) {
      console.error("[DEBUG] setLinkSession failed", err);
    }
    this.markDirty();
  }

  /** @emoji 📶 When true (default), WASM draw LOD follows camera zoom; when false, optional {@link BoardRenderer.setForcedDrawLod} pins the tier. */
  setAutomaticLod(next: boolean): void {
    if (this.automaticLod === next) {
      return;
    }
    this.automaticLod = next;
    this.forcedDrawLodLabel = undefined;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.lastAutomaticLodForWasm = null;
    this.lastForcedDrawLodLabelForWasm = null;
    this.lastPushedDescriptorJson = null;
    this.markDirty();
  }

  /** @emoji 📶 Pins WASM draw LOD when {@link BoardRenderer.setAutomaticLod} is false; pass undefined to follow zoom bands. */
  setForcedDrawLod(next: BoardDrawLodKind | undefined): void {
    const norm = this.automaticLod || next === undefined ? undefined : isBoardDrawLodKind(next) ? next : undefined;
    if (this.forcedDrawLodLabel === norm) {
      return;
    }
    this.forcedDrawLodLabel = norm;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.lastForcedDrawLodLabelForWasm = null;
    this.markDirty();
  }

  private effectiveDrawLodLabel(): BoardDrawLodKind {
    if (!this.automaticLod && this.forcedDrawLodLabel !== undefined) {
      return this.forcedDrawLodLabel;
    }
    return resolveBoardLodLabelFromThresholds(this.camera.zoom, this.lodZoomThresholds);
  }

  /** @emoji 🧲 Enables snapping dragged nodes to the finest visible LOD grid on the WASM host. */
  setGridSnapEnabled(enabled: boolean): void {
    if (this.gridSnapEnabled === enabled) {
      return;
    }
    this.gridSnapEnabled = enabled;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.lastGridSnapEnabledForWasm = null;
    this.markDirty();
  }

  /** @emoji 🔗 Sets kind-compatibility rules for link gestures (empty = unrestricted). */
  setKindCompatibility(entries: readonly KindCompatEntry[] | undefined): void {
    const normalized = (entries ?? []).map((p) => {
      const rawSp = String(p.specificity ?? "handle")
        .trim()
        .toLowerCase();
      const specificity = rawSp === "general" || rawSp === "node" || rawSp === "edge" || rawSp === "handle" || rawSp === "wire" ? rawSp : "handle";
      return {
        source: String(p.source ?? "").trim(),
        target: String(p.target ?? "").trim(),
        bidirectional: p.bidirectional === true,
        important: p.important === true,
        specificity,
      };
    });
    const json = JSON.stringify(normalized);
    if (json === this.kindCompatJson) {
      return;
    }
    this.kindCompatJson = json;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.pushSceneToWasmDriver();
  }

  /** @emoji 🧩 Sets WASM semantic kind catalogs (handles, wires, nodes, edges). */
  setKindCatalogs(bundle: KindCatalogBundle | undefined): void {
    const merged: KindCatalogBundle = {
      edges: bundle?.edges ?? DEFAULT_KIND_CATALOG_BUNDLE.edges,
      handles: bundle?.handles ?? DEFAULT_KIND_CATALOG_BUNDLE.handles,
      nodes: bundle?.nodes ?? DEFAULT_KIND_CATALOG_BUNDLE.nodes,
      wires: bundle?.wires ?? DEFAULT_KIND_CATALOG_BUNDLE.wires,
    };
    this.kindCatalogsBundle = merged;
    const json = serializeKindCatalogBundle(merged);
    if (json === this.kindCatalogsJson) {
      return;
    }
    this.kindCatalogsJson = json;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.pushSceneToWasmDriver();
  }

  /** @emoji 📍 Applies declarative node x/y while keeping wasm-dragged coordinates when React props still show the pre-drag authoring values. */
  applyNodePositionFromProps(nodeId: string, x: number, y: number, instance: BoardSceneNode): void {
    const last = this.lastNodeAuthoringPositionById.get(nodeId);
    const propsUnchangedSinceLastSync = last !== undefined && last.x === x && last.y === y;
    const sceneMatchesDescriptor = instance.x === x && instance.y === y;
    if (propsUnchangedSinceLastSync && !sceneMatchesDescriptor) {
      return;
    }
    instance.setPosition(x, y);
    this.lastNodeAuthoringPositionById.set(nodeId, { x, y });
  }

  /** @emoji 🧹 Drops cached declarative coordinates for a removed node id (see {@link BoardRenderer.applyNodePositionFromProps}). */
  evictNodeAuthoringPosition(nodeId: string): void {
    this.lastNodeAuthoringPositionById.delete(nodeId);
  }

  getCameraSnapshot = (): CameraState => this.cameraStore.getSnapshot();

  subscribeCamera = (listener: () => void): (() => void) => this.cameraStore.subscribe(listener);

  getDrawLodSnapshot = (): BoardDrawLodKind => this.drawLodStore.getSnapshot();

  subscribeDrawLod = (listener: () => void): (() => void) => this.drawLodStore.subscribe(listener);

  on<TKey extends keyof BoardEventMap>(name: TKey, handler: (payload: BoardEventMap[TKey]) => void): () => void {
    return this.emitter.on(name, handler);
  }

  emit<TKey extends keyof BoardEventMap>(name: TKey, payload: BoardEventMap[TKey]): void {
    this.emitter.emit(name, payload);
  }

  private enqueueBoardGraphObservationFlush(): void {
    if (this.graphObservationFlushPending) {
      return;
    }
    this.graphObservationFlushPending = true;
    queueMicrotask(() => {
      this.graphObservationFlushPending = false;
      this.flushBoardGraphObservation();
    });
  }

  private flushBoardGraphObservation(): void {
    const prev = this.lastGraphObservation;
    if (prev === null) {
      this.lastGraphObservation = computeBoardGraphObservationSnapshot(this.scene);
      return;
    }
    const next = computeBoardGraphObservationSnapshot(this.scene);
    let anyEmitted = false;
    const prevRoot = new Set(prev.rootIds);
    const nextRoot = new Set(next.rootIds);
    const rootUnion = new Set([...prevRoot, ...nextRoot]);
    for (const id of rootUnion) {
      const inPrev = prevRoot.has(id);
      const inNext = nextRoot.has(id);
      const sigP = prev.nodeSigById.get(id);
      const sigN = next.nodeSigById.get(id);
      if (inPrev !== inNext || (inNext && sigP !== sigN)) {
        this.emitter.emit("parentNodeChange", { id });
        anyEmitted = true;
      }
    }
    const prevParentEdge = new Set(prev.parentEdgeIds);
    const nextParentEdge = new Set(next.parentEdgeIds);
    for (const id of new Set([...prevParentEdge, ...nextParentEdge])) {
      const sigP = prev.edgeSigById.get(id);
      const sigN = next.edgeSigById.get(id);
      if (prevParentEdge.has(id) !== nextParentEdge.has(id) || (nextParentEdge.has(id) && sigP !== sigN)) {
        this.emitter.emit("parentEdgeChange", { id });
        anyEmitted = true;
      }
    }
    if (!sortedStringArraysEqual(prev.childNodeIds, next.childNodeIds)) {
      this.emitter.emit("childNodesChange", { nodeIds: next.childNodeIds, rootIds: next.rootIds });
      anyEmitted = true;
    }
    if (!sortedStringArraysEqual(prev.childEdgeIds, next.childEdgeIds)) {
      this.emitter.emit("childEdgesChange", { edgeIds: next.childEdgeIds, rootIds: next.rootIds });
      anyEmitted = true;
    }
    const nextChildNodeSet = new Set(next.childNodeIds);
    for (const id of nextChildNodeSet) {
      if (prev.nodeSigById.get(id) !== next.nodeSigById.get(id)) {
        this.emitter.emit("childNodeChange", { id });
        anyEmitted = true;
      }
    }
    const nextChildEdgeSet = new Set(next.childEdgeIds);
    for (const id of nextChildEdgeSet) {
      if (prev.edgeSigById.get(id) !== next.edgeSigById.get(id)) {
        this.emitter.emit("childEdgeChange", { id });
        anyEmitted = true;
      }
    }
    const allNodeIds = new Set([...prev.nodeSigById.keys(), ...next.nodeSigById.keys()]);
    for (const id of allNodeIds) {
      const sigP = prev.nodeSigById.get(id);
      const sigN = next.nodeSigById.get(id);
      if (sigP === sigN) {
        continue;
      }
      if (sigP === undefined && sigN !== undefined) {
        this.emitter.emit("nodeCreate", { id });
        anyEmitted = true;
        continue;
      }
      if (sigP !== undefined && sigN !== undefined) {
        this.emitter.emit("nodeChange", { id });
        anyEmitted = true;
      }
    }
    const allEdgeIds = new Set([...prev.edgeSigById.keys(), ...next.edgeSigById.keys()]);
    for (const id of allEdgeIds) {
      const sigP = prev.edgeSigById.get(id);
      const sigN = next.edgeSigById.get(id);
      if (sigP === sigN || sigP === undefined || sigN === undefined) {
        continue;
      }
      this.emitter.emit("edgeChange", { id });
      anyEmitted = true;
    }
    const allWireIds = new Set([...prev.wireSigById.keys(), ...next.wireSigById.keys()]);
    for (const id of allWireIds) {
      const sigP = prev.wireSigById.get(id);
      const sigN = next.wireSigById.get(id);
      if (sigP === sigN || sigP === undefined || sigN === undefined) {
        continue;
      }
      this.emitter.emit("wireChange", { id });
      anyEmitted = true;
    }
    if (anyEmitted) {
      this.emitter.emit("change", undefined);
    }
    this.lastGraphObservation = next;
  }

  batch(action: () => void): void {
    this.batchDepth += 1;
    try {
      action();
    } finally {
      this.batchDepth -= 1;
      if (this.batchDepth === 0) {
        this.enqueueBoardGraphObservationFlush();
      }
      if (this.batchDepth === 0 && this.invalidated) {
        this.invalidate();
      }
    }
  }

  defineStyle(name: string, style: BoardStyle): void {
    this.styles.set(name, style);
    this.markDirty();
  }

  getStyle(name: string | null, fallbackName: string): BoardStyle {
    return this.styles.get(name ?? fallbackName) ?? this.styles.get(fallbackName) ?? {};
  }

  setSize(width: number, height: number, dpr = this.dpr): void {
    this.width = Math.max(1, Math.round(width));
    this.height = Math.max(1, Math.round(height));
    this.dpr = Math.max(1, dpr);
    if (this.canvas) {
      const nextW = Math.round(this.width * this.dpr);
      const nextH = Math.round(this.height * this.dpr);
      if (this.canvas.width !== nextW || this.canvas.height !== nextH) {
        this.canvas.width = nextW;
        this.canvas.height = nextH;
      }
    }
    this.markDirty();
  }

  setCamera(x: number, y: number, zoom: number): void {
    const z = clamp(zoom, MIN_ZOOM, MAX_ZOOM);
    const next: CameraState = { x, y, zoom: z };
    if (pointsEqual(this.camera, next) && nearlyEqual(this.camera.zoom, next.zoom)) {
      return;
    }
    this.camera.x = next.x;
    this.camera.y = next.y;
    this.camera.zoom = next.zoom;
    this.cameraStore.setSnapshot({ ...this.camera }, (left, right) => pointsEqual(left, right) && nearlyEqual(left.zoom, right.zoom));
    this.emitter.emit("camera", { ...this.camera });
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.session.setCamera(x, y, z);
    this.applyWasmDrainToScene(this.session.drainEventsJson());
  }

  subscribeFrame(listener: FrameListener): () => void {
    this.frameListeners.add(listener);
    return () => {
      this.frameListeners.delete(listener);
    };
  }

  worldToScreen(point: Point): Point {
    return {
      x: (point.x - this.camera.x) * this.camera.zoom + this.width / 2,
      y: (point.y - this.camera.y) * this.camera.zoom + this.height / 2,
    };
  }

  screenToWorld(point: Point): Point {
    return {
      x: (point.x - this.width / 2) / this.camera.zoom + this.camera.x,
      y: (point.y - this.height / 2) / this.camera.zoom + this.camera.y,
    };
  }

  markDirty(): void {
    this.invalidated = true;
    if (this.batchDepth > 0) {
      return;
    }
    this.enqueueBoardGraphObservationFlush();
    this.invalidate();
  }

  invalidate(): void {
    if (this.isDisposed) {
      return;
    }
    this.invalidated = true;
    if (this.renderPipelineDepth > 0) {
      return;
    }
    this.emit("invalidate", undefined);
    if (this.renderMode === "headless-test") {
      return;
    }
    if (this.rafId !== null) {
      return;
    }
    const requestFrame = globalThis.requestAnimationFrame?.bind(globalThis);
    if (!requestFrame) {
      this.render(Date.now());
      return;
    }
    this.rafId = requestFrame((timestamp) => {
      this.rafId = null;
      this.render(timestamp);
    });
  }

  /** @emoji 🧹 Drops WASM parsed-SVG icon scenes so the next draw rebuilds from current {@link Node.iconKind} bytes (catalog hot-swap or identical key with new SVG). */
  clearIconVectorCache(): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    try {
      this.session.clearIconVectorCache();
    } catch (err) {
      console.error("[DEBUG] clearIconVectorCache failed", err);
    }
    this.invalidate();
  }

  render(timestamp = globalThis.performance?.now?.() ?? Date.now()): void {
    if (this.renderPipelineDepth > 0) {
      this.invalidated = true;
      return;
    }
    this.renderPipelineDepth += 1;
    const frameDelta = this.lastRenderTimestamp === null ? 0 : timestamp - this.lastRenderTimestamp;
    this.lastRenderTimestamp = timestamp;
    this.invalidated = false;
    try {
      if (this.renderMode === "headless-test") {
        if (!this.suppressSceneToWasmPush) {
          this.pushSceneToWasmDriver();
        }
      } else if (this.canvas && !this.gpuSurfaceUnavailable) {
        let gpuReady = this.readGpuReady();
        if (!gpuReady && !this.gpuSurfaceInitPromise) {
          this.gpuSurfaceInitPromise = this.initGpuSurfaceOnce()
            .then(() => {
              this.gpuSurfaceInitPromise = null;
              this.markDirty();
            })
            .catch((err: unknown) => {
              console.error("[DEBUG] BoardRenderer GPU surface init failed", err);
              this.gpuSurfaceErrorDetail = summarizeRasterSurfaceFailure(err);
              this.gpuSurfaceUnavailable = true;
              this.cachedWasmGpuReady = false;
              this.gpuSurfaceInitPromise = null;
              this.markDirty();
            });
        }
        gpuReady = this.readGpuReady();
        if (gpuReady) {
          this.syncGpuFrame();
        }
      }
      this.paintTextOverlays();
      const frameState: FrameState = {
        camera: { ...this.camera },
        renderer: this,
        selection: this.selectionStore.getSnapshot(),
      };
      for (const listener of this.frameListeners) {
        listener(frameState, frameDelta);
      }
      this.applyCanvasDebugAttributes();
    } finally {
      this.renderPipelineDepth -= 1;
      if (this.renderPipelineDepth === 0 && this.invalidated && !this.isDisposed) {
        this.invalidate();
      }
    }
  }

  private async initGpuSurfaceOnce(): Promise<void> {
    if (!this.canvas || this.renderMode === "headless-test") {
      return;
    }
    await ensureElementsBoardWasmLoaded();
    const lw = this.width;
    const lh = this.height;
    const dpr = this.dpr;
    const cw = Math.max(1, Math.round(lw * dpr));
    const ch = Math.max(1, Math.round(lh * dpr));
    if (this.canvas.width !== cw || this.canvas.height !== ch) {
      this.canvas.width = cw;
      this.canvas.height = ch;
    }
    this.wasmSessionBorrowDepth += 1;
    this.cachedWasmGpuReady = false;
    try {
      await this.session.attach_canvas(this.canvas, lw, lh, dpr);
    } finally {
      this.wasmSessionBorrowDepth = Math.max(0, this.wasmSessionBorrowDepth - 1);
      if (this.wasmSessionBorrowDepth === 0 && !this.isDisposed) {
        this.invalidate();
      }
    }
    const o = this.selectionOptions;
    this.session.setSelectionOptions(o.method, boardSelectionModeForHost(o.mode), o.targets.nodes, o.targets.edges, o.targets.handles);
    this.session.setWorldRasterTiling(this.worldRasterTiling);
    this.pushLodAndGridSnapToWasmSession();
    this.session.setHandleLinkCompatJson(this.kindCompatJson);
    try {
      this.session.setKindCatalogsJson(this.kindCatalogsJson);
    } catch (err) {
      console.error("[DEBUG] setKindCatalogsJson failed after attach_canvas", err);
    }
    this.lastPushedKindCatalogsJson = this.kindCatalogsJson;
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.syncGpuReadyCacheFromSession();
  }

  private descriptorJsonForWasmHost(): string {
    const committedSelection = this.selectionIds;
    const nodes: Record<string, unknown>[] = [];
    for (const node of this.scene.nodes.values()) {
      const base: Record<string, unknown> = {
        id: node.id,
        x: node.x,
        y: node.y,
        draggable: node.draggable,
        selected: committedSelection.has(node.id),
        style: node.style,
        text: node.text,
        visible: node.visible,
      };
      if (node.iconKind) {
        base.iconKind = node.iconKind;
      }
      if (node.root) {
        base.root = true;
      }
      if (node.nodeKind.trim() !== "") {
        base.nodeKind = node.nodeKind;
      }
      if (Object.keys(node.userData).length > 0) {
        base.userData = node.userData;
      }
      if (node.textAutofit) {
        base.textAutofit = true;
      }
      if (node.textFontFamily !== BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT) {
        base.textFontFamily = node.textFontFamily;
      }
      if (node.textFontSize !== BOARD_NODE_TEXT_FONT_PX_DEFAULT) {
        base.textFontSize = node.textFontSize;
      }
      if (node.textAlignment !== BOARD_NODE_TEXT_ALIGNMENT_DEFAULT) {
        base.textAlignment = node.textAlignment;
      }
      if (node.shape === "rectangle") {
        base.shape = "rectangle";
        base.width = node.width;
        base.height = node.height;
      } else {
        base.shape = "circle";
        base.radius = node.radius;
      }
      nodes.push(base);
    }
    const handles: Record<string, unknown>[] = [];
    for (const handle of this.scene.handles.values()) {
      const row: Record<string, unknown> = {
        id: handle.id,
        nodeId: handle.node.id,
        angle: handle.angle,
        radius: handle.radius,
        selected: committedSelection.has(handle.id),
        style: handle.style,
        visible: handle.visible,
        handleKind: handle.handleKind,
      };
      if (handle.color) {
        row.color = handle.color;
      }
      if (handle.iconKind) {
        row.iconKind = handle.iconKind;
      }
      handles.push(row);
    }
    const edges: Record<string, unknown>[] = [];
    for (const edge of this.scene.edges.values()) {
      const er: Record<string, unknown> = {
        id: edge.id,
        source: edge.source.id,
        target: edge.target.id,
        selected: committedSelection.has(edge.id),
        style: edge.style,
        visible: edge.visible,
      };
      if (edge.edgeKind.trim() !== "") {
        er.edgeKind = edge.edgeKind;
      }
      edges.push(er);
    }
    const wires: Record<string, unknown>[] = [];
    for (const wire of this.scene.wires.values()) {
      const row: Record<string, unknown> = {
        id: wire.id,
        source: wire.source.id,
        selected: committedSelection.has(wire.id),
        style: wire.style,
        visible: wire.visible,
      };
      if (wire.target) {
        row.target = wire.target.id;
      }
      if (wire.endX != null && wire.endY != null) {
        row.endX = wire.endX;
        row.endY = wire.endY;
      }
      if (wire.wireKind.trim() !== "") {
        row.wireKind = wire.wireKind;
      }
      wires.push(row);
    }
    return JSON.stringify({ nodes, handles, edges, wires });
  }

  private syncBoardAppearanceFromDocument(): void {
    if (this.renderMode === "headless-test") {
      return;
    }
    if (typeof document === "undefined") {
      return;
    }
    try {
      const json = serializeElementsBoardVelloThemeJson();
      if (json !== this.lastVelloThemeJson) {
        this.lastVelloThemeJson = json;
        this.session.setVelloThemeJson(json);
      }
    } catch {
      this.lastVelloThemeJson = "";
    }
    const styles = boardDefaultStylesFromElementsUiTokens();
    for (const [key, value] of Object.entries(styles)) {
      this.styles.set(key, value);
    }
  }

  private bumpWasmHostSceneMergeResyncEpoch(): void {
    const prev = this.wasmHostSceneMergeResyncStore.getSnapshot();
    this.wasmHostSceneMergeResyncStore.setSnapshot(prev + 1, (a, b) => a === b);
  }

  private pushLodAndGridSnapToWasmSession(): void {
    const lodJson = JSON.stringify({
      minimapMaxZoom: this.lodZoomThresholds.minimapMaxZoom,
      overviewMaxZoom: this.lodZoomThresholds.overviewMaxZoom,
      compactMaxZoom: this.lodZoomThresholds.compactMaxZoom,
      normalMaxZoom: this.lodZoomThresholds.normalMaxZoom,
      detailMaxZoom: this.lodZoomThresholds.detailMaxZoom,
    });
    if (lodJson !== this.lastLodThresholdsJsonForWasm) {
      try {
        this.session.setLodZoomThresholdsJson(lodJson);
        this.lastLodThresholdsJsonForWasm = lodJson;
      } catch (err) {
        console.error("[DEBUG] setLodZoomThresholdsJson failed", err);
      }
    }
    if (this.lastGridSnapEnabledForWasm !== this.gridSnapEnabled) {
      this.session.setGridSnapEnabled(this.gridSnapEnabled);
      this.lastGridSnapEnabledForWasm = this.gridSnapEnabled;
    }
    if (this.lastGridFactorForWasm !== this.gridFactor) {
      try {
        this.session.setGridFactor(this.gridFactor);
        this.lastGridFactorForWasm = this.gridFactor;
      } catch (err) {
        console.error("[DEBUG] setGridFactor failed", err);
      }
    }
    if (this.lastAutomaticLodForWasm !== this.automaticLod) {
      this.session.setAutomaticLod(this.automaticLod);
      this.lastAutomaticLodForWasm = this.automaticLod;
    }
    const forcedWasm = this.forcedDrawLodLabel ?? "";
    if (this.lastForcedDrawLodLabelForWasm !== forcedWasm) {
      this.session.setForcedDrawLodLabel(forcedWasm);
      this.lastForcedDrawLodLabelForWasm = forcedWasm;
    }
  }

  /** @emoji 🛡️ Defers WASM scene push when `attach_canvas` or `renderFrame` still holds a session borrow; sets {@link BoardRenderer.invalidated} so the next frame retries. */
  private pushSceneToWasmDriver(): void {
    if (this.suppressSceneToWasmPush) {
      return;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    const o = this.selectionOptions;
    this.session.setSize(this.width, this.height, this.dpr);
    this.session.setSelectionOptions(o.method, boardSelectionModeForHost(o.mode), o.targets.nodes, o.targets.edges, o.targets.handles);
    this.session.setWorldRasterTiling(this.worldRasterTiling);
    this.pushLodAndGridSnapToWasmSession();
    this.session.setHandleLinkCompatJson(this.kindCompatJson);
    if (this.kindCatalogsJson !== this.lastPushedKindCatalogsJson) {
      try {
        this.session.setKindCatalogsJson(this.kindCatalogsJson);
      } catch (err) {
        console.error("[DEBUG] setKindCatalogsJson failed", err);
      }
      this.lastPushedKindCatalogsJson = this.kindCatalogsJson;
    }
    const deferDescriptorSync = this.session.isDraggingAreaSelect() || this.session.defersDescriptorSyncFromJs() || this.preselectIds.size > 0;
    if (this.lastDescriptorPushDeferred && !deferDescriptorSync) {
      this.lastPushedDescriptorJson = null;
    }
    this.lastDescriptorPushDeferred = deferDescriptorSync;
    if (!deferDescriptorSync) {
      const desc = this.descriptorJsonForWasmHost();
      if (desc !== this.lastPushedDescriptorJson) {
        try {
          this.session.syncDescriptorJson(desc);
          this.lastPushedDescriptorJson = desc;
        } catch (err) {
          console.error("[DEBUG] syncDescriptorJson failed", err);
        }
      }
    }
    this.session.setCamera(this.camera.x, this.camera.y, this.camera.zoom);
    this.syncBoardAppearanceFromDocument();
    this.applyWasmDrainToScene(this.session.drainEventsJson());
  }

  private applyWasmDrainToScene(raw: string): void {
    let rows: Array<{ name: string; payload: Record<string, unknown> }>;
    try {
      rows = JSON.parse(raw) as Array<{ name: string; payload: Record<string, unknown> }>;
    } catch {
      return;
    }
    if (rows.length === 0) {
      return;
    }
    let graphMutatedForHostMerge = false;
    this.suppressSceneToWasmPush = true;
    try {
      for (const row of rows) {
        switch (row.name) {
          case "camera": {
            const p = row.payload as { x: number; y: number; zoom: number };
            const nextZoom = clamp(p.zoom, MIN_ZOOM, MAX_ZOOM);
            if (pointsEqual(this.camera, { x: p.x, y: p.y }) && nearlyEqual(this.camera.zoom, nextZoom)) {
              break;
            }
            this.camera.x = p.x;
            this.camera.y = p.y;
            this.camera.zoom = nextZoom;
            this.cameraStore.setSnapshot({ ...this.camera }, (left, right) => pointsEqual(left, right) && nearlyEqual(left.zoom, right.zoom));
            this.emitter.emit("camera", { ...this.camera });
            break;
          }
          case "select": {
            const ids = (row.payload as { ids: string[] }).ids ?? [];
            this.updateSelection(ids, true);
            break;
          }
          case "preselect": {
            const payload = row.payload as { ids?: string[]; removedIds?: string[] };
            this.updatePreselection(payload.ids ?? [], payload.removedIds ?? [], true);
            break;
          }
          case "preselectCancel": {
            this.updatePreselection([], [], false);
            this.applySelectionChromeToSceneObjects();
            this.emit("preselectCancel", BOARD_PRESELECT_EMPTY);
            break;
          }
          case "hover": {
            const hid = row.payload.id;
            const next = hid === null || hid === undefined ? null : String(hid);
            this.updateHover(next);
            this.publishHover();
            break;
          }
          case "nodeMove": {
            const id = String(row.payload.id);
            const x = Number(row.payload.x);
            const y = Number(row.payload.y);
            const node = this.scene.nodes.get(id);
            if (node) {
              if (nearlyEqual(node.x, x) && nearlyEqual(node.y, y)) {
                break;
              }
              node.setPosition(x, y);
            }
            this.emitter.emit("nodeMove", { id, x, y });
            break;
          }
          case "edgeDelete": {
            const id = String((row.payload as { id: string }).id);
            const edge = this.scene.edges.get(id);
            if (edge) {
              this.clearWasmHostAuthorshipForEdge(id);
              this.scene.remove(edge);
              graphMutatedForHostMerge = true;
            } else {
              this.clearWasmHostAuthorshipForEdge(id);
              graphMutatedForHostMerge = true;
              this.emitSceneDeleteEvent("edgeDelete", { id });
            }
            break;
          }
          case "nodeDelete": {
            const id = String((row.payload as { id: string }).id);
            const node = this.scene.nodes.get(id);
            if (node) {
              this.scene.remove(node);
              graphMutatedForHostMerge = true;
            } else {
              this.emitSceneDeleteEvent("nodeDelete", { id });
            }
            break;
          }
          case "edgeCreate": {
            const id = String(row.payload.id ?? "");
            const sourceId = String(row.payload.source ?? "");
            const targetId = String(row.payload.target ?? "");
            if (!id || !sourceId || !targetId || this.scene.edges.has(id)) {
              break;
            }
            const sourceObj = this.scene.getObjectById(sourceId);
            const targetObj = this.scene.getObjectById(targetId);
            if (!(sourceObj instanceof BoardSceneHandle) || !(targetObj instanceof BoardSceneHandle)) {
              break;
            }
            this.wasmHostAuthoredEdgeIds.add(id);
            this.wasmHostAuthoredLinkByEdgeId.set(id, { source: sourceId, target: targetId });
            this.scene.ingestWasmEdge(new BoardSceneEdge({ id, source: sourceObj, target: targetObj }));
            graphMutatedForHostMerge = true;
            this.emitter.emit("edgeCreate", { id, source: sourceId, target: targetId });
            break;
          }
          case "proximityConnect": {
            const id = String((row.payload as { id?: string }).id ?? "");
            const sourceId = String((row.payload as { source?: string }).source ?? "");
            const targetId = String((row.payload as { target?: string }).target ?? "");
            if (!id || !sourceId || !targetId) {
              break;
            }
            this.emitter.emit("proximityConnect", { id, source: sourceId, target: targetId });
            break;
          }
          case "indirectConnect": {
            const id = String((row.payload as { id?: string }).id ?? "");
            const sourceId = String((row.payload as { source?: string }).source ?? "");
            const targetId = String((row.payload as { target?: string }).target ?? "");
            if (!id || !sourceId || !targetId) {
              break;
            }
            this.emitter.emit("indirectConnect", { id, source: sourceId, target: targetId });
            break;
          }
          case "linkCompatibleNodes": {
            const p = row.payload as { source?: string; nodeIds?: string[] };
            this.emitter.emit("linkCompatibleNodes", {
              source: String(p.source ?? ""),
              nodeIds: Array.isArray(p.nodeIds) ? p.nodeIds.map(String) : [],
            });
            break;
          }
          case "linkTargetRing": {
            const p = row.payload as { source?: string; nodeId?: string | null; handleIds?: string[] };
            const nodeId = p.nodeId === null || p.nodeId === undefined ? null : String(p.nodeId);
            this.emitter.emit("linkTargetRing", {
              source: String(p.source ?? ""),
              nodeId,
              handleIds: Array.isArray(p.handleIds) ? p.handleIds.map(String) : [],
            });
            break;
          }
          default:
            break;
        }
      }
    } finally {
      this.suppressSceneToWasmPush = false;
      this.enqueueBoardGraphObservationFlush();
      if (graphMutatedForHostMerge) {
        this.bumpWasmHostSceneMergeResyncEpoch();
      }
    }
  }

  /** @emoji 🏷️ Draws node captions on {@link BoardRenderer.attachTextOverlayCanvas} (GPU path has no text primitives). */
  private paintTextOverlays(): void {
    if (this.renderMode === "headless-test" || !this.textOverlayCanvas) {
      return;
    }
    const el = this.textOverlayCanvas;
    const nextW = Math.max(1, Math.round(this.width * this.dpr));
    const nextH = Math.max(1, Math.round(this.height * this.dpr));
    if (el.width !== nextW || el.height !== nextH) {
      el.width = nextW;
      el.height = nextH;
    }
    const ctx = el.getContext("2d");
    if (!ctx) {
      return;
    }
    const inset = 0.88;
    ctx.setTransform(this.dpr, 0, 0, this.dpr, 0, 0);
    ctx.clearRect(0, 0, this.width, this.height);
    const lod = this.effectiveDrawLodLabel();
    const chrome = boardElementInteractionChrome(this.selectionIds, this.preselectStore.getSnapshot());
    for (const node of this.scene.nodes.values()) {
      if (!node.visible) {
        continue;
      }
      const caption = boardTextOverlayCaptionForLod(node.text ?? "", lod, node.iconKind);
      if (caption === null) {
        continue;
      }
      let maxW: number;
      let maxH: number;
      if (node.shape === "rectangle") {
        maxW = node.width * this.camera.zoom * inset;
        maxH = node.height * this.camera.zoom * inset;
      } else {
        const d = 2 * node.radius * this.camera.zoom * inset;
        maxW = d;
        maxH = d;
      }
      if (maxW < 4 || maxH < 4) {
        continue;
      }
      const boxCenter = this.worldToScreen({ x: node.x, y: node.y });
      const style = this.getStyle(node.style, boardInteractionChromeStyleKey("node", node.id, chrome));
      const family = node.textFontFamily;
      ctx.fillStyle = style.stroke ?? BOARD_STYLES_HEADLESS_FALLBACK.node.stroke ?? "#001117";
      if (node.textAutofit) {
        const fontPx = boardFitTextFontPx(ctx, caption, maxW, maxH, 4, 512, family);
        ctx.font = boardBuildCanvasFontSpec(fontPx, family);
        let line = caption;
        if (ctx.measureText(line).width > maxW) {
          line = boardEllipsisTextToWidth(ctx, caption, maxW);
        }
        ctx.textAlign = "center";
        ctx.textBaseline = "middle";
        ctx.fillText(line, boxCenter.x, boxCenter.y);
        continue;
      }
      const fontPx = node.textFontSize;
      ctx.font = boardBuildCanvasFontSpec(fontPx, family);
      const line = boardEllipsisTextToWidth(ctx, caption, maxW);
      const anchor = boardNodeTextPlacementAnchor(boxCenter.x, boxCenter.y, maxW, maxH, node.textAlignment);
      ctx.textAlign = anchor.textAlign;
      ctx.textBaseline = anchor.textBaseline;
      ctx.fillText(line, anchor.fillX, anchor.fillY);
    }
    const drawHandleLabels = lod === "detail" || lod === "micro";
    if (drawHandleLabels) {
      const handleFontPx = lod === "detail" ? 10 : 11;
      ctx.font = boardBuildCanvasFontSpec(handleFontPx, BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT);
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      for (const handle of this.scene.handles.values()) {
        if (!handle.visible) {
          continue;
        }
        const node = handle.node;
        if (!node.visible) {
          continue;
        }
        const rawLabel = boardHandleKindOverlayLabel(handle.handleKind, this.kindCatalogsBundle);
        const caption = boardHandleOverlayCaptionForLod(rawLabel, lod);
        if (caption === null) {
          continue;
        }
        const handleWorld = computeHandlePosition(node, handle.angle);
        const handleScreen = this.worldToScreen(handleWorld);
        const nodeScreen = this.worldToScreen({ x: node.x, y: node.y });
        const dx = handleScreen.x - nodeScreen.x;
        const dy = handleScreen.y - nodeScreen.y;
        const len = Math.hypot(dx, dy);
        const outward = len > 1e-6 ? 10 / len : 0;
        const labelX = handleScreen.x + dx * outward;
        const labelY = handleScreen.y + dy * outward;
        const style = this.getStyle(handle.style, boardInteractionChromeStyleKey("handle", handle.id, chrome));
        ctx.fillStyle = style.stroke ?? BOARD_STYLES_HEADLESS_FALLBACK.handle.stroke ?? "#001117";
        ctx.fillText(caption, labelX, labelY);
      }
    }
  }

  /** @emoji 🎨 Presents one GPU frame after {@link BoardRenderer.pushSceneToWasmDriver} (same order as pre-369 main-thread canvas: no WASM scene push until the swapchain exists). */
  private syncGpuFrame(): void {
    if (this.renderMode === "headless-test" || !this.readGpuReady()) {
      return;
    }
    if (this.wasmGpuFrameDepth > 0) {
      return;
    }
    if (!this.suppressSceneToWasmPush) {
      this.pushSceneToWasmDriver();
    }
    this.wasmGpuFrameDepth += 1;
    try {
      this.session.renderFrame();
      this.gpuSurfacePresentedFrame = true;
      this.gpuSurfaceErrorDetail = "";
    } catch (err: unknown) {
      console.error("[DEBUG] BoardRenderer GPU surface frame failed", err);
      this.gpuSurfaceErrorDetail = summarizeRasterSurfaceFailure(err);
      this.gpuSurfaceUnavailable = true;
      this.gpuSurfacePresentedFrame = false;
      this.cachedWasmGpuReady = false;
    } finally {
      this.wasmGpuFrameDepth -= 1;
      if (this.wasmGpuFrameDepth === 0) {
        this.syncGpuReadyCacheFromSession();
      }
    }
  }

  /** @emoji 🛡️ Reads GPU-surface attach state without calling WASM while `renderFrame` may still borrow the session across `device.poll` re-entry. */
  private readGpuReady(): boolean {
    if (this.wasmGpuFrameDepth > 0 || this.wasmSessionBorrowDepth > 0) {
      return this.cachedWasmGpuReady;
    }
    this.syncGpuReadyCacheFromSession();
    return this.cachedWasmGpuReady;
  }

  /** @emoji 📡 Refreshes {@link BoardRenderer.cachedWasmGpuReady} from WASM when no in-flight GPU frame holds the session borrow. */
  private syncGpuReadyCacheFromSession(): void {
    if (this.isDisposed) {
      this.cachedWasmGpuReady = false;
      return;
    }
    if (this.wasmSessionBorrowDepth > 0 || this.wasmGpuFrameDepth > 0) {
      return;
    }
    try {
      this.cachedWasmGpuReady = this.session.gpuReady();
    } catch {
      this.cachedWasmGpuReady = false;
    }
  }

  /** @emoji 🔗 Clears WASM‑host authorship for an edge id (gesture link tracking + endpoint map); call when the link is deleted, adopted into JSX, or purged from the merged descriptor. */
  clearWasmHostAuthorshipForEdge(edgeId: string): void {
    this.wasmHostAuthoredEdgeIds.delete(edgeId);
    this.wasmHostAuthoredLinkByEdgeId.delete(edgeId);
  }

  dispose(): void {
    this.isDisposed = true;
    this.detachCanvasListeners();
    this.textOverlayCanvas = null;
    this.wasmHostAuthoredEdgeIds.clear();
    this.wasmHostAuthoredLinkByEdgeId.clear();
    this.session.free();
    this.gpuSurfacePresentedFrame = false;
    this.gpuSurfaceErrorDetail = "";
    this.lastGraphObservation = null;
    if (this.rafId !== null && globalThis.cancelAnimationFrame) {
      globalThis.cancelAnimationFrame(this.rafId);
    }
    this.runWithoutSceneDeleteEvents(() => {
      this.scene.clear();
    });
    if (BoardRenderer.activeRenderer === this) {
      BoardRenderer.activeRenderer = null;
    }
    if (this.canvas) {
      const el = this.canvas as BoardCanvasElement;
      if (el.__boardRenderer === this) {
        delete el.__boardRenderer;
      }
    }
  }

  private attachCanvasListeners(): void {
    if (!this.canvas) {
      return;
    }
    this.canvas.tabIndex = 0;
    this.canvas.style.touchAction = "none";
    const bindings = new BoardEventBindingController();
    bindings.listen(this.canvas, "contextmenu", this.handleContextMenu as EventListener);
    bindings.listen(this.canvas, "pointerdown", this.handlePointerDown as EventListener);
    bindings.listen(this.canvas, "pointermove", this.handlePointerMove as EventListener);
    bindings.listen(this.canvas, "pointerup", this.handlePointerUp as EventListener);
    bindings.listen(this.canvas, "pointerleave", this.handlePointerLeave as EventListener);
    bindings.listen(this.canvas, "wheel", this.handleWheel as EventListener, { passive: false });
    bindings.listen(globalThis, "keydown", this.handleWindowKeyDown as EventListener, true);
    (this as BoardRenderer & { __eventBindings?: BoardEventBindingController }).__eventBindings = bindings;
  }

  private detachCanvasListeners(): void {
    (this as BoardRenderer & { __eventBindings?: BoardEventBindingController }).__eventBindings?.dispose();
    (this as BoardRenderer & { __eventBindings?: BoardEventBindingController }).__eventBindings = undefined;
  }

  /** @emoji 🎨 Reapplies scene `selected` / `highlighted` from committed selection and preselection only. */
  syncInteractionChrome(): void {
    this.applySelectionChromeToSceneObjects();
  }

  private applySelectionChromeToSceneObjects(): void {
    const { highlightedIds, selectedIds } = boardElementInteractionChrome(this.selectionIds, this.preselectStore.getSnapshot());
    for (const object of this.scene.getAllObjects()) {
      object.selected = selectedIds.has(object.id);
      object.highlighted = highlightedIds.has(object.id);
    }
  }

  private updateSelection(ids: Iterable<string>, emit: boolean): void {
    const nextIds = new Set(ids);
    const nextSnapshot = createSelectionSnapshot(nextIds);
    if (arrayEqual(nextSnapshot.ids, this.selectionStore.getSnapshot().ids)) {
      return;
    }
    this.selectionIds = nextIds;
    if (emit) {
      this.updatePreselection([], [], false);
    }
    this.applySelectionChromeToSceneObjects();
    if (emit) {
      this.emit("select", nextSnapshot);
    }
    this.selectionStore.setSnapshot(nextSnapshot, (left, right) => arrayEqual(left.ids, right.ids));
    this.markDirty();
  }

  private updatePreselection(ids: Iterable<string>, removedIds: Iterable<string>, emit: boolean): void {
    const nextSnapshot = createPreselectSnapshot(ids, removedIds);
    if (preselectSnapshotsEqual(nextSnapshot, this.preselectStore.getSnapshot())) {
      return;
    }
    this.preselectIds = new Set(nextSnapshot.ids);
    this.preselectRemovedIds = new Set(nextSnapshot.removedIds);
    this.preselectStore.setSnapshot(nextSnapshot, preselectSnapshotsEqual);
    this.applySelectionChromeToSceneObjects();
    if (emit) {
      this.emit("preselect", nextSnapshot);
    }
    this.markDirty();
  }

  private updateHover(id: string | null): void {
    if (this.hoveredId === id) {
      return;
    }
    this.hoveredId = id;
  }

  /** @emoji 📡 Emits {@link BoardEventMap.hover} using the last recorded pointer and current {@link BoardRenderer.hoveredId}. */
  private publishHover(): void {
    const world = this.screenToWorld({ x: this.lastPointerScreenX, y: this.lastPointerScreenY });
    this.emit("hover", {
      clientX: this.lastPointerClientX,
      clientY: this.lastPointerClientY,
      id: this.hoveredId,
      screenX: this.lastPointerScreenX,
      screenY: this.lastPointerScreenY,
      worldX: world.x,
      worldY: world.y,
    });
  }

  private recordPointerClient(clientX: number, clientY: number, screenX: number, screenY: number): void {
    this.lastPointerClientX = clientX;
    this.lastPointerClientY = clientY;
    this.lastPointerScreenX = screenX;
    this.lastPointerScreenY = screenY;
  }

  private deleteSelectedObjects(): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.session.deleteSelection();
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.invalidate();
  }

  private readonly handleWindowKeyDown = (event: KeyboardEvent): void => {
    if (event.repeat) {
      return;
    }
    if (BoardRenderer.activeRenderer !== this) {
      return;
    }
    if (event.key === "Escape") {
      if (this.wasmSessionCallBlockedForReentry()) {
        this.invalidated = true;
        return;
      }
      if (this.session.cancelAreaSelect()) {
        event.preventDefault();
        this.applyWasmDrainToScene(this.session.drainEventsJson());
        this.invalidate();
      }
      return;
    }
    if (!shouldBoardHandleDeleteShortcut()) {
      return;
    }
    if (event.key !== "Delete" && event.key !== "Backspace") {
      return;
    }
    if (this.selectionIds.size === 0) {
      return;
    }
    event.preventDefault();
    this.deleteSelectedObjects();
  };

  private applyCanvasDebugAttributes(): void {
    if (!this.canvas) {
      return;
    }
    if (this.renderMode === "headless-test") {
      this.canvas.dataset.boardSurfaceState = "off";
      delete this.canvas.dataset.boardSurfaceFailure;
    } else if (this.gpuSurfaceUnavailable) {
      this.canvas.dataset.boardSurfaceState = "error";
      if (this.gpuSurfaceErrorDetail) {
        this.canvas.dataset.boardSurfaceFailure = this.gpuSurfaceErrorDetail.slice(0, 512);
      }
    } else if (this.gpuSurfacePresentedFrame && this.readGpuReady()) {
      this.canvas.dataset.boardSurfaceState = "ready";
      delete this.canvas.dataset.boardSurfaceFailure;
    } else if (this.gpuSurfaceInitPromise) {
      this.canvas.dataset.boardSurfaceState = "init";
      delete this.canvas.dataset.boardSurfaceFailure;
    } else {
      this.canvas.dataset.boardSurfaceState = "pending";
      delete this.canvas.dataset.boardSurfaceFailure;
    }
    this.canvas.dataset.boardRaster = "gpu";
    this.canvas.dataset.boardWorldTiling = this.worldRasterTiling;
    const lod = this.effectiveDrawLodLabel();
    this.drawLodStore.setSnapshot(lod, (left, right) => left === right);
    this.canvas.dataset.boardLod = lod;
    this.canvas.dataset.boardSceneNodeCount = String(this.scene.nodes.size);
    this.canvas.dataset.boardZoom = String(Math.round(this.camera.zoom * 1000) / 1000);
    this.canvas.dataset.boardSelection = sortedSelectionIds(this.selectionIds).join(",");
    this.canvas.dataset.boardHover = this.hoveredId ?? "";
    this.canvas.setAttribute("data-board-camera", `${this.camera.x},${this.camera.y}`);
  }

  private readonly handleContextMenu = (event: MouseEvent): void => {
    if (!this.canvas) {
      return;
    }
    event.preventDefault();
    BoardRenderer.activeRenderer = this;
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    const rect = this.canvas.getBoundingClientRect();
    const sx = event.clientX - rect.left;
    const sy = event.clientY - rect.top;
    this.recordPointerClient(event.clientX, event.clientY, sx, sy);
    this.session.pointerMoveScreen(sx, sy, false, false);
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.publishHover();
    const world = this.screenToWorld({ x: sx, y: sy });
    this.emit("contextmenu", { clientX: event.clientX, clientY: event.clientY, id: this.hoveredId, x: world.x, y: world.y });
    this.invalidate();
  };

  private readonly handlePointerDown = (event: PointerEvent): void => {
    if (!this.canvas) {
      return;
    }
    if (event.button !== 0 && event.button !== 1) {
      return;
    }
    BoardRenderer.activeRenderer = this;
    this.canvas.focus({ preventScroll: true });
    if (typeof event.pointerId === "number") {
      this.canvas.setPointerCapture?.(event.pointerId);
    }
    event.preventDefault();
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      if (typeof event.pointerId === "number") {
        this.canvas.releasePointerCapture?.(event.pointerId);
      }
      return;
    }
    const rect = this.canvas.getBoundingClientRect();
    const sx = event.clientX - rect.left;
    const sy = event.clientY - rect.top;
    this.recordPointerClient(event.clientX, event.clientY, sx, sy);
    this.session.pointerDownScreen(sx, sy, event.button, event.shiftKey, event.ctrlKey || event.metaKey);
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.publishHover();
    this.invalidate();
  };

  private readonly handlePointerMove = (event: PointerEvent): void => {
    if (!this.canvas) {
      return;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    const rect = this.canvas.getBoundingClientRect();
    const sx = event.clientX - rect.left;
    const sy = event.clientY - rect.top;
    this.recordPointerClient(event.clientX, event.clientY, sx, sy);
    this.session.pointerMoveScreen(sx, sy, event.shiftKey, event.ctrlKey || event.metaKey);
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    if (!this.session.isDraggingAreaSelect()) {
      this.publishHover();
    }
    this.invalidate();
  };

  private readonly handlePointerUp = (event: PointerEvent): void => {
    if (!this.canvas) {
      return;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      if (typeof event.pointerId === "number") {
        this.canvas.releasePointerCapture?.(event.pointerId);
      }
      return;
    }
    const rect = this.canvas.getBoundingClientRect();
    const sx = event.clientX - rect.left;
    const sy = event.clientY - rect.top;
    this.recordPointerClient(event.clientX, event.clientY, sx, sy);
    this.session.pointerUpScreen(sx, sy, event.shiftKey, event.ctrlKey || event.metaKey);
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.publishHover();
    if (typeof event.pointerId === "number") {
      this.canvas.releasePointerCapture?.(event.pointerId);
    }
    this.invalidate();
  };

  private readonly handlePointerLeave = (event: PointerEvent): void => {
    if (this.canvas) {
      const rect = this.canvas.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      this.recordPointerClient(event.clientX, event.clientY, sx, sy);
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.session.pointerLeaveScreen();
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.publishHover();
    this.invalidate();
  };

  private readonly handleWheel = (event: WheelEvent): void => {
    if (!this.canvas) {
      return;
    }
    event.preventDefault();
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    const rect = this.canvas.getBoundingClientRect();
    const sx = event.clientX - rect.left;
    const sy = event.clientY - rect.top;
    this.recordPointerClient(event.clientX, event.clientY, sx, sy);
    this.session.wheelScreen(sx, sy, event.deltaY);
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.publishHover();
    this.invalidate();
  };
}
//#endregion 🔖Renderer

//#region 🔖Vitest
const boardVitest = (
  import.meta as ImportMeta & {
    vitest?: {
      describe: typeof import("vitest").describe;
      expect: typeof import("vitest").expect;
      it: typeof import("vitest").it;
      vi: typeof import("vitest").vi;
    };
  }
).vitest;

if (boardVitest) {
  const { beforeAll, describe, expect, it, vi } = boardVitest;

  beforeAll(async () => {
    await ensureElementsBoardWasmLoaded();
  });

  describe("boardFitTextFontPx", () => {
    it("chooses the largest font bounded by line height and measured width", () => {
      const state = { font: "" };
      const ctx: CanvasTextMeasuring = {
        get font() {
          return state.font;
        },
        set font(value: string) {
          state.font = value;
        },
        measureText(text: string) {
          const match = /^(\d+)px/u.exec(state.font);
          const size = match ? Number(match[1]) : 0;
          return { width: size * text.length };
        },
      };
      const fit = boardFitTextFontPx(ctx, "aa", 100, 24, 4, 200, "monospace");
      expect(fit).toBe(20);
    });
  });

  describe("boardEllipsisTextToWidth", () => {
    it("returns the original string when it already fits", () => {
      const ctx: CanvasTextMeasuring = {
        font: "14px monospace",
        measureText: (t: string) => ({ width: t.length * 7 }),
      };
      expect(boardEllipsisTextToWidth(ctx, "short", 400)).toBe("short");
    });

    it("truncates with an ellipsis when the string is too wide", () => {
      const ctx: CanvasTextMeasuring = {
        font: "10px monospace",
        measureText: (t: string) => ({ width: t.length * 8 }),
      };
      const out = boardEllipsisTextToWidth(ctx, "abcdefghij", 50);
      expect(out.endsWith("…")).toBe(true);
      expect(ctx.measureText(out).width).toBeLessThanOrEqual(50);
      expect(out.length).toBeLessThan("abcdefghij".length + 1);
    });
  });

  describe("boardNodeTextPlacementAnchor", () => {
    it("anchors west at the left-middle of the node-centered box", () => {
      const a = boardNodeTextPlacementAnchor(100, 50, 80, 40, "w");
      expect(a).toEqual({ fillX: 60, fillY: 50, textAlign: "left", textBaseline: "middle" });
    });
  });

  function createMockCanvas(width = 800, height = 600): { canvas: HTMLCanvasElement; context: BoardCanvasContext } {
    const canvas = document.createElement("canvas");
    const context = {
      arc: vi.fn(),
      beginPath: vi.fn(),
      bezierCurveTo: vi.fn(),
      clearRect: vi.fn(),
      clip: vi.fn(),
      closePath: vi.fn(),
      fill: vi.fn(),
      fillRect: vi.fn(),
      fillStyle: "#000000",
      fillText: vi.fn(),
      font: "",
      lineCap: "round" as CanvasLineCap,
      lineJoin: "round" as CanvasLineJoin,
      lineTo: vi.fn(),
      lineWidth: 1,
      measureText: vi.fn((s: string) => ({ width: s.length * 6 })),
      moveTo: vi.fn(),
      rect: vi.fn(),
      restore: vi.fn(),
      save: vi.fn(),
      setLineDash: vi.fn(),
      setTransform: vi.fn(),
      stroke: vi.fn(),
      strokeRect: vi.fn(),
      strokeStyle: "#000000",
      textAlign: "start" as CanvasTextAlign,
      textBaseline: "alphabetic" as CanvasTextBaseline,
    } satisfies BoardCanvasContext;
    Object.defineProperty(canvas, "clientWidth", { configurable: true, value: width });
    Object.defineProperty(canvas, "clientHeight", { configurable: true, value: height });
    Object.defineProperty(canvas, "getContext", { configurable: true, value: () => context });
    Object.defineProperty(canvas, "getBoundingClientRect", {
      configurable: true,
      value: () => ({ bottom: height, height, left: 0, right: width, top: 0, width, x: 0, y: 0 }),
    });
    return { canvas, context };
  }

  describe("board hover publication", () => {
    it("emits hover with hit id and pointer/world coordinates after pointermove", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const hovers: BoardHoverPayload[] = [];
      renderer.on("hover", (h) => hovers.push(h));
      const node = new BoardSceneNode({ id: "hover-node", radius: 24, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer.render();
      const p = renderer.worldToScreen({ x: 0, y: 0 });
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, clientX: p.x, clientY: p.y }));
      expect(hovers.length).toBeGreaterThanOrEqual(1);
      const last = hovers.at(-1)!;
      expect(last.id).toBe("hover-node");
      expect(last.clientX).toBeCloseTo(p.x);
      expect(last.worldX).toBeCloseTo(0, 1);
      renderer.dispose();
    });
  });

  describe("board geometry helpers", () => {
    it("places cubic edge control arms along circle normals at the anchors", () => {
      const sourceNode = new BoardSceneNode({ id: "a", radius: 40, x: 0, y: 0 });
      const targetNode = new BoardSceneNode({ id: "b", radius: 40, x: 300, y: 0 });
      const sourceHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: sourceNode });
      const targetHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "b:h0", node: targetNode });
      const curve = computeEdgeBezier(sourceHandle, targetHandle);

      expect(curve.p0.x).toBeCloseTo(40);
      expect(curve.p0.y).toBeCloseTo(0);
      expect(curve.p3.x).toBeCloseTo(260);
      expect(curve.p3.y).toBeCloseTo(0);
      const sourceRadial0 = { x: curve.p0.x - sourceNode.x, y: curve.p0.y - sourceNode.y };
      const arm0 = { x: curve.p1.x - curve.p0.x, y: curve.p1.y - curve.p0.y };
      const targetApproach1 = { x: targetNode.x - curve.p3.x, y: targetNode.y - curve.p3.y };
      const arm1 = { x: curve.p3.x - curve.p2.x, y: curve.p3.y - curve.p2.y };
      const align0 = (sourceRadial0.x * arm0.x + sourceRadial0.y * arm0.y) / (Math.hypot(sourceRadial0.x, sourceRadial0.y) * Math.hypot(arm0.x, arm0.y));
      const align1 = Math.abs((targetApproach1.x * arm1.x + targetApproach1.y * arm1.y) / (Math.hypot(targetApproach1.x, targetApproach1.y) * Math.hypot(arm1.x, arm1.y)));
      expect(align0).toBeGreaterThan(0.99);
      expect(align1).toBeGreaterThan(0.99);
    });

    it("builds a radial cubic for a wire whose far end is a free world point", () => {
      const sourceNode = new BoardSceneNode({ id: "a", radius: 40, x: 0, y: 0 });
      const sourceHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: sourceNode });
      const curve = computeWireBezier(sourceHandle, { x: 200, y: 100 });
      expect(curve.p0.x).toBeCloseTo(40);
      expect(curve.p0.y).toBeCloseTo(0);
      expect(curve.p3.x).toBeCloseTo(200);
      expect(curve.p3.y).toBeCloseTo(100);
    });

    it("places rectangle handles on the perimeter by north-zero CCW angle", () => {
      const rectNode = new BoardSceneNode({ height: 20, id: "r", shape: "rectangle", width: 40, x: 100, y: 50 });
      const p0 = computeHandlePosition(rectNode, 0);
      expect(p0.x).toBeCloseTo(100);
      expect(p0.y).toBeCloseTo(40);
      const pW = computeHandlePosition(rectNode, Math.PI / 2);
      expect(pW.x).toBeCloseTo(80);
      expect(pW.y).toBeCloseTo(50);
      const pS = computeHandlePosition(rectNode, Math.PI);
      expect(pS.x).toBeCloseTo(100);
      expect(pS.y).toBeCloseTo(60);
      const pE = computeHandlePosition(rectNode, (3 * Math.PI) / 2);
      expect(pE.x).toBeCloseTo(120);
      expect(pE.y).toBeCloseTo(50);
    });

    it("labels minimap, overview, compact, normal, detail, and micro LOD bands from zoom thresholds", () => {
      expect(resolveBoardLodLabel(0.1)).toBe("minimap");
      expect(resolveBoardLodLabel(0.25)).toBe("overview");
      expect(resolveBoardLodLabel(0.4)).toBe("compact");
      expect(resolveBoardLodLabel(0.9)).toBe("normal");
      expect(resolveBoardLodLabel(1.3)).toBe("detail");
      expect(resolveBoardLodLabel(2.6)).toBe("micro");
      const tight: BoardLodZoomThresholds = {
        minimapMaxZoom: 0.2,
        overviewMaxZoom: 0.35,
        compactMaxZoom: 0.45,
        normalMaxZoom: 0.6,
        detailMaxZoom: 1,
      };
      expect(resolveBoardLodLabelFromThresholds(0.15, tight)).toBe("minimap");
      expect(resolveBoardLodLabelFromThresholds(0.3, tight)).toBe("overview");
      expect(resolveBoardLodLabelFromThresholds(0.4, tight)).toBe("compact");
      expect(resolveBoardLodLabelFromThresholds(0.5, tight)).toBe("normal");
      expect(resolveBoardLodLabelFromThresholds(0.7, tight)).toBe("detail");
      expect(resolveBoardLodLabelFromThresholds(1.1, tight)).toBe("micro");
    });

    it("switches caption policy across the six LOD bands", () => {
      expect(boardTextOverlayCaptionForLod("Node Label", "minimap", null)).toBeNull();
      expect(boardTextOverlayCaptionForLod("Node Label", "overview", null)).toBeNull();
      expect(boardTextOverlayCaptionForLod("Node Label", "compact", null)).toBe("Node La…");
      expect(boardTextOverlayCaptionForLod("Node Label", "normal", null)).toBe("Node La…");
      expect(boardTextOverlayCaptionForLod("Node Label", "detail", "catalog-icon")).toBe("Node La…");
      expect(boardTextOverlayCaptionForLod("0123456789012345", "micro", null)).toBe("01234567890…");
      expect(boardHandleOverlayCaptionForLod("Handle Label", "compact")).toBeNull();
      expect(boardHandleOverlayCaptionForLod("Handle Label", "detail")).toBe("Handl…");
      expect(boardHandleOverlayCaptionForLod("Handle Label", "micro")).toBe("Handle …");
    });
  });

  describe("board renderer render pipeline", () => {
    it("coalesces nested render calls from frame listeners instead of re-entering the render pass", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      let frameCount = 0;
      renderer.subscribeFrame(() => {
        frameCount += 1;
        if (frameCount === 1) {
          renderer.render();
        }
      });
      renderer.render();
      expect(frameCount).toBe(1);
      renderer.dispose();
    });
  });

  describe("board scene", () => {
    it("stores nodes, handles, and edges with stable ids and emits edge creation", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const edgeEvents: Array<{ id: string; source: string; target: string }> = [];
      renderer.on("edgeCreate", (event) => edgeEvents.push(event));

      const sourceNode = new BoardSceneNode({ id: "source", radius: 36, x: 0, y: 0 });
      const targetNode = new BoardSceneNode({ id: "target", radius: 36, x: 220, y: 80 });
      const sourceHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new BoardSceneEdge({ source: sourceHandle, id: "edge-1", target: targetHandle });

      renderer.scene.add(sourceNode).add(targetNode).add(edge);

      expect(renderer.scene.getObjectById("source")).toBe(sourceNode);
      expect(renderer.scene.getObjectById("src-h")).toBe(sourceHandle);
      expect(renderer.scene.getObjectById("edge-1")).toBe(edge);
      expect(edgeEvents).toEqual([{ id: "edge-1", source: "src-h", target: "tgt-h" }]);

      renderer.dispose();
    });

    it("stores wires and drops them when an endpoint handle is removed", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const created: string[] = [];
      const destroyed: string[] = [];
      renderer.on("wireCreate", (e) => created.push(e.id));
      renderer.on("wireDestroy", (e) => destroyed.push(e.id));
      const n = new BoardSceneNode({ id: "n", radius: 22, x: 0, y: 0 });
      const h = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "h0", node: n });
      const w = new BoardSceneWire({ endX: 90, endY: 0, id: "w-1", source: h, target: null });
      renderer.scene.add(n).add(w);
      expect(renderer.scene.wires.get("w-1")).toBe(w);
      expect(w.kind).toBe("wire");
      expect(created).toEqual(["w-1"]);
      renderer.scene.remove(h);
      expect(renderer.scene.wires.has("w-1")).toBe(false);
      expect(destroyed).toEqual(["w-1"]);
      renderer.dispose();
    });

    it("creates an edge when linking two handles with a pointer drag through WASM", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      renderer.setCamera(0, 0, BOARD_LOD_DETAIL_MIN_ZOOM);
      const edgeEvents: Array<{ id: string; source: string; target: string }> = [];
      renderer.on("edgeCreate", (event) => edgeEvents.push(event));

      const a = new BoardSceneNode({ id: "a", radius: 40, x: 0, y: 0 });
      const b = new BoardSceneNode({ id: "b", radius: 40, x: 280, y: 0 });
      new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: a });
      new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "b:h0", node: b });
      renderer.scene.add(a).add(b);
      renderer.render();

      const p0 = renderer.worldToScreen(computeHandlePosition(a, 0));
      const pMid = renderer.worldToScreen({ x: 140, y: 0 });
      const p1 = renderer.worldToScreen(computeHandlePosition(b, Math.PI));

      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: p0.x, clientY: p0.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, clientX: pMid.x + 20, clientY: pMid.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, clientX: p1.x, clientY: p1.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: p1.x, clientY: p1.y }));

      expect(edgeEvents.length).toBe(1);
      expect(renderer.scene.edges.has(edgeEvents[0].id)).toBe(true);
      expect(edgeEvents[0].source).toBe("a:h0");
      expect(edgeEvents[0].target).toBe("b:h0");

      renderer.dispose();
    });

    it("dispose does not emit structural delete events that would clear play fixtures", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const nodeDeletes: string[] = [];
      renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));
      const root = new BoardSceneNode({ id: "keep", radius: 24, x: 0, y: 0 });
      renderer.scene.add(root);
      renderer.dispose();
      expect(nodeDeletes).toEqual([]);
    });

    it("deletes selected edges and nodes when Delete reaches the window listener after pointerdown", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const edgeDeletes: string[] = [];
      const nodeDeletes: string[] = [];
      renderer.on("edgeDelete", (event) => edgeDeletes.push(event.id));
      renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));

      const sourceNode = new BoardSceneNode({ id: "source", radius: 36, x: 0, y: 0 });
      const targetNode = new BoardSceneNode({ id: "target", radius: 36, x: 220, y: 0 });
      const sourceHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new BoardSceneEdge({ source: sourceHandle, id: "edge-1", target: targetHandle });
      renderer.scene.add(sourceNode).add(targetNode).add(edge);
      renderer.render();

      canvas.focus();
      const mid = cubicBezierPoint(edge.curve, 0.5);
      const screen = renderer.worldToScreen(mid);
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: screen.x, clientY: screen.y }));
      window.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key: "Delete" }));

      expect(renderer.scene.edges.has("edge-1")).toBe(false);
      expect(edgeDeletes).toEqual(["edge-1"]);
      expect(renderer.selection.getSnapshot().ids).toEqual([]);

      const nodeScreen = renderer.worldToScreen({ x: sourceNode.x, y: sourceNode.y });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: nodeScreen.x, clientY: nodeScreen.y }));
      window.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key: "Delete" }));

      expect(renderer.scene.nodes.has("source")).toBe(false);
      expect(nodeDeletes).toContain("source");

      renderer.dispose();
    });

    it("does not delete the board selection while a text field owns focus", () => {
      const { canvas } = createMockCanvas();
      document.body.appendChild(canvas);
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const sourceNode = new BoardSceneNode({ id: "source", radius: 36, x: 0, y: 0 });
      const targetNode = new BoardSceneNode({ id: "target", radius: 36, x: 220, y: 0 });
      const sourceHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new BoardSceneEdge({ source: sourceHandle, id: "edge-1", target: targetHandle });
      renderer.scene.add(sourceNode).add(targetNode).add(edge);
      renderer.render();

      const mid = cubicBezierPoint(edge.curve, 0.5);
      const screen = renderer.worldToScreen(mid);
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: screen.x, clientY: screen.y }));

      const input = document.createElement("input");
      document.body.appendChild(input);
      input.focus();
      window.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key: "Delete" }));
      expect(renderer.scene.edges.has("edge-1")).toBe(true);

      input.remove();
      window.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key: "Delete" }));
      expect(renderer.scene.edges.has("edge-1")).toBe(false);

      renderer.dispose();
      canvas.remove();
    });

    it("moves a selected draggable node from pointer events without React involvement", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const movableNode = new BoardSceneNode({ draggable: true, id: "movable", radius: 30, x: 0, y: 0 });
      renderer.scene.add(movableNode);
      renderer.render();

      const downEvent = new MouseEvent("pointerdown", { button: 0, clientX: 400, clientY: 300 });
      const moveEvent = new MouseEvent("pointermove", { button: 0, clientX: 460, clientY: 340 });
      const upEvent = new MouseEvent("pointerup", { button: 0, clientX: 460, clientY: 340 });
      canvas.dispatchEvent(downEvent);
      canvas.dispatchEvent(moveEvent);
      canvas.dispatchEvent(upEvent);

      expect(renderer.selection.getSnapshot().ids).toEqual(["movable"]);
      expect(movableNode.x).toBeCloseTo(60);
      expect(movableNode.y).toBeCloseTo(40);

      renderer.dispose();
    });

    it("moves every selected draggable node when dragging one of them", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const a = new BoardSceneNode({ draggable: true, id: "a", radius: 20, x: 0, y: 0 });
      const b = new BoardSceneNode({ draggable: true, id: "b", radius: 20, x: 100, y: 0 });
      renderer.scene.add(a).add(b);
      renderer.render();
      renderer.setSelectionIds(["a", "b"]);
      const screenA = renderer.worldToScreen({ x: 0, y: 0 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: screenA.x, clientY: screenA.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: screenA.x + 10, clientY: screenA.y + 5 }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: screenA.x + 10, clientY: screenA.y + 5 }));
      expect(a.x).toBeCloseTo(10, 5);
      expect(a.y).toBeCloseTo(5, 5);
      expect(b.x).toBeCloseTo(110, 5);
      expect(b.y).toBeCloseTo(5, 5);
      expect([...renderer.selection.getSnapshot().ids].sort()).toEqual(["a", "b"]);
      renderer.dispose();
    });

    it("keeps imperative node coordinates when declarative props still show pre-drag authoring values", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const node = new BoardSceneNode({ id: "n", radius: 10, x: 10, y: 20 });
      renderer.scene.add(node);
      renderer.applyNodePositionFromProps(node.id, 10, 20, node);
      node.setPosition(100, 200);
      renderer.applyNodePositionFromProps(node.id, 10, 20, node);
      expect(node.x).toBe(100);
      expect(node.y).toBe(200);
      renderer.applyNodePositionFromProps(node.id, 5, 6, node);
      expect(node.x).toBe(5);
      expect(node.y).toBe(6);
      renderer.dispose();
    });

    it("boardElementInteractionChrome maps preselect preview and anchor removal", () => {
      expect(boardElementInteractionChrome(["a"], { ids: ["a", "b"], removedIds: ["a"] })).toEqual({
        highlightedIds: new Set(["a"]),
        selectedIds: new Set(["b"]),
      });
      expect(boardElementInteractionChrome(["a"], BOARD_PRESELECT_EMPTY)).toEqual({
        highlightedIds: new Set(),
        selectedIds: new Set(["a"]),
      });
    });

    it("applies imperative selection via setSelectionIds", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const sourceNode = new BoardSceneNode({ id: "source", radius: 20, x: 0, y: 0 });
      const targetNode = new BoardSceneNode({ id: "target", radius: 20, x: 100, y: 0 });
      renderer.scene.add(sourceNode).add(targetNode);
      renderer.setSelectionIds(["target"]);
      expect(renderer.selection.getSnapshot().ids).toEqual(["target"]);
      expect(targetNode.selected).toBe(true);
      expect(sourceNode.selected).toBe(false);
      renderer.dispose();
    });

    it("syncs selection silently for controlled hosts without emitting select", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const node = new BoardSceneNode({ id: "solo", radius: 20, x: 0, y: 0 });
      renderer.scene.add(node);
      const selects: BoardSelectionSnapshot[] = [];
      renderer.on("select", (snap) => selects.push(snap));
      renderer.setSelectionIdsSilent(["solo"]);
      expect(renderer.selection.getSnapshot().ids).toEqual(["solo"]);
      expect(selects).toEqual([]);
      renderer.dispose();
    });

    it("keeps committed selection empty during rectangle preselect from empty", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test", selection: { mode: "additive" } });
      const node = new BoardSceneNode({ id: "solo", radius: 20, x: 200, y: 0 });
      renderer.scene.add(node);
      renderer.render();
      const s0 = renderer.worldToScreen({ x: 120, y: -40 });
      const s1 = renderer.worldToScreen({ x: 280, y: 40 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: s0.x, clientY: s0.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual([]);
      expect(renderer.preselection.getSnapshot().removedIds).toEqual([]);
      expect(node.selected).toBe(true);
      expect(node.highlighted).toBe(false);
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual(["solo"]);
      renderer.dispose();
    });

    it("emits preselect while rectangle-selecting and clears on commit", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test", selection: { mode: "additive" } });
      const a = new BoardSceneNode({ id: "a", radius: 20, x: 0, y: 0 });
      const b = new BoardSceneNode({ id: "b", radius: 20, x: 120, y: 0 });
      renderer.scene.add(a).add(b);
      renderer.render();
      const preselects: BoardPreselectSnapshot[] = [];
      renderer.on("preselect", (snap) => preselects.push(snap));
      const s0 = renderer.worldToScreen({ x: -40, y: -40 });
      const s1 = renderer.worldToScreen({ x: 160, y: 40 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: s0.x, clientY: s0.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));
      expect(preselects.length).toBeGreaterThan(0);
      expect(preselects.at(-1)?.ids.includes("b")).toBe(true);
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual(["a", "b"]);
      expect(renderer.preselection.getSnapshot()).toEqual(BOARD_PRESELECT_EMPTY);
      renderer.dispose();
    });

    it("syncPreselectionSilent applies selected chrome on scene objects", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const a = new BoardSceneNode({ id: "a", radius: 20, x: 0, y: 0 });
      const b = new BoardSceneNode({ id: "b", radius: 20, x: 200, y: 0 });
      renderer.scene.add(a).add(b);
      renderer.setSelectionIds(["a"]);
      renderer.syncPreselectionSilent({ ids: ["b"], removedIds: [] });
      expect(b.selected).toBe(true);
      expect(b.highlighted).toBe(false);
      expect(a.selected).toBe(false);
      expect(a.highlighted).toBe(false);
      renderer.dispose();
    });

    it("splits preselect preview into selected and highlighted scene chrome", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const a = new BoardSceneNode({ id: "a", radius: 20, x: 0, y: 0 });
      const b = new BoardSceneNode({ id: "b", radius: 20, x: 200, y: 0 });
      renderer.scene.add(a).add(b);
      renderer.setSelectionIds(["a"]);
      renderer.syncPreselectionSilent({ ids: ["b"], removedIds: ["a"] });
      expect(b.selected).toBe(true);
      expect(b.highlighted).toBe(false);
      expect(a.selected).toBe(false);
      expect(a.highlighted).toBe(true);
      expect(renderer.selection.getSnapshot().ids).toEqual(["a"]);
      renderer.dispose();
    });

    it("cancels area-select on Escape without changing committed selection", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const a = new BoardSceneNode({ id: "a", radius: 20, x: 0, y: 0 });
      const b = new BoardSceneNode({ id: "b", radius: 20, x: 200, y: 0 });
      renderer.scene.add(a).add(b);
      renderer.setSelectionIds(["a"]);
      renderer.render();
      const cancels: BoardPreselectSnapshot[] = [];
      renderer.on("preselectCancel", (snap) => cancels.push(snap));
      const s0 = renderer.worldToScreen({ x: 120, y: -40 });
      const s1 = renderer.worldToScreen({ x: 280, y: 40 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: s0.x, clientY: s0.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));
      expect(renderer.preselection.getSnapshot().ids.length).toBeGreaterThan(0);
      window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
      expect(cancels.length).toBe(1);
      expect(renderer.selection.getSnapshot().ids).toEqual(["a"]);
      expect(renderer.preselection.getSnapshot()).toEqual(BOARD_PRESELECT_EMPTY);
      expect(a.selected).toBe(true);
      expect(b.selected).toBe(false);
      renderer.dispose();
    });

    it("opens rectangle selection from a left-button drag and applies directional partial versus enclosing rules", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test", selection: { mode: "additive" } });
      const node = new BoardSceneNode({ id: "node", radius: 20, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer.render();

      const rightDragStart = renderer.worldToScreen({ x: -10, y: -30 });
      const rightDragEnd = renderer.worldToScreen({ x: 10, y: 30 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: rightDragStart.x, clientY: rightDragStart.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: rightDragEnd.x, clientY: rightDragEnd.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: rightDragEnd.x, clientY: rightDragEnd.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual([]);

      const leftDragStart = renderer.worldToScreen({ x: 30, y: -30 });
      const leftDragEnd = renderer.worldToScreen({ x: -10, y: 30 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: leftDragStart.x, clientY: leftDragStart.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: leftDragEnd.x, clientY: leftDragEnd.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: leftDragEnd.x, clientY: leftDragEnd.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual(["node"]);

      renderer.dispose();
    });

    it("clears selection when clicking the background without dragging", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const node = new BoardSceneNode({ draggable: true, id: "solo", radius: 36, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer.render();

      const onNode = renderer.worldToScreen({ x: 0, y: 0 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: onNode.x, clientY: onNode.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: onNode.x, clientY: onNode.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual(["solo"]);

      const background = renderer.worldToScreen({ x: 900, y: 900 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: background.x, clientY: background.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: background.x, clientY: background.y }));

      expect(renderer.selection.getSnapshot().ids).toEqual([]);
      expect(node.selected).toBe(false);
      expect(node.highlighted).toBe(false);
      renderer.dispose();
    });

    it("boardInteractionChromeStyleKey follows selection ids not stale scene flags", () => {
      const node = new BoardSceneNode({ id: "solo", radius: 20, x: 0, y: 0 });
      node.selected = true;
      node.highlighted = false;
      const chrome = boardElementInteractionChrome([], BOARD_PRESELECT_EMPTY);
      expect(boardInteractionChromeStyleKey("node", node.id, chrome)).toBe("node");
      const chromeSel = boardElementInteractionChrome(["solo"], BOARD_PRESELECT_EMPTY);
      expect(boardInteractionChromeStyleKey("node", node.id, chromeSel)).toBe("node.selected");
    });

    it("stale silent selection sync undoes background deselect until controlled prop updates", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const node = new BoardSceneNode({ draggable: true, id: "solo", radius: 36, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer.render();

      const onNode = renderer.worldToScreen({ x: 0, y: 0 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: onNode.x, clientY: onNode.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: onNode.x, clientY: onNode.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual(["solo"]);

      const background = renderer.worldToScreen({ x: 900, y: 900 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: background.x, clientY: background.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: background.x, clientY: background.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual([]);

      renderer.setSelectionIdsSilent(["solo"]);
      expect(renderer.selection.getSnapshot().ids).toEqual(["solo"]);
      renderer.dispose();
    });

    it("includes handles in rectangle selection with nodes and edges target", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({
        canvas,
        renderMode: "headless-test",
        selection: { method: "rectangle", mode: "invertive", targets: { ...BOARD_SELECTION_TARGETS_DEFAULT } },
      });
      const sourceNode = new BoardSceneNode({ id: "source", radius: 40, x: 0, y: 0 });
      const targetNode = new BoardSceneNode({ id: "target", radius: 40, x: 200, y: 0 });
      const sourceHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new BoardSceneEdge({ source: sourceHandle, id: "edge-1", target: targetHandle });
      renderer.scene.add(sourceNode).add(targetNode).add(edge);
      renderer.render();

      const s0 = renderer.worldToScreen({ x: -90, y: -70 });
      const s1 = renderer.worldToScreen({ x: 90, y: 70 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: s0.x, clientY: s0.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));

      const ids = renderer.selection.getSnapshot().ids;
      expect(ids.includes("source")).toBe(true);
      expect(ids.includes("src-h")).toBe(true);
      renderer.dispose();
    });

    it("supports lasso targets and additive subtractive invertive selection modes", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test", selection: { method: "lasso", mode: "additive", targets: { nodes: false, edges: true, handles: false } } });
      const sourceNode = new BoardSceneNode({ id: "source", radius: 12, x: -80, y: 0 });
      const targetNode = new BoardSceneNode({ id: "target", radius: 12, x: 80, y: 0 });
      const sourceHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new BoardSceneEdge({ source: sourceHandle, id: "edge", target: targetHandle });
      renderer.scene.add(sourceNode).add(targetNode).add(edge);
      renderer.render();

      const drawLasso = (points: Point[]): void => {
        const [start, ...rest] = points.map((point) => renderer.worldToScreen(point));
        canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: start.x, clientY: start.y }));
        for (const point of rest) {
          canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: point.x, clientY: point.y }));
        }
        const end = rest.at(-1) ?? start;
        canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: end.x, clientY: end.y }));
      };

      drawLasso([
        { x: 30, y: -30 },
        { x: -30, y: -30 },
        { x: -30, y: 30 },
        { x: 30, y: 30 },
        { x: -30, y: 0 },
      ]);
      expect(renderer.selection.getSnapshot().ids).toEqual(["edge"]);

      renderer.setSelectionOptions({ method: "rectangle", mode: "subtractive", targets: { nodes: false, edges: true, handles: false } });
      const subtractStart = renderer.worldToScreen({ x: 20, y: -10 });
      const subtractEnd = renderer.worldToScreen({ x: -20, y: 10 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: subtractStart.x, clientY: subtractStart.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: subtractEnd.x, clientY: subtractEnd.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: subtractEnd.x, clientY: subtractEnd.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual([]);

      renderer.setSelectionOptions({ mode: "invertive", targets: { nodes: true, edges: false, handles: false } });
      renderer.setSelectionIds(["source"]);
      const invertStart = renderer.worldToScreen({ x: 100, y: -30 });
      const invertEnd = renderer.worldToScreen({ x: -100, y: 30 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: invertStart.x, clientY: invertStart.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: invertEnd.x, clientY: invertEnd.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: invertEnd.x, clientY: invertEnd.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual(["target"]);

      renderer.dispose();
    });

    it("maps default selection to replace and honors Ctrl and Shift modifiers", () => {
      const { canvas } = createMockCanvas();
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test", selection: { mode: "default", targets: { nodes: true, edges: false, handles: false } } });
      const a = new BoardSceneNode({ id: "a", radius: 12, x: 0, y: 0 });
      const b = new BoardSceneNode({ id: "b", radius: 12, x: 80, y: 0 });
      renderer.scene.add(a).add(b);
      renderer.render();

      const clickNode = (id: "a" | "b", init: { ctrlKey?: boolean; metaKey?: boolean; shiftKey?: boolean } = {}): void => {
        const p = renderer.worldToScreen(id === "a" ? { x: 0, y: 0 } : { x: 80, y: 0 });
        canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: p.x, clientY: p.y, ...init }));
        canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: p.x, clientY: p.y, ...init }));
      };

      renderer.setSelectionIds(["a"]);
      clickNode("b", { shiftKey: true });
      expect(renderer.selection.getSnapshot().ids).toEqual(["a", "b"]);

      clickNode("a", { ctrlKey: true });
      expect(renderer.selection.getSnapshot().ids).toEqual(["b"]);

      clickNode("b", { ctrlKey: true, shiftKey: true });
      expect(renderer.selection.getSnapshot().ids).toEqual([]);

      clickNode("a");
      expect(renderer.selection.getSnapshot().ids).toEqual(["a"]);
      renderer.dispose();
    });
  });

  describe("board fixture io", () => {
    it("parses minimal v1 fixture payloads", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 1, y: 2, zoom: 0.5 },
        edges: [{ id: "e1", source: "a:h0", target: "b:h0" }],
        meta: {},
        nodes: [
          { handles: [{ angle: 0, id: "a:h0" }], id: "a", radius: 10, text: "α", x: 0, y: 0 },
          { handles: [{ angle: 3.14, id: "b:h0" }], id: "b", radius: 10, x: 50, y: 0 },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(parsed).not.toBeNull();
      expect(parsed?.nodes).toHaveLength(2);
      expect(parsed?.nodes[0]?.handles[0]?.handleKind).toBe(BUILTIN_PORT_HANDLE_KIND);
      expect(parsed?.nodes[0]).toMatchObject({ id: "a", shape: "circle", radius: 10, text: "α" });
      expect(parsed?.nodes[1]).toMatchObject({ id: "b", shape: "circle" });
      expect(parsed?.edges[0]?.id).toBe("e1");
      expect(parsed?.camera.zoom).toBe(0.5);
    });

    it("parses rectangle fixture nodes", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          {
            handles: [{ angle: 0, id: "box:h0" }],
            height: 24,
            id: "box",
            shape: "rectangle",
            text: "crate",
            width: 48,
            x: 10,
            y: -5,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(parsed?.nodes[0]).toMatchObject({ shape: "rectangle", width: 48, height: 24, id: "box", text: "crate" });
    });

    it("parses optional iconKind on fixture nodes", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          {
            handles: [{ angle: 0, id: "ic.h" }],
            iconKind: '  <svg xmlns="http://www.w3.org/2000/svg"/> ',
            id: "ic",
            radius: 10,
            x: 0,
            y: 0,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(parsed?.nodes[0]).toMatchObject({
        id: "ic",
        iconKind: '<svg xmlns="http://www.w3.org/2000/svg"/>',
      });
    });

    it("parses optional metabolism catalog iconKind on fixture nodes", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          {
            handles: [{ angle: 0, id: "nk.h" }],
            iconKind: "  capsule-with-balcony_p  ",
            id: "nk",
            radius: 10,
            x: 0,
            y: 0,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(parsed?.nodes[0]).toMatchObject({ id: "nk", iconKind: "capsule-with-balcony_p" });
    });

    it("parses optional iconKind on fixture handles", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          {
            handles: [{ angle: 0, id: "hk.h", iconKind: "  typst:$1+1$  " }],
            id: "hk",
            radius: 10,
            x: 0,
            y: 0,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      const n = parsed?.nodes[0];
      expect(n && "handles" in n ? n.handles[0] : undefined).toMatchObject({ id: "hk.h", iconKind: "typst:$1+1$" });
    });

    it("classifies board icon selector modes for UI tabs", () => {
      expect(classifyElementsBoardIconSelectorMode("")).toBe("math");
      expect(classifyElementsBoardIconSelectorMode("typst:$x$")).toBe("math");
      expect(classifyElementsBoardIconSelectorMode("$x$")).toBe("math");
      expect(classifyElementsBoardIconSelectorMode("emoji:😀")).toBe("emoji");
      expect(classifyElementsBoardIconSelectorMode("data:image/png;base64,abc")).toBe("data");
      expect(classifyElementsBoardIconSelectorMode("image:data:image/jpeg;base64,xyz")).toBe("data");
      expect(classifyElementsBoardIconSelectorMode("<svg")).toBe("vector");
      expect(classifyElementsBoardIconSelectorMode("capsule-with-balcony_p")).toBe("vector");
      expect(classifyElementsBoardIconSelectorMode("😀")).toBe("emoji");
    });

    it("parses optional textAutofit on fixture nodes", () => {
      const circle = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [{ handles: [{ angle: 0, id: "c.h" }], id: "c", radius: 12, text: "cap", textAutofit: true, x: 0, y: 0 }],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(circle?.nodes[0]).toMatchObject({ id: "c", textAutofit: true, text: "cap" });
      const rect = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          {
            handles: [{ angle: 0, id: "r.h" }],
            height: 20,
            id: "r",
            shape: "rectangle",
            text: "wide",
            textAutofit: true,
            width: 80,
            x: 0,
            y: 0,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(rect?.nodes[0]).toMatchObject({ id: "r", textAutofit: true });
    });

    it("parses optional caption font, size, and alignment", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          {
            handles: [{ angle: 0, id: "z.h" }],
            id: "z",
            radius: 8,
            text: "z",
            textAlignment: "ne",
            textFontFamily: " Georgia ",
            textFontSize: 18,
            x: 0,
            y: 0,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(parsed?.nodes[0]).toMatchObject({
        id: "z",
        textFontFamily: "Georgia",
        textFontSize: 18,
        textAlignment: "ne",
      });
      expect(
        parseBoardFixtureV1({
          camera: { x: 0, y: 0, zoom: 1 },
          edges: [],
          nodes: [{ handles: [{ angle: 0, id: "bad.aln" }], id: "bad", radius: 3, textAlignment: "xx", x: 0, y: 0 }],
          schema: "puzzle.2d.fixture/v1",
        })?.nodes[0],
      ).not.toHaveProperty("textAlignment");
    });

    it("parses optional handle radius on fixture nodes", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          {
            handles: [{ angle: 1.2, id: "h1", radius: 4.5 }],
            height: 20,
            id: "r1",
            shape: "rectangle",
            width: 30,
            x: 0,
            y: 0,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(parsed?.nodes[0]?.handles[0]).toMatchObject({ angle: 1.2, id: "h1", radius: 4.5 });
    });

    it("rejects fixture edges that use legacy `from`/`to` instead of source/target", () => {
      expect(
        parseBoardFixtureV1({
          camera: { x: 0, y: 0, zoom: 1 },
          edges: [{ from: "a:h0", id: "e1", to: "b:h0" }],
          nodes: [
            { handles: [{ angle: 0, id: "a:h0" }], id: "a", radius: 20, x: 0, y: 0 },
            { handles: [{ angle: 3.14, id: "b:h0" }], id: "b", radius: 20, x: 100, y: 0 },
          ],
          schema: "puzzle.2d.fixture/v1",
        }),
      ).toBeNull();
    });

    it("rejects fixture nodes that use legacy `label` instead of `text`", () => {
      expect(
        parseBoardFixtureV1({
          camera: { x: 0, y: 0, zoom: 1 },
          edges: [],
          nodes: [{ handles: [{ angle: 0, id: "n1.h" }], id: "n1", label: "legacy", radius: 5, x: 0, y: 0 }],
          schema: "puzzle.2d.fixture/v1",
        }),
      ).toBeNull();
    });

    it("rejects WASM kind catalog rows that use legacy `label` instead of `name`", () => {
      const session = new BoardSession();
      expect(() =>
        session.setKindCatalogsJson(
          JSON.stringify({
            handleKinds: [{ id: "h", label: "legacy", color: "#112233" }],
          }),
        ),
      ).toThrow(/legacy label/);
    });

    it("parses optional nodeKind on circle and rectangle fixture nodes", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          { handles: [{ angle: 0, id: "c:h" }], id: "c", nodeKind: "semio.kit.node.a", radius: 4, x: 0, y: 0 },
          {
            handles: [{ angle: 1, id: "r:h" }],
            height: 8,
            id: "r",
            nodeKind: "semio.kit.node.b",
            shape: "rectangle",
            width: 6,
            x: 1,
            y: 2,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(parsed?.nodes[0]).toMatchObject({ id: "c", nodeKind: "semio.kit.node.a" });
      expect(parsed?.nodes[1]).toMatchObject({ id: "r", nodeKind: "semio.kit.node.b" });
    });

    it("mergeKindCatalogBundleByRowId overlays rows by id", () => {
      const merged = mergeKindCatalogBundleByRowId(DEFAULT_KIND_CATALOG_BUNDLE, {
        handles: [{ color: "#ff0000", defaultWireKind: BUILTIN_LINK_WIRE_KIND, id: BUILTIN_PORT_HANDLE_KIND, name: "Patched" }],
        nodes: [{ id: "semio.metabolism.light.node.x", name: "Capsule" }],
      });
      expect(merged.handles?.find((h) => h.id === BUILTIN_PORT_HANDLE_KIND)?.name).toBe("Patched");
      expect(merged.handles?.find((h) => h.id === BUILTIN_PORT_HANDLE_KIND)?.color).toBe("#ff0000");
      expect(merged.nodes?.some((n) => n.id === "semio.metabolism.light.node.x")).toBe(true);
    });

    it("maps kit piece name to node text", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          {
            handles: [{ angle: 0, id: "a:h" }],
            height: 10,
            id: "a",
            shape: "rectangle",
            text: "cs_sl0_d0_t_f0_b_c0",
            width: 12,
            x: 1,
            y: 2,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(parsed?.nodes[0]).toMatchObject({
        id: "a",
        shape: "rectangle",
        text: "cs_sl0_d0_t_f0_b_c0",
      });
      expect(boardFixtureNodeCaption(parsed!.nodes[0]!)).toBe("cs_sl0_d0_t_f0_b_c0");
    });

    it("rejects wrong schema or malformed nodes", () => {
      expect(parseBoardFixtureV1({ schema: "other", nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } })).toBeNull();
      expect(parseBoardFixtureV1({ schema: "puzzle.2d.fixture/v1", nodes: "x", edges: [], camera: { x: 0, y: 0, zoom: 1 } })).toBeNull();
      expect(
        parseBoardFixtureV1({
          camera: { x: 0, y: 0, zoom: 1 },
          edges: [],
          nodes: [{ handles: [], id: "bad", shape: "triangle", x: 0, y: 0 }],
          schema: "puzzle.2d.fixture/v1",
        }),
      ).toBeNull();
    });

    it("round-trips drag codec for v1 fixtures", () => {
      const fixture: BoardFixtureV1 = {
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [{ id: "e1", source: "a:h0", target: "b:h0" }],
        meta: {},
        nodes: [
          {
            handles: [{ angle: 0, handleKind: BUILTIN_PORT_HANDLE_KIND, id: "a:h0" }],
            id: "a",
            radius: 10,
            shape: "circle",
            text: "A",
            textAlignment: "c",
            textAutofit: true,
            textFontSize: 11,
            x: 0,
            y: 0,
          },
          {
            handles: [{ angle: 3.14, handleKind: BUILTIN_PORT_HANDLE_KIND, id: "b:h0" }],
            id: "b",
            radius: 10,
            shape: "circle",
            text: "B",
            x: 50,
            y: 0,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      };
      const decoded = decodeBoardFixtureFromDragV1(encodeBoardFixtureForDragV1(fixture));
      expect(decoded).toEqual(fixture);
    });

    it("parses optional root on fixture nodes", () => {
      const parsed = parseBoardFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [
          { handles: [{ angle: 0, id: "r.h" }], id: "r", radius: 6, root: true, x: 0, y: 0 },
          {
            handles: [{ angle: 0, id: "sq.h" }],
            height: 10,
            id: "sq",
            root: false,
            shape: "rectangle",
            width: 10,
            x: 1,
            y: 2,
          },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(parsed?.nodes[0]).toMatchObject({ id: "r", root: true });
      expect(parsed?.nodes[1]).toMatchObject({ id: "sq", shape: "rectangle" });
      expect(parsed?.nodes[1]).not.toHaveProperty("root");
    });
  });

  describe("puzzle 2d force graph layout", () => {
    it("spreads linked nodes using wasm+nalgebra layout", () => {
      const fixture: BoardFixtureV1 = {
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [{ id: "e1", source: "a:h0", target: "b:h0" }],
        nodes: [
          { handles: [{ angle: 0, id: "a:h0" }], id: "a", radius: 40, shape: "circle", x: 0, y: 0 },
          { handles: [{ angle: Math.PI, id: "b:h0" }], id: "b", radius: 40, shape: "circle", x: 2, y: 0 },
        ],
        schema: "puzzle.2d.fixture/v1",
      };
      const laid = layoutBoardFixtureForceGraph(fixture, { gravity: 0, iterations: 220, idealEdgeLength: 200, randomSeed: 11 });
      const ax = (laid.nodes[0] as { x: number }).x;
      const bx = (laid.nodes[1] as { x: number }).x;
      expect(Math.abs(bx - ax)).toBeGreaterThan(90);
      expect(laid.schema).toBe("puzzle.2d.fixture/v1");
    });

    it("throws on invalid fixture schema from wasm", () => {
      const bad = { camera: { x: 0, y: 0, zoom: 1 }, edges: [], nodes: [], schema: "wrong" } as unknown as BoardFixtureV1;
      expect(() => layoutBoardFixtureForceGraph(bad)).toThrow();
    });
  });

  describe("board directed graph observation", () => {
    it("computes subtree from roots along directed edges", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const root = new BoardSceneNode({ id: "root", radius: 10, root: true, x: 0, y: 0 });
      const mid = new BoardSceneNode({ id: "mid", radius: 10, x: 50, y: 0 });
      const leaf = new BoardSceneNode({ id: "leaf", radius: 10, x: 100, y: 0 });
      const hRoot = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "root:h0", node: root });
      const hMidTarget = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "mid:h0", node: mid });
      const hMidSource = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "mid:h1", node: mid });
      const hLeafTarget = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "leaf:h0", node: leaf });
      const e1 = new BoardSceneEdge({ source: hRoot, id: "e1", target: hMidTarget });
      const e2 = new BoardSceneEdge({ source: hMidSource, id: "e2", target: hLeafTarget });
      renderer.scene.add(root).add(mid).add(leaf).add(e1).add(e2);
      const snap = computeBoardGraphObservationSnapshot(renderer.scene);
      expect(snap.rootIds).toEqual(["root"]);
      expect(snap.childNodeIds).toEqual(["leaf", "mid"]);
      expect(snap.childEdgeIds).toEqual(["e1", "e2"]);
      expect(snap.parentEdgeIds).toEqual(["e1"]);
      renderer.dispose();
    });

    it("emits graph observation events after scene mutations flush", async () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const events: string[] = [];
      const unsubs = [
        renderer.on("change", () => events.push("change")),
        renderer.on("nodeChange", (p) => events.push(`nodeChange:${p.id}`)),
        renderer.on("parentNodeChange", (p) => events.push(`parentNodeChange:${p.id}`)),
        renderer.on("parentEdgeChange", (p) => events.push(`parentEdgeChange:${p.id}`)),
        renderer.on("childNodesChange", (p) => events.push(`childNodesChange:${p.nodeIds.join(",")}`)),
        renderer.on("childEdgesChange", (p) => events.push(`childEdgesChange:${p.edgeIds.join(",")}`)),
        renderer.on("childNodeChange", (p) => events.push(`childNodeChange:${p.id}`)),
        renderer.on("childEdgeChange", (p) => events.push(`childEdgeChange:${p.id}`)),
      ];
      const root = new BoardSceneNode({ id: "root", radius: 10, root: true, x: 0, y: 0 });
      const child = new BoardSceneNode({ id: "child", radius: 10, x: 40, y: 0 });
      const hRootSource = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "root:h0", node: root });
      const hChildTarget = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "child:h0", node: child });
      const edge = new BoardSceneEdge({ source: hRootSource, id: "link", target: hChildTarget });
      renderer.scene.add(root).add(child).add(edge);
      await Promise.resolve();
      expect(events.some((e) => e.startsWith("parentNodeChange:root"))).toBe(true);
      expect(events.some((e) => e === "childNodesChange:child")).toBe(true);
      expect(events.some((e) => e === "childEdgesChange:link")).toBe(true);
      expect(events.some((e) => e === "change")).toBe(true);
      for (const u of unsubs) {
        u();
      }
      renderer.dispose();
    });

    it("emits nodeCreate when a node id first appears after the baseline observation", async () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const created: string[] = [];
      const off = renderer.on("nodeCreate", (p) => created.push(p.id));
      const root = new BoardSceneNode({ id: "root", radius: 10, root: true, x: 0, y: 0 });
      renderer.scene.add(root);
      await Promise.resolve();
      const child = new BoardSceneNode({ id: "child", radius: 10, x: 50, y: 0 });
      renderer.scene.add(child);
      await Promise.resolve();
      expect(created).toEqual(["root", "child"]);
      off();
      renderer.dispose();
    });

    it("emits edgeChange when an existing edge signature changes", async () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const changes: string[] = [];
      const off = renderer.on("edgeChange", (p) => changes.push(p.id));
      const root = new BoardSceneNode({ id: "root", radius: 10, root: true, x: 0, y: 0 });
      const child = new BoardSceneNode({ id: "child", radius: 10, x: 40, y: 0 });
      const hRootSource = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "root:h0", node: root });
      const hChildTarget = new BoardSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "child:h0", node: child });
      const edge = new BoardSceneEdge({ source: hRootSource, id: "link", target: hChildTarget });
      renderer.scene.add(root).add(child).add(edge);
      await Promise.resolve();
      edge.visible = false;
      renderer.markDirty();
      await Promise.resolve();
      expect(changes).toEqual(["link"]);
      off();
      renderer.dispose();
    });
  });
}
//#endregion 🔖Vitest

let boardSchedulerPriority = NoEventPriority;

/** @emoji 🧩 Static host surface required by the secondary renderer beyond board scene mutations. */
export const BOARD_HOST_MOUNT_DEFAULTS: Record<string, unknown> = {
  HostTransitionContext: reactHostPort.createContext(null) as never,
  NotPendingTransition: null,
  acquireResource: () => null,
  acquireSingletonInstance: () => null,
  appendChildToContainerChildSet: () => {},
  bindToConsole: () => () => undefined,
  canHydrateActivityInstance: () => false,
  canHydrateFormStateMarker: () => false,
  canHydrateInstance: () => false,
  canHydrateSuspenseInstance: () => false,
  canHydrateTextInstance: () => false,
  clearSuspenseBoundary: () => {},
  cloneHiddenInstance: () => {
    throw new Error("Board host: cloneHiddenInstance unsupported");
  },
  cloneHiddenTextInstance: () => {
    throw new Error("Board host: cloneHiddenTextInstance unsupported");
  },
  cloneInstance: () => {
    throw new Error("Board host: cloneInstance unsupported");
  },
  commitHydratedActivityInstance: () => null,
  commitHydratedContainer: () => null,
  commitHydratedInstance: () => null,
  commitHydratedSuspenseInstance: () => null,
  commitTextUpdate: () => {},
  createContainerChildSet: () => ({}),
  createHoistableInstance: () => null,
  diffHydratedPropsForDevWarnings: () => {},
  diffHydratedTextForDevWarnings: () => null,
  describeHydratableInstanceForDevWarnings: () => {},
  extraDevToolsConfig: {},
  finalizeContainerChildren: () => {},
  finalizeHydratedChildren: () => null,
  findFiberRoot: () => null,
  flushHydrationEvents: () => null,
  getBoundingRect: () => null,
  getFirstHydratableChild: () => null,
  getFirstHydratableChildWithinActivityInstance: () => null,
  getFirstHydratableChildWithinContainer: () => null,
  getFirstHydratableChildWithinSingleton: () => null,
  getFirstHydratableChildWithinSuspenseInstance: () => null,
  getHoistableRoot: () => null,
  getNextHydratableInstanceAfterActivityInstance: () => null,
  getNextHydratableInstanceAfterSuspenseInstance: () => null,
  getNextHydratableSibling: () => null,
  getNextHydratableSiblingAfterSingleton: () => null,
  getResource: () => null,
  getSuspendedCommitReason: () => null,
  getSuspenseInstanceFallbackErrorDetails: () => null,
  getTextContent: () => null,
  hideDehydratedBoundary: () => null,
  hideInstance: () => {},
  hideTextInstance: () => {},
  hydrateActivityInstance: () => null,
  hydrateHoistable: () => null,
  hydrateInstance: () => null,
  hydrateSuspenseInstance: () => null,
  hydrateTextInstance: () => null,
  isFormStateMarkerMatching: () => false,
  isHiddenSubtree: () => false,
  isHostHoistableType: () => false,
  isHostSingletonType: () => false,
  isSingletonScope: () => false,
  isSuspenseInstanceFallback: () => false,
  isSuspenseInstancePending: () => false,
  matchAccessibilityRole: () => false,
  mayResourceSuspendCommit: () => false,
  maySuspendCommit: () => false,
  maySuspendCommitInSyncRender: () => false,
  maySuspendCommitOnUpdate: () => false,
  mountHoistable: () => null,
  preloadInstance: () => true,
  preloadResource: () => false,
  prepareToCommitHoistables: () => null,
  registerSuspenseInstanceRetry: () => {},
  releaseResource: () => null,
  releaseSingletonInstance: () => null,
  rendererPackageName: "@puzzle/2d/react",
  rendererVersion: "0.1.0",
  replaceContainerChildren: () => {},
  resetFormInstance: () => {},
  resetTextContent: () => {},
  resolveEventTimeStamp: () => -1.1,
  resolveEventType: () => null,
  resolveSingletonInstance: () => null,
  setCurrentUpdatePriority(p: number) {
    boardSchedulerPriority = p;
  },
  getCurrentUpdatePriority() {
    return boardSchedulerPriority;
  },
  resolveUpdatePriority() {
    if (boardSchedulerPriority !== NoEventPriority) {
      return boardSchedulerPriority;
    }
    const w = globalThis as typeof globalThis & { event?: Event };
    const t = w.event?.type;
    if (t === "click" || t === "contextmenu" || t === "dblclick" || t === "pointercancel" || t === "pointerdown" || t === "pointerup") {
      return DiscreteEventPriority;
    }
    if (t === "pointermove" || t === "pointerout" || t === "pointerover" || t === "pointerenter" || t === "pointerleave" || t === "wheel") {
      return ContinuousEventPriority;
    }
    return DefaultEventPriority;
  },
  setFocusIfFocusable: () => false,
  setupIntersectionObserver: () => () => undefined,
  shouldAttemptEagerTransition: () => false,
  shouldDeleteUnhydratedTailInstances: () => false,
  startSuspendingCommit: () => null,
  supportsResources: false,
  supportsSingletons: false,
  supportsTestSelectors: false,
  suspendInstance: () => {},
  suspendResource: () => false,
  trackSchedulerEvent: () => {},
  unhideDehydratedBoundary: () => null,
  unhideInstance: () => {},
  unhideTextInstance: () => {},
  unmountHoistable: () => null,
  validateHydratableInstance: () => {},
  validateHydratableTextInstance: () => {},
  waitForCommitToBeReady: () => null,
  clearSuspenseBoundaryFromContainer: () => {},
};

function reportBoardHostUncaughtError(error: unknown, errorInfo: { componentStack?: string | null }): void {
  const report = (globalThis as typeof globalThis & { reportError?: (e: unknown) => void }).reportError;
  if (report) {
    report(error);
    return;
  }
  console.error(error, errorInfo.componentStack ?? "");
}

function reportBoardHostCaughtError(error: unknown, errorInfo: { componentStack?: string | null; errorBoundary?: unknown }): void {
  const report = (globalThis as typeof globalThis & { reportError?: (e: unknown) => void }).reportError;
  if (report) {
    report(error);
    return;
  }
  console.error(error, errorInfo.componentStack ?? "", errorInfo.errorBoundary ?? "");
}

function reportBoardHostRecoverableError(error: unknown): void {
  const report = (globalThis as typeof globalThis & { reportError?: (e: unknown) => void }).reportError;
  if (report) {
    report(error);
    return;
  }
  console.error(error);
}

//#region 🔖HostKinds
export const BOARD_HOST_NODE = "puzzle.2d/node";
export const BOARD_HOST_HANDLE = "puzzle.2d/handle";
export const BOARD_HOST_EDGE = "puzzle.2d/edge";
export const BOARD_HOST_WIRE = "puzzle.2d/wire";

export type BoardHostType = typeof BOARD_HOST_NODE | typeof BOARD_HOST_HANDLE | typeof BOARD_HOST_EDGE | typeof BOARD_HOST_WIRE;

interface BoardHostNode {
  kind: "node";
  impl: BoardSceneNode;
  renderer: BoardRenderer;
  readonly handleChildren: Set<BoardHostHandle>;
}

interface BoardHostHandle {
  kind: "handle";
  impl: BoardSceneHandle | null;
  props: BoardHandleProps;
  renderer: BoardRenderer;
}

interface BoardHostEdge {
  kind: "edge";
  impl: BoardSceneEdge | null;
  props: BoardEdgeProps;
  renderer: BoardRenderer;
}

interface BoardHostWire {
  kind: "wire";
  impl: BoardSceneWire | null;
  props: BoardWireProps;
  renderer: BoardRenderer;
}

export type BoardHostInstance = BoardHostNode | BoardHostHandle | BoardHostEdge | BoardHostWire;
//#endregion 🔖HostKinds

//#region 🔖PropApply
function newBoardNodeFromProps(props: BoardSceneNodeOptions): BoardSceneNode {
  if (props.shape === "rectangle") {
    return new BoardSceneNode({
      draggable: props.draggable ?? true,
      height: props.height,
      iconKind: props.iconKind,
      id: props.id,
      root: props.root,
      shape: "rectangle",
      style: props.style,
      text: props.text,
      textAlignment: props.textAlignment,
      textAutofit: props.textAutofit,
      textFontFamily: props.textFontFamily,
      textFontSize: props.textFontSize,
      userData: props.userData,
      visible: props.visible,
      width: props.width,
      x: props.x,
      y: props.y,
    });
  }
  return new BoardSceneNode({
    draggable: props.draggable ?? true,
    iconKind: props.iconKind,
    id: props.id,
    radius: props.radius,
    root: props.root,
    style: props.style,
    text: props.text,
    textAlignment: props.textAlignment,
    textAutofit: props.textAutofit,
    textFontFamily: props.textFontFamily,
    textFontSize: props.textFontSize,
    userData: props.userData,
    visible: props.visible,
    x: props.x,
    y: props.y,
  });
}

function applyNodeProps(renderer: BoardRenderer, instance: BoardSceneNode, props: BoardSceneNodeOptions): void {
  instance.draggable = props.draggable ?? true;
  instance.style = props.style ?? null;
  instance.userData = { ...(props.userData ?? {}) };
  instance.visible = props.visible ?? true;
  instance.root = props.root === true;
  instance.textAutofit = props.textAutofit ?? false;
  instance.textAlignment = props.textAlignment ?? BOARD_NODE_TEXT_ALIGNMENT_DEFAULT;
  instance.textFontFamily = typeof props.textFontFamily === "string" && props.textFontFamily.trim() !== "" ? props.textFontFamily.trim() : BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT;
  const psz = props.textFontSize;
  instance.textFontSize = typeof psz === "number" && Number.isFinite(psz) && psz > 0 ? psz : BOARD_NODE_TEXT_FONT_PX_DEFAULT;
  instance.iconKind = typeof props.iconKind === "string" && props.iconKind.trim() !== "" ? props.iconKind.trim() : null;
  const nk = typeof props.nodeKind === "string" ? props.nodeKind.trim() : "";
  instance.nodeKind = nk;
  renderer.applyNodePositionFromProps(instance.id, props.x, props.y, instance);
  instance.setText(props.text ?? null);
  if (props.shape === "rectangle") {
    instance.setRectangleSize(props.width, props.height);
  } else {
    instance.setRadius(props.radius);
  }
}

function applyHandleProps(instance: BoardSceneHandle, props: BoardHandleProps, node: BoardSceneNode): void {
  if (instance.node !== node) {
    instance.node.detachHandle(instance);
    node.attachHandle(instance);
    instance.node = node;
  }
  instance.style = props.style ?? null;
  instance.userData = { ...(props.userData ?? {}) };
  instance.visible = props.visible ?? true;
  instance.radius = props.radius ?? 8;
  instance.handleKind = (props.handleKind ?? "").trim();
  instance.iconKind = typeof props.iconKind === "string" && props.iconKind.trim() !== "" ? props.iconKind.trim() : null;
  const rawC = props.color;
  const cs = rawC === undefined || rawC === null ? "" : String(rawC).trim();
  instance.color = cs !== "" ? cs : null;
  instance.setAngle(props.angle);
}

function applyEdgeProps(instance: BoardSceneEdge, props: BoardEdgeProps, sourceHandle: BoardSceneHandle, targetHandle: BoardSceneHandle): void {
  instance.style = props.style ?? null;
  instance.userData = { ...(props.userData ?? {}) };
  instance.visible = props.visible ?? true;
  instance.edgeKind = typeof props.edgeKind === "string" ? props.edgeKind.trim() : "";
  instance.setEndpoints(sourceHandle, targetHandle);
}

function applyWireProps(instance: BoardSceneWire, props: BoardWireProps, sourceHandle: BoardSceneHandle, targetHandle: BoardSceneHandle | null): void {
  instance.style = props.style ?? null;
  instance.userData = { ...(props.userData ?? {}) };
  instance.visible = props.visible ?? true;
  instance.wireKind = typeof props.wireKind === "string" ? props.wireKind.trim() : "";
  const tid = (props.target ?? "").trim();
  const nextTarget = tid !== "" ? targetHandle : null;
  const ex = props.endX;
  const ey = props.endY;
  const endOk = typeof ex === "number" && Number.isFinite(ex) && typeof ey === "number" && Number.isFinite(ey);
  if (nextTarget) {
    instance.setAnchors(sourceHandle, nextTarget, null);
  } else if (endOk) {
    instance.setAnchors(sourceHandle, null, { x: ex, y: ey });
  } else {
    instance.setAnchors(sourceHandle, null, null);
  }
}

function nodeShapeSyncKey(props: BoardSceneNodeOptions): "circle" | "rectangle" {
  return props.shape === "rectangle" ? "rectangle" : "circle";
}

function instanceShapeSyncKey(node: BoardSceneNode): "circle" | "rectangle" {
  return node.shape;
}

function propsEqualHandle(a: BoardHandleProps, b: BoardHandleProps): boolean {
  const ac = a.color === undefined || a.color === null ? "" : String(a.color).trim();
  const bc = b.color === undefined || b.color === null ? "" : String(b.color).trim();
  return (
    a.id === b.id &&
    a.angle === b.angle &&
    a.radius === b.radius &&
    (a.handleKind ?? "").trim() === (b.handleKind ?? "").trim() &&
    (a.iconKind ?? "") === (b.iconKind ?? "") &&
    ac === bc &&
    a.highlighted === b.highlighted &&
    a.selected === b.selected &&
    a.style === b.style &&
    a.visible === b.visible &&
    shallowEqualRecord(a.userData ?? {}, b.userData ?? {})
  );
}

function propsEqualEdge(a: BoardEdgeProps, b: BoardEdgeProps): boolean {
  return (
    a.id === b.id &&
    a.source === b.source &&
    a.target === b.target &&
    (a.edgeKind ?? "").trim() === (b.edgeKind ?? "").trim() &&
    a.highlighted === b.highlighted &&
    a.selected === b.selected &&
    a.style === b.style &&
    a.visible === b.visible &&
    shallowEqualRecord(a.userData ?? {}, b.userData ?? {})
  );
}

function propsEqualWire(a: BoardWireProps, b: BoardWireProps): boolean {
  return (
    a.id === b.id &&
    a.source === b.source &&
    (a.wireKind ?? "").trim() === (b.wireKind ?? "").trim() &&
    (a.target ?? "") === (b.target ?? "") &&
    (a.endX ?? Number.NaN) === (b.endX ?? Number.NaN) &&
    (a.endY ?? Number.NaN) === (b.endY ?? Number.NaN) &&
    a.highlighted === b.highlighted &&
    a.selected === b.selected &&
    a.style === b.style &&
    a.visible === b.visible &&
    shallowEqualRecord(a.userData ?? {}, b.userData ?? {})
  );
}

function propsEqualNode(a: BoardSceneNodeOptions, b: BoardSceneNodeOptions): boolean {
  if (
    a.id !== b.id ||
    a.x !== b.x ||
    a.y !== b.y ||
    a.draggable !== b.draggable ||
    a.highlighted !== b.highlighted ||
    a.selected !== b.selected ||
    a.style !== b.style ||
    a.visible !== b.visible ||
    a.text !== b.text ||
    (a.textAutofit ?? false) !== (b.textAutofit ?? false) ||
    (a.textAlignment ?? BOARD_NODE_TEXT_ALIGNMENT_DEFAULT) !== (b.textAlignment ?? BOARD_NODE_TEXT_ALIGNMENT_DEFAULT) ||
    (a.textFontFamily ?? BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT) !== (b.textFontFamily ?? BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT) ||
    (a.textFontSize ?? BOARD_NODE_TEXT_FONT_PX_DEFAULT) !== (b.textFontSize ?? BOARD_NODE_TEXT_FONT_PX_DEFAULT) ||
    (a.root === true) !== (b.root === true) ||
    (a.iconKind ?? "") !== (b.iconKind ?? "") ||
    (a.nodeKind ?? "").trim() !== (b.nodeKind ?? "").trim()
  ) {
    return false;
  }
  if (!shallowEqualRecord(a.userData ?? {}, b.userData ?? {})) {
    return false;
  }
  if (nodeShapeSyncKey(a) !== nodeShapeSyncKey(b)) {
    return false;
  }
  if (a.shape === "rectangle" && b.shape === "rectangle") {
    return a.width === b.width && a.height === b.height;
  }
  return (a as { radius: number }).radius === (b as { radius: number }).radius;
}
//#endregion 🔖PropApply

//#region 🔖MountHelpers
function mountHandleUnderNode(renderer: BoardRenderer, nodeHost: BoardHostNode, handleHost: BoardHostHandle): void {
  if (handleHost.impl?.parent) {
    return;
  }
  nodeHost.handleChildren.add(handleHost);
  const impl = new BoardSceneHandle({ ...handleHost.props, node: nodeHost.impl });
  handleHost.impl = impl;
  renderer.batch(() => {
    renderer.scene.add(impl);
  });
  renderer.invalidate();
}

function mountNode(renderer: BoardRenderer, nodeHost: BoardHostNode): void {
  if (nodeHost.impl.parent) {
    return;
  }
  renderer.batch(() => {
    renderer.scene.add(nodeHost.impl);
  });
  renderer.invalidate();
}

function mountEdge(renderer: BoardRenderer, edgeHost: BoardHostEdge): void {
  if (edgeHost.impl?.parent) {
    return;
  }
  const source = renderer.scene.getObjectById(edgeHost.props.source);
  const target = renderer.scene.getObjectById(edgeHost.props.target);
  if (!(source instanceof BoardSceneHandle) || !(target instanceof BoardSceneHandle)) {
    return;
  }
  renderer.batch(() => {
    if (!edgeHost.impl) {
      edgeHost.impl = new BoardSceneEdge({ ...edgeHost.props, source, target });
      renderer.scene.add(edgeHost.impl);
    } else {
      applyEdgeProps(edgeHost.impl, edgeHost.props, source, target);
    }
  });
  renderer.invalidate();
}

function mountWire(renderer: BoardRenderer, wireHost: BoardHostWire): void {
  if (wireHost.impl?.parent) {
    return;
  }
  const source = renderer.scene.getObjectById(wireHost.props.source);
  if (!(source instanceof BoardSceneHandle)) {
    return;
  }
  const tid = (wireHost.props.target ?? "").trim();
  let target: BoardSceneHandle | null = null;
  if (tid !== "") {
    const t = renderer.scene.getObjectById(tid);
    if (!(t instanceof BoardSceneHandle)) {
      return;
    }
    target = t;
  }
  renderer.batch(() => {
    if (!wireHost.impl) {
      const ex = wireHost.props.endX;
      const ey = wireHost.props.endY;
      wireHost.impl = new BoardSceneWire({
        id: wireHost.props.id,
        source,
        target,
        selected: wireHost.props.selected,
        style: wireHost.props.style,
        userData: wireHost.props.userData,
        visible: wireHost.props.visible,
        endX: typeof ex === "number" && Number.isFinite(ex) ? ex : null,
        endY: typeof ey === "number" && Number.isFinite(ey) ? ey : null,
      });
      renderer.scene.add(wireHost.impl);
    } else {
      applyWireProps(wireHost.impl, wireHost.props, source, target);
    }
  });
  renderer.invalidate();
}

function replaceNodeImpl(renderer: BoardRenderer, host: BoardHostNode, nextProps: BoardSceneNodeOptions): void {
  if (instanceShapeSyncKey(host.impl) !== nodeShapeSyncKey(nextProps)) {
    renderer.batch(() => {
      for (const handleHost of host.handleChildren) {
        if (handleHost.impl?.parent) {
          renderer.scene.remove(handleHost.impl);
        }
        handleHost.impl = null;
      }
      renderer.scene.remove(host.impl);
      host.impl = newBoardNodeFromProps(nextProps);
      renderer.scene.add(host.impl);
      for (const handleHost of host.handleChildren) {
        mountHandleUnderNode(renderer, host, handleHost);
      }
    });
    renderer.invalidate();
    return;
  }
  renderer.batch(() => {
    applyNodeProps(renderer, host.impl, nextProps);
  });
  renderer.invalidate();
}

function isBoardRenderer(value: unknown): value is BoardRenderer {
  return value instanceof BoardRenderer;
}

function appendToBoardParent(parent: BoardRenderer | BoardHostInstance, child: BoardHostInstance): void {
  const renderer = child.renderer;
  if (isBoardRenderer(parent)) {
    if (child.kind === "node") {
      mountNode(renderer, child);
    } else if (child.kind === "edge") {
      mountEdge(renderer, child);
    } else if (child.kind === "wire") {
      mountWire(renderer, child);
    }
    return;
  }
  if (parent.kind === "node" && child.kind === "handle") {
    mountHandleUnderNode(renderer, parent, child);
  }
}

function detachHandleFromNode(nodeHost: BoardHostNode, handleHost: BoardHostHandle): void {
  nodeHost.handleChildren.delete(handleHost);
}

const boardEmptyHostContext = Object.freeze({});
//#endregion 🔖MountHelpers

//#region 🔖HostMountInternals
const boardSceneHost = Reconciler({
  ...BOARD_HOST_MOUNT_DEFAULTS,
  supportsMutation: true,
  supportsPersistence: false,
  supportsHydration: false,
  isPrimaryRenderer: false,
  warnsIfNotActing: true,
  supportsMicrotasks: true,
  /** @emoji ⚡ Runs microtasks synchronously so `updateContainer` from a parent `useLayoutEffect` commits before `act()` reads the imperative scene (nested host roots + cross-root context bridge). */
  scheduleMicrotask: (fn: () => unknown) => {
    fn();
  },
  noTimeout: -1,
  scheduleTimeout: setTimeout,
  cancelTimeout: clearTimeout,

  getRootHostContext: () => boardEmptyHostContext,
  getChildHostContext: () => boardEmptyHostContext,

  createInstance(type, props, rootContainer) {
    const renderer = rootContainer;
    if (type === BOARD_HOST_NODE) {
      return { kind: "node", handleChildren: new Set(), impl: newBoardNodeFromProps(props as NodeOptions), renderer };
    }
    if (type === BOARD_HOST_HANDLE) {
      return { kind: "handle", impl: null, props: props as BoardHandleProps, renderer };
    }
    if (type === BOARD_HOST_EDGE) {
      return { kind: "edge", impl: null, props: props as BoardEdgeProps, renderer };
    }
    if (type === BOARD_HOST_WIRE) {
      return { kind: "wire", impl: null, props: props as BoardWireProps, renderer };
    }
    throw new Error(`Unknown board host type: ${String(type)}`);
  },

  createTextInstance() {
    throw new Error("Text children are not supported inside the board host tree.");
  },

  shouldSetTextContent: () => false,

  appendInitialChild(parent, child) {
    appendToBoardParent(parent as BoardRenderer | BoardHostInstance, child);
  },

  appendChild(parent, child) {
    appendToBoardParent(parent as BoardRenderer | BoardHostInstance, child);
  },

  appendChildToContainer(container, child) {
    if (child.kind === "node") {
      mountNode(container, child);
    } else if (child.kind === "edge") {
      mountEdge(container, child);
    } else if (child.kind === "wire") {
      mountWire(container, child);
    }
  },

  insertBefore(parent, child, _beforeChild) {
    appendToBoardParent(parent as BoardRenderer | BoardHostInstance, child);
  },

  insertInContainerBefore(container, child, _beforeChild) {
    if (child.kind === "node") {
      mountNode(container, child);
    } else if (child.kind === "edge") {
      mountEdge(container, child);
    } else if (child.kind === "wire") {
      mountWire(container, child);
    }
  },

  removeChild(parent, child) {
    const renderer = child.renderer;
    if (!isBoardRenderer(parent) && parent.kind === "node" && child.kind === "handle") {
      detachHandleFromNode(parent, child);
    }
    if (child.impl?.parent) {
      renderer.scene.remove(child.impl);
    }
    if (child.kind === "handle" || child.kind === "edge" || child.kind === "wire") {
      child.impl = null;
    }
    renderer.invalidate();
  },

  removeChildFromContainer(container, child) {
    if (child.kind === "node") {
      const nh = child as BoardHostNode;
      for (const h of [...nh.handleChildren]) {
        detachHandleFromNode(nh, h);
        if (h.impl?.parent) {
          container.scene.remove(h.impl);
        }
        h.impl = null;
      }
      nh.handleChildren.clear();
      if (nh.impl.parent) {
        container.scene.remove(nh.impl);
      }
      container.invalidate();
      return;
    }
    if (child.impl?.parent) {
      container.scene.remove(child.impl);
    }
    if (child.kind === "handle" || child.kind === "edge" || child.kind === "wire") {
      child.impl = null;
    }
    container.invalidate();
  },

  /** @emoji 🧹 No-op: host stack calls this on root before mutation; scene graph is driven by append/remove only (see {@link unmountBoardHostMount}). */
  clearContainer(_container: BoardRenderer) {},

  finalizeInitialChildren() {
    return false;
  },

  getPublicInstance(instance) {
    return instance;
  },

  prepareForCommit() {
    return null;
  },
  resetAfterCommit() {},
  preparePortalMount() {},

  prepareUpdate(instance, type, oldProps, newProps) {
    if (type === BOARD_HOST_NODE) {
      return !propsEqualNode(oldProps as NodeOptions, newProps as NodeOptions);
    }
    if (type === BOARD_HOST_HANDLE) {
      return !propsEqualHandle(oldProps as BoardHandleProps, newProps as BoardHandleProps);
    }
    if (type === BOARD_HOST_EDGE) {
      return !propsEqualEdge(oldProps as BoardEdgeProps, newProps as BoardEdgeProps);
    }
    if (type === BOARD_HOST_WIRE) {
      return !propsEqualWire(oldProps as BoardWireProps, newProps as BoardWireProps);
    }
    return false;
  },

  commitUpdate(instance, _payload, type, oldProps, nextProps) {
    const renderer = instance.renderer;
    if (type === BOARD_HOST_NODE) {
      const next = nextProps as NodeOptions;
      const prev = oldProps as NodeOptions;
      const host = instance as BoardHostNode;
      if (instanceShapeSyncKey(host.impl) !== nodeShapeSyncKey(next)) {
        replaceNodeImpl(renderer, host, next);
        return;
      }
      if (prev.x !== next.x || prev.y !== next.y) {
        renderer.evictNodeAuthoringPosition(next.id);
      }
      renderer.batch(() => {
        applyNodeProps(renderer, host.impl, next);
      });
      renderer.invalidate();
      return;
    }
    if (type === BOARD_HOST_HANDLE) {
      const h = instance as BoardHostHandle;
      h.props = nextProps as BoardHandleProps;
      if (!h.impl) {
        return;
      }
      const parentNode = h.impl.node;
      renderer.batch(() => {
        applyHandleProps(h.impl!, h.props, parentNode);
      });
      renderer.invalidate();
      return;
    }
    if (type === BOARD_HOST_EDGE) {
      const e = instance as BoardHostEdge;
      e.props = nextProps as BoardEdgeProps;
      const from = renderer.scene.getObjectById(e.props.source);
      const to = renderer.scene.getObjectById(e.props.target);
      if (!(from instanceof BoardSceneHandle) || !(to instanceof BoardSceneHandle)) {
        return;
      }
      renderer.batch(() => {
        if (!e.impl) {
          e.impl = new BoardSceneEdge({ ...e.props, source: from, target: to });
          renderer.scene.add(e.impl);
        } else {
          applyEdgeProps(e.impl, e.props, from, to);
        }
      });
      renderer.invalidate();
      return;
    }
    if (type === BOARD_HOST_WIRE) {
      const w = instance as BoardHostWire;
      w.props = nextProps as BoardWireProps;
      const from = renderer.scene.getObjectById(w.props.source);
      if (!(from instanceof BoardSceneHandle)) {
        return;
      }
      const tid = (w.props.target ?? "").trim();
      let to: BoardSceneHandle | null = null;
      if (tid !== "") {
        const t = renderer.scene.getObjectById(tid);
        if (!(t instanceof BoardSceneHandle)) {
          return;
        }
        to = t;
      }
      renderer.batch(() => {
        if (!w.impl) {
          const ex = w.props.endX;
          const ey = w.props.endY;
          w.impl = new BoardSceneWire({
            id: w.props.id,
            source: from,
            target: to,
            selected: w.props.selected,
            style: w.props.style,
            userData: w.props.userData,
            visible: w.props.visible,
            endX: typeof ex === "number" && Number.isFinite(ex) ? ex : null,
            endY: typeof ey === "number" && Number.isFinite(ey) ? ey : null,
          });
          renderer.scene.add(w.impl);
        } else {
          applyWireProps(w.impl, w.props, from, to);
        }
      });
      renderer.invalidate();
    }
  },

  commitMount() {},

  detachDeletedInstance() {},

  getInstanceFromNode: () => null,
  beforeActiveInstanceBlur() {},
  afterActiveInstanceBlur() {},
  prepareScopeUpdate() {},
  getInstanceFromScope: () => null,

  getCurrentEventPriority: () => DefaultEventPriority,
  requestPaint() {},
} as never);

export type BoardHostMount = ReturnType<typeof boardSceneHost.createContainer>;

/** @emoji 🌱 Creates a legacy-mode host mount bound to {@link BoardRenderer} for synchronous subtree commits with DOM `act()`. */
export function createBoardHostMount(renderer: BoardRenderer): BoardHostMount {
  return boardSceneHost.createContainer(
    renderer,
    LegacyRoot,
    null,
    false,
    null,
    "board:",
    reportBoardHostUncaughtError,
    reportBoardHostCaughtError,
    reportBoardHostRecoverableError,
    undefined,
  );
}

/** @emoji 🔄 Schedules host work and ties post-commit to {@link BoardRenderer.invalidate}. */
export function updateBoardHostMount(root: BoardHostMount, element: ReactElement | null, parent: null): void {
  boardSceneHost.updateContainer(element, root, parent, () => {
    const renderer = root.containerInfo;
    renderer.invalidate();
  });
}

/** @emoji 🧹 Unmounts the host subtree without disposing {@link BoardRenderer}. */
export function unmountBoardHostMount(root: BoardHostMount): void {
  updateBoardHostMount(root, null, null);
  const renderer = root.containerInfo as BoardRenderer;
  renderer.runWithoutSceneDeleteEvents(() => {
    renderer.scene.clear();
  });
  renderer.invalidate();
}

export { boardSceneHost };
//#endregion 🔖HostMountInternals

// #region 🎨ReactCanvas
import { FiberProvider as HostMountProvider, useContextBridge as useHostMountBridge } from "its-fine";
import { Children, Fragment, act, createElement, isValidElement, type CSSProperties, type DragEvent, type ReactElement, type ReactNode } from "react";
import { createRoot } from "react-dom/client";

//#region 🔖Kinds
export interface BoardCanvasProps {
  camera?: Partial<CameraState>;
  children?: ReactNode;
  /** @emoji 🎧 DOM-tree descendants with {@link BoardContext} (e.g. {@link useBoardEvent}); not mounted in the board host reconciler. */
  companions?: ReactNode;
  className?: string;
  contextMenu?: ContextMenuItem[];
  /** @emoji 📥 When true, accepts in-app fixture drags using {@link BOARD_FIXTURE_DRAG_V1_MIME} (not OS file drops). */
  fixtureDragDrop?: boolean;
  height?: number;
  /** @emoji 🔗 Allowed kind pairs for link gestures (`specificity` tiers + `important`); empty omits filtering. */
  kindCompatibility?: readonly KindCompatEntry[];
  /** @emoji 🧩 Central semantic kind catalogs (handles, wires, nodes, edges) for WASM defaults + compatibility. */
  kindCatalogs?: KindCatalogBundle;
  onFixtureDrop?: (detail: BoardFixtureDropDetail) => void;
  /** @emoji 🖱️ Fires after pointer-driven hit tests (same cadence as canvas moves); use for tooltips and status. */
  onHover?: (payload: BoardHoverPayload) => void;
  onReady?: (renderer: BoardRenderer) => void;
  /** @emoji 🔔 Fires after any graph observation emission in this flush (see other `on*` graph props). */
  onChange?: () => void;
  /** @emoji 📶 LOD zoom bands for WASM draw + overlay captions (`data-board-lod`). */
  lodZoomThresholds?: BoardLodZoomThresholds;
  /** @emoji 📶 When true (default), camera zoom selects draw LOD; when false, {@link lod} pins the tier. */
  automaticLod?: boolean;
  /** @emoji 📶 Pinned draw LOD when `automaticLod` is false. */
  lod?: BoardDrawLodKind;
  /** @emoji 📶 Emits whenever the resolved WASM draw LOD band changes. */
  onLodChange?: (lod: BoardDrawLodKind) => void;
  /** @emoji 📐 Positive multiplier for LOD world grid steps on the WASM host (default {@link DEFAULT_BOARD_GRID_FACTOR}). */
  gridFactor?: number;
  /** @emoji 🧲 When true, node drags snap to the finest visible LOD grid on the WASM host. */
  gridSnapEnabled?: boolean;
  onChildEdgeChange?: (payload: BoardGraphEdgeIdPayload) => void;
  onChildEdgesChange?: (payload: BoardChildEdgesChangePayload) => void;
  onChildNodeChange?: (payload: BoardGraphNodeIdPayload) => void;
  onChildNodesChange?: (payload: BoardChildNodesChangePayload) => void;
  onNodeChange?: (payload: BoardGraphNodeIdPayload) => void;
  onParentEdgeChange?: (payload: BoardGraphEdgeIdPayload) => void;
  onParentNodeChange?: (payload: BoardGraphNodeIdPayload) => void;
  /** @emoji 🎥 Camera pan/zoom center in world space plus zoom factor (same payload as {@link BoardCanvasProps.onViewportChange}). */
  onCamera?: (state: CameraState) => void;
  /** @emoji 🖱️ Right-click surface hit before built-in context UI resolves menu items. */
  onContextMenu?: (payload: BoardEventMap["contextmenu"]) => void;
  /** @emoji 🪢 Direct handle-to-handle link commit ({@link BoardEventMap.edgeCreate}). */
  onConnect?: (payload: BoardEdgeLinkPayload) => void;
  /** @emoji 📦 Fires once for {@link BoardEventMap.nodeCreate}, {@link BoardEventMap.edgeCreate}, or {@link BoardEventMap.wireCreate}. */
  onCreate?: (payload: BoardStructureCreatePayload) => void;
  /** @emoji 📦 Fires once for {@link BoardEventMap.nodeDelete}, {@link BoardEventMap.edgeDelete}, or {@link BoardEventMap.wireDestroy}. */
  onDelete?: (payload: BoardStructureDeletePayload) => void;
  /** @emoji 🖱️ Node drag motion from WASM (`nodeMove`). */
  onDrag?: (payload: BoardEventMap["nodeMove"]) => void;
  onEdgeChange?: (payload: BoardGraphEdgeIdPayload) => void;
  onEdgeCreate?: (payload: BoardEdgeLinkPayload) => void;
  onEdgeDelete?: (payload: { id: string }) => void;
  /** @emoji 🧭 Second click on an indirect handle ring target after {@link BoardEventMap.edgeCreate}. */
  onIndirectConnect?: (payload: BoardEdgeLinkPayload) => void;
  /** @emoji ♻️ GPU/text invalidation tick (coalesced `invalidate`). */
  onInvalidate?: () => void;
  onNodeCreate?: (payload: BoardGraphNodeIdPayload) => void;
  onNodeDelete?: (payload: { id: string }) => void;
  /** @emoji 🧲 Snap commit on pointer-up after a link drag (`proximityConnect` after `edgeCreate`). */
  onProximityConnect?: (payload: BoardEdgeLinkPayload) => void;
  /** @emoji 🎯 Emits while a link drag highlights compatible target parts ({@link BoardEventMap.linkCompatibleNodes}). */
  onLinkCompatibleNodes?: (payload: BoardLinkCompatibleNodesPayload) => void;
  /** @emoji ⭕ Emits while a link drag shows an indirect anchor ring ({@link BoardEventMap.linkTargetRing}). */
  onLinkTargetRing?: (payload: BoardLinkTargetRingPayload) => void;
  /** @emoji 🔗 Host-driven link preview for cross-surface gestures (cleared when `source` is empty). */
  linkSession?: BoardLinkSessionSnapshot | null;
  onSelect?: (snapshot: BoardSelectionSnapshot) => void;
  /** @emoji ✅ Controlled committed selection (`onSelect` should update this). */
  selection?: BoardSelectionSnapshot | readonly string[];
  /** @emoji ✅ Uncontrolled initial committed selection. */
  defaultSelection?: BoardSelectionSnapshot | readonly string[];
  /** @emoji 👁️ Controlled area-select preview (`onPreselect` should update this). */
  preselection?: BoardPreselectSnapshot;
  /** @emoji 👁️ Uncontrolled initial area-select preview. */
  defaultPreselection?: BoardPreselectSnapshot;
  onPreselect?: (snapshot: BoardPreselectSnapshot) => void;
  /** @emoji 🖱️ Controlled hover target id (`onHover` should update this). */
  hoveredId?: string | null;
  /** @emoji 🖱️ Uncontrolled initial hover target id. */
  defaultHoveredId?: string | null;
  onWireChange?: (payload: BoardGraphWireIdPayload) => void;
  onWireCreate?: (payload: BoardWireSnapshotPayload) => void;
  onWireDestroy?: (payload: BoardGraphWireIdPayload) => void;
  /** @emoji ↔️ Camera center changed without zoom delta beyond float noise. */
  onPan?: (state: CameraState) => void;
  /** @emoji 🔎 Zoom factor changed on the camera snapshot. */
  onZoom?: (state: CameraState) => void;
  /** @emoji 🪟 Preferred alias for {@link BoardCanvasProps.onCamera} (viewport = camera snapshot). */
  onViewportChange?: (state: CameraState) => void;
  renderMode?: RenderMode;
  selectionMethod?: BoardSelectionMethod;
  selectionMode?: BoardSelectionMode;
  /** @emoji 🎯 Independent toggles for which kinds participate in marquee/lasso and hit picking. */
  selectionTargets?: BoardSelectionTargets;
  style?: CSSProperties;
  width?: number;
  /** 🧩 World-space clip tiling for Vello (`world-clip`, default) vs monolithic scene (`none`). */
  worldRasterTiling?: WorldRasterTilingKind;
}

export type BoardNodeCircleProps = {
  children?: ReactNode;
  contextMenu?: ContextMenuItem[];
  draggable?: boolean;
  id: string;
  radius: number;
  /** @emoji 🌳 Declares a directed subtree root (edges: parent {@link Handle} → child {@link Handle}). */
  root?: boolean;
  highlighted?: boolean;
  selected?: boolean;
  shape?: "circle";
  style?: string;
  text?: string;
  /** @emoji 🏷️ Runtime icon encoding (`typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG) for detail LOD vector paint. */
  iconKind?: string;
  /** @emoji 🧩 Semantic node-kind id for WASM compatibility and catalog defaults. */
  nodeKind?: string;
  /** @emoji 📏 When true, caption scales to fit inside the node on the text overlay canvas. */
  textAutofit?: boolean;
  /** @emoji 🧭 Caption alignment inside the node box when not autofitting. */
  textAlignment?: BoardNodeTextAlignment;
  /** @emoji 🔤 CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Caption size in layout px when not autofitting. */
  textFontSize?: number;
  userData?: Record<string, unknown>;
  visible?: boolean;
  x: number;
  y: number;
};

export type BoardNodeRectangleProps = {
  children?: ReactNode;
  contextMenu?: ContextMenuItem[];
  draggable?: boolean;
  height: number;
  id: string;
  /** @emoji 🌳 Declares a directed subtree root (edges: parent {@link Handle} → child {@link Handle}). */
  root?: boolean;
  highlighted?: boolean;
  selected?: boolean;
  shape: "rectangle";
  style?: string;
  text?: string;
  /** @emoji 🏷️ Runtime icon encoding (`typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG) for detail LOD vector paint. */
  iconKind?: string;
  /** @emoji 🧩 Semantic node-kind id for WASM compatibility and catalog defaults. */
  nodeKind?: string;
  /** @emoji 📏 When true, caption scales to fit inside the node on the text overlay canvas. */
  textAutofit?: boolean;
  /** @emoji 🧭 Caption alignment inside the node box when not autofitting. */
  textAlignment?: BoardNodeTextAlignment;
  /** @emoji 🔤 CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Caption size in layout px when not autofitting. */
  textFontSize?: number;
  userData?: Record<string, unknown>;
  visible?: boolean;
  width: number;
  x: number;
  y: number;
};

/** @emoji 🟠 Declarative node marker: {@link BoardNodeCircleProps} or {@link BoardNodeRectangleProps}. */
export type BoardNodeProps = BoardNodeCircleProps | BoardNodeRectangleProps;

export interface NodeDescriptor extends BoardNodeProps {
  handles: HandleDescriptor[];
}

export interface HandleDescriptor extends BoardHandleProps {
  nodeId: string;
}

export interface EdgeDescriptor extends BoardEdgeProps {}

export interface WireDescriptor extends BoardWireProps {}

interface BoardSceneDescriptor {
  edges: EdgeDescriptor[];
  handles: HandleDescriptor[];
  nodes: NodeDescriptor[];
  wires: WireDescriptor[];
}
//#endregion 🔖Kinds

//#region 🔖Context
const BoardContext = reactHostPort.createContext<BoardRenderer | null>(null);
let activeBoardRenderer: BoardRenderer | null = null;

//#endregion 🔖Context

//#region 🔖Markers
/** 🟠 Host intrinsic for the secondary board host; assign to JSX {@link BOARD_HOST_NODE}. */
export const Node = BOARD_HOST_NODE;

/** 🟣 Host intrinsic for board handles nested under {@link Node}. */
export const Handle = BOARD_HOST_HANDLE;

/** 🪢 Host intrinsic for directed edges between handle ids. */
export const Edge = BOARD_HOST_EDGE;

/** 🧵 Host intrinsic for transient wires from a handle to another handle or a free world end. */
export const Wire = BOARD_HOST_WIRE;

/** @emoji 🗼 Optional per-id context menus when building {@link boardFixtureSceneMarkers}. */
export interface BoardFixtureSceneMarkersOptions {
  edgeContextMenuForId?: (edgeId: string) => ContextMenuItem[] | undefined;
  nodeContextMenuForId?: (nodeId: string) => ContextMenuItem[] | undefined;
}

/** @emoji 🗼 Declarative {@link Node}/{@link Edge} tree for {@link BoardCanvas} `children` from {@link BoardFixtureV1} (Fragment of host markers only). */
export function boardFixtureSceneMarkers(fixture: BoardFixtureV1, options?: BoardFixtureSceneMarkersOptions): ReactElement {
  const nodeMenu = options?.nodeContextMenuForId;
  const edgeMenu = options?.edgeContextMenuForId;
  return (
    <>
      {fixture.nodes.map((node) =>
        node.shape === "rectangle" ? (
          <Node
            contextMenu={nodeMenu?.(node.id)}
            draggable
            height={node.height}
            id={node.id}
            key={node.id}
            {...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
            shape="rectangle"
            text={boardFixtureNodeCaption(node)}
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
              <Handle angle={handle.angle} color={handle.color} handleKind={handle.handleKind} id={handle.id} key={handle.id} radius={handle.radius} {...(handle.iconKind ? { iconKind: handle.iconKind } : {})} />
            ))}
          </Node>
        ) : (
          <Node
            contextMenu={nodeMenu?.(node.id)}
            draggable
            id={node.id}
            key={node.id}
            {...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
            radius={node.radius}
            text={boardFixtureNodeCaption(node)}
            textAlignment={node.textAlignment}
            textAutofit={node.textAutofit === true}
            textFontFamily={node.textFontFamily}
            textFontSize={node.textFontSize}
            x={node.x}
            y={node.y}
            {...(node.iconKind ? { iconKind: node.iconKind } : {})}
          >
            {node.handles.map((handle) => (
              <Handle angle={handle.angle} color={handle.color} handleKind={handle.handleKind} id={handle.id} key={handle.id} radius={handle.radius} {...(handle.iconKind ? { iconKind: handle.iconKind } : {})} />
            ))}
          </Node>
        ),
      )}
      {fixture.edges.map((edge) => (
        <Edge contextMenu={edgeMenu?.(edge.id)} id={edge.id} key={edge.id} source={edge.source} target={edge.target} />
      ))}
    </>
  );
}
//#endregion 🔖Markers

//#region 🔖Descriptor Build
function isMarkerElement(element: ReactElement): boolean {
  return element.type === BOARD_HOST_NODE || element.type === BOARD_HOST_HANDLE || element.type === BOARD_HOST_EDGE || element.type === BOARD_HOST_WIRE;
}

function appendHandleDescriptors(children: ReactNode, nodeId: string, handles: HandleDescriptor[]): void {
  Children.forEach(children, (child) => {
    if (!isValidElement(child)) {
      return;
    }
    if (child.type === Fragment) {
      appendHandleDescriptors((child as ReactElement<{ children?: ReactNode }>).props.children, nodeId, handles);
      return;
    }
    if (child.type === BOARD_HOST_HANDLE) {
      const props = child.props as BoardHandleProps;
      handles.push({ ...props, nodeId });
    }
  });
}

export function buildBoardSceneDescriptor(children: ReactNode): BoardSceneDescriptor {
  const descriptor: BoardSceneDescriptor = { edges: [], handles: [], nodes: [], wires: [] };

  const visit = (node: ReactNode): void => {
    Children.forEach(node, (child) => {
      if (!isValidElement(child)) {
        return;
      }
      if (child.type === Fragment) {
        visit((child as ReactElement<{ children?: ReactNode }>).props.children);
        return;
      }
      if (child.type === BOARD_HOST_NODE) {
        const props = child.props as BoardNodeProps;
        const handles: HandleDescriptor[] = [];
        appendHandleDescriptors(props.children, props.id, handles);
        descriptor.nodes.push({ ...props, handles });
        descriptor.handles.push(...handles);
        return;
      }
      if (child.type === BOARD_HOST_EDGE) {
        descriptor.edges.push(child.props as BoardEdgeProps);
        return;
      }
      if (child.type === BOARD_HOST_WIRE) {
        descriptor.wires.push(child.props as BoardWireProps);
        return;
      }
    });
  };

  visit(children);
  return descriptor;
}
//#endregion 🔖Descriptor Build

function requireRenderer(renderer: BoardRenderer | null): BoardRenderer {
  if (!renderer) {
    throw new Error("BoardCanvas did not publish its renderer.");
  }
  return renderer;
}

//#region 🔖Scene Sync
/** @emoji 🔗 Merges WASM‑created edges into the JSX descriptor until React children list the same edge id (then authorship is cleared via {@link BoardRenderer.clearWasmHostAuthorshipForEdge}). */
export function mergeWasmHostAuthoredEdgesIntoDescriptor(renderer: BoardRenderer, descriptor: BoardSceneDescriptor): BoardSceneDescriptor {
  const jsxEdgeIds = new Set(descriptor.edges.map((edge) => edge.id));
  for (const id of Array.from(renderer.wasmHostAuthoredEdgeIds)) {
    if (jsxEdgeIds.has(id)) {
      renderer.clearWasmHostAuthorshipForEdge(id);
    }
  }
  const extra: EdgeDescriptor[] = [];
  for (const id of Array.from(renderer.wasmHostAuthoredEdgeIds)) {
    const edge = renderer.scene.edges.get(id);
    if (edge) {
      extra.push({
        id: edge.id,
        source: edge.source.id,
        target: edge.target.id,
        edgeKind: edge.edgeKind || undefined,
        selected: edge.selected,
        style: edge.style ?? undefined,
        visible: edge.visible,
        userData: { ...edge.userData },
      });
      continue;
    }
    const link = renderer.wasmHostAuthoredLinkByEdgeId.get(id);
    if (!link) {
      renderer.clearWasmHostAuthorshipForEdge(id);
      continue;
    }
    const sourceH = renderer.scene.getObjectById(link.source);
    const targetH = renderer.scene.getObjectById(link.target);
    if (!(sourceH instanceof BoardSceneHandle) || !(targetH instanceof BoardSceneHandle)) {
      continue;
    }
    extra.push({
      id,
      source: link.source,
      target: link.target,
      edgeKind: renderer.scene.edges.get(id)?.edgeKind || undefined,
      selected: false,
      visible: true,
      userData: {},
    });
  }
  if (extra.length === 0) {
    return descriptor;
  }
  return { ...descriptor, edges: [...descriptor.edges, ...extra] };
}

/** 🔁 Declarative-to-imperative scene sync that preserves stable instances by id. */
export function syncBoardScene(renderer: BoardRenderer, descriptor: BoardSceneDescriptor): void {
  const desiredNodeIds = new Set(descriptor.nodes.map((node) => node.id));
  const desiredHandleIds = new Set(descriptor.handles.map((handle) => handle.id));
  const desiredEdgeIds = new Set(descriptor.edges.map((edge) => edge.id));
  const desiredWireIds = new Set(descriptor.wires.map((wire) => wire.id));

  renderer.batch(() => {
    for (const nodeDescriptor of descriptor.nodes) {
      let existingNode = renderer.scene.getObjectById(nodeDescriptor.id);
      if (existingNode instanceof BoardSceneNode && instanceShapeSyncKey(existingNode) !== nodeShapeSyncKey(nodeDescriptor)) {
        renderer.scene.remove(existingNode);
        existingNode = undefined;
      }
      const resolvedExisting = renderer.scene.getObjectById(nodeDescriptor.id);
      const { handles: _handles, ...nodeProps } = nodeDescriptor;
      const node = resolvedExisting instanceof BoardSceneNode ? resolvedExisting : newBoardNodeFromProps(nodeProps);
      if (!(resolvedExisting instanceof BoardSceneNode)) {
        renderer.scene.add(node);
      }
      applyNodeProps(renderer, node, nodeProps);
    }

    for (const handleDescriptor of descriptor.handles) {
      const parentNode = renderer.scene.getObjectById(handleDescriptor.nodeId);
      if (!(parentNode instanceof BoardSceneNode)) {
        continue;
      }
      const existingHandle = renderer.scene.getObjectById(handleDescriptor.id);
      const { nodeId: _nodeId, ...handleProps } = handleDescriptor;
      const handle = existingHandle instanceof BoardSceneHandle ? existingHandle : new BoardSceneHandle({ ...handleProps, node: parentNode });
      if (!(existingHandle instanceof BoardSceneHandle)) {
        renderer.scene.add(handle);
      }
      applyHandleProps(handle, handleProps, parentNode);
    }

    for (const edgeDescriptor of descriptor.edges) {
      const sourceHandle = renderer.scene.getObjectById(edgeDescriptor.source);
      const targetHandle = renderer.scene.getObjectById(edgeDescriptor.target);
      if (!(sourceHandle instanceof BoardSceneHandle) || !(targetHandle instanceof BoardSceneHandle)) {
        continue;
      }
      const existingEdge = renderer.scene.getObjectById(edgeDescriptor.id);
      const edge = existingEdge instanceof BoardSceneEdge ? existingEdge : new BoardSceneEdge({ ...edgeDescriptor, source: sourceHandle, target: targetHandle });
      if (!(existingEdge instanceof BoardSceneEdge)) {
        renderer.scene.add(edge);
      }
      applyEdgeProps(edge, edgeDescriptor, sourceHandle, targetHandle);
    }

    for (const wireDescriptor of descriptor.wires) {
      const sourceHandle = renderer.scene.getObjectById(wireDescriptor.source);
      if (!(sourceHandle instanceof BoardSceneHandle)) {
        continue;
      }
      const tid = (wireDescriptor.target ?? "").trim();
      let targetHandle: BoardSceneHandle | null = null;
      if (tid !== "") {
        const t = renderer.scene.getObjectById(tid);
        if (!(t instanceof BoardSceneHandle)) {
          continue;
        }
        targetHandle = t;
      }
      const existingWire = renderer.scene.getObjectById(wireDescriptor.id);
      const ex = wireDescriptor.endX;
      const ey = wireDescriptor.endY;
      const wire =
        existingWire instanceof BoardSceneWire
          ? existingWire
          : new BoardSceneWire({
              id: wireDescriptor.id,
              source: sourceHandle,
              target: targetHandle,
              selected: wireDescriptor.selected,
              style: wireDescriptor.style,
              userData: wireDescriptor.userData,
              visible: wireDescriptor.visible,
              endX: typeof ex === "number" && Number.isFinite(ex) ? ex : null,
              endY: typeof ey === "number" && Number.isFinite(ey) ? ey : null,
            });
      if (!(existingWire instanceof BoardSceneWire)) {
        renderer.scene.add(wire);
      }
      applyWireProps(wire, wireDescriptor, sourceHandle, targetHandle);
    }

    renderer.runWithoutSceneDeleteEvents(() => {
      for (const edge of Array.from(renderer.scene.edges.values())) {
        if (!desiredEdgeIds.has(edge.id)) {
          renderer.clearWasmHostAuthorshipForEdge(edge.id);
          renderer.scene.remove(edge);
        }
      }
      for (const wire of Array.from(renderer.scene.wires.values())) {
        if (!desiredWireIds.has(wire.id)) {
          renderer.scene.remove(wire);
        }
      }
      for (const handle of Array.from(renderer.scene.handles.values())) {
        if (!desiredHandleIds.has(handle.id)) {
          renderer.scene.remove(handle);
        }
      }
      for (const node of Array.from(renderer.scene.nodes.values())) {
        if (!desiredNodeIds.has(node.id)) {
          renderer.scene.remove(node);
        }
      }
    });
  });

  renderer.syncInteractionChrome();
  renderer.invalidate();
}
//#endregion 🔖Scene Sync

//#region 🔖HostMountBridge
/** @emoji 🌉 Secondary host root per {@link BoardRenderer}; scene sync runs on `children` changes and on {@link BoardRenderer.subscribeWasmHostSceneMergeResync} bumps (WASM graph drains), camera only on `camera` prop changes so marker/selection JSX churn does not reset pan/zoom. */
function BoardHostSubtree({ camera, children, renderer }: { camera?: Partial<CameraState>; children: ReactNode; renderer: BoardRenderer }): null {
  const hostMountRef = reactHostPort.useRef<BoardHostMount | null>(null);
  const mountedRendererRef = reactHostPort.useRef<BoardRenderer | null>(null);
  const Bridge = useHostMountBridge();
  const wasmHostSceneMergeResyncEpoch = reactHostPort.useSyncExternalStore(renderer.subscribeWasmHostSceneMergeResync, renderer.getWasmHostSceneMergeResyncEpoch, renderer.getWasmHostSceneMergeResyncEpoch);

  reactHostPort.useLayoutEffect(() => {
    if (hostMountRef.current === null || mountedRendererRef.current !== renderer) {
      if (hostMountRef.current) {
        unmountBoardHostMount(hostMountRef.current);
        hostMountRef.current = null;
      }
      hostMountRef.current = createBoardHostMount(renderer);
      mountedRendererRef.current = renderer;
    }
    updateBoardHostMount(hostMountRef.current, createElement(Bridge, null, children), null);
    const jsxDescriptor = buildBoardSceneDescriptor(children);
    syncBoardScene(renderer, mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsxDescriptor));
  }, [children, renderer, wasmHostSceneMergeResyncEpoch]);

  reactHostPort.useLayoutEffect(() => {
    const cx = camera?.x ?? 0;
    const cy = camera?.y ?? 0;
    const cz = camera?.zoom ?? 1;
    renderer.setCamera(cx, cy, cz);
  }, [camera?.x, camera?.y, camera?.zoom, renderer]);

  reactHostPort.useLayoutEffect(
    () => () => {
      if (hostMountRef.current) {
        unmountBoardHostMount(hostMountRef.current);
        hostMountRef.current = null;
        mountedRendererRef.current = null;
      }
    },
    [],
  );

  return null;
}
//#endregion 🔖HostMountBridge

//#region 🔖Canvas
/** 🖼️ React board root that keeps the hot path inside the imperative renderer. */
export function BoardCanvas({
  camera,
  children,
  companions,
  className,
  contextMenu,
  fixtureDragDrop,
  height,
  gridFactor,
  gridSnapEnabled,
  kindCatalogs,
  kindCompatibility,
  lodZoomThresholds,
  automaticLod,
  lod,
  onLodChange,
  onCamera,
  onChange,
  onChildEdgeChange,
  onChildEdgesChange,
  onChildNodeChange,
  onChildNodesChange,
  onConnect,
  onContextMenu,
  onCreate,
  onDelete,
  onDrag,
  onEdgeChange,
  onEdgeCreate,
  onEdgeDelete,
  onFixtureDrop,
  onHover,
  onIndirectConnect,
  onInvalidate,
  onLinkCompatibleNodes,
  onLinkTargetRing,
  linkSession,
  onNodeChange,
  onNodeCreate,
  onNodeDelete,
  onPan,
  onParentEdgeChange,
  onParentNodeChange,
  onProximityConnect,
  onReady,
  onSelect,
  selection,
  defaultSelection,
  preselection,
  defaultPreselection,
  onPreselect,
  hoveredId: hoveredIdProp,
  defaultHoveredId,
  onViewportChange,
  onWireChange,
  onWireCreate,
  onWireDestroy,
  onZoom,
  renderMode,
  selectionMethod,
  selectionMode,
  selectionTargets,
  style,
  width,
  worldRasterTiling,
}: BoardCanvasProps): ReactElement {
  const canvasRef = reactHostPort.useRef<HTMLCanvasElement | null>(null);
  const textOverlayCanvasRef = reactHostPort.useRef<HTMLCanvasElement | null>(null);
  const containerRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const [contextRenderer, setContextRenderer] = reactHostPort.useState<BoardRenderer | null>(null);
  const rendererRef = reactHostPort.useRef<BoardRenderer | null>(null);
  const [uncontrolledSelection, setUncontrolledSelection] = reactHostPort.useState<BoardSelectionSnapshot>(() => normalizeBoardSelectionProp(defaultSelection));
  const [uncontrolledPreselection, setUncontrolledPreselection] = reactHostPort.useState<BoardPreselectSnapshot>(() => normalizeBoardPreselectProp(defaultPreselection));
  const [uncontrolledHoveredId, setUncontrolledHoveredId] = reactHostPort.useState<string | null>(defaultHoveredId ?? null);
  const selectionControlled = selection !== undefined;
  const preselectionControlled = preselection !== undefined;
  const hoveredControlled = hoveredIdProp !== undefined;
  const resolvedSelection = selectionControlled ? normalizeBoardSelectionProp(selection) : uncontrolledSelection;
  const resolvedPreselection = preselectionControlled ? normalizeBoardPreselectProp(preselection) : uncontrolledPreselection;
  const resolvedHoveredId = hoveredControlled ? (hoveredIdProp ?? null) : uncontrolledHoveredId;
  const boardTargetMenusRef = reactHostPort.useRef(new Map<string, ContextMenuItem[]>());
  const [surfaceContextMenu, setSurfaceContextMenu] = reactHostPort.useState<{ clientX: number; clientY: number; items: ContextMenuItem[] } | null>(null);
  const [fixtureDragActive, setFixtureDragActive] = reactHostPort.useState(false);
  const fileDragDepthRef = reactHostPort.useRef(0);
  const resolvedFixtureDragDrop = fixtureDragDrop ?? Boolean(onFixtureDrop);
  const handleDragEnter = reactHostPort.useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!resolvedFixtureDragDrop) {
        return;
      }
      if (![...event.dataTransfer.types].includes(BOARD_FIXTURE_DRAG_V1_MIME)) {
        return;
      }
      fileDragDepthRef.current += 1;
      setFixtureDragActive(true);
    },
    [resolvedFixtureDragDrop],
  );

  const handleDragLeave = reactHostPort.useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!resolvedFixtureDragDrop) {
        return;
      }
      if (event.currentTarget.contains(event.relatedTarget as globalThis.Node)) {
        return;
      }
      fileDragDepthRef.current = Math.max(0, fileDragDepthRef.current - 1);
      if (fileDragDepthRef.current === 0) {
        setFixtureDragActive(false);
      }
    },
    [resolvedFixtureDragDrop],
  );

  const handleDragOver = reactHostPort.useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!resolvedFixtureDragDrop) {
        return;
      }
      if ([...event.dataTransfer.types].includes(BOARD_FIXTURE_DRAG_V1_MIME)) {
        event.preventDefault();
        event.dataTransfer.dropEffect = "copy";
      }
    },
    [resolvedFixtureDragDrop],
  );

  const handleDrop = reactHostPort.useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!resolvedFixtureDragDrop) {
        return;
      }
      event.preventDefault();
      fileDragDepthRef.current = 0;
      setFixtureDragActive(false);
      const text = event.dataTransfer.getData(BOARD_FIXTURE_DRAG_V1_MIME);
      const fixture = decodeBoardFixtureFromDragV1(text);
      if (!fixture) {
        return;
      }
      const canvas = canvasRef.current;
      const renderer = rendererRef.current;
      if (!canvas || !renderer) {
        return;
      }
      const bounds = canvas.getBoundingClientRect();
      const screen = { x: event.clientX - bounds.left, y: event.clientY - bounds.top };
      const world = renderer.screenToWorld(screen);
      const detail: BoardFixtureDropDetail = { fixture, screen, world };
      onFixtureDrop?.(detail);
      renderer.emit("fixtureDrop", detail);
    },
    [onFixtureDrop, resolvedFixtureDragDrop],
  );

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    const descriptor = buildBoardSceneDescriptor(children);
    const next = new Map<string, ContextMenuItem[]>();
    for (const n of descriptor.nodes) {
      if (n.contextMenu?.length) {
        next.set(n.id, n.contextMenu);
      }
    }
    for (const h of descriptor.handles) {
      if (h.contextMenu?.length) {
        next.set(h.id, h.contextMenu);
      }
    }
    for (const e of descriptor.edges) {
      if (e.contextMenu?.length) {
        next.set(e.id, e.contextMenu);
      }
    }
    boardTargetMenusRef.current = next;
  }, [children, contextRenderer]);

  reactHostPort.useEffect(() => {
    if (!contextRenderer || (!onHover && hoveredControlled)) {
      return () => undefined;
    }
    return contextRenderer.on("hover", (payload) => {
      if (!hoveredControlled) {
        setUncontrolledHoveredId(payload.id);
      }
      onHover?.(payload);
    });
  }, [contextRenderer, hoveredControlled, onHover]);

  reactHostPort.useEffect(() => {
    if (!contextRenderer) {
      return () => undefined;
    }
    const unsubs: Array<() => void> = [];
    if (onChange) {
      unsubs.push(contextRenderer.on("change", onChange));
    }
    if (onNodeCreate) {
      unsubs.push(contextRenderer.on("nodeCreate", onNodeCreate));
    }
    if (onNodeChange) {
      unsubs.push(contextRenderer.on("nodeChange", onNodeChange));
    }
    if (onNodeDelete) {
      unsubs.push(contextRenderer.on("nodeDelete", onNodeDelete));
    }
    if (onParentNodeChange) {
      unsubs.push(contextRenderer.on("parentNodeChange", onParentNodeChange));
    }
    if (onParentEdgeChange) {
      unsubs.push(contextRenderer.on("parentEdgeChange", onParentEdgeChange));
    }
    if (onChildNodeChange) {
      unsubs.push(contextRenderer.on("childNodeChange", onChildNodeChange));
    }
    if (onChildEdgeChange) {
      unsubs.push(contextRenderer.on("childEdgeChange", onChildEdgeChange));
    }
    if (onChildNodesChange) {
      unsubs.push(contextRenderer.on("childNodesChange", onChildNodesChange));
    }
    if (onChildEdgesChange) {
      unsubs.push(contextRenderer.on("childEdgesChange", onChildEdgesChange));
    }
    if (onEdgeChange) {
      unsubs.push(contextRenderer.on("edgeChange", onEdgeChange));
    }
    if (onEdgeCreate) {
      unsubs.push(contextRenderer.on("edgeCreate", onEdgeCreate));
    }
    if (onEdgeDelete) {
      unsubs.push(contextRenderer.on("edgeDelete", onEdgeDelete));
    }
    if (onWireCreate) {
      unsubs.push(contextRenderer.on("wireCreate", onWireCreate));
    }
    if (onWireChange) {
      unsubs.push(contextRenderer.on("wireChange", onWireChange));
    }
    if (onWireDestroy) {
      unsubs.push(contextRenderer.on("wireDestroy", onWireDestroy));
    }
    return () => {
      for (const u of unsubs) {
        u();
      }
    };
  }, [
    contextRenderer,
    onChange,
    onChildEdgeChange,
    onChildEdgesChange,
    onChildNodeChange,
    onChildNodesChange,
    onEdgeChange,
    onEdgeCreate,
    onEdgeDelete,
    onNodeChange,
    onNodeCreate,
    onNodeDelete,
    onParentEdgeChange,
    onParentNodeChange,
    onWireChange,
    onWireCreate,
    onWireDestroy,
  ]);

  reactHostPort.useEffect(() => {
    if (!contextRenderer) {
      return () => undefined;
    }
    const unsubs: Array<() => void> = [];
    if (onConnect) {
      unsubs.push(contextRenderer.on("edgeCreate", onConnect));
    }
    if (onIndirectConnect) {
      unsubs.push(contextRenderer.on("indirectConnect", onIndirectConnect));
    }
    if (onProximityConnect) {
      unsubs.push(contextRenderer.on("proximityConnect", onProximityConnect));
    }
    return () => {
      for (const u of unsubs) {
        u();
      }
    };
  }, [contextRenderer, onConnect, onIndirectConnect, onProximityConnect]);

  reactHostPort.useEffect(() => {
    if (!contextRenderer) {
      return () => undefined;
    }
    const unsubs: Array<() => void> = [];
    if (onLinkCompatibleNodes) {
      unsubs.push(contextRenderer.on("linkCompatibleNodes", onLinkCompatibleNodes));
    }
    if (onLinkTargetRing) {
      unsubs.push(contextRenderer.on("linkTargetRing", onLinkTargetRing));
    }
    return () => {
      for (const u of unsubs) {
        u();
      }
    };
  }, [contextRenderer, onLinkCompatibleNodes, onLinkTargetRing]);

  reactHostPort.useEffect(() => {
    if (!contextRenderer || (!onCamera && !onViewportChange && !onPan && !onZoom)) {
      return () => undefined;
    }
    let prev = contextRenderer.getCameraSnapshot();
    return contextRenderer.on("camera", (next) => {
      onCamera?.(next);
      onViewportChange?.(next);
      if (Math.abs(prev.zoom - next.zoom) > 1e-9) {
        onZoom?.(next);
      }
      if (Math.abs(prev.x - next.x) > 1e-6 || Math.abs(prev.y - next.y) > 1e-6) {
        onPan?.(next);
      }
      prev = next;
    });
  }, [contextRenderer, onCamera, onPan, onViewportChange, onZoom]);

  reactHostPort.useEffect(() => {
    if (!contextRenderer) {
      return () => undefined;
    }
    const unsubs: Array<() => void> = [];
    if (onSelect || !selectionControlled) {
      unsubs.push(
        contextRenderer.on("select", (snapshot) => {
          if (!selectionControlled) {
            setUncontrolledSelection(snapshot);
          }
          onSelect?.(snapshot);
        }),
      );
    }
    if (onPreselect || !preselectionControlled) {
      const onPreselectEvent = (snapshot: BoardPreselectSnapshot): void => {
        if (!preselectionControlled) {
          setUncontrolledPreselection(snapshot);
        }
        onPreselect?.(snapshot);
      };
      unsubs.push(contextRenderer.on("preselect", onPreselectEvent));
      unsubs.push(contextRenderer.on("preselectCancel", onPreselectEvent));
    }
    if (onInvalidate) {
      unsubs.push(contextRenderer.on("invalidate", onInvalidate));
    }
    if (onDrag) {
      unsubs.push(contextRenderer.on("nodeMove", onDrag));
    }
    return () => {
      for (const u of unsubs) {
        u();
      }
    };
  }, [contextRenderer, onDrag, onInvalidate, onPreselect, onSelect, preselectionControlled, selectionControlled]);

  reactHostPort.useEffect(() => {
    if (!contextRenderer || (!onCreate && !onDelete)) {
      return () => undefined;
    }
    const unsubs: Array<() => void> = [];
    if (onCreate) {
      unsubs.push(
        contextRenderer.on("nodeCreate", (p) => {
          onCreate({ kind: "node", id: p.id });
        }),
      );
      unsubs.push(
        contextRenderer.on("edgeCreate", (p) => {
          onCreate({ kind: "edge", id: p.id, source: p.source, target: p.target });
        }),
      );
      unsubs.push(
        contextRenderer.on("wireCreate", (payload) => {
          onCreate({ kind: "wire", payload });
        }),
      );
    }
    if (onDelete) {
      unsubs.push(
        contextRenderer.on("nodeDelete", (p) => {
          onDelete({ kind: "node", id: p.id });
        }),
      );
      unsubs.push(
        contextRenderer.on("edgeDelete", (p) => {
          onDelete({ kind: "edge", id: p.id });
        }),
      );
      unsubs.push(
        contextRenderer.on("wireDestroy", (p) => {
          onDelete({ kind: "wire", id: p.id });
        }),
      );
    }
    return () => {
      for (const u of unsubs) {
        u();
      }
    };
  }, [contextRenderer, onCreate, onDelete]);

  reactHostPort.useEffect(() => {
    if (!contextRenderer) {
      return () => undefined;
    }
    return contextRenderer.on("contextmenu", (payload) => {
      onContextMenu?.(payload);
      const items = payload.id ? (boardTargetMenusRef.current.get(payload.id) ?? []) : (contextMenu ?? []);
      if (!items.length) {
        return;
      }
      setSurfaceContextMenu({ clientX: payload.clientX, clientY: payload.clientY, items });
    });
  }, [contextMenu, contextRenderer, onContextMenu]);

  reactHostPort.useLayoutEffect(() => {
    if (!canvasRef.current) {
      return;
    }
    const canvas = canvasRef.current;
    const renderer = new BoardRenderer({
      canvas,
      automaticLod: automaticLod ?? true,
      gridFactor: gridFactor ?? DEFAULT_BOARD_GRID_FACTOR,
      gridSnapEnabled: gridSnapEnabled ?? false,
      lodZoomThresholds: lodZoomThresholds ?? DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
      ...(lod !== undefined ? { lod } : {}),
      renderMode,
      selection: { method: selectionMethod, mode: selectionMode, targets: selectionTargets },
      worldRasterTiling,
    });
    rendererRef.current = renderer;
    activeBoardRenderer = renderer;
    setContextRenderer(renderer);
    return () => {
      const r = renderer;
      queueMicrotask(() => {
        r.dispose();
        if (activeBoardRenderer === r) {
          activeBoardRenderer = null;
        }
        if (rendererRef.current === r) {
          rendererRef.current = null;
        }
      });
    };
  }, [renderMode]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setWorldRasterTilingOption(worldRasterTiling);
  }, [worldRasterTiling]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setGridFactor(gridFactor ?? DEFAULT_BOARD_GRID_FACTOR);
  }, [gridFactor]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setLodZoomThresholds(lodZoomThresholds ?? DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS);
  }, [lodZoomThresholds]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setGridSnapEnabled(gridSnapEnabled ?? false);
  }, [gridSnapEnabled]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setLinkSession(linkSession ?? null);
  }, [linkSession]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setAutomaticLod(automaticLod ?? true);
    renderer.setForcedDrawLod(lod);
  }, [automaticLod, lod]);

  reactHostPort.useEffect(() => {
    if (!contextRenderer) {
      return;
    }
    onReady?.(contextRenderer);
  }, [contextRenderer, onReady]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.attachTextOverlayCanvas(textOverlayCanvasRef.current);
    return () => {
      renderer.attachTextOverlayCanvas(null);
    };
  }, [contextRenderer, renderMode]);

  reactHostPort.useEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer || typeof document === "undefined" || typeof MutationObserver === "undefined") {
      return undefined;
    }
    if (renderMode === "headless-test") {
      return undefined;
    }
    const root = document.documentElement;
    const observer = new MutationObserver(() => {
      renderer.invalidate();
    });
    observer.observe(root, { attributeFilter: ["class", "style"], attributes: true });
    return () => {
      observer.disconnect();
    };
  }, [contextRenderer, renderMode]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setSelectionOptions({ method: selectionMethod, mode: selectionMode, targets: selectionTargets });
  }, [selectionMethod, selectionMode, selectionTargets]);

  const lastSyncedControlledSelectionRef = reactHostPort.useRef<BoardSelectionSnapshot | null>(null);
  const lastSyncedControlledPreselectionRef = reactHostPort.useRef<BoardPreselectSnapshot | null>(null);

  reactHostPort.useLayoutEffect(() => {
    lastSyncedControlledSelectionRef.current = null;
    lastSyncedControlledPreselectionRef.current = null;
  }, [contextRenderer]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    if (lastSyncedControlledSelectionRef.current !== null && boardSelectionSnapshotsEqual(resolvedSelection, lastSyncedControlledSelectionRef.current)) {
      return;
    }
    lastSyncedControlledSelectionRef.current = resolvedSelection;
    renderer.setSelectionIdsSilent(resolvedSelection.ids);
  }, [resolvedSelection]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    if (lastSyncedControlledPreselectionRef.current !== null && boardPreselectSnapshotsEqual(resolvedPreselection, lastSyncedControlledPreselectionRef.current)) {
      return;
    }
    lastSyncedControlledPreselectionRef.current = resolvedPreselection;
    renderer.syncPreselectionSilent(resolvedPreselection);
  }, [resolvedPreselection]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.syncHoveredIdSilent(resolvedHoveredId);
  }, [resolvedHoveredId]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setKindCatalogs(kindCatalogs ?? DEFAULT_KIND_CATALOG_BUNDLE);
  }, [kindCatalogs]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setKindCompatibility(kindCompatibility);
  }, [kindCompatibility]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    const container = containerRef.current;
    if (!renderer || !container) {
      return;
    }

    const applySize = (): void => {
      const nextWidth = width ?? container.clientWidth ?? 1;
      const nextHeight = height ?? container.clientHeight ?? 1;
      renderer.setSize(nextWidth, nextHeight, globalThis.devicePixelRatio || 1);
      renderer.render();
    };

    applySize();
    if (typeof ResizeObserver === "undefined") {
      return undefined;
    }

    const observer = new ResizeObserver(() => {
      const schedule =
        typeof globalThis.requestAnimationFrame === "function"
          ? (fn: () => void) => {
              globalThis.requestAnimationFrame(fn);
            }
          : (fn: () => void) => {
              queueMicrotask(fn);
            };
      schedule(applySize);
    });
    observer.observe(container);
    return () => {
      observer.disconnect();
    };
  }, [height, width]);

  return (
    <BoardContext.Provider value={contextRenderer}>
      <div
        className={["flex min-h-0 min-w-0 flex-1 flex-col", className, fixtureDragActive ? "ring-2 ring-[color:var(--color-accent)] ring-offset-2 ring-offset-[color:var(--color-base)]" : ""].filter(Boolean).join(" ") || undefined}
        onDragEnter={handleDragEnter}
        onDragLeave={handleDragLeave}
        onDragOver={handleDragOver}
        onDrop={(e) => void handleDrop(e)}
        ref={containerRef}
        style={{ height: height ?? "100%", position: "relative", width: width ?? "100%", ...(style ?? {}) }}
      >
        <canvas className="min-h-0 min-w-0 flex-1 touch-none" data-testid="board-canvas" ref={canvasRef} style={{ display: "block", height: "100%", width: "100%" }} />
        {renderMode === "headless-test" ? null : <canvas aria-hidden className="pointer-events-none absolute inset-0 min-h-0 min-w-0" data-testid="board-text-overlay" ref={textOverlayCanvasRef} />}
        {contextRenderer ? (
          <>
            <HostMountProvider>
              <BoardHostSubtree camera={camera} children={children} renderer={contextRenderer} />
              {onLodChange ? <BoardDrawLodReporter onLodChange={onLodChange} /> : null}
            </HostMountProvider>
            {companions}
          </>
        ) : null}
        <ContextMenuController
          items={surfaceContextMenu?.items ?? []}
          onOpenChange={(nextOpen) => {
            if (!nextOpen) {
              setSurfaceContextMenu(null);
            }
          }}
          open={surfaceContextMenu !== null}
          position={surfaceContextMenu ? { x: surfaceContextMenu.clientX, y: surfaceContextMenu.clientY } : null}
        />
      </div>
    </BoardContext.Provider>
  );
}
//#endregion 🔖Canvas

//#region 🔖Hooks
/** @emoji 📶 Subscribes to {@link BoardRenderer} draw LOD band changes for window measure labels. */
export function BoardDrawLodReporter({ onLodChange }: { onLodChange?: (lod: BoardDrawLodKind) => void }): null {
  const renderer = useBoard();
  const lod = reactHostPort.useSyncExternalStore(renderer.subscribeDrawLod, renderer.getDrawLodSnapshot, renderer.getDrawLodSnapshot);
  reactHostPort.useEffect(() => {
    onLodChange?.(lod);
  }, [lod, onLodChange]);
  return null;
}

/** 🎯 Access the imperative board renderer from within BoardCanvas descendants (DOM or secondary host tree). */
export function useBoard(): BoardRenderer {
  const renderer = reactHostPort.useContext(BoardContext);
  if (renderer) {
    return renderer;
  }
  if (activeBoardRenderer) {
    return activeBoardRenderer;
  }
  throw new Error("useBoard must be used inside BoardCanvas.");
}

/** 📷 Read and update camera state through an external store subscription. */
export function useCamera(): [CameraState, (camera: CameraState) => void] {
  const renderer = useBoard();
  const snapshot = reactHostPort.useSyncExternalStore(renderer.subscribeCamera, renderer.getCameraSnapshot, renderer.getCameraSnapshot);
  return [snapshot, (nextCamera) => renderer.setCamera(nextCamera.x, nextCamera.y, nextCamera.zoom)];
}

/** ✅ Subscribe to semantic selection ids without pushing React through the drag hot path. */
export function useSelection(): BoardSelectionSnapshot {
  const renderer = useBoard();
  return reactHostPort.useSyncExternalStore(renderer.subscribeSelection, renderer.getSelectionSnapshot, renderer.getSelectionSnapshot);
}

/** @emoji 👁️ Subscribe to area-select preview ids (and anchor-removed ids) on the active board renderer. */
export function usePreselection(): BoardPreselectSnapshot {
  const renderer = useBoard();
  return reactHostPort.useSyncExternalStore(renderer.subscribePreselect, renderer.getPreselectSnapshot, renderer.getPreselectSnapshot);
}

/** 📡 Bind a board event listener with stable cleanup (`fixtureDrop`, `hover`, `change` / graph observation events, `contextmenu`, …). */
export function useBoardEvent<TKey extends keyof BoardEventMap>(name: TKey, handler: (payload: BoardEventMap[TKey]) => void): void {
  const renderer = useBoard();
  reactHostPort.useEffect(() => renderer.on(name, handler), [handler, name, renderer]);
}

/** ⏱️ Subscribe to imperative frame callbacks emitted after each render pass. */
export function useFrame(callback: (state: FrameState, dt: number) => void): void {
  const renderer = useBoard();
  reactHostPort.useEffect(() => renderer.subscribeFrame(callback), [callback, renderer]);
}

/** 🔄 Imperatively request another render for the active board root. */
export function invalidate(renderer?: BoardRenderer): void {
  (renderer ?? activeBoardRenderer)?.invalidate();
}
//#endregion 🔖Hooks

//#region 🔖Vitest
const boardReactVitest = (
  import.meta as ImportMeta & {
    vitest?: {
      afterEach: typeof import("vitest").afterEach;
      describe: typeof import("vitest").describe;
      expect: typeof import("vitest").expect;
      it: typeof import("vitest").it;
      vi: typeof import("vitest").vi;
    };
  }
).vitest;

if (boardReactVitest) {
  const { afterEach, beforeAll, describe, expect, it, vi } = boardReactVitest;
  (globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

  beforeAll(async () => {
    await ensureElementsBoardWasmLoaded();
  });

  function installCanvasStub(): () => void {
    const getContextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockImplementation(() => {
      return {
        arc: vi.fn(),
        beginPath: vi.fn(),
        bezierCurveTo: vi.fn(),
        clearRect: vi.fn(),
        clip: vi.fn(),
        closePath: vi.fn(),
        fill: vi.fn(),
        fillRect: vi.fn(),
        fillStyle: "#000000",
        fillText: vi.fn(),
        font: "",
        lineCap: "round",
        lineJoin: "round",
        lineTo: vi.fn(),
        lineWidth: 1,
        measureText: vi.fn((s: string) => ({ width: s.length * 6 })),
        moveTo: vi.fn(),
        rect: vi.fn(),
        restore: vi.fn(),
        save: vi.fn(),
        setLineDash: vi.fn(),
        setTransform: vi.fn(),
        stroke: vi.fn(),
        strokeRect: vi.fn(),
        strokeStyle: "#000000",
        textAlign: "center",
        textBaseline: "middle",
      } as unknown as CanvasRenderingContext2D;
    });
    return () => {
      getContextSpy.mockRestore();
    };
  }

  afterEach(() => {
    document.body.innerHTML = "";
  });

  function BoardSelectListenerStub(): null {
    useBoardEvent("select", () => undefined);
    return null;
  }

  describe("board react helpers", () => {
    it("boardFixtureSceneMarkers maps nakagin fixture into scene descriptors", async () => {
      const nakaginFixtureJson = (await import("../fixture/nakagin-capsule-tower.2d.json")).default as unknown;
      const fixture = parseBoardFixtureV1(nakaginFixtureJson);
      expect(fixture?.nodes.length).toBeGreaterThan(100);
      const descriptor = buildBoardSceneDescriptor(boardFixtureSceneMarkers(fixture!));
      expect(descriptor.nodes.length).toBe(fixture!.nodes.length);
      expect(descriptor.edges.length).toBe(fixture!.edges.length);
      expect(descriptor.handles.length).toBeGreaterThan(fixture!.nodes.length);
    });

    it("createBoardHostMount registers React 19 error reporters on the host root", () => {
      const canvas = document.createElement("canvas");
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      const mount = createBoardHostMount(renderer);
      expect(typeof mount.onUncaughtError).toBe("function");
      expect(typeof mount.onCaughtError).toBe("function");
      expect(typeof mount.onRecoverableError).toBe("function");
      renderer.dispose();
    });

    it("builds a flat scene descriptor from declarative markers", () => {
      const descriptor = buildBoardSceneDescriptor(
        <>
          <Node id="a" radius={24} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>
          <Edge id="edge-1" source="a:h0" target="a:h0" />
        </>,
      );

      expect(descriptor.nodes).toHaveLength(1);
      expect(descriptor.handles).toEqual([
        {
          angle: 0,
          color: undefined,
          contextMenu: undefined,
          handleKind: "port",
          id: "a:h0",
          nodeId: "a",
          radius: undefined,
          selected: undefined,
          style: undefined,
          userData: undefined,
          visible: undefined,
        },
      ]);
      expect(descriptor.edges).toEqual([{ contextMenu: undefined, id: "edge-1", selected: undefined, source: "a:h0", style: undefined, target: "a:h0", userData: undefined, visible: undefined }]);
    });

    it("preserves contextMenu entries on descriptors", () => {
      const nodeMenu: ContextMenuItem[] = [{ id: "n1", label: "Node" }];
      const handleMenu: ContextMenuItem[] = [{ id: "h1", label: "Handle" }];
      const edgeMenu: ContextMenuItem[] = [{ id: "e1", label: "Edge" }];
      const descriptor = buildBoardSceneDescriptor(
        <>
          <Node contextMenu={nodeMenu} id="a" radius={24} x={0} y={0}>
            <Handle handleKind="port" angle={0} contextMenu={handleMenu} id="a:h0" />
          </Node>
          <Edge contextMenu={edgeMenu} id="edge-1" source="a:h0" target="a:h0" />
        </>,
      );
      expect(descriptor.nodes[0]?.contextMenu).toEqual(nodeMenu);
      expect(descriptor.handles[0]?.contextMenu).toEqual(handleMenu);
      expect(descriptor.edges[0]?.contextMenu).toEqual(edgeMenu);
    });

    it("mergeWasmHostAuthoredEdgesIntoDescriptor keeps WASM gesture edges across JSX-only syncs until adopted", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const jsx = buildBoardSceneDescriptor(
        <>
          <Node id="a" radius={40} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>
          <Node id="b" radius={40} x={200} y={0}>
            <Handle handleKind="port" angle={Math.PI} id="b:h0" />
          </Node>
        </>,
      );
      syncBoardScene(renderer, jsx);
      const sourceHandle = renderer.scene.handles.get("a:h0");
      const targetHandle = renderer.scene.handles.get("b:h0");
      expect(sourceHandle).toBeDefined();
      expect(targetHandle).toBeDefined();
      renderer.scene.ingestWasmEdge(new BoardSceneEdge({ id: "edge-link-99", source: sourceHandle as BoardSceneHandle, target: targetHandle as BoardSceneHandle }));
      renderer.wasmHostAuthoredEdgeIds.add("edge-link-99");
      renderer.wasmHostAuthoredLinkByEdgeId.set("edge-link-99", { source: "a:h0", target: "b:h0" });
      const merged = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsx);
      expect(merged.edges.some((e) => e.id === "edge-link-99")).toBe(true);
      syncBoardScene(renderer, merged);
      expect(renderer.scene.edges.has("edge-link-99")).toBe(true);
      const merged2 = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsx);
      syncBoardScene(renderer, merged2);
      expect(renderer.scene.edges.has("edge-link-99")).toBe(true);
      const adopted = buildBoardSceneDescriptor(
        <>
          <Node id="a" radius={40} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>
          <Node id="b" radius={40} x={200} y={0}>
            <Handle handleKind="port" angle={Math.PI} id="b:h0" />
          </Node>
          <Edge id="edge-link-99" source="a:h0" target="b:h0" />
        </>,
      );
      mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, adopted);
      expect(renderer.wasmHostAuthoredEdgeIds.has("edge-link-99")).toBe(false);
      expect(renderer.wasmHostAuthoredLinkByEdgeId.has("edge-link-99")).toBe(false);
      renderer.dispose();
    });

    it("mergeWasmHostAuthoredEdgesIntoDescriptor rebuilds from link map after scene edge removal", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const jsx = buildBoardSceneDescriptor(
        <>
          <Node id="a" radius={40} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>
          <Node id="b" radius={40} x={200} y={0}>
            <Handle handleKind="port" angle={Math.PI} id="b:h0" />
          </Node>
        </>,
      );
      syncBoardScene(renderer, jsx);
      const sourceHandle = renderer.scene.handles.get("a:h0");
      const targetHandle = renderer.scene.handles.get("b:h0");
      expect(sourceHandle).toBeDefined();
      expect(targetHandle).toBeDefined();
      const edge = new BoardSceneEdge({
        id: "edge-link-map",
        source: sourceHandle as BoardSceneHandle,
        target: targetHandle as BoardSceneHandle,
      });
      renderer.scene.ingestWasmEdge(edge);
      renderer.wasmHostAuthoredEdgeIds.add("edge-link-map");
      renderer.wasmHostAuthoredLinkByEdgeId.set("edge-link-map", { source: "a:h0", target: "b:h0" });
      renderer.scene.remove(edge);
      expect(renderer.scene.edges.has("edge-link-map")).toBe(false);
      const merged = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsx);
      expect(merged.edges.some((e) => e.id === "edge-link-map")).toBe(true);
      syncBoardScene(renderer, merged);
      expect(renderer.scene.edges.has("edge-link-map")).toBe(true);
      renderer.dispose();
    });

    it("keeps wasm-only link edges after graph drain by re-running JSX merge when children omit Edge markers", async () => {
      const restoreCanvas = installCanvasStub();
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);
      let readyRenderer: BoardRenderer | null = null;
      await act(async () => {
        root.render(
          <BoardCanvas
            camera={{ x: 0, y: 0, zoom: BOARD_LOD_DETAIL_MIN_ZOOM }}
            height={600}
            onReady={(r) => {
              readyRenderer = r;
            }}
            renderMode="headless-test"
            width={800}
          >
            <Node id="a" radius={40} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
            <Node id="b" radius={40} x={280} y={0}>
              <Handle handleKind="port" angle={Math.PI} id="b:h0" />
            </Node>
          </BoardCanvas>,
        );
        await Promise.resolve();
        await Promise.resolve();
      });
      const canvas = container.querySelector("canvas") as HTMLCanvasElement & { __boardRenderer?: BoardRenderer };
      const renderer = requireRenderer(canvas.__boardRenderer ?? readyRenderer);
      Object.defineProperty(canvas, "clientWidth", { configurable: true, value: 800 });
      Object.defineProperty(canvas, "clientHeight", { configurable: true, value: 600 });
      Object.defineProperty(canvas, "getBoundingClientRect", {
        configurable: true,
        value: () => ({ bottom: 600, height: 600, left: 0, right: 800, top: 0, width: 800, x: 0, y: 0 }),
      });
      expect(renderer.scene.getObjectById("a:h0")).toBeDefined();
      expect(renderer.getWasmHostSceneMergeResyncEpoch()).toBe(0);
      renderer.render();
      const nodeA = renderer.scene.getObjectById("a") as BoardSceneNode;
      const nodeB = renderer.scene.getObjectById("b") as BoardSceneNode;
      const p0 = renderer.worldToScreen(computeHandlePosition(nodeA, 0));
      const pMid = renderer.worldToScreen({ x: 140, y: 0 });
      const p1 = renderer.worldToScreen(computeHandlePosition(nodeB, Math.PI));
      await act(async () => {
        canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: p0.x, clientY: p0.y }));
        canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, clientX: pMid.x + 20, clientY: pMid.y }));
        canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, clientX: p1.x, clientY: p1.y }));
        canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: p1.x, clientY: p1.y }));
        await Promise.resolve();
        await Promise.resolve();
      });
      const linkIds = [...renderer.scene.edges.keys()].filter((k) => k.startsWith("edge-link-"));
      expect(linkIds.length).toBe(1);
      expect(renderer.wasmHostAuthoredEdgeIds.has(linkIds[0]!)).toBe(true);
      expect(renderer.getWasmHostSceneMergeResyncEpoch()).toBeGreaterThan(0);
      await act(async () => {
        root.render(
          <BoardCanvas
            camera={{ x: 0, y: 0, zoom: BOARD_LOD_DETAIL_MIN_ZOOM }}
            height={600}
            onReady={(r) => {
              readyRenderer = r;
            }}
            renderMode="headless-test"
            width={800}
          >
            <Node id="a" radius={40} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
            <Node id="b" radius={40} x={280} y={0}>
              <Handle handleKind="port" angle={Math.PI} id="b:h0" />
            </Node>
          </BoardCanvas>,
        );
      });
      const linkIdsAfterRelayout = [...renderer.scene.edges.keys()].filter((k) => k.startsWith("edge-link-"));
      expect(linkIdsAfterRelayout).toEqual(linkIds);
      await act(async () => {
        root.unmount();
      });
      document.body.removeChild(container);
      restoreCanvas();
    });

    it("emits contextmenu with hovered id after wasm hit pass", () => {
      const restoreCanvas = installCanvasStub();
      const canvas = document.createElement("canvas");
      Object.defineProperty(canvas, "clientWidth", { configurable: true, value: 800 });
      Object.defineProperty(canvas, "clientHeight", { configurable: true, value: 600 });
      Object.defineProperty(canvas, "getBoundingClientRect", {
        configurable: true,
        value: () => ({ bottom: 600, height: 600, left: 0, right: 800, top: 0, width: 800, x: 0, y: 0 }),
      });
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      syncBoardScene(
        renderer,
        buildBoardSceneDescriptor(
          <Node id="hit" radius={50} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="hit:h0" />
          </Node>,
        ),
      );
      renderer.render();
      const payloads: Array<{ id: string | null }> = [];
      renderer.on("contextmenu", (ev) => payloads.push({ id: ev.id }));
      const at = renderer.worldToScreen({ x: 0, y: 0 });
      canvas.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, clientX: at.x, clientY: at.y }));
      expect(payloads).toHaveLength(1);
      expect(payloads[0]?.id).toBe("hit");
      renderer.dispose();
      restoreCanvas();
    });

    it("emits contextmenu with null id when pointer misses scene objects", () => {
      const restoreCanvas = installCanvasStub();
      const canvas = document.createElement("canvas");
      Object.defineProperty(canvas, "clientWidth", { configurable: true, value: 800 });
      Object.defineProperty(canvas, "clientHeight", { configurable: true, value: 600 });
      Object.defineProperty(canvas, "getBoundingClientRect", {
        configurable: true,
        value: () => ({ bottom: 600, height: 600, left: 0, right: 800, top: 0, width: 800, x: 0, y: 0 }),
      });
      const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
      syncBoardScene(
        renderer,
        buildBoardSceneDescriptor(
          <Node id="lonely" radius={10} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="lonely:h0" />
          </Node>,
        ),
      );
      renderer.render();
      const ids: Array<string | null> = [];
      renderer.on("contextmenu", (ev) => ids.push(ev.id));
      const far = renderer.worldToScreen({ x: 1_000_000, y: 1_000_000 });
      canvas.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, clientX: far.x, clientY: far.y }));
      expect(ids).toEqual([null]);
      renderer.dispose();
      restoreCanvas();
    });

    it("syncBoardScene ignores descriptor selected flags and reapplies interaction chrome", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const descriptor = buildBoardSceneDescriptor(<Node id="solo" radius={36} selected x={0} y={0} text="caption" />);
      syncBoardScene(renderer, descriptor);
      const node = renderer.scene.nodes.get("solo");
      expect(node?.selected).toBe(false);
      renderer.setSelectionIds(["solo"]);
      syncBoardScene(renderer, descriptor);
      expect(node?.selected).toBe(true);
      renderer.setSelectionIds([]);
      syncBoardScene(renderer, descriptor);
      expect(node?.selected).toBe(false);
      renderer.dispose();
    });

    it("buildBoardSceneDescriptor ignores opaque components (use secondary host for nested composition)", () => {
      function OpaqueScene(): ReactElement {
        return (
          <Node id="inner" radius={8} x={1} y={2}>
            <Handle handleKind="port" angle={0} id="inner.h" />
          </Node>
        );
      }
      const descriptor = buildBoardSceneDescriptor(
        <>
          <OpaqueScene />
        </>,
      );
      expect(descriptor.nodes).toHaveLength(0);
      expect(descriptor.handles).toHaveLength(0);
    });

    it("secondary host mounts handle under node without BoardCanvas", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const hostMount = createBoardHostMount(renderer);
      act(() => {
        updateBoardHostMount(
          hostMount,
          createElement(BOARD_HOST_NODE, { draggable: true, id: "host-a-node", radius: 10, selected: false, visible: true, x: 0, y: 0 }, createElement(BOARD_HOST_HANDLE, { angle: 0, id: "host-a-handle", selected: false, visible: true })),
          null,
        );
      });
      expect(renderer.scene.getObjectById("host-a-node")).toBeInstanceOf(BoardSceneNode);
      expect(renderer.scene.getObjectById("host-a-handle")).toBeInstanceOf(BoardSceneHandle);
      unmountBoardHostMount(hostMount);
      renderer.dispose();
    });

    it("mounts handle children for flat host markers", async () => {
      const restoreCanvas = installCanvasStub();
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);

      await act(async () => {
        root.render(
          <BoardCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" width={160}>
            <Node id="direct" radius={10} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="direct.h" />
            </Node>
          </BoardCanvas>,
        );
        await Promise.resolve();
      });

      const canvas = container.querySelector("canvas");
      const renderer = (canvas as HTMLCanvasElement & { __boardRenderer?: BoardRenderer }).__boardRenderer;
      expect(renderer?.scene.getObjectById("direct")).toBeInstanceOf(BoardSceneNode);
      expect(renderer?.scene.getObjectById("direct.h")).toBeInstanceOf(BoardSceneHandle);

      await act(async () => {
        root.unmount();
      });
      restoreCanvas();
    });

    it("mounts nodes through wrapper components via the secondary host", async () => {
      const restoreCanvas = installCanvasStub();
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);

      function WrappedScene(): ReactElement {
        return (
          <Node id="wrapped" radius={14} x={3} y={4}>
            <Handle handleKind="port" angle={0} id="wrapped.h" />
          </Node>
        );
      }

      await act(async () => {
        root.render(
          <BoardCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" width={160}>
            <WrappedScene />
          </BoardCanvas>,
        );
        await Promise.resolve();
      });

      const canvas = container.querySelector("canvas");
      const renderer = (canvas as HTMLCanvasElement & { __boardRenderer?: BoardRenderer }).__boardRenderer;
      expect(renderer?.scene.getObjectById("wrapped")).toBeInstanceOf(BoardSceneNode);
      expect(renderer?.scene.getObjectById("wrapped.h")).toBeInstanceOf(BoardSceneHandle);

      await act(async () => {
        root.unmount();
      });
      restoreCanvas();
    });

    it("syncs declarative updates into stable imperative instances", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const firstDescriptor = buildBoardSceneDescriptor(
        <Node draggable id="a" radius={24} x={10} y={20}>
          <Handle handleKind="port" angle={0} id="a:h0" />
        </Node>,
      );
      syncBoardScene(renderer, firstDescriptor);

      const firstNode = renderer.scene.getObjectById("a");
      const secondDescriptor = buildBoardSceneDescriptor(
        <Node draggable id="a" radius={30} x={40} y={50}>
          <Handle handleKind="port" angle={Math.PI / 2} id="a:h0" />
        </Node>,
      );
      syncBoardScene(renderer, secondDescriptor);

      const secondNode = renderer.scene.getObjectById("a");
      expect(secondNode).toBe(firstNode);
      expect(secondNode).toBeInstanceOf(BoardSceneNode);
      expect((secondNode as BoardSceneNode).x).toBe(40);
      expect((secondNode as BoardSceneNode).radius).toBe(30);

      renderer.dispose();
    });

    it("syncs handleKind from declarative handles into scene instances", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const descriptor = buildBoardSceneDescriptor(
        <Node id="n" radius={20} x={0} y={0}>
          <Handle angle={0} handleKind="slot-a" id="h1" />
        </Node>,
      );
      syncBoardScene(renderer, descriptor);
      const h = renderer.scene.getObjectById("h1") as BoardSceneHandle;
      expect(h.handleKind).toBe("slot-a");
      renderer.dispose();
    });

    it("replaces the imperative node when declarative shape changes from circle to rectangle", () => {
      const renderer = new BoardRenderer({ renderMode: "headless-test" });
      const circleDescriptor = buildBoardSceneDescriptor(
        <Node id="a" radius={20} x={0} y={0}>
          <Handle handleKind="port" angle={0} id="a:h0" />
        </Node>,
      );
      syncBoardScene(renderer, circleDescriptor);
      const firstNode = renderer.scene.getObjectById("a");
      const rectDescriptor = buildBoardSceneDescriptor(
        <Node height={30} id="a" shape="rectangle" width={40} x={0} y={0}>
          <Handle handleKind="port" angle={0} id="a:h0" />
        </Node>,
      );
      syncBoardScene(renderer, rectDescriptor);
      const secondNode = renderer.scene.getObjectById("a");
      expect(secondNode).not.toBe(firstNode);
      expect((secondNode as BoardSceneNode).shape).toBe("rectangle");
      expect((secondNode as BoardSceneNode).width).toBe(40);
      renderer.dispose();
    });

    it("mounts BoardCanvas and updates scene objects when JSX props change", async () => {
      const restoreCanvas = installCanvasStub();
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);
      let readyRenderer: BoardRenderer | null = null;
      const onReadyNoop = (): void => undefined;

      await act(async () => {
        root.render(
          <BoardCanvas
            camera={{ x: 0, y: 0, zoom: 1 }}
            height={480}
            onReady={(renderer) => {
              readyRenderer = renderer;
            }}
            renderMode="headless-test"
            width={640}
          >
            <Node draggable id="a" radius={28} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
            <Node id="b" radius={28} x={180} y={0}>
              <Handle handleKind="port" angle={Math.PI} id="b:h0" />
            </Node>
            <Edge id="edge-1" source="a:h0" target="b:h0" />
          </BoardCanvas>,
        );
        await Promise.resolve();
      });
      expect(readyRenderer).not.toBeNull();
      const createdRenderer = requireRenderer(readyRenderer);
      expect(createdRenderer.scene.getObjectById("edge-1")).toBeInstanceOf(BoardSceneEdge);

      await act(async () => {
        root.render(
          <BoardCanvas camera={{ x: 20, y: 10, zoom: 1.2 }} height={480} onReady={onReadyNoop} renderMode="headless-test" width={640}>
            <Node draggable id="a" radius={28} x={120} y={40}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
            <Node id="b" radius={28} x={180} y={0}>
              <Handle handleKind="port" angle={Math.PI} id="b:h0" />
            </Node>
            <Edge id="edge-1" source="a:h0" target="b:h0" />
          </BoardCanvas>,
        );
        await Promise.resolve();
      });
      /** Secondary host commit can trail the outer `act` tick; mirror JSX into the imperative scene before reading coordinates. */
      const movedDescriptor = buildBoardSceneDescriptor(
        <>
          <Node draggable id="a" radius={28} x={120} y={40}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>
          <Node id="b" radius={28} x={180} y={0}>
            <Handle handleKind="port" angle={Math.PI} id="b:h0" />
          </Node>
          <Edge id="edge-1" source="a:h0" target="b:h0" />
        </>,
      );
      syncBoardScene(createdRenderer, movedDescriptor);
      const canvasAfterMove = container.querySelector("canvas");
      const rendererAfterMove = requireRenderer((canvasAfterMove as HTMLCanvasElement & { __boardRenderer?: BoardRenderer | undefined }).__boardRenderer);
      const movedNode = rendererAfterMove.scene.getObjectById("a") as BoardSceneNode;
      expect(movedNode.x).toBe(120);
      expect(movedNode.y).toBe(40);
      expect(rendererAfterMove.getCameraSnapshot()).toEqual({ x: 20, y: 10, zoom: 1.2 });

      await act(async () => {
        root.unmount();
      });
      restoreCanvas();
    });

    it("does not dispose BoardRenderer when only selection props change", async () => {
      const restoreCanvas = installCanvasStub();
      const disposeSpy = vi.spyOn(BoardRenderer.prototype, "dispose");
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);

      await act(async () => {
        root.render(
          <BoardCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" selectionMethod="rectangle" selectionMode="additive" selectionTargets={{ nodes: true, edges: false, handles: false }} width={160}>
            <Node id="a" radius={12} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
          </BoardCanvas>,
        );
        await Promise.resolve();
      });

      disposeSpy.mockClear();

      await act(async () => {
        root.render(
          <BoardCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" selectionMethod="lasso" selectionMode="invertive" selectionTargets={{ nodes: false, edges: true, handles: false }} width={160}>
            <Node id="a" radius={12} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
          </BoardCanvas>,
        );
        await Promise.resolve();
      });

      expect(disposeSpy).not.toHaveBeenCalled();
      const canvas = container.querySelector("canvas");
      const renderer = requireRenderer((canvas as HTMLCanvasElement & { __boardRenderer?: BoardRenderer | undefined }).__boardRenderer ?? null);
      expect(renderer.getSelectionOptions().method).toBe("lasso");
      expect(renderer.getSelectionOptions().mode).toBe("invertive");
      expect(renderer.getSelectionOptions().targets).toEqual({ nodes: false, edges: true, handles: false });

      await act(async () => {
        root.unmount();
      });
      expect(disposeSpy).toHaveBeenCalledTimes(1);
      disposeSpy.mockRestore();
      restoreCanvas();
    });

    it("defers BoardCanvas children until the renderer exists so useBoard hooks do not throw", async () => {
      const restoreCanvas = installCanvasStub();
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);

      await act(async () => {
        root.render(
          <BoardCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" width={160}>
            <BoardSelectListenerStub />
            <Node draggable id="a" radius={12} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
          </BoardCanvas>,
        );
        await Promise.resolve();
      });

      await act(async () => {
        root.unmount();
      });
      restoreCanvas();
    });
  });
}
//#endregion 🔖Vitest
