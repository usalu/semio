/** ↩️ generation3d rotate-transforms/↩️inverse — mirror of the BASE operators restored whole. */
import type { RotateTransforms } from "../🦠️mutation/🟦️.ts";
import type { UpdateWidget } from "../../🩹update-widget/🦠️mutation/🟦️.ts";
import type { Widget } from "../../🌱️create-widget/🦠️mutation/🟦️.ts";

export function inverse(payload: RotateTransforms, baseOperators: readonly Widget[]): UpdateWidget[] {
  return payload.angle === 0 ? [] : baseOperators.map((widget) => ({ widget }));
}
