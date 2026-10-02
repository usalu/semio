/** 🧹️ `retire-product-group` wire twin: the leaf payload `RetireProductGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RetireProductGroup {
  id: string;
}

export const parseRetireProductGroup: NormWireReader<RetireProductGroup> = normWireObject<RetireProductGroup>({ id: normWireRequired(normWireString) });
