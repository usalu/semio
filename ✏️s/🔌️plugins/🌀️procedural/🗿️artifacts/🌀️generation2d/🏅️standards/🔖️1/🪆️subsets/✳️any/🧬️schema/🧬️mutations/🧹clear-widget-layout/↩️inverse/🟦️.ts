/** ↩️ generation2d clear-widget-layout inverse — mirrors `inverse()` (…/🧹clear-widget-layout/↩️inverse/🦀️.rs): restores the captured BASE layout entry, or a no-op when the widget already had none. */
import type { ClearWidgetLayout } from "../🦠️mutation/🟦️.ts";
import type { MoveWidget, WidgetLayout } from "../../📍move-widget/🦠️mutation/🟦️.ts";

export function inverse(payload: ClearWidgetLayout, baseLayout: WidgetLayout | undefined): MoveWidget[] {
  return baseLayout ? [{ id: payload.id, layout: baseLayout }] : [];
}
