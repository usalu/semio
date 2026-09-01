/** ↩️ procedural3d delete-generation/↩️inverse — mirror of the BASE-lookup recreate-generation inverse. */
import type { DeleteGeneration } from "../🦠️mutation/🟦️component.ts";
import type { CreateGeneration, FormGeneration } from "../../➕create-generation/🦠️mutation/🟦️component.ts";

export function inverse(_payload: DeleteGeneration, baseGeneration: FormGeneration | undefined): CreateGeneration[] {
  return baseGeneration === undefined ? [] : [{ generation: baseGeneration }];
}
