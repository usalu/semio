/** 🧱️ `change-frame-member-basis-member-id` wire twin: the leaf payload `ChangeFrameMemberBasisMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameMemberBasisMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeFrameMemberBasisMemberId: NormWireReader<ChangeFrameMemberBasisMemberId> = normWireObject<ChangeFrameMemberBasisMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
