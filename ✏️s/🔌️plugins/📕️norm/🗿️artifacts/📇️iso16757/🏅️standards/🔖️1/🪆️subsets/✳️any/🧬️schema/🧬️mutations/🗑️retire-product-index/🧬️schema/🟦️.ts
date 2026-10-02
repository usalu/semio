/** 🗑️ `retire-product-index` wire twin: the leaf payload `RetireProductIndex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RetireProductIndex {
  id: string;
}

export const parseRetireProductIndex: NormWireReader<RetireProductIndex> = normWireObject<RetireProductIndex>({ id: normWireRequired(normWireString) });
