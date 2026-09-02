/** 🔺️ procedural3d change-schema/🔺️diff — mirror of the whole-artifact schema-field delta builder. */
import type { ChangeSchema } from "../🦠️mutation/🟦️.ts";

export function diff(payload: ChangeSchema): { schema: string } {
  return { schema: payload.newSchema };
}
