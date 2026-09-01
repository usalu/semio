/** 🔺️ procedural2d delete-widget diff — mirrors `diff()` (…/🗑️delete-widget/🔺️diff/🦀️component.rs), a sparse id-keyed removal from the fixture's widget collection. */
import type { DeleteWidget } from "../🦠️mutation/🟦️component.ts";

export interface DeleteWidgetDiff {
  widgets: { removed: string[]; set: never[] };
}

export function diff(payload: DeleteWidget): DeleteWidgetDiff {
  return { widgets: { removed: [payload.id], set: [] } };
}
