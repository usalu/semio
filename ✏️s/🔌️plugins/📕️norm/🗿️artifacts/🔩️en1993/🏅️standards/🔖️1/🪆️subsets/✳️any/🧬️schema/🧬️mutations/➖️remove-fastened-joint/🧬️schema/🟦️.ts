/** ➖️ `remove-fastened-joint` wire twin: the leaf payload `RemoveFastenedJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFastenedJoint {
  index: number;
}

export const parseRemoveFastenedJoint: NormWireReader<RemoveFastenedJoint> = normWireObject<RemoveFastenedJoint>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
