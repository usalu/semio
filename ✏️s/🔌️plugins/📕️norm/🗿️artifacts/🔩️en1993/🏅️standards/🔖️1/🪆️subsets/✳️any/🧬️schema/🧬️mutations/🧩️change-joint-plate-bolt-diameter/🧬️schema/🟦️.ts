/** 🧩️ `change-joint-plate-bolt-diameter` wire twin: the leaf payload `ChangeJointPlateBoltDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateBoltDiameter {
  id: string;
  newBoltDiameter: number;
}

export const parseChangeJointPlateBoltDiameter: NormWireReader<ChangeJointPlateBoltDiameter> = normWireObject<ChangeJointPlateBoltDiameter>({ id: normWireRequired(normWireString), newBoltDiameter: normWireRequired(normWireNumber) });
