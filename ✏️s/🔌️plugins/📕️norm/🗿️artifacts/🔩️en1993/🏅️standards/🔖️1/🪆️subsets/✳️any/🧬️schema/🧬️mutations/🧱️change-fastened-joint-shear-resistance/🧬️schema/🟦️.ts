/** 🧱️ `change-fastened-joint-shear-resistance` wire twin: the leaf payload `ChangeFastenedJointShearResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointShearResistance {
  id: string;
  newShearResistance: number;
}

export const parseChangeFastenedJointShearResistance: NormWireReader<ChangeFastenedJointShearResistance> = normWireObject<ChangeFastenedJointShearResistance>({ id: normWireRequired(normWireString), newShearResistance: normWireRequired(normWireNumber) });
