// #region 🧲Header
/** @emoji 🌊 `@flow/react` — WASM flow renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef, useState } from "react";
import { serializeGraphVelloThemePaletteJson } from "@ui/styling";
import initFlowWasm, { FlowSession, initSync } from "../core/pkg/flow_core.js";

// #region 🔖GpuWasmBridge
if (import.meta.env.VITEST) {
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../core/pkg/flow_core_bg.wasm");
  initSync({ module: readFileSync(wasmPath) });
} else {
  await initFlowWasm();
}

export async function ensureFlowWasmLoaded(): Promise<void> {
  await initFlowWasm();
}

export { FlowSession };
// #endregion 🔖GpuWasmBridge

// #region 🔖Fixture
export interface FlowFixtureV1 {
  readonly schema: "flow.fixture/v1";
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly widgets: readonly FlowWidgetV1[];
  readonly synapses: readonly { readonly id: string; readonly from: string; readonly to: string }[];
  readonly layout?: Readonly<Record<string, { readonly x: number; readonly y: number }>>;
}

export type FlowWidgetV1 =
  | { readonly kind: "neuron"; readonly id: string; readonly neuronKind: string }
  | { readonly kind: "inputSlider"; readonly id: string; readonly value: number }
  | { readonly kind: "inputNote"; readonly id: string; readonly text: string }
  | { readonly kind: "outputPreview"; readonly id: string }
  | { readonly kind: "outputAction"; readonly id: string; readonly action: string };

export const FLOW_DEFAULT_FIXTURE: FlowFixtureV1 = {
  schema: "flow.fixture/v1",
  camera: { x: 0, y: 0, zoom: 1 },
  widgets: [
    { kind: "inputSlider", id: "slider", value: 3 },
    { kind: "neuron", id: "add", neuronKind: "math.add" },
    { kind: "outputPreview", id: "preview" },
  ],
  synapses: [
    { id: "s1", from: "slider", to: "add" },
    { id: "s2", from: "add", to: "preview" },
  ],
};

export function flowFixtureToJson(fixture: FlowFixtureV1): string {
  return JSON.stringify(fixture);
}
// #endregion 🔖Fixture

// #region 🔖FlowStore
export interface FlowStore {
  load(): string | null;
  save(fixtureJson: string): void;
  clear(): void;
}

const FLOW_STORE_KEY = "flow.fixture/v1";

export function createLocalFlowStore(storage: Pick<Storage, "getItem" | "setItem" | "removeItem"> = globalThis.localStorage): FlowStore {
  return {
    load(): string | null {
      return storage.getItem(FLOW_STORE_KEY);
    },
    save(fixtureJson: string): void {
      storage.setItem(FLOW_STORE_KEY, fixtureJson);
    },
    clear(): void {
      storage.removeItem(FLOW_STORE_KEY);
    },
  };
}
// #endregion 🔖FlowStore

// #region 🔖Catalogue
export interface CatalogueItem {
  readonly kind: string;
  readonly neuronKind?: string;
  readonly action?: string;
  readonly name: string;
  readonly summary: string;
}

export interface CatalogueSection {
  readonly id: string;
  readonly title: string;
  readonly items: readonly CatalogueItem[];
}

export const FLOW_WIDGET_DRAG_V1_MIME = "application/x-flow-widget-v1";
export const FLOW_WIDGET_DRAG_PLAIN_MIME = "text/plain";

/** @emoji 📍 Flow widget palette drop: descriptor plus pointer in CSS space and world on the canvas. */
export interface FlowWidgetDropDetail {
  readonly descriptor: string;
  readonly screen: { readonly x: number; readonly y: number };
  readonly world: { readonly x: number; readonly y: number };
}

export function flowCatalogueItemDescriptor(item: CatalogueItem): string {
  if (item.kind === "neuron" && item.neuronKind) {
    return JSON.stringify({ kind: "neuron", neuronKind: item.neuronKind });
  }
  if (item.kind === "outputAction") {
    return JSON.stringify({ kind: "outputAction", action: item.action ?? "log" });
  }
  return JSON.stringify({ kind: item.kind });
}

