/** 🧱️ `change-plated-lat-pressure-coef` wire twin: the leaf payload `ChangePlatedLatPressureCoef`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedLatPressureCoef {
  id: string;
  newLateralPressureCoefficient: number;
}

export const parseChangePlatedLatPressureCoef: NormWireReader<ChangePlatedLatPressureCoef> = normWireObject<ChangePlatedLatPressureCoef>({ id: normWireRequired(normWireString), newLateralPressureCoefficient: normWireRequired(normWireNumber) });
