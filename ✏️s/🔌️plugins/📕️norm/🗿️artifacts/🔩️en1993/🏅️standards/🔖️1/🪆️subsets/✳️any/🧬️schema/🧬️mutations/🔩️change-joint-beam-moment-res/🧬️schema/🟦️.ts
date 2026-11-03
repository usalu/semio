/** 🔩️ `change-joint-beam-moment-res` wire twin: the leaf payload `ChangeJointBeamMomentRes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBeamMomentRes {
  id: string;
  newBeamMomentResistance: number;
}

export const parseChangeJointBeamMomentRes: NormWireReader<ChangeJointBeamMomentRes> = normWireObject<ChangeJointBeamMomentRes>({ id: normWireRequired(normWireString), newBeamMomentResistance: normWireRequired(normWireNumber) });
