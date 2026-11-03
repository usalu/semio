/** ✏️ `change-joint-beam-id` wire twin: the leaf payload `ChangeJointBeamId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBeamId {
  mutation: "changeJointBeamId";
  id: string;
  newBeamId: string;
}

export const parseChangeJointBeamId: NormWireReader<ChangeJointBeamId> = normWireObject<ChangeJointBeamId>({ mutation: normWireRequired(normWireLiteral("changeJointBeamId")), id: normWireRequired(normWireString), newBeamId: normWireRequired(normWireString) });
