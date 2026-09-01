// #region 🧲️Header
// 💻️ .storybook/stories/framework/hosts/BlockListHost.stories.tsx
// Specs: Host the framework renderer's `BlockListHost` with zero WASM engine — the Blockly-like step/block
// editor is pure `dnd-kit` + declarative JSON, so a story-local reducer round-trips `addStep`/`removeStep`/
// `moveStep`/`addBlock`/`removeBlock`/`moveBlock` for real.
// Summary: `reduceStoryBlockListAction` mirrors `BlockListHost`'s dispatched actions (`framework/os/renderer/js/react/index.tsx`).
// Clicking a palette entry's `addBlock` (no `stepId` — `PalettePanel`'s click handler only ever sends `{ kind }`)
// targets the *last* step, matching the only sane host-app default when the plugin protocol itself doesn't say which step.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { BlockListHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, BlockListScene, UiComponentSceneNode } from "@semio-tech/framework";

//#region StoryTypes
type StoryBlock = { readonly id: string; readonly label: string; readonly kind: string; readonly description?: string };
type StoryStep = { readonly id: string; readonly title: string; readonly description?: string; readonly blocks: readonly StoryBlock[] };
type StoryPaletteEntry = { readonly blockKind: string; readonly label: string; readonly iconId: string };
type StoryBlockListState = { readonly steps: readonly StoryStep[]; readonly palette: readonly StoryPaletteEntry[]; readonly stepCounter: number; readonly blockCounter: number };
//#endregion StoryTypes

//#region Fixtures
const STORY_PALETTE: readonly StoryPaletteEntry[] = [
  { blockKind: "move", label: "Move", iconId: "move" },
  { blockKind: "wait", label: "Wait", iconId: "clock" },
  { blockKind: "say", label: "Say", iconId: "message-circle" },
];

const STORY_INITIAL_STEPS: readonly StoryStep[] = [
  { id: "step-1", title: "Step 1", description: "Initial setup", blocks: [{ id: "step-1-block-1", label: "Move", kind: "move" }] },
  { id: "step-2", title: "Step 2", blocks: [] },
];
//#endregion Fixtures

//#region Reducer
const STORY_BLOCK_LIST_CONTROLLER_ID = "block-list-story";

function storyPaletteLabel(palette: readonly StoryPaletteEntry[], kind: string): string {
  return palette.find((entry) => entry.blockKind === kind)?.label ?? kind;
}

