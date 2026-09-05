// #region 🧲️Header
// 💻️ .storybook/story/puzzle/2d/Board.stories.tsx
// Specs: Host the framework renderer's `🖥️Board2dHost` for Storybook + Playwright selection/camera/utility checks.
// Summary: Mounts the host directly against a `UiComponentSceneNode`; a story-local reducer emulates the `puzzle2d-play` Rust plugin's `applyBoardEvents`/selection/utility actions so the controlled scene ⇄️ session loop round-trips without a running dev server.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Board2dHost } from "../../../../framework/product/os/module/renderer/js/react/index.tsx";
import type { ActionDescriptor, UiComponentSceneNode } from "../../../../framework/product/os/module/renderer/js/react/index.tsx";

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

/** @emoji 🎲️ Mints a story-local brush id that avoids every id already present in `existingIds` — mirrors `unique_node_id`/`unique_edge_id`'s collision re-mint without needing the real per-session serial counter. */
function newStoryBrushEntityId(kind: "node" | "edge", existingIds: ReadonlySet<string>): string {
  let index = 1;
  let candidate = `brush-${kind}-${index}`;
  while (existingIds.has(candidate)) {
    index += 1;
    candidate = `brush-${kind}-${index}`;
  }
  return candidate;
}

/** @emoji 📬️ Story-local mirror of `apply_board_events_from_json` in `puzzle/plugin/rs/d2/mod.rs` — only the event kinds the stories exercise. */
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
      case "edgeDelete": {
        const id = payload.id;
        fixture = { ...fixture, edges: fixture.edges.filter((edge) => edge.id !== id) };
        break;
      }
      case "edgeCreate": {
        fixture = { ...fixture, edges: [...fixture.edges, payload as StoryPuzzle2dEntity] };
        break;
      }
      case "brushPlace": {
        const nodeIds = new Set(fixture.nodes.map((existing) => existing.id));
        const edgeIds = new Set(fixture.edges.map((existing) => existing.id));
        const requestedNodeId = typeof payload.nodeId === "string" ? payload.nodeId : undefined;
        const nodeId = requestedNodeId && !nodeIds.has(requestedNodeId) ? requestedNodeId : newStoryBrushEntityId("node", nodeIds);
        const requestedEdgeId = typeof payload.edgeId === "string" ? payload.edgeId : undefined;
        const edgeId = requestedEdgeId && !edgeIds.has(requestedEdgeId) ? requestedEdgeId : newStoryBrushEntityId("edge", edgeIds);
        const nodeKind = typeof payload.nodeKind === "string" ? payload.nodeKind : "node";
        const shape = typeof payload.shape === "string" ? payload.shape : "circle";
        const node: StoryPuzzle2dEntity = {
          id: nodeId,
          nodeKind,
          shape,
          x: typeof payload.x === "number" ? payload.x : 0,
          y: typeof payload.y === "number" ? payload.y : 0,
          text: nodeKind,
          handles: Array.isArray(payload.handles) ? payload.handles : [],
          ...(shape === "rectangle"
            ? { width: typeof payload.width === "number" ? payload.width : 48, height: typeof payload.height === "number" ? payload.height : 48 }
            : { radius: typeof payload.radius === "number" ? payload.radius : 24 }),
          ...(payload.iconKind !== undefined ? { iconKind: payload.iconKind } : {}),
        };
        const sourceHandleId = typeof payload.sourceHandleId === "string" ? payload.sourceHandleId : "";
        const edges =
          sourceHandleId !== ""
            ? [...fixture.edges, { id: edgeId, edgeKind: "link", source: sourceHandleId, target: `${nodeId}:v${typeof payload.targetHandleIndex === "number" ? payload.targetHandleIndex : 0}` }]
            : fixture.edges;
        fixture = { ...fixture, nodes: [...fixture.nodes, node], edges };
        break;
      }
      default:
        break;
    }
  }
  return { fixture, runtime };
}

