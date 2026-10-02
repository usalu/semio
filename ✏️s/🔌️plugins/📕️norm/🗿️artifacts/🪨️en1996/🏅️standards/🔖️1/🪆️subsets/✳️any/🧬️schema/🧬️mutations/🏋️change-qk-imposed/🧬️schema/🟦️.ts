/** 🏋️ `change-qk-imposed` wire twin: the leaf payload `ChangeQKImposed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeQKImposed {
  wallIndex: number;
  index: number;
  newQKImposedPa: number;
}

export const parseChangeQKImposed: NormWireReader<ChangeQKImposed> = normWireObject<ChangeQKImposed>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newQKImposedPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
