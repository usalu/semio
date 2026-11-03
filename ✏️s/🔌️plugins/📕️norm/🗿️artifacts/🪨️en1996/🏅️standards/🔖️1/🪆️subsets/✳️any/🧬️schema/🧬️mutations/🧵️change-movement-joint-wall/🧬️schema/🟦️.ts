/** 🧵 `change-movement-joint-wall` wire twin: the leaf payload `ChangeMovementJointWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMovementJointWall {
  id: string;
  newWallId: string;
}

export const parseChangeMovementJointWall: NormWireReader<ChangeMovementJointWall> = normWireObject<ChangeMovementJointWall>({ id: normWireRequired(normWireString), newWallId: normWireRequired(normWireString) });
