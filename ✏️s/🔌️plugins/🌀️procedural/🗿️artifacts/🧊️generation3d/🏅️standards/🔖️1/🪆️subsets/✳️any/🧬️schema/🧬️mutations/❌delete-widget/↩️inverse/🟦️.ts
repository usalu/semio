/** ↩️ generation3d delete-widget/↩️inverse — mirror of the BASE-lookup recreate-widget inverse. */
import type { DeleteWidget } from "../🦠️mutation/🟦️.ts";
import type { CreateWidget, Widget } from "../../🌱create-widget/🦠️mutation/🟦️.ts";

export function inverse(_payload: DeleteWidget, baseWidget: { index: number; widget: Widget } | undefined): CreateWidget[] {
  return baseWidget === undefined ? [] : [{ index: baseWidget.index, widget: baseWidget.widget }];
}
