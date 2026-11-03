/** 🏗️ `change-member-restrained-ltb` wire twin: the leaf payload `ChangeMemberRestrainedLtb`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRestrainedLtb {
  mutation: "changeMemberRestrainedLtb";
  memberId: string;
  newRestrainedLtb: boolean;
}

export const parseChangeMemberRestrainedLtb: NormWireReader<ChangeMemberRestrainedLtb> = normWireObject<ChangeMemberRestrainedLtb>({ mutation: normWireRequired(normWireLiteral("changeMemberRestrainedLtb")), memberId: normWireRequired(normWireString), newRestrainedLtb: normWireRequired(normWireBoolean) });
