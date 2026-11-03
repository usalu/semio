/** 📏️ `change-column-length-m` wire twin: the leaf payload `ChangeColumnLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnLengthM {
  id: string;
  newLengthM: number;
}

export const parseChangeColumnLengthM: NormWireReader<ChangeColumnLengthM> = normWireObject<ChangeColumnLengthM>({ id: normWireRequired(normWireString), newLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
