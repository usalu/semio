/** 🔺️ generation3d scale-transforms/🔺️diff — mirror of `compose_scale`: the operator's BASE factors times the payload
 * factors, refused when a product is zero or not finite. */
import type { ScaleTransforms } from "../🦠️mutation/🟦️.ts";

export function diff(payload: ScaleTransforms, baseFactors: readonly [number, number, number] = [1, 1, 1]): [number, number, number] | undefined {
  const next: [number, number, number] = [baseFactors[0] * payload.sx, baseFactors[1] * payload.sy, baseFactors[2] * payload.sz];
  return next.every((value) => Number.isFinite(value) && value !== 0) ? next : undefined;
}
