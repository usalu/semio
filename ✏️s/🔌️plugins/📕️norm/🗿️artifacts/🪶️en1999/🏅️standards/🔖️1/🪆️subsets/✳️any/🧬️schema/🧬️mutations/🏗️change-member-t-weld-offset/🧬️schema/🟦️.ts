/** 🏗️ `change-member-t-weld-offset` wire twin: the leaf payload `ChangeMemberTWeldOffset`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberTWeldOffset {
  mutation: "changeMemberTWeldOffset";
  memberId: string;
  newTransverseWeldOffset: number;
}

export const parseChangeMemberTWeldOffset: NormWireReader<ChangeMemberTWeldOffset> = normWireObject<ChangeMemberTWeldOffset>({ mutation: normWireRequired(normWireLiteral("changeMemberTWeldOffset")), memberId: normWireRequired(normWireString), newTransverseWeldOffset: normWireRequired(normWireNumber) });