export function encodeFlowWidgetDescriptorForDragV1(descriptorJson: string): string {
  return descriptorJson;
}

export function decodeFlowWidgetDescriptorFromDragV1(encoded: string): string | null {
  const trimmed = encoded.trim();
  if (!trimmed) return null;
  try {
    const parsed = JSON.parse(trimmed) as { kind?: string };
    return typeof parsed.kind === "string" ? trimmed : null;
  } catch {
    return null;
  }
}

export function readFlowWidgetDragDataTransfer(dataTransfer: DataTransfer): string | null {
  const custom = dataTransfer.getData(FLOW_WIDGET_DRAG_V1_MIME);
  if (custom?.trim()) {
    return decodeFlowWidgetDescriptorFromDragV1(custom);
  }
  const plain = dataTransfer.getData(FLOW_WIDGET_DRAG_PLAIN_MIME);
  if (plain?.trim()) {
    return decodeFlowWidgetDescriptorFromDragV1(plain);
  }
  return null;
}

export function flowPlayCatalogueItemDragData(item: CatalogueItem): Record<string, string> {
  const encoded = encodeFlowWidgetDescriptorForDragV1(flowCatalogueItemDescriptor(item));
  return { [FLOW_WIDGET_DRAG_V1_MIME]: encoded, [FLOW_WIDGET_DRAG_PLAIN_MIME]: encoded };
}

export const flowWidgetPaletteDragRef = { active: false };
export const flowWidgetPalettePointerDragRef = { active: false, encoded: null as string | null };
export const flowWidgetDropPointerToWorldRef = {
  current: null as ((clientX: number, clientY: number) => { screen: { x: number; y: number }; world: { x: number; y: number } } | null) | null,
};

/** @emoji 🖱️ Begins pointer palette drag with an encoded widget descriptor. */
export function beginFlowWidgetPalettePointerDrag(encoded: string): void {
  flowWidgetPalettePointerDragRef.active = true;
  flowWidgetPalettePointerDragRef.encoded = encoded;
  flowWidgetPaletteDragRef.active = true;
  window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: { encoded } }));
}

/** @emoji 🖱️ Ends pointer palette drag without committing a drop. */
export function cancelFlowWidgetPalettePointerDrag(): void {
  if (!flowWidgetPalettePointerDragRef.active && !flowWidgetPaletteDragRef.active) {
    return;
  }
  flowWidgetPalettePointerDragRef.active = false;
  flowWidgetPalettePointerDragRef.encoded = null;
  flowWidgetPaletteDragRef.active = false;
  window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: null }));
}

/** @emoji 🎯 True when client coordinates are over the flow widget drop host. */
export function isClientPointOverFlowWidgetDropHost(clientX: number, clientY: number, host: HTMLElement | null | undefined): boolean {
  if (!host) return false;
  const rect = host.getBoundingClientRect();
  return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom;
}

/** @emoji 📥 Commits a palette widget drop at client coordinates. */
export function commitFlowWidgetDropAtClient(
  clientX: number,
  clientY: number,
  descriptor: string,
  host: HTMLElement | null | undefined,
  onWidgetDrop: ((detail: FlowWidgetDropDetail) => void) | undefined,
): boolean {
  const toWorld = flowWidgetDropPointerToWorldRef.current;
  if (!toWorld || !onWidgetDrop) return false;
  const mapped = toWorld(clientX, clientY);
  if (!mapped) return false;
  if (!isClientPointOverFlowWidgetDropHost(clientX, clientY, host)) return false;
  onWidgetDrop({ descriptor, screen: mapped.screen, world: mapped.world });
  return true;
}

