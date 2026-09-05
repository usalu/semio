// #region 🧲️Header
// 💻️ .storybook/stories/block/5d/World.stories.tsx
// Specs: Host the framework renderer's `World3dHost` for the block5d app's `block5d-world` window — the part
// kind's 3D representation plus its grips as world vortices — driven by the REAL shipped example documents
// (`🗿️artifacts/🖐️5d/…/📚️examples/*/🖼️assets/*/🗣️.dsl.semio`) and the REAL GLB meshes they name.
// Summary: The 3D half of the pair whose 2D half is `./Board.stories.tsx` (block5d has two windows,
// `block5d-board` and `block5d-world`, so the pair is split by window). Same mesh-catalog resolution as
// `../3d/World.stories.tsx`: the document's public `/mesh/…` identity goes through `resolveMeshAsset`
// (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json`), and the GLB it names is `?url`-imported by
// `../scene.ts` so Vite emits it — the `/mesh` transport route is a `mesh-collection` playground asset and a
// GENERATED Storybook scope cannot declare `assets`. Both block5d examples reference exactly one
// representation and both resolve, so nothing is dropped here (unlike `../3d/World.stories.tsx`'s nakagin
// `1:500` representation).
// The `mesh:` line the window's Rust `render` prints (`✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs`) is
// mirrored in the overlay panel, and grips ride the world's vortex slot exactly as block3d's rim vortices do.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { World3dHost } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { BLOCK5D_STORY_EXAMPLE_IDS, admitBlockStoryRepresentations, block5dStoryStateFor, block5dWorldRenderLines, blockStoryActionArgs, buildBlock5dWorldSceneNode, reduceBlock5dStoryAction, type Block5dStoryState } from "../scene";

//#region StoryHost
const BLOCK5D_STORY_CONTROLLER_ID = "block5d-story";

type Block5dWorldRuntime = { readonly selectedIds: readonly string[]; readonly hoveredId: string | null };

/** @emoji 🖱️ Story-local mirror of `instanceMergeArg`/`componentMergeArg` — see `../3d/World.stories.tsx`'s copy. */
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

function Block5dWorldStoryHost({ initialExampleId }: { readonly initialExampleId: string }): ReactElement {
  const [state, setState] = useState<Block5dStoryState>(() => block5dStoryStateFor(initialExampleId));
  const [runtime, setRuntime] = useState<Block5dWorldRuntime>({ selectedIds: [], hoveredId: null });

  const admitted = useMemo(() => admitBlockStoryRepresentations(state.snapshot.representations), [state]);
  /** 🎯️ `World3dHost` dispatches `worldPick` with `id` as the INDEX into `instancesJson`, never the instance id. */
  const instanceIds = useMemo(() => admitted.filter((entry) => entry.resolved !== null).map((entry) => entry.id), [admitted]);

  const onAction = useCallback(
    (descriptor: ActionDescriptor): void => {
      const args = blockStoryActionArgs(descriptor.args);
      const merge = typeof args.merge === "string" ? args.merge : "replace";
      setState((current) => reduceBlock5dStoryAction(current, descriptor.action, descriptor.args));
      setRuntime((current) => {
        switch (descriptor.action) {
          case "setActiveExample":
            return { selectedIds: [], hoveredId: null };
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

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>): void => onAction({ controllerId: BLOCK5D_STORY_CONTROLLER_ID, action, args: { surfaceId: "block5d.play.world", ...args } }),
    [onAction],
  );

  const node = useMemo(() => buildBlock5dWorldSceneNode(state.snapshot, admitted, runtime.selectedIds, runtime.hoveredId), [state, admitted, runtime]);
  const lines = useMemo(() => block5dWorldRenderLines(state.snapshot), [state]);
  const debug = useMemo(
    () =>
      JSON.stringify({
        exampleId: state.exampleId,
        partKind: state.snapshot.partKind.id,
        camera: state.snapshot.camera3d,
        representations: admitted.map((entry) => ({ id: entry.id, name: entry.name, meshUrl: entry.meshUrl, rendered: entry.resolved !== null })),
        droppedRepresentations: admitted.filter((entry) => entry.resolved === null).map((entry) => ({ id: entry.id, meshUrl: entry.meshUrl, reason: "mesh identity absent from the framework mesh catalog" })),
        gripCount: state.snapshot.grips.length,
        selection: runtime.selectedIds,
        hovered: runtime.hoveredId,
      }),
    [state, admitted, runtime],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", flexWrap: "wrap", gap: 4, padding: 4 }}>
        {BLOCK5D_STORY_EXAMPLE_IDS.map((exampleId) => (
          <button key={exampleId} type="button" data-testid={`block5d-world-example-${exampleId}`} onClick={() => dispatch("setActiveExample", { exampleId })} disabled={exampleId === state.exampleId}>
            {exampleId}
          </button>
        ))}
        <button type="button" data-testid="block5d-world-add-grip" onClick={() => dispatch("addGrip", {})}>
          Add grip
        </button>
        <button type="button" data-testid="block5d-world-remove-grip" onClick={() => dispatch("removeGrip", {})}>
          Remove grip
        </button>
      </div>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <World3dHost node={node} onAction={onAction} />
        <ol data-testid="block5d-world-window" style={{ position: "absolute", top: 4, left: 4, margin: 0, padding: "2px 6px", listStyle: "none", fontFamily: "var(--font-mono, monospace)", fontSize: 11, pointerEvents: "none" }}>
          {lines.map((line) => (
            <li key={line} style={{ whiteSpace: "pre" }}>
              {line}
            </li>
          ))}
        </ol>
      </div>
      <pre data-testid="block5d-world-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🧱️block🖐️5d/World",
  component: Block5dWorldStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Block5dWorldStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🎬️ `🌲️hexagonal-cut-concrete-forest-left` — the `default` representation's real GLB plus one `b-l` grip at `@4.05,4.68,3`. */
export const HexagonalCutConcreteForestLeft: Story = {
  args: {
    initialExampleId: "hexagonal-cut-concrete-forest-left",
  },
};

/** 🏢️ `🏢️nakagin-capsule` (`Capsule J`) — the `Full Detail` representation's real GLB from the metabolism catalog plus the single `door` grip at `@0,-1.6,1.2`. */
export const NakaginCapsule: Story = {
  args: {
    initialExampleId: "nakagin-capsule",
  },
};
