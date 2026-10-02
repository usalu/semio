/** ➖️ `remove-member` wire twin: the leaf payload `RemoveMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveMember {
  index: number;
}

export const parseRemoveMember: NormWireReader<RemoveMember> = normWireObject<RemoveMember>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
