/** 🔺️ generation2d change-schema diff — mirrors `diff()` (…/🔤change-schema/🔺️diff/🦀️.rs), a scalar-field write on the fixture's schema id. */
import type { ChangeSchema } from "../🦠️mutation/🟦️.ts";

export interface ChangeSchemaDiff {
  schema: string;
}

export function diff(payload: ChangeSchema): ChangeSchemaDiff {
  return { schema: payload.schema };
}
