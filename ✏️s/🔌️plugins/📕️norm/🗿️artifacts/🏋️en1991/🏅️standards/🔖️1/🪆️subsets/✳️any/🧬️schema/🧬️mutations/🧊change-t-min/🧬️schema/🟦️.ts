/** 🧊 `change-t-min` wire twin: the leaf payload `ChangeTMin`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeTMin {
  newTMin: number;
}

export const parseChangeTMin: NormWireReader<ChangeTMin> = normWireObject<ChangeTMin>({ newTMin: normWireRequired(normWireNumber) });
