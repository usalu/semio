// #region 🧲Header
// 💻 .storybook/story/puzzle/5d/Timeline.stories.tsx
// Specs: Compose `World3dHost` + `GraphTimelineHost` against the real puzzle-5d example fixtures — "5D" (3 spatial + assembly-order + fastener graph) rendered as a 3D world you can scrub through an assembly history.
// Summary: `puzzle/5d`'s real fixture schema (`puzzle/5d/example/*.puzzle5d`) has no persisted checkpoint history (the plugin's `d5` module doesn't wire `graph_timeline` yet — grepped `puzzle/plugin/rs/lib.rs`, no `HistoryColumn`/`checkoutCheckpoint` hits under `mod d5`), so this story *synthesizes* one checkpoint per fixture `parts[]` entry (assembly order = array order, newest first per `HistoryColumn`'s docstring in `framework/ui/js/react/index.tsx`) and lets `GraphTimelineHost`'s `checkoutCheckpoint` action scrub how many parts `World3dHost` reveals — same story-local-reducer pattern as `../3d/World.stories.tsx` and `../2d/Board.stories.tsx`. Fixture data comes from the real `.puzzle5d` DSL-text fixtures (`Puzzle5dProjection`'s `dsl::DslDocument` grammar) — raw-imported as text and parsed via `@semio-tech/puzzle-5d-rs`'s `puzzle5dParseDslJson` wasm export (the same `parse_dsl` Rust uses, reused as the single source of truth instead of duplicating the DSL grammar in TypeScript).
// Mesh/reference-asset caveats are identical to `../3d/World.stories.tsx`: no `mesh-collection` route for this scope and the referenced GLBs don't exist on disk, so parts render as `World3dHost`'s neutral placeholder box.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import type { HistoryColumn } from "@semio-tech/ui-react";
import { useCallback, useEffect, useMemo, useState, type ReactElement } from "react";

import { GraphTimelineHost, World3dHost } from "../../../../framework/os/renderer/js/react/index.tsx";
import type { ActionDescriptor, UiComponentSceneNode } from "../../../../framework/os/renderer/js/react/index.tsx";

import concreteForestFixtureDsl from "../../../../s/plugin/puzzle/5d/example/concrete-forest.puzzle5d?raw";
import nakaginCapsuleTowerFixtureDsl from "../../../../s/plugin/puzzle/5d/example/nakagin-capsule-tower.puzzle5d?raw";

//#region StoryTypes
type Vec3 = readonly [number, number, number];
type Quat = readonly [number, number, number, number];

type StoryPuzzle5dGrip = {
  readonly id: string;
  readonly gripKind?: string;
  readonly "3d": { readonly position: Vec3; readonly direction?: Vec3; readonly radius?: number; readonly label?: string };
};

type StoryPuzzle5dPart = {
  readonly id: string;
  readonly partKind?: string;
  readonly grips?: readonly StoryPuzzle5dGrip[];
  readonly "3d": { readonly origin: Vec3; readonly orientation?: Quat; readonly meshUrl?: string; readonly label?: string };
};

type StoryPuzzle5dCamera3d = { readonly position: Vec3; readonly target: Vec3; readonly zoom: number };

type StoryPuzzle5dFixture = {
  readonly schema: string;
  readonly label?: string;
  readonly camera3d: StoryPuzzle5dCamera3d;
  readonly parts: readonly StoryPuzzle5dPart[];
};

type StoryPuzzle5dRuntime = {
  readonly revealCount: number;
  readonly selectedIds: readonly string[];
  readonly hoveredId: string | null;
};

type StoryPuzzle5dState = { readonly fixture: StoryPuzzle5dFixture; readonly runtime: StoryPuzzle5dRuntime };
//#endregion StoryTypes

