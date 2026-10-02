/** ✂️ `remove-effect` wire twin: the leaf payload `RemoveEffect`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveEffect {
  mutation: "removeEffect";
  index: number;
}

export const parseRemoveEffect: NormWireReader<RemoveEffect> = normWireObject<RemoveEffect>({ mutation: normWireRequired(normWireLiteral("removeEffect")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
