/** 🏗️ `change-crane-claimed` wire twin: the leaf payload `ChangeCraneClaimed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeCraneClaimed {
  newCraneClaimed: boolean;
}

export const parseChangeCraneClaimed: NormWireReader<ChangeCraneClaimed> = normWireObject<ChangeCraneClaimed>({ newCraneClaimed: normWireRequired(normWireBoolean) });
