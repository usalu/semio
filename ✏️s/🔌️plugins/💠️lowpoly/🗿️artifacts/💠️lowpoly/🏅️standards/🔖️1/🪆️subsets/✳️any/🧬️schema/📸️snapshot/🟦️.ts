/** 🧬️ Lowpoly snapshot schema — artifact-lane fields only. */
import type { LowpolyObject } from "../🟦️.ts";
import {parseLowpolyArtifact}from"../🟦️.ts";

export interface LowpolySnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objects: LowpolyObject[];
}
/** 🛂️ Validate precisely the persisted Lowpoly schema and ordered objects. */
export function parseLowpolySnapshot(value:unknown):LowpolySnapshot{return parseLowpolyArtifact(value);}
