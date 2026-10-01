/** ↩️ generation3d rename-generation/↩️inverse — mirror of the BASE-lookup old-name inverse builder. */
import type { RenameGeneration } from "../🦠️mutation/🟦️.ts";

export function inverse(payload: RenameGeneration, baseName: string | undefined): RenameGeneration[] {
  return baseName === undefined ? [] : [{ id: payload.id, newName: baseName }];
}
