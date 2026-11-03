/** 🏋️ `change-member-action-ntk` wire twin: the leaf payload `ChangeMemberActionNTK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberActionNTK {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionNTK: NormWireReader<ChangeMemberActionNTK> = normWireObject<ChangeMemberActionNTK>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