//#region WasmFixtureLoader
/** @emoji 🧵 Lazily loads+inits `@semio-tech/puzzle-5d-rs`'s wasm module once (mirrors `framework/os/renderer/js/react/index.tsx`'s `createEngineSession` caching), then exposes `parse_dsl`'d fixture JSON via the crate's `puzzle5dParseDslJson` free export. */
type Puzzle5dWasmModule = { readonly default: (input?: unknown) => Promise<unknown>; readonly puzzle5dParseDslJson: (dslText: string) => string };
let puzzle5dWasmModulePromise: Promise<Puzzle5dWasmModule> | null = null;
function loadPuzzle5dWasm(): Promise<Puzzle5dWasmModule> {
  if (!puzzle5dWasmModulePromise) {
    puzzle5dWasmModulePromise = import("@semio-tech/puzzle-5d-rs").then(async (mod) => {
      await (mod as unknown as Puzzle5dWasmModule).default();
      return mod as unknown as Puzzle5dWasmModule;
    });
  }
  return puzzle5dWasmModulePromise;
}

async function parsePuzzle5dFixtureDsl(dslText: string): Promise<StoryPuzzle5dFixture> {
  const mod = await loadPuzzle5dWasm();
  return JSON.parse(mod.puzzle5dParseDslJson(dslText)) as StoryPuzzle5dFixture;
}
//#endregion WasmFixtureLoader

//#region HistorySynthesis
/** @emoji 🗄️ Synthesizes one linear `HistoryColumn` per fixture part (see header docstring) — newest (last-assembled) part first. */
function historyColumnsFromParts(parts: readonly StoryPuzzle5dPart[]): readonly HistoryColumn[] {
  return [...parts]
    .map((part, index) => ({
      checkpointId: part.id,
      timestamp: String(index + 1),
      labels: index === parts.length - 1 ? ["latest"] : [],
      authors: [],
      parentCheckpointId: index > 0 ? parts[index - 1]!.id : undefined,
      description: part.partKind ?? part.id,
      lane: 0,
      alternativeIds: [],
    }))
    .reverse();
}
//#endregion HistorySynthesis

