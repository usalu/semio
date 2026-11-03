/** 🔗️ `change-conn-member-id` wire twin: the leaf payload `ChangeConnMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnMemberId {
  mutation: "changeConnMemberId";
  connectionId: string;
  newMemberId: string;
}

export const parseChangeConnMemberId: NormWireReader<ChangeConnMemberId> = normWireObject<ChangeConnMemberId>({ mutation: normWireRequired(normWireLiteral("changeConnMemberId")), connectionId: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
