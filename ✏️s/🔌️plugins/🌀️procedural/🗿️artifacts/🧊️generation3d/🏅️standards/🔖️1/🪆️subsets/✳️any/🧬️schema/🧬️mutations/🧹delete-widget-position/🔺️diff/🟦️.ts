/** 🔺️ generation3d delete-widget-position/🔺️diff — mirror of the id-only layout-removed delta builder. */
import type { DeleteWidgetPosition } from "../🦠️mutation/🟦️.ts";
import type { WidgetLayout } from "../../📍move-widget/🦠️mutation/🟦️.ts";

export function diff(payload: DeleteWidgetPosition): { layout: { removed: string[]; set: Array<[string, WidgetLayout]> } } {
  return { layout: { removed: [payload.id], set: [] } };
}
