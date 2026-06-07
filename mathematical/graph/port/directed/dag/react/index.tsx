// #region 🧲Header
/** @emoji 🌳 `@dag/react` — WASM DAG renderer + React canvas. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useRef } from "react";
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

// #region 🔖Fixture
export interface DagFixtureV1 {
  readonly schema: "dag.fixture/v1";
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly nodes: readonly DagIoNodeV1[];
  readonly edges: readonly { readonly id: string; readonly source: string; readonly target: string }[];
}

export interface DagIoNodeV1 {
  readonly id: string;
  readonly name: string;
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly inputs: readonly { readonly id: string; readonly label: string }[];
  readonly outputs: readonly { readonly id: string; readonly label: string }[];
}

export const DAG_DEFAULT_FIXTURE: DagFixtureV1 = {
  schema: "dag.fixture/v1",
  camera: { x: 0, y: 0, zoom: 1 },
  nodes: [
    { id: "source", name: "Source", x: -360, y: 0, width: 160, height: 72, inputs: [], outputs: [{ id: "out", label: "value" }] },
    { id: "scale", name: "Scale", x: -120, y: -80, width: 160, height: 72, inputs: [{ id: "in", label: "value" }], outputs: [{ id: "out", label: "scaled" }] },
    { id: "offset", name: "Offset", x: -120, y: 80, width: 160, height: 72, inputs: [{ id: "in", label: "value" }], outputs: [{ id: "out", label: "shifted" }] },
    {
      id: "combine",
      name: "Combine",
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
      id: "split",
      name: "Split",
      x: 360,
      y: 0,
      width: 160,
      height: 96,
      inputs: [{ id: "in", label: "merged" }],
      outputs: [
        { id: "lo", label: "lo" },
        { id: "hi", label: "hi" },
      ],
    },
    { id: "sink", name: "Sink", x: 600, y: -48, width: 160, height: 72, inputs: [{ id: "in", label: "result" }], outputs: [] },
  ],
  edges: [
    { id: "e1", source: "source:out", target: "scale:in" },
    { id: "e2", source: "source:out", target: "offset:in" },
    { id: "e3", source: "scale:out", target: "combine:a" },
    { id: "e4", source: "offset:out", target: "combine:b" },
    { id: "e5", source: "combine:out", target: "split:in" },
    { id: "e6", source: "split:lo", target: "sink:in" },
  ],
};

export function dagFixtureToJson(fixture: DagFixtureV1): string {
  return JSON.stringify(fixture);
}
// #endregion 🔖Fixture

// #region 🔖DagCanvas
export interface DagCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
}

export function DagCanvas({ fixtureJson, className }: DagCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<DagSession | null>(null);
  const rafRef = useRef<number | null>(null);

  const renderFrame = useCallback(() => {
    try {
      sessionRef.current?.renderFrame();
    } catch {
      /* gpu not ready */
    }
  }, []);

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
    <div ref={containerRef} className={className ?? "relative h-full w-full min-h-0 min-w-0 bg-[#14161c]"}>
      <canvas ref={canvasRef} className="block h-full w-full touch-none" />
    </div>
  );
}
// #endregion 🔖DagCanvas

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("dag fixture", () => {
    it("default fixture has six nodes", () => {
      expect(DAG_DEFAULT_FIXTURE.nodes.length).toBe(6);
      expect(DAG_DEFAULT_FIXTURE.edges.length).toBe(6);
    });
  });
}
// #endregion 🧪Tests
