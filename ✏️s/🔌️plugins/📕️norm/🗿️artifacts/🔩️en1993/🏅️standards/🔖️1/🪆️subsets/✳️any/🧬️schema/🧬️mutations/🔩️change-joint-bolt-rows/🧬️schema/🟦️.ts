/** 🔩️ `change-joint-bolt-rows` wire twin: the leaf payload `ChangeJointBoltRows`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBoltRows {
  id: string;
  newBoltRows: number;
}

export const parseChangeJointBoltRows: NormWireReader<ChangeJointBoltRows> = normWireObject<ChangeJointBoltRows>({ id: normWireRequired(normWireString), newBoltRows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
