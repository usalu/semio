/** 🏋️ `change-member-action-fc90-k` wire twin: the leaf payload `ChangeMemberActionFC90K`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberActionFC90K {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeMemberActionFC90K: NormWireReader<ChangeMemberActionFC90K> = normWireObject<ChangeMemberActionFC90K>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
