/** 🚫️ `retire-product` wire twin: the leaf payload `RetireProduct`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RetireProduct {
  id: string;
}

export const parseRetireProduct: NormWireReader<RetireProduct> = normWireObject<RetireProduct>({ id: normWireRequired(normWireString) });
