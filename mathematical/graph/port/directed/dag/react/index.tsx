// #region 🧲Header
/** @emoji 🌳 `@dag/react` — WASM DAG renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef } from "react";
import { serializeGraphVelloThemePaletteJson } from "@ui/styling";
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
export interface DagPortV1 {
  readonly id: string;
  readonly label: string;
}

export interface DagMediaV1 {
  readonly kind: "image" | "svg" | "pdf" | "video";
  readonly src: string;
}

interface DagNodeBaseV1 {
  readonly id: string;
  readonly name: string;
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
}

export interface DagComputationNodeV1 extends DagNodeBaseV1 {
  readonly kind: "computation";
  readonly inputs: readonly DagPortV1[];
  readonly outputs: readonly DagPortV1[];
}

export interface DagSliderNodeV1 extends DagNodeBaseV1 {
  readonly kind: "slider";
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly value: number;
  readonly output: DagPortV1;
}

export interface DagSelectNodeV1 extends DagNodeBaseV1 {
  readonly kind: "select";
  readonly options: readonly string[];
  readonly selected: number;
  readonly output: DagPortV1;
}

export interface DagScreenNodeV1 extends DagNodeBaseV1 {
  readonly kind: "screen";
  readonly media?: DagMediaV1;
  readonly input: DagPortV1;
}

export type DagNodeV1 = DagComputationNodeV1 | DagSliderNodeV1 | DagSelectNodeV1 | DagScreenNodeV1;

export interface DagFixtureV1 {
  readonly schema: "dag.fixture/v1";
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly nodes: readonly DagNodeV1[];
  readonly edges: readonly { readonly id: string; readonly source: string; readonly target: string }[];
}

const DAG_DEMO_SCREEN_SVG =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E";

export const DAG_DEFAULT_FIXTURE: DagFixtureV1 = {
  schema: "dag.fixture/v1",
  camera: { x: 0, y: 0, zoom: 1 },
  nodes: [
    { id: "slider", name: "Amount", kind: "slider", x: -400, y: -40, width: 180, height: 80, min: 0, max: 10, step: 0.5, value: 5, output: { id: "out", label: "value" } },
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

export function dagFixtureToJson(fixture: DagFixtureV1): string {
  return JSON.stringify(fixture);
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
    if (typeof document === "undefined") return;
    const session = sessionRef.current;
    if (!session) return;
    try {
      session.setVelloThemeJson(serializeGraphVelloThemePaletteJson());
    } catch {
      /* document theme tokens not ready */
    }
  }, []);

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
}
// #endregion 🧪Tests
