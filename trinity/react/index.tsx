// #region 🧲Header
/** @emoji 🔺 `@semio-tech/trinity-react` — WASM trinity graph renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef } from "react";
import { clearColorResolveCache, serializeGraphVelloThemePaletteJson } from "@semio-tech/ui-styling";
import initTrinityWasm, { TrinitySession, initSync } from "../rewrite/engine/pkg/trinity_rewrite.js";
import nakaginFixtureJson from "../fixture/nakagin-capsule-tower.trinity.json";

// #region 🔖GpuWasmBridge
if (import.meta.env.VITEST) {
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../rewrite/engine/pkg/trinity_rewrite_bg.wasm");
  initSync({ module: readFileSync(wasmPath) });
} else {
  await initTrinityWasm();
}

export async function ensureTrinityWasmLoaded(): Promise<void> {
  await initTrinityWasm();
}

export { TrinitySession };
// #endregion 🔖GpuWasmBridge

// #region 🔖Fixture
export interface TrinityPortV1 {
  readonly id: string;
  readonly kind: string;
  readonly direction: "in" | "out";
}

export interface TrinityNodeV1 {
  readonly id: string;
  readonly kind: string;
  readonly name: string;
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly properties?: Record<string, unknown>;
  readonly ports?: readonly TrinityPortV1[];
}

export interface TrinityFixtureV1 {
  readonly schema: "trinity.graph/v1";
  readonly name: string;
  readonly manifest: Record<string, unknown>;
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly rootNodeId?: string;
  readonly nodes: readonly TrinityNodeV1[];
  readonly edges: readonly { readonly id: string; readonly kind: string; readonly source: string; readonly target: string; readonly properties?: Record<string, unknown> }[];
}

export const TRINITY_DEFAULT_FIXTURE: TrinityFixtureV1 = nakaginFixtureJson as TrinityFixtureV1;
export const TRINITY_DEFAULT_FIXTURE_JSON = JSON.stringify(TRINITY_DEFAULT_FIXTURE);

export function trinityFixtureToJson(fixture: TrinityFixtureV1): string {
  return JSON.stringify(fixture);
}

export function parseTrinityFixtureJson(json: string): TrinityFixtureV1 | null {
  try {
    const parsed = JSON.parse(json) as TrinityFixtureV1;
    if (parsed.schema !== "trinity.graph/v1" || !Array.isArray(parsed.nodes) || !Array.isArray(parsed.edges)) return null;
    return parsed;
  } catch {
    return null;
  }
}

export interface TrinityReorganizeRequest {
  readonly epoch: number;
  readonly optionsJson: string;
}

export interface TrinityJackResultV1 {
  readonly columns: readonly string[];
  readonly rows: readonly (readonly unknown[])[];
}

export interface TrinityJackTokenV1 {
  readonly class: "keyword" | "ident" | "number" | "string" | "operator" | "punctuation" | "error";
  readonly start: number;
  readonly end: number;
}

export interface TrinityJackCompletionV1 {
  readonly label: string;
  readonly kind: string;
  readonly detail?: string;
  readonly insert: string;
}

export function runJackOnFixture(fixtureJson: string, query: string): TrinityJackResultV1 {
  const session = new TrinitySession();
  session.loadFixtureJson(fixtureJson);
  return JSON.parse(session.runJackJson(query)) as TrinityJackResultV1;
}

export function tokenizeJackOnFixture(fixtureJson: string, source: string): readonly TrinityJackTokenV1[] {
  const session = new TrinitySession();
  session.loadFixtureJson(fixtureJson);
  return JSON.parse(session.tokenizeJackJson(source)) as readonly TrinityJackTokenV1[];
}

export function completeJackOnFixture(fixtureJson: string, source: string, cursor: number): readonly TrinityJackCompletionV1[] {
  const session = new TrinitySession();
  session.loadFixtureJson(fixtureJson);
  return JSON.parse(session.completeJackJson(source, cursor)) as readonly TrinityJackCompletionV1[];
}

export function applyRewriteOnFixture(fixtureJson: string, ruleJson: string): string {
  const session = new TrinitySession();
  session.loadFixtureJson(fixtureJson);
  return session.applyRewriteJson(ruleJson);
}
// #endregion 🔖Fixture

// #region 🔖Panels
export function buildTrinityPlayHierarchyTree(fixtureJson: string, selectedNodeIds: readonly string[]): import("@semio-tech/framework-playground-core").UiNode {
  const fixture = parseTrinityFixtureJson(fixtureJson);
  if (!fixture) {
    return { type: "tree", sections: [{ id: "trinity-hierarchy.invalid", label: "Hierarchy", defaultOpen: true, items: [{ id: "trinity-hierarchy.invalid.msg", label: "Invalid trinity fixture" }] }] };
  }
  const nodeItems = fixture.nodes.map((node) => ({
    id: `trinity-hierarchy.node.${node.id}`,
    label: node.name || node.id,
    description: node.kind,
  }));
  const edgeItems = fixture.edges.map((edge) => ({
    id: `trinity-hierarchy.edge.${edge.id}`,
    label: `${edge.source} → ${edge.target}`,
    description: edge.kind,
  }));
  return {
    type: "tree",
    sections: [
      { id: "trinity-hierarchy.nodes", label: "Pieces", defaultOpen: true, items: nodeItems },
      { id: "trinity-hierarchy.edges", label: "Connections", defaultOpen: false, items: edgeItems },
    ],
    selectedIds: selectedNodeIds.map((id) => `trinity-hierarchy.node.${id}`),
  };
}

export function buildTrinityPlayCatalogueTree(): import("@semio-tech/framework-playground-core").UiNode {
  return {
    type: "tree",
    sections: [
      {
        id: "trinity-catalogue.kinds",
        label: "Catalogue",
        defaultOpen: true,
        items: [
          { id: "trinity-catalogue.piece", label: "Piece", description: "node" },
          { id: "trinity-catalogue.connection", label: "Connection", description: "edge" },
          { id: "trinity-catalogue.connector", label: "Connector", description: "port" },
        ],
      },
    ],
  };
}

export function buildTrinityPlayInspectorTree(fixtureJson: string, selectedNodeIds: readonly string[]): import("@semio-tech/framework-playground-core").UiNode {
  const fixture = parseTrinityFixtureJson(fixtureJson);
  if (!fixture || selectedNodeIds.length !== 1) {
    return { type: "tree", sections: [{ id: "trinity-inspector.empty", label: "Inspection", defaultOpen: true, items: [{ id: "trinity-inspector.empty.msg", label: "Select one piece" }] }] };
  }
  const node = fixture.nodes.find((row) => row.id === selectedNodeIds[0]);
  if (!node) {
    return { type: "tree", sections: [{ id: "trinity-inspector.missing", label: "Inspection", defaultOpen: true, items: [{ id: "trinity-inspector.missing.msg", label: "Piece not found" }] }] };
  }
  const flat = node.properties?.flatPosition as { u?: number; v?: number } | undefined;
  return {
    type: "tree",
    sections: [
      {
        id: "trinity-inspector.node",
        label: node.name,
        defaultOpen: true,
        items: [
          { id: "trinity-inspector.kind", label: "Kind", description: node.kind },
          { id: "trinity-inspector.flat", label: "Flat position", description: flat ? `u=${flat.u ?? 0}, v=${flat.v ?? 0}` : "(derived)" },
          { id: "trinity-inspector.ports", label: "Connectors", description: String(node.ports?.length ?? 0) },
        ],
      },
    ],
  };
}
// #endregion 🔖Panels

// #region 🔖TrinityCanvas
export interface TrinityCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
  readonly reorganize?: TrinityReorganizeRequest;
  readonly onFixtureChange?: (fixtureJson: string) => void;
}

export function TrinityCanvas({ fixtureJson, className, reorganize, onFixtureChange }: TrinityCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<TrinitySession | null>(null);
  const rafRef = useRef<number | null>(null);
  const onFixtureChangeRef = useRef(onFixtureChange);

  const syncVelloTheme = useCallback(() => {
    const session = sessionRef.current;
    if (!session || typeof document === "undefined") return;
    try {
      clearColorResolveCache();
      session.setVelloThemeJson(serializeGraphVelloThemePaletteJson());
    } catch {
      /* theme not ready */
    }
  }, []);

  const renderFrame = useCallback(() => {
    try {
      syncVelloTheme();
      sessionRef.current?.renderFrame();
    } catch {
      /* gpu not ready */
    }
  }, [syncVelloTheme]);

  useEffect(() => {
    onFixtureChangeRef.current = onFixtureChange;
  }, [onFixtureChange]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !reorganize || reorganize.epoch <= 0) return;
    try {
      session.reorganize(reorganize.optionsJson);
      onFixtureChangeRef.current?.(session.fixtureJson());
      renderFrame();
    } catch (err) {
      console.log(`[DEBUG] trinity canvas reorganize failed: ${String(err)}`);
    }
  }, [reorganize?.epoch, reorganize?.optionsJson, renderFrame]);

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    const session = new TrinitySession();
    sessionRef.current = session;
    session.loadFixtureJson(fixtureJson ?? TRINITY_DEFAULT_FIXTURE_JSON);
    console.log("[DEBUG] trinity canvas loaded fixture");
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
        const r = container.getBoundingClientRect();
        const nextDpr = globalThis.devicePixelRatio || 1;
        const w = Math.max(1, Math.round(r.width));
        const h = Math.max(1, Math.round(r.height));
        canvas.width = Math.round(w * nextDpr);
        canvas.height = Math.round(h * nextDpr);
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;
        session.setSize(w, h, nextDpr);
        renderFrame();
      };
      resize();
      const ro = new ResizeObserver(resize);
      ro.observe(container);
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
        session.pointerUp(ev.clientX - r.left, ev.clientY - r.top);
        try {
          onFixtureChangeRef.current?.(session.fixtureJson());
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
        canvas.removeEventListener("pointerdown", onPointerDown);
        canvas.removeEventListener("pointermove", onPointerMove);
        canvas.removeEventListener("pointerup", onPointerUp);
        canvas.removeEventListener("pointerleave", onPointerUp);
        if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      };
    });
    return () => {
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      sessionRef.current = null;
    };
  }, [fixtureJson, renderFrame]);

  return (
    <div ref={containerRef} className={className ?? "relative h-full w-full min-h-0 min-w-0 bg-canvas"}>
      <canvas ref={canvasRef} className="block h-full w-full touch-none" />
    </div>
  );
}
// #endregion 🔖TrinityCanvas

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("trinity fixture", () => {
    it("default nakagin fixture parses", () => {
      const fixture = parseTrinityFixtureJson(TRINITY_DEFAULT_FIXTURE_JSON);
      expect(fixture?.schema).toBe("trinity.graph/v1");
      expect(fixture?.nodes.length).toBeGreaterThan(0);
    });

    it("buildTrinityPlayHierarchyTree lists nodes", () => {
      const tree = buildTrinityPlayHierarchyTree(TRINITY_DEFAULT_FIXTURE_JSON, []);
      expect(tree.type).toBe("tree");
    });

    it("runJackOnFixture returns nakagin core name", () => {
      const result = runJackOnFixture(TRINITY_DEFAULT_FIXTURE_JSON, "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name");
      expect(result.rows.length).toBe(1);
    });

    it("tokenizeJackOnFixture highlights keywords", () => {
      const tokens = tokenizeJackOnFixture(TRINITY_DEFAULT_FIXTURE_JSON, "MATCH (a:Piece)");
      expect(tokens.some((row) => row.class === "keyword" && row.start === 0)).toBe(true);
    });

    it("completeJackOnFixture suggests MATCH", () => {
      const items = completeJackOnFixture(TRINITY_DEFAULT_FIXTURE_JSON, "MAT", 3);
      expect(items.some((row) => row.label === "MATCH")).toBe(true);
    });
  });
}
// #endregion 🧪Tests
