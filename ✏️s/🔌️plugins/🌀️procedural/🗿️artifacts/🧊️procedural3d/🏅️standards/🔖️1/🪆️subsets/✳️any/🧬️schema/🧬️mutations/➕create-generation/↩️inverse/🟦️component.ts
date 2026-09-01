/** ↩️ procedural3d create-generation/↩️inverse — mirror of the id-only delete-generation inverse builder. */
import type { CreateGeneration } from "../🦠️mutation/🟦️component.ts";
import type { DeleteGeneration } from "../../🗑delete-generation/🦠️mutation/🟦️component.ts";

export function inverse(payload: CreateGeneration): DeleteGeneration[] {
  return [{ id: payload.generation.id }];
}
