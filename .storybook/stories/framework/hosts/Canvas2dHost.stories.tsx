// #region 🧲Header
// 💻 .storybook/stories/framework/hosts/Canvas2dHost.stories.tsx
// Specs: Host the framework renderer's `Canvas2dHost` with zero WASM engine — its `sessionFactory` builds a
// `JsonLayersCanvasSession`, a pure-`CanvasRenderingContext2D` implementation of `GraphWasmSession`
// (`@semio-tech/infinite-cavas-react-renderer`'s generic canvas host), so no `cdylib` session is involved.
// Summary: `layersJson` drives real drawing (bounds-based node boxes, a dashed "wire" line) with zero fixture
// setup; wheel-zoom/middle-drag-pan already round-trip for real inside `JsonLayersCanvasSession` itself, so the
// story-local reducer only needs to fold the debounced `setCamera` dispatch back into `cameraX`/`cameraY`/`zoom`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Canvas2dHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, Canvas2dScene, UiComponentSceneNode } from "@semio-tech/framework-core";

//#region StoryTypes
type StoryCamera = { readonly x: number; readonly y: number; readonly zoom: number };
type StoryLayer = Record<string, unknown>;
type StoryCanvas2dState = { readonly camera: StoryCamera; readonly layers: readonly StoryLayer[] };
//#endregion StoryTypes

//#region Fixtures
/** 🟦🟠 Two bounds-based node boxes (rect + circle) plus a dashed "wire" line — exercises `drawBoundsLayer`'s rect/circle branches and the `x0`/`y0`/`x1`/`y1` line branch in `JsonLayersCanvasSession.renderFrame`. */
const STORY_LAYERS: readonly StoryLayer[] = [
  { id: "node-alpha", kind: "box", role: "node", name: "Alpha", x: -140, y: -50, width: 140, height: 70, color: "#38bdf8", selected: false },
  { id: "node-beta", kind: "circle", role: "node", name: "Beta", x: 40, y: 30, width: 96, height: 96, color: "#f97316", selected: true },
  { id: "wire-alpha-beta", role: "wire", x0: 0, y0: -15, x1: 88, y1: 78, color: "#94a3b8" },
];
//#endregion Fixtures

//#region Reducer
/** @emoji 🎥 Story-local mirror of the `setCamera` handling a real host app performs against `Canvas2dHost`'s debounced camera-sync dispatch (`framework/renderer/react/index.tsx`'s `CAMERA_SYNC_DEBOUNCE_MS`). */
function reduceStoryCanvas2dAction(state: StoryCanvas2dState, descriptor: ActionDescriptor): StoryCanvas2dState {
  if (descriptor.action !== "setCamera") return state;
  const camera = (descriptor.args as { readonly camera?: StoryCamera } | undefined)?.camera;
  return camera ? { ...state, camera } : state;
}
//#endregion Reducer

//#region SceneNode
const STORY_CANVAS_2D_CONTROLLER_ID = "canvas-2d-story";

function buildStoryCanvas2dScene(state: StoryCanvas2dState): Canvas2dScene {
  return { cameraX: state.camera.x, cameraY: state.camera.y, zoom: state.camera.zoom, layersJson: JSON.stringify(state.layers) };
}
//#endregion SceneNode

//#region StoryHost
function Canvas2dStoryHost({ initialLayers }: { readonly initialLayers: readonly StoryLayer[] }): ReactElement {
  const [state, setState] = useState<StoryCanvas2dState>({ camera: { x: 0, y: 0, zoom: 1 }, layers: initialLayers });
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setLastAction(descriptor);
    setState((current) => reduceStoryCanvas2dAction(current, descriptor));
  }, []);

  const node: UiComponentSceneNode = useMemo(
    () => ({ type: "componentScene", surfaceId: "canvas-2d.story.overview", controllerId: STORY_CANVAS_2D_CONTROLLER_ID, componentKind: "canvas-2d", canvas2d: buildStoryCanvas2dScene(state) }),
    [state],
  );
  const debug = useMemo(() => JSON.stringify({ camera: state.camera, layerCount: state.layers.length, lastAction }), [state, lastAction]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <Canvas2dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="canvas-2d-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌hosts/Canvas2dHost",
  component: Canvas2dStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof Canvas2dStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🖼️ Two node boxes and a wire — mouse-wheel to zoom, middle-drag (or `transformMove` utility) to pan; the camera round-trips back through `setCamera` into the debug readout. */
export const Scene: Story = {
  args: { initialLayers: STORY_LAYERS },
};

/** 🕳️ `layersJson: "[]"` — `JsonLayersCanvasSession.renderFrame` draws its own "Empty canvas" placeholder text. */
export const EmptyCanvas: Story = {
  args: { initialLayers: [] },
};
