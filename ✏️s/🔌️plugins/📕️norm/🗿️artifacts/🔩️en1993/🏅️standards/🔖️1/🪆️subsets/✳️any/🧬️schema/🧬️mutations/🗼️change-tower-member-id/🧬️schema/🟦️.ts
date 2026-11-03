/** 🗼️ `change-tower-member-id` wire twin: the leaf payload `ChangeTowerMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeTowerMemberId: NormWireReader<ChangeTowerMemberId> = normWireObject<ChangeTowerMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
