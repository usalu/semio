/** 🏗️ `change-crane-member-id` wire twin: the leaf payload `ChangeCraneMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeCraneMemberId: NormWireReader<ChangeCraneMemberId> = normWireObject<ChangeCraneMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
