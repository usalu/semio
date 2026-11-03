/** 🏛️ `change-member-punch-column-width` wire twin: the leaf payload `ChangeMemberPunchColumnWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPunchColumnWidth {
  memberId: string;
  newPunchingColumnWidth: number;
}

export const parseChangeMemberPunchColumnWidth: NormWireReader<ChangeMemberPunchColumnWidth> = normWireObject<ChangeMemberPunchColumnWidth>({ memberId: normWireRequired(normWireString), newPunchingColumnWidth: normWireRequired(normWireNumber) });
