/** 🔩️ `change-joint-shear-planes` wire twin: the leaf payload `ChangeJointShearPlanes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointShearPlanes {
  id: string;
  newShearPlanes: number;
}

export const parseChangeJointShearPlanes: NormWireReader<ChangeJointShearPlanes> = normWireObject<ChangeJointShearPlanes>({ id: normWireRequired(normWireString), newShearPlanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
