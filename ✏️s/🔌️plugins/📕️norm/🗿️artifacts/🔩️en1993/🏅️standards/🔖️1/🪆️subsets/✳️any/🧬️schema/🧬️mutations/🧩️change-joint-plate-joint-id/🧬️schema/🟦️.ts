/** 🧩️ `change-joint-plate-joint-id` wire twin: the leaf payload `ChangeJointPlateJointId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateJointId {
  id: string;
  newJointId: string;
}

export const parseChangeJointPlateJointId: NormWireReader<ChangeJointPlateJointId> = normWireObject<ChangeJointPlateJointId>({ id: normWireRequired(normWireString), newJointId: normWireRequired(normWireString) });
