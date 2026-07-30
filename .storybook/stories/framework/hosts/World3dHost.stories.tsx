// #region 🧲Header
// 💻 .storybook/stories/framework/hosts/World3dHost.stories.tsx
// Specs: Host the framework renderer's `World3dHost` — a pure `@react-three/fiber` + `@semio-tech/infinite-world-r3f`
// component (`WorldCanvas`) that owns no WASM engine of its own, EXCEPT when `scene.terrainJson` is present:
// it then mounts `WorldTerrainLayer`, which drives the real `framework/surface/terrain/rs` `TerrainSession`.
// Summary: `MinimalViewport` needs no wasm loader at all; `TerrainViewport` sets `parameters.wasm: ["terrain"]`
// so `withWasm` gates first paint on `WASM_LOADERS.terrain` (see `.storybook/preview.tsx`) resolving.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useState, type ReactElement } from "react";

import { World3dHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, UiComponentSceneNode, World3dScene } from "@semio-tech/framework-core";

//#region SceneFixtures
const MINIMAL_SCENE: World3dScene = {
  cameraJson: "{}",
  meshesJson: "[]",
  instancesJson: "[]",
  selectionJson: "{}",
  interactionJson: '{"activeUtility":"select"}',
};

/** ⛰️ Matches the `terrainJson` fixture in `framework/os/renderer/js/react/index.test.ts` ("accepts extended world 3d scene fields") — mounts `WorldTerrainLayer` internally, backed by the real `TerrainSession` WASM engine. */
const TERRAIN_SCENE: World3dScene = {
  ...MINIMAL_SCENE,
  terrainJson: JSON.stringify({
    tileUrlTemplate: "/storybook-missing-dem/{z}/{x}/{y}.png",
    projectOriginLon: 9.7382,
    projectOriginLat: 52.3759,
    exaggeration: 1.5,
    colorRamp: "hypsometric",
    minZoom: 6,
    maxZoom: 14,
  }),
};
//#endregion SceneFixtures

//#region StoryHost
function World3dStoryHost({ scene, controllerId, surfaceId }: { readonly scene: World3dScene; readonly controllerId: string; readonly surfaceId: string }): ReactElement {
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((action: ActionDescriptor): void => {
    setLastAction(action);
  }, []);

  const node: UiComponentSceneNode = { type: "componentScene", surfaceId, controllerId, componentKind: "world-3d", world3d: scene };

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", minHeight: "24rem", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <World3dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="world-3d-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {JSON.stringify({ lastAction })}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌hosts/World3dHost",
  component: World3dStoryHost,
  parameters: { layout: "fullscreen" },
  // The r3f `WorldCanvas` render loop makes this heavy/nondeterministic to re-render as a static docs snapshot;
  // `!autodocs` un-tags the inherited global default.
  tags: ["!autodocs"],
} satisfies Meta<typeof World3dStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🌐 No `terrainJson` — pure r3f viewport, no WASM engine involved. */
export const MinimalViewport: Story = {
  args: { scene: MINIMAL_SCENE, controllerId: "puzzle3d-play", surfaceId: "puzzle.3d.play.viewport" },
};

/** ⛰️ `terrainJson` present — mounts `WorldTerrainLayer` against the real `TerrainSession` WASM engine. */
export const TerrainViewport: Story = {
  args: { scene: TERRAIN_SCENE, controllerId: "puzzle3d-play", surfaceId: "puzzle.3d.play.terrain" },
  parameters: { wasm: ["terrain"] },
};
