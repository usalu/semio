/** ↩️ procedural3d move-widget/↩️inverse — mirror of the BASE-lookup old-position-or-delete inverse. */
import type { MoveWidget, WidgetLayout } from "../🦠️mutation/🟦️.ts";
import type { DeleteWidgetPosition } from "../../🧹delete-widget-position/🦠️mutation/🟦️.ts";

export function inverse(payload: MoveWidget, basePosition: WidgetLayout | undefined): Array<MoveWidget | DeleteWidgetPosition> {
  return basePosition === undefined ? [{ id: payload.id }] : [{ id: payload.id, layout: basePosition }];
}
