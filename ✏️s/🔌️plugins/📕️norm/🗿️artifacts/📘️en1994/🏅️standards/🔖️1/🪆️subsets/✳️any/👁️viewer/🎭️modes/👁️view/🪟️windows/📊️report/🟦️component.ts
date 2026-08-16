/** 📊️ EN 1994 viewer — Report window: typed twin of `🦀️component.rs`'s view-model. Read-only
 * mirror of the `TableWindowKit` payload (columns/rows of strings) — no mutation-shaped fields,
 * matching the viewer's `ViewEmit`-only contract. */

export interface En1994ViewReportRow {
  clause: string;
  status: string;
  utilization: string;
  message: string;
}

export interface En1994ViewReportViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: En1994ViewReportRow[];
}

export const EN1994_VIEW_REPORT_WINDOW_KIND_ID = "framework.window.table" as const;
export const EN1994_VIEW_REPORT_BODY_KEY = "framework.window.table" as const;
