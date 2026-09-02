/** 🔺️ procedural3d change-generation-value/🔺️diff — mirror of the single `GenerationMutation::UpdateValues` op delta builder. */
import type { ChangeGenerationValue } from "../🦠️mutation/🟦️.ts";

export function diff(payload: ChangeGenerationValue): { generation: { ops: Array<{ kind: "updateValues"; id: string; questionId: string; value: unknown }> } } {
  return { generation: { ops: [{ kind: "updateValues", id: payload.id, questionId: payload.questionId, value: payload.newValue }] } };
}
