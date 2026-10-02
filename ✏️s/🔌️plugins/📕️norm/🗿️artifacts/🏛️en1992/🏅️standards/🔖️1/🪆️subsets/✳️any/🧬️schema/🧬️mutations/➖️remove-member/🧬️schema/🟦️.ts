/** ➖️ `remove-member` wire twin: the leaf payload `RemoveMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveMember {
  memberId: string;
}

export const parseRemoveMember: NormWireReader<RemoveMember> = normWireObject<RemoveMember>({ memberId: normWireRequired(normWireString) });
