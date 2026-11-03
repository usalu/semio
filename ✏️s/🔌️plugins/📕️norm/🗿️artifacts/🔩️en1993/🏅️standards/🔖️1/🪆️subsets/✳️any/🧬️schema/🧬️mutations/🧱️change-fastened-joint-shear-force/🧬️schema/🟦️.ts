/** 🧱️ `change-fastened-joint-shear-force` wire twin: the leaf payload `ChangeFastenedJointShearForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointShearForce {
  id: string;
  newShearForce: number;
}

export const parseChangeFastenedJointShearForce: NormWireReader<ChangeFastenedJointShearForce> = normWireObject<ChangeFastenedJointShearForce>({ id: normWireRequired(normWireString), newShearForce: normWireRequired(normWireNumber) });
