// #region 🧲Header
/** @emoji 🌳 `@semio-tech/dag-react` — WASM DAG renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef } from "react";
import { syncSessionVelloTheme, resolveColorHex, resolveSemanticColorHex, tokenVar } from "@semio-tech/ui-styling";
import { useVelloThemeSync } from "@semio-tech/ui-react";
import initDagWasm, { DagSession, initSync } from "../pkg/mathematical_graph_port_directed_dag.js";

// #region 🔖GpuWasmBridge
if (import.meta.env.VITEST) {
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../pkg/mathematical_graph_port_directed_dag_bg.wasm");
  initSync({ module: readFileSync(wasmPath) });
} else {
  await initDagWasm();
}

export async function ensureDagWasmLoaded(): Promise<void> {
  await initDagWasm();
}

export { DagSession };
// #endregion 🔖GpuWasmBridge

// #region 🔖Lod
/** @emoji 📶 WASM draw LOD tier label (matches `drawLodLabel` / `setForcedDrawLodLabel`). */
export type DagDrawLodKind = "compact" | "detail" | "micro" | "minimap" | "normal" | "overview";

const DAG_DRAW_LOD_KINDS: readonly DagDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

/** @emoji ✅ True when `label` is a pinned WASM draw LOD tier. */
export function isDagDrawLodKind(label: string): label is DagDrawLodKind {
  return (DAG_DRAW_LOD_KINDS as readonly string[]).includes(label);
}

/** @emoji 📶 Compile-time LOD row mirrored from {@link DagSession.lodScaleJson}. */
export interface DagLodEntry {
  readonly id: DagDrawLodKind;
  readonly name: string;
  readonly description: string;
  readonly maxZoom: number;
}

let dagLodScaleCache: readonly DagLodEntry[] | null = null;

function parseDagLodScaleJson(raw: string): readonly DagLodEntry[] {
  const rows = JSON.parse(raw) as Array<{ id?: string; name?: string; description?: string; maxZoom?: number }>;
  if (!Array.isArray(rows)) {
    return [];
  }
  const out: DagLodEntry[] = [];
  for (const row of rows) {
    if (typeof row.id !== "string" || !isDagDrawLodKind(row.id) || typeof row.maxZoom !== "number") {
      continue;
    }
    out.push({
      id: row.id,
      name: typeof row.name === "string" ? row.name : row.id,
      description: typeof row.description === "string" ? row.description : "",
      maxZoom: row.maxZoom,
    });
  }
  return out;
}

/** @emoji 📶 Fixed LOD table declared in DAG WASM (single source of truth). */
export function getDagLodScale(): readonly DagLodEntry[] {
  if (!dagLodScaleCache) {
    dagLodScaleCache = parseDagLodScaleJson(new DagSession().lodScaleJson());
  }
  return dagLodScaleCache;
}

/** @emoji 📶 Ordered LOD tier ids for play window selects. */
export function dagPlayLodTiers(): readonly DagDrawLodKind[] {
  return getDagLodScale().map((lod) => lod.id);
}

/** @emoji 📶 Select value: camera zoom picks the draw LOD band. */
export const DAG_LOD_MODE_AUTOMATIC = "automatic" as const;

/** @emoji 📶 DAG play / window LOD select value (`automatic` or a pinned {@link DagDrawLodKind}). */
export type DagLodModeKind = typeof DAG_LOD_MODE_AUTOMATIC | DagDrawLodKind;

/** @emoji 📶 Maps a window LOD select value to canvas LOD fields. */
export function dagLodCanvasProps(mode: DagLodModeKind): { automaticLod: boolean; lod?: DagDrawLodKind } {
  if (mode === DAG_LOD_MODE_AUTOMATIC) {
    return { automaticLod: true };
  }
  return { automaticLod: false, lod: mode };
}

/** @emoji 📶 Automatic LOD select row label showing the live zoom-derived tier. */
export function dagLodAutomaticSelectLabel(effectiveTier: DagDrawLodKind): string {
  const row = getDagLodScale().find((lod) => lod.id === effectiveTier);
  const name = row?.name ?? effectiveTier;
  return `Automatic · ${name}`;
}

export function dagPlayLodTierMenuLabel(tier: DagDrawLodKind): string {
  const row = getDagLodScale().find((lod) => lod.id === tier);
  return row?.name ?? tier.charAt(0).toUpperCase() + tier.slice(1);
}
// #endregion 🔖Lod

