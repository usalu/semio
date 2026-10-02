/** 🏭 `change-silo-claimed` wire twin: the leaf payload `ChangeSiloClaimed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSiloClaimed {
  newSiloClaimed: boolean;
}

export const parseChangeSiloClaimed: NormWireReader<ChangeSiloClaimed> = normWireObject<ChangeSiloClaimed>({ newSiloClaimed: normWireRequired(normWireBoolean) });
