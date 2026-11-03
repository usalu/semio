/** ↕️ `change-member-action-vk` wire twin: the leaf payload `ChangeMemberActionVK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberActionVK {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionVK: NormWireReader<ChangeMemberActionVK> = normWireObject<ChangeMemberActionVK>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
