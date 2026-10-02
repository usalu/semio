/** ➖️ `remove-layer` wire twin: the leaf payload `RemoveLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveLayer {
  elementId: string;
  index: number;
}

export const parseRemoveLayer: NormWireReader<RemoveLayer> = normWireObject<RemoveLayer>({ elementId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
