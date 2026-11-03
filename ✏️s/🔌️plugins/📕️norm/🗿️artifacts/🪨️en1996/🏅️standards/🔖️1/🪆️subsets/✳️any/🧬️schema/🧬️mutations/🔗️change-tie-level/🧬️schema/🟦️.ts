/** 🔗️ `change-tie-level` wire twin: the leaf payload `ChangeTieLevel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieLevel {
  wallId: string;
  id: string;
  newLevelM: number;
}

export const parseChangeTieLevel: NormWireReader<ChangeTieLevel> = normWireObject<ChangeTieLevel>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newLevelM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
