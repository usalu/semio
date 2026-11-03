/** 🏗️ `change-member-member-alpha-cr` wire twin: the leaf payload `ChangeMemberMemberAlphaCr`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMemberAlphaCr {
  mutation: "changeMemberMemberAlphaCr";
  id: string;
  newMemberAlphaCr: number;
}

export const parseChangeMemberMemberAlphaCr: NormWireReader<ChangeMemberMemberAlphaCr> = normWireObject<ChangeMemberMemberAlphaCr>({ mutation: normWireRequired(normWireLiteral("changeMemberMemberAlphaCr")), id: normWireRequired(normWireString), newMemberAlphaCr: normWireRequired(normWireNumber) });
