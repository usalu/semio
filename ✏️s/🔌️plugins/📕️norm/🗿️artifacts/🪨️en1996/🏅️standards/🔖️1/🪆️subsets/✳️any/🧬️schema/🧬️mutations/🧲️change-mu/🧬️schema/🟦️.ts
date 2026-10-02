/** 🧲️ `change-mu` wire twin: the leaf payload `ChangeMu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMu {
  index: number;
  newMu: number;
}

export const parseChangeMu: NormWireReader<ChangeMu> = normWireObject<ChangeMu>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMu: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
