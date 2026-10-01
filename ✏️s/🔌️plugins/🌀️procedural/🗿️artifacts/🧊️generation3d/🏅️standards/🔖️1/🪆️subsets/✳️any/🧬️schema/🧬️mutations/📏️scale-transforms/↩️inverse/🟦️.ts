/** ↩️ generation3d scale-transforms/↩️inverse — mirror of the BASE operators restored whole. */
import type { ScaleTransforms } from "../🦠️mutation/🟦️.ts";
import type { UpdateWidget } from "../../🩹update-widget/🦠️mutation/🟦️.ts";
import type { Widget } from "../../🌱️create-widget/🦠️mutation/🟦️.ts";

export function inverse(payload: ScaleTransforms, baseOperators: readonly Widget[]): UpdateWidget[] {
  return payload.sx === 1 && payload.sy === 1 && payload.sz === 1 ? [] : baseOperators.map((widget) => ({ widget }));
}
