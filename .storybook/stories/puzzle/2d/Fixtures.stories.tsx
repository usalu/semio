// #region 🧲Header
// 💻 .storybook/story/puzzle/2d/Fixtures.stories.tsx
// Specs: Host the framework renderer's `Board2dHost` against the *real* puzzle-2d example fixtures (not hand-authored story data).
// Summary: Same zero-WASM pattern as `./Board.stories.tsx` (a story-local reducer emulating `apply_board_events_from_json`), but the fixture JSON is imported straight from `puzzle/2d/example/*.json` — this file only proves those real fixtures round-trip through the host; the interaction-mechanics coverage (lasso/brush/lod) lives in `Board.stories.tsx`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Board2dHost } from "../../../../framework/renderer/react/index.tsx";
import type { ActionDescriptor, UiComponentSceneNode } from "../../../../framework/renderer/react/index.tsx";

import concreteForestFixture from "../../../../puzzle/2d/example/concrete-forest.2d.json";
import nakaginCapsuleTowerFixture from "../../../../puzzle/2d/example/nakagin-capsule-tower.2d.json";

//#region StoryTypes
type StoryPuzzle2dEntity = Record<string, unknown> & { readonly id: string };

type StoryPuzzle2dFixture = {
  readonly schema: string;
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly nodes: readonly StoryPuzzle2dEntity[];
  readonly edges: readonly StoryPuzzle2dEntity[];
  readonly meta?: { readonly kindCatalogs?: unknown; readonly kindCompatibility?: unknown };
};

type StoryPuzzle2dRuntime = {
  readonly selectedIds: readonly string[];
  readonly activeUtility: string;
  readonly selectionMethod: string;
  readonly gridSnapEnabled: boolean;
  readonly gridFactor: number;
  readonly suggestionOffset: number;
  readonly nodeKindWeights: Record<string, number>;
  readonly handleKindWeights: Record<string, number>;
  readonly lodMode: string;
};

type StoryPuzzle2dState = { readonly fixture: StoryPuzzle2dFixture; readonly runtime: StoryPuzzle2dRuntime };
//#endregion StoryTypes

//#region PluginEmulator
const STORY_DEFAULT_RUNTIME: StoryPuzzle2dRuntime = {
  selectedIds: [],
  activeUtility: "select",
  selectionMethod: "rectangle",
  gridSnapEnabled: false,
  gridFactor: 1,
  suggestionOffset: 0,
  nodeKindWeights: {},
  handleKindWeights: {},
  lodMode: "automatic",
};

/** @emoji 📬 Story-local mirror of `apply_board_events_from_json` in `puzzle/plugin/rs/d2/mod.rs` (see `./Board.stories.tsx`'s copy) — only the event kinds the real fixtures need to demonstrate (camera pan/zoom, select, drag, delete). */
function applyStoryBoardEvents(state: StoryPuzzle2dState, eventsJson: string): StoryPuzzle2dState {
  let events: readonly { readonly name: string; readonly payload?: Record<string, unknown> }[] = [];
  try {
    events = JSON.parse(eventsJson) as typeof events;
  } catch {
    return state;
  }
  let { fixture, runtime } = state;
  for (const event of events) {
    const payload = event.payload ?? {};
    switch (event.name) {
      case "camera": {
        const { x, y, zoom } = payload as { x?: number; y?: number; zoom?: number };
        if (typeof x === "number" && typeof y === "number" && typeof zoom === "number") fixture = { ...fixture, camera: { x, y, zoom } };
        break;
      }
      case "select": {
        const ids = payload.ids;
        if (Array.isArray(ids)) runtime = { ...runtime, selectedIds: ids.filter((id): id is string => typeof id === "string") };
        break;
      }
      case "nodeMove": {
        const { id, x, y } = payload as { id?: string; x?: number; y?: number };
        fixture = { ...fixture, nodes: fixture.nodes.map((node) => (node.id === id ? { ...node, x, y } : node)) };
        break;
      }
      case "nodeDragEnd": {
        const moves = payload.moves;
        if (Array.isArray(moves)) {
          fixture = {
            ...fixture,
            nodes: fixture.nodes.map((node) => {
              const move = (moves as { id?: string; x?: number; y?: number }[]).find((entry) => entry.id === node.id);
              return move ? { ...node, x: move.x, y: move.y } : node;
            }),
          };
        }
        break;
      }
      case "nodeDelete": {
        const id = payload.id;
        fixture = { ...fixture, nodes: fixture.nodes.filter((node) => node.id !== id) };
        runtime = { ...runtime, selectedIds: [] };
        break;
      }
      default:
        break;
    }
  }
  return { fixture, runtime };
}

