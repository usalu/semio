/** 🔺️ procedural3d delete-widget/🔺️diff — mirror of the id-only widgets-removed delta builder. */
import type { DeleteWidget } from "../🦠️mutation/🟦️component.ts";
import type { Widget } from "../../🌱create-widget/🦠️mutation/🟦️component.ts";

export function diff(payload: DeleteWidget): { widgets: { removed: string[]; set: Array<[number, Widget]> } } {
  return { widgets: { removed: [payload.id], set: [] } };
}
