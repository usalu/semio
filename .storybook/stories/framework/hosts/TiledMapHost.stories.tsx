// #region 🧲️Header
// 💻️ .storybook/stories/framework/hosts/TiledMapHost.stories.tsx
// Specs: Host the framework renderer's `TiledMapHost` against the real prebuilt `framework/surface/tiled-map/rs`
// `MapSession` WASM engine.
// Summary: A debug-readout host mounts `TiledMapHost` against a `TiledMapScene`; tile URLs point at a
// storybook-relative, intentionally-missing path — the 404s they produce are inert (caught and recorded as
// `tileMiss`, never thrown) and filtered by the accompanying playwright spec, so no real network dependency
// is introduced. `parameters.wasm: ["tiled-map"]` gates first paint on `WASM_LOADERS["tiled-map"]` resolving.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useState, type ReactElement } from "react";

import { TiledMapHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, TiledMapScene, UiComponentSceneNode } from "@semio-tech/framework";

//#region SceneFixtures
const VECTOR_SCENE: TiledMapScene = {
  mapFixtureJson: JSON.stringify({ positions: [{ id: "hq", label: "HQ", name: "Headquarters" }] }),
  cameraJson: '{"x":0,"y":0,"zoom":1}',
  renderMode: "vector",
  vectorStyle: "colored",
  lodMode: "automatic",
  tileUrlTemplate: "/storybook-missing-tiles/{z}/{x}/{y}.png",
  vectorTileUrlTemplate: "/storybook-missing-tiles/vector/{z}/{x}/{y}.pbf",
  layerVisibilityJson: "{}",
  layerStrokeScaleJson: "{}",
  selectionJson: "[]",
  hoverJson: "{}",
  selectionMethod: "rectangle",
  selectionMode: "feature",
};

const IMAGE_SCENE: TiledMapScene = { ...VECTOR_SCENE, renderMode: "image", vectorStyle: "figureGround" };
//#endregion SceneFixtures

//#region StoryHost
function TiledMapStoryHost({ scene, controllerId, surfaceId }: { readonly scene: TiledMapScene; readonly controllerId: string; readonly surfaceId: string }): ReactElement {
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((action: ActionDescriptor): void => {
    setLastAction(action);
  }, []);

  const node: UiComponentSceneNode = { type: "componentScene", surfaceId, controllerId, componentKind: "tiled-map", tiledMap: scene };

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <TiledMapHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="tiled-map-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {JSON.stringify({ lastAction })}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌️hosts/TiledMapHost",
  component: TiledMapStoryHost,
  parameters: { layout: "fullscreen", wasm: ["tiled-map"] },
  tags: ["autodocs"],
} satisfies Meta<typeof TiledMapStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

export const VectorRender: Story = {
  args: { scene: VECTOR_SCENE, controllerId: "gis-play", surfaceId: "gis.play.map" },
};

export const ImageRender: Story = {
  args: { scene: IMAGE_SCENE, controllerId: "gis-play", surfaceId: "gis.play.map-image" },
};
