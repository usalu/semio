/** 🏋️ `change-member-n-ed` wire twin: the leaf payload `ChangeMemberNEd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberNEd {
  mutation: "changeMemberNEd";
  memberId: string;
  actionId: string;
  newNK: number;
}

export const parseChangeMemberNEd: NormWireReader<ChangeMemberNEd> = normWireObject<ChangeMemberNEd>({ mutation: normWireRequired(normWireLiteral("changeMemberNEd")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newNK: normWireRequired(normWireNumber) });