/** @emoji 🧩️ Story-local mirror of the `addStep`/`removeStep`/`moveStep`/`addBlock`/`removeBlock`/`moveBlock` handling a real host app performs against `BlockListHost`'s dispatched actions (`dispatchBlockListAction`). */
function reduceStoryBlockListAction(state: StoryBlockListState, descriptor: ActionDescriptor): StoryBlockListState {
  const args = (descriptor.args ?? {}) as Record<string, unknown>;
  switch (descriptor.action) {
    case "addStep": {
      const stepCounter = state.stepCounter + 1;
      const step: StoryStep = { id: `step-${stepCounter}`, title: `Step ${stepCounter}`, blocks: [] };
      return { ...state, steps: [...state.steps, step], stepCounter };
    }
    case "removeStep": {
      const stepId = String(args.stepId ?? "");
      return { ...state, steps: state.steps.filter((step) => step.id !== stepId) };
    }
    case "moveStep": {
      const stepId = String(args.stepId ?? "");
      const index = Number(args.index ?? 0);
      const steps = [...state.steps];
      const from = steps.findIndex((step) => step.id === stepId);
      if (from < 0) return state;
      const [moved] = steps.splice(from, 1);
      steps.splice(Math.max(0, Math.min(steps.length, index)), 0, moved);
      return { ...state, steps };
    }
    case "addBlock": {
      const kind = String(args.kind ?? "");
      const targetStepId = typeof args.stepId === "string" ? args.stepId : state.steps[state.steps.length - 1]?.id;
      if (!targetStepId || !kind) return state;
      const blockCounter = state.blockCounter + 1;
      const block: StoryBlock = { id: `block-${blockCounter}`, label: storyPaletteLabel(state.palette, kind), kind };
      return { ...state, blockCounter, steps: state.steps.map((step) => (step.id === targetStepId ? { ...step, blocks: [...step.blocks, block] } : step)) };
    }
    case "removeBlock": {
      const stepId = String(args.stepId ?? "");
      const blockId = String(args.blockId ?? "");
      return { ...state, steps: state.steps.map((step) => (step.id === stepId ? { ...step, blocks: step.blocks.filter((block) => block.id !== blockId) } : step)) };
    }
    case "moveBlock": {
      const blockId = String(args.blockId ?? "");
      const fromStepId = String(args.fromStepId ?? "");
      const toStepId = String(args.toStepId ?? fromStepId);
      const index = Number(args.index ?? 0);
      const source = state.steps.find((step) => step.id === fromStepId);
      const block = source?.blocks.find((entry) => entry.id === blockId);
      if (!block) return state;
      return {
        ...state,
        steps: state.steps.map((step) => {
          if (step.id === fromStepId && step.id === toStepId) {
            const blocks = step.blocks.filter((entry) => entry.id !== blockId);
            blocks.splice(Math.max(0, Math.min(blocks.length, index)), 0, block);
            return { ...step, blocks };
          }
          if (step.id === fromStepId) return { ...step, blocks: step.blocks.filter((entry) => entry.id !== blockId) };
          if (step.id === toStepId) {
            const blocks = [...step.blocks];
            blocks.splice(Math.max(0, Math.min(blocks.length, index)), 0, block);
            return { ...step, blocks };
          }
          return step;
        }),
      };
    }
    default:
      return state;
  }
}
//#endregion Reducer

//#region StoryHost
function BlockListStoryHost({ initialSteps }: { readonly initialSteps: readonly StoryStep[] }): ReactElement {
  const [state, setState] = useState<StoryBlockListState>(() => ({ steps: initialSteps, palette: STORY_PALETTE, stepCounter: initialSteps.length, blockCounter: 0 }));

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setState((current) => reduceStoryBlockListAction(current, descriptor));
  }, []);

  const scene: BlockListScene = useMemo(() => ({ stepsJson: JSON.stringify(state.steps), paletteJson: JSON.stringify(state.palette) }), [state]);
  const node: UiComponentSceneNode = useMemo(() => ({ type: "componentScene", surfaceId: "block-list.story.overview", controllerId: STORY_BLOCK_LIST_CONTROLLER_ID, componentKind: "block-list", blockList: scene }), [scene]);
  const debug = useMemo(() => JSON.stringify({ steps: state.steps.map((step) => ({ id: step.id, blocks: step.blocks.map((block) => block.id) })) }), [state]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <BlockListHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="block-list-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}

/** @emoji 🕳️ `BlockListHost` with an absent `blockList` scene — exercises the `emptyLabel` fallback path with zero fixture setup. */
function BlockListStoryEmptyHost(): ReactElement {
  const node: UiComponentSceneNode = { type: "componentScene", surfaceId: "block-list.story.empty", controllerId: STORY_BLOCK_LIST_CONTROLLER_ID, componentKind: "block-list" };
  return (
    <div style={{ height: "100%", width: "100%" }}>
      <BlockListHost node={node} onAction={() => undefined} />
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌️hosts/BlockListHost",
  component: BlockListStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof BlockListStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🧩️ Two steps, a palette of three block kinds — "Add Step" (`addStep`), a palette click (`addBlock`, targets the last step), a block's delete button (`removeBlock`), and drag-reorder (`moveStep`/`moveBlock`) all round-trip. */
export const Editable: Story = {
  args: { initialSteps: STORY_INITIAL_STEPS },
};

/** 🕳️ No `blockList` scene — the `emptyLabel` fallback. */
export const EmptyScene: Story = {
  render: () => <BlockListStoryEmptyHost />,
};
