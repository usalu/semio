// #region 🧲️Header
// 💻️ .storybook/stories/remodel/Report.stories.tsx
// Specs: Component-level coverage for remodel's `remodeling-report` window — the `analyze` mode's Table surface
// (`✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs`), which `🗣️Interpreter` resolves to `TableHost` for
// `componentKind: "table"`.
// Summary: Mounts `TableHost` against the `TableScene` `report_table_json` builds for the config's
// `report_table` selection, with the six dataset names reachable from a real `setReportTable` dispatch through
// the story reducer (the same `SetReportTable` command the plugin's own dataset switcher emits). `TableHost` is
// a pure declarative renderer over `columnsJson`/`rowsJson`, so sorting and row selection round-trip for real
// with no wasm. Columns are marked `sortable` by the projection so the sort is exercisable; the plugin's own
// column list omits that flag, which is noted in `../scene.ts`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { TableHost } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { REMODEL_DEFAULT_CONFIG, REMODEL_POPULATED_SCENE } from "./fixture";
import { REMODEL_EDITOR_CONTROLLER_ID, reduceRemodelStoryAction, remodelReportTableJson, remodelWindowNode } from "./scene";

/** 📊️ Every dataset name `report_table_json` matches, plus the fallback probe — `"nonsense"` must land on the frame list. */
const REPORT_TABLES = ["frames", "cameras", "tracks", "gcps", "qcStages", "matches", "nonsense"] as const;

//#region StoryHost
function RemodelReportStoryHost({ initialTable }: { readonly initialTable: string }): ReactElement {
  const [config, setConfig] = useState({ ...REMODEL_DEFAULT_CONFIG, reportTable: initialTable });
  const onAction = useCallback((descriptor: ActionDescriptor): void => setConfig((current) => reduceRemodelStoryAction(current, descriptor)), []);
  const node = useMemo(() => remodelWindowNode("remodeling-report", REMODEL_POPULATED_SCENE, config), [config]);
  const debug = useMemo(() => {
    const { columnsJson, rowsJson } = remodelReportTableJson(REMODEL_POPULATED_SCENE, config.reportTable);
    return JSON.stringify({
      windowKindId: "remodeling-report",
      surfaceKind: "table",
      reportTable: config.reportTable,
      columnLabels: (JSON.parse(columnsJson) as { readonly label: string }[]).map((column) => column.label),
      rowCount: (JSON.parse(rowsJson) as unknown[]).length,
    });
  }, [config]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", gap: 6, padding: 4, flexWrap: "wrap" }}>
        {REPORT_TABLES.map((table) => (
          <button key={table} type="button" data-testid={`remodel-report-table-${table}`} onClick={() => onAction({ controllerId: REMODEL_EDITOR_CONTROLLER_ID, action: "setReportTable", args: { table } })}>
            {table}
          </button>
        ))}
      </div>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <TableHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="remodel-report-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "📸️remodel📊️Report",
  component: RemodelReportStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof RemodelReportStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🎞️ `RemodelingConfig::default()`'s `"frames"` dataset — three rows, one per (stream, frame) pair of the populated document. */
export const FrameList: Story = { args: { initialTable: "frames" } };

/** 📷️ `"cameras"` — the two calibrated cameras, including `cam-b`'s null `RMS (px)`. */
export const Cameras: Story = { args: { initialTable: "cameras" } };

/** 🎯️ `"gcps"` — `Ridge` (no observations) and `Corner` (one), with their world coordinates. */
export const GroundControlPoints: Story = { args: { initialTable: "gcps" } };

/** 🏃️ `"tracks"` — the single `track-a` motion track, its `Debug`-formatted class and mean speed. */
export const Tracks: Story = { args: { initialTable: "tracks" } };

/** 🚦️ `"qcStages"` — the one-row stage/status projection of the document's mid-run `bundle-adjusting` job. */
export const QcStages: Story = { args: { initialTable: "qcStages" } };

/** 🗒️ `"matches"` — the documented gap row: pairwise match data is reconstruction-runtime scratch and is never distilled into durable document state. */
export const Matches: Story = { args: { initialTable: "matches" } };
