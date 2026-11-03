/** 🔩️ `change-row-bolts` wire twin: the leaf payload `ChangeRowBolts`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowBolts {
  id: string;
  newBolts: number;
}

export const parseChangeRowBolts: NormWireReader<ChangeRowBolts> = normWireObject<ChangeRowBolts>({ id: normWireRequired(normWireString), newBolts: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
