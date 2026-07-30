// #region 🧲Header
// 💻 .storybook/stories/framework/hosts/TableHost.stories.tsx
// Specs: Host the framework renderer's `TableHost` with zero WASM engine — `TableHost` is a pure declarative
// renderer over `TableScene.columnsJson`/`rowsJson` (`@semio-tech/ui-react`'s `Table`), so a story-local
// reducer alone is enough to make sorting, row selection, the stepper cell, and the row-action button all
// round-trip for real.
// Summary: `reduceStoryTableAction` mirrors the subset of a real host app's `sortTable`/`selectRow`/
// `adjustCount`/`removeRow` handling `TableHost` (`framework/os/renderer/js/react/index.tsx`) actually dispatches —
// `dispatchCellAction` merges a stepper's `{ delta }` or a row-action button's `{}` into that cell's own
// `ActionDescriptor.args`, so the reducer reads `rowId` back out of `args` rather than needing a separate wire.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { TableHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, TableScene, UiComponentSceneNode } from "@semio-tech/framework-core";

//#region StoryTypes
type StoryTableRow = { readonly id: string; readonly name: string; readonly kind: string; readonly count: number };
type StorySort = { readonly columnId: string; readonly direction: "asc" | "desc" };
type StoryTableState = { readonly rows: readonly StoryTableRow[]; readonly selectedIds: readonly string[]; readonly sort: StorySort | null };
//#endregion StoryTypes

//#region Fixtures
const STORY_TABLE_ROWS: readonly StoryTableRow[] = [
  { id: "row-beta", name: "Beta", kind: "handle", count: 5 },
  { id: "row-alpha", name: "Alpha", kind: "seed", count: 2 },
  { id: "row-gamma", name: "Gamma", kind: "seed", count: 0 },
];
//#endregion Fixtures

//#region Reducer
const STORY_TABLE_CONTROLLER_ID = "table-story";

/** @emoji 🧮 Story-local mirror of the `sortTable`/`selectRow`/`adjustCount`/`removeRow` handling a real host app performs against `TableHost`'s dispatched actions. */
function reduceStoryTableAction(state: StoryTableState, descriptor: ActionDescriptor): StoryTableState {
  const args = (descriptor.args ?? {}) as Record<string, unknown>;
  switch (descriptor.action) {
    case "sortTable": {
      const columnId = String(args.columnId ?? "");
      const direction = args.direction === "desc" ? "desc" : "asc";
      const rows = [...state.rows].sort((left, right) => {
        const leftValue = (left as unknown as Record<string, unknown>)[columnId];
        const rightValue = (right as unknown as Record<string, unknown>)[columnId];
        if (leftValue === rightValue) return 0;
        const cmp = (leftValue ?? "") < (rightValue ?? "") ? -1 : 1;
        return direction === "asc" ? cmp : -cmp;
      });
      return { ...state, rows, sort: { columnId, direction } };
    }
    case "selectRow": {
      const row = args.row as { readonly id?: string } | undefined;
      return { ...state, selectedIds: row?.id ? [row.id] : [] };
    }
    case "adjustCount": {
      const rowId = String(args.rowId ?? "");
      const delta = Number(args.delta ?? 0);
      return { ...state, rows: state.rows.map((row) => (row.id === rowId ? { ...row, count: Math.max(0, Math.min(10, row.count + delta)) } : row)) };
    }
    case "removeRow": {
      const rowId = String(args.rowId ?? "");
      return { ...state, rows: state.rows.filter((row) => row.id !== rowId), selectedIds: state.selectedIds.filter((id) => id !== rowId) };
    }
    default:
      return state;
  }
}
//#endregion Reducer

//#region SceneNode
function buildStoryTableScene(state: StoryTableState): TableScene {
  const rows = state.rows.map((row) => ({
    id: row.id,
    name: { kind: "text", value: row.name },
    kind: { kind: "text", value: row.kind },
    count: { kind: "stepper", value: row.count, min: 0, max: 10, step: 1, action: { controllerId: STORY_TABLE_CONTROLLER_ID, action: "adjustCount", args: { rowId: row.id } } },
    remove: { kind: "buttons", buttons: [{ iconId: "trash-2", label: "Remove", action: { controllerId: STORY_TABLE_CONTROLLER_ID, action: "removeRow", args: { rowId: row.id } } }] },
  }));
  return {
    columnsJson: JSON.stringify([
      { id: "name", label: "Name", sortable: true },
      { id: "kind", label: "Kind", sortable: true },
      { id: "count", label: "Count", sortable: false },
      { id: "remove", label: "", sortable: false },
    ]),
    rowsJson: JSON.stringify(rows),
    selectionJson: JSON.stringify({ selectedIds: state.selectedIds }),
    sortJson: state.sort ? JSON.stringify(state.sort) : undefined,
  };
}
//#endregion SceneNode

//#region StoryHost
function TableStoryHost({ initialRows }: { readonly initialRows: readonly StoryTableRow[] }): ReactElement {
  const [state, setState] = useState<StoryTableState>(() => ({ rows: initialRows, selectedIds: [], sort: null }));

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setState((current) => reduceStoryTableAction(current, descriptor));
  }, []);

  const node: UiComponentSceneNode = useMemo(
    () => ({ type: "componentScene", surfaceId: "table.story.overview", controllerId: STORY_TABLE_CONTROLLER_ID, componentKind: "table", table: buildStoryTableScene(state) }),
    [state],
  );
  const debug = useMemo(() => JSON.stringify({ order: state.rows.map((row) => row.id), selectedIds: state.selectedIds, sort: state.sort }), [state]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <TableHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="table-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}

/** @emoji 🕳️ `TableHost` with an absent `table` scene — exercises the `emptySceneLabel` fallback path with zero fixture setup. */
function TableStoryEmptyHost(): ReactElement {
  const node: UiComponentSceneNode = { type: "componentScene", surfaceId: "table.story.empty", controllerId: STORY_TABLE_CONTROLLER_ID, componentKind: "table" };
  return (
    <div style={{ height: "100%", width: "100%" }}>
      <TableHost node={node} onAction={() => undefined} />
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌hosts/TableHost",
  component: TableStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof TableStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🧮 Sortable Name/Kind columns, a stepper cell (`adjustCount`), and a row-action delete button (`removeRow`) — click a header to re-sort, the stepper to change count, the row to select it. */
export const SortableWithActions: Story = {
  args: { initialRows: STORY_TABLE_ROWS },
};

/** 🕳️ No `table` scene — the `emptySceneLabel` fallback. */
export const EmptyScene: Story = {
  render: () => <TableStoryEmptyHost />,
};
