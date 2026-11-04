/** 🚦 `change-reliability-member-id` wire twin: the leaf payload `ChangeReliabilityMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReliabilityMemberId {
  mutation: "changeReliabilityMemberId";
  id: string;
  newMemberId: string;
}

export const parseChangeReliabilityMemberId: NormWireReader<ChangeReliabilityMemberId> = normWireObject<ChangeReliabilityMemberId>({ mutation: normWireRequired(normWireLiteral("changeReliabilityMemberId")), id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
