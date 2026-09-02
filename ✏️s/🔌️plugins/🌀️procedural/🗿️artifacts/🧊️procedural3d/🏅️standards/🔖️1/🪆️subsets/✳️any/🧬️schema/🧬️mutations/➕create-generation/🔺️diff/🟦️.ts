/** 🔺️ procedural3d create-generation/🔺️diff — mirror of the single `GenerationMutation::Add` op delta builder. */
import type { CreateGeneration, FormGeneration } from "../🦠️mutation/🟦️.ts";

export function diff(payload: CreateGeneration): { generation: { ops: Array<{ kind: "add"; generation: FormGeneration }> } } {
  return { generation: { ops: [{ kind: "add", generation: payload.generation }] } };
}
