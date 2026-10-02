/** ➖ `remove-slab` wire twin: the leaf payload `RemoveSlab`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveSlab {
  index: number;
}

export const parseRemoveSlab: NormWireReader<RemoveSlab> = normWireObject<RemoveSlab>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