// #region 🔖Fixture
export interface DagPort {
  readonly id: string;
  readonly label: string;
}

export interface DagMedia {
  readonly kind: "image" | "svg" | "pdf" | "video";
  readonly src: string;
}

interface DagNodeBase {
  readonly id: string;
  readonly name: string;
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
}

export interface DagComputationNode extends DagNodeBase {
  readonly kind: "computation";
  readonly inputs: readonly DagPort[];
  readonly outputs: readonly DagPort[];
}

export interface DagSliderNode extends DagNodeBase {
  readonly kind: "slider";
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly value: number;
  readonly output: DagPort;
}

export interface DagSelectNode extends DagNodeBase {
  readonly kind: "select";
  readonly options: readonly string[];
  readonly selected: number;
  readonly output: DagPort;
}

export interface DagScreenNode extends DagNodeBase {
  readonly kind: "screen";
  readonly media?: DagMedia;
  readonly input: DagPort;
}

export type DagNode = DagComputationNode | DagSliderNode | DagSelectNode | DagScreenNode;

export interface DagFixture {
  readonly schema: "dag.fixture";
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly nodes: readonly DagNode[];
  readonly edges: readonly { readonly id: string; readonly source: string; readonly target: string }[];
}

const DAG_DEMO_SCREEN_SVG =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E";

export const DAG_DEFAULT_FIXTURE: DagFixture = {
  schema: "dag.fixture",
  camera: { x: 0, y: 0, zoom: 1 },
  nodes: [
    { id: "slider", name: "Amount", kind: "slider", x: -400, y: -40, width: 70, height: 14, min: 0, max: 10, step: 0.5, value: 5, output: { id: "out", label: "value" } },
    { id: "mode", name: "Mode", kind: "select", x: -400, y: 80, width: 180, height: 80, options: ["Add", "Multiply", "Max"], selected: 0, output: { id: "out", label: "mode" } },
    { id: "scale", name: "Scale", kind: "computation", x: -120, y: -40, width: 160, height: 72, inputs: [{ id: "in", label: "value" }], outputs: [{ id: "out", label: "scaled" }] },
    {
      id: "combine",
      name: "Combine",
      kind: "computation",
      x: 120,
      y: 0,
      width: 160,
      height: 96,
      inputs: [
        { id: "a", label: "a" },
        { id: "b", label: "b" },
      ],
      outputs: [{ id: "out", label: "merged" }],
    },
    {
      id: "screen",
      name: "Preview",
      kind: "screen",
      x: 400,
      y: 0,
      width: 200,
      height: 140,
      media: { kind: "svg", src: DAG_DEMO_SCREEN_SVG },
      input: { id: "in", label: "result" },
    },
  ],
  edges: [
    { id: "e1", source: "slider:out", target: "scale:in" },
    { id: "e2", source: "scale:out", target: "combine:a" },
    { id: "e3", source: "mode:out", target: "combine:b" },
    { id: "e4", source: "combine:out", target: "screen:in" },
  ],
};

export function dagFixtureToJson(fixture: DagFixture): string {
  return JSON.stringify(fixture);
}

export type DagFixtureEditOp =
  | { readonly op: "setDocument"; readonly document: DagFixture }
  | { readonly op: "renameNode"; readonly oldId: string; readonly newId: string }
  | { readonly op: "patchNode"; readonly nodeId: string; readonly field: string; readonly value: unknown }
  | { readonly op: "patchNodes"; readonly nodeIds: readonly string[]; readonly field: string; readonly value: unknown };

function dagRemapPortId(port: string, oldId: string, newId: string): string {
  return port.startsWith(`${oldId}:`) ? `${newId}:${port.slice(oldId.length + 1)}` : port;
}

