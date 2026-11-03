/** ✏️ `change-joint-column-id` wire twin: the leaf payload `ChangeJointColumnId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnId {
  mutation: "changeJointColumnId";
  id: string;
  newColumnId: string;
}

export const parseChangeJointColumnId: NormWireReader<ChangeJointColumnId> = normWireObject<ChangeJointColumnId>({ mutation: normWireRequired(normWireLiteral("changeJointColumnId")), id: normWireRequired(normWireString), newColumnId: normWireRequired(normWireString) });
