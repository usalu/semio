/** 🏋️ `change-member-action-fc90-k` wire twin: the leaf payload `ChangeMemberActionFC90K`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberActionFC90K {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionFC90K: NormWireReader<ChangeMemberActionFC90K> = normWireObject<ChangeMemberActionFC90K>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
