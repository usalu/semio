/** 🧷️ `change-restraint-member-id` wire twin: the leaf payload `ChangeRestraintMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeRestraintMemberId: NormWireReader<ChangeRestraintMemberId> = normWireObject<ChangeRestraintMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
