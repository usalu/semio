/** 🏋️ `change-gk-slab` wire twin: the leaf payload `ChangeGKSlab`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeGKSlab {
  wallIndex: number;
  index: number;
  newGKSlabN: number;
}

export const parseChangeGKSlab: NormWireReader<ChangeGKSlab> = normWireObject<ChangeGKSlab>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newGKSlabN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
