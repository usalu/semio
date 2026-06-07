// #region 🧲Header
/** @emoji 🌊 `@flow/react` — WASM flow renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef, useState } from "react";
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

export const FLOW_DRAG_MIME = "application/x-flow-widget";

function catalogueItemDescriptor(item: CatalogueItem): string {
  if (item.kind === "neuron" && item.neuronKind) {
    return JSON.stringify({ kind: "neuron", neuronKind: item.neuronKind });
  }
  if (item.kind === "outputAction") {
    return JSON.stringify({ kind: "outputAction", action: item.action ?? "log" });
  }
  return JSON.stringify({ kind: item.kind });
}

export interface FlowCatalogueProps {
  readonly sections: readonly CatalogueSection[];
  readonly className?: string;
}

export function FlowCatalogue({ sections, className }: FlowCatalogueProps): React.JSX.Element {
  return (
    <aside
      className={className ?? "pointer-events-auto flex w-52 shrink-0 flex-col gap-2 overflow-y-auto border-r border-[var(--color-border)] bg-[var(--color-surface-2)] p-2 text-sm"}
      data-testid="flow-catalogue"
    >
      {sections.map((section) => (
        <details key={section.id} open className="group rounded border border-[var(--color-border)] bg-[var(--color-surface-1)]">
          <summary className="cursor-pointer select-none px-2 py-1.5 font-medium">{section.title}</summary>
          <ul className="flex flex-col gap-1 p-1">
            {section.items.map((item) => (
              <li key={`${section.id}-${item.kind}-${item.neuronKind ?? item.name}`}>
                <button
                  type="button"
                  draggable
                  className="w-full cursor-grab rounded px-2 py-1.5 text-left hover:bg-[var(--color-surface-3)] active:cursor-grabbing"
                  data-testid={`flow-catalogue-item-${item.neuronKind ?? item.kind}`}
                  onDragStart={(e) => {
                    e.dataTransfer.setData(FLOW_DRAG_MIME, catalogueItemDescriptor(item));
                    e.dataTransfer.effectAllowed = "copy";
                  }}
                >
                  <div className="font-medium">{item.name}</div>
                  <div className="text-xs text-[var(--color-muted)]">{item.summary}</div>
                </button>
              </li>
            ))}
          </ul>
        </details>
      ))}
    </aside>
  );
}
// #endregion 🔖Catalogue

// #region 🔖FlowCanvas
export interface FlowCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
  readonly store?: FlowStore;
  readonly onPreviewText?: (text: string) => void;
  readonly onFixtureChange?: (fixtureJson: string) => void;
}

export function FlowCanvas({ fixtureJson, className, store, onPreviewText, onFixtureChange }: FlowCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<FlowSession | null>(null);
  const rafRef = useRef<number | null>(null);
  const onPreviewTextRef = useRef(onPreviewText);
  const onFixtureChangeRef = useRef(onFixtureChange);
  const storeRef = useRef(store ?? createLocalFlowStore());
  const pointerRef = useRef({ active: false, pan: false, id: -1 });
  const [previewText, setPreviewText] = useState("—");
  const [sliderValue, setSliderValue] = useState(3);
  const [catalogueSections, setCatalogueSections] = useState<CatalogueSection[]>([]);

  useEffect(() => {
    onPreviewTextRef.current = onPreviewText;
  }, [onPreviewText]);

  useEffect(() => {
    onFixtureChangeRef.current = onFixtureChange;
  }, [onFixtureChange]);

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
      sessionRef.current?.renderFrame();
    } catch {
      /* gpu not ready */
    }
  }, []);

  const evaluate = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    session.evaluate();
    const text = session.previewText();
    setPreviewText((prev) => (prev === text ? prev : text));
    onPreviewTextRef.current?.(text);
    console.log(`[DEBUG] flow evaluate preview: ${text}`);
  }, []);

  const loadCatalogue = useCallback((session: FlowSession) => {
    try {
      const parsed = JSON.parse(session.catalogueJson()) as CatalogueSection[];
      setCatalogueSections(parsed);
    } catch {
      setCatalogueSections([]);
    }
  }, []);

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
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    const initW = Math.max(1, Math.round(rect.width));
    const initH = Math.max(1, Math.round(rect.height));
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

  const onDragOver = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    if (e.dataTransfer.types.includes(FLOW_DRAG_MIME)) {
      e.preventDefault();
      e.dataTransfer.dropEffect = "copy";
    }
  }, []);

  const onDrop = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      const session = sessionRef.current;
      const canvas = canvasRef.current;
      if (!session || !canvas) return;
      const descriptor = e.dataTransfer.getData(FLOW_DRAG_MIME);
      if (!descriptor) return;
      e.preventDefault();
      const { x, y } = clientToCanvas(e.clientX, e.clientY);
      const world = JSON.parse(session.worldFromScreen(x, y)) as { x: number; y: number };
      session.addWidget(descriptor, world.x, world.y);
      evaluate();
      persistFixture();
      renderFrame();
      console.log(`[DEBUG] flow add widget at ${world.x.toFixed(1)}, ${world.y.toFixed(1)}`);
    },
    [clientToCanvas, evaluate, persistFixture, renderFrame],
  );

  const onSliderChange = useCallback(
    (value: number) => {
      setSliderValue(value);
      sessionRef.current?.setSliderValue("slider", value);
      evaluate();
      persistFixture();
    },
    [evaluate, persistFixture],
  );

  const onResetFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    storeRef.current.clear();
    const json = flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
    session.loadFixtureJson(json);
    evaluate();
    persistFixture();
    renderFrame();
  }, [evaluate, persistFixture, renderFrame]);

  return (
    <div className={className ?? "relative flex h-full min-h-0 w-full min-w-0 bg-[var(--color-surface-1)]"} onDragOver={onDragOver} onDrop={onDrop}>
      <FlowCatalogue sections={catalogueSections} />
      <div ref={containerRef} className="relative min-h-0 min-w-0 flex-1">
        <canvas
          ref={canvasRef}
          className="absolute inset-0 block h-full w-full touch-none"
          onPointerDown={onPointerDown}
          onPointerMove={onPointerMove}
          onPointerUp={onPointerUp}
          onPointerCancel={onPointerUp}
          onWheel={onWheel}
        />
        <div className="pointer-events-auto absolute bottom-3 left-3 flex flex-col gap-2 rounded-md border border-[var(--color-border)] bg-[var(--color-surface-2)] p-3 text-sm shadow-md">
          <label className="flex items-center gap-2">
            <span className="w-14">Slider</span>
            <input type="range" min={0} max={10} step={0.1} value={sliderValue} onChange={(ev) => onSliderChange(Number(ev.target.value))} />
            <span className="tabular-nums">{sliderValue.toFixed(1)}</span>
          </label>
          <div>
            Preview: <strong className="tabular-nums">{previewText}</strong>
          </div>
          <button type="button" className="rounded border border-[var(--color-border)] px-2 py-1 text-xs hover:bg-[var(--color-surface-3)]" onClick={onResetFixture}>
            Reset flow
          </button>
        </div>
      </div>
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
      expect(catalogueItemDescriptor(item)).toContain("math.add");
    });
  });
}
// #endregion 🧪Tests
