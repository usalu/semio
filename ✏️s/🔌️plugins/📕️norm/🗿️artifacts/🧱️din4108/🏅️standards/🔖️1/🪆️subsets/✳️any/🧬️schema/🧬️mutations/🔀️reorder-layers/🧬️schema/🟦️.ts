/** 🔀️ `reorder-layers` wire twin: the leaf payload `ReorderLayers`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ReorderLayers {
  elementId: string;
  from: number;
  to: number;
}

export const parseReorderLayers: NormWireReader<ReorderLayers> = normWireObject<ReorderLayers>({ elementId: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
