/** ➖ `change-assumed-crane-wheel` wire twin: the leaf payload `ChangeAssumedCraneWheel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedCraneWheel {
  newAssumedCraneWheel: number;
}

export const parseChangeAssumedCraneWheel: NormWireReader<ChangeAssumedCraneWheel> = normWireObject<ChangeAssumedCraneWheel>({ newAssumedCraneWheel: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
