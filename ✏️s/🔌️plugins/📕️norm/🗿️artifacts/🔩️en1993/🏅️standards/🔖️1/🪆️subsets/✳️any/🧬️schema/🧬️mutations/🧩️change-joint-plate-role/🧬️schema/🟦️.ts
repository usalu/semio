/** 🧩️ `change-joint-plate-role` wire twin: the leaf payload `ChangeJointPlateRole`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateRole {
  id: string;
  newRole: string;
}

export const parseChangeJointPlateRole: NormWireReader<ChangeJointPlateRole> = normWireObject<ChangeJointPlateRole>({ id: normWireRequired(normWireString), newRole: normWireRequired(normWireString) });
