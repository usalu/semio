/** 🔺️ procedural3d create-widget/🔺️diff — mirror of the append-only widgets-set delta builder. */
import type { CreateWidget, Widget } from "../🦠️mutation/🟦️.ts";

export function diff(payload: CreateWidget): { widgets: { removed: string[]; set: Array<[number, Widget]> } } {
  return { widgets: { removed: [], set: [[payload.index, payload.widget]] } };
}
