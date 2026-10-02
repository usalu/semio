/** 🎯️ `change-member-role` wire twin: the leaf payload `ChangeMemberRole`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type MemberRole, parseMemberRole } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberRole {
  memberId: string;
  newValue: MemberRole;
}

export const parseChangeMemberRole: NormWireReader<ChangeMemberRole> = normWireObject<ChangeMemberRole>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(parseMemberRole) });
