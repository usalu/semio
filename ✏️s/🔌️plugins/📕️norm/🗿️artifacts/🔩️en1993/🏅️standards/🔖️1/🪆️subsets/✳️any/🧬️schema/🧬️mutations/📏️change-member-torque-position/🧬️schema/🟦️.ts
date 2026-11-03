/** 📏️ `change-member-torque-position` wire twin: the leaf payload `ChangeMemberTorquePosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberTorquePosition {
  id: string;
  newTorquePosition: number;
}

export const parseChangeMemberTorquePosition: NormWireReader<ChangeMemberTorquePosition> = normWireObject<ChangeMemberTorquePosition>({ id: normWireRequired(normWireString), newTorquePosition: normWireRequired(normWireNumber) });
