/** ↩️ generation3d change-generation-preview/↩️inverse — mirror of the BASE-preview restore inverse builder. */
import type { ChangeGenerationPreview } from "../🦠️mutation/🟦️.ts";

export function inverse(payload: ChangeGenerationPreview, basePreview: string | null): ChangeGenerationPreview[] {
  return basePreview === payload.text ? [] : [{ text: basePreview }];
}
