/** 🧱️ `change-fatigue-stress-cycles` wire twin: the leaf payload `ChangeFatigueStressCycles`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressCycles {
  id: string;
  newCycles: number;
}

export const parseChangeFatigueStressCycles: NormWireReader<ChangeFatigueStressCycles> = normWireObject<ChangeFatigueStressCycles>({ id: normWireRequired(normWireString), newCycles: normWireRequired(normWireNumber) });
