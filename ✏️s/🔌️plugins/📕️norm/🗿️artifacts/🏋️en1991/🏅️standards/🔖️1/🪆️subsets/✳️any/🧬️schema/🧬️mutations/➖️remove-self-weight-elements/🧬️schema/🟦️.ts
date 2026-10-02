/** ➖️ `remove-self-weight-elements` wire twin: the leaf payload `RemoveSelfWeightElements`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveSelfWeightElements {
  index: number;
}

export const parseRemoveSelfWeightElements: NormWireReader<RemoveSelfWeightElements> = normWireObject<RemoveSelfWeightElements>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
