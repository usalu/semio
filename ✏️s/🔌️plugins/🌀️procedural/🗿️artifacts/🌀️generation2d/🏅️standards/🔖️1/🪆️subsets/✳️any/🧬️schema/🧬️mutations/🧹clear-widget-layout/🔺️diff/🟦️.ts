/** 🔺️ generation2d clear-widget-layout diff — mirrors `diff()` (…/🧹clear-widget-layout/🔺️diff/🦀️.rs), a sparse id-keyed removal from the fixture's layout collection. */
import type { ClearWidgetLayout } from "../🦠️mutation/🟦️.ts";

export interface ClearWidgetLayoutDiff {
  layout: { removed: string[]; set: never[] };
}

export function diff(payload: ClearWidgetLayout): ClearWidgetLayoutDiff {
  return { layout: { removed: [payload.id], set: [] } };
}
