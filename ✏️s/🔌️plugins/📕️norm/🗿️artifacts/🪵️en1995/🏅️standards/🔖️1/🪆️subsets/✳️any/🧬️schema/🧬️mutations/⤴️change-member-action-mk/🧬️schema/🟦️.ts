/** ⤴️ `change-member-action-mk` wire twin: the leaf payload `ChangeMemberActionMK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberActionMK {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionMK: NormWireReader<ChangeMemberActionMK> = normWireObject<ChangeMemberActionMK>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
