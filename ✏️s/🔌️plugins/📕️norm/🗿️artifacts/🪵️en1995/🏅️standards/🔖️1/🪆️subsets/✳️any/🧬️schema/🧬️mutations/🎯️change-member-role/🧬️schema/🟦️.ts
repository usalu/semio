/** 🎯️ `change-member-role` wire twin: the leaf payload `ChangeMemberRole`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type MemberRole, parseMemberRole } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberRole {
  memberId: string;
  newValue: MemberRole;
}

export const parseChangeMemberRole: NormWireReader<ChangeMemberRole> = normWireObject<ChangeMemberRole>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(parseMemberRole) });
