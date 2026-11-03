/** 🧱️ `change-fastened-joint-tension-force` wire twin: the leaf payload `ChangeFastenedJointTensionForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointTensionForce {
  id: string;
  newTensionForce: number;
}

export const parseChangeFastenedJointTensionForce: NormWireReader<ChangeFastenedJointTensionForce> = normWireObject<ChangeFastenedJointTensionForce>({ id: normWireRequired(normWireString), newTensionForce: normWireRequired(normWireNumber) });
