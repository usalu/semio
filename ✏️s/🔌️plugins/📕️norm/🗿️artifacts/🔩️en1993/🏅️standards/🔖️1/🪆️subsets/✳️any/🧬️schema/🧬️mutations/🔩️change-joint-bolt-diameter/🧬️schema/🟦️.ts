/** 🔩️ `change-joint-bolt-diameter` wire twin: the leaf payload `ChangeJointBoltDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBoltDiameter {
  id: string;
  newBoltDiameter: number;
}

export const parseChangeJointBoltDiameter: NormWireReader<ChangeJointBoltDiameter> = normWireObject<ChangeJointBoltDiameter>({ id: normWireRequired(normWireString), newBoltDiameter: normWireRequired(normWireNumber) });
