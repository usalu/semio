/** 🔩️ `change-joint-stiffness-ratio` wire twin: the leaf payload `ChangeJointStiffnessRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointStiffnessRatio {
  id: string;
  newStiffnessRatio: number;
}

export const parseChangeJointStiffnessRatio: NormWireReader<ChangeJointStiffnessRatio> = normWireObject<ChangeJointStiffnessRatio>({ id: normWireRequired(normWireString), newStiffnessRatio: normWireRequired(normWireNumber) });
