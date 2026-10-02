/** 🧱 `change-beam-slab-thickness-m` wire twin: the leaf payload `ChangeBeamSlabThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBeamSlabThicknessM {
  index: number;
  newSlabThicknessM: number;
}

export const parseChangeBeamSlabThicknessM: NormWireReader<ChangeBeamSlabThicknessM> = normWireObject<ChangeBeamSlabThicknessM>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newSlabThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
