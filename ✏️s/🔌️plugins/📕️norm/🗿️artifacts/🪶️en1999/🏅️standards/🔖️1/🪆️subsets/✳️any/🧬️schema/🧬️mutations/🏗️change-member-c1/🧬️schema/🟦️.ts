/** 🏗️ `change-member-c1` wire twin: the leaf payload `ChangeMemberC1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberC1 {
  mutation: "changeMemberC1";
  memberId: string;
  newC1: number;
}

export const parseChangeMemberC1: NormWireReader<ChangeMemberC1> = normWireObject<ChangeMemberC1>({ mutation: normWireRequired(normWireLiteral("changeMemberC1")), memberId: normWireRequired(normWireString), newC1: normWireRequired(normWireNumber) });
