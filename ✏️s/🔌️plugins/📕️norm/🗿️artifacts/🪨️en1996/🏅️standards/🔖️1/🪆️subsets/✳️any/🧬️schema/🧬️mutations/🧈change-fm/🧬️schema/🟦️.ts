/** 🧈 `change-fm` wire twin: the leaf payload `ChangeFm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFm {
  id: string;
  newMortarStrengthPa: number;
}

export const parseChangeFm: NormWireReader<ChangeFm> = normWireObject<ChangeFm>({ id: normWireRequired(normWireString), newMortarStrengthPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
