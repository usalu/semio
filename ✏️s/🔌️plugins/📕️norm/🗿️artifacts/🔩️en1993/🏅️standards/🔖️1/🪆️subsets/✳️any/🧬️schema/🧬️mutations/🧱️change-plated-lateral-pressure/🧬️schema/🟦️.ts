/** 🧱️ `change-plated-lateral-pressure` wire twin: the leaf payload `ChangePlatedLateralPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedLateralPressure {
  id: string;
  newLateralPressure: number;
}

export const parseChangePlatedLateralPressure: NormWireReader<ChangePlatedLateralPressure> = normWireObject<ChangePlatedLateralPressure>({ id: normWireRequired(normWireString), newLateralPressure: normWireRequired(normWireNumber) });
