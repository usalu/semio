/** ⭕️ `change-beam-stud-diameter-m` wire twin: the leaf payload `ChangeBeamStudDiameterM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBeamStudDiameterM {
  index: number;
  newDiameterM: number;
}

export const parseChangeBeamStudDiameterM: NormWireReader<ChangeBeamStudDiameterM> = normWireObject<ChangeBeamStudDiameterM>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newDiameterM: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
