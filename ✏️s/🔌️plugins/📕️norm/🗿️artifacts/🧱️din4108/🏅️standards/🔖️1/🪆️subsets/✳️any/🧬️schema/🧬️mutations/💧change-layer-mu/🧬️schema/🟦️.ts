/** 💧 `change-layer-mu` wire twin: the leaf payload `ChangeLayerMu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLayerMu {
  elementId: string;
  index: number;
  newMu: number;
}

export const parseChangeLayerMu: NormWireReader<ChangeLayerMu> = normWireObject<ChangeLayerMu>({ elementId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMu: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
