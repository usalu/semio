/** 🔺️ generation3d change-generation-preview/🔺️diff — mirror of the play-state preview delta builder. */
import type { ChangeGenerationPreview } from "../🦠️mutation/🟦️.ts";

export function diff(payload: ChangeGenerationPreview): { generation: { previewText: string | null } } {
  return { generation: { previewText: payload.text } };
}
