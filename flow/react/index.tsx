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

// #region 🔖FlowCanvas
export interface FlowCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
  readonly onPreviewText?: (text: string) => void;
}

export function FlowCanvas({ fixtureJson, className, onPreviewText }: FlowCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<FlowSession | null>(null);
  const rafRef = useRef<number | null>(null);
  const onPreviewTextRef = useRef(onPreviewText);
  const [previewText, setPreviewText] = useState("—");
  const [sliderValue, setSliderValue] = useState(3);

  useEffect(() => {
    onPreviewTextRef.current = onPreviewText;
  }, [onPreviewText]);

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

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    const session = new FlowSession();
    sessionRef.current = session;
    const json = fixtureJson ?? flowFixtureToJson(FLOW_DEFAULT_FIXTURE);
    session.loadFixtureJson(json);
    evaluate();
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
      const tick = () => {
        renderFrame();
        rafRef.current = requestAnimationFrame(tick);
      };
      rafRef.current = requestAnimationFrame(tick);
      return () => {
        ro.disconnect();
        if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      };
    });
    return () => {
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      sessionRef.current = null;
    };
  }, [evaluate, fixtureJson, renderFrame]);

  const onSliderChange = useCallback(
    (value: number) => {
      setSliderValue(value);
      sessionRef.current?.setSliderValue("slider", value);
      evaluate();
    },
    [evaluate],
  );

  return (
    <div ref={containerRef} className={className ?? "relative h-full min-h-0 w-full min-w-0 bg-[var(--color-surface-1)]"}>
      <canvas ref={canvasRef} className="absolute inset-0 block h-full w-full touch-none" />
      <div className="pointer-events-auto absolute bottom-3 left-3 flex flex-col gap-2 rounded-md border border-[var(--color-border)] bg-[var(--color-surface-2)] p-3 text-sm shadow-md">
        <label className="flex items-center gap-2">
          <span className="w-14">Slider</span>
          <input
            type="range"
            min={0}
            max={10}
            step={0.1}
            value={sliderValue}
            onChange={(e) => onSliderChange(Number(e.target.value))}
          />
          <span className="tabular-nums">{sliderValue.toFixed(1)}</span>
        </label>
        <div>
          Preview: <strong className="tabular-nums">{previewText}</strong>
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
}
// #endregion 🧪Tests
