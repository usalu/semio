/** ↪️ `change-tower-m-rd-nm` wire twin: the leaf payload `ChangeTowerMRdNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerMRdNm {
  mutation: "changeTowerMRdNm";
  id: string;
  newMRdNm: number;
}

export const parseChangeTowerMRdNm: NormWireReader<ChangeTowerMRdNm> = normWireObject<ChangeTowerMRdNm>({ mutation: normWireRequired(normWireLiteral("changeTowerMRdNm")), id: normWireRequired(normWireString), newMRdNm: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
