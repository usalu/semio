/** ✏️ `change-bridge-pier-moment-rd-nm` wire twin: the leaf payload `ChangeBridgePierMomentRdNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgePierMomentRdNm {
  mutation: "changeBridgePierMomentRdNm";
  id: string;
  newPierMomentRdNm: number;
}

export const parseChangeBridgePierMomentRdNm: NormWireReader<ChangeBridgePierMomentRdNm> = normWireObject<ChangeBridgePierMomentRdNm>({ mutation: normWireRequired(normWireLiteral("changeBridgePierMomentRdNm")), id: normWireRequired(normWireString), newPierMomentRdNm: normWireRequired(normWireNumber) });
