/** ↩️ procedural2d move-widget inverse — mirrors `inverse()` (…/📍move-widget/↩️inverse/🦀️component.rs): moves back to the captured BASE layout when the widget already had one, or clears the layout entry entirely when this move is what created it. */
import type { MoveWidget, WidgetLayout } from "../🦠️mutation/🟦️component.ts";
import type { ClearWidgetLayout } from "../../🧹clear-widget-layout/🦠️mutation/🟦️component.ts";

export function inverse(payload: MoveWidget, baseLayout: WidgetLayout | undefined): Array<MoveWidget | ClearWidgetLayout> {
  return baseLayout ? [{ id: payload.id, layout: baseLayout }] : [{ id: payload.id }];
}
