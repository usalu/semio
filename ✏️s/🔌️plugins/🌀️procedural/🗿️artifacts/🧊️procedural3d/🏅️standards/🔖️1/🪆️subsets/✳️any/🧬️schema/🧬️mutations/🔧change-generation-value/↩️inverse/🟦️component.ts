/** ↩️ procedural3d change-generation-value/↩️inverse — mirror of the BASE-lookup old-value (defaulting to null) inverse. */
import type { ChangeGenerationValue } from "../🦠️mutation/🟦️component.ts";

export function inverse(payload: ChangeGenerationValue, baseValue: unknown): ChangeGenerationValue[] {
  return baseValue === undefined ? [] : [{ id: payload.id, questionId: payload.questionId, newValue: baseValue }];
}
