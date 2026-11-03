/** 🕳️ `change-hollow-member-id` wire twin: the leaf payload `ChangeHollowMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeHollowMemberId: NormWireReader<ChangeHollowMemberId> = normWireObject<ChangeHollowMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
