/** ↩️ procedural3d change-schema/↩️inverse — mirror of the self-inverse pre-state schema restore. */
import type { ChangeSchema } from "../🦠️mutation/🟦️component.ts";

export function inverse(_payload: ChangeSchema, baseSchema: string): ChangeSchema[] {
  return [{ newSchema: baseSchema }];
}
