/** ↩️ procedural3d create-widget/↩️inverse — mirror of the id-only delete-widget inverse builder. */
import { widgetId } from "../🦠️mutation/🟦️.ts";
import type { CreateWidget } from "../🦠️mutation/🟦️.ts";
import type { DeleteWidget } from "../../❌delete-widget/🦠️mutation/🟦️.ts";

export function inverse(payload: CreateWidget): DeleteWidget[] {
  return [{ id: widgetId(payload.widget) }];
}