/** @emoji 🚪 Applies one semantic DAG fixture edit (CQRS projection applier). */
export function applyDagFixtureEditOp(fixture: DagFixture, op: DagFixtureEditOp): DagFixture {
  switch (op.op) {
    case "setDocument":
      return op.document;
    case "renameNode": {
      const trimmed = op.newId.trim();
      if (!trimmed || trimmed === op.oldId || fixture.nodes.some((node) => node.id === trimmed)) {
        return fixture;
      }
      const nodes = fixture.nodes.map((node) => (node.id === op.oldId ? ({ ...node, id: trimmed } as DagNode) : node));
      const edges = fixture.edges.map((edge) => ({
        ...edge,
        source: dagRemapPortId(edge.source, op.oldId, trimmed),
        target: dagRemapPortId(edge.target, op.oldId, trimmed),
      }));
      return { ...fixture, nodes, edges };
    }
    case "patchNode":
      return applyDagFixtureEditOp(fixture, {
        op: "patchNodes",
        nodeIds: [op.nodeId],
        field: op.field,
        value: op.value,
      });
    case "patchNodes": {
      const targets = new Set(op.nodeIds);
      const nodes = fixture.nodes.map((node) => {
        if (!targets.has(node.id)) return node;
        if (op.field === "value" || op.field === "min" || op.field === "max" || op.field === "step" || op.field === "selected") {
          const numeric = typeof op.value === "number" ? op.value : Number(op.value);
          if (!Number.isFinite(numeric)) return node;
          return { ...node, [op.field]: numeric } as DagNode;
        }
        if (typeof op.value !== "string") return node;
        return { ...node, [op.field]: op.value } as DagNode;
      });
      return { ...fixture, nodes };
    }
  }
}

/** @emoji ↩️ Inverts a DAG fixture edit from the pre-apply projection. */
export function backwardsDagFixtureEditOp(fixture: DagFixture, op: DagFixtureEditOp): readonly DagFixtureEditOp[] {
  switch (op.op) {
    case "setDocument":
      return [{ op: "setDocument", document: fixture }];
    case "renameNode":
      return [{ op: "renameNode", oldId: op.newId, newId: op.oldId }];
    case "patchNode": {
      const node = fixture.nodes.find((row) => row.id === op.nodeId);
      if (!node) return [{ op: "setDocument", document: fixture }];
      return [{ op: "patchNode", nodeId: op.nodeId, field: op.field, value: (node as Record<string, unknown>)[op.field] }];
    }
    case "patchNodes":
      return op.nodeIds.flatMap((nodeId) => {
        const node = fixture.nodes.find((row) => row.id === nodeId);
        if (!node) return [];
        return [{ op: "patchNode", nodeId, field: op.field, value: (node as Record<string, unknown>)[op.field] }];
      });
  }
}

/** @emoji 📊 Returns the DAG fixture edit payload for persistence diffs. */
export function diffDagFixtureEditOp(_fixture: DagFixture, operation: DagFixtureEditOp): unknown {
  return operation;
}

export type DagLayoutOrientation = "leftRight" | "topBottom";

export interface DagReorganizeRequest {
  readonly epoch: number;
  readonly optionsJson: string;
}

interface DagNodeOverlayRect {
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
}

interface DagNodeOverlayEntry {
  readonly id: string;
  readonly mediaKind: string;
  readonly src: string;
  readonly rect: DagNodeOverlayRect;
}
// #endregion 🔖Fixture

// #region 🔖NodeOverlays
function createMediaOverlayElement(mediaKind: string, src: string): HTMLElement {
  if (mediaKind === "video") {
    const video = document.createElement("video");
    video.src = src;
    video.muted = true;
    video.loop = true;
    video.autoplay = true;
    video.playsInline = true;
    return video;
  }
  if (mediaKind === "pdf") {
    const embed = document.createElement("embed");
    embed.src = src;
    embed.type = "application/pdf";
    return embed;
  }
  const img = document.createElement("img");
  img.src = src;
  img.alt = "";
  return img;
}

function syncDagNodeOverlays(container: HTMLDivElement, session: DagSession, elements: Map<string, HTMLElement>): void {
  let overlays: DagNodeOverlayEntry[] = [];
  try {
    const json = session.nodeOverlaysJson();
    if (json) overlays = JSON.parse(json) as DagNodeOverlayEntry[];
  } catch {
    overlays = [];
  }
  const active = new Set<string>();
  for (const entry of overlays) {
    active.add(entry.id);
    let el = elements.get(entry.id);
    if (!el) {
      el = createMediaOverlayElement(entry.mediaKind, entry.src);
      el.setAttribute("data-dag-overlay-id", entry.id);
      el.style.position = "absolute";
      el.style.pointerEvents = "none";
      el.style.objectFit = "contain";
      container.appendChild(el);
      elements.set(entry.id, el);
    } else if (el instanceof HTMLImageElement || el instanceof HTMLVideoElement) {
      if (el.src !== entry.src) el.src = entry.src;
    } else if (el instanceof HTMLEmbedElement && el.src !== entry.src) {
      el.src = entry.src;
    }
    el.style.left = `${entry.rect.x}px`;
    el.style.top = `${entry.rect.y}px`;
    el.style.width = `${entry.rect.w}px`;
    el.style.height = `${entry.rect.h}px`;
  }
  for (const [id, el] of elements) {
    if (!active.has(id)) {
      el.remove();
      elements.delete(id);
    }
  }
}
// #endregion 🔖NodeOverlays

