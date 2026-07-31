// #region 🧲️Header
// 💻️ .storybook/story/infinite/GraphWasmCanvas.stories.tsx
// Specs: Host `GraphWasmCanvas` (`framework/product/os/module/infinite/canvas/react-renderer/index.tsx`) against a pure-JS mock `GraphWasmSession` — no real WASM session, no puzzle-2d program.
// Summary: `GraphWasmCanvas` only depends on the small `GraphWasmSession` interface (`attachCanvas`/`setSize`/`renderFrame`/pointer hooks); the mock here paints a deterministic checkerboard + a pointer-tracked marker onto the raw `<canvas>` 2D context so the story proves the reconciler-free host wiring (attach → resize → RAF loop → pointer events) without pulling in any leaf bundle's `cdylib`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react";
import { useState, type ReactElement } from "react";

import { GraphWasmCanvas } from "../../../framework/product/os/module/infinite/canvas/react-renderer/index.tsx";
import type { CanvasInputModifiers, GraphWasmSession } from "../../../framework/product/os/module/infinite/canvas/react-renderer/index.tsx";

//#region MockSession
const STORY_CHECKER_CELL_PX = 32;
const STORY_CHECKER_COLOR_A = "#1d4ed8";
const STORY_CHECKER_COLOR_B = "#93c5fd";
const STORY_POINTER_COLOR = "#f97316";

/** @emoji 🕸️ Pure-JS stand-in for a leaf bundle's `cdylib` session: paints a deterministic checkerboard sized to the logical canvas plus a marker at the last pointer position — enough to exercise `GraphWasmCanvas`'s attach/resize/RAF/pointer wiring with zero WASM. */
function createMockGraphWasmSession(onPointerCount: (count: number) => void): GraphWasmSession {
  let ctx: CanvasRenderingContext2D | null = null;
  let logicalWidth = 0;
  let logicalHeight = 0;
  let dpr = 1;
  let pointer: { readonly x: number; readonly y: number } | null = null;
  let pointerCount = 0;

  const paint = (): void => {
    if (!ctx) return;
    ctx.save();
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, logicalWidth, logicalHeight);
    for (let y = 0; y < logicalHeight; y += STORY_CHECKER_CELL_PX) {
      for (let x = 0; x < logicalWidth; x += STORY_CHECKER_CELL_PX) {
        const parity = (Math.round(x / STORY_CHECKER_CELL_PX) + Math.round(y / STORY_CHECKER_CELL_PX)) % 2;
        ctx.fillStyle = parity === 0 ? STORY_CHECKER_COLOR_A : STORY_CHECKER_COLOR_B;
        ctx.fillRect(x, y, STORY_CHECKER_CELL_PX, STORY_CHECKER_CELL_PX);
      }
    }
    if (pointer) {
      ctx.fillStyle = STORY_POINTER_COLOR;
      ctx.beginPath();
      ctx.arc(pointer.x, pointer.y, 10, 0, Math.PI * 2);
      ctx.fill();
    }
    ctx.restore();
  };

  const trackPointer = (x: number, y: number): void => {
    pointer = { x, y };
    pointerCount += 1;
    onPointerCount(pointerCount);
  };

  return {
    async attachCanvas(canvas, logicalW, logicalH, deviceRatio) {
      ctx = canvas.getContext("2d");
      logicalWidth = logicalW;
      logicalHeight = logicalH;
      dpr = deviceRatio;
    },
    setSize(width, height, deviceRatio) {
      logicalWidth = width;
      logicalHeight = height;
      dpr = deviceRatio;
    },
    renderFrame() {
      paint();
    },
    pointerDown(x, y) {
      trackPointer(x, y);
    },
    pointerMove(x, y) {
      trackPointer(x, y);
    },
    pointerUp(x, y, _modifiers?: CanvasInputModifiers) {
      trackPointer(x, y);
    },
    wheel() {
      /* no zoom in this mock */
    },
  };
}
//#endregion MockSession

//#region StoryHost
function GraphWasmCanvasStoryHost({ enablePointer }: { readonly enablePointer: boolean }): ReactElement {
  const [pointerCount, setPointerCount] = useState(0);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div className="semio-graph-wasm-canvas-story" style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <GraphWasmCanvas sessionFactory={() => createMockGraphWasmSession(setPointerCount)} enablePointer={enablePointer} />
      </div>
      <pre data-testid="graph-wasm-canvas-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {JSON.stringify({ pointerEvents: pointerCount })}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "♾️infinite/GraphWasmCanvas",
  component: GraphWasmCanvasStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof GraphWasmCanvasStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

export const MockSession: Story = {
  args: {
    enablePointer: true,
  },
};

export const PointerDisabled: Story = {
  args: {
    enablePointer: false,
  },
};
