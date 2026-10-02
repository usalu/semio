/** 🗂️ `rename-product-group` wire twin: the leaf payload `RenameProductGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RenameProductGroup {
  id: string;
  newName: string;
}

export const parseRenameProductGroup: NormWireReader<RenameProductGroup> = normWireObject<RenameProductGroup>({ id: normWireRequired(normWireString), newName: normWireRequired(normWireString) });