// #region 🔖DagCanvas
export interface DagCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
  readonly reorganize?: DagReorganizeRequest;
  readonly onFixtureChange?: (fixtureJson: string) => void;
  readonly automaticLod?: boolean;
  readonly lod?: DagDrawLodKind;
  readonly onLodChange?: (lod: DagDrawLodKind) => void;
}

export function DagCanvas({ fixtureJson, className, reorganize, onFixtureChange, automaticLod = true, lod, onLodChange }: DagCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const overlayRef = useRef<HTMLDivElement>(null);
  const sessionRef = useRef<DagSession | null>(null);
  const overlayElementsRef = useRef(new Map<string, HTMLElement>());
  const rafRef = useRef<number | null>(null);
  const onFixtureChangeRef = useRef(onFixtureChange);
  const onLodChangeRef = useRef(onLodChange);
  const lastAutomaticLodRef = useRef<boolean | null>(null);
  const lastForcedLodRef = useRef<string | null>(null);
  const lastReportedLodRef = useRef<DagDrawLodKind | null>(null);

  const syncVelloTheme = useCallback(() => {
    syncSessionVelloTheme(sessionRef.current);
  }, []);

  useVelloThemeSync(syncVelloTheme);

  const syncLodMode = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    const nextAutomatic = automaticLod ?? true;
    if (lastAutomaticLodRef.current !== nextAutomatic) {
      session.setAutomaticLod(nextAutomatic);
      lastAutomaticLodRef.current = nextAutomatic;
    }
    const forced = nextAutomatic ? "" : lod && isDagDrawLodKind(lod) ? lod : "";
    if (lastForcedLodRef.current !== forced) {
      session.setForcedDrawLodLabel(forced);
      lastForcedLodRef.current = forced;
    }
  }, [automaticLod, lod]);

  const reportDrawLod = useCallback(() => {
    const session = sessionRef.current;
    if (!session || !onLodChangeRef.current) return;
    try {
      const label = session.drawLodLabel();
      if (!isDagDrawLodKind(label)) return;
      if (lastReportedLodRef.current === label) return;
      lastReportedLodRef.current = label;
      onLodChangeRef.current(label);
    } catch {
      /* session not ready */
    }
  }, []);

  const renderFrame = useCallback(() => {
    const session = sessionRef.current;
    const overlayRoot = overlayRef.current;
    syncLodMode();
    if (session && overlayRoot) {
      try {
        syncDagNodeOverlays(overlayRoot, session, overlayElementsRef.current);
      } catch {
        /* overlays not ready */
      }
    }
    try {
      syncVelloTheme();
      session?.renderFrame();
      reportDrawLod();
    } catch {
      /* gpu not ready */
    }
  }, [reportDrawLod, syncLodMode, syncVelloTheme]);

  useEffect(() => {
    onFixtureChangeRef.current = onFixtureChange;
  }, [onFixtureChange]);

  useEffect(() => {
    onLodChangeRef.current = onLodChange;
  }, [onLodChange]);

  useEffect(() => {
    lastAutomaticLodRef.current = null;
    lastForcedLodRef.current = null;
    renderFrame();
  }, [automaticLod, lod, renderFrame]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !reorganize || reorganize.epoch <= 0) return;
    try {
      session.reorganize(reorganize.optionsJson);
      console.log("[DEBUG] dag canvas reorganized:", session.fixtureJson());
      onFixtureChangeRef.current?.(session.fixtureJson());
      renderFrame();
    } catch (err) {
      console.log(`[DEBUG] dag canvas reorganize failed: ${String(err)}`);
    }
  }, [reorganize?.epoch, reorganize?.optionsJson, renderFrame]);

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    const session = new DagSession();
    sessionRef.current = session;
    const json = fixtureJson ?? dagFixtureToJson(DAG_DEFAULT_FIXTURE);
    session.loadFixtureJson(json);
    console.log("[DEBUG] dag canvas loaded fixture");
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    const initW = Math.max(1, Math.round(rect.width));
    const initH = Math.max(1, Math.round(rect.height));
    canvas.width = Math.round(initW * dpr);
    canvas.height = Math.round(initH * dpr);
    canvas.style.width = `${initW}px`;
    canvas.style.height = `${initH}px`;
    void session.attachCanvas(canvas, initW, initH, dpr).then(() => {
      const resize = () => {
        const rect = container.getBoundingClientRect();
        const dpr = globalThis.devicePixelRatio || 1;
        const w = Math.max(1, Math.round(rect.width));
        const h = Math.max(1, Math.round(rect.height));
        canvas.width = Math.round(w * dpr);
        canvas.height = Math.round(h * dpr);
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;
        session.setSize(w, h, dpr);
        renderFrame();
      };
      resize();
      const ro = new ResizeObserver(resize);
      ro.observe(container);
      const visualViewport = globalThis.visualViewport;
      visualViewport?.addEventListener("resize", resize);
      const tick = () => {
        renderFrame();
        rafRef.current = requestAnimationFrame(tick);
      };
      rafRef.current = requestAnimationFrame(tick);
      const onPointerDown = (ev: PointerEvent) => {
        const r = canvas.getBoundingClientRect();
        session.pointerDown(ev.clientX - r.left, ev.clientY - r.top, ev.shiftKey);
        renderFrame();
      };
      const onPointerMove = (ev: PointerEvent) => {
        const r = canvas.getBoundingClientRect();
        session.pointerMove(ev.clientX - r.left, ev.clientY - r.top);
        renderFrame();
      };
      const onPointerUp = (ev: PointerEvent) => {
        const r = canvas.getBoundingClientRect();
        const sx = ev.clientX - r.left;
        const sy = ev.clientY - r.top;
        session.pointerUp(sx, sy);
        try {
          console.log("[DEBUG] dag fixture after pointer:", session.fixtureJson());
        } catch {
          /* fixture not ready */
        }
        renderFrame();
      };
      canvas.addEventListener("pointerdown", onPointerDown);
      canvas.addEventListener("pointermove", onPointerMove);
      canvas.addEventListener("pointerup", onPointerUp);
      canvas.addEventListener("pointerleave", onPointerUp);
      return () => {
        ro.disconnect();
        visualViewport?.removeEventListener("resize", resize);
        canvas.removeEventListener("pointerdown", onPointerDown);
        canvas.removeEventListener("pointermove", onPointerMove);
        canvas.removeEventListener("pointerup", onPointerUp);
        canvas.removeEventListener("pointerleave", onPointerUp);
        if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      };
    });
    return () => {
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      for (const el of overlayElementsRef.current.values()) el.remove();
      overlayElementsRef.current.clear();
      sessionRef.current = null;
    };
  }, [fixtureJson, renderFrame]);

  return (
    <div ref={containerRef} className={className ?? "relative h-full w-full min-h-0 min-w-0 bg-canvas"}>
      <canvas ref={canvasRef} className="block h-full w-full touch-none" />
      <div ref={overlayRef} className="pointer-events-none absolute inset-0 overflow-hidden" data-dag-media-overlays="" />
    </div>
  );
}
// #endregion 🔖DagCanvas

