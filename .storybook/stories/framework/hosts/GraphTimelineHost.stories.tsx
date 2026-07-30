// #region 🧲Header
// 💻 .storybook/stories/framework/hosts/GraphTimelineHost.stories.tsx
// Specs: Host the framework renderer's `GraphTimelineHost` with zero WASM engine — it renders `@semio-tech/ui-react`'s
// `HistoryTable` straight off `GraphTimelineScene.columnsJson` (a `HistoryColumn[]`), no engine involved.
// Summary: A debug-readout host records the `checkoutCheckpoint` action `HistoryTable`'s row click dispatches;
// `HistoryTable` itself has no selected-row prop to reflect back (see `framework/ui/js/react/index.tsx`'s `HistoryTableProps`),
// so — unlike the reducer-backed hosts in this scope — there is no further scene state to round-trip.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useState, type ReactElement } from "react";

import { GraphTimelineHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, GraphTimelineScene, UiComponentSceneNode } from "@semio-tech/framework-core";
import type { HistoryColumn } from "@semio-tech/ui-react";

//#region SceneFixtures
/** 🌳 Two forked branches off a shared root — matches the swimlane/fork-elbow shape `HistoryTable`'s own test exercises (`framework/ui/js/react/index.tsx`, "renders swimlane guides and fork elbows"). */
const BRANCHING_COLUMNS: readonly HistoryColumn[] = [
  { checkpointId: "c4", timestamp: "2026-07-19T10:04:00Z", labels: ["feature-hosts"], authors: [{ id: "u1", name: "Ueli" }], parentCheckpointId: "c2", description: "wire up TableHost story", lane: 1, alternativeIds: ["hosts"] },
  { checkpointId: "c3", timestamp: "2026-07-19T10:03:00Z", labels: ["feature-editor"], authors: [{ id: "u2", name: "Nadia" }], parentCheckpointId: "c2", description: "editor wasm gate", lane: 2, alternativeIds: ["editor"] },
  { checkpointId: "c2", timestamp: "2026-07-19T10:02:00Z", labels: [], authors: [{ id: "u1", name: "Ueli" }], parentCheckpointId: "c1", description: "split scopes", lane: 0, alternativeIds: [] },
  { checkpointId: "c1", timestamp: "2026-07-19T10:01:00Z", labels: ["main"], authors: [{ id: "u3", name: "Priya" }], parentCheckpointId: undefined, description: "root", lane: 0, alternativeIds: [] },
];

/** 📏 A single lane, no forks or labels/authors — exercises the "checkpoint chip + unknown avatar" fallback row rendering. */
const LINEAR_COLUMNS: readonly HistoryColumn[] = [
  { checkpointId: "l3", timestamp: "3", labels: [], authors: [], parentCheckpointId: "l2", description: undefined, lane: 0, alternativeIds: [] },
  { checkpointId: "l2", timestamp: "2", labels: [], authors: [], parentCheckpointId: "l1", description: undefined, lane: 0, alternativeIds: [] },
  { checkpointId: "l1", timestamp: "1", labels: [], authors: [], parentCheckpointId: undefined, description: undefined, lane: 0, alternativeIds: [] },
];
//#endregion SceneFixtures

//#region StoryHost
function GraphTimelineStoryHost({ columns, controllerId, surfaceId }: { readonly columns: readonly HistoryColumn[]; readonly controllerId: string; readonly surfaceId: string }): ReactElement {
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((action: ActionDescriptor): void => {
    setLastAction(action);
  }, []);

  const scene: GraphTimelineScene = { columnsJson: JSON.stringify(columns) };
  const node: UiComponentSceneNode = { type: "componentScene", surfaceId, controllerId, componentKind: "graph-timeline", graphTimeline: scene };

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <GraphTimelineHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="graph-timeline-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {JSON.stringify({ lastAction })}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌hosts/GraphTimelineHost",
  component: GraphTimelineStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof GraphTimelineStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🌳 Two forked branches — clicking a row dispatches `checkoutCheckpoint`, visible in the debug readout. */
export const Branching: Story = {
  args: { columns: BRANCHING_COLUMNS, controllerId: "vcs-play", surfaceId: "vcs.play.timeline" },
};

/** 📏 Single lane, no labels/authors — the fallback checkpoint-chip/unknown-avatar row rendering. */
export const Linear: Story = {
  args: { columns: LINEAR_COLUMNS, controllerId: "vcs-play", surfaceId: "vcs.play.timeline-linear" },
};

/** 🕳️ No checkpoints at all — the em-dash placeholder row. */
export const Empty: Story = {
  args: { columns: [], controllerId: "vcs-play", surfaceId: "vcs.play.timeline-empty" },
};
