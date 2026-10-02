/** ➖️ `remove-silo` wire twin: the leaf payload `RemoveSilo`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveSilo {
  mutation: "removeSilo";
  index: number;
}

export const parseRemoveSilo: NormWireReader<RemoveSilo> = normWireObject<RemoveSilo>({ mutation: normWireRequired(normWireLiteral("removeSilo")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