/** @emoji 🧩 Story-local mirror of a subset of `Puzzle2dPlayApp::handle_action_patch_ops` — enough to click/pan/zoom/delete against the real fixtures. */
function reduceStoryPuzzle2dAction(state: StoryPuzzle2dState, action: string, args: Record<string, unknown> | undefined): StoryPuzzle2dState {
  const { fixture, runtime } = state;
  switch (action) {
    case "applyBoardEvents":
      return applyStoryBoardEvents(state, typeof args?.eventsJson === "string" ? args.eventsJson : "[]");
    case "setCamera": {
      const camera = args?.camera as { x: number; y: number; zoom: number } | undefined;
      return camera ? { fixture: { ...fixture, camera }, runtime } : state;
    }
    case "setSelection": {
      const ids = args?.ids;
      return { fixture, runtime: { ...runtime, selectedIds: Array.isArray(ids) ? (ids as string[]) : [] } };
    }
    case "selectAll":
      return { fixture, runtime: { ...runtime, selectedIds: fixture.nodes.map((node) => node.id) } };
    case "clearSelection":
      return { fixture, runtime: { ...runtime, selectedIds: [] } };
    case "deleteSelection": {
      const selected = new Set(runtime.selectedIds);
      return {
        fixture: {
          ...fixture,
          nodes: fixture.nodes.filter((node) => !selected.has(node.id)),
          edges: fixture.edges.filter((edge) => !selected.has(edge.id) && !selected.has(String(edge.source)) && !selected.has(String(edge.target))),
        },
        runtime: { ...runtime, selectedIds: [] },
      };
    }
    default:
      return state;
  }
}
//#endregion PluginEmulator

//#region SceneNode
function buildStorySceneNode(state: StoryPuzzle2dState, interactive: boolean): UiComponentSceneNode {
  const { fixture, runtime } = state;
  return {
    type: "componentScene",
    surfaceId: "puzzle2d.story.fixtures",
    controllerId: "puzzle2d-fixtures-story",
    componentKind: "board-2d",
    board2d: {
      fixtureJson: JSON.stringify(fixture),
      cameraJson: JSON.stringify(fixture.camera),
      glyphCatalogsJson: JSON.stringify(fixture.meta?.kindCatalogs ?? {}),
      selectionJson: JSON.stringify(runtime.selectedIds),
      interactive,
      activeUtility: runtime.activeUtility,
      selectionMethod: runtime.selectionMethod,
      gridSnapEnabled: runtime.gridSnapEnabled,
      gridFactor: runtime.gridFactor,
      suggestionOffset: runtime.suggestionOffset,
      brushWeightsJson: JSON.stringify({ nodeWeights: runtime.nodeKindWeights, handleWeights: runtime.handleKindWeights }),
      placementCompatibilityJson: JSON.stringify(fixture.meta?.kindCompatibility ?? []),
      lodMode: runtime.lodMode,
    },
  };
}
//#endregion SceneNode

//#region StoryHost
function Board2dFixtureStoryHost({ fixture, interactive }: { readonly fixture: StoryPuzzle2dFixture; readonly interactive: boolean }): ReactElement {
  const [state, setState] = useState<StoryPuzzle2dState>(() => ({ fixture, runtime: STORY_DEFAULT_RUNTIME }));

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setState((current) => reduceStoryPuzzle2dAction(current, descriptor.action, descriptor.args));
  }, []);

  const node = useMemo(() => buildStorySceneNode(state, interactive), [state, interactive]);
  const debug = useMemo(() => JSON.stringify({ selection: state.runtime.selectedIds, camera: state.fixture.camera, nodeCount: state.fixture.nodes.length, edgeCount: state.fixture.edges.length }), [state]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <Board2dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="puzzle2d-fixture-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🧩puzzle🩻2d/Fixtures",
  component: Board2dFixtureStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Board2dFixtureStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🏯 180 nodes / 179 edges — the real Nakagin Capsule Tower 2D board fixture (`puzzle/2d/example/nakagin-capsule-tower.2d.json`). */
export const NakaginCapsuleTower: Story = {
  args: {
    fixture: nakaginCapsuleTowerFixture as unknown as StoryPuzzle2dFixture,
    interactive: true,
  },
};

/** 🌲 The real Concrete Forest 2D board fixture (`puzzle/2d/example/concrete-forest.2d.json`). */
export const ConcreteForest: Story = {
  args: {
    fixture: concreteForestFixture as unknown as StoryPuzzle2dFixture,
    interactive: true,
  },
};