// #region 🔖LabelOverlay
const DAG_LABEL_SCREEN_PX = 11;
const DAG_LABEL_FONT_FAMILY = "ui-sans-serif, system-ui, sans-serif";

/** @emoji 🏷️ One node or port label row from {@link DagOverlaySession.labelOverlayPaintStateJson}. */
export interface DagLabelOverlayRow {
  readonly id: string;
  readonly kind?: "port" | "node";
  readonly text: string;
  readonly layout: "horizontal" | "vertical";
  readonly align?: "left" | "center" | "right";
  readonly x: number;
  readonly y: number;
  readonly nodeW: number;
  readonly nodeH: number;
  readonly fontScreenPx?: number;
  readonly maxScreenH?: number;
  readonly ghost?: boolean;
}

/** @emoji 🎨 Label overlay paint payload from DAG WASM. */
export interface DagLabelOverlayPaintState {
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly width: number;
  readonly height: number;
  readonly labels: readonly DagLabelOverlayRow[];
}

/** @emoji 🎯 Area-select preview snapshot. */
export interface DagPreselectSnapshot {
  readonly ids: readonly string[];
  readonly removedIds: readonly string[];
}

/** @emoji 📦 Screen-space selection union bounds. */
export interface DagSelectionUnionBoundsScreen {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

/** @emoji 🔌 Minimal DAG session surface for label overlay painting. */
export interface DagOverlaySession {
  labelOverlayPaintStateJson(): string;
}

/** @emoji 🎯 Hover/selection state fed into label overlay painting. */
export interface DagLabelOverlayInteraction {
  readonly hoveredId: string | null;
  readonly selectedIds: readonly string[];
  readonly preselect: DagPreselectSnapshot;
  readonly dimmedIds?: readonly string[];
}

/** @emoji 🖱️ Area-select method for marquee overlay computation. */
export type DagSelectionMethod = "rectangle" | "lasso";

/** @emoji 📐 Marquee overlay state for {@link computeDagMarqueeOverlay}. */
export type DagMarqueeOverlayState =
  | { readonly coverage: "full" | "partial"; readonly shape: "rect"; readonly rect: { readonly x: number; readonly y: number; readonly width: number; readonly height: number } }
  | { readonly coverage: "full" | "partial"; readonly shape: "polygon"; readonly points: readonly { readonly x: number; readonly y: number }[] };

/** @emoji 🌍 Maps world coordinates to screen space. */
export function dagWorldToScreen(
  point: { readonly x: number; readonly y: number },
  camera: { readonly x: number; readonly y: number; readonly zoom: number },
  width: number,
  height: number,
): { readonly x: number; readonly y: number } {
  return {
    x: (point.x - camera.x) * camera.zoom + width / 2,
    y: (point.y - camera.y) * camera.zoom + height / 2,
  };
}

function dagClampLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(4, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.2 <= maxH) {
    return px;
  }
  let low = 4;
  let high = px;
  let best = 4;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
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

function dagClampPortLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(8, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.25 <= maxH) {
    return px;
  }
  let low = 8;
  let high = px;
  let best = 8;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
    if (ctx.measureText(text).width <= maxW) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

/** @emoji 🎯 Committed selection vs area-select preview chrome for label overlays. */
export function dagElementInteractionChrome(
  selectionIds: Iterable<string>,
  preselection: DagPreselectSnapshot,
): { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> } {
  if (!preselection.ids.length && !preselection.removedIds.length) {
    return { selectedIds: new Set(selectionIds), highlightedIds: new Set() };
  }
  return { selectedIds: new Set(preselection.ids), highlightedIds: new Set(preselection.removedIds) };
}

/** @emoji 🎨 Resolves label fill color from interaction chrome. */
export function dagOverlayLabelFill(
  nodeId: string,
  ghost: boolean,
  hoveredId: string | null,
  chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> },
  dimmedIds: readonly string[] = [],
): string {
  if (ghost) {
    return resolveColorHex(tokenVar("secondary"), "secondary");
  }
  if (dimmedIds.includes(nodeId)) {
    return resolveSemanticColorHex("border-element-color", "gray");
  }
  if (chrome.selectedIds.has(nodeId)) {
    return resolveSemanticColorHex("border-emphasized-color", "dark");
  }
  if (chrome.highlightedIds.has(nodeId)) {
    return resolveColorHex(tokenVar("secondary"), "secondary");
  }
  if (hoveredId === nodeId) {
    return resolveSemanticColorHex("border-emphasized-color", "dark");
  }
  return resolveSemanticColorHex("border-element-color", "gray");
}

