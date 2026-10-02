/** ↔️ `change-slab-span` wire twin: the leaf payload `ChangeSlabSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSlabSpan {
  wallIndex: number;
  index: number;
  newSlabSpanM: number;
}

export const parseChangeSlabSpan: NormWireReader<ChangeSlabSpan> = normWireObject<ChangeSlabSpan>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newSlabSpanM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
