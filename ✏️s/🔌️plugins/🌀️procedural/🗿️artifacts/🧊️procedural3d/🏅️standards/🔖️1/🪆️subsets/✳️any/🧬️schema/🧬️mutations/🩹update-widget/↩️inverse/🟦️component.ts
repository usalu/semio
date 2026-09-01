/** ↩️ procedural3d update-widget/↩️inverse — mirror of the BASE-lookup whole-body restore inverse. */
import type { UpdateWidget } from "../🦠️mutation/🟦️component.ts";
import type { Widget } from "../../🌱create-widget/🦠️mutation/🟦️component.ts";

export function inverse(_payload: UpdateWidget, baseWidget: Widget | undefined): UpdateWidget[] {
  return baseWidget === undefined ? [] : [{ widget: baseWidget }];
}
