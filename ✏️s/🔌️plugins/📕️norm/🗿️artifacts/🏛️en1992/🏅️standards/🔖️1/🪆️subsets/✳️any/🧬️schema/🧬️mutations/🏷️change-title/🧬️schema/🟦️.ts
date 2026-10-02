/** 🏷️ `change-title` wire twin: the leaf payload `ChangeTitle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeTitle {
  newTitle: string;
}

export const parseChangeTitle: NormWireReader<ChangeTitle> = normWireObject<ChangeTitle>({ newTitle: normWireRequired(normWireString) });
