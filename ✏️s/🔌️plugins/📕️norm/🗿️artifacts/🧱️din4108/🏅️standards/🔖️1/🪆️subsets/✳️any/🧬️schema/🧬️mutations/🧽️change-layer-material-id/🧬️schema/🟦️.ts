/** 🧽️ `change-layer-material-id` wire twin: the leaf payload `ChangeLayerMaterialId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLayerMaterialId {
  elementId: string;
  index: number;
  newMaterialId: string;
}

export const parseChangeLayerMaterialId: NormWireReader<ChangeLayerMaterialId> = normWireObject<ChangeLayerMaterialId>({ elementId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMaterialId: normWireRequired(normWireString) });
