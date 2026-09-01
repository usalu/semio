/** 🔺️ procedural2d change-schema diff — mirrors `diff()` (…/🔤change-schema/🔺️diff/🦀️component.rs), a scalar-field write on the fixture's schema id. */
import type { ChangeSchema } from "../🦠️mutation/🟦️component.ts";

export interface ChangeSchemaDiff {
  schema: string;
}

export function diff(payload: ChangeSchema): ChangeSchemaDiff {
  return { schema: payload.schema };
}