//#region PluginEmulator
/** @emoji 🖱️ Story-local mirror of `instanceMergeArg` (`framework/os/renderer/js/react/index.tsx`) — see `../3d/World.stories.tsx`'s copy. */
function applyStoryMerge(current: readonly string[], id: string, merge: string): string[] {
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

/** @emoji 🧩 Story-local reducer: `checkoutCheckpoint` (from `GraphTimelineHost`) scrubs `revealCount`; `worldPick`/`setHover`/`setCamera` (from `World3dHost`) mirror the same subset `../3d/World.stories.tsx` implements, resolved against the *currently revealed* parts slice. */
function reduceStoryPuzzle5dAction(state: StoryPuzzle5dState, action: string, args: Record<string, unknown> | undefined): StoryPuzzle5dState {
  const { fixture, runtime } = state;
  const revealed = fixture.parts.slice(0, runtime.revealCount);
  switch (action) {
    case "checkoutCheckpoint": {
      const checkpointId = args?.checkpointId;
      if (typeof checkpointId !== "string") return state;
      const index = fixture.parts.findIndex((part) => part.id === checkpointId);
      if (index < 0) return state;
      return { fixture, runtime: { ...runtime, revealCount: index + 1, selectedIds: [], hoveredId: null } };
    }
    case "worldPick": {
      const index = Number(args?.id);
      const merge = typeof args?.merge === "string" ? args.merge : "replace";
      const target = revealed[index];
      if (!target) return state;
      return { fixture, runtime: { ...runtime, selectedIds: applyStoryMerge(runtime.selectedIds, target.id, merge) } };
    }
    case "setHover": {
      const objectId = args?.objectId;
      return { fixture, runtime: { ...runtime, hoveredId: typeof objectId === "string" ? objectId : null } };
    }
    case "setCamera":
      // 📷 Orbit-drag camera updates are kept live by `World3dHost`'s own Three.js controls; the fixture's persisted `camera3d` is intentionally left untouched so re-scrubbing the timeline doesn't jump the view.
      return state;
    default:
      return state;
  }
}
//#endregion PluginEmulator

//#region SceneNode
function buildStoryWorld3dNode(fixture: StoryPuzzle5dFixture, runtime: StoryPuzzle5dRuntime): UiComponentSceneNode {
  const revealed = fixture.parts.slice(0, runtime.revealCount);
  const selectedIds = new Set(runtime.selectedIds);

  // 📦 No mesh `url`/`data` on purpose — see `../3d/World.stories.tsx`'s header docstring (no `mesh-collection` route for this scope, and the GLBs don't exist on disk either).
  const meshes = revealed.map((part) => ({ id: part.id }));
  const instances = revealed.map((part) => ({
    id: part.id,
    meshId: part.id,
    position: part["3d"].origin,
    rotation: part["3d"].orientation,
    selected: selectedIds.has(part.id),
    hovered: runtime.hoveredId === part.id,
  }));
  const vortices = revealed.flatMap((part) =>
    (part.grips ?? []).map((grip) => ({
      fullId: `${part.id}:${grip.id}`,
      objectId: part.id,
      vortexKind: grip.gripKind,
      position: grip["3d"].position,
      direction: grip["3d"].direction,
      radius: grip["3d"].radius,
    })),
  );

  return {
    type: "componentScene",
    surfaceId: "puzzle5d.story.world",
    controllerId: "puzzle5d-story",
    componentKind: "world-3d",
    world3d: {
      cameraJson: JSON.stringify(fixture.camera3d),
      meshesJson: JSON.stringify(meshes),
      instancesJson: JSON.stringify(instances),
      selectionJson: JSON.stringify({ ids: runtime.selectedIds, selectionMode: "object" }),
      vorticesJson: JSON.stringify(vortices),
    },
  };
}

function buildStoryGraphTimelineNode(fixture: StoryPuzzle5dFixture): UiComponentSceneNode {
  return {
    type: "componentScene",
    surfaceId: "puzzle5d.story.timeline",
    controllerId: "puzzle5d-story",
    componentKind: "graph-timeline",
    graphTimeline: {
      columnsJson: JSON.stringify(historyColumnsFromParts(fixture.parts)),
    },
  };
}
//#endregion SceneNode

//#region StoryHost
function Puzzle5dTimelineStoryHost({ fixtureDsl }: { readonly fixtureDsl: string }): ReactElement {
  const [state, setState] = useState<StoryPuzzle5dState | null>(null);

  useEffect(() => {
    let cancelled = false;
    parsePuzzle5dFixtureDsl(fixtureDsl).then((fixture) => {
      if (!cancelled) setState({ fixture, runtime: { revealCount: fixture.parts.length, selectedIds: [], hoveredId: null } });
    });
    return () => {
      cancelled = true;
    };
  }, [fixtureDsl]);

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setState((current) => (current ? reduceStoryPuzzle5dAction(current, descriptor.action, descriptor.args) : current));
  }, []);

  const worldNode = useMemo(() => (state ? buildStoryWorld3dNode(state.fixture, state.runtime) : null), [state]);
  const timelineNode = useMemo(() => (state ? buildStoryGraphTimelineNode(state.fixture) : null), [state]);
  const debug = useMemo(
    () => (state ? JSON.stringify({ revealCount: state.runtime.revealCount, partCount: state.fixture.parts.length, selection: state.runtime.selectedIds }) : "loading"),
    [state],
  );

  if (!state || !worldNode || !timelineNode) {
    return <div data-testid="puzzle5d-timeline-loading">Loading fixture…</div>;
  }

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", display: "flex", flex: "1 1 auto", minHeight: 0 }}>
        <div style={{ position: "relative", flex: "2 1 auto", minWidth: 0 }}>
          <World3dHost node={worldNode} onAction={onAction} />
        </div>
        <div style={{ position: "relative", flex: "1 1 260px", minWidth: 220, borderLeft: "1px solid var(--border, #3333)", overflow: "auto" }}>
          <GraphTimelineHost node={timelineNode} onAction={onAction} />
        </div>
      </div>
      <pre data-testid="puzzle5d-timeline-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🧩puzzle🕐5d",
  component: Puzzle5dTimelineStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Puzzle5dTimelineStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🌲 The real Concrete Forest 5D fixture (`puzzle/5d/example/concrete-forest.puzzle5d`) — 1 part, so a single-checkpoint timeline. */
export const ConcreteForest: Story = {
  args: {
    fixtureDsl: concreteForestFixtureDsl,
  },
};

/** 🏯 The real Nakagin Capsule Tower 5D fixture (`puzzle/5d/example/nakagin-capsule-tower.puzzle5d`) — 180 parts; scrub `GraphTimelineHost`'s checkpoints to watch `World3dHost` reassemble the tower. */
export const NakaginCapsuleTower: Story = {
  args: {
    fixtureDsl: nakaginCapsuleTowerFixtureDsl,
  },
};
