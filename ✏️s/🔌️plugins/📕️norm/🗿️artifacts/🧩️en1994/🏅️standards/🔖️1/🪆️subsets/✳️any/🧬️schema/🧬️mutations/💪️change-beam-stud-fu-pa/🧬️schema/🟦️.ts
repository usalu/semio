/** 💪️ `change-beam-stud-fu-pa` wire twin: the leaf payload `ChangeBeamStudFUPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBeamStudFUPa {
  index: number;
  newFUPa: number;
}

export const parseChangeBeamStudFUPa: NormWireReader<ChangeBeamStudFUPa> = normWireObject<ChangeBeamStudFUPa>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newFUPa: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
