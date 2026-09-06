// #region 🧲️Header
// 💻️ .storybook/stories/remodel/Model.stories.tsx
// Specs: Component-level coverage for remodel's `remodeling-main` window — the `model` mode's World3d viewport
// (`✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️.rs`), which `🗣️Interpreter`'s host switch resolves to
// `World3dHost` for `componentKind: "world-3d"`.
// Summary: Mounts `World3dHost` directly against the `UiComponentSceneNode` `../scene.ts` projects from the
// story-local populated document, with a story-local reducer standing in for the plugin's `setCamera`/
// `setLayerVisibility` dispatch. `World3dHost` is plain three.js/r3f with no plugin wasm, so everything here
// renders for real: four point-cloud layers (sparse, dense, recovered camera poses, GCP world positions),
// each toggleable from the layer buttons, and the camera round-tripping back through `setCamera`.
// ⚠️ `meshesJson` is `"[]"` because neither story document carries a `durableArtifacts` map for the composed
// `s.stdio.semio@v1/mesh` CHILD — the plugin renders these same documents mesh-less for the same reason. The
// mesh INSTANCE is still emitted (the Rust gates it only on `config.layers.mesh`), so the host shows its own
// missing-mesh placeholder for it; that is the runtime's behavior, faithfully reproduced.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { World3dHost } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { REMODEL_DEFAULT_CONFIG, REMODEL_EMPTY_SCENE, REMODEL_POPULATED_SCENE, type RemodelScene } from "./fixture";
import { REMODEL_EDITOR_CONTROLLER_ID, reduceRemodelStoryAction, remodelMainWorldScene, remodelWindowNode } from "./scene";

const LAYER_IDS = ["mesh", "sparse", "dense", "cameras", "gcps"] as const;

//#region StoryHost
function RemodelModelStoryHost({ document }: { readonly document: RemodelScene }): ReactElement {
  const [config, setConfig] = useState(REMODEL_DEFAULT_CONFIG);
  const onAction = useCallback((descriptor: ActionDescriptor): void => setConfig((current) => reduceRemodelStoryAction(current, descriptor)), []);
  const node = useMemo(() => remodelWindowNode("remodeling-main", document, config), [document, config]);
  const debug = useMemo(() => {
    const scene = remodelMainWorldScene(document, config);
    return JSON.stringify({
      windowKindId: "remodeling-main",
      surfaceKind: "world-3d",
      camera: config.camera,
      layers: config.layers,
      pointLayerIds: (JSON.parse(scene.pointsJson ?? "[]") as { readonly id: string }[]).map((layer) => layer.id),
      meshCount: (JSON.parse(scene.meshesJson) as unknown[]).length,
      instanceCount: (JSON.parse(scene.instancesJson) as unknown[]).length,
      meshUnresolvedReason: "no durableArtifacts map on the document — resolve_bounded_remodeling_mesh yields None",
    });
  }, [document, config]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", gap: 6, padding: 4, flexWrap: "wrap" }}>
        {LAYER_IDS.map((layer) => (
          <button key={layer} type="button" data-testid={`remodel-model-layer-${layer}`} onClick={() => onAction({ controllerId: REMODEL_EDITOR_CONTROLLER_ID, action: "setLayerVisibility", args: { layer, visible: !config.layers[layer] } })}>
            {layer}: {config.layers[layer] ? "on" : "off"}
          </button>
        ))}
      </div>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <World3dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="remodel-model-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "📸️remodel🧊️Model",
  component: RemodelModelStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof RemodelModelStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 📸️ The shared populated fixture mid-run: a two-point sparse cloud, a one-point dense cloud, one recovered camera pose and two GCP markers — toggle any of the five layers from the buttons above the viewport. */
export const PopulatedScene: Story = { args: { document: REMODEL_POPULATED_SCENE } };

/** 🌱️ The boot document (`default_remodeling_scene()`, also what the shipped `📚️examples/🎬️demo` DSL contains): no clouds, no poses, no GCPs — `pointsJson` is absent entirely and only the placeholder mesh instance remains. */
export const BootDocument: Story = { args: { document: REMODEL_EMPTY_SCENE } };
