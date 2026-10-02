/** 📏️ `change-beam-span-m` wire twin: the leaf payload `ChangeBeamSpanM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBeamSpanM {
  index: number;
  newSpanM: number;
}

export const parseChangeBeamSpanM: NormWireReader<ChangeBeamSpanM> = normWireObject<ChangeBeamSpanM>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newSpanM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