/** @emoji 🧩️ Story-local mirror of a subset of `Puzzle2dPlayApp::handle_action_patch_operations` — enough for the interaction stories to round-trip. */
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
    case "setActiveUtility":
      return { fixture, runtime: { ...runtime, activeUtility: typeof args?.utilityId === "string" ? args.utilityId : "select" } };
    case "setSelectionFlag": {
      const flag = args?.flag === "locked" ? "locked" : "hidden";
      const value = Boolean(args?.value);
      const selected = new Set(runtime.selectedIds);
      const patch = (entity: StoryPuzzle2dEntity): StoryPuzzle2dEntity => (selected.has(entity.id) ? { ...entity, [flag]: value } : entity);
      return { fixture: { ...fixture, nodes: fixture.nodes.map(patch), edges: fixture.edges.map(patch) }, runtime };
    }
    case "duplicateSelection": {
      const selected = new Set(runtime.selectedIds);
      const clones = fixture.nodes.filter((node) => selected.has(node.id)).map((node) => ({ ...node, id: `${node.id}-copy`, x: (node.x as number) + 24, y: (node.y as number) + 24 }));
      if (clones.length === 0) return state;
      return { fixture: { ...fixture, nodes: [...fixture.nodes, ...clones] }, runtime: { ...runtime, selectedIds: clones.map((clone) => clone.id) } };
    }
    case "selectSameKind": {
      const kinds = new Set(fixture.nodes.filter((node) => runtime.selectedIds.includes(node.id)).map((node) => node.nodeKind));
      return { fixture, runtime: { ...runtime, selectedIds: fixture.nodes.filter((node) => kinds.has(node.nodeKind)).map((node) => node.id) } };
    }
    case "addNode": {
      const id = `story-node-${fixture.nodes.length + 1}`;
      const node: StoryPuzzle2dEntity = { id, nodeKind: args?.kind ?? "node", shape: args?.shape ?? "circle", x: args?.x ?? 0, y: args?.y ?? 0, radius: args?.radius ?? 24, text: id, handles: [] };
      return { fixture: { ...fixture, nodes: [...fixture.nodes, node] }, runtime };
    }
    default:
      return state;
  }
}
//#endregion PluginEmulator

//#region SceneNode
function catalogRowSubset(row: Record<string, unknown>, keys: readonly string[]): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  for (const key of keys) if (row[key] !== undefined && row[key] !== null) out[key] = row[key];
  return out;
}

function catalogRowsSubset(catalogs: Record<string, unknown>, slice: string, keys: readonly string[]): Record<string, unknown>[] | undefined {
  const rows = catalogs[slice];
  return Array.isArray(rows) ? rows.map((row) => catalogRowSubset(row as Record<string, unknown>, keys)) : undefined;
}

/** @emoji 🗂️ Story-local mirror of `board_kind_catalogs_json` — the document owns `nodes`/`🐙️handles`/`edges`/`wires`, the board engine reads `nodeKinds`/`handleKinds`/`edgeKinds`/`wireKinds` and rejects any row still carrying the document's `label`, so each row is narrowed to the keys the engine reads and an absent slice is omitted rather than emitted empty. Keeping the fixtures document-shaped is deliberate: hand-writing engine-shaped catalogs here is what let the missing production translation go unnoticed. */
function storyBoardKindCatalogsJson(fixture: StoryPuzzle2dFixture): string {
  const catalogs = fixture.meta?.kindCatalogs as Record<string, unknown> | undefined;
  if (!catalogs) return "{}";
  const nodeRows = Array.isArray(catalogs.nodes) ? catalogs.nodes : undefined;
  const slices: readonly (readonly [string, readonly Record<string, unknown>[] | undefined])[] = [
    ["handleKinds", catalogRowsSubset(catalogs, "handles", ["id", "name", "color", "defaultWireKind", "scale"])],
    ["wireKinds", catalogRowsSubset(catalogs, "wires", ["id", "name", "defaultEdgeKind"])],
    [
      "nodeKinds",
      nodeRows?.map((row) => {
        const node = row as Record<string, unknown>;
        const templates = Array.isArray(node.handles) ? node.handles : [];
        return {
          ...catalogRowSubset(node, ["id", "name", "icon", "color", "shape", "scale"]),
          handles: templates
            .map((template) => template as Record<string, unknown>)
            .filter((template) => typeof template.handleKind === "string" && template.handleKind.trim() !== "")
            .map((template) => catalogRowSubset(template, ["handleKind", "angle", "radius"])),
        };
      }),
    ],
    ["edgeKinds", catalogRowsSubset(catalogs, "edges", ["id", "name", "color", "stroke", "pattern", "sourceTip", "targetTip", "directed"])],
  ];
  return JSON.stringify(Object.fromEntries(slices.filter((entry): entry is readonly [string, readonly Record<string, unknown>[]] => entry[1] !== undefined)));
}

