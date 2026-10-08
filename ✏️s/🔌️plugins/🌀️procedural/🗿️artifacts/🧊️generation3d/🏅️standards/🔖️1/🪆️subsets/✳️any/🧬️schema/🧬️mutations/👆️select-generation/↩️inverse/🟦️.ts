/** ↩️ generation3d select-generation/↩️inverse — mirror of the BASE-selection restore inverse builder. */
import type { SelectGeneration } from "../🦠️mutation/🟦️.ts";

export function inverse(payload: SelectGeneration, baseSelected: string | null, generationIds: readonly string[]): SelectGeneration[] {
  const missing = payload.generationId !== null && !generationIds.includes(payload.generationId);
  return baseSelected === payload.generationId || missing ? [] : [{ generationId: baseSelected }];
}
