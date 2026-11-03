/** 🧵 `insert-movement-joint` wire twin: the leaf payload `InsertMovementJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type MovementJoint, parseMovementJoint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMovementJoint {
  index: number;
  item: MovementJoint;
}

export const parseInsertMovementJoint: NormWireReader<InsertMovementJoint> = normWireObject<InsertMovementJoint>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseMovementJoint) });
