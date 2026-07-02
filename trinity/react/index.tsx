// #region 🧲Header
/** @emoji 🔺 `@semio-tech/trinity-react` — WASM trinity graph renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef } from "react";
import {
  UI_INSPECTOR_MIXED_PLACEHOLDER,
  uiDeclarativeSectionsToTree,
  uiInspectorGroupsToTree,
  uiInspectorMixedText,
  uiInspectorReadonlyField,
} from "@semio-tech/framework-playground-core";
import { syncSessionVelloTheme } from "@semio-tech/ui-styling";
import { useVelloThemeSync } from "@semio-tech/ui-react";
import initTrinityWasm, { TrinitySession, initSync, ruleQueryJson } from "../rewrite/engine/pkg/trinity_rewrite.js";
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
export interface TrinityPort {
  readonly id: string;
  readonly kind: string;
  readonly direction: "in" | "out";
}

export interface TrinityNode {
  readonly id: string;
  readonly kind: string;
  readonly name: string;
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly properties?: Record<string, unknown>;
  readonly ports?: readonly TrinityPort[];
}

export interface TrinityFixture {
  readonly schema: "trinity.graph";
  readonly name: string;
  readonly manifestId?: string;
  readonly manifest?: Record<string, unknown>;
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly rootNodeId?: string;
  readonly nodes: readonly TrinityNode[];
  readonly edges: readonly { readonly id: string; readonly kind: string; readonly source: string; readonly target: string; readonly properties?: Record<string, unknown> }[];
}

export const TRINITY_DEFAULT_FIXTURE: TrinityFixture = nakaginFixtureJson as TrinityFixture;
export const TRINITY_DEFAULT_FIXTURE_JSON = JSON.stringify(TRINITY_DEFAULT_FIXTURE);

export function trinityFixtureToJson(fixture: TrinityFixture): string {
  return JSON.stringify(fixture);
}

export function parseTrinityFixtureJson(json: string): TrinityFixture | null {
  try {
    const parsed = JSON.parse(json) as TrinityFixture;
    if (parsed.schema !== "trinity.graph" || !Array.isArray(parsed.nodes) || !Array.isArray(parsed.edges)) return null;
    return parsed;
  } catch {
    return null;
  }
}

export interface TrinityReorganizeRequest {
  readonly epoch: number;
  readonly optionsJson: string;
}

export type TrinityVcsCommandKind = "undo" | "redo" | "commitCheckpoint";

export interface TrinityVcsRequest {
  readonly kind: TrinityVcsCommandKind;
  readonly epoch: number;
  readonly message?: string;
}

export interface TrinityJackDispatchRequest {
  readonly query: string;
  readonly epoch: number;
}

export type TrinityJackResultKind = "table" | "graph";

export interface TrinityJackRun {
  readonly kind: TrinityJackResultKind;
  readonly columns: readonly string[];
  readonly rows: readonly (readonly unknown[])[];
  readonly graphFixture?: TrinityFixture;
  readonly fixtureJson: string;
}

export interface TrinityJackToken {
  readonly class: "keyword" | "ident" | "number" | "string" | "operator" | "punctuation" | "error";
  readonly start: number;
  readonly end: number;
}

export interface TrinityJackCompletion {
  readonly label: string;
  readonly kind: string;
  readonly detail?: string;
  readonly insert: string;
}

export type RuleParameterKind = "string" | "number" | "boolean";

export interface RuleParameter {
  readonly name: string;
  readonly kind: RuleParameterKind;
  readonly default: string | number | boolean | null;
}

export function runJackOnFixture(fixtureJson: string, query: string): TrinityJackRun {
  const session = new TrinitySession();
  session.loadFixtureJson(fixtureJson);
  return JSON.parse(session.runJackJsonWithFixture(query)) as TrinityJackRun;
}

export function tokenizeJackOnFixture(fixtureJson: string, source: string): readonly TrinityJackToken[] {
  const session = new TrinitySession();
  session.loadFixtureJson(fixtureJson);
  return JSON.parse(session.tokenizeJackJson(source)) as readonly TrinityJackToken[];
}

export function completeJackOnFixture(fixtureJson: string, source: string, cursor: number): readonly TrinityJackCompletion[] {
  const session = new TrinitySession();
  session.loadFixtureJson(fixtureJson);
  return JSON.parse(session.completeJackJson(source, cursor)) as readonly TrinityJackCompletion[];
}

export function createJackLspWorker(fixtureJson?: string): Worker {
  const worker = new Worker(new URL("../jack/lsp/worker.ts", import.meta.url), { type: "module" });
  worker.postMessage({ op: "init", fixtureJson });
  return worker;
}

export function applyRewriteOnFixture(fixtureJson: string, ruleJson: string, bindingsJson = "{}"): string {
  const session = new TrinitySession();
  session.loadFixtureJson(fixtureJson);
  return session.applyRewriteJson(ruleJson, bindingsJson);
}

export function ruleQueryOnFixture(ruleJson: string, bindingsJson = "{}"): string {
  const parsed = JSON.parse(ruleQueryJson(ruleJson, bindingsJson)) as { query?: string };
  return parsed.query ?? "";
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

export function buildTrinityPlayInspectorTree(
  fixtureJson: string,
  selectedNodeIds: readonly string[],
  controllerId: string,
): import("@semio-tech/framework-playground-core").UiNode {
  const fixture = parseTrinityFixtureJson(fixtureJson);
  if (!fixture || selectedNodeIds.length === 0) {
    return uiDeclarativeSectionsToTree([
      {
        type: "section",
        id: "trinity-inspector.empty",
        label: "Inspection",
        children: [{ type: "text", value: "Select one or more pieces" }],
      },
    ]);
  }
  const nodes = selectedNodeIds
    .map((id) => fixture.nodes.find((row) => row.id === id))
    .filter((node): node is TrinityNode => Boolean(node));
  if (!nodes.length) {
    return uiDeclarativeSectionsToTree([
      {
        type: "section",
        id: "trinity-inspector.missing",
        label: "Inspection",
        children: [{ type: "text", value: "Piece not found" }],
      },
    ]);
  }
  const nodeIds = nodes.map((node) => node.id);
  const nameMixed = uiInspectorMixedText(nodes.map((node) => node.name));
  const kindMixed = uiInspectorMixedText(nodes.map((node) => node.kind));
  const flatLabels = nodes.map((node) => {
    const flat = node.properties?.flatPosition as { u?: number; v?: number } | undefined;
    return flat ? `u=${flat.u ?? 0}, v=${flat.v ?? 0}` : "(derived)";
  });
  const flatMixed = uiInspectorMixedText(flatLabels);
  const portCounts = nodes.map((node) => String(node.ports?.length ?? 0));
  const portsMixed = uiInspectorMixedText(portCounts);
  const trinityPlayCmd = (command: string, args?: Record<string, unknown>) => ({ controllerId, command, args });
  return uiInspectorGroupsToTree([
    {
      id: "trinity-inspector.geometry",
      label: "Geometry",
      fields: [
        uiInspectorReadonlyField(
          "trinity-inspector.flat",
          "Flat position",
          flatMixed.uniform ? (flatLabels[0] ?? "") : flatMixed.placeholder ?? UI_INSPECTOR_MIXED_PLACEHOLDER,
        ),
        uiInspectorReadonlyField(
          "trinity-inspector.ports",
          "Connectors",
          portsMixed.uniform ? (portCounts[0] ?? "") : portsMixed.placeholder ?? UI_INSPECTOR_MIXED_PLACEHOLDER,
        ),
      ],
    },
    {
      id: "trinity-inspector.identity",
      label: "Identity",
      fields: [
        {
          type: "field",
          id: "trinity-inspector.name",
          label: "Name",
          child: {
            type: "input",
            id: "trinity-inspector.name.input",
            inputKind: "text",
            value: nameMixed.value,
            placeholder: nameMixed.placeholder,
            onChange: trinityPlayCmd("patchTrinityNodes", { nodeIds, field: "name" }),
          },
        },
        uiInspectorReadonlyField(
          "trinity-inspector.kind",
          "Kind",
          kindMixed.uniform ? (nodes[0]?.kind ?? "") : kindMixed.placeholder ?? UI_INSPECTOR_MIXED_PLACEHOLDER,
        ),
        uiInspectorReadonlyField(
          "trinity-inspector.id",
          "Id",
          nodeIds.length === 1 ? (nodeIds[0] ?? "") : `${nodeIds.length} selected`,
        ),
      ],
    },
  ]);
}
// #endregion 🔖Panels

// #region 🔖Lod
export type TrinityDrawLodKind = "minimap" | "overview" | "compact" | "normal" | "detail" | "micro";

export interface TrinityLodRow {
  readonly id: TrinityDrawLodKind;
  readonly name: string;
  readonly description: string;
  readonly maxZoom: number;
}

const TRINITY_DRAW_LOD_KINDS = new Set<TrinityDrawLodKind>(["minimap", "overview", "compact", "normal", "detail", "micro"]);

export function isTrinityDrawLodKind(value: string): value is TrinityDrawLodKind {
  return TRINITY_DRAW_LOD_KINDS.has(value as TrinityDrawLodKind);
}

export const TRINITY_LOD_MODE_AUTOMATIC = "automatic" as const;

export type TrinityLodModeKind = typeof TRINITY_LOD_MODE_AUTOMATIC | TrinityDrawLodKind;

let trinityLodScaleCache: readonly TrinityLodRow[] | null = null;

export function getTrinityLodScale(): readonly TrinityLodRow[] {
  if (!trinityLodScaleCache) {
    const session = new TrinitySession();
    trinityLodScaleCache = JSON.parse(session.lodScaleJson()) as TrinityLodRow[];
  }
  return trinityLodScaleCache;
}

export function trinityPlayLodTiers(): readonly TrinityDrawLodKind[] {
  return getTrinityLodScale().map((lod) => lod.id);
}

export function trinityLodCanvasProps(mode: TrinityLodModeKind): { automaticLod: boolean; lod?: TrinityDrawLodKind } {
  if (mode === TRINITY_LOD_MODE_AUTOMATIC) {
    return { automaticLod: true };
  }
  return { automaticLod: false, lod: mode };
}

export function trinityLodAutomaticSelectLabel(effectiveTier: TrinityDrawLodKind): string {
  const row = getTrinityLodScale().find((lod) => lod.id === effectiveTier);
  const name = row?.name ?? effectiveTier;
  return `Automatic · ${name}`;
}

export function trinityPlayLodTierMenuLabel(tier: TrinityDrawLodKind): string {
  const row = getTrinityLodScale().find((lod) => lod.id === tier);
  return row?.name ?? tier.charAt(0).toUpperCase() + tier.slice(1);
}
// #endregion 🔖Lod

// #region 🔖TrinityCanvas
export interface TrinityCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
  readonly reorganize?: TrinityReorganizeRequest;
  readonly jackDispatch?: TrinityJackDispatchRequest;
  readonly vcsRequest?: TrinityVcsRequest;
  readonly onFixtureChange?: (fixtureJson: string) => void;
  readonly onJackDispatchComplete?: (resultJson: string) => void;
  readonly onVcsApplied?: (generation: number) => void;
  readonly onSelectionChange?: (nodeIds: readonly string[]) => void;
  readonly highlightedNodeIds?: readonly string[];
  readonly highlightedNodeIdsSignal?: number;
  readonly automaticLod?: boolean;
  readonly lod?: TrinityDrawLodKind;
  readonly onLodChange?: (lod: TrinityDrawLodKind) => void;
}

function waitForLayoutSize(container: HTMLElement, min = 8): Promise<void> {
  return new Promise((resolve) => {
    let attempts = 0;
    const probe = () => {
      const rect = container.getBoundingClientRect();
      if (rect.width >= min && rect.height >= min) {
        resolve();
        return;
      }
      attempts += 1;
      if (attempts > 120) {
        resolve();
        return;
      }
      requestAnimationFrame(probe);
    };
    probe();
  });
}

export function TrinityCanvas({
  fixtureJson,
  className,
  reorganize,
  jackDispatch,
  vcsRequest,
  onFixtureChange,
  onJackDispatchComplete,
  onVcsApplied,
  onSelectionChange,
  highlightedNodeIds,
  highlightedNodeIdsSignal = 0,
  automaticLod = true,
  lod,
  onLodChange,
}: TrinityCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<TrinitySession | null>(null);
  const rafRef = useRef<number | null>(null);
  const onFixtureChangeRef = useRef(onFixtureChange);
  const onJackDispatchCompleteRef = useRef(onJackDispatchComplete);
  const onVcsAppliedRef = useRef(onVcsApplied);
  const onSelectionChangeRef = useRef(onSelectionChange);
  const onLodChangeRef = useRef(onLodChange);
  const lastAutomaticLodRef = useRef<boolean | null>(null);
  const lastForcedLodRef = useRef<string | null>(null);
  const lastReportedLodRef = useRef<TrinityDrawLodKind | null>(null);
  const lastHighlightedJsonRef = useRef<string>("[]");

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
    const forced = nextAutomatic ? "" : lod && isTrinityDrawLodKind(lod) ? lod : "";
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
      if (!isTrinityDrawLodKind(label)) return;
      if (lastReportedLodRef.current === label) return;
      lastReportedLodRef.current = label;
      onLodChangeRef.current(label);
    } catch {
      /* session not ready */
    }
  }, []);

  const reportSelection = useCallback(() => {
    const session = sessionRef.current;
    if (!session || !onSelectionChangeRef.current) return;
    try {
      const ids = JSON.parse(session.selectedNodeIdsJson()) as string[];
      onSelectionChangeRef.current(ids);
    } catch {
      /* selection not ready */
    }
  }, []);

  const renderFrame = useCallback(() => {
    try {
      if (!sessionRef.current?.gpuReady()) return;
      syncLodMode();
      syncVelloTheme();
      sessionRef.current?.renderFrame();
      reportDrawLod();
    } catch {
      /* gpu not ready */
    }
  }, [reportDrawLod, syncLodMode, syncVelloTheme]);

  useEffect(() => {
    onFixtureChangeRef.current = onFixtureChange;
  }, [onFixtureChange]);

  useEffect(() => {
    onJackDispatchCompleteRef.current = onJackDispatchComplete;
  }, [onJackDispatchComplete]);

  useEffect(() => {
    onVcsAppliedRef.current = onVcsApplied;
  }, [onVcsApplied]);

  useEffect(() => {
    onSelectionChangeRef.current = onSelectionChange;
  }, [onSelectionChange]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session?.gpuReady()) return;
    const nextJson = JSON.stringify(highlightedNodeIds ?? []);
    if (lastHighlightedJsonRef.current === nextJson) return;
    lastHighlightedJsonRef.current = nextJson;
    try {
      session.setHighlightedNodeIdsJson(nextJson);
      renderFrame();
    } catch {
      /* highlight not ready */
    }
  }, [highlightedNodeIdsSignal, highlightedNodeIds, renderFrame]);

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
      onFixtureChangeRef.current?.(session.fixtureJson());
      renderFrame();
    } catch {
      /* reorganize not ready */
    }
  }, [reorganize?.epoch, reorganize?.optionsJson, renderFrame]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !jackDispatch || jackDispatch.epoch <= 0) return;
    try {
      const resultJson = session.runJackJsonWithFixture(jackDispatch.query);
      onFixtureChangeRef.current?.(session.fixtureJson());
      onJackDispatchCompleteRef.current?.(resultJson);
      renderFrame();
    } catch (err) {
      try {
        onJackDispatchCompleteRef.current?.(
          JSON.stringify({
            kind: "table",
            columns: ["error"],
            rows: [[String(err)]],
            fixtureJson: session.fixtureJson(),
          }),
        );
      } catch {
        /* session not ready */
      }
    }
  }, [jackDispatch?.epoch, jackDispatch?.query, renderFrame]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !vcsRequest || vcsRequest.epoch <= 0) return;
    try {
      if (vcsRequest.kind === "undo") {
        session.undo();
      } else if (vcsRequest.kind === "redo") {
        session.redo();
      } else {
        session.commitCheckpoint(vcsRequest.message ?? "");
      }
      onFixtureChangeRef.current?.(session.fixtureJson());
      onVcsAppliedRef.current?.(session.storeGeneration());
      renderFrame();
    } catch {
      /* vcs not ready */
    }
  }, [vcsRequest?.epoch, vcsRequest?.kind, vcsRequest?.message, renderFrame]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session) return;
    const nextFixture = fixtureJson ?? TRINITY_DEFAULT_FIXTURE_JSON;
    try {
      if (session.fixtureJson() !== nextFixture) {
        session.loadFixtureJson(nextFixture);
        renderFrame();
      }
    } catch {
      /* fixture not ready */
    }
  }, [fixtureJson, renderFrame]);

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    let cancelled = false;
    let cleanupInner: (() => void) | undefined;
    const session = new TrinitySession();
    sessionRef.current = session;
    session.loadFixtureJson(fixtureJson ?? TRINITY_DEFAULT_FIXTURE_JSON);

    const resize = () => {
      if (cancelled) return;
      const r = container.getBoundingClientRect();
      const nextDpr = globalThis.devicePixelRatio || 1;
      const w = Math.max(8, Math.round(r.width));
      const h = Math.max(8, Math.round(r.height));
      const pw = Math.max(1, Math.round(w * nextDpr));
      const ph = Math.max(1, Math.round(h * nextDpr));
      if (canvas.width !== pw || canvas.height !== ph) {
        canvas.width = pw;
        canvas.height = ph;
      }
      canvas.style.width = `${w}px`;
      canvas.style.height = `${h}px`;
      session.setSize(w, h, nextDpr);
      renderFrame();
    };

    void (async () => {
      await new Promise<void>((resolve) => {
        requestAnimationFrame(() => requestAnimationFrame(() => resolve()));
      });
      if (cancelled) return;
      await waitForLayoutSize(container);
      if (cancelled) return;
      resize();
      const rect = container.getBoundingClientRect();
      const dpr = globalThis.devicePixelRatio || 1;
      const initW = Math.max(8, Math.round(rect.width));
      const initH = Math.max(8, Math.round(rect.height));
      try {
        await session.attachCanvas(canvas, initW, initH, dpr);
      } catch {
        return;
      }
      if (cancelled) {
        session.detachGpu();
        return;
      }
      resize();
      const ro = new ResizeObserver(resize);
      ro.observe(container);
      const tick = () => {
        if (cancelled) return;
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
          reportSelection();
        } catch {
          /* fixture not ready */
        }
        renderFrame();
      };
      const onWheel = (ev: WheelEvent) => {
        ev.preventDefault();
        const r = canvas.getBoundingClientRect();
        session.wheelScreen(ev.clientX - r.left, ev.clientY - r.top, ev.deltaY);
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
      canvas.addEventListener("wheel", onWheel, { passive: false });
      cleanupInner = () => {
        ro.disconnect();
        canvas.removeEventListener("pointerdown", onPointerDown);
        canvas.removeEventListener("pointermove", onPointerMove);
        canvas.removeEventListener("pointerup", onPointerUp);
        canvas.removeEventListener("pointerleave", onPointerUp);
        canvas.removeEventListener("wheel", onWheel);
        if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      };
    })();

    return () => {
      cancelled = true;
      cleanupInner?.();
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      session.detachGpu();
      sessionRef.current = null;
    };
  }, [renderFrame, reportSelection]);

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
    it("default nakagin fixture parses expanded graph", () => {
      const fixture = parseTrinityFixtureJson(TRINITY_DEFAULT_FIXTURE_JSON);
      expect(fixture?.schema).toBe("trinity.graph");
      expect(fixture?.nodes.length).toBe(9);
      expect(fixture?.edges.length).toBe(6);
      const root = fixture?.nodes.find((node) => node.id === fixture?.rootNodeId);
      expect(root?.name).toBe("b");
      expect(root?.properties?.label).toBe("tower-core");
    });

    it("buildTrinityPlayHierarchyTree lists nodes", () => {
      const tree = buildTrinityPlayHierarchyTree(TRINITY_DEFAULT_FIXTURE_JSON, []);
      expect(tree.type).toBe("tree");
    });

    it("buildTrinityPlayInspectorTree exposes batch name field", () => {
      const fixture = parseTrinityFixtureJson(TRINITY_DEFAULT_FIXTURE_JSON);
      expect(fixture).not.toBeNull();
      const nodeIds = fixture!.nodes.slice(0, 2).map((node) => node.id);
      const tree = buildTrinityPlayInspectorTree(TRINITY_DEFAULT_FIXTURE_JSON, nodeIds, "trinity-jack-play");
      expect(tree.type).toBe("tree");
      const identitySection = tree.sections.find((section) => section.id === "trinity-inspector.identity");
      const nameField = identitySection?.items.find((item) => item.id === "trinity-inspector.name");
      expect(nameField?.control?.onChange?.command).toBe("patchTrinityNodes");
      expect(nameField?.control?.onChange?.args).toMatchObject({ nodeIds, field: "name" });
    });

    it("runJackOnFixture returns nakagin core name", () => {
      const result = runJackOnFixture(TRINITY_DEFAULT_FIXTURE_JSON, "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name");
      expect(result.kind).toBe("table");
      expect(result.rows.length).toBe(1);
      expect(result.fixtureJson.length).toBeGreaterThan(0);
    });

    it("getTrinityLodScale exposes six bands", () => {
      expect(getTrinityLodScale().length).toBe(6);
    });

    it("tokenizeJackOnFixture highlights keywords", () => {
      const tokens = tokenizeJackOnFixture(TRINITY_DEFAULT_FIXTURE_JSON, "MATCH (a:Piece)");
      expect(tokens.some((row) => row.class === "keyword" && row.start === 0)).toBe(true);
    });

    it("completeJackOnFixture suggests MATCH", () => {
      const items = completeJackOnFixture(TRINITY_DEFAULT_FIXTURE_JSON, "MAT", 3);
      expect(items.some((row) => row.label === "MATCH")).toBe(true);
    });

    it("ruleQueryOnFixture builds MATCH query from rule", () => {
      const rule = JSON.stringify({
        name: "label-core",
        lhs: { pattern: { leftVar: "a", leftKind: "Piece" }, whereClause: "a.name = 'b'" },
        rhs: {
          create: [],
          delete: [],
          set: [{ var: "a", prop: "label", value: "$label" }],
          merge: [],
          parameters: [{ name: "label", kind: "string", default: "nakagin-core" }],
        },
      });
      const query = ruleQueryOnFixture(rule, JSON.stringify({ label: "override-core" }));
      expect(query).toContain("MATCH (a:Piece)");
      expect(query).toContain("SET a.label = 'override-core'");
    });
  });
}
// #endregion 🧪Tests
