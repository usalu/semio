/** ➖ `remove-permanent` wire twin: the leaf payload `RemovePermanent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemovePermanent {
  mutation: "removePermanent";
  index: number;
}

export const parseRemovePermanent: NormWireReader<RemovePermanent> = normWireObject<RemovePermanent>({ mutation: normWireRequired(normWireLiteral("removePermanent")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
