/** ↩️ generation3d drag-transforms/↩️inverse — mirror of the BASE operators restored whole. */
import type { DragTransforms } from "../🦠️mutation/🟦️.ts";
import type { UpdateWidget } from "../../🩹update-widget/🦠️mutation/🟦️.ts";
import type { Widget } from "../../🌱️create-widget/🦠️mutation/🟦️.ts";

export function inverse(payload: DragTransforms, baseOperators: readonly Widget[]): UpdateWidget[] {
  return payload.dx === 0 && payload.dy === 0 && payload.dz === 0 ? [] : baseOperators.map((widget) => ({ widget }));
}
