/** ➖ `remove-footing` wire twin: the leaf payload `RemoveFooting`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveFooting {
  mutation: "removeFooting";
  index: number;
}

export const parseRemoveFooting: NormWireReader<RemoveFooting> = normWireObject<RemoveFooting>({ mutation: normWireRequired(normWireLiteral("removeFooting")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
