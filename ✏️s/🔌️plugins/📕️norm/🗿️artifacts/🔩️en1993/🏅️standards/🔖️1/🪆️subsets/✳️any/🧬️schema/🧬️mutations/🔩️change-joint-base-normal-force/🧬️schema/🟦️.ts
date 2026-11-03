/** 🔩️ `change-joint-base-normal-force` wire twin: the leaf payload `ChangeJointBaseNormalForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBaseNormalForce {
  id: string;
  newBaseNormalForce: number;
}

export const parseChangeJointBaseNormalForce: NormWireReader<ChangeJointBaseNormalForce> = normWireObject<ChangeJointBaseNormalForce>({ id: normWireRequired(normWireString), newBaseNormalForce: normWireRequired(normWireNumber) });
