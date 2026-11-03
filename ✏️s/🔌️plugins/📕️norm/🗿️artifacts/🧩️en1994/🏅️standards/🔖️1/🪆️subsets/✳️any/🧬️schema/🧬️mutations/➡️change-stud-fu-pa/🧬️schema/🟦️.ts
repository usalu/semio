/** ➡️ `change-stud-fu-pa` wire twin: the leaf payload `ChangeStudFUPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStudFUPa {
  id: string;
  newFUPa: number;
}

export const parseChangeStudFUPa: NormWireReader<ChangeStudFUPa> = normWireObject<ChangeStudFUPa>({ id: normWireRequired(normWireString), newFUPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
