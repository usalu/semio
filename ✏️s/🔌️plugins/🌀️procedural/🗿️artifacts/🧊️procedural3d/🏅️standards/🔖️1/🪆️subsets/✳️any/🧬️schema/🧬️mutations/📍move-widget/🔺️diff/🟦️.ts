/** 🔺️ procedural3d move-widget/🔺️diff — mirror of the id-keyed layout-set delta builder. */
import type { MoveWidget, WidgetLayout } from "../🦠️mutation/🟦️.ts";

export function diff(payload: MoveWidget): { layout: { removed: string[]; set: Array<[string, WidgetLayout]> } } {
  return { layout: { removed: [], set: [[payload.id, payload.layout]] } };
}
