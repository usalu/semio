// #region 🧲️Header
// 💻️ .storybook/stories/framework/hosts/InkCanvasHost.stories.tsx
// Specs: Host the framework renderer's `InkCanvasHost` with zero WASM engine — the whiteboard-style ink
// surface (pointer gestures, hit-testing, resize handles) is implemented entirely in React/DOM (`framework/
// renderer/react/index.tsx`), so a story-local reducer over `InkCanvasEvent`s is enough for real interaction.
// Summary: `applyStoryInkEvents` mirrors `applyEventsLocal`'s operation vocabulary (`addBlock`/`updateBlock`/
// `removeBlock`/`putAsset`/`setCamera`) that a real plugin's `inkApplyEvents` handler would apply; the reducer
// also handles the sibling `setSelection`/`setCamera`/`setHover` actions `InkCanvasHost` dispatches directly
// (`inkCanvasActions`). Interactive drag/draw/pan/erase and click-to-select therefore all round-trip live.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { InkCanvasHost } from "@semio-tech/framework-renderer-react";
import type { InkCanvasEvent, InkDocument, InkItem } from "@semio-tech/framework-renderer-react";
import { inkCanvasActions } from "@semio-tech/framework";
import type { ActionDescriptor, InkCanvasScene, UiComponentSceneNode } from "@semio-tech/framework";

//#region Fixtures
const STORY_INK_DOCUMENT: InkDocument = {
  schema: "ink.document",
  id: "story-ink-doc",
  camera: { x: 0, y: 0, zoom: 1 },
  activeUtility: "selectDirect",
  gridVisible: true,
  gridSpacing: 24,
  blocks: [
    {
      id: "text-1",
      kind: "text",
      name: "Heading",
      x: 40,
      y: 40,
      width: 220,
      height: 48,
      visible: true,
      locked: false,
      paragraphs: [{ runs: [{ text: "Design notes", bold: true }] }],
      fontSize: 20,
      fontWeight: "bold",
      align: "left",
    },
    {
      id: "stroke-1",
      kind: "stroke",
      name: "Sketch",
      x: 0,
      y: 0,
      width: 240,
      height: 160,
      visible: true,
      locked: false,
      points: [
        [60, 140],
        [120, 180],
        [180, 140],
        [220, 200],
      ],
      strokeWidth: 3,
      color: [0.4, 0.7, 1, 1],
    },
    {
      id: "table-1",
      kind: "table",
      name: "Parts",
      x: 60,
      y: 240,
      width: 240,
      height: 100,
      visible: true,
      locked: false,
      columns: ["Item", "Qty"],
      rows: [
        [{ content: "Bolt" }, { content: "12" }],
        [{ content: "Nut" }, { content: "12" }],
      ],
    },
  ] satisfies readonly InkItem[],
};
//#endregion Fixtures

//#region Reducer
type StoryInkState = { readonly document: InkDocument; readonly selection: readonly string[]; readonly hoveredId: string | null };

/** @emoji ✍️ Story-local mirror of `applyEventsLocal` (`framework/os/renderer/js/react/index.tsx`) — the subset of a real plugin's `inkApplyEvents` operation vocabulary the stories exercise. */
function applyStoryInkEvents(document: InkDocument, events: readonly InkCanvasEvent[]): InkDocument {
  let blocks = document.blocks;
  let next = document;
  for (const event of events) {
    switch (event.operation) {
      case "addBlock":
        blocks = [...blocks, event.block];
        break;
      case "updateBlock":
        blocks = blocks.map((block) => (block.id === event.blockId ? event.block : block));
        break;
      case "removeBlock":
        blocks = blocks.filter((block) => block.id !== event.blockId);
        break;
      case "putAsset":
        next = { ...next, assets: { ...(next.assets ?? {}), [event.key]: event.asset } };
        break;
      case "setCamera":
        next = { ...next, camera: event.camera };
        break;
    }
  }
  return { ...next, blocks };
}

/** @emoji ✍️ Story-local mirror of the `inkCanvasActions.applyEvents`/`setSelection`/`setCamera`/`setHover` handling a real host app performs against `InkCanvasHost`'s dispatched actions. */
function reduceStoryInkAction(state: StoryInkState, descriptor: ActionDescriptor): StoryInkState {
  const args = (descriptor.args ?? {}) as Record<string, unknown>;
  switch (descriptor.action) {
    case inkCanvasActions.applyEvents: {
      const events = typeof args.eventsJson === "string" ? (JSON.parse(args.eventsJson) as readonly InkCanvasEvent[]) : [];
      const document = applyStoryInkEvents(state.document, events);
      const selectIds = args.selectIds;
      const selection = Array.isArray(selectIds) ? selectIds.filter((id): id is string => typeof id === "string") : state.selection;
      return { ...state, document, selection };
    }
    case inkCanvasActions.setSelection: {
      const ids = args.ids;
      return { ...state, selection: Array.isArray(ids) ? ids.filter((id): id is string => typeof id === "string") : [] };
    }
    case inkCanvasActions.setCamera: {
      const camera = args.camera as InkDocument["camera"] | undefined;
      return camera ? { ...state, document: { ...state.document, camera } } : state;
    }
    case inkCanvasActions.setHover: {
      const id = args.id;
      return { ...state, hoveredId: typeof id === "string" ? id : null };
    }
    default:
      return state;
  }
}
//#endregion Reducer

//#region SceneNode
const STORY_INK_CONTROLLER_ID = "ink-canvas-story";

function buildStoryInkScene(state: StoryInkState, interactive: boolean, viewMode: string): InkCanvasScene {
  return {
    documentJson: JSON.stringify(state.document),
    selectionJson: JSON.stringify(state.selection),
    hoveredId: state.hoveredId ?? undefined,
    activeUtility: state.document.activeUtility ?? "selectDirect",
    viewMode,
    interactive,
  };
}
//#endregion SceneNode

//#region StoryHost
function InkCanvasStoryHost({ interactive, viewMode }: { readonly interactive: boolean; readonly viewMode: string }): ReactElement {
  const [state, setState] = useState<StoryInkState>({ document: STORY_INK_DOCUMENT, selection: [], hoveredId: null });

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setState((current) => reduceStoryInkAction(current, descriptor));
  }, []);

  const node: UiComponentSceneNode = useMemo(
    () => ({ type: "componentScene", surfaceId: "ink-canvas.story.overview", controllerId: STORY_INK_CONTROLLER_ID, componentKind: "ink-canvas", inkCanvas: buildStoryInkScene(state, interactive, viewMode) }),
    [state, interactive, viewMode],
  );
  const debug = useMemo(() => JSON.stringify({ blockCount: state.document.blocks.length, selection: state.selection, hoveredId: state.hoveredId, camera: state.document.camera }), [state]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <InkCanvasHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="ink-canvas-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌️hosts/InkCanvasHost",
  component: InkCanvasStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof InkCanvasStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** ✍️ A text heading, a sketch stroke, and a table block — direct-select, drag, pan (middle-drag or alt-drag), and pencil/eraser utilities all round-trip through `inkApplyEvents`. */
export const Editable: Story = {
  args: { interactive: true, viewMode: "edit" },
};

/** 🔒️ `interactive: false` + `viewMode: "navigator"` — the read-only preview path (pointer gestures are gated off, resize handles never show). */
export const NavigatorPreview: Story = {
  args: { interactive: false, viewMode: "navigator" },
};
