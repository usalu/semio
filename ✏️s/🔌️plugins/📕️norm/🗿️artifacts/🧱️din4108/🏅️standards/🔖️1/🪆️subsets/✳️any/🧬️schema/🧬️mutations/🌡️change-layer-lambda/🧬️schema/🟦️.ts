/** 🌡️ `change-layer-lambda` wire twin: the leaf payload `ChangeLayerLambda`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLayerLambda {
  elementId: string;
  index: number;
  newLambda: number;
}

export const parseChangeLayerLambda: NormWireReader<ChangeLayerLambda> = normWireObject<ChangeLayerLambda>({ elementId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newLambda: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
