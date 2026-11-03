/** 🏋️ `change-action-member-id` wire twin: the leaf payload `ChangeActionMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeActionMemberId: NormWireReader<ChangeActionMemberId> = normWireObject<ChangeActionMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
