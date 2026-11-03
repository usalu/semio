/** 🔩️ `change-joint-braced-frame` wire twin: the leaf payload `ChangeJointBracedFrame`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBracedFrame {
  id: string;
  newBracedFrame: number;
}

export const parseChangeJointBracedFrame: NormWireReader<ChangeJointBracedFrame> = normWireObject<ChangeJointBracedFrame>({ id: normWireRequired(normWireString), newBracedFrame: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
