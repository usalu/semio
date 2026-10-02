/** ➖️ `remove-fatigue-detail` wire twin: the leaf payload `RemoveFatigueDetail`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveFatigueDetail {
  index: number;
}

export const parseRemoveFatigueDetail: NormWireReader<RemoveFatigueDetail> = normWireObject<RemoveFatigueDetail>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
