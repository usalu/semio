/** 🏢️ `change-storey-member-id` wire twin: the leaf payload `ChangeStoreyMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeStoreyMemberId: NormWireReader<ChangeStoreyMemberId> = normWireObject<ChangeStoreyMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
