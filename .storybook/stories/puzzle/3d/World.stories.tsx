// #region 🧲Header
// 💻 .storybook/story/puzzle/3d/World.stories.tsx
// Specs: Host the framework renderer's `World3dHost` for Storybook + Playwright selection/camera checks, driven by the *real* puzzle-3d example fixtures.
// Summary: Mounts the host directly against a `UiComponentSceneNode`; a story-local reducer emulates the subset of `puzzle3d-play`'s (`puzzle/plugin/rs/lib.rs`'s `d3` module, `handle_action`) object/vortex pick + camera + delete/duplicate actions the story exercises, so the controlled scene ⇄ session loop round-trips without a running dev server — mirrors `../puzzle/2d/Board.stories.tsx`'s pattern. Fixture data comes from the real `puzzle/3d/example/*.puzzle3d` DSL-text fixtures (`Puzzle3dProjection`'s `dsl::DslDocument` grammar) — raw-imported as text and parsed via `@semio-tech/puzzle-3d-rs`'s `puzzle3dParseDslJson` wasm export (the same `parse_dsl` Rust uses, reused as the single source of truth instead of duplicating the DSL grammar in TypeScript).
// Real GLB mesh assets referenced by `meshUrl` in these fixtures aren't part of this Storybook scope's asset pipeline (no `mesh-collection` route is registered for `puzzle/3d`, and the GLBs themselves don't exist in this checkout) — object instances are therefore built *without* a mesh `url`, so `World3dHost` renders its built-in neutral placeholder box per object instead of attempting (and failing) a GLTF fetch. Reference-plane images do exist on disk, so those load for real via a Vite asset import.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useEffect, useMemo, useState, type ReactElement } from "react";

import { World3dHost } from "../../../../framework/product/os/module/renderer/js/react/index.tsx";
import type { ActionDescriptor, UiComponentSceneNode } from "../../../../framework/product/os/module/renderer/js/react/index.tsx";

import concreteForestFixtureDsl from "../../../../s/plugin/puzzle/app/3d/example/concrete-forest.puzzle3d?raw";
import nakaginCapsuleTowerFixtureDsl from "../../../../s/plugin/puzzle/app/3d/example/nakagin-capsule-tower.puzzle3d?raw";

import abbauAufbauReferenceUrl from "../../../../framework/product/os/module/infinite/fixture/abbau-aufbau-masterarbeit-grundriss.jpg";
import rathausAhlenReferenceUrl from "../../../../framework/product/os/module/infinite/fixture/rathaus-ahlen-grundriss.png";

//#region StoryTypes
type Vec3 = readonly [number, number, number];
type Quat = readonly [number, number, number, number];

type StoryWorld3dVortex = {
  readonly id: string;
  readonly vortexKind?: string;
  readonly label?: string;
  readonly position: Vec3;
  readonly direction?: Vec3;
  readonly radius?: number;
};

type StoryWorld3dObject = {
  readonly id: string;
  readonly label?: string;
  readonly objectKind?: string;
  readonly meshUrl?: string;
  readonly origin: Vec3;
  readonly orientation?: Quat;
  readonly vortices?: readonly StoryWorld3dVortex[];
};

type StoryWorld3dReference = {
  readonly id: string;
  readonly source: { readonly url: string; readonly mediaKind: string };
  readonly origin: Vec3;
  readonly widthWorld?: number;
  readonly locked?: boolean;
  readonly hidden?: boolean;
};

type StoryWorld3dCamera = { readonly position: Vec3; readonly target: Vec3; readonly zoom: number };

type StoryWorld3dFixture = {
  readonly schema: string;
  readonly camera: StoryWorld3dCamera;
  readonly objects: readonly StoryWorld3dObject[];
  readonly references?: readonly StoryWorld3dReference[];
};

type StoryWorld3dRuntime = {
  readonly selectedIds: readonly string[];
  readonly hoveredId: string | null;
  readonly activeUtility: string;
};

type StoryWorld3dState = { readonly fixture: StoryWorld3dFixture; readonly runtime: StoryWorld3dRuntime };
//#endregion StoryTypes

//#region ReferenceAssetOverrides
/** @emoji 🖼️ Fixture `references[].source.url` values point at the `/infinite-fixture/*` dev-only static route, which isn't registered for the `puzzle/3d` Storybook scope — remap the two known fixture URLs to real Vite-imported asset URLs so the reference planes actually load instead of 404ing (silently, per `WorldReferenceLayer`'s catch — see `framework/os/kernel/infinite/world/r3f/index.tsx`). */
const STORY_REFERENCE_URL_OVERRIDES: Record<string, string> = {
  "/infinite-fixture/abbau-aufbau-masterarbeit-grundriss.jpg": abbauAufbauReferenceUrl,
  "/infinite-fixture/rathaus-ahlen-grundriss.png": rathausAhlenReferenceUrl,
};
//#endregion ReferenceAssetOverrides

