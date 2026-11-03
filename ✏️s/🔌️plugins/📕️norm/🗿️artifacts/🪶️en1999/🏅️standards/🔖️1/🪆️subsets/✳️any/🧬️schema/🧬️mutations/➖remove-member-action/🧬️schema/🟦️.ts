/** 🗂️ `remove-member-action` wire twin: the leaf payload `RemoveMemberAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveMemberAction {
  mutation: "removeMemberAction";
  memberId: string;
  id: string;
}

export const parseRemoveMemberAction: NormWireReader<RemoveMemberAction> = normWireObject<RemoveMemberAction>({ mutation: normWireRequired(normWireLiteral("removeMemberAction")), memberId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
