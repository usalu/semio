/** ➖️ `remove-foundation` wire twin: the leaf payload `RemoveFoundation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveFoundation {
  mutation: "removeFoundation";
  index: number;
}

export const parseRemoveFoundation: NormWireReader<RemoveFoundation> = normWireObject<RemoveFoundation>({ mutation: normWireRequired(normWireLiteral("removeFoundation")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
