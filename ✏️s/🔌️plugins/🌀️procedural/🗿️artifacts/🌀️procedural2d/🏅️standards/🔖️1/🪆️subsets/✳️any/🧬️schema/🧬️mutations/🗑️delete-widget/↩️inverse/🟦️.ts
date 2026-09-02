/** ↩️ procedural2d delete-widget inverse — mirrors `inverse()` (…/🗑️delete-widget/↩️inverse/🦀️.rs): recreates the removed widget at its captured BASE index, or a no-op when the id was already absent. */
import type { DeleteWidget } from "../🦠️mutation/🟦️.ts";
import type { CreateWidget, Widget } from "../../🌱create-widget/🦠️mutation/🟦️.ts";

export function inverse(_payload: DeleteWidget, baseWidget: { index: number; widget: Widget } | undefined): CreateWidget[] {
  return baseWidget ? [{ index: baseWidget.index, widget: baseWidget.widget }] : [];
}
