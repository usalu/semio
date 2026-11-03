/** 🔩️ `change-joint-beam-moment-resistance` wire twin: the leaf payload `ChangeJointBeamMomentResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBeamMomentResistance {
  id: string;
  newBeamMomentResistance: number;
}

export const parseChangeJointBeamMomentResistance: NormWireReader<ChangeJointBeamMomentResistance> = normWireObject<ChangeJointBeamMomentResistance>({ id: normWireRequired(normWireString), newBeamMomentResistance: normWireRequired(normWireNumber) });