/** @emoji 🏷️ Paints node and port labels on a 2D overlay canvas. */
export function paintDagLabelOverlays(
  labelOverlayPaintStateJson: string,
  canvas: HTMLCanvasElement,
  width: number,
  height: number,
  dpr: number,
  interaction: DagLabelOverlayInteraction,
): void {
  let state: DagLabelOverlayPaintState;
  try {
    state = JSON.parse(labelOverlayPaintStateJson) as DagLabelOverlayPaintState;
  } catch {
    return;
  }
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pixelW = Math.max(1, Math.round(width * dpr));
  const pixelH = Math.max(1, Math.round(height * dpr));
  if (canvas.width !== pixelW || canvas.height !== pixelH) {
    canvas.width = pixelW;
    canvas.height = pixelH;
  }
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, width, height);
  const zoom = Math.max(0.05, Number(state.camera?.zoom) || 1);
  const camera = {
    x: Number(state.camera?.x) || 0,
    y: Number(state.camera?.y) || 0,
    zoom,
  };
  const viewportW = Number(state.width) || width;
  const viewportH = Number(state.height) || height;
  const chrome = dagElementInteractionChrome(interaction.selectedIds, interaction.preselect);
  const dimmedIds = interaction.dimmedIds ?? [];
  const inset = 0.88;
  for (const row of state.labels ?? []) {
    const text = typeof row.text === "string" ? row.text.trim() : "";
    if (!text) continue;
    const anchor = dagWorldToScreen({ x: Number(row.x), y: Number(row.y) }, camera, viewportW, viewportH);
    const isPort = row.kind === "port" || row.align === "left" || row.align === "right";
    const maxW = Math.max(4, Number(row.nodeW) * zoom * inset);
    const maxH = Math.max(
      4,
      isPort && Number.isFinite(Number(row.maxScreenH)) && Number(row.maxScreenH) > 0
        ? Number(row.maxScreenH)
        : Number(row.nodeH) * zoom * inset,
    );
    const fontScreenPx = Number(row.fontScreenPx);
    const targetPx = Number.isFinite(fontScreenPx) && fontScreenPx > 0 ? fontScreenPx : DAG_LABEL_SCREEN_PX;
    const fontPx = isPort
      ? dagClampPortLabelFontPx(ctx, text, targetPx, maxW, maxH)
      : dagClampLabelFontPx(ctx, text, targetPx, maxW, maxH);
    ctx.font = `${fontPx}px ${DAG_LABEL_FONT_FAMILY}`;
    ctx.fillStyle = dagOverlayLabelFill(row.id, row.ghost === true, interaction.hoveredId, chrome, dimmedIds);
    ctx.globalAlpha = row.ghost ? 0.85 : dimmedIds.includes(row.id) ? 0.5 : 1;
    if (row.layout === "vertical") {
      ctx.save();
      ctx.translate(anchor.x, anchor.y);
      ctx.rotate(-Math.PI / 2);
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(text, 0, 0);
      ctx.restore();
    } else {
      const align = row.align === "left" || row.align === "right" ? row.align : "center";
      ctx.textAlign = align;
      ctx.textBaseline = "middle";
      ctx.fillText(text, anchor.x, anchor.y);
    }
    ctx.globalAlpha = 1;
  }
}

