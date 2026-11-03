/** 🪚️ `change-chase-length` wire twin: the leaf payload `ChangeChaseLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeChaseLength {
  id: string;
  newLengthM: number;
}

export const parseChangeChaseLength: NormWireReader<ChangeChaseLength> = normWireObject<ChangeChaseLength>({ id: normWireRequired(normWireString), newLengthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
