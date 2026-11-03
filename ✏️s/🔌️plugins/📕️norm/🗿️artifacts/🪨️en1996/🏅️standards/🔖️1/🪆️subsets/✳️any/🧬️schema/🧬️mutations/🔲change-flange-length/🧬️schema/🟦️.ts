/** 🔲 `change-flange-length` wire twin: the leaf payload `ChangeFlangeLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFlangeLength {
  wallId: string;
  id: string;
  newLengthM: number;
}

export const parseChangeFlangeLength: NormWireReader<ChangeFlangeLength> = normWireObject<ChangeFlangeLength>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
