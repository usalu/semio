/** 🪢️ `change-bar-length` wire twin: the leaf payload `ChangeBarLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarLength {
  wallId: string;
  id: string;
  newLengthM: number;
}

export const parseChangeBarLength: NormWireReader<ChangeBarLength> = normWireObject<ChangeBarLength>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
