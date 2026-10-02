/** 🏷️ `rename-product` wire twin: the leaf payload `RenameProduct`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RenameProduct {
  id: string;
  newName: string;
}

export const parseRenameProduct: NormWireReader<RenameProduct> = normWireObject<RenameProduct>({ id: normWireRequired(normWireString), newName: normWireRequired(normWireString) });
