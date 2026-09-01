/** ↩️ procedural2d change-schema inverse — mirrors `inverse()` (…/🔤change-schema/↩️inverse/🦀️component.rs): unconditionally restores the captured BASE schema id (the schema field always exists). */
import type { ChangeSchema } from "../🦠️mutation/🟦️component.ts";

export function inverse(_payload: ChangeSchema, baseSchema: string): ChangeSchema[] {
  return [{ schema: baseSchema }];
}