//#region WasmFixtureLoader
/** @emoji 🧵 Lazily loads+inits `@semio-tech/puzzle-3d-rs`'s wasm module once (mirrors `framework/product/os/module/renderer/js/react/index.tsx`'s `createEngineSession` caching), then exposes `parse_dsl`'d fixture JSON via the crate's `puzzle3dParseDslJson` free export. */
type Puzzle3dWasmModule = { readonly default: (input?: unknown) => Promise<unknown>; readonly puzzle3dParseDslJson: (dslText: string) => string };
let puzzle3dWasmModulePromise: Promise<Puzzle3dWasmModule> | null = null;
function loadPuzzle3dWasm(): Promise<Puzzle3dWasmModule> {
  if (!puzzle3dWasmModulePromise) {
    puzzle3dWasmModulePromise = import("@semio-tech/puzzle-3d-rs/pkg/puzzle_3d.js").then(async (mod) => {
      await (mod as unknown as Puzzle3dWasmModule).default();
      return mod as unknown as Puzzle3dWasmModule;
    });
  }
  return puzzle3dWasmModulePromise;
}

async function parsePuzzle3dFixtureDsl(dslText: string): Promise<StoryWorld3dFixture> {
  const mod = await loadPuzzle3dWasm();
  return JSON.parse(mod.puzzle3dParseDslJson(dslText)) as StoryWorld3dFixture;
}
//#endregion WasmFixtureLoader

//#region PluginEmulator
const STORY_DEFAULT_RUNTIME: StoryWorld3dRuntime = {
  selectedIds: [],
  hoveredId: null,
  activeUtility: "select",
};

/** @emoji 🖱️ Story-local mirror of `instanceMergeArg`/`componentMergeArg` (`framework/product/os/module/renderer/js/react/index.tsx`) — applies a `worldPick`/`worldSelect`/`worldVortexSelect` merge mode to the current selection. */
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

/** @emoji 🧩 Story-local mirror of a subset of `d3::Puzzle3dPlayApp::handle_action` (`puzzle/plugin/rs/lib.rs`) — enough for the story to click/hover/orbit/delete/duplicate against the real fixtures. */
function reduceStoryWorld3dAction(state: StoryWorld3dState, action: string, args: Record<string, unknown> | undefined): StoryWorld3dState {
  const { fixture, runtime } = state;
  switch (action) {
    case "worldPick": {
      const index = Number(args?.id);
      const merge = typeof args?.merge === "string" ? args.merge : "replace";
      const target = fixture.objects[index];
      if (!target) return state;
      return { fixture, runtime: { ...runtime, selectedIds: applyStoryWorldMerge(runtime.selectedIds, target.id, merge) } };
    }
    case "worldSelect": {
      const ids = Array.isArray(args?.ids) ? (args.ids as string[]) : [];
      const merge = typeof args?.merge === "string" ? args.merge : "replace";
      if (merge === "replace") return { fixture, runtime: { ...runtime, selectedIds: ids } };
      let selectedIds = runtime.selectedIds;
      for (const id of ids) selectedIds = applyStoryWorldMerge(selectedIds, id, merge);
      return { fixture, runtime: { ...runtime, selectedIds } };
    }
    case "worldVortexSelect": {
      const fullId = args?.fullId;
      const merge = typeof args?.merge === "string" ? args.merge : "replace";
      if (typeof fullId !== "string") return state;
      return { fixture, runtime: { ...runtime, selectedIds: applyStoryWorldMerge(runtime.selectedIds, fullId, merge) } };
    }
    case "setHover": {
      const objectId = args?.objectId;
      return { fixture, runtime: { ...runtime, hoveredId: typeof objectId === "string" ? objectId : null } };
    }
    case "worldVortexHover": {
      const fullId = args?.fullId;
      return { fixture, runtime: { ...runtime, hoveredId: typeof fullId === "string" ? fullId : null } };
    }
    case "setCamera": {
      const camera = args?.camera as StoryWorld3dCamera | undefined;
      return camera ? { fixture: { ...fixture, camera }, runtime } : state;
    }
    case "setActiveUtility": {
      const utilityId = args?.utilityId;
      return { fixture, runtime: { ...runtime, activeUtility: typeof utilityId === "string" ? utilityId : "select" } };
    }
    case "selectSameKindSelection": {
      const first = fixture.objects.find((object) => object.id === runtime.selectedIds[0]);
      if (!first?.objectKind) return state;
      const ids = fixture.objects.filter((object) => object.objectKind === first.objectKind).map((object) => object.id);
      return { fixture, runtime: { ...runtime, selectedIds: ids } };
    }
    case "deleteSelection": {
      const selected = new Set(runtime.selectedIds);
      return {
        fixture: { ...fixture, objects: fixture.objects.filter((object) => !selected.has(object.id)) },
        runtime: { ...runtime, selectedIds: [], hoveredId: null },
      };
    }
    case "duplicateSelection": {
      const selected = new Set(runtime.selectedIds);
      const clones = fixture.objects
        .filter((object) => selected.has(object.id))
        .map((object) => ({ ...object, id: `${object.id}-copy`, origin: [object.origin[0] + 5, object.origin[1] + 5, object.origin[2]] as Vec3, vortices: undefined }));
      if (clones.length === 0) return state;
      return { fixture: { ...fixture, objects: [...fixture.objects, ...clones] }, runtime: { ...runtime, selectedIds: clones.map((clone) => clone.id) } };
    }
    default:
      return state;
  }
}
//#endregion PluginEmulator

