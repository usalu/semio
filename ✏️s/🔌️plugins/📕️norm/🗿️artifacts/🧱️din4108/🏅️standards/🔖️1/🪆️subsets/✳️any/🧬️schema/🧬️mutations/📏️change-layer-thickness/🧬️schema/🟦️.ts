/** 📏️ `change-layer-thickness` wire twin: the leaf payload `ChangeLayerThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLayerThickness {
  elementId: string;
  index: number;
  newThicknessM: number;
}

export const parseChangeLayerThickness: NormWireReader<ChangeLayerThickness> = normWireObject<ChangeLayerThickness>({ elementId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
