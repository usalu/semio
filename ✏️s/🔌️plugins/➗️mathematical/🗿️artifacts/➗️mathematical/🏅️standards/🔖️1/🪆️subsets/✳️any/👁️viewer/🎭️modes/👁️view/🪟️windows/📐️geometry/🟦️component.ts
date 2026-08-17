/** 📐️ Mathematical viewer — Geometry window: typed twin of `🦀️component.rs`'s view-model. Mirrors
 * the Rust `render(document: &MathematicalSnapshot) -> UiNode` boundary — a flat table of the point
 * cloud's coordinates, built on the framework `TableWindowKit` (no hull/centroid overlay, no
 * editable cell: the read-only counterpart of `✏️editor/…/📐️geometry/🟦️component.ts`). */

/** 👁️ The Geometry window's typed view-model — one row per point, in document order. */
export interface MathematicalGeometryTableViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: ["#", "x", "y"];
  rows: [string, string, string][];
}

export const MATH_VIEW_WINDOW_GEOMETRY = "framework.window.table" as const;
export const MATH_VIEW_BODY_GEOMETRY = "framework.window.table" as const;
