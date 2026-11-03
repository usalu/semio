/** 🔩️ `change-joint-moment-demand` wire twin: the leaf payload `ChangeJointMomentDemand`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointMomentDemand {
  id: string;
  newMomentDemand: number;
}

export const parseChangeJointMomentDemand: NormWireReader<ChangeJointMomentDemand> = normWireObject<ChangeJointMomentDemand>({ id: normWireRequired(normWireString), newMomentDemand: normWireRequired(normWireNumber) });
