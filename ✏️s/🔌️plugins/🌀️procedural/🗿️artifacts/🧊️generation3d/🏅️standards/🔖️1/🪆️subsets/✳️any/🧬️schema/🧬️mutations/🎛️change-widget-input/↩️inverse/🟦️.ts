/** ↩️ generation3d change-widget-input/↩️inverse — mirror of the BASE operator (or text source) restored whole. */
import type { ChangeWidgetInput } from "../🦠️mutation/🟦️.ts";
import type { UpdateWidget } from "../../🩹update-widget/🦠️mutation/🟦️.ts";
import type { Widget } from "../../🌱️create-widget/🦠️mutation/🟦️.ts";

export function inverse(_payload: ChangeWidgetInput, baseWidget: Widget | undefined): UpdateWidget[] {
  return baseWidget === undefined ? [] : [{ widget: baseWidget }];
}
