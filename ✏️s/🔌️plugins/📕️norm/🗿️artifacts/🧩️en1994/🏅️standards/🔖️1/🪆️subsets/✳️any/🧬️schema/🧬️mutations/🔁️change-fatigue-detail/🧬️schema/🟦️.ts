/** 🔁️ `change-fatigue-detail` wire twin: the leaf payload `ChangeFatigueDetail`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFatigueDetail {
  newFatigueDetail: string;
}

export const parseChangeFatigueDetail: NormWireReader<ChangeFatigueDetail> = normWireObject<ChangeFatigueDetail>({ newFatigueDetail: normWireRequired(normWireString) });
