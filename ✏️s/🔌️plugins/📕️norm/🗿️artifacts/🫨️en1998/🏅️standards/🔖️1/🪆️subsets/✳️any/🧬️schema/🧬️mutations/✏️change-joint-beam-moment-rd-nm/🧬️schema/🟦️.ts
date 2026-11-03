/** ✏️ `change-joint-beam-moment-rd-nm` wire twin: the leaf payload `ChangeJointBeamMomentRdNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBeamMomentRdNm {
  mutation: "changeJointBeamMomentRdNm";
  id: string;
  newBeamMomentRdNm: number;
}

export const parseChangeJointBeamMomentRdNm: NormWireReader<ChangeJointBeamMomentRdNm> = normWireObject<ChangeJointBeamMomentRdNm>({ mutation: normWireRequired(normWireLiteral("changeJointBeamMomentRdNm")), id: normWireRequired(normWireString), newBeamMomentRdNm: normWireRequired(normWireNumber) });
