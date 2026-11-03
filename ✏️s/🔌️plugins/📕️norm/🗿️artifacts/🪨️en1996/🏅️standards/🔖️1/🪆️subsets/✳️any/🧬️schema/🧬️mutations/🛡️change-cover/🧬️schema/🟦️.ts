/** 🛡️ `change-cover` wire twin: the leaf payload `ChangeCover`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCover {
  id: string;
  newCoverM: number;
}

export const parseChangeCover: NormWireReader<ChangeCover> = normWireObject<ChangeCover>({ id: normWireRequired(normWireString), newCoverM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
