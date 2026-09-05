// #region 🧲️Header
// 💻️ .storybook/stories/block/3d/World.stories.tsx
// Specs: Host the framework renderer's `World3dHost` for the block3d app's `block3d-world` window, driven by
// the REAL shipped example documents (`🗿️artifacts/🧊️3d/…/📚️examples/*/🖼️assets/*/🗣️.dsl.semio`) and the REAL GLB
// meshes their `representations` name.
// Summary: Mounts the host directly against a `UiComponentSceneNode` (`componentKind: "world-3d"`) projected
// the way `🧊️3d/…/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs` builds it — one mesh + one instance per
// representation at the document origin, every rim vortex at its document position/direction/radius coloured
// by its `vortex-kind-extra` row — with a story-local reducer for the pick/hover/camera subset the host
// dispatches, mirroring `../puzzle/3d/World.stories.tsx`.
// Unlike puzzle's 3D story (which renders `World3dHost`'s neutral placeholder box because its fixtures' GLBs
// are not in this checkout), block's meshes DO exist and are resolved through the framework mesh catalog:
// `resolveMeshAsset` (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json`) maps the document's public
// `/mesh/…` identity to its source GLB, which `../scene.ts` `?url`-imports so Vite emits it — necessary
// because the `/mesh` transport route is a `mesh-collection` playground asset and a GENERATED Storybook scope
// cannot declare `assets`. `meshAssetTransportUrl` (which `World3dHost` calls on every mesh url) passes a
// non-`/mesh/` url straight through, so the host loads the real GLB.
// ⚠️ The nakagin `Capsule J` document's SECOND representation, `r1` `"1:500"`, names
// `/mesh/capsule_J.1to500.glb` — an identity NO mesh catalog in the repo declares (verified against both
// `🥽️mesh/📇️catalog.json` and the `🌱️metabolism/🎨️representation` collection it composes). It is therefore
// DROPPED from the scene rather than rendered mesh-less, and the drop is reported in the debug readout's
// `droppedRepresentations`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { World3dHost } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { admitBlockStoryRepresentations, block3dStorySnapshotFor, blockStoryActionArgs, buildBlock3dWorldSceneNode } from "../scene";

//#region PluginEmulator
type Block3dStoryRuntime = { readonly selectedIds: readonly string[]; readonly hoveredId: string | null };

/** @emoji 🖱️ Story-local mirror of `instanceMergeArg`/`componentMergeArg` — applies a `worldPick`/`worldSelect`/`worldVortexSelect` merge mode to the current selection (verbatim the helper `../puzzle/3d/World.stories.tsx` uses). */
function applyStoryWorldMerge(current: readonly string[], id: string, merge: string): string[] {
  const set = new Set(current);
  if (merge === "replace") return [id];
  if (merge === "add") {
    set.add(id);
    return [...set];
  }
  if (merge === "remove") {
    set.delete(id);
    return [...set];
  }
  if (set.has(id)) set.delete(id);
  else set.add(id);
  return [...set];
}
//#endregion PluginEmulator

//#region StoryHost
function Block3dWorldStoryHost({ exampleId }: { readonly exampleId: string }): ReactElement {
  const snapshot = useMemo(() => block3dStorySnapshotFor(exampleId), [exampleId]);
  const admitted = useMemo(() => admitBlockStoryRepresentations(snapshot.representations), [snapshot]);
  const [runtime, setRuntime] = useState<Block3dStoryRuntime>({ selectedIds: [], hoveredId: null });

  /** 🎯️ `World3dHost` dispatches `worldPick` with `id` as the INDEX into the scene's `instancesJson`, never the instance id — so the story resolves it against the same admitted-representation order the scene was built from. */
  const instanceIds = useMemo(() => admitted.filter((entry) => entry.resolved !== null).map((entry) => entry.id), [admitted]);

  const onAction = useCallback(
    (descriptor: ActionDescriptor): void => {
      const args = blockStoryActionArgs(descriptor.args);
      const merge = typeof args.merge === "string" ? args.merge : "replace";
      setRuntime((current) => {
        switch (descriptor.action) {
          case "worldPick": {
            const id = typeof args.id === "number" ? instanceIds[args.id] : typeof args.id === "string" ? args.id : undefined;
            return id === undefined ? { ...current, selectedIds: [] } : { ...current, selectedIds: applyStoryWorldMerge(current.selectedIds, id, merge) };
          }
          case "worldSelect": {
            const ids = Array.isArray(args.ids) ? (args.ids as string[]) : [];
            if (merge === "replace") return { ...current, selectedIds: ids };
            return { ...current, selectedIds: ids.reduce<readonly string[]>((selection, id) => applyStoryWorldMerge(selection, id, merge), current.selectedIds) };
          }
          case "worldVortexSelect": {
            const fullId = typeof args.fullId === "string" ? args.fullId : undefined;
            return fullId === undefined ? current : { ...current, selectedIds: applyStoryWorldMerge(current.selectedIds, fullId, merge) };
          }
          case "setHover":
            return { ...current, hoveredId: typeof args.objectId === "string" ? args.objectId : null };
          case "worldVortexHover":
            return { ...current, hoveredId: typeof args.fullId === "string" ? args.fullId : null };
          default:
            return current;
        }
      });
    },
    [instanceIds],
  );

  const node = useMemo(() => buildBlock3dWorldSceneNode(snapshot, admitted, runtime.selectedIds, runtime.hoveredId), [snapshot, admitted, runtime]);
  const debug = useMemo(
    () =>
      JSON.stringify({
        exampleId,
        objectKind: snapshot.objectKind.id,
        camera: snapshot.camera3d,
        representations: admitted.map((entry) => ({ id: entry.id, name: entry.name, meshUrl: entry.meshUrl, rendered: entry.resolved !== null })),
        droppedRepresentations: admitted.filter((entry) => entry.resolved === null).map((entry) => ({ id: entry.id, meshUrl: entry.meshUrl, reason: "mesh identity absent from the framework mesh catalog" })),
        vortexKindIds: snapshot.vortexKinds.map((kind) => kind.id),
        vortexCount: snapshot.vortices.length,
        selection: runtime.selectedIds,
        hovered: runtime.hoveredId,
      }),
    [exampleId, snapshot, admitted, runtime],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <World3dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="block3d-world-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🧱️block🧊️3d",
  component: Block3dWorldStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Block3dWorldStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🎬️ `🌲️hexagonal-cut-concrete-forest-left` — one `default` representation whose `/mesh/🧊️hexagonal-cut-concrete-forest-left.glb` resolves to a real GLB, plus 11 rim vortices across 6 vortex kinds. */
export const HexagonalCutConcreteForestLeft: Story = {
  args: {
    exampleId: "hexagonal-cut-concrete-forest-left",
  },
};

/** 🏢️ `🏢️nakagin-capsule` (`Capsule J`) — the `Full Detail` representation renders from the metabolism catalog's real GLB; the `1:500` one is dropped because `/mesh/capsule_J.1to500.glb` is in no catalog (see `droppedRepresentations` in the debug readout). */
export const NakaginCapsule: Story = {
  args: {
    exampleId: "nakagin-capsule",
  },
};
