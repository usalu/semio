/** ↩️ generation3d move-nodes/↩️inverse — mirror of the absolute BASE positions restored. */
import type { MoveNodes } from "../🦠️mutation/🟦️.ts";
import type { MoveWidget, WidgetLayout } from "../../📍️move-widget/🦠️mutation/🟦️.ts";
import{binary64Value}from"../../../🟦️.ts";

export function inverse(payload: MoveNodes, baseLayout: Readonly<Record<string, WidgetLayout>>): MoveWidget[] {
  if (binary64Value(payload.dx) === 0 && binary64Value(payload.dy) === 0) return [];
  return payload.ids.filter((id) => baseLayout[id] !== undefined).map((id) => ({ id, layout: baseLayout[id] }));
}
