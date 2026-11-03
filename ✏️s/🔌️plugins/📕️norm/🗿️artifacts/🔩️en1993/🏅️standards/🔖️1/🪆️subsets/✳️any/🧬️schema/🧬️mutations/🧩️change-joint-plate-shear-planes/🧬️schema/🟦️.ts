/** 🧩️ `change-joint-plate-shear-planes` wire twin: the leaf payload `ChangeJointPlateShearPlanes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateShearPlanes {
  id: string;
  newShearPlanes: number;
}

export const parseChangeJointPlateShearPlanes: NormWireReader<ChangeJointPlateShearPlanes> = normWireObject<ChangeJointPlateShearPlanes>({ id: normWireRequired(normWireString), newShearPlanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
