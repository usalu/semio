/** 📐️ `change-slab-bearing-depth` wire twin: the leaf payload `ChangeSlabBearingDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSlabBearingDepth {
  index: number;
  newSlabBearingDepthM: number;
}

export const parseChangeSlabBearingDepth: NormWireReader<ChangeSlabBearingDepth> = normWireObject<ChangeSlabBearingDepth>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newSlabBearingDepthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
