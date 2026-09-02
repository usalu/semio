/** 📊️ VDI 3805 viewer — Report window: typed twin of `🦀️.rs`'s view-model. Read-only
 * mirror of the `TableWindowKit` payload (columns/rows of strings) — no mutation-shaped fields,
 * matching the viewer's `ViewEmit`-only contract. */

export interface Vdi3805ViewReportRow {
  clause: string;
  status: string;
  utilization: string;
  message: string;
}

export interface Vdi3805ViewReportViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: Vdi3805ViewReportRow[];
}

export const VDI3805_VIEW_REPORT_WINDOW_KIND_ID = "framework.window.table" as const;
export const VDI3805_VIEW_REPORT_BODY_KEY = "framework.window.table" as const;
