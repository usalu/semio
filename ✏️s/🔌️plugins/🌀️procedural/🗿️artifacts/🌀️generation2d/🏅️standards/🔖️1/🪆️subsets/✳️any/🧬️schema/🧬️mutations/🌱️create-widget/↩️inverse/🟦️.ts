/** ↩️ generation2d create-widget inverse — mirrors `inverse()` (…/🌱️create-widget/↩️inverse/🦀️.rs): always one `delete-widget` for the created widget's id, read off the payload's own JSON-blob widget (no BASE lookup needed). */
import type { CreateWidget } from "../🦠️mutation/🟦️.ts";
import type { DeleteWidget } from "../../🗑️delete-widget/🦠️mutation/🟦️.ts";

export function inverse(payload: CreateWidget): DeleteWidget[] {
  return [{ id: (JSON.parse(payload.widget) as { id: string }).id }];
}
