/** ✏️ `change-joint-col-moment-rd-nm` wire twin: the leaf payload `ChangeJointColMomentRdNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColMomentRdNm {
  mutation: "changeJointColMomentRdNm";
  index: number;
  newColumnMomentRdNm: number;
}

export const parseChangeJointColMomentRdNm: NormWireReader<ChangeJointColMomentRdNm> = normWireObject<ChangeJointColMomentRdNm>({ mutation: normWireRequired(normWireLiteral("changeJointColMomentRdNm")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newColumnMomentRdNm: normWireRequired(normWireNumber) });
