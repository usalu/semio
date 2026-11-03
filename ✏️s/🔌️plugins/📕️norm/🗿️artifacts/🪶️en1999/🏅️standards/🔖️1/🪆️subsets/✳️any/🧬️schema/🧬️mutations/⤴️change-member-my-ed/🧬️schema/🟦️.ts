/** ⤴️ `change-member-my-ed` wire twin: the leaf payload `ChangeMemberMYEd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMYEd {
  mutation: "changeMemberMYEd";
  memberId: string;
  actionId: string;
  newMYK: number;
}

export const parseChangeMemberMYEd: NormWireReader<ChangeMemberMYEd> = normWireObject<ChangeMemberMYEd>({ mutation: normWireRequired(normWireLiteral("changeMemberMYEd")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newMYK: normWireRequired(normWireNumber) });
