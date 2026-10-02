/** 🏋️ `change-member-action-ntk` wire twin: the leaf payload `ChangeMemberActionNTK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberActionNTK {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionNTK: NormWireReader<ChangeMemberActionNTK> = normWireObject<ChangeMemberActionNTK>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
