/** 🏗️ `change-crane-wheel-eccentricity` wire twin: the leaf payload `ChangeCraneWheelEccentricity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneWheelEccentricity {
  id: string;
  newWheelEccentricity: number;
}

export const parseChangeCraneWheelEccentricity: NormWireReader<ChangeCraneWheelEccentricity> = normWireObject<ChangeCraneWheelEccentricity>({ id: normWireRequired(normWireString), newWheelEccentricity: normWireRequired(normWireNumber) });
