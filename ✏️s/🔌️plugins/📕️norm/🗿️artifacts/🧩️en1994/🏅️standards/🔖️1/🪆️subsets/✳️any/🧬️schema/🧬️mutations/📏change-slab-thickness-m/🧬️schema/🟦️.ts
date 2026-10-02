/** 📏 `change-slab-thickness-m` wire twin: the leaf payload `ChangeSlabThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSlabThicknessM {
  index: number;
  newConcreteThicknessM: number;
}

export const parseChangeSlabThicknessM: NormWireReader<ChangeSlabThicknessM> = normWireObject<ChangeSlabThicknessM>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newConcreteThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
