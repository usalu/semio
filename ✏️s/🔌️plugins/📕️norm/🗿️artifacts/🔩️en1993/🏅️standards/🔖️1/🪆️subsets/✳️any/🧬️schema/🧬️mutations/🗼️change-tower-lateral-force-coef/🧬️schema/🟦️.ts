/** 🗼️ `change-tower-lateral-force-coef` wire twin: the leaf payload `ChangeTowerLateralForceCoef`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerLateralForceCoef {
  id: string;
  newLateralForceCoefficient: number;
}

export const parseChangeTowerLateralForceCoef: NormWireReader<ChangeTowerLateralForceCoef> = normWireObject<ChangeTowerLateralForceCoef>({ id: normWireRequired(normWireString), newLateralForceCoefficient: normWireRequired(normWireNumber) });
