/** 🦠️ `change-beam-stud-count` wire twin: the leaf payload `ChangeBeamStudCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBeamStudCount {
  index: number;
  newTotalCount: number;
}

export const parseChangeBeamStudCount: NormWireReader<ChangeBeamStudCount> = normWireObject<ChangeBeamStudCount>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newTotalCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