/** @emoji 📋 Parses a JSON node id array from DAG WASM. */
export function parseDagNodeIdArray(json: string): string[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((value): value is string => typeof value === "string") : [];
  } catch {
    return [];
  }
}

/** @emoji 🎯 Parses area-select preview snapshot JSON. */
export function parseDagPreselectJson(json: string): DagPreselectSnapshot {
  try {
    const parsed = JSON.parse(json) as { ids?: unknown; removedIds?: unknown };
    const ids = Array.isArray(parsed.ids) ? parsed.ids.filter((value): value is string => typeof value === "string") : [];
    const removedIds = Array.isArray(parsed.removedIds) ? parsed.removedIds.filter((value): value is string => typeof value === "string") : [];
    return { ids, removedIds };
  } catch {
    return { ids: [], removedIds: [] };
  }
}

/** @emoji 📍 Parses area-select preview point list JSON. */
export function parseDagSelectionPreviewPoints(json: string): readonly { readonly x: number; readonly y: number }[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed
      .map((entry) => {
        if (!Array.isArray(entry) || entry.length < 2) return null;
        const x = Number(entry[0]);
        const y = Number(entry[1]);
        return Number.isFinite(x) && Number.isFinite(y) ? { x, y } : null;
      })
      .filter((entry): entry is { x: number; y: number } => entry != null);
  } catch {
    return [];
  }
}

/** @emoji 📦 Parses screen-space selection union bounds JSON. */
export function parseDagSelectionUnionBoundsScreen(json: string): DagSelectionUnionBoundsScreen | null {
  if (json.trim() === "null") return null;
  try {
    const parsed = JSON.parse(json) as { x?: unknown; y?: unknown; width?: unknown; height?: unknown };
    const x = Number(parsed.x);
    const y = Number(parsed.y);
    const width = Number(parsed.width);
    const height = Number(parsed.height);
    if (![x, y, width, height].every(Number.isFinite)) return null;
    if (width <= 0 || height <= 0) return null;
    return { x, y, width, height };
  } catch {
    return null;
  }
}

