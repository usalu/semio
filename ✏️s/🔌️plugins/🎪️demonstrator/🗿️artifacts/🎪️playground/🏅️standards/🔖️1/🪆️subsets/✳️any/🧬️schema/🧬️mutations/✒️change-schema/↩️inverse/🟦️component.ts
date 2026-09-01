/** ↩️ demonstrator playground change-schema/↩️inverse — mirror of the BASE-lookup old-schema
 * restore inverse. */
import type { ChangeSchema } from "../🟦️component.ts";

export function inverse(_payload: ChangeSchema, baseSchema: string): [ChangeSchema] {
  return [{ new_schema: baseSchema }];
}
