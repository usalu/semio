/** 📐️ `change-slab-action-q-area-pa` wire twin: the leaf payload `ChangeSlabActionQAreaPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSlabActionQAreaPa {
  index: number;
  actionIndex: number;
  newQAreaPa: number;
}

export const parseChangeSlabActionQAreaPa: NormWireReader<ChangeSlabActionQAreaPa> = normWireObject<ChangeSlabActionQAreaPa>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), actionIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newQAreaPa: normWireRequired(normWireNumber) });
