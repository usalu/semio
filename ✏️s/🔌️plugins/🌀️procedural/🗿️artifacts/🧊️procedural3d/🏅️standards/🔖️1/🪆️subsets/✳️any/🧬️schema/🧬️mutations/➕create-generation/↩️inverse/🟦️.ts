/** ↩️ procedural3d create-generation/↩️inverse — mirror of the id-only delete-generation inverse builder. */
import type { CreateGeneration } from "../🦠️mutation/🟦️.ts";
import type { DeleteGeneration } from "../../🗑delete-generation/🦠️mutation/🟦️.ts";

export function inverse(payload: CreateGeneration): DeleteGeneration[] {
  return [{ id: payload.generation.id }];
}
