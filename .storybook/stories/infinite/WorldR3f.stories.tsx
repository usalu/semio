// #region 🧲Header
// 💻 .storybook/story/infinite/WorldR3f.stories.tsx
// Specs: Compose the raw `@semio-tech/infinite-world-r3f` layer primitives (`WorldChunks`/`ViewRadiusLayer`/`WorldLodGridHelper`/`GridLayer`/`WorldLayer`/`WorldLayerStack`) inside a plain `@react-three/fiber` `Canvas`, independent of `World3dHost`'s full componentScene wiring (that's `../puzzle/3d/World.stories.tsx`'s job).
// Summary: `WorldChunks`/`ViewRadiusLayer` bucket-and-cull their children by an `origin` prop read straight off each child element (see `chunkKey`/`useVisibleChunkKeys` in `framework/os/kernel/infinite/world/r3f/index.tsx`), so a grid of plain `<mesh>` boxes tagged with `origin` is enough to exercise real chunk visibility culling with zero fixtures/WASM. `WorldLodGridHelper`/`GridLayer` need the `useLod()` context that only `WorldLodBridge` provides — wired here exactly as `World3dHost` wires it.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useMemo, useRef, type ReactElement } from "react";

import {
  Canvas,
  DEFAULT_LOD_GRID_FACTOR,
  DEFAULT_MANUAL_LOD,
  GridLayer,
  ViewRadiusLayer,
  WorldLayer,
  WorldLayerStack,
  WorldLodBridge,
  WorldOrbitGated,
  WorldOrbitViewSnapGateProvider,
  type Vec3,
} from "../../../framework/os/kernel/infinite/world/r3f/index.tsx";

//#region ChunkField
const STORY_CHUNK_GRID_EXTENT = 3;
const STORY_CHUNK_SPACING = 12;
const STORY_CHUNK_SIZE = 20;
const STORY_CHUNK_COLORS = ["#2563eb", "#f97316", "#16a34a", "#dc2626"] as const;

/** @emoji 🧊 A grid of boxes tagged with `origin` (read by `WorldChunks`/`ViewRadiusLayer` to bucket+cull) so real chunk visibility culling has something to cull. */
function useStoryChunkOrigins(): readonly Vec3[] {
  return useMemo(() => {
    const origins: Vec3[] = [];
    for (let ix = -STORY_CHUNK_GRID_EXTENT; ix <= STORY_CHUNK_GRID_EXTENT; ix += 1) {
      for (let iy = -STORY_CHUNK_GRID_EXTENT; iy <= STORY_CHUNK_GRID_EXTENT; iy += 1) {
        origins.push([ix * STORY_CHUNK_SPACING, iy * STORY_CHUNK_SPACING, 0]);
      }
    }
    return origins;
  }, []);
}

/** @emoji 🧊 `origin` is this component's own (fully-typed) prop, not a Three.js one — `WorldChunks` reads `child.props.origin` straight off the `<StoryChunkBox>` element it buckets (see header docstring), never off the `<mesh>` it renders internally. */
function StoryChunkBox({ origin }: { readonly origin: Vec3 }): ReactElement {
  const colorIndex = (Math.round(origin[0] / STORY_CHUNK_SIZE) + Math.round(origin[1] / STORY_CHUNK_SIZE) + STORY_CHUNK_COLORS.length * 2) % STORY_CHUNK_COLORS.length;
  return (
    <mesh position={origin}>
      <boxGeometry args={[4, 4, 4]} />
      <meshStandardMaterial color={STORY_CHUNK_COLORS[colorIndex]} />
    </mesh>
  );
}
//#endregion ChunkField

