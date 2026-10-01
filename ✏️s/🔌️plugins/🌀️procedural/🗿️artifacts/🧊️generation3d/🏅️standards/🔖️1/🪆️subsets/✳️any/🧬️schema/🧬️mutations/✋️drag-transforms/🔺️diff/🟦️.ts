/** 🔺️ generation3d drag-transforms/🔺️diff — mirror of the offset composition one translate operator receives. */
import type { DragTransforms } from "../🦠️mutation/🟦️.ts";

export function diff(payload: DragTransforms, baseOffset: readonly [number, number, number] = [0, 0, 0]): [number, number, number] {
  return [baseOffset[0] + payload.dx, baseOffset[1] + payload.dy, baseOffset[2] + payload.dz];
}
