/** ✏️ `change-pier-moment-rd-nm` wire twin: the leaf payload `ChangePierMomentRdNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierMomentRdNm {
  mutation: "changePierMomentRdNm";
  id: string;
  newMomentRdNm: number;
}

export const parseChangePierMomentRdNm: NormWireReader<ChangePierMomentRdNm> = normWireObject<ChangePierMomentRdNm>({ mutation: normWireRequired(normWireLiteral("changePierMomentRdNm")), id: normWireRequired(normWireString), newMomentRdNm: normWireRequired(normWireNumber) });
