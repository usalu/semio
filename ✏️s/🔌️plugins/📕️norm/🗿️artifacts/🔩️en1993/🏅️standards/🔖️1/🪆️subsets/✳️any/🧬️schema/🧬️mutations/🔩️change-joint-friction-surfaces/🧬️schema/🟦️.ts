/** 🔩️ `change-joint-friction-surfaces` wire twin: the leaf payload `ChangeJointFrictionSurfaces`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointFrictionSurfaces {
  id: string;
  newFrictionSurfaces: number;
}

export const parseChangeJointFrictionSurfaces: NormWireReader<ChangeJointFrictionSurfaces> = normWireObject<ChangeJointFrictionSurfaces>({ id: normWireRequired(normWireString), newFrictionSurfaces: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
