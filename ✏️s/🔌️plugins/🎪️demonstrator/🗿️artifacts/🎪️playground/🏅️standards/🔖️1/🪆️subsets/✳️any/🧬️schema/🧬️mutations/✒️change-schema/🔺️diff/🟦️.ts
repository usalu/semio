/** 🔺️ demonstrator playground change-schema/🔺️diff — mirror of the single-field schema patch
 * delta builder. */
import type { ChangeSchema } from "../🟦️.ts";

export function diff(payload: ChangeSchema): { schema: string } {
  return { schema: payload.new_schema };
}
