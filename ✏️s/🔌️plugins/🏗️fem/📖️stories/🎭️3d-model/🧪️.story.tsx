// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/🏗️fem/📖️stories/🎭️3d-model/🧪️.story.tsx
// Specs: Host the framework renderer's `🌐️World3dHost` for the fem3d play app's `fem3d-model` window
// (body key `fem3d.play.model`), driven by the REAL shipped example document
// (`🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`).
// Summary: The window encodes `SurfaceKind::World3d` through `crate::app_surface::world_3d_surface`, which
// the shell dispatches to `World3dHost` — plain `@react-three/fiber` + `WorldCanvas`, owning no wasm engine
// (only a `terrainJson` scene would pull one), so this world paints for real with no plugin wasm and no
// `WASM_LOADERS` entry. The scene is a port of the window's `#[cfg(test)] render` path
// (`fem3d_scene_parts` → `world3d_meshes_json_from_kinds(["box"])` + `fem3d_structural_instances`), NOT of
// its runtime `render_with_progress` path: at runtime `meshes_json`/`instances_json` stay the literal `"[]"`
// and all geometry arrives on `scene.snapshot`, the `live_visual` page lease, which is a mounted-reactor
// artifact with no browser equivalent. `fem3d_solid_mesh_entries` (the tet-mesher's `solid-*` boundary
// meshes) is likewise Rust-only and is reported as a counted omission rather than approximated.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { World3dHost } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import type { ActionDescriptor } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

import {
  buildFem3dSceneNode,
  fem3dStoryStateFor,
  fem3dStructuralInstances,
  fem3dSummaryLines,
  FEM_STORY_LOCALES,
  FEM3D_SOLID_MESH_OMISSION,
  FEM3D_STORY_EXAMPLE_ID,
  FEM3D_CLEARED_EXAMPLE_ID,
  femStoryLabel,
  femStoryOmissions,
  reduceFem3dStoryAction,
  type Fem3dStoryState,
  type FemStoryLocale,
} from "../🧭️coordination/🧫️fixtures/🧫️scene/🟦️.ts";

//#region StoryHost
const FEM3D_MODEL_BODY_KEY = "fem3d.play.model";
const FEM3D_EDITOR_CONTROLLER_ID = "fem3d-play";

function Fem3dModelStoryHost({ initialExampleId, locale }: { readonly initialExampleId: string; readonly locale: FemStoryLocale }): ReactElement {
  const [state, setState] = useState<Fem3dStoryState>(() => fem3dStoryStateFor(initialExampleId, locale));
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setLastAction(descriptor);
    setState((current) => reduceFem3dStoryAction(current, descriptor.action, descriptor.args));
  }, []);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>): void => onAction({ controllerId: FEM3D_EDITOR_CONTROLLER_ID, action, args: { surfaceId: FEM3D_MODEL_BODY_KEY, ...args } }),
    [onAction],
  );

  const instances = useMemo(() => fem3dStructuralInstances(state.snapshot), [state.snapshot]);
  const node = useMemo(() => buildFem3dSceneNode(instances, state.camera, FEM3D_MODEL_BODY_KEY, FEM3D_EDITOR_CONTROLLER_ID), [instances, state.camera]);
  const debug = useMemo(
    () => JSON.stringify({ exampleId: state.exampleId, locale: state.locale, instanceCount: instances.length, nodeIds: state.snapshot.nodes.map((entry) => entry.id), omitted: femStoryOmissions([FEM3D_SOLID_MESH_OMISSION]), lastAction }),
    [state, instances, lastAction],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", minHeight: "24rem", flexDirection: "column" }}>
      <div style={{ display: "flex", flexWrap: "wrap", gap: 4, padding: 4 }}>
        <button type="button" data-testid="fem3d-load-default" onClick={() => dispatch("setActiveExample", { exampleId: FEM3D_STORY_EXAMPLE_ID })}>
          {femStoryLabel("loadExample", state.locale)}: {FEM3D_STORY_EXAMPLE_ID}
        </button>
        <button type="button" data-testid="fem3d-load-shipped-id" onClick={() => dispatch("setActiveExample", { exampleId: FEM3D_CLEARED_EXAMPLE_ID })}>
          {femStoryLabel("example", state.locale)}: {FEM3D_CLEARED_EXAMPLE_ID}
        </button>
        <button type="button" data-testid="fem3d-add-node" onClick={() => dispatch("addNode", { x: 4, y: 5, z: 8.4 })}>
          {femStoryLabel("addNode", state.locale)}
        </button>
      </div>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }} data-testid="fem3d-model-world">
        <World3dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="fem3d-model-window" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {fem3dSummaryLines(state.snapshot, state.locale).join("\n")}
      </pre>
      <pre data-testid="fem3d-model-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🏗️fem🧊️3d/Model",
  component: Fem3dModelStoryHost,
  parameters: { layout: "fullscreen" },
  argTypes: { locale: { control: "inline-radio", options: FEM_STORY_LOCALES } },
  tags: ["autodocs"],
} satisfies Meta<typeof Fem3dModelStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🧊️ The bundled fixture under its registered `"demo"` id: 16 nodes plus 16 oriented HEA200 frame prisms. */
export const DefaultExampleEnglish: Story = {
  args: { initialExampleId: FEM3D_STORY_EXAMPLE_ID, locale: "en-US" },
};

/** 🇩🇪️ The same scene with German panel labels from the supplied OS locale. */
export const DefaultExampleGerman: Story = {
  args: { initialExampleId: FEM3D_STORY_EXAMPLE_ID, locale: "de-DE" },
};

/** 🧹️ Explicit empty-document selection preserves the window navigation and OS locale. */
export const ShippedExampleId: Story = {
  args: { initialExampleId: FEM3D_CLEARED_EXAMPLE_ID, locale: "en-US" },
};
