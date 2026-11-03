/** 🧵 `remove-movement-joint` wire twin: the leaf payload `RemoveMovementJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveMovementJoint {
  id: string;
}

export const parseRemoveMovementJoint: NormWireReader<RemoveMovementJoint> = normWireObject<RemoveMovementJoint>({ id: normWireRequired(normWireString) });
