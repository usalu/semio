/** ✏️ `change-joint-storey-id` wire twin: the leaf payload `ChangeJointStoreyId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointStoreyId {
  mutation: "changeJointStoreyId";
  id: string;
  newStoreyId: string;
}

export const parseChangeJointStoreyId: NormWireReader<ChangeJointStoreyId> = normWireObject<ChangeJointStoreyId>({ mutation: normWireRequired(normWireLiteral("changeJointStoreyId")), id: normWireRequired(normWireString), newStoreyId: normWireRequired(normWireString) });
