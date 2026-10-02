/** 🔩 `change-f-yd` wire twin: the leaf payload `ChangeFYd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFYd {
  index: number;
  newFYdPa: number;
}

export const parseChangeFYd: NormWireReader<ChangeFYd> = normWireObject<ChangeFYd>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newFYdPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
