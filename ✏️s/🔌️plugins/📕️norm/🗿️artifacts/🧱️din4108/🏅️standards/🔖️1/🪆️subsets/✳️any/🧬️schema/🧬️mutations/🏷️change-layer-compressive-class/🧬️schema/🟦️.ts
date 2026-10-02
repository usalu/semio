/** 🏷️ `change-layer-compressive-class` wire twin: the leaf payload `ChangeLayerCompressiveClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLayerCompressiveClass {
  elementId: string;
  index: number;
  newCompressiveClass: string;
}

export const parseChangeLayerCompressiveClass: NormWireReader<ChangeLayerCompressiveClass> = normWireObject<ChangeLayerCompressiveClass>({ elementId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newCompressiveClass: normWireRequired(normWireString) });
