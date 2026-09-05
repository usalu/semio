/** 📊️ EN 1998 viewer — Report window: typed twin of `🦀️.rs`'s view-model. Read-only
 * mirror of the `TableWindowKit` payload (columns/rows of strings) — no mutation-shaped fields,
 * matching the viewer's `ViewEmit`-only contract. */

export interface En1998ViewReportRow {
  clause: string;
  status: string;
  utilization: string;
  message: string;
}

export interface En1998ViewReportViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: En1998ViewReportRow[];
}

export const EN1998_VIEW_REPORT_WINDOW_KIND_ID = "framework.window.table" as const;
export const EN1998_VIEW_REPORT_BODY_KEY = "framework.window.table" as const;