/** @emoji 📏 Compares selection union bounds for overlay state updates. */
export function dagSelectionUnionBoundsEqual(
  left: DagSelectionUnionBoundsScreen | null,
  right: DagSelectionUnionBoundsScreen | null,
): boolean {
  if (left === right) return true;
  if (!left || !right) return false;
  return left.x === right.x && left.y === right.y && left.width === right.width && left.height === right.height;
}

/** @emoji 📐 Computes marquee overlay state from preview points. */
export function computeDagMarqueeOverlay(
  points: readonly { readonly x: number; readonly y: number }[],
  crossing: boolean,
  method: DagSelectionMethod,
): DagMarqueeOverlayState | null {
  if (points.length < 2) return null;
  const coverage = crossing ? "partial" : "full";
  if (method === "lasso" && points.length >= 3) {
    return { coverage, shape: "polygon", points };
  }
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const minX = Math.min(...xs);
  const minY = Math.min(...ys);
  const maxX = Math.max(...xs);
  const maxY = Math.max(...ys);
  return { coverage, shape: "rect", rect: { x: minX, y: minY, width: maxX - minX, height: maxY - minY } };
}

/** @emoji 📐 Selection union bounding rectangle chrome. */
export function DagSelectionBoundsBox({
  rect,
}: {
  readonly rect: DagSelectionUnionBoundsScreen;
}): React.JSX.Element {
  return (
    <div
      className="pointer-events-none absolute border border-emphasized"
      style={{ left: rect.x, top: rect.y, width: rect.width, height: rect.height }}
    />
  );
}
// #endregion 🔖LabelOverlay

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("dag lod", () => {
    it("maps automatic and pinned modes to canvas props", () => {
      expect(dagLodCanvasProps("automatic")).toEqual({ automaticLod: true });
      expect(dagLodCanvasProps("detail")).toEqual({ automaticLod: false, lod: "detail" });
    });

    it("exposes wasm lod tiers", () => {
      const tiers = getDagLodScale().map((row) => row.id);
      expect(tiers).toContain("normal");
      expect(tiers).toContain("micro");
    });
  });

  describe("dag fixture", () => {
    it("default fixture has five nodes and four edges", () => {
      expect(DAG_DEFAULT_FIXTURE.nodes.length).toBe(5);
      expect(DAG_DEFAULT_FIXTURE.edges.length).toBe(4);
    });

    it("default fixture includes all node kinds", () => {
      const kinds = DAG_DEFAULT_FIXTURE.nodes.map((n) => n.kind);
      expect(kinds).toContain("slider");
      expect(kinds).toContain("select");
      expect(kinds).toContain("computation");
      expect(kinds).toContain("screen");
    });
  });

  describe("dag overlay", () => {
    it("parses node id arrays", () => {
      expect(parseDagNodeIdArray('["a","b"]')).toEqual(["a", "b"]);
      expect(parseDagNodeIdArray("invalid")).toEqual([]);
    });

    it("parses preselect snapshot", () => {
      const snap = parseDagPreselectJson(JSON.stringify({ ids: ["a"], removedIds: ["b"] }));
      expect(snap.ids).toEqual(["a"]);
      expect(snap.removedIds).toEqual(["b"]);
    });

    it("compares selection union bounds", () => {
      const rect = { x: 1, y: 2, width: 10, height: 20 };
      expect(dagSelectionUnionBoundsEqual(rect, { ...rect })).toBe(true);
      expect(dagSelectionUnionBoundsEqual(rect, { ...rect, x: 2 })).toBe(false);
      expect(dagSelectionUnionBoundsEqual(null, null)).toBe(true);
    });

    it("computes marquee overlay from preview points", () => {
      const points = parseDagSelectionPreviewPoints(JSON.stringify([[0, 1], [3, 4]]));
      const overlay = computeDagMarqueeOverlay(points, false, "rectangle");
      expect(overlay?.shape).toBe("rect");
      if (overlay?.shape === "rect") {
        expect(overlay.rect).toEqual({ x: 0, y: 1, width: 3, height: 3 });
      }
    });

    it("derives interaction chrome from preselect", () => {
      const chrome = dagElementInteractionChrome(["a"], { ids: ["b"], removedIds: ["c"] });
      expect([...chrome.selectedIds]).toEqual(["b"]);
      expect([...chrome.highlightedIds]).toEqual(["c"]);
    });
  });
}
// #endregion 🧪Tests
