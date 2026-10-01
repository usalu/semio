/** 🔺️ generation3d move-nodes/🔺️diff — mirror of the layout-set delta: every placed widget moved by the offset. */
import type { MoveNodes } from "../🦠️mutation/🟦️.ts";
import type { WidgetLayout } from "../../📍️move/🦠️mutation/🟦️.ts";

export function diff(payload: MoveNodes, baseLayout: Readonly<Record<string, WidgetLayout>>): { layout: { removed: string[]; set: Array<[string, WidgetLayout]> } } {
  const set = payload.ids.filter((id) => baseLayout[id] !== undefined).map((id): [string, WidgetLayout] => [id, { x: baseLayout[id].x + payload.dx, y: baseLayout[id].y + payload.dy }]);
  return { layout: { removed: [], set } };
}
