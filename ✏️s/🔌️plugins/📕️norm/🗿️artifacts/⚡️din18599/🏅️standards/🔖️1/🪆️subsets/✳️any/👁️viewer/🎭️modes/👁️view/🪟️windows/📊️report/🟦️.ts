/** 📊️ DIN V 18599 viewer — Report window: typed twin of `🦀️.rs`'s view-model. Read-only
 * mirror of the `TableWindowKit` payload (columns/rows of strings) — no mutation-shaped fields,
 * matching the viewer's `ViewEmit`-only contract. */

export interface Din18599ViewReportRow {
  clause: string;
  status: string;
  utilization: string;
  message: string;
}

export interface Din18599ViewReportViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: Din18599ViewReportRow[];
}

export const DIN18599_VIEW_REPORT_WINDOW_KIND_ID = "framework.window.table" as const;
export const DIN18599_VIEW_REPORT_BODY_KEY = "framework.window.table" as const;
