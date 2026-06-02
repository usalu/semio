// #region 🧲Header
/** @emoji 📋 `@puzzle/2d/react` — WASM puzzle 2d renderer + React canvas (depends only on `@ui/react`). */
// #endregion 🧲Header

// #region 🔌Adapters
import { ContextMenuController, reactHostPort, type ContextMenuItem } from "@ui/react";
import React from "react";
import Reconciler from "react-reconciler";
import { ContinuousEventPriority, DefaultEventPriority, DiscreteEventPriority, LegacyRoot, NoEventPriority } from "react-reconciler/constants";
// #endregion 🔌Adapters

type Puzzle2dListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

class Puzzle2dEventBindingController {
  private readonly cleanups: Array<() => void> = [];

  listen(target: Puzzle2dListenerTarget | null | undefined, kind: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void {
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
import initPuzzle2dWasm, { boardComputeEdgeBezier, boardHandlePositionCircle, boardHandlePositionRectangle, boardRedrawHandlesFixtureJson, boardRedrawLayoutFixtureJson, BoardSession, initSync } from "../rs/pkg/puzzle_2d.js";

if (import.meta.env.VITEST) {
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../rs/pkg/puzzle_2d_bg.wasm");
  initSync({ module: readFileSync(wasmPath) });
} else {
  await initPuzzle2dWasm();
}

/** @emoji 🌐 Idempotent: resolves after the wasm-bindgen `web` target has finished instantiating. */
export async function ensurePuzzle2dWasmLoaded(): Promise<void> {
  await initPuzzle2dWasm();
}

export { BoardSession };
// #endregion 🔖GpuWasmBridge

//#region 🔖Kinds
export type Puzzle2dSceneObjectKind = "node" | "handle" | "edge" | "wire";
export type RenderMode = "main-thread" | "worker-offscreen" | "headless-test";
export type Puzzle2dSelectionMethod = "lasso" | "rectangle";
export type Puzzle2dSelectionMode = "additive" | "default" | "invertive" | "subtractive";

/** @emoji 🎯 Which graph kinds participate in rectangle/lasso selection and hit picking. */
export interface Puzzle2dSelectionTargets {
  edges: boolean;
  handles: boolean;
  nodes: boolean;
}

/** @emoji 🎯 Default: nodes, edges, and handles all participate (matches prior `nodes&edges`). */
export const PUZZLE_2D_SELECTION_TARGETS_DEFAULT: Puzzle2dSelectionTargets = {
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

/** @emoji 🎯 Local handle template on a {@link NodeKind} (perimeter angle in board space). */
export interface NodeKindHandleTemplate {
  readonly handleKind: string;
  readonly angle: number;
  readonly radius?: number;
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
  /** @emoji 🎯 Local handle templates for palette / brush instantiation on this node kind. */
  handles?: readonly NodeKindHandleTemplate[];
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

/** @emoji 📚 Central WASM+host registries for semantic puzzle 2d kinds (omit slices to leave prior catalog entries untouched when pushing partial updates is not supported — always send full merged bundle from callers). */
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

/** @emoji 📚 Default {@link KindCatalogBundle} for {@link Puzzle2dCanvas} when callers omit `kindCatalogs`. */
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

function parseNodeKindHandleTemplates(raw: unknown): NodeKindHandleTemplate[] | undefined {
  if (!Array.isArray(raw) || raw.length === 0) {
    return undefined;
  }
  const handles: NodeKindHandleTemplate[] = [];
  for (const row of raw) {
    if (!row || typeof row !== "object") {
      continue;
    }
    const box = row as Record<string, unknown>;
    const handleKind = typeof box.handleKind === "string" ? box.handleKind.trim() : "";
    const angle = box.angle;
    if (handleKind === "" || typeof angle !== "number" || !Number.isFinite(angle)) {
      continue;
    }
    const radius = box.radius;
    handles.push({
      handleKind,
      angle,
      ...(typeof radius === "number" && Number.isFinite(radius) ? { radius } : {}),
    });
  }
  return handles.length ? handles : undefined;
}

function parseNodeKindsFromFixtureJson(raw: unknown): readonly NodeKind[] | undefined {
  if (!Array.isArray(raw)) {
    return undefined;
  }
  const nodes: NodeKind[] = [];
  for (const row of raw) {
    if (!row || typeof row !== "object") {
      continue;
    }
    const box = row as Record<string, unknown>;
    const id = typeof box.id === "string" ? box.id.trim() : "";
    if (id === "") {
      continue;
    }
    const name = typeof box.name === "string" ? box.name.trim() : id;
    const shapeRaw = box.shape;
    const shape = shapeRaw === "circle" || shapeRaw === "rectangle" ? shapeRaw : undefined;
    const handles = parseNodeKindHandleTemplates(box.handles);
    nodes.push({
      id,
      name,
      ...(shape !== undefined ? { shape } : {}),
      ...(typeof box.color === "string" && box.color.trim() !== "" ? { color: box.color.trim() } : {}),
      ...(typeof box.stroke === "string" && box.stroke.trim() !== "" ? { stroke: box.stroke.trim() } : {}),
      ...(typeof box.icon === "string" && box.icon.trim() !== "" ? { icon: box.icon.trim() } : {}),
      ...(box.defaultShapeProps && typeof box.defaultShapeProps === "object" ? { defaultShapeProps: box.defaultShapeProps as Record<string, unknown> } : {}),
      ...(typeof box.defaultHandleKind === "string" && box.defaultHandleKind.trim() !== "" ? { defaultHandleKind: box.defaultHandleKind.trim() } : {}),
      ...(handles ? { handles } : {}),
    });
  }
  return nodes.length ? nodes : undefined;
}

/** @emoji 🗂️ Returns `meta.kindCatalogs` from raw puzzle 2d fixture JSON when present (nodes/handles slices only). */
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
  const nodes = parseNodeKindsFromFixtureJson(nodesRaw);
  if (nodes) {
    out.nodes = nodes;
  }
  if (Array.isArray(handlesRaw)) {
    out.handles = handlesRaw as readonly HandleKind[];
  }
  if (out.nodes === undefined && out.handles === undefined) {
    return undefined;
  }
  return out;
}

/** @emoji 🔗 Returns `meta.kindCompatibility` from raw puzzle 2d fixture JSON when present. */
export function puzzle2dFixtureMetaKindCompatibility(raw: unknown): readonly KindCompatEntry[] | undefined {
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
      if (e.handles?.length) {
        row.handles = e.handles.map((h) => ({
          handleKind: h.handleKind,
          angle: h.angle,
          ...(h.radius !== undefined ? { radius: h.radius } : {}),
        }));
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

/** @emoji 🏷️ WASM host snapshot for text overlay paint (matches the last {@link BoardSession.renderFrame}). */
interface Puzzle2dOverlayPaintStateWasm {
  readonly camera: CameraState;
  readonly lod: string;
  readonly nodes: ReadonlyArray<{ readonly id: string; readonly x: number; readonly y: number }>;
}

export interface Puzzle2dSelectionSnapshot {
  ids: string[];
}

/** @emoji 👁️ Rectangle/lasso drag preview ids plus anchor ids leaving the committed selection during the gesture. */
export interface Puzzle2dPreselectSnapshot {
  ids: string[];
  removedIds: string[];
}

/** @emoji 👁️ Empty area-select preview (no ids highlighted, none marked removed). */
export const PUZZLE_2D_PRESELECT_EMPTY: Puzzle2dPreselectSnapshot = { ids: [], removedIds: [] };

/** @emoji 🎯 Committed selection vs area-select preview chrome (`preselect∖selection` selected, `removedIds` highlighted). */
export function puzzle2dElementInteractionChrome(selectionIds: Iterable<string>, preselection: Puzzle2dPreselectSnapshot): { highlightedIds: Set<string>; selectedIds: Set<string> } {
  const selection = new Set(selectionIds);
  if (preselection.ids.length === 0) {
    return { selectedIds: selection, highlightedIds: new Set() };
  }
  const selectedIds = new Set(preselection.ids.filter((id) => !selection.has(id)));
  const highlightedIds = new Set(preselection.removedIds);
  return { selectedIds, highlightedIds };
}

/** @emoji 🎨 Resolves headless / fallback style key from interaction chrome flags (selected beats highlighted). */
export function puzzle2dObjectChromeStyleKey(base: "edge" | "handle" | "node", object: Puzzle2dSceneObject): string {
  if (object.selected) {
    return `${base}.selected`;
  }
  if (object.highlighted) {
    return `${base}.highlighted`;
  }
  return base;
}

/** @emoji 🎨 Style key from committed selection / preselect only (not scene object flags). */
export function puzzle2dInteractionChromeStyleKey(base: "edge" | "handle" | "node", id: string, chrome: { highlightedIds: Set<string>; selectedIds: Set<string> }): string {
  if (chrome.selectedIds.has(id)) {
    return `${base}.selected`;
  }
  if (chrome.highlightedIds.has(id)) {
    return `${base}.highlighted`;
  }
  return base;
}

export interface Puzzle2dSelectionOptions {
  method?: Puzzle2dSelectionMethod;
  mode?: Puzzle2dSelectionMode;
  targets?: Partial<Puzzle2dSelectionTargets>;
}

/** @emoji 🎯 Resolved selection options passed to WASM (`targets` fully specified). */
export type ResolvedPuzzle2dSelectionOptions = {
  method: Puzzle2dSelectionMethod;
  mode: Puzzle2dSelectionMode;
  targets: Puzzle2dSelectionTargets;
};

export interface Puzzle2dStyle {
  fill?: string;
  stroke?: string;
  strokeWidth?: number;
}

export interface FrameState {
  camera: CameraState;
  renderer: Puzzle2dRenderer;
  selection: Puzzle2dSelectionSnapshot;
}

export interface CubicBezierCurve {
  p0: Point;
  p1: Point;
  p2: Point;
  p3: Point;
}

/** @emoji 📄 Handle record inside {@link Puzzle2dFixtureV1}; optional `radius` overrides default world-space hit/draw size. */
export interface Puzzle2dFixtureHandleV1 {
  angle: number;
  /** @emoji 🔗 Required after {@link parsePuzzle2dFixtureV1}; JSON may omit it and receive {@link BUILTIN_PORT_HANDLE_KIND}. */
  handleKind: string;
  id: string;
  /** @emoji 🎨 Optional CSS `#rgb` / `#rrggbb` / `#rrggbbaa` overriding the catalog color for this handle. */
  color?: string;
  /** @emoji 🏷️ Optional WASM detail LOD icon string (`typst:` / `$…`, `emoji:`, `data:` / raster data URLs, catalog id, or inline SVG). */
  iconKind?: string;
  radius?: number;
}

/** @emoji 📄 Circle node: {@link Puzzle2dFixtureCircleNodeV1.x}/{@link Puzzle2dFixtureCircleNodeV1.y} are the disk center in layout space; handle {@link Puzzle2dFixtureHandleV1.angle} aims at the connected neighbor (radians). */
export interface Puzzle2dFixtureCircleNodeV1 {
  cad?: { x: number; y: number; z: number } | null;
  handles: Puzzle2dFixtureHandleV1[];
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
  textAlignment?: Puzzle2dNodeTextAlignment;
  /** @emoji 🔤 Optional CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Optional caption size in layout px when not using autofit. */
  textFontSize?: number;
  x: number;
  y: number;
}

/** @emoji 📄 Axis-aligned rectangle: center (x,y) in layout space, full width/height (not half-extents); handle `angle` is **0 at north** (top center), **CCW** in `[0,2π)` (`π/4` NW corner, `π/2` west, …); circles use **east-zero** polar `atan2(dy,dx)`. */
export interface Puzzle2dFixtureRectangleNodeV1 {
  cad?: { x: number; y: number; z: number } | null;
  handles: Puzzle2dFixtureHandleV1[];
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
  textAlignment?: Puzzle2dNodeTextAlignment;
  /** @emoji 🔤 Optional CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Optional caption size in layout px when not using autofit. */
  textFontSize?: number;
  width: number;
  x: number;
  y: number;
}

/** @emoji 📄 Node record inside {@link Puzzle2dFixtureV1} (circle or rectangle body). */
export type Puzzle2dFixtureNodeV1 = Puzzle2dFixtureCircleNodeV1 | Puzzle2dFixtureRectangleNodeV1;

/** @emoji 📄 Edge record inside {@link Puzzle2dFixtureV1}. */
export interface Puzzle2dFixtureEdgeV1 {
  id: string;
  source: string;
  target: string;
}

/** @emoji 📄 Parsed `puzzle.2d.fixture/v1` JSON for declarative puzzle 2d scenes. */
export interface Puzzle2dFixtureV1 {
  camera: CameraState;
  edges: Puzzle2dFixtureEdgeV1[];
  meta?: Record<string, unknown>;
  nodes: Puzzle2dFixtureNodeV1[];
  schema: string;
}

// #region 🏷️IconSelectorMode

/** @emoji 🎛️ Puzzle 2d `iconKind` editor tab (`math` = `typst:` / leading `$`, `data` = data URLs, `emoji` = `emoji:` …, `vector` = catalog / inline SVG). */
export type Puzzle2dIconSelectorMode = "data" | "emoji" | "math" | "vector";

function stripLegacyImageDataPrefixForPuzzle2dIcon(raw: string): string {
  const t = raw.trim();
  return t.startsWith("image:") ? t.slice("image:".length).trim() : t;
}

function isRasterDataUrlPayloadForPuzzle2dIcon(s: string): boolean {
  const u = s.trim().toLowerCase();
  return u.startsWith("data:image/png;base64,") || u.startsWith("data:image/jpeg;base64,") || u.startsWith("data:image/jpg;base64,");
}

function looksLikeAsciiCatalogishVectorStemForPuzzle2dIcon(s: string): boolean {
  const t = s.trim();
  if (t === "") {
    return false;
  }
  if (!/^[\w.-]+$/.test(t)) {
    return false;
  }
  return /[.-_]/.test(t) || t.length > 48;
}

/** @emoji 🧭 Picks a {@link Puzzle2dIconSelectorMode} tab for a stored puzzle 2d icon string (align with `puzzle2d_resolve_icon_kind` in `puzzle/2d/rs/lib.rs`). */
export function classifyPuzzle2dIconSelectorMode(raw: string): Puzzle2dIconSelectorMode {
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
  if (lower.startsWith("data:") || isRasterDataUrlPayloadForPuzzle2dIcon(stripLegacyImageDataPrefixForPuzzle2dIcon(t))) {
    return "data";
  }
  if (lower.startsWith("<?xml") || lower.includes("<svg")) {
    return "vector";
  }
  if (looksLikeAsciiCatalogishVectorStemForPuzzle2dIcon(t)) {
    return "vector";
  }
  return "emoji";
}

// #endregion 🏷️IconSelectorMode

/** @emoji 🕸️ JSON options for {@link layoutPuzzle2dFixtureForceGraph} (camelCase; matches Rust `ForceGraphLayoutOptions` / dimforge `nalgebra` spring layout). */
export interface Puzzle2dForceGraphLayoutOptions {
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
export type Puzzle2dHierarchicalTreeDirectionKind = "downwards" | "left" | "right" | "upwards";

/** @emoji 🕸️ WASM redraw dispatcher mode for {@link layoutPuzzle2dFixtureRedrawNodes}. */
export type Puzzle2dRedrawModeKind = "force-graph" | "hierarchical-tree";

/** @emoji 🧩 Options for {@link layoutPuzzle2dFixtureRedrawNodes} (camelCase; mirrors Rust `RedrawFixtureOptions`). */
export interface Puzzle2dRedrawLayoutOptions {
  mode: Puzzle2dRedrawModeKind;
  redrawHandlesAfter: boolean;
  centerX?: number;
  centerY?: number;
  randomSeed?: number;
  forceGraph?: Puzzle2dForceGraphLayoutOptions;
  hierarchicalTree?: {
    direction: Puzzle2dHierarchicalTreeDirectionKind;
    layerSpacing: number;
    siblingGap: number;
  };
}

/** @emoji 🕸️ Runs WASM force-directed layout on fixture node centers (edges via handle ids); uses dimforge `nalgebra` in Rust. */
export function layoutPuzzle2dFixtureForceGraph(fixture: Puzzle2dFixtureV1, options?: Puzzle2dForceGraphLayoutOptions): Puzzle2dFixtureV1 {
  const out = boardRedrawLayoutFixtureJson(
    JSON.stringify(fixture),
    JSON.stringify({
      forceGraph: options ?? {},
      mode: "force-graph",
      redrawHandlesAfter: false,
    }),
  );
  return JSON.parse(out) as Puzzle2dFixtureV1;
}

/** @emoji 🧩 Runs WASM fixture redraw (force graph or hierarchical tree) with optional chained handle snap. */
export function layoutPuzzle2dFixtureRedrawNodes(fixture: Puzzle2dFixtureV1, options: Puzzle2dRedrawLayoutOptions): Puzzle2dFixtureV1 {
  const out = boardRedrawLayoutFixtureJson(JSON.stringify(fixture), JSON.stringify(options));
  return JSON.parse(out) as Puzzle2dFixtureV1;
}

/** @emoji 🔗 Snaps fixture handle angles to straight chords between linked node centers (WASM). */
export function layoutPuzzle2dFixtureRedrawHandles(fixture: Puzzle2dFixtureV1): Puzzle2dFixtureV1 {
  const out = boardRedrawHandlesFixtureJson(JSON.stringify(fixture));
  return JSON.parse(out) as Puzzle2dFixtureV1;
}

/** @emoji 🖱️ Hit-under-pointer payload for {@link Puzzle2dEventMap.hover} (tooltips, status, …). */
export interface Puzzle2dHoverPayload {
  clientX: number;
  clientY: number;
  id: string | null;
  /** @emoji 📐 Canvas-local CSS pixels passed to {@link Puzzle2dRenderer.screenToWorld}. */
  screenX: number;
  screenY: number;
  worldX: number;
  worldY: number;
}

/** @emoji 🪪 Payload for {@link Puzzle2dEventMap.nodeChange} and other single-node graph notifications. */
export interface Puzzle2dGraphNodeIdPayload {
  id: string;
}

/** @emoji 🪪 Payload for {@link Puzzle2dEventMap.childEdgeChange} and {@link Puzzle2dEventMap.parentEdgeChange}. */
export interface Puzzle2dGraphEdgeIdPayload {
  id: string;
}

/** @emoji 🌳 Emitted when the multiset of subtree child node ids under all {@link Node.root} nodes changes. */
export interface Puzzle2dChildNodesChangePayload {
  rootIds: string[];
  nodeIds: string[];
}

/** @emoji 🌳 Emitted when the multiset of subtree edge ids under roots changes (see {@link Puzzle2dChildNodesChangePayload}). */
export interface Puzzle2dChildEdgesChangePayload {
  rootIds: string[];
  edgeIds: string[];
}

/** @emoji 🪢 Payload for {@link Puzzle2dEventMap.edgeCreate} and gesture connect aliases. */
export interface Puzzle2dEdgeLinkPayload {
  id: string;
  source: string;
  target: string;
}

/** @emoji 🧵 Payload for {@link Puzzle2dEventMap.wireCreate} (declarative / scene wire). */
export interface Puzzle2dWireSnapshotPayload {
  endX: number | null;
  endY: number | null;
  id: string;
  source: string;
  target: string | null;
  wireKind: string;
}

/** @emoji 🪪 Payload for {@link Puzzle2dEventMap.wireChange} / {@link Puzzle2dEventMap.wireDestroy}. */
export interface Puzzle2dGraphWireIdPayload {
  id: string;
}

/** @emoji 📦 Optional aggregate for {@link Puzzle2dCanvasProps.onCreate} (node, edge, or wire). */
export type Puzzle2dStructureCreatePayload = { kind: "edge"; id: string; source: string; target: string } | { kind: "node"; id: string } | { kind: "wire"; payload: Puzzle2dWireSnapshotPayload };

/** @emoji 📦 Optional aggregate for {@link Puzzle2dCanvasProps.onDelete} (node, edge, or wire). */
export type Puzzle2dStructureDeletePayload = { kind: "edge"; id: string } | { kind: "node"; id: string } | { kind: "wire"; id: string };

export interface Puzzle2dLinkCompatibleNodesPayload {
  readonly source: string;
  readonly nodeIds: readonly string[];
}

export interface Puzzle2dLinkTargetRingPayload {
  readonly source: string;
  readonly nodeId: string | null;
  readonly handleIds: readonly string[];
}

/** @emoji 🖌️ Active puzzle 2d viewport tool. */
export type Puzzle2dActiveTool = "select" | "brush";

/** @emoji 📐 Default brush node span in world units (play authoring uses the same value). */
export const DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX = 40;

/** @emoji 📐 Default brush flush offset (`2 ×` node diameter). */
export const DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX = DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX * 2;

/** @emoji 🖌️ WASM `brushPlace` payload for fixture commit. */
export interface Puzzle2dBrushPlacePayload {
  readonly handles: readonly { readonly angle: number; readonly handleKind: string; readonly radius?: number }[];
  readonly nodeKind: string;
  readonly shape: "circle" | "rectangle";
  readonly sourceHandleId: string;
  readonly targetHandleIndex: number;
  readonly x: number;
  readonly y: number;
  readonly edgeId?: string;
  readonly height?: number;
  readonly iconKind?: string;
  readonly nodeId?: string;
  readonly radius?: number;
  readonly width?: number;
}

/** @emoji 🖌️ Brush candidate node kinds while hovering a slot. */
export interface Puzzle2dBrushCandidatesPayload {
  readonly candidates: readonly string[];
  readonly index: number;
  readonly sourceHandleId: string;
}

/** @emoji 🖌️ Shared brush slot state mirrored across play authoring panes. */
export interface Puzzle2dBrushSessionSnapshot {
  readonly candidateIndex: number;
  readonly candidates: readonly string[];
  readonly preview: Puzzle2dEventMap["brushPreview"] | null;
  readonly sourceHandleId: string | null;
}

/** @emoji 🔗 Host-driven link gesture preview mirrored across flat surfaces (see {@link Puzzle2dCanvasProps.linkSession}). */
export interface Puzzle2dLinkSessionSnapshot {
  readonly source: string;
  readonly endX: number;
  readonly endY: number;
  readonly compatiblePartIds: readonly string[];
  readonly ringPartId: string | null;
  readonly ringAnchorIds: readonly string[];
}

export interface Puzzle2dEventMap {
  camera: CameraState;
  change: undefined;
  childEdgeChange: Puzzle2dGraphEdgeIdPayload;
  childEdgesChange: Puzzle2dChildEdgesChangePayload;
  childNodeChange: Puzzle2dGraphNodeIdPayload;
  childNodesChange: Puzzle2dChildNodesChangePayload;
  contextmenu: { clientX: number; clientY: number; id: string | null; x: number; y: number };
  edgeChange: Puzzle2dGraphEdgeIdPayload;
  edgeCreate: Puzzle2dEdgeLinkPayload;
  edgeDelete: { id: string };
  fixtureDrop: Puzzle2dFixtureDropDetail;
  hover: Puzzle2dHoverPayload;
  indirectConnect: Puzzle2dEdgeLinkPayload;
  linkCompatibleNodes: Puzzle2dLinkCompatibleNodesPayload;
  linkTargetRing: Puzzle2dLinkTargetRingPayload;
  invalidate: undefined;
  nodeChange: Puzzle2dGraphNodeIdPayload;
  nodeCreate: Puzzle2dGraphNodeIdPayload;
  nodeDelete: { id: string };
  nodeMove: { id: string; x: number; y: number };
  nodeDragEnd: { moves: Array<{ id: string; x: number; y: number }> };
  parentEdgeChange: Puzzle2dGraphEdgeIdPayload;
  parentNodeChange: Puzzle2dGraphNodeIdPayload;
  proximityConnect: Puzzle2dEdgeLinkPayload;
  brushCandidates: Puzzle2dBrushCandidatesPayload;
  brushPlace: Puzzle2dBrushPlacePayload;
  brushPreview: { readonly edge: { readonly sourceHandleId: string; readonly targetHandleIndex: number } | null; readonly node: Record<string, unknown> | null };
  select: Puzzle2dSelectionSnapshot;
  preselect: Puzzle2dPreselectSnapshot;
  preselectCancel: Puzzle2dPreselectSnapshot;
  wireChange: Puzzle2dGraphWireIdPayload;
  wireCreate: Puzzle2dWireSnapshotPayload;
  wireDestroy: Puzzle2dGraphWireIdPayload;
}

export interface Puzzle2dSceneObjectOptions {
  draggable?: boolean;
  highlighted?: boolean;
  id: string;
  selected?: boolean;
  style?: string;
  userData?: Record<string, unknown>;
  visible?: boolean;
}

/** @emoji 🔵 World-space circle node (center + radius). */
export type CircleNodeOptions = Puzzle2dSceneObjectOptions & {
  handles?: Puzzle2dSceneHandle[];
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
  textAlignment?: Puzzle2dNodeTextAlignment;
  /** @emoji 🔤 CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Caption size in layout px when not using autofit. */
  textFontSize?: number;
  x: number;
  y: number;
};

/** @emoji 🟩 World-space axis-aligned rectangle node (center + full width and height). */
export type RectangleNodeOptions = Puzzle2dSceneObjectOptions & {
  handles?: Puzzle2dSceneHandle[];
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
  textAlignment?: Puzzle2dNodeTextAlignment;
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

export interface HandleOptions extends Puzzle2dSceneObjectOptions {
  angle: number;
  /** @emoji 🎨 Optional CSS hex fill overriding the handle-kind catalog color on the WASM host. */
  color?: string | null;
  /** @emoji 🏷️ Optional WASM detail LOD icon string (`typst:` / `$…`, `emoji:`, `data:` / raster data URLs, catalog id, or inline SVG). */
  iconKind?: string;
  /** @emoji 🔗 Semantic handle kind for WASM link compatibility (not {@link Puzzle2dSceneObject.kind}). */
  handleKind: string;
  node: Puzzle2dSceneNode;
  radius?: number;
}

/** @emoji 🟣 Declarative handle marker props (React + reconciler). */
export interface Puzzle2dHandleProps {
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
export interface Puzzle2dEdgeProps {
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

/** @emoji 🧵 Declarative wire props: anchored at {@link Puzzle2dWireProps.source}; either {@link Puzzle2dWireProps.target} handle id **or** {@link Puzzle2dWireProps.endX}/{@link Puzzle2dWireProps.endY} world end. */
export interface Puzzle2dWireProps {
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

export interface EdgeOptions extends Puzzle2dSceneObjectOptions {
  edgeKind?: string;
  source: Puzzle2dSceneHandle;
  target: Puzzle2dSceneHandle;
}

export interface WireOptions extends Puzzle2dSceneObjectOptions {
  endX?: number | null;
  endY?: number | null;
  source: Puzzle2dSceneHandle;
  target: Puzzle2dSceneHandle | null;
  wireKind?: string;
}

type FrameListener = (state: FrameState, dt: number) => void;
type Puzzle2dCanvasElement = HTMLCanvasElement & { __puzzle2dRenderer?: Puzzle2dRenderer };
type Puzzle2dCanvasContext = Pick<
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
export const PUZZLE_2D_CAMERA_ZOOM_MIN = 0.05;
/** @emoji 🔎 Largest allowed world scale (most zoomed-in). */
export const PUZZLE_2D_CAMERA_ZOOM_MAX = 32;

const MIN_ZOOM = PUZZLE_2D_CAMERA_ZOOM_MIN;
const MAX_ZOOM = PUZZLE_2D_CAMERA_ZOOM_MAX;

/** @emoji ⌨️ True when Delete/Backspace should reach the puzzle 2d canvas instead of staying in a focused text control. */
function shouldPuzzle2dHandleDeleteShortcut(): boolean {
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
/** 📐 Quantized large grid step in world units (LOD grids scale `10` / `2.5` / `0.5` / `0.1` by {@link DEFAULT_PUZZLE_2D_GRID_FACTOR}). */
export const PUZZLE_2D_LOD_GRID_MAJOR_QUANTUM = 10;

/** @emoji 📐 Positive multiplier for LOD world grid steps (`10×` / `2.5×` / `0.5×` / `0.1×` world units per band); default `10` yields `100` / `25` / `5` / `1`. */
export const DEFAULT_PUZZLE_2D_GRID_FACTOR = 10;

/** @emoji 📐 Default LOD zoom boundaries (world scale / CSS pixels); minimap < `minimapMaxZoom` < overview < `overviewMaxZoom` < compact < `compactMaxZoom` < normal < `normalMaxZoom` < detail < `detailMaxZoom` ≤ micro. */
export interface Puzzle2dLodZoomThresholds {
  minimapMaxZoom: number;
  overviewMaxZoom: number;
  compactMaxZoom: number;
  normalMaxZoom: number;
  detailMaxZoom: number;
}

export const DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS: Puzzle2dLodZoomThresholds = {
  minimapMaxZoom: 0.15,
  overviewMaxZoom: 0.35,
  compactMaxZoom: 0.55,
  normalMaxZoom: 1.25,
  detailMaxZoom: 2.5,
};

/** 📐 Alias of {@link DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS.minimapMaxZoom}. */
export const PUZZLE_2D_LOD_MINIMAP_MAX_ZOOM = DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS.minimapMaxZoom;

/** 📐 Alias of {@link DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS.normalMaxZoom} (detail band starts here). */
export const PUZZLE_2D_LOD_DETAIL_MIN_ZOOM = DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS.normalMaxZoom;

/** 📐 Alias of {@link DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS.detailMaxZoom} (micro band starts here). */
export const PUZZLE_2D_LOD_MICRO_MIN_ZOOM = DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS.detailMaxZoom;

/** @emoji 📶 LOD label for `data-puzzle2d-lod` using explicit thresholds. */
export function resolvePuzzle2dLodLabelFromThresholds(zoom: number, t: Puzzle2dLodZoomThresholds): Puzzle2dDrawLodKind {
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

/** @emoji 📶 LOD tier for `data-puzzle2d-lod` using {@link DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS}. */
export function resolvePuzzle2dLodLabel(zoom: number): Puzzle2dDrawLodKind {
  return resolvePuzzle2dLodLabelFromThresholds(zoom, DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS);
}

/** @emoji 📶 WASM draw LOD tier label (matches `data-puzzle2d-lod` / `setForcedDrawLodLabel`). */
export type Puzzle2dDrawLodKind = "compact" | "detail" | "micro" | "minimap" | "normal" | "overview";

/** @emoji 📶 Select value: camera zoom picks the draw LOD band. */
export const PUZZLE_2D_LOD_MODE_AUTOMATIC = "automatic" as const;

/** @emoji 📶 Puzzle 2d play / window LOD select value (`automatic` or a pinned {@link Puzzle2dDrawLodKind}). */
export type Puzzle2dLodModeKind = typeof PUZZLE_2D_LOD_MODE_AUTOMATIC | Puzzle2dDrawLodKind;

const PUZZLE_2D_DRAW_LOD_KINDS: readonly Puzzle2dDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

/** @emoji ✅ True when `label` is a pinned WASM draw LOD tier. */
export function isPuzzle2dDrawLodKind(label: string): label is Puzzle2dDrawLodKind {
  return (PUZZLE_2D_DRAW_LOD_KINDS as readonly string[]).includes(label);
}

/** @emoji 📶 Maps a window LOD select value to {@link Puzzle2dCanvasProps} LOD fields. */
export function puzzle2dLodCanvasProps(mode: Puzzle2dLodModeKind): { automaticLod: boolean; lod?: Puzzle2dDrawLodKind } {
  if (mode === PUZZLE_2D_LOD_MODE_AUTOMATIC) {
    return { automaticLod: true };
  }
  return { automaticLod: false, lod: mode };
}

/** @emoji 📶 Automatic LOD select row label showing the live zoom-derived tier. */
export function puzzle2dLodAutomaticSelectLabel(effectiveTier: Puzzle2dDrawLodKind): string {
  return `Automatic · ${effectiveTier.charAt(0).toUpperCase()}${effectiveTier.slice(1)}`;
}

/** @emoji 🎨 Offline / headless paint defaults aligned with `elements/core/styling/tokens.json` `board_vello_canvas` sRGB (Vello host defaults before DOM tokens sync). */
const PUZZLE_2D_STYLES_HEADLESS_FALLBACK: Record<string, Puzzle2dStyle> = {
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

const DEFAULT_STYLES: Record<string, Puzzle2dStyle> = PUZZLE_2D_STYLES_HEADLESS_FALLBACK;

//#region 🎨ElementsUiPuzzle2dPaint
/** @emoji 🎨 Elements semantic tokens for committed selection chrome (primary, not secondary). */
const PUZZLE_2D_CSS_COLOR_PRIMARY = "var(--color-primary)";
const PUZZLE_2D_CSS_SELECTED_FILL = "color-mix(in oklab, var(--color-primary) 28%, var(--color-panel))";
/** @emoji 🎨 Secondary-tinted fill for preselect exit / highlight chrome only. */
const PUZZLE_2D_CSS_HIGHLIGHTED_FILL = "color-mix(in oklab, var(--color-secondary) 24%, var(--color-panel))";

/** @emoji 🎨 Resolves UI semantic CSS (`@ui/styling/ui.css` / `@theme`) for 2d canvas + Vello: only `var(--…)` tokens wired here — no ad-hoc palettes. */
const PUZZLE_2D_VELLO_THEME_FALLBACK_RGBA = {
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

function puzzle2dParseCssColorToRgba8888(css: string, fallback: [number, number, number, number]): [number, number, number, number] {
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

function puzzle2dProbeCssComputed(property: "color" | "backgroundColor", value: string): string {
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

function puzzle2dDefaultStylesFromElementsUiTokens(): Record<string, Puzzle2dStyle> {
  const f = PUZZLE_2D_STYLES_HEADLESS_FALLBACK;
  const c = (prop: "color" | "backgroundColor", expr: string, fb: string): string => {
    const raw = puzzle2dProbeCssComputed(prop, expr);
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
    "edge.selected": { stroke: c("color", PUZZLE_2D_CSS_COLOR_PRIMARY, f["edge.selected"].stroke ?? "#ff344f"), strokeWidth: 3 },
    handle: {
      fill: c("backgroundColor", "var(--color-base)", f.handle.fill ?? "#f7f3e3"),
      stroke: c("color", "var(--color-element)", f.handle.stroke ?? "#001117"),
      strokeWidth: 2,
    },
    "handle.highlighted": {
      fill: c("backgroundColor", PUZZLE_2D_CSS_HIGHLIGHTED_FILL, f["handle.highlighted"]?.fill ?? "#c4e4d5"),
      stroke: c("color", "var(--color-secondary)", f["handle.highlighted"]?.stroke ?? "#34d1bf"),
      strokeWidth: 1.5,
    },
    "handle.selected": {
      fill: c("backgroundColor", PUZZLE_2D_CSS_SELECTED_FILL, f["handle.selected"].fill ?? "#ff344f"),
      stroke: c("color", PUZZLE_2D_CSS_COLOR_PRIMARY, f["handle.selected"].stroke ?? "#ff344f"),
      strokeWidth: 2,
    },
    node: {
      fill: c("backgroundColor", "var(--color-panel)", f.node.fill ?? "#eeeadb"),
      stroke: c("color", "var(--color-element)", f.node.stroke ?? "#001117"),
      strokeWidth: 2,
    },
    "node.highlighted": {
      fill: c("backgroundColor", PUZZLE_2D_CSS_HIGHLIGHTED_FILL, f["node.highlighted"]?.fill ?? "#c4e4d5"),
      stroke: c("color", "var(--color-secondary)", f["node.highlighted"]?.stroke ?? "#34d1bf"),
      strokeWidth: 2,
    },
    "node.selected": {
      fill: c("backgroundColor", PUZZLE_2D_CSS_SELECTED_FILL, f["node.selected"].fill ?? "#f0c8cc"),
      stroke: c("color", PUZZLE_2D_CSS_COLOR_PRIMARY, f["node.selected"].stroke ?? "#ff344f"),
      strokeWidth: 3,
    },
  };
}

function serializePuzzle2dVelloThemeJson(): string {
  const fb = PUZZLE_2D_VELLO_THEME_FALLBACK_RGBA;
  const pc = (prop: "color" | "backgroundColor", expr: string, fall: [number, number, number, number]): number[] => {
    const raw = puzzle2dProbeCssComputed(prop, expr);
    return [...puzzle2dParseCssColorToRgba8888(raw, fall)];
  };
  const payload = {
    rasterClear: pc("backgroundColor", "var(--base)", fb.rasterClear),
    gridMinorStroke: (() => {
      const border = puzzle2dParseCssColorToRgba8888(puzzle2dProbeCssComputed("color", "var(--color-border)"), [fb.gridMinorStroke[0], fb.gridMinorStroke[1], fb.gridMinorStroke[2], 255]);
      return [border[0], border[1], border[2], fb.gridMinorStroke[3]];
    })(),
    edgeStroke: pc("color", "var(--color-muted-foreground)", fb.edgeStroke),
    edgeStrokeHovered: pc("color", "var(--color-hover-base)", fb.edgeStrokeHovered),
    edgeStrokeSelected: pc("color", PUZZLE_2D_CSS_COLOR_PRIMARY, fb.edgeStrokeSelected),
    edgeStrokeSelectionExit: pc("color", "var(--color-secondary)", fb.edgeStrokeSelectionExit),
    edgeStrokeDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 38%, transparent)", fb.edgeStrokeDisabled),
    nodeFill: pc("backgroundColor", "var(--color-panel)", fb.nodeFill),
    nodeStroke: pc("color", "var(--color-element)", fb.nodeStroke),
    nodeFillHovered: pc("backgroundColor", "var(--color-hover-panel)", fb.nodeFillHovered),
    nodeStrokeHovered: pc("color", "var(--color-hover-base)", fb.nodeStrokeHovered),
    nodeFillSelected: pc("backgroundColor", PUZZLE_2D_CSS_SELECTED_FILL, fb.nodeFillSelected),
    nodeStrokeSelected: pc("color", PUZZLE_2D_CSS_COLOR_PRIMARY, fb.nodeStrokeSelected),
    nodeFillSelectionExit: pc("backgroundColor", PUZZLE_2D_CSS_HIGHLIGHTED_FILL, fb.nodeFillSelectionExit),
    nodeStrokeSelectionExit: pc("color", "var(--color-secondary)", fb.nodeStrokeSelectionExit),
    nodeFillDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-panel) 50%, transparent)", fb.nodeFillDisabled),
    nodeStrokeDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 38%, transparent)", fb.nodeStrokeDisabled),
    indirectHandleFill: pc("backgroundColor", PUZZLE_2D_CSS_HIGHLIGHTED_FILL, fb.indirectHandleFill),
    indirectHandleStroke: pc("color", "var(--color-secondary)", fb.indirectHandleStroke),
    handleFill: pc("backgroundColor", "var(--color-base)", fb.handleFill),
    handleStroke: pc("color", "var(--color-element)", fb.handleStroke),
    handleFillHovered: pc("backgroundColor", "var(--color-hover-panel)", fb.handleFillHovered),
    handleStrokeHovered: pc("color", "var(--color-hover-base)", fb.handleStrokeHovered),
    handleFillSelected: pc("backgroundColor", PUZZLE_2D_CSS_COLOR_PRIMARY, fb.handleFillSelected),
    handleStrokeSelected: pc("color", PUZZLE_2D_CSS_COLOR_PRIMARY, fb.handleStrokeSelected),
    handleFillSelectionExit: pc("backgroundColor", PUZZLE_2D_CSS_HIGHLIGHTED_FILL, fb.handleFillSelectionExit),
    handleStrokeSelectionExit: pc("color", "var(--color-secondary)", fb.handleStrokeSelectionExit),
    handleFillDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-panel) 50%, transparent)", fb.handleFillDisabled),
    handleStrokeDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 38%, transparent)", fb.handleStrokeDisabled),
    wireStroke: pc("color", "var(--color-muted-foreground)", fb.wireStroke),
    wireStrokeHovered: pc("color", "var(--color-hover-base)", fb.wireStrokeHovered),
    wireStrokeSelected: pc("color", PUZZLE_2D_CSS_COLOR_PRIMARY, fb.wireStrokeSelected),
    wireStrokeHighlighted: pc("color", "var(--color-secondary)", fb.wireStrokeHighlighted),
    wireStrokeDisabled: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 38%, transparent)", fb.wireStrokeDisabled),
    selectionPreviewFill: pc("backgroundColor", "color-mix(in oklab, var(--color-accent) 14%, transparent)", fb.selectionPreviewFill),
    selectionPreviewStroke: pc("backgroundColor", "color-mix(in oklab, var(--color-accent) 75%, transparent)", fb.selectionPreviewStroke),
  };
  return JSON.stringify(payload);
}
//#endregion 🎨ElementsUiPuzzle2dPaint

/** @emoji 🧭 Caption anchor inside the node box (compass, origin at node center). */
export const PUZZLE_2D_NODE_TEXT_ALIGNMENTS = ["c", "e", "n", "ne", "nw", "s", "se", "sw", "w"] as const;
export type Puzzle2dNodeTextAlignment = (typeof PUZZLE_2D_NODE_TEXT_ALIGNMENTS)[number];

/** @emoji 🎯 Default: centered in the node box (`c`). */
export const PUZZLE_2D_NODE_TEXT_ALIGNMENT_DEFAULT: Puzzle2dNodeTextAlignment = "c";

/** @emoji 🔤 Default overlay caption size (layout px) when `textAutofit` is false. */
export const PUZZLE_2D_NODE_TEXT_FONT_PX_DEFAULT = 14;

/** @emoji 🔤 Default sans stack for overlay captions. */
export const PUZZLE_2D_NODE_TEXT_FONT_FAMILY_DEFAULT = "system-ui,Segoe UI,sans-serif";

/** @emoji ✅ True when `value` is a known {@link Puzzle2dNodeTextAlignment} token. */
export function isPuzzle2dNodeTextAlignment(value: string): value is Puzzle2dNodeTextAlignment {
  return (PUZZLE_2D_NODE_TEXT_ALIGNMENTS as readonly string[]).includes(value);
}

/** @emoji 🖋️ Builds a `CanvasRenderingContext2D.font` string from size and family. */
export function puzzle2dBuildCanvasFontSpec(px: number, fontFamily: string): string {
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

function preselectSnapshotsEqual(left: Puzzle2dPreselectSnapshot, right: Puzzle2dPreselectSnapshot): boolean {
  return arrayEqual(left.ids, right.ids) && arrayEqual(left.removedIds, right.removedIds);
}

/** @emoji 🧩 Compares committed selection snapshots by sorted id list. */
export function puzzle2dSelectionSnapshotsEqual(left: Puzzle2dSelectionSnapshot, right: Puzzle2dSelectionSnapshot): boolean {
  return arrayEqual(left.ids, right.ids);
}

/** @emoji 🧩 Compares preselect snapshots by ids and removedIds. */
export function puzzle2dPreselectSnapshotsEqual(left: Puzzle2dPreselectSnapshot, right: Puzzle2dPreselectSnapshot): boolean {
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

function createSelectionSnapshot(ids: Iterable<string>): Puzzle2dSelectionSnapshot {
  return { ids: sortedSelectionIds(ids) };
}

function createPreselectSnapshot(ids: Iterable<string>, removedIds: Iterable<string>): Puzzle2dPreselectSnapshot {
  return { ids: sortedSelectionIds(ids), removedIds: sortedSelectionIds(removedIds) };
}

/** @emoji 🧩 Normalizes {@link Puzzle2dSelectionSnapshot} or a bare id list into a sorted snapshot. */
export function normalizePuzzle2dSelectionProp(value: Puzzle2dSelectionSnapshot | readonly string[] | undefined): Puzzle2dSelectionSnapshot {
  if (value === undefined) {
    return { ids: [] };
  }
  if (Array.isArray(value)) {
    return createSelectionSnapshot(value);
  }
  return createSelectionSnapshot(value.ids);
}

/** @emoji 🧩 Normalizes {@link Puzzle2dPreselectSnapshot} props for controlled puzzle 2d interaction state. */
export function normalizePuzzle2dPreselectProp(value: Puzzle2dPreselectSnapshot | undefined): Puzzle2dPreselectSnapshot {
  if (value === undefined) {
    return PUZZLE_2D_PRESELECT_EMPTY;
  }
  return createPreselectSnapshot(value.ids, value.removedIds);
}

function resolveSelectionOptions(options: Puzzle2dSelectionOptions | undefined): ResolvedPuzzle2dSelectionOptions {
  return {
    method: options?.method ?? "rectangle",
    mode: options?.mode ?? "default",
    targets: {
      edges: options?.targets?.edges ?? PUZZLE_2D_SELECTION_TARGETS_DEFAULT.edges,
      handles: options?.targets?.handles ?? PUZZLE_2D_SELECTION_TARGETS_DEFAULT.handles,
      nodes: options?.targets?.nodes ?? PUZZLE_2D_SELECTION_TARGETS_DEFAULT.nodes,
    },
  };
}

function puzzle2dSelectionModeForHost(mode: Puzzle2dSelectionMode): string {
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
export function puzzle2dFixtureNodeCaption(node: Puzzle2dFixtureNodeV1): string | undefined {
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

function fixtureOptionalTextAlignment(node: Record<string, unknown>): Puzzle2dNodeTextAlignment | undefined {
  const v = node.textAlignment;
  return typeof v === "string" && isPuzzle2dNodeTextAlignment(v) ? v : undefined;
}

/** @emoji 🧾 Validates unknown JSON into {@link Puzzle2dFixtureV1} or returns null. */
export function parsePuzzle2dFixtureV1(raw: unknown): Puzzle2dFixtureV1 | null {
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
  const nodes: Puzzle2dFixtureNodeV1[] = [];
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
    const handles: Puzzle2dFixtureHandleV1[] = [];
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
      const base: Puzzle2dFixtureHandleV1 = {
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
  const edges: Puzzle2dFixtureEdgeV1[] = [];
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

/** @emoji 📌 MIME for in-app puzzle 2d fixture drags (not host filesystem file drops). */
export const PUZZLE_2D_FIXTURE_DRAG_V1_MIME = "application/x-puzzle-2d-fixture-v1";

/** @emoji 🧩 `Puzzle2dFixtureV1.meta.puzzle2dFixtureDragKind` — shelf palette drops merge one node at the pointer; any other payload replaces the scene. */
export const PUZZLE_2D_FIXTURE_DRAG_KIND_PALETTE_NODE = "palette-node";

/** @emoji 📍 Payload for puzzle 2d canvas fixture drops: scene plus pointer in canvas CSS space and mapped world coordinates. */
export interface Puzzle2dFixtureDropDetail {
  fixture: Puzzle2dFixtureV1;
  screen: { x: number; y: number };
  world: { x: number; y: number };
}

/** @emoji 📦 Serializes a validated fixture for {@link PUZZLE_2D_FIXTURE_DRAG_V1_MIME}. */
export function encodePuzzle2dFixtureForDragV1(fixture: Puzzle2dFixtureV1): string {
  return JSON.stringify(fixture);
}

/** @emoji 📋 Fallback MIME for hosts that only expose `text/plain` on drop (same JSON as {@link PUZZLE_2D_FIXTURE_DRAG_V1_MIME}). */
export const PUZZLE_2D_FIXTURE_DRAG_PLAIN_MIME = "text/plain";

/** @emoji 📤 Writes puzzle 2d fixture drag payload (custom MIME + `text/plain` fallback). */
export function setPuzzle2dFixtureDragDataTransfer(dataTransfer: DataTransfer, fixture: Puzzle2dFixtureV1): void {
  const encoded = encodePuzzle2dFixtureForDragV1(fixture);
  dataTransfer.setData(PUZZLE_2D_FIXTURE_DRAG_V1_MIME, encoded);
  dataTransfer.setData(PUZZLE_2D_FIXTURE_DRAG_PLAIN_MIME, encoded);
}

/** @emoji 📥 Reads puzzle 2d fixture drag payload from a drop `DataTransfer`. */
export function readPuzzle2dFixtureDragDataTransfer(dataTransfer: DataTransfer): Puzzle2dFixtureV1 | null {
  const custom = dataTransfer.getData(PUZZLE_2D_FIXTURE_DRAG_V1_MIME);
  if (custom.trim() !== "") {
    const parsed = decodePuzzle2dFixtureFromDragV1(custom);
    if (parsed) {
      return parsed;
    }
  }
  const plain = dataTransfer.getData(PUZZLE_2D_FIXTURE_DRAG_PLAIN_MIME);
  if (plain.trim() === "") {
    return null;
  }
  return decodePuzzle2dFixtureFromDragV1(plain);
}

/** @emoji 📋 Deep-clones a validated fixture for isolated React state (avoids mutating {@link PUZZLE_2D_PLAY_DEFAULT_FIXTURE}). */
export function clonePuzzle2dFixtureV1(fixture: Puzzle2dFixtureV1): Puzzle2dFixtureV1 {
  return JSON.parse(JSON.stringify(fixture)) as Puzzle2dFixtureV1;
}

/** @emoji 📥 Parses drag payload from {@link PUZZLE_2D_FIXTURE_DRAG_V1_MIME}. */
export function decodePuzzle2dFixtureFromDragV1(text: string): Puzzle2dFixtureV1 | null {
  let raw: unknown;
  try {
    raw = JSON.parse(text) as unknown;
  } catch {
    return null;
  }
  return parsePuzzle2dFixtureV1(raw);
}

function puzzle2dAuthoringId(prefix: string): string {
  if (typeof globalThis.crypto !== "undefined" && typeof globalThis.crypto.randomUUID === "function") {
    return `${prefix}-${globalThis.crypto.randomUUID()}`;
  }
  return `${prefix}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}

function isPaletteNodeDragFixture(fixture: Puzzle2dFixtureV1): boolean {
  if (fixture.meta?.puzzle2dFixtureDragKind === PUZZLE_2D_FIXTURE_DRAG_KIND_PALETTE_NODE) {
    return true;
  }
  if (fixture.nodes.length !== 1 || fixture.edges.length > 0) {
    return false;
  }
  const seedId = fixture.nodes[0]?.id ?? "";
  return seedId.startsWith("palette-seed-");
}

/** @emoji 🧩 When the drag payload is a palette seed, returns one node placed at the drop world point; else null so the scene should be replaced. */
export function mergePaletteNodeFromDrop(detail: Puzzle2dFixtureDropDetail): Puzzle2dFixtureNodeV1 | null {
  if (!isPaletteNodeDragFixture(detail.fixture)) {
    return null;
  }
  const template = detail.fixture.nodes[0];
  if (!template) {
    return null;
  }
  const newId = puzzle2dAuthoringId("node");
  return {
    ...template,
    handles: template.handles.map((h, i) => ({ ...h, id: `${newId}.h${i}` })),
    id: newId,
    x: detail.world.x,
    y: detail.world.y,
  };
}

/** @emoji 🧭 North-zero rectangle handle angle from type-local CAD `point` (x right, y front in kit space). */
export function puzzle2dRectangleHandleAngleFromCadPoint(x: number, y: number): number {
  return Math.atan2(-x, -y);
}

/** @emoji 🔌 Kit type connector row (CAD point + port handle kind) for catalog extraction. */
export interface KitConnectorCadRow {
  readonly point?: { readonly x: number; readonly y: number; readonly z: number };
  readonly direction?: { readonly x: number; readonly y: number; readonly z: number };
  readonly port?: { readonly handleKind?: string };
}

/** @emoji 🧲 Builds {@link NodeKind.handles} from kit connectors; keeps every distinct perimeter angle (same `handleKind` allowed). */
export function puzzle2dNodeKindHandlesFromKitConnectors(connectors: readonly KitConnectorCadRow[], defaultRadius = 3): NodeKindHandleTemplate[] {
  const seen = new Set<string>();
  const out: NodeKindHandleTemplate[] = [];
  for (const connector of connectors) {
    const handleKind = connector.port?.handleKind?.trim() ?? "";
    const point = connector.point;
    if (handleKind === "" || !point) {
      continue;
    }
    const angle = puzzle2dRectangleHandleAngleFromCadPoint(point.x, point.y);
    const key = `${handleKind}|${angle.toFixed(6)}`;
    if (seen.has(key)) {
      continue;
    }
    seen.add(key);
    out.push({ handleKind, angle, radius: defaultRadius });
  }
  return out;
}

/** @emoji 🎯 Builds fixture handles for a new node from {@link NodeKind.handles} templates. */
export function puzzle2dFixtureHandlesFromNodeKind(nodeId: string, templates: readonly NodeKindHandleTemplate[]): Puzzle2dFixtureHandleV1[] {
  return templates.map((entry, index) => ({
    id: `${nodeId}:h${index}`,
    angle: entry.angle,
    handleKind: entry.handleKind,
    ...(entry.radius !== undefined ? { radius: entry.radius } : {}),
  }));
}

/** @emoji 🖌️ Result of {@link applyBrushPlacementToFixture} (placed ids for structural-delete guards). */
export type Puzzle2dBrushPlacementApplyResult =
  | { readonly kind: "unchanged" }
  | { readonly fixture: Puzzle2dFixtureV1; readonly kind: "placed"; readonly nodeId: string; readonly edgeId: string };

/** @emoji 🖌️ Appends a brushed node and parent edge from a WASM {@link Puzzle2dBrushPlacePayload}. */
export function applyBrushPlacementToFixture(
  fixture: Puzzle2dFixtureV1,
  payload: Puzzle2dBrushPlacePayload,
  catalogs?: KindCatalogBundle,
): Puzzle2dBrushPlacementApplyResult {
  const nodeId = payload.nodeId?.trim() || `puzzle2d.brush.${crypto.randomUUID()}`;
  const handles = puzzle2dFixtureHandlesFromNodeKind(nodeId, payload.handles);
  const targetHandle = handles[payload.targetHandleIndex];
  if (!targetHandle) {
    return { kind: "unchanged" };
  }
  const edgeId = payload.edgeId?.trim() || `puzzle2d.brush.edge.${crypto.randomUUID()}`;
  const placedEdge = fixture.edges.find((e) => e.id === edgeId);
  if (fixture.nodes.some((n) => n.id === nodeId) && placedEdge?.source === payload.sourceHandleId) {
    return { kind: "placed", fixture, nodeId, edgeId };
  }
  if (fixture.edges.some((e) => e.source === payload.sourceHandleId || e.target === payload.sourceHandleId)) {
    return { kind: "unchanged" };
  }
  if (handles.some((h) => fixture.edges.some((e) => e.source === h.id || e.target === h.id))) {
    return { kind: "unchanged" };
  }
  const edge: Puzzle2dFixtureEdgeV1 = { id: edgeId, source: payload.sourceHandleId, target: targetHandle.id };
  const iconKind = payload.iconKind?.trim() || puzzle2dIconKindForBrushNodeKind(fixture, catalogs, payload.nodeKind);
  const base = { handles, id: nodeId, nodeKind: payload.nodeKind, x: payload.x, y: payload.y, ...(iconKind ? { iconKind } : {}) };
  const node: Puzzle2dFixtureNodeV1 =
    payload.shape === "rectangle"
      ? { ...base, height: payload.height ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX, shape: "rectangle", width: payload.width ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX }
      : { ...base, radius: payload.radius ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX / 2 };
  return {
    kind: "placed",
    fixture: { ...fixture, edges: [...fixture.edges, edge], nodes: [...fixture.nodes, node] },
    nodeId,
    edgeId,
  };
}

/** @emoji 🖌️ Appends a brushed node and edge to an imperative {@link Puzzle2dRenderer} scene (same rules as {@link applyBrushPlacementToFixture}). */
export function applyBrushPlacementToRendererScene(
  renderer: Puzzle2dRenderer,
  payload: Puzzle2dBrushPlacePayload,
  catalogs?: KindCatalogBundle,
): boolean {
  const sourceObj = renderer.scene.getObjectById(payload.sourceHandleId);
  if (!isPuzzle2dSceneHandleObject(sourceObj)) {
    return false;
  }
  if ([...renderer.scene.edges.values()].some((e) => e.source.id === payload.sourceHandleId || e.target.id === payload.sourceHandleId)) {
    return false;
  }
  const nodeId = payload.nodeId?.trim() || `puzzle2d.brush.${crypto.randomUUID()}`;
  if (renderer.scene.nodes.has(nodeId)) {
    return false;
  }
  const edgeId = payload.edgeId?.trim() || `puzzle2d.brush.edge.${crypto.randomUUID()}`;
  if (renderer.scene.edges.has(edgeId)) {
    return false;
  }
  const handles = puzzle2dFixtureHandlesFromNodeKind(nodeId, payload.handles);
  const targetHandle = handles[payload.targetHandleIndex];
  if (!targetHandle) {
    return false;
  }
  if (handles.some((h) => [...renderer.scene.edges.values()].some((e) => e.source.id === h.id || e.target.id === h.id))) {
    return false;
  }
  const catalogIcon = catalogs?.nodes?.find((row) => row.id === payload.nodeKind)?.icon?.trim();
  const iconKind = payload.iconKind?.trim() || (catalogIcon !== "" ? catalogIcon : undefined);
  const nodeProps =
    payload.shape === "rectangle"
      ? {
          draggable: true as const,
          height: payload.height ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
          iconKind,
          id: nodeId,
          nodeKind: payload.nodeKind,
          shape: "rectangle" as const,
          width: payload.width ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
          x: payload.x,
          y: payload.y,
        }
      : {
          draggable: true as const,
          iconKind,
          id: nodeId,
          nodeKind: payload.nodeKind,
          radius: payload.radius ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX / 2,
          shape: "circle" as const,
          x: payload.x,
          y: payload.y,
        };
  const node = newPuzzle2dNodeFromProps(nodeProps);
  renderer.scene.add(node);
  for (const handleRow of handles) {
    const handle = new Puzzle2dSceneHandle({
      angle: handleRow.angle,
      handleKind: handleRow.handleKind,
      id: handleRow.id,
      node,
      ...(handleRow.radius !== undefined ? { radius: handleRow.radius } : {}),
    });
    renderer.scene.add(handle);
  }
  const targetObj = renderer.scene.getObjectById(targetHandle.id);
  if (!isPuzzle2dSceneHandleObject(targetObj)) {
    return false;
  }
  renderer.scene.ingestWasmEdge(new Puzzle2dSceneEdge({ id: edgeId, source: sourceObj, target: targetObj }));
  renderer.markSceneDescriptorDirty();
  return true;
}
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
export function puzzle2dFitTextFontPx(ctx: CanvasTextMeasuring, text: string, maxW: number, maxH: number, minPx: number, maxPx: number, fontFamily: string = PUZZLE_2D_NODE_TEXT_FONT_FAMILY_DEFAULT): number {
  const lo = Math.max(4, minPx);
  const hi = Math.max(lo, maxPx);
  let best = lo;
  let low = lo;
  let high = hi;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = puzzle2dBuildCanvasFontSpec(mid, fontFamily);
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
export function puzzle2dEllipsisTextToWidth(ctx: CanvasTextMeasuring, text: string, maxWidth: number): string {
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
export function puzzle2dNodeTextPlacementAnchor(centerX: number, centerY: number, maxW: number, maxH: number, alignment: Puzzle2dNodeTextAlignment): { fillX: number; fillY: number; textAlign: CanvasTextAlign; textBaseline: CanvasTextBaseline } {
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

/** @emoji 🧾 Minimal 2D canvas text metrics surface for {@link puzzle2dFitTextFontPx}. */
export type CanvasTextMeasuring = Pick<CanvasRenderingContext2D, "font" | "measureText">;

/** 🧭 Builds a cubic whose control arms leave/arrive along circle normals (radial), not along handle tangents. */
export function computeEdgeBezier(sourceHandle: Puzzle2dSceneHandle, targetHandle: Puzzle2dSceneHandle): CubicBezierCurve {
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
export function computeWireBezier(sourceHandle: Puzzle2dSceneHandle, endWorld: Point): CubicBezierCurve {
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
/** 🧱 Base retained scene object with scene identity and shared flags. */
export class Puzzle2dSceneObject {
  draggable: boolean;
  highlighted: boolean;
  parent: Puzzle2dScene | null = null;
  selected: boolean;
  style: string | null;
  userData: Record<string, unknown>;
  visible: boolean;

  protected renderer: Puzzle2dRenderer | null = null;

  constructor(
    public readonly id: string,
    options: Puzzle2dSceneObjectOptions,
  ) {
    this.draggable = options.draggable ?? false;
    this.highlighted = options.highlighted ?? false;
    this.selected = options.selected ?? false;
    this.style = options.style ?? null;
    this.userData = { ...(options.userData ?? {}) };
    this.visible = options.visible ?? true;
  }

  get kind(): Puzzle2dSceneObjectKind {
    throw new Error("Puzzle2dSceneObject.kind must be implemented by subclasses.");
  }

  attachRenderer(renderer: Puzzle2dRenderer | null): void {
    this.renderer = renderer;
  }

  dispose(): void {
    this.parent?.remove(this);
  }
}

/** 🟠 Puzzle 2d node: circle (radius) or axis-aligned rectangle (width × height) centered at (x,y). */
export class Puzzle2dSceneNode extends Puzzle2dSceneObject {
  handles: Puzzle2dSceneHandle[] = [];
  height: number;
  radius: number;
  shape: "circle" | "rectangle";
  text: string | null;
  /** @emoji 🏷️ Runtime icon string forwarded to WASM detail LOD (`typst:` / `$…`, `emoji:`, `data:` / raster data URLs, baked catalog id, or inline SVG). */
  iconKind: string | null;
  /** @emoji 📏 When true, {@link Puzzle2dRenderer} scales overlay text to the node interior (always drawn at node center). */
  textAutofit: boolean;
  /** @emoji 🧭 When not autofitting, anchors single-line caption inside the node-centered box. */
  textAlignment: Puzzle2dNodeTextAlignment;
  /** @emoji 🔤 CSS font family string for overlay captions. */
  textFontFamily: string;
  /** @emoji 🔤 Font size in layout px when not autofitting. */
  textFontSize: number;
  width: number;
  x: number;
  y: number;
  /** @emoji 🌳 When true, {@link computePuzzle2dGraphObservationSnapshot} treats each {@link Edge} as parent {@link Edge.source} → child {@link Edge.target} along node ids. */
  root: boolean;
  /** @emoji 🧩 Semantic node-kind id forwarded to WASM for catalog defaults and compatibility. */
  nodeKind: string;

  constructor(options: Puzzle2dSceneNodeOptions) {
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
    this.textAlignment = options.textAlignment ?? PUZZLE_2D_NODE_TEXT_ALIGNMENT_DEFAULT;
    this.textFontFamily = typeof options.textFontFamily === "string" && options.textFontFamily.trim() !== "" ? options.textFontFamily.trim() : PUZZLE_2D_NODE_TEXT_FONT_FAMILY_DEFAULT;
    const rawSize = options.textFontSize;
    this.textFontSize = typeof rawSize === "number" && Number.isFinite(rawSize) && rawSize > 0 ? rawSize : PUZZLE_2D_NODE_TEXT_FONT_PX_DEFAULT;
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

  get kind(): Puzzle2dSceneObjectKind {
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

  attachHandle(handle: Puzzle2dSceneHandle): void {
    if (this.handles.includes(handle)) {
      return;
    }
    handle.node = this;
    this.handles.push(handle);
  }

  detachHandle(handle: Puzzle2dSceneHandle): void {
    this.handles = this.handles.filter((candidate) => candidate !== handle);
  }
}

/** 🟣 Tangent handle anchored to a node boundary at a polar angle. */
export class Puzzle2dSceneHandle extends Puzzle2dSceneObject {
  angle: number;
  /** @emoji 🎨 CSS `#…` fill override for the WASM host; `null` uses catalog / theme only. */
  color: string | null;
  /** @emoji 🔗 Semantic kind for ordered link compatibility on the host (JSON `handleKind`). */
  handleKind: string;
  node: Puzzle2dSceneNode;
  radius: number;

  constructor(options: Puzzle2dSceneHandleOptions) {
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

  get kind(): Puzzle2dSceneObjectKind {
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
export class Puzzle2dSceneEdge extends Puzzle2dSceneObject {
  /** @emoji 🧩 Semantic edge-kind id forwarded to WASM. */
  edgeKind: string;
  source: Puzzle2dSceneHandle;
  target: Puzzle2dSceneHandle;

  constructor(options: Puzzle2dSceneEdgeOptions) {
    super(options.id, options);
    this.source = options.source;
    this.target = options.target;
    const ek = typeof options.edgeKind === "string" ? options.edgeKind.trim() : "";
    this.edgeKind = ek;
  }

  get kind(): Puzzle2dSceneObjectKind {
    return "edge";
  }

  get curve(): CubicBezierCurve {
    return computeEdgeBezier(this.source, this.target);
  }

  setEndpoints(sourceHandle: Puzzle2dSceneHandle, targetHandle: Puzzle2dSceneHandle): this {
    this.source = sourceHandle;
    this.target = targetHandle;
    return this;
  }
}

/** 🧵 Transient cubic from one {@link Handle} to another handle or a free world point (in‑progress link drag). */
export class Puzzle2dSceneWire extends Puzzle2dSceneObject {
  endX: number | null;
  endY: number | null;
  source: Puzzle2dSceneHandle;
  target: Puzzle2dSceneHandle | null;
  /** @emoji 🧩 Semantic wire-kind id forwarded to WASM. */
  wireKind: string;

  constructor(options: Puzzle2dSceneWireOptions) {
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

  get kind(): Puzzle2dSceneObjectKind {
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

  setAnchors(sourceHandle: Puzzle2dSceneHandle, targetHandle: Puzzle2dSceneHandle | null, endWorld?: Point | null): this {
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

/** @emoji 🧪 Kind-based scene object guards (Vite may load duplicate module copies where `instanceof` fails). */
function isPuzzle2dSceneNodeObject(obj: Puzzle2dSceneObject | undefined | null): obj is Puzzle2dSceneNode {
  return obj?.kind === "node";
}

function isPuzzle2dSceneHandleObject(obj: Puzzle2dSceneObject | undefined | null): obj is Puzzle2dSceneHandle {
  return obj?.kind === "handle";
}

function isPuzzle2dSceneEdgeObject(obj: Puzzle2dSceneObject | undefined | null): obj is Puzzle2dSceneEdge {
  return obj?.kind === "edge";
}

function isPuzzle2dSceneWireObject(obj: Puzzle2dSceneObject | undefined | null): obj is Puzzle2dSceneWire {
  return obj?.kind === "wire";
}
//#endregion 🔖Objects

type Puzzle2dNodeObject = Puzzle2dSceneNode;
type Puzzle2dHandleObject = Puzzle2dSceneHandle;
type Puzzle2dEdgeObject = Puzzle2dSceneEdge;
type Puzzle2dWireObject = Puzzle2dSceneWire;

//#region 🔖Scene
/** 🧭 Retained scene catalog owning nodes, handles, edges, and wires by stable id. */
export class Puzzle2dScene {
  readonly edges = new Map<string, Puzzle2dSceneEdge>();
  readonly handles = new Map<string, Puzzle2dSceneHandle>();
  readonly nodes = new Map<string, Puzzle2dSceneNode>();
  readonly wires = new Map<string, Puzzle2dSceneWire>();

  constructor(private renderer: Puzzle2dRenderer | null = null) {}

  setRenderer(renderer: Puzzle2dRenderer | null): void {
    this.renderer = renderer;
    for (const object of this.getAllObjects()) {
      object.attachRenderer(renderer);
    }
  }

  add(object: Puzzle2dSceneObject): this {
    if (isPuzzle2dSceneNodeObject(object)) {
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
      this.renderer?.markDirty({ observeGraph: true });
      return this;
    }

    if (isPuzzle2dSceneHandleObject(object)) {
      if (!this.nodes.has(object.node.id)) {
        this.add(object.node);
      }
      this.handles.set(object.id, object);
      object.parent = this;
      object.attachRenderer(this.renderer);
      object.node.attachHandle(object);
      this.renderer?.markDirty({ observeGraph: true });
      return this;
    }

    if (isPuzzle2dSceneWireObject(object)) {
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
      this.renderer?.markDirty({ observeGraph: true });
      return this;
    }

    if (isPuzzle2dSceneEdgeObject(object)) {
      const edge = object as Edge;
      const existed = this.edges.has(edge.id);
      this.edges.set(edge.id, edge);
      edge.parent = this;
      edge.attachRenderer(this.renderer);
      if (!existed) {
        this.renderer?.emit("edgeCreate", { id: edge.id, source: edge.source.id, target: edge.target.id });
      }
      this.renderer?.markDirty({ observeGraph: true });
      return this;
    }

    this.edges.set(object.id, object as Edge);
    object.parent = this;
    object.attachRenderer(this.renderer);
    this.renderer?.emit("edgeCreate", { id: object.id, source: (object as Edge).source.id, target: (object as Edge).target.id });
    this.renderer?.markDirty({ observeGraph: true });
    return this;
  }

  /** @emoji 🔗 Inserts a WASM‑drained edge without emitting {@link Puzzle2dEventMap.edgeCreate} (the renderer applies that once per drain row). */
  ingestWasmEdge(edge: Puzzle2dSceneEdge): this {
    this.edges.set(edge.id, edge);
    edge.parent = this;
    edge.attachRenderer(this.renderer);
    this.renderer?.markDirty({ observeGraph: true });
    return this;
  }

  remove(object: Puzzle2dSceneObject): this {
    if (isPuzzle2dSceneNodeObject(object)) {
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
      this.renderer?.markDirty({ observeGraph: true });
      return this;
    }

    if (isPuzzle2dSceneHandleObject(object)) {
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
      this.renderer?.markDirty({ observeGraph: true });
      return this;
    }

    if (isPuzzle2dSceneWireObject(object)) {
      this.renderer?.emitSceneDeleteEvent("wireDestroy", { id: object.id });
      this.wires.delete(object.id);
      object.parent = null;
      object.attachRenderer(null);
      this.renderer?.markDirty({ observeGraph: true });
      return this;
    }

    this.renderer?.emitSceneDeleteEvent("edgeDelete", { id: object.id });
    this.edges.delete(object.id);
    object.parent = null;
    object.attachRenderer(null);
    this.renderer?.markDirty({ observeGraph: true });
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

  getObjectById(id: string): Puzzle2dSceneObject | undefined {
    return this.nodes.get(id) ?? this.handles.get(id) ?? this.edges.get(id) ?? this.wires.get(id);
  }

  getAllObjects(): Puzzle2dSceneObject[] {
    return [...this.nodes.values(), ...this.handles.values(), ...this.edges.values(), ...this.wires.values()];
  }
}
//#endregion 🔖Scene

//#region 🔖DirectedGraphObservation
/** @emoji 🧮 Immutable snapshot for {@link Puzzle2dRenderer} hierarchy callbacks (roots + directed reachability along {@link Edge.source}→{@link Edge.target}). */
export interface Puzzle2dGraphObservationSnapshot {
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

function puzzle2dGraphNodeSig(node: Puzzle2dSceneNode): string {
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

function puzzle2dGraphEdgeSig(edge: Puzzle2dSceneEdge): string {
  return JSON.stringify({
    source: edge.source.id,
    id: edge.id,
    selected: edge.selected,
    style: edge.style,
    target: edge.target.id,
    visible: edge.visible,
  });
}

function puzzle2dGraphWireSig(wire: Puzzle2dSceneWire): string {
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
export function computePuzzle2dGraphObservationSnapshot(scene: Puzzle2dScene): Puzzle2dGraphObservationSnapshot {
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
    nodeSigById.set(node.id, puzzle2dGraphNodeSig(node));
  }
  const edgeSigById = new Map<string, string>();
  for (const edge of scene.edges.values()) {
    edgeSigById.set(edge.id, puzzle2dGraphEdgeSig(edge));
  }
  const wireSigById = new Map<string, string>();
  for (const wire of scene.wires.values()) {
    wireSigById.set(wire.id, puzzle2dGraphWireSig(wire));
  }
  return { childEdgeIds, childNodeIds, edgeSigById, nodeSigById, parentEdgeIds, rootIds, wireSigById };
}
//#endregion 🔖DirectedGraphObservation

/** @emoji 🧯 Normalizes WebGPU errors for `data-puzzle2d-surface-failure` (E2E + local debugging). */
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

function puzzle2dAbbreviateCaption(raw: string, maxChars: number): string {
  return raw.length <= maxChars ? raw : `${raw.slice(0, Math.max(1, maxChars - 1))}…`;
}

/** @emoji 🏷️ Abbreviated node caption for the text overlay canvas, or null when the LOD band hides node labels. */
export function puzzle2dTextOverlayCaptionForLod(raw: string, lod: Puzzle2dDrawLodKind, iconKind: string | null): string | null {
  const t = raw.trim();
  if (t === "") {
    return null;
  }
  if (lod === "minimap" || lod === "overview") {
    return null;
  }
  if (lod === "compact" || lod === "normal") {
    return puzzle2dAbbreviateCaption(t, 8);
  }
  if (lod === "detail") {
    return puzzle2dAbbreviateCaption(t, (iconKind?.trim() ?? "") !== "" ? 8 : 10);
  }
  if (lod === "micro") {
    return puzzle2dAbbreviateCaption(t, 12);
  }
  return null;
}

/** @emoji 🏷️ Abbreviated handle caption for the text overlay canvas, or null when the LOD band hides handle labels. */
export function puzzle2dHandleOverlayCaptionForLod(raw: string, lod: Puzzle2dDrawLodKind): string | null {
  const t = raw.trim();
  if (t === "") {
    return null;
  }
  if (lod !== "detail" && lod !== "micro") {
    return null;
  }
  return puzzle2dAbbreviateCaption(t, lod === "detail" ? 6 : 8);
}

/** @emoji 🧩 Resolves a handle-kind catalog label for overlay captions. */
export function puzzle2dHandleKindOverlayLabel(handleKind: string, catalogs: KindCatalogBundle): string {
  return puzzle2dKindCatalogRowName(handleKind, catalogs.handles);
}

/** @emoji 🧩 Resolves a node-kind catalog label for panels and overlays. */
export function puzzle2dNodeKindOverlayLabel(nodeKind: string, catalogs: KindCatalogBundle): string {
  return puzzle2dKindCatalogRowName(nodeKind, catalogs.nodes);
}

/** @emoji 🔀 Merges {@link DEFAULT_KIND_CATALOG_BUNDLE} with `meta.kindCatalogs` on a parsed fixture. */
export function puzzle2dFixtureMergedKindCatalogs(fixture: Puzzle2dFixtureV1): KindCatalogBundle {
  return mergeKindCatalogBundleByRowId(DEFAULT_KIND_CATALOG_BUNDLE, fixtureMetaKindCatalogBundle(fixture) ?? {});
}

/** @emoji 🏷️ Icon for a brushed node: catalog `icon`, else the first fixture peer with the same `nodeKind`. */
export function puzzle2dIconKindForBrushNodeKind(fixture: Puzzle2dFixtureV1, catalogs: KindCatalogBundle | undefined, nodeKind: string): string | undefined {
  const kindId = nodeKind.trim();
  if (kindId === "") {
    return undefined;
  }
  const fromCatalog = catalogs?.nodes?.find((row) => row.id === kindId)?.icon?.trim();
  if (fromCatalog) {
    return fromCatalog;
  }
  const peer = fixture.nodes.find((node) => node.nodeKind === kindId && node.iconKind?.trim());
  return peer?.iconKind?.trim();
}

function puzzle2dCatalogNameLooksLikeI18nKey(name: string): boolean {
  return /^semio\.(sketchpad|metabolism)\./.test(name.trim());
}

/** @emoji 🏷️ Resolves a kind-catalog row `name` for a stable kind id (never returns raw semio i18n keys). */
export function puzzle2dKindCatalogRowName(kindId: string, rows: readonly { readonly id: string; readonly name: string }[] | undefined): string {
  const id = kindId.trim();
  if (id === "") {
    return "";
  }
  for (const row of rows ?? []) {
    if (row.id === id) {
      const name = row.name?.trim() ?? "";
      if (name !== "" && !puzzle2dCatalogNameLooksLikeI18nKey(name)) {
        return name;
      }
      break;
    }
  }
  if (id === BUILTIN_PORT_HANDLE_KIND) {
    return "Port";
  }
  return puzzle2dHumanizeKindIdTail(id);
}

function puzzle2dHumanizeKindIdTail(kindId: string): string {
  const colon = kindId.lastIndexOf(":");
  if (colon >= 0 && colon < kindId.length - 1) {
    const tail = kindId.slice(colon + 1).trim();
    if (tail !== "" && !/^[0-9a-f-]{8}-/i.test(tail)) {
      return tail.replace(/_/g, " ");
    }
  }
  const icon = kindId.match(/^capsule_(.+)$/i);
  if (icon) {
    return icon[1]!.replace(/_/g, " ");
  }
  return "Item";
}

/** @emoji 🏷️ Primary tree/panel label for a fixture node (caption, then kind name — never the instance id). */
export function puzzle2dFixtureNodeDisplayLabel(node: Puzzle2dFixtureNodeV1, catalogs: KindCatalogBundle): string {
  const caption = puzzle2dFixtureNodeCaption(node);
  if (caption) {
    return caption;
  }
  const kind = node.nodeKind?.trim();
  if (kind) {
    const kindLabel = puzzle2dNodeKindOverlayLabel(kind, catalogs);
    if (kindLabel) {
      return kindLabel;
    }
  }
  const icon = node.iconKind?.trim();
  if (icon) {
    return puzzle2dHumanizeKindIdTail(icon);
  }
  return "Node";
}

/** @emoji 🏷️ Secondary line for a fixture node (kind name when caption is the primary label). */
export function puzzle2dFixtureNodeDisplayDescription(node: Puzzle2dFixtureNodeV1, catalogs: KindCatalogBundle): string | undefined {
  const caption = puzzle2dFixtureNodeCaption(node);
  const kind = node.nodeKind?.trim();
  if (!kind) {
    return undefined;
  }
  const kindLabel = puzzle2dNodeKindOverlayLabel(kind, catalogs);
  if (!kindLabel || kindLabel === caption) {
    return undefined;
  }
  return kindLabel;
}

function puzzle2dFixtureHandleRoleSuffix(handleId: string): string | undefined {
  const colon = handleId.lastIndexOf(":");
  if (colon < 0 || colon >= handleId.length - 1) {
    return undefined;
  }
  const tail = handleId.slice(colon + 1).trim();
  return tail === "link" ? undefined : tail.replace(/_/g, " ");
}

/** @emoji 🏷️ Primary tree/panel label for a fixture handle (kind name, optional role suffix — not the instance id). */
export function puzzle2dFixtureHandleDisplayLabel(handle: Puzzle2dFixtureHandleV1, catalogs: KindCatalogBundle): string {
  const kindLabel = puzzle2dHandleKindOverlayLabel(handle.handleKind ?? BUILTIN_PORT_HANDLE_KIND, catalogs);
  const suffix = puzzle2dFixtureHandleRoleSuffix(handle.id);
  return suffix ? `${kindLabel} · ${suffix}` : kindLabel;
}

/** @emoji 🏷️ Resolves a handle endpoint id (`nodeId:role` or catalog handle id) to a panel label. */
export function puzzle2dFixtureHandleEndpointDisplayLabel(handleId: string, fixture: Puzzle2dFixtureV1, catalogs: KindCatalogBundle): string {
  for (const node of fixture.nodes) {
    const handle = node.handles.find((row) => row.id === handleId);
    if (handle) {
      const nodeLabel = puzzle2dFixtureNodeDisplayLabel(node, catalogs);
      const kindLabel = puzzle2dHandleKindOverlayLabel(handle.handleKind ?? BUILTIN_PORT_HANDLE_KIND, catalogs);
      return kindLabel === "Port" || kindLabel === nodeLabel ? `${nodeLabel} · link` : `${nodeLabel} · ${kindLabel}`;
    }
  }
  const colon = handleId.lastIndexOf(":");
  if (colon > 0) {
    const nodeId = handleId.slice(0, colon);
    const node = fixture.nodes.find((row) => row.id === nodeId);
    if (node) {
      const nodeLabel = puzzle2dFixtureNodeDisplayLabel(node, catalogs);
      const role = handleId.slice(colon + 1).trim();
      return role === "link" ? `${nodeLabel} · link` : `${nodeLabel} · ${role.replace(/_/g, " ")}`;
    }
  }
  const catalogLabel = puzzle2dHandleKindOverlayLabel(handleId, catalogs);
  if (catalogLabel !== handleId && catalogLabel !== "Item") {
    return catalogLabel;
  }
  return puzzle2dHumanizeKindIdTail(handleId);
}

/** @emoji 🏷️ Primary tree/panel label for an edge (endpoint labels, not the edge uuid). */
export function puzzle2dFixtureEdgeDisplayLabel(edge: Puzzle2dFixtureEdgeV1, fixture: Puzzle2dFixtureV1, catalogs: KindCatalogBundle): string {
  const source = puzzle2dFixtureHandleEndpointDisplayLabel(edge.source, fixture, catalogs);
  const target = puzzle2dFixtureHandleEndpointDisplayLabel(edge.target, fixture, catalogs);
  return `${source} → ${target}`;
}

/** @emoji 🏷️ Resolves any puzzle 2d scene object id to a panel label when possible. */
export function puzzle2dFixtureObjectDisplayLabel(objectId: string, fixture: Puzzle2dFixtureV1, catalogs: KindCatalogBundle): string {
  const node = fixture.nodes.find((row) => row.id === objectId);
  if (node) {
    return puzzle2dFixtureNodeDisplayLabel(node, catalogs);
  }
  const edge = fixture.edges.find((row) => row.id === objectId);
  if (edge) {
    return puzzle2dFixtureEdgeDisplayLabel(edge, fixture, catalogs);
  }
  for (const row of fixture.nodes) {
    const handle = row.handles.find((h) => h.id === objectId);
    if (handle) {
      return puzzle2dFixtureHandleDisplayLabel(handle, catalogs);
    }
  }
  return puzzle2dHumanizeKindIdTail(objectId);
}

//#region 🔖Renderer
/** @emoji 🧿 WASM drain rows that never change scene topology (skip {@link Puzzle2dRenderer.enqueuePuzzle2dGraphObservationFlush}). */
const PUZZLE2D_DRAIN_SKIP_GRAPH_OBSERVATION = new Set([
  "camera",
  "hover",
  "select",
  "preselect",
  "preselectCancel",
  "nodeMove",
  "nodeDragEnd",
  "linkCompatibleNodes",
  "linkTargetRing",
  "proximityConnect",
  "indirectConnect",
  "brushPreview",
  "brushCandidates",
  "brushPlace",
]);

let puzzle2dDebugInvalidateCount = 0;
let puzzle2dDebugRenderCount = 0;

/** @emoji 📊 Test/diagnostic counters for invalidate/render churn ([DEBUG] perf validation). */
export function puzzle2dDebugPerfCounters(): { readonly invalidate: number; readonly render: number } {
  return { invalidate: puzzle2dDebugInvalidateCount, render: puzzle2dDebugRenderCount };
}

/** @emoji 🧹 Resets {@link puzzle2dDebugPerfCounters} tallies. */
export function puzzle2dResetDebugPerfCounters(): void {
  puzzle2dDebugInvalidateCount = 0;
  puzzle2dDebugRenderCount = 0;
}

/** 🎛️ Slim imperative shell: DOM/RAF, one {@link BoardSession} (WASM `BoardHost` + optional GPU), JSON scene sync, and event drains mirroring WASM onto the JS scene graph for React/tests. */
export class Puzzle2dRenderer {
  static activeRenderer: Puzzle2dRenderer | null = null;

  readonly camera: CameraState = { ...DEFAULT_CAMERA };
  readonly scene: Puzzle2dScene;
  readonly session: BoardSession;
  /** @emoji 🔗 Edge ids created by the WASM host (link gesture) until the same id appears in React `children`; merged into the descriptor passed to {@link syncPuzzle2dScene}. */
  readonly wasmHostAuthoredEdgeIds = new Set<string>();
  /** @emoji 🔗 Endpoint ids for each {@link Puzzle2dRenderer.wasmHostAuthoredEdgeIds} entry so merge can rebuild the descriptor if the scene edge was removed transiently (e.g. handle purge ordering). */
  readonly wasmHostAuthoredLinkByEdgeId = new Map<string, { source: string; target: string }>();
  /** @emoji 🚫 Edge/node ids removed by user Delete; declarative resync must not resurrect them on the source pane before fixture commits. */
  private readonly authoritativeStructuralSuppressions = new Set<string>();

  private batchDepth = 0;
  /** @emoji 🔇 While >0, {@link Puzzle2dScene.remove} does not emit delete events (dispose / JSX resync). */
  private suppressSceneDeleteEvents = 0;
  /** @emoji 🎮 While >0, {@link emitSceneDeleteEvent} reaches play/fixture listeners (user Delete only); resync drains stay scene-local. */
  private structuralDeleteFixtureMirrorDepth = 0;
  /** @emoji 🔁 Nesting depth for {@link Puzzle2dRenderer.render}; defers {@link Puzzle2dRenderer.invalidate} so ResizeObserver / layout cannot re-enter WASM during `renderFrame` (`borrow_fail`). */
  private renderPipelineDepth = 0;
  /** @emoji ⛓️ Tracks async WASM session borrows such as {@link BoardSession.attach_canvas} so sync probes like `gpuReady()` do not re-enter the same `RefCell`. */
  private wasmSessionBorrowDepth = 0;
  /** @emoji 🧷 Tracks `pushSceneToWasmDriver` + {@link BoardSession.renderFrame} where `device.poll` may synchronously re-enter JS while WASM still borrows `BoardSession`. */
  private wasmGpuFrameDepth = 0;

  /** @emoji 🚧 True while wasm-bindgen holds `&mut BoardSession`; any other JS→wasm call on this session must defer (see commit 379 + follow-up). */
  private wasmSessionCallBlockedForReentry(): boolean {
    return this.wasmSessionBorrowDepth > 0 || this.wasmGpuFrameDepth > 0;
  }

  /** @emoji 🔇 Runs `fn` without {@link Puzzle2dEventMap.nodeDelete} / edge / wire delete emissions (internal teardown or descriptor resync). */
  runWithoutSceneDeleteEvents(fn: () => void): void {
    this.suppressSceneDeleteEvents += 1;
    try {
      fn();
    } finally {
      this.suppressSceneDeleteEvents -= 1;
    }
  }

  /** @emoji 🎮 True when imperative delete events should update {@link Puzzle2dFixtureV1} authorship (user Delete), not WASM/scene resync. */
  mirrorsStructuralDeletesToFixture(): boolean {
    return this.structuralDeleteFixtureMirrorDepth > 0;
  }

  /** @emoji 🎮 Runs `fn` while structural delete events propagate to play/fixture listeners. */
  withFixtureStructuralDeleteMirror(fn: () => void): void {
    this.structuralDeleteFixtureMirrorDepth += 1;
    try {
      fn();
    } finally {
      this.structuralDeleteFixtureMirrorDepth -= 1;
    }
  }

  /** @emoji 📣 Forwards structural delete events to play/fixture listeners unless suppressed or disposed. */
  emitSceneDeleteEvent<TKey extends "nodeDelete" | "edgeDelete" | "wireDestroy">(name: TKey, payload: Puzzle2dEventMap[TKey]): void {
    if (this.suppressSceneDeleteEvents > 0 || this.isDisposed) {
      return;
    }
    if (this.structuralDeleteFixtureMirrorDepth <= 0 && name !== "wireDestroy") {
      return;
    }
    this.emit(name, payload);
    const structural: Puzzle2dStructureDeletePayload =
      name === "nodeDelete" ? { kind: "node", id: payload.id } : name === "edgeDelete" ? { kind: "edge", id: payload.id } : { kind: "wire", id: payload.id };
    this.pruneHostDeclarativeStructuralDelete(structural);
    puzzle2dBroadcastStructuralRemove(this, structural);
  }

  /** @emoji 🪢 Records how many edges the declarative host expects (0 = no gate on WASM push). */
  setDeclarativeSceneEdgeExpectation(count: number): void {
    this.declarativeSceneEdgeExpectation = Math.max(0, Math.floor(count));
  }

  /** @emoji 📌 Stores the declarative host descriptor used to recover edges after WASM structural drains. */
  rememberHostDeclarativeSceneDescriptor(descriptor: Puzzle2dSceneDescriptor): void {
    this.hostDeclarativeSceneDescriptor = this.descriptorWithoutAuthoritativeRemovals(descriptor);
  }

  /** @emoji 🚫 True when user Delete removed this structural id and JSX/WASM resync must not bring it back. */
  isAuthoritativeStructuralRemovalSuppressed(kind: "edge" | "node", id: string): boolean {
    return this.authoritativeStructuralSuppressions.has(`${kind}:${id}`);
  }

  private markAuthoritativeStructuralRemoval(payload: Puzzle2dStructureDeletePayload): void {
    if (payload.kind === "wire") {
      return;
    }
    this.authoritativeStructuralSuppressions.add(`${payload.kind}:${payload.id}`);
  }

  /** @emoji 🔄 Clears structural suppressions for fixture ids so authoritative sync can restore edges (brush slot occupancy). */
  clearAuthoritativeStructuralSuppressionsForDescriptor(descriptor: Puzzle2dSceneDescriptor): void {
    for (const node of descriptor.nodes) {
      this.authoritativeStructuralSuppressions.delete(`node:${node.id}`);
    }
    for (const edge of descriptor.edges) {
      this.authoritativeStructuralSuppressions.delete(`edge:${edge.id}`);
    }
  }

  /** @emoji 🧹 Strips user-deleted ids from a declarative descriptor before host remember / edge ensure. */
  descriptorWithoutAuthoritativeRemovals(descriptor: Puzzle2dSceneDescriptor): Puzzle2dSceneDescriptor {
    const nodes = descriptor.nodes.filter((node) => !this.isAuthoritativeStructuralRemovalSuppressed("node", node.id));
    const nodeIds = new Set(nodes.map((node) => node.id));
    const handles = descriptor.handles.filter((handle) => nodeIds.has(handle.nodeId));
    const handleIds = new Set(handles.map((handle) => handle.id));
    const edges = descriptor.edges.filter(
      (edge) => !this.isAuthoritativeStructuralRemovalSuppressed("edge", edge.id) && handleIds.has(edge.source) && handleIds.has(edge.target),
    );
    const wires = descriptor.wires.filter((wire) => {
      if (!handleIds.has(wire.source)) {
        return false;
      }
      return wire.target === undefined || handleIds.has(wire.target);
    });
    const nextNodes = nodes.map((node) => ({
      ...node,
      handles: node.handles.filter((handle) => handleIds.has(handle.id)),
    }));
    return { edges, handles, nodes: nextNodes, wires };
  }

  /** @emoji ✂️ Drops a structural id from {@link Puzzle2dRenderer.hostDeclarativeSceneDescriptor} after an authoritative delete so zoom drains do not resurrect it. */
  pruneHostDeclarativeStructuralDelete(payload: Puzzle2dStructureDeletePayload): void {
    this.markAuthoritativeStructuralRemoval(payload);
    const host = this.hostDeclarativeSceneDescriptor;
    if (!host) {
      return;
    }
    if (payload.kind === "edge") {
      const edges = host.edges.filter((edge) => edge.id !== payload.id);
      if (edges.length === host.edges.length) {
        return;
      }
      const next = this.descriptorWithoutAuthoritativeRemovals({ ...host, edges });
      this.hostDeclarativeSceneDescriptor = next;
      this.setDeclarativeSceneEdgeExpectation(next.edges.length);
      return;
    }
    if (payload.kind === "wire") {
      const wires = host.wires.filter((wire) => wire.id !== payload.id);
      if (wires.length === host.wires.length) {
        return;
      }
      this.rememberHostDeclarativeSceneDescriptor({ ...host, wires });
      return;
    }
    const removedHandleIds = new Set(host.handles.filter((handle) => handle.nodeId === payload.id).map((handle) => handle.id));
    for (const edge of host.edges) {
      if (removedHandleIds.has(edge.source) || removedHandleIds.has(edge.target)) {
        this.markAuthoritativeStructuralRemoval({ kind: "edge", id: edge.id });
      }
    }
    const nodes = host.nodes.filter((node) => node.id !== payload.id);
    if (nodes.length === host.nodes.length) {
      return;
    }
    const handles = host.handles.filter((handle) => handle.nodeId !== payload.id);
    const edges = host.edges.filter((edge) => !removedHandleIds.has(edge.source) && !removedHandleIds.has(edge.target));
    const nextNodes = nodes.map((node) => ({
      ...node,
      handles: node.handles.filter((handle) => handle.nodeId !== payload.id),
    }));
    const next = this.descriptorWithoutAuthoritativeRemovals({ ...host, edges, handles, nodes: nextNodes });
    this.hostDeclarativeSceneDescriptor = next;
    this.setDeclarativeSceneEdgeExpectation(next.edges.length);
  }

  /** @emoji 🪢 Re-syncs imperative edges from {@link Puzzle2dRenderer.rememberHostDeclarativeSceneDescriptor} when WASM drains cleared them but the host still expects edges. */
  reconcileHostDeclarativeSceneEdges(): boolean {
    const descriptor = this.hostDeclarativeSceneDescriptor;
    if (!descriptor || descriptor.edges.length === 0) {
      return false;
    }
    if (this.scene.edges.size >= descriptor.edges.length) {
      return false;
    }
    const missingEdges = descriptor.edges.filter((edge) => !this.scene.edges.has(edge.id));
    if (missingEdges.length === 0) {
      return false;
    }
    this.resetDeclarativeSceneSyncFingerprint();
    syncPuzzle2dScene(this, descriptor);
    this.setDeclarativeSceneEdgeExpectation(descriptor.edges.length);
    return true;
  }

  /** @emoji 🔔 Marks the WASM descriptor cache stale after scene graph or selection chrome changes. */
  markSceneDescriptorDirty(): void {
    this.sceneDescriptorEpoch += 1;
    this.textOverlayContentEpoch += 1;
    this.lastPushedDescriptorJson = null;
    this.lastSceneWasmFingerprint = null;
    this.lastPushedSceneDescriptorEpoch = -1;
  }

  /** @emoji 🧷 Records the declarative descriptor fingerprint after a successful {@link syncPuzzle2dScene} (not cleared by {@link Puzzle2dRenderer.markSceneDescriptorDirty}). */
  rememberDeclarativeSceneSyncFingerprint(descriptor: Puzzle2dSceneDescriptor): void {
    this.lastSyncedDescriptorFingerprint = puzzle2dSceneDescriptorFingerprint(descriptor);
  }

  /** @emoji 🧹 Clears declarative sync coalescing after the secondary host unmounts and wipes {@link Puzzle2dRenderer.scene}. */
  resetDeclarativeSceneSyncFingerprint(): void {
    this.lastSyncedDescriptorFingerprint = null;
  }

  /** @emoji ⏭️ Skips imperative scene graph work when the declarative descriptor fingerprint is unchanged. */
  skipSceneSyncIfDescriptorUnchanged(descriptor: Puzzle2dSceneDescriptor): boolean {
    if (descriptor.edges.length > 0 && this.scene.edges.size !== descriptor.edges.length) {
      this.lastSyncedDescriptorFingerprint = null;
      return false;
    }
    const fingerprint = puzzle2dSceneDescriptorFingerprint(descriptor);
    if (this.lastSyncedDescriptorFingerprint === fingerprint) {
      if (!puzzle2dSceneDescriptorMatchesScene(this, descriptor)) {
        this.lastSyncedDescriptorFingerprint = null;
        return false;
      }
      this.syncInteractionChrome();
      return true;
    }
    return false;
  }

  /** @emoji 🔇 Applies a peer pane selection commit without emitting {@link Puzzle2dEventMap.select}. */
  applySelectionFromPeerSilent(ids: readonly string[]): void {
    if (this.isDisposed) {
      return;
    }
    const nextSnapshot = createSelectionSnapshot(new Set(ids));
    const selectionUnchanged = puzzle2dSelectionSnapshotsEqual(nextSnapshot, this.selectionStore.getSnapshot());
    const hadPreselect = !preselectSnapshotsEqual(this.preselectStore.getSnapshot(), PUZZLE_2D_PRESELECT_EMPTY);
    if (hadPreselect) {
      this.preselectIds = new Set();
      this.preselectRemovedIds = new Set();
      this.preselectStore.setSnapshot(PUZZLE_2D_PRESELECT_EMPTY, preselectSnapshotsEqual);
    }
    if (!selectionUnchanged) {
      this.selectionIds = new Set(ids);
      this.selectionStore.setSnapshot(nextSnapshot, (left, right) => arrayEqual(left.ids, right.ids));
    }
    if (!selectionUnchanged || hadPreselect) {
      this.syncSelectionChromeToSceneObjectsIfNeeded();
    }
    if (selectionUnchanged && !hadPreselect) {
      return;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      return;
    }
    try {
      if (!selectionUnchanged) {
        this.session.setSelectionIdsJsonSilent(JSON.stringify(nextSnapshot.ids));
      }
      if (hadPreselect) {
        this.session.setPreselectStateJsonSilent(JSON.stringify(PUZZLE_2D_PRESELECT_EMPTY));
      }
    } catch (err) {
      console.error("[DEBUG] applySelectionFromPeerSilent failed", err);
    }
    this.scheduleInputInvalidate();
  }

  /** @emoji 🔇 Applies a peer pane node drag without emitting {@link Puzzle2dEventMap.nodeMove}. */
  applyNodePositionSilent(nodeId: string, x: number, y: number): void {
    if (this.isDisposed) {
      return;
    }
    const node = this.scene.nodes.get(nodeId);
    if (!node) {
      return;
    }
    if (nearlyEqual(node.x, x) && nearlyEqual(node.y, y)) {
      return;
    }
    this.suppressSceneToWasmPush = true;
    try {
      node.setPosition(x, y);
      this.lastNodeAuthoringPositionById.set(nodeId, { x, y });
    } finally {
      this.suppressSceneToWasmPush = false;
    }
    this.pendingIncrementalNodeMoves.set(nodeId, { x, y });
    this.bumpTextOverlayGeometryEpoch();
    this.invalidate();
  }

  /** @emoji 🔇 Applies a peer pane structural removal without emitting delete events. */
  applyStructuralRemoveSilent(payload: Puzzle2dStructureDeletePayload): void {
    if (this.isDisposed) {
      return;
    }
    let graphMutated = false;
    this.runWithoutSceneDeleteEvents(() => {
      if (payload.kind === "wire") {
        const wire = this.scene.wires.get(payload.id);
        if (wire) {
          this.scene.remove(wire);
          graphMutated = true;
        }
        return;
      }
      if (payload.kind === "edge") {
        const edge = this.scene.edges.get(payload.id);
        this.clearWasmHostAuthorshipForEdge(payload.id);
        if (edge) {
          this.scene.remove(edge);
          graphMutated = true;
        }
        return;
      }
      const node = this.scene.nodes.get(payload.id);
      if (node) {
        this.scene.remove(node);
        graphMutated = true;
      }
    });
    if (!graphMutated) {
      return;
    }
    this.pruneHostDeclarativeStructuralDelete(payload);
    this.bumpWasmHostSceneMergeResyncEpoch();
    this.markSceneDescriptorDirty();
    this.invalidate();
  }

  /** @emoji ✂️ Commits an authoritative structural delete to peers and the host declarative snapshot (used when WASM drains skip scene delete events). */
  private commitAuthoritativeStructuralDelete(payload: Puzzle2dStructureDeletePayload): void {
    this.pruneHostDeclarativeStructuralDelete(payload);
    puzzle2dBroadcastStructuralRemove(this, payload);
  }

  /** @emoji 🧹 Clears {@link Puzzle2dRenderer.applyNodePositionFromProps} caches so declarative fixture commits always win on every pane. */
  clearNodeAuthoringPositionCache(): void {
    this.lastNodeAuthoringPositionById.clear();
  }

  /** @emoji 🔗 Whether this renderer still participates in multi-pane authoring peer sync. */
  authoringPeerActive(): boolean {
    return !this.isDisposed;
  }
  /** @emoji 💾 Last `gpuReady` snapshot; used while {@link Puzzle2dRenderer.wasmGpuFrameDepth} is non-zero to avoid `RefCell` conflicts with in-flight `renderFrame`. */
  private cachedWasmGpuReady = false;
  private cameraStore = new SnapshotStore<CameraState>({ ...DEFAULT_CAMERA });
  private drawLodStore = new SnapshotStore<Puzzle2dDrawLodKind>(resolvePuzzle2dLodLabelFromThresholds(DEFAULT_CAMERA.zoom, DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS));
  private canvas: HTMLCanvasElement | null;
  private dpr = 1;
  private emitter = new TypedEmitter<Puzzle2dEventMap>();
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
  private selectionOptions: ResolvedPuzzle2dSelectionOptions;
  private selectionStore = new SnapshotStore<Puzzle2dSelectionSnapshot>({ ids: [] });
  private preselectIds = new Set<string>();
  private preselectRemovedIds = new Set<string>();
  private preselectStore = new SnapshotStore<Puzzle2dPreselectSnapshot>(PUZZLE_2D_PRESELECT_EMPTY);
  private styles = new Map<string, Puzzle2dStyle>(Object.entries(DEFAULT_STYLES));
  private gpuSurfaceErrorDetail = "";
  private gpuSurfaceInitPromise: Promise<void> | null = null;
  private gpuSurfacePresentedFrame = false;
  private gpuSurfaceUnavailable = false;
  private lastPushedDescriptorJson: string | null = null;
  private lastSceneWasmFingerprint: string | null = null;
  private lastSyncedDescriptorFingerprint: string | null = null;
  /** @emoji 🪢 Declarative host edge count (play fixture); blocks WASM descriptor push while the imperative scene is still edgeless. */
  private declarativeSceneEdgeExpectation = 0;
  /** @emoji 📌 Latest declarative scene from the host fixture prop; recovers edges after WASM resync drains. */
  private hostDeclarativeSceneDescriptor: Puzzle2dSceneDescriptor | null = null;
  private sceneDescriptorEpoch = 0;
  private lastPushedSceneDescriptorEpoch = -1;
  private lastVelloThemeJson = "";
  private textOverlayContentEpoch = 0;
  private readonly textOverlayLayoutCache = new Map<string, { readonly line: string; readonly fontPx: number }>();
  private textOverlayPainted = false;
  private lastOverlayCameraX = Number.NaN;
  private lastOverlayCameraY = Number.NaN;
  private lastOverlayCameraZoom = Number.NaN;
  private lastOverlayWidth = -1;
  private lastOverlayHeight = -1;
  private lastOverlayDpr = -1;
  private lastOverlayLod: Puzzle2dDrawLodKind | "" = "";
  private lastOverlaySelection: Puzzle2dSelectionSnapshot | null = null;
  private lastOverlayPreselect: Puzzle2dPreselectSnapshot | null = null;
  private lastOverlayContentEpoch = -1;
  private lastOverlayVelloThemeJson = "";
  private lastDescriptorPushDeferred = false;
  private wasmDeferHadStructuralMutation = false;
  private lastAppliedChromeSelectedIds = new Set<string>();
  private lastAppliedChromeHighlightedIds = new Set<string>();
  private pointerGestureCameraAtStart: CameraState | null = null;
  private scheduledSelectEmitRafId: number | null = null;
  private pendingSelectEmitSnapshot: Puzzle2dSelectionSnapshot | null = null;
  private kindCompatJson = "[]";
  private kindCatalogsBundle: KindCatalogBundle = DEFAULT_KIND_CATALOG_BUNDLE;
  private kindCatalogsJson = serializeKindCatalogBundle(DEFAULT_KIND_CATALOG_BUNDLE);
  private lastPushedKindCatalogsJson: string | null = null;
  private wasmHostSceneMergeResyncStore = new SnapshotStore<number>(0);
  private lastNodeAuthoringPositionById = new Map<string, { x: number; y: number }>();
  private pendingIncrementalNodeMoves = new Map<string, { x: number; y: number }>();
  private wasmPushSceneDrainAlreadyApplied = false;
  private viewportWheelEmitRafId: number | null = null;
  private wheelCameraReactSyncTimeoutId: ReturnType<typeof setTimeout> | null = null;
  private wheelZoomGestureActive = false;
  /** @emoji 📷 While true, {@link Puzzle2dHostSubtree} must not apply a lagging controlled `camera` prop over the live WASM viewport. */
  private wasmViewportLeading = false;
  private wheelFlushRafId: number | null = null;
  private pendingWheelScreen: { sx: number; sy: number; deltaY: number } | null = null;
  private inputInvalidateRafId: number | null = null;
  /** @emoji 🖌️ Replays brush pointer leave/move after {@link Puzzle2dRenderer.wasmSessionCallBlockedForReentry} drops an input event. */
  private pendingBrushWasmFlush: "leave" | "move" | null = null;
  /** @emoji 🖌️ Deferred {@link Puzzle2dRenderer.setBrushSession} JSON when WASM reentry blocks the mirror push. */
  private pendingBrushSessionJsonForWasm: string | null = null;
  private lastPushedCameraX = Number.NaN;
  private lastPushedCameraY = Number.NaN;
  private lastPushedCameraZoom = Number.NaN;
  private lastEmittedHoverId: string | null | undefined;
  private lastPushedWidth = -1;
  private lastPushedHeight = -1;
  private lastPushedDpr = -1;
  private suppressSceneToWasmPush = false;
  private graphObservationFlushPending = false;
  private lastGraphObservation: Puzzle2dGraphObservationSnapshot | null = null;
  private width = 1;
  private height = 1;
  private textOverlayCanvas: HTMLCanvasElement | null = null;

  private lodZoomThresholds: Puzzle2dLodZoomThresholds = { ...DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS };
  private automaticLod = true;
  private forcedDrawLodLabel: Puzzle2dDrawLodKind | undefined = undefined;
  private gridSnapEnabled = false;
  private gridFactor = DEFAULT_PUZZLE_2D_GRID_FACTOR;
  private activeTool: Puzzle2dActiveTool = "select";
  private brushFlushDistance = DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX;
  private brushNodeKindWeights: Record<string, number> = {};
  private brushHandleKindWeights: Record<string, number> = {};
  private lastBrushKindWeightsJsonForWasm: string | null = null;
  private brushNodeSize = DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX;
  private lastLodThresholdsJsonForWasm: string | null = null;
  private lastActiveToolForWasm: Puzzle2dActiveTool | null = null;
  private lastBrushFlushDistanceForWasm: number | null = null;
  private lastBrushNodeSizeForWasm: number | null = null;
  private lastAutomaticLodForWasm: boolean | null = null;
  private lastForcedDrawLodLabelForWasm: string | null = null;
  private lastGridSnapEnabledForWasm: boolean | null = null;
  private lastGridFactorForWasm: number | null = null;

  worldRasterTiling: WorldRasterTilingKind;

  constructor(
    options: {
      canvas?: HTMLCanvasElement | null;
      renderMode?: RenderMode;
      selection?: Puzzle2dSelectionOptions;
      worldRasterTiling?: WorldRasterTilingKind;
      lodZoomThresholds?: Puzzle2dLodZoomThresholds;
      /** @emoji 📶 When true (default), WASM draw LOD follows camera zoom; when false, {@link lod} pins the tier when set. */
      automaticLod?: boolean;
      /** @emoji 📶 Pinned draw LOD when `automaticLod` is false. */
      lod?: Puzzle2dDrawLodKind;
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
      : { ...DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS };
    this.gridSnapEnabled = options.gridSnapEnabled ?? false;
    const gf = options.gridFactor;
    this.gridFactor = typeof gf === "number" && Number.isFinite(gf) && gf > 0 && gf <= 1e6 ? gf : DEFAULT_PUZZLE_2D_GRID_FACTOR;
    this.automaticLod = options.automaticLod ?? true;
    const optLod = options.lod;
    this.forcedDrawLodLabel = !this.automaticLod && optLod !== undefined && isPuzzle2dDrawLodKind(optLod) ? optLod : undefined;
    if (this.automaticLod) {
      this.forcedDrawLodLabel = undefined;
    }
    this.scene = new Puzzle2dScene(this);
    this.session = new BoardSession();
    const initialSel = this.selectionOptions;
    this.session.setSelectionOptions(initialSel.method, puzzle2dSelectionModeForHost(initialSel.mode), initialSel.targets.nodes, initialSel.targets.edges, initialSel.targets.handles);
    this.session.setHandleLinkCompatJson(this.kindCompatJson);
    try {
      this.session.setKindCatalogsJson(this.kindCatalogsJson);
    } catch (err) {
      console.error("[DEBUG] setKindCatalogsJson failed during Puzzle2dRenderer init", err);
    }
    this.lastPushedKindCatalogsJson = this.kindCatalogsJson;
    this.applyWasmDrainToScene(this.session.drainEventsJson(), { silentStructuralRemoves: true });
    this.attachCanvasListeners();
    if (this.canvas) {
      (this.canvas as Puzzle2dCanvasElement).__puzzle2dRenderer = this;
      const initialWidth = this.canvas.clientWidth || this.canvas.width || 1;
      const initialHeight = this.canvas.clientHeight || this.canvas.height || 1;
      this.setSize(initialWidth, initialHeight, globalThis.devicePixelRatio || 1);
    }
    this.lastGraphObservation = computePuzzle2dGraphObservationSnapshot(this.scene);
    puzzle2dRegisterAuthoringPeer(this);
  }

  readonly renderMode: RenderMode;

  /** @emoji 🔗 Subscribes to an epoch bumped when WASM event drains mutate edges/nodes so the React host can re-merge {@link Puzzle2dRenderer.wasmHostAuthoredEdgeIds} into JSX sync without waiting for `children` identity changes. */
  subscribeWasmHostSceneMergeResync = (listener: () => void): (() => void) => this.wasmHostSceneMergeResyncStore.subscribe(listener);

  /** @emoji 🔗 Snapshot for {@link Puzzle2dRenderer.subscribeWasmHostSceneMergeResync} (use with `useSyncExternalStore`). */
  getWasmHostSceneMergeResyncEpoch = (): number => this.wasmHostSceneMergeResyncStore.getSnapshot();

  get selection(): {
    getSnapshot: () => Puzzle2dSelectionSnapshot;
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

  getSelectionSnapshot = (): Puzzle2dSelectionSnapshot => this.selectionStore.getSnapshot();

  get preselection(): {
    getSnapshot: () => Puzzle2dPreselectSnapshot;
    subscribe: (listener: () => void) => () => void;
  } {
    return {
      getSnapshot: this.getPreselectSnapshot,
      subscribe: this.subscribePreselect,
    };
  }

  subscribePreselect = (listener: () => void): (() => void) => this.preselectStore.subscribe(listener);

  getPreselectSnapshot = (): Puzzle2dPreselectSnapshot => this.preselectStore.getSnapshot();

  /** @emoji ✅ Replaces the active selection set and syncs `selected` flags on scene objects. */
  setSelectionIds(ids: Iterable<string>): void {
    const nextSnapshot = createSelectionSnapshot(new Set(ids));
    if (puzzle2dSelectionSnapshotsEqual(nextSnapshot, this.selectionStore.getSnapshot())) {
      return;
    }
    this.updateSelection(ids, true);
    if (this.wasmSessionCallBlockedForReentry()) {
      return;
    }
    try {
      this.session.setSelectionIdsJson(JSON.stringify(nextSnapshot.ids));
    } catch (err) {
      console.error("[DEBUG] setSelectionIdsJson failed", err);
    }
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.invalidate();
  }

  /** @emoji 🔇 Controlled sync: updates WASM + JS selection without emitting `select`. */
  setSelectionIdsSilent(ids: Iterable<string>): void {
    const nextSnapshot = createSelectionSnapshot(new Set(ids));
    if (puzzle2dSelectionSnapshotsEqual(nextSnapshot, this.selectionStore.getSnapshot())) {
      return;
    }
    this.updateSelection(ids, false);
    if (this.wasmSessionCallBlockedForReentry()) {
      return;
    }
    try {
      this.session.setSelectionIdsJsonSilent(JSON.stringify(nextSnapshot.ids));
    } catch (err) {
      console.error("[DEBUG] setSelectionIdsJsonSilent failed", err);
    }
    this.invalidate();
  }

  /** @emoji 👁️ Controlled sync: mirrors area-select preview chrome on this canvas without emitting `preselect`. */
  syncPreselectionSilent(snapshot: Puzzle2dPreselectSnapshot): void {
    const normalized = normalizePuzzle2dPreselectProp(snapshot);
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
    this.invalidate();
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
    this.invalidate();
  }

  getSelectionOptions(): ResolvedPuzzle2dSelectionOptions {
    return { ...this.selectionOptions, targets: { ...this.selectionOptions.targets } };
  }

  /** @emoji 🎯 Updates area-selection behavior for left-button drag gestures. */
  setSelectionOptions(options: Puzzle2dSelectionOptions): void {
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
    this.session.setSelectionOptions(next.method, puzzle2dSelectionModeForHost(next.mode), next.targets.nodes, next.targets.edges, next.targets.handles);
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
  setLodZoomThresholds(next: Puzzle2dLodZoomThresholds): void {
    const c: Puzzle2dLodZoomThresholds = {
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

  /** @emoji 📐 Positive multiplier for LOD world grid steps on the WASM host (see {@link DEFAULT_PUZZLE_2D_GRID_FACTOR}). */
  setGridFactor(next: number): void {
    const n = typeof next === "number" && Number.isFinite(next) && next > 0 && next <= 1e6 ? next : DEFAULT_PUZZLE_2D_GRID_FACTOR;
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
  private lastBrushSessionJsonForWasm: string | null = null;

  /** @emoji 🖌️ Mirrors shared brush slot preview into this pane's WASM host. */
  setBrushSession(snapshot: Puzzle2dBrushSessionSnapshot | null): void {
    const json = snapshot
      ? JSON.stringify({
          sourceHandleId: snapshot.sourceHandleId ?? "",
          candidates: snapshot.candidates,
          index: snapshot.candidateIndex,
          preview: snapshot.preview,
        })
      : "";
    if (json === this.lastBrushSessionJsonForWasm) {
      return;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.pendingBrushSessionJsonForWasm = json;
      this.invalidated = true;
      return;
    }
    this.pushBrushSessionJsonToWasm(json);
  }

  private pushBrushSessionJsonToWasm(json: string): void {
    try {
      if (json.length === 0) {
        this.session.clearBrushSessionJson();
      } else {
        this.session.setBrushSessionJson(json);
      }
      this.applyWasmDrainToScene(this.session.drainEventsJson());
      this.lastBrushSessionJsonForWasm = json;
    } catch (err) {
      console.error("[DEBUG] setBrushSession failed", err);
    }
    this.markDirty();
  }

  private flushPendingBrushSessionMirror(): void {
    const pending = this.pendingBrushSessionJsonForWasm;
    if (pending === null || this.wasmSessionCallBlockedForReentry()) {
      return;
    }
    if (pending === this.lastBrushSessionJsonForWasm) {
      this.pendingBrushSessionJsonForWasm = null;
      return;
    }
    this.pendingBrushSessionJsonForWasm = null;
    this.pushBrushSessionJsonToWasm(pending);
  }

  /** @emoji 📡 Pushes the imperative scene graph to WASM immediately (brush place and structural edits). */
  pushAuthoritativeSceneToWasmHost(): void {
    this.ensureImperativeSceneForWasmPush();
    this.pushAuthoritativeDescriptorToWasmSession();
    this.invalidate();
  }

  /** @emoji 🧩 Hydrates the imperative scene from the host descriptor when the canvas mounted before fixture sync. */
  private ensureImperativeSceneForWasmPush(): void {
    const descriptor = this.hostDeclarativeSceneDescriptor;
    if (!descriptor || this.scene.nodes.size >= descriptor.nodes.length) {
      return;
    }
    this.resetDeclarativeSceneSyncFingerprint();
    syncPuzzle2dScene(this, descriptor);
    puzzle2dEnsureSceneEdgesFromDescriptor(this, descriptor);
  }

  /** @emoji 🔗 Mirrors a host {@link Puzzle2dLinkSessionSnapshot} into WASM for cross-surface link preview. */
  setLinkSession(snapshot: Puzzle2dLinkSessionSnapshot | null): void {
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

  /** @emoji 📶 When true (default), WASM draw LOD follows camera zoom; when false, optional {@link Puzzle2dRenderer.setForcedDrawLod} pins the tier. */
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
    this.markSceneDescriptorDirty();
    this.markDirty();
  }

  /** @emoji 📶 Pins WASM draw LOD when {@link Puzzle2dRenderer.setAutomaticLod} is false; pass undefined to follow zoom bands. */
  setForcedDrawLod(next: Puzzle2dDrawLodKind | undefined): void {
    const norm = this.automaticLod || next === undefined ? undefined : isPuzzle2dDrawLodKind(next) ? next : undefined;
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

  private effectiveDrawLodLabel(): Puzzle2dDrawLodKind {
    if (!this.automaticLod && this.forcedDrawLodLabel !== undefined) {
      return this.forcedDrawLodLabel;
    }
    return resolvePuzzle2dLodLabelFromThresholds(this.camera.zoom, this.lodZoomThresholds);
  }

  /** @emoji 🖌️ Select or brush viewport tool on the WASM host. */
  setActiveTool(tool: Puzzle2dActiveTool): void {
    if (this.activeTool === tool) {
      return;
    }
    this.activeTool = tool;
    this.pendingBrushWasmFlush = null;
    this.lastActiveToolForWasm = null;
    this.markDirty();
  }

  /** @emoji 📐 Brush slot offset along handle outward normal (world units). */
  setBrushFlushDistance(distance: number): void {
    const next = Number.isFinite(distance) && distance >= 0 ? distance : DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX;
    if (nearlyEqual(this.brushFlushDistance, next)) {
      return;
    }
    this.brushFlushDistance = next;
    this.lastBrushFlushDistanceForWasm = null;
    this.markDirty();
  }

  /** @emoji 🎚️ Per-kind brush suggestion weights pushed to WASM (node + handle groups). */
  setBrushKindWeights(nodeWeights: Readonly<Record<string, number>>, handleWeights: Readonly<Record<string, number>>): void {
    this.brushNodeKindWeights = { ...nodeWeights };
    this.brushHandleKindWeights = { ...handleWeights };
    this.lastBrushKindWeightsJsonForWasm = null;
    this.markDirty();
  }

  /** @emoji 🖌️ Selects the active brush candidate by catalog index. */
  setBrushCandidateIndex(index: number): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    try {
      this.session.brushSetCandidateIndex(index);
      this.applyWasmDrainToScene(this.session.drainEventsJson());
      this.scheduleInputInvalidate();
    } catch (err) {
      console.error("[DEBUG] brushSetCandidateIndex failed", err);
    }
  }

  /** @emoji 📐 Brush preview node span in world units. */
  setBrushNodeSize(size: number): void {
    const next = Number.isFinite(size) && size > 0 ? size : DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX;
    if (nearlyEqual(this.brushNodeSize, next)) {
      return;
    }
    this.brushNodeSize = next;
    this.lastBrushNodeSizeForWasm = null;
    this.markDirty();
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
  applyNodePositionFromProps(nodeId: string, x: number, y: number, instance: Puzzle2dSceneNode): void {
    const last = this.lastNodeAuthoringPositionById.get(nodeId);
    const propsUnchangedSinceLastSync = last !== undefined && last.x === x && last.y === y;
    const sceneMatchesDescriptor = instance.x === x && instance.y === y;
    if (propsUnchangedSinceLastSync && !sceneMatchesDescriptor) {
      return;
    }
    instance.setPosition(x, y);
    this.lastNodeAuthoringPositionById.set(nodeId, { x, y });
  }

  /** @emoji 🧹 Drops cached declarative coordinates for a removed node id (see {@link Puzzle2dRenderer.applyNodePositionFromProps}). */
  evictNodeAuthoringPosition(nodeId: string): void {
    this.lastNodeAuthoringPositionById.delete(nodeId);
  }

  getCameraSnapshot = (): CameraState => this.cameraStore.getSnapshot();

  subscribeCamera = (listener: () => void): (() => void) => this.cameraStore.subscribe(listener);

  getDrawLodSnapshot = (): Puzzle2dDrawLodKind => this.drawLodStore.getSnapshot();

  subscribeDrawLod = (listener: () => void): (() => void) => this.drawLodStore.subscribe(listener);

  on<TKey extends keyof Puzzle2dEventMap>(name: TKey, handler: (payload: Puzzle2dEventMap[TKey]) => void): () => void {
    return this.emitter.on(name, handler);
  }

  emit<TKey extends keyof Puzzle2dEventMap>(name: TKey, payload: Puzzle2dEventMap[TKey]): void {
    this.emitter.emit(name, payload);
  }

  /** @emoji 📣 Notifies React hosts of the committed viewport camera after pan/zoom gestures complete. */
  private emitPublicCameraChange(): void {
    this.emitter.emit("camera", { ...this.camera });
  }

  /** @emoji ⏱️ Coalesces committed `select` events to one React update per animation frame (click / marquee end). */
  private scheduleCommittedSelectEmit(snapshot: Puzzle2dSelectionSnapshot): void {
    this.pendingSelectEmitSnapshot = snapshot;
    if (this.renderMode === "headless-test") {
      this.flushCommittedSelectEmit();
      return;
    }
    if (this.scheduledSelectEmitRafId !== null) {
      return;
    }
    const requestFrame = globalThis.requestAnimationFrame?.bind(globalThis);
    if (!requestFrame) {
      this.flushCommittedSelectEmit();
      return;
    }
    this.scheduledSelectEmitRafId = requestFrame(() => {
      this.scheduledSelectEmitRafId = null;
      this.flushCommittedSelectEmit();
    });
  }

  private flushCommittedSelectEmit(): void {
    const snapshot = this.pendingSelectEmitSnapshot;
    if (snapshot === null) {
      return;
    }
    this.pendingSelectEmitSnapshot = null;
    this.emitter.emit("select", snapshot);
  }

  /** @emoji 📷 Mirrors WASM host camera into the JS store without `camera` events (wheel hot path). */
  private syncCameraFromWasmHostSilent(): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      return;
    }
    let raw: { x: number; y: number; zoom: number };
    try {
      raw = JSON.parse(this.session.cameraJson()) as { x: number; y: number; zoom: number };
    } catch {
      return;
    }
    const nextZoom = clamp(raw.zoom, MIN_ZOOM, MAX_ZOOM);
    const next: CameraState = { x: raw.x, y: raw.y, zoom: nextZoom };
    if (pointsEqual(this.camera, next) && nearlyEqual(this.camera.zoom, next.zoom)) {
      return;
    }
    this.camera.x = next.x;
    this.camera.y = next.y;
    this.camera.zoom = next.zoom;
    this.cameraStore.setSnapshot({ ...this.camera }, (left, right) => pointsEqual(left, right) && nearlyEqual(left.zoom, right.zoom));
    this.wasmViewportLeading = true;
    this.bumpTextOverlayGeometryEpoch();
  }

  /** @emoji 📷 Clears {@link Puzzle2dRenderer.wasmViewportLeading} once the host `camera` prop matches the live viewport; returns whether host sync may proceed. */
  clearWasmViewportLeadingIfHostCameraMatches(host: CameraState): boolean {
    if (!this.wasmViewportLeading) {
      return true;
    }
    if (pointsEqual(this.camera, host) && nearlyEqual(this.camera.zoom, host.zoom)) {
      this.wasmViewportLeading = false;
      return true;
    }
    return false;
  }

  /** @emoji 🔍 Mirrors {@link Puzzle2dRenderer.wheelZoomGestureActive} to WASM (skip grid rebuild while zooming). */
  private pushWheelZoomActiveToWasmSession(): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      return;
    }
    try {
      this.session.setWheelZoomActive(this.wheelZoomGestureActive);
    } catch (err) {
      console.error("[DEBUG] setWheelZoomActive failed", err);
    }
  }

  /** @emoji ⏱️ Defers React `camera` commits until wheel zoom idles (GPU still renders every coalesced frame). */
  private scheduleDeferredPublicCameraEmit(): void {
    if (this.renderMode === "headless-test") {
      this.wheelZoomGestureActive = false;
      this.pushWheelZoomActiveToWasmSession();
      this.emitPublicCameraChange();
      return;
    }
    this.wheelZoomGestureActive = true;
    this.pushWheelZoomActiveToWasmSession();
    if (this.wheelCameraReactSyncTimeoutId !== null) {
      clearTimeout(this.wheelCameraReactSyncTimeoutId);
    }
    this.wheelCameraReactSyncTimeoutId = setTimeout(() => {
      this.wheelCameraReactSyncTimeoutId = null;
      this.wheelZoomGestureActive = false;
      this.pushWheelZoomActiveToWasmSession();
      this.emitPublicCameraChange();
      this.invalidate();
    }, 120);
  }

  /** @emoji 🖱️ Applies coalesced wheel deltas before GPU/text overlay so the frame camera matches {@link Puzzle2dRenderer.syncGpuFrame}. */
  private ensurePendingWheelFlushedBeforeFrame(): void {
    if (this.pendingWheelScreen === null) {
      return;
    }
    if (this.wheelFlushRafId !== null && globalThis.cancelAnimationFrame) {
      globalThis.cancelAnimationFrame(this.wheelFlushRafId);
      this.wheelFlushRafId = null;
    }
    this.flushPendingWheelScreen();
  }

  /** @emoji 🖱️ Flushes coalesced wheel deltas once per animation frame. */
  private flushPendingWheelScreen(): void {
    const pending = this.pendingWheelScreen;
    if (!pending || !this.canvas) {
      this.pendingWheelScreen = null;
      return;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.pendingWheelScreen = null;
    this.session.wheelScreen(pending.sx, pending.sy, pending.deltaY);
    this.wasmViewportLeading = true;
    this.syncCameraFromWasmHostSilent();
    this.wasmPushSceneDrainAlreadyApplied = true;
    this.scheduleDeferredPublicCameraEmit();
    this.invalidate();
  }

  private schedulePendingWheelFlush(): void {
    if (this.wheelFlushRafId !== null) {
      return;
    }
    const requestFrame = globalThis.requestAnimationFrame?.bind(globalThis);
    if (!requestFrame) {
      this.flushPendingWheelScreen();
      return;
    }
    this.wheelFlushRafId = requestFrame(() => {
      this.wheelFlushRafId = null;
      this.flushPendingWheelScreen();
    });
  }

  private enqueuePuzzle2dGraphObservationFlush(): void {
    if (this.graphObservationFlushPending) {
      return;
    }
    this.graphObservationFlushPending = true;
    queueMicrotask(() => {
      this.graphObservationFlushPending = false;
      this.flushPuzzle2dGraphObservation();
    });
  }

  private flushPuzzle2dGraphObservation(): void {
    const prev = this.lastGraphObservation;
    if (prev === null) {
      this.lastGraphObservation = computePuzzle2dGraphObservationSnapshot(this.scene);
      return;
    }
    const next = computePuzzle2dGraphObservationSnapshot(this.scene);
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
        this.enqueuePuzzle2dGraphObservationFlush();
      }
      if (this.batchDepth === 0 && this.invalidated) {
        this.invalidate();
      }
    }
  }

  defineStyle(name: string, style: Puzzle2dStyle): void {
    this.styles.set(name, style);
    this.markDirty();
  }

  getStyle(name: string | null, fallbackName: string): Puzzle2dStyle {
    return this.styles.get(name ?? fallbackName) ?? this.styles.get(fallbackName) ?? {};
  }

  /** @emoji 📐 Updates layout size; returns false when width/height/dpr are unchanged (skips overlay invalidation). */
  setSize(width: number, height: number, dpr = this.dpr): boolean {
    const nextWidth = Math.max(1, Math.round(width));
    const nextHeight = Math.max(1, Math.round(height));
    const nextDpr = Math.max(1, dpr);
    if (this.width === nextWidth && this.height === nextHeight && this.dpr === nextDpr) {
      return false;
    }
    this.width = nextWidth;
    this.height = nextHeight;
    this.dpr = nextDpr;
    if (this.canvas) {
      const nextW = Math.round(this.width * this.dpr);
      const nextH = Math.round(this.height * this.dpr);
      if (this.canvas.width !== nextW || this.canvas.height !== nextH) {
        this.canvas.width = nextW;
        this.canvas.height = nextH;
      }
    }
    this.markDirty();
    return true;
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
    this.wasmViewportLeading = false;
    this.emitter.emit("camera", { ...this.camera });
    this.pushCameraToWasmSession(z);
  }

  /** @emoji 🔇 Controlled-host camera sync without `camera` events or WASM drain parsing. */
  setCameraSilent(x: number, y: number, zoom: number): void {
    const z = clamp(zoom, MIN_ZOOM, MAX_ZOOM);
    const next: CameraState = { x, y, zoom: z };
    if (pointsEqual(this.camera, next) && nearlyEqual(this.camera.zoom, next.zoom)) {
      return;
    }
    this.camera.x = next.x;
    this.camera.y = next.y;
    this.camera.zoom = next.zoom;
    this.cameraStore.setSnapshot({ ...this.camera }, (left, right) => pointsEqual(left, right) && nearlyEqual(left.zoom, right.zoom));
    if (!this.wasmHostOwnsViewportCamera()) {
      this.pushCameraToWasmSession(z);
    }
  }

  private pushCameraToWasmSession(zoom: number): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    if (this.wasmHostOwnsViewportCamera()) {
      return;
    }
    this.session.setCamera(this.camera.x, this.camera.y, zoom);
    this.lastPushedCameraX = this.camera.x;
    this.lastPushedCameraY = this.camera.y;
    this.lastPushedCameraZoom = zoom;
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

  /** @emoji 🏷️ Marks the 2D text overlay stale after node/handle geometry moves without a full scene descriptor push. */
  private bumpTextOverlayGeometryEpoch(): void {
    this.textOverlayContentEpoch += 1;
  }

  /** @emoji 🖌️ Requests a repaint; pass `observeGraph: true` when scene topology or authored props changed. */
  markDirty(options?: { readonly observeGraph?: boolean }): void {
    this.textOverlayContentEpoch += 1;
    this.textOverlayLayoutCache.clear();
    this.invalidated = true;
    if (this.batchDepth > 0) {
      return;
    }
    if (options?.observeGraph) {
      this.enqueuePuzzle2dGraphObservationFlush();
    }
    this.invalidate();
  }

  /** @emoji ⏱️ Coalesces hover/pan/zoom repaints to one frame while WASM still receives every pointer/wheel event. */
  scheduleInputInvalidate(): void {
    if (this.isDisposed) {
      return;
    }
    if (this.session.defersDescriptorSyncFromJs()) {
      this.invalidate();
      return;
    }
    if (this.renderMode === "headless-test") {
      this.invalidate();
      return;
    }
    this.invalidated = true;
    if (this.rafId !== null || this.inputInvalidateRafId !== null) {
      return;
    }
    const requestFrame = globalThis.requestAnimationFrame?.bind(globalThis);
    if (!requestFrame) {
      this.invalidate();
      return;
    }
    this.inputInvalidateRafId = requestFrame(() => {
      this.inputInvalidateRafId = null;
      this.invalidate();
    });
  }

  invalidate(): void {
    if (this.isDisposed) {
      return;
    }
    puzzle2dDebugInvalidateCount += 1;
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
    puzzle2dDebugRenderCount += 1;
    this.renderPipelineDepth += 1;
    const frameDelta = this.lastRenderTimestamp === null ? 0 : timestamp - this.lastRenderTimestamp;
    this.lastRenderTimestamp = timestamp;
    this.invalidated = false;
    try {
      this.ensurePendingWheelFlushedBeforeFrame();
      let syncedGpuThisFrame = false;
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
              console.error("[DEBUG] Puzzle2dRenderer GPU surface init failed", err);
              this.gpuSurfaceErrorDetail = summarizeRasterSurfaceFailure(err);
              this.gpuSurfaceUnavailable = true;
              this.cachedWasmGpuReady = false;
              this.gpuSurfaceInitPromise = null;
              this.markDirty();
            });
        }
        gpuReady = this.readGpuReady();
        if (gpuReady) {
          syncedGpuThisFrame = this.syncGpuFrame();
          if (this.wasmHostOwnsViewportCamera()) {
            this.syncLastPushedCameraFromWasmHost();
          }
        }
      }
      this.paintTextOverlays(syncedGpuThisFrame);
      const frameState: FrameState = {
        camera: { ...this.camera },
        renderer: this,
        selection: this.selectionStore.getSnapshot(),
      };
      for (const listener of this.frameListeners) {
        listener(frameState, frameDelta);
      }
      this.applyCanvasDebugAttributes();
      this.flushPendingBrushWasmInput();
      this.flushPendingBrushSessionMirror();
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
    await ensurePuzzle2dWasmLoaded();
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
    this.session.setSelectionOptions(o.method, puzzle2dSelectionModeForHost(o.mode), o.targets.nodes, o.targets.edges, o.targets.handles);
    this.session.setWorldRasterTiling(this.worldRasterTiling);
    this.pushLodAndGridSnapToWasmSession();
    this.session.setHandleLinkCompatJson(this.kindCompatJson);
    try {
      this.session.setKindCatalogsJson(this.kindCatalogsJson);
    } catch (err) {
      console.error("[DEBUG] setKindCatalogsJson failed after attach_canvas", err);
    }
    this.lastPushedKindCatalogsJson = this.kindCatalogsJson;
    this.applyWasmDrainToScene(this.session.drainEventsJson(), { silentStructuralRemoves: true });
    this.reconcileHostDeclarativeSceneEdges();
    this.syncGpuReadyCacheFromSession();
  }

  private descriptorJsonForWasmHost(): string {
    const fingerprint = puzzle2dSceneWasmFingerprintFromRenderer(this);
    if (
      this.lastSceneWasmFingerprint === fingerprint &&
      this.lastPushedDescriptorJson !== null &&
      puzzle2dWasmDescriptorJsonMatchesScene(this, this.lastPushedDescriptorJson)
    ) {
      return this.lastPushedDescriptorJson;
    }
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
      if (node.textFontFamily !== PUZZLE_2D_NODE_TEXT_FONT_FAMILY_DEFAULT) {
        base.textFontFamily = node.textFontFamily;
      }
      if (node.textFontSize !== PUZZLE_2D_NODE_TEXT_FONT_PX_DEFAULT) {
        base.textFontSize = node.textFontSize;
      }
      if (node.textAlignment !== PUZZLE_2D_NODE_TEXT_ALIGNMENT_DEFAULT) {
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
    const json = JSON.stringify({ nodes, handles, edges, wires });
    this.lastSceneWasmFingerprint = fingerprint;
    return json;
  }

  private syncPuzzle2dAppearanceFromDocument(): void {
    if (this.renderMode === "headless-test") {
      return;
    }
    if (typeof document === "undefined") {
      return;
    }
    try {
      const json = serializePuzzle2dVelloThemeJson();
      if (json !== this.lastVelloThemeJson) {
        this.lastVelloThemeJson = json;
        this.session.setVelloThemeJson(json);
      }
    } catch {
      this.lastVelloThemeJson = "";
    }
    const styles = puzzle2dDefaultStylesFromElementsUiTokens();
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
    if (this.lastActiveToolForWasm !== this.activeTool) {
      this.session.setActiveTool(this.activeTool);
      this.lastActiveToolForWasm = this.activeTool;
    }
    if (this.lastBrushFlushDistanceForWasm === null || !nearlyEqual(this.lastBrushFlushDistanceForWasm, this.brushFlushDistance)) {
      this.session.setBrushFlushDistance(this.brushFlushDistance);
      this.lastBrushFlushDistanceForWasm = this.brushFlushDistance;
    }
    const brushKindWeightsJson = JSON.stringify({
      nodeWeights: this.brushNodeKindWeights,
      handleWeights: this.brushHandleKindWeights,
    });
    if (this.lastBrushKindWeightsJsonForWasm !== brushKindWeightsJson) {
      this.session.setBrushKindWeights(brushKindWeightsJson);
      this.lastBrushKindWeightsJsonForWasm = brushKindWeightsJson;
    }
    if (this.lastBrushNodeSizeForWasm === null || !nearlyEqual(this.lastBrushNodeSizeForWasm, this.brushNodeSize)) {
      this.session.setBrushNodeSize(this.brushNodeSize);
      this.lastBrushNodeSizeForWasm = this.brushNodeSize;
    }
  }

  /** @emoji 📍 Flushes peer-pane node drags through {@link BoardSession.setNodePositionsJson} without invalidating the full descriptor cache. */
  private flushPendingIncrementalNodeMovesToWasm(): boolean {
    if (this.pendingIncrementalNodeMoves.size === 0) {
      return false;
    }
    const moves = [...this.pendingIncrementalNodeMoves.entries()].map(([id, position]) => ({ id, x: position.x, y: position.y }));
    this.pendingIncrementalNodeMoves.clear();
    try {
      this.session.setNodePositionsJson(JSON.stringify(moves));
    } catch (err) {
      console.error("[DEBUG] setNodePositionsJson failed", err);
      return false;
    }
    return true;
  }

  /** @emoji ✅ Pushes committed selection and area-select preview to WASM without a full descriptor round-trip. */
  private pushWasmSelectionAndPreselectToSession(): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      return;
    }
    const selectionJson = JSON.stringify(this.selectionStore.getSnapshot().ids);
    const preselectJson = JSON.stringify(this.preselectStore.getSnapshot());
    try {
      this.session.setSelectionIdsJsonSilent(selectionJson);
      this.session.setPreselectStateJsonSilent(preselectJson);
    } catch (err) {
      console.error("[DEBUG] pushWasmSelectionAndPreselectToSession failed", err);
    }
  }

  /** @emoji 📷 True when pan/zoom/drag left the authoritative viewport on the WASM host (do not stomp with lagging JS props). */
  private wasmHostOwnsViewportCamera(): boolean {
    if (this.wheelZoomGestureActive || this.wasmViewportLeading) {
      return true;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      return false;
    }
    try {
      return this.session.defersDescriptorSyncFromJs();
    } catch {
      return false;
    }
  }

  /** @emoji 📷 Reads the live WASM viewport camera without mutating the JS store. */
  private readWasmCameraSnapshot(): CameraState | null {
    if (this.wasmSessionCallBlockedForReentry()) {
      return null;
    }
    try {
      const raw = JSON.parse(this.session.cameraJson()) as { x: number; y: number; zoom: number };
      const zoom = clamp(raw.zoom, MIN_ZOOM, MAX_ZOOM);
      const x = Number(raw.x);
      const y = Number(raw.y);
      if (!Number.isFinite(x) || !Number.isFinite(y)) {
        return null;
      }
      return { x, y, zoom };
    } catch {
      return null;
    }
  }

  /** @emoji 📌 Keeps {@link Puzzle2dRenderer.lastPushedCameraX} aligned with the WASM host after wheel/pan skips JS camera push. */
  private syncLastPushedCameraFromWasmHost(): void {
    const cam = this.readWasmCameraSnapshot();
    if (!cam) {
      return;
    }
    this.lastPushedCameraX = cam.x;
    this.lastPushedCameraY = cam.y;
    this.lastPushedCameraZoom = cam.zoom;
  }

  /** @emoji 📐 Pushes viewport size and camera when the scene descriptor cache is still valid (pan/zoom/selection chrome). */
  private pushWasmViewportAndSizeToSession(): void {
    if (this.lastPushedWidth !== this.width || this.lastPushedHeight !== this.height || this.lastPushedDpr !== this.dpr) {
      this.session.setSize(this.width, this.height, this.dpr);
      this.lastPushedWidth = this.width;
      this.lastPushedHeight = this.height;
      this.lastPushedDpr = this.dpr;
    }
    if (this.wasmHostOwnsViewportCamera()) {
      return;
    }
    if (
      this.camera.x === this.lastPushedCameraX &&
      this.camera.y === this.lastPushedCameraY &&
      nearlyEqual(this.camera.zoom, this.lastPushedCameraZoom)
    ) {
      return;
    }
    this.session.setCamera(this.camera.x, this.camera.y, this.camera.zoom);
    this.lastPushedCameraX = this.camera.x;
    this.lastPushedCameraY = this.camera.y;
    this.lastPushedCameraZoom = this.camera.zoom;
  }

  /** @emoji 🛡️ Defers WASM scene push when `attach_canvas` or `renderFrame` still holds a session borrow; sets {@link Puzzle2dRenderer.invalidated} so the next frame retries. */
  private pushSceneToWasmDriver(): void {
    if (this.suppressSceneToWasmPush) {
      return;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    const descriptorCacheValid =
      this.lastPushedDescriptorJson !== null &&
      this.lastPushedSceneDescriptorEpoch === this.sceneDescriptorEpoch &&
      puzzle2dWasmDescriptorJsonMatchesScene(this, this.lastPushedDescriptorJson);
    const deferDescriptorSync = this.session.isDraggingAreaSelect() || this.session.defersDescriptorSyncFromJs() || this.preselectIds.size > 0;
    if (descriptorCacheValid) {
      this.flushPendingIncrementalNodeMovesToWasm();
      this.pushWasmViewportAndSizeToSession();
      this.pushWasmSelectionAndPreselectToSession();
      this.wasmPushSceneDrainAlreadyApplied = false;
      return;
    }
    const o = this.selectionOptions;
    this.session.setSize(this.width, this.height, this.dpr);
    this.lastPushedWidth = this.width;
    this.lastPushedHeight = this.height;
    this.lastPushedDpr = this.dpr;
    this.session.setSelectionOptions(o.method, puzzle2dSelectionModeForHost(o.mode), o.targets.nodes, o.targets.edges, o.targets.handles);
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
    if (this.lastDescriptorPushDeferred && !deferDescriptorSync && this.wasmDeferHadStructuralMutation) {
      this.markSceneDescriptorDirty();
    }
    this.lastDescriptorPushDeferred = deferDescriptorSync;
    if (!deferDescriptorSync) {
      this.wasmDeferHadStructuralMutation = false;
    }
    const flushedIncrementalNodeMoves = this.flushPendingIncrementalNodeMovesToWasm();
    if (!deferDescriptorSync) {
      if (this.scene.nodes.size === 0) {
        this.invalidated = true;
        return;
      }
      let deferEdgelessWasmDescriptorPush = this.declarativeSceneEdgeExpectation > 0 && this.scene.edges.size === 0;
      if (deferEdgelessWasmDescriptorPush) {
        deferEdgelessWasmDescriptorPush = !this.reconcileHostDeclarativeSceneEdges();
      }
      const skipFullDescriptorSync = (flushedIncrementalNodeMoves && descriptorCacheValid) || descriptorCacheValid;
      if (!deferEdgelessWasmDescriptorPush && !skipFullDescriptorSync) {
        const desc = this.descriptorJsonForWasmHost();
        if (desc !== this.lastPushedDescriptorJson) {
          try {
            this.session.syncDescriptorJson(desc);
            this.lastPushedDescriptorJson = desc;
            this.lastPushedSceneDescriptorEpoch = this.sceneDescriptorEpoch;
          } catch (err) {
            console.error("[DEBUG] syncDescriptorJson failed", err);
          }
        } else {
          this.lastPushedSceneDescriptorEpoch = this.sceneDescriptorEpoch;
        }
      } else if (deferEdgelessWasmDescriptorPush) {
        this.invalidated = true;
      }
    }
    this.pushWasmViewportAndSizeToSession();
    this.syncPuzzle2dAppearanceFromDocument();
    this.applyWasmDrainToScene(this.session.drainEventsJson(), { silentStructuralRemoves: true });
  }

  /** @emoji 📡 Pushes the imperative scene graph to WASM immediately after user structural deletes (avoids stale WASM edges on the source pane). */
  private pushAuthoritativeDescriptorToWasmSession(): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.ensureImperativeSceneForWasmPush();
    if (this.scene.nodes.size === 0) {
      return;
    }
    const desc = this.descriptorJsonForWasmHost();
    try {
      this.session.syncDescriptorJson(desc);
      this.lastPushedDescriptorJson = desc;
      this.lastPushedSceneDescriptorEpoch = this.sceneDescriptorEpoch;
      this.lastSceneWasmFingerprint = puzzle2dSceneWasmFingerprintFromRenderer(this);
    } catch (err) {
      console.error("[DEBUG] syncDescriptorJson failed", err);
    }
    this.applyWasmDrainToScene(this.session.drainEventsJson(), { silentStructuralRemoves: true });
  }

  private applyWasmDrainToScene(
    raw: string,
    options?: { readonly silentStructuralRemoves?: boolean; readonly silentCamera?: boolean },
  ): void {
    const silentStructuralRemoves =
      options?.silentStructuralRemoves ??
      (this.declarativeSceneEdgeExpectation > 0 && this.structuralDeleteFixtureMirrorDepth <= 0);
    const drainOptions = { silentStructuralRemoves, silentCamera: options?.silentCamera };
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
    let needsGraphObservation = false;
    this.suppressSceneToWasmPush = true;
    try {
      for (const row of rows) {
        if (!PUZZLE2D_DRAIN_SKIP_GRAPH_OBSERVATION.has(row.name)) {
          needsGraphObservation = true;
        }
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
            this.wasmViewportLeading = true;
            this.bumpTextOverlayGeometryEpoch();
            if (!drainOptions.silentCamera) {
              this.emitter.emit("camera", { ...this.camera });
            }
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
            this.syncSelectionChromeToSceneObjectsIfNeeded();
            this.emit("preselectCancel", PUZZLE_2D_PRESELECT_EMPTY);
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
              this.lastNodeAuthoringPositionById.set(id, { x, y });
              this.bumpTextOverlayGeometryEpoch();
            }
            this.emitter.emit("nodeMove", { id, x, y });
            puzzle2dBroadcastNodeMove(this, { id, x, y });
            break;
          }
          case "nodeDragEnd": {
            const moves = (row.payload as { moves?: Array<{ id?: string; x?: number; y?: number }> }).moves ?? [];
            const payload = moves
              .map((move) => ({
                id: String(move.id ?? ""),
                x: Number(move.x),
                y: Number(move.y),
              }))
              .filter((move) => move.id !== "" && Number.isFinite(move.x) && Number.isFinite(move.y));
            if (payload.length > 0) {
              this.emitter.emit("nodeDragEnd", { moves: payload });
            }
            break;
          }
          case "edgeDelete": {
            const id = String((row.payload as { id: string }).id);
            const spuriousWasmEdgeDelete =
              drainOptions.silentStructuralRemoves &&
              this.declarativeSceneEdgeExpectation > 0 &&
              this.scene.edges.has(id) &&
              !this.isAuthoritativeStructuralRemovalSuppressed("edge", id);
            if (spuriousWasmEdgeDelete) {
              const edge = this.scene.edges.get(id);
              if (edge) {
                this.clearWasmHostAuthorshipForEdge(id);
                this.runWithoutSceneDeleteEvents(() => {
                  this.scene.remove(edge);
                });
                graphMutatedForHostMerge = true;
              }
              break;
            }
            const edge = this.scene.edges.get(id);
            if (edge) {
              this.clearWasmHostAuthorshipForEdge(id);
              const remove = () => {
                this.scene.remove(edge);
              };
              if (drainOptions.silentStructuralRemoves) {
                this.runWithoutSceneDeleteEvents(remove);
              } else {
                remove();
              }
              graphMutatedForHostMerge = true;
            } else {
              this.clearWasmHostAuthorshipForEdge(id);
              graphMutatedForHostMerge = true;
            }
            if (!spuriousWasmEdgeDelete) {
              const payload = { kind: "edge" as const, id };
              if (this.structuralDeleteFixtureMirrorDepth > 0) {
                this.commitAuthoritativeStructuralDelete(payload);
              } else {
                this.pruneHostDeclarativeStructuralDelete(payload);
              }
            }
            break;
          }
          case "nodeDelete": {
            const id = String((row.payload as { id: string }).id);
            const node = this.scene.nodes.get(id);
            if (node) {
              const remove = () => {
                this.scene.remove(node);
              };
              if (drainOptions.silentStructuralRemoves) {
                this.runWithoutSceneDeleteEvents(remove);
              } else {
                remove();
              }
              graphMutatedForHostMerge = true;
            }
            if (!drainOptions.silentStructuralRemoves || this.structuralDeleteFixtureMirrorDepth > 0) {
              const payload = { kind: "node" as const, id };
              if (this.structuralDeleteFixtureMirrorDepth > 0 && drainOptions.silentStructuralRemoves) {
                this.commitAuthoritativeStructuralDelete(payload);
              } else {
                this.pruneHostDeclarativeStructuralDelete(payload);
              }
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
            if (!isPuzzle2dSceneHandleObject(sourceObj) || !isPuzzle2dSceneHandleObject(targetObj)) {
              break;
            }
            this.wasmHostAuthoredEdgeIds.add(id);
            this.wasmHostAuthoredLinkByEdgeId.set(id, { source: sourceId, target: targetId });
            this.scene.ingestWasmEdge(new Puzzle2dSceneEdge({ id, source: sourceObj, target: targetObj }));
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
          case "brushPlace": {
            const p = row.payload as Record<string, unknown>;
            const handlesRaw = Array.isArray(p.handles) ? p.handles : [];
            const handles = handlesRaw
              .map((row) => {
                const h = row as Record<string, unknown>;
                const handleKind = String(h.handleKind ?? "").trim();
                const angle = Number(h.angle);
                if (handleKind === "" || !Number.isFinite(angle)) {
                  return null;
                }
                const radius = h.radius === undefined ? undefined : Number(h.radius);
                return { handleKind, angle, ...(radius !== undefined && Number.isFinite(radius) ? { radius } : {}) };
              })
              .filter((h): h is NonNullable<typeof h> => h !== null);
            const shape = String(p.shape ?? "circle") === "rectangle" ? "rectangle" : "circle";
            const iconKindRaw = p.iconKind;
            const iconKind = typeof iconKindRaw === "string" && iconKindRaw.trim() !== "" ? iconKindRaw.trim() : undefined;
            const nodeIdRaw = p.nodeId;
            const edgeIdRaw = p.edgeId;
            const nodeId = typeof nodeIdRaw === "string" && nodeIdRaw.trim() !== "" ? nodeIdRaw.trim() : undefined;
            const edgeId = typeof edgeIdRaw === "string" && edgeIdRaw.trim() !== "" ? edgeIdRaw.trim() : undefined;
            const payload: Puzzle2dBrushPlacePayload = {
              handles,
              nodeKind: String(p.nodeKind ?? ""),
              shape,
              sourceHandleId: String(p.sourceHandleId ?? ""),
              targetHandleIndex: Number(p.targetHandleIndex ?? 0),
              x: Number(p.x),
              y: Number(p.y),
              ...(nodeId ? { nodeId } : {}),
              ...(edgeId ? { edgeId } : {}),
              ...(iconKind ? { iconKind } : {}),
              ...(shape === "rectangle"
                ? { height: Number(p.height), width: Number(p.width) }
                : { radius: Number(p.radius) }),
            };
            if (payload.nodeKind === "" || payload.sourceHandleId === "" || !Number.isFinite(payload.x) || !Number.isFinite(payload.y)) {
              break;
            }
            puzzle2dInvokeBrushPlaceCommit(payload);
            this.emitter.emit("brushPlace", payload);
            break;
          }
          case "brushPreview": {
            const previewPayload = row.payload as Puzzle2dEventMap["brushPreview"];
            this.emitter.emit("brushPreview", previewPayload);
            puzzle2dUpdateBrushSessionFromSource(this, null, previewPayload);
            break;
          }
          case "brushCandidates": {
            const p = row.payload as { sourceHandleId?: string; candidates?: string[]; index?: number };
            const candidatesPayload: Puzzle2dBrushCandidatesPayload = {
              sourceHandleId: String(p.sourceHandleId ?? ""),
              candidates: Array.isArray(p.candidates) ? p.candidates.map(String) : [],
              index: Number(p.index ?? 0),
            };
            this.emitter.emit("brushCandidates", candidatesPayload);
            puzzle2dUpdateBrushSessionFromSource(this, candidatesPayload, null);
            break;
          }
          default:
            break;
        }
      }
    } finally {
      this.suppressSceneToWasmPush = false;
      if (graphMutatedForHostMerge || needsGraphObservation) {
        this.enqueuePuzzle2dGraphObservationFlush();
      }
      if (graphMutatedForHostMerge) {
        this.wasmDeferHadStructuralMutation = true;
        this.bumpWasmHostSceneMergeResyncEpoch();
        this.markSceneDescriptorDirty();
      }
      const hostEdgeExpectation = this.hostDeclarativeSceneDescriptor?.edges.length ?? this.declarativeSceneEdgeExpectation;
      if (hostEdgeExpectation > 0 && this.hostDeclarativeSceneDescriptor && this.scene.edges.size < hostEdgeExpectation) {
        if (this.reconcileHostDeclarativeSceneEdges()) {
          this.markSceneDescriptorDirty();
          this.bumpWasmHostSceneMergeResyncEpoch();
        }
      }
    }
  }

  /** @emoji 🧪 Whether the 2D text overlay must repaint (camera, LOD, selection chrome, scene text, theme). */
  textOverlayDirty(): boolean {
    if (!this.textOverlayPainted) {
      return true;
    }
    if (this.wheelZoomGestureActive) {
      return true;
    }
    const lod = this.effectiveDrawLodLabel();
    const wasmCam = this.readWasmCameraSnapshot();
    const selection = this.selectionStore.getSnapshot();
    const preselect = this.preselectStore.getSnapshot();
    const selectionChromeDirty =
      this.lastOverlaySelection === null ||
      !puzzle2dSelectionSnapshotsEqual(selection, this.lastOverlaySelection) ||
      this.lastOverlayPreselect === null ||
      !preselectSnapshotsEqual(preselect, this.lastOverlayPreselect);
    const cameraDiffersFromLastOverlay = (cam: CameraState): boolean =>
      cam.x !== this.lastOverlayCameraX || cam.y !== this.lastOverlayCameraY || !nearlyEqual(cam.zoom, this.lastOverlayCameraZoom);
    const viewportDirty =
      cameraDiffersFromLastOverlay(this.camera) ||
      (this.wasmHostOwnsViewportCamera() && wasmCam !== null && cameraDiffersFromLastOverlay(wasmCam)) ||
      this.width !== this.lastOverlayWidth ||
      this.height !== this.lastOverlayHeight ||
      this.dpr !== this.lastOverlayDpr ||
      lod !== this.lastOverlayLod;
    return (
      this.textOverlayContentEpoch !== this.lastOverlayContentEpoch ||
      this.lastVelloThemeJson !== this.lastOverlayVelloThemeJson ||
      selectionChromeDirty ||
      viewportDirty
    );
  }

  /** @emoji 📷 False while pan/zoom/drag defers host camera props so {@link Puzzle2dHostSubtree} does not stomp the live WASM viewport. */
  acceptsHostCameraProp(): boolean {
    if (this.wheelZoomGestureActive) {
      return false;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      return true;
    }
    try {
      return !this.session.defersDescriptorSyncFromJs();
    } catch {
      return true;
    }
  }

  /** @emoji 🏷️ Reads camera, draw LOD, and node centers from the WASM host (authoritative for the GPU frame). */
  private readOverlayPaintStateFromWasm(): { readonly camera: CameraState; readonly lod: Puzzle2dDrawLodKind; readonly centersById: ReadonlyMap<string, Point> } | null {
    if (this.wasmSessionCallBlockedForReentry()) {
      return null;
    }
    let raw: Puzzle2dOverlayPaintStateWasm;
    try {
      raw = JSON.parse(this.session.overlayPaintStateJson()) as Puzzle2dOverlayPaintStateWasm;
    } catch {
      return null;
    }
    const zoom = clamp(Number(raw.camera?.zoom), MIN_ZOOM, MAX_ZOOM);
    const camera: CameraState = {
      x: Number(raw.camera?.x),
      y: Number(raw.camera?.y),
      zoom,
    };
    if (!Number.isFinite(camera.x) || !Number.isFinite(camera.y)) {
      return null;
    }
    const lodRaw = typeof raw.lod === "string" ? raw.lod.trim() : "";
    const lod: Puzzle2dDrawLodKind = isPuzzle2dDrawLodKind(lodRaw) ? lodRaw : this.effectiveDrawLodLabel();
    const centersById = new Map<string, Point>();
    if (Array.isArray(raw.nodes)) {
      for (const row of raw.nodes) {
        if (!row || typeof row.id !== "string") {
          continue;
        }
        const x = Number(row.x);
        const y = Number(row.y);
        if (Number.isFinite(x) && Number.isFinite(y)) {
          centersById.set(row.id, { x, y });
        }
      }
    }
    return { camera, lod, centersById };
  }

  private worldToScreenWithCamera(point: Point, camera: CameraState): Point {
    return {
      x: (point.x - camera.x) * camera.zoom + this.width / 2,
      y: (point.y - camera.y) * camera.zoom + this.height / 2,
    };
  }

  /** @emoji 📌 Records the overlay inputs used by the last {@link Puzzle2dRenderer.paintTextOverlays} pass. */
  private rememberTextOverlayPainted(overlayCamera: CameraState, overlayLod: Puzzle2dDrawLodKind): void {
    this.textOverlayPainted = true;
    this.lastOverlayCameraX = overlayCamera.x;
    this.lastOverlayCameraY = overlayCamera.y;
    this.lastOverlayCameraZoom = overlayCamera.zoom;
    this.lastOverlayWidth = this.width;
    this.lastOverlayHeight = this.height;
    this.lastOverlayDpr = this.dpr;
    this.lastOverlayLod = overlayLod;
    this.lastOverlaySelection = this.selectionStore.getSnapshot();
    this.lastOverlayPreselect = this.preselectStore.getSnapshot();
    this.lastOverlayContentEpoch = this.textOverlayContentEpoch;
    this.lastOverlayVelloThemeJson = this.lastVelloThemeJson;
  }

  /** @emoji 🏷️ Draws node captions on {@link Puzzle2dRenderer.attachTextOverlayCanvas} (GPU path has no text primitives). */
  private paintTextOverlays(syncedGpuThisFrame = false): void {
    if (this.renderMode === "headless-test" || !this.textOverlayCanvas) {
      return;
    }
    if (!syncedGpuThisFrame && !this.wheelZoomGestureActive && !this.textOverlayDirty()) {
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
    if (syncedGpuThisFrame || this.wheelZoomGestureActive || this.wasmHostOwnsViewportCamera()) {
      this.syncCameraFromWasmHostSilent();
    }
    const wasmOverlay = this.readOverlayPaintStateFromWasm();
    const overlayCamera = wasmOverlay?.camera ?? this.readWasmCameraSnapshot() ?? this.camera;
    const lod = wasmOverlay?.lod ?? this.effectiveDrawLodLabel();
    const overlayZoom = overlayCamera.zoom;
    const chrome = puzzle2dElementInteractionChrome(this.selectionIds, this.preselectStore.getSnapshot());
    for (const node of this.scene.nodes.values()) {
      if (!node.visible) {
        continue;
      }
      const caption = puzzle2dTextOverlayCaptionForLod(node.text ?? "", lod, node.iconKind);
      if (caption === null) {
        continue;
      }
      let maxW: number;
      let maxH: number;
      if (node.shape === "rectangle") {
        maxW = node.width * overlayZoom * inset;
        maxH = node.height * overlayZoom * inset;
      } else {
        const d = 2 * node.radius * overlayZoom * inset;
        maxW = d;
        maxH = d;
      }
      if (maxW < 4 || maxH < 4) {
        continue;
      }
      const center = wasmOverlay?.centersById.get(node.id) ?? { x: node.x, y: node.y };
      const boxCenter = this.worldToScreenWithCamera(center, overlayCamera);
      const style = this.getStyle(node.style, puzzle2dInteractionChromeStyleKey("node", node.id, chrome));
      const family = node.textFontFamily;
      ctx.fillStyle = style.stroke ?? PUZZLE_2D_STYLES_HEADLESS_FALLBACK.node.stroke ?? "#001117";
      if (node.textAutofit) {
        const layoutKey = `${node.id}\0${lod}\0${Math.round(maxW)}\0${Math.round(maxH)}\0${caption}\0${family ?? ""}`;
        let layout = this.textOverlayLayoutCache.get(layoutKey);
        if (!layout) {
          const fontPx = puzzle2dFitTextFontPx(ctx, caption, maxW, maxH, 4, 512, family);
          ctx.font = puzzle2dBuildCanvasFontSpec(fontPx, family);
          let line = caption;
          if (ctx.measureText(line).width > maxW) {
            line = puzzle2dEllipsisTextToWidth(ctx, caption, maxW);
          }
          layout = { line, fontPx };
          this.textOverlayLayoutCache.set(layoutKey, layout);
        }
        ctx.font = puzzle2dBuildCanvasFontSpec(layout.fontPx, family);
        ctx.textAlign = "center";
        ctx.textBaseline = "middle";
        ctx.fillText(layout.line, boxCenter.x, boxCenter.y);
        continue;
      }
      const fixedKey = `${node.id}\0${lod}\0${Math.round(maxW)}\0${node.textFontSize}\0${caption}\0${family ?? ""}`;
      let fixedLayout = this.textOverlayLayoutCache.get(fixedKey);
      if (!fixedLayout) {
        const fontPx = node.textFontSize;
        ctx.font = puzzle2dBuildCanvasFontSpec(fontPx, family);
        const line = puzzle2dEllipsisTextToWidth(ctx, caption, maxW);
        fixedLayout = { line, fontPx };
        this.textOverlayLayoutCache.set(fixedKey, fixedLayout);
      }
      ctx.font = puzzle2dBuildCanvasFontSpec(fixedLayout.fontPx, family);
      const line = fixedLayout.line;
      const anchor = puzzle2dNodeTextPlacementAnchor(boxCenter.x, boxCenter.y, maxW, maxH, node.textAlignment);
      ctx.textAlign = anchor.textAlign;
      ctx.textBaseline = anchor.textBaseline;
      ctx.fillText(line, anchor.fillX, anchor.fillY);
    }
    const drawHandleLabels = lod === "detail" || lod === "micro";
    if (drawHandleLabels) {
      const handleFontPx = lod === "detail" ? 10 : 11;
      ctx.font = puzzle2dBuildCanvasFontSpec(handleFontPx, PUZZLE_2D_NODE_TEXT_FONT_FAMILY_DEFAULT);
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
        const rawLabel = puzzle2dHandleKindOverlayLabel(handle.handleKind, this.kindCatalogsBundle);
        const caption = puzzle2dHandleOverlayCaptionForLod(rawLabel, lod);
        if (caption === null) {
          continue;
        }
        const nodeCenter = wasmOverlay?.centersById.get(node.id) ?? { x: node.x, y: node.y };
        const handleWorld = computeHandlePosition(
          { height: node.height, radius: node.radius, shape: node.shape, width: node.width, x: nodeCenter.x, y: nodeCenter.y },
          handle.angle,
        );
        const handleScreen = this.worldToScreenWithCamera(handleWorld, overlayCamera);
        const nodeScreen = this.worldToScreenWithCamera(nodeCenter, overlayCamera);
        const dx = handleScreen.x - nodeScreen.x;
        const dy = handleScreen.y - nodeScreen.y;
        const len = Math.hypot(dx, dy);
        const outward = len > 1e-6 ? 10 / len : 0;
        const labelX = handleScreen.x + dx * outward;
        const labelY = handleScreen.y + dy * outward;
        const style = this.getStyle(handle.style, puzzle2dInteractionChromeStyleKey("handle", handle.id, chrome));
        ctx.fillStyle = style.stroke ?? PUZZLE_2D_STYLES_HEADLESS_FALLBACK.handle.stroke ?? "#001117";
        ctx.fillText(caption, labelX, labelY);
      }
    }
    this.rememberTextOverlayPainted(overlayCamera, lod);
  }

  /** @emoji 🎨 Presents one GPU frame after {@link Puzzle2dRenderer.pushSceneToWasmDriver} (same order as pre-369 main-thread canvas: no WASM scene push until the swapchain exists). */
  private syncGpuFrame(): boolean {
    if (this.renderMode === "headless-test" || !this.readGpuReady()) {
      return false;
    }
    if (this.wasmGpuFrameDepth > 0) {
      return false;
    }
    if (!this.suppressSceneToWasmPush) {
      if (this.wasmPushSceneDrainAlreadyApplied) {
        this.wasmPushSceneDrainAlreadyApplied = false;
        this.pushWasmViewportAndSizeToSession();
      } else {
        this.pushSceneToWasmDriver();
      }
    }
    this.wasmGpuFrameDepth += 1;
    try {
      this.session.renderFrame();
      this.gpuSurfacePresentedFrame = true;
      this.gpuSurfaceErrorDetail = "";
      return true;
    } catch (err: unknown) {
      console.error("[DEBUG] Puzzle2dRenderer GPU surface frame failed", err);
      this.gpuSurfaceErrorDetail = summarizeRasterSurfaceFailure(err);
      this.gpuSurfaceUnavailable = true;
      this.gpuSurfacePresentedFrame = false;
      this.cachedWasmGpuReady = false;
      return false;
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

  /** @emoji 📡 Refreshes {@link Puzzle2dRenderer.cachedWasmGpuReady} from WASM when no in-flight GPU frame holds the session borrow. */
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
    if (this.viewportWheelEmitRafId !== null && globalThis.cancelAnimationFrame) {
      globalThis.cancelAnimationFrame(this.viewportWheelEmitRafId);
      this.viewportWheelEmitRafId = null;
    }
    if (this.wheelCameraReactSyncTimeoutId !== null) {
      clearTimeout(this.wheelCameraReactSyncTimeoutId);
      this.wheelCameraReactSyncTimeoutId = null;
    }
    if (this.wheelFlushRafId !== null && globalThis.cancelAnimationFrame) {
      globalThis.cancelAnimationFrame(this.wheelFlushRafId);
      this.wheelFlushRafId = null;
    }
    this.pendingWheelScreen = null;
    if (this.inputInvalidateRafId !== null && globalThis.cancelAnimationFrame) {
      globalThis.cancelAnimationFrame(this.inputInvalidateRafId);
      this.inputInvalidateRafId = null;
    }
    if (this.scheduledSelectEmitRafId !== null && globalThis.cancelAnimationFrame) {
      globalThis.cancelAnimationFrame(this.scheduledSelectEmitRafId);
      this.scheduledSelectEmitRafId = null;
    }
    this.pendingSelectEmitSnapshot = null;
    puzzle2dUnregisterAuthoringPeer(this);
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
    if (Puzzle2dRenderer.activeRenderer === this) {
      Puzzle2dRenderer.activeRenderer = null;
    }
    if (this.canvas) {
      const el = this.canvas as Puzzle2dCanvasElement;
      if (el.__puzzle2dRenderer === this) {
        delete el.__puzzle2dRenderer;
      }
    }
  }

  private attachCanvasListeners(): void {
    if (!this.canvas) {
      return;
    }
    this.canvas.tabIndex = 0;
    this.canvas.style.touchAction = "none";
    this.canvas.style.outline = "none";
    const bindings = new Puzzle2dEventBindingController();
    bindings.listen(this.canvas, "contextmenu", this.handleContextMenu as EventListener);
    bindings.listen(this.canvas, "pointerdown", this.handlePointerDown as EventListener);
    bindings.listen(this.canvas, "pointermove", this.handlePointerMove as EventListener);
    bindings.listen(this.canvas, "pointerup", this.handlePointerUp as EventListener);
    bindings.listen(this.canvas, "pointerleave", this.handlePointerLeave as EventListener);
    bindings.listen(this.canvas, "wheel", this.handleWheel as EventListener, { passive: false });
    bindings.listen(globalThis, "keydown", this.handleWindowKeyDown as EventListener, true);
    (this as Puzzle2dRenderer & { __eventBindings?: Puzzle2dEventBindingController }).__eventBindings = bindings;
  }

  private detachCanvasListeners(): void {
    (this as Puzzle2dRenderer & { __eventBindings?: Puzzle2dEventBindingController }).__eventBindings?.dispose();
    (this as Puzzle2dRenderer & { __eventBindings?: Puzzle2dEventBindingController }).__eventBindings = undefined;
  }

  /** @emoji 🎨 Reapplies scene `selected` / `highlighted` from committed selection and preselection only. */
  syncInteractionChrome(): void {
    this.applySelectionChromeToSceneObjects();
  }

  /** @emoji 🧿 True when every scene object may need selection chrome flags (descriptor epoch or first push). */
  private selectionChromeNeedsSceneObjectSync(): boolean {
    return this.lastPushedDescriptorJson === null || this.lastPushedSceneDescriptorEpoch !== this.sceneDescriptorEpoch;
  }

  private syncSelectionChromeToSceneObjectsIfNeeded(): void {
    if (this.selectionChromeNeedsSceneObjectSync()) {
      this.applySelectionChromeToSceneObjects();
      return;
    }
    this.applySelectionChromeDelta();
  }

  private rememberAppliedInteractionChrome(chrome: { highlightedIds: Set<string>; selectedIds: Set<string> }): void {
    this.lastAppliedChromeSelectedIds = new Set(chrome.selectedIds);
    this.lastAppliedChromeHighlightedIds = new Set(chrome.highlightedIds);
  }

  private applySelectionChromeToSceneObjects(): void {
    const chrome = puzzle2dElementInteractionChrome(this.selectionIds, this.preselectStore.getSnapshot());
    for (const object of this.scene.getAllObjects()) {
      const wantSelected = chrome.selectedIds.has(object.id);
      const wantHighlighted = chrome.highlightedIds.has(object.id);
      if (object.selected === wantSelected && object.highlighted === wantHighlighted) {
        continue;
      }
      object.selected = wantSelected;
      object.highlighted = wantHighlighted;
    }
    this.rememberAppliedInteractionChrome(chrome);
  }

  /** @emoji 🎯 Updates only objects whose selection chrome changed (fast path when descriptor cache is valid). */
  private applySelectionChromeDelta(): void {
    const chrome = puzzle2dElementInteractionChrome(this.selectionIds, this.preselectStore.getSnapshot());
    const touch = new Set([
      ...this.lastAppliedChromeSelectedIds,
      ...chrome.selectedIds,
      ...this.lastAppliedChromeHighlightedIds,
      ...chrome.highlightedIds,
    ]);
    for (const id of touch) {
      const object = this.scene.getObjectById(id);
      if (!object) {
        continue;
      }
      const wantSelected = chrome.selectedIds.has(id);
      const wantHighlighted = chrome.highlightedIds.has(id);
      if (object.selected === wantSelected && object.highlighted === wantHighlighted) {
        continue;
      }
      object.selected = wantSelected;
      object.highlighted = wantHighlighted;
    }
    this.rememberAppliedInteractionChrome(chrome);
  }

  private updateSelection(ids: Iterable<string>, emit: boolean): void {
    const nextIds = new Set(ids);
    const nextSnapshot = createSelectionSnapshot(nextIds);
    if (arrayEqual(nextSnapshot.ids, this.selectionStore.getSnapshot().ids)) {
      return;
    }
    this.selectionIds = nextIds;
    if (emit && !preselectSnapshotsEqual(this.preselectStore.getSnapshot(), PUZZLE_2D_PRESELECT_EMPTY)) {
      this.updatePreselection([], [], false);
      puzzle2dBroadcastPreselectSilent(this, PUZZLE_2D_PRESELECT_EMPTY);
    }
    this.syncSelectionChromeToSceneObjectsIfNeeded();
    this.selectionStore.setSnapshot(nextSnapshot, (left, right) => arrayEqual(left.ids, right.ids));
    if (emit) {
      puzzle2dBroadcastSelectionSilent(this, nextSnapshot.ids);
      this.scheduleCommittedSelectEmit(nextSnapshot);
    }
    this.scheduleInputInvalidate();
  }

  private updatePreselection(ids: Iterable<string>, removedIds: Iterable<string>, emit: boolean): void {
    const nextSnapshot = createPreselectSnapshot(ids, removedIds);
    if (preselectSnapshotsEqual(nextSnapshot, this.preselectStore.getSnapshot())) {
      return;
    }
    this.preselectIds = new Set(nextSnapshot.ids);
    this.preselectRemovedIds = new Set(nextSnapshot.removedIds);
    this.preselectStore.setSnapshot(nextSnapshot, preselectSnapshotsEqual);
    this.syncSelectionChromeToSceneObjectsIfNeeded();
    if (emit) {
      puzzle2dBroadcastPreselectSilent(this, nextSnapshot);
      this.emit("preselect", nextSnapshot);
    }
    this.scheduleInputInvalidate();
  }

  private updateHover(id: string | null): void {
    if (this.hoveredId === id) {
      return;
    }
    this.hoveredId = id;
  }

  /** @emoji 📡 Emits {@link Puzzle2dEventMap.hover} when {@link Puzzle2dRenderer.hoveredId} changes (not every pointermove). */
  private publishHover(): void {
    if (this.lastEmittedHoverId === this.hoveredId) {
      return;
    }
    this.lastEmittedHoverId = this.hoveredId;
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

  /** @emoji 🖌️ Replays a deferred brush pointer leave/move after GPU/session reentry unblocks. */
  private flushPendingBrushWasmInput(): void {
    const pending = this.pendingBrushWasmFlush;
    if (pending === null || this.activeTool !== "brush" || this.wasmSessionCallBlockedForReentry()) {
      return;
    }
    this.pendingBrushWasmFlush = null;
    if (pending === "leave") {
      this.session.pointerLeaveScreen();
    } else {
      this.session.pointerMoveScreen(this.lastPointerScreenX, this.lastPointerScreenY, false, false);
    }
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.wasmPushSceneDrainAlreadyApplied = true;
    this.scheduleInputInvalidate();
  }

  private queuePendingBrushWasmFlush(kind: "leave" | "move"): void {
    if (this.activeTool !== "brush") {
      return;
    }
    this.pendingBrushWasmFlush = kind;
    this.invalidated = true;
  }

  private deleteSelectedObjects(): void {
    if (this.wasmSessionCallBlockedForReentry()) {
      this.invalidated = true;
      return;
    }
    this.withFixtureStructuralDeleteMirror(() => {
      this.session.deleteSelection();
      this.applyWasmDrainToScene(this.session.drainEventsJson());
    });
    this.pushAuthoritativeDescriptorToWasmSession();
    this.invalidate();
  }

  private readonly handleWindowKeyDown = (event: KeyboardEvent): void => {
    if (event.repeat) {
      return;
    }
    if (Puzzle2dRenderer.activeRenderer !== this) {
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
    if (this.activeTool === "brush" && event.key === "Tab") {
      event.preventDefault();
      if (this.wasmSessionCallBlockedForReentry()) {
        this.invalidated = true;
        return;
      }
      this.session.brushCycleCandidate(!event.shiftKey);
      this.applyWasmDrainToScene(this.session.drainEventsJson());
      this.scheduleInputInvalidate();
      return;
    }
    if (!shouldPuzzle2dHandleDeleteShortcut()) {
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
      this.canvas.dataset.puzzle2dSurfaceState = "off";
      delete this.canvas.dataset.puzzle2dSurfaceFailure;
    } else if (this.gpuSurfaceUnavailable) {
      this.canvas.dataset.puzzle2dSurfaceState = "error";
      if (this.gpuSurfaceErrorDetail) {
        this.canvas.dataset.puzzle2dSurfaceFailure = this.gpuSurfaceErrorDetail.slice(0, 512);
      }
    } else if (this.gpuSurfacePresentedFrame && this.readGpuReady()) {
      this.canvas.dataset.puzzle2dSurfaceState = "ready";
      delete this.canvas.dataset.puzzle2dSurfaceFailure;
    } else if (this.gpuSurfaceInitPromise) {
      this.canvas.dataset.puzzle2dSurfaceState = "init";
      delete this.canvas.dataset.puzzle2dSurfaceFailure;
    } else {
      this.canvas.dataset.puzzle2dSurfaceState = "pending";
      delete this.canvas.dataset.puzzle2dSurfaceFailure;
    }
    this.canvas.dataset.puzzle2dRaster = "gpu";
    this.canvas.dataset.puzzle2dWorldTiling = this.worldRasterTiling;
    const lod = this.effectiveDrawLodLabel();
    if (!this.wheelZoomGestureActive) {
      this.drawLodStore.setSnapshot(lod, (left, right) => left === right);
    }
    this.canvas.dataset.puzzle2dLod = lod;
    this.canvas.dataset.puzzle2dSceneNodeCount = String(this.scene.nodes.size);
    this.canvas.dataset.puzzle2dZoom = String(Math.round(this.camera.zoom * 1000) / 1000);
    this.canvas.dataset.puzzle2dSelection = sortedSelectionIds(this.selectionIds).join(",");
    this.canvas.dataset.puzzle2dHover = this.hoveredId ?? "";
    this.canvas.setAttribute("data-puzzle2d-camera", `${this.camera.x},${this.camera.y}`);
  }

  private readonly handleContextMenu = (event: MouseEvent): void => {
    if (!this.canvas) {
      return;
    }
    event.preventDefault();
    Puzzle2dRenderer.activeRenderer = this;
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
    const world = this.screenToWorld({ x: sx, y: sy });
    this.emit("contextmenu", { clientX: event.clientX, clientY: event.clientY, id: this.hoveredId, x: world.x, y: world.y });
    this.scheduleInputInvalidate();
  };

  private readonly handlePointerDown = (event: PointerEvent): void => {
    if (!this.canvas) {
      return;
    }
    if (event.button !== 0 && event.button !== 1) {
      return;
    }
    Puzzle2dRenderer.activeRenderer = this;
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
    this.pointerGestureCameraAtStart = { ...this.camera };
    this.session.pointerDownScreen(sx, sy, event.button, event.shiftKey, event.ctrlKey || event.metaKey);
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.scheduleInputInvalidate();
  };

  private readonly handlePointerMove = (event: PointerEvent): void => {
    if (!this.canvas) {
      return;
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.queuePendingBrushWasmFlush("move");
      return;
    }
    this.pendingBrushWasmFlush = null;
    const rect = this.canvas.getBoundingClientRect();
    const sx = event.clientX - rect.left;
    const sy = event.clientY - rect.top;
    this.recordPointerClient(event.clientX, event.clientY, sx, sy);
    this.session.pointerMoveScreen(sx, sy, event.shiftKey, event.ctrlKey || event.metaKey);
    const silentCamera = this.session.defersDescriptorSyncFromJs();
    this.applyWasmDrainToScene(this.session.drainEventsJson(), { silentCamera });
    this.wasmPushSceneDrainAlreadyApplied = true;
    this.scheduleInputInvalidate();
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
    const gestureStart = this.pointerGestureCameraAtStart;
    this.pointerGestureCameraAtStart = null;
    if (gestureStart !== null && (!pointsEqual(gestureStart, this.camera) || !nearlyEqual(gestureStart.zoom, this.camera.zoom))) {
      this.emitPublicCameraChange();
    }
    this.wasmPushSceneDrainAlreadyApplied = true;
    this.scheduleInputInvalidate();
    if (typeof event.pointerId === "number") {
      this.canvas.releasePointerCapture?.(event.pointerId);
    }
  };

  private readonly handlePointerLeave = (event: PointerEvent): void => {
    if (this.canvas) {
      const rect = this.canvas.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      this.recordPointerClient(event.clientX, event.clientY, sx, sy);
    }
    if (this.wasmSessionCallBlockedForReentry()) {
      this.queuePendingBrushWasmFlush("leave");
      return;
    }
    this.pendingBrushWasmFlush = null;
    this.session.pointerLeaveScreen();
    this.applyWasmDrainToScene(this.session.drainEventsJson());
    this.scheduleInputInvalidate();
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
    const prev = this.pendingWheelScreen;
    this.pendingWheelScreen = {
      sx,
      sy,
      deltaY: (prev?.deltaY ?? 0) + event.deltaY,
    };
    this.schedulePendingWheelFlush();
  };
}
//#endregion 🔖Renderer

//#region 🔖Vitest
const puzzle2dVitest = (
  import.meta as ImportMeta & {
    vitest?: {
      describe: typeof import("vitest").describe;
      expect: typeof import("vitest").expect;
      it: typeof import("vitest").it;
      vi: typeof import("vitest").vi;
    };
  }
).vitest;

if (puzzle2dVitest) {
  const { beforeAll, describe, expect, it, vi } = puzzle2dVitest;

  beforeAll(async () => {
    await ensurePuzzle2dWasmLoaded();
  });

  describe("puzzle2dFitTextFontPx", () => {
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
      const fit = puzzle2dFitTextFontPx(ctx, "aa", 100, 24, 4, 200, "monospace");
      expect(fit).toBe(20);
    });
  });

  describe("puzzle2dEllipsisTextToWidth", () => {
    it("returns the original string when it already fits", () => {
      const ctx: CanvasTextMeasuring = {
        font: "14px monospace",
        measureText: (t: string) => ({ width: t.length * 7 }),
      };
      expect(puzzle2dEllipsisTextToWidth(ctx, "short", 400)).toBe("short");
    });

    it("truncates with an ellipsis when the string is too wide", () => {
      const ctx: CanvasTextMeasuring = {
        font: "10px monospace",
        measureText: (t: string) => ({ width: t.length * 8 }),
      };
      const out = puzzle2dEllipsisTextToWidth(ctx, "abcdefghij", 50);
      expect(out.endsWith("…")).toBe(true);
      expect(ctx.measureText(out).width).toBeLessThanOrEqual(50);
      expect(out.length).toBeLessThan("abcdefghij".length + 1);
    });
  });

  describe("puzzle2dNodeTextPlacementAnchor", () => {
    it("anchors west at the left-middle of the node-centered box", () => {
      const a = puzzle2dNodeTextPlacementAnchor(100, 50, 80, 40, "w");
      expect(a).toEqual({ fillX: 60, fillY: 50, textAlign: "left", textBaseline: "middle" });
    });

    it("anchors center at the node box center by default", () => {
      const a = puzzle2dNodeTextPlacementAnchor(100, 50, 80, 40, PUZZLE_2D_NODE_TEXT_ALIGNMENT_DEFAULT);
      expect(a).toEqual({ fillX: 100, fillY: 50, textAlign: "center", textBaseline: "middle" });
    });
  });

  function createMockCanvas(width = 800, height = 600): { canvas: HTMLCanvasElement; context: Puzzle2dCanvasContext } {
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
    } satisfies Puzzle2dCanvasContext;
    Object.defineProperty(canvas, "clientWidth", { configurable: true, value: width });
    Object.defineProperty(canvas, "clientHeight", { configurable: true, value: height });
    Object.defineProperty(canvas, "getContext", { configurable: true, value: () => context });
    Object.defineProperty(canvas, "getBoundingClientRect", {
      configurable: true,
      value: () => ({ bottom: height, height, left: 0, right: width, top: 0, width, x: 0, y: 0 }),
    });
    return { canvas, context };
  }

  describe("puzzle2d hover publication", () => {
    it("emits hover with hit id and pointer/world coordinates after pointermove", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const hovers: Puzzle2dHoverPayload[] = [];
      renderer.on("hover", (h) => hovers.push(h));
      const node = new Puzzle2dSceneNode({ id: "hover-node", radius: 24, x: 0, y: 0 });
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

  describe("puzzle2d geometry helpers", () => {
    it("places cubic edge control arms along circle normals at the anchors", () => {
      const sourceNode = new Puzzle2dSceneNode({ id: "a", radius: 40, x: 0, y: 0 });
      const targetNode = new Puzzle2dSceneNode({ id: "b", radius: 40, x: 300, y: 0 });
      const sourceHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: sourceNode });
      const targetHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "b:h0", node: targetNode });
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
      const sourceNode = new Puzzle2dSceneNode({ id: "a", radius: 40, x: 0, y: 0 });
      const sourceHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: sourceNode });
      const curve = computeWireBezier(sourceHandle, { x: 200, y: 100 });
      expect(curve.p0.x).toBeCloseTo(40);
      expect(curve.p0.y).toBeCloseTo(0);
      expect(curve.p3.x).toBeCloseTo(200);
      expect(curve.p3.y).toBeCloseTo(100);
    });

    it("places rectangle handles on the perimeter by north-zero CCW angle", () => {
      const rectNode = new Puzzle2dSceneNode({ height: 20, id: "r", shape: "rectangle", width: 40, x: 100, y: 50 });
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
      expect(resolvePuzzle2dLodLabel(0.1)).toBe("minimap");
      expect(resolvePuzzle2dLodLabel(0.25)).toBe("overview");
      expect(resolvePuzzle2dLodLabel(0.4)).toBe("compact");
      expect(resolvePuzzle2dLodLabel(0.9)).toBe("normal");
      expect(resolvePuzzle2dLodLabel(1.3)).toBe("detail");
      expect(resolvePuzzle2dLodLabel(2.6)).toBe("micro");
      const tight: Puzzle2dLodZoomThresholds = {
        minimapMaxZoom: 0.2,
        overviewMaxZoom: 0.35,
        compactMaxZoom: 0.45,
        normalMaxZoom: 0.6,
        detailMaxZoom: 1,
      };
      expect(resolvePuzzle2dLodLabelFromThresholds(0.15, tight)).toBe("minimap");
      expect(resolvePuzzle2dLodLabelFromThresholds(0.3, tight)).toBe("overview");
      expect(resolvePuzzle2dLodLabelFromThresholds(0.4, tight)).toBe("compact");
      expect(resolvePuzzle2dLodLabelFromThresholds(0.5, tight)).toBe("normal");
      expect(resolvePuzzle2dLodLabelFromThresholds(0.7, tight)).toBe("detail");
      expect(resolvePuzzle2dLodLabelFromThresholds(1.1, tight)).toBe("micro");
    });

    it("switches caption policy across the six LOD bands", () => {
      expect(puzzle2dTextOverlayCaptionForLod("Node Label", "minimap", null)).toBeNull();
      expect(puzzle2dTextOverlayCaptionForLod("Node Label", "overview", null)).toBeNull();
      expect(puzzle2dTextOverlayCaptionForLod("Node Label", "compact", null)).toBe("Node La…");
      expect(puzzle2dTextOverlayCaptionForLod("Node Label", "normal", null)).toBe("Node La…");
      expect(puzzle2dTextOverlayCaptionForLod("Node Label", "detail", "catalog-icon")).toBe("Node La…");
      expect(puzzle2dTextOverlayCaptionForLod("0123456789012345", "micro", null)).toBe("01234567890…");
      expect(puzzle2dHandleOverlayCaptionForLod("Handle Label", "compact")).toBeNull();
      expect(puzzle2dHandleOverlayCaptionForLod("Handle Label", "detail")).toBe("Handl…");
      expect(puzzle2dHandleOverlayCaptionForLod("Handle Label", "micro")).toBe("Handle …");
    });
  });

  describe("puzzle 2d renderer render pipeline", () => {
    it("setSize returns false when layout dimensions are unchanged", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      expect(renderer.setSize(640, 480, 1)).toBe(true);
      expect(renderer.setSize(640, 480, 1)).toBe(false);
      renderer.dispose();
    });

    it("unchanged setSize does not schedule extra invalidate passes", () => {
      puzzle2dResetDebugPerfCounters();
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      renderer.setSize(100, 100, 1);
      const afterFirst = puzzle2dDebugPerfCounters().invalidate;
      renderer.setSize(100, 100, 1);
      expect(puzzle2dDebugPerfCounters().invalidate).toBe(afterFirst);
      renderer.dispose();
    });

    it("descriptorJsonForWasmHost reuses last json when scene wasm fingerprint is unchanged", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n1", radius: 20, x: 0, y: 0, text: "A" });
      renderer.scene.add(node);
      renderer.markSceneDescriptorDirty();
      const first = (renderer as { descriptorJsonForWasmHost(): string }).descriptorJsonForWasmHost();
      const second = (renderer as { descriptorJsonForWasmHost(): string }).descriptorJsonForWasmHost();
      expect(second).toBe(first);
      renderer.dispose();
    });

    it("syncPuzzle2dScene skips graph work when descriptor fingerprint is unchanged", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const descriptor = buildPuzzle2dSceneDescriptor(<Node id="solo" radius={36} x={0} y={0} />);
      expect(renderer.skipSceneSyncIfDescriptorUnchanged(descriptor)).toBe(false);
      syncPuzzle2dScene(renderer, descriptor);
      expect(renderer.skipSceneSyncIfDescriptorUnchanged(descriptor)).toBe(true);
      renderer.dispose();
    });

    it("descriptorJsonForWasmHost rebuilds when scene gains edges but cached JSON was edgeless", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const nodeOnly = buildPuzzle2dSceneDescriptor(<Node draggable id="solo" radius={36} x={0} y={0} />);
      syncPuzzle2dScene(renderer, nodeOnly);
      const edgeless = (renderer as { descriptorJsonForWasmHost(): string }).descriptorJsonForWasmHost();
      expect(JSON.parse(edgeless).edges.length).toBe(0);
      const withEdge = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={10} x={0} y={0}>
            <Handle angle={0} handleKind={BUILTIN_PORT_HANDLE_KIND} id="a:h0" />
          </Node>
          <Node id="b" radius={10} x={40} y={0}>
            <Handle angle={Math.PI} handleKind={BUILTIN_PORT_HANDLE_KIND} id="b:h0" />
          </Node>
          <Edge id="edge-1" source="a:h0" target="b:h0" />
        </>,
      );
      syncPuzzle2dScene(renderer, withEdge);
      expect(renderer.scene.edges.size).toBe(1);
      const withEdgesJson = (renderer as { descriptorJsonForWasmHost(): string }).descriptorJsonForWasmHost();
      expect(JSON.parse(withEdgesJson).edges.length).toBe(1);
      renderer.dispose();
    });

    it("skipSceneSyncIfDescriptorUnchanged does not skip when descriptor lists edges but the scene has none", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const descriptor = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={10} x={0} y={0}>
            <Handle angle={0} handleKind={BUILTIN_PORT_HANDLE_KIND} id="a:h0" />
          </Node>
          <Node id="b" radius={10} x={40} y={0}>
            <Handle angle={Math.PI} handleKind={BUILTIN_PORT_HANDLE_KIND} id="b:h0" />
          </Node>
          <Edge id="edge-1" source="a:h0" target="b:h0" />
        </>,
      );
      renderer.rememberDeclarativeSceneSyncFingerprint(descriptor);
      expect(renderer.scene.edges.size).toBe(0);
      expect(renderer.skipSceneSyncIfDescriptorUnchanged(descriptor)).toBe(false);
      syncPuzzle2dScene(renderer, descriptor);
      expect(renderer.scene.edges.size).toBe(1);
      renderer.dispose();
    });

    it("syncPuzzle2dScene does not skip after host unmount cleared the scene while the descriptor is unchanged", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const descriptor = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={10} x={0} y={0}>
            <Handle angle={0} handleKind={BUILTIN_PORT_HANDLE_KIND} id="a:h0" />
          </Node>
          <Node id="b" radius={10} x={40} y={0}>
            <Handle angle={Math.PI} handleKind={BUILTIN_PORT_HANDLE_KIND} id="b:h0" />
          </Node>
          <Edge id="edge-1" source="a:h0" target="b:h0" />
        </>,
      );
      const hostMount = createPuzzle2dHostMount(renderer);
      syncPuzzle2dScene(renderer, descriptor);
      expect(renderer.scene.edges.size).toBe(1);
      unmountPuzzle2dHostMount(hostMount);
      expect(renderer.scene.edges.size).toBe(0);
      expect(renderer.skipSceneSyncIfDescriptorUnchanged(descriptor)).toBe(false);
      syncPuzzle2dScene(renderer, descriptor);
      expect(renderer.scene.edges.size).toBe(1);
      renderer.dispose();
    });

    it("coalesces nested render calls from frame listeners instead of re-entering the render pass", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
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

  describe("puzzle2d scene", () => {
    it("stores nodes, handles, and edges with stable ids and emits edge creation", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const edgeEvents: Array<{ id: string; source: string; target: string }> = [];
      renderer.on("edgeCreate", (event) => edgeEvents.push(event));

      const sourceNode = new Puzzle2dSceneNode({ id: "source", radius: 36, x: 0, y: 0 });
      const targetNode = new Puzzle2dSceneNode({ id: "target", radius: 36, x: 220, y: 80 });
      const sourceHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new Puzzle2dSceneEdge({ source: sourceHandle, id: "edge-1", target: targetHandle });

      renderer.scene.add(sourceNode).add(targetNode).add(edge);

      expect(renderer.scene.getObjectById("source")).toBe(sourceNode);
      expect(renderer.scene.getObjectById("src-h")).toBe(sourceHandle);
      expect(renderer.scene.getObjectById("edge-1")).toBe(edge);
      expect(edgeEvents).toEqual([{ id: "edge-1", source: "src-h", target: "tgt-h" }]);

      renderer.dispose();
    });

    it("stores wires and drops them when an endpoint handle is removed", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const created: string[] = [];
      const destroyed: string[] = [];
      renderer.on("wireCreate", (e) => created.push(e.id));
      renderer.on("wireDestroy", (e) => destroyed.push(e.id));
      const n = new Puzzle2dSceneNode({ id: "n", radius: 22, x: 0, y: 0 });
      const h = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "h0", node: n });
      const w = new Puzzle2dSceneWire({ endX: 90, endY: 0, id: "w-1", source: h, target: null });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      renderer.setCamera(0, 0, PUZZLE_2D_LOD_DETAIL_MIN_ZOOM);
      const edgeEvents: Array<{ id: string; source: string; target: string }> = [];
      renderer.on("edgeCreate", (event) => edgeEvents.push(event));

      const a = new Puzzle2dSceneNode({ id: "a", radius: 40, x: 0, y: 0 });
      const b = new Puzzle2dSceneNode({ id: "b", radius: 40, x: 280, y: 0 });
      new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: a });
      new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "b:h0", node: b });
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

    it("broadcasts nodeMove and structural deletes to peer renderers", () => {
      const { canvas: canvasA } = createMockCanvas();
      const { canvas: canvasB } = createMockCanvas();
      const rendererA = new Puzzle2dRenderer({ canvas: canvasA, renderMode: "headless-test" });
      const rendererB = new Puzzle2dRenderer({ canvas: canvasB, renderMode: "headless-test" });
      const nodeA = new Puzzle2dSceneNode({ id: "shared", radius: 28, x: 10, y: 20 });
      const nodeB = new Puzzle2dSceneNode({ id: "shared", radius: 28, x: 10, y: 20 });
      rendererA.scene.add(nodeA);
      rendererB.scene.add(nodeB);
      rendererB["pushSceneToWasmDriver"]();
      const syncDescriptorJson = vi.spyOn(rendererB.session, "syncDescriptorJson");
      const setNodePositionsJson = vi.spyOn(rendererB.session, "setNodePositionsJson");
      rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "nodeMove", payload: { id: "shared", x: 90, y: 110 } }]));
      rendererB["pushSceneToWasmDriver"]();
      expect(nodeA.x).toBe(90);
      expect(nodeB.x).toBe(90);
      expect(nodeB.y).toBe(110);
      expect(setNodePositionsJson).toHaveBeenCalledTimes(1);
      expect(setNodePositionsJson.mock.calls[0]?.[0]).toContain('"id":"shared"');
      expect(syncDescriptorJson).not.toHaveBeenCalled();
      const nodeDeletes: string[] = [];
      rendererA.on("nodeDelete", (event) => nodeDeletes.push(event.id));
      rendererA["pushSceneToWasmDriver"]();
      rendererA.setSelectionIdsSilent(["shared"]);
      rendererA["deleteSelectedObjects"]();
      expect(nodeDeletes).toContain("shared");
      expect(rendererA.scene.nodes.has("shared")).toBe(false);
      expect(rendererB.scene.nodes.has("shared")).toBe(false);
      rendererA.dispose();
      rendererB.dispose();
    });

    it("broadcasts edge deletes to peer renderers after authoritative wasm drain", () => {
      const { canvas: canvasA } = createMockCanvas();
      const { canvas: canvasB } = createMockCanvas();
      const rendererA = new Puzzle2dRenderer({ canvas: canvasA, renderMode: "headless-test" });
      const rendererB = new Puzzle2dRenderer({ canvas: canvasB, renderMode: "headless-test" });
      const sourceNodeA = new Puzzle2dSceneNode({ id: "a", radius: 10, x: 0, y: 0 });
      const targetNodeA = new Puzzle2dSceneNode({ id: "b", radius: 10, x: 40, y: 0 });
      const sourceNodeB = new Puzzle2dSceneNode({ id: "a", radius: 10, x: 0, y: 0 });
      const targetNodeB = new Puzzle2dSceneNode({ id: "b", radius: 10, x: 40, y: 0 });
      const sourceHandleA = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: sourceNodeA });
      const targetHandleA = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "b:h0", node: targetNodeA });
      const sourceHandleB = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: sourceNodeB });
      const targetHandleB = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "b:h0", node: targetNodeB });
      const edgeA = new Puzzle2dSceneEdge({ id: "edge-1", source: sourceHandleA, target: targetHandleA });
      const edgeB = new Puzzle2dSceneEdge({ id: "edge-1", source: sourceHandleB, target: targetHandleB });
      rendererA.scene.add(sourceNodeA).add(targetNodeA).add(edgeA);
      rendererB.scene.add(sourceNodeB).add(targetNodeB).add(edgeB);
      const descriptor = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={10} x={0} y={0}>
            <Handle angle={0} handleKind={BUILTIN_PORT_HANDLE_KIND} id="a:h0" />
          </Node>
          <Node id="b" radius={10} x={40} y={0}>
            <Handle angle={Math.PI} handleKind={BUILTIN_PORT_HANDLE_KIND} id="b:h0" />
          </Node>
          <Edge id="edge-1" source="a:h0" target="b:h0" />
        </>,
      );
      rendererA.rememberHostDeclarativeSceneDescriptor(descriptor);
      rendererB.rememberHostDeclarativeSceneDescriptor(descriptor);
      rendererA.setDeclarativeSceneEdgeExpectation(descriptor.edges.length);
      rendererB.setDeclarativeSceneEdgeExpectation(descriptor.edges.length);
      rendererA.withFixtureStructuralDeleteMirror(() => {
        rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "edgeDelete", payload: { id: "edge-1" } }]));
      });
      expect(rendererA.scene.edges.has("edge-1")).toBe(false);
      expect(rendererB.scene.edges.has("edge-1")).toBe(false);
      rendererA["applyWasmDrainToScene"](JSON.stringify([]), { silentStructuralRemoves: true });
      expect(rendererA.scene.edges.has("edge-1")).toBe(false);
      expect(rendererB.scene.edges.has("edge-1")).toBe(false);
      rendererA.dispose();
      rendererB.dispose();
    });

    it("ending area-select defer without structural mutation does not force full syncDescriptorJson", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      const syncDescriptorJson = vi.spyOn(renderer.session, "syncDescriptorJson");
      renderer["updatePreselection"](["n"], [], true);
      renderer["pushSceneToWasmDriver"]();
      syncDescriptorJson.mockClear();
      renderer["updatePreselection"]([], [], false);
      renderer["updateSelection"](["n"], false);
      renderer["pushSceneToWasmDriver"]();
      expect(syncDescriptorJson).not.toHaveBeenCalled();
      renderer.dispose();
    });

    it("broadcasts selection to peer renderers without full syncDescriptorJson", () => {
      const { canvas: canvasA } = createMockCanvas();
      const { canvas: canvasB } = createMockCanvas();
      const rendererA = new Puzzle2dRenderer({ canvas: canvasA, renderMode: "headless-test" });
      const rendererB = new Puzzle2dRenderer({ canvas: canvasB, renderMode: "headless-test" });
      const nodeA = new Puzzle2dSceneNode({ id: "shared", radius: 28, x: 0, y: 0 });
      const nodeB = new Puzzle2dSceneNode({ id: "shared", radius: 28, x: 0, y: 0 });
      rendererA.scene.add(nodeA);
      rendererB.scene.add(nodeB);
      rendererA["pushSceneToWasmDriver"]();
      rendererB["pushSceneToWasmDriver"]();
      const syncDescriptorJson = vi.spyOn(rendererB.session, "syncDescriptorJson");
      rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "select", payload: { ids: ["shared"] } }]));
      expect(rendererA.selection.getSnapshot().ids).toEqual(["shared"]);
      expect(rendererB.selection.getSnapshot().ids).toEqual(["shared"]);
      expect(syncDescriptorJson).not.toHaveBeenCalled();
      rendererA.dispose();
      rendererB.dispose();
    });

    it("clears stale peer preselect before applying committed deselect chrome", () => {
      const { canvas: canvasA } = createMockCanvas();
      const { canvas: canvasB } = createMockCanvas();
      const rendererA = new Puzzle2dRenderer({ canvas: canvasA, renderMode: "headless-test" });
      const rendererB = new Puzzle2dRenderer({ canvas: canvasB, renderMode: "headless-test" });
      const nodeA = new Puzzle2dSceneNode({ id: "a", radius: 28, x: 0, y: 0 });
      const nodeB = new Puzzle2dSceneNode({ id: "b", radius: 28, x: 80, y: 0 });
      const peerA = new Puzzle2dSceneNode({ id: "a", radius: 28, x: 0, y: 0 });
      const peerB = new Puzzle2dSceneNode({ id: "b", radius: 28, x: 80, y: 0 });
      rendererA.scene.add(nodeA).add(nodeB);
      rendererB.scene.add(peerA).add(peerB);
      rendererA["pushSceneToWasmDriver"]();
      rendererB["pushSceneToWasmDriver"]();
      rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "preselect", payload: { ids: ["a", "b"], removedIds: [] } }]));
      rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "select", payload: { ids: ["a", "b"] } }]));
      expect(rendererB.selection.getSnapshot().ids).toEqual(["a", "b"]);
      expect(rendererB.preselection.getSnapshot()).toEqual(PUZZLE_2D_PRESELECT_EMPTY);
      expect(peerA.selected).toBe(true);
      expect(peerB.selected).toBe(true);
      rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "select", payload: { ids: [] } }]));
      rendererB["pushSceneToWasmDriver"]();
      expect(rendererB.selection.getSnapshot().ids).toEqual([]);
      expect(rendererB.preselection.getSnapshot()).toEqual(PUZZLE_2D_PRESELECT_EMPTY);
      expect(peerA.selected).toBe(false);
      expect(peerB.selected).toBe(false);
      rendererA.dispose();
      rendererB.dispose();
    });

    it("clears stale peer preselect when committed selection is broadcast after area-select", () => {
      const { canvas: canvasA } = createMockCanvas();
      const { canvas: canvasB } = createMockCanvas();
      const rendererA = new Puzzle2dRenderer({ canvas: canvasA, renderMode: "headless-test" });
      const rendererB = new Puzzle2dRenderer({ canvas: canvasB, renderMode: "headless-test" });
      const nodeA = new Puzzle2dSceneNode({ id: "solo", radius: 36, x: 0, y: 0 });
      const nodeB = new Puzzle2dSceneNode({ id: "solo", radius: 36, x: 0, y: 0 });
      rendererA.scene.add(nodeA);
      rendererB.scene.add(nodeB);
      rendererA["pushSceneToWasmDriver"]();
      rendererB["pushSceneToWasmDriver"]();
      rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "preselect", payload: { ids: ["solo"], removedIds: [] } }]));
      expect(rendererB.getPreselectSnapshot().ids).toEqual(["solo"]);
      rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "select", payload: { ids: ["solo"] } }]));
      expect(rendererB.selection.getSnapshot().ids).toEqual(["solo"]);
      expect(rendererB.getPreselectSnapshot()).toEqual(PUZZLE_2D_PRESELECT_EMPTY);
      expect(nodeB.selected).toBe(true);
      rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "select", payload: { ids: [] } }]));
      expect(rendererB.selection.getSnapshot().ids).toEqual([]);
      expect(rendererB.getPreselectSnapshot()).toEqual(PUZZLE_2D_PRESELECT_EMPTY);
      expect(nodeB.selected).toBe(false);
      rendererA.dispose();
      rendererB.dispose();
    });

    it("background deselect on one pane clears committed selection on peer renderers", () => {
      const { canvas: canvasA } = createMockCanvas();
      const { canvas: canvasB } = createMockCanvas();
      const rendererA = new Puzzle2dRenderer({ canvas: canvasA, renderMode: "headless-test" });
      const rendererB = new Puzzle2dRenderer({ canvas: canvasB, renderMode: "headless-test" });
      const nodeA = new Puzzle2dSceneNode({ draggable: true, id: "solo", radius: 36, x: 0, y: 0 });
      const nodeB = new Puzzle2dSceneNode({ draggable: true, id: "solo", radius: 36, x: 0, y: 0 });
      rendererA.scene.add(nodeA);
      rendererB.scene.add(nodeB);
      rendererA.render();
      rendererB["pushSceneToWasmDriver"]();
      const onNode = rendererA.worldToScreen({ x: 0, y: 0 });
      canvasA.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: onNode.x, clientY: onNode.y }));
      canvasA.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: onNode.x, clientY: onNode.y }));
      expect(rendererB.selection.getSnapshot().ids).toEqual(["solo"]);
      const background = rendererA.worldToScreen({ x: 900, y: 900 });
      canvasA.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: background.x, clientY: background.y }));
      canvasA.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: background.x, clientY: background.y }));
      expect(rendererA.selection.getSnapshot().ids).toEqual([]);
      expect(rendererB.selection.getSnapshot().ids).toEqual([]);
      expect(nodeB.selected).toBe(false);
      rendererA.dispose();
      rendererB.dispose();
    });

    it("setSelectionIdsSilent uses setSelectionIdsJsonSilent without full syncDescriptorJson", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      const syncDescriptorJson = vi.spyOn(renderer.session, "syncDescriptorJson");
      renderer.setSelectionIdsSilent(["n"]);
      renderer["pushSceneToWasmDriver"]();
      expect(syncDescriptorJson).not.toHaveBeenCalled();
      renderer.dispose();
    });

    it("textOverlayDirty after live node drag geometry", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption" });
      renderer.scene.add(node);
      renderer.render();
      renderer["rememberTextOverlayPainted"]({ ...renderer.camera }, renderer.effectiveDrawLodLabel());
      expect(renderer.textOverlayDirty()).toBe(false);
      renderer["applyWasmDrainToScene"](JSON.stringify([{ name: "nodeMove", payload: { id: "n", x: 40, y: 30 } }]));
      expect(node.x).toBe(40);
      expect(renderer.textOverlayDirty()).toBe(true);
      renderer.dispose();
    });

    it("textOverlayDirty stays false across hover-only updates", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "label" });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      renderer["rememberTextOverlayPainted"]({ ...renderer.camera }, renderer.effectiveDrawLodLabel());
      expect(renderer.textOverlayDirty()).toBe(false);
      renderer["applyWasmDrainToScene"](JSON.stringify([{ name: "hover", payload: { id: "n" } }]));
      expect(renderer.textOverlayDirty()).toBe(false);
      renderer.dispose();
    });

    it("textOverlayDirty after camera, selection chrome, content epoch, and theme changes", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "label" });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      renderer["rememberTextOverlayPainted"]({ ...renderer.camera }, renderer.effectiveDrawLodLabel());
      expect(renderer.textOverlayDirty()).toBe(false);
      renderer.camera.x = 40;
      expect(renderer.textOverlayDirty()).toBe(true);
      renderer["rememberTextOverlayPainted"]({ ...renderer.camera }, renderer.effectiveDrawLodLabel());
      renderer.setSelectionIdsSilent(["n"]);
      expect(renderer.textOverlayDirty()).toBe(true);
      renderer["rememberTextOverlayPainted"]({ ...renderer.camera }, renderer.effectiveDrawLodLabel());
      expect(renderer.textOverlayDirty()).toBe(false);
      renderer.markDirty();
      expect(renderer.textOverlayDirty()).toBe(true);
      renderer["rememberTextOverlayPainted"]({ ...renderer.camera }, renderer.effectiveDrawLodLabel());
      renderer["lastVelloThemeJson"] = '{"changed":true}';
      expect(renderer.textOverlayDirty()).toBe(true);
      renderer.dispose();
    });

    it("paintTextOverlays repaints when selection chrome changes", () => {
      const { canvas } = createMockCanvas();
      const overlay = document.createElement("canvas");
      const fillText = vi.fn();
      const overlayCtx: Puzzle2dCanvasContext = {
        arc: vi.fn(),
        beginPath: vi.fn(),
        bezierCurveTo: vi.fn(),
        clearRect: vi.fn(),
        clip: vi.fn(),
        closePath: vi.fn(),
        fill: vi.fn(),
        fillRect: vi.fn(),
        fillStyle: "#000000",
        fillText,
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
        textAlign: "start",
        textBaseline: "alphabetic",
      };
      Object.defineProperty(overlay, "getContext", { configurable: true, value: () => overlayCtx });
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "main-thread" });
      renderer.attachTextOverlayCanvas(overlay);
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption" });
      renderer.scene.add(node);
      renderer.setCamera(0, 0, 1);
      renderer["paintTextOverlays"]();
      expect(fillText).toHaveBeenCalled();
      fillText.mockClear();
      renderer.setSelectionIdsSilent(["n"]);
      renderer["paintTextOverlays"]();
      expect(fillText).toHaveBeenCalled();
      renderer.dispose();
    });

    it("textOverlayDirty when camera changes during wheel zoom", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption" });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      renderer["rememberTextOverlayPainted"]({ ...renderer.camera }, renderer.effectiveDrawLodLabel());
      renderer["wheelZoomGestureActive"] = true;
      renderer.camera.x = 80;
      expect(renderer.textOverlayDirty()).toBe(true);
      renderer.dispose();
    });

    it("paintTextOverlays repaints during wheel zoom", () => {
      const { canvas } = createMockCanvas();
      const overlay = document.createElement("canvas");
      const fillText = vi.fn();
      const overlayCtx: Puzzle2dCanvasContext = {
        arc: vi.fn(),
        beginPath: vi.fn(),
        bezierCurveTo: vi.fn(),
        clearRect: vi.fn(),
        clip: vi.fn(),
        closePath: vi.fn(),
        fill: vi.fn(),
        fillRect: vi.fn(),
        fillStyle: "#000000",
        fillText,
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
        textAlign: "start",
        textBaseline: "alphabetic",
      };
      Object.defineProperty(overlay, "getContext", { configurable: true, value: () => overlayCtx });
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "main-thread" });
      renderer.attachTextOverlayCanvas(overlay);
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption" });
      renderer.scene.add(node);
      renderer.setCamera(0, 0, 1);
      renderer["paintTextOverlays"]();
      expect(fillText).toHaveBeenCalled();
      fillText.mockClear();
      renderer["wheelZoomGestureActive"] = true;
      renderer.setCamera(120, 0, 2);
      renderer["paintTextOverlays"]();
      expect(fillText).toHaveBeenCalled();
      renderer.dispose();
    });

    it("acceptsHostCameraProp is false during wheel zoom and deferred WASM gestures", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      expect(renderer.acceptsHostCameraProp()).toBe(true);
      renderer["wheelZoomGestureActive"] = true;
      expect(renderer.acceptsHostCameraProp()).toBe(false);
      renderer["wheelZoomGestureActive"] = false;
      vi.spyOn(renderer.session, "defersDescriptorSyncFromJs").mockReturnValue(true);
      expect(renderer.acceptsHostCameraProp()).toBe(false);
      renderer.dispose();
    });

    it("paintTextOverlays uses wasm node centers when they differ from the js scene", () => {
      const { canvas } = createMockCanvas();
      const overlay = document.createElement("canvas");
      const fillText = vi.fn();
      const overlayCtx: Puzzle2dCanvasContext = {
        arc: vi.fn(),
        beginPath: vi.fn(),
        bezierCurveTo: vi.fn(),
        clearRect: vi.fn(),
        clip: vi.fn(),
        closePath: vi.fn(),
        fill: vi.fn(),
        fillRect: vi.fn(),
        fillStyle: "#000000",
        fillText,
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
        textAlign: "start",
        textBaseline: "alphabetic",
      };
      Object.defineProperty(overlay, "getContext", { configurable: true, value: () => overlayCtx });
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "main-thread" });
      renderer.attachTextOverlayCanvas(overlay);
      renderer.setSize(800, 600, 1);
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption", textAutofit: true });
      renderer.scene.add(node);
      const overlayCamera = { x: 0, y: 0, zoom: 1 };
      vi.spyOn(renderer.session, "overlayPaintStateJson").mockReturnValue(
        JSON.stringify({
          camera: overlayCamera,
          lod: "normal",
          nodes: [{ id: "n", x: 200, y: 100 }],
        }),
      );
      renderer.setCamera(overlayCamera.x, overlayCamera.y, overlayCamera.zoom);
      renderer["paintTextOverlays"]();
      expect(fillText).toHaveBeenCalled();
      const [, labelX, labelY] = fillText.mock.calls[0] as [string, number, number];
      const originScreen = renderer["worldToScreenWithCamera"]({ x: 0, y: 0 }, overlayCamera);
      const wasmScreen = renderer["worldToScreenWithCamera"]({ x: 200, y: 100 }, overlayCamera);
      expect(labelX).toBe(wasmScreen.x);
      expect(labelY).toBe(wasmScreen.y);
      expect(labelX).not.toBe(originScreen.x);
      expect(labelY).not.toBe(originScreen.y);
      renderer.dispose();
    });

    it("pushWasmViewportAndSizeToSession does not stomp wasm camera during wheel zoom", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption" });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      const setCamera = vi.spyOn(renderer.session, "setCamera");
      renderer.camera.x = 0;
      renderer.camera.y = 0;
      renderer.camera.zoom = 1;
      renderer["lastPushedCameraX"] = 0;
      renderer["lastPushedCameraY"] = 0;
      renderer["lastPushedCameraZoom"] = 1;
      renderer["wheelZoomGestureActive"] = true;
      renderer["wasmViewportLeading"] = true;
      vi.spyOn(renderer.session, "cameraJson").mockReturnValue(JSON.stringify({ x: 50, y: 40, zoom: 2 }));
      renderer["pushWasmViewportAndSizeToSession"]();
      expect(setCamera).not.toHaveBeenCalled();
      renderer.dispose();
    });

    it("pushSceneToWasmDriver does not stomp wasm camera when host owns viewport", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption" });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      const setCamera = vi.spyOn(renderer.session, "setCamera");
      setCamera.mockClear();
      renderer.camera.x = 0;
      renderer.camera.y = 0;
      renderer.camera.zoom = 1;
      renderer["wheelZoomGestureActive"] = true;
      renderer["wasmViewportLeading"] = true;
      renderer["pushSceneToWasmDriver"]();
      expect(setCamera).not.toHaveBeenCalled();
      renderer.dispose();
    });

    it("setCameraSilent does not push to wasm while viewport is wasm-led", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const setCamera = vi.spyOn(renderer.session, "setCamera");
      renderer["wasmViewportLeading"] = true;
      renderer.setCameraSilent(80, 0, 2);
      expect(setCamera).not.toHaveBeenCalled();
      renderer.dispose();
    });

    it("flushPendingWheelScreen keeps pending wheel when wasm borrow blocks", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      renderer["wasmGpuFrameDepth"] = 1;
      renderer["pendingWheelScreen"] = { sx: 1, sy: 2, deltaY: -40 };
      const wheelScreen = vi.spyOn(renderer.session, "wheelScreen");
      renderer["flushPendingWheelScreen"]();
      expect(wheelScreen).not.toHaveBeenCalled();
      expect(renderer["pendingWheelScreen"]).toEqual({ sx: 1, sy: 2, deltaY: -40 });
      renderer.dispose();
    });

    it("textOverlayDirty is true when wasm camera diverges from last painted overlay", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption" });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      renderer["rememberTextOverlayPainted"]({ x: 0, y: 0, zoom: 1 }, "normal");
      renderer.camera.x = 0;
      renderer.camera.y = 0;
      renderer.camera.zoom = 1;
      vi.spyOn(renderer.session, "cameraJson").mockReturnValue(JSON.stringify({ x: 80, y: 0, zoom: 2 }));
      renderer["wasmViewportLeading"] = true;
      expect(renderer.textOverlayDirty()).toBe(true);
      renderer.dispose();
    });

    it("paintTextOverlays repaints when syncedGpuThisFrame even if overlay cache is clean", () => {
      const { canvas } = createMockCanvas();
      const overlay = document.createElement("canvas");
      const fillText = vi.fn();
      const overlayCtx: Puzzle2dCanvasContext = {
        arc: vi.fn(),
        beginPath: vi.fn(),
        bezierCurveTo: vi.fn(),
        clearRect: vi.fn(),
        clip: vi.fn(),
        closePath: vi.fn(),
        fill: vi.fn(),
        fillRect: vi.fn(),
        fillStyle: "#000000",
        fillText,
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
        textAlign: "start",
        textBaseline: "alphabetic",
      };
      Object.defineProperty(overlay, "getContext", { configurable: true, value: () => overlayCtx });
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "main-thread" });
      renderer.attachTextOverlayCanvas(overlay);
      renderer.setSize(800, 600, 1);
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption", textAutofit: true });
      renderer.scene.add(node);
      const overlayCamera = { x: 0, y: 0, zoom: 1 };
      vi.spyOn(renderer.session, "overlayPaintStateJson").mockReturnValue(
        JSON.stringify({
          camera: overlayCamera,
          lod: "normal",
          nodes: [{ id: "n", x: 0, y: 0 }],
        }),
      );
      renderer.setCamera(overlayCamera.x, overlayCamera.y, overlayCamera.zoom);
      renderer["paintTextOverlays"](false);
      expect(fillText).toHaveBeenCalledTimes(1);
      fillText.mockClear();
      renderer["rememberTextOverlayPainted"](overlayCamera, "normal");
      expect(renderer.textOverlayDirty()).toBe(false);
      vi.spyOn(renderer.session, "overlayPaintStateJson").mockReturnValue(
        JSON.stringify({
          camera: { x: 40, y: 0, zoom: 2 },
          lod: "normal",
          nodes: [{ id: "n", x: 0, y: 0 }],
        }),
      );
      renderer["wasmViewportLeading"] = true;
      renderer["paintTextOverlays"](true);
      expect(fillText).toHaveBeenCalledTimes(1);
      renderer.dispose();
    });

    it("render flushes pending wheel before GPU and text overlay", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const wheelScreen = vi.spyOn(renderer.session, "wheelScreen");
      renderer["pendingWheelScreen"] = { sx: 10, sy: 20, deltaY: -120 };
      renderer.render(0);
      expect(wheelScreen).toHaveBeenCalledWith(10, 20, -120);
      expect(renderer["pendingWheelScreen"]).toBeNull();
      renderer.dispose();
    });

    it("clearWasmViewportLeadingIfHostCameraMatches blocks stale host camera until props catch up", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      renderer.setCameraSilent(50, 40, 2);
      renderer["wasmViewportLeading"] = true;
      expect(renderer.clearWasmViewportLeadingIfHostCameraMatches({ x: 0, y: 0, zoom: 1 })).toBe(false);
      expect(renderer.clearWasmViewportLeadingIfHostCameraMatches({ x: 50, y: 40, zoom: 2 })).toBe(true);
      expect(renderer["wasmViewportLeading"]).toBe(false);
      renderer.dispose();
    });

    it("paintTextOverlays repaints selection chrome during wheel zoom", () => {
      const { canvas } = createMockCanvas();
      const overlay = document.createElement("canvas");
      const fillText = vi.fn();
      const overlayCtx: Puzzle2dCanvasContext = {
        arc: vi.fn(),
        beginPath: vi.fn(),
        bezierCurveTo: vi.fn(),
        clearRect: vi.fn(),
        clip: vi.fn(),
        closePath: vi.fn(),
        fill: vi.fn(),
        fillRect: vi.fn(),
        fillStyle: "#000000",
        fillText,
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
        textAlign: "start",
        textBaseline: "alphabetic",
      };
      Object.defineProperty(overlay, "getContext", { configurable: true, value: () => overlayCtx });
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "main-thread" });
      renderer.attachTextOverlayCanvas(overlay);
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0, text: "caption" });
      renderer.scene.add(node);
      renderer.setCamera(0, 0, 1);
      renderer["paintTextOverlays"]();
      fillText.mockClear();
      renderer["wheelZoomGestureActive"] = true;
      renderer.setSelectionIdsSilent(["n"]);
      renderer["paintTextOverlays"]();
      expect(fillText).toHaveBeenCalled();
      renderer.dispose();
    });

    it("broadcasts preselect to peer renderers without full syncDescriptorJson", () => {
      const { canvas: canvasA } = createMockCanvas();
      const { canvas: canvasB } = createMockCanvas();
      const rendererA = new Puzzle2dRenderer({ canvas: canvasA, renderMode: "headless-test" });
      const rendererB = new Puzzle2dRenderer({ canvas: canvasB, renderMode: "headless-test" });
      const nodeA = new Puzzle2dSceneNode({ id: "a", radius: 24, x: 0, y: 0 });
      const nodeB = new Puzzle2dSceneNode({ id: "b", radius: 24, x: 80, y: 0 });
      rendererA.scene.add(nodeA).add(nodeB);
      rendererB.scene.add(new Puzzle2dSceneNode({ id: "a", radius: 24, x: 0, y: 0 })).add(new Puzzle2dSceneNode({ id: "b", radius: 24, x: 80, y: 0 }));
      rendererA["pushSceneToWasmDriver"]();
      rendererB["pushSceneToWasmDriver"]();
      const syncDescriptorJson = vi.spyOn(rendererB.session, "syncDescriptorJson");
      rendererA.setSelectionIdsSilent(["a"]);
      rendererA["applyWasmDrainToScene"](JSON.stringify([{ name: "preselect", payload: { ids: ["b"], removedIds: [] } }]));
      expect(rendererB.preselection.getSnapshot().ids).toEqual(["b"]);
      expect(syncDescriptorJson).not.toHaveBeenCalled();
      rendererA.dispose();
      rendererB.dispose();
    });

    it("setSelectionIdsSilent does not enqueue graph observation change events", async () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      await Promise.resolve();
      const graphEvents: string[] = [];
      const off = renderer.on("change", () => graphEvents.push("change"));
      renderer.setSelectionIdsSilent(["n"]);
      await Promise.resolve();
      expect(graphEvents).toEqual([]);
      off();
      renderer.dispose();
    });

    it("wasm drain select does not enqueue graph observation change events", async () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      await Promise.resolve();
      const graphEvents: string[] = [];
      const off = renderer.on("change", () => graphEvents.push("change"));
      renderer["applyWasmDrainToScene"](JSON.stringify([{ name: "select", payload: { ids: ["n"] } }]));
      await Promise.resolve();
      expect(graphEvents).toEqual([]);
      expect([...renderer.selectionIds]).toEqual(["n"]);
      off();
      renderer.dispose();
    });

    it("skips full syncDescriptorJson when only the camera changes", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 24, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer["pushSceneToWasmDriver"]();
      const syncDescriptorJson = vi.spyOn(renderer.session, "syncDescriptorJson");
      const cameraEvents: CameraState[] = [];
      renderer.on("camera", (cam) => cameraEvents.push(cam));
      renderer["applyWasmDrainToScene"](JSON.stringify([{ name: "camera", payload: { x: 12, y: 34, zoom: 1.5 } }]), { silentCamera: true });
      renderer["wasmPushSceneDrainAlreadyApplied"] = true;
      renderer["pushSceneToWasmDriver"]();
      expect(renderer.camera.x).toBe(12);
      expect(renderer.camera.zoom).toBe(1.5);
      expect(syncDescriptorJson).not.toHaveBeenCalled();
      expect(cameraEvents).toEqual([]);
      renderer["emitPublicCameraChange"]();
      expect(cameraEvents.length).toBe(1);
      renderer.dispose();
    });

    it("wasm drain nodeDelete for missing scene ids does not emit structural delete events", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const nodeDeletes: string[] = [];
      renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));
      const root = new Puzzle2dSceneNode({ id: "keep", radius: 24, x: 0, y: 0 });
      renderer.scene.add(root);
      renderer["applyWasmDrainToScene"](
        JSON.stringify([{ name: "nodeDelete", payload: { id: "ghost" } }]),
      );
      expect(nodeDeletes).toEqual([]);
      expect(renderer.scene.nodes.has("keep")).toBe(true);
      renderer.dispose();
    });

    it("wasm drain nodeDelete for an existing scene id does not emit structural delete events", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const nodeDeletes: string[] = [];
      renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));
      const root = new Puzzle2dSceneNode({ id: "keep", radius: 24, x: 0, y: 0 });
      renderer.scene.add(root);
      renderer["applyWasmDrainToScene"](JSON.stringify([{ name: "nodeDelete", payload: { id: "keep" } }]), { silentStructuralRemoves: true });
      expect(nodeDeletes).toEqual([]);
      expect(renderer.scene.nodes.has("keep")).toBe(false);
      renderer.dispose();
    });

    it("wasm drain edgeDelete with silentStructuralRemoves restores edges from host declarative snapshot", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const descriptor = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={10} x={0} y={0}>
            <Handle angle={0} handleKind={BUILTIN_PORT_HANDLE_KIND} id="a:h0" />
          </Node>
          <Node id="b" radius={10} x={40} y={0}>
            <Handle angle={Math.PI} handleKind={BUILTIN_PORT_HANDLE_KIND} id="b:h0" />
          </Node>
          <Edge id="edge-1" source="a:h0" target="b:h0" />
        </>,
      );
      syncPuzzle2dScene(renderer, descriptor);
      renderer.rememberHostDeclarativeSceneDescriptor(descriptor);
      renderer.setDeclarativeSceneEdgeExpectation(descriptor.edges.length);
      const edgeDeletes: string[] = [];
      renderer.on("edgeDelete", (event) => edgeDeletes.push(event.id));
      renderer["applyWasmDrainToScene"](JSON.stringify([{ name: "edgeDelete", payload: { id: "edge-1" } }]), { silentStructuralRemoves: true });
      expect(edgeDeletes).toEqual([]);
      expect(renderer.scene.edges.size).toBe(1);
      expect(renderer.scene.edges.has("edge-1")).toBe(true);
      renderer.dispose();
    });

    it("puzzle2dEnsureSceneEdgesFromDescriptor does not resurrect authoritatively deleted edges", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const descriptor = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={10} x={0} y={0}>
            <Handle angle={0} handleKind={BUILTIN_PORT_HANDLE_KIND} id="a:h0" />
          </Node>
          <Node id="b" radius={10} x={40} y={0}>
            <Handle angle={Math.PI} handleKind={BUILTIN_PORT_HANDLE_KIND} id="b:h0" />
          </Node>
          <Edge id="edge-1" source="a:h0" target="b:h0" />
        </>,
      );
      syncPuzzle2dScene(renderer, descriptor);
      renderer.withFixtureStructuralDeleteMirror(() => {
        renderer["applyWasmDrainToScene"](JSON.stringify([{ name: "edgeDelete", payload: { id: "edge-1" } }]));
      });
      expect(renderer.scene.edges.has("edge-1")).toBe(false);
      puzzle2dEnsureSceneEdgesFromDescriptor(renderer, descriptor);
      expect(renderer.scene.edges.has("edge-1")).toBe(false);
      renderer.dispose();
    });

    it("authoritative edge delete prunes host declarative snapshot so silent zoom drains do not restore edges", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const descriptor = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={10} x={0} y={0}>
            <Handle angle={0} handleKind={BUILTIN_PORT_HANDLE_KIND} id="a:h0" />
          </Node>
          <Node id="b" radius={10} x={40} y={0}>
            <Handle angle={Math.PI} handleKind={BUILTIN_PORT_HANDLE_KIND} id="b:h0" />
          </Node>
          <Edge id="edge-1" source="a:h0" target="b:h0" />
        </>,
      );
      syncPuzzle2dScene(renderer, descriptor);
      renderer.rememberHostDeclarativeSceneDescriptor(descriptor);
      renderer.setDeclarativeSceneEdgeExpectation(descriptor.edges.length);
      renderer.withFixtureStructuralDeleteMirror(() => {
        renderer["applyWasmDrainToScene"](JSON.stringify([{ name: "edgeDelete", payload: { id: "edge-1" } }]));
      });
      expect(renderer.scene.edges.has("edge-1")).toBe(false);
      renderer["applyWasmDrainToScene"](JSON.stringify([]), { silentStructuralRemoves: true });
      expect(renderer.scene.edges.has("edge-1")).toBe(false);
      renderer.dispose();
    });

    it("dispose does not emit structural delete events that would clear play fixtures", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const nodeDeletes: string[] = [];
      renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));
      const root = new Puzzle2dSceneNode({ id: "keep", radius: 24, x: 0, y: 0 });
      renderer.scene.add(root);
      renderer.dispose();
      expect(nodeDeletes).toEqual([]);
    });

    it("scene remove does not emit structural delete without fixture mirror", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const edgeDeletes: string[] = [];
      renderer.on("edgeDelete", (event) => edgeDeletes.push(event.id));
      const sourceNode = new Puzzle2dSceneNode({ id: "a", radius: 10, x: 0, y: 0 });
      const targetNode = new Puzzle2dSceneNode({ id: "b", radius: 10, x: 40, y: 0 });
      const sourceHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: sourceNode });
      const targetHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "b:h0", node: targetNode });
      const edge = new Puzzle2dSceneEdge({ id: "e1", source: sourceHandle, target: targetHandle });
      renderer.scene.add(sourceNode).add(targetNode).add(edge);
      renderer.scene.remove(edge);
      expect(edgeDeletes).toEqual([]);
      expect(renderer.mirrorsStructuralDeletesToFixture()).toBe(false);
      renderer.dispose();
    });

    it("scene remove emits structural delete inside fixture mirror", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const edgeDeletes: string[] = [];
      renderer.on("edgeDelete", (event) => edgeDeletes.push(event.id));
      const sourceNode = new Puzzle2dSceneNode({ id: "a", radius: 10, x: 0, y: 0 });
      const targetNode = new Puzzle2dSceneNode({ id: "b", radius: 10, x: 40, y: 0 });
      const sourceHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "a:h0", node: sourceNode });
      const targetHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "b:h0", node: targetNode });
      const edge = new Puzzle2dSceneEdge({ id: "e1", source: sourceHandle, target: targetHandle });
      renderer.scene.add(sourceNode).add(targetNode).add(edge);
      renderer.withFixtureStructuralDeleteMirror(() => {
        renderer.scene.remove(edge);
      });
      expect(edgeDeletes).toEqual(["e1"]);
      renderer.dispose();
    });

    it("deletes selected edges and nodes when Delete reaches the window listener after pointerdown", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const edgeDeletes: string[] = [];
      const nodeDeletes: string[] = [];
      renderer.on("edgeDelete", (event) => edgeDeletes.push(event.id));
      renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));

      const sourceNode = new Puzzle2dSceneNode({ id: "source", radius: 36, x: 0, y: 0 });
      const targetNode = new Puzzle2dSceneNode({ id: "target", radius: 36, x: 220, y: 0 });
      const sourceHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new Puzzle2dSceneEdge({ source: sourceHandle, id: "edge-1", target: targetHandle });
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

    it("does not delete the puzzle 2d selection while a text field owns focus", () => {
      const { canvas } = createMockCanvas();
      document.body.appendChild(canvas);
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const sourceNode = new Puzzle2dSceneNode({ id: "source", radius: 36, x: 0, y: 0 });
      const targetNode = new Puzzle2dSceneNode({ id: "target", radius: 36, x: 220, y: 0 });
      const sourceHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new Puzzle2dSceneEdge({ source: sourceHandle, id: "edge-1", target: targetHandle });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const movableNode = new Puzzle2dSceneNode({ draggable: true, id: "movable", radius: 30, x: 0, y: 0 });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const a = new Puzzle2dSceneNode({ draggable: true, id: "a", radius: 20, x: 0, y: 0 });
      const b = new Puzzle2dSceneNode({ draggable: true, id: "b", radius: 20, x: 100, y: 0 });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "n", radius: 10, x: 10, y: 20 });
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

    it("puzzle2dElementInteractionChrome maps preselect preview and anchor removal", () => {
      expect(puzzle2dElementInteractionChrome(["a"], { ids: ["a", "b"], removedIds: ["a"] })).toEqual({
        highlightedIds: new Set(["a"]),
        selectedIds: new Set(["b"]),
      });
      expect(puzzle2dElementInteractionChrome(["a"], PUZZLE_2D_PRESELECT_EMPTY)).toEqual({
        highlightedIds: new Set(),
        selectedIds: new Set(["a"]),
      });
    });

    it("applies imperative selection via setSelectionIds", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const sourceNode = new Puzzle2dSceneNode({ id: "source", radius: 20, x: 0, y: 0 });
      const targetNode = new Puzzle2dSceneNode({ id: "target", radius: 20, x: 100, y: 0 });
      renderer.scene.add(sourceNode).add(targetNode);
      renderer.setSelectionIds(["target"]);
      expect(renderer.selection.getSnapshot().ids).toEqual(["target"]);
      expect(targetNode.selected).toBe(true);
      expect(sourceNode.selected).toBe(false);
      renderer.dispose();
    });

    it("syncs selection silently for controlled hosts without emitting select", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ id: "solo", radius: 20, x: 0, y: 0 });
      renderer.scene.add(node);
      const selects: Puzzle2dSelectionSnapshot[] = [];
      renderer.on("select", (snap) => selects.push(snap));
      renderer.setSelectionIdsSilent(["solo"]);
      expect(renderer.selection.getSnapshot().ids).toEqual(["solo"]);
      expect(selects).toEqual([]);
      renderer.dispose();
    });

    it("keeps committed selection empty during rectangle preselect from empty", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test", selection: { mode: "additive" } });
      const node = new Puzzle2dSceneNode({ id: "solo", radius: 20, x: 200, y: 0 });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test", selection: { mode: "additive" } });
      const a = new Puzzle2dSceneNode({ id: "a", radius: 20, x: 0, y: 0 });
      const b = new Puzzle2dSceneNode({ id: "b", radius: 20, x: 120, y: 0 });
      renderer.scene.add(a).add(b);
      renderer.render();
      const preselects: Puzzle2dPreselectSnapshot[] = [];
      renderer.on("preselect", (snap) => preselects.push(snap));
      const s0 = renderer.worldToScreen({ x: -40, y: -40 });
      const s1 = renderer.worldToScreen({ x: 160, y: 40 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: s0.x, clientY: s0.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));
      expect(preselects.length).toBeGreaterThan(0);
      expect(preselects.at(-1)?.ids.includes("b")).toBe(true);
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual(["a", "b"]);
      expect(renderer.preselection.getSnapshot()).toEqual(PUZZLE_2D_PRESELECT_EMPTY);
      renderer.dispose();
    });

    it("syncPreselectionSilent applies selected chrome on scene objects", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const a = new Puzzle2dSceneNode({ id: "a", radius: 20, x: 0, y: 0 });
      const b = new Puzzle2dSceneNode({ id: "b", radius: 20, x: 200, y: 0 });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const a = new Puzzle2dSceneNode({ id: "a", radius: 20, x: 0, y: 0 });
      const b = new Puzzle2dSceneNode({ id: "b", radius: 20, x: 200, y: 0 });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const a = new Puzzle2dSceneNode({ id: "a", radius: 20, x: 0, y: 0 });
      const b = new Puzzle2dSceneNode({ id: "b", radius: 20, x: 200, y: 0 });
      renderer.scene.add(a).add(b);
      renderer.setSelectionIds(["a"]);
      renderer.render();
      const cancels: Puzzle2dPreselectSnapshot[] = [];
      renderer.on("preselectCancel", (snap) => cancels.push(snap));
      const s0 = renderer.worldToScreen({ x: 120, y: -40 });
      const s1 = renderer.worldToScreen({ x: 280, y: 40 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: s0.x, clientY: s0.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: s1.x, clientY: s1.y }));
      expect(renderer.preselection.getSnapshot().ids.length).toBeGreaterThan(0);
      window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
      expect(cancels.length).toBe(1);
      expect(renderer.selection.getSnapshot().ids).toEqual(["a"]);
      expect(renderer.preselection.getSnapshot()).toEqual(PUZZLE_2D_PRESELECT_EMPTY);
      expect(a.selected).toBe(true);
      expect(b.selected).toBe(false);
      renderer.dispose();
    });

    it("opens rectangle selection from a left-button drag and applies directional partial versus enclosing rules", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test", selection: { mode: "additive" } });
      const node = new Puzzle2dSceneNode({ id: "node", radius: 20, x: 0, y: 0 });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ draggable: true, id: "solo", radius: 36, x: 0, y: 0 });
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

    it("background deselect keeps warm descriptor cache without full scene resync", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ draggable: true, id: "solo", radius: 36, x: 0, y: 0 });
      renderer.scene.add(node);
      renderer.render();
      const descriptorEpochBefore = renderer["sceneDescriptorEpoch"];
      const descriptorJsonBefore = renderer["lastPushedDescriptorJson"];

      const onNode = renderer.worldToScreen({ x: 0, y: 0 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: onNode.x, clientY: onNode.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: onNode.x, clientY: onNode.y }));
      expect(renderer.selection.getSnapshot().ids).toEqual(["solo"]);

      puzzle2dResetDebugPerfCounters();
      const background = renderer.worldToScreen({ x: 900, y: 900 });
      canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: background.x, clientY: background.y }));
      canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: background.x, clientY: background.y }));

      expect(renderer.selection.getSnapshot().ids).toEqual([]);
      expect(renderer["sceneDescriptorEpoch"]).toBe(descriptorEpochBefore);
      expect(renderer["lastPushedDescriptorJson"]).toBe(descriptorJsonBefore);
      expect(renderer["lastPushedSceneDescriptorEpoch"]).toBe(descriptorEpochBefore);
      renderer.dispose();
    });

    it("puzzle2dInteractionChromeStyleKey follows selection ids not stale scene flags", () => {
      const node = new Puzzle2dSceneNode({ id: "solo", radius: 20, x: 0, y: 0 });
      node.selected = true;
      node.highlighted = false;
      const chrome = puzzle2dElementInteractionChrome([], PUZZLE_2D_PRESELECT_EMPTY);
      expect(puzzle2dInteractionChromeStyleKey("node", node.id, chrome)).toBe("node");
      const chromeSel = puzzle2dElementInteractionChrome(["solo"], PUZZLE_2D_PRESELECT_EMPTY);
      expect(puzzle2dInteractionChromeStyleKey("node", node.id, chromeSel)).toBe("node.selected");
    });

    it("stale silent selection sync undoes background deselect until controlled prop updates", () => {
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const node = new Puzzle2dSceneNode({ draggable: true, id: "solo", radius: 36, x: 0, y: 0 });
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
      const renderer = new Puzzle2dRenderer({
        canvas,
        renderMode: "headless-test",
        selection: { method: "rectangle", mode: "invertive", targets: { ...PUZZLE_2D_SELECTION_TARGETS_DEFAULT } },
      });
      const sourceNode = new Puzzle2dSceneNode({ id: "source", radius: 40, x: 0, y: 0 });
      const targetNode = new Puzzle2dSceneNode({ id: "target", radius: 40, x: 200, y: 0 });
      const sourceHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new Puzzle2dSceneEdge({ source: sourceHandle, id: "edge-1", target: targetHandle });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test", selection: { method: "lasso", mode: "additive", targets: { nodes: false, edges: true, handles: false } } });
      const sourceNode = new Puzzle2dSceneNode({ id: "source", radius: 12, x: -80, y: 0 });
      const targetNode = new Puzzle2dSceneNode({ id: "target", radius: 12, x: 80, y: 0 });
      const sourceHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "src-h", node: sourceNode });
      const targetHandle = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "tgt-h", node: targetNode });
      const edge = new Puzzle2dSceneEdge({ source: sourceHandle, id: "edge", target: targetHandle });
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test", selection: { mode: "default", targets: { nodes: true, edges: false, handles: false } } });
      const a = new Puzzle2dSceneNode({ id: "a", radius: 12, x: 0, y: 0 });
      const b = new Puzzle2dSceneNode({ id: "b", radius: 12, x: 80, y: 0 });
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

  describe("puzzle 2d fixture io", () => {
    it("parses minimal v1 fixture payloads", () => {
      const parsed = parsePuzzle2dFixtureV1({
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

    it("mergePaletteNodeFromDrop places a palette seed at the drop world point", () => {
      const dragFixture = decodePuzzle2dFixtureFromDragV1(
        encodePuzzle2dFixtureForDragV1({
          camera: { x: 0, y: 0, zoom: 1 },
          edges: [],
          meta: { puzzle2dFixtureDragKind: PUZZLE_2D_FIXTURE_DRAG_KIND_PALETTE_NODE },
          nodes: [{ handles: [{ angle: 0, id: "palette-seed-circle.h0" }], id: "palette-seed-circle", radius: 24, x: 0, y: 0 }],
          schema: "puzzle.2d.fixture/v1",
        }),
      );
      expect(dragFixture).not.toBeNull();
      const merged = mergePaletteNodeFromDrop({ fixture: dragFixture!, screen: { x: 0, y: 0 }, world: { x: 120, y: 80 } });
      expect(merged).not.toBeNull();
      expect(merged!.x).toBe(120);
      expect(merged!.y).toBe(80);
      expect(merged!.id).not.toBe("palette-seed-circle");
    });

    it("parses rectangle fixture nodes", () => {
      const parsed = parsePuzzle2dFixtureV1({
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
      const parsed = parsePuzzle2dFixtureV1({
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
      const parsed = parsePuzzle2dFixtureV1({
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
      const parsed = parsePuzzle2dFixtureV1({
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

    it("classifies puzzle 2d icon selector modes for UI tabs", () => {
      expect(classifyPuzzle2dIconSelectorMode("")).toBe("math");
      expect(classifyPuzzle2dIconSelectorMode("typst:$x$")).toBe("math");
      expect(classifyPuzzle2dIconSelectorMode("$x$")).toBe("math");
      expect(classifyPuzzle2dIconSelectorMode("emoji:😀")).toBe("emoji");
      expect(classifyPuzzle2dIconSelectorMode("data:image/png;base64,abc")).toBe("data");
      expect(classifyPuzzle2dIconSelectorMode("image:data:image/jpeg;base64,xyz")).toBe("data");
      expect(classifyPuzzle2dIconSelectorMode("<svg")).toBe("vector");
      expect(classifyPuzzle2dIconSelectorMode("capsule-with-balcony_p")).toBe("vector");
      expect(classifyPuzzle2dIconSelectorMode("😀")).toBe("emoji");
    });

    it("parses optional textAutofit on fixture nodes", () => {
      const circle = parsePuzzle2dFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        nodes: [{ handles: [{ angle: 0, id: "c.h" }], id: "c", radius: 12, text: "cap", textAutofit: true, x: 0, y: 0 }],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(circle?.nodes[0]).toMatchObject({ id: "c", textAutofit: true, text: "cap" });
      const rect = parsePuzzle2dFixtureV1({
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
      const parsed = parsePuzzle2dFixtureV1({
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
        parsePuzzle2dFixtureV1({
          camera: { x: 0, y: 0, zoom: 1 },
          edges: [],
          nodes: [{ handles: [{ angle: 0, id: "bad.aln" }], id: "bad", radius: 3, textAlignment: "xx", x: 0, y: 0 }],
          schema: "puzzle.2d.fixture/v1",
        })?.nodes[0],
      ).not.toHaveProperty("textAlignment");
    });

    it("parses optional handle radius on fixture nodes", () => {
      const parsed = parsePuzzle2dFixtureV1({
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
        parsePuzzle2dFixtureV1({
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
        parsePuzzle2dFixtureV1({
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
      const parsed = parsePuzzle2dFixtureV1({
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

    it("parses node-kind handle templates from fixture meta.kindCatalogs", () => {
      const catalogs = fixtureMetaKindCatalogBundle({
        meta: {
          kindCatalogs: {
            nodes: [
              {
                id: "semio.kit.node.capsule",
                name: "Capsule",
                handles: [{ angle: 0.805, radius: 3, handleKind: "semio.kit.handle.door" }],
              },
            ],
          },
        },
      });
      expect(catalogs?.nodes?.[0]?.handles).toEqual([{ angle: 0.805, radius: 3, handleKind: "semio.kit.handle.door" }]);
    });

    it("puzzle2dRectangleHandleAngleFromCadPoint matches board north-zero rectangle convention", () => {
      expect(puzzle2dRectangleHandleAngleFromCadPoint(-1.3, -1.25)).toBeCloseTo(0.805, 3);
    });

    it("puzzle2dFixtureHandlesFromNodeKind maps templates to fixture handles", () => {
      const handles = puzzle2dFixtureHandlesFromNodeKind("n1", [{ angle: 1.2, radius: 3, handleKind: "semio.kit.handle.a" }]);
      expect(handles).toEqual([{ angle: 1.2, handleKind: "semio.kit.handle.a", id: "n1:h0", radius: 3 }]);
    });

    it("applyBrushPlacementToFixture is idempotent when the same brush ids are already in the fixture", () => {
      const fixture = parsePuzzle2dFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [{ id: "edge-a", source: "a:h0", target: "b:h0" }],
        nodes: [
          { handles: [{ angle: 0, id: "a:h0" }], id: "a", radius: 40, x: 0, y: 0 },
          { handles: [{ angle: Math.PI, id: "b:h0" }], id: "brush-node", nodeKind: "k", radius: 20, x: 120, y: 0 },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(fixture).toBeTruthy();
      const payload: Puzzle2dBrushPlacePayload = {
        edgeId: "edge-a",
        handles: [{ angle: Math.PI, handleKind: "port" }],
        nodeId: "brush-node",
        nodeKind: "k",
        shape: "circle",
        sourceHandleId: "a:h0",
        targetHandleIndex: 0,
        x: 120,
        y: 0,
        radius: 20,
      };
      const result = applyBrushPlacementToFixture(fixture!, payload);
      expect(result.kind).toBe("placed");
      if (result.kind === "placed") {
        expect(result.fixture.nodes).toHaveLength(2);
        expect(result.fixture.edges).toHaveLength(1);
      }
    });

    it("applyBrushPlacementToFixture appends node and parent edge", () => {
      const fixture: Puzzle2dFixtureV1 = {
        schema: "puzzle.2d.fixture/v1",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [{ id: "a", x: 0, y: 0, radius: 20, handles: [{ id: "a:h0", angle: 0, handleKind: "port" }] }],
        edges: [],
      };
      const result = applyBrushPlacementToFixture(fixture, {
        handles: [{ angle: Math.PI, handleKind: "port" }],
        nodeKind: "brush.kind",
        shape: "circle",
        sourceHandleId: "a:h0",
        targetHandleIndex: 0,
        x: 80,
        y: 0,
        radius: 20,
      });
      expect(result.kind).toBe("placed");
      if (result.kind !== "placed") {
        return;
      }
      expect(result.fixture.nodes).toHaveLength(2);
      expect(result.fixture.edges).toHaveLength(1);
      expect(result.fixture.edges[0]?.source).toBe("a:h0");
      expect(result.fixture.edges[0]?.target).toMatch(/:h0$/);
    });

    it("applyBrushPlacementToFixture copies iconKind from fixture peer nodeKind", () => {
      const fixture: Puzzle2dFixtureV1 = {
        schema: "puzzle.2d.fixture/v1",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [
          { id: "peer", nodeKind: "capsule.kind", iconKind: "capsule_J", x: 0, y: 0, width: 40, height: 40, shape: "rectangle", handles: [{ id: "peer:h0", angle: 0, handleKind: "port" }] },
          { id: "a", x: 0, y: 0, radius: 20, handles: [{ id: "a:h0", angle: 0, handleKind: "port" }] },
        ],
        edges: [],
      };
      const result = applyBrushPlacementToFixture(
        fixture,
        {
          handles: [{ angle: Math.PI, handleKind: "port" }],
          nodeKind: "capsule.kind",
          shape: "rectangle",
          sourceHandleId: "a:h0",
          targetHandleIndex: 0,
          x: 80,
          y: 0,
          width: 40,
          height: 40,
        },
        {},
      );
      expect(result.kind).toBe("placed");
      if (result.kind !== "placed") {
        return;
      }
      const brushed = result.fixture.nodes.find((node) => node.id.startsWith("puzzle2d.brush."));
      expect(brushed).toMatchObject({ nodeKind: "capsule.kind", iconKind: "capsule_J" });
    });

    it("puzzle2dIsBrushPlacementStructuralDeleteGuarded respects TTL guard", () => {
      puzzle2dGuardBrushPlacementStructuralDeletes("puzzle2d.brush.node-a", "puzzle2d.brush.edge-a", 60_000);
      expect(puzzle2dIsBrushPlacementStructuralDeleteGuarded("puzzle2d.brush.node-a")).toBe(true);
      expect(puzzle2dIsBrushPlacementStructuralDeleteGuarded("puzzle2d.brush.edge-a")).toBe(true);
      expect(puzzle2dIsBrushPlacementStructuralDeleteGuarded("other")).toBe(false);
    });

    it("puzzle2dSubscribeBrushSession notifies on sync updates", () => {
      let notifyCount = 0;
      const unsub = puzzle2dSubscribeBrushSession(() => {
        notifyCount += 1;
      });
      const snapshot: Puzzle2dBrushSessionSnapshot = {
        candidateIndex: 0,
        candidates: ["brush.kind"],
        preview: { edge: { sourceHandleId: "a:h0", targetHandleIndex: 0 }, node: { nodeKind: "brush.kind", radius: 20, shape: "circle", x: 80, y: 0 } },
        sourceHandleId: "a:h0",
      };
      puzzle2dSyncBrushSessionToAllAuthoringPeers(snapshot);
      expect(puzzle2dGetBrushSessionSnapshot()?.preview?.node).toBeTruthy();
      puzzle2dSyncBrushSessionToAllAuthoringPeers(null);
      expect(puzzle2dGetBrushSessionSnapshot()).toBeNull();
      expect(notifyCount).toBeGreaterThanOrEqual(2);
      unsub();
    });

    it("mirrors brush session onto every authoring peer except the driving renderer", async () => {
      await ensurePuzzle2dWasmLoaded();
      const drivingCanvas = createMockCanvas();
      const mirrorCanvas = createMockCanvas();
      const driving = new Puzzle2dRenderer({ canvas: drivingCanvas.canvas, renderMode: "headless-test" });
      const mirror = new Puzzle2dRenderer({ canvas: mirrorCanvas.canvas, renderMode: "headless-test" });
      const brushCatalogs = {
        handles: [{ id: "port", name: "Port", color: "#888888" }],
        nodes: [{ id: "brush.kind", name: "Brush", handles: [{ handleKind: "port", angle: Math.PI }] }],
      };
      const brushCompat = [{ source: "port", target: "port" }] as const;
      const setup = (renderer: Puzzle2dRenderer) => {
        renderer.setCamera(0, 0, 1);
        const node = new Puzzle2dSceneNode({ id: "a", radius: 40, x: 0, y: 0 });
        new Puzzle2dSceneHandle({ handleKind: "port", angle: 0, id: "a:h0", node });
        renderer.scene.add(node);
        renderer.setActiveTool("brush");
        renderer.setBrushFlushDistance(80);
        renderer.setBrushNodeSize(40);
        renderer.setKindCatalogs(brushCatalogs);
        renderer.setKindCompatibility(brushCompat);
        renderer.render();
      };
      setup(driving);
      setup(mirror);
      const snapshot: Puzzle2dBrushSessionSnapshot = {
        candidateIndex: 0,
        candidates: ["brush.kind"],
        preview: {
          edge: { sourceHandleId: "a:h0", targetHandleIndex: 0 },
          node: { nodeKind: "brush.kind", radius: 20, shape: "circle", x: 80, y: 0, handles: [{ handleKind: "port", angle: Math.PI }] },
        },
        sourceHandleId: "a:h0",
      };
      const mirrorBefore = mirror.session.encodedSceneHint();
      puzzle2dSyncBrushSessionToAllAuthoringPeers(snapshot, driving);
      mirror.render();
      expect(mirror.session.encodedSceneHint()).toBeGreaterThan(mirrorBefore);
      puzzle2dSyncBrushSessionToAllAuthoringPeers(null);
      driving.dispose();
      mirror.dispose();
    });

    it("flushPendingBrushSessionMirror applies deferred setBrushSession JSON", async () => {
      await ensurePuzzle2dWasmLoaded();
      const mirror = new Puzzle2dRenderer({ renderMode: "headless-test" });
      mirror.setCamera(0, 0, 1);
      const node = new Puzzle2dSceneNode({ id: "a", radius: 40, x: 0, y: 0 });
      new Puzzle2dSceneHandle({ handleKind: "port", angle: 0, id: "a:h0", node });
      mirror.scene.add(node);
      mirror.setActiveTool("brush");
      mirror.setKindCatalogs({
        handles: [{ id: "port", name: "Port", color: "#888888" }],
        nodes: [{ id: "brush.kind", name: "Brush", handles: [{ handleKind: "port", angle: Math.PI }] }],
      });
      mirror.render();
      const pendingJson = JSON.stringify({
        sourceHandleId: "a:h0",
        candidates: ["brush.kind"],
        index: 0,
        preview: {
          edge: { sourceHandleId: "a:h0", targetHandleIndex: 0 },
          node: { nodeKind: "brush.kind", radius: 20, shape: "circle", x: 80, y: 0, handles: [{ handleKind: "port", angle: Math.PI }] },
        },
      });
      mirror["pendingBrushSessionJsonForWasm"] = pendingJson;
      mirror["flushPendingBrushSessionMirror"]();
      mirror.render();
      expect(mirror["lastBrushSessionJsonForWasm"]).toBe(pendingJson);
      mirror.dispose();
    });

    it("brushPlace fires when pointer leaves brush slot", async () => {
      await ensurePuzzle2dWasmLoaded();
      const { canvas } = createMockCanvas();
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      renderer.setCamera(0, 0, 1);
      renderer.setActiveTool("brush");
      renderer.setBrushFlushDistance(80);
      renderer.setBrushNodeSize(40);
      renderer.setKindCatalogs({
        handles: [{ id: "port", name: "Port", color: "#888888" }],
        nodes: [{ id: "brush.kind", name: "Brush", handles: [{ handleKind: "port", angle: Math.PI }] }],
      });
      renderer.setKindCompatibility([{ source: "port", target: "port" }]);
      const node = new Puzzle2dSceneNode({ id: "a", radius: 40, x: 0, y: 0 });
      new Puzzle2dSceneHandle({ handleKind: "port", angle: 0, id: "a:h0", node });
      renderer.scene.add(node);
      renderer.render();
      const handleWorld = computeHandlePosition({ height: 80, radius: 40, shape: "circle", width: 80, x: 0, y: 0 }, 0);
      const slotWorld = { x: handleWorld.x + (handleWorld.x - 0) * 2, y: handleWorld.y + (handleWorld.y - 0) * 2 };
      const slotScreen = renderer.worldToScreen(slotWorld);
      const farScreen = renderer.worldToScreen({ x: 500, y: 500 });
      const placed: Puzzle2dBrushPlacePayload[] = [];
      renderer.on("brushPlace", (payload) => placed.push(payload));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, clientX: slotScreen.x, clientY: slotScreen.y }));
      canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, clientX: farScreen.x, clientY: farScreen.y }));
      expect(placed).toHaveLength(1);
      expect(placed[0]?.sourceHandleId).toBe("a:h0");
      expect(placed[0]?.nodeKind).toBe("brush.kind");
      renderer.dispose();
    });

    it("puzzle2dNodeKindHandlesFromKitConnectors keeps two connectors with the same handleKind at different CAD points", () => {
      const handleKind = "semio.kit.handle.core-rect-bottom";
      const handles = puzzle2dNodeKindHandlesFromKitConnectors([
        { point: { x: -7.5, y: -7.7, z: 7.5 }, port: { handleKind } },
        { point: { x: -18.6, y: -7.7, z: 7.5 }, port: { handleKind } },
      ]);
      expect(handles).toHaveLength(2);
      expect(handles[0]?.angle).not.toBeCloseTo(handles[1]?.angle ?? 0, 4);
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

    it("puzzle2dFixtureNodeDisplayLabel prefers caption over kind name and never shows instance id", () => {
      const catalogs: KindCatalogBundle = {
        nodes: [{ id: "semio.metabolism.light.node.k", name: "Capsule" }],
        handles: [{ color: "#888", defaultWireKind: BUILTIN_LINK_WIRE_KIND, id: "semio.metabolism.light.handle.h", name: "door east" }],
      };
      const node = {
        handles: [{ angle: 0, handleKind: "semio.metabolism.light.handle.h", id: "piece:link" }],
        id: "01890804-66f2-4544-98f0-b6f0c0615492",
        nodeKind: "semio.metabolism.light.node.k",
        radius: 10,
        shape: "circle" as const,
        text: "cs_sl1_d0_t_f4_b_c1",
        x: 0,
        y: 0,
      };
      expect(puzzle2dFixtureNodeDisplayLabel(node, catalogs)).toBe("cs_sl1_d0_t_f4_b_c1");
      expect(puzzle2dFixtureNodeDisplayDescription(node, catalogs)).toBe("Capsule");
      expect(puzzle2dKindCatalogRowName("semio.sketchpad.app.kit.defaultTypeName", catalogs.nodes)).toBe("Item");
      expect(puzzle2dFixtureHandleDisplayLabel(node.handles[0]!, catalogs)).toBe("door east");
    });

    it("puzzle2dFixtureEdgeDisplayLabel uses endpoint labels not edge uuid", () => {
      const catalogs: KindCatalogBundle = {
        handles: [{ color: "#888", defaultWireKind: BUILTIN_LINK_WIRE_KIND, id: BUILTIN_PORT_HANDLE_KIND, name: "Port" }],
      };
      const fixture: Puzzle2dFixtureV1 = {
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [{ id: "ff58a7b3-40c5-4a45-a260-c124706a1b8c", source: "a:link", target: "b:link" }],
        nodes: [
          { handles: [{ angle: 0, id: "a:link" }], id: "a", radius: 10, shape: "circle", text: "Alpha", x: 0, y: 0 },
          { handles: [{ angle: 0, id: "b:link" }], id: "b", radius: 10, shape: "circle", text: "Beta", x: 1, y: 0 },
        ],
      };
      expect(puzzle2dFixtureEdgeDisplayLabel(fixture.edges[0]!, fixture, catalogs)).toBe("Alpha · link → Beta · link");
      expect(puzzle2dFixtureObjectDisplayLabel("a", fixture, catalogs)).toBe("Alpha");
    });

    it("maps kit piece name to node text", () => {
      const parsed = parsePuzzle2dFixtureV1({
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
      expect(puzzle2dFixtureNodeCaption(parsed!.nodes[0]!)).toBe("cs_sl0_d0_t_f0_b_c0");
    });

    it("rejects wrong schema or malformed nodes", () => {
      expect(parsePuzzle2dFixtureV1({ schema: "other", nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } })).toBeNull();
      expect(parsePuzzle2dFixtureV1({ schema: "puzzle.2d.fixture/v1", nodes: "x", edges: [], camera: { x: 0, y: 0, zoom: 1 } })).toBeNull();
      expect(
        parsePuzzle2dFixtureV1({
          camera: { x: 0, y: 0, zoom: 1 },
          edges: [],
          nodes: [{ handles: [], id: "bad", shape: "triangle", x: 0, y: 0 }],
          schema: "puzzle.2d.fixture/v1",
        }),
      ).toBeNull();
    });

    it("reads fixture drag payload from custom MIME or text/plain fallback", () => {
      const fixture: Puzzle2dFixtureV1 = {
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [],
        meta: { puzzle2dFixtureDragKind: PUZZLE_2D_FIXTURE_DRAG_KIND_PALETTE_NODE },
        nodes: [{ handles: [{ angle: 0, handleKind: BUILTIN_PORT_HANDLE_KIND, id: "seed.h0" }], id: "palette-seed-circle", radius: 12, x: 0, y: 0 }],
        schema: "puzzle.2d.fixture/v1",
      };
      const encoded = encodePuzzle2dFixtureForDragV1(fixture);
      const fromCustom = {
        getData: (mime: string) => (mime === PUZZLE_2D_FIXTURE_DRAG_V1_MIME ? encoded : ""),
      } as DataTransfer;
      expect(readPuzzle2dFixtureDragDataTransfer(fromCustom)?.nodes[0]?.id).toBe("palette-seed-circle");
      const fromPlain = {
        getData: (mime: string) => (mime === PUZZLE_2D_FIXTURE_DRAG_PLAIN_MIME ? encoded : ""),
      } as DataTransfer;
      expect(readPuzzle2dFixtureDragDataTransfer(fromPlain)?.nodes[0]?.id).toBe("palette-seed-circle");
    });

    it("round-trips drag codec for v1 fixtures", () => {
      const fixture: Puzzle2dFixtureV1 = {
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
      const decoded = decodePuzzle2dFixtureFromDragV1(encodePuzzle2dFixtureForDragV1(fixture));
      expect(decoded).toEqual(fixture);
    });

    it("parses optional root on fixture nodes", () => {
      const parsed = parsePuzzle2dFixtureV1({
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
      const fixture: Puzzle2dFixtureV1 = {
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [{ id: "e1", source: "a:h0", target: "b:h0" }],
        nodes: [
          { handles: [{ angle: 0, id: "a:h0" }], id: "a", radius: 40, shape: "circle", x: 0, y: 0 },
          { handles: [{ angle: Math.PI, id: "b:h0" }], id: "b", radius: 40, shape: "circle", x: 2, y: 0 },
        ],
        schema: "puzzle.2d.fixture/v1",
      };
      const laid = layoutPuzzle2dFixtureForceGraph(fixture, { gravity: 0, iterations: 220, idealEdgeLength: 200, randomSeed: 11 });
      const ax = (laid.nodes[0] as { x: number }).x;
      const bx = (laid.nodes[1] as { x: number }).x;
      expect(Math.abs(bx - ax)).toBeGreaterThan(90);
      expect(laid.schema).toBe("puzzle.2d.fixture/v1");
    });

    it("throws on invalid fixture schema from wasm", () => {
      const bad = { camera: { x: 0, y: 0, zoom: 1 }, edges: [], nodes: [], schema: "wrong" } as unknown as Puzzle2dFixtureV1;
      expect(() => layoutPuzzle2dFixtureForceGraph(bad)).toThrow();
    });
  });

  describe("puzzle2d directed graph observation", () => {
    it("computes subtree from roots along directed edges", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const root = new Puzzle2dSceneNode({ id: "root", radius: 10, root: true, x: 0, y: 0 });
      const mid = new Puzzle2dSceneNode({ id: "mid", radius: 10, x: 50, y: 0 });
      const leaf = new Puzzle2dSceneNode({ id: "leaf", radius: 10, x: 100, y: 0 });
      const hRoot = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "root:h0", node: root });
      const hMidTarget = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "mid:h0", node: mid });
      const hMidSource = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "mid:h1", node: mid });
      const hLeafTarget = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "leaf:h0", node: leaf });
      const e1 = new Puzzle2dSceneEdge({ source: hRoot, id: "e1", target: hMidTarget });
      const e2 = new Puzzle2dSceneEdge({ source: hMidSource, id: "e2", target: hLeafTarget });
      renderer.scene.add(root).add(mid).add(leaf).add(e1).add(e2);
      const snap = computePuzzle2dGraphObservationSnapshot(renderer.scene);
      expect(snap.rootIds).toEqual(["root"]);
      expect(snap.childNodeIds).toEqual(["leaf", "mid"]);
      expect(snap.childEdgeIds).toEqual(["e1", "e2"]);
      expect(snap.parentEdgeIds).toEqual(["e1"]);
      renderer.dispose();
    });

    it("emits graph observation events after scene mutations flush", async () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
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
      const root = new Puzzle2dSceneNode({ id: "root", radius: 10, root: true, x: 0, y: 0 });
      const child = new Puzzle2dSceneNode({ id: "child", radius: 10, x: 40, y: 0 });
      const hRootSource = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "root:h0", node: root });
      const hChildTarget = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "child:h0", node: child });
      const edge = new Puzzle2dSceneEdge({ source: hRootSource, id: "link", target: hChildTarget });
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
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const created: string[] = [];
      const off = renderer.on("nodeCreate", (p) => created.push(p.id));
      const root = new Puzzle2dSceneNode({ id: "root", radius: 10, root: true, x: 0, y: 0 });
      renderer.scene.add(root);
      await Promise.resolve();
      const child = new Puzzle2dSceneNode({ id: "child", radius: 10, x: 50, y: 0 });
      renderer.scene.add(child);
      await Promise.resolve();
      expect(created).toEqual(["root", "child"]);
      off();
      renderer.dispose();
    });

    it("emits edgeChange when an existing edge signature changes", async () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const changes: string[] = [];
      const off = renderer.on("edgeChange", (p) => changes.push(p.id));
      const root = new Puzzle2dSceneNode({ id: "root", radius: 10, root: true, x: 0, y: 0 });
      const child = new Puzzle2dSceneNode({ id: "child", radius: 10, x: 40, y: 0 });
      const hRootSource = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: 0, id: "root:h0", node: root });
      const hChildTarget = new Puzzle2dSceneHandle({ handleKind: BUILTIN_PORT_HANDLE_KIND, angle: Math.PI, id: "child:h0", node: child });
      const edge = new Puzzle2dSceneEdge({ source: hRootSource, id: "link", target: hChildTarget });
      renderer.scene.add(root).add(child).add(edge);
      await Promise.resolve();
      edge.visible = false;
      renderer.markDirty({ observeGraph: true });
      await Promise.resolve();
      expect(changes).toEqual(["link"]);
      off();
      renderer.dispose();
    });
  });
}
//#endregion 🔖Vitest

let puzzle2dSchedulerPriority = NoEventPriority;

/** @emoji 🧩 Static host surface required by the secondary renderer beyond puzzle 2d scene mutations. */
export const PUZZLE_2D_HOST_MOUNT_DEFAULTS: Record<string, unknown> = {
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
    throw new Error("Puzzle 2d host: cloneHiddenInstance unsupported");
  },
  cloneHiddenTextInstance: () => {
    throw new Error("Puzzle 2d host: cloneHiddenTextInstance unsupported");
  },
  cloneInstance: () => {
    throw new Error("Puzzle 2d host: cloneInstance unsupported");
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
    puzzle2dSchedulerPriority = p;
  },
  getCurrentUpdatePriority() {
    return puzzle2dSchedulerPriority;
  },
  resolveUpdatePriority() {
    if (puzzle2dSchedulerPriority !== NoEventPriority) {
      return puzzle2dSchedulerPriority;
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

function reportPuzzle2dHostUncaughtError(error: unknown, errorInfo: { componentStack?: string | null }): void {
  const report = (globalThis as typeof globalThis & { reportError?: (e: unknown) => void }).reportError;
  if (report) {
    report(error);
    return;
  }
  console.error(error, errorInfo.componentStack ?? "");
}

function reportPuzzle2dHostCaughtError(error: unknown, errorInfo: { componentStack?: string | null; errorBoundary?: unknown }): void {
  const report = (globalThis as typeof globalThis & { reportError?: (e: unknown) => void }).reportError;
  if (report) {
    report(error);
    return;
  }
  console.error(error, errorInfo.componentStack ?? "", errorInfo.errorBoundary ?? "");
}

function reportPuzzle2dHostRecoverableError(error: unknown): void {
  const report = (globalThis as typeof globalThis & { reportError?: (e: unknown) => void }).reportError;
  if (report) {
    report(error);
    return;
  }
  console.error(error);
}

//#region 🔖HostKinds
export const PUZZLE_2D_HOST_NODE = "puzzle.2d/node";
export const PUZZLE_2D_HOST_HANDLE = "puzzle.2d/handle";
export const PUZZLE_2D_HOST_EDGE = "puzzle.2d/edge";
export const PUZZLE_2D_HOST_WIRE = "puzzle.2d/wire";

export type Puzzle2dHostType = typeof PUZZLE_2D_HOST_NODE | typeof PUZZLE_2D_HOST_HANDLE | typeof PUZZLE_2D_HOST_EDGE | typeof PUZZLE_2D_HOST_WIRE;

interface Puzzle2dHostTreeNode {
  kind: "node";
  impl: Puzzle2dSceneNode;
  renderer: Puzzle2dRenderer;
  readonly handleChildren: Set<Puzzle2dHostHandle>;
}

interface Puzzle2dHostHandle {
  kind: "handle";
  impl: Puzzle2dSceneHandle | null;
  props: Puzzle2dHandleProps;
  renderer: Puzzle2dRenderer;
}

interface Puzzle2dHostEdge {
  kind: "edge";
  impl: Puzzle2dSceneEdge | null;
  props: Puzzle2dEdgeProps;
  renderer: Puzzle2dRenderer;
}

interface Puzzle2dHostWire {
  kind: "wire";
  impl: Puzzle2dSceneWire | null;
  props: Puzzle2dWireProps;
  renderer: Puzzle2dRenderer;
}

export type Puzzle2dHostInstance = Puzzle2dHostTreeNode | Puzzle2dHostHandle | Puzzle2dHostEdge | Puzzle2dHostWire;
//#endregion 🔖HostKinds

//#region 🔖PropApply
function newPuzzle2dNodeFromProps(props: Puzzle2dSceneNodeOptions): Puzzle2dSceneNode {
  if (props.shape === "rectangle") {
    return new Puzzle2dSceneNode({
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
  return new Puzzle2dSceneNode({
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

function applyNodeProps(renderer: Puzzle2dRenderer, instance: Puzzle2dSceneNode, props: Puzzle2dSceneNodeOptions): void {
  instance.draggable = props.draggable ?? true;
  instance.style = props.style ?? null;
  instance.userData = { ...(props.userData ?? {}) };
  instance.visible = props.visible ?? true;
  instance.root = props.root === true;
  instance.textAutofit = props.textAutofit ?? false;
  instance.textAlignment = props.textAlignment ?? PUZZLE_2D_NODE_TEXT_ALIGNMENT_DEFAULT;
  instance.textFontFamily = typeof props.textFontFamily === "string" && props.textFontFamily.trim() !== "" ? props.textFontFamily.trim() : PUZZLE_2D_NODE_TEXT_FONT_FAMILY_DEFAULT;
  const psz = props.textFontSize;
  instance.textFontSize = typeof psz === "number" && Number.isFinite(psz) && psz > 0 ? psz : PUZZLE_2D_NODE_TEXT_FONT_PX_DEFAULT;
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

function applyHandleProps(instance: Puzzle2dSceneHandle, props: Puzzle2dHandleProps, node: Puzzle2dSceneNode): void {
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

function applyEdgeProps(instance: Puzzle2dSceneEdge, props: Puzzle2dEdgeProps, sourceHandle: Puzzle2dSceneHandle, targetHandle: Puzzle2dSceneHandle): void {
  instance.style = props.style ?? null;
  instance.userData = { ...(props.userData ?? {}) };
  instance.visible = props.visible ?? true;
  instance.edgeKind = typeof props.edgeKind === "string" ? props.edgeKind.trim() : "";
  instance.setEndpoints(sourceHandle, targetHandle);
}

function applyWireProps(instance: Puzzle2dSceneWire, props: Puzzle2dWireProps, sourceHandle: Puzzle2dSceneHandle, targetHandle: Puzzle2dSceneHandle | null): void {
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

function nodeShapeSyncKey(props: Puzzle2dSceneNodeOptions): "circle" | "rectangle" {
  return props.shape === "rectangle" ? "rectangle" : "circle";
}

function instanceShapeSyncKey(node: Puzzle2dSceneNode): "circle" | "rectangle" {
  return node.shape;
}

function propsEqualHandle(a: Puzzle2dHandleProps, b: Puzzle2dHandleProps): boolean {
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

function propsEqualEdge(a: Puzzle2dEdgeProps, b: Puzzle2dEdgeProps): boolean {
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

function propsEqualWire(a: Puzzle2dWireProps, b: Puzzle2dWireProps): boolean {
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

function propsEqualNode(a: Puzzle2dSceneNodeOptions, b: Puzzle2dSceneNodeOptions): boolean {
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
    (a.textAlignment ?? PUZZLE_2D_NODE_TEXT_ALIGNMENT_DEFAULT) !== (b.textAlignment ?? PUZZLE_2D_NODE_TEXT_ALIGNMENT_DEFAULT) ||
    (a.textFontFamily ?? PUZZLE_2D_NODE_TEXT_FONT_FAMILY_DEFAULT) !== (b.textFontFamily ?? PUZZLE_2D_NODE_TEXT_FONT_FAMILY_DEFAULT) ||
    (a.textFontSize ?? PUZZLE_2D_NODE_TEXT_FONT_PX_DEFAULT) !== (b.textFontSize ?? PUZZLE_2D_NODE_TEXT_FONT_PX_DEFAULT) ||
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
function mountHandleUnderNode(renderer: Puzzle2dRenderer, nodeHost: Puzzle2dHostTreeNode, handleHost: Puzzle2dHostHandle): void {
  if (handleHost.impl?.parent) {
    return;
  }
  nodeHost.handleChildren.add(handleHost);
  const impl = new Puzzle2dSceneHandle({ ...handleHost.props, node: nodeHost.impl });
  handleHost.impl = impl;
  renderer.batch(() => {
    renderer.scene.add(impl);
  });
  renderer.invalidate();
}

function mountNode(renderer: Puzzle2dRenderer, nodeHost: Puzzle2dHostTreeNode): void {
  if (nodeHost.impl.parent) {
    return;
  }
  renderer.batch(() => {
    renderer.scene.add(nodeHost.impl);
  });
  renderer.invalidate();
}

function mountEdge(renderer: Puzzle2dRenderer, edgeHost: Puzzle2dHostEdge): void {
  if (edgeHost.impl?.parent) {
    return;
  }
  const source = renderer.scene.getObjectById(edgeHost.props.source);
  const target = renderer.scene.getObjectById(edgeHost.props.target);
  if (!isPuzzle2dSceneHandleObject(source) || !isPuzzle2dSceneHandleObject(target)) {
    return;
  }
  renderer.batch(() => {
    if (!edgeHost.impl) {
      edgeHost.impl = new Puzzle2dSceneEdge({ ...edgeHost.props, source, target });
      renderer.scene.add(edgeHost.impl);
    } else {
      applyEdgeProps(edgeHost.impl, edgeHost.props, source, target);
    }
  });
  renderer.invalidate();
}

function mountWire(renderer: Puzzle2dRenderer, wireHost: Puzzle2dHostWire): void {
  if (wireHost.impl?.parent) {
    return;
  }
  const source = renderer.scene.getObjectById(wireHost.props.source);
  if (!isPuzzle2dSceneHandleObject(source)) {
    return;
  }
  const tid = (wireHost.props.target ?? "").trim();
  let target: Puzzle2dSceneHandle | null = null;
  if (tid !== "") {
    const t = renderer.scene.getObjectById(tid);
    if (!isPuzzle2dSceneHandleObject(t)) {
      return;
    }
    target = t;
  }
  renderer.batch(() => {
    if (!wireHost.impl) {
      const ex = wireHost.props.endX;
      const ey = wireHost.props.endY;
      wireHost.impl = new Puzzle2dSceneWire({
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

function replaceNodeImpl(renderer: Puzzle2dRenderer, host: Puzzle2dHostTreeNode, nextProps: Puzzle2dSceneNodeOptions): void {
  if (instanceShapeSyncKey(host.impl) !== nodeShapeSyncKey(nextProps)) {
    renderer.batch(() => {
      renderer.runWithoutSceneDeleteEvents(() => {
        for (const handleHost of host.handleChildren) {
          if (handleHost.impl?.parent) {
            renderer.scene.remove(handleHost.impl);
          }
          handleHost.impl = null;
        }
        renderer.scene.remove(host.impl);
      });
      host.impl = newPuzzle2dNodeFromProps(nextProps);
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

function isPuzzle2dRenderer(value: unknown): value is Puzzle2dRenderer {
  return value instanceof Puzzle2dRenderer;
}

function appendToPuzzle2dHostParent(parent: Puzzle2dRenderer | Puzzle2dHostInstance, child: Puzzle2dHostInstance): void {
  const renderer = child.renderer;
  if (isPuzzle2dRenderer(parent)) {
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

function detachHandleFromNode(nodeHost: Puzzle2dHostTreeNode, handleHost: Puzzle2dHostHandle): void {
  nodeHost.handleChildren.delete(handleHost);
}

/** @emoji 🔇 Host reconciler teardown must not emit play {@link Puzzle2dEventMap.nodeDelete} (descriptor resync already uses {@link Puzzle2dRenderer.runWithoutSceneDeleteEvents}). */
function removeSceneObjectWithoutDeleteEvent(renderer: Puzzle2dRenderer, object: Puzzle2dSceneObject): void {
  if (!object.parent) {
    return;
  }
  renderer.runWithoutSceneDeleteEvents(() => {
    renderer.scene.remove(object);
  });
}

const puzzle2dEmptyHostContext = Object.freeze({});
//#endregion 🔖MountHelpers

//#region 🔖HostMountInternals
const puzzle2dSceneHost = Reconciler({
  ...PUZZLE_2D_HOST_MOUNT_DEFAULTS,
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

  getRootHostContext: () => puzzle2dEmptyHostContext,
  getChildHostContext: () => puzzle2dEmptyHostContext,

  createInstance(type, props, rootContainer) {
    const renderer = rootContainer;
    if (type === PUZZLE_2D_HOST_NODE) {
      return { kind: "node", handleChildren: new Set(), impl: newPuzzle2dNodeFromProps(props as NodeOptions), renderer };
    }
    if (type === PUZZLE_2D_HOST_HANDLE) {
      return { kind: "handle", impl: null, props: props as Puzzle2dHandleProps, renderer };
    }
    if (type === PUZZLE_2D_HOST_EDGE) {
      return { kind: "edge", impl: null, props: props as Puzzle2dEdgeProps, renderer };
    }
    if (type === PUZZLE_2D_HOST_WIRE) {
      return { kind: "wire", impl: null, props: props as Puzzle2dWireProps, renderer };
    }
    throw new Error(`Unknown puzzle2d host type: ${String(type)}`);
  },

  createTextInstance() {
    throw new Error("Text children are not supported inside the puzzle2d host tree.");
  },

  shouldSetTextContent: () => false,

  appendInitialChild(parent, child) {
    appendToPuzzle2dHostParent(parent as Puzzle2dRenderer | Puzzle2dHostInstance, child);
  },

  appendChild(parent, child) {
    appendToPuzzle2dHostParent(parent as Puzzle2dRenderer | Puzzle2dHostInstance, child);
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
    appendToPuzzle2dHostParent(parent as Puzzle2dRenderer | Puzzle2dHostInstance, child);
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
    if (!isPuzzle2dRenderer(parent) && parent.kind === "node" && child.kind === "handle") {
      detachHandleFromNode(parent, child);
    }
    if (child.impl?.parent) {
      removeSceneObjectWithoutDeleteEvent(renderer, child.impl);
    }
    if (child.kind === "handle" || child.kind === "edge" || child.kind === "wire") {
      child.impl = null;
    }
    renderer.invalidate();
  },

  removeChildFromContainer(container, child) {
    if (child.kind === "node") {
      const nh = child as Puzzle2dHostTreeNode;
      container.runWithoutSceneDeleteEvents(() => {
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
      });
      container.invalidate();
      return;
    }
    if (child.impl?.parent) {
      removeSceneObjectWithoutDeleteEvent(container, child.impl);
    }
    if (child.kind === "handle" || child.kind === "edge" || child.kind === "wire") {
      child.impl = null;
    }
    container.invalidate();
  },

  /** @emoji 🧹 No-op: host stack calls this on root before mutation; scene graph is driven by append/remove only (see {@link unmountPuzzle2dHostMount}). */
  clearContainer(_container: Puzzle2dRenderer) {},

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
    if (type === PUZZLE_2D_HOST_NODE) {
      return !propsEqualNode(oldProps as NodeOptions, newProps as NodeOptions);
    }
    if (type === PUZZLE_2D_HOST_HANDLE) {
      return !propsEqualHandle(oldProps as Puzzle2dHandleProps, newProps as Puzzle2dHandleProps);
    }
    if (type === PUZZLE_2D_HOST_EDGE) {
      return !propsEqualEdge(oldProps as Puzzle2dEdgeProps, newProps as Puzzle2dEdgeProps);
    }
    if (type === PUZZLE_2D_HOST_WIRE) {
      return !propsEqualWire(oldProps as Puzzle2dWireProps, newProps as Puzzle2dWireProps);
    }
    return false;
  },

  commitUpdate(instance, _payload, type, oldProps, nextProps) {
    const renderer = instance.renderer;
    if (type === PUZZLE_2D_HOST_NODE) {
      const next = nextProps as NodeOptions;
      const prev = oldProps as NodeOptions;
      const host = instance as Puzzle2dHostTreeNode;
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
    if (type === PUZZLE_2D_HOST_HANDLE) {
      const h = instance as Puzzle2dHostHandle;
      h.props = nextProps as Puzzle2dHandleProps;
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
    if (type === PUZZLE_2D_HOST_EDGE) {
      const e = instance as Puzzle2dHostEdge;
      e.props = nextProps as Puzzle2dEdgeProps;
      const from = renderer.scene.getObjectById(e.props.source);
      const to = renderer.scene.getObjectById(e.props.target);
      if (!isPuzzle2dSceneHandleObject(from) || !isPuzzle2dSceneHandleObject(to)) {
        return;
      }
      renderer.batch(() => {
        if (!e.impl) {
          e.impl = new Puzzle2dSceneEdge({ ...e.props, source: from, target: to });
          renderer.scene.add(e.impl);
        } else {
          applyEdgeProps(e.impl, e.props, from, to);
        }
      });
      renderer.invalidate();
      return;
    }
    if (type === PUZZLE_2D_HOST_WIRE) {
      const w = instance as Puzzle2dHostWire;
      w.props = nextProps as Puzzle2dWireProps;
      const from = renderer.scene.getObjectById(w.props.source);
      if (!isPuzzle2dSceneHandleObject(from)) {
        return;
      }
      const tid = (w.props.target ?? "").trim();
      let to: Puzzle2dSceneHandle | null = null;
      if (tid !== "") {
        const t = renderer.scene.getObjectById(tid);
        if (!isPuzzle2dSceneHandleObject(t)) {
          return;
        }
        to = t;
      }
      renderer.batch(() => {
        if (!w.impl) {
          const ex = w.props.endX;
          const ey = w.props.endY;
          w.impl = new Puzzle2dSceneWire({
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

export type Puzzle2dHostMount = ReturnType<typeof puzzle2dSceneHost.createContainer>;

/** @emoji 🌱 Creates a legacy-mode host mount bound to {@link Puzzle2dRenderer} for synchronous subtree commits with DOM `act()`. */
export function createPuzzle2dHostMount(renderer: Puzzle2dRenderer): Puzzle2dHostMount {
  return puzzle2dSceneHost.createContainer(
    renderer,
    LegacyRoot,
    null,
    false,
    null,
    "puzzle2d:",
    reportPuzzle2dHostUncaughtError,
    reportPuzzle2dHostCaughtError,
    reportPuzzle2dHostRecoverableError,
    undefined,
  );
}

/** @emoji 🔄 Schedules host work; optional `onCommitted` runs after the host flush (before {@link Puzzle2dRenderer.invalidate}). */
export function updatePuzzle2dHostMount(root: Puzzle2dHostMount, element: ReactElement | null, parent: null, onCommitted?: () => void): void {
  puzzle2dSceneHost.updateContainer(element, root, parent, () => {
    const renderer = root.containerInfo as Puzzle2dRenderer;
    onCommitted?.();
    renderer.invalidate();
  });
}

/** @emoji 🧹 Unmounts the host subtree without disposing {@link Puzzle2dRenderer}. */
export function unmountPuzzle2dHostMount(root: Puzzle2dHostMount): void {
  updatePuzzle2dHostMount(root, null, null);
  const renderer = root.containerInfo as Puzzle2dRenderer;
  renderer.runWithoutSceneDeleteEvents(() => {
    renderer.scene.clear();
  });
  renderer.resetDeclarativeSceneSyncFingerprint();
  renderer.markSceneDescriptorDirty();
  renderer.invalidate();
}

export { puzzle2dSceneHost };
//#endregion 🔖HostMountInternals

// #region 🎨ReactCanvas
import { FiberProvider as HostMountProvider, useContextBridge as useHostMountBridge } from "its-fine";
import { Children, Fragment, act, createElement, isValidElement, type CSSProperties, type DragEvent, type ReactElement, type ReactNode } from "react";
import { createRoot } from "react-dom/client";

//#region 🔖Kinds
export interface Puzzle2dCanvasProps {
  camera?: Partial<CameraState>;
  children?: ReactNode;
  /** @emoji 🗂️ When set, imperative scene sync uses this descriptor instead of {@link buildPuzzle2dSceneDescriptor}(`children`) (play passes {@link buildPuzzle2dSceneDescriptorFromFixture}). */
  declarativeSceneDescriptor?: Puzzle2dSceneDescriptor;
  /** @emoji 🎧 DOM-tree descendants with {@link Puzzle2dContext} (e.g. {@link usePuzzle2dEvent}); not mounted in the puzzle2d host reconciler. */
  companions?: ReactNode;
  className?: string;
  contextMenu?: ContextMenuItem[];
  /** @emoji 📥 When true, accepts in-app fixture drags using {@link PUZZLE_2D_FIXTURE_DRAG_V1_MIME} (not OS file drops). */
  fixtureDragDrop?: boolean;
  height?: number;
  /** @emoji 🔗 Allowed kind pairs for link gestures (`specificity` tiers + `important`); empty omits filtering. */
  kindCompatibility?: readonly KindCompatEntry[];
  /** @emoji 🧩 Central semantic kind catalogs (handles, wires, nodes, edges) for WASM defaults + compatibility. */
  kindCatalogs?: KindCatalogBundle;
  onFixtureDrop?: (detail: Puzzle2dFixtureDropDetail) => void;
  /** @emoji 🖱️ Fires after pointer-driven hit tests (same cadence as canvas moves); use for tooltips and status. */
  onHover?: (payload: Puzzle2dHoverPayload) => void;
  onReady?: (renderer: Puzzle2dRenderer) => void;
  /** @emoji 🔔 Fires after any graph observation emission in this flush (see other `on*` graph props). */
  onChange?: () => void;
  /** @emoji 📶 LOD zoom bands for WASM draw + overlay captions (`data-puzzle2d-lod`). */
  lodZoomThresholds?: Puzzle2dLodZoomThresholds;
  /** @emoji 📶 When true (default), camera zoom selects draw LOD; when false, {@link lod} pins the tier. */
  automaticLod?: boolean;
  /** @emoji 📶 Pinned draw LOD when `automaticLod` is false. */
  lod?: Puzzle2dDrawLodKind;
  /** @emoji 📶 Emits whenever the resolved WASM draw LOD band changes. */
  onLodChange?: (lod: Puzzle2dDrawLodKind) => void;
  /** @emoji 📐 Positive multiplier for LOD world grid steps on the WASM host (default {@link DEFAULT_PUZZLE_2D_GRID_FACTOR}). */
  gridFactor?: number;
  /** @emoji 🧲 When true, node drags snap to the finest visible LOD grid on the WASM host. */
  gridSnapEnabled?: boolean;
  onChildEdgeChange?: (payload: Puzzle2dGraphEdgeIdPayload) => void;
  onChildEdgesChange?: (payload: Puzzle2dChildEdgesChangePayload) => void;
  onChildNodeChange?: (payload: Puzzle2dGraphNodeIdPayload) => void;
  onChildNodesChange?: (payload: Puzzle2dChildNodesChangePayload) => void;
  onNodeChange?: (payload: Puzzle2dGraphNodeIdPayload) => void;
  onParentEdgeChange?: (payload: Puzzle2dGraphEdgeIdPayload) => void;
  onParentNodeChange?: (payload: Puzzle2dGraphNodeIdPayload) => void;
  /** @emoji 🎥 Camera pan/zoom center in world space plus zoom factor (same payload as {@link Puzzle2dCanvasProps.onViewportChange}). */
  onCamera?: (state: CameraState) => void;
  /** @emoji 🖱️ Right-click surface hit before built-in context UI resolves menu items. */
  onContextMenu?: (payload: Puzzle2dEventMap["contextmenu"]) => void;
  /** @emoji 🪢 Direct handle-to-handle link commit ({@link Puzzle2dEventMap.edgeCreate}). */
  onConnect?: (payload: Puzzle2dEdgeLinkPayload) => void;
  /** @emoji 📦 Fires once for {@link Puzzle2dEventMap.nodeCreate}, {@link Puzzle2dEventMap.edgeCreate}, or {@link Puzzle2dEventMap.wireCreate}. */
  onCreate?: (payload: Puzzle2dStructureCreatePayload) => void;
  /** @emoji 📦 Fires once for {@link Puzzle2dEventMap.nodeDelete}, {@link Puzzle2dEventMap.edgeDelete}, or {@link Puzzle2dEventMap.wireDestroy}. */
  onDelete?: (payload: Puzzle2dStructureDeletePayload) => void;
  /** @emoji 🖱️ Node drag motion from WASM (`nodeMove`); live panes sync imperatively — avoid heavy declarative fixture commits per frame. */
  onDrag?: (payload: Puzzle2dEventMap["nodeMove"]) => void;
  /** @emoji 🏁 Node drag release from WASM (`nodeDragEnd`); commit declarative fixture coordinates once. */
  onDragEnd?: (payload: Puzzle2dEventMap["nodeDragEnd"]) => void;
  onEdgeChange?: (payload: Puzzle2dGraphEdgeIdPayload) => void;
  onEdgeCreate?: (payload: Puzzle2dEdgeLinkPayload) => void;
  onEdgeDelete?: (payload: { id: string }) => void;
  /** @emoji 🧭 Second click on an indirect handle ring target after {@link Puzzle2dEventMap.edgeCreate}. */
  onIndirectConnect?: (payload: Puzzle2dEdgeLinkPayload) => void;
  /** @emoji ♻️ GPU/text invalidation tick (coalesced `invalidate`). */
  onInvalidate?: () => void;
  onNodeCreate?: (payload: Puzzle2dGraphNodeIdPayload) => void;
  onNodeDelete?: (payload: { id: string }) => void;
  /** @emoji 🧲 Snap commit on pointer-up after a link drag (`proximityConnect` after `edgeCreate`). */
  onProximityConnect?: (payload: Puzzle2dEdgeLinkPayload) => void;
  /** @emoji 🎯 Emits while a link drag highlights compatible target parts ({@link Puzzle2dEventMap.linkCompatibleNodes}). */
  onLinkCompatibleNodes?: (payload: Puzzle2dLinkCompatibleNodesPayload) => void;
  /** @emoji ⭕ Emits while a link drag shows an indirect anchor ring ({@link Puzzle2dEventMap.linkTargetRing}). */
  onLinkTargetRing?: (payload: Puzzle2dLinkTargetRingPayload) => void;
  /** @emoji 🔗 Host-driven link preview for cross-surface gestures (cleared when `source` is empty). */
  linkSession?: Puzzle2dLinkSessionSnapshot | null;
  /** @emoji 🖌️ Active viewport tool forwarded to the WASM host. */
  activeTool?: Puzzle2dActiveTool;
  /** @emoji 📐 Brush slot offset along handle outward normal (world units). */
  brushFlushDistance?: number;
  /** @emoji 📐 Brush preview node span in world units. */
  brushNodeSize?: number;
  /** @emoji 🖌️ Paint-style brush commit when the cursor leaves a slot ({@link Puzzle2dEventMap.brushPlace}). */
  onBrushPlace?: (payload: Puzzle2dBrushPlacePayload) => void;
  /** @emoji 🖌️ Brush candidate node kinds while hovering a slot. */
  onBrushCandidates?: (payload: Puzzle2dBrushCandidatesPayload) => void;
  onSelect?: (snapshot: Puzzle2dSelectionSnapshot) => void;
  /** @emoji ✅ Controlled committed selection (`onSelect` should update this). */
  selection?: Puzzle2dSelectionSnapshot | readonly string[];
  /** @emoji ✅ Uncontrolled initial committed selection. */
  defaultSelection?: Puzzle2dSelectionSnapshot | readonly string[];
  /** @emoji 👁️ Controlled area-select preview (`onPreselect` should update this). */
  preselection?: Puzzle2dPreselectSnapshot;
  /** @emoji 👁️ Uncontrolled initial area-select preview. */
  defaultPreselection?: Puzzle2dPreselectSnapshot;
  onPreselect?: (snapshot: Puzzle2dPreselectSnapshot) => void;
  /** @emoji 🔁 Bumps when shared fixture graph authoring changes so every pane reapplies declarative node positions. */
  sceneAuthoringEpoch?: number;
  /** @emoji 🖱️ Controlled hover target id (`onHover` should update this). */
  hoveredId?: string | null;
  /** @emoji 🖱️ Uncontrolled initial hover target id. */
  defaultHoveredId?: string | null;
  onWireChange?: (payload: Puzzle2dGraphWireIdPayload) => void;
  onWireCreate?: (payload: Puzzle2dWireSnapshotPayload) => void;
  onWireDestroy?: (payload: Puzzle2dGraphWireIdPayload) => void;
  /** @emoji ↔️ Camera center changed without zoom delta beyond float noise. */
  onPan?: (state: CameraState) => void;
  /** @emoji 🔎 Zoom factor changed on the camera snapshot. */
  onZoom?: (state: CameraState) => void;
  /** @emoji 🪟 Preferred alias for {@link Puzzle2dCanvasProps.onCamera} (viewport = camera snapshot). */
  onViewportChange?: (state: CameraState) => void;
  renderMode?: RenderMode;
  selectionMethod?: Puzzle2dSelectionMethod;
  selectionMode?: Puzzle2dSelectionMode;
  /** @emoji 🎯 Independent toggles for which kinds participate in marquee/lasso and hit picking. */
  selectionTargets?: Puzzle2dSelectionTargets;
  style?: CSSProperties;
  width?: number;
  /** 🧩 World-space clip tiling for Vello (`world-clip`, default) vs monolithic scene (`none`). */
  worldRasterTiling?: WorldRasterTilingKind;
}

export type Puzzle2dNodeCircleProps = {
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
  textAlignment?: Puzzle2dNodeTextAlignment;
  /** @emoji 🔤 CSS font family for overlay caption. */
  textFontFamily?: string;
  /** @emoji 🔤 Caption size in layout px when not autofitting. */
  textFontSize?: number;
  userData?: Record<string, unknown>;
  visible?: boolean;
  x: number;
  y: number;
};

export type Puzzle2dNodeRectangleProps = {
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
  textAlignment?: Puzzle2dNodeTextAlignment;
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

/** @emoji 🟠 Declarative node marker: {@link Puzzle2dNodeCircleProps} or {@link Puzzle2dNodeRectangleProps}. */
export type Puzzle2dNodeProps = Puzzle2dNodeCircleProps | Puzzle2dNodeRectangleProps;

export interface NodeDescriptor extends Puzzle2dNodeProps {
  handles: HandleDescriptor[];
}

export interface HandleDescriptor extends Puzzle2dHandleProps {
  nodeId: string;
}

export interface EdgeDescriptor extends Puzzle2dEdgeProps {}

export interface WireDescriptor extends Puzzle2dWireProps {}

export interface Puzzle2dSceneDescriptor {
  edges: EdgeDescriptor[];
  handles: HandleDescriptor[];
  nodes: NodeDescriptor[];
  wires: WireDescriptor[];
}
//#endregion 🔖Kinds

//#region 🔖Context
const Puzzle2dContext = reactHostPort.createContext<Puzzle2dRenderer | null>(null);
let activePuzzle2dRenderer: Puzzle2dRenderer | null = null;

//#endregion 🔖Context

//#region 🔖Markers
/** 🟠 Host intrinsic for the secondary puzzle2d host; assign to JSX {@link PUZZLE_2D_HOST_NODE}. */
export const Node = PUZZLE_2D_HOST_NODE;

/** 🟣 Host intrinsic for puzzle 2d handles nested under {@link Node}. */
export const Handle = PUZZLE_2D_HOST_HANDLE;

/** 🪢 Host intrinsic for directed edges between handle ids. */
export const Edge = PUZZLE_2D_HOST_EDGE;

/** 🧵 Host intrinsic for transient wires from a handle to another handle or a free world end. */
export const Wire = PUZZLE_2D_HOST_WIRE;

/** @emoji 🗼 Optional per-id context menus when building {@link puzzle2dFixtureSceneMarkers}. */
export interface Puzzle2dFixtureSceneMarkersOptions {
  edgeContextMenuForId?: (edgeId: string) => ContextMenuItem[] | undefined;
  nodeContextMenuForId?: (nodeId: string) => ContextMenuItem[] | undefined;
}

/** @emoji 🗼 Declarative {@link Node}/{@link Edge} tree for {@link Puzzle2dCanvas} `children` from {@link Puzzle2dFixtureV1} (Fragment of host markers only). */
export function puzzle2dFixtureSceneMarkers(fixture: Puzzle2dFixtureV1, options?: Puzzle2dFixtureSceneMarkersOptions): ReactElement {
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
            text={puzzle2dFixtureNodeCaption(node)}
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
            text={puzzle2dFixtureNodeCaption(node)}
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
  return element.type === PUZZLE_2D_HOST_NODE || element.type === PUZZLE_2D_HOST_HANDLE || element.type === PUZZLE_2D_HOST_EDGE || element.type === PUZZLE_2D_HOST_WIRE;
}

function forEachPuzzle2dHostMarkerChild(node: ReactNode, visitChild: (child: ReactElement) => void): void {
  const walk = (current: ReactNode): void => {
    if (current == null || current === false || current === true) {
      return;
    }
    if (Array.isArray(current)) {
      for (const entry of current) {
        walk(entry);
      }
      return;
    }
    Children.forEach(current, (child) => {
      if (!isValidElement(child)) {
        return;
      }
      if (child.type === Fragment) {
        walk((child as ReactElement<{ children?: ReactNode }>).props.children);
        return;
      }
      visitChild(child);
    });
  };
  walk(node);
}

function appendHandleDescriptors(children: ReactNode, nodeId: string, handles: HandleDescriptor[]): void {
  forEachPuzzle2dHostMarkerChild(children, (child) => {
    if (!isValidElement(child)) {
      return;
    }
    if (child.type === Fragment) {
      appendHandleDescriptors((child as ReactElement<{ children?: ReactNode }>).props.children, nodeId, handles);
      return;
    }
    if (child.type === PUZZLE_2D_HOST_HANDLE) {
      const props = child.props as Puzzle2dHandleProps;
      handles.push({ ...props, nodeId });
    }
  });
}

/** @emoji 🗂️ Builds a {@link Puzzle2dSceneDescriptor} from {@link Puzzle2dFixtureV1} without walking React `children` (stable play sync). */
export function buildPuzzle2dSceneDescriptorFromFixture(fixture: Puzzle2dFixtureV1): Puzzle2dSceneDescriptor {
  const nodes: NodeDescriptor[] = [];
  const handles: HandleDescriptor[] = [];
  for (const node of fixture.nodes) {
    const nodeHandles: HandleDescriptor[] = node.handles.map((handle) => ({
      angle: handle.angle,
      handleKind: handle.handleKind,
      id: handle.id,
      nodeId: node.id,
      ...(handle.color !== undefined ? { color: handle.color } : {}),
      ...(handle.radius !== undefined ? { radius: handle.radius } : {}),
      ...(handle.iconKind !== undefined ? { iconKind: handle.iconKind } : {}),
    }));
    handles.push(...nodeHandles);
    const caption = puzzle2dFixtureNodeCaption(node);
    const shared = {
      draggable: true as const,
      handles: nodeHandles,
      id: node.id,
      ...(caption !== undefined ? { text: caption } : {}),
      ...(node.textAutofit === true ? { textAutofit: true as const } : {}),
      ...(node.textFontFamily !== undefined ? { textFontFamily: node.textFontFamily } : {}),
      ...(node.textFontSize !== undefined ? { textFontSize: node.textFontSize } : {}),
      ...(node.textAlignment !== undefined ? { textAlignment: node.textAlignment } : {}),
      ...(node.root === true ? { root: true as const } : {}),
      ...(node.iconKind !== undefined ? { iconKind: node.iconKind } : {}),
      ...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {}),
      x: node.x,
      y: node.y,
    };
    if (node.shape === "rectangle") {
      nodes.push({ ...shared, height: node.height, shape: "rectangle", width: node.width });
    } else {
      nodes.push({ ...shared, radius: node.radius, shape: "circle" });
    }
  }
  const edges: EdgeDescriptor[] = fixture.edges.map((edge) => ({
    id: edge.id,
    source: edge.source,
    target: edge.target,
  }));
  return { edges, handles, nodes, wires: [] };
}

export function buildPuzzle2dSceneDescriptor(children: ReactNode): Puzzle2dSceneDescriptor {
  const descriptor: Puzzle2dSceneDescriptor = { edges: [], handles: [], nodes: [], wires: [] };

  forEachPuzzle2dHostMarkerChild(children, (child) => {
    if (child.type === PUZZLE_2D_HOST_NODE) {
      const props = child.props as Puzzle2dNodeProps;
      const nodeHandles: HandleDescriptor[] = [];
      appendHandleDescriptors(props.children, props.id, nodeHandles);
      descriptor.nodes.push({ ...props, handles: nodeHandles });
      descriptor.handles.push(...nodeHandles);
      return;
    }
    if (child.type === PUZZLE_2D_HOST_EDGE) {
      descriptor.edges.push(child.props as Puzzle2dEdgeProps);
      return;
    }
    if (child.type === PUZZLE_2D_HOST_WIRE) {
      descriptor.wires.push(child.props as Puzzle2dWireProps);
    }
  });

  return descriptor;
}

/** @emoji 🗂️ Picks the richest declarative descriptor for host sync (fixture prop wins; falls back to React `children` edges). */
function puzzle2dResolveHostSceneDescriptor(declarativeSceneDescriptor: Puzzle2dSceneDescriptor | undefined, children: ReactNode): Puzzle2dSceneDescriptor {
  const fromChildren = buildPuzzle2dSceneDescriptor(children);
  if (!declarativeSceneDescriptor) {
    return fromChildren;
  }
  if (declarativeSceneDescriptor.edges.length > 0) {
    return declarativeSceneDescriptor;
  }
  if (fromChildren.edges.length > 0) {
    return { ...declarativeSceneDescriptor, edges: fromChildren.edges };
  }
  return declarativeSceneDescriptor;
}

/** @emoji 🪢 Re-runs {@link syncPuzzle2dScene} when the descriptor lists edges but the imperative scene does not (WASM drain / mount race). */
function puzzle2dEnsureSceneEdgesFromDescriptor(renderer: Puzzle2dRenderer, descriptor: Puzzle2dSceneDescriptor): void {
  const filtered = renderer.descriptorWithoutAuthoritativeRemovals(descriptor);
  if (filtered.edges.length === 0 || renderer.scene.edges.size === filtered.edges.length) {
    return;
  }
  renderer.setDeclarativeSceneEdgeExpectation(filtered.edges.length);
  renderer.resetDeclarativeSceneSyncFingerprint();
  syncPuzzle2dScene(renderer, filtered);
}
//#endregion 🔖Descriptor Build

function requireRenderer(renderer: Puzzle2dRenderer | null): Puzzle2dRenderer {
  if (!renderer) {
    throw new Error("Puzzle2dCanvas did not publish its renderer.");
  }
  return renderer;
}

//#region 🔖Scene Sync
/** @emoji 🔗 Merges WASM‑created edges into the JSX descriptor until React children list the same edge id (then authorship is cleared via {@link Puzzle2dRenderer.clearWasmHostAuthorshipForEdge}). */
export function mergeWasmHostAuthoredEdgesIntoDescriptor(renderer: Puzzle2dRenderer, descriptor: Puzzle2dSceneDescriptor): Puzzle2dSceneDescriptor {
  const descriptorBase = renderer.descriptorWithoutAuthoritativeRemovals(descriptor);
  const jsxEdgeIds = new Set(descriptorBase.edges.map((edge) => edge.id));
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
    if (!isPuzzle2dSceneHandleObject(sourceH) || !isPuzzle2dSceneHandleObject(targetH)) {
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
    return descriptorBase;
  }
  return { ...descriptorBase, edges: [...descriptorBase.edges, ...extra] };
}

//#region 🔖MultiViewAuthoring
const puzzle2dAuthoringPeerRenderers = new Set<Puzzle2dRenderer>();
let puzzle2dSharedBrushSession: Puzzle2dBrushSessionSnapshot | null = null;
let puzzle2dBrushPlaceCommitHandler: ((payload: Puzzle2dBrushPlacePayload) => void) | null = null;
const puzzle2dBrushSessionListeners = new Set<() => void>();

/** @emoji 🧪 Reads the shared brush session (tests only). */
export function puzzle2dSharedBrushSessionForTests(): Puzzle2dBrushSessionSnapshot | null {
  return puzzle2dSharedBrushSession;
}

/** @emoji 🖌️ Snapshot for {@link puzzle2dSubscribeBrushSession} / `useSyncExternalStore`. */
export function puzzle2dGetBrushSessionSnapshot(): Puzzle2dBrushSessionSnapshot | null {
  return puzzle2dSharedBrushSession;
}

/** @emoji 🖌️ Subscribes to shared brush session updates (play mirrors Overview / Zoom / Selection). */
export function puzzle2dSubscribeBrushSession(listener: () => void): () => void {
  puzzle2dBrushSessionListeners.add(listener);
  return () => {
    puzzle2dBrushSessionListeners.delete(listener);
  };
}

function puzzle2dNotifyBrushSessionListeners(): void {
  for (const listener of puzzle2dBrushSessionListeners) {
    listener();
  }
}

/** @emoji 🖌️ Commits a WASM brush placement into the fixture and every authoring pane scene. */
export function puzzle2dCommitBrushPlacementToPlay(
  payload: Puzzle2dBrushPlacePayload,
  options: {
    readonly catalogsForFixture: (fixture: Puzzle2dFixtureV1) => KindCatalogBundle | undefined;
    readonly patchFixture: (updater: (prev: Puzzle2dFixtureV1) => Puzzle2dFixtureV1) => void;
  },
): boolean {
  puzzle2dSyncBrushSessionToAllAuthoringPeers(null);
  let placed = false;
  options.patchFixture((prev) => {
    const catalogs = options.catalogsForFixture(prev);
    const result = applyBrushPlacementToFixture(prev, payload, catalogs);
    if (result.kind !== "placed") {
      return prev;
    }
    placed = true;
    puzzle2dGuardBrushPlacementStructuralDeletes(result.nodeId, result.edgeId);
    puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(result.fixture);
    return result.fixture;
  });
  if (placed) {
    puzzle2dPushAuthoritativeSceneToAllAuthoringPeers();
  }
  return placed;
}

/** @emoji 🖌️ Registers play (or host) fixture commit for WASM {@link Puzzle2dEventMap.brushPlace} drains. */
export function puzzle2dSetBrushPlaceCommitHandler(handler: ((payload: Puzzle2dBrushPlacePayload) => void) | null): void {
  puzzle2dBrushPlaceCommitHandler = handler;
}

function puzzle2dInvokeBrushPlaceCommit(payload: Puzzle2dBrushPlacePayload): void {
  puzzle2dBrushPlaceCommitHandler?.(payload);
}

function puzzle2dBrushPreviewIsEmpty(preview: Puzzle2dEventMap["brushPreview"] | null | undefined): boolean {
  if (!preview) {
    return true;
  }
  return preview.node === null || preview.node === undefined;
}

function puzzle2dUpdateBrushSessionFromSource(
  source: Puzzle2dRenderer,
  candidates: Puzzle2dBrushCandidatesPayload | null,
  preview: Puzzle2dEventMap["brushPreview"] | null | undefined,
): void {
  const prev = puzzle2dSharedBrushSession;
  const resolvedPreview =
    preview === undefined ? (prev?.preview ?? null) : puzzle2dBrushPreviewIsEmpty(preview) ? null : preview;
  const sourceFromCandidates = candidates?.sourceHandleId?.trim() ?? "";
  const sourceHandleId =
    sourceFromCandidates.length > 0
      ? sourceFromCandidates
      : resolvedPreview?.edge?.sourceHandleId?.trim() ||
        (preview !== undefined && puzzle2dBrushPreviewIsEmpty(preview) ? null : prev?.sourceHandleId) ||
        null;
  const nextCandidates = candidates !== null ? candidates.candidates : (prev?.candidates ?? []);
  if (candidates !== null && sourceFromCandidates.length === 0 && nextCandidates.length === 0) {
    const previewEmpty = preview === undefined ? !prev?.preview : puzzle2dBrushPreviewIsEmpty(preview);
    if (previewEmpty) {
      puzzle2dSyncBrushSessionToAllAuthoringPeers(null, source);
      return;
    }
  }
  const next: Puzzle2dBrushSessionSnapshot = {
    candidateIndex: candidates?.index ?? prev?.candidateIndex ?? 0,
    candidates: nextCandidates,
    preview: resolvedPreview,
    sourceHandleId: sourceHandleId && sourceHandleId.length > 0 ? sourceHandleId : null,
  };
  if (!next.sourceHandleId && !next.preview && next.candidates.length === 0) {
    puzzle2dSyncBrushSessionToAllAuthoringPeers(null, source);
    return;
  }
  puzzle2dSyncBrushSessionToAllAuthoringPeers(next, source);
}

/** @emoji 🖌️ Mirrors brush slot preview onto every authoring pane except the driving renderer. */
export function puzzle2dSyncBrushSessionToAllAuthoringPeers(snapshot: Puzzle2dBrushSessionSnapshot | null, skip?: Puzzle2dRenderer): void {
  puzzle2dSharedBrushSession = snapshot;
  puzzle2dNotifyBrushSessionListeners();
  for (const peer of puzzle2dAuthoringPeerRenderers) {
    if (!peer.authoringPeerActive() || peer === skip) {
      continue;
    }
    peer.setBrushSession(snapshot);
  }
}

/** @emoji 📡 Pushes the imperative scene to WASM on every authoring pane (after brush place). */
export function puzzle2dPushAuthoritativeSceneToAllAuthoringPeers(): void {
  for (const peer of puzzle2dAuthoringPeerRenderers) {
    if (!peer.authoringPeerActive()) {
      continue;
    }
    peer.pushAuthoritativeSceneToWasmHost();
  }
}

function puzzle2dRegisterAuthoringPeer(renderer: Puzzle2dRenderer): void {
  puzzle2dAuthoringPeerRenderers.add(renderer);
  if (puzzle2dSharedBrushSession) {
    renderer.setBrushSession(puzzle2dSharedBrushSession);
  }
}

function puzzle2dUnregisterAuthoringPeer(renderer: Puzzle2dRenderer): void {
  puzzle2dAuthoringPeerRenderers.delete(renderer);
}

function puzzle2dBroadcastNodeMove(source: Puzzle2dRenderer, payload: { id: string; x: number; y: number }): void {
  for (const peer of puzzle2dAuthoringPeerRenderers) {
    if (peer === source || !peer.authoringPeerActive()) {
      continue;
    }
    peer.applyNodePositionSilent(payload.id, payload.x, payload.y);
  }
}

function puzzle2dBroadcastStructuralRemove(source: Puzzle2dRenderer, payload: Puzzle2dStructureDeletePayload): void {
  for (const peer of puzzle2dAuthoringPeerRenderers) {
    if (peer === source || !peer.authoringPeerActive()) {
      continue;
    }
    peer.applyStructuralRemoveSilent(payload);
  }
}

function puzzle2dBroadcastSelectionSilent(source: Puzzle2dRenderer, ids: readonly string[]): void {
  for (const peer of puzzle2dAuthoringPeerRenderers) {
    if (peer === source || !peer.authoringPeerActive()) {
      continue;
    }
    peer.applySelectionFromPeerSilent(ids);
  }
}

function puzzle2dBroadcastPreselectSilent(source: Puzzle2dRenderer, snapshot: Puzzle2dPreselectSnapshot): void {
  for (const peer of puzzle2dAuthoringPeerRenderers) {
    if (peer === source || !peer.authoringPeerActive()) {
      continue;
    }
    peer.syncPreselectionSilent(snapshot);
  }
}

/** @emoji ✅ Imperatively mirrors committed selection onto every authoring pane (hierarchy / shell without canvas re-render). */
export function puzzle2dSyncSelectionToAllAuthoringPeers(ids: readonly string[]): void {
  for (const peer of puzzle2dAuthoringPeerRenderers) {
    if (!peer.authoringPeerActive()) {
      continue;
    }
    peer.applySelectionFromPeerSilent(ids);
  }
}

/** @emoji 🛡️ Fresh brush placement ids shielded from play structural-delete resync bursts. */
const puzzle2dBrushStructuralDeleteGuardIds = new Set<string>();

/** @emoji 🛡️ Guards a brushed node/edge from {@link puzzle2dIsBrushPlacementStructuralDeleteGuarded} for a short TTL. */
export function puzzle2dGuardBrushPlacementStructuralDeletes(nodeId: string, edgeId: string, ttlMs = 600): void {
  puzzle2dBrushStructuralDeleteGuardIds.add(nodeId);
  puzzle2dBrushStructuralDeleteGuardIds.add(edgeId);
  if (typeof globalThis.setTimeout !== "function") {
    return;
  }
  globalThis.setTimeout(() => {
    puzzle2dBrushStructuralDeleteGuardIds.delete(nodeId);
    puzzle2dBrushStructuralDeleteGuardIds.delete(edgeId);
  }, ttlMs);
}

/** @emoji 🛡️ True when play must ignore a structural delete for a just-placed brush instance. */
export function puzzle2dIsBrushPlacementStructuralDeleteGuarded(id: string): boolean {
  return puzzle2dBrushStructuralDeleteGuardIds.has(id);
}

/** @emoji 🔄 Syncs a committed fixture graph into every authoring pane (same descriptor on every peer). */
export function puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(fixture: Puzzle2dFixtureV1): void {
  const descriptor = buildPuzzle2dSceneDescriptorFromFixture(fixture);
  for (const peer of puzzle2dAuthoringPeerRenderers) {
    if (!peer.authoringPeerActive()) {
      continue;
    }
    peer.clearAuthoritativeStructuralSuppressionsForDescriptor(descriptor);
    for (const edgeId of Array.from(peer.wasmHostAuthoredEdgeIds)) {
      peer.clearWasmHostAuthorshipForEdge(edgeId);
    }
    peer.rememberHostDeclarativeSceneDescriptor(descriptor);
    peer.setDeclarativeSceneEdgeExpectation(descriptor.edges.length);
    peer.resetDeclarativeSceneSyncFingerprint();
    syncPuzzle2dScene(peer, descriptor);
    puzzle2dEnsureSceneEdgesFromDescriptor(peer, descriptor);
    peer.markSceneDescriptorDirty();
    peer.invalidate();
  }
}
//#endregion 🔖MultiViewAuthoring

/** @emoji 🔍 True when imperative {@link Puzzle2dRenderer.scene} counts match a declarative descriptor (guards skip after host unmount clears the graph). */
function puzzle2dSceneDescriptorMatchesScene(renderer: Puzzle2dRenderer, descriptor: Puzzle2dSceneDescriptor): boolean {
  return (
    renderer.scene.nodes.size === descriptor.nodes.length &&
    renderer.scene.handles.size === descriptor.handles.length &&
    renderer.scene.edges.size === descriptor.edges.length &&
    renderer.scene.wires.size === descriptor.wires.length
  );
}

/** @emoji 🔑 Stable fingerprint for declarative scene descriptors; skips no-op {@link syncPuzzle2dScene} passes. */
export function puzzle2dSceneDescriptorFingerprint(descriptor: Puzzle2dSceneDescriptor): string {
  const nodeParts = descriptor.nodes
    .map((node) => {
      const shape = node.shape === "rectangle" ? `r:${node.width},${node.height}` : `c:${node.radius}`;
      const handleParts = node.handles
        .map((h) => `${h.id}:${h.angle},${h.radius},${h.handleKind},${h.visible !== false ? 1 : 0}`)
        .sort()
        .join(";");
      return `n:${node.id}:${node.x},${node.y},${shape},${node.text ?? ""},${node.visible !== false ? 1 : 0},${node.selected ? 1 : 0},${node.draggable !== false ? 1 : 0},${node.style ?? ""},${node.nodeKind ?? ""},${node.iconKind ?? ""},h[${handleParts}]`;
    })
    .sort()
    .join("|");
  const edgeParts = descriptor.edges
    .map((e) => `e:${e.id}:${e.source}:${e.target},${e.visible !== false ? 1 : 0},${e.selected ? 1 : 0},${e.style ?? ""},${e.edgeKind ?? ""}`)
    .sort()
    .join("|");
  const wireParts = descriptor.wires
    .map((w) => `w:${w.id}:${w.source}:${w.target ?? ""},${w.endX ?? ""},${w.endY ?? ""},${w.visible !== false ? 1 : 0},${w.selected ? 1 : 0}`)
    .sort()
    .join("|");
  return `${nodeParts}#${edgeParts}#${wireParts}`;
}

function puzzle2dWasmDescriptorGraphCounts(json: string): {
  readonly edges: number;
  readonly handles: number;
  readonly nodes: number;
  readonly wires: number;
} | null {
  try {
    const parsed = JSON.parse(json) as { edges?: unknown; handles?: unknown; nodes?: unknown; wires?: unknown };
    return {
      edges: Array.isArray(parsed.edges) ? parsed.edges.length : 0,
      handles: Array.isArray(parsed.handles) ? parsed.handles.length : 0,
      nodes: Array.isArray(parsed.nodes) ? parsed.nodes.length : 0,
      wires: Array.isArray(parsed.wires) ? parsed.wires.length : 0,
    };
  } catch {
    return null;
  }
}

/** @emoji 🔍 True when cached WASM descriptor JSON matches imperative {@link Puzzle2dRenderer.scene} counts (guards edgeless stale cache). */
function puzzle2dWasmDescriptorJsonMatchesScene(renderer: Puzzle2dRenderer, json: string): boolean {
  const counts = puzzle2dWasmDescriptorGraphCounts(json);
  if (!counts) {
    return false;
  }
  return (
    counts.nodes === renderer.scene.nodes.size &&
    counts.handles === renderer.scene.handles.size &&
    counts.edges === renderer.scene.edges.size &&
    counts.wires === renderer.scene.wires.size
  );
}

function puzzle2dSceneWasmFingerprintFromRenderer(renderer: Puzzle2dRenderer): string {
  const selection = renderer.selectionStore.getSnapshot().ids.join(",");
  const nodeParts: string[] = [];
  for (const node of renderer.scene.nodes.values()) {
    const shape = node.shape === "rectangle" ? `r:${node.width},${node.height}` : `c:${node.radius}`;
    nodeParts.push(
      `n:${node.id}:${node.x},${node.y},${shape},${node.text ?? ""},${node.visible ? 1 : 0},${renderer.selectionIds.has(node.id) ? 1 : 0},${node.draggable ? 1 : 0},${node.style ?? ""},${node.nodeKind},${node.iconKind ?? ""}`,
    );
  }
  nodeParts.sort();
  const handleParts: string[] = [];
  for (const handle of renderer.scene.handles.values()) {
    handleParts.push(
      `h:${handle.id}:${handle.node.id},${handle.angle},${handle.radius},${handle.handleKind},${handle.visible ? 1 : 0},${renderer.selectionIds.has(handle.id) ? 1 : 0}`,
    );
  }
  handleParts.sort();
  const edgeParts: string[] = [];
  for (const edge of renderer.scene.edges.values()) {
    edgeParts.push(
      `e:${edge.id}:${edge.source.id}:${edge.target.id},${edge.visible ? 1 : 0},${renderer.selectionIds.has(edge.id) ? 1 : 0},${edge.style ?? ""},${edge.edgeKind}`,
    );
  }
  edgeParts.sort();
  const wireParts: string[] = [];
  for (const wire of renderer.scene.wires.values()) {
    wireParts.push(
      `w:${wire.id}:${wire.source.id}:${wire.target?.id ?? ""},${wire.endX ?? ""},${wire.endY ?? ""},${wire.visible ? 1 : 0},${renderer.selectionIds.has(wire.id) ? 1 : 0}`,
    );
  }
  wireParts.sort();
  return `${selection}#${nodeParts.join("|")}#${handleParts.join("|")}#${edgeParts.join("|")}#${wireParts.join("|")}`;
}

/** 🔁 Declarative-to-imperative scene sync that preserves stable instances by id. */
export function syncPuzzle2dScene(renderer: Puzzle2dRenderer, descriptor: Puzzle2dSceneDescriptor): void {
  if (renderer.skipSceneSyncIfDescriptorUnchanged(descriptor)) {
    return;
  }

  const desiredNodeIds = new Set(descriptor.nodes.map((node) => node.id));
  const desiredHandleIds = new Set(descriptor.handles.map((handle) => handle.id));
  const desiredEdgeIds = new Set(descriptor.edges.map((edge) => edge.id));
  const desiredWireIds = new Set(descriptor.wires.map((wire) => wire.id));

  renderer.batch(() => {
    for (const nodeDescriptor of descriptor.nodes) {
      let existingNode = renderer.scene.getObjectById(nodeDescriptor.id);
      if (isPuzzle2dSceneNodeObject(existingNode) && instanceShapeSyncKey(existingNode) !== nodeShapeSyncKey(nodeDescriptor)) {
        renderer.runWithoutSceneDeleteEvents(() => {
          renderer.scene.remove(existingNode);
        });
        existingNode = undefined;
      }
      const resolvedExisting = renderer.scene.getObjectById(nodeDescriptor.id);
      const { handles: _handles, ...nodeProps } = nodeDescriptor;
      const node = isPuzzle2dSceneNodeObject(resolvedExisting) ? resolvedExisting : newPuzzle2dNodeFromProps(nodeProps);
      if (!isPuzzle2dSceneNodeObject(resolvedExisting)) {
        renderer.scene.add(node);
      }
      applyNodeProps(renderer, node, nodeProps);
    }

    for (const handleDescriptor of descriptor.handles) {
      const parentNode = renderer.scene.getObjectById(handleDescriptor.nodeId);
      if (!isPuzzle2dSceneNodeObject(parentNode)) {
        continue;
      }
      const existingHandle = renderer.scene.getObjectById(handleDescriptor.id);
      const { nodeId: _nodeId, ...handleProps } = handleDescriptor;
      const handle = isPuzzle2dSceneHandleObject(existingHandle) ? existingHandle : new Puzzle2dSceneHandle({ ...handleProps, node: parentNode });
      if (!isPuzzle2dSceneHandleObject(existingHandle)) {
        renderer.scene.add(handle);
      }
      applyHandleProps(handle, handleProps, parentNode);
    }

    for (const edgeDescriptor of descriptor.edges) {
      const sourceHandle = renderer.scene.getObjectById(edgeDescriptor.source);
      const targetHandle = renderer.scene.getObjectById(edgeDescriptor.target);
      if (!isPuzzle2dSceneHandleObject(sourceHandle) || !isPuzzle2dSceneHandleObject(targetHandle)) {
        continue;
      }
      const existingEdge = renderer.scene.getObjectById(edgeDescriptor.id);
      const edge = isPuzzle2dSceneEdgeObject(existingEdge) ? existingEdge : new Puzzle2dSceneEdge({ ...edgeDescriptor, source: sourceHandle, target: targetHandle });
      if (!isPuzzle2dSceneEdgeObject(existingEdge)) {
        renderer.scene.add(edge);
      }
      applyEdgeProps(edge, edgeDescriptor, sourceHandle, targetHandle);
    }

    for (const wireDescriptor of descriptor.wires) {
      const sourceHandle = renderer.scene.getObjectById(wireDescriptor.source);
      if (!isPuzzle2dSceneHandleObject(sourceHandle)) {
        continue;
      }
      const tid = (wireDescriptor.target ?? "").trim();
      let targetHandle: Puzzle2dSceneHandle | null = null;
      if (tid !== "") {
        const t = renderer.scene.getObjectById(tid);
        if (!isPuzzle2dSceneHandleObject(t)) {
          continue;
        }
        targetHandle = t;
      }
      const existingWire = renderer.scene.getObjectById(wireDescriptor.id);
      const ex = wireDescriptor.endX;
      const ey = wireDescriptor.endY;
      const wire =
        isPuzzle2dSceneWireObject(existingWire)
          ? existingWire
          : new Puzzle2dSceneWire({
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
      if (!isPuzzle2dSceneWireObject(existingWire)) {
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
  renderer.markSceneDescriptorDirty();
  renderer.rememberDeclarativeSceneSyncFingerprint(descriptor);
  renderer.invalidate();
}
//#endregion 🔖Scene Sync

//#region 🔖HostMountBridge
/** @emoji 🌉 Secondary host root per {@link Puzzle2dRenderer}; scene sync runs on `children` changes and on {@link Puzzle2dRenderer.subscribeWasmHostSceneMergeResync} bumps (WASM graph drains), camera only on `camera` prop changes so marker/selection JSX churn does not reset pan/zoom. */
function Puzzle2dHostSubtree({
  camera,
  children,
  declarativeSceneDescriptor,
  renderer,
  sceneAuthoringEpoch,
}: {
  camera?: Partial<CameraState>;
  children: ReactNode;
  declarativeSceneDescriptor?: Puzzle2dSceneDescriptor;
  renderer: Puzzle2dRenderer;
  sceneAuthoringEpoch?: number;
}): null {
  const hostMountRef = reactHostPort.useRef<Puzzle2dHostMount | null>(null);
  const mountedRendererRef = reactHostPort.useRef<Puzzle2dRenderer | null>(null);
  const Bridge = useHostMountBridge();
  const wasmHostSceneMergeResyncEpoch = reactHostPort.useSyncExternalStore(renderer.subscribeWasmHostSceneMergeResync, renderer.getWasmHostSceneMergeResyncEpoch, renderer.getWasmHostSceneMergeResyncEpoch);

  reactHostPort.useLayoutEffect(() => {
    if (sceneAuthoringEpoch !== undefined) {
      renderer.clearNodeAuthoringPositionCache();
    }
  }, [renderer, sceneAuthoringEpoch]);

  reactHostPort.useLayoutEffect(() => {
    if (hostMountRef.current === null || mountedRendererRef.current !== renderer) {
      if (hostMountRef.current) {
        unmountPuzzle2dHostMount(hostMountRef.current);
        hostMountRef.current = null;
      }
      hostMountRef.current = createPuzzle2dHostMount(renderer);
      mountedRendererRef.current = renderer;
    }
    updatePuzzle2dHostMount(hostMountRef.current, createElement(Bridge, null, children), null);
    const jsxDescriptor = puzzle2dResolveHostSceneDescriptor(declarativeSceneDescriptor, children);
    const merged = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsxDescriptor);
    renderer.rememberHostDeclarativeSceneDescriptor(merged);
    renderer.setDeclarativeSceneEdgeExpectation(merged.edges.length);
    syncPuzzle2dScene(renderer, merged);
    puzzle2dEnsureSceneEdgesFromDescriptor(renderer, merged);
    if (merged.edges.length > 0 && renderer.scene.edges.size === 0) {
      queueMicrotask(() => {
        if (renderer.isDisposed) {
          return;
        }
        puzzle2dEnsureSceneEdgesFromDescriptor(renderer, merged);
        if (renderer.scene.edges.size > 0) {
          renderer.invalidate();
        }
      });
    }
  }, [children, declarativeSceneDescriptor, renderer, wasmHostSceneMergeResyncEpoch]);

  reactHostPort.useLayoutEffect(() => {
    if (!renderer.acceptsHostCameraProp()) {
      return;
    }
    const cx = camera?.x ?? 0;
    const cy = camera?.y ?? 0;
    const cz = camera?.zoom ?? 1;
    if (!renderer.clearWasmViewportLeadingIfHostCameraMatches({ x: cx, y: cy, zoom: cz })) {
      return;
    }
    renderer.setCameraSilent(cx, cy, cz);
  }, [camera?.x, camera?.y, camera?.zoom, renderer]);

  reactHostPort.useLayoutEffect(
    () => () => {
      if (hostMountRef.current) {
        unmountPuzzle2dHostMount(hostMountRef.current);
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
/** 🖼️ React puzzle 2d root that keeps the hot path inside the imperative renderer. */
export function Puzzle2dCanvas({
  camera,
  children,
  declarativeSceneDescriptor,
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
  onDragEnd,
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
  activeTool,
  brushFlushDistance,
  brushNodeSize,
  onBrushPlace,
  onBrushCandidates,
  onReady,
  onSelect,
  selection,
  defaultSelection,
  preselection,
  defaultPreselection,
  onPreselect,
  sceneAuthoringEpoch,
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
}: Puzzle2dCanvasProps): ReactElement {
  const canvasRef = reactHostPort.useRef<HTMLCanvasElement | null>(null);
  const textOverlayCanvasRef = reactHostPort.useRef<HTMLCanvasElement | null>(null);
  const containerRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const [contextRenderer, setContextRenderer] = reactHostPort.useState<Puzzle2dRenderer | null>(null);
  const rendererRef = reactHostPort.useRef<Puzzle2dRenderer | null>(null);
  const [uncontrolledSelection, setUncontrolledSelection] = reactHostPort.useState<Puzzle2dSelectionSnapshot>(() => normalizePuzzle2dSelectionProp(defaultSelection));
  const [uncontrolledPreselection, setUncontrolledPreselection] = reactHostPort.useState<Puzzle2dPreselectSnapshot>(() => normalizePuzzle2dPreselectProp(defaultPreselection));
  const [uncontrolledHoveredId, setUncontrolledHoveredId] = reactHostPort.useState<string | null>(defaultHoveredId ?? null);
  const selectionControlled = selection !== undefined;
  const preselectionControlled = preselection !== undefined;
  const hoveredControlled = hoveredIdProp !== undefined;
  const resolvedSelection = selectionControlled ? normalizePuzzle2dSelectionProp(selection) : uncontrolledSelection;
  const resolvedPreselection = preselectionControlled ? normalizePuzzle2dPreselectProp(preselection) : uncontrolledPreselection;
  const resolvedHoveredId = hoveredControlled ? (hoveredIdProp ?? null) : uncontrolledHoveredId;
  const puzzle2dTargetMenusRef = reactHostPort.useRef(new Map<string, ContextMenuItem[]>());
  const [surfaceContextMenu, setSurfaceContextMenu] = reactHostPort.useState<{ clientX: number; clientY: number; items: ContextMenuItem[] } | null>(null);
  const [fixtureDragActive, setFixtureDragActive] = reactHostPort.useState(false);
  const fileDragDepthRef = reactHostPort.useRef(0);
  const resolvedFixtureDragDrop = fixtureDragDrop ?? Boolean(onFixtureDrop);
  const handleDragEnter = reactHostPort.useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!resolvedFixtureDragDrop) {
        return;
      }
      if (![...event.dataTransfer.types].includes(PUZZLE_2D_FIXTURE_DRAG_V1_MIME)) {
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
      if ([...event.dataTransfer.types].includes(PUZZLE_2D_FIXTURE_DRAG_V1_MIME)) {
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
      const fixture = readPuzzle2dFixtureDragDataTransfer(event.dataTransfer);
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
      const detail: Puzzle2dFixtureDropDetail = { fixture, screen, world };
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
    const descriptor = buildPuzzle2dSceneDescriptor(children);
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
    puzzle2dTargetMenusRef.current = next;
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
      return undefined;
    }
    const unsubs: Array<() => void> = [];
    if (onBrushPlace) {
      unsubs.push(contextRenderer.on("brushPlace", onBrushPlace));
    }
    if (onBrushCandidates) {
      unsubs.push(contextRenderer.on("brushCandidates", onBrushCandidates));
    }
    return () => {
      for (const unsub of unsubs) {
        unsub();
      }
    };
  }, [contextRenderer, onBrushCandidates, onBrushPlace]);

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
      const onPreselectEvent = (snapshot: Puzzle2dPreselectSnapshot): void => {
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
    if (onDragEnd) {
      unsubs.push(contextRenderer.on("nodeDragEnd", onDragEnd));
    }
    return () => {
      for (const u of unsubs) {
        u();
      }
    };
  }, [contextRenderer, onDrag, onDragEnd, onInvalidate, onPreselect, onSelect, preselectionControlled, selectionControlled]);

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
      const items = payload.id ? (puzzle2dTargetMenusRef.current.get(payload.id) ?? []) : (contextMenu ?? []);
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
    const renderer = new Puzzle2dRenderer({
      canvas,
      automaticLod: automaticLod ?? true,
      gridFactor: gridFactor ?? DEFAULT_PUZZLE_2D_GRID_FACTOR,
      gridSnapEnabled: gridSnapEnabled ?? false,
      lodZoomThresholds: lodZoomThresholds ?? DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS,
      ...(lod !== undefined ? { lod } : {}),
      renderMode,
      selection: { method: selectionMethod, mode: selectionMode, targets: selectionTargets },
      worldRasterTiling,
    });
    rendererRef.current = renderer;
    activePuzzle2dRenderer = renderer;
    setContextRenderer(renderer);
    return () => {
      const r = renderer;
      queueMicrotask(() => {
        r.dispose();
        if (activePuzzle2dRenderer === r) {
          activePuzzle2dRenderer = null;
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
    const descriptor = puzzle2dResolveHostSceneDescriptor(declarativeSceneDescriptor, children);
    renderer.rememberHostDeclarativeSceneDescriptor(descriptor);
    renderer.setDeclarativeSceneEdgeExpectation(descriptor.edges.length);
    puzzle2dEnsureSceneEdgesFromDescriptor(renderer, descriptor);
  }, [children, contextRenderer, declarativeSceneDescriptor]);

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
    renderer.setGridFactor(gridFactor ?? DEFAULT_PUZZLE_2D_GRID_FACTOR);
  }, [gridFactor]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setLodZoomThresholds(lodZoomThresholds ?? DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS);
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
    const observer = new MutationObserver((records) => {
      for (const record of records) {
        if (record.type !== "attributes") {
          continue;
        }
        const name = record.attributeName;
        if (name !== "class" && name !== "style") {
          continue;
        }
        const target = record.target as HTMLElement;
        const next = target.getAttribute(name);
        if (record.oldValue === next) {
          continue;
        }
        renderer.invalidate();
        return;
      }
    });
    observer.observe(root, { attributeFilter: ["class", "style"], attributeOldValue: true, attributes: true });
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

  const lastSyncedControlledSelectionRef = reactHostPort.useRef<Puzzle2dSelectionSnapshot | null>(null);
  const lastSyncedControlledPreselectionRef = reactHostPort.useRef<Puzzle2dPreselectSnapshot | null>(null);

  reactHostPort.useLayoutEffect(() => {
    lastSyncedControlledSelectionRef.current = null;
    lastSyncedControlledPreselectionRef.current = null;
  }, [contextRenderer]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    if (lastSyncedControlledSelectionRef.current !== null && puzzle2dSelectionSnapshotsEqual(resolvedSelection, lastSyncedControlledSelectionRef.current)) {
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
    if (lastSyncedControlledPreselectionRef.current !== null && puzzle2dPreselectSnapshotsEqual(resolvedPreselection, lastSyncedControlledPreselectionRef.current)) {
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
    if (!renderer) {
      return;
    }
    renderer.setActiveTool(activeTool ?? "select");
  }, [activeTool]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setBrushFlushDistance(brushFlushDistance ?? DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX);
  }, [brushFlushDistance]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.setBrushNodeSize(brushNodeSize ?? DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX);
  }, [brushNodeSize]);

  reactHostPort.useLayoutEffect(() => {
    const renderer = rendererRef.current;
    const container = containerRef.current;
    if (!renderer || !container) {
      return;
    }

    const applySize = (): void => {
      const nextWidth = width ?? container.clientWidth ?? 1;
      const nextHeight = height ?? container.clientHeight ?? 1;
      const nextDpr = globalThis.devicePixelRatio || 1;
      if (!renderer.setSize(nextWidth, nextHeight, nextDpr)) {
        return;
      }
      renderer.invalidate();
    };

    applySize();
    if (typeof ResizeObserver === "undefined") {
      return undefined;
    }

    let resizeRafId: number | null = null;
    const observer = new ResizeObserver(() => {
      if (resizeRafId !== null) {
        return;
      }
      const schedule =
        typeof globalThis.requestAnimationFrame === "function"
          ? (fn: () => void) => {
              resizeRafId = globalThis.requestAnimationFrame(() => {
                resizeRafId = null;
                fn();
              });
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
    <Puzzle2dContext.Provider value={contextRenderer}>
      <div
        className={["relative box-border min-h-0 min-w-0 size-full select-none", className, fixtureDragActive ? "ring-2 ring-[color:var(--color-accent)] ring-offset-2 ring-offset-[color:var(--color-base)]" : ""].filter(Boolean).join(" ") || undefined}
        onDragEnter={handleDragEnter}
        onDragLeave={handleDragLeave}
        onDragOver={handleDragOver}
        onDrop={(e) => void handleDrop(e)}
        ref={containerRef}
        style={{ height: height ?? "100%", position: "relative", width: width ?? "100%", ...(style ?? {}) }}
      >
        <canvas className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" data-testid="puzzle2d-canvas" ref={canvasRef} />
        {renderMode === "headless-test" ? null : <canvas aria-hidden className="pointer-events-none absolute inset-0 block size-full min-h-0 min-w-0" data-testid="puzzle2d-text-overlay" ref={textOverlayCanvasRef} />}
        {contextRenderer ? (
          <>
            <HostMountProvider>
              <Puzzle2dHostSubtree
                camera={camera}
                children={children}
                declarativeSceneDescriptor={declarativeSceneDescriptor}
                renderer={contextRenderer}
                sceneAuthoringEpoch={sceneAuthoringEpoch}
              />
              {onLodChange ? <Puzzle2dDrawLodReporter onLodChange={onLodChange} /> : null}
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
    </Puzzle2dContext.Provider>
  );
}
//#endregion 🔖Canvas

//#region 🔖Hooks
/** @emoji 📶 Subscribes to {@link Puzzle2dRenderer} draw LOD band changes for window measure labels. */
export function Puzzle2dDrawLodReporter({ onLodChange }: { onLodChange?: (lod: Puzzle2dDrawLodKind) => void }): null {
  const renderer = usePuzzle2dRenderer();
  const lod = reactHostPort.useSyncExternalStore(renderer.subscribeDrawLod, renderer.getDrawLodSnapshot, renderer.getDrawLodSnapshot);
  reactHostPort.useEffect(() => {
    onLodChange?.(lod);
  }, [lod, onLodChange]);
  return null;
}

/** @emoji 🎯 Returns the focused {@link Puzzle2dRenderer} when a canvas is mounted (play/toolbar bridges). */
export function puzzle2dActiveRenderer(): Puzzle2dRenderer | null {
  return activePuzzle2dRenderer;
}

/** 🎯 Access the imperative puzzle 2d renderer from within Puzzle2dCanvas descendants (DOM or secondary host tree). */
export function usePuzzle2dRenderer(): Puzzle2dRenderer {
  const renderer = reactHostPort.useContext(Puzzle2dContext);
  if (renderer) {
    return renderer;
  }
  if (activePuzzle2dRenderer) {
    return activePuzzle2dRenderer;
  }
  throw new Error("usePuzzle2dRenderer must be used inside Puzzle2dCanvas.");
}

/** 📷 Read and update camera state through an external store subscription. */
export function useCamera(): [CameraState, (camera: CameraState) => void] {
  const renderer = usePuzzle2dRenderer();
  const snapshot = reactHostPort.useSyncExternalStore(renderer.subscribeCamera, renderer.getCameraSnapshot, renderer.getCameraSnapshot);
  return [snapshot, (nextCamera) => renderer.setCamera(nextCamera.x, nextCamera.y, nextCamera.zoom)];
}

/** ✅ Subscribe to semantic selection ids without pushing React through the drag hot path. */
export function useSelection(): Puzzle2dSelectionSnapshot {
  const renderer = usePuzzle2dRenderer();
  return reactHostPort.useSyncExternalStore(renderer.subscribeSelection, renderer.getSelectionSnapshot, renderer.getSelectionSnapshot);
}

/** @emoji 👁️ Subscribe to area-select preview ids (and anchor-removed ids) on the active puzzle 2d renderer. */
export function usePreselection(): Puzzle2dPreselectSnapshot {
  const renderer = usePuzzle2dRenderer();
  return reactHostPort.useSyncExternalStore(renderer.subscribePreselect, renderer.getPreselectSnapshot, renderer.getPreselectSnapshot);
}

/** 📡 Bind a puzzle 2d event listener with stable cleanup (`fixtureDrop`, `hover`, `change` / graph observation events, `contextmenu`, …). */
export function usePuzzle2dEvent<TKey extends keyof Puzzle2dEventMap>(name: TKey, handler: (payload: Puzzle2dEventMap[TKey]) => void): void {
  const renderer = usePuzzle2dRenderer();
  reactHostPort.useEffect(() => renderer.on(name, handler), [handler, name, renderer]);
}

/** ⏱️ Subscribe to imperative frame callbacks emitted after each render pass. */
export function useFrame(callback: (state: FrameState, dt: number) => void): void {
  const renderer = usePuzzle2dRenderer();
  reactHostPort.useEffect(() => renderer.subscribeFrame(callback), [callback, renderer]);
}

/** 🔄 Imperatively request another render for the active puzzle 2d root. */
export function invalidate(renderer?: Puzzle2dRenderer): void {
  (renderer ?? activePuzzle2dRenderer)?.invalidate();
}
//#endregion 🔖Hooks

//#region 🔖Vitest
const puzzle2dReactVitest = (
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

if (puzzle2dReactVitest) {
  const { afterEach, beforeAll, describe, expect, it, vi } = puzzle2dReactVitest;
  (globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

  beforeAll(async () => {
    await ensurePuzzle2dWasmLoaded();
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

  function Puzzle2dSelectListenerStub(): null {
    usePuzzle2dEvent("select", () => undefined);
    return null;
  }

  describe("puzzle2d react helpers", () => {
    it("puzzle2dFixtureSceneMarkers maps nakagin fixture into scene descriptors", async () => {
      const nakaginFixtureJson = (await import("../fixture/nakagin-capsule-tower.2d.json")).default as unknown;
      const fixture = parsePuzzle2dFixtureV1(nakaginFixtureJson);
      expect(fixture?.nodes.length).toBeGreaterThan(100);
      const descriptor = buildPuzzle2dSceneDescriptor(puzzle2dFixtureSceneMarkers(fixture!));
      expect(descriptor.nodes.length).toBe(fixture!.nodes.length);
      expect(descriptor.edges.length).toBe(fixture!.edges.length);
      expect(descriptor.handles.length).toBeGreaterThan(fixture!.nodes.length);
    });

    it("buildPuzzle2dSceneDescriptorFromFixture maps nakagin fixture without React children", async () => {
      const nakaginFixtureJson = (await import("../fixture/nakagin-capsule-tower.2d.json")).default as unknown;
      const fixture = parsePuzzle2dFixtureV1(nakaginFixtureJson);
      expect(fixture).toBeTruthy();
      const descriptor = buildPuzzle2dSceneDescriptorFromFixture(fixture!);
      expect(descriptor.nodes.length).toBe(fixture!.nodes.length);
      expect(descriptor.edges.length).toBe(fixture!.edges.length);
      expect(descriptor.handles.length).toBeGreaterThan(fixture!.nodes.length);
    });

    it("nakagin fixture encodes edge vector paths at play overview zoom in wasm", async () => {
      const nakaginFixtureJson = (await import("../fixture/nakagin-capsule-tower.2d.json")).default as unknown;
      const fixture = parsePuzzle2dFixtureV1(nakaginFixtureJson);
      expect(fixture).toBeTruthy();
      await ensurePuzzle2dWasmLoaded();
      const descriptor = buildPuzzle2dSceneDescriptor(puzzle2dFixtureSceneMarkers(fixture!));
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test", width: 1200, height: 800 });
      syncPuzzle2dScene(renderer, descriptor);
      const cam = fixture!.camera ?? { x: 0, y: 0, zoom: 0.2407 };
      renderer.setCameraSilent(cam.x, cam.y, cam.zoom);
      renderer.setSize(1200, 800, 1);
      (renderer as { pushSceneToWasmDriver(): void }).pushSceneToWasmDriver();
      const withEdges = renderer.session.encodedSceneHint();
      const stripped = JSON.parse((renderer as { descriptorJsonForWasmHost(): string }).descriptorJsonForWasmHost());
      stripped.edges = [];
      renderer.session.syncDescriptorJson(JSON.stringify(stripped));
      const withoutEdges = renderer.session.encodedSceneHint();
      expect(withEdges).toBeGreaterThan(withoutEdges);
      expect(withEdges - withoutEdges).toBeGreaterThan(100);
      renderer.dispose();
    });

    it("createPuzzle2dHostMount registers React 19 error reporters on the host root", () => {
      const canvas = document.createElement("canvas");
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      const mount = createPuzzle2dHostMount(renderer);
      expect(typeof mount.onUncaughtError).toBe("function");
      expect(typeof mount.onCaughtError).toBe("function");
      expect(typeof mount.onRecoverableError).toBe("function");
      renderer.dispose();
    });

    it("builds a flat scene descriptor from declarative markers", () => {
      const descriptor = buildPuzzle2dSceneDescriptor(
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
      const descriptor = buildPuzzle2dSceneDescriptor(
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
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const jsx = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={40} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>
          <Node id="b" radius={40} x={200} y={0}>
            <Handle handleKind="port" angle={Math.PI} id="b:h0" />
          </Node>
        </>,
      );
      syncPuzzle2dScene(renderer, jsx);
      const sourceHandle = renderer.scene.handles.get("a:h0");
      const targetHandle = renderer.scene.handles.get("b:h0");
      expect(sourceHandle).toBeDefined();
      expect(targetHandle).toBeDefined();
      renderer.scene.ingestWasmEdge(new Puzzle2dSceneEdge({ id: "edge-link-99", source: sourceHandle as Puzzle2dSceneHandle, target: targetHandle as Puzzle2dSceneHandle }));
      renderer.wasmHostAuthoredEdgeIds.add("edge-link-99");
      renderer.wasmHostAuthoredLinkByEdgeId.set("edge-link-99", { source: "a:h0", target: "b:h0" });
      const merged = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsx);
      expect(merged.edges.some((e) => e.id === "edge-link-99")).toBe(true);
      syncPuzzle2dScene(renderer, merged);
      expect(renderer.scene.edges.has("edge-link-99")).toBe(true);
      const merged2 = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsx);
      syncPuzzle2dScene(renderer, merged2);
      expect(renderer.scene.edges.has("edge-link-99")).toBe(true);
      const adopted = buildPuzzle2dSceneDescriptor(
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
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const jsx = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={40} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>
          <Node id="b" radius={40} x={200} y={0}>
            <Handle handleKind="port" angle={Math.PI} id="b:h0" />
          </Node>
        </>,
      );
      syncPuzzle2dScene(renderer, jsx);
      const sourceHandle = renderer.scene.handles.get("a:h0");
      const targetHandle = renderer.scene.handles.get("b:h0");
      expect(sourceHandle).toBeDefined();
      expect(targetHandle).toBeDefined();
      const edge = new Puzzle2dSceneEdge({
        id: "edge-link-map",
        source: sourceHandle as Puzzle2dSceneHandle,
        target: targetHandle as Puzzle2dSceneHandle,
      });
      renderer.scene.ingestWasmEdge(edge);
      renderer.wasmHostAuthoredEdgeIds.add("edge-link-map");
      renderer.wasmHostAuthoredLinkByEdgeId.set("edge-link-map", { source: "a:h0", target: "b:h0" });
      renderer.scene.remove(edge);
      expect(renderer.scene.edges.has("edge-link-map")).toBe(false);
      const merged = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsx);
      expect(merged.edges.some((e) => e.id === "edge-link-map")).toBe(true);
      syncPuzzle2dScene(renderer, merged);
      expect(renderer.scene.edges.has("edge-link-map")).toBe(true);
      renderer.dispose();
    });

    it("puzzle2dSyncFixtureDescriptorToAllAuthoringPeers gives every pane the same graph", () => {
      const fixture = parsePuzzle2dFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [{ id: "edge-real", source: "a:h0", target: "b:h0" }],
        nodes: [
          { handles: [{ angle: 0, id: "a:h0" }], id: "a", radius: 40, x: 0, y: 0 },
          { handles: [{ angle: Math.PI, id: "b:h0" }], id: "b", radius: 40, x: 200, y: 0 },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(fixture).toBeTruthy();
      const peerA = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const peerB = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const jsx = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={40} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>
          <Node id="b" radius={40} x={200} y={0}>
            <Handle handleKind="port" angle={Math.PI} id="b:h0" />
          </Node>
        </>,
      );
      syncPuzzle2dScene(peerA, jsx);
      syncPuzzle2dScene(peerB, jsx);
      const sourceHandle = peerA.scene.handles.get("a:h0");
      const targetHandle = peerA.scene.handles.get("b:h0");
      expect(sourceHandle).toBeDefined();
      expect(targetHandle).toBeDefined();
      peerA.scene.ingestWasmEdge(
        new Puzzle2dSceneEdge({
          id: "edge-ghost",
          source: sourceHandle as Puzzle2dSceneHandle,
          target: targetHandle as Puzzle2dSceneHandle,
        }),
      );
      peerA.wasmHostAuthoredEdgeIds.add("edge-ghost");
      peerA.wasmHostAuthoredLinkByEdgeId.set("edge-ghost", { source: "a:h0", target: "b:h0" });
      expect(peerA.scene.edges.has("edge-ghost")).toBe(true);
      expect(peerB.scene.edges.size).toBe(0);
      puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(fixture!);
      expect(peerA.scene.edges.size).toBe(1);
      expect(peerB.scene.edges.size).toBe(1);
      expect(peerA.scene.edges.has("edge-real")).toBe(true);
      expect(peerB.scene.edges.has("edge-real")).toBe(true);
      expect(peerA.scene.edges.has("edge-ghost")).toBe(false);
      peerA.dispose();
      peerB.dispose();
    });

    it("puzzle2dSyncFixtureDescriptorToAllAuthoringPeers clears edge suppressions so fixture edges reach scene", () => {
      const fixture = parsePuzzle2dFixtureV1({
        camera: { x: 0, y: 0, zoom: 1 },
        edges: [{ id: "edge-real", source: "a:h0", target: "b:h0" }],
        nodes: [
          { handles: [{ angle: 0, id: "a:h0" }], id: "a", radius: 40, x: 0, y: 0 },
          { handles: [{ angle: Math.PI, id: "b:h0" }], id: "b", radius: 40, x: 200, y: 0 },
        ],
        schema: "puzzle.2d.fixture/v1",
      });
      expect(fixture).toBeTruthy();
      const peer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const jsx = buildPuzzle2dSceneDescriptor(
        <>
          <Node id="a" radius={40} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>
          <Node id="b" radius={40} x={200} y={0}>
            <Handle handleKind="port" angle={Math.PI} id="b:h0" />
          </Node>
          <Edge id="edge-real" source="a:h0" target="b:h0" />
        </>,
      );
      syncPuzzle2dScene(peer, jsx);
      peer.pruneHostDeclarativeStructuralDelete({ kind: "edge", id: "edge-real" });
      peer.runWithoutSceneDeleteEvents(() => {
        const edge = peer.scene.edges.get("edge-real");
        if (edge) {
          peer.scene.remove(edge);
        }
      });
      expect(peer.scene.edges.has("edge-real")).toBe(false);
      expect(peer.isAuthoritativeStructuralRemovalSuppressed("edge", "edge-real")).toBe(true);
      puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(fixture!);
      expect(peer.isAuthoritativeStructuralRemovalSuppressed("edge", "edge-real")).toBe(false);
      expect(peer.scene.edges.has("edge-real")).toBe(true);
      peer.dispose();
    });

    it("keeps wasm-only link edges after graph drain by re-running JSX merge when children omit Edge markers", async () => {
      const restoreCanvas = installCanvasStub();
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);
      let readyRenderer: Puzzle2dRenderer | null = null;
      await act(async () => {
        root.render(
          <Puzzle2dCanvas
            camera={{ x: 0, y: 0, zoom: PUZZLE_2D_LOD_DETAIL_MIN_ZOOM }}
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
          </Puzzle2dCanvas>,
        );
        await Promise.resolve();
        await Promise.resolve();
      });
      const canvas = container.querySelector("canvas") as HTMLCanvasElement & { __puzzle2dRenderer?: Puzzle2dRenderer };
      const renderer = requireRenderer(canvas.__puzzle2dRenderer ?? readyRenderer);
      Object.defineProperty(canvas, "clientWidth", { configurable: true, value: 800 });
      Object.defineProperty(canvas, "clientHeight", { configurable: true, value: 600 });
      Object.defineProperty(canvas, "getBoundingClientRect", {
        configurable: true,
        value: () => ({ bottom: 600, height: 600, left: 0, right: 800, top: 0, width: 800, x: 0, y: 0 }),
      });
      expect(renderer.scene.getObjectById("a:h0")).toBeDefined();
      expect(renderer.getWasmHostSceneMergeResyncEpoch()).toBe(0);
      renderer.render();
      const nodeA = renderer.scene.getObjectById("a") as Puzzle2dSceneNode;
      const nodeB = renderer.scene.getObjectById("b") as Puzzle2dSceneNode;
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
          <Puzzle2dCanvas
            camera={{ x: 0, y: 0, zoom: PUZZLE_2D_LOD_DETAIL_MIN_ZOOM }}
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
          </Puzzle2dCanvas>,
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      syncPuzzle2dScene(
        renderer,
        buildPuzzle2dSceneDescriptor(
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
      const renderer = new Puzzle2dRenderer({ canvas, renderMode: "headless-test" });
      syncPuzzle2dScene(
        renderer,
        buildPuzzle2dSceneDescriptor(
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

    it("syncPuzzle2dScene ignores descriptor selected flags and reapplies interaction chrome", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const descriptor = buildPuzzle2dSceneDescriptor(<Node id="solo" radius={36} selected x={0} y={0} text="caption" />);
      syncPuzzle2dScene(renderer, descriptor);
      const node = renderer.scene.nodes.get("solo");
      expect(node?.selected).toBe(false);
      renderer.setSelectionIds(["solo"]);
      syncPuzzle2dScene(renderer, descriptor);
      expect(node?.selected).toBe(true);
      renderer.setSelectionIds([]);
      syncPuzzle2dScene(renderer, descriptor);
      expect(node?.selected).toBe(false);
      renderer.dispose();
    });

    it("buildPuzzle2dSceneDescriptor ignores opaque components (use secondary host for nested composition)", () => {
      function OpaqueScene(): ReactElement {
        return (
          <Node id="inner" radius={8} x={1} y={2}>
            <Handle handleKind="port" angle={0} id="inner.h" />
          </Node>
        );
      }
      const descriptor = buildPuzzle2dSceneDescriptor(
        <>
          <OpaqueScene />
        </>,
      );
      expect(descriptor.nodes).toHaveLength(0);
      expect(descriptor.handles).toHaveLength(0);
    });

    it("secondary host mounts handle under node without Puzzle2dCanvas", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const hostMount = createPuzzle2dHostMount(renderer);
      act(() => {
        updatePuzzle2dHostMount(
          hostMount,
          createElement(PUZZLE_2D_HOST_NODE, { draggable: true, id: "host-a-node", radius: 10, selected: false, visible: true, x: 0, y: 0 }, createElement(PUZZLE_2D_HOST_HANDLE, { angle: 0, id: "host-a-handle", selected: false, visible: true })),
          null,
        );
      });
      expect(renderer.scene.getObjectById("host-a-node")?.kind).toBe("node");
      expect(renderer.scene.getObjectById("host-a-handle")?.kind).toBe("handle");
      unmountPuzzle2dHostMount(hostMount);
      renderer.dispose();
    });

    it("host marker replacement does not emit nodeDelete", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const nodeDeletes: string[] = [];
      renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));
      const hostMount = createPuzzle2dHostMount(renderer);
      act(() => {
        updatePuzzle2dHostMount(
          hostMount,
          createElement(PUZZLE_2D_HOST_NODE, { id: "host-a", radius: 10, x: 0, y: 0 }, createElement(PUZZLE_2D_HOST_HANDLE, { angle: 0, id: "host-a.h" })),
          null,
        );
      });
      act(() => {
        updatePuzzle2dHostMount(
          hostMount,
          createElement(PUZZLE_2D_HOST_NODE, { height: 20, id: "host-b", shape: "rectangle", width: 30, x: 1, y: 2 }, createElement(PUZZLE_2D_HOST_HANDLE, { angle: 0, id: "host-b.h" })),
          null,
        );
      });
      expect(nodeDeletes).toEqual([]);
      unmountPuzzle2dHostMount(hostMount);
      renderer.dispose();
    });

    it("appending a host marker keeps existing nodes and emits no nodeDelete", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const nodeDeletes: string[] = [];
      renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));
      const hostMount = createPuzzle2dHostMount(renderer);
      const mkNode = (id: string, x: number) =>
        createElement(
          PUZZLE_2D_HOST_NODE,
          { id, radius: 10, x, y: 0 },
          createElement(PUZZLE_2D_HOST_HANDLE, { angle: 0, id: `${id}.h` }),
        );
      const initial = Array.from({ length: 10 }, (_, i) => mkNode(`n${i}`, i * 30));
      act(() => {
        updatePuzzle2dHostMount(hostMount, createElement(Fragment, null, ...initial), null);
      });
      expect(renderer.scene.nodes.size).toBe(10);
      nodeDeletes.length = 0;
      act(() => {
        updatePuzzle2dHostMount(hostMount, createElement(Fragment, null, ...initial, mkNode("n10", 300)), null);
      });
      expect(renderer.scene.nodes.size).toBe(11);
      expect(nodeDeletes).toEqual([]);
      unmountPuzzle2dHostMount(hostMount);
      renderer.dispose();
    });

    it("mounts handle children for flat host markers", async () => {
      const restoreCanvas = installCanvasStub();
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);

      await act(async () => {
        root.render(
          <Puzzle2dCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" width={160}>
            <Node id="direct" radius={10} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="direct.h" />
            </Node>
          </Puzzle2dCanvas>,
        );
        await Promise.resolve();
      });

      const canvas = container.querySelector("canvas");
      const renderer = (canvas as HTMLCanvasElement & { __puzzle2dRenderer?: Puzzle2dRenderer }).__puzzle2dRenderer;
      expect(renderer?.scene.getObjectById("direct")?.kind).toBe("node");
      expect(renderer?.scene.getObjectById("direct.h")?.kind).toBe("handle");

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
          <Puzzle2dCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" width={160}>
            <WrappedScene />
          </Puzzle2dCanvas>,
        );
        await Promise.resolve();
      });

      const canvas = container.querySelector("canvas");
      const renderer = (canvas as HTMLCanvasElement & { __puzzle2dRenderer?: Puzzle2dRenderer }).__puzzle2dRenderer;
      expect(renderer?.scene.getObjectById("wrapped")?.kind).toBe("node");
      expect(renderer?.scene.getObjectById("wrapped.h")?.kind).toBe("handle");

      await act(async () => {
        root.unmount();
      });
      restoreCanvas();
    });

    it("syncs declarative updates into stable imperative instances", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const firstDescriptor = buildPuzzle2dSceneDescriptor(
        <Node draggable id="a" radius={24} x={10} y={20}>
          <Handle handleKind="port" angle={0} id="a:h0" />
        </Node>,
      );
      syncPuzzle2dScene(renderer, firstDescriptor);

      const firstNode = renderer.scene.getObjectById("a");
      const secondDescriptor = buildPuzzle2dSceneDescriptor(
        <Node draggable id="a" radius={30} x={40} y={50}>
          <Handle handleKind="port" angle={Math.PI / 2} id="a:h0" />
        </Node>,
      );
      syncPuzzle2dScene(renderer, secondDescriptor);

      const secondNode = renderer.scene.getObjectById("a");
      expect(secondNode).toBe(firstNode);
      expect(secondNode?.kind).toBe("node");
      expect((secondNode as Puzzle2dSceneNode).x).toBe(40);
      expect((secondNode as Puzzle2dSceneNode).radius).toBe(30);

      renderer.dispose();
    });

    it("syncs handleKind from declarative handles into scene instances", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const descriptor = buildPuzzle2dSceneDescriptor(
        <Node id="n" radius={20} x={0} y={0}>
          <Handle angle={0} handleKind="slot-a" id="h1" />
        </Node>,
      );
      syncPuzzle2dScene(renderer, descriptor);
      const h = renderer.scene.getObjectById("h1") as Puzzle2dSceneHandle;
      expect(h.handleKind).toBe("slot-a");
      renderer.dispose();
    });

    it("replaces the imperative node when declarative shape changes from circle to rectangle", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const circleDescriptor = buildPuzzle2dSceneDescriptor(
        <Node id="a" radius={20} x={0} y={0}>
          <Handle handleKind="port" angle={0} id="a:h0" />
        </Node>,
      );
      syncPuzzle2dScene(renderer, circleDescriptor);
      const firstNode = renderer.scene.getObjectById("a");
      const rectDescriptor = buildPuzzle2dSceneDescriptor(
        <Node height={30} id="a" shape="rectangle" width={40} x={0} y={0}>
          <Handle handleKind="port" angle={0} id="a:h0" />
        </Node>,
      );
      syncPuzzle2dScene(renderer, rectDescriptor);
      const secondNode = renderer.scene.getObjectById("a");
      expect(secondNode).not.toBe(firstNode);
      expect((secondNode as Puzzle2dSceneNode).shape).toBe("rectangle");
      expect((secondNode as Puzzle2dSceneNode).width).toBe(40);
      renderer.dispose();
    });

    it("sync shape replacement does not emit nodeDelete", () => {
      const renderer = new Puzzle2dRenderer({ renderMode: "headless-test" });
      const nodeDeletes: string[] = [];
      renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));
      syncPuzzle2dScene(
        renderer,
        buildPuzzle2dSceneDescriptor(
          <Node id="a" radius={20} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>,
        ),
      );
      syncPuzzle2dScene(
        renderer,
        buildPuzzle2dSceneDescriptor(
          <Node height={30} id="a" shape="rectangle" width={40} x={0} y={0}>
            <Handle handleKind="port" angle={0} id="a:h0" />
          </Node>,
        ),
      );
      expect(nodeDeletes).toEqual([]);
      renderer.dispose();
    });

    it("mounts Puzzle2dCanvas and updates scene objects when JSX props change", async () => {
      const restoreCanvas = installCanvasStub();
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);
      let readyRenderer: Puzzle2dRenderer | null = null;
      const onReadyNoop = (): void => undefined;

      await act(async () => {
        root.render(
          <Puzzle2dCanvas
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
          </Puzzle2dCanvas>,
        );
        await Promise.resolve();
      });
      expect(readyRenderer).not.toBeNull();
      const createdRenderer = requireRenderer(readyRenderer);
      expect(createdRenderer.scene.getObjectById("edge-1")?.kind).toBe("edge");
      const canvasEl = container.querySelector<HTMLCanvasElement>('[data-testid="puzzle2d-canvas"]');
      expect(canvasEl?.className).toContain("outline-none");
      expect(canvasEl?.style.outline).toBe("none");

      await act(async () => {
        root.render(
          <Puzzle2dCanvas camera={{ x: 20, y: 10, zoom: 1.2 }} height={480} onReady={onReadyNoop} renderMode="headless-test" width={640}>
            <Node draggable id="a" radius={28} x={120} y={40}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
            <Node id="b" radius={28} x={180} y={0}>
              <Handle handleKind="port" angle={Math.PI} id="b:h0" />
            </Node>
            <Edge id="edge-1" source="a:h0" target="b:h0" />
          </Puzzle2dCanvas>,
        );
        await Promise.resolve();
      });
      /** Secondary host commit can trail the outer `act` tick; mirror JSX into the imperative scene before reading coordinates. */
      const movedDescriptor = buildPuzzle2dSceneDescriptor(
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
      syncPuzzle2dScene(createdRenderer, movedDescriptor);
      const canvasAfterMove = container.querySelector("canvas");
      const rendererAfterMove = requireRenderer((canvasAfterMove as HTMLCanvasElement & { __puzzle2dRenderer?: Puzzle2dRenderer | undefined }).__puzzle2dRenderer);
      const movedNode = rendererAfterMove.scene.getObjectById("a") as Puzzle2dSceneNode;
      expect(movedNode.x).toBe(120);
      expect(movedNode.y).toBe(40);
      expect(rendererAfterMove.getCameraSnapshot()).toEqual({ x: 20, y: 10, zoom: 1.2 });

      await act(async () => {
        root.unmount();
      });
      restoreCanvas();
    });

    it("does not dispose Puzzle2dRenderer when only selection props change", async () => {
      const restoreCanvas = installCanvasStub();
      const disposeSpy = vi.spyOn(Puzzle2dRenderer.prototype, "dispose");
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);

      await act(async () => {
        root.render(
          <Puzzle2dCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" selectionMethod="rectangle" selectionMode="additive" selectionTargets={{ nodes: true, edges: false, handles: false }} width={160}>
            <Node id="a" radius={12} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
          </Puzzle2dCanvas>,
        );
        await Promise.resolve();
      });

      disposeSpy.mockClear();

      await act(async () => {
        root.render(
          <Puzzle2dCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" selectionMethod="lasso" selectionMode="invertive" selectionTargets={{ nodes: false, edges: true, handles: false }} width={160}>
            <Node id="a" radius={12} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
          </Puzzle2dCanvas>,
        );
        await Promise.resolve();
      });

      expect(disposeSpy).not.toHaveBeenCalled();
      const canvas = container.querySelector("canvas");
      const renderer = requireRenderer((canvas as HTMLCanvasElement & { __puzzle2dRenderer?: Puzzle2dRenderer | undefined }).__puzzle2dRenderer ?? null);
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

    it("defers Puzzle2dCanvas children until the renderer exists so usePuzzle2dRenderer hooks do not throw", async () => {
      const restoreCanvas = installCanvasStub();
      const container = document.createElement("div");
      document.body.appendChild(container);
      const root = createRoot(container);

      await act(async () => {
        root.render(
          <Puzzle2dCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" width={160}>
            <Puzzle2dSelectListenerStub />
            <Node draggable id="a" radius={12} x={0} y={0}>
              <Handle handleKind="port" angle={0} id="a:h0" />
            </Node>
          </Puzzle2dCanvas>,
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
