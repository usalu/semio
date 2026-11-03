/** 🧩️ `change-joint-plate-pitch` wire twin: the leaf payload `ChangeJointPlatePitch`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlatePitch {
  id: string;
  newPitch: number;
}

export const parseChangeJointPlatePitch: NormWireReader<ChangeJointPlatePitch> = normWireObject<ChangeJointPlatePitch>({ id: normWireRequired(normWireString), newPitch: normWireRequired(normWireNumber) });
