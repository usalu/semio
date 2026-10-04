/** 🌡️ `change-t-max` wire twin: the leaf payload `ChangeTMax`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeTMax {
  newTMax: number;
}

export const parseChangeTMax: NormWireReader<ChangeTMax> = normWireObject<ChangeTMax>({ newTMax: normWireRequired(normWireNumber) });
