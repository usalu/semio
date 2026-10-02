/** 🗑️ `remove-product` wire twin: the leaf payload `RemoveProduct`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveProduct {
  id: string;
}

export const parseRemoveProduct: NormWireReader<RemoveProduct> = normWireObject<RemoveProduct>({ id: normWireRequired(normWireString) });
