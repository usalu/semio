/** 🔩️ `change-joint-friction-mu` wire twin: the leaf payload `ChangeJointFrictionMu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointFrictionMu {
  id: string;
  newFrictionMu: number;
}

export const parseChangeJointFrictionMu: NormWireReader<ChangeJointFrictionMu> = normWireObject<ChangeJointFrictionMu>({ id: normWireRequired(normWireString), newFrictionMu: normWireRequired(normWireNumber) });
