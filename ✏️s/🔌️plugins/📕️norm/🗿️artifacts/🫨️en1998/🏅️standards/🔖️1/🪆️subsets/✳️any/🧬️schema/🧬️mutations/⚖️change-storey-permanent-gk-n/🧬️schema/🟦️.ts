/** ⚖️ `change-storey-permanent-gk-n` wire twin: the leaf payload `ChangeStoreyPermanentGkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeStoreyPermanentGkN {
  mutation: "changeStoreyPermanentGkN";
  buildingIndex: number;
  storeyIndex: number;
  newPermanentGkN: number;
}

export const parseChangeStoreyPermanentGkN: NormWireReader<ChangeStoreyPermanentGkN> = normWireObject<ChangeStoreyPermanentGkN>({ mutation: normWireRequired(normWireLiteral("changeStoreyPermanentGkN")), buildingIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), storeyIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newPermanentGkN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
