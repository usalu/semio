/** 🧵 `change-movement-joint-position` wire twin: the leaf payload `ChangeMovementJointPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMovementJointPosition {
  id: string;
  newPositionM: number;
}

export const parseChangeMovementJointPosition: NormWireReader<ChangeMovementJointPosition> = normWireObject<ChangeMovementJointPosition>({ id: normWireRequired(normWireString), newPositionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