/** @emoji 🖱️ Ends pointer palette drag and drops on the viewport when over the host. */
export function endFlowWidgetPalettePointerDrag(
  clientX: number,
  clientY: number,
  host: HTMLElement | null | undefined,
  onWidgetDrop: ((detail: FlowWidgetDropDetail) => void) | undefined,
): void {
  if (!flowWidgetPalettePointerDragRef.active) return;
  const encoded = flowWidgetPalettePointerDragRef.encoded;
  cancelFlowWidgetPalettePointerDrag();
  if (!encoded) return;
  const descriptor = decodeFlowWidgetDescriptorFromDragV1(encoded);
  if (!descriptor) return;
  commitFlowWidgetDropAtClient(clientX, clientY, descriptor, host, onWidgetDrop);
}

/** @emoji 🔍 True when `dataTransfer.types` carries a flow widget palette drag. */
export function flowWidgetDragMimeInTypes(types: readonly string[]): boolean {
  return types.includes(FLOW_WIDGET_DRAG_V1_MIME) || types.includes(FLOW_WIDGET_DRAG_PLAIN_MIME);
}

/** @emoji 🔍 Whether the viewport should accept a palette widget drop for this drag gesture. */
export function flowWidgetDragAcceptsTransfer(types: readonly string[]): boolean {
  if (flowWidgetPalettePointerDragRef.active || flowWidgetPaletteDragRef.active) {
    return true;
  }
  return flowWidgetDragMimeInTypes(types);
}

/** @emoji 🖱️ {@link TreeDragAndDropController} for workbench rows that carry flow widget palette `dragData`. */
export function flowWidgetPaletteTreeDragController(
  dragDataByItemId: ReadonlyMap<string, Record<string, string>>,
): import("@framework/platform/core").TreeDragAndDropController {
  const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
    const payload = dragData?.[FLOW_WIDGET_DRAG_V1_MIME];
    return payload?.trim() ? payload : undefined;
  };
  return {
    getDragData: ({ sourceItem }) => dragDataByItemId.get(sourceItem.id),
    pointerPaletteDrag: {
      readEncodedDragPayload: readEncoded,
      begin: beginFlowWidgetPalettePointerDrag,
      cancel: cancelFlowWidgetPalettePointerDrag,
    },
    onDragStart: ({ sourceItem }) => {
      if (flowWidgetPalettePointerDragRef.active) return;
      flowWidgetPaletteDragRef.active = true;
      const payload = readEncoded(dragDataByItemId.get(sourceItem.id));
      if (payload) {
        window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: { encoded: payload } }));
      }
    },
    onDragEnd: () => {
      if (flowWidgetPalettePointerDragRef.active) return;
      flowWidgetPaletteDragRef.active = false;
      window.dispatchEvent(new CustomEvent("flow-widget-drag-session", { detail: null }));
    },
  };
}

