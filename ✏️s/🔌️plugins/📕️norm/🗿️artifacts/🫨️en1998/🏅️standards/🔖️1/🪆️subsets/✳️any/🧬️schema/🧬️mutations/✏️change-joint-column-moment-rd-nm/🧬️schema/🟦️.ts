/** ✏️ `change-joint-column-moment-rd-nm` wire twin: the leaf payload `ChangeJointColumnMomentRdNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnMomentRdNm {
  mutation: "changeJointColumnMomentRdNm";
  id: string;
  newColumnMomentRdNm: number;
}

export const parseChangeJointColumnMomentRdNm: NormWireReader<ChangeJointColumnMomentRdNm> = normWireObject<ChangeJointColumnMomentRdNm>({ mutation: normWireRequired(normWireLiteral("changeJointColumnMomentRdNm")), id: normWireRequired(normWireString), newColumnMomentRdNm: normWireRequired(normWireNumber) });
