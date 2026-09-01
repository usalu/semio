/** 🔺️ procedural3d delete-generation/🔺️diff — mirror of the single `GenerationMutation::Remove` op delta builder. */
import type { DeleteGeneration } from "../🦠️mutation/🟦️component.ts";

export function diff(payload: DeleteGeneration): { generation: { ops: Array<{ kind: "remove"; id: string }> } } {
  return { generation: { ops: [{ kind: "remove", id: payload.id }] } };
}
