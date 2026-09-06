// #region 🧲️Header
// 💻️ .storybook/stories/remodel/Frames.stories.tsx
// Specs: Component-level coverage for remodel's `remodeling-frames` window — the `capture` mode's Canvas2d
// frame view (`✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs`), which `🗣️Interpreter` resolves to
// `Canvas2dHost` for `componentKind: "canvas-2d"`.
// Summary: Mounts `Canvas2dHost` against the projected `Canvas2dScene`. No wasm is involved — the host builds a
// `JsonLayersCanvasSession` (a pure `CanvasRenderingContext2D` implementation), so wheel-zoom and middle-drag pan
// round-trip for real and the debounced `setCamera` dispatch folds back through the story reducer.
// ⚠️ TWO real gaps this story exists to expose, both verified in source rather than assumed:
//   1. `frames_layers_json` keys its layers `type: "image"` / `type: "points"` and shapes the point list as
//      `[{x, y, label}]`. `JsonLayersCanvasSession.renderFrame` keys on `kind` (`"image"`/`"polyline"`/`"circle"`/…)
//      and expects `points: [[x, y], …]` — so the plugin's own payload draws nothing recognizable. The
//      `PluginPayload*` stories render exactly what the plugin emits; `HostShapedLayers` renders the same
//      content re-keyed into the schema the host speaks, so the difference is visible side by side.
//   2. The image layer never appears at all for these documents: `remodeling_asset` resolves the asset CHILD
//      handle through `durable_artifacts`, which neither document carries — so only GCP observations can draw.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Canvas2dHost } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { REMODEL_DEFAULT_CONFIG, REMODEL_POPULATED_SCENE, type RemodelConfig } from "./fixture";
import { REMODEL_EDITOR_CONTROLLER_ID, reduceRemodelStoryAction, remodelFramesLayersJson, remodelWindowNode } from "./scene";

/** 🎞️ Every (stream, frame) pair the populated document offers as a cursor target, plus the unset cursor. */
const CURSOR_CHOICES: readonly { readonly id: string; readonly cursor: RemodelConfig["frameCursor"] }[] = [
  { id: "unset", cursor: { streamId: null, frameIndex: 0 } },
  { id: "stream-a#0", cursor: { streamId: "stream-a", frameIndex: 0 } },
  { id: "stream-a#1", cursor: { streamId: "stream-a", frameIndex: 1 } },
  { id: "stream-b#0", cursor: { streamId: "stream-b", frameIndex: 0 } },
];

//#region StoryHost
function RemodelFramesStoryHost({ initialCursor, hostShaped }: { readonly initialCursor: RemodelConfig["frameCursor"]; readonly hostShaped: boolean }): ReactElement {
  const [config, setConfig] = useState<RemodelConfig>({ ...REMODEL_DEFAULT_CONFIG, frameCursor: initialCursor });
  const onAction = useCallback((descriptor: ActionDescriptor): void => setConfig((current) => reduceRemodelStoryAction(current, descriptor)), []);
  const node = useMemo(() => remodelWindowNode("remodeling-frames", REMODEL_POPULATED_SCENE, config, hostShaped), [config, hostShaped]);
  const debug = useMemo(
    () =>
      JSON.stringify({
        windowKindId: "remodeling-frames",
        surfaceKind: "canvas-2d",
        frameCursor: config.frameCursor,
        pluginLayersJson: remodelFramesLayersJson(REMODEL_POPULATED_SCENE, config.frameCursor),
        renderedWith: hostShaped ? "story adapter (kind-keyed, host-readable)" : "the plugin's own payload (type-keyed, unreadable by JsonLayersCanvasSession)",
        imageLayerOmittedReason: "no durableArtifacts map — remodeling_asset resolves the asset CHILD to nothing",
      }),
    [config, hostShaped],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", gap: 6, padding: 4, flexWrap: "wrap" }}>
        {CURSOR_CHOICES.map((choice) => (
          <button key={choice.id} type="button" data-testid={`remodel-frames-cursor-${choice.id}`} onClick={() => onAction({ controllerId: REMODEL_EDITOR_CONTROLLER_ID, action: "setFrameCursor", args: { streamId: choice.cursor.streamId, frameIndex: choice.cursor.frameIndex } })}>
            {choice.id}
          </button>
        ))}
      </div>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <Canvas2dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="remodel-frames-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "📸️remodel🖼️Frames",
  component: RemodelFramesStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof RemodelFramesStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🕳️ The default cursor (`stream_id: None`) — `frames_layers_json` short-circuits to `"[]"` and the session paints its own empty-canvas placeholder. This is what the window shows on boot. */
export const PluginPayloadUnsetCursor: Story = { args: { initialCursor: { streamId: null, frameIndex: 0 }, hostShaped: false } };

/** ⚠️ Cursor on `stream-b#0`, which carries the `Corner` GCP observation at pixel `[10, 20]` — the plugin emits a `type: "points"` layer that `JsonLayersCanvasSession` cannot read, so only its id is printed as fallback text. The exact payload is in the debug readout. */
export const PluginPayloadCursoredFrame: Story = { args: { initialCursor: { streamId: "stream-b", frameIndex: 0 }, hostShaped: false } };

/** 🩹️ The same observation re-keyed by the story into the `kind: "circle"` bounds schema `JsonLayersCanvasSession` actually speaks — what the window would paint once the plugin emits a host-shaped layer list. */
export const HostShapedLayers: Story = { args: { initialCursor: { streamId: "stream-b", frameIndex: 0 }, hostShaped: true } };