function buildStorySceneNode(state: StoryPuzzle2dState, interactive: boolean): UiComponentSceneNode {
  const { fixture, runtime } = state;
  return {
    type: "componentScene",
    surfaceId: "puzzle2d.story.overview",
    controllerId: "puzzle2d-story",
    componentKind: "board-2d",
    board2d: {
      fixtureJson: JSON.stringify(fixture),
      cameraJson: JSON.stringify(fixture.camera),
      glyphCatalogsJson: storyBoardKindCatalogsJson(fixture),
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

//#region Fixtures
const STORY_DEFAULT_FIXTURE: StoryPuzzle2dFixture = {
  schema: "puzzle.2d.fixture",
  camera: { x: 140, y: 60, zoom: 1 },
  nodes: [
    { id: "alpha", nodeKind: "seed", shape: "circle", x: 0, y: 0, radius: 44, text: "alpha", handles: [{ id: "alpha:v0", handleKind: "port", angle: 0, radius: 6 }] },
    { id: "beta", nodeKind: "seed", shape: "circle", x: 280, y: 120, radius: 40, text: "beta", handles: [{ id: "beta:v0", handleKind: "port", angle: Math.PI, radius: 6 }] },
  ],
  edges: [{ id: "link-1", edgeKind: "link", source: "alpha:v0", target: "beta:v0" }],
};

const STORY_BRUSH_FIXTURE: StoryPuzzle2dFixture = {
  ...STORY_DEFAULT_FIXTURE,
  meta: {
    kindCatalogs: { nodes: [{ id: "seed", name: "Seed" }], handles: [{ id: "port", name: "Port" }] },
    kindCompatibility: [{ source: "port", target: "port", bidirectional: true, specificity: 0 }],
  },
};

/** @emoji 🖌️ Same board as `STORY_BRUSH_FIXTURE` but `alpha` carries a second, unconnected `port` handle (`alpha:v1`) so the brush has a free slot to preview and commit into, and the `seed` node kind declares a handle template so `brush_compatible_candidates` has something to offer. Catalogs stay in the **document** `nodes`/`🐙️handles` shape — `storyBoardKindCatalogsJson` does the translation, exactly as production does. */
const STORY_BRUSH_OPEN_SLOT_FIXTURE: StoryPuzzle2dFixture = {
  schema: "puzzle.2d.fixture",
  camera: { x: 140, y: 60, zoom: 1 },
  nodes: [
    {
      id: "alpha",
      nodeKind: "seed",
      shape: "circle",
      x: 0,
      y: 0,
      radius: 44,
      text: "alpha",
      handles: [
        { id: "alpha:v0", handleKind: "port", angle: 0, radius: 6 },
        { id: "alpha:v1", handleKind: "port", angle: Math.PI / 2, radius: 6 },
      ],
    },
    { id: "beta", nodeKind: "seed", shape: "circle", x: 280, y: 120, radius: 40, text: "beta", handles: [{ id: "beta:v0", handleKind: "port", angle: Math.PI, radius: 6 }] },
  ],
  edges: [{ id: "link-1", edgeKind: "link", source: "alpha:v0", target: "beta:v0" }],
  meta: {
    kindCatalogs: {
      nodes: [{ id: "seed", name: "Seed", label: "Seed", handles: [{ id: "t0", name: "t0", label: "T0", handleKind: "port", angle: 0 }] }],
      handles: [{ id: "port", label: "Port", color: "#888888" }],
      edges: [],
      wires: [],
    },
    kindCompatibility: [{ source: "port", target: "port", bidirectional: true, specificity: 0 }],
  },
};
//#endregion Fixtures

//#region StoryHost
function Board2dStoryHost({ initialFixture, initialRuntime, interactive }: { readonly initialFixture: StoryPuzzle2dFixture; readonly initialRuntime: Partial<StoryPuzzle2dRuntime>; readonly interactive: boolean }): ReactElement {
  const [state, setState] = useState<StoryPuzzle2dState>(() => ({ fixture: initialFixture, runtime: { ...STORY_DEFAULT_RUNTIME, ...initialRuntime } }));

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setState((current) => reduceStoryPuzzle2dAction(current, descriptor.action, descriptor.args));
  }, []);

  const node = useMemo(() => buildStorySceneNode(state, interactive), [state, interactive]);
  const debug = useMemo(() => JSON.stringify({ selection: state.runtime.selectedIds, camera: state.fixture.camera, activeUtility: state.runtime.activeUtility, nodeCount: state.fixture.nodes.length, edgeCount: state.fixture.edges.length }), [state]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <Board2dHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="puzzle2d-board-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🧩️puzzle🩻️2d",
  component: Board2dStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Board2dStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

export const OverviewSelect: Story = {
  args: {
    initialFixture: STORY_DEFAULT_FIXTURE,
    initialRuntime: {},
    interactive: true,
  },
};

export const LassoSelect: Story = {
  args: {
    initialFixture: STORY_DEFAULT_FIXTURE,
    initialRuntime: { selectionMethod: "lasso" },
    interactive: true,
  },
};

export const BrushUtility: Story = {
  args: {
    initialFixture: STORY_BRUSH_FIXTURE,
    initialRuntime: { activeUtility: "brush" },
    interactive: true,
  },
};

export const BrushPlacement: Story = {
  args: {
    initialFixture: STORY_BRUSH_OPEN_SLOT_FIXTURE,
    initialRuntime: { activeUtility: "brush" },
    interactive: true,
  },
};

export const ForcedLodPane: Story = {
  args: {
    initialFixture: STORY_DEFAULT_FIXTURE,
    initialRuntime: { lodMode: "detail" },
    interactive: false,
  },
};