//#region SceneNode
function buildStoryWorld3dSceneNode(state: StoryWorld3dState): UiComponentSceneNode {
  const { fixture, runtime } = state;
  const selectedIds = new Set(runtime.selectedIds);

  // 📦 No `url`/`data` on purpose (see header docstring) — falls back to `World3dHost`'s neutral placeholder box per instance.
  const meshes = fixture.objects.map((object) => ({ id: object.id }));

  const instances = fixture.objects.map((object) => ({
    id: object.id,
    meshId: object.id,
    position: object.origin,
    rotation: object.orientation,
    selected: selectedIds.has(object.id),
    hovered: runtime.hoveredId === object.id,
  }));

  const vortices = fixture.objects.flatMap((object) =>
    (object.vortices ?? []).map((vortex) => ({
      fullId: vortex.id,
      objectId: object.id,
      vortexKind: vortex.vortexKind,
      position: vortex.position,
      direction: vortex.direction,
      radius: vortex.radius,
      selected: selectedIds.has(vortex.id),
      hovered: runtime.hoveredId === vortex.id,
    })),
  );

  const references = (fixture.references ?? [])
    .filter((reference) => !reference.hidden)
    .map((reference) => ({
      id: reference.id,
      url: STORY_REFERENCE_URL_OVERRIDES[reference.source.url] ?? reference.source.url,
      origin: reference.origin,
      widthWorld: reference.widthWorld,
      locked: reference.locked,
    }));

  return {
    type: "componentScene",
    surfaceId: "puzzle3d.story.overview",
    controllerId: "puzzle3d-story",
    componentKind: "world-3d",
    world3d: {
      cameraJson: JSON.stringify(fixture.camera),
      meshesJson: JSON.stringify(meshes),
      instancesJson: JSON.stringify(instances),
      selectionJson: JSON.stringify({ ids: runtime.selectedIds, selectionMode: "object" }),
      vorticesJson: JSON.stringify(vortices),
      referencesJson: JSON.stringify(references),
      interactionJson: JSON.stringify({ activeUtility: runtime.activeUtility }),
    },
  };
}
//#endregion SceneNode

//#region StoryHost
function World3dStoryHost({ fixtureDsl }: { readonly fixtureDsl: string }): ReactElement {
  const [state, setState] = useState<StoryWorld3dState | null>(null);

  useEffect(() => {
    let cancelled = false;
    parsePuzzle3dFixtureDsl(fixtureDsl).then((fixture) => {
      if (!cancelled) setState({ fixture, runtime: STORY_DEFAULT_RUNTIME });
    });
    return () => {
      cancelled = true;
    };
  }, [fixtureDsl]);

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setState((current) => (current ? reduceStoryWorld3dAction(current, descriptor.action, descriptor.args) : current));
  }, []);

  const node = useMemo(() => (state ? buildStoryWorld3dSceneNode(state) : null), [state]);
  const debug = useMemo(
    () => (state ? JSON.stringify({ selection: state.runtime.selectedIds, camera: state.fixture.camera, objectCount: state.fixture.objects.length }) : "loading"),
    [state],
  );

  if (!state || !node) {
    return <div data-testid="puzzle3d-world-loading">Loading fixture…</div>;
  }

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <World3dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="puzzle3d-world-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🧩puzzle🧊3d",
  component: World3dStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof World3dStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🌲 The real Concrete Forest 3D world fixture (`puzzle/3d/example/concrete-forest.puzzle3d`) — 1 object, 2 locked/hidden reference planes. */
export const ConcreteForest: Story = {
  args: {
    fixtureDsl: concreteForestFixtureDsl,
  },
};

/** 🏯 The real Nakagin Capsule Tower 3D world fixture (`puzzle/3d/example/nakagin-capsule-tower.puzzle3d`) — 180 objects. */
export const NakaginCapsuleTower: Story = {
  args: {
    fixtureDsl: nakaginCapsuleTowerFixtureDsl,
  },
};
