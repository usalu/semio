/** 🔺️ generation3d move-nodes/🔺️diff — mirror of the layout-set delta: every placed widget moved by the offset. */
import type { MoveNodes } from "../🦠️mutation/🟦️.ts";
import type { WidgetLayout } from "../../📍️move/🦠️mutation/🟦️.ts";
import{binary64,binary64Value}from"../../../🟦️.ts";

export function diff(payload: MoveNodes, baseLayout: Readonly<Record<string, WidgetLayout>>): { layout: { removed: string[]; set: Array<[string, WidgetLayout]> } } {
  const set = payload.ids.filter((id) => baseLayout[id] !== undefined).map((id): [string, WidgetLayout] => [id, { x: binary64(binary64Value(baseLayout[id].x) + binary64Value(payload.dx)), y: binary64(binary64Value(baseLayout[id].y) + binary64Value(payload.dy)) }]);
  return { layout: { removed: [], set } };
}
