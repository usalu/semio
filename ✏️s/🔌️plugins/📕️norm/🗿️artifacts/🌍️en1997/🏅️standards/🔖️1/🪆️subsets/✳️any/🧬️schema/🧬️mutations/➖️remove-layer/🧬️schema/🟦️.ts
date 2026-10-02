/** ➖️ `remove-layer` wire twin: the leaf payload `RemoveLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveLayer {
  mutation: "removeLayer";
  index: number;
}

export const parseRemoveLayer: NormWireReader<RemoveLayer> = normWireObject<RemoveLayer>({ mutation: normWireRequired(normWireLiteral("removeLayer")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
