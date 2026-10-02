/** 🗑️ `retire-product-class` wire twin: the leaf payload `RetireProductClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RetireProductClass {
  id: string;
}

export const parseRetireProductClass: NormWireReader<RetireProductClass> = normWireObject<RetireProductClass>({ id: normWireRequired(normWireString) });
