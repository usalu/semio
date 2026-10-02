/** 🗑️ `retire-product-series` wire twin: the leaf payload `RetireProductSeries`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RetireProductSeries {
  id: string;
}

export const parseRetireProductSeries: NormWireReader<RetireProductSeries> = normWireObject<RetireProductSeries>({ id: normWireRequired(normWireString) });
