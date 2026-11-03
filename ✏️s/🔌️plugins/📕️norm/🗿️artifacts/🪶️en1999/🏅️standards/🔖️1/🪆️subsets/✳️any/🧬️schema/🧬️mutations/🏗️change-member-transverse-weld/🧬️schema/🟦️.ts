/** 🏗️ `change-member-transverse-weld` wire twin: the leaf payload `ChangeMemberTransverseWeld`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberTransverseWeld {
  mutation: "changeMemberTransverseWeld";
  memberId: string;
  newTransverseWeld: boolean;
}

export const parseChangeMemberTransverseWeld: NormWireReader<ChangeMemberTransverseWeld> = normWireObject<ChangeMemberTransverseWeld>({ mutation: normWireRequired(normWireLiteral("changeMemberTransverseWeld")), memberId: normWireRequired(normWireString), newTransverseWeld: normWireRequired(normWireBoolean) });
