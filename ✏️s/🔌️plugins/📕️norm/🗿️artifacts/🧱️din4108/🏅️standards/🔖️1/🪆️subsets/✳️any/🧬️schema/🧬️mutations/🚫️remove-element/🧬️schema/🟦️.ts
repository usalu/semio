/** 🚫️ `remove-element` wire twin: the leaf payload `RemoveElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveElement {
  index: number;
}

export const parseRemoveElement: NormWireReader<RemoveElement> = normWireObject<RemoveElement>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
