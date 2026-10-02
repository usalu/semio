/** 💧️ `change-rh-int` wire twin: the leaf payload `ChangeRhInt`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeRhInt {
  newRhInt: number;
}

export const parseChangeRhInt: NormWireReader<ChangeRhInt> = normWireObject<ChangeRhInt>({ newRhInt: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
