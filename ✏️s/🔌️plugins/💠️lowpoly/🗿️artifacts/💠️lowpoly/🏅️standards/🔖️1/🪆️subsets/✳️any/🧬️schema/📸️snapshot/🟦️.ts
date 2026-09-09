/** 🧬️ Lowpoly snapshot schema — artifact-lane fields only. */
import type { LowpolyObject } from "../🟦️.ts";

export interface LowpolySnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objects: LowpolyObject[];
}
