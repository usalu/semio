/** ↪️ `change-tower-m-rd-nm` wire twin: the leaf payload `ChangeTowerMRdNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeTowerMRdNm {
  mutation: "changeTowerMRdNm";
  index: number;
  newMRdNm: number;
}

export const parseChangeTowerMRdNm: NormWireReader<ChangeTowerMRdNm> = normWireObject<ChangeTowerMRdNm>({ mutation: normWireRequired(normWireLiteral("changeTowerMRdNm")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMRdNm: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
