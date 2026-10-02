/** ❄️ `change-qk-snow` wire twin: the leaf payload `ChangeQKSnow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeQKSnow {
  wallIndex: number;
  index: number;
  newQKSnowPa: number;
}

export const parseChangeQKSnow: NormWireReader<ChangeQKSnow> = normWireObject<ChangeQKSnow>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newQKSnowPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