export function parseFlowCatalogueSections(json: string): CatalogueSection[] {
  try {
    const parsed = JSON.parse(json) as CatalogueSection[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}
// #endregion 🔖Catalogue

export interface FlowReorganizeRequest {
  readonly epoch: number;
  readonly optionsJson: string;
}

// #region 🔖FlowCanvas
export interface FlowCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
  readonly store?: FlowStore;
  readonly fixtureDragDrop?: boolean;
  readonly reorganize?: FlowReorganizeRequest;
  readonly onPreviewText?: (text: string) => void;
  readonly onFixtureChange?: (fixtureJson: string) => void;
  readonly onCatalogueReady?: (sections: readonly CatalogueSection[]) => void;
  readonly onWidgetDrop?: (detail: FlowWidgetDropDetail) => void;
}

export function FlowCanvas({
  fixtureJson,
  className,
  store,
  fixtureDragDrop = false,
  reorganize,
  onPreviewText,
  onFixtureChange,
  onCatalogueReady,
  onWidgetDrop,
}: FlowCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<FlowSession | null>(null);
  const rafRef = useRef<number | null>(null);
  const onPreviewTextRef = useRef(onPreviewText);
  const onFixtureChangeRef = useRef(onFixtureChange);
  const onCatalogueReadyRef = useRef(onCatalogueReady);
  const onWidgetDropRef = useRef(onWidgetDrop);
  const storeRef = useRef(store ?? createLocalFlowStore());
  const pointerRef = useRef({ active: false, pan: false, id: -1 });
  const fixtureDragDepthRef = useRef(0);
  const lastVelloThemeJsonRef = useRef("");
  const [fixtureDragActive, setFixtureDragActive] = useState(false);

  const syncVelloTheme = useCallback(() => {
    if (typeof document === "undefined") return;
    try {
      const json = serializeGraphVelloThemePaletteJson();
      if (json !== lastVelloThemeJsonRef.current) {
        lastVelloThemeJsonRef.current = json;
        sessionRef.current?.setVelloThemeJson(json);
      }
    } catch {
      lastVelloThemeJsonRef.current = "";
    }
  }, []);

  useEffect(() => {
    onPreviewTextRef.current = onPreviewText;
  }, [onPreviewText]);

  useEffect(() => {
    onFixtureChangeRef.current = onFixtureChange;
  }, [onFixtureChange]);

  useEffect(() => {
    onCatalogueReadyRef.current = onCatalogueReady;
  }, [onCatalogueReady]);

  useEffect(() => {
    onWidgetDropRef.current = onWidgetDrop;
  }, [onWidgetDrop]);

  useEffect(() => {
    if (store) storeRef.current = store;
  }, [store]);

  const persistFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const json = session.fixtureJson();
      storeRef.current.save(json);
      onFixtureChangeRef.current?.(json);
    } catch {
      /* fixture not ready */
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

  const evaluate = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    session.evaluate();
    const text = session.previewText();
    onPreviewTextRef.current?.(text);
    console.log(`[DEBUG] flow evaluate preview: ${text}`);
  }, []);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !reorganize || reorganize.epoch <= 0) return;
    try {
      session.reorganize(reorganize.optionsJson);
      console.log("[DEBUG] flow canvas reorganized:", session.fixtureJson());
      evaluate();
      persistFixture();
      renderFrame();
    } catch (err) {
      console.log(`[DEBUG] flow canvas reorganize failed: ${String(err)}`);
    }
  }, [reorganize?.epoch, reorganize?.optionsJson, evaluate, persistFixture, renderFrame]);

  const loadCatalogue = useCallback((session: FlowSession) => {
    const sections = parseFlowCatalogueSections(session.catalogueJson());
    onCatalogueReadyRef.current?.(sections);
  }, []);

  const commitWidgetDrop = useCallback(
    (detail: FlowWidgetDropDetail) => {
      const session = sessionRef.current;
      if (!session) return false;
      const handler = onWidgetDropRef.current;
      if (handler) {
        handler(detail);
      } else {
        session.addWidget(detail.descriptor, detail.world.x, detail.world.y);
        evaluate();
        persistFixture();
      }
      renderFrame();
      console.log(`[DEBUG] flow add widget at ${detail.world.x.toFixed(1)}, ${detail.world.y.toFixed(1)}`);
      return true;
    },
    [evaluate, persistFixture, renderFrame],
  );

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    const session = new FlowSession();
    sessionRef.current = session;
    const saved = storeRef.current.load();
    const json = saved ?? fixtureJson ?? flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
    session.loadFixtureJson(json);
    loadCatalogue(session);
    evaluate();
    try {
      const layout = JSON.parse(session.fixtureJson()).layout ?? {};
      console.log(`[DEBUG] flow fixture layout: ${JSON.stringify(layout)}`);
    } catch {
      /* fixture not serializable yet */
    }
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    const initW = Math.max(1, Math.round(rect.width));
    const initH = Math.max(1, Math.round(rect.height));
    session.setSize(initW, initH, dpr);
    canvas.width = Math.round(initW * dpr);
    canvas.height = Math.round(initH * dpr);
    canvas.style.width = `${initW}px`;
    canvas.style.height = `${initH}px`;
    let cleanupResize: (() => void) | undefined;
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
      const tick = () => {
        renderFrame();
        rafRef.current = requestAnimationFrame(tick);
      };
      rafRef.current = requestAnimationFrame(tick);
      cleanupResize = () => {
        ro.disconnect();
        if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      };
    });
    return () => {
      cleanupResize?.();
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      sessionRef.current = null;
    };
  }, [evaluate, fixtureJson, loadCatalogue, renderFrame]);

  const clientToCanvas = useCallback((clientX: number, clientY: number): { x: number; y: number } => {
    const canvas = canvasRef.current;
    if (!canvas) return { x: clientX, y: clientY };
    const rect = canvas.getBoundingClientRect();
    return { x: clientX - rect.left, y: clientY - rect.top };
  }, []);

  const onPointerDown = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || e.button > 2) return;
      e.currentTarget.setPointerCapture(e.pointerId);
      pointerRef.current = { active: true, pan: e.button === 1 || e.shiftKey, id: e.pointerId };
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerDownScreen(x, y, e.metaKey || e.ctrlKey, pointerRef.current.pan);
      renderFrame();
    },
    [clientToCanvas, renderFrame],
  );

  const onPointerMove = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || !pointerRef.current.active || pointerRef.current.id !== e.pointerId) return;
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerMoveScreen(x, y);
      renderFrame();
    },
    [clientToCanvas, renderFrame],
  );

  const onPointerUp = useCallback(
    (e: React.PointerEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session || pointerRef.current.id !== e.pointerId) return;
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.pointerUpScreen(x, y);
      pointerRef.current = { active: false, pan: false, id: -1 };
      evaluate();
      persistFixture();
      renderFrame();
    },
    [clientToCanvas, evaluate, persistFixture, renderFrame],
  );

  const onWheel = useCallback(
    (e: React.WheelEvent<HTMLCanvasElement>) => {
      const session = sessionRef.current;
      if (!session) return;
      e.preventDefault();
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      session.wheel(x, y, e.deltaY);
      persistFixture();
      renderFrame();
    },
    [clientToCanvas, persistFixture, renderFrame],
  );

  const resetFixtureDragDepth = useCallback(() => {
    fixtureDragDepthRef.current = 0;
    setFixtureDragActive(false);
  }, []);

  const onDragEnter = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      if (!fixtureDragDrop) return;
      if (!flowWidgetDragAcceptsTransfer([...e.dataTransfer.types])) return;
      fixtureDragDepthRef.current += 1;
      setFixtureDragActive(true);
    },
    [fixtureDragDrop],
  );

  const onDragLeave = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      if (!fixtureDragDrop) return;
      const target = e.currentTarget as HTMLElement;
      const related = e.relatedTarget as Node | null;
      if (related && target.contains(related)) return;
      fixtureDragDepthRef.current = Math.max(0, fixtureDragDepthRef.current - 1);
      if (fixtureDragDepthRef.current === 0) {
        setFixtureDragActive(false);
      }
    },
    [fixtureDragDrop],
  );

  const onDragOver = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      if (!fixtureDragDrop) return;
      if (!flowWidgetDragAcceptsTransfer([...e.dataTransfer.types])) return;
      e.preventDefault();
      e.dataTransfer.dropEffect = "copy";
    },
    [fixtureDragDrop],
  );

  const commitWidgetDropAtClient = useCallback(
    (clientX: number, clientY: number, descriptor: string) => {
      const session = sessionRef.current;
      const host = containerRef.current ?? canvasRef.current;
      if (!session || !host) return false;
      const rect = host.getBoundingClientRect();
      const screen = { x: clientX - rect.left, y: clientY - rect.top };
      const world = JSON.parse(session.worldFromScreen(screen.x, screen.y)) as { x: number; y: number };
      return commitWidgetDrop({ descriptor, screen, world });
    },
    [commitWidgetDrop],
  );

  const onDrop = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      if (!fixtureDragDrop) return;
      const descriptor = readFlowWidgetDragDataTransfer(e.dataTransfer);
      const accepts = descriptor !== null || flowWidgetDragAcceptsTransfer([...e.dataTransfer.types]);
      if (!accepts) return;
      e.preventDefault();
      resetFixtureDragDepth();
      if (!descriptor) return;
      commitWidgetDropAtClient(e.clientX, e.clientY, descriptor);
    },
    [commitWidgetDropAtClient, fixtureDragDrop, resetFixtureDragDepth],
  );

  useEffect(() => {
    if (!fixtureDragDrop) {
      flowWidgetDropPointerToWorldRef.current = null;
      return;
    }
    flowWidgetDropPointerToWorldRef.current = (clientX, clientY) => {
      const session = sessionRef.current;
      const host = containerRef.current ?? canvasRef.current;
      if (!session || !host) return null;
      const rect = host.getBoundingClientRect();
      const screen = { x: clientX - rect.left, y: clientY - rect.top };
      const world = JSON.parse(session.worldFromScreen(screen.x, screen.y)) as { x: number; y: number };
      return { screen, world };
    };
    return () => {
      flowWidgetDropPointerToWorldRef.current = null;
    };
  }, [fixtureDragDrop]);

  useEffect(() => {
    if (!fixtureDragDrop) return;
    const dropHost = (): HTMLElement | null => containerRef.current ?? canvasRef.current;
    const onWindowPointerUp = (event: PointerEvent) => {
      if (!flowWidgetPalettePointerDragRef.active) return;
      resetFixtureDragDepth();
      endFlowWidgetPalettePointerDrag(event.clientX, event.clientY, dropHost(), (detail) => {
        commitWidgetDrop(detail);
      });
    };
    const onWindowPointerCancel = () => {
      if (!flowWidgetPalettePointerDragRef.active) return;
      resetFixtureDragDepth();
      cancelFlowWidgetPalettePointerDrag();
    };
    window.addEventListener("pointerup", onWindowPointerUp);
    window.addEventListener("pointercancel", onWindowPointerCancel);
    return () => {
      window.removeEventListener("pointerup", onWindowPointerUp);
      window.removeEventListener("pointercancel", onWindowPointerCancel);
    };
  }, [commitWidgetDrop, fixtureDragDrop, resetFixtureDragDepth]);

  return (
    <div
      ref={containerRef}
      className={className ?? `relative h-full min-h-0 w-full min-w-0 bg-canvas${fixtureDragActive ? " ring-2 ring-inset ring-accent" : ""}`}
      onDragEnter={onDragEnter}
      onDragLeave={onDragLeave}
      onDragOver={onDragOver}
      onDrop={onDrop}
    >
      <canvas
        ref={canvasRef}
        className="absolute inset-0 block h-full w-full touch-none"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerCancel={onPointerUp}
        onWheel={onWheel}
      />
    </div>
  );
}
// #endregion 🔖FlowCanvas

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("flow react fixture", () => {
    it("default fixture serializes", () => {
      const json = flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
      expect(json).toContain("flow.fixture/v1");
      expect(json).toContain("math.add");
    });
  });

  describe("flow store", () => {
    it("round-trips fixture json", () => {
      const backing = new Map<string, string>();
      const store = createLocalFlowStore({
        getItem: (k) => backing.get(k) ?? null,
        setItem: (k, v) => {
          backing.set(k, v);
        },
        removeItem: (k) => {
          backing.delete(k);
        },
      });
      const json = flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
      store.save(json);
      expect(store.load()).toBe(json);
      store.clear();
      expect(store.load()).toBeNull();
    });
  });

  describe("flow catalogue descriptor", () => {
    it("builds neuron descriptor", () => {
      const item: CatalogueItem = { kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" };
      expect(flowCatalogueItemDescriptor(item)).toContain("math.add");
    });

    it("round-trips drag payload", () => {
      const item: CatalogueItem = { kind: "neuron", neuronKind: "math.add", name: "Add", summary: "Sum" };
      const encoded = encodeFlowWidgetDescriptorForDragV1(flowCatalogueItemDescriptor(item));
      expect(decodeFlowWidgetDescriptorFromDragV1(encoded)).toContain("math.add");
    });
  });
}
// #endregion 🧪Tests
