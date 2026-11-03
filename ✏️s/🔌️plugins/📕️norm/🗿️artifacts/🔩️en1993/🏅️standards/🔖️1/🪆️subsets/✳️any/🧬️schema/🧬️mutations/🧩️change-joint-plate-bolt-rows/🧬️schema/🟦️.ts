/** 🧩️ `change-joint-plate-bolt-rows` wire twin: the leaf payload `ChangeJointPlateBoltRows`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateBoltRows {
  id: string;
  newBoltRows: number;
}

export const parseChangeJointPlateBoltRows: NormWireReader<ChangeJointPlateBoltRows> = normWireObject<ChangeJointPlateBoltRows>({ id: normWireRequired(normWireString), newBoltRows: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
