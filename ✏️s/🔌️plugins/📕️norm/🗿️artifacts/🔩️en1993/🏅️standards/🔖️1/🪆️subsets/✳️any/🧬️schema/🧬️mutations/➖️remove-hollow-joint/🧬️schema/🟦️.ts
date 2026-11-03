/** ➖️ `remove-hollow-joint` wire twin: the leaf payload `RemoveHollowJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveHollowJoint {
  index: number;
}

export const parseRemoveHollowJoint: NormWireReader<RemoveHollowJoint> = normWireObject<RemoveHollowJoint>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
