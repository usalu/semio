/** ➡️ `change-stud-diameter-m` wire twin: the leaf payload `ChangeStudDiameterM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStudDiameterM {
  id: string;
  newDiameterM: number;
}

export const parseChangeStudDiameterM: NormWireReader<ChangeStudDiameterM> = normWireObject<ChangeStudDiameterM>({ id: normWireRequired(normWireString), newDiameterM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
