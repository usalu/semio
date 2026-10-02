/** 🧈 `change-fm` wire twin: the leaf payload `ChangeFm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFm {
  index: number;
  newMortarStrengthPa: number;
}

export const parseChangeFm: NormWireReader<ChangeFm> = normWireObject<ChangeFm>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMortarStrengthPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
