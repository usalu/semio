// #region 🧲️Header
// 💻️ .storybook/stories/framework/hosts/WorldTerrainLayer.stories.tsx
// Specs: Host the framework renderer's `🗺️WorldTerrainLayer` (chunked DEM-tile terrain mesh) against the real
// prebuilt `framework/surface/terrain/rs` `TerrainSession` WASM engine.
// Summary: `🗺️WorldTerrainLayer` renders bare r3f primitives (`<group>`/`<mesh>`) — normally mounted inside
// `World3dHost`'s own `<WorldCanvas>` — so this story wraps it in a standalone `@react-three/fiber` `<Canvas>`.
// The DEM tile URL template is intentionally storybook-relative/missing: `TerrainTileRenderer` catches fetch
// failures into a `tileMiss` set (never throws), so the layer renders an empty (but real, wasm-backed) mesh
// group rather than depending on a live tile server. `parameters.wasm: ["terrain"]` gates first paint on
// `WASM_LOADERS.terrain` (see `.storybook/preview.tsx`) resolving.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import type { ReactElement } from "react";

import { WorldTerrainLayer } from "@semio-tech/framework-renderer-react";
import { ThreeCanvas } from "@semio-tech/ui-react";

//#region SceneFixtures
/** ⛰️ Matches the `WorldTerrainStyle` fixture in `framework/os/renderer/js/react/index.test.ts` ("accepts extended world 3d scene fields"). */
const HANNOVER_TERRAIN_JSON = JSON.stringify({
  tileUrlTemplate: "/storybook-missing-dem/{z}/{x}/{y}.png",
  projectOriginLon: 9.7382,
  projectOriginLat: 52.3759,
  exaggeration: 1.5,
  colorRamp: "hypsometric",
  minZoom: 6,
  maxZoom: 14,
});
//#endregion SceneFixtures

//#region StoryHost
function WorldTerrainLayerStoryHost({ terrainJson, cameraPosition, cameraTarget }: { readonly terrainJson: string | undefined; readonly cameraPosition: readonly [number, number, number]; readonly cameraTarget: readonly [number, number, number] }): ReactElement {
  return (
    <div style={{ height: "100%", width: "100%", minHeight: "24rem" }}>
      <ThreeCanvas camera={{ position: cameraPosition as [number, number, number], fov: 50 }}>
        <ambientLight intensity={1.2} />
        <directionalLight position={[12, 18, 10]} intensity={2} />
        <WorldTerrainLayer terrainJson={terrainJson} cameraPosition={cameraPosition} cameraTarget={cameraTarget} />
      </ThreeCanvas>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌️hosts/WorldTerrainLayer",
  component: WorldTerrainLayerStoryHost,
  parameters: { layout: "fullscreen", wasm: ["terrain"] },
  // r3f's continuous render loop (`requestAnimationFrame` tick in `TerrainTileRenderer`/`Canvas`) makes this
  // story nondeterministic to re-render as a static docs snapshot; `!autodocs` un-tags the inherited global default.
  tags: ["!autodocs"],
} satisfies Meta<typeof WorldTerrainLayerStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

export const HypsometricRamp: Story = {
  args: { terrainJson: HANNOVER_TERRAIN_JSON, cameraPosition: [0, 40, 120], cameraTarget: [0, 0, 0] },
};
