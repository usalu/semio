/** 🔺️ procedural3d rename-generation/🔺️diff — mirror of the single `GenerationMutation::Rename` op delta builder. */
import type { RenameGeneration } from "../🦠️mutation/🟦️.ts";

export function diff(payload: RenameGeneration): { generation: { ops: Array<{ kind: "rename"; id: string; name: string }> } } {
  return { generation: { ops: [{ kind: "rename", id: payload.id, name: payload.newName }] } };
}
