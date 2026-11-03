/** ✏️ `change-joint-building-id` wire twin: the leaf payload `ChangeJointBuildingId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBuildingId {
  mutation: "changeJointBuildingId";
  id: string;
  newBuildingId: string;
}

export const parseChangeJointBuildingId: NormWireReader<ChangeJointBuildingId> = normWireObject<ChangeJointBuildingId>({ mutation: normWireRequired(normWireLiteral("changeJointBuildingId")), id: normWireRequired(normWireString), newBuildingId: normWireRequired(normWireString) });
