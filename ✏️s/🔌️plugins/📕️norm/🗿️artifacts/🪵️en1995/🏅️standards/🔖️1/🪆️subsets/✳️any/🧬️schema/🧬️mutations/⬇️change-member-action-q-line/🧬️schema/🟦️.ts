/** ⬇️ `change-member-action-q-line` wire twin: the leaf payload `ChangeMemberActionQLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberActionQLine {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionQLine: NormWireReader<ChangeMemberActionQLine> = normWireObject<ChangeMemberActionQLine>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
