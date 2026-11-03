/** 🧱️ `change-forming-gain-member-id` wire twin: the leaf payload `ChangeFormingGainMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFormingGainMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeFormingGainMemberId: NormWireReader<ChangeFormingGainMemberId> = normWireObject<ChangeFormingGainMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
