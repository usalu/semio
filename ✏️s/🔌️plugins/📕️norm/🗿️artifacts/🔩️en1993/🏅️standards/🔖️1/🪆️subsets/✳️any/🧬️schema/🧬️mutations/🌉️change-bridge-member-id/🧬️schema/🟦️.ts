/** 🌉️ `change-bridge-member-id` wire twin: the leaf payload `ChangeBridgeMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeBridgeMemberId: NormWireReader<ChangeBridgeMemberId> = normWireObject<ChangeBridgeMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
