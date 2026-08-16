/** 📊️ DIN EN 16798 viewer — Report window: typed twin of `🦀️component.rs`'s view-model. Read-only
 * mirror of the `TableWindowKit` payload (columns/rows of strings) — no mutation-shaped fields,
 * matching the viewer's `ViewEmit`-only contract. */

export interface Din16798ViewReportRow {
  clause: string;
  status: string;
  utilization: string;
  message: string;
}

export interface Din16798ViewReportViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: Din16798ViewReportRow[];
}

export const DIN16798_VIEW_REPORT_WINDOW_KIND_ID = "framework.window.table" as const;
export const DIN16798_VIEW_REPORT_BODY_KEY = "framework.window.table" as const;
