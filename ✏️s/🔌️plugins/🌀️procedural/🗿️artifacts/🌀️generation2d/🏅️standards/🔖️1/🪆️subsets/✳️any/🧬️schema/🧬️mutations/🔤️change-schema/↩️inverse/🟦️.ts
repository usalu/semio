/** ↩️ generation2d change-schema inverse — mirrors `inverse()` (…/🔤️change-schema/↩️inverse/🦀️.rs): unconditionally restores the captured BASE schema id (the schema field always exists). */
import type { ChangeSchema } from "../🦠️mutation/🟦️.ts";

export function inverse(_payload: ChangeSchema, baseSchema: string): ChangeSchema[] {
  return [{ schema: baseSchema }];
}
