/** ➡️ `change-stud-xm` wire twin: the leaf payload `ChangeStudXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStudXM {
  id: string;
  newXM: number;
}

export const parseChangeStudXM: NormWireReader<ChangeStudXM> = normWireObject<ChangeStudXM>({ id: normWireRequired(normWireString), newXM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
