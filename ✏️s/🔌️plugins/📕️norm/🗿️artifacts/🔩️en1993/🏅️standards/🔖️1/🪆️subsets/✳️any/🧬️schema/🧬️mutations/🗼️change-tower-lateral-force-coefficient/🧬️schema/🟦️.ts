/** 🗼️ `change-tower-lateral-force-coefficient` wire twin: the leaf payload `ChangeTowerLateralForceCoefficient`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerLateralForceCoefficient {
  id: string;
  newLateralForceCoefficient: number;
}

export const parseChangeTowerLateralForceCoefficient: NormWireReader<ChangeTowerLateralForceCoefficient> = normWireObject<ChangeTowerLateralForceCoefficient>({ id: normWireRequired(normWireString), newLateralForceCoefficient: normWireRequired(normWireNumber) });
