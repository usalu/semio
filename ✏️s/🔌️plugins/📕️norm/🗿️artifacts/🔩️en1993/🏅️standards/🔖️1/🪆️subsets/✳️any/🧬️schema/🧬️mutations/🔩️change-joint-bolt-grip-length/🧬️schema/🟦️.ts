/** 🔩️ `change-joint-bolt-grip-length` wire twin: the leaf payload `ChangeJointBoltGripLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBoltGripLength {
  id: string;
  newBoltGripLength: number;
}

export const parseChangeJointBoltGripLength: NormWireReader<ChangeJointBoltGripLength> = normWireObject<ChangeJointBoltGripLength>({ id: normWireRequired(normWireString), newBoltGripLength: normWireRequired(normWireNumber) });