//#region StoryHost
function WorldR3fChunkedStoryHost({ maxDistance, showLodGrid }: { readonly maxDistance: number; readonly showLodGrid: boolean }): ReactElement {
  const origins = useStoryChunkOrigins();
  const lodRef = useRef(DEFAULT_MANUAL_LOD);

  return (
    <div className="semio-world-r3f-story relative h-full min-h-[24rem] w-full">
      <Canvas camera={{ fov: 50, position: [46, -46, 34], near: 0.1, far: 5000 }} style={{ width: "100%", height: "100%" }}>
        <ambientLight intensity={1.2} />
        <directionalLight position={[20, 20, 30]} intensity={1.4} />
        <WorldOrbitViewSnapGateProvider>
          <WorldOrbitGated zoom={1} />
          <WorldLodBridge lodRef={lodRef} distanceReference={40} gridFactor={DEFAULT_LOD_GRID_FACTOR} gridSnapEnabled={false} showLodGrid={showLodGrid} automaticLod gridDatum={[0, 0, 0]} depthVariableLod={false} manualLod={DEFAULT_MANUAL_LOD}>
            <WorldLayerStack>
              <WorldLayer order={0} name="chunked-field">
                <ViewRadiusLayer chunkSize={STORY_CHUNK_SIZE} maxDistance={maxDistance}>
                  {origins.map((origin) => (
                    <StoryChunkBox key={origin.join(",")} origin={origin} />
                  ))}
                </ViewRadiusLayer>
              </WorldLayer>
            </WorldLayerStack>
          </WorldLodBridge>
        </WorldOrbitViewSnapGateProvider>
      </Canvas>
    </div>
  );
}

function WorldR3fOrderedLayersStoryHost(): ReactElement {
  const lodRef = useRef(DEFAULT_MANUAL_LOD);

  return (
    <div className="semio-world-r3f-story relative h-full min-h-[24rem] w-full">
      <Canvas camera={{ fov: 50, position: [24, -24, 18], near: 0.1, far: 5000 }} style={{ width: "100%", height: "100%" }}>
        <ambientLight intensity={1.2} />
        <directionalLight position={[10, 10, 16]} intensity={1.4} />
        <WorldOrbitViewSnapGateProvider>
          <WorldOrbitGated zoom={1} />
          <WorldLodBridge lodRef={lodRef} distanceReference={30} gridFactor={DEFAULT_LOD_GRID_FACTOR} gridSnapEnabled={false} showLodGrid={false} automaticLod depthVariableLod={false} manualLod={DEFAULT_MANUAL_LOD}>
            <WorldLayerStack>
              {/* 📐 Background layer (negative order): the LOD grid, composed explicitly via `GridLayer` instead of `WorldLodBridge`'s built-in `showLodGrid` toggle. */}
              <WorldLayer order={-1} name="grid-background">
                <GridLayer gridDatum={[0, 0, 0]} />
              </WorldLayer>
              {/* 🧊 Foreground layer (order 0): the actual content, rendered after (i.e. visually atop) the grid. */}
              <WorldLayer order={0} name="content">
                <mesh position={[0, 0, 2]}>
                  <boxGeometry args={[4, 4, 4]} />
                  <meshStandardMaterial color="#2563eb" />
                </mesh>
              </WorldLayer>
            </WorldLayerStack>
          </WorldLodBridge>
        </WorldOrbitViewSnapGateProvider>
      </Canvas>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "♾️infinite/WorldR3f",
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

/** 🧊 `WorldChunks`/`ViewRadiusLayer` bucketing a 7x7 grid of origin-tagged boxes, plus `WorldLodBridge`'s built-in LOD grid. */
export const ChunkedField: StoryObj<typeof WorldR3fChunkedStoryHost> = {
  render: (args) => <WorldR3fChunkedStoryHost {...args} />,
  args: {
    maxDistance: 300,
    showLodGrid: true,
  },
};

/** 🧊 A tight view radius so most chunks are culled — same field, `maxDistance` lowered below the grid's span. */
export const CulledField: StoryObj<typeof WorldR3fChunkedStoryHost> = {
  render: (args) => <WorldR3fChunkedStoryHost {...args} />,
  args: {
    maxDistance: 24,
    showLodGrid: false,
  },
};

/** 🗂️ `WorldLayer`/`WorldLayerStack` ordering an explicit `GridLayer` background behind foreground content. */
export const OrderedLayers: StoryObj<typeof WorldR3fOrderedLayersStoryHost> = {
  render: () => <WorldR3fOrderedLayersStoryHost />,
};
