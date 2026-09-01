/** 🔺️ procedural3d update-widget/🔺️diff — mirror of the whole-body widget patch delta builder. */
import type { UpdateWidget } from "../🦠️mutation/🟦️component.ts";
import type { Widget } from "../../🌱create-widget/🦠️mutation/🟦️component.ts";

export function diff(payload: UpdateWidget): { widgets: { removed: string[]; set: Array<[number, Widget]> } } {
  return { widgets: { removed: [], set: [[0, payload.widget]] } };
}
