/** ➖️ `remove-pile` wire twin: the leaf payload `RemovePile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemovePile {
  index: number;
}

export const parseRemovePile: NormWireReader<RemovePile> = normWireObject<RemovePile>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
