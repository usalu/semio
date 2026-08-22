// #region 🧲️Header
// 💻️ .storybook/stories/framework/hosts/Paint2dHost.stories.tsx
// Specs: Host the framework renderer's `Paint2dHost` against the real prebuilt `framework/surface/paint/rs`
// `RasterSession` WASM engine.
// Summary: A debug-readout host mounts `Paint2dHost` against a `Paint2dScene`; `parameters.wasm: ["paint-2d"]`
// gates first paint on `WASM_LOADERS["paint-2d"]` (see `.storybook/preview.tsx`) resolving.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useState, type ReactElement } from "react";

import { Paint2dHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, Paint2dScene, UiComponentSceneNode } from "@semio-tech/framework-core";

//#region SceneFixtures
const COMPOSITE_SCENE: Paint2dScene = {
  documentSyncJson: JSON.stringify({ schema: "raster.document", id: "raster", layers: [] }),
  assetsJson: "{}",
  cameraJson: '{"x":0,"y":0,"zoom":1}',
  selectionJson: "[]",
  activeUtility: "selectMarquee",
  brushSize: 24,
  brushOpacity: 1,
  viewMode: "composite",
};

const NAVIGATOR_SCENE: Paint2dScene = {
  ...COMPOSITE_SCENE,
  viewMode: "navigator",
  compositeViewportJson: '{"width":640,"height":480}',
};
//#endregion SceneFixtures

//#region StoryHost
function Paint2dStoryHost({ scene, controllerId, surfaceId }: { readonly scene: Paint2dScene; readonly controllerId: string; readonly surfaceId: string }): ReactElement {
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((action: ActionDescriptor): void => {
    setLastAction(action);
  }, []);

  const node: UiComponentSceneNode = { type: "componentScene", surfaceId, controllerId, componentKind: "paint-2d", paint2d: scene };

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <Paint2dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="paint-2d-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {JSON.stringify({ lastAction })}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌️hosts/Paint2dHost",
  component: Paint2dStoryHost,
  parameters: { layout: "fullscreen", wasm: ["paint-2d"] },
  tags: ["autodocs"],
} satisfies Meta<typeof Paint2dStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

export const CompositeView: Story = {
  args: { scene: COMPOSITE_SCENE, controllerId: "raster-play", surfaceId: "raster.play.viewport" },
};

export const NavigatorView: Story = {
  args: { scene: NAVIGATOR_SCENE, controllerId: "raster-play", surfaceId: "raster.play.navigator" },
};
