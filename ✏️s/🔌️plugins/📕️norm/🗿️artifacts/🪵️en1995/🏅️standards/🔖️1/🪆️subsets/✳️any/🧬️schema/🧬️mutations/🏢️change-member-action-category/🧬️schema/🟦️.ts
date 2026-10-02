/** 🏢️ `change-member-action-category` wire twin: the leaf payload `ChangeMemberActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberActionCategory {
  memberId: string;
  actionId: string;
  newValue: string;
}

export const parseChangeMemberActionCategory: NormWireReader<ChangeMemberActionCategory> = normWireObject<ChangeMemberActionCategory>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
