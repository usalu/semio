/** 🔩️ `change-row-joint-id` wire twin: the leaf payload `ChangeRowJointId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowJointId {
  id: string;
  newJointId: string;
}

export const parseChangeRowJointId: NormWireReader<ChangeRowJointId> = normWireObject<ChangeRowJointId>({ id: normWireRequired(normWireString), newJointId: normWireRequired(normWireString) });
