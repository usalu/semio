/** ↩️ procedural2d clear-widget-layout inverse — mirrors `inverse()` (…/🧹clear-widget-layout/↩️inverse/🦀️component.rs): restores the captured BASE layout entry, or a no-op when the widget already had none. */
import type { ClearWidgetLayout } from "../🦠️mutation/🟦️component.ts";
import type { MoveWidget, WidgetLayout } from "../../📍move-widget/🦠️mutation/🟦️component.ts";

export function inverse(payload: ClearWidgetLayout, baseLayout: WidgetLayout | undefined): MoveWidget[] {
  return baseLayout ? [{ id: payload.id, layout: baseLayout }] : [];
}
