/** 🧱 `change-support-sides` wire twin: the leaf payload `ChangeSupportSides`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSupportSides {
  index: number;
  newSupportSides: number;
}

export const parseChangeSupportSides: NormWireReader<ChangeSupportSides> = normWireObject<ChangeSupportSides>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newSupportSides: normWireRequired(normWireRange(normWireInteger, {"minimum":2,"maximum":4})) });
