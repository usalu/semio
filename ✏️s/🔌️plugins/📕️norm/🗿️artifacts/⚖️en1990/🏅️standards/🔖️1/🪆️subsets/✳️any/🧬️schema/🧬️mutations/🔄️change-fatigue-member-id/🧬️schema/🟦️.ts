/** 🔄 `change-fatigue-member-id` wire twin: the leaf payload `ChangeFatigueMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueMemberId {
  mutation: "changeFatigueMemberId";
  id: string;
  newMemberId: string;
}

export const parseChangeFatigueMemberId: NormWireReader<ChangeFatigueMemberId> = normWireObject<ChangeFatigueMemberId>({ mutation: normWireRequired(normWireLiteral("changeFatigueMemberId")), id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
