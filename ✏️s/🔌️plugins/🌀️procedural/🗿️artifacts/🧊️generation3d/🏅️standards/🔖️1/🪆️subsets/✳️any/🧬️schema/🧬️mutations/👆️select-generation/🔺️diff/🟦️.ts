/** 🔺️ generation3d select-generation/🔺️diff — mirror of the play-state selection delta builder. */
import type { SelectGeneration } from "../🦠️mutation/🟦️.ts";

export function diff(payload: SelectGeneration): { generation: { selectedGenerationId: string | null } } {
  return { generation: { selectedGenerationId: payload.generationId } };
}
