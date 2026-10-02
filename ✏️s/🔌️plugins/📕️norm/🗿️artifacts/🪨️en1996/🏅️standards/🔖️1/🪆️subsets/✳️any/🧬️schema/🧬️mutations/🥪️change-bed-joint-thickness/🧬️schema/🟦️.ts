/** 🥪️ `change-bed-joint-thickness` wire twin: the leaf payload `ChangeBedJointThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBedJointThickness {
  index: number;
  newBedJointThicknessM: number;
}

export const parseChangeBedJointThickness: NormWireReader<ChangeBedJointThickness> = normWireObject<ChangeBedJointThickness>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newBedJointThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
