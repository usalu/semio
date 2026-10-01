/** ↩️ generation3d delete-widget-position/↩️inverse — mirror of the BASE-lookup recreate-position inverse. */
import type { DeleteWidgetPosition } from "../🦠️mutation/🟦️.ts";
import type { MoveWidget, WidgetLayout } from "../../📍️move/🦠️mutation/🟦️.ts";

export function inverse(payload: DeleteWidgetPosition, basePosition: WidgetLayout | undefined): MoveWidget[] {
  return basePosition === undefined ? [] : [{ id: payload.id, layout: basePosition }];
}
