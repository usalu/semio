/** 🔺️ procedural2d move-widget diff — mirrors `diff()` (…/📍move-widget/🔺️diff/🦀️.rs), a sparse id-keyed upsert into the fixture's layout collection. */
import type { MoveWidget, WidgetLayout } from "../🦠️mutation/🟦️.ts";

export interface MoveWidgetDiff {
  layout: { removed: string[]; set: Array<[string, WidgetLayout]> };
}

export function diff(payload: MoveWidget): MoveWidgetDiff {
  return { layout: { removed: [], set: [[payload.id, payload.layout]] } };
}
