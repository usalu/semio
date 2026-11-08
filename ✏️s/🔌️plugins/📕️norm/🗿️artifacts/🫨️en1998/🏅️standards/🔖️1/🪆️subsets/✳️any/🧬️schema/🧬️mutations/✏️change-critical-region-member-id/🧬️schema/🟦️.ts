/** ✏️ `change-critical-region-member-id` wire twin: the leaf payload `ChangeCriticalRegionMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionMemberId {
  mutation: "changeCriticalRegionMemberId";
  id: string;
  newMemberId: string;
}

export const parseChangeCriticalRegionMemberId: NormWireReader<ChangeCriticalRegionMemberId> = normWireObject<ChangeCriticalRegionMemberId>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionMemberId")), id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
